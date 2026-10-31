//! The application's models feature.
//!
//! The Models & PDKs workspace (the project's models, the installed packs and
//! the RSpice library, the Model Hub's shelf and releases, corners and bins),
//! the device-model editor and its qualification, measurement correlation,
//! unsigned PDK authoring and the PDK settings, together with the feature's
//! commands, its state slot and its registration. It works on the shared
//! model in `rspice-app-state`, uses the platform services of
//! `rspice-app-services` and the run tier of `rspice-app-runs`, and registers
//! with the workbench framework of `rspice-app-workbench`; it reaches no
//! other feature.
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

// The model's, the services', the run tier's and the framework's modules, at
// the paths the models feature has always named them by.
use rspice_app_services::io;
use rspice_app_state::{diagnostics, product, quantity, state, time_compat};
use rspice_app_workbench::ui;

pub mod services {
    //! The models feature's services, at their application paths.

    pub mod model_qualification;
    pub(crate) use rspice_app_services::services::model_hub;
}

mod simulation {
    pub(crate) use rspice_app_runs::simulation::controller;
}

pub mod workbench {
    //! The models feature's modules, at their application paths.

    pub(crate) use rspice_app_workbench::workbench::routing::navigation::RouteTransitionSource;
    pub(crate) use rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId;
    pub(crate) use rspice_app_workbench::workbench::routing::surface_route::SurfaceRoute;
    pub(crate) use rspice_app_workbench::workbench::{framework, layout};
    pub(crate) use rspice_ui_kit::panels as design_system;

    pub mod app {
        /// Arming the schematic's cursor for a part the shelf placed is an
        /// operation on the model, at the path the shelf's documentation and
        /// its cases name it by.
        pub use rspice_app_state::workbench::app::schematic::placement::arm_pack_part;

        pub mod dialogs {
            pub mod pdk_settings;
        }
    }
    pub mod app_state {
        pub(crate) use rspice_app_services::workbench::slots::platform::PlatformState;
        pub(crate) use rspice_app_state::workbench::app_state::design_history;
        pub(crate) use rspice_app_state::workbench::app_state::model::{self, Model};

        pub mod session {
            pub mod pdk_settings;
        }
    }
    pub mod chrome {
        pub use rspice_app_workbench::workbench::chrome::*;

        pub mod document_bar {
            pub use rspice_app_workbench::workbench::chrome::document_bar::*;

            pub mod models;
        }
        pub mod toolbar {
            pub use rspice_app_workbench::workbench::chrome::toolbar::*;

            pub mod models;
        }
    }
    pub mod docks {
        pub use rspice_app_workbench::workbench::docks::*;

        pub mod inspector {
            pub use rspice_app_workbench::workbench::docks::inspector::*;

            pub mod models;
        }
        pub mod navigator {
            pub use rspice_app_workbench::workbench::docks::navigator::*;

            pub mod models;
        }
    }
    pub mod documents {
        pub use rspice_app_workbench::workbench::documents::*;

        pub mod model_correlation;
        pub mod model_editor;
    }
    pub mod models;
    pub mod slots {
        pub(crate) use models::ModelsState;

        pub mod models;
    }
    pub(crate) mod state {
        pub(crate) use rspice_app_state::workbench::workspace::Workspace;
        pub(crate) use rspice_app_workbench::workbench::state::workbench_session::{
            self, WorkbenchSession, WorkspaceDocumentId,
        };
    }
    pub mod surfaces {
        pub use rspice_app_workbench::workbench::surfaces::*;

        pub mod model_correlation;
        pub mod model_correlation_controller;
        pub mod model_editor;
        pub mod models;
        pub mod pdk_technology_admin;
    }
    pub(crate) mod workflows {
        pub(crate) use rspice_app_services::workbench::workflows::export_workflow;
    }
}
