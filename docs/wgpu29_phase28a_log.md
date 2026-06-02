# wgrab 第28Aフェーズログ

## 結論

- CPAL input stream: default input device stream prototype implemented
- WgrabAudioFrame samples: interleaved normalized `f32`
- audio input example: `examples/audio_cpal_input_smoke.rs`
- audio CI: input smoke check and marker gate added
- target checks: Windows and macOS target checks completed with warnings only;
  Linux host checks stopped in `alsa-sys` because `pkg-config` is unavailable
  on this machine
- package/dry-run: see `logs/phase28a_cargo_package.txt` and
  `logs/phase28a_cargo_publish_dry_run.txt`
- ready for A/V sync model: yes as a prototype frame source

## Branch

- branch: wgrab-cpal-input-stream-prototype
- commit: phase branch commit
- status: implementation phase complete

## Implemented

- types: `WgrabAudioFrame`, `WgrabAudioStream`, `WgrabAudioError`
- methods: `CpalAudioBackend::build_default_input_stream`,
  `WgrabAudioStream::try_next_frame`
- examples: `audio_cpal_input_smoke`
- workflow: audio runtime input-smoke check and marker gate

## CI markers

- Windows: pending CI
- macOS: pending CI
- Ubuntu: pending CI

## Deferred

- system audio loopback
- timestamp precision
- lock-free ring buffer
- resampling
- A/V synchronization
- Web Audio implementation
