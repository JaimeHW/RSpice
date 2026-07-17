//! TUNER — the live parameter loop in the right panel.
//!
//! Rows bind to the buffer's `.param name=value` lines: a log slider, the
//! engineering-notation readout, and a diff pip on the touched line until
//! the next run baselines it. "live" re-simulates on every slider change
//! (queued behind the running engine), "on release" on commit. Values are
//! written back in SPICE notation (`Meg` = 1e6, `m` = 1e-3).

use std::collections::HashMap;

use egui::{Color32, Ui};

use crate::common::AppState;
use crate::properties::engineering::{format_engineering_value, parse_engineering_value};
use crate::ui::plot::{self, Axis, PlotSpec, Trace, XScale};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{chip, section_header};

const MINI_BODE_GAIN_CACHE_KEY: u64 = 0x4E45_544C_0001;
const MINI_BODE_PHASE_CACHE_KEY: u64 = 0x4E45_544C_0002;

/// One tunable row: a `.param` assignment found in the buffer.
#[derive(Clone)]
struct ParamRow {
    name: String,
    /// 0-based buffer line carrying the assignment.
    line: usize,
    /// Parsed value; `None` for `{expression}` bindings (not sliderable).
    value: Option<f64>,
    /// Raw value text, for the expression readout.
    raw: String,
    /// Optional exact slider range from an `@tune` annotation.
    range: Option<(f64, f64)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TunedMetricRow {
    name: &'static str,
    value: String,
    highlight: bool,
}

/// Render the tuner right panel (Netlist workspace).
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let editable = state.ui.netlist.active_document == super::ActiveNetlistDocument::OwnedSource
        && state.workspace.has_editable_netlist_source();

    section_header(ui, "Tuner", None);
    let rows = scan_params(&state.simulation.netlist_content);
    let reset_payload = reset_values(&rows, &state.ui.netlist.last_run_params);
    let mut reset_requested = false;

    // Mode toggle: live re-sim vs on-release.
    ui.add_enabled_ui(editable, |ui| {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            let live = state.ui.netlist.tuner_live;
            if chip(ui, "live", live)
                .on_hover_text("Re-simulate on every slider change")
                .clicked()
            {
                state.ui.netlist.tuner_live = true;
            }
            if chip(ui, "on release", !live)
                .on_hover_text("Re-simulate when the drag commits")
                .clicked()
            {
                state.ui.netlist.tuner_live = false;
            }
            if !reset_payload.is_empty()
                && chip(ui, "reset", false)
                    .on_hover_text("Reset numeric .param values to the last successful run")
                    .clicked()
            {
                reset_requested = true;
            }
        });
    });
    ui.add_space(6.0);

    if let Some(summary) = super::summary::active_run_summary(state) {
        render_mini_bode(ui, state, &summary);
        render_as_tuned(
            ui,
            &summary,
            state.ui.preferences.quantity_presentation_policy(),
        );
        ui.add_space(8.0);
    }

    if rows.is_empty() {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("No .param lines — add `.param name=value` to tune")
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.text_faint),
            );
        });
        return;
    }
    if reset_requested {
        apply_reset_to_last_run(ui, state, &rows);
        return;
    }

    // Slider edits collected first; the buffer rewrite happens after the
    // row loop so the scan stays consistent within the frame.
    let mut write: Option<(ParamRow, f64, bool)> = None;

    for row in rows {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(&row.name)
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(c.text),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                let readout = match row.value {
                    Some(value) => format_engineering_value(value),
                    None => row.raw.clone(),
                };
                let color = if row.value.is_some() {
                    c.traces[1]
                } else {
                    c.text_faint
                };
                ui.label(
                    egui::RichText::new(readout)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(color),
                );
            });
        });

        let Some(value) = row.value else {
            continue; // expression-bound: readout only
        };

        let (lo, hi) = slider_range(state, &row.name, value, row.range);
        let mut slider_value = value;
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.spacing_mut().slider_width = slider_width_for_available(ui.available_width());
            let logarithmic = lo > 0.0;
            let response = ui.add_enabled(
                editable,
                egui::Slider::new(&mut slider_value, lo..=hi)
                    .logarithmic(logarithmic)
                    .show_value(false),
            );
            if response.changed() && slider_value != value {
                let fire = state.ui.netlist.tuner_live;
                write = Some((row, slider_value, fire));
            } else if response.drag_stopped() && !state.ui.netlist.tuner_live {
                write = Some((row, slider_value, true));
            }
        });
    }

    if let Some((row, new_value, fire_run)) = write {
        apply_param_edit(ui, state, &row, new_value, fire_run);
    }
}

