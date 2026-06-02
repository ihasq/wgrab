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
