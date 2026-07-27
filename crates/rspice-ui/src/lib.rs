//! RSpice UI - Commercial-Grade Circuit Simulator Interface
//!
//! A high-performance GUI for the RSpice circuit simulation engine,
//! built with egui for GPU-accelerated desktop deployment.
//!
//! # Architecture
//!
//! This crate is the RSpice application, deliberately kept whole. The
//! simulation engine lives in `rspice-core` and `rspice-veriloga`; everything
//! the application itself owns — persisted design state, project I/O, run
//! orchestration, viewer mathematics, and chrome — stays here. Modules that
//! never mention `egui` are therefore expected, not misplaced: they are the
//! application layer, not the presentation layer.
//!
//! Because there is no crate boundary to lean on, the module layering is
//! enforced by `tests/module_layering.rs` instead. A module may reference
//! any module below it and none at or above it. Lowest layer first:
//!
//! | Layer | Modules | Owns |
//! |-------|---------|------|
//! | 0 | `product`, `quantity` | Framework-independent contracts, typed identities, unit-safe presentation policy |
//! | 1 | `results`, `ui` | Versioned result documents; the design system (tokens, palette, widgets, plot engine) |
//! | 2 | `hardcopy` | Persisted page setup, print mappings, and source-set records |
//! | 3 | `state` | The persisted design, library, and project model |
//! | 4 | `analysis`, `automation_workflow`, `io` | Viewer mathematics, the CI workflow language, file formats |
//! | 5 | `services` | DRC, licensing, and the per-analysis engine adapters |
//! | 6 | `simulation` | Analysis plans, netlist generation, run orchestration |
//! | 7 | `properties` | Component property editing |
//! | 8 | `panels` | Docked auxiliary panels |
//! | 9 | `schematic` | The schematic document engine |
//! | 10 | `workbench` | Application chrome, surfaces, navigation, commands |
//! | 11 | `common` | The application root: [`RSpiceApp`], dialogs, workflows |
//!
//! Known departures from this order are recorded, counted, and ratcheted
//! down in that test's `ALLOWED_VIOLATIONS` table. Adding to it is not a way
//! to unblock new code — a fresh violation means the code is in the wrong
//! module.

// Temporary allowance for existing external/SPICE naming conventions.
#![allow(non_snake_case)]
// NOTE: this crate previously carried a blanket `#![allow(deprecated)]`. It
// was hiding 63 real egui 0.34 migration sites across 20 APIs — the panel
// constructors (`TopBottomPanel`/`SidePanel` -> `Panel::top`/`left`, and
// `Panel::show(ctx)` -> `show_inside(ui)`), `Frame::rounding`,
// `Ui::set_enabled`, `Context::screen_rect`, and the `Popup` family. Those
// warnings are left visible on purpose: the migration is an egui API change
// with layout consequences that needs its own visual verification pass, and a
// silent allow is how it stayed invisible in the first place.
// The desktop build detaches from its console on Windows and the browser
// build has no stderr at all, so anything printed is a diagnostic nobody
// will ever read. Route it through `log` and the application log buffer.
#![deny(clippy::print_stdout, clippy::print_stderr)]
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

/// Simulation management - Controller, dialogs, netlist generation
pub mod simulation;

/// Side panels - Project browser and properties panel
pub mod panels;

/// Property editing - Component properties and design variables
pub mod properties;

/// Common UI components - Menu bar, status bar, theme, main app
pub mod common;

/// The RSpice design system - tokens, palettes, fonts, icons, widgets
pub mod ui;

/// Persisted page-setup contracts and deterministic pagination. Document
/// adapters, scene rendering, the platform print boundary, and the dialogs
/// live in `workbench::hardcopy`; this is the layer `state` can persist.
pub mod hardcopy;

/// The contract-driven application workbench. This is the only owner of
/// application chrome, responsive composition, and top-level navigation.
pub mod workbench;

/// Versioned visualization documents, immutable dataset bindings, exact-data
/// queries, viewer compatibility, and progressive result operations.
pub mod results;

/// Canonical commercial product model, typed identities, command outcomes,
/// and fail-closed object lifecycles. This layer is UI-framework independent.
pub mod product;

/// Strict project-scoped Automation/CI workflow language and deterministic
/// evidence artifact rendering. This domain is UI-framework independent.
pub mod automation_workflow;

// =============================================================================
// Core Infrastructure
// =============================================================================

/// Backend services (file I/O, simulation runner)
pub mod services;

/// File I/O (library parser, session, netlist, waveform)
pub mod io;

/// Application state management
pub mod state;

/// Unit-safe user presentation and UI quantity-input policy. Values entering
/// or leaving this module are always expressed in their documented SI base
/// units; deck dialect and PDK database-unit semantics live elsewhere.
pub mod quantity;

/// Shared output specification helpers for analysis/sensitivity paths
pub(crate) mod output_spec;

// =============================================================================
// Re-exports
// =============================================================================

/// Re-export the main application type
pub use common::RSpiceApp;

#[cfg(target_arch = "wasm32")]
pub fn run_rspice_ui_worker_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    simulation::runner::worker_contract::run_worker_request_value(value)
}

#[cfg(target_arch = "wasm32")]
pub fn run_rspice_ui_veriloga_compile_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    workbench::code_workspace::run_veriloga_worker_request_value(value)
}

#[cfg(target_arch = "wasm32")]
pub fn run_rspice_ui_hardcopy_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    common::app::run_hardcopy_worker_request_value(value)
}
