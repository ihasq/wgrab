#![cfg(target_os = "windows")]
#![cfg(feature = "dxgi")]

use crate::prelude::CaptureStream;

use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum WindowsDxgiCaptureStreamError {
    NoAdapter(String),
}

impl Display for WindowsDxgiCaptureStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoAdapter(error) => f.write_fmt(format_args!(
                "WindowsDxgiCaptureStreamError::NoAdapter(\"{}\")",
                error
            )),
        }
    }
}

impl Error for WindowsDxgiCaptureStreamError {
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

/// A capture stream which can inter-operate with DXGI
pub trait WindowsDxgiCaptureStream {
    /// Get the DXGI adapter used by the capture stream for frame generation
    fn get_dxgi_adapter(
        &self,
    ) -> Result<windows::Win32::Graphics::Dxgi::IDXGIAdapter, WindowsDxgiCaptureStreamError>;
    /// Get the DXGI device used by the capture stream for frame generation
    fn get_dxgi_device(&self) -> windows::Win32::Graphics::Dxgi::IDXGIDevice;
}

impl WindowsDxgiCaptureStream for CaptureStream {
    fn get_dxgi_adapter(
        &self,
    ) -> Result<windows::Win32::Graphics::Dxgi::IDXGIAdapter, WindowsDxgiCaptureStreamError> {
        if let Some(dxgi_adapter) = self.impl_capture_stream.dxgi_adapter.clone() {
            Ok(dxgi_adapter)
        } else {
            match &self.impl_capture_stream.dxgi_adapter_error {
                Some(error) => Err(WindowsDxgiCaptureStreamError::NoAdapter(error.clone())),
                None => unreachable!("Should have dxgi_adapter_error if dxgi_adapter is None"),
            }
        }
    }

    fn get_dxgi_device(&self) -> windows::Win32::Graphics::Dxgi::IDXGIDevice {
        self.impl_capture_stream.dxgi_device.clone()
    }
}
