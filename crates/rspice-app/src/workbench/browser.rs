//! What the browser build has to do differently.
//!
//! Navigation, file import, and download are the three places where the web
//! platform refuses to behave like the desktop: history is a shared mutable
//! stack, a file picker is asynchronous and user-gestured, and saving is a
//! synthesized anchor click. Each is `#[cfg]`-gated to wasm32 except the
//! import token, which the tests exercise on both. A tab also has no taskbar
//! to mark when a run ends out of sight, so it asks to notify instead; the
//! desktop's answer to that is always "unavailable".

#[cfg(target_arch = "wasm32")]
pub(crate) mod accessibility;
pub(crate) mod background_alerts;
#[cfg(target_arch = "wasm32")]
pub(crate) mod download;
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) mod file_import;
pub(crate) mod navigation;
#[cfg(all(target_arch = "wasm32", feature = "browser-qualification"))]
pub(crate) mod qualification;
