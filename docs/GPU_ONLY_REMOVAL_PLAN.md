# GPU-only removal plan

## Goal

wgrab is moving to a GPU-only public capture API centered on
`WgpuCaptureFrame`.

Deprecated CPU-readable and raw platform output APIs will be removed in a future
breaking-change phase.

No APIs are removed in Phase 17X.

## Phase 19X result

Deprecated legacy capture output APIs have been removed from the public API.

Removed:

- CPU-readable bitmap APIs
- raw IOSurface output APIs
- raw Metal output APIs
- raw D3D11 output APIs
- raw DXGI frame surface output API

Retained:

- `WgpuCaptureFrame`
- `WgpuVideoFrameGpuOnlyExt`
- `WgpuVideoFrameExt::get_wgpu_texture`
- `WindowsDxgiCaptureStream::get_dxgi_adapter`
- `WindowsDxgiCaptureStream::get_dxgi_device`
- diagnostic APIs

Package metadata remains unchanged.

## Removal version policy

Planned deprecation version:

- `0.5.0`

Planned removal version:

- undecided
- candidate: `0.6.0` or first `wgrab`-branded breaking release

## A. Removed in Phase 19X

| API | Current feature | Replacement | Removal phase |
|---|---|---|---|
| `PooledBitmap` | `bitmap` | `WgpuCaptureFrame` | Phase 19X |
| `BitmapData` / `BitmapDataMut` | `bitmap` | `WgpuCaptureFrame` | Phase 19X |
| `FrameBitmap` | `bitmap` | `WgpuCaptureFrame` | Phase 19X |
| `BoxedSliceFrameBitmap` | `bitmap` | `WgpuCaptureFrame` | Phase 19X |
| `PooledFrameBitmap` | `bitmap` | `WgpuCaptureFrame` | Phase 19X |
| `FrameBitmapPool` | `bitmap` | `WgpuCaptureFrame` | Phase 19X |
| `VideoFrameBitmap::get_bitmap` | `bitmap` | `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame` | Phase 19X |
| `VideoFrameBitmap::try_get_pooled_bitmap` | `bitmap` | `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame` | Phase 19X |
| `VideoFrameBitmap::get_pooled_bitmap` | `bitmap` | `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame` | Phase 19X |
| `IoSurface` public raw output | `iosurface` | `WgpuCaptureFrame` | Phase 19X |
| `IoSurface::get_raw` | `iosurface` | `WgpuCaptureFrame` | Phase 19X |
| `MacosIoSurfaceVideoFrameExt::get_iosurface` | `iosurface` | `WgpuCaptureFrame` | Phase 19X |
| `MetalVideoFramePlaneTexture` | `metal` | `WgpuVideoFramePlaneTexture` / `WgpuCaptureFrame` | Phase 19X |
| `MetalVideoFrameExt::get_metal_texture` | `metal` | `WgpuCaptureFrame` | Phase 19X |
| `MetalCaptureStreamExt::get_metal_device` | `metal` | caller-provided `wgpu::Device` | Phase 19X |
| `WindowsDx11VideoFrame::get_dx11_surface` | `dx11` | `WgpuCaptureFrame` | Phase 19X |
| `WindowsDx11VideoFrame::get_dx11_texture` | `dx11` | `WgpuCaptureFrame` | Phase 19X |
| `WindowsDx11CaptureStream::get_dx11_device` | `dx11` | caller-provided `wgpu::Device` | Phase 19X |
| `WindowsDxgiVideoFrame::get_dxgi_surface` | `dxgi` | `WgpuCaptureFrame` | Phase 19X |

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
| generated docs release handling | whether generated docs are regenerated, removed, or kept temporarily |

## Diagnostic APIs

Diagnostic APIs remain public.

Reason:

- they are not primary capture output APIs
- they support debugging, CI, and downstream integration
- they do not contradict the GPU-only capture output direction

## Deferred warning cleanup

macOS extern ABI warnings are deferred to a cleanup phase.

They are not part of the GPU-only removal plan.

## Decisions required before removal implementation

Before removing deprecated APIs, decide:

1. package name
2. version
3. release channel
4. whether removal happens before or after first GPU-only registry release
5. generated docs policy
6. compatibility alias strategy

## Phase 19X distribution decision

The owner selected Option C: Git dependency only.

No package name, version, crates.io publishing, tag, GitHub Release, or generated
docs change is made in Phase 19X.
