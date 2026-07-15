use std::collections::HashMap;

use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;

use super::{EngineBridge, ensure_not_aborted};
use crate::output_spec::{
    OutputSpec, OutputVoltageSpec, ac_output_value, collect_sensitivity_parameters,
    dc_output_value, normalized_sensitivity, parse_output_spec, resolve_sensitivity_ac_frequency,
    sensitivity_delta, validate_sensitivity_output_spec,
};
use crate::simulation::config::SensitivityConfig;
use crate::simulation::results::SimulationResult;
use crate::simulation::runner::SimulationError;

impl EngineBridge {
    /// Run sensitivity analysis.
    pub(super) fn run_sensitivity(
        &self,
        netlist: &rspice_core::Netlist,
        config: &SensitivityConfig,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        ensure_not_aborted(abort)?;
        let engine = self.engine_for_netlist(netlist);
        let ac_frequency = resolve_sensitivity_ac_frequency(config.ac_mode, config.frequency);
        ensure_not_aborted(abort)?;
        let ac_frequency = ac_frequency.map_err(SimulationError::InvalidConfig)?;

        let dc_result = engine
            .run_dc_op_with_abort(netlist, abort)
            .map_err(|e| self.translate_error(e))?;
        ensure_not_aborted(abort)?;
        let circuit = engine
            .build_circuit(netlist)
            .map_err(|e| self.translate_error(e))?;
        ensure_not_aborted(abort)?;
        let output_spec = parse_output_spec(&config.output_var, &dc_result.node_names, &circuit);
        ensure_not_aborted(abort)?;
        let output_spec = output_spec.ok_or_else(|| {
            SimulationError::InvalidConfig(format!(
                "Sensitivity output '{}' could not be resolved to a node or branch",
                config.output_var
            ))
        })?;
        let output_validation = validate_sensitivity_output_spec(&output_spec);
        ensure_not_aborted(abort)?;
        output_validation.map_err(SimulationError::InvalidConfig)?;

        let nominal_value = if let Some(freq) = ac_frequency {
            self.run_ac_output_at_frequency_with_abort(&engine, netlist, &output_spec, freq, abort)?
                .norm()
        } else {
            let output = dc_output_value(&dc_result, &output_spec);
            ensure_not_aborted(abort)?;
            output.map_err(SimulationError::InvalidConfig)?
        };
        ensure_not_aborted(abort)?;

        let parameters = collect_sensitivity_parameters(netlist);
        ensure_not_aborted(abort)?;
        if parameters.is_empty() {
            return Ok(SimulationResult::Sensitivity {
                sensitivities: HashMap::new(),
                normalized: HashMap::new(),
            });
        }

        let mut sensitivities = HashMap::new();
        let mut normalized = HashMap::new();
        let mut perturbed_netlist = netlist.clone();

        for (param_name, param_value) in parameters {
            ensure_not_aborted(abort)?;
            if !param_value.is_finite() || param_value == 0.0 {
                continue;
            }

            let sensitivity = if let Some(freq) = ac_frequency {
                let result =
                    finite_difference_derivative_with_abort(param_value, abort, |candidate| {
                        ensure_not_aborted(abort)?;
                        perturbed_netlist.params.set(&param_name, candidate);
                        self.run_ac_output_at_frequency_with_abort(
                            &engine,
                            &perturbed_netlist,
                            &output_spec,
                            freq,
                            abort,
                        )
                        .map(|value| value.norm())
                    });
                perturbed_netlist.params.set(&param_name, param_value);
                match result {
                    Ok(raw) => raw,
                    Err(SimulationError::Aborted) => return Err(SimulationError::Aborted),
                    Err(_) => continue,
                }
            } else {
                match &output_spec {
                    OutputSpec::Voltage(vspec) => {
                        match self.run_dc_output_sensitivity_with_abort(
                            &engine,
                            netlist,
                            *vspec,
                            &param_name,
                            param_value,
                            abort,
                        ) {
                            Ok(raw) => raw,
                            Err(SimulationError::Aborted) => {
                                return Err(SimulationError::Aborted);
                            }
                            Err(_) => continue,
                        }
                    }
                    OutputSpec::BranchCurrent { .. } => {
                        let result = finite_difference_derivative_with_abort(
                            param_value,
                            abort,
                            |candidate| {
                                ensure_not_aborted(abort)?;
                                perturbed_netlist.params.set(&param_name, candidate);
                                let dc_result = engine
                                    .run_dc_op_with_abort(&perturbed_netlist, abort)
                                    .map_err(|error| self.translate_error(error))?;
                                dc_output_value(&dc_result, &output_spec)
                                    .map_err(SimulationError::InvalidConfig)
                            },
                        );
                        perturbed_netlist.params.set(&param_name, param_value);
                        match result {
                            Ok(raw) => raw,
                            Err(SimulationError::Aborted) => {
                                return Err(SimulationError::Aborted);
                            }
                            Err(_) => continue,
                        }
                    }
                }
            };

            ensure_not_aborted(abort)?;
            sensitivities.insert(param_name.clone(), sensitivity);
            normalized.insert(
                param_name.clone(),
                normalized_sensitivity(sensitivity, param_value, nominal_value),
            );
        }

        ensure_not_aborted(abort)?;
        Ok(SimulationResult::Sensitivity {
            sensitivities,
            normalized,
        })
    }

