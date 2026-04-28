use super::*;

fn resolve_resistor_eval_context(
    netlist: &Netlist,
    model_def: Option<&crate::netlist::ModelDef>,
    instance_params: &[(String, f64)],
    temperature_kelvin: f64,
) -> Result<(crate::netlist::ParamContext, f64, f64), SimulationError> {
    let mut current_temp_c = crate::analysis::temperature::kelvin_to_celsius(temperature_kelvin);
    if let Some(temp) = instance_param(instance_params, &["TEMP"]) {
        current_temp_c = normalize_temperature_param_to_celsius(temp);
    } else if let Some(dtemp) = instance_param(instance_params, &["DTEMP"]) {
        current_temp_c += dtemp;
    }

    let base_tnom_c = netlist.options.tnom.unwrap_or(27.0);
    let Some(model_def) = model_def else {
        let mut ctx = netlist.params.clone();
        ctx.set("TEMP", current_temp_c);
        ctx.set("TEMPER", current_temp_c);
        ctx.set("TNOM", base_tnom_c);
        return Ok((ctx, current_temp_c, base_tnom_c));
    };

    let initial_ctx = build_model_eval_context(netlist, model_def, current_temp_c, base_tnom_c);
    let model_tnom_c = resolve_model_param(model_def, &["TNOM"], &initial_ctx)?
        .map(normalize_temperature_param_to_celsius)
        .unwrap_or(base_tnom_c);

    let ctx = if (model_tnom_c - base_tnom_c).abs() > f64::EPSILON {
        build_model_eval_context(netlist, model_def, current_temp_c, model_tnom_c)
    } else {
        initial_ctx
    };

    Ok((ctx, current_temp_c, model_tnom_c))
}

fn apply_resistor_instance_scaling(
    element_name: &str,
    quantity_label: &str,
    mut resistance: f64,
    instance_params: &[(String, f64)],
) -> Result<f64, SimulationError> {
    if let Some(scale) = instance_param(instance_params, &["SCALE"]) {
        if !scale.is_finite() || scale <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Resistor '{}' has invalid SCALE={} (must be finite and > 0)",
                element_name, scale
            )));
        }
        resistance *= scale;
    }

    if let Some(mult) = instance_param(instance_params, &["M", "MULT"]) {
        if !mult.is_finite() || mult <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Resistor '{}' has invalid multiplicity M={} (must be finite and > 0)",
                element_name, mult
            )));
        }
        resistance /= mult;
    }

    if !resistance.is_finite() || resistance <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Resistor '{}' resolved to invalid {} {}",
            element_name, quantity_label, resistance
        )));
    }

    Ok(resistance)
}

