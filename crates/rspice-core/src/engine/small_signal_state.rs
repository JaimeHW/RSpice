//! Small-signal analyses around an accepted transient state (design S10).
//!
//! `.AC`, `.NOISE`, `.TF`, `.SP`, `.STB`, `.PZ`, `.SENS` and `.DISTO` linearize
//! around the operating point unless the card says otherwise. With
//! `ACTIMES=t1[,t2,…]` (Spectre's `actimes`) the card linearizes around the
//! accepted state of the deck's own `.TRAN` run at each requested time, as a
//! frozen-time operating point: every accepted analog state at `T` — charges,
//! operator states, `idt` values — the source values at `T`, and the accepted
//! discrete state, frozen (S2). Pending activations are not run.
//!
//! The state is captured live (`transient::state_capture`): a clone of the
//! circuit at the accepted point, so no device has to know how to serialize
//! itself and none of the checkpoint blockers apply. The card's analysis then
//! runs through its ordinary entry point on an engine that holds the state,
//! and the one shared bias preparation adopts the captured circuit instead of
//! solving and committing an operating point.
//!
//! The analysis builds its circuit as the transient did — for the same stop
//! time, so with the transient's own topology — and the adopted state must be
//! the same elaboration: a run whose analysis circuit differs from the
//! transient's is refused, never silently linearized around the wrong state.

use std::sync::Arc;

use super::small_signal_bias::SmallSignalBias;
use super::transient::state_capture::CapturedTransientState;
use super::{
    CapabilityAnalysis, Engine, SParameterRun, SensitivityCardResult, SimulationError,
    StbAnalysisResult, TransientStartupMode,
};
use crate::abort_signal::{AbortSignal, ModelRunSignal};
use crate::analysis::{
    AcResult, DistortionAnalysisResult, NoiseResult, PoleZeroResult, TransferFunctionResult,
};
use crate::netlist::AnalysisCommand;
use crate::{CircuitData, Netlist, Value};

/// What one small-signal card produced, in its family's own result type.
#[derive(Debug, Clone)]
pub enum SmallSignalCardResult {
    /// `.AC`: one point per frequency.
    Ac(Vec<AcResult>),
    /// `.NOISE`: one point per frequency.
    Noise(Vec<NoiseResult>),
    /// `.TF`.
    TransferFunction(TransferFunctionResult),
    /// `.SP`.
    SParameters(Box<SParameterRun>),
    /// `.STB`.
    Stability(Box<StbAnalysisResult>),
    /// `.PZ`.
    PoleZero(PoleZeroResult),
    /// `.SENS … AC`.
    Sensitivity(Box<SensitivityCardResult>),
    /// `.DISTO`.
    Distortion(Box<DistortionAnalysisResult>),
}

/// One small-signal card's result around the accepted transient state at one
/// time.
#[derive(Debug, Clone)]
pub struct StateLinearizedResult {
    /// The accepted transient time the card linearized at, in seconds.
    pub state_time: Value,
    /// The card's result around that state.
    pub result: SmallSignalCardResult,
}

/// The `.TRAN` window a state's transient runs in.
#[derive(Clone, Copy, Debug)]
pub(in crate::engine) struct StateTransientWindow {
    pub(in crate::engine) tstop: Value,
    pub(in crate::engine) max_step: Value,
    pub(in crate::engine) startup_mode: TransientStartupMode,
}

impl StateTransientWindow {
    /// The window of the deck's first `.TRAN` card, resolved as a run of that
    /// card resolves it, so the state transient is that card's trajectory.
    pub(in crate::engine) fn of_first_transient(
        netlist: &Netlist,
    ) -> Result<Option<Self>, SimulationError> {
        let Some((step, stop, start, max_step, uic)) =
            netlist.analyses.iter().find_map(|analysis| match analysis {
                AnalysisCommand::Tran {
                    step,
                    stop,
                    start,
                    max_step,
                    uic,
                } => Some((*step, *stop, *start, *max_step, *uic)),
                _ => None,
            })
        else {
            return Ok(None);
        };
        let max_step =
            crate::analysis::transient::resolve_transient_maximum_step(step, stop, start, max_step)
                .map_err(|error| {
                    SimulationError::Netlist(format!(
                        "the .TRAN card a small-signal state is taken from is invalid: {error}"
                    ))
                })?;
        Ok(Some(Self {
            tstop: stop,
            max_step,
            startup_mode: TransientStartupMode::from_uic(uic),
        }))
    }
}

