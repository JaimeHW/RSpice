//! Waveform stack layout and local viewer interaction over host-qualified sources.
use super::{
    CursorDomain, ReadoutPolicy, SHARED_X_SLOT, StripModel, StripTrace, UnitPane, anchor_key,
    cursor_interpolation,
    cursor_values::marker_position,
    dock::{marker_color, marker_label},
    extent::{self, FamilyEnvelopePlan},
    header::{self as pane_header, WAVE_PANE_HEADER_HEIGHT, elide},
    navigation::{self, WAVE_SHARED_X_HEIGHT},
    pane::{self as pane_view, ResolvedExpr, pane_auto_y},
    trace_key,
};
use crate::{
    derived::{DerivedSeries, Extremes},
    presentation::well_hint,
    session::{ExprEditor, ResultViewerState, XyStrip},
    strip::{LegendChip, StripHeader},
};
use egui::Ui;
use rspice_results::result_presentation::{
    AnalysisPresentationKey, ExprTrace, ResultViewer, WavePanePresentationKey,
};
use rspice_results::waveform::WaveformValues;
use rspice_ui_kit::{
    plot::{self, CursorId, CursorPair, XScale, sample::sample_at_with_shape},
    tokens::Tokens,
};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;
use std::sync::Arc;

/// Display preferences captured for one stack frame.
#[derive(Clone, Copy)]
pub struct StackOptions {
    pub pane_chrome: bool,
    pub readout: ReadoutPolicy,
    pub quantity: QuantityPresentationPolicy,
    /// Stroke of the active run's traces, in points.
    pub trace_width: f32,
}

/// Source qualification, expression evaluation and persistent transactions stay
/// with the host. Calls occur in painting order, so later panes observe edits
/// made by earlier panes without copying or replacing the viewer session.
pub trait StackHost {
    fn session(&self) -> &ResultViewerState;
    fn session_mut(&mut self) -> &mut ResultViewerState;
    fn expression_is_complex(&self, analysis: AnalysisPresentationKey, text: &str) -> bool;
    fn edit_expression(&mut self, ui: &mut Ui, model: &StripModel);
    fn resolve_expressions(&mut self, model: &StripModel, tokens: &Tokens) -> Vec<ResolvedExpr>;
    fn forget_expression(&mut self, analysis: AnalysisPresentationKey, text: String);
    /// Add an expression to the strip, as a conversion from an expression's
    /// own menu asks.
    fn add_expression(&mut self, _model: &StripModel, _text: String) {}
    fn specification_limits(
        &self,
        model: &StripModel,
        pane: &UnitPane,
        tokens: &Tokens,
    ) -> Vec<plot::LimitLine>;
    /// What the comparison that last answered marks on `pane`, whose
    /// expressions are `expressions`. A host that compares nothing marks
    /// nothing.
    fn comparison_marks(
        &mut self,
        _model: &StripModel,
        _pane: &UnitPane,
        _expressions: &[ResolvedExpr],
        _tokens: &Tokens,
    ) -> pane_view::ComparisonMarks {
        pane_view::ComparisonMarks::default()
    }
    fn pane_header(
        &mut self,
        ui: &mut Ui,
        model: &StripModel,
        pane: &UnitPane,
        input: pane_header::PaneHeader,
    ) -> pane_header::UnitPaneHeaderResponse;
    fn family_envelopes(&mut self, model: &StripModel, pane: &UnitPane) -> Arc<FamilyEnvelopePlan>;
    fn place_marker(&mut self, model: &StripModel, trace: &StripTrace, x: f64);
    fn note_sample_reads(&mut self, overview: usize, extrema: usize);
    /// Say why an arrangement of the strips the reader asked for cannot be
    /// made: a fifth unit on a strip, say.
    fn refuse(&mut self, _reason: String) {}
    /// Whether the host takes a note added at a point of a plot, which the
    /// plot's menu then offers.
    fn offers_notes(&self) -> bool {
        false
    }
    /// Add a note at (`x`, `y`) of `pane`'s plot, `y` read on its first axis.
    fn add_note(&mut self, _model: &StripModel, _pane: &UnitPane, _x: f64, _y: f64) {}
    /// Hand `trace` to the waveform calculator, as the plot's menu asks.
    fn send_to_calculator(&mut self, _model: &StripModel, _trace: &StripTrace) {}
    /// Evaluate `text` as the abscissa a pane is drawn against, once per
    /// member of the strip. A host without a calculator has none to offer.
    fn resolve_abscissa_expression(
        &mut self,
        _model: &StripModel,
        text: &str,
    ) -> Result<super::xy::ResolvedAbscissa, String> {
        Err(format!("{text} cannot be evaluated here"))
    }
    /// Height of the logic and bus lanes this strip shows above its panes;
    /// zero when it shows none.
    fn event_lane_height(&self, _model: &StripModel) -> f32 {
        0.0
    }
    /// Draw the strip's logic and bus lanes ([`super::logic::show_lanes`])
    /// on the panes' own time axis.
    fn show_event_lanes(&mut self, _ui: &mut Ui, _model: &StripModel, _frame: EventLaneFrame) {}
    /// Height of the event-marker strip above this strip's lanes and panes;
    /// zero when the run has no marker, so the strip takes no room.
    fn event_marker_height(&self, _model: &StripModel) -> f32 {
        0.0
    }
    /// Draw the strip's HDL records and assertion violations
    /// ([`super::logic::show_marker_strip`]) on the panes' time axis.
    fn show_event_markers(&mut self, _ui: &mut Ui, _model: &StripModel, _frame: EventLaneFrame) {}
    /// The real event lanes (`E()`) placed in `pane`, as step traces
    /// ([`super::logic::real_step_trace`]) that share its unit axis.
    ///
    /// `view` is the strip's visible time window, which a stored run's lanes are read over.
    fn real_step_traces(
        &mut self,
        _model: &StripModel,
        _pane: &UnitPane,
        _tokens: &Tokens,
        _view: (f64, f64),
    ) -> Vec<ResolvedExpr> {
        Vec::new()
    }
}

/// The time axis an event-lane host draws its lanes on, aligned with the
/// strip's panes to the pixel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EventLaneFrame {
    pub x_range: (f64, f64),
    pub left_margin: f32,
    /// The gutter's name column; its value column is the rest of the margin.
    pub name_width: f32,
    pub right_margin: f32,
}

/// The shares of the strip the name column keeps between until the reader
/// drags it: it fits the longest name, but never takes more than a quarter
/// of the strip from the time axis, nor less than a twelfth.
const LANE_NAME_SHARE: (f32, f32) = (1.0 / 12.0, 0.25);
const LANE_NAME_MIN: f32 = 28.0;
const LANE_NAME_MAX: f32 = 420.0;
/// The value column's floor: the panes' Y tick labels sit in it too.
const LANE_VALUE_MIN: f32 = 44.0;

/// A name column width the reader dragged to, kept to what the strip can
/// give it.
pub fn clamp_name_width(width: f32) -> f32 {
    width.clamp(LANE_NAME_MIN, LANE_NAME_MAX)
}

/// A strip's left gutter, as EZwave and Verdi lay it out: a name column the
/// reader sizes, then a value column wide enough for any value the lanes can
/// read out, then the time axis. The panes' plot area starts where the lanes'
/// time axis does, so a strip with lanes is aligned to the pixel. A strip
/// without lanes keeps the plain Y-axis margin and no name column.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gutter {
    pub name: f32,
    /// What an axis standing outside the first axis' labels takes beyond
    /// the first axis' own column: as wide on every strip of the analysis
    /// as on the strip that needs most, so every plot starts on one rule.
    pub axes: f32,
    pub value: f32,
    /// What the axes standing right of the plot take, on the same terms.
    pub right: f32,
    /// How many of a strip's added axes have room beside its plot, in the
    /// order they take it, and how many have been measured: those between
    /// wait in the strip's trace list for room.
    pub kept: usize,
    pub measured: usize,
}

impl Gutter {
    /// A gutter of `name` and `value` columns with no added axis.
    pub fn plain(name: f32, value: f32) -> Self {
        Self {
            name,
            axes: 0.0,
            value,
            right: 0.0,
            kept: 0,
            measured: 0,
        }
    }

    pub fn width(self) -> f32 {
        self.name + self.axes + self.value
    }

    /// What every plot of the strip keeps right of its area: the axes
    /// standing there, or the breathing strip when none does.
    pub fn right_margin(self) -> f32 {
        navigation::WAVE_SHARED_RIGHT_MARGIN.max(self.right)
    }
}

fn strip_gutter(ui: &Ui, host: &impl StackHost, model: &StripModel) -> Gutter {
    let (name, value) = gutter_columns(ui, host, model);
    let fit = super::axes::fit_axes(
        ui.available_width(),
        (name, value),
        super::axes::remembered_room(ui.ctx(), model.analysis_key),
    );
    Gutter {
        name,
        axes: fit.left,
        value,
        right: fit.right,
        kept: fit.kept,
        measured: fit.measured,
    }
}

/// The name and value columns of a strip's left gutter.
fn gutter_columns(ui: &Ui, host: &impl StackHost, model: &StripModel) -> (f32, f32) {
    let axis = wave_left_margin(host.session());
    if host.event_lane_height(model) <= 0.0 {
        return (0.0, axis);
    }
    let specs =
        super::lanes::lane_specs(model, host.session().event_lanes.get(&model.analysis_key));
    let advance = ui
        .painter()
        .layout_no_wrap(
            "0".to_owned(),
            rspice_ui_kit::theme::mono(
                rspice_ui_kit::tokens::FS_MICRO,
                rspice_ui_kit::theme::FontWeight::Regular,
            ),
            egui::Color32::WHITE,
        )
        .size()
        .x;
    let chars = super::lanes::value_column_chars(&specs).min(16);
    let fit = || {
        let strip = ui.available_width();
        super::logic::name_column_fit(ui, &specs).clamp(
            strip * LANE_NAME_SHARE.0,
            (strip * LANE_NAME_SHARE.1).max(strip * LANE_NAME_SHARE.0),
        )
    };
    (
        clamp_name_width(host.session().lane_name_width.unwrap_or_else(fit)),
        (chars as f32).mul_add(advance, 8.0).max(LANE_VALUE_MIN),
    )
}

// Mockup gutter geometry: a 64 px left gutter carries the Y ticks and the
// X-strip's band labels; the right edge keeps only a 14 px breathing strip
// now that no pane owns a secondary axis.
const WAVE_SHARED_LEFT_MARGIN: f32 = 64.0;

/// The mockup's per-sheet left gutter: the noise sheet's nV/√Hz tick
/// labels need 88 px where the shared 64 px gutter suffices elsewhere.
fn wave_left_margin(results: &ResultViewerState) -> f32 {
    match results.viewer {
        ResultViewer::NoiseContrib => 88.0,
        _ => WAVE_SHARED_LEFT_MARGIN,
    }
}
const WAVE_MIN_PLOT_HEIGHT: f32 = 24.0;

