//! RSpice Application Entry Point
//!
//! Cross-platform entry point for RSpice UI.
//! Uses egui for commercial-grade GPU-accelerated rendering.

// =============================================================================
// Desktop Entry Point - Commercial-grade GPU-native UI
// =============================================================================

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Initialize logging with wgpu_core set to warn to reduce noise
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info,wgpu_core=warn"),
    )
    .init();

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
// Web Entry Point (placeholder for future web support)
// =============================================================================

#[cfg(target_arch = "wasm32")]
fn main() {
    // Web support can be added in the future using egui web
    panic!("Web support is not yet implemented. Use desktop version.");
}
