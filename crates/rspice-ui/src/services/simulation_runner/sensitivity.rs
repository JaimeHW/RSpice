use super::build_engine_config;
use crate::output_spec::{
    collect_sensitivity_parameters, dc_output_value, finite_difference_derivative,
    normalized_sensitivity, parse_output_spec, resolve_sensitivity_ac_frequency,
    run_ac_output_at_frequency, run_dc_output_sensitivity, validate_sensitivity_output_spec,
    OutputSpec,
};
use rspice_core::engine::Engine;
use rspice_core::Value;

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
    let output_var = output_var.trim();
    if output_var.is_empty() {
        return Err("Sensitivity output_var is required".to_string());
    }
    let ac_frequency = resolve_sensitivity_ac_frequency(ac_mode, frequency)?;

    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;
    let engine = Engine::new(build_engine_config(&netlist, None));

    let circuit = engine.build_circuit(&netlist).map_err(|e| {
        format!(
            "Circuit build error (required for sensitivity output resolution): {}",
            e
        )
    })?;
    let dc_result = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC OP error (required for sensitivity): {}", e))?;

    let output_spec =
        parse_output_spec(output_var, &dc_result.node_names, &circuit).ok_or_else(|| {
            format!(
                "Sensitivity output '{}' could not be resolved to a node or branch",
                output_var
            )
        })?;
    validate_sensitivity_output_spec(&output_spec)?;

    let nominal_output = if let Some(freq) = ac_frequency {
        run_ac_output_at_frequency(&engine, &netlist, &output_spec, freq)
            .map(|value| value.norm())?
    } else {
        dc_output_value(&dc_result, &output_spec)?
    };

    let params = collect_sensitivity_parameters(&netlist);
    if params.is_empty() {
        return Ok(SensitivityData {
            output_var: output_var.to_string(),
            sensitivities: Vec::new(),
        });
    }

    let mut sensitivities = Vec::new();
    let mut perturbed_netlist = netlist.clone();
    for (name, value) in params {
        if !value.is_finite() || value == 0.0 {
            continue;
        }

        let raw = if let Some(freq) = ac_frequency {
            let result = finite_difference_derivative(value, |candidate| {
                perturbed_netlist.params.set(&name, candidate);
                run_ac_output_at_frequency(&engine, &perturbed_netlist, &output_spec, freq)
                    .map(|value| value.norm())
            });
            perturbed_netlist.params.set(&name, value);
            result.map_err(|e| format!("Sensitivity error for parameter '{}': {}", name, e))?
        } else if let OutputSpec::Voltage(vspec) = &output_spec {
            run_dc_output_sensitivity(&engine, &netlist, *vspec, &name, value)
                .map_err(|e| format!("Sensitivity error for parameter '{}': {}", name, e))?
        } else {
            let result = finite_difference_derivative(value, |candidate| {
                perturbed_netlist.params.set(&name, candidate);
                let dc_result = engine
                    .run_dc_op(&perturbed_netlist)
                    .map_err(|e| format!("DC OP error (perturbation): {}", e))?;
                dc_output_value(&dc_result, &output_spec)
            });
            perturbed_netlist.params.set(&name, value);
            result.map_err(|e| format!("Sensitivity error for parameter '{}': {}", name, e))?
        };

        let normalized = normalized_sensitivity(raw, value, nominal_output);
        sensitivities.push((name, raw, normalized));
    }

    sensitivities.sort_by(|a, b| {
        b.2.abs()
            .partial_cmp(&a.2.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(SensitivityData {
        output_var: output_var.to_string(),
        sensitivities,
    })
}
