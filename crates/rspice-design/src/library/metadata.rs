//! Authored cell and source-view port and parameter contracts.

use std::collections::{HashMap, HashSet};

use rspice_design_model::port::{PortDirection, PortSpec};
use serde::{Deserialize, Serialize};

use super::{Cell, View, ViewType};
use crate::schematic::component::InstanceMultiplicity;

/// Lossless parameter contract authored by a cell or concrete implementation
/// view.  Legacy name lists remain supported as optional parameters, while a
/// structured contract carries aliases, requiredness, and exact defaults.
#[derive(Debug, Clone, PartialEq)]
pub struct CellParameter {
    pub name: String,
    pub aliases: Vec<String>,
    pub required: bool,
    /// The default as source text; `None` for a compiled module's parameter
    /// means its module computes it from other parameters.
    pub default_value: Option<String>,
    /// What a compiled module declares beyond the name: type, scope, range,
    /// units and description. An authored CDF contract states none of it.
    pub declaration: Option<ParameterDeclaration>,
}

/// A compiled module's declaration of one of its parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterDeclaration {
    pub value_type: ParameterValueType,
    pub scope: ParameterScope,
    /// The values the module accepts; `None` when it accepts any.
    pub range: Option<ParameterRange>,
    pub units: Option<String>,
    pub description: Option<String>,
}

/// The type a module declares a parameter with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterValueType {
    Real,
    Integer,
    String,
}

/// Whether a module declares a parameter per instance or per model card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterScope {
    Instance,
    Model,
}

/// One `from` interval and its `exclude` values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParameterRange {
    pub lower: RangeBound,
    pub upper: RangeBound,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<RangeValue>,
}

/// One end of a `from` interval.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RangeBound {
    /// `-inf` or `inf`.
    Unbounded,
    /// `[` or `]`: the bound itself is accepted.
    Inclusive(RangeValue),
    /// `(` or `)`: the bound itself is refused.
    Exclusive(RangeValue),
}

/// A bound or excluded value: a number a property editor can enforce, or the
/// source text of one that depends on other parameters, which only the
/// compiled module can check once an instance's values are known.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RangeValue {
    Constant(f64),
    Dependent(String),
}

/// A parameter's number as source text: plain between 1m and 1Meg, and
/// otherwise in exponent form, so `1e-12` is never twelve zeros long. Either
/// form reads back to exactly the same number.
pub fn parameter_number_text(value: f64) -> String {
    let magnitude = value.abs();
    if magnitude == 0.0 || (1e-3..1e6).contains(&magnitude) {
        value.to_string()
    } else {
        format!("{value:e}")
    }
}

impl ParameterRange {
    /// The range as Verilog-A writes it: `from [0:inf) exclude 0.5`.
    pub fn source_text(&self) -> String {
        let value = |value: &RangeValue| match value {
            RangeValue::Constant(value) => parameter_number_text(*value),
            RangeValue::Dependent(text) => text.clone(),
        };
        let bounded = !matches!(
            (&self.lower, &self.upper),
            (RangeBound::Unbounded, RangeBound::Unbounded)
        ) || self.exclude.is_empty();
        let mut text = String::new();
        if bounded {
            let (open, lower) = match &self.lower {
                RangeBound::Unbounded => ('(', "-inf".to_owned()),
                RangeBound::Inclusive(bound) => ('[', value(bound)),
                RangeBound::Exclusive(bound) => ('(', value(bound)),
            };
            let (upper, close) = match &self.upper {
                RangeBound::Unbounded => ("inf".to_owned(), ')'),
                RangeBound::Inclusive(bound) => (value(bound), ']'),
                RangeBound::Exclusive(bound) => (value(bound), ')'),
            };
            text = format!("from {open}{lower}:{upper}{close}");
        }
        for excluded in &self.exclude {
            if !text.is_empty() {
                text.push(' ');
            }
            text.push_str("exclude ");
            text.push_str(&value(excluded));
        }
        text
    }

    /// Whether any bound or excluded value depends on other parameters.
    pub fn has_dependent_terms(&self) -> bool {
        let dependent = |bound: &RangeBound| {
            matches!(
                bound,
                RangeBound::Inclusive(RangeValue::Dependent(_))
                    | RangeBound::Exclusive(RangeValue::Dependent(_))
            )
        };
        dependent(&self.lower)
            || dependent(&self.upper)
            || self
                .exclude
                .iter()
                .any(|value| matches!(value, RangeValue::Dependent(_)))
    }
}

/// Metadata key under which a compiled Verilog-A view states its parameters.
pub const VERILOGA_PARAMETER_CONTRACT_KEY: &str = "veriloga.parameter_contract";

/// A model-bound symbol's emitted parameter fields, in instance-line order.
const MODEL_BOUND_PARAMETER_ORDER_KEY: &str = "netlist.parameter_order";

/// Every key a cell or view states a parameter contract under.
const PARAMETER_CONTRACT_KEYS: [&str; 6] = [
    "cdf.parameter_contract",
    "netlist.parameter_contract",
    VERILOGA_PARAMETER_CONTRACT_KEY,
    "cdf.parameters",
    "netlist.parameters",
    "veriloga.parameters",
];

