use super::*;

/// Resolve the resnoise.c flicker-noise terms for a resistor instance:
/// `(coefficient, AF, EF)` where the density is
/// `coefficient·|I|^AF / f^EF`, with the model KF and the effective noise
/// area `(L − 2·SHORT)^LF · (W − 2·NARROW)^WF` (1.0 when no geometry is
/// given, per ressetup.c) folded into the coefficient. Returns `None`
/// when the model carries no KF.
pub(in crate::engine::builder) fn resolve_resistor_flicker_noise(
    netlist: &Netlist,
    model_name: Option<&str>,
    instance_params: &[(String, f64)],
    temperature_kelvin: f64,
) -> Result<Option<(f64, f64, f64)>, SimulationError> {
    let Some(model_name) = model_name else {
        return Ok(None);
    };
    let Some(model_def) = find_model_def(netlist, model_name) else {
        return Ok(None);
    };
    let (eval_ctx, _, _) = resolve_resistor_eval_context(
        netlist,
        Some(model_def),
        instance_params,
        temperature_kelvin,
    )?;
    let Some(kf) =
        resolve_model_param(model_def, &["KF"], &eval_ctx)?.filter(|v| v.is_finite() && *v > 0.0)
    else {
        return Ok(None);
    };
    let af = resolve_model_param(model_def, &["AF"], &eval_ctx)?
        .filter(|v| v.is_finite() && *v > 0.0)
        .unwrap_or(1.0);
    let ef = resolve_model_param(model_def, &["EF"], &eval_ctx)?
        .filter(|v| v.is_finite() && *v > 0.0)
        .unwrap_or(1.0);
    let lf = resolve_model_param(model_def, &["LF"], &eval_ctx)?
        .filter(|v| v.is_finite())
        .unwrap_or(1.0);
    let wf = resolve_model_param(model_def, &["WF"], &eval_ctx)?
        .filter(|v| v.is_finite())
        .unwrap_or(1.0);
    let short = resolve_model_param(model_def, &["SHORT"], &eval_ctx)?.unwrap_or(0.0);
    let narrow = resolve_model_param(model_def, &["NARROW"], &eval_ctx)?.unwrap_or(0.0);

    let length = instance_param(instance_params, &["L", "LENGTH"]).or_else(|| {
        resolve_model_param(model_def, &["L", "LENGTH"], &eval_ctx)
            .ok()
            .flatten()
    });
    let width = instance_param(instance_params, &["W", "WIDTH"]).or_else(|| {
        resolve_model_param(model_def, &["W", "WIDTH", "DEFW"], &eval_ctx)
            .ok()
            .flatten()
    });
    let eff_noise_area = if length.is_some() || width.is_some() {
        let l_eff = (length.unwrap_or(0.0) - 2.0 * short).max(0.0);
        let w_eff = (width.unwrap_or(0.0) - 2.0 * narrow).max(0.0);
        let area = l_eff.powf(lf) * w_eff.powf(wf);
        if area.is_finite() && area > 0.0 {
            area
        } else {
            return Ok(None);
        }
    } else {
        1.0
    };

    Ok(Some((kf / eff_noise_area, af, ef)))
}

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
        let mut ctx = base_eval_context(netlist);
        set_temperature_scalars(&mut ctx, current_temp_c, base_tnom_c);
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

fn resolve_resistor_model_level(
    element_name: &str,
    model_name: &str,
    model_def: &crate::netlist::ModelDef,
    eval_ctx: &crate::netlist::ParamContext,
) -> Result<i32, SimulationError> {
    for (name, value) in &model_def.string_params {
        if name.eq_ignore_ascii_case("LEVEL") {
            return Err(SimulationError::Circuit(format!(
                "Resistor '{}' model '{}' has non-numeric LEVEL=\"{}\"; LEVEL selectors must be finite integers",
                element_name, model_name, value
            )));
        }
    }

    let Some(level) = resolve_model_param(model_def, &["LEVEL"], eval_ctx)? else {
        return Ok(1);
    };
    let rounded = level.round();
    if !level.is_finite() || (level - rounded).abs() > 1e-9 {
        return Err(SimulationError::Circuit(format!(
            "Resistor '{}' model '{}' has unsupported resistor LEVEL={} (LEVEL selectors must be finite integers)",
            element_name, model_name, level
        )));
    }

    match rounded as i32 {
        0 | 1 | 2 => Ok(rounded as i32),
        level => Err(SimulationError::Circuit(format!(
            "Resistor '{}' model '{}' requests unsupported resistor LEVEL={}",
            element_name, model_name, level
        ))),
    }
}

