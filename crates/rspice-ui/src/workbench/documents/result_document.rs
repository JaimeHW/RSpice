//! The Results workspace.
//!
//! A 41 px dataset-viewer tab strip, a distinct 31 px plot instrument or
//! purpose strip, a fill-height document well carrying the active viewer
//! (waveform strips, Bode, FFT, eye, histogram, and the
//! Nyquist/Smith/pole-zero diagnostics), and a content-fit readout strip.

mod bode;
mod create_document;
mod eye;
mod fft;
mod hist;
pub(crate) mod manifest;
mod noise_contrib;
mod nyquist;
mod op_inspector;
mod persistent_document;
mod pz;
mod sensitivity;
mod smith;
mod specs;
mod table;
mod transfer_function;

pub(crate) fn open_specification_editor(state: &mut AppState) {
    specs::open_editor(state);
}

/// Open the dataset/manifest browser in its canonical Results frame.
///
/// The navigator and inspector are parts of that frame; this command exposes
/// them instead of creating a second dataset browser with independent state.
pub(crate) fn open_dataset_browser(app: &mut RSpiceApp) {
    app.state.workbench.activate(Workspace::Results);
    app.state.workbench.navigator_visible = true;
    app.state.workbench.inspector_visible = true;
    app.state.workbench.focus_navigator_search = true;
    app.state.ui.results.viewer = ResultViewer::Manifest;
}

/// Open a fresh immutable-dataset-bound result-document transaction.
pub(crate) fn open_create_document(app: &mut RSpiceApp) {
    create_document::open(app);
}

mod strip;
mod waves;
pub(crate) use waves::copy_cursor_text;

pub(crate) use waves::toggle_visibility;

use std::collections::HashSet;

use egui::{Ui, WidgetInfo, WidgetType};
use serde::{Deserialize, Serialize};

use super::visualization_family::SourceSampleSelection;
use crate::product::{AnalysisInstanceId, DatasetId};
use crate::simulation::SimulationController;
use crate::simulation::controller::DerivedViewerLoadState;
use crate::state::{AnalysisResult, SharedWaveformValues, SimulationRun, WaveformData};
use crate::ui::plot::{CursorPair, DecimationCache, InteractionMode};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{chip, docbar_at_height};
use crate::workbench::app_state::ActiveViewer;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::state::Workspace;
use crate::workbench::{AppState, RSpiceApp};

pub type WaveformSeries = (SharedWaveformValues, SharedWaveformValues);
pub type WaveformSeriesResult = Result<WaveformSeries, String>;
type WindowStatsKey = (u64, u64, u64);
type WindowStats = Option<(f64, f64, f64)>;

/// Stable identity of one retained analysis within one immutable dataset.
///
/// Current results use the exact prepared-task identity. A legacy result has
/// no such provenance, so its run-local analysis id is safe only when paired
/// with the immutable dataset id. Neither representation depends on vector
/// position, which prevents presentation state from moving to another
/// analysis when retained results are reordered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct AnalysisPresentationKey {
    dataset_id: DatasetId,
    source: AnalysisPresentationSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum AnalysisPresentationSource {
    Prepared(AnalysisInstanceId),
    Legacy(u64),
}

impl AnalysisPresentationKey {
    pub(crate) fn new(dataset_id: DatasetId, analysis: &AnalysisResult) -> Self {
        let source = analysis.provenance().map_or(
            AnalysisPresentationSource::Legacy(analysis.id),
            |provenance| AnalysisPresentationSource::Prepared(provenance.source_instance_id()),
        );
        Self { dataset_id, source }
    }

    pub(crate) const fn dataset_id(self) -> DatasetId {
        self.dataset_id
    }

    pub(crate) fn resolve(self, run: &SimulationRun) -> Option<(usize, &AnalysisResult)> {
        (run.dataset_id == self.dataset_id)
            .then(|| {
                run.analyses
                    .iter()
                    .enumerate()
                    .find(|(_, analysis)| Self::new(run.dataset_id, analysis) == self)
            })
            .flatten()
    }
}

/// Stable identity of one source waveform representation inside an analysis.
///
/// `source_name` follows the retained waveform through reordering. `kind` and
/// `family_group` distinguish real/imaginary, magnitude/phase, and projected
/// family traces that intentionally share the same source waveform.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct TracePresentationKey {
    source_name: String,
    kind: u8,
    family_group: u64,
}

/// Fully dataset-bound identity of one presented waveform.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct WaveformPresentationKey {
    analysis: AnalysisPresentationKey,
    trace: TracePresentationKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PlotPresentationKey {
    Global(usize),
    Analysis(AnalysisPresentationKey),
}

/// The result viewers, in tab order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ResultViewer {
    /// Stacked waveform strips, one per analysis.
    #[default]
    Waves,
    /// Loop-gain stability view with margin markers.
    Bode,
    /// Spectrum with harmonic markers.
    Fft,
    /// Eye diagram with compliance mask.
    Eye,
    /// Monte-Carlo distribution.
    Hist,
    /// Per-device operating-point inspector (Spectre-style OP info).
    Op,
    /// Ranked band-integrated noise contributors.
    NoiseContrib,
    /// Ranked signed parameter-sensitivity contributions.
    Contribution,
    /// Scalar DC transfer gain and input/output resistances.
    TransferFunction,
    /// Measurements × runs matrix against spec bounds.
    Specs,
    /// The retained samples of one analysis, as rows.
    Table,
    /// Legacy Nyquist surface (pre-redesign chrome).
    Nyquist,
    /// Legacy Smith chart surface.
    Smith,
    /// Legacy pole-zero surface.
    PoleZero,
    /// Immutable task and retained-value inventory for the active dataset.
    ///
    /// This is dataset-native and deliberately has no Visualization Studio
    /// viewer-document identity.
    Manifest,
}

impl ResultViewer {
    /// Compact command/status label.
    pub fn label(self) -> &'static str {
        match self {
            ResultViewer::Waves => "WAVES",
            ResultViewer::Bode => "BODE",
            ResultViewer::Fft => "FFT",
            ResultViewer::Eye => "EYE",
            ResultViewer::Hist => "HIST",
            ResultViewer::Op => "OP",
            ResultViewer::NoiseContrib => "NOISE",
            ResultViewer::Contribution => "SENS",
            ResultViewer::TransferFunction => "XF",
            ResultViewer::Specs => "SPECS",
            ResultViewer::Table => "TABLE",
            ResultViewer::Nyquist => "NYQ",
            ResultViewer::Smith => "SMITH",
            ResultViewer::PoleZero => "PZ",
            ResultViewer::Manifest => "MANIFEST",
        }
    }

    const PRIMARY: [ResultViewer; 11] = [
        ResultViewer::Waves,
        ResultViewer::Bode,
        ResultViewer::Fft,
        ResultViewer::Eye,
        ResultViewer::Hist,
        ResultViewer::Op,
        ResultViewer::NoiseContrib,
        ResultViewer::Contribution,
        ResultViewer::TransferFunction,
        ResultViewer::Specs,
        ResultViewer::Table,
    ];
    const LEGACY: [ResultViewer; 3] = [
        ResultViewer::Nyquist,
        ResultViewer::Smith,
        ResultViewer::PoleZero,
    ];
    const DATASET_NATIVE: [ResultViewer; 1] = [ResultViewer::Manifest];

    /// Human-readable document-tab label from the upgraded Results mockup.
    const fn tab_label(self) -> &'static str {
        match self {
            ResultViewer::Waves => "Waves",
            ResultViewer::Bode => "Bode",
            ResultViewer::Fft => "FFT",
            ResultViewer::Eye => "Eye",
            ResultViewer::Hist => "Histogram",
            ResultViewer::Op => "OP",
            ResultViewer::NoiseContrib => "Noise",
            ResultViewer::Contribution => "Sensitivity",
            ResultViewer::TransferFunction => "XF",
            ResultViewer::Specs => "Specs",
            ResultViewer::Table => "Table",
            ResultViewer::Nyquist => "Nyquist",
            ResultViewer::Smith => "Smith",
            ResultViewer::PoleZero => "PZ",
            ResultViewer::Manifest => "Manifest",
        }
    }

    const fn tab_icon(self) -> WorkbenchIcon {
        match self {
            ResultViewer::Waves | ResultViewer::NoiseContrib => WorkbenchIcon::Results,
            ResultViewer::Bode
            | ResultViewer::Fft
            | ResultViewer::Hist
            | ResultViewer::Contribution => WorkbenchIcon::Results,
            ResultViewer::Eye
            | ResultViewer::Nyquist
            | ResultViewer::Smith
            | ResultViewer::PoleZero => WorkbenchIcon::Target,
            ResultViewer::Op
            | ResultViewer::TransferFunction
            | ResultViewer::Specs
            | ResultViewer::Table => WorkbenchIcon::Grid,
            ResultViewer::Manifest => WorkbenchIcon::Layers,
        }
    }
}

/// Mutually exclusive primary pointer tool for instrument-style result plots.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum ResultPlotTool {
    /// Normal result interaction: cursors/markers plus wheel zoom and panning.
    #[default]
    Cursor,
    /// Primary drag draws a zoom rectangle.
    BoxZoom,
    /// Primary drag pans the plot viewport.
    Pan,
    /// Primary click or drag places the active pane's horizontal cursor.
    HorizontalCursor,
}

impl ResultPlotTool {
    const fn interaction_mode(self) -> InteractionMode {
        match self {
            Self::Cursor => InteractionMode::All,
            Self::BoxZoom => InteractionMode::Zoom,
            Self::Pan => InteractionMode::Pan,
            Self::HorizontalCursor => InteractionMode::Select,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ViewerAvailability {
    available: bool,
    reason: &'static str,
}

impl ViewerAvailability {
    const fn available(reason: &'static str) -> Self {
        Self {
            available: true,
            reason,
        }
    }

    const fn unavailable(reason: &'static str) -> Self {
        Self {
            available: false,
            reason,
        }
    }
}

/// User zoom/pan override for one plot. `None` per axis means automatic
/// fit-to-data; gestures in the plot engine populate the ranges and a
/// double-click (or the strip's FIT action) clears them.
#[derive(Debug, Clone, Copy, Default)]
pub struct PlotView {
    /// X range override (data space).
    pub x: Option<(f64, f64)>,
    /// Left-Y range override.
    pub y: Option<(f64, f64)>,
    /// Right-Y range override (dual-axis strips).
    pub y_right: Option<(f64, f64)>,
}

impl PlotView {
    /// Whether any axis is zoomed away from the automatic view.
    pub fn is_zoomed(&self) -> bool {
        self.x.is_some() || self.y.is_some() || self.y_right.is_some()
    }

    /// Fold one frame's gesture result into the override.
    pub fn apply(&mut self, change: &crate::ui::plot::ViewChange) {
        if change.reset {
            *self = Self::default();
            return;
        }
        if let Some(x) = change.x {
            self.x = Some(x);
        }
        if let Some(y) = change.y {
            self.y = Some(y);
        }
        if let Some(y_right) = change.y_right {
            self.y_right = Some(y_right);
        }
    }
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

/// What a result marker asserts, and therefore how it draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MarkerKind {
    /// A freeform annotation on a sample.
    #[default]
    Note,
    /// A called-out extremum or feature of the curve.
    Peak,
    /// A limit the design is measured against. Drawn as a limit line,
    /// because a spec constrains the axis position, not one curve.
    Spec,
}

impl MarkerKind {
    /// Every kind, in cycle order.
    #[cfg(test)]
    pub const ALL: [MarkerKind; 3] = [MarkerKind::Note, MarkerKind::Peak, MarkerKind::Spec];

    /// Short label used on the chip and in the marker list.
    pub const fn label(self) -> &'static str {
        match self {
            MarkerKind::Note => "note",
            MarkerKind::Peak => "peak",
            MarkerKind::Spec => "spec",
        }
    }

    /// The next kind in the cycle (the marker row's kind control).
    pub const fn next(self) -> Self {
        match self {
            MarkerKind::Note => MarkerKind::Peak,
            MarkerKind::Peak => MarkerKind::Spec,
            MarkerKind::Spec => MarkerKind::Note,
        }
    }

    /// A spec marker constrains the X position alone and so carries no
    /// trace value in the readout.
    pub const fn rides_a_trace(self) -> bool {
        !matches!(self, MarkerKind::Spec)
    }
}

/// A user-placed marker on a waveform strip.
///
/// The anchor is a *signal*, not one solve of it: `y` is resampled from the
/// trace every frame, so a marker survives zoom, pan, and retained-vector
/// reordering without drifting onto a different dataset or curve.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultMarker {
    /// Stable per-session id. Renders as `M{id}`.
    pub id: u32,
    /// Dataset-bound identity of the strip this marker lives on.
    pub analysis: AnalysisPresentationKey,
    /// Dataset-bound identity of the trace the marker rides.
    pub anchor: WaveformPresentationKey,
    /// Display name of the anchored trace, for the marker list.
    pub trace_name: String,
    /// Anchor position in the strip's X data space.
    pub x: f64,
    pub kind: MarkerKind,
    /// Free text shown after the id on the tag. May be empty.
    pub note: String,
}

/// Stable identity of one unit-scoped waveform pane.
///
/// The analysis key retains the exact dataset identity; the unit is the pane
/// grouping contract and remains independent of transient pane ordinals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct WavePanePresentationKey {
    pub analysis: AnalysisPresentationKey,
    pub unit: String,
}

