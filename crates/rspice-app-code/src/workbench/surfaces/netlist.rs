//! Mockup-owned Netlist document surface.
//!
//! The center well is deliberately flat: one 33-point document toolbar over
//! an exact-entry editor. Generated and owned source are independent retained
//! documents and switching between them never deletes either one.
//!
//! This module owns the surface's frame — reconciliation entry, the review
//! banner, file drop, deterministic formatting, and the readiness predicates
//! every other part reads. Each submodule owns one transaction of its own.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

mod comparison_view;
mod deck_bands;
mod dialect_chip;
pub mod generation;
mod read_only;
mod revision;
pub mod source_menu;
pub mod toolbar;
pub mod transfer;

use egui::Ui;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::MessageId;
use crate::workbench::code::SourceFeature;
use crate::workbench::code::commands::SourceRequest;
use crate::workbench::design_system::{
    Problem, WorkbenchIcon, empty_state, empty_state_with_actions, paint_problem_mark,
};
use crate::workbench::documents::netlist_document::{
    ExecutedDeckVerification, NetlistRoot, cross_probe,
};
use crate::workbench::framework::feature::Cx;

/// Publish `editor` again once it has drawn, with its owner's history as
/// `editor` gives it: the editor's own history may have taken or made a step
/// while it drew, and a command raised before the page draws again answers
/// from this.
pub(super) fn settle_source_editor(
    ctx: &egui::Context,
    workbench: &mut crate::workbench::WorkbenchSession,
    editor: crate::workbench::documents::text_editor_commands::ActiveSourceEditor,
) {
    if workbench
        .active_source_editor
        .is_some_and(|shown| shown.id == editor.id)
    {
        workbench.active_source_editor = Some(
            crate::workbench::documents::text_editor_commands::with_history_steps(ctx, editor),
        );
    }
}

/// Publish the source editor a page shows as its document, and hand it any
/// request the menus, palette or shortcuts made for it. Called before the
/// editor is drawn so it applies the request in the same frame.
pub(super) fn publish_source_editor(
    ctx: &egui::Context,
    workbench: &mut crate::workbench::WorkbenchSession,
    editor: crate::workbench::documents::text_editor_commands::ActiveSourceEditor,
) {
    workbench.active_source_editor =
        Some(crate::workbench::documents::text_editor_commands::with_history_steps(ctx, editor));
    if let Some(request) = workbench.outbox.take() {
        crate::workbench::documents::text_editor_commands::queue_source_editor_request(
            ctx, editor.id, request,
        );
    }
    if let Some(step) = workbench
        .outbox
        .take::<crate::workbench::documents::source_editing::ProblemStep>()
    {
        crate::workbench::documents::source_editing::queue_problem_step(ctx, editor.id, step);
    }
}

/// Step the netlist editor's caret to the next or previous finding of the
/// deck it shows, when F8 or Shift+F8 asked, and say which it reached.
pub fn step_netlist_problem(
    ctx: &egui::Context,
    model: &crate::workbench::app_state::Model,
    netlist: &crate::workbench::documents::netlist_document::NetlistDocumentState,
    workbench: &mut crate::workbench::WorkbenchSession,
    editor_id: egui::Id,
) {
    use crate::workbench::documents::source_editing;

    let Some(step) = source_editing::take_problem_step(ctx, editor_id) else {
        return;
    };
    let buffer = &model.simulation.source.netlist_content;
    let diagnostics = std::sync::Arc::clone(&netlist.diagnostics);
    // The buffer's own findings, in source order. A finding in an included
    // file names no line of this buffer, so it is not stepped to.
    let mut located = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.is_current())
        .filter_map(|diagnostic| {
            let line = diagnostic.line?;
            let text = buffer.split('\n').nth(line)?;
            // Zero-based byte column to one-based character column.
            let column = diagnostic.column.map(|column| {
                text.get(..column.min(text.len()))
                    .map_or(column, |prefix| prefix.chars().count())
                    + 1
            });
            Some((line + 1, column, diagnostic))
        })
        .collect::<Vec<_>>();
    located.sort_by_key(|(line, column, _)| (*line, *column));
    let places = located
        .iter()
        .map(|(line, column, _)| (*line, *column))
        .collect::<Vec<_>>();
    let messages = workbench.messages();
    if step == source_editing::ProblemStep::AtCaret {
        // The finding the caret is at or after on its line, or else the
        // line's first.
        let before = buffer
            .char_indices()
            .nth(netlist.cursor_char_index)
            .map_or(buffer.as_str(), |(byte, _)| &buffer[..byte]);
        let line = before.matches('\n').count() + 1;
        let column = before
            .rsplit('\n')
            .next()
            .map_or(0, |text| text.chars().count())
            + 1;
        let mut on_line = located.iter().filter(|(at, ..)| *at == line);
        let found = on_line
            .clone()
            .rfind(|(_, at, _)| at.unwrap_or(1) <= column)
            .or_else(|| on_line.next());
        let announcement = found.map_or_else(
            || messages.text(MessageId::SourceDiagnosticNoneAtCaret),
            |(line, column, diagnostic)| {
                source_editing::problem_at_caret_announcement(
                    messages,
                    diagnostic.severity,
                    &diagnostic.canonical.code,
                    (*line, column.unwrap_or(1)),
                    &diagnostic.message,
                    false,
                )
            },
        );
        workbench.prompt(announcement);
        return;
    }
    let Some(index) = source_editing::problem_step_index(
        ctx,
        editor_id,
        buffer,
        &places,
        netlist.cursor_char_index,
        step,
    ) else {
        workbench.prompt(messages.text(MessageId::SourceDiagnosticNone));
        return;
    };
    let (line, column, diagnostic) = located[index];
    let announcement = source_editing::problem_step_announcement(
        messages,
        diagnostic.severity,
        index,
        located.len(),
        line,
        &diagnostic.message,
    );
    source_editing::reveal_source_location(ctx, editor_id, line, column.unwrap_or(1));
    workbench.prompt(announcement);
}

/// Hand a line revealed in the netlist editor to it the way this page asks
/// for a line.
pub fn reveal_in_netlist(
    ui: &Ui,
    model: &crate::workbench::app_state::Model,
    netlist: &mut crate::workbench::documents::netlist_document::NetlistDocumentState,
    editor_id: egui::Id,
) {
    let buffer = &model.simulation.source.netlist_content;
    let Some(line) =
        crate::workbench::documents::text_editor_commands::take_reveal_line(ui, editor_id, || {
            buffer.line_count()
        })
    else {
        return;
    };
    netlist.requested_line = Some(line);
}

