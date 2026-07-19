//! Typed contract for replacing one placed schematic instance.
//!
//! Replacement retains the placed object's stable identity and placement while
//! changing its primitive or library master.  Connectivity and parameter
//! preservation are reviewed against explicit source and target contracts;
//! callers may supply authored-symbol terminal locations without coupling this
//! domain layer to the application library manager.

use std::fmt;

use super::{Component, ComponentType, LibraryCellInstance, Point, PortDirection};

/// One named terminal in an instance replacement contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchematicReplacementTerminal {
    name: String,
    direction: Option<PortDirection>,
    offset: Point,
    aliases: Vec<String>,
}

impl SchematicReplacementTerminal {
    pub fn new(name: impl Into<String>, offset: Point) -> Self {
        Self {
            name: name.into(),
            direction: None,
            offset,
            aliases: Vec::new(),
        }
    }

    pub fn with_direction(mut self, direction: PortDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn with_aliases(mut self, aliases: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.aliases = aliases.into_iter().map(Into::into).collect();
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn direction(&self) -> Option<PortDirection> {
        self.direction
    }

    pub const fn offset(&self) -> Point {
        self.offset
    }

    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }
}

/// One target parameter and the source spellings that may map to it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchematicReplacementParameter {
    name: String,
    aliases: Vec<String>,
    required: bool,
    default_value: Option<String>,
}

impl SchematicReplacementParameter {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            aliases: Vec::new(),
            required: false,
            default_value: None,
        }
    }

    pub fn with_aliases(mut self, aliases: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.aliases = aliases.into_iter().map(Into::into).collect();
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn with_default(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }

    pub const fn is_required(&self) -> bool {
        self.required
    }

    pub fn default_value(&self) -> Option<&str> {
        self.default_value.as_deref()
    }
}

/// Source-side compatibility contract captured with the selected instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchematicReplacementSourceSpec {
    terminals: Vec<SchematicReplacementTerminal>,
    parameter_keys: Vec<String>,
}

impl SchematicReplacementSourceSpec {
    pub fn new(
        terminals: Vec<SchematicReplacementTerminal>,
        parameter_keys: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            terminals,
            parameter_keys: parameter_keys.into_iter().map(Into::into).collect(),
        }
    }

    /// Derive the placed component's built-in/generated terminal contract.
    /// Authored symbol users should replace the terminals with [`Self::with_terminals`].
    pub fn from_component(component: &Component) -> Result<Self, SchematicReplacementError> {
        let terminals = default_component_terminals(component);
        let parameter_keys = parse_replacement_parameters_strict(&component.params)?
            .into_keys()
            .collect();
        Ok(Self {
            terminals,
            parameter_keys,
        })
    }

    pub fn with_terminals(mut self, terminals: Vec<SchematicReplacementTerminal>) -> Self {
        self.terminals = terminals;
        self
    }

    pub fn with_parameter_keys(
        mut self,
        parameter_keys: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.parameter_keys = parameter_keys.into_iter().map(Into::into).collect();
        self
    }

    pub fn terminals(&self) -> &[SchematicReplacementTerminal] {
        &self.terminals
    }

    pub fn parameter_keys(&self) -> &[String] {
        &self.parameter_keys
    }
}

/// How the target's primary value/model field is populated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SchematicReplacementValuePolicy {
    /// Use the value authored by the replacement target.
    #[default]
    UseTarget,
    /// Retain the placed source value. Useful for symbol-compatible primitive swaps.
    PreserveSource,
}

/// Self-contained primitive or library-cell replacement target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchematicReplacementTargetSpec {
    kind: ComponentType,
    value: String,
    default_params: String,
    symbol_variant: Option<String>,
    library_cell: Option<LibraryCellInstance>,
    terminals: Vec<SchematicReplacementTerminal>,
    parameters: Vec<SchematicReplacementParameter>,
    value_policy: SchematicReplacementValuePolicy,
}

