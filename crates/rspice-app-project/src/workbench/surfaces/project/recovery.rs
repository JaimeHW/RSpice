//! The recovery page: the project's checkpoints and where they are kept.
//!
//! A checkpoint is a complete, integrity-checked copy of the project. Comparing
//! or restoring one never mutates or consumes it, and restoring one first keeps
//! the project as it was as a checkpoint of its own, so going back cannot cost
//! the reader the state they went back from.

use super::page::{self, BODY_TOP, CARD_GAP, HEADER_TOP, STACK_BREAKPOINT, paint_elided};
pub use super::*;
use crate::workbench::app::dialogs::project::{CheckpointComparison, CheckpointDeletion};
use crate::workbench::design_system::{property_row_path, property_row_problem};
use crate::workbench::lifecycle::project_checkpoint::{
    CheckpointEdit, ProjectCheckpointQuarantine, ProjectCheckpointSummary, budget_label,
    budget_use, normalized_note,
};
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::project::lifecycle_view;
use rspice_simulation_contract::run_set::format_bytes;

/// The one sentence the page opens with: what a checkpoint is, and the
/// promise restoring one keeps.
const RECOVERY_NOTE: &str = "A checkpoint is a complete, verified copy of the project. \
     Restoring one puts it back as unsaved changes, after keeping the project as it is now \
     as a checkpoint.";
/// The room over and under a checkpoint row's two lines, and between them.
const ROW_PAD: f32 = 9.0;
const LINE_GAP: f32 = 3.0;
/// Below this card width a row's actions move under its text.
const CHECKPOINT_ROW_STACK_WIDTH: f32 = 460.0;
const ROW_INSET: f32 = 12.0;
/// Side of a problem mark set in front of a line on this page, and the
/// advance from its left edge to the text after it: read from the kit.
use crate::ui::widgets::{PROBLEM_MARK_ADVANCE as MARK_ADVANCE, PROBLEM_MARK_SIDE as MARK_SIDE};

#[cfg(target_arch = "wasm32")]
struct BrowserManualCheckpointCompletion {
    project_id: String,
    result:
        Result<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary, String>,
}

#[cfg(target_arch = "wasm32")]
use crate::workbench::app::dialogs::project::{
    BROWSER_CHECKPOINT_CHANGES, BrowserCheckpointChange,
};
use crate::workbench::app::dialogs::project::{CHECKPOINT_NOT_CREATED, ChangeOutcome};

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_MANUAL_CHECKPOINT_PENDING: std::cell::Cell<bool> =
        const { std::cell::Cell::new(false) };
    static BROWSER_MANUAL_CHECKPOINT_COMPLETIONS: std::cell::RefCell<std::collections::VecDeque<BrowserManualCheckpointCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

/// Everything the page draws, read before any of it is drawn, so an action
/// taken on this frame changes the next one rather than half of this one.
struct RecoverySnapshot {
    checkpoints: Vec<ProjectCheckpointSummary>,
    /// The artifacts that failed verification.
    set_aside: Vec<ProjectCheckpointQuarantine>,
    /// What the reader asked to delete, waiting for the answer.
    delete_review: Option<CheckpointDeletion>,
    /// The checkpoint a note is being written on.
    note_target: Option<uuid::Uuid>,
    error: Option<String>,
    loading: bool,
    creating: bool,
    /// The checkpoint a running comparison is comparing with.
    comparing: Option<uuid::Uuid>,
    /// The checkpoint a running restore is putting back.
    restoring: Option<uuid::Uuid>,
    /// The checkpoint the reader asked to restore, waiting for the answer.
    restore_review: Option<uuid::Uuid>,
    /// A checkpoint saved as a new project, and where.
    saved_copy: Option<(uuid::Uuid, std::path::PathBuf)>,
    /// What the last comparison found, for the checkpoint it compared.
    comparison: Option<(uuid::Uuid, CheckpointComparison)>,
    /// Checkpoint work is running; another waits for it.
    busy: bool,
    selected: Option<String>,
    location: String,
    stored_bytes: u64,
    /// The disk space automatic checkpoints are kept inside, and how much
    /// of it they take.
    budget_bytes: u64,
    budget_used: u64,
    now_unix_ms: Option<u64>,
}

impl RecoverySnapshot {
    fn capture(cx: &Cx<'_, ProjectFeature>) -> Self {
        let recovery = &cx.state.project_checkpoint_recovery;
        #[cfg(target_arch = "wasm32")]
        let (loading, creating, comparing, restoring) = (
            recovery.loading && !recovery.initialized,
            manual_checkpoint_pending(),
            None,
            browser_restore_pending(),
        );
        #[cfg(not(target_arch = "wasm32"))]
        let (loading, creating, comparing, restoring) = {
            use crate::workbench::app::dialogs::project::CheckpointJobKind;
            let kind = recovery.job.as_ref().map(|job| &job.kind);
            (
                !recovery.initialized && recovery.error.is_none(),
                kind == Some(&CheckpointJobKind::Create),
                match kind {
                    Some(CheckpointJobKind::Compare(checkpoint)) => {
                        Some(checkpoint.checkpoint_id())
                    }
                    _ => None,
                },
                match kind {
                    Some(CheckpointJobKind::Restore(checkpoint)) => {
                        Some(checkpoint.checkpoint_id())
                    }
                    _ => None,
                },
            )
        };
        #[cfg(target_arch = "wasm32")]
        let busy = creating || recovery.changing || restoring.is_some();
        #[cfg(not(target_arch = "wasm32"))]
        let busy = recovery.job.is_some();
        Self {
            checkpoints: recovery.checkpoints.clone(),
            set_aside: recovery.quarantined.clone(),
            delete_review: recovery.delete_review.clone(),
            note_target: recovery.note_draft.as_ref().map(|(id, _)| *id),
            error: recovery.error.clone(),
            loading,
            creating,
            comparing,
            restoring,
            restore_review: recovery.restore_review,
            saved_copy: recovery.saved_copy.clone(),
            comparison: recovery.comparison.clone(),
            busy,
            selected: cx.state.project_checkpoint_selection.clone(),
            location: recovery.location.clone().unwrap_or_default(),
            stored_bytes: recovery
                .checkpoints
                .iter()
                .map(ProjectCheckpointSummary::snapshot_byte_len)
                .sum(),
            budget_bytes: crate::workbench::lifecycle::project_checkpoint::budget_bytes(cx.model),
            budget_used: budget_use(&recovery.checkpoints),
            now_unix_ms: crate::time_compat::checked_unix_time_ms().ok(),
        }
    }

