# wgrab 第34L-Windowsフェーズログ

## 結論

- WASAPI loopback backend: direct default render endpoint prototype added
- example: `examples/audio_wasapi_loopback_smoke.rs`
- CI: Windows-only WASAPI smoke check and marker gate added
- package/dry-run: completed with existing warnings only
- ready for macOS system audio: pending pushed CI

## Branch

- branch: wgrab-wasapi-loopback-prototype
- commit: pending
- status: local checks complete

## Implementation

- COM: initializes MTA and keeps a guard for the creator thread
- device: uses `IMMDeviceEnumerator::GetDefaultAudioEndpoint(eRender, eConsole)`
- audio client: activates `IAudioClient` and initializes shared loopback mode
- capture client: reads packets from `IAudioCaptureClient` on a capture thread
- sample conversion: supports f32 mix format and 16-bit PCM, normalizes to f32
- WgrabAudioFrame: appends interleaved f32 samples into `WgrabAudioStream`

## CI result

- Windows: target check and WASAPI example check completed with warnings only
- macOS: x86_64 and aarch64 target checks completed with warnings only
- Ubuntu: package verification completed with existing warnings only

## Deferred

- application loopback
- device selection
- strict non-silent check
- precise timestamp
- macOS ScreenCaptureKit audio
