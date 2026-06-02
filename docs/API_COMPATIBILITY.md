# API compatibility policy

## Current compatibility target

The current target is CrabGrab API compatibility with modernized internals.

## Public API protection

The following should be treated as protected unless explicitly approved:

- public modules
- public structs
- public enums
- public traits
- public functions
- feature names
- platform extension traits
- capture configuration builders
- frame access APIs
- wgpu extension APIs
- metal feature public API
- windows feature public API

## Deferred breaking changes

The following are deferred:

- replacing existing public `metal` API with `objc2-metal`
- replacing public `Result<_, String>` APIs with typed errors
- full `objc2` 0.6 migration
- removing legacy feature names
- global formatting cleanup that touches unrelated files

## Internal modernization

Internal implementation details may change as long as public behavior remains compatible.

Approved internal changes include:

- `wgpu` 29 interop
- guard-style `as_hal`
- `windows` crate COM types for DX12 wgpu path
- `objc2-metal` types for macOS wgpu interop path
- CI backend smoke examples