fn model_has_param(model_def: &crate::netlist::ModelDef, names: &[&str]) -> bool {
    names.iter().any(|candidate| {
        model_def
            .params
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case(candidate))
            || model_def
                .expr_params
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case(candidate))
            || model_def
                .string_params
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case(candidate))
    })
}

fn level2_requests_self_consistent_thermal_resistor(
    model_def: &crate::netlist::ModelDef,
    instance_params: &[(String, f64)],
) -> bool {
    let has_l = instance_param(instance_params, &["L", "LENGTH"]).is_some();
    let has_a = instance_param(instance_params, &["A", "AREA"]).is_some();
    let has_instance_material = instance_param(instance_params, &["RESISTIVITY"]).is_some()
        && instance_param(instance_params, &["HEATCAPACITY"]).is_some();
    let has_model_material = model_has_param(model_def, &["RESISTIVITY"])
        && model_has_param(model_def, &["HEATCAPACITY"]);

    has_l && has_a && (has_instance_material || has_model_material)
}

fn resistor_uses_xyce_default_value(instance_params: &[(String, f64)]) -> bool {
    instance_param(
        instance_params,
        &[crate::netlist::XYCE_DEFAULT_RESISTOR_VALUE_MARKER],
    )
    .is_some()
}

fn resolve_level2_resistor_electrical_subset(
    element_name: &str,
    model_name: &str,
    value: f64,
    value_expr: Option<&str>,
    model_def: &crate::netlist::ModelDef,
    instance_params: &[(String, f64)],
    eval_ctx: &crate::netlist::ParamContext,
) -> Result<f64, SimulationError> {
    let uses_xyce_default = resistor_uses_xyce_default_value(instance_params);
    let mut resistance = instance_param(instance_params, &["R", "VALUE"]);
    if resistance.is_none() && !uses_xyce_default && value.is_finite() && value > 0.0 {
        resistance = Some(value);
    }
    if resistance.is_none()
        && let Some(expr) = value_expr
    {
        resistance = Some(
            crate::netlist::expr::eval_expression(expr, eval_ctx).map_err(|e| {
                SimulationError::Circuit(format!(
                    "Resistor '{}' value expression could not be resolved: {}",
                    element_name, e
                ))
            })?,
        );
    }

    if resistance.is_none() {
        let Some(rsh) = resolve_model_param(model_def, &["RSH", "SHEETRES", "SHEETR"], eval_ctx)?
        else {
            if uses_xyce_default {
                return Ok(1000.0);
            }
            return Err(SimulationError::Circuit(format!(
                "Resistor '{}' model '{}' requires instance R or RSH with L/W geometry for native Xyce LEVEL=2",
                element_name, model_name
            )));
        };
        if !rsh.is_finite() || rsh <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Resistor '{}' model '{}' has invalid RSH={} (must be finite and > 0)",
                element_name, model_name, rsh
            )));
        }

        let l = instance_param(instance_params, &["L", "LENGTH"]).or_else(|| {
            resolve_model_param(model_def, &["L", "LENGTH"], eval_ctx)
                .ok()
                .flatten()
        });
        if uses_xyce_default && l.is_none() {
            return Ok(1000.0);
        }
        let l = l.ok_or_else(|| {
            SimulationError::Circuit(format!(
                "Resistor '{}' model '{}' requires L/LENGTH when using RSH with native Xyce LEVEL=2",
                element_name, model_name
            ))
        })?;
        let w = instance_param(instance_params, &["W", "WIDTH"])
            .or_else(|| {
                resolve_model_param(model_def, &["W", "WIDTH", "DEFW"], eval_ctx)
                    .ok()
                    .flatten()
            })
            .unwrap_or(10.0e-6);
        let narrow = resolve_model_param(model_def, &["NARROW"], eval_ctx)?.unwrap_or(0.0);
        let l_eff = l - narrow;
        let w_eff = w - narrow;
        if !l_eff.is_finite() || !w_eff.is_finite() || l_eff <= 0.0 || w_eff < 0.0 {
            if uses_xyce_default {
                return Ok(1000.0);
            }
            return Err(SimulationError::Circuit(format!(
                "Resistor '{}' has invalid Xyce LEVEL=2 effective geometry (L={}, W={}, NARROW={})",
                element_name, l, w, narrow
            )));
        }
        resistance = Some(if w_eff == 0.0 {
            f64::INFINITY
        } else {
            rsh * l_eff / w_eff
        });
    }

    let multiplier = resolve_model_param(model_def, &["R"], eval_ctx)?.unwrap_or(1.0);
    if !multiplier.is_finite() || multiplier <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Resistor '{}' model '{}' has invalid LEVEL=2 model R multiplier {} (must be finite and > 0)",
            element_name, model_name, multiplier
        )));
    }

    Ok(resistance.ok_or_else(|| {
        SimulationError::Circuit(format!(
            "Resistor '{}' model '{}' LEVEL=2 value could not be resolved",
            element_name, model_name
        ))
    })? * multiplier)
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

    if resistance.is_infinite() && resistance.is_sign_positive() {
        return Ok(resistance);
    }

    if !resistance.is_finite() {
        return Err(SimulationError::Circuit(format!(
            "Resistor '{}' resolved to invalid {} {}",
            element_name, quantity_label, resistance
        )));
    }

    Ok(resistance)
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Level1ModelDefaultResistance {
    resistance: f64,
    from_synthetic_default: bool,
}

