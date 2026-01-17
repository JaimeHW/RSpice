//! RSpice Desktop Application Entry Point

use dioxus::desktop::{Config, WindowBuilder};
use rspice_ui::App;

fn main() {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting RSpice UI...");

    // Configure window to be visible and centered
    let config = Config::new().with_window(
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