/// Between two strips: a gap of canvas, then a hairline in border ink on top
/// of the next strip's header. A strip's overview track then plainly ends its
/// own strip, and the next header plainly starts a new one.
const STRIP_GAP: f32 = 8.0;
const STRIP_DIVIDER: f32 = 1.0;
/// What an analysis' block takes beside its strips: its header and the
/// shared X axis under them.
const STRIP_BLOCK_CHROME: f32 = 76.0;
/// The least height a strip of an analysis' block is given before the
/// stack scrolls rather than squeeze it further.
const STRIP_MIN_HEIGHT: f32 = 64.0;
/// The least height of an analysis' block, or of a strip drawn against a
/// signal.
const BLOCK_MIN_HEIGHT: f32 = 140.0;

/// The extremes a pane's traces draw: in the X `window`, or over the whole
/// record when there is none. Each trace's block extremes are built once per
/// data version, and a window reads them in time logarithmic in its length,
/// so a pan or a zoom never rescans the samples.
///
/// The fit is per pane because each pane carries its own unit: fitting
/// volts and amps to one range would flatten whichever is smaller.
pub fn pane_trace_extremes(
    derived: &mut DerivedSeries,
    model: &StripModel,
    indices: &[usize],
    window: Option<(f64, f64)>,
    mut note_samples: impl FnMut(usize),
) -> Extremes {
    let mut extremes = Extremes::default();
    for trace in indices.iter().filter_map(|index| model.traces.get(*index)) {
        // A stored trace's extent comes from its tiles, never its whole column.
        if trace.y.is_stored()
            && let Some(extent) = derived.stored_extent_or(trace_key(model, trace), window, || {
                trace.y.extent_over(&trace.x, window)
            })
        {
            extremes.absorb(Extremes {
                min: extent.min,
                max: extent.max,
                min_positive: extent.min_positive,
            });
            continue;
        }
        // Held in memory, or held once its read has arrived.
        let Some((x, y)) = trace.held() else {
            continue;
        };
        let index = derived.extrema_or(trace_key(model, trace), &y, &mut note_samples);
        extremes.absorb(match window {
            Some(window) => index.window(&x, &y, &trace.shape, window),
            None => index.whole(),
        });
    }
    extremes
}

/// The extremes an expression draws in the X `window`, or over its whole
/// series when there is none.
fn expression_extremes(
    derived: &mut DerivedSeries,
    expression: &ResolvedExpr,
    window: Option<(f64, f64)>,
) -> Extremes {
    let Some((x, y)) = super::held_pair(&expression.x, &expression.y) else {
        return Extremes::default();
    };
    let index = derived.extrema_or(expression.cache_key, &y, |_| {});
    match window {
        Some(window) => index.window(&x, &y, &expression.shape, window),
        None => index.whole(),
    }
}

/// The whole-series extremes of the expressions a pane draws.
fn expressions_range(exprs: &[ResolvedExpr]) -> Option<(f64, f64)> {
    exprs
        .iter()
        .filter_map(|expression| expression.y_extremes)
        .reduce(|(lo, hi), (a, b)| (lo.min(a), hi.max(b)))
}

/// A linear fit's span, padded 8 % so no curve runs along the frame.
fn padded((min, max): (f64, f64)) -> (f64, f64) {
    let pad = (max - min) * 0.08;
    // A span that cannot survive its own padding is flat as far as any axis
    // can draw it: exactly constant, or constant to within a few ulps. Both
    // widen the same way, because a range finer than the tick ladder can
    // subdivide leaves the pane with no labels at all.
    if pad <= 0.0 {
        return (min - 1.0, max + 1.0);
    }
    (min - pad, max + pad)
}

/// Y range of one pane's traces over the whole record, padded 8 %.
pub fn pane_y_range(
    derived: &mut DerivedSeries,
    model: &StripModel,
    indices: &[usize],
    note_samples: impl FnMut(usize),
) -> Option<(f64, f64)> {
    pane_trace_extremes(derived, model, indices, None, note_samples)
        .range()
        .map(padded)
}

/// What one pane's automatic Y fit is taken over.
pub struct PaneFit<'a> {
    /// The pane's traces, as indices into the strip model's.
    pub traces: &'a [usize],
    /// The expressions the pane draws.
    pub expressions: &'a [ResolvedExpr],
    /// The specification limits the pane draws, which must stay on it.
    pub limits: &'a [plot::LimitLine],
    /// The X window the pane shows.
    pub window: (f64, f64),
    /// Whether the axis is logarithmic, which fits the positive values.
    pub log: bool,
}

/// The Y interval a pane shows when the reader has not pinned one: what its
/// curves draw in the X window it shows, as ViVA and EZwave refit a strip on
/// an X zoom, padded by the same rule a whole-record fit is.
///
/// A window that holds none of the pane's values, past the end of its data,
/// fits the whole record instead, so panning there leaves the pane its axis.
pub fn pane_fit(
    derived: &mut DerivedSeries,
    model: &StripModel,
    fit: &PaneFit<'_>,
    mut note_samples: impl FnMut(usize),
) -> Option<(f64, f64)> {
    let mut measure = |window: Option<(f64, f64)>| {
        let traces = pane_trace_extremes(derived, model, fit.traces, window, &mut note_samples);
        let mut expressions = Extremes::default();
        for expression in fit.expressions {
            expressions.absorb(expression_extremes(derived, expression, window));
        }
        (traces, expressions)
    };
    let (mut traces, mut expressions) = measure(Some(fit.window));
    let mut all = traces;
    all.absorb(expressions);
    if all.range().is_none() {
        (traces, expressions) = measure(None);
        all = traces;
        all.absorb(expressions);
    }
    if fit.log {
        pane_view::pane_auto_log_y(all.positive_range(), fit.limits)
    } else {
        pane_auto_y(traces.range().map(padded), expressions.range(), fit.limits)
    }
}

pub fn model_is_visible(
    model: &StripModel,
    models: &[StripModel],
    results: &ResultViewerState,
) -> bool {
    match results.maximized_strip {
        Some(maximized) if models.iter().any(|item| item.analysis_key == maximized) => {
            model.analysis_key == maximized
        }
        _ => !results.hidden_strips.contains(&model.analysis_key),
    }
}

pub fn active_pane<'a>(
    models: &'a [StripModel],
    results: &ResultViewerState,
) -> Option<(&'a StripModel, usize, UnitPane<'a>)> {
    let active = results.active_wave_pane.as_ref()?;
    let model = models
        .iter()
        .find(|model| model.analysis_key == active.analysis)
        .filter(|model| model_is_visible(model, models, results))?;
    model
        .unit_panes()
        .into_iter()
        .enumerate()
        .find(|(_, pane)| pane.unit == active.unit)
        .map(|(ordinal, pane)| (model, ordinal, pane))
}

fn pane_log_y_key(model: &StripModel, pane: &UnitPane) -> WavePanePresentationKey {
    WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: pane.quantity_unit().to_owned(),
    }
}

pub fn pane_log_y(results: &ResultViewerState, model: &StripModel, pane: &UnitPane) -> bool {
    results.log_y_panes.contains(&pane_log_y_key(model, pane))
}

pub fn set_pane_log_y(
    results: &mut ResultViewerState,
    model: &StripModel,
    pane: &UnitPane,
    enabled: bool,
) {
    let key = pane_log_y_key(model, pane);
    if enabled {
        results.log_y_panes.insert(key);
    } else {
        results.log_y_panes.remove(&key);
    }
}

/// The X window the reader put the analysis' strips in: one for all of
/// them, kept on [`SHARED_X_SLOT`].
pub fn shared_x_view(
    results: &ResultViewerState,
    analysis: AnalysisPresentationKey,
) -> Option<(f64, f64)> {
    results
        .analysis_plot_view_pane(ResultViewer::Waves, analysis, SHARED_X_SLOT)
        .x
}

/// Set the strip's X window, as the reader zoomed, panned or fitted it: which ends following
/// its run's newest rows.
pub fn set_shared_x_view(
    results: &mut ResultViewerState,
    analysis: AnalysisPresentationKey,
    range: Option<(f64, f64)>,
) {
    results.live_extent.follow.remove(&analysis);
    results
        .analysis_plot_view_pane_mut(ResultViewer::Waves, analysis, SHARED_X_SLOT)
        .x = range;
}

/// How a strip's height is shared out.
///
/// Its panes split what the shared X axis leaves equally, as ViVA's and
/// EZwave's strips do: no pane's quantity is secondary to another's, and a
/// pane weighted down was left too short to label its own axis.
#[derive(Debug, Clone, Copy, PartialEq)]
struct WaveStackGeometry {
    pane_height: f32,
    shared_x_height: f32,
    seam_height: f32,
}

fn wave_stack_geometry(available_height: f32, pane_count: usize) -> WaveStackGeometry {
    if pane_count == 0 || !available_height.is_finite() {
        return WaveStackGeometry {
            pane_height: 0.0,
            shared_x_height: 0.0,
            seam_height: 0.0,
        };
    }
    let available = available_height.max(0.0);
    let seam_count = pane_count.saturating_sub(1) as f32;
    let seam_height = if seam_count > 0.0 {
        (available / seam_count).min(1.0)
    } else {
        0.0
    };
    let content = (available - seam_height * seam_count).max(0.0);
    // The normal 50 px navigator is retained when space permits and shrinks
    // proportionally in constrained multi-pane/multi-strip arrangements.
    // The sum is exact: this function never asks the parent to grow.
    let shared_x_height = (content * RULER_SHARE).min(WAVE_SHARED_X_HEIGHT);
    let pane_height = ((content - shared_x_height) / pane_count as f32).max(0.0);
    WaveStackGeometry {
        pane_height,
        shared_x_height,
        seam_height,
    }
}

/// Palette color for the i-th trace slot of a strip (waveforms, then
/// expressions).
pub fn expr_color(tokens: &Tokens, slot: usize) -> egui::Color32 {
    tokens.color.traces[slot % tokens.color.traces.len()]
}

/// The palette slot the strip's `slot`-th expression draws in.
///
/// Waveform traces take the leading slots and expressions the ones after —
/// and "the ones after" has to be counted the same way wherever the colour is
/// asked for. The legend counted only the active run's traces while the
/// canvas counted every trace it held, overlays included, so the moment a
/// strip carried a second run the chip beside an expression was a different
/// colour from the curve it named.
pub fn expr_palette_slot(model: &StripModel, slot: usize) -> usize {
    model.traces.len() + slot
}

/// What a strip says when every trace it holds is hidden.
pub const NO_VISIBLE_TRACE: &str = "Nothing plotted — show a trace in the legend";