fn slider_width_for_available(available: f32) -> f32 {
    (available - 24.0).clamp(80.0, 220.0)
}

fn render_mini_bode(
    ui: &mut Ui,
    state: &mut AppState,
    summary: &super::summary::NetlistRunSummary,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let Some(spec) = mini_bode_spec(summary, c.traces[0], c.traces[2], quantity_policy) else {
        return;
    };

    ui.add_space(4.0);
    state
        .ui
        .results
        .cache
        .ensure_version(state.simulation.data_version);
    ui.allocate_ui(egui::vec2(ui.available_width(), 130.0), |ui| {
        plot::show(ui, &spec, &mut state.ui.results.cache, None, None);
    });
    ui.add_space(4.0);
}

fn render_as_tuned(
    ui: &mut Ui,
    summary: &super::summary::NetlistRunSummary,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) {
    section_header(ui, "As tuned", None);
    let t = Tokens::get(ui.ctx());
    for row in as_tuned_rows(summary, quantity_policy) {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(row.name)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(row.value)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(if row.highlight {
                            t.color.accent
                        } else {
                            t.color.text
                        }),
                );
            });
        });
    }
}

fn as_tuned_rows(
    summary: &super::summary::NetlistRunSummary,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) -> [TunedMetricRow; 3] {
    [
        TunedMetricRow {
            name: "UGF",
            value: fmt_opt(summary.stability.ugf, |value| {
                quantity_policy.format_frequency(value, 2)
            }),
            highlight: true,
        },
        TunedMetricRow {
            name: "PM",
            value: fmt_opt(summary.stability.pm_deg, |value| {
                quantity_policy.format_angle(value.to_radians(), 1)
            }),
            highlight: true,
        },
        TunedMetricRow {
            name: "GM",
            value: fmt_opt(summary.stability.gm_db, |value| format!("{value:.1} dB")),
            highlight: false,
        },
    ]
}

fn fmt_opt(value: Option<f64>, f: impl FnOnce(f64) -> String) -> String {
    value.map(f).unwrap_or_else(|| "-".to_string())
}

fn mini_bode_spec<'a>(
    summary: &'a super::summary::NetlistRunSummary,
    gain_color: Color32,
    phase_color: Color32,
    quantity_policy: crate::quantity::QuantityPresentationPolicy,
) -> Option<PlotSpec<'a>> {
    let bode = summary.bode.as_ref()?;
    let x0 = bode
        .frequency
        .iter()
        .copied()
        .find(|value| value.is_finite() && *value > 0.0)?;
    let x1 = bode
        .frequency
        .iter()
        .copied()
        .rev()
        .find(|value| value.is_finite() && *value > x0)?;
    if !matches!(x1.partial_cmp(&x0), Some(std::cmp::Ordering::Greater)) {
        return None;
    }

    let (g_min, g_max) =
        finite_pair(summary.stability.gain_extremes).or_else(|| finite_extremes(&bode.gain_db))?;
    let gain_pad = ((g_max - g_min) * 0.1).max(3.0);
    let y0 = (g_min - gain_pad).min(-10.0);
    let y1 = (g_max + gain_pad).max(10.0);
    if !matches!(y1.partial_cmp(&y0), Some(std::cmp::Ordering::Greater)) {
        return None;
    }

    let (frequency_scale, frequency_offset, frequency_unit) =
        quantity_policy.frequency_axis_transform();
    let frequency_axis = Axis::log_decades(x0, x1, "Hz").with_display_transform(
        frequency_scale,
        frequency_offset,
        frequency_unit,
    );
    let mut spec = PlotSpec::new(
        frequency_axis,
        XScale::Log10,
        Axis::linear_with(y0, y1, "dB", 4),
    )
    .accessible_name("Tuning response preview");
    spec.left_margin = 46.0;
    spec.ref_lines.push(plot::RefLine { y: 0.0 });

    if let Some(phase) = &bode.phase_deg {
        let (p_min, p_max) = summary
            .stability
            .phase_extremes
            .and_then(finite_pair)
            .or_else(|| finite_extremes(phase))
            .unwrap_or((-180.0, 0.0));
        let p0 = p_min.min(-180.0);
        let p1 = p_max.max(0.0);
        if matches!(p1.partial_cmp(&p0), Some(std::cmp::Ordering::Greater)) {
            let (angle_scale, angle_offset, angle_unit) = quantity_policy.degree_axis_transform();
            let phase_axis = Axis::linear_with(p0, p1, "deg", 3).with_display_transform(
                angle_scale,
                angle_offset,
                angle_unit,
            );
            spec.y_right = Some((phase_axis, phase_color));
            spec.traces.push(
                Trace::new(&bode.frequency, phase, phase_color)
                    .right()
                    .dashed()
                    .thin()
                    .cache_key(MINI_BODE_PHASE_CACHE_KEY),
            );
        }
    }

    spec.traces.push(
        Trace::new(&bode.frequency, &bode.gain_db, gain_color)
            .thin()
            .cache_key(MINI_BODE_GAIN_CACHE_KEY),
    );

    Some(spec)
}

