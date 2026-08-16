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
use crate::workbench::documents::text_editor_commands::{EditorSyntax, show_go_to_line};
use crate::workbench::{AppState, MessageId};

#[cfg(test)]
use super::diagnostics::line_column_for_span;
use super::{
    Diagnostic, DiagnosticSeverity, NetlistDiagnosticCollection, completion,
    diagnostics::{parser_diagnostics, unknown_reference_diagnostics},
    highlight,
};

/// Editor body font size (gutter follows it).
const FONT_SIZE: f32 = 11.0;
/// Width reserved for the mockup's line-number gutter.
const GUTTER_W: f32 = 47.0;
const CODE_LEFT_PADDING: f32 = 12.0;
const CODE_TOP_PADDING: i8 = 8;
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
    let netlist = &state.ui.netlist;
    if netlist.active_document == super::ActiveNetlistDocument::GeneratedDiff {
        return base.with("comparison");
    }
    let root = netlist
        .active_dependency_root
        .unwrap_or(netlist.active_document);
    let document_id = match root {
        super::ActiveNetlistDocument::Generated => netlist
            .generated_document
            .as_ref()
            .map(|document| document.id()),
        super::ActiveNetlistDocument::OwnedSource => netlist
            .owned_document
            .as_ref()
            .or(state.workspace.netlist_document.as_ref())
            .map(|document| document.id()),
        super::ActiveNetlistDocument::GeneratedDiff => None,
    };
    match (document_id, netlist.active_dependency_identity.as_deref()) {
        (Some(document_id), Some(identity)) => base.with((document_id, identity)),
        (Some(document_id), None) => base.with(document_id),
        (None, Some(identity)) => base.with(("dependency", identity)),
        (None, None) => base.with(match root {
            super::ActiveNetlistDocument::Generated => "generated",
            super::ActiveNetlistDocument::OwnedSource => "owned",
            super::ActiveNetlistDocument::GeneratedDiff => "comparison",
        }),
    }
}