/// One horizontal measurement cursor bound to an exact waveform pane.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct HorizontalWaveCursor {
    pub pane: WavePanePresentationKey,
    pub y: f64,
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

/// Per-session results-workspace state. Only the viewer selection persists;
/// caches and cursors are transient.
#[derive(Debug, Clone, Default)]
pub struct ResultsState {
    /// Active viewer tab.
    pub viewer: ResultViewer,
    /// Device-local page selection for each project-owned result document.
    ///
    /// Page selection is presentation state, not part of the immutable
    /// visualization document. Stable document and page identities prevent a
    /// project reload or page reorder from selecting a different page.
    persistent_document_pages: std::collections::HashMap<
        crate::product::ResultDocumentId,
        crate::results::visualization_document::PageId,
    >,
    /// The A│B cursor tool.
    pub cursor_tool: CursorTool,
    /// The marker tool.
    pub marker_tool: MarkerTool,
    /// Primary plot pointer tool shown in the 31 px instrument strip.
    plot_tool: ResultPlotTool,
    /// Exact retained waveform selected from a trace chip. This is only an
    /// identity into the canonical result dataset; it never copies samples or
    /// creates a second result owner.
    pub(crate) selected_trace: Option<SelectedResultTrace>,
    /// User-placed markers across every waveform strip.
    pub markers: Vec<ResultMarker>,
    /// Id allocator for `markers`. Monotonic within a session so a marker
    /// label never silently changes meaning after a deletion.
    next_marker_id: u32,
    /// Marker whose note is being edited in the readout strip, if any.
    pub editing_marker: Option<u32>,
    /// A/B cursors (data-space X of the strip they live on).
    pub cursors: CursorPair,
    /// Dataset-bound strip identity the cursors were placed on.
    pub cursor_strip: Option<usize>,
    /// Exact visible trace nearest cursor A when A was placed.
    pub(super) cursor_a_anchor: Option<WaveformPresentationKey>,
    /// Unit-scoped waveform pane receiving instrument actions.
    pub(super) active_wave_pane: Option<WavePanePresentationKey>,
    /// Horizontal cursor, bound to the pane whose Y axis owns its value.
    pub(super) horizontal_cursor: Option<HorizontalWaveCursor>,
    /// Draw exact project specification bounds compatible with visible axes.
    pub(super) show_spec_limits: bool,
    /// Draw derived min/max curves only when retained family samples exist.
    pub(super) show_family_envelope: bool,
    /// Draw unlabeled minor subdivisions between authoritative major ticks.
    pub(super) show_minor_grid: bool,
    /// Share the same A/B cursor positions across every compatible waveform
    /// strip instead of scoping them to `cursor_strip`.
    pub linked_cursors: bool,
    /// Decimation envelope cache.
    pub cache: DecimationCache,
    /// Derived dB/phase series cache.
    pub derived: DerivedSeries,
    /// Fingerprint-keyed strip-model cache for the waves viewer.
    models: waves::ModelsCache,
    /// Presentation-only exact family rows selected by Visualization Studio.
    /// Source datasets are never modified by this projection.
    pub(crate) sample_selection: Option<SourceSampleSelection>,
    /// Session-only visibility overrides for exact family group traces. These
    /// are presentation state and never alter source WaveformData visibility.
    hidden_family_traces: HashSet<waves::FamilyTraceVisibilityKey>,
    /// Strips hidden via the strip-close action.
    pub hidden_strips: HashSet<AnalysisPresentationKey>,
    /// Strip currently maximized via the strip action, if any.
    pub maximized_strip: Option<AnalysisPresentationKey>,
    /// Cached FFT display arrays for the active spectrum revision.
    pub fft_series: Option<FftSeries>,
    /// Cached Bode margins + extremes for the active data version.
    pub bode: Option<BodeDerived>,
    /// Cached Nyquist stability numbers for the active data version.
    pub nyquist: Option<nyquist::NyquistDerived>,
    /// Baked EYE density texture for the active eye revision and size.
    pub eye_texture: Option<EyeTexture>,
    /// `simulation.data_version` last seen by the workspace; when it
    /// advances, cursors are cleared so they never report stale data.
    seen_version: u64,
    /// Zoom/pan overrides per plot, keyed by stable plot identity. Survives
    /// re-runs on purpose — keeping the zoomed window across parameter
    /// tweaks is how engineers compare iterations.
    pub views: std::collections::HashMap<(ResultViewer, PlotPresentationKey, usize), PlotView>,
    /// User expression traces per dataset-bound waves strip, evaluated by
    /// the calculator against that analysis' waveforms.
    /// Compatibility projection for integrations that still address the
    /// active run by ordinal. The Results document migrates these entries
    /// into `analysis_exprs` before use; stable state lives there.
    pub exprs: std::collections::HashMap<usize, Vec<ExprTrace>>,
    pub(crate) analysis_exprs: std::collections::HashMap<AnalysisPresentationKey, Vec<ExprTrace>>,
    /// Identity map behind the ordinal compatibility projection.
    expr_projection_keys: std::collections::HashMap<usize, AnalysisPresentationKey>,
    /// The inline expression editor, when open (one strip at a time).
    pub expr_editor: Option<ExprEditor>,
    /// Evaluated expression series, keyed by (stable analysis, expression);
    /// refreshed when the simulation data version advances.
    pub expr_cache: std::collections::HashMap<(usize, String), ExprSeries>,
    pub(crate) analysis_expr_cache:
        std::collections::HashMap<(AnalysisPresentationKey, String), ExprSeries>,
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
    /// Open spec-editor rows (None = matrix view). Transient.
    pub spec_drafts: Option<Vec<specs::SpecDraft>>,
    /// Row/column selection for the TABLE viewer.
    pub table: table::TableView,
    /// Last row count the table rendered, as its footer states it. Written
    /// by the viewer so the docbar reports what is actually on screen
    /// rather than recomputing a second, possibly different, answer.
    pub table_status: Option<String>,
}

/// Stable session identity for a selected retained waveform.
///
/// The immutable dataset, prepared analysis identity, and source waveform
/// name survive retained-result reordering. Duplicate source names fail
/// closed instead of silently inheriting an old ordinal selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SelectedResultTrace {
    waveform: WaveformPresentationKey,
}

impl SelectedResultTrace {
    pub(crate) fn from_identity(
        analysis: AnalysisPresentationKey,
        source_name: impl Into<String>,
    ) -> Self {
        Self {
            waveform: WaveformPresentationKey {
                analysis,
                trace: TracePresentationKey {
                    source_name: source_name.into(),
                    kind: 0,
                    family_group: 0,
                },
            },
        }
    }

    pub(crate) fn from_run_indices(
        run: &SimulationRun,
        analysis_index: usize,
        waveform_index: usize,
    ) -> Option<Self> {
        let analysis = run.analyses.get(analysis_index)?;
        let waveform = analysis.waveforms.get(waveform_index)?;
        Some(Self::from_identity(
            AnalysisPresentationKey::new(run.dataset_id, analysis),
            waveform.name.clone(),
        ))
    }

    pub(crate) const fn analysis_key(&self) -> AnalysisPresentationKey {
        self.waveform.analysis
    }

    pub(crate) fn source_name(&self) -> &str {
        &self.waveform.trace.source_name
    }

    pub(crate) const fn dataset_id(&self) -> DatasetId {
        self.waveform.analysis.dataset_id()
    }

    pub(crate) fn resolve<'a>(
        &self,
        run: &'a SimulationRun,
    ) -> Option<(usize, usize, &'a AnalysisResult, &'a WaveformData)> {
        let (analysis_index, analysis) = self.waveform.analysis.resolve(run)?;
        let mut matching = analysis
            .waveforms
            .iter()
            .enumerate()
            .filter(|(_, waveform)| waveform.name == self.waveform.trace.source_name);
        let (waveform_index, waveform) = matching.next()?;
        matching
            .next()
            .is_none()
            .then_some((analysis_index, waveform_index, analysis, waveform))
    }
}

/// One user expression trace on a waves strip.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExprTrace {
    /// The calculator expression as typed ("V(out)/V(in)").
    pub text: String,
    /// Legend-chip visibility.
    #[serde(default = "default_visible")]
    pub visible: bool,
}

