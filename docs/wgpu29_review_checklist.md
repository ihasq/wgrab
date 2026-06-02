# wgpu29 review checklist

## API migration

- [ ] `wgpu = 29.0.3`
- [ ] old `hal` feature removed
- [ ] old callback-style `as_hal` removed
- [ ] backend mismatch handled explicitly

## Windows DX12

- [ ] no `d3d12::ComPtr` in wgpu path
- [ ] no `winapi::um::d3d12::ID3D12Resource` in wgpu path
- [ ] no `transmute_copy` COM lifetime hack
- [ ] Windows DX12 backend smoke passes

## macOS Metal

- [ ] no `raw_device().lock()` in wgpu path
- [ ] no `metal::Texture` in wgpu path
- [ ] `objc2-metal` texture path compiles
- [ ] macOS Metal backend smoke passes

## CI

- [ ] Windows DX12 backend smoke passes
- [ ] macOS Metal arm64 backend smoke passes
- [ ] macOS Metal Intel backend smoke passes
- [ ] Ubuntu Lavapipe Vulkan smoke passes
- [ ] capture probe marker is visible in job summary
- [ ] marker missing is failure
- [ ] strict capture remains manual-only

## Deferred

- [ ] full `objc2` 0.6 migration
- [ ] public `metal` API deprecation
- [ ] full formatting cleanup
- [ ] local physical capture validation if needed