    /// The page's one-word state beside its title, and the mark it earns.
    ///
    /// Counting the checkpoints is the fact; calling them verified was the
    /// page reporting that nothing had gone wrong, in green.
    fn status(&self) -> (String, Option<Problem>) {
        if self.error.is_some() {
            ("unreadable".to_owned(), Some(Problem::Failure))
        } else if self.loading {
            ("listing".to_owned(), None)
        } else if !self.set_aside.is_empty() {
            (
                format!("{} set aside", self.set_aside.len()),
                Some(Problem::Warning),
            )
        } else if self.checkpoints.is_empty() {
            ("no checkpoints".to_owned(), None)
        } else if self.checkpoints.len() == 1 {
            ("1 checkpoint".to_owned(), None)
        } else {
            (format!("{} checkpoints", self.checkpoints.len()), None)
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn storage_location(model: &crate::workbench::app_state::Model) -> String {
    crate::workbench::lifecycle::project_checkpoint::storage_location(model)
        .map_or_else(|error| error, |path| path.display().to_string())
}

#[cfg(target_arch = "wasm32")]
fn storage_location(_model: &crate::workbench::app_state::Model) -> String {
    "This browser's site storage".to_owned()
}

enum RecoveryIntent {
    Create,
    RevisionHistory,
    Reload,
    Select(String),
    Compare(ProjectCheckpointSummary),
    /// Put away what the last comparison found.
    CloseComparison,
    /// Ask whether to restore it, in place.
    AskRestore(ProjectCheckpointSummary),
    CancelRestore,
    Restore(ProjectCheckpointSummary),
    /// Write it as a new project file, asking where.
    SaveAsNewProject(ProjectCheckpointSummary),
    /// Open the project a checkpoint was saved as.
    OpenSavedCopy(std::path::PathBuf),
    DismissSavedCopy,
    /// Start writing a note on the checkpoint.
    WriteNote(ProjectCheckpointSummary),
    /// Keep the note being written.
    KeepNote(ProjectCheckpointSummary),
    CancelNote,
    Pin(ProjectCheckpointSummary, bool),
    /// Ask whether to delete it, in place.
    AskDelete(CheckpointDeletion),
    CancelDelete,
    Delete(ProjectCheckpointSummary),
    DeleteSetAside(ProjectCheckpointQuarantine),
}

impl RecoveryIntent {
    fn execute(self, ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) {
        match self {
            Self::Create => create_manual_checkpoint(ctx, cx),
            Self::RevisionHistory => cx.execute(ProjectCommand::RevisionHistory.key()),
            Self::Reload => cx.state.project_checkpoint_recovery.invalidate(),
            Self::Select(id) => cx.state.project_checkpoint_selection = Some(id),
            Self::Compare(checkpoint) => {
                cx.state.project_checkpoint_selection =
                    Some(checkpoint.checkpoint_id().to_string());
                compare_project_checkpoint(ctx, cx, checkpoint);
            }
            Self::CloseComparison => cx.state.project_checkpoint_recovery.comparison = None,
            Self::AskRestore(checkpoint) => {
                let recovery = &mut cx.state.project_checkpoint_recovery;
                recovery.note_draft = None;
                recovery.delete_review = None;
                recovery.restore_review = Some(checkpoint.checkpoint_id());
                cx.state.project_checkpoint_selection =
                    Some(checkpoint.checkpoint_id().to_string());
            }
            Self::CancelRestore => cx.state.project_checkpoint_recovery.restore_review = None,
            Self::Restore(checkpoint) => {
                cx.state.project_checkpoint_recovery.restore_review = None;
                restore_project_checkpoint(ctx, cx, checkpoint);
            }
            Self::SaveAsNewProject(checkpoint) => {
                cx.state.project_checkpoint_selection =
                    Some(checkpoint.checkpoint_id().to_string());
                export_project_checkpoint_copy(ctx, cx, checkpoint);
            }
            Self::OpenSavedCopy(path) => {
                cx.state.project_checkpoint_recovery.saved_copy = None;
                // Leaving the open project asks about its unsaved work first,
                // as Open recent does.
                cx.wb.outbox.push(
                    crate::workbench::project::commands::ProjectRequest::GoOnLeaving {
                        action: crate::workbench::app::ConfirmationAction::OpenRecent,
                        recent: Some((path, crate::services::recent_files::RecentKind::Project)),
                    },
                );
            }
            Self::DismissSavedCopy => cx.state.project_checkpoint_recovery.saved_copy = None,
            Self::WriteNote(checkpoint) => {
                let recovery = &mut cx.state.project_checkpoint_recovery;
                recovery.delete_review = None;
                recovery.restore_review = None;
                recovery.note_draft = Some((
                    checkpoint.checkpoint_id(),
                    checkpoint.note().unwrap_or_default().to_owned(),
                ));
                cx.state.project_checkpoint_selection =
                    Some(checkpoint.checkpoint_id().to_string());
            }
            Self::KeepNote(checkpoint) => {
                let Some((_, draft)) = cx.state.project_checkpoint_recovery.note_draft.take()
                else {
                    return;
                };
                match normalized_note(&draft) {
                    Ok(note) if note.as_deref() == checkpoint.note() => {}
                    Ok(note) => change_checkpoint(
                        ctx,
                        cx,
                        CheckpointChange::Edit(checkpoint, CheckpointEdit::Note(note)),
                    ),
                    Err(error) => cx.model.report_failure(CHECKPOINT_NOT_CHANGED, error),
                }
            }
            Self::CancelNote => cx.state.project_checkpoint_recovery.note_draft = None,
            Self::Pin(checkpoint, pinned) => change_checkpoint(
                ctx,
                cx,
                CheckpointChange::Edit(checkpoint, CheckpointEdit::Pin(pinned)),
            ),
            Self::AskDelete(deletion) => {
                let recovery = &mut cx.state.project_checkpoint_recovery;
                recovery.note_draft = None;
                recovery.restore_review = None;
                if let CheckpointDeletion::Checkpoint(id) = &deletion {
                    cx.state.project_checkpoint_selection = Some(id.to_string());
                }
                recovery.delete_review = Some(deletion);
            }
            Self::CancelDelete => cx.state.project_checkpoint_recovery.delete_review = None,
            Self::Delete(checkpoint) => {
                cx.state.project_checkpoint_recovery.delete_review = None;
                change_checkpoint(ctx, cx, CheckpointChange::Delete(checkpoint));
            }
            Self::DeleteSetAside(artifact) => {
                cx.state.project_checkpoint_recovery.delete_review = None;
                change_checkpoint(ctx, cx, CheckpointChange::DeleteSetAside(artifact));
            }
        }
    }
}

/// A change the reader makes to a checkpoint, or to what was set aside.
enum CheckpointChange {
    Edit(ProjectCheckpointSummary, CheckpointEdit),
    Delete(ProjectCheckpointSummary),
    DeleteSetAside(ProjectCheckpointQuarantine),
}

/// The notice a note or pin that could not be written raises.
const CHECKPOINT_NOT_CHANGED: &str = "Checkpoint not changed";

impl CheckpointChange {
    /// The Console line the change leaves once it is made. A note or a pin
    /// shows on the row it changed; a deletion leaves nothing to look at.
    fn record(&self) -> Option<String> {
        match self {
            Self::Edit(..) => None,
            Self::Delete(checkpoint) => Some(format!(
                "Deleted checkpoint \u{201c}{}\u{201d} from {}.",
                checkpoint.title(),
                crate::time_compat::utc_stamp(checkpoint.created_unix_ms())
            )),
            Self::DeleteSetAside(artifact) => Some(format!(
                "Deleted the damaged checkpoint {}.",
                artifact.label()
            )),
        }
    }

    const fn failed(&self) -> &'static str {
        match self {
            Self::Edit(..) => CHECKPOINT_NOT_CHANGED,
            Self::Delete(_) | Self::DeleteSetAside(_) => "Checkpoint not deleted",
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn apply(self) -> ChangeOutcome {
        use crate::workbench::lifecycle::project_checkpoint as store;
        let (record, failed) = (self.record(), self.failed());
        match self {
            Self::Edit(checkpoint, edit) => store::edit(&checkpoint, &edit),
            Self::Delete(checkpoint) => store::delete(&checkpoint),
            Self::DeleteSetAside(artifact) => store::delete_set_aside(&artifact),
        }
        .map(|()| record)
        .map_err(|error| (failed, error))
    }
}

/// Make a change on a background thread and list the checkpoints after it.
#[cfg(not(target_arch = "wasm32"))]
fn change_checkpoint(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>, change: CheckpointChange) {
    use crate::workbench::app::dialogs::project::{
        CheckpointJob, CheckpointJobAnswer, CheckpointJobKind,
    };
    if cx.state.project_checkpoint_recovery.job.is_some() {
        return;
    }
    let listing = match crate::workbench::lifecycle::project_checkpoint::listing(cx.model) {
        Ok(listing) => listing,
        Err(error) => {
            cx.model.report_failure(change.failed(), error);
            return;
        }
    };
    let source = catalog_source(cx);
    let recovery = &mut cx.state.project_checkpoint_recovery;
    recovery.listed_for = Some(source);
    recovery.job = Some(CheckpointJob::start(
        ctx,
        CheckpointJobKind::Change,
        recovery,
        move || CheckpointJobAnswer::Changed {
            outcome: change.apply(),
            listed: listing.list(),
        },
    ));
}

/// Start a change in the browser store; its answer is taken by
/// [`poll_browser_checkpoint_changes`].
#[cfg(target_arch = "wasm32")]
fn change_checkpoint(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>, change: CheckpointChange) {
    use crate::workbench::lifecycle::project_checkpoint as store;
    if cx.state.project_checkpoint_recovery.changing {
        return;
    }
    cx.state.project_checkpoint_recovery.changing = true;
    let project_id = cx.model.workspace.content.project.id().to_string();
    let (record, failed) = (change.record(), change.failed());
    let repaint = ctx.clone();
    let complete = move |result: Result<(), String>| {
        BROWSER_CHECKPOINT_CHANGES.with(|queue| {
            queue.borrow_mut().push_back(BrowserCheckpointChange {
                project_id,
                outcome: result.map(|()| record).map_err(|error| (failed, error)),
                from_page: true,
            });
        });
        repaint.request_repaint();
    };
    match change {
        CheckpointChange::Edit(checkpoint, edit) => store::start_edit(&checkpoint, edit, complete),
        CheckpointChange::Delete(checkpoint) => store::start_delete(&checkpoint, complete),
        CheckpointChange::DeleteSetAside(artifact) => {
            store::start_delete_set_aside(&artifact, complete);
        }
    }
}

/// Say what a change did.
fn report_change(cx: &mut Cx<'_, ProjectFeature>, outcome: ChangeOutcome) {
    match outcome {
        Ok(Some(line)) => {
            cx.model.record(LogSeverity::Info, LogSource::Studio, line);
        }
        Ok(None) => {}
        Err((title, error)) => cx.model.report_failure(title, error),
    }
}

/// Take what the browser store finished: the page's own changes, and the
/// work a save left it, whose failures are said here since nothing else is
/// waiting for them.
#[cfg(target_arch = "wasm32")]
pub(super) fn poll_browser_checkpoint_changes(cx: &mut Cx<'_, ProjectFeature>) {
    let project_id = cx.model.workspace.content.project.id().to_string();
    let completions =
        BROWSER_CHECKPOINT_CHANGES.with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    for completion in completions {
        if completion.from_page {
            cx.state.project_checkpoint_recovery.changing = false;
        }
        if completion.project_id != project_id {
            continue;
        }
        cx.state.project_checkpoint_recovery.invalidate();
        report_change(cx, completion.outcome);
    }
    for failure in crate::workbench::lifecycle::project_checkpoint::take_browser_store_failures() {
        cx.model.report_warning(
            "Checkpoints not updated",
            format!("The project was saved, but {failure}."),
        );
    }
}

/// What the catalog is listed for: the saved project, whose generation a
/// save moves, and in the browser the store's own finished changes.
fn catalog_source(cx: &Cx<'_, ProjectFeature>) -> (u64, u64) {
    let saved = crate::workbench::lifecycle::project_lifecycle::accepted_generation(cx.platform);
    #[cfg(target_arch = "wasm32")]
    let store = crate::workbench::lifecycle::project_checkpoint::browser_store_changes();
    #[cfg(not(target_arch = "wasm32"))]
    let store = 0;
    (saved, store)
}

fn keep_first(slot: &mut Option<RecoveryIntent>, candidate: Option<RecoveryIntent>) {
    if slot.is_none() {
        *slot = candidate;
    }
}

pub fn recovery(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
    #[cfg(target_arch = "wasm32")]
    poll_browser_manual_checkpoint(cx);
    ensure_project_recovery_catalog(ui.ctx(), cx);
    let snapshot = RecoverySnapshot::capture(cx);
    // The note being written is the one thing on the page typed into: the
    // field edits this copy, which goes back to the state after drawing.
    let mut note_draft = cx
        .state
        .project_checkpoint_recovery
        .note_draft
        .as_ref()
        .map(|(_, draft)| draft.clone())
        .unwrap_or_default();
    let mut intent = None;
    page::show(
        ui,
        "workbench.project.recovery.page",
        |ui, inset, content_width| {
            keep_first(
                &mut intent,
                recovery_header(ui, &snapshot, inset, content_width),
            );
            ui.add_space(BODY_TOP);
            if content_width >= STACK_BREAKPOINT {
                let (mut left_intent, mut right_intent) = (None, None);
                page::columns(
                    ui,
                    inset,
                    content_width,
                    |left| left_intent = checkpoints_card(left, &snapshot, &mut note_draft),
                    |right| {
                        storage_card(right, &snapshot);
                        right_intent = set_aside_card(right, &snapshot);
                    },
                );
                keep_first(&mut intent, left_intent);
                keep_first(&mut intent, right_intent);
            } else {
                // One column as tall as its cards: the page, not the column,
                // owns the height, so no card is cut to what is left of it.
                page::stacked_column(ui, inset, content_width, |ui| {
                    keep_first(
                        &mut intent,
                        checkpoints_card(ui, &snapshot, &mut note_draft),
                    );
                    ui.add_space(CARD_GAP);
                    storage_card(ui, &snapshot);
                    keep_first(&mut intent, set_aside_card(ui, &snapshot));
                });
            }
        },
    );
    if let Some((_, draft)) = &mut cx.state.project_checkpoint_recovery.note_draft
        && *draft != note_draft
    {
        *draft = note_draft;
    }
    if let Some(intent) = intent {
        intent.execute(&ui.ctx().clone(), cx);
    }
}

/// Title, state, the page's one note, and its two actions, above the rule
/// every carded project page opens with.
fn recovery_header(
    ui: &mut Ui,
    snapshot: &RecoverySnapshot,
    inset: f32,
    content_width: f32,
) -> Option<RecoveryIntent> {
    let tokens = Tokens::get(ui.ctx());
    ui.add_space(HEADER_TOP);
    let intent = page::inset_column(ui, inset, content_width, |ui| {
        let (status, problem) = snapshot.status();
        let title_font = theme::sans(20.0, FontWeight::SemiBold);
        let title_height = ui.fonts_mut(|fonts| fonts.row_height(&title_font));
        let (title_rect, title_response) =
            ui.allocate_exact_size(vec2(content_width, title_height), Sense::hover());
        let title = ui.painter().text(
            title_rect.left_center(),
            Align2::LEFT_CENTER,
            "Recovery",
            title_font,
            tokens.color.text,
        );
        let status_left = title.right() + 12.0;
        let text_left = if let Some(problem) = problem {
            paint_problem_mark(
                ui.painter(),
                Rect::from_center_size(
                    pos2(status_left + MARK_SIDE * 0.5, title_rect.center().y + 2.0),
                    vec2(MARK_SIDE, MARK_SIDE),
                ),
                problem,
            );
            status_left + MARK_ADVANCE
        } else {
            status_left
        };
        ui.painter().text(
            pos2(text_left, title_rect.center().y + 2.0),
            Align2::LEFT_CENTER,
            &status,
            theme::mono(tokens::FS_0, FontWeight::Medium),
            tokens.color.text_dim,
        );
        let announcement = problem.map_or_else(
            || format!("Recovery · {status}"),
            |problem| problem.announce(&format!("Recovery · {status}")),
        );
        title_response
            .widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), &announcement));

        ui.add_space(3.0);
        text_block(ui, RECOVERY_NOTE, body_font(), tokens.color.text_dim);
        ui.add_space(12.0);
        let mut intent = None;
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = vec2(8.0, 8.0);
            let create = Button::new(if snapshot.creating {
                "Creating checkpoint\u{2026}"
            } else {
                "Create checkpoint"
            })
            .icon(crate::ui::icons::Icon::Add)
            .accent()
            .enabled(!snapshot.busy)
            .show(ui);
            if create.clicked() {
                intent = Some(RecoveryIntent::Create);
            }
            if Button::new("Revision history\u{2026}").show(ui).clicked() {
                intent = Some(RecoveryIntent::RevisionHistory);
            }
        });
        intent
    });
    page::header_rule(ui);
    intent
}