fn default_visible() -> bool {
    true
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

/// One evaluated expression series (owned arrays, cheap to clone).
#[derive(Debug, Clone)]
pub struct ExprSeries {
    /// `simulation.data_version` the series was computed against.
    pub version: u64,
    /// Evaluation result, or the error to show on the strip.
    pub series: WaveformSeriesResult,
}

impl ResultsState {
    /// Return the selection only while it resolves to the exact active
    /// retained dataset and source waveform that created it.
    pub(crate) fn valid_selected_trace<'a>(
        &'a self,
        simulation: &crate::state::SimulationState,
    ) -> Option<&'a SelectedResultTrace> {
        let selected = self.selected_trace.as_ref()?;
        let run = simulation.active_run()?;
        selected.resolve(run).map(|_| selected)
    }

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
    pub(super) fn cursor_placement_enabled(&self) -> bool {
        self.plot_tool == ResultPlotTool::Cursor && self.cursor_tool.is_armed()
    }

    pub(super) fn horizontal_cursor_placement_enabled(&self) -> bool {
        self.plot_tool == ResultPlotTool::HorizontalCursor
    }

    pub(super) fn cursor_a_is_next(&self) -> bool {
        self.cursors.a.is_none() || self.cursors.b.is_some()
    }

    pub fn toggle_linked_cursors(&mut self) {
        self.linked_cursors = !self.linked_cursors;
    }

    /// Arm or disarm the marker tool. Disarming keeps the markers — they are
    /// document content, not a transient readout like the A/B pair.
    pub fn toggle_marker_tool(&mut self) {
        self.marker_tool = MarkerTool(!self.marker_tool.is_armed());
        if !self.marker_tool.is_armed() {
            self.editing_marker = None;
        }
    }

    /// Place a marker and return its id.
    pub fn add_marker(
        &mut self,
        analysis: AnalysisPresentationKey,
        anchor: WaveformPresentationKey,
        trace_name: String,
        x: f64,
    ) -> u32 {
        self.next_marker_id += 1;
        let id = self.next_marker_id;
        self.markers.push(ResultMarker {
            id,
            analysis,
            anchor,
            trace_name,
            x,
            kind: MarkerKind::default(),
            note: String::new(),
        });
        id
    }

    pub fn marker_mut(&mut self, id: u32) -> Option<&mut ResultMarker> {
        self.markers.iter_mut().find(|marker| marker.id == id)
    }

    /// Remove one marker, and with it any note edit it owned.
    pub fn remove_marker(&mut self, id: u32) {
        self.markers.retain(|marker| marker.id != id);
        if self.editing_marker == Some(id) {
            self.editing_marker = None;
        }
    }

    /// Markers on one strip, in placement order.
    pub fn strip_markers(
        &self,
        analysis: AnalysisPresentationKey,
    ) -> impl Iterator<Item = &ResultMarker> {
        self.markers
            .iter()
            .filter(move |marker| marker.analysis == analysis)
    }

    fn set_sample_selection(&mut self, selection: Option<SourceSampleSelection>) {
        let current = self
            .sample_selection
            .as_ref()
            .map(SourceSampleSelection::fingerprint);
        let next = selection.as_ref().map(SourceSampleSelection::fingerprint);
        if current != next {
            self.models = waves::ModelsCache::default();
            self.expr_cache.clear();
            self.analysis_expr_cache.clear();
            self.cache = DecimationCache::default();
            self.derived = DerivedSeries::default();
            self.clear_cursors();
            self.hidden_family_traces.clear();
        }
        self.sample_selection = selection;
    }

    fn toggle_family_trace_visibility(&mut self, key: waves::FamilyTraceVisibilityKey) {
        if !self.hidden_family_traces.insert(key) {
            self.hidden_family_traces.remove(&key);
        }
        self.models = waves::ModelsCache::default();
        self.cache = DecimationCache::default();
        self.derived = DerivedSeries::default();
        self.clear_cursors();
    }

    fn reconcile_expression_projection(&mut self, simulation: &crate::state::SimulationState) {
        let Some(run) = simulation.active_run() else {
            self.exprs.clear();
            self.expr_projection_keys.clear();
            return;
        };
        let current = run
            .analyses
            .iter()
            .enumerate()
            .map(|(index, analysis)| {
                (
                    index,
                    AnalysisPresentationKey::new(run.dataset_id, analysis),
                )
            })
            .collect::<Vec<_>>();

        // An ordinal row may be imported only while it still names the
        // identity it named when this projection was emitted. A reorder
        // therefore cannot relabel an expression as belonging to the new
        // occupant of the same slot.
        for (index, key) in &current {
            let may_import = self
                .expr_projection_keys
                .get(index)
                .is_none_or(|projected| projected == key);
            if may_import && let Some(projected) = self.exprs.get(index) {
                self.analysis_exprs.insert(*key, projected.clone());
            }
        }
        self.exprs = current
            .iter()
            .filter_map(|(index, key)| {
                self.analysis_exprs
                    .get(key)
                    .cloned()
                    .map(|exprs| (*index, exprs))
            })
            .collect();
        self.expr_projection_keys = current.into_iter().collect();
    }

    fn sync_expression_projection(
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

    /// Clear result UI state that is tied to the active project/design data.
    pub fn clear_project_scoped_state(&mut self) {
        let viewer = self.viewer;
        let phase_continuous = self.phase_continuous;
        *self = Self {
            viewer,
            phase_continuous,
            ..Self::default()
        };
    }

    pub(crate) fn persistent_document_page(
        &self,
        document_id: crate::product::ResultDocumentId,
    ) -> Option<crate::results::visualization_document::PageId> {
        self.persistent_document_pages.get(&document_id).copied()
    }

    fn select_persistent_document_page(
        &mut self,
        document_id: crate::product::ResultDocumentId,
        page_id: crate::results::visualization_document::PageId,
    ) {
        self.persistent_document_pages.insert(document_id, page_id);
    }

    /// The zoom/pan override for a single-pane plot.
    pub fn plot_view(&self, viewer: ResultViewer, index: usize) -> PlotView {
        self.plot_view_pane_for(viewer, PlotPresentationKey::Global(index), 0)
    }

    /// The zoom/pan override for one pane of one plot.
    ///
    /// Y is per pane because each pane carries its own unit — one zoom
    /// factor across volts and amps would mean nothing.
    #[cfg(test)]
    pub fn plot_view_pane(&self, viewer: ResultViewer, index: usize, pane: usize) -> PlotView {
        self.plot_view_pane_for(viewer, PlotPresentationKey::Global(index), pane)
    }

    fn plot_view_pane_for(
        &self,
        viewer: ResultViewer,
        plot: PlotPresentationKey,
        pane: usize,
    ) -> PlotView {
        self.views
            .get(&(viewer, plot, pane))
            .copied()
            .unwrap_or_default()
    }

    pub(super) fn analysis_plot_view_pane(
        &self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
        pane: usize,
    ) -> PlotView {
        self.plot_view_pane_for(viewer, PlotPresentationKey::Analysis(analysis), pane)
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

    fn plot_view_pane_mut_for(
        &mut self,
        viewer: ResultViewer,
        plot: PlotPresentationKey,
        pane: usize,
    ) -> &mut PlotView {
        self.views.entry((viewer, plot, pane)).or_default()
    }

    pub(super) fn analysis_plot_view_pane_mut(
        &mut self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
        pane: usize,
    ) -> &mut PlotView {
        self.plot_view_pane_mut_for(viewer, PlotPresentationKey::Analysis(analysis), pane)
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

    fn reset_plot_view_for(&mut self, viewer: ResultViewer, plot: PlotPresentationKey) {
        self.views
            .retain(|(key_viewer, key_plot, _), _| (*key_viewer, *key_plot) != (viewer, plot));
    }

    pub(super) fn reset_analysis_plot_view(
        &mut self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
    ) {
        self.reset_plot_view_for(viewer, PlotPresentationKey::Analysis(analysis));
    }

    pub(super) fn reset_analysis_plot_view_pane(
        &mut self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
        pane: usize,
    ) {
        self.views
            .remove(&(viewer, PlotPresentationKey::Analysis(analysis), pane));
    }

    /// Reset every pane and plot owned by one viewer (the viewer-wide FIT/F
    /// action). This deliberately does not assume a single plot at slot zero.
    #[cfg(test)]
    pub(crate) fn reset_viewer_plot_views(&mut self, viewer: ResultViewer) {
        self.views
            .retain(|(key_viewer, _, _), _| *key_viewer != viewer);
    }

    /// Whether any pane of one plot is zoomed away from the automatic view.
    #[cfg(test)]
    pub fn strip_is_zoomed(&self, viewer: ResultViewer, index: usize) -> bool {
        self.plot_is_zoomed(viewer, PlotPresentationKey::Global(index))
    }

    fn plot_is_zoomed(&self, viewer: ResultViewer, plot: PlotPresentationKey) -> bool {
        self.views.iter().any(|((key_viewer, key_index, _), view)| {
            (*key_viewer, *key_index) == (viewer, plot) && view.is_zoomed()
        })
    }

    pub(super) fn analysis_strip_is_zoomed(
        &self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
    ) -> bool {
        self.plot_is_zoomed(viewer, PlotPresentationKey::Analysis(analysis))
    }

    /// Restore a strip addressed by the current run's transient ordinal.
    /// Command/search integrations may still produce an ordinal, but the
    /// retained presentation state is removed by its stable identity.
    pub(crate) fn restore_analysis_strip(
        &mut self,
        simulation: &crate::state::SimulationState,
        analysis_index: usize,
    ) {
        let Some(run) = simulation.active_run() else {
            return;
        };
        let Some(analysis) = run.analyses.get(analysis_index) else {
            return;
        };
        self.hidden_strips
            .remove(&AnalysisPresentationKey::new(run.dataset_id, analysis));
    }
}

/// Frequency/magnitude-dB arrays derived from the active FFT, keyed on the
/// FFT state's spectrum revision — the FFT can be recomputed (window, size,
/// source) without a simulation data-version bump, and keying on the data's
/// allocation address could serve stale arrays after a reallocation.
#[derive(Debug, Clone)]
pub struct FftSeries {
    pub(crate) revision: u64,
    pub(crate) frequency: std::sync::Arc<[f64]>,
    pub(crate) magnitude_db: std::sync::Arc<[f64]>,
    /// Cached finite (lo, hi) of the magnitude within view: (x1 bits, lo, hi).
    pub(crate) y_extremes: Option<(u64, f64, f64)>,
}

/// GPU density image for the EYE viewer, baked once per (data revision,
/// plot size, trace color) — per frame the eye then costs one textured
/// quad instead of restroking every folded acquisition.
#[derive(Clone)]
pub struct EyeTexture {
    pub(crate) revision: u64,
    pub(crate) size: [usize; 2],
    pub(crate) color: egui::Color32,
    pub(crate) handle: egui::TextureHandle,
}

impl std::fmt::Debug for EyeTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EyeTexture")
            .field("revision", &self.revision)
            .field("size", &self.size)
            .finish_non_exhaustive()
    }
}

/// Stability numbers and axis extremes computed from the active AC curves.
/// BODE's center view and right panel share one cached compute, keyed on
/// the data version and the resolved magnitude waveform.
#[derive(Debug, Clone, Copy)]
pub struct BodeDerived {
    pub(crate) version: u64,
    pub(crate) analysis_index: usize,
    pub(crate) mag_index: usize,
    /// DC (lowest-frequency) gain in dB.
    pub(crate) adc_db: Option<f64>,
    /// Unity-gain frequency (Hz).
    pub(crate) ugf: Option<f64>,
    /// Phase margin (deg) at the UGF.
    pub(crate) pm_deg: Option<f64>,
    /// Frequency where phase crosses −180° (Hz).
    pub(crate) f180: Option<f64>,
    /// Gain margin (dB) at f180.
    pub(crate) gm_db: Option<f64>,
    /// −3 dB bandwidth (Hz).
    pub(crate) f3db: Option<f64>,
    /// Finite (min, max) of the gain curve.
    pub(crate) gain_extremes: (f64, f64),
    /// Finite (min, max) of the phase curve, when phase data exists.
    pub(crate) phase_extremes: Option<(f64, f64)>,
}

/// Finite (min, max) of a slice, if any finite values exist.
pub(super) fn finite_extremes(values: &[f64]) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for &v in values {
        if v.is_finite() {
            lo = lo.min(v);
            hi = hi.max(v);
        }
    }
    (lo <= hi).then_some((lo, hi))
}

/// Cache of series derived from waveform data (dB conversions), cleared when
/// the simulation data version changes.
#[derive(Debug, Clone, Default)]
pub struct DerivedSeries {
    version: u64,
    map: std::collections::HashMap<u64, SharedWaveformValues>,
    /// Cached finite (min, max) per series key — axis autoranges must not
    /// rescan millions of samples per frame.
    ranges: std::collections::HashMap<u64, Option<(f64, f64)>>,
    /// Cached windowed (min, max, rms) measurements, keyed by
    /// (series key, window-start bits, window-end bits).
    stats: std::collections::HashMap<WindowStatsKey, WindowStats>,
}

impl DerivedSeries {
    fn ensure_version(&mut self, version: u64) {
        if self.version != version {
            self.map.clear();
            self.ranges.clear();
            self.stats.clear();
            self.version = version;
        }
    }