/// One entry of a `veriloga.parameter_contract` value.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EncodedDeclaredParameter {
    name: String,
    #[serde(rename = "type")]
    value_type: ParameterValueType,
    scope: ParameterScope,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    range: Option<ParameterRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    units: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    aliases: Vec<String>,
}

/// `parameters` as a `veriloga.parameter_contract` value: a JSON array of
/// `{name, type, scope, default?, range?, units?, description?, aliases?}`
/// objects in declaration order. Every parameter must carry its declaration.
pub fn encode_veriloga_parameter_contract(parameters: &[CellParameter]) -> Result<String, String> {
    let encoded = parameters
        .iter()
        .map(|parameter| {
            let declaration = parameter
                .declaration
                .as_ref()
                .ok_or_else(|| format!("parameter `{}` carries no declaration", parameter.name))?;
            Ok(EncodedDeclaredParameter {
                name: parameter.name.clone(),
                value_type: declaration.value_type,
                scope: declaration.scope,
                default: parameter.default_value.clone(),
                range: declaration.range.clone(),
                units: declaration.units.clone(),
                description: declaration.description.clone(),
                aliases: parameter.aliases.clone(),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    serde_json::to_string(&encoded)
        .map_err(|error| format!("the parameter contract could not be encoded: {error}"))
}

fn decode_veriloga_parameter_contract(encoded: &str) -> Result<Vec<CellParameter>, String> {
    let encoded = serde_json::from_str::<Vec<EncodedDeclaredParameter>>(encoded)
        .map_err(|error| format!("Verilog-A parameter contract JSON is invalid: {error}"))?;
    Ok(encoded
        .into_iter()
        .map(|parameter| CellParameter {
            name: parameter.name,
            aliases: parameter.aliases,
            required: false,
            default_value: parameter.default,
            declaration: Some(ParameterDeclaration {
                value_type: parameter.value_type,
                scope: parameter.scope,
                range: parameter.range,
                units: parameter.units,
                description: parameter.description,
            }),
        })
        .collect())
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

/// Whether `cell` or `view` states a parameter contract at all. One that
/// states an empty contract accepts no parameter; one that states none passes
/// its instances' parameters through unchecked.
pub fn declares_parameter_contract(cell: &Cell, view: Option<&View>) -> bool {
    std::iter::once(&cell.metadata)
        .chain(view.map(|view| &view.metadata))
        .any(|metadata| {
            PARAMETER_CONTRACT_KEYS
                .iter()
                .any(|key| metadata.contains_key(*key))
        })
}

/// The parameters an instance of `view` may set, in the order its instance
/// line emits them, or `None` when its master declares none and an instance's
/// parameters pass through unchecked.
///
/// This is the one rule placement and configuration materialization bind an
/// allowlist by. Materialization reads it at netlist time, from the master as
/// it is then, so an instance placed before its module gained a parameter
/// accepts it. A
/// model-bound symbol states its form's emitted fields as
/// `netlist.parameter_order`, and its form, not this contract, is what its
/// property sheet presents. Every other master declares through
/// [`cell_parameter_contract`], the reader the property sheet uses, so what the
/// sheet offers and what the deck accepts are one list.
pub fn instance_parameter_order(cell: &Cell, view: &View) -> Result<Option<Vec<String>>, String> {
    if let Some(order) = [&view.metadata, &cell.metadata]
        .into_iter()
        .find_map(|metadata| metadata.get(MODEL_BOUND_PARAMETER_ORDER_KEY))
        .map(|encoded| parse_names(encoded))
        .filter(|order| !order.is_empty())
    {
        return Ok(Some(order));
    }
    if !declares_parameter_contract(cell, Some(view)) {
        return Ok(None);
    }
    Ok(Some(
        cell_parameter_contract(cell, Some(view))?
            .into_iter()
            .map(|parameter| parameter.name)
            .collect(),
    ))
}

fn metadata_parameter_contract(
    metadata: &HashMap<String, String>,
) -> Result<Vec<CellParameter>, String> {
    if let Some(encoded) = metadata
        .get("cdf.parameter_contract")
        .or_else(|| metadata.get("netlist.parameter_contract"))
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
                declaration: None,
            })
            .collect::<Vec<_>>();
        validate_parameter_contract(&parameters)?;
        return Ok(parameters);
    }
    if let Some(encoded) = metadata.get(VERILOGA_PARAMETER_CONTRACT_KEY) {
        let parameters = decode_veriloga_parameter_contract(encoded)?;
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
                            declaration: None,
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
                declaration: None,
            }]
        );
    }

    fn declared(
        name: &str,
        value_type: ParameterValueType,
        default: Option<&str>,
        range: Option<ParameterRange>,
    ) -> CellParameter {
        CellParameter {
            name: name.to_owned(),
            aliases: Vec::new(),
            required: false,
            default_value: default.map(str::to_owned),
            declaration: Some(ParameterDeclaration {
                value_type,
                scope: ParameterScope::Model,
                range,
                units: None,
                description: None,
            }),
        }
    }

    /// A compile writes the module's parameters with everything a property
    /// editor needs; the one reader gives them back as written.
    #[test]
    fn a_compiled_modules_contract_keeps_types_ranges_units_and_descriptions() {
        let mut gain = declared(
            "gain",
            ParameterValueType::Real,
            Some("2.5"),
            Some(ParameterRange {
                lower: RangeBound::Exclusive(RangeValue::Constant(0.0)),
                upper: RangeBound::Inclusive(RangeValue::Dependent("vmax".to_owned())),
                exclude: vec![RangeValue::Constant(1e-12)],
            }),
        );
        if let Some(declaration) = gain.declaration.as_mut() {
            declaration.scope = ParameterScope::Instance;
            declaration.units = Some("V".to_owned());
            declaration.description = Some("Small-signal gain".to_owned());
        }
        gain.aliases = vec!["av".to_owned()];
        let parameters = vec![
            gain,
            declared("stages", ParameterValueType::Integer, Some("3"), None),
            declared("vmax", ParameterValueType::Real, None, None),
        ];
        let encoded = encode_veriloga_parameter_contract(&parameters).unwrap();
        assert_eq!(
            encoded,
            r#"[{"name":"gain","type":"real","scope":"instance","default":"2.5","range":{"lower":{"exclusive":0.0},"upper":{"inclusive":"vmax"},"exclude":[1e-12]},"units":"V","description":"Small-signal gain","aliases":["av"]},{"name":"stages","type":"integer","scope":"model","default":"3"},{"name":"vmax","type":"real","scope":"model"}]"#
        );

        let mut view = View::new("veriloga", ViewType::VerilogA);
        view.metadata
            .insert(VERILOGA_PARAMETER_CONTRACT_KEY.to_owned(), encoded);
        let cell = Cell::new("amp");
        assert_eq!(
            cell_parameter_contract(&cell, Some(&view)).unwrap(),
            parameters
        );
        assert_eq!(
            parameters[0]
                .declaration
                .as_ref()
                .and_then(|declaration| declaration.range.as_ref())
                .map(ParameterRange::source_text)
                .as_deref(),
            Some("from (0:vmax] exclude 1e-12")
        );
        assert!(
            encode_veriloga_parameter_contract(&[CellParameter {
                declaration: None,
                ..parameters[1].clone()
            }])
            .is_err()
        );
    }

    /// The allowlist an instance binds is the contract the property sheet
    /// reads; a contract stated empty accepts nothing, and a master stating
    /// none leaves its instances unchecked.
    #[test]
    fn an_instances_allowlist_is_the_contract_its_property_sheet_reads() {
        let cell = Cell::new("amp");
        let mut view = View::new("veriloga", ViewType::VerilogA);
        assert_eq!(instance_parameter_order(&cell, &view), Ok(None));

        view.metadata.insert(
            VERILOGA_PARAMETER_CONTRACT_KEY.to_owned(),
            encode_veriloga_parameter_contract(&[
                declared("gain", ParameterValueType::Real, Some("1"), None),
                declared("Vth", ParameterValueType::Real, Some("0.4"), None),
            ])
            .unwrap(),
        );
        assert_eq!(
            instance_parameter_order(&cell, &view),
            Ok(Some(vec!["gain".to_owned(), "Vth".to_owned()]))
        );

        view.metadata
            .insert(VERILOGA_PARAMETER_CONTRACT_KEY.to_owned(), "[]".to_owned());
        assert!(declares_parameter_contract(&cell, Some(&view)));
        assert_eq!(instance_parameter_order(&cell, &view), Ok(Some(Vec::new())));

        // A model-bound symbol's emitted fields are its form's, not this
        // contract's.
        view.metadata
            .insert("netlist.parameter_order".to_owned(), "scale".to_owned());
        assert_eq!(
            instance_parameter_order(&cell, &view),
            Ok(Some(vec!["scale".to_owned()]))
        );
    }

    #[test]
    fn a_range_reads_as_the_module_writes_it() {
        let range = |lower, upper, exclude| ParameterRange {
            lower,
            upper,
            exclude,
        };
        assert_eq!(
            range(
                RangeBound::Inclusive(RangeValue::Constant(1.0)),
                RangeBound::Unbounded,
                Vec::new()
            )
            .source_text(),
            "from [1:inf)"
        );
        assert_eq!(
            range(
                RangeBound::Unbounded,
                RangeBound::Exclusive(RangeValue::Constant(2e6)),
                Vec::new()
            )
            .source_text(),
            "from (-inf:2e6)"
        );
        let exclude_only = range(
            RangeBound::Unbounded,
            RangeBound::Unbounded,
            vec![
                RangeValue::Constant(0.0),
                RangeValue::Dependent("forbidden".to_owned()),
            ],
        );
        assert_eq!(exclude_only.source_text(), "exclude 0 exclude forbidden");
        assert!(exclude_only.has_dependent_terms());
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
