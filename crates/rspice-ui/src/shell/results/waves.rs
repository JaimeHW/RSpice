//! WAVES — stacked waveform strips, one per analysis in the active run.
//!
//! Each strip carries its analysis' traces with the strip grammar (header ·
//! legend · actions over a document well). AC strips convert magnitude to dB
//! on the left axis and route phase traces, dashed, to a right axis. A/B
//! cursors live on one strip at a time; their values, deltas and windowed
//! measurements render in the right panel.

use std::sync::Arc;

use egui::Ui;

use crate::analysis::calculator;
use crate::common::AppState;
use crate::state::{AnalysisType, SimulationState};
use crate::ui::plot::{self, Axis, CursorPair, PlotSpec, Trace, XScale, fmt_si, sample_at};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::section_header;

use super::strip::{LegendChip, StripHeader};
use super::{
    DerivedSeries, ExprEditor, ExprSeries, ExprTrace, ResultsState, waveform_color, well_hint,
};

/// How a trace's Y values are interpreted.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TraceKind {
    /// Plain values (V, A, sweep output).
    Value,
    /// dB-converted AC magnitude.
    MagnitudeDb,
    /// Phase in degrees (right axis, dashed).
    PhaseDeg,
}

/// One trace of a strip, with owned `Arc` handles into the run data.
struct StripTrace {
    waveform_index: usize,
    name: String,
    color: egui::Color32,
    x: Arc<[f64]>,
    y: Arc<[f64]>,
    kind: TraceKind,
    visible: bool,
    /// The run this trace belongs to (cache-key discriminator).
    run_id: u64,
    /// Overlay traces come from a non-active run: same signal hue, reduced
    /// weight, visibility slaved to the active run's matching signal.
    overlay: bool,
}

/// One strip (== one analysis of the active run).
pub(super) struct StripModel {
    analysis_index: usize,
    kind_tag: String,
    subtitle: String,
    x_scale: XScale,
    x_unit: &'static str,
    y_unit: &'static str,
    /// Phase traces carry the unwrapped (continuous) series instead of the
    /// raw ±180°-wrapped samples. Folded into the cache keys.
    phase_continuous: bool,
    /// Number of active-run traces at the front of `traces`; everything
    /// after is overlay. The legend lists only this prefix (signal owns
    /// hue — one chip per signal, all runs).
    signal_trace_count: usize,
    traces: Vec<StripTrace>,
}

/// Frame cache for the strip models. Building them clones every trace name
/// and walks all overlay runs, and both the center view and the right panel
/// ask for them each frame — the fingerprint covers everything the models
/// read, so the rebuild only happens when an input actually changes.
#[derive(Default, Clone)]
pub(super) struct ModelsCache(Option<(u64, Arc<Vec<StripModel>>)>);

impl std::fmt::Debug for ModelsCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelsCache(..)")
    }
}

/// Everything `build_models` reads: run data version, the display-run set,
/// per-trace visibility and stored color, phase mode, and the theme palette.
fn models_fingerprint(simulation: &SimulationState, phase_continuous: bool, t: &Tokens) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    simulation.data_version.hash(&mut h);
    phase_continuous.hash(&mut h);
    for color in &t.color.traces {
        color.to_array().hash(&mut h);
    }
    for run in simulation.display_runs() {
        run.id.hash(&mut h);
        for analysis in &run.analyses {
            analysis.waveforms.len().hash(&mut h);
            for waveform in &analysis.waveforms {
                waveform.visible.hash(&mut h);
                waveform.color.hash(&mut h);
            }
        }
    }
    h.finish()
}

/// Fingerprint-cached [`build_models`]; the returned handle is cheap to
/// clone and stays valid across later state borrows.
pub(super) fn cached_models(
    simulation: &SimulationState,
    results: &mut ResultsState,
    t: &Tokens,
) -> Arc<Vec<StripModel>> {
    let fp = models_fingerprint(simulation, results.phase_continuous, t);
    if let Some((cached_fp, models)) = &results.models.0 {
        if *cached_fp == fp {
            return Arc::clone(models);
        }
    }
    let models = Arc::new(build_models(
        simulation,
        &mut results.derived,
        t,
        results.phase_continuous,
    ));
    results.models.0 = Some((fp, Arc::clone(&models)));
    models
}

/// Fold a run identity into a cache key for overlay traces. Active-run
/// keys stay unchanged so existing envelopes/ranges remain warm.
fn run_mixed_key(base: u64, run_id: u64, overlay: bool) -> u64 {
    if overlay {
        base ^ run_id.wrapping_mul(0x9E37_79B9_7F4A_7C15)
    } else {
        base
    }
}

impl StripModel {
    fn x_label(&self) -> &'static str {
        match self.x_scale {
            XScale::Log10 => "f",
            XScale::Linear => {
                if self.x_unit == "s" {
                    "t"
                } else {
                    "x"
                }
            }
        }
    }

    fn format_x(&self, x: f64) -> String {
        fmt_si(
            x,
            if self.x_unit.is_empty() {
                ""
            } else {
                self.x_unit
            },
            3,
        )
    }

    fn format_trace_value(&self, trace: &StripTrace, value: f64) -> String {
        match trace.kind {
            TraceKind::Value => fmt_si(value, self.y_unit, 3),
            TraceKind::MagnitudeDb => format!("{value:.1} dB"),
            TraceKind::PhaseDeg => format!("{value:.1} °"),
        }
    }
}

