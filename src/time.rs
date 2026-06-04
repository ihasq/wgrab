use std::collections::VecDeque;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WgrabTimestampedFrameInfo {
    pub timestamp: Option<WgrabTimestamp>,
    pub timestamp_quality: WgrabTimestampQuality,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WgrabTimestamped<T> {
    pub payload: T,
    pub timestamp: Option<WgrabTimestamp>,
    pub timestamp_quality: WgrabTimestampQuality,
}

impl<T> WgrabTimestamped<T> {
    pub fn new(
        payload: T,
        timestamp: Option<WgrabTimestamp>,
        timestamp_quality: WgrabTimestampQuality,
    ) -> Self {
        Self {
            payload,
            timestamp,
            timestamp_quality,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WgrabAvQueueMatch {
    pub pair: WgrabAvPairInfo,
    pub audio_index: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WgrabAvOwnedQueueMatch {
    pub pair: WgrabAvPairInfo,
    pub video_index: Option<usize>,
    pub audio_index: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WgrabAvOwnedPair<V, A> {
    pub video: WgrabTimestamped<V>,
    pub audio: WgrabTimestamped<A>,
    pub pair: WgrabAvPairInfo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WgrabAvSyncQueueConfig {
    pub tolerance: WgrabAvSyncTolerance,
    pub max_video_frames: usize,
    pub max_audio_frames: usize,
}

impl Default for WgrabAvSyncQueueConfig {
    fn default() -> Self {
        Self {
            tolerance: WgrabAvSyncTolerance::video_60hz(),
            max_video_frames: 8,
            max_audio_frames: 64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WgrabAvSyncQueue {
    config: WgrabAvSyncQueueConfig,
    audio: VecDeque<WgrabTimestampedFrameInfo>,
    video: VecDeque<WgrabTimestampedFrameInfo>,
}

impl WgrabAvSyncQueue {
    pub fn new(config: WgrabAvSyncQueueConfig) -> Self {
        Self {
            config,
            audio: VecDeque::new(),
            video: VecDeque::new(),
        }
    }

    pub fn config(&self) -> WgrabAvSyncQueueConfig {
        self.config
    }

    pub fn push_audio_timestamp(
        &mut self,
        timestamp: Option<WgrabTimestamp>,
        quality: WgrabTimestampQuality,
    ) {
        self.audio.push_back(WgrabTimestampedFrameInfo {
            timestamp,
            timestamp_quality: quality,
        });
        truncate_oldest(&mut self.audio, self.config.max_audio_frames);
    }

    pub fn push_video_timestamp(
        &mut self,
        timestamp: Option<WgrabTimestamp>,
        quality: WgrabTimestampQuality,
    ) {
        self.video.push_back(WgrabTimestampedFrameInfo {
            timestamp,
            timestamp_quality: quality,
        });
        truncate_oldest(&mut self.video, self.config.max_video_frames);
    }

    pub fn audio_len(&self) -> usize {
        self.audio.len()
    }

    pub fn video_len(&self) -> usize {
        self.video.len()
    }

    pub fn match_latest_video(&self) -> Option<WgrabAvQueueMatch> {
        let video = self.video.back()?;

        let Some(video_timestamp) = video.timestamp else {
            return Some(WgrabAvQueueMatch {
                pair: pair_video_audio_timestamps(
                    None,
                    video.timestamp_quality,
                    None,
                    WgrabTimestampQuality::Unavailable,
                    self.config.tolerance,
                ),
                audio_index: None,
            });
        };

        let nearest_audio = self
            .audio
            .iter()
            .enumerate()
            .filter_map(|(index, audio)| {
                audio.timestamp.map(|audio_timestamp| {
                    (
                        index,
                        audio,
                        timestamp_delta_abs_nanos(video_timestamp, audio_timestamp),
                    )
                })
            })
            .min_by_key(|(_, _, delta)| *delta);

        match nearest_audio {
            Some((audio_index, audio, _)) => Some(WgrabAvQueueMatch {
                pair: pair_video_audio_timestamps(
                    Some(video_timestamp),
                    video.timestamp_quality,
                    audio.timestamp,
                    audio.timestamp_quality,
                    self.config.tolerance,
                ),
                audio_index: Some(audio_index),
            }),
            None => Some(WgrabAvQueueMatch {
                pair: pair_video_audio_timestamps(
                    Some(video_timestamp),
                    video.timestamp_quality,
                    None,
                    WgrabTimestampQuality::Unavailable,
                    self.config.tolerance,
                ),
                audio_index: None,
            }),
        }
    }
}

fn truncate_oldest(queue: &mut VecDeque<WgrabTimestampedFrameInfo>, max_len: usize) {
    while queue.len() > max_len {
        queue.pop_front();
    }
}

/// Prototype owning A/V frame queue.
///
/// This queue owns payloads and matches the latest video payload with the
/// nearest audio payload by timestamp.
///
/// It does not perform drift correction, resampling, scheduling, or muxing.
#[derive(Debug, Clone)]
pub struct WgrabAvFrameQueue<V, A> {
    config: WgrabAvSyncQueueConfig,
    video: VecDeque<WgrabTimestamped<V>>,
    audio: VecDeque<WgrabTimestamped<A>>,
}

impl<V, A> WgrabAvFrameQueue<V, A> {
    pub fn new(config: WgrabAvSyncQueueConfig) -> Self {
        Self {
            config,
            video: VecDeque::new(),
            audio: VecDeque::new(),
        }
    }

    pub fn config(&self) -> WgrabAvSyncQueueConfig {
        self.config
    }

    pub fn push_video(&mut self, frame: WgrabTimestamped<V>) {
        self.video.push_back(frame);
        while self.video.len() > self.config.max_video_frames {
            self.video.pop_front();
        }
    }

    pub fn push_audio(&mut self, frame: WgrabTimestamped<A>) {
        self.audio.push_back(frame);
        while self.audio.len() > self.config.max_audio_frames {
            self.audio.pop_front();
        }
    }

    pub fn video_len(&self) -> usize {
        self.video.len()
    }

    pub fn audio_len(&self) -> usize {
        self.audio.len()
    }

    pub fn peek_latest_video_match(&self) -> Option<WgrabAvOwnedQueueMatch> {
        let video_index = self.video.len().checked_sub(1)?;
        let video = self.video.get(video_index)?;

        if video.timestamp.is_none() {
            return Some(WgrabAvOwnedQueueMatch {
                pair: pair_video_audio_timestamps(
                    video.timestamp,
                    video.timestamp_quality,
                    None,
                    WgrabTimestampQuality::Unavailable,
                    self.config.tolerance,
                ),
                video_index: Some(video_index),
                audio_index: None,
            });
        }

        let mut best: Option<(usize, WgrabAvPairInfo)> = None;

        for (audio_index, audio) in self.audio.iter().enumerate() {
            let pair = pair_video_audio_timestamps(
                video.timestamp,
                video.timestamp_quality,
                audio.timestamp,
                audio.timestamp_quality,
                self.config.tolerance,
            );

            if let Some(delta) = pair.delta_nanos {
                match best {
                    None => best = Some((audio_index, pair)),
                    Some((_, existing)) => {
                        if delta < existing.delta_nanos.unwrap_or(u64::MAX) {
                            best = Some((audio_index, pair));
                        }
                    }
                }
            }
        }

        match best {
            Some((audio_index, pair)) => Some(WgrabAvOwnedQueueMatch {
                pair,
                video_index: Some(video_index),
                audio_index: Some(audio_index),
            }),
            None => Some(WgrabAvOwnedQueueMatch {
                pair: pair_video_audio_timestamps(
                    video.timestamp,
                    video.timestamp_quality,
                    None,
                    WgrabTimestampQuality::Unavailable,
                    self.config.tolerance,
                ),
                video_index: Some(video_index),
                audio_index: None,
            }),
        }
    }

    pub fn pop_latest_video_pair(&mut self) -> Option<WgrabAvOwnedPair<V, A>> {
        let matched = self.peek_latest_video_match()?;

        if matched.pair.status != WgrabAvPairStatus::Paired {
            return None;
        }

        let video_index = matched.video_index?;
        let audio_index = matched.audio_index?;

        let video = self.video.remove(video_index)?;
        let audio = self.audio.remove(audio_index)?;

        Some(WgrabAvOwnedPair {
            video,
            audio,
            pair: matched.pair,
        })
    }
}
