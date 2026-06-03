# wgrab 第35L-macOSフェーズログ

## 結論

- ScreenCaptureKit audio backend: prototype backend skeleton added
- example: `examples/audio_sck_audio_smoke.rs`
- CI: macOS-only ScreenCaptureKit audio smoke check and marker gate added
- package/dry-run: success
- ready for A/V sync integration: pending pushed CI

## Branch

- branch: wgrab-screencapturekit-audio-prototype
- commit: pending
- status: local checks passed; push CI pending

## Implementation

- SCStreamConfiguration: default display stream configuration
- capturesAudio: enabled with 48 kHz stereo prototype settings
- stream output: `SCStreamOutputType::Audio`
- sample conversion: `CMSampleBuffer` to `AVAudioPCMBuffer` to interleaved f32
- timestamp: sample buffer timestamp is read by the wrapper; stream frame API still uses dequeue-time fallback
- WgrabAudioFrame: existing `WgrabAudioStream::try_next_frame()` path

## CI result

- Windows: pending
- macOS: local cross-target checks passed
- Ubuntu: unchanged in this phase

## Deferred

- microphone capture
- excludes current process audio policy
- precise timestamp alignment
- strict non-silent check
- Web Audio implementation
