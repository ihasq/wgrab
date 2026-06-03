use std::env;
#[cfg(target_os = "macos")]
use std::thread;
#[cfg(target_os = "macos")]
use std::time::Duration;

fn main() {
    let args = Args::parse();

    let result = std::panic::catch_unwind(|| run_smoke(args.duration_ms)).unwrap_or_else(|_| {
        SmokeResult::Failed {
            reason: "ScreenCaptureKit audio smoke panicked".to_string(),
        }
    });

    match result {
        SmokeResult::Ok {
            sample_rate,
            channels,
            frames,
            samples,
            max_abs,
            silent,
            timestamp_nanos,
            timestamp_quality,
        } => {
            println!("CI_WGRAB_AUDIO_SCK_OK");
            println!("sample_rate={sample_rate}");
            println!("channels={channels}");
            println!("frames={frames}");
            println!("samples={samples}");
            println!("max_abs={max_abs:.8}");
            println!("silent={silent}");
            println!("timestamp_nanos={timestamp_nanos}");
            println!("timestamp_quality={timestamp_quality}");
        }
        SmokeResult::Unavailable { reason } => {
            println!("CI_WGRAB_AUDIO_SCK_UNAVAILABLE");
            println!("reason={reason}");
            if !args.allow_unavailable {
                std::process::exit(1);
            }
        }
        SmokeResult::Failed { reason } => {
            println!("CI_WGRAB_AUDIO_SCK_FAILED");
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
    allow_unavailable: bool,
}

impl Args {
    fn parse() -> Self {
        let mut ci = false;
        let mut duration_ms = 250;
        let mut allow_unavailable = false;
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
                _ => {}
            }
        }

        Self {
            ci,
            duration_ms,
            allow_unavailable,
        }
    }
}

enum SmokeResult {
    Ok {
        sample_rate: u32,
        channels: u16,
        frames: usize,
        samples: usize,
        max_abs: f32,
        silent: bool,
        timestamp_nanos: String,
        timestamp_quality: String,
    },
    Unavailable {
        reason: String,
    },
    Failed {
        reason: String,
    },
}

#[cfg(target_os = "macos")]
fn run_smoke(duration_ms: u64) -> SmokeResult {
    use wgrab::feature::audio::{WgrabAudioContext, WgrabAudioError};

    let context = WgrabAudioContext::new();
    let stream = match context.build_screencapturekit_audio_stream() {
        Ok(stream) => stream,
        Err(WgrabAudioError::ScreenCaptureKitUnavailable(error))
        | Err(WgrabAudioError::UnsupportedSampleFormat(error)) => {
            return SmokeResult::Unavailable { reason: error };
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
            let max_abs = max_abs(frame.samples_f32());
            SmokeResult::Ok {
                sample_rate: format.sample_rate,
                channels: format.channels,
                frames: frame.frames(),
                samples: frame.samples_f32().len(),
                max_abs,
                silent: max_abs < 0.00001,
                timestamp_nanos: frame
                    .timestamp()
                    .map(|timestamp| timestamp.as_nanos().to_string())
                    .unwrap_or_else(|| "none".to_string()),
                timestamp_quality: format!("{:?}", frame.timestamp_quality()),
            }
        }
        Ok(None) => SmokeResult::Unavailable {
            reason: "no samples captured from ScreenCaptureKit audio".to_string(),
        },
        Err(error) => SmokeResult::Failed {
            reason: error.to_string(),
        },
    }
}

#[cfg(not(target_os = "macos"))]
fn run_smoke(_duration_ms: u64) -> SmokeResult {
    SmokeResult::Unavailable {
        reason: "non_macos_target".to_string(),
    }
}

#[cfg(target_os = "macos")]
fn max_abs(samples: &[f32]) -> f32 {
    samples
        .iter()
        .fold(0.0f32, |current, sample| current.max(sample.abs()))
}
