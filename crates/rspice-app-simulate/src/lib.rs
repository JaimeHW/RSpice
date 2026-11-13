//! The RSpice application's simulate feature: the Simulation and
//! Verification Studios, the analysis plan and its run sets, the plan manager
//! and the jobs manager.
//!
//! The feature's files keep the paths they had inside `rspice-app`. This root
//! declares them there and re-exports, at the same paths, the lower crates'
//! items they name.

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

use rspice_app_services::io;
use rspice_app_state::{diagnostics, product, quantity, state, time_compat};
// Only the cases and the fixtures the shell's tests reach read the results crate.
#[cfg(any(test, feature = "test-support"))]
use rspice_app_state::results;
use rspice_app_workbench::ui;

pub mod simulation {
    pub(crate) use rspice_app_runs::simulation::controller;
    pub use rspice_app_runs::simulation::status;
    pub(crate) use rspice_app_runs::simulation::{
        NetlistShown, RunHost, ViewerHost, ViewerReading,
    };
    pub(crate) use rspice_app_state::simulation::placed_sources;
    pub(crate) use rspice_app_state::simulation::plan;
    pub(crate) use rspice_simulation::execution;
    pub(crate) use rspice_simulation_contract::accuracy;
    pub(crate) use rspice_simulation_contract::config;

    pub(crate) use controller::SimulationController;

    pub mod dialog;
}

pub mod workbench {
    pub(crate) use rspice_app_runs::workbench::preflight;
    pub(crate) use rspice_app_services::workbench::workflows;
    pub(crate) use rspice_app_state::workbench::preferences;
    pub(crate) use rspice_app_workbench::workbench::{framework, layout, localization, routing};
    pub(crate) use rspice_ui_kit::panels as design_system;

    pub use preferences::ChoicePreference;
    pub(crate) use routing::navigation::RouteTransitionSource;
    pub(crate) use routing::surface_catalog::SurfaceId;
    pub(crate) use routing::surface_route::SurfaceRoute;

    pub mod app {
        pub mod actions {
            pub mod tuning_staging;
        }
        pub mod dialogs {
            pub mod plan_removal;
        }

        #[cfg(test)]
        pub(in crate::workbench) use dialogs::plan_removal::REVEAL_BLOCKER;
        pub(crate) use dialogs::plan_removal::{
            PlanRemovalConsequence, PlanRemovalTarget, PlanRemovalTone,
        };
    }
    pub mod app_state {
        pub(crate) use rspice_app_runs::workbench::app_state::run_preflight;
        pub(crate) use rspice_app_state::workbench::app_state::{design_checks, model, sim_setup};
        pub(crate) use rspice_app_workbench::workbench::app_state::session;
        pub(crate) use rspice_simulation_contract::plan_catalog::SimulationPlanImportDocument;

        pub(crate) use model::{Model, SlotFacts};
        pub(crate) use session::shortcuts::runtime_command_platform;
        pub(crate) use sim_setup::plan_catalog::{
            SimulationPlanCloneOptions, SimulationPlanLineage, SimulationPlanName,
        };
        pub(crate) use sim_setup::{
            AcSetup, DcSetup, ReferencePvtPoint, SimSetupState, SimulationSavePolicy, TranSetup,
        };
    }
    pub mod chrome {
        pub use rspice_app_workbench::workbench::chrome::*;

        pub mod document_bar {
            pub use rspice_app_workbench::workbench::chrome::document_bar::*;

            pub mod simulate;
        }
        pub mod toolbar {
            pub use rspice_app_workbench::workbench::chrome::toolbar::*;

            pub mod run_config_chip;
            pub mod simulate;
        }
    }
    pub mod docks {
        pub use rspice_app_workbench::workbench::docks::*;

        pub mod inspector {
            pub use rspice_app_workbench::workbench::docks::inspector::*;

            pub mod simulation;
            pub mod verify;
        }
        pub mod navigator {
            pub use rspice_app_workbench::workbench::docks::navigator::*;

            pub mod simulate;
            pub mod verification_footer;
            pub mod verify;
        }
    }
    pub mod documents {
        pub use rspice_app_workbench::workbench::documents::*;
    }
    pub mod simulate;
    pub mod slots {
        pub(crate) use rspice_app_runs::workbench::slots::runs;
        pub(crate) use rspice_app_services::workbench::slots::platform;

        pub(crate) use platform::PlatformState;
        pub(crate) use runs::RunsState;
        pub(crate) use simulate::SimulateState;

        pub mod simulate;
    }
    pub mod state {
        pub(crate) use rspice_app_state::workbench::workspace::Workspace;
        pub(crate) use rspice_app_workbench::workbench::state::workbench_session::{
            self, WorkbenchSession, WorkspaceDocumentId,
        };

        // The session types derive their stored form.
        use serde::{Deserialize, Serialize};

        pub mod simulate_session;
        pub mod workflow_drafts;

        pub use simulate_session::*;
        pub use workflow_drafts::*;
    }
    pub mod surfaces {
        pub use rspice_app_workbench::workbench::surfaces::*;

        pub mod simulate;
        pub mod verify;
    }
    pub mod tools {
        pub mod jobs_manager;
    }
}
