use std::process::ExitCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Backend {
    Dx12,
    Metal,
    Vulkan,
}

impl Backend {
    fn name(self) -> &'static str {
        match self {
            Self::Dx12 => "dx12",
            Self::Metal => "metal",
            Self::Vulkan => "vulkan",
        }
    }

    fn wgpu_backend(self) -> wgpu::Backend {
        match self {
            Self::Dx12 => wgpu::Backend::Dx12,
            Self::Metal => wgpu::Backend::Metal,
            Self::Vulkan => wgpu::Backend::Vulkan,
        }
    }

    fn wgpu_backends(self) -> wgpu::Backends {
        match self {
            Self::Dx12 => wgpu::Backends::DX12,
            Self::Metal => wgpu::Backends::METAL,
            Self::Vulkan => wgpu::Backends::VULKAN,
        }
    }
}

#[derive(Debug)]
struct Args {
    backend: Backend,
    require_hal: bool,
}

#[derive(Debug)]
enum SmokeResult {
    Ok {
        backend: Backend,
        info: wgpu::AdapterInfo,
    },
    Unavailable {
        backend: Backend,
        reason: String,
    },
    Failed {
        backend: Backend,
        reason: String,
    },
}

impl SmokeResult {
    fn print(&self) {
        match self {
            Self::Ok { backend, info } => {
                println!("CI_BACKEND_SMOKE_OK");
                println!("backend={}", backend.name());
                println!("adapter_name={}", info.name);
                println!("adapter_backend={:?}", info.backend);
                println!("adapter_type={:?}", info.device_type);
            }
            Self::Unavailable { backend, reason } => {
                println!("CI_BACKEND_SMOKE_UNAVAILABLE");
                println!("backend={}", backend.name());
                println!("reason={reason}");
            }
            Self::Failed { backend, reason } => {
                println!("CI_BACKEND_SMOKE_FAILED");
                println!("backend={}", backend.name());
                println!("reason={reason}");
            }
        }
    }
}

fn parse_args() -> Result<Args, String> {
    let mut backend = None;
    let mut require_hal = false;
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--backend" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--backend requires dx12, metal, or vulkan".to_string())?;
                backend = Some(match value.as_str() {
                    "dx12" => Backend::Dx12,
                    "metal" => Backend::Metal,
                    "vulkan" => Backend::Vulkan,
                    _ => return Err(format!("unsupported backend: {value}")),
                });
            }
            "--require-hal" => require_hal = true,
            _ => return Err(format!("unknown argument: {arg}")),
        }
    }

    Ok(Args {
        backend: backend.ok_or_else(|| "--backend is required".to_string())?,
        require_hal,
    })
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(args) => args,
        Err(reason) => {
            SmokeResult::Failed {
                backend: Backend::Vulkan,
                reason,
            }
            .print();
            return ExitCode::from(1);
        }
    };

    let result = pollster::block_on(run(args));
    let failed = matches!(result, SmokeResult::Failed { .. });
    result.print();
    if failed {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

async fn run(args: Args) -> SmokeResult {
    let backend = args.backend;
    let mut instance_desc = wgpu::InstanceDescriptor::new_without_display_handle();
    instance_desc.backends = backend.wgpu_backends();
    instance_desc.flags = wgpu::InstanceFlags::VALIDATION;
    let instance = wgpu::Instance::new(instance_desc);

    let adapters = instance.enumerate_adapters(backend.wgpu_backends()).await;
    println!("backend={}", backend.name());
    println!("adapter_count={}", adapters.len());
    for adapter in &adapters {
        println!("adapter={:#?}", adapter.get_info());
    }

    let adapter = match instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
    {
        Ok(adapter) => adapter,
        Err(error) => {
            return SmokeResult::Unavailable {
                backend,
                reason: format!("request_adapter failed: {error:?}"),
            };
        }
    };

    let info = adapter.get_info();
    if info.backend != backend.wgpu_backend() {
        return SmokeResult::Failed {
            backend,
            reason: format!(
                "requested backend {}, got {:?}",
                backend.name(),
                info.backend
            ),
        };
    }

    let (device, _queue) = match adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("crabgrab-ci-wgpu-backend-smoke-device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: wgpu::MemoryHints::MemoryUsage,
            trace: wgpu::Trace::Off,
        })
        .await
    {
        Ok(device_queue) => device_queue,
        Err(error) => {
            return SmokeResult::Failed {
                backend,
                reason: format!("request_device failed: {error:?}"),
            };
        }
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("crabgrab-ci-wgpu-backend-smoke-texture"),
        size: wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    drop(view);
    drop(texture);

    if args.require_hal {
        match require_hal_guard(&device, backend) {
            Ok(()) => {}
            Err(reason) => {
                return SmokeResult::Failed { backend, reason };
            }
        }
    }

    SmokeResult::Ok { backend, info }
}

fn require_hal_guard(device: &wgpu::Device, backend: Backend) -> Result<(), String> {
    match backend {
        Backend::Dx12 => require_dx12_hal(device),
        Backend::Metal => require_metal_hal(device),
        Backend::Vulkan => Ok(()),
    }
}

#[cfg(target_os = "windows")]
fn require_dx12_hal(device: &wgpu::Device) -> Result<(), String> {
    // SAFETY:
    // We only borrow the underlying HAL device through wgpu's guard and
    // immediately drop the guard without touching raw handles.
    let hal = unsafe { device.as_hal::<wgpu::hal::api::Dx12>() };
    hal.map(drop)
        .ok_or_else(|| "as_hal::<Dx12>() returned None".to_string())
}

#[cfg(not(target_os = "windows"))]
fn require_dx12_hal(_device: &wgpu::Device) -> Result<(), String> {
    Err("Dx12 HAL guard is only available on Windows".to_string())
}

#[cfg(target_os = "macos")]
fn require_metal_hal(device: &wgpu::Device) -> Result<(), String> {
    // SAFETY:
    // We only borrow the underlying HAL device through wgpu's guard and
    // immediately drop the guard without touching raw handles.
    let hal = unsafe { device.as_hal::<wgpu::hal::api::Metal>() };
    hal.map(drop)
        .ok_or_else(|| "as_hal::<Metal>() returned None".to_string())
}

#[cfg(not(target_os = "macos"))]
fn require_metal_hal(_device: &wgpu::Device) -> Result<(), String> {
    Err("Metal HAL guard is only available on macOS".to_string())
}
