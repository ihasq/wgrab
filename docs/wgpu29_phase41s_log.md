# wgrab 第41Sフェーズログ

## 結論

- sync queue: added
- queue config: added
- nearest-neighbor matching: added
- deterministic example: added
- target checks: passed
- package/dry-run: success
- CI: success
- ready for frame queue prototype: yes

## Branch

- branch: wgrab-av-sync-queue-prototype
- commit: pending
- status: local checks, package/dry-run, and push CI passed

## Implemented

- types: `WgrabTimestampedFrameInfo`, `WgrabAvQueueMatch`, `WgrabAvSyncQueueConfig`, `WgrabAvSyncQueue`
- methods: push audio/video timestamps, queue lengths, match latest video
- examples: `av_sync_queue_smoke`
- docs: A/V sync model, audio API sketch, GPU-only API

## Pairing behavior

- latest video: `match_latest_video()` targets the newest video metadata
- nearest audio: audio candidate with minimum absolute timestamp delta
- tolerance: caller-provided `WgrabAvSyncTolerance`
- missing timestamp: missing video/audio timestamps are reported as pair statuses
- overflow: push drops oldest entries beyond configured limits

## Deferred

- owning frame queues
- matched frame removal
- drift correction
- resampling
- scheduler
