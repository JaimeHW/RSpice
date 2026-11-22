//! Local result viewer controls, selections, annotations and viewports.
use crate::{
    chrome::instrument::ResultPlotTool,
    derived::DerivedSeries,
    events::DigitalEventSelection,
    eye_diagram::view::EyeTexture,
    fft::view::FftSeries,
    presentation::PlotView,
    selection::{ResultBrowserSelectionKey, SelectedResultTrace},
    soa::{SoaRuleFilter, SoaRuleSelection},
};
use rspice_results::{
    events::projection::BusRadix,
    family_projection::SourceSampleSelection,
    result_presentation::{
        AnalysisPresentationKey, AnalysisPresentationSource, ExprTrace, MarkerKind,
        ResultArtifactPresentationKey, ResultViewer, SourceWaveformPresentationKey,
        WavePanePresentationKey, WaveformPresentationKey, viewer_uses_wave_stack,
    },
    visualization_document::{MarkerId, PaneId},
};
use rspice_ui_kit::plot::{CursorId, CursorPair, DecimationCache};
use rspice_vocabulary::product::{DatasetId, ResultDocumentId};
use std::collections::{HashMap, HashSet};

/// One axis interval, low then high, in data space.
pub type AxisExtent = (f64, f64);
/// The X and Y intervals one plot last drew.
pub type DrawnAxes = (AxisExtent, AxisExtent);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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

/// Where one marker lives: the view that keeps it, the pane, and the view's
/// own serial for it at full width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkerSelector {
    pub document_id: ResultDocumentId,
    pub pane_id: PaneId,
    pub marker_id: MarkerId,
}

/// An uncommitted edit of one marker's purpose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkerEditDraft {
    pub selector: MarkerSelector,
    pub note: String,
    pub kind: MarkerKind,
    /// A crossing's level as typed; `None` until the dialog first shows it.
    pub level: Option<TypedEntry>,
    /// An offset's amount as typed; `None` until the dialog first shows it.
    pub by: Option<TypedEntry>,
}

/// A number typed in a dialog beside the text it was seeded with, so one
/// left as it was keeps the value it was seeded from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedEntry {
    pub seed: String,
    pub text: String,
}

/// Per-frame projection of the markers the view being drawn keeps.
///
/// The view, a saved document or a working view, is the only owner of
/// these: the overlay is rebuilt by projection every frame and never
/// serialized. It lets the renderer and the readout draw a marker without
/// reading the view.
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

/// One marker to draw or list.
#[derive(Debug, Clone, Copy)]
pub struct MarkerView<'a>(pub &'a DocumentMarker);

impl<'a> MarkerView<'a> {
    pub fn selector(self) -> MarkerSelector {
        MarkerSelector {
            document_id: self.0.document_id,
            pane_id: self.0.pane_id,
            marker_id: self.0.retained_id,
        }
    }

    pub fn analysis(self) -> AnalysisPresentationKey {
        self.0.analysis
    }

    pub fn anchor(self) -> &'a WaveformPresentationKey {
        &self.0.anchor
    }

    pub fn trace_name(self) -> &'a str {
        &self.0.trace_name
    }

    pub fn x(self) -> f64 {
        self.0.x
    }

    pub fn kind(self) -> MarkerKind {
        self.0.kind
    }

    pub fn note(self) -> &'a str {
        &self.0.note
    }

    /// What the tag reads on the plot and in the readout.
    pub fn display_id(self) -> String {
        format!("M{}", self.0.retained_id.get())
    }
}

/// One column of the Cursors table: a vertical cursor or a marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadingColumnId {
    Cursor(CursorId),
    Marker(MarkerSelector),
}

/// A waveform pane's horizontal measurement cursors, H1 and H2, as values
/// on the Y axis that pane draws.
pub type HorizontalCursors = [Option<f64>; 2];

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

/// A form waiting for the reader to pick a trace in the strip, as ViVA's
/// calculator waits for a waveform clicked in the graph: while armed, a
/// click on a plot takes the curve nearest it, and a click on a legend entry
/// takes that entry, instead of placing a cursor.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum TracePick {
    #[default]
    Off,
    Armed,
    /// The trace the reader clicked: its waveform in the dataset, and the
    /// name the strip shows it by.
    Picked {
        source: String,
        name: String,
    },
}

impl TracePick {
    pub const fn is_armed(&self) -> bool {
        matches!(self, Self::Armed)
    }

    /// Take `trace` when armed; otherwise leave the pick as it is.
    pub fn take(&mut self, trace: &crate::waves::StripTrace) {
        if self.is_armed() {
            *self = Self::Picked {
                source: trace.source_waveform_name.clone(),
                name: trace.name.clone(),
            };
        }
    }
}

/// A strip that draws one unit pane's curves against a signal or an
/// expression of its analysis, over a span of the analysis' sweep. It stands
/// after its analysis' strip, which goes on drawing against the sweep.
#[derive(Debug, Clone, PartialEq)]
pub struct XyStrip {
    pub analysis: AnalysisPresentationKey,
    /// The unit pane whose curves it draws, by its unit.
    pub unit: String,
    pub abscissa: rspice_results::visualization_document::Abscissa,
    /// The span of the sweep drawn; the whole record when `None`.
    pub sweep_range: Option<(f64, f64)>,
    /// Whether the span follows the window of the sweep its analysis' strip
    /// shows, as a scope's XY display follows its zoom, in place of
    /// `sweep_range`.
    pub follow_view: bool,
    /// Whether the abscissa is read over decades.
    pub log_x: bool,
    /// The abscissa range shown, when the reader zoomed or panned it.
    pub x_view: Option<(f64, f64)>,
    /// The ordinate range shown, when the reader zoomed or panned it.
    pub y_view: Option<(f64, f64)>,
}

