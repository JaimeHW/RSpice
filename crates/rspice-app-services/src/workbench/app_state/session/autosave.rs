//! Autosave: interrupted-session recovery that never touches the reader's file.
//!
//! A project with a file and unsaved work is written at the autosave interval
//! as an Autosave checkpoint beside it, off the frame. Opening it, or starting
//! with it, looks for an autosave another session left that is newer than the
//! file and differs from it, and asks once what to do with it.
//!
//! The browser does the same with a project saved to a file or to the
//! browser's own storage: its autosave is kept in the browser's durable
//! records, encoded and written in a browser task of its own, and offered
//! when it was written over the save the project's file still holds.
//!
//! A standalone schematic with a path is written the same way to a
//! `.autosave` sibling. A clean save retires only a checkpoint demonstrably
//! owned by this process. Opening a file offers an eligible bound checkpoint
//! before loading either snapshot.

#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};

use egui::Context;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::{Dialog, DialogChoice, DialogSize};

use crate::diagnostics::{LogSeverity, LogSource};
#[cfg(not(target_arch = "wasm32"))]
use crate::state::SchematicState;
use crate::time_compat::Instant;
use crate::workbench::app_state::session::browser_session;
use crate::workbench::lifecycle::durable_records::{self, BrowserRecords, DurableRecords};
use crate::workbench::lifecycle::project_checkpoint::{
    self, CheckpointEdit, DocumentDifference, ProjectCheckpointReason, ProjectCheckpointSummary,
};
use crate::workbench::lifecycle::project_lifecycle::{
    self, LifecycleParts, LifecycleSourceMut, PlatformState,
};
use rspice_app_state::workbench::app_state::model::Model;

/// Write a checkpoint when the interval has elapsed on a dirty,
/// path-bearing schematic. Called once per frame; cheap when idle. `last` is
/// when the timer was armed or the last checkpoint written.
#[cfg(not(target_arch = "wasm32"))]
pub fn autosave_tick(
    platform: &mut PlatformState,
    model: &mut Model,
    last: &mut Option<std::time::Instant>,
    ctx: &Context,
) {
    let minutes = platform.autosave_minutes;
    if minutes == 0 || !model.schematic.session.is_dirty || model.schematic.session.read_only {
        *last = None;
        return;
    }
    let Some(path) = model.schematic.session.current_file.clone() else {
        // Nothing to anchor a checkpoint to — save once and autosave
        // takes over.
        return;
    };

    let now = std::time::Instant::now();
    let interval = std::time::Duration::from_secs(u64::from(minutes) * 60);
    let Some(armed) = *last else {
        // First dirty frame arms the timer; the checkpoint follows one
        // full interval later.
        *last = Some(now);
        ctx.request_repaint_after(interval);
        return;
    };
    let elapsed = now.duration_since(armed);
    if elapsed < interval {
        // Idle windows produce no frames — keep one queued so the
        // checkpoint lands on time without an interaction.
        ctx.request_repaint_after(interval - elapsed);
        return;
    }

    // A checkpoint that was written is not an event: only its failure is,
    // and only when it starts. A failure that persists would otherwise
    // raise an error at every interval, and an error stays until answered;
    // the recovery is recorded instead, so the Console shows the span.
    let was_failing = platform.autosave_failing;
    let written =
        crate::workbench::lifecycle::recovery_checkpoint::write_checkpoint(&path, &model.schematic);
    platform.autosave_failing = written.is_err();
    match written {
        Err(e) if !was_failing => model.report_failure("Autosave failed", e.to_string()),
        Ok(_) if was_failing => {
            model.record(
                LogSeverity::Info,
                LogSource::Studio,
                "Autosave is working again.",
            );
        }
        Err(_) | Ok(_) => {}
    }
    *last = Some(now);
}

/// An answer a background thread leaves for the frame to take.
type Slot<T> = std::sync::Arc<std::sync::Mutex<Option<T>>>;

fn take_slot<T>(slot: &mut Option<Slot<T>>) -> Option<T> {
    let found = slot
        .as_ref()?
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .take()?;
    *slot = None;
    Some(found)
}

/// Work for a background thread whose answer the frame takes later.
#[cfg(not(target_arch = "wasm32"))]
fn spawn_into<T: Send + 'static>(
    ctx: Option<&Context>,
    work: impl FnOnce() -> T + Send + 'static,
) -> Slot<T> {
    let slot: Slot<T> = Slot::default();
    let filled = std::sync::Arc::clone(&slot);
    let repaint = ctx.cloned();
    crate::workbench::platform::spawn_or_inline(move || {
        let answer = work();
        *filled
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(answer);
        if let Some(repaint) = repaint {
            repaint.request_repaint();
        }
    });
    slot
}