/// Build strip models for every plottable analysis of the active run.
/// `phase_continuous` swaps phase traces to their unwrapped series.
pub(super) fn build_models(
    simulation: &SimulationState,
    derived: &mut DerivedSeries,
    tokens: &Tokens,
    phase_continuous: bool,
) -> Vec<StripModel> {
    let display_runs = simulation.display_runs();
    let Some((&run, overlay_runs)) = display_runs.split_first() else {
        return Vec::new();
    };
    let mut models = Vec::new();

    for (analysis_index, analysis) in run.analyses.iter().enumerate() {
        if analysis.waveforms.is_empty() {
            continue;
        }
        let (x_scale, x_unit, y_unit) = match analysis.analysis_type {
            AnalysisType::Ac | AnalysisType::Noise => (XScale::Log10, "Hz", "dB"),
            AnalysisType::Transient => (XScale::Linear, "s", "V"),
            AnalysisType::DcSweep => (XScale::Linear, "V", "V"),
            _ => (XScale::Linear, "", "V"),
        };

        let mut traces = Vec::new();
        for (waveform_index, waveform) in analysis.waveforms.iter().enumerate() {
            let color = waveform_color(waveform, waveform_index, tokens);
            let is_phase = waveform.name.starts_with("phase(");
            let is_mag = waveform.name.starts_with('|');
            let kind = if analysis.analysis_type == AnalysisType::Ac && is_phase {
                TraceKind::PhaseDeg
            } else if analysis.analysis_type == AnalysisType::Ac && is_mag {
                TraceKind::MagnitudeDb
            } else {
                TraceKind::Value
            };
            let y = match kind {
                TraceKind::MagnitudeDb => derived.db(
                    (analysis_index as u64) << 32 | waveform_index as u64,
                    &waveform.y,
                ),
                // Continuous phase display: cached unwrapped copy of the
                // wrapped samples, same key convention as `db`.
                TraceKind::PhaseDeg if phase_continuous => derived.unwrapped(
                    (analysis_index as u64) << 32 | waveform_index as u64,
                    &waveform.y,
                ),
                _ => Arc::clone(&waveform.y),
            };
            traces.push(StripTrace {
                waveform_index,
                name: waveform.name.clone(),
                color,
                x: Arc::clone(&waveform.x),
                y,
                kind,
                visible: waveform.visible,
                run_id: run.id,
                overlay: false,
            });
        }
        let signal_trace_count = traces.len();

        // Overlay runs: match the same analysis (type, then label when it
        // disambiguates) and merge traces by signal name. Signal owns hue —
        // overlay traces reuse the active trace's color and visibility —
        // run owns weight (applied at draw time).
        let mut overlaid_run_count = 0usize;
        for overlay_run in overlay_runs {
            let overlay_analysis = overlay_run
                .analyses
                .iter()
                .find(|candidate| {
                    candidate.analysis_type == analysis.analysis_type
                        && candidate.label == analysis.label
                })
                .or_else(|| {
                    overlay_run
                        .analyses
                        .iter()
                        .find(|candidate| candidate.analysis_type == analysis.analysis_type)
                });
            let Some(overlay_analysis) = overlay_analysis else {
                continue;
            };

            let mut contributed = false;
            for signal_index in 0..signal_trace_count {
                let (signal_name, signal_color, signal_kind, signal_visible) = {
                    let signal = &traces[signal_index];
                    (
                        signal.name.clone(),
                        signal.color,
                        signal.kind,
                        signal.visible,
                    )
                };
                let Some((overlay_index, overlay_waveform)) = overlay_analysis
                    .waveforms
                    .iter()
                    .enumerate()
                    .find(|(_, waveform)| waveform.name == signal_name)
                else {
                    continue;
                };

                let base_key = (analysis_index as u64) << 32 | overlay_index as u64;
                let derived_key = run_mixed_key(base_key, overlay_run.id, true);
                let y = match signal_kind {
                    TraceKind::MagnitudeDb => derived.db(derived_key, &overlay_waveform.y),
                    TraceKind::PhaseDeg if phase_continuous => {
                        derived.unwrapped(derived_key, &overlay_waveform.y)
                    }
                    _ => Arc::clone(&overlay_waveform.y),
                };
                traces.push(StripTrace {
                    waveform_index: overlay_index,
                    name: signal_name,
                    color: signal_color,
                    x: Arc::clone(&overlay_waveform.x),
                    y,
                    kind: signal_kind,
                    visible: signal_visible,
                    run_id: overlay_run.id,
                    overlay: true,
                });
                contributed = true;
            }
            if contributed {
                overlaid_run_count += 1;
            }
        }

        let mut subtitle = analysis.label.clone();
        if overlaid_run_count > 0 {
            subtitle = format!(
                "{subtitle} · +{overlaid_run_count} run{} overlaid",
                if overlaid_run_count == 1 { "" } else { "s" }
            );
        }

        models.push(StripModel {
            analysis_index,
            kind_tag: analysis.analysis_type.short_label().to_uppercase(),
            subtitle,
            x_scale,
            x_unit,
            y_unit,
            phase_continuous,
            signal_trace_count,
            traces,
        });
    }
    models
}