/// What a strip says when its analysis holds no samples to plot.
const NO_SAMPLES: &str = "Nothing plotted — the analysis has no samples";

/// A complex expression is drawn as its magnitude, and says so.
pub fn expression_label(expr: &ExprTrace, complex: bool) -> String {
    if complex || expr.complex_policy.is_legacy() {
        format!("mag({})", elide(&expr.text, 24))
    } else {
        elide(&expr.text, 24)
    }
}

fn strip_area_id() -> egui::Id {
    egui::Id::new("rspice.results.strip-area")
}

/// Where the strip last lay on the screen, for a prompt that must sit over
/// it without covering a trace the reader is about to click.
pub(crate) fn last_strip_area(ctx: &egui::Context) -> Option<egui::Rect> {
    ctx.data(|data| data.get_temp(strip_area_id()))
}

pub fn show(ui: &mut Ui, host: &mut impl StackHost, models: &[StripModel], options: StackOptions) {
    let t = Tokens::get(ui.ctx());
    let pane_chrome = options.pane_chrome;
    // Apply hide/maximize strip state.
    let results = host.session();
    let visible: Vec<&StripModel> = match results.maximized_strip {
        Some(max_key) if models.iter().any(|m| m.analysis_key == max_key) => models
            .iter()
            .filter(|m| m.analysis_key == max_key)
            .collect(),
        _ => models
            .iter()
            .filter(|m| !results.hidden_strips.contains(&m.analysis_key))
            .collect(),
    };
    if visible.is_empty() {
        well_hint(
            ui,
            "Nothing plotted — every strip is closed; restore them with Hidden above",
        );
        return;
    }

    // Deferred state mutations (collected while iterating immutably).
    let mut toggle_maximize: Option<AnalysisPresentationKey> = None;
    let mut close_strip: Option<AnalysisPresentationKey> = None;
    let mut fit_strip: Option<AnalysisPresentationKey> = None;
    let mut toggle_expr: Option<(AnalysisPresentationKey, usize)> = None;
    let mut remove_expr: Option<(AnalysisPresentationKey, usize)> = None;
    let mut open_editor: Option<AnalysisPresentationKey> = None;
    let avail = ui.available_rect_before_wrap();
    ui.ctx()
        .data_mut(|data| data.insert_temp(strip_area_id(), avail));
    let n = visible.len();
    // Each analysis' strip is followed by the strips that plot its panes
    // against a signal, in the order the reader made them, and every strip
    // takes an equal share of the height.
    let rows: Vec<(&StripModel, Option<usize>)> = visible
        .iter()
        .flat_map(|model| {
            std::iter::once((*model, None)).chain(
                host.session()
                    .xy_strips
                    .iter()
                    .enumerate()
                    .filter(|(_, strip)| strip.analysis == model.analysis_key)
                    .map(|(index, _)| (*model, Some(index)))
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    let separators = rows.len().saturating_sub(1) as f32 * (STRIP_GAP + STRIP_DIVIDER);
    let share = (avail.height() - separators) / rows.len() as f32;
    // An analysis' block grows with its strips, each kept readable, and the
    // stack scrolls once the blocks outgrow the window.
    let row_heights: Vec<f32> = rows
        .iter()
        .map(|(model, xy)| {
            let needed = if xy.is_some() {
                BLOCK_MIN_HEIGHT
            } else {
                let strips = model.unit_panes().len() as f32;
                BLOCK_MIN_HEIGHT.max(strips.mul_add(STRIP_MIN_HEIGHT, STRIP_BLOCK_CHROME))
            };
            share.max(needed)
        })
        .collect();
    let mut close_xy: Option<usize> = None;
    let maximized = host.session().maximized_strip.is_some();
    let linked_cursor_domain = host
        .session()
        .cursor_strip
        .and_then(|owner| models.iter().find(|model| model.analysis_index == owner))
        .map(|model| model.cursor_domain());

    egui::ScrollArea::vertical()
        .id_salt("rspice.results.strips")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            for (position, &(model, xy)) in rows.iter().enumerate() {
                if position > 0 {
                    let (seam, _) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width(), STRIP_GAP + STRIP_DIVIDER),
                        egui::Sense::hover(),
                    );
                    let (gap, divider) = seam.split_top_bottom_at_y(seam.top() + STRIP_GAP);
                    ui.painter().rect_filled(gap, 0.0, t.color.canvas_bg);
                    ui.painter().rect_filled(divider, 0.0, t.color.border);
                }
                let strip_height = row_heights[position];
                if let Some(index) = xy {
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), strip_height),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_min_height(strip_height);
                            let mut edits = ExpressionEdits::default();
                            // An XY strip keeps a place for the maximize
                            // control the strips drawn against time carry
                            // beside their close one, so every scale chip
                            // ends on one rule.
                            let slots = 1 + usize::from(pane_chrome && !maximized && n > 1);
                            if show_xy_strip(
                                ui,
                                host,
                                model,
                                (index, slots),
                                linked_cursor_domain.as_ref(),
                                options,
                                &mut edits,
                            ) {
                                close_xy = Some(index);
                            }
                            if let Some(slot) = edits.toggle {
                                toggle_expr = Some((model.analysis_key, slot));
                            }
                            if let Some(slot) = edits.remove {
                                remove_expr = Some((model.analysis_key, slot));
                            }
                        },
                    );
                    continue;
                }
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), strip_height),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.set_min_height(strip_height);
                        // Analysis identity remains in the strip header.
                        // Unit-owned signal legends live in the pane headers;
                        // only unitless expressions remain alongside the
                        // analysis identity here.
                        let strip_exprs: Vec<ExprTrace> = host
                            .session()
                            .analysis_exprs
                            .get(&model.analysis_key)
                            .cloned()
                            .unwrap_or_default();
                        let expr_labels: Vec<String> = strip_exprs
                            .iter()
                            .map(|expr| {
                                let complex =
                                    host.expression_is_complex(model.analysis_key, &expr.text);
                                expression_label(expr, complex)
                            })
                            .collect();
                        let legend: Vec<LegendChip<'_>> = strip_exprs
                            .iter()
                            .enumerate()
                            .map(|(i, expr)| LegendChip {
                                name: &expr_labels[i],
                                color: expr_color(&t, expr_palette_slot(model, i)),
                                on: expr.visible,
                            })
                            .collect();

                        let zoomed = host
                            .session()
                            .analysis_strip_is_zoomed(ResultViewer::Waves, model.analysis_key);
                        let mut conversions = |ui: &mut Ui, index: usize| {
                            if let Some(expr) = strip_exprs.get(index) {
                                expression_conversions(ui, host, model, index, expr, &t);
                            }
                        };
                        // The window controls the header keeps: maximize,
                        // and close beside it when several strips are shown.
                        // Each pane's row ends under them.
                        let closable = pane_chrome && !maximized && n > 1;
                        let window_slots = 1 + usize::from(closable);
                        let header = StripHeader::new(&model.kind_tag, &model.subtitle, &legend)
                            .id_salt(model.analysis_key)
                            .incomplete(model.incomplete)
                            .maximized(maximized)
                            .closable(closable)
                            .zoomed(zoomed)
                            .expr_action(pane_chrome)
                            .removable_from(0)
                            .chip_menu(&mut conversions)
                            .pane_actions(pane_chrome)
                            .x_scale(true)
                            .show(ui);
                        // The strip header owns the shared X axis, and with
                        // it the axis' scale toggle.
                        if let Some(rect) = header.scale_rect
                            && crate::strip::x_scale_chip(
                                ui,
                                egui::Id::new(("rspice.results.x-scale", model.analysis_key)),
                                rect,
                                (
                                    model.x_scale == XScale::Log10,
                                    log_x_refusal(model).as_deref(),
                                ),
                            )
                        {
                            let scale = match model.x_scale {
                                XScale::Log10 => XScale::Linear,
                                XScale::Linear => XScale::Log10,
                            };
                            let session = host.session_mut();
                            session.x_scales.insert(model.analysis_key, scale);
                            // A range read on one scale is no viewport on the
                            // other.
                            set_shared_x_view(session, model.analysis_key, None);
                        }
                        if let Some(chip_index) = header.legend_clicked {
                            toggle_expr = Some((model.analysis_key, chip_index));
                        }
                        if let Some(chip_index) = header.legend_visibility_clicked {
                            toggle_expr = Some((model.analysis_key, chip_index));
                        }
                        if let Some(chip_index) = header.legend_removed {
                            remove_expr = Some((model.analysis_key, chip_index));
                        }
                        if header.maximize_clicked {
                            toggle_maximize = Some(model.analysis_key);
                        }
                        if header.close_clicked {
                            close_strip = Some(model.analysis_key);
                        }
                        if header.fit_clicked {
                            fit_strip = Some(model.analysis_key);
                        }
                        if header.add_expr_clicked {
                            open_editor = Some(model.analysis_key);
                        }

                        host.edit_expression(ui, model);

                        // Strips scrolled out of view skip the plot body
                        // entirely (range lookups, envelope mapping, shape
                        // building) — only the space is reserved.
                        let plot_rect = ui.available_rect_before_wrap();
                        if ui.is_rect_visible(plot_rect) {
                            let mut edits = ExpressionEdits::default();
                            show_strip_plot(
                                ui,
                                host,
                                (model, window_slots),
                                linked_cursor_domain.as_ref(),
                                options,
                                &mut edits,
                            );
                            if let Some(slot) = edits.toggle {
                                toggle_expr = Some((model.analysis_key, slot));
                            }
                            if let Some(slot) = edits.remove {
                                remove_expr = Some((model.analysis_key, slot));
                            }
                        } else {
                            ui.allocate_exact_size(plot_rect.size(), egui::Sense::hover());
                        }
                    },
                );
            }
        });

    // Apply deferred mutations.
    let results = host.session_mut();
    if let Some(index) = close_xy
        && index < results.xy_strips.len()
    {
        results.xy_strips.remove(index);
    }
    if let Some(idx) = toggle_maximize {
        results.maximized_strip = (results.maximized_strip != Some(idx)).then_some(idx);
    }
    if let Some(idx) = close_strip {
        results.hidden_strips.insert(idx);
        if models
            .iter()
            .find(|model| model.analysis_key == idx)
            .is_some_and(|model| results.cursor_strip == Some(model.analysis_index))
        {
            results.clear_cursors();
        }
    }
    if let Some(key) = fit_strip {
        results.reset_analysis_plot_view(ResultViewer::Waves, key);
    }
    if let Some((analysis, index)) = toggle_expr
        && let Some(expr) = results
            .analysis_exprs
            .get_mut(&analysis)
            .and_then(|list| list.get_mut(index))
    {
        expr.visible = !expr.visible;
        if let Some(model) = models.iter().find(|model| model.analysis_key == analysis) {
            results.sync_expression_projection(analysis, model.analysis_index);
        }
    }
    if let Some((analysis, index)) = remove_expr
        && let Some(list) = host.session_mut().analysis_exprs.get_mut(&analysis)
    {
        let removed = (index < list.len()).then(|| list.remove(index));
        if let Some(removed) = removed {
            host.session_mut()
                .expression_units
                .remove(&(analysis, removed.text.clone()));
            host.forget_expression(analysis, removed.text);
        }
        let results = host.session_mut();
        if results
            .analysis_exprs
            .get(&analysis)
            .is_some_and(Vec::is_empty)
        {
            results.analysis_exprs.remove(&analysis);
        }
        if let Some(model) = models.iter().find(|model| model.analysis_key == analysis) {
            results.sync_expression_projection(analysis, model.analysis_index);
        }
    }
    if let Some(analysis) = open_editor {
        host.session_mut().expr_editor = Some(ExprEditor {
            analysis,
            text: String::new(),
            error: None,
            want_focus: true,
        });
        host.session_mut().expr_editor_unit.clear();
    }
}

