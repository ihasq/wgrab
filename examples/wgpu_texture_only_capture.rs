//! GPU-only capture example.
//!
//! This example intentionally does not read captured pixels back to CPU memory.

#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::process::ExitCode;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::sync::Arc;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::time::Duration;

#[cfg(any(target_os = "macos", target_os = "windows"))]
use futures::{channel::mpsc, executor::block_on, StreamExt as _};
#[cfg(any(target_os = "macos", target_os = "windows"))]
use wgrab::feature::wgpu::{
    WgpuCaptureConfigExt as _, WgpuCaptureFrame, WgpuVideoFrameGpuOnlyExt as _,
    WgpuVideoFramePlaneTexture,
};
#[cfg(any(target_os = "macos", target_os = "windows"))]
use wgrab::prelude::*;

#[cfg(any(target_os = "macos", target_os = "windows"))]
struct Args {
    frames: usize,
    ci: bool,
    allow_capture_unavailable: bool,
    strict_capture: bool,
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
struct Gfx {
    device: wgpu::Device,
    _queue: wgpu::Queue,
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
impl AsRef<wgpu::Device> for Gfx {
    fn as_ref(&self) -> &wgpu::Device {
        &self.device
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn parse_args() -> Result<Args, String> {
    let mut frames = 1;
    let mut ci = false;
    let mut allow_capture_unavailable = false;
    let mut strict_capture = false;
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
            "--allow-capture-unavailable" => allow_capture_unavailable = true,
            "--strict-capture" => strict_capture = true,
            _ => return Err(format!("unknown argument: {arg}")),
        }
    }