fn checkpoints_card(
    ui: &mut Ui,
    snapshot: &RecoverySnapshot,
    note_draft: &mut String,
) -> Option<RecoveryIntent> {
    // The count is the page's state beside its title; the card does not
    // repeat it.
    page::card(ui, "Checkpoints", "", None, |ui| {
        if let Some(error) = &snapshot.error {
            return catalog_error(ui, error);
        }
        if snapshot.loading {
            card_message(ui, "Listing checkpoints\u{2026}", None);
            return None;
        }
        if snapshot.checkpoints.is_empty() {
            card_message(
                ui,
                "No checkpoints yet",
                Some(
                    "RSpice takes one before each save that changes the project and before \
                     risky changes. Create one yourself before an edit you may want to undo.",
                ),
            );
            return None;
        }
        let mut intent = None;
        let count = snapshot.checkpoints.len();
        for (index, checkpoint) in snapshot.checkpoints.iter().enumerate() {
            let id = checkpoint.checkpoint_id();
            let selected = snapshot.selected.as_deref() == Some(id.to_string().as_str());
            let separated = index + 1 < count;
            let writing_note = snapshot.note_target == Some(id);
            let deleting = snapshot.delete_review == Some(CheckpointDeletion::Checkpoint(id));
            let restoring = snapshot.restore_review == Some(id);
            let saved_copy = snapshot
                .saved_copy
                .as_ref()
                .filter(|(saved, _)| *saved == id)
                .map(|(_, path)| path);
            let comparison = snapshot
                .comparison
                .as_ref()
                .filter(|(compared, _)| *compared == id)
                .map(|(_, found)| found);
            let prompted = writing_note
                || deleting
                || restoring
                || saved_copy.is_some()
                || comparison.is_some();
            keep_first(
                &mut intent,
                checkpoint_row(
                    ui,
                    checkpoint,
                    RowState {
                        selected,
                        comparing: snapshot.comparing == Some(id),
                        restoring: snapshot.restoring == Some(id),
                        busy: snapshot.busy,
                    },
                    snapshot.now_unix_ms,
                    separated && !prompted,
                ),
            );
            if restoring {
                keep_first(
                    &mut intent,
                    row_prompt(ui, selected, separated, |ui| {
                        restore_prompt(ui, checkpoint, snapshot.busy)
                    }),
                );
            } else if let Some(path) = saved_copy {
                keep_first(
                    &mut intent,
                    row_prompt(ui, selected, separated, |ui| saved_copy_prompt(ui, path)),
                );
            } else if writing_note {
                keep_first(
                    &mut intent,
                    row_prompt(ui, selected, separated, |ui| {
                        note_prompt(ui, checkpoint, note_draft, snapshot.busy)
                    }),
                );
            } else if deleting {
                keep_first(
                    &mut intent,
                    row_prompt(ui, selected, separated, |ui| {
                        delete_prompt(
                            ui,
                            &format!(
                                "Delete \u{201c}{}\u{201d} from {}? It cannot be brought back.",
                                checkpoint.title(),
                                crate::time_compat::utc_stamp(checkpoint.created_unix_ms())
                            ),
                            &format!("Delete {}", checkpoint.title()),
                            snapshot.busy,
                        )
                        .map(|delete| {
                            if delete {
                                RecoveryIntent::Delete(checkpoint.clone())
                            } else {
                                RecoveryIntent::CancelDelete
                            }
                        })
                    }),
                );
            } else if let Some(found) = comparison {
                keep_first(
                    &mut intent,
                    row_prompt(ui, selected, separated, |ui| comparison_list(ui, found)),
                );
            }
        }
        intent
    })
}

/// A question asked in place under its row, in the row's own fill: the words
/// and the answers, then the row's rule.
fn row_prompt<R>(
    ui: &mut Ui,
    selected: bool,
    separated: bool,
    body: impl FnOnce(&mut Ui) -> R,
) -> R {
    let tokens = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let background = ui.painter().add(egui::Shape::Noop);
    let shown = ui.scope(|ui| {
        page::inset_column(ui, ROW_INSET, (width - 2.0 * ROW_INSET).max(1.0), |ui| {
            let answer = body(ui);
            ui.add_space(12.0);
            answer
        })
    });
    let rect = Rect::from_min_size(
        shown.response.rect.min,
        vec2(width, shown.response.rect.height()),
    );
    if selected {
        ui.painter().set(
            background,
            egui::Shape::rect_filled(rect, 0.0, tokens.color.bg_active),
        );
        ui.painter().rect_filled(
            Rect::from_min_size(rect.left_top(), vec2(2.0, rect.height())),
            0.0,
            tokens.color.accent,
        );
    }
    if separated {
        ui.painter().hline(
            (rect.left() + ROW_INSET)..=(rect.right() - ROW_INSET),
            rect.bottom() - 0.5,
            Stroke::new(1.0, tokens.color.border),
        );
    }
    shown.inner
}