impl XyStrip {
    pub fn new(
        analysis: AnalysisPresentationKey,
        unit: impl Into<String>,
        abscissa: rspice_results::visualization_document::Abscissa,
    ) -> Self {
        Self {
            analysis,
            unit: unit.into(),
            abscissa,
            sweep_range: None,
            follow_view: false,
            log_x: false,
            x_view: None,
            y_view: None,
        }
    }

    /// Whether this strip draws `unit`'s curves of `analysis` against
    /// `abscissa`: a strip's identity.
    pub fn is(
        &self,
        analysis: AnalysisPresentationKey,
        unit: &str,
        abscissa: &rspice_results::visualization_document::Abscissa,
    ) -> bool {
        self.analysis == analysis && self.unit == unit && &self.abscissa == abscissa
    }
}

/// The strips drawn against a signal or an expression, in the order the
/// reader made them.
pub type XyStrips = Vec<XyStrip>;

/// A strip a document pane keeps drawn against a signal or an expression.
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentXyPlot {
    /// The unit pane whose curves it draws, by its unit.
    pub unit: String,
    pub abscissa: rspice_results::visualization_document::Abscissa,
    /// The span of the sweep drawn; the whole record when `None`.
    pub sweep_range: Option<(f64, f64)>,
    /// Whether the span follows the analysis' strip.
    pub follow_view: bool,
    /// Whether the abscissa is read over decades.
    pub log_x: bool,
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

/// The family members the reader picked: brushed in the scatter sheet or
/// clicked as a histogram bin. Every sheet that shows members reads and
/// writes this one selection, so a trial picked in one is marked in the
/// others.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemberSelection {
    source: Option<AnalysisPresentationKey>,
    rows: Vec<usize>,
}

impl MemberSelection {
    /// The rows picked on `source`, in the population's trial order. A
    /// selection made on another result is dropped rather than read as rows
    /// of this one.
    pub fn on(&mut self, source: AnalysisPresentationKey) -> &mut Vec<usize> {
        if self.source != Some(source) {
            self.source = Some(source);
            self.rows.clear();
        }
        &mut self.rows
    }

    /// The rows picked on `source`, without taking the selection over.
    #[must_use]
    pub fn rows_on(&self, source: AnalysisPresentationKey) -> &[usize] {
        if self.source == Some(source) {
            &self.rows
        } else {
            &[]
        }
    }
}

/// One window's own place in Results: the analysis of its run it shows, the
/// sheet, the cursors, the zoom and the selection. Two windows on one project
/// each keep theirs; the run a window shows is its active document. Saved
/// with the session, except the selection, which names one run's traces.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct WindowResultsView {
    /// The run the window shows.
    pub run: Option<DatasetId>,
    /// The shown run's analysis, by its place in the run.
    pub analysis: Option<usize>,
    /// The sheet; `None` until the window has shown Results, and for a
    /// sheet this build does not have.
    #[serde(deserialize_with = "known_viewer")]
    pub viewer: Option<ResultViewer>,
    /// The quick view's sheet while the window shows a saved document's.
    #[serde(deserialize_with = "known_viewer")]
    pub quick_view_viewer: Option<ResultViewer>,
    pub cursor_a: Option<f64>,
    pub cursor_b: Option<f64>,
    pub cursor_strip: Option<usize>,
    pub horizontal_cursors: Vec<(WavePanePresentationKey, HorizontalCursors)>,
    /// The zoom of each plot the window zoomed.
    pub views: Vec<SavedPlotView>,
    #[serde(skip)]
    pub cursor_a_anchor: Option<WaveformPresentationKey>,
    #[serde(skip)]
    pub stepping_cursor: CursorId,
    #[serde(skip)]
    pub selected_trace: Option<SelectedResultTrace>,
    #[serde(skip)]
    pub selected_result_artifact: Option<ResultArtifactPresentationKey>,
    #[serde(skip)]
    pub active_wave_pane: Option<WavePanePresentationKey>,
}

/// A saved sheet, or none for one this build does not have.
fn known_viewer<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<ResultViewer>, D::Error> {
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum Saved {
        Known(ResultViewer),
        Other(serde::de::IgnoredAny),
    }
    Ok(
        match <Option<Saved> as serde::Deserialize>::deserialize(deserializer)? {
            Some(Saved::Known(viewer)) => Some(viewer),
            Some(Saved::Other(_)) | None => None,
        },
    )
}

impl WindowResultsView {
    /// This view, keeping the run and analysis `previous` showed while this
    /// one shows none: a window whose run is not loaded yet, or was put
    /// away, goes back to it when it is.
    #[must_use]
    pub fn keeping_the_run_of(mut self, previous: &Self) -> Self {
        if self.run.is_none() {
            self.run = previous.run;
            self.analysis = previous.analysis;
        }
        self
    }
}

/// The zoom one plot keeps, as a window saves it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SavedPlotView {
    pub viewer: ResultViewer,
    pub plot: PlotPresentationKey,
    pub ordinal: usize,
    pub x: Option<AxisExtent>,
    pub y: Option<AxisExtent>,
}

