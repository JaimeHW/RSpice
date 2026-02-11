//! Common UI Components Module
//!
//! Shared UI components used across the application.
//! Contains the main application shell and reusable UI elements.
//!
//! - `app` - Main RSpiceApp application struct (egui::App implementation)
//! - `menu_bar` - Application menu bar
//! - `status_bar` - Status bar with simulation state and metrics
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
pub mod analysis_navigation;
pub mod examples;
pub(crate) mod file_actions;
pub(crate) mod export_workflow;
pub(crate) mod file_workflow;
pub mod menu_bar;
pub mod simulation_analysis_tabs;
pub mod status_bar;
pub mod theme;

// Re-export main application type
pub use app::{AppState, ConsoleMessage, RSpiceApp};
pub use theme::RSpiceTheme;
