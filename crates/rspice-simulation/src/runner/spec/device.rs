//! Dispatch for device-level analyses.

use std::collections::HashMap;

use crate::engine_services as svc_runner;
use crate::error::SimulationError;
use crate::results::{SimulationResult, WaveformData};
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_simulation_contract::analysis_spec::OptimizationAlgorithm;
use rspice_simulation_contract::analysis_spec::OptimizationGoal;

pub(super) fn run_device_spec(
    spec: AnalysisSpec,
    netlist: &str,
    study_base: Option<&crate::study::StudyRunConfig>,
    environment: Option<crate::runner::AnalysisExecutionEnvironment>,
    context: svc_runner::ServiceContext<'_>,
) -> Result<SimulationResult, SimulationError> {
    let abort = context.abort;
    super::ensure_not_aborted(abort)?;
    match spec {
        AnalysisSpec::Optimization { .. } => {
            run_optimization(spec, netlist, study_base, environment, context)
        }
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
            svc_runner::SoaRunConfig {
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
            },
            context,
        ),
        // Dispatched by the whole specification rather than destructured
        // here: the card is written by the one writer the Analyses page also
        // displays, so the run cannot ask for a study the page did not state.
        ref dc_mismatch @ AnalysisSpec::DcMismatch { .. } => {
            run_dc_mismatch(netlist, dc_mismatch, context)
        }
        other => Err(super::misrouted_spec_error("device", &other)),
    }
}