impl ResultViewerState {
    /// What of this state is the shown window's own, for when another window
    /// is drawn. `analysis` is the shown run's analysis.
    pub fn window_view(
        &self,
        run: Option<DatasetId>,
        analysis: Option<usize>,
    ) -> WindowResultsView {
        let mut views: Vec<_> = self
            .views
            .iter()
            .map(|(&(viewer, plot, ordinal), view)| SavedPlotView {
                viewer,
                plot,
                ordinal,
                x: view.x,
                y: view.y,
            })
            .collect();
        views
            .sort_by_cached_key(|saved| format!("{:?}", (saved.viewer, saved.plot, saved.ordinal)));
        let mut horizontal_cursors: Vec<_> = self
            .horizontal_cursors
            .iter()
            .map(|(pane, cursors)| (pane.clone(), *cursors))
            .collect();
        horizontal_cursors.sort_by_cached_key(|(pane, _)| format!("{pane:?}"));
        WindowResultsView {
            run,
            analysis,
            viewer: Some(self.viewer),
            quick_view_viewer: None,
            cursor_a: self.cursors.a,
            cursor_b: self.cursors.b,
            cursor_strip: self.cursor_strip,
            horizontal_cursors,
            views,
            cursor_a_anchor: self.cursor_a_anchor.clone(),
            stepping_cursor: self.stepping_cursor,
            selected_trace: self.selected_trace.clone(),
            selected_result_artifact: self.selected_result_artifact.clone(),
            active_wave_pane: self.active_wave_pane.clone(),
        }
    }

    /// Put a window's own state back, as it was when it was last drawn. A
    /// window that never showed Results keeps the sheet on screen.
    pub fn show_window_view(&mut self, view: &WindowResultsView) {
        if let Some(viewer) = view.viewer {
            self.viewer = viewer;
        }
        self.cursors.a = view.cursor_a;
        self.cursors.b = view.cursor_b;
        self.cursor_strip = view.cursor_strip;
        self.horizontal_cursors = view.horizontal_cursors.iter().cloned().collect();
        self.views = view
            .views
            .iter()
            .map(|saved| {
                (
                    (saved.viewer, saved.plot, saved.ordinal),
                    PlotView {
                        x: saved.x,
                        y: saved.y,
                    },
                )
            })
            .collect();
        self.cursor_a_anchor = view.cursor_a_anchor.clone();
        self.stepping_cursor = view.stepping_cursor;
        self.selected_trace = view.selected_trace.clone();
        self.selected_result_artifact = view.selected_result_artifact.clone();
        self.active_wave_pane = view.active_wave_pane.clone();
    }
}

/// What a view gives one signal's trace beyond its colour: the style the
/// reader chose and the name they gave it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct WaveformLook {
    pub style: rspice_results::visualization_document::TraceStyle,
    /// The name the reader gave the trace; `None` keeps its signal's.
    pub name: Option<String>,
}

/// The reader's ticks: a signal named here is on the plot or off it whatever
/// its dataset chose. An edit gives them a new revision, so what is built
/// from them asks whether they changed without reading them all, and which
/// signals an edit touched is kept, so what was built from an earlier
/// revision changes by those signals alone. Edits made while nothing holds
/// the revision they leave share one: a gesture or a drawing of many
/// signals is one change to whatever reads them next.
///
/// They also keep which signals the reader set since a view last took them,
/// so what a frame changed is read without comparing them all.
#[derive(Debug, Default)]
pub struct VisibilityOverrides {
    overrides: HashMap<SourceWaveformPresentationKey, bool>,
    revision: rspice_vocabulary::source_revision::SourceRevision,
    /// The signals [`Self::set`] changed since [`Self::take_set`]; `None`
    /// once a mutable borrow may have changed any of them.
    set_since: Option<Vec<SourceWaveformPresentationKey>>,
    /// The signals edited one at a time, oldest first. An edit that does
    /// not say which signals it touched empties it.
    edited: Vec<SourceWaveformPresentationKey>,
    /// Each revision an edit left while something held it, oldest first,
    /// with where in `edited` the edits since it begin.
    left: Vec<(rspice_vocabulary::source_revision::SourceRevision, usize)>,
}

impl Clone for VisibilityOverrides {
    /// The ticks, their revision and what the reader set; the copy's edits
    /// start from it.
    fn clone(&self) -> Self {
        Self {
            overrides: self.overrides.clone(),
            revision: self.revision.clone(),
            set_since: self.set_since.clone(),
            edited: Vec::new(),
            left: Vec::new(),
        }
    }
}

impl VisibilityOverrides {
    /// Equal to an earlier revision while no edit was made since it.
    pub fn revision(&self) -> rspice_vocabulary::source_revision::SourceRevision {
        self.revision.clone()
    }

    /// The reader shows `key`'s signal (`Some(true)`), hides it
    /// (`Some(false)`) or leaves it to its dataset (`None`); kept as one the
    /// reader set. Whether that changed anything.
    pub fn set(&mut self, key: SourceWaveformPresentationKey, visible: Option<bool>) -> bool {
        if !self.put(&key, visible) {
            return false;
        }
        if let Some(set) = &mut self.set_since {
            set.push(key);
        }
        true
    }

    /// [`Self::set`] for what a view draws, not what the reader set.
    pub fn project(&mut self, key: &SourceWaveformPresentationKey, visible: Option<bool>) -> bool {
        self.put(key, visible)
    }

