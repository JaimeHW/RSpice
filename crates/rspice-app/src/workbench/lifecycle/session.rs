//! Session state shared by document engines.
//!
//! Top-level navigation, responsive layout, docks, and drawers belong to
//! [`crate::workbench::WorkbenchState`]. This module contains only cross-frame document
//! interaction state and durable presentation preferences.

use rspice_design::symbol::edit::SymbolHistory;
use serde::{Deserialize, Serialize};

use crate::state::{GridStyle, SchematicVisibilityPolicy, SchematicWireRoutingStyle};
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

/// What the shell has seen of runs and campaigns, so that each one's end is
/// announced exactly once.
///
/// Every run seen before it ended is watched, not just the newest: a
/// campaign starts its next member in the same update that finishes the
/// last, so by the time a frame looks the newest run has already moved on.
/// Transient by construction.
#[derive(Debug, Clone, Default)]
pub(crate) struct RunWatch {
    /// Runs seen before they ended, by stable identity. A run that was over
    /// when it was first seen, such as history opened with a project, is
    /// never in here, so it is never announced as having just finished.
    running: std::collections::HashSet<crate::product::RunId>,
    /// Campaign outcomes the controller reported that the shell has not
    /// announced yet, oldest first.
    campaign_outcomes: Vec<crate::simulation::controller::prepared_run::CampaignOutcome>,
}

impl RunWatch {
    /// Watch a run from the moment it opens, before any frame has seen it.
    /// A run that ends inside the update that started it is still one that
    /// was seen running.
    pub(crate) fn run_started(&mut self, run_id: crate::product::RunId) {
        self.running.insert(run_id);
    }

    /// Bring the watch up to date with the runs as they stand, newest first,
    /// and return the runs that ended since it last looked, oldest first.
    pub(crate) fn observe(
        &mut self,
        runs: impl IntoIterator<Item = (crate::product::RunId, crate::state::SimulationRunLifecycle)>,
    ) -> Vec<crate::product::RunId> {
        use crate::state::SimulationRunLifecycle as Lifecycle;

        let mut running = std::collections::HashSet::new();
        let mut ended = Vec::new();
        for (run_id, lifecycle) in runs {
            if matches!(
                lifecycle,
                Lifecycle::Preparing | Lifecycle::Running | Lifecycle::Cancelling
            ) {
                running.insert(run_id);
            } else if lifecycle.is_terminal() && self.running.contains(&run_id) {
                ended.push(run_id);
            }
        }
        // A run that left the history while running is not announced, and
        // is not kept waiting for an end it can no longer reach.
        self.running = running;
        ended.reverse();
        ended
    }

    /// Hold a campaign outcome until the shell announces it.
    pub(crate) fn campaign_outcome(
        &mut self,
        outcome: crate::simulation::controller::prepared_run::CampaignOutcome,
    ) {
        self.campaign_outcomes.push(outcome);
    }

    /// The campaign outcomes not yet announced, oldest first.
    pub(crate) fn take_campaign_outcomes(
        &mut self,
    ) -> Vec<crate::simulation::controller::prepared_run::CampaignOutcome> {
        std::mem::take(&mut self.campaign_outcomes)
    }
}

/// What the last save of the session kept for the next start. Transient.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) enum SessionKept {
    /// All of it, or nothing has been saved yet.
    #[default]
    Whole,
    /// All but the runs, which the store refused. Holds the run history they
    /// were refused for, so later saves do not encode the same runs again
    /// only to have them refused.
    WithoutRuns(crate::state::RunHistoryRevision),
    /// None of it: the store refused even the session without its runs.
    Nothing,
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
    /// The shell's egui context, kept so that a failure reported by a
    /// workflow that holds no frame (a save, an open) is still toasted on the
    /// frame clock. Set once when the shell starts. Transient.
    pub(crate) toast_context: egui::Context,
    /// The last autosave checkpoint could not be written. A failure is said
    /// when it starts, not again at every interval it persists. Transient.
    pub(crate) autosave_failing: bool,
    /// What the last save of the session kept. Like `autosave_failing`, it
    /// makes a refusal said when it starts, not at every save. Transient.
    pub(crate) session_kept: SessionKept,
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
    pub(crate) run_watch: RunWatch,
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
    /// This is deliberately excluded from `UiSessionStateSer`: source-map
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