/// The transient state every small-signal analysis on one engine linearizes
/// around.
#[derive(Debug)]
pub(in crate::engine) struct TransientStateBias {
    window: StateTransientWindow,
    state: HeldTransientState,
}

#[derive(Debug)]
enum HeldTransientState {
    /// A state captured at an accepted point. Each analysis adopts a clone.
    ///
    /// Behind a lock because an engine is shared across worker threads and a
    /// circuit's device caches are not: the lock is taken once, to clone.
    Captured(std::sync::Mutex<CapturedTransientState>),
    /// Run the state transient to this time on whichever netlist the analysis
    /// prepares. A sensitivity perturbs the deck, and its state moves with
    /// the perturbation, so every perturbed solve reruns the transient.
    Rerun(Value),
}

impl TransientStateBias {
    /// The stop time the state's circuit was built for. Every circuit an
    /// engine holding this state builds is elaborated for the same stop time,
    /// so it has the transient's own topology.
    pub(in crate::engine) fn transient_stop(&self) -> Value {
        self.window.tstop
    }
}

/// The cards that linearize, and the analysis each refuses under.
fn small_signal_card(
    netlist: &Netlist,
    analysis_index: usize,
) -> Result<&AnalysisCommand, SimulationError> {
    let card = netlist.analyses.get(analysis_index).ok_or_else(|| {
        SimulationError::Netlist(format!(
            "analysis {analysis_index} does not exist; the deck has {} analysis cards",
            netlist.analyses.len()
        ))
    })?;
    match card {
        AnalysisCommand::Ac { .. }
        | AnalysisCommand::Noise { .. }
        | AnalysisCommand::Tf { .. }
        | AnalysisCommand::Sp { .. }
        | AnalysisCommand::Stb { .. }
        | AnalysisCommand::PoleZero { .. }
        | AnalysisCommand::Sensitivity { .. }
        | AnalysisCommand::Disto { .. } => Ok(card),
        other => Err(SimulationError::Netlist(format!(
            "only .AC, .NOISE, .TF, .SP, .STB, .PZ, .SENS and .DISTO linearize around a \
             transient state; analysis {analysis_index} is {}",
            card_name(other)
        ))),
    }
}

fn card_name(card: &AnalysisCommand) -> &'static str {
    match card {
        AnalysisCommand::Op => ".OP",
        AnalysisCommand::Dc { .. } => ".DC",
        AnalysisCommand::Ac { .. } => ".AC",
        AnalysisCommand::AcData { .. } => ".AC DATA",
        AnalysisCommand::Tran { .. } => ".TRAN",
        AnalysisCommand::Noise { .. } => ".NOISE",
        AnalysisCommand::NoiseData { .. } => ".NOISE DATA",
        AnalysisCommand::Tf { .. } => ".TF",
        AnalysisCommand::Sp { .. } => ".SP",
        AnalysisCommand::Stb { .. } => ".STB",
        AnalysisCommand::PoleZero { .. } => ".PZ",
        AnalysisCommand::Sensitivity { .. } => ".SENS",
        AnalysisCommand::Disto { .. } => ".DISTO",
        _ => "not a small-signal card",
    }
}

/// Whether two elaborations of one deck are the same system: the same
/// unknowns, named the same way, in the same order.
fn ensure_same_elaboration(
    analysis_circuit: &CircuitData,
    state: &CapturedTransientState,
    analysis: CapabilityAnalysis,
) -> Result<(), SimulationError> {
    let held = &state.circuit;
    if analysis_circuit.matrix_size() == held.matrix_size()
        && analysis_circuit.node_names_sorted() == held.node_names_sorted()
        && analysis_circuit.branch_names_sorted() == held.branch_names_sorted()
    {
        return Ok(());
    }
    Err(SimulationError::Circuit(format!(
        "{} cannot linearize around the transient state at t={:e} s: the circuit it \
         elaborates has {} unknowns and the transient's has {}, so the state is not a state \
         of this circuit",
        analysis.label(),
        state.time,
        analysis_circuit.matrix_size(),
        held.matrix_size()
    )))
}

