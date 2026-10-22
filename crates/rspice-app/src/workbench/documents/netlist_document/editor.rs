//! The netlist source surface: a selectable, immutable generated view or an
//! editable project-owned document, plus line numbers and diagnostic/diff
//! pips. Full diagnostics are projected by the Code inspector.
//!
//! Diagnostics come from one debounced parse of the buffer with the same
//! resolver the runner uses; the squiggle (underline), gutter pip, and
//! inspector all read that single vector.
//!
//! Every netlist renders through the rope-backed virtual editor, whatever its
//! size. An editor that swapped substrate at a byte threshold would change
//! folding, multi-selection, IME, undo, and completion behaviour at an
//! arbitrary boundary, and the small-document path was the one nothing
//! exercised.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use egui::Ui;
use rspice_core::abort_signal::{AbortSignal, NoAbort};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::Problem;
use crate::workbench::documents::text_editor_commands::{
    EditorSyntax, StandardCommand, show_go_to_line,
};
use crate::workbench::{AppState, MessageId};

use super::{
    DiagnosticSeverity, completion,
    diagnostics::{parser_diagnostics, unknown_reference_diagnostics},
    highlight,
};
#[cfg(test)]
use crate::state::netlist_diagnostics::line_column_for_span;
use crate::state::netlist_diagnostics::{Diagnostic, NetlistDiagnosticCollection};

/// Editor body font size (gutter follows it).
const FONT_SIZE: f32 = 11.0;
/// Width reserved for the mockup's line-number gutter.
const GUTTER_W: f32 = 47.0;
const CODE_LEFT_PADDING: f32 = 12.0;
const CODE_TOP_PADDING: i8 = 8;
/// Row pitch; the gutter, the squiggles, and a pointer hit-test all step by it.
const LINE_HEIGHT: f32 = 17.05;
/// Seconds of typing silence before the buffer re-parses.
const PARSE_DEBOUNCE: f64 = 0.35;
const SYNTHETIC_EDITOR_SOURCE: &str = "__rspice_netlist_editor__.cir";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IncludeAccess {
    NativeFilesystem,
    AuthenticatedBundleOnly,
}

const fn platform_include_access() -> IncludeAccess {
    if cfg!(target_arch = "wasm32") {
        IncludeAccess::AuthenticatedBundleOnly
    } else {
        IncludeAccess::NativeFilesystem
    }
}

/// Stable per-document id so cursor, selection, undo, and scroll state do not
/// leak between an owned root, generated root, include, or comparison tab.
pub(crate) fn editor_id(state: &AppState) -> egui::Id {
    let base = egui::Id::new("rspice.netlist.editor.text");
    let netlist = &state.code.netlist;
    if netlist.active_document == super::ActiveNetlistDocument::GeneratedDiff {
        return base.with("comparison");
    }
    let root = netlist
        .active_dependency_root
        .unwrap_or(netlist.active_document);
    let document_id = match root {
        super::ActiveNetlistDocument::Generated => state
            .model
            .netlist_artifacts
            .generated_document
            .as_ref()
            .map(|document| document.id()),
        super::ActiveNetlistDocument::OwnedSource => netlist
            .owned_document
            .as_ref()
            .or(state.model.workspace.content.netlist_document.as_ref())
            .map(|document| document.id()),
        super::ActiveNetlistDocument::GeneratedDiff | super::ActiveNetlistDocument::RunSnapshot => {
            None
        }
    };
    match (document_id, netlist.active_dependency_identity.as_deref()) {
        (Some(document_id), Some(identity)) => base.with((document_id, identity)),
        (Some(document_id), None) => base.with(document_id),
        (None, Some(identity)) => base.with(("dependency", identity)),
        (None, None) => base.with(match root {
            super::ActiveNetlistDocument::Generated => "generated",
            super::ActiveNetlistDocument::OwnedSource => "owned",
            super::ActiveNetlistDocument::GeneratedDiff => "comparison",
            super::ActiveNetlistDocument::RunSnapshot => "run-snapshot",
        }),
    }
}

