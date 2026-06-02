# GPU-only internalization audit

## Goal

wgrab is moving to a GPU-only public API centered on `WgpuCaptureFrame`.

Raw platform objects should become internal implementation details unless there
is a strong compatibility reason to keep them public temporarily.

## A. Keep public

APIs that should remain public.

| API | File | Reason |
|---|---|---|
| `WgpuCaptureFrame` | `src/feature/wgpu/gpu_only.rs` | Primary GPU-only capture output. |
| `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame` | `src/feature/wgpu/gpu_only.rs` | Current public bridge from legacy frames to GPU-only frame output. |
| `WgpuCaptureConfig` | `src/feature/wgpu/gpu_only.rs` | Reserved public configuration surface for GPU-only capture. |
| `WgpuCaptureStream` | `src/feature/wgpu/gpu_only.rs` | Reserved public stream surface for GPU-only capture. |
| `WgpuCaptureConfigExt::with_wgpu_device` | `src/feature/wgpu/mod.rs` | Maintains caller-provided device contract and avoids hidden wgpu devices. |

## B. Keep public temporarily, deprecated

APIs that are deprecated but remain for migration.

| API | File | Replacement | Removal phase |
|---|---|---|---|
| `VideoFrameBitmap` | `src/feature/bitmap/mod.rs` | `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame` | Later breaking-change phase. |
| `FrameBitmap*` / `BitmapData*` | `src/feature/bitmap/mod.rs` | `WgpuCaptureFrame` metadata and texture accessors | Later breaking-change phase. |
| `FrameBitmapPool` | `src/feature/bitmap/mod.rs` | GPU frame lifecycle / caller-managed GPU queueing | Later breaking-change phase. |
| `MacosIoSurfaceVideoFrameExt` / `IoSurface` | `src/feature/iosurface/mod.rs` | `WgpuCaptureFrame` | Later breaking-change phase. |
| `MetalVideoFrameExt` / `MetalCaptureStreamExt` | `src/feature/metal/mod.rs` | `WgpuCaptureFrame` | Later breaking-change phase. |
| `WindowsDx11VideoFrame` / `WindowsDx11CaptureStream` | `src/feature/dx11/mod.rs` | `WgpuCaptureFrame` | Later breaking-change phase. |

## C. Internalize later

APIs that should become `pub(crate)` or move to internal modules.

| API | File | Internal role | Public replacement |
|---|---|---|---|
| `IoSurface` internals | `src/feature/iosurface/mod.rs` | macOS bridge from capture frame to Metal/wgpu texture. | `WgpuCaptureFrame` |
| `MetalVideoFrameExt::get_metal_texture` implementation path | `src/feature/metal/mod.rs` | Legacy Metal texture construction and compatibility bridge. | `WgpuCaptureFrame` |
| `WindowsDx11VideoFrame::get_dx11_texture` implementation path | `src/feature/dx11/mod.rs` | Windows capture resource bridge used by bitmap/wgpu internals. | `WgpuCaptureFrame` |
| `WindowsDxgiVideoFrame::get_dxgi_surface` | `src/feature/dxgi/mod.rs` | Raw DXGI surface extraction from captured D3D11 resource. | `WgpuCaptureFrame` |
| `WindowsDxgiCaptureStream::{get_dxgi_adapter,get_dxgi_device}` | `src/feature/dxgi/mod.rs` | Windows adapter/device interop support. | `WgpuCaptureFrame` plus internal interop. |
| `objc_wrap` IOSurface / CoreMedia / ScreenCaptureKit helpers | `src/platform/macos/objc_wrap.rs` | Low-level macOS capture implementation support. | None; implementation detail. |

## D. Compatibility shim needed

APIs that cannot be removed/internalized without a shim.