/// Stable per-trace identity shared by the decimation, range, and
/// measurement caches. Phase traces fold in the wrapped/continuous choice
/// so a toggle never serves stale envelopes, ranges, or stats.
fn trace_key(model: &StripModel, trace: &StripTrace) -> u64 {
    let continuous = (trace.kind == TraceKind::PhaseDeg && model.phase_continuous) as u64;
    let base = (model.analysis_index as u64) << 40
        | continuous << 39
        | (trace.waveform_index as u64) << 2
        | trace.kind as u64;
    run_mixed_key(base, trace.run_id, trace.overlay)
}

/// Y range of the visible traces on one axis side, padded 8 %. Per-trace
/// extremes are cached on the data version — never rescanned per frame.
fn y_range(derived: &mut DerivedSeries, model: &StripModel, phase: bool) -> Option<(f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for trace in &model.traces {
        let is_phase = trace.kind == TraceKind::PhaseDeg;
        if is_phase != phase || !trace.visible {
            continue;
        }
        let extremes =
            derived.range_or(trace_key(model, trace), || super::finite_extremes(&trace.y));
        if let Some((lo, hi)) = extremes {
            min = min.min(lo);
            max = max.max(hi);
        }
    }
    if !min.is_finite() {
        return None;
    }
    if min == max {
        return Some((min - 1.0, max + 1.0));
    }
    let pad = (max - min) * 0.08;
    Some((min - pad, max + pad))
}