/// Render the editor and diagnostics strip. Returns the source well, for its
/// right-click menu.
pub fn show(ui: &mut Ui, state: &mut AppState) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let editable = super::active_netlist_source_is_editable(state);
    let editor_id = editor_id(state);

    refresh_diagnostics(ui, state);

    // Popover keys (⇥, ↑↓, Esc) must be consumed before the editor so
    // an open popover owns them.
    let completion_keys = if editable {
        completion::consume_keys(ui, &state.code.netlist)
    } else {
        // Generated output owns no editor state. Close a popover that may have
        // survived a transition away from an owned source document.
        state.code.netlist.completion_open = false;
        (0, false, false)
    };

    // Document well backdrop.
    let well = ui.available_rect_before_wrap();
    ui.painter().rect_filled(well, 0.0, c.bg_inset);

    let font = theme::mono(FONT_SIZE, FontWeight::Regular);
    let diagnostics = Arc::clone(&state.code.netlist.diagnostics);
    let edited_lines = state.manual_run.edited_lines.clone();
    let cross_probe_line = (state.code.netlist.active_document
        == super::ActiveNetlistDocument::Generated)
        .then_some(state.code.netlist.cross_probe_line)
        .flatten()
        .map(|line| line.saturating_sub(1));

    // Take the buffer out so the layouter and the post-edit bookkeeping
    // don't fight over `state`.
    let mut buffer = std::mem::take(&mut state.model.simulation.source.netlist_content);
    let requested_line = state
        .code
        .netlist
        .requested_line
        .take()
        .or_else(|| show_go_to_line(ui, editor_id, &buffer));

    let diff_document =
        state.code.netlist.active_document == super::ActiveNetlistDocument::GeneratedDiff;

    let mut completion_changed = false;
    let changed;
    let cursor_line;
    let cursor_char_index;
    let selected_char_range;
    let hover_char_index;
    let editor_response;
    let anchor;
    let unanswered_history;

    {
        let messages = state.workbench.messages();
        let accessible_label = messages.text(if editable {
            MessageId::EditorProjectSpiceNetlistEditor
        } else {
            match state.code.netlist.active_document {
                super::ActiveNetlistDocument::Generated => {
                    MessageId::EditorGeneratedSpiceNetlistViewer
                }
                super::ActiveNetlistDocument::OwnedSource => {
                    MessageId::EditorProjectSpiceNetlistViewer
                }
                super::ActiveNetlistDocument::GeneratedDiff => {
                    MessageId::EditorGeneratedSpiceComparisonViewer
                }
                super::ActiveNetlistDocument::RunSnapshot => {
                    MessageId::EditorRunSnapshotSpiceNetlistViewer
                }
            }
        });
        let style = crate::workbench::documents::virtual_text_editor::VirtualEditorStyle {
            accessible_label: &accessible_label,
            messages,
            font: font.clone(),
            line_height: LINE_HEIGHT,
            gutter_width: GUTTER_W,
            code_left_padding: CODE_LEFT_PADDING,
            top_padding: CODE_TOP_PADDING as f32,
            background: c.bg_inset,
            hover_background: c.bg_hover,
            active_background: c.accent.gamma_multiply(0.075),
            selection_background: c.accent.gamma_multiply(0.28),
            text: c.text,
            text_dim: c.text_dim,
            text_faint: c.text_faint,
            border: c.border,
            accent: c.accent,
            error: c.err,
            warning: c.warn,
            edited_lines: &edited_lines,
            breakpoints: &[],
            breakpoints_enabled: false,
            current_line: None,
            cross_probe_line,
        };
        let revision = state.code.netlist.revision;
        let output = crate::workbench::documents::virtual_text_editor::show_virtual_text_editor(
            ui,
            editor_id,
            &mut buffer,
            revision,
            editable,
            EditorSyntax::Spice,
            requested_line,
            &style,
            |ui, text, _line, font| {
                let job = if diff_document {
                    highlight::diff_layout_job(text, font, &c)
                } else {
                    highlight::layout_job(text, font, &c, &[])
                };
                ui.fonts_mut(|fonts| fonts.layout_job(job))
            },
            |logical_line| {
                diagnostics
                    .severity_for_line(logical_line)
                    .and_then(diagnostic_problem)
            },
            |logical_line| {
                diagnostics.severity_for_line(logical_line).map(|severity| {
                    format!(
                        "{} diagnostic on this line.",
                        messages.text(match severity {
                            DiagnosticSeverity::Hint => MessageId::EditorAccessibleSeverityHint,
                            DiagnosticSeverity::Info => {
                                MessageId::EditorAccessibleSeverityInformation
                            }
                            DiagnosticSeverity::Warning => {
                                MessageId::EditorAccessibleSeverityWarning
                            }
                            DiagnosticSeverity::Error => MessageId::EditorAccessibleSeverityError,
                        })
                    )
                })
            },
        );
        changed = output.changed;
        cursor_line = output.cursor_line;
        cursor_char_index = output.cursor_char_index;
        selected_char_range = output.selected_char_range;
        hover_char_index = output.hover_char_index;
        unanswered_history = output.unanswered_history;
        editor_response = output.response.clone();
        anchor = completion::CompletionAnchor {
            focused: output.response.has_focus(),
            caret_char_index: output.cursor_char_index,
            screen_position: output.caret_anchor,
        };
    }

    let now = ui.input(|input| input.time);
    state.code.netlist.cursor_line = cursor_line;
    state.code.netlist.cursor_char_index = cursor_char_index;
    state.code.netlist.selected_byte_range = selected_char_range.and_then(|(start, end)| {
        let (start, end) = (start.min(end), start.max(end));
        (start < end).then(|| {
            let byte_at = |char_index: usize| {
                buffer
                    .char_indices()
                    .nth(char_index)
                    .map_or(buffer.len(), |(byte, _)| byte)
            };
            byte_at(start)..byte_at(end)
        })
    });

    // Completion is an edit and therefore exists only for project-owned source.
    if editable
        && let Some((start, end, text, caret)) = completion::show(
            ui,
            &mut state.code.netlist,
            anchor,
            &buffer,
            completion_keys,
        )
    {
        completion_changed = true;
        buffer.replace_range(start..end, &text);
        completion::place_caret(ui, editor_id, caret);
    }
    show_signature_help(ui, &mut state.code.netlist, editor_id, anchor);
    let hover_symbol =
        hover_char_index.and_then(|index| super::language::symbol_name_at(&buffer, index));
    let hover_value =
        hover_char_index.and_then(|index| super::language::value_hover_at(state, &buffer, index));

    // All editor writes pass through the ownership guard. Even if a future UI
    // path reports a change for read-only content, it cannot implicitly create
    // `workspace.netlist_source`.
    let source_changed = (changed || completion_changed)
        && commit_owned_source_edit(state, &buffer, cursor_line, now);
    if !source_changed {
        // The buffer was taken at the beginning of the frame; restore it on
        // unchanged and generated-document frames.
        state.model.simulation.source.netlist_content = buffer;
    }
    if source_changed && state.code.netlist.active_dependency_identity.is_none() {
        super::refresh_diff_pips_from_baseline(state);
    }
    // The editor's history starts again whenever the document is replaced
    // underneath it, so once it is spent the next older step is the
    // netlist's own: a replace-all or a rename.
    if let Some(step) = unanswered_history {
        let stepped = if step == StandardCommand::Undo {
            super::undo_netlist_edit(state)
        } else {
            super::redo_netlist_edit(state)
        };
        if let Err(error) = stepped {
            state.prompt(error);
        }
    }
    // A value the deck actually answers outranks the project symbol, and so
    // does the parser's word that a line is not read: the project symbol
    // would name that line's definition as if it bound something. A token
    // that resolves to neither still says so, so a hover never goes silent.
    if let Some(hover) = hover_value
        .as_ref()
        .filter(|hover| hover.value.is_some() || hover.not_read)
    {
        show_value_hover(&editor_response, hover);
    } else if let Some(symbol) = hover_symbol
        && let Some(hover) = super::language::symbol_hover(state, &symbol)
    {
        editor_response.clone().on_hover_ui_at_pointer(|ui| {
            ui.set_max_width(360.0);
            ui.label(egui::RichText::new(hover.title).strong());
            ui.label(hover.detail);
        });
    } else if let Some(hover) = hover_value.as_ref() {
        show_value_hover(&editor_response, hover);
    }
    editor_response
}

