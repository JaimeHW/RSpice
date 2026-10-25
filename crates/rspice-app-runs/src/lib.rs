//! The application's run tier.
//!
//! Preparing a run from the model, its preflight, dispatching it to the
//! engine, watching it, and retaining and post-processing what it produced:
//! the simulation controller and its prepared runs and campaigns, the run's
//! output contract, the run preflight, and the runs' own slot. It works on
//! the shared model in `rspice-app-state` with the platform services of
//! `rspice-app-services`, and reaches no feature: what it asks of the
//! application it asks through `RunHost`.
//!
//! `rspice-app` re-exports each module at the path it has always had, so the
//! modules keep the layout they had inside it.

#![cfg_attr(target_arch = "wasm32", allow(dead_code))]
#![allow(clippy::too_many_arguments)]
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

// The model's and the services' modules, at the paths the run tier has always
// named them by.
#[cfg(any(test, feature = "test-support"))]
use rspice_app_services::io;
#[cfg(test)]
use rspice_app_state::results;
use rspice_app_state::{diagnostics, product, quantity, state, time_compat};

/// Locating the production half of a source file that inspects itself, for the
/// guards that assert their own shipped code takes no panic shortcuts.
#[cfg(test)]
mod source_guard;

pub mod simulation {
    //! Application simulation setup, source preparation and dispatch
    //! coordination.
    //!
    //! Commercial-grade simulation execution and configuration. Manages the
    //! complete simulation workflow from netlist generation to result display.

    pub use rspice_simulation_contract::accuracy;
    pub use rspice_simulation_contract::config;
    pub mod controller;
    pub use rspice_simulation::execution;
    #[cfg(test)]
    pub use rspice_simulation_contract::cost_probe;
    pub(crate) mod manual_run;
    pub mod multi_run;
    #[cfg(test)]
    pub(crate) mod netlist_gen;
    pub mod output_contract;
    pub use rspice_app_state::simulation::placed_sources;
    pub use rspice_app_state::simulation::plan;
    pub use rspice_simulation::point_family;
    #[cfg(any(test, feature = "test-support"))]
    pub mod results;
    mod run_host;
    #[cfg(test)]
    pub(crate) mod run_set_tests;
    pub use rspice_app_state::simulation::stimulus_realize;
    pub use rspice_app_state::simulation::table_route;
    pub use rspice_simulation::runner;
    pub mod status;
    #[cfg(any(test, feature = "test-support"))]
    pub mod veriloga_tests;

    // The netlist generator, options translator, and
    // engine bridge are all reached through their own modules; flattening their
    // types here duplicated the path without shortening any call site.
    pub use config::AnalysisConfig;
    pub use controller::SimulationController;
    pub use rspice_simulation::results::{SampledWaveform, SimulationResult};
    pub use run_host::{NetlistShown, RunHost, ViewerReading};
    // Test-only aliases for private modules: execution reaches the bridge and
    // these helpers through their module paths.
    pub use runner::SimulationRunner;
    pub use status::SimulationStatus;
}

pub mod workbench {
    pub mod app_state {
        pub mod run_preflight;
    }
    pub mod preflight;
    pub mod slots {
        pub mod runs;
    }
}
