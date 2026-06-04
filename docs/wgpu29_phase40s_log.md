# wgrab 第40Sフェーズログ

## 結論

- pairing helper: added
- pair result: added
- tolerance: default candidates added
- example: added
- target checks: cross-target checks passed; pairing smoke is runtime/backend independent
- package/dry-run: success
- push CI: success
- ready for sync hardening: yes

## Branch

- branch: wgrab-av-sync-pairing-prototype
- commit: pending
- status: local checks, package/dry-run, and push CI passed

## Implemented

- types: `WgrabAvPairStatus`, `WgrabAvPairInfo`
- functions: `pair_video_audio_timestamps`
- examples: `av_sync_pairing_smoke` runs without capture/audio device features
- docs: A/V sync, audio API sketch, GPU-only API

## Pairing policy

- tolerance: caller-provided `WgrabAvSyncTolerance`
- status: paired, missing timestamp, or outside tolerance
- timestamp quality: returned for both video and audio timestamps

## Deferred

- frame queues
- drift correction
- resampling
- scheduling
- muxing
- backend timestamp precision