/// One mono line reading `token = value`, with the provenance or the reason
/// there is no value under it. No severity colour: a name the deck has not
/// bound yet is an ordinary state of a document being written.
fn show_value_hover(response: &egui::Response, hover: &super::language::NetlistValueHover) {
    response.clone().on_hover_ui_at_pointer(|ui| {
        ui.set_max_width(360.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            ui.label(egui::RichText::new(&hover.token).monospace().strong());
            if let Some(value) = &hover.value {
                ui.label(egui::RichText::new(format!("= {value}")).monospace());
            }
        });
        if let Some(detail) = &hover.detail {
            ui.weak(detail);
        }
    });
}

fn show_signature_help(
    ui: &mut Ui,
    netlist: &mut super::NetlistDocumentState,
    editor_id: egui::Id,
    anchor: completion::CompletionAnchor,
) {
    if !netlist.signature_help.open {
        return;
    }
    if netlist.signature_help.revision != netlist.revision
        || netlist.signature_help.cursor_char_index != anchor.caret_char_index
        || !anchor.focused
    {
        netlist.signature_help.open = false;
        return;
    }
    if ui
        .ctx()
        .input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
    {
        netlist.signature_help.open = false;
        return;
    }
    let Some(mut position) = anchor.screen_position else {
        netlist.signature_help.open = false;
        return;
    };
    let viewport = ui.ctx().content_rect().shrink(8.0);
    position.x = position
        .x
        .min(viewport.right() - 320.0)
        .max(viewport.left());
    position.y = (position.y + 6.0)
        .min(viewport.bottom() - 80.0)
        .max(viewport.top());
    egui::Area::new(editor_id.with("signature-help"))
        .order(egui::Order::Foreground)
        .fixed_pos(position)
        .show(ui.ctx(), |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.set_min_width(300.0);
                ui.set_max_width(360.0);
                ui.label(egui::RichText::new(&netlist.signature_help.title).strong());
                ui.label(&netlist.signature_help.detail);
                ui.weak("Signature help · Esc to close");
            });
        });
}

