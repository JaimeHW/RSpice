//! Per-instance model copies for retained, explicitly calibrated aging changes.

use super::*;
use crate::netlist::{InstanceModelOverlay, ModelDef};

/// Resolve through the same geometry-bin selector used by the MOS builder.
/// Only parameters with an audited native mapping are accepted. In particular,
/// a user-authored spelling must never be silently ignored by a compact model.
pub(in crate::engine) fn reliability_model_parameters(
    netlist: &Netlist,
    element: &Element,
    requested_model: &str,
    parameters: impl IntoIterator<Item = String>,
    temperature: f64,
) -> Result<InstanceModelOverlay, SimulationError> {
    let model = selected_model(netlist, element, requested_model, temperature)?;
    let level = model_param(&model.params, &["LEVEL"]).unwrap_or(1.0);
    if !matches!(level, 1.0 | 2.0 | 3.0)
        || !["NMOS", "PMOS"]
            .iter()
            .any(|name| model.model_type.eq_ignore_ascii_case(name))
    {
        return Err(refusal(format!(
            "Aged re-simulation needs a parameter adapter for model '{}' ({} LEVEL={level})",
            model.name, model.model_type
        )));
    }
    let mut resolved = BTreeMap::new();
    for parameter in parameters {
        let key = parameter.to_ascii_uppercase();
        if !matches!(key.as_str(), "VTO" | "KP" | "GAMMA" | "PHI") {
            return Err(refusal(format!(
                "Aged re-simulation has no qualified parameter mapping for '{}:{parameter}'; classic MOS supports VTO, KP, GAMMA and PHI",
                model.name
            )));
        }
        let value = model_param(&model.params, &[&key]).ok_or_else(|| {
            refusal(format!(
                "Aging parameter '{}:{key}' needs an explicit, resolved numeric fresh model value",
                model.name
            ))
        })?;
        validate_value(&key, value)?;
        resolved.insert(key, value);
    }
    Ok(InstanceModelOverlay {
        requested_model: requested_model.into(),
        selected_model: model.name.clone(),
        parameters: resolved,
    })
}

fn selected_model<'a>(
    netlist: &'a Netlist,
    element: &Element,
    requested_model: &str,
    temperature: f64,
) -> Result<&'a ModelDef, SimulationError> {
    // Statistical draws are materialized under their original model identity.
    // A deterministic aging overlay must not replace or redraw those values.
    if !netlist.spectre_statistics.variations.is_empty() {
        return Err(refusal(
            "Aged model copies require a materialized deterministic statistical sample",
        ));
    }
    let ElementKind::Mosfet {
        model,
        instance_params,
        ..
    } = &element.kind
    else {
        return Err(refusal(format!(
            "Device '{}' has no aged compact-model parameter adapter",
            element.name
        )));
    };
    if !model.eq_ignore_ascii_case(requested_model) {
        return Err(refusal(format!(
            "Device '{}' uses compact model '{model}', not '{requested_model}'",
            element.name
        )));
    }
    find_binned_model_def(netlist, &element.name, model, instance_params, temperature)?.ok_or_else(
        || {
            refusal(format!(
                "Aged device '{}' needs an explicit compact-model card",
                element.name
            ))
        },
    )
}

fn validate_value(parameter: &str, value: f64) -> Result<(), SimulationError> {
    if !value.is_finite()
        || (matches!(parameter, "KP" | "GAMMA") && value < 0.0)
        || (parameter == "PHI" && value <= 0.0)
    {
        Err(refusal(format!(
            "Aged model parameter {parameter}={value} is outside its native domain"
        )))
    } else {
        Ok(())
    }
}

pub(super) fn apply_instance_model_overlays(
    netlist: &mut Netlist,
    elements: &mut [Element],
    temperature: f64,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    let overrides = netlist.ast_overlay.instance_models.clone();
    for (device, overlay) in overrides {
        check_build_abort(abort)?;
        let element = elements
            .iter_mut()
            .find(|element| element.name.eq_ignore_ascii_case(&device))
            .ok_or_else(|| {
                refusal(format!(
                    "Aged device '{device}' was not found after hierarchy expansion"
                ))
            })?;
        let fresh = reliability_model_parameters(
            netlist,
            element,
            &overlay.requested_model,
            overlay.parameters.keys().cloned(),
            temperature,
        )?;
        if !fresh
            .selected_model
            .eq_ignore_ascii_case(&overlay.selected_model)
        {
            return Err(refusal(format!(
                "Aged device '{device}' selected a different model bin"
            )));
        }
        let mut model =
            selected_model(netlist, element, &overlay.requested_model, temperature)?.clone();
        for (key, value) in &overlay.parameters {
            validate_value(key, *value)?;
            model
                .params
                .retain(|(name, _)| !name.eq_ignore_ascii_case(key));
            model
                .expr_params
                .retain(|(name, _)| !name.eq_ignore_ascii_case(key));
            model.params.push((key.clone(), *value));
        }
        let mut suffix = 0usize;
        loop {
            check_build_abort(abort)?;
            model.name = format!("__rspice_aged_{device}_{suffix}");
            if !netlist
                .models
                .iter()
                .any(|existing| existing.name.eq_ignore_ascii_case(&model.name))
            {
                break;
            }
            suffix += 1;
        }
        *element_model_name_mut(&mut element.kind).expect("validated MOS model") =
            model.name.clone();
        netlist.models.push(model);
    }
    Ok(())
}

fn refusal(message: impl Into<String>) -> SimulationError {
    SimulationError::Circuit(message.into())
}
