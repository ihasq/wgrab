# wgrab 第19Xフェーズログ

## 結論

- legacy API removal: completed
- retained APIs: completed
- package metadata: unchanged
- generated docs: unchanged
- GPU-only independence: passed
- negative grep: known false positive only
- target checks: success
- push CI: pending
- ready for package/version decision: pending push CI

## Branch

- branch: `wgrab-gpu-only-api-design`
- commit: pending
- status: pending push CI

## Removed APIs

### CPU-readable bitmap APIs

- `PooledBitmap`
- `BitmapData*`
- `FrameBitmap*`
- `BoxedSliceFrameBitmap`
- `PooledFrameBitmap`
- `FrameBitmapPool`
- `VideoFrameBitmap`
- `get_bitmap`
- `try_get_pooled_bitmap`
- `get_pooled_bitmap`

### Raw platform output APIs

- `IoSurface` public raw output
- `IoSurface::get_raw`
- `MacosIoSurfaceVideoFrameExt::get_iosurface`
- `MetalVideoFramePlaneTexture`
- `MetalVideoFrameExt::get_metal_texture`
- `MetalCaptureStreamExt::get_metal_device`
- `WindowsDx11VideoFrame::get_dx11_surface`
- `WindowsDx11VideoFrame::get_dx11_texture`
- `WindowsDx11CaptureStream::get_dx11_device`
- `WindowsDxgiVideoFrame::get_dxgi_surface`

## Retained APIs

- `WgpuCaptureFrame`
- `WgpuVideoFrameGpuOnlyExt`
- `WgpuVideoFrameExt::get_wgpu_texture`
- `WgpuCaptureConfigExt::with_wgpu_device`
- `WindowsDxgiCaptureStream::get_dxgi_adapter`
- `WindowsDxgiCaptureStream::get_dxgi_device`
- diagnostic APIs

## Public API diff summary

- CPU-readable bitmap public items were removed.
- raw IOSurface / Metal / D3D11 / DXGI frame output public shims were removed.
- macOS IOSurface and Windows D3D11 helpers remain internal for the wgpu GPU-only path.
- DXGI adapter/device access remains public for compatibility.

## Checks

- removed API grep: empty
- GPU-only legacy dependency grep: empty
- CPU readback negative grep: `GetIoSurfaceError::NoImageBuffer` false positive only
- Windows target check: success
- macOS x86 target check: success
- macOS arm target check: success
- texture-only example check: success
- texture-only warning index: macOS extern ABI warning only, deferred to cleanup

## Package/version

- package name: unchanged
- version: unchanged
- distribution strategy: Git dependency only
- crates.io publish: not performed

## Deferred

- package name decision
- version bump
- crates.io strategy
- generated docs cleanup
- macOS extern ABI cleanup
- Linux native wgpu feature split
