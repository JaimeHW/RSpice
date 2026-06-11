//! TUNER — the live parameter loop in the right panel.
//!
//! Rows bind to the buffer's `.param name=value` lines: a log slider, the
//! engineering-notation readout, and a diff pip on the touched line until
//! the next run baselines it. "live" re-simulates on every slider change
//! (queued behind the running engine), "on release" on commit. Values are
//! written back in SPICE notation (`Meg` = 1e6, `m` = 1e-3).

use egui::Ui;

use crate::common::AppState;
use crate::properties::engineering::{format_engineering_value, parse_engineering_value};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{chip, section_header};

/// One tunable row: a `.param` assignment found in the buffer.
struct ParamRow {
    name: String,
    /// 0-based buffer line carrying the assignment.
    line: usize,
    /// Parsed value; `None` for `{expression}` bindings (not sliderable).
    value: Option<f64>,
    /// Raw value text, for the expression readout.
    raw: String,
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

    let rows = scan_params(&state.simulation.netlist_content);
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

    for row in rows {
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
                    c.traces[1]
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

        let (lo, hi) = slider_range(state, &row.name, value);
        let mut slider_value = value;
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.spacing_mut().slider_width = (ui.available_width() - 16.0).max(80.0);
            let logarithmic = lo > 0.0;
            let response = ui.add(
                egui::Slider::new(&mut slider_value, lo..=hi)
                    .logarithmic(logarithmic)
                    .show_value(false),
            );
            if response.changed() && slider_value != value {
                let fire = state.shell.netlist.tuner_live;
                write = Some((row, slider_value, fire));
            } else if response.drag_stopped() && !state.shell.netlist.tuner_live {
                write = Some((row, slider_value, true));
            }
        });
    }

    if let Some((row, new_value, fire_run)) = write {
        apply_param_edit(ui, state, &row, new_value, fire_run);
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

/// All `.param` rows in the buffer.
fn scan_params(buffer: &str) -> Vec<ParamRow> {
    let mut rows = Vec::new();
    for (idx, line) in buffer.lines().enumerate() {
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
            rows.push(ParamRow {
                name,
                line: idx,
                value,
                raw,
            });
        }
    }
    rows
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
}