/// Work for a browser task whose answer the frame takes later.
fn spawn_task_into<T: 'static>(
    ctx: Option<&Context>,
    work: impl std::future::Future<Output = T> + 'static,
) -> Slot<T> {
    let slot: Slot<T> = Slot::default();
    let filled = std::sync::Arc::clone(&slot);
    durable_records::spawn_task(ctx, async move {
        let answer = work.await;
        *filled
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(answer);
    });
    slot
}

/// An autosave written, and whether the session's runs gave way to it.
#[derive(Debug, Clone)]
pub struct AutosaveWritten {
    pub summary: ProjectCheckpointSummary,
    pub runs_given_up: bool,
}

/// The open project's autosave: the timer, the checkpoint being written off
/// the frame, the autosaves this process wrote, and the recovered work an
/// open or a start found.
#[derive(Debug, Clone, Default)]
pub struct ProjectAutosave {
    /// When the next autosave is due: an interval after the first frame
    /// with unsaved work, or after the last autosave started.
    due: Option<Instant>,
    /// The autosave being written on a background thread, or in a browser
    /// task.
    writing: Option<Slot<Result<AutosaveWritten, String>>>,
    /// The autosaves this process wrote. Never offered back to it as
    /// recovered work, and removed when it lets the project go.
    written: Vec<ProjectCheckpointSummary>,
    /// The saved project they were written over. A save after them removed
    /// them, or left them older than the file, where no start offers them.
    written_over: Option<u64>,
    /// The last autosave could not be written: said when it starts, not at
    /// every interval it persists.
    failing: bool,
    /// The application is closing after its unsaved work was saved or given
    /// up, as File > Exit asks; closing the window does not ask.
    exit_reviewed: bool,
    /// The search for recovered work an open or a start began.
    search: Option<Slot<Result<Option<RecoveredWork>, String>>>,
    /// Recovered work found for the open project, for the dialog to offer.
    pub offer: Option<RecoveredWork>,
    /// The working project as a browser start reopened it, with the unsaved
    /// work its session kept, while recovered work is looked for.
    pub restart_stamp: Option<project_lifecycle::WorkingStamp>,
    /// The search, after an open failed, for a checkpoint of that file.
    #[cfg(not(target_arch = "wasm32"))]
    rescue_search: Option<Slot<Option<UnopenedProject>>>,
    /// A project file that would not open and its newest checkpoint that
    /// reads intact, for the dialog to offer.
    #[cfg(not(target_arch = "wasm32"))]
    pub rescue: Option<UnopenedProject>,
}

/// A project file that would not open, and the newest of the checkpoints
/// beside it that reads back intact.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
pub struct UnopenedProject {
    pub path: PathBuf,
    /// Why it would not open, as the open said.
    pub error: String,
    pub checkpoint: ProjectCheckpointSummary,
}

impl ProjectAutosave {
    /// Make the autosave due now, for a test that cannot wait an interval.
    #[cfg(any(test, feature = "test-support"))]
    pub fn arm_due(&mut self) {
        self.due = Some(Instant::now());
    }

    /// Whether an autosave is being written or recovered work looked for.
    pub fn busy(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        let rescuing = self.rescue_search.is_some();
        #[cfg(target_arch = "wasm32")]
        let rescuing = false;
        self.writing.is_some() || self.search.is_some() || rescuing
    }

    /// The autosaves this process wrote and still holds.
    pub fn written(&self) -> &[ProjectCheckpointSummary] {
        &self.written
    }
}

/// An autosave of the open project newer than its file and different from
/// it, as an open or a start found it: read back and verified, with what it
/// changes.
#[derive(Debug, Clone)]
pub struct RecoveredWork {
    pub project_id: String,
    pub project_name: String,
    pub autosave: ProjectCheckpointSummary,
    /// The autosave's project, read back and verified.
    pub project: Box<crate::io::ProjectSnapshot>,
    /// The documents in which the saved project differs from the autosave,
    /// as the Recovery page's comparison states them.
    pub differences: Vec<DocumentDifference>,
}

