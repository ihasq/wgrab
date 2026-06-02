# Changelog

## Unreleased - wgpu29 successor baseline

### Added

- GitHub Actions backend smoke tests for Windows DX12, macOS Metal, and Ubuntu Lavapipe Vulkan.
- `wgpu_backend_smoke` example for backend/runtime validation.
- CI-mode capture probe handling with explicit markers.
- Descriptor debug path for wgpu capture interop validation.
- Successor and API compatibility documentation.

### Changed

- Updated optional `wgpu` integration to `wgpu 29.0.3`.
- Reworked `Device::as_hal` usage for the current wgpu API.
- Migrated Windows DX12 wgpu interop internals to `windows` crate COM types.
- Migrated macOS wgpu Metal interop internals to `objc2-metal`.

### Preserved

- Existing CrabGrab API surface where practical.
- Existing public `metal` API.
- Existing `objc2 = 0.5` wrapper.
- Existing feature names where practical.

### Metadata

- Updated repository metadata to `https://github.com/ihasq/wgrab`.

### Release

- Selected RC tag candidate: `wgrab-wgpu29-rc1`.
- crates.io publishing remains deferred.

### Deferred

- Full `objc2` 0.6 migration.
- Public `metal` API deprecation.
- Public typed error redesign.
- Full formatting cleanup.
- Crate publishing decision.
