use super::*;

/// Resolve the effective inductance of a linear inductor instance.
///
/// Resolution order:
/// 1. Explicit instance value (`L1 a b 1m`, or `L=`/`VALUE=` named form)
/// 2. Model-card `L`/`IND` value for non-Xyce dialects
/// 3. Ngspice-compatible `NT` plus `LENGTH` and `DIA`/`CSECT` geometry
///
/// followed by the Xyce model-card `L` multiplier when enabled,
/// TC1/TC2 temperature scaling about the model's TNOM, and
/// instance scaling: `SCALE` multiplies the value, `M`/`MULT` parallel
/// multiplicity divides it (m parallel inductors of value L behave as L/m).
pub(in crate::engine::builder) fn resolve_inductor_instance_value(
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
                "Inductor '{}' references unknown model '{}'",
                element_name, model_name
            ))
        })?;
        ensure_model_type(
            "Inductor",
            element_name,
            model_name,
            model_def,
            &["L", "IND", "INDUCTOR"],
        )?;
        Some(model_def)
    } else {
        None
    };

    let (eval_ctx, current_temp_c, tnom_c) =
        resolve_passive_eval_context(netlist, model_def, instance_params, temperature_kelvin)?;

    let mut inductance = instance_param(instance_params, &["L", "IND", "VALUE"]);
    if inductance.is_none() && value.is_finite() && value > 0.0 {
        inductance = Some(value);
    }

    let mut xyce_model_multiplier = 1.0;
    if let Some(model_def) = model_def {
        if spice_dialect == SpiceDialect::Xyce {
            xyce_model_multiplier =
                resolve_model_param(model_def, &["L"], &eval_ctx)?.unwrap_or(1.0);
        } else if inductance.is_none() {
            inductance =
                resolve_model_param(model_def, &["L", "IND", "VALUE", "INDUCTANCE"], &eval_ctx)?;
            if inductance.is_none() {
                inductance = synthesize_ngspice_geometry_inductance(
                    element_name,
                    model_def,
                    instance_params,
                    &eval_ctx,
                )?;
            }
        }

        if inductance.is_none() {
            let requirement = if spice_dialect == SpiceDialect::Xyce {
                "requires an instance L/IND value; Xyce model L is only a multiplier"
            } else {
                "requires an L/IND value or a complete NT/LENGTH/DIA-or-CSECT geometry"
            };
            return Err(SimulationError::Circuit(format!(
                "Inductor '{}' model '{}' {requirement}",
                element_name,
                model_name.unwrap_or_default()
            )));
        }
    }

    let mut resolved = inductance.ok_or_else(|| {
        SimulationError::Circuit(format!(
            "Inductor '{}' has no valid inductance value",
            element_name
        ))
    })?;

    resolved *= xyce_model_multiplier;

    let tc1 = match instance_param(instance_params, &["TC1"]) {
        Some(value) => value,
        None => match model_def {
            Some(model_def) => resolve_model_param(model_def, &["TC1"], &eval_ctx)?.unwrap_or(0.0),
            None => 0.0,
        },
    };
    let tc2 = match instance_param(instance_params, &["TC2"]) {
        Some(value) => value,
        None => match model_def {
            Some(model_def) => resolve_model_param(model_def, &["TC2"], &eval_ctx)?.unwrap_or(0.0),
            None => 0.0,
        },
    };
    if tc1 != 0.0 || tc2 != 0.0 {
        let dt = current_temp_c - tnom_c;
        resolved *= 1.0 + tc1 * dt + tc2 * dt * dt;
    }

    if let Some(scale) = instance_param(instance_params, &["SCALE"]) {
        if !scale.is_finite() || scale <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Inductor '{}' has invalid SCALE={} (must be finite and > 0)",
                element_name, scale
            )));
        }
        resolved *= scale;
    }

    if let Some(mult) = instance_param(instance_params, &["M", "MULT"]) {
        if !mult.is_finite() || mult <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Inductor '{}' has invalid multiplicity M={} (must be finite and > 0)",
                element_name, mult
            )));
        }
        resolved /= mult;
    }

    if !resolved.is_finite() || resolved <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Inductor '{}' resolved to invalid inductance {}",
            element_name, resolved
        )));
    }

    Ok(resolved)
}