/// Write an Autosave checkpoint of the open project when the interval has
/// passed with unsaved work in it. The project is read off the model on the
/// frame; writing it runs on a background thread, and its answer is taken on
/// a later frame. Only a project with a file is autosaved: its checkpoints
/// are kept beside it. Returns whether an autosave was written since the last
/// call, so the Recovery page lists again.
pub fn project_autosave_tick(state: &mut dyn LifecycleSourceMut, ctx: &Context) -> bool {
    let parts = &mut state.lifecycle_parts();
    let written = take_written_autosave(parts);
    let saved = project_lifecycle::accepted_generation(parts.platform);
    let autosave = &mut parts.platform.project_autosave;
    if autosave.written_over.is_some_and(|over| over != saved) {
        autosave.written.clear();
        autosave.written_over = None;
    }
    let minutes = parts.platform.autosave_minutes;
    let unsaved = autosaved_work(parts);
    let autosave = &mut parts.platform.project_autosave;
    if minutes == 0 || !unsaved {
        autosave.due = None;
        return written;
    }
    let now = Instant::now();
    let interval = std::time::Duration::from_secs(u64::from(minutes) * 60);
    let Some(due) = autosave.due else {
        autosave.due = Some(now + interval);
        ctx.request_repaint_after(interval);
        return written;
    };
    // A shorter interval chosen in Preferences since the timer was armed
    // counts from now.
    let due = due.min(now + interval);
    autosave.due = Some(due);
    if now < due {
        // An idle window paints no frames: one is kept queued so the autosave
        // is written on time without an interaction.
        ctx.request_repaint_after(due - now);
        return written;
    }
    if autosave.writing.is_some() {
        return written;
    }
    autosave.due = Some(now + interval);
    match start_writing(parts, Some(ctx)) {
        Ok(writing) => parts.platform.project_autosave.writing = Some(writing),
        Err(error) => note_autosave_result(parts, Err(error)),
    }
    written
}

/// Start writing an Autosave checkpoint of the open project: read off the
/// model here, encoded and written on a background thread, or, in the
/// browser, in a task of its own into the browser's records, where the
/// session's runs give way to it when storage is full.
fn start_writing(
    parts: &LifecycleParts<'_>,
    ctx: Option<&Context>,
) -> Result<Slot<Result<AutosaveWritten, String>>, String> {
    if let Some(records) = durable_records::browser_records() {
        // The autosaves this session wrote hold older work, which this one
        // replaces.
        let capture = project_checkpoint::capture_for_records(
            parts,
            ProjectCheckpointReason::Autosave,
            saved_generation(parts.platform),
        )?
        .superseding(parts.platform.project_autosave.written.clone());
        return Ok(spawn_task_into(ctx, async move {
            let gives_way: &dyn DurableRecords = &*records.session;
            capture
                .publish(
                    &*records.checkpoints,
                    Some((gives_way, browser_session::RUNS_KEY)),
                )
                .await
                .map(|written| AutosaveWritten {
                    summary: written.summary,
                    runs_given_up: written.gave_way,
                })
        }));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let capture = project_checkpoint::capture(parts, ProjectCheckpointReason::Autosave)?;
        Ok(spawn_into(ctx, move || {
            capture.publish().map(|created| AutosaveWritten {
                summary: created.summary,
                runs_given_up: false,
            })
        }))
    }
    #[cfg(target_arch = "wasm32")]
    Err("this browser keeps no records".to_owned())
}

/// The save of the open project its browser autosaves are written over: the
/// generation its binding accepted. The desktop dates its autosaves against
/// the file instead.
fn saved_generation(platform: &PlatformState) -> Option<u64> {
    platform
        .browser_project_binding_receipt
        .as_ref()
        .map(|receipt| receipt.accepted_generation)
}

/// Whether the open project has unsaved work an autosave would keep: only a
/// project with a saved home, which its checkpoints are kept with (beside its
/// file on the desktop; in the browser, its binding to a file or to the
/// browser's own storage), and one this session may write.
fn autosaved_work(parts: &mut LifecycleParts<'_>) -> bool {
    let saved = if durable_records::browser_records().is_some() {
        parts.platform.project_lifecycle.binding().is_some()
    } else {
        parts.model.workspace.content.project.path.is_some()
    };
    parts.platform.project_lifecycle.is_open()
        && saved
        && !parts.model.safe_mode.project_read_only()
        && project_lifecycle::has_unsaved_changes(parts)
}

/// Write an autosave of the open project's unsaved work now, as the browser
/// does when its page is left: a browser has no exit to write it at.
pub fn write_now(state: &mut dyn LifecycleSourceMut, ctx: Option<&Context>) {
    let parts = &mut state.lifecycle_parts();
    if parts.platform.autosave_minutes == 0
        || parts.platform.project_autosave.writing.is_some()
        || !autosaved_work(parts)
    {
        return;
    }
    match start_writing(parts, ctx) {
        Ok(writing) => parts.platform.project_autosave.writing = Some(writing),
        Err(error) => note_autosave_result(parts, Err(error)),
    }
}

/// Take an autosave the background thread finished writing.
fn take_written_autosave(parts: &mut LifecycleParts<'_>) -> bool {
    let Some(result) = take_slot(&mut parts.platform.project_autosave.writing) else {
        return false;
    };
    let written = result.is_ok();
    note_autosave_result(parts, result);
    written
}

