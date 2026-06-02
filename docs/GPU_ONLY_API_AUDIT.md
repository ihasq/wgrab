# GPU-only API audit

This audit classifies current public output APIs against the future GPU-only direction.

The audit is descriptive only. No API is removed in phase 11G.

## A. Keep

APIs that already fit the GPU-only direction.

| API | File | Reason |
|---|---|---|
| `WgpuCaptureConfigExt::with_wgpu_device` | `src/feature/wgpu/mod.rs` | Requires caller-provided `wgpu::Device`, which matches the no-hidden-device contract. |
| `WgpuVideoFrameExt::get_wgpu_texture` | `src/feature/wgpu/mod.rs` | Produces `wgpu::Texture`; this is the closest existing API to the new direction. |
| `WgpuVideoFramePlaneTexture` | `src/feature/wgpu/mod.rs` | Plane selection can remain useful for multi-plane GPU textures. |
| `WgpuVideoFrameError::WrongWgpuBackend` | `src/feature/wgpu/mod.rs` | Explicit backend mismatch error is required by the GPU-only design. |

## B. Replace with wgpu::Texture API

APIs that should be replaced by a `wgpu::Texture`-based output.

| API | File | Current output | Proposed replacement |
|---|---|---|---|
| `VideoFrameBitmap::get_bitmap` | `src/feature/bitmap/mod.rs` | `BoxedSliceFrameBitmap` CPU bitmap | `WgpuCaptureFrame::texture()` or `WgpuVideoFrameExt::get_wgpu_texture` during transition. |
| `VideoFrameBitmap::try_get_pooled_bitmap` | `src/feature/bitmap/mod.rs` | Optional pooled CPU bitmap | GPU frame queue or caller-side GPU texture handling; no pooled CPU output. |
| `VideoFrameBitmap::get_pooled_bitmap` | `src/feature/bitmap/mod.rs` | Pooled CPU bitmap | GPU-resident frame output; no CPU bitmap pool. |
| `FrameBitmap*` public bitmap data types | `src/feature/bitmap/mod.rs` | CPU-readable image containers | Texture metadata plus `wgpu::Texture`; caller performs readback outside wgrab if required. |

## C. Deprecate then remove

APIs that expose CPU-readable data or raw platform objects.

| API | File | Reason | Migration |
|---|---|---|---|
| `BoxedSliceFrameBitmap` | `src/feature/bitmap/mod.rs` | Owns CPU-readable bitmap data. | Use GPU texture output. |
| `PooledFrameBitmap` | `src/feature/bitmap/mod.rs` | Owns reusable CPU-readable bitmap data. | Use GPU frame lifecycle; avoid CPU pools. |
| `FrameBitmapPool` | `src/feature/bitmap/mod.rs` | Exists only to manage CPU bitmap buffers. | Remove when bitmap output is removed. |
| `BitmapData*` traits | `src/feature/bitmap/mod.rs` | Public CPU data abstractions. | Remove with bitmap feature retirement or keep only in legacy compatibility branch. |

## D. Internalize

APIs that may remain internally but should not be public.

| API | File | Internal role |
|---|---|---|
| `MacosIoSurfaceVideoFrameExt::get_iosurface` | `src/feature/iosurface/mod.rs` | Internal macOS bridge from capture frame to Metal/wgpu texture. |
| `IoSurface::get_raw` | `src/feature/iosurface/mod.rs` | Raw IOSurface handle for internal platform interop. |
| `MetalVideoFrameExt::get_metal_texture` | `src/feature/metal/mod.rs` | Internal macOS texture construction path; not primary public output. |
| `MetalCaptureStreamExt::get_metal_device` | `src/feature/metal/mod.rs` | Platform device access; should not be the primary output API. |
| `WindowsDx11VideoFrame::get_dx11_surface` | `src/feature/dx11/mod.rs` | Internal Windows capture resource bridge. |
| `WindowsDx11VideoFrame::get_dx11_texture` | `src/feature/dx11/mod.rs` | Internal Windows capture resource bridge. |
| `WindowsDx11CaptureStream::get_dx11_device` | `src/feature/dx11/mod.rs` | Platform device access for interop. |
| `objc_wrap` IOSurface / CGImage / CMSampleBuffer helpers | `src/platform/macos/objc_wrap.rs` | Low-level platform implementation details. |

## Out of scope

Audio APIs such as `AudioFrame::audio_channel_buffer` expose audio sample buffers, not captured screen regions. They are not part of the GPU-only video output decision in phase 11G.

## Phase 13D result

Deprecated:

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

Deferred:

- API removal
- feature removal
- public module removal
- package rename
- version bump

Not deprecated yet because still internal implementation dependency:

- `objc_wrap` platform bridge types and functions
- `dxgi` feature APIs
- diagnostic structs

Reason:

These are either low-level platform implementation details, diagnostics, or
outside the primary CPU/readback and raw-output deprecation set for phase 13D.
