//! Waveform stack layout and local viewer interaction over host-qualified sources.
use super::{
    CursorDomain, ReadoutPolicy, StripModel, StripTrace, UnitPane, anchor_key,
    cursor_interpolation,
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
    session::{ExprEditor, HorizontalWaveCursor, ResultViewerState},
    strip::{LegendChip, StripHeader},
};
use egui::Ui;
use rspice_app_types::quantity::QuantityPresentationPolicy;
use rspice_results::result_presentation::{
    AnalysisPresentationKey, ExprTrace, ResultViewer, WavePanePresentationKey,
};
use rspice_ui_kit::{
    plot::{self, CursorPair, DisplayDecimation},
    tokens::Tokens,
};
use std::sync::Arc;

/// Display preferences captured for one stack frame.
#[derive(Clone, Copy)]
pub struct StackOptions {
    pub pane_chrome: bool,
    pub readout: ReadoutPolicy,
    pub quantity: QuantityPresentationPolicy,
    pub display_decimation: DisplayDecimation,
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
    let n = visible.len();
    let separators = (n.saturating_sub(1)) as f32;
    let strip_height = ((avail.height() - separators) / n as f32).max(140.0);
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
            for (position, model) in visible.iter().enumerate() {
                if position > 0 {
                    // 1 px border seam between strips.
                    let (seam, _) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width(), 1.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(seam, 0.0, t.color.border);
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
                        let header = StripHeader::new(&model.kind_tag, &model.subtitle, &legend)
                            .incomplete(model.incomplete)
                            .maximized(maximized)
                            .closable(pane_chrome && !maximized && n > 1)
                            .zoomed(zoomed)
                            .expr_action(pane_chrome)
                            .removable_from(0)
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
                            show_strip_plot(
                                ui,
                                host,
                                model,
                                linked_cursor_domain.as_ref(),
                                options,
                            );
                        } else {
                            ui.allocate_exact_size(plot_rect.size(), egui::Sense::hover());
                        }
                    },
                );
            }
        });

    // Apply deferred mutations.
    let results = host.session_mut();
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
    }
}

#[derive(Clone, Copy)]
struct StripFrame<'a> {
    model: &'a StripModel,
    x_domain: (f64, f64),
    pane_count: usize,
    linked_cursor_domain: Option<&'a CursorDomain>,
    options: StackOptions,
}

/// Resolve source ownership, then apply the navigator's actions to the app.
fn show_shared_x_axis(ui: &mut Ui, host: &mut impl StackHost, height: f32, frame: StripFrame<'_>) {
    let StripFrame {
        model,
        x_domain: full_domain,
        pane_count,
        linked_cursor_domain,
        options,
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
        left_margin: wave_left_margin(host.session()),
        quantity_policy: options.quantity,
        cursors: (cursor_owner || linked_cursor).then_some(host.session().cursors),
    };
    let output = navigation::show_shared_x_axis(ui, &mut host.session_mut().derived, model, input);
    host.note_sample_reads(output.overview_samples_read, output.extrema_samples_read);
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
        match cursor {
            navigation::CursorMove::A(x) => {
                host.session_mut().cursors.a = Some(x);
                host.session_mut().cursor_a_anchor = None;
            }
            navigation::CursorMove::B(x) => host.session_mut().cursors.b = Some(x),
        }
    }
}

/// One strip, drawn as one pane per unit.
///
/// Signals route to the pane that owns their unit; the panes stack and
/// share the strip's X domain, so a strip stays one measurement read
/// against as many scales as it genuinely needs.
fn show_strip_plot(
    ui: &mut Ui,
    host: &mut impl StackHost,
    model: &StripModel,
    linked_cursor_domain: Option<&CursorDomain>,
    options: StackOptions,
) {
    let t = Tokens::get(ui.ctx());
    let Some(x_domain) = model.x_range else {
        well_hint(ui, "No data");
        return;
    };
    let panes = model.unit_panes();
    if panes.is_empty() {
        well_hint(ui, "No visible traces — enable one in the legend");
        return;
    }

    // Expression traces participate in the first pane's automatic fit.
    let exprs = host.resolve_expressions(model, &t);
    let available = ui.available_rect_before_wrap();
    let count = panes.len();
    let geometry = wave_stack_geometry(available.height(), count);
    let frame = StripFrame {
        model,
        x_domain,
        pane_count: count,
        linked_cursor_domain,
        options,
    };

    for (ordinal, pane) in panes.iter().enumerate() {
        if ordinal > 0 {
            let (seam, _) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), geometry.seam_height),
                egui::Sense::hover(),
            );
            ui.painter().rect_filled(seam, 0.0, t.color.canvas_grid);
        }
        let pane_height = geometry.pane_height(ordinal, count);
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), pane_height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_height(pane_height);
                show_unit_pane(
                    ui,
                    host,
                    pane,
                    ordinal,
                    // Only the first pane carries the strip's expressions:
                    // an expression has no declared unit, so it cannot be
                    // routed to a unit-owning pane on the evidence available.
                    if ordinal == 0 { &exprs } else { &[] },
                    frame,
                );
            },
        );
    }
    show_shared_x_axis(ui, host, geometry.shared_x_height, frame);
}

fn show_unit_pane(
    ui: &mut Ui,
    host: &mut impl StackHost,
    pane: &UnitPane,
    ordinal: usize,
    exprs: &[ResolvedExpr],
    frame: StripFrame<'_>,
) {
    let StripFrame {
        model,
        x_domain,
        pane_count,
        linked_cursor_domain,
        options,
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
    };
    let header = host.pane_header(ui, model, pane, ordinal, input);
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
        well_hint(ui, "No visible traces — enable one in the legend");
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
                x: marker.x(),
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
        left_margin: wave_left_margin(host.session()),
        readout: presentation,
        quantity: quantity_policy,
        display_decimation: options.display_decimation,
        minor_grid: host.session().show_minor_grid,
        horizontal_cursor: host
            .session()
            .horizontal_cursor
            .as_ref()
            .filter(|cursor| cursor.pane == pane_key)
            .map(|cursor| cursor.y),
        horizontal_cursor_interactive: host.session().horizontal_cursor_placement_enabled(),
        cursors,
        specification_limits,
        markers,
        expressions: exprs,
        family_envelopes,
        find_nearest: host.session().marker_tool.is_armed()
            || (host.session().cursor_placement_enabled() && host.session().cursor_a_is_next()),
    };
    let drawn = pane_view::show_plot(ui, &mut host.session_mut().cache, model, pane, input);
    let response = drawn.response;
    if response.response.hovered()
        || response.response.dragged()
        || response.clicked_x.is_some()
        || response.horizontal_cursor_y.is_some()
    {
        host.session_mut().active_wave_pane = Some(pane_key.clone());
    }
    if let Some(y) = response.horizontal_cursor_y {
        host.session_mut().horizontal_cursor = Some(HorizontalWaveCursor { pane: pane_key, y });
    }

    // The marker tool takes the click when armed: one click cannot both
    // annotate and move a cursor, and the armed chip says which it will do.
    if let Some(clicked_x) = response.clicked_x
        && host.session().marker_tool.is_armed()
    {
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

    // The mockup's `.plot-pane.active::before`: a 2 px rail down the pane
    // that received the instrument's actions. Painted last so it reads over
    // the header fill and the canvas alike.
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