    Ok(Args {
        frames,
        ci,
        allow_capture_unavailable,
        strict_capture,
    })
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn capture_unavailable(args: &Args, reason: impl AsRef<str>) -> Result<(), String> {
    let reason = reason.as_ref();
    if args.ci && args.allow_capture_unavailable && !args.strict_capture {
        println!("CI_WGRAB_TEXTURE_ONLY_UNAVAILABLE");
        println!("reason={reason}");
        Ok(())
    } else {
        Err(reason.to_string())
    }
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

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| block_on(run(&args))));
    match result {
        Ok(Ok(())) => ExitCode::SUCCESS,
        Ok(Err(reason)) => {
            if args.ci {
                println!("CI_WGRAB_TEXTURE_ONLY_FAILED");
                println!("reason={reason}");
            } else {
                eprintln!("GPU-only texture capture failed: {reason}");
            }
            ExitCode::from(1)
        }
        Err(payload) => {
            let reason = panic_reason(payload);
            if args.ci {
                println!("CI_WGRAB_TEXTURE_ONLY_FAILED");
                println!("reason=panic: {reason}");
            } else {
                eprintln!("GPU-only texture capture panicked: {reason}");
            }
            ExitCode::from(1)
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn panic_reason(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(reason) = payload.downcast_ref::<&'static str>() {
        (*reason).to_string()
    } else if let Some(reason) = payload.downcast_ref::<String>() {
        reason.clone()
    } else {
        "unknown panic payload".to_string()
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
async fn run(args: &Args) -> Result<(), String> {
    let token = match CaptureStream::test_access(false) {
        Some(token) => token,
        None => match CaptureStream::request_access(false).await {
            Some(token) => token,
            None => {
                capture_unavailable(args, "capture access denied or unavailable")?;
                return Ok(());
            }
        },
    };

    let mut instance_desc = wgpu::InstanceDescriptor::new_without_display_handle();
    #[cfg(target_os = "windows")]
    {
        instance_desc.backends = wgpu::Backends::DX12;
    }
    #[cfg(target_os = "macos")]
    {
        instance_desc.backends = wgpu::Backends::METAL;
    }
    instance_desc.flags = wgpu::InstanceFlags::VALIDATION;

    let instance = wgpu::Instance::new(instance_desc);
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .map_err(|error| format!("request_adapter failed: {error:?}"));
    let adapter = match adapter {
        Ok(adapter) => adapter,
        Err(reason) => {
            capture_unavailable(args, reason)?;
            return Ok(());
        }
    };
    println!("wgpu adapter: {:#?}", adapter.get_info());

    let device_result = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("wgrab-texture-only-capture-device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: wgpu::MemoryHints::MemoryUsage,
            trace: wgpu::Trace::Off,
        })
        .await
        .map_err(|error| format!("request_device failed: {error:?}"));
    let (device, queue) = match device_result {
        Ok(device_and_queue) => device_and_queue,
        Err(reason) => {
            capture_unavailable(args, reason)?;
            return Ok(());
        }
    };
    device.on_uncaptured_error(Arc::new(|error| {
        eprintln!("wgpu uncaptured error: {error:?}");
    }));

    let gfx = Arc::new(Gfx {
        device,
        _queue: queue,
    });
    let content = match CapturableContent::new(CapturableContentFilter::DISPLAYS).await {
        Ok(content) => content,
        Err(error) => {
            capture_unavailable(args, format!("capturable content failed: {error:?}"))?;
            return Ok(());
        }
    };
    let display = match content.displays().next() {
        Some(display) => display,
        None => {
            capture_unavailable(args, "expected at least one capturable display")?;
            return Ok(());
        }
    };
    let config = CaptureConfig::with_display(display, CapturePixelFormat::Bgra8888)
        .with_wgpu_device(gfx.clone())
        .map_err(|error| format!("with_wgpu_device failed: {error}"))?;

    let (tx, mut rx) = mpsc::unbounded::<Result<VideoFrame, String>>();
    let mut stream = match CaptureStream::new(token, config, move |event_result| match event_result
    {
        Ok(StreamEvent::Video(frame)) => {
            let _ = tx.unbounded_send(Ok(frame));
        }
        Ok(StreamEvent::End) => {
            let _ = tx.unbounded_send(Err("capture stream ended".to_string()));
        }
        Ok(_) => {}
        Err(error) => {
            let _ = tx.unbounded_send(Err(format!("stream error: {error:?}")));
        }
    }) {
        Ok(stream) => stream,
        Err(error) => {
            capture_unavailable(args, format!("capture stream failed: {error:?}"))?;
            return Ok(());
        }
    };

    let mut last_frame = None;
    for frame_index in 0..args.frames {
        let frame = match rx.next().await {
            Some(Ok(frame)) => frame,
            Some(Err(reason)) => {
                capture_unavailable(args, reason)?;
                return Ok(());
            }
            None => {
                capture_unavailable(args, "capture stream closed before frame arrived")?;
                return Ok(());
            }
        };

        let gpu_frame = frame
            .get_wgpu_capture_frame(
                WgpuVideoFramePlaneTexture::Rgba,
                Some("wgrab texture-only capture frame"),
            )
            .map_err(|error| format!("get_wgpu_capture_frame failed: {error}"))?;
        let view = gpu_frame.create_view(None);
        report_frame(frame_index + 1, &gpu_frame, true);
        drop(view);
        last_frame = Some(gpu_frame);
    }

    stream
        .stop()
        .map_err(|error| format!("stream stop failed: {error:?}"))?;
    std::thread::sleep(Duration::from_millis(100));

    if args.ci {
        println!("CI_WGRAB_TEXTURE_ONLY_OK");
        println!("frame_count={}", args.frames);
        if let Some(frame) = &last_frame {
            let size = frame.size();
            println!("texture_width={}", size.width);
            println!("texture_height={}", size.height);
            println!("texture_format={:?}", frame.format());
            println!("texture_usage={:?}", frame.usage());
            println!("timestamp_nanos={}", timestamp_nanos(frame));
            println!("view_created=true");
        }
    } else {
        println!(
            "wgrab GPU-only texture capture: OK ({} frame(s))",
            args.frames
        );
    }

    Ok(())
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn report_frame(frame_index: usize, frame: &WgpuCaptureFrame, view_created: bool) {
    let _texture = frame.texture();
    let size = frame.size();
    println!(
        "GPU-only frame {frame_index} OK: texture_width={}, texture_height={}, texture_format={:?}, texture_usage={:?}, timestamp_nanos={}, view_created={view_created}",
        size.width,
        size.height,
        frame.format(),
        frame.usage(),
        timestamp_nanos(frame)
    );
    if std::env::var_os("CRABGRAB_WGPU_DEBUG_DESCRIPTOR").is_some() {
        println!("CI_DESCRIPTOR_DUMP_BEGIN");
        println!("wgpu descriptor:");
        println!("  width={}", size.width);
        println!("  height={}", size.height);
        println!("  depth_or_array_layers={}", size.depth_or_array_layers);
        println!("  format={:?}", frame.format());
        println!("  usage={:?}", frame.usage());
        println!("CI_DESCRIPTOR_DUMP_END");
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn timestamp_nanos(frame: &WgpuCaptureFrame) -> String {
    frame
        .timestamp()
        .map(|timestamp| timestamp.as_nanos().to_string())
        .unwrap_or_else(|| "none".to_string())
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