/// Solve one DC mismatch spread and retain it as typed evidence.
fn run_dc_mismatch(
    netlist: &str,
    spec: &AnalysisSpec,
    context: svc_runner::ServiceContext<'_>,
) -> Result<SimulationResult, SimulationError> {
    let abort = context.abort;
    let AnalysisSpec::DcMismatch {
        normalized_contributions,
        ..
    } = spec
    else {
        return Err(super::misrouted_spec_error("device", spec));
    };
    let card_line = crate::analysis_preparation::build_dc_mismatch_command(spec)
        .map_err(SimulationError::InvalidConfig)?;
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_dc_mismatch_analysis_with_context(netlist, &card_line, context)
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
    let evidence = rspice_results::dc_mismatch::DcMismatchEvidence {
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
            .map(
                |row| rspice_results::dc_mismatch::DcMismatchContributorEvidence {
                    instance: row.instance.clone(),
                    parameter: row.parameter.clone(),
                    scope: match row.scope {
                        rspice_core::analysis::dcmatch::DcMatchScope::Mismatch => {
                            rspice_results::dc_mismatch::DcMismatchScopeEvidence::Mismatch
                        }
                        rspice_core::analysis::dcmatch::DcMatchScope::Process => {
                            rspice_results::dc_mismatch::DcMismatchScopeEvidence::Process
                        }
                    },
                    sigma_parameter: row.sigma_parameter,
                    sensitivity: row.sensitivity,
                    contribution: row.contribution,
                    share: row.share,
                },
            )
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
    spec: AnalysisSpec,
    netlist: &str,
    study_base: Option<&crate::study::StudyRunConfig>,
    environment: Option<crate::runner::AnalysisExecutionEnvironment>,
    context: svc_runner::ServiceContext<'_>,
) -> Result<SimulationResult, SimulationError> {
    let abort = context.abort;
    let AnalysisSpec::Optimization {
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
    } = spec
    else {
        return Err(super::misrouted_spec_error("optimization", &spec));
    };
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
        crate::runner::study::run_optimization_with_context(
            base,
            &cfg,
            netlist,
            environment,
            context,
        )?
    } else {
        let environment = environment.map(|point| rspice_core::engine::MonteCarloEnvironment {
            temperature_celsius: point.temperature_celsius,
            supply_voltage: point.supply_voltage,
            nominal_supply_voltage: point.nominal_supply_voltage,
            supply_source_names: point.supply_source_names,
        });
        super::run_abort_aware_service(abort, || {
            svc_runner::run_optimization_analysis_with_context(
                netlist,
                &cfg,
                environment.as_ref(),
                context,
            )
        })?
    };

    let result_values = data
        .variable_traces
        .values()
        .fold(
            data.iterations
                .len()
                .saturating_mul(2)
                .saturating_add(data.costs.len()),
            |count, values| {
                count
                    .saturating_add(data.iterations.len())
                    .saturating_add(values.len())
            },
        )
        .saturating_add(data.best_variables.len())
        .saturating_add(1)
        .saturating_add(data.best_objectives.len().saturating_mul(5))
        .saturating_add(data.best_constraints.len().saturating_mul(6));
    if result_values > context.limits.max_result_values {
        return Err(SimulationError::ResourceLimit {
            resource: "result_values".into(),
            requested: result_values,
            limit: context.limits.max_result_values,
        });
    }

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
    cfg: svc_runner::SoaRunConfig,
    context: svc_runner::ServiceContext<'_>,
) -> Result<SimulationResult, SimulationError> {
    let abort = context.abort;
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_soa_analysis_with_context(netlist, &cfg, context)
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
                rspice_results::safety::soa_envelope_limit_waveform_name(
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
                rspice_results::safety::soa_envelope_voltage_waveform_name(
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
                rspice_results::safety::soa_power_limit_waveform_name(&trace.device_id),
                data.time.clone(),
                derating.limits_w.clone(),
                "W",
                "s",
            );
            insert_scalar_waveform(
                &mut waveforms,
                rspice_results::safety::soa_derating_temperature_waveform_name(&trace.device_id),
                data.time.clone(),
                derating.temperatures_kelvin.clone(),
                "K",
                "s",
            );
        }
        insert_scalar_waveform(
            &mut waveforms,
            rspice_results::safety::soa_stress_waveform_name(&trace.device_id, trace.parameter),
            data.time.clone(),
            trace.values.clone(),
            &trace.unit,
            "s",
        );
    }

    let (time, source_history) = if let Some(reporting) = data.reporting {
        let mut source = rspice_results::soa_source::SoaSourceHistory {
            time: data.time,
            waveforms: waveforms
                .drain()
                .map(
                    |(name, wave)| rspice_results::soa_source::SoaSourceWaveform {
                        name,
                        unit: wave.y_unit,
                        values: wave.y_values,
                    },
                )
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

#[cfg(test)]
mod policy_tests {
    use super::*;

    #[test]
    fn optimization_admits_all_published_history_columns_before_copying() {
        let spec: AnalysisSpec = serde_json::from_value(serde_json::json!({"Optimization": {
            "variables": [{"name": "R", "min": 1.0, "max": 2.0, "initial": 1.5}],
            "objective_node": "out", "objective_ref": "0", "goal": "Target",
            "target": 0.5, "algorithm": "PatternSearch", "max_iterations": 4,
            "cost_tolerance": 1e-6, "fd_step": 1e-3, "initial_step": 0.25, "min_step": 1e-6
        }}))
        .unwrap();
        let deck =
            "Optimization projection\n.param R=1.5\nV1 in 0 1\nR1 in out {R}\nR2 out 0 1\n.end\n";
        let mut limits = rspice_core::ResourceLimits::default();
        let run = |limits| {
            run_device_spec(
                spec.clone(),
                deck,
                None,
                None,
                svc_runner::ServiceContext {
                    source_path: None,
                    limits,
                    abort: &rspice_core::NoAbort,
                },
            )
        };
        let SimulationResult::Optimization { iterations, .. } = run(limits).unwrap() else {
            panic!("optimization result")
        };
        let retained = iterations.len() * 5 + 2;
        // The optimizer reserves fifteen history values. The completed plot
        // also owns its axes and best-point scalars; the core DC solve fits.
        assert!(retained > 15);
        limits.max_result_values = 15;
        let error = run(limits).unwrap_err();
        assert!(
            matches!(&error, SimulationError::ResourceLimit { resource, requested, limit: 15 }
            if resource == "result_values" && *requested == retained),
            "{error:?}, retained={retained}"
        );
    }
}
