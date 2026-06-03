# wgrab 第30Lフェーズログ

## 結論

- Windows system audio: WASAPI loopback is the primary platform candidate;
  CPAL WASAPI output-device-as-input loopback should be prototyped first
- macOS system audio: ScreenCaptureKit audio is the primary screen-coupled
  candidate; CPAL CoreAudio loopback remains useful for generic output capture
- CPAL system audio capability: promising for device-level loopback but not
  assumed sufficient for screen/window-aligned capture
- unified system audio design: all native backends feed `WgrabAudioFrame`,
  `WgrabAudioFormat`, and `WgrabTimestamp`
- A/V sync implications: native packet/sample timestamps should replace
  dequeue-time timestamps when available
- target checks: Windows and macOS target checks completed with warnings only
- ready for system audio prototype: yes, after CI

## Branch

- branch: wgrab-system-audio-investigation
- commit: phase branch commit
- status: documentation phase complete

## Decisions

- CPAL: generic native audio backend and first loopback prototype route where
  it exposes enough device-level behavior
- Windows: investigate CPAL WASAPI loopback first, direct WASAPI fallback
- macOS: prefer ScreenCaptureKit audio for screen-coupled capture
- public API: keep raw platform audio objects out of the primary public output

## Deferred

- WASAPI implementation
- ScreenCaptureKit audio implementation
- CPAL loopback implementation
- system audio runtime CI
- Web audio implementation
