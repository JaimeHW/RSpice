//! The application's workbench framework.
//!
//! What the parts of the application share to work with one another without
//! reaching into one another's state: the command registry, feature
//! registration and its ports, the requests the parts raise for one another,
//! the workbench's own session, routing between surfaces, the interface text
//! catalog, and the chrome, docks and tools every feature is shown in. It works
//! on the shared model in `rspice-app-state`, the platform services of
//! `rspice-app-services` and the run status of `rspice-app-runs`, and reaches
//! no feature: what a feature contributes it contributes through the
//! framework's ports and contribution points.
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

// The model's, the services' and the run tier's modules, at the paths the
// workbench has always named them by.
use rspice_app_runs::simulation;
use rspice_app_services::{io, services};
use rspice_app_state::{diagnostics, product, quantity, state, time_compat};

/// Shared UI kit exports used by application presentation.
pub mod ui;

pub mod workbench {
    //! The workbench's framework modules, at their application paths.

    pub use rspice_ui_kit::panels as design_system;

    pub mod app {
        pub mod command_palette;
        pub mod dialogs {
            pub mod confirmation;
            pub mod confirmation_state;
            pub mod fact_grid;
            pub mod model_browser;
            pub mod operation_primitives;
            pub mod review_primitives;
            pub mod window_session;
        }
    }
    pub mod app_state {
        pub use rspice_app_services::workbench::slots::platform::PlatformState;
        pub use rspice_app_state::workbench::app_state::model::Model;
        pub use rspice_app_state::workbench::app_state::{diagnostic_store, waivers};
        pub mod problems_view;
        pub mod session {
            pub mod shortcuts;
        }
    }
    pub mod browser {
        pub use rspice_app_services::workbench::browser::background_alerts;
        pub mod navigation;
    }
    pub mod chrome;
    pub mod docks;
    pub mod documents;
    pub mod framework;
    pub mod layout;
    pub mod lifecycle {
        pub use rspice_app_services::workbench::lifecycle::project_lifecycle;
        pub mod window_session;
    }
    pub mod localization;
    pub mod routing;
    pub mod state {
        pub mod console_views;
        pub mod navigator_tree;
        pub mod retired_documents;
        pub mod workbench_session;

        pub use console_views::{
            ConsoleChannel, ConsoleReveal, ConsoleSelection, ConsoleSeverity, ConsoleView,
            NotificationFilter, SpecialistToolBrowserState, SpecialistToolFilter,
        };
        pub use rspice_app_state::workbench::workspace::Workspace;
        pub use workbench_session::{
            ConsolePage, Drawer, StatusPrompt, WidthClass, WorkbenchSession, WorkspaceDocumentId,
            WorkspaceLayoutState, default_console_height,
        };
    }
    pub mod surfaces;
    pub mod tools {
        pub mod notification_center;
        pub mod specialist_tool_browser;
    }
}
