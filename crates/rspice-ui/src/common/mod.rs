//! Common UI Components Module
//!
//! Shared UI components used across the application.
//! Contains the main application shell and reusable UI elements.
//!
//! - `app` - Main RSpiceApp application struct (egui::App implementation)
//! - `menu_bar` - Application menu bar
//! - `theme` - Application theming and colors
//!
//! # Architecture
//!
//! The common module provides the application scaffolding:
//! - `RSpiceApp` is the main entry point that coordinates all other modules
//! - Menu bar provides file, edit, view, simulation, and help menus
//! - Status bar shows simulation progress and tool tips
//! - Theme provides consistent styling across all components

pub mod app;
pub mod examples;
pub(crate) mod export_workflow;
pub(crate) mod file_actions;
pub(crate) mod file_workflow;
pub mod menu_bar;
pub(crate) mod project_workflow;
pub mod simulation_analysis_tabs;
pub(crate) mod time_compat;

// Re-export main application type
pub use app::{AppState, ConsoleMessage, RSpiceApp};

/// Run `work` on a background thread natively; inline on wasm32, which has no
/// threads — the browser build solves on the UI thread until simulations move
/// into a Web Worker.
pub(crate) fn spawn_or_inline(work: impl FnOnce() + Send + 'static) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::thread::spawn(work);
    }
    #[cfg(target_arch = "wasm32")]
    {
        work();
    }
}
