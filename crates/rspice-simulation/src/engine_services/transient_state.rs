//! One small-signal card linearized around one transient state.
//!
//! A task whose card carries `ACTIMES=` or `CHECKPOINT=` holds its parent's
//! `.TRAN` card and its own (`execution::headless_tasks`). Whichever runner
//! serves the task's analysis — the engine bridge for the configured kinds,
//! the services for the rest — solves the card here, through the engine's
//! state entry, and projects the one result as it projects a run at the
//! operating point.
//!
//! An `ACTIMES=` card bound to its `.TRAN` task (`execution::snapshot::
//! bound_cards`) does not run the transient again: the task's run landed on
//! the card's time and captured the state there into the runner's
//! [`TransientStateStore`], and the card linearizes around that. A card that
//! finds no state there — its transient ran on another runner, or not at all
//! — runs its own, as an unbound card does.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::{Engine, StateLinearizedResult, TransientCheckpoint, TransientState};
use rspice_core::netlist::{AnalysisCommand, Netlist, SmallSignalStateSource};
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_simulation_contract::config::AnalysisConfig;
use rspice_vocabulary::product::{AnalysisInstanceId, ContentDigest};

use super::ServiceRunError;
use crate::execution::BoundStateReader;

/// States a `.TRAN` task's run captured for the state cards bound to it,
/// waiting for their tasks.
///
/// One per runner. A study runs its tasks one after another on one runner,
/// each transient before the cards that depend on it, so a card's task finds
/// the state its transient captured for it here, and takes it. A state is
/// read only by a task of the prepared run whose transient captured it, and
/// the store holds one prepared run's states at a time.
#[derive(Debug, Default)]
pub(crate) struct TransientStateStore {
    states: Mutex<HashMap<StateKey, (f64, TransientState)>>,
}

/// The prepared run, the transient that captured the state, and the task
/// that reads it.
type StateKey = (ContentDigest, AnalysisInstanceId, AnalysisInstanceId);