/// X range of a strip (shared X across its traces).
fn x_range(model: &StripModel) -> Option<(f64, f64)> {
    let x = &model.traces.first()?.x;
    if x.is_empty() {
        return None;
    }
    let (mut x0, x1) = (x[0], x[x.len() - 1]);
    if model.x_scale == XScale::Log10 {
        if x1 <= 0.0 {
            return None;
        }
        if x0 <= 0.0 {
            x0 = x.iter().copied().find(|&v| v > 0.0)?;
        }
    }
    (x1 > x0).then_some((x0, x1))
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the strip stack.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let models = cached_models(&state.simulation, &mut state.shell.results, &t);
    if models.is_empty() {
        let hint = if state.simulation.active_run().is_none() {
            "No results yet — press F5 or the Run button to simulate"
        } else {
            "The active run has no plottable analyses"
        };
        well_hint(ui, hint);
        return;
    }

    // Apply hide/maximize strip state.
    let results = &state.shell.results;
    let visible: Vec<&StripModel> = match results.maximized_strip {
        Some(max_idx) if models.iter().any(|m| m.analysis_index == max_idx) => models
            .iter()
            .filter(|m| m.analysis_index == max_idx)
            .collect(),
        _ => models
            .iter()
            .filter(|m| !results.hidden_strips.contains(&m.analysis_index))
            .collect(),
    };
    if visible.is_empty() {
        well_hint(ui, "All strips hidden — restore them from the document bar");
        return;
    }

    // Deferred state mutations (collected while iterating immutably).
    let mut toggle_trace: Option<(usize, usize)> = None;
    let mut toggle_maximize: Option<usize> = None;
    let mut close_strip: Option<usize> = None;
    let mut fit_strip: Option<usize> = None;
    let mut toggle_expr: Option<(usize, usize)> = None;
    let mut remove_expr: Option<(usize, usize)> = None;
    let mut open_editor: Option<usize> = None;

    let avail = ui.available_rect_before_wrap();
    let n = visible.len();
    let separators = (n.saturating_sub(1)) as f32;
    let strip_height = ((avail.height() - separators) / n as f32).max(140.0);
    let maximized = state.shell.results.maximized_strip.is_some();

    egui::ScrollArea::vertical()
        .id_salt("volta.results.strips")
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
                        // Legend chips list signals only (the active-run
                        // prefix): one chip per signal toggles it across
                        // every overlaid run.
                        let legend: Vec<LegendChip<'_>> = model
                            .traces
                            .iter()
                            .take(model.signal_trace_count)
                            .map(|trace| LegendChip {
                                name: &trace.name,
                                color: trace.color,
                                on: trace.visible,
                            })
                            .collect();
                        // Expression chips follow the waveform chips; long
                        // expressions are elided so a chip never eats the row.
                        let strip_exprs: Vec<ExprTrace> = state
                            .shell
                            .results
                            .exprs
                            .get(&model.analysis_index)
                            .cloned()
                            .unwrap_or_default();
                        let expr_labels: Vec<String> =
                            strip_exprs.iter().map(|e| elide(&e.text, 24)).collect();
                        let mut legend = legend;
                        for (i, expr) in strip_exprs.iter().enumerate() {
                            legend.push(LegendChip {
                                name: &expr_labels[i],
                                color: expr_color(&t, model.signal_trace_count + i),
                                on: expr.visible,
                            });
                        }

                        let zoomed = state
                            .shell
                            .results
                            .plot_view(super::ResultViewer::Waves, model.analysis_index)
                            .is_zoomed();
                        let header = StripHeader::new(&model.kind_tag, &model.subtitle, &legend)
                            .maximized(maximized)
                            .closable(!maximized && n > 1)
                            .zoomed(zoomed)
                            .expr_action(true)
                            .removable_from(model.signal_trace_count)
                            .show(ui);
                        if let Some(chip_index) = header.legend_clicked {
                            if chip_index < model.signal_trace_count {
                                if let Some(trace) = model.traces.get(chip_index) {
                                    toggle_trace =
                                        Some((model.analysis_index, trace.waveform_index));
                                }
                            } else {
                                toggle_expr = Some((
                                    model.analysis_index,
                                    chip_index - model.signal_trace_count,
                                ));
                            }
                        }
                        if let Some(chip_index) = header.legend_removed {
                            if chip_index >= model.signal_trace_count {
                                remove_expr = Some((
                                    model.analysis_index,
                                    chip_index - model.signal_trace_count,
                                ));
                            }
                        }
                        if header.maximize_clicked {
                            toggle_maximize = Some(model.analysis_index);
                        }
                        if header.close_clicked {
                            close_strip = Some(model.analysis_index);
                        }
                        if header.fit_clicked {
                            fit_strip = Some(model.analysis_index);
                        }
                        if header.add_expr_clicked {
                            open_editor = Some(model.analysis_index);
                        }

                        expr_editor_row(ui, state, model.analysis_index);

                        // Strips scrolled out of view skip the plot body
                        // entirely (range lookups, envelope mapping, shape
                        // building) — only the space is reserved.
                        let plot_rect = ui.available_rect_before_wrap();
                        if ui.is_rect_visible(plot_rect) {
                            show_strip_plot(ui, state, model);
                        } else {
                            ui.allocate_exact_size(plot_rect.size(), egui::Sense::hover());
                        }
                    },
                );
            }
        });

    // Apply deferred mutations.
    if let Some((analysis_index, waveform_index)) = toggle_trace {
        toggle_visibility(state, analysis_index, waveform_index);
    }
    let results = &mut state.shell.results;
    if let Some(idx) = toggle_maximize {
        results.maximized_strip = (results.maximized_strip != Some(idx)).then_some(idx);
    }
    if let Some(idx) = close_strip {
        results.hidden_strips.insert(idx);
        if results.cursor_strip == Some(idx) {
            results.clear_cursors();
        }
    }
    if let Some(idx) = fit_strip {
        results.reset_plot_view(super::ResultViewer::Waves, idx);
    }
    if let Some((analysis, index)) = toggle_expr {
        if let Some(expr) = results
            .exprs
            .get_mut(&analysis)
            .and_then(|list| list.get_mut(index))
        {
            expr.visible = !expr.visible;
        }
    }
    if let Some((analysis, index)) = remove_expr {
        if let Some(list) = results.exprs.get_mut(&analysis) {
            if index < list.len() {
                let removed = list.remove(index);
                results.expr_cache.remove(&(analysis, removed.text));
            }
            if list.is_empty() {
                results.exprs.remove(&analysis);
            }
        }
    }
    if let Some(analysis) = open_editor {
        results.expr_editor = Some(ExprEditor {
            analysis_index: analysis,
            text: String::new(),
            error: None,
            want_focus: true,
        });
    }
}

/// Shorten a label to `max` characters with a typographic ellipsis.
fn elide(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_owned();
    }
    let mut out: String = text.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

/// Palette color for the i-th trace slot of a strip (waveforms, then
/// expressions).
fn expr_color(tokens: &Tokens, slot: usize) -> egui::Color32 {
    tokens.color.traces[slot % tokens.color.traces.len()]
}

