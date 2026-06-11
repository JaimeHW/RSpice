//! The editable netlist surface: syntax-highlighted `TextEdit`, a gutter
//! with line numbers and diagnostic/diff pips, and the diagnostics strip.
//!
//! Diagnostics come from one debounced parse of the buffer with the same
//! resolver the runner uses; the squiggle (underline), the gutter pip,
//! and the bottom strip all read that single vector.

use std::collections::HashSet;

use egui::Ui;

use crate::common::AppState;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::{Diagnostic, highlight};

/// Editor body font size (gutter follows it).
const FONT_SIZE: f32 = 12.5;
/// Width reserved for line numbers and pips.
const GUTTER_W: f32 = 64.0;
/// Seconds of typing silence before the buffer re-parses.
const PARSE_DEBOUNCE: f64 = 0.35;

/// Render the editor and diagnostics strip.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    refresh_diagnostics(ui, state);

    // Document well backdrop.
    let well = ui.available_rect_before_wrap();
    ui.painter().rect_filled(well, 0.0, c.canvas_bg);

    // Reserve the diagnostics strip before the editor takes the rest.
    let strip_rows = state.shell.netlist.diagnostics.len().min(3);
    let strip_h = if strip_rows > 0 {
        strip_rows as f32 * 22.0 + 10.0
    } else {
        0.0
    };
    let editor_h = (ui.available_height() - strip_h).max(60.0);

    let font = theme::mono(FONT_SIZE, FontWeight::Regular);
    let error_lines: HashSet<usize> = state
        .shell
        .netlist
        .diagnostics
        .iter()
        .filter_map(|d| d.line)
        .collect();
    let edited_lines = state.shell.netlist.edited_lines.clone();

    // Take the buffer out so the layouter and the post-edit bookkeeping
    // don't fight over `state`.
    let mut buffer = std::mem::take(&mut state.simulation.netlist_content);

    let layouter_font = font.clone();
    let mut layouter = |ui: &Ui, text: &str, _wrap_width: f32| {
        let job = highlight::layout_job(text, layouter_font.clone(), &c, &error_lines);
        ui.fonts(|fonts| fonts.layout_job(job))
    };

    let mut changed = false;
    let mut cursor_line = state.shell.netlist.cursor_line;

    ui.allocate_ui(egui::vec2(ui.available_width(), editor_h), |ui| {
        egui::ScrollArea::both()
            .id_salt("volta.netlist.editor")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let output = egui::TextEdit::multiline(&mut buffer)
                    .code_editor()
                    .font(font.clone())
                    .desired_width(f32::INFINITY)
                    .desired_rows(30)
                    .frame(false)
                    .margin(egui::Margin {
                        left: GUTTER_W,
                        right: 12.0,
                        top: 6.0,
                        bottom: 6.0,
                    })
                    .layouter(&mut layouter)
                    .show(ui);

                if let Some(range) = output.cursor_range {
                    cursor_line = range.primary.rcursor.row;
                }
                changed = output.response.changed();

                // Gutter: numbers plus diagnostic (err) and diff (accent)
                // pips, aligned to the galley's actual rows.
                let painter = ui.painter();
                let origin = output.galley_pos;
                let gutter_font = theme::mono(FONT_SIZE - 1.5, FontWeight::Regular);
                for (idx, row) in output.galley.rows.iter().enumerate() {
                    let y = origin.y + row.rect.center().y;
                    if !ui.clip_rect().y_range().contains(y) {
                        continue;
                    }
                    let color = if error_lines.contains(&idx) {
                        c.err
                    } else if idx == cursor_line {
                        c.text_dim
                    } else {
                        c.text_faint
                    };
                    painter.text(
                        egui::pos2(origin.x - 22.0, y),
                        egui::Align2::RIGHT_CENTER,
                        (idx + 1).to_string(),
                        gutter_font.clone(),
                        color,
                    );
                    if error_lines.contains(&idx) {
                        painter.circle_filled(egui::pos2(origin.x - 12.0, y), 2.5, c.err);
                    } else if edited_lines.contains(&idx) {
                        painter.circle_filled(egui::pos2(origin.x - 12.0, y), 2.5, c.accent);
                    }
                }
            });
    });

    // Post-edit bookkeeping, after the buffer borrow ends.
    state.shell.netlist.cursor_line = cursor_line;
    if changed {
        let now = ui.input(|input| input.time);
        let netlist = &mut state.shell.netlist;
        netlist.revision += 1;
        netlist.last_edit_time = now;
        netlist.edited_lines.insert(cursor_line);
        // Editing makes the buffer the source of truth for runs.
        state.workspace.netlist_source = Some(buffer.clone());
    }
    state.simulation.netlist_content = buffer;

    if strip_rows > 0 {
        diagnostics_strip(ui, state, strip_rows);
    }
}

/// Bottom strip: the first few diagnostics as `line N · message` rows.
fn diagnostics_strip(ui: &mut Ui, state: &AppState, rows: usize) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();

    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(width, rows as f32 * 22.0 + 10.0),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, c.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.top() + 0.5,
        egui::Stroke::new(1.0, c.border),
    );

    for (idx, diagnostic) in state.shell.netlist.diagnostics.iter().take(rows).enumerate() {
        let y = rect.top() + 5.0 + idx as f32 * 22.0 + 11.0;
        ui.painter()
            .circle_filled(egui::pos2(rect.left() + 14.0, y), 2.5, c.err);
        let location = diagnostic
            .line
            .map(|line| format!("line {} · ", line + 1))
            .unwrap_or_default();
        ui.painter().text(
            egui::pos2(rect.left() + 26.0, y),
            egui::Align2::LEFT_CENTER,
            format!("{location}{}", diagnostic.message),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.err,
        );
    }
}

/// Debounced re-parse: the squiggles, pips, and strip all derive from
/// this one result.
fn refresh_diagnostics(ui: &Ui, state: &mut AppState) {
    let revision = state.shell.netlist.revision;
    if state.shell.netlist.diag_revision == Some(revision) {
        return;
    }
    let now = ui.input(|input| input.time);
    let settled = now - state.shell.netlist.last_edit_time >= PARSE_DEBOUNCE;
    if !settled && state.shell.netlist.diag_revision.is_some() {
        // Still typing: keep the stale diagnostics one beat longer.
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(120));
        return;
    }

    let buffer = &state.simulation.netlist_content;
    let diagnostics = if buffer.trim().is_empty() {
        Vec::new()
    } else {
        parse_diagnostics(buffer)
    };
    let netlist = &mut state.shell.netlist;
    netlist.diagnostics = diagnostics;
    netlist.diag_revision = Some(revision);
}

/// Parse the buffer and map the error (the parser stops at the first) to
/// a diagnostic. Includes are *not* resolved here — `.include`/`.lib`
/// lines are inert in this pass, which keeps keystroke linting free of
/// file IO; errors inside included files still surface at run time.
fn parse_diagnostics(buffer: &str) -> Vec<Diagnostic> {
    match rspice_core::Netlist::parse(buffer) {
        Ok(_) => Vec::new(),
        Err(rspice_core::netlist::ParseError::Syntax { line, message }) => vec![Diagnostic {
            // Parser lines are 1-based; `line == 0` means "unlocated".
            line: line.checked_sub(1),
            message,
        }],
        Err(other) => vec![Diagnostic {
            line: None,
            message: other.to_string(),
        }],
    }
}
