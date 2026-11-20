//! One runner for Results work that can take long: an import, an export, a pack, a check of a
//! kept segment.
//!
//! A job runs on a thread of its own natively. It states its progress as work done of work
//! known, can be cancelled at any time, and ends with a result, a stated failure, or cancelled.
//! The window asks [`ResultsJob::state`] once a frame: atomics, and a lock the job holds only to
//! name a new step, so a frame never waits on the job's work.
//!
//! In the browser a job runs to its end when it starts: the page has one thread, and the
//! segments a Results job reads and writes live in the page's memory, where the simulation
//! worker cannot reach them.
//!
//! A job that is cancelled or fails leaves nothing behind: what it wrote is its own to delete,
//! and a value it would have handed back is dropped on its thread.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};

/// How a job ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobOutcome<T> {
    Done(T),
    /// The job could not finish, in words for the reader.
    Failed(String),
    Cancelled,
}

/// Why a job's work stopped before its end.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobStop {
    Cancelled,
    Failed(String),
}

impl From<String> for JobStop {
    fn from(reason: String) -> Self {
        Self::Failed(reason)
    }
}

impl From<&str> for JobStop {
    fn from(reason: &str) -> Self {
        Self::Failed(reason.to_owned())
    }
}

/// What a frame reads of a job.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobState {
    /// Still working: what it is doing, and its work done of its work known (zero known while
    /// it has not measured its work yet).
    Running { step: String, done: u64, total: u64 },
    /// Ended; its outcome is waiting for [`ResultsJob::take_outcome`].
    Finished,
    /// Ended, and its outcome was taken.
    Taken,
}

impl JobState {
    /// The share of its work a running job has done, from 0 to 1; `None` while its work is not
    /// known yet.
    pub fn fraction(&self) -> Option<f32> {
        match self {
            Self::Running { done, total, .. } if *total > 0 => {
                Some(((*done).min(*total) as f64 / *total as f64) as f32)
            }
            _ => None,
        }
    }
}

/// The part of a job its work and its window both reach.
#[derive(Default)]
struct Progress {
    cancel: AtomicBool,
    done: AtomicU64,
    total: AtomicU64,
    step: Mutex<String>,
    held: AtomicBool,
}

struct Shared<T> {
    progress: Arc<Progress>,
    outcome: Mutex<Option<JobOutcome<T>>>,
    finished: AtomicBool,
    taken: AtomicBool,
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// What a job's work is handed: where it states its progress and learns it was cancelled.
pub struct JobContext {
    progress: Arc<Progress>,
}

impl JobContext {
    /// A context no window watches and nothing cancels: work run outside a job.
    pub fn detached() -> Self {
        Self {
            progress: Arc::default(),
        }
    }

    /// Whether the reader cancelled the job.
    pub fn cancelled(&self) -> bool {
        self.progress.cancel.load(Ordering::Acquire)
    }

    /// `Err(JobStop::Cancelled)` once the reader cancelled the job: what work checks between
    /// its pieces.
    pub fn check(&self) -> Result<(), JobStop> {
        if self.cancelled() {
            Err(JobStop::Cancelled)
        } else {
            Ok(())
        }
    }

    /// The flag that turns true when the job is cancelled, for work that takes one.
    pub fn cancel_flag(&self) -> &AtomicBool {
        &self.progress.cancel
    }

    /// Start a step of the work: what it is doing, in the reader's words, and how much work it
    /// is. Work done goes back to zero.
    pub fn step(&self, label: &str, total: u64) {
        *lock(&self.progress.step) = label.to_owned();
        self.progress.done.store(0, Ordering::Relaxed);
        self.progress.total.store(total, Ordering::Relaxed);
    }

    /// The work done of the current step so far.
    pub fn done(&self, done: u64) {
        self.progress.done.store(done, Ordering::Relaxed);
    }
}

/// A Results job: its progress, its cancel, and how it ended.
pub struct ResultsJob<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for ResultsJob<T> {
    fn clone(&self) -> Self {
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> std::fmt::Debug for ResultsJob<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResultsJob")
            .field("state", &self.state())
            .finish_non_exhaustive()
    }
}

/// Two handles are one job.
impl<T> PartialEq for ResultsJob<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.shared, &other.shared)
    }
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static HOLD: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Make jobs started on this thread wait before they work until [`ResultsJob::release`] or a
/// cancel, so a test watches a job that is running however fast its work is.
#[cfg(any(test, feature = "test-support"))]
pub fn hold_jobs_for_tests(hold: bool) {
    HOLD.with(|cell| cell.set(hold));
}