/// Commit a text/completion edit without promoting generated output to owned
/// source. Returns whether an existing owned source actually changed.
fn commit_owned_source_edit(
    state: &mut AppState,
    buffer: &str,
    cursor_line: usize,
    now: f64,
) -> bool {
    let committed = if state.code.netlist.active_dependency_identity.is_some() {
        super::replace_owned_dependency_source(state, buffer.to_owned())
    } else {
        super::replace_owned_source(state, buffer.to_owned())
    };
    if !committed {
        return false;
    }
    let netlist = &mut state.code.netlist;
    netlist.last_edit_time = now;
    if netlist.active_dependency_identity.is_none() {
        state.manual_run.edited_lines.insert(cursor_line);
    }
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
        DiagnosticSeverity::Hint => "hint · ",
        DiagnosticSeverity::Error => "error · ",
        DiagnosticSeverity::Warning => "warning · ",
        DiagnosticSeverity::Info => "info · ",
    }
}

/// The gutter mark a diagnostic earns: a warning or an error, and nothing for
/// a note or a hint.
fn diagnostic_problem(severity: DiagnosticSeverity) -> Option<Problem> {
    match severity {
        DiagnosticSeverity::Error => Some(Problem::Failure),
        DiagnosticSeverity::Warning => Some(Problem::Warning),
        DiagnosticSeverity::Info | DiagnosticSeverity::Hint => None,
    }
}

/// Debounced re-parse: the squiggles, pips, and strip all derive from
/// this one result.
fn refresh_diagnostics(ui: &Ui, state: &mut AppState) {
    let revision = state.code.netlist.revision;
    if state.code.netlist.diag_revision == Some(revision)
        && state.code.netlist.diag_library_folders == state.library_folders()
    {
        return;
    }
    let now = ui.input(|input| input.time);
    let settled = now - state.code.netlist.last_edit_time >= PARSE_DEBOUNCE;
    if !settled && state.code.netlist.diag_revision.is_some() {
        // Retain the prior records for Problems/history, but never decorate
        // newer bytes with stale source ranges.
        Arc::make_mut(&mut state.code.netlist.diagnostics).mark_all_stale();
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(120));
        return;
    }
    validate_visible_source(state, revision);
}

/// Validate the visible source as it stands and publish the result, bound to
/// `revision`.
fn validate_visible_source(state: &mut AppState, revision: u64) {
    state.code.netlist.diag_library_folders = state.library_folders().to_vec();
    if state.code.netlist.active_document == super::ActiveNetlistDocument::GeneratedDiff {
        Arc::make_mut(&mut state.code.netlist.diagnostics).clear();
        state.code.netlist.diag_revision = Some(revision);
        return;
    }

    let buffer = state.model.simulation.source.netlist_content.clone();
    let materialized = if state.code.netlist.active_dependency_identity.is_some() {
        Ok(buffer.clone())
    } else if state.code.netlist.active_document == super::ActiveNetlistDocument::OwnedSource {
        rspice_simulation::netlist_preparation::owned_source::compose_owned_netlist_execution_source(
            state.model.workspace.content.netlist_descriptor.as_ref(),
            state
                .model
                .workspace
                .content
                .netlist_document
                .as_ref()
                .and_then(|document| document.generated_artifact()),
            &buffer,
        )
    } else {
        Ok(buffer.clone())
    };
    let source_path = if let Some(dependency) = super::active_dependency(state) {
        dependency.locator().native_origin().map(Path::new)
    } else {
        match state.code.netlist.active_document {
            super::ActiveNetlistDocument::OwnedSource => {
                state.model.workspace.content.netlist_source_path.as_deref()
            }
            super::ActiveNetlistDocument::Generated => {
                state.model.schematic.session.current_file.as_deref()
            }
            super::ActiveNetlistDocument::GeneratedDiff
            | super::ActiveNetlistDocument::RunSnapshot => None,
        }
    };
    let materialized_external_line = materialized
        .as_ref()
        .ok()
        .and_then(|source| first_external_dependency_line(source));
    // A narrow override is parsed as the deck it executes as: the generated
    // base with the override spliced in. Every position the parse reports is
    // a position in that deck, and has to be carried back into the text the
    // editor shows and a fix edits.
    let composed = materialized
        .as_ref()
        .ok()
        .filter(|source| **source != buffer)
        .cloned();
    let in_buffer = |diagnostics: Vec<Diagnostic>| match composed.as_deref() {
        Some(composed) => super::diagnostics::in_authored_text(diagnostics, composed, &buffer),
        None => diagnostics,
    };
    let sealed_sources = if platform_include_access() == IncludeAccess::AuthenticatedBundleOnly
        && materialized_external_line.is_some()
    {
        match state.model.model_library_manager.seal_execution_sources() {
            Ok(sources) => Some(sources),
            Err(error) => {
                let netlist = &mut state.code.netlist;
                let mut diagnostics = in_buffer(vec![
                    Diagnostic::error(format!(
                        "The imported browser source bundle is invalid: {error}"
                    ))
                    .with_line(materialized_external_line),
                ]);
                let validation_id = crate::state::content_digest(&buffer).to_string();
                for diagnostic in &mut diagnostics {
                    diagnostic.bind_validation(revision, &validation_id);
                }
                netlist.diagnostics = Arc::new(bounded_netlist_diagnostics(
                    diagnostics,
                    &buffer,
                    revision,
                    &validation_id,
                ));
                netlist.diag_revision = Some(revision);
                return;
            }
        }
    } else {
        None
    };
    let include_search = state.include_search_chain();
    // Only a parse of the buffer itself, as its own root, says which of its
    // lines a run reads. A composed override's parse reads the lines of the
    // composed deck, and a dependency's sections are selected, and its
    // conditions decided, by the deck that includes it.
    let reads_buffer_as_root =
        composed.is_none() && state.code.netlist.active_dependency_identity.is_none();
    let (mut diagnostics, symbols, unread) = if buffer.trim().is_empty() {
        (Vec::new(), Some(Vec::new()), Some(Vec::new()))
    } else {
        match &materialized {
            Ok(source) => {
                let (diagnostics, symbols, unread) = parse_buffer_with_context(
                    source,
                    source_path,
                    &include_search,
                    platform_include_access(),
                    sealed_sources.as_ref(),
                    &NoAbort,
                );
                (
                    in_buffer(diagnostics),
                    symbols,
                    unread.filter(|_| reads_buffer_as_root),
                )
            }
            Err(error) => (vec![Diagnostic::error(error.clone())], None, None),
        }
    };
    diagnostics.extend(shadowed_include_diagnostics(state, &buffer));
    let validation_id = crate::state::content_digest(&buffer).to_string();
    for diagnostic in &mut diagnostics {
        diagnostic.bind_validation(revision, &validation_id);
    }
    let project_symbols = super::language::project_index(state)
        .map(|index| index.completion_symbols())
        .unwrap_or_default();
    let netlist = &mut state.code.netlist;
    netlist.diagnostics = Arc::new(bounded_netlist_diagnostics(
        diagnostics,
        &buffer,
        revision,
        &validation_id,
    ));
    if let Some(mut symbols) = symbols {
        let mut seen = symbols
            .iter()
            .map(|symbol| (symbol.kind, symbol.name.to_ascii_lowercase()))
            .collect::<std::collections::HashSet<_>>();
        symbols.extend(
            project_symbols
                .into_iter()
                .filter(|symbol| seen.insert((symbol.kind, symbol.name.to_ascii_lowercase()))),
        );
        netlist.symbols = symbols;
    }
    netlist.unread_source = unread.map(|unread| (revision, unread.into()));
    netlist.diag_revision = Some(revision);
}

