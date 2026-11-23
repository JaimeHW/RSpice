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

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use egui::Ui;
use rspice_app_runs::workbench::slots::runs::RunsState;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::netlist::IncludeResolution;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::Problem;
use crate::workbench::code::commands::NetlistEditorRequest;
use crate::workbench::documents::text_editor_commands::show_go_to_line;
use crate::workbench::state::WorkbenchSession;
use crate::workbench::{MessageCatalog, MessageId};

use super::{
    DiagnosticSeverity, completion,
    diagnostics::{parser_diagnostics, unknown_reference_diagnostics},
    highlight,
};
use crate::state::netlist_diagnostics::{
    CheckedInclude, DeckReading, Diagnostic, NetlistDiagnosticCollection,
};
use crate::workbench::documents::editor_font;

const CODE_LEFT_PADDING: f32 = 12.0;
/// Seconds of typing silence before the buffer re-parses.
const PARSE_DEBOUNCE: f64 = 0.35;

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
/// leak between the generated netlist, the Added lines, a deck, an include or
/// a run's input, over the model and the netlist editor's state.
pub fn editor_id_in(model: &super::Model, netlist: &super::NetlistDocumentState) -> egui::Id {
    let base = egui::Id::new("rspice.netlist.editor.text");
    let root = netlist.front.root_kind();
    let document_id = match root {
        super::NetlistRoot::Generated => model
            .netlist_artifacts
            .generated_document
            .as_ref()
            .map(|document| document.id()),
        super::NetlistRoot::Deck => netlist.open_deck.as_ref().map(|deck| deck.tab_id()),
        super::NetlistRoot::AddedLines
        | super::NetlistRoot::Comparison
        | super::NetlistRoot::RunInput => None,
    };
    match (document_id, netlist.front.include()) {
        (Some(document_id), Some(identity)) => base.with((document_id, identity)),
        (Some(document_id), None) => base.with(document_id),
        (None, Some(identity)) => base.with(("dependency", identity)),
        (None, None) => base.with(match root {
            super::NetlistRoot::Generated => "generated",
            super::NetlistRoot::AddedLines => "added-lines",
            super::NetlistRoot::Deck => "owned",
            super::NetlistRoot::Comparison => "comparison",
            super::NetlistRoot::RunInput => "run-snapshot",
        }),
    }
}

/// The editors of every document the page lists as a tab, over the model,
/// the netlist editor's state and the include tabs the reader has open: each
/// root, each open include, the run's deck and the editor on screen. A
/// comparison is a list of lines, not an editor. These are the editors whose text, caret and undo history are
/// kept while the reader moves between tabs.
pub fn open_editor_ids_in(
    model: &super::Model,
    netlist: &super::NetlistDocumentState,
    open_documents: &std::collections::HashSet<crate::workbench::state::WorkspaceDocumentId>,
) -> Vec<egui::Id> {
    use crate::workbench::state::WorkspaceDocumentId;
    let base = egui::Id::new("rspice.netlist.editor.text");
    let roots = [
        model
            .netlist_artifacts
            .generated_document
            .as_ref()
            .map(|document| document.id()),
        netlist.open_deck.as_ref().map(|deck| deck.tab_id()),
    ];
    let mut open = ["generated", "owned", "run-snapshot"]
        .map(|name| base.with(name))
        .to_vec();
    open.extend(roots.into_iter().flatten().map(|id| base.with(id)));
    open.extend(open_documents.iter().filter_map(|document| match document {
        WorkspaceDocumentId::NetlistDependency {
            root,
            logical_identity,
        } => Some(base.with((*root, logical_identity.as_str()))),
        _ => None,
    }));
    open.push(editor_id_in(model, netlist));
    open
}

/// Let go of the editor, and what the page read of its text, of every
/// document no longer in `open`: a closed include tab, or every document of
/// a project closed.
pub fn release_closed_editors(ctx: &egui::Context, open: &[egui::Id]) {
    let owner = egui::Id::new("rspice.netlist.editor.text");
    for closed in
        crate::workbench::documents::virtual_text_editor::retain_sessions(ctx, owner, open)
    {
        ctx.data_mut(|data| {
            data.remove::<highlight::SourceComments>(closed.with("source-comments"));
        });
    }
}