/// Keep what an autosave wrote, and say a failure once, when it starts.
fn note_autosave_result(parts: &mut LifecycleParts<'_>, result: Result<AutosaveWritten, String>) {
    let saved = project_lifecycle::accepted_generation(parts.platform);
    if result.as_ref().is_ok_and(|written| written.runs_given_up) {
        browser_session::runs_gave_way(parts.model, parts.platform);
    }
    let autosave = &mut parts.platform.project_autosave;
    let was_failing = autosave.failing;
    autosave.failing = result.is_err();
    match result.map(|written| written.summary) {
        Ok(summary) => {
            // Retention keeps only the newest autosave.
            autosave.written.clear();
            autosave.written.push(summary);
            autosave.written_over = Some(saved);
            if was_failing {
                parts.model.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    "Autosave is working again.",
                );
            }
        }
        Err(error) if !was_failing => {
            parts.model.report_failure("Autosave failed", error);
        }
        Err(_) => {}
    }
}

/// The open project is going away, closed or replaced, with its unsaved work
/// saved or given up: the autosaves this process wrote of it go before the
/// next project is looked at, and nothing found for it is offered any more.
pub fn project_left(platform: &mut PlatformState) {
    remove_autosaves(std::mem::take(&mut platform.project_autosave).written);
}

/// Remove autosaves this process wrote, from the browser's records or from
/// beside the project's file.
fn remove_autosaves(written: Vec<ProjectCheckpointSummary>) {
    if written.is_empty() {
        return;
    }
    if let Some(records) = durable_records::browser_records() {
        durable_records::spawn_task(None, async move {
            for summary in written {
                if let Err(error) =
                    project_checkpoint::delete_records(&*records.checkpoints, &summary).await
                {
                    log::warn!("an autosave this session wrote was not removed: {error}");
                }
            }
        });
        return;
    }
    #[cfg(not(target_arch = "wasm32"))]
    for summary in written {
        if let Err(error) = project_checkpoint::delete(&summary) {
            log::warn!("an autosave this session wrote was not removed: {error}");
        }
    }
}

/// The exit was asked for and its unsaved work saved or given up.
pub fn note_reviewed_exit(platform: &mut PlatformState) {
    platform.project_autosave.exit_reviewed = true;
}

