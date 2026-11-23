//! A deck's run, prepared off the frame from the deck as Run took it.
//!
//! Run on a deck prepared the whole run in the frame that took the press:
//! its includes spliced in, the deck parsed and flattened, its models sealed
//! and its tasks built. That froze the window for about a second at a 10 MB
//! deck and six and a half at 60 MB. The press now takes the deck as it
//! stands, its text shared rather than copied, freezes what preparing the run
//! reads, and hands both to a worker. The frame loop polls it, the status bar
//! says the run is preparing, and Stop drops it. The run runs what the press
//! took: an edit made while it prepares is not the run's. Check and netlist
//! prepares the deck the same way and keeps nothing of it but what it found.
//!
//! It comes in three parts. The press,
//! [`SimulationController::start_deck_preparation`], freezes the inputs and
//! starts the worker. The worker, [`prepare_on_worker`], prepares the run and
//! reports a [`WorkerOutcome`], one case for each way a preparation ends. The
//! apply, [`SimulationController::poll_deck_run_preparation`], takes the
//! outcome in a later frame and turns it into the [`DeckRunVerdict`] the
//! frame acts on, retaining a prepared run for dispatch.
//!
//! The worker reports first and frees afterwards: what it built for a run
//! that stopped, or for a check, is dropped once the report is made, so
//! nothing waits on the freeing of a parsed and flattened deck.
//!
//! In the browser, and in the application's tests unless they ask otherwise,
//! the preparation runs inline, [`prepare_on_frame`], so there the press
//! still prepares the run.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, PoisonError, mpsc};

use super::prepared_run::FrozenDeckPreparation;
use super::run_inputs::RetainedRunInputs;
use super::*;
use crate::simulation::ManualDeckCopy;
use crate::simulation::execution::{
    PreparationError, PreparationStage, PreparedRunMetadata, PreparedRunSnapshot,
};
use rspice_simulation::execution::run_memory::MemoryWarning;

/// What a press asks of a deck's preparation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeckAction {
    /// Run: the prepared run is retained for dispatch.
    Run,
    /// Check and netlist: the deck is prepared as Run prepares it, and only
    /// what the preparation found is kept.
    Check,
}

/// What the press hands the worker beside the deck: a step the worker takes
/// before it prepares anything, such as saving the deck's files, which says
/// whether the preparation goes on. A step that holds it has said why
/// itself.
pub type BeforePreparing = Box<dyn FnOnce() -> bool + Send>;

/// What a deck's preparation came to, as the worker reports it: one case
/// for each way it ends, each of which the apply turns into a
/// [`DeckRunVerdict`].
enum WorkerOutcome {
    /// The run, prepared from the deck as the press took it.
    Prepared(Box<PreparedRunSnapshot>),
    /// The run, prepared, needs more memory than the machine has free and
    /// fits what it has installed (`warning`): it waits for the reader to
    /// run it anyway.
    NeedsMemory {
        snapshot: Box<PreparedRunSnapshot>,
        warning: MemoryWarning,
    },
    /// The deck checked: what preparing its run found.
    Checked(Box<PreparedRunMetadata>),
    /// Preparing the run refused the deck.
    Refused(PreparationError),
    /// Preparing it reached a Verilog-A source another thread is compiling,
    /// which a preparation on the frame does not wait for.
    Compiling,
    /// The step before preparing held it.
    Held,
    /// Stop reached the worker.
    Stopped,
}

impl WorkerOutcome {
    /// What `prepared` comes to for `action`, and what is left to free once
    /// it is reported: a check keeps the prepared run's findings alone.
    fn of(
        prepared: Result<PreparedRunSnapshot, PreparationError>,
        action: DeckAction,
    ) -> (Self, Option<PreparedRunSnapshot>) {
        match (prepared, action) {
            (Ok(snapshot), DeckAction::Run) => match snapshot.memory_warning() {
                Some(warning) => (
                    Self::NeedsMemory {
                        snapshot: Box::new(snapshot),
                        warning,
                    },
                    None,
                ),
                None => (Self::Prepared(Box::new(snapshot)), None),
            },
            (Ok(snapshot), DeckAction::Check) => {
                (Self::Checked(Box::new(snapshot.metadata())), Some(snapshot))
            }
            (Err(error), _) => (Self::Refused(error), None),
        }
    }
}

