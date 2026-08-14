//! Parametric sweep.
//!
//! Steps one design parameter and returns a result set per point.

use super::super::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted};
use super::mapping::describe_step_target;
use super::sweep_points::expand_step_sweep_values_with_abort;
use super::types::ParametricData;
use super::types::{CornerBaseMode, CornerFrequencySweep};
use rspice_core::abort_signal::AbortSignal;
#[cfg(test)]
use rspice_core::abort_signal::NoAbort;
use rspice_core::engine::Engine;
use rspice_core::netlist::{AnalysisCommand, StepSweep, StepTarget};
use std::path::Path;

/// Run parametric analysis, reporting failures as strings. Test-only; see
/// [`run_parametric_analysis_with_source_path_and_abort`].
#[cfg(test)]
pub fn run_parametric_analysis(netlist_text: &str) -> Result<ParametricData, String> {
    run_parametric_analysis_with_abort(netlist_text, &NoAbort).map_err(|error| error.to_string())
}

/// Run parametric analysis with cooperative cancellation and no source path.
/// Test-only; see [`run_parametric_analysis`].
#[cfg(test)]
pub fn run_parametric_analysis_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<ParametricData> {
    run_parametric_analysis_with_source_path_and_abort(netlist_text, None, abort)
}

/// Run parametric analysis by executing the first design-parameter `.STEP`
/// command in the netlist, with source-path resolution and cooperative
/// cancellation through parsing, point expansion, solves, and mapping.
///
/// A `.STEP TEMP` is passed over rather than executed. Temperature is a PVT
/// axis: it is planned as its own declaration and expanded into one solve per
/// temperature before the run is authorized, so executing it here would solve
/// the same space a second time. It also used to shadow the parameter step a
/// deck declaring both had asked for, because the first `.STEP` in the deck
/// won and the temperature card is normally written first.
pub fn run_parametric_analysis_with_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<ParametricData> {
    run_parametric_analysis_with_base_and_source_path_and_abort(
        netlist_text,
        source_path,
        &CornerBaseMode::Op,
        abort,
    )
}

/// Execute a design-parameter `.STEP` using the authored paired base
/// analysis. Each stepped netlist is solved independently and contributes the
/// terminal value of that analysis to the parametric family.
pub fn run_parametric_analysis_with_base_and_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    base_mode: &CornerBaseMode,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<ParametricData> {
    super::types::validate_base_mode("Parametric", base_mode).map_err(ServiceRunError::Failure)?;
    let netlist = super::super::parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    ensure_not_aborted(abort)?;

    let step_cmd = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Step(step) if step.target != StepTarget::Temp => Some(step),
            _ => None,
        })
        .ok_or_else(|| {
            ServiceRunError::Failure(
                "Parametric analysis requires a design-parameter .STEP command in the netlist"
                    .to_string(),
            )
        })?;

    let is_data_sweep = matches!(step_cmd.sweep, StepSweep::Data { .. });
    let engine_config = super::super::build_engine_config(&netlist, None);
    let values = if is_data_sweep {
        Vec::new()
    } else {
        expand_step_sweep_values_with_abort(
            &step_cmd.sweep,
            engine_config.resource_limits.max_batch_runs,
            abort,
        )?
    };
    if values.is_empty() && !is_data_sweep {
        return Err(ServiceRunError::Failure(
            "Parametric analysis has no sweep points to execute".to_string(),
        ));
    }

    let engine = Engine::new(engine_config);
    let stepped = engine
        .step_netlists_for_command_with_abort(&netlist, step_cmd, &values, abort)
        .map_err(|error| ServiceRunError::from_core("Parametric analysis error", error))?;
    let mut results = Vec::with_capacity(stepped.len());
    for (value, stepped_netlist) in stepped {
        ensure_not_aborted(abort)?;
        let (names, values) = run_base_analysis(&engine, &stepped_netlist, base_mode, abort)
            .map_err(|error| ServiceRunError::from_core("Parametric base analysis error", error))?;
        results.push((value, names, values));
    }

    if results.is_empty() {
        return Err(ServiceRunError::Failure(
            "Parametric analysis produced no converged sweep points".to_string(),
        ));
    }

    let num_failures = if is_data_sweep {
        0
    } else {
        values.len().saturating_sub(results.len())
    };
    let sweep_values = results
        .iter()
        .map(|(value, _, _)| *value)
        .collect::<Vec<_>>();
    if sweep_values.iter().any(|value| !value.is_finite()) {
        return Err(ServiceRunError::Failure(
            "Parametric analysis returned a non-finite sweep coordinate".to_owned(),
        ));
    }
    let names = results[0].1.clone();
    validate_terminal_quantities(&names, &results[0].2, "reference parametric point")
        .map_err(|error| ServiceRunError::from_core("Parametric result error", error))?;
    for (point_index, (_, point_names, point_values)) in results.iter().enumerate().skip(1) {
        validate_terminal_quantities(
            point_names,
            point_values,
            &format!("parametric point {}", point_index + 1),
        )
        .map_err(|error| ServiceRunError::from_core("Parametric result error", error))?;
        if point_names.len() != names.len()
            || point_names
                .iter()
                .zip(&names)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
        {
            return Err(ServiceRunError::Failure(format!(
                "Parametric point {} changed the solved quantity basis",
                point_index + 1
            )));
        }
    }
    let mut voltages = Vec::with_capacity(names.len());
    for (quantity_index, name) in names.into_iter().enumerate() {
        let mut values = Vec::with_capacity(results.len());
        for (_, _, point_values) in &results {
            values.push(point_values[quantity_index]);
        }
        voltages.push((base_mode.metric_label().format_trace_name(&name), values));
    }

    Ok(ParametricData {
        target: describe_step_target(step_cmd),
        sweep_values,
        voltages,
        num_failures,
    })
}