impl Engine {
    /// A per-run copy of this engine that linearizes every small-signal
    /// analysis around `state`. [`Self::run_small_signal_card`] consumes it,
    /// so the state never outlives the one card it was built for.
    fn at_transient_state(&self, netlist: &Netlist, state: TransientStateBias) -> Engine {
        let mut engine = self.resolved_for_netlist(netlist);
        engine.small_signal_state = Some(Arc::new(state));
        engine
    }

    /// Run the state transient of `window` on `netlist` and capture its
    /// accepted state at each of `times`, on an engine that holds no state of
    /// its own.
    fn capture_transient_states(
        &self,
        netlist: &Netlist,
        window: StateTransientWindow,
        times: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<CapturedTransientState>, SimulationError> {
        let mut engine = self.resolved_for_netlist(netlist);
        engine.small_signal_state = None;
        engine
            .run_tran_capturing_states(
                netlist,
                window.tstop,
                window.max_step,
                window.startup_mode,
                times,
                abort,
            )
            .map(|(_, captured)| captured)
    }

    /// Prepare the bias `analysis` linearizes around when this engine holds a
    /// transient state (S10).
    ///
    /// The captured circuit and solution are adopted as they were accepted:
    /// there is no DC solve and no equilibrium commit, because the state is
    /// already accepted, and no Verilog-A analysis begins, because beginning
    /// one would reinitialize the very state being linearized around. The
    /// device refusals, the threshold findings, and the device caches aligned
    /// with the bias are the operating point's.
    pub(in crate::engine) fn prepare_small_signal_bias_at_state(
        &self,
        netlist: &Netlist,
        circuit: Option<CircuitData>,
        analysis: CapabilityAnalysis,
        physical: u8,
        state: &TransientStateBias,
        abort: &dyn AbortSignal,
    ) -> Result<SmallSignalBias, SimulationError> {
        let run_scope = ModelRunSignal::if_needed(abort);
        let abort: &dyn AbortSignal = run_scope.as_ref().map_or(abort, |scope| scope);
        let captured = match &state.state {
            HeldTransientState::Captured(captured) => captured
                .lock()
                .map_err(|_| {
                    SimulationError::Circuit(
                        "an analysis panicked while it held the transient state".to_owned(),
                    )
                })?
                .clone(),
            HeldTransientState::Rerun(time) => self
                .capture_transient_states(netlist, state.window, &[*time], abort)?
                .pop()
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "the transient captured no state at t={time:e} s"
                    ))
                })?,
        };
        if let Some(analysis_circuit) = &circuit {
            ensure_same_elaboration(analysis_circuit, &captured, analysis)?;
        }
        drop(circuit);
        let CapturedTransientState {
            time: state_time,
            circuit: mut adopted,
            solution: bias,
        } = captured;
        Self::ensure_analysis_capability(&adopted, analysis)?;
        Self::ensure_small_signal_devices(&adopted, analysis, physical)?;
        let matrix = if adopted.matrix_size() == 0 {
            Self::model_observation_matrix()?
        } else {
            self.build_matrix(&adopted)?
        };
        adopted.link_indices(&matrix);
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        #[cfg(feature = "veriloga")]
        for finding in self.held_threshold_findings(&adopted, &bias) {
            log::warn!(
                "[mixed.threshold] {} at t={state_time:e} s: {finding}",
                analysis.label(),
            );
        }
        #[cfg(not(feature = "veriloga"))]
        let _ = state_time;
        adopted
            .finish_veriloga_equilibrium_operating_point(physical)
            .map_err(SimulationError::Circuit)?;
        adopted.refresh_jiles_atherton_inductances(&bias);
        Self::prepare_small_signal_state(&mut adopted, &bias)?;
        Ok(SmallSignalBias {
            circuit: adopted,
            matrix,
            bias,
        })
    }

    /// Run the small-signal card at `analysis_index` around the accepted
    /// state of the deck's first `.TRAN` card at each of `times`.
    ///
    /// The transient runs once, to the last time, landing on every requested
    /// time; the card then runs once per time around the state accepted
    /// there. A `.SENS … AC` card reruns the state transient for every
    /// perturbation instead, because the state moves with the perturbed
    /// parameter. A DC `.SENS` card is refused: the operating-point tangents
    /// it reports are not derivatives of a transient state.
    ///
    /// Called by the command line and the simulation service for cards that
    /// carry `ACTIMES=`.
    pub fn run_small_signal_at_transient_times(
        &self,
        netlist: &Netlist,
        analysis_index: usize,
        times: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<StateLinearizedResult>, SimulationError> {
        let card = small_signal_card(netlist, analysis_index)?;
        let window = StateTransientWindow::of_first_transient(netlist)?.ok_or_else(|| {
            SimulationError::Netlist(
                "ACTIMES= linearizes around the state the deck's .TRAN card reaches, and the \
                 deck has no .TRAN card"
                    .to_owned(),
            )
        })?;
        if let AnalysisCommand::Sensitivity { ac_sweep, .. } = card {
            if ac_sweep.is_none() {
                return Err(dc_sensitivity_at_state_refusal());
            }
            // Validate the request as the capture would, before any rerun.
            super::transient::state_capture::TransientStateCapture::new(times, window.tstop)?;
            return times
                .iter()
                .map(|&time| {
                    let engine = self.at_transient_state(
                        netlist,
                        TransientStateBias {
                            window,
                            state: HeldTransientState::Rerun(time),
                        },
                    );
                    Ok(StateLinearizedResult {
                        state_time: time,
                        result: engine.run_small_signal_card(netlist, card, abort)?,
                    })
                })
                .collect();
        }
        self.capture_transient_states(netlist, window, times, abort)?
            .into_iter()
            .map(|captured| {
                let state_time = captured.time;
                let engine = self.at_transient_state(
                    netlist,
                    TransientStateBias {
                        window,
                        state: HeldTransientState::Captured(std::sync::Mutex::new(captured)),
                    },
                );
                Ok(StateLinearizedResult {
                    state_time,
                    result: engine.run_small_signal_card(netlist, card, abort)?,
                })
            })
            .collect()
    }

    /// Run one small-signal card through its own entry point, on the per-run
    /// engine [`Self::at_transient_state`] built for it.
    fn run_small_signal_card(
        self,
        netlist: &Netlist,
        card: &AnalysisCommand,
        abort: &dyn AbortSignal,
    ) -> Result<SmallSignalCardResult, SimulationError> {
        let grid = |variation, points, start, stop| {
            crate::analysis::ac::try_ac_sweep_frequencies_bounded_with_abort(
                variation,
                points,
                start,
                stop,
                self.config.resource_limits.max_analysis_points,
                abort,
            )
            .map_err(SimulationError::from)
        };
        Ok(match card {
            AnalysisCommand::Ac {
                variation,
                points,
                start_freq,
                stop_freq,
            } => SmallSignalCardResult::Ac(self.run_ac_with_abort(
                netlist,
                &grid(*variation, *points, *start_freq, *stop_freq)?,
                abort,
            )?),
            AnalysisCommand::Noise {
                output_node,
                reference_node,
                input_source,
                variation,
                points,
                start_freq,
                stop_freq,
            } => SmallSignalCardResult::Noise(self.run_noise_named_with_input_source_and_abort(
                netlist,
                output_node,
                reference_node.as_deref(),
                input_source,
                &grid(*variation, *points, *start_freq, *stop_freq)?,
                self.config.temperature,
                abort,
            )?),
            AnalysisCommand::Tf {
                output_node,
                reference_node,
                output_is_current,
                input_source,
            } => SmallSignalCardResult::TransferFunction(self.run_transfer_function_with_abort(
                netlist,
                output_node,
                reference_node.as_deref(),
                *output_is_current,
                input_source,
                abort,
            )?),
            AnalysisCommand::Sp { .. } => SmallSignalCardResult::SParameters(Box::new(
                self.run_sp_with_abort(netlist, card, abort)?,
            )),
            AnalysisCommand::Stb { .. } => {
                let config = crate::analysis::StbConfig::try_from(card).map_err(|error| {
                    SimulationError::Netlist(format!("invalid .STB card: {error}"))
                })?;
                SmallSignalCardResult::Stability(Box::new(
                    self.run_stb_with_abort(netlist, config, abort)?,
                ))
            }
            AnalysisCommand::PoleZero { .. } => SmallSignalCardResult::PoleZero(
                self.run_pz_from_card_with_abort(netlist, card, abort)?,
            ),
            AnalysisCommand::Sensitivity { .. } => SmallSignalCardResult::Sensitivity(Box::new(
                self.run_sensitivity_from_card_with_abort(netlist, card, abort)?,
            )),
            AnalysisCommand::Disto {
                variation,
                points,
                start_freq,
                stop_freq,
                f2_over_f1,
            } => SmallSignalCardResult::Distortion(Box::new(self.run_distortion_with_abort(
                netlist,
                &grid(*variation, *points, *start_freq, *stop_freq)?,
                *f2_over_f1,
                abort,
            )?)),
            other => {
                return Err(SimulationError::Netlist(format!(
                    "{} does not linearize around a transient state",
                    card_name(other)
                )));
            }
        })
    }
}