/// The inline expression editor row under a strip header (when open for
/// this strip): mono input, Enter/Add commits, Esc closes, error inline.
fn expr_editor_row(ui: &mut Ui, state: &mut AppState, analysis_index: usize) {
    let open_for_strip = state
        .shell
        .results
        .expr_editor
        .as_ref()
        .is_some_and(|e| e.analysis_index == analysis_index);
    if !open_for_strip {
        return;
    }

    let t = Tokens::get(ui.ctx());
    let c = t.color;

    enum Action {
        None,
        Commit,
        Cancel,
    }
    let mut action = Action::None;

    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 34.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, c.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );

    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(egui::vec2(10.0, 5.0)))
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    let row = &mut child;
    row.spacing_mut().item_spacing.x = 8.0;

    row.label(
        egui::RichText::new("expr")
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(c.text_dim),
    );

    let editor = state
        .shell
        .results
        .expr_editor
        .as_mut()
        .expect("checked above");
    let response = row.add(
        egui::TextEdit::singleline(&mut editor.text)
            .font(theme::mono(tokens::FS_1, FontWeight::Regular))
            .hint_text("V(out)/V(in) · dB(V(out)) · deriv(V(out))")
            .desired_width(340.0),
    );
    if editor.want_focus {
        response.request_focus();
        editor.want_focus = false;
    }
    if response.lost_focus() && row.input(|i| i.key_pressed(egui::Key::Enter)) {
        action = Action::Commit;
    }
    if row.input(|i| i.key_pressed(egui::Key::Escape)) {
        action = Action::Cancel;
    }
    if crate::ui::widgets::Button::new("Add").show(row).clicked() {
        action = Action::Commit;
    }
    if let Some(error) = &editor.error {
        row.label(
            egui::RichText::new(elide(error, 64))
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(c.err),
        );
    }

    match action {
        Action::None => {}
        Action::Cancel => state.shell.results.expr_editor = None,
        Action::Commit => {
            let text = state
                .shell
                .results
                .expr_editor
                .as_ref()
                .map(|e| e.text.trim().to_owned())
                .unwrap_or_default();
            if text.is_empty() {
                state.shell.results.expr_editor = None;
                return;
            }
            let (series, extremes) = evaluate_expression(&state.simulation, analysis_index, &text);
            match series {
                Ok(series) => {
                    state.shell.results.expr_cache.insert(
                        (analysis_index, text.clone()),
                        ExprSeries {
                            version: state.simulation.data_version,
                            series: Ok(series),
                            y_extremes: extremes,
                        },
                    );
                    state
                        .shell
                        .results
                        .exprs
                        .entry(analysis_index)
                        .or_default()
                        .push(ExprTrace {
                            text,
                            visible: true,
                        });
                    state.shell.results.expr_editor = None;
                }
                Err(error) => {
                    if let Some(editor) = state.shell.results.expr_editor.as_mut() {
                        editor.error = Some(error);
                        editor.want_focus = true;
                    }
                }
            }
        }
    }
}

/// Evaluate one expression against an analysis' waveforms. Scalars become a
/// constant trace across the analysis' x span.
fn evaluate_expression(
    simulation: &SimulationState,
    analysis_index: usize,
    text: &str,
) -> (Result<(Arc<[f64]>, Arc<[f64]>), String>, Option<(f64, f64)>) {
    let Some(analysis) = simulation
        .active_run()
        .and_then(|run| run.analyses.get(analysis_index))
    else {
        return (Err("analysis no longer exists".to_owned()), None);
    };

    let ctx = calculator::WaveformsContext::new(&analysis.waveforms);
    let expr = calculator::parser::parse(text);
    match calculator::evaluator::evaluate(&expr, &ctx) {
        Ok(calculator::CalcValue::Waveform(x, y)) if !x.is_empty() => {
            let extremes = super::finite_extremes(&y);
            (Ok((x.into(), y.into())), extremes)
        }
        Ok(calculator::CalcValue::Waveform(..)) => {
            (Err("expression produced no samples".to_owned()), None)
        }
        Ok(calculator::CalcValue::Scalar(value)) => {
            let span = analysis
                .waveforms
                .first()
                .and_then(|w| (w.x.len() >= 2).then(|| (w.x[0], w.x[w.x.len() - 1])));
            match span {
                Some((x0, x1)) => (
                    Ok((vec![x0, x1].into(), vec![value, value].into())),
                    Some((value, value)),
                ),
                None => (Err("scalar result with no x span".to_owned()), None),
            }
        }
        Err(error) => (Err(error.to_string()), None),
    }
}

/// One expression trace resolved for plotting.
struct ResolvedExpr {
    x: Arc<[f64]>,
    y: Arc<[f64]>,
    color: egui::Color32,
    cache_key: u64,
    label: String,
    y_extremes: Option<(f64, f64)>,
}

/// Refresh the expression cache for a strip at the current data version and
/// hand back plottable series (visible expressions, successful evaluations).
fn resolve_strip_exprs(
    state: &mut AppState,
    model: &StripModel,
    tokens: &Tokens,
) -> Vec<ResolvedExpr> {
    let exprs: Vec<(usize, ExprTrace)> = state
        .shell
        .results
        .exprs
        .get(&model.analysis_index)
        .map(|list| list.iter().cloned().enumerate().collect())
        .unwrap_or_default();
    if exprs.is_empty() {
        return Vec::new();
    }

    let version = state.simulation.data_version;
    let mut resolved = Vec::new();
    for (slot, expr) in exprs {
        let key = (model.analysis_index, expr.text.clone());
        let fresh = state
            .shell
            .results
            .expr_cache
            .get(&key)
            .is_some_and(|s| s.version == version);
        if !fresh {
            let (series, extremes) =
                evaluate_expression(&state.simulation, model.analysis_index, &expr.text);
            if let Err(error) = &series {
                state.push_user_message(crate::common::app::ConsoleMessage::warning(format!(
                    "expression `{}`: {}",
                    expr.text, error
                )));
            }
            state.shell.results.expr_cache.insert(
                key.clone(),
                ExprSeries {
                    version,
                    series,
                    y_extremes: extremes,
                },
            );
        }
        if !expr.visible {
            continue;
        }
        if let Some(ExprSeries {
            series: Ok((x, y)),
            y_extremes,
            ..
        }) = state.shell.results.expr_cache.get(&key)
        {
            resolved.push(ResolvedExpr {
                x: Arc::clone(x),
                y: Arc::clone(y),
                color: expr_color(tokens, model.traces.len() + slot),
                cache_key: expr_cache_key(model.analysis_index, &expr.text),
                label: elide(&expr.text, 24),
                y_extremes: *y_extremes,
            });
        }
    }
    resolved
}