/// The control height a prompt's field and answers take: the kit's, raised
/// to the touch target on a tablet.
fn prompt_control_height(ui: &Ui) -> f32 {
    let tokens = Tokens::get(ui.ctx());
    if tokens.metrics.is_touch() {
        tokens.metrics.touch_target()
    } else {
        tokens.metrics.ctl_h
    }
}

/// The note field under a checkpoint's row and its two answers. Enter keeps
/// the note, Escape leaves it as it was.
fn note_prompt(
    ui: &mut Ui,
    checkpoint: &ProjectCheckpointSummary,
    draft: &mut String,
    busy: bool,
) -> Option<RecoveryIntent> {
    let height = prompt_control_height(ui);
    let field = ui.add_sized(
        [ui.available_width(), height],
        egui::TextEdit::singleline(draft)
            .id_salt((
                "workbench.project.recovery.note",
                checkpoint.checkpoint_id(),
            ))
            .hint_text("What this checkpoint is for")
            .vertical_align(egui::Align::Center),
    );
    ui.ctx().accesskit_node_builder(field.id, |node| {
        node.set_label(format!("Note on {}", checkpoint.reason().label()));
    });
    if ui.memory(|memory| memory.focused().is_none()) {
        field.request_focus();
    }
    let (enter, escape) = ui.input(|input| {
        (
            input.key_pressed(egui::Key::Enter),
            input.key_pressed(egui::Key::Escape),
        )
    });
    let mut intent = None;
    if field.lost_focus() && escape {
        intent = Some(RecoveryIntent::CancelNote);
    } else if field.lost_focus() && enter && !busy {
        intent = Some(RecoveryIntent::KeepNote(checkpoint.clone()));
    }
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        if Button::new("Keep note")
            .accent()
            .enabled(!busy)
            .min_height(height)
            .show(ui)
            .clicked()
        {
            intent = Some(RecoveryIntent::KeepNote(checkpoint.clone()));
        }
        if Button::new("Cancel").min_height(height).show(ui).clicked() {
            intent = Some(RecoveryIntent::CancelNote);
        }
    });
    intent
}

/// A deletion asked about in place: the question, which names what goes,
/// then Delete and Cancel. Answers whether to delete, once one is clicked.
fn delete_prompt(ui: &mut Ui, question: &str, delete_label: &str, busy: bool) -> Option<bool> {
    let tokens = Tokens::get(ui.ctx());
    let height = prompt_control_height(ui);
    text_block(ui, question, body_font(), tokens.color.text);
    ui.add_space(8.0);
    let mut answer = None;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        if Button::new("Delete")
            .accent()
            .destructive(true)
            .enabled(!busy)
            .min_height(height)
            .accessible_label(delete_label)
            .show(ui)
            .clicked()
        {
            answer = Some(true);
        }
        if Button::new("Cancel").min_height(height).show(ui).clicked() {
            answer = Some(false);
        }
    });
    answer
}

/// A restore asked about in place: what it puts back and what it keeps, then
/// Restore and Cancel.
fn restore_prompt(
    ui: &mut Ui,
    checkpoint: &ProjectCheckpointSummary,
    busy: bool,
) -> Option<RecoveryIntent> {
    let tokens = Tokens::get(ui.ctx());
    let height = prompt_control_height(ui);
    text_block(
        ui,
        &format!(
            "Restore \u{201c}{}\u{201d} from {}? Its project becomes unsaved changes; the \
             project as it is now is kept as \u{201c}{}\u{201d}.",
            checkpoint.title(),
            crate::time_compat::utc_stamp(checkpoint.created_unix_ms()),
            crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointReason::Restore
                .label()
        ),
        body_font(),
        tokens.color.text,
    );
    ui.add_space(8.0);
    let mut intent = None;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        if Button::new("Restore")
            .accent()
            .enabled(!busy)
            .min_height(height)
            .accessible_label(&format!("Restore {}", checkpoint.title()))
            .show(ui)
            .clicked()
        {
            intent = Some(RecoveryIntent::Restore(checkpoint.clone()));
        }
        if Button::new("Cancel").min_height(height).show(ui).clicked() {
            intent = Some(RecoveryIntent::CancelRestore);
        }
    });
    intent
}

/// A checkpoint just saved as a new project: where it went, and the offer to
/// open it.
fn saved_copy_prompt(ui: &mut Ui, path: &std::path::Path) -> Option<RecoveryIntent> {
    let tokens = Tokens::get(ui.ctx());
    let height = prompt_control_height(ui);
    text_block(
        ui,
        &format!("Saved as {}.", path.display()),
        body_font(),
        tokens.color.text,
    );
    ui.add_space(8.0);
    let mut intent = None;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        if Button::new("Open it")
            .accent()
            .min_height(height)
            .accessible_label("Open the saved project")
            .show(ui)
            .clicked()
        {
            intent = Some(RecoveryIntent::OpenSavedCopy(path.to_path_buf()));
        }
        if Button::new("Not now").min_height(height).show(ui).clicked() {
            intent = Some(RecoveryIntent::DismissSavedCopy);
        }
    });
    intent
}

/// What a comparison found: each document in which the open project differs
/// from the checkpoint, and how.
fn comparison_list(ui: &mut Ui, found: &CheckpointComparison) -> Option<RecoveryIntent> {
    let tokens = Tokens::get(ui.ctx());
    let height = prompt_control_height(ui);
    let heading = if found.len() == 1 {
        "The current project differs in 1 document:".to_owned()
    } else {
        format!("The current project differs in {} documents:", found.len())
    };
    text_block(ui, &heading, body_font(), tokens.color.text);
    ui.add_space(4.0);
    let width = ui.available_width();
    let kind_width = 104.0;
    // A line of words and the room around it, at the density's type.
    let line = ui.fonts_mut(|fonts| fonts.row_height(&body_font())) + 4.0;
    for difference in found {
        let (rect, response) = ui.allocate_exact_size(vec2(width, line), Sense::hover());
        let document = difference.document.label();
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), &document));
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_value(difference.kind.label());
        });
        ui.painter().text(
            pos2(rect.left(), rect.center().y),
            Align2::LEFT_CENTER,
            difference.kind.label(),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            tokens.color.text_dim,
        );
        paint_elided(
            ui,
            pos2(rect.left() + kind_width, rect.top() + 2.0),
            &document,
            body_font(),
            tokens.color.text,
            (width - kind_width).max(1.0),
        );
    }
    ui.add_space(8.0);
    let mut intent = None;
    if Button::new("Close")
        .min_height(height)
        .accessible_label("Close the comparison")
        .show(ui)
        .clicked()
    {
        intent = Some(RecoveryIntent::CloseComparison);
    }
    intent
}

/// What can be done to a checkpoint besides comparing and restoring it: the
/// row's More menu and its context menu.
fn checkpoint_menu(
    ui: &mut Ui,
    checkpoint: &ProjectCheckpointSummary,
    busy: bool,
    intent: &mut Option<RecoveryIntent>,
) {
    ui.set_min_width(150.0);
    let (pin_label, pin_hover) = if checkpoint.pinned() {
        (
            "Unpin",
            "Keep it only as long as the rule for its kind does",
        )
    } else {
        ("Pin", "Keep it until you delete it")
    };
    if ui
        .add_enabled(!busy, egui::Button::new(pin_label))
        .on_hover_text(pin_hover)
        .clicked()
    {
        *intent = Some(RecoveryIntent::Pin(
            checkpoint.clone(),
            !checkpoint.pinned(),
        ));
        ui.close();
    }
    let note_label = if checkpoint.note().is_some() {
        "Edit note\u{2026}"
    } else {
        "Add note\u{2026}"
    };
    if ui
        .add_enabled(!busy, egui::Button::new(note_label))
        .clicked()
    {
        *intent = Some(RecoveryIntent::WriteNote(checkpoint.clone()));
        ui.close();
    }
    if ui
        .add_enabled(!busy, egui::Button::new("Save as new project\u{2026}"))
        .on_hover_text("Write this checkpoint as a project file of its own")
        .clicked()
    {
        *intent = Some(RecoveryIntent::SaveAsNewProject(checkpoint.clone()));
        ui.close();
    }
    ui.separator();
    if ui
        .add_enabled(!busy, egui::Button::new("Delete\u{2026}"))
        .clicked()
    {
        *intent = Some(RecoveryIntent::AskDelete(CheckpointDeletion::Checkpoint(
            checkpoint.checkpoint_id(),
        )));
        ui.close();
    }
}

/// A card's whole body when it has no rows: a title and, optionally, a
/// sentence under it.
fn card_message(ui: &mut Ui, title: &str, detail: Option<&str>) {
    let tokens = Tokens::get(ui.ctx());
    ui.add_space(14.0);
    let width = ui.available_width() - 2.0 * (ROW_INSET + 2.0);
    page::inset_column(ui, ROW_INSET + 2.0, width, |ui| {
        text_block(ui, title, title_font(), tokens.color.text);
        if let Some(detail) = detail {
            ui.add_space(4.0);
            text_block(ui, detail, body_font(), tokens.color.text_dim);
        }
    });
    ui.add_space(5.0);
}

/// A checkpoint row's title and detail lines.
fn row_title_font() -> egui::FontId {
    theme::sans(tokens::FS_1, FontWeight::Medium)
}

fn detail_font() -> egui::FontId {
    theme::mono(tokens::FS_0, FontWeight::Regular)
}

/// The least a checkpoint row beside its actions is: its two lines and the
/// room over, between and under them, at the density's type.
pub fn checkpoint_row_least_height(ctx: &Context) -> f32 {
    let (title, detail) = ctx.fonts_mut(|fonts| {
        (
            fonts.row_height(&row_title_font()),
            fonts.row_height(&detail_font()),
        )
    });
    ROW_PAD + title + LINE_GAP + detail + ROW_PAD
}

