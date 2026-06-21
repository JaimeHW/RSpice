//! The editable netlist surface: syntax-highlighted `TextEdit`, a gutter
//! with line numbers and diagnostic/diff pips, and the diagnostics strip.
//!
//! Diagnostics come from one debounced parse of the buffer with the same
//! resolver the runner uses; the squiggle (underline), the gutter pip,
//! and the bottom strip all read that single vector.

use egui::Ui;

use crate::common::AppState;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::{
    Diagnostic, DiagnosticSeverity, completion,
    diagnostics::{line_column_for_span, unknown_reference_diagnostics},
    highlight,
};

/// Editor body font size (gutter follows it).
const FONT_SIZE: f32 = 12.5;
/// Width reserved for line numbers and pips.
const GUTTER_W: f32 = 64.0;
/// Seconds of typing silence before the buffer re-parses.
const PARSE_DEBOUNCE: f64 = 0.35;

/// Stable id so the completion accept can reposition the caret.
fn editor_id() -> egui::Id {
    egui::Id::new("volta.netlist.editor.text")
}

/// Render the editor and diagnostics strip.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    refresh_diagnostics(ui, state);

    // Popover keys (⇥, ↑↓, Esc) must be consumed before the TextEdit so
    // an open popover owns them.
    let completion_keys = completion::consume_keys(ui, &state.shell.netlist);

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
    let diagnostics = state.shell.netlist.diagnostics.clone();
    let edited_lines = state.shell.netlist.edited_lines.clone();

    // Take the buffer out so the layouter and the post-edit bookkeeping
    // don't fight over `state`.
    let mut buffer = std::mem::take(&mut state.simulation.netlist_content);

    let layouter_font = font.clone();
    let mut layouter = |ui: &Ui, text: &str, _wrap_width: f32| {
        let job = highlight::layout_job(text, layouter_font.clone(), &c, &diagnostics);
        ui.fonts(|fonts| fonts.layout_job(job))
    };

    let mut changed = false;
    let mut source_changed = false;
    let mut cursor_line = state.shell.netlist.cursor_line;
    let mut te_output: Option<egui::text_edit::TextEditOutput> = None;

    ui.allocate_ui(egui::vec2(ui.available_width(), editor_h), |ui| {
        egui::ScrollArea::both()
            .id_salt("volta.netlist.editor")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let output = egui::TextEdit::multiline(&mut buffer)
                    .id(editor_id())
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
                    let diagnostic_severity = line_diagnostic_severity(&diagnostics, idx, &buffer);
                    let color = if let Some(severity) = diagnostic_severity {
                        diagnostic_color(severity, &c)
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
                    if let Some(severity) = diagnostic_severity {
                        painter.circle_filled(
                            egui::pos2(origin.x - 12.0, y),
                            2.5,
                            diagnostic_color(severity, &c),
                        );
                    } else if edited_lines.contains(&idx) {
                        painter.circle_filled(egui::pos2(origin.x - 12.0, y), 2.5, c.accent);
                    }
                }

                te_output = Some(output);
            });
    });

    // Post-edit bookkeeping, after the buffer borrow ends.
    let now = ui.input(|input| input.time);
    state.shell.netlist.cursor_line = cursor_line;
    if changed {
        source_changed = true;
        let netlist = &mut state.shell.netlist;
        netlist.revision += 1;
        netlist.last_edit_time = now;
        netlist.edited_lines.insert(cursor_line);
        // Editing makes the buffer the source of truth for runs.
        state.workspace.netlist_source = Some(buffer.clone());
        state.workspace.netlist_source_path = None;
        state.workspace.set_netlist_source_dirty(true);
    }

    // Completion popover: trigger, render, and apply an acceptance.
    if let Some(output) = &te_output
        && let Some((start, end, text, caret)) = completion::show(
            ui,
            &mut state.shell.netlist,
            output,
            &buffer,
            completion_keys,
        )
    {
        source_changed = true;
        buffer.replace_range(start..end, &text);
        completion::place_caret(ui, editor_id(), caret);
        let netlist = &mut state.shell.netlist;
        netlist.revision += 1;
        netlist.last_edit_time = now;
        netlist.edited_lines.insert(cursor_line);
        state.workspace.netlist_source = Some(buffer.clone());
        state.workspace.netlist_source_path = None;
        state.workspace.set_netlist_source_dirty(true);
    }

    state.simulation.netlist_content = buffer;
    if source_changed {
        super::refresh_diff_pips_from_baseline(state);
    }

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

    for (idx, diagnostic) in state
        .shell
        .netlist
        .diagnostics
        .iter()
        .take(rows)
        .enumerate()
    {
        let y = rect.top() + 5.0 + idx as f32 * 22.0 + 11.0;
        let color = diagnostic_color(diagnostic.severity, &c);
        ui.painter()
            .circle_filled(egui::pos2(rect.left() + 14.0, y), 2.5, color);
        let text =
            diagnostic_strip_text(diagnostic, &state.simulation.netlist_content, rect.width());
        ui.painter().text(
            egui::pos2(rect.left() + 26.0, y),
            egui::Align2::LEFT_CENTER,
            text,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            color,
        );
    }
}

