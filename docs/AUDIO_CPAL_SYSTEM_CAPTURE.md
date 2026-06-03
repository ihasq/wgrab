# CPAL system audio capture capability

## Goal

Determine whether CPAL can serve system audio capture needs directly, or
whether wgrab needs platform-specific system audio backends.

## Known role

CPAL is the native audio backend for generic device/stream capture.

## Questions

| Question | Answer | Evidence | Decision |
|---|---|---|---|
| Does CPAL enumerate Windows loopback endpoints? | Not as a separate high-level system-audio API. CPAL can enumerate devices and its WASAPI backend documents output-device-as-input loopback behavior. | CPAL 0.17.3 source and README. | Prototype CPAL render-device loopback before adding direct WASAPI. |
| Can CPAL select default output loopback capture? | Likely, by selecting the default output device and building an input stream on it on WASAPI. This needs runtime validation. | CPAL WASAPI backend comment. | Treat as a candidate, not a final decision. |
| Can CPAL capture macOS system audio directly? | CPAL 0.17 adds CoreAudio loopback recording on macOS > 14.6. | CPAL 0.17 release notes and local CPAL CoreAudio loopback source. | Consider CPAL loopback for generic macOS output-device capture. |
| Does CPAL provide timestamps suitable for A/V sync? | CPAL exposes `InputCallbackInfo::timestamp()` with callback and capture stream instants. Different streams may have different origins. | CPAL `StreamInstant` and `InputStreamTimestamp` docs. | Useful but must be normalized into `WgrabTimestamp`. |
| Does CPAL expose enough metadata for system audio? | Maybe for device-level loopback. It may not expose screen/window association, ScreenCaptureKit alignment, or process-specific policy. | CPAL abstracts host/device/stream behavior. | Platform-specific backends remain likely for capture-session alignment. |

## Preliminary policy

CPAL remains the native audio abstraction.

If system audio requires platform APIs, those APIs should feed into the same
`WgrabAudioFrame` model.

## CPAL-specific implications

- CPAL should remain the first prototype route for generic native audio streams.
- CPAL may be sufficient for Windows default-output loopback if render endpoint
  selection and timestamp behavior are adequate.
- CPAL may be sufficient for macOS generic output-device loopback on macOS
  versions supported by its CoreAudio loopback implementation.
- ScreenCaptureKit remains the preferred macOS route when audio must align with
  ScreenCaptureKit video frames.
- CPAL callback timestamps are a better source than dequeue-time prototype
  timestamps, but they need stream-origin normalization.

## Reference material

- CPAL README: <https://github.com/RustAudio/cpal>
- CPAL releases: <https://github.com/RustAudio/cpal/releases>

## Phase 31A CPAL loopback capability probe

wgrab first investigates whether CPAL can expose system-audio-like devices
through normal host/device enumeration.

This phase uses heuristics only.

The loopback heuristic treats the following as possible signals:

- device name contains `loopback`
- device name contains `monitor`
- device name contains `what u hear`
- device name contains `stereo mix`
- device name contains `output`
- device name contains `speaker`
- device is the default output and also supports input configs

This is not a final loopback detector.

A direct WASAPI or ScreenCaptureKit backend remains a candidate if CPAL does not
provide sufficient system audio capture.

## Phase 32A CPAL system-audio candidate stream

This phase attempts to build an input stream from CPAL loopback candidates.

A candidate stream is considered a prototype only.

It may not guarantee true system audio. The candidate may be silent, may be a
virtual input device, or may not correspond to the active screen capture source.

If CPAL cannot reliably expose system audio, wgrab should use
platform-specific backends:

- Windows: WASAPI loopback
- macOS: ScreenCaptureKit audio

## Phase 33A hardening

The CPAL system-audio candidate path now records:

- candidate selection score
- selection reasons
- input/output config counts
- default formats
- audio energy statistics
- silence detection

Silence is not treated as CI failure because CI runners may not play audio.
