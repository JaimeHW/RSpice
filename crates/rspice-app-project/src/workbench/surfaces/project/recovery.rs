//! The project's checkpoints: listed, created, compared, restored, noted,
//! pinned, deleted and saved as new projects, each on a background thread
//! or in the browser's store, for the History page that lists them.
//!
//! A checkpoint is a complete, integrity-checked copy of the project. Comparing
//! or restoring one never mutates or consumes it, and restoring one first keeps
//! the project as it was as a checkpoint of its own, so going back cannot cost
//! the reader the state they went back from.

pub use super::*;
use crate::workbench::app::dialogs::project::{CheckpointComparison, CheckpointDeletion};
use crate::workbench::lifecycle::project_checkpoint::{
    CheckpointEdit, ProjectCheckpointQuarantine, ProjectCheckpointSummary, normalized_note,
    technology_restore,
};
use crate::workbench::project::lifecycle_view;

/// The one word for a checkpoint store that cannot be read, wherever its
/// state is a value: the inspector's Integrity row.
pub const STORE_UNREADABLE: &str = "Unreadable";

/// What checkpoint work is running, read before the page draws, so an action
/// taken on this frame changes the next one rather than half of this one.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct CheckpointWork {
    /// The catalog is being listed for the first time.
    pub(super) loading: bool,
    pub(super) creating: bool,
    /// The checkpoint a running comparison is comparing with.
    pub(super) comparing: Option<uuid::Uuid>,
    /// The checkpoint a running restore is putting back.
    pub(super) restoring: Option<uuid::Uuid>,
    /// Checkpoint work is running; another waits for it.
    pub(super) busy: bool,
}

impl CheckpointWork {
    pub(super) fn read(cx: &Cx<'_, ProjectFeature>) -> Self {
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
            loading,
            creating,
            comparing,
            restoring,
            busy,
        }
    }
}

/// Take what the browser finished for a checkpoint the reader created, and
/// list the open project's checkpoints if they are not listed: what the
/// History page does before it reads them.
pub(super) fn refresh(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) {
    #[cfg(target_arch = "wasm32")]
    poll_browser_manual_checkpoint(cx);
    ensure_project_recovery_catalog(ctx, cx);
}

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

#[cfg(not(target_arch = "wasm32"))]
fn storage_location(model: &crate::workbench::app_state::Model) -> String {
    crate::workbench::lifecycle::project_checkpoint::storage_location(model)
        .map_or_else(|error| error, |path| path.display().to_string())
}

#[cfg(target_arch = "wasm32")]
fn storage_location(_model: &crate::workbench::app_state::Model) -> String {
    "This browser's site storage".to_owned()
}

