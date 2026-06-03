use std::env;

use wgrab::feature::audio::WgrabAudioContext;

fn main() {
    let args = Args::parse();

    let result = std::panic::catch_unwind(|| run_probe(args.try_stream)).unwrap_or_else(|_| {
        ProbeResult::Failed {
            reason: "probe panicked".to_string(),
        }
    });

    match result {
        ProbeResult::Ok {
            device_count,
            loopback_candidate_count,
            lines,
        } => {
            println!("CI_WGRAB_AUDIO_LOOPBACK_PROBE_OK");
            println!("device_count={device_count}");
            println!("loopback_candidate_count={loopback_candidate_count}");
            for line in lines {
                println!("{line}");
            }
        }
        ProbeResult::Unavailable { reason } => {
            println!("CI_WGRAB_AUDIO_LOOPBACK_PROBE_UNAVAILABLE");
            println!("reason={reason}");
        }
        ProbeResult::Failed { reason } => {
            println!("CI_WGRAB_AUDIO_LOOPBACK_PROBE_FAILED");
            println!("reason={reason}");
            if args.ci {
                std::process::exit(1);
            }
        }
    }
}

struct Args {
    ci: bool,
    try_stream: bool,
}

impl Args {
    fn parse() -> Self {
        let mut ci = false;
        let mut try_stream = false;

        for arg in env::args().skip(1) {
            match arg.as_str() {
                "--ci" => ci = true,
                "--try-stream" => try_stream = true,
                _ => {}
            }
        }

        Self { ci, try_stream }
    }
}

enum ProbeResult {
    Ok {
        device_count: usize,
        loopback_candidate_count: usize,
        lines: Vec<String>,
    },
    Unavailable {
        reason: String,
    },
    Failed {
        reason: String,
    },
}

fn run_probe(try_stream: bool) -> ProbeResult {
    let context = WgrabAudioContext::new();
    let backend = context.cpal_backend();
    let reports = backend.device_reports();

    if reports.is_empty() {
        return ProbeResult::Unavailable {
            reason: "no CPAL audio devices were reported".to_string(),
        };
    }

    let candidates = backend.loopback_candidates();

    let mut lines = Vec::new();
    for (index, candidate) in candidates.iter().enumerate() {
        let report = &candidate.report;
        lines.push(format!("candidate[{index}].name={}", report.name));
        lines.push(format!("candidate[{index}].score={}", candidate.score));
        for reason in &candidate.reasons {
            lines.push(format!("candidate[{index}].reason={reason}"));
        }
        lines.push(format!(
            "candidate[{index}].is_default_input={}",
            report.is_default_input
        ));
        lines.push(format!(
            "candidate[{index}].is_default_output={}",
            report.is_default_output
        ));
        lines.push(format!(
            "candidate[{index}].supports_input={}",
            report.supports_input
        ));
        lines.push(format!(
            "candidate[{index}].supports_output={}",
            report.supports_output
        ));
        lines.push(format!(
            "candidate[{index}].input_config_count={}",
            report.input_config_count
        ));
        lines.push(format!(
            "candidate[{index}].output_config_count={}",
            report.output_config_count
        ));
        lines.push(format!(
            "candidate[{index}].default_input_format={}",
            format_audio_format(report.default_input_format)
        ));
        lines.push(format!(
            "candidate[{index}].default_output_format={}",
            format_audio_format(report.default_output_format)
        ));

        if try_stream {
            match backend.probe_input_stream_for_device_name(&report.name) {
                Ok(format) => {
                    lines.push(format!("candidate[{index}].stream_probe=ok"));
                    lines.push(format!(
                        "candidate[{index}].sample_rate={}",
                        format.sample_rate
                    ));
                    lines.push(format!("candidate[{index}].channels={}", format.channels));
                    lines.push(format!(
                        "candidate[{index}].sample_format={:?}",
                        format.sample_format
                    ));
                }
                Err(error) => {
                    lines.push(format!("candidate[{index}].stream_probe=failed"));
                    lines.push(format!("candidate[{index}].stream_error={error}"));
                }
            }
        }
    }

    ProbeResult::Ok {
        device_count: reports.len(),
        loopback_candidate_count: candidates.len(),
        lines,
    }
}

fn format_audio_format(format: Option<wgrab::feature::audio::WgrabAudioFormat>) -> String {
    format
        .map(|format| {
            format!(
                "sample_rate:{},channels:{},sample_format:{:?}",
                format.sample_rate, format.channels, format.sample_format
            )
        })
        .unwrap_or_else(|| "none".to_string())
}
