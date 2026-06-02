use cpal::traits::{DeviceTrait, HostTrait};

use crate::feature::audio::{WgrabAudioFormat, WgrabSampleFormat};

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
            .map(|devices| devices.filter_map(device_name).collect())
    }

    pub fn output_device_names(&self) -> Result<Vec<String>, cpal::DevicesError> {
        self.host
            .output_devices()
            .map(|devices| devices.filter_map(device_name).collect())
    }

    pub fn default_input_device_name(&self) -> Option<String> {
        self.host.default_input_device().and_then(device_name)
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
}

fn device_name(device: cpal::Device) -> Option<String> {
    device
        .description()
        .ok()
        .map(|description| description.name().to_string())
}

fn cpal_sample_format_to_wgrab(format: cpal::SampleFormat) -> WgrabSampleFormat {
    match format {
        cpal::SampleFormat::F32 => WgrabSampleFormat::F32,
        cpal::SampleFormat::I16 => WgrabSampleFormat::I16,
        cpal::SampleFormat::U16 => WgrabSampleFormat::U16,
        _ => WgrabSampleFormat::Unknown,
    }
}
