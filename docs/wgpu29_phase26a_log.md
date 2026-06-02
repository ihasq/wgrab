# wgrab 第26Aフェーズログ

## 結論

- CPAL dependency: added as optional dependency
- audio feature: added
- audio API skeleton: added
- CPAL backend skeleton: added
- audio probe example: added
- audio CI: added
- target checks: Windows/macOS success; local Linux requires ALSA system deps
- package/dry-run: success
- ready for audio stream prototype: pending on push CI

## Branch

- branch: `wgrab-cpal-audio-backend`
- commit: pending
- status: dirty before Phase 26A commit

## Implemented

- features: `audio`, `audio-cpal`
- types:
  - `WgrabAudioContext`
  - `WgrabAudioStream`
  - `WgrabAudioFrame`
  - `WgrabAudioFormat`
  - `WgrabSampleFormat`
  - `WgrabTimestamp`
  - `CpalAudioBackend`
- examples: `examples/audio_cpal_probe.rs`
- workflows: `.github/workflows/audio-runtime.yml`

## CI markers

- Windows: pending
- macOS: pending
- Ubuntu: pending

## Local verification

- Windows audio target check: success
- macOS x86 audio target check: success
- macOS arm audio target check: success
- audio example cross-checks: success
- local Linux audio check: blocked by missing `pkg-config` / ALSA development
  files in this environment
- Ubuntu CI installs `libasound2-dev` and `pkg-config` before checking `audio`
- cargo package: success with `--allow-dirty`
- cargo publish dry-run: success with `--allow-dirty`; existing `wgrab 0.1.0`
  warning is expected because 0.1.0 is already published

## Deferred

- actual audio stream capture
- system audio loopback
- A/V sync model
- Web Audio implementation
- crates.io publish of audio feature release