    /// Fetch or compute the cached finite (min, max) of a series.
    pub fn range_or(
        &mut self,
        key: u64,
        build: impl FnOnce() -> Option<(f64, f64)>,
    ) -> Option<(f64, f64)> {
        *self.ranges.entry(key).or_insert_with(build)
    }

    /// Fetch or compute cached windowed (min, max, rms) measurements.
    pub fn stats_or(
        &mut self,
        key: (u64, u64, u64),
        build: impl FnOnce() -> Option<(f64, f64, f64)>,
    ) -> Option<(f64, f64, f64)> {
        *self.stats.entry(key).or_insert_with(build)
    }

    /// Fetch or build a derived series under `key`.
    pub fn get_or(
        &mut self,
        key: u64,
        build: impl FnOnce() -> SharedWaveformValues,
    ) -> SharedWaveformValues {
        if let Some(hit) = self.map.get(&key) {
            return std::sync::Arc::clone(hit);
        }
        let series = build();
        self.map.insert(key, std::sync::Arc::clone(&series));
        series
    }

    /// 20·log₁₀ of a linear-magnitude series, cached under `key`.
    pub fn db(&mut self, key: u64, magnitude: &[f64]) -> SharedWaveformValues {
        self.get_or(key, || {
            std::sync::Arc::new(
                magnitude
                    .iter()
                    .map(|&m| 20.0 * m.max(1e-30).log10())
                    .collect::<Vec<_>>(),
            )
        })
    }

    /// Key-space bit separating power-density dB entries from ordinary
    /// magnitude-dB entries. Noise spectra remain retained as exact power
    /// densities; this cache is presentation-only.
    const POWER_DB_KEY_BIT: u64 = 1 << 61;

    /// Ten times log10 of a non-negative power-density series.
    ///
    /// Zero has no finite logarithm and is therefore a gap in the derived
    /// curve. Cursor readouts continue to report the exact retained linear
    /// value; no artificial display floor becomes result evidence.
    pub fn db_power(&mut self, key: u64, power: &[f64]) -> SharedWaveformValues {
        self.get_or(Self::POWER_DB_KEY_BIT | key, || {
            std::sync::Arc::new(
                power
                    .iter()
                    .map(|&value| {
                        if value.is_finite() && value > 0.0 {
                            10.0 * value.log10()
                        } else {
                            f64::NAN
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        })
    }

    /// Key-space bit separating unwrapped-phase entries from the dB entries,
    /// which share the `(analysis << 32 | waveform)` key convention.
    const UNWRAP_KEY_BIT: u64 = 1 << 62;

    /// Continuous (unwrapped) copy of a ±180°-wrapped phase-degree series,
    /// cached under `key` like `db`.
    pub fn unwrapped(&mut self, key: u64, phase_deg: &[f64]) -> SharedWaveformValues {
        self.get_or(Self::UNWRAP_KEY_BIT | key, || {
            std::sync::Arc::new(crate::analysis::calculator::functions::unwrap_phase_deg(
                phase_deg,
            ))
        })
    }

    /// Finite (min, max) of the unwrapped series for `key`, cached alongside
    /// the per-trace ranges.
    pub fn unwrapped_range(&mut self, key: u64, phase_deg: &[f64]) -> Option<(f64, f64)> {
        let series = self.unwrapped(key, phase_deg);
        self.range_or(Self::UNWRAP_KEY_BIT | key, || finite_extremes(&series))
    }
}

// ---------------------------------------------------------------------------
// shared right-panel furniture
// ---------------------------------------------------------------------------

/// A results stat table: dimmed names left, mono values right, key rows
/// accent-highlighted (at most one or two per table, by design).
/// Forward map for the XY (equal-aspect) viewers: data → screen, matching
/// the plot engine's linear mapping.
pub(super) fn xy_screen_pos(
    plot_rect: egui::Rect,
    point: (f64, f64),
    x_range: (f64, f64),
    y_range: (f64, f64),
) -> egui::Pos2 {
    let fx = (point.0 - x_range.0) / (x_range.1 - x_range.0);
    let fy = (point.1 - y_range.0) / (y_range.1 - y_range.0);
    egui::pos2(
        plot_rect.left() + (fx as f32) * plot_rect.width(),
        plot_rect.bottom() - (fy as f32) * plot_rect.height(),
    )
}

/// One row of a point readout card.
pub(super) type CardRow = (String, String);

/// The floating point-readout card the XY viewers show on hover/pin:
/// colored mono title + k/v rows, anchored beside the point and clamped
/// to the plot. Painted, not laid out — it floats over the chart.
pub(super) fn point_card(
    ui: &Ui,
    bounds: egui::Rect,
    anchor: egui::Pos2,
    title: &str,
    title_color: egui::Color32,
    rows: &[CardRow],
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let painter = ui.painter();

    let title_galley = painter.layout_no_wrap(
        title.to_owned(),
        theme::mono(11.0, FontWeight::Medium),
        title_color,
    );
    let mut key_width = 0.0f32;
    let mut value_width = 0.0f32;
    let galleys: Vec<_> = rows
        .iter()
        .map(|(k, v)| {
            let kg = painter.layout_no_wrap(
                k.clone(),
                theme::mono(11.0, FontWeight::Regular),
                c.text_dim,
            );
            let vg =
                painter.layout_no_wrap(v.clone(), theme::mono(11.0, FontWeight::Regular), c.text);
            key_width = key_width.max(kg.size().x);
            value_width = value_width.max(vg.size().x);
            (kg, vg)
        })
        .collect();

    let (pad_x, pad_y, gap, line_h) = (9.0, 6.0, 12.0, 16.0);
    let width = (key_width + gap + value_width).max(title_galley.size().x) + pad_x * 2.0;
    let height = pad_y * 2.0 + line_h * (rows.len() as f32 + 1.0);

    let mut origin = anchor + egui::vec2(14.0, -height - 8.0);
    if origin.x + width > bounds.right() - 4.0 {
        origin.x = anchor.x - width - 14.0;
    }
    origin.y = origin.y.clamp(
        bounds.top() + 4.0,
        (bounds.bottom() - height - 4.0).max(bounds.top() + 4.0),
    );

    let rect = egui::Rect::from_min_size(origin, egui::vec2(width, height));
    painter.rect(
        rect,
        t.radius,
        c.bg_elevated,
        egui::Stroke::new(1.0, c.border_strong),
        egui::StrokeKind::Inside,
    );
    painter.galley(
        egui::pos2(origin.x + pad_x, origin.y + pad_y),
        title_galley,
        title_color,
    );
    for (i, (kg, vg)) in galleys.into_iter().enumerate() {
        let y = origin.y + pad_y + line_h * (i as f32 + 1.0);
        painter.galley(egui::pos2(origin.x + pad_x, y), kg, c.text_dim);
        painter.galley(
            egui::pos2(
                origin.x + pad_x + key_width + gap + value_width - vg.size().x,
                y,
            ),
            vg,
            c.text,
        );
    }
}

pub(super) fn stat_table(ui: &mut Ui, rows: &[(&str, String, bool)]) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();
    let (name_width, value_width) = stat_column_widths(width);
    for (i, (name, value, highlight)) in rows.iter().enumerate() {
        let name_galley = ui.painter().layout(
            (*name).to_owned(),
            theme::sans(tokens::FS_1, FontWeight::Regular),
            if *highlight { c.text } else { c.text_dim },
            name_width,
        );
        let value_galley = ui.painter().layout(
            value.clone(),
            theme::mono(tokens::FS_1, FontWeight::Regular),
            if *highlight { c.accent } else { c.text },
            value_width,
        );
        let row_height = 25.0_f32.max(name_galley.size().y.max(value_galley.size().y) + 8.0);
        let (rect, _) = ui.allocate_exact_size(egui::vec2(width, row_height), egui::Sense::hover());
        if !ui.is_rect_visible(rect) {
            continue;
        }
        let painter = ui.painter();
        let name_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left() + 12.0, rect.top() + 4.0),
            egui::vec2(name_width, row_height - 8.0),
        );
        let value_rect = egui::Rect::from_min_size(
            egui::pos2(name_rect.right() + 8.0, rect.top() + 4.0),
            egui::vec2(value_width, row_height - 8.0),
        );
        painter
            .with_clip_rect(name_rect)
            .galley(name_rect.min, name_galley, c.text_dim);
        painter.with_clip_rect(value_rect).galley(
            egui::pos2(value_rect.right() - value_galley.size().x, value_rect.top()),
            value_galley,
            c.text,
        );
        if i + 1 < rows.len() {
            painter.hline(
                rect.x_range(),
                rect.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );
        }
    }
}

fn stat_column_widths(width: f32) -> (f32, f32) {
    const OUTER_INSET: f32 = 24.0;
    const GAP: f32 = 8.0;
    let content = (width - OUTER_INSET - GAP).max(0.0);
    let name = content * 0.42;
    (name, content - name)
}

/// A faint explanatory note under a right-panel section.
pub(super) fn panel_note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .inner_margin(egui::Margin {
            left: 12,
            right: 12,
            top: 4,
            bottom: 10,
        })
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(text)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
        });
}

/// Parse a `#rrggbb` trace color, falling back to the palette cycle.
pub fn trace_color(hex: &str, fallback: egui::Color32) -> egui::Color32 {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6
        && let Ok(value) = u32::from_str_radix(hex, 16)
    {
        return egui::Color32::from_rgb(
            ((value >> 16) & 0xff) as u8,
            ((value >> 8) & 0xff) as u8,
            (value & 0xff) as u8,
        );
    }
    fallback
}

/// Resolve a waveform's display color from its stored hex + palette fallback.
pub fn waveform_color(waveform: &WaveformData, index: usize, t: &Tokens) -> egui::Color32 {
    trace_color(
        &waveform.color,
        t.color.traces[index % t.color.traces.len()],
    )
}

/// Centered faint hint on an empty document well.
pub fn well_hint(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let rect = ui.available_rect_before_wrap();
    let response = ui.interact(
        rect,
        ui.id().with(("result-well-status", text)),
        egui::Sense::hover(),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, true, text));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
        node.set_label(text);
    });
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        theme::sans(tokens::FS_2, FontWeight::Regular),
        t.color.text_faint,
    );
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the Results workspace center view (docbar + active viewer).
pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    show_with_chrome(ui, app, ResultChrome::Full);
    create_document::show(ui.ctx(), app);
}

/// Render one project-owned result document selected by its stable identity.
///
/// Unlike the dataset quick-view, this projection is driven by the exact
/// pages, panes, viewer identities, and immutable bindings retained in the
/// project document.
pub(crate) fn show_persistent_document(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    document_id: crate::product::ResultDocumentId,
) {
    persistent_document::show(ui, app, document_id);
    create_document::show(ui.ctx(), app);
}

/// Resolve and select the primary binding of a project-owned result document.
///
/// Document-tab activation and post-create activation share this path so a
/// document can never select one retained result when created and a different
/// one when reopened.
pub(crate) fn activate_persistent_document(
    state: &mut AppState,
    document_id: crate::product::ResultDocumentId,
) -> bool {
    persistent_document::activate(state, document_id)
}

