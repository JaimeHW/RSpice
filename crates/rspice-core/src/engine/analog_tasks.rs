//! Host delivery and model completion at public analysis boundaries.

use super::{Engine, SimulationError};
use crate::abort_signal::ModelRunSignal;
use crate::{AbortSignal, CircuitData, ModelFinish, ModelFinishPoint, SimulationOutcome};
use rspice_veriloga_runtime::{AnalogTaskArgument, AnalogTaskEvent, AnalogTaskKind};

pub(super) struct TransientModelCandidate {
    pub finish: Option<ModelFinish>,
    pub refinement_time: Option<f64>,
    /// `refinement_time` is a Verilog-A analog event's own root (`cross`,
    /// `above`), whose body runs in the solve that lands on it, rather than a
    /// mixed module's A/D threshold crossing.
    pub analog_event_root: bool,
    pub discontinuity: bool,
}

pub(super) struct FrequencyModelPoint {
    pub analysis: u8,
    pub frequency: f64,
    pub final_step: bool,
}

/// One analysis state participating in a complete frequency point. SP keeps
/// AC and noise identities separate while accepting their common endpoint.
pub(super) struct FrequencyModelState<'a> {
    pub circuit: &'a mut CircuitData,
    pub matrix: &'a mut crate::solver::StaticMatrix,
    pub bias: &'a [f64],
    pub analysis: u8,
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

/// One accepted analog output call as the run's HDL output record (a call
/// whose model has no source text names the model as its file), or `None`
/// for a call that prints nothing: a `$monitor` whose arguments have not
/// changed since it last printed, or any call at a small-signal frequency
/// point, which linearizes the accepted bias rather than evaluating the model
/// at a new solution.
fn analog_output_record(
    event: &AnalogTaskEvent<'_>,
    point: ModelFinishPoint,
    monitors: &mut std::collections::HashMap<(String, u32), Box<[AnalogTaskArgument]>>,
) -> Option<crate::config::HdlOutputRecord> {
    use crate::config::{HdlOrigin, HdlOutputRecord, HdlPoint, HdlSeverity, HdlStream};
    let point = match point {
        ModelFinishPoint::Initialization => HdlPoint::Initialization,
        ModelFinishPoint::OperatingPoint => HdlPoint::OperatingPoint,
        ModelFinishPoint::Transient { time } => HdlPoint::Transient { time },
        ModelFinishPoint::DcSweep { value } => HdlPoint::DcSweep { value },
        ModelFinishPoint::Frequency { .. } => return None,
    };
    if event.call.kind == AnalogTaskKind::Monitor {
        let key = (event.instance.to_owned(), event.call.site);
        if monitors.get(&key) == Some(&event.call.arguments) {
            return None;
        }
        monitors.insert(key, event.call.arguments.clone());
    }
    let mut text = format_analog_output(event);
    let stream = match event.call.kind {
        AnalogTaskKind::Info => HdlStream::Diagnostic {
            severity: HdlSeverity::Info,
        },
        AnalogTaskKind::Warning => HdlStream::Diagnostic {
            severity: HdlSeverity::Warning,
        },
        AnalogTaskKind::Error => HdlStream::Diagnostic {
            severity: HdlSeverity::Error,
        },
        kind => {
            if kind != AnalogTaskKind::Write {
                text.push('\n');
            }
            HdlStream::Stdout
        }
    };
    Some(HdlOutputRecord {
        point,
        tick: 0,
        seconds: event.call.time,
        stream,
        text,
        origin: HdlOrigin {
            scope: event.instance.into(),
            file: event
                .source
                .map_or_else(|| event.model.into(), |source| source.file.into()),
            line: event.source.map_or(0, |source| source.line),
        },
    })
}

/// The `$display` formatter of the digital host (IEEE 1364-2005 section
/// 17.1.1), with `%m` naming the instance.
#[cfg(feature = "veriloga")]
fn format_analog_output(event: &AnalogTaskEvent<'_>) -> String {
    use crate::xspice::verilog::effects::format::{
        FormatArgument, FormatContext, TimeFormat, format,
    };
    use rspice_veriloga::canonical_ir::digital::DigitalRadix;
    use rspice_veriloga::canonical_ir::digital_eval::DigitalScalar;
    let scalars = event
        .call
        .arguments
        .iter()
        .map(|argument| match argument {
            AnalogTaskArgument::Real(value) => DigitalScalar::Real(*value),
            AnalogTaskArgument::Integer(value) => DigitalScalar::Integer(
                (*value).clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32,
            ),
            AnalogTaskArgument::String(_) => DigitalScalar::Effect,
        })
        .collect::<Vec<_>>();
    let arguments = event
        .call
        .arguments
        .iter()
        .zip(&scalars)
        .map(|(argument, scalar)| match argument {
            AnalogTaskArgument::String(text) => FormatArgument::Literal(text),
            _ => FormatArgument::Value(scalar, true),
        })
        .collect::<Vec<_>>();
    let time_format = TimeFormat::default_for(0);
    format(
        &arguments,
        &FormatContext {
            radix: DigitalRadix::Decimal,
            scope: event.instance,
            module: event.model,
            time_unit: 0,
            time_format: &time_format,
        },
    )
}