fn bounded_netlist_diagnostics(
    diagnostics: Vec<Diagnostic>,
    buffer: &str,
    revision: u64,
    validation_id: &str,
) -> NetlistDiagnosticCollection {
    NetlistDiagnosticCollection::try_new(diagnostics, buffer).unwrap_or_else(|error| {
        let mut diagnostic = Diagnostic::current(
            "rspice.diagnostics",
            "DIAGNOSTIC-CAPACITY",
            DiagnosticSeverity::Error,
            "Netlist diagnostic collection exceeded the supported maximum",
        );
        diagnostic.details = error;
        diagnostic.bind_validation(revision, validation_id);
        NetlistDiagnosticCollection::try_new(vec![diagnostic], buffer).unwrap_or_default()
    })
}

/// Parse an in-memory source that owns no external dependency path.
///
/// Tests and dependency-free callers retain this convenience entry point;
/// the live editor uses [`parse_buffer_with_context`] so include resolution
/// receives the exact active-document origin used by execution.
#[cfg(test)]
fn parse_buffer(buffer: &str) -> (Vec<Diagnostic>, Option<Vec<completion::SymbolEntry>>) {
    let (diagnostics, symbols, _) = parse_buffer_with_context(
        buffer,
        None,
        &rspice_simulation::netlist_preparation::IncludeSearchChain::default(),
        platform_include_access(),
        None,
        &NoAbort,
    );
    (diagnostics, symbols)
}

/// One parse of the visible source: its findings, the symbols it defines,
/// and the lines of the source itself the parse did not read. The last two
/// are absent when the parse did not complete.
type BufferParse = (
    Vec<Diagnostic>,
    Option<Vec<completion::SymbolEntry>>,
    Option<Vec<rspice_core::netlist::UnreadSourceRegion>>,
);

