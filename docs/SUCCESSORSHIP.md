# wgrab succession policy

## Project status

`ihasq/wgrab` is maintained as a successor fork of CrabGrab.

The goal is to preserve the CrabGrab API surface where practical while modernizing internal platform interop and graphics dependencies.

## Compatibility policy

- Existing public APIs should be preserved unless there is a documented safety or correctness reason to change them.
- Internal implementation may change to follow current platform APIs.
- `wgpu` interop is maintained against current `wgpu` APIs.
- Public `metal` APIs are preserved for compatibility.
- Full migration to newer `objc2` APIs is deferred unless it can be done without breaking existing users.

## Modernization policy

Allowed modernization:

- Updating `wgpu`
- Updating internal HAL interop
- Updating Windows DX12 internals to `windows` crate COM types
- Updating macOS wgpu interop internals to `objc2-metal`
- Adding CI backend smoke tests
- Adding debug-only descriptor validation

Not allowed without explicit owner decision:

- Removing public `metal` APIs
- Renaming public modules
- Changing public error types
- Dropping Windows or macOS support
- Requiring paid or self-hosted runners for normal CI
