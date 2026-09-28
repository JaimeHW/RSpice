//! Typed bulk component edits, validation and document-local commit.

use super::super::{
    component::{Component, ComponentDisplayMode},
    replacement::{
        format_replacement_parameters, parse_replacement_parameters_strict,
        valid_replacement_parameter_name,
    },
};
use super::{DocumentEdit, Schematic};
use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SelectionBulkProperty {
    #[default]
    ModelSection,
    Temperature,
    Tolerance,
    Display,
    ParameterOverride,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SelectionBulkUnsetBehavior {
    #[default]
    LeaveUnchanged,
    SetExplicitValue,
    RestoreInheritedValue,
}

pub fn apply_bulk_edit(
    schematic: &mut Schematic,
    target_ids: &BTreeSet<u64>,
    property: SelectionBulkProperty,
    value: &str,
    unset: SelectionBulkUnsetBehavior,
    current_property: Option<&str>,
) -> Result<DocumentEdit<Vec<u64>>, String> {
    let normalized = validate_bulk_value(property, value, unset, current_property)?;
    let parameter_key = if property == SelectionBulkProperty::ParameterOverride {
        parameter_key_from_filter(value, None).ok()
    } else {
        None
    };
    let mut replacements = Vec::new();
    let mut found = HashSet::new();
    for (index, component) in schematic.document.components.iter().enumerate() {
        if !target_ids.contains(&component.id) {
            continue;
        }
        found.insert(component.id);
        let mut candidate = component.clone();
        let changed = mutate_component(
            &mut candidate,
            property,
            &normalized,
            unset,
            parameter_key.as_deref(),
        )?;
        if changed {
            replacements.push((index, candidate));
        }
    }
    if found.len() != target_ids.len() {
        return Err("At least one selected component is no longer present.".to_owned());
    }
    if replacements.is_empty() {
        return Ok(DocumentEdit {
            value: Vec::new(),
            committed: false,
        });
    }
    schematic.begin_operation("Bulk edit schematic properties");
    let changed_ids = replacements
        .iter()
        .map(|(_, component)| component.id)
        .collect::<Vec<_>>();
    for (index, component) in replacements {
        schematic.document.components[index] = component;
    }
    let committed = schematic.end_operation();
    Ok(DocumentEdit {
        value: changed_ids,
        committed,
    })
}

pub fn mutate_component(
    component: &mut Component,
    property: SelectionBulkProperty,
    normalized: &str,
    unset: SelectionBulkUnsetBehavior,
    parameter_key_override: Option<&str>,
) -> Result<bool, String> {
    let restore = unset == SelectionBulkUnsetBehavior::RestoreInheritedValue
        || normalized.eq_ignore_ascii_case("inherit");
    match property {
        SelectionBulkProperty::ModelSection => {
            let Some(binding) = component.library_cell.as_mut() else {
                return Err(format!(
                    "{} has no library/cell binding for a model section.",
                    component.name
                ));
            };
            if unset == SelectionBulkUnsetBehavior::LeaveUnchanged
                && binding.model_section.is_none()
            {
                return Ok(false);
            }
            let next = (!restore).then(|| normalized.to_owned());
            if binding.model_section == next {
                return Ok(false);
            }
            binding.model_section = next;
            Ok(true)
        }
        SelectionBulkProperty::Temperature | SelectionBulkProperty::Tolerance => {
            let key = if property == SelectionBulkProperty::Temperature {
                "temp"
            } else {
                "tol"
            };
            mutate_parameter(component, key, normalized, unset, restore)
        }
        SelectionBulkProperty::Display => {
            if unset == SelectionBulkUnsetBehavior::LeaveUnchanged
                && component.display_mode == ComponentDisplayMode::Inherit
            {
                return Ok(false);
            }
            let next = if restore {
                ComponentDisplayMode::Inherit
            } else {
                ComponentDisplayMode::parse(normalized).ok_or_else(|| {
                    "Display must be name and value, name only, value only, hidden, or inherit."
                        .to_owned()
                })?
            };
            if component.display_mode == next {
                return Ok(false);
            }
            component.display_mode = next;
            Ok(true)
        }
        SelectionBulkProperty::ParameterOverride => {
            let (key, explicit_value) = normalized
                .split_once('=')
                .ok_or_else(|| "Parameter override must use name=value syntax.".to_owned())?;
            let key = parameter_key_override
                .unwrap_or(key)
                .trim()
                .to_ascii_lowercase();
            mutate_parameter(component, &key, explicit_value.trim(), unset, restore)
        }
    }
}

fn mutate_parameter(
    component: &mut Component,
    key: &str,
    value: &str,
    unset: SelectionBulkUnsetBehavior,
    restore: bool,
) -> Result<bool, String> {
    let mut parameters = parse_replacement_parameters_strict(&component.params)
        .map_err(|error| format!("{} has invalid parameters: {error}", component.name))?;
    let key = key.to_ascii_lowercase();
    if unset == SelectionBulkUnsetBehavior::LeaveUnchanged && !parameters.contains_key(&key) {
        return Ok(false);
    }
    if restore {
        if parameters.remove(&key).is_none() {
            return Ok(false);
        }
    } else if parameters.get(&key).is_some_and(|current| current == value) {
        return Ok(false);
    } else {
        parameters.insert(key, value.to_owned());
    }
    component.params = format_replacement_parameters(&parameters);
    Ok(true)
}

pub fn validate_bulk_value(
    property: SelectionBulkProperty,
    value: &str,
    unset: SelectionBulkUnsetBehavior,
    current_property: Option<&str>,
) -> Result<String, String> {
    let value = value.trim();
    if unset == SelectionBulkUnsetBehavior::RestoreInheritedValue {
        return if property == SelectionBulkProperty::ParameterOverride {
            parameter_key_from_filter(value, current_property).map(|key| format!("{key}=inherit"))
        } else {
            Ok("inherit".to_owned())
        };
    }
    if value.is_empty() {
        return Err("A new value is required.".to_owned());
    }
    match property {
        SelectionBulkProperty::ModelSection => {
            let normalized = value.to_ascii_lowercase();
            if !matches!(normalized.as_str(), "tt" | "ff" | "ss" | "inherit") {
                return Err("Model section must be tt, ff, ss, or inherit.".to_owned());
            }
            Ok(normalized)
        }
        SelectionBulkProperty::Temperature => {
            let parsed = value
                .parse::<f64>()
                .map_err(|_| "Temperature must be a finite Celsius value.".to_owned())?;
            if !parsed.is_finite() || parsed < -273.15 {
                return Err("Temperature must be finite and no lower than -273.15 °C.".to_owned());
            }
            Ok(value.to_owned())
        }
        SelectionBulkProperty::Tolerance => {
            let number = value.strip_suffix('%').unwrap_or(value).trim();
            let parsed = number
                .parse::<f64>()
                .map_err(|_| "Tolerance must be a non-negative finite number.".to_owned())?;
            if !parsed.is_finite() || parsed < 0.0 {
                return Err("Tolerance must be a non-negative finite number.".to_owned());
            }
            Ok(value.to_owned())
        }
        SelectionBulkProperty::Display => ComponentDisplayMode::parse(value)
            .map(|mode| mode.label().to_owned())
            .ok_or_else(|| {
                "Display must be name and value, name only, value only, hidden, or inherit."
                    .to_owned()
            }),
        SelectionBulkProperty::ParameterOverride => {
            let key = parameter_key_from_filter(value, current_property)?;
            let explicit_value = value
                .split_once('=')
                .map(|(_, value)| value)
                .unwrap_or(value)
                .trim();
            if !valid_replacement_parameter_name(&key) {
                return Err("Parameter override has an invalid parameter name.".to_owned());
            }
            if explicit_value.is_empty() {
                return Err("Parameter override requires a value after '='.".to_owned());
            }
            Ok(format!("{key}={explicit_value}"))
        }
    }
}

pub fn parameter_key_from_filter(
    new_value: &str,
    current_property: Option<&str>,
) -> Result<String, String> {
    let key = new_value
        .split_once('=')
        .map(|(key, _)| key)
        .or_else(|| {
            current_property.and_then(|expression| {
                expression
                    .split_once('=')
                    .map(|(key, _)| key)
                    .or(Some(expression))
            })
        })
        .unwrap_or_default()
        .trim();
    if !valid_replacement_parameter_name(key) {
        return Err("Parameter override requires a valid name=value property.".to_owned());
    }
    Ok(key.to_ascii_lowercase())
}
