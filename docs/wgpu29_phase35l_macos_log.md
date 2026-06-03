# wgrab 第35L-macOSフェーズログ

## 結論

- ScreenCaptureKit audio backend: prototype backend skeleton added with metadata frame queue
- example: `examples/audio_sck_audio_smoke.rs`
- CI: macOS-only ScreenCaptureKit audio smoke check and marker gate added
- package/dry-run: success with `--allow-dirty` for pre-commit verification logs
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
- timestamp: sample buffer presentation timestamp is forwarded for ScreenCaptureKit audio frames; dequeue-time timestamp remains the fallback for aggregate-buffer backends
- WgrabAudioFrame: ScreenCaptureKit audio uses the internal metadata queue before the aggregate fallback

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
