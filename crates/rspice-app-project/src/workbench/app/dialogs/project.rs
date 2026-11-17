//! Project-lifecycle dialogs: the transactions that create or replace the
//! whole session project rather than editing something inside one, and the
//! configuration transactions that decide how the project netlists.
//!
//! The project surface draws two more project-wide transactions, attaching a
//! technology and recovering from a checkpoint; their drafts live here, below
//! the dialog aggregate that holds them.

pub mod configuration;
pub mod configuration_dialogs;
pub mod new_project;
mod properties;

pub use properties::ProjectPropertiesDialogState;

/// Retained draft for the mockup-owned project technology transaction.
/// Selection remains isolated until the primary action verifies every pinned
/// source and commits one project metadata revision.
#[derive(Debug, Clone, Default)]
pub struct TechnologyAttachmentDialogState {
    pub open: bool,
    /// The dialog detaches the project's technology rather than attaching
    /// one.
    pub(crate) detach: bool,
    pub(crate) selected_library: Option<String>,
    pub(crate) selected_pdk_package: Option<String>,
    pub(crate) actor_id: String,
    pub(crate) authority_id: String,
    pub(crate) reason: String,
    pub(crate) migration_reviewed: bool,
    pub(crate) validation_error: Option<String>,
    /// The change waiting on its checkpoint, while that is written.
    pub(crate) in_flight: Option<TechnologyChangeInFlight>,
}

/// What a technology transaction does once its checkpoint is written.
#[derive(Debug, Clone)]
pub(crate) enum TechnologyChange {
    Attach {
        binding: crate::state::ProjectTechnologyBinding,
        migration_evidence: Option<crate::state::pdk_config::PdkTechnologyMigrationEvidence>,
    },
    /// The project's technology removed.
    Detach,
}

/// A technology change waiting on the whole-project checkpoint written
/// before it, off the frame: the project it was asked of, at the revision
/// the checkpoint captures, and the change with who made it.
#[derive(Debug, Clone)]
pub(crate) struct TechnologyChangeInFlight {
    pub(crate) project_id: String,
    pub(crate) expected_revision: u64,
    pub(crate) change: TechnologyChange,
    pub(crate) authority: crate::state::ProjectTechnologyChangeAuthority,
    /// The working project as the checkpoint captured it: the change
    /// commits only onto that project.
    pub(crate) captured: crate::workbench::lifecycle::project_lifecycle::WorkingStamp,
    /// The checkpoint being written on a background thread.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) job: CheckpointJob,
}

#[derive(Debug, Clone, Default)]
pub struct ProjectCheckpointRecoveryState {
    pub project_id: Option<String>,
    pub checkpoints: Vec<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary>,
    pub quarantined:
        Vec<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointQuarantine>,
    pub error: Option<String>,
    pub initialized: bool,
    /// Where the checkpoints are kept, as the page states it, worked out
    /// once per project rather than on every frame.
    pub(crate) location: Option<String>,
    /// What the catalog was listed for: the saved project it belongs to and,
    /// in the browser, the store's own changes. A save moves the first, so
    /// the page lists again for it.
    pub(crate) listed_for: Option<(u64, u64)>,
    /// The note being written on a checkpoint, before it is kept.
    pub(crate) note_draft: Option<(uuid::Uuid, String)>,
    /// What the reader asked to delete, waiting for the answer.
    pub(crate) delete_review: Option<CheckpointDeletion>,
    /// The checkpoint the reader asked to restore, waiting for the answer.
    pub(crate) restore_review: Option<uuid::Uuid>,
    /// A checkpoint read back for a restore, with the "Before restore"
    /// checkpoint already written: the shell puts it in the open project.
    pub restored: Option<RestoredCheckpoint>,
    /// A checkpoint saved as a new project, and where: the page offers to
    /// open it.
    pub saved_copy: Option<(uuid::Uuid, std::path::PathBuf)>,
    /// What the last comparison found, for the checkpoint it compared.
    pub(crate) comparison: Option<(uuid::Uuid, CheckpointComparison)>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) loading: bool,
    /// A note, pin or deletion is being written in the browser.
    #[cfg(target_arch = "wasm32")]
    pub(crate) changing: bool,
    /// The listing, creation or comparison running on a background thread.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) job: Option<CheckpointJob>,
    /// Holds each job started from here until the test releases it.
    #[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
    pub job_hold: Option<CheckpointJobHold>,
}

