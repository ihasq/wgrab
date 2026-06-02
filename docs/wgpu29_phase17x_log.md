# wgrab 第17Xフェーズログ

## 結論

- removal matrix: 作成
- migration guide: 削除前提の移行表を追記
- versioning notes: 作成
- diagnostic policy: public 維持
- macOS ABI warning: cleanup phase へ defer
- GPU-only independence: deprecated public shim 依存なし
- negative grep: 既知 false positive のみ
- target checks: 成功
- push CI: 実行予定
- ready for owner removal decision: push CI確認後に判断

## Branch

- branch: wgrab-gpu-only-api-design
- commit: このログを含む commit
- status: commit 前検証完了

## Removal candidates

- bitmap CPU-readable output APIs
- IOSurface raw output APIs
- Metal raw output APIs
- D3D11 raw output APIs
- targeted DXGI raw output API: `WindowsDxgiVideoFrame::get_dxgi_surface`

## Compatibility shims retained

- `WindowsDxgiCaptureStream::get_dxgi_adapter`
- `WindowsDxgiCaptureStream::get_dxgi_device`

## Keep public

- `WgpuCaptureFrame`
- `WgpuCaptureStream`
- `WgpuCaptureConfig`
- `WgpuVideoFrameGpuOnlyExt`
- `WgpuCaptureConfigExt::with_wgpu_device`
- `WgpuVideoFrameExt::get_wgpu_texture`
- diagnostic feature APIs

## Internalize later

- IOSurface bridge
- Metal texture bridge
- D3D11/D3D12 resource bridge
- DXGI shared resource bridge
- `objc_wrap` platform helpers

## Versioning options

- Option A: stay on `crabgrab`, bump to `0.5.0`
- Option B: publish as `wgrab`
- Option C: Git dependency only for now

No package or version change is made in Phase 17X.

## Local checks

- `logs/phase17x_deprecated_attributes.txt`: deprecated API index
- `logs/phase17x_public_items_snapshot.txt`: public item snapshot
- `logs/phase17x_gpu_only_legacy_dependency_grep.txt`: empty
- `logs/phase17x_gpu_only_negative_grep.txt`: `NoImageBuffer` false positive only
- `logs/phase17x_check_error_index.txt`: empty
- `logs/phase17x_texture_only_warning_error_index.txt`: existing macOS `extern` ABI warnings only

## Deferred

- actual removal
- package/version decision
- crates.io strategy
- macOS extern ABI cleanup
- Linux native wgpu feature split
