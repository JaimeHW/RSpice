//! Waveform stack layout and local viewer interaction over host-qualified sources.
use super::{
    CursorDomain, ReadoutPolicy, StripModel, StripTrace, UnitPane, anchor_key,
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
    derived::DerivedSeries,
    presentation::well_hint,
    session::{ExprEditor, ResultViewerState, XyStrip},
    strip::{LegendChip, StripHeader},
};
use egui::Ui;
use rspice_results::result_presentation::{
    AnalysisPresentationKey, ExprTrace, ResultViewer, WavePanePresentationKey,
};
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
    fn pane_header(
        &mut self,
        ui: &mut Ui,
        model: &StripModel,
        pane: &UnitPane,
        ordinal: usize,
        input: pane_header::PaneHeader,
    ) -> pane_header::UnitPaneHeaderResponse;
    fn family_envelopes(&mut self, model: &StripModel, pane: &UnitPane) -> Arc<FamilyEnvelopePlan>;
    fn place_marker(&mut self, model: &StripModel, trace: &StripTrace, x: f64);
    fn note_sample_reads(&mut self, overview: usize, extrema: usize);
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
    fn real_step_traces(
        &mut self,
        _model: &StripModel,
        _pane: &UnitPane,
        _tokens: &Tokens,
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
    pub value: f32,
}

impl Gutter {
    pub fn width(self) -> f32 {
        self.name + self.value
    }
}

