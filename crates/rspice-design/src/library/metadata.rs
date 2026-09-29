//! Authored cell and source-view port and parameter contracts.

use std::collections::{HashMap, HashSet};

use rspice_design_model::port::{PortDirection, PortSpec};
use serde::Deserialize;

use super::{Cell, View};
use crate::schematic::component::InstanceMultiplicity;

/// Lossless parameter contract authored by a cell or concrete implementation
/// view.  Legacy name lists remain supported as optional parameters, while a
/// structured contract carries aliases, requiredness, and exact defaults.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellParameter {
    pub name: String,
    pub aliases: Vec<String>,
    pub required: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct EncodedParameterContract {
    name: String,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    required: bool,
    #[serde(default, alias = "default_value")]
    default: Option<String>,
}

/// Read source-view port names using the stored metadata key precedence.
pub fn metadata_ports(metadata: &HashMap<String, String>) -> Option<Vec<PortSpec>> {
    let encoded = metadata
        .get("netlist.ports")
        .or_else(|| metadata.get("netlist.terminals"))
        .or_else(|| metadata.get("veriloga.ports"))?;
    let names = parse_name_list(encoded);
    Some(
        names
            .into_iter()
            .map(|name| PortSpec {
                name,
                direction: PortDirection::InOut,
            })
            .collect(),
    )
}

/// The parameters one cellview declares: the cell's contract with the view's
/// laid over it, name by name.
///
/// This is the single reader of the CDF contract metadata. Placement writes
/// the resulting order onto every instance and the hierarchy resolver writes
/// the same declarations into `.subckt … params:`, so neither can invent a
/// parameter the other has never heard of.
pub fn cell_parameter_contract(
    cell: &Cell,
    view: Option<&View>,
) -> Result<Vec<CellParameter>, String> {
    merge_parameter_contracts(
        metadata_parameter_contract(&cell.metadata),
        view.map_or_else(
            || Ok(Vec::new()),
            |view| metadata_parameter_contract(&view.metadata),
        ),
    )
}

fn metadata_parameter_contract(
    metadata: &HashMap<String, String>,
) -> Result<Vec<CellParameter>, String> {
    if let Some(encoded) = metadata
        .get("cdf.parameter_contract")
        .or_else(|| metadata.get("netlist.parameter_contract"))
        .or_else(|| metadata.get("veriloga.parameter_contract"))
    {
        let encoded = serde_json::from_str::<Vec<EncodedParameterContract>>(encoded)
            .map_err(|error| format!("parameter contract JSON is invalid: {error}"))?;
        let parameters = encoded
            .into_iter()
            .map(|parameter| CellParameter {
                name: parameter.name,
                aliases: parameter.aliases,
                required: parameter.required,
                default_value: parameter.default,
            })
            .collect::<Vec<_>>();
        validate_parameter_contract(&parameters)?;
        return Ok(parameters);
    }
    metadata
        .get("cdf.parameters")
        .or_else(|| metadata.get("netlist.parameters"))
        .or_else(|| metadata.get("veriloga.parameters"))
        .map_or_else(
            || Ok(Vec::new()),
            |encoded| {
                parse_parameter_names(encoded).map(|names| {
                    names
                        .into_iter()
                        .map(|name| CellParameter {
                            name,
                            aliases: Vec::new(),
                            required: false,
                            default_value: None,
                        })
                        .collect()
                })
            },
        )
}

fn parse_name_list(encoded: &str) -> Vec<String> {
    let parsed = serde_json::from_str::<Vec<String>>(encoded).unwrap_or_else(|_| {
        encoded
            .split([',', ' ', '\t', '\n'])
            .filter_map(|name| {
                let name = name.trim();
                (!name.is_empty()).then(|| name.to_owned())
            })
            .collect()
    });
    let mut seen = HashSet::new();
    parsed
        .into_iter()
        .filter(|name| seen.insert(name.to_ascii_lowercase()))
        .collect()
}

fn parse_parameter_names(encoded: &str) -> Result<Vec<String>, String> {
    let names = parse_name_list(encoded);
    if names.is_empty() && !encoded.trim().is_empty() {
        return Err("parameter metadata does not contain a valid name".to_owned());
    }
    for name in &names {
        validate_parameter_name(name)?;
    }
    Ok(names)
}

