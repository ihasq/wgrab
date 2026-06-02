# wgrab web audio backend decision

## Decision

Native audio backend:

- CPAL

Web audio backend:

- Web Audio / AudioWorklet first
- CPAL AudioWorklet is investigated as an optional processing backend
- CPAL is not assumed to solve web capture input by itself

## Why

CPAL provides a cross-platform audio abstraction for native targets.

On WebAssembly, CPAL supports the Web Audio API through its `wasm-bindgen`
feature and provides an optional `audioworklet` backend for lower-latency web
audio processing.

Browser capture setup is still web-specific. Permissions and stream creation
are asynchronous browser flows based on `getUserMedia` or `getDisplayMedia`,
and wgrab must preserve that boundary instead of hiding it behind the native
CPAL model.

## Layering

### Browser main thread

Responsible for:

- permissions
- `getDisplayMedia`
- `getUserMedia`
- `MediaStream`
- `AudioContext`
- `MediaStreamAudioSourceNode`
- `AudioWorkletNode`

### AudioWorklet thread

Responsible for:

- low-latency audio processing
- sample buffering
- timestamp capture
- sending frames to wgrab wasm state

### wgrab public API

Future candidates:

- `WgrabWebAudioContext`
- `WgrabWebAudioStream`
- `WgrabWebAudioFrame`

## CPAL role on web

CPAL may be used if its AudioWorklet backend can satisfy wgrab requirements.

If not, wgrab will implement a `web-sys` AudioWorklet backend directly.

## Requirements for CPAL AudioWorklet path

- wasm atomics
- SharedArrayBuffer
- cross-origin isolation
- AudioWorklet module loader
- browser support
- input/capture capability validation

## Phase 27W decision

For Phase 27W, wgrab does not add `web-sys`, `wasm-bindgen`, CPAL
`audioworklet`, or wasm CI.

The decision is to keep CPAL as the native backend and design the browser path
around Web Audio / AudioWorklet directly. CPAL AudioWorklet remains a candidate
processing backend only after capture input compatibility is validated.

## Reference material

- CPAL README: <https://github.com/RustAudio/cpal>
- MDN AudioWorklet: <https://developer.mozilla.org/en-US/docs/Web/API/AudioWorklet>
- wasm-bindgen AudioWorklet example: <https://rustwasm.github.io/docs/wasm-bindgen/examples/wasm-audio-worklet.html>