/// The application is closing. Unsaved work a closed window leaves is
/// written as an autosave, which the next start offers back with its time;
/// the session keeps none of it. Work saved or given up at File > Exit takes
/// the autosaves this process wrote with it. Written here, on the way out,
/// since no frame follows.
#[cfg(not(target_arch = "wasm32"))]
pub fn at_exit(state: &mut dyn LifecycleSourceMut) {
    let parts = &mut state.lifecycle_parts();
    if parts.platform.project_autosave.exit_reviewed || !autosaved_work(parts) {
        remove_own_autosaves(parts.platform);
        return;
    }
    match project_checkpoint::capture(parts, ProjectCheckpointReason::Autosave)
        .and_then(project_checkpoint::CheckpointCapture::publish)
    {
        // Retention kept only this one, the newest autosave.
        Ok(_) => parts.platform.project_autosave.written.clear(),
        // The last autosave written on time is what the next start offers.
        Err(error) => {
            log::warn!("the unsaved work was not autosaved at exit: {error}");
            parts.platform.project_autosave.written.clear();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn remove_own_autosaves(platform: &mut PlatformState) {
    remove_autosaves(std::mem::take(&mut platform.project_autosave.written));
}

/// Look for recovered work for the project just opened or reopened at start,
/// on a background thread: its newest autosave, if another session wrote it
/// after the project file and it holds something the file does not. A
/// project without a file has nowhere such an autosave is kept.
pub fn start_recovered_work_search(
    model: &Model,
    platform: &mut PlatformState,
    ctx: Option<&Context>,
) {
    let autosave = &mut platform.project_autosave;
    autosave.offer = None;
    autosave.search = None;
    autosave.restart_stamp = None;
    if let Some(records) = durable_records::browser_records() {
        start_browser_search(model, platform, records, ctx);
        return;
    }
    #[cfg(not(target_arch = "wasm32"))]
    start_native_search(model, platform, ctx);
}

/// A browser start reopened the project its session kept, now bound again to
/// its saved home, with the unsaved work the session kept: an autosave the
/// last session left of it is looked for, and the working project as it is
/// now remembered, to put the saved one back if the reader is asked.
pub fn start_restart_search(parts: &mut LifecycleParts<'_>) {
    start_recovered_work_search(parts.model, parts.platform, None);
    let stamp = project_lifecycle::working_stamp(&*parts);
    parts.platform.project_autosave.restart_stamp = Some(stamp);
}

/// Look for recovered work in the browser's records: the project's newest
/// autosave another session wrote over the save its file still holds, when
/// it differs from that save. Only a project with a saved home has one.
fn start_browser_search(
    model: &Model,
    platform: &mut PlatformState,
    records: BrowserRecords,
    ctx: Option<&Context>,
) {
    if platform.project_lifecycle.binding().is_none() {
        return;
    }
    let Ok(saved) = platform.project_lifecycle.accepted_snapshot() else {
        return;
    };
    let generation = saved_generation(platform);
    let own = platform
        .project_autosave
        .written
        .iter()
        .map(ProjectCheckpointSummary::checkpoint_id)
        .collect::<Vec<_>>();
    let project_id = model.workspace.content.project.id().to_string();
    let project_name = model.workspace.content.project.name().to_owned();
    platform.project_autosave.search = Some(spawn_task_into(ctx, async move {
        find_recovered_work_in(&*records.checkpoints, &project_id, &own, &saved, generation)
            .await
            .map(|found| {
                found.map(|(autosave, project, differences)| RecoveredWork {
                    project_id,
                    project_name,
                    autosave,
                    project: Box::new(project),
                    differences,
                })
            })
    }));
}

/// The project's newest autosave in `records` that this process did not
/// write, when it was written over the save `generation` names and differs
/// from `saved`: read back and verified, and pinned so the autosaves this
/// session writes do not prune it before the reader saves.
async fn find_recovered_work_in(
    records: &dyn DurableRecords,
    project_id: &str,
    own: &[uuid::Uuid],
    saved: &crate::io::ProjectSnapshot,
    generation: Option<u64>,
) -> Result<
    Option<(
        ProjectCheckpointSummary,
        crate::io::ProjectSnapshot,
        Vec<DocumentDifference>,
    )>,
    String,
> {
    let checkpoints = project_checkpoint::list_records(records, project_id).await?;
    let Some(newest) = checkpoints.into_iter().find(|checkpoint| {
        checkpoint.reason() == ProjectCheckpointReason::Autosave
            && !own.contains(&checkpoint.checkpoint_id())
    }) else {
        return Ok(None);
    };
    if newest.saved_generation() != generation {
        return Ok(None);
    }
    let recovered = project_checkpoint::load_record(records, &newest).await?;
    let differences = project_checkpoint::differences_between(saved, &recovered)?;
    if differences.is_empty() {
        return Ok(None);
    }
    if !newest.pinned()
        && let Err(error) =
            project_checkpoint::edit_records(records, &newest, &CheckpointEdit::Pin(true)).await
    {
        log::warn!("the recovered autosave was not kept from retention: {error}");
    }
    Ok(Some((newest, recovered, differences)))
}

/// Look beside the project's file for recovered work, on a background
/// thread.
#[cfg(not(target_arch = "wasm32"))]
fn start_native_search(model: &Model, platform: &mut PlatformState, ctx: Option<&Context>) {
    let autosave = &mut platform.project_autosave;
    let Some(path) = model.workspace.content.project.path.clone() else {
        return;
    };
    let Ok(listing) = project_checkpoint::listing(model) else {
        return;
    };
    let own = autosave
        .written
        .iter()
        .map(ProjectCheckpointSummary::checkpoint_id)
        .collect::<Vec<_>>();
    let project_id = model.workspace.content.project.id().to_string();
    let project_name = model.workspace.content.project.name().to_owned();
    autosave.search = Some(spawn_into(ctx, move || {
        find_recovered_work(&listing, &path, &own).map(|found| {
            found.map(|(autosave, project, differences)| RecoveredWork {
                project_id,
                project_name,
                autosave,
                project: Box::new(project),
                differences,
            })
        })
    }));
}

/// Take what the search for recovered work found, for the project still open.
pub fn take_recovered_work(model: &mut Model, platform: &mut PlatformState) {
    let Some(found) = take_slot(&mut platform.project_autosave.search) else {
        return;
    };
    match found {
        Ok(Some(found)) if found.project_id == model.workspace.content.project.id().to_string() => {
            platform.project_autosave.offer = Some(found);
        }
        Ok(_) => {}
        Err(error) => {
            model.record(
                LogSeverity::Warning,
                LogSource::Studio,
                format!("Recovered work for the project could not be looked for: {error}"),
            );
        }
    }
}

/// The project's newest autosave that this process did not write, when it is
/// newer than the project file at `path` and differs from it: read back and
/// verified, with the documents the saved project differs in.
#[cfg(not(target_arch = "wasm32"))]
fn find_recovered_work(
    listing: &project_checkpoint::CheckpointListing,
    path: &std::path::Path,
    own: &[uuid::Uuid],
) -> Result<
    Option<(
        ProjectCheckpointSummary,
        crate::io::ProjectSnapshot,
        Vec<DocumentDifference>,
    )>,
    String,
> {
    let catalog = listing.list()?;
    let Some(newest) = catalog.checkpoints.into_iter().find(|checkpoint| {
        checkpoint.reason() == ProjectCheckpointReason::Autosave
            && !own.contains(&checkpoint.checkpoint_id())
    }) else {
        return Ok(None);
    };
    let saved_at = std::fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
        .and_then(|modified| modified.duration_since(std::time::UNIX_EPOCH).ok())
        .map_or(0, |since| {
            u64::try_from(since.as_millis()).unwrap_or(u64::MAX)
        });
    if newest.created_unix_ms() <= saved_at {
        return Ok(None);
    }
    let recovered = project_checkpoint::load(&newest)?;
    let saved = crate::io::load_project_file(path)
        .map_err(|error| format!("the project file could not be read: {error}"))?;
    let differences = project_checkpoint::differences_between(&saved, &recovered)?;
    if differences.is_empty() {
        return Ok(None);
    }
    // Kept until the project is saved, whatever the reader answers: the
    // autosaves this session writes would otherwise replace it.
    if !newest.pinned()
        && let Err(error) =
            project_checkpoint::edit(&newest, &project_checkpoint::CheckpointEdit::Pin(true))
    {
        log::warn!("the recovered autosave was not kept from retention: {error}");
    }
    Ok(Some((newest, recovered, differences)))
}

/// An open of the project file at `path` failed: look beside it, on a
/// background thread, for the newest checkpoint that reads back intact.
#[cfg(not(target_arch = "wasm32"))]
pub fn start_unopened_project_search(platform: &mut PlatformState, path: &Path, error: String) {
    let path = path.to_path_buf();
    let autosave = &mut platform.project_autosave;
    autosave.rescue = None;
    autosave.rescue_search = Some(spawn_into(None, move || {
        let catalog = project_checkpoint::list_beside(&path)
            .inspect_err(|error| {
                log::warn!("no checkpoint of {} was listed: {error}", path.display())
            })
            .ok()?;
        let checkpoint = catalog
            .checkpoints
            .into_iter()
            .find(|checkpoint| project_checkpoint::verify(checkpoint).is_ok())?;
        Some(UnopenedProject {
            path,
            error,
            checkpoint,
        })
    }));
}

/// Take what the search after a failed open found.
#[cfg(not(target_arch = "wasm32"))]
pub fn take_unopened_project(platform: &mut PlatformState) {
    if let Some(found) = take_slot(&mut platform.project_autosave.rescue_search) {
        platform.project_autosave.rescue = found;
    }
}

/// Offer the newest intact checkpoint of a project file that would not open,
/// saved as a new project: the file itself is left as it is. Returns the
/// checkpoint when the reader asked for it.
#[cfg(not(target_arch = "wasm32"))]
pub fn unopened_project_dialog(
    platform: &mut PlatformState,
    ctx: &Context,
) -> Option<Option<ProjectCheckpointSummary>> {
    let found = platform.project_autosave.rescue.as_ref()?;
    let file = found.path.file_name().map_or_else(
        || found.path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    );
    let created = found.checkpoint.created_unix_ms();
    let sentence = format!(
        "\u{201c}{file}\u{201d} did not open: {}. Its newest checkpoint that reads intact, \
         \u{201c}{}\u{201d}, is from {} ({}). Save it as a new project to open it; the file \
         is left as it is.",
        found.error.trim_end_matches('.'),
        found.checkpoint.title(),
        checkpoint_time(created),
        autosave_age(created),
    );
    let choice = Dialog::new("Open a checkpoint instead?", "Save as new project\u{2026}")
        .description(&sentence)
        .size(DialogSize::Transaction)
        .secondary("Not now")
        .hint("Every checkpoint of a recent project is also on the launcher's Recovery page.")
        .show(ctx, |ui| {
            let c = Tokens::get(ui.ctx()).color;
            ui.label(
                egui::RichText::new(&sentence)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.text),
            );
        });
    let save = match choice {
        DialogChoice::None => return None,
        DialogChoice::Primary => true,
        DialogChoice::Secondary | DialogChoice::Ghost | DialogChoice::Cancelled => false,
    };
    let found = platform.project_autosave.rescue.take()?;
    Some(save.then_some(found.checkpoint))
}

/// What the reader chose in the recovered-work dialog.
pub enum RecoveredWorkAnswer {
    /// Put the autosave back in the open project as unsaved changes.
    Keep(RecoveredWork),
    /// Keep the saved project open; the autosave stays on the Recovery page
    /// until the next save.
    OpenSaved(RecoveredWork),
    /// Show the Recovery page with what the autosave changes listed.
    Compare(RecoveredWork),
}

/// Ask once what to do with recovered work: keep it, open the saved project,
/// or compare the two. Esc opens the saved project.
pub fn recovered_work_dialog(
    platform: &mut PlatformState,
    ctx: &Context,
) -> Option<RecoveredWorkAnswer> {
    let found = platform.project_autosave.offer.as_ref()?;
    let modified = found.autosave.created_unix_ms();
    let sentence = recovered_work_sentence(
        &found.project_name,
        &checkpoint_time(modified),
        &autosave_age(modified),
        found.differences.len(),
    );
    let choice = Dialog::new("Recover unsaved changes?", "Keep the recovered changes")
        .description(&sentence)
        .size(DialogSize::Transaction)
        .secondary("Open the saved project")
        .ghost("Compare")
        .hint("Esc opens the saved project; the changes stay on the Recovery page until you save.")
        .show(ctx, |ui| {
            let c = Tokens::get(ui.ctx()).color;
            ui.label(
                egui::RichText::new(&sentence)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.text),
            );
            ui.add_space(6.0);
            for difference in &found.differences {
                ui.label(
                    egui::RichText::new(format!(
                        "{}  {}",
                        difference.kind.label(),
                        difference.document.label()
                    ))
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.text_dim),
                );
            }
        });
    let answer = match choice {
        DialogChoice::None => return None,
        DialogChoice::Primary => RecoveredWorkAnswer::Keep,
        DialogChoice::Ghost => RecoveredWorkAnswer::Compare,
        DialogChoice::Secondary | DialogChoice::Cancelled => RecoveredWorkAnswer::OpenSaved,
    };
    platform.project_autosave.offer.take().map(answer)
}

