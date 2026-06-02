#![allow(deprecated)]

use std::sync::Arc;
use std::{error::Error, fmt::Display};

use crate::prelude::{CaptureConfig, CaptureStream, VideoFrame};

mod gpu_only;

pub use gpu_only::{
    WgpuCaptureConfig, WgpuCaptureFrame, WgpuCaptureStream, WgpuVideoFrameGpuOnlyExt,
};

#[cfg(target_os = "macos")]
use crate::feature::iosurface::{GetIoSurfaceError, IoSurface, MacosIoSurfaceVideoFrameExt};
#[cfg(target_os = "macos")]
use crate::platform::macos::{capture_stream::MacosCaptureConfig, frame::MacosVideoFrame};
#[cfg(target_os = "macos")]
use crate::platform::platform_impl::objc_wrap::CVPixelFormat;
#[cfg(target_os = "macos")]
use objc2_06::{rc::Retained, runtime::ProtocolObject};
#[cfg(target_os = "macos")]
use objc2_metal::{MTLCPUCacheMode, MTLDevice, MTLPixelFormat, MTLResource, MTLStorageMode, MTLTexture, MTLTextureDescriptor, MTLTextureType, MTLTextureUsage};
#[cfg(target_os = "macos")]
use wgpu::hal::metal as hal_mtl;
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{HMODULE, WAIT_OBJECT_0};
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{CloseHandle, GENERIC_ALL};
#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_UNKNOWN, D3D_FEATURE_LEVEL_11_0};
#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Direct3D11::{D3D11CreateDevice, ID3D11Device5, ID3D11DeviceContext4, ID3D11Fence, D3D11_CREATE_DEVICE_DEBUG, D3D11_SDK_VERSION, D3D11_TEXTURE2D_DESC};
#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Direct3D12::{D3D12_CLEAR_VALUE, D3D12_FENCE_FLAG_SHARED};
#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Direct3D12::{ID3D12Fence, D3D12_CPU_PAGE_PROPERTY_UNKNOWN, D3D12_HEAP_FLAG_SHARED, D3D12_HEAP_PROPERTIES, D3D12_HEAP_TYPE_DEFAULT, D3D12_MEMORY_POOL_UNKNOWN, D3D12_RESOURCE_DESC, D3D12_RESOURCE_DIMENSION_TEXTURE2D, D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS, D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS, D3D12_RESOURCE_STATE_COMMON, D3D12_TEXTURE_LAYOUT_UNKNOWN};
#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory, IDXGIAdapter4, IDXGIFactory5};
#[cfg(target_os = "windows")]
use windows::Win32::System::Threading::{CreateEventA, WaitForSingleObjectEx, INFINITE};

#[cfg(target_os = "windows")]
use crate::platform::windows::capture_stream::WindowsCaptureConfig;
#[cfg(target_os = "windows")]
use crate::feature::dx11::*;
#[cfg(target_os = "windows")]
use windows::{core::Interface, Graphics::DirectX::DirectXPixelFormat, Win32::Graphics::{Direct3D11::ID3D11Texture2D, Direct3D11::D3D11_CREATE_DEVICE_BGRA_SUPPORT, Direct3D12::{ID3D12Resource, D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET}}};

#[cfg(target_os = "macos")]
type Objc2MetalDevice = Retained<ProtocolObject<dyn MTLDevice>>;

#[cfg(target_os = "macos")]
type Objc2MetalTexture = Retained<ProtocolObject<dyn MTLTexture>>;

