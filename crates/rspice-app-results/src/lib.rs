//! The application's results feature.
//!
//! The Results workspace and its data browser, the result documents and the
//! viewers that draw them, the calculator and its expressions, run details and
//! the result authority the inspector states, result import and waveform
//! export, and the feature's commands, state slot and registration. It works on
//! the shared model in `rspice-app-state`, reads the runs of `rspice-app-runs`,
//! uses the platform services of `rspice-app-services` and registers with the
//! workbench framework of `rspice-app-workbench`, and it reaches no other
//! feature: the commands it labels and binds come in through the framework's
//! context.
//!
//! `rspice-app` re-exports each module at the path it has always had, so the
//! modules keep the layout they had inside it.

#![cfg_attr(target_arch = "wasm32", allow(dead_code))]
#![allow(clippy::too_many_arguments)]
#![deny(clippy::print_stdout, clippy::print_stderr)]

// The model's, the services', the run tier's and the framework's modules, at
// the paths the results feature has always named them by.
use rspice_app_services::io;
use rspice_app_state::{diagnostics, product, quantity, results, source_revision, state};
use rspice_app_workbench::ui;

pub mod analysis;

pub(crate) mod simulation {
    //! The run tier's modules, at their application paths.

    pub(crate) use controller::SimulationController;
    pub(crate) use rspice_app_runs::simulation::controller;
    #[cfg(test)]
    pub(crate) use rspice_app_runs::simulation::results;
    pub(crate) use rspice_app_runs::simulation::{ViewerHost, ViewerReading};
    pub(crate) use rspice_app_state::simulation::plan;
}

pub mod workbench {
    //! The results feature's modules, at their application paths, and the
    //! model's, the services' and the framework's items they name through
    //! `crate::workbench::`.

    pub(crate) use documents::result_document::ResultViewer;
    pub(crate) use rspice_app_state::workbench::preferences;
    pub(crate) use rspice_app_state::workbench::preferences::{
        ChoicePreference, ComplexNumberDisplay, EngineeringExportFormat, ScalarPreference,
        TogglePreference,
    };
    pub(crate) use rspice_app_workbench::workbench::framework;
    pub(crate) use rspice_app_workbench::workbench::layout;
    pub(crate) use rspice_app_workbench::workbench::localization;
    pub(crate) use rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId;
    pub(crate) use rspice_ui_kit::panels as design_system;

    pub(crate) mod browser {
        //! The browser build's file import, which the result-dataset import uses.

        #[cfg(target_arch = "wasm32")]
        pub(crate) use rspice_app_services::workbench::browser::file_import;
    }

    pub mod app_state {
        pub(crate) use rspice_app_services::workbench::slots::platform::PlatformState;
        pub(crate) use rspice_app_state::workbench::app_state::model::Model;

        pub mod session {
            pub mod calculator;
        }
        pub mod viewer_capabilities;
    }
    pub mod chrome {
        pub mod activity_rail {
            pub mod results;
        }
        pub mod document_bar {
            pub(crate) use rspice_app_workbench::workbench::chrome::document_bar::*;

            pub mod results;
        }
        pub mod status_bar {
            pub mod results;
        }
        pub mod toolbar {
            pub mod results;
        }
    }
    pub mod commands {
        pub mod result_navigation;
    }
    pub mod docks {
        pub(crate) use inspector::run_details::{RunDetailsDialogState, show as show_run_details};

        pub mod inspector {
            pub mod result_authority;
            pub mod results;
            pub mod run_details;
        }
        pub mod navigator {
            pub mod results;
            pub mod results_actions;
            pub mod results_rows;
        }
    }
    pub mod documents {
        pub mod result_document;
    }
    pub mod menu_bar {
        pub(crate) use waveform_export::{
            DatasetExportPicker, action_export_csv_with_io, show_dataset_export_picker,
        };

        pub mod waveform_export;
    }
    pub mod results;
    pub mod slots {
        pub(crate) use results::ResultsState;

        pub mod results;
    }
    pub mod state {
        pub(crate) use rspice_app_state::workbench::workspace::Workspace;
        pub(crate) use rspice_app_workbench::workbench::state::console_views::ConsoleChannel;
        pub(crate) use rspice_app_workbench::workbench::state::workbench_session;
        pub(crate) use workbench_session::{ConsolePage, WorkbenchSession, WorkspaceDocumentId};

        pub mod plan_provenance;
    }
    pub mod workflows {
        pub(crate) use rspice_app_services::workbench::workflows::export_workflow;

        pub mod result_import_workflow;
    }
}