/// Stable decimation-cache identity for an expression trace. The high bit
/// keeps it out of the waveform trace_key space.
fn expr_cache_key(analysis_index: usize, text: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    (analysis_index, text).hash(&mut hasher);
    hasher.finish() | (1 << 63)
}

/// Flip a waveform's visibility on the run, keeping the live copy in sync.
pub(crate) fn toggle_visibility(
    state: &mut AppState,
    analysis_index: usize,
    waveform_index: usize,
) {
    let Some(run_idx) = state.simulation.active_run_idx else {
        return;
    };
    let mut name: Option<String> = None;
    if let Some(waveform) = state
        .simulation
        .runs
        .get_mut(run_idx)
        .and_then(|run| run.analyses.get_mut(analysis_index))
        .and_then(|analysis| analysis.waveforms.get_mut(waveform_index))
    {
        waveform.visible = !waveform.visible;
        name = Some(waveform.name.clone());
    }
    // Mirror into the live waveform list when this is the active analysis.
    if state.simulation.active_analysis_idx == Some(analysis_index) {
        if let Some(name) = name {
            if let Some(live) = state
                .simulation
                .waveforms
                .iter_mut()
                .find(|w| w.name == name)
            {
                live.visible = !live.visible;
            }
        }
    }
}

fn show_strip_plot(ui: &mut Ui, state: &mut AppState, model: &StripModel) {
    let t = Tokens::get(ui.ctx());
    let Some((x0, x1)) = x_range(model) else {
        well_hint(ui, "No data");
        return;
    };

    // Expression traces participate in the automatic fit alongside the
    // run's visible traces.
    let exprs = resolve_strip_exprs(state, model, &t);
    let model_range = y_range(&mut state.shell.results.derived, model, false);
    let auto_y = {
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        if let Some((a, b)) = model_range {
            lo = a;
            hi = b;
        }
        for expr in &exprs {
            if let Some((a, b)) = expr.y_extremes {
                lo = lo.min(a);
                hi = hi.max(b);
            }
        }
        if !lo.is_finite() || !hi.is_finite() {
            None
        } else if lo == hi {
            Some((lo - 1.0, hi + 1.0))
        } else {
            Some((lo, hi))
        }
    };
    let Some((y0, y1)) = auto_y else {
        well_hint(ui, "No visible traces — enable one in the legend");
        return;
    };

    // User zoom/pan overrides the automatic fit per axis.
    let view = state
        .shell
        .results
        .plot_view(super::ResultViewer::Waves, model.analysis_index);
    let (x0, x1) = view.x.unwrap_or((x0, x1));
    let (y0, y1) = view.y.unwrap_or((y0, y1));

    let x_axis = match model.x_scale {
        XScale::Log10 => Axis::log_decades(x0, x1, model.x_unit),
        XScale::Linear => Axis::linear(x0, x1, model.x_unit),
    };
    let mut spec = PlotSpec::new(x_axis, model.x_scale, Axis::linear(y0, y1, model.y_unit));

    // Right (phase) axis when phase traces are visible.
    let has_phase = model
        .traces
        .iter()
        .any(|trace| trace.kind == TraceKind::PhaseDeg && trace.visible);
    if has_phase {
        if let Some((p0, p1)) = y_range(&mut state.shell.results.derived, model, true) {
            let axis = match view.y_right {
                // Zoomed: plain linear ticks — the 45° lattice would emit
                // hundreds of gridlines (or none) at arbitrary zoom depths.
                Some((z0, z1)) => Axis::linear_with(z0, z1, "°", 5),
                None => {
                    // Round phase bounds to 45° so ticks land on familiar angles.
                    let p0 = (p0 / 45.0).floor() * 45.0;
                    let p1 = (p1 / 45.0).ceil() * 45.0;
                    let ticks: Vec<f64> = (0..=((p1 - p0) / 45.0) as i64)
                        .map(|i| p0 + i as f64 * 45.0)
                        .collect();
                    Axis::with_ticks(p0, p1, "°", &ticks)
                }
            };
            spec.y_right = Some((axis, t.color.traces[2]));
        }
    }
    // 0 dB reference on log-magnitude strips.
    if model.y_unit == "dB" && y0 < 0.0 && y1 > 0.0 {
        spec.ref_lines.push(plot::RefLine { y: 0.0 });
    }

    // Run owns weight: overlay traces keep the signal hue at reduced alpha
    // and stroke, painted first so the active run draws at full strength
    // on top.
    let draw_order = model
        .traces
        .iter()
        .filter(|trace| trace.overlay)
        .chain(model.traces.iter().filter(|trace| !trace.overlay));
    for trace in draw_order {
        if !trace.visible {
            continue;
        }
        let color = if trace.overlay {
            trace.color.gamma_multiply(0.40)
        } else {
            trace.color
        };
        let mut plot_trace =
            Trace::new(&trace.x, &trace.y, color).cache_key(trace_key(model, trace));
        if trace.overlay {
            plot_trace = plot_trace.thin();
        }
        if trace.kind == TraceKind::PhaseDeg {
            plot_trace = plot_trace.right().dashed();
        }
        spec.traces.push(plot_trace);
    }
    for expr in &exprs {
        spec.traces.push(
            Trace::new(&expr.x, &expr.y, expr.color)
                .thin()
                .cache_key(expr.cache_key),
        );
    }

    let cursors = (state.shell.results.cursor_strip == Some(model.analysis_index))
        .then_some(state.shell.results.cursors);

    let readout = |x: f64| -> Vec<(String, String)> {
        let mut rows = vec![(model.x_label().to_owned(), model.format_x(x))];
        for trace in model.traces.iter().filter(|t| t.visible).take(6) {
            let value = sample_at(&trace.x, &trace.y, x);
            rows.push((trace.name.clone(), model.format_trace_value(trace, value)));
        }
        for expr in exprs.iter().take(3) {
            let value = sample_at(&expr.x, &expr.y, x);
            rows.push((expr.label.clone(), fmt_si(value, "", 3)));
        }
        rows
    };

    let response = plot::show(
        ui,
        &spec,
        &mut state.shell.results.cache,
        cursors.as_ref(),
        Some(&readout),
    );

    if let Some(clicked_x) = response.clicked_x {
        let results = &mut state.shell.results;
        if results.cursor_strip != Some(model.analysis_index) {
            results.cursors = CursorPair::default();
            results.cursor_strip = Some(model.analysis_index);
        }
        results.cursors.place(clicked_x);
    }

    if response.view.any() {
        state
            .shell
            .results
            .plot_view_mut(super::ResultViewer::Waves, model.analysis_index)
            .apply(&response.view);
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Cursors + measurements readout.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    if let Some(action) = section_header(ui, "Cursors", Some("Clear")) {
        if action.clicked() {
            state.shell.results.clear_cursors();
        }
    }

    let models = cached_models(&state.simulation, &mut state.shell.results, &t);
    let cursor_model = state
        .shell
        .results
        .cursor_strip
        .and_then(|idx| models.iter().find(|m| m.analysis_index == idx));

    let cursors = state.shell.results.cursors;
    match (cursor_model, cursors.a) {
        (Some(model), Some(a)) => {
            cursor_block(ui, "A", c.accent, &model.format_x(a), &value_rows(model, a));
            if let Some(b) = cursors.b {
                cursor_block(
                    ui,
                    "B",
                    c.traces[4],
                    &model.format_x(b),
                    &value_rows(model, b),
                );
                cursor_block(
                    ui,
                    "B − A",
                    c.text_faint,
                    &model.format_x(b - a),
                    &delta_rows(model, a, b),
                );
            }
        }
        _ => {
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                ui.label(
                    egui::RichText::new("Click a plot to place cursor A, again for B. Esc clears.")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(c.text_faint),
                );
            });
        }
    }

    // Measurements over the cursor window (or the full range).
    let measured_model = cursor_model.or_else(|| models.first());
    if let Some(model) = measured_model {
        let window = match (cursors.a, cursors.b) {
            (Some(a), Some(b)) => Some((a.min(b), a.max(b))),
            _ => None,
        };
        let title = if window.is_some() {
            "Measurements · A–B"
        } else {
            "Measurements"
        };
        section_header(ui, title, None);
        measurement_rows(ui, &mut state.shell.results.derived, model, window);
    }
}