/// A capture config which can be supplied with a Wgpu device
pub trait WgpuCaptureConfigExt: Sized {
    fn with_wgpu_device(self, device: Arc<dyn AsRef<wgpu::Device> + Send + Sync + 'static>) -> Result<Self, String>;
}

impl WgpuCaptureConfigExt for CaptureConfig {
    /// Supply a Wgpu device to the config, allowing the generation of Wgpu textures from video frames
    fn with_wgpu_device(self, wgpu_device: Arc<dyn AsRef<wgpu::Device> + Send + Sync + 'static>) -> Result<Self, String> {
        #[cfg(target_os = "macos")]
        {
            let wgpu_device_ref = AsRef::<wgpu::Device>::as_ref(&*wgpu_device);
            let hal_device = get_metal_hal_device(wgpu_device_ref)
                .map_err(|error| error.to_string())?;
            drop(hal_device);
            Ok(Self {
                impl_capture_config: MacosCaptureConfig {
                    wgpu_device: Some(wgpu_device.clone()),
                    ..self.impl_capture_config
                },
                ..self
            })
        }
        #[cfg(target_os = "windows")]
        {
            unsafe {
                let wgpu_device_ref = AsRef::<wgpu::Device>::as_ref(&*wgpu_device);
                let hal_device = get_dx12_hal_device(wgpu_device_ref)
                    .map_err(|error| error.to_string())?;
                let d3d12_device = hal_device.raw_device();
                let adapter_luid = d3d12_device.GetAdapterLuid();
                let dxgi_factory: IDXGIFactory5 = CreateDXGIFactory()
                    .map_err(|error| format!("Failed to create dxgi factory: {}", error.to_string()))?;
                let dxgi_adapter = dxgi_factory.EnumAdapterByLuid(adapter_luid)
                    .map_err(|error| format!("Failed to find matching dxgi adapter for wgpu device: {}", error.to_string()))
                    .map(|dxgi_adapter: IDXGIAdapter4| dxgi_adapter)?;
                let dxgi_adapter = dxgi_adapter.cast::<IDXGIAdapter4>().unwrap();
                let mut d3d11_device = None;
                D3D11CreateDevice (
                    &dxgi_adapter,
                    D3D_DRIVER_TYPE_UNKNOWN,
                    HMODULE::default(),
                    D3D11_CREATE_DEVICE_BGRA_SUPPORT | D3D11_CREATE_DEVICE_DEBUG,
                    Some(&[D3D_FEATURE_LEVEL_11_0]),
                    D3D11_SDK_VERSION,
                    Some(&mut d3d11_device),
                    None,
                    None
                ).map_err(|error| format!("Failed to create d3d11 device from dxgi adapter: {}", error.to_string()))?;
                let d3d11_device = d3d11_device.unwrap();
                drop(hal_device);
                Ok(Self {
                    impl_capture_config: WindowsCaptureConfig {
                        d3d11_device: Some(d3d11_device),
                        wgpu_device: Some(wgpu_device),
                        dxgi_adapter: Some(dxgi_adapter),
                        ..self.impl_capture_config
                    },
                    ..self
                })
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Identifies planes of a video frame
pub enum WgpuVideoFramePlaneTexture {
     /// The single RGBA plane for an RGBA format frame
     Rgba,
     /// The Luminance (Y, brightness) plane for a YCbCr format frame
     Luminance,
     /// The Chrominance (CbCr, Blue/Red) plane for a YCbCr format frame
     Chroma
}


/// Represents an error getting the texture from a video frame
#[derive(Clone, Debug)]
pub enum WgpuVideoFrameError {
    /// the backend texture couldn't be fetched
    NoBackendTexture,
    /// The requested plane isn't valid for this frame
    InvalidVideoPlaneTexture,
    /// No Wgpu device was supplied to the capture stream
    NoWgpuDevice,
    /// The supplied Wgpu device is not using the required backend
    WrongWgpuBackend {
        expected: &'static str,
        actual: wgpu::Backend,
    },
    Other(String)
}


impl Display for WgpuVideoFrameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoBackendTexture => f.write_str("WgpuVideoFrameError::NoBackendTexture"),
            Self::InvalidVideoPlaneTexture => f.write_str("WgpuVideoFrameError::InvalidVideoPlaneTexture"),
            Self::NoWgpuDevice => f.write_str("WgpuVideoFrameError::NoWgpuDevice"),
            Self::WrongWgpuBackend { expected, actual } => f.write_fmt(format_args!(
                "wgpu device is not using the required backend: expected {}, actual {:?}",
                expected,
                actual
            )),
            Self::Other(error) => f.write_fmt(format_args!("WgpuVideoFrameError::Other(\"{}\")", error)),
        }
    }
}

impl Error for WgpuVideoFrameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[cfg(target_os = "windows")]
fn get_dx12_hal_device(
    device: &wgpu::Device,
) -> Result<
    impl std::ops::Deref<Target = wgpu::hal::dx12::Device> + '_,
    WgpuVideoFrameError,
> {
    let actual = device.adapter_info().backend;

    // SAFETY:
    // We only borrow the underlying HAL device through wgpu's guard.
    // We do not destroy the raw HAL device, and the guard is kept alive
    // for the duration of all raw handle accesses in the caller's scope.
    unsafe {
        device
            .as_hal::<wgpu::hal::api::Dx12>()
            .ok_or(WgpuVideoFrameError::WrongWgpuBackend {
                expected: "Dx12",
                actual,
            })
    }
}

#[cfg(target_os = "macos")]
fn get_metal_hal_device(
    device: &wgpu::Device,
) -> Result<
    impl std::ops::Deref<Target = hal_mtl::Device> + '_,
    WgpuVideoFrameError,
> {
    let actual = device.adapter_info().backend;

    // SAFETY:
    // We only borrow the underlying HAL device through wgpu's guard.
    // We do not destroy the raw HAL device, and the guard is kept alive
    // for the duration of all raw handle accesses in the caller's scope.
    unsafe {
        device
            .as_hal::<wgpu::hal::api::Metal>()
            .ok_or(WgpuVideoFrameError::WrongWgpuBackend {
                expected: "Metal",
                actual,
            })
    }
}

#[cfg(target_os = "macos")]
fn metal_pixel_format_to_wgpu(
    pixel_format: MTLPixelFormat,
) -> Result<wgpu::TextureFormat, WgpuVideoFrameError> {
    match pixel_format {
        MTLPixelFormat::BGRA8Unorm => Ok(wgpu::TextureFormat::Bgra8Unorm),
        MTLPixelFormat::BGRA8Unorm_sRGB => Ok(wgpu::TextureFormat::Bgra8UnormSrgb),
        MTLPixelFormat::RGBA8Sint => Ok(wgpu::TextureFormat::Rgba8Sint),
        MTLPixelFormat::RGBA8Uint => Ok(wgpu::TextureFormat::Rgba8Uint),
        MTLPixelFormat::RGBA8Unorm => Ok(wgpu::TextureFormat::Rgba8Unorm),
        MTLPixelFormat::RGBA8Unorm_sRGB => Ok(wgpu::TextureFormat::Rgba8UnormSrgb),
        MTLPixelFormat::RGBA8Snorm => Ok(wgpu::TextureFormat::Rgba8Snorm),
        MTLPixelFormat::RGB10A2Uint => Ok(wgpu::TextureFormat::Rgb10a2Uint),
        MTLPixelFormat::RGB10A2Unorm => Ok(wgpu::TextureFormat::Rgb10a2Unorm),
        MTLPixelFormat::RG8Sint => Ok(wgpu::TextureFormat::Rg8Sint),
        MTLPixelFormat::RG8Snorm => Ok(wgpu::TextureFormat::Rg8Snorm),
        MTLPixelFormat::RG8Uint => Ok(wgpu::TextureFormat::Rg8Uint),
        MTLPixelFormat::RG8Unorm => Ok(wgpu::TextureFormat::Rg8Unorm),
        MTLPixelFormat::R8Sint => Ok(wgpu::TextureFormat::R8Sint),
        MTLPixelFormat::R8Snorm => Ok(wgpu::TextureFormat::R8Snorm),
        MTLPixelFormat::R8Uint => Ok(wgpu::TextureFormat::R8Uint),
        MTLPixelFormat::R8Unorm => Ok(wgpu::TextureFormat::R8Unorm),
        _ => Err(WgpuVideoFrameError::Other(format!(
            "Unsupported Metal pixel format: {pixel_format:?}"
        ))),
    }
}

#[cfg(target_os = "macos")]
fn metal_texture_dimension(
    texture_type: MTLTextureType,
) -> Result<wgpu::TextureDimension, WgpuVideoFrameError> {
    match texture_type {
        MTLTextureType::Type2D | MTLTextureType::Type2DMultisample => Ok(wgpu::TextureDimension::D2),
        _ => Err(WgpuVideoFrameError::Other(format!(
            "Unsupported Metal texture type: {texture_type:?}"
        ))),
    }
}

#[cfg(target_os = "macos")]
fn metal_texture_usage_to_wgpu(
    usage: MTLTextureUsage,
    storage_mode: MTLStorageMode,
) -> wgpu::TextureUsages {
    let render_usage = if usage.contains(MTLTextureUsage::RenderTarget) {
        wgpu::TextureUsages::RENDER_ATTACHMENT
    } else {
        wgpu::TextureUsages::empty()
    };
    let shader_read_usage = if usage.contains(MTLTextureUsage::ShaderRead) {
        wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING
    } else {
        wgpu::TextureUsages::empty()
    };
    let shader_write_usage = if usage.contains(MTLTextureUsage::ShaderWrite) {
        wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING
    } else {
        wgpu::TextureUsages::empty()
    };
    let copy_usage = match storage_mode {
        MTLStorageMode::Managed | MTLStorageMode::Private | MTLStorageMode::Shared => {
            wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC
        }
        MTLStorageMode::Memoryless => wgpu::TextureUsages::empty(),
        _ => wgpu::TextureUsages::empty(),
    };
    render_usage | shader_read_usage | shader_write_usage | copy_usage
}

#[cfg(target_os = "macos")]
fn objc2_iosurface_ref(io_surface: &IoSurface) -> &objc2_io_surface::IOSurfaceRef {
    // SAFETY:
    // `IoSurface` stores a valid IOSurfaceRef and holds an IOSurface use-count
    // for the duration of this borrow. This does not transfer ownership.
    unsafe { &*(io_surface.get_raw() as *const objc2_io_surface::IOSurfaceRef) }
}

#[cfg(target_os = "macos")]
fn objc2_metal_texture_from_iosurface(
    device: &Objc2MetalDevice,
    io_surface: &IoSurface,
    plane: WgpuVideoFramePlaneTexture,
) -> Result<(Objc2MetalTexture, usize), WgpuVideoFrameError> {
    let pixel_format = io_surface
        .get_pixel_format()
        .ok_or_else(|| WgpuVideoFrameError::Other("Unable to get pixel format from IOSurface".to_string()))?;

    let (plane_index, metal_pixel_format) = match pixel_format {
        CVPixelFormat::BGRA8888 => match plane {
            WgpuVideoFramePlaneTexture::Rgba => (0, MTLPixelFormat::BGRA8Unorm),
            _ => return Err(WgpuVideoFrameError::InvalidVideoPlaneTexture),
        },
        CVPixelFormat::V420 | CVPixelFormat::F420 => match plane {
            WgpuVideoFramePlaneTexture::Luminance => (0, MTLPixelFormat::R8Uint),
            WgpuVideoFramePlaneTexture::Chroma => (1, MTLPixelFormat::RG8Uint),
            _ => return Err(WgpuVideoFrameError::InvalidVideoPlaneTexture),
        },
        _ => {
            return Err(WgpuVideoFrameError::Other(format!(
                "Unsupported IOSurface pixel format: {pixel_format:?}"
            )))
        }
    };

    let width = if plane_index == 0 {
        io_surface.get_width()
    } else {
        io_surface.get_width_of_plane(plane_index)
    };
    let height = if plane_index == 0 {
        io_surface.get_height()
    } else {
        io_surface.get_height_of_plane(plane_index)
    };

    let descriptor = MTLTextureDescriptor::new();
    descriptor.setTextureType(MTLTextureType::Type2D);
    descriptor.setPixelFormat(metal_pixel_format);
    unsafe {
        descriptor.setWidth(width);
        descriptor.setHeight(height);
        descriptor.setSampleCount(1);
        descriptor.setMipmapLevelCount(1);
    }
    descriptor.setStorageMode(MTLStorageMode::Shared);
    descriptor.setCpuCacheMode(MTLCPUCacheMode::DefaultCache);

    device
        .newTextureWithDescriptor_iosurface_plane(
            &descriptor,
            objc2_iosurface_ref(io_surface),
            plane_index,
        )
        .map(|texture| (texture, plane_index))
        .ok_or_else(|| WgpuVideoFrameError::Other("Failed to create Metal texture from IOSurface".to_string()))
}

#[cfg(target_os = "macos")]
fn should_debug_macos_wgpu_descriptor() -> bool {
    cfg!(feature = "wgpu-debug-descriptor")
        || (cfg!(debug_assertions) && std::env::var_os("CRABGRAB_WGPU_DEBUG_DESCRIPTOR").is_some())
}

#[cfg(target_os = "macos")]
fn debug_macos_wgpu_descriptor(
    io_surface: &IoSurface,
    plane_index: usize,
    metal_texture: &Objc2MetalTexture,
    descriptor: &wgpu::TextureDescriptor<'_>,
) -> Result<(), WgpuVideoFrameError> {
    if !should_debug_macos_wgpu_descriptor() {
        return Ok(());
    }

    let io_surface_plane_width = if plane_index == 0 {
        io_surface.get_width()
    } else {
        io_surface.get_width_of_plane(plane_index)
    };
    let io_surface_plane_height = if plane_index == 0 {
        io_surface.get_height()
    } else {
        io_surface.get_height_of_plane(plane_index)
    };
    let io_surface_plane_bytes_per_row = if io_surface.get_plane_count() == 0 {
        io_surface.get_bytes_per_row()
    } else {
        io_surface.get_bytes_per_row_of_plane(plane_index)
    };
    let mapped_format = metal_pixel_format_to_wgpu(metal_texture.pixelFormat())?;

    println!("CI_DESCRIPTOR_DUMP_BEGIN");
    println!("CrabGrab wgpu descriptor validation:");
    println!("  IOSurface:");
    println!("    width: {}", io_surface.get_width());
    println!("    height: {}", io_surface.get_height());
    println!("    pixel format: {:?}", io_surface.get_pixel_format());
    println!("    plane count: {}", io_surface.get_plane_count());
    println!("    selected plane: {plane_index}");
    println!("    plane width: {io_surface_plane_width}");
    println!("    plane height: {io_surface_plane_height}");
    println!("    plane bytes per row: {io_surface_plane_bytes_per_row}");
    println!("  Metal texture:");
    println!("    width: {}", metal_texture.width());
    println!("    height: {}", metal_texture.height());
    println!("    depth: {}", metal_texture.depth());
    println!("    array length: {}", metal_texture.arrayLength());
    println!("    mipmap level count: {}", metal_texture.mipmapLevelCount());
    println!("    sample count: {}", metal_texture.sampleCount());
    println!("    texture type: {:?}", metal_texture.textureType());
    println!("    pixel format: {:?}", metal_texture.pixelFormat());
    println!("    usage: {:?}", metal_texture.usage());
    println!("    storage mode: {:?}", metal_texture.storageMode());
    println!("  wgpu descriptor:");
    println!("    width: {}", descriptor.size.width);
    println!("    height: {}", descriptor.size.height);
    println!("    depth_or_array_layers: {}", descriptor.size.depth_or_array_layers);
    println!("    mip_level_count: {}", descriptor.mip_level_count);
    println!("    sample_count: {}", descriptor.sample_count);
    println!("    dimension: {:?}", descriptor.dimension);
    println!("    format: {:?}", descriptor.format);
    println!("    usage: {:?}", descriptor.usage);
    println!("    view_formats: {:?}", descriptor.view_formats);
    println!("CI_DESCRIPTOR_DUMP_END");

    assert_eq!(metal_texture.width() as u32, descriptor.size.width);
    assert_eq!(metal_texture.height() as u32, descriptor.size.height);
    assert_eq!(metal_texture.mipmapLevelCount().max(1) as u32, descriptor.mip_level_count);
    assert_eq!(metal_texture.sampleCount().max(1) as u32, descriptor.sample_count);
    assert_eq!(mapped_format, descriptor.format);
    assert_eq!(io_surface_plane_width as u32, descriptor.size.width);
    assert_eq!(io_surface_plane_height as u32, descriptor.size.height);

    Ok(())
}

/// A video frame which can be used to create Wgpu textures
pub trait WgpuVideoFrameExt {
    /// Get the texture for the given plane of the video frame
    fn get_wgpu_texture(&self, plane: WgpuVideoFramePlaneTexture, label: Option<&'static str>) -> Result<wgpu::Texture, WgpuVideoFrameError>;
}

impl WgpuVideoFrameExt for VideoFrame {
    fn get_wgpu_texture(&self, plane: WgpuVideoFramePlaneTexture, label: Option<&'static str>) -> Result<wgpu::Texture, WgpuVideoFrameError> {
        #[cfg(target_os = "macos")]
        {
            let wgpu_device = match &self.impl_video_frame {
                MacosVideoFrame::SCStream(sc_stream_frame) => sc_stream_frame.wgpu_device.clone(),
                MacosVideoFrame::CGDisplayStream(cg_display_stream_frame) => cg_display_stream_frame.wgpu_device.clone(),
            }.ok_or(WgpuVideoFrameError::NoWgpuDevice)?;
            let io_surface = MacosIoSurfaceVideoFrameExt::get_iosurface(self)
                .map_err(|error| match error {
                    GetIoSurfaceError::NoImageBuffer | GetIoSurfaceError::NoIoSurface => {
                        WgpuVideoFrameError::NoBackendTexture
                    }
                })?;
            let hal_device = get_metal_hal_device((&*wgpu_device).as_ref())?;
            let (metal_texture, plane_index) = objc2_metal_texture_from_iosurface(
                hal_device.raw_device(),
                &io_surface,
                plane,
            )?;
            let texture_type = metal_texture.textureType();
            let format = metal_pixel_format_to_wgpu(metal_texture.pixelFormat())?;
            let descriptor = wgpu::TextureDescriptor {
                label,
                size: wgpu::Extent3d {
                    width: metal_texture.width() as u32,
                    height: metal_texture.height() as u32,
                    depth_or_array_layers: metal_texture.arrayLength().max(1) as u32,
                },
                mip_level_count: metal_texture.mipmapLevelCount().max(1) as u32,
                sample_count: metal_texture.sampleCount().max(1) as u32,
                dimension: metal_texture_dimension(texture_type)?,
                format,
                usage: metal_texture_usage_to_wgpu(metal_texture.usage(), metal_texture.storageMode()),
                view_formats: &[],
            };
            debug_macos_wgpu_descriptor(&io_surface, plane_index, &metal_texture, &descriptor)?;
            let wgpu_metal_texture = unsafe {
                hal_mtl::Device::texture_from_raw(
                    metal_texture.clone(),
                    descriptor.format,
                    texture_type,
                    metal_texture.arrayLength().max(1) as u32,
                    metal_texture.mipmapLevelCount().max(1) as u32,
                    wgpu::hal::CopyExtent {
                        width: metal_texture.width() as u32,
                        height: metal_texture.height() as u32,
                        depth: metal_texture.depth() as u32,
                    },
                )
            };
            // SAFETY:
            // - `wgpu_metal_texture` was created from a Metal texture associated with the
            //   same Metal device that backs this wgpu Device.
            // - `descriptor` matches the raw Metal texture's size, mip count,
            //   sample count, dimension, and format.
            // - The underlying capture pipeline has fully initialized the texture
            //   before it is wrapped by wgpu.
            Ok(unsafe {
                (&*wgpu_device)
                    .as_ref()
                    .create_texture_from_hal::<wgpu::hal::api::Metal>(wgpu_metal_texture, &descriptor)
            })
        }
        #[cfg(target_os = "windows")]
        {
            if plane != WgpuVideoFramePlaneTexture::Rgba {
                return Err(WgpuVideoFrameError::InvalidVideoPlaneTexture);
            }
            let wgpu_device = self.impl_video_frame.wgpu_device.as_ref()
                .ok_or(WgpuVideoFrameError::NoWgpuDevice)?.clone();
            let d3d11_5_device = self.impl_video_frame.device.cast::<ID3D11Device5>()
                .map_err(|error| WgpuVideoFrameError::Other(format!("Device is incompatible with resource sharing interface: {}", error)))?;
            let (frame_texture, pixel_format) = WindowsDx11VideoFrame::get_dx11_texture(self)
                .map_err(|_| WgpuVideoFrameError::NoBackendTexture)?;
            
            let wgpu_format = match pixel_format {
                DirectXPixelFormat::B8G8R8A8Typeless => wgpu::TextureFormat::Bgra8Unorm,
                DirectXPixelFormat::B8G8R8A8UIntNormalized => wgpu::TextureFormat::Bgra8Unorm,
                DirectXPixelFormat::B8G8R8A8UIntNormalizedSrgb => wgpu::TextureFormat::Bgra8UnormSrgb,
                DirectXPixelFormat::R10G10B10A2Typeless => wgpu::TextureFormat::Rgb10a2Uint,
                DirectXPixelFormat::R10G10B10A2UInt => wgpu::TextureFormat::Rgb10a2Uint,
                DirectXPixelFormat::R10G10B10A2UIntNormalized => wgpu::TextureFormat::Rgb10a2Unorm,
                DirectXPixelFormat::R16G16B16A16Float => wgpu::TextureFormat::Rgba16Float,
                _ => return Err(WgpuVideoFrameError::Other("Unsupported DirectXPixelFormat".to_string()))
            };
            unsafe {
                {
                    let wgpu_dx12_device = get_dx12_hal_device(AsRef::as_ref(&*wgpu_device))?;
                    let d3d12_device = wgpu_dx12_device.raw_device();
                    let d3d12_queue = wgpu_dx12_device.raw_queue();

                    let mut frame_desc = D3D11_TEXTURE2D_DESC::default();
                    frame_texture.GetDesc(&mut frame_desc as *mut _);

                    let wgpu_size = wgpu::Extent3d {
                        width: frame_desc.Width,
                        height: frame_desc.Height,
                        depth_or_array_layers: frame_desc.ArraySize,
                    };

                    let d3d12_texture_desc = D3D12_RESOURCE_DESC {
                        Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
                        Alignment: 0,
                        Width: frame_desc.Width as u64,
                        Height: frame_desc.Height,
                        DepthOrArraySize: frame_desc.ArraySize as u16,
                        MipLevels: frame_desc.MipLevels as u16,
                        Format: frame_desc.Format,
                        SampleDesc: frame_desc.SampleDesc,
                        Layout: D3D12_TEXTURE_LAYOUT_UNKNOWN,
                        Flags: D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET | D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS | D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS
                    };
                    let d3d12_texture_heap_properties = D3D12_HEAP_PROPERTIES {
                        Type: D3D12_HEAP_TYPE_DEFAULT,
                        CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
                        MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
                        CreationNodeMask: 0,
                        VisibleNodeMask: 0,
                    };  
                    let d3d12_texture_clear_value = D3D12_CLEAR_VALUE {
                        Format: frame_desc.Format,
                        Anonymous: windows::Win32::Graphics::Direct3D12::D3D12_CLEAR_VALUE_0 {
                            Color: [0.0, 0.0, 0.0, 0.0]
                        }
                    };

                    let mut d3d12_texture = None;
                    d3d12_device.CreateCommittedResource(
                        &d3d12_texture_heap_properties as *const _,
                        D3D12_HEAP_FLAG_SHARED,
                        &d3d12_texture_desc as *const _,
                        D3D12_RESOURCE_STATE_COMMON,
                        Some(&d3d12_texture_clear_value),
                        &mut d3d12_texture as *mut _
                    ).map_err(|error| WgpuVideoFrameError::Other(format!("Failed to create d3d12 texture: {}", error.to_string())))?;
                    let d3d12_texture: ID3D12Resource = d3d12_texture.unwrap();

                    let dxgi_shared_texture_handle = d3d12_device.CreateSharedHandle(
                        &d3d12_texture,
                        None,
                        GENERIC_ALL.0,
                        None
                    ).map_err(|error| WgpuVideoFrameError::Other(format!("Failed to share d3d12 texture: {}", error.to_string())))?;

                    let d3d11_shared_texture: ID3D11Texture2D = d3d11_5_device.OpenSharedResource1(dxgi_shared_texture_handle)
                    .map_err(|error| WgpuVideoFrameError::Other(format!("Failed to use dxgi shared texture in d3d11: {}", error.to_string())))?;

                    let d3d12_fence: ID3D12Fence = d3d12_device.CreateFence(0, D3D12_FENCE_FLAG_SHARED)
                        .map_err(|error|  WgpuVideoFrameError::Other(format!("Failed to create fence: {}", error)))?;
                    let fence_event = CreateEventA(None, false, false, None)
                        .map_err(|error|  WgpuVideoFrameError::Other(format!("Failed to create fence event: {}", error)))?;
                    d3d12_fence.SetEventOnCompletion(1, fence_event)
                        .map_err(|error|  WgpuVideoFrameError::Other(format!("Failed to set fence completion event: {}", error.to_string())))?;

                    let dxgi_shared_fence_handle = d3d12_device.CreateSharedHandle(
                        &d3d12_fence,
                        None,
                        GENERIC_ALL.0,
                        None
                    ).map_err(|error| WgpuVideoFrameError::Other(format!("Failed to share fence with dxgi: {}", error.to_string())))?;

                    let mut d3d11_shared_fence = None;
                    d3d11_5_device.OpenSharedFence(dxgi_shared_fence_handle, &mut d3d11_shared_fence)
                        .map_err(|error| WgpuVideoFrameError::Other(format!("Failed to use dxgi shared fence: {}", error.to_string())))?;
                    let d3d11_shared_fence: ID3D11Fence = d3d11_shared_fence.unwrap();

                    {
                        let device_context: ID3D11DeviceContext4 = self.impl_video_frame.device.GetImmediateContext()
                            .map_err(|error| WgpuVideoFrameError::Other(format!("Failed to get d3d11 device context: {}", error.to_string())))?
                            .cast()
                            .map_err(|error| WgpuVideoFrameError::Other(format!("Failed to get d3d11 device context v4: {}", error.to_string())))?;
                        device_context.CopyResource(&d3d11_shared_texture, &frame_texture);
                        device_context.Signal(&d3d11_shared_fence, 1)
                            .map_err(|error| WgpuVideoFrameError::Other(format!("Failed to queue fence signal: {}", error.to_string())))?;
                        drop(frame_texture);
                        drop(d3d11_shared_texture);
                        drop(d3d11_shared_fence);
                        device_context.Flush();
                    }

                    CloseHandle(dxgi_shared_texture_handle)
                        .map_err(|error| WgpuVideoFrameError::Other(format!("Failed to close shared texture handle: {}", error.to_string())))?;

                    let hal_texture = wgpu::hal::dx12::Device::texture_from_raw(
                        d3d12_texture.clone(),
                        wgpu_format,
                        wgpu::TextureDimension::D2,
                        wgpu_size,
                        frame_desc.MipLevels.max(1),
                        frame_desc.SampleDesc.Count
                    );

                    d3d12_queue.Wait(&d3d12_fence, 1)
                        .map_err(|error| WgpuVideoFrameError::Other(format!("Failed to enqueue wait on fence: {}", error.to_string())))?;

                    if WaitForSingleObjectEx(fence_event, INFINITE, false) != WAIT_OBJECT_0 {
                        Err(WgpuVideoFrameError::Other(format!("Failed wait on completion fence")))?
                    }

                    CloseHandle(dxgi_shared_fence_handle)
                        .map_err(|error| WgpuVideoFrameError::Other(format!("Failed to close shared fence handle: {}", error.to_string())))?;
                    CloseHandle(fence_event)
                        .map_err(|error| WgpuVideoFrameError::Other(format!("Failed to close fence event handle: {}", error.to_string())))?;
                    
                    let desc = wgpu::TextureDescriptor {
                        label,
                        size: wgpu_size,
                        mip_level_count: frame_desc.MipLevels.max(1),
                        sample_count: frame_desc.SampleDesc.Count,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu_format,
                        usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
                        view_formats: &[wgpu_format]
                    };
                    // SAFETY:
                    // - `hal_texture` was created from a D3D12 resource opened on the same
                    //   D3D12 device that backs this wgpu Device.
                    // - `desc` matches the D3D12 resource size, mip count, sample count,
                    //   dimension, and format.
                    // - The D3D11 -> D3D12 copy and fence wait have completed before
                    //   the resource is wrapped by wgpu.
                    let result = Ok((*wgpu_device).as_ref().create_texture_from_hal::<wgpu::hal::api::Dx12>(hal_texture, &desc));

                    result
                }
            }
        }
    }
}

/// A capture stream which may have had a Wgpu device instance supplied to it
pub trait WgpuCaptureStreamExt {
    /// Gets the Wgpu device wrapper supplied to `CaptureConfig::with_wgpu_device(..)`
    fn get_wgpu_device_wrapper(&self) -> Option<Arc<dyn AsRef<wgpu::Device> + Send + Sync + 'static>>;
    /// Gets the Wgpu device referenced by device wrapper supplied to `CaptureConfig::with_wgpu_device(..)`
    fn get_wgpu_device(&self) -> Option<&wgpu::Device>;
}

impl WgpuCaptureStreamExt for CaptureStream {
    fn get_wgpu_device(&self) -> Option<&wgpu::Device> {
        #[cfg(target_os = "macos")]
        { self.impl_capture_stream.wgpu_device.as_ref().map(|wgpu_device| AsRef::<wgpu::Device>::as_ref(wgpu_device.as_ref())) }
        #[cfg(target_os = "windows")]
        { self.impl_capture_stream.wgpu_device.as_ref().map(|wgpu_device| AsRef::<wgpu::Device>::as_ref(wgpu_device.as_ref())) }
    }

    fn get_wgpu_device_wrapper(&self) -> Option<Arc<dyn AsRef<wgpu::Device> + Send + Sync + 'static>> {
        #[cfg(target_os = "macos")]
        { self.impl_capture_stream.wgpu_device.clone() }
        #[cfg(target_os = "windows")]
        { self.impl_capture_stream.wgpu_device.clone() }
    }
}
