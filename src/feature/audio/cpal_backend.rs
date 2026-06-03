use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::feature::audio::{
    WgrabAudioDeviceReport, WgrabAudioError, WgrabAudioFormat, WgrabAudioStream, WgrabSampleFormat,
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
        let (stream, buffer, format) = build_stream_for_device(&device)?;

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

    pub fn device_reports(&self) -> Vec<WgrabAudioDeviceReport> {
        let default_input_name = self.default_input_device_name();
        let default_output_name = self.default_output_device_name();
        let mut reports = Vec::new();

        if let Ok(devices) = self.host.input_devices() {
            for device in devices {
                if let Some(name) = device_name(&device) {
                    merge_device_report(
                        &mut reports,
                        &name,
                        default_input_name.as_deref() == Some(name.as_str()),
                        default_output_name.as_deref() == Some(name.as_str()),
                        supports_input(&device),
                        false,
                    );
                }
            }
        }

        if let Ok(devices) = self.host.output_devices() {
            for device in devices {
                if let Some(name) = device_name(&device) {
                    merge_device_report(
                        &mut reports,
                        &name,
                        default_input_name.as_deref() == Some(name.as_str()),
                        default_output_name.as_deref() == Some(name.as_str()),
                        false,
                        supports_output(&device),
                    );
                }
            }
        }

        for report in &mut reports {
            report.loopback_candidate = loopback_candidate(report);
        }

        reports.sort_by(|left, right| left.name.cmp(&right.name));
        reports
    }

    pub fn default_output_device_name(&self) -> Option<String> {
        self.host
            .default_output_device()
            .and_then(|device| device_name(&device))
    }

    pub fn probe_input_stream_for_device_name(
        &self,
        name: &str,
    ) -> Result<WgrabAudioFormat, WgrabAudioError> {
        for device in self.devices_matching_name(name) {
            if let Ok((stream, _buffer, format)) = build_stream_for_device(&device) {
                drop(stream);
                return Ok(format);
            }
        }

        Err(WgrabAudioError::BuildStreamFailed(format!(
            "no input stream could be built for device name '{name}'"
        )))
    }

    fn devices_matching_name(&self, name: &str) -> Vec<cpal::Device> {
        let mut devices = Vec::new();

        if let Ok(input_devices) = self.host.input_devices() {
            devices.extend(
                input_devices.filter(|device| device_name(device).as_deref() == Some(name)),
            );
        }

        if let Ok(output_devices) = self.host.output_devices() {
            devices.extend(
                output_devices.filter(|device| device_name(device).as_deref() == Some(name)),
            );
        }

        devices
    }
}

fn build_stream_for_device(
    device: &cpal::Device,
) -> Result<(cpal::Stream, Arc<Mutex<Vec<f32>>>, WgrabAudioFormat), WgrabAudioError> {
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

    Ok((stream, buffer, format))
}

fn device_name(device: &cpal::Device) -> Option<String> {
    device
        .description()
        .ok()
        .map(|description| description.name().to_string())
}

fn merge_device_report(
    reports: &mut Vec<WgrabAudioDeviceReport>,
    name: &str,
    is_default_input: bool,
    is_default_output: bool,
    supports_input: bool,
    supports_output: bool,
) {
    if let Some(report) = reports.iter_mut().find(|report| report.name == name) {
        report.is_default_input |= is_default_input;
        report.is_default_output |= is_default_output;
        report.supports_input |= supports_input;
        report.supports_output |= supports_output;
        return;
    }

    reports.push(WgrabAudioDeviceReport {
        name: name.to_string(),
        is_default_input,
        is_default_output,
        supports_input,
        supports_output,
        loopback_candidate: false,
    });
}

fn supports_input(device: &cpal::Device) -> bool {
    device
        .supported_input_configs()
        .map(|mut configs| configs.next().is_some())
        .unwrap_or(false)
}

fn supports_output(device: &cpal::Device) -> bool {
    device
        .supported_output_configs()
        .map(|mut configs| configs.next().is_some())
        .unwrap_or(false)
}

fn loopback_candidate(report: &WgrabAudioDeviceReport) -> bool {
    let name = report.name.to_ascii_lowercase();
    name.contains("loopback")
        || name.contains("monitor")
        || name.contains("what u hear")
        || name.contains("stereo mix")
        || name.contains("output")
        || name.contains("speaker")
        || (report.is_default_output && report.supports_input)
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
