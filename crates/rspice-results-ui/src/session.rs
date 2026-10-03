//! Local result viewer controls, selections, annotations and viewports.
use crate::{
    chrome::instrument::ResultPlotTool,
    derived::DerivedSeries,
    events::DigitalEventSelection,
    eye_diagram::{EyeTimebase, view::EyeTexture},
    fft::view::FftSeries,
    presentation::PlotView,
    selection::{
        ResultArtifactPresentationKey, ResultBrowserSelectionKey, SelectedResultTrace,
        SourceWaveformPresentationKey,
    },
    soa::{SoaRuleFilter, SoaRuleSelection},
    specs::editor::SpecDraft,
};
use rspice_app_types::product::{DatasetId, ResultDocumentId};
use rspice_results::{
    events::projection::BusRadix,
    family_projection::SourceSampleSelection,
    result_presentation::{
        AnalysisPresentationKey, AnalysisPresentationSource, ExprTrace, MarkerKind, ResultMarker,
        ResultViewer, WavePanePresentationKey, WaveformPresentationKey, viewer_uses_wave_stack,
    },
    visualization_document::{MarkerId, PaneId},
};
use rspice_ui_kit::plot::{CursorPair, DecimationCache};
use std::collections::{HashMap, HashSet};

/// One axis interval, low then high, in data space.
pub type AxisExtent = (f64, f64);
/// The X and Y intervals one plot last drew.
pub type DrawnAxes = (AxisExtent, AxisExtent);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlotPresentationKey {
    Global(usize),
    /// One waveform stack strip, named by the analysis it draws rather than
    /// by the one dataset that analysis produced.
    ///
    /// A re-run mints a new dataset identity, so keying the strip by the
    /// dataset threw away the reader's zoom on every run — while the
    /// ordinal-keyed single-canvas sheets beside it kept theirs, which is
    /// what [`ResultViewerState::views`] says the map is for.
    Analysis(AnalysisPresentationSource),
    Document(ResultDocumentId, PaneId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PersistentPaneContext {
    pub document_id: ResultDocumentId,
    pub pane_id: PaneId,
    pub analysis: AnalysisPresentationKey,
}

/// A viewport gesture on the active sheet, as asked for by a command.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewGesture {
    /// Drop every pinned viewport on the active plot.
    Fit,
    /// Halve the visible interval about its centre.
    ZoomIn,
    /// Double the visible interval about its centre.
    ZoomOut,
    /// Pin one or both axes to exact data-space intervals.
    ///
    /// `None` leaves that axis unchanged. This is queued instead of writing a
    /// viewport directly so the waveform stack can resolve the active stable
    /// analysis and unit-pane identity inside its drawing pass.
    SetRanges {
        x: Option<(f64, f64)>,
        y: Option<(f64, f64)>,
    },
}

/// The A│B cursor tool: armed, a click on a plot places cursor A and then
/// B; disarmed, plots ignore cursor clicks and the readout strip stands
/// down, so the tool state and what is on screen can never disagree.
///
/// Armed by default — placing a cursor is the first thing anyone does with
/// a waveform, and an unarmed default would read as a dead plot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorTool(bool);

impl Default for CursorTool {
    fn default() -> Self {
        Self(true)
    }
}

impl CursorTool {
    /// `true` when plot clicks place cursors.
    pub const fn is_armed(self) -> bool {
        self.0
    }
}

/// Which store owns one marker, and where in it.
///
/// The two stores are genuinely different objects: a quick marker is a
/// project-scoped annotation on a dataset, and a document marker is a retained
/// entity of one project-owned visualization document. Naming the store in the
/// identity means every edit routes by construction — there is no shared
/// integer space to collide in, and no truncation of the document's full-width
/// entity serial into a session id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerSelector {
    /// Project-persisted quick-view marker in [`ResultViewerState::markers`].
    Quick(u32),
    /// Retained marker of one project-owned visualization document pane.
    Document {
        document_id: ResultDocumentId,
        pane_id: PaneId,
        marker_id: MarkerId,
    },
}

/// An uncommitted edit of one marker's purpose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkerEditDraft {
    pub selector: MarkerSelector,
    pub note: String,
    pub kind: MarkerKind,
}

/// Per-frame projection of one persistent pane's retained markers.
///
/// The project-owned visualization document is the only owner of these: the
/// overlay is rebuilt by projection every frame, never serialized, and never
/// saved into the project's quick-view marker list. It exists so the renderer
/// and the readout can draw a retained marker without the document having to
/// impersonate a quick one.
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentMarker {
    pub document_id: ResultDocumentId,
    pub pane_id: PaneId,
    /// The document's own retained serial, at full width.
    pub retained_id: MarkerId,
    /// Dataset-bound identity of the strip this marker lives on.
    pub analysis: AnalysisPresentationKey,
    /// Dataset-bound identity of the trace the marker rides.
    pub anchor: WaveformPresentationKey,
    pub trace_name: String,
    pub x: f64,
    pub kind: MarkerKind,
    pub note: String,
}

