//! Tests that need the whole application.
//!
//! Each one observes a slice's behaviour through `AppState` or `RSpiceApp`,
//! which only the shell can construct, so it lives here rather than beside
//! the code it tests. One module per slice.

mod cloud;
mod code;
// Its tests run the whole frame through eframe's native test frame, and the
// browser runner runs no plain test.
#[cfg(not(target_arch = "wasm32"))]
mod design;
mod framework;
mod hardcopy;
mod model;
mod models;
mod project;
// The browser jobs' clicks and keys through eframe's native test frame.
#[cfg(not(target_arch = "wasm32"))]
mod replay;
mod results;
mod run;
mod services;
mod settings;
mod sheets;
mod simulate;