fn finite_pair((lo, hi): (f64, f64)) -> Option<(f64, f64)> {
    (lo.is_finite() && hi.is_finite() && lo <= hi).then_some((lo, hi))
}

fn finite_extremes(values: &[f64]) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for &value in values {
        if value.is_finite() {
            lo = lo.min(value);
            hi = hi.max(value);
        }
    }
    (lo <= hi).then_some((lo, hi))
}

/// Slider bounds for one parameter, captured the first time the parameter
/// appears so the range doesn't chase the value mid-drag. Positive values
/// get a log decade either side; others a symmetric linear span.
fn slider_range(
    state: &mut AppState,
    name: &str,
    value: f64,
    annotated: Option<(f64, f64)>,
) -> (f64, f64) {
    if let Some(range) = annotated {
        state
            .ui
            .netlist
            .param_ranges
            .insert(name.to_ascii_lowercase(), range);
        return range;
    }

    *state
        .ui
        .netlist
        .param_ranges
        .entry(name.to_ascii_lowercase())
        .or_insert_with(|| {
            if value > 0.0 {
                (value / 10.0, value * 10.0)
            } else if value < 0.0 {
                (value * 10.0, value / 10.0)
            } else {
                (0.0, 1.0)
            }
        })
}

/// Rewrite the parameter's value in the buffer and mark the line edited.
/// The run request fires even when the value is already written — the
/// on-release commit lands on a frame where the drag updated nothing.
fn apply_param_edit(ui: &Ui, state: &mut AppState, row: &ParamRow, value: f64, fire_run: bool) {
    if state.ui.netlist.active_document != super::ActiveNetlistDocument::OwnedSource
        || !state.workspace.has_editable_netlist_source()
    {
        return;
    }

    let formatted = format_engineering_value(value);
    let buffer = &state.simulation.netlist_content;

    // Locate the value span inside the row's line, fresh from the buffer.
    let span = line_at(buffer, row.line).and_then(|(line_start, line)| {
        value_span(line, &row.name).map(|(start, end)| {
            (
                line_start + start,
                line_start + end,
                &line[start..end] != formatted.as_str(),
            )
        })
    });

    if let Some((start, end, differs)) = span
        && differs
    {
        let mut next = String::with_capacity(buffer.len() + 8);
        next.push_str(&buffer[..start]);
        next.push_str(&formatted);
        next.push_str(&buffer[end..]);

        let replaced = super::replace_owned_source(state, next);
        debug_assert!(replaced, "an editable source changed before tuner commit");
        if !replaced {
            return;
        }
        let netlist = &mut state.ui.netlist;
        netlist.last_edit_time = ui.input(|input| input.time);
        netlist.edited_lines.insert(row.line);
        super::refresh_diff_pips_from_baseline(state);
    }

    if fire_run {
        super::request_run(state);
    }
}

/// Byte offset and slice of the 0-based line `index`.
fn line_at(buffer: &str, index: usize) -> Option<(usize, &str)> {
    let mut offset = 0usize;
    for (idx, line) in buffer.split('\n').enumerate() {
        if idx == index {
            return Some((offset, line));
        }
        offset += line.len() + 1;
    }
    None
}