/// Before the page draws, in the source feature's context: rebuild the
/// generated deck, show the document the page is on, and pick up a re-run
/// queued behind the last run. The deck page's editor is put in front here,
/// before the shell takes the dispatcher's answers the page's context holds,
/// so they are the answers the editor gets; the page publishes it again as
/// it draws it.
pub fn prepare_workspace(cx: &mut Cx<'_, SourceFeature>) {
    use crate::workbench::documents::netlist_document::{
        active_netlist_source_is_editable_in, can_redo_netlist_edit, can_undo_netlist_edit,
        editor_id_in, session,
    };

    // A project with decks opens its first one, which the page then shows,
    // unless the session the application was restored with opens another.
    if !session::restores_a_deck(&cx.state.netlist) {
        crate::workbench::documents::deck_files::open_project_deck(
            cx.model,
            &mut cx.state.netlist,
            &mut cx.state.netlist_open_documents,
            cx.runs,
            cx.wb.messages(),
        );
    }
    generation::reconcile_documents(cx);
    // The tabs the last launch left open reopen once, over the generated
    // netlist just built, whose includes are among them.
    session::reopen(
        cx.model,
        &mut cx.state.netlist,
        &mut cx.state.netlist_open_documents,
        cx.runs,
        cx.wb.messages(),
    );
    crate::workbench::documents::netlist_document::prepare(
        cx.model,
        &mut cx.state.netlist,
        cx.runs,
    );
    // A deck whose file is missing, or one being read, has no text in
    // front: no editor to find, undo or zoom in.
    if !generated_primary_unavailable(cx.model, &cx.state.netlist)
        && !crate::workbench::documents::deck_files::reading_deck_in_front(&cx.state.netlist)
        && crate::workbench::documents::deck_files::missing_deck_in_front(
            cx.model,
            &cx.state.netlist,
        )
        .is_none()
    {
        let editor = crate::workbench::documents::text_editor_commands::ActiveSourceEditor {
            id: editor_id_in(cx.model, &cx.state.netlist),
            editable: active_netlist_source_is_editable_in(cx.model, &cx.state.netlist),
            navigable: true,
            can_undo: can_undo_netlist_edit(&cx.state.netlist),
            can_redo: can_redo_netlist_edit(&cx.state.netlist),
            has_selection: false,
        };
        cx.wb.active_source_editor = Some(
            crate::workbench::documents::text_editor_commands::with_held_history_steps(
                cx.wb.active_source_editor,
                editor,
            ),
        );
    }
}

/// Whether the deck editor's right-click menu is open, or may open on what
/// this frame carries. The page's context holds the menu's answers only
/// then, so the shell reads this before it lends the context.
pub fn source_menu_open(
    ctx: &egui::Context,
    model: &crate::workbench::app_state::Model,
    netlist: &crate::workbench::documents::netlist_document::NetlistDocumentState,
) -> bool {
    crate::workbench::documents::virtual_text_editor::context_menu_may_show(
        ctx,
        crate::workbench::documents::netlist_document::editor_id_in(model, netlist),
    )
}

/// Draw the deck page in the source feature's context, once it has
/// prepared. The context holds the dispatcher's answers for the commands
/// the feature reads, and its menu's while [`source_menu_open`] says the
/// source editor's menu is open or opening.
pub fn show_prepared(ui: &mut Ui, cx: &mut Cx<'_, SourceFeature>) {
    // A file dropped on the page opens as a deck once the page has drawn.
    let dropped = ui.ctx().input(|input| input.raw.dropped_files.clone());
    if !dropped.is_empty() {
        cx.wb.outbox.push(SourceRequest::OpenDroppedDeck(dropped));
    }
    toolbar::code_toolbar(ui, cx);
    // The run's band states the run and reports what was clicked; routing
    // that click is this surface's transaction, not the painter's.
    match run_strip(ui, cx.model, &cx.state.netlist, cx.runs, cx.wb.messages()) {
        Some(RunStripAction::OpenDeckSnapshot) => {
            if !crate::workbench::documents::netlist_document::open_run_deck_snapshot_in(
                cx.model,
                &mut cx.state.netlist,
                cx.runs,
            ) {
                let text = cx.wb.messages().format(
                    crate::workbench::MessageId::NetlistRunDeckUnavailable,
                    &[("reason", crate::state::absent_deck_reason())],
                );
                cx.wb.prompt(text);
            }
        }
        Some(RunStripAction::Compare) => {
            if let Err(error) =
                crate::workbench::documents::netlist_document::compare_run_deck_snapshot_in(
                    cx.model,
                    &mut cx.state.netlist,
                    cx.runs,
                    cx.wb.messages(),
                )
            {
                cx.wb.prompt(error);
            }
        }
        Some(RunStripAction::OpenTaskDeck { run_id, point }) => {
            if !crate::workbench::documents::netlist_document::open_executed_deck_in(
                cx.model,
                &mut cx.state.netlist,
                cx.runs,
                run_id,
                point,
            ) {
                let text = cx.wb.messages().format(
                    crate::workbench::MessageId::NetlistRunSourceUnavailable,
                    &[
                        ("id", &run_id.to_string()),
                        ("reason", crate::state::absent_deck_reason()),
                    ],
                );
                cx.wb.prompt(text);
            }
        }
        Some(RunStripAction::OpenInResults(run_id)) => {
            // The selection is the destination; the workspace is only where it
            // is shown. A run the history no longer holds refuses the
            // selection, and opening Results anyway landed the reader on
            // whichever dataset happened to be active as if it were the one
            // they asked for. The dispatcher opens Results once the page has
            // drawn.
            if cx.model.simulation.select_run_by_sequence(run_id) {
                cx.execute(crate::workbench::framework::command_registry::ids::RESULTS_WORKSPACE);
            } else {
                let text = cx.wb.messages().format(
                    crate::workbench::MessageId::NetlistRunNotKept,
                    &[("id", &run_id.to_string())],
                );
                cx.wb.prompt(text);
            }
        }
        None => {}
    }
    deck_bands::show(ui, cx);
    dialect_chip::deck_hint_band(ui, cx.model, &cx.state.netlist, cx.wb.messages());
    control_disposition_band(ui, &cx.state.netlist, cx.wb.messages());
    if crate::workbench::documents::text_editor_commands::take_format_document_request(
        ui,
        crate::workbench::documents::netlist_document::editor_id_in(cx.model, &cx.state.netlist),
    ) {
        // Formatting is carried out once the page has drawn. A format the
        // palette, a menu or a shortcut asked for is taken a frame after it
        // was asked, in a frame no input began, so this frame asks for the
        // next at once to draw what formatting wrote.
        ui.ctx().request_repaint();
        cx.wb.outbox.push(SourceRequest::FormatNetlistDocument);
    }
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_inset).show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        if crate::workbench::documents::deck_files::reading_deck_in_front(&cx.state.netlist) {
            // The deck in front is being read: the page says so where its
            // text will be, inset as the editor insets its text.
            egui::Frame::new()
                .inner_margin(egui::Margin::symmetric(12, 8))
                .show(ui, |ui| {
                    crate::workbench::code::netlist_first::reading_line(ui, cx.wb.messages());
                });
        } else if let Some(missing) = crate::workbench::documents::deck_files::missing_deck_in_front(
            cx.model,
            &cx.state.netlist,
        ) {
            // The deck in front is missing: the page says so, where it was
            // last, and offers the band's two ways out.
            let messages = cx.wb.messages();
            deck_bands::missing_body(ui, cx, &missing, messages);
        } else if generated_primary_unavailable(cx.model, &cx.state.netlist) {
            let messages = cx.wb.messages();
            if cx.state.netlist.open_deck.is_none()
                && cx.model.netlist_artifacts.generated_document.is_none()
            {
                use crate::workbench::code::commands::SourceCommand;
                let mut pressed = None;
                empty_state_with_actions(
                    ui,
                    WorkbenchIcon::Netlist,
                    &messages.text(MessageId::NetlistEmptyWorkspaceTitle),
                    &messages.text(MessageId::NetlistEmptyWorkspaceDescription),
                    |ui| {
                        if ui
                            .button(SourceCommand::NewDeck.declared_spec().label)
                            .clicked()
                        {
                            pressed = Some(SourceCommand::NewDeck);
                        }
                        // A deck file opens only where there are files.
                        if cfg!(not(target_arch = "wasm32"))
                            && ui
                                .button(messages.text(MessageId::NetlistOpenDeck))
                                .clicked()
                        {
                            pressed = Some(SourceCommand::OpenNetlist);
                        }
                    },
                );
                if let Some(command) = pressed {
                    // The dispatcher carries it out once the page has drawn.
                    cx.execute(command.key());
                }
            } else {
                let title = messages.text(MessageId::NetlistGeneratedUnavailable);
                let default_description =
                    messages.text(MessageId::NetlistGeneratedUnavailableDescription);
                empty_state(
                    ui,
                    WorkbenchIcon::Netlist,
                    &title,
                    cx.model
                        .netlist_artifacts
                        .generation_error
                        .as_deref()
                        .unwrap_or(&default_description),
                );
            }
        } else if cx.state.netlist.front.is_comparison() {
            // A comparison is a view of two texts, not a text: its lines are
            // listed with both texts' numbers rather than put in an editor.
            let reveal = cx.state.netlist.requested_line.take();
            comparison_view::show(ui, &cx.state.netlist.generated_diff_source, reveal);
        } else {
            let editor_id = crate::workbench::documents::netlist_document::editor_id_in(
                cx.model,
                &cx.state.netlist,
            );
            let editable =
                crate::workbench::documents::netlist_document::active_netlist_source_is_editable_in(
                    cx.model,
                    &cx.state.netlist,
                );
            let page_editor =
                |netlist: &crate::workbench::documents::netlist_document::NetlistDocumentState| {
                    crate::workbench::documents::text_editor_commands::ActiveSourceEditor {
                        id: editor_id,
                        editable,
                        navigable: true,
                        can_undo:
                            crate::workbench::documents::netlist_document::can_undo_netlist_edit(
                                netlist,
                            ),
                        can_redo:
                            crate::workbench::documents::netlist_document::can_redo_netlist_edit(
                                netlist,
                            ),
                        has_selection: false,
                    }
                };
            publish_source_editor(ui.ctx(), cx.wb, page_editor(&cx.state.netlist));
            step_netlist_problem(ui.ctx(), cx.model, &cx.state.netlist, cx.wb, editor_id);
            reveal_in_netlist(ui, cx.model, &mut cx.state.netlist, editor_id);
            // A tab closed, or a project closed, lets go of its editor and
            // its undo history; every listed tab keeps its own.
            crate::workbench::documents::netlist_document::release_closed_editors(
                ui.ctx(),
                &crate::workbench::documents::netlist_document::open_editor_ids_in(
                    cx.model,
                    &cx.state.netlist,
                    &cx.state.netlist_open_documents,
                ),
            );
            let response = crate::workbench::documents::netlist_document::show_editor(
                ui,
                cx.model,
                &mut cx.state.netlist,
                &cx.state.source_editing.include_resolutions,
                cx.runs,
                cx.wb,
            );
            settle_source_editor(ui.ctx(), cx.wb, page_editor(&cx.state.netlist));
            // The right-click menu's rows read the page's context, which
            // holds the menu's answers while the menu is open or opening; the
            // row chosen runs once the page has drawn.
            let entries = crate::workbench::code::SOURCE_EDITOR_MENU;
            let choice = crate::workbench::chrome::title_bar::source_editor_context_menu_over(
                &response,
                |ctx| source_menu::rows(entries, cx, ctx),
            );
            if let Some(choice) = choice {
                cx.execute(choice.command());
            }
            // Double-clicking an instance's name on the generated netlist
            // goes to that instance on its schematic, as Show in schematic
            // does from the menu.
            if response.double_clicked()
                && cross_probe::generated_netlist_in_front(&cx.state.netlist)
                && matches!(
                    cross_probe::caret_name(cx.model, &cx.state.netlist),
                    Some(cross_probe::CaretName::Element { head: true, .. })
                )
            {
                cx.execute(crate::workbench::code::commands::SourceCommand::ShowInSchematic.key());
            }
        }
    });
    // Find asked for on this page opens the find bar of the deck's editor,
    // which takes it the next time it draws.
    if std::mem::take(&mut cx.state.netlist.find.open) {
        crate::workbench::documents::virtual_text_editor::queue_find_bar(
            ui.ctx(),
            crate::workbench::documents::netlist_document::editor_id_in(
                cx.model,
                &cx.state.netlist,
            ),
            false,
        );
    }
    // The dialogs draw in the source feature's context. What one raises is
    // answered once the page has drawn.
    revision::comparison_dialog_window(ui.ctx(), cx);
    transfer::export_generated_dialog_window(ui.ctx(), cx);
}