/// What the recovered-work dialog says was found: whose changes, from when,
/// and how many documents they touch.
fn recovered_work_sentence(project: &str, time: &str, age: &str, documents: usize) -> String {
    let documents = if documents == 1 {
        "1 document".to_owned()
    } else {
        format!("{documents} documents")
    };
    format!(
        "RSpice closed with unsaved changes to \u{201c}{project}\u{201d}. Its autosave from \
         {time} ({age}) differs from the saved project in {documents}."
    )
}

/// What the reader chose in the restore dialog, with the checkpoint work the
/// choice needed already done. Loading it, and then dropping the pending
/// restore, is the caller's.
#[cfg(not(target_arch = "wasm32"))]
pub enum AutosaveRestore {
    /// The verified checkpoint, as unsaved changes over the file's identity,
    /// and the line to record once it is loaded.
    Checkpoint {
        path: PathBuf,
        schematic: Box<SchematicState>,
        note: String,
    },
    /// The saved file, and the line to record once it is opened.
    SavedFile { path: PathBuf, note: Option<String> },
}

/// Show the restore decision an open deferred: Restore reads the checkpoint,
/// Discard deletes it, Esc keeps it. A checkpoint that could not be read or
/// deleted is recorded here and the decision stays pending.
#[cfg(not(target_arch = "wasm32"))]
pub fn restore_dialog(
    model: &mut Model,
    platform: &PlatformState,
    ctx: &Context,
) -> Option<AutosaveRestore> {
    let candidate = platform.pending_autosave_restore.clone()?;
    let path = candidate.source.clone();
    let checkpoint = candidate.checkpoint.clone();
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string());
    let modified = candidate.binding.identity.modified_unix_ms;
    let age = autosave_age(modified);
    let time = autosave_time(modified);
    let can_discard = candidate.can_discard();

    let choice = Dialog::new("Restore autosave?", "Restore")
        .description(
            "An interrupted session left an autosave of this drawing. Restore it, keep it, or discard it.",
        )
        .size(DialogSize::Transaction)
        .ghost(if can_discard {
            "Discard"
        } else {
            "Keep autosave"
        })
        .hint("Esc opens the saved file and keeps the autosave.")
        .show(ctx, |ui| {
            let c = Tokens::get(ui.ctx()).color;
            ui.label(
                egui::RichText::new(restore_sentence(&file_name, &time, &age))
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.text_dim),
            );
        });

    match choice {
        DialogChoice::None | DialogChoice::Secondary => None,
        DialogChoice::Primary => {
            let restored = crate::workbench::lifecycle::recovery_checkpoint::read_bound_checkpoint(
                &path,
                &checkpoint,
                &candidate.binding,
            )
            .and_then(|bytes| {
                let text = std::str::from_utf8(&bytes)
                    .map_err(|error| format!("the autosave is not UTF-8 JSON: {error}"))?;
                crate::io::schematic_io::load_schematic_text(text, Some(&path))
                    .map_err(|error| error.to_string())
            });
            match restored {
                Ok(mut schematic) => {
                    schematic.session.current_file = Some(path.clone());
                    schematic.session.is_dirty = true;
                    Some(AutosaveRestore::Checkpoint {
                        path,
                        schematic: Box::new(schematic),
                        note: format!("Restored the autosave from {age}; save to keep it."),
                    })
                }
                Err(error) => {
                    model.record(
                        LogSeverity::Error,
                        LogSource::Studio,
                        format!(
                            "The autosave could not be restored: it changed or could not be read: {error}"
                        ),
                    );
                    None
                }
            }
        }
        DialogChoice::Ghost => {
            if can_discard {
                match crate::workbench::lifecycle::recovery_checkpoint::discard_bound_checkpoint(
                    &path,
                    &checkpoint,
                    &candidate.binding,
                ) {
                    Ok(()) => Some(AutosaveRestore::SavedFile { path, note: None }),
                    Err(error) => {
                        model.record(
                            LogSeverity::Error,
                            LogSource::Studio,
                            format!(
                                "The autosave could not be discarded and is still on disk: {error}"
                            ),
                        );
                        None
                    }
                }
            } else {
                Some(AutosaveRestore::SavedFile {
                    path,
                    note: Some(format!(
                        "Kept the autosave for review in Recovery: {}",
                        checkpoint.display()
                    )),
                })
            }
        }
        DialogChoice::Cancelled => Some(AutosaveRestore::SavedFile {
            path,
            note: Some(format!("Kept the autosave at {}.", checkpoint.display())),
        }),
    }
}

