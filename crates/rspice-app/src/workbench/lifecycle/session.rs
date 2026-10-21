//! Session state shared by document engines.
//!
//! Top-level navigation, responsive layout, docks, and drawers belong to
//! [`crate::workbench::WorkbenchState`]. This module contains only cross-frame document
//! interaction state and durable presentation preferences.

use rspice_design::symbol::edit::SymbolHistory;
use serde::{Deserialize, Serialize};

use crate::state::{GridStyle, SchematicVisibilityPolicy};
use crate::ui::Theme;
use crate::ui::widgets::Toasts;

/// Object family searched by the schematic bulk-edit workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelectionBulkObjectKind {
    #[default]
    InstancesAndParameters,
    NetsAndLabels,
    PortsAndPins,
    GraphicsAndNotes,
}

impl SelectionBulkObjectKind {
    pub const ALL: [Self; 4] = [
        Self::InstancesAndParameters,
        Self::NetsAndLabels,
        Self::PortsAndPins,
        Self::GraphicsAndNotes,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::InstancesAndParameters => "Instances + parameters",
            Self::NetsAndLabels => "Nets and labels",
            Self::PortsAndPins => "Ports and pins",
            Self::GraphicsAndNotes => "Graphics and notes",
        }
    }
}

/// Project extent inspected by the schematic bulk-edit workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelectionBulkHierarchyScope {
    #[default]
    ActiveHierarchyPath,
    CurrentSheet,
    CompleteProject,
}

impl SelectionBulkHierarchyScope {
    pub const ALL: [Self; 3] = [
        Self::ActiveHierarchyPath,
        Self::CurrentSheet,
        Self::CompleteProject,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::ActiveHierarchyPath => "Active hierarchy path",
            Self::CurrentSheet => "Current sheet",
            Self::CompleteProject => "Complete project",
        }
    }
}

/// Device-local, reusable query used by the mockup-owned bulk editor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SelectionBulkFilter {
    pub query: String,
    pub object_kind: SelectionBulkObjectKind,
    pub hierarchy_scope: SelectionBulkHierarchyScope,
    pub model_cell: String,
    pub current_property: String,
}

impl Default for SelectionBulkFilter {
    fn default() -> Self {
        Self {
            query: String::new(),
            object_kind: SelectionBulkObjectKind::InstancesAndParameters,
            hierarchy_scope: SelectionBulkHierarchyScope::ActiveHierarchyPath,
            model_cell: String::new(),
            current_property: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedSelectionBulkFilter {
    pub name: String,
    pub filter: SelectionBulkFilter,
}

/// Persisted active query and named filters. This is user/session state, never
/// project-owned design data.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SelectionBulkFilterSession {
    pub active: SelectionBulkFilter,
    saved: Vec<SavedSelectionBulkFilter>,
}

impl SelectionBulkFilterSession {
    const MAX_SAVED_FILTERS: usize = 32;
    const MAX_FILTER_NAME_BYTES: usize = 96;

    pub fn saved(&self) -> &[SavedSelectionBulkFilter] {
        &self.saved
    }

    pub fn save_active(&mut self, name: &str) -> Result<(), String> {
        let name = name.trim();
        if name.is_empty() {
            return Err("A saved filter name is required.".to_owned());
        }
        if name.len() > Self::MAX_FILTER_NAME_BYTES || name.chars().any(char::is_control) {
            return Err("The saved filter name is invalid or too long.".to_owned());
        }
        if let Some(existing) = self
            .saved
            .iter_mut()
            .find(|entry| entry.name.eq_ignore_ascii_case(name))
        {
            existing.name = name.to_owned();
            existing.filter = self.active.clone();
            return Ok(());
        }
        if self.saved.len() >= Self::MAX_SAVED_FILTERS {
            return Err(format!(
                "At most {} selection filters may be saved.",
                Self::MAX_SAVED_FILTERS
            ));
        }
        self.saved.push(SavedSelectionBulkFilter {
            name: name.to_owned(),
            filter: self.active.clone(),
        });
        self.saved
            .sort_by(|left, right| left.name.to_lowercase().cmp(&right.name.to_lowercase()));
        Ok(())
    }

    pub fn load(&mut self, name: &str) -> bool {
        let Some(filter) = self
            .saved
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case(name))
            .map(|entry| entry.filter.clone())
        else {
            return false;
        };
        self.active = filter;
        true
    }

    pub fn remove(&mut self, name: &str) -> bool {
        let before = self.saved.len();
        self.saved
            .retain(|entry| !entry.name.eq_ignore_ascii_case(name));
        self.saved.len() != before
    }
}

pub use rspice_schematic_editor::symbol_editor::session::{
    SymbolEditorSession, SymbolGridSpacing, SymbolTool,
};

#[derive(Debug, Clone, Default)]
pub struct SymbolUiState {
    pub editor: SymbolEditorSession,
    pub history: SymbolHistory,
    /// The symbol being edited in place on a parent schematic, if any.
    pub in_place: Option<Box<SymbolInPlace>>,
}

/// A symbol edited in place on the schematic that places one of its
/// instances.
///
/// The symbol view is the open document, so its tools, inspector, undo and
/// save are the symbol editor's own. This records what the canvas needs to
/// draw the symbol where the instance stands, over its parent, and what it
/// takes to hand the author back.
#[derive(Debug, Clone)]
pub struct SymbolInPlace {
    /// The symbol view every edit goes to.
    pub symbol: crate::state::CellViewRef,
    /// The schematic the instance stands on.
    pub parent: crate::state::CellViewRef,
    pub instance_id: u64,
    pub instance_name: String,
    /// The symbol origin when the session began. The instance is drawn from
    /// it for the whole session, so dragging the origin cannot move the frame
    /// the pointer is read in.
    pub origin: crate::state::Point,
    /// The parent canvas's pan and zoom: carried in, and handed back on exit.
    pub pan: (f64, f64),
    pub zoom: f64,
    /// A zoom a command asked for, carried out on the canvas's next frame.
    pub zoom_request: Option<rspice_schematic_editor::view::navigation::ZoomRequest>,
    /// Whether the symbol had a tab of its own before the session opened it.
    pub symbol_was_open: bool,
    /// The symbol as the session began, or as last saved in it. Leaving with
    /// anything else asks first.
    pub baseline: (
        crate::state::SymbolDocument,
        crate::state::SymbolEditorMetadata,
    ),
    pub baseline_depth: usize,
    /// The question asked on leaving with unsaved edits, while it is open.
    pub leaving: Option<SymbolInPlaceExit>,
    /// Where to land once the save dialog publishes the symbol.
    pub exit_after_save: Option<SymbolInPlaceExit>,
    /// The parent is drawn with nothing selected or hovered: it is context
    /// for the edit, not a drawing being edited.
    pub parent_editor: rspice_schematic_editor::session::EditorSession,
}

/// Where leaving an in-place symbol edit lands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolInPlaceExit {
    /// The parent schematic, as the session found it.
    Parent,
    /// One level of the parent's occurrence, chosen on the breadcrumb.
    Level(usize),
}

