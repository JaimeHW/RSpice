//! The netlist source surface: a selectable, immutable generated view or an
//! editable project-owned `TextEdit`, plus line numbers, diagnostic/diff pips,
//! and the diagnostics strip.
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
    diagnostics::{line_column_for_span, parser_diagnostics, unknown_reference_diagnostics},
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
    egui::Id::new("rspice.netlist.editor.text")
}

fn line_for_char_index(text: &str, char_index: usize) -> usize {
    text.chars()
        .take(char_index)
        .filter(|ch| *ch == '\n')
        .count()
}

/// Render the editor and diagnostics strip.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let editable = state.workspace.has_editable_netlist_source();

    refresh_diagnostics(ui, state);

    // Popover keys (⇥, ↑↓, Esc) must be consumed before the TextEdit so
    // an open popover owns them.
    let completion_keys = if editable {
        completion::consume_keys(ui, &state.ui.netlist)
    } else {
        // Generated output owns no editor state. Close a popover that may have
        // survived a transition away from an owned source document.
        state.ui.netlist.completion_open = false;
        (0, false, false)
    };

    // Document well backdrop.
    let well = ui.available_rect_before_wrap();
    ui.painter().rect_filled(well, 0.0, c.canvas_bg);

    // Reserve the diagnostics strip before the editor takes the rest.
    let strip_rows = state.ui.netlist.diagnostics.len().min(3);
    let strip_h = if strip_rows > 0 {
        strip_rows as f32 * 22.0 + 10.0
    } else {
        0.0
    };
    let editor_h = (ui.available_height() - strip_h).max(60.0);

    let font = theme::mono(FONT_SIZE, FontWeight::Regular);
    let diagnostics = state.ui.netlist.diagnostics.clone();
    let edited_lines = state.ui.netlist.edited_lines.clone();

    // Take the buffer out so the layouter and the post-edit bookkeeping
    // don't fight over `state`.
    let mut buffer = std::mem::take(&mut state.simulation.netlist_content);

    let layouter_font = font.clone();
    let mut layouter = |ui: &Ui, text: &dyn egui::TextBuffer, _wrap_width: f32| {
        let job = highlight::layout_job(text.as_str(), layouter_font.clone(), &c, &diagnostics);
        ui.fonts_mut(|fonts| fonts.layout_job(job))
    };

    let mut changed = false;
    let mut completion_changed = false;
    let mut cursor_line = state.ui.netlist.cursor_line;
    let mut te_output: Option<egui::text_edit::TextEditOutput> = None;

    ui.allocate_ui(egui::vec2(ui.available_width(), editor_h), |ui| {
        egui::ScrollArea::both()
            .id_salt("rspice.netlist.editor")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let mut show_text = |text: &mut dyn egui::TextBuffer| {
                    egui::TextEdit::multiline(text)
                        .id(editor_id())
                        .code_editor()
                        .font(font.clone())
                        .desired_width(f32::INFINITY)
                        .desired_rows(30)
                        .frame(egui::Frame::NONE)
                        .margin(egui::Margin {
                            left: GUTTER_W as i8,
                            right: 12,
                            top: 6,
                            bottom: 6,
                        })
                        .layouter(&mut layouter)
                        .show(ui)
                };
                let output = if editable {
                    show_text(&mut buffer)
                } else {
                    // `&str` implements egui's immutable `TextBuffer`: users can
                    // focus, select, and copy generated source, but no keyboard,
                    // paste, IME, or accessibility edit can mutate its bytes.
                    let mut read_only = buffer.as_str();
                    show_text(&mut read_only)
                };

                if let Some(range) = output.cursor_range {
                    cursor_line = line_for_char_index(&buffer, range.primary.index);
                }
                changed = output.response.changed();

                // Gutter: numbers plus diagnostic (err) and diff (accent)
                // pips, aligned to the galley's actual rows.
                let painter = ui.painter();
                let origin = output.galley_pos;
                let gutter_font = theme::mono(FONT_SIZE - 1.5, FontWeight::Regular);
                for (idx, row) in output.galley.rows.iter().enumerate() {
                    let y = origin.y + row.rect().center().y;
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

    let now = ui.input(|input| input.time);
    state.ui.netlist.cursor_line = cursor_line;

    // Completion is an edit and therefore exists only for project-owned source.
    if editable
        && let Some(output) = &te_output
        && let Some((start, end, text, caret)) =
            completion::show(ui, &mut state.ui.netlist, output, &buffer, completion_keys)
    {
        completion_changed = true;
        buffer.replace_range(start..end, &text);
        completion::place_caret(ui, editor_id(), caret);
    }

    // All editor writes pass through the ownership guard. Even if a future UI
    // path reports a change for read-only content, it cannot implicitly create
    // `workspace.netlist_source`.
    let source_changed = (changed || completion_changed)
        && commit_owned_source_edit(state, &buffer, cursor_line, now);
    if !source_changed {
        // The buffer was taken at the beginning of the frame; restore it on
        // unchanged and generated-document frames.
        state.simulation.netlist_content = buffer;
    }
    if source_changed {
        super::refresh_diff_pips_from_baseline(state);
    }

    if strip_rows > 0 {
        diagnostics_strip(ui, state, strip_rows);
    }
}

/// Commit a text/completion edit without promoting generated output to owned
/// source. Returns whether an existing owned source actually changed.
fn commit_owned_source_edit(
    state: &mut AppState,
    buffer: &str,
    cursor_line: usize,
    now: f64,
) -> bool {
    if !state
        .workspace
        .replace_editable_netlist_source(buffer.to_owned())
    {
        return false;
    }

    state.simulation.netlist_content = buffer.to_owned();
    let netlist = &mut state.ui.netlist;
    netlist.revision = netlist.revision.wrapping_add(1);
    netlist.last_edit_time = now;
    netlist.edited_lines.insert(cursor_line);
    true
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

    for (idx, diagnostic) in state.ui.netlist.diagnostics.iter().take(rows).enumerate() {
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
    let revision = state.ui.netlist.revision;
    if state.ui.netlist.diag_revision == Some(revision) {
        return;
    }
    let now = ui.input(|input| input.time);
    let settled = now - state.ui.netlist.last_edit_time >= PARSE_DEBOUNCE;
    if !settled && state.ui.netlist.diag_revision.is_some() {
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
    let netlist = &mut state.ui.netlist;
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
        Ok(netlist) => {
            let mut diagnostics = parser_diagnostics(&netlist);
            diagnostics.extend(unknown_reference_diagnostics(buffer));
            (diagnostics, Some(harvest_symbols(&netlist)))
        }
        Err(error) => (vec![parse_error_diagnostic(error)], None),
    }
}

fn parse_error_diagnostic(error: rspice_core::netlist::ParseError) -> Diagnostic {
    use rspice_core::netlist::ParseError;

    match error {
        ParseError::Syntax { line, message } => {
            // Parser lines are 1-based; `line == 0` means "unlocated".
            Diagnostic::error(message).with_line(line.checked_sub(1))
        }
        ParseError::MissingSubcircuitEnds(error) => {
            Diagnostic::error(error.to_string()).with_line(error.opened_at.line.checked_sub(1))
        }
        ParseError::DuplicateSubcircuitPortBinding(error) => Diagnostic::error(format!(
            "{}\nSubcircuit: {} (canonical {}) · instance: {} · formal {} · positions {} and {} · effective nodes {} and {}",
            error,
            error.subcircuit_name,
            error.canonical_subcircuit_name,
            error.qualified_instance_name,
            error.formal_port,
            error.first_position,
            error.conflicting_position,
            error.first_actual_node,
            error.conflicting_actual_node,
        )),
        ParseError::GlobalSubcircuitPortBinding(error) => Diagnostic::error(format!(
            "{}\nSubcircuit: {} (canonical {}) · instance: {} · formal {} · position {} · effective node {}",
            error,
            error.subcircuit_name,
            error.canonical_subcircuit_name,
            error.qualified_instance_name,
            error.formal_port,
            error.position,
            error.actual_node,
        )),
        ParseError::DeviceInitialCondition(error) => {
            let origin = device_initial_condition_diagnostic_origin(&error);
            Diagnostic::error(error.to_string()).with_line(origin.line.checked_sub(1))
        }
        other => Diagnostic::error(other.to_string()),
    }
}

fn device_initial_condition_diagnostic_origin(
    error: &rspice_core::netlist::DeviceInitialConditionError,
) -> &rspice_core::netlist::NetlistSourceLocation {
    use rspice_core::netlist::DeviceInitialConditionError;

    match error {
        DeviceInitialConditionError::DuplicateDirective { duplicate, .. } => duplicate,
        DeviceInitialConditionError::MalformedSource { record_origin, .. } => record_origin,
        DeviceInitialConditionError::MissingInformation { origin }
        | DeviceInitialConditionError::MalformedDirective { origin, .. }
        | DeviceInitialConditionError::SourceUnavailable { origin, .. }
        | DeviceInitialConditionError::NonFiniteValue { origin, .. }
        | DeviceInitialConditionError::UnresolvedSource { origin, .. }
        | DeviceInitialConditionError::InvalidArity { origin, .. }
        | DeviceInitialConditionError::UnsupportedTarget { origin, .. } => origin,
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
        let (diagnostics, symbols) = parse_buffer("title\n.enddata\n");

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
    fn parse_buffer_maps_missing_ends_to_opening_line() {
        let source = "* missing ends\n.subckt Cell a b\nR1 a b 1\n.end\n";

        let (diagnostics, symbols) = parse_buffer(source);

        assert!(symbols.is_none());
        assert_eq!(diagnostics.len(), 1);
        let diagnostic = &diagnostics[0];
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert_eq!(diagnostic.line, Some(1));
        assert!(diagnostic.message.contains("Subcircuit CELL missing .ENDS"));
        assert!(diagnostic.message.contains("reached .END"));
    }

    #[test]
    fn parse_buffer_maps_initcond_duplicate_to_duplicate_card_line() {
        let source = "duplicate initcond\n\
                      .INITCOND C1 IC=1\n\
                      .INITCOND malformed second card\n\
                      C1 1 0 1u\n\
                      .END\n";

        let (diagnostics, symbols) = parse_buffer(source);

        assert!(symbols.is_none());
        assert_eq!(diagnostics.len(), 1);
        let diagnostic = &diagnostics[0];
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert_eq!(diagnostic.line, Some(2));
        assert!(diagnostic.message.contains("may appear only once"));
    }

    #[test]
    fn duplicate_subcircuit_binding_diagnostic_preserves_hierarchy_context() {
        let diagnostic = parse_error_diagnostic(
            rspice_core::netlist::ParseError::DuplicateSubcircuitPortBinding(Box::new(
                rspice_core::netlist::DuplicateSubcircuitPortBindingError {
                    subcircuit_name: "inv1".into(),
                    canonical_subcircuit_name: "INV1".into(),
                    instance_name: "Xinv1".into(),
                    canonical_instance_name: "XINV1".into(),
                    qualified_instance_name: "TOP.Xinv1".into(),
                    formal_port: "GND".into(),
                    first_position: 4,
                    conflicting_position: 8,
                    first_actual_node: "0".into(),
                    conflicting_actual_node: "VDD".into(),
                },
            )),
        );

        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert_eq!(diagnostic.line, None);
        assert!(
            diagnostic
                .message
                .contains("Subcircuit: inv1 (canonical INV1)")
        );
        assert!(diagnostic.message.contains("TOP.Xinv1"));
        assert!(diagnostic.message.contains("positions 4 and 8"));
        assert!(diagnostic.message.contains("effective nodes 0 and VDD"));
    }

    #[test]
    fn global_subcircuit_binding_diagnostic_preserves_effective_node() {
        let diagnostic = parse_error_diagnostic(
            rspice_core::netlist::ParseError::GlobalSubcircuitPortBinding(Box::new(
                rspice_core::netlist::GlobalSubcircuitPortBindingError {
                    subcircuit_name: "cell".into(),
                    canonical_subcircuit_name: "CELL".into(),
                    instance_name: "X1".into(),
                    canonical_instance_name: "X1".into(),
                    qualified_instance_name: "TOP.X1".into(),
                    formal_port: "$G_SHARED".into(),
                    position: 1,
                    actual_node: "LOCAL".into(),
                },
            )),
        );

        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert!(
            diagnostic
                .message
                .contains("Subcircuit: cell (canonical CELL)")
        );
        assert!(diagnostic.message.contains("TOP.X1"));
        assert!(diagnostic.message.contains("effective node LOCAL"));
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
            fix: Some(
                crate::workbench::netlist_document::diagnostics::DiagnosticFix {
                    label: "Replace with nch".to_string(),
                    span: 18..22,
                    replacement: "nch".to_string(),
                },
            ),
        };

        let text = diagnostic_strip_text(&diagnostic, source, 160.0);

        assert!(!text.contains("fix:"));
        assert!(text.ends_with("..."), "{text}");
        assert!(diagnostic_strip_text_fits(&text, 160.0));
    }

    #[test]
    fn editor_commit_rejects_generated_artifact_mutation() {
        let mut state = AppState::default();
        state.simulation.netlist_content = "generated\n.op\n.end\n".to_owned();
        state.ui.netlist.revision = 4;

        assert!(!commit_owned_source_edit(
            &mut state,
            "silently edited\n.end\n",
            1,
            2.5,
        ));
        assert!(state.workspace.netlist_source.is_none());
        assert_eq!(state.simulation.netlist_content, "generated\n.op\n.end\n");
        assert!(!state.workspace.netlist_source_dirty);
        assert_eq!(state.ui.netlist.revision, 4);
        assert!(state.ui.netlist.edited_lines.is_empty());
    }

    #[test]
    fn editor_commit_updates_only_an_existing_owned_source() {
        let mut state = AppState::default();
        state.workspace.netlist_source = Some("owned\n.op\n.end\n".to_owned());
        state.workspace.netlist_source_path = Some(std::path::PathBuf::from("imported/owned.cir"));
        state.simulation.netlist_content = "owned\n.op\n.end\n".to_owned();
        state.ui.netlist.revision = 9;

        assert!(commit_owned_source_edit(
            &mut state,
            "owned\n.tran 1n 1u\n.end\n",
            1,
            7.25,
        ));
        assert_eq!(
            state.workspace.netlist_source.as_deref(),
            Some("owned\n.tran 1n 1u\n.end\n")
        );
        assert_eq!(
            state.simulation.netlist_content,
            "owned\n.tran 1n 1u\n.end\n"
        );
        assert!(state.workspace.netlist_source_path.is_none());
        assert!(state.workspace.netlist_source_dirty);
        assert_eq!(state.ui.netlist.revision, 10);
        assert_eq!(state.ui.netlist.last_edit_time, 7.25);
        assert!(state.ui.netlist.edited_lines.contains(&1));
    }
}
