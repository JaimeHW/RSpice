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

/// Retained draft for the mockup-owned project technology transaction.
/// Selection remains isolated until the primary action verifies every pinned
/// source and commits one project metadata revision.
#[derive(Debug, Clone, Default)]
pub struct TechnologyAttachmentDialogState {
    pub open: bool,
    pub(crate) selected_library: Option<String>,
    pub(crate) selected_pdk_package: Option<String>,
    pub(crate) actor_id: String,
    pub(crate) authority_id: String,
    pub(crate) reason: String,
    pub(crate) migration_reviewed: bool,
    pub(crate) validation_error: Option<String>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) checkpoint_pending: bool,
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
    #[cfg(target_arch = "wasm32")]
    pub(crate) loading: bool,
    /// The listing, creation or comparison running on a background thread.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) job: Option<CheckpointJob>,
    /// Holds each job started from here until the test releases it.
    #[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
    pub job_hold: Option<CheckpointJobHold>,
}

impl ProjectCheckpointRecoveryState {
    /// List the checkpoints again. A listing already running may have read
    /// the directory before the change, so it is dropped and a new one
    /// starts; a creation or comparison still reports, and a creation's
    /// listing is followed by a fresh one.
    pub(crate) fn invalidate(&mut self) {
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

/// What a checkpoint job is doing, for the page to say while it runs.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CheckpointJobKind {
    List,
    Create,
    Compare(crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary),
}

/// What a checkpoint job found.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub(crate) enum CheckpointJobAnswer {
    Listed(
        Result<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointCatalog, String>,
    ),
    Created(Result<crate::workbench::lifecycle::project_checkpoint::CheckpointCreated, String>),
    Compared(Result<bool, String>),
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
    pub(crate) fn open(
        &mut self,
        selected_library: Option<String>,
        selected_pdk_package: Option<String>,
    ) {
        *self = Self {
            open: true,
            selected_library,
            selected_pdk_package,
            actor_id: String::new(),
            authority_id: String::new(),
            reason: String::new(),
            migration_reviewed: false,
            validation_error: None,
            #[cfg(target_arch = "wasm32")]
            checkpoint_pending: false,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}
