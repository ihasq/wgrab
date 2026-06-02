# wgrab 第13Dフェーズログ

## 結論

- bitmap deprecation: 実装
- raw platform output deprecation: public extension trait / accessor に実装
- migration guide: `docs/GPU_ONLY_MIGRATION_GUIDE.md`
- README update: GPU-only direction 追記
- texture-only example clean: Windows / macOS target check 成功
- negative grep: 新規 GPU-only path は該当なし
- target checks: 成功
- push CI: 実行予定
- ready for internalization planning: push CI確認後に判断

## Branch

- branch: wgrab-gpu-only-api-design
- commit: このログを含む commit
- status: commit 前検証完了

## Deprecated APIs

### CPU-readable bitmap APIs

- `PooledBitmap`
- `BitmapDataBgra8x4`
- `BitmapDataArgbUnormPacked2101010`
- `BitmapDataRgbaF16x4`
- `BitmapDataLuma`
- `BitmapDataChroma`
- `FrameBitmapBgraUnorm8x4`
- `FrameBitmapArgbUnormPacked2101010`
- `FrameBitmapRgbaF16x4`
- `FrameBitmapYCbCr`
- `FrameBitmap`
- `BoxedSliceFrameBitmap`
- `PooledFrameBitmap`
- `FrameBitmapPool`
- `VideoFrameBitmap`
- `VideoFrameBitmap::get_bitmap`
- `VideoFrameBitmap::try_get_pooled_bitmap`
- `VideoFrameBitmap::get_pooled_bitmap`

### Raw platform output APIs

- `IoSurface`
- `IoSurface::get_raw`
- `MacosIoSurfaceVideoFrameExt`
- `MacosIoSurfaceVideoFrameExt::get_iosurface`
- `MetalVideoFramePlaneTexture`
- `MetalVideoFrameExt`
- `MetalVideoFrameExt::get_metal_texture`
- `MetalCaptureStreamExt`
- `MetalCaptureStreamExt::get_metal_device`
- `WindowsDx11VideoFrame`
- `WindowsDx11VideoFrame::get_dx11_surface`
- `WindowsDx11VideoFrame::get_dx11_texture`
- `WindowsDx11CaptureStream`
- `WindowsDx11CaptureStream::get_dx11_device`

## Not deprecated yet

- `objc_wrap` low-level platform bridge types and functions
- `dxgi` feature APIs
- diagnostic structs

reason: 第13Dでは public CPU-readable output と primary raw platform output extension に限定した。

## Migration path

- old: bitmap / IOSurface / Metal / DX11 raw output APIs
- new: `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame` and `WgpuCaptureFrame`

## Warning summary

- deprecated warnings: macOS target logs に既存 `extern` ABI warning あり
- accepted: `extern` ABI warning は既存 platform wrapper 由来として許容
- needs follow-up: GPU-only deprecation note warning は texture-only example に出ていない

`logs/phase13d_texture_only_gpu_deprecation_warnings.txt` は空。
`logs/phase13d_texture_only_example_error_index.txt` は空。
`logs/phase13d_check_error_index.txt` は空。

Native Linux の `cargo check --example wgpu_texture_only_capture --features wgpu,wgpu-debug-descriptor`
は既存 `wgpu` feature の target-specific dependency 制約で失敗する。既存
`wgpu_capture_smoke` example でも同じ失敗が再現するため、第13Dの example
clean 判定は Windows / macOS target check を基準にした。

## Negative grep

`logs/phase13d_gpu_only_negative_grep.txt` は既存 wgpu path の
`GetIoSurfaceError::NoImageBuffer` に反応した。これは CPU readback API ではなく
既存 IOSurface 取得エラー名の false positive として分類する。

## Checks

- Windows target check: 成功
- macOS x86 target check: 成功
- macOS arm target check: 成功
- texture-only example Windows target check: 成功
- texture-only example macOS x86 target check: 成功
- texture-only example macOS arm target check: 成功
- rustfmt: `examples/wgpu_texture_only_capture.rs` は成功
- full/source rustfmt: 既存 formatting 差分を拾うため第13Dでは適用しない

## Deferred

- actual removal
- module internalization
- package rename
- version bump
- crates.io strategy