/// Render the editor and diagnostics strip over the model, the netlist
/// editor's state, where the page's includes resolved, the runs (for the
/// lines a run edited) and the workbench session. Returns the source well,
/// for its right-click menu.
pub fn show(
    ui: &mut Ui,
    model: &mut super::Model,
    netlist: &mut super::NetlistDocumentState,
    include_resolutions: &BTreeMap<String, IncludeResolution>,
    runs: &RunsState,
    workbench: &mut WorkbenchSession,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let editable = super::active_netlist_source_is_editable_in(model, netlist);
    let editor_id = editor_id_in(model, netlist);

    refresh_diagnostics(
        ui,
        model,
        netlist,
        include_resolutions,
        workbench.messages(),
    );
    super::references::settle_deck_references(model, netlist, workbench);
    // Hover, completion and the navigator read the project index of the deck
    // as it stands; its build starts once the text changes, off this thread.
    super::language::project_index_in(ui.ctx(), model, netlist);

    // Popover keys (⇥, ↑↓, Esc) must be consumed before the editor so
    // an open popover owns them, unless the editor's right-click menu is
    // open: then they are the menu's, Escape to close it and the arrows to
    // step its rows, and neither the popover nor the signature hint takes
    // them.
    let menu_open =
        crate::workbench::documents::virtual_text_editor::context_menu_open(ui.ctx(), editor_id);
    let completion_keys = if editable {
        completion::consume_keys(ui, netlist, menu_open)
    } else {
        // Generated output owns no editor state. Close a popover that may have
        // survived a transition away from an owned source document.
        netlist.completion_open = false;
        (0, false, false)
    };
    // An open signature hint answers Escape too, before the editor would
    // take the key to let go of focus.
    if netlist.signature_help.open
        && !menu_open
        && ui
            .ctx()
            .input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
    {
        netlist.signature_help.open = false;
    }
    super::value_at_caret::take_escape(ui.ctx(), editor_id, menu_open);

    // Document well backdrop.
    let well = ui.available_rect_before_wrap();
    ui.painter().rect_filled(well, 0.0, c.bg_inset);

    let text_size = editor_font::text_size(ui.ctx());
    let font = theme::mono(text_size, FontWeight::Regular);
    let diagnostics = Arc::clone(&netlist.diagnostics);
    // Each finding is marked under what it is about, read once per check.
    let problems = problem_marks_in(
        ui.ctx(),
        editor_id,
        netlist,
        &model.simulation.source.netlist_content,
    );
    let marks = problems
        .iter()
        .map(|problem| problem.mark.clone())
        .collect::<Vec<_>>();
    super::start_changed_lines(ui.ctx(), model, netlist, runs);
    let edited_lines = &runs.manual_run.edited_lines;
    let net_marks = super::cross_probe::net_marks(model, netlist).to_vec();
    let cross_probe_line = (netlist.front.shows_generated())
        .then_some(netlist.cross_probe_line)
        .flatten()
        .map(|line| line.saturating_sub(1));

    // How the engine reads the shown text decides its comments and the
    // marker Toggle comment writes.
    let shown_source = super::dialect::shown_source_in(model, netlist).unwrap_or_default();
    let comments_id = editor_id.with("source-comments");
    let mut source_comments = ui
        .ctx()
        .data_mut(|data| data.remove_temp::<highlight::SourceComments>(comments_id))
        .unwrap_or_default();

    // Take the buffer out so the layouter and the post-edit bookkeeping
    // don't fight over `model`.
    let mut buffer = std::mem::take(&mut model.simulation.source.netlist_content);
    let requested_line = netlist
        .requested_line
        .take()
        .or_else(|| show_go_to_line(ui, editor_id, || buffer.line_count(), workbench.messages()));

    let mut completion_changed = false;
    let changed;
    let cursor_line;
    let cursor_column;
    let cursor_char_index;
    let selected_byte_range;
    let hover_char_index;
    let editor_response;
    let anchor;
    let unanswered_history;
    let view;
    let paste_refused;
    // The first time a document draws after a launch, its editor shows it
    // as the last launch left it.
    let front = netlist.front.clone();
    super::session::hand_back_view(ui.ctx(), netlist, &front, editor_id);

    {
        let messages = workbench.messages();
        let accessible_label = messages.text(if netlist.front == super::NetlistTab::AddedLines {
            MessageId::EditorAddedLinesEditor
        } else if editable {
            MessageId::EditorProjectSpiceNetlistEditor
        } else {
            match netlist.front.root_kind() {
                super::NetlistRoot::Generated => MessageId::EditorGeneratedSpiceNetlistViewer,
                super::NetlistRoot::AddedLines => MessageId::EditorAddedLinesEditor,
                super::NetlistRoot::Deck => MessageId::EditorProjectSpiceNetlistViewer,
                super::NetlistRoot::Comparison => MessageId::EditorGeneratedSpiceComparisonViewer,
                super::NetlistRoot::RunInput => MessageId::EditorRunSnapshotSpiceNetlistViewer,
            }
        });
        let style = crate::workbench::documents::virtual_text_editor::VirtualEditorStyle {
            accessible_label: &accessible_label,
            messages,
            font: font.clone(),
            line_height: editor_font::line_height(text_size),
            gutter_width: editor_font::gutter_width(text_size),
            code_left_padding: CODE_LEFT_PADDING,
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
            edited_lines,
            cross_probe_line,
            marks: &marks,
            marks_outdated: false,
            occurrences: &net_marks,
        };
        // The buffer holds the rope the editor gave it at the editor's
        // revision, so the editor takes it back as it left it: shared, with
        // nothing copied or compared.
        let mut text = buffer.snapshot();
        let output = crate::workbench::documents::virtual_text_editor::show_virtual_text_editor(
            ui,
            editor_id,
            &mut text,
            editable,
            shown_source.syntax(),
            requested_line,
            &style,
            |ui, row, font| {
                // Coloured from the line's start, then cut to the part shown.
                let (line, comments) = source_comments.on_line(ui.ctx(), row, &shown_source);
                row.shown_part(&line, highlight::layout_job(&line, &comments, font, &c))
            },
            |logical_line| {
                diagnostics
                    .severity_for_line(logical_line)
                    .and_then(diagnostic_problem)
            },
            |logical_line| {
                diagnostics.severity_for_line(logical_line).map(|severity| {
                    messages.format(
                        MessageId::EditorAccessibleLineSeverity,
                        &[(
                            "severity",
                            &messages.text(match severity {
                                DiagnosticSeverity::Hint => MessageId::EditorAccessibleSeverityHint,
                                DiagnosticSeverity::Info => {
                                    MessageId::EditorAccessibleSeverityInformation
                                }
                                DiagnosticSeverity::Warning => {
                                    MessageId::EditorAccessibleSeverityWarning
                                }
                                DiagnosticSeverity::Error => {
                                    MessageId::EditorAccessibleSeverityError
                                }
                            }),
                        )],
                    )
                })
            },
        );
        changed = output.changed;
        if changed {
            buffer.set_snapshot(&text);
        }
        // Where the caret and the selection stand in bytes, read from the
        // editor's rope rather than counted from the deck's first character.
        let rope = text.rope();
        let byte_of = |char_index: usize| rope.char_to_byte(char_index.min(rope.len_chars()));
        let caret_byte = byte_of(output.cursor_char_index);
        selected_byte_range = output.selected_char_range.and_then(|(start, end)| {
            let (start, end) = (start.min(end), start.max(end));
            (start < end).then(|| byte_of(start)..byte_of(end))
        });
        cursor_line = output.cursor_line;
        cursor_column = output.cursor_column;
        cursor_char_index = output.cursor_char_index;
        hover_char_index = output.hover_char_index;
        unanswered_history = output.unanswered_history;
        view = output.view;
        paste_refused = output.paste_refused;
        editor_response = output.response.clone();
        anchor = completion::CompletionAnchor {
            focused: output.response.has_focus(),
            caret_char_index: output.cursor_char_index,
            caret_byte,
            screen_position: output.caret_anchor,
        };
    }
    ui.ctx()
        .data_mut(|data| data.insert_temp(comments_id, source_comments));

    super::session::keep_view(netlist, &front, view);
    let now = ui.input(|input| input.time);
    netlist.cursor_line = cursor_line;
    netlist.cursor_char_index = cursor_char_index;
    netlist.selected_byte_range = selected_byte_range;

    // The right-click menu, open over the editor or opening in this frame,
    // closes the completion list and the signature hint, so no two popups
    // overlap; the list stays closed until the next edit, as Escape leaves
    // it.
    if crate::workbench::documents::virtual_text_editor::context_menu_open(ui.ctx(), editor_id)
        || editor_response.secondary_clicked()
    {
        netlist.completion_open = false;
        netlist.completion_dismissed_at = Some(netlist.revision);
        netlist.signature_help.open = false;
    }

    // Completion is an edit and therefore exists only for project-owned source.
    if editable
        && let Some((start, end, text, caret)) = completion::show(
            ui,
            netlist,
            anchor,
            &buffer,
            completion_keys,
            workbench.messages(),
        )
    {
        completion_changed = true;
        buffer.replace_range(start..end, &text);
        completion::place_caret(ui, editor_id, caret);
    }
    show_signature_help(ui, netlist, editor_id, anchor, workbench.messages());

    // All editor writes pass through the ownership guard, which the shell
    // runs once the page has drawn, before anything else the frame raised.
    // Even if a future UI path reports a change for read-only content, it
    // cannot implicitly create `workspace.netlist_source`.
    if changed || completion_changed {
        workbench.outbox.push(NetlistEditorRequest::Commit {
            front: netlist.front.clone(),
            cursor_line,
            now,
        });
    }
    // The buffer was taken at the beginning of the frame. What the guard
    // refuses is shown until the next frame draws the project's deck again.
    model.simulation.source.netlist_content = buffer;
    // What the caret stands on is read once it has moved, so the editor's
    // menu and the commands it offers read the name rather than the deck.
    super::cross_probe::follow_caret(ui.ctx(), model, netlist, cursor_line, cursor_column);
    super::value_at_caret::show(
        ui,
        model,
        netlist,
        workbench,
        editor_id,
        anchor,
        editor_response.rect,
    );
    if paste_refused {
        let refusal = crate::workbench::documents::virtual_text_editor::paste_refused_text(
            ui.ctx(),
            workbench.messages(),
        );
        workbench.prompt(refusal);
    }
    // The editor's history starts again whenever the document is replaced
    // underneath it, so once it is spent the next older step is the
    // netlist's own: a replace-all or a rename.
    if let Some(step) = unanswered_history {
        workbench
            .outbox
            .push(NetlistEditorRequest::StepHistory(step));
    }
    // A marked problem under the pointer is what the reader is pointing at.
    // A value the deck actually answers outranks the project symbol, and so
    // does the parser's word that a line is not read: the project symbol
    // would name that line's definition as if it bound something. A token
    // that resolves to neither still says so, so a hover never goes silent.
    let hovered_problems = hover_char_index
        .map(|index| {
            let byte = buffer_byte_of(&model.simulation.source.netlist_content, index);
            problems
                .iter()
                .filter(|problem| {
                    problem.bytes.contains(&byte)
                        || (problem.bytes.is_empty() && problem.bytes.start == byte)
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if !hovered_problems.is_empty() {
        editor_response.clone().on_hover_ui_at_pointer(|ui| {
            ui.set_max_width(360.0);
            for problem in hovered_problems {
                let tone = match problem.severity {
                    DiagnosticSeverity::Error => c.err,
                    DiagnosticSeverity::Warning => c.warn,
                    DiagnosticSeverity::Info | DiagnosticSeverity::Hint => c.text_dim,
                };
                ui.label(egui::RichText::new(&problem.message).color(tone));
            }
        });
    } else if let Some(card) = hover_char_index.and_then(|index| {
        super::value_at_caret::hover_card_at(
            ui.ctx(),
            model,
            netlist,
            workbench.messages(),
            &[index],
        )
    }) {
        editor_response
            .clone()
            .on_hover_ui_at_pointer(|ui| card.ui(ui));
    }
    editor_response
}

/// The marks the editor drew last, with the revision and the findings they
/// were read from. The findings are held weakly, so their address names them
/// for as long as the marks are kept.
pub(super) type HeldProblemMarks = (
    u64,
    std::sync::Weak<NetlistDiagnosticCollection>,
    Arc<[super::diagnostics::ProblemMark]>,
);

/// Where the editor `editor_id` keeps the marks it drew.
fn problem_marks_id(editor_id: egui::Id) -> egui::Id {
    editor_id.with("problem-marks")
}

/// The findings of the shown text as the editor marks them, read once per
/// check of the text rather than once per frame.
fn problem_marks_in(
    ctx: &egui::Context,
    editor_id: egui::Id,
    netlist: &super::NetlistDocumentState,
    buffer: &crate::state::RevisedText,
) -> Arc<[super::diagnostics::ProblemMark]> {
    let id = problem_marks_id(editor_id);
    let current = |(revision, collection, _): &HeldProblemMarks| {
        *revision == netlist.revision
            && std::sync::Weak::ptr_eq(collection, &Arc::downgrade(&netlist.diagnostics))
    };
    if let Some(held) = ctx.data(|data| data.get_temp::<HeldProblemMarks>(id))
        && current(&held)
    {
        return held.2;
    }
    // The check that found them marked them on its own thread; marks are
    // worked out here only for findings that came another way.
    let marks = match netlist.checked_marks.clone() {
        Some(checked) if current(&checked) => checked.2,
        _ => {
            super::diagnostics::problem_marks(&netlist.diagnostics, buffer, netlist.revision).into()
        }
    };
    let held = (
        netlist.revision,
        Arc::downgrade(&netlist.diagnostics),
        Arc::clone(&marks),
    );
    ctx.data_mut(|data| data.insert_temp::<HeldProblemMarks>(id, held));
    marks
}

/// The byte of `text` its character `char_index` starts at.
fn buffer_byte_of(text: &str, char_index: usize) -> usize {
    text.char_indices()
        .nth(char_index)
        .map_or(text.len(), |(byte, _)| byte)
}

fn show_signature_help(
    ui: &mut Ui,
    netlist: &mut super::NetlistDocumentState,
    editor_id: egui::Id,
    anchor: completion::CompletionAnchor,
    messages: crate::workbench::MessageCatalog,
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
                ui.weak(messages.text(crate::workbench::MessageId::NetlistSignatureHelpHint));
            });
        });
}