/// Ngspice 46 section 3.3.11 geometry synthesis for a linear inductor.
///
/// The model owns magnetic geometry. `NT` is the one geometry parameter that
/// may be overridden on the instance. `DIA` takes precedence over `CSECT`, and
/// the long-solenoid value is multiplied by the same Lundin/Nagaoka correction
/// used by ngspice for finite-length coils.
fn synthesize_ngspice_geometry_inductance(
    element_name: &str,
    model_def: &crate::netlist::ModelDef,
    instance_params: &[(String, f64)],
    eval_ctx: &crate::netlist::ParamContext,
) -> Result<Option<f64>, SimulationError> {
    let model_turns = resolve_model_param(model_def, &["NT"], eval_ctx)?;
    let length = resolve_model_param(model_def, &["LENGTH"], eval_ctx)?;
    let diameter = resolve_model_param(model_def, &["DIA"], eval_ctx)?;
    // DIA has semantic precedence, so an unused lower-precedence CSECT
    // expression must not make an otherwise valid model fail resolution.
    let cross_section = if diameter.is_none() {
        resolve_model_param(model_def, &["CSECT"], eval_ctx)?
    } else {
        None
    };
    let permeability = resolve_model_param(model_def, &["MU"], eval_ctx)?;
    let turns = instance_param(instance_params, &["NT"]).or(model_turns);

    let any_geometry = turns.is_some()
        || length.is_some()
        || diameter.is_some()
        || cross_section.is_some()
        || permeability.is_some();
    if !any_geometry {
        return Ok(None);
    }

    let require_positive = |name: &str, value: Option<f64>| {
        let value = value.ok_or_else(|| {
            SimulationError::Circuit(format!(
                "Inductor '{element_name}' geometry model '{}' is missing {name}",
                model_def.name
            ))
        })?;
        if !value.is_finite() || value <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Inductor '{element_name}' geometry parameter {name} must be finite and positive, got {value}"
            )));
        }
        Ok(value)
    };

    let turns = require_positive("NT", turns)?;
    let length = require_positive("LENGTH", length)?;
    let permeability = require_positive("MU", Some(permeability.unwrap_or(1.0)))?;
    let area = if let Some(diameter) = diameter {
        let diameter = require_positive("DIA", Some(diameter))?;
        std::f64::consts::PI * diameter * diameter / 4.0
    } else {
        require_positive("CSECT", cross_section)?
    };
    if !area.is_finite() || area <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Inductor '{element_name}' geometry produced invalid cross-sectional area {area}"
        )));
    }

    const VACUUM_PERMEABILITY: f64 = 4.0e-7 * std::f64::consts::PI;
    let correction = ngspice_lundin_correction(length, area);
    let inductance =
        permeability * VACUUM_PERMEABILITY * turns * turns * (area / length) * correction;
    if !inductance.is_finite() || inductance <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Inductor '{element_name}' geometry produced invalid inductance {inductance}"
        )));
    }
    Ok(Some(inductance))
}

