#![cfg(target_os = "windows")]
#![cfg(feature = "dx11")]

use windows::{
    core::Interface,
    Graphics::DirectX::{Direct3D11::IDirect3DSurface, DirectXPixelFormat},
    Win32::{
        Graphics::Direct3D11::ID3D11Texture2D,
        System::WinRT::Direct3D11::IDirect3DDxgiInterfaceAccess,
    },
};

use std::error::Error;
use std::fmt::Display;

use crate::prelude::VideoFrame;

#[derive(Debug, Clone)]
pub(crate) enum WindowsDx11VideoFrameError {
    Other(String),
}

impl Display for WindowsDx11VideoFrameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other(error) => f.write_fmt(format_args!(
                "WindowsDx11VideoFrameError::Other(\"{}\")",
                error
            )),
        }
    }
}

impl Error for WindowsDx11VideoFrameError {
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

fn windows_dx11_surface_for_video_frame(
    frame: &VideoFrame,
) -> Result<(IDirect3DSurface, DirectXPixelFormat), WindowsDx11VideoFrameError> {
    frame
        .impl_video_frame
        .frame
        .Surface()
        .map_err(|e| {
            WindowsDx11VideoFrameError::Other(format!(
                "Failed to get frame surface: {}",
                e.to_string()
            ))
        })
        .map(|surface| (surface, frame.impl_video_frame.pixel_format))
}

pub(crate) fn windows_dx11_texture_for_video_frame(
    frame: &VideoFrame,
) -> Result<(ID3D11Texture2D, DirectXPixelFormat), WindowsDx11VideoFrameError> {
    let (surface, pixel_format) = windows_dx11_surface_for_video_frame(frame)?;
    let dxgi_interface_access = surface
        .cast::<IDirect3DDxgiInterfaceAccess>()
        .map_err(|e| {
            WindowsDx11VideoFrameError::Other(format!(
                "Failed to cast surface to dxgi interface access: {}",
                e.to_string()
            ))
        })?;
    let texture =
        unsafe { dxgi_interface_access.GetInterface::<ID3D11Texture2D>() }.map_err(|e| {
            WindowsDx11VideoFrameError::Other(format!(
                "Failed to get ID3D11Texture interface {}",
                e.to_string()
            ))
        })?;
    Ok((texture, pixel_format))
}