/// Render the compact upgraded-mockup projection used beside another
/// engineering document. This is the same canonical retained result state,
/// not a second result document.
pub fn show_compact_split(ui: &mut Ui, app: &mut RSpiceApp) {
    show_with_chrome(ui, app, ResultChrome::CompactSplit);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ResultChrome {
    Full,
    CompactSplit,
}

fn show_with_chrome(ui: &mut Ui, app: &mut RSpiceApp, chrome: ResultChrome) {
    app.state.ui.results.set_sample_selection(None);
    prepare_viewer_state(app);
    let plan_before_docbar = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(|plan| (plan.id(), plan.revision()));
    match chrome {
        ResultChrome::Full => {
            show_docbar(ui, app);
            if viewer_has_sheet_bar(app.state.ui.results.viewer) {
                show_sheet_bar(ui, &mut app.state);
            }
        }
        ResultChrome::CompactSplit => show_compact_docbar(ui, &mut app.state),
    }
    crate::ui::plot::set_interaction_mode(
        ui.ctx(),
        app.state.ui.results.plot_tool.interaction_mode(),
    );
    let plan_after_docbar = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(|plan| (plan.id(), plan.revision()));
    if plan_after_docbar != plan_before_docbar {
        app.invalidate_simulation_preflight();
    }

    // The stage bottom holds at most one thin, single-purpose readout strip.
    // It is content-fit, so a stage with nothing to read out gives the whole
    // area back to the document.
    let strip_height = match chrome {
        ResultChrome::Full => readout_strip_height(&app.state),
        ResultChrome::CompactSplit => 0.0,
    };
    let available = ui.available_rect_before_wrap();
    let well_height = (available.height() - strip_height).max(0.0);
    ui.allocate_ui(egui::vec2(available.width(), well_height), |ui| {
        ui.set_min_height(well_height);
        show_viewer_well(ui, app, chrome);
    });
    if strip_height > 0.0 {
        waves::readout_strip(ui, &mut app.state, strip_height);
    }
}

/// Structured result documents own their controls inside their content and
/// therefore collapse the conditional 31 px sheet row completely. Plot and
/// custom-canvas sheets retain that row for their instrument or purpose bar.
const fn viewer_has_sheet_bar(viewer: ResultViewer) -> bool {
    !matches!(
        viewer,
        ResultViewer::Op
            | ResultViewer::TransferFunction
            | ResultViewer::Specs
            | ResultViewer::Table
            | ResultViewer::Manifest
    )
}

/// Height of the stage's readout strip for the active viewer, or zero.
///
/// Only the cursor-bearing waveform viewers carry a readout; structured
/// documents (OP, specs, tables) have no cursor to report.
fn readout_strip_height(state: &AppState) -> f32 {
    match state.ui.results.viewer {
        ResultViewer::Waves => waves::readout_strip_height(state),
        _ => 0.0,
    }
}

pub(crate) fn viewer_is_available(state: &AppState, viewer: ResultViewer) -> bool {
    viewer_availability(state, viewer).available
}

pub(crate) fn viewer_unavailability_reason(
    state: &AppState,
    viewer: ResultViewer,
) -> Option<&'static str> {
    let availability = viewer_availability(state, viewer);
    (!availability.available).then_some(availability.reason)
}

/// Render only the canonical result viewer well. Visualization Studio owns
/// its document toolbar, library, exact-data dock, and entity inspector, so
/// embedding the same retained renderer must not duplicate the quick-view
/// docbar or create a second result document.
pub(crate) fn show_embedded_with_sample_selection(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    selection: Option<SourceSampleSelection>,
) {
    app.state.ui.results.set_sample_selection(selection);
    prepare_viewer_state(app);
    show_viewer_well(ui, app, ResultChrome::Full);
}

/// Render one already-resolved persistent pane through an exact existing
/// viewer. The caller owns binding and compatibility checks; this helper only
/// refreshes renderer caches and deliberately restores the requested viewer
/// after quick-view reconciliation so no fallback viewer can be substituted.
pub(super) fn show_persistent_pane_viewer(ui: &mut Ui, app: &mut RSpiceApp, viewer: ResultViewer) {
    app.state.ui.results.set_sample_selection(None);
    prepare_viewer_state(app);
    app.state.ui.results.viewer = viewer;
    show_viewer_well(ui, app, ResultChrome::Full);
}

fn prepare_viewer_state(app: &mut RSpiceApp) {
    let data_version = app.state.simulation.data_version;
    let results = &mut app.state.ui.results;
    if results.seen_version != data_version {
        results.seen_version = data_version;
        results.clear_cursors();
        results.horizontal_cursor = None;
        results.active_wave_pane = None;
        results.selected_trace = None;
        // Pinned XY readouts index into the old run's point arrays;
        // a same-shape new run would silently relabel them.
        results.rf_pin.clear();
    }
    results.cache.ensure_version(data_version);
    results.derived.ensure_version(data_version);

    reconcile_active_viewer(&mut app.state);
}

fn show_viewer_well(ui: &mut Ui, app: &mut RSpiceApp, chrome: ResultChrome) {
    let t = Tokens::get(ui.ctx());
    // The document well backdrop; viewers paint on top. The rect doubles
    // as the crop window for viewer PNG export.
    let well = ui.available_rect_before_wrap();
    ui.painter().rect_filled(well, 0.0, t.color.canvas_bg);
    app.state.ui.results.well_rect = Some(well);
    let viewer = app.state.ui.results.viewer;
    let panel = ui.interact(
        well,
        ui.id().with(("result-viewer-panel", viewer)),
        egui::Sense::hover(),
    );
    let panel_label = format!("{} result viewer", viewer.tab_label());
    panel.widget_info(|| WidgetInfo::labeled(WidgetType::Label, true, &panel_label));
    ui.ctx().accesskit_node_builder(panel.id, |node| {
        node.set_role(egui::accesskit::Role::TabPanel);
        node.set_label(panel_label);
    });

    if viewer != ResultViewer::Manifest && !app.state.simulation.has_results() {
        let shortcut = app.state.ui.preferences.shortcuts().resolved_label(
            crate::workbench::commands::vocabulary::Command::RunSimulation,
            crate::workbench::app_state::runtime_command_platform(ui.ctx()),
            ui.ctx().os(),
        );
        let hint = if shortcut.is_empty() {
            "No results yet — run a simulation".to_owned()
        } else {
            format!("No results yet — run a simulation ({shortcut})")
        };
        well_hint(ui, &hint);
        return;
    }

    match viewer {
        ResultViewer::Waves => match chrome {
            ResultChrome::Full => waves::show(ui, &mut app.state),
            ResultChrome::CompactSplit => waves::show_compact(ui, &mut app.state),
        },
        ResultViewer::Bode => bode::show(ui, &mut app.state),
        ResultViewer::Fft => {
            if ensure_derived(ui, app, ActiveViewer::Fft) {
                fft::show(ui, &mut app.state);
            }
        }
        ResultViewer::Eye => {
            if ensure_derived(ui, app, ActiveViewer::EyeDiagram) {
                eye::show(ui, &mut app.state);
            }
        }
        ResultViewer::Hist => hist::show(ui, &mut app.state),
        ResultViewer::Op => op_inspector::show(ui, &mut app.state),
        ResultViewer::NoiseContrib => noise_contrib::show(ui, &mut app.state),
        ResultViewer::Contribution => sensitivity::show(ui, &mut app.state),
        ResultViewer::TransferFunction => transfer_function::show(ui, &mut app.state),
        ResultViewer::Specs => specs::show(ui, &mut app.state),
        ResultViewer::Table => table::show(ui, &mut app.state),
        ResultViewer::Nyquist => nyquist::show(ui, &mut app.state),
        ResultViewer::Smith => smith::show(ui, &mut app.state),
        ResultViewer::PoleZero => pz::show(ui, &mut app.state),
        ResultViewer::Manifest => manifest::show(ui, &app.state),
    }
}

/// Split projection of the mockup's viewer-tab row. Run binding, output
/// controls, result-document creation and properties remain owned by the full
/// Results workspace; every compatible existing viewer stays reachable here.
fn show_compact_docbar(ui: &mut Ui, state: &mut AppState) {
    docbar_at_height(ui, RESULT_VIEWER_TABS_HEIGHT, |ui| {
        let tabs = egui::ScrollArea::horizontal()
            .id_salt("rspice.results.split.viewer-tabs")
            .auto_shrink([false, true])
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| ui.horizontal(|ui| viewer_tabs(ui, state)));
        ui.ctx()
            .accesskit_node_builder(tabs.inner.response.id, |node| {
                node.set_role(egui::accesskit::Role::TabList);
                node.set_label("Compatible result viewers");
            });
    });
}

fn show_docbar(ui: &mut Ui, app: &mut RSpiceApp) {
    show_docbar_for_family(ui, app, None);
}

pub(super) fn show_persistent_docbar(ui: &mut Ui, app: &mut RSpiceApp, family_label: &str) {
    show_docbar_for_family(ui, app, Some(family_label));
}

fn show_docbar_for_family(ui: &mut Ui, app: &mut RSpiceApp, family_label: Option<&str>) {
    let mut create_document = false;
    let mut open_properties = false;
    docbar_at_height(ui, RESULT_VIEWER_TABS_HEIGHT, |ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            open_properties =
                viewer_picker(ui, WorkbenchIcon::Sliders, "Properties…", "Plot properties");
            create_document = viewer_picker(
                ui,
                WorkbenchIcon::Add,
                "Create result document…",
                "Create a dataset-bound result document",
            );
            viewer_picker_separator(ui);

            let tabs_size = ui.available_size();
            ui.allocate_ui_with_layout(
                tabs_size,
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    egui::ScrollArea::horizontal()
                        .id_salt("rspice.results.viewer-tabs")
                        .auto_shrink([false, true])
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .show(ui, |ui| {
                            let tabs = ui.horizontal(|ui| {
                                viewer_tabs_filtered(ui, &mut app.state, |viewer| {
                                    family_label
                                        .is_none_or(|family| family_allows_viewer(family, viewer))
                                })
                            });
                            ui.ctx().accesskit_node_builder(tabs.response.id, |node| {
                                node.set_role(egui::accesskit::Role::TabList);
                                node.set_label("Compatible result viewers");
                            });
                        });
                },
            );
        });
    });

    if create_document {
        create_document::open(app);
    } else if open_properties {
        crate::workbench::documents::visualization_studio::open(app);
        crate::workbench::documents::visualization_studio::open_document_properties(app);
    }
}

const RESULT_VIEWER_TABS_HEIGHT: f32 = 41.0;
const RESULT_VIEWER_TAB_HEIGHT: f32 = 30.0;
const RESULT_SHEET_BAR_HEIGHT: f32 = 31.0;
const RESULT_INSTRUMENT_CONTROL_HEIGHT: f32 = 23.0;

fn viewer_picker(ui: &mut Ui, icon: WorkbenchIcon, label: &str, accessible_label: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let galley = ui.painter().layout_no_wrap(
        label.to_owned(),
        theme::sans(tokens::FS_1, FontWeight::Regular),
        t.color.text_dim,
    );
    let width = galley.size().x + 36.0;
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(width, RESULT_VIEWER_TAB_HEIGHT),
        egui::Sense::click(),
    );
    response
        .widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), accessible_label));
    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();
        if hovered {
            ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
        }
        let color = if hovered {
            t.color.text
        } else {
            t.color.text_dim
        };
        icon.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(rect.left() + 14.0, rect.center().y),
                egui::vec2(13.0, 13.0),
            ),
            color,
        );
        ui.painter().galley(
            egui::pos2(rect.left() + 26.0, rect.center().y - galley.size().y * 0.5),
            galley,
            color,
        );
        theme::paint_focus_ring(ui, &response, rect);
    }
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}

fn viewer_picker_separator(ui: &mut Ui) {
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(1.0, RESULT_VIEWER_TAB_HEIGHT),
        egui::Sense::hover(),
    );
    ui.painter().vline(
        rect.center().x,
        rect.y_range(),
        egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
    );
}