/// Commit the text the editor left as the model's netlist source at the
/// end of its frame over `front`, typed on `cursor_line` at input time
/// `now`, and mark the lines it changed against the run's baseline when the
/// deck itself is in front. An edit to a document no longer in front is
/// not the one in front's to take.
pub fn commit_edit(
    model: &mut super::Model,
    netlist: &mut super::NetlistDocumentState,
    runs: &mut RunsState,
    front: &super::NetlistTab,
    cursor_line: usize,
    now: f64,
) {
    if netlist.front != *front {
        return;
    }
    // Only the Added lines, the open deck and its open includes take an
    // edit: the generated netlist, a comparison and a run's input are
    // read-only.
    if netlist.front == super::NetlistTab::AddedLines {
        let lines = model.simulation.source.netlist_content.as_str().to_owned();
        if super::edit_added_lines(model, netlist, lines) {
            netlist.last_edit_time = now;
            netlist.typed_revision = Some(netlist.revision);
        }
        return;
    }
    if !super::keep_editor_text(model, netlist) {
        return;
    }
    netlist.last_edit_time = now;
    netlist.typed_revision = Some(netlist.revision);
    if netlist.front.include().is_none() {
        // The line typed on is marked at once; the diff that marks the rest
        // is worked out away from the frame.
        runs.manual_run.edited_lines.insert(cursor_line);
        super::refresh_diff_pips_from_baseline_in(model, netlist, runs);
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

/// Keep the findings of the text before for Problems and history, but mark
/// them stale so they never decorate newer bytes. Findings already marked
/// are left as they are: every frame until the next check reports asks, and
/// a collection written again is a new one to whatever was read from it,
/// such as the editor's marks.
fn mark_findings_stale(diagnostics: &mut Arc<NetlistDiagnosticCollection>) {
    let unmarked = diagnostics.iter().any(|diagnostic| {
        diagnostic.canonical.currentness
            != rspice_vocabulary::diagnostics::DiagnosticCurrentness::StaleSource
    });
    if unmarked {
        Arc::make_mut(diagnostics).mark_all_stale();
    }
}

/// Debounced check of the shown source: the squiggles, pips, and strip all
/// derive from its one result. Once typing pauses, the text as it stands is
/// checked away from the editor's thread. Until that check reports, the
/// findings of the text before are kept, marked stale so they never mark the
/// newer text; a check of a text the editor has moved past is stopped and
/// what it found is dropped. Findings are worded in `messages`.
fn refresh_diagnostics(
    ui: &Ui,
    model: &super::Model,
    netlist: &mut super::NetlistDocumentState,
    include_resolutions: &BTreeMap<String, IncludeResolution>,
    messages: crate::workbench::MessageCatalog,
) {
    let revision = netlist.revision;
    let folders = model.library_folders();
    if let Some(checked) = netlist.live_check.take_finished() {
        receive_check(netlist, folders, checked);
    }
    if netlist.diag_revision == Some(revision) && netlist.diag_library_folders == folders {
        return;
    }
    netlist.live_check.stop_unless(revision, folders);
    if netlist.diag_revision.is_some() {
        mark_findings_stale(&mut netlist.diagnostics);
    }
    if netlist.live_check.is_checking(revision, folders) {
        return;
    }
    let now = ui.input(|input| input.time);
    let settled = now - netlist.last_edit_time >= PARSE_DEBOUNCE;
    if !settled && netlist.diag_revision.is_some() {
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(120));
        return;
    }
    let check = SourceCheck::of(model, netlist, include_resolutions, revision, messages);
    let wake = ui.ctx().clone();
    netlist
        .live_check
        .start(check, move || wake.request_repaint());
    // The application's tests read what a frame's text was found to hold in
    // that same frame.
    #[cfg(any(test, feature = "test-support"))]
    if !netlist.live_check.detached {
        netlist.live_check.wait();
    }
    if let Some(checked) = netlist.live_check.take_finished() {
        receive_check(netlist, folders, checked);
    }
}

/// Bring the findings to the shown source as it stands, whichever workspace
/// is in front: a check of it already running is waited for, else it is
/// checked here. Run and Check and netlist call this, so what Problems
/// lists after the press is what the text they read holds, never what an
/// older text held. Findings are worded in `messages`.
pub(crate) fn bring_findings_current(
    model: &super::Model,
    netlist: &mut super::NetlistDocumentState,
    include_resolutions: &BTreeMap<String, IncludeResolution>,
    messages: crate::workbench::MessageCatalog,
) {
    let revision = netlist.revision;
    let folders = model.library_folders();
    netlist.live_check.stop_unless(revision, folders);
    netlist.live_check.wait();
    if let Some(checked) = netlist.live_check.take_finished() {
        receive_check(netlist, folders, checked);
    }
    if netlist.diag_revision == Some(revision) && netlist.diag_library_folders == folders {
        return;
    }
    let checked = SourceCheck::of(model, netlist, include_resolutions, revision, messages)
        .run(&rspice_core::abort_signal::NoAbort);
    publish_check(netlist, checked);
}

/// Take what a check found: shown when it read the text as it stands
/// through the library folders as they stand, dropped when either has moved
/// on since.
fn receive_check(
    netlist: &mut super::NetlistDocumentState,
    folders: &[PathBuf],
    checked: CheckedSource,
) {
    if checked.revision == netlist.revision && checked.library_folders == folders {
        publish_check(netlist, checked);
    }
}

/// Validate the visible source as it stands and publish the result, bound to
/// `revision`, over the model, the netlist editor's state and where the
/// page's includes resolved, its findings worded in `messages`. It runs
/// here, on the caller's thread.
#[cfg(test)]
fn validate_visible_source_in(
    model: &super::Model,
    netlist: &mut super::NetlistDocumentState,
    include_resolutions: &BTreeMap<String, IncludeResolution>,
    revision: u64,
    messages: crate::workbench::MessageCatalog,
) {
    let checked = SourceCheck::of(model, netlist, include_resolutions, revision, messages)
        .run(&rspice_core::abort_signal::NoAbort);
    publish_check(netlist, checked);
}

/// Make what a check found the findings, symbols and unread lines of the
/// text it read.
fn publish_check(netlist: &mut super::NetlistDocumentState, checked: CheckedSource) {
    netlist.diag_library_folders = checked.library_folders;
    // A check that could not read the text keeps the reading of the last one
    // that could, so a slip while typing does not flip the chip.
    let reading = checked.reading.or_else(|| netlist.diagnostics.reading());
    netlist.diagnostics = Arc::new(
        checked
            .diagnostics
            .with_reading(reading)
            .with_includes(checked.includes.map_or_else(|| Arc::from([]), Arc::from)),
    );
    if let Some(symbols) = checked.symbols {
        netlist.symbols = symbols;
    }
    if let Some(unread) = checked.unread {
        netlist.unread_source = unread.map(|unread| (checked.revision, unread.into()));
    }
    netlist.dialect_hint = checked.dialect_hint;
    netlist.checked_marks = Some((
        checked.revision,
        Arc::downgrade(&netlist.diagnostics),
        checked.marks,
    ));
    netlist.missing_file_lines = checked.missing_file_lines;
    netlist.diag_revision = Some(checked.revision);
}

/// The live check of the shown source, run away from the editor's thread.
/// One runs at a time: the check of a newer text stops the one before, and
/// what a stopped check found is never received. The browser build has no
/// threads and checks in place.
#[derive(Clone, Default)]
pub(super) struct LiveCheck {
    #[cfg(not(target_arch = "wasm32"))]
    running: Option<RunningCheck>,
    /// A check that finished before a frame took it.
    finished: Option<CheckedSource>,
    /// Whether a frame leaves the check it starts running, as the
    /// application does. A test that leaves it so reads what was found
    /// once it reports; the rest read it in the frame that starts it.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) detached: bool,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
struct RunningCheck {
    revision: u64,
    library_folders: Vec<PathBuf>,
    stop: Arc<rspice_core::abort_signal::AtomicAbort>,
    receiver: Arc<std::sync::Mutex<std::sync::mpsc::Receiver<CheckedSource>>>,
}

impl std::fmt::Debug for LiveCheck {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(not(target_arch = "wasm32"))]
        let running = self.running.as_ref().map(|running| running.revision);
        #[cfg(target_arch = "wasm32")]
        let running = None::<u64>;
        formatter
            .debug_struct("LiveCheck")
            .field("running", &running)
            .field(
                "finished",
                &self.finished.as_ref().map(|done| done.revision),
            )
            .finish()
    }
}

