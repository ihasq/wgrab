use wgrab::time::{
    WgrabAvFrameQueue, WgrabAvPairStatus, WgrabAvSyncQueueConfig, WgrabTimestamp,
    WgrabTimestampQuality, WgrabTimestamped,
};

fn main() {
    let mut queue =
        WgrabAvFrameQueue::<&'static str, &'static str>::new(WgrabAvSyncQueueConfig::default());

    queue.push_audio(WgrabTimestamped::new(
        "audio-a",
        Some(WgrabTimestamp::from_nanos(1_000_000_000)),
        WgrabTimestampQuality::DequeueTime,
    ));

    queue.push_audio(WgrabTimestamped::new(
        "audio-b",
        Some(WgrabTimestamp::from_nanos(1_010_000_000)),
        WgrabTimestampQuality::DequeueTime,
    ));

    queue.push_video(WgrabTimestamped::new(
        "video",
        Some(WgrabTimestamp::from_nanos(1_008_000_000)),
        WgrabTimestampQuality::Backend,
    ));

    let peeked = queue.peek_latest_video_match().expect("match info");
    assert_eq!(peeked.pair.status, WgrabAvPairStatus::Paired);
    assert_eq!(peeked.audio_index, Some(1));

    let pair = queue.pop_latest_video_pair().expect("owned pair");
    assert_eq!(pair.video.payload, "video");
    assert_eq!(pair.audio.payload, "audio-b");
    assert_eq!(pair.pair.status, WgrabAvPairStatus::Paired);
    assert_eq!(queue.video_len(), 0);
    assert_eq!(queue.audio_len(), 1);

    queue.push_audio(WgrabTimestamped::new(
        "audio-c",
        Some(WgrabTimestamp::from_nanos(2_000_000_000)),
        WgrabTimestampQuality::DequeueTime,
    ));
    queue.push_video(WgrabTimestamped::new(
        "video-outside",
        Some(WgrabTimestamp::from_nanos(3_000_000_000)),
        WgrabTimestampQuality::Backend,
    ));

    let outside = queue.peek_latest_video_match().expect("match info");
    assert_eq!(outside.pair.status, WgrabAvPairStatus::OutsideTolerance);
    assert!(queue.pop_latest_video_pair().is_none());
    assert_eq!(queue.video_len(), 1);
    assert_eq!(queue.audio_len(), 2);

    println!("CI_WGRAB_AV_SYNC_OWNING_QUEUE_OK");
}
