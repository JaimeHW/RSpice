use std::collections::HashMap;

use super::EngineBridge;
use crate::output_spec::{
    OutputSpec, collect_sensitivity_parameters, dc_output_value, finite_difference_derivative,
    normalized_sensitivity, parse_output_spec, resolve_sensitivity_ac_frequency,
    run_ac_output_at_frequency, run_dc_output_sensitivity, validate_sensitivity_output_spec,
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
    ) -> Result<SimulationResult, SimulationError> {
        let engine = self.engine_for_netlist(netlist);
        let ac_frequency = resolve_sensitivity_ac_frequency(config.ac_mode, config.frequency)
            .map_err(SimulationError::InvalidConfig)?;

        let dc_result = engine
            .run_dc_op(netlist)
            .map_err(|e| self.translate_error(e))?;
        let circuit = engine
            .build_circuit(netlist)
            .map_err(|e| self.translate_error(e))?;
        let output_spec = parse_output_spec(&config.output_var, &dc_result.node_names, &circuit)
            .ok_or_else(|| {
                SimulationError::InvalidConfig(format!(
                    "Sensitivity output '{}' could not be resolved to a node or branch",
                    config.output_var
                ))
            })?;
        validate_sensitivity_output_spec(&output_spec).map_err(SimulationError::InvalidConfig)?;

        let nominal_value = if let Some(freq) = ac_frequency {
            run_ac_output_at_frequency(&engine, netlist, &output_spec, freq)
                .map_err(SimulationError::InvalidConfig)?
                .norm()
        } else {
            dc_output_value(&dc_result, &output_spec).map_err(SimulationError::InvalidConfig)?
        };

        let parameters = collect_sensitivity_parameters(netlist);
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
            if !param_value.is_finite() || param_value == 0.0 {
                continue;
            }

            let sensitivity = if let Some(freq) = ac_frequency {
                let result = finite_difference_derivative(param_value, |candidate| {
                    perturbed_netlist.params.set(&param_name, candidate);
                    run_ac_output_at_frequency(&engine, &perturbed_netlist, &output_spec, freq)
                        .map(|value| value.norm())
                });
                perturbed_netlist.params.set(&param_name, param_value);
                match result {
                    Ok(raw) => raw,
                    Err(_) => continue,
                }
            } else {
                match &output_spec {
                    OutputSpec::Voltage(vspec) => {
                        match run_dc_output_sensitivity(
                            &engine,
                            netlist,
                            *vspec,
                            &param_name,
                            param_value,
                        ) {
                            Ok(raw) => raw,
                            Err(_) => continue,
                        }
                    }
                    OutputSpec::BranchCurrent { .. } => {
                        let result = finite_difference_derivative(param_value, |candidate| {
                            perturbed_netlist.params.set(&param_name, candidate);
                            let dc_result = engine
                                .run_dc_op(&perturbed_netlist)
                                .map_err(|e| e.to_string())?;
                            dc_output_value(&dc_result, &output_spec)
                        });
                        perturbed_netlist.params.set(&param_name, param_value);
                        match result {
                            Ok(raw) => raw,
                            Err(_) => continue,
                        }
                    }
                }
            };

            sensitivities.insert(param_name.clone(), sensitivity);
            normalized.insert(
                param_name.clone(),
                normalized_sensitivity(sensitivity, param_value, nominal_value),
            );
        }

        Ok(SimulationResult::Sensitivity {
            sensitivities,
            normalized,
        })
    }
}