fn value_rows(model: &StripModel, x: f64) -> Vec<(String, String)> {
    model
        .traces
        .iter()
        .filter(|trace| trace.visible)
        .take(6)
        .map(|trace| {
            let value = sample_at(&trace.x, &trace.y, x);
            (trace.name.clone(), model.format_trace_value(trace, value))
        })
        .collect()
}

fn delta_rows(model: &StripModel, a: f64, b: f64) -> Vec<(String, String)> {
    let mut rows = Vec::new();
    let dx = b - a;
    match model.x_scale {
        XScale::Linear if model.x_unit == "s" => {
            rows.push(("Δt".to_owned(), fmt_si(dx, "s", 3)));
            if dx != 0.0 {
                rows.push(("1/Δt".to_owned(), fmt_si(1.0 / dx.abs(), "Hz", 2)));
            }
        }
        XScale::Log10 => {
            rows.push(("Δf".to_owned(), fmt_si(dx, "Hz", 2)));
        }
        _ => rows.push(("Δx".to_owned(), fmt_si(dx, model.x_unit, 3))),
    }
    for trace in model.traces.iter().filter(|t| t.visible).take(4) {
        let dv = sample_at(&trace.x, &trace.y, b) - sample_at(&trace.x, &trace.y, a);
        rows.push((
            format!("Δ{}", trace.name),
            model.format_trace_value(trace, dv),
        ));
    }
    // dB/decade slope between cursors on log strips.
    if model.x_scale == XScale::Log10 && a > 0.0 && b > 0.0 {
        let dlog = (b.log10() - a.log10()).abs();
        if dlog > 1e-12 {
            if let Some(mag) = model
                .traces
                .iter()
                .find(|t| t.kind == TraceKind::MagnitudeDb && t.visible)
            {
                let ddb = sample_at(&mag.x, &mag.y, b) - sample_at(&mag.x, &mag.y, a);
                rows.push(("slope".to_owned(), format!("{:.1} dB/dec", ddb / dlog)));
            }
        }
    }
    rows
}

