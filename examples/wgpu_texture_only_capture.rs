#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::process::ExitCode;

#[cfg(any(target_os = "macos", target_os = "windows"))]
use crabgrab::feature::wgpu::{
    WgpuCaptureConfig, WgpuCaptureFrame, WgpuCaptureStream, WgpuVideoFrameGpuOnlyExt,
};
#[cfg(any(target_os = "macos", target_os = "windows"))]
use crabgrab::prelude::*;

#[cfg(any(target_os = "macos", target_os = "windows"))]
struct Args {
    frames: usize,
    ci: bool,
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn parse_args() -> Result<Args, String> {
    let mut frames = 1;
    let mut ci = false;
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--frames" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--frames requires a positive integer".to_string())?;
                frames = value
                    .parse::<usize>()
                    .map_err(|error| format!("invalid --frames value: {error}"))?
                    .max(1);
            }
            "--ci" => ci = true,
            _ => return Err(format!("unknown argument: {arg}")),
        }
    }

    Ok(Args { frames, ci })
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(args) => args,
        Err(reason) => {
            println!("CI_WGRAB_TEXTURE_ONLY_FAILED");
            println!("reason={reason}");
            return ExitCode::from(1);
        }
    };

    let _config_type = std::any::type_name::<WgpuCaptureConfig>();
    let _stream_type = std::any::type_name::<WgpuCaptureStream>();
    let _frame_type = std::any::type_name::<WgpuCaptureFrame>();
    let _frame_method = <VideoFrame as WgpuVideoFrameGpuOnlyExt>::get_wgpu_capture_frame;

    if args.ci {
        println!("CI_WGRAB_TEXTURE_ONLY_UNAVAILABLE");
        println!("reason=GPU-only capture stream runtime wiring is deferred");
        println!("frames={}", args.frames);
    } else {
        println!("wgrab GPU-only texture capture skeleton is available.");
        println!(
            "runtime wiring is deferred; requested frames={}",
            args.frames
        );
    }

    ExitCode::SUCCESS
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn main() -> std::process::ExitCode {
    let is_ci = std::env::args().any(|arg| arg == "--ci");
    if is_ci {
        println!("CI_WGRAB_TEXTURE_ONLY_UNAVAILABLE");
        println!("reason=GPU-only capture is only implemented for Windows and macOS");
    } else {
        eprintln!("wgpu_texture_only_capture is implemented for Windows and macOS.");
    }
    std::process::ExitCode::SUCCESS
}
