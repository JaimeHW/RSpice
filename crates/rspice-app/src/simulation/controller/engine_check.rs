//! Preflight's run preparation and engine pass, off the UI thread.
//!
//! Preflight prepares the run a press would dispatch — every point's deck
//! generated, its models materialized, its saved outputs bound against a parse
//! of the deck — and then has the engine build each of those decks, without
//! solving, to learn what it refuses there. On a PDK design with a multi-point
//! Run Set each stage costs seconds even in release. Doing them inside the
//! frame that took the press froze the window for as long as they took.
//!
//! The press now only freezes what preparation reads; one worker prepares
//! from that copy and then builds the decks it prepared, in order. The frame
//! loop polls it, the status line states the stage and how far it has got,
//! and Stop cancels either stage. In the browser the worker runs inline, so
//! there the press frame still does both stages, as it did before.
//!
//! The prepared run is retained for dispatch when the pipeline ends, and only
//! when nothing has dropped or replaced the retained run since the press: an
//! edit drops it at its own commit boundary, so a verdict that lands after one
//! describes inputs nobody will dispatch. It is retained with the identity of
//! the inputs frozen at the press, and dispatch runs this very run once that
//! identity still holds; see `run_inputs`.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};

use super::prepared_run::FrozenRunPreparation;
use super::run_inputs::{RetainedRunInputs, RunInputIdentity, SourceFileAnswers};
use super::*;
use crate::simulation::execution::{
    DeckBuildProgress, EngineRefusal, PreparationError, PreparationProgress, PreparedRunMetadata,
    PreparedRunSnapshot,
};

/// Which stage a pipeline is in, and how far it has got.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreflightStage {
    /// Preparing the run. The total is zero until the run's points are known,
    /// and stays zero for a run that is not expanded to any.
    Preparing(PreparationProgress),
    /// The engine building the prepared run's decks.
    EngineChecking(DeckBuildProgress),
}

/// What the worker found.
enum WorkerVerdict {
    NotPrepared(PreparationError),
    Checked {
        snapshot: Box<PreparedRunSnapshot>,
        refusals: Vec<EngineRefusal>,
        source_files: SourceFileAnswers,
    },
}

/// A pipeline in flight.
pub(super) struct PendingPreflight {
    receiver: mpsc::Receiver<WorkerVerdict>,
    cancel: Arc<AtomicBool>,
    stage: Arc<Mutex<PreflightStage>>,
    /// The retained-run generation at the press.
    generation: u64,
    /// The identity of the inputs frozen at the press, which the prepared
    /// run is retained with and its dispatch checks.
    identity: RunInputIdentity,
}

/// How a pipeline ended.
#[derive(Debug)]
pub(crate) struct PreflightOutcome {
    pub(crate) verdict: PreflightVerdict,
    /// Whether nothing dropped or replaced the retained run since the press.
    /// When something did, the verdict describes inputs nobody will dispatch,
    /// and the prepared run was not retained.
    pub(crate) current: bool,
}

#[derive(Debug)]
pub(crate) enum PreflightVerdict {
    /// Preparation refused the run.
    NotPrepared(PreparationError),
    /// The run was prepared, and retained when the outcome is current, and
    /// the engine refused what `refusals` lists.
    Checked {
        metadata: Box<PreparedRunMetadata>,
        refusals: Vec<EngineRefusal>,
    },
    /// The worker ended without a verdict while the engine built the decks.
    NoVerdict,
}

fn set_stage(stage: &Mutex<PreflightStage>, now: PreflightStage) {
    if let Ok(mut stage) = stage.lock() {
        *stage = now;
    }
}

impl SimulationController {
    /// Freeze what preparing the run set reads, and start a worker that
    /// prepares it and then builds its decks. The retained run, and any
    /// pipeline still in flight, are dropped. Fails, starting nothing, when
    /// the configured root cannot be projected.
    pub(crate) fn start_preflight(&mut self, state: &AppState) -> Result<(), PreparationError> {
        self.cancel_preflight();
        self.clear_prepared_run();
        let frozen = FrozenRunPreparation::freeze(state)?;
        let (sender, receiver) = mpsc::channel();
        let cancel = Arc::new(AtomicBool::new(false));
        let stage = Arc::new(Mutex::new(PreflightStage::Preparing(
            PreparationProgress::default(),
        )));
        let pending = PendingPreflight {
            receiver,
            cancel: Arc::clone(&cancel),
            stage: Arc::clone(&stage),
            generation: self.retained_run_generation,
            identity: RunInputIdentity::capture(state),
        };
        #[cfg(test)]
        let hooks = self.preflight_test_hooks.take_for_worker();
        spawn_or_inline(move || {
            #[cfg(test)]
            hooks.enter("preparing");
            let prepared = frozen.prepare(&cancel, &|progress| {
                set_stage(&stage, PreflightStage::Preparing(progress));
            });
            if cancel.load(Ordering::Relaxed) {
                return;
            }
            let verdict = match prepared {
                Err(error) => WorkerVerdict::NotPrepared(error),
                Ok((snapshot, source_files)) => {
                    set_stage(
                        &stage,
                        PreflightStage::EngineChecking(DeckBuildProgress::default()),
                    );
                    #[cfg(test)]
                    hooks.enter("engine checking");
                    let refusals = snapshot.engine_refusals_reporting(&cancel, &|progress| {
                        set_stage(&stage, PreflightStage::EngineChecking(progress));
                    });
                    WorkerVerdict::Checked {
                        snapshot: Box::new(snapshot),
                        refusals,
                        source_files,
                    }
                }
            };
            if !cancel.load(Ordering::Relaxed) {
                let _ = sender.send(verdict);
            }
        });
        self.preflight = Some(pending);
        Ok(())
    }

