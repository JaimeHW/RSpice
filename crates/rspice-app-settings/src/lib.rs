//! The application's settings feature.
//!
//! Preferences and the windows it opens, the preferences file, the shortcut
//! library in use and its device-local store, keyboard shortcut profiles and
//! their portable artifacts, the settings commands, and the feature's state
//! slot and registration. It works on the shared model in `rspice-app-state`,
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

// The model's, the services' and the framework's modules, at the paths the
// settings feature has always named them by.
use rspice_app_services::{io, services};
use rspice_app_state::{diagnostics, state, time_compat};
use rspice_app_workbench::ui;

pub mod workbench {
    //! The settings feature's modules, at their application paths.

    pub(crate) use rspice_app_state::workbench::preferences;
    pub(crate) use rspice_app_state::workbench::preferences::{
        BackgroundTaskAttention, ChoicePreference, ConsoleLaunchBehavior, FolderListPreference,
        ScalarPreference, TogglePreference, UserPreferences, WorkspacePreferences, WorkspacePreset,
    };
    pub(crate) use rspice_app_workbench::workbench::routing::navigation::RouteTransitionSource;
    pub(crate) use rspice_app_workbench::workbench::routing::surface_catalog::SurfaceId;
    pub(crate) use rspice_app_workbench::workbench::routing::surface_route::SurfaceRoute;
    pub(crate) use rspice_app_workbench::workbench::state::WorkbenchSession;
    pub(crate) use rspice_app_workbench::workbench::{design_system, framework, state};

    pub mod app {
        pub mod dialogs {
            pub(crate) use rspice_app_workbench::workbench::app::dialogs::review_primitives;

            pub mod preferences;
        }
    }
    pub mod app_state {
        pub(crate) use rspice_app_services::workbench::slots::platform::PlatformState;
        pub(crate) use rspice_app_state::workbench::app_state::model::Model;
        pub(crate) use rspice_app_state::workbench::app_state::schematic_preferences;
        pub(crate) use session::shortcuts::runtime_command_platform;

        pub mod session {
            pub(crate) use rspice_app_workbench::workbench::app_state::session::shortcuts;

            pub mod preferences_file;
            pub mod shortcut_library;
        }
    }
    pub(crate) mod browser {
        #[cfg(target_arch = "wasm32")]
        pub(crate) use rspice_app_services::workbench::browser::accessibility;
        #[cfg(target_arch = "wasm32")]
        pub(crate) use rspice_app_services::workbench::browser::download;
        #[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
        pub(crate) use rspice_app_services::workbench::browser::file_import;
    }
    // Showing a folder in the file manager, which only the desktop build does
    // for real.
    #[cfg(all(
        not(any(test, feature = "test-support")),
        not(target_arch = "wasm32"),
        any(target_os = "windows", target_os = "macos", target_os = "linux")
    ))]
    pub(crate) mod lifecycle {
        pub(crate) use rspice_app_services::workbench::lifecycle::recovery;
    }
    pub mod settings;
    pub mod shortcuts;
    pub mod slots {
        pub(crate) use settings::SettingsState;

        pub mod settings;
    }
}
