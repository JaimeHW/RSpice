//! Authored cell and source-view port and parameter contracts.

use std::collections::{HashMap, HashSet};

use rspice_design_model::port::{PortDirection, PortSpec};
use serde::{Deserialize, Serialize};

use super::{Cell, View, ViewType};
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

/// Metadata key under which a Verilog-A view states its interface.
pub const VERILOGA_PORTS_KEY: &str = "veriloga.ports";

/// Whether a port's discipline is continuous or discrete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortDomain {
    Continuous,
    Discrete,
}

/// One port of a source view's interface, as `veriloga.ports` states it.
///
/// A continuous vector is one bus port named with its range, `d[7:0]`, whose
/// conductors expand in declaration order. A discrete vector is one port
/// under its bare name: its bits travel as one digital signal, so it is one
/// terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceViewPort {
    pub name: String,
    pub direction: PortDirection,
    /// The declared discipline, such as `electrical` or `logic`.
    pub discipline: String,
    pub domain: PortDomain,
}

impl SourceViewPort {
    pub fn spec(&self) -> PortSpec {
        PortSpec {
            name: self.name.clone(),
            direction: self.direction,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EncodedSourceViewPort {
    name: String,
    direction: String,
    discipline: String,
    domain: PortDomain,
}

/// `ports` as a `veriloga.ports` value: a JSON array of
/// `{name, direction, discipline, domain}` objects.
pub fn encode_source_view_ports(ports: &[SourceViewPort]) -> Result<String, String> {
    serde_json::to_string(
        &ports
            .iter()
            .map(|port| EncodedSourceViewPort {
                name: port.name.clone(),
                direction: port.direction.keyword().to_owned(),
                discipline: port.discipline.clone(),
                domain: port.domain,
            })
            .collect::<Vec<_>>(),
    )
    .map_err(|error| format!("the interface could not be encoded: {error}"))
}

/// The ports a `veriloga.ports` value states with their disciplines, when it
/// states them. A legacy names-only list states none.
pub fn decode_source_view_ports(encoded: &str) -> Option<Vec<SourceViewPort>> {
    let encoded = serde_json::from_str::<Vec<EncodedSourceViewPort>>(encoded).ok()?;
    Some(
        encoded
            .into_iter()
            .map(|port| SourceViewPort {
                name: port.name,
                direction: PortDirection::parse(&port.direction),
                discipline: port.discipline,
                domain: port.domain,
            })
            .collect(),
    )
}

/// The ports a port-list value states, in order. The structured form carries
/// each port's direction; a legacy list of names states none, so each of its
/// ports is `inout`.
pub fn decode_port_list(encoded: &str) -> Vec<PortSpec> {
    match decode_source_view_ports(encoded) {
        Some(ports) => ports.iter().map(SourceViewPort::spec).collect(),
        None => parse_names(encoded)
            .into_iter()
            .map(|name| PortSpec {
                name,
                direction: PortDirection::InOut,
            })
            .collect(),
    }
}

/// The encoded port list a source view or its cell states, by key precedence.
pub(crate) fn metadata_port_list(metadata: &HashMap<String, String>) -> Option<&str> {
    metadata
        .get("netlist.ports")
        .or_else(|| metadata.get("netlist.terminals"))
        .or_else(|| metadata.get(VERILOGA_PORTS_KEY))
        .map(String::as_str)
}

/// The Verilog-A view whose module defines `cell`'s ports.
///
/// A cell's `schematic` view, when it has one, defines its ports, and a
/// module that disagrees with it is a finding about the module. Otherwise the
/// last clean compile of its Verilog-A view does.
pub fn module_interface_view(cell: &Cell) -> Option<&View> {
    if cell.get_view("schematic").is_some() {
        return None;
    }
    cell.views_sorted()
        .into_iter()
        .find(|view| view.view_type == ViewType::VerilogA)
}

/// Read source-view ports using the stored metadata key precedence.
pub fn metadata_ports(metadata: &HashMap<String, String>) -> Option<Vec<PortSpec>> {
    let mut seen = HashSet::new();
    Some(
        decode_port_list(metadata_port_list(metadata)?)
            .into_iter()
            .filter(|port| seen.insert(port.name.to_ascii_lowercase()))
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

/// A JSON array of names, or names separated by commas or whitespace.
fn parse_names(encoded: &str) -> Vec<String> {
    serde_json::from_str::<Vec<String>>(encoded).unwrap_or_else(|_| {
        encoded
            .split([',', ' ', '\t', '\n'])
            .filter_map(|name| {
                let name = name.trim();
                (!name.is_empty()).then(|| name.to_owned())
            })
            .collect()
    })
}

fn parse_name_list(encoded: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    parse_names(encoded)
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

    /// The structured interface a compile writes round-trips with each port's
    /// direction and discipline; readers used to force every port `inout`.
    #[test]
    fn a_structured_interface_keeps_its_directions_and_a_legacy_one_reads_inout() {
        let port = |name: &str, direction, discipline: &str, domain| SourceViewPort {
            name: name.to_owned(),
            direction,
            discipline: discipline.to_owned(),
            domain,
        };
        let ports = [
            port(
                "sense",
                PortDirection::In,
                "electrical",
                PortDomain::Continuous,
            ),
            port(
                "d[3:0]",
                PortDirection::InOut,
                "electrical",
                PortDomain::Continuous,
            ),
            port("mode", PortDirection::Out, "logic", PortDomain::Discrete),
        ];
        let encoded = encode_source_view_ports(&ports).unwrap();
        assert_eq!(
            encoded,
            r#"[{"name":"sense","direction":"in","discipline":"electrical","domain":"continuous"},{"name":"d[3:0]","direction":"inout","discipline":"electrical","domain":"continuous"},{"name":"mode","direction":"out","discipline":"logic","domain":"discrete"}]"#
        );
        assert_eq!(
            decode_source_view_ports(&encoded).as_deref(),
            Some(&ports[..])
        );

        let mut metadata = HashMap::from([(VERILOGA_PORTS_KEY.to_owned(), encoded)]);
        let read = |metadata: &HashMap<String, String>| {
            metadata_ports(metadata)
                .unwrap()
                .into_iter()
                .map(|port| (port.name, port.direction))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            read(&metadata),
            [
                ("sense".to_owned(), PortDirection::In),
                ("d[3:0]".to_owned(), PortDirection::InOut),
                ("mode".to_owned(), PortDirection::Out),
            ]
        );

        metadata.insert(
            VERILOGA_PORTS_KEY.to_owned(),
            r#"["sense","mode"]"#.to_owned(),
        );
        assert_eq!(
            decode_source_view_ports(&metadata[VERILOGA_PORTS_KEY]),
            None
        );
        assert_eq!(
            read(&metadata),
            [
                ("sense".to_owned(), PortDirection::InOut),
                ("mode".to_owned(), PortDirection::InOut),
            ]
        );
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
