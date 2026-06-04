#[cfg(feature = "audio-cpal")]
mod cpal_backend;

#[cfg(target_os = "macos")]
mod screencapturekit_audio;

#[cfg(windows)]
mod wasapi_loopback;

#[cfg(feature = "audio-cpal")]
pub use cpal_backend::CpalAudioBackend;

use std::collections::VecDeque;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub use crate::time::{
    pair_video_audio_timestamps, WgrabAvPairInfo, WgrabAvPairStatus, WgrabAvQueueMatch,
    WgrabAvSyncQueue, WgrabAvSyncQueueConfig, WgrabAvSyncTolerance, WgrabTimestamp,
    WgrabTimestampQuality, WgrabTimestampedFrameInfo,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WgrabSampleFormat {
    F32,
    I16,
    U16,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WgrabAudioFormat {
    pub sample_rate: u32,
    pub channels: u16,
    pub sample_format: WgrabSampleFormat,
}

#[derive(Debug, Clone)]
pub struct WgrabAudioDeviceReport {
    pub name: String,
    pub is_default_input: bool,
    pub is_default_output: bool,
    pub supports_input: bool,
    pub supports_output: bool,
    pub loopback_candidate: bool,
    pub input_config_count: usize,
    pub output_config_count: usize,
    pub default_input_format: Option<WgrabAudioFormat>,
    pub default_output_format: Option<WgrabAudioFormat>,
}

#[derive(Debug, Clone)]
pub struct WgrabAudioLoopbackCandidate {
    pub report: WgrabAudioDeviceReport,
    pub score: i32,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WgrabAudioSource {
    DefaultInput,
    SystemAudioCandidate,
    #[cfg(target_os = "windows")]
    WasapiLoopback,
    #[cfg(target_os = "macos")]
    ScreenCaptureKitAudio,
}

#[derive(Debug, Clone)]
pub struct WgrabCaptureClock {
    start: Instant,
}

impl WgrabCaptureClock {
    pub fn start_now() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn now(&self) -> WgrabTimestamp {
        WgrabTimestamp::from_nanos(self.start.elapsed().as_nanos() as u64)
    }
}

pub struct WgrabAudioFrame {
    format: WgrabAudioFormat,
    timestamp: Option<WgrabTimestamp>,
    timestamp_quality: WgrabTimestampQuality,
    frames: usize,
    samples: Vec<f32>,
}

impl WgrabAudioFrame {
    pub(crate) fn new(
        format: WgrabAudioFormat,
        timestamp: Option<WgrabTimestamp>,
        timestamp_quality: WgrabTimestampQuality,
        frames: usize,
        samples: Vec<f32>,
    ) -> Self {
        Self {
            format,
            timestamp,
            timestamp_quality,
            frames,
            samples,
        }
    }

    pub fn format(&self) -> WgrabAudioFormat {
        self.format
    }

    pub fn timestamp(&self) -> Option<WgrabTimestamp> {
        self.timestamp
    }

    pub fn timestamp_quality(&self) -> WgrabTimestampQuality {
        self.timestamp_quality
    }

    pub fn frames(&self) -> usize {
        self.frames
    }

    pub fn samples_f32(&self) -> &[f32] {
        &self.samples
    }
}

pub(crate) struct WgrabQueuedAudioFrame {
    format: WgrabAudioFormat,
    timestamp: Option<WgrabTimestamp>,
    timestamp_quality: WgrabTimestampQuality,
    frames: usize,
    samples: Vec<f32>,
}

impl WgrabQueuedAudioFrame {
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    pub(crate) fn new(
        format: WgrabAudioFormat,
        timestamp: Option<WgrabTimestamp>,
        timestamp_quality: WgrabTimestampQuality,
        frames: usize,
        samples: Vec<f32>,
    ) -> Self {
        Self {
            format,
            timestamp,
            timestamp_quality,
            frames,
            samples,
        }
    }
}

pub struct WgrabAudioContext;

impl WgrabAudioContext {
    pub fn new() -> Self {
        Self
    }

    #[cfg(feature = "audio-cpal")]
    pub fn cpal_backend(&self) -> CpalAudioBackend {
        CpalAudioBackend::default_host()
    }

    #[cfg(windows)]
    pub fn build_system_audio_stream(&self) -> Result<WgrabAudioStream, WgrabAudioError> {
        wasapi_loopback::build_default_loopback_stream()
    }

    #[cfg(target_os = "macos")]
    pub fn build_screencapturekit_audio_stream(&self) -> Result<WgrabAudioStream, WgrabAudioError> {
        screencapturekit_audio::build_default_screencapturekit_audio_stream()
    }
}

impl Default for WgrabAudioContext {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum WgrabAudioError {
    NoInputDevice,
    NoLoopbackCandidate,
    DefaultInputConfigFailed(String),
    BuildStreamFailed(String),
    PlayStreamFailed(String),
    UnsupportedSampleFormat(String),
    BufferUnavailable,
    WasapiUnavailable(String),
    WasapiInitializationFailed(String),
    ScreenCaptureKitUnavailable(String),
}

impl fmt::Display for WgrabAudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoInputDevice => write!(f, "no default input device is available"),
            Self::NoLoopbackCandidate => write!(f, "no CPAL loopback candidate is available"),
            Self::DefaultInputConfigFailed(error) => {
                write!(f, "default input config failed: {error}")
            }
            Self::BuildStreamFailed(error) => write!(f, "build input stream failed: {error}"),
            Self::PlayStreamFailed(error) => write!(f, "play input stream failed: {error}"),
            Self::UnsupportedSampleFormat(format) => {
                write!(f, "unsupported sample format: {format}")
            }
            Self::BufferUnavailable => write!(f, "audio sample buffer is unavailable"),
            Self::WasapiUnavailable(error) => write!(f, "WASAPI loopback unavailable: {error}"),
            Self::WasapiInitializationFailed(error) => {
                write!(f, "WASAPI initialization failed: {error}")
            }
            Self::ScreenCaptureKitUnavailable(error) => {
                write!(f, "ScreenCaptureKit audio unavailable: {error}")
            }
        }
    }
}

impl std::error::Error for WgrabAudioError {}

pub struct WgrabAudioStream {
    // CPAL streams stop capturing when dropped, so this field intentionally
    // owns the live stream for the lifetime of the prototype stream object.
    #[cfg(feature = "audio-cpal")]
    #[allow(dead_code)]
    pub(crate) stream: Option<cpal::Stream>,
    #[cfg(windows)]
    #[allow(dead_code)]
    wasapi_loopback_stream: Option<wasapi_loopback::WasapiLoopbackStream>,
    #[cfg(target_os = "macos")]
    #[allow(dead_code)]
    screencapturekit_audio_stream: Option<screencapturekit_audio::ScreenCaptureKitAudioStream>,
    pub(crate) buffer: Arc<Mutex<Vec<f32>>>,
    pub(crate) queued_frames: Arc<Mutex<VecDeque<WgrabQueuedAudioFrame>>>,
    format: WgrabAudioFormat,
    device_name: Option<String>,
    source: WgrabAudioSource,
    clock: WgrabCaptureClock,
}

impl WgrabAudioStream {
    pub(crate) fn new(
        #[cfg(feature = "audio-cpal")] stream: Option<cpal::Stream>,
        buffer: Arc<Mutex<Vec<f32>>>,
        format: WgrabAudioFormat,
        device_name: Option<String>,
        source: WgrabAudioSource,
    ) -> Self {
        Self {
            #[cfg(feature = "audio-cpal")]
            stream,
            #[cfg(windows)]
            wasapi_loopback_stream: None,
            #[cfg(target_os = "macos")]
            screencapturekit_audio_stream: None,
            buffer,
            queued_frames: Arc::new(Mutex::new(VecDeque::new())),
            format,
            device_name,
            source,
            clock: WgrabCaptureClock::start_now(),
        }
    }

    #[cfg(windows)]
    pub(crate) fn new_wasapi_loopback(
        stream: wasapi_loopback::WasapiLoopbackStream,
        buffer: Arc<Mutex<Vec<f32>>>,
        format: WgrabAudioFormat,
        device_name: Option<String>,
    ) -> Self {
        Self {
            #[cfg(feature = "audio-cpal")]
            stream: None,
            wasapi_loopback_stream: Some(stream),
            #[cfg(target_os = "macos")]
            screencapturekit_audio_stream: None,
            buffer,
            queued_frames: Arc::new(Mutex::new(VecDeque::new())),
            format,
            device_name,
            source: WgrabAudioSource::WasapiLoopback,
            clock: WgrabCaptureClock::start_now(),
        }
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn new_screencapturekit_audio(
        stream: screencapturekit_audio::ScreenCaptureKitAudioStream,
        buffer: Arc<Mutex<Vec<f32>>>,
        queued_frames: Arc<Mutex<VecDeque<WgrabQueuedAudioFrame>>>,
        format: WgrabAudioFormat,
        device_name: Option<String>,
    ) -> Self {
        Self {
            #[cfg(feature = "audio-cpal")]
            stream: None,
            #[cfg(windows)]
            wasapi_loopback_stream: None,
            screencapturekit_audio_stream: Some(stream),
            buffer,
            queued_frames,
            format,
            device_name,
            source: WgrabAudioSource::ScreenCaptureKitAudio,
            clock: WgrabCaptureClock::start_now(),
        }
    }

    pub fn format(&self) -> WgrabAudioFormat {
        self.format
    }

    pub fn device_name(&self) -> Option<&str> {
        self.device_name.as_deref()
    }

    pub fn source(&self) -> WgrabAudioSource {
        self.source
    }

    pub fn try_next_frame(&self) -> Result<Option<WgrabAudioFrame>, WgrabAudioError> {
        if let Some(frame) = self
            .queued_frames
            .lock()
            .map_err(|_| WgrabAudioError::BufferUnavailable)?
            .pop_front()
        {
            return Ok(Some(WgrabAudioFrame::new(
                frame.format,
                frame.timestamp,
                frame.timestamp_quality,
                frame.frames,
                frame.samples,
            )));
        }

        let mut buffer = self
            .buffer
            .lock()
            .map_err(|_| WgrabAudioError::BufferUnavailable)?;

        if buffer.is_empty() {
            return Ok(None);
        }

        let samples: Vec<f32> = buffer.drain(..).collect();
        let channels = usize::from(self.format.channels);
        let frames = if channels == 0 {
            0
        } else {
            samples.len() / channels
        };
        // NOTE:
        // This prototype timestamps the frame at dequeue time. Future phases
        // should use backend-provided stream timing where available, e.g. CPAL
        // callback timestamps or platform-specific capture timestamps.
        let timestamp = Some(self.clock.now());

        Ok(Some(WgrabAudioFrame::new(
            self.format,
            timestamp,
            WgrabTimestampQuality::DequeueTime,
            frames,
            samples,
        )))
    }
}
