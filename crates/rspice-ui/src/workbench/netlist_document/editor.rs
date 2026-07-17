//! The netlist source surface: a selectable, immutable generated view or an
//! editable project-owned `TextEdit`, plus line numbers and diagnostic/diff
//! pips. Full diagnostics are projected by the Code inspector.
//!
//! Diagnostics come from one debounced parse of the buffer with the same
//! resolver the runner uses; the squiggle (underline), gutter pip, and
//! inspector all read that single vector.

use egui::Ui;

use crate::common::AppState;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;

use super::{
    Diagnostic, DiagnosticSeverity, completion,
    diagnostics::{line_column_for_span, parser_diagnostics, unknown_reference_diagnostics},
    highlight,
};

/// Editor body font size (gutter follows it).
const FONT_SIZE: f32 = 11.0;
/// Width reserved for the mockup's line-number gutter.
const GUTTER_W: f32 = 47.0;
const GUTTER_NUMBER_RIGHT_PADDING: f32 = 11.0;
const CODE_LEFT_PADDING: f32 = 12.0;
const CODE_RIGHT_PADDING: i8 = 20;
const CODE_TOP_PADDING: i8 = 8;
const CODE_BOTTOM_PADDING: i8 = 36;
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
    let editable = state.ui.netlist.active_document == super::ActiveNetlistDocument::OwnedSource
        && state.workspace.has_editable_netlist_source();

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
    ui.painter().rect_filled(well, 0.0, c.bg_inset);

    let editor_h = ui.available_height().max(60.0);

    let font = theme::mono(FONT_SIZE, FontWeight::Regular);
    let diagnostics = state.ui.netlist.diagnostics.clone();
    let edited_lines = state.ui.netlist.edited_lines.clone();

    // Take the buffer out so the layouter and the post-edit bookkeeping
    // don't fight over `state`.
    let mut buffer = std::mem::take(&mut state.simulation.netlist_content);
    if let Some(requested_line) = state.ui.netlist.requested_line.take() {
        let char_index = char_index_for_line(&buffer, requested_line);
        completion::place_caret(ui, editor_id(), char_index);
        state.ui.netlist.cursor_line = requested_line.saturating_sub(1);
    }

    let layouter_font = font.clone();
    let diff_document =
        state.ui.netlist.active_document == super::ActiveNetlistDocument::GeneratedDiff;
    let mut layouter = |ui: &Ui, text: &dyn egui::TextBuffer, _wrap_width: f32| {
        let job = if diff_document {
            highlight::diff_layout_job(text.as_str(), layouter_font.clone(), &c)
        } else {
            highlight::layout_job(text.as_str(), layouter_font.clone(), &c, &diagnostics)
        };
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
                // Reserve background paint slots before TextEdit emits its
                // glyphs and selection, then fill them once galley row
                // geometry is known. This preserves crisp text/selection while
                // matching the mockup's active-line wash and accent rail.
                let hover_background = ui.painter().add(egui::Shape::Noop);
                let active_background = ui.painter().add(egui::Shape::Noop);
                let active_rail = ui.painter().add(egui::Shape::Noop);
                let mut show_text = |text: &mut dyn egui::TextBuffer| {
                    egui::TextEdit::multiline(text)
                        .id(editor_id())
                        .code_editor()
                        .font(font.clone())
                        .desired_width(f32::INFINITY)
                        .desired_rows(30)
                        .frame(egui::Frame::NONE)
                        .margin(egui::Margin {
                            left: (GUTTER_W + CODE_LEFT_PADDING) as i8,
                            right: CODE_RIGHT_PADDING,
                            top: CODE_TOP_PADDING,
                            bottom: CODE_BOTTOM_PADDING,
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
                let row_rect = |row: &egui::epaint::text::PlacedRow| {
                    egui::Rect::from_min_max(
                        egui::pos2(output.response.rect.left(), origin.y + row.rect().top()),
                        egui::pos2(output.response.rect.right(), origin.y + row.rect().bottom()),
                    )
                };
                if let Some(row) = output.galley.rows.get(cursor_line) {
                    let rect = row_rect(row);
                    painter.set(
                        active_background,
                        egui::Shape::rect_filled(rect, 0.0, c.accent.gamma_multiply(0.075)),
                    );
                    painter.set(
                        active_rail,
                        egui::Shape::rect_filled(
                            egui::Rect::from_min_max(
                                rect.left_top(),
                                egui::pos2(rect.left() + 2.0, rect.bottom()),
                            ),
                            0.0,
                            c.accent,
                        ),
                    );
                }
                if let Some(pointer) = ui.input(|input| input.pointer.hover_pos())
                    && output.response.rect.contains(pointer)
                    && let Some((index, row)) = output
                        .galley
                        .rows
                        .iter()
                        .enumerate()
                        .find(|(_, row)| row_rect(row).contains(pointer))
                    && index != cursor_line
                {
                    painter.set(
                        hover_background,
                        egui::Shape::rect_filled(row_rect(row), 0.0, c.bg_hover),
                    );
                }
                painter.vline(
                    output.response.rect.left() + GUTTER_W,
                    output.response.rect.y_range(),
                    egui::Stroke::new(1.0, c.border),
                );
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
                        egui::pos2(
                            output.response.rect.left() + GUTTER_W - GUTTER_NUMBER_RIGHT_PADDING,
                            y,
                        ),
                        egui::Align2::RIGHT_CENTER,
                        (idx + 1).to_string(),
                        gutter_font.clone(),
                        color,
                    );
                    if let Some(severity) = diagnostic_severity {
                        painter.circle_filled(
                            egui::pos2(output.response.rect.left() + GUTTER_W - 5.0, y),
                            2.5,
                            diagnostic_color(severity, &c),
                        );
                    } else if edited_lines.contains(&idx) {
                        painter.circle_filled(
                            egui::pos2(output.response.rect.left() + GUTTER_W - 5.0, y),
                            2.5,
                            c.accent,
                        );
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
}

fn char_index_for_line(source: &str, one_based_line: usize) -> usize {
    let target = one_based_line.max(1);
    let mut line = 1;
    for (index, ch) in source.chars().enumerate() {
        if line == target {
            return index;
        }
        if ch == '\n' {
            line += 1;
        }
    }
    source.chars().count()
}

/// Commit a text/completion edit without promoting generated output to owned
/// source. Returns whether an existing owned source actually changed.
fn commit_owned_source_edit(
    state: &mut AppState,
    buffer: &str,
    cursor_line: usize,
    now: f64,
) -> bool {
    if state.ui.netlist.active_document != super::ActiveNetlistDocument::OwnedSource {
        return false;
    }
    if !super::replace_owned_source(state, buffer.to_owned()) {
        return false;
    }
    let netlist = &mut state.ui.netlist;
    netlist.last_edit_time = now;
    netlist.edited_lines.insert(cursor_line);
    true
}

#[cfg(test)]
const DIAGNOSTIC_STRIP_TEXT_X: f32 = 26.0;
#[cfg(test)]
const DIAGNOSTIC_STRIP_RIGHT_PADDING: f32 = 8.0;
#[cfg(test)]
const DIAGNOSTIC_STRIP_CHAR_WIDTH: f32 = 7.0;

#[cfg(test)]
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

#[cfg(test)]
fn diagnostic_strip_text_fits(text: &str, width: f32) -> bool {
    text.chars().count() <= diagnostic_strip_char_budget(width)
}

#[cfg(test)]
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

#[cfg(test)]
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

#[cfg(test)]
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

#[cfg(test)]
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

    if state.ui.netlist.active_document == super::ActiveNetlistDocument::GeneratedDiff {
        state.ui.netlist.diagnostics.clear();
        state.ui.netlist.diag_revision = Some(revision);
        return;
    }

    let buffer = state.simulation.netlist_content.clone();
    let materialized =
        if state.ui.netlist.active_document == super::ActiveNetlistDocument::OwnedSource {
            crate::common::netlist_workflow::compose_owned_netlist_execution_source(state, &buffer)
        } else {
            Ok(buffer.clone())
        };
    let (diagnostics, symbols) = if buffer.trim().is_empty() {
        (Vec::new(), Some(Vec::new()))
    } else {
        match materialized {
            Ok(source) => parse_buffer(&source),
            Err(error) => (vec![Diagnostic::error(error)], None),
        }
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
            if let Err(error) = rspice_core::netlist::validate_output_symbols(&netlist) {
                diagnostics.extend(parse_error_diagnostics(error));
            }
            diagnostics.extend(unknown_reference_diagnostics(buffer));
            (diagnostics, Some(harvest_symbols(&netlist)))
        }
        Err(error) => (parse_error_diagnostics(error), None),
    }
}

/// Convert a typed parse failure into editor diagnostics. Aggregate semantic
/// failures remain one row per authored card so distinct source origins are
/// localized without flooding the strip; ordered and repeated occurrences are
/// retained inside that card's message.
fn parse_error_diagnostics(error: rspice_core::netlist::ParseError) -> Vec<Diagnostic> {
    let rspice_core::netlist::ParseError::OutputSymbolValidation(validation) = error else {
        return vec![parse_error_diagnostic(error)];
    };

    let mut groups = Vec::<(
        rspice_core::netlist::OutputDirectiveKind,
        rspice_core::netlist::NetlistSourceLocation,
        Vec<String>,
    )>::new();
    for unresolved in validation.unresolved {
        let entry = format!(
            "{} `{}` via {}",
            unresolved.kind, unresolved.symbol, unresolved.operator
        );
        match groups.last_mut() {
            Some((directive, origin, entries))
                if *directive == unresolved.directive && *origin == unresolved.origin =>
            {
                entries.push(entry);
            }
            _ => groups.push((unresolved.directive, unresolved.origin, vec![entry])),
        }
    }

    groups
        .into_iter()
        .map(|(directive, origin, entries)| {
            let line = origin.line.checked_sub(1);
            Diagnostic::error(format!(
                "Undefined output symbols in {directive} ({origin}): {}",
                entries.join(", ")
            ))
            .with_line(line)
        })
        .collect()
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
        ParseError::UndefinedMutualInductorReference(error) => Diagnostic::error(format!(
            "{}\nCoupling: {} (canonical {}) · missing inductor: {} (canonical {}) · scope: {} · reference position {}",
            error,
            error.qualified_coupling_name,
            error.canonical_coupling_name,
            error.qualified_inductor_name,
            error.canonical_inductor_name,
            error.scope_name.as_deref().unwrap_or("top level"),
            error.reference_position,
        ))
        .with_line(error.origin.line.checked_sub(1)),
        ParseError::OutputSymbolValidation(_) => {
            unreachable!("aggregate output-symbol errors are expanded before scalar mapping")
        }
        ParseError::StartupDirectiveConflict(error) => Diagnostic::error(format!(
            "{}\nFirst startup mode: {} at {} · conflicting mode: {} at {}",
            error,
            error.first_kind.as_spice_directive(),
            error.first,
            error.conflicting_kind.as_spice_directive(),
            error.conflicting,
        ))
        .with_line(error.conflicting.line.checked_sub(1)),
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
    fn undefined_mutual_inductor_diagnostic_preserves_origin_and_identity() {
        let diagnostic = parse_error_diagnostic(
            rspice_core::netlist::ParseError::UndefinedMutualInductorReference(Box::new(
                rspice_core::netlist::UndefinedMutualInductorReferenceError {
                    origin: rspice_core::netlist::NetlistSourceLocation::in_file("bug75.cir", 12),
                    authored_coupling_name: "K3".into(),
                    canonical_coupling_name: "K3".into(),
                    qualified_coupling_name: "K3".into(),
                    authored_inductor_name: "L2".into(),
                    canonical_inductor_name: "L2".into(),
                    qualified_inductor_name: "L2".into(),
                    scope_name: None,
                    reference_position: 2,
                },
            )),
        );

        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert_eq!(diagnostic.line, Some(11));
        assert!(diagnostic.message.contains("Undefined inductor L2"));
        assert!(diagnostic.message.contains("Coupling: K3 (canonical K3)"));
        assert!(diagnostic.message.contains("scope: top level"));
        assert!(diagnostic.message.contains("reference position 2"));
    }

    #[test]
    fn output_symbol_diagnostics_preserve_origins_order_and_repetitions() {
        let error = rspice_core::netlist::ParseError::OutputSymbolValidation(Box::new(
            rspice_core::netlist::OutputSymbolValidationError {
                unresolved: vec![
                    rspice_core::netlist::UnresolvedOutputSymbol {
                        directive: rspice_core::netlist::OutputDirectiveKind::Print,
                        origin: rspice_core::netlist::NetlistSourceLocation::in_memory(7),
                        operator: "V".into(),
                        symbol: "missing".into(),
                        kind: rspice_core::netlist::OutputSymbolKind::Node,
                    },
                    rspice_core::netlist::UnresolvedOutputSymbol {
                        directive: rspice_core::netlist::OutputDirectiveKind::Measure,
                        origin: rspice_core::netlist::NetlistSourceLocation::in_memory(11),
                        operator: "I".into(),
                        symbol: "Rbogus".into(),
                        kind: rspice_core::netlist::OutputSymbolKind::Device,
                    },
                    rspice_core::netlist::UnresolvedOutputSymbol {
                        directive: rspice_core::netlist::OutputDirectiveKind::Measure,
                        origin: rspice_core::netlist::NetlistSourceLocation::in_memory(11),
                        operator: "I".into(),
                        symbol: "Rbogus".into(),
                        kind: rspice_core::netlist::OutputSymbolKind::Device,
                    },
                ],
            },
        ));

        let diagnostics = parse_error_diagnostics(error);

        assert_eq!(diagnostics.len(), 2);
        assert_eq!(diagnostics[0].line, Some(6));
        assert_eq!(diagnostics[1].line, Some(10));
        assert!(diagnostics[0].message.contains("node `missing`"));
        assert!(diagnostics[1].message.contains("device `Rbogus`"));
        assert_eq!(diagnostics[1].message.matches("device `Rbogus`").count(), 2);
    }

    #[test]
    fn startup_conflict_diagnostic_points_at_conflicting_card_and_preserves_first_origin() {
        let diagnostic =
            parse_error_diagnostic(rspice_core::netlist::ParseError::StartupDirectiveConflict(
                Box::new(rspice_core::netlist::StartupDirectiveConflictError {
                    first_kind: rspice_core::netlist::StartupDirectiveKind::Ic,
                    first: rspice_core::netlist::NetlistSourceLocation::in_file("deck.cir", 7),
                    conflicting_kind: rspice_core::netlist::StartupDirectiveKind::NodeSet,
                    conflicting: rspice_core::netlist::NetlistSourceLocation::in_file(
                        "included.cir",
                        11,
                    ),
                }),
            ));

        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert_eq!(diagnostic.line, Some(10));
        assert!(diagnostic.message.contains(".IC at deck.cir:7"));
        assert!(diagnostic.message.contains(".NODESET at included.cir:11"));
    }

    #[test]
    fn editor_keeps_completion_symbols_when_only_output_validation_fails() {
        let source = "output validation\n\
                      V1 1 0 1\n\
                      .PRINT OP V(missing)\n\
                      .END\n";

        let (diagnostics, symbols) = parse_buffer(source);

        assert!(
            symbols.is_some(),
            "semantic lint must retain completion state"
        );
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].line, Some(2));
        assert!(diagnostics[0].message.contains("node `missing` via V"));
        assert!(diagnostics[0].message.contains(".PRINT"));
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
        state.ui.netlist.active_document = super::super::ActiveNetlistDocument::OwnedSource;
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
        assert_eq!(
            state.workspace.netlist_source_path.as_deref(),
            Some(std::path::Path::new("imported/owned.cir"))
        );
        assert!(state.workspace.netlist_source_dirty);
        assert_eq!(state.ui.netlist.revision, 10);
        assert_eq!(state.ui.netlist.last_edit_time, 7.25);
        assert!(state.ui.netlist.edited_lines.contains(&1));
    }
}
