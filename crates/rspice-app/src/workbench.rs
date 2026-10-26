//! Contract-driven, responsive RSpice application workbench.
//!
//! This is the sole owner of application chrome and top-level task layout.
//! Document engines such as the schematic canvas and precision result viewers
//! render inside its central surface but do not own navigation or chrome.
//!
//! # What the shell is made of
//!
//! The declarations below are one sorted list. They were previously three
//! blocks accreted by the `common` merge and the `panels` fold, with a thread
//! helper wedged between the first two and 470 lines of frame rendering after
//! the last — so the module root was the least useful place to look for the
//! shell's shape. Rendering now lives in [`frame`], platform branching in
//! [`platform`].
//!
//! Five kinds of view container appear here, and the distinction is load
//! bearing:
//!
//! - `chrome` — the frame around everything: title bar, activity rail,
//!   document bar, toolbar, status bar. Always present, never a document.
//! - `docks` — context panels that flank the central surface (navigator,
//!   inspector, console, drawers). Dismissible; they never own a document.
//! - `surfaces` — the canonical task owners that fill the central area. One
//!   is active at a time, and it is what the current route names.
//! - `tools` — application-modal tools that float above the active surface
//!   (project launcher, jobs list, specialist-tool browser, calculator,
//!   notification centre). They own neither a route nor a document: they
//!   open over whatever is beneath, do one job, and close.
//! - `app/dialogs` — modal dialogs. They block the surface beneath until
//!   dismissed, which is what separates them from a `tools` float.
//!
//! Its interior layering is ordered and ratcheted by
//! `tests/module_layering.rs`, not by this file.
pub(crate) mod account_organization;
pub(crate) mod app;
pub(crate) mod app_state;
pub(crate) mod browser {
    //! What the browser build has to do differently.
    //!
    //! Navigation, file import, and download are the three places where the web
    //! platform refuses to behave like the desktop: history is a shared mutable
    //! stack, a file picker is asynchronous and user-gestured, and saving is a
    //! synthesized anchor click. Each is `#[cfg]`-gated to wasm32 except the
    //! import token, which the tests exercise on both. A tab also has no taskbar
    //! to mark when a run ends out of sight, so it asks to notify instead; the
    //! desktop's answer to that is always "unavailable".