fn title_font() -> egui::FontId {
    theme::sans(tokens::FS_2, FontWeight::Medium)
}

fn body_font() -> egui::FontId {
    theme::sans(tokens::FS_1, FontWeight::Regular)
}

/// A card's title with the kit's mark in front of it, for the one card whose
/// whole body is the problem. The mark carries the severity; the words stay
/// in the page's ordinary title tone.
fn marked_title(ui: &mut Ui, problem: Problem, text: &str) {
    let tokens = Tokens::get(ui.ctx());
    let font = title_font();
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), font, tokens.color.text);
    let (rect, response) = ui.allocate_exact_size(
        vec2(
            (galley.size().x + MARK_ADVANCE).min(ui.available_width().max(1.0)),
            galley.size().y.max(MARK_SIDE),
        ),
        Sense::hover(),
    );
    let announcement = problem.announce(text);
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), &announcement));
    paint_problem_mark(
        ui.painter(),
        Rect::from_center_size(
            pos2(rect.left() + MARK_SIDE * 0.5, rect.center().y),
            vec2(MARK_SIDE, MARK_SIDE),
        ),
        problem,
    );
    ui.painter().galley(
        pos2(rect.left() + MARK_ADVANCE, rect.top()),
        galley,
        tokens.color.text,
    );
}

/// Wrapped copy, painted as text rather than laid out as a selectable label.
fn text_block(ui: &mut Ui, text: &str, font: egui::FontId, color: Color32) {
    let galley = ui
        .painter()
        .layout(text.to_owned(), font, color, ui.available_width().max(1.0));
    let (rect, response) = ui.allocate_exact_size(galley.size(), Sense::hover());
    ui.painter().galley(rect.min, galley, color);
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), text));
}

fn catalog_error(ui: &mut Ui, error: &str) -> Option<RecoveryIntent> {
    let tokens = Tokens::get(ui.ctx());
    ui.add_space(14.0);
    let width = ui.available_width() - 2.0 * (ROW_INSET + 2.0);
    let reload = page::inset_column(ui, ROW_INSET + 2.0, width, |ui| {
        marked_title(ui, Problem::Failure, "Checkpoints could not be read");
        ui.add_space(4.0);
        text_block(
            ui,
            &sentence_case(error),
            body_font(),
            tokens.color.text_dim,
        );
        ui.add_space(10.0);
        Button::new("Try again")
            .icon(crate::ui::icons::Icon::Refresh)
            .show(ui)
            .clicked()
    });
    ui.add_space(5.0);
    reload.then_some(RecoveryIntent::Reload)
}

/// What a checkpoint row shows besides the checkpoint.
#[derive(Clone, Copy)]
struct RowState {
    selected: bool,
    /// This row's checkpoint is being compared with the project.
    comparing: bool,
    /// This row's checkpoint is being put back in the project.
    restoring: bool,
    /// Checkpoint work is running, so the row's comparison waits.
    busy: bool,
}

fn checkpoint_row(
    ui: &mut Ui,
    checkpoint: &ProjectCheckpointSummary,
    state: RowState,
    now_unix_ms: Option<u64>,
    separated: bool,
) -> Option<RecoveryIntent> {
    let RowState {
        selected,
        comparing,
        restoring,
        busy,
    } = state;
    let tokens = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let stacked = width < CHECKPOINT_ROW_STACK_WIDTH;
    let control_height = prompt_control_height(ui);
    let created = checkpoint_created(checkpoint.created_unix_ms(), now_unix_ms);
    // The size goes to the hover: checkpoints of one project differ little,
    // and the time is what tells two of them apart. A checkpoint the reader
    // wrote a note on is named by it, so its reason moves here; a pinned one
    // says so first.
    let size = format_bytes(checkpoint.snapshot_byte_len());
    let pinned = if checkpoint.pinned() {
        "Pinned \u{b7} "
    } else {
        ""
    };
    let reason = if checkpoint.note().is_some() {
        format!("{} \u{b7} ", checkpoint.reason().label())
    } else {
        String::new()
    };
    let detail = format!(
        "{pinned}{reason}{created} \u{b7} revision {}",
        checkpoint.project_revision()
    );
    // Over its actions on a narrow card, a row's detail wraps rather than
    // being cut: there is no pointer to hover for the rest of it.
    let detail_galley = stacked.then(|| {
        ui.painter().layout(
            detail.clone(),
            detail_font(),
            tokens.color.text_faint,
            (width - 2.0 * ROW_INSET).max(1.0),
        )
    });
    let title_line = ui.fonts_mut(|fonts| fonts.row_height(&row_title_font()));
    let detail_height = detail_galley.as_ref().map_or_else(
        || ui.fonts_mut(|fonts| fonts.row_height(&detail_font())),
        |galley| galley.size().y,
    );
    let text_height = title_line + LINE_GAP + detail_height;
    let height = if stacked {
        ROW_PAD + text_height + ROW_PAD + control_height + ROW_PAD
    } else {
        checkpoint_row_least_height(ui.ctx()).max(control_height + 16.0)
    };
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::click());
    if selected {
        ui.painter().rect_filled(rect, 0.0, tokens.color.bg_active);
        ui.painter().rect_filled(
            Rect::from_min_size(rect.left_top(), vec2(2.0, rect.height())),
            0.0,
            tokens.color.accent,
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, tokens.color.bg_hover);
    }
    if separated {
        ui.painter().hline(
            (rect.left() + ROW_INSET)..=(rect.right() - ROW_INSET),
            rect.bottom() - 0.5,
            Stroke::new(1.0, tokens.color.border),
        );
    }

    let compare = Button::new(if comparing {
        "Comparing\u{2026}"
    } else {
        "Compare"
    })
    .enabled(!busy);
    // Restoring while a checkpoint is being written or compared would read
    // from a list the job is about to change.
    let restore = Button::new(if restoring {
        "Restoring\u{2026}"
    } else {
        "Restore\u{2026}"
    })
    .enabled(!busy);
    let more_label = format!("More actions for {}", checkpoint.title());
    let more = Button::new("More").menu().accessible_label(&more_label);
    let actions_width = compare.measured_width(ui)
        + 6.0
        + restore.measured_width(ui)
        + 6.0
        + more.measured_width(ui);
    let actions = if stacked {
        Rect::from_min_size(
            pos2(
                rect.left() + ROW_INSET,
                rect.bottom() - ROW_PAD - control_height,
            ),
            vec2(actions_width, control_height),
        )
    } else {
        Rect::from_min_max(
            pos2(
                rect.right() - ROW_INSET - actions_width,
                rect.center().y - control_height * 0.5,
            ),
            pos2(
                rect.right() - ROW_INSET,
                rect.center().y + control_height * 0.5,
            ),
        )
    };
    let text_right = if stacked {
        rect.right() - ROW_INSET
    } else {
        actions.left() - 12.0
    };
    let text_top = if stacked {
        rect.top() + ROW_PAD
    } else {
        rect.center().y - text_height * 0.5
    };
    let text_width = (text_right - rect.left() - ROW_INSET).max(1.0);
    paint_elided(
        ui,
        pos2(rect.left() + ROW_INSET, text_top),
        checkpoint.title(),
        row_title_font(),
        tokens.color.text,
        text_width,
    );
    let detail_top = pos2(rect.left() + ROW_INSET, text_top + title_line + LINE_GAP);
    match detail_galley {
        Some(galley) => {
            ui.painter()
                .galley(detail_top, galley, tokens.color.text_faint);
        }
        None => {
            paint_elided(
                ui,
                detail_top,
                &detail,
                detail_font(),
                tokens.color.text_faint,
                text_width,
            );
        }
    }

    let mut intent = None;
    // A child, not a scope: a scope moves the list's cursor to the bottom of
    // the buttons, which sit above the bottom of the row, so the next row
    // would start inside this one.
    let mut actions_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(actions)
            .layout(Layout::left_to_right(Align::Center)),
    );
    actions_ui.spacing_mut().item_spacing.x = 6.0;
    if compare
        .accessible_label(&format!(
            "Compare {} with the current project",
            checkpoint.title()
        ))
        .show(&mut actions_ui)
        .on_hover_text("List what differs between the current project and this checkpoint")
        .clicked()
    {
        intent = Some(RecoveryIntent::Compare(checkpoint.clone()));
    }
    if restore
        .accessible_label(&format!("Restore {} into the project", checkpoint.title()))
        .show(&mut actions_ui)
        .on_hover_text("Put this checkpoint back in the open project")
        .clicked()
    {
        intent = Some(RecoveryIntent::AskRestore(checkpoint.clone()));
    }
    let more = more.show(&mut actions_ui);
    egui::Popup::menu(&more).show(|ui| checkpoint_menu(ui, checkpoint, busy, &mut intent));
    crate::ui::widgets::context_menu_popup(&response)
        .show(|ui| checkpoint_menu(ui, checkpoint, busy, &mut intent));

    let row_label = format!("{}, {detail}, {size}", checkpoint.title());
    response.widget_info(|| {
        WidgetInfo::selected(
            WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            row_label.clone(),
        )
    });
    theme::paint_focus_ring(ui, &response, rect);
    let response = response.on_hover_text(format!(
        "{}\n{size}\nCheckpoint {}",
        checkpoint.reason().label(),
        checkpoint.checkpoint_id()
    ));
    if response.clicked() && intent.is_none() {
        intent = Some(RecoveryIntent::Select(
            checkpoint.checkpoint_id().to_string(),
        ));
    }
    intent
}