    /// Make `analysis`' overrides exactly `desired`, for what a view draws,
    /// not what the reader set. Whether that changed anything.
    pub fn project_analysis(
        &mut self,
        analysis: AnalysisPresentationKey,
        desired: HashMap<SourceWaveformPresentationKey, bool>,
    ) -> bool {
        let gone: Vec<SourceWaveformPresentationKey> = self
            .overrides
            .keys()
            .filter(|key| key.analysis() == analysis && !desired.contains_key(*key))
            .cloned()
            .collect();
        let mut changed = false;
        for key in &gone {
            changed |= self.put(key, None);
        }
        for (key, visible) in &desired {
            changed |= self.put(key, Some(*visible));
        }
        changed
    }

    /// The signals whose ticks were edited since `revision`, oldest first,
    /// a signal edited twice named twice; `None` when the edits since it are
    /// no longer known.
    pub fn edits_since(
        &self,
        revision: &rspice_vocabulary::source_revision::SourceRevision,
    ) -> Option<impl Iterator<Item = &SourceWaveformPresentationKey>> {
        let from = if *revision == self.revision {
            self.edited.len()
        } else {
            self.left.iter().rev().find(|(left, _)| left == revision)?.1
        };
        Some(self.edited[from..].iter())
    }

    /// How many signals [`Self::take_set`] would give, each as often as it
    /// was set; `None` as it gives none.
    pub fn set_count(&self) -> Option<usize> {
        self.set_since.as_ref().map(Vec::len)
    }

    /// The signals the reader set since this was last asked, each as often
    /// as it was set, and forget them: `None` when anything may have
    /// changed any signal.
    pub fn take_set(&mut self) -> Option<Vec<SourceWaveformPresentationKey>> {
        self.set_since.replace(Vec::new())
    }

    fn put(&mut self, key: &SourceWaveformPresentationKey, visible: Option<bool>) -> bool {
        // The ticks copy the key only for a signal they did not hold.
        let changed = match visible {
            Some(visible) => match self.overrides.get_mut(key) {
                Some(held) => std::mem::replace(held, visible) != visible,
                None => {
                    self.overrides.insert(key.clone(), visible);
                    true
                }
            },
            None => self.overrides.remove(key).is_some(),
        };
        if changed {
            self.note(key.clone());
        }
        changed
    }

    /// Record an edit of `key`'s tick, under a new revision when something
    /// holds the one it leaves. Edits beyond twice the ticks are let go:
    /// rebuilding from the ticks then reads no more than replaying the
    /// edits would.
    fn note(&mut self, key: SourceWaveformPresentationKey) {
        if self.revision.is_observed() {
            self.left.push((self.revision.clone(), self.edited.len()));
            self.revision.advance();
        }
        self.edited.push(key);
        if self.edited.len() > 2 * self.overrides.len().max(1) {
            let gone = self.edited.len() / 2;
            self.edited.drain(..gone);
            self.left.retain_mut(|(_, from)| {
                let kept = *from >= gone;
                *from = from.saturating_sub(gone);
                kept
            });
        }
    }
}

impl std::ops::Deref for VisibilityOverrides {
    type Target = HashMap<SourceWaveformPresentationKey, bool>;

    fn deref(&self) -> &Self::Target {
        &self.overrides
    }
}

