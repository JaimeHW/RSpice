use super::*;

/// Resolve the effective inductance of a linear inductor instance.
///
/// Resolution order:
/// 1. Explicit instance value (`L1 a b 1m`, or `L=`/`VALUE=` named form)
/// 2. Model-card `L`/`IND` value
///
/// followed by TC1/TC2 temperature scaling about the model's TNOM and
/// instance scaling: `SCALE` multiplies the value, `M`/`MULT` parallel
/// multiplicity divides it (m parallel inductors of value L behave as L/m).
pub(in crate::engine::builder) fn resolve_inductor_instance_value(
    netlist: &Netlist,
    element_name: &str,
    value: f64,
    model_name: Option<&str>,
    instance_params: &[(String, f64)],
    temperature_kelvin: f64,
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

    if let Some(model_def) = model_def
        && inductance.is_none()
    {
        inductance = resolve_model_param(
            model_def,
            &["L", "IND", "VALUE", "INDUCTANCE"],
            &eval_ctx,
        )?;

        if inductance.is_none() {
            return Err(SimulationError::Circuit(format!(
                "Inductor '{}' model '{}' requires an L/IND value (turns-based \
                 NT/CSECT/LENGTH inductance synthesis is not supported)",
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
        )
        .expect("inductor resolves")
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
}
