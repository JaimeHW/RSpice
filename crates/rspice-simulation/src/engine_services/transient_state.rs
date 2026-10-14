//! One small-signal card linearized around one transient state.
//!
//! A task whose card carries `ACTIMES=` or `CHECKPOINT=` holds its parent's
//! `.TRAN` card and its own (`execution::headless_tasks`). Whichever runner
//! serves the task's analysis — the engine bridge for the configured kinds,
//! the services for the rest — solves the card here, through the engine's
//! state entry, and projects the one result as it projects a run at the
//! operating point.

use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::{Engine, StateLinearizedResult, TransientCheckpoint};
use rspice_core::netlist::{AnalysisCommand, Netlist, SmallSignalStateSource};
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_simulation_contract::config::AnalysisConfig;

use super::ServiceRunError;

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