fn show_sheet_bar(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let viewer = state.ui.results.viewer;
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), RESULT_SHEET_BAR_HEIGHT),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    let accessible_label = if viewer == ResultViewer::Waves {
        "Plot instrument controls"
    } else {
        "Result sheet controls"
    };
    response
        .widget_info(|| WidgetInfo::labeled(WidgetType::Other, ui.is_enabled(), accessible_label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Toolbar);
        node.set_label(accessible_label);
    });

    let content = rect.shrink2(egui::vec2(8.0, 0.0));
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(content)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    child.spacing_mut().item_spacing.x = 4.0;
    if viewer == ResultViewer::Waves {
        show_wave_instrument(&mut child, state);
    } else {
        child.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            inline_result_actions(ui, state);
            let remaining = ui.available_size();
            ui.allocate_ui_with_layout(
                remaining,
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.label(
                        egui::RichText::new(sheet_purpose(state))
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                },
            );
        });
    }
}

fn show_wave_instrument(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    waves::reconcile_active_pane(state, &t);
    let limits_available = waves::spec_limits_available(state, &t);
    let envelope_available = waves::family_envelope_available(state, &t);
    let marker_available = waves::marker_at_cursor_a_available(state, &t);
    if !limits_available {
        state.ui.results.show_spec_limits = false;
    }
    if !envelope_available {
        state.ui.results.show_family_envelope = false;
    }

    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        export_menu(ui, state);
        let remaining = ui.available_size();
        ui.allocate_ui_with_layout(
            remaining,
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                let cursor_active = state.ui.results.plot_tool == ResultPlotTool::Cursor
                    && state.ui.results.cursor_tool.is_armed();
                if instrument_control(ui, "A|B", cursor_active, "A/B cursor tool").clicked() {
                    if cursor_active {
                        state.ui.results.toggle_cursor_tool();
                    } else {
                        state.ui.results.plot_tool = ResultPlotTool::Cursor;
                        if !state.ui.results.cursor_tool.is_armed() {
                            state.ui.results.toggle_cursor_tool();
                        }
                    }
                }
                if instrument_control(
                    ui,
                    "H",
                    state.ui.results.plot_tool == ResultPlotTool::HorizontalCursor,
                    "Horizontal cursor - click or drag in the active pane",
                )
                .clicked()
                {
                    state.ui.results.plot_tool = ResultPlotTool::HorizontalCursor;
                }

                instrument_separator(ui);
                if instrument_control(
                    ui,
                    "BOX",
                    state.ui.results.plot_tool == ResultPlotTool::BoxZoom,
                    "Box zoom - drag a region",
                )
                .clicked()
                {
                    state.ui.results.plot_tool = ResultPlotTool::BoxZoom;
                }
                if instrument_control(
                    ui,
                    "PAN",
                    state.ui.results.plot_tool == ResultPlotTool::Pan,
                    "Pan viewport - drag the plot",
                )
                .clicked()
                {
                    state.ui.results.plot_tool = ResultPlotTool::Pan;
                }

                instrument_separator(ui);
                if instrument_control(ui, "Z+", false, "Zoom active pane in 2x").clicked() {
                    waves::zoom_active_pane(state, &t, 0.5);
                }
                if instrument_control(ui, "Z-", false, "Zoom active pane out 2x").clicked() {
                    waves::zoom_active_pane(state, &t, 2.0);
                }
                if instrument_control(ui, "FIT", false, "Fit active waveform pane").clicked() {
                    waves::fit_active_pane(state, &t);
                }

                instrument_separator(ui);
                if ui
                    .add_enabled_ui(limits_available, |ui| {
                        instrument_control(
                            ui,
                            "LIM",
                            state.ui.results.show_spec_limits,
                            "Show exact compatible project specification limits",
                        )
                    })
                    .inner
                    .clicked()
                {
                    state.ui.results.show_spec_limits = !state.ui.results.show_spec_limits;
                }
                if ui
                    .add_enabled_ui(envelope_available, |ui| {
                        instrument_control(
                            ui,
                            "ENV",
                            state.ui.results.show_family_envelope,
                            "Show min/max envelope from retained family samples",
                        )
                    })
                    .inner
                    .clicked()
                {
                    state.ui.results.show_family_envelope = !state.ui.results.show_family_envelope;
                }
                if instrument_control(
                    ui,
                    "GRID",
                    state.ui.results.show_minor_grid,
                    "Show minor waveform grid",
                )
                .clicked()
                {
                    state.ui.results.show_minor_grid = !state.ui.results.show_minor_grid;
                }
                if ui
                    .add_enabled_ui(marker_available, |ui| {
                        instrument_control(
                            ui,
                            "+M",
                            false,
                            "Drop marker at cursor A on selected or nearest visible trace",
                        )
                    })
                    .inner
                    .clicked()
                {
                    waves::drop_marker_at_cursor_a(state, &t);
                }
            },
        );
    });
}

fn instrument_control<'a>(
    ui: &mut Ui,
    label: &'a str,
    active: bool,
    tooltip: &'a str,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let galley = ui.painter().layout_no_wrap(
        label.to_owned(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        if active {
            t.color.accent
        } else {
            t.color.text_dim
        },
    );
    let width = (galley.size().x + 12.0).max(26.0);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(width, RESULT_INSTRUMENT_CONTROL_HEIGHT),
        egui::Sense::click(),
    );
    response
        .widget_info(|| WidgetInfo::selected(WidgetType::Button, ui.is_enabled(), active, tooltip));
    if ui.is_rect_visible(rect) {
        let hover = response.hovered();
        if active || hover {
            ui.painter().rect_filled(
                rect,
                2.0,
                if active {
                    t.color.accent_dim
                } else {
                    t.color.bg_hover
                },
            );
        }
        if active {
            ui.painter().hline(
                egui::Rangef::new(rect.left() + 2.0, rect.right() - 2.0),
                rect.bottom() - 0.5,
                egui::Stroke::new(1.0, t.color.accent),
            );
        }
        ui.painter().galley(
            egui::pos2(
                rect.center().x - galley.size().x * 0.5,
                rect.center().y - galley.size().y * 0.5,
            ),
            galley,
            if active {
                t.color.accent
            } else if hover {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
        theme::paint_focus_ring(ui, &response, rect);
    }
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(tooltip)
}

fn instrument_separator(ui: &mut Ui) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(7.0, 17.0), egui::Sense::hover());
    ui.painter().vline(
        rect.center().x,
        rect.y_range(),
        egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
    );
}

fn sheet_purpose(state: &AppState) -> String {
    let viewer = state.ui.results.viewer;
    let detail = viewer_availability(state, viewer).reason;
    format!("{} · {detail}", viewer.tab_label())
}

fn export_menu(ui: &mut Ui, state: &mut AppState) {
    ui.menu_button("Export…", |ui| {
        if ui.button("Waveform data (CSV)…").clicked() {
            state.ui.export_csv_requested = true;
            ui.close();
        }
        if ui.button("Viewer image (PNG)…").clicked() {
            state.ui.export_png_requested = true;
            ui.close();
        }
    });
}

fn inline_result_actions(ui: &mut Ui, state: &mut AppState) {
    export_menu(ui, state);
    match state.ui.results.viewer {
        ResultViewer::Waves => {
            let linked_shortcut = state.ui.preferences.shortcuts().resolved_label(
                crate::workbench::commands::vocabulary::Command::ToggleLinkedCursors,
                crate::workbench::app_state::runtime_command_platform(ui.ctx()),
                ui.ctx().os(),
            );
            let results = &mut state.ui.results;
            if !results.hidden_strips.is_empty() {
                let n = results.hidden_strips.len();
                if chip(ui, &format!("{n} hidden"), true)
                    .on_hover_text("Restore closed strips")
                    .clicked()
                {
                    results.hidden_strips.clear();
                }
            }
            // A│B is a tool, not a clear button: it arms plot clicks and
            // owns whether the readout strip is on screen at all.
            let armed = results.cursor_tool.is_armed();
            let response = chip(ui, "A│B", armed).on_hover_text(if armed {
                "Cursor tool armed — click a plot to place A, again for B; Esc clears"
            } else {
                "Cursor tool off — plots ignore cursor clicks"
            });
            response.widget_info(|| WidgetInfo::selected(WidgetType::Button, true, armed, "A│B"));
            if response.clicked() {
                results.toggle_cursor_tool();
            }
            // Markers are annotation, not readout: the tool arms deliberately
            // and the markers it placed outlive disarming it.
            let marking = results.marker_tool.is_armed();
            let marker_count = results.markers.len();
            let marker_label = if marker_count == 0 {
                "MARK".to_owned()
            } else {
                format!("MARK {marker_count}")
            };
            let response = chip(ui, &marker_label, marking).on_hover_text(if marking {
                "Marker tool armed — click a plot to mark the nearest trace"
            } else {
                "Marker tool off — click to arm; existing markers stay on their plots"
            });
            response.widget_info(|| {
                WidgetInfo::selected(WidgetType::Button, true, marking, marker_label.as_str())
            });
            if response.clicked() {
                results.toggle_marker_tool();
            }
            let linked_label = "Linked A/B cursors";
            let linked = results.linked_cursors;
            let linked_tooltip = if linked {
                "A/B positions are shared across plots with matching analysis and X-axis domains"
            } else {
                "A/B positions are scoped to the active plot"
            };
            let linked_tooltip = if linked_shortcut.is_empty() {
                linked_tooltip.to_owned()
            } else {
                format!("{linked_tooltip} · {linked_shortcut}")
            };
            let response = chip(ui, linked_label, linked).on_hover_text(linked_tooltip);
            response.widget_info(|| {
                WidgetInfo::selected(WidgetType::Button, true, linked, linked_label)
            });
            if !linked_shortcut.is_empty() {
                ui.ctx().accesskit_node_builder(response.id, |node| {
                    node.set_keyboard_shortcut(linked_shortcut.as_str());
                });
            }
            if response.clicked() {
                results.toggle_linked_cursors();
            }
        }
        ResultViewer::Fft => {
            let label = state
                .analysis
                .fft_state
                .data
                .as_ref()
                .map(|d| format!("{} · {}", d.window.display_name(), d.fft_size));
            if let Some(label) = label {
                ui.label(
                    egui::RichText::new(label)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(Tokens::get(ui.ctx()).color.text_faint),
                );
            }
        }
        ResultViewer::Eye => {
            let mask_on = state.analysis.eye_diagram_state.show_mask;
            if chip(ui, "mask", mask_on).clicked() {
                state.analysis.eye_diagram_state.show_mask = !mask_on;
            }
        }
        ResultViewer::Op => {
            let filter = &mut state.ui.results.op_filter;
            if !filter.is_empty() && chip(ui, "clear", true).clicked() {
                filter.clear();
            }
            ui.add(
                egui::TextEdit::singleline(filter)
                    .desired_width(150.0)
                    .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                    .hint_text("filter devices…"),
            );
        }
        ResultViewer::Specs => {
            if state.ui.results.spec_drafts.is_some() {
                if ui.button("Discard").clicked() {
                    state.ui.results.spec_drafts = None;
                }
                if ui.button("Apply").clicked() && !specs::apply_drafts(state) {
                    state.push_sim_message(crate::diagnostics::ConsoleMessage::warning(
                        "Specs not applied — fix the invalid bound first",
                    ));
                }
            } else if ui.button("Edit specs…").clicked() {
                specs::open_editor(state);
            }
        }
        ResultViewer::Table => table::inline_actions(ui, state),
        _ => {}
    }
}

fn viewer_tabs(ui: &mut Ui, state: &mut AppState) {
    viewer_tabs_filtered(ui, state, |_| true);
}