fn parse_buffer_with_context(
    buffer: &str,
    source_path: Option<&Path>,
    include_search: &rspice_simulation::netlist_preparation::IncludeSearchChain,
    include_access: IncludeAccess,
    sealed_sources: Option<&crate::state::model_library::SealedModelExecutionSources>,
    abort: &dyn AbortSignal,
) -> BufferParse {
    let external_line = first_external_dependency_line(buffer);
    if source_path.is_none()
        && let Some(line) = external_line
    {
        return (
            vec![Diagnostic::error(
                "Relative .include/.inc/.lib sources require an imported deck origin before they can be resolved and sealed.",
            )
            .with_line(Some(line))],
            None,
            None,
        );
    }

    let parse_source = match editor_parse_source(source_path) {
        Ok(path) => path,
        Err(error) => return (vec![Diagnostic::error(error)], None, None),
    };
    let options = rspice_core::netlist::NetlistParseOptions {
        resource_limits: rspice_core::ResourceLimits::default(),
        ..Default::default()
    };
    let parsed = if external_line.is_none() {
        // A dependency-free deck needs no ambient filesystem or browser-file
        // capability. Parsing it in-memory is both sufficient and prevents
        // the authenticated-bundle policy from rejecting ordinary owned
        // source merely because the target is WASM.
        rspice_core::Netlist::parse_with_options_and_abort(buffer, options, abort)
    } else {
        match include_access {
            IncludeAccess::NativeFilesystem => {
                // One owner decides which engine entry point a host-file parse
                // goes through, so the editor lints the deck against exactly
                // the chain a run would seal it against.
                include_search.parse_with_abort(buffer, Some(&parse_source), options, abort)
            }
            IncludeAccess::AuthenticatedBundleOnly => {
                let Some(sealed_sources) = sealed_sources else {
                    return (
                        vec![Diagnostic::error(
                            "In the browser, external .include/.inc/.lib sources must come from an imported source bundle.",
                        )
                        .with_line(external_line)],
                        None,
                        None,
                    );
                };
                let bundle = match sealed_sources.bundle_for_root(&parse_source, buffer) {
                    Ok(bundle) => bundle,
                    Err(error) => {
                        return (
                            vec![Diagnostic::error(error).with_line(external_line)],
                            None,
                            None,
                        );
                    }
                };
                rspice_core::Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
                    buffer,
                    &parse_source,
                    bundle,
                    options,
                    abort,
                )
            }
        }
    };
    match parsed {
        Ok(netlist) => {
            let unread = netlist.unread_root_source().cloned().collect::<Vec<_>>();
            let mut diagnostics = parser_diagnostics(&netlist, Some(&parse_source));
            if let Err(error) = rspice_core::netlist::validate_output_requests(&netlist) {
                diagnostics.extend(parse_error_diagnostics_at(error, Some(&parse_source)));
            }
            diagnostics.extend(unknown_reference_diagnostics(buffer, &unread));
            (diagnostics, Some(harvest_symbols(&netlist)), Some(unread))
        }
        Err(rspice_core::netlist::ParseWithAbortError::Parse(error)) => (
            parse_error_diagnostics_at(error, Some(&parse_source)),
            None,
            None,
        ),
        Err(rspice_core::netlist::ParseWithAbortError::Aborted) => (
            vec![
                Diagnostic::current(
                    "rspice.netlist.parser",
                    "SPICE-VALIDATION-CANCELLED",
                    DiagnosticSeverity::Info,
                    "Live netlist diagnostics were cancelled before completion.",
                )
                .with_source_path(source_path.map(Path::to_path_buf)),
            ],
            None,
            None,
        ),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn editor_parse_source(source_path: Option<&Path>) -> Result<PathBuf, String> {
    let Some(path) = source_path else {
        return Ok(PathBuf::from(SYNTHETIC_EDITOR_SOURCE));
    };
    if path.is_absolute() {
        return Ok(path.canonicalize().unwrap_or_else(|_| path.to_path_buf()));
    }
    let current = std::env::current_dir()
        .map_err(|error| format!("Could not resolve the netlist source origin: {error}"))?;
    let absolute = current.join(path);
    Ok(absolute.canonicalize().unwrap_or(absolute))
}

#[cfg(target_arch = "wasm32")]
fn editor_parse_source(source_path: Option<&Path>) -> Result<PathBuf, String> {
    Ok(source_path
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from(SYNTHETIC_EDITOR_SOURCE)))
}

/// One warning per include whose relative name names a second file later in
/// the search chain.
///
/// The chain was walked by the engine when the closure was resolved; nothing
/// here re-walks it, so the Problems row, the navigator row and the deck that
/// actually ran cannot disagree about which file won.
fn shadowed_include_diagnostics(state: &AppState, buffer: &str) -> Vec<Diagnostic> {
    if state.code.code_workspace.include_resolutions.is_empty() {
        return Vec::new();
    }
    buffer
        .lines()
        .enumerate()
        .filter_map(|(line, raw)| {
            let requested = rspice_core::netlist::parse_include_directive(raw).or_else(|| {
                rspice_core::netlist::parse_lib_directive(raw)
                    .and_then(|(path, section)| section.is_some().then_some(path))
            })?;
            let resolution = state
                .code
                .code_workspace
                .include_resolutions
                .get(&requested)?;
            let shadowed = resolution.shadowed().next()?;
            Some(
                Diagnostic::current(
                    "rspice.netlist.include",
                    "SPICE-INCLUDE-SHADOWED",
                    DiagnosticSeverity::Warning,
                    format!(
                        "'{requested}' resolved to {} and shadows {}",
                        resolution.resolved().display(),
                        shadowed.path().display()
                    ),
                )
                .with_line(Some(line)),
            )
        })
        .collect()
}