impl ProjectCheckpointRecoveryState {
    /// Show what a comparison with `checkpoint` found under its row.
    pub fn set_comparison(&mut self, checkpoint: uuid::Uuid, found: CheckpointComparison) {
        self.comparison = Some((checkpoint, found));
    }

    /// List the checkpoints again. A listing already running may have read
    /// the directory before the change, so it is dropped and a new one
    /// starts; a creation or comparison still reports, and a creation's
    /// listing is followed by a fresh one.
    pub fn invalidate(&mut self) {
        self.initialized = false;
        self.error = None;
        #[cfg(not(target_arch = "wasm32"))]
        if self
            .job
            .as_ref()
            .is_some_and(|job| job.kind == CheckpointJobKind::List)
        {
            self.job = None;
        }
    }
}

/// A checkpoint to put back in the open project as unsaved changes, read and
/// verified off the frame, and the "Before restore" checkpoint that keeps the
/// project as it was.
#[derive(Debug, Clone)]
pub struct RestoredCheckpoint {
    pub checkpoint: crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
    pub project: crate::io::ProjectSnapshot,
    pub before: crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary,
}

/// What a comparison of the open project with a checkpoint found, by
/// document.
pub type CheckpointComparison =
    Vec<crate::workbench::lifecycle::project_checkpoint::DocumentDifference>;

/// The notice a checkpoint that could not be written raises: the action that
/// asked for it otherwise looks as if it did nothing.
pub(crate) const CHECKPOINT_NOT_CREATED: &str = "Checkpoint not created";
/// The notice a restore that could not be made raises.
pub(crate) const CHECKPOINT_NOT_RESTORED: &str = "Checkpoint not restored";

/// What a change to a checkpoint did: the Console line one that was made
/// leaves, if any, or the notice and reason of one that could not be.
pub(crate) type ChangeOutcome = Result<Option<String>, (&'static str, String)>;

/// A change to the browser's checkpoints that finished after the frame that
/// started it, for the Recovery page to take.
#[cfg(target_arch = "wasm32")]
pub(crate) struct BrowserCheckpointChange {
    pub(crate) project_id: String,
    pub(crate) outcome: ChangeOutcome,
    /// Started from the Recovery page, which waits for it.
    pub(crate) from_page: bool,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    pub(crate) static BROWSER_CHECKPOINT_CHANGES: std::cell::RefCell<std::collections::VecDeque<BrowserCheckpointChange>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

/// The completion of a checkpoint a transaction took and did not wait for:
/// the Recovery page lists again once it is written, and one that could not
/// be written raises a notice.
#[cfg(target_arch = "wasm32")]
pub(crate) fn checkpoint_written_later(
    project_id: String,
) -> impl FnOnce(
    Result<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary, String>,
) + 'static {
    move |result| {
        BROWSER_CHECKPOINT_CHANGES.with(|queue| {
            queue.borrow_mut().push_back(BrowserCheckpointChange {
                project_id,
                outcome: result
                    .map(|_| None)
                    .map_err(|error| (CHECKPOINT_NOT_CREATED, error)),
                from_page: false,
            });
        });
    }
}

/// What the reader asked to delete from the Recovery page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CheckpointDeletion {
    /// A checkpoint, by its identity.
    Checkpoint(uuid::Uuid),
    /// An artifact that failed verification, by its label.
    SetAside(String),
}

/// What a checkpoint job is doing, for the page to say while it runs.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CheckpointJobKind {
    List,
    Create,
    Compare(Box<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary>),
    /// A "Before restore" checkpoint written, then this one read back.
    Restore(Box<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary>),
    /// A note, a pin or a deletion, then the listing after it.
    Change,
}

