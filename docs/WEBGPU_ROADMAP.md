# wgrab WebGPU roadmap

## Goal

wgrab should eventually support browser capture using Web APIs and WebGPU.

Target pipeline:

```text
navigator.mediaDevices.getDisplayMedia()
  -> MediaStream
  -> MediaStreamTrack
  -> MediaStreamTrackProcessor
  -> VideoFrame
  -> GPUDevice.importExternalTexture()
  -> GPUExternalTexture
  -> wgrab web GPU frame abstraction
```

## Direction

The web target should follow the same GPU-only direction as native wgrab.

wgrab should not expose captured frames as CPU-readable byte buffers.

## Native and web parity

Native primary output:

- `WgpuCaptureFrame`
- `wgpu::Texture`

Future web primary output candidate:

- `WgrabWebCaptureFrame`
- `GPUExternalTexture`
- a wgpu-compatible abstraction if wgpu exposes a suitable API

## Important API facts

- `getDisplayMedia()` returns a `MediaStream`.
- `MediaStreamTrackProcessor.readable` yields `VideoFrame` objects.
- `GPUDevice.importExternalTexture()` can import a `VideoFrame` as `GPUExternalTexture`.
- `wgpu` supports WebGPU on wasm.

## Native/web API boundary

The web API should not pretend that `GPUExternalTexture` is the same object model
as native `wgpu::Texture` unless wgpu exposes a safe compatible abstraction.

The public API may need a web-specific frame type that mirrors
`WgpuCaptureFrame` conceptually:

- GPU-resident frame output
- size / format metadata where available
- texture binding path suitable for rendering
- permission and browser capability errors

## Non-goals for wgrab 0.1.0

- no `web_sys` implementation
- no `wasm-bindgen` integration
- no wasm CI
- no browser capture runtime
- no `VideoFrame` wrapper
- no `GPUExternalTexture` wrapper

WebGPU / web_sys support is not a blocker for publishing `wgrab 0.1.0`.

## Future audio integration

The web roadmap may eventually include audio capture from the same `MediaStream`.

Potential path:

```text
getDisplayMedia
  -> MediaStream audio track
  -> MediaStreamAudioSourceNode
  -> AudioContext
  -> AudioWorkletNode
```

This is not part of `wgrab 0.1.0`.

## Future feature names

Candidate Cargo features:

- `web`
- `webgpu`
- `web-sys`
- `wasm`

No feature is added yet.

## Open questions

- Should web support live in the main crate or a companion crate?
- How should `GPUExternalTexture` map to the native `WgpuCaptureFrame` concept?
- Should wgrab expose a web-specific frame type instead of pretending it is a `wgpu::Texture`?
- How should browser permission errors map to wgrab errors?
- Which browser matrix should be supported first?

## Reference material

- MDN: `MediaDevices.getDisplayMedia()`
  <https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getDisplayMedia>
- MDN: Screen Capture API
  <https://developer.mozilla.org/en-US/docs/Web/API/Screen_Capture_API>
- MDN: `MediaStreamTrackProcessor`
  <https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackProcessor>
- MDN: `GPUDevice.importExternalTexture()`
  <https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/importExternalTexture>
- wgpu docs
  <https://wgpu.rs/doc/wgpu/>
