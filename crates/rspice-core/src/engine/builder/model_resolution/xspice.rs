use super::*;

pub(in crate::engine::builder) struct ResolvedXspiceModel {
    pub(in crate::engine::builder) code_model: std::sync::Arc<dyn crate::xspice::CodeModel>,
    pub(in crate::engine::builder) numeric_params: Vec<(String, f64)>,
    pub(in crate::engine::builder) string_params: Vec<(String, String)>,
}

fn merge_numeric_params(base: &[(String, f64)], overrides: &[(String, f64)]) -> Vec<(String, f64)> {
    let mut merged = base.to_vec();

    for (name, value) in overrides {
        if let Some(existing) = merged
            .iter_mut()
            .find(|(existing_name, _)| existing_name.eq_ignore_ascii_case(name))
        {
            existing.1 = *value;
        } else {
            merged.push((name.clone(), *value));
        }
    }

    merged
}

pub(in crate::engine::builder) fn resolve_xspice_model_instance(
    netlist: &Netlist,
    registry: &crate::xspice::CodeModelRegistry,
    model_name: &str,
    instance_params: &[(String, f64)],
) -> Result<ResolvedXspiceModel, SimulationError> {
    if let Some(code_model) = registry.get(model_name) {
        return Ok(ResolvedXspiceModel {
            code_model,
            numeric_params: instance_params.to_vec(),
            string_params: Vec::new(),
        });
    }

    let model_def = find_model_def(netlist, model_name).ok_or_else(|| {
        SimulationError::Circuit(format!("Unknown XSPICE model '{}'", model_name))
    })?;

    let code_model = registry.get(&model_def.model_type).ok_or_else(|| {
        SimulationError::Circuit(format!(
            "Unknown XSPICE model '{}' (alias '{}' resolves to unregistered code model '{}')",
            model_name, model_def.name, model_def.model_type
        ))
    })?;

    Ok(ResolvedXspiceModel {
        code_model,
        numeric_params: merge_numeric_params(&model_def.params, instance_params),
        string_params: model_def.string_params.clone(),
    })
}
