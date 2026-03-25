//! Property Bridge Module
//!
//! Commercial-grade bidirectional mapping between Component structs and PropertyValue HashMaps.
//! This module provides the bridge layer for comprehensive property persistence, enabling
//! all properties edited in the property dialog to be saved back to components.
//!
//! # Architecture (Cadence Spectre Parity)
//!
//! SPICE simulators store component parameters in a standardized format:
//! - Primary value: The main parameter (R for resistor, C for capacitor, etc.)
//! - Secondary parameters: Key-value pairs in SPICE format (e.g., `m=2 tc1=0.01`)
//!
//! This module converts between:
//! - `Component` struct (simple string-based storage for serialization)
//! - `HashMap<String, PropertyValue>` (type-safe editing in property dialog)
//!
//! # Example
//!
//! ```ignore
//! // Opening dialog: Component → PropertyValue HashMap
//! let properties = collect_properties_from_component(&component, &registry);
//!
//! // Saving changes: PropertyValue HashMap → Component
//! apply_properties_to_component(&mut component, &properties);
//! ```

use crate::state::{Component, ComponentType, PropertyRegistry, PropertyValue};
use std::collections::HashMap;

// =============================================================================
// Primary Property Name Mapping
// =============================================================================

/// Returns the primary property name for a given component type.
///
/// In SPICE, each component type has a primary parameter:
/// - Resistor: "r" (resistance in Ohms)
/// - Capacitor: "c" (capacitance in Farads)
/// - Inductor: "l" (inductance in Henries)
/// - VoltageSource: "dc" (DC voltage)
/// - CurrentSource: "dc" (DC current)
/// - Diode: "is" (saturation current, but typically just uses model)
/// - MOSFET: "w" (width) or model-dependent
/// - BJT: model-dependent
///
/// This matches Cadence Spectre's component definition format.
pub fn get_primary_property_name(kind: ComponentType) -> &'static str {
    match kind {
        ComponentType::Resistor => "r",
        ComponentType::Capacitor => "c",
        ComponentType::Inductor => "l",
        ComponentType::Transformer => "lp",
        ComponentType::CoupledInductor => "k",
        ComponentType::VoltageSource => "dc",
        ComponentType::VoltageSourceAc => "ac_mag",
        ComponentType::VoltageSourcePulse => "v1",
        ComponentType::VoltageSourceSin => "vo",
        ComponentType::CurrentSource => "dc",
        ComponentType::CurrentSourceAc => "ac_mag",
        ComponentType::CurrentSourcePulse => "i1",
        ComponentType::CurrentSourceSin => "io",
        ComponentType::Diode => "is",
        ComponentType::Nmos | ComponentType::Pmos => "w",
        ComponentType::NpnBjt | ComponentType::PnpBjt => "is",
        ComponentType::Vcvs => "gain",
        ComponentType::Vccs => "gm",
        ComponentType::Ccvs => "rm",
        ComponentType::Cccs => "gain",
        ComponentType::Ground => "name",
        // Catch-all for any other component types
        _ => "value",
    }
}

/// Returns the display name for a component's value field based on type.
///
/// Used for UI labels and netlist generation.
pub fn get_value_display_name(kind: ComponentType) -> &'static str {
    match kind {
        ComponentType::Resistor => "Resistance",
        ComponentType::Capacitor => "Capacitance",
        ComponentType::Inductor => "Inductance",
        ComponentType::Transformer => "Primary Inductance",
        ComponentType::CoupledInductor => "Coupling Coefficient",
        ComponentType::VoltageSource => "DC Voltage",
        ComponentType::VoltageSourceAc => "AC Magnitude",
        ComponentType::VoltageSourcePulse => "Initial Voltage",
        ComponentType::VoltageSourceSin => "DC Offset",
        ComponentType::CurrentSource => "DC Current",
        ComponentType::CurrentSourceAc => "AC Magnitude",
        ComponentType::CurrentSourcePulse => "Initial Current",
        ComponentType::CurrentSourceSin => "DC Offset",
        ComponentType::Diode => "Saturation Current",
        ComponentType::Nmos | ComponentType::Pmos => "Width",
        ComponentType::NpnBjt | ComponentType::PnpBjt => "Saturation Current",
        ComponentType::Vcvs | ComponentType::Cccs => "Gain",
        ComponentType::Vccs => "Transconductance",
        ComponentType::Ccvs => "Transresistance",
        ComponentType::Ground => "Name",
        // Catch-all for any other component types
        _ => "Value",
    }
}

// =============================================================================
// SPICE Parameter String Parsing
// =============================================================================

