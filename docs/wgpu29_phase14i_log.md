# wgrab 第14Iフェーズログ

## 結論

- internalization audit: 作成
- dxgi classification: raw output / shim candidate として分類
- dependency map: 作成
- linux native wgpu note: 記録
- negative grep: 新規 GPU-only path は該当なし
- target checks: 成功
- push CI: 実行予定
- ready for internalization implementation: push CI確認後に判断

## Branch

- branch: wgrab-gpu-only-api-design
- commit: このログを含む commit
- status: commit 前検証完了

## Classification summary

### Keep public

- `WgpuCaptureFrame`
- `WgpuVideoFrameGpuOnlyExt`
- `WgpuCaptureConfig`
- `WgpuCaptureStream`
- `WgpuCaptureConfigExt::with_wgpu_device`

### Deprecated, keep temporarily

- bitmap CPU-readable APIs
- iosurface raw output APIs
- metal raw output APIs
- dx11 raw output APIs

### Internalize later

- IOSurface bridge
- Metal texture bridge
- D3D11 / D3D12 resource bridge
- DXGI shared resource / adapter / device bridge
- objc_wrap platform helpers

### Shim needed

- `metal` feature public exports
- `iosurface` feature public exports
- `dx11` feature public exports
- `dxgi` feature public exports

### Investigate

- diagnostic structs
- reachability of `objc_wrap` public helpers

## DXGI decision

- `WindowsDxgiVideoFrame::get_dxgi_surface`: raw output, internalize candidate
- `WindowsDxgiCaptureStream::{get_dxgi_adapter,get_dxgi_device}`: raw adapter/device access, shim candidate
- Blanket deprecation is not done in Phase 14I.

## Linux native wgpu decision

Native Linux `cargo check --features wgpu` remains future work. Phase 14I keeps
Linux validation scoped to the standalone Ubuntu Lavapipe `wgpu` smoke workflow.

## Negative grep

`logs/phase14i_gpu_only_negative_grep.txt` は既存 wgpu path の
`GetIoSurfaceError::NoImageBuffer` に反応した。これは CPU readback API ではなく
既存 IOSurface 取得エラー名の false positive として分類する。

## Checks

- Windows target check: 成功
- macOS x86 target check: 成功
- macOS arm target check: 成功
- `logs/phase14i_check_error_index.txt`: 空

## Deferred

- actual pub(crate) changes
- module moves
- compatibility shims
- package/version strategy
