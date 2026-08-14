//! Sensitivity analysis over the engine bridge.

use std::collections::HashMap;

use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;

use super::{EngineBridge, ensure_not_aborted};
use crate::output_spec::{
    OutputSpec, OutputVoltageSpec, SensitivityMathError, ac_output_value,
    collect_sensitivity_parameters, dc_output_value, finite_difference_derivative,
    normalized_sensitivity, parse_output_spec, resolve_sensitivity_ac_frequency,
    validate_sensitivity_output_spec,
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
        if !nominal_value.is_finite() {
            return Err(SimulationError::SolverError(
                "Sensitivity nominal output is non-finite".to_owned(),
            ));
        }
        ensure_not_aborted(abort)?;

        let parameters = collect_sensitivity_parameters(netlist);
        ensure_not_aborted(abort)?;
        if parameters.is_empty() {
            return Err(SimulationError::InvalidConfig(
                "Sensitivity analysis found no eligible design parameters".to_owned(),
            ));
        }

        let mut sensitivities = HashMap::new();
        let mut normalized = HashMap::new();
        let mut perturbed_netlist = netlist.clone();

        for (param_name, param_value) in parameters {
            ensure_not_aborted(abort)?;
            if !param_value.is_finite() {
                return Err(SimulationError::InvalidConfig(format!(
                    "Sensitivity parameter '{param_name}' is non-finite"
                )));
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
                    Err(error) => return Err(error),
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
                            Err(error) => return Err(error),
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
                            Err(error) => return Err(error),
                        }
                    }
                }
            };

            ensure_not_aborted(abort)?;
            if !sensitivity.is_finite() {
                return Err(SimulationError::SolverError(format!(
                    "Sensitivity parameter '{param_name}' produced a non-finite derivative"
                )));
            }
            sensitivities.insert(param_name.clone(), sensitivity);
            if let Some(value) = normalized_sensitivity(sensitivity, param_value, nominal_value) {
                normalized.insert(param_name.clone(), value);
            }
        }

        ensure_not_aborted(abort)?;
        Ok(SimulationResult::Sensitivity {
            output: config.output_var.trim().to_owned(),
            ac_mode: config.ac_mode,
            frequency_hz: ac_frequency,
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

impl From<SensitivityMathError> for SimulationError {
    fn from(error: SensitivityMathError) -> Self {
        match error {
            SensitivityMathError::Aborted => Self::Aborted,
            other => Self::InvalidConfig(other.message().to_owned()),
        }
    }
}

fn finite_difference_derivative_with_abort<F>(
    param_value: Value,
    abort: &dyn AbortSignal,
    evaluate_output: F,
) -> Result<Value, SimulationError>
where
    F: FnMut(Value) -> Result<Value, SimulationError>,
{
    finite_difference_derivative(param_value, abort, evaluate_output)
}

#[cfg(test)]
mod tests {
    use rspice_core::abort_signal::{AtomicAbort, ImmediateAbort, NoAbort};

    use super::*;

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

        assert!(matches!(result, Err(SimulationError::Aborted)));
        assert_eq!(evaluations, 1);
    }

    #[test]
    fn finite_difference_rejects_a_non_finite_parameter() {
        let result = finite_difference_derivative_with_abort(Value::NAN, &NoAbort, Ok);
        assert!(matches!(result, Err(SimulationError::InvalidConfig(_))));
    }

    #[test]
    fn finite_difference_preserves_a_typed_abort_before_any_evaluation() {
        let mut evaluations = 0;
        let result = finite_difference_derivative_with_abort(1.0, &ImmediateAbort, |value| {
            evaluations += 1;
            Ok(value)
        });

        assert!(matches!(result, Err(SimulationError::Aborted)));
        assert_eq!(evaluations, 0, "an aborted run must not touch the engine");
    }
}
