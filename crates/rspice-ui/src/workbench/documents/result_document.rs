//! The Results workspace.
//!
//! A 41 px dataset-viewer tab strip, a distinct 31 px plot instrument or
//! purpose strip, a fill-height document well carrying the active viewer
//! (waveform strips, Bode, FFT, eye, histogram, and the
//! Nyquist/Smith/pole-zero diagnostics), and a content-fit readout strip.

mod bode;
mod create_document;
mod events;
mod eye;
mod fft;
mod harmonic_balance;
mod hist;
pub(crate) mod manifest;
mod noise_contrib;
mod nyquist;
mod op_inspector;
mod optimization;
mod persistent_document;
mod phase_noise;
mod pz;
mod reliability;
mod sensitivity;
mod smith;
mod soa;
mod specs;
mod table;
mod transfer_function;
mod virtual_rows;

pub(crate) fn open_specification_editor(state: &mut AppState) {
    specs::open_editor(state);
}

pub(crate) fn harmonic_balance_analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    harmonic_balance::analysis_is_renderable(analysis)
}

pub(crate) fn harmonic_balance_waveform_is_renderable(waveform: &WaveformData) -> bool {
    harmonic_balance::spectrum_trace_is_renderable(waveform)
}

pub(crate) fn phase_noise_analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    phase_noise::phase_noise_is_renderable(analysis)
}

pub(crate) fn phase_noise_waveform_is_renderable(waveform: &WaveformData) -> bool {
    phase_noise::phase_noise_waveform_is_renderable(waveform)
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
pub(crate) use waves::{
    ActivePaneFacts, SharedXStatus, active_pane_facts, active_shared_x_status,
    analysis_default_unit, browser_signal_is_current, browser_signal_unit,
};

pub(crate) use waves::toggle_visibility;

use std::collections::{HashMap, HashSet};

use egui::{Ui, WidgetInfo, WidgetType};
use serde::{Deserialize, Serialize};

use super::visualization_family::SourceSampleSelection;
use crate::product::{AnalysisInstanceId, DatasetId};
use crate::simulation::SimulationController;
use crate::simulation::controller::DerivedViewerLoadState;
use crate::state::{
    AnalysisResult, SharedWaveformValues, SimulationRun, SoaParameterEvidence, WaveformData,
};
use crate::ui::icons::Icon;
use crate::ui::plot::{CursorPair, DecimationCache, InteractionMode};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{IconButton, chip, docbar_at_height};
use crate::workbench::app_state::ActiveViewer;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::state::Workspace;
use crate::workbench::{AppState, RSpiceApp};

/// One axis interval, low then high, in data space.
pub(crate) type AxisExtent = (f64, f64);
/// The X and Y intervals one plot last drew.
pub(crate) type DrawnAxes = (AxisExtent, AxisExtent);

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

/// Whether retained evidence belongs to the stable analysis authored in the
/// simulation plan. Deterministically expanded executions (for example PVT
/// points) have distinct execution identities and must still match their
/// common authored analysis.
pub(crate) fn analysis_matches_authored_source(
    analysis: &AnalysisResult,
    authored_source_id: AnalysisInstanceId,
) -> bool {
    analysis
        .provenance()
        .is_some_and(|provenance| provenance.authored_source_instance_id() == authored_source_id)
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
    /// Swept-source or swept-parameter DC transfer curves.
    DcSweep,
    /// Loop-gain stability view with margin markers.
    Bode,
    /// Spectrum with harmonic markers.
    Fft,
    /// Retained complex harmonic-balance coefficient spectrum.
    HarmonicBalance,
    /// Periodic phase-noise spectrum versus offset frequency.
    PhaseNoise,
    /// Eye diagram with compliance mask.
    Eye,
    /// Monte-Carlo distribution.
    Hist,
    /// Per-device operating-point inspector (Spectre-style OP info).
    Op,
    /// Ordinary-noise root spectral density with contributor evidence.
    NoiseContrib,
    /// Ranked signed parameter-sensitivity contributions.
    Contribution,
    /// Scalar DC transfer gain and input/output resistances.
    TransferFunction,
    /// Measurements × runs matrix against spec bounds.
    Specs,
    /// The retained samples of one analysis, as rows.
    Table,
    /// Nyquist loop-gain stability surface.
    Nyquist,
    /// Smith-chart RF/network surface.
    Smith,
    /// Complex-plane pole-zero surface.
    PoleZero,
    /// Committed XSPICE digital and real-valued event history.
    Events,
    /// Safe-operating-area rule evidence with per-rule stress history.
    Soa,
    /// Ageing shift and lifetime checkpoints per stressed device.
    Reliability,
    /// Optimizer cost convergence and the candidate history behind it.
    Optimization,
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
            ResultViewer::DcSweep => "DC",
            ResultViewer::Bode => "BODE",
            ResultViewer::Fft => "FFT",
            ResultViewer::HarmonicBalance => "HB",
            ResultViewer::PhaseNoise => "PNOISE",
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
            ResultViewer::Events => "EVENTS",
            ResultViewer::Soa => "SOA",
            ResultViewer::Reliability => "AGEING",
            ResultViewer::Optimization => "OPT",
            ResultViewer::Manifest => "MANIFEST",
        }
    }

    const PRIMARY: [ResultViewer; 21] = [
        ResultViewer::Waves,
        ResultViewer::DcSweep,
        ResultViewer::Bode,
        ResultViewer::NoiseContrib,
        ResultViewer::Nyquist,
        ResultViewer::Fft,
        ResultViewer::HarmonicBalance,
        ResultViewer::PhaseNoise,
        ResultViewer::Smith,
        ResultViewer::TransferFunction,
        ResultViewer::Contribution,
        ResultViewer::Op,
        ResultViewer::Specs,
        ResultViewer::Table,
        ResultViewer::Hist,
        ResultViewer::Eye,
        ResultViewer::PoleZero,
        // Specialist sheets last: each needs evidence an ordinary run does not
        // produce — XSPICE event nodes, or a whole campaign analysis kind — so
        // leading with them would push the everyday sheets rightward for the
        // sake of tabs that are usually dim.
        ResultViewer::Events,
        ResultViewer::Soa,
        ResultViewer::Reliability,
        ResultViewer::Optimization,
    ];
    const DATASET_NATIVE: [ResultViewer; 1] = [ResultViewer::Manifest];

    /// Every sheet the workspace can show, tab order first.
    ///
    /// Production code always has a viewer in hand; this exists so a test can
    /// assert a property holds for the whole set rather than a list someone
    /// has to remember to extend.
    #[cfg(test)]
    pub(crate) fn every() -> impl Iterator<Item = ResultViewer> {
        Self::PRIMARY.into_iter().chain(Self::DATASET_NATIVE)
    }

    /// Human-readable document-tab label from the upgraded Results mockup.
    pub(crate) const fn tab_label(self) -> &'static str {
        match self {
            ResultViewer::Waves => "Waves",
            ResultViewer::DcSweep => "DC Sweep",
            ResultViewer::Bode => "Bode",
            ResultViewer::Fft => "FFT",
            ResultViewer::HarmonicBalance => "HB Tones",
            ResultViewer::PhaseNoise => "Phase Noise",
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
            ResultViewer::Events => "Events",
            ResultViewer::Soa => "SOA",
            ResultViewer::Reliability => "Ageing",
            ResultViewer::Optimization => "Optimization",
            ResultViewer::Manifest => "Manifest",
        }
    }

    /// The Visualization Studio viewer document this sheet renders, if any.
    ///
    /// `None` means the sheet is dataset-native: it reads the retained result
    /// directly and has no place in the Studio's document catalog, so binding
    /// a page to it would name a renderer that cannot draw the page.
    ///
    /// One owner on purpose. Three copies of this map existed — in the
    /// persistent-document layer, the Studio, and the workbench state — and
    /// they disagreed about the noise sheet: two said `viewer-bode`, the
    /// Studio's own said `viewer-spectrum`, which the catalog rejects for a
    /// `noise` analysis. Whether a noise sheet could be pinned into a
    /// document depended on which path asked.
    pub(crate) const fn viewer_document_id(self) -> Option<&'static str> {
        Some(match self {
            ResultViewer::Manifest
            | ResultViewer::Events
            | ResultViewer::Soa
            | ResultViewer::Reliability
            | ResultViewer::Optimization => return None,
            ResultViewer::Waves | ResultViewer::DcSweep => "viewer-waveform",
            ResultViewer::Bode | ResultViewer::Nyquist | ResultViewer::NoiseContrib => {
                "viewer-bode"
            }
            ResultViewer::Fft | ResultViewer::HarmonicBalance => "viewer-spectrum",
            ResultViewer::PhaseNoise => "viewer-phase-noise",
            ResultViewer::Eye => "eye-viewer",
            ResultViewer::Hist => "viewer-histogram",
            ResultViewer::Op | ResultViewer::Specs | ResultViewer::Table => "viewer-table",
            ResultViewer::Contribution => "viewer-contribution",
            ResultViewer::TransferFunction => "viewer-transfer-function",
            ResultViewer::Smith => "viewer-smith",
            ResultViewer::PoleZero => "viewer-pz",
        })
    }

    /// The sheet that draws a retained pane bound to this viewer document.
    ///
    /// The inverse of [`Self::viewer_document_id`] is not one-to-one — three
    /// sheets render `viewer-table` — so the choice has to be made once, here.
    /// It was made twice instead, and the two answers differed: the persistent
    /// document layer drew the Table sheet and the Studio drew Specs, so the
    /// same pane showed exact samples in one surface and a specification matrix
    /// in the other. Table is the truthful answer, because
    /// `renderer_supports_analysis` admits a `viewer-table` pane on retained
    /// waveforms — which is what Table reads and what Specs does not.
    pub(crate) fn from_viewer_document_id(id: &str) -> Option<Self> {
        Some(match id {
            "viewer-waveform" => ResultViewer::Waves,
            "viewer-bode" => ResultViewer::Bode,
            "viewer-spectrum" => ResultViewer::Fft,
            "viewer-phase-noise" => ResultViewer::PhaseNoise,
            "viewer-smith" => ResultViewer::Smith,
            "viewer-table" => ResultViewer::Table,
            "viewer-histogram" => ResultViewer::Hist,
            "eye-viewer" => ResultViewer::Eye,
            "viewer-pz" => ResultViewer::PoleZero,
            "viewer-contribution" => ResultViewer::Contribution,
            "viewer-transfer-function" => ResultViewer::TransferFunction,
            _ => return None,
        })
    }

    const fn tab_icon(self) -> WorkbenchIcon {
        match self {
            ResultViewer::Waves | ResultViewer::DcSweep | ResultViewer::NoiseContrib => {
                WorkbenchIcon::Results
            }
            ResultViewer::Bode
            | ResultViewer::Fft
            | ResultViewer::HarmonicBalance
            | ResultViewer::PhaseNoise
            | ResultViewer::Hist
            | ResultViewer::Contribution => WorkbenchIcon::Results,
            ResultViewer::Eye
            | ResultViewer::Nyquist
            | ResultViewer::Smith
            | ResultViewer::PoleZero => WorkbenchIcon::Target,
            ResultViewer::Op
            | ResultViewer::TransferFunction
            | ResultViewer::Specs
            | ResultViewer::Soa
            | ResultViewer::Reliability
            | ResultViewer::Events
            | ResultViewer::Table => WorkbenchIcon::Grid,
            ResultViewer::Optimization => WorkbenchIcon::Results,
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

/// A viewport gesture on the active sheet, as asked for by a command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ViewGesture {
    /// Drop every pinned viewport on the active plot.
    Fit,
    /// Halve the visible interval about its centre.
    ZoomIn,
    /// Double the visible interval about its centre.
    ZoomOut,
}

/// Whether a viewer draws through the shared unit-pane waveform stack.
///
/// Those sheets key their viewports by analysis; every other viewer is a
/// single canvas keyed by plot ordinal. A gesture that does not ask this
/// question first will write to a store the sheet never reads.
pub(crate) const fn viewer_uses_wave_stack(viewer: ResultViewer) -> bool {
    matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::DcSweep
            | ResultViewer::Bode
            | ResultViewer::NoiseContrib
    )
}

/// Queue a viewport gesture for the active result sheet.
pub(crate) fn request_view_gesture(state: &mut AppState, gesture: ViewGesture) {
    state.ui.results.pending_view_gesture = Some(gesture);
}

/// Whether the active sheet has a viewport that fitting can release.
///
/// Every plot sheet does; the structured documents (OP, specs, table, XF,
/// manifest) have no viewport at all and must not offer the gesture.
pub(crate) fn fit_gesture_available(state: &AppState) -> bool {
    state.simulation.has_results() && viewer_draws_a_pane(state.ui.results.viewer)
}

/// Whether this sheet draws a plot pane at all.
///
/// The evidence tables (OP, specs, sample table, event history, manifest) and
/// the scalar transfer-function readout draw no axes, so they have no
/// viewport, no fit, and no limit mask to report.
pub(crate) const fn viewer_draws_a_pane(viewer: ResultViewer) -> bool {
    !matches!(
        viewer,
        ResultViewer::Op
            | ResultViewer::Specs
            | ResultViewer::Table
            | ResultViewer::Events
            | ResultViewer::TransferFunction
            | ResultViewer::Manifest
    )
}

/// Whether the active sheet can be magnified about its centre.
///
/// Only the unit-pane stack exposes the retained extents a zoom step has to
/// be computed against; the single-canvas viewers own their own gestures.
pub(crate) fn zoom_gesture_available(state: &AppState) -> bool {
    state.simulation.has_results() && viewer_uses_wave_stack(state.ui.results.viewer)
}

/// Apply any queued viewport gesture, now that the sheet's models and theme
/// tokens exist.
fn apply_pending_view_gesture(ui: &Ui, state: &mut AppState) {
    let Some(gesture) = state.ui.results.pending_view_gesture.take() else {
        return;
    };
    let viewer = state.ui.results.viewer;
    if !viewer_uses_wave_stack(viewer) {
        // Single-canvas viewers keep their views under plot ordinals, and
        // only fit is expressible without their renderer's own extents.
        if gesture == ViewGesture::Fit {
            state.ui.results.reset_viewer_plot_views(viewer);
        }
        return;
    }
    match gesture {
        ViewGesture::Fit => waves::fit_active_strip(state),
        ViewGesture::ZoomIn => waves::zoom_active_pane(state, &Tokens::get(ui.ctx()), 0.5),
        ViewGesture::ZoomOut => waves::zoom_active_pane(state, &Tokens::get(ui.ctx()), 2.0),
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
    /// Y range override.
    pub y: Option<(f64, f64)>,
}

impl PlotView {
    /// Whether any axis is zoomed away from the automatic view.
    pub fn is_zoomed(&self) -> bool {
        self.x.is_some() || self.y.is_some()
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
    /// Every kind a marker may be given.
    pub const ALL: [MarkerKind; 3] = [MarkerKind::Note, MarkerKind::Peak, MarkerKind::Spec];

    /// Short label used on the chip and in the marker list.
    pub const fn label(self) -> &'static str {
        match self {
            MarkerKind::Note => "note",
            MarkerKind::Peak => "peak",
            MarkerKind::Spec => "spec",
        }
    }

    /// What choosing this kind asserts, spelled out in the edit dialog.
    pub const fn dialog_label(self) -> &'static str {
        match self {
            MarkerKind::Note => "Note — a remark about this point on the curve",
            MarkerKind::Peak => "Peak — a called-out extremum or feature",
            MarkerKind::Spec => "Spec — a limit line the design is measured against",
        }
    }

    /// A spec marker constrains the X position alone and so carries no
    /// trace value in the readout.
    pub const fn rides_a_trace(self) -> bool {
        !matches!(self, MarkerKind::Spec)
    }
}

/// An uncommitted edit of one marker's purpose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MarkerEditDraft {
    pub id: u32,
    pub note: String,
    pub kind: MarkerKind,
}