impl TransientStateStore {
    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<StateKey, (f64, TransientState)>> {
        self.states
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

/// The part a task plays in the hand-off: the prepared run it belongs to,
/// the task itself, the transient whose captured state it reads, and the
/// tasks it captures states for.
#[derive(Debug, Clone, Default)]
pub(crate) struct TransientStateRole {
    run: Option<ContentDigest>,
    task: Option<AnalysisInstanceId>,
    transient: Option<AnalysisInstanceId>,
    readers: Vec<BoundStateReader>,
}

impl TransientStateRole {
    pub(crate) const fn new(
        run: ContentDigest,
        task: AnalysisInstanceId,
        transient: Option<AnalysisInstanceId>,
        readers: Vec<BoundStateReader>,
    ) -> Self {
        Self {
            run: Some(run),
            task: Some(task),
            transient,
            readers,
        }
    }
}

/// A task's role, and the store its runner keeps, for the run on this thread.
#[derive(Debug)]
pub(crate) struct TransientStateHandoff {
    store: Arc<TransientStateStore>,
    role: TransientStateRole,
}

impl TransientStateHandoff {
    pub(crate) const fn new(store: Arc<TransientStateStore>, role: TransientStateRole) -> Self {
        Self { store, role }
    }
}

thread_local! {
    /// The hand-off of the run executing on this thread. The bridge and the
    /// services solve a task on the thread the runner started it on.
    static HANDOFF: RefCell<Option<TransientStateHandoff>> = const { RefCell::new(None) };
}

/// Install `handoff` for the run on this thread until the guard drops.
#[must_use]
pub(crate) fn install_transient_state_handoff(
    handoff: TransientStateHandoff,
) -> TransientStateHandoffGuard {
    let previous = HANDOFF.with(|slot| slot.replace(Some(handoff)));
    TransientStateHandoffGuard { previous }
}

/// What removes a run's hand-off, on every exit path.
pub(crate) struct TransientStateHandoffGuard {
    previous: Option<TransientStateHandoff>,
}

impl Drop for TransientStateHandoffGuard {
    fn drop(&mut self) {
        let previous = self.previous.take();
        HANDOFF.with(|slot| {
            slot.replace(previous);
        });
    }
}

/// The times a `.TRAN` deck's state cards name, which its run lands on and
/// captures: in order, once each, and inside the run.
pub(crate) fn state_capture_times(netlist: &Netlist, tstop: f64) -> Vec<f64> {
    let mut times = netlist
        .small_signal_states
        .iter()
        .filter_map(|request| match &request.source {
            SmallSignalStateSource::TransientTimes(times) => Some(times.iter().copied()),
            SmallSignalStateSource::CheckpointFile(_) => None,
        })
        .flatten()
        .filter(|time| time.is_finite() && (0.0..=tstop).contains(time))
        .collect::<Vec<_>>();
    times.sort_by(f64::total_cmp);
    times.dedup();
    times
}

/// Hand the states a transient captured at `times` to the tasks bound to it,
/// through the store of the run on this thread. A run with no store, or no
/// readers, drops them; so does the store, for every state another prepared
/// run left unread.
pub(crate) fn publish_transient_states(times: &[f64], states: Vec<TransientState>) {
    HANDOFF.with(|slot| {
        let slot = slot.borrow();
        let Some(TransientStateHandoff { store, role }) = slot.as_ref() else {
            return;
        };
        let (Some(run), Some(transient)) = (role.run, role.task) else {
            return;
        };
        let mut held = store.lock();
        held.retain(|(held_run, held_transient, _), _| {
            *held_run == run && *held_transient != transient
        });
        for reader in &role.readers {
            if let Some(index) = times
                .iter()
                .position(|time| time.to_bits() == reader.time_s.to_bits())
                && let Some(state) = states.get(index)
            {
                held.insert(
                    (run, transient, reader.reader),
                    (reader.time_s, state.clone()),
                );
            }
        }
    });
}

/// The state the transient this task reads captured for it at `time`.
fn take_transient_state(time: f64) -> Option<TransientState> {
    HANDOFF.with(|slot| {
        let slot = slot.borrow();
        let TransientStateHandoff { store, role } = slot.as_ref()?;
        let key = (role.run?, role.transient?, role.task?);
        let mut held = store.lock();
        match held.get(&key) {
            Some((captured, _)) if captured.to_bits() == time.to_bits() => {
                held.remove(&key).map(|(_, state)| state)
            }
            _ => None,
        }
    })
}

/// Where a small-signal service run linearizes the circuit.
#[derive(Clone, Copy)]
pub(crate) enum Bias<'a> {
    /// The deck's operating point.
    OperatingPoint,
    /// The one transient state the deck's card names: `checkpoint` is the
    /// image a `CHECKPOINT=` card names, and `None` for `ACTIMES=`.
    TransientState {
        checkpoint: Option<&'a TransientCheckpoint>,
    },
}

/// The analysis of a task around a transient state: the card it runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StateCard {
    Ac,
    Noise,
    TransferFunction,
    SParameters,
    Stability,
    PoleZero,
    Sensitivity,
    Distortion,
}

impl StateCard {
    /// The card a typed request runs around a state, when it is one.
    pub(crate) const fn of_spec(spec: &AnalysisSpec) -> Option<Self> {
        Some(match spec {
            AnalysisSpec::Ac { .. } => Self::Ac,
            AnalysisSpec::Noise { .. } => Self::Noise,
            AnalysisSpec::Tf { .. } => Self::TransferFunction,
            AnalysisSpec::SParameter { .. } => Self::SParameters,
            AnalysisSpec::Stb { .. } => Self::Stability,
            AnalysisSpec::PoleZero { .. } => Self::PoleZero,
            AnalysisSpec::Sensitivity { .. } => Self::Sensitivity,
            AnalysisSpec::Disto { .. } => Self::Distortion,
            _ => return None,
        })
    }