impl LiveCheck {
    /// Whether the check of `revision` through `folders` is running.
    fn is_checking(&self, revision: u64, folders: &[PathBuf]) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.running.as_ref().is_some_and(|running| {
                running.revision == revision && running.library_folders == folders
            })
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (revision, folders);
            false
        }
    }

    /// Stop a running check of any text but `revision` through `folders`.
    fn stop_unless(&mut self, revision: u64, folders: &[PathBuf]) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(running) = self
            .running
            .take_if(|running| running.revision != revision || running.library_folders != folders)
        {
            running.stop.set();
        }
        #[cfg(target_arch = "wasm32")]
        let _ = (revision, folders);
    }

    /// Start `check`, stopping any check running; `wake` is called when it
    /// is done.
    fn start(&mut self, check: SourceCheck, wake: impl FnOnce() + Send + 'static) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::sync::{Mutex, mpsc};

            if let Some(running) = self.running.take() {
                running.stop.set();
            }
            let stop = Arc::new(rspice_core::abort_signal::AtomicAbort::new());
            let (sender, receiver) = mpsc::channel();
            let (revision, library_folders) = (check.revision, check.library_folders.clone());
            let stopped = Arc::clone(&stop);
            let spawned = std::thread::Builder::new()
                .name("netlist-check".to_owned())
                .spawn(move || {
                    if sender.send(check.run(&stopped)).is_ok() {
                        wake();
                    }
                });
            if spawned.is_ok() {
                self.running = Some(RunningCheck {
                    revision,
                    library_folders,
                    stop,
                    receiver: Arc::new(Mutex::new(receiver)),
                });
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = wake;
            self.finished = Some(check.run(&rspice_core::abort_signal::NoAbort));
        }
    }

    /// What a check found, once it is done.
    fn take_finished(&mut self) -> Option<CheckedSource> {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(running) = self.running.take() {
            use std::sync::mpsc::TryRecvError;

            match running
                .receiver
                .lock()
                .map_or(Err(TryRecvError::Disconnected), |receiver| {
                    receiver.try_recv()
                }) {
                Ok(checked) => self.finished = Some(checked),
                Err(TryRecvError::Empty) => self.running = Some(running),
                Err(TryRecvError::Disconnected) => {}
            }
        }
        self.finished.take()
    }

    /// Wait for the running check to finish.
    fn wait(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(running) = self.running.take() {
            let received = running
                .receiver
                .lock()
                .ok()
                .and_then(|receiver| receiver.recv().ok());
            if received.is_some() {
                self.finished = received;
            }
        }
    }
}

/// What a check found in one revision of the shown source.
#[derive(Clone)]
struct CheckedSource {
    revision: u64,
    library_folders: Vec<PathBuf>,
    diagnostics: NetlistDiagnosticCollection,
    /// The symbols the source defines; absent when the parse did not
    /// complete, so the last ones found stand.
    symbols: Option<Vec<completion::SymbolEntry>>,
    /// The lines of the source the parse did not read; absent when the
    /// check stopped before the parse and says nothing of them.
    unread: Option<Option<Vec<rspice_core::netlist::UnreadSourceRegion>>>,
    /// How the engine read the text; absent when the check could not read
    /// it, so the last reading stands.
    reading: Option<DeckReading>,
    /// The `.include` and `.lib` lines of the deck the check read, each with
    /// the file it found; absent when the check did not read the buffer as
    /// its own deck.
    includes: Option<Vec<CheckedInclude>>,
    /// What the buffer, read as a root deck, says of the program that wrote
    /// it; absent for the Added lines and a dependency.
    dialect_hint: Option<super::dialect::DialectHint>,
    /// The findings as the editor marks them, worked out here so the frame
    /// the check lands in does not read the text for them.
    marks: Arc<[super::diagnostics::ProblemMark]>,
    /// The lines the findings of files the check could not read point at,
    /// by their zero-based line, as it read them.
    missing_file_lines: Arc<[(usize, String)]>,
}

/// What a check of the shown source reads, taken from the model at one
/// revision. Everything it needs travels with it, so it runs away from the
/// editor's thread.
struct SourceCheck {
    revision: u64,
    library_folders: Vec<PathBuf>,
    /// The text checked, shared with the editor rather than copied.
    buffer: crate::state::RevisedText,
    deck: CheckedDeck,
    source_path: Option<PathBuf>,
    include_search: rspice_simulation::netlist_preparation::IncludeSearchChain,
    sealed_sources:
        Option<Result<crate::state::model_library::SealedModelExecutionSources, String>>,
    /// Only a parse of the buffer itself, as its own root, says which of its
    /// lines a run reads. A dependency's sections are selected, and its
    /// conditions decided, by the deck that includes it.
    is_dependency: bool,
    /// For a deck the project keeps as text: where the engine reads the
    /// shown file, and every file of the deck there, read through them
    /// rather than any disk.
    stored: Option<(PathBuf, Vec<(PathBuf, String)>)>,
    /// Whether the text is the Added lines, which are checked for the lines
    /// the generated netlist cannot take rather than parsed as a deck.
    added_lines: bool,
    include_resolutions: BTreeMap<String, IncludeResolution>,
    /// The name the page shows the text under: what a source with no file
    /// is read as, and what its findings name.
    shown_path: Option<PathBuf>,
    /// The words the findings the application states itself are put in.
    messages: crate::workbench::MessageCatalog,
}

/// The deck a check parses.
enum CheckedDeck {
    /// The buffer, as its own deck.
    Buffer,
    /// The deck as already composed: `None` when it is the buffer itself.
    Composed(Result<Option<String>, String>),
}

impl CheckedDeck {
    /// The deck as composed: `None` when it is the buffer itself.
    fn compose(self) -> Result<Option<String>, String> {
        match self {
            Self::Buffer => Ok(None),
            Self::Composed(composed) => composed,
        }
    }
}

impl SourceCheck {
    /// The check of the visible source at `revision`, over the model, the
    /// netlist editor's state and where the page's includes resolved, its
    /// findings worded in `messages`.
    fn of(
        model: &super::Model,
        netlist: &super::NetlistDocumentState,
        include_resolutions: &BTreeMap<String, IncludeResolution>,
        revision: u64,
        messages: crate::workbench::MessageCatalog,
    ) -> Self {
        let buffer = model.simulation.source.netlist_content.clone();
        let is_dependency = netlist.front.include().is_some();
        let added_lines = netlist.front == super::NetlistTab::AddedLines;
        let deck = CheckedDeck::Buffer;
        let source_path = if let Some(dependency) = super::active_dependency_in(model, netlist) {
            dependency.locator().native_origin().map(PathBuf::from)
        } else {
            match netlist.front.root_kind() {
                super::NetlistRoot::Deck => {
                    super::active_deck_file_in(netlist).map(|file| file.path.clone())
                }
                super::NetlistRoot::Generated => model.schematic.session.current_file.clone(),
                super::NetlistRoot::AddedLines
                | super::NetlistRoot::Comparison
                | super::NetlistRoot::RunInput => None,
            }
        };
        let stored = (netlist.front.root_kind() == super::NetlistRoot::Deck)
            .then(|| crate::workbench::documents::deck_files::stored_sources(netlist, &buffer))
            .flatten();
        // The browser build reads included sources from the libraries it
        // sealed, and checks in place: whether the deck names one is read
        // here, where the libraries can be sealed. A deck kept as text reads
        // them beside its own files, on the desktop too.
        let (deck, sealed_sources) = if stored.is_some() {
            let sealed = first_external_dependency_line(&buffer)
                .map(|_| model.model_library_manager.seal_execution_sources());
            (deck, sealed)
        } else if platform_include_access() == IncludeAccess::AuthenticatedBundleOnly {
            let composed = deck.compose();
            let names_external = match &composed {
                Ok(Some(deck)) => first_external_dependency_line(deck).is_some(),
                Ok(None) => first_external_dependency_line(&buffer).is_some(),
                Err(_) => false,
            };
            let sealed =
                names_external.then(|| model.model_library_manager.seal_execution_sources());
            (CheckedDeck::Composed(composed), sealed)
        } else {
            (deck, None)
        };
        Self {
            revision,
            library_folders: model.library_folders().to_vec(),
            buffer,
            deck,
            source_path,
            include_search: model.include_search_chain(),
            sealed_sources,
            is_dependency,
            stored,
            added_lines,
            include_resolutions: include_resolutions.clone(),
            shown_path: super::dialect::shown_source_in(model, netlist)
                .map(|shown| shown.path)
                .filter(|path| !path.as_os_str().is_empty()),
            messages,
        }
    }