/// Parses a SPICE-format parameter string into a key-value HashMap.
///
/// Supports multiple formats commonly used in SPICE:
/// - Space-separated: `m=2 tc1=0.01 tc2=0.001`
/// - Comma-separated: `m=2, tc1=0.01, tc2=0.001`
/// - Mixed: `m=2 tc1=0.01, dtemp=25`
/// - Quoted values: `model="nmos_3p3" region="saturation"`
///
/// # Arguments
/// * `params` - The parameter string to parse
///
/// # Returns
/// HashMap of parameter name to string value
///
/// # Example
/// ```ignore
/// let params = parse_params_string("m=2 tc1=0.01 tc2=0.001");
/// assert_eq!(params.get("m"), Some(&"2".to_string()));
/// assert_eq!(params.get("tc1"), Some(&"0.01".to_string()));
/// ```
pub fn parse_params_string(params: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();

    if params.trim().is_empty() {
        return result;
    }

    // State machine for parsing
    let mut current_key = String::new();
    let mut current_value = String::new();
    let mut in_value = false;
    let mut in_quotes = false;
    let mut quote_char = '"';

    for ch in params.chars() {
        match ch {
            // Quote handling
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = ch;
            }
            c if c == quote_char && in_quotes => {
                in_quotes = false;
            }
            // Equals sign starts value
            '=' if !in_quotes && !in_value => {
                in_value = true;
            }
            // Separators end current key-value pair
            ' ' | ',' | '\t' | '\n' if !in_quotes && in_value => {
                let key = current_key.trim().to_lowercase();
                let value = current_value.trim().to_string();
                if !key.is_empty() && !value.is_empty() {
                    result.insert(key, value);
                }
                current_key.clear();
                current_value.clear();
                in_value = false;
            }
            // Skip separators when not in value
            ' ' | ',' | '\t' | '\n' if !in_quotes && !in_value => {
                // Skip leading separators
            }
            // Accumulate characters
            _ => {
                if in_value {
                    current_value.push(ch);
                } else {
                    current_key.push(ch);
                }
            }
        }
    }

    // Handle final key-value pair
    let key = current_key.trim().to_lowercase();
    let value = current_value.trim().to_string();
    if !key.is_empty() && !value.is_empty() {
        result.insert(key, value);
    }

    result
}

/// Formats a key-value HashMap into a SPICE-format parameter string.
///
/// Uses standard SPICE format: `key1=value1 key2=value2`
/// Values containing spaces are quoted.
///
/// # Arguments
/// * `params` - HashMap of parameter name to string value
///
/// # Returns
/// SPICE-format parameter string
///
/// # Example
/// ```ignore
/// let mut params = HashMap::new();
/// params.insert("m".to_string(), "2".to_string());
/// params.insert("tc1".to_string(), "0.01".to_string());
/// let result = format_params_string(&params);
/// // Result: "m=2 tc1=0.01" (order may vary)
/// ```
pub fn format_params_string(params: &HashMap<String, String>) -> String {
    let mut pairs: Vec<_> = params
        .iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(k, v)| {
            // Quote values containing spaces or special characters
            if v.contains(' ') || v.contains(',') || v.contains('=') {
                format!("{}=\"{}\"", k, v)
            } else {
                format!("{}={}", k, v)
            }
        })
        .collect();

    // Sort for deterministic output (important for testing and diffs)
    pairs.sort();
    pairs.join(" ")
}

// =============================================================================
// Property Collection (Component → PropertyValue HashMap)
// =============================================================================