const DIAGNOSTIC_STRIP_TEXT_X: f32 = 26.0;
const DIAGNOSTIC_STRIP_RIGHT_PADDING: f32 = 8.0;
const DIAGNOSTIC_STRIP_CHAR_WIDTH: f32 = 7.0;

fn diagnostic_strip_text(diagnostic: &Diagnostic, buffer: &str, width: f32) -> String {
    let location = diagnostic_location(diagnostic, buffer);
    let primary = format!(
        "{}{location}{}",
        diagnostic_label(diagnostic.severity),
        diagnostic.message
    );
    let with_fix = diagnostic
        .fix
        .as_ref()
        .map(|fix| format!("{primary} · fix: {}", fix.label));

    if let Some(text) = with_fix
        && diagnostic_strip_text_fits(&text, width)
    {
        return text;
    }
    if diagnostic_strip_text_fits(&primary, width) {
        return primary;
    }
    truncate_diagnostic_strip_text(&primary, width)
}

fn diagnostic_strip_text_fits(text: &str, width: f32) -> bool {
    text.chars().count() <= diagnostic_strip_char_budget(width)
}

fn truncate_diagnostic_strip_text(text: &str, width: f32) -> String {
    let budget = diagnostic_strip_char_budget(width);
    if text.chars().count() <= budget {
        return text.to_string();
    }
    if budget <= 3 {
        return text.chars().take(budget).collect();
    }
    let mut truncated: String = text.chars().take(budget - 3).collect();
    truncated.push_str("...");
    truncated
}

fn diagnostic_strip_char_budget(width: f32) -> usize {
    ((width - DIAGNOSTIC_STRIP_TEXT_X - DIAGNOSTIC_STRIP_RIGHT_PADDING).max(0.0)
        / DIAGNOSTIC_STRIP_CHAR_WIDTH)
        .floor() as usize
}

fn line_diagnostic_severity(
    diagnostics: &[Diagnostic],
    line: usize,
    buffer: &str,
) -> Option<DiagnosticSeverity> {
    diagnostics
        .iter()
        .filter(|diagnostic| diagnostic_line(diagnostic, buffer) == Some(line))
        .map(|diagnostic| diagnostic.severity)
        .max()
}

fn diagnostic_line(diagnostic: &Diagnostic, buffer: &str) -> Option<usize> {
    diagnostic.line.or_else(|| {
        diagnostic
            .span
            .as_ref()
            .map(|span| line_column_for_span(buffer, span.start).0)
    })
}

fn diagnostic_location(diagnostic: &Diagnostic, buffer: &str) -> String {
    let derived = diagnostic
        .span
        .as_ref()
        .map(|span| line_column_for_span(buffer, span.start));
    let line = diagnostic.line.or_else(|| derived.map(|(line, _)| line));
    let column = diagnostic
        .column
        .or_else(|| derived.map(|(_, column)| column));

    match (line, column) {
        (Some(line), Some(column)) => format!("line {}:{} · ", line + 1, column + 1),
        (Some(line), None) => format!("line {} · ", line + 1),
        _ => String::new(),
    }
}

fn diagnostic_label(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Error => "error · ",
        DiagnosticSeverity::Warning => "warning · ",
        DiagnosticSeverity::Info => "info · ",
    }
}