/// One marker to draw or list, whichever store owns it.
///
/// Both stores can be live at once — a document pane can carry a quick marker
/// when the retained document does not own the clicked trace — so every
/// consumer reads one union rather than picking a store and being wrong half
/// the time.
#[derive(Debug, Clone, Copy)]
pub enum MarkerView<'a> {
    Quick(&'a ResultMarker),
    Document(&'a DocumentMarker),
}

impl<'a> MarkerView<'a> {
    pub fn selector(self) -> MarkerSelector {
        match self {
            MarkerView::Quick(marker) => MarkerSelector::Quick(marker.id),
            MarkerView::Document(marker) => MarkerSelector::Document {
                document_id: marker.document_id,
                pane_id: marker.pane_id,
                marker_id: marker.retained_id,
            },
        }
    }

    pub fn analysis(self) -> AnalysisPresentationKey {
        match self {
            MarkerView::Quick(marker) => marker.analysis,
            MarkerView::Document(marker) => marker.analysis,
        }
    }

    pub fn anchor(self) -> &'a WaveformPresentationKey {
        match self {
            MarkerView::Quick(marker) => &marker.anchor,
            MarkerView::Document(marker) => &marker.anchor,
        }
    }

    pub fn trace_name(self) -> &'a str {
        match self {
            MarkerView::Quick(marker) => &marker.trace_name,
            MarkerView::Document(marker) => &marker.trace_name,
        }
    }

    pub fn x(self) -> f64 {
        match self {
            MarkerView::Quick(marker) => marker.x,
            MarkerView::Document(marker) => marker.x,
        }
    }

    pub fn kind(self) -> MarkerKind {
        match self {
            MarkerView::Quick(marker) => marker.kind,
            MarkerView::Document(marker) => marker.kind,
        }
    }

    pub fn note(self) -> &'a str {
        match self {
            MarkerView::Quick(marker) => &marker.note,
            MarkerView::Document(marker) => &marker.note,
        }
    }

    /// What the tag reads on the plot and in the readout.
    ///
    /// The two stores allocate independently, so they are given distinct
    /// prefixes: an `M7` quick marker and a `D7` retained one can share a
    /// strip without either label claiming to name the other.
    pub fn display_id(self) -> String {
        match self {
            MarkerView::Quick(marker) => format!("M{}", marker.id),
            MarkerView::Document(marker) => format!("D{}", marker.retained_id.get()),
        }
    }
}

/// One horizontal measurement cursor bound to an exact waveform pane.
#[derive(Debug, Clone, PartialEq)]
pub struct HorizontalWaveCursor {
    pub pane: WavePanePresentationKey,
    pub y: f64,
}

/// One optimizer candidate picked out of the iteration history.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptimizationSelection {
    pub analysis: AnalysisPresentationKey,
    pub iteration_index: usize,
}

/// The marker tool: armed, a plot click drops a marker on the nearest
/// visible trace. Off by default — unlike cursors, annotating is a
/// deliberate act, and an always-armed default would litter the plot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MarkerTool(bool);

impl MarkerTool {
    /// `true` when plot clicks place markers.
    pub const fn is_armed(self) -> bool {
        self.0
    }
}

/// State of the inline expression editor under a strip header.
#[derive(Debug, Clone)]
pub struct ExprEditor {
    /// The dataset-bound strip the editor is attached to.
    pub analysis: AnalysisPresentationKey,
    /// Text being edited.
    pub text: String,
    /// Last evaluation error, shown inline.
    pub error: Option<String>,
    /// Request keyboard focus on the next frame (set when opened).
    pub want_focus: bool,
}

/// Which axis an explicit range applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneAxis {
    X,
    Y,
}

