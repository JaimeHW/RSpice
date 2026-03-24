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
//! - `viewers/` - Specialized viewer selection
//! - `services/` - Backend services (file I/O, simulation runner)
//! - `state/` - Application state management
//! - `utils/` - Utility functions

// Temporary allowance for existing external/SPICE naming conventions.
#![allow(non_snake_case)]
#![cfg_attr(
    test,
    allow(
        clippy::assertions_on_constants,
        clippy::bool_assert_comparison,
        clippy::cloned_ref_to_slice_refs,
        clippy::default_constructed_unit_structs,
        clippy::expect_fun_call,
        clippy::field_reassign_with_default,
        clippy::items_after_test_module,
        clippy::len_zero,
        clippy::manual_range_contains,
        clippy::manual_repeat_n,
        clippy::needless_range_loop,
        clippy::unnecessary_cast,
        clippy::unnecessary_get_then_check,
        clippy::unnecessary_unwrap,
        clippy::useless_vec
    )
)]

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

/// Utility functions and helpers
pub mod utils;

/// Shared output specification helpers for analysis/sensitivity paths
pub(crate) mod output_spec;

// =============================================================================
// Re-exports
// =============================================================================

/// Re-export the main application type
pub use common::RSpiceApp;