fn diagnostic_color(
    severity: DiagnosticSeverity,
    c: &crate::ui::palette::Palette,
) -> egui::Color32 {
    match severity {
        DiagnosticSeverity::Error => c.err,
        DiagnosticSeverity::Warning => c.warn,
        DiagnosticSeverity::Info => c.text_dim,
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
    let (diagnostics, symbols) = if buffer.trim().is_empty() {
        (Vec::new(), Some(Vec::new()))
    } else {
        parse_buffer(buffer)
    };
    let netlist = &mut state.shell.netlist;
    netlist.diagnostics = diagnostics;
    if let Some(symbols) = symbols {
        netlist.symbols = symbols;
    }
    netlist.diag_revision = Some(revision);
}

/// Parse the buffer: diagnostics out of the error path, completion
/// symbols out of the success path (a broken parse keeps the previous
/// symbols). Includes are *not* resolved here — `.include`/`.lib` lines
/// are inert in this pass, which keeps keystroke linting free of file
/// IO; errors inside included files still surface at run time.
fn parse_buffer(buffer: &str) -> (Vec<Diagnostic>, Option<Vec<completion::SymbolEntry>>) {
    match rspice_core::Netlist::parse(buffer) {
        Ok(netlist) => (
            unknown_reference_diagnostics(buffer),
            Some(harvest_symbols(&netlist)),
        ),
        Err(rspice_core::netlist::ParseError::Syntax { line, message }) => (
            // Parser lines are 1-based; `line == 0` means "unlocated".
            vec![Diagnostic::error(message).with_line(line.checked_sub(1))],
            None,
        ),
        Err(other) => (vec![Diagnostic::error(other.to_string())], None),
    }
}

/// Completion symbols from a clean parse: model cards and subcircuits.
fn harvest_symbols(netlist: &rspice_core::Netlist) -> Vec<completion::SymbolEntry> {
    let mut symbols = Vec::new();
    for model in &netlist.models {
        symbols.push(completion::SymbolEntry {
            name: model.name.clone(),
            kind: "model",
            detail: model.model_type.to_ascii_uppercase(),
            doc: format!(
                "{} model card from this deck · {} parameter(s).",
                model.model_type.to_ascii_uppercase(),
                model.params.len() + model.expr_params.len() + model.string_params.len()
            ),
        });
    }
    for subckt in &netlist.subcircuits {
        symbols.push(completion::SymbolEntry {
            name: subckt.name.clone(),
            kind: "subckt",
            detail: format!("{} ports", subckt.ports.len()),
            doc: format!(".subckt {} {}", subckt.name, subckt.ports.join(" ")),
        });
    }
    symbols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_buffer_maps_syntax_error_to_structured_diagnostic() {
        let (diagnostics, symbols) = parse_buffer("title\nR1 out 0\n");

        assert!(symbols.is_none());
        assert_eq!(diagnostics.len(), 1);
        let diagnostic = &diagnostics[0];
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert_eq!(diagnostic.line, Some(1));
        assert_eq!(diagnostic.column, None);
        assert!(diagnostic.span.is_none());
        assert!(diagnostic.fix.is_none());
        assert!(!diagnostic.message.trim().is_empty());
    }

    #[test]
    fn parse_buffer_appends_unknown_reference_lints_after_clean_parse() {
        let source = "deck\nM1 d g s b nchh W=1u L=1u\n.model nch nmos\n.op\n.end\n";

        let (diagnostics, symbols) = parse_buffer(source);

        assert!(symbols.is_some());
        assert_eq!(diagnostics.len(), 1);
        let diagnostic = &diagnostics[0];
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert_eq!(&source[diagnostic.span.clone().unwrap()], "nchh");
        assert_eq!(diagnostic.fix.as_ref().unwrap().replacement, "nch");
    }

    #[test]
    fn diagnostic_strip_text_omits_fix_and_truncates_for_phone_width() {
        let source = "deck\nM1 d g s b nchh W=1u L=1u\n.model nch nmos\n.end\n";
        let diagnostic = Diagnostic {
            severity: DiagnosticSeverity::Error,
            span: Some(18..22),
            line: Some(1),
            column: Some(11),
            message: "Unknown model `nchh` in this deck.".to_string(),
            fix: Some(crate::shell::views::netlist::diagnostics::DiagnosticFix {
                label: "Replace with nch".to_string(),
                span: 18..22,
                replacement: "nch".to_string(),
            }),
        };

        let text = diagnostic_strip_text(&diagnostic, source, 160.0);

        assert!(!text.contains("fix:"));
        assert!(text.ends_with("..."), "{text}");
        assert!(diagnostic_strip_text_fits(&text, 160.0));
    }
}