    /// The stage the pipeline in flight is in, or `None` when none is.
    pub(crate) fn preflight_stage(&self) -> Option<PreflightStage> {
        self.preflight.as_ref().map(|pending| {
            pending.stage.lock().map_or(
                PreflightStage::Preparing(PreparationProgress::default()),
                |stage| *stage,
            )
        })
    }

    /// Stop the pipeline in flight. Its worker stops at the next stage
    /// boundary, deck or abort point, and its verdict is dropped. Returns
    /// whether a pipeline was in flight.
    pub(crate) fn cancel_preflight(&mut self) -> bool {
        let Some(pending) = self.preflight.take() else {
            return false;
        };
        pending.cancel.store(true, Ordering::Relaxed);
        true
    }

    /// The pipeline's verdict, once it has one. `None` while it runs or when
    /// none is in flight. A prepared run is retained here, when nothing has
    /// moved the retained run since the press.
    pub(crate) fn poll_preflight(&mut self) -> Option<PreflightOutcome> {
        let verdict = match self.preflight.as_ref()?.receiver.try_recv() {
            Ok(verdict) => Some(verdict),
            Err(mpsc::TryRecvError::Empty) => return None,
            Err(mpsc::TryRecvError::Disconnected) => None,
        };
        let pending = self.preflight.take()?;
        let current = pending.generation == self.retained_run_generation;
        let verdict = match verdict {
            // The worker ended without sending: a stage it was in failed.
            None => match pending.stage.lock().map(|stage| *stage) {
                Ok(PreflightStage::EngineChecking(_)) => PreflightVerdict::NoVerdict,
                _ => PreflightVerdict::NotPrepared(PreparationError::new(
                    crate::simulation::execution::PreparationStage::Netlist,
                    "Preparing the run ended without a verdict",
                )),
            },
            Some(WorkerVerdict::NotPrepared(error)) => PreflightVerdict::NotPrepared(error),
            Some(WorkerVerdict::Checked {
                snapshot,
                refusals,
                source_files,
            }) => {
                let metadata = Box::new(snapshot.metadata());
                let retained = if current {
                    self.retain_prepared_run(
                        *snapshot,
                        Some(RetainedRunInputs {
                            identity: pending.identity,
                            source_files,
                        }),
                    )
                } else {
                    Ok(())
                };
                match retained {
                    Ok(()) => PreflightVerdict::Checked { metadata, refusals },
                    Err(error) => PreflightVerdict::NotPrepared(error),
                }
            }
        };
        Some(PreflightOutcome { verdict, current })
    }
}

/// Test-only control of the pipeline's worker: hold it before it prepares,
/// and record which thread ran each stage.
#[cfg(test)]
#[derive(Default)]
pub(super) struct PreflightTestHooks {
    gate: Option<mpsc::Receiver<()>>,
    stages: Arc<Mutex<Vec<(&'static str, std::thread::ThreadId)>>>,
}

#[cfg(test)]
pub(super) struct WorkerHooks {
    gate: Option<mpsc::Receiver<()>>,
    stages: Arc<Mutex<Vec<(&'static str, std::thread::ThreadId)>>>,
}

#[cfg(test)]
impl PreflightTestHooks {
    fn take_for_worker(&mut self) -> WorkerHooks {
        WorkerHooks {
            gate: self.gate.take(),
            stages: Arc::clone(&self.stages),
        }
    }
}

#[cfg(test)]
impl WorkerHooks {
    /// Record the stage beginning on this thread. The first stage waits until
    /// the test releases the worker, when it holds it.
    fn enter(&self, stage: &'static str) {
        if let Ok(mut stages) = self.stages.lock() {
            stages.push((stage, std::thread::current().id()));
        }
        if stage == "preparing"
            && let Some(gate) = &self.gate
        {
            let _ = gate.recv();
        }
    }
}

/// The worker owns its hooks, so they drop when it ends, however it ends.
#[cfg(test)]
impl Drop for WorkerHooks {
    fn drop(&mut self) {
        if let Ok(mut stages) = self.stages.lock() {
            stages.push(("finished", std::thread::current().id()));
        }
    }
}

#[cfg(test)]
impl SimulationController {
    /// Hold the next pipeline's worker before it prepares anything, until the
    /// returned sender sends or is dropped.
    pub(crate) fn hold_next_preflight_worker(&mut self) -> mpsc::Sender<()> {
        let (release, gate) = mpsc::channel();
        self.preflight_test_hooks.gate = Some(gate);
        release
    }

    /// Each stage the pipelines ran, with the thread it ran on.
    pub(crate) fn preflight_worker_stages(&self) -> Vec<(&'static str, std::thread::ThreadId)> {
        self.preflight_test_hooks
            .stages
            .lock()
            .map(|stages| stages.clone())
            .unwrap_or_default()
    }
}
