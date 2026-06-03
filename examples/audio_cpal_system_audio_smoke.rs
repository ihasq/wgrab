use std::env;
use std::thread;
use std::time::Duration;

use wgrab::feature::audio::{WgrabAudioContext, WgrabAudioError};

fn main() {
    let args = Args::parse();

    let result = std::panic::catch_unwind(|| run_smoke(args.duration_ms)).unwrap_or_else(|_| {
        SmokeResult::Failed {
            reason: "system audio smoke panicked".to_string(),
        }
    });

    match result {
        SmokeResult::Ok {
            source,
            device_name,
            sample_rate,
            channels,
            frames,
            samples,
            timestamp_nanos,
        } => {
            println!("CI_WGRAB_AUDIO_SYSTEM_OK");
            println!("source={source}");
            println!("device_name={device_name}");
            println!("sample_rate={sample_rate}");
            println!("channels={channels}");
            println!("frames={frames}");
            println!("samples={samples}");
            println!("timestamp_nanos={timestamp_nanos}");
        }
        SmokeResult::Unavailable { reason } => {
            println!("CI_WGRAB_AUDIO_SYSTEM_UNAVAILABLE");
            println!("reason={reason}");
            if args.strict && !args.allow_unavailable {
                std::process::exit(1);
            }
        }
        SmokeResult::Failed { reason } => {
            println!("CI_WGRAB_AUDIO_SYSTEM_FAILED");
            println!("reason={reason}");
            if args.ci || args.strict {
                std::process::exit(1);
            }
        }
    }
}

struct Args {
    ci: bool,
    duration_ms: u64,
    allow_unavailable: bool,
    strict: bool,
}

impl Args {
    fn parse() -> Self {
        let mut ci = false;
        let mut duration_ms = 250;
        let mut allow_unavailable = false;
        let mut strict = false;
        let mut args = env::args().skip(1);

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--ci" => ci = true,
                "--duration-ms" => {
                    if let Some(value) = args.next() {
                        duration_ms = value.parse().unwrap_or(duration_ms);
                    }
                }
                "--allow-unavailable" => allow_unavailable = true,
                "--strict" => strict = true,
                _ => {}
            }
        }

        Self {
            ci,
            duration_ms,
            allow_unavailable,
            strict,
        }
    }
}

enum SmokeResult {
    Ok {
        source: String,
        device_name: String,
        sample_rate: u32,
        channels: u16,
        frames: usize,
        samples: usize,
        timestamp_nanos: String,
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
    let stream = match backend.build_loopback_candidate_stream() {
        Ok(stream) => stream,
        Err(error @ WgrabAudioError::NoLoopbackCandidate)
        | Err(error @ WgrabAudioError::DefaultInputConfigFailed(_))
        | Err(error @ WgrabAudioError::BuildStreamFailed(_)) => {
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
                source: format!("{:?}", stream.source()),
                device_name: stream.device_name().unwrap_or("unknown").to_string(),
                sample_rate: format.sample_rate,
                channels: format.channels,
                frames: frame.frames(),
                samples: frame.samples_f32().len(),
                timestamp_nanos: frame
                    .timestamp()
                    .map(|timestamp| timestamp.as_nanos().to_string())
                    .unwrap_or_else(|| "none".to_string()),
            }
        }
        Ok(None) => SmokeResult::Unavailable {
            reason: "no samples captured from CPAL system-audio candidate".to_string(),
        },
        Err(error) => SmokeResult::Failed {
            reason: error.to_string(),
        },
    }
}
