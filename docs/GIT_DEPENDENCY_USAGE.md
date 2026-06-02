# Using wgrab as a Git dependency

Until package publishing strategy is finalized, downstream users can depend on this repository directly.

## Branch-based dependency

```toml
crabgrab = { git = "https://github.com/ihasq/wgrab", branch = "wgrab-gpu-only-api-design" }
```

## Revision-pinned dependency

```toml
crabgrab = { git = "https://github.com/ihasq/wgrab", rev = "<commit>" }
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
