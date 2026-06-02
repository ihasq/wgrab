use std::fmt;

use crate::prelude::VideoFrame;

use super::{WgpuVideoFrameError, WgpuVideoFrameExt, WgpuVideoFramePlaneTexture};

/// GPU-only capture configuration.
///
/// The caller must provide a wgpu device compatible with the platform backend:
/// - Windows: DX12
/// - macOS: Metal
///
/// wgrab does not create a hidden rendering device for the capture output.
pub struct WgpuCaptureConfig {
    _private: (),
}

impl fmt::Debug for WgpuCaptureConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WgpuCaptureConfig").finish_non_exhaustive()
    }
}

/// GPU-only capture stream skeleton.
///
/// The concrete platform stream wiring will be introduced incrementally. This
/// type reserves the public API surface for texture-first capture streams.
pub struct WgpuCaptureStream {
    _private: (),
}

impl fmt::Debug for WgpuCaptureStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WgpuCaptureStream").finish_non_exhaustive()
    }
}

/// A GPU-resident captured frame.
///
/// This frame owns the `wgpu::Texture` wrapper for the captured content and
/// exposes texture metadata without exposing CPU-readable frame data.
pub struct WgpuCaptureFrame {
    texture: wgpu::Texture,
    size: wgpu::Extent3d,
    format: wgpu::TextureFormat,
    usage: wgpu::TextureUsages,
}

impl fmt::Debug for WgpuCaptureFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WgpuCaptureFrame")
            .field("size", &self.size)
            .field("format", &self.format)
            .field("usage", &self.usage)
            .finish_non_exhaustive()
    }
}

impl WgpuCaptureFrame {
    /// Returns the GPU texture for this captured frame.
    pub fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    /// Returns the captured texture size.
    pub fn size(&self) -> wgpu::Extent3d {
        self.size
    }

    /// Returns the captured texture format.
    pub fn format(&self) -> wgpu::TextureFormat {
        self.format
    }

    /// Returns the captured texture usage flags.
    pub fn usage(&self) -> wgpu::TextureUsages {
        self.usage
    }

    /// Creates a view of the captured texture.
    pub fn create_view(
        &self,
        descriptor: Option<&wgpu::TextureViewDescriptor<'_>>,
    ) -> wgpu::TextureView {
        match descriptor {
            Some(descriptor) => self.texture.create_view(descriptor),
            None => self
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
        }
    }

    pub(crate) fn from_wgpu_texture(
        texture: wgpu::Texture,
        size: wgpu::Extent3d,
        format: wgpu::TextureFormat,
        usage: wgpu::TextureUsages,
    ) -> Self {
        Self {
            texture,
            size,
            format,
            usage,
        }
    }
}

/// Extension trait for converting a captured video frame into a GPU-only frame.
pub trait WgpuVideoFrameGpuOnlyExt {
    /// Creates a GPU-only frame for the selected plane.
    fn get_wgpu_capture_frame(
        &self,
        plane: WgpuVideoFramePlaneTexture,
        label: Option<&'static str>,
    ) -> Result<WgpuCaptureFrame, WgpuVideoFrameError>;
}

impl WgpuVideoFrameGpuOnlyExt for VideoFrame {
    fn get_wgpu_capture_frame(
        &self,
        plane: WgpuVideoFramePlaneTexture,
        label: Option<&'static str>,
    ) -> Result<WgpuCaptureFrame, WgpuVideoFrameError> {
        let texture = self.get_wgpu_texture(plane, label)?;
        let size = texture.size();
        let format = texture.format();
        let usage = texture.usage();

        Ok(WgpuCaptureFrame::from_wgpu_texture(
            texture, size, format, usage,
        ))
    }
}