    #[cfg(target_arch = "wasm32")]
    pub(crate) use rspice_app_services::workbench::browser::accessibility;
    pub(crate) use rspice_app_services::workbench::browser::background_alerts;
    #[cfg(target_arch = "wasm32")]
    pub(crate) use rspice_app_services::workbench::browser::download;
    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) use rspice_app_services::workbench::browser::file_import;
    #[cfg(all(target_arch = "wasm32", feature = "browser-qualification"))]
    pub(crate) use rspice_app_services::workbench::browser::qualification;
    #[cfg(target_arch = "wasm32")]
    pub(crate) use rspice_app_workbench::workbench::browser::navigation;
}
pub(crate) mod chrome {
    pub use rspice_app_workbench::workbench::chrome::*;
    pub(in crate::workbench) mod presence;
    pub(crate) mod sheet_strip;
    pub(crate) mod activity_rail {
        pub use rspice_app_workbench::workbench::chrome::activity_rail::*;
        pub(in crate::workbench) mod results;
    }
    pub(crate) mod document_bar {
        pub use rspice_app_workbench::workbench::chrome::document_bar::*;
        pub(crate) mod code;
        pub(crate) mod design;
        pub(in crate::workbench) mod models;
        pub(in crate::workbench) mod project;
        pub(in crate::workbench) mod results;
        pub(in crate::workbench) mod simulate;
    }
    pub(crate) mod status_bar {
        pub use rspice_app_workbench::workbench::chrome::status_bar::*;
        pub(crate) mod design;
        pub(crate) mod results;
        pub(crate) mod sheets;
        pub(in crate::workbench) mod shell;
    }
    pub(crate) mod title_bar {
        pub use rspice_app_workbench::workbench::chrome::title_bar::*;
        pub(in crate::workbench) mod code;
        pub(in crate::workbench) mod design;
    }
    pub(crate) mod toolbar {
        pub use rspice_app_workbench::workbench::chrome::toolbar::*;
        pub(in crate::workbench) mod code;
        pub(crate) mod design;
        pub(in crate::workbench) mod models;
        pub(in crate::workbench) mod project;
        pub(crate) mod results;
        pub(in crate::workbench) mod run_config_chip;
        pub(crate) mod simulate;
    }
}
pub(crate) mod cloud;
pub(crate) mod commands;
mod cross_probe;
pub(crate) mod diagnostic_routes;
pub(crate) use rspice_ui_kit::panels as design_system;
mod dispatch;
pub(crate) mod docks {
    pub(crate) use inspector::run_details::{RunDetailsDialogState, show as show_run_details};
    pub(crate) use navigator::stimulus::browser_state::StimulusBrowserState;
    pub use rspice_app_workbench::workbench::docks::*;
    pub(crate) mod inspector {
        pub use rspice_app_workbench::workbench::docks::inspector::*;
        pub(in crate::workbench) mod code_workspace;
        pub(crate) mod design;
        mod executed_deck;
        pub(in crate::workbench) mod models;
        pub(in crate::workbench) mod project;
        mod result_authority;
        pub(crate) mod results;
        /// Visible to the dock layer: the data browser opens it from a run's row, and
        /// the frame draws it whichever dock is showing.
        pub(crate) mod run_details;
        pub(in crate::workbench) mod simulation;
        /// Visible to the dock layer so the library browser's own gate can render this
        /// pane beside it: the two docks state the same counts about the same design,
        /// and a review that only ever opened one of them would not see them disagree.
        pub(in crate::workbench) mod stimulus;
        pub(in crate::workbench) mod symbol;
        pub(in crate::workbench) mod verify;
        pub(in crate::workbench) mod veriloga;
        /// The SOA verdict the Verify flow states is the one an instance's Checks row
        /// states, read the same way.
        pub(super) use design::analysis_soa_display;
    }
    pub(super) mod navigator {
        pub use rspice_app_workbench::workbench::docks::navigator::*;
        pub(in crate::workbench) mod design;
        pub(in crate::workbench) mod models;
        pub(in crate::workbench) mod netlist;
        pub(in crate::workbench) mod project;
        pub(in crate::workbench) mod results;
        mod results_actions;
        mod results_rows;
        pub(in crate::workbench) mod simulate;
        pub(in crate::workbench) mod source_bundle;
        pub(in crate::workbench) mod stimulus;
        pub(in crate::workbench) mod symbol;
        pub(in crate::workbench) mod verify;
        pub(in crate::workbench) mod veriloga;
        use netlist::*;
        mod verification_footer;
        use verification_footer::*;
    }
}
pub(crate) mod documents {
    pub use rspice_app_workbench::workbench::documents::*;
    pub(crate) mod code_workspace;
    pub(crate) mod model_correlation;
    pub(crate) mod model_editor;
    pub(crate) mod netlist_document;
    pub(crate) mod result_document;
    pub(crate) mod text_document_model;
    pub(crate) mod text_editor_commands;
    pub(crate) mod virtual_text_editor;
}
// The bundled circuits Help's Examples dialog opens as projects are model
// logic in `rspice_app_state::workbench::examples`. The test fixtures beside
// them build through the application's own APIs, so they stay here.
#[cfg(test)]
pub(crate) mod examples {
    /// The one hierarchical project the hierarchy suites build from. It is a
    /// fixture like the flat example circuits, but a whole project rather
    /// than a schematic, so it is authored through the application's own APIs
    /// instead of being drawn.
    pub(crate) mod hierarchy_reference;

