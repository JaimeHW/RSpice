//! Preserve rating inputs without changing model evaluation or random draws.
use super::*;
use crate::circuit::{DeviceModelSafety, ModelSafetyValue};

fn is_rating(name: &str) -> bool {
    matches!(
        name,
        "VGS_MAX"
            | "VGSR_MAX"
            | "VGD_MAX"
            | "VGDR_MAX"
            | "VGB_MAX"
            | "VGBR_MAX"
            | "VDS_MAX"
            | "VBS_MAX"
            | "VBSR_MAX"
            | "VBD_MAX"
            | "VBDR_MAX"
            | "VBE_MAX"
            | "VBC_MAX"
            | "VCE_MAX"
            | "FV_MAX"
            | "BV_MAX"
            | "ID_MAX"
            | "IDR_MAX"
            | "IC_MAX"
            | "IB_MAX"
            | "PD_MAX"
            | "TE_MAX"
    )
}

fn retained(name: &str) -> bool {
    is_rating(name)
        || matches!(
            name,
            "LEVEL" | "TNOM" | "RTH0" | "DERATING" | "PCHAN" | "PCHANNEL" | "PMOS"
        )
}

pub(super) fn record(
    circuit: &mut CircuitData,
    device: &str,
    requested_model: &str,
    selected: Option<&crate::netlist::ModelDef>,
    foundation: Option<&FoundationModelCard>,
) {
    let mut parameters = BTreeMap::new();
    let (model_name, model_type) = if let Some(model) = selected {
        for (name, value) in &model.params {
            let key = name.to_ascii_uppercase();
            if retained(&key) {
                parameters.insert(key, ModelSafetyValue::Numeric(*value));
            }
        }
        let mut unresolved = |name: &str, reason: String| {
            let key = name.to_ascii_uppercase();
            if retained(&key) {
                // A deferred or non-scalar field must never appear resolved
                // merely because a numeric field with the same name exists.
                parameters.insert(key, ModelSafetyValue::Unresolved(reason));
            }
        };
        for (name, expr) in &model.expr_params {
            unresolved(name, format!("unresolved expression: {expr}"));
        }
        for (name, value) in &model.string_params {
            unresolved(name, format!("string value: {value}"));
        }
        for name in model
            .string_vector_params
            .iter()
            .map(|(n, _)| n)
            .chain(model.real_vector_params.iter().map(|(n, _)| n))
            .chain(model.real_vector_expr_params.iter().map(|(n, _)| n))
            .chain(model.integer_vector_params.iter().map(|(n, _)| n))
        {
            unresolved(name, "non-scalar value".into());
        }
        (model.name.as_str(), model.model_type.as_str())
    } else if let Some(card) = foundation {
        for (name, value) in &card.params {
            if retained(name) {
                parameters.insert(name.clone(), ModelSafetyValue::Numeric(*value));
            }
        }
        (requested_model, card.model_type.as_str())
    } else {
        return;
    };
    if parameters.keys().any(|name| is_rating(name)) {
        circuit.device_model_safety.insert(
            device.to_ascii_uppercase(),
            DeviceModelSafety {
                model_name: model_name.into(),
                model_type: model_type.into(),
                generated: false,
                parameters,
            },
        );
    }
}