impl SchematicReplacementTargetSpec {
    /// Built-in primitive target using its canonical generated terminals.
    pub fn primitive(kind: ComponentType) -> Self {
        let component = Component::new(0, kind, Point::origin());
        Self {
            kind,
            value: kind.default_value().to_owned(),
            default_params: String::new(),
            symbol_variant: None,
            library_cell: None,
            terminals: default_component_terminals(&component),
            parameters: Vec::new(),
            value_policy: SchematicReplacementValuePolicy::UseTarget,
        }
    }

    /// Hierarchical/library target using the binding's exact interface order.
    pub fn library_cell(binding: LibraryCellInstance) -> Self {
        let component = Component::new(0, ComponentType::CellInstance, Point::origin())
            .with_library_cell(binding.clone());
        Self {
            kind: ComponentType::CellInstance,
            value: binding.cell.clone(),
            default_params: String::new(),
            symbol_variant: None,
            library_cell: Some(binding),
            terminals: default_component_terminals(&component),
            parameters: Vec::new(),
            value_policy: SchematicReplacementValuePolicy::UseTarget,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_default_params(mut self, params: impl Into<String>) -> Self {
        self.default_params = params.into();
        self
    }

    pub fn with_symbol_variant(mut self, variant: Option<String>) -> Self {
        self.symbol_variant = variant;
        self
    }

    pub fn with_terminals(mut self, terminals: Vec<SchematicReplacementTerminal>) -> Self {
        self.terminals = terminals;
        self
    }

    pub fn with_parameters(mut self, parameters: Vec<SchematicReplacementParameter>) -> Self {
        self.parameters = parameters;
        self
    }

    pub fn with_value_policy(mut self, policy: SchematicReplacementValuePolicy) -> Self {
        self.value_policy = policy;
        self
    }

    pub const fn kind(&self) -> ComponentType {
        self.kind
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn default_params(&self) -> &str {
        &self.default_params
    }

    pub fn symbol_variant(&self) -> Option<&str> {
        self.symbol_variant.as_deref()
    }

    pub fn library_binding(&self) -> Option<&LibraryCellInstance> {
        self.library_cell.as_ref()
    }

    pub fn terminals(&self) -> &[SchematicReplacementTerminal] {
        &self.terminals
    }

    pub fn parameters(&self) -> &[SchematicReplacementParameter] {
        &self.parameters
    }

    pub const fn value_policy(&self) -> SchematicReplacementValuePolicy {
        self.value_policy
    }
}

/// Immutable authority for the exact selected component and source contract.
#[derive(Debug, Clone, PartialEq)]
pub struct SchematicReplacementAuthority {
    pub(crate) component_id: u64,
    pub(crate) topology_version: u64,
    pub(crate) source_component: Component,
    pub(crate) source_spec: SchematicReplacementSourceSpec,
}

impl SchematicReplacementAuthority {
    pub const fn component_id(&self) -> u64 {
        self.component_id
    }

    pub fn source_component(&self) -> &Component {
        &self.source_component
    }

    pub fn source_spec(&self) -> &SchematicReplacementSourceSpec {
        &self.source_spec
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchematicReplacementMappingStatus {
    Exact,
    Alias,
    Unmapped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchematicReplacementTerminalMapping {
    pub source: String,
    pub target: Option<String>,
    pub status: SchematicReplacementMappingStatus,
    pub connected: bool,
    /// Direction is advisory in positional SPICE netlists. A false value is
    /// review evidence, not an automatic connectivity rejection.
    pub direction_compatible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchematicReplacementParameterMapping {
    pub source: String,
    pub target: Option<String>,
    pub status: SchematicReplacementMappingStatus,
    pub has_authored_value: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchematicReplacementSemanticStatus {
    Preserved,
    CompatibleChange,
}

/// Complete compatibility evidence shown before committing replacement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchematicReplacementCompatibility {
    pub component_id: u64,
    pub source_terminal_count: usize,
    pub target_terminal_count: usize,
    pub mapped_terminal_count: usize,
    pub connected_terminal_count: usize,
    pub source_parameter_count: usize,
    pub target_parameter_count: usize,
    pub mapped_parameter_count: usize,
    pub terminal_mappings: Vec<SchematicReplacementTerminalMapping>,
    pub parameter_mappings: Vec<SchematicReplacementParameterMapping>,
    pub model_status: SchematicReplacementSemanticStatus,
    pub netlist_status: SchematicReplacementSemanticStatus,
}

impl SchematicReplacementCompatibility {
    pub fn all_connected_terminals_mapped(&self) -> bool {
        self.terminal_mappings
            .iter()
            .all(|mapping| !mapping.connected || mapping.target.is_some())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchematicReplacementWireEdit {
    pub wire_id: u64,
    pub original_points: Vec<Point>,
    pub replacement_points: Vec<Point>,
    /// Original point index to replacement route index. Connection records
    /// are updated through this map when orthogonal routing inserts a bend.
    pub point_indices: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchematicReplacementImpact {
    pub component_id: u64,
    pub preserved_connections: usize,
    pub relocated_wire_points: usize,
    pub preserved_parameters: usize,
    pub dropped_parameters: usize,
    pub topology_changed: bool,
}

/// Exact candidate produced by compatibility review and applied by commit.
#[derive(Debug, Clone, PartialEq)]
pub struct SchematicReplacementPreview {
    pub component: Component,
    pub connections: Vec<super::WireConnection>,
    pub wire_edits: Vec<SchematicReplacementWireEdit>,
    pub compatibility: SchematicReplacementCompatibility,
    pub impact: SchematicReplacementImpact,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchematicReplacementError {
    ReadOnly,
    SelectExactlyOneInstance,
    StaleAuthority,
    SourceInstanceMissing { component_id: u64 },
    InvalidSourceContract { reason: String },
    InvalidTargetContract { reason: String },
    DuplicateTerminalName { name: String },
    DuplicateParameterName { name: String },
    AmbiguousTerminalAlias { name: String },
    AmbiguousParameterAlias { name: String },
    UnmappedConnectedTerminal { terminal: String },
    IncompatibleTerminalDirection { source: String, target: String },
    MissingRequiredParameter { parameter: String },
    MalformedParameterString { reason: String },
    StaleConnection { wire_id: u64, point_index: usize },
    UnsupportedInteriorConnection { wire_id: u64, point_index: usize },
    SharedWireAnchor { wire_id: u64, point_index: usize },
    FixedElectricalAnchor { point: Point },
    DegenerateWire { wire_id: u64 },
    OrthogonalRouteUnavailable { wire_id: u64 },
    CoordinateOverflow,
    GeometryCollision { other_component_id: u64 },
    NoChanges,
    CommitFailed,
}

impl fmt::Display for SchematicReplacementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadOnly => formatter.write_str("The active schematic is read-only."),
            Self::SelectExactlyOneInstance => {
                formatter.write_str("Select exactly one complete component instance to replace.")
            }
            Self::StaleAuthority => formatter.write_str(
                "The selected instance or schematic topology changed; reopen replacement review.",
            ),
            Self::SourceInstanceMissing { component_id } => {
                write!(
                    formatter,
                    "Selected component {component_id} no longer exists."
                )
            }
            Self::InvalidSourceContract { reason } => {
                write!(
                    formatter,
                    "The source instance contract is invalid: {reason}."
                )
            }
            Self::InvalidTargetContract { reason } => {
                write!(formatter, "The replacement target is invalid: {reason}.")
            }
            Self::DuplicateTerminalName { name } => {
                write!(
                    formatter,
                    "Terminal name '{name}' is declared more than once."
                )
            }
            Self::DuplicateParameterName { name } => {
                write!(
                    formatter,
                    "Parameter name '{name}' is declared more than once."
                )
            }
            Self::AmbiguousTerminalAlias { name } => {
                write!(
                    formatter,
                    "Terminal alias '{name}' resolves to more than one target pin."
                )
            }
            Self::AmbiguousParameterAlias { name } => write!(
                formatter,
                "Parameter alias '{name}' resolves to more than one target parameter."
            ),
            Self::UnmappedConnectedTerminal { terminal } => write!(
                formatter,
                "Connected source terminal '{terminal}' has no replacement mapping."
            ),
            Self::IncompatibleTerminalDirection { source, target } => write!(
                formatter,
                "Source terminal '{source}' and replacement terminal '{target}' have incompatible directions."
            ),
            Self::MissingRequiredParameter { parameter } => write!(
                formatter,
                "Required replacement parameter '{parameter}' has no mapped value or default."
            ),
            Self::MalformedParameterString { reason } => {
                write!(formatter, "Component parameters are malformed: {reason}.")
            }
            Self::StaleConnection {
                wire_id,
                point_index,
            } => write!(
                formatter,
                "Wire {wire_id} point {point_index} no longer matches its recorded terminal."
            ),
            Self::UnsupportedInteriorConnection {
                wire_id,
                point_index,
            } => write!(
                formatter,
                "Wire {wire_id} point {point_index} is an interior terminal anchor and cannot be relocated safely."
            ),
            Self::SharedWireAnchor {
                wire_id,
                point_index,
            } => write!(
                formatter,
                "Wire {wire_id} point {point_index} is shared by another component connection."
            ),
            Self::FixedElectricalAnchor { point } => write!(
                formatter,
                "The terminal at ({}, {}) shares a fixed junction, label, or conductor anchor.",
                point.x, point.y
            ),
            Self::DegenerateWire { wire_id } => write!(
                formatter,
                "Relocating the terminal would create degenerate wire {wire_id}."
            ),
            Self::OrthogonalRouteUnavailable { wire_id } => write!(
                formatter,
                "Wire {wire_id} cannot retain a valid orthogonal route for the replacement pins."
            ),
            Self::CoordinateOverflow => formatter.write_str(
                "The replacement symbol exceeds the supported schematic coordinate range.",
            ),
            Self::GeometryCollision { other_component_id } => write!(
                formatter,
                "The replacement symbol collides with component {other_component_id}."
            ),
            Self::NoChanges => formatter
                .write_str("The selected instance already has the reviewed replacement contract."),
            Self::CommitFailed => formatter
                .write_str("The validated instance replacement could not be committed atomically."),
        }
    }
}

impl std::error::Error for SchematicReplacementError {}

pub(crate) fn default_component_terminals(
    component: &Component,
) -> Vec<SchematicReplacementTerminal> {
    if component.kind == ComponentType::CellInstance {
        return component
            .instance_pin_layout()
            .into_iter()
            .enumerate()
            .map(|(index, (name, offset))| {
                let direction = component
                    .library_cell
                    .as_ref()
                    .and_then(|binding| binding.terminal_dirs.get(index))
                    .copied();
                let mut terminal = SchematicReplacementTerminal::new(
                    name.unwrap_or_else(|| (index + 1).to_string()),
                    offset,
                );
                terminal.direction = direction;
                terminal
            })
            .collect();
    }

    component
        .kind
        .terminal_offsets()
        .iter()
        .map(|(name, offset)| SchematicReplacementTerminal::new(*name, *offset))
        .collect()
}

pub(crate) fn parse_replacement_parameters_strict(
    input: &str,
) -> Result<std::collections::HashMap<String, String>, SchematicReplacementError> {
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;
    let mut result = std::collections::HashMap::new();
    while index < chars.len() {
        while index < chars.len() && (chars[index].is_whitespace() || chars[index] == ',') {
            index += 1;
        }
        if index == chars.len() {
            break;
        }
        let key_start = index;
        while index < chars.len()
            && chars[index] != '='
            && !chars[index].is_whitespace()
            && chars[index] != ','
        {
            index += 1;
        }
        let key: String = chars[key_start..index].iter().collect();
        if !valid_replacement_parameter_name(&key) {
            return Err(SchematicReplacementError::MalformedParameterString {
                reason: format!("'{key}' is not a valid parameter name"),
            });
        }
        while index < chars.len() && chars[index].is_whitespace() {
            index += 1;
        }
        if index >= chars.len() || chars[index] != '=' {
            return Err(SchematicReplacementError::MalformedParameterString {
                reason: format!("parameter '{key}' is missing '='"),
            });
        }
        index += 1;
        while index < chars.len() && chars[index].is_whitespace() {
            index += 1;
        }
        if index >= chars.len() || chars[index] == ',' {
            return Err(SchematicReplacementError::MalformedParameterString {
                reason: format!("parameter '{key}' has no value"),
            });
        }
        let value_start = index;
        match chars[index] {
            quote @ ('\'' | '"') => {
                index += 1;
                let mut escaped = false;
                while index < chars.len() {
                    let character = chars[index];
                    index += 1;
                    if escaped {
                        escaped = false;
                    } else if character == '\\' {
                        escaped = true;
                    } else if character == quote {
                        break;
                    }
                }
                if chars.get(index.saturating_sub(1)) != Some(&quote) {
                    return Err(SchematicReplacementError::MalformedParameterString {
                        reason: format!("parameter '{key}' has an unterminated quoted value"),
                    });
                }
            }
            '{' => {
                let mut depth = 0usize;
                let mut quote = None;
                let mut escaped = false;
                while index < chars.len() {
                    let character = chars[index];
                    index += 1;
                    if let Some(active_quote) = quote {
                        if escaped {
                            escaped = false;
                        } else if character == '\\' {
                            escaped = true;
                        } else if character == active_quote {
                            quote = None;
                        }
                        continue;
                    }
                    match character {
                        '\'' | '"' => quote = Some(character),
                        '{' => depth += 1,
                        '}' => {
                            depth = depth.saturating_sub(1);
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                if depth != 0 || quote.is_some() {
                    return Err(SchematicReplacementError::MalformedParameterString {
                        reason: format!("parameter '{key}' has an unterminated brace expression"),
                    });
                }
            }
            _ => {
                while index < chars.len() && !chars[index].is_whitespace() && chars[index] != ',' {
                    index += 1;
                }
            }
        }
        let value: String = chars[value_start..index].iter().collect();
        if value.is_empty() {
            return Err(SchematicReplacementError::MalformedParameterString {
                reason: format!("parameter '{key}' has an empty value"),
            });
        }
        let key = key.trim().to_ascii_lowercase();
        if result.insert(key.clone(), value).is_some() {
            return Err(SchematicReplacementError::MalformedParameterString {
                reason: format!("parameter '{key}' is assigned more than once"),
            });
        }
        if index < chars.len() && !chars[index].is_whitespace() && chars[index] != ',' {
            return Err(SchematicReplacementError::MalformedParameterString {
                reason: format!("parameter '{key}' is not followed by a separator"),
            });
        }
    }
    Ok(result)
}

pub(crate) fn format_replacement_parameters(
    parameters: &std::collections::HashMap<String, String>,
) -> String {
    let mut parameters = parameters.iter().collect::<Vec<_>>();
    parameters.sort_by(|(left, _), (right, _)| left.cmp(right));
    parameters
        .into_iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn valid_replacement_parameter_name(name: &str) -> bool {
    let mut bytes = name.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_parameter_parser_is_lossless_for_spice_values() {
        let parsed = parse_replacement_parameters_strict(
            r#"m = 2, expr={a + nested({b, c})}, corner="tt slow", path='a\'b'"#,
        )
        .expect("valid SPICE parameter forms");
        assert_eq!(parsed.get("m").map(String::as_str), Some("2"));
        assert_eq!(
            parsed.get("expr").map(String::as_str),
            Some("{a + nested({b, c})}")
        );
        assert_eq!(
            parsed.get("corner").map(String::as_str),
            Some(r#""tt slow""#)
        );
        assert_eq!(parsed.get("path").map(String::as_str), Some(r#"'a\'b'"#));
        let formatted = format_replacement_parameters(&parsed);
        assert_eq!(
            parse_replacement_parameters_strict(&formatted).unwrap(),
            parsed
        );
    }

    #[test]
    fn strict_parameter_parser_rejects_unterminated_or_duplicate_values() {
        assert!(parse_replacement_parameters_strict("expr={a + b").is_err());
        assert!(parse_replacement_parameters_strict("corner=\"tt").is_err());
        assert!(parse_replacement_parameters_strict("m=1 M=2").is_err());
        assert!(parse_replacement_parameters_strict("m = , gain=2").is_err());
    }
}