fn first_external_dependency_line(buffer: &str) -> Option<usize> {
    buffer.lines().enumerate().find_map(|(line, raw)| {
        let include = rspice_core::netlist::parse_include_directive(raw).is_some();
        let external_lib = rspice_core::netlist::parse_lib_directive(raw)
            .is_some_and(|(_, section)| section.is_some());
        (include || external_lib).then_some(line)
    })
}

/// Convert a typed parse failure into editor diagnostics. Aggregate semantic
/// failures remain one row per authored card so distinct source origins are
/// localized without flooding the strip; ordered and repeated occurrences are
/// retained inside that card's message.
#[cfg(test)]
fn parse_error_diagnostics(error: rspice_core::netlist::ParseError) -> Vec<Diagnostic> {
    parse_error_diagnostics_at(error, None)
}

fn parse_error_diagnostics_at(
    error: rspice_core::netlist::ParseError,
    editor_source_path: Option<&Path>,
) -> Vec<Diagnostic> {
    let rspice_core::netlist::ParseError::OutputSymbolValidation(validation) = error else {
        return vec![parse_error_diagnostic_at(error, editor_source_path)];
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
            Diagnostic::error(format!(
                "Undefined output symbols in {directive} ({origin}): {}",
                entries.join(", ")
            ))
            .with_source_location(&origin, editor_source_path)
        })
        .collect()
}

#[cfg(test)]
fn parse_error_diagnostic(error: rspice_core::netlist::ParseError) -> Diagnostic {
    parse_error_diagnostic_at(error, None)
}

fn parse_error_diagnostic_at(
    error: rspice_core::netlist::ParseError,
    editor_source_path: Option<&Path>,
) -> Diagnostic {
    use rspice_core::netlist::ParseError;

    match error {
        ParseError::Syntax { line, message } => {
            // Parser lines are 1-based; `line == 0` means "unlocated".
            let (message, origin) = mapped_syntax_origin(message, line, editor_source_path);
            Diagnostic::error(message).with_source_location(&origin, editor_source_path)
        }
        ParseError::MissingSubcircuitEnds(error) => {
            Diagnostic::error(error.to_string())
                .with_source_location(&error.opened_at, editor_source_path)
        }
        ParseError::DuplicateSubcircuitPortBinding(error) => Diagnostic::error(format!(
            "{}\nSubcircuit: {} (read as {}) · instance: {} · formal {} · positions {} and {} · effective nodes {} and {}",
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
            "{}\nSubcircuit: {} (read as {}) · instance: {} · formal {} · position {} · effective node {}",
            error,
            error.subcircuit_name,
            error.canonical_subcircuit_name,
            error.qualified_instance_name,
            error.formal_port,
            error.position,
            error.actual_node,
        )),
        ParseError::UndefinedSubcircuit(error) => Diagnostic::error(format!(
            "{}\nSubcircuit: {} (read as {}) · instance: {} (read as {})",
            error,
            error.subcircuit_name,
            error.canonical_subcircuit_name,
            error.qualified_instance_name,
            error.canonical_instance_name,
        )),
        ParseError::MissingDeviceModel(error) => Diagnostic::error(format!(
            "{}\nDevice: {} (read as {}) · type: {}",
            error, error.device_name, error.canonical_device_name, error.device_type,
        )),
        ParseError::UndefinedMutualInductorReference(error) => Diagnostic::error(format!(
            "{}\nCoupling: {} (read as {}) · missing inductor: {} (read as {}) · scope: {} · reference position {}",
            error,
            error.qualified_coupling_name,
            error.canonical_coupling_name,
            error.qualified_inductor_name,
            error.canonical_inductor_name,
            error.scope_name.as_deref().unwrap_or("top level"),
            error.reference_position,
        ))
        .with_source_location(&error.origin, editor_source_path),
        ParseError::OutputSymbolValidation(error) => Diagnostic::error(error.to_string()),
        ParseError::OutputExpressionValidation(error) => Diagnostic::error(format!(
            "{}\nDirective: {} · expression: {{{}}}",
            error, error.directive, error.expression
        ))
        .with_source_location(&error.origin, editor_source_path),
        ParseError::StartupDirectiveConflict(error) => Diagnostic::error(format!(
            "{}\nFirst startup mode: {} at {} · conflicting mode: {} at {}",
            error,
            error.first_kind.as_spice_directive(),
            error.first,
            error.conflicting_kind.as_spice_directive(),
            error.conflicting,
        ))
        .with_source_location(&error.conflicting, editor_source_path),
        ParseError::DeviceInitialCondition(error) => {
            let origin = device_initial_condition_diagnostic_origin(&error);
            Diagnostic::error(error.to_string()).with_source_location(origin, editor_source_path)
        }
        other => Diagnostic::error(other.to_string()),
    }
}