fn strip_gutter(ui: &Ui, host: &impl StackHost, model: &StripModel) -> Gutter {
    let axis = wave_left_margin(host.session());
    if host.event_lane_height(model) <= 0.0 {
        return Gutter {
            name: 0.0,
            value: axis,
        };
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
    Gutter {
        name: clamp_name_width(host.session().lane_name_width.unwrap_or_else(fit)),
        value: (chars as f32).mul_add(advance, 8.0).max(LANE_VALUE_MIN),
    }
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

/// Y range of one pane's traces, padded 8 %. Per-trace extremes are cached
/// on the data version — never rescanned per frame.
///
/// The fit is per pane because each pane carries its own unit: fitting
/// volts and amps to one range would flatten whichever is smaller.
pub fn pane_y_range(
    derived: &mut DerivedSeries,
    model: &StripModel,
    indices: &[usize],
    mut note_samples: impl FnMut(usize),
) -> Option<(f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for trace in indices.iter().filter_map(|index| model.traces.get(*index)) {
        let extremes = derived.range_or(trace_key(model, trace), || {
            note_samples(trace.y.len());
            rspice_results::measurements::finite_extremes(&trace.y)
        });
        if let Some((lo, hi)) = extremes {
            min = min.min(lo);
            max = max.max(hi);
        }
    }
    if !min.is_finite() {
        return None;
    }
    let pad = (max - min) * 0.08;
    // A span that cannot survive its own padding is flat as far as any axis
    // can draw it: exactly constant, or constant to within a few ulps. Both
    // widen the same way, because a range finer than the tick ladder can
    // subdivide leaves the pane with no labels at all.
    if pad <= 0.0 {
        return Some((min - 1.0, max + 1.0));
    }
    Some((min - pad, max + pad))
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
        unit: pane.unit.to_owned(),
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

pub fn shared_x_view(
    results: &ResultViewerState,
    analysis: AnalysisPresentationKey,
    pane_count: usize,
) -> Option<(f64, f64)> {
    (0..pane_count).find_map(|ordinal| {
        results
            .analysis_plot_view_pane(ResultViewer::Waves, analysis, ordinal)
            .x
    })
}

pub fn set_shared_x_view(
    results: &mut ResultViewerState,
    analysis: AnalysisPresentationKey,
    pane_count: usize,
    range: Option<(f64, f64)>,
) {
    for ordinal in 0..pane_count {
        results
            .analysis_plot_view_pane_mut(ResultViewer::Waves, analysis, ordinal)
            .x = range;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct WaveStackGeometry {
    /// Height of one weight unit, before a pane's own weight is applied.
    pane_unit_height: f32,
    total_weight: f32,
    shared_x_height: f32,
    seam_height: f32,
}

/// The mockup's pane weights: the sheet's primary quantity takes three parts
/// and every companion pane two, so a supply-current or phase pane reads as
/// the secondary evidence it is instead of splitting the stack evenly.
fn pane_weight(ordinal: usize, pane_count: usize) -> f32 {
    if pane_count <= 1 {
        1.0
    } else if ordinal == 0 {
        3.0
    } else {
        2.0
    }
}

impl WaveStackGeometry {
    fn pane_height(&self, ordinal: usize, pane_count: usize) -> f32 {
        (self.pane_unit_height * pane_weight(ordinal, pane_count)).max(0.0)
    }
}

fn wave_stack_geometry(available_height: f32, pane_count: usize) -> WaveStackGeometry {
    if pane_count == 0 || !available_height.is_finite() {
        return WaveStackGeometry {
            pane_unit_height: 0.0,
            total_weight: 1.0,
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
    let shared_x_height = (content * 0.28).min(WAVE_SHARED_X_HEIGHT);
    let total_weight: f32 = (0..pane_count)
        .map(|ordinal| pane_weight(ordinal, pane_count))
        .sum();
    let pane_unit_height = ((content - shared_x_height) / total_weight).max(0.0);
    WaveStackGeometry {
        pane_unit_height,
        total_weight,
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

pub fn expression_label(expr: &ExprTrace, complex: bool) -> String {
    if expr.complex_policy.is_legacy() {
        format!("legacy magnitude · {}", elide(&expr.text, 24))
    } else if complex {
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
        well_hint(ui, "All strips hidden — restore them from the document bar");
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
    let separators = (rows.len().saturating_sub(1)) as f32;
    let strip_height = ((avail.height() - separators) / rows.len() as f32).max(140.0);
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
                    // 1 px border seam between strips.
                    let (seam, _) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width(), 1.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(seam, 0.0, t.color.border);
                }
                if let Some(index) = xy {
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), strip_height),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_min_height(strip_height);
                            let mut edits = ExpressionEdits::default();
                            if show_xy_strip(
                                ui,
                                host,
                                model,
                                index,
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
                        let header = StripHeader::new(&model.kind_tag, &model.subtitle, &legend)
                            .id_salt(model.analysis_key)
                            .incomplete(model.incomplete)
                            .maximized(maximized)
                            .closable(pane_chrome && !maximized && n > 1)
                            .zoomed(zoomed)
                            .expr_action(pane_chrome)
                            .removable_from(0)
                            .chip_menu(&mut conversions)
                            .pane_actions(pane_chrome)
                            .show(ui);
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
                                model,
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
    ui.menu_button("Digital", |ui| {
        // The threshold starts at the middle of what the expression draws.
        let values = host
            .resolve_expressions(model, t)
            .into_iter()
            .find(|resolved| resolved.slot == slot)
            .map(|resolved| resolved.y)
            .unwrap_or_default();
        if let Some(text) = super::logic::convert::digital_form_of(ui, &expr.text, &values) {
            host.add_expression(model, text);
        }
    });
}

#[derive(Clone, Copy)]
pub(crate) struct StripFrame<'a> {
    pub(crate) model: &'a StripModel,
    pub(crate) x_domain: (f64, f64),
    pub(crate) pane_count: usize,
    pub(crate) linked_cursor_domain: Option<&'a CursorDomain>,
    pub(crate) options: StackOptions,
    pub(crate) gutter: Gutter,
}

/// Why a strip's X axis cannot be logarithmic, when it cannot: some X it
/// shows is negative, or none is positive. A domain that starts at zero, as
/// time does, is shown from its first positive sample.
pub fn log_x_refusal(model: &StripModel) -> Option<String> {
    let (mut minimum, mut maximum) = (f64::INFINITY, f64::NEG_INFINITY);
    for x in model
        .traces
        .iter()
        .filter(|trace| trace.visible)
        .flat_map(|trace| trace.x.iter().copied())
        .filter(|x| x.is_finite())
    {
        minimum = minimum.min(x);
        maximum = maximum.max(x);
    }
    let name = if model.x_label.is_empty() {
        "the X axis"
    } else {
        model.x_label.as_str()
    };
    if !maximum.is_finite() {
        return Some("the strip shows no X values".to_owned());
    }
    if minimum < 0.0 {
        return Some(format!(
            "{name} reaches {}, and a logarithmic axis shows positive values only",
            rspice_ui_kit::plot::fmt_si_significant(minimum, &model.x_unit, 4)
        ));
    }
    (maximum <= 0.0).then(|| format!("{name} has no positive value"))
}

/// Resolve source ownership, then apply the navigator's actions to the app.
fn show_shared_x_axis(ui: &mut Ui, host: &mut impl StackHost, height: f32, frame: StripFrame<'_>) {
    let StripFrame {
        model,
        x_domain: full_domain,
        pane_count,
        linked_cursor_domain,
        options,
        gutter,
    } = frame;
    if height <= 1.0 {
        return;
    }
    let current =
        shared_x_view(host.session(), model.analysis_key, pane_count).unwrap_or(full_domain);
    let cursor_owner = host.session().cursor_strip == Some(model.analysis_index);
    let linked_cursor =
        host.session().linked_cursors && linked_cursor_domain == Some(&model.cursor_domain());
    let input = navigation::SharedXAxis {
        full_domain,
        current,
        height,
        left_margin: gutter.width(),
        quantity_policy: options.quantity,
        significant_digits: usize::from(options.readout.displayed_significant_digits().get()),
        cursors: (cursor_owner || linked_cursor).then_some(host.session().cursors),
        log_refusal: log_x_refusal(model),
    };
    let output = navigation::show_shared_x_axis(ui, &mut host.session_mut().derived, model, input);
    host.note_sample_reads(output.overview_samples_read, output.extrema_samples_read);
    if output.scale_toggled {
        let scale = match model.x_scale {
            XScale::Log10 => XScale::Linear,
            XScale::Linear => XScale::Log10,
        };
        let session = host.session_mut();
        session.x_scales.insert(model.analysis_key, scale);
        // A range read on one scale is no viewport on the other.
        set_shared_x_view(session, model.analysis_key, pane_count, None);
        return;
    }
    if let Some(viewport) = output.viewport {
        let range = match viewport {
            navigation::SharedXViewChange::Fit => None,
            navigation::SharedXViewChange::Range(range) => Some(range),
        };
        set_shared_x_view(host.session_mut(), model.analysis_key, pane_count, range);
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
    model: &StripModel,
    linked_cursor_domain: Option<&CursorDomain>,
    options: StackOptions,
    edits: &mut ExpressionEdits,
) {
    let t = Tokens::get(ui.ctx());
    // A strip with signals and nothing of them plotted has no pane, and so
    // no legend to enable one in: they wait in the signal browser.
    const NOTHING_PLOTTED: &str = "Nothing plotted — tick a signal in Signals";
    let Some(x_domain) = model.x_range else {
        let retained = !model.traces.is_empty() || !model.lane_traces.is_empty();
        well_hint(ui, if retained { NOTHING_PLOTTED } else { "No data" });
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
    for (pane, exprs) in panes.iter().zip(&mut pane_exprs) {
        exprs.extend(host.real_step_traces(model, pane, &t));
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
            pane_auto_y(range, exprs, &[]).is_none()
        })
        .collect();
    host.note_sample_reads(0, extrema_samples);
    let collapsed_count = collapsed.iter().filter(|collapsed| **collapsed).count();
    let available = ui.available_rect_before_wrap();
    let count = panes.len();
    let gutter = strip_gutter(ui, host, model);
    let marker_height = event_marker_band(ui, host, model, (x_domain, gutter), count);
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
        pane_count: count,
        linked_cursor_domain,
        options,
        gutter,
    };

    let mut expanded_ordinal = 0;
    for (ordinal, pane) in panes.iter().enumerate() {
        if ordinal > 0 {
            let (seam, _) = ui.allocate_exact_size(
                egui::vec2(
                    ui.available_width(),
                    1.0_f32.min(ui.available_height().max(0.0)),
                ),
                egui::Sense::hover(),
            );
            ui.painter().rect_filled(seam, 0.0, t.color.canvas_grid);
        }
        let pane_height = if collapsed[ordinal] {
            WAVE_PANE_HEADER_HEIGHT.min(ui.available_height())
        } else {
            expanded_ordinal += 1;
            geometry.pane_height(expanded_ordinal - 1, expanded_count)
        };
        let pane_exprs = std::mem::take(&mut pane_exprs[ordinal]);
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), pane_height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_height(pane_height);
                show_unit_pane(ui, host, pane, ordinal, &pane_exprs, frame, edits);
            },
        );
    }
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
            let value = sample_at_with_shape(
                &first.x,
                &first.y,
                &first.shape,
                x,
                cursor_interpolation(presentation.cursor_interpolation()),
            );
            super::units::fmt_in_unit_readout(
                value,
                pane.axis_unit(),
                usize::from(presentation.displayed_significant_digits().get()),
            )
        });
        Some(pane_header::PaneExpression {
            slot,
            text: expr.text.clone(),
            label: label(expr),
            color: first.color,
            swatch: swatch(slot, first.family_style),
            visible: true,
            at_cursor,
            curves: members.len(),
            values: Arc::clone(&first.y),
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
            values: Arc::default(),
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
            .position(|pane| pane.expressions.contains(&expr.slot))
            .or((!routed).then_some(0));
        if let Some(exprs) = ordinal.and_then(|ordinal| pane_exprs.get_mut(ordinal)) {
            exprs.push(expr);
        }
    }
    pane_exprs
}

/// One strip plotting a unit pane's curves against a signal or an
/// expression: a header naming the abscissa, with the span of the sweep it
/// draws and a close button; the pane's own header, whose legend and scale
/// it shares; and the plot, whose one horizontal axis is the abscissa. No
/// sweep ruler and no overview stand under it. True when the reader closed
/// it.
fn show_xy_strip(
    ui: &mut Ui,
    host: &mut impl StackHost,
    model: &StripModel,
    index: usize,
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
        .show(ui);
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
        .map(|domain| super::xy::drawn_span(host.session(), &strip, domain, panes.len()));
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
        well_hint(ui, "No data");
        return false;
    };
    let Some(ordinal) = panes.iter().position(|pane| pane.unit == strip.unit) else {
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
        pane_count: panes.len(),
        linked_cursor_domain,
        options,
        gutter,
    };

    // The pane's header: its legend, its unit and its scale, which the strip
    // shares with the pane it plots.
    let pane_range = pane_y_range(&mut host.session_mut().derived, model, &pane.traces, |_| {});
    let auto_y = pane_auto_y(pane_range, &exprs, &[]);
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
        name_width: (gutter.name > 0.0).then_some(gutter.name),
        expressions: pane_expressions(host, model, pane, &exprs, cursor_a, &t),
        plotted: auto_y.is_some(),
        abscissa: options
            .pane_chrome
            .then(|| abscissa_menu(host.session(), model, pane, Some(strip.abscissa.clone()))),
    };
    let header = ui
        .push_id(("xy-strip-pane", identity), |ui| {
            host.pane_header(ui, model, pane, ordinal, input)
        })
        .inner;
    edits.toggle = edits.toggle.or(header.toggle_expression);
    edits.remove = edits.remove.or(header.remove_expression);
    if let Some(width) = header.resize_name {
        host.session_mut().lane_name_width = Some(clamp_name_width(width));
    }
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
                },
            );
        });
    } else if ui.available_height() >= WAVE_MIN_PLOT_HEIGHT {
        well_hint(ui, "No visible traces — enable one in the legend");
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
}

