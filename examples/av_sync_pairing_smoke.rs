use wgrab::time::{
    pair_video_audio_timestamps, WgrabAvPairStatus, WgrabAvSyncTolerance, WgrabTimestamp,
    WgrabTimestampQuality,
};

fn main() {
    let tolerance = WgrabAvSyncTolerance::from_millis(16);

    let paired = pair_video_audio_timestamps(
        Some(WgrabTimestamp::from_nanos(1_000_000_000)),
        WgrabTimestampQuality::Backend,
        Some(WgrabTimestamp::from_nanos(1_010_000_000)),
        WgrabTimestampQuality::DequeueTime,
        tolerance,
    );

    assert_eq!(paired.status, WgrabAvPairStatus::Paired);
    assert_eq!(paired.delta_nanos, Some(10_000_000));

    let outside = pair_video_audio_timestamps(
        Some(WgrabTimestamp::from_nanos(1_000_000_000)),
        WgrabTimestampQuality::Backend,
        Some(WgrabTimestamp::from_nanos(1_050_000_000)),
        WgrabTimestampQuality::DequeueTime,
        tolerance,
    );

    assert_eq!(outside.status, WgrabAvPairStatus::OutsideTolerance);
    assert_eq!(outside.delta_nanos, Some(50_000_000));

    let missing_video = pair_video_audio_timestamps(
        None,
        WgrabTimestampQuality::Unavailable,
        Some(WgrabTimestamp::from_nanos(1_000_000_000)),
        WgrabTimestampQuality::DequeueTime,
        tolerance,
    );

    assert_eq!(
        missing_video.status,
        WgrabAvPairStatus::MissingVideoTimestamp
    );
    assert_eq!(missing_video.delta_nanos, None);

    let missing_audio = pair_video_audio_timestamps(
        Some(WgrabTimestamp::from_nanos(1_000_000_000)),
        WgrabTimestampQuality::Backend,
        None,
        WgrabTimestampQuality::Unavailable,
        tolerance,
    );

    assert_eq!(
        missing_audio.status,
        WgrabAvPairStatus::MissingAudioTimestamp
    );
    assert_eq!(missing_audio.delta_nanos, None);

    println!("CI_WGRAB_AV_SYNC_PAIRING_OK");
}
