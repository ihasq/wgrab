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

    pub fn as_nanos(self) -> u64 {
        self.nanos
    }
}

pub fn timestamp_delta_abs_nanos(a: WgrabTimestamp, b: WgrabTimestamp) -> u64 {
    a.as_nanos().abs_diff(b.as_nanos())
}
