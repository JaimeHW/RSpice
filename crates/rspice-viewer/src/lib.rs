//! Published-page figure runtime.
//!
//! `rspice-publish` renders every figure as static SVG inside the page
//! document; this crate is the optional layer above it. When a reader
//! activates a figure, the page fetches the figure's sealed hydration
//! payload and mounts this runtime on the figure's canvas: schematic sheets
//! gain pan and zoom, plots gain live axes and cursor readout over the
//! sealed datasets. Hydration is progressive enhancement — any rejection
//! (integrity, schema, transform) leaves the static rendering in place, so
//! the runtime never presents an approximation of sealed results.
//!
//! The crate is wasm-first but native-testable: everything except the
//! `wasm-bindgen` boundary compiles and tests on the host, and the palette
//! module ratchets color parity against the page stylesheet in
//! `rspice-publish`.

pub mod payload;
pub mod plot;
pub mod scene;
pub mod theme;
pub mod transform;

#[cfg(target_arch = "wasm32")]
mod web;