    /// Check the source; `stop` ends the parse early once the text has moved
    /// on. What it finds is marked as the editor marks it, here rather than
    /// on the frame the check lands in.
    fn run(self, stop: &dyn AbortSignal) -> CheckedSource {
        let buffer = self.buffer.clone();
        let mut checked = self.check(stop);
        checked.marks =
            super::diagnostics::problem_marks(&checked.diagnostics, &buffer, checked.revision)
                .into();
        checked.missing_file_lines = missing_file_lines(&checked.diagnostics, &buffer);
        checked
    }

    fn check(self, stop: &dyn AbortSignal) -> CheckedSource {
        let Self {
            revision,
            library_folders,
            buffer,
            deck,
            source_path,
            include_search,
            sealed_sources,
            is_dependency,
            stored,
            added_lines,
            include_resolutions,
            shown_path,
            messages,
        } = self;
        if added_lines {
            let validation_id = crate::state::content_digest(&buffer).to_string();
            let mut diagnostics = super::added_lines::refusals(&buffer);
            for diagnostic in &mut diagnostics {
                diagnostic.bind_validation(revision, &validation_id);
            }
            return CheckedSource {
                revision,
                library_folders,
                diagnostics: bounded_netlist_diagnostics(
                    diagnostics,
                    &buffer,
                    revision,
                    &validation_id,
                    messages,
                ),
                symbols: None,
                unread: None,
                reading: None,
                includes: None,
                dialect_hint: None,
                marks: Arc::from([]),
                missing_file_lines: Arc::from([]),
            };
        }
        // The hint reads the whole text, so it is read here, away from the
        // frame, rather than by the band that shows it.
        let dialect_hint = (!is_dependency)
            .then(|| super::dialect::hint(&buffer))
            .flatten();
        // A narrow override is parsed as the deck it executes as: the
        // generated base with the override spliced in. Every position the
        // parse reports is a position in that deck, and has to be carried
        // back into the text the editor shows and a fix edits.
        let (composed, unmaterialized) = match deck.compose() {
            Ok(composed) => (composed, None),
            Err(error) => (None, Some(error)),
        };
        let materialized = match (&composed, unmaterialized) {
            (_, Some(error)) => Err(error),
            (Some(deck), None) => Ok(deck.as_str()),
            (None, None) => Ok(buffer.as_str()),
        };
        let in_buffer = |diagnostics: Vec<Diagnostic>| match composed.as_deref() {
            Some(composed) => {
                super::diagnostics::in_authored_text(diagnostics, composed, &buffer, messages)
            }
            None => diagnostics,
        };
        let bound = |mut diagnostics: Vec<Diagnostic>| {
            let validation_id = crate::state::content_digest(&buffer).to_string();
            for diagnostic in &mut diagnostics {
                diagnostic.bind_validation(revision, &validation_id);
            }
            bounded_netlist_diagnostics(diagnostics, &buffer, revision, &validation_id, messages)
        };
        let sealed_sources = match sealed_sources.transpose() {
            Ok(sources) => sources,
            Err(error) => {
                let line = materialized
                    .as_ref()
                    .ok()
                    .and_then(|source| first_external_dependency_line(source));
                let diagnostics = in_buffer(vec![
                    Diagnostic::error(messages.format(
                        MessageId::NetlistImportedIncludesUnreadable,
                        &[("error", &error)],
                    ))
                    .with_line(line),
                ]);
                return CheckedSource {
                    revision,
                    library_folders,
                    diagnostics: bound(diagnostics),
                    symbols: None,
                    unread: None,
                    reading: None,
                    includes: None,
                    dialect_hint,
                    marks: Arc::from([]),
                    missing_file_lines: Arc::from([]),
                };
            }
        };
        let reads_buffer_as_root = composed.is_none() && !is_dependency;
        let (mut diagnostics, symbols, unread, reading, includes) = if buffer.trim().is_empty() {
            let spectre = rspice_core::netlist::is_spectre_source(
                source_path
                    .as_deref()
                    .or(shown_path.as_deref())
                    .unwrap_or(Path::new("")),
                &buffer,
            );
            (
                Vec::new(),
                Some(Vec::new()),
                Some(Vec::new()),
                Some(DeckReading::of(&Default::default(), spectre)),
                Some(Vec::new()),
            )
        } else {
            match materialized {
                Ok(source) => {
                    let (diagnostics, symbols, unread, reading, includes) = match &stored {
                        Some((at, files)) => {
                            parse_stored(source, at, files, sealed_sources.as_ref(), messages, stop)
                        }
                        None => parse_buffer_with_context(
                            source,
                            source_path.as_deref(),
                            shown_path.as_deref(),
                            &include_search,
                            platform_include_access(),
                            sealed_sources.as_ref(),
                            stop,
                            messages,
                        ),
                    };
                    (
                        in_buffer(diagnostics),
                        symbols,
                        unread.filter(|_| reads_buffer_as_root),
                        reading,
                        includes.filter(|_| reads_buffer_as_root),
                    )
                }
                Err(error) => (vec![Diagnostic::error(error)], None, None, None, None),
            }
        };
        // A shadowed include is reported from what this check walked when it
        // read the deck's includes, else from the last run's walk.
        let walked = includes.as_ref().map(|includes| {
            includes
                .iter()
                .map(|include| (include.requested.clone(), include.resolution.clone()))
                .collect::<BTreeMap<_, _>>()
        });
        diagnostics.extend(shadowed_include_diagnostics(
            walked.as_ref().unwrap_or(&include_resolutions),
            &buffer,
            messages,
        ));
        super::diagnostics::as_the_reader_sees_them(
            &mut diagnostics,
            &buffer,
            shown_path.as_deref(),
            messages,
        );
        CheckedSource {
            revision,
            library_folders,
            diagnostics: bound(diagnostics),
            symbols,
            unread: Some(unread),
            reading,
            includes,
            dialect_hint,
            marks: Arc::from([]),
            missing_file_lines: Arc::from([]),
        }
    }
}

