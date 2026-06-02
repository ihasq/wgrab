use std::{
    fmt::Debug,
    hash::Hash,
    time::{Duration, Instant},
};

use crate::{
    capturable_content::{CapturableContentError, CapturableContentFilter},
    capture_stream::{
        CaptureConfig, CapturePixelFormat, StreamCreateError, StreamError, StreamEvent,
        StreamStopError,
    },
    frame::{
        AudioBufferError, AudioCaptureFrame, AudioChannelCount, AudioChannelData, AudioSampleRate,
        VideoCaptureFrame,
    },
    util::{Rect, Size},
};

fn zero_size() -> Size {
    Size {
        width: 0.0,
        height: 0.0,
    }
}

fn zero_rect() -> Rect {
    Rect {
        origin: crate::util::Point::ZERO,
        size: zero_size(),
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ImplCapturableContentFilter;

impl ImplCapturableContentFilter {
    pub(crate) const DEFAULT: Self = Self;
    pub(crate) const NORMAL_WINDOWS: Self = Self;
}

pub(crate) struct ImplCapturableContent {
    pub(crate) windows: Vec<ImplCapturableWindow>,
    pub(crate) displays: Vec<ImplCapturableDisplay>,
}

impl ImplCapturableContent {
    pub(crate) async fn new(
        _filter: CapturableContentFilter,
    ) -> Result<Self, CapturableContentError> {
        Err(CapturableContentError::Other(
            "screen capture is only supported on Windows and macOS".to_string(),
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ImplCapturableWindow;

impl ImplCapturableWindow {
    pub(crate) fn from_impl(window: ImplCapturableWindow) -> Self {
        window
    }

    pub(crate) fn title(&self) -> String {
        String::new()
    }

    pub(crate) fn rect(&self) -> Rect {
        zero_rect()
    }

    pub(crate) fn application(&self) -> ImplCapturableApplication {
        ImplCapturableApplication
    }

    pub(crate) fn is_visible(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ImplCapturableDisplay;

impl ImplCapturableDisplay {
    pub(crate) fn from_impl(display: ImplCapturableDisplay) -> Self {
        display
    }

    pub(crate) fn rect(&self) -> Rect {
        zero_rect()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ImplCapturableApplication;

impl ImplCapturableApplication {
    pub(crate) fn identifier(&self) -> String {
        String::new()
    }

    pub(crate) fn name(&self) -> String {
        String::new()
    }

    pub(crate) fn pid(&self) -> i32 {
        0
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ImplAudioCaptureConfig;

impl ImplAudioCaptureConfig {
    pub(crate) fn new() -> Self {
        Self
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ImplCaptureConfig;

impl ImplCaptureConfig {
    pub(crate) fn new() -> Self {
        Self
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ImplCaptureAccessToken;

impl ImplCaptureAccessToken {
    pub(crate) fn allows_borderless(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub(crate) struct ImplCaptureStream;

impl ImplCaptureStream {
    pub(crate) fn supported_pixel_formats() -> &'static [CapturePixelFormat] {
        &[]
    }

    pub(crate) fn check_access(_borderless: bool) -> Option<ImplCaptureAccessToken> {
        None
    }

    pub(crate) async fn request_access(_borderless: bool) -> Option<ImplCaptureAccessToken> {
        None
    }

    pub(crate) fn new(
        _token: ImplCaptureAccessToken,
        _config: CaptureConfig,
        _callback: Box<impl FnMut(Result<StreamEvent, StreamError>) + Send + 'static>,
    ) -> Result<Self, StreamCreateError> {
        Err(StreamCreateError::Other(
            "screen capture is only supported on Windows and macOS".to_string(),
        ))
    }

    pub(crate) fn stop(&self) -> Result<(), StreamStopError> {
        Err(StreamStopError::Other(
            "screen capture is only supported on Windows and macOS".to_string(),
        ))
    }
}

pub(crate) struct ImplAudioFrame;

impl AudioCaptureFrame for ImplAudioFrame {
    fn sample_rate(&self) -> AudioSampleRate {
        AudioSampleRate::Hz8000
    }

    fn channel_count(&self) -> AudioChannelCount {
        AudioChannelCount::Mono
    }

    fn audio_channel_buffer(
        &mut self,
        _channel: usize,
    ) -> Result<AudioChannelData<'_>, AudioBufferError> {
        Err(AudioBufferError::Other(
            "screen capture is only supported on Windows and macOS".to_string(),
        ))
    }

    fn duration(&self) -> Duration {
        Duration::ZERO
    }

    fn origin_time(&self) -> Duration {
        Duration::ZERO
    }

    fn frame_id(&self) -> u64 {
        0
    }
}

pub(crate) struct ImplVideoFrame;

impl VideoCaptureFrame for ImplVideoFrame {
    fn size(&self) -> Size {
        zero_size()
    }

    fn dpi(&self) -> f64 {
        0.0
    }

    fn duration(&self) -> Duration {
        Duration::ZERO
    }

    fn origin_time(&self) -> Duration {
        Duration::ZERO
    }

    fn capture_time(&self) -> Instant {
        Instant::now()
    }

    fn frame_id(&self) -> u64 {
        0
    }

    fn content_rect(&self) -> Rect {
        zero_rect()
    }
}