fn resolve_level1_model_default_resistance(
    element_name: &str,
    model_def: &crate::netlist::ModelDef,
    instance_params: &[(String, f64)],
    eval_ctx: &crate::netlist::ParamContext,
) -> Result<Level1ModelDefaultResistance, SimulationError> {
    let rsh = resolve_model_param(model_def, &["RSH", "SHEETRES", "SHEETR"], eval_ctx)?;
    let Some(rsh) = rsh.filter(|value| value.is_finite() && *value != 0.0) else {
        return Ok(Level1ModelDefaultResistance {
            resistance: 1000.0,
            from_synthetic_default: true,
        });
    };

    let (squares, from_synthetic_default) = if let Some(nsq) =
        instance_param(instance_params, &["NRS", "NRSQ", "NSQ", "SQUARES"])
    {
        (nsq, false)
    } else if let Some(nsq) =
        resolve_model_param(model_def, &["NRS", "NRSQ", "NSQ", "SQUARES"], eval_ctx)?
    {
        (nsq, false)
    } else if let Some(l) = instance_param(instance_params, &["L", "LENGTH"])
        .or_else(|| {
            resolve_model_param(model_def, &["L", "LENGTH"], eval_ctx)
                .ok()
                .flatten()
        })
        .filter(|value| value.is_finite() && *value != 0.0)
    {
        let w = instance_param(instance_params, &["W", "WIDTH"])
            .or_else(|| {
                resolve_model_param(model_def, &["W", "WIDTH", "DEFW"], eval_ctx)
                    .ok()
                    .flatten()
            })
            .unwrap_or(10.0e-6);
        let narrow = resolve_model_param(model_def, &["NARROW"], eval_ctx)?.unwrap_or(0.0);
        let short = resolve_model_param(model_def, &["SHORT"], eval_ctx)?.unwrap_or(0.0);
        let l_eff = l - 2.0 * short;
        let w_eff = w - 2.0 * narrow;
        if !l_eff.is_finite() || !w_eff.is_finite() || l_eff <= 0.0 || w_eff < 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Resistor '{}' has invalid effective geometry (L={}, W={}, SHORT={}, NARROW={})",
                element_name, l, w, short, narrow
            )));
        }
        if w_eff == 0.0 {
            (f64::INFINITY, false)
        } else {
            (l_eff / w_eff, false)
        }
    } else {
        return Ok(Level1ModelDefaultResistance {
            resistance: 1000.0,
            from_synthetic_default: true,
        });
    };

    if !(squares.is_infinite() && squares.is_sign_positive())
        && (!squares.is_finite() || squares <= 0.0)
    {
        return Err(SimulationError::Circuit(format!(
            "Resistor '{}' has invalid number of squares {}",
            element_name, squares
        )));
    }

    Ok(Level1ModelDefaultResistance {
        resistance: rsh * squares,
        from_synthetic_default,
    })
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
    let uses_xyce_default = resistor_uses_xyce_default_value(instance_params);
    let resistor_level = if let (Some(model_def), Some(model_name)) = (model_def, model_name) {
        let level = resolve_resistor_model_level(element_name, model_name, model_def, &eval_ctx)?;
        if level == 2
            && level2_requests_self_consistent_thermal_resistor(model_def, instance_params)
        {
            return Err(SimulationError::Circuit(format!(
                "Resistor '{}' model '{}' requests Xyce LEVEL=2 self-consistent thermal resistor state, which has no native implementation yet",
                element_name, model_name
            )));
        }
        Some(level)
    } else {
        None
    };

    let mut resistance = if let (Some(2), Some(model_def), Some(model_name)) =
        (resistor_level, model_def, model_name)
    {
        Some(resolve_level2_resistor_electrical_subset(
            element_name,
            model_name,
            value,
            value_expr,
            model_def,
            instance_params,
            &eval_ctx,
        )?)
    } else {
        let mut resistance = instance_param(instance_params, &["R", "VALUE"]);
        if resistance.is_none() && uses_xyce_default && model_def.is_none() {
            resistance = Some(1000.0);
        }
        if resistance.is_none() && !uses_xyce_default && value.is_finite() {
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
        resistance
    };

    if let Some(model_def) = model_def {
        let resistance_multiplier = resolve_model_param(
            model_def,
            &["R", "RES", "R0", "VALUE", "RESISTANCE"],
            &eval_ctx,
        )?;

        let mut uses_synthetic_model_default = false;
        if resistance.is_none() {
            let model_default = resolve_level1_model_default_resistance(
                element_name,
                model_def,
                instance_params,
                &eval_ctx,
            )?;
            uses_synthetic_model_default = model_default.from_synthetic_default;
            resistance = Some(model_default.resistance);
        }

        if resistor_level != Some(2) {
            if uses_synthetic_model_default {
                if let Some(model_resistance) = resistance_multiplier {
                    resistance = Some(model_resistance);
                }
            } else {
                let resistance_multiplier = resistance_multiplier.unwrap_or(1.0);
                resistance = resistance.map(|value| value * resistance_multiplier);
            }
        }
    } else if resistance.is_none() && uses_xyce_default {
        resistance = Some(1000.0);
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

    let tce = instance_param(instance_params, &["TCE"]).or_else(|| {
        model_def.and_then(|model_def| {
            resolve_model_param(model_def, &["TCE"], &eval_ctx)
                .ok()
                .flatten()
        })
    });
    if let Some(tce) = tce {
        let temp_delta_c = current_temp_c - tnom_c;
        let scale = 1.01_f64.powf(tce * temp_delta_c);
        if !scale.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "Resistor '{}' TCE temperature scaling produced non-finite resistance multiplier {}",
                element_name, scale
            )));
        }
        resolved *= scale;
    } else {
        let tc1 = instance_param(instance_params, &["TC1", "TC"])
            .or_else(|| {
                model_def.and_then(|model_def| {
                    resolve_model_param(model_def, &["TC1", "TC"], &eval_ctx)
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
            let temp_ctx =
                crate::analysis::TemperatureContext::from_celsius(current_temp_c, tnom_c);
            resolved = crate::analysis::ResistorTempCoeffs::new(tc1, tc2)
                .scale_resistance(resolved, &temp_ctx);
        }
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
            if dc_resistance.is_infinite() && dc_resistance.is_sign_positive() {
                return Ok(dc_resistance);
            }
            if !dc_resistance.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "Resistor '{}' resolved to invalid small-signal resistance {}",
                    element_name, dc_resistance
                )));
            }
            Ok(dc_resistance)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resolve_resistor_from_source(source: &str, name: &str) -> (f64, Option<f64>) {
        resolve_resistor_from_source_at(source, name, crate::constants::TEMP_REFERENCE)
    }

    fn resolve_resistor_from_source_at(
        source: &str,
        name: &str,
        temperature_kelvin: f64,
    ) -> (f64, Option<f64>) {
        let netlist = crate::netlist::Netlist::parse(source).expect("test netlist parses");
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .expect("test resistor exists");

        let crate::netlist::ElementKind::Resistor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            panic!("test element is not a resistor");
        };

        let dc = resolve_resistor_instance_value(
            &netlist,
            &element.name,
            *value,
            value_expr.as_deref(),
            model.as_deref(),
            instance_params,
            temperature_kelvin,
        )
        .expect("resistor resolves");
        let ac = instance_param(instance_params, &["AC"])
            .map(|_| resolve_resistor_small_signal_value(&element.name, dc, instance_params))
            .transpose()
            .expect("resistor AC value resolves");

        (dc, ac)
    }

    #[test]
    fn resistor_tce_uses_xyce_percent_compounding_and_precedence() {
        let temperature_kelvin = crate::analysis::temperature::celsius_to_kelvin(37.0);
        let expected_instance = 1.01_f64.powf(3.0 * 10.0);
        let expected_model = 1.01_f64.powf(4.0 * 10.0);
        let expected_override = 1.01_f64.powf(-6.0 * 10.0);

        let (instance_tce, _) = resolve_resistor_from_source_at(
            r#"instance TCE overrides polynomial TC
R1 a 0 1 TC1=1 TC2=2 TCE=3
.end
"#,
            "R1",
            temperature_kelvin,
        );
        assert!(
            (instance_tce - expected_instance).abs() < 1e-12,
            "instance TCE resistance {instance_tce}, expected {expected_instance}"
        );

        let (model_tce, _) = resolve_resistor_from_source_at(
            r#"model TCE overrides instance polynomial TC
R2 a 0 1 RMOD TC1=1 TC2=2
.model RMOD R (TCE=4)
.end
"#,
            "R2",
            temperature_kelvin,
        );
        assert!(
            (model_tce - expected_model).abs() < 1e-12,
            "model TCE resistance {model_tce}, expected {expected_model}"
        );

        let (instance_overrides_model, _) = resolve_resistor_from_source_at(
            r#"instance TCE overrides model TCE
R3 a 0 1 RMOD TCE=-6
.model RMOD R (TCE=4)
.end
"#,
            "R3",
            temperature_kelvin,
        );
        assert!(
            (instance_overrides_model - expected_override).abs() < 1e-12,
            "instance override TCE resistance {instance_overrides_model}, expected {expected_override}"
        );
    }

    #[test]
    fn rsh_geometry_subtracts_short_and_narrow_from_both_edges() {
        let (dc, ac) = resolve_resistor_from_source(
            r#"finite geometry
R1 n 0 rmod L=12u W=4u
.model rmod R RSH=1000 NARROW=1u SHORT=1u
.end
"#,
            "R1",
        );

        assert!((dc - 5_000.0).abs() < 1e-9, "resolved DC resistance {dc}");
        assert_eq!(ac, None);
    }

    #[test]
    fn rsh_geometry_with_zero_effective_width_is_open_in_dc() {
        let source = r#"res_array geometry
R3 4 0 rmodel1 L=11u W=2u ac=2.5k
.model rmodel1 R RSH=1000 NARROW=1u
.end
"#;

        let (dc, ac) = resolve_resistor_from_source(source, "R3");

        assert!(dc.is_infinite() && dc.is_sign_positive());
        assert_eq!(ac, Some(2_500.0));

        let netlist = crate::netlist::Netlist::parse(source).expect("test netlist parses");
        let circuit = crate::engine::Engine::default()
            .build_circuit(&netlist)
            .expect("circuit builds");
        let resistor_idx = circuit
            .resistors
            .names
            .iter()
            .position(|resistor_name| resistor_name.eq_ignore_ascii_case("R3"))
            .expect("R3 stored");

        assert_eq!(circuit.resistors.conductances[resistor_idx], 0.0);
        assert_eq!(
            circuit.resistors.small_signal_conductance(resistor_idx),
            1.0 / 2_500.0
        );
    }

    #[test]
    fn direct_negative_resistance_is_valid_spice() {
        let (dc, ac) = resolve_resistor_from_source(
            r#"negative resistor
R1 a 0 -100
.end
"#,
            "R1",
        );

        assert_eq!(dc, -100.0);
        assert_eq!(ac, None);
    }

    #[test]
    fn direct_zero_resistance_resolves_for_branch_form_stamping() {
        let netlist = crate::netlist::Netlist::parse(
            r#"zero resistor
R1 a 0 0
.end
"#,
        )
        .expect("test netlist parses");
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("R1"))
            .expect("test resistor exists");
        let crate::netlist::ElementKind::Resistor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            panic!("test element is not a resistor");
        };

        let resolved = resolve_resistor_instance_value(
            &netlist,
            &element.name,
            *value,
            value_expr.as_deref(),
            model.as_deref(),
            instance_params,
            crate::constants::TEMP_REFERENCE,
        )
        .expect("zero-ohm resistor resolves before branch-form stamping");

        assert_eq!(resolved, 0.0);
    }

    #[test]
    fn xyce_value_less_resistors_default_to_one_kilohm() {
        let (plain, _) = resolve_resistor_from_source(
            r#"value-less resistor
R1 a 0
.end
"#,
            "R1",
        );
        assert_eq!(plain, 1000.0);

        let (level1, _) = resolve_resistor_from_source(
            r#"value-less level1 resistor
R2 a 0 rmod
.model rmod R RSH=1 LEVEL=1
.end
"#,
            "R2",
        );
        assert_eq!(level1, 1000.0);

        let (level2, _) = resolve_resistor_from_source(
            r#"value-less level2 resistor
R3 a 0 rmod
.model rmod R RSH=2 LEVEL=2
.end
"#,
            "R3",
        );
        assert_eq!(level2, 1000.0);
    }

    #[test]
    fn value_less_level1_modeled_resistor_uses_model_r_as_default() {
        let (model_r, _) = resolve_resistor_from_source(
            r#"value-less modeled resistor
R1 a 0 rmod
.model rmod R R=2k
.end
"#,
            "R1",
        );
        assert_eq!(model_r, 2000.0);

        let (model_res_alias, _) = resolve_resistor_from_source(
            r#"value-less modeled resistor with RES alias
R2 a 0 rmod
.model rmod R RES=43
.end
"#,
            "R2",
        );
        assert_eq!(model_res_alias, 43.0);
    }

    #[test]
    fn level1_geometry_keeps_model_r_multiplier_path() {
        let (geometry, _) = resolve_resistor_from_source(
            r#"modeled resistor geometry with multiplier
R1 a 0 rmod L=2 W=1
.model rmod R RSH=100 R=2
.end
"#,
            "R1",
        );
        assert_eq!(geometry, 400.0);
    }
}
