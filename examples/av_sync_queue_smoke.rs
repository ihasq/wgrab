use wgrab::time::{
    WgrabAvPairStatus, WgrabAvSyncQueue, WgrabAvSyncQueueConfig, WgrabTimestamp,
    WgrabTimestampQuality,
};

fn main() {
    let mut queue = WgrabAvSyncQueue::new(WgrabAvSyncQueueConfig::default());

    queue.push_audio_timestamp(
        Some(WgrabTimestamp::from_nanos(1_000_000_000)),
        WgrabTimestampQuality::DequeueTime,
    );

    queue.push_audio_timestamp(
        Some(WgrabTimestamp::from_nanos(1_010_000_000)),
        WgrabTimestampQuality::DequeueTime,
    );

    queue.push_video_timestamp(
        Some(WgrabTimestamp::from_nanos(1_008_000_000)),
        WgrabTimestampQuality::Backend,
    );

    let matched = queue.match_latest_video().expect("expected latest video");
    assert_eq!(matched.pair.status, WgrabAvPairStatus::Paired);
    assert_eq!(matched.pair.delta_nanos, Some(2_000_000));
    assert_eq!(matched.audio_index, Some(1));

    queue.push_video_timestamp(None, WgrabTimestampQuality::Unavailable);
    let missing_video = queue.match_latest_video().expect("expected latest video");
    assert_eq!(
        missing_video.pair.status,
        WgrabAvPairStatus::MissingVideoTimestamp
    );
    assert_eq!(missing_video.audio_index, None);

    let mut small_queue = WgrabAvSyncQueue::new(WgrabAvSyncQueueConfig {
        max_audio_frames: 1,
        max_video_frames: 1,
        ..WgrabAvSyncQueueConfig::default()
    });

    small_queue.push_audio_timestamp(
        Some(WgrabTimestamp::from_nanos(1_000_000_000)),
        WgrabTimestampQuality::DequeueTime,
    );
    small_queue.push_audio_timestamp(
        Some(WgrabTimestamp::from_nanos(1_010_000_000)),
        WgrabTimestampQuality::DequeueTime,
    );
    small_queue.push_video_timestamp(
        Some(WgrabTimestamp::from_nanos(1_008_000_000)),
        WgrabTimestampQuality::Backend,
    );

    assert_eq!(small_queue.audio_len(), 1);
    assert_eq!(small_queue.video_len(), 1);
    assert_eq!(
        small_queue
            .match_latest_video()
            .expect("expected latest video")
            .audio_index,
        Some(0)
    );

    println!("CI_WGRAB_AV_SYNC_QUEUE_OK");
}