const RUN_STRIP_GAP: f32 = 8.0;
const RUN_STRIP_DOT_DIAMETER: f32 = 5.0;
const RUN_STRIP_DOT_GAP: f32 = 6.0;
/// Side of a band's problem mark: the kit's own, so it matches every other
/// marked line in the product.
use crate::ui::widgets::PROBLEM_MARK_SIDE as RUN_STRIP_MARK_SIDE;

/// Which statement the strip is making about the run it names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunStripPhase {
    /// The working deck is byte-for-byte the deck this run executed.
    Current,
    /// The working deck has moved on since this run executed.
    Edited,
    /// This run is executing the deck now.
    Running,
    /// The strip is the run snapshot document's own header.
    Snapshot,
}

/// Everything the strip states, read back off the run's own receipt.
///
/// The strip keeps no copy of the run's identity: if the run is no longer
/// retained, or was not a manual deck run, there is nothing to state and the
/// strip does not appear.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunStripProjection {
    pub run_id: u64,
    pub revision: u64,
    pub phase: RunStripPhase,
    /// Which point of the run the document is showing, when it is showing an
    /// executed deck rather than this session's manual baseline. A run's deck
    /// digest describes the source it was authorized over, not the per-point
    /// source a corner actually solved, so naming the point is the only way
    /// the header can be a true statement about what is below it.
    pub point: Option<String>,
    /// Where that point sits among the run's retained decks, and how many
    /// there are. Zero tasks means this session holds none of the run's decks,
    /// which is what makes the hop to them absent rather than offered and then
    /// refused.
    pub point_index: usize,
    pub tasks: usize,
    /// While the run is in flight, the file of the deck it runs, when the deck is a file.
    pub deck: Option<String>,
    /// While the run is in flight, the stage the engine has reached, in its own words.
    pub activity: Option<String>,
    /// What the deck on screen was checked to be, when there is one. Absent
    /// while the strip is describing the manual baseline, which is the deck
    /// somebody typed and was never sealed on a receipt.
    pub verification: Option<ExecutedDeckVerification>,
}