/// A marker anchor naming one retained signal's plain value projection.
///
/// Production anchors come from the drawn trace, which carries the family
/// discriminator this cannot know; this exists so persistence tests can name
/// a signal without standing up a whole strip.
#[cfg(test)]
pub(crate) fn marker_anchor_for(
    analysis: AnalysisPresentationKey,
    source_name: &str,
) -> WaveformPresentationKey {
    WaveformPresentationKey {
        analysis,
        trace: TracePresentationKey {
            source_name: source_name.to_owned(),
            kind: 0,
            family_group: 0,
        },
    }
}

/// Restore the markers a project retained, dropping any whose analysis is not
/// in the reopened datasets.
pub(crate) fn restore_markers(state: &mut AppState, markers: Vec<ResultMarker>) {
    let retained: Vec<ResultMarker> = markers
        .into_iter()
        .filter(|marker| {
            state
                .simulation
                .runs
                .iter()
                .any(|run| marker.analysis.resolve(run).is_some())
        })
        .collect();
    state.ui.results.adopt_markers(retained);
}

/// Restore the reader's logarithmic-axis pane choices, dropping any whose
/// analysis this project no longer retains.
///
/// Same rule as the markers above: a presentation decision that cannot find
/// the dataset it was made about is not a decision about anything.
pub(crate) fn restore_log_y_panes(state: &mut AppState, panes: Vec<WavePanePresentationKey>) {
    state.ui.results.log_y_panes = panes
        .into_iter()
        .filter(|pane| {
            state
                .simulation
                .runs
                .iter()
                .any(|run| pane.analysis.resolve(run).is_some())
        })
        .collect();
}

/// A user-placed marker on a waveform strip.
///
/// The anchor is a *signal*, not one solve of it: `y` is resampled from the
/// trace every frame, so a marker survives zoom, pan, and retained-vector
/// reordering without drifting onto a different dataset or curve.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultMarker {
    /// Stable per-project id. Renders as `M{id}`.
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WavePanePresentationKey {
    pub analysis: AnalysisPresentationKey,
    pub unit: String,
}

/// One horizontal measurement cursor bound to an exact waveform pane.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct HorizontalWaveCursor {
    pub pane: WavePanePresentationKey,
    pub y: f64,
}

/// Which safe-operating-area rules the SOA table lists.
///
/// Attention and passing are complements, so a rule is in exactly one of them
/// and the two counts always sum to the evaluated total.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum SoaRuleFilter {
    #[default]
    All,
    Violations,
    Passing,
}

/// One SOA rule picked out of the evidence table.
///
/// A rule is identified by its device and the stressed parameter, never by row
/// ordinal: the filter reorders the table and the analysis may be re-run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SoaRuleSelection {
    pub analysis: AnalysisPresentationKey,
    pub device_id: String,
    pub parameter: SoaParameterEvidence,
}

/// One ageing checkpoint picked out of the reliability table.
///
/// The checkpoint year is held as raw bits so the selection compares exactly
/// against the retained value instead of through a float tolerance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReliabilitySelection {
    pub analysis: AnalysisPresentationKey,
    pub device_id: String,
    pub checkpoint_year_bits: u64,
}

/// One optimizer candidate picked out of the iteration history.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OptimizationSelection {
    pub analysis: AnalysisPresentationKey,
    pub iteration_index: usize,
}

/// Where one row of the event history came from.
///
/// Exact rows are the schedule the event solver committed. Projected rows are
/// reconstructed from an older project's `D(..)`/`E(..)` waveforms, which were
/// sampled on the analog grid — the distinction is reported, never hidden,
/// because a projected time is an approximation of the real one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EventSelectionSource {
    ExactDigital,
    ExactReal,
    ProjectedDigital,
    ProjectedReal,
}

/// One event row's position in the merged, time-ordered history.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct EventOrderEntry {
    pub source: EventSelectionSource,
    pub trace_index: usize,
    pub point_index: usize,
    pub time_s: f64,
    pub initial: bool,
}

/// The merged event order for one analysis.
///
/// Built once per analysis rather than per frame: merging every node's
/// schedule is O(events log events), and the answer only changes when the
/// analysis does.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EventOrderCache {
    pub analysis: AnalysisPresentationKey,
    /// Whether the rows are the committed schedule rather than a projection.
    pub exact: bool,
    pub rows: Vec<EventOrderEntry>,
}

/// One event picked out of the history.
///
/// Addressed by node name rather than trace ordinal so the selection survives
/// a re-run that registers event nodes in a different order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DigitalEventSelection {
    pub analysis: AnalysisPresentationKey,
    pub source: EventSelectionSource,
    pub trace_name: String,
    pub point_index: usize,
}