/// Without the digital host, the arguments in order, as written.
#[cfg(not(feature = "veriloga"))]
fn format_analog_output(event: &AnalogTaskEvent<'_>) -> String {
    event
        .call
        .arguments
        .iter()
        .map(|argument| match argument {
            AnalogTaskArgument::Real(value) => format!("{value}"),
            AnalogTaskArgument::Integer(value) => format!("{value}"),
            AnalogTaskArgument::String(text) => text.clone(),
        })
        .collect()
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
        Self::solve_accepted_frequency_group(
            &mut [FrequencyModelState {
                circuit,
                matrix,
                bias,
                analysis: point.analysis,
            }],
            point.frequency,
            point.final_step,
            abort,
            |states, final_step| solve(states[0].circuit, final_step),
        )
    }

    /// Finish every numerical projection before observing tasks. A finish in
    /// either SP phase makes both phases final; no phase can publish control
    /// while another still holds an unvalidated candidate.
    pub(super) fn solve_accepted_frequency_group<T>(
        states: &mut [FrequencyModelState<'_>],
        frequency: f64,
        final_step: bool,
        abort: &dyn AbortSignal,
        mut solve: impl FnMut(&mut [FrequencyModelState<'_>], bool) -> Result<T, SimulationError>,
    ) -> Result<T, SimulationError> {
        let position = ModelFinishPoint::Frequency { frequency };
        let observe = |states: &mut [FrequencyModelState<'_>]| {
            let mut first = None;
            for state in states {
                state
                    .circuit
                    .evaluate_frequency_analog_candidate(state.matrix, state.bias, state.analysis)
                    .map_err(SimulationError::Circuit)?;
                let pending = Self::candidate_equilibrium_finish(state.circuit, position)?;
                if first.is_none() {
                    first = pending;
                }
            }
            Ok::<_, SimulationError>(first)
        };
        let mut result = solve(states, final_step)?;
        let pending = observe(states)?;
        if pending.is_some() && !final_step {
            result = solve(states, true)?;
            observe(states)?;
        }
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        for state in states.iter_mut() {
            state
                .circuit
                .accept_veriloga_analysis_point()
                .map_err(SimulationError::Circuit)?;
        }
        Self::publish_pending_model_finish(abort, pending)?;
        for state in states {
            Self::deliver_accepted_analog_tasks(state.circuit, abort, position)?;
        }
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
        let output = circuit.analog_task_output.clone();
        let mut monitors = std::mem::take(&mut circuit.analog_monitor_values);
        let visited = circuit.visit_accepted_analog_tasks(&mut |event| {
            if event.call.kind.is_output() {
                if let Some(sink) = &output
                    && let Some(record) = analog_output_record(&event, point, &mut monitors)
                {
                    sink.emit(&record);
                }
                return;
            }
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
        });
        circuit.analog_monitor_values = monitors;
        visited.map_err(SimulationError::Circuit)?;
        if let Some(message) = invalid {
            return Err(SimulationError::Circuit(message.into()));
        }
        Ok(control.is_finished())
    }

    /// A `$finish` the bias candidate of an equilibrium point requested, from
    /// any analog model including a mixed module's held analog half.
    pub(super) fn candidate_equilibrium_finish(
        circuit: &CircuitData,
        point: ModelFinishPoint,
    ) -> Result<Option<ModelFinish>, SimulationError> {
        circuit
            .first_equilibrium_candidate_analog_task(AnalogTaskKind::Finish)
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
        abort: &dyn AbortSignal,
        circuit: &mut CircuitData,
        time: f64,
        dt: f64,
        solution: &[f64],
        companion: crate::circuit::XspiceCompanionPolicy<'_>,
        minimum_timestep: f64,
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
        let (mixed_refinement_time, mixed_event_root, mixed_discontinuity) = {
            let mixed = circuit.visit_mixed_transient_candidate_task(
                abort,
                time,
                dt,
                solution,
                companion,
                minimum_timestep,
                initial_step,
                final_step,
                AnalogTaskKind::Finish,
                false,
                &mut consume,
            )?;
            (
                mixed.refinement,
                mixed.analog_event_root,
                mixed.discontinuity,
            )
        };
        #[cfg(not(feature = "veriloga"))]
        let (mixed_refinement_time, mixed_event_root, mixed_discontinuity) = (None, false, false);
        let _ = (
            abort,
            dt,
            solution,
            companion,
            minimum_timestep,
            initial_step,
            final_step,
        );
        if let Some(error) = invalid {
            return Err(SimulationError::Circuit(error.into()));
        }
        let analog_refinement_time = refinement_time;
        let refinement_time = [analog_refinement_time, mixed_refinement_time]
            .into_iter()
            .flatten()
            .reduce(f64::min);
        // On a tie the analog event's body runs at the landing all the same.
        let analog_event_root = refinement_time.is_some_and(|root| {
            analog_refinement_time == Some(root)
                || (mixed_event_root && mixed_refinement_time == Some(root))
        });
        if candidate.is_some() && refinement_time.is_none() {
            circuit
                .validate_nonmixed_model_acceptance()
                .map_err(SimulationError::Circuit)?;
            #[cfg(feature = "veriloga")]
            circuit.visit_mixed_transient_candidate_task(
                abort,
                time,
                dt,
                solution,
                companion,
                minimum_timestep,
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
            analog_event_root,
            discontinuity: circuit.veriloga_discontinuity_rising() || mixed_discontinuity,
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