/// The conversions an expression's chip offers, as a trace's or a lane's
/// menu offers them: a value read as logic with `digital()`, a logic lane
/// drawn as a staircase with `analog()`.
fn expression_conversions(
    ui: &mut Ui,
    host: &mut impl StackHost,
    model: &StripModel,
    slot: usize,
    expr: &ExprTrace,
    t: &Tokens,
) {
    if super::logic::convert::draws_as_logic(&expr.text) {
        if ui
            .button("Analog")
            .on_hover_text("Draw the lane's value as a staircase trace, analog()")
            .clicked()
        {
            host.add_expression(model, format!("analog({})", expr.text));
            ui.close();
        }
        return;
    }
    rspice_ui_kit::widgets::menu_button(ui, "Digital", |ui| {
        // The threshold starts at the middle of what the expression draws.
        let values = host
            .resolve_expressions(model, t)
            .into_iter()
            .find(|resolved| resolved.slot == slot)
            .map(|resolved| resolved.y)
            .unwrap_or_default();
        if let Some(text) = super::logic::convert::digital_form_of(ui, &expr.text, || {
            values
                .fetch_whole()
                .ready()
                .and_then(|values| super::logic::convert::mid_range(&values))
        }) {
            host.add_expression(model, text);
        }
    });
}

#[derive(Clone, Copy)]
pub(crate) struct StripFrame<'a> {
    pub(crate) model: &'a StripModel,
    pub(crate) x_domain: (f64, f64),
    pub(crate) linked_cursor_domain: Option<&'a CursorDomain>,
    pub(crate) options: StackOptions,
    pub(crate) gutter: Gutter,
    /// How many window controls the strip header keeps, so each pane's row
    /// ends under them on the same rule.
    pub(crate) window_slots: usize,
}

/// The share of a strip its ruler takes, up to the ruler's own height.
pub(crate) const RULER_SHARE: f32 = 0.28;

/// Why a strip's X axis cannot be logarithmic, when it cannot: some X it
/// shows is negative, or none is positive. A domain that starts at zero, as
/// time does, is shown from its first positive sample.
///
///
/// It reads the bounds the model keeps ([`StripModel::x_bounds`]), never a
/// column: the chip asks on every frame.
pub fn log_x_refusal(model: &StripModel) -> Option<String> {
    let name = if model.x_label.is_empty() {
        "the X axis"
    } else {
        model.x_label.as_str()
    };
    let Some((minimum, maximum)) = model.x_bounds else {
        return Some("the strip shows no X values".to_owned());
    };
    if minimum < 0.0 {
        return Some(format!(
            "{name} reaches {}, and a logarithmic axis shows positive values only",
            rspice_ui_kit::plot::fmt_si_significant(minimum, &model.x_unit, 4)
        ));
    }
    (maximum <= 0.0).then(|| format!("{name} has no positive value"))
}

/// The least and greatest finite X the strip's visible traces hold; `None`
/// when they hold none. A stored run's time axis ascends, so its ends bound
/// it and only they are read; other axes are read once here, as the model is
/// built.
pub(crate) fn visible_x_bounds(model: &StripModel) -> Option<(f64, f64)> {
    let (mut minimum, mut maximum) = (f64::INFINITY, f64::NEG_INFINITY);
    let mut take = |x: f64| {
        if x.is_finite() {
            minimum = minimum.min(x);
            maximum = maximum.max(x);
        }
    };
    let mut seen: Vec<&WaveformValues> = Vec::new();
    for trace in model.traces.iter().filter(|trace| trace.visible) {
        let x = &trace.x;
        // A run's traces share its axis: read it once.
        if seen.iter().any(|axis| WaveformValues::ptr_eq(axis, x)) {
            continue;
        }
        seen.push(x);
        if x.is_ascending_axis()
            && let Some((first, last)) = x.ends()
        {
            take(first);
            take(last);
            continue;
        }
        if let Some(x) = x.fetch_whole().ready() {
            x.iter().copied().for_each(&mut take);
        }
    }
    maximum.is_finite().then_some((minimum, maximum))
}

/// Follow a running transient's newest rows: its window keeps the width it has and ends at the
/// newest time. A strip of anything else has nothing to follow.
pub fn start_following(results: &mut ResultViewerState, model: &StripModel) {
    if model.live_frame.is_some() {
        results.live_extent.follow.insert(model.analysis_key);
        follow_leading_edge(results, model);
    }
}

/// Keep a following strip's window on its run's newest rows. Once the run ended its strip has
/// no live frame, and following ends with the window where it was.
pub fn follow_leading_edge(results: &mut ResultViewerState, model: &StripModel) {
    if !results.live_extent.follow.contains(&model.analysis_key) {
        return;
    }
    let (Some(frame), Some((_, leading))) = (model.live_frame, model.x_bounds) else {
        results.live_extent.follow.remove(&model.analysis_key);
        return;
    };
    let view = shared_x_view(results, model.analysis_key).unwrap_or(frame);
    let followed = super::extent::LiveExtent::followed(view, leading);
    if followed != view {
        results
            .analysis_plot_view_pane_mut(ResultViewer::Waves, model.analysis_key, SHARED_X_SLOT)
            .x = Some(followed);
    }
}

/// Resolve source ownership, then apply the navigator's actions to the app.
fn show_shared_x_axis(ui: &mut Ui, host: &mut impl StackHost, height: f32, frame: StripFrame<'_>) {
    let StripFrame {
        model,
        x_domain: full_domain,
        linked_cursor_domain,
        options,
        gutter,
        ..
    } = frame;
    if height <= 1.0 {
        return;
    }
    follow_leading_edge(host.session_mut(), model);
    let current = shared_x_view(host.session(), model.analysis_key).unwrap_or(full_domain);
    let cursor_owner = host.session().cursor_strip == Some(model.analysis_index);
    let linked_cursor =
        host.session().linked_cursors && linked_cursor_domain == Some(&model.cursor_domain());
    let input = navigation::SharedXAxis {
        full_domain,
        current,
        height,
        left_margin: gutter.width(),
        right_margin: gutter.right_margin(),
        quantity_policy: options.quantity,
        significant_digits: usize::from(options.readout.displayed_significant_digits().get()),
        cursors: (cursor_owner || linked_cursor).then_some(host.session().cursors),
    };
    let output = navigation::show_shared_x_axis(ui, &mut host.session_mut().derived, model, input);
    host.note_sample_reads(output.overview_samples_read, output.extrema_samples_read);
    if let Some(viewport) = output.viewport {
        match viewport {
            navigation::SharedXViewChange::Fit => {
                set_shared_x_view(host.session_mut(), model.analysis_key, None);
            }
            navigation::SharedXViewChange::Range(range) => {
                set_shared_x_view(host.session_mut(), model.analysis_key, Some(range));
            }
            navigation::SharedXViewChange::Follow => {
                start_following(host.session_mut(), model);
            }
        }
    }
    if let Some(cursor) = output.cursor {
        if !cursor_owner && !linked_cursor {
            host.session_mut().clear_cursors();
            host.session_mut().cursor_strip = Some(model.analysis_index);
        }
        let session = host.session_mut();
        match cursor {
            navigation::CursorMove::A(x) => {
                session.cursors.a = Some(x);
                session.cursor_a_anchor = None;
                session.stepping_cursor = CursorId::A;
            }
            navigation::CursorMove::B(x) => {
                session.cursors.b = Some(x);
                session.stepping_cursor = CursorId::B;
            }
        }
    }
}

/// One strip, drawn as one pane per unit.
///
/// Signals route to the pane that owns their unit; the panes stack and
/// share the strip's X domain, so a strip stays one measurement read
/// against as many scales as it genuinely needs.
/// What a pane's legend asked of the strip's expressions this frame,
/// applied with the strip header's own once every strip is drawn, so no
/// pane drawn after it reads a list it has changed.
#[derive(Default)]
struct ExpressionEdits {
    toggle: Option<usize>,
    remove: Option<usize>,
}

