# CPAL web compatibility matrix

## Questions

| Question | Answer | Evidence | Decision |
|---|---|---|---|
| Does CPAL support wasm? | Yes. | CPAL lists WebAssembly as a supported platform and documents a Web Audio API backend. | Keep CPAL under investigation for wasm audio. |
| Does CPAL support Web Audio backend? | Yes. | CPAL documents `wasm-bindgen` as the Web Audio API backend feature required for browser audio support. | Do not enable it in Phase 27W. |
| Does CPAL support AudioWorklet backend? | Yes, as an optional backend. | CPAL documents `audioworklet` for `wasm32-unknown-unknown` and describes it as lower-latency AudioWorklet processing. | Treat it as optional until wgrab validates capture input. |
| Does CPAL AudioWorklet support the needed input/capture path? | Not proven for wgrab. | CPAL supports input/output streams in general, but wgrab needs browser permission, `MediaStream`, and display-audio capture setup. | Do not assume CPAL solves web capture input. |
| Does CPAL require atomics / SharedArrayBuffer? | Yes for the AudioWorklet path. | CPAL documents atomics flags and cross-origin headers for `SharedArrayBuffer`; wasm-bindgen worklet examples also require threaded wasm setup. | Keep this behind future explicit web features. |
| Can wgrab use CPAL for getDisplayMedia audio track capture? | Not as the primary assumption. | `getDisplayMedia` returns browser `MediaStream` tracks, which are created through asynchronous browser APIs before audio graph processing. | Design a direct Web Audio / AudioWorklet backend first. |

## Preliminary decision

CPAL remains the native backend.

For web capture, wgrab should design a direct Web Audio / AudioWorklet backend
unless CPAL proves sufficient for input capture.

## Evidence notes

- CPAL supported platforms list WebAssembly with Web Audio API as the default
  backend and Audio Worklet as an optional backend.
- CPAL optional features document `wasm-bindgen` for Web Audio and
  `audioworklet` for AudioWorklet.
- CPAL `audioworklet` requires atomics support and cross-origin headers for
  `SharedArrayBuffer`.
- AudioWorklet runs processor code in a Web Audio rendering thread and is
  available only in secure browser contexts.

## Reference material

- CPAL README: <https://github.com/RustAudio/cpal>
- MDN AudioWorklet: <https://developer.mozilla.org/en-US/docs/Web/API/AudioWorklet>
- wasm-bindgen AudioWorklet example: <https://rustwasm.github.io/docs/wasm-bindgen/examples/wasm-audio-worklet.html>