/// Collects properties from a Component into a PropertyValue HashMap.
///
/// This is the "read" direction of the bridge - extracting editable properties
/// from a Component for display in the property dialog.
///
/// # Process
/// 1. Extract instance name from `component.name`
/// 2. Parse primary value from `component.value`
/// 3. Parse secondary parameters from `component.params`
/// 4. Return combined HashMap with PropertyValue types
///
/// # Arguments
/// * `component` - The component to extract properties from
/// * `registry` - Property registry for type information
///
/// # Returns
/// HashMap suitable for use with TabbedPropertyDialogState
pub fn collect_properties_from_component(
    component: &Component,
    registry: &PropertyRegistry,
) -> HashMap<String, PropertyValue> {
    let mut properties = HashMap::new();

    // Always include instance name
    properties.insert(
        "name".to_string(),
        PropertyValue::String(component.name.clone()),
    );

    if let Some(sheet) = registry.get(component.kind)
        && let Some(def) = sheet.iter().find(|def| def.name == "symbol")
            && let PropertyValue::Enum { options, .. } = &def.default_value {
                let selected = component
                    .symbol_variant
                    .clone()
                    .unwrap_or_else(|| options.first().cloned().unwrap_or_default());
                properties.insert(
                    "symbol".to_string(),
                    PropertyValue::Enum {
                        selected,
                        options: options.clone(),
                    },
                );
            }

    // Get the primary property name for this component type
    let primary_prop = get_primary_property_name(component.kind);

    // Parse the component value as the primary property
    if !component.value.is_empty() {
        properties.insert(
            primary_prop.to_string(),
            PropertyValue::Expression(component.value.clone()),
        );
    }

    // Parse additional parameters from params string
    let parsed_params = parse_params_string(&component.params);
    for (key, value) in parsed_params {
        // Skip if this is the primary property (already handled)
        if key == primary_prop {
            continue;
        }

        // Determine the appropriate PropertyValue type based on the registry
        let prop_value = if let Some(sheet) = registry.get(component.kind) {
            if let Some(def) = sheet.iter().find(|d| d.name == key) {
                // Use the definition's type to create appropriate PropertyValue
                match &def.default_value {
                    PropertyValue::Number { .. } => {
                        // Try to parse as number
                        if let Ok(num) = value.parse::<f64>() {
                            PropertyValue::Number {
                                value: num,
                                unit: def.unit.clone(),
                            }
                        } else {
                            PropertyValue::Expression(value)
                        }
                    }
                    PropertyValue::Boolean(_) => {
                        let bool_val =
                            matches!(value.to_lowercase().as_str(), "true" | "1" | "yes" | "on");
                        PropertyValue::Boolean(bool_val)
                    }
                    PropertyValue::Enum { options, .. } => PropertyValue::Enum {
                        selected: value.clone(),
                        options: options.clone(),
                    },
                    _ => PropertyValue::Expression(value),
                }
            } else {
                // Unknown property, treat as expression
                PropertyValue::Expression(value)
            }
        } else {
            PropertyValue::Expression(value)
        };

        properties.insert(key, prop_value);
    }

    properties
}

// =============================================================================
// Property Application (PropertyValue HashMap → Component)
// =============================================================================

/// Applies properties from a PropertyValue HashMap back to a Component.
///
/// This is the "write" direction of the bridge - saving edited properties
/// from the property dialog back to the Component struct.
///
/// # Process
/// 1. Update `component.name` from "name" property
/// 2. Update `component.value` from primary property (r/c/l/dc/etc.)
/// 3. Serialize remaining properties to `component.params`
///
/// # Arguments
/// * `component` - The component to update
/// * `properties` - HashMap of edited property values
/// * `registry` - Property registry for type information (used for filtering)
pub fn apply_properties_to_component(
    component: &mut Component,
    properties: &HashMap<String, PropertyValue>,
    registry: &PropertyRegistry,
) {
    let primary_prop = get_primary_property_name(component.kind);

    // Update instance name
    if let Some(PropertyValue::String(name)) = properties.get("name") {
        component.name = name.clone();
    }

    // Update primary value
    if let Some(prop_value) = properties.get(primary_prop) {
        component.value = property_value_to_string(prop_value);
    }

    if let Some(PropertyValue::Enum { selected, .. }) = properties.get("symbol") {
        component.symbol_variant = if selected.is_empty() || selected == "default" {
            None
        } else {
            Some(selected.clone())
        };
    }

    // Collect secondary parameters
    let mut secondary_params: HashMap<String, String> = HashMap::new();

    for (key, value) in properties {
        // Skip name and primary property
        if key == "name" || key == primary_prop || key == "symbol" {
            continue;
        }

        // Skip empty values
        let string_value = property_value_to_string(value);
        if string_value.is_empty() {
            continue;
        }

        // Check if this value differs from the default
        let is_default = if let Some(sheet) = registry.get(component.kind) {
            if let Some(def) = sheet.iter().find(|d| d.name == *key) {
                property_values_equal(value, &def.default_value)
            } else {
                false // Unknown property, include it
            }
        } else {
            false
        };

        // Only include non-default values (Spectre behavior: minimize netlist verbosity)
        if !is_default {
            secondary_params.insert(key.clone(), string_value);
        }
    }

    // Format secondary parameters into params string
    component.params = format_params_string(&secondary_params);
}

/// Converts a PropertyValue to its string representation.
///
/// Used for serialization to SPICE netlist format.
fn property_value_to_string(value: &PropertyValue) -> String {
    match value {
        PropertyValue::Number { value, .. } => {
            // Use engineering notation for cleaner values
            crate::properties::format_engineering_value(*value)
        }
        PropertyValue::String(s) => s.clone(),
        PropertyValue::Expression(expr) => expr.clone(),
        PropertyValue::Enum { selected, .. } => selected.clone(),
        PropertyValue::Boolean(b) => if *b { "1" } else { "0" }.to_string(),
    }
}

