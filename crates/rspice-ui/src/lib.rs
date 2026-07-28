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
//! | 4 | `analysis`, `automation_workflow`, `diagnostics`, `io` | Viewer mathematics, the CI workflow language, console/log model, file formats |
//! | 5 | `services` | DRC, licensing, and the per-analysis engine adapters |
//! | 6 | `simulation` | Analysis plans, netlist generation, run orchestration |
//! | 7 | `properties` | Component property editing |
//! | 8 | `schematic` | The schematic document engine |
//! | 9 | `workbench` | The application shell: [`RSpiceApp`], state, dialogs, chrome, surfaces, commands, and the workflows that mutate them |
//!
//! `workbench` is about half the crate, so one position in this table does not
//! describe it. Its own submodules are ordered by that test's
//! `WORKBENCH_LAYERS`, on the same rules.
//!
//! Known departures from both orders are recorded, counted, and ratcheted
//! down in the `ALLOWED_VIOLATIONS` and `ALLOWED_WORKBENCH_VIOLATIONS` tables.
//! Adding to either is not a way to unblock new code — a fresh violation
//! means the code is in the wrong module.

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
// NOTE: closing the public surface (see the visibility note below) turned 192
// items into `dead_code` warnings. They were never reachable — the compiler
// simply could not say so while their modules were `pub`. All 192 are dead on
// native, on wasm32, and with tests compiled; `--lib` alone is not enough,
// because it hides anything only a `#[cfg(test)]` block or a browser-only
// path calls.
//
// They are deliberately not swept. `workbench::simulation_analysis_tabs` is
// the reason: 26 of its items are unreachable, but it is one coherent catalog
// of 25 analysis tabs whose two index tables happen to have no reader.
// Deleting the unreferenced half would leave a catalog that no longer
// describes the product. Retire these per module — decide whether each thing
// is finished-but-unwired or genuinely abandoned — not with a bulk delete.
//
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
pub(crate) mod analysis;

/// Schematic editor - Canvas, export, toolbar, symbol library
pub(crate) mod schematic;

/// Simulation management - Controller, dialogs, netlist generation
pub(crate) mod simulation;


/// Property editing - Component properties and design variables
pub(crate) mod properties;

/// The RSpice design system - tokens, palettes, fonts, icons, widgets
pub(crate) mod ui;

/// Persisted page-setup contracts and deterministic pagination. Document
/// adapters, scene rendering, the platform print boundary, and the dialogs
/// live in `workbench::hardcopy`; this is the layer `state` can persist.
pub(crate) mod hardcopy;

/// The contract-driven application workbench. This is the only owner of
/// application chrome, responsive composition, and top-level navigation.
pub(crate) mod workbench;

/// Versioned visualization documents, immutable dataset bindings, exact-data
/// queries, viewer compatibility, and progressive result operations.
pub(crate) mod results;

/// Canonical commercial product model, typed identities, command outcomes,
/// and fail-closed object lifecycles. This layer is UI-framework independent.
pub(crate) mod product;

/// Strict project-scoped Automation/CI workflow language and deterministic
/// evidence artifact rendering. This domain is UI-framework independent.
pub(crate) mod automation_workflow;

// =============================================================================
// Core Infrastructure
// =============================================================================

/// Backend services (file I/O, simulation runner)
pub(crate) mod services;

/// File I/O (library parser, session, netlist, waveform)
pub(crate) mod io;

/// Application state management
pub(crate) mod state;

/// Unit-safe user presentation and UI quantity-input policy. Values entering
/// or leaving this module are always expressed in their documented SI base
/// units; deck dialect and PDK database-unit semantics live elsewhere.
pub(crate) mod quantity;

/// Diagnostics the application reports about itself: the console message
/// model and the structured, filterable application log.
pub(crate) mod diagnostics;

/// Clock shims for the browser build. `std::time::{Instant, SystemTime}` trap
/// at runtime on wasm32-unknown-unknown, so every layer uses these instead.
pub(crate) mod time_compat;

/// Shared output specification helpers for analysis/sensitivity paths
pub(crate) mod output_spec;

// =============================================================================
// The crate's entire external surface
// =============================================================================
//
// `rspice-ui` is an application, not a library. Its only consumers are the
// desktop and browser binary in `main.rs`, the `license_tool` example, and
// the integration tests -- nothing in the workspace depends on it. Every
// module above is therefore `pub(crate)`, and everything reachable from
// outside is named here.
//
// That is not tidiness. A `pub` module is one the compiler must assume some
// unseen caller uses, so it cannot report an unreachable item inside it. The
// eight modules that used to be `pub` covered 262k lines -- 45% of the crate
// -- in which dead code could not be detected at all. Adding a `pub mod` to
// reach something from a test re-opens that hole; add a re-export here
// instead.

/// The application root, constructed by both the desktop and browser entry
/// points.
pub use workbench::RSpiceApp;

/// Native logging environment for the desktop binary.
#[cfg(not(target_arch = "wasm32"))]
pub use workbench::logging::native_log_env;

/// Typed identities, for `tests/simulation_configuration_contract.rs`.
pub use product::{AnalysisInstanceId, SimulationPlanId};

/// License-key verification, shared with the `license_tool` example that
/// issues the keys this parses.
pub use services::license::{
    LicensePayload, SIGNING_DOMAIN, crockford_encode, date_from_unix_days, group5, parse_and_verify,
};

/// Design-variable netlist emission, pinned by the configuration contract.
pub use simulation::netlist_gen::{DesignVariableNetlistContext, design_variable_parameter_lines};

/// The persisted project model the configuration contract exercises.
pub use state::{
    CellViewRef, DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity,
    DesignVariableRange, DesignVariableScope, DesignVariableSweepEligibility, ProjectWorkspace,
    SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision,
    SavedOutputStreaming, SimulationPlanPayload, SimulationPlanPayloadRecord,
};

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
    workbench::documents::code_workspace::run_veriloga_worker_request_value(value)
}

#[cfg(target_arch = "wasm32")]
pub fn run_rspice_ui_hardcopy_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    workbench::app::run_hardcopy_worker_request_value(value)
}