fn bounded_netlist_diagnostics(
    diagnostics: Vec<Diagnostic>,
    buffer: &str,
    revision: u64,
    validation_id: &str,
    messages: MessageCatalog,
) -> NetlistDiagnosticCollection {
    NetlistDiagnosticCollection::try_new(diagnostics, buffer).unwrap_or_else(|error| {
        let mut diagnostic = Diagnostic::current(
            "rspice.diagnostics",
            "DIAGNOSTIC-CAPACITY",
            DiagnosticSeverity::Error,
            messages.text(MessageId::NetlistTooManyFindings),
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
    let (diagnostics, symbols, _, _, _) = parse_buffer_with_context(
        buffer,
        None,
        None,
        &rspice_simulation::netlist_preparation::IncludeSearchChain::default(),
        platform_include_access(),
        None,
        &rspice_core::abort_signal::NoAbort,
        crate::workbench::MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates),
    );
    (diagnostics, symbols)
}

/// One parse of the visible source: its findings, the symbols it defines,
/// the lines of the source itself the parse did not read, and how the engine
/// read it. The last three are absent when the parse did not complete.
type BufferParse = (
    Vec<Diagnostic>,
    Option<Vec<completion::SymbolEntry>>,
    Option<Vec<rspice_core::netlist::UnreadSourceRegion>>,
    Option<DeckReading>,
    Option<Vec<CheckedInclude>>,
);

/// The `.include` and `.lib` lines a parse read, each with the file it
/// found, and for a `.lib` line that selects a section, the sections that
/// file declares: a library two lines name is read for them once.
fn checked_includes(netlist: &rspice_core::Netlist) -> Vec<CheckedInclude> {
    let mut sections: BTreeMap<PathBuf, Arc<[rspice_core::library::LibSectionSummary]>> =
        BTreeMap::new();
    netlist
        .resolved_includes()
        .iter()
        .map(|edge| {
            let declared = if edge.selected_section().is_some() {
                sections
                    .entry(edge.resolved_path().to_path_buf())
                    .or_insert_with(|| {
                        rspice_core::library::enumerate_lib_sections(edge.source().as_bytes())
                            .into()
                    })
                    .clone()
            } else {
                Arc::from([])
            };
            CheckedInclude {
                owner: edge.owner_path().to_path_buf(),
                line: edge.directive_line(),
                requested: edge.requested_path().to_owned(),
                path: edge.resolved_path().to_path_buf(),
                section: edge.selected_section().map(str::to_owned),
                resolution: edge.resolution().clone(),
                sections: declared,
            }
        })
        .collect()
}

/// Check `buffer`, the text of the file `source_path`, or of no file and
/// called `name`, as the engine reads it under that name: SPICE, or Spectre
/// for a `.scs` name or a `simulator lang=` line.
fn parse_buffer_with_context(
    buffer: &str,
    source_path: Option<&Path>,
    name: Option<&Path>,
    include_search: &rspice_simulation::netlist_preparation::IncludeSearchChain,
    include_access: IncludeAccess,
    sealed_sources: Option<&crate::state::model_library::SealedModelExecutionSources>,
    abort: &dyn AbortSignal,
    messages: MessageCatalog,
) -> BufferParse {
    let external_line = first_external_dependency_line(buffer);
    let file = match source_path
        .map(|path| editor_parse_source(path, messages))
        .transpose()
    {
        Ok(file) => file,
        Err(error) => return (vec![Diagnostic::error(error)], None, None, None, None),
    };
    let spectre = rspice_core::netlist::is_spectre_source(
        file.as_deref().or(name).unwrap_or(Path::new("")),
        buffer,
    );
    let options = rspice_core::netlist::NetlistParseOptions {
        resource_limits: rspice_core::ResourceLimits::default(),
        ..Default::default()
    };
    // The name the parse reads the text under, which its findings name, and
    // the parse.
    let (parse_source, parsed) = match (file, external_line) {
        (None, Some(line)) => {
            return (
                vec![
                    Diagnostic::error(messages.text(MessageId::NetlistDeckHasNoFolder))
                        .with_line(Some(line)),
                ],
                None,
                None,
                None,
                None,
            );
        }
        (file, None) if !spectre => (
            file.or_else(|| name.map(Path::to_path_buf)),
            // A dependency-free deck needs no ambient filesystem or
            // browser-file capability. Parsing it in-memory is both
            // sufficient and prevents the authenticated-bundle policy from
            // rejecting ordinary owned source merely because the target is
            // WASM.
            rspice_core::Netlist::parse_with_options_and_abort(buffer, options, abort),
        ),
        (Some(file), _) if include_access == IncludeAccess::NativeFilesystem => {
            // One owner decides which engine entry point a host-file parse
            // goes through, so the editor lints the deck against exactly the
            // chain a run would seal it against.
            let parsed = include_search.parse_with_abort(buffer, Some(&file), options, abort);
            (Some(file), parsed)
        }
        (file, None) => {
            // Only a parse under a name reaches the Spectre front end. A
            // Spectre deck with no file to read includes from, or in the
            // browser, is read alone, under its own name.
            let root = spectre_root(file.as_deref().or(name));
            let bundle = match rspice_core::netlist::SealedSourceBundle::try_new([(
                root.clone(),
                buffer.to_owned(),
            )]) {
                Ok(bundle) => bundle,
                Err(error) => {
                    return (
                        parse_error_diagnostics_at(error, None, messages),
                        None,
                        None,
                        None,
                        None,
                    );
                }
            };
            let parsed =
                rspice_core::Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
                    buffer, &root, bundle, options, abort,
                );
            (Some(root), parsed)
        }
        (Some(file), Some(_)) => {
            let Some(sealed_sources) = sealed_sources else {
                return (
                    vec![
                        Diagnostic::error(messages.text(MessageId::NetlistBrowserIncludesImported))
                            .with_line(external_line),
                    ],
                    None,
                    None,
                    None,
                    None,
                );
            };
            let bundle = match sealed_sources.bundle_for_root(&file, buffer) {
                Ok(bundle) => bundle,
                Err(error) => {
                    return (
                        vec![Diagnostic::error(error).with_line(external_line)],
                        None,
                        None,
                        None,
                        None,
                    );
                }
            };
            let parsed =
                rspice_core::Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
                    buffer, &file, bundle, options, abort,
                );
            (Some(file), parsed)
        }
    };
    parse_findings(
        parsed,
        parse_source.as_deref(),
        buffer,
        spectre,
        source_path,
        messages,
    )
}

/// Check `buffer`, the file of a deck the project keeps as text that the
/// engine reads at `at`, among `files`, every file of the deck where the
/// engine reads it, and beside the model sources sealed for it.
fn parse_stored(
    buffer: &str,
    at: &Path,
    files: &[(PathBuf, String)],
    sealed_sources: Option<&crate::state::model_library::SealedModelExecutionSources>,
    messages: MessageCatalog,
    abort: &dyn AbortSignal,
) -> BufferParse {
    let spectre = rspice_core::netlist::is_spectre_source(at, buffer);
    let bundle = match sealed_sources {
        Some(sealed) => sealed.relative_bundle_with(files.to_vec()),
        None => rspice_core::netlist::SealedSourceBundle::try_new_relative(files.iter().cloned())
            .map_err(|error| error.to_string()),
    };
    let bundle = match bundle {
        Ok(bundle) => bundle,
        Err(error) => return (vec![Diagnostic::error(error)], None, None, None, None),
    };
    let options = rspice_core::netlist::NetlistParseOptions {
        resource_limits: rspice_core::ResourceLimits::default(),
        ..Default::default()
    };
    let parsed = rspice_core::Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
        buffer, at, bundle, options, abort,
    );
    let missing = match &parsed {
        Err(rspice_core::netlist::ParseWithAbortError::Parse(
            rspice_core::netlist::ParseError::MissingSources(missing),
        )) => missing.iter().filter_map(not_in_deck).collect(),
        Err(rspice_core::netlist::ParseWithAbortError::Parse(missing)) => {
            not_in_deck(missing).into_iter().collect()
        }
        Ok(_) | Err(rspice_core::netlist::ParseWithAbortError::Aborted) => Vec::new(),
    };
    let mut found = parse_findings(parsed, Some(at), buffer, spectre, None, messages);
    // Each line that names a file the deck does not keep says so in the
    // reader's words, and offers to add it.
    for (origin, requested, outside) in missing {
        let Some(diagnostic) = found.0.iter_mut().find(|diagnostic| {
            diagnostic.fix.is_none()
                && diagnostic.source_path == origin.path
                && diagnostic.source_line == origin.line.checked_sub(1)
        }) else {
            continue;
        };
        let id = if outside {
            MessageId::NetlistFileOutsideDeck
        } else {
            MessageId::NetlistFileNotInDeck
        };
        diagnostic.message = messages.format(id, &[("path", &requested)]);
        diagnostic.fix = add_file_fix(files, &origin, &requested, outside, messages);
        diagnostic.refresh_canonical_location();
    }
    found
}

/// Where a file a deck kept as text names and does not keep is named, the
/// name, and whether it is outside the deck, when `error` says so.
fn not_in_deck(
    error: &rspice_core::netlist::ParseError,
) -> Option<(rspice_core::netlist::NetlistSourceLocation, String, bool)> {
    match error {
        rspice_core::netlist::ParseError::SourceNotInDeck {
            origin: Some(origin),
            requested,
            outside,
        } => Some((origin.clone(), requested.clone(), *outside)),
        _ => None,
    }
}

/// Add file…, for the line at `origin` of a deck kept as text that names
/// `requested`, a file the deck does not keep, `outside` it by an absolute
/// path: the picked file is kept where the line reads it, or beside the
/// file the line is in by its name, and the line then names it so.
fn add_file_fix(
    files: &[(PathBuf, String)],
    origin: &rspice_core::netlist::NetlistSourceLocation,
    requested: &str,
    outside: bool,
    messages: crate::workbench::MessageCatalog,
) -> Option<crate::state::netlist_diagnostics::DiagnosticFix> {
    let owner = rspice_design::deck::stored_file_path(origin.path.as_deref()?);
    let line = origin.line.checked_sub(1)?;
    let text = files
        .iter()
        .find(|(identity, _)| rspice_design::deck::stored_file_path(identity) == owner)
        .map(|(_, text)| text.as_str())?;
    let span = crate::workbench::documents::deck_files::named_path_span(text, line, requested)?;
    let keep_as = crate::workbench::documents::deck_files::kept_path(&owner, requested, outside);
    let rewrite_to = outside.then(|| {
        keep_as
            .rsplit('/')
            .next()
            .unwrap_or(keep_as.as_str())
            .to_owned()
    });
    Some(crate::state::netlist_diagnostics::DiagnosticFix {
        label: messages.text(MessageId::NetlistAddFileToDeck),
        replacement: rewrite_to
            .clone()
            .unwrap_or_else(|| text[span.clone()].to_owned()),
        span,
        add_file: Some(crate::state::netlist_diagnostics::DeckFileRequest {
            owner,
            line,
            requested: requested.to_owned(),
            keep_as,
            rewrite_to,
        }),
    })
}