/// Transient, device-local recovery point for a visibility-policy change.
#[derive(Debug, Clone)]
pub(crate) struct SchematicVisibilityRecovery {
    pub(crate) view_path: String,
    pub(crate) policy: SchematicVisibilityPolicy,
    pub(crate) routing_mode: crate::state::WireRoutingMode,
    pub(crate) net_highlight: crate::state::NetHighlightState,
    pub(crate) selection: crate::state::Selection,
}

/// Persistent and per-frame state shared by workbench document engines.
#[derive(Debug, Clone, Default)]
pub struct UiSessionState {
    /// Theme selection (direction / mode / density).
    pub theme: Theme,
    /// Autosave checkpoint interval in minutes; 0 = off. Checkpoints write
    /// next to the project file and never touch it.
    pub autosave_minutes: u8,
    /// Canonical user/device preference overrides not already owned by the
    /// theme or autosave runtime.
    pub preferences: crate::workbench::preferences::UserPreferences,
    /// Decimal-mark information supplied by the native/web platform edge.
    /// `None` is deliberate: engineering code must not guess the host locale.
    pub number_locale: crate::quantity::UiNumberLocale,
    /// Device-local UI message locale. Engineering source, identifiers, and
    /// portable numeric syntax never consult this presentation setting.
    pub text_locale: crate::workbench::UiTextLocale,
    /// Browser-only opt-in for egui's Web Speech interaction feedback.
    /// Native assistive technology is negotiated automatically by AccessKit.
    pub browser_spoken_feedback: bool,
    /// Toast queue.
    pub toasts: Toasts,
    /// The last autosave checkpoint could not be written. A failure is said
    /// when it starts, not again at every interval it persists. Transient.
    pub(crate) autosave_failing: bool,
    /// What the last save of the session kept. Like `autosave_failing`, it
    /// makes a refusal said when it starts, not at every save. Transient.
    pub(crate) session_kept: crate::services::session::SessionKept,
    /// A retained result document asked to be taken back to the plan that
    /// produced it.
    ///
    /// Result documents are read-only projections and do not dispatch; the
    /// frame drains this the way it already drains the export requests they
    /// raise. Transient.
    pub(crate) open_producing_plan_requested: bool,
    /// The runs seen running and the campaign outcomes not yet announced.
    ///
    /// The shell announces a completion on the *transition* into a terminal
    /// lifecycle rather than on the terminal state itself, so opening a
    /// project whose history already holds a finished run never claims a run
    /// just completed.
    pub(crate) run_watch: crate::simulation::controller::prepared_run::RunWatch,
    /// Schematic-space cursor position, reported by the canvas each frame the
    /// pointer hovers it; consumed by the status bar.
    pub canvas_hover: Option<(f64, f64)>,
    /// Schematic-space center of the visible canvas, reported each frame the
    /// canvas renders; paste targets fall back to it when the cursor is not
    /// over the canvas (menu-driven paste).
    pub canvas_view_center: Option<(f64, f64)>,
    /// `simulation.data_version` at the last time the Results view was shown;
    /// drives the "new results" badge on the Results tab.
    pub results_seen_version: u64,
    /// Canvas grid style (dots / lines / off).
    pub grid: GridStyle,
    /// Last non-Off display style retained for backward-compatible session
    /// restoration. The current `G` command cycles dots, lines, and off.
    pub grid_restore_style: GridStyle,
    /// Schematic editor selection classes. This is restored with the UI
    /// session but never written into project-owned schematic data.
    pub schematic_selection_filter: crate::state::SchematicSelectionFilter,
    /// What Copy and Cut last took from a schematic. It belongs to the app,
    /// not to one cell view's editor, so a copy made in one cell view pastes
    /// into another. Never persisted: a session restored later must not
    /// reopen with design objects nobody committed to a drawing.
    pub schematic_clipboard: rspice_design::schematic::clipboard::ClipboardData,
    /// Active and named filters for exact schematic bulk-edit queries.
    pub schematic_bulk_edit_filters: SelectionBulkFilterSession,
    /// Device-local schematic display policy. Presentation choices are
    /// restored with the session and are never written to project data.
    pub schematic_visibility: SchematicVisibilityPolicy,
    /// Optional drawing-sheet construction detail shown around the permanent
    /// authored paper boundary. Output inclusion remains independently owned
    /// by Print and Export.
    pub drawing_sheet_layers: crate::state::DrawingSheetLayerVisibility,
    /// Exact runtime snap-target configuration used by the Grid & snap
    /// popover. The document grid pitch remains project-owned.
    pub schematic_snap: crate::state::SnapEngine,
    /// Exact runtime conductor routing mode. This retains the horizontal- or
    /// vertical-first orthogonal preference that the display policy's
    /// user-facing three-way style intentionally abstracts.
    pub schematic_routing_mode: crate::state::WireRoutingMode,
    /// What Duplicate does with a terminal whose net keeps existing outside
    /// the copied set. Restored with the UI session; never project data.
    pub duplicate_external_nets: crate::state::DuplicateExternalNets,
    /// Last visibility-policy presentation snapshot for the active view.
    pub(crate) schematic_visibility_recovery: Option<SchematicVisibilityRecovery>,
    /// Device-local edge-triggered schematic selection synchronization.
    /// This is deliberately excluded from the session wire: source-map
    /// and retained-result highlights are presentation state, not project
    /// data or a durable preference value.
    pub(crate) schematic_cross_probe: crate::workbench::cross_probe::SchematicCrossProbeRuntime,
    /// Device-local working and named engineering-table views. Project-owned
    /// named views live in `ProjectWorkspace`; this store is the personal
    /// preference authority restored on desktop, web, and tablet hosts.
    pub engineering_table_views: crate::state::engineering_table::EngineeringTableViewStore,
    /// One-shot request to export the visible waveforms as CSV (needs the
    /// app's IO backend, so it is handled at the workbench boundary).
    pub export_csv_requested: bool,
    /// One-shot exact export of one dataset-bound typed Data Browser entity.
    /// The key, rather than a copied payload, crosses the UI/IO boundary so
    /// the export adapter must re-resolve immutable evidence at publication.
    pub(crate) export_result_quantities_requested:
        Option<Vec<rspice_results_ui::selection::ResultBrowserSelectionKey>>,
    /// One-shot request to export the active view as a publication figure.
    ///
    /// This used to capture the window and crop it, which handed the reader
    /// back the pixels they already had at whatever resolution the display
    /// happened to be. It opens the hardcopy export workflow instead — the
    /// one pipeline that renders a sheet as PDF/A, PDF, SVG or PNG at a
    /// chosen resolution, and the same one Print already used.
    pub export_figure_requested: bool,
    /// One-shot platform clipboard payload emitted by commands that copy
    /// engineering readouts. Kept transient so clipboard intent is never
    /// serialized into a project/session.
    pub clipboard_text_request: Option<String>,
    /// The source editor the page on screen shows as its document, published
    /// by that page every frame it draws one. The Edit menu, the palette and
    /// the shortcuts act on it rather than on a schematic hidden behind it.
    pub(crate) active_source_editor:
        Option<crate::workbench::documents::text_editor_commands::ActiveSourceEditor>,
    /// One-shot request for [`Self::active_source_editor`], handed to the
    /// editor by the page that draws it.
    pub(crate) source_editor_request:
        Option<crate::workbench::documents::text_editor_commands::SourceEditorRequest>,
    /// One-shot request to change platform fullscreen state. Keeping this
    /// transient prevents the web backend from receiving the same viewport
    /// command every frame and preserves the user's persisted preference in
    /// [`crate::workbench::WorkbenchState`].
    pub(crate) full_screen_request: Option<bool>,
    /// Results workspace state (viewer, cursors, plot caches).
    pub results: crate::workbench::documents::result_document::ResultsState,
    /// Netlist editor state (diagnostics, diff pips, tuner mode).
    pub netlist: crate::workbench::documents::netlist_document::NetlistDocumentState,
    /// Visible Code & Automation page and revision-bound operation receipts.
    pub code_workspace: crate::workbench::documents::code_workspace::CodeWorkspaceRuntimeState,
    /// Runtime state for the Schematic-family symbol editor surface.
    pub symbol: SymbolUiState,
}