pub fn run_strip_projection(
    model: &crate::workbench::app_state::Model,
    netlist: &crate::workbench::documents::netlist_document::NetlistDocumentState,
    runs: &rspice_app_runs::workbench::slots::runs::RunsState,
) -> Option<RunStripProjection> {
    use crate::workbench::documents::netlist_document::{
        effective_active_document_in, working_deck_source_in,
    };

    let active = effective_active_document_in(netlist);
    let (run_id, phase) = if active == NetlistRoot::RunInput {
        // Which run the snapshot belongs to is whichever binding put it there.
        // `last_run_id` is the *manual baseline* binding, and an executed deck
        // is opened from a run that never touched it — reading only that one
        // left the header silent about exactly the runs this viewer was added
        // for.
        let run_id = match netlist.executed_deck_view {
            Some(selection) => selection.run_id,
            None => runs.manual_run.last_run_id?,
        };
        (run_id, RunStripPhase::Snapshot)
    } else {
        if netlist.front.include().is_some()
            || !matches!(active, NetlistRoot::Generated | NetlistRoot::Deck)
        {
            return None;
        }
        match runs.manual_run.pending_manual_run_id {
            Some(running) => (running, RunStripPhase::Running),
            None => {
                let run_id = runs.manual_run.last_run_id?;
                let baseline = runs.manual_run.last_run_buffer.as_ref()?;
                let phase = if netlist
                    .run_baseline_shown
                    .same(working_deck_source_in(model, netlist), baseline)
                {
                    RunStripPhase::Current
                } else {
                    RunStripPhase::Edited
                };
                (run_id, phase)
            }
        }
    };
    // The manual-deck filter belongs to the *baseline* half of this strip: a
    // schematic run has no editor buffer to be current with or edited against,
    // so there is nothing to state about it there. An executed deck is a
    // statement about the run itself, which every run can make.
    let point = netlist.executed_deck_view.and_then(|selection| {
        model
            .simulation
            .retained
            .executed_decks
            .get(selection.run_id)
            .and_then(|deck| deck.point(selection.point))
            .map(|point| point.label.clone())
    });
    let receipt = model
        .simulation
        .retained
        .run_by_sequence(run_id)?
        .prepared_receipt()
        .filter(|receipt| {
            point.is_some()
                || receipt.source_domain() == crate::state::AnalysisResultSourceDomain::ManualDeck
        })?;
    // The run named by the strip, not the run whose deck is open: in the
    // baseline phases they are the same run, and this is what decides whether
    // the hop to its task decks can be offered at all.
    let tasks = model
        .simulation
        .retained
        .executed_decks
        .get(run_id)
        .map_or(0, |deck| deck.points.len());
    // Tied to the point actually resolving: a selection whose run the archive
    // has since dropped shows no deck, and a verdict about bytes nobody can
    // see is worse than silence.
    let verification = point.as_ref().and_then(|_| {
        netlist
            .executed_deck_view
            .map(|selection| selection.verification)
    });
    let running = phase == RunStripPhase::Running;
    Some(RunStripProjection {
        run_id,
        revision: receipt.project_revision().get(),
        phase,
        point,
        point_index: netlist
            .executed_deck_view
            .map_or(0, |selection| selection.point),
        tasks,
        deck: netlist
            .open_deck
            .as_ref()
            .filter(|_| running && active == NetlistRoot::Deck)
            .map(|deck| deck.root.name()),
        activity: model.simulation.execution.phase.clone().filter(|_| running),
        verification,
    })
}

/// What the user asked the strip for, for the surface to carry out.
#[derive(Clone, Copy)]
enum RunStripAction {
    OpenDeckSnapshot,
    OpenInResults(u64),
    Compare,
    /// Open the exact source one dispatched task's engine read. The point
    /// index travels with the request because the strip is the only thing that
    /// knows which point the reader was looking at.
    OpenTaskDeck {
        run_id: u64,
        point: usize,
    },
}

/// The band between the deck toolbar and the editor that says which run the
/// text in front belongs to: one row in the advisory bands' recipe, its
/// words in the interface's face, the run named once, and the routes out of
/// it as the kit's compact buttons at the band's far end. While a run is in
/// flight the band states that and offers nothing that could race it.
fn run_strip(
    ui: &mut Ui,
    model: &crate::workbench::app_state::Model,
    netlist: &crate::workbench::documents::netlist_document::NetlistDocumentState,
    runs: &rspice_app_runs::workbench::slots::runs::RunsState,
    messages: crate::workbench::MessageCatalog,
) -> Option<RunStripAction> {
    let projection = run_strip_projection(model, netlist, runs)?;
    let run_id = projection.run_id.to_string();
    let (statement, statement_hint) = run_strip_statement(&projection, messages);
    // A text that differs from what the run recorded is worth a mark, and
    // a warning's: it is a difference to know about, not an error.
    let difference = projection.verification.and_then(verification_chip);
    // A dot marks live state only; a finished run's standing is in the
    // words.
    let live = projection.phase == RunStripPhase::Running;
    let mut action = None;
    advisory_band(ui, None, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = RUN_STRIP_GAP;
            match projection.phase {
                RunStripPhase::Running => {}
                RunStripPhase::Snapshot => {
                    let open_run =
                        messages.format(MessageId::NetlistRunSnapshotOpenRun, &[("id", &run_id)]);
                    if band_button(ui, &open_run, None).clicked() {
                        action = Some(RunStripAction::OpenInResults(projection.run_id));
                    }
                    if let Some(next) = task_deck_hop(&projection)
                        && let Some(hop) = task_deck_button(ui, &messages, &projection, next)
                    {
                        action = Some(hop);
                    }
                    // An executed deck has no working copy: it is the source
                    // a point was handed after expansion and corner
                    // materialization, and a diff against the deck being
                    // edited would report every one of those as an edit.
                    // The control is absent rather than offered and then
                    // refused.
                    if projection.point.is_none() {
                        let compare = messages.text(MessageId::NetlistRunSnapshotCompare);
                        let compare_hint =
                            messages.text(MessageId::NetlistRunSnapshotCompareTooltip);
                        if band_button(ui, &compare, Some(&compare_hint)).clicked() {
                            action = Some(RunStripAction::Compare);
                        }
                    }
                }
                RunStripPhase::Current | RunStripPhase::Edited => {
                    let results = messages.text(MessageId::NetlistRunStripOpenResults);
                    if band_button(ui, &results, None).clicked() {
                        action = Some(RunStripAction::OpenInResults(projection.run_id));
                    }
                    let deck = messages.text(MessageId::NetlistRunStripOpenDeck);
                    if band_button(ui, &deck, None).clicked() {
                        action = Some(RunStripAction::OpenDeckSnapshot);
                    }
                    if let Some(next) = task_deck_hop(&projection)
                        && let Some(hop) = task_deck_button(ui, &messages, &projection, next)
                    {
                        action = Some(hop);
                    }
                }
            }
            if projection.phase != RunStripPhase::Running {
                ui.add_space(RUN_STRIP_GAP);
            }
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = RUN_STRIP_DOT_GAP;
                if live {
                    let t = Tokens::get(ui.ctx());
                    let (dot, _) = ui.allocate_exact_size(
                        egui::vec2(RUN_STRIP_DOT_DIAMETER, RUN_STRIP_DOT_DIAMETER),
                        egui::Sense::hover(),
                    );
                    ui.painter().circle_filled(
                        dot.center(),
                        RUN_STRIP_DOT_DIAMETER / 2.0,
                        t.color.accent,
                    );
                }
                // The difference is the shorter phrase and stands after the
                // run's name; the name yields its width first.
                if let Some((label, hint)) = difference {
                    let words = messages.text(label);
                    let font = egui::TextStyle::Body.resolve(ui.style());
                    let width = ui
                        .painter()
                        .layout_no_wrap(words.clone(), font, egui::Color32::PLACEHOLDER)
                        .size()
                        .x;
                    let reserve = RUN_STRIP_GAP + RUN_STRIP_MARK_SIDE + RUN_STRIP_DOT_GAP + width;
                    let room = (ui.available_width() - reserve).max(0.0);
                    ui.scope(|ui| {
                        ui.set_max_width(room);
                        ui.add(egui::Label::new(&statement).truncate())
                            .on_hover_text(&statement_hint);
                    });
                    ui.add_space(RUN_STRIP_GAP - RUN_STRIP_DOT_GAP);
                    let (mark, _) = ui.allocate_exact_size(
                        egui::vec2(RUN_STRIP_MARK_SIDE, RUN_STRIP_MARK_SIDE),
                        egui::Sense::hover(),
                    );
                    paint_problem_mark(ui.painter(), mark, Problem::Warning);
                    ui.add(egui::Label::new(&words).truncate())
                        .on_hover_text(messages.format(hint, &[("id", &run_id)]));
                } else {
                    ui.add(egui::Label::new(&statement).truncate())
                        .on_hover_text(&statement_hint);
                }
            });
        });
    });
    action
}