/// A DC `.SENS` card reports operating-point tangents, which are not the
/// derivatives of a transient state.
fn dc_sensitivity_at_state_refusal() -> SimulationError {
    crate::UnsupportedCapabilityError::new(
        "analysis.sens.transient_state.dc",
        "a DC .SENS card reports operating-point tangents, not derivatives of a transient \
         state; add an AC sweep to the card to linearize it around the state",
    )
    .for_analysis_kind(crate::identity::AnalysisKind::Sensitivity)
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::NoAbort;

    /// An RC low-pass driven by a step: the state at `T` holds the capacitor
    /// mid-charge, and the AC response around it is the RC pole's whatever the
    /// state, which pins the plumbing without a nonlinear device.
    #[test]
    fn a_linear_circuit_linearizes_to_the_same_response_at_every_state() {
        let netlist = Netlist::parse(
            "rc at states\n\
             v1 in 0 pulse(0 1 0 1n 1n 10u 20u) ac 1\n\
             r1 in out 1k\n\
             c1 out 0 1n\n\
             .tran 10n 5u\n\
             .ac dec 2 1k 1meg actimes=1u,3u\n\
             .end\n",
        )
        .expect("the deck parses");
        let index = netlist
            .analyses
            .iter()
            .position(|card| matches!(card, AnalysisCommand::Ac { .. }))
            .expect("the .ac card");
        let engine = Engine::default();
        let results = engine
            .run_small_signal_at_transient_times(&netlist, index, &[1.0e-6, 3.0e-6], &NoAbort)
            .expect("the card runs at both states");
        assert_eq!(
            results
                .iter()
                .map(|result| result.state_time)
                .collect::<Vec<_>>(),
            [1.0e-6, 3.0e-6]
        );
        for result in &results {
            let SmallSignalCardResult::Ac(points) = &result.result else {
                panic!("an .ac card gives AC points")
            };
            for point in points {
                let out = point
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("out"))
                    .expect("out is published");
                let expected = num_complex::Complex64::new(1.0, 0.0)
                    / num_complex::Complex64::new(
                        1.0,
                        2.0 * std::f64::consts::PI * point.frequency * 1.0e3 * 1.0e-9,
                    );
                let actual = point.voltages[out];
                assert!(
                    (actual - expected).norm() <= 1.0e-12 * expected.norm().max(1.0),
                    "at {} Hz: {actual} against the RC pole's {expected}",
                    point.frequency
                );
            }
        }
    }
}
