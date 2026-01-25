//! RSpice Application Entry Point
//!
//! Cross-platform entry point for RSpice UI.
//! - Desktop: Uses dioxus desktop runtime
//! - Web: Delegates to library's launch_web() function

// =============================================================================
// egui Entry Point - Commercial-grade GPU-native UI
// =============================================================================
// This is the recommended entry point for production use.
// Provides 60fps GPU-accelerated rendering without webview overhead.

#[cfg(all(not(target_arch = "wasm32"), feature = "egui"))]
fn main() {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting RSpice UI with egui (commercial-grade GPU rendering)...");

    // Configure eframe options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("RSpice - Circuit Simulator")
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([800.0, 600.0]),
        vsync: true,
        multisampling: 4,
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    // Launch the egui application
    if let Err(e) = eframe::run_native(
        "RSpice",
        options,
        Box::new(|cc| Ok(Box::new(rspice_ui::RSpiceApp::new(cc)))),
    ) {
        log::error!("Failed to launch RSpice: {}", e);
        std::process::exit(1);
    }
}

// =============================================================================
// Dioxus Native Entry Point (experimental)
// =============================================================================

#[cfg(all(not(target_arch = "wasm32"), feature = "native", not(feature = "egui")))]
fn main() {
    use rspice_ui::App;

    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting RSpice UI with dioxus-native (experimental)...");

    // Launch with native renderer (direct wgpu, no webview)
    dioxus::native::launch(App);
}

// =============================================================================
// Dioxus Desktop Entry Point (webview fallback)
// =============================================================================

#[cfg(all(
    not(target_arch = "wasm32"),
    not(feature = "egui"),
    not(feature = "native")
))]
fn main() {
    use dioxus::desktop::{Config, WindowBuilder};
    use rspice_ui::App;

    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting RSpice UI with desktop webview...");

    // Configure window
    let config = Config::new().with_menu(None).with_window(
        WindowBuilder::new()
            .with_title("RSpice - Circuit Simulator")
            .with_inner_size(dioxus::desktop::LogicalSize::new(1400.0, 900.0))
            .with_position(dioxus::desktop::LogicalPosition::new(100.0, 100.0)),
    );

    // Launch desktop application
    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}

// Web entry point
#[cfg(target_arch = "wasm32")]
fn main() {
    rspice_ui::launch_web();
}
