use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::feature::audio::{
    WgrabAudioError, WgrabAudioFormat, WgrabAudioStream, WgrabSampleFormat,
};

pub struct CpalAudioBackend {
    host: cpal::Host,
}

impl CpalAudioBackend {
    pub fn default_host() -> Self {
        Self {
            host: cpal::default_host(),
        }
    }

    pub fn host_id_name(&self) -> &'static str {
        self.host.id().name()
    }

    pub fn input_device_names(&self) -> Result<Vec<String>, cpal::DevicesError> {
        self.host
            .input_devices()
            .map(|devices| devices.filter_map(|device| device_name(&device)).collect())
    }

    pub fn output_device_names(&self) -> Result<Vec<String>, cpal::DevicesError> {
        self.host
            .output_devices()
            .map(|devices| devices.filter_map(|device| device_name(&device)).collect())
    }

    pub fn default_input_device_name(&self) -> Option<String> {
        self.host
            .default_input_device()
            .and_then(|device| device_name(&device))
    }

    pub fn default_input_format(&self) -> Option<WgrabAudioFormat> {
        let device = self.host.default_input_device()?;
        let config = device.default_input_config().ok()?;

        Some(WgrabAudioFormat {
            sample_rate: config.sample_rate(),
            channels: config.channels(),
            sample_format: cpal_sample_format_to_wgrab(config.sample_format()),
        })
    }

    pub fn build_default_input_stream(&self) -> Result<WgrabAudioStream, WgrabAudioError> {
        let device = self
            .host
            .default_input_device()
            .ok_or(WgrabAudioError::NoInputDevice)?;
        let device_name = device_name(&device);
        let supported_config = device
            .default_input_config()
            .map_err(|error| WgrabAudioError::DefaultInputConfigFailed(error.to_string()))?;
        let sample_format = supported_config.sample_format();
        let stream_config = supported_config.config();
        let format = WgrabAudioFormat {
            sample_rate: stream_config.sample_rate,
            channels: stream_config.channels,
            sample_format: WgrabSampleFormat::F32,
        };
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let stream = match sample_format {
            cpal::SampleFormat::F32 => {
                build_input_stream(&device, &stream_config, Arc::clone(&buffer), append_f32)?
            }
            cpal::SampleFormat::I16 => {
                build_input_stream(&device, &stream_config, Arc::clone(&buffer), append_i16)?
            }
            cpal::SampleFormat::U16 => {
                build_input_stream(&device, &stream_config, Arc::clone(&buffer), append_u16)?
            }
            _ => {
                return Err(WgrabAudioError::UnsupportedSampleFormat(format!(
                    "{sample_format:?}"
                )));
            }
        };

        stream
            .play()
            .map_err(|error| WgrabAudioError::PlayStreamFailed(error.to_string()))?;

        Ok(WgrabAudioStream::new(
            Some(stream),
            buffer,
            format,
            device_name,
        ))
    }
}

fn device_name(device: &cpal::Device) -> Option<String> {
    device
        .description()
        .ok()
        .map(|description| description.name().to_string())
}

fn build_input_stream<T, F>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    buffer: Arc<Mutex<Vec<f32>>>,
    append: F,
) -> Result<cpal::Stream, WgrabAudioError>
where
    T: cpal::SizedSample,
    F: Fn(&[T], &mut Vec<f32>) + Send + 'static,
{
    device
        .build_input_stream(
            config,
            move |data: &[T], _| {
                if let Ok(mut samples) = buffer.lock() {
                    append(data, &mut samples);
                }
            },
            move |error| {
                eprintln!("wgrab CPAL input stream error: {error}");
            },
            None,
        )
        .map_err(|error| WgrabAudioError::BuildStreamFailed(error.to_string()))
}

fn append_f32(data: &[f32], samples: &mut Vec<f32>) {
    samples.extend_from_slice(data);
}

fn append_i16(data: &[i16], samples: &mut Vec<f32>) {
    samples.extend(data.iter().map(|sample| *sample as f32 / i16::MAX as f32));
}

fn append_u16(data: &[u16], samples: &mut Vec<f32>) {
    samples.extend(
        data.iter()
            .map(|sample| (*sample as f32 / u16::MAX as f32) * 2.0 - 1.0),
    );
}

fn cpal_sample_format_to_wgrab(format: cpal::SampleFormat) -> WgrabSampleFormat {
    match format {
        cpal::SampleFormat::F32 => WgrabSampleFormat::F32,
        cpal::SampleFormat::I16 => WgrabSampleFormat::I16,
        cpal::SampleFormat::U16 => WgrabSampleFormat::U16,
        _ => WgrabSampleFormat::Unknown,
    }
}
