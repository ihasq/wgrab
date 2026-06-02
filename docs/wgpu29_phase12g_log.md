# wgrab 第12Gフェーズログ

## 結論

- WgpuCaptureFrame: 実装
- WgpuCaptureStream: skeleton 実装
- WgpuCaptureConfig: skeleton 実装
- GPU-only frame method: `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame`
- texture-only example: `examples/wgpu_texture_only_capture.rs`
- negative grep: 新規 GPU-only path は該当なし
- target checks: 成功
- push CI: 実行予定
- ready for deprecation planning: push CI確認後に判断

## Branch

- branch: wgrab-gpu-only-api-design
- commit: このログを含む commit
- status: commit 前検証完了

## Implemented API

- types: `WgpuCaptureFrame`, `WgpuCaptureStream`, `WgpuCaptureConfig`
- traits: `WgpuVideoFrameGpuOnlyExt`
- methods: `texture`, `size`, `format`, `usage`, `create_view`, `get_wgpu_capture_frame`
- examples: `wgpu_texture_only_capture`

`get_wgpu_capture_frame` は既存 `get_wgpu_texture` path を利用して
`wgpu::Texture` を取得し、`texture.size()`, `texture.format()`,
`texture.usage()` から `WgpuCaptureFrame` の metadata を構築する。

## CPU readback check

- new GPU-only path: `copy_texture_to_buffer`, `MAP_READ`, mapped buffer, byte/image output は該当なし
- existing legacy bitmap path: 第12Gでは未変更

`logs/phase12g_gpu_only_negative_grep.txt` は既存 wgpu path の
`GetIoSurfaceError::NoImageBuffer` に反応した。これは CPU readback API ではなく
既存 IOSurface 取得エラー名の false positive として分類する。

`logs/phase12g_gpu_only_new_path_negative_grep.txt` は空。

## Checks

- Windows target check: 成功
- macOS x86 target check: 成功
- macOS arm target check: 成功
- texture-only example Windows check: 成功
- texture-only example macOS x86 check: 成功
- texture-only example macOS arm check: 成功
- `logs/phase12g_check_error_index.txt`: 空
- `logs/phase12g_check_texture_example_error_index.txt`: 空
- rustfmt check: 追加/変更 example は成功

## Deferred

- legacy API deprecation
- legacy API removal
- package rename
- version bump
- crates.io strategy
- strict runtime capture
