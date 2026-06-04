/// Monotonic capture timestamp used for audio/video synchronization.
///
/// This timestamp is not wall-clock time. It represents elapsed time on
/// wgrab's capture timeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WgrabTimestamp {
    pub nanos: u64,
}

impl WgrabTimestamp {
    pub fn from_nanos(nanos: u64) -> Self {
        Self { nanos }
    }

    pub fn as_nanos(self) -> u64 {
        self.nanos
    }
}

/// Describes the source quality of a capture timestamp.
///
/// This is intended for A/V synchronization diagnostics. It is not a precision
/// guarantee.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WgrabTimestampQuality {
    /// Timestamp comes from backend-provided media/capture timing.
    Backend,
    /// Timestamp comes from a wgrab monotonic capture clock.
    CaptureClock,
    /// Timestamp is assigned when samples/frames are dequeued.
    DequeueTime,
    /// Timestamp is not available.
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WgrabAvSyncTolerance {
    pub nanos: u64,
}

impl WgrabAvSyncTolerance {
    pub fn from_millis(ms: u64) -> Self {
        Self {
            nanos: ms.saturating_mul(1_000_000),
        }
    }

    /// Initial tolerance candidate for 60 Hz video pairing.
    ///
    /// This is a diagnostic pairing tolerance, not a synchronization guarantee.
    pub fn video_60hz() -> Self {
        Self::from_millis(16)
    }

    /// Initial tolerance candidate for 30 Hz video pairing.
    ///
    /// This is a diagnostic pairing tolerance, not a synchronization guarantee.
    pub fn video_30hz() -> Self {
        Self::from_millis(33)
    }

    pub fn as_nanos(self) -> u64 {
        self.nanos
    }
}

pub fn timestamp_delta_abs_nanos(a: WgrabTimestamp, b: WgrabTimestamp) -> u64 {
    a.as_nanos().abs_diff(b.as_nanos())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WgrabAvPairStatus {
    Paired,
    MissingVideoTimestamp,
    MissingAudioTimestamp,
    OutsideTolerance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WgrabAvPairInfo {
    pub status: WgrabAvPairStatus,
    pub delta_nanos: Option<u64>,
    pub tolerance: WgrabAvSyncTolerance,
    pub video_timestamp_quality: WgrabTimestampQuality,
    pub audio_timestamp_quality: WgrabTimestampQuality,
}

pub fn pair_video_audio_timestamps(
    video_timestamp: Option<WgrabTimestamp>,
    video_quality: WgrabTimestampQuality,
    audio_timestamp: Option<WgrabTimestamp>,
    audio_quality: WgrabTimestampQuality,
    tolerance: WgrabAvSyncTolerance,
) -> WgrabAvPairInfo {
    match (video_timestamp, audio_timestamp) {
        (None, _) => WgrabAvPairInfo {
            status: WgrabAvPairStatus::MissingVideoTimestamp,
            delta_nanos: None,
            tolerance,
            video_timestamp_quality: video_quality,
            audio_timestamp_quality: audio_quality,
        },
        (_, None) => WgrabAvPairInfo {
            status: WgrabAvPairStatus::MissingAudioTimestamp,
            delta_nanos: None,
            tolerance,
            video_timestamp_quality: video_quality,
            audio_timestamp_quality: audio_quality,
        },
        (Some(video), Some(audio)) => {
            let delta = timestamp_delta_abs_nanos(video, audio);
            WgrabAvPairInfo {
                status: if delta <= tolerance.as_nanos() {
                    WgrabAvPairStatus::Paired
                } else {
                    WgrabAvPairStatus::OutsideTolerance
                },
                delta_nanos: Some(delta),
                tolerance,
                video_timestamp_quality: video_quality,
                audio_timestamp_quality: audio_quality,
            }
        }
    }
}
