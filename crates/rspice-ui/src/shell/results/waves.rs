//! WAVES — stacked waveform strips, one per analysis in the active run.
//!
//! Each strip carries its analysis' traces with the strip grammar (header ·
//! legend · actions over a document well). AC strips convert magnitude to dB
//! on the left axis and route phase traces, dashed, to a right axis. A/B
//! cursors live on one strip at a time; their values, deltas and windowed
//! measurements render in the right panel.

use std::sync::Arc;

use egui::Ui;

use crate::common::AppState;
use crate::state::{AnalysisType, SimulationState};
use crate::ui::plot::{
    self, Axis, CursorPair, PlotSpec, Trace, XScale, fmt_si, sample_at,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::section_header;

use super::strip::{self, LegendChip};
use super::{DerivedSeries, waveform_color, well_hint};

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
}

/// One strip (== one analysis of the active run).
pub(super) struct StripModel {
    analysis_index: usize,
    kind_tag: String,
    subtitle: String,
    x_scale: XScale,
    x_unit: &'static str,
    y_unit: &'static str,
    traces: Vec<StripTrace>,
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
        fmt_si(x, if self.x_unit.is_empty() { "" } else { self.x_unit }, 3)
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
pub(super) fn build_models(
    simulation: &SimulationState,
    derived: &mut DerivedSeries,
    tokens: &Tokens,
) -> Vec<StripModel> {
    let Some(run) = simulation.active_run() else {
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
            });
        }

        models.push(StripModel {
            analysis_index,
            kind_tag: analysis.analysis_type.short_label().to_uppercase(),
            subtitle: analysis.label.clone(),
            x_scale,
            x_unit,
            y_unit,
            traces,
        });
    }
    models
}

/// Stable per-trace identity shared by the decimation, range, and
/// measurement caches.
fn trace_key(analysis_index: usize, trace: &StripTrace) -> u64 {
    (analysis_index as u64) << 40 | (trace.waveform_index as u64) << 2 | trace.kind as u64
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
        let extremes = derived.range_or(trace_key(model.analysis_index, trace), || {
            super::finite_extremes(&trace.y)
        });
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
    let models = build_models(
        &state.simulation,
        &mut state.shell.results.derived,
        &t,
    );
    if models.is_empty() {
        well_hint(ui, "The active run has no plottable analyses");
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
                        let legend: Vec<LegendChip<'_>> = model
                            .traces
                            .iter()
                            .map(|trace| LegendChip {
                                name: &trace.name,
                                color: trace.color,
                                on: trace.visible,
                            })
                            .collect();
                        let header = strip::header(
                            ui,
                            &model.kind_tag,
                            &model.subtitle,
                            &legend,
                            maximized,
                            !maximized && n > 1,
                        );
                        if let Some(chip_index) = header.legend_clicked {
                            if let Some(trace) = model.traces.get(chip_index) {
                                toggle_trace = Some((model.analysis_index, trace.waveform_index));
                            }
                        }
                        if header.maximize_clicked {
                            toggle_maximize = Some(model.analysis_index);
                        }
                        if header.close_clicked {
                            close_strip = Some(model.analysis_index);
                        }

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
}

/// Flip a waveform's visibility on the run, keeping the live copy in sync.
pub(crate) fn toggle_visibility(state: &mut AppState, analysis_index: usize, waveform_index: usize) {
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
    let Some((y0, y1)) = y_range(&mut state.shell.results.derived, model, false) else {
        well_hint(ui, "No visible traces — enable one in the legend");
        return;
    };

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
            // Round phase bounds to 45° so ticks land on familiar angles.
            let p0 = (p0 / 45.0).floor() * 45.0;
            let p1 = (p1 / 45.0).ceil() * 45.0;
            let ticks: Vec<f64> = (0..=((p1 - p0) / 45.0) as i64)
                .map(|i| p0 + i as f64 * 45.0)
                .collect();
            spec.y_right = Some((
                Axis::with_ticks(p0, p1, "°", &ticks),
                t.color.traces[2],
            ));
        }
    }
    // 0 dB reference on log-magnitude strips.
    if model.y_unit == "dB" && y0 < 0.0 && y1 > 0.0 {
        spec.ref_lines.push(plot::RefLine { y: 0.0 });
    }

    for trace in &model.traces {
        if !trace.visible {
            continue;
        }
        let mut plot_trace = Trace::new(&trace.x, &trace.y, trace.color)
            .cache_key(trace_key(model.analysis_index, trace));
        if trace.kind == TraceKind::PhaseDeg {
            plot_trace = plot_trace.right().dashed();
        }
        spec.traces.push(plot_trace);
    }

    let cursors = (state.shell.results.cursor_strip == Some(model.analysis_index))
        .then_some(state.shell.results.cursors);

    let readout = |x: f64| -> Vec<(String, String)> {
        let mut rows = vec![(model.x_label().to_owned(), model.format_x(x))];
        for trace in model.traces.iter().filter(|t| t.visible).take(6) {
            let value = sample_at(&trace.x, &trace.y, x);
            rows.push((trace.name.clone(), model.format_trace_value(trace, value)));
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

    let models = build_models(&state.simulation, &mut state.shell.results.derived, &t);
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
                cursor_block(ui, "B", c.traces[4], &model.format_x(b), &value_rows(model, b));
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
            let (head, _) =
                ui.allocate_exact_size(egui::vec2(width, 24.0), egui::Sense::hover());
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
        let key = (trace_key(model.analysis_index, trace), a_bits, b_bits);
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
    let refs: Vec<(&str, &str)> = rows
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    crate::ui::widgets::measurement_table(ui, &refs);
}
