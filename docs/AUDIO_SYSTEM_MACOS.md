# macOS system audio capture investigation

## Goal

Investigate how wgrab should capture system audio on macOS while keeping
`WgrabAudioFrame` as the public output.

## Candidate: ScreenCaptureKit audio

ScreenCaptureKit can capture screen and audio content.
`SCStreamConfiguration.capturesAudio` controls whether audio is captured.

Potential pipeline:

```text
ScreenCaptureKit SCStream
  -> audio sample buffers
  -> normalized f32 interleaved samples
  -> WgrabAudioFrame
```

## Questions

| Question | Answer | Evidence | Decision |
|---|---|---|---|
| Should system audio use ScreenCaptureKit rather than CPAL? | For screen-coupled capture, yes as the primary candidate. CPAL macOS loopback may still be useful for generic output-device capture. | Apple documents ScreenCaptureKit screen and audio sample output from the same stream. CPAL 0.17.3 adds CoreAudio loopback for macOS > 14.6. | Prefer ScreenCaptureKit audio when the video source is ScreenCaptureKit. |
| How are audio sample buffers delivered? | `SCStreamOutput` receives `CMSampleBuffer` values with output type `.audio`. | Apple ScreenCaptureKit sample and `SCStreamOutputType.audio` docs. | Convert audio sample buffers to normalized `f32` samples. |
| What timestamp is available? | `CMSampleBuffer` timing should be used as the native timestamp source. | ScreenCaptureKit emits media `CMSampleBuffer` values with metadata/timing. | Map sample timing to `WgrabTimestamp` in a future implementation phase. |
| How does ScreenCaptureKit audio align with video frames? | ScreenCaptureKit can deliver screen and audio outputs from the same `SCStream`, making it the best candidate for A/V alignment. | Apple sample processes `.screen` and `.audio` output types through the stream output delegate. | Keep this as the preferred macOS system-audio path for capture sessions. |
| How are sample rate and channel count configured? | `SCStreamConfiguration` exposes `sampleRate` and `channelCount`; Apple documents mono/stereo channel behavior. | Apple `SCStreamConfiguration` docs. | Preserve configured values in `WgrabAudioFormat`. |
| What permissions are required? | ScreenCaptureKit uses screen recording permission for screen capture; audio capture is part of the configured stream. | Apple ScreenCaptureKit sample requires Screen Recording permission. | Document permission handling in the implementation phase. |

## Public API policy

Do not expose raw CoreAudio or ScreenCaptureKit audio buffers as primary public
API.

System audio should surface as `WgrabAudioFrame`.

## Investigation notes

- `capturesAudio` is false by default and must be enabled for audio capture.
- `sampleRate`, `channelCount`, and `excludesCurrentProcessAudio` are key
  configuration values for wgrab policy.
- `SCStreamOutputType.audio` wraps audio samples in an `AudioBufferList` inside
  a `CMSampleBuffer`.
- ScreenCaptureKit is the strongest macOS candidate for audio that should align
  with captured screen/window frames.
- CPAL CoreAudio loopback is still worth a separate prototype for generic
  output-device capture and non-ScreenCaptureKit sessions.

## Reference material

- Apple ScreenCaptureKit: <https://developer.apple.com/documentation/screencapturekit>
- Apple `SCStreamConfiguration`: <https://developer.apple.com/documentation/screencapturekit/scstreamconfiguration>
- Apple `SCStreamOutputType.audio`: <https://developer.apple.com/documentation/screencapturekit/scstreamoutputtype/audio>
- CPAL releases: <https://github.com/RustAudio/cpal/releases>

## Phase 35L-macOS prototype

wgrab adds a ScreenCaptureKit audio prototype.

The prototype enables `SCStreamConfiguration.capturesAudio` and attempts to
receive audio sample buffers.

The public output remains `WgrabAudioFrame`.

CI may only prove that the backend compiles or reports unavailable. Real system
audio requires a macOS environment with capture permission and active audio.
