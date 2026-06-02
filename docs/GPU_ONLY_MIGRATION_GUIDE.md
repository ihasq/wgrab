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

## What remains temporarily available

Legacy APIs remain available during the deprecation window, but they are no
longer the preferred path.

## Deferred removals

Actual removal is deferred to a later breaking-change phase.

## Raw platform API note

Raw platform APIs are deprecated or under review for future internalization.

Use `WgpuCaptureFrame` unless you are maintaining legacy interop code.

## Deprecated raw output shims

Deprecated raw platform APIs remain callable temporarily, but new code should
not use them.

Use `WgpuCaptureFrame` for capture output.

## Primary runtime path

The primary runtime path is now:

1. create or provide a backend-compatible `wgpu::Device`
2. capture a frame
3. call `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame`
4. use `WgpuCaptureFrame::texture` or `WgpuCaptureFrame::create_view`

The `wgpu_texture_only_capture` example demonstrates this path and does not
read captured pixels back to CPU memory.

## Future removal warning

Deprecated CPU-readable and raw platform output APIs are planned for removal in
a future breaking-change release.

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
