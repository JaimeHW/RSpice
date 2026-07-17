use std::collections::HashMap;
use std::path::Path;

use rspice_core::abort_signal::AbortSignal;

use crate::services::simulation_runner as svc_runner;
use crate::simulation::multi_run::{AnalysisSpec, OptimizationAlgorithm, OptimizationGoal};
use crate::simulation::results::{SimulationResult, WaveformData};
use crate::simulation::runner::SimulationError;

pub(super) fn run_device_spec(
    spec: AnalysisSpec,
    netlist: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    super::ensure_not_aborted(abort)?;
    match spec {
        AnalysisSpec::Reliability {
            target_years,
            enable_hci,
            enable_nbti,
            enable_em,
            min_stress_voltage,
        } => run_reliability(
            netlist,
            ReliabilityRunRequest {
                target_years,
                enable_hci,
                enable_nbti,
                enable_em,
                min_stress_voltage,
            },
            source_path,
            abort,
        ),
        AnalysisSpec::Optimization {
            variables,
            objective_node,
            objective_ref,
            goal,
            target,
            algorithm,
            max_iterations,
            cost_tolerance,
            fd_step,
            initial_step,
            min_step,
        } => run_optimization(
            netlist,
            variables,
            objective_node,
            objective_ref,
            goal,
            target,
            algorithm,
            max_iterations,
            cost_tolerance,
            fd_step,
            initial_step,
            min_step,
            source_path,
            abort,
        ),
        AnalysisSpec::Soa {
            stop_time,
            step_time,
            check_vgs_max,
            max_vgs,
            check_vds_max,
            max_vds,
            check_vbe_max,
            max_vbe,
            check_vce_max,
            max_vce,
        } => run_soa(
            netlist,
            stop_time,
            step_time,
            check_vgs_max,
            max_vgs,
            check_vds_max,
            max_vds,
            check_vbe_max,
            max_vbe,
            check_vce_max,
            max_vce,
            source_path,
            abort,
        ),
        other => Err(super::misrouted_spec_error("device", &other)),
    }
}

struct ReliabilityRunRequest {
    target_years: Vec<f64>,
    enable_hci: bool,
    enable_nbti: bool,
    enable_em: bool,
    min_stress_voltage: f64,
}

fn run_reliability(
    netlist: &str,
    request: ReliabilityRunRequest,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let ReliabilityRunRequest {
        target_years,
        enable_hci,
        enable_nbti,
        enable_em,
        min_stress_voltage,
    } = request;
    let cfg = svc_runner::ReliabilityRunConfig {
        target_years,
        enable_hci,
        enable_nbti,
        enable_em,
        min_stress_voltage,
    };
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_reliability_analysis_with_config_and_source_path_and_abort(
            netlist,
            &cfg,
            source_path,
            abort,
        )
    })?;

    let mut waveforms = HashMap::new();
    for device in &data.device_results {
        super::ensure_not_aborted(abort)?;
        let mut years = Vec::with_capacity(data.years.len());
        let mut vth = Vec::with_capacity(data.years.len());
        let mut mobility = Vec::with_capacity(data.years.len());
        let mut rds = Vec::with_capacity(data.years.len());

        for years_key in &data.years {
            super::ensure_not_aborted(abort)?;
            let key = format!("{}y", years_key);
            let shift = device.shifts.get(&key).cloned().unwrap_or_default();
            years.push(*years_key);
            vth.push(shift.vth_shift);
            mobility.push(shift.mobility_shift);
            rds.push(shift.rds_shift);
        }

        insert_scalar_waveform(
            &mut waveforms,
            format!("DVTH({})", device.device_id),
            years.clone(),
            vth,
            "V",
            "year",
        );
        insert_scalar_waveform(
            &mut waveforms,
            format!("DMU({})", device.device_id),
            years.clone(),
            mobility,
            "ratio",
            "year",
        );
        insert_scalar_waveform(
            &mut waveforms,
            format!("DRDS({})", device.device_id),
            years,
            rds,
            "ratio",
            "year",
        );
    }

    Ok(SimulationResult::Reliability {
        years: data.years,
        waveforms,
        device_results: data.device_results,
    })
}

