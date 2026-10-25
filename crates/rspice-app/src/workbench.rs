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
    pub(crate) mod navigation;
    #[cfg(all(target_arch = "wasm32", feature = "browser-qualification"))]
    pub(crate) use rspice_app_services::workbench::browser::qualification;
}
pub(crate) mod chrome;
pub(crate) mod commands;
mod cross_probe;
pub(crate) mod diagnostic_routes;
pub(crate) use rspice_ui_kit::panels as design_system;
mod dispatch;
pub(crate) mod docks;
pub(crate) mod documents;
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
pub(crate) mod framework;
pub(crate) mod hardcopy_adapters;
pub(crate) mod layout;
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
    pub(crate) mod window_session;
}
pub(crate) mod live_session;
pub(crate) mod localization;
pub(crate) mod logging;
pub(crate) mod menu_bar;
pub(crate) use rspice_app_services::workbench::platform;
pub(crate) use rspice_app_state::workbench::preferences;
pub(crate) mod preflight;
pub(crate) mod publication_snapshot;
pub(crate) mod routing;
pub(crate) mod settings;
pub(crate) mod sheets;
mod shell_host;
pub(crate) mod shortcuts;
pub(crate) mod slots;
pub(crate) mod state;
pub(crate) mod surfaces;
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