/// What the dialog says the autosave is: the drawing, when it was saved on
/// the reader's clock, and how long ago.
fn restore_sentence(file_name: &str, time: &str, age: &str) -> String {
    format!(
        "Restore the autosaved drawing from {time} ({age}), or open the saved \
         '{file_name}' and keep the autosave."
    )
}

/// When the autosave was written, on the reader's clock: an autosave is this
/// computer's own record of its interrupted work.
fn autosave_time(modified_unix_ms: u64) -> String {
    (modified_unix_ms != 0)
        .then(|| crate::time_compat::local_date_time(modified_unix_ms))
        .flatten()
        .map_or_else(
            || "an earlier session".to_owned(),
            |stamp| {
                format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}",
                    stamp.year(),
                    u8::from(stamp.month()),
                    stamp.day(),
                    stamp.hour(),
                    stamp.minute()
                )
            },
        )
}

/// When a project checkpoint was taken, spelled as the Recovery page and the
/// Console spell it: a checkpoint travels with its project, so its time is
/// UTC, and a dialog must not name it at a second time.
fn checkpoint_time(created_unix_ms: u64) -> String {
    if created_unix_ms == 0 {
        "an earlier session".to_owned()
    } else {
        crate::time_compat::utc_stamp(created_unix_ms)
    }
}

