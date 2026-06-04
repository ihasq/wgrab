# wgrab 第42Sフェーズログ

## 結論

- owning queue: added
- timestamped payload: added
- peek match: added
- pop pair: added
- deterministic example: added
- target checks: passed
- package/dry-run: success
- CI: success
- ready for real frame queue integration: yes

## Branch

- branch: wgrab-av-sync-owning-queue
- commit: pending
- status: local checks, package/dry-run, and push CI passed

## Implemented

- types: `WgrabTimestamped`, `WgrabAvFrameQueue`, `WgrabAvOwnedQueueMatch`, `WgrabAvOwnedPair`
- methods: push audio/video, peek latest video match, pop latest video pair
- examples: `av_sync_owning_queue_smoke`
- docs: A/V sync model, GPU-only API, audio API sketch

## Semantics

- peek: reports nearest audio for latest video without removing frames
- pop: removes video/audio only when pair status is `Paired`
- overflow: push drops oldest entries beyond configured limits
- missing timestamps: no payloads are removed
- outside tolerance: no payloads are removed

## Deferred

- direct WgpuCaptureFrame/WgrabAudioFrame helper
- drift correction
- resampling
- scheduler
- muxing
