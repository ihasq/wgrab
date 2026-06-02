use std::env;
use std::thread;
use std::time::Duration;

use wgrab::feature::audio::{WgrabAudioContext, WgrabAudioError};

fn main() {
    let args = Args::parse();

    match run_smoke(args.duration_ms) {
        SmokeResult::Ok {
            device_name,
            sample_rate,
            channels,
            sample_format,
            frames,
            samples,
        } => {
            println!("CI_WGRAB_AUDIO_INPUT_OK");
            println!("device_name={device_name}");
            println!("sample_rate={sample_rate}");
            println!("channels={channels}");
            println!("sample_format={sample_format}");
            println!("frames={frames}");
            println!("samples={samples}");
        }
        SmokeResult::Unavailable { reason } => {
            println!("CI_WGRAB_AUDIO_INPUT_UNAVAILABLE");
            println!("reason={reason}");
        }
        SmokeResult::Failed { reason } => {
            println!("CI_WGRAB_AUDIO_INPUT_FAILED");
            println!("reason={reason}");
            if args.ci {
                std::process::exit(1);
            }
        }
    }
}

struct Args {
    ci: bool,
    duration_ms: u64,
}

impl Args {
    fn parse() -> Self {
        let mut ci = false;
        let mut duration_ms = 250;
        let mut args = env::args().skip(1);

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--ci" => ci = true,
                "--duration-ms" => {
                    if let Some(value) = args.next() {
                        duration_ms = value.parse().unwrap_or(duration_ms);
                    }
                }
                _ => {}
            }
        }

        Self { ci, duration_ms }
    }
}

enum SmokeResult {
    Ok {
        device_name: String,
        sample_rate: u32,
        channels: u16,
        sample_format: &'static str,
        frames: usize,
        samples: usize,
    },
    Unavailable {
        reason: String,
    },
    Failed {
        reason: String,
    },
}

fn run_smoke(duration_ms: u64) -> SmokeResult {
    let context = WgrabAudioContext::new();
    let backend = context.cpal_backend();
    let stream = match backend.build_default_input_stream() {
        Ok(stream) => stream,
        Err(error @ WgrabAudioError::NoInputDevice)
        | Err(error @ WgrabAudioError::DefaultInputConfigFailed(_)) => {
            return SmokeResult::Unavailable {
                reason: error.to_string(),
            };
        }
        Err(error) => {
            return SmokeResult::Failed {
                reason: error.to_string(),
            };
        }
    };

    thread::sleep(Duration::from_millis(duration_ms));

    match stream.try_next_frame() {
        Ok(Some(frame)) => {
            let format = frame.format();
            SmokeResult::Ok {
                device_name: stream.device_name().unwrap_or("unknown").to_string(),
                sample_rate: format.sample_rate,
                channels: format.channels,
                sample_format: "F32",
                frames: frame.frames(),
                samples: frame.samples_f32().len(),
            }
        }
        Ok(None) => SmokeResult::Unavailable {
            reason: "no input samples captured during smoke duration".to_string(),
        },
        Err(error) => SmokeResult::Failed {
            reason: error.to_string(),
        },
    }
}
