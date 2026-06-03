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
}
