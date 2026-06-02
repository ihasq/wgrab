#![cfg(target_os = "macos")]
#![cfg(feature = "iosurface")]

use std::os::raw::c_void;

use std::error::Error;
use std::fmt::Display;

use crate::{
    platform::{
        macos::{frame::MacosVideoFrame, objc_wrap::IOSurfaceRef},
        platform_impl::objc_wrap::{
            CVPixelFormat, IOSurface as PlatformIoSurface, IOSurfaceDecrementUseCount,
            IOSurfaceIncrementUseCount,
        },
    },
    prelude::VideoFrame,
};

pub(crate) struct IoSurface(IOSurfaceRef);

impl IoSurface {
    pub(crate) fn as_ptr(&self) -> *const c_void {
        self.0
    }

    pub(crate) fn get_pixel_format(&self) -> Option<CVPixelFormat> {
        PlatformIoSurface(self.0).get_pixel_format()
    }

    pub(crate) fn get_width(&self) -> usize {
        PlatformIoSurface(self.0).get_width()
    }

    pub(crate) fn get_height(&self) -> usize {
        PlatformIoSurface(self.0).get_height()
    }

    pub(crate) fn get_width_of_plane(&self, plane: usize) -> usize {
        PlatformIoSurface(self.0).get_width_of_plane(plane)
    }

    pub(crate) fn get_height_of_plane(&self, plane: usize) -> usize {
        PlatformIoSurface(self.0).get_height_of_plane(plane)
    }

    pub(crate) fn get_bytes_per_row(&self) -> usize {
        PlatformIoSurface(self.0).get_bytes_per_row()
    }

    pub(crate) fn get_bytes_per_row_of_plane(&self, plane: usize) -> usize {
        PlatformIoSurface(self.0).get_bytes_per_row_of_plane(plane)
    }

    pub(crate) fn get_plane_count(&self) -> usize {
        PlatformIoSurface(self.0).get_plane_count()
    }

    pub(crate) fn from_ref_unretained(r: IOSurfaceRef) -> Self {
        unsafe {
            IOSurfaceIncrementUseCount(r);
        }
        IoSurface(r)
    }
}

impl Clone for IoSurface {
    fn clone(&self) -> Self {
        unsafe {
            IOSurfaceIncrementUseCount(self.0);
        }
        IoSurface(self.0)
    }
}

impl Drop for IoSurface {
    fn drop(&mut self) {
        unsafe {
            IOSurfaceDecrementUseCount(self.0);
        }
    }
}

#[derive(Debug)]
/// Represents an error when getting the IOSurface behind this video frame
pub(crate) enum GetIoSurfaceError {
    /// There was no image buffer in this frame
    NoImageBuffer,
    /// There was no IOSurface in the frame's image buffer
    NoIoSurface,
}

impl Display for GetIoSurfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoImageBuffer => f.write_str("GetIoSurfaceError::NoImageBuffer"),
            Self::NoIoSurface => f.write_str("GetIoSurfaceError::NoIoSurface"),
        }
    }
}

impl Error for GetIoSurfaceError {
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

pub(crate) fn macos_frame_iosurface(frame: &VideoFrame) -> Result<IoSurface, GetIoSurfaceError> {
    match &frame.impl_video_frame {
        MacosVideoFrame::SCStream(frame) => match frame.sample_buffer.get_image_buffer() {
            Some(image_buffer) => match image_buffer.iosurface_ptr() {
                Some(ptr) => Ok(IoSurface::from_ref_unretained(ptr)),
                None => Err(GetIoSurfaceError::NoIoSurface),
            },
            None => Err(GetIoSurfaceError::NoImageBuffer),
        },
        MacosVideoFrame::CGDisplayStream(frame) => {
            Ok(IoSurface::from_ref_unretained(frame.io_surface.0))
        }
    }
}
