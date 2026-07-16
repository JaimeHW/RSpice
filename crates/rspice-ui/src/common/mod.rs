//! Common UI Components Module
//!
//! Shared UI components used across the application.
//! Contains application workflows and reusable UI services.
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
#[cfg(target_arch = "wasm32")]
pub(crate) mod browser_accessibility;
#[cfg(target_arch = "wasm32")]
pub(crate) mod browser_download;
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) mod browser_file_import;
pub mod examples;
pub(crate) mod export_workflow;
pub(crate) mod file_actions;
pub(crate) mod file_workflow;
pub mod logging;
pub mod menu_bar;
pub(crate) mod netlist_workflow;
pub(crate) mod project_lifecycle;
pub(crate) mod project_workflow;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod recovery_checkpoint;
pub mod shortcut_artifacts;
pub mod shortcut_library_persistence;
pub mod shortcut_profile_workflow;
pub mod simulation_analysis_tabs;
pub mod time_compat;

// Re-export main application type
pub use app::{AppState, ConsoleMessage, RSpiceApp};

/// Run `work` on a background thread natively; inline on wasm32 for short UI
/// tasks that are not routed through the browser simulation worker.
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