/// Device-local result viewer state over explicit retained identities.
#[derive(Debug, Clone, Default)]
pub struct ResultViewerState {
    /// Active viewer tab.
    pub viewer: ResultViewer,
    /// Device-local page selection for each project-owned result document.
    ///
    /// Page selection is presentation state, not part of the immutable
    /// visualization document. Stable document and page identities prevent a
    /// project reload or page reorder from selecting a different page.
    pub persistent_document_pages: std::collections::HashMap<
        rspice_app_types::product::ResultDocumentId,
        rspice_results::visualization_document::PageId,
    >,
    /// Device-local pane selection for each project-owned result document.
    ///
    /// Kept per document for the same reason page selection is: which pane a
    /// reader is working in is a fact about one document. A single global
    /// selection meant activating a pane in one document selected the pane
    /// with the same serial in every other open one, and switching documents
    /// forgot where the reader had been.
    pub persistent_document_panes:
        std::collections::HashMap<rspice_app_types::product::ResultDocumentId, PaneId>,
    /// Exact retained pane currently projected through a native renderer.
    /// This scopes otherwise session-only plot state and routes edits back to
    /// the project-owned visualization document.
    pub persistent_pane_context: Option<PersistentPaneContext>,
    /// Markers of the persistent pane currently being projected.
    ///
    /// Single-pane and wholesale-replaced: panes project, render and capture
    /// strictly in sequence, and the active pane re-projects last, so the
    /// stage readout always observes the pane the reader is working in. Each
    /// entry still names its own document and pane so a read can verify
    /// against `persistent_pane_context` rather than trust the cadence.
    ///
    /// Transient by construction — it is a projection of retained document
    /// entities, never a second owner of them, and is never serialized.
    pub document_markers: Vec<DocumentMarker>,
    /// The A│B cursor tool.
    pub cursor_tool: CursorTool,
    /// Polar sheet controls: the network term, the radius ruling, the decade
    /// marks and the normalization.
    pub polar: crate::polar::PolarSheetState,
    /// Contribution sheet control: the frequency a swept study is read at.
    pub study: crate::sensitivity::study::SensitivitySheetState,
    /// Scatter sheet controls and the brushed trial selection.
    pub scatter: crate::scatter::ScatterSheetState,
    /// Box/violin sheet controls: the grouping, the margin scale, the body
    /// and the whisker rule.
    pub box_violin: crate::box_violin::BoxViolinSheetState,
    /// The marker tool.
    pub marker_tool: MarkerTool,
    /// Primary plot pointer tool shown in the 31 px instrument strip.
    pub plot_tool: ResultPlotTool,
    /// Exact retained waveform selected from a trace chip. This is only an
    /// identity into the canonical result dataset; it never copies samples or
    /// creates a second result owner.
    pub selected_trace: Option<SelectedResultTrace>,
    /// Exact non-waveform Data Browser row selected for inspection. Like
    /// `selected_trace`, this is only an immutable source identity.
    pub selected_result_artifact: Option<ResultArtifactPresentationKey>,
    /// How the Events sheet spells a bus word. One per sheet, like the polar
    /// sheet's radius ruling: a radix is how the reader is reading, not a
    /// property of any one declaration.
    pub event_bus_radix: BusRadix,
    /// Buses whose member rows the Events sheet lists beside the bus row.
    ///
    /// Keyed by bus name alone rather than by analysis: a reader who opened
    /// `count[1:0]` to its bits wants the same of the same bus in the next
    /// run, and a declaration that is not in the active analysis contributes
    /// no rows either way.
    pub expanded_event_buses: std::collections::BTreeSet<String>,
    /// User-placed markers across every waveform strip.
    pub markers: Vec<ResultMarker>,
    /// Signals starred in the results data browser. A deliberate,
    /// session-scoped mark like `markers`, keyed by immutable dataset, stable
    /// analysis identity, and retained waveform name. Equal names in another
    /// run cannot inherit this mark.
    pub favorite_signals: HashSet<SourceWaveformPresentationKey>,
    /// Personal favorites for typed non-waveform quantities.
    pub favorite_result_artifacts: HashSet<ResultArtifactPresentationKey>,
    /// Most-recent-first stable waveform identities the user selected or
    /// revealed, deduplicated and bounded: the browser's Recent scope is this
    /// order.
    pub recent_signals: Vec<SourceWaveformPresentationKey>,
    /// Most-recent-first typed artifact identities.
    pub recent_result_artifacts: Vec<ResultArtifactPresentationKey>,
    /// Quantities check-marked in the browser for a batch action. Session
    /// state like the marks above it; the immutable dataset never sees it and
    /// equal names in another retained analysis are not selected implicitly.
    pub checked_result_quantities: HashSet<ResultBrowserSelectionKey>,
    /// Stable end point for Shift+click / Shift+Arrow range selection in the
    /// filtered browser inventory. The range itself is recomputed from the
    /// current deterministic row order, so filtering never leaves an ordinal
    /// pointing at a different quantity.
    pub browser_range_anchor: Option<ResultBrowserSelectionKey>,
    /// Highest allocated or restored quick-marker ID in this live project.
    /// Restoring markers can raise it; deleting markers never lowers it.
    next_marker_id: u32,
    /// Allocation history owned by the current Results draft. Revert restores
    /// this value while the live allocator above retains its monotonic floor.
    marker_allocation_high_water: u32,
    /// The open marker-purpose dialog's uncommitted edit, if any.
    ///
    /// Editing is transactional: nothing reaches the marker until Apply, so
    /// a half-typed label can always be abandoned. Reclassifying a marker is
    /// a decision, not a side effect of clicking its kind.
    pub marker_edit: Option<MarkerEditDraft>,
    /// Whether the stage-level cursor/marker dock is collapsed. Session-only
    /// presentation state; retained result documents never serialize it.
    pub readout_collapsed: bool,
    /// A/B cursors (data-space X of the strip they live on).
    pub cursors: CursorPair,
    /// Dataset-bound strip identity the cursors were placed on.
    pub cursor_strip: Option<usize>,
    /// Exact visible trace nearest cursor A when A was placed.
    pub cursor_a_anchor: Option<WaveformPresentationKey>,
    /// Unit-scoped waveform pane receiving instrument actions.
    pub active_wave_pane: Option<WavePanePresentationKey>,
    /// Horizontal cursor, bound to the pane whose Y axis owns its value.
    pub horizontal_cursor: Option<HorizontalWaveCursor>,
    /// Unit-scoped panes the user put on a logarithmic Y axis.
    ///
    /// This lived in egui's persisted memory under a hand-built id, which put
    /// a project-scoped presentation decision outside every owner that knows
    /// about projects: `clear_project_scoped_state` could not reach it, the
    /// project file could not carry it, and it accumulated an entry for every
    /// dataset the user ever opened. It is the same class of fact as a marker
    /// and it is kept the same way.
    pub log_y_panes: HashSet<WavePanePresentationKey>,
    /// Draw exact project specification bounds compatible with visible axes.
    pub show_spec_limits: bool,
    /// Draw derived min/max curves only when retained family samples exist.
    pub show_family_envelope: bool,
    /// Draw unlabeled minor subdivisions between authoritative major ticks.
    pub show_minor_grid: bool,
    /// Share the same A/B cursor positions across every compatible waveform
    /// strip instead of scoping them to `cursor_strip`.
    pub linked_cursors: bool,
    /// Decimation envelope cache.
    pub cache: DecimationCache,
    /// Derived dB/phase series cache.
    pub derived: DerivedSeries,
    /// Presentation-only exact family rows selected by Visualization Studio.
    /// Source datasets are never modified by this projection.
    pub sample_selection: Option<SourceSampleSelection>,
    /// Session-only visibility overrides for exact family group traces. These
    /// are presentation state and never alter source WaveformData visibility.
    pub hidden_family_traces: HashSet<crate::waves::FamilyTraceVisibilityKey>,
    /// Session-only source-waveform visibility overrides for quick views.
    ///
    /// A missing key means "use the immutable dataset default". Keeping this
    /// tri-state representation allows a source that defaults hidden to be
    /// revealed without writing into solver-owned result data.
    pub waveform_visibility: HashMap<SourceWaveformPresentationKey, bool>,
    /// Strips hidden via the strip-close action.
    pub hidden_strips: HashSet<AnalysisPresentationKey>,
    /// Strip currently maximized via the strip action, if any.
    pub maximized_strip: Option<AnalysisPresentationKey>,
    /// Cached FFT display arrays for the active spectrum revision.
    pub fft_series: Option<FftSeries>,
    /// Baked EYE density texture for the active eye revision and size.
    pub eye_texture: Option<EyeTexture>,
    /// Zoom/pan overrides per plot, keyed by stable plot identity. Survives
    /// re-runs on purpose — keeping the zoomed window across parameter
    /// tweaks is how engineers compare iterations.
    pub views: std::collections::HashMap<(ResultViewer, PlotPresentationKey, usize), PlotView>,
    /// What each single-canvas sheet's axes actually spanned when it last
    /// drew, pinned or fitted.
    ///
    /// The axis-limit editor has to open on the interval the reader is
    /// looking at, and an unpinned sheet's interval is derived inside the
    /// sheet from its own data. The waveform stack is deliberately absent:
    /// it reports its panes through `active_pane_facts`, which knows which
    /// of several panes is active — something a last-drawn record cannot.
    /// Transient.
    pub drawn_axes: std::collections::HashMap<ResultViewer, DrawnAxes>,
    /// A viewport gesture asked for from outside the drawing pass.
    ///
    /// Menus, the command palette and shortcuts all reach the workspace
    /// without an `egui::Context`, and resolving which pane a gesture applies
    /// to needs the sheet's built models and theme tokens. Queueing here and
    /// applying inside the viewer well keeps one owner for the gesture
    /// instead of a second, token-free approximation of pane resolution.
    pub pending_view_gesture: Option<ViewGesture>,
    /// User expression traces per dataset-bound waves strip, evaluated by
    /// the calculator against that analysis' waveforms.
    /// Compatibility projection for integrations that still address the
    /// active run by ordinal. The Results document migrates these entries
    /// into `analysis_exprs` before use; stable state lives there.
    pub exprs: std::collections::HashMap<usize, Vec<ExprTrace>>,
    pub analysis_exprs: std::collections::HashMap<AnalysisPresentationKey, Vec<ExprTrace>>,
    /// Bit period the reader has pinned the eye to, per result.
    ///
    /// Absent means the eye recovers the period from the waveform. Project
    /// scoped like the log-axis panes: a rate stated about one project's
    /// result is not a statement about the next project's, and
    /// `clear_project_scoped_state` resets it with the rest of the document.
    pub eye_timebase: std::collections::HashMap<crate::eye_diagram::EyeTimebaseKey, EyeTimebase>,
    /// Identity map behind the ordinal compatibility projection.
    pub expr_projection_keys: std::collections::HashMap<usize, AnalysisPresentationKey>,
    /// The inline expression editor, when open (one strip at a time).
    pub expr_editor: Option<ExprEditor>,
    /// Pinned data point per XY viewer (trace slot, point index) — the
    /// Smith/Nyquist/PZ click-to-pin readout.
    pub rf_pin: std::collections::HashMap<ResultViewer, (usize, usize)>,
    /// Display AC phase traces unwrapped into a continuous curve instead of
    /// wrapped to ±180°. Applies to the BODE phase trace and the waves
    /// strips' phase traces; the margin math always reads the raw wrapped
    /// arrays. Transient — not persisted with the session.
    pub phase_continuous: bool,
    /// Screen rect of the document well (docbar excluded) from the last
    /// rendered frame — the crop window for viewer PNG export. Transient.
    pub well_rect: Option<egui::Rect>,
    /// OP inspector device-name filter (docbar input). Transient.
    pub op_filter: String,
    /// OP inspector sort: (column key, descending). Transient.
    pub op_sort: Option<(String, bool)>,
    /// The axis-limit field being typed, if any.
    ///
    /// Editing is transactional like the marker dialog: the pinned interval
    /// only moves on commit, so a half-typed bound never rescales the plot
    /// under the reader's hands. Transient.
    pub axis_limit_draft: Option<(ResultViewer, PaneAxis, String)>,
    /// Open spec-editor rows (None = matrix view). Transient.
    pub spec_drafts: Option<Vec<SpecDraft>>,
    /// SOA rule whose evidence the inspector reports. Transient.
    pub selected_soa_rule: Option<SoaRuleSelection>,
    /// Verdict filter applied to the SOA rule table. Transient.
    pub soa_rule_filter: SoaRuleFilter,
    /// Whether the selected SOA rule's stress history is drawn above the
    /// table. Off by default: the table is the evidence, the trace is the
    /// follow-up question.
    pub soa_stress_trace_open: bool,
    /// Optimizer candidate whose retained cost and variables the inspector
    /// reports. Transient.
    pub selected_optimization: Option<OptimizationSelection>,
    /// Event whose exact value and provenance the inspector reports.
    /// Transient.
    pub selected_digital_event: Option<DigitalEventSelection>,
    /// Row/column selection for the TABLE viewer.
    pub table: crate::table::TableView,
    /// Last row count the table rendered, as its footer states it. Written
    /// by the viewer so the docbar reports what is actually on screen
    /// rather than recomputing a second, possibly different, answer.
    pub table_status: Option<String>,
}

