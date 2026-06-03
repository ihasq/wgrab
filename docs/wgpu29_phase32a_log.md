# wgrab 第32Aフェーズログ

## 結論

- CPAL loopback stream: added prototype builder for loopback candidate streams
- WgrabAudioSource: added `DefaultInput` and `SystemAudioCandidate`
- system audio smoke example: `examples/audio_cpal_system_audio_smoke.rs`
- audio CI: system audio smoke check and marker gate added
- package/dry-run: see `logs/phase32a_cargo_package.txt` and
  `logs/phase32a_cargo_publish_dry_run.txt`
- ready for platform backend decision: pending pushed CI

## Branch

- branch: wgrab-cpal-system-audio-prototype
- commit: pending
- status: local checks complete

## Results

- Windows: target check completed with warnings only
- macOS: x86_64 and aarch64 target checks completed with warnings only
- Ubuntu: local host audio checks require ALSA `pkg-config`; CI is authoritative

## Decision

- CPAL sufficient: pending system-audio candidate smoke results
- Windows direct WASAPI needed: pending Windows candidate behavior
- macOS ScreenCaptureKit needed: still likely for screen-coupled capture

## Deferred

- direct WASAPI implementation
- ScreenCaptureKit audio implementation
- silence detection
- true system-audio verification
- stable device identifiers
