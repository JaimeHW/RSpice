use super::*;

pub(crate) const XYCE_DEFAULT_CAPACITOR_AGE_DEGRADATION: f64 = 0.0233;

/// Resolve the effective capacitance of a capacitor instance.
///
/// Resolution order mirrors ngspice's CAP model (`capsetup.c`/`captemp.c`):
/// 1. Explicit instance value (`C1 a b 1u`, or `C=`/`VALUE=` named form)
/// 2. Model-card `C`/`CAP` value
/// 3. Geometric form from sheet capacitance:
///    `C = CJ*(W-NARROW)*(L-SHORT) + 2*CJSW*((W-NARROW)+(L-SHORT))`
///
/// followed, for Xyce, by instance AGE/D degradation; TC1/TC2 temperature
/// scaling about the model's TNOM; and
/// instance scaling: `SCALE` multiplies the value, `M`/`MULT` parallel
/// multiplicity multiplies the value (m parallel capacitors add).
pub(in crate::engine::builder) fn resolve_capacitor_instance_value(
    netlist: &Netlist,
    element_name: &str,
    value: f64,
    model_name: Option<&str>,
    instance_params: &[(String, f64)],
    temperature_kelvin: f64,
    spice_dialect: SpiceDialect,
) -> Result<f64, SimulationError> {
    let model_def = if let Some(model_name) = model_name {
        let model_def = find_model_def(netlist, model_name).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "Capacitor '{}' references unknown model '{}'",
                element_name, model_name
            ))
        })?;
        ensure_model_type(
            "Capacitor",
            element_name,
            model_name,
            model_def,
            &["C", "CAP", "CAPACITOR"],
        )?;
        Some(model_def)
    } else {
        None
    };

    let (eval_ctx, current_temp_c, tnom_c) =
        resolve_passive_eval_context(netlist, model_def, instance_params, temperature_kelvin)?;

    let mut capacitance = instance_param(instance_params, &["C", "CAP", "VALUE"]);
    let explicit_capacitance_given = capacitance.is_some() || (value.is_finite() && value >= 0.0);
    if capacitance.is_none() && value.is_finite() && value >= 0.0 {
        capacitance = Some(value);
    }

    let mut xyce_model_multiplier = 1.0;
    if let Some(model_def) = model_def {
        if spice_dialect == SpiceDialect::Xyce {
            xyce_model_multiplier =
                resolve_model_param(model_def, &["C"], &eval_ctx)?.unwrap_or(1.0);
            if capacitance.is_none() {
                let cj = resolve_model_param(model_def, &["CJ"], &eval_ctx)?.unwrap_or(0.0);
                let cjsw = resolve_model_param(model_def, &["CJSW"], &eval_ctx)?.unwrap_or(0.0);

                let length = instance_param(instance_params, &["L"]).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Capacitor '{}' model '{}' requires L when using Xyce semiconductor capacitor geometry",
                        element_name,
                        model_name.unwrap_or_default()
                    ))
                })?;
                let width = instance_param(instance_params, &["W"])
                    .or_else(|| {
                        resolve_model_param(model_def, &["DEFW"], &eval_ctx)
                            .ok()
                            .flatten()
                    })
                    .unwrap_or(1.0e-6);
                let narrow = resolve_model_param(model_def, &["NARROW"], &eval_ctx)?.unwrap_or(0.0);

                let w_eff = width - narrow;
                let l_eff = length - narrow;
                if !w_eff.is_finite() || !l_eff.is_finite() || w_eff <= 0.0 || l_eff <= 0.0 {
                    return Err(SimulationError::Circuit(format!(
                        "Capacitor '{}' has invalid Xyce effective geometry (W={}, L={}, NARROW={})",
                        element_name, width, length, narrow
                    )));
                }

                capacitance = Some(cj * l_eff * w_eff + 2.0 * cjsw * (l_eff + w_eff));
            }
        } else {
            if capacitance.is_none() {
                capacitance = resolve_model_param(
                    model_def,
                    &["C", "CAP", "VALUE", "CAPACITANCE"],
                    &eval_ctx,
                )?;
            }

            if capacitance.is_none() {
                let cj = resolve_model_param(model_def, &["CJ", "CJA"], &eval_ctx)?.ok_or_else(
                    || {
                        SimulationError::Circuit(format!(
                            "Capacitor '{}' model '{}' requires C/CAP or CJ with geometry",
                            element_name,
                            model_name.unwrap_or_default()
                        ))
                    },
                )?;
                let cjsw =
                    resolve_model_param(model_def, &["CJSW", "CJP"], &eval_ctx)?.unwrap_or(0.0);

                let width = instance_param(instance_params, &["W", "WIDTH"])
                    .or_else(|| {
                        resolve_model_param(model_def, &["W", "WIDTH", "DEFW"], &eval_ctx)
                            .ok()
                            .flatten()
                    })
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Capacitor '{}' model '{}' requires W/WIDTH (or DEFW) when using CJ",
                            element_name,
                            model_name.unwrap_or_default()
                        ))
                    })?;
                let length = instance_param(instance_params, &["L", "LENGTH"])
                    .or_else(|| {
                        resolve_model_param(model_def, &["L", "LENGTH", "DEFL"], &eval_ctx)
                            .ok()
                            .flatten()
                    })
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Capacitor '{}' model '{}' requires L/LENGTH (or DEFL) when using CJ",
                            element_name,
                            model_name.unwrap_or_default()
                        ))
                    })?;
                let narrow = resolve_model_param(model_def, &["NARROW"], &eval_ctx)?.unwrap_or(0.0);
                let short = resolve_model_param(model_def, &["SHORT"], &eval_ctx)?.unwrap_or(0.0);

                let w_eff = width - narrow;
                let l_eff = length - short;
                if !w_eff.is_finite() || !l_eff.is_finite() || w_eff <= 0.0 || l_eff <= 0.0 {
                    return Err(SimulationError::Circuit(format!(
                        "Capacitor '{}' has invalid effective geometry (W={}, L={}, NARROW={}, SHORT={})",
                        element_name, width, length, narrow, short
                    )));
                }

                capacitance = Some(cj * w_eff * l_eff + 2.0 * cjsw * (w_eff + l_eff));
            }
        }
    }

    let mut resolved = capacitance.ok_or_else(|| {
        if model_name.is_some() {
            SimulationError::Circuit(format!(
                "Capacitor '{}' model-based value could not be resolved",
                element_name
            ))
        } else {
            SimulationError::Circuit(format!(
                "Capacitor '{}' has no valid capacitance value",
                element_name
            ))
        }
    })?;

    if spice_dialect == SpiceDialect::Xyce {
        let age = instance_param(instance_params, &["AGE"]);
        let degradation = instance_param(instance_params, &["D"])
            .unwrap_or(XYCE_DEFAULT_CAPACITOR_AGE_DEGRADATION);
        if !degradation.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "Capacitor '{}' has non-finite Xyce aging coefficient D={}",
                element_name, degradation
            )));
        }
        if let Some(age) = age {
            if !explicit_capacitance_given {
                return Err(SimulationError::Circuit(format!(
                    "Capacitor '{}' specifies AGE but has no explicit C instance value",
                    element_name
                )));
            }
            if !age.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "Capacitor '{}' has non-finite Xyce AGE={}",
                    element_name, age
                )));
            }
            if age >= 1.0 {
                resolved *= 1.0 - degradation * age.log10();
            }
        }
    }

    resolved *= xyce_model_multiplier;

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
        let coeffs = crate::analysis::temperature::CapacitorTempCoeffs {
            tc1,
            tc2,
            ..Default::default()
        };
        resolved = coeffs.scale_capacitance(resolved, &temp_ctx);
    }

    if let Some(scale) = instance_param(instance_params, &["SCALE"]) {
        if !scale.is_finite() || scale <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Capacitor '{}' has invalid SCALE={} (must be finite and > 0)",
                element_name, scale
            )));
        }
        resolved *= scale;
    }

    if let Some(mult) = instance_param(instance_params, &["M", "MULT"]) {
        if !mult.is_finite() || mult <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Capacitor '{}' has invalid multiplicity M={} (must be finite and > 0)",
                element_name, mult
            )));
        }
        resolved *= mult;
    }

    if !resolved.is_finite() || resolved < 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Capacitor '{}' resolved to invalid capacitance {}",
            element_name, resolved
        )));
    }

    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resolve_capacitor_from_source(source: &str, name: &str) -> f64 {
        let netlist = crate::netlist::Netlist::parse(source).expect("test netlist parses");
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .expect("test capacitor exists");

        let crate::netlist::ElementKind::Capacitor {
            value,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            panic!("test element is not a capacitor");
        };

        resolve_capacitor_instance_value(
            &netlist,
            &element.name,
            *value,
            model.as_deref(),
            instance_params,
            crate::constants::TEMP_REFERENCE,
            SpiceDialect::Ngspice,
        )
        .expect("capacitor resolves")
    }

    #[test]
    fn plain_value_passes_through() {
        let c = resolve_capacitor_from_source("plain cap\nC1 a 0 2.2u\n.end\n", "C1");
        assert!((c - 2.2e-6).abs() < 1e-18, "resolved {c}");
    }

    #[test]
    fn multiplicity_multiplies_capacitance() {
        let c = resolve_capacitor_from_source("m cap\nC1 a 0 1u m=4\n.end\n", "C1");
        assert!((c - 4.0e-6).abs() < 1e-18, "resolved {c}");
    }

    #[test]
    fn model_geometry_matches_ngspice_cap_formula() {
        // C = CJ*(W-NARROW)*(L-SHORT) + 2*CJSW*((W-NARROW)+(L-SHORT))
        //   = 1e-3*(9u)*(19u) + 2*1e-9*(9u+19u) = 171e-12 + 56e-15
        let c = resolve_capacitor_from_source(
            "geom cap\nC1 a 0 cmod W=10u L=20u\n.model cmod C CJ=1e-3 CJSW=1e-9 NARROW=1u SHORT=1u\n.end\n",
            "C1",
        );
        let expected = 1e-3 * 9e-6 * 19e-6 + 2.0 * 1e-9 * (9e-6 + 19e-6);
        assert!(
            ((c - expected) / expected).abs() < 1e-12,
            "resolved {c}, expected {expected}"
        );
    }

    #[test]
    fn tc_scaling_applies_about_tnom() {
        // 27C nominal run temperature equals TNOM here, so pick an explicit
        // instance temperature to exercise the polynomial.
        let c = resolve_capacitor_from_source(
            "tc cap\nC1 a 0 cmod temp=127\n.model cmod C C=1u TC1=1e-3 TC2=1e-6\n.end\n",
            "C1",
        );
        let dt = 100.0;
        let expected = 1e-6 * (1.0 + 1e-3 * dt + 1e-6 * dt * dt);
        assert!(
            ((c - expected) / expected).abs() < 1e-12,
            "resolved {c}, expected {expected}"
        );
    }

    #[test]
    fn high_instance_temp_is_celsius_not_kelvin() {
        let c = resolve_capacitor_from_source(
            "hot cap\nC1 a 0 cmod 1u temp=727\n.model cmod C TC1=1.77m TC2=-0.63u\n.end\n",
            "C1",
        );
        let dt = 700.0;
        let expected = 1e-6 * (1.0 + 1.77e-3 * dt - 0.63e-6 * dt * dt);
        assert!(
            ((c - expected) / expected).abs() < 1e-12,
            "resolved {c}, expected {expected}"
        );
    }

    #[test]
    fn xyce_semiconductor_geometry_uses_model_c_as_multiplier() {
        let netlist = crate::netlist::Netlist::parse(
            "xyce geom cap\nC1 a 0 cmod L=20u W=1u\n.model cmod C CJ=1 C=2\n.end\n",
        )
        .expect("test netlist parses");
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("C1"))
            .expect("test capacitor exists");
        let crate::netlist::ElementKind::Capacitor {
            value,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            panic!("test element is not a capacitor");
        };

        let c = resolve_capacitor_instance_value(
            &netlist,
            &element.name,
            *value,
            model.as_deref(),
            instance_params,
            crate::constants::TEMP_REFERENCE,
            SpiceDialect::Xyce,
        )
        .expect("capacitor resolves");
        let expected = 1.0 * 20.0e-6 * 1.0e-6 * 2.0;
        assert!((c - expected).abs() < 1e-18, "resolved {c}");
    }

    #[test]
    fn xyce_age_degradation_precedes_model_temperature_and_multiplicity_scaling() {
        let netlist = crate::netlist::Netlist::parse(
            "aged cap\n\
             C1 a 0 cmod 4u AGE=87600 D=0.0233 TC1=0.01 TEMP=37 M=2\n\
             .model cmod C C=3 TNOM=27\n\
             .end\n",
        )
        .expect("aged capacitor parses");
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("C1"))
            .expect("aged capacitor exists");
        let crate::netlist::ElementKind::Capacitor {
            value,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            panic!("test element is not a capacitor");
        };

        let resolved = resolve_capacitor_instance_value(
            &netlist,
            &element.name,
            *value,
            model.as_deref(),
            instance_params,
            crate::constants::TEMP_REFERENCE,
            SpiceDialect::Xyce,
        )
        .expect("aged capacitor resolves");
        let age_factor = 1.0 - 0.0233 * 87600.0_f64.log10();
        let expected = 4e-6 * age_factor * 3.0 * (1.0 + 0.01 * 10.0) * 2.0;
        assert!(
            ((resolved - expected) / expected).abs() < 1e-12,
            "resolved {resolved}, expected {expected}"
        );

        let circuit = Engine::new(crate::engine::SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..crate::engine::SimulationConfig::default()
        })
        .build_circuit(&netlist)
        .expect("aged capacitor circuit builds");
        let capacitors = circuit.capacitor_storage();
        let index = capacitors
            .names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("C1"))
            .expect("aged capacitor reaches circuit storage");
        assert_eq!(
            capacitors.capacitances[index].to_bits(),
            expected.to_bits(),
            "the Xyce-resolved aged capacitance must be the value stamped at runtime"
        );
    }

    #[test]
    fn xyce_age_requires_an_explicit_capacitance() {
        let netlist = crate::netlist::Netlist::parse(
            "invalid aged geometry cap\n\
             C1 a 0 cmod L=2u W=3u AGE=100\n\
             .model cmod C CJ=1\n\
             .end\n",
        )
        .expect("aged geometry capacitor parses");
        let element = &netlist.elements[0];
        let crate::netlist::ElementKind::Capacitor {
            value,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            panic!("test element is not a capacitor");
        };

        let error = resolve_capacitor_instance_value(
            &netlist,
            &element.name,
            *value,
            model.as_deref(),
            instance_params,
            crate::constants::TEMP_REFERENCE,
            SpiceDialect::Xyce,
        )
        .expect_err("AGE without explicit C must fail");
        assert!(error.to_string().contains("no explicit C"), "{error}");
    }
}