fn show_strip_plot(
    ui: &mut Ui,
    host: &mut impl StackHost,
    (model, window_slots): (&StripModel, usize),
    linked_cursor_domain: Option<&CursorDomain>,
    options: StackOptions,
    edits: &mut ExpressionEdits,
) {
    let t = Tokens::get(ui.ctx());
    // A strip with signals and nothing of them plotted has no pane, and so
    // no legend to enable one in: they wait in the Data browser.
    const NOTHING_PLOTTED: &str = "Nothing plotted — tick a signal in the Data browser";
    let Some(x_domain) = model.x_range else {
        let retained = !model.traces.is_empty() || !model.lane_traces.is_empty();
        well_hint(
            ui,
            if retained {
                NOTHING_PLOTTED
            } else {
                NO_SAMPLES
            },
        );
        return;
    };
    let panes = model.unit_panes();
    let wanted_lanes = host.event_lane_height(model).max(0.0);
    if panes.is_empty() && wanted_lanes == 0.0 {
        well_hint(ui, NOTHING_PLOTTED);
        return;
    }

    // A real event lane is placed in a pane, and joins that pane's traces on
    // its unit axis.
    let mut pane_exprs = route_expressions(host, model, &panes, &t);
    let view = shared_x_view(host.session(), model.analysis_key).unwrap_or(x_domain);
    for (pane, exprs) in panes.iter().zip(&mut pane_exprs) {
        exprs.extend(host.real_step_traces(model, pane, &t, view));
    }
    // A pane with nothing to show keeps its header row, where its hidden
    // traces wait to be shown again and its plotted ones are named, and gives
    // its height to the others: nothing is plotted on it, or nothing plotted
    // on it holds a finite value to draw.
    let mut extrema_samples = 0;
    let collapsed: Vec<bool> = panes
        .iter()
        .zip(&pane_exprs)
        .map(|(pane, exprs)| {
            let range = pane_y_range(
                &mut host.session_mut().derived,
                model,
                &pane.traces,
                |count| extrema_samples += count,
            );
            pane_auto_y(range, expressions_range(exprs), &[]).is_none()
        })
        .collect();
    host.note_sample_reads(0, extrema_samples);
    let collapsed_count = collapsed.iter().filter(|collapsed| **collapsed).count();
    let available = ui.available_rect_before_wrap();
    let count = panes.len();
    let gutter = strip_gutter(ui, host, model);
    let marker_height = event_marker_band(ui, host, model, (x_domain, gutter));
    let lane_height = marker_height
        + event_lane_band(ui, host, model, (x_domain, gutter), count, wanted_lanes, &t);
    let expanded_count = count - collapsed_count;
    let geometry = wave_stack_geometry(
        available.height() - lane_height - collapsed_count as f32 * (WAVE_PANE_HEADER_HEIGHT + 1.0),
        expanded_count,
    );
    let frame = StripFrame {
        model,
        x_domain,
        linked_cursor_domain,
        options,
        gutter,
        window_slots,
    };
    // The strips share what is left by their weights, as the reader sized
    // them; a divider being dragged moves the line between two of them
    // until it is let go.
    let expanded_total = geometry.pane_height * expanded_count as f32;
    let weight_sum: f32 = panes
        .iter()
        .zip(&collapsed)
        .filter(|(_, collapsed)| !**collapsed)
        .map(|(pane, _)| pane.weight)
        .sum();
    let mut heights: Vec<f32> = panes
        .iter()
        .zip(&collapsed)
        .map(|(pane, collapsed)| {
            if *collapsed || weight_sum <= 0.0 {
                0.0
            } else {
                expanded_total * pane.weight / weight_sum
            }
        })
        .collect();
    for index in 0..count.saturating_sub(1) {
        let offset = super::strip_drag::divider_offset(ui.ctx(), model, index);
        if offset != 0.0 && !collapsed[index] && !collapsed[index + 1] {
            heights[index] += offset;
            heights[index + 1] -= offset;
        }
    }
    let mut rects = Vec::with_capacity(count);
    for (ordinal, pane) in panes.iter().enumerate() {
        if ordinal > 0 {
            if collapsed[ordinal - 1] || collapsed[ordinal] {
                let (seam, _) = ui.allocate_exact_size(
                    egui::vec2(
                        ui.available_width(),
                        1.0_f32.min(ui.available_height().max(0.0)),
                    ),
                    egui::Sense::hover(),
                );
                ui.painter().rect_filled(seam, 0.0, t.color.canvas_grid);
            } else {
                super::strip_drag::divider(
                    ui,
                    host,
                    model,
                    ordinal - 1,
                    (heights[ordinal - 1], heights[ordinal]),
                    panes[ordinal - 1].locked || pane.locked,
                );
            }
        }
        let pane_height = if collapsed[ordinal] {
            WAVE_PANE_HEADER_HEIGHT.min(ui.available_height())
        } else {
            heights[ordinal]
        };
        let pane_exprs = std::mem::take(&mut pane_exprs[ordinal]);
        let drawn = ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), pane_height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_height(pane_height);
                show_unit_pane(ui, host, pane, &pane_exprs, frame, edits);
            },
        );
        rects.push(drawn.response.rect);
    }
    super::strip_drag::drops(ui, host, model, &rects);
    let shared_x_height = if expanded_count == 0 {
        WAVE_SHARED_X_HEIGHT.min((available.height() - lane_height).max(0.0))
    } else {
        geometry.shared_x_height
    };
    show_shared_x_axis(ui, host, shared_x_height, frame);
}

/// The legend entries of the strip's expressions on `pane`: one for each
/// expression it draws, standing for all the curves of a family, then one
/// for each hidden expression it would draw.
fn pane_expressions(
    host: &impl StackHost,
    model: &StripModel,
    pane: &UnitPane,
    exprs: &[ResolvedExpr],
    cursor_a: Option<(f64, ReadoutPolicy, QuantityPresentationPolicy)>,
    t: &Tokens,
) -> Vec<pane_header::PaneExpression> {
    let Some(list) = host.session().analysis_exprs.get(&model.analysis_key) else {
        return Vec::new();
    };
    let label = |expr: &ExprTrace| {
        expression_label(
            expr,
            host.expression_is_complex(model.analysis_key, &expr.text),
        )
    };
    let swatch = |slot: usize, family_style| pane_view::SwatchStyle {
        palette_slot: Some(expression_hue_slot(model, slot)),
        family_style,
        ordinal: 0,
        style: Default::default(),
    };
    let mut slots: Vec<usize> = Vec::new();
    for expr in exprs {
        // A real lane's step trace has no place in the expression list.
        if expr.slot != usize::MAX && !slots.contains(&expr.slot) {
            slots.push(expr.slot);
        }
    }
    let drawn = slots.into_iter().filter_map(|slot| {
        let expr = list.get(slot)?;
        let members: Vec<&ResolvedExpr> = exprs.iter().filter(|e| e.slot == slot).collect();
        let first = members.first()?;
        let at_cursor = cursor_a.map(|(x, presentation, _)| {
            let value = super::held_pair(&first.x, &first.y).map_or(f64::NAN, |(xs, ys)| {
                sample_at_with_shape(
                    &xs,
                    &ys,
                    &first.shape,
                    x,
                    cursor_interpolation(presentation.cursor_interpolation()),
                )
            });
            rspice_ui_kit::plot::fmt_si_significant(
                value,
                pane.axis_unit(),
                usize::from(presentation.displayed_significant_digits().get()),
            )
        });
        // A signal read from another run says which run and how.
        let label = match &first.reading {
            Some(reading) => format!("{} · {reading}", label(expr)),
            None => label(expr),
        };
        Some(pane_header::PaneExpression {
            slot,
            text: expr.text.clone(),
            label,
            color: first.color,
            swatch: swatch(slot, first.family_style),
            visible: true,
            at_cursor,
            curves: members.len(),
            values: first.y.clone(),
        })
    });
    let hidden = pane.hidden_expressions.iter().filter_map(|&slot| {
        let expr = list.get(slot)?;
        Some(pane_header::PaneExpression {
            slot,
            text: expr.text.clone(),
            label: label(expr),
            color: expr_color(t, expr_palette_slot(model, slot)),
            swatch: swatch(slot, None),
            visible: false,
            at_cursor: None,
            curves: 0,
            values: Default::default(),
        })
    });
    drawn.chain(hidden).collect()
}

/// The hue an expression's curve takes among the palette's, which also
/// picks its dash and marker when traces are drawn colour-safe, as a
/// signal's hue picks its own.
pub(crate) fn expression_hue_slot(model: &StripModel, slot: usize) -> usize {
    expr_palette_slot(model, slot) % rspice_ui_kit::palette::TRACE_HUES
}

/// The strip's expressions, each in the pane its unit routed it to, where it
/// joins that pane's automatic fit; one whose unit cannot be stated has a
/// pane of its own. A host that routes no expression keeps them in the first
/// pane.
fn route_expressions(
    host: &mut impl StackHost,
    model: &StripModel,
    panes: &[UnitPane],
    t: &Tokens,
) -> Vec<Vec<ResolvedExpr>> {
    let routed = !model.expression_panes.is_empty();
    let mut pane_exprs: Vec<Vec<ResolvedExpr>> = panes.iter().map(|_| Vec::new()).collect();
    for expr in host.resolve_expressions(model, t) {
        let ordinal = panes
            .iter()
            .position(|pane| pane.all_expressions().any(|slot| slot == expr.slot))
            .or((!routed).then_some(0));
        if let Some(exprs) = ordinal.and_then(|ordinal| pane_exprs.get_mut(ordinal)) {
            exprs.push(expr);
        }
    }
    pane_exprs
}