fn run_base_analysis(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    mode: &CornerBaseMode,
    abort: &dyn AbortSignal,
) -> Result<(Vec<String>, Vec<f64>), rspice_core::SimulationError> {
    match mode {
        CornerBaseMode::Op => {
            let result = engine.run_dc_op_with_abort(netlist, abort)?;
            Ok((
                result.node_names.into_iter().skip(1).collect(),
                result.node_voltages.into_iter().skip(1).collect(),
            ))
        }
        CornerBaseMode::DcSweep {
            source_name,
            start,
            stop,
            step,
        } => {
            let result = engine
                .run_dc_sweep_with_abort(netlist, source_name, *start, *stop, *step, abort)?
                .into_iter()
                .last()
                .ok_or_else(|| {
                    rspice_core::SimulationError::Circuit(
                        "DC base sweep produced no points".to_owned(),
                    )
                })?
                .1;
            Ok((
                result.node_names.into_iter().skip(1).collect(),
                result.node_voltages.into_iter().skip(1).collect(),
            ))
        }
        CornerBaseMode::DcSweepNested {
            source_name,
            start,
            stop,
            step,
            source2,
            start2,
            stop2,
            step2,
        } => {
            let second = rspice_core::netlist::DcSecondSweep::linear(
                source2.clone(),
                *start2,
                *stop2,
                *step2,
            );
            let result = engine
                .run_dc_sweep2_with_abort(
                    netlist,
                    source_name,
                    *start,
                    *stop,
                    *step,
                    Some(&second),
                    abort,
                )?
                .into_iter()
                .last()
                .ok_or_else(|| {
                    rspice_core::SimulationError::Circuit(
                        "Nested DC base sweep produced no points".to_owned(),
                    )
                })?
                .1;
            Ok((
                result.node_names.into_iter().skip(1).collect(),
                result.node_voltages.into_iter().skip(1).collect(),
            ))
        }
        CornerBaseMode::Transient {
            stop_time,
            step_time,
        } => terminal_transient(engine, netlist, *stop_time, *step_time, abort),
        CornerBaseMode::TransientWindow {
            stop_time,
            step_time,
            max_timestep,
            ..
        } => terminal_transient(
            engine,
            netlist,
            *stop_time,
            max_timestep.unwrap_or(*step_time),
            abort,
        ),
        CornerBaseMode::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        } => {
            let variation = match sweep {
                CornerFrequencySweep::Decade => rspice_core::netlist::FreqVariation::Dec,
                CornerFrequencySweep::Octave => rspice_core::netlist::FreqVariation::Oct,
                CornerFrequencySweep::Linear => rspice_core::netlist::FreqVariation::Lin,
            };
            let frequencies = rspice_core::analysis::ac::ac_sweep_frequencies(
                variation,
                *points_per_unit,
                *start_freq,
                *stop_freq,
            );
            let result = engine
                .run_ac_with_abort(netlist, &frequencies, abort)?
                .into_iter()
                .last()
                .ok_or_else(|| {
                    rspice_core::SimulationError::Circuit(
                        "AC base sweep produced no points".to_owned(),
                    )
                })?;
            Ok((
                result.node_names,
                result
                    .voltages
                    .into_iter()
                    .map(|value| value.norm())
                    .collect(),
            ))
        }
    }
}

fn terminal_transient(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    stop_time: f64,
    max_timestep: f64,
    abort: &dyn AbortSignal,
) -> Result<(Vec<String>, Vec<f64>), rspice_core::SimulationError> {
    let result = engine.run_tran_with_abort(netlist, stop_time, max_timestep, abort)?;
    if result.node_names.len() != result.voltages.len() {
        return Err(rspice_core::SimulationError::Circuit(format!(
            "transient base sweep returned {} node names but {} waveforms",
            result.node_names.len(),
            result.voltages.len()
        )));
    }
    let mut values = Vec::with_capacity(result.voltages.len());
    for (node_index, waveform) in result.voltages.iter().enumerate() {
        if abort.is_aborted() {
            return Err(rspice_core::SimulationError::Aborted);
        }
        let value = waveform.last().copied().ok_or_else(|| {
            rspice_core::SimulationError::Circuit(format!(
                "transient base sweep returned an empty waveform for node '{}'",
                result.node_names[node_index]
            ))
        })?;
        if !value.is_finite() {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "transient base sweep returned a non-finite terminal value for node '{}'",
                result.node_names[node_index]
            )));
        }
        values.push(value);
    }
    Ok((result.node_names, values))
}