fn validate_parameter_contract(parameters: &[CellParameter]) -> Result<(), String> {
    let mut owners = HashMap::<String, String>::new();
    for parameter in parameters {
        for name in std::iter::once(&parameter.name).chain(&parameter.aliases) {
            validate_parameter_name(name)?;
            let key = name.to_ascii_lowercase();
            if let Some(owner) = owners.insert(key.clone(), parameter.name.clone()) {
                return Err(format!(
                    "parameter spelling `{key}` is owned by both `{owner}` and `{}`",
                    parameter.name
                ));
            }
        }
    }
    Ok(())
}

fn validate_parameter_name(name: &str) -> Result<(), String> {
    let mut chars = name.chars();
    let valid_first = chars
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic() || character == '_');
    if !valid_first || chars.any(|character| !character.is_ascii_alphanumeric() && character != '_')
    {
        return Err(format!("`{name}` is not a valid parameter name"));
    }
    if name.eq_ignore_ascii_case(InstanceMultiplicity::PARAMETER_NAME) {
        return Err(InstanceMultiplicity::RESERVED_GUIDANCE.to_owned());
    }
    Ok(())
}

fn merge_parameter_contracts(
    left: Result<Vec<CellParameter>, String>,
    right: Result<Vec<CellParameter>, String>,
) -> Result<Vec<CellParameter>, String> {
    let mut merged = left?;
    for parameter in right? {
        if let Some(existing) = merged
            .iter_mut()
            .find(|existing| existing.name.eq_ignore_ascii_case(&parameter.name))
        {
            *existing = parameter;
        } else {
            merged.push(parameter);
        }
    }
    validate_parameter_contract(&merged)?;
    Ok(merged)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_metadata_accepts_json_and_legacy_lists() {
        let mut metadata = HashMap::new();
        metadata.insert("veriloga.ports".to_owned(), r#"["in","out"]"#.to_owned());
        assert_eq!(metadata_ports(&metadata).unwrap().len(), 2);
        metadata.insert("veriloga.ports".to_owned(), "in, out vss".to_owned());
        assert_eq!(metadata_ports(&metadata).unwrap().len(), 3);
    }

    #[test]
    fn invalid_parameter_contract_is_not_silently_coerced() {
        assert!(parse_parameter_names("gain bad-name").is_err());
    }

    #[test]
    fn structured_parameter_contract_preserves_required_defaults_and_aliases() {
        let mut metadata = HashMap::new();
        metadata.insert(
            "cdf.parameter_contract".to_owned(),
            r#"[{"name":"gain","aliases":["av"],"required":true,"default":"10"}]"#.to_owned(),
        );
        let parameters = metadata_parameter_contract(&metadata).unwrap();
        assert_eq!(
            parameters,
            [CellParameter {
                name: "gain".to_owned(),
                aliases: vec!["av".to_owned()],
                required: true,
                default_value: Some("10".to_owned()),
            }]
        );
    }

    #[test]
    fn structured_parameter_contract_rejects_ambiguous_aliases_and_unknown_fields() {
        let mut metadata = HashMap::new();
        metadata.insert(
            "cdf.parameter_contract".to_owned(),
            r#"[{"name":"gain","aliases":["av"]},{"name":"av"}]"#.to_owned(),
        );
        assert!(metadata_parameter_contract(&metadata).is_err());
        metadata.insert(
            "cdf.parameter_contract".to_owned(),
            r#"[{"name":"gain","type":"real"}]"#.to_owned(),
        );
        assert!(metadata_parameter_contract(&metadata).is_err());
    }

    #[test]
    fn the_multiplicity_spelling_cannot_be_declared_as_a_cell_parameter() {
        let mut metadata = HashMap::new();
        metadata.insert(
            "cdf.parameter_contract".to_owned(),
            r#"[{"name":"M"}]"#.to_owned(),
        );
        assert_eq!(
            metadata_parameter_contract(&metadata).unwrap_err(),
            InstanceMultiplicity::RESERVED_GUIDANCE
        );

        metadata.insert(
            "cdf.parameter_contract".to_owned(),
            r#"[{"name":"gain","aliases":["m"]}]"#.to_owned(),
        );
        assert_eq!(
            metadata_parameter_contract(&metadata).unwrap_err(),
            InstanceMultiplicity::RESERVED_GUIDANCE
        );

        let legacy = HashMap::from([("cdf.parameters".to_owned(), "gain, m".to_owned())]);
        assert_eq!(
            metadata_parameter_contract(&legacy).unwrap_err(),
            InstanceMultiplicity::RESERVED_GUIDANCE
        );
    }
}
