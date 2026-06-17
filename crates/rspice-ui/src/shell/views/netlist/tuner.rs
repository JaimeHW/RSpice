//! TUNER — the live parameter loop in the right panel.
//!
//! Rows bind to the buffer's `.param name=value` lines: a log slider, the
//! engineering-notation readout, and a diff pip on the touched line until
//! the next run baselines it. "live" re-simulates on every slider change
//! (queued behind the running engine), "on release" on commit. Values are
//! written back in SPICE notation (`Meg` = 1e6, `m` = 1e-3).

use std::collections::HashMap;

use egui::Ui;

use crate::common::AppState;
use crate::properties::engineering::{format_engineering_value, parse_engineering_value};
use crate::ui::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{chip, section_header};

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
    /// Optional explicit slider range from a preceding `* @tune` annotation.
    range: Option<(f64, f64)>,
    /// Last successful run's numeric value for this parameter.
    baseline: Option<f64>,
}

/// Render the tuner right panel (Netlist workspace).
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    section_header(ui, "Tuner", None);

    // Mode toggle: live re-sim vs on-release.
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        let live = state.shell.netlist.tuner_live;
        if chip(ui, "live", live)
            .on_hover_text("Re-simulate on every slider change")
            .clicked()
        {
            state.shell.netlist.tuner_live = true;
        }
        if chip(ui, "on release", !live)
            .on_hover_text("Re-simulate when the drag commits")
            .clicked()
        {
            state.shell.netlist.tuner_live = false;
        }
    });
    ui.add_space(6.0);

    let mut rows = scan_params(&state.simulation.netlist_content);
    for row in &mut rows {
        row.baseline = state
            .shell
            .netlist
            .last_run_params
            .get(&row.name.to_ascii_lowercase())
            .copied();
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

    // Slider edits collected first; the buffer rewrite happens after the
    // row loop so the scan stays consistent within the frame.
    let mut write: Option<(ParamRow, f64, bool)> = None;

    for row in &rows {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(&row.name)
                    .font(theme::mono(tokens::FS_1, FontWeight::Medium))
                    .color(c.text),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                let readout = match row.value {
                    Some(value) => format_engineering_value(value),
                    None => row.raw.clone(),
                };
                let color = if row.value.is_some() {
                    match (row.value, row.baseline) {
                        (Some(value), Some(baseline))
                            if (value - baseline).abs() > f64::EPSILON =>
                        {
                            c.accent
                        }
                        _ => c.traces[1],
                    }
                } else {
                    c.text_faint
                };
                ui.label(
                    egui::RichText::new(readout)
                        .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                        .color(color),
                );
            });
        });

        let Some(value) = row.value else {
            continue; // expression-bound: readout only
        };

        let (lo, hi) = row
            .range
            .unwrap_or_else(|| slider_range(state, &row.name, value));
        let mut slider_value = value;
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.spacing_mut().slider_width = (ui.available_width() - 24.0).clamp(80.0, 220.0);
            let logarithmic = lo > 0.0;
            let response = ui.add(
                egui::Slider::new(&mut slider_value, lo..=hi)
                    .logarithmic(logarithmic)
                    .show_value(false),
            );
            if response.changed() && slider_value != value {
                let fire = state.shell.netlist.tuner_live;
                write = Some((row.clone(), slider_value, fire));
            } else if response.drag_stopped() && !state.shell.netlist.tuner_live {
                write = Some((row.clone(), slider_value, true));
            }
        });
    }

    if let Some((row, new_value, fire_run)) = write {
        apply_param_edit(ui, state, &row, new_value, fire_run);
    }

    ui.add_space(8.0);
    let reset_clicked = crate::ui::widgets::Button::new("reset to last run")
        .enabled(!state.shell.netlist.last_run_params.is_empty())
        .show(ui)
        .clicked();
    if reset_clicked {
        let values = reset_values(&rows, &state.shell.netlist.last_run_params);
        let changed = !values.is_empty();
        for (name, value) in values {
            if let Some(row) = rows.iter().find(|row| row.name.eq_ignore_ascii_case(&name)) {
                apply_param_edit(ui, state, row, value, false);
            }
        }
        if changed {
            super::request_run(state);
        }
    }

    if let Some(summary) = super::summary::active_run_summary(state) {
        render_mini_bode(ui, state, &summary);
        render_as_tuned(ui, &summary);
    }
}