/// Coarse human age of an autosave ("3 min ago"), from the modification
/// time read once when it was found and offered: the dialog asks the clock,
/// never the file.
fn autosave_age(modified_unix_ms: u64) -> String {
    let elapsed = crate::time_compat::checked_unix_time_ms()
        .ok()
        .filter(|_| modified_unix_ms != 0)
        .and_then(|now| now.checked_sub(modified_unix_ms));
    match elapsed {
        None => "an earlier session".to_owned(),
        Some(age) => crate::time_compat::relative_age(std::time::Duration::from_millis(age)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The dialog names what it restores and when, in the reader's words:
    /// the autosaved drawing, at a time on the clock.
    #[test]
    fn the_restore_sentence_names_the_autosaved_drawing_and_its_time() {
        let sentence = restore_sentence(
            "amp.rsch",
            &autosave_time(1_760_000_000_000),
            &autosave_age(1_760_000_000_000),
        );
        assert!(
            sentence.starts_with("Restore the autosaved drawing from 20"),
            "{sentence}"
        );
        assert!(sentence.contains("'amp.rsch'"), "{sentence}");
        for internal in ["reviewed bytes", "checkpoint"] {
            assert!(!sentence.contains(internal), "{sentence}");
        }
        assert_eq!(autosave_time(0), "an earlier session");
    }

    /// A project checkpoint is named at the time the Recovery page and the
    /// Console give it, not at the reader's local time.
    #[test]
    fn a_project_checkpoint_is_named_at_its_recovery_page_time() {
        let taken = 1_760_000_000_000;
        let time = checkpoint_time(taken);
        assert_eq!(time, crate::time_compat::utc_stamp(taken));
        assert!(time.ends_with(" UTC"), "{time}");
        let sentence = recovered_work_sentence("amp", &time, "3 min ago", 2);
        assert!(
            sentence.contains(&format!("autosave from {time} (3 min ago)")),
            "{sentence}"
        );
        assert_eq!(checkpoint_time(0), "an earlier session");
    }
}
