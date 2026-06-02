# GPU-only removal plan

## Goal

wgrab is moving to a GPU-only public capture API centered on
`WgpuCaptureFrame`.

Deprecated CPU-readable and raw platform output APIs will be removed in a future
breaking-change phase.

No APIs are removed in Phase 17X.

## Removal version policy

Planned deprecation version:

- `0.5.0`

Planned removal version:

- undecided
- candidate: `0.6.0` or first `wgrab`-branded breaking release

## A. Remove later

| API | Current feature | Replacement | Planned removal |
|---|---|---|---|
| `PooledBitmap` | `bitmap` | `WgpuCaptureFrame` | undecided |
| `BitmapData` / `BitmapDataMut` | `bitmap` | `WgpuCaptureFrame` | undecided |
| `FrameBitmap` | `bitmap` | `WgpuCaptureFrame` | undecided |
| `BoxedSliceFrameBitmap` | `bitmap` | `WgpuCaptureFrame` | undecided |
| `PooledFrameBitmap` | `bitmap` | `WgpuCaptureFrame` | undecided |
| `FrameBitmapPool` | `bitmap` | `WgpuCaptureFrame` | undecided |
| `VideoFrameBitmap::get_bitmap` | `bitmap` | `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame` | undecided |
| `VideoFrameBitmap::try_get_pooled_bitmap` | `bitmap` | `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame` | undecided |
| `VideoFrameBitmap::get_pooled_bitmap` | `bitmap` | `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame` | undecided |
| `IoSurface` | `iosurface` | `WgpuCaptureFrame` | undecided |
| `IoSurface::get_raw` | `iosurface` | `WgpuCaptureFrame` | undecided |
| `MacosIoSurfaceVideoFrameExt::get_iosurface` | `iosurface` | `WgpuCaptureFrame` | undecided |
| `MetalVideoFramePlaneTexture` | `metal` | `WgpuVideoFramePlaneTexture` / `WgpuCaptureFrame` | undecided |
| `MetalVideoFrameExt::get_metal_texture` | `metal` | `WgpuCaptureFrame` | undecided |
| `MetalCaptureStreamExt::get_metal_device` | `metal` | caller-provided `wgpu::Device` | undecided |
| `WindowsDx11VideoFrame::get_dx11_surface` | `dx11` | `WgpuCaptureFrame` | undecided |
| `WindowsDx11VideoFrame::get_dx11_texture` | `dx11` | `WgpuCaptureFrame` | undecided |
| `WindowsDx11CaptureStream::get_dx11_device` | `dx11` | caller-provided `wgpu::Device` | undecided |
| `WindowsDxgiVideoFrame::get_dxgi_surface` | `dxgi` | `WgpuCaptureFrame` | undecided |

## B. Keep as compatibility shim

| API | Current feature | Reason |
|---|---|---|
| `WindowsDxgiCaptureStream::get_dxgi_adapter` | `dxgi` | diagnostic / compatibility / migration window |
| `WindowsDxgiCaptureStream::get_dxgi_device` | `dxgi` | diagnostic / compatibility / migration window |

## C. Keep public

| API | Feature | Reason |
|---|---|---|
| `WgpuCaptureFrame` | `wgpu` | primary GPU-only capture output |
| `WgpuCaptureStream` | `wgpu` | reserved GPU-only stream API |
| `WgpuCaptureConfig` | `wgpu` | reserved GPU-only config API |
| `WgpuVideoFrameGpuOnlyExt` | `wgpu` | primary frame conversion API |
| `WgpuCaptureConfigExt::with_wgpu_device` | `wgpu` | caller-provided device contract |
| `WgpuVideoFrameExt::get_wgpu_texture` | `wgpu` | compatibility and lower-level wgpu interop |
| diagnostic APIs | `diagnostic` | diagnostics are not primary capture output APIs |

## D. Internalize later

| API | Internal role | Public replacement |
|---|---|---|
| IOSurface bridge | macOS capture resource bridge | `WgpuCaptureFrame` |
| Metal texture bridge | macOS Metal-to-wgpu interop | `WgpuCaptureFrame` |
| D3D11/D3D12 resource bridge | Windows capture resource bridge | `WgpuCaptureFrame` |
| DXGI shared resource bridge | Windows resource sharing and synchronization | `WgpuCaptureFrame` |
| `objc_wrap` platform helpers | macOS platform support | `WgpuCaptureFrame` / platform-neutral public APIs |

## E. Investigate before removal

| API | Question |
|---|---|
| public platform wrapper types reachable through feature modules | whether downstream users depend on them directly |
| generated docs under `docs/*_docs` | whether they should be regenerated or removed before a breaking release |
| `WgpuVideoFrameExt::get_wgpu_texture` | whether it remains as a lower-level GPU API or becomes an internal helper after `WgpuCaptureFrame` is fully primary |

## Diagnostic APIs

Diagnostic APIs remain public.

Reason:

- they are not primary capture output APIs
- they support debugging, CI, and downstream integration
- they do not contradict the GPU-only capture output direction

## Deferred warning cleanup

macOS extern ABI warnings are deferred to a cleanup phase.

They are not part of the GPU-only removal plan.
