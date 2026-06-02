# wgrab GPU-only API policy

## Direction

wgrab is moving toward a GPU-only capture API.

Captured regions should be exposed as `wgpu::Texture`-backed frames, not CPU-readable byte buffers.

## Public contract

The public capture output should provide:

- `wgpu::Texture`
- texture size
- texture format
- optional texture view helper
- backend/device compatibility errors

The public capture output should not provide:

- `Vec<u8>`
- `&[u8]`
- bitmap/image buffers
- CPU-readable staging buffers
- mapped buffers
- raw platform graphics objects as the primary output

## Meaning of "no CPU/DRAM path"

This policy means:

- wgrab does not intentionally copy captured frames into CPU-readable memory.
- wgrab does not expose mapped buffers.
- wgrab does not expose byte slices or image buffers.
- wgrab does not implement texture readback APIs.

It does not claim that platform internals never use unified memory or OS-managed memory.

## Internal implementation

Internal platform interop may use:

- IOSurface
- Metal texture
- D3D11/D3D12 resources
- wgpu HAL interop
- synchronization primitives

These are implementation details.

## Compatibility stance

This is a breaking direction relative to CrabGrab compatibility.

The previous `wgrab-wgpu29-rc1` tag remains the compatibility-oriented modernization baseline.

## CI requirements

Required:

- Windows DX12 backend smoke
- macOS Metal backend smoke
- Ubuntu Lavapipe Vulkan backend smoke
- compile check for GPU-only examples

Required negative checks:

- no public API returning `Vec<u8>` capture frames
- no public API returning byte slices for capture frames
- no public API exposing readback buffers

Capability probe:

- capture runtime may be `CI_CAPTURE_SMOKE_OK` or `CI_CAPTURE_UNAVAILABLE`

## API layers

### Primary API

- `WgpuCaptureFrame`
- `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame`

This is the preferred GPU-only capture output.

### Advanced lower-level GPU API

- `WgpuVideoFrameExt::get_wgpu_texture`

This remains public as a lower-level GPU API.

It does not expose CPU-readable capture output and therefore does not violate the GPU-only direction.

### Deprecated legacy APIs

- CPU-readable bitmap APIs
- raw platform output APIs

## Future web target

wgrab may support browser capture in the future.

The intended web direction is:

```text
getDisplayMedia -> VideoFrame -> WebGPU
```

This future web path must preserve the GPU-only principle.

It should not expose CPU-readable capture bytes.

WebGPU / web_sys support is a roadmap item and is not a blocker for publishing
`wgrab 0.1.0`.
