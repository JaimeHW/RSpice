use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, parse_runner_netlist_with_abort,
};
use crate::output_spec::{
    OutputSpec, OutputVoltageSpec, ac_output_value, collect_sensitivity_parameters,
    dc_output_value, normalized_sensitivity, parse_output_spec, resolve_sensitivity_ac_frequency,
    sensitivity_delta, validate_sensitivity_output_spec,
};
use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::engine::Engine;
use std::path::Path;

/// Sensitivity analysis data
#[derive(Debug, Clone)]
pub struct SensitivityData {
    /// Output variable that sensitivities are computed for
    pub output_var: String,
    /// (parameter_name, raw_sensitivity, normalized_sensitivity)
    pub sensitivities: Vec<(String, Value, Value)>,
}

/// Run sensitivity analysis against all global netlist parameters.
pub fn run_sensitivity_analysis(
    netlist_text: &str,
    output_var: &str,
    ac_mode: bool,
    frequency: Option<Value>,
) -> Result<SensitivityData, String> {
    run_sensitivity_analysis_with_abort(netlist_text, output_var, ac_mode, frequency, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run sensitivity analysis with cooperative cancellation.
pub fn run_sensitivity_analysis_with_abort(
    netlist_text: &str,
    output_var: &str,
    ac_mode: bool,
    frequency: Option<Value>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SensitivityData> {
    run_sensitivity_analysis_with_source_path_and_abort(
        netlist_text,
        output_var,
        ac_mode,
        frequency,
        None,
        abort,
    )
}

/// Run sensitivity analysis with a source path used to resolve relative
/// includes and model file references.
pub fn run_sensitivity_analysis_with_source_path(
    netlist_text: &str,
    output_var: &str,
    ac_mode: bool,
    frequency: Option<Value>,
    source_path: Option<&Path>,
) -> Result<SensitivityData, String> {
    run_sensitivity_analysis_with_source_path_and_abort(
        netlist_text,
        output_var,
        ac_mode,
        frequency,
        source_path,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run sensitivity analysis with source-path resolution and cooperative
/// cancellation through parsing, nominal solves, parameter perturbations, and
/// result conversion.
pub fn run_sensitivity_analysis_with_source_path_and_abort(
    netlist_text: &str,
    output_var: &str,
    ac_mode: bool,
    frequency: Option<Value>,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SensitivityData> {
    ensure_not_aborted(abort)?;
    let output_var = output_var.trim();
    if output_var.is_empty() {
        return Err(ServiceRunError::Failure(
            "Sensitivity output_var is required".to_string(),
        ));
    }
    let ac_frequency =
        resolve_sensitivity_ac_frequency(ac_mode, frequency).map_err(ServiceRunError::Failure)?;
    ensure_not_aborted(abort)?;

    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    ensure_not_aborted(abort)?;
    let engine = Engine::new(build_engine_config(&netlist, None));

    let circuit = engine.build_circuit(&netlist).map_err(|e| {
        ServiceRunError::from_core(
            "Circuit build error (required for sensitivity output resolution)",
            e,
        )
    })?;
    ensure_not_aborted(abort)?;
    let dc_result = engine
        .run_dc_op_with_abort(&netlist, abort)
        .map_err(|error| {
            ServiceRunError::from_core("DC OP error (required for sensitivity)", error)
        })?;

    ensure_not_aborted(abort)?;
    let output_spec =
        parse_output_spec(output_var, &dc_result.node_names, &circuit).ok_or_else(|| {
            ServiceRunError::Failure(format!(
                "Sensitivity output '{}' could not be resolved to a node or branch",
                output_var
            ))
        })?;
    ensure_not_aborted(abort)?;
    validate_sensitivity_output_spec(&output_spec).map_err(ServiceRunError::Failure)?;

    let nominal_output = if let Some(freq) = ac_frequency {
        run_ac_output_at_frequency_with_abort(&engine, &netlist, &output_spec, freq, abort)?.norm()
    } else {
        let output = dc_output_value(&dc_result, &output_spec).map_err(ServiceRunError::Failure)?;
        ensure_not_aborted(abort)?;
        output
    };

    let params = collect_sensitivity_parameters(&netlist);
    ensure_not_aborted(abort)?;
    if params.is_empty() {
        return Ok(SensitivityData {
            output_var: output_var.to_string(),
            sensitivities: Vec::new(),
        });
    }

    let mut sensitivities = Vec::new();
    let mut perturbed_netlist = netlist.clone();
    for (parameter_index, (name, value)) in params.into_iter().enumerate() {
        poll_periodically(abort, parameter_index)?;
        if !value.is_finite() || value == 0.0 {
            continue;
        }

        let raw = if let Some(freq) = ac_frequency {
            let result = finite_difference_derivative_with_abort(value, abort, |candidate| {
                ensure_not_aborted(abort)?;
                perturbed_netlist.params.set(&name, candidate);
                run_ac_output_at_frequency_with_abort(
                    &engine,
                    &perturbed_netlist,
                    &output_spec,
                    freq,
                    abort,
                )
                .map(|value| value.norm())
            });
            perturbed_netlist.params.set(&name, value);
            result.map_err(|error| contextualize_parameter_error(&name, error))?
        } else if let OutputSpec::Voltage(vspec) = &output_spec {
            run_dc_output_sensitivity_with_abort(&engine, &netlist, *vspec, &name, value, abort)
                .map_err(|error| contextualize_parameter_error(&name, error))?
        } else {
            let result = finite_difference_derivative_with_abort(value, abort, |candidate| {
                ensure_not_aborted(abort)?;
                perturbed_netlist.params.set(&name, candidate);
                let dc_result = engine
                    .run_dc_op_with_abort(&perturbed_netlist, abort)
                    .map_err(|error| {
                        ServiceRunError::from_core("DC OP error (perturbation)", error)
                    })?;
                ensure_not_aborted(abort)?;
                dc_output_value(&dc_result, &output_spec).map_err(ServiceRunError::Failure)
            });
            perturbed_netlist.params.set(&name, value);
            result.map_err(|error| contextualize_parameter_error(&name, error))?
        };

        ensure_not_aborted(abort)?;
        let normalized = normalized_sensitivity(raw, value, nominal_output);
        sensitivities.push((name, raw, normalized));
    }

    ensure_not_aborted(abort)?;
    sensitivities.sort_by(|a, b| {
        b.2.abs()
            .partial_cmp(&a.2.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    ensure_not_aborted(abort)?;

    Ok(SensitivityData {
        output_var: output_var.to_string(),
        sensitivities,
    })
}

fn contextualize_parameter_error(parameter: &str, error: ServiceRunError) -> ServiceRunError {
    match error {
        ServiceRunError::Aborted => ServiceRunError::Aborted,
        ServiceRunError::Failure(message) => ServiceRunError::Failure(format!(
            "Sensitivity error for parameter '{parameter}': {message}"
        )),
    }
}

fn run_ac_output_at_frequency_with_abort(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    output_spec: &OutputSpec,
    frequency: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Complex64> {
    ensure_not_aborted(abort)?;
    let ac_results = engine
        .run_ac_with_abort(netlist, &[frequency], abort)
        .map_err(|error| {
            ServiceRunError::from_core(&format!("AC analysis error at {frequency} Hz"), error)
        })?;
    ensure_not_aborted(abort)?;
    let point = ac_results.first().ok_or_else(|| {
        ServiceRunError::Failure(format!("AC analysis produced no data at {frequency} Hz"))
    })?;
    let output = ac_output_value(point, output_spec).map_err(ServiceRunError::Failure)?;
    ensure_not_aborted(abort)?;
    Ok(output)
}

fn run_dc_output_sensitivity_with_abort(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    output_spec: OutputVoltageSpec,
    param_name: &str,
    param_value: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Value> {
    ensure_not_aborted(abort)?;
    let pos_sensitivity = if output_spec.pos == 0 {
        0.0
    } else {
        engine
            .run_sensitivity_with_abort(
                netlist,
                output_spec.pos,
                param_name,
                param_value,
                None,
                abort,
            )
            .map_err(|error| ServiceRunError::from_core("DC sensitivity error", error))?
    };
    ensure_not_aborted(abort)?;

    let neg_sensitivity = match output_spec.neg {
        Some(0) | None => 0.0,
        Some(index) => engine
            .run_sensitivity_with_abort(netlist, index, param_name, param_value, None, abort)
            .map_err(|error| ServiceRunError::from_core("DC sensitivity error", error))?,
    };
    ensure_not_aborted(abort)?;
    Ok(pos_sensitivity - neg_sensitivity)
}

fn finite_difference_derivative_with_abort<F>(
    param_value: Value,
    abort: &dyn AbortSignal,
    mut evaluate_output: F,
) -> ServiceRunResult<Value>
where
    F: FnMut(Value) -> ServiceRunResult<Value>,
{
    ensure_not_aborted(abort)?;
    if !param_value.is_finite() {
        return Err(ServiceRunError::Failure(
            "Sensitivity parameter value must be finite".to_string(),
        ));
    }

    let delta = sensitivity_delta(param_value);
    let plus = evaluate_output(param_value + delta)?;
    ensure_not_aborted(abort)?;
    let minus = evaluate_output(param_value - delta)?;
    ensure_not_aborted(abort)?;
    if !plus.is_finite() || !minus.is_finite() {
        return Err(ServiceRunError::Failure(
            "Sensitivity perturbation produced non-finite outputs".to_string(),
        ));
    }

    let derivative = (plus - minus) / (2.0 * delta);
    if !derivative.is_finite() {
        return Err(ServiceRunError::Failure(
            "Sensitivity finite-difference derivative is non-finite".to_string(),
        ));
    }
    ensure_not_aborted(abort)?;
    Ok(derivative)
}

#[cfg(test)]
mod tests {
    use rspice_core::abort_signal::{AtomicAbort, ImmediateAbort};

    use super::*;

    #[test]
    fn sensitivity_runner_preserves_typed_abort_before_parse_or_validation() {
        let result = run_sensitivity_analysis_with_abort(
            "not a netlist",
            "",
            true,
            Some(-1.0),
            &ImmediateAbort,
        );

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn finite_difference_polls_between_parameter_perturbations() {
        let abort = AtomicAbort::new();
        let mut evaluations = 0;
        let result = finite_difference_derivative_with_abort(1.0, &abort, |_| {
            evaluations += 1;
            if evaluations == 1 {
                abort.set();
            }
            Ok(2.0)
        });

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert_eq!(evaluations, 1);
    }
}