/// What the reader asked of a checkpoint, or of the page's checkpoints.
pub(super) enum RecoveryIntent {
    Create,
    Reload,
    Compare(ProjectCheckpointSummary),
    /// Put away what the last comparison found.
    CloseComparison,
    /// Ask whether to restore it, in place.
    AskRestore(ProjectCheckpointSummary),
    CancelRestore,
    Restore(ProjectCheckpointSummary),
    /// Show the Dependencies page, where a technology is installed and
    /// changed.
    OpenDependencies,
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
    pub(super) fn execute(self, ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) {
        match self {
            Self::Create => create_manual_checkpoint(ctx, cx),
            Self::Reload => cx.state.project_checkpoint_recovery.invalidate(),
            Self::Compare(checkpoint) => {
                cx.state.project_history_selection = Some(checkpoint.checkpoint_id().to_string());
                compare_project_checkpoint(ctx, cx, checkpoint);
            }
            Self::CloseComparison => cx.state.project_checkpoint_recovery.comparison = None,
            Self::AskRestore(checkpoint) => {
                let recovery = &mut cx.state.project_checkpoint_recovery;
                recovery.note_draft = None;
                recovery.delete_review = None;
                recovery.restore_review = Some(checkpoint.checkpoint_id());
                cx.state.project_history_selection = Some(checkpoint.checkpoint_id().to_string());
            }
            Self::CancelRestore => cx.state.project_checkpoint_recovery.restore_review = None,
            Self::Restore(checkpoint) => {
                cx.state.project_checkpoint_recovery.restore_review = None;
                if let Some(refusal) = technology_restore(cx.model, checkpoint.technology())
                    .refusal(checkpoint.title())
                {
                    cx.model.report_failure(
                        crate::workbench::app::dialogs::project::CHECKPOINT_NOT_RESTORED,
                        refusal,
                    );
                    return;
                }
                restore_project_checkpoint(ctx, cx, checkpoint);
            }
            Self::OpenDependencies => {
                cx.state.project_checkpoint_recovery.restore_review = None;
                cx.state.project_page =
                    crate::workbench::project::workspace_state::ProjectPage::Dependencies;
            }
            Self::SaveAsNewProject(checkpoint) => {
                cx.state.project_history_selection = Some(checkpoint.checkpoint_id().to_string());
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
                cx.state.project_history_selection = Some(checkpoint.checkpoint_id().to_string());
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
                    cx.state.project_history_selection = Some(id.to_string());
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
                crate::time_compat::local_stamp(checkpoint.created_unix_ms())
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

/// What is wrong when the store that keeps the project's checkpoints cannot
/// be read, and what to do. What the system said goes to the Console.
#[cfg(not(target_arch = "wasm32"))]
pub(super) const CATALOG_UNREADABLE: &str = "RSpice could not open the folder that keeps this project's \
     checkpoints. Check that you are allowed to open it, then try again. The Console has what \
     the system said.";
#[cfg(target_arch = "wasm32")]
pub(super) const CATALOG_UNREADABLE: &str = "The browser did not let RSpice read the checkpoints it keeps \
     for this project. Check that this site may store data, then try again. The Console has \
     what the browser said.";

/// When a checkpoint was made and how long ago, as its record in the
/// inspector states it.
pub(crate) fn checkpoint_created(created_unix_ms: u64, now_unix_ms: Option<u64>) -> String {
    if created_unix_ms == 0 {
        return "time unavailable".to_owned();
    }
    format!(
        "{} \u{b7} {}",
        crate::time_compat::local_stamp(created_unix_ms),
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
        **recovery = Default::default();
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
                let error_before = recovery.error.take();
                recovery.initialized = true;
                recovery.error = Some(error);
                record_catalog_failure(cx, error_before.as_deref());
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
pub(super) fn restore_refusal(cx: &Cx<'_, ProjectFeature>) -> Option<&'static str> {
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
            // The person at this application, whom a technology the
            // checkpoint brings back is recorded under.
            let actor = cx.peers.cloud_session().review_actor(cx.peers.cloud());
            cx.state.project_checkpoint_recovery.restored = Some(RestoredCheckpoint {
                checkpoint,
                project: *project,
                before,
                actor,
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
    let damaged_before = damaged_labels(recovery);
    let error_before = recovery.error.clone();
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
    record_found_damage(cx, &damaged_before);
    record_catalog_failure(cx, error_before.as_deref());
}

/// The damaged checkpoints the page lists, by label.
pub(super) fn damaged_labels(
    recovery: &crate::workbench::app::dialogs::project::ProjectCheckpointRecoveryState,
) -> Vec<String> {
    recovery
        .quarantined
        .iter()
        .map(|artifact| artifact.label().to_owned())
        .collect()
}

/// Writes each damaged checkpoint a listing found that the listing before it
/// did not to the Console, with the store's own account of it: the page says
/// what is wrong in the reader's words, and the Console keeps what was found.
pub(super) fn record_found_damage(cx: &mut Cx<'_, ProjectFeature>, before: &[String]) {
    let found = cx
        .state
        .project_checkpoint_recovery
        .quarantined
        .iter()
        .filter(|artifact| !before.iter().any(|label| label == artifact.label()))
        .map(|artifact| {
            format!(
                "Damaged checkpoint {}: {} ({})",
                artifact.label(),
                artifact.problem(),
                artifact.detail()
            )
        })
        .collect::<Vec<_>>();
    for line in found {
        cx.model
            .record(LogSeverity::Warning, LogSource::Studio, line);
    }
}

/// Writes what the system said when the project's checkpoints could not be
/// listed to the Console, once for each failure the listing before it did
/// not have: the page says what is wrong in the reader's words.
pub(super) fn record_catalog_failure(cx: &mut Cx<'_, ProjectFeature>, before: Option<&str>) {
    let Some(error) = cx.state.project_checkpoint_recovery.error.as_deref() else {
        return;
    };
    if before == Some(error) {
        return;
    }
    let line = format!("Checkpoints could not be read: {error}");
    cx.model
        .record(LogSeverity::Warning, LogSource::Studio, line);
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
