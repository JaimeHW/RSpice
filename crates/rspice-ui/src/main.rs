//! RSpice Application Entry Point
//!
//! Cross-platform entry point for RSpice UI.
//! - Desktop: Uses dioxus desktop runtime
//! - Web: Delegates to library's launch_web() function

// Desktop entry point
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use dioxus::desktop::{Config, WindowBuilder};
    use rspice_ui::App;

    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting RSpice UI...");

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
