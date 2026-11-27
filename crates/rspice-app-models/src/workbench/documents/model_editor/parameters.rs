//! The parameter helpers the editor's definition, override sync and delta
//! share.

use super::*;

pub(super) fn parameter_value_text(value: &ParameterValue) -> String {
    match value {
        ParameterValue::Numeric(value) => value.to_string(),
        ParameterValue::String(value) => value.clone(),
    }
}

pub(super) fn synchronize_parameter_overrides(
    metadata: &mut ModelDefinitionMetadata,
    parameters: &[ParameterDefinition],
) {
    for parameter in parameters {
        let ParameterSource::Overridden { section } = &parameter.source else {
            continue;
        };
        let Some(section) = metadata
            .sections
            .iter_mut()
            .find(|candidate| candidate.name.eq_ignore_ascii_case(section))
        else {
            continue;
        };
        let Some(key) = section
            .overrides
            .keys()
            .find(|name| name.eq_ignore_ascii_case(&parameter.name))
            .cloned()
        else {
            continue;
        };
        section.overrides.insert(key, parameter.value.clone());
    }
}

pub(super) fn parse_optional_finite_bound(
    value: &str,
    field: String,
    diagnostics: &mut Vec<ModelFieldDiagnostic>,
) -> Option<FiniteF64> {
    if value.is_empty() {
        return None;
    }
    match value.parse::<f64>() {
        Ok(value) => match FiniteF64::new(value) {
            Ok(value) => Some(value),
            Err(_) => {
                diagnostics.push(ModelFieldDiagnostic {
                    field,
                    message: "Bound must be finite".to_owned(),
                });
                None
            }
        },
        Err(error) => {
            diagnostics.push(ModelFieldDiagnostic {
                field,
                message: format!("Invalid numeric bound: {error}"),
            });
            None
        }
    }
}

pub(super) fn parameter_definitions(
    metadata: &ModelDefinitionMetadata,
) -> BTreeMap<String, &ParameterDefinition> {
    metadata
        .parameters
        .iter()
        .map(|parameter| (parameter.name.to_ascii_lowercase(), parameter))
        .collect()
}
