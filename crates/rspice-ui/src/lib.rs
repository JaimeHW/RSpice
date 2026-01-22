//! RSpice UI - Modern Circuit Simulator Interface
//!
//! A high-performance GUI for the RSpice circuit simulation engine,
//! built with Dioxus for cross-platform desktop and web deployment.

// Suppress warnings for incomplete features (schematic editor, symbols, etc.)
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod app;
mod components;
mod state;
mod theme;
pub mod utils;
mod views;

pub use app::App;
pub use theme::Theme;

// =============================================================================
// Platform-specific initialization
// =============================================================================

/// Initialize and launch the web application.
/// Called from main.rs for WASM builds.
#[cfg(target_arch = "wasm32")]
pub fn launch_web() {
    // Set up better panic messages for debugging
    console_error_panic_hook::set_once();

    // Initialize console logging
    console_log::init_with_level(log::Level::Info).ok();

    log::info!("RSpice Web UI starting...");

    // Spawn async GPU initialization in the background
    // This runs concurrently with Dioxus startup for faster perceived load time
    wasm_bindgen_futures::spawn_local(async {
        log::info!("Initializing WebGPU...");
        match views::waveform_gpu::init_web_gpu().await {
            Some(_) => log::info!("WebGPU initialized successfully"),
            None => log::warn!("WebGPU not available, using SVG fallback"),
        }
    });

    // Launch Dioxus web application
    dioxus::launch(App);
}
