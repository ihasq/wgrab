# wgrab-wgpu29-rc1

## Status

Release candidate for the wgrab successor baseline.

This is not a crates.io release.

## Baseline

- Branch: `wgpu29-phase5c-marker-hardening`
- Commit: `038d1c451fddfb5b69f98416d6ac13974b9d0f4f`
- Tag: `wgrab-wgpu29-rc1`

## Project direction

`ihasq/wgrab` is maintained as a successor fork of CrabGrab.

The direction is to preserve the CrabGrab API surface while modernizing internal platform and GPU interop implementation.

## Highlights

- Updated optional `wgpu` integration to `wgpu 29.0.3`.
- Removed old `wgpu` `hal` feature usage.
- Replaced callback-style `Device::as_hal` usage with current guard-style API.
- Modernized Windows DX12 wgpu interop to `windows` crate COM types.
- Modernized macOS wgpu Metal interop path to `objc2-metal`.
- Preserved the existing public `metal` API.
- Preserved the existing `objc2 = 0.5` wrapper.
- Added GitHub Actions backend smoke tests for:
  - Windows DX12
  - macOS Metal arm64
  - macOS Metal Intel
  - Ubuntu Lavapipe Vulkan
- Added marker-gated capture probes.
- Added descriptor debug path for wgpu capture interop validation.

## Validation

Latest accepted validation before tagging:

- Windows DX12 backend smoke: success
- macOS Metal arm64 backend smoke: success
- macOS Metal Intel backend smoke: success
- Ubuntu Lavapipe Vulkan backend smoke: success
- Windows target check: success
- macOS x86 target check: success
- macOS arm target check: success
- forbidden grep: empty
- public API snapshot diff from phase7S: none

## Compatibility

Preserved:

- Existing CrabGrab API surface where practical
- Existing public `metal` API
- Existing `objc2 = 0.5` wrapper
- Existing feature names where practical

Deferred:

- crates.io publishing
- package rename decision
- version bump decision
- crate ownership / owner transfer decision
- full `objc2` migration
- public `metal` API deprecation
- typed error API redesign
- full formatting cleanup
- dedicated physical strict capture runner

## Notes

This tag is intended as a source-level release candidate for the successor fork. It is not a published crate release.