/// The design's `.cursor-block`: inset bordered block with a letter head and
/// mono key/value rows.
fn cursor_block(
    ui: &mut Ui,
    letter: &str,
    dot: egui::Color32,
    x_label: &str,
    rows: &[(String, String)],
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    egui::Frame::none()
        .fill(c.bg_inset)
        .stroke(egui::Stroke::new(1.0, c.border))
        .rounding(t.radius)
        .outer_margin(egui::Margin {
            left: 12.0,
            right: 12.0,
            top: 2.0,
            bottom: 6.0,
        })
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            let width = ui.available_width();
            // Head row.
            let (head, _) = ui.allocate_exact_size(egui::vec2(width, 24.0), egui::Sense::hover());
            let painter = ui.painter();
            painter.rect_filled(
                egui::Rect::from_center_size(
                    egui::pos2(head.left() + 13.0, head.center().y),
                    egui::vec2(8.0, 8.0),
                ),
                2.0,
                dot,
            );
            painter.text(
                egui::pos2(head.left() + 24.0, head.center().y),
                egui::Align2::LEFT_CENTER,
                letter,
                theme::mono(tokens::FS_0, FontWeight::Medium),
                c.text,
            );
            painter.text(
                egui::pos2(head.right() - 9.0, head.center().y),
                egui::Align2::RIGHT_CENTER,
                x_label,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                c.text_dim,
            );
            painter.hline(
                head.x_range(),
                head.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );
            // Value rows.
            for (key, value) in rows {
                let (row, _) =
                    ui.allocate_exact_size(egui::vec2(width, 18.0), egui::Sense::hover());
                let painter = ui.painter();
                painter.text(
                    egui::pos2(row.left() + 9.0, row.center().y),
                    egui::Align2::LEFT_CENTER,
                    key,
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    c.text_dim,
                );
                painter.text(
                    egui::pos2(row.right() - 9.0, row.center().y),
                    egui::Align2::RIGHT_CENTER,
                    value,
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    c.text,
                );
            }
            ui.add_space(5.0);
        });
}

/// min / max / rms rows per visible trace, optionally windowed to [a, b].
/// The single-pass stats are cached per (trace, window, data version), so
/// no samples are rescanned until the cursors or the data move.
fn measurement_rows(
    ui: &mut Ui,
    derived: &mut DerivedSeries,
    model: &StripModel,
    window: Option<(f64, f64)>,
) {
    use crate::waveform::measurements as basic;

    // Window identity for the cache key; u64::MAX is a NaN bit pattern no
    // finite cursor can produce, marking the full-range case.
    let (a_bits, b_bits) = match window {
        Some((a, b)) => (a.to_bits(), b.to_bits()),
        None => (u64::MAX, u64::MAX),
    };

    let mut rows: Vec<(String, String)> = Vec::new();
    for trace in model.traces.iter().filter(|t| t.visible).take(4) {
        let key = (trace_key(model, trace), a_bits, b_bits);
        let stats = derived.stats_or(key, || {
            let (start, end) = match window {
                Some((a, b)) => {
                    let start = trace.x.partition_point(|&v| v < a);
                    let end = trace.x.partition_point(|&v| v <= b);
                    (start, end.max(start))
                }
                None => (0, trace.y.len()),
            };
            basic::calculate_min_max_rms(&trace.y[start..end])
        });
        let Some((min, max, rms)) = stats else {
            continue;
        };
        let fmt = |v: f64| -> String {
            match trace.kind {
                TraceKind::Value => fmt_si(v, model.y_unit, 3),
                TraceKind::MagnitudeDb => format!("{v:.1} dB"),
                TraceKind::PhaseDeg => format!("{v:.1} °"),
            }
        };
        rows.push((format!("{} min", trace.name), fmt(min)));
        rows.push((format!("{} max", trace.name), fmt(max)));
        if trace.kind == TraceKind::Value {
            rows.push((format!("{} rms", trace.name), fmt(rms)));
        }
    }
    let refs: Vec<(&str, &str)> = rows.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    crate::ui::widgets::measurement_table(ui, &refs);
}
