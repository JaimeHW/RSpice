//! The application's sheets feature.
//!
//! The drawing sheet a schematic is drawn on: its properties, formats and
//! sizes, the sheets of one drawing and the strip that steps between them,
//! the inspector's Sheet section, and the dialogs for drawing-sheet defaults,
//! custom sizes, the format manager, the overflow review and sheet deletion.
//! It works on the shared model in `rspice-app-state` and registers with the
//! workbench framework of `rspice-app-workbench`, and it reaches no other
//! feature: what it reads of design it reads through the framework's ports.
//!
//! `rspice-app` re-exports each module at the path it has always had, so the
//! modules keep the layout they had inside it.

#![cfg_attr(target_arch = "wasm32", allow(dead_code))]
#![allow(clippy::too_many_arguments)]
#![deny(clippy::print_stdout, clippy::print_stderr)]

// The model's, the services' and the framework's modules, at the paths the
// sheets feature has always named them by.
use rspice_app_services::io;
use rspice_app_state::{diagnostics, state};
use rspice_app_workbench::ui;

pub mod workbench {
    //! The sheets feature's modules, at their application paths.

    pub(crate) use rspice_app_state::workbench::preferences::DrawingSheetPersonalPreferences;
    pub(crate) use rspice_app_workbench::workbench::{design_system, framework, layout, state};

    pub mod app {
        pub(crate) use actions::sheets;
        pub(crate) use dialogs::drawing_sheet_defaults::{
            DrawingSheetDefaultsDialogState, open_drawing_sheet_defaults,
        };
        pub(crate) use dialogs::drawing_sheet_overflow::open_drawing_sheet_overflow_review;
        pub(crate) use dialogs::drawing_sheet_presets::{
            DrawingSheetPresetDialogsState, open_custom_sheet_size_library,
        };
        pub(crate) use dialogs::drawing_sheet_support::{
            DrawingSheetSupportState, open_sheet_format_manager,
        };
        pub(crate) use rspice_app_state::workbench::app::schematic::edit_authority::SchematicEditAuthority;

        pub mod actions {
            pub mod sheets;
        }
        pub mod dialogs {
            pub mod delete_sheet;
            pub mod drawing_sheet_defaults;
            pub mod drawing_sheet_overflow;
            pub mod drawing_sheet_presets;
            pub mod drawing_sheet_preview;
            pub mod drawing_sheet_support;
        }
        pub mod drawing_sheet_edit;
    }
    pub(crate) mod app_state {
        pub(crate) use rspice_app_state::workbench::app_state::design_history::DesignManagementHistoryEntry;
        pub(crate) use rspice_app_state::workbench::app_state::model::Model;
        pub(crate) use rspice_app_state::workbench::app_state::schematic_preferences;
        pub(crate) mod session {
            pub(crate) use rspice_app_workbench::workbench::app_state::session::shortcuts;
        }
    }
    pub mod chrome {
        pub mod sheet_strip;
        pub mod status_bar {
            pub mod sheets;
        }
    }
    pub mod docks {
        pub mod inspector {
            pub(crate) use rspice_app_workbench::workbench::docks::inspector::shared;
            pub mod design {
                pub mod sheet;
            }
        }
    }
    pub mod sheets;
    pub mod slots {
        pub(crate) use sheets::SheetsState;
        pub mod sheets;
    }
}
