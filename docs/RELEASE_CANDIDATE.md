# wgrab release candidate notes

## RC status

This branch is a release candidate for the wgrab successor baseline.

## Baseline

- branch: `wgpu29-phase5c-marker-hardening`
- commit: `b684e64746970884e0bb5a0715e2d413f9d74049`

## Successor direction

`ihasq/wgrab` is maintained as a successor fork of CrabGrab.

The project direction is to preserve the CrabGrab API surface while modernizing internal platform and GPU interop implementation.

## Major modernization

- Updated `wgpu` support to `29.0.3`.
- Removed old `wgpu` `hal` feature usage.
- Replaced callback-style `Device::as_hal` with guard-style API.
- Modernized Windows DX12 wgpu interop to `windows` crate COM types.
- Modernized macOS wgpu Metal interop path to `objc2-metal`.
- Preserved existing public `metal` API.
- Preserved existing `objc2 = 0.5` wrapper.
- Added Windows/macOS/Ubuntu GitHub Actions backend smoke tests.
- Added marker-gated capture probes.

## CI gate

Required for this RC:

- `wgpu runtime / Windows DX12 wgpu runtime`
- `wgpu runtime / macOS Metal wgpu runtime (macos-15, arm64)`
- `wgpu runtime / macOS Metal wgpu runtime (macos-15-intel, intel)`
- `wgpu linux vulkan smoke / Ubuntu Lavapipe Vulkan smoke`

Manual-only:

- strict capture runtime

## Package metadata review

- package name: `crabgrab`
- version: `0.4.0`
- repository: `https://github.com/AugmendTech/CrabGrab`
- license: `MIT OR Apache-2.0`
- crates.io publish: deferred

The repository metadata still points to the original CrabGrab repository. This phase intentionally does not change package metadata; updating it is an owner release decision.

## GitHub Actions gate

### Required

- wgpu runtime / Windows DX12 wgpu runtime
- wgpu runtime / macOS Metal wgpu runtime (macos-15, arm64)
- wgpu runtime / macOS Metal wgpu runtime (macos-15-intel, intel)
- wgpu linux vulkan smoke / Ubuntu Lavapipe Vulkan smoke

### Capability probe

- Windows capture probe: marker-gated
- macOS capture probe: marker-gated

### Manual-only

- Manual strict capture runtime

## Tag candidates

Not created in this phase.

Candidate names:

- `wgrab-wgpu29-rc1`
- `successor-baseline-wgpu29-rc1`

## Deferred

- crates.io publishing
- release tag creation
- full `objc2` migration
- public `metal` API deprecation
- public typed error redesign
- full formatting cleanup
- dedicated physical strict capture runner