/// The byte span of `name`'s assigned value within a `.param` line.
fn value_span(line: &str, name: &str) -> Option<(usize, usize)> {
    let assignments = scan_assignments(line)?;
    assignments
        .into_iter()
        .find(|(n, _, _)| n.eq_ignore_ascii_case(name))
        .map(|(_, start, end)| (start, end))
}

/// Every `.param` assignment in the buffer, for completion: the spans are
/// line-local and only the names matter to callers.
pub(super) fn buffer_assignments(buffer: &str) -> Vec<(String, usize, usize)> {
    buffer
        .lines()
        .filter_map(scan_assignments)
        .flatten()
        .collect()
}

/// All `.param` rows in the buffer.
fn scan_params(buffer: &str) -> Vec<ParamRow> {
    let mut rows = Vec::new();
    let mut named_ranges: HashMap<String, (f64, f64)> = HashMap::new();
    let mut next_range: Option<(f64, f64)> = None;
    for (idx, line) in buffer.lines().enumerate() {
        if let Some(annotation) = tune_annotation(line) {
            match annotation {
                TuneAnnotation::Named { name, range } => {
                    named_ranges.insert(name.to_ascii_lowercase(), range);
                }
                TuneAnnotation::Next(range) => {
                    next_range = Some(range);
                }
            }
            continue;
        }

        let Some(assignments) = scan_assignments(line) else {
            continue;
        };
        for (name, start, end) in assignments {
            let raw = line[start..end].to_owned();
            let value = if raw.starts_with('{') {
                None
            } else {
                parse_engineering_value(&raw).ok()
            };
            let range = named_ranges
                .get(&name.to_ascii_lowercase())
                .copied()
                .or_else(|| next_range.take());
            rows.push(ParamRow {
                name,
                line: idx,
                value,
                raw,
                range,
            });
        }
    }
    rows
}

enum TuneAnnotation {
    Named { name: String, range: (f64, f64) },
    Next((f64, f64)),
}

fn tune_annotation(line: &str) -> Option<TuneAnnotation> {
    let trimmed = line.trim_start();
    let comment = trimmed.strip_prefix('*')?.trim_start();
    let rest = comment.strip_prefix("@tune")?.trim();
    let mut parts = rest.split_whitespace();
    let first = parts.next()?;
    if first.contains("..") {
        return parse_tune_range(first).map(TuneAnnotation::Next);
    }

    let range = parse_tune_range(parts.next()?)?;
    Some(TuneAnnotation::Named {
        name: first.to_string(),
        range,
    })
}

fn parse_tune_range(raw: &str) -> Option<(f64, f64)> {
    let (lo, hi) = raw.split_once("..")?;
    let lo = parse_engineering_value(lo).ok()?;
    let hi = parse_engineering_value(hi).ok()?;
    if lo <= hi {
        Some((lo, hi))
    } else {
        Some((hi, lo))
    }
}

fn reset_values(rows: &[ParamRow], baseline: &HashMap<String, f64>) -> Vec<(String, f64)> {
    rows.iter()
        .filter_map(|row| {
            let current = row.value?;
            let baseline_value = *baseline.get(&row.name.to_ascii_lowercase())?;
            let scale = current.abs().max(baseline_value.abs()).max(1.0);
            if (current - baseline_value).abs() <= f64::EPSILON * scale {
                return None;
            }
            Some((row.name.clone(), baseline_value))
        })
        .collect()
}

fn apply_reset_to_last_run(ui: &Ui, state: &mut AppState, rows: &[ParamRow]) {
    let baseline = state.ui.netlist.last_run_params.clone();
    let mut changed = false;
    for row in rows {
        let Some(value) = baseline.get(&row.name.to_ascii_lowercase()).copied() else {
            continue;
        };
        let before = state.simulation.netlist_content.clone();
        apply_param_edit(ui, state, row, value, false);
        changed |= before != state.simulation.netlist_content;
    }

    if changed {
        super::request_run(state);
    }
}