    fn run_ac_output_at_frequency_with_abort(
        &self,
        engine: &rspice_core::Engine,
        netlist: &rspice_core::Netlist,
        output_spec: &OutputSpec,
        frequency: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Complex64, SimulationError> {
        ensure_not_aborted(abort)?;
        let ac_results = engine
            .run_ac_with_abort(netlist, &[frequency], abort)
            .map_err(|error| self.translate_error(error))?;
        ensure_not_aborted(abort)?;
        let point = ac_results.first().ok_or_else(|| {
            SimulationError::InvalidConfig(format!(
                "AC analysis produced no data at {frequency} Hz"
            ))
        })?;
        let output = ac_output_value(point, output_spec);
        ensure_not_aborted(abort)?;
        output.map_err(SimulationError::InvalidConfig)
    }

    fn run_dc_output_sensitivity_with_abort(
        &self,
        engine: &rspice_core::Engine,
        netlist: &rspice_core::Netlist,
        output_spec: OutputVoltageSpec,
        param_name: &str,
        param_value: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Value, SimulationError> {
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
                .map_err(|error| self.translate_error(error))?
        };
        ensure_not_aborted(abort)?;

        let neg_sensitivity = match output_spec.neg {
            Some(0) | None => 0.0,
            Some(index) => engine
                .run_sensitivity_with_abort(netlist, index, param_name, param_value, None, abort)
                .map_err(|error| self.translate_error(error))?,
        };
        ensure_not_aborted(abort)?;
        Ok(pos_sensitivity - neg_sensitivity)
    }
}

fn finite_difference_derivative_with_abort<F>(
    param_value: Value,
    abort: &dyn AbortSignal,
    mut evaluate_output: F,
) -> Result<Value, SimulationError>
where
    F: FnMut(Value) -> Result<Value, SimulationError>,
{
    ensure_not_aborted(abort)?;
    if !param_value.is_finite() {
        return Err(SimulationError::InvalidConfig(
            "Sensitivity parameter value must be finite".to_string(),
        ));
    }

    let delta = sensitivity_delta(param_value);
    let plus = evaluate_output(param_value + delta)?;
    ensure_not_aborted(abort)?;
    let minus = evaluate_output(param_value - delta)?;
    ensure_not_aborted(abort)?;
    if !plus.is_finite() || !minus.is_finite() {
        return Err(SimulationError::InvalidConfig(
            "Sensitivity perturbation produced non-finite outputs".to_string(),
        ));
    }

    let derivative = (plus - minus) / (2.0 * delta);
    if !derivative.is_finite() {
        return Err(SimulationError::InvalidConfig(
            "Sensitivity finite-difference derivative is non-finite".to_string(),
        ));
    }
    Ok(derivative)
}