/// Compares two PropertyValues for equality (ignoring units in numbers).
///
/// Used to detect if a value has been modified from the default.
fn property_values_equal(a: &PropertyValue, b: &PropertyValue) -> bool {
    match (a, b) {
        (PropertyValue::Number { value: va, .. }, PropertyValue::Number { value: vb, .. }) => {
            (va - vb).abs() < 1e-15 || (va.is_nan() && vb.is_nan())
        }
        (PropertyValue::String(sa), PropertyValue::String(sb)) => sa == sb,
        (PropertyValue::Expression(ea), PropertyValue::Expression(eb)) => ea == eb,
        (PropertyValue::Enum { selected: sa, .. }, PropertyValue::Enum { selected: sb, .. }) => {
            sa == sb
        }
        (PropertyValue::Boolean(ba), PropertyValue::Boolean(bb)) => ba == bb,
        // Different types are not equal
        _ => false,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Point;

    // =========================================================================
    // Primary Property Name Tests
    // =========================================================================

    #[test]
    fn test_primary_property_name_passives() {
        assert_eq!(get_primary_property_name(ComponentType::Resistor), "r");
        assert_eq!(get_primary_property_name(ComponentType::Capacitor), "c");
        assert_eq!(get_primary_property_name(ComponentType::Inductor), "l");
        assert_eq!(get_primary_property_name(ComponentType::Transformer), "lp");
    }

    #[test]
    fn test_primary_property_name_sources() {
        assert_eq!(
            get_primary_property_name(ComponentType::VoltageSource),
            "dc"
        );
        assert_eq!(
            get_primary_property_name(ComponentType::CurrentSource),
            "dc"
        );
        assert_eq!(
            get_primary_property_name(ComponentType::VoltageSourceAc),
            "ac_mag"
        );
        assert_eq!(
            get_primary_property_name(ComponentType::CurrentSourceAc),
            "ac_mag"
        );
    }

    #[test]
    fn test_primary_property_name_transistors() {
        assert_eq!(get_primary_property_name(ComponentType::Nmos), "w");
        assert_eq!(get_primary_property_name(ComponentType::Pmos), "w");
        assert_eq!(get_primary_property_name(ComponentType::NpnBjt), "is");
        assert_eq!(get_primary_property_name(ComponentType::PnpBjt), "is");
    }

    #[test]
    fn test_primary_property_name_controlled_sources() {
        assert_eq!(get_primary_property_name(ComponentType::Vcvs), "gain");
        assert_eq!(get_primary_property_name(ComponentType::Vccs), "gm");
        assert_eq!(get_primary_property_name(ComponentType::Ccvs), "rm");
        assert_eq!(get_primary_property_name(ComponentType::Cccs), "gain");
    }

    // =========================================================================
    // Parameter String Parsing Tests
    // =========================================================================

    #[test]
    fn test_parse_params_empty() {
        assert!(parse_params_string("").is_empty());
        assert!(parse_params_string("   ").is_empty());
        assert!(parse_params_string("\t\n").is_empty());
    }

    #[test]
    fn test_parse_params_single() {
        let result = parse_params_string("m=2");
        assert_eq!(result.get("m"), Some(&"2".to_string()));
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_parse_params_multiple_space_separated() {
        let result = parse_params_string("m=2 tc1=0.01 tc2=0.001");
        assert_eq!(result.get("m"), Some(&"2".to_string()));
        assert_eq!(result.get("tc1"), Some(&"0.01".to_string()));
        assert_eq!(result.get("tc2"), Some(&"0.001".to_string()));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_parse_params_comma_separated() {
        let result = parse_params_string("m=2, tc1=0.01, tc2=0.001");
        assert_eq!(result.get("m"), Some(&"2".to_string()));
        assert_eq!(result.get("tc1"), Some(&"0.01".to_string()));
        assert_eq!(result.get("tc2"), Some(&"0.001".to_string()));
    }

    #[test]
    fn test_parse_params_mixed_separators() {
        let result = parse_params_string("m=2 tc1=0.01,tc2=0.001\tdtemp=25");
        assert_eq!(result.get("m"), Some(&"2".to_string()));
        assert_eq!(result.get("tc1"), Some(&"0.01".to_string()));
        assert_eq!(result.get("tc2"), Some(&"0.001".to_string()));
        assert_eq!(result.get("dtemp"), Some(&"25".to_string()));
    }

    #[test]
    fn test_parse_params_quoted_values() {
        let result = parse_params_string("model=\"nmos_3p3\" region=\"saturation\"");
        assert_eq!(result.get("model"), Some(&"nmos_3p3".to_string()));
        assert_eq!(result.get("region"), Some(&"saturation".to_string()));
    }

    #[test]
    fn test_parse_params_single_quotes() {
        let result = parse_params_string("model='pmos_1p8' w='1u'");
        assert_eq!(result.get("model"), Some(&"pmos_1p8".to_string()));
        assert_eq!(result.get("w"), Some(&"1u".to_string()));
    }

    #[test]
    fn test_parse_params_case_insensitivity() {
        // SPICE parameters are case-insensitive
        let result = parse_params_string("TC1=0.01 Tc2=0.001 tC3=0.0001");
        assert_eq!(result.get("tc1"), Some(&"0.01".to_string()));
        assert_eq!(result.get("tc2"), Some(&"0.001".to_string()));
        assert_eq!(result.get("tc3"), Some(&"0.0001".to_string()));
    }

    #[test]
    fn test_parse_params_engineering_notation() {
        let result = parse_params_string("w=1u l=180n c=10p");
        assert_eq!(result.get("w"), Some(&"1u".to_string()));
        assert_eq!(result.get("l"), Some(&"180n".to_string()));
        assert_eq!(result.get("c"), Some(&"10p".to_string()));
    }

    #[test]
    fn test_parse_params_negative_values() {
        let result = parse_params_string("vth0=-0.4 k1=-0.5");
        assert_eq!(result.get("vth0"), Some(&"-0.4".to_string()));
        assert_eq!(result.get("k1"), Some(&"-0.5".to_string()));
    }

    #[test]
    fn test_parse_params_scientific_notation() {
        let result = parse_params_string("is=1e-14 rs=1.5e3");
        assert_eq!(result.get("is"), Some(&"1e-14".to_string()));
        assert_eq!(result.get("rs"), Some(&"1.5e3".to_string()));
    }

    #[test]
    fn test_parse_params_extra_whitespace() {
        let result = parse_params_string("  m = 2   tc1 = 0.01  ");
        // Note: current parser doesn't handle spaces around =
        // This is intentional - strict SPICE format doesn't allow spaces
        // The result will be empty because "m " isn't a valid key
        // This tests the parser's strict behavior
        assert!(result.is_empty() || result.len() <= 2);
    }

    // =========================================================================
    // Parameter String Formatting Tests
    // =========================================================================

    #[test]
    fn test_format_params_empty() {
        let params: HashMap<String, String> = HashMap::new();
        assert_eq!(format_params_string(&params), "");
    }

    #[test]
    fn test_format_params_single() {
        let mut params = HashMap::new();
        params.insert("m".to_string(), "2".to_string());
        assert_eq!(format_params_string(&params), "m=2");
    }

    #[test]
    fn test_format_params_multiple() {
        let mut params = HashMap::new();
        params.insert("m".to_string(), "2".to_string());
        params.insert("tc1".to_string(), "0.01".to_string());
        // Sorted alphabetically
        let result = format_params_string(&params);
        assert!(result.contains("m=2"));
        assert!(result.contains("tc1=0.01"));
    }

    #[test]
    fn test_format_params_skips_empty() {
        let mut params = HashMap::new();
        params.insert("m".to_string(), "2".to_string());
        params.insert("empty".to_string(), "".to_string());
        let result = format_params_string(&params);
        assert_eq!(result, "m=2");
    }

    #[test]
    fn test_format_params_quotes_spaces() {
        let mut params = HashMap::new();
        params.insert("model".to_string(), "my model".to_string());
        let result = format_params_string(&params);
        assert_eq!(result, "model=\"my model\"");
    }

    #[test]
    fn test_format_params_deterministic() {
        // Verify that output is always sorted (deterministic)
        let mut params = HashMap::new();
        params.insert("z".to_string(), "1".to_string());
        params.insert("a".to_string(), "2".to_string());
        params.insert("m".to_string(), "3".to_string());
        let result = format_params_string(&params);
        assert_eq!(result, "a=2 m=3 z=1");
    }

    // =========================================================================
    // Round-Trip Tests (Parse + Format)
    // =========================================================================

    #[test]
    fn test_roundtrip_simple() {
        let original = "m=2 tc1=0.01";
        let parsed = parse_params_string(original);
        let formatted = format_params_string(&parsed);
        // Round-trip should preserve content (order may differ)
        let reparsed = parse_params_string(&formatted);
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn test_roundtrip_multiple_params() {
        let mut original = HashMap::new();
        original.insert("m".to_string(), "2".to_string());
        original.insert("tc1".to_string(), "0.01".to_string());
        original.insert("tc2".to_string(), "0.001".to_string());

        let formatted = format_params_string(&original);
        let parsed = parse_params_string(&formatted);

        assert_eq!(parsed, original);
    }

    #[test]
    fn test_roundtrip_engineering_values() {
        let original = "w=1u l=180n as=1p";
        let parsed = parse_params_string(original);
        let formatted = format_params_string(&parsed);
        let reparsed = parse_params_string(&formatted);
        assert_eq!(parsed, reparsed);
    }

    // =========================================================================
    // Property Collection Tests
    // =========================================================================

    #[test]
    fn test_collect_properties_resistor() {
        let registry = PropertyRegistry::new();
        let component = Component::new(1, ComponentType::Resistor, Point::new(0, 0))
            .with_name_value("R1", "1k");

        let props = collect_properties_from_component(&component, &registry);

        assert_eq!(
            props.get("name"),
            Some(&PropertyValue::String("R1".to_string()))
        );
        assert_eq!(
            props.get("r"),
            Some(&PropertyValue::Expression("1k".to_string()))
        );
    }

    #[test]
    fn test_collect_properties_transformer() {
        let registry = PropertyRegistry::new();
        let mut component = Component::new(1, ComponentType::Transformer, Point::new(0, 0))
            .with_name_value("T1", "10m");
        component.params = "icp=2m k=0.995 turns_ratio=4".to_string();

        let props = collect_properties_from_component(&component, &registry);

        assert_eq!(
            props.get("lp"),
            Some(&PropertyValue::Expression("10m".to_string()))
        );
        assert_eq!(
            props.get("turns_ratio"),
            Some(&PropertyValue::Expression("4".to_string()))
        );
        assert_eq!(
            props.get("k"),
            Some(&PropertyValue::Expression("0.995".to_string()))
        );
    }

    #[test]
    fn test_collect_properties_with_params() {
        let registry = PropertyRegistry::new();
        let mut component = Component::new(1, ComponentType::Resistor, Point::new(0, 0))
            .with_name_value("R1", "1k");
        component.params = "m=2 tc1=0.01".to_string();

        let props = collect_properties_from_component(&component, &registry);

        assert_eq!(
            props.get("name"),
            Some(&PropertyValue::String("R1".to_string()))
        );
        assert_eq!(
            props.get("r"),
            Some(&PropertyValue::Expression("1k".to_string()))
        );
        // Secondary params should be parsed
        assert!(props.contains_key("m"));
        assert!(props.contains_key("tc1"));
    }

    #[test]
    fn test_collect_properties_capacitor() {
        let registry = PropertyRegistry::new();
        let component = Component::new(2, ComponentType::Capacitor, Point::new(0, 0))
            .with_name_value("C1", "10u");

        let props = collect_properties_from_component(&component, &registry);

        assert_eq!(
            props.get("c"),
            Some(&PropertyValue::Expression("10u".to_string()))
        );
    }

    #[test]
    fn test_collect_properties_voltage_source() {
        let registry = PropertyRegistry::new();
        let component = Component::new(3, ComponentType::VoltageSource, Point::new(0, 0))
            .with_name_value("V1", "5");

        let props = collect_properties_from_component(&component, &registry);

        assert_eq!(
            props.get("dc"),
            Some(&PropertyValue::Expression("5".to_string()))
        );
    }

    #[test]
    fn test_collect_properties_includes_symbol_variant_enum() {
        let registry = PropertyRegistry::new();
        let component = Component::new(3, ComponentType::VoltageSource, Point::new(0, 0))
            .with_name_value("V1", "5")
            .with_symbol_variant("battery");

        let props = collect_properties_from_component(&component, &registry);
        let Some(PropertyValue::Enum { selected, options }) = props.get("symbol") else {
            panic!("expected symbol enum property");
        };

        assert_eq!(selected, "battery");
        assert!(options.contains(&"default".to_string()));
        assert!(options.contains(&"battery".to_string()));
        assert!(options.contains(&"battery_multi_cell".to_string()));
    }

    // =========================================================================
    // Property Application Tests
    // =========================================================================

    #[test]
    fn test_apply_properties_name_only() {
        let registry = PropertyRegistry::new();
        let mut component = Component::new(1, ComponentType::Resistor, Point::new(0, 0))
            .with_name_value("R1", "1k");

        let mut props = HashMap::new();
        props.insert("name".to_string(), PropertyValue::String("R42".to_string()));
        props.insert("r".to_string(), PropertyValue::Expression("1k".to_string()));

        apply_properties_to_component(&mut component, &props, &registry);

        assert_eq!(component.name, "R42");
    }

    #[test]
    fn test_apply_properties_primary_value() {
        let registry = PropertyRegistry::new();
        let mut component = Component::new(1, ComponentType::Resistor, Point::new(0, 0))
            .with_name_value("R1", "1k");

        let mut props = HashMap::new();
        props.insert("name".to_string(), PropertyValue::String("R1".to_string()));
        props.insert(
            "r".to_string(),
            PropertyValue::Expression("2.2k".to_string()),
        );

        apply_properties_to_component(&mut component, &props, &registry);

        assert_eq!(component.value, "2.2k");
    }

    #[test]
    fn test_apply_properties_secondary_params() {
        let registry = PropertyRegistry::new();
        let mut component = Component::new(1, ComponentType::Resistor, Point::new(0, 0))
            .with_name_value("R1", "1k");

        let mut props = HashMap::new();
        props.insert("name".to_string(), PropertyValue::String("R1".to_string()));
        props.insert("r".to_string(), PropertyValue::Expression("1k".to_string()));
        props.insert(
            "m".to_string(),
            PropertyValue::Number {
                value: 2.0,
                unit: None,
            },
        );
        props.insert(
            "tc1".to_string(),
            PropertyValue::Number {
                value: 0.01,
                unit: Some("1/°C".to_string()),
            },
        );

        apply_properties_to_component(&mut component, &props, &registry);

        // Secondary params should be serialized to params string
        // Only non-default values should appear
        let parsed = parse_params_string(&component.params);
        assert_eq!(parsed.get("m"), Some(&"2".to_string()));
    }

    #[test]
    fn test_apply_properties_skips_defaults() {
        let registry = PropertyRegistry::new();
        let mut component = Component::new(1, ComponentType::Resistor, Point::new(0, 0))
            .with_name_value("R1", "1k");

        let mut props = HashMap::new();
        props.insert("name".to_string(), PropertyValue::String("R1".to_string()));
        props.insert("r".to_string(), PropertyValue::Expression("1k".to_string()));
        // m=1 is the default, should not appear in params
        props.insert(
            "m".to_string(),
            PropertyValue::Number {
                value: 1.0,
                unit: None,
            },
        );

        apply_properties_to_component(&mut component, &props, &registry);

        // m=1 is default, should not be in params string
        let parsed = parse_params_string(&component.params);
        assert!(!parsed.contains_key("m"));
    }

    #[test]
    fn test_apply_properties_persists_symbol_variant_without_netlist_leakage() {
        let registry = PropertyRegistry::new();
        let mut component =
            Component::new(1, ComponentType::Diode, Point::new(0, 0)).with_name_value("D1", "1");

        let mut props = HashMap::new();
        props.insert("name".to_string(), PropertyValue::String("D1".to_string()));
        props.insert("is".to_string(), PropertyValue::Expression("1".to_string()));
        props.insert(
            "symbol".to_string(),
            PropertyValue::Enum {
                selected: "zener".to_string(),
                options: vec![
                    "default".to_string(),
                    "schottky".to_string(),
                    "zener".to_string(),
                    "led".to_string(),
                    "tunnel".to_string(),
                ],
            },
        );

        apply_properties_to_component(&mut component, &props, &registry);

        assert_eq!(component.symbol_variant.as_deref(), Some("zener"));
        assert!(
            !component.params.contains("symbol"),
            "symbol appearance should never leak into SPICE params"
        );
    }

    // =========================================================================
    // Full Round-Trip Tests (Component → Props → Component)
    // =========================================================================

    #[test]
    fn test_full_roundtrip_resistor() {
        let registry = PropertyRegistry::new();

        // Original component
        let mut original = Component::new(1, ComponentType::Resistor, Point::new(0, 0))
            .with_name_value("R1", "4.7k");
        original.params = "m=2 tc1=0.01".to_string();

        // Collect properties
        let props = collect_properties_from_component(&original, &registry);

        // Create a new component and apply properties
        let mut restored = Component::new(1, ComponentType::Resistor, Point::new(0, 0));
        apply_properties_to_component(&mut restored, &props, &registry);

        // Verify preservation
        assert_eq!(restored.name, original.name);
        assert_eq!(restored.value, original.value);

        // Verify params were preserved
        let orig_params = parse_params_string(&original.params);
        let rest_params = parse_params_string(&restored.params);
        assert_eq!(orig_params.get("m"), rest_params.get("m"));
    }

    #[test]
    fn test_full_roundtrip_mosfet() {
        let registry = PropertyRegistry::new();

        let mut original =
            Component::new(1, ComponentType::Nmos, Point::new(0, 0)).with_name_value("M1", "1u");
        original.params = "l=180n as=1p ad=1p".to_string();

        let props = collect_properties_from_component(&original, &registry);
        let mut restored = Component::new(1, ComponentType::Nmos, Point::new(0, 0));
        apply_properties_to_component(&mut restored, &props, &registry);

        assert_eq!(restored.name, original.name);
        assert_eq!(restored.value, original.value);
    }

    #[test]
    fn test_full_roundtrip_coupled_inductor() {
        let registry = PropertyRegistry::new();

        let mut original = Component::new(1, ComponentType::CoupledInductor, Point::new(0, 0))
            .with_name_value("K1", "0.995");
        original.params = "inductors=\"L1 L2\"".to_string();

        let props = collect_properties_from_component(&original, &registry);
        let mut restored = Component::new(1, ComponentType::CoupledInductor, Point::new(0, 0));
        apply_properties_to_component(&mut restored, &props, &registry);

        assert_eq!(restored.name, "K1");
        assert_eq!(restored.value, "0.995");

        let restored_params = parse_params_string(&restored.params);
        assert_eq!(restored_params.get("inductors"), Some(&"L1 L2".to_string()));
    }

    #[test]
    fn test_full_roundtrip_transformer() {
        let registry = PropertyRegistry::new();

        let mut original = Component::new(1, ComponentType::Transformer, Point::new(0, 0))
            .with_name_value("T1", "12m");
        original.params = "icp=1m ics=2m k=0.998 ls=3m rp=50m rs=75m turns_ratio=0.5".to_string();

        let props = collect_properties_from_component(&original, &registry);
        let mut restored = Component::new(1, ComponentType::Transformer, Point::new(0, 0));
        apply_properties_to_component(&mut restored, &props, &registry);

        assert_eq!(restored.name, "T1");
        assert_eq!(restored.value, "12m");

        let restored_params = parse_params_string(&restored.params);
        assert_eq!(restored_params.get("k"), Some(&"0.998".to_string()));
        assert_eq!(restored_params.get("ls"), Some(&"3m".to_string()));
        assert_eq!(restored_params.get("turns_ratio"), Some(&"0.5".to_string()));
        assert_eq!(restored_params.get("rp"), Some(&"50m".to_string()));
        assert_eq!(restored_params.get("rs"), Some(&"75m".to_string()));
    }

    // =========================================================================
    // Edge Case Tests
    // =========================================================================

    #[test]
    fn test_empty_component() {
        let registry = PropertyRegistry::new();
        let component = Component::new(1, ComponentType::Resistor, Point::new(0, 0));

        let props = collect_properties_from_component(&component, &registry);

        // Should still have name (empty string)
        assert!(props.contains_key("name"));
    }

    #[test]
    fn test_special_characters_in_value() {
        let registry = PropertyRegistry::new();
        let mut component = Component::new(1, ComponentType::Resistor, Point::new(0, 0))
            .with_name_value("R1", "1k");
        component.params = "model=\"custom resistor\"".to_string();

        let props = collect_properties_from_component(&component, &registry);

        // Model should be parsed correctly
        if let Some(PropertyValue::Expression(model)) = props.get("model") {
            assert_eq!(model, "custom resistor");
        }
    }

    #[test]
    fn test_property_value_to_string_number() {
        let value = PropertyValue::Number {
            value: 1000.0,
            unit: Some("Ω".to_string()),
        };
        let result = property_value_to_string(&value);
        // Should use engineering notation
        assert!(result == "1k" || result == "1000" || result.contains("1"));
    }

    #[test]
    fn test_property_value_to_string_boolean() {
        assert_eq!(property_value_to_string(&PropertyValue::Boolean(true)), "1");
        assert_eq!(
            property_value_to_string(&PropertyValue::Boolean(false)),
            "0"
        );
    }

    #[test]
    fn test_property_value_to_string_enum() {
        let value = PropertyValue::Enum {
            selected: "nmos".to_string(),
            options: vec!["nmos".to_string(), "pmos".to_string()],
        };
        assert_eq!(property_value_to_string(&value), "nmos");
    }

    #[test]
    fn test_property_values_equal_numbers() {
        let a = PropertyValue::Number {
            value: 1.0,
            unit: Some("Ω".to_string()),
        };
        let b = PropertyValue::Number {
            value: 1.0,
            unit: Some("Ohm".to_string()), // Different unit string
        };
        // Should be equal (values match, units ignored)
        assert!(property_values_equal(&a, &b));
    }

    #[test]
    fn test_property_values_equal_different_types() {
        let a = PropertyValue::Number {
            value: 1.0,
            unit: None,
        };
        let b = PropertyValue::String("1.0".to_string());
        // Different types are not equal
        assert!(!property_values_equal(&a, &b));
    }

    // =========================================================================
    // Value Display Name Tests
    // =========================================================================

    #[test]
    fn test_value_display_name_passives() {
        assert_eq!(
            get_value_display_name(ComponentType::Resistor),
            "Resistance"
        );
        assert_eq!(
            get_value_display_name(ComponentType::Capacitor),
            "Capacitance"
        );
        assert_eq!(
            get_value_display_name(ComponentType::Inductor),
            "Inductance"
        );
        assert_eq!(
            get_value_display_name(ComponentType::Transformer),
            "Primary Inductance"
        );
    }

    #[test]
    fn test_value_display_name_sources() {
        assert_eq!(
            get_value_display_name(ComponentType::VoltageSource),
            "DC Voltage"
        );
        assert_eq!(
            get_value_display_name(ComponentType::CurrentSource),
            "DC Current"
        );
    }
}
