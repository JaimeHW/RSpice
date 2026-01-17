//! RSpice UI - Modern Circuit Simulator Interface
//!
//! A professional-grade GUI for the RSpice circuit simulation engine,
//! built with Dioxus for cross-platform desktop and web deployment.

#![allow(non_snake_case)]

mod app;
mod components;
mod state;
mod theme;
mod views;

pub use app::App;
pub use theme::Theme;
