use super::*;

pub(in crate::engine::builder) fn expression_references_circuit_state(expression: &str) -> bool {
    let upper = expression.to_ascii_uppercase();
    upper.contains("V(") || upper.contains("I(")
}

pub(in crate::engine::builder) fn temperature_param_to_celsius(value: f64) -> f64 {
    if value > 200.0 {
        crate::analysis::temperature::kelvin_to_celsius(value)
    } else {
        value
    }
}

pub(in crate::engine::builder) fn effective_instance_temperature_celsius(
    instance_params: &[(String, f64)],
    temperature_kelvin: f64,
) -> f64 {
    let mut current_temp_c = crate::analysis::temperature::kelvin_to_celsius(temperature_kelvin);
    if let Some((_, temp)) = instance_params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("TEMP"))
    {
        current_temp_c = temperature_param_to_celsius(*temp);
    } else if let Some((_, dtemp)) = instance_params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("DTEMP"))
    {
        current_temp_c += *dtemp;
    }
    current_temp_c
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
    instance_params: &[(String, f64)],
    temperature_kelvin: f64,
) -> Result<(), SimulationError> {
    let np = circuit.get_or_create_node(&element.nodes[0]);
    let nn = circuit.get_or_create_node(&element.nodes[1]);
    let current_temp_c =
        effective_instance_temperature_celsius(instance_params, temperature_kelvin);
    let tnom_c = netlist.options.tnom.unwrap_or(27.0);
    let tc1 = instance_params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("TC1"))
        .map(|(_, value)| *value)
        .unwrap_or(0.0);
    let tc2 = instance_params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("TC2"))
        .map(|(_, value)| *value)
        .unwrap_or(0.0);
    let temp_scale = temperature_scale_factor(current_temp_c, tnom_c, tc1, tc2);
    let mult = instance_params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("M") || name.eq_ignore_ascii_case("MULT"))
        .map(|(_, value)| *value)
        .unwrap_or(1.0);
    if !mult.is_finite() || mult <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Resistor '{}' has invalid multiplicity M={} (must be finite and > 0)",
            element.name, mult
        )));
    }
    if !temp_scale.is_finite() || temp_scale <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Resistor '{}' resolved to invalid temperature scaling factor {}",
            element.name, temp_scale
        )));
    }

    let params = base_eval_context(netlist);
    let prepared = prepare_behavioral_expression(expression, &params).map_err(|e| {
        SimulationError::Circuit(format!(
            "Resistor '{}' behavioral value expression could not be prepared: {}",
            element.name, e
        ))
    })?;
    let current_expression = if (mult - 1.0).abs() < f64::EPSILON {
        format!(
            "(V({},{})/(({})*{}))",
            element.nodes[0], element.nodes[1], prepared, temp_scale
        )
    } else {
        format!(
            "(({}*V({},{}))/(({})*{}))",
            mult, element.nodes[0], element.nodes[1], prepared, temp_scale
        )
    };

    let mut bcs = crate::device::BehavioralCurrentSource::new(
        element.name.clone(),
        np,
        nn,
        &current_expression,
    )
    .map_err(SimulationError::Circuit)?;
    bcs.set_expression_dialect(netlist.params.expression_dialect());
    circuit.behavioral_sources.add_current(bcs);
    Ok(())
}
