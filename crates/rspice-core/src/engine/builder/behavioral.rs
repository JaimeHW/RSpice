use super::*;
use crate::Value;

pub(in crate::engine::builder) fn expression_references_circuit_state(expression: &str) -> bool {
    crate::netlist::expr::behavioral_expression_references_runtime_quantity(expression)
}

pub(in crate::engine::builder) fn temperature_scale_factor(
    current_temp_c: f64,
    tnom_c: f64,
    tc1: f64,
    tc2: f64,
) -> f64 {
    let delta_t = current_temp_c - tnom_c;
    1.0 + tc1 * delta_t + tc2 * delta_t * delta_t
}

pub(in crate::engine::builder) fn prepare_temperature_scaled_behavioral_expression(
    expression: &str,
    params: &crate::netlist::ParamContext,
    temperature_kelvin: f64,
    tnom_c: f64,
    tc1: f64,
    tc2: f64,
) -> Result<String, SimulationError> {
    let prepared = prepare_behavioral_expression(expression, params)
        .map_err(|e| SimulationError::Circuit(format!("Behavioral expression: {e}")))?;
    if tc1 == 0.0 && tc2 == 0.0 {
        return Ok(prepared);
    }

    let current_temp_c = crate::analysis::temperature::kelvin_to_celsius(temperature_kelvin);
    let scale = temperature_scale_factor(current_temp_c, tnom_c, tc1, tc2);
    Ok(format!("(({})*{})", prepared, scale))
}

pub(in crate::engine::builder) fn add_behavioral_resistor(
    circuit: &mut CircuitData,
    netlist: &Netlist,
    element: &crate::netlist::Element,
    expression: &str,
    model_name: Option<&str>,
    instance_params: &[(String, f64)],
    temperature_kelvin: f64,
    expression_gmin: Value,
    resource_limits: ResourceLimits,
    spice_dialect: SpiceDialect,
) -> Result<(), SimulationError> {
    let np = circuit.get_or_create_node(&element.nodes[0]);
    let nn = circuit.get_or_create_node(&element.nodes[1]);
    let policy = resolve_behavioral_resistor_policy(
        netlist,
        &element.name,
        model_name,
        instance_params,
        temperature_kelvin,
        spice_dialect,
    )?;
    if !policy.scale.is_finite() || policy.scale == 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Resistor '{}' resolved to unsupported zero or non-finite behavioral resistance scale {}",
            element.name, policy.scale
        )));
    }

    let params = base_eval_context(netlist);
    let prepared = prepare_behavioral_expression(expression, &params).map_err(|e| {
        SimulationError::Circuit(format!(
            "Resistor '{}' behavioral value expression could not be prepared: {}",
            element.name, e
        ))
    })?;
    let current_expression = format!(
        "(V({},{})/(({})*{}))",
        element.nodes[0], element.nodes[1], prepared, policy.scale
    );

    let mut bcs = crate::device::BehavioralCurrentSource::new_with_source_path_and_limits(
        element.name.clone(),
        np,
        nn,
        &current_expression,
        netlist.source_path.as_deref(),
        resource_limits,
    )
    .map_err(SimulationError::Circuit)?;
    bcs.set_expression_dialect(netlist.params.expression_dialect());
    bcs.set_temperature(policy.temperature_celsius);
    bcs.set_gmin(expression_gmin);
    bcs.enable_two_terminal_observables();
    circuit.behavioral_sources.add_current(bcs);
    Ok(())
}