/// Whether one analysis' retained evidence passes its own validator.
///
/// Memoized per analysis because the answer walks the whole payload and only
/// changes when the datasets do; `prepare_viewer_state` clears the memo on a
/// new data version.
pub(crate) fn retained_evidence_is_valid(
    state: &mut AppState,
    analysis: AnalysisPresentationKey,
) -> bool {
    if let Some(known) = state.ui.results.retained_evidence_validity.get(&analysis) {
        return *known;
    }
    let valid = state
        .simulation
        .runs
        .iter()
        .find_map(|run| analysis.resolve(run))
        .is_some_and(|(_, resolved)| resolved.validate_retained_evidence().is_ok());
    state
        .ui
        .results
        .retained_evidence_validity
        .insert(analysis, valid);
    valid
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
    /// Signals starred in the results data browser. A deliberate,
    /// session-scoped mark like `markers`, keyed by retained waveform name:
    /// the browser's Favorites scope reads exactly this set.
    pub(crate) favorite_signals: std::collections::BTreeSet<String>,
    /// Most-recent-first waveform names the user selected or revealed,
    /// deduplicated and bounded: the browser's Recent scope is this order.
    pub(crate) recent_signals: Vec<String>,
    /// Quantities check-marked in the browser for a batch action. Session
    /// state like the marks above it; the immutable dataset never sees it.
    pub(crate) checked_signals: std::collections::BTreeSet<String>,
    /// Id allocator for `markers`. Monotonic within a project so a marker
    /// label never silently changes meaning after a deletion or a reload.
    next_marker_id: u32,
    /// The open marker-purpose dialog's uncommitted edit, if any.
    ///
    /// Editing is transactional: nothing reaches the marker until Apply, so
    /// a half-typed label can always be abandoned. Reclassifying a marker is
    /// a decision, not a side effect of clicking its kind.
    pub(super) marker_edit: Option<MarkerEditDraft>,
    /// Whether the stage-level cursor/marker dock is collapsed. Session-only
    /// presentation state; retained result documents never serialize it.
    pub readout_collapsed: bool,
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
    /// Unit-scoped panes the user put on a logarithmic Y axis.
    ///
    /// This lived in egui's persisted memory under a hand-built id, which put
    /// a project-scoped presentation decision outside every owner that knows
    /// about projects: `clear_project_scoped_state` could not reach it, the
    /// project file could not carry it, and it accumulated an entry for every
    /// dataset the user ever opened. It is the same class of fact as a marker
    /// and it is kept the same way.
    pub(crate) log_y_panes: HashSet<WavePanePresentationKey>,
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
    /// What each single-canvas sheet's axes actually spanned when it last
    /// drew, pinned or fitted.
    ///
    /// The axis-limit editor has to open on the interval the reader is
    /// looking at, and an unpinned sheet's interval is derived inside the
    /// sheet from its own data. The waveform stack is deliberately absent:
    /// it reports its panes through `active_pane_facts`, which knows which
    /// of several panes is active — something a last-drawn record cannot.
    /// Transient.
    pub(crate) drawn_axes: std::collections::HashMap<ResultViewer, DrawnAxes>,
    /// A viewport gesture asked for from outside the drawing pass.
    ///
    /// Menus, the command palette and shortcuts all reach the workspace
    /// without an `egui::Context`, and resolving which pane a gesture applies
    /// to needs the sheet's built models and theme tokens. Queueing here and
    /// applying inside the viewer well keeps one owner for the gesture
    /// instead of a second, token-free approximation of pane resolution.
    pending_view_gesture: Option<ViewGesture>,
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
    /// The axis-limit field being typed, if any.
    ///
    /// Editing is transactional like the marker dialog: the pinned interval
    /// only moves on commit, so a half-typed bound never rescales the plot
    /// under the reader's hands. Transient.
    pub(crate) axis_limit_draft: Option<(ResultViewer, PaneAxis, String)>,
    /// Open spec-editor rows (None = matrix view). Transient.
    pub spec_drafts: Option<Vec<specs::SpecDraft>>,
    /// SOA rule whose evidence the inspector reports. Transient.
    pub(super) selected_soa_rule: Option<SoaRuleSelection>,
    /// Verdict filter applied to the SOA rule table. Transient.
    pub(super) soa_rule_filter: SoaRuleFilter,
    /// Whether the selected SOA rule's stress history is drawn above the
    /// table. Off by default: the table is the evidence, the trace is the
    /// follow-up question.
    pub(super) soa_stress_trace_open: bool,
    /// Ageing checkpoint whose retained shift the inspector reports.
    /// Transient.
    pub(super) selected_reliability: Option<ReliabilitySelection>,
    /// Optimizer candidate whose retained cost and variables the inspector
    /// reports. Transient.
    pub(super) selected_optimization: Option<OptimizationSelection>,
    /// Event whose exact value and provenance the inspector reports.
    /// Transient.
    pub(super) selected_digital_event: Option<DigitalEventSelection>,
    /// Merged event order for the analysis the EVENTS sheet last drew.
    pub(super) event_order_cache: Option<EventOrderCache>,
    /// Memoized retained-evidence verdict per analysis; see
    /// [`retained_evidence_is_valid`].
    pub(super) retained_evidence_validity: HashMap<AnalysisPresentationKey, bool>,
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

    /// Remove one marker, and with it any open edit of it.
    pub fn remove_marker(&mut self, id: u32) {
        self.markers.retain(|marker| marker.id != id);
        if self
            .marker_edit
            .as_ref()
            .is_some_and(|draft| draft.id == id)
        {
            self.marker_edit = None;
        }
    }

    /// Adopt markers restored from a project, keeping the id allocator ahead
    /// of every label already in use.
    pub(crate) fn adopt_markers(&mut self, markers: Vec<ResultMarker>) {
        self.next_marker_id = markers
            .iter()
            .map(|marker| marker.id)
            .max()
            .unwrap_or(0)
            .max(self.next_marker_id);
        self.markers = markers;
        self.marker_edit = None;
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

    /// Flip one signal's membership in the browser's Favorites scope.
    pub(crate) fn toggle_favorite_signal(&mut self, name: &str) {
        if !self.favorite_signals.remove(name) {
            self.favorite_signals.insert(name.to_owned());
        }
    }

    pub(crate) fn is_favorite_signal(&self, name: &str) -> bool {
        self.favorite_signals.contains(name)
    }

    /// Record a deliberate signal interaction for the Recent scope: front
    /// insertion, deduplicated, bounded so the scope stays a shortlist.
    pub(crate) fn note_recent_signal(&mut self, name: &str) {
        const RECENT_SIGNAL_CAP: usize = 24;
        self.recent_signals.retain(|recent| recent != name);
        self.recent_signals.insert(0, name.to_owned());
        self.recent_signals.truncate(RECENT_SIGNAL_CAP);
    }

    /// Position in the Recent shortlist; `None` when never noted.
    pub(crate) fn recent_signal_rank(&self, name: &str) -> Option<usize> {
        self.recent_signals.iter().position(|recent| recent == name)
    }

    pub(crate) fn toggle_checked_signal(&mut self, name: &str) {
        if !self.checked_signals.remove(name) {
            self.checked_signals.insert(name.to_owned());
        }
    }

    pub(crate) fn is_checked_signal(&self, name: &str) -> bool {
        self.checked_signals.contains(name)
    }

    pub(crate) fn clear_checked_signals(&mut self) {
        self.checked_signals.clear();
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

    /// Drop every pinned viewport this sheet owns.
    ///
    /// A single-canvas sheet is not always a single *plot*: the ageing sheet
    /// draws a degradation curve and a lifetime curve under ordinals 0 and 1.
    /// A Fit that reset only ordinal 0 would leave half the sheet zoomed.
    pub(crate) fn reset_viewer_plot_views(&mut self, viewer: ResultViewer) {
        self.views
            .retain(|(key_viewer, _, _), _| *key_viewer != viewer);
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

    /// Drop every analysis-keyed viewport override of one viewer.
    pub(super) fn reset_all_analysis_plot_views(&mut self, viewer: ResultViewer) {
        self.views.retain(|(key_viewer, key_plot, _), _| {
            *key_viewer != viewer || !matches!(key_plot, PlotPresentationKey::Analysis(_))
        });
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

    /// Whether any pane of one strip pins the given axis.
    pub(super) fn analysis_strip_axis_is_pinned(
        &self,
        viewer: ResultViewer,
        analysis: AnalysisPresentationKey,
        axis: PaneAxis,
    ) -> bool {
        let plot = PlotPresentationKey::Analysis(analysis);
        self.views.iter().any(|((key_viewer, key_plot, _), view)| {
            (*key_viewer, *key_plot) == (viewer, plot)
                && match axis {
                    PaneAxis::X => view.x.is_some(),
                    PaneAxis::Y => view.y.is_some(),
                }
        })
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
    pub(crate) x_range_bits: [u64; 2],
    pub(crate) y_range_bits: [u64; 2],
    pub(crate) handle: egui::TextureHandle,
}

impl std::fmt::Debug for EyeTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EyeTexture")
            .field("revision", &self.revision)
            .field("size", &self.size)
            .field("x_range_bits", &self.x_range_bits)
            .field("y_range_bits", &self.y_range_bits)
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

    /// Key-space bit separating unwrapped-phase entries from the dB entries,
    /// which share the `(analysis << 32 | waveform)` key convention.
    const UNWRAP_KEY_BIT: u64 = 1 << 62;

    /// Key-space bit for nV/√Hz noise-density projections of retained
    /// V²/Hz PSD series, sharing the same key convention.
    const NOISE_DENSITY_KEY_BIT: u64 = 1 << 61;

    /// nV/√Hz spectral density of a retained V²/Hz PSD series, cached
    /// under `key` like `db`. Negative retained samples clamp to zero
    /// rather than inventing NaNs the plot would silently drop.
    pub fn noise_density_nv(&mut self, key: u64, psd_v2_per_hz: &[f64]) -> SharedWaveformValues {
        self.get_or(Self::NOISE_DENSITY_KEY_BIT | key, || {
            std::sync::Arc::new(
                psd_v2_per_hz
                    .iter()
                    .map(|&value| 1.0e9 * value.max(0.0).sqrt())
                    .collect::<Vec<_>>(),
            )
        })
    }

    /// Continuous (unwrapped) copy of a ±180°-wrapped phase-degree series,
    /// cached under `key` like `db`.
    pub fn unwrapped(&mut self, key: u64, phase_deg: &[f64]) -> SharedWaveformValues {
        self.get_or(Self::UNWRAP_KEY_BIT | key, || {
            std::sync::Arc::new(crate::analysis::calculator::functions::unwrap_phase_deg(
                phase_deg,
            ))
        })
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

/// Couple an equal-aspect XY viewer's navigation ranges so wheel zoom,
/// axis-constrained pan, and zoom boxes cannot distort circles or root maps.
pub(super) fn square_xy_view_change(
    current_x: (f64, f64),
    current_y: (f64, f64),
    change: crate::ui::plot::ViewChange,
) -> crate::ui::plot::ViewChange {
    if change.reset || (change.x.is_none() && change.y.is_none()) {
        return change;
    }
    let next_x = change.x.unwrap_or(current_x);
    let next_y = change.y.unwrap_or(current_y);
    let x_span = (next_x.1 - next_x.0).abs();
    let y_span = (next_y.1 - next_y.0).abs();
    let span = match (change.x, change.y) {
        (Some(_), None) => x_span,
        (None, Some(_)) => y_span,
        (Some(_), Some(_)) => x_span.max(y_span),
        (None, None) => unreachable!("empty view changes return above"),
    }
    .max(f64::EPSILON);
    let x_center = (next_x.0 + next_x.1) * 0.5;
    let y_center = (next_y.0 + next_y.1) * 0.5;
    crate::ui::plot::ViewChange {
        x: Some((x_center - span * 0.5, x_center + span * 0.5)),
        y: Some((y_center - span * 0.5, y_center + span * 0.5)),
        reset: false,
    }
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
    results_keymap(ui, app);
    show_with_chrome(ui, app, ResultChrome::Full);
    waves::marker_dialog::show(ui.ctx(), &mut app.state);
    create_document::show(ui.ctx(), app);
}

/// The workspace's own single-key gestures, as the mockup's results keymap
/// defines them: fit, zoom, grid, cursor tool, drop a trace, nudge a cursor.
///
/// Two kinds of key meet here. Fit, zoom and grid are commands, and they are
/// claimed through the chord the user's own profile shows for them — their
/// registry context is the engineering canvas, which never becomes active on
/// this workspace, so the resolver would never fire them here. The rest are
/// view gestures no menu carries and no command names.
///
/// Everything is a bare key, so the map stands down whenever a widget wants
/// the keyboard or a modifier is held: nothing here may fire while the reader
/// is typing into a filter, an expression, or a marker label.
fn results_keymap(ui: &Ui, app: &mut RSpiceApp) {
    let ctx = ui.ctx();
    // A focused widget owns the keyboard: a filter, an expression, or a
    // marker label must never have a letter stolen from it by this map.
    if ctx.memory(|memory| memory.focused().is_some()) {
        return;
    }
    // Nor may the map fire underneath a dialog.
    if app.state.ui.results.marker_edit.is_some() {
        return;
    }
    if !app.state.simulation.has_results() {
        return;
    }
    let plain = ctx
        .input(|input| !input.modifiers.command && !input.modifiers.alt && !input.modifiers.ctrl);
    if !plain {
        return;
    }

    // Actions that already exist as commands keep the chord the rest of the
    // product shows for them, read from the user's own profile, so rebinding
    // one in preferences rebinds it here. Their registry context is the
    // engineering canvas and never becomes active on this workspace, which is
    // why the keystroke has to be claimed here rather than left to the
    // resolver.
    for (command, gesture) in [
        (
            crate::workbench::commands::vocabulary::Command::ZoomFit,
            ViewGesture::Fit,
        ),
        (
            crate::workbench::commands::vocabulary::Command::ZoomIn,
            ViewGesture::ZoomIn,
        ),
        (
            crate::workbench::commands::vocabulary::Command::ZoomOut,
            ViewGesture::ZoomOut,
        ),
    ] {
        let permitted = match gesture {
            ViewGesture::Fit => fit_gesture_available(&app.state),
            ViewGesture::ZoomIn | ViewGesture::ZoomOut => zoom_gesture_available(&app.state),
        };
        if permitted && consume_command_key(ctx, &app.state, command) {
            request_view_gesture(&mut app.state, gesture);
        }
    }
    if viewer_uses_wave_stack(app.state.ui.results.viewer)
        && consume_command_key(
            ctx,
            &app.state,
            crate::workbench::commands::vocabulary::Command::CycleGrid,
        )
    {
        app.state.ui.results.show_minor_grid = !app.state.ui.results.show_minor_grid;
    }

    if ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::C)) {
        app.state.ui.results.toggle_cursor_tool();
    }
    if ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Delete)) {
        hide_selected_trace(&mut app.state);
    }
    // Arrow nudging is a cursor gesture, so it only exists while cursors do.
    if app.state.ui.results.cursor_readout_active() {
        let shift = ctx.input(|input| input.modifiers.shift);
        let modifiers = if shift {
            egui::Modifiers::SHIFT
        } else {
            egui::Modifiers::NONE
        };
        let steps = if ctx.input_mut(|input| input.consume_key(modifiers, egui::Key::ArrowRight)) {
            1.0
        } else if ctx.input_mut(|input| input.consume_key(modifiers, egui::Key::ArrowLeft)) {
            -1.0
        } else {
            0.0
        };
        if steps != 0.0 {
            let t = Tokens::get(ctx);
            waves::nudge_cursor(&mut app.state, &t, shift, steps);
        }
    }
}

/// Claim the modifier-free keystroke the profile binds to `command`.
///
/// Only bare or shift-only chords are claimed: these are viewport gestures
/// pressed with one hand over the plot. A profile that moves the command onto
/// a modifier chord simply leaves this workspace without a key for it — the
/// menu entry is still the way in, and no key is silently invented.
fn consume_command_key(
    ctx: &egui::Context,
    state: &AppState,
    command: crate::workbench::commands::vocabulary::Command,
) -> bool {
    let platform = crate::workbench::app_state::runtime_command_platform(ctx);
    state
        .ui
        .preferences
        .shortcuts()
        .effective_bindings(command)
        .iter()
        .filter(|binding| binding.supports(platform))
        .filter_map(|binding| {
            let strokes = binding.sequence().strokes();
            let [stroke] = strokes else { return None };
            (!stroke.primary() && !stroke.alt()).then_some((stroke.key(), stroke.shift()))
        })
        .any(|(key, shift)| {
            let modifiers = if shift {
                egui::Modifiers::SHIFT
            } else {
                egui::Modifiers::NONE
            };
            ctx.input_mut(|input| input.consume_key(modifiers, key))
        })
}

/// Take the selected trace off the sheet, the way the mockup's Delete does.
///
/// Hiding is the reversible form of removal here: the retained dataset is
/// immutable, so a trace leaves the plot by losing visibility and comes back
/// from the pane's add-signal menu.
fn hide_selected_trace(state: &mut AppState) {
    let Some(selected) = state.ui.results.valid_selected_trace(&state.simulation) else {
        return;
    };
    let analysis_key = selected.analysis_key();
    let source_name = selected.source_name().to_owned();
    let Some(run) = state.simulation.active_run() else {
        return;
    };
    let located = run
        .analyses
        .iter()
        .enumerate()
        .find_map(|(index, analysis)| {
            (AnalysisPresentationKey::new(run.dataset_id, analysis) == analysis_key).then(|| {
                analysis
                    .waveforms
                    .iter()
                    .position(|waveform| waveform.name == source_name)
                    .map(|waveform_index| (index, waveform_index))
            })?
        });
    if let Some((analysis_index, waveform_index)) = located {
        waves::toggle_visibility(state, analysis_index, waveform_index);
        state.ui.results.selected_trace = None;
    }
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
    // A project-owned document is the same workspace with a retained page
    // layout, so it answers the same keys and owns the same marker dialog.
    results_keymap(ui, app);
    persistent_document::show(ui, app, document_id);
    waves::marker_dialog::show(ui.ctx(), &mut app.state);
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
            if result_stage_bar_visible(&app.state) {
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

    // The stage bottom holds one bounded cursor/marker dock. Its header and
    // owning scroll body are content-fit, so a stage with nothing to report
    // gives the whole area back to the document.
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

/// Whether the active viewer owns a stage-local bar below the viewer tabs.
///
/// Plot and custom-canvas sheets use the 31 px instrument/purpose bar. OP,
/// Specs, and Table use the mockup's 40 px structured-document strip. XF and
/// Manifest own no controls there and therefore collapse the row completely.
const fn viewer_has_sheet_bar(viewer: ResultViewer) -> bool {
    !matches!(
        viewer,
        ResultViewer::TransferFunction | ResultViewer::Manifest
    )
}

const fn viewer_has_structured_strip(viewer: ResultViewer) -> bool {
    matches!(
        viewer,
        ResultViewer::Op | ResultViewer::Specs | ResultViewer::Table
    )
}

fn result_stage_bar_visible(state: &AppState) -> bool {
    state
        .simulation
        .active_run()
        .is_some_and(|run| !run.analyses.is_empty())
        && viewer_has_sheet_bar(state.ui.results.viewer)
}

/// Height of the stage's readout strip for the active viewer, or zero.
///
/// Only the cursor-bearing waveform viewers carry a readout; structured
/// documents (OP, specs, tables) have no cursor to report.
fn readout_strip_height(state: &AppState) -> f32 {
    match state.ui.results.viewer {
        ResultViewer::Waves
        | ResultViewer::DcSweep
        | ResultViewer::Bode
        | ResultViewer::NoiseContrib => waves::readout_strip_height(state),
        _ => 0.0,
    }
}

/// The sheet a bound pane actually draws, once the analysis behind it is known.
///
/// Several viewer documents cover a pair of sheets that differ only by what the
/// analysis turned out to be — one waveform renderer serves transient and DC
/// sweeps, one spectrum renderer serves FFT and harmonic balance, one AC
/// renderer serves Bode and ordinary noise. Which of the pair to draw is one
/// question, so it is answered once: the persistent-document layer and the
/// Studio each answered it separately and the Studio's copy was missing the
/// noise rule, so the same pane drew the noise spectrum in a result document
/// and reported an unsatisfiable Bode contract in the Studio.
pub(crate) fn project_viewer_for_analysis(
    viewer: ResultViewer,
    analysis: &crate::state::AnalysisResult,
) -> ResultViewer {
    use crate::state::AnalysisType;

    match viewer {
        ResultViewer::Waves if analysis.analysis_type == AnalysisType::DcSweep => {
            ResultViewer::DcSweep
        }
        ResultViewer::Fft if analysis.analysis_type == AnalysisType::HarmonicBalance => {
            ResultViewer::HarmonicBalance
        }
        ResultViewer::Bode if bode::ordinary_noise_spectrum_is_renderable(analysis) => {
            ResultViewer::NoiseContrib
        }
        _ => viewer,
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

pub(crate) fn prepare_viewer_state(app: &mut RSpiceApp) {
    let data_version = app.state.simulation.data_version;
    // The user's display-cache budget is session state, and the cache is not,
    // so applying it only where the setting is edited left a restored session
    // running on the default until the reader happened to reopen that panel.
    // Reasserting it each frame keeps the two in step by construction; it
    // costs a comparison unless the budget actually shrank.
    let budget = app.state.workbench.visualization_studio.tile_memory_mib;
    let results = &mut app.state.ui.results;
    results.cache.set_memory_budget_mib(budget);
    if results.seen_version != data_version {
        results.seen_version = data_version;
        results.clear_cursors();
        results.horizontal_cursor = None;
        results.active_wave_pane = None;
        results.selected_trace = None;
        // Pinned XY readouts index into the old run's point arrays;
        // a same-shape new run would silently relabel them.
        results.rf_pin.clear();
        // Both are derived from the retained datasets, so a new version makes
        // them answers to an old question.
        results.retained_evidence_validity.clear();
        results.event_order_cache = None;
    }
    results.cache.ensure_version(data_version);
    results.derived.ensure_version(data_version);

    reconcile_active_viewer(&mut app.state);
}

/// Record what a single-canvas sheet's axes just spanned.
///
/// Called by the sheet that drew, because only it knows which plot on the
/// sheet is the one the inspector speaks for.
pub(super) fn record_drawn_axes(
    results: &mut ResultsState,
    viewer: ResultViewer,
    response: &crate::ui::plot::PlotResponse,
) {
    results.drawn_axes.insert(viewer, response.axes);
}

/// Which axis an explicit range applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PaneAxis {
    X,
    Y,
}

/// The interval the active sheet's axis is showing, pinned or fitted.
pub(crate) fn active_axis_range(
    state: &AppState,
    facts: &waves::ActivePaneFacts,
    axis: PaneAxis,
) -> Option<(f64, f64)> {
    let viewer = state.ui.results.viewer;
    if viewer_uses_wave_stack(viewer) {
        return match axis {
            PaneAxis::X => facts.x_extent,
            PaneAxis::Y => facts.y_extent,
        };
    }
    let drawn = state.ui.results.drawn_axes.get(&viewer).copied();
    let pinned = state.ui.results.plot_view(viewer, 0);
    match axis {
        PaneAxis::X => pinned.x.or_else(|| drawn.map(|(x, _)| x)),
        PaneAxis::Y => pinned.y.or_else(|| drawn.map(|(_, y)| y)),
    }
}

/// Whether the active sheet's axis is pinned to an explicit interval rather
/// than fitting its data.
pub(crate) fn active_axis_is_pinned(state: &AppState, axis: PaneAxis) -> bool {
    let viewer = state.ui.results.viewer;
    if viewer_uses_wave_stack(viewer) {
        return waves::active_pane_axis_is_pinned(&state.ui.results, axis);
    }
    let pinned = state.ui.results.plot_view(viewer, 0);
    match axis {
        PaneAxis::X => pinned.x.is_some(),
        PaneAxis::Y => pinned.y.is_some(),
    }
}

/// Render an interval so the field it came from can read it back without loss.
pub(crate) fn format_axis_range((minimum, maximum): (f64, f64)) -> String {
    format!(
        "{} … {}",
        format_axis_bound(minimum),
        format_axis_bound(maximum)
    )
}

fn format_axis_bound(value: f64) -> String {
    if value == 0.0 {
        "0".to_owned()
    } else if (1.0e-3..1.0e6).contains(&value.abs()) {
        format!("{value:.7}")
    } else {
        format!("{value:.7e}")
    }
}

/// Read an interval typed as `min … max`, in engineering notation.
///
/// Accepts the ellipsis [`format_axis_range`] writes back, plus `..`, a comma
/// or plain space, so a pair copied out of a datasheet or a netlist parses
/// without being reformatted first.
///
/// A reversed or degenerate interval is refused rather than quietly sorted.
/// On a logarithmic axis those are not the same request, and silently
/// swapping the bounds would hide a typo that changes what the reader sees.
pub(crate) fn parse_axis_range(text: &str) -> Option<(f64, f64)> {
    let cleaned = text.replace('…', " ").replace("..", " ").replace(',', " ");
    let mut parts = cleaned.split_whitespace();
    let minimum = crate::quantity::engineering::parse_engineering_value(parts.next()?).ok()?;
    let maximum = crate::quantity::engineering::parse_engineering_value(parts.next()?).ok()?;
    if parts.next().is_some() {
        return None;
    }
    (minimum.is_finite() && maximum.is_finite() && maximum > minimum).then_some((minimum, maximum))
}

/// Pin the active sheet's axis to an explicit interval, or clear it back to
/// automatic fit.
///
/// Routes on [`viewer_uses_wave_stack`] for the same reason every other
/// viewport gesture does: the stack keys its viewports by analysis, and a
/// write to the single-canvas store would land where the sheet never reads.
pub(crate) fn set_active_axis_range(
    tokens: &Tokens,
    state: &mut AppState,
    axis: PaneAxis,
    range: Option<(f64, f64)>,
) -> bool {
    let viewer = state.ui.results.viewer;
    if viewer_uses_wave_stack(viewer) {
        return waves::set_active_pane_axis_range(tokens, state, axis, range);
    }
    let view = state
        .ui
        .results
        .plot_view_pane_mut_for(viewer, PlotPresentationKey::Global(0), 0);
    match axis {
        PaneAxis::X => view.x = range,
        PaneAxis::Y => view.y = range,
    }
    true
}

fn show_viewer_well(ui: &mut Ui, app: &mut RSpiceApp, chrome: ResultChrome) {
    apply_pending_view_gesture(ui, &mut app.state);
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
        ResultViewer::Waves | ResultViewer::DcSweep => match chrome {
            ResultChrome::Full => waves::show(ui, &mut app.state),
            ResultChrome::CompactSplit => waves::show_compact(ui, &mut app.state),
        },
        ResultViewer::Bode => waves::show_bode(ui, &mut app.state),
        ResultViewer::Fft => {
            if ensure_derived(ui, app, ActiveViewer::Fft) {
                fft::show(ui, &mut app.state);
            }
        }
        ResultViewer::HarmonicBalance => harmonic_balance::show(ui, &mut app.state),
        ResultViewer::PhaseNoise => phase_noise::show(ui, &mut app.state),
        ResultViewer::Eye => {
            if ensure_derived(ui, app, ActiveViewer::EyeDiagram) {
                eye::show(ui, &mut app.state);
            }
        }
        ResultViewer::Hist => hist::show(ui, &mut app.state),
        ResultViewer::Op => op_inspector::show(ui, &mut app.state),
        ResultViewer::NoiseContrib => waves::show_noise(ui, &mut app.state),
        ResultViewer::Contribution => sensitivity::show(ui, &mut app.state),
        ResultViewer::TransferFunction => transfer_function::show(ui, &mut app.state),
        ResultViewer::Specs => specs::show(ui, &mut app.state),
        ResultViewer::Table => table::show(ui, &mut app.state),
        ResultViewer::Nyquist => nyquist::show(ui, &mut app.state),
        ResultViewer::Smith => smith::show(ui, &mut app.state),
        ResultViewer::PoleZero => pz::show(ui, &mut app.state),
        ResultViewer::Events => events::show(ui, &mut app.state),
        ResultViewer::Soa => soa::show(ui, &mut app.state),
        ResultViewer::Reliability => reliability::show(ui, &mut app.state),
        ResultViewer::Optimization => optimization::show(ui, &mut app.state),
        ResultViewer::Manifest => manifest::show(ui, &app.state),
    }
}

/// Split projection of the mockup's viewer-tab row. Run binding, output
/// controls, result-document creation and properties remain owned by the full
/// Results workspace; every compatible existing viewer stays reachable here.
fn show_compact_docbar(ui: &mut Ui, state: &mut AppState) {
    docbar_at_height(ui, ResultBarMetrics::of(ui).viewer_tabs, |ui| {
        viewer_tab_scroller(ui, "rspice.results.split.viewer-tabs", |ui| {
            viewer_tabs(ui, state);
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
    docbar_at_height(ui, ResultBarMetrics::of(ui).viewer_tabs, |ui| {
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
                    viewer_tab_scroller(ui, "rspice.results.viewer-tabs", |ui| {
                        viewer_tabs_filtered(ui, &mut app.state, |viewer| {
                            family_label.is_none_or(|family| family_allows_viewer(family, viewer))
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

/// The Results shell own bar geometry, resolved against the pointer.
///
/// These were fixed numbers, so the workspace kept its workstation rows on a
/// tablet while `chip` and `IconButton` grew to the 44 px target the rest of
/// the shell already honours — a control taller than the band holding it.
/// Each row keeps the mockup fine-pointer height, or the touch target plus
/// that row own chrome, whichever the pointer calls for.
#[derive(Debug, Clone, Copy, PartialEq)]
struct ResultBarMetrics {
    viewer_tabs: f32,
    viewer_tab: f32,
    sheet_bar: f32,
    structured_strip: f32,
    instrument_control: f32,
}

/// Space a row keeps around its control at the mockup fine-pointer sizes:
/// 41 − 30 for the tab strip, 31 − 23 for the instrument bar.
const RESULT_TAB_STRIP_CHROME: f32 = 11.0;
const RESULT_CONTROL_ROW_CHROME: f32 = 8.0;

impl ResultBarMetrics {
    fn resolve(tokens: &Tokens) -> Self {
        let fine = Self {
            viewer_tabs: 41.0,
            viewer_tab: 30.0,
            sheet_bar: 31.0,
            structured_strip: 40.0,
            instrument_control: 23.0,
        };
        if !tokens.metrics.is_touch() {
            return fine;
        }
        let target = tokens.metrics.ctl_h;
        Self {
            viewer_tabs: fine.viewer_tabs.max(target + RESULT_TAB_STRIP_CHROME),
            viewer_tab: fine.viewer_tab.max(target),
            sheet_bar: fine.sheet_bar.max(target + RESULT_CONTROL_ROW_CHROME),
            structured_strip: fine
                .structured_strip
                .max(target + RESULT_CONTROL_ROW_CHROME),
            instrument_control: fine.instrument_control.max(target),
        }
    }

    fn of(ui: &Ui) -> Self {
        Self::resolve(&Tokens::get(ui.ctx()))
    }
}

/// Horizontal viewer-tab list with the mockup's overflow chevrons: 20×30
/// paddles flanking the list, present only while it genuinely overflows,
/// stepping the scroll position by the mockup's 220 px increment and
/// disabling at either extreme.
fn viewer_tab_scroller(ui: &mut Ui, salt: &'static str, add_tabs: impl FnOnce(&mut Ui)) {
    const SCROLL_STEP: f32 = 220.0;
    const CHEVRON_WIDTH: f32 = 20.0;
    let overflow_id = egui::Id::new(("results.viewer-tabs.overflow", salt));
    // Measured on the previous frame — the leading chevron must reserve its
    // width before the list lays out.
    let (overflowing, at_start, at_end) = ui
        .ctx()
        .data(|data| data.get_temp::<(bool, bool, bool)>(overflow_id))
        .unwrap_or((false, true, true));
    let mut step = 0.0_f32;
    if overflowing && viewer_tab_overflow_chevron(ui, -1.0, !at_start) {
        step -= SCROLL_STEP;
    }
    let trailing = if overflowing { CHEVRON_WIDTH } else { 0.0 };
    let width = (ui.available_width() - trailing).max(1.0);
    let output = egui::ScrollArea::horizontal()
        .id_salt(salt)
        .max_width(width)
        .auto_shrink([false, true])
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
        .show(ui, |ui| {
            let row = ui.horizontal(|ui| add_tabs(ui));
            ui.ctx().accesskit_node_builder(row.response.id, |node| {
                node.set_role(egui::accesskit::Role::TabList);
                node.set_label("Compatible result viewers");
            });
        });
    if overflowing && viewer_tab_overflow_chevron(ui, 1.0, !at_end) {
        step += SCROLL_STEP;
    }
    let visible = output.inner_rect.width();
    let content = output.content_size.x;
    let max_offset = (content - visible).max(0.0);
    let mut state = output.state;
    if step != 0.0 {
        state.offset.x = (state.offset.x + step).clamp(0.0, max_offset);
        state.store(ui.ctx(), output.id);
        ui.ctx().request_repaint();
    }
    ui.ctx().data_mut(|data| {
        data.insert_temp(
            overflow_id,
            (
                content > visible + 0.5,
                state.offset.x <= 0.5,
                state.offset.x >= max_offset - 0.5,
            ),
        );
    });
}

/// One 20×30 overflow paddle. `direction` is −1 for the leading (scroll
/// left) chevron and +1 for the trailing one; the hairline sits on the side
/// facing the tab list. Returns true when an enabled paddle was clicked.
fn viewer_tab_overflow_chevron(ui: &mut Ui, direction: f32, enabled: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(20.0, ResultBarMetrics::of(ui).viewer_tab),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Button,
            enabled,
            if direction < 0.0 {
                "Scroll viewer tabs backward"
            } else {
                "Scroll viewer tabs forward"
            },
        )
    });
    if ui.is_rect_visible(rect) {
        let hovered = enabled && response.hovered();
        if hovered {
            ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
        }
        let mut color = if hovered {
            t.color.text
        } else {
            t.color.text_dim
        };
        if !enabled {
            color = color.gamma_multiply(0.3);
        }
        let center = rect.center();
        let arm_x = center.x - 2.0 * direction;
        let apex_x = center.x + 2.0 * direction;
        ui.painter().add(egui::Shape::line(
            vec![
                egui::pos2(arm_x, center.y - 3.5),
                egui::pos2(apex_x, center.y),
                egui::pos2(arm_x, center.y + 3.5),
            ],
            egui::Stroke::new(1.2, color),
        ));
        let border_x = if direction < 0.0 {
            rect.right() - 0.5
        } else {
            rect.left() + 0.5
        };
        ui.painter().vline(
            border_x,
            rect.y_range(),
            egui::Stroke::new(1.0, t.color.border),
        );
        theme::paint_focus_ring(ui, &response, rect);
    }
    enabled && response.clicked()
}

fn viewer_picker(ui: &mut Ui, icon: WorkbenchIcon, label: &str, accessible_label: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let galley = ui.painter().layout_no_wrap(
        label.to_owned(),
        theme::sans(tokens::FS_1, FontWeight::Regular),
        t.color.text_dim,
    );
    let width = galley.size().x + 36.0;
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(width, ResultBarMetrics::of(ui).viewer_tab),
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
        egui::vec2(1.0, ResultBarMetrics::of(ui).viewer_tab),
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
    let structured = viewer_has_structured_strip(viewer);
    let metrics = ResultBarMetrics::resolve(&t);
    let height = if structured {
        metrics.structured_strip
    } else {
        metrics.sheet_bar
    };
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    let accessible_label = if structured {
        "Structured result controls"
    } else if matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::DcSweep
            | ResultViewer::Bode
            | ResultViewer::NoiseContrib
    ) {
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

    let content = rect.shrink2(egui::vec2(if structured { 12.0 } else { 8.0 }, 0.0));
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(content)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    child.spacing_mut().item_spacing.x = 4.0;
    if structured {
        show_structured_result_strip(&mut child, state);
    } else if matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::DcSweep
            | ResultViewer::Bode
            | ResultViewer::NoiseContrib
    ) {
        show_wave_instrument(&mut child, state);
    } else {
        child.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // The evidence sheets exist to be recorded, so they carry the same
            // export affordance the other tabular sheets do. The plotted
            // sheets keep theirs on the instrument strip instead.
            if matches!(
                viewer,
                ResultViewer::Events
                    | ResultViewer::Soa
                    | ResultViewer::Reliability
                    | ResultViewer::Optimization
            ) {
                export_menu(ui, state);
            }
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

fn show_structured_result_strip(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let title = match state.ui.results.viewer {
        ResultViewer::Op => "Operating point · DC solution",
        ResultViewer::Specs => "Specifications",
        ResultViewer::Table => "Exact retained samples",
        _ => return,
    };

    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        export_menu(ui, state);
        let remaining = ui.available_size();
        ui.allocate_ui_with_layout(
            remaining,
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                egui::ScrollArea::horizontal()
                    .id_salt(("rspice.results.structured-strip", state.ui.results.viewer))
                    .auto_shrink([false, true])
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 10.0;
                            ui.label(
                                egui::RichText::new(title)
                                    .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                                    .color(t.color.text),
                            );
                            result_viewer_actions(ui, state);
                        });
                    });
            },
        );
    });
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
        // The collapsed readout's numbers ride the bar's right edge, where
        // the strip that owns them would otherwise be.
        if let Some(readout) = waves::inline_cursor_readout(state, &t) {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(readout)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            )
            .on_hover_text("Cursor readout · expand the strip for per-trace values");
        }
        let remaining = ui.available_size();
        ui.allocate_ui_with_layout(
            remaining,
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                egui::ScrollArea::horizontal()
                    .id_salt("rspice.results.wave-instrument")
                    .auto_shrink([false, true])
                    .scroll_bar_visibility(
                        egui::scroll_area::ScrollBarVisibility::AlwaysHidden,
                    )
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 4.0;
                            let cursor_active = state.ui.results.plot_tool
                                == ResultPlotTool::Cursor
                                && state.ui.results.cursor_tool.is_armed();
                            if instrument_control(ui, "A|B", cursor_active, "A/B cursor tool")
                                .clicked()
                            {
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

                            // Box zoom and pan are gestures, so they carry
                            // glyphs; the lettered controls on this bar are
                            // the named modes beside them.
                            instrument_separator(ui);
                            if IconButton::new(Icon::BoxZoom)
                                .side(ResultBarMetrics::of(ui).instrument_control)
                                .on(state.ui.results.plot_tool == ResultPlotTool::BoxZoom)
                                .tooltip("Box zoom - drag a region")
                                .show(ui)
                                .clicked()
                            {
                                state.ui.results.plot_tool = ResultPlotTool::BoxZoom;
                            }
                            if IconButton::new(Icon::Pan)
                                .side(ResultBarMetrics::of(ui).instrument_control)
                                .on(state.ui.results.plot_tool == ResultPlotTool::Pan)
                                .tooltip("Pan viewport - drag the plot")
                                .show(ui)
                                .clicked()
                            {
                                state.ui.results.plot_tool = ResultPlotTool::Pan;
                            }

                            // Viewport controls carry their glyphs like the
                            // mockup: magnitude and fit are gestures, and the
                            // lettered controls beside them are named modes.
                            instrument_separator(ui);
                            if IconButton::new(Icon::ZoomIn)
                                .side(ResultBarMetrics::of(ui).instrument_control)
                                .tooltip("Zoom active pane in 2x")
                                .show(ui)
                                .clicked()
                            {
                                waves::zoom_active_pane(state, &t, 0.5);
                            }
                            if IconButton::new(Icon::ZoomOut)
                                .side(ResultBarMetrics::of(ui).instrument_control)
                                .tooltip("Zoom active pane out 2x")
                                .show(ui)
                                .clicked()
                            {
                                waves::zoom_active_pane(state, &t, 2.0);
                            }
                            if IconButton::new(Icon::ZoomFit)
                                .side(ResultBarMetrics::of(ui).instrument_control)
                                .tooltip("Fit active waveform pane")
                                .show(ui)
                                .clicked()
                            {
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
                                state.ui.results.show_spec_limits =
                                    !state.ui.results.show_spec_limits;
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
                                state.ui.results.show_family_envelope =
                                    !state.ui.results.show_family_envelope;
                            }
                            if instrument_control(
                                ui,
                                "GRID",
                                state.ui.results.show_minor_grid,
                                "Show minor waveform grid",
                            )
                            .clicked()
                            {
                                state.ui.results.show_minor_grid =
                                    !state.ui.results.show_minor_grid;
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

                            let hidden = hidden_wave_strip_count(state);
                            if hidden > 0 {
                                instrument_separator(ui);
                                if instrument_control(
                                    ui,
                                    &format!("{hidden} HIDDEN"),
                                    true,
                                    "Restore closed waveform strips",
                                )
                                .clicked()
                                {
                                    state.ui.results.hidden_strips.clear();
                                }
                            }
                        });
                    });
            },
        );
    });
}

