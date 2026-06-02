use std::env;

use wgrab::feature::audio::{CpalAudioBackend, WgrabAudioContext};

fn main() {
    let ci = env::args().any(|arg| arg == "--ci");

    match run_probe() {
        ProbeResult::Ok { device, format } => {
            println!("CI_WGRAB_AUDIO_CPAL_OK");
            println!("audio_backend=cpal");
            println!("default_input_device={device}");
            println!("sample_rate={}", format.sample_rate);
            println!("channels={}", format.channels);
            println!("sample_format={:?}", format.sample_format);
        }
        ProbeResult::Unavailable { reason } => {
            println!("CI_WGRAB_AUDIO_CPAL_UNAVAILABLE");
            println!("reason={reason}");
        }
        ProbeResult::Failed { reason } => {
            println!("CI_WGRAB_AUDIO_CPAL_FAILED");
            println!("reason={reason}");
            if ci {
                std::process::exit(1);
            }
        }
    }
}

enum ProbeResult {
    Ok {
        device: String,
        format: wgrab::feature::audio::WgrabAudioFormat,
    },
    Unavailable {
        reason: String,
    },
    Failed {
        reason: String,
    },
}

fn run_probe() -> ProbeResult {
    let context = WgrabAudioContext::new();
    let backend = context.cpal_backend();

    println!("audio_host={}", backend.host_id_name());

    match backend.input_device_names() {
        Ok(devices) => {
            println!("input_device_count={}", devices.len());
        }
        Err(error) => {
            return ProbeResult::Unavailable {
                reason: format!("input device enumeration unavailable: {error}"),
            };
        }
    }

    match backend.output_device_names() {
        Ok(devices) => {
            println!("output_device_count={}", devices.len());
        }
        Err(error) => {
            println!("output_device_count=unknown");
            println!("output_device_error={error}");
        }
    }

    let Some(device) = backend.default_input_device_name() else {
        return ProbeResult::Unavailable {
            reason: "default input device unavailable".to_string(),
        };
    };

    let Some(format) = backend.default_input_format() else {
        return ProbeResult::Unavailable {
            reason: "default input format unavailable".to_string(),
        };
    };

    ProbeResult::Ok { device, format }
}