fn viewer_tabs_filtered(
    ui: &mut Ui,
    state: &mut AppState,
    mut include: impl FnMut(ResultViewer) -> bool,
) {
    let t = Tokens::get(ui.ctx());
    ui.spacing_mut().item_spacing.x = 0.0;

    let current = state.ui.results.viewer;
    let mut clicked: Option<ResultViewer> = None;

    for viewer in ResultViewer::PRIMARY {
        let availability = viewer_availability(state, viewer);
        if !include(viewer) || !availability.available {
            continue;
        }
        if viewer_tab(ui, viewer, current == viewer, availability) {
            clicked = Some(viewer);
        }
    }
    let has_primary = ResultViewer::PRIMARY
        .into_iter()
        .any(|viewer| include(viewer) && viewer_availability(state, viewer).available);
    let has_legacy = ResultViewer::LEGACY
        .into_iter()
        .any(|viewer| include(viewer) && viewer_availability(state, viewer).available);
    if has_primary && has_legacy {
        // Legacy surfaces, gated on data availability and visually set apart.
        let tab_height = ui.available_height().min(40.0);
        let (sep, _) = ui.allocate_exact_size(egui::vec2(13.0, tab_height), egui::Sense::hover());
        ui.painter().vline(
            sep.center().x,
            egui::Rangef::new(sep.center().y - 7.0, sep.center().y + 7.0),
            egui::Stroke::new(1.0, t.color.border),
        );
    }
    for viewer in ResultViewer::LEGACY {
        let availability = viewer_availability(state, viewer);
        if !include(viewer) || !availability.available {
            continue;
        }
        if viewer_tab(ui, viewer, current == viewer, availability) {
            clicked = Some(viewer);
        }
    }
    for viewer in ResultViewer::DATASET_NATIVE {
        let availability = viewer_availability(state, viewer);
        if !include(viewer) || !availability.available {
            continue;
        }
        if viewer_tab(ui, viewer, current == viewer, availability) {
            clicked = Some(viewer);
        }
    }
    if let Some(viewer) = clicked {
        state.ui.results.viewer = viewer;
    }
}

fn family_allows_viewer(family_label: &str, viewer: ResultViewer) -> bool {
    if viewer == ResultViewer::Manifest {
        return true;
    }
    match family_label {
        "Waveform worksheet" => matches!(
            viewer,
            ResultViewer::Waves
                | ResultViewer::Eye
                | ResultViewer::TransferFunction
                | ResultViewer::Table
        ),
        "Frequency & stability" => matches!(
            viewer,
            ResultViewer::Bode
                | ResultViewer::NoiseContrib
                | ResultViewer::TransferFunction
                | ResultViewer::Nyquist
                | ResultViewer::PoleZero
        ),
        "RF & network" => matches!(viewer, ResultViewer::Fft | ResultViewer::Smith),
        "Statistics & yield" => matches!(viewer, ResultViewer::Hist | ResultViewer::Contribution),
        // These specialist mockup families have no native quick modes. They
        // resolve to the dataset manifest rather than substituting an
        // unrelated electrical viewer.
        "Digital & AMS events" | "Fields & physical" | "Photonics" | "Report page" => false,
        // Imported or migrated documents with no recognized family label keep
        // their exact retained panes reachable.
        _ => true,
    }
}

fn reconcile_active_viewer(state: &mut AppState) {
    if viewer_availability(state, state.ui.results.viewer).available {
        return;
    }
    if state.simulation.active_run().is_none() {
        return;
    }
    if let Some(viewer) = ResultViewer::PRIMARY
        .into_iter()
        .chain(ResultViewer::LEGACY)
        .chain(ResultViewer::DATASET_NATIVE)
        .find(|viewer| viewer_availability(state, *viewer).available)
    {
        state.ui.results.viewer = viewer;
    }
}

fn viewer_availability(state: &AppState, viewer: ResultViewer) -> ViewerAvailability {
    let active_run = state.simulation.active_run();
    match viewer {
        ResultViewer::Waves => {
            if active_run.is_some_and(|run| {
                run.analyses
                    .iter()
                    .any(|analysis| !analysis.waveforms.is_empty())
            }) {
                ViewerAvailability::available("Waveforms are present in the active dataset")
            } else {
                ViewerAvailability::unavailable("Requires waveform data in the active dataset")
            }
        }
        ResultViewer::Bode => {
            let has_ac_response = active_run
                .and_then(|run| {
                    crate::state::ac_bode_summary_for_selection(
                        run,
                        state.simulation.active_analysis_idx,
                    )
                })
                .is_some();
            let has_noise_spectrum = active_run.is_some_and(|run| {
                run.analyses
                    .iter()
                    .any(bode::ordinary_noise_spectrum_is_renderable)
            });
            if has_ac_response || has_noise_spectrum {
                ViewerAvailability::available(
                    "An AC response or ordinary noise spectrum is available",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires a usable AC response or ordinary noise spectrum in the active dataset",
                )
            }
        }
        ResultViewer::Fft => specialized_availability(state, ActiveViewer::Fft),
        ResultViewer::Eye => specialized_availability(state, ActiveViewer::EyeDiagram),
        ResultViewer::Hist => specialized_availability(state, ActiveViewer::Histogram),
        ResultViewer::Op => {
            if state
                .simulation
                .active_analysis()
                .and_then(|analysis| analysis.device_op.as_ref())
                .is_some_and(|report| !report.is_empty())
            {
                ViewerAvailability::available("Device operating-point data is available")
            } else {
                ViewerAvailability::unavailable(
                    "Requires a device operating-point report in the active dataset",
                )
            }
        }
        ResultViewer::NoiseContrib => {
            if active_run.is_some_and(|run| {
                run.analyses.iter().any(|analysis| {
                    analysis
                        .noise_summary
                        .as_ref()
                        .is_some_and(|summary| !summary.rows.is_empty())
                })
            }) {
                ViewerAvailability::available("Noise-contributor data is available")
            } else {
                ViewerAvailability::unavailable("Requires a noise analysis with contributor data")
            }
        }
        ResultViewer::Contribution => {
            if sensitivity::active_payload_is_valid(state) {
                ViewerAvailability::available(
                    "Retained sensitivity contributions are available for the active analysis",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires the active analysis to contain a valid retained sensitivity payload",
                )
            }
        }
        ResultViewer::TransferFunction => {
            if transfer_function::active_payload_is_valid(state) {
                ViewerAvailability::available(
                    "Retained transfer-function evidence is available for the active analysis",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires the active analysis to contain a valid retained transfer-function payload",
                )
            }
        }
        ResultViewer::Specs => {
            let has_measurements = state.simulation.runs.iter().any(|run| {
                run.analyses
                    .iter()
                    .any(|analysis| !analysis.measurements.is_empty())
            });
            if !state.workspace.specs.is_empty() || has_measurements {
                ViewerAvailability::available("Specification or measurement data is available")
            } else {
                ViewerAvailability::unavailable("Requires specifications or measured results")
            }
        }
        ResultViewer::Table => {
            // The table reads whatever the WAVES stage can plot: if a strip
            // exists, its retained samples can be listed.
            if state
                .simulation
                .active_run()
                .is_some_and(|run| run.analyses.iter().any(|a| !a.waveforms.is_empty()))
            {
                ViewerAvailability::available("Retained samples are available to list")
            } else {
                ViewerAvailability::unavailable("Requires an analysis with retained samples")
            }
        }
        ResultViewer::Nyquist => specialized_availability(state, ActiveViewer::Nyquist),
        ResultViewer::Smith => specialized_availability(state, ActiveViewer::SmithChart),
        ResultViewer::PoleZero => specialized_availability(state, ActiveViewer::PoleZero),
        ResultViewer::Manifest => {
            if active_run.is_some() {
                ViewerAvailability::available("The immutable active-dataset inventory is available")
            } else {
                ViewerAvailability::unavailable("Requires a selected retained dataset")
            }
        }
    }
}

fn specialized_availability(state: &AppState, viewer: ActiveViewer) -> ViewerAvailability {
    let capability = state.viewer_capability(viewer);
    ViewerAvailability {
        available: capability.available,
        reason: capability.reason,
    }
}

/// One viewer tab, per the mockup: full-strip hit target, compact horizontal
/// padding, hover fill, and a 2 px bottom rule when active.
fn viewer_tab(
    ui: &mut Ui,
    viewer: ResultViewer,
    active: bool,
    availability: ViewerAvailability,
) -> bool {
    use crate::ui::theme::mix;

    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let mut job = egui::text::LayoutJob::default();
    job.append(
        viewer.tab_label(),
        0.0,
        egui::TextFormat {
            font_id: theme::sans(tokens::FS_1, FontWeight::Regular),
            color: egui::Color32::PLACEHOLDER,
            ..Default::default()
        },
    );
    let galley = ui.fonts_mut(|f| f.layout_job(job));

    let height = RESULT_VIEWER_TAB_HEIGHT.min(ui.available_height());
    let horizontal_padding = 20.0;
    let icon_width = 13.0;
    let icon_gap = 6.0;
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            galley.size().x + horizontal_padding + icon_width + icon_gap,
            height,
        ),
        if availability.available {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::SelectableLabel,
            availability.available && ui.is_enabled(),
            viewer.tab_label(),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Tab);
        if active {
            node.set_selected(true);
        } else {
            node.set_selected(false);
        }
    });
    if !ui.is_rect_visible(rect) {
        return false;
    }

    let hover = ui.ctx().animate_bool_with_time(
        response.id,
        availability.available && !active && response.hovered(),
        ui.style().animation_time,
    );
    let (fill, text_color) = if active {
        (egui::Color32::TRANSPARENT, c.text)
    } else if !availability.available {
        (egui::Color32::TRANSPARENT, c.text_faint)
    } else {
        (
            mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
            mix(c.text_dim, c.text, hover),
        )
    };

    let painter = ui.painter();
    if fill != egui::Color32::TRANSPARENT {
        painter.rect_filled(rect, 0.0, fill);
    }
    let icon_rect = egui::Rect::from_center_size(
        egui::pos2(
            rect.left() + horizontal_padding * 0.5 + icon_width * 0.5,
            rect.center().y,
        ),
        egui::vec2(icon_width, icon_width),
    );
    viewer.tab_icon().paint(painter, icon_rect, text_color);
    painter.galley(
        egui::pos2(
            icon_rect.right() + icon_gap,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        text_color,
    );
    if active {
        painter.rect_filled(
            egui::Rect::from_min_max(
                egui::pos2(rect.left() + 8.0, rect.bottom() - 2.0),
                egui::pos2(rect.right() - 8.0, rect.bottom()),
            ),
            0.0,
            c.accent,
        );
    }

    theme::paint_focus_ring(ui, &response, rect);

    if !availability.available {
        response.on_hover_text(availability.reason);
        return false;
    }
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
        && !active
}

