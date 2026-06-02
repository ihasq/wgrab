# Using wgrab as a Git dependency

Until package publishing strategy is finalized, downstream users can depend on this repository directly.

## Branch-based dependency

```toml
wgrab = { git = "https://github.com/ihasq/wgrab", branch = "wgrab-crates-io-prep" }
```

## Revision-pinned dependency

```toml
wgrab = { git = "https://github.com/ihasq/wgrab", rev = "<commit>" }
```

## Future package alias example

If the successor crate is published as `wgrab`, users may be able to keep Rust imports stable with Cargo dependency aliasing:

```toml
crabgrab = { package = "wgrab", version = "..." }
```

## Notes

The GPU-only API is centered on:

- `WgpuCaptureFrame`
- `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame`

## Recommended current usage

Until registry publishing is decided, use a Git dependency.

```toml
wgrab = { git = "https://github.com/ihasq/wgrab", branch = "wgrab-crates-io-prep" }
```

For reproducible builds, pin a revision:

```toml
wgrab = { git = "https://github.com/ihasq/wgrab", rev = "<commit>" }
```

If a downstream project wants to keep `crabgrab::` imports temporarily, use a
Cargo dependency alias:

```toml
crabgrab = { package = "wgrab", git = "https://github.com/ihasq/wgrab", branch = "wgrab-crates-io-prep" }
```

## Current API direction

Use:

- `WgpuCaptureFrame`
- `WgpuVideoFrameGpuOnlyExt::get_wgpu_capture_frame`

Do not use removed legacy APIs:

- CPU bitmap output
- raw IOSurface output
- raw Metal output
- raw D3D11 frame output
- raw DXGI frame surface output