/// One strip plotting a unit pane's curves against a signal or an
/// expression: a header naming the abscissa, with the span of the sweep it
/// draws, its X scale chip and a close button; the pane's own header, whose legend and scale
/// it shares; and the plot, whose one horizontal axis is the abscissa's
/// ruler, without the overview track the sweep's has. True when the reader
/// closed it.
fn show_xy_strip(
    ui: &mut Ui,
    host: &mut impl StackHost,
    model: &StripModel,
    (index, slots): (usize, usize),
    linked_cursor_domain: Option<&CursorDomain>,
    options: StackOptions,
    edits: &mut ExpressionEdits,
) -> bool {
    let Some(strip) = host.session().xy_strips.get(index).cloned() else {
        return false;
    };
    let t = Tokens::get(ui.ctx());
    let identity = super::stable_hash(&(strip.analysis, &strip.unit, &strip.abscissa));
    let subtitle = format!("vs {}", super::xy::abscissa_label(&strip.abscissa));
    let field_width = super::xy::sweep_range_field_width(ui, model);
    let header = StripHeader::new(&model.kind_tag, &subtitle, &[])
        .id_salt(("xy-strip", identity))
        .incomplete(model.incomplete)
        .maximizable(false)
        .closable(options.pane_chrome)
        .zoomed(strip.x_view.is_some() || strip.y_view.is_some())
        .field(field_width)
        .pane_actions(options.pane_chrome)
        .x_scale(true)
        .window_slots(slots)
        .show(ui);
    let scale_rect = header.scale_rect;
    if ui
        .ctx()
        .data_mut(|data| data.get_temp::<u64>(xy_reveal_id()))
        == Some(identity)
    {
        ui.ctx().data_mut(|data| data.remove::<u64>(xy_reveal_id()));
        ui.scroll_to_rect(ui.max_rect(), None);
    }
    if header.close_clicked {
        return true;
    }
    let panes = model.unit_panes();
    let span = model
        .x_range
        .map(|domain| super::xy::drawn_span(host.session(), &strip, domain));
    if let (Some(rect), Some(span)) = (header.field_rect, span)
        && let Some(edit) = super::xy::sweep_range_field(
            ui,
            rect,
            model,
            (&strip, span),
            (
                identity,
                usize::from(options.readout.displayed_significant_digits().get()),
                options.quantity,
            ),
        )
        && let Some(held) = host.session_mut().xy_strips.get_mut(index)
    {
        match edit {
            super::xy::SpanEdit::Typed(range) => {
                held.sweep_range = range;
                held.follow_view = false;
            }
            super::xy::SpanEdit::Follow(true) => held.follow_view = true,
            // Let go, the strip keeps the span it was following.
            super::xy::SpanEdit::Follow(false) => {
                held.follow_view = false;
                held.sweep_range = (Some(span) != model.x_range).then_some(span);
            }
        }
        // A range read on one span is no view of another.
        if edit != super::xy::SpanEdit::Follow(false) {
            held.x_view = None;
            held.y_view = None;
        }
    }
    if header.fit_clicked
        && let Some(held) = host.session_mut().xy_strips.get_mut(index)
    {
        held.x_view = None;
        held.y_view = None;
    }
    let Some(x_domain) = model.x_range else {
        well_hint(ui, NO_SAMPLES);
        return false;
    };
    let Some(ordinal) = panes.iter().position(|pane| pane.axis == strip.unit) else {
        well_hint(
            ui,
            &format!("This analysis has no {} curves to plot", strip.unit),
        );
        return false;
    };
    let mut pane_exprs = route_expressions(host, model, &panes, &t);
    let exprs = std::mem::take(&mut pane_exprs[ordinal]);
    let pane = &panes[ordinal];
    let gutter = strip_gutter(ui, host, model);
    let frame = StripFrame {
        model,
        x_domain,
        linked_cursor_domain,
        options,
        gutter,
        window_slots: slots,
    };

    // The pane's header: its legend, its unit and its scale, which the strip
    // shares with the pane it plots.
    let pane_range = pane_y_range(&mut host.session_mut().derived, model, &pane.traces, |_| {});
    let auto_y = pane_auto_y(pane_range, expressions_range(&exprs), &[]);
    let log_y_available = auto_y.is_some_and(|(minimum, maximum)| minimum > 0.0 && maximum > 0.0);
    let mut log_y = pane_log_y(host.session(), model, pane) && log_y_available;
    let cursor_a = (host.session().cursor_readout_active()
        && host.session().cursor_strip == Some(model.analysis_index))
    .then(|| {
        host.session()
            .cursors
            .a
            .map(|x| (x, options.readout, options.quantity))
    })
    .flatten();
    let input = pane_header::PaneHeader {
        height: WAVE_PANE_HEADER_HEIGHT.min(ui.available_height()),
        active: false,
        log_y,
        log_y_available,
        cursor_a,
        expressions: pane_expressions(host, model, pane, &exprs, cursor_a, &t),
        plotted: auto_y.is_some(),
        abscissa: options
            .pane_chrome
            .then(|| abscissa_menu(host.session(), model, pane, Some(strip.abscissa.clone()))),
        window_slots: slots,
        arrange: false,
        folded_axes: Vec::new(),
    };
    let header = ui
        .push_id(("xy-strip-pane", identity), |ui| {
            host.pane_header(ui, model, pane, input)
        })
        .inner;
    edits.toggle = edits.toggle.or(header.toggle_expression);
    edits.remove = edits.remove.or(header.remove_expression);
    if let Some(abscissa) = header.abscissa {
        repoint_xy_strip(host.session_mut(), index, abscissa);
        return false;
    }
    if (header.autoscale_y || header.toggle_log_y)
        && let Some(held) = host.session_mut().xy_strips.get_mut(index)
    {
        held.y_view = None;
    }
    if header.toggle_log_y {
        log_y = !log_y;
        set_pane_log_y(host.session_mut(), model, pane, log_y);
    }
    if ui.available_height() >= WAVE_MIN_PLOT_HEIGHT && (auto_y.is_some() || !exprs.is_empty()) {
        ui.push_id(("xy-strip-plot", identity), |ui| {
            super::xy::show(
                ui,
                host,
                super::xy::XyInput {
                    pane,
                    exprs: &exprs,
                    frame,
                    strip: index,
                    log_y,
                    scale_rect,
                },
            );
        });
    } else if ui.available_height() >= WAVE_MIN_PLOT_HEIGHT {
        well_hint(ui, NO_VISIBLE_TRACE);
    }
    false
}

/// Plot the strip at `index` against `abscissa` instead. A strip that would
/// repeat another plot of the same curves gives way to it.
pub(crate) fn repoint_xy_strip(
    session: &mut ResultViewerState,
    index: usize,
    abscissa: rspice_results::visualization_document::Abscissa,
) {
    let Some(strip) = session.xy_strips.get(index) else {
        return;
    };
    let (analysis, unit) = (strip.analysis, strip.unit.clone());
    if session
        .xy_strips
        .iter()
        .any(|held| held.is(analysis, &unit, &abscissa))
    {
        session.xy_strips.remove(index);
        return;
    }
    let strip = &mut session.xy_strips[index];
    strip.abscissa = abscissa;
    strip.x_view = None;
    // Another quantity is read on a linear axis until the reader asks.
    strip.log_x = false;
}

/// At most this share of a strip goes to event lanes when it also has panes;
/// more lanes than fit scroll within it.
const EVENT_LANE_SHARE: f32 = 0.5;

/// The strip's axes with no room beside its plot, each named by its unit
/// and its curves, for its trace list.
fn folded_axes(model: &StripModel, pane: &UnitPane, gutter: Gutter) -> Vec<String> {
    pane.secondary
        .iter()
        .take(gutter.measured)
        .skip(gutter.kept)
        .map(|axis| {
            let unit = if axis.unit.is_empty() {
                pane_view::NO_UNIT
            } else {
                axis.unit
            };
            let names: Vec<&str> = axis
                .traces
                .iter()
                .filter_map(|index| model.traces.get(*index).map(|trace| trace.name.as_str()))
                .collect();
            format!("{unit} axis: {}", names.join(", "))
        })
        .collect()
}

/// The time axis the strip's panes are drawn on, for the bands above them.
fn event_frame(
    host: &impl StackHost,
    model: &StripModel,
    (x_domain, gutter): ((f64, f64), Gutter),
) -> EventLaneFrame {
    EventLaneFrame {
        x_range: shared_x_view(host.session(), model.analysis_key).unwrap_or(x_domain),
        left_margin: gutter.width(),
        name_width: gutter.name,
        right_margin: gutter.right_margin(),
    }
}

/// Draw the strip's event-marker strip at its top, when the run has any
/// marker, and return the height it took.
fn event_marker_band(
    ui: &mut Ui,
    host: &mut impl StackHost,
    model: &StripModel,
    domain: ((f64, f64), Gutter),
) -> f32 {
    let available = ui.available_rect_before_wrap().height().max(0.0);
    let height = host.event_marker_height(model).max(0.0).min(available);
    if height <= 0.0 {
        return 0.0;
    }
    let frame = event_frame(host, model, domain);
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), height),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.set_height(height);
            host.show_event_markers(ui, model, frame);
        },
    );
    height
}

/// Draw the strip's logic and bus lanes above its panes, on their time axis,
/// and return the height they took, seam included.
fn event_lane_band(
    ui: &mut Ui,
    host: &mut impl StackHost,
    model: &StripModel,
    domain: ((f64, f64), Gutter),
    pane_count: usize,
    wanted: f32,
    t: &Tokens,
) -> f32 {
    if wanted <= 0.0 {
        return 0.0;
    }
    let available = ui.available_rect_before_wrap().height().max(0.0);
    let cap = if pane_count == 0 {
        (available - WAVE_SHARED_X_HEIGHT).max(0.0)
    } else {
        available * EVENT_LANE_SHARE
    };
    let height = wanted.min(cap);
    if height <= 0.0 {
        return 0.0;
    }
    let frame = event_frame(host, model, domain);
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), height),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.set_height(height);
            // A scroll area is at least 64 points tall unless told otherwise,
            // which would push the panes below the strip for one or two lanes.
            egui::ScrollArea::vertical()
                .id_salt(("event-lanes", model.analysis_index))
                .max_height(height)
                .min_scrolled_height(height)
                .auto_shrink([false, false])
                .show(ui, |ui| host.show_event_lanes(ui, model, frame));
        },
    );
    if pane_count == 0 {
        return height;
    }
    let (seam, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), egui::Sense::hover());
    ui.painter().rect_filled(seam, 0.0, t.color.canvas_grid);
    height + 1.0
}