impl UiSessionState {
    /// New document session with safe autosave defaults.
    pub fn new() -> Self {
        Self {
            // Crash recovery is not opt-in; Off stays available.
            autosave_minutes: 5,
            ..Self::default()
        }
    }

    /// Replace the platform-provided locale used by interactive UI quantity
    /// fields. Portable project and SPICE parsers never read this value.
    pub fn set_number_locale(&mut self, locale: crate::quantity::UiNumberLocale) {
        self.number_locale = locale;
    }

    #[must_use]
    pub(crate) const fn messages(&self) -> crate::workbench::MessageCatalog {
        crate::workbench::MessageCatalog::new(self.text_locale)
    }

    /// The reader's result presentation, with the significant digits of the
    /// project result document the Results workspace is showing, if any.
    #[must_use]
    pub(crate) fn result_presentation_policy(
        &self,
    ) -> crate::workbench::preferences::ResultPresentationPolicy {
        let policy = self.preferences.result_presentation_policy();
        self.results
            .session
            .document_presentation
            .map_or(policy, |document| {
                policy.with_significant_digits(document.significant_digits)
            })
    }

    pub(crate) fn set_grid_style(&mut self, style: GridStyle) {
        if style.visible() {
            self.grid_restore_style = style;
        }
        self.grid = style;
    }