pub(in crate::engine::builder) fn resolve_resistor_instance_value(
    netlist: &Netlist,
    element_name: &str,
    value: f64,
    value_expr: Option<&str>,
    model_name: Option<&str>,
    instance_params: &[(String, f64)],
    temperature_kelvin: f64,
) -> Result<f64, SimulationError> {
    let model_def = if let Some(model_name) = model_name {
        let model_def = find_model_def(netlist, model_name).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "Resistor '{}' references unknown model '{}'",
                element_name, model_name
            ))
        })?;
        ensure_model_type(
            "Resistor",
            element_name,
            model_name,
            model_def,
            &["R", "RES", "RESISTOR"],
        )?;
        Some(model_def)
    } else {
        None
    };

    let (eval_ctx, current_temp_c, tnom_c) =
        resolve_resistor_eval_context(netlist, model_def, instance_params, temperature_kelvin)?;
    let mut resistance = instance_param(instance_params, &["R", "VALUE"]);
    if resistance.is_none() && value.is_finite() && value > 0.0 {
        resistance = Some(value);
    }
    if resistance.is_none()
        && let Some(expr) = value_expr
    {
        resistance = Some(
            crate::netlist::expr::eval_expression(expr, &eval_ctx).map_err(|e| {
                SimulationError::Circuit(format!(
                    "Resistor '{}' value expression could not be resolved: {}",
                    element_name, e
                ))
            })?,
        );
    }

    if let Some(model_def) = model_def {
        if resistance.is_none() {
            resistance = resolve_model_param(
                model_def,
                &["R", "RES", "R0", "VALUE", "RESISTANCE"],
                &eval_ctx,
            )?;
        }

        if resistance.is_none() {
            let rsh = resolve_model_param(model_def, &["RSH", "SHEETRES", "SHEETR"], &eval_ctx)?
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Resistor '{}' model '{}' requires R/RES or RSH with geometry",
                        element_name,
                        model_name.unwrap_or_default()
                    ))
                })?;

            if !rsh.is_finite() || rsh <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "Resistor '{}' model '{}' has invalid RSH={} (must be finite and > 0)",
                    element_name,
                    model_name.unwrap_or_default(),
                    rsh
                )));
            }

            let squares = if let Some(nsq) =
                instance_param(instance_params, &["NRS", "NRSQ", "NSQ", "SQUARES"])
            {
                nsq
            } else if let Some(nsq) =
                resolve_model_param(model_def, &["NRS", "NRSQ", "NSQ", "SQUARES"], &eval_ctx)?
            {
                nsq
            } else {
                let l = instance_param(instance_params, &["L", "LENGTH"])
                    .or_else(|| {
                        resolve_model_param(model_def, &["L", "LENGTH"], &eval_ctx)
                            .ok()
                            .flatten()
                    })
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Resistor '{}' model '{}' requires L/LENGTH when using RSH",
                            element_name,
                            model_name.unwrap_or_default()
                        ))
                    })?;
                let w = instance_param(instance_params, &["W", "WIDTH"])
                    .or_else(|| {
                        resolve_model_param(model_def, &["W", "WIDTH", "DEFW"], &eval_ctx)
                            .ok()
                            .flatten()
                    })
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Resistor '{}' model '{}' requires W/WIDTH (or DEFW) when using RSH",
                            element_name,
                            model_name.unwrap_or_default()
                        ))
                    })?;
                let narrow = resolve_model_param(model_def, &["NARROW"], &eval_ctx)?.unwrap_or(0.0);
                let short = resolve_model_param(model_def, &["SHORT"], &eval_ctx)?.unwrap_or(0.0);
                let l_eff = l - short;
                let w_eff = w - narrow;
                if !l_eff.is_finite() || !w_eff.is_finite() || l_eff <= 0.0 || w_eff <= 0.0 {
                    return Err(SimulationError::Circuit(format!(
                        "Resistor '{}' has invalid effective geometry (L={}, W={}, SHORT={}, NARROW={})",
                        element_name, l, w, short, narrow
                    )));
                }
                l_eff / w_eff
            };

            if !squares.is_finite() || squares <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "Resistor '{}' has invalid number of squares {}",
                    element_name, squares
                )));
            }
            resistance = Some(rsh * squares);
        }
    }

    let mut resolved = resistance.ok_or_else(|| {
        if model_name.is_some() {
            SimulationError::Circuit(format!(
                "Resistor '{}' model-based value could not be resolved",
                element_name
            ))
        } else {
            SimulationError::Circuit(format!(
                "Resistor '{}' has no valid resistance value",
                element_name
            ))
        }
    })?;

    let tc1 = instance_param(instance_params, &["TC1"])
        .or_else(|| {
            model_def.and_then(|model_def| {
                resolve_model_param(model_def, &["TC1"], &eval_ctx)
                    .ok()
                    .flatten()
            })
        })
        .unwrap_or(0.0);
    let tc2 = instance_param(instance_params, &["TC2"])
        .or_else(|| {
            model_def.and_then(|model_def| {
                resolve_model_param(model_def, &["TC2"], &eval_ctx)
                    .ok()
                    .flatten()
            })
        })
        .unwrap_or(0.0);
    if tc1 != 0.0 || tc2 != 0.0 {
        let temp_ctx = crate::analysis::TemperatureContext::from_celsius(current_temp_c, tnom_c);
        resolved = crate::analysis::ResistorTempCoeffs::new(tc1, tc2)
            .scale_resistance(resolved, &temp_ctx);
    }

    apply_resistor_instance_scaling(element_name, "resistance", resolved, instance_params)
}

pub(in crate::engine::builder) fn resolve_resistor_small_signal_value(
    element_name: &str,
    dc_resistance: f64,
    instance_params: &[(String, f64)],
) -> Result<f64, SimulationError> {
    match instance_param(instance_params, &["AC"]) {
        Some(ac_resistance) => apply_resistor_instance_scaling(
            element_name,
            "small-signal resistance",
            ac_resistance,
            instance_params,
        ),
        None => {
            if !dc_resistance.is_finite() || dc_resistance <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "Resistor '{}' resolved to invalid small-signal resistance {}",
                    element_name, dc_resistance
                )));
            }
            Ok(dc_resistance)
        }
    }
}
