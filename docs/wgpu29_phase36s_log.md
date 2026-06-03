# wgrab 第36Sフェーズログ

## 結論

- WgpuCaptureFrame timestamp: optional `WgrabTimestamp` accessor added
- audio/video timestamp API: `WgrabAudioFrame::timestamp()` and `WgpuCaptureFrame::timestamp()`
- examples: texture-only example reports `timestamp_nanos`
- target checks: passed
- package/dry-run: success with `--allow-dirty` for pre-commit verification logs
- push CI: branch pushed; GitHub did not trigger runs because `origin/main` does not contain these workflow files
- ready for sync hardening: local target checks passed; pushed CI unavailable under current repo workflow configuration

## Branch

- branch: wgrab-av-sync-integration
- commit: see phase completion report
- status: local checks passed; branch pushed; push CI did not trigger

## Implemented

- types: shared `wgrab::time::WgrabTimestamp`, `WgrabAvSyncTolerance`
- methods: `WgpuCaptureFrame::timestamp()`
- examples: `examples/wgpu_texture_only_capture.rs`
- docs: A/V sync model and GPU/audio boundary notes

## Timestamp availability

- Windows video: existing `VideoFrame::origin_time()` normalized into `WgrabTimestamp`
- macOS video: existing `VideoFrame::origin_time()` normalized into `WgrabTimestamp`
- CPAL audio: prototype dequeue-time timestamp
- WASAPI audio: prototype dequeue-time timestamp
- SCK audio: sample-buffer presentation timestamp when conversion succeeds

## Deferred

- drift correction
- resampling
- muxing
- precise backend timestamps
- web audio timestamps
