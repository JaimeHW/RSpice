//! A deck's run, prepared off the frame from the deck as Run took it.
//!
//! Run on a deck prepared the whole run in the frame that took the press:
//! its includes spliced in, the deck parsed and flattened, its models sealed
//! and its tasks built. That froze the window for about a second at a 10 MB
//! deck and six and a half at 60 MB. The press now takes the deck as it
//! stands, its text shared rather than copied, freezes what preparing the run
//! reads, and hands both to a worker. The frame loop polls it, the status bar
//! says the run is preparing, and Stop drops it. The run runs what the press
//! took: an edit made while it prepares is not the run's.
//!
//! It comes in three parts. The press,
//! [`SimulationController::start_deck_run_preparation`], freezes the inputs
//! and starts the worker. The worker, [`prepare_on_worker`], prepares the run
//! and sends a [`WorkerOutcome`], one case for each way a preparation ends.
//! The apply, [`SimulationController::poll_deck_run_preparation`], takes the
//! outcome in a later frame and turns it into the [`DeckRunVerdict`] the
//! frame acts on, retaining a prepared run for dispatch.
//!
//! In the browser, and in the application's tests unless they ask otherwise,
//! the preparation runs inline, [`prepare_on_frame`], so there the press
//! still prepares the run.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock, mpsc};

use super::prepared_run::FrozenDeckPreparation;
use super::run_inputs::RetainedRunInputs;
use super::*;
use crate::simulation::ManualDeckCopy;
use crate::simulation::execution::{
    PreparationError, PreparationStage, PreparedRunMetadata, PreparedRunSnapshot,
};

/// What a deck's preparation came to, as the worker sends it: one case for
/// each way it ends, each of which the apply turns into a
/// [`DeckRunVerdict`].
enum WorkerOutcome {
    /// The run, prepared from the deck as the press took it.
    Prepared(Box<PreparedRunSnapshot>),
    /// Preparing the run refused the deck.
    Refused(PreparationError),
    /// Preparing it reached a Verilog-A source another thread is compiling,
    /// which a preparation on the frame does not wait for.
    Compiling,
}

impl WorkerOutcome {
    fn of(prepared: Result<PreparedRunSnapshot, PreparationError>) -> Self {
        match prepared {
            Ok(snapshot) => Self::Prepared(Box::new(snapshot)),
            Err(error) => Self::Refused(error),
        }
    }
}

/// The worker: prepare the run from what the press froze, stopped once
/// `cancel` is raised. A constant function or a loop the preparation
/// evaluates polls the same flag, so Stop ends it too. A worker waits for a
/// Verilog-A source another thread compiles.
fn prepare_on_worker(frozen: &FrozenDeckPreparation, cancel: &Arc<AtomicBool>) -> WorkerOutcome {
    WorkerOutcome::of(rspice_core::engine::with_loop_abort(cancel, || {
        frozen.prepare(cancel)
    }))
}

/// The same on the frame, where there is no worker. A frame does not wait
/// for a Verilog-A source another thread compiles: the preparation stops
/// there, and the frame loop asks again once the compile ends.
fn prepare_on_frame(frozen: &FrozenDeckPreparation) -> WorkerOutcome {
    rspice_simulation::project_veriloga::external_runtimes::unless_compiling(|| {
        frozen.prepare(&rspice_core::NoAbort)
    })
    .map_or(WorkerOutcome::Compiling, WorkerOutcome::of)
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
    /// Preparing the run refused the deck.
    Refused(PreparationError),
    /// Preparing it reached a Verilog-A source another thread is compiling.
    Compiling,
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

impl SimulationController {
    /// The press: start preparing the run of `deck`, the deck as a Run press
    /// took it, from what preparing it reads of `state`, frozen now. The retained run,
    /// and any deck preparation in flight, are dropped. Off the frame where
    /// the application prepares decks away from it; on it otherwise, where
    /// the verdict is in when this returns.
    pub fn start_deck_run_preparation(&mut self, state: &impl RunHost, deck: ManualDeckCopy) {
        self.cancel_deck_run_preparation();
        self.clear_prepared_run();
        let frozen = FrozenDeckPreparation::freeze(state, deck.clone());
        let (sender, receiver) = mpsc::channel();
        let cancel = Arc::new(AtomicBool::new(false));
        #[cfg(any(test, feature = "test-support"))]
        let hold = self.deck_preparation_hold.take();
        #[cfg(not(any(test, feature = "test-support")))]
        let hold: Option<()> = None;
        if prepares_away() || hold.is_some() {
            let cancel = Arc::clone(&cancel);
            spawn_or_inline(move || {
                // However the worker ends, even in a panic, the frame that
                // takes its verdict is asked for.
                let _wake = WakeOnEnd;
                #[cfg(any(test, feature = "test-support"))]
                if let Some(hold) = &hold {
                    let _ = hold.release.recv();
                }
                let outcome = prepare_on_worker(&frozen, &cancel);
                if !cancel.load(Ordering::Relaxed) {
                    let _ = sender.send(outcome);
                }
            });
        } else {
            let _ = sender.send(prepare_on_frame(&frozen));
        }
        self.deck_preparation = Some(PendingDeckPreparation {
            receiver,
            cancel,
            deck,
        });
    }

    /// Whether a deck's run is being prepared.
    #[must_use]
    pub fn deck_run_preparing(&self) -> bool {
        self.deck_preparation.is_some()
    }

    /// Stop the deck preparation in flight: its worker stops at its next
    /// abort point, and what it prepares is dropped. Returns whether one was
    /// in flight.
    pub fn cancel_deck_run_preparation(&mut self) -> bool {
        let Some(pending) = self.deck_preparation.take() else {
            return false;
        };
        pending.cancel.store(true, Ordering::Relaxed);
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
            WorkerOutcome::Prepared(snapshot) => {
                let metadata = Box::new(snapshot.metadata());
                match self.retain_prepared_run(*snapshot, RetainedRunInputs::Pressed(deck)) {
                    Ok(()) => DeckRunVerdict::Prepared(metadata),
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
    /// Disconnects once the worker has ended, however it ended.
    pub ended: mpsc::Receiver<()>,
}

/// The worker's side of a [`DeckPreparationHold`].
#[cfg(any(test, feature = "test-support"))]
pub(super) struct WorkerHold {
    release: mpsc::Receiver<()>,
    /// Dropped with the worker.
    _ended: mpsc::Sender<()>,
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
            _ended: ended,
        });
        DeckPreparationHold {
            release,
            ended: worker_ended,
        }
    }
}
