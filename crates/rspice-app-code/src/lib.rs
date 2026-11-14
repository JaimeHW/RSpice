//! The RSpice application's code feature: the netlist and Verilog-A source
//! editors, their documents and workflows, the source dialogs and the
//! automation workspace.
//!
//! The feature's files keep the paths they had inside `rspice-app`. This root
//! declares them there and re-exports, at the same paths, the lower crates'
//! items they name.

#![allow(non_snake_case)]
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

use rspice_app_services::{automation_runtime, io};
#[cfg(test)]
use rspice_app_state::fixture_root;
use rspice_app_state::{diagnostics, product, quantity, state, time_compat};
use rspice_app_workbench::ui;

pub mod automation_workflow;
#[cfg(test)]
mod source_guard;

pub mod services {
    pub(crate) use rspice_app_services::services::recent_files;
}

pub mod simulation {
    pub(crate) use rspice_app_runs::simulation::NetlistShown;
    pub(crate) use rspice_app_runs::simulation::controller;
    pub(crate) use rspice_app_state::simulation::plan;
    pub(crate) use rspice_app_state::simulation::table_route;
}

pub mod workbench {
    pub(crate) use rspice_app_services::workbench::lifecycle;
    pub(crate) use rspice_app_workbench::workbench::{framework, localization};
    pub(crate) use rspice_ui_kit::panels as design_system;

    #[cfg(any(test, feature = "test-support"))]
    pub(crate) use localization::UiTextLocale;
    pub(crate) use localization::{MessageCatalog, MessageId};
    #[cfg(test)]
    pub(crate) use rspice_app_state::workbench::preferences::FolderListPreference;
    pub(crate) use state::WorkbenchSession;

    pub mod app {
        pub mod actions {
            pub mod veriloga_placement;
            pub mod veriloga_symbol;
        }
        pub mod dialogs {
            pub mod library {
                pub mod new_veriloga_device;
                pub mod veriloga_import;
            }
            pub mod source_document;
            pub mod source_find;
            pub mod source_language_tools;
            pub mod veriloga_build_settings;
        }

        pub(crate) use dialogs::source_document::{
            begin_new_top_deck_in, open_source_document_dialog_in,
            source_document_dialog_is_available,
        };
        pub(crate) use dialogs::veriloga_build_settings::{
            VerilogABuildSettingsDraft, veriloga_build_settings_available,
        };
    }
    pub mod app_state {
        pub(crate) use rspice_app_state::workbench::app_state::design_history::{
            VerilogABuildSettingsHistoryEntry, VerilogAModuleBinding,
            publish_symbol_definition_candidate,
        };
        pub(crate) use rspice_app_state::workbench::app_state::{
            cell_usage, diagnostic_store, model,
        };

        pub(crate) use crate::workbench::slots::PlatformState;
        pub(crate) use model::Model;
    }
    pub mod browser {
        #[cfg(target_arch = "wasm32")]
        pub(crate) use rspice_app_services::workbench::browser::file_import;
    }
    pub mod chrome {
        pub use rspice_app_workbench::workbench::chrome::*;

        pub mod document_bar {
            pub use rspice_app_workbench::workbench::chrome::document_bar::*;

            pub mod code;
        }
        pub mod title_bar {
            pub use rspice_app_workbench::workbench::chrome::title_bar::*;

            pub mod code;
        }
    }
    pub mod code;
    pub mod commands {
        pub mod code_context;
    }
    pub mod docks {
        pub use rspice_app_workbench::workbench::docks::*;

        pub mod inspector {
            pub use rspice_app_workbench::workbench::docks::inspector::*;

            pub mod code_workspace;
            pub mod veriloga;
        }
        pub mod navigator {
            pub use rspice_app_workbench::workbench::docks::navigator::*;

            pub mod netlist;
            pub mod source_bundle;
            pub mod veriloga;
        }
    }
    pub mod documents {
        pub use rspice_app_workbench::workbench::documents::*;

        pub mod code_workspace;
        pub mod editor_font;
        pub mod netlist_document;
        pub mod text_document_model;
        pub mod text_editor_commands;
        pub mod virtual_text_editor;
    }
    pub mod menu_bar {
        pub mod netlist_compat;
        pub mod netlist_export;

        pub(crate) use netlist_export::{
            action_export_generated_netlist_with_options, build_menu_netlist_in,
            netlist_failure_text, record_netlist_warnings_in,
        };
    }
    pub mod slots {
        pub(crate) use rspice_app_services::workbench::slots::platform;

        pub(crate) use code::{CodeState, netlist_shown_in, veriloga_compilation_in};
        pub(crate) use platform::PlatformState;

        pub mod code;
    }
    pub mod state {
        pub(crate) use rspice_app_state::workbench::workspace::Workspace;
        pub use rspice_app_workbench::workbench::state::workbench_session::ActiveSourceEditor;
        pub(crate) use rspice_app_workbench::workbench::state::workbench_session::{
            self, SourceReference, SourceReferences, WorkbenchSession, WorkspaceDocumentId,
        };
    }
    pub mod surfaces {
        pub use rspice_app_workbench::workbench::surfaces::*;

        pub mod automation;
        pub mod netlist;
        pub mod veriloga;
    }
    pub mod workflows {
        pub(crate) use rspice_app_services::workbench::workflows::export_workflow;

        pub mod netlist_workflow;
    }
}
