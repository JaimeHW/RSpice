//! What the browser build has to do differently.
//!
//! Navigation, file import, and download are the three places where the web
//! platform refuses to behave like the desktop: history is a shared mutable
//! stack, a file picker is asynchronous and user-gestured, and saving is a
//! synthesized anchor click. Each is `#[cfg]`-gated to wasm32 except the
//! import token, which the tests exercise on both.

#[cfg(target_arch = "wasm32")]
pub(crate) mod accessibility;
#[cfg(target_arch = "wasm32")]
pub(crate) mod download;
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) mod file_import;
pub(crate) mod navigation;
