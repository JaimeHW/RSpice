//! Desktop and browser application composition.
//!
//! Domain data, execution, hardcopy rendering and reusable editor/viewer
//! presentation belong to the sibling crates. This crate binds those owners
//! to host services, document lifecycles, commands, navigation and workbench UI.
//!
//! Internal layers and the application crates' conventions are checked by
//! `tests/app_conventions.rs`; package dependency boundaries are checked by
//! `tools/ci/check_app_crate_dependencies.py`.

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
// Modules
// =============================================================================

/// The contract-driven application workbench. This is the only owner of
/// application chrome, responsive composition, and top-level navigation.
pub(crate) mod workbench;

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
pub use rspice_app_state::product::{AnalysisInstanceId, SimulationPlanId};

/// The persisted project model the configuration contract exercises.
pub use rspice_app_state::state::{
    CellViewRef, DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity,
    DesignVariableRange, DesignVariableScope, DesignVariableSweepEligibility, ProjectWorkspace,
    SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
    SavedOutputPrecision, SavedOutputStreaming, SimulationPlanPayload, SimulationPlanPayloadRecord,
};
