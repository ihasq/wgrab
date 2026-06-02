#![cfg(target_os = "macos")]
#![cfg(feature = "metal")]

// The legacy public Metal raw-output API was removed in Phase 19X.
// Metal interop now lives behind the `wgpu` GPU-only path.
