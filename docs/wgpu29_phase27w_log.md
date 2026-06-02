# wgrab 第27Wフェーズログ

## 結論

- CPAL web role: optional wasm processing candidate, not the primary browser
  capture setup layer
- AudioWorklet decision: preferred web processing boundary for low-latency
  audio
- direct Web Audio backend: design first, prototype deferred
- deployment requirements: wasm-bindgen, worklet loading, atomics,
  SharedArrayBuffer, cross-origin isolation, and secure context documented
- target checks: Windows and macOS target checks completed with warnings only;
  Linux host checks stopped in `alsa-sys` because `pkg-config` is unavailable
  on this machine
- ready for web audio prototype: yes for design planning, no dependency or CI
  changes made in this phase

## Branch

- branch: wgrab-web-audio-worklet-design
- commit: phase branch commit
- status: documentation phase complete

## Decisions

- native CPAL: remains the native audio backend
- web CPAL: investigated as optional, not assumed sufficient for capture input
- AudioWorklet: preferred web processing thread boundary
- web-specific frame: keep as a future candidate to avoid hiding browser
  permission and lifetime constraints

## Deferred

- web-sys dependency
- wasm-bindgen dependency
- AudioWorklet implementation
- CPAL audioworklet feature
- wasm CI
- A/V sync implementation