/// Serialized subset of [`UiSessionState`]. Responsive layout is persisted by
/// [`crate::workbench::WorkbenchState`]; transient document interaction signals are not.
#[derive(Serialize, Deserialize)]
pub struct UiSessionStateSer {
    #[serde(default)]
    theme: Theme,
    /// Legacy on/off flag, still written so older builds keep their
    /// setting; `grid_style` wins when present.
    #[serde(default = "default_true")]
    show_grid: bool,
    #[serde(default)]
    grid_style: Option<GridStyle>,
    #[serde(default)]
    grid_restore_style: Option<GridStyle>,
    #[serde(default)]
    schematic_selection_filter: crate::state::SchematicSelectionFilter,
    #[serde(default)]
    schematic_bulk_edit_filters: SelectionBulkFilterSession,
    #[serde(default)]
    schematic_visibility: SchematicVisibilityPolicy,
    #[serde(default)]
    drawing_sheet_layers: crate::state::DrawingSheetLayerVisibility,
    #[serde(default)]
    schematic_snap: crate::state::SnapEngine,
    #[serde(default)]
    schematic_routing_mode: Option<crate::state::WireRoutingMode>,
    #[serde(default)]
    duplicate_external_nets: crate::state::DuplicateExternalNets,
    #[serde(default)]
    engineering_table_views: crate::state::engineering_table::EngineeringTableViewStore,
    // `theme` and the next three fields are still written here so an older
    // build finds them, but the session no longer owns them: the preferences
    // file is read over this copy at launch
    // (`app_state::session::preferences_file`).
    #[serde(default = "default_autosave_minutes")]
    autosave_minutes: u8,
    #[serde(default)]
    preferences: crate::workbench::preferences::UserPreferences,
    #[serde(default)]
    browser_spoken_feedback: bool,
    #[serde(default)]
    text_locale: crate::workbench::UiTextLocale,
    #[serde(default)]
    result_viewer: crate::workbench::documents::result_document::ResultViewer,
    /// Bit periods the reader pinned eyes to, for results that carry prepared
    /// provenance. Legacy keys name a run-local analysis id that a later
    /// session will never mint again, so they are dropped on save rather than
    /// restored onto whichever result inherits the number.
    #[serde(default)]
    eye_timebase: Vec<(
        rspice_results_ui::eye_diagram::EyeTimebaseKey,
        rspice_results_ui::eye_diagram::EyeTimebase,
    )>,
}

/// The autosave interval a session starts with when nothing chose one.
pub(crate) const DEFAULT_AUTOSAVE_MINUTES: u8 = 5;