/// What one parse of `buffer`, read under the name `parse_source`, found:
/// its findings, symbols, unread lines, reading and includes.
fn parse_findings(
    parsed: Result<rspice_core::Netlist, rspice_core::netlist::ParseWithAbortError>,
    parse_source: Option<&Path>,
    buffer: &str,
    spectre: bool,
    source_path: Option<&Path>,
    messages: MessageCatalog,
) -> BufferParse {
    match parsed {
        Ok(netlist) => {
            let unread = netlist.unread_root_source().cloned().collect::<Vec<_>>();
            let mut diagnostics = parser_diagnostics(&netlist, parse_source, messages);
            if let Err(error) = rspice_core::netlist::validate_output_requests(&netlist) {
                diagnostics.extend(parse_error_diagnostics_at(error, parse_source, messages));
            }
            diagnostics.extend(unknown_reference_diagnostics(
                buffer,
                Some(&netlist),
                &unread,
                messages,
            ));
            (
                diagnostics,
                Some(harvest_symbols(&netlist, messages)),
                Some(unread),
                Some(DeckReading::of(&netlist.options, spectre)),
                Some(checked_includes(&netlist)),
            )
        }
        Err(rspice_core::netlist::ParseWithAbortError::Parse(error)) => (
            parse_error_diagnostics_at(error, parse_source, messages),
            None,
            None,
            None,
            None,
        ),
        Err(rspice_core::netlist::ParseWithAbortError::Aborted) => (
            vec![
                Diagnostic::current(
                    "rspice.netlist.parser",
                    "SPICE-VALIDATION-CANCELLED",
                    DiagnosticSeverity::Info,
                    messages.text(MessageId::NetlistCheckStopped),
                )
                .with_source_path(source_path.map(Path::to_path_buf)),
            ],
            None,
            None,
            None,
            None,
        ),
    }
}