/// What the band says about the run, once: its name and how the text in
/// front stands against it, then the point or the deck it ran and the
/// revision it ran at, "Run 12 input · nominal · revision 3"; and the
/// sentence its hover gives.
fn run_strip_statement(
    projection: &RunStripProjection,
    messages: crate::workbench::MessageCatalog,
) -> (String, String) {
    let run_id = projection.run_id.to_string();
    let (name, hint) = match projection.phase {
        RunStripPhase::Current => (
            messages.format(MessageId::NetlistRunStripRun, &[("id", &run_id)]),
            MessageId::NetlistRunStripCurrentTooltip,
        ),
        RunStripPhase::Edited => (
            messages.format(MessageId::NetlistRunStripEdited, &[("id", &run_id)]),
            MessageId::NetlistRunStripEditedTooltip,
        ),
        RunStripPhase::Running => (
            messages.format(MessageId::NetlistRunStripRun, &[("id", &run_id)]),
            MessageId::NetlistRunStripRunningTooltip,
        ),
        RunStripPhase::Snapshot => (
            messages.format(MessageId::NetlistRunSnapshotDocument, &[("id", &run_id)]),
            MessageId::NetlistRunStripCurrentTooltip,
        ),
    };
    let revision = projection.revision.to_string();
    // A run in flight is named by the deck it runs and the stage it has reached, "Run 4 ·
    // amp.sp · DC operating point": what it is doing, not the bookkeeping it will be filed under.
    let activity = (projection.phase == RunStripPhase::Running).then(|| {
        projection
            .activity
            .clone()
            .unwrap_or_else(|| messages.text(MessageId::NetlistRunStripRunning))
    });
    let detail = if let Some(activity) = activity {
        match projection.deck.as_deref() {
            Some(deck) => messages.format(
                MessageId::NetlistRunStripRunningDeck,
                &[("deck", deck), ("activity", &activity)],
            ),
            None => activity,
        }
    } else {
        match projection.point.as_deref() {
            Some(point) => messages.format(
                MessageId::NetlistRunStripExecutedPoint,
                &[("point", point), ("revision", &revision)],
            ),
            None => messages.format(
                MessageId::NetlistRunStripIdentity,
                &[("revision", &revision)],
            ),
        }
    };
    (
        messages.format(
            MessageId::NetlistRunStripStatement,
            &[("run", &name), ("detail", &detail)],
        ),
        messages.format(hint, &[("id", &run_id)]),
    )
}

/// Which task deck the strip's hop would open, or `None` when there is none
/// to offer.
///
/// A run whose decks this session does not hold offers nothing: the route
/// would be a control that refuses. Showing the run's own baseline, the hop
/// lands on its first task; showing a task already, it advances and wraps, so
/// a sweep's points are reachable one row at a time without a picker the strip
/// has no width for.
const fn task_deck_hop(projection: &RunStripProjection) -> Option<usize> {
    if projection.tasks == 0 {
        return None;
    }
    match projection.verification {
        None => Some(0),
        Some(_) if projection.tasks > 1 => Some((projection.point_index + 1) % projection.tasks),
        Some(_) => None,
    }
}

/// The hop control, labelled by whether it opens a run's decks or steps
/// through them.
fn task_deck_button(
    ui: &mut Ui,
    messages: &crate::workbench::localization::MessageCatalog,
    projection: &RunStripProjection,
    next: usize,
) -> Option<RunStripAction> {
    let run_id = projection.run_id.to_string();
    let (label, hint) = if projection.verification.is_some() {
        let index = (projection.point_index + 1).to_string();
        let count = projection.tasks.to_string();
        let arguments = [("index", index.as_str()), ("count", count.as_str())];
        (
            messages.format(MessageId::NetlistRunStripNextTask, &arguments),
            messages.format(MessageId::NetlistRunStripNextTaskTooltip, &arguments),
        )
    } else {
        (
            messages.text(MessageId::NetlistRunStripOpenTaskDeck),
            messages.format(
                MessageId::NetlistRunStripOpenTaskDeckTooltip,
                &[("id", run_id.as_str())],
            ),
        )
    };
    band_button(ui, &label, Some(&hint))
        .clicked()
        .then_some(RunStripAction::OpenTaskDeck {
            run_id: projection.run_id,
            point: next,
        })
}

/// What the band says about the text below it, when there is something to
/// say.
///
/// Three of the four states report that nothing is wrong — a reproduced
/// digest, a point's own materialized source, a run that recorded no digest
/// at all — and the band does not spend a mark on them. The fourth is a text
/// that differs from the one the run recorded, and that is the one worth a
/// warning's mark.
const fn verification_chip(
    verification: ExecutedDeckVerification,
) -> Option<(MessageId, MessageId)> {
    match verification {
        ExecutedDeckVerification::Unmatched => Some((
            MessageId::NetlistRunStripDeckUnmatched,
            MessageId::NetlistRunStripDeckUnmatchedTooltip,
        )),
        ExecutedDeckVerification::Verified
        | ExecutedDeckVerification::PointVariant
        | ExecutedDeckVerification::NotRecorded => None,
    }
}

/// One of the band's routes: the kit's compact button, with what it does on
/// its hover when the label alone does not say.
fn band_button(ui: &mut Ui, label: &str, hint: Option<&str>) -> egui::Response {
    let response = crate::ui::widgets::Button::new(label).compact().show(ui);
    if let Some(hint) = hint {
        response.on_hover_text(hint)
    } else {
        response
    }
}

/// The banner recipe every advisory band above the editor shares: one inset
/// row that states a condition of the document, with a problem's mark as the
/// only colour on it. It is one row of the header's height: its buttons are
/// the kit's compact ones, with the same gap above and below them as in the
/// header, so the band and the header read as one family.
fn advisory_band(ui: &mut Ui, problem: Option<Problem>, contents: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_inset)
        .stroke(egui::Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::symmetric(8, 3))
        .show(ui, |ui| {
            // A frame shrinks to its content: the band runs the page's whole
            // width however short its words are.
            ui.set_min_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                if let Some(problem) = problem {
                    let (mark, _) = ui.allocate_exact_size(
                        egui::vec2(RUN_STRIP_MARK_SIDE, RUN_STRIP_MARK_SIDE),
                        egui::Sense::hover(),
                    );
                    paint_problem_mark(ui.painter(), mark, problem);
                }
                contents(ui);
            });
        });
}

/// What became of the active document's `.control` region, stated once above
/// the deck.
///
/// The per-command verdicts already reach the gutter and Problems through the
/// parse; this band exists so the loss is visible without hunting for the
/// block, and it counts the same published diagnostics those rows read rather
/// than deriving a second tally from the buffer.
fn control_disposition_band(
    ui: &mut Ui,
    netlist: &crate::workbench::documents::netlist_document::NetlistDocumentState,
    messages: crate::workbench::MessageCatalog,
) {
    let Some(summary) = crate::workbench::documents::netlist_document::control_disposition_summary(
        &netlist.diagnostics,
    ) else {
        return;
    };
    let promoted = summary.promoted.to_string();
    let dropped = summary.dropped.to_string();
    let arguments = [
        ("promoted", promoted.as_str()),
        ("dropped", dropped.as_str()),
    ];
    // Nothing was lost when every command was promoted, so the band reports
    // rather than warns; the mark is reserved for the deck that gave something
    // up.
    let problem = (summary.dropped > 0).then_some(Problem::Warning);
    let mut hover = messages.format(MessageId::NetlistControlRegionDetail, &arguments);
    for example in &summary.examples {
        hover.push('\n');
        hover.push_str(example);
    }
    advisory_band(ui, problem, |ui| {
        ui.label(
            egui::RichText::new(messages.text(MessageId::NetlistControlRegionDisposition)).strong(),
        )
        .on_hover_text(&hover);
        ui.label(
            egui::RichText::new(messages.format(MessageId::NetlistControlRegionCounts, &arguments))
                .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
        )
        .on_hover_text(&hover);
    });
}

