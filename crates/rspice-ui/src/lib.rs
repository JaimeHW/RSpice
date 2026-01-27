//! RSpice UI - Commercial-Grade Circuit Simulator Interface
//!
//! A high-performance GUI for the RSpice circuit simulation engine,
//! built with egui for GPU-accelerated desktop deployment.
//!
//! # Architecture
//!
//! The crate is organized by domain following professional EDA tool patterns:
//!
//! - `analysis/` - Analysis viewers (Bode, FFT, Smith chart, etc.)
//! - `schematic/` - Schematic editor (canvas, export, toolbar, symbols)
//! - `waveform/` - Waveform viewer with cursors and measurements
//! - `simulation/` - Simulation controller and dialogs
//! - `cross_probing/` - Cross-probing between views
//! - `panels/` - Side panels (project browser, properties)
//! - `properties/` - Property editing and design variables
//! - `common/` - Shared components (menu bar, status bar, theme)
//! - `viewers/` - Integrated multi-view displays
//! - `services/` - Backend services (file I/O, simulation runner)
//! - `state/` - Application state management
//! - `ui/` - Dialog state and shortcuts
//! - `utils/` - Utility functions

// Suppress warnings during development
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

// =============================================================================
// Domain Modules (Organized by Feature)
// =============================================================================

/// Analysis viewers - Bode, FFT, histogram, Nyquist, pole-zero, Smith chart, eye diagram
pub mod analysis;

/// Schematic editor - Canvas, export, toolbar, symbol library
pub mod schematic;

/// Waveform viewer - Multi-trace display with cursors and measurements
pub mod waveform;

/// Simulation management - Controller, dialogs, netlist generation
pub mod simulation;

/// Cross-probing - Synchronized selection between schematic and waveform
pub mod cross_probing;

/// Side panels - Project browser and properties panel
pub mod panels;

/// Property editing - Component properties and design variables
pub mod properties;

/// Common UI components - Menu bar, status bar, theme, main app
pub mod common;

/// Integrated viewers - Multi-view synchronized displays
pub mod viewers;

// =============================================================================
// Core Infrastructure
// =============================================================================

/// Backend services (file I/O, simulation runner)
pub mod services;

/// File I/O (library parser, session, netlist, waveform)
pub mod io;

/// Integration (event bus, manager, state sync)
pub mod integration;

/// Application state management
pub mod state;

/// Dialog state and keyboard shortcuts
pub mod ui;

/// Utility functions and helpers
pub mod utils;

// =============================================================================
// Re-exports
// =============================================================================

/// Re-export the main application type
pub use common::RSpiceApp;
