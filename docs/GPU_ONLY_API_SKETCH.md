# GPU-only API sketch

## Primary types

```rust
pub struct WgpuCaptureStream {
    // internal
}

pub struct WgpuCaptureFrame {
    // internal
}

pub struct WgpuCaptureConfig {
    // internal
}
```

## Frame API

```rust
impl WgpuCaptureFrame {
    pub fn texture(&self) -> &wgpu::Texture;
    pub fn size(&self) -> wgpu::Extent3d;
    pub fn format(&self) -> wgpu::TextureFormat;
    pub fn usage(&self) -> wgpu::TextureUsages;
}
```

## Optional view helper

```rust
impl WgpuCaptureFrame {
    pub fn create_view(&self, descriptor: Option<&wgpu::TextureViewDescriptor>) -> wgpu::TextureView;
}
```

## Device ownership

wgrab should not create an unrelated hidden `wgpu::Device`.

The user should provide a `wgpu::Device` or a wrapper around it.

Reason:

- the captured texture must be usable by the caller's renderer
- HAL interop requires backend compatibility
- hidden devices make cross-device texture use invalid or impossible

## Backend contract

The user-provided `wgpu::Device` must use the platform backend required by the capture implementation.

- Windows: DX12
- macOS: Metal

If the backend does not match, wgrab returns a backend mismatch error.

## Texture lifetime

A `WgpuCaptureFrame` owns or retains the underlying platform texture/resource for as long as the `wgpu::Texture` wrapper is valid.

The frame must not expose CPU-readable memory.

## Transitional compatibility

The current `WgpuVideoFrameExt::get_wgpu_texture` API can serve as an intermediate compatibility bridge.

The eventual GPU-only API should prefer a frame type that exposes metadata (`size`, `format`, `usage`) alongside the texture so callers do not need platform-specific inspection.