fn generated_primary_unavailable(
    model: &crate::workbench::app_state::Model,
    netlist: &crate::workbench::documents::netlist_document::NetlistDocumentState,
) -> bool {
    netlist.front.include().is_none()
        && netlist.front.shows_generated()
        && !generated_primary_ready_in(model)
}

/// Whether the generated deck is there to show, over the model.
fn generated_primary_ready_in(model: &crate::workbench::app_state::Model) -> bool {
    model.netlist_artifacts.generated_document.is_some()
        && !model.netlist_artifacts.generated_source.is_empty()
}

fn active_document_available(
    model: &crate::workbench::app_state::Model,
    netlist: &crate::workbench::documents::netlist_document::NetlistDocumentState,
    runs: &rspice_app_runs::workbench::slots::runs::RunsState,
) -> bool {
    if netlist.front.include().is_some() {
        return crate::workbench::documents::netlist_document::active_dependency_in(model, netlist)
            .is_some()
            || crate::workbench::documents::netlist_document::active_deck_file_in(netlist)
                .is_some();
    }
    match netlist.front.root_kind() {
        NetlistRoot::Generated => generated_primary_ready_in(model),
        NetlistRoot::AddedLines => true,
        NetlistRoot::Deck => netlist.open_deck.is_some(),
        NetlistRoot::Comparison => !netlist.generated_diff_source.is_empty(),
        NetlistRoot::RunInput => {
            crate::workbench::documents::netlist_document::run_deck_snapshot_run_id_in(runs)
                .is_some()
        }
    }
}

/// What is wrong with the active document, or `None` when nothing is.
///
/// A deck that parses, a comparison that is ready and a sealed run snapshot
/// are the ordinary cases. The toolbar used to announce each of them in green;
/// the tab already names the document, so the status now speaks only when the
/// deck cannot be generated or the check found errors in it: how many, of
/// every kind. Warnings stay in the Problems list and out of the count.
fn document_error_status(
    model: &crate::workbench::app_state::Model,
    netlist: &crate::workbench::documents::netlist_document::NetlistDocumentState,
    messages: crate::workbench::MessageCatalog,
) -> Option<(String, Problem)> {
    if matches!(
        netlist.front.root_kind(),
        NetlistRoot::Comparison | NetlistRoot::RunInput
    ) {
        return None;
    }
    if netlist.front.shows_generated()
        && (model.netlist_artifacts.generation_error.is_some()
            || !generated_primary_ready_in(model)
            || model.netlist_artifacts.generated_input_digest
                != model.netlist_artifacts.current_generation_input_digest)
    {
        let retained_artifact = generated_primary_ready_in(model);
        return Some((
            if retained_artifact {
                messages.text(MessageId::NetlistGenerationStaleBlocked)
            } else {
                messages.text(MessageId::NetlistGenerationBlocked)
            },
            Problem::Warning,
        ));
    }
    let errors = netlist
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.is_current()
                && diagnostic.severity
                    == crate::workbench::documents::netlist_document::DiagnosticSeverity::Error
        })
        .count();
    if errors > 0 {
        let count = errors.to_string();
        return Some((
            messages.format(
                if errors == 1 {
                    MessageId::NetlistErrorCountSingular
                } else {
                    MessageId::NetlistErrorCount
                },
                &[("count", &count)],
            ),
            Problem::Failure,
        ));
    }
    None
}
#[cfg(any(test, feature = "test-support"))]
pub mod fixtures;
#[cfg(test)]
mod tests {
    use super::fixtures::{retain_generated, seal_manual_run};
    use super::*;
    use crate::workbench::code::tests::{SourceHarness, harness};
    use crate::workbench::documents::netlist_document::NetlistTab;

    fn configured_veriloga_state() -> (SourceHarness, String) {
        let mut harness = harness();
        let reference = crate::state::CellViewRef::new("behavioral", "gain", "veriloga");
        let mut view = crate::state::View::new("veriloga", crate::state::ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), "sealed_gain".to_owned());
        view.metadata
            .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
        let mut cell = crate::state::Cell::new("gain");
        cell.add_view(view);
        let mut library = crate::state::Library::new("behavioral");
        library.add_cell(cell);
        harness.model.library_manager.add_library(library);

        let bundle = crate::state::ProjectSourceBundle::try_new(
            crate::state::ProjectSourceOwner::cell_view(reference),
            "behavioral/gain.va",
            "`ifdef NEVER\n`include \"behavioral/inactive.va\"\n`endif\n`include \"behavioral/gain_constants.va\"\nmodule sealed_gain(p, n); inout p, n; electrical p, n; analog I(p,n) <+ `RSPICE_GAIN * V(p,n); endmodule\n",
            [
                crate::state::ProjectSourceFile::try_new(
                    "behavioral/gain_constants.va",
                    "`define RSPICE_GAIN 1.0\n",
                )
                .expect("valid included source"),
                crate::state::ProjectSourceFile::try_new(
                    "behavioral/inactive.va",
                    "module must_not_enter_provenance; endmodule\n",
                )
                .expect("valid inactive source"),
            ],
            [
                crate::state::ProjectSourceDependency::try_new(
                    "behavioral/gain.va",
                    "behavioral/gain_constants.va",
                )
                .expect("valid dependency edge"),
                crate::state::ProjectSourceDependency::try_new(
                    "behavioral/gain.va",
                    "behavioral/inactive.va",
                )
                .expect("valid inactive dependency edge"),
            ],
        )
        .expect("valid source closure");
        harness
            .model
            .workspace
            .content
            .project_sources
            .insert_bundle(bundle)
            .expect("attach cell-view source");

