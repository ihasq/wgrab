# wgrab 第23Wフェーズログ

## 結論

- WebGPU roadmap: documented
- GPU-only web direction: documented
- publish blocker: no
- target checks: success
- package: success
- publish dry-run: success
- ready for owner publish decision: yes

## Branch

- branch: `wgrab-crates-io-prep`
- commit: pending
- status: dirty before Phase 23W commit

## Web roadmap

- capture source: `navigator.mediaDevices.getDisplayMedia()`
- frame source: `VideoFrame`
- GPU integration: `GPUDevice.importExternalTexture()`
- future output: `GPUExternalTexture` or a wgpu-compatible web frame abstraction

## Native/web parity

- native primary output: `WgpuCaptureFrame` / `wgpu::Texture`
- web future output: `WgrabWebCaptureFrame` / `GPUExternalTexture` candidate
- CPU-readable output: not planned

## Non-goals

- web implementation: not in Phase 23W
- wasm CI: not in Phase 23W
- feature changes: none
- dependency changes: none

## Publish status

- `wgrab 0.1.0` publish: still owner approval pending
- owner UI availability: pending
- owner approval: pending

## Verification

- target checks: `logs/phase23w_check_error_index.txt` is empty
- cargo package: success with `--allow-dirty` for Phase 23W docs/README changes
- cargo publish dry-run: success with `--allow-dirty` for Phase 23W docs/README changes
- package size: 50 files, 402.7KiB unpacked, 80.5KiB compressed
- warnings: two unsupported/default-target dead code warnings, unchanged from publish preflight

## 判断を求めたい点

1. Web support should live in the main crate or a companion crate.
2. Web frame output should be a web-specific type or a wgpu-compatible abstraction.
3. Browser support matrix should be defined before any wasm implementation starts.