impl std::ops::DerefMut for VisibilityOverrides {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Any mutable borrow may edit them, any of them, and does not say
        // which.
        self.edited.clear();
        self.left.clear();
        self.revision.advance();
        self.set_since = None;
        &mut self.overrides
    }
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
        rspice_vocabulary::product::ResultDocumentId,
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
        std::collections::HashMap<rspice_vocabulary::product::ResultDocumentId, PaneId>,
    /// Exact retained pane currently projected through a native renderer.
    /// This scopes otherwise session-only plot state and routes edits back to
    /// the project-owned visualization document.
    pub persistent_pane_context: Option<PersistentPaneContext>,
    /// The panes of the working view the quick view draws, one per analysis
    /// of the shown run it has a pane for. Like the persistent context, it
    /// routes what the reader does on a strip to the view that keeps it.
    pub working_view_panes: Vec<PersistentPaneContext>,
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
    pub polar: Box<crate::polar::PolarSheetState>,
    /// Smith sheet controls: the grid, the response reading and Z₀, the
    /// responses drawn, and the two-port circles.
    pub smith: Box<crate::smith_chart::SmithSheetState>,
    /// Contribution sheet control: the frequency a swept study is read at.
    pub study: Box<crate::sensitivity::study::SensitivitySheetState>,
    /// Scatter sheet controls.
    pub scatter: Box<crate::scatter::ScatterSheetState>,
    /// Sweep sheet controls: the value drawn, the swept quantity on X and
    /// where a signal is read.
    pub sweep: Box<crate::sweep::SweepSheetState>,
    /// The table at the optimization sheet's foot.
    pub optimization_table: crate::optimization::OptimizationTable,
    /// The family members the reader picked, in any sheet that shows them.
    /// Boxed: the results slot is built by value and has a stack budget.
    pub members: Box<MemberSelection>,
    /// Box/violin sheet controls: the grouping, the margin scale, the body
    /// and the whisker rule.
    pub box_violin: Box<crate::box_violin::BoxViolinSheetState>,
    /// The marker tool.
    pub marker_tool: MarkerTool,
    /// A trace a form is waiting for the reader to click in the strip.
    pub trace_pick: TracePick,
    /// Primary plot pointer tool shown in the 31 px instrument strip.
    pub plot_tool: ResultPlotTool,
    /// Exact retained waveform selected from a trace chip. This is only an
    /// identity into the canonical result dataset; it never copies samples or
    /// creates a second result owner.
    pub selected_trace: Option<SelectedResultTrace>,
    /// The traces Ctrl-clicked beside the selected one, of its strip: the
    /// inspector and the Style menu style them together with it.
    pub also_selected_traces: Vec<SelectedResultTrace>,
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
    /// Which of its lists the Events sheet shows, and which rows of it: the
    /// search and the time window. One per sheet, as the radix is.
    pub event_filter: crate::events::EventFilter,
    /// The logic and bus lanes each waveform strip shows above its panes:
    /// their order, each bus's radix and expansion, the reader's buses and
    /// the selected rows. Which lanes exist follows the plotted `D()`
    /// columns; this holds what the run cannot say.
    pub event_lanes: HashMap<AnalysisPresentationKey, crate::waves::lanes::EventLaneLayout>,
    /// The width of the waveform strips' name column, as the reader dragged
    /// it; `None` until they do. One width for every strip, so stacked strips
    /// keep their lanes and panes on one edge.
    pub lane_name_width: Option<f32>,
    /// Most-recent-first stable waveform identities the user selected or
    /// revealed, deduplicated and bounded: the browser's Recent scope is this
    /// order.
    pub recent_signals: Vec<SourceWaveformPresentationKey>,
    /// Most-recent-first typed artifact identities.
    pub recent_result_artifacts: Vec<ResultArtifactPresentationKey>,
    /// Quantities ticked in the browser that have no curve to plot: a
    /// waveform's tick is its plot membership, and only that. Session state
    /// like the marks above it; the immutable dataset never sees it and equal
    /// names in another retained analysis are not selected implicitly.
    pub checked_result_quantities: HashSet<ResultBrowserSelectionKey>,
    /// Stable end point for Shift+click / Shift+Arrow range selection in the
    /// filtered browser inventory. The range itself is recomputed from the
    /// current deterministic row order, so filtering never leaves an ordinal
    /// pointing at a different quantity.
    pub browser_range_anchor: Option<ResultBrowserSelectionKey>,
    /// The open marker-purpose dialog's uncommitted edit, if any.
    ///
    /// Editing is transactional: nothing reaches the marker until Apply, so
    /// a half-typed label can always be abandoned. Reclassifying a marker is
    /// a decision, not a side effect of clicking its kind.
    pub marker_edit: Option<MarkerEditDraft>,
    /// A/B cursors (data-space X of the strip they live on).
    pub cursors: CursorPair,
    /// Dataset-bound strip identity the cursors were placed on.
    pub cursor_strip: Option<usize>,
    /// Exact visible trace nearest cursor A when A was placed.
    pub cursor_a_anchor: Option<WaveformPresentationKey>,
    /// Unit-scoped waveform pane receiving instrument actions.
    pub active_wave_pane: Option<WavePanePresentationKey>,
    /// A waveform pane whose "Plot against" choices the command list asked
    /// for: the stack opens them over the pane the next time it draws it.
    pub plot_against_request: Option<WavePanePresentationKey>,
    /// The cursor the arrow keys and the jump commands move. A click that
    /// places a cursor makes it the stepping one, as does dragging it.
    pub stepping_cursor: CursorId,
    /// Horizontal cursors, kept per pane because each is a value on the Y
    /// axis of the pane it was placed on: placing them on one pane leaves
    /// another pane's where they were.
    pub horizontal_cursors: HashMap<WavePanePresentationKey, HorizontalCursors>,
    /// The column the Cursors table reads every other column's differences
    /// against: the first column while `None` or once it is gone.
    pub readout_base: Option<ReadingColumnId>,
    /// Cursors and placed markers stand on retained samples rather than
    /// between them: the Cursors page's Snap.
    pub snap_to_samples: bool,
    /// Unit-scoped panes the user put on a logarithmic Y axis.
    ///
    /// This lived in egui's persisted memory under a hand-built id, which put
    /// a project-scoped presentation decision outside every owner that knows
    /// about projects: `clear_project_scoped_state` could not reach it, the
    /// project file could not carry it, and it accumulated an entry for every
    /// dataset the user ever opened. It is the same class of fact as a marker
    /// and it is kept the same way.
    pub log_y_panes: HashSet<WavePanePresentationKey>,
    /// What the strips of a running transient keep in view as they grow.
    pub live_extent: crate::waves::extent::LiveExtent,
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
    /// Presentation-only exact family rows the drawn sheet shows. Source
    /// datasets are never modified by this projection.
    pub sample_selection: Option<SourceSampleSelection>,
    /// Family slicing applied over a quick view, per analysis. A project
    /// result document keeps its own with each pane instead.
    pub family_policies: HashMap<
        AnalysisPresentationKey,
        rspice_results::visualization_document::FamilyPresentationPolicy,
    >,
    /// Session-only visibility overrides for exact family group traces. These
    /// are presentation state and never alter source RetainedTrace visibility.
    pub hidden_family_traces: HashSet<crate::waves::FamilyTraceVisibilityKey>,
    /// Session-only source-waveform visibility overrides for quick views.
    ///
    /// A missing key means "use the immutable dataset default". Keeping this
    /// tri-state representation allows a source that defaults hidden to be
    /// revealed without writing into solver-owned result data.
    pub waveform_visibility: VisibilityOverrides,
    /// The retained analyses' signals by name, made as they are asked for.
    pub signal_indices: crate::signal_index::SignalIndices,
    /// The colours the view being drawn gives its traces, by signal. A
    /// missing key draws the signal in the run's colour or its palette place;
    /// the run keeps no colour a reader chose.
    pub waveform_colors:
        HashMap<SourceWaveformPresentationKey, rspice_results::visualization_document::TraceColor>,
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
    /// The unit the reader declared for an expression, by its strip and
    /// text, which replaces the unit unit algebra infers for it. Boxed, as
    /// the editor's unit is, to keep the results state inside its stack
    /// budget.
    pub expression_units: Box<std::collections::HashMap<(AnalysisPresentationKey, String), String>>,
    /// The unit typed in the open expression editor; empty to take the
    /// inferred one.
    pub expr_editor_unit: Box<String>,
    /// The X scale the reader chose for a strip, in place of its analysis'
    /// own (logarithmic for a frequency sweep, linear otherwise). Boxed to
    /// keep the results state inside its stack budget.
    pub x_scales:
        Box<std::collections::HashMap<AnalysisPresentationKey, rspice_ui_kit::plot::XScale>>,
    /// The strips that draw a unit pane against a signal or an expression,
    /// each after its analysis' strip. Boxed, as the X scales are.
    pub xy_strips: Box<XyStrips>,
    /// The style and name the view being drawn gives each signal's trace
    /// beyond its colour.
    pub waveform_looks: HashMap<SourceWaveformPresentationKey, WaveformLook>,
    /// The strips the reader arranged each analysis' waveform stack into, as
    /// the view being drawn keeps them; an analysis it has none for gives
    /// each unit a strip of its own.
    pub strip_layouts:
        HashMap<AnalysisPresentationKey, Vec<rspice_results::visualization_document::Strip>>,
    /// What the reader set the eye sheet to, per result: the signal and the
    /// clock that fold it, its levels, the error rate it is read at and its
    /// mask.
    ///
    /// Absent means the sheet's defaults: a recovered clock on the spectrum's
    /// signal. Project scoped like the log-axis panes: a rate stated about
    /// one project's result is not a statement about the next project's, and
    /// `clear_project_scoped_state` resets it with the rest of the document.
    pub eye_settings: std::collections::HashMap<
        crate::eye_diagram::EyeTimebaseKey,
        crate::eye_diagram::EyeSettings,
    >,
    /// Identity map behind the ordinal compatibility projection.
    pub expr_projection_keys: std::collections::HashMap<usize, AnalysisPresentationKey>,
    /// The inline expression editor, when open (one strip at a time).
    pub expr_editor: Option<ExprEditor>,
    /// Pinned data point per XY viewer (trace slot, point index) — the
    /// Smith/Nyquist/PZ click-to-pin readout.
    pub rf_pin: std::collections::HashMap<ResultViewer, (usize, usize)>,
    /// The X a sheet's readout is pinned at, per authored analysis: a click
    /// on a spectrum or phase-noise plot pins its readout there. The run's
    /// working view keeps it, so the next run reads at the same X.
    pub readout_pins: HashMap<(AnalysisPresentationSource, ResultViewer), f64>,
    /// What the reader left a settings-keeping sheet showing, per authored
    /// analysis: the Smith chart's markers, the network matrix's form and
    /// format. The run's working view keeps them, as it keeps the pins.
    pub sheet_settings: HashMap<
        (AnalysisPresentationSource, ResultViewer),
        rspice_results::visualization_document::SheetSettingValue,
    >,
    /// Display AC phase traces unwrapped into a continuous curve instead of
    /// wrapped to ±180°. Applies to the BODE phase trace and the waves
    /// strips' phase traces; the margin math always reads the raw wrapped
    /// arrays. Transient — not persisted with the session.
    pub phase_continuous: bool,
    /// Decibels or linear magnitude, decade or linear frequency: the
    /// reader's preference, which the host keeps this in step with.
    pub ac_display: crate::waves::AcDisplay,
    /// The digits and phase of the project result document being shown.
    /// They stand in for the reader's own while it is, and go when the
    /// workspace leaves it.
    pub document_presentation:
        Option<rspice_results::visualization_document::VisualizationPresentationPolicy>,
    /// OP sheet name filter (docbar input). Transient.
    pub op_filter: String,
    /// OP sheet sort, per section. Transient.
    pub op_sort: crate::op_inspector::OpSort,
    /// OP sheet sections the reader has folded, by title. Transient.
    pub op_collapsed: std::collections::BTreeSet<String>,
    /// OP sheet row the reader picked. Transient.
    pub op_selected: Option<crate::op_inspector::OpSelection>,
    /// The other run's operating point the OP sheet compares with. Transient.
    pub op_compare: Option<crate::op_inspector::OpCompare>,
    /// The axis-limit field being typed, if any.
    ///
    /// Editing is transactional like the marker dialog: the pinned interval
    /// only moves on commit, so a half-typed bound never rescales the plot
    /// under the reader's hands. Transient.
    pub axis_limit_draft: Option<(ResultViewer, PaneAxis, String)>,
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
    pub const RETAINED_PANE_ORDINAL: usize = 0;
    /// Clear cursors (Esc, Clear action).
    pub fn clear_cursors(&mut self) {
        self.cursors.clear();
        self.cursor_strip = None;
        self.cursor_a_anchor = None;
        self.stepping_cursor = CursorId::A;
    }

    /// Arm the A│B tool, as a request to move a cursor implies.
    pub fn arm_cursor_tool(&mut self) {
        self.cursor_tool = CursorTool(true);
    }

    /// Hand the arrow keys and the jump commands to the other cursor.
    pub fn switch_stepping_cursor(&mut self) {
        self.stepping_cursor = self.stepping_cursor.other();
    }

    /// The horizontal cursors placed on `pane`.
    pub fn pane_horizontal_cursors(&self, pane: &WavePanePresentationKey) -> HorizontalCursors {
        self.horizontal_cursors
            .get(pane)
            .copied()
            .unwrap_or_default()
    }

    /// Put horizontal cursor `slot` (0 is H1, 1 is H2) of `pane` at `y`.
    pub fn set_horizontal_cursor(&mut self, pane: WavePanePresentationKey, slot: usize, y: f64) {
        if let Some(cursor) = self
            .horizontal_cursors
            .entry(pane)
            .or_default()
            .get_mut(slot)
        {
            *cursor = y.is_finite().then_some(y);
        }
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
        self.cursors.next_placed() == Some(CursorId::A)
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

    /// Replace the overlay with the markers of the view being drawn.
    pub fn project_document_markers(&mut self, markers: Vec<DocumentMarker>) {
        self.document_markers = markers;
        if let Some(draft) = &self.marker_edit
            && let MarkerSelector { marker_id, .. } = draft.selector
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

    /// Markers on one strip, in placement order: those of the view that keeps
    /// the strip. The overlay is verified against the pane that keeps the
    /// strip, so a stale projection cannot leak one view's markers onto
    /// another's strip.
    pub fn strip_markers(
        &self,
        analysis: AnalysisPresentationKey,
    ) -> impl Iterator<Item = MarkerView<'_>> {
        let pane = self.pane_for(analysis);
        self.document_markers
            .iter()
            .filter(move |marker| {
                marker.analysis == analysis
                    && pane.is_some_and(|context| {
                        context.document_id == marker.document_id
                            && context.pane_id == marker.pane_id
                    })
            })
            .map(MarkerView)
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

    /// Show a document pane's expressions, with the units declared for
    /// them, on its analysis' strip.
    pub fn project_document_expressions(
        &mut self,
        analysis: AnalysisPresentationKey,
        analysis_index: usize,
        traces: Vec<ExprTrace>,
        units: Vec<(String, String)>,
    ) {
        let held = self
            .analysis_exprs
            .get(&analysis)
            .map_or(&[][..], Vec::as_slice);
        if held != traces.as_slice() {
            if traces.is_empty() {
                self.analysis_exprs.remove(&analysis);
            } else {
                self.analysis_exprs.insert(analysis, traces);
            }
        }
        self.expression_units
            .retain(|(owner, _), _| *owner != analysis);
        for (text, unit) in units {
            self.expression_units.insert((analysis, text), unit);
        }
        self.sync_expression_projection(analysis, analysis_index);
    }

    /// Draw a document pane's analysis strip on the X scale the pane keeps.
    pub fn project_document_x_scale(
        &mut self,
        analysis: AnalysisPresentationKey,
        scale: rspice_ui_kit::plot::XScale,
    ) {
        if self.x_scales.get(&analysis) != Some(&scale) {
            self.x_scales.insert(analysis, scale);
        }
    }

    /// Show the strips a document pane keeps drawn against a signal or an
    /// expression. A strip the pane keeps keeps the zoom the reader gave it.
    pub fn project_document_xy_strips(
        &mut self,
        analysis: AnalysisPresentationKey,
        plots: Vec<DocumentXyPlot>,
    ) {
        let held: Vec<_> = self
            .xy_strips
            .iter()
            .filter(|strip| strip.analysis == analysis)
            .map(|strip| DocumentXyPlot {
                unit: strip.unit.clone(),
                abscissa: strip.abscissa.clone(),
                sweep_range: strip.sweep_range,
                follow_view: strip.follow_view,
                log_x: strip.log_x,
            })
            .collect();
        if held == plots {
            return;
        }
        let previous: Vec<XyStrip> = self
            .xy_strips
            .iter()
            .filter(|strip| strip.analysis == analysis)
            .cloned()
            .collect();
        self.xy_strips.retain(|strip| strip.analysis != analysis);
        for plot in plots {
            let mut strip = previous
                .iter()
                .find(|strip| strip.is(analysis, &plot.unit, &plot.abscissa))
                .cloned()
                .unwrap_or_else(|| XyStrip::new(analysis, plot.unit, plot.abscissa));
            strip.sweep_range = plot.sweep_range;
            strip.follow_view = plot.follow_view;
            if strip.log_x != plot.log_x {
                strip.log_x = plot.log_x;
                strip.x_view = None;
            }
            self.xy_strips.push(strip);
        }
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

    /// Clear design results within the same project.
    pub fn clear_design_scoped_state(&mut self) {
        self.clear_project_scoped_state();
    }

    /// Reset result UI state and identities when replacing the whole project.
    pub fn clear_project_scoped_state(&mut self) {
        let viewer = self.viewer;
        let phase_continuous = self.phase_continuous;
        let ac_display = self.ac_display;
        *self = Self {
            viewer,
            phase_continuous,
            ac_display,
            ..Self::default()
        };
    }

    pub fn persistent_document_page(
        &self,
        document_id: rspice_vocabulary::product::ResultDocumentId,
    ) -> Option<rspice_results::visualization_document::PageId> {
        self.persistent_document_pages.get(&document_id).copied()
    }

    pub fn select_persistent_document_page(
        &mut self,
        document_id: rspice_vocabulary::product::ResultDocumentId,
        page_id: rspice_results::visualization_document::PageId,
    ) {
        self.persistent_document_pages.insert(document_id, page_id);
    }

    pub fn persistent_document_pane(
        &self,
        document_id: rspice_vocabulary::product::ResultDocumentId,
    ) -> Option<PaneId> {
        self.persistent_document_panes.get(&document_id).copied()
    }

    pub fn select_persistent_document_pane(
        &mut self,
        document_id: rspice_vocabulary::product::ResultDocumentId,
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
        self.working_view_panes.clear();
        self.persistent_pane_context = Some(PersistentPaneContext {
            document_id,
            pane_id,
            analysis,
        });
    }

    /// The pane that keeps what the reader does on `analysis`' strip: the
    /// persistent pane being drawn, or the working view's pane for it.
    pub fn pane_for(&self, analysis: AnalysisPresentationKey) -> Option<PersistentPaneContext> {
        self.persistent_pane_context
            .filter(|context| context.analysis == analysis)
            .or_else(|| {
                self.working_view_panes
                    .iter()
                    .copied()
                    .find(|context| context.analysis == analysis)
            })
    }

    /// Leave the persistent projection: the pane context and the retained
    /// marker overlay it scopes both belong to the document that was open.
    /// With none open, the overlay is the working view's, and stays.
    pub fn leave_persistent_document(&mut self) {
        if self.persistent_pane_context.is_none() && self.document_presentation.is_none() {
            return;
        }
        self.persistent_pane_context = None;
        self.document_markers.clear();
        self.document_presentation = None;
    }

    /// Whether phase traces are drawn unwrapped: as the document being shown
    /// keeps it, or as the reader set it for quick views.
    pub fn presented_phase_continuous(&self) -> bool {
        self.document_presentation
            .map_or(self.phase_continuous, |document| document.phase_continuous)
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

    pub fn clear_checked_signals(&mut self) {
        self.checked_result_quantities.clear();
        self.browser_range_anchor = None;
    }

    pub fn set_browser_range_anchor(&mut self, key: ResultBrowserSelectionKey) {
        self.browser_range_anchor = Some(key);
    }

    /// The rows from the range anchor to `target`, in list order. An anchor
    /// the list no longer holds starts the range over at `target`.
    pub fn browser_range(
        &mut self,
        target: &ResultBrowserSelectionKey,
        ordered_visible: &[ResultBrowserSelectionKey],
    ) -> Vec<ResultBrowserSelectionKey> {
        let anchor = self.browser_range_anchor.as_ref().unwrap_or(target);
        let Some(anchor_index) = ordered_visible.iter().position(|key| key == anchor) else {
            self.browser_range_anchor = Some(target.clone());
            return vec![target.clone()];
        };
        let Some(target_index) = ordered_visible.iter().position(|key| key == target) else {
            return Vec::new();
        };
        let (start, end) = if anchor_index <= target_index {
            (anchor_index, target_index)
        } else {
            (target_index, anchor_index)
        };
        ordered_visible[start..=end].to_vec()
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

    /// Drop one axis' override on every pane of every strip of the sheet, as
    /// [`Self::clear_analysis_plot_view_axis`] drops it on one strip.
    pub fn clear_every_analysis_plot_view_axis(&mut self, viewer: ResultViewer, axis: PaneAxis) {
        if let Some(context) = self.persistent_pane_context {
            self.clear_analysis_plot_view_axis(viewer, context.analysis, axis);
            return;
        }
        self.views.retain(|(key_viewer, key_plot, _), view| {
            if *key_viewer != viewer || !matches!(key_plot, PlotPresentationKey::Analysis(_)) {
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
    /// document. `eye_settings` keeps its prepared keys for the same reason,
    /// and only its dataset-named legacy keys are pruned.
    ///
    pub fn retain_datasets(&mut self, retained: &HashSet<DatasetId>) {
        let live = |analysis: AnalysisPresentationKey| retained.contains(&analysis.dataset_id());
        self.log_y_panes.retain(|pane| live(pane.analysis));
        self.analysis_exprs.retain(|analysis, _| live(*analysis));
        self.expression_units
            .retain(|(analysis, _), _| live(*analysis));
        self.x_scales.retain(|analysis, _| live(*analysis));
        self.xy_strips.retain(|strip| live(strip.analysis));
        self.strip_layouts.retain(|analysis, _| live(*analysis));
        self.expr_projection_keys
            .retain(|_, analysis| live(*analysis));
        self.hidden_strips.retain(|analysis| live(*analysis));
        self.maximized_strip = self.maximized_strip.filter(|analysis| live(*analysis));
        self.recent_signals.retain(|key| live(key.analysis()));
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
        self.waveform_colors.retain(|key, _| live(key.analysis()));
        self.waveform_looks.retain(|key, _| live(key.analysis()));
        self.hidden_family_traces.retain(|key| live(key.analysis()));
        self.eye_settings.retain(|key, _| match key {
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