/// Parse a `.param` line into `(name, value_start, value_end)` triples
/// (byte offsets into `line`). Returns `None` for non-`.param` lines.
pub(super) fn scan_assignments(line: &str) -> Option<Vec<(String, usize, usize)>> {
    let trimmed = line.trim_start();
    let prefix_len = line.len() - trimmed.len();
    let lower = trimmed.to_ascii_lowercase();
    if !(lower.starts_with(".param") || lower.starts_with(".parameter")) {
        return None;
    }
    let after_cmd = trimmed.find(char::is_whitespace)?;
    let mut out = Vec::new();
    let bytes = trimmed.as_bytes();
    let mut i = after_cmd;

    while i < bytes.len() {
        // Skip whitespace.
        while i < bytes.len() && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] == b';' || bytes[i] == b'$' {
            break;
        }
        // Name token.
        let name_start = i;
        while i < bytes.len() {
            let ch = bytes[i] as char;
            if ch.is_whitespace() || ch == '=' {
                break;
            }
            i += 1;
        }
        let name = trimmed[name_start..i].to_owned();
        // Skip to '='.
        while i < bytes.len() && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] != b'=' {
            break; // malformed tail; the parser diagnostics own this
        }
        i += 1;
        while i < bytes.len() && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        // Value: `{balanced expression}` or a bare token.
        let value_start = i;
        if bytes[i] == b'{' {
            let mut depth = 0i32;
            while i < bytes.len() {
                match bytes[i] {
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            i += 1;
                            break;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
        } else {
            while i < bytes.len() && !(bytes[i] as char).is_whitespace() {
                i += 1;
            }
        }
        out.push((name, prefix_len + value_start, prefix_len + i));
    }

    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;

    use egui::Color32;

    use super::super::summary;
    use crate::state::{AcBodeMetrics, AcBodeSummary};

    #[test]
    fn scans_multiple_assignments_with_spans() {
        let line = "  .param itail=20u  cl = 1.5p vdd={supply*2}";
        let rows = scan_assignments(line).expect("a .param line");
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].0, "itail");
        assert_eq!(&line[rows[0].1..rows[0].2], "20u");
        assert_eq!(rows[1].0, "cl");
        assert_eq!(&line[rows[1].1..rows[1].2], "1.5p");
        assert_eq!(rows[2].0, "vdd");
        assert_eq!(&line[rows[2].1..rows[2].2], "{supply*2}");
    }

    #[test]
    fn non_param_lines_are_ignored() {
        assert!(scan_assignments("R1 in out 4.7k").is_none());
        assert!(scan_assignments("* .param comment=1").is_none());
    }

    #[test]
    fn value_span_targets_the_named_parameter() {
        let line = ".param a=1 b=2";
        let span = value_span(line, "B").expect("case-insensitive match");
        assert_eq!(&line[span.0..span.1], "2");
    }

    #[test]
    fn tune_annotation_by_name_sets_exact_range() {
        let rows = scan_params("* @tune itail 5u..60u\n.param itail=20u\n");
        let (lo, hi) = rows[0].range.expect("range");
        assert!((lo - 5e-6).abs() < 1e-18);
        assert!((hi - 60e-6).abs() < 1e-17);
    }

    #[test]
    fn tune_annotation_before_param_targets_next_assignment() {
        let rows = scan_params("* @tune 0.5p..8p\n.param cl=2p\n");
        let (lo, hi) = rows[0].range.expect("range");
        assert!((lo - 0.5e-12).abs() < 1e-24);
        assert!((hi - 8e-12).abs() < 1e-23);
    }

    #[test]
    fn reset_payload_uses_last_run_values_only_for_numeric_params() {
        let rows = scan_params(".param a=2 b={expr}\n");
        let mut baseline = std::collections::HashMap::new();
        baseline.insert("a".to_string(), 1.0);
        baseline.insert("b".to_string(), 3.0);

        assert_eq!(reset_values(&rows, &baseline), vec![("a".to_string(), 1.0)]);
    }

    #[test]
    fn slider_width_is_bounded_for_compact_panels() {
        assert_eq!(slider_width_for_available(40.0), 80.0);
        assert_eq!(slider_width_for_available(144.0), 120.0);
        assert_eq!(slider_width_for_available(600.0), 220.0);
    }

    #[test]
    fn as_tuned_rows_format_present_and_missing_stability_metrics() {
        let summary = summary::NetlistRunSummary {
            stability: AcBodeMetrics {
                ugf: Some(10.0),
                pm_deg: Some(45.25),
                gm_db: None,
                ..Default::default()
            },
            bode: None,
            measurements: HashMap::new(),
        };

        let rows = as_tuned_rows(
            &summary,
            crate::quantity::QuantityPresentationPolicy::default(),
        );

        assert_eq!(
            rows,
            [
                TunedMetricRow {
                    name: "UGF",
                    value: "10.00 Hz".to_string(),
                    highlight: true,
                },
                TunedMetricRow {
                    name: "PM",
                    value: "45.2 °".to_string(),
                    highlight: true,
                },
                TunedMetricRow {
                    name: "GM",
                    value: "-".to_string(),
                    highlight: false,
                },
            ]
        );
    }

    #[test]
    fn mini_bode_spec_uses_active_ac_curves_and_dual_axis() {
        let summary = summary::NetlistRunSummary {
            stability: AcBodeMetrics {
                gain_extremes: (-40.0, 20.0),
                phase_extremes: Some((-225.0, -45.0)),
                ..Default::default()
            },
            bode: Some(AcBodeSummary {
                signal: "V(out)".to_string(),
                frequency: Arc::new(vec![1.0, 10.0, 100.0, 1000.0]),
                gain_db: Arc::new(vec![20.0, 0.0, -20.0, -40.0]),
                phase_deg: Some(Arc::new(vec![-45.0, -135.0, -180.0, -225.0])),
                metrics: AcBodeMetrics::default(),
                analysis_index: 2,
                mag_index: 3,
                phase_index: Some(4),
            }),
            measurements: HashMap::new(),
        };

        let spec = mini_bode_spec(
            &summary,
            Color32::RED,
            Color32::BLUE,
            crate::quantity::QuantityPresentationPolicy::default(),
        )
        .expect("mini bode spec");

        assert_eq!(spec.x.unit, "Hz");
        assert_eq!(spec.x.min, 1.0);
        assert_eq!(spec.x.max, 1000.0);
        assert_eq!(spec.x_scale, crate::ui::plot::XScale::Log10);
        assert_eq!(spec.y.unit, "dB");
        assert!(spec.y.min <= -40.0);
        assert!(spec.y.max >= 20.0);
        assert!(spec.y_right.is_some());
        assert_eq!(spec.ref_lines.len(), 1);
        assert_eq!(spec.traces.len(), 2);
        assert_eq!(spec.traces[0].side, crate::ui::plot::YSide::Right);
        assert!(spec.traces[0].dashed);
        assert_eq!(spec.traces[1].side, crate::ui::plot::YSide::Left);
    }

    #[test]
    fn tuner_cannot_mutate_or_promote_generated_output() {
        let mut state = AppState::default();
        state.simulation.netlist_content = ".param gain=1\n.end\n".to_owned();
        let row = scan_params(&state.simulation.netlist_content)
            .into_iter()
            .next()
            .expect("numeric parameter");
        let ctx = egui::Context::default();

        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                apply_param_edit(ui, &mut state, &row, 2.0, false);
            });
        });

        assert_eq!(state.simulation.netlist_content, ".param gain=1\n.end\n");
        assert!(state.workspace.netlist_source.is_none());
        assert!(!state.workspace.netlist_source_dirty);
        assert_eq!(state.ui.netlist.revision, 0);
    }

    #[test]
    fn tuner_updates_and_dirties_an_owned_source() {
        let mut state = AppState::default();
        state.simulation.netlist_content = ".param gain=1\n.end\n".to_owned();
        state.workspace.netlist_source = Some(state.simulation.netlist_content.clone());
        state.ui.netlist.active_document = super::super::ActiveNetlistDocument::OwnedSource;
        state.workspace.netlist_source_path = Some(std::path::PathBuf::from("decks/owned.cir"));
        let row = scan_params(&state.simulation.netlist_content)
            .into_iter()
            .next()
            .expect("numeric parameter");
        let ctx = egui::Context::default();

        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                apply_param_edit(ui, &mut state, &row, 2.0, false);
            });
        });

        assert_eq!(state.simulation.netlist_content, ".param gain=2\n.end\n");
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some(".param gain=2\n.end\n")
        );
        assert_eq!(
            state.workspace.netlist_source_path.as_deref(),
            Some(std::path::Path::new("decks/owned.cir"))
        );
        assert!(state.workspace.netlist_source_dirty);
        assert_eq!(state.ui.netlist.revision, 1);
        assert!(state.ui.netlist.edited_lines.contains(&0));
    }
}