    /// The same for a configured request, which the engine bridge serves.
    pub(crate) const fn of_config(config: &AnalysisConfig) -> Option<Self> {
        Some(match config {
            AnalysisConfig::Ac(_) => Self::Ac,
            AnalysisConfig::Noise(_) => Self::Noise,
            AnalysisConfig::PoleZero(_) => Self::PoleZero,
            AnalysisConfig::Sensitivity(_) => Self::Sensitivity,
            AnalysisConfig::DcOp(_) | AnalysisConfig::DcSweep(_) | AnalysisConfig::Transient(_) => {
                return None;
            }
        })
    }

    const fn runs(self, card: &AnalysisCommand) -> bool {
        matches!(
            (self, card),
            (Self::Ac, AnalysisCommand::Ac { .. })
                | (Self::Noise, AnalysisCommand::Noise { .. })
                | (Self::TransferFunction, AnalysisCommand::Tf { .. })
                | (Self::SParameters, AnalysisCommand::Sp { .. })
                | (Self::Stability, AnalysisCommand::Stb { .. })
                | (Self::PoleZero, AnalysisCommand::PoleZero { .. })
                | (Self::Sensitivity, AnalysisCommand::Sensitivity { .. })
                | (Self::Distortion, AnalysisCommand::Disto { .. })
        )
    }
}

/// Why a card around a state did not run.
pub(crate) enum StateRunError {
    /// The engine's own refusal or failure, for the caller to translate as it
    /// translates the engine's errors at the operating point.
    Engine(rspice_core::SimulationError),
    /// The task is not one card around one state.
    Refused(String),
}

impl StateRunError {
    /// The services' form, with `context` naming the analysis.
    pub(crate) fn into_service(self, context: &str) -> ServiceRunError {
        match self {
            Self::Engine(error) => ServiceRunError::from_core(context, error),
            Self::Refused(message) => ServiceRunError::Failure(message),
        }
    }
}

/// Run `netlist`'s one `card` around the one state it names, on `engine`.
///
/// The deck's transient runs to the card's one time, or `checkpoint` — the
/// image a `CHECKPOINT=` card names — is restored; then the card runs there
/// through its own entry point.
pub(crate) fn linearize_at_state(
    engine: &Engine,
    netlist: &Netlist,
    card: StateCard,
    checkpoint: Option<&TransientCheckpoint>,
    abort: &dyn AbortSignal,
) -> Result<StateLinearizedResult, StateRunError> {
    let refuse = |message: &str| {
        Err(StateRunError::Refused(format!(
            "A small-signal task around a transient state {message}"
        )))
    };
    if abort.is_aborted() {
        return Err(StateRunError::Engine(rspice_core::SimulationError::Aborted));
    }
    let [request] = netlist.small_signal_states.as_slice() else {
        return refuse("holds one card around one state");
    };
    let index = request.analysis_index;
    if !netlist
        .analyses
        .get(index)
        .is_some_and(|analysis| card.runs(analysis))
    {
        return refuse("runs the card its analysis names");
    }
    match (&request.source, checkpoint) {
        (SmallSignalStateSource::TransientTimes(times), None) => {
            let [time] = times.as_slice() else {
                return refuse("is one state, at one time");
            };
            // The state the card's `.TRAN` task captured for it, when its run
            // served the card: the same transient, landed on the same time.
            if card != StateCard::Sensitivity
                && let Some(state) = take_transient_state(*time)
            {
                return engine
                    .run_small_signal_at_transient_state(netlist, index, &state, abort)
                    .map_err(StateRunError::Engine);
            }
            let mut states = engine
                .run_small_signal_at_transient_times(netlist, index, &[*time], abort)
                .map_err(StateRunError::Engine)?;
            match (states.pop(), states.is_empty()) {
                (Some(state), true) => Ok(state),
                _ => refuse("publishes one result"),
            }
        }
        (SmallSignalStateSource::CheckpointFile(_), Some(checkpoint)) => engine
            .run_small_signal_at_checkpoint(netlist, index, checkpoint, abort)
            .map_err(StateRunError::Engine),
        (SmallSignalStateSource::CheckpointFile(name), None) => refuse(&format!(
            "carries the image of its checkpoint file {name}, and this one carries none"
        )),
        (SmallSignalStateSource::TransientTimes(_), Some(_)) => {
            refuse("names its times or a checkpoint, not both")
        }
    }
}