/// The rules automatic checkpoints are kept under. A browser writes no
/// autosave: its working session is what it keeps between visits.
#[cfg(not(target_arch = "wasm32"))]
const AUTOMATIC_SPACE_HELP: &str = "Automatic checkpoints are kept inside this space, the \
     oldest dropped first. Only the newest autosave is kept, and a save removes it. Set the \
     space in Preferences, under Projects and files.";
#[cfg(target_arch = "wasm32")]
const AUTOMATIC_SPACE_HELP: &str = "Automatic checkpoints are kept inside this space, the \
     oldest dropped first. Set the space in Preferences, under Projects and files.";

/// Where checkpoints live and the rules they are kept under.
fn storage_card(ui: &mut Ui, snapshot: &RecoverySnapshot) {
    page::card(ui, "Storage", "", None, |ui| {
        ui.add_space(3.0);
        property_row_path(ui, "Location", &snapshot.location);
        // Unknown until the listing is in; "0 B" would say there is none.
        let (space_used, budget) = if snapshot.loading {
            ("Listing\u{2026}".to_owned(), "Listing\u{2026}".to_owned())
        } else {
            (
                format_bytes(snapshot.stored_bytes),
                format!(
                    "{} of {}",
                    format_bytes(snapshot.budget_used),
                    budget_label(snapshot.budget_bytes)
                ),
            )
        };
        property_row(ui, "Space used", &space_used);
        property_row(ui, "Automatic space", &budget).on_hover_text(AUTOMATIC_SPACE_HELP);
        // Integrity is a row only when it is not in order. "All checkpoints
        // verified" and "Nothing to check" were the card reporting that
        // nothing had gone wrong, which is what the absence of the row says.
        if snapshot.error.is_some() {
            property_row_problem(ui, "Integrity", "Not checked", Some(Problem::Failure));
        } else if !snapshot.set_aside.is_empty() {
            property_row_problem(
                ui,
                "Integrity",
                &format!("{} damaged, set aside", snapshot.set_aside.len()),
                Some(Problem::Warning),
            );
        }
        // The rule they are kept by, in a sentence that wraps: as two rows
        // its values were cut short on a phone.
        let tokens = Tokens::get(ui.ctx());
        ui.add_space(8.0);
        let width = ui.available_width() - 2.0 * ROW_INSET;
        page::inset_column(ui, ROW_INSET, width, |ui| {
            text_block(ui, RETENTION_NOTE, body_font(), tokens.color.text_dim);
        });
        ui.add_space(12.0);
    });
}

/// How long checkpoints are kept, and which RSpice takes by itself.
const RETENTION_NOTE: &str = "Manual and pinned checkpoints are kept until you delete them. \
     RSpice also takes one before each save that changes the project, a restore, a technology \
     change and a library deletion; those are kept in the automatic space, oldest dropped first.";

/// Artifacts that failed verification. They are never offered for restore
/// and retention never removes them; each is deleted here, after a question
/// that names it. The card is drawn only when there is something in it.
fn set_aside_card(ui: &mut Ui, snapshot: &RecoverySnapshot) -> Option<RecoveryIntent> {
    if snapshot.set_aside.is_empty() {
        return None;
    }
    let tokens = Tokens::get(ui.ctx());
    ui.add_space(CARD_GAP);
    page::card(
        ui,
        "Set aside",
        &snapshot.set_aside.len().to_string(),
        Some(Problem::Warning),
        |ui| {
            ui.add_space(10.0);
            let width = ui.available_width() - 2.0 * ROW_INSET;
            let height = prompt_control_height(ui);
            page::inset_column(ui, ROW_INSET, width, |ui| {
                text_block(
                    ui,
                    "These files failed verification, so they are never offered for restore.",
                    body_font(),
                    tokens.color.text_dim,
                );
                let mut intent = None;
                for artifact in &snapshot.set_aside {
                    let label = artifact.label();
                    ui.add_space(10.0);
                    // The file's whole name, wrapped: the one thing that
                    // tells two damaged files apart is never cut.
                    text_block(
                        ui,
                        label,
                        theme::mono(tokens::FS_0, FontWeight::Medium),
                        tokens.color.text,
                    );
                    text_block(
                        ui,
                        &sentence_case(artifact.reason()),
                        body_font(),
                        tokens.color.text_dim,
                    );
                    ui.add_space(6.0);
                    let deletion = CheckpointDeletion::SetAside(label.to_owned());
                    if snapshot.delete_review.as_ref() == Some(&deletion) {
                        let answer = delete_prompt(
                            ui,
                            &format!(
                                "Delete the damaged checkpoint {label}? It cannot be brought back."
                            ),
                            &format!("Delete {label}"),
                            snapshot.busy,
                        );
                        keep_first(
                            &mut intent,
                            answer.map(|delete| {
                                if delete {
                                    RecoveryIntent::DeleteSetAside(artifact.clone())
                                } else {
                                    RecoveryIntent::CancelDelete
                                }
                            }),
                        );
                    } else if Button::new("Delete\u{2026}")
                        .enabled(!snapshot.busy)
                        .min_height(height)
                        .accessible_label(&format!("Delete {label}"))
                        .show(ui)
                        .clicked()
                    {
                        keep_first(&mut intent, Some(RecoveryIntent::AskDelete(deletion)));
                    }
                }
                intent
            })
        },
    )
}

fn sentence_case(text: &str) -> String {
    let mut characters = text.chars();
    match characters.next() {
        Some(first) => first.to_uppercase().chain(characters).collect(),
        None => String::new(),
    }
}

/// When a checkpoint was made, written in the row itself: two checkpoints
/// made minutes apart are told apart by their time, and a tablet has no hover
/// to read it from. The time is UTC, the spelling every stored timestamp in
/// this product is written out in; the age follows it.
fn checkpoint_created(created_unix_ms: u64, now_unix_ms: Option<u64>) -> String {
    if created_unix_ms == 0 {
        return "time unavailable".to_owned();
    }
    format!(
        "{} \u{b7} {}",
        crate::time_compat::utc_stamp(created_unix_ms),
        checkpoint_age(created_unix_ms, now_unix_ms)
    )
}