/// What a checkpoint job found.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub(crate) enum CheckpointJobAnswer {
    Listed(
        Result<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointCatalog, String>,
    ),
    /// Boxed: the creation's summary and catalog dwarf the other answers.
    Created(
        Box<Result<crate::workbench::lifecycle::project_checkpoint::CheckpointCreated, String>>,
    ),
    Compared(Result<CheckpointComparison, String>),
    /// The "Before restore" checkpoint, then the checkpoint to restore read
    /// back: none is read when the first could not be written.
    Restored {
        before:
            Box<Result<crate::workbench::lifecycle::project_checkpoint::CheckpointCreated, String>>,
        project: Option<Result<Box<crate::io::ProjectSnapshot>, String>>,
    },
    /// What a change did and the catalog listed after it.
    Changed {
        outcome: ChangeOutcome,
        listed: Result<
            crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointCatalog,
            String,
        >,
    },
}

/// Checkpoint work on a background thread: the page goes on painting while
/// it runs, says what it is waiting for, and takes the answer on the first
/// frame after it is in.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
pub(crate) struct CheckpointJob {
    pub(crate) kind: CheckpointJobKind,
    answer: std::sync::Arc<std::sync::Mutex<Option<CheckpointJobAnswer>>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl CheckpointJob {
    /// Run `work` on a background thread and ask `ctx` for a frame when it
    /// has answered.
    pub(crate) fn start(
        ctx: &egui::Context,
        kind: CheckpointJobKind,
        recovery: &ProjectCheckpointRecoveryState,
        work: impl FnOnce() -> CheckpointJobAnswer + Send + 'static,
    ) -> Self {
        let answer = std::sync::Arc::new(std::sync::Mutex::new(None));
        let slot = std::sync::Arc::clone(&answer);
        let repaint = ctx.clone();
        #[cfg(any(test, feature = "test-support"))]
        let hold = recovery.job_hold.clone();
        #[cfg(not(any(test, feature = "test-support")))]
        let _ = recovery;
        rspice_app_services::workbench::platform::spawn_or_inline(move || {
            #[cfg(any(test, feature = "test-support"))]
            if let Some(hold) = hold {
                hold.wait();
            }
            let found = work();
            if let Ok(mut slot) = slot.lock() {
                *slot = Some(found);
            }
            repaint.request_repaint();
        });
        Self { kind, answer }
    }

    /// The answer, once, if it is in.
    pub(crate) fn take_answer(&self) -> Option<CheckpointJobAnswer> {
        self.answer.lock().ok()?.take()
    }
}

/// A gate a test closes to keep checkpoint jobs running while it paints.
#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
#[derive(Debug, Clone, Default)]
pub struct CheckpointJobHold(std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>);

#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
impl CheckpointJobHold {
    fn wait(&self) {
        let (released, signal) = &*self.0;
        let mut released = released.lock().unwrap_or_else(|poison| poison.into_inner());
        while !*released {
            released = signal
                .wait(released)
                .unwrap_or_else(|poison| poison.into_inner());
        }
    }

    /// Let every held job, and every job started after, run.
    pub fn release(&self) {
        let (released, signal) = &*self.0;
        *released.lock().unwrap_or_else(|poison| poison.into_inner()) = true;
        signal.notify_all();
    }
}

impl TechnologyAttachmentDialogState {
    /// Open on `selected_library` and `selected_pdk_package`, the change
    /// made and approved by `actor`, the person at this application, and no
    /// reason given yet.
    pub(crate) fn open(
        &mut self,
        selected_library: Option<String>,
        selected_pdk_package: Option<String>,
        actor: String,
    ) {
        *self = Self {
            open: true,
            detach: false,
            selected_library,
            selected_pdk_package,
            authority_id: actor.clone(),
            actor_id: actor,
            reason: String::new(),
            migration_reviewed: false,
            validation_error: None,
            in_flight: None,
        };
    }

    /// Open on detaching the project's technology, the change made and
    /// approved by `actor`, the person at this application.
    pub(crate) fn open_detach(&mut self, actor: String) {
        *self = Self {
            open: true,
            detach: true,
            authority_id: actor.clone(),
            actor_id: actor,
            ..Self::default()
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}