impl ResultViewerState {
    /// The unit-pane ordinal a retained pane's axis ranges describe.
    ///
    /// A retained pane carries one horizontal and one vertical axis, while the
    /// waveform stack draws several unit panes inside it — volts and amps do
    /// not share a Y scale. The document can therefore only state one of those
    /// verticals, and it states the first, by construction. Every other unit
    /// pane's zoom is session state and stays out of the document rather than
    /// being written into it under whichever ordinal a hash map happened to
    /// yield first.
    const RETAINED_PANE_ORDINAL: usize = 0;
    /// Clear cursors (Esc, Clear action).
    pub fn clear_cursors(&mut self) {
        self.cursors.clear();
        self.cursor_strip = None;
        self.cursor_a_anchor = None;
    }

    /// Arm or disarm the A│B tool. Disarming clears the pair, because a
    /// cursor nobody can move or read is not a cursor.
    pub fn toggle_cursor_tool(&mut self) {
        self.cursor_tool = CursorTool(!self.cursor_tool.is_armed());
        if !self.cursor_tool.is_armed() {
            self.clear_cursors();
        }
    }

    /// `true` when the cursor readout has something to say: the tool is
    /// armed and at least cursor A is placed.
    pub fn cursor_readout_active(&self) -> bool {
        self.cursor_tool.is_armed() && self.cursors.any()
    }

    /// Whether an ordinary plot click should place A/B. Box-zoom and pan own
    /// their primary drag/click gestures even while existing cursors remain
    /// visible in the readout strip.
    pub fn cursor_placement_enabled(&self) -> bool {
        self.plot_tool == ResultPlotTool::Cursor && self.cursor_tool.is_armed()
    }

