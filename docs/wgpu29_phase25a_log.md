# wgrab 第25Aフェーズログ

## 結論

- audio roadmap: documented
- native backend strategy: CPAL first candidate
- web backend strategy: Web Audio first candidate
- API sketch: documented
- publish blocker: no
- target checks: success
- ready for audio investigation phase: yes

## Branch

- branch: `wgrab-crates-io-prep`
- commit: pending
- status: dirty before Phase 25A commit

## Audio direction

- native: investigate CPAL as the first backend abstraction
- web: investigate Web Audio with `AudioContext`, `MediaStreamAudioSourceNode`,
  and `AudioWorkletNode`
- primary public API: `WgrabAudioContext`, `WgrabAudioStream`,
  `WgrabAudioFrame`
- synchronization: design shared monotonic timestamp model for pairing
  `WgpuCaptureFrame` and `WgrabAudioFrame`

## Non-goals

- implementation: none in Phase 25A
- dependencies: no CPAL / web-sys / wasm-bindgen dependency added
- wasm CI: not added
- publish: no version bump or crates.io publish

## Deferred

- CPAL investigation
- platform system audio capture
- Web Audio prototype
- AudioWorklet prototype
- A/V sync model
- metadata repair for `documentation` / `homepage`