/// Render the editor and diagnostics strip.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let editable = super::active_netlist_source_is_editable(state);
    let editor_id = editor_id(state);

    refresh_diagnostics(ui, state);

    // Popover keys (⇥, ↑↓, Esc) must be consumed before the editor so
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

    let font = theme::mono(FONT_SIZE, FontWeight::Regular);
    let diagnostics = Arc::clone(&state.ui.netlist.diagnostics);
    let edited_lines = state.ui.netlist.edited_lines.clone();
    let cross_probe_line = (state.ui.netlist.active_document
        == super::ActiveNetlistDocument::Generated)
        .then_some(state.ui.netlist.cross_probe_line)
        .flatten()
        .map(|line| line.saturating_sub(1));

    // Take the buffer out so the layouter and the post-edit bookkeeping
    // don't fight over `state`.
    let mut buffer = std::mem::take(&mut state.simulation.netlist_content);
    let requested_line = state
        .ui
        .netlist
        .requested_line
        .take()
        .or_else(|| show_go_to_line(ui, editor_id, &buffer));

    let diff_document =
        state.ui.netlist.active_document == super::ActiveNetlistDocument::GeneratedDiff;

    let mut completion_changed = false;
    let changed;
    let cursor_line;
    let cursor_char_index;
    let selected_char_range;
    let hover_char_index;
    let editor_response;
    let anchor;

    {
        let messages = state.ui.messages();
        let accessible_label = messages.text(if editable {
            MessageId::EditorProjectSpiceNetlistEditor
        } else {
            match state.ui.netlist.active_document {
                super::ActiveNetlistDocument::Generated => {
                    MessageId::EditorGeneratedSpiceNetlistViewer
                }
                super::ActiveNetlistDocument::OwnedSource => {
                    MessageId::EditorProjectSpiceNetlistViewer
                }
                super::ActiveNetlistDocument::GeneratedDiff => {
                    MessageId::EditorGeneratedSpiceComparisonViewer
                }
            }
        });
        let style = crate::workbench::documents::virtual_text_editor::VirtualEditorStyle {
            accessible_label: &accessible_label,
            messages,
            font: font.clone(),
            line_height: 17.05,
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
            information: c.info,
            edited_lines: &edited_lines,
            breakpoints: &[],
            breakpoints_enabled: false,
            current_line: None,
            cross_probe_line,
        };
        let revision = state.ui.netlist.revision;
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
                    .map(|severity| diagnostic_color(severity, &c))
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
        editor_response = output.response.clone();
        anchor = completion::CompletionAnchor {
            focused: output.response.has_focus(),
            caret_char_index: output.cursor_char_index,
            screen_position: output.caret_anchor,
        };
    }

    let now = ui.input(|input| input.time);
    state.ui.netlist.cursor_line = cursor_line;
    state.ui.netlist.cursor_char_index = cursor_char_index;
    state.ui.netlist.selected_byte_range = selected_char_range.and_then(|(start, end)| {
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
        && let Some((start, end, text, caret)) =
            completion::show(ui, &mut state.ui.netlist, anchor, &buffer, completion_keys)
    {
        completion_changed = true;
        buffer.replace_range(start..end, &text);
        completion::place_caret(ui, editor_id, caret);
    }
    show_signature_help(ui, &mut state.ui.netlist, editor_id, anchor);
    let hover_symbol =
        hover_char_index.and_then(|index| super::language::symbol_name_at(&buffer, index));

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
    if source_changed && state.ui.netlist.active_dependency_identity.is_none() {
        super::refresh_diff_pips_from_baseline(state);
    }
    if let Some(symbol) = hover_symbol
        && let Some(hover) = super::language::symbol_hover(state, &symbol)
    {
        editor_response.on_hover_ui_at_pointer(|ui| {
            ui.set_max_width(360.0);
            ui.label(egui::RichText::new(hover.title).strong());
            ui.label(hover.detail);
        });
    }
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
    let committed = if state.ui.netlist.active_dependency_identity.is_some() {
        super::replace_owned_dependency_source(state, buffer.to_owned())
    } else {
        super::replace_owned_source(state, buffer.to_owned())
    };
    if !committed {
        return false;
    }
    let netlist = &mut state.ui.netlist;
    netlist.last_edit_time = now;
    if netlist.active_dependency_identity.is_none() {
        netlist.edited_lines.insert(cursor_line);
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

fn diagnostic_color(
    severity: DiagnosticSeverity,
    c: &crate::ui::palette::Palette,
) -> egui::Color32 {
    match severity {
        DiagnosticSeverity::Hint => c.text_dim,
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
        // Retain the prior records for Problems/history, but never decorate
        // newer bytes with stale source ranges.
        Arc::make_mut(&mut state.ui.netlist.diagnostics).mark_all_stale();
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(120));
        return;
    }

    if state.ui.netlist.active_document == super::ActiveNetlistDocument::GeneratedDiff {
        Arc::make_mut(&mut state.ui.netlist.diagnostics).clear();
        state.ui.netlist.diag_revision = Some(revision);
        return;
    }

    let buffer = state.simulation.netlist_content.clone();
    let materialized = if state.ui.netlist.active_dependency_identity.is_some() {
        Ok(buffer.clone())
    } else if state.ui.netlist.active_document == super::ActiveNetlistDocument::OwnedSource {
        crate::workbench::workflows::netlist_workflow::compose_owned_netlist_execution_source(
            state, &buffer,
        )
    } else {
        Ok(buffer.clone())
    };
    let source_path = if let Some(dependency) = super::active_dependency(state) {
        dependency.locator().native_origin().map(Path::new)
    } else {
        match state.ui.netlist.active_document {
            super::ActiveNetlistDocument::OwnedSource => {
                state.workspace.netlist_source_path.as_deref()
            }
            super::ActiveNetlistDocument::Generated => state.schematic.current_file.as_deref(),
            super::ActiveNetlistDocument::GeneratedDiff => None,
        }
    };
    let materialized_external_line = materialized
        .as_ref()
        .ok()
        .and_then(|source| first_external_dependency_line(source));
    let sealed_sources = if platform_include_access() == IncludeAccess::AuthenticatedBundleOnly
        && materialized_external_line.is_some()
    {
        match state.model_library_manager.seal_execution_sources() {
            Ok(sources) => Some(sources),
            Err(error) => {
                let netlist = &mut state.ui.netlist;
                let mut diagnostics = vec![
                    Diagnostic::error(format!(
                        "Authenticated browser source bundle is invalid: {error}"
                    ))
                    .with_line(materialized_external_line),
                ];
                let validation_id = super::source_content_digest(&buffer).to_string();
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
    let (mut diagnostics, symbols) = if buffer.trim().is_empty() {
        (Vec::new(), Some(Vec::new()))
    } else {
        match materialized {
            Ok(source) => parse_buffer_with_context(
                &source,
                source_path,
                platform_include_access(),
                sealed_sources.as_ref(),
                &NoAbort,
            ),
            Err(error) => (vec![Diagnostic::error(error)], None),
        }
    };
    let validation_id = super::source_content_digest(&buffer).to_string();
    for diagnostic in &mut diagnostics {
        diagnostic.bind_validation(revision, &validation_id);
    }
    let project_symbols = super::language::project_index(state)
        .map(|index| index.completion_symbols())
        .unwrap_or_default();
    let netlist = &mut state.ui.netlist;
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
    parse_buffer_with_context(buffer, None, platform_include_access(), None, &NoAbort)
}

fn parse_buffer_with_context(
    buffer: &str,
    source_path: Option<&Path>,
    include_access: IncludeAccess,
    sealed_sources: Option<&crate::state::model_library::SealedModelExecutionSources>,
    abort: &dyn AbortSignal,
) -> (Vec<Diagnostic>, Option<Vec<completion::SymbolEntry>>) {
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
        );
    }

    let parse_source = match editor_parse_source(source_path) {
        Ok(path) => path,
        Err(error) => return (vec![Diagnostic::error(error)], None),
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
                rspice_core::Netlist::parse_with_path_and_options_and_abort(
                    buffer,
                    &parse_source,
                    options,
                    abort,
                )
            }
            IncludeAccess::AuthenticatedBundleOnly => {
                let Some(sealed_sources) = sealed_sources else {
                    return (
                        vec![Diagnostic::error(
                            "External .include/.inc/.lib sources require an authenticated imported source bundle in this browser session.",
                        )
                        .with_line(external_line)],
                        None,
                    );
                };
                let bundle = match sealed_sources.bundle_for_root(&parse_source, buffer) {
                    Ok(bundle) => bundle,
                    Err(error) => {
                        return (
                            vec![Diagnostic::error(error).with_line(external_line)],
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
            let mut diagnostics = parser_diagnostics(&netlist, Some(&parse_source));
            if let Err(error) = rspice_core::netlist::validate_output_requests(&netlist) {
                diagnostics.extend(parse_error_diagnostics_at(error, Some(&parse_source)));
            }
            diagnostics.extend(unknown_reference_diagnostics(buffer));
            (diagnostics, Some(harvest_symbols(&netlist)))
        }
        Err(rspice_core::netlist::ParseWithAbortError::Parse(error)) => {
            (parse_error_diagnostics_at(error, Some(&parse_source)), None)
        }
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
        ParseError::UndefinedSubcircuit(error) => Diagnostic::error(format!(
            "{}\nSubcircuit: {} (canonical {}) · instance: {} (canonical {})",
            error,
            error.subcircuit_name,
            error.canonical_subcircuit_name,
            error.qualified_instance_name,
            error.canonical_instance_name,
        )),
        ParseError::MissingDeviceModel(error) => Diagnostic::error(format!(
            "{}\nDevice: {} (canonical {}) · type: {}",
            error, error.device_name, error.canonical_device_name, error.device_type,
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
            let source = PathBuf::from(&message[..marker_start]);
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
    fn editor_diagnostic_pipeline_has_no_panic_shortcuts() {
        let production = crate::source_guard::production_source(include_str!("editor.rs"));
        for forbidden in [".expect(", ".unwrap(", "panic!(", "unreachable!("] {
            assert!(
                !production.contains(forbidden),
                "netlist editor production code contains panic shortcut {forbidden}"
            );
        }
    }

    /// The defect this exists for: completion used to be wired to a
    /// `TextEdit` branch that only ran for an *empty* buffer, so the popover
    /// was unreachable for every real deck while its own unit tests passed.
    /// Assert it against a document with content in it, through `show`, or
    /// the same class of regression comes back invisible.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn completion_reaches_a_document_that_has_content_in_it() {
        let mut state = AppState::default();
        let source = "test deck\n.tr";
        state.workspace.netlist_source = Some(source.to_owned());
        state.simulation.netlist_content = source.to_owned();
        state.ui.netlist.active_document = super::super::ActiveNetlistDocument::OwnedSource;
        state.ui.netlist.active_document_initialized = true;
        assert!(
            super::super::active_netlist_source_is_editable(&state),
            "fixture must be an editable owned document"
        );

        let context = egui::Context::default();
        crate::ui::Theme::default().apply(&context);
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 600.0),
            )),
            ..Default::default()
        };
        // Frame one builds the model and takes focus; the caret request is
        // applied on the frame after, which is when an anchor exists.
        for frame in 0..3 {
            let _ = context.clone().run_ui(input.clone(), |ui| {
                if frame == 0 {
                    let editor_id = editor_id(&state);
                    ui.ctx()
                        .memory_mut(|memory| memory.request_focus(editor_id));
                    crate::workbench::documents::text_editor_commands::queue_caret_char_index(
                        ui.ctx(),
                        editor_id,
                        source.chars().count(),
                    );
                }
                show(ui, &mut state);
            });
        }

        assert!(
            state.ui.netlist.completion_open,
            "the dot-command popover must open for a caret trailing `.tr` in a non-empty deck"
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    struct IncludeFixtureDir(PathBuf);

    #[cfg(not(target_arch = "wasm32"))]
    impl IncludeFixtureDir {
        fn new(label: &str) -> Self {
            use std::sync::atomic::{AtomicU64, Ordering};

            static NEXT_ID: AtomicU64 = AtomicU64::new(0);
            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            // A deck linted against an authenticated bundle is matched to the
            // sealed sources by path identity, and those were recorded through
            // `std::fs::canonicalize`. Rooting the fixture at an aliased
            // temporary directory would put the deck in a second identity
            // space that no sealed source can be found in.
            let path = crate::fixture_root::canonical_temp_dir().join(format!(
                "rspice-editor-include-{label}-{}-{id}",
                std::process::id()
            ));
            std::fs::create_dir_all(&path).expect("create include fixture directory");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl Drop for IncludeFixtureDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn parse_native_fixture(
        source: &str,
        root: &Path,
    ) -> (Vec<Diagnostic>, Option<Vec<completion::SymbolEntry>>) {
        parse_buffer_with_context(
            source,
            Some(root),
            IncludeAccess::NativeFilesystem,
            None,
            &NoAbort,
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn live_lint_resolves_nested_includes_from_each_owning_source() {
        let fixture = IncludeFixtureDir::new("nested");
        let root = fixture.path().join("root.cir");
        std::fs::write(
            fixture.path().join("child.inc"),
            ".include \"nested/grand.inc\"\n",
        )
        .expect("write child include");
        std::fs::create_dir_all(fixture.path().join("nested")).expect("create nested directory");
        std::fs::write(
            fixture.path().join("nested/grand.inc"),
            ".subckt CELL a b\nRIN a b 1k\n.ends CELL\n",
        )
        .expect("write grandchild include");
        let source = "nested include lint\n.include \"child.inc\"\nX1 out 0 CELL\n.end\n";

        let (diagnostics, symbols) = parse_native_fixture(source, &root);

        assert!(diagnostics.is_empty(), "{diagnostics:#?}");
        let symbols = symbols.expect("clean dependency closure harvests symbols");
        assert!(symbols.iter().any(|symbol| symbol.name == "CELL"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn live_lint_honors_external_lib_section_selection() {
        let fixture = IncludeFixtureDir::new("lib-section");
        let root = fixture.path().join("root.cir");
        std::fs::write(
            fixture.path().join("models.lib"),
            ".lib TT\n.model DTT D(Is=1e-14)\n.endl TT\n\
             .lib FF\n.enddata\n.endl FF\n",
        )
        .expect("write sectioned library");
        let source = "selected library lint\n.lib \"models.lib\" TT\nD1 out 0 DTT\n.end\n";

        let (diagnostics, symbols) = parse_native_fixture(source, &root);

        assert!(diagnostics.is_empty(), "{diagnostics:#?}");
        assert!(
            symbols
                .expect("selected section parses")
                .iter()
                .any(|symbol| symbol.name.eq_ignore_ascii_case("DTT"))
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn live_lint_reports_missing_include_at_root_directive() {
        let fixture = IncludeFixtureDir::new("missing");
        let root = fixture.path().join("root.cir");
        let source = "missing include lint\n.include \"missing.inc\"\n.end\n";

        let (diagnostics, symbols) = parse_native_fixture(source, &root);

        assert!(symbols.is_none());
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].line, Some(1));
        assert_eq!(diagnostics[0].source_line, Some(1));
        assert_eq!(diagnostics[0].source_path.as_deref(), Some(root.as_path()));
        assert!(diagnostics[0].message.contains("Include file not found"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn live_lint_reports_include_cycle_at_root_dependency_edge() {
        let fixture = IncludeFixtureDir::new("cycle");
        let root = fixture.path().join("root.cir");
        std::fs::write(fixture.path().join("a.inc"), ".include \"b.inc\"\n")
            .expect("write first cycle member");
        std::fs::write(fixture.path().join("b.inc"), ".include \"a.inc\"\n")
            .expect("write second cycle member");
        let source = "cyclic include lint\n.include \"a.inc\"\n.end\n";

        let (diagnostics, symbols) = parse_native_fixture(source, &root);

        assert!(symbols.is_none());
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].line, Some(1));
        assert_eq!(diagnostics[0].source_path.as_deref(), Some(root.as_path()));
        assert!(
            diagnostics[0]
                .message
                .to_ascii_lowercase()
                .contains("circular include")
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn live_lint_keeps_included_syntax_error_on_included_source() {
        let fixture = IncludeFixtureDir::new("included-syntax");
        let root = fixture.path().join("root.cir");
        let child = fixture.path().join("child.inc");
        std::fs::write(&child, "R1 out 0 1k FIRST SECOND\n")
            .expect("write invalid included source");
        let source = "included syntax lint\n.include \"child.inc\"\n.end\n";

        let (diagnostics, symbols) = parse_native_fixture(source, &root);
        let canonical_child = child.canonicalize().expect("canonicalize included source");

        assert!(symbols.is_none());
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].source_path.as_deref(),
            Some(canonical_child.as_path())
        );
        assert_eq!(diagnostics[0].source_line, Some(0));
        assert_eq!(
            diagnostics[0].line, None,
            "an included-source line must not paint the root editor gutter"
        );
        assert!(diagnostics[0].message.contains("Unexpected trailing token"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn live_lint_keeps_included_warning_off_the_root_gutter() {
        let fixture = IncludeFixtureDir::new("included-warning");
        let root = fixture.path().join("root.cir");
        let child = fixture.path().join("child.inc");
        std::fs::write(&child, "Rchild out 0\n").expect("write warning-producing include");
        let source = "included warning lint\n.include \"child.inc\"\n.end\n";

        let (diagnostics, symbols) = parse_native_fixture(source, &root);
        let canonical_child = child.canonicalize().expect("canonicalize included source");

        assert!(symbols.is_some());
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, DiagnosticSeverity::Warning);
        assert_eq!(
            diagnostics[0].source_path.as_deref(),
            Some(canonical_child.as_path())
        );
        assert_eq!(diagnostics[0].source_line, Some(0));
        assert_eq!(
            diagnostics[0].line, None,
            "an included warning must not paint the root editor gutter"
        );
    }

    #[test]
    fn live_lint_fails_closed_when_browser_cannot_resolve_include() {
        let source = "browser include lint\n.include \"models/device.lib\"\n.end\n";
        let (diagnostics, symbols) = parse_buffer_with_context(
            source,
            Some(Path::new("project/root.cir")),
            IncludeAccess::AuthenticatedBundleOnly,
            None,
            &NoAbort,
        );

        assert!(symbols.is_none());
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].line, Some(1));
        assert!(
            diagnostics[0]
                .message
                .contains("authenticated imported source bundle")
        );
    }

    #[test]
    fn live_lint_parses_dependency_free_browser_deck_without_a_bundle() {
        let source = "browser in-memory lint\nV1 input 0 5\nR1 input 0 1k\n.op\n.end\n";
        let (diagnostics, symbols) = parse_buffer_with_context(
            source,
            Some(Path::new("project/root.cir")),
            IncludeAccess::AuthenticatedBundleOnly,
            None,
            &NoAbort,
        );

        assert!(symbols.is_some());
        assert!(diagnostics.iter().all(|diagnostic| {
            !diagnostic
                .message
                .contains("authenticated imported source bundle")
        }));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn live_lint_parses_authenticated_browser_model_bundle_without_filesystem_fallback() {
        let fixture = IncludeFixtureDir::new("authenticated-browser");
        let root = fixture.path().join("root.cir");
        let model = fixture.path().join("device.lib");
        std::fs::write(&model, ".lib TT\n.model DSEALED D(Is=1e-14)\n.endl TT\n")
            .expect("write authenticated model library");
        let mut manager = crate::state::ModelLibraryManager::new();
        manager
            .load_library_file(&model, Some("TT"))
            .expect("import authenticated model library");
        let sealed = manager
            .seal_execution_sources()
            .expect("seal authenticated model library");
        let source = "authenticated browser lint\n.lib \"device.lib\" TT\nD1 out 0 DSEALED\n.end\n";

        let (diagnostics, symbols) = parse_buffer_with_context(
            source,
            Some(&root),
            IncludeAccess::AuthenticatedBundleOnly,
            Some(&sealed),
            &NoAbort,
        );

        assert!(diagnostics.is_empty(), "{diagnostics:#?}");
        assert!(
            symbols
                .expect("authenticated source parses")
                .iter()
                .any(|symbol| symbol.name.eq_ignore_ascii_case("DSEALED"))
        );
    }

    #[test]
    fn live_lint_uses_abort_aware_parser_contract() {
        let source = "cancelled lint\nR1 out 0 1k\n.end\n";
        let (diagnostics, symbols) = parse_buffer_with_context(
            source,
            None,
            IncludeAccess::NativeFilesystem,
            None,
            &rspice_core::abort_signal::ImmediateAbort,
        );

        assert!(symbols.is_none());
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, DiagnosticSeverity::Info);
        assert!(diagnostics[0].message.contains("cancelled"));
    }

    /// The gutter is a mockup contract, not a layout accident: line numbers,
    /// diagnostic pips, and the fold markers all measure from it.
    #[test]
    fn editor_owns_the_canonical_gutter_and_code_insets() {
        assert_eq!(GUTTER_W, 47.0);
        assert_eq!(CODE_LEFT_PADDING, 12.0);
        assert_eq!(GUTTER_W + CODE_LEFT_PADDING, 59.0);
        assert_eq!(CODE_TOP_PADDING, 8);
    }

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
    fn mapped_syntax_origin_accepts_windows_verbatim_drive_paths() {
        let source = r"\\?\C:\projects\rspice\models\child.inc";
        let (detail, origin) = mapped_syntax_origin(
            format!("{source}:17: Unexpected trailing token"),
            17,
            Some(Path::new(r"C:\projects\rspice\root.cir")),
        );

        assert_eq!(detail, "Unexpected trailing token");
        assert_eq!(origin.path.as_deref(), Some(Path::new(source)));
        assert_eq!(origin.line, 17);
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
    fn undefined_subcircuit_diagnostic_preserves_hierarchy_identity() {
        let diagnostic =
            parse_error_diagnostic(rspice_core::netlist::ParseError::UndefinedSubcircuit(
                Box::new(rspice_core::netlist::UndefinedSubcircuitError {
                    subcircuit_name: "missing".into(),
                    canonical_subcircuit_name: "MISSING".into(),
                    instance_name: "x1".into(),
                    canonical_instance_name: "X1".into(),
                    qualified_instance_name: "TOP.X1".into(),
                }),
            ));

        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert!(
            diagnostic
                .message
                .contains("Subcircuit: missing (canonical MISSING)")
        );
        assert!(diagnostic.message.contains("TOP.X1 (canonical X1)"));
    }

    #[test]
    fn missing_device_model_diagnostic_preserves_device_identity() {
        let diagnostic =
            parse_error_diagnostic(rspice_core::netlist::ParseError::MissingDeviceModel(
                Box::new(rspice_core::netlist::MissingDeviceModelError {
                    line: 4,
                    device_name: "d1".into(),
                    canonical_device_name: "D1".into(),
                    device_type: "DIODE".into(),
                }),
            ));

        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert!(diagnostic.message.contains("Device: d1 (canonical D1)"));
        assert!(diagnostic.message.contains("type: DIODE"));
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
        assert_eq!(diagnostic.line, None);
        assert_eq!(diagnostic.source_line, Some(11));
        assert_eq!(
            diagnostic.source_path.as_deref(),
            Some(Path::new("bug75.cir"))
        );
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
        assert_eq!(diagnostic.line, None);
        assert_eq!(diagnostic.source_line, Some(10));
        assert_eq!(
            diagnostic.source_path.as_deref(),
            Some(Path::new("included.cir"))
        );
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
    fn output_expression_diagnostic_is_localized_to_the_authored_request() {
        let error = rspice_core::netlist::ParseError::OutputExpressionValidation(Box::new(
            rspice_core::netlist::OutputExpressionValidationError {
                directive: rspice_core::netlist::OutputDirectiveKind::Print,
                origin: rspice_core::netlist::NetlistSourceLocation::in_file("deck.cir", 9),
                expression: "FABS(V(1))".into(),
                issue: rspice_core::netlist::OutputExpressionIssue::UnknownFunction {
                    function: "FABS".into(),
                },
            },
        ));
        let diagnostic = parse_error_diagnostic_at(error, Some(Path::new("deck.cir")));
        assert_eq!(diagnostic.line, Some(8));
        assert_eq!(diagnostic.source_line, Some(8));
        assert!(diagnostic.message.contains(".PRINT"));
        assert!(diagnostic.message.contains("FABS"));
    }

    #[test]
    fn diagnostic_strip_text_omits_fix_and_truncates_for_phone_width() {
        let source = "deck\nM1 d g s b nchh W=1u L=1u\n.model nch nmos\n.end\n";
        let mut diagnostic = Diagnostic::current(
            "rspice.netlist.semantic",
            "SPICE-UNKNOWN-MODEL",
            DiagnosticSeverity::Error,
            "Unknown model `nchh` in this deck.",
        );
        diagnostic.source_line = Some(1);
        diagnostic.span = Some(18..22);
        diagnostic.line = Some(1);
        diagnostic.column = Some(11);
        diagnostic.fix = Some(
            crate::workbench::documents::netlist_document::diagnostics::DiagnosticFix {
                label: "Replace with nch".to_string(),
                span: 18..22,
                replacement: "nch".to_string(),
            },
        );
        diagnostic.refresh_canonical_location();

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
