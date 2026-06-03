# Windows system audio capture investigation

## Goal

Investigate how wgrab should capture system audio on Windows while keeping
`WgrabAudioFrame` as the public output.

## Candidate: WASAPI loopback

Windows WASAPI loopback can capture the audio stream being played by a rendering
endpoint device.

Potential pipeline:

```text
render endpoint device
  -> WASAPI loopback capture stream
  -> normalized f32 interleaved samples
  -> WgrabAudioFrame
```

## Questions

| Question | Answer | Evidence | Decision |
|---|---|---|---|
| Can CPAL expose loopback devices sufficiently? | Maybe for endpoint loopback. CPAL 0.17.3 WASAPI notes that using a WASAPI output device as an input device transparently enables loopback mode. | Local CPAL 0.17.3 source, `src/host/wasapi/mod.rs`; Microsoft WASAPI loopback docs. | Prototype CPAL output-device-as-input first if it can select the desired render endpoint. |
| Does wgrab need direct WASAPI integration? | Possibly. Direct WASAPI may be needed for precise endpoint selection, stream flags, process loopback, and timestamp control. | Microsoft documents `IMMDevice`, `IAudioClient`, `AUDCLNT_STREAMFLAGS_LOOPBACK`, and `IAudioCaptureClient` as the loopback path. | Keep direct WASAPI as fallback if CPAL does not expose enough control. |
| How are timestamps obtained? | CPAL exposes `InputCallbackInfo` with capture/callback stream instants. Direct WASAPI may expose packet timing through the capture client path. | CPAL `InputStreamTimestamp`; WASAPI capture client model. | Normalize backend timing into `WgrabTimestamp`. |
| How are channel layout and sample format handled? | WASAPI shared-mode loopback follows the render endpoint mix format. wgrab should normalize to interleaved `f32`. | WASAPI shared-mode loopback captures from the audio engine mix. | Keep `WgrabAudioFormat` as the normalized public metadata. |
| How does loopback relate to selected display/window capture? | Endpoint loopback captures an output device mix, not a display/window-specific audio source. | WASAPI loopback is tied to a rendering endpoint device. | Treat display/window association as a higher-level policy problem. |

## Public API policy

Do not expose raw WASAPI objects as primary public API.

System audio should surface as `WgrabAudioFrame`.

## Investigation notes

- WASAPI loopback is shared-mode only.
- The default output device is the first useful prototype target.
- A future system audio config should allow explicit render endpoint selection.
- Hardware "Stereo Mix" style devices are not reliable as the primary path
  because they are adapter/vendor dependent and may require manual user setup.
- Direct WASAPI remains relevant if CPAL cannot expose endpoint identity,
  process-loopback policy, or timestamp behavior clearly enough.

## Reference material

- Microsoft Learn: <https://learn.microsoft.com/en-us/windows/win32/coreaudio/loopback-recording>
- CPAL README: <https://github.com/RustAudio/cpal>

## Phase 34L-Windows prototype

wgrab adds a direct WASAPI loopback prototype.

The prototype uses the default render endpoint and captures loopback audio into
`WgrabAudioFrame`.

CI may only prove that the backend initializes or reports unavailable.
Non-silent audio requires a real environment with active playback.