#[allow(clippy::too_many_arguments)]
fn run_optimization(
    netlist: &str,
    variables: Vec<crate::simulation::multi_run::OptimizationVariable>,
    objective_node: String,
    objective_ref: String,
    goal: OptimizationGoal,
    target: Option<f64>,
    algorithm: OptimizationAlgorithm,
    max_iterations: usize,
    cost_tolerance: f64,
    fd_step: f64,
    initial_step: f64,
    min_step: f64,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let mut configured_variables = Vec::with_capacity(variables.len());
    for variable in variables {
        super::ensure_not_aborted(abort)?;
        configured_variables.push(svc_runner::OptimizationVariable {
            name: variable.name,
            min: variable.min,
            max: variable.max,
            initial: variable.initial,
        });
    }
    let cfg = svc_runner::OptimizationRunConfig {
        variables: configured_variables,
        objective_node,
        objective_ref,
        goal: match goal {
            OptimizationGoal::Minimize => svc_runner::OptimizationGoalMode::Minimize,
            OptimizationGoal::Maximize => svc_runner::OptimizationGoalMode::Maximize,
            OptimizationGoal::Target => svc_runner::OptimizationGoalMode::Target,
        },
        target,
        algorithm: match algorithm {
            OptimizationAlgorithm::GradientDescent => {
                svc_runner::OptimizationAlgorithmMode::GradientDescent
            }
            OptimizationAlgorithm::PatternSearch => {
                svc_runner::OptimizationAlgorithmMode::PatternSearch
            }
            OptimizationAlgorithm::SimulatedAnnealing => {
                svc_runner::OptimizationAlgorithmMode::SimulatedAnnealing
            }
        },
        max_iterations,
        cost_tolerance,
        fd_step,
        initial_step,
        min_step,
    };

    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_optimization_analysis_with_config_and_source_path_and_abort(
            netlist,
            &cfg,
            source_path,
            abort,
        )
    })?;

    let mut waveforms = HashMap::new();
    super::ensure_not_aborted(abort)?;
    insert_scalar_waveform(
        &mut waveforms,
        "OPT_COST".to_string(),
        data.iterations.clone(),
        data.costs.clone(),
        "cost",
        "iter",
    );
    for (name, values) in &data.variable_traces {
        super::ensure_not_aborted(abort)?;
        insert_scalar_waveform(
            &mut waveforms,
            format!("OPT_{}", name),
            data.iterations.clone(),
            values.clone(),
            "value",
            "iter",
        );
    }

    Ok(SimulationResult::Optimization {
        iterations: data.iterations,
        waveforms,
        best_cost: data.best_cost,
        best_variables: data.best_variables,
        converged: data.converged,
    })
}

#[allow(clippy::too_many_arguments)]
fn run_soa(
    netlist: &str,
    stop_time: f64,
    step_time: f64,
    check_vgs_max: bool,
    max_vgs: f64,
    check_vds_max: bool,
    max_vds: f64,
    check_vbe_max: bool,
    max_vbe: f64,
    check_vce_max: bool,
    max_vce: f64,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let cfg = svc_runner::SoaRunConfig {
        stop_time,
        step_time,
        check_vgs_max,
        max_vgs,
        check_vds_max,
        max_vds,
        check_vbe_max,
        max_vbe,
        check_vce_max,
        max_vce,
    };
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_soa_analysis_with_config_and_source_path_and_abort(
            netlist,
            &cfg,
            source_path,
            abort,
        )
    })?;
    let mut waveforms = HashMap::new();
    super::ensure_not_aborted(abort)?;
    insert_scalar_waveform(
        &mut waveforms,
        "SOA_VIOLATION_COUNT".to_string(),
        data.time.clone(),
        data.violation_count.clone(),
        "count",
        "s",
    );

    Ok(SimulationResult::Soa {
        time: data.time,
        waveforms,
        violations: data.violations,
        evaluations: data.evaluations,
    })
}

fn insert_scalar_waveform(
    waveforms: &mut HashMap<String, WaveformData>,
    name: String,
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    y_unit: &str,
    x_unit: &str,
) {
    waveforms.insert(
        name.clone(),
        WaveformData {
            name,
            x_values,
            y_values,
            y_unit: y_unit.to_string(),
            x_unit: x_unit.to_string(),
            is_complex: false,
            y_imag: None,
        },
    );
}
