# GPU-only migration guide

## Summary

wgrab is moving from CPU-readable and raw platform output APIs toward GPU-only
capture output.

Use:

- `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame`
- `WgpuCaptureFrame::texture`
- `WgpuCaptureFrame::create_view`
- `WgpuCaptureFrame::size`
- `WgpuCaptureFrame::format`

Avoid:

- CPU bitmap APIs
- byte slice APIs
- image buffer APIs
- raw IOSurface / Metal / D3D output APIs

## Before: CPU bitmap path

```rust
// old style
let bitmap = frame.get_bitmap()?;
```

## After: GPU-only path

```rust
use crabgrab::feature::wgpu::{
    WgpuCaptureFrame,
    WgpuVideoFrameGpuOnlyExt,
    WgpuVideoFramePlaneTexture,
};

let gpu_frame: WgpuCaptureFrame = frame.get_wgpu_capture_frame(
    WgpuVideoFramePlaneTexture::Rgba,
    Some("capture-frame"),
)?;

let view = gpu_frame.create_view(None);
let texture = gpu_frame.texture();
let size = gpu_frame.size();
let format = gpu_frame.format();
```

## Why

The GPU-only API avoids exposing captured regions as CPU-readable bytes and
keeps the capture output in a form directly usable by a wgpu renderer.

## Removed legacy APIs

The deprecated CPU-readable bitmap and raw platform output APIs have been
removed in the GPU-only branch.

Use:

- `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame`
- `WgpuCaptureFrame`
- `WgpuCaptureFrame::texture`
- `WgpuCaptureFrame::create_view`

Advanced users may use:

- `WgpuVideoFrameExt::get_wgpu_texture`

## Raw platform API note

Raw platform capture output APIs are no longer part of the GPU-only public
capture output surface.

## Primary runtime path

The primary runtime path is now:

1. create or provide a backend-compatible `wgpu::Device`
2. capture a frame
3. call `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame`
4. use `WgpuCaptureFrame::texture` or `WgpuCaptureFrame::create_view`

The `wgpu_texture_only_capture` example demonstrates this path and does not
read captured pixels back to CPU memory.

## Removed legacy API warning

Deprecated CPU-readable and raw platform output APIs have been removed on the
GPU-only branch.

New code should migrate to:

- `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame`
- `WgpuCaptureFrame`
- `WgpuCaptureFrame::texture`
- `WgpuCaptureFrame::create_view`

## Migration table

| Legacy API | Replacement |
|---|---|
| `get_bitmap` | `get_wgpu_capture_frame` |
| `get_iosurface` | `WgpuCaptureFrame` |
| `get_metal_texture` | `WgpuCaptureFrame` |
| `get_dx11_texture` | `WgpuCaptureFrame` |
| `get_dxgi_surface` | `WgpuCaptureFrame` |