fn mapped_syntax_origin(
    message: String,
    line: usize,
    editor_source_path: Option<&Path>,
) -> (String, rspice_core::netlist::NetlistSourceLocation) {
    if line > 0 {
        let marker = format!(":{line}: ");
        // Source-mapped core errors use `<path>:<line>: <detail>`. Search from
        // the right so Windows drive prefixes (`C:` and `\\?\C:`) and any
        // earlier colon-bearing path component remain part of the path.
        if let Some(marker_start) = message.rfind(&marker)
            && marker_start > 0
        {
            let source = canonical_diagnostic_source(PathBuf::from(&message[..marker_start]));
            let detail = message[marker_start + marker.len()..].to_owned();
            return (
                detail,
                rspice_core::netlist::NetlistSourceLocation::in_file(source, line),
            );
        }
    }
    let origin = match editor_source_path {
        Some(path) => rspice_core::netlist::NetlistSourceLocation::in_file(path, line),
        None => rspice_core::netlist::NetlistSourceLocation::in_memory(line),
    };
    (message, origin)
}

/// A source-mapped error shows an included file without the Windows verbatim
/// `\\?\` prefix; every other diagnostic names the file by its canonical path.
fn canonical_diagnostic_source(path: PathBuf) -> PathBuf {
    #[cfg(not(target_arch = "wasm32"))]
    if path.is_absolute() {
        return path.canonicalize().unwrap_or(path);
    }
    path
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
mod tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod library_folder_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod include_shadow_tests {
    use super::*;

    /// Two directories on the chain hold the same relative name. The winner
    /// and the shadow both come from the engine's own walk, so the warning
    /// cannot disagree with the deck that would run.
    fn shadowed_fixture(label: &str) -> (std::path::PathBuf, AppState) {
        let root = crate::fixture_root::canonical_temp_dir().join(format!(
            "rspice-include-shadow-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time after epoch")
                .as_nanos()
        ));
        let deck_dir = root.join("deck");
        let first = root.join("first");
        let second = root.join("second");
        for directory in [&deck_dir, &first, &second] {
            std::fs::create_dir_all(directory).expect("create shadow fixture directory");
        }
        std::fs::write(first.join("models.lib"), ".model DFIRST D(Is=1e-14)\n")
            .expect("write first library");
        std::fs::write(second.join("models.lib"), ".model DSECOND D(Is=1e-14)\n")
            .expect("write second library");

        let deck_path = deck_dir.join("top.cir");
        let source = "shadow deck\n.include models.lib\n.end\n";
        let chain = rspice_simulation::netlist_preparation::IncludeSearchChain::resolve(
            &[first, second],
            Some(root.as_path()),
        );
        let mut processor = rspice_core::netlist::IncludeProcessor::new(&deck_path);
        chain.apply_to(&mut processor);
        processor
            .expand_content(source, &deck_path)
            .expect("the chain resolves the include");

        let mut state = AppState::default();
        for dependency in processor.resolved_dependencies() {
            state.code.code_workspace.include_resolutions.insert(
                dependency.requested_path().to_owned(),
                dependency.resolution().clone(),
            );
        }
        (root, state)
    }

    #[test]
    fn a_shadowed_include_becomes_one_warning_bound_to_its_own_card() {
        let (root, state) = shadowed_fixture("warned");
        let diagnostics =
            shadowed_include_diagnostics(&state, "shadow deck\n.include models.lib\n.end\n");

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, DiagnosticSeverity::Warning);
        assert_eq!(diagnostics[0].line, Some(1));
        assert!(
            diagnostics[0].message.contains("first"),
            "the warning must name the file that won: {}",
            diagnostics[0].message
        );
        assert!(
            diagnostics[0].message.contains("second"),
            "the warning must name the file it shadows: {}",
            diagnostics[0].message
        );
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn an_include_with_one_candidate_produces_no_warning() {
        let (root, mut state) = shadowed_fixture("quiet");
        std::fs::remove_file(root.join("second").join("models.lib")).expect("remove the shadow");
        // Re-walk the chain now that only one candidate holds the name.
        let deck_path = root.join("deck").join("top.cir");
        let chain = rspice_simulation::netlist_preparation::IncludeSearchChain::resolve(
            &[root.join("first"), root.join("second")],
            Some(root.as_path()),
        );
        let mut processor = rspice_core::netlist::IncludeProcessor::new(&deck_path);
        chain.apply_to(&mut processor);
        processor
            .expand_content("shadow deck\n.include models.lib\n.end\n", &deck_path)
            .expect("the chain still resolves the include");
        state.code.code_workspace.include_resolutions.clear();
        for dependency in processor.resolved_dependencies() {
            state.code.code_workspace.include_resolutions.insert(
                dependency.requested_path().to_owned(),
                dependency.resolution().clone(),
            );
        }

        assert!(
            shadowed_include_diagnostics(&state, "shadow deck\n.include models.lib\n.end\n")
                .is_empty()
        );
        std::fs::remove_dir_all(&root).ok();
    }
}
