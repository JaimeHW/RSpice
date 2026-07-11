//! Parser for ngspice XSPICE `ifspec.ifs` interface specifications.
//!
//! ngspice treats these files as the authoritative source for code-model port
//! and parameter metadata. RSpice keeps its model metadata in Rust, so this
//! parser exists to make parity auditable instead of relying on manual review.

use crate::{Complex64, Value};
use std::fmt;

use super::{ParamType, PortDirection, PortType};

#[derive(Debug, Clone, PartialEq)]
pub struct IfSpec {
    pub c_function_name: Option<String>,
    pub spice_model_name: String,
    pub description: Option<String>,
    pub ports: Vec<IfSpecPort>,
    pub parameters: Vec<IfSpecParameter>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfSpecPort {
    pub name: String,
    pub description: Option<String>,
    pub direction: PortDirection,
    pub default_type: PortType,
    pub allowed_types: Vec<PortType>,
    pub is_vector: bool,
    pub vector_bounds: IfSpecBounds,
    pub null_allowed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfSpecParameter {
    pub name: String,
    pub description: Option<String>,
    pub param_type: IfSpecParamType,
    pub default: IfSpecDefault,
    pub limits: IfSpecBounds,
    pub is_vector: bool,
    pub vector_bounds: IfSpecBounds,
    pub null_allowed: bool,
}

impl IfSpecParameter {
    pub fn required(&self) -> bool {
        matches!(self.default, IfSpecDefault::None) && !self.null_allowed
    }

    pub fn rspice_param_type(&self) -> Option<ParamType> {
        match (self.param_type, self.is_vector) {
            (IfSpecParamType::Real, false) => Some(ParamType::Real),
            (IfSpecParamType::Real, true) => Some(ParamType::RealVector),
            (IfSpecParamType::Integer, false) => Some(ParamType::Integer),
            (IfSpecParamType::Integer, true) => Some(ParamType::IntegerVector),
            (IfSpecParamType::Boolean, false) => Some(ParamType::Boolean),
            (IfSpecParamType::Boolean, true) => Some(ParamType::IntegerVector),
            (IfSpecParamType::Complex, false) => Some(ParamType::Complex),
            (IfSpecParamType::Complex, true) => Some(ParamType::ComplexVector),
            (IfSpecParamType::String, false) => Some(ParamType::String),
            (IfSpecParamType::String, true) => Some(ParamType::StringVector),
            (IfSpecParamType::Pointer, _) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfSpecParamType {
    Real,
    Integer,
    Boolean,
    Complex,
    String,
    Pointer,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfSpecDefault {
    None,
    Real(Value),
    Integer(i64),
    Boolean(bool),
    Complex(Complex64),
    String(String),
    RealVector(Vec<Value>),
    IntegerVector(Vec<i64>),
    ComplexVector(Vec<Complex64>),
    StringVector(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfSpecBounds {
    None,
    Range {
        min: Option<IfSpecBoundValue>,
        max: Option<IfSpecBoundValue>,
    },
    SameAs(String),
}

impl IfSpecBounds {
    pub fn min_usize(&self) -> Option<usize> {
        match self {
            Self::Range {
                min: Some(IfSpecBoundValue::Integer(value)),
                ..
            } if *value >= 0 => Some(*value as usize),
            _ => None,
        }
    }

    pub fn max_usize(&self) -> Option<usize> {
        match self {
            Self::Range {
                max: Some(IfSpecBoundValue::Integer(value)),
                ..
            } if *value >= 0 => Some(*value as usize),
            _ => None,
        }
    }

    pub fn min_real(&self) -> Option<Value> {
        match self {
            Self::Range {
                min: Some(value), ..
            } => Some(value.as_real()),
            _ => None,
        }
    }

    pub fn max_real(&self) -> Option<Value> {
        match self {
            Self::Range {
                max: Some(value), ..
            } => Some(value.as_real()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfSpecBoundValue {
    Integer(i64),
    Decimal(IfSpecDecimal),
}

impl IfSpecBoundValue {
    pub fn as_real(self) -> Value {
        match self {
            Self::Integer(value) => value as Value,
            Self::Decimal(value) => value.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IfSpecDecimal(pub Value);

impl Eq for IfSpecDecimal {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfSpecParseError {
    message: String,
}

impl IfSpecParseError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for IfSpecParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for IfSpecParseError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SectionKind {
    Name,
    Port,
    Parameter,
    StaticVar,
    Other,
}

#[derive(Debug, Clone)]
struct Section {
    kind: SectionKind,
    rows: Vec<(String, Vec<String>)>,
}

pub fn parse_ifspec(input: &str) -> Result<IfSpec, IfSpecParseError> {
    let uncommented = strip_c_comments(input);
    let sections = collect_sections(&uncommented)?;

    let mut spec = IfSpec {
        c_function_name: None,
        spice_model_name: String::new(),
        description: None,
        ports: Vec::new(),
        parameters: Vec::new(),
    };

    for section in sections {
        match section.kind {
            SectionKind::Name => apply_name_table(&mut spec, &section.rows)?,
            SectionKind::Port => spec.ports.extend(parse_port_table(&section.rows)?),
            SectionKind::Parameter => spec
                .parameters
                .extend(parse_parameter_table(&section.rows)?),
            SectionKind::StaticVar | SectionKind::Other => {}
        }
    }

    if spec.spice_model_name.is_empty() {
        return Err(IfSpecParseError::new("missing Spice_Model_Name"));
    }

    Ok(spec)
}

fn strip_c_comments(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_comment = false;

    while let Some(ch) = chars.next() {
        if in_comment {
            if ch == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_comment = false;
            } else if ch == '\n' {
                output.push('\n');
            }
            continue;
        }

        if ch == '/' && chars.peek() == Some(&'*') {
            chars.next();
            in_comment = true;
            continue;
        }

        output.push(ch);
    }

    output
}

fn collect_sections(input: &str) -> Result<Vec<Section>, IfSpecParseError> {
    let mut sections = Vec::new();
    let mut current: Option<Section> = None;

    for (line_index, line) in input.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(kind) = parse_section_marker(trimmed) {
            if let Some(section) = current.take() {
                sections.push(section);
            }
            current = Some(Section {
                kind,
                rows: Vec::new(),
            });
            continue;
        }

        let Some(section) = current.as_mut() else {
            continue;
        };
        if matches!(section.kind, SectionKind::Other) {
            continue;
        }

        if let Some((key, raw_values)) = split_table_row(trimmed) {
            let values = tokenize_values(raw_values).map_err(|message| {
                IfSpecParseError::new(format!("line {}: {message}", line_index + 1))
            })?;
            section.rows.push((key.to_string(), values));
        }
    }

    if let Some(section) = current {
        sections.push(section);
    }

    Ok(sections)
}

fn parse_section_marker(line: &str) -> Option<SectionKind> {
    let marker = line.strip_suffix(':')?.trim();
    match marker.to_ascii_uppercase().as_str() {
        "NAME_TABLE" => Some(SectionKind::Name),
        "PORT_TABLE" => Some(SectionKind::Port),
        "PARAMETER_TABLE" => Some(SectionKind::Parameter),
        "STATIC_VAR_TABLE" => Some(SectionKind::StaticVar),
        _ if marker.ends_with("_TABLE") => Some(SectionKind::Other),
        _ => None,
    }
}

fn split_table_row(line: &str) -> Option<(&str, &str)> {
    let (key, value) = line.split_once(':')?;
    Some((key.trim(), value.trim()))
}

fn tokenize_values(input: &str) -> Result<Vec<String>, String> {
    let mut values = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.peek().copied() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }

        if ch == '"' {
            chars.next();
            let mut token = String::new();
            let mut closed = false;
            for value_ch in chars.by_ref() {
                if value_ch == '"' {
                    closed = true;
                    break;
                }
                token.push(value_ch);
            }
            if !closed {
                return Err("unterminated quoted string".to_string());
            }
            values.push(token);
            continue;
        }

        if ch == '[' {
            chars.next();
            let mut token = String::from("[");
            let mut closed = false;
            for value_ch in chars.by_ref() {
                token.push(value_ch);
                if value_ch == ']' {
                    closed = true;
                    break;
                }
            }
            if !closed {
                return Err("unterminated bracket value".to_string());
            }
            values.push(token);
            continue;
        }

        if ch == '<' {
            chars.next();
            let mut token = String::from("<");
            let mut closed = false;
            for value_ch in chars.by_ref() {
                token.push(value_ch);
                if value_ch == '>' {
                    closed = true;
                    break;
                }
            }
            if !closed {
                return Err("unterminated angle-bracket value".to_string());
            }
            values.push(token);
            continue;
        }

        let mut token = String::new();
        while let Some(value_ch) = chars.peek().copied() {
            if value_ch.is_whitespace() {
                break;
            }
            token.push(value_ch);
            chars.next();
        }
        values.push(token);
    }

    Ok(values)
}

fn apply_name_table(
    spec: &mut IfSpec,
    rows: &[(String, Vec<String>)],
) -> Result<(), IfSpecParseError> {
    for (key, values) in rows {
        let value = values.first().cloned();
        match key {
            key if key.eq_ignore_ascii_case("C_Function_Name") => {
                spec.c_function_name = value;
            }
            key if key.eq_ignore_ascii_case("Spice_Model_Name") => {
                spec.spice_model_name = value.unwrap_or_default();
            }
            key if key.eq_ignore_ascii_case("Description") => {
                spec.description = value;
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_port_table(rows: &[(String, Vec<String>)]) -> Result<Vec<IfSpecPort>, IfSpecParseError> {
    let names = required_row(rows, "Port_Name")?;
    let count = names.len();
    let descriptions = optional_row(rows, "Description", count)?;
    let directions = required_row_exact(rows, "Direction", count)?;
    let default_types = required_row_exact(rows, "Default_Type", count)?;
    let allowed_types = required_row_exact(rows, "Allowed_Types", count)?;
    let vectors = required_row_exact(rows, "Vector", count)?;
    let bounds = required_row_exact(rows, "Vector_Bounds", count)?;
    let nulls = required_row_exact(rows, "Null_Allowed", count)?;

    let mut ports = Vec::with_capacity(count);
    for index in 0..count {
        ports.push(IfSpecPort {
            name: names[index].clone(),
            description: descriptions[index].clone(),
            direction: parse_direction(&directions[index])?,
            default_type: parse_port_type(&default_types[index])?,
            allowed_types: parse_allowed_types(&allowed_types[index])?,
            is_vector: parse_yes_no(&vectors[index], "Vector")?,
            vector_bounds: parse_bounds(&bounds[index])?,
            null_allowed: parse_yes_no(&nulls[index], "Null_Allowed")?,
        });
    }
    Ok(ports)
}

fn parse_parameter_table(
    rows: &[(String, Vec<String>)],
) -> Result<Vec<IfSpecParameter>, IfSpecParseError> {
    let names = required_row(rows, "Parameter_Name")?;
    let count = names.len();
    let descriptions = optional_row(rows, "Description", count)?;
    let data_types = required_row_exact(rows, "Data_Type", count)?;
    let defaults = required_row_exact(rows, "Default_Value", count)?;
    let limits = required_row_exact(rows, "Limits", count)?;
    let vectors = required_row_exact(rows, "Vector", count)?;
    let bounds = required_row_exact(rows, "Vector_Bounds", count)?;
    let nulls = required_row_exact(rows, "Null_Allowed", count)?;

    let mut params = Vec::with_capacity(count);
    for index in 0..count {
        let param_type = parse_param_type(&data_types[index])?;
        let is_vector = parse_yes_no(&vectors[index], "Vector")?;
        params.push(IfSpecParameter {
            name: names[index].clone(),
            description: descriptions[index].clone(),
            param_type,
            default: parse_default(param_type, is_vector, &defaults[index])?,
            limits: parse_bounds(&limits[index])?,
            is_vector,
            vector_bounds: parse_bounds(&bounds[index])?,
            null_allowed: parse_yes_no(&nulls[index], "Null_Allowed")?,
        });
    }
    Ok(params)
}

fn required_row<'a>(
    rows: &'a [(String, Vec<String>)],
    key: &str,
) -> Result<&'a [String], IfSpecParseError> {
    rows.iter()
        .find(|(row_key, _)| row_key.eq_ignore_ascii_case(key))
        .map(|(_, values)| values.as_slice())
        .ok_or_else(|| IfSpecParseError::new(format!("missing {key} row")))
}

fn required_row_exact<'a>(
    rows: &'a [(String, Vec<String>)],
    key: &str,
    count: usize,
) -> Result<&'a [String], IfSpecParseError> {
    let row = required_row(rows, key)?;
    if row.len() != count {
        return Err(IfSpecParseError::new(format!(
            "{key} row has {} value(s), expected {count}",
            row.len()
        )));
    }
    Ok(row)
}

fn optional_row(
    rows: &[(String, Vec<String>)],
    key: &str,
    count: usize,
) -> Result<Vec<Option<String>>, IfSpecParseError> {
    let Some((_, values)) = rows
        .iter()
        .find(|(row_key, _)| row_key.eq_ignore_ascii_case(key))
    else {
        return Ok(vec![None; count]);
    };

    if values.len() != count {
        return Err(IfSpecParseError::new(format!(
            "{key} row has {} value(s), expected {count}",
            values.len()
        )));
    }

    Ok(values.iter().cloned().map(Some).collect())
}

fn parse_direction(value: &str) -> Result<PortDirection, IfSpecParseError> {
    match value.to_ascii_lowercase().as_str() {
        "in" => Ok(PortDirection::In),
        "out" => Ok(PortDirection::Out),
        "inout" | "in_out" | "in-out" => Ok(PortDirection::InOut),
        _ => Err(IfSpecParseError::new(format!(
            "unknown port direction '{value}'"
        ))),
    }
}

fn parse_port_type(value: &str) -> Result<PortType, IfSpecParseError> {
    match value.to_ascii_lowercase().as_str() {
        "v" => Ok(PortType::Voltage),
        "vd" => Ok(PortType::DifferentialVoltage),
        "g" => Ok(PortType::Conductance),
        "gd" => Ok(PortType::DifferentialConductance),
        "h" => Ok(PortType::Hybrid),
        "hd" => Ok(PortType::DifferentialHybrid),
        "i" => Ok(PortType::Current),
        "id" => Ok(PortType::DifferentialCurrent),
        "vnam" => Ok(PortType::VoltageName),
        "d" => Ok(PortType::Digital),
        "real" => Ok(PortType::Real),
        "int" => Ok(PortType::Integer),
        _ => Err(IfSpecParseError::new(format!(
            "unknown port type '{value}'"
        ))),
    }
}

fn parse_allowed_types(value: &str) -> Result<Vec<PortType>, IfSpecParseError> {
    parse_bracket_list(value)?
        .into_iter()
        .map(|value| parse_port_type(&value))
        .collect()
}

fn parse_param_type(value: &str) -> Result<IfSpecParamType, IfSpecParseError> {
    match value.to_ascii_lowercase().as_str() {
        "real" => Ok(IfSpecParamType::Real),
        "int" | "integer" => Ok(IfSpecParamType::Integer),
        "boolean" | "bool" => Ok(IfSpecParamType::Boolean),
        "complex" => Ok(IfSpecParamType::Complex),
        "string" => Ok(IfSpecParamType::String),
        "pointer" => Ok(IfSpecParamType::Pointer),
        _ => Err(IfSpecParseError::new(format!(
            "unknown parameter type '{value}'"
        ))),
    }
}

fn parse_yes_no(value: &str, field: &str) -> Result<bool, IfSpecParseError> {
    match value.to_ascii_lowercase().as_str() {
        "yes" | "true" => Ok(true),
        "no" | "false" => Ok(false),
        _ => Err(IfSpecParseError::new(format!(
            "invalid {field} value '{value}'"
        ))),
    }
}

fn parse_default(
    param_type: IfSpecParamType,
    is_vector: bool,
    value: &str,
) -> Result<IfSpecDefault, IfSpecParseError> {
    if value == "-" {
        return Ok(IfSpecDefault::None);
    }

    match (param_type, is_vector) {
        (IfSpecParamType::Real, false) => Ok(IfSpecDefault::Real(parse_real(value)?)),
        (IfSpecParamType::Real, true) => parse_value_list(value)
            .into_iter()
            .map(|value| parse_real(&value))
            .collect::<Result<Vec<_>, _>>()
            .map(IfSpecDefault::RealVector),
        (IfSpecParamType::Integer, false) => Ok(IfSpecDefault::Integer(parse_integer(value)?)),
        (IfSpecParamType::Integer, true) => parse_value_list(value)
            .into_iter()
            .map(|value| parse_integer(&value))
            .collect::<Result<Vec<_>, _>>()
            .map(IfSpecDefault::IntegerVector),
        (IfSpecParamType::Boolean, false) => Ok(IfSpecDefault::Boolean(parse_bool(value)?)),
        (IfSpecParamType::Boolean, true) => parse_value_list(value)
            .into_iter()
            .map(|value| parse_bool(&value).map(i64::from))
            .collect::<Result<Vec<_>, _>>()
            .map(IfSpecDefault::IntegerVector),
        (IfSpecParamType::Complex, false) => Ok(IfSpecDefault::Complex(parse_complex(value)?)),
        (IfSpecParamType::Complex, true) => {
            Ok(IfSpecDefault::ComplexVector(vec![parse_complex(value)?]))
        }
        (IfSpecParamType::String, false) => Ok(IfSpecDefault::String(value.to_string())),
        (IfSpecParamType::String, true) => {
            Ok(IfSpecDefault::StringVector(if value.starts_with('[') {
                parse_bracket_list(value)?
            } else {
                vec![value.to_string()]
            }))
        }
        (IfSpecParamType::Pointer, _) => Ok(IfSpecDefault::None),
    }
}

fn parse_bounds(value: &str) -> Result<IfSpecBounds, IfSpecParseError> {
    if value == "-" {
        return Ok(IfSpecBounds::None);
    }

    if value.starts_with('[') {
        let parts = parse_bracket_list(value)?;
        if parts.len() != 2 {
            return Err(IfSpecParseError::new(format!(
                "bounds '{value}' must have exactly two endpoints"
            )));
        }
        return Ok(IfSpecBounds::Range {
            min: parse_bound_endpoint(&parts[0])?,
            max: parse_bound_endpoint(&parts[1])?,
        });
    }

    Ok(IfSpecBounds::SameAs(value.to_string()))
}

fn parse_bound_endpoint(value: &str) -> Result<Option<IfSpecBoundValue>, IfSpecParseError> {
    if value == "-" {
        return Ok(None);
    }

    if value.contains(['.', 'e', 'E']) {
        return Ok(Some(IfSpecBoundValue::Decimal(IfSpecDecimal(parse_real(
            value,
        )?))));
    }

    Ok(Some(IfSpecBoundValue::Integer(parse_integer(value)?)))
}

fn parse_bracket_list(value: &str) -> Result<Vec<String>, IfSpecParseError> {
    let Some(inner) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    else {
        return Err(IfSpecParseError::new(format!(
            "expected bracket list, got '{value}'"
        )));
    };

    Ok(inner
        .split([',', ' ', '\t'])
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect())
}

fn parse_value_list(value: &str) -> Vec<String> {
    if value.starts_with('[') {
        parse_bracket_list(value).unwrap_or_else(|_| vec![value.to_string()])
    } else {
        vec![value.to_string()]
    }
}

fn parse_real(value: &str) -> Result<Value, IfSpecParseError> {
    value
        .parse::<Value>()
        .map_err(|err| IfSpecParseError::new(format!("invalid real '{value}': {err}")))
}

fn parse_integer(value: &str) -> Result<i64, IfSpecParseError> {
    value
        .parse::<i64>()
        .map_err(|err| IfSpecParseError::new(format!("invalid integer '{value}': {err}")))
}

fn parse_bool(value: &str) -> Result<bool, IfSpecParseError> {
    match value.to_ascii_lowercase().as_str() {
        "true" | "yes" | "1" => Ok(true),
        "false" | "no" | "0" => Ok(false),
        _ => Err(IfSpecParseError::new(format!("invalid boolean '{value}'"))),
    }
}

fn parse_complex(value: &str) -> Result<Complex64, IfSpecParseError> {
    let parts = parse_delimited_numeric_pair(value)?;
    if parts.len() != 2 {
        return Err(IfSpecParseError::new(format!(
            "complex default '{value}' must have real and imaginary parts"
        )));
    }
    Ok(Complex64::new(
        parse_real(&parts[0])?,
        parse_real(&parts[1])?,
    ))
}

fn parse_delimited_numeric_pair(value: &str) -> Result<Vec<String>, IfSpecParseError> {
    let inner = if let Some(inner) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        inner
    } else if let Some(inner) = value
        .strip_prefix('<')
        .and_then(|value| value.strip_suffix('>'))
    {
        inner
    } else {
        return Err(IfSpecParseError::new(format!(
            "expected bracket or angle pair, got '{value}'"
        )));
    };

    Ok(inner
        .split([',', ' ', '\t'])
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_handles_multiple_port_and_parameter_tables() {
        let spec = parse_ifspec(
            r#"
NAME_TABLE:
Spice_Model_Name:      demo
C_Function_Name:       cm_demo
Description:           "demo model"

PORT_TABLE:
Port_Name:             in             clk
Description:           "input"        "clock"
Direction:             in             in
Default_Type:          d              d
Allowed_Types:         [d]            [d]
Vector:                yes            no
Vector_Bounds:         -              -
Null_Allowed:          yes            no

PORT_TABLE:
Port_Name:             out
Description:           "output"
Direction:             out
Default_Type:          d
Allowed_Types:         [d]
Vector:                yes
Vector_Bounds:         [1 -]
Null_Allowed:          no

PARAMETER_TABLE:
Parameter_Name:        process_file
Description:           "executable"
Data_Type:             string
Default_Value:         -
Limits:                -
Vector:                no
Vector_Bounds:         -
Null_Allowed:          no

PARAMETER_TABLE:
Parameter_Name:        delay
Description:           "delay"
Data_Type:             real
Default_Value:         1.0e-9
Limits:                [1e-12 -]
Vector:                no
Vector_Bounds:         -
Null_Allowed:          yes
"#,
        )
        .expect("ifspec parses");

        assert_eq!(spec.spice_model_name, "demo");
        assert_eq!(spec.ports.len(), 3);
        assert_eq!(spec.ports[2].vector_bounds.min_usize(), Some(1));
        assert_eq!(spec.parameters.len(), 2);
        assert!(spec.parameters[0].required());
        assert!(!spec.parameters[1].required());
        assert_eq!(spec.parameters[1].limits.min_real(), Some(1.0e-12));
    }
}