/// The open project's catalog, kept for the project it was read for: listed
/// once on a background thread, and again only when it was invalidated.
pub(super) fn ensure_project_recovery_catalog(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) {
    let project_id = cx.model.workspace.content.project.id().to_string();
    if cx.state.project_checkpoint_recovery.project_id.as_deref() != Some(project_id.as_str()) {
        let recovery = &mut cx.state.project_checkpoint_recovery;
        #[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
        let job_hold = recovery.job_hold.take();
        *recovery = Default::default();
        recovery.project_id = Some(project_id.clone());
        #[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
        {
            recovery.job_hold = job_hold;
        }
    }
    // A save moves the checkpoints (a first save beside the file), adds one
    // and lets retention run: the catalog read before it is listed again.
    let source = catalog_source(cx);
    if let Some(listed_for) = cx.state.project_checkpoint_recovery.listed_for
        && listed_for != source
    {
        let recovery = &mut cx.state.project_checkpoint_recovery;
        recovery.listed_for = None;
        recovery.location = None;
        recovery.invalidate();
    }
    if cx.state.project_checkpoint_recovery.location.is_none() {
        cx.state.project_checkpoint_recovery.location = Some(storage_location(cx.model));
    }
    #[cfg(not(target_arch = "wasm32"))]
    take_checkpoint_job_answer(cx);
    if cx.state.project_checkpoint_recovery.initialized {
        return;
    }
    cx.state.project_checkpoint_recovery.listed_for = Some(source);

    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::workbench::app::dialogs::project::{
            CheckpointJob, CheckpointJobAnswer, CheckpointJobKind,
        };
        if cx.state.project_checkpoint_recovery.job.is_some() {
            return;
        }
        match crate::workbench::lifecycle::project_checkpoint::listing(cx.model) {
            Ok(listing) => {
                let recovery = &mut cx.state.project_checkpoint_recovery;
                recovery.job = Some(CheckpointJob::start(
                    ctx,
                    CheckpointJobKind::List,
                    recovery,
                    move || CheckpointJobAnswer::Listed(listing.list()),
                ));
            }
            Err(error) => {
                let recovery = &mut cx.state.project_checkpoint_recovery;
                recovery.initialized = true;
                recovery.error = Some(error);
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        if cx.state.project_checkpoint_recovery.loading {
            return;
        }
        cx.state.project_checkpoint_recovery.loading = true;
        let queued_project_id = project_id;
        let repaint = ctx.clone();
        crate::workbench::lifecycle::project_checkpoint::start_list(cx.model, move |result| {
            BROWSER_RECOVERY_CATALOG_COMPLETIONS.with(|queue| {
                queue
                    .borrow_mut()
                    .push_back(BrowserRecoveryCatalogCompletion {
                        project_id: queued_project_id,
                        result,
                    });
            });
            repaint.request_repaint();
        });
    }
}

fn checkpoint_age(created_unix_ms: u64, now_unix_ms: Option<u64>) -> String {
    // A checkpoint stamped after the clock's present says the clock moved,
    // not how old the checkpoint is.
    let Some(elapsed) = now_unix_ms
        .filter(|_| created_unix_ms != 0)
        .and_then(|now| now.checked_sub(created_unix_ms))
    else {
        return "time unavailable".to_owned();
    };
    crate::time_compat::relative_age(std::time::Duration::from_millis(elapsed))
}

/// Answer a compare on the status bar: the reader asked and is looking. Only
/// a compare that could not be made is toasted.
fn report_comparison(
    cx: &mut Cx<'_, ProjectFeature>,
    checkpoint: &crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
    compared: Result<CheckpointComparison, String>,
) {
    let recovery = &mut cx.state.project_checkpoint_recovery;
    match compared {
        Ok(found) if found.is_empty() => {
            recovery.comparison = None;
            cx.wb
                .prompt("The current project is identical to this checkpoint.");
        }
        Ok(found) => {
            cx.wb.prompt(format!(
                "The current project differs from \u{201c}{}\u{201d} in {}.",
                checkpoint.title(),
                if found.len() == 1 {
                    "1 document".to_owned()
                } else {
                    format!("{} documents", found.len())
                }
            ));
            recovery.comparison = Some((checkpoint.checkpoint_id(), found));
        }
        Err(error) => {
            recovery.comparison = None;
            cx.model.report_failure("Could not compare", error);
        }
    }
}

/// Compare the project as it is now with a checkpoint, document by document.
/// The project is read off the model here; reading the checkpoint back and
/// fingerprinting both runs on a background thread.
#[cfg(not(target_arch = "wasm32"))]
fn compare_project_checkpoint(
    ctx: &Context,
    cx: &mut Cx<'_, ProjectFeature>,
    checkpoint: ProjectCheckpointSummary,
) {
    use crate::workbench::app::dialogs::project::{
        CheckpointJob, CheckpointJobAnswer, CheckpointJobKind,
    };
    if cx.state.project_checkpoint_recovery.job.is_some() {
        return;
    }
    let current = match crate::workbench::lifecycle::project_checkpoint::capture_current_state(
        &lifecycle_view(cx.model, cx.platform, cx.wb, &cx.peers),
    ) {
        Ok(current) => current,
        Err(error) => {
            report_comparison(cx, &checkpoint, Err(error));
            return;
        }
    };
    let recovery = &mut cx.state.project_checkpoint_recovery;
    let compared = checkpoint.clone();
    recovery.job = Some(CheckpointJob::start(
        ctx,
        CheckpointJobKind::Compare(Box::new(checkpoint)),
        recovery,
        move || CheckpointJobAnswer::Compared(current.differences(&compared)),
    ));
}

#[cfg(target_arch = "wasm32")]
fn compare_project_checkpoint(
    ctx: &Context,
    cx: &mut Cx<'_, ProjectFeature>,
    checkpoint: ProjectCheckpointSummary,
) {
    let current = match crate::workbench::lifecycle::project_checkpoint::capture_current_state(
        &lifecycle_view(cx.model, cx.platform, cx.wb, &cx.peers),
    ) {
        Ok(current) => current,
        Err(error) => {
            report_comparison(cx, &checkpoint, Err(error));
            return;
        }
    };
    let project_id = cx.model.workspace.content.project.id().to_string();
    let repaint = ctx.clone();
    let compared = checkpoint.clone();
    crate::workbench::lifecycle::project_checkpoint::start_load(checkpoint, move |loaded| {
        let found = loaded.and_then(|project| current.differences_from(&project));
        BROWSER_CHECKPOINT_READS.with(|queue| {
            queue.borrow_mut().push_back(BrowserCheckpointRead {
                project_id,
                checkpoint: compared,
                read: BrowserRead::Compared(found),
            });
        });
        repaint.request_repaint();
    });
}

/// Put a checkpoint back in the open project: a "Before restore" checkpoint
/// of the project as it is now is written first, then the checkpoint is read
/// back, both on a background thread, and the shell installs what was read.
/// No "Before restore" checkpoint, no restore.
#[cfg(not(target_arch = "wasm32"))]
fn restore_project_checkpoint(
    ctx: &Context,
    cx: &mut Cx<'_, ProjectFeature>,
    checkpoint: ProjectCheckpointSummary,
) {
    use crate::workbench::app::dialogs::project::{
        CHECKPOINT_NOT_RESTORED, CheckpointJob, CheckpointJobAnswer, CheckpointJobKind,
    };
    use crate::workbench::lifecycle::project_checkpoint::{ProjectCheckpointReason, capture, load};
    if cx.state.project_checkpoint_recovery.job.is_some() {
        return;
    }
    if let Some(refusal) = restore_refusal(cx) {
        cx.model.report_failure(CHECKPOINT_NOT_RESTORED, refusal);
        return;
    }
    let before = match capture(
        &lifecycle_view(cx.model, cx.platform, cx.wb, &cx.peers),
        ProjectCheckpointReason::Restore,
    ) {
        Ok(before) => before,
        Err(error) => {
            cx.model.report_failure(
                CHECKPOINT_NOT_RESTORED,
                format!("the project as it is now could not be kept first: {error}"),
            );
            return;
        }
    };
    let recovery = &mut cx.state.project_checkpoint_recovery;
    recovery.comparison = None;
    let restored = checkpoint.clone();
    recovery.job = Some(CheckpointJob::start(
        ctx,
        CheckpointJobKind::Restore(Box::new(checkpoint)),
        recovery,
        move || {
            let before = before.publish();
            let project = before.is_ok().then(|| load(&restored).map(Box::new));
            CheckpointJobAnswer::Restored {
                before: Box::new(before),
                project,
            }
        },
    ));
}

#[cfg(target_arch = "wasm32")]
fn restore_project_checkpoint(
    ctx: &Context,
    cx: &mut Cx<'_, ProjectFeature>,
    checkpoint: ProjectCheckpointSummary,
) {
    use crate::workbench::app::dialogs::project::CHECKPOINT_NOT_RESTORED;
    use crate::workbench::lifecycle::project_checkpoint::{
        ProjectCheckpointReason, start_create, start_load,
    };
    if browser_restore_pending().is_some() {
        return;
    }
    if let Some(refusal) = restore_refusal(cx) {
        cx.model.report_failure(CHECKPOINT_NOT_RESTORED, refusal);
        return;
    }
    cx.state.project_checkpoint_recovery.comparison = None;
    let project_id = cx.model.workspace.content.project.id().to_string();
    let repaint = ctx.clone();
    BROWSER_RESTORE_PENDING.with(|pending| pending.set(Some(checkpoint.checkpoint_id())));
    let restored = checkpoint.clone();
    let started = start_create(
        &lifecycle_view(cx.model, cx.platform, cx.wb, &cx.peers),
        ProjectCheckpointReason::Restore,
        move |before| {
            let checkpoint = restored.clone();
            let finish = move |read: BrowserRead| {
                BROWSER_CHECKPOINT_READS.with(|queue| {
                    queue.borrow_mut().push_back(BrowserCheckpointRead {
                        project_id,
                        checkpoint: restored,
                        read,
                    });
                });
                repaint.request_repaint();
            };
            match before {
                Ok(before) => {
                    start_load(checkpoint, move |project| {
                        finish(BrowserRead::Restored(Ok((before, project))));
                    });
                }
                Err(error) => finish(BrowserRead::Restored(Err(error))),
            }
        },
    );
    if let Err(error) = started {
        BROWSER_RESTORE_PENDING.with(|pending| pending.set(None));
        cx.model.report_failure(
            CHECKPOINT_NOT_RESTORED,
            format!("the project as it is now could not be kept first: {error}"),
        );
    }
}

/// Why the open project cannot take a checkpoint back now, if it cannot.
fn restore_refusal(cx: &Cx<'_, ProjectFeature>) -> Option<&'static str> {
    if cx.model.safe_mode.project_read_only() {
        Some("the project is open read-only")
    } else if cx.model.simulation.execution.has_active_execution() {
        Some("a simulation is running")
    } else {
        None
    }
}

/// Hand a checkpoint read back for a restore to the shell, which puts it in
/// the open project, and list the checkpoints again with the "Before restore"
/// one among them.
fn hand_over_restore(
    cx: &mut Cx<'_, ProjectFeature>,
    checkpoint: ProjectCheckpointSummary,
    before: Result<ProjectCheckpointSummary, String>,
    project: Option<Result<Box<crate::io::ProjectSnapshot>, String>>,
) {
    use crate::workbench::app::dialogs::project::{CHECKPOINT_NOT_RESTORED, RestoredCheckpoint};
    let recovery = &mut cx.state.project_checkpoint_recovery;
    recovery.initialized = false;
    match (before, project) {
        (Err(error), _) => cx.model.report_failure(
            CHECKPOINT_NOT_RESTORED,
            format!("the project as it is now could not be kept first: {error}"),
        ),
        (Ok(_), None) => {}
        (Ok(_), Some(Err(error))) => cx.model.report_failure(CHECKPOINT_NOT_RESTORED, error),
        (Ok(before), Some(Ok(project))) => {
            recovery.restored = Some(RestoredCheckpoint {
                checkpoint,
                project: *project,
                before,
            });
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn manual_checkpoint_pending() -> bool {
    BROWSER_MANUAL_CHECKPOINT_PENDING.with(std::cell::Cell::get)
}

/// Create a checkpoint the reader asked for. The project is read off the
/// model here; writing, verifying and listing run on a background thread.
/// The Console line is its record; a failure is a notice.
#[cfg(not(target_arch = "wasm32"))]
fn create_manual_checkpoint(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) {
    use crate::workbench::app::dialogs::project::{
        CheckpointJob, CheckpointJobAnswer, CheckpointJobKind,
    };
    if cx.state.project_checkpoint_recovery.job.is_some() {
        return;
    }
    let capture = crate::workbench::lifecycle::project_checkpoint::capture(
        &lifecycle_view(cx.model, cx.platform, cx.wb, &cx.peers),
        crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointReason::Manual,
    );
    match capture {
        Ok(capture) => {
            let recovery = &mut cx.state.project_checkpoint_recovery;
            recovery.job = Some(CheckpointJob::start(
                ctx,
                CheckpointJobKind::Create,
                recovery,
                move || CheckpointJobAnswer::Created(Box::new(capture.publish())),
            ));
        }
        Err(error) => cx.model.report_failure(CHECKPOINT_NOT_CREATED, error),
    }
}

/// Take a checkpoint job's answer, if it is in, for the project it was
/// started for: the catalog it read, the checkpoint it wrote, or the
/// comparison it made.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn take_checkpoint_job_answer(cx: &mut Cx<'_, ProjectFeature>) {
    use crate::workbench::app::dialogs::project::{CheckpointJobAnswer, CheckpointJobKind};
    let recovery = &mut cx.state.project_checkpoint_recovery;
    let project_id = cx.model.workspace.content.project.id().to_string();
    if recovery.project_id.as_deref() != Some(project_id.as_str()) {
        // A job for a project that is no longer open is dropped with its
        // catalog when the page next draws.
        return;
    }
    let Some(job) = recovery.job.as_ref() else {
        return;
    };
    let Some(answer) = job.take_answer() else {
        return;
    };
    let Some(job) = recovery.job.take() else {
        return;
    };
    match answer {
        CheckpointJobAnswer::Listed(listed) => install_catalog(recovery, listed),
        CheckpointJobAnswer::Created(created) => match *created {
            Ok(created) => {
                // Asked to list again while writing: the listing the
                // creation made may predate what changed, so another
                // follows.
                let relist = !recovery.initialized;
                install_catalog(recovery, created.catalog);
                if relist {
                    recovery.initialized = false;
                }
                record_manual_checkpoint(cx, &created.summary);
                if let Some(warning) = created.retention_warning {
                    cx.model
                        .record(LogSeverity::Warning, LogSource::Studio, warning);
                }
            }
            Err(error) => cx.model.report_failure(CHECKPOINT_NOT_CREATED, error),
        },
        CheckpointJobAnswer::Compared(compared) => {
            if let CheckpointJobKind::Compare(checkpoint) = job.kind {
                report_comparison(cx, &checkpoint, compared);
            }
        }
        CheckpointJobAnswer::Restored { before, project } => {
            if let CheckpointJobKind::Restore(checkpoint) = job.kind {
                let before = (*before).map(|created| {
                    if let Some(warning) = created.retention_warning {
                        cx.model
                            .record(LogSeverity::Warning, LogSource::Studio, warning);
                    }
                    created.summary
                });
                hand_over_restore(cx, *checkpoint, before, project);
            }
        }
        CheckpointJobAnswer::Changed { outcome, listed } => {
            let relist = !recovery.initialized;
            install_catalog(recovery, listed);
            if relist {
                recovery.initialized = false;
            }
            report_change(cx, outcome);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn install_catalog(
    recovery: &mut crate::workbench::app::dialogs::project::ProjectCheckpointRecoveryState,
    listed: Result<
        crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointCatalog,
        String,
    >,
) {
    recovery.initialized = true;
    match listed {
        Ok(catalog) => {
            recovery.checkpoints = catalog.checkpoints;
            recovery.quarantined = catalog.quarantined;
            recovery.error = None;
        }
        Err(error) => recovery.error = Some(error),
    }
}

fn record_manual_checkpoint(
    cx: &mut Cx<'_, ProjectFeature>,
    checkpoint: &crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
) {
    cx.model.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!(
            "Created project checkpoint {} at revision {}.",
            short_identity(&checkpoint.checkpoint_id().to_string()),
            checkpoint.project_revision()
        ),
    );
}

#[cfg(target_arch = "wasm32")]
fn create_manual_checkpoint(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) {
    let project_id = cx.model.workspace.content.project.id().to_string();
    let repaint = ctx.clone();
    BROWSER_MANUAL_CHECKPOINT_PENDING.with(|pending| pending.set(true));
    let started = crate::workbench::lifecycle::project_checkpoint::start_create(
        &lifecycle_view(cx.model, cx.platform, cx.wb, &cx.peers),
        crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointReason::Manual,
        move |result| {
            BROWSER_MANUAL_CHECKPOINT_COMPLETIONS.with(|queue| {
                queue
                    .borrow_mut()
                    .push_back(BrowserManualCheckpointCompletion { project_id, result });
            });
            repaint.request_repaint();
        },
    );
    if let Err(error) = started {
        BROWSER_MANUAL_CHECKPOINT_PENDING.with(|pending| pending.set(false));
        cx.model.report_failure(CHECKPOINT_NOT_CREATED, error);
    }
}

#[cfg(target_arch = "wasm32")]
fn poll_browser_manual_checkpoint(cx: &mut Cx<'_, ProjectFeature>) {
    let current_project_id = cx.model.workspace.content.project.id().to_string();
    let completions = BROWSER_MANUAL_CHECKPOINT_COMPLETIONS
        .with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    if completions.is_empty() {
        return;
    }
    BROWSER_MANUAL_CHECKPOINT_PENDING.with(|pending| pending.set(false));
    for completion in completions {
        if completion.project_id != current_project_id {
            continue;
        }
        match completion.result {
            Ok(checkpoint) => {
                cx.state.project_checkpoint_recovery.initialized = false;
                record_manual_checkpoint(cx, &checkpoint);
            }
            Err(error) => cx.model.report_failure(CHECKPOINT_NOT_CREATED, error),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn export_project_checkpoint_copy(
    _: &Context,
    cx: &mut Cx<'_, ProjectFeature>,
    checkpoint: crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
) {
    let filename =
        crate::workbench::lifecycle::project_checkpoint::recovery_copy_file_name(&checkpoint);
    let Some(destination) = rfd::FileDialog::new()
        .add_filter("RSpice Project", &["rspiceproj"])
        .set_file_name(&filename)
        .save_file()
    else {
        return;
    };
    match crate::workbench::lifecycle::project_checkpoint::publish_recovery_copy(
        &checkpoint,
        &destination,
    ) {
        Ok(()) => {
            cx.model.record(
                LogSeverity::Info,
                LogSource::Studio,
                format!(
                    "Saved checkpoint \u{201c}{}\u{201d} as a new project: {}.",
                    checkpoint.title(),
                    destination.display()
                ),
            );
            cx.wb.prompt(format!(
                "Saved \u{201c}{}\u{201d} as a new project.",
                checkpoint.title()
            ));
            cx.state.project_checkpoint_recovery.saved_copy =
                Some((checkpoint.checkpoint_id(), destination));
        }
        Err(error) => cx.model.report_failure("Not saved as a new project", error),
    }
}

/// A checkpoint the browser read back for a comparison or a restore, after
/// the frame that asked for it.
#[cfg(target_arch = "wasm32")]
struct BrowserCheckpointRead {
    project_id: String,
    checkpoint: ProjectCheckpointSummary,
    read: BrowserRead,
}

#[cfg(target_arch = "wasm32")]
enum BrowserRead {
    Compared(Result<CheckpointComparison, String>),
    /// The "Before restore" checkpoint, then the checkpoint read back.
    Restored(
        Result<
            (
                ProjectCheckpointSummary,
                Result<crate::io::ProjectSnapshot, String>,
            ),
            String,
        >,
    ),
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_CHECKPOINT_READS: std::cell::RefCell<std::collections::VecDeque<BrowserCheckpointRead>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
    static BROWSER_RESTORE_PENDING: std::cell::Cell<Option<uuid::Uuid>> =
        const { std::cell::Cell::new(None) };
}

#[cfg(target_arch = "wasm32")]
fn browser_restore_pending() -> Option<uuid::Uuid> {
    BROWSER_RESTORE_PENDING.with(std::cell::Cell::get)
}

/// Take the comparisons and restores the browser finished reading, for the
/// project still open.
#[cfg(target_arch = "wasm32")]
pub(super) fn poll_browser_checkpoint_reads(cx: &mut Cx<'_, ProjectFeature>) {
    let reads =
        BROWSER_CHECKPOINT_READS.with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    let current_project_id = cx.model.workspace.content.project.id().to_string();
    for read in reads {
        if matches!(read.read, BrowserRead::Restored(_)) {
            BROWSER_RESTORE_PENDING.with(|pending| pending.set(None));
        }
        if read.project_id != current_project_id {
            continue;
        }
        match read.read {
            BrowserRead::Compared(found) => report_comparison(cx, &read.checkpoint, found),
            BrowserRead::Restored(Err(error)) => {
                hand_over_restore(cx, read.checkpoint, Err(error), None);
            }
            BrowserRead::Restored(Ok((before, project))) => {
                hand_over_restore(cx, read.checkpoint, Ok(before), Some(project.map(Box::new)))
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(super) fn export_project_checkpoint_copy(
    ctx: &Context,
    cx: &mut Cx<'_, ProjectFeature>,
    checkpoint: crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
) {
    let filename =
        crate::workbench::lifecycle::project_checkpoint::recovery_copy_file_name(&checkpoint);
    let project_id = cx.model.workspace.content.project.id().to_string();
    let queued_filename = filename.clone();
    let repaint = ctx.clone();
    crate::workbench::lifecycle::project_checkpoint::start_recovery_copy_bytes(
        checkpoint,
        std::path::PathBuf::from(&filename),
        move |result| {
            BROWSER_RECOVERY_COPY_COMPLETIONS.with(|queue| {
                queue.borrow_mut().push_back(BrowserRecoveryCopyCompletion {
                    project_id,
                    filename: queued_filename,
                    result,
                });
            });
            repaint.request_repaint();
        },
    );
}