/// The worker's one report, made once, however the worker ends: sent, it
/// asks for the frame that takes it, and releases a test's hold.
struct Report {
    sender: mpsc::Sender<WorkerOutcome>,
    _wake: WakeOnEnd,
    #[cfg(any(test, feature = "test-support"))]
    _hold: Option<mpsc::Sender<()>>,
}

/// The stop signal a worker prepares under. The first poll that finds Stop
/// raised reports the worker stopped, before the preparation unwinds and
/// frees what it built, so Stop is answered within one step of the
/// preparation whatever that step built.
struct ReportingStop {
    cancel: Arc<AtomicBool>,
    report: Mutex<Option<Report>>,
}

impl ReportingStop {
    /// Report `outcome`, unless a report was made.
    fn report(&self, outcome: WorkerOutcome) {
        let report = self
            .report
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        if let Some(report) = report {
            let _ = report.sender.send(outcome);
        }
    }

    fn stopped(&self) -> bool {
        self.cancel.load(Ordering::Relaxed)
    }
}

impl rspice_core::abort_signal::AbortSignal for ReportingStop {
    fn is_aborted(&self) -> bool {
        if !self.stopped() {
            return false;
        }
        self.report(WorkerOutcome::Stopped);
        true
    }
}

/// The worker: take the step before preparing, then prepare the run from
/// what the press froze, stopped once Stop is raised, and report how it
/// ended before anything it built is freed. A constant function or a loop
/// the preparation evaluates polls the same signal, so Stop ends it too. A
/// worker waits for a Verilog-A source another thread compiles.
fn prepare_on_worker(
    frozen: &FrozenDeckPreparation,
    action: DeckAction,
    before: BeforePreparing,
    signal: &ReportingStop,
) {
    if !before() {
        signal.report(WorkerOutcome::Held);
        return;
    }
    if signal.stopped() {
        signal.report(WorkerOutcome::Stopped);
        return;
    }
    let prepared = rspice_core::engine::with_loop_abort(signal, || frozen.prepare(signal));
    let (outcome, left) = WorkerOutcome::of(prepared, action);
    if signal.stopped() {
        signal.report(WorkerOutcome::Stopped);
    } else {
        signal.report(outcome);
    }
    // Freed after the report.
    drop(left);
}

/// The same on the frame, where there is no worker. A frame does not wait
/// for a Verilog-A source another thread compiles: the preparation stops
/// there, and the frame loop asks again once the compile ends.
fn prepare_on_frame(
    frozen: &FrozenDeckPreparation,
    action: DeckAction,
    before: BeforePreparing,
) -> WorkerOutcome {
    if !before() {
        return WorkerOutcome::Held;
    }
    rspice_simulation::project_veriloga::external_runtimes::unless_compiling(|| {
        frozen.prepare(&rspice_core::NoAbort)
    })
    .map_or(WorkerOutcome::Compiling, |prepared| {
        WorkerOutcome::of(prepared, action).0
    })
}

/// A deck's preparation in flight.
pub(super) struct PendingDeckPreparation {
    receiver: mpsc::Receiver<WorkerOutcome>,
    cancel: Arc<AtomicBool>,
    /// The deck as the press took it: what the run runs, and what it is
    /// named by.
    deck: ManualDeckCopy,
}

/// How a deck's preparation ended, as the frame acts on it.
#[derive(Debug)]
pub enum DeckRunVerdict {
    /// The run was prepared from the deck as the press took it, and is
    /// retained for dispatch.
    Prepared(Box<PreparedRunMetadata>),
    /// The run was prepared and is retained, and needs more memory than the
    /// machine has free (`warning`): it dispatches once the reader runs it
    /// anyway.
    NeedsMemory {
        metadata: Box<PreparedRunMetadata>,
        warning: MemoryWarning,
    },
    /// The deck was checked: what preparing its run found. Nothing of it is
    /// retained.
    Checked(Box<PreparedRunMetadata>),
    /// Preparing the run refused the deck.
    Refused(PreparationError),
    /// Preparing it reached a Verilog-A source another thread is compiling.
    Compiling,
    /// The step before preparing held it, and said why.
    Held,
}

