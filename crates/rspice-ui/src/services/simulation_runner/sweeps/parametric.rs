use super::super::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted};
use super::execution::run_temperature_sweep;
use super::mapping::{
    describe_step_target, map_dc_sweep_results_with_abort, map_temperature_results,
};
use super::sweep_points::expand_step_sweep_values_with_abort;
use super::types::{CornerBaseMode, ParametricData, TempRunConfig};
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::engine::Engine;
use rspice_core::netlist::{AnalysisCommand, StepSweep, StepTarget};
use std::path::Path;

/// Run parametric analysis by executing the first `.STEP` command in the netlist.
pub fn run_parametric_analysis(netlist_text: &str) -> Result<ParametricData, String> {
    run_parametric_analysis_with_abort(netlist_text, &NoAbort).map_err(|error| error.to_string())
}

/// Run parametric analysis with cooperative cancellation.
pub fn run_parametric_analysis_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<ParametricData> {
    run_parametric_analysis_with_source_path_and_abort(netlist_text, None, abort)
}

/// Run parametric analysis by executing the first `.STEP` command in the
/// netlist, resolving relative includes from the source path when provided.
pub fn run_parametric_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<ParametricData, String> {
    run_parametric_analysis_with_source_path_and_abort(netlist_text, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run parametric analysis with source-path resolution and cooperative
/// cancellation through parsing, point expansion, solves, and mapping.
pub fn run_parametric_analysis_with_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<ParametricData> {
    let netlist = super::super::parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    ensure_not_aborted(abort)?;

    let step_cmd = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Step(step) => Some(step),
            _ => None,
        })
        .ok_or_else(|| {
            ServiceRunError::Failure(
                "Parametric analysis requires a .STEP command in the netlist".to_string(),
            )
        })?;

    let is_data_sweep = matches!(step_cmd.sweep, StepSweep::Data { .. });
    let values = if is_data_sweep {
        Vec::new()
    } else {
        expand_step_sweep_values_with_abort(&step_cmd.sweep, abort)?
    };
    if values.is_empty() && !is_data_sweep {
        return Err(ServiceRunError::Failure(
            "Parametric analysis has no sweep points to execute".to_string(),
        ));
    }

    let results = if step_cmd.target == StepTarget::Temp {
        let cfg = TempRunConfig {
            temperatures_c: values.clone(),
            base_mode: CornerBaseMode::Op,
        };
        return run_parametric_analysis_with_netlist_and_config(&netlist, &cfg, "TEMP", abort);
    } else {
        let engine = Engine::new(super::super::build_engine_config(&netlist, None));
        engine
            .run_step_command_with_abort(&netlist, step_cmd, &values, abort)
            .map_err(|error| ServiceRunError::from_core("Parametric analysis error", error))?
    };

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
    let (sweep_values, voltages) = map_dc_sweep_results_with_abort(&results, abort)?;

    Ok(ParametricData {
        target: describe_step_target(step_cmd),
        num_points: sweep_values.len(),
        sweep_values,
        voltages,
        num_failures,
    })
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
        assert_eq!(data.num_points, 2);
        assert_eq!(data.num_failures, 0);

        let (_, out_values) = data
            .voltages
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("V(out)"))
            .expect("V(out) trace");
        assert!((out_values[0] - 0.5).abs() < 1e-12);
        assert!((out_values[1] - (1.0 / 3.0)).abs() < 1e-12);
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
}

/// Run temperature sweep analysis with explicit base-mode configuration.
pub fn run_parametric_analysis_with_config(
    netlist_text: &str,
    config: &TempRunConfig,
) -> Result<ParametricData, String> {
    run_parametric_analysis_with_config_and_abort(netlist_text, config, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run an explicitly configured temperature sweep with cooperative
/// cancellation.
pub fn run_parametric_analysis_with_config_and_abort(
    netlist_text: &str,
    config: &TempRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<ParametricData> {
    run_parametric_analysis_with_config_and_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run temperature sweep analysis with explicit base-mode configuration and a
/// source path used to resolve relative includes and model file references.
pub fn run_parametric_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &TempRunConfig,
    source_path: Option<&Path>,
) -> Result<ParametricData, String> {
    run_parametric_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        config,
        source_path,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run an explicitly configured temperature sweep with source-path resolution
/// and cooperative cancellation.
pub fn run_parametric_analysis_with_config_and_source_path_and_abort(
    netlist_text: &str,
    config: &TempRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<ParametricData> {
    let netlist = super::super::parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    run_parametric_analysis_with_netlist_and_config(&netlist, config, "TEMP", abort)
}

fn run_parametric_analysis_with_netlist_and_config(
    netlist: &rspice_core::Netlist,
    config: &TempRunConfig,
    target: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<ParametricData> {
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;

    let results = run_temperature_sweep(netlist, &config.temperatures_c, &config.base_mode, abort)?;
    if results.is_empty() {
        return Err(ServiceRunError::Failure(
            "Parametric analysis produced no converged sweep points".to_string(),
        ));
    }

    let num_failures = config.temperatures_c.len().saturating_sub(results.len());
    let metric_label = config.base_mode.metric_label();
    let (sweep_values, voltages) = map_temperature_results(&results, metric_label, abort)?;

    Ok(ParametricData {
        target: target.to_string(),
        num_points: sweep_values.len(),
        sweep_values,
        voltages,
        num_failures,
    })
}