/// Nagaoka coefficient using the Lundin approximation implemented by ngspice.
/// Ngspice deliberately disables the correction for
/// sub-micrometre length or sub-square-micrometre area.
fn ngspice_lundin_correction(length: f64, cross_section: f64) -> f64 {
    if cross_section < 1.0e-12 || length < 1.0e-6 {
        return 1.0;
    }
    let aspect = 2.0 * (cross_section / std::f64::consts::PI).sqrt() / length;
    let aspect_squared = aspect * aspect;
    let aspect_fourth = aspect_squared * aspect_squared;
    if aspect < 1.0 {
        let numerator = 1.0 + 0.383_901 * aspect_squared + 0.017_108 * aspect_fourth;
        let denominator = 1.0 + 0.258_952 * aspect_squared;
        numerator / denominator - 4.0 * aspect / (3.0 * std::f64::consts::PI)
    } else {
        let inverse_squared = 1.0 / aspect_squared;
        let inverse_fourth = 1.0 / aspect_fourth;
        let numerator = (2.0 * std::f64::consts::LN_2 + aspect.ln() - 0.5)
            * (1.0 + 0.383_901 * inverse_squared + 0.017_108 * inverse_fourth);
        let denominator = 1.0 + 0.258_952 * inverse_squared;
        let correction = 0.093_842 * inverse_squared + 0.002_029 * inverse_fourth
            - 0.000_801 * inverse_squared * inverse_fourth;
        2.0 * (numerator / denominator + correction) / (std::f64::consts::PI * aspect)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resolve_inductor_from_source(source: &str, name: &str) -> f64 {
        let netlist = crate::netlist::Netlist::parse(source).expect("test netlist parses");
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .expect("test inductor exists");

        let crate::netlist::ElementKind::Inductor {
            value,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            panic!("test element is not an inductor");
        };

        resolve_inductor_instance_value(
            &netlist,
            &element.name,
            *value,
            model.as_deref(),
            instance_params,
            crate::constants::TEMP_REFERENCE,
            SpiceDialect::Ngspice,
        )
        .expect("inductor resolves")
    }

    fn resolve_inductor_from_source_with_dialect(
        source: &str,
        name: &str,
        spice_dialect: SpiceDialect,
    ) -> Result<f64, SimulationError> {
        let netlist = crate::netlist::Netlist::parse(source).expect("test netlist parses");
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .expect("test inductor exists");

        let crate::netlist::ElementKind::Inductor {
            value,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            panic!("test element is not an inductor");
        };

        resolve_inductor_instance_value(
            &netlist,
            &element.name,
            *value,
            model.as_deref(),
            instance_params,
            crate::constants::TEMP_REFERENCE,
            spice_dialect,
        )
    }

    #[test]
    fn plain_value_passes_through() {
        let l = resolve_inductor_from_source("plain ind\nL1 a 0 10m\n.end\n", "L1");
        assert!((l - 10e-3).abs() < 1e-15, "resolved {l}");
    }

    #[test]
    fn multiplicity_divides_inductance() {
        let l = resolve_inductor_from_source("m ind\nL1 a 0 10m m=5\n.end\n", "L1");
        assert!((l - 2e-3).abs() < 1e-15, "resolved {l}");
    }

    #[test]
    fn model_card_value_and_tc_apply() {
        let l = resolve_inductor_from_source(
            "model ind\nL1 a 0 lmod temp=77\n.model lmod L L=1m TC1=2e-4\n.end\n",
            "L1",
        );
        let expected = 1e-3 * (1.0 + 2e-4 * 50.0);
        assert!(
            ((l - expected) / expected).abs() < 1e-12,
            "resolved {l}, expected {expected}"
        );
    }

    #[test]
    fn ngspice_cross_section_geometry_matches_the_lundin_oracle() {
        let inductance = resolve_inductor_from_source(
            "ngspice finite solenoid\n\
             L1 a 0 imod\n\
             .model imod L CSECT=126.7u LENGTH=1.69m NT=17 MU=1\n\
             .end\n",
            "L1",
        );

        // Independent evaluation of ngspice 46 equation 3.17 plus the
        // source-published Lundin coefficient for this geometry.
        const ORACLE_HENRIES: f64 = 6.714_481_731_512_433e-6;
        assert!(
            ((inductance - ORACLE_HENRIES) / ORACLE_HENRIES).abs() < 2.0e-15,
            "resolved {inductance:.17e}, oracle {ORACLE_HENRIES:.17e}"
        );
    }

    #[test]
    fn ngspice_diameter_geometry_honors_precedence_turns_scaling_and_temperature() {
        let inductance = resolve_inductor_from_source(
            "ngspice diameter solenoid\n\
             L1 a 0 imod NT=34 SCALE=2 M=4 TEMP=77\n\
             .model imod L DIA=12.7m CSECT=1p LENGTH=1.27m NT=17 MU=1 TC1=1m TNOM=27\n\
             .end\n",
            "L1",
        );

        // DIA wins over CSECT. Doubling turns multiplies L by four, SCALE/M
        // contributes one half, and the 50 C temperature delta contributes
        // 1.05, for a net factor of 2.1 over the ngspice diameter oracle.
        const ORACLE_HENRIES: f64 = 7.365_272_776_620_191_5e-6 * 2.1;
        assert!(
            ((inductance - ORACLE_HENRIES) / ORACLE_HENRIES).abs() < 2.0e-15,
            "resolved {inductance:.17e}, oracle {ORACLE_HENRIES:.17e}"
        );
    }

    #[test]
    fn ngspice_long_coil_uses_the_small_aspect_lundin_branch() {
        let inductance = resolve_inductor_from_source(
            "ngspice long solenoid\n\
             L1 a 0 imod\n\
             .model imod L DIA=1m LENGTH=10m NT=10\n\
             .end\n",
            "L1",
        );
        const ORACLE_HENRIES: f64 = 9.463_042_342_513_863e-9;
        assert!(
            ((inductance - ORACLE_HENRIES) / ORACLE_HENRIES).abs() < 2.0e-15,
            "resolved {inductance:.17e}, oracle {ORACLE_HENRIES:.17e}"
        );
    }

    #[test]
    fn incomplete_or_invalid_geometry_fails_closed() {
        for (label, model, expected) in [
            ("missing length", "NT=10 CSECT=1u", "missing LENGTH"),
            ("missing area", "NT=10 LENGTH=1m", "missing CSECT"),
            (
                "invalid turns",
                "NT=0 LENGTH=1m CSECT=1u",
                "NT must be finite and positive",
            ),
            (
                "invalid permeability",
                "NT=10 LENGTH=1m CSECT=1u MU=-1",
                "MU must be finite and positive",
            ),
        ] {
            let source = format!("{label}\nL1 a 0 imod\n.model imod L {model}\n.end\n");
            let error =
                resolve_inductor_from_source_with_dialect(&source, "L1", SpiceDialect::Ngspice)
                    .expect_err("malformed geometry must be rejected");
            assert!(
                error.to_string().contains(expected),
                "{label}: expected '{expected}' in {error}"
            );
        }
    }

    #[test]
    fn xyce_model_l_multiplies_explicit_inductance() {
        let l = resolve_inductor_from_source_with_dialect(
            "xyce ind\nL1 a 0 lmod 10m temp=90\n.model lmod L L=2 TC1=0.010 TC2=0.926e-4\n.end\n",
            "L1",
            SpiceDialect::Xyce,
        )
        .expect("inductor resolves");
        let expected = 10e-3 * 2.0 * (1.0 + 0.010 * 63.0 + 0.926e-4 * 63.0 * 63.0);
        assert!(
            ((l - expected) / expected).abs() < 1e-12,
            "resolved {l}, expected {expected}"
        );
    }

    #[test]
    fn xyce_model_l_without_instance_value_stays_invalid() {
        let err = resolve_inductor_from_source_with_dialect(
            "xyce missing ind\nL1 a 0 lmod\n.model lmod L L=2\n.end\n",
            "L1",
            SpiceDialect::Xyce,
        )
        .expect_err("Xyce inductor model L is a multiplier, not a replacement value");

        assert!(
            err.to_string().contains("requires an instance L/IND value"),
            "unexpected error: {err}"
        );
    }
}