    /// Cycle the upgraded toolbar's exact presentation order.
    pub(crate) fn cycle_grid_style(&mut self) -> GridStyle {
        let next = match self.grid {
            GridStyle::Dots => GridStyle::Lines,
            GridStyle::Lines => GridStyle::Off,
            GridStyle::Off => GridStyle::Dots,
        };
        self.set_grid_style(next);
        next
    }

    pub(crate) fn request_full_screen(&mut self, enabled: bool) {
        self.full_screen_request = Some(enabled);
    }

    pub(crate) fn take_full_screen_request(&mut self) -> Option<bool> {
        self.full_screen_request.take()
    }
}

/// Map any stored interval onto the 2, 5 and 10 minutes Preferences offers.
pub(crate) fn normalize_autosave_minutes(minutes: u8) -> u8 {
    match minutes {
        2 | 5 | 10 => minutes,
        0..=3 => 2,
        4..=7 => 5,
        _ => 10,
    }
}

#[cfg(test)]
mod symbol_selection_tests {
    use super::*;

    #[test]
    fn named_bulk_filters_update_load_and_delete_case_insensitively() {
        let mut filters = SelectionBulkFilterSession::default();
        filters.active.query = "R*".to_owned();
        filters.save_active("Passives").expect("save");
        filters.active.query = "C*".to_owned();
        filters.save_active("passives").expect("replace");
        assert_eq!(filters.saved().len(), 1);
        assert!(filters.load("PASSIVES"));
        assert_eq!(filters.active.query, "C*");
        assert!(filters.remove("pAsSiVeS"));
        assert!(filters.saved().is_empty());
    }
}

#[cfg(test)]
mod platform_request_tests {
    use crate::workbench::UiSessionState;

    #[test]
    fn full_screen_request_is_consumed_exactly_once() {
        let mut session = UiSessionState::new();
        session.request_full_screen(true);

        assert_eq!(session.take_full_screen_request(), Some(true));
        assert_eq!(session.take_full_screen_request(), None);
    }

    #[test]
    fn newest_full_screen_request_supersedes_an_unapplied_request() {
        let mut session = UiSessionState::new();
        session.request_full_screen(true);
        session.request_full_screen(false);

        assert_eq!(session.take_full_screen_request(), Some(false));
    }
}