/// Slider bounds for one parameter, captured the first time the parameter
/// appears so the range doesn't chase the value mid-drag. Positive values
/// get a log decade either side; others a symmetric linear span.
fn slider_range(state: &mut AppState, name: &str, value: f64) -> (f64, f64) {
    *state
        .shell
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

        state.simulation.netlist_content = next.clone();
        state.workspace.netlist_source = Some(next);

        let netlist = &mut state.shell.netlist;
        netlist.revision += 1;
        netlist.last_edit_time = ui.input(|input| input.time);
        netlist.edited_lines.insert(row.line);
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

fn reset_values(rows: &[ParamRow], baseline: &HashMap<String, f64>) -> Vec<(String, f64)> {
    rows.iter()
        .filter(|row| row.value.is_some())
        .filter_map(|row| {
            baseline
                .get(&row.name.to_ascii_lowercase())
                .map(|value| (row.name.clone(), *value))
        })
        .collect()
}

fn render_mini_bode(
    ui: &mut Ui,
    state: &mut AppState,
    summary: &super::summary::NetlistRunSummary,
) {
    let Some((&x0, &x1)) = summary
        .frequency
        .first()
        .zip(summary.frequency.last())
        .filter(|(lo, hi)| **lo > 0.0 && **hi > **lo)
    else {
        return;
    };
    let (g_min, g_max) = summary.stability.gain_extremes.unwrap_or((-20.0, 20.0));
    let pad = ((g_max - g_min) * 0.12).max(6.0);
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    ui.add_space(8.0);
    ui.allocate_ui(egui::vec2(ui.available_width(), 130.0), |ui| {
        let mut spec = PlotSpec::new(
            Axis::log_decades(x0, x1, "Hz"),
            XScale::Log10,
            Axis::linear(g_min - pad, g_max + pad, "dB"),
        );
        spec.left_margin = 42.0;
        spec.ref_lines.push(plot::RefLine { y: 0.0 });
        spec.traces.push(
            Trace::new(&summary.frequency, &summary.gain_db, c.traces[0])
                .thin()
                .cache_key(0x4E45_544C_4953_5401),
        );
        if let Some(ugf) = summary.stability.ugf {
            spec.markers.push(plot::Marker {
                x: ugf,
                y: 0.0,
                side: plot::YSide::Left,
                color: c.accent,
                label: "UGF".to_string(),
                drop_line: true,
                label_dy: 0.0,
            });
        }
        plot::show(ui, &spec, &mut state.shell.results.cache, None, None);
    });
}

fn render_as_tuned(ui: &mut Ui, summary: &super::summary::NetlistRunSummary) {
    section_header(ui, "As tuned", None);
    let rows = [
        (
            "ADC",
            fmt_opt(summary.stability.adc_db, |v| format!("{v:.1} dB")),
            false,
        ),
        (
            "UGF",
            fmt_opt(summary.stability.ugf, |v| fmt_si(v, "Hz", 2)),
            true,
        ),
        (
            "PM",
            fmt_opt(summary.stability.pm_deg, |v| format!("{v:.1} deg")),
            true,
        ),
        (
            "GM",
            fmt_opt(summary.stability.gm_db, |v| format!("{v:.1} dB")),
            false,
        ),
        (
            "f3dB",
            fmt_opt(summary.stability.f3db, |v| fmt_si(v, "Hz", 1)),
            false,
        ),
        (
            "f180",
            fmt_opt(summary.stability.f180, |v| fmt_si(v, "Hz", 1)),
            false,
        ),
    ];
    let t = Tokens::get(ui.ctx());
    for (name, value, highlight) in rows {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(name)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(value)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(if highlight {
                            t.color.accent
                        } else {
                            t.color.text
                        }),
                );
            });
        });
    }
}

fn fmt_opt(value: Option<f64>, f: impl FnOnce(f64) -> String) -> String {
    value.map(f).unwrap_or_else(|| "-".to_string())
}

pub(crate) fn scan_assignments_for_baseline(line: &str) -> Option<Vec<(String, usize, usize)>> {
    scan_assignments(line)
}

/// All `.param` rows in the buffer.
fn scan_params(buffer: &str) -> Vec<ParamRow> {
    let mut rows = Vec::new();
    let mut named_ranges: HashMap<String, (f64, f64)> = HashMap::new();
    let mut pending_next_range: Option<(f64, f64)> = None;

    for (idx, line) in buffer.lines().enumerate() {
        if let Some((name, range)) = parse_tune_comment(line) {
            if let Some(name) = name {
                named_ranges.insert(name, range);
            } else {
                pending_next_range = Some(range);
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
            let key = name.to_ascii_lowercase();
            let range = named_ranges
                .remove(&key)
                .or_else(|| pending_next_range.take());
            rows.push(ParamRow {
                name,
                line: idx,
                value,
                raw,
                range,
                baseline: None,
            });
        }
    }
    rows
}

fn parse_tune_comment(line: &str) -> Option<(Option<String>, (f64, f64))> {
    let text = line.trim_start().strip_prefix('*')?.trim();
    let tail = text.strip_prefix("@tune")?.trim();
    let mut parts = tail.split_whitespace();
    let first = parts.next()?;
    let (name, range_text) = if first.contains("..") {
        (None, first)
    } else {
        (Some(first.to_ascii_lowercase()), parts.next()?)
    };
    let (lo, hi) = range_text.split_once("..")?;
    let lo = parse_engineering_value(lo).ok()?;
    let hi = parse_engineering_value(hi).ok()?;
    (hi > lo).then_some((name, (lo, hi)))
}

/// Parse a `.param` line into `(name, value_start, value_end)` triples
/// (byte offsets into `line`). Returns `None` for non-`.param` lines.
fn scan_assignments(line: &str) -> Option<Vec<(String, usize, usize)>> {
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
        assert_range_close(rows[0].range, (5e-6, 60e-6));
    }

    #[test]
    fn tune_annotation_before_param_targets_next_assignment() {
        let rows = scan_params("* @tune 0.5p..8p\n.param cl=2p\n");
        assert_range_close(rows[0].range, (0.5e-12, 8e-12));
    }

    #[test]
    fn reset_payload_uses_last_run_values_only_for_numeric_params() {
        let rows = scan_params(".param a=2 b={expr}\n");
        let mut baseline = std::collections::HashMap::new();
        baseline.insert("a".to_string(), 1.0);
        assert_eq!(reset_values(&rows, &baseline), vec![("a".to_string(), 1.0)]);
    }

    fn assert_range_close(actual: Option<(f64, f64)>, expected: (f64, f64)) {
        let actual = actual.expect("range");
        assert!((actual.0 - expected.0).abs() <= expected.0.abs() * 1e-12);
        assert!((actual.1 - expected.1).abs() <= expected.1.abs() * 1e-12);
    }
}