        let mut placed = crate::state::LibraryCellInstance::new("behavioral", "gain", "schematic");
        placed.terminal_order = vec!["p".to_owned(), "n".to_owned()];
        harness
            .model
            .schematic
            .add_library_cell_component(crate::state::Point::new(20, 20), placed);
        harness
            .model
            .workspace
            .content
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Mixed-signal".to_owned(),
                root: crate::state::CellViewRef::default_top(),
                dut_path: "/X1".to_owned(),
                executable_view_policy: vec!["veriloga".to_owned()],
                stop_views: vec!["veriloga".to_owned()],
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                cell_bindings: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Mixed-signal design".to_owned(),
            })
            .expect("create mixed-signal configuration");
        let projection = harness
            .model
            .workspace
            .configuration_execution_projection(
                &harness.model.library_manager,
                &harness.model.workspace.content.active_view,
                &harness.model.schematic,
            )
            .expect("resolve configured behavioral view");
        let source_key = projection
            .plan()
            .binding(&crate::state::InstancePath::parse("/X1").expect("fixture instance path"))
            .and_then(|binding| binding.project_veriloga())
            .expect("project Verilog-A binding")
            .source_key()
            .to_owned();
        (harness, source_key)
    }

    /// Open `text` as the deck at `C:/p/amp.cir`, in front.
    fn open_test_deck(harness: &mut SourceHarness, text: &str) {
        harness.state.netlist.open_deck = Some(
            crate::workbench::documents::deck_files::OpenDeck::in_memory("C:/p/amp.cir", text),
        );
        assert!(crate::workbench::documents::deck_files::show_deck(
            &mut harness.model,
            &mut harness.state.netlist
        ));
    }

    /// The open deck's text, as the session holds it.
    fn deck_text(harness: &SourceHarness) -> &str {
        &harness.state.netlist.open_deck.as_ref().unwrap().root.text
    }

    #[test]
    fn the_open_deck_and_the_generated_primary_coexist_without_overwrite() {
        let mut harness = harness();
        retain_generated(
            &mut harness.model,
            &mut harness.state.netlist,
            "generated\n.end\n",
        );
        let retained_generated = harness.model.netlist_artifacts.generated_source.clone();
        open_test_deck(&mut harness, "deck\n.end\n");
        assert!(
            crate::workbench::documents::netlist_document::edit_deck_file(
                &mut harness.model,
                &mut harness.state.netlist,
                "deck edit\n.end\n".to_owned()
            )
        );

        assert!(
            crate::workbench::documents::netlist_document::open_generated_primary_in(
                &mut harness.model,
                &mut harness.state.netlist
            )
        );
        assert_eq!(
            harness.model.simulation.source.netlist_content,
            retained_generated
        );
        assert_eq!(deck_text(&harness), "deck edit\n.end\n");
        assert!(
            !crate::workbench::documents::netlist_document::open_generated_primary_in(
                &mut harness.model,
                &mut harness.state.netlist
            )
        );
    }

    #[test]
    fn showing_the_open_deck_never_overwrites_its_text() {
        let mut harness = harness();
        retain_generated(
            &mut harness.model,
            &mut harness.state.netlist,
            "new generated\n.end\n",
        );
        open_test_deck(&mut harness, "the deck\n.end\n");
        assert!(
            crate::workbench::documents::netlist_document::open_generated_primary_in(
                &mut harness.model,
                &mut harness.state.netlist
            )
        );

        assert!(crate::workbench::documents::deck_files::show_deck(
            &mut harness.model,
            &mut harness.state.netlist
        ));
        assert_eq!(
            harness.model.simulation.source.netlist_content,
            "the deck\n.end\n"
        );
        assert_eq!(deck_text(&harness), "the deck\n.end\n");
    }

    #[test]
    fn empty_generated_primary_reports_blocked_without_claiming_staleness() {
        let mut harness = harness();
        harness.state.netlist.front = NetlistTab::Generated;
        harness.model.netlist_artifacts.generation_error =
            Some("Add a circuit to the schematic to generate its netlist.".to_owned());

        assert!(generated_primary_unavailable(
            &harness.model,
            &harness.state.netlist
        ));
        assert!(!generated_primary_ready_in(&harness.model));
        assert_eq!(
            document_error_status(
                &harness.model,
                &harness.state.netlist,
                harness.wb.messages()
            ),
            Some(("could not generate".to_owned(), Problem::Warning))
        );
    }

    #[test]
    fn retained_generated_primary_reports_stale_when_regeneration_is_blocked() {
        let mut harness = harness();
        retain_generated(
            &mut harness.model,
            &mut harness.state.netlist,
            "retained\n.end\n",
        );
        harness.state.netlist.front = NetlistTab::Generated;
        harness.model.netlist_artifacts.generation_error = Some("Regeneration failed.".to_owned());

        assert!(!generated_primary_unavailable(
            &harness.model,
            &harness.state.netlist
        ));
        assert!(generated_primary_ready_in(&harness.model));
        assert_eq!(
            document_error_status(
                &harness.model,
                &harness.state.netlist,
                harness.wb.messages()
            ),
            Some((
                "older than the schematic · could not generate".to_owned(),
                Problem::Warning
            ))
        );
    }

    #[test]
    fn split_generated_state_fails_closed_as_unavailable() {
        let mut harness = harness();
        retain_generated(
            &mut harness.model,
            &mut harness.state.netlist,
            "retained\n.end\n",
        );
        harness.state.netlist.front = NetlistTab::Generated;
        harness.model.netlist_artifacts.generated_source.clear();
        harness.model.netlist_artifacts.generation_error = None;

        assert!(harness.model.netlist_artifacts.generated_document.is_some());
        assert!(generated_primary_unavailable(
            &harness.model,
            &harness.state.netlist
        ));
        assert!(!generated_primary_ready_in(&harness.model));
        assert!(!active_document_available(
            &harness.model,
            &harness.state.netlist,
            &harness.runs
        ));
        assert_eq!(
            document_error_status(
                &harness.model,
                &harness.state.netlist,
                harness.wb.messages()
            ),
            Some(("could not generate".to_owned(), Problem::Warning))
        );
    }

    /// The header counts the errors the check found, of every kind: an
    /// unknown model or subcircuit is an error, not a syntax error, and a
    /// warning is not one of them.
    #[test]
    fn the_header_counts_errors_of_every_kind_and_leaves_warnings_out() {
        use crate::state::netlist_diagnostics::{Diagnostic, NetlistDiagnosticCollection};
        let deck = "* amp\nQ1 c b 0 nosuch\nX1 a b nosub\nR1 a 0 1k\n.op\n.end\n";
        let mut harness = harness();
        open_test_deck(&mut harness, deck);
        let mut warning = Diagnostic::error("R1 has one connection.").with_line(Some(3));
        warning.severity =
            crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning;
        harness.state.netlist.diagnostics = std::sync::Arc::new(
            NetlistDiagnosticCollection::try_new(
                vec![
                    Diagnostic::error("Unknown model 'nosuch' in this deck.").with_line(Some(1)),
                    Diagnostic::error("Unknown subcircuit `nosub`.").with_line(Some(2)),
                    warning,
                ],
                deck,
            )
            .unwrap(),
        );

        assert_eq!(
            document_error_status(
                &harness.model,
                &harness.state.netlist,
                harness.wb.messages()
            ),
            Some(("2 errors".to_owned(), Problem::Failure))
        );
    }

    fn ran_owned_deck(deck: &str) -> SourceHarness {
        let mut harness = harness();
        open_test_deck(&mut harness, deck);
        harness.runs.manual_run.last_run_buffer = Some(deck.into());
        harness.runs.manual_run.last_run_id = Some(7);
        seal_manual_run(&mut harness.model, 7, 0xAB, 4);
        harness
    }

    #[test]
    fn run_strip_states_the_run_identity_from_the_run_receipt_alone() {
        let harness = ran_owned_deck("deck\nR1 out 0 1k\n.op\n.end\n");

        let projection =
            run_strip_projection(&harness.model, &harness.state.netlist, &harness.runs)
                .expect("a completed run owns the strip");
        assert_eq!(projection.phase, RunStripPhase::Current);
        assert_eq!(projection.run_id, 7);
        assert_eq!(projection.revision, 4);
    }

    /// The strip says the deck moved past the run on an edit of the same
    /// length, and that it is current again once it is back to the text the
    /// run read, at a revision of its own.
    #[test]
    fn run_strip_follows_the_working_deck_past_the_run_and_back() {
        let mut harness = ran_owned_deck("deck\nR1 out 0 1k\n.op\n.end\n");
        assert!(
            crate::workbench::documents::netlist_document::edit_deck_file(
                &mut harness.model,
                &mut harness.state.netlist,
                "deck\nR1 out 0 2k\n.op\n.end\n".to_owned()
            )
        );

        assert_eq!(
            run_strip_projection(&harness.model, &harness.state.netlist, &harness.runs)
                .map(|projection| projection.phase),
            Some(RunStripPhase::Edited)
        );

        assert!(
            crate::workbench::documents::netlist_document::edit_deck_file(
                &mut harness.model,
                &mut harness.state.netlist,
                "deck\nR1 out 0 1k\n.op\n.end\n".to_owned()
            )
        );
        assert_eq!(
            run_strip_projection(&harness.model, &harness.state.netlist, &harness.runs)
                .map(|projection| projection.phase),
            Some(RunStripPhase::Current)
        );
    }

    #[test]
    fn run_strip_states_the_run_in_flight_and_never_the_stale_baseline() {
        let mut harness = ran_owned_deck("deck\nR1 out 0 1k\n.op\n.end\n");
        harness
            .state
            .netlist
            .open_deck
            .as_mut()
            .unwrap()
            .root
            .text
            .set("deck\nR1 out 0 2k\n.op\n.end\n".to_owned());
        seal_manual_run(&mut harness.model, 8, 0xCD, 5);
        harness.runs.manual_run.pending_manual_run_id = Some(8);

        let projection =
            run_strip_projection(&harness.model, &harness.state.netlist, &harness.runs)
                .expect("an active run owns the strip");
        assert_eq!(projection.phase, RunStripPhase::Running);
        assert_eq!(projection.run_id, 8);
        assert_eq!(projection.revision, 5);
    }

    #[test]
    fn run_strip_is_silent_without_a_retained_manual_run() {
        let mut harness = ran_owned_deck("deck\nR1 out 0 1k\n.op\n.end\n");
        harness.model.simulation.retained.runs.clear();

        assert!(
            run_strip_projection(&harness.model, &harness.state.netlist, &harness.runs).is_none()
        );
    }

    #[test]
    fn run_strip_does_not_speak_for_an_include_or_a_comparison() {
        let mut harness = ran_owned_deck("deck\nR1 out 0 1k\n.op\n.end\n");
        harness.state.netlist.front = NetlistTab::Compare {
            left: crate::workbench::documents::netlist_document::CompareSide::PreviousGenerated(0),
            right: crate::workbench::documents::netlist_document::CompareSide::Generated,
        };
        assert!(
            run_strip_projection(&harness.model, &harness.state.netlist, &harness.runs).is_none()
        );

        harness.state.netlist.front = NetlistTab::Include {
            deck: harness.state.netlist.open_deck.as_ref().map(|deck| deck.id),
            path: "models/r.inc".to_owned(),
        };
        assert!(
            run_strip_projection(&harness.model, &harness.state.netlist, &harness.runs).is_none()
        );
    }

    #[test]
    fn run_snapshot_document_owns_the_strip_as_its_immutable_header() {
        let mut harness = ran_owned_deck("deck\nR1 out 0 1k\n.op\n.end\n");
        assert!(
            crate::workbench::documents::netlist_document::open_run_deck_snapshot_in(
                &mut harness.model,
                &mut harness.state.netlist,
                &harness.runs
            )
        );

        let projection =
            run_strip_projection(&harness.model, &harness.state.netlist, &harness.runs)
                .expect("the snapshot states its run");
        assert_eq!(projection.phase, RunStripPhase::Snapshot);
        assert_eq!(projection.run_id, 7);
        assert!(
            !crate::workbench::documents::netlist_document::active_netlist_source_is_editable_in(
                &harness.model,
                &harness.state.netlist
            )
        );
    }

    /// The strip's route into a run's task decks, and what it says about the
    /// one on screen.
    ///
    /// The hop exists only where it can be honoured: a run whose decks this
    /// project does not hold offers nothing, a run holding one task offers no
    /// step, and a run holding several cycles through them. The badge follows
    /// the deck rather than the run, so it appears only once a task deck is
    /// what the editor below is showing.
    #[test]
    fn the_run_strip_routes_into_the_task_decks_and_states_what_they_verified_as() {
        use crate::state::{ExecutedDeck, ExecutedDeckPoint};

        const DECK: &str = "deck\nR1 out 0 1k\n.op\n.end\n";

        let mut harness = ran_owned_deck(DECK);
        let baseline = run_strip_projection(&harness.model, &harness.state.netlist, &harness.runs)
            .expect("a completed run owns the strip");
        assert_eq!(baseline.tasks, 0);
        assert_eq!(
            task_deck_hop(&baseline),
            None,
            "a run whose decks are not held offers no route to them"
        );
        assert_eq!(baseline.verification, None);

        let point = |label: &str, text: &str| {
            let deck: std::sync::Arc<str> = std::sync::Arc::from(text);
            ExecutedDeckPoint {
                label: label.to_owned(),
                model_sources: crate::state::sealed_model_sources(&deck),
                deck,
            }
        };
        harness
            .model
            .simulation
            .retained
            .executed_decks
            .retain(ExecutedDeck {
                run_id: 7,
                points: vec![
                    point("TT 27C", DECK),
                    point("SS 27C", "deck\n.lib cmos.lib ss\nR1 out 0 1k\n.op\n.end\n"),
                ],
            });

        let held = run_strip_projection(&harness.model, &harness.state.netlist, &harness.runs)
            .expect("the run still owns the strip");
        assert_eq!(held.tasks, 2);
        assert_eq!(
            task_deck_hop(&held),
            Some(0),
            "a baseline header routes to the run's first task"
        );

        assert!(
            crate::workbench::documents::netlist_document::open_executed_deck_in(
                &mut harness.model,
                &mut harness.state.netlist,
                &harness.runs,
                7,
                0,
            ),
            "the route the strip offers is one the document honours"
        );
        let first = run_strip_projection(&harness.model, &harness.state.netlist, &harness.runs)
            .expect("the executed deck owns the strip");
        assert_eq!(first.phase, RunStripPhase::Snapshot);
        assert_eq!(first.point.as_deref(), Some("TT 27C"));
        assert_eq!(first.point_index, 0);
        assert!(
            first.verification.is_some(),
            "a deck on screen is a deck the strip has a verdict about"
        );
        assert_eq!(
            task_deck_hop(&first),
            Some(1),
            "and the next task is one row away"
        );

        assert!(
            crate::workbench::documents::netlist_document::open_executed_deck_in(
                &mut harness.model,
                &mut harness.state.netlist,
                &harness.runs,
                7,
                1,
            )
        );
        let last = run_strip_projection(&harness.model, &harness.state.netlist, &harness.runs)
            .expect("the executed deck owns the strip");
        assert_eq!(last.point.as_deref(), Some("SS 27C"));
        assert_eq!(
            task_deck_hop(&last),
            Some(0),
            "the last task wraps rather than dead-ending"
        );
    }

    #[test]
    fn generated_deck_does_not_inject_unreferenced_project_wide_veriloga() {
        let harness = harness();
        let source = "R1 1 0 1k\n.end\n".to_owned();

        let dependencies = generation::generated_project_source_dependencies(
            &harness.model,
            &source,
            crate::workbench::MessageCatalog::new(
                crate::workbench::UiTextLocale::EnglishUnitedStates,
            ),
        )
        .expect("unreferenced source does not create a dependency");
        assert!(dependencies.is_empty());
    }

    #[test]
    fn generated_deck_retains_the_exact_transitive_cell_view_source_closure() {
        let (harness, source_key) = configured_veriloga_state();
        let source = format!("configured deck\n.veriloga \"{source_key}\" sealed_gain\n.end\n");

        let dependencies = generation::generated_project_source_dependencies(
            &harness.model,
            &source,
            crate::workbench::MessageCatalog::new(
                crate::workbench::UiTextLocale::EnglishUnitedStates,
            ),
        )
        .expect("retain exact project source closure");

        assert_eq!(dependencies.len(), 2);
        assert!(dependencies.iter().all(|dependency| {
            !dependency
                .resolution()
                .source()
                .is_some_and(|source| source.contains("must_not_enter_provenance"))
        }));
        let root = dependencies
            .iter()
            .find(|dependency| dependency.direct_include_index().is_some())
            .expect("direct root dependency");
        assert_eq!(root.requested_locator(), source_key);
        assert!(root.parent().is_none());
        assert!(
            root.resolution()
                .source()
                .unwrap()
                .contains("module sealed_gain")
        );
        let included = dependencies
            .iter()
            .find(|dependency| dependency.parent().is_some())
            .expect("transitive included dependency");
        assert_eq!(included.requested_locator(), "behavioral/gain_constants.va");
        assert_eq!(
            included.resolution().source(),
            Some("`define RSPICE_GAIN 1.0\n")
        );
    }
}
