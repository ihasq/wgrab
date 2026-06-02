# wgrab 第16Rフェーズログ

## 結論

- texture-only runtime probe: workflow に追加
- GPU-only example: `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame` 経路へ接続
- deprecated shim independence: deprecated public shim 直接呼び出しなし
- negative grep: 既知 false positive のみ
- target checks: 成功
- push CI: 実行予定
- ready for removal planning: push CI確認後に判断

## Branch

- branch: wgrab-gpu-only-api-design
- commit: このログを含む commit
- status: commit 前検証完了

## Runtime markers

- Windows: push CI確認後に記録
- macOS arm64: push CI確認後に記録
- macOS Intel: push CI確認後に記録

## GPU-only path

- frame API: `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame`
- texture API: `WgpuCaptureFrame::texture`
- texture view: `WgpuCaptureFrame::create_view`
- descriptor/debug: `CRABGRAB_WGPU_DEBUG_DESCRIPTOR=1` で descriptor summary を出力
- CPU readback: 追加なし
- capability classification: texture wrapping 前の adapter/device setup 不可は
  `--allow-capture-unavailable` 時に `CI_WGRAB_TEXTURE_ONLY_UNAVAILABLE`
  として扱う。`get_wgpu_capture_frame` 以降の失敗は
  `CI_WGRAB_TEXTURE_ONLY_FAILED` のまま。

## Local checks

- `logs/phase16r_gpu_only_legacy_dependency_grep.txt`: deprecated public shim 参照なし
- `logs/phase16r_gpu_only_negative_grep.txt`: 既知 false positive のみ
- `logs/phase16r_check_error_index.txt`: 空
- `logs/phase16r_texture_only_warning_error_index.txt`: 既存 macOS `extern` ABI warning のみ
- `logs/phase16r_texture_only_gpu_deprecation_warning_index.txt`: 空

## Deferred

- deprecated shim removal
- dxgi adapter/device decision
- diagnostic cleanup
- macOS extern ABI cleanup
- package/version strategy
