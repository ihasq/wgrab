# GPU-only internal dependencies

## macOS GPU-only path

```text
ScreenCaptureKit
  -> IOSurface
  -> Metal texture / objc2-metal
  -> wgpu HAL Metal texture
  -> wgpu::Texture
  -> WgpuCaptureFrame
```

## Windows GPU-only path

```text
Windows capture / D3D11
  -> shared resource / synchronization
  -> D3D12 resource
  -> wgpu HAL DX12 texture
  -> wgpu::Texture
  -> WgpuCaptureFrame
```

## Public boundary

Public:

- `WgpuCaptureFrame`
- `WgpuVideoFrameGpuOnlyExt`
- `WgpuCaptureConfig`
- `WgpuCaptureStream`

Internal:

- IOSurface
- Metal texture
- D3D11 resource
- D3D12 resource
- DXGI shared handles
- platform synchronization

## Phase 15I boundary

Public boundary:

- `WgpuCaptureFrame`
- `WgpuVideoFrameGpuOnlyExt`

Compatibility boundary:

- deprecated bitmap/raw platform shims

Internal boundary:

- IOSurface bridge
- Metal bridge
- D3D/DXGI resource bridge

## Phase 16R runtime boundary

Runtime public output:

- `WgpuCaptureFrame`

Runtime public usage:

- `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame`
- `WgpuCaptureFrame::texture`
- `WgpuCaptureFrame::create_view`
- `WgpuCaptureFrame::size`
- `WgpuCaptureFrame::format`
- `WgpuCaptureFrame::usage`

Deprecated raw platform shims are not used by the GPU-only example.
