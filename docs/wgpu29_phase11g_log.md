# wgrab 第11Gフェーズログ

## 結論

- GPU-only policy: documented
- API audit: completed as initial classification
- API sketch: documented
- migration plan: documented
- release draft pause: documented
- crates.io strategy: deferred
- ready for implementation planning: pending push CI

## Branch

- branch: `wgrab-gpu-only-api-design`
- commit: pending
- status: target checks passed before commit

## Owner decisions

- future API surface: GPU-only capture output
- CPU-readable output: do not expose as public capture output
- raw platform public output: do not expose as primary output
- primary capture output: `wgpu::Texture`
- previous RC tag: `wgrab-wgpu29-rc1` remains the CrabGrab-compatible modernization baseline

## Audit summary

- Keep: existing wgpu texture/device extension surface
- Replace: bitmap and pooled bitmap frame output APIs
- Deprecate then remove: CPU-readable bitmap data abstractions
- Internalize: IOSurface, Metal texture, DX11/DX12 raw resource output APIs

## Local validation

- public output candidates: `logs/phase11g_public_output_candidates.txt`
- public function candidates: `logs/phase11g_public_function_candidates.txt`
- CPU/readback candidates: `logs/phase11g_cpu_readback_candidates.txt`
- Windows target check: success
- macOS x86 target check: success
- macOS arm target check: success
- check error index: `logs/phase11g_check_error_index.txt` is empty

## Deferred

- actual API removal
- implementation
- package rename
- crates.io strategy
- versioning
- GitHub Release publication

## 判断を求めたい点

1. `WgpuCaptureFrame` / `WgpuCaptureStream` の命名を確定するか。
2. 既存 `bitmap`, `iosurface`, `metal`, `dx11` feature を deprecate-first にするか、GPU-only branchで即削除するか。
3. GPU-only API実装前に package/version 方針を先に固めるか。
