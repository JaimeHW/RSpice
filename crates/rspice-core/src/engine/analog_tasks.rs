//! Host delivery and model completion at public analysis boundaries.

use super::{Engine, SimulationError};
use crate::abort_signal::ModelRunSignal;
use crate::{AbortSignal, CircuitData, ModelFinish, ModelFinishPoint, SimulationOutcome};
use rspice_veriloga_runtime::{AnalogTaskArgument, AnalogTaskEvent, AnalogTaskKind};

pub(super) struct TransientModelCandidate {
    pub finish: Option<ModelFinish>,
    pub refinement_time: Option<f64>,
}

pub(super) struct FrequencyModelPoint {
    pub analysis: u8,
    pub frequency: f64,
    pub final_step: bool,
}

fn finish_level(event: &AnalogTaskEvent<'_>) -> Result<u8, &'static str> {
    let [AnalogTaskArgument::Integer(level)] = event.call.arguments.as_ref() else {
        return Err("analog finish call has an invalid argument snapshot");
    };
    if event.call.kind != AnalogTaskKind::Finish || !(0..=2).contains(level) {
        return Err("analog task has no implemented host control handler");
    }
    Ok(*level as u8)
}

impl Engine {
    /// Finish an AC/noise bias point before any frequency result exists. A
    /// finish makes this both the initial and final point of the analysis;
    /// its final equations must converge before the request is published.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn accept_frequency_operating_point(
        &self,
        netlist: &crate::Netlist,
        circuit: &mut CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        solution: &[f64],
        analysis: u8,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        if circuit.has_point_analog_tasks()
            || (solution.is_empty() && circuit.has_any_veriloga_devices())
        {
            Self::evaluate_analog_candidate(circuit, matrix, solution)?;
        }
        let pending =
            Self::candidate_equilibrium_finish(circuit, ModelFinishPoint::OperatingPoint)?;
        if pending.is_some() {
            circuit
                .prepare_veriloga_equilibrium_analysis_point(analysis, true, true)
                .map_err(SimulationError::Circuit)?;
            let final_solution = if solution.is_empty() {
                Vec::new()
            } else {
                self.solve_dc_operating_point_with_startup_and_abort(
                    netlist,
                    circuit,
                    matrix,
                    super::core::DcOpStartup::PreviousSolution(solution),
                    abort,
                )?
            };
            Self::evaluate_analog_candidate(circuit, matrix, &final_solution)?;
        }
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        circuit
            .accept_veriloga_analysis_point()
            .map_err(SimulationError::Circuit)?;
        Self::publish_pending_model_finish(abort, pending)?;
        Self::deliver_accepted_analog_tasks(circuit, abort, ModelFinishPoint::OperatingPoint)?;
        Self::ensure_model_run_active(abort)
    }

    /// Solve and accept one point on a private copy of the bias state. The
    /// caller invokes this in sweep order for point tasks or portless models
    /// whose bodies are not visited by small-signal matrix assembly.
    pub(super) fn solve_accepted_frequency_point<T>(
        circuit: &mut CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        bias: &[f64],
        point: FrequencyModelPoint,
        abort: &dyn AbortSignal,
        mut solve: impl FnMut(&mut CircuitData, bool) -> Result<T, SimulationError>,
    ) -> Result<T, SimulationError> {
        let mut result = solve(circuit, point.final_step)?;
        circuit
            .evaluate_frequency_analog_candidate(matrix, bias, point.analysis)
            .map_err(SimulationError::Circuit)?;
        let position = ModelFinishPoint::Frequency {
            frequency: point.frequency,
        };
        let pending = Self::candidate_equilibrium_finish(circuit, position)?;
        if pending.is_some() && !point.final_step {
            result = solve(circuit, true)?;
            circuit
                .evaluate_frequency_analog_candidate(matrix, bias, point.analysis)
                .map_err(SimulationError::Circuit)?;
        }
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        circuit
            .accept_veriloga_analysis_point()
            .map_err(SimulationError::Circuit)?;
        Self::publish_pending_model_finish(abort, pending)?;
        Self::deliver_accepted_analog_tasks(circuit, abort, position)?;
        Ok(result)
    }

    pub(super) fn model_observation_matrix() -> Result<crate::solver::StaticMatrix, SimulationError>
    {
        // The generated stamp adapter requires a nonempty sparse workspace.
        // A circuit with no unknowns uses one unused row only to execute model
        // bodies; it is never solved or included in the returned result.
        crate::solver::StaticMatrix::from_triplets(1, 1, &[]).map_err(SimulationError::Solver)
    }

    pub(super) fn evaluate_analog_candidate(
        circuit: &mut CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        solution: &[f64],
    ) -> Result<(), SimulationError> {
        #[cfg(feature = "veriloga")]
        circuit
            .evaluate_veriloga_timepoint(solution)
            .map_err(SimulationError::Circuit)?;
        #[cfg(feature = "veriloga-builtins-base")]
        circuit
            .evaluate_generated_veriloga_timepoint(matrix, solution)
            .map_err(SimulationError::Circuit)?;
        let _ = (circuit, matrix, solution);
        Ok(())
    }

    /// Run one analysis with an explicit normal-completion outcome. A model
    /// may finish during initialization, before any numerical result exists.
    /// The callback must pass the supplied signal to its analysis entry point.
    /// Reusing this Engine or its caller's abort flag does not reuse model control.
    pub fn run_with_outcome<T>(
        &self,
        abort: &dyn AbortSignal,
        run: impl FnOnce(&Self, &dyn AbortSignal) -> Result<T, SimulationError>,
    ) -> Result<SimulationOutcome<T>, SimulationError> {
        let signal = ModelRunSignal::new(abort);
        match run(self, &signal) {
            Ok(result) => match signal.model_control().and_then(|control| control.finish()) {
                Some(finish) => Ok(SimulationOutcome::Finished {
                    result: Some(result),
                    finish,
                }),
                None => Ok(SimulationOutcome::Completed(result)),
            },
            Err(SimulationError::ModelFinished(finish)) => Ok(SimulationOutcome::Finished {
                result: None,
                finish: *finish,
            }),
            Err(error) => Err(error),
        }
    }

    pub(super) fn ensure_model_run_active(abort: &dyn AbortSignal) -> Result<(), SimulationError> {
        if let Some(finish) = abort.model_control().and_then(|control| control.finish()) {
            return Err(SimulationError::ModelFinished(Box::new(finish)));
        }
        Ok(())
    }

    pub(super) fn deliver_accepted_analog_tasks(
        circuit: &mut CircuitData,
        abort: &dyn AbortSignal,
        point: ModelFinishPoint,
    ) -> Result<bool, SimulationError> {
        let control = abort.model_control().ok_or_else(|| {
            SimulationError::Circuit("analog task delivery has no simulation run scope".into())
        })?;
        let mut invalid = None;
        circuit
            .visit_accepted_analog_tasks(&mut |event| {
                let level = match finish_level(&event) {
                    Ok(level) => level,
                    Err(error) => {
                        invalid = Some(error);
                        return;
                    }
                };
                if !control.is_finished() {
                    control.request_finish(ModelFinish {
                        instance: event.instance.into(),
                        model: event.model.into(),
                        site: event.call.site,
                        point,
                        diagnostic_level: level,
                    });
                }
            })
            .map_err(SimulationError::Circuit)?;
        if let Some(message) = invalid {
            return Err(SimulationError::Circuit(message.into()));
        }
        Ok(control.is_finished())
    }

    pub(super) fn candidate_equilibrium_finish(
        circuit: &CircuitData,
        point: ModelFinishPoint,
    ) -> Result<Option<ModelFinish>, SimulationError> {
        #[cfg(feature = "veriloga")]
        circuit.ensure_no_mixed_signal_hosts("equilibrium task acceptance")?;
        circuit
            .first_nonmixed_candidate_analog_task(AnalogTaskKind::Finish)
            .map_err(SimulationError::Circuit)?
            .map(|event| {
                circuit
                    .validate_nonmixed_model_acceptance()
                    .map_err(SimulationError::Circuit)?;
                let diagnostic_level =
                    finish_level(&event).map_err(|error| SimulationError::Circuit(error.into()))?;
                Ok(ModelFinish {
                    instance: event.instance.into(),
                    model: event.model.into(),
                    site: event.call.site,
                    point,
                    diagnostic_level,
                })
            })
            .transpose()
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn inspect_transient_model_candidate(
        circuit: &mut CircuitData,
        time: f64,
        dt: f64,
        solution: &[f64],
        coefficients: &crate::numerics::integration::CompanionCoefficients,
        initial_step: bool,
        final_step: bool,
    ) -> Result<TransientModelCandidate, SimulationError> {
        let point = ModelFinishPoint::Transient { time };
        let refinement_time = circuit
            .veriloga_event_refinement_time()
            .map_err(SimulationError::Circuit)?;
        let mut candidate = None;
        let mut invalid = None;
        let mut consume = |event: AnalogTaskEvent<'_>| match finish_level(&event) {
            Ok(level) if candidate.is_none() => {
                candidate = Some(ModelFinish {
                    instance: event.instance.into(),
                    model: event.model.into(),
                    site: event.call.site,
                    point,
                    diagnostic_level: level,
                });
            }
            Err(error) => invalid = Some(error),
            _ => {}
        };
        if refinement_time.is_none()
            && let Some(event) = circuit
                .first_nonmixed_candidate_analog_task(AnalogTaskKind::Finish)
                .map_err(SimulationError::Circuit)?
        {
            consume(event);
        }
        #[cfg(feature = "veriloga")]
        let mixed_refinement_time = circuit.visit_mixed_transient_candidate_task(
            time,
            dt,
            solution,
            coefficients,
            initial_step,
            final_step,
            AnalogTaskKind::Finish,
            false,
            &mut consume,
        )?;
        #[cfg(not(feature = "veriloga"))]
        let mixed_refinement_time = None;
        let _ = (dt, solution, coefficients, initial_step, final_step);
        if let Some(error) = invalid {
            return Err(SimulationError::Circuit(error.into()));
        }
        let refinement_time = [refinement_time, mixed_refinement_time]
            .into_iter()
            .flatten()
            .reduce(f64::min);
        if candidate.is_some() && refinement_time.is_none() {
            circuit
                .validate_nonmixed_model_acceptance()
                .map_err(SimulationError::Circuit)?;
            #[cfg(feature = "veriloga")]
            circuit.visit_mixed_transient_candidate_task(
                time,
                dt,
                solution,
                coefficients,
                initial_step,
                final_step,
                AnalogTaskKind::Finish,
                true,
                &mut |_| {},
            )?;
        }
        Ok(TransientModelCandidate {
            finish: candidate,
            refinement_time,
        })
    }

    pub(super) fn publish_pending_model_finish(
        abort: &dyn AbortSignal,
        finish: Option<ModelFinish>,
    ) -> Result<(), SimulationError> {
        if let Some(finish) = finish {
            abort
                .model_control()
                .ok_or_else(|| {
                    SimulationError::Circuit("model finish has no simulation run scope".into())
                })?
                .request_finish(finish);
        }
        Ok(())
    }

    pub(super) fn deliver_initial_analog_tasks(
        circuit: &mut CircuitData,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        Self::deliver_accepted_analog_tasks(circuit, abort, ModelFinishPoint::Initialization)?;
        Self::ensure_model_run_active(abort)
    }
}