fn show_unit_pane(
    ui: &mut Ui,
    host: &mut impl StackHost,
    pane: &UnitPane,
    exprs: &[ResolvedExpr],
    frame: StripFrame<'_>,
    edits: &mut ExpressionEdits,
) {
    let StripFrame {
        model,
        x_domain,
        linked_cursor_domain,
        options,
        gutter,
        window_slots,
    } = frame;
    let t = Tokens::get(ui.ctx());
    let presentation = options.readout;
    let quantity_policy = options.quantity;
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    // The pane's own top edge, kept so the active rail can span header and
    // canvas together once the pane's full height is known.
    let pane_top = ui.available_rect_before_wrap().top();
    // The expressions the strip's other axes read, each with its axis; the
    // rest, and any real lane's steps, read against its first.
    let all_exprs = exprs;
    let (secondary_exprs, exprs): (Vec<&ResolvedExpr>, Vec<&ResolvedExpr>) =
        exprs.iter().partition(|expr| {
            expr.slot != usize::MAX
                && pane
                    .secondary
                    .iter()
                    .any(|axis| axis.expressions.contains(&expr.slot))
        });
    let exprs: Vec<ResolvedExpr> = exprs.into_iter().cloned().collect();
    let exprs = exprs.as_slice();

    // Over the whole record: whether the pane draws anything at all, and
    // whether anything it draws could stand on a logarithmic axis.
    let mut extrema_samples = 0;
    let mut whole = pane_trace_extremes(
        &mut host.session_mut().derived,
        model,
        &pane.traces,
        None,
        |count| extrema_samples += count,
    );
    for expression in exprs {
        whole.absorb(expression_extremes(
            &mut host.session_mut().derived,
            expression,
            None,
        ));
    }
    let specification_limits = if host.session().show_spec_limits {
        host.specification_limits(model, pane, &t)
    } else {
        Vec::new()
    };
    let comparison = host.comparison_marks(model, pane, exprs, &t);
    // A pane with nothing to draw has no Y axis for its header to act on.
    let plotted = whole.range().is_some();
    // A logarithmic axis draws a pane's positive values; one with none has
    // nothing to draw on it.
    let log_y_available = whole.positive_range().is_some();
    let mut log_y = pane_log_y(host.session(), model, pane) && log_y_available;
    if !log_y_available && log_y {
        set_pane_log_y(host.session_mut(), model, pane, false);
    }
    let pane_key = WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: pane.unit.to_owned(),
    };
    let cursor_a = (host.session().cursor_readout_active()
        && host.session().cursor_strip == Some(model.analysis_index))
    .then(|| {
        host.session()
            .cursors
            .a
            .map(|x| (x, options.readout, options.quantity))
    })
    .flatten();
    let input = pane_header::PaneHeader {
        height: WAVE_PANE_HEADER_HEIGHT.min(ui.available_height()),
        active: host.session().active_wave_pane.as_ref() == Some(&pane_key),
        log_y,
        log_y_available,
        cursor_a,
        expressions: pane_expressions(host, model, pane, all_exprs, cursor_a, &t),
        plotted,
        abscissa: options
            .pane_chrome
            .then(|| abscissa_menu(host.session(), model, pane, None)),
        window_slots,
        arrange: options.pane_chrome,
        folded_axes: folded_axes(model, pane, gutter),
    };
    let header = host.pane_header(ui, model, pane, input);
    if let Some(strips) = header.arrange {
        super::strip_drag::arrange(host, model, Ok(strips));
    }
    edits.toggle = edits.toggle.or(header.toggle_expression);
    edits.remove = edits.remove.or(header.remove_expression);
    if let Some(abscissa) = header.abscissa {
        open_xy_strip(ui.ctx(), host.session_mut(), model, pane.axis, abscissa);
    }
    if header.autoscale_y || header.toggle_log_y {
        let view = host.session_mut().analysis_plot_view_pane_mut(
            ResultViewer::Waves,
            model.analysis_key,
            pane.slot,
        );
        view.y = None;
    }
    if header.toggle_log_y {
        log_y = !log_y;
        set_pane_log_y(host.session_mut(), model, pane, log_y);
    }

    // The automatic fit follows the X window: a zoom in time refits Y to
    // what the window shows, until the reader pins a Y range of their own.
    let window = shared_x_view(host.session(), model.analysis_key).unwrap_or(x_domain);
    let auto_y = pane_fit(
        &mut host.session_mut().derived,
        model,
        &PaneFit {
            traces: &pane.traces,
            expressions: exprs,
            limits: &specification_limits,
            window,
            log: log_y,
        },
        |count| extrema_samples += count,
    );
    host.note_sample_reads(0, extrema_samples);
    // A difference's tolerance band stays in view with the difference.
    let auto_y = match (auto_y, comparison.tolerance.as_ref().filter(|_| !log_y)) {
        (Some((low, high)), Some(band)) => {
            Some(band.extremes().map_or((low, high), |(bottom, top)| {
                (low.min(bottom), high.max(top))
            }))
        }
        (auto_y, _) => auto_y,
    };
    let Some((auto_y0, auto_y1)) = auto_y else {
        // A pane with nothing plotted is its header alone; the hint is for a
        // pane whose plotted traces hold no finite value to fit.
        if ui.available_height() >= WAVE_MIN_PLOT_HEIGHT {
            well_hint(ui, NO_VISIBLE_TRACE);
        }
        return;
    };
    if ui.available_height() < WAVE_MIN_PLOT_HEIGHT {
        return;
    }

    // User zoom/pan overrides the automatic fit per axis, per pane.
    let pane_view =
        host.session()
            .analysis_plot_view_pane(ResultViewer::Waves, model.analysis_key, pane.slot);
    let (x0, x1) = window;
    let family_envelopes = host
        .session()
        .show_family_envelope
        .then(|| host.family_envelopes(model, pane));
    let family_envelopes: &[extent::FamilyEnvelopeSeries] = family_envelopes
        .as_deref()
        .map_or(&[], FamilyEnvelopePlan::series);
    let pane_key = WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: pane.unit.to_owned(),
    };
    let model_cursor_domain = model.cursor_domain();
    let cursor_domain_matches = linked_cursor_domain == Some(&model_cursor_domain);
    let cursors = (host.session().cursor_strip == Some(model.analysis_index)
        || (host.session().linked_cursors && cursor_domain_matches))
        .then_some(host.session().cursors);
    // A click on a strip the pair does not stand on starts the pair afresh
    // there, so it places A however many cursors stood elsewhere.
    let places_a = cursors.is_none() || host.session().cursor_a_is_next();
    let markers = pane_view::plot_markers(
        model,
        pane,
        interpolation,
        host.session()
            .strip_markers(model.analysis_key)
            .map(|marker| pane_view::MarkerPresentation {
                anchor: marker.anchor(),
                x: marker_position(model, marker.anchor(), marker.x(), marker.kind())
                    .unwrap_or_else(|seed| seed),
                kind: marker.kind(),
                color: marker_color(marker.kind(), &t),
                label: marker_label(marker),
            }),
    );
    // Each axis after the first fits the curves it reads, over the same
    // window, until the reader pins a range of its own on it.
    let secondary_axis_exprs: Vec<Vec<ResolvedExpr>> = pane
        .secondary
        .iter()
        .map(|axis| {
            secondary_exprs
                .iter()
                .filter(|expr| axis.expressions.contains(&expr.slot))
                .map(|expr| (*expr).clone())
                .collect()
        })
        .collect();
    let mut secondary_samples = 0;
    let secondary: Vec<pane_view::SecondaryPlot<'_>> = pane
        .secondary
        .iter()
        .zip(&secondary_axis_exprs)
        .filter_map(|(axis, axis_exprs)| {
            let fitted = pane_fit(
                &mut host.session_mut().derived,
                model,
                &PaneFit {
                    traces: &axis.traces,
                    expressions: axis_exprs,
                    limits: &[],
                    window,
                    log: false,
                },
                |count| secondary_samples += count,
            );
            let pinned = host
                .session()
                .analysis_plot_view_pane(ResultViewer::Waves, model.analysis_key, axis.slot)
                .y;
            Some(pane_view::SecondaryPlot {
                unit: axis.unit,
                traces: &axis.traces,
                expressions: axis_exprs,
                range: pinned.or(fitted)?,
                slot: axis.slot,
            })
        })
        .collect();
    host.note_sample_reads(0, secondary_samples);
    // With more than one axis, each says whose it is: the first sets its
    // labels in its curves' colour when they share one, and otherwise
    // carries a swatch of each beside its unit.
    let primary_colours = if secondary.is_empty() {
        Vec::new()
    } else {
        super::axes::axis_colours(
            pane.traces
                .iter()
                .filter_map(|index| model.traces.get(*index).map(|trace| trace.color))
                .chain(exprs.iter().map(|expr| expr.color)),
        )
    };
    let y_ink = match primary_colours.as_slice() {
        [only] => Some(*only),
        _ => None,
    };
    let input = pane_view::PanePlot {
        x_range: (x0, x1),
        auto_y: (auto_y0, auto_y1),
        y_view: pane_view.y,
        log_y,
        left_margin: gutter.width(),
        readout: presentation,
        quantity: quantity_policy,
        minor_grid: host.session().show_minor_grid,
        trace_width: options.trace_width,
        horizontal_cursors: host.session().pane_horizontal_cursors(&pane_key),
        horizontal_cursor_interactive: host.session().horizontal_cursor_placement_enabled(),
        cursors,
        specification_limits,
        markers,
        comparison,
        expressions: exprs,
        family_envelopes,
        find_nearest: host.session().marker_tool.is_armed()
            || host.session().trace_pick.is_armed()
            || (host.session().cursor_placement_enabled() && places_a),
        secondary: &secondary,
        right_margin: gutter.right_margin(),
        y_ink,
    };
    let drawn = pane_view::show_plot(ui, &mut host.session_mut().cache, model, pane, input);
    let y_labels_width = drawn.y_labels_width;
    let response = drawn.response;
    note_plot_stop(ui.ctx(), response.response.id);
    let plot_choice = plot_menu(
        ui,
        host,
        model,
        pane,
        (&response.response, drawn.context.as_ref()),
        (options.pane_chrome, window),
    );
    // The first axis' unit row is every axis' unit row.
    let caption_y = response
        .y_caption
        .map_or(response.plot_rect.top() - 16.0, |caption| {
            caption.center().y
        });
    if primary_colours.len() > 1
        && let Some(caption) = response.y_caption
    {
        // As many swatches as the first axis' column has room for beside
        // its unit.
        let room = caption.left() - (response.plot_rect.left() - gutter.value);
        let fit = primary_colours
            .len()
            .min((room / super::axes::swatches_width(1)).floor().max(0.0) as usize);
        super::axes::paint_swatches(
            ui.painter(),
            &primary_colours[..fit],
            (caption.left(), false),
            caption.center().y,
        );
    }
    let axes_response = super::axes::secondary_axes(
        ui,
        model,
        &secondary,
        (response.plot_rect, gutter, y_labels_width, caption_y),
        &t,
    );
    if !secondary.is_empty() {
        super::axes::note_room(ui.ctx(), model.analysis_key, axes_response.room);
    }
    let secondary_views = axes_response.ranges;
    // A pane becomes the active one when the reader acts on it: a click, a
    // drag, a cursor placed. Pointing at it is not acting on it, and a pane
    // that took the instrument's actions as the pointer crossed it handed
    // them to whichever pane lay between the reader's pane and the toolbar.
    if response.response.clicked()
        || response.response.secondary_clicked()
        || response.response.drag_started()
        || response.response.dragged()
        || response.clicked_x.is_some()
        || response.horizontal_cursor.is_some()
    {
        host.session_mut().active_wave_pane = Some(pane_key.clone());
    }
    if let Some((slot, y)) = response.horizontal_cursor {
        host.session_mut().set_horizontal_cursor(pane_key, slot, y);
    }

    // A form waiting for a trace takes the click first: the reader asked to
    // pick a curve, and a cursor moving under it would be a second answer.
    if host.session().trace_pick.is_armed() {
        if response.response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
        }
        // The pane finds the nearest curve only for a click.
        if let Some(trace) = drawn.nearest_trace {
            host.session_mut().trace_pick.take(trace);
        }
    } else if let Some(clicked_x) = response.clicked_x
        && host.session().marker_tool.is_armed()
    {
        // The marker tool takes the click when armed: one click cannot both
        // annotate and move a cursor, and the armed chip says which it will do.
        let nearest = drawn.nearest_trace;
        if let Some(trace) = nearest {
            // The pane being drawn owns the marker, and placing one is the
            // first half of saying what it means, so the dialog opens on it.
            host.place_marker(model, trace, clicked_x);
        }
    } else if let Some(clicked_x) = response.clicked_x
        && host.session().cursor_placement_enabled()
    {
        let nearest_anchor =
            places_a.then(|| drawn.nearest_trace.map(|trace| anchor_key(model, trace)));
        let results = host.session_mut();
        if cursors.is_none() {
            results.cursors = CursorPair::default();
        }
        results.cursor_strip = Some(model.analysis_index);
        if places_a {
            results.cursor_a_anchor = nearest_anchor.flatten();
        }
        if let Some(placed) = results.cursors.place(clicked_x) {
            results.stepping_cursor = placed;
        }
    }
    // A cursor dragged by its line moves as one dragged by its ruler box.
    if let Some((cursor, x)) = response.cursor_drag {
        let session = host.session_mut();
        session.cursors.set(cursor, x);
        if cursor == CursorId::A {
            session.cursor_a_anchor = None;
        }
        session.stepping_cursor = cursor;
    }

    let fit = plot_choice == Some(super::plot_menu::PlotChoice::Fit);
    if let Some(choice) = plot_choice.filter(|_| !fit) {
        act_on_plot_choice(ui, host, model, pane, choice);
    }
    if response.view.reset || fit {
        // A fit reaches every axis the strip reads: its first and the
        // columns beside it.
        set_shared_x_view(host.session_mut(), model.analysis_key, None);
        for slot in std::iter::once(pane.slot).chain(secondary.iter().map(|axis| axis.slot)) {
            host.session_mut()
                .analysis_plot_view_pane_mut(ResultViewer::Waves, model.analysis_key, slot)
                .y = None;
        }
    } else if response.view.any() {
        if let Some(x) = response.view.x {
            set_shared_x_view(host.session_mut(), model.analysis_key, Some(x));
        }
        let view = host.session_mut().analysis_plot_view_pane_mut(
            ResultViewer::Waves,
            model.analysis_key,
            pane.slot,
        );
        if let Some(y) = response.view.y {
            view.y = Some(y);
        }
    }

    for (slot, range) in secondary_views {
        host.session_mut()
            .analysis_plot_view_pane_mut(ResultViewer::Waves, model.analysis_key, slot)
            .y = range;
    }

    paint_active_rail(ui, host, model, pane, pane_top, &t);
}

