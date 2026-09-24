//! Dispatch for device-level analyses.

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
    study_base: Option<&crate::simulation::runner::study::StudyRunConfig>,
    environment: Option<crate::simulation::runner::AnalysisExecutionEnvironment>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    super::ensure_not_aborted(abort)?;
    match spec {
        AnalysisSpec::Optimization {
            search,
            variables,
            objective_unit,
            objective_expression,
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
            search,
            variables,
            objective_unit,
            objective_expression,
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
            study_base,
            environment,
            abort,
        ),
        AnalysisSpec::Soa {
            import_model_voltage_ratings,
            observation,
            rules,
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
            import_model_voltage_ratings,
            observation,
            rules,
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
        // Dispatched by the whole specification rather than destructured
        // here: the card is written by the one writer the Analyses page also
        // displays, so the run cannot ask for a study the page did not state.
        ref dc_mismatch @ AnalysisSpec::DcMismatch { .. } => {
            run_dc_mismatch(netlist, dc_mismatch, source_path, abort)
        }
        other => Err(super::misrouted_spec_error("device", &other)),
    }
}

/// Solve one DC mismatch spread and retain it as typed evidence.
fn run_dc_mismatch(
    netlist: &str,
    spec: &AnalysisSpec,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let AnalysisSpec::DcMismatch {
        normalized_contributions,
        ..
    } = spec
    else {
        return Err(super::misrouted_spec_error("device", spec));
    };
    let card_line = crate::simulation::SimulationController::build_dc_mismatch_command(spec)
        .map_err(SimulationError::InvalidConfig)?;
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_dc_mismatch_analysis_with_source_path_and_abort(
            netlist,
            &card_line,
            source_path,
            abort,
        )
    })?;
    super::ensure_not_aborted(abort)?;

    let result = &data.result;
    let count = |value: usize| -> Result<u64, SimulationError> {
        u64::try_from(value).map_err(|_| {
            SimulationError::InvalidConfig(
                "the DC mismatch report counted more contributors than can be retained".to_owned(),
            )
        })
    };
    // The unit follows the probe the engine echoed back, which is the rule
    // the core result document uses for the same scalars.
    let output_unit = if result.output.starts_with("I(") {
        "A"
    } else {
        "V"
    };
    let evidence = crate::state::DcMismatchEvidence {
        output: result.output.clone(),
        output_unit: output_unit.to_owned(),
        nominal_value: result.nominal_value,
        sigma_multiplier: result.sigma_multiplier,
        sigma_total: result.sigma_total,
        sigma_mismatch: result.sigma_mismatch,
        sigma_process: result.sigma_process,
        include_mismatch: data.card.mismatch,
        include_process: data.card.process,
        contributor_limit: count(data.card.contributor_limit)?,
        threshold: data.card.threshold,
        normalized_contributions: *normalized_contributions,
        applied_correlations_mismatch: count(result.applied_correlations_mismatch)?,
        applied_correlations_process: count(result.applied_correlations_process)?,
        evaluated_contributors: count(result.evaluated_contributors)?,
        contributors: result
            .contributors
            .iter()
            .map(|row| crate::state::DcMismatchContributorEvidence {
                instance: row.instance.clone(),
                parameter: row.parameter.clone(),
                scope: match row.scope {
                    rspice_core::analysis::dcmatch::DcMatchScope::Mismatch => {
                        crate::state::DcMismatchScopeEvidence::Mismatch
                    }
                    rspice_core::analysis::dcmatch::DcMatchScope::Process => {
                        crate::state::DcMismatchScopeEvidence::Process
                    }
                },
                sigma_parameter: row.sigma_parameter,
                sensitivity: row.sensitivity,
                contribution: row.contribution,
                share: row.share,
            })
            .collect(),
    };
    evidence
        .validate()
        .map_err(SimulationError::InvalidConfig)?;
    Ok(SimulationResult::DcMismatch {
        evidence: std::sync::Arc::new(evidence),
    })
}

