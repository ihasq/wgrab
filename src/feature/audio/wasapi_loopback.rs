use std::ffi::c_void;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use windows::core::{Interface, GUID};
use windows::Win32::Media::Audio::{
    eConsole, eRender, IAudioCaptureClient, IAudioClient, IMMDeviceEnumerator, MMDeviceEnumerator,
    AUDCLNT_BUFFERFLAGS_SILENT, AUDCLNT_SHAREMODE_SHARED, AUDCLNT_STREAMFLAGS_LOOPBACK,
    WAVEFORMATEX, WAVEFORMATEXTENSIBLE, WAVE_FORMAT_PCM,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, CLSCTX_ALL,
    COINIT_MULTITHREADED,
};

use crate::feature::audio::{
    WgrabAudioError, WgrabAudioFormat, WgrabAudioStream, WgrabSampleFormat,
};

const WAVE_FORMAT_IEEE_FLOAT: u16 = 3;
const WAVE_FORMAT_EXTENSIBLE: u16 = 0xfffe;
const KSDATAFORMAT_SUBTYPE_PCM: GUID = GUID::from_u128(0x00000001_0000_0010_8000_00aa00389b71);
const KSDATAFORMAT_SUBTYPE_IEEE_FLOAT: GUID =
    GUID::from_u128(0x00000003_0000_0010_8000_00aa00389b71);
const REFERENCE_TIME_PER_SECOND: i64 = 10_000_000;

#[derive(Debug, Clone, Copy)]
enum WasapiSampleKind {
    F32,
    I16,
}

struct ComGuard {
    uninitialize: bool,
}

impl ComGuard {
    fn initialize_mta() -> Self {
        let uninitialize = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).is_ok() };
        Self { uninitialize }
    }
}

impl Drop for ComGuard {
    fn drop(&mut self) {
        if self.uninitialize {
            unsafe {
                CoUninitialize();
            }
        }
    }
}

struct RawCaptureClient(*mut c_void);

unsafe impl Send for RawCaptureClient {}
unsafe impl Sync for RawCaptureClient {}

impl RawCaptureClient {
    fn new(client: IAudioCaptureClient) -> Self {
        Self(client.into_raw())
    }

    unsafe fn into_client(self) -> IAudioCaptureClient {
        IAudioCaptureClient::from_raw(self.0)
    }
}

pub(crate) struct WasapiLoopbackStream {
    audio_client: IAudioClient,
    stop: Arc<AtomicBool>,
    thread: Option<thread::JoinHandle<()>>,
    _com: ComGuard,
}