static DECK_RUN_WAKER: OnceLock<Box<dyn Fn() + Send + Sync>> = OnceLock::new();

/// Prepare the run of each deck Run is pressed on from now on away from the
/// frame, `waker` asking for the frame that takes its verdict: what the
/// application does as it starts. Where there are no threads, a deck's run
/// is prepared on the frame.
pub fn prepare_deck_runs_away_from_frame(waker: impl Fn() + Send + Sync + 'static) {
    let _ = DECK_RUN_WAKER.set(Box::new(waker));
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Whether the deck runs this thread asks for are prepared away from the
    /// frame.
    static AWAY_HERE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Prepare the deck runs this thread asks for from now on away from the
/// frame, as the application does, with no frame asked for: what a test of
/// the frames meanwhile does.
#[cfg(any(test, feature = "test-support"))]
pub fn prepare_deck_runs_away_here() {
    AWAY_HERE.set(true);
}

/// Asks for a frame as it drops, where the application gave a way to.
struct WakeOnEnd;

impl Drop for WakeOnEnd {
    fn drop(&mut self) {
        if let Some(wake) = DECK_RUN_WAKER.get() {
            wake();
        }
    }
}

/// Whether a deck's run asked for now is prepared away from the frame.
fn prepares_away() -> bool {
    if cfg!(target_arch = "wasm32") {
        return false;
    }
    #[cfg(any(test, feature = "test-support"))]
    if AWAY_HERE.get() {
        return true;
    }
    DECK_RUN_WAKER.get().is_some()
}

/// Free `outcome` away from the frame: a prepared run holds the parsed and
/// flattened deck, which takes a large deck's preparation a good part of a
/// second to free.
fn free_away_from_frame(outcome: WorkerOutcome) {
    match outcome {
        WorkerOutcome::Prepared(snapshot) | WorkerOutcome::NeedsMemory { snapshot, .. } => {
            spawn_or_inline(move || drop(snapshot));
        }
        outcome => drop(outcome),
    }
}

impl SimulationController {
    /// The press: start preparing `deck`, the deck as the press took it, for
    /// `action`, from what preparing it reads of `state`, frozen now;
    /// `before` runs first, where the preparation runs. Any deck preparation
    /// in flight is dropped, and for Run the retained run too. Off the frame
    /// where the application prepares decks away from it; on it otherwise,
    /// where the verdict is in when this returns.
    pub fn start_deck_preparation(
        &mut self,
        state: &impl RunHost,
        deck: ManualDeckCopy,
        action: DeckAction,
        before: BeforePreparing,
    ) {
        self.cancel_deck_run_preparation();
        if action == DeckAction::Run {
            self.clear_prepared_run();
        }
        let frozen = FrozenDeckPreparation::freeze(state, deck.clone());
        let (sender, receiver) = mpsc::channel();
        let cancel = Arc::new(AtomicBool::new(false));
        #[cfg(any(test, feature = "test-support"))]
        let hold = self.deck_preparation_hold.take();
        #[cfg(not(any(test, feature = "test-support")))]
        let hold: Option<()> = None;
        if prepares_away() || hold.is_some() {
            #[cfg(any(test, feature = "test-support"))]
            let (release, hold) = match hold {
                Some(WorkerHold { release, ended }) => (Some(release), Some(ended)),
                None => (None, None),
            };
            let signal = ReportingStop {
                cancel: Arc::clone(&cancel),
                // However the worker ends, even in a panic, the frame that
                // takes its report is asked for.
                report: Mutex::new(Some(Report {
                    sender,
                    _wake: WakeOnEnd,
                    #[cfg(any(test, feature = "test-support"))]
                    _hold: hold,
                })),
            };
            spawn_or_inline(move || {
                #[cfg(any(test, feature = "test-support"))]
                if let Some(release) = &release {
                    let _ = release.recv();
                }
                prepare_on_worker(&frozen, action, before, &signal);
            });
        } else {
            let _ = sender.send(prepare_on_frame(&frozen, action, before));
        }
        self.deck_preparation = Some(PendingDeckPreparation {
            receiver,
            cancel,
            deck,
        });
    }

    /// Whether a deck is being prepared.
    #[must_use]
    pub fn deck_run_preparing(&self) -> bool {
        self.deck_preparation.is_some()
    }

    /// Stop the deck preparation in flight: its worker stops at its next
    /// abort point, and what it prepares is dropped, away from the frame.
    /// Returns whether one was in flight.
    pub fn cancel_deck_run_preparation(&mut self) -> bool {
        let Some(pending) = self.deck_preparation.take() else {
            return false;
        };
        pending.cancel.store(true, Ordering::Relaxed);
        // A run the worker prepared before Stop reached it waits here.
        if let Ok(outcome) = pending.receiver.try_recv() {
            free_away_from_frame(outcome);
        }
        true
    }

    /// The apply: the verdict of the deck preparation in flight, once it
    /// has one; `None` while it prepares, and when none is in flight.
    pub fn poll_deck_run_preparation(&mut self) -> Option<DeckRunVerdict> {
        let outcome = match self.deck_preparation.as_ref()?.receiver.try_recv() {
            Ok(outcome) => outcome,
            Err(mpsc::TryRecvError::Empty) => return None,
            // The worker ended without sending: preparing the deck failed.
            Err(mpsc::TryRecvError::Disconnected) => WorkerOutcome::Refused(PreparationError::new(
                PreparationStage::Netlist,
                "Preparing the run ended without a verdict",
            )),
        };
        let pending = self.deck_preparation.take()?;
        Some(self.apply_deck_preparation(outcome, pending.deck))
    }

    /// What `outcome` comes to for the run of `deck`, the deck as the press
    /// took it: a prepared run is retained here for dispatch, held to that
    /// deck.
    fn apply_deck_preparation(
        &mut self,
        outcome: WorkerOutcome,
        deck: ManualDeckCopy,
    ) -> DeckRunVerdict {
        match outcome {
            WorkerOutcome::Compiling => DeckRunVerdict::Compiling,
            WorkerOutcome::Refused(error) => DeckRunVerdict::Refused(error),
            WorkerOutcome::Held | WorkerOutcome::Stopped => DeckRunVerdict::Held,
            WorkerOutcome::Checked(metadata) => DeckRunVerdict::Checked(metadata),
            WorkerOutcome::Prepared(snapshot) => {
                let metadata = Box::new(snapshot.metadata());
                match self.retain_prepared_run(*snapshot, RetainedRunInputs::Pressed(deck)) {
                    Ok(()) => DeckRunVerdict::Prepared(metadata),
                    Err(error) => DeckRunVerdict::Refused(error),
                }
            }
            WorkerOutcome::NeedsMemory { snapshot, warning } => {
                let metadata = Box::new(snapshot.metadata());
                match self.retain_prepared_run(*snapshot, RetainedRunInputs::Pressed(deck)) {
                    Ok(()) => DeckRunVerdict::NeedsMemory { metadata, warning },
                    Err(error) => DeckRunVerdict::Refused(error),
                }
            }
        }
    }
}

/// A test's hold on the next deck preparation's worker.
#[cfg(any(test, feature = "test-support"))]
pub struct DeckPreparationHold {
    /// Sending on it, or dropping it, lets the worker prepare.
    pub release: mpsc::Sender<()>,
    /// Disconnects once the worker has reported how it ended, however it
    /// ended.
    pub ended: mpsc::Receiver<()>,
}

/// The worker's side of a [`DeckPreparationHold`].
#[cfg(any(test, feature = "test-support"))]
pub(super) struct WorkerHold {
    release: mpsc::Receiver<()>,
    /// Dropped with the worker's report.
    ended: mpsc::Sender<()>,
}

#[cfg(any(test, feature = "test-support"))]
impl SimulationController {
    /// Hold the next deck preparation's worker, away from the frame, before
    /// it prepares anything, until the hold releases it.
    pub fn hold_next_deck_run_preparation(&mut self) -> DeckPreparationHold {
        let (release, released) = mpsc::channel();
        let (ended, worker_ended) = mpsc::channel();
        self.deck_preparation_hold = Some(WorkerHold {
            release: released,
            ended,
        });
        DeckPreparationHold {
            release,
            ended: worker_ended,
        }
    }
}