fn run_optimization(
    netlist: &str,
    search: svc_runner::OptimizationSearchControls,
    variables: Vec<crate::simulation::multi_run::OptimizationVariable>,
    objective_unit: String,
    objective_expression: Option<String>,
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
    study_base: Option<&crate::simulation::runner::study::StudyRunConfig>,
    environment: Option<crate::simulation::runner::AnalysisExecutionEnvironment>,
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
        search,
        variables: configured_variables,
        objective_unit,
        objective_expression,
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

    let data = if let Some(base) = study_base {
        crate::simulation::runner::study::run_optimization(
            base,
            &cfg,
            netlist,
            source_path,
            environment,
            abort,
        )?
    } else {
        let environment = environment.map(|point| rspice_core::engine::MonteCarloEnvironment {
            temperature_celsius: point.temperature_celsius,
            supply_voltage: point.supply_voltage,
            nominal_supply_voltage: point.nominal_supply_voltage,
            supply_source_names: point.supply_source_names,
        });
        super::run_abort_aware_service(abort, || match environment.as_ref() {
            Some(point) => {
                svc_runner::run_optimization_analysis_with_environment_and_source_path_and_abort(
                    netlist,
                    &cfg,
                    source_path,
                    Some(point),
                    abort,
                )
            }
            None => svc_runner::run_optimization_analysis_with_config_and_source_path_and_abort(
                netlist,
                &cfg,
                source_path,
                abort,
            ),
        })?
    };

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
        best_objectives: data.best_objectives,
        best_constraints: data.best_constraints,
        converged: data.converged,
    })
}

fn run_soa(
    netlist: &str,
    import_model_voltage_ratings: bool,
    observation: svc_runner::SoaObservationConfig,
    rules: Vec<svc_runner::SoaRuleConfig>,
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
        import_model_voltage_ratings,
        observation,
        rules,
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
    for trace in &data.stress_history {
        super::ensure_not_aborted(abort)?;
        if let Some(envelope) = &trace.envelope {
            insert_scalar_waveform(
                &mut waveforms,
                crate::results::safety::soa_envelope_limit_waveform_name(
                    &trace.device_id,
                    trace.parameter,
                ),
                data.time.clone(),
                envelope.limits_a.clone(),
                "A",
                "s",
            );
            insert_scalar_waveform(
                &mut waveforms,
                crate::results::safety::soa_envelope_voltage_waveform_name(
                    &trace.device_id,
                    trace.parameter,
                ),
                data.time.clone(),
                envelope.voltages_v.clone(),
                "V",
                "s",
            );
        }
        if let Some(derating) = &trace.derating {
            insert_scalar_waveform(
                &mut waveforms,
                crate::results::safety::soa_power_limit_waveform_name(&trace.device_id),
                data.time.clone(),
                derating.limits_w.clone(),
                "W",
                "s",
            );
            insert_scalar_waveform(
                &mut waveforms,
                crate::results::safety::soa_derating_temperature_waveform_name(&trace.device_id),
                data.time.clone(),
                derating.temperatures_kelvin.clone(),
                "K",
                "s",
            );
        }
        insert_scalar_waveform(
            &mut waveforms,
            crate::results::safety::soa_stress_waveform_name(&trace.device_id, trace.parameter),
            data.time.clone(),
            trace.values.clone(),
            &trace.unit,
            "s",
        );
    }

    let (time, source_history) = if let Some(reporting) = data.reporting {
        let mut source = crate::state::SoaSourceHistory {
            time: data.time,
            waveforms: waveforms
                .drain()
                .map(|(name, wave)| crate::state::SoaSourceWaveform {
                    name,
                    unit: wave.y_unit,
                    values: wave.y_values,
                })
                .collect(),
        };
        source.waveforms.sort_by(|a, b| a.name.cmp(&b.name));
        for wave in &source.waveforms {
            super::ensure_not_aborted(abort)?;
            let values = source
                .report_values(wave, &reporting)
                .map_err(SimulationError::SolverError)?;
            insert_scalar_waveform(
                &mut waveforms,
                wave.name.clone(),
                reporting.times().to_vec(),
                values,
                &wave.unit,
                "s",
            );
        }
        (
            reporting.times().to_vec(),
            Some(std::sync::Arc::new(source)),
        )
    } else {
        (data.time, None)
    };
    Ok(SimulationResult::Soa {
        source_history,
        convergence: data.convergence,
        time,
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
    _x_unit: &str,
) {
    waveforms.insert(
        name.clone(),
        WaveformData {
            name,
            x_values,
            y_values,
            y_unit: y_unit.to_string(),
            is_complex: false,
            y_imag: None,
        },
    );
}

#[cfg(test)]
mod soa_reporting_tests;
