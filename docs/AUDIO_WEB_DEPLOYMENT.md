# wgrab web audio deployment requirements

## AudioWorklet

AudioWorklet requires a browser environment and an `AudioContext`.

The worklet processor runs on the Web Audio rendering thread.

## Wasm AudioWorklet requirements

Potential requirements:

- wasm-bindgen
- ES module loader or generated JS glue
- atomics if using threaded wasm
- SharedArrayBuffer if using shared memory
- Cross-Origin-Opener-Policy
- Cross-Origin-Embedder-Policy
- HTTPS or localhost secure context

## wgrab policy

Do not add these requirements to native builds.

Keep web audio behind explicit future web features.

## Phase 27W deployment decision

Phase 27W documents these requirements only.

It does not add wasm build CI, `web-sys`, `wasm-bindgen`, CPAL
`audioworklet`, or browser deployment headers to the crate.
