//! RSpice UI - Modern Circuit Simulator Interface
//!
//! A professional-grade GUI for the RSpice circuit simulation engine,
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