fn hidden_wave_strip_count(state: &AppState) -> usize {
    let Some(run) = state.simulation.active_run() else {
        return 0;
    };
    state
        .ui
        .results
        .hidden_strips
        .iter()
        .filter(|key| key.dataset_id() == run.dataset_id && key.resolve(run).is_some())
        .count()
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
        egui::vec2(width, ResultBarMetrics::of(ui).instrument_control),
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

/// Why one analysis' retained evidence is less than a complete run, if it is.
///
/// The typed-evidence sheets all refuse to draw an unsuccessful analysis, so
/// their reader is never misled. The waveform sheets deliberately do draw
/// one — where a transient stopped converging is exactly what an engineer
/// opens the plot to find out — and for a long time they drew it identically
/// to a run that finished. A curve that ends early is not visibly different
/// from a sweep that was specified to end there.
pub(crate) fn incomplete_evidence_reason(analysis: &AnalysisResult) -> Option<&'static str> {
    if !analysis.success {
        return Some("the run did not complete — these samples stop where it failed");
    }
    if analysis.validate_retained_evidence().is_err() {
        return Some("the retained evidence failed validation");
    }
    None
}

/// The same question for whichever analysis the active sheet is speaking for.
fn active_incomplete_evidence_reason(state: &AppState) -> Option<&'static str> {
    let run = state.simulation.active_run()?;
    if viewer_uses_wave_stack(state.ui.results.viewer) {
        // The stack draws every analysis of the run at once, so the bar
        // speaks for the run: one failed strip makes the sheet's evidence
        // incomplete even when the strip beside it converged.
        return run.analyses.iter().find_map(incomplete_evidence_reason);
    }
    incomplete_evidence_reason(state.simulation.active_analysis()?)
}

