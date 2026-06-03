# wgrab 第29Sフェーズログ

## 結論

- timestamp model: `WgrabTimestamp` is monotonic elapsed capture time, not wall
  clock time
- capture clock: `WgrabCaptureClock` prototype added using `Instant`
- audio frame timestamp: `try_next_frame()` assigns dequeue-time timestamps
- audio input example: `timestamp_nanos` output added
- target checks: Windows and macOS target checks completed with warnings only;
  Linux host checks stopped in `alsa-sys` because `pkg-config` is unavailable
  on this machine
- package/dry-run: see `logs/phase29s_cargo_package.txt` and
  `logs/phase29s_cargo_publish_dry_run.txt`
- push CI: checked after branch push
- ready for system audio investigation: yes, after CI

## Branch

- branch: wgrab-av-sync-model
- commit: phase branch commit
- status: implementation phase complete

## Implemented

- types: `WgrabTimestamp`, `WgrabCaptureClock`
- methods: `WgrabTimestamp::from_nanos`, `WgrabTimestamp::as_nanos`,
  `WgrabCaptureClock::start_now`, `WgrabCaptureClock::now`
- examples: `audio_cpal_input_smoke` timestamp output
- docs: `docs/AV_SYNC_MODEL.md`

## Timestamp model

- current: audio frames are timestamped when samples are dequeued from the
  prototype buffer
- future: backend-native callback/capture timestamps should replace dequeue-time
  timestamps where available

## Deferred

- backend-native timestamps
- drift correction
- resampling
- video frame timestamp API
- system audio capture
- Web Audio timestamping