    mod netlist_integration_tests;
}
mod features;
pub(crate) mod frame;
pub(crate) use rspice_app_workbench::workbench::framework;
pub(crate) mod hardcopy;
pub(crate) mod hardcopy_adapters;
pub(crate) use rspice_app_workbench::workbench::layout;
pub(crate) mod lifecycle {
    //! Opening, checkpointing, and recovering a working session.
    //!
    //! `project_lifecycle` owns the transactional document registry and the
    //! accepted baseline; `project_checkpoint` and `recovery_checkpoint` write
    //! the crash-recovery record; `recovery` reads it back. `session` and
    //! `window_session` are the runtime state a restart has to reconstruct — open
    //! views, selection, symbol editor, window bounds — none of which the project
    //! file itself persists.

    pub(crate) use rspice_app_services::workbench::lifecycle::project_checkpoint;
    pub(crate) use rspice_app_services::workbench::lifecycle::project_lifecycle;
    pub(crate) use rspice_app_services::workbench::lifecycle::recovery;
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) use rspice_app_services::workbench::lifecycle::recovery_checkpoint;
    pub(crate) mod session;
    pub(crate) use rspice_app_workbench::workbench::lifecycle::window_session;
}
pub(crate) mod live_session;
pub(crate) use rspice_app_workbench::workbench::localization;
pub(crate) mod logging;
pub(crate) mod menu_bar;
pub(crate) use rspice_app_runs::workbench::preflight;
pub(crate) use rspice_app_state::workbench::preferences;
pub(crate) mod results;
pub(crate) use rspice_app_workbench::workbench::routing;
/// The shell's transactions over the whole project, run where a feature's
/// work asks for one.
mod session {
    pub(crate) mod project_transactions;
}
pub(crate) mod settings;
pub(crate) mod sheets;
mod shell_host;
pub(crate) mod shortcuts;
pub(crate) mod slots;
pub(crate) mod state;
pub(crate) mod surfaces {
    pub use rspice_app_workbench::workbench::surfaces::*;
    pub(super) mod automation;
    pub(super) mod design;
    pub(super) mod library_cellview_manager;
    pub(super) mod model_correlation;
    mod model_correlation_controller;
    pub(super) mod model_editor;
    pub(super) mod models;
    pub(super) mod netlist;
    pub(super) mod pdk_technology_admin;
    pub(super) mod project;
    pub(super) mod results;
    pub(super) mod simulate;
    pub(super) mod stimulus;
    pub(super) mod verify;
    mod veriloga;
    /// The simulation plan workflows are opened from global chrome, so the frame
    /// hosts them rather than the Simulate surface. See
    /// [`simulate::show_workflow_dialogs`].
    pub(in crate::workbench) use simulate::show_workflow_dialogs as show_simulation_workflow_dialogs;
}
pub(crate) mod tools;
pub(crate) mod user_identity;
pub(crate) mod workflows;

pub(crate) use cross_probe::synchronize_schematic_cross_probe;
pub use documents::result_document::ResultViewer;
pub use lifecycle::session::{SymbolGridSpacing, SymbolTool};
pub use lifecycle::window_session::ApplicationWindowId;
pub use localization::UiTextLocale;
pub(crate) use localization::{MessageCatalog, MessageId};
pub(crate) use preferences::DrawingSheetPersonalPreferences;
pub use preferences::{
    BackgroundTaskAttention, ChoicePreference, ComplexNumberDisplay, ConsoleLaunchBehavior,
    EngineeringExportFormat, FolderListPreference, ScalarPreference, TogglePreference,
    UserPreferences, WorkspacePreferences, WorkspacePreset,
};
#[cfg(any(test, target_arch = "wasm32"))]
pub use routing::navigation::BrowserHistoryEffect;
pub use routing::navigation::{RouteTransitionSource, SurfaceNavigation};
pub use routing::surface_catalog::{SurfaceArchetype, SurfaceId};
pub use routing::surface_route::SurfaceRoute;
pub use rspice_design::symbol::edit::{SymbolCommitIntent, SymbolDocumentSnapshot};
pub use state::WorkbenchSession;

pub use app::RSpiceApp;
pub use app_state::AppState;
pub(crate) use frame::{show, show_embedded_secondary, show_secondary};