fn sheet_purpose(state: &AppState) -> String {
    let viewer = state.ui.results.viewer;
    let detail = viewer_availability(state, viewer).reason;
    match active_incomplete_evidence_reason(state) {
        Some(caution) => format!("{} · {detail} · {caution}", viewer.tab_label()),
        None => format!("{} · {detail}", viewer.tab_label()),
    }
}

fn export_menu(ui: &mut Ui, state: &mut AppState) {
    ui.menu_button("Export…", |ui| {
        // Not "waveform data": the export routes on the active sheet and then
        // on the retained payload, so it writes a spectrum here and SOA rules,
        // ageing checkpoints, optimizer candidates or an event history there —
        // none of them samples.
        if ui.button("Result data (CSV)…").clicked() {
            state.ui.export_csv_requested = true;
            ui.close();
        }
        // The figure goes through the publication pipeline, which renders this
        // sheet as PDF/A, PDF, SVG or PNG at a chosen resolution. A window
        // screenshot used to live here instead: the same pixels the reader
        // already had, at whatever the display happened to be, with the chrome
        // cropped off by rectangle. One label for both targets, because the
        // browser reaches the same pipeline through its own worker.
        if ui.button("Viewer figure (PDF, SVG, PNG)…").clicked() {
            state.ui.export_figure_requested = true;
            ui.close();
        }
    });
}

fn inline_result_actions(ui: &mut Ui, state: &mut AppState) {
    export_menu(ui, state);
    result_viewer_actions(ui, state);
}

fn result_viewer_actions(ui: &mut Ui, state: &mut AppState) {
    match state.ui.results.viewer {
        ResultViewer::Waves | ResultViewer::DcSweep => {
            let linked_shortcut = state.ui.preferences.shortcuts().resolved_label(
                crate::workbench::commands::vocabulary::Command::ToggleLinkedCursors,
                crate::workbench::app_state::runtime_command_platform(ui.ctx()),
                ui.ctx().os(),
            );
            let results = &mut state.ui.results;
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
    ui.spacing_mut().item_spacing.x = 0.0;

    let current = state.ui.results.viewer;
    let mut clicked: Option<ResultViewer> = None;

    // The strip lists only the sheets this dataset can feed. With 22 viewers
    // and a typical run feeding a handful, a full row of disabled tabs would
    // bury the ones that work; the Visualization Studio catalog is where every
    // viewer and its requirements are published.
    for viewer in ResultViewer::PRIMARY
        .into_iter()
        .chain(ResultViewer::DATASET_NATIVE)
    {
        if !include(viewer) || !viewer_availability(state, viewer).available {
            continue;
        }
        if viewer_tab(ui, viewer, current == viewer) {
            clicked = Some(viewer);
        }
    }
    if let Some(viewer) = clicked {
        state.ui.results.viewer = viewer;
    }
}

/// Which sheets a persistent result document's docbar offers.
///
/// The family owns the answer, next to the `includes` list the Create dialog
/// composes documents from, so the two cannot drift: this restated the mapping
/// once and drifted immediately — the digital family bound its first pane to
/// the waveform renderer and then refused every waveform sheet, and the
/// phase-noise and table sheets were unreachable in families the dialog binds
/// them for.
///
/// Imported or renamed pages resolve to no family and keep every sheet their
/// dataset can feed reachable.
fn family_allows_viewer(family_label: &str, viewer: ResultViewer) -> bool {
    create_document::ResultDocumentFamily::from_label(family_label)
        .is_none_or(|family| family.offers_sheet(viewer))
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
                run.analyses.iter().any(|analysis| {
                    analysis.analysis_type == crate::state::AnalysisType::Transient
                        && !analysis.waveforms.is_empty()
                })
            }) {
                ViewerAvailability::available(
                    "Transient waveforms are present in the active dataset",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires transient waveform data in the active dataset",
                )
            }
        }
        ResultViewer::DcSweep => {
            if active_run.is_some_and(|run| {
                run.analyses.iter().any(|analysis| {
                    analysis.analysis_type == crate::state::AnalysisType::DcSweep
                        && !analysis.waveforms.is_empty()
                })
            }) {
                ViewerAvailability::available(
                    "A retained swept-source or swept-parameter DC transfer is available",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires DC sweep waveform data in the active dataset",
                )
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
            if has_ac_response {
                ViewerAvailability::available("An AC response is available")
            } else {
                ViewerAvailability::unavailable(
                    "Requires a usable AC response in the active dataset",
                )
            }
        }
        ResultViewer::Fft => specialized_availability(state, ActiveViewer::Fft),
        ResultViewer::HarmonicBalance => {
            if harmonic_balance::active_analysis_is_renderable(state) {
                ViewerAvailability::available(
                    "The selected analysis contains retained complex HB coefficients",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires the selected harmonic-balance analysis to retain complex coefficients",
                )
            }
        }
        ResultViewer::PhaseNoise => {
            if active_run.is_some_and(|run| {
                run.analyses
                    .iter()
                    .any(phase_noise::phase_noise_is_renderable)
            }) {
                ViewerAvailability::available(
                    "A retained periodic phase-noise spectrum is available",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires PNOISE/QPNOISE data explicitly retained as phase noise",
                )
            }
        }
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
                run.analyses
                    .iter()
                    .any(bode::ordinary_noise_spectrum_is_renderable)
            }) {
                ViewerAvailability::available("A retained ordinary-noise spectrum is available")
            } else {
                ViewerAvailability::unavailable(
                    "Requires a usable ordinary-noise spectrum in the active dataset",
                )
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
        ResultViewer::Events => {
            if events::active_analysis_is_renderable(state) {
                ViewerAvailability::available(
                    "A committed XSPICE event history is available for the active analysis",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires a transient analysis with retained XSPICE event nodes",
                )
            }
        }
        ResultViewer::Soa => {
            if soa::active_payload_is_valid(state) {
                ViewerAvailability::available(
                    "Retained safe-operating-area evidence is available for the active analysis",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires the active analysis to contain a valid retained SOA payload",
                )
            }
        }
        ResultViewer::Reliability => {
            if reliability::active_payload_is_valid(state) {
                ViewerAvailability::available(
                    "Retained ageing evidence is available for the active analysis",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires the active analysis to contain a valid retained reliability payload",
                )
            }
        }
        ResultViewer::Optimization => {
            if optimization::active_metadata_is_valid(state) {
                ViewerAvailability::available(
                    "A retained optimizer cost history is available for the active analysis",
                )
            } else {
                ViewerAvailability::unavailable(
                    "Requires the active analysis to carry a retained optimization history",
                )
            }
        }
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
///
/// Every tab drawn is a tab that can be opened. The strip lists only the
/// sheets the active dataset can feed (see [`viewer_tabs_filtered`]), so there
/// is no disabled state to paint here — the Visualization Studio catalog is
/// where the full set of viewers and their requirements are published.
fn viewer_tab(ui: &mut Ui, viewer: ResultViewer, active: bool) -> bool {
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

    let height = ResultBarMetrics::of(ui)
        .viewer_tab
        .min(ui.available_height());
    let horizontal_padding = 20.0;
    let icon_width = 13.0;
    let icon_gap = 6.0;
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            galley.size().x + horizontal_padding + icon_width + icon_gap,
            height,
        ),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::SelectableLabel,
            ui.is_enabled(),
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
        !active && response.hovered(),
        ui.style().animation_time,
    );
    let (fill, text_color) = if active {
        (egui::Color32::TRANSPARENT, c.text)
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
        ResultViewer::Waves | ResultViewer::DcSweep => waves::right_panel(ui, state),
        ResultViewer::Bode => bode::right_panel(ui, state),
        ResultViewer::Fft => fft::right_panel(ui, state),
        ResultViewer::HarmonicBalance => harmonic_balance::right_panel(ui, state),
        ResultViewer::PhaseNoise => phase_noise::right_panel(ui, state),
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
        ResultViewer::Events => events::right_panel(ui, state),
        ResultViewer::Soa => soa::right_panel(ui, state),
        ResultViewer::Reliability => reliability::right_panel(ui, state),
        ResultViewer::Optimization => optimization::right_panel(ui, state),
        ResultViewer::Manifest => manifest::right_panel(ui, state),
    }
}

#[cfg(test)]
mod availability_tests {
    use super::*;
    use crate::product::{ContentDigest, ObjectRevision};
    use crate::state::{
        AnalysisResult, AnalysisResultPayload, AnalysisResultProvenance,
        AnalysisResultSourceDomain, AnalysisType, SensitivityResultMode, SensitivityResultRow,
        SimulationRun, WaveformData,
    };
    use crate::workbench::app_state::SpecializedViewerCacheProvenance;

