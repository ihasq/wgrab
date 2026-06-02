#[cfg(target_os = "macos")]
/// Macos-specific extensions
pub mod macos;

#[cfg(target_os = "macos")]
pub(crate) use macos as platform_impl;

#[cfg(target_os = "windows")]
/// Windows-specific extensions
pub mod windows;

#[cfg(target_os = "windows")]
pub(crate)  use windows as platform_impl;

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub(crate) mod unsupported;

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub(crate) use unsupported as platform_impl;