fn held_for_tests() -> bool {
    #[cfg(any(test, feature = "test-support"))]
    {
        HOLD.with(std::cell::Cell::get)
    }
    #[cfg(not(any(test, feature = "test-support")))]
    false
}

impl<T: Send + 'static> ResultsJob<T> {
    /// Start `work` as a job; `name` names its thread.
    pub fn start(
        name: &str,
        work: impl FnOnce(&JobContext) -> Result<T, JobStop> + Send + 'static,
    ) -> Self {
        let progress = Arc::new(Progress::default());
        progress.held.store(held_for_tests(), Ordering::Release);
        let shared = Arc::new(Shared {
            progress,
            outcome: Mutex::new(None),
            finished: AtomicBool::new(false),
            taken: AtomicBool::new(false),
        });
        let job = Self {
            shared: Arc::clone(&shared),
        };
        let worker = Arc::clone(&shared);
        let run = move || {
            let context = JobContext {
                progress: Arc::clone(&worker.progress),
            };
            while worker.progress.held.load(Ordering::Acquire) && !context.cancelled() {
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            let outcome = if context.cancelled() {
                JobOutcome::Cancelled
            } else {
                match work(&context) {
                    Ok(value) => JobOutcome::Done(value),
                    Err(JobStop::Cancelled) => JobOutcome::Cancelled,
                    Err(JobStop::Failed(reason)) => JobOutcome::Failed(reason),
                }
            };
            // A job cancelled at its very end still ends cancelled: the reader was told it would
            // leave nothing, and its value is dropped here, on its own thread.
            let outcome = if context.cancelled() {
                drop(outcome);
                JobOutcome::Cancelled
            } else {
                outcome
            };
            *lock(&worker.outcome) = Some(outcome);
            worker.finished.store(true, Ordering::Release);
        };
        #[cfg(not(target_arch = "wasm32"))]
        {
            let spawned = std::thread::Builder::new()
                .name(format!("rspice-{name}"))
                .spawn(run);
            if let Err(error) = spawned {
                *lock(&shared.outcome) = Some(JobOutcome::Failed(format!(
                    "the work could not start: {error}"
                )));
                shared.finished.store(true, Ordering::Release);
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = name;
            shared.progress.held.store(false, Ordering::Release);
            run();
        }
        job
    }
}

impl<T> ResultsJob<T> {
    /// What a frame shows of the job. Never waits on the job's work.
    pub fn state(&self) -> JobState {
        if self.shared.taken.load(Ordering::Acquire) {
            return JobState::Taken;
        }
        if self.shared.finished.load(Ordering::Acquire) {
            return JobState::Finished;
        }
        let progress = &self.shared.progress;
        JobState::Running {
            step: progress
                .step
                .try_lock()
                .map(|step| step.clone())
                .unwrap_or_default(),
            done: progress.done.load(Ordering::Relaxed),
            total: progress.total.load(Ordering::Relaxed),
        }
    }

    /// Whether the job has ended.
    pub fn is_finished(&self) -> bool {
        self.shared.finished.load(Ordering::Acquire)
    }

    /// How the job ended, once: `None` while it runs and after the outcome was taken.
    pub fn take_outcome(&self) -> Option<JobOutcome<T>> {
        if !self.is_finished() {
            return None;
        }
        let outcome = lock(&self.shared.outcome).take();
        if outcome.is_some() {
            self.shared.taken.store(true, Ordering::Release);
        }
        outcome
    }

    /// Ask the job to stop. Its work stops at its next check, deletes what it wrote, and the job
    /// ends cancelled.
    pub fn cancel(&self) {
        self.shared.progress.cancel.store(true, Ordering::Release);
    }

    /// Whether the job was asked to stop.
    pub fn is_cancelled(&self) -> bool {
        self.shared.progress.cancel.load(Ordering::Acquire)
    }

    /// Let a job held by [`hold_jobs_for_tests`] work.
    #[cfg(any(test, feature = "test-support"))]
    pub fn release(&self) {
        self.shared.progress.held.store(false, Ordering::Release);
    }

    /// Wait, at most `patience`, for the job to end; whether it did. For tests and for quitting,
    /// never for a frame.
    pub fn wait(&self, patience: std::time::Duration) -> bool {
        let until = std::time::Instant::now() + patience;
        while !self.is_finished() {
            if std::time::Instant::now() >= until {
                return false;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        true
    }
}

#[cfg(test)]
mod tests;