/// Draw the plot's menu over `plot`, kept where the right click that opened
/// it landed, and return what the reader chose from it. Opened from the
/// keyboard, it acts on the middle of `window`.
fn plot_menu(
    ui: &mut Ui,
    host: &mut impl StackHost,
    model: &StripModel,
    pane: &UnitPane,
    (plot, context): (&egui::Response, Option<&pane_view::PlotContext<'_>>),
    (chrome, window): (bool, (f64, f64)),
) -> Option<super::plot_menu::PlotChoice> {
    use super::plot_menu::{MenuOffers, MenuPlace};
    let place_id = plot.id.with("plot-menu-place");
    if let Some(context) = context {
        let place = MenuPlace {
            x: context.x,
            y: context.y,
            trace: context.trace.and_then(|trace| {
                model
                    .traces
                    .iter()
                    .position(|held| std::ptr::eq(held, trace))
            }),
            values: context.values.clone(),
        };
        ui.data_mut(|data| data.insert_temp(place_id, place));
    }
    let place = ui
        .data(|data| data.get_temp::<MenuPlace>(place_id))
        .unwrap_or(MenuPlace {
            x: 0.5 * (window.0 + window.1),
            y: f64::NAN,
            trace: None,
            values: Vec::new(),
        });
    let against = chrome.then(|| abscissa_menu(host.session(), model, pane, None));
    let offers = MenuOffers {
        notes: host.offers_notes(),
        against: against.as_ref().map(|menu| {
            (
                menu,
                pane.quantity_unit(),
                plot.id.with("plot-against-draft"),
            )
        }),
    };
    let choice = crate::context_menu::context_menu(plot, |ui| {
        super::plot_menu::show(ui, model, &place, &offers)
    })
    .and_then(|inner| inner.inner);
    // "Plot against" from the command list opens the choices over the pane
    // it was asked for.
    let against_id = plot.id.with("plot-against-menu");
    let pane_key = WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: pane.unit.to_owned(),
    };
    if host.session().plot_against_request.as_ref() == Some(&pane_key) {
        host.session_mut().plot_against_request = None;
        egui::Popup::open_id(ui.ctx(), against_id);
    }
    let against_choice = offers.against.and_then(|(menu, unit, draft_id)| {
        egui::Popup::new(against_id, ui.ctx().clone(), plot, ui.layer_id())
            .kind(egui::PopupKind::Menu)
            .layout(egui::Layout::top_down_justified(egui::Align::Min))
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .open_memory(None)
            .show(|ui| {
                pane_header::abscissa_choices(
                    ui,
                    unit,
                    menu,
                    &|name: &str| name.to_owned(),
                    draft_id,
                )
            })
            .and_then(|inner| inner.inner)
    });
    if against_choice.is_some() {
        egui::Popup::close_id(ui.ctx(), against_id);
    }
    let choice = choice.or(against_choice.map(super::plot_menu::PlotChoice::PlotAgainst));
    choice.map(|choice| match choice {
        super::plot_menu::PlotChoice::CopyValues => {
            ui.ctx()
                .copy_text(super::plot_menu::copied_values(&place.values));
            super::plot_menu::PlotChoice::CopyValues
        }
        super::plot_menu::PlotChoice::Note => {
            host.add_note(model, pane, place.x, place.y);
            super::plot_menu::PlotChoice::Note
        }
        super::plot_menu::PlotChoice::Cursor(cursor) => {
            place_cursor(host, model, cursor, place.x, place.trace);
            super::plot_menu::PlotChoice::Cursor(cursor)
        }
        super::plot_menu::PlotChoice::Marker(index) => {
            if let Some(trace) = model.traces.get(index) {
                host.place_marker(model, trace, place.x);
            }
            super::plot_menu::PlotChoice::Marker(index)
        }
        other => other,
    })
}

/// Do what the plot's menu was asked beyond what [`plot_menu`] does itself.
fn act_on_plot_choice(
    ui: &Ui,
    host: &mut impl StackHost,
    model: &StripModel,
    pane: &UnitPane,
    choice: super::plot_menu::PlotChoice,
) {
    match choice {
        super::plot_menu::PlotChoice::Calculator(index) => {
            if let Some(trace) = model.traces.get(index) {
                host.send_to_calculator(model, trace);
            }
        }
        super::plot_menu::PlotChoice::PlotAgainst(abscissa) => {
            open_xy_strip(ui.ctx(), host.session_mut(), model, pane.axis, abscissa);
        }
        _ => {}
    }
}

/// Put `cursor` at `x` on `model`'s strip, as a click there places one: a
/// pair standing on another strip starts afresh here, and A rides the
/// curve nearest the click.
fn place_cursor(
    host: &mut impl StackHost,
    model: &StripModel,
    cursor: CursorId,
    x: f64,
    nearest: Option<usize>,
) {
    let anchor = nearest
        .and_then(|index| model.traces.get(index))
        .map(|trace| anchor_key(model, trace));
    let session = host.session_mut();
    if session.cursor_strip != Some(model.analysis_index) {
        session.cursors = CursorPair::default();
    }
    session.cursor_strip = Some(model.analysis_index);
    session.cursors.set(cursor, x);
    if cursor == CursorId::A {
        session.cursor_a_anchor = anchor;
    }
    session.stepping_cursor = cursor;
}

/// Plot the `unit` pane's curves of `model` against `abscissa` in a strip of
/// their own after `model`'s, as ViVA's YvsY makes a new graph; the strip
/// drawn against the sweep stays as it was. A plot the stack already shows
/// is brought into view rather than made twice.
pub(crate) fn open_xy_strip(
    ctx: &egui::Context,
    session: &mut ResultViewerState,
    model: &StripModel,
    unit: &str,
    abscissa: rspice_results::visualization_document::Abscissa,
) {
    let analysis = model.analysis_key;
    if !session
        .xy_strips
        .iter()
        .any(|strip| strip.is(analysis, unit, &abscissa))
    {
        session
            .xy_strips
            .push(XyStrip::new(analysis, unit, abscissa.clone()));
    }
    ctx.data_mut(|data| {
        data.insert_temp(
            xy_reveal_id(),
            super::stable_hash(&(analysis, unit, &abscissa)),
        );
    });
    ctx.request_repaint();
}

/// Where the stack notes the strip it was asked to bring into view.
fn xy_reveal_id() -> egui::Id {
    egui::Id::new("rspice.results.xy-strip.reveal")
}

/// What a pane's curves may be plotted against: any of the strip's signals
/// (the pane's own first) or its expressions.
pub(crate) fn abscissa_menu(
    session: &ResultViewerState,
    model: &StripModel,
    pane: &UnitPane,
    current: Option<rspice_results::visualization_document::Abscissa>,
) -> pane_header::AbscissaMenu {
    let mut own = Vec::new();
    let mut others = Vec::new();
    for trace in model.traces.iter().take(model.signal_trace_count) {
        if !matches!(
            trace.kind,
            super::TraceKind::Value | super::TraceKind::Real | super::TraceKind::Imaginary
        ) || own.contains(&trace.source_waveform_name)
            || others.contains(&trace.source_waveform_name)
        {
            continue;
        }
        if model.trace_unit(trace) == pane.quantity_unit() {
            own.push(trace.source_waveform_name.clone());
        } else {
            others.push(trace.source_waveform_name.clone());
        }
    }
    own.extend(others);
    pane_header::AbscissaMenu {
        current,
        signals: own,
        expressions: session
            .analysis_exprs
            .get(&model.analysis_key)
            .into_iter()
            .flatten()
            .map(|expr| expr.text.clone())
            .collect(),
    }
}

/// The mockup's `.plot-pane.active::before`: a 2 px rail down the pane that
/// received the instrument's actions. Painted last so it reads over the
/// header fill and the canvas alike.
fn paint_active_rail(
    ui: &Ui,
    host: &mut impl StackHost,
    model: &StripModel,
    pane: &UnitPane,
    pane_top: f32,
    t: &Tokens,
) {
    let pane_active = host.session_mut().active_wave_pane.as_ref()
        == Some(&WavePanePresentationKey {
            analysis: model.analysis_key,
            unit: pane.unit.to_owned(),
        });
    if pane_active {
        let bottom = ui.min_rect().bottom().max(pane_top);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                egui::pos2(ui.min_rect().left(), pane_top),
                egui::pos2(ui.min_rect().left() + 2.0, bottom),
            ),
            0.0,
            t.color.accent,
        );
    }
}

#[cfg(test)]
mod tests;

/// The plots the stack drew in one pass, top to bottom.
#[derive(Clone, Default)]
struct PlotStops {
    pass: u64,
    plots: Vec<egui::Id>,
}

fn plot_stops_id() -> egui::Id {
    egui::Id::new("rspice.results.plot-stops")
}

/// Note a plot the stack drew this pass, after the ones above it.
pub(crate) fn note_plot_stop(ctx: &egui::Context, plot: egui::Id) {
    let pass = ctx.cumulative_pass_nr();
    ctx.data_mut(|data| {
        let stops = data.get_temp_mut_or_default::<PlotStops>(plot_stops_id());
        if stops.pass != pass {
            *stops = PlotStops {
                pass,
                plots: Vec::new(),
            };
        }
        stops.plots.push(plot);
    });
}

/// The plots of every strip and pane the stack drew this pass, top to
/// bottom: where F6 and Shift+F6 step inside the Results document before
/// they go on to the next part of the workbench. Empty in a pass that drew
/// no stack.
pub fn plot_stops(ctx: &egui::Context) -> Vec<egui::Id> {
    let pass = ctx.cumulative_pass_nr();
    ctx.data(|data| data.get_temp::<PlotStops>(plot_stops_id()))
        .filter(|stops| stops.pass == pass)
        .map(|stops| stops.plots)
        .unwrap_or_default()
}
