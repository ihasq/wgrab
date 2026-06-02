#[cfg(target_os = "macos")]
use std::sync::Arc;
#[cfg(target_os = "macos")]
use std::time::Duration;

#[cfg(target_os = "macos")]
use crabgrab::feature::wgpu::{
    WgpuCaptureConfigExt as _, WgpuVideoFrameExt as _, WgpuVideoFramePlaneTexture,
};
#[cfg(target_os = "macos")]
use crabgrab::prelude::*;
#[cfg(target_os = "macos")]
use futures::{channel::mpsc, executor::block_on, StreamExt as _};

#[cfg(target_os = "macos")]
struct Gfx {
    device: wgpu::Device,
    _queue: wgpu::Queue,
}

#[cfg(target_os = "macos")]
impl AsRef<wgpu::Device> for Gfx {
    fn as_ref(&self) -> &wgpu::Device {
        &self.device
    }
}

#[cfg(target_os = "macos")]
fn frame_count_from_args() -> usize {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--frames" {
            if let Some(value) = args.next() {
                return value.parse().unwrap_or(1);
            }
        }
    }
    1
}

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    block_on(async {
        let frames_to_capture = frame_count_from_args().max(1);

        let token = match CaptureStream::test_access(false) {
            Some(token) => token,
            None => CaptureStream::request_access(false)
                .await
                .ok_or("capture access denied")?,
        };

        let mut instance_desc = wgpu::InstanceDescriptor::new_without_display_handle();
        instance_desc.backends = wgpu::Backends::METAL;
        instance_desc.flags = wgpu::InstanceFlags::VALIDATION;
        let instance = wgpu::Instance::new(instance_desc);
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .map_err(|error| format!("request_adapter failed: {error:?}"))?;
        println!("wgpu adapter: {:#?}", adapter.get_info());

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("crabgrab-wgpu-capture-smoke-device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
                trace: wgpu::Trace::Off,
            })
            .await
            .map_err(|error| format!("request_device failed: {error:?}"))?;
        device.on_uncaptured_error(Arc::new(|error| {
            eprintln!("wgpu uncaptured error: {error:?}");
        }));

        let gfx = Arc::new(Gfx {
            device,
            _queue: queue,
        });
        let content = CapturableContent::new(CapturableContentFilter::DISPLAYS)
            .await
            .map_err(|error| format!("capturable content failed: {error:?}"))?;
        let display = content
            .displays()
            .next()
            .ok_or("expected at least one capturable display")?;
        let config = CaptureConfig::with_display(display, CapturePixelFormat::Bgra8888)
            .with_wgpu_device(gfx.clone())
            .map_err(|error| format!("with_wgpu_device failed: {error}"))?;

        let (tx, mut rx) = mpsc::unbounded::<Result<VideoFrame, String>>();
        let mut stream = CaptureStream::new(token, config, move |event_result| match event_result {
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
        })
        .map_err(|error| format!("capture stream failed: {error:?}"))?;

        for frame_index in 0..frames_to_capture {
            let frame = rx
                .next()
                .await
                .ok_or("capture stream closed before frame arrived")?
                .map_err(|error| format!("capture frame failed: {error}"))?;
            let texture = frame
                .get_wgpu_texture(WgpuVideoFramePlaneTexture::Rgba, Some("crabgrab wgpu smoke"))
                .map_err(|error| format!("get_wgpu_texture failed: {error}"))?;
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            println!(
                "wgpu capture smoke frame {} OK: size={:?}, format={:?}",
                frame_index + 1,
                texture.size(),
                texture.format()
            );
            drop(view);
            drop(texture);
        }

        stream
            .stop()
            .map_err(|error| format!("stream stop failed: {error:?}"))?;
        std::thread::sleep(Duration::from_millis(100));
        println!("wgpu capture smoke: OK ({frames_to_capture} frame(s))");
        Ok(())
    })
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("wgpu_capture_smoke is a macOS runtime validation example.");
}