    pub fn horizontal_cursor_placement_enabled(&self) -> bool {
        self.plot_tool == ResultPlotTool::HorizontalCursor
    }

    pub fn cursor_a_is_next(&self) -> bool {
        self.cursors.a.is_none() || self.cursors.b.is_some()
    }

    pub fn toggle_linked_cursors(&mut self) {
        self.linked_cursors = !self.linked_cursors;
    }

    /// Arm or disarm the marker tool. Disarming keeps the markers — they are
    /// document content, not a transient readout like the A/B pair.
    #[cfg(any(test, feature = "test-support"))]
    pub fn toggle_marker_tool(&mut self) {
        self.marker_tool = MarkerTool(!self.marker_tool.is_armed());
    }

    /// Place a valid marker without reusing an identity in this live project.
    pub fn add_marker(
        &mut self,
        analysis: AnalysisPresentationKey,
        anchor: WaveformPresentationKey,
        trace_name: String,
        x: f64,
    ) -> Result<u32, String> {
        ResultMarker::validate_placement(analysis, &anchor, x)?;
        let id = self.marker_id_high_water().max(self.next_marker_id).checked_add(1)
            .ok_or("result marker identity space is exhausted; existing markers can still be edited or removed")?;
        self.next_marker_id = id;
        self.marker_allocation_high_water = id;
        self.markers.push(ResultMarker {
            id,
            analysis,
            anchor,
            trace_name,
            x,
            kind: MarkerKind::default(),
            note: String::new(),
        });
        Ok(id)
    }

    pub fn marker_mut(&mut self, id: u32) -> Option<&mut ResultMarker> {
        self.markers.iter_mut().find(|marker| marker.id == id)
    }

    /// Remove one quick marker, and with it any open edit of it.
    pub fn remove_marker(&mut self, id: u32) {
        self.remember_marker_ids();
        self.markers.retain(|marker| marker.id != id);
        if self
            .marker_edit
            .as_ref()
            .is_some_and(|draft| draft.selector == MarkerSelector::Quick(id))
        {
            self.marker_edit = None;
        }
    }

    /// Adopt markers restored from a project, keeping the id allocator ahead
    /// of every label already in use.
    pub fn adopt_markers(&mut self, markers: Vec<ResultMarker>, high_water: u32) {
        let live_high_water = self.next_marker_id.max(self.marker_id_high_water());
        self.marker_allocation_high_water = markers
            .iter()
            .map(|marker| marker.id)
            .max()
            .unwrap_or(0)
            .max(high_water);
        self.next_marker_id = live_high_water.max(self.marker_allocation_high_water);
        self.markers = markers;
        self.marker_edit = None;
    }

    pub fn marker_id_high_water(&self) -> u32 {
        self.marker_allocation_high_water.max(
            self.markers
                .iter()
                .map(|marker| marker.id)
                .max()
                .unwrap_or(0),
        )
    }

    /// Retain even IDs inserted through the legacy mutable marker collection
    /// before removing the last evidence of them.
    pub fn remember_marker_ids(&mut self) {
        self.marker_allocation_high_water = self.marker_id_high_water();
        self.next_marker_id = self.next_marker_id.max(self.marker_allocation_high_water);
    }

    /// Replace the overlay with one persistent pane's retained markers.
    ///
    /// This writes nothing into the quick-view store and never touches the
    /// quick id allocator: a retained marker's serial belongs to its document,
    /// and letting it advance `next_marker_id` made the two stores contend for
    /// one integer space.
    pub fn project_document_markers(&mut self, markers: Vec<DocumentMarker>) {
        self.document_markers = markers;
        if let Some(draft) = &self.marker_edit
            && let MarkerSelector::Document { marker_id, .. } = draft.selector
            && !self
                .document_markers
                .iter()
                .any(|marker| marker.retained_id == marker_id)
        {
            // The marker vanished under the dialog: the draft has nothing to
            // apply to and must not linger.
            self.marker_edit = None;
        }
    }

    /// One retained overlay marker by its document serial.
    pub fn document_marker(&self, marker_id: MarkerId) -> Option<&DocumentMarker> {
        self.document_markers
            .iter()
            .find(|marker| marker.retained_id == marker_id)
    }

