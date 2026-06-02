#[cfg(feature = "audio-cpal")]
mod cpal_backend;

#[cfg(feature = "audio-cpal")]
pub use cpal_backend::CpalAudioBackend;

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
}

impl WgrabAudioFrame {
    pub fn format(&self) -> WgrabAudioFormat {
        self.format
    }

    pub fn timestamp(&self) -> Option<WgrabTimestamp> {
        self.timestamp
    }

    pub fn frames(&self) -> usize {
        self.frames
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

pub struct WgrabAudioStream;
