//! RSpice Desktop Application Entry Point

use dioxus::prelude::*;
use rspice_ui::App;

fn main() {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting RSpice UI...");

    // Launch desktop application
    dioxus::launch(App);
}