    /// Markers on one strip, in placement order, from both stores.
    ///
    /// Retained document markers come last so they paint over the quick ones:
    /// on a persistent pane the document's own annotation is the authority.
    /// The overlay is additionally verified against the live pane context, so
    /// a stale projection cannot leak a document's markers onto a quick strip.
    pub fn strip_markers(
        &self,
        analysis: AnalysisPresentationKey,
    ) -> impl Iterator<Item = MarkerView<'_>> {
        let context = self.persistent_pane_context;
        self.markers
            .iter()
            .filter(move |marker| marker.analysis == analysis)
            .map(MarkerView::Quick)
            .chain(
                self.document_markers
                    .iter()
                    .filter(move |marker| {
                        marker.analysis == analysis
                            && context.is_some_and(|context| {
                                context.document_id == marker.document_id
                                    && context.pane_id == marker.pane_id
                            })
                    })
                    .map(MarkerView::Document),
            )
    }

    pub fn waveform_visibility(
        &self,
        key: &SourceWaveformPresentationKey,
        dataset_default: bool,
    ) -> bool {
        self.waveform_visibility
            .get(key)
            .copied()
            .unwrap_or(dataset_default)
    }

    pub fn sync_expression_projection(
        &mut self,
        analysis: AnalysisPresentationKey,
        analysis_index: usize,
    ) {
        match self.analysis_exprs.get(&analysis).cloned() {
            Some(exprs) => {
                self.exprs.insert(analysis_index, exprs);
            }
            None => {
                self.exprs.remove(&analysis_index);
            }
        }
        self.expr_projection_keys.insert(analysis_index, analysis);
    }

    /// Clear design results within the same project, retaining its marker IDs.
    pub fn clear_design_scoped_state(&mut self) {
        self.remember_marker_ids();
        let next_marker_id = self.next_marker_id;
        let marker_allocation_high_water = self.marker_allocation_high_water;
        self.clear_project_scoped_state();
        self.next_marker_id = next_marker_id;
        self.marker_allocation_high_water = marker_allocation_high_water;
    }

    /// Reset result UI state and identities when replacing the whole project.
    pub fn clear_project_scoped_state(&mut self) {
        let viewer = self.viewer;
        let phase_continuous = self.phase_continuous;
        *self = Self {
            viewer,
            phase_continuous,
            ..Self::default()
        };
    }

    pub fn persistent_document_page(
        &self,
        document_id: rspice_app_types::product::ResultDocumentId,
    ) -> Option<rspice_results::visualization_document::PageId> {
        self.persistent_document_pages.get(&document_id).copied()
    }

    pub fn select_persistent_document_page(
        &mut self,
        document_id: rspice_app_types::product::ResultDocumentId,
        page_id: rspice_results::visualization_document::PageId,
    ) {
        self.persistent_document_pages.insert(document_id, page_id);
    }

    pub fn persistent_document_pane(
        &self,
        document_id: rspice_app_types::product::ResultDocumentId,
    ) -> Option<PaneId> {
        self.persistent_document_panes.get(&document_id).copied()
    }

    pub fn select_persistent_document_pane(
        &mut self,
        document_id: rspice_app_types::product::ResultDocumentId,
        pane_id: PaneId,
    ) {
        self.persistent_document_panes.insert(document_id, pane_id);
    }

    pub fn enter_persistent_pane(
        &mut self,
        document_id: ResultDocumentId,
        pane_id: PaneId,
        analysis: AnalysisPresentationKey,
    ) {
        self.persistent_pane_context = Some(PersistentPaneContext {
            document_id,
            pane_id,
            analysis,
        });
    }

    /// Leave the persistent projection: the pane context and the retained
    /// marker overlay it scopes both belong to the document that was open.
    pub fn leave_persistent_document(&mut self) {
        self.persistent_pane_context = None;
        self.document_markers.clear();
    }

    pub fn persistent_viewer_key(viewer: ResultViewer) -> ResultViewer {
        if viewer_uses_wave_stack(viewer) {
            ResultViewer::Waves
        } else {
            viewer
        }
    }

    pub fn project_persistent_plot_view(
        &mut self,
        viewer: ResultViewer,
        x: Option<(f64, f64)>,
        y: Option<(f64, f64)>,
    ) {
        let Some(context) = self.persistent_pane_context else {
            return;
        };
        let viewer = Self::persistent_viewer_key(viewer);
        let plot = PlotPresentationKey::Document(context.document_id, context.pane_id);
        let key = (viewer, plot, Self::RETAINED_PANE_ORDINAL);
        // Only the ordinal the document actually states is replaced. Clearing
        // every ordinal here destroyed the zoom of every other unit pane on
        // every frame the document drew, so a multi-pane document could not
        // hold a zoom on anything but its first pane.
        if x.is_some() || y.is_some() {
            self.views.insert(key, PlotView { x, y });
        } else {
            self.views.remove(&key);
        }
    }

    pub fn persistent_plot_view(&self, viewer: ResultViewer) -> PlotView {
        let Some(context) = self.persistent_pane_context else {
            return PlotView::default();
        };
        let viewer = Self::persistent_viewer_key(viewer);
        let plot = PlotPresentationKey::Document(context.document_id, context.pane_id);
        self.views
            .get(&(viewer, plot, Self::RETAINED_PANE_ORDINAL))
            .copied()
            .unwrap_or_default()
    }

    /// Flip one signal's membership in the browser's Favorites scope.
    pub fn toggle_favorite_signal(&mut self, key: SourceWaveformPresentationKey) {
        if !self.favorite_signals.remove(&key) {
            self.favorite_signals.insert(key);
        }
    }

    pub fn is_favorite_signal(&self, key: &SourceWaveformPresentationKey) -> bool {
        self.favorite_signals.contains(key)
    }

    /// Record a deliberate signal interaction for the Recent scope: front
    /// insertion, deduplicated, bounded so the scope stays a shortlist.
    pub fn note_recent_signal(&mut self, key: SourceWaveformPresentationKey) {
        const RECENT_SIGNAL_CAP: usize = 24;
        self.recent_signals.retain(|recent| recent != &key);
        self.recent_signals.insert(0, key);
        self.recent_signals.truncate(RECENT_SIGNAL_CAP);
    }

    /// Position in the Recent shortlist; `None` when never noted.
    pub fn recent_signal_rank(&self, key: &SourceWaveformPresentationKey) -> Option<usize> {
        self.recent_signals.iter().position(|recent| recent == key)
    }

    pub fn toggle_checked_result_quantity(&mut self, key: ResultBrowserSelectionKey) {
        if !self.checked_result_quantities.remove(&key) {
            self.checked_result_quantities.insert(key);
        }
    }

    pub fn is_checked_signal(&self, key: &SourceWaveformPresentationKey) -> bool {
        self.checked_result_quantities
            .contains(&ResultBrowserSelectionKey::Waveform(key.clone()))
    }

    pub fn is_checked_result_artifact(&self, key: &ResultArtifactPresentationKey) -> bool {
        self.checked_result_quantities
            .contains(&ResultBrowserSelectionKey::Artifact(key.clone()))
    }

    pub fn clear_checked_signals(&mut self) {
        self.checked_result_quantities.clear();
        self.browser_range_anchor = None;
    }

    pub fn set_browser_range_anchor(&mut self, key: ResultBrowserSelectionKey) {
        self.browser_range_anchor = Some(key);
    }

    pub fn select_checked_result_range(
        &mut self,
        target: &ResultBrowserSelectionKey,
        ordered_visible: &[ResultBrowserSelectionKey],
    ) {
        let anchor = self.browser_range_anchor.as_ref().unwrap_or(target);
        let Some(anchor_index) = ordered_visible.iter().position(|key| key == anchor) else {
            self.checked_result_quantities.insert(target.clone());
            self.browser_range_anchor = Some(target.clone());
            return;
        };
        let Some(target_index) = ordered_visible.iter().position(|key| key == target) else {
            return;
        };
        let (start, end) = if anchor_index <= target_index {
            (anchor_index, target_index)
        } else {
            (target_index, anchor_index)
        };
        self.checked_result_quantities
            .extend(ordered_visible[start..=end].iter().cloned());
    }

    pub fn select_visible_signals(&mut self, ordered_visible: &[ResultBrowserSelectionKey]) {
        self.checked_result_quantities
            .extend(ordered_visible.iter().cloned());
        self.browser_range_anchor = ordered_visible.last().cloned();
    }

    pub fn toggle_favorite_result_artifact(&mut self, key: ResultArtifactPresentationKey) {
        if !self.favorite_result_artifacts.remove(&key) {
            self.favorite_result_artifacts.insert(key);
        }
    }

    pub fn is_favorite_result_artifact(&self, key: &ResultArtifactPresentationKey) -> bool {
        self.favorite_result_artifacts.contains(key)
    }

    pub fn note_recent_result_artifact(&mut self, key: ResultArtifactPresentationKey) {
        const RECENT_ARTIFACT_CAP: usize = 24;
        self.recent_result_artifacts.retain(|recent| recent != &key);
        self.recent_result_artifacts.insert(0, key);
        self.recent_result_artifacts.truncate(RECENT_ARTIFACT_CAP);
    }

    pub fn recent_result_artifact_rank(
        &self,
        key: &ResultArtifactPresentationKey,
    ) -> Option<usize> {
        self.recent_result_artifacts
            .iter()
            .position(|recent| recent == key)
    }

    /// The zoom/pan override for a single-pane plot.
    pub fn plot_view(&self, viewer: ResultViewer, index: usize) -> PlotView {
        self.plot_view_pane_for(viewer, PlotPresentationKey::Global(index), 0)
    }

    /// The zoom/pan override for one pane of one plot.
    ///
    /// Y is per pane because each pane carries its own unit — one zoom
    /// factor across volts and amps would mean nothing.
    #[cfg(any(test, feature = "test-support"))]
    pub fn plot_view_pane(&self, viewer: ResultViewer, index: usize, pane: usize) -> PlotView {
        self.plot_view_pane_for(viewer, PlotPresentationKey::Global(index), pane)
    }

    pub fn plot_view_pane_for(
        &self,
        viewer: ResultViewer,
        plot: PlotPresentationKey,
        pane: usize,
    ) -> PlotView {
        let plot = self.scoped_plot_key(plot);
        self.views
            .get(&(viewer, plot, pane))
            .copied()
            .unwrap_or_default()
    }

    pub fn analysis_plot_view_pane(
        &self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
        pane: usize,
    ) -> PlotView {
        self.plot_view_pane_for(
            viewer,
            PlotPresentationKey::Analysis(analysis.authored()),
            pane,
        )
    }

    /// Mutable zoom/pan override for one pane of one plot.
    pub fn plot_view_pane_mut(
        &mut self,
        viewer: ResultViewer,
        index: usize,
        pane: usize,
    ) -> &mut PlotView {
        self.plot_view_pane_mut_for(viewer, PlotPresentationKey::Global(index), pane)
    }

    pub fn plot_view_pane_mut_for(
        &mut self,
        viewer: ResultViewer,
        plot: PlotPresentationKey,
        pane: usize,
    ) -> &mut PlotView {
        let plot = self.scoped_plot_key(plot);
        self.views.entry((viewer, plot, pane)).or_default()
    }

    pub fn scoped_plot_key(&self, plot: PlotPresentationKey) -> PlotPresentationKey {
        let Some(context) = self.persistent_pane_context else {
            return plot;
        };
        match plot {
            PlotPresentationKey::Global(_) => {
                PlotPresentationKey::Document(context.document_id, context.pane_id)
            }
            PlotPresentationKey::Analysis(authored) if authored == context.analysis.authored() => {
                PlotPresentationKey::Document(context.document_id, context.pane_id)
            }
            PlotPresentationKey::Analysis(_) | PlotPresentationKey::Document(_, _) => plot,
        }
    }

    /// Crate-visible because the pinned window a waveform pane holds is read
    /// back by the Export/Print adapters, which have to be able to state the
    /// key production actually writes.
    pub fn analysis_plot_view_pane_mut(
        &mut self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
        pane: usize,
    ) -> &mut PlotView {
        self.plot_view_pane_mut_for(
            viewer,
            PlotPresentationKey::Analysis(analysis.authored()),
            pane,
        )
    }

    /// Mutable zoom/pan override for a single-pane plot.
    pub fn plot_view_mut(&mut self, viewer: ResultViewer, index: usize) -> &mut PlotView {
        self.plot_view_pane_mut(viewer, index, 0)
    }

    /// Drop the zoom/pan override for one plot, every pane (FIT action).
    /// Fitting a strip fits all of it — leaving one pane zoomed would make
    /// the strip's panes disagree about the window they show.
    pub fn reset_plot_view(&mut self, viewer: ResultViewer, index: usize) {
        self.reset_plot_view_for(viewer, PlotPresentationKey::Global(index));
    }

    /// Drop every pinned viewport this sheet owns.
    ///
    /// A sheet can own several plots. Reset every ordinal so Fit does not
    /// leave any of its plots zoomed.
    pub fn reset_viewer_plot_views(&mut self, viewer: ResultViewer) {
        if let Some(context) = self.persistent_pane_context {
            let plot = PlotPresentationKey::Document(context.document_id, context.pane_id);
            self.views
                .retain(|(key_viewer, key_plot, _), _| (*key_viewer, *key_plot) != (viewer, plot));
            return;
        }
        self.views
            .retain(|(key_viewer, _, _), _| *key_viewer != viewer);
    }

    pub fn reset_plot_view_for(&mut self, viewer: ResultViewer, plot: PlotPresentationKey) {
        let plot = self.scoped_plot_key(plot);
        self.views
            .retain(|(key_viewer, key_plot, _), _| (*key_viewer, *key_plot) != (viewer, plot));
    }

    pub fn reset_analysis_plot_view(
        &mut self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
    ) {
        self.reset_plot_view_for(viewer, PlotPresentationKey::Analysis(analysis.authored()));
    }

    /// Drop every analysis-keyed viewport override of one viewer.
    pub fn reset_all_analysis_plot_views(&mut self, viewer: ResultViewer) {
        if let Some(context) = self.persistent_pane_context {
            self.reset_plot_view_for(
                viewer,
                PlotPresentationKey::Analysis(context.analysis.authored()),
            );
            return;
        }
        self.views.retain(|(key_viewer, key_plot, _), _| {
            *key_viewer != viewer || !matches!(key_plot, PlotPresentationKey::Analysis(_))
        });
    }

    /// Drop one axis' override on every pane of one strip, whatever ordinal it
    /// was stored under.
    ///
    /// The wave stack's panes share an abscissa, so an X window released on
    /// one ordinal and left on the rest is a strip whose panes disagree. Every
    /// stored ordinal is visited rather than the ones the strip currently
    /// draws, because a strip that has lost a pane keeps the departed
    /// ordinal's entry — and that entry goes on reporting the strip as zoomed
    /// with no pane left to fit it from. An entry with nothing left to say is
    /// removed, so the pinned-axis and zoomed predicates read the same answer.
    pub fn clear_analysis_plot_view_axis(
        &mut self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
        axis: PaneAxis,
    ) {
        let plot = self.scoped_plot_key(PlotPresentationKey::Analysis(analysis.authored()));
        self.views.retain(|(key_viewer, key_plot, _), view| {
            if (*key_viewer, *key_plot) != (viewer, plot) {
                return true;
            }
            match axis {
                PaneAxis::X => view.x = None,
                PaneAxis::Y => view.y = None,
            }
            view.is_zoomed()
        });
    }

    /// Whether any pane of one plot is zoomed away from the automatic view.
    #[cfg(any(test, feature = "test-support"))]
    pub fn strip_is_zoomed(&self, viewer: ResultViewer, index: usize) -> bool {
        self.plot_is_zoomed(viewer, PlotPresentationKey::Global(index))
    }

    pub fn plot_is_zoomed(&self, viewer: ResultViewer, plot: PlotPresentationKey) -> bool {
        let plot = self.scoped_plot_key(plot);
        self.views.iter().any(|((key_viewer, key_index, _), view)| {
            (*key_viewer, *key_index) == (viewer, plot) && view.is_zoomed()
        })
    }

    pub fn analysis_strip_is_zoomed(
        &self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
    ) -> bool {
        self.plot_is_zoomed(viewer, PlotPresentationKey::Analysis(analysis.authored()))
    }

    /// Whether any pane of one strip pins the given axis.
    pub fn analysis_strip_axis_is_pinned(
        &self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
        axis: PaneAxis,
    ) -> bool {
        let plot = self.scoped_plot_key(PlotPresentationKey::Analysis(analysis.authored()));
        self.views.iter().any(|((key_viewer, key_plot, _), view)| {
            (*key_viewer, *key_plot) == (viewer, plot)
                && match axis {
                    PaneAxis::X => view.x.is_some(),
                    PaneAxis::Y => view.y.is_some(),
                }
        })
    }

    /// Drop presentation state naming datasets the project no longer retains.
    ///
    /// Retention discards a dataset. Every mark, filter, expression group and
    /// memo the reader made *about* that dataset is then a statement about
    /// something that no longer exists: left in place it accumulates for the
    /// life of the session, and the project file is written with expression
    /// groups and markers that resolve against nothing when it is reopened.
    ///
    /// Two stores are deliberately absent. `views` keys a strip's window by
    /// the authored analysis rather than by one dataset, precisely so the
    /// window survives a re-run; a document-keyed window belongs to its
    /// document. `eye_timebase` keeps its prepared keys for the same reason,
    /// and only its dataset-named legacy keys are pruned.
    ///
    pub fn retain_datasets(&mut self, retained: &HashSet<DatasetId>) {
        self.remember_marker_ids();
        let live = |analysis: AnalysisPresentationKey| retained.contains(&analysis.dataset_id());
        self.markers.retain(|marker| live(marker.analysis));
        self.log_y_panes.retain(|pane| live(pane.analysis));
        self.analysis_exprs.retain(|analysis, _| live(*analysis));
        self.expr_projection_keys
            .retain(|_, analysis| live(*analysis));
        self.hidden_strips.retain(|analysis| live(*analysis));
        self.maximized_strip = self.maximized_strip.filter(|analysis| live(*analysis));
        self.favorite_signals.retain(|key| live(key.analysis()));
        self.recent_signals.retain(|key| live(key.analysis()));
        self.favorite_result_artifacts
            .retain(|key| live(key.analysis()));
        self.recent_result_artifacts
            .retain(|key| live(key.analysis()));
        self.checked_result_quantities
            .retain(|key| retained.contains(&key.dataset_id()));
        self.browser_range_anchor = self
            .browser_range_anchor
            .take()
            .filter(|key| retained.contains(&key.dataset_id()));
        self.waveform_visibility
            .retain(|key, _| live(key.analysis()));
        self.hidden_family_traces.retain(|key| live(key.analysis()));
        self.eye_timebase.retain(|key, _| match key {
            crate::eye_diagram::EyeTimebaseKey::Prepared(_) => true,
            crate::eye_diagram::EyeTimebaseKey::Legacy(dataset, _) => retained.contains(dataset),
        });
        self.selected_trace = self
            .selected_trace
            .take()
            .filter(|selected| live(selected.analysis_key()));
        self.selected_result_artifact = self
            .selected_result_artifact
            .take()
            .filter(|key| live(key.analysis()));
    }
}

#[cfg(test)]
mod tests;
