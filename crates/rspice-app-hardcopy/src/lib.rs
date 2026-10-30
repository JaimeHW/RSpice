//! The application's hardcopy feature.
//!
//! Page Setup, Print and Export Active View with their workers and print
//! backends, report authoring, the retained hardcopy sources and the
//! publication snapshot, with the feature's commands, its state slot and its
//! registration. It works on the shared model in `rspice-app-state`, uses the
//! platform services of `rspice-app-services` and the run tier of
//! `rspice-app-runs`, and registers with the workbench framework of
//! `rspice-app-workbench`. It reaches no other feature: what it prints of the
//! results workspace comes in through the framework's results port.
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

// The model's, the services' and the framework's modules, at the paths the
// hardcopy feature has always named them by.
use rspice_app_services::io;
use rspice_app_state::{diagnostics, product, quantity, results, state, time_compat};
use rspice_app_workbench::ui;

pub mod workbench {
    //! The hardcopy feature's modules, at their application paths.

    pub(crate) use rspice_app_state::workbench::preferences;
    pub(crate) use rspice_app_workbench::workbench::routing::navigation::RouteTransitionSource;
    pub(crate) use rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId;
    pub(crate) use rspice_app_workbench::workbench::routing::surface_route::SurfaceRoute;
    pub(crate) use rspice_app_workbench::workbench::{framework, state};

    pub mod app {
        pub(crate) use dialogs::hardcopy::{
            HardcopyDialogState, HardcopyParts, HardcopyWorkflow, open_hardcopy_workflow,
            render_hardcopy_dialog,
        };
        pub(crate) use rspice_app_state::workbench::app::schematic::edit_authority::SchematicEditAuthority;

        pub mod dialogs {
            pub mod hardcopy;
        }
    }
    pub mod app_state {
        pub(crate) use rspice_app_services::workbench::slots::platform::PlatformState;
        pub(crate) use rspice_app_state::workbench::app_state::model::Model;
    }
    pub(crate) mod lifecycle {
        pub(crate) use rspice_app_services::workbench::lifecycle::project_lifecycle;
    }
    pub(crate) mod workflows {
        pub(crate) use rspice_app_services::workbench::workflows::export_workflow;
    }
    pub mod hardcopy;
    pub mod hardcopy_adapters;
    pub mod slots {
        pub(crate) use hardcopy::HardcopyState;

        pub mod hardcopy;
    }
}