impl Drop for WasapiLoopbackStream {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        unsafe {
            let _ = self.audio_client.Stop();
        }
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

pub(crate) fn build_default_loopback_stream() -> Result<WgrabAudioStream, WgrabAudioError> {
    unsafe {
        let com = ComGuard::initialize_mta();

        let device_enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).map_err(|error| {
                WgrabAudioError::WasapiUnavailable(format!(
                    "failed to create IMMDeviceEnumerator: {error}"
                ))
            })?;
        let device = device_enumerator
            .GetDefaultAudioEndpoint(eRender, eConsole)
            .map_err(|error| {
                WgrabAudioError::WasapiUnavailable(format!(
                    "failed to get default render endpoint: {error}"
                ))
            })?;
        let audio_client: IAudioClient = device.Activate(CLSCTX_ALL, None).map_err(|error| {
            WgrabAudioError::WasapiUnavailable(format!("failed to activate IAudioClient: {error}"))
        })?;

        let mix_format_ptr = audio_client.GetMixFormat().map_err(|error| {
            WgrabAudioError::WasapiInitializationFailed(format!(
                "failed to get WASAPI mix format: {error}"
            ))
        })?;
        if mix_format_ptr.is_null() {
            return Err(WgrabAudioError::WasapiInitializationFailed(
                "WASAPI mix format pointer was null".to_string(),
            ));
        }

        let mix_format = match MixFormat::from_wave_format(mix_format_ptr) {
            Ok(format) => format,
            Err(error) => {
                CoTaskMemFree(Some(mix_format_ptr as *const c_void));
                return Err(error);
            }
        };
        let buffer_duration = REFERENCE_TIME_PER_SECOND / 10;
        let initialize_result = audio_client.Initialize(
            AUDCLNT_SHAREMODE_SHARED,
            AUDCLNT_STREAMFLAGS_LOOPBACK,
            buffer_duration,
            0,
            mix_format_ptr,
            None,
        );
        CoTaskMemFree(Some(mix_format_ptr as *const c_void));

        initialize_result.map_err(|error| {
            WgrabAudioError::WasapiInitializationFailed(format!(
                "failed to initialize WASAPI loopback stream: {error}"
            ))
        })?;

        let capture_client: IAudioCaptureClient = audio_client.GetService().map_err(|error| {
            WgrabAudioError::WasapiInitializationFailed(format!(
                "failed to get IAudioCaptureClient: {error}"
            ))
        })?;

        let buffer = Arc::new(Mutex::new(Vec::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let thread = spawn_capture_thread(
            RawCaptureClient::new(capture_client),
            Arc::clone(&buffer),
            Arc::clone(&stop),
            mix_format.sample_kind,
            mix_format.channels,
        );

        audio_client.Start().map_err(|error| {
            WgrabAudioError::WasapiInitializationFailed(format!(
                "failed to start WASAPI loopback stream: {error}"
            ))
        })?;

        let stream = WasapiLoopbackStream {
            audio_client,
            stop,
            thread: Some(thread),
            _com: com,
        };

        Ok(WgrabAudioStream::new_wasapi_loopback(
            stream,
            buffer,
            WgrabAudioFormat {
                sample_rate: mix_format.sample_rate,
                channels: mix_format.channels,
                sample_format: WgrabSampleFormat::F32,
            },
            Some("default render endpoint".to_string()),
        ))
    }
}

struct MixFormat {
    sample_rate: u32,
    channels: u16,
    sample_kind: WasapiSampleKind,
}

impl MixFormat {
    unsafe fn from_wave_format(format: *const WAVEFORMATEX) -> Result<Self, WgrabAudioError> {
        let tag = ptr::addr_of!((*format).wFormatTag).read_unaligned();
        let bits_per_sample = ptr::addr_of!((*format).wBitsPerSample).read_unaligned();
        let sample_rate = ptr::addr_of!((*format).nSamplesPerSec).read_unaligned();
        let channels = ptr::addr_of!((*format).nChannels).read_unaligned();
        let sample_kind = sample_kind(format, tag, bits_per_sample)?;

        Ok(Self {
            sample_rate,
            channels,
            sample_kind,
        })
    }
}

unsafe fn sample_kind(
    format: *const WAVEFORMATEX,
    tag: u16,
    bits_per_sample: u16,
) -> Result<WasapiSampleKind, WgrabAudioError> {
    match tag {
        tag if tag == WAVE_FORMAT_PCM as u16 && bits_per_sample == 16 => Ok(WasapiSampleKind::I16),
        WAVE_FORMAT_IEEE_FLOAT if bits_per_sample == 32 => Ok(WasapiSampleKind::F32),
        WAVE_FORMAT_EXTENSIBLE => {
            let extensible = format as *const WAVEFORMATEXTENSIBLE;
            let sub_format = ptr::addr_of!((*extensible).SubFormat).read_unaligned();
            if sub_format == KSDATAFORMAT_SUBTYPE_PCM && bits_per_sample == 16 {
                Ok(WasapiSampleKind::I16)
            } else if sub_format == KSDATAFORMAT_SUBTYPE_IEEE_FLOAT && bits_per_sample == 32 {
                Ok(WasapiSampleKind::F32)
            } else {
                Err(WgrabAudioError::UnsupportedSampleFormat(format!(
                    "WASAPI extensible format bits={} sub_format={sub_format:?}",
                    bits_per_sample
                )))
            }
        }
        tag => Err(WgrabAudioError::UnsupportedSampleFormat(format!(
            "WASAPI wave format tag={tag} bits={}",
            bits_per_sample
        ))),
    }
}

fn spawn_capture_thread(
    capture_client: RawCaptureClient,
    buffer: Arc<Mutex<Vec<f32>>>,
    stop: Arc<AtomicBool>,
    sample_kind: WasapiSampleKind,
    channels: u16,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let _com = ComGuard::initialize_mta();
        let capture_client = unsafe { capture_client.into_client() };

        while !stop.load(Ordering::Relaxed) {
            match unsafe { capture_client.GetNextPacketSize() } {
                Ok(mut packet_frames) => {
                    while packet_frames > 0 && !stop.load(Ordering::Relaxed) {
                        if capture_packet(&capture_client, &buffer, sample_kind, channels).is_err()
                        {
                            stop.store(true, Ordering::Relaxed);
                            break;
                        }
                        packet_frames = unsafe { capture_client.GetNextPacketSize().unwrap_or(0) };
                    }
                }
                Err(_) => {
                    stop.store(true, Ordering::Relaxed);
                }
            }

            thread::sleep(Duration::from_millis(5));
        }
    })
}

fn capture_packet(
    capture_client: &IAudioCaptureClient,
    buffer: &Arc<Mutex<Vec<f32>>>,
    sample_kind: WasapiSampleKind,
    channels: u16,
) -> windows::core::Result<()> {
    unsafe {
        let mut data_ptr: *mut u8 = ptr::null_mut();
        let mut frame_count = 0u32;
        let mut flags = 0u32;

        capture_client.GetBuffer(&mut data_ptr, &mut frame_count, &mut flags, None, None)?;

        if let Ok(mut samples) = buffer.lock() {
            let sample_count = frame_count as usize * usize::from(channels);
            if flags & AUDCLNT_BUFFERFLAGS_SILENT.0 as u32 != 0 || data_ptr.is_null() {
                samples.extend(std::iter::repeat(0.0).take(sample_count));
            } else {
                match sample_kind {
                    WasapiSampleKind::F32 => {
                        let data = std::slice::from_raw_parts(data_ptr as *const f32, sample_count);
                        samples.extend_from_slice(data);
                    }
                    WasapiSampleKind::I16 => {
                        let data = std::slice::from_raw_parts(data_ptr as *const i16, sample_count);
                        samples.extend(data.iter().map(|sample| *sample as f32 / i16::MAX as f32));
                    }
                }
            }
        }

        capture_client.ReleaseBuffer(frame_count)
    }
}