/// The absolute name a deck that is no file on disk is read under: its own
/// name under the root folder, or `netlist` there when it has none.
fn spectre_root(name: Option<&Path>) -> PathBuf {
    let name = name.unwrap_or(Path::new("netlist"));
    if name.has_root() {
        name.to_path_buf()
    } else {
        Path::new("/").join(name)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn editor_parse_source(path: &Path, messages: MessageCatalog) -> Result<PathBuf, String> {
    if path.is_absolute() {
        return Ok(path.canonicalize().unwrap_or_else(|_| path.to_path_buf()));
    }
    let current = std::env::current_dir().map_err(|error| {
        messages.format(
            MessageId::NetlistDeckFolderUnknown,
            &[("error", &error.to_string())],
        )
    })?;
    let absolute = current.join(path);
    Ok(absolute.canonicalize().unwrap_or(absolute))
}

#[cfg(target_arch = "wasm32")]
fn editor_parse_source(path: &Path, _messages: MessageCatalog) -> Result<PathBuf, String> {
    Ok(path.to_path_buf())
}

/// One warning per include whose relative name names a second file later in
/// the search chain.
///
/// The chain was walked by the engine when the closure was resolved; nothing
/// here re-walks it, so the Problems row, the navigator row and the deck that
/// actually ran cannot disagree about which file won.
fn shadowed_include_diagnostics(
    include_resolutions: &BTreeMap<String, IncludeResolution>,
    buffer: &str,
    messages: MessageCatalog,
) -> Vec<Diagnostic> {
    if include_resolutions.is_empty() {
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
            let resolution = include_resolutions.get(&requested)?;
            let shadowed = resolution.shadowed().next()?;
            Some(
                Diagnostic::current(
                    "rspice.netlist.include",
                    "SPICE-INCLUDE-SHADOWED",
                    DiagnosticSeverity::Warning,
                    messages.format(
                        MessageId::NetlistIncludeShadows,
                        &[
                            ("name", &requested),
                            ("found", &resolution.resolved().display().to_string()),
                            ("hidden", &shadowed.path().display().to_string()),
                        ],
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
    parse_error_diagnostics_at(
        error,
        None,
        crate::workbench::MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates),
    )
}

fn parse_error_diagnostics_at(
    error: rspice_core::netlist::ParseError,
    editor_source_path: Option<&Path>,
    messages: MessageCatalog,
) -> Vec<Diagnostic> {
    // Every file the deck names that could not be read is a finding at the
    // line that names it.
    if let rspice_core::netlist::ParseError::MissingSources(missing) = error {
        return missing
            .into_iter()
            .flat_map(|missing| parse_error_diagnostics_at(missing, editor_source_path, messages))
            .collect();
    }
    let rspice_core::netlist::ParseError::OutputSymbolValidation(validation) = error else {
        return vec![parse_error_diagnostic_at(
            error,
            editor_source_path,
            messages,
        )];
    };

    let mut groups = Vec::<(
        rspice_core::netlist::OutputDirectiveKind,
        rspice_core::netlist::NetlistSourceLocation,
        Vec<String>,
    )>::new();
    for unresolved in validation.unresolved {
        let entry = messages.format(
            MessageId::NetlistUndefinedOutput,
            &[
                ("kind", &unresolved.kind.to_string()),
                ("symbol", &unresolved.symbol.to_string()),
                ("operator", &unresolved.operator.to_string()),
            ],
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
            Diagnostic::error(messages.format(
                MessageId::NetlistUndefinedOutputs,
                &[
                    ("directive", &directive.to_string()),
                    ("place", &origin.to_string()),
                    ("names", &entries.join(", ")),
                ],
            ))
            .with_source_location(&origin, editor_source_path)
        })
        .collect()
}

/// The code of a finding at a line that names a file nothing holds.
const NOT_FOUND: &str = "SPICE-INCLUDE-NOT-FOUND";
/// The code of a finding at a line of a deck kept as text that names a file
/// the deck does not keep.
const MISSING_FROM_DECK: &str = "SPICE-INCLUDE-NOT-IN-DECK";

/// Whether `diagnostic` says the line it is at names a file the check could
/// not read, so the outline can mark that include as missing.
pub(crate) fn is_missing_file_finding(diagnostic: &Diagnostic) -> bool {
    matches!(&*diagnostic.canonical.code, NOT_FOUND | MISSING_FROM_DECK)
}

/// The lines of `buffer` that `diagnostics`' findings of files a check could
/// not read point at, by their zero-based line, read in one pass.
fn missing_file_lines(
    diagnostics: &NetlistDiagnosticCollection,
    buffer: &crate::state::RevisedText,
) -> Arc<[(usize, String)]> {
    let wanted = diagnostics
        .iter()
        .filter(|finding| is_missing_file_finding(finding))
        .filter_map(|finding| finding.line)
        .collect::<std::collections::BTreeSet<_>>();
    let Some(&last) = wanted.last() else {
        return Arc::from([]);
    };
    buffer
        .lines()
        .enumerate()
        .take(last + 1)
        .filter(|(line, _)| wanted.contains(line))
        .map(|(line, text)| (line, text.to_owned()))
        .collect()
}

/// The finding at `origin`, the line that names a file the check could not
/// read, or at the top of the deck when no line is known.
fn missing_file_finding(
    code: &str,
    message: String,
    origin: Option<rspice_core::netlist::NetlistSourceLocation>,
    editor_source_path: Option<&Path>,
) -> Diagnostic {
    let origin = origin.unwrap_or_else(|| match editor_source_path {
        Some(path) => rspice_core::netlist::NetlistSourceLocation::in_file(path, 0),
        None => rspice_core::netlist::NetlistSourceLocation::in_memory(0),
    });
    Diagnostic::current(
        "rspice.netlist.parser",
        code,
        DiagnosticSeverity::Error,
        message,
    )
    .with_source_location(&origin, editor_source_path)
}

#[cfg(test)]
fn parse_error_diagnostic(error: rspice_core::netlist::ParseError) -> Diagnostic {
    parse_error_diagnostic_at(
        error,
        None,
        crate::workbench::MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates),
    )
}

fn parse_error_diagnostic_at(
    error: rspice_core::netlist::ParseError,
    editor_source_path: Option<&Path>,
    messages: MessageCatalog,
) -> Diagnostic {
    use rspice_core::netlist::ParseError;

    match error {
        ParseError::Syntax { line, message } => {
            // Parser lines are 1-based; `line == 0` means "unlocated".
            let (message, origin) = mapped_syntax_origin(message, line, editor_source_path);
            Diagnostic::error(message).with_source_location(&origin, editor_source_path)
        }
        // The check of a deck kept as text words it from the catalogue.
        ParseError::SourceNotInDeck {
            origin,
            requested,
            outside,
        } => {
            let message = ParseError::SourceNotInDeck {
                origin: None,
                requested,
                outside,
            }
            .to_string();
            missing_file_finding(MISSING_FROM_DECK, message, origin, editor_source_path)
        }
        ParseError::SourceNotFound {
            origin,
            requested,
            looked_in,
        } => {
            let message = ParseError::SourceNotFound {
                origin: None,
                requested,
                looked_in,
            }
            .to_string();
            let origin = origin.map(|origin| rspice_core::netlist::NetlistSourceLocation {
                path: origin.path.map(canonical_diagnostic_source),
                line: origin.line,
            });
            missing_file_finding(NOT_FOUND, message, origin, editor_source_path)
        }
        ParseError::MissingSubcircuitEnds(error) => Diagnostic::error(error.to_string())
            .with_source_location(&error.opened_at, editor_source_path),
        ParseError::DuplicateSubcircuitPortBinding(error) => {
            let detail = messages.format(
                MessageId::NetlistDuplicatePortDetail,
                &[
                    ("subcircuit", &error.subcircuit_name.to_string()),
                    (
                        "subcircuit_read",
                        &error.canonical_subcircuit_name.to_string(),
                    ),
                    ("instance", &error.qualified_instance_name.to_string()),
                    ("formal", &error.formal_port.to_string()),
                    ("first", &error.first_position.to_string()),
                    ("second", &error.conflicting_position.to_string()),
                    ("first_node", &error.first_actual_node.to_string()),
                    ("second_node", &error.conflicting_actual_node.to_string()),
                ],
            );
            Diagnostic::error(format!("{error}\n{detail}"))
        }
        ParseError::GlobalSubcircuitPortBinding(error) => {
            let detail = messages.format(
                MessageId::NetlistGlobalPortDetail,
                &[
                    ("subcircuit", &error.subcircuit_name.to_string()),
                    (
                        "subcircuit_read",
                        &error.canonical_subcircuit_name.to_string(),
                    ),
                    ("instance", &error.qualified_instance_name.to_string()),
                    ("formal", &error.formal_port.to_string()),
                    ("position", &error.position.to_string()),
                    ("node", &error.actual_node.to_string()),
                ],
            );
            Diagnostic::error(format!("{error}\n{detail}"))
        }
        ParseError::UndefinedSubcircuit(error) => {
            let detail = messages.format(
                MessageId::NetlistUndefinedSubcircuitDetail,
                &[
                    ("subcircuit", &error.subcircuit_name.to_string()),
                    (
                        "subcircuit_read",
                        &error.canonical_subcircuit_name.to_string(),
                    ),
                    ("instance", &error.qualified_instance_name.to_string()),
                    ("instance_read", &error.canonical_instance_name.to_string()),
                ],
            );
            Diagnostic::error(format!("{error}\n{detail}"))
        }
        ParseError::MissingDeviceModel(error) => {
            let detail = messages.format(
                MessageId::NetlistMissingModelDetail,
                &[
                    ("device", &error.device_name.to_string()),
                    ("device_read", &error.canonical_device_name.to_string()),
                    ("kind", &error.device_type.to_string()),
                ],
            );
            Diagnostic::error(format!("{error}\n{detail}"))
        }
        ParseError::UndefinedMutualInductorReference(error) => {
            let scope = error
                .scope_name
                .clone()
                .unwrap_or_else(|| messages.text(MessageId::NetlistTopLevel));
            let detail = messages.format(
                MessageId::NetlistCouplingDetail,
                &[
                    ("coupling", &error.qualified_coupling_name.to_string()),
                    ("coupling_read", &error.canonical_coupling_name.to_string()),
                    ("inductor", &error.qualified_inductor_name.to_string()),
                    ("inductor_read", &error.canonical_inductor_name.to_string()),
                    ("scope", &scope),
                    ("position", &error.reference_position.to_string()),
                ],
            );
            Diagnostic::error(format!("{error}\n{detail}"))
                .with_source_location(&error.origin, editor_source_path)
        }
        ParseError::OutputSymbolValidation(error) => Diagnostic::error(error.to_string()),
        ParseError::OutputExpressionValidation(error) => Diagnostic::error(format!(
            "{error}\n{}",
            messages.format(
                MessageId::NetlistExpressionDetail,
                &[
                    ("directive", &error.directive.to_string()),
                    ("expression", &format!("{{{}}}", error.expression)),
                ],
            )
        ))
        .with_source_location(&error.origin, editor_source_path),
        ParseError::StartupDirectiveConflict(error) => Diagnostic::error(format!(
            "{error}\n{}",
            messages.format(
                MessageId::NetlistStartupConflictDetail,
                &[
                    ("first", error.first_kind.as_spice_directive()),
                    ("first_place", &error.first.to_string()),
                    ("second", error.conflicting_kind.as_spice_directive()),
                    ("second_place", &error.conflicting.to_string()),
                ],
            )
        ))
        .with_source_location(&error.conflicting, editor_source_path),
        ParseError::DeviceInitialCondition(error) => {
            let origin = device_initial_condition_diagnostic_origin(&error);
            Diagnostic::error(error.to_string()).with_source_location(origin, editor_source_path)
        }
        ParseError::ResourceLimit(refusal) => {
            Diagnostic::error(messages.resource_refusal(&refusal))
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
fn harvest_symbols(
    netlist: &rspice_core::Netlist,
    messages: MessageCatalog,
) -> Vec<completion::SymbolEntry> {
    let mut symbols = Vec::new();
    for model in &netlist.models {
        symbols.push(completion::SymbolEntry {
            name: model.name.clone(),
            kind: "model",
            detail: model.model_type.to_ascii_uppercase(),
            doc: messages.format(
                MessageId::NetlistModelCardDoc,
                &[
                    ("kind", &model.model_type.to_ascii_uppercase()),
                    (
                        "parameters",
                        &messages.counted(
                            model.params.len()
                                + model.expr_params.len()
                                + model.string_params.len(),
                            MessageId::NetlistParameterCountSingular,
                            MessageId::NetlistParameterCount,
                        ),
                    ),
                ],
            ),
        });
    }
    for subckt in &netlist.subcircuits {
        symbols.push(completion::SymbolEntry {
            name: subckt.name.clone(),
            kind: "subckt",
            detail: messages.counted(
                subckt.ports.len(),
                MessageId::NetlistPortCountSingular,
                MessageId::NetlistPortCount,
            ),
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
    use crate::workbench::code::tests::{SourceHarness, harness};

    /// Two directories on the chain hold the same relative name. The winner
    /// and the shadow both come from the engine's own walk, so the warning
    /// cannot disagree with the deck that would run.
    fn shadowed_fixture(label: &str) -> (std::path::PathBuf, SourceHarness) {
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

        let mut harness = harness();
        for dependency in processor.resolved_dependencies() {
            harness.state.source_editing.include_resolutions.insert(
                dependency.requested_path().to_owned(),
                dependency.resolution().clone(),
            );
        }
        (root, harness)
    }

    #[test]
    fn a_shadowed_include_becomes_one_warning_bound_to_its_own_card() {
        let (root, harness) = shadowed_fixture("warned");
        let diagnostics = shadowed_include_diagnostics(
            &harness.state.source_editing.include_resolutions,
            "shadow deck\n.include models.lib\n.end\n",
            crate::workbench::MessageCatalog::new(
                crate::workbench::UiTextLocale::EnglishUnitedStates,
            ),
        );

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
        let (root, mut harness) = shadowed_fixture("quiet");
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
        harness.state.source_editing.include_resolutions.clear();
        for dependency in processor.resolved_dependencies() {
            harness.state.source_editing.include_resolutions.insert(
                dependency.requested_path().to_owned(),
                dependency.resolution().clone(),
            );
        }

        assert!(
            shadowed_include_diagnostics(
                &harness.state.source_editing.include_resolutions,
                "shadow deck\n.include models.lib\n.end\n",
                crate::workbench::MessageCatalog::new(
                    crate::workbench::UiTextLocale::EnglishUnitedStates
                ),
            )
            .is_empty()
        );
        std::fs::remove_dir_all(&root).ok();
    }
}
