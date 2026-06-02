#[cfg(feature = "audio-cpal")]
mod cpal_backend;

#[cfg(feature = "audio-cpal")]
pub use cpal_backend::CpalAudioBackend;

use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WgrabSampleFormat {
    F32,
    I16,
    U16,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WgrabAudioFormat {
    pub sample_rate: u32,
    pub channels: u16,
    pub sample_format: WgrabSampleFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WgrabTimestamp {
    // Placeholder; a concrete clock model will be designed later.
    pub nanos: u64,
}

pub struct WgrabAudioFrame {
    format: WgrabAudioFormat,
    timestamp: Option<WgrabTimestamp>,
    frames: usize,
    samples: Vec<f32>,
}

impl WgrabAudioFrame {
    pub(crate) fn new(
        format: WgrabAudioFormat,
        timestamp: Option<WgrabTimestamp>,
        frames: usize,
        samples: Vec<f32>,
    ) -> Self {
        Self {
            format,
            timestamp,
            frames,
            samples,
        }
    }

    pub fn format(&self) -> WgrabAudioFormat {
        self.format
    }

    pub fn timestamp(&self) -> Option<WgrabTimestamp> {
        self.timestamp
    }

    pub fn frames(&self) -> usize {
        self.frames
    }

    pub fn samples_f32(&self) -> &[f32] {
        &self.samples
    }
}

pub struct WgrabAudioContext;

impl WgrabAudioContext {
    pub fn new() -> Self {
        Self
    }

    #[cfg(feature = "audio-cpal")]
    pub fn cpal_backend(&self) -> CpalAudioBackend {
        CpalAudioBackend::default_host()
    }
}

impl Default for WgrabAudioContext {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum WgrabAudioError {
    NoInputDevice,
    DefaultInputConfigFailed(String),
    BuildStreamFailed(String),
    PlayStreamFailed(String),
    UnsupportedSampleFormat(String),
    BufferUnavailable,
}

impl fmt::Display for WgrabAudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoInputDevice => write!(f, "no default input device is available"),
            Self::DefaultInputConfigFailed(error) => {
                write!(f, "default input config failed: {error}")
            }
            Self::BuildStreamFailed(error) => write!(f, "build input stream failed: {error}"),
            Self::PlayStreamFailed(error) => write!(f, "play input stream failed: {error}"),
            Self::UnsupportedSampleFormat(format) => {
                write!(f, "unsupported sample format: {format}")
            }
            Self::BufferUnavailable => write!(f, "audio sample buffer is unavailable"),
        }
    }
}

impl std::error::Error for WgrabAudioError {}

pub struct WgrabAudioStream {
    // CPAL streams stop capturing when dropped, so this field intentionally
    // owns the live stream for the lifetime of the prototype stream object.
    #[cfg(feature = "audio-cpal")]
    #[allow(dead_code)]
    pub(crate) stream: Option<cpal::Stream>,
    pub(crate) buffer: Arc<Mutex<Vec<f32>>>,
    format: WgrabAudioFormat,
    device_name: Option<String>,
}

impl WgrabAudioStream {
    pub(crate) fn new(
        #[cfg(feature = "audio-cpal")] stream: Option<cpal::Stream>,
        buffer: Arc<Mutex<Vec<f32>>>,
        format: WgrabAudioFormat,
        device_name: Option<String>,
    ) -> Self {
        Self {
            #[cfg(feature = "audio-cpal")]
            stream,
            buffer,
            format,
            device_name,
        }
    }

    pub fn format(&self) -> WgrabAudioFormat {
        self.format
    }

    pub fn device_name(&self) -> Option<&str> {
        self.device_name.as_deref()
    }

    pub fn try_next_frame(&self) -> Result<Option<WgrabAudioFrame>, WgrabAudioError> {
        let mut buffer = self
            .buffer
            .lock()
            .map_err(|_| WgrabAudioError::BufferUnavailable)?;

        if buffer.is_empty() {
            return Ok(None);
        }

        let samples: Vec<f32> = buffer.drain(..).collect();
        let channels = usize::from(self.format.channels);
        let frames = if channels == 0 {
            0
        } else {
            samples.len() / channels
        };

        Ok(Some(WgrabAudioFrame::new(
            self.format,
            None,
            frames,
            samples,
        )))
    }
}