fn validate_terminal_quantities(
    names: &[String],
    values: &[f64],
    context: &str,
) -> Result<(), rspice_core::SimulationError> {
    if names.is_empty() || names.len() != values.len() {
        return Err(rspice_core::SimulationError::Circuit(format!(
            "{context} returned {} quantity names for {} values",
            names.len(),
            values.len()
        )));
    }
    let mut normalized = std::collections::HashSet::with_capacity(names.len());
    if names.iter().any(|name| {
        let name = name.trim().to_ascii_lowercase();
        name.is_empty() || !normalized.insert(name)
    }) || values.iter().any(|value| !value.is_finite())
    {
        return Err(rspice_core::SimulationError::Circuit(format!(
            "{context} returned an empty/duplicate identity or non-finite value"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    struct AbortOnPoll {
        abort_on: usize,
        polls: AtomicUsize,
    }

    impl AbortOnPoll {
        fn new(abort_on: usize) -> Self {
            Self {
                abort_on,
                polls: AtomicUsize::new(0),
            }
        }
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    const PARAMETRIC_DECK: &str = "\
Parametric cancellation
.param rload=1k
V1 in 0 1
R1 in out {rload}
R2 out 0 1k
.step param rload list 1k 2k 3k 4k 5k 6k
.end
";

    #[test]
    fn parametric_analysis_runs_step_data_table_rows() {
        let data = run_parametric_analysis(
            "data step\n\
             .param rload=1k\n\
             V1 in 0 1\n\
             R1 in out {rload}\n\
             R2 out 0 1k\n\
             .data sweep\n\
             + rload\n\
             + 1k\n\
             + 2k\n\
             .enddata\n\
             .step data=sweep\n\
             .end\n",
        )
        .expect(".STEP DATA should execute through the parametric runner");

        assert_eq!(data.target, "DATA SWEEP");
        assert_eq!(data.sweep_values, vec![0.0, 1.0]);
        assert_eq!(data.num_failures, 0);

        let (_, out_values) = data
            .voltages
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("V(out)"))
            .expect("V(out) trace");
        assert!((out_values[0] - 0.5).abs() < 1e-12);
        assert!((out_values[1] - (1.0 / 3.0)).abs() < 1e-12);
    }

    /// A deck that declares both steps asked for both, and the temperature one
    /// is already solved point by point by its own declaration. Taking it here
    /// as well ran the same sweep twice and never ran the parameter step.
    #[test]
    fn a_temperature_step_never_shadows_the_parameter_step_beside_it() {
        let data = run_parametric_analysis(
            "both steps\n\
             .param rload=1k\n\
             V1 in 0 1\n\
             R1 in out {rload}\n\
             R2 out 0 1k\n\
             .step temp list -40 27 85\n\
             .step param rload list 1k 3k\n\
             .end\n",
        )
        .expect("the parameter step is the one this runner executes");

        assert!(
            data.target.eq_ignore_ascii_case("PARAM rload"),
            "{}",
            data.target
        );
        assert_eq!(data.sweep_values, vec![1000.0, 3000.0]);
    }

    #[test]
    fn parametric_honors_early_abort_before_invalid_input() {
        let abort = AbortOnPoll::new(1);
        let result = run_parametric_analysis_with_abort("invalid", &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn parametric_honors_abort_during_point_execution() {
        let abort = AbortOnPoll::new(8);
        let result = run_parametric_analysis_with_abort(PARAMETRIC_DECK, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.polls.load(Ordering::Relaxed) >= 8);
    }

    #[test]
    fn parametric_transient_base_solves_every_stepped_netlist() {
        let data = run_parametric_analysis_with_base_and_source_path_and_abort(
            "transient step\n\
             .param rload=1k\n\
             V1 in 0 pulse(0 1 0 1n 1n 20n 40n)\n\
             R1 in out {rload}\n\
             C1 out 0 1p\n\
             .tran 0.5n 10n\n\
             .step param rload list 1k 2k\n\
             .end\n",
            None,
            &CornerBaseMode::Transient {
                stop_time: 10e-9,
                step_time: 0.5e-9,
            },
            &NoAbort,
        )
        .expect("transient parameter base");

        assert_eq!(data.sweep_values, vec![1000.0, 2000.0]);
        let (_, output) = data
            .voltages
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("V(out)"))
            .expect("terminal output trace");
        assert_eq!(output.len(), 2);
        assert!(output.iter().all(|value| value.is_finite()));
    }
}