/// Gate FFT/eye rendering on the controller's derived-data loader. Returns
/// `true` when the viewer can render.
fn ensure_derived(ui: &mut Ui, app: &mut RSpiceApp, viewer: ActiveViewer) -> bool {
    let Some(analysis_type) = app
        .state
        .simulation
        .active_analysis()
        .map(|analysis| analysis.analysis_type)
    else {
        well_hint(ui, "Select an analysis with transient data");
        return false;
    };
    if !SimulationController::analysis_supports_transient_derivation(analysis_type) {
        well_hint(ui, "The active analysis has no usable time-domain source");
        return false;
    }
    match app
        .simulation_controller
        .ensure_transient_viewer_data(&mut app.state, viewer)
    {
        DerivedViewerLoadState::Ready => true,
        DerivedViewerLoadState::Loading => {
            well_hint(ui, "Preparing derived data from the active transient…");
            ui.ctx().request_repaint();
            false
        }
        DerivedViewerLoadState::Unavailable => {
            well_hint(ui, "The active analysis does not contain a usable source");
            false
        }
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Render the Results context right panel for the active viewer.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    match state.ui.results.viewer {
        ResultViewer::Waves => waves::right_panel(ui, state),
        ResultViewer::Bode => bode::right_panel(ui, state),
        ResultViewer::Fft => fft::right_panel(ui, state),
        ResultViewer::Eye => eye::right_panel(ui, state),
        ResultViewer::Hist => hist::right_panel(ui, state),
        ResultViewer::Op => op_inspector::right_panel(ui, state),
        ResultViewer::NoiseContrib => noise_contrib::right_panel(ui, state),
        ResultViewer::Contribution => sensitivity::right_panel(ui, state),
        ResultViewer::TransferFunction => transfer_function::right_panel(ui, state),
        ResultViewer::Specs => specs::right_panel(ui, state),
        ResultViewer::Table => table::right_panel(ui, state),
        ResultViewer::Nyquist => nyquist::right_panel(ui, state),
        ResultViewer::Smith => smith::right_panel(ui, state),
        ResultViewer::PoleZero => pz::right_panel(ui, state),
        ResultViewer::Manifest => manifest::right_panel(ui, state),
    }
}

#[cfg(test)]
mod availability_tests {
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisResultPayload, AnalysisType, SensitivityResultMode,
        SensitivityResultRow, SimulationRun, WaveformData,
    };

    fn state_with_analysis(analysis: AnalysisResult) -> AppState {
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));
        state
    }

    #[test]
    fn presentation_state_follows_analysis_identity_after_reorder() {
        let first =
            AnalysisResult::new(101, AnalysisType::Transient, "TRAN A").with_waveforms(vec![
                WaveformData::new("V(a)", vec![0.0, 1.0], vec![1.0, 2.0], "#ffbd2e"),
            ]);
        let second =
            AnalysisResult::new(202, AnalysisType::Transient, "TRAN B").with_waveforms(vec![
                WaveformData::new("V(b)", vec![0.0, 1.0], vec![3.0, 4.0], "#4ec9b0"),
            ]);
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(first);
        run.add_analysis(second);
        let dataset_id = run.dataset_id;
        let first_key = AnalysisPresentationKey::new(dataset_id, &run.analyses[0]);
        let second_key = AnalysisPresentationKey::new(dataset_id, &run.analyses[1]);
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));

        state.ui.results.hidden_strips.insert(first_key);
        state.ui.results.maximized_strip = Some(second_key);
        state.ui.results.analysis_exprs.insert(
            first_key,
            vec![ExprTrace {
                text: "V(a) * 2".to_owned(),
                visible: true,
            }],
        );
        state
            .ui
            .results
            .analysis_plot_view_pane_mut(ResultViewer::Waves, first_key, 0)
            .x = Some((0.25, 0.75));
        state.ui.results.table.analysis = Some(second_key);

        state.simulation.runs[0].analyses.swap(0, 1);
        let reordered = state.simulation.active_run().expect("active retained run");
        assert_eq!(
            first_key.resolve(reordered).map(|(index, _)| index),
            Some(1)
        );
        assert_eq!(
            second_key.resolve(reordered).map(|(index, _)| index),
            Some(0)
        );
        assert!(state.ui.results.hidden_strips.contains(&first_key));
        assert_eq!(state.ui.results.maximized_strip, Some(second_key));
        assert_eq!(
            state.ui.results.analysis_exprs[&first_key][0].text,
            "V(a) * 2"
        );
        assert_eq!(
            state
                .ui
                .results
                .analysis_plot_view_pane(ResultViewer::Waves, first_key, 0)
                .x,
            Some((0.25, 0.75))
        );
        assert_eq!(state.ui.results.table.analysis, Some(second_key));
    }

    #[test]
    fn table_and_marker_waveform_identity_survive_waveform_reorder() {
        let analysis =
            AnalysisResult::new(101, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("V(a)", vec![0.0, 1.0], vec![1.0, 2.0], "#ffbd2e"),
                WaveformData::new("V(b)", vec![0.0, 1.0], vec![3.0, 4.0], "#4ec9b0"),
            ]);
        let mut state = state_with_analysis(analysis);
        let run = state.simulation.active_run().expect("active retained run");
        let analysis_key = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
        let trace = TracePresentationKey {
            source_name: "V(a)".to_owned(),
            kind: 0,
            family_group: 0,
        };
        let waveform = WaveformPresentationKey {
            analysis: analysis_key,
            trace: trace.clone(),
        };
        state.ui.results.table.analysis = Some(analysis_key);
        state.ui.results.table.columns = vec![trace];
        state
            .ui
            .results
            .add_marker(analysis_key, waveform.clone(), "V(a)".to_owned(), 0.5);

        state.simulation.runs[0].analyses[0].waveforms.swap(0, 1);
        assert_eq!(
            state.ui.results.table.columns[0].source_name, "V(a)",
            "the selected table column must not become the new waveform at slot zero"
        );
        assert_eq!(state.ui.results.markers[0].anchor, waveform);
        assert_eq!(
            state.ui.results.markers[0].anchor.trace.source_name, "V(a)",
            "the marker must remain attached to its source signal"
        );
    }

    #[test]
    fn selected_trace_identity_fails_closed_after_active_dataset_changes() {
        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e"),
            ]);
        let mut state = state_with_analysis(analysis.clone());
        state.ui.results.selected_trace = Some(
            SelectedResultTrace::from_run_indices(
                state.simulation.active_run().expect("active retained run"),
                0,
                0,
            )
            .expect("selected retained trace"),
        );
        assert!(
            state
                .ui
                .results
                .valid_selected_trace(&state.simulation)
                .is_some()
        );

        state.simulation.start_run().add_analysis(analysis);

        assert!(
            state
                .ui
                .results
                .valid_selected_trace(&state.simulation)
                .is_none()
        );
    }

    #[test]
    fn waveform_data_does_not_enable_incompatible_result_viewers() {
        let state = state_with_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0, 2.0],
                    vec![0.0, 1.0, 0.0],
                    "#00aaff",
                ),
            ]),
        );

        assert!(viewer_availability(&state, ResultViewer::Waves).available);
        assert!(!viewer_availability(&state, ResultViewer::Bode).available);
        assert!(!viewer_availability(&state, ResultViewer::Hist).available);
        assert!(!viewer_availability(&state, ResultViewer::NoiseContrib).available);
        assert!(!viewer_availability(&state, ResultViewer::Contribution).available);
    }

    #[test]
    fn op_viewer_requires_the_selected_analysis_device_report() {
        let report = rspice_core::circuit::DeviceOpReport {
            entries: vec![rspice_core::circuit::DeviceOpEntry {
                name: "M1".to_owned(),
                device_kind: "MOSFET",
                region: Some("saturation"),
                params: Vec::new(),
            }],
        };
        let mut run = SimulationRun::new(1);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::DcOp, "OP with report").with_device_op(report),
        );
        run.add_analysis(AnalysisResult::new(
            2,
            AnalysisType::DcOp,
            "OP without report",
        ));
        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));

        assert!(state.simulation.select_analysis(1));
        assert!(!viewer_availability(&state, ResultViewer::Op).available);
        assert!(state.simulation.select_analysis(0));
        assert!(viewer_availability(&state, ResultViewer::Op).available);
    }

    #[test]
    fn contribution_viewer_requires_the_active_valid_sensitivity_payload() {
        let payload = AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: SensitivityResultMode::Dc,
            rows: vec![SensitivityResultRow {
                parameter: "r1".to_owned(),
                raw: 0.25,
                normalized: 0.5,
            }],
        };
        let state = state_with_analysis(
            AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS")
                .with_result_payload(payload.clone()),
        );
        assert!(viewer_availability(&state, ResultViewer::Contribution).available);

        let mut wrong = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
        wrong.result_payload = Some(payload);
        let wrong_analysis = state_with_analysis(wrong);
        assert!(!viewer_availability(&wrong_analysis, ResultViewer::Contribution).available);
    }

    #[test]
    fn incompatible_active_viewer_falls_back_to_compatible_view() {
        let mut state = state_with_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0, 2.0],
                    vec![0.0, 1.0, 0.0],
                    "#00aaff",
                ),
            ]),
        );
        state.ui.results.viewer = ResultViewer::Hist;

        reconcile_active_viewer(&mut state);

        assert_eq!(state.ui.results.viewer, ResultViewer::Waves);
    }

    #[test]
    fn result_shell_matches_mockup_bar_geometry() {
        assert_eq!(RESULT_VIEWER_TABS_HEIGHT, 41.0);
        assert_eq!(RESULT_VIEWER_TAB_HEIGHT, 30.0);
        assert_eq!(RESULT_SHEET_BAR_HEIGHT, 31.0);
        assert_eq!(RESULT_INSTRUMENT_CONTROL_HEIGHT, 23.0);
    }

    #[test]
    fn structured_result_documents_collapse_the_conditional_sheet_bar() {
        for viewer in [
            ResultViewer::Op,
            ResultViewer::TransferFunction,
            ResultViewer::Specs,
            ResultViewer::Table,
            ResultViewer::Manifest,
        ] {
            assert!(!viewer_has_sheet_bar(viewer), "{viewer:?}");
        }
        for viewer in [
            ResultViewer::Waves,
            ResultViewer::Bode,
            ResultViewer::Fft,
            ResultViewer::Eye,
            ResultViewer::Hist,
            ResultViewer::NoiseContrib,
            ResultViewer::Contribution,
            ResultViewer::Nyquist,
            ResultViewer::Smith,
            ResultViewer::PoleZero,
        ] {
            assert!(viewer_has_sheet_bar(viewer), "{viewer:?}");
        }
    }

    #[test]
    fn persistent_document_families_expose_only_their_mockup_viewers() {
        assert!(family_allows_viewer(
            "Waveform worksheet",
            ResultViewer::Waves
        ));
        assert!(family_allows_viewer(
            "Waveform worksheet",
            ResultViewer::Eye
        ));
        assert!(!family_allows_viewer(
            "Waveform worksheet",
            ResultViewer::Smith
        ));

        assert!(family_allows_viewer(
            "Frequency & stability",
            ResultViewer::Bode
        ));
        assert!(family_allows_viewer(
            "Frequency & stability",
            ResultViewer::PoleZero
        ));
        assert!(!family_allows_viewer(
            "Frequency & stability",
            ResultViewer::Hist
        ));

        assert!(family_allows_viewer(
            "Statistics & yield",
            ResultViewer::Contribution
        ));
        assert!(!family_allows_viewer(
            "Statistics & yield",
            ResultViewer::Nyquist
        ));
        assert!(!family_allows_viewer(
            "Fields & physical",
            ResultViewer::Waves
        ));
        assert!(!family_allows_viewer("Report page", ResultViewer::Waves));
        for family in [
            "Waveform worksheet",
            "Frequency & stability",
            "RF & network",
            "Statistics & yield",
            "Digital & AMS events",
            "Fields & physical",
            "Photonics",
            "Report page",
        ] {
            assert!(
                family_allows_viewer(family, ResultViewer::Manifest),
                "{family}"
            );
        }
    }

    #[test]
    fn stat_columns_are_disjoint_at_phone_panel_width() {
        let width = 240.0;
        let (name, value) = stat_column_widths(width);
        assert!(name > 0.0);
        assert!(value > name);
        assert!((name + value + 24.0 + 8.0 - width).abs() < f32::EPSILON * width);
    }
}