fn default_autosave_minutes() -> u8 {
    DEFAULT_AUTOSAVE_MINUTES
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

fn default_true() -> bool {
    true
}

impl Default for UiSessionStateSer {
    fn default() -> Self {
        Self::from(&UiSessionState::new())
    }
}

impl From<&UiSessionState> for UiSessionStateSer {
    fn from(session: &UiSessionState) -> Self {
        Self {
            theme: session.theme,
            show_grid: session.grid.visible(),
            grid_style: Some(session.grid),
            grid_restore_style: Some(if session.grid_restore_style.visible() {
                session.grid_restore_style
            } else if session.grid.visible() {
                session.grid
            } else {
                GridStyle::Dots
            }),
            schematic_selection_filter: session.schematic_selection_filter,
            schematic_bulk_edit_filters: session.schematic_bulk_edit_filters.clone(),
            schematic_visibility: session.schematic_visibility,
            drawing_sheet_layers: session.drawing_sheet_layers,
            schematic_snap: session.schematic_snap.clone(),
            schematic_routing_mode: Some(session.schematic_routing_mode),
            duplicate_external_nets: session.duplicate_external_nets,
            engineering_table_views: session.engineering_table_views.clone(),
            autosave_minutes: normalize_autosave_minutes(session.autosave_minutes),
            preferences: session.preferences.clone(),
            browser_spoken_feedback: session.browser_spoken_feedback,
            text_locale: session.text_locale,
            result_viewer: session.results.session.viewer,
            eye_timebase: session
                .results
                .session
                .eye_timebase
                .iter()
                .filter(|(key, _)| key.is_persistable())
                .map(|(key, timebase)| (*key, *timebase))
                .collect(),
        }
    }
}

impl From<UiSessionStateSer> for UiSessionState {
    fn from(ser: UiSessionStateSer) -> Self {
        let grid = ser.grid_style.unwrap_or(if ser.show_grid {
            GridStyle::Dots
        } else {
            GridStyle::Off
        });
        let grid_restore_style = ser
            .grid_restore_style
            .filter(|style| style.visible())
            .unwrap_or(if grid.visible() {
                grid
            } else {
                GridStyle::Dots
            });
        let mut preferences = ser.preferences;
        preferences.normalize();
        let schematic_routing_mode =
            ser.schematic_routing_mode
                .unwrap_or(match ser.schematic_visibility.wire_routing {
                    SchematicWireRoutingStyle::Orthogonal => {
                        crate::state::WireRoutingMode::HorizontalFirst
                    }
                    SchematicWireRoutingStyle::FortyFiveDegree => {
                        crate::state::WireRoutingMode::FortyFiveDegree
                    }
                    SchematicWireRoutingStyle::FreeAngle => crate::state::WireRoutingMode::Diagonal,
                });
        let mut session = Self {
            theme: ser.theme,
            grid,
            grid_restore_style,
            schematic_selection_filter: ser.schematic_selection_filter,
            schematic_bulk_edit_filters: ser.schematic_bulk_edit_filters,
            schematic_visibility: ser.schematic_visibility,
            drawing_sheet_layers: ser.drawing_sheet_layers,
            schematic_snap: ser.schematic_snap,
            schematic_routing_mode,
            duplicate_external_nets: ser.duplicate_external_nets,
            engineering_table_views: ser.engineering_table_views,
            autosave_minutes: normalize_autosave_minutes(ser.autosave_minutes),
            preferences,
            browser_spoken_feedback: ser.browser_spoken_feedback,
            text_locale: ser.text_locale,
            ..Self::new()
        };
        session.results.session.viewer = ser.result_viewer;
        session.results.session.eye_timebase = ser
            .eye_timebase
            .into_iter()
            .filter(|(key, _)| key.is_persistable())
            .collect();
        session
    }
}

#[cfg(test)]
mod symbol_selection_tests {
    use super::*;
    use crate::state::{
        SchematicAnnotationVisibility, SchematicBackAnnotationContent, SchematicNetHighlighting,
        SchematicParameterLabelVisibility, SchematicReviewMarkerVisibility,
    };

    #[test]
    fn schematic_selection_filter_round_trips_with_the_ui_session() {
        let mut session = UiSessionState::new();
        session.schematic_selection_filter.instances = false;
        session.schematic_selection_filter.annotations = false;
        let serialized =
            serde_json::to_value(UiSessionStateSer::from(&session)).expect("serialize session");
        let wire: UiSessionStateSer =
            serde_json::from_value(serialized).expect("deserialize session");
        let restored = UiSessionState::from(wire);

        assert_eq!(
            restored.schematic_selection_filter,
            session.schematic_selection_filter
        );
    }

    #[test]
    fn ui_text_locale_round_trips_and_legacy_sessions_default_to_english() {
        let mut session = UiSessionState::new();
        session.text_locale = crate::workbench::UiTextLocale::PseudoExpanded;
        let serialized =
            serde_json::to_value(UiSessionStateSer::from(&session)).expect("serialize session");
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serialized).expect("deserialize session"),
        );
        assert_eq!(
            restored.text_locale,
            crate::workbench::UiTextLocale::PseudoExpanded
        );

        let legacy = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serde_json::json!({}))
                .expect("legacy session"),
        );
        assert_eq!(
            legacy.text_locale,
            crate::workbench::UiTextLocale::EnglishUnitedStates
        );
    }

    #[test]
    fn legacy_ui_session_defaults_to_all_selection_classes() {
        let wire: UiSessionStateSer =
            serde_json::from_value(serde_json::json!({})).expect("legacy session without a filter");
        let restored = UiSessionState::from(wire);

        assert!(
            restored.schematic_selection_filter.instances
                && restored.schematic_selection_filter.wires
                && restored.schematic_selection_filter.labels
                && restored.schematic_selection_filter.annotations
        );
    }

    #[test]
    fn schematic_bulk_edit_filters_round_trip_with_the_ui_session() {
        let mut session = UiSessionState::new();
        session.schematic_bulk_edit_filters.active = SelectionBulkFilter {
            query: "OPA*".to_owned(),
            object_kind: SelectionBulkObjectKind::InstancesAndParameters,
            hierarchy_scope: SelectionBulkHierarchyScope::CompleteProject,
            model_cell: "OPA189*".to_owned(),
            current_property: "section=tt".to_owned(),
        };
        session
            .schematic_bulk_edit_filters
            .save_active("TT amplifiers")
            .expect("save filter");

        let serialized =
            serde_json::to_value(UiSessionStateSer::from(&session)).expect("serialize session");
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serialized).expect("deserialize session"),
        );

        assert_eq!(
            restored.schematic_bulk_edit_filters,
            session.schematic_bulk_edit_filters
        );
    }

    #[test]
    fn personal_engineering_table_views_round_trip_with_the_ui_session() {
        let mut session = UiSessionState::new();
        let dataset =
            crate::state::engineering_table::active_schematic_dataset(&Default::default());
        let view = crate::state::engineering_table::EngineeringTableView::for_dataset(&dataset);
        session
            .engineering_table_views
            .save(
                "My device view",
                crate::state::engineering_table::EngineeringViewScope::Personal,
                view,
                true,
                &dataset,
            )
            .expect("valid personal view");

        let serialized =
            serde_json::to_value(UiSessionStateSer::from(&session)).expect("serialize session");
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serialized).expect("deserialize session"),
        );

        assert_eq!(
            restored.engineering_table_views,
            session.engineering_table_views
        );
    }

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

    #[test]
    fn legacy_ui_session_defaults_bulk_edit_filters_without_project_mutation() {
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serde_json::json!({}))
                .expect("legacy session without bulk filters"),
        );
        assert_eq!(
            restored.schematic_bulk_edit_filters,
            SelectionBulkFilterSession::default()
        );
    }

    #[test]
    fn schematic_visibility_policy_round_trips_with_the_ui_session() {
        let mut session = UiSessionState::new();
        session.schematic_visibility = SchematicVisibilityPolicy {
            annotations: SchematicAnnotationVisibility::ViolationsOnly,
            back_annotation: SchematicBackAnnotationContent::VoltagesCurrentsAndPower,
            parameter_labels: SchematicParameterLabelVisibility::Hidden,
            wire_routing: SchematicWireRoutingStyle::FortyFiveDegree,
            net_highlighting: SchematicNetHighlighting::Off,
            review_markers: SchematicReviewMarkerVisibility::All,
        };

        let serialized =
            serde_json::to_value(UiSessionStateSer::from(&session)).expect("serialize session");
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serialized).expect("deserialize session"),
        );

        assert_eq!(restored.schematic_visibility, session.schematic_visibility);
    }

    /// Earlier builds saved a hierarchy-context choice with the policy (C-13,
    /// J-3: the dimmed-parent overlay it governed is gone). A session written
    /// then still loads, and every choice saved beside it survives.
    #[test]
    fn a_session_saved_with_the_retired_hierarchy_choice_still_loads() {
        let legacy = serde_json::json!({
            "schematic_visibility": {
                "hierarchy": "full-visible-hierarchy",
                "annotations": "violations-only",
                "back_annotation": "voltages-only",
                "parameter_labels": "hidden",
                "wire_routing": "forty-five-degree",
                "net_highlighting": "off",
                "review_markers": "all"
            }
        });
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(legacy)
                .expect("a session carrying the retired hierarchy choice loads"),
        );

        assert_eq!(
            restored.schematic_visibility,
            SchematicVisibilityPolicy {
                annotations: SchematicAnnotationVisibility::ViolationsOnly,
                back_annotation: SchematicBackAnnotationContent::VoltagesOnly,
                parameter_labels: SchematicParameterLabelVisibility::Hidden,
                wire_routing: SchematicWireRoutingStyle::FortyFiveDegree,
                net_highlighting: SchematicNetHighlighting::Off,
                review_markers: SchematicReviewMarkerVisibility::All,
            }
        );
    }

    /// Before every sheet took its standard's frame, the drawing-sheet layers
    /// held a guide to the printable margins around an inset frame, and the
    /// centring-mark layer was the registration-mark layer. An eframe session
    /// written then still loads, keeping the mark choice.
    #[test]
    fn a_ron_session_saved_with_retired_drawing_sheet_layers_still_loads() {
        let session = UiSessionState::new();
        let today = ron::to_string(&session.drawing_sheet_layers).unwrap();
        let written = ron::to_string(&UiSessionStateSer::from(&session)).unwrap();
        assert!(written.contains(&today), "the session holds its layers");
        let legacy = written.replacen(
            &today,
            "(margins:false,zones:false,title_block:true,registration_marks:false,\
             dimensions_at_detail_zoom:true)",
            1,
        );

        let restored = UiSessionState::from(
            ron::from_str::<UiSessionStateSer>(&legacy)
                .expect("a session carrying the retired layers loads"),
        );

        assert_eq!(
            restored.drawing_sheet_layers,
            crate::state::DrawingSheetLayerVisibility {
                zones: false,
                title_block: true,
                centring_marks: false,
                dimensions_at_detail_zoom: true,
            }
        );
    }

    #[test]
    fn grid_cycle_is_dots_then_lines_then_off_and_round_trips_exactly() {
        let mut session = UiSessionState::new();
        session.set_grid_style(GridStyle::Dots);
        assert_eq!(session.cycle_grid_style(), GridStyle::Lines);
        assert_eq!(session.cycle_grid_style(), GridStyle::Off);
        assert_eq!(session.grid, GridStyle::Off);
        assert_eq!(session.grid_restore_style, GridStyle::Lines);

        let serialized =
            serde_json::to_value(UiSessionStateSer::from(&session)).expect("serialize session");
        let mut restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serialized).expect("deserialize session"),
        );

        assert_eq!(restored.grid, GridStyle::Off);
        assert_eq!(restored.grid_restore_style, GridStyle::Lines);
        assert_eq!(restored.cycle_grid_style(), GridStyle::Dots);
        assert_eq!(restored.cycle_grid_style(), GridStyle::Lines);
    }

    #[test]
    fn legacy_ui_session_uses_safe_visibility_defaults() {
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serde_json::json!({}))
                .expect("legacy session without visibility policy"),
        );

        assert_eq!(
            restored.schematic_visibility,
            SchematicVisibilityPolicy::default()
        );
    }

    #[test]
    fn schematic_snap_and_exact_routing_round_trip_with_the_ui_session() {
        let mut session = UiSessionState::new();
        session.schematic_snap.enabled = false;
        session.schematic_snap.snap_to_wire_segments = false;
        session.schematic_snap.snap_radius = 7;
        session.schematic_routing_mode = crate::state::WireRoutingMode::VerticalFirst;

        let serialized =
            serde_json::to_value(UiSessionStateSer::from(&session)).expect("serialize session");
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serialized).expect("deserialize session"),
        );

        assert_eq!(restored.schematic_snap, session.schematic_snap);
        assert_eq!(
            restored.schematic_routing_mode,
            crate::state::WireRoutingMode::VerticalFirst
        );
    }

    #[test]
    fn duplicate_external_nets_round_trips_with_the_ui_session() {
        let mut session = UiSessionState::new();
        session.duplicate_external_nets =
            crate::state::DuplicateExternalNets::PreserveNamedNetAttachment;

        let serialized =
            serde_json::to_value(UiSessionStateSer::from(&session)).expect("serialize session");
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serialized).expect("deserialize session"),
        );

        assert_eq!(
            restored.duplicate_external_nets,
            crate::state::DuplicateExternalNets::PreserveNamedNetAttachment
        );
    }

    #[test]
    fn legacy_ui_session_uses_snap_and_routing_defaults() {
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serde_json::json!({}))
                .expect("legacy session without editor runtime settings"),
        );

        assert_eq!(restored.schematic_snap, crate::state::SnapEngine::default());
        assert_eq!(
            restored.schematic_routing_mode,
            crate::state::WireRoutingMode::HorizontalFirst
        );
        assert_eq!(
            restored.duplicate_external_nets,
            crate::state::DuplicateExternalNets::LeaveUnconnected
        );
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