    fn state_with_analysis(analysis: AnalysisResult) -> AppState {
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));
        state
    }

    #[test]
    fn authored_source_matching_accepts_a_distinct_expanded_execution_identity() {
        let authored_source_id = AnalysisInstanceId::new();
        let expanded_execution_id = AnalysisInstanceId::new();
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "PVT-expanded TRAN")
            .with_provenance(
                AnalysisResultProvenance::new_with_authored_source_domain(
                    AnalysisResultSourceDomain::SimulationPlan,
                    expanded_execution_id,
                    authored_source_id,
                    ObjectRevision::INITIAL,
                    ContentDigest::from_bytes([0x57; 32]),
                    Vec::new(),
                )
                .expect("expanded provenance is valid"),
            );

        assert_ne!(
            analysis
                .provenance()
                .expect("provenance")
                .source_instance_id(),
            authored_source_id
        );
        assert!(analysis_matches_authored_source(
            &analysis,
            authored_source_id
        ));
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
    fn dc_sweep_is_a_distinct_mockup_viewer_and_waveform_projection() {
        let mut run = SimulationRun::new(1);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("V(time)", vec![0.0, 1.0], vec![0.0, 1.0], "#00aaff"),
            ]),
        );
        run.add_analysis(
            AnalysisResult::new(2, AnalysisType::DcSweep, "DC").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.1, 0.9], "#ffbd2e"),
            ]),
        );
        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));

        assert!(viewer_availability(&state, ResultViewer::Waves).available);
        assert!(viewer_availability(&state, ResultViewer::DcSweep).available);

        state.ui.results.viewer = ResultViewer::Waves;
        let presentation = state.ui.preferences.result_presentation_policy();
        let waves = waves::cached_models(
            &state.simulation,
            &mut state.ui.results,
            presentation.complex_number_display(),
            &Tokens::default(),
        );
        assert_eq!(waves.len(), 1);
        assert_eq!(waves[0].analysis_type(), AnalysisType::Transient);

        state.ui.results.viewer = ResultViewer::DcSweep;
        let dc = waves::cached_models(
            &state.simulation,
            &mut state.ui.results,
            presentation.complex_number_display(),
            &Tokens::default(),
        );
        assert_eq!(dc.len(), 1);
        assert_eq!(dc[0].analysis_type(), AnalysisType::DcSweep);
    }

    #[test]
    fn ordinary_noise_enables_noise_but_never_substitutes_for_bode() {
        let state = state_with_analysis(
            AnalysisResult::new(1, AnalysisType::Noise, "NOISE").with_waveforms(vec![
                WaveformData::new("inoise", vec![1.0, 10.0], vec![1.0e-9, 2.0e-9], "#00aaff"),
            ]),
        );

        assert!(viewer_availability(&state, ResultViewer::NoiseContrib).available);
        assert!(!viewer_availability(&state, ResultViewer::Bode).available);
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
        let mut tokens = Tokens::default();
        tokens.metrics.ctl_h = 28.0;
        let fine = ResultBarMetrics::resolve(&tokens);
        assert_eq!(fine.viewer_tabs, 41.0);
        assert_eq!(fine.viewer_tab, 30.0);
        assert_eq!(fine.sheet_bar, 31.0);
        assert_eq!(fine.structured_strip, 40.0);
        assert_eq!(fine.instrument_control, 23.0);
    }

    /// Under a coarse pointer the shell raises every control to a 44 px
    /// target, and these rows have to make room for the controls they hold.
    ///
    /// The old fixed geometry meant a 44 px chip was laid out inside a 31 px
    /// band on a tablet — taller than the row containing it — while the title
    /// bar, drawers, navigator and console all grew correctly around it. The
    /// assertion is the containment, not the numbers: a row that merely got
    /// bigger is no use if it is still shorter than its own controls.
    #[test]
    fn result_bars_make_room_for_a_touch_target() {
        let mut tokens = Tokens::default();
        tokens.metrics.ctl_h = 44.0;
        let touch = ResultBarMetrics::resolve(&tokens);

        assert!(touch.viewer_tab >= 44.0, "{}", touch.viewer_tab);
        assert!(
            touch.instrument_control >= 44.0,
            "{}",
            touch.instrument_control
        );
        assert!(
            touch.viewer_tabs >= touch.viewer_tab,
            "the strip must contain its own tab: {} < {}",
            touch.viewer_tabs,
            touch.viewer_tab
        );
        assert!(
            touch.sheet_bar >= touch.instrument_control,
            "the instrument bar must contain its own controls: {} < {}",
            touch.sheet_bar,
            touch.instrument_control
        );
        assert!(
            touch.structured_strip >= touch.instrument_control,
            "the structured strip must contain its own controls: {} < {}",
            touch.structured_strip,
            touch.instrument_control
        );
    }

    #[test]
    fn result_tabs_follow_the_upgraded_mockup_mode_order() {
        assert_eq!(
            ResultViewer::PRIMARY,
            [
                ResultViewer::Waves,
                ResultViewer::DcSweep,
                ResultViewer::Bode,
                ResultViewer::NoiseContrib,
                ResultViewer::Nyquist,
                ResultViewer::Fft,
                ResultViewer::HarmonicBalance,
                ResultViewer::PhaseNoise,
                ResultViewer::Smith,
                ResultViewer::TransferFunction,
                ResultViewer::Contribution,
                ResultViewer::Op,
                ResultViewer::Specs,
                ResultViewer::Table,
                ResultViewer::Hist,
                ResultViewer::Eye,
                ResultViewer::PoleZero,
                ResultViewer::Events,
                ResultViewer::Soa,
                ResultViewer::Reliability,
                ResultViewer::Optimization,
            ]
        );
    }

    #[test]
    fn structured_result_controls_have_a_reachable_40_px_strip() {
        for viewer in [ResultViewer::Op, ResultViewer::Specs, ResultViewer::Table] {
            assert!(viewer_has_structured_strip(viewer), "{viewer:?}");
            assert!(viewer_has_sheet_bar(viewer), "{viewer:?}");
        }
        for viewer in [ResultViewer::TransferFunction, ResultViewer::Manifest] {
            assert!(!viewer_has_structured_strip(viewer), "{viewer:?}");
            assert!(!viewer_has_sheet_bar(viewer), "{viewer:?}");
        }
        for viewer in [
            ResultViewer::Waves,
            ResultViewer::DcSweep,
            ResultViewer::Bode,
            ResultViewer::Fft,
            ResultViewer::HarmonicBalance,
            ResultViewer::PhaseNoise,
            ResultViewer::Eye,
            ResultViewer::Hist,
            ResultViewer::NoiseContrib,
            ResultViewer::Contribution,
            ResultViewer::Nyquist,
            ResultViewer::Smith,
            ResultViewer::PoleZero,
        ] {
            assert!(!viewer_has_structured_strip(viewer), "{viewer:?}");
            assert!(viewer_has_sheet_bar(viewer), "{viewer:?}");
        }
    }

    #[test]
    fn stage_bar_stands_down_without_a_retained_dataset() {
        let empty = AppState::default();
        assert_eq!(empty.ui.results.viewer, ResultViewer::Waves);
        assert!(!result_stage_bar_visible(&empty));

        let mut allocated = AppState::default();
        allocated.simulation.start_run();
        assert!(!result_stage_bar_visible(&allocated));

        let retained = state_with_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN"));
        assert!(result_stage_bar_visible(&retained));
    }

    #[test]
    fn hidden_wave_strips_make_the_instrument_restore_control_reachable() {
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
        let mut state = state_with_analysis(analysis);
        let run = state.simulation.active_run().expect("active retained run");
        let key = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
        state.ui.results.hidden_strips.insert(key);

        let mut other_run = SimulationRun::new(2);
        other_run.add_analysis(AnalysisResult::new(
            2,
            AnalysisType::Transient,
            "Other TRAN",
        ));
        let other_key = AnalysisPresentationKey::new(
            other_run.dataset_id,
            other_run.analyses.first().expect("other analysis"),
        );
        state.simulation.runs.push(other_run);
        state.ui.results.hidden_strips.insert(other_key);

        assert_eq!(hidden_wave_strip_count(&state), 1);
    }

    const MOCKUP_FAMILIES: [&str; 8] = [
        "Waveform worksheet",
        "Frequency & stability",
        "RF & network",
        "Statistics & yield",
        "Digital & AMS events",
        "Fields & physical",
        "Photonics",
        "Report page",
    ];

    #[test]
    fn persistent_document_families_scope_to_their_own_plot_types() {
        assert!(family_allows_viewer(
            "Waveform worksheet",
            ResultViewer::Waves
        ));
        assert!(family_allows_viewer(
            "Waveform worksheet",
            ResultViewer::DcSweep
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
        assert!(!family_allows_viewer("Photonics", ResultViewer::Waves));
    }

    /// Dataset-native sheets carry evidence the bound dataset either has or has
    /// not; they are not one family's plot mode, and availability is their only
    /// gate. A family that hid them would drop evidence the same dataset shows
    /// in the workspace the moment the user promoted the view to a document.
    #[test]
    fn every_family_offers_the_dataset_native_sheets() {
        for family in MOCKUP_FAMILIES {
            for viewer in
                ResultViewer::every().filter(|viewer| viewer.viewer_document_id().is_none())
            {
                assert!(family_allows_viewer(family, viewer), "{family} {viewer:?}");
            }
        }
    }

    /// `viewer_document_id` and `from_viewer_document_id` are one map read in
    /// two directions, so every answer either gives must agree with the other.
    /// Three sheets share `viewer-table`, which is exactly where the second
    /// copy of the inverse had drifted onto a different one.
    #[test]
    fn the_viewer_document_map_agrees_with_itself_in_both_directions() {
        use crate::results::viewer_catalog::VIEWER_DOCUMENTS;

        for viewer in ResultViewer::every() {
            let Some(document_id) = viewer.viewer_document_id() else {
                assert_eq!(
                    ResultViewer::from_viewer_document_id(viewer.label()),
                    None,
                    "{viewer:?} is dataset-native and must not answer to a document id"
                );
                continue;
            };
            assert!(
                VIEWER_DOCUMENTS
                    .iter()
                    .any(|document| document.id == document_id),
                "{viewer:?} claims {document_id}, which the catalog does not publish"
            );
            let drawn_by = ResultViewer::from_viewer_document_id(document_id)
                .unwrap_or_else(|| panic!("{document_id} has no sheet"));
            assert_eq!(
                drawn_by.viewer_document_id(),
                Some(document_id),
                "{document_id} resolves to {drawn_by:?}, which renders something else"
            );
        }
    }

    /// Pages carry their family only as their title, so a renamed or imported
    /// page resolves to no family at all. It must fall back to offering every
    /// sheet the dataset can feed, never to hiding all of them.
    #[test]
    fn a_page_outside_the_mockup_families_keeps_every_sheet_reachable() {
        for viewer in ResultViewer::every() {
            assert!(family_allows_viewer("Transient review", viewer));
            assert!(family_allows_viewer("Waveform worksheet · 01", viewer));
        }
    }

    fn soa_analysis() -> AnalysisResult {
        use crate::state::{
            AnalysisResultFamilyMetadata, SoaEvaluationEvidence, SoaParameterEvidence,
            SoaRuleVerdictEvidence, SoaViolationEvidence, SoaViolationSeverityEvidence,
        };
        let time = vec![0.0, 1.0e-9];
        AnalysisResult::new(1, AnalysisType::Soa, "SOA")
            .with_family_metadata(AnalysisResultFamilyMetadata::Soa { time: time.clone() })
            .with_waveforms(vec![
                WaveformData::new(
                    "SOA_VIOLATION_COUNT",
                    time.clone(),
                    vec![0.0, 0.0],
                    "#ffbd2e",
                ),
                // The stress history the producer now retains, named exactly as
                // the sheet addresses it.
                WaveformData::new(
                    crate::services::safety::soa_stress_waveform_name(
                        "M1",
                        crate::services::safety::SoAParameter::Vds,
                    ),
                    time,
                    vec![2.0, 3.0],
                    "#00aaff",
                ),
            ])
            .with_result_payload(AnalysisResultPayload::Soa {
                evaluations: vec![SoaEvaluationEvidence {
                    device_id: "M1".to_owned(),
                    parameter: SoaParameterEvidence::DrainSourceVoltage,
                    limit_value: 3.3,
                    worst_actual_value: 3.0,
                    worst_time_s: 1.0e-9,
                    sample_count: 2,
                    unit: "V".to_owned(),
                    description: "Maximum drain-source voltage".to_owned(),
                    // 3.0 V against a 3.3 V limit is inside the warning band,
                    // which the payload validator derives rather than trusts.
                    verdict: SoaRuleVerdictEvidence::Warning,
                }],
                // A non-passing rule must carry the exact event at its worst
                // point; the validator refuses a verdict with nothing behind it.
                violations: vec![SoaViolationEvidence {
                    device_id: "M1".to_owned(),
                    parameter: SoaParameterEvidence::DrainSourceVoltage,
                    limit_value: 3.3,
                    actual_value: 3.0,
                    time_s: 1.0e-9,
                    severity: SoaViolationSeverityEvidence::Warning,
                }],
            })
    }

    fn reliability_analysis() -> AnalysisResult {
        use crate::state::{
            AnalysisResultFamilyMetadata, ReliabilityCheckpointEvidence, ReliabilityDeviceEvidence,
            ReliabilityShiftEvidence, ReliabilityStressEvidence,
        };
        AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
            .with_family_metadata(AnalysisResultFamilyMetadata::Reliability {
                years: vec![1.0, 5.0],
            })
            .with_result_payload(AnalysisResultPayload::Reliability {
                devices: vec![ReliabilityDeviceEvidence {
                    device_id: "M1".to_owned(),
                    stress: ReliabilityStressEvidence {
                        average_gate_stress_v: 1.1,
                        average_drain_stress_v: 1.8,
                        average_temperature_k: 358.0,
                        duration_s: 1.0e-6,
                    },
                    checkpoints: vec![
                        ReliabilityCheckpointEvidence {
                            years: 1.0,
                            shift: ReliabilityShiftEvidence {
                                threshold_voltage_shift_v: 1.0e-3,
                                mobility_shift: -1.0e-4,
                                drain_source_resistance_shift: 2.0e-3,
                            },
                        },
                        ReliabilityCheckpointEvidence {
                            years: 5.0,
                            shift: ReliabilityShiftEvidence {
                                threshold_voltage_shift_v: 4.0e-3,
                                mobility_shift: -4.0e-4,
                                drain_source_resistance_shift: 8.0e-3,
                            },
                        },
                    ],
                }],
            })
    }

    fn optimization_analysis() -> AnalysisResult {
        use crate::state::AnalysisResultFamilyMetadata;
        AnalysisResult::new(1, AnalysisType::Optimization, "Optimization")
            .with_family_metadata(AnalysisResultFamilyMetadata::Optimization {
                iterations: vec![0.0, 1.0],
                best_cost: 0.25,
                best_variables: [("w".to_owned(), 1.5e-6)].into_iter().collect(),
                converged: true,
            })
            // The names the optimization runner emits: one OPT_COST trace over
            // the iteration axis, plus OPT_<variable> per design variable.
            .with_waveforms(vec![
                WaveformData::new("OPT_COST", vec![0.0, 1.0], vec![1.0, 0.25], "#ffbd2e"),
                WaveformData::new("OPT_w", vec![0.0, 1.0], vec![1.0e-6, 1.5e-6], "#00aaff"),
            ])
    }

    fn events_analysis() -> AnalysisResult {
        use crate::state::{
            DigitalEventPointEvidence, DigitalEventTraceEvidence, RealEventPointEvidence,
            RealEventTraceEvidence,
        };
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(
            AnalysisResultPayload::TransientEvents {
                digital_traces: vec![DigitalEventTraceEvidence {
                    node_name: "clk".to_owned(),
                    points: vec![
                        DigitalEventPointEvidence {
                            time_s: 0.0,
                            value_code: 0,
                        },
                        DigitalEventPointEvidence {
                            time_s: 5.0e-10,
                            value_code: 1,
                        },
                        DigitalEventPointEvidence {
                            time_s: 1.0e-9,
                            value_code: 12,
                        },
                    ],
                }],
                real_traces: vec![RealEventTraceEvidence {
                    node_name: "level".to_owned(),
                    points: vec![RealEventPointEvidence {
                        time_s: 2.5e-10,
                        value: 0.75,
                    }],
                }],
            },
        )
    }

    /// Every campaign analysis the Simulate catalog can launch must land on a
    /// sheet that draws it. These four sheets shipped in the repository for
    /// months with no `mod` declaration, so SOA, ageing, optimizer and event
    /// runs produced results the Results workspace could not show at all.
    #[test]
    fn every_campaign_analysis_reaches_a_sheet_that_can_draw_it() {
        for (analysis, viewer) in [
            (soa_analysis(), ResultViewer::Soa),
            (reliability_analysis(), ResultViewer::Reliability),
            (optimization_analysis(), ResultViewer::Optimization),
            (events_analysis(), ResultViewer::Events),
        ] {
            assert!(
                analysis.validate_retained_evidence().is_ok(),
                "{viewer:?} fixture is not valid evidence"
            );
            let mut state = state_with_analysis(analysis);
            assert!(
                viewer_availability(&state, viewer).available,
                "{viewer:?} is unreachable"
            );
            // The sheet's own evidence gate must agree with the tab that
            // offered it, or the tab opens onto a refusal.
            let key = active_analysis_key(&state);
            assert!(retained_evidence_is_valid(&mut state, key), "{viewer:?}");
        }
    }

    /// A transient with no event nodes must not grow an empty event payload:
    /// the tab would light up on a deck that has nothing to show.
    #[test]
    fn a_transient_without_event_nodes_offers_no_event_sheet() {
        let state = transient_state();
        assert!(!viewer_availability(&state, ResultViewer::Events).available);
    }

    /// Render the real tab strip and read the accessibility tree it publishes.
    ///
    /// The contract is that a sheet gets a tab exactly when the dataset can
    /// feed it — so each new sheet has to appear on its own evidence and stay
    /// out of the strip on everyone else's. Availability alone would not catch
    /// a viewer that is gated correctly but never drawn.
    fn rendered_tab_labels(analysis: AnalysisResult) -> Vec<String> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut state = state_with_analysis(analysis);
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_680.0, 1_020.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| viewer_tabs(ui, &mut state));
            },
        );
        output
            .platform_output
            .accesskit_update
            .expect("the viewer tab strip publishes an accessibility tree")
            .nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Tab)
            .filter_map(|(_, node)| node.label().map(str::to_owned))
            .collect()
    }

    #[test]
    fn each_new_sheet_gets_a_tab_on_its_own_evidence_and_only_then() {
        let cases = [
            (soa_analysis(), "SOA"),
            (reliability_analysis(), "Ageing"),
            (optimization_analysis(), "Optimization"),
            (events_analysis(), "Events"),
        ];
        for (analysis, expected) in &cases {
            let labels = rendered_tab_labels(analysis.clone());
            assert!(
                labels.iter().any(|label| label == expected),
                "{expected} has no tab on its own dataset; strip is {labels:?}"
            );
            for (_, other) in cases.iter().filter(|(_, other)| other != expected) {
                assert!(
                    !labels.iter().any(|label| label == other),
                    "{other} offered itself on the {expected} dataset; strip is {labels:?}"
                );
            }
        }
    }

    /// Every tab the strip draws opens; the strip filters the rest out rather
    /// than painting them disabled. A tab that reported itself unavailable
    /// would mean `viewer_tab` had grown a state its only caller excludes.
    #[test]
    fn every_tab_the_strip_draws_can_be_opened() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut state = state_with_analysis(soa_analysis());
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_680.0, 1_020.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| viewer_tabs(ui, &mut state));
            },
        );
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("the viewer tab strip publishes an accessibility tree")
            .nodes;
        let tabs = nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Tab)
            .collect::<Vec<_>>();
        assert!(!tabs.is_empty(), "the strip drew no tabs at all");
        for (_, node) in tabs {
            assert!(
                !node.is_disabled(),
                "the strip drew a disabled tab: {:?}",
                node.label()
            );
        }
    }

    /// Draw each newly reachable sheet through a real frame and tessellate it.
    ///
    /// Availability only proves the tab lights up. This proves the sheet
    /// behind it lays out, paints and meshes — the failure mode these viewers
    /// were most exposed to, having never been compiled at all.
    #[test]
    fn every_newly_reachable_sheet_draws_and_meshes() {
        for (analysis, viewer) in [
            (soa_analysis(), ResultViewer::Soa),
            (reliability_analysis(), ResultViewer::Reliability),
            (optimization_analysis(), ResultViewer::Optimization),
            (events_analysis(), ResultViewer::Events),
        ] {
            let mut state = state_with_analysis(analysis);
            state.ui.results.viewer = viewer;
            if viewer == ResultViewer::Soa {
                // The stress trace is the sheet's only plot; a card that never
                // opens is a card this test never covers.
                state.ui.results.soa_stress_trace_open = true;
                state.ui.results.selected_soa_rule = Some(SoaRuleSelection {
                    analysis: active_analysis_key(&state),
                    device_id: "M1".to_owned(),
                    parameter: crate::state::SoaParameterEvidence::DrainSourceVoltage,
                });
            }
            let mut app = RSpiceApp::test_instance();
            app.state = state;
            draw_sheet_and_tessellate(&mut app, viewer);
        }
    }

    /// Every sheet, painted on evidence that sheet declares itself able to
    /// draw.
    ///
    /// The fixture has to satisfy the sheet's own `viewer_availability`
    /// contract, so this cannot pass by painting an empty state and calling it
    /// coverage. Fourteen of the twenty-two sheets had never been through a
    /// real frame in any test; their module tests derive and format values
    /// without ever painting them, and a value that escapes an axis mapping
    /// does not fail an assert — it becomes a vertex at infinity.
    #[test]
    fn every_sheet_draws_the_evidence_it_claims() {
        for viewer in ResultViewer::every() {
            let mut app = app_showing(viewer);
            assert!(
                viewer_availability(&app.state, viewer).available,
                "the {viewer:?} fixture does not satisfy the sheet's own contract: {}",
                viewer_availability(&app.state, viewer).reason
            );
            draw_sheet_and_tessellate(&mut app, viewer);
        }
    }

    /// A dataset each sheet reports itself able to draw. Built from retained
    /// evidence and, where a sheet reads a derived cache, through the same
    /// derivation the controller runs after a completed analysis — never by
    /// asserting a capability the data does not support.
    fn app_showing(viewer: ResultViewer) -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        let analysis = match viewer {
            ResultViewer::Waves
            | ResultViewer::Table
            | ResultViewer::Manifest
            | ResultViewer::Eye
            | ResultViewer::Fft => transient_analysis(),
            ResultViewer::DcSweep => AnalysisResult::new(1, AnalysisType::DcSweep, "DC")
                .with_waveforms(vec![WaveformData::new(
                    "V(out)",
                    vec![0.0, 0.5, 1.0, 1.5],
                    vec![0.0, 0.4, 0.9, 1.2],
                    "#00aaff",
                )]),
            ResultViewer::Bode | ResultViewer::Nyquist | ResultViewer::Smith => ac_analysis(),
            ResultViewer::HarmonicBalance => {
                AnalysisResult::new(1, AnalysisType::HarmonicBalance, "HB").with_waveforms(vec![
                    WaveformData::new(
                        "V(out)",
                        vec![0.0, 1.0e9, 2.0e9],
                        vec![1.0, 0.1, 0.01],
                        "#00aaff",
                    )
                    .with_complex_components(
                        "V(out)",
                        vec![1.0, 0.1, 0.01],
                        vec![0.0, 0.02, 0.001],
                    ),
                ])
            }
            ResultViewer::PhaseNoise => AnalysisResult::new(1, AnalysisType::Pnoise, "PNOISE")
                .with_family_metadata(crate::state::AnalysisResultFamilyMetadata::PeriodicNoise {
                    output_quantity: crate::state::PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
                    carrier_frequency_hz: Some(2.4e9),
                })
                .with_waveforms(vec![WaveformData::new(
                    "phase_noise",
                    vec![1.0e3, 1.0e4, 1.0e5],
                    vec![-80.0, -100.0, -120.0],
                    "#00aaff",
                )]),
            ResultViewer::Hist => monte_carlo_analysis(),
            ResultViewer::Op => AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_device_op(
                rspice_core::circuit::DeviceOpReport {
                    entries: vec![rspice_core::circuit::DeviceOpEntry {
                        name: "M1".to_owned(),
                        device_kind: "MOSFET",
                        region: Some("saturation"),
                        params: Vec::new(),
                    }],
                },
            ),
            ResultViewer::NoiseContrib => AnalysisResult::new(1, AnalysisType::Noise, "NOISE")
                .with_waveforms(vec![WaveformData::new(
                    "inoise",
                    vec![1.0, 10.0, 100.0],
                    vec![1.0e-9, 2.0e-9, 4.0e-9],
                    "#00aaff",
                )]),
            ResultViewer::Contribution => AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS")
                .with_result_payload(AnalysisResultPayload::Sensitivity {
                    output: "V(out)".to_owned(),
                    result_mode: SensitivityResultMode::Dc,
                    rows: vec![
                        SensitivityResultRow {
                            parameter: "r1".to_owned(),
                            raw: 0.25,
                            normalized: 0.5,
                        },
                        SensitivityResultRow {
                            parameter: "r2".to_owned(),
                            raw: -0.125,
                            normalized: -0.25,
                        },
                    ],
                }),
            ResultViewer::TransferFunction => transfer_function_analysis(),
            ResultViewer::Specs => {
                app.state.workspace.specs.push(crate::state::SpecEntry {
                    measurement: "V(out)".to_owned(),
                    expression: String::new(),
                    min: Some(-2.0),
                    max: Some(5.0),
                    unit: "V".to_owned(),
                    scope: crate::state::SpecPointScope::AllPoints,
                });
                transient_analysis()
            }
            ResultViewer::PoleZero => AnalysisResult::new(1, AnalysisType::PoleZero, "PZ")
                .with_result_payload(AnalysisResultPayload::PoleZero {
                    poles: vec![crate::state::ComplexResultValue {
                        real: -1.0e3,
                        imaginary: 2.0e3,
                    }],
                    zeros: vec![crate::state::ComplexResultValue {
                        real: -5.0e3,
                        imaginary: 0.0,
                    }],
                    gain: 10.0,
                }),
            ResultViewer::Events => events_analysis(),
            ResultViewer::Soa => soa_analysis(),
            ResultViewer::Reliability => reliability_analysis(),
            ResultViewer::Optimization => optimization_analysis(),
        };

        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        app.state.simulation.runs = vec![run];
        assert!(app.state.simulation.select_run(0));
        assert!(app.state.simulation.select_analysis(0));

        // Three sheets read a cache the controller fills after a run rather
        // than the retained analysis itself. Derive it exactly as the
        // controller's post-processing does, so the fixture cannot claim a
        // capability the data would not actually produce.
        match viewer {
            ResultViewer::Nyquist => derive_nyquist(&mut app),
            ResultViewer::Smith => derive_smith(&mut app),
            ResultViewer::Hist => derive_histogram(&mut app),
            _ => {}
        }

        if viewer == ResultViewer::Soa {
            // The stress trace is the sheet's only plot; a card that never
            // opens is a card this test never covers.
            app.state.ui.results.soa_stress_trace_open = true;
            app.state.ui.results.selected_soa_rule = Some(SoaRuleSelection {
                analysis: active_analysis_key(&app.state),
                device_id: "M1".to_owned(),
                parameter: crate::state::SoaParameterEvidence::DrainSourceVoltage,
            });
        }
        app
    }

    fn transient_analysis() -> AnalysisResult {
        let time: Vec<f64> = (0..64).map(|index| index as f64 * 1.0e-9).collect();
        let values: Vec<f64> = time
            .iter()
            .map(|t| (t * 1.0e9 * std::f64::consts::TAU / 8.0).sin())
            .collect();
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![WaveformData::new("V(out)", time, values, "#00aaff")])
    }

    /// A resonant second-order response, swept densely enough to decimate.
    ///
    /// Four points was not a sweep: it sat under every reduction threshold, so
    /// the Bode, Nyquist and Smith fixtures exercised only the raw-stroke
    /// path and never the one a real AC run takes. The resonance also makes
    /// the complex locus genuinely non-monotone in its real part, so the
    /// precondition assertion in `decimate_minmax` fires if either locus
    /// sheet ever loses its `parametric` declaration.
    const AC_FIXTURE_POINTS: usize = 8_192;

    fn ac_analysis() -> AnalysisResult {
        let (mut frequency, mut real, mut imaginary) = (
            Vec::with_capacity(AC_FIXTURE_POINTS),
            Vec::with_capacity(AC_FIXTURE_POINTS),
            Vec::with_capacity(AC_FIXTURE_POINTS),
        );
        let (mut magnitude, mut phase) = (
            Vec::with_capacity(AC_FIXTURE_POINTS),
            Vec::with_capacity(AC_FIXTURE_POINTS),
        );
        // H(jω) = 1 / (1 − (ω/ω0)² + j·ω/(Q·ω0)), log-swept through resonance.
        let (natural, quality) = (1.0e5_f64, 6.0_f64);
        for index in 0..AC_FIXTURE_POINTS {
            let decade = 3.0 + 4.0 * index as f64 / (AC_FIXTURE_POINTS - 1) as f64;
            let f = 10.0_f64.powf(decade);
            let ratio = f / natural;
            let (denominator_real, denominator_imaginary) = (1.0 - ratio * ratio, ratio / quality);
            let square =
                denominator_real * denominator_real + denominator_imaginary * denominator_imaginary;
            let (re, im) = (denominator_real / square, -denominator_imaginary / square);
            frequency.push(f);
            real.push(re);
            imaginary.push(im);
            magnitude.push(re.hypot(im));
            phase.push(im.atan2(re).to_degrees());
        }
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
            WaveformData::new("|V(out)|", frequency.clone(), magnitude, "#00aaff")
                .with_complex_components("V(out)", real, imaginary),
            WaveformData::new("phase(V(out))", frequency, phase, "#ffaa00"),
        ])
    }

    fn monte_carlo_analysis() -> AnalysisResult {
        AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![0.0, 1.0, 2.0, 3.0],
                vec![0.95, 1.02, 0.98, 1.05],
                "#00aaff",
            ),
        ])
    }

    fn transfer_function_analysis() -> AnalysisResult {
        use crate::state::{
            TransferFunctionAccuracyEvidence, TransferFunctionNormalizationEvidence,
            TransferFunctionQuantityEvidence, TransferFunctionScalarEvidence,
        };
        AnalysisResult::new(1, AnalysisType::Tf, "XF").with_result_payload(
            AnalysisResultPayload::TransferFunction {
                input_source: "vin".to_owned(),
                output_expression: "V(out)".to_owned(),
                input_quantity: TransferFunctionQuantityEvidence::Voltage,
                output_quantity: TransferFunctionQuantityEvidence::Voltage,
                input_unit: "V".to_owned(),
                output_unit: "V".to_owned(),
                normalization: TransferFunctionNormalizationEvidence::None,
                accuracy: TransferFunctionAccuracyEvidence::Balanced,
                gain: Some(TransferFunctionScalarEvidence::Finite(-12.5)),
                input_resistance: Some(TransferFunctionScalarEvidence::Finite(1.0e6)),
                output_resistance: Some(TransferFunctionScalarEvidence::Finite(50.0)),
                nominal_input: None,
                nominal_output: None,
            },
        )
    }

    fn in_flight_cache_provenance(app: &RSpiceApp) -> SpecializedViewerCacheProvenance {
        let run = app
            .state
            .simulation
            .active_run()
            .expect("the fixture retained one run");
        let dataset_id = run.dataset_id;
        let analysis = app
            .state
            .simulation
            .active_analysis()
            .expect("the fixture selected one analysis");
        SpecializedViewerCacheProvenance::for_analysis(dataset_id, analysis)
    }

    fn ac_complex_trace(app: &RSpiceApp) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let analysis = app
            .state
            .simulation
            .active_analysis()
            .expect("the fixture selected one analysis");
        let waveform = analysis
            .waveforms
            .iter()
            .find(|waveform| waveform.complex.is_some())
            .expect("the AC fixture retains one complex trace");
        let complex = waveform
            .complex
            .as_ref()
            .expect("the trace was selected for its complex components");
        (
            waveform.x.to_vec(),
            complex.real.to_vec(),
            complex.imag.to_vec(),
        )
    }

    fn derive_nyquist(app: &mut RSpiceApp) {
        let (frequency, real, imaginary) = ac_complex_trace(app);
        let provenance = in_flight_cache_provenance(app);
        app.state.analysis.nyquist_state.load_data(
            crate::analysis::nyquist::data::NyquistData::from_arrays(
                "V(out)", &frequency, &real, &imaginary,
            ),
        );
        app.state
            .bind_specialized_viewer_cache(ActiveViewer::Nyquist, provenance);
    }

    fn derive_smith(app: &mut RSpiceApp) {
        let (frequency, real, imaginary) = ac_complex_trace(app);
        let provenance = in_flight_cache_provenance(app);
        app.state
            .analysis
            .smith_chart_state
            .load_sparam_data("S11", &frequency, &real, &imaginary);
        app.state
            .bind_specialized_viewer_cache(ActiveViewer::SmithChart, provenance);
    }

    fn derive_histogram(app: &mut RSpiceApp) {
        use crate::analysis::histogram::data::{Histogram, HistogramBin};

        let counts = [2_usize, 5, 9, 4];
        let edges = [0.90_f64, 0.95, 1.00, 1.05, 1.10];
        let bins: Vec<HistogramBin> = counts
            .iter()
            .enumerate()
            .map(|(index, count)| HistogramBin {
                lower: edges[index],
                upper: edges[index + 1],
                count: *count,
                weight: *count as f64,
            })
            .collect();
        let total_count: usize = counts.iter().sum();
        let provenance = in_flight_cache_provenance(app);
        app.state
            .analysis
            .histogram_state
            .load_histogram(Histogram {
                name: "V(out)".to_owned(),
                bins,
                total_count,
                total_weight: total_count as f64,
                underflow: 0,
                overflow: 0,
                data_min: edges[0],
                data_max: edges[edges.len() - 1],
            });
        app.state
            .bind_specialized_viewer_cache(ActiveViewer::Histogram, provenance);
    }

    /// Every sheet, painted with no retained dataset at all.
    ///
    /// That is the workspace's state before the first run and the one a user
    /// meets first, and it is where a plot divides by a zero-width axis span or
    /// reads the first of an empty series. Fourteen of the twenty-two sheets had
    /// never been through a real frame in any test — their module tests derive
    /// and format values without ever painting them.
    #[test]
    fn every_sheet_paints_an_empty_dataset() {
        for viewer in ResultViewer::every() {
            let mut app = RSpiceApp::test_instance();
            draw_sheet_and_tessellate(&mut app, viewer);
            assert_eq!(
                app.state.ui.results.viewer, viewer,
                "{viewer:?} did not stay selected across its own frame"
            );
        }
    }

    /// Run one sheet plus its inspector panel through a real egui frame and
    /// assert every mesh vertex is finite. A value that escapes an axis
    /// mapping does not panic — it becomes a vertex at infinity and
    /// degenerates the whole draw call.
    ///
    /// Painted through `show_persistent_pane_viewer`, the same dispatch the
    /// workbench itself paints through, so no sheet can pass here by a route
    /// the product never takes.
    fn draw_sheet_and_tessellate(app: &mut RSpiceApp, viewer: ResultViewer) {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1440.0, 900.0),
            )),
            ..Default::default()
        };
        // Twice: the first frame builds the caches, the second is the path a
        // reader actually spends their time on.
        for _ in 0..2 {
            let output = ctx.run_ui(input.clone(), |ctx| {
                egui::Panel::right("inspector").show(ctx, |ui| right_panel(ui, &mut app.state));
                egui::CentralPanel::default()
                    .show(ctx, |ui| show_persistent_pane_viewer(ui, app, viewer));
            });
            for primitive in ctx.tessellate(output.shapes, output.pixels_per_point) {
                let egui::epaint::Primitive::Mesh(mesh) = primitive.primitive else {
                    continue;
                };
                assert!(
                    mesh.vertices
                        .iter()
                        .all(|vertex| vertex.pos.x.is_finite() && vertex.pos.y.is_finite()),
                    "{viewer:?} put a non-finite vertex in the mesh",
                );
            }
        }
    }

    /// Paint one sheet and count the primitives it actually emits.
    fn drawn_shape_count(app: &mut RSpiceApp, viewer: ResultViewer) -> usize {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1440.0, 900.0),
            )),
            ..Default::default()
        };
        let mut shapes = 0;
        for _ in 0..2 {
            let output = ctx.run_ui(input.clone(), |ctx| {
                egui::CentralPanel::default()
                    .show(ctx, |ui| show_persistent_pane_viewer(ui, app, viewer));
            });
            shapes = output.shapes.len();
        }
        shapes
    }

    fn operating_point_app(devices: usize) -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        let analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_device_op(
            rspice_core::circuit::DeviceOpReport {
                entries: (0..devices)
                    .map(|index| rspice_core::circuit::DeviceOpEntry {
                        // Spread across scopes so the group headers and the
                        // per-scope column sets are exercised too.
                        name: format!("X{}.M{index}", index % 8),
                        device_kind: "MOSFET",
                        region: Some("saturation"),
                        params: Vec::new(),
                    })
                    .collect(),
            },
        );
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        app.state.simulation.runs = vec![run];
        assert!(app.state.simulation.select_run(0));
        assert!(app.state.simulation.select_analysis(0));
        app
    }

    fn sensitivity_app(parameters: usize) -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        let analysis = AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS")
            .with_result_payload(AnalysisResultPayload::Sensitivity {
                output: "V(out)".to_owned(),
                result_mode: SensitivityResultMode::Dc,
                rows: (0..parameters)
                    .map(|index| SensitivityResultRow {
                        // Zero-padded: the payload requires strictly sorted
                        // parameter names, and "p10" sorts before "p2".
                        parameter: format!("p{index:06}"),
                        raw: index as f64 * 1.0e-3,
                        normalized: (index as f64 * 1.0e-3).tanh(),
                    })
                    .collect(),
            });
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        app.state.simulation.runs = vec![run];
        assert!(app.state.simulation.select_run(0));
        assert!(app.state.simulation.select_analysis(0));
        app
    }

    fn manifest_app(analyses: usize) -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        let mut run = SimulationRun::new(1);
        for index in 0..analyses {
            run.add_analysis(AnalysisResult::new(
                index as u64 + 1,
                AnalysisType::Transient,
                format!("TRAN {index}"),
            ));
        }
        app.state.simulation.runs = vec![run];
        assert!(app.state.simulation.select_run(0));
        assert!(app.state.simulation.select_analysis(0));
        app
    }

    /// The evidence tables must cost their viewport, not their dataset.
    ///
    /// `save_device_op` on a real block emits one row per device, a swept
    /// design ranks one sensitivity row per parameter, and a save-all run
    /// retains one manifest row per task. All three laid out, sensed and
    /// painted every retained row on every frame, so the sheet's cost grew
    /// without bound while the window stayed the same size. A row count is
    /// not the assertion — the drawn primitive count is.
    #[test]
    fn the_evidence_tables_cost_their_viewport_not_their_dataset() {
        for (viewer, small, large) in [
            (
                ResultViewer::Op,
                drawn_shape_count(&mut operating_point_app(64), ResultViewer::Op),
                drawn_shape_count(&mut operating_point_app(20_000), ResultViewer::Op),
            ),
            (
                ResultViewer::Contribution,
                drawn_shape_count(&mut sensitivity_app(64), ResultViewer::Contribution),
                drawn_shape_count(&mut sensitivity_app(20_000), ResultViewer::Contribution),
            ),
            (
                ResultViewer::Manifest,
                drawn_shape_count(&mut manifest_app(64), ResultViewer::Manifest),
                drawn_shape_count(&mut manifest_app(4_000), ResultViewer::Manifest),
            ),
        ] {
            assert!(
                large < small * 2,
                "{viewer:?} drew {large} primitives for the large dataset against {small} \
                 for the small one — it is still laying out rows it cannot show"
            );
        }
    }

    /// A run that stopped early must not draw like one that finished.
    ///
    /// The waveform sheets deliberately still plot it — where a transient
    /// stopped converging is what the plot is for — but for a long time they
    /// plotted it with no mark at all, while every typed-evidence sheet
    /// refused outright. A curve that ends early is indistinguishable from a
    /// sweep specified to end there.
    #[test]
    fn a_run_that_did_not_complete_says_so_on_the_sheet_that_draws_it() {
        let mut app = app_showing(ResultViewer::Waves);
        assert_eq!(
            active_incomplete_evidence_reason(&app.state),
            None,
            "a converged fixture must not claim a caution"
        );
        let clean_purpose = sheet_purpose(&app.state);

        app.state.simulation.runs[0].analyses[0].success = false;

        // Still drawable: refusing the partial samples would take away the
        // one view that answers "where did it go wrong?".
        assert!(
            viewer_availability(&app.state, ResultViewer::Waves).available,
            "partial transient samples must stay drawable"
        );
        let purpose = sheet_purpose(&app.state);
        assert_ne!(purpose, clean_purpose);
        assert!(
            purpose.contains("did not complete"),
            "the sheet bar has to say it in words: {purpose}"
        );

        // And the strip that draws it carries the mark, so a stack whose
        // other analyses converged cannot pass the failed one off as clean.
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_680.0, 1_020.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| waves::show(ui, &mut app.state));
            },
        );
        let labels: Vec<String> = output
            .platform_output
            .accesskit_update
            .expect("the waveform stack publishes an accessibility tree")
            .nodes
            .iter()
            .filter_map(|(_, node)| node.label().map(str::to_owned))
            .collect();
        assert!(
            labels
                .iter()
                .any(|label| label.contains("stop where it failed")),
            "no pane published the incomplete-evidence reason: {labels:?}"
        );
    }

    #[test]
    fn a_typed_interval_round_trips_through_the_field_that_shows_it() {
        for range in [
            (0.0, 5.0),
            (-1.5e-3, 2.25e-3),
            (1.0e6, 1.0e9),
            (-1.0e-15, 1.0e-14),
        ] {
            let text = format_axis_range(range);
            let parsed = parse_axis_range(&text)
                .unwrap_or_else(|| panic!("{text} did not read back as an interval"));
            assert!(
                (parsed.0 - range.0).abs() <= range.0.abs() * 1.0e-6
                    && (parsed.1 - range.1).abs() <= range.1.abs() * 1.0e-6,
                "{range:?} rendered as {text} and read back as {parsed:?}"
            );
        }
    }

    #[test]
    fn an_interval_parses_from_the_separators_an_engineer_would_paste() {
        for text in ["1m … 5m", "1m..5m", "1m,5m", "1m 5m", "  1m   5m  "] {
            assert_eq!(
                parse_axis_range(text),
                Some((1.0e-3, 5.0e-3)),
                "failed on {text:?}"
            );
        }
        assert_eq!(parse_axis_range("-2.5 -1.5"), Some((-2.5, -1.5)));
    }

    #[test]
    fn a_reversed_or_degenerate_interval_is_refused_rather_than_sorted() {
        // On a log axis "5m … 1m" is not the same request as "1m … 5m", and
        // quietly swapping them would hide the typo behind a correct-looking
        // plot.
        assert_eq!(parse_axis_range("5m … 1m"), None);
        assert_eq!(parse_axis_range("1m … 1m"), None);
        assert_eq!(parse_axis_range("1m"), None);
        assert_eq!(parse_axis_range("1m … 2m … 3m"), None);
        assert_eq!(parse_axis_range(""), None);
        assert_eq!(parse_axis_range("wide open"), None);
    }

    /// An explicit interval has to land in the store the sheet reads.
    ///
    /// The waveform stack keys its viewports by analysis and every other
    /// sheet by plot ordinal. A previous fit command wrote to the ordinal
    /// store for all of them, so Fit was a silent no-op on the four sheets
    /// people actually use. The same trap is waiting for anything that
    /// writes a viewport, so this asserts through the reader, not the map.
    #[test]
    fn an_explicit_interval_lands_in_the_store_its_own_sheet_reads() {
        let tokens = Tokens::default();
        let mut app = app_showing(ResultViewer::Waves);
        let analysis = active_analysis_key(&app.state);
        app.state.ui.results.active_wave_pane = Some(WavePanePresentationKey {
            analysis,
            unit: "V".to_owned(),
        });

        assert!(set_active_axis_range(
            &tokens,
            &mut app.state,
            PaneAxis::X,
            Some((1.0e-6, 4.0e-6))
        ));
        assert_eq!(
            app.state
                .ui
                .results
                .analysis_plot_view_pane(ResultViewer::Waves, analysis, 0)
                .x,
            Some((1.0e-6, 4.0e-6)),
            "the waveform stack must read its own analysis-keyed viewport"
        );
        assert!(active_axis_is_pinned(&app.state, PaneAxis::X));

        set_active_axis_range(&tokens, &mut app.state, PaneAxis::X, None);
        assert!(!active_axis_is_pinned(&app.state, PaneAxis::X));

        let mut app = app_showing(ResultViewer::Smith);
        app.state.ui.results.viewer = ResultViewer::Smith;
        assert!(set_active_axis_range(
            &tokens,
            &mut app.state,
            PaneAxis::Y,
            Some((-0.5, 0.5))
        ));
        assert_eq!(
            app.state.ui.results.plot_view(ResultViewer::Smith, 0).y,
            Some((-0.5, 0.5)),
            "a single-canvas sheet keeps its viewport under the plot ordinal"
        );
        assert!(active_axis_is_pinned(&app.state, PaneAxis::Y));
    }

    /// The editor has to open on the interval the reader can see, including
    /// on a sheet that is fitting its data rather than pinned.
    #[test]
    fn the_axis_editor_opens_on_what_the_sheet_actually_drew() {
        let mut app = app_showing(ResultViewer::Smith);
        app.state.ui.results.viewer = ResultViewer::Smith;
        draw_sheet_and_tessellate(&mut app, ResultViewer::Smith);

        let facts = ActivePaneFacts {
            unit: None,
            analysis: None,
            traces: None,
            runs: None,
            scale: None,
            limit_mask: "none bound",
            x_viewport: None,
            y_viewport: None,
            x_extent: None,
            y_extent: None,
            pinned: None,
        };
        let x = active_axis_range(&app.state, &facts, PaneAxis::X)
            .expect("the Smith sheet recorded the interval it drew");
        assert!(
            x.0 < x.1 && x.0.is_finite() && x.1.is_finite(),
            "recorded a degenerate interval: {x:?}"
        );
        assert!(
            !active_axis_is_pinned(&app.state, PaneAxis::X),
            "a fitted sheet must not report itself pinned just because it drew"
        );
    }

    /// Every sheet, drawn at the touch composition the tablet build uses.
    ///
    /// The shell has raised its own rows to a 44 px target for a long time
    /// while this workspace kept fixed workstation heights, so its chips and
    /// icon buttons grew past the bands holding them. A frame is what proves
    /// the composition survives, not the constants alone.
    #[test]
    fn every_sheet_draws_at_a_touch_composition() {
        for viewer in ResultViewer::every() {
            let mut app = app_showing(viewer);
            app.state.ui.results.viewer = viewer;
            let ctx = egui::Context::default();
            crate::ui::Theme::default().apply(&ctx);
            crate::ui::Theme::default()
                .apply_responsive_metrics_with_target(&ctx, Some(crate::ui::tokens::TOUCH_TARGET));
            assert!(
                Tokens::get(&ctx).metrics.is_touch(),
                "the fixture did not reach a touch composition"
            );
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_024.0, 1_366.0),
                )),
                ..Default::default()
            };
            for _ in 0..2 {
                let output = ctx.run_ui(input.clone(), |ctx| {
                    egui::CentralPanel::default()
                        .show(ctx, |ui| show_persistent_pane_viewer(ui, &mut app, viewer));
                });
                for primitive in ctx.tessellate(output.shapes, output.pixels_per_point) {
                    let egui::epaint::Primitive::Mesh(mesh) = primitive.primitive else {
                        continue;
                    };
                    assert!(
                        mesh.vertices
                            .iter()
                            .all(|vertex| vertex.pos.x.is_finite() && vertex.pos.y.is_finite()),
                        "{viewer:?} put a non-finite vertex in the mesh at a touch composition",
                    );
                }
            }
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

    fn transient_state() -> AppState {
        state_with_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e"),
            ]),
        )
    }

    fn active_analysis_key(state: &AppState) -> AnalysisPresentationKey {
        let run = state.simulation.active_run().expect("active retained run");
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
    }

    /// The defect this pins: the waveform sheets key their viewports by
    /// analysis, so a fit written against the plot-ordinal store cleared
    /// nothing. The pin writes through the production path deliberately —
    /// a test that writes the ordinal store proves only that the store works.
    #[test]
    fn fitting_the_wave_stack_clears_the_viewport_the_sheet_actually_reads() {
        let mut state = transient_state();
        let key = active_analysis_key(&state);
        state
            .ui
            .results
            .analysis_plot_view_pane_mut(ResultViewer::Waves, key, 0)
            .y = Some((0.0, 1.0));
        state
            .ui
            .results
            .analysis_plot_view_pane_mut(ResultViewer::Waves, key, 1)
            .x = Some((0.0, 0.5));
        state.ui.results.active_wave_pane = Some(WavePanePresentationKey {
            analysis: key,
            unit: "V".to_owned(),
        });

        waves::fit_active_strip(&mut state);

        for ordinal in 0..2 {
            assert!(
                !state
                    .ui
                    .results
                    .analysis_plot_view_pane(ResultViewer::Waves, key, ordinal)
                    .is_zoomed(),
                "pane {ordinal} kept a pinned viewport after the strip was fitted"
            );
        }
    }

    #[test]
    fn fitting_without_an_active_pane_releases_every_strip() {
        let mut state = transient_state();
        let key = active_analysis_key(&state);
        state
            .ui
            .results
            .analysis_plot_view_pane_mut(ResultViewer::Waves, key, 0)
            .y = Some((0.0, 1.0));
        state.ui.results.active_wave_pane = None;

        waves::fit_active_strip(&mut state);

        assert!(
            !state
                .ui
                .results
                .analysis_plot_view_pane(ResultViewer::Waves, key, 0)
                .is_zoomed()
        );
    }

    /// Magnifying needs the sheet's retained extents, which only the unit-pane
    /// stack exposes. Offering it elsewhere would be an enabled control that
    /// cannot act.
    #[test]
    fn zoom_is_offered_only_where_it_can_be_carried_out() {
        let mut state = transient_state();
        state.ui.results.viewer = ResultViewer::Waves;
        assert!(zoom_gesture_available(&state));
        assert!(fit_gesture_available(&state));

        state.ui.results.viewer = ResultViewer::Fft;
        assert!(!zoom_gesture_available(&state));
        assert!(
            fit_gesture_available(&state),
            "a single-canvas plot still has a viewport to release"
        );

        state.ui.results.viewer = ResultViewer::Table;
        assert!(!fit_gesture_available(&state));
        assert!(!zoom_gesture_available(&state));
    }

    /// A signal whose name declares no accessor reads in its analysis' own
    /// quantity. The calculator used to call every such signal a voltage.
    #[test]
    fn an_unqualified_signal_reads_in_its_analysis_quantity() {
        assert_eq!(analysis_default_unit(AnalysisType::Noise), "V^2/Hz");
        assert_eq!(analysis_default_unit(AnalysisType::Ac), "dB");
        assert_eq!(analysis_default_unit(AnalysisType::Transient), "V");
        assert_eq!(
            browser_signal_unit(
                "onoise_spectrum",
                analysis_default_unit(AnalysisType::Noise)
            ),
            "V^2/Hz"
        );
        assert_eq!(
            browser_signal_unit("I(VDD)", analysis_default_unit(AnalysisType::Noise)),
            "A",
            "an explicit accessor still wins over the analysis default"
        );
    }

    #[test]
    fn restored_markers_keep_their_labels_and_advance_the_id_allocator() {
        let mut state = transient_state();
        let key = active_analysis_key(&state);
        let anchor = WaveformPresentationKey {
            analysis: key,
            trace: TracePresentationKey {
                source_name: "V(out)".to_owned(),
                kind: 0,
                family_group: 0,
            },
        };
        let marker = ResultMarker {
            id: 7,
            analysis: key,
            anchor,
            trace_name: "V(out)".to_owned(),
            x: 0.5,
            kind: MarkerKind::Peak,
            note: "settling".to_owned(),
        };

        restore_markers(&mut state, vec![marker]);

        assert_eq!(state.ui.results.markers.len(), 1);
        assert_eq!(state.ui.results.markers[0].note, "settling");
        let next = state.ui.results.add_marker(
            key,
            state.ui.results.markers[0].anchor.clone(),
            "V(out)".to_owned(),
            0.75,
        );
        assert!(
            next > 7,
            "a restored label must not be handed out a second time"
        );
    }

    /// A marker naming a dataset the reopened project no longer retains has
    /// nothing to draw on, and must not be adopted as if it did.
    #[test]
    fn markers_for_absent_datasets_are_dropped_on_restore() {
        let mut state = transient_state();
        let key = active_analysis_key(&state);
        let foreign = AnalysisPresentationKey::new(
            crate::product::DatasetId::new(),
            &AnalysisResult::new(9, AnalysisType::Transient, "TRAN"),
        );
        let anchor = WaveformPresentationKey {
            analysis: foreign,
            trace: TracePresentationKey {
                source_name: "V(gone)".to_owned(),
                kind: 0,
                family_group: 0,
            },
        };
        let kept = ResultMarker {
            id: 1,
            analysis: key,
            anchor: anchor.clone(),
            trace_name: "V(out)".to_owned(),
            x: 0.1,
            kind: MarkerKind::Note,
            note: String::new(),
        };
        let dropped = ResultMarker {
            id: 2,
            analysis: foreign,
            anchor,
            trace_name: "V(gone)".to_owned(),
            x: 0.1,
            kind: MarkerKind::Note,
            note: String::new(),
        };

        restore_markers(&mut state, vec![kept, dropped]);

        assert_eq!(state.ui.results.markers.len(), 1);
        assert_eq!(state.ui.results.markers[0].id, 1);
    }
}