| API | File | Compatibility risk | Proposed shim |
|---|---|---|---|
| `metal` feature public exports | `src/prelude.rs`, `src/feature/mod.rs` | Existing users may import `MetalVideoFrameExt` from prelude or feature module. | Keep deprecated extension trait until breaking release; provide migration docs to `WgpuCaptureFrame`. |
| `iosurface` feature public exports | `src/prelude.rs`, `src/feature/mod.rs` | Existing macOS interop users may depend on raw IOSurface access. | Keep deprecated extension trait until breaking release; internalize implementation behind compatibility layer later. |
| `dx11` feature public exports | `src/prelude.rs`, `src/feature/mod.rs` | Existing Windows interop users may use D3D11 surfaces/textures directly. | Keep deprecated extension trait until breaking release; provide `WgpuCaptureFrame` migration path. |
| `dxgi` feature public exports | `src/prelude.rs`, `src/feature/mod.rs` | Existing Windows users may rely on `IDXGISurface`, adapter, or device access. | Add targeted deprecation in a future phase, then keep shim until breaking release. |

## E. Investigate

APIs whose role is unclear.

| API | File | Question |
|---|---|---|
| `diagnostic` structs | `src/feature/diagnostic/mod.rs` | Diagnostics expose platform-derived information, but not primary raw output. Keep public unless owner decides diagnostics should be internal. |
| `objc_wrap` public helpers | `src/platform/macos/objc_wrap.rs` | Many are public within platform implementation. Confirm whether any are reachable through stable user-facing modules before changing visibility. |

## Feature classification

### bitmap

deprecated, remove later.

### iosurface

internalize later. The wgpu Metal path may keep IOSurface as an internal bridge.
The public raw output extension remains deprecated during migration.

### metal

internalize later. Existing public `metal` feature is not removed in this
planning phase. Compatibility shim is required.

### dx11

internalize later. Windows capture interop still needs D3D11 internally. Public
D3D11 output remains deprecated during migration.

### dxgi

audit first, then targeted deprecation/internalization. Public DXGI surface,
adapter, and device access are raw platform outputs and compatibility shim
candidates.

### objc_wrap

internal implementation support. Public visibility in this module is not treated
as stable top-level API without a reachability review.

## Existing deprecations

- bitmap: public CPU-readable bitmap types, pools, traits, and accessors.
- iosurface: `IoSurface`, `IoSurface::get_raw`, `MacosIoSurfaceVideoFrameExt`.
- metal: `MetalVideoFramePlaneTexture`, `MetalVideoFrameExt`, `MetalCaptureStreamExt`.
- dx11: `WindowsDx11VideoFrame`, `WindowsDx11CaptureStream`.
- dxgi: none yet.

## DXGI policy

DXGI APIs are not automatically deprecated as a group.

They are classified by role:

- raw capture output: deprecate/internalize
- internal resource sharing: internalize
- diagnostic helper: investigate
- required Windows interop bridge: keep internal

No code changes are made in Phase 14I.

## Linux native `--features wgpu`

Native Linux `cargo check --features wgpu` is not a Phase 14I goal.

Current CI validates `wgpu 29` Vulkan runtime through a standalone Ubuntu
Lavapipe smoke test without adding Linux capture support to wgrab.

Future work may split the `wgpu` feature or add Linux-friendly
backend-smoke-only checks.

## Phase 15I result

### Public primary API

- `WgpuCaptureFrame`
- `WgpuVideoFrameGpuOnlyExt`
- `WgpuCaptureConfig`
- `WgpuCaptureStream`

### Deprecated public shims kept

- bitmap APIs
- iosurface raw output APIs
- metal raw output APIs
- dx11 raw output APIs
- targeted dxgi raw output API: `WindowsDxgiVideoFrame::get_dxgi_surface`

### Internalized

- `macos_frame_iosurface`
- `macos_metal_texture_for_video_frame`
- `windows_dx11_surface_for_video_frame`
- `windows_dx11_texture_for_video_frame`
- `windows_dxgi_surface_for_video_frame`

### Still public due to compatibility

- `IoSurface`
- `MacosIoSurfaceVideoFrameExt`
- `MetalVideoFrameExt`
- `MetalCaptureStreamExt`
- `WindowsDx11VideoFrame`
- `WindowsDx11CaptureStream`
- `WindowsDxgiVideoFrame`
- `WindowsDxgiCaptureStream`
- `WindowsDxgiCaptureStream::{get_dxgi_adapter,get_dxgi_device}`

### Diagnostic policy

Diagnostic APIs remain public because they are not primary capture output APIs.

No diagnostic API is deprecated in Phase 15I.
