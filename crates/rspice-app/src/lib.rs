//! Desktop and browser application composition.
//!
//! Domain data, execution, hardcopy rendering and reusable editor/viewer
//! presentation belong to the sibling crates. This crate binds those owners
//! to host services, document lifecycles, commands, navigation and workbench UI.
//!
//! Internal layers are checked by `tests/module_layering.rs`; package dependency
//! boundaries are checked by `tools/ci/check_app_crate_dependencies.py`.

// Temporary allowance for existing external/SPICE naming conventions.
#![allow(non_snake_case)]
// Desktop-only workbench paths remain unreachable in the browser build.
// Native and test builds continue to diagnose ordinary dead code.
#![cfg_attr(target_arch = "wasm32", allow(dead_code))]
// Rendering and transaction entry points may take independent source, state,
// layout and authority inputs. Group parameters when they form one contract.
#![allow(clippy::too_many_arguments)]
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

/// Application binding for schematic and symbol editors.
pub(crate) mod schematic;

pub(crate) mod simulation {
    //! The state tier's simulation modules and the run tier's controller, at
    //! the paths the shell's not-yet-moved readers still name. Every other
    //! reader names the owning crate's module.

    pub(crate) use rspice_app_state::simulation::placed_sources;
    pub(crate) use rspice_app_state::simulation::plan;
    pub(crate) use rspice_app_state::simulation::stimulus_realize;
    pub(crate) use rspice_app_state::simulation::table_route;

    pub use rspice_app_runs::simulation::controller::SimulationController;
}

/// Project-bound property editing workflows.
pub(crate) mod properties;

/// Shared UI kit exports used by application presentation.
pub(crate) use rspice_app_workbench::ui;

/// The contract-driven application workbench. This is the only owner of
/// application chrome, responsive composition, and top-level navigation.
pub(crate) mod workbench;

/// Shared product identity and outcome vocabulary.
pub(crate) use rspice_app_state::product;

/// Strict project-scoped Automation/CI workflow language and deterministic
/// evidence artifact rendering. This domain is UI-framework independent.
use rspice_app_services::automation_runtime;

// =============================================================================
// Core Infrastructure
// =============================================================================

pub(crate) mod services {
    //! The service modules at the paths the shell's not-yet-moved readers
    //! still name, and the design rule checks' tests. Every other reader names
    //! the owning crate's module.

    #[cfg(test)]
    mod drc_tests;
    pub(crate) use rspice_app_cloud::services::license;
    pub(crate) use rspice_app_services::services::model_hub;
}

/// Application file workflows and platform I/O adapters.
pub(crate) use rspice_app_services::io;

/// Application-held domain owners and session adapters.
pub(crate) use rspice_app_state::state;

/// Unit-safe user presentation and UI quantity-input policy. Values entering
/// or leaving this module are always expressed in their documented SI base
/// units; deck dialect and PDK database-unit semantics live elsewhere.
pub(crate) use rspice_app_state::quantity;

/// Diagnostics the application reports about itself: the console message
/// model and the structured, filterable application log.
pub(crate) use rspice_app_state::diagnostics;

/// Clock shims for the browser build. `std::time::{Instant, SystemTime}` trap
/// at runtime on wasm32-unknown-unknown, so every layer uses these instead.
pub(crate) use rspice_app_state::time_compat;

/// Locating the production half of a source file that inspects itself, for the
/// guards that assert their own shipped code takes no panic shortcuts.
#[cfg(test)]
mod source_guard;

/// Tests that construct the whole application to observe one slice's
/// behaviour, kept in the shell because only the shell can build it.
#[cfg(test)]
mod feature_tests;

// =============================================================================
// The crate's entire external surface
// =============================================================================
//
// Keep implementation modules private so the compiler can diagnose unused
// code. Expose entrypoint and integration contracts explicitly below.

/// The application root, constructed by both the desktop and browser entry
/// points.
pub use workbench::RSpiceApp;

/// The desktop binary's logger: stderr as before, plus the running analysis's
/// own Console log.
#[cfg(not(target_arch = "wasm32"))]
pub use workbench::logging::install_studio_logger;

/// Typed identities, for `tests/simulation_configuration_contract.rs`.
pub use product::{AnalysisInstanceId, SimulationPlanId};

/// The persisted project model the configuration contract exercises.
pub use state::{
    CellViewRef, DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity,
    DesignVariableRange, DesignVariableScope, DesignVariableSweepEligibility, ProjectWorkspace,
    SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
    SavedOutputPrecision, SavedOutputStreaming, SimulationPlanPayload, SimulationPlanPayloadRecord,
};
