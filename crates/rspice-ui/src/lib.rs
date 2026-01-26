//! RSpice UI - Modern Circuit Simulator Interface
//!
//! A high-performance GUI for the RSpice circuit simulation engine,
//! built with egui for commercial-grade GPU-accelerated desktop deployment.

// Suppress warnings for incomplete features
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

// =============================================================================
// Core Modules
// =============================================================================

/// Application services (file I/O, simulation runner, etc.)
pub mod services;

/// Application state management
pub mod state;

/// Utility functions and helpers
pub mod utils;

// =============================================================================
// egui Application
// =============================================================================

/// Commercial-grade egui application with GPU-native rendering
pub mod egui_app;

/// Re-export the main application type
pub use egui_app::RSpiceApp;