/// At most this share of a strip goes to event lanes when it also has panes;
/// more lanes than fit scroll within it.
const EVENT_LANE_SHARE: f32 = 0.5;

/// The time axis the strip's panes are drawn on, for the bands above them.
fn event_frame(
    host: &impl StackHost,
    model: &StripModel,
    (x_domain, gutter): ((f64, f64), Gutter),
    pane_count: usize,
) -> EventLaneFrame {
    EventLaneFrame {
        x_range: shared_x_view(host.session(), model.analysis_key, pane_count).unwrap_or(x_domain),
        left_margin: gutter.width(),
        name_width: gutter.name,
        right_margin: navigation::WAVE_SHARED_RIGHT_MARGIN,
    }
}

/// Draw the strip's event-marker strip at its top, when the run has any
/// marker, and return the height it took.
fn event_marker_band(
    ui: &mut Ui,
    host: &mut impl StackHost,
    model: &StripModel,
    domain: ((f64, f64), Gutter),
    pane_count: usize,
) -> f32 {
    let available = ui.available_rect_before_wrap().height().max(0.0);
    let height = host.event_marker_height(model).max(0.0).min(available);
    if height <= 0.0 {
        return 0.0;
    }
    let frame = event_frame(host, model, domain, pane_count);
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
    let frame = event_frame(host, model, domain, pane_count);
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
    ordinal: usize,
    exprs: &[ResolvedExpr],
    frame: StripFrame<'_>,
    edits: &mut ExpressionEdits,
) {
    let StripFrame {
        model,
        x_domain,
        pane_count,
        linked_cursor_domain,
        options,
        gutter,
    } = frame;
    let t = Tokens::get(ui.ctx());
    let presentation = options.readout;
    let quantity_policy = options.quantity;
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let (x0, x1) = x_domain;
    // The pane's own top edge, kept so the active rail can span header and
    // canvas together once the pane's full height is known.
    let pane_top = ui.available_rect_before_wrap().top();

    let mut extrema_samples = 0;
    let pane_range = pane_y_range(
        &mut host.session_mut().derived,
        model,
        &pane.traces,
        |count| extrema_samples += count,
    );
    host.note_sample_reads(0, extrema_samples);
    let specification_limits = if host.session().show_spec_limits {
        host.specification_limits(model, pane, &t)
    } else {
        Vec::new()
    };
    let auto_y = pane_auto_y(pane_range, exprs, &specification_limits);
    // A pane with nothing to draw has no Y axis for its header to act on.
    let plotted = pane_auto_y(pane_range, exprs, &[]).is_some();
    let log_y_available = auto_y.is_some_and(|(minimum, maximum)| minimum > 0.0 && maximum > 0.0);
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
        name_width: (gutter.name > 0.0).then_some(gutter.name),
        expressions: pane_expressions(host, model, pane, exprs, cursor_a, &t),
        plotted,
        abscissa: options
            .pane_chrome
            .then(|| abscissa_menu(host.session(), model, pane, None)),
    };
    let header = host.pane_header(ui, model, pane, ordinal, input);
    edits.toggle = edits.toggle.or(header.toggle_expression);
    edits.remove = edits.remove.or(header.remove_expression);
    if let Some(width) = header.resize_name {
        host.session_mut().lane_name_width = Some(clamp_name_width(width));
    }
    if let Some(abscissa) = header.abscissa {
        open_xy_strip(ui.ctx(), host.session_mut(), model, pane.unit, abscissa);
    }
    if header.autoscale_y || header.toggle_log_y {
        let view = host.session_mut().analysis_plot_view_pane_mut(
            ResultViewer::Waves,
            model.analysis_key,
            ordinal,
        );
        view.y = None;
    }
    if header.toggle_log_y {
        log_y = !log_y;
        set_pane_log_y(host.session_mut(), model, pane, log_y);
    }

    let Some((auto_y0, auto_y1)) = auto_y else {
        // A pane with nothing plotted is its header alone; the hint is for a
        // pane whose plotted traces hold no finite value to fit.
        if ui.available_height() >= WAVE_MIN_PLOT_HEIGHT {
            well_hint(ui, "No visible traces — enable one in the legend");
        }
        return;
    };
    if ui.available_height() < WAVE_MIN_PLOT_HEIGHT {
        return;
    }

    // User zoom/pan overrides the automatic fit per axis, per pane.
    let pane_view =
        host.session()
            .analysis_plot_view_pane(ResultViewer::Waves, model.analysis_key, ordinal);
    let (x0, x1) =
        shared_x_view(host.session(), model.analysis_key, pane_count).unwrap_or((x0, x1));
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
        expressions: exprs,
        family_envelopes,
        find_nearest: host.session().marker_tool.is_armed()
            || host.session().trace_pick.is_armed()
            || (host.session().cursor_placement_enabled() && host.session().cursor_a_is_next()),
    };
    let drawn = pane_view::show_plot(ui, &mut host.session_mut().cache, model, pane, input);
    let response = drawn.response;
    if response.response.hovered()
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
        let placing_cursor_a = host.session().cursor_a_is_next();
        let nearest_anchor =
            placing_cursor_a.then(|| drawn.nearest_trace.map(|trace| anchor_key(model, trace)));
        let results = host.session_mut();
        if results.cursor_strip != Some(model.analysis_index)
            && (!results.linked_cursors || !cursor_domain_matches)
        {
            results.cursors = CursorPair::default();
        }
        results.cursor_strip = Some(model.analysis_index);
        if placing_cursor_a {
            results.cursor_a_anchor = nearest_anchor.flatten();
        }
        results.stepping_cursor = results.cursors.next_placed();
        results.cursors.place(clicked_x);
    }

    if response.view.reset {
        set_shared_x_view(host.session_mut(), model.analysis_key, pane_count, None);
        let view = host.session_mut().analysis_plot_view_pane_mut(
            ResultViewer::Waves,
            model.analysis_key,
            ordinal,
        );
        view.y = None;
    } else if response.view.any() {
        if let Some(x) = response.view.x {
            set_shared_x_view(host.session_mut(), model.analysis_key, pane_count, Some(x));
        }
        let view = host.session_mut().analysis_plot_view_pane_mut(
            ResultViewer::Waves,
            model.analysis_key,
            ordinal,
        );
        if let Some(y) = response.view.y {
            view.y = Some(y);
        }
    }

    paint_active_rail(ui, host, model, pane, pane_top, &t);
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
        if model.trace_unit(trace) == pane.unit {
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
