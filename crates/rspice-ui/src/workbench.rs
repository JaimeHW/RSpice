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
pub(crate) mod browser;
pub(crate) mod chrome;
pub(crate) mod commands;
mod cross_probe;
pub(crate) mod design_system;
mod docks;
pub(crate) mod documents;
// The bundled circuits are fixtures: every consumer is a test. The
// application builds schematics from projects and from the library.
#[cfg(test)]
pub(crate) mod examples;
mod feature_availability;
pub(crate) mod feature_availability_data;
pub(crate) mod frame;
pub(crate) mod hardcopy_adapters;
mod layout;
pub(crate) mod lifecycle;
pub(crate) mod live_session;
pub(crate) mod localization;
pub(crate) mod logging;
pub(crate) mod menu_bar;
pub(crate) mod platform;
mod preferences;
mod preflight;
pub(crate) mod publication_snapshot;
pub(crate) mod routing;
pub(crate) mod shortcuts;
pub(crate) mod simulation_analysis_tabs;
pub(crate) mod state;
mod surfaces;
pub(crate) mod tools;
pub(crate) mod workflows;

pub use app::{
    DRAWING_SHEET_PACKAGE_MAX_BYTES, DrawingSheetPackageEncoding, DrawingSheetPackageInspection,
    DrawingSheetPackageVerification, PublishedDrawingSheetPackage,
    drawing_sheet_publisher_public_key, inspect_drawing_sheet_package,
    publish_organization_drawing_sheet_package, verify_published_drawing_sheet_package,
};
pub(crate) use cross_probe::synchronize_schematic_cross_probe;
pub use documents::result_document::ResultViewer;
pub(crate) use lifecycle::session::{SchematicSelectionRecovery, SchematicVisibilityRecovery};
pub use lifecycle::session::{
    SelectionBulkFilter, SelectionBulkHierarchyScope, SelectionBulkObjectKind, SymbolClipboard,
    SymbolDocumentSnapshot, SymbolGridSpacing, SymbolSelection, SymbolTool, UiSessionState,
    UiSessionStateSer, mirror_point_h_about, mirror_point_v_about, mirror_shape_h_about,
    mirror_shape_v_about, rotate_point_cw_about, rotate_shape_cw_about,
};
pub use lifecycle::window_session::ApplicationWindowId;
pub use localization::UiTextLocale;
pub(crate) use localization::{MessageCatalog, MessageId};
pub(crate) use preferences::DrawingSheetPersonalPreferences;
pub use preferences::{
    BackgroundTaskAttention, ChoicePreference, ComplexNumberDisplay, ConsoleLaunchBehavior,
    CursorInterpolation, EngineeringExportFormat, LargeDatasetDisplay, ResultPresentationPolicy,
    ScalarPreference, TogglePreference, UserPreferences, WorkspacePreferences, WorkspacePreset,
};
pub use routing::availability::{
    SurfaceExecutionAvailability, SurfaceRouteUnavailable, route_availability, surface_availability,
};
#[cfg(any(test, target_arch = "wasm32"))]
pub use routing::navigation::BrowserHistoryEffect;
pub use routing::navigation::{RouteTransition, RouteTransitionSource, SurfaceNavigation};
pub use routing::surface_catalog::{SurfaceArchetype, SurfaceId};
pub use routing::surface_route::SurfaceRoute;
pub use shortcuts::{
    ChordTimeoutPolicy, ContextPrecedencePolicy, ProtectedShortcutPolicy, ShortcutBindingSlot,
    ShortcutPreferences, ShortcutProfileAudit, ShortcutProfileIssue, ShortcutProfileIssueCode,
    ShortcutProfileIssueSeverity, ShortcutSequence, ShortcutStroke, SingleKeyCanvasPolicy,
    shortcut_context_precedence_rank,
};
pub use state::{EngineeringProfile, WorkbenchState};
pub use workflows::capability_workflow::CapabilityWorkflowId;

pub use app::RSpiceApp;
pub use app_state::AppState;
pub(crate) use frame::{
    enter_full_screen_presentation, exit_full_screen_presentation, show, show_embedded_secondary,
    show_route_overlays, show_secondary,
};
pub(crate) use platform::spawn_or_inline;
