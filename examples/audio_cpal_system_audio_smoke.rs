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
            candidate_score,
            candidate_reasons,
            max_abs,
            mean_abs,
            rms,
            silent,
        } => {
            println!("CI_WGRAB_AUDIO_SYSTEM_OK");
            println!("source={source}");
            println!("device_name={device_name}");
            println!("sample_rate={sample_rate}");
            println!("channels={channels}");
            println!("frames={frames}");
            println!("samples={samples}");
            println!("sample_count={samples}");
            println!("timestamp_nanos={timestamp_nanos}");
            println!("candidate_score={candidate_score}");
            println!("candidate_reasons={candidate_reasons}");
            println!("max_abs={max_abs:.8}");
            println!("mean_abs={mean_abs:.8}");
            println!("rms={rms:.8}");
            println!("silent={silent}");
            if args.strict_non_silent && silent {
                std::process::exit(1);
            }
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
    strict_non_silent: bool,
}

impl Args {
    fn parse() -> Self {
        let mut ci = false;
        let mut duration_ms = 250;
        let mut allow_unavailable = false;
        let mut strict = false;
        let mut strict_non_silent = false;
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
                "--strict-non-silent" => strict_non_silent = true,
                _ => {}
            }
        }

        Self {
            ci,
            duration_ms,
            allow_unavailable,
            strict,
            strict_non_silent,
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
        candidate_score: i32,
        candidate_reasons: String,
        max_abs: f32,
        mean_abs: f32,
        rms: f32,
        silent: bool,
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
    let (stream, candidate) = match backend.build_loopback_candidate_stream_with_candidate() {
        Ok(result) => result,
        Err(error @ WgrabAudioError::NoLoopbackCandidate)
        | Err(error @ WgrabAudioError::DefaultInputConfigFailed(_))
        | Err(error @ WgrabAudioError::BuildStreamFailed(_)) => {
            return SmokeResult::Unavailable {
                reason: match error {
                    WgrabAudioError::NoLoopbackCandidate => "no_loopback_candidate".to_string(),
                    error => error.to_string(),
                },
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
            let stats = AudioStats::from_samples(frame.samples_f32());
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
                candidate_score: candidate.score,
                candidate_reasons: candidate.reasons.join(","),
                max_abs: stats.max_abs,
                mean_abs: stats.mean_abs,
                rms: stats.rms,
                silent: stats.silent,
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

struct AudioStats {
    max_abs: f32,
    mean_abs: f32,
    rms: f32,
    silent: bool,
}

impl AudioStats {
    fn from_samples(samples: &[f32]) -> Self {
        if samples.is_empty() {
            return Self {
                max_abs: 0.0,
                mean_abs: 0.0,
                rms: 0.0,
                silent: true,
            };
        }

        let mut max_abs = 0.0f32;
        let mut abs_sum = 0.0f64;
        let mut square_sum = 0.0f64;

        for sample in samples {
            let abs = sample.abs();
            max_abs = max_abs.max(abs);
            abs_sum += f64::from(abs);
            let sample = f64::from(*sample);
            square_sum += sample * sample;
        }

        let sample_count = samples.len() as f64;
        let mean_abs = (abs_sum / sample_count) as f32;
        let rms = (square_sum / sample_count).sqrt() as f32;

        Self {
            max_abs,
            mean_abs,
            rms,
            silent: max_abs < 0.00001,
        }
    }
}
