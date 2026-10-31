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

/// Calculator adaptation over retained application results.
#[cfg(test)]
pub(crate) use rspice_app_results::analysis;

/// Application binding for schematic and symbol editors.
pub(crate) mod schematic;

pub(crate) mod simulation {
    //! Application simulation setup, source preparation and dispatch
    //! coordination.
    //!
    //! Commercial-grade simulation execution and configuration. Manages the
    //! complete simulation workflow from netlist generation to result display.

    pub(crate) use rspice_app_runs::simulation::controller;
    pub(crate) use rspice_simulation_contract::accuracy;
    pub(crate) use rspice_simulation_contract::config;
    #[cfg(test)]
    pub(crate) use rspice_simulation_contract::cost_probe;
    pub(crate) mod dialog;
    #[cfg(test)]
    pub(crate) use rspice_app_runs::simulation::multi_run;
    #[cfg(test)]
    pub(crate) use rspice_app_runs::simulation::output_contract;
    #[cfg(test)]
    pub(crate) use rspice_app_runs::simulation::results;
    pub(crate) use rspice_app_runs::simulation::status;
    #[cfg(test)]
    pub(crate) use rspice_app_runs::simulation::veriloga_tests;
    pub(crate) use rspice_app_state::simulation::placed_sources;
    pub(crate) use rspice_app_state::simulation::plan;
    pub(crate) use rspice_app_state::simulation::stimulus_realize;
    pub(crate) use rspice_app_state::simulation::table_route;
    pub(crate) use rspice_simulation::execution;

    // The netlist generator, options translator, and
    // engine bridge are all reached through their own modules; flattening their
    // types here duplicated the path without shortening any call site.
    pub use controller::SimulationController;
    pub(crate) use rspice_app_runs::simulation::{
        NetlistShown, RunHost, ViewerHost, ViewerReading,
    };
    #[cfg(test)]
    pub use rspice_simulation::results::SampledWaveform;
}

/// Project-bound property editing workflows.
pub(crate) mod properties;

/// Shared UI kit exports used by application presentation.
pub(crate) use rspice_app_workbench::ui;

/// The contract-driven application workbench. This is the only owner of
/// application chrome, responsive composition, and top-level navigation.
pub(crate) mod workbench;

/// Result document exports and application integration fixtures.
#[cfg(test)]
pub(crate) use rspice_app_state::results;

/// Shared product identity and outcome vocabulary.
pub(crate) use rspice_app_state::product;

/// Strict project-scoped Automation/CI workflow language and deterministic
/// evidence artifact rendering. This domain is UI-framework independent.
use rspice_app_services::automation_runtime;
pub(crate) mod automation_workflow;

// =============================================================================
// Core Infrastructure
// =============================================================================

pub(crate) mod services {
    //! Application service adapters and platform integration.
    //!
    //! Services Module
    //!
    //! Backend services for the RSpice UI application.
    //! Contains specialized processing and analysis utilities.

    pub(crate) use rspice_app_cloud::services::cloud_account;
    #[cfg(test)]
    mod drc_tests;
    pub(crate) use rspice_app_cloud::services::license;
    pub(crate) use rspice_app_services::services::model_hub;
    pub(crate) mod model_qualification;
    pub(crate) use rspice_app_services::services::recent_files;
    pub(crate) use rspice_app_services::services::session;

    // No flattening re-exports: every consumer of a service type names the module
    // that defines it, so the owner of a name is readable at the use site.
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

/// The build this process runs, installed by the shell at startup for the
/// layers below it.
pub(crate) use rspice_app_services::build_identity;
/// Clock shims for the browser build. `std::time::{Instant, SystemTime}` trap
/// at runtime on wasm32-unknown-unknown, so every layer uses these instead.
pub(crate) use rspice_app_state::time_compat;

/// Locating the production half of a source file that inspects itself, for the
/// guards that assert their own shipped code takes no panic shortcuts.
#[cfg(test)]
mod source_guard;

/// The real on-disk identity of the temporary directory, for fixtures that
/// hand paths to a subject which records them through `std::fs::canonicalize`.
#[cfg(test)]
use rspice_app_state::fixture_root;

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
