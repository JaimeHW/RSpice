//! The RSpice application's project feature: the project pages, the
//! launcher and examples browser, the Library browser and its dialogs,
//! configuration sets, design management and revision history.
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
use rspice_app_workbench::ui;

mod services {
    pub(crate) use rspice_app_services::services::recent_files;
}

mod simulation {
    pub(crate) use rspice_app_state::simulation::table_route;
}

pub mod workbench {
    pub(crate) use rspice_app_services::workbench::lifecycle;
    pub(crate) use rspice_app_state::workbench::preferences;
    pub(crate) use rspice_app_workbench::workbench::{framework, layout, localization, routing};
    pub(crate) use rspice_ui_kit::panels as design_system;

    pub(crate) use localization::MessageCatalog;
    pub(crate) use preferences::DrawingSheetPersonalPreferences;
    pub(crate) use routing::navigation::RouteTransitionSource;
    pub(crate) use routing::surface_catalog::SurfaceId;
    pub(crate) use routing::surface_route::SurfaceRoute;
    pub(crate) use state::WorkbenchSession;

    pub mod app {
        pub(crate) use rspice_app_state::workbench::app::schematic::edit_authority::SchematicEditAuthority;
        pub(crate) use rspice_app_workbench::workbench::app::dialogs::confirmation_state::ConfirmationAction;

        pub mod dialogs {
            pub(crate) use rspice_app_workbench::workbench::app::dialogs::review_primitives;

            pub mod check_and_save;
            pub mod check_and_save_validation;
            pub mod design_management;
            pub mod library {
                mod shared;

                pub use shared::LIBRARY_CATALOG_STALE_MESSAGE;
                pub(in crate::workbench) use shared::validate_lcv_name;
            }
            pub mod project;
            pub mod project_revision_history;
        }

        pub(crate) use dialogs::check_and_save::open_check_and_save_dialog;
        pub(crate) use dialogs::design_management::{
            DesignManagementDialogState, open_design_management_dialog,
        };
        pub(crate) use dialogs::project::TechnologyAttachmentDialogState;
        pub(crate) use dialogs::project_revision_history::ProjectRevisionHistoryDialogState;
    }
    mod app_state {
        pub(crate) use rspice_app_state::workbench::app_state::{
            design_checks, design_history, model, sim_setup,
        };
        pub(crate) use rspice_app_state::workbench::workspace::{
            default_model_library_manager, restore_session_model_library_manager,
        };

        pub(crate) use crate::workbench::slots::PlatformState;
        pub(crate) use design_checks::DesignCheckStatus;
        pub(crate) use design_history::DesignManagementHistoryEntry;
        pub(crate) use model::{Model, SlotFacts};
        pub(crate) use sim_setup::SimSetupState;
        pub(crate) use sim_setup::plan_catalog::SimulationPlanLineage;
    }
    mod browser {
        #[cfg(target_arch = "wasm32")]
        pub(crate) use rspice_app_services::workbench::browser::download;
        #[cfg(target_arch = "wasm32")]
        pub(crate) use rspice_app_services::workbench::browser::file_import;
    }
    pub mod chrome {
        pub use rspice_app_workbench::workbench::chrome::*;

        pub mod document_bar {
            pub use rspice_app_workbench::workbench::chrome::document_bar::*;

            pub mod project;
        }
        pub mod toolbar {
            pub use rspice_app_workbench::workbench::chrome::toolbar::*;

            pub mod project;
        }
    }
    pub mod docks {
        pub use rspice_app_workbench::workbench::docks::*;

        pub mod inspector {
            pub use rspice_app_workbench::workbench::docks::inspector::*;

            pub mod project;
        }
        pub mod navigator {
            pub use rspice_app_workbench::workbench::docks::navigator::*;

            pub mod project;
        }
    }
    pub mod project;
    pub mod slots {
        pub(crate) use rspice_app_services::workbench::slots::platform;

        pub(crate) use platform::PlatformState;
        pub(crate) use project::{BuiltinModelsBrowser, ProjectState};

        pub mod project;
    }
    mod state {
        pub(crate) use rspice_app_state::workbench::workspace::Workspace;
        pub(crate) use rspice_app_workbench::workbench::state::workbench_session::{
            self, WorkbenchSession, WorkspaceDocumentId,
        };
    }
    pub mod surfaces {
        pub use rspice_app_workbench::workbench::surfaces::*;

        pub mod project;
    }
    pub mod tools {
        pub mod examples_browser;
        pub mod project_launcher;
    }
    pub mod workflows {
        pub(crate) use rspice_app_services::workbench::workflows::export_workflow;

        pub mod file_workflow;
        pub mod project_workflow;
    }
}
