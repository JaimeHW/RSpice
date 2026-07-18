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
        ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl => "pwl_data",
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
        ComponentType::Port => "Port name",
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
        && let PropertyValue::Enum { options, .. } = &def.default_value
    {
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

    // Parse the component value as the primary property. A newly placed PWL
    // source has no authored value yet, so seed its transaction from the CDF
    // default at this bridge boundary. Once inside a transaction, an explicit
    // empty string remains authored input and is rejected by the PWL editor.
    if component.value.is_empty() && component.kind.is_pwl_source() {
        if let Some(default) = registry
            .get(component.kind)
            .and_then(|sheet| sheet.iter().find(|def| def.name == primary_prop))
            .map(|def| def.default_value.clone())
        {
            properties.insert(primary_prop.to_owned(), default);
        }
    } else if !component.value.is_empty() {
        let value = if component.kind.is_pwl_source() {
            PropertyValue::String(component.value.clone())
        } else {
            PropertyValue::Expression(component.value.clone())
        };
        properties.insert(primary_prop.to_string(), value);
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
                        // Engineering notation ("1k", "10n") counts as numeric.
                        if let Ok(num) = crate::properties::parse_engineering_value(&value) {
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
        // Rust's finite-f64 display is the shortest decimal that round-trips
        // to the same binary value. This boundary feeds the durable SPICE
        // model, so presentation-oriented engineering formatting (which may
        // round) must never be used here.
        PropertyValue::Number { value, .. } => value.to_string(),
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
            va == vb || (va.is_nan() && vb.is_nan())
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

    #[test]
    fn numeric_serialization_is_shortest_and_round_trips_exactly() {
        let values = [
            1.234_567_890_123_456_7,
            1.234_567_890_123_456_7e-27,
            f64::from_bits(1),
            f64::MAX,
        ];

        for value in values {
            let serialized = property_value_to_string(&PropertyValue::number(value));
            let parsed = serialized
                .parse::<f64>()
                .unwrap_or_else(|error| panic!("{serialized:?} is not a decimal: {error}"));
            assert_eq!(
                parsed.to_bits(),
                value.to_bits(),
                "serialization changed {value:?} to {parsed:?}"
            );
        }
    }

    #[test]
    fn component_bridge_preserves_high_precision_primary_values() {
        let registry = PropertyRegistry::new();
        let value = 1.234_567_890_123_456_7e-6;
        let mut component = Component::new(1, ComponentType::Resistor, Point::origin());
        let properties = HashMap::from([("r".to_owned(), PropertyValue::number(value))]);

        apply_properties_to_component(&mut component, &properties, &registry);

        assert_eq!(
            component
                .value
                .parse::<f64>()
                .expect("serialized resistance"),
            value
        );
    }

    #[test]
    fn component_bridge_does_not_drop_a_small_nonzero_default_delta() {
        let registry = PropertyRegistry::new();
        let value = 5.0e-16;
        let mut component = Component::new(1, ComponentType::Resistor, Point::origin());
        let properties = HashMap::from([("tc1".to_owned(), PropertyValue::number(value))]);

        assert!(!property_values_equal(
            &PropertyValue::number(value),
            &PropertyValue::number(0.0)
        ));
        apply_properties_to_component(&mut component, &properties, &registry);

        let serialized = parse_params_string(&component.params);
        let tc1 = serialized.get("tc1").expect("non-default tc1 is retained");
        assert_eq!(tc1.parse::<f64>().expect("serialized tc1"), value);
    }

    #[test]
    fn blank_optional_source_defaults_are_omitted_from_durable_parameters() {
        let registry = PropertyRegistry::new();
        let mut component = Component::new(1, ComponentType::VoltageSource, Point::origin());
        let properties = HashMap::from([
            (
                "pacdbm".to_owned(),
                PropertyValue::Expression(String::new()),
            ),
            ("rp".to_owned(), PropertyValue::Expression(String::new())),
        ]);

        apply_properties_to_component(&mut component, &properties, &registry);

        assert!(component.params.is_empty());
        assert!(!component.params.contains("inf"));
    }
}

#[cfg(test)]
mod pwl_tests {
    use super::*;
    use crate::state::Point;

    #[test]
    fn new_pwl_sources_seed_the_registry_waveform_default() {
        let registry = PropertyRegistry::new();

        for (kind, expected) in [
            (ComponentType::VoltageSourcePwl, "0 0 1u 1 2u 0"),
            (ComponentType::CurrentSourcePwl, "0 0 1u 1m 2u 0"),
        ] {
            let component = Component::new(1, kind, Point::origin());
            let properties = collect_properties_from_component(&component, &registry);

            assert_eq!(
                properties.get("pwl_data"),
                Some(&PropertyValue::String(expected.to_owned()))
            );
        }
    }

    #[test]
    fn authored_pwl_source_uses_the_schema_string_type() {
        let registry = PropertyRegistry::new();
        let component = Component::new(1, ComponentType::VoltageSourcePwl, Point::origin())
            .with_name_value("V1", "0 0 2n 1");

        let properties = collect_properties_from_component(&component, &registry);

        assert_eq!(
            properties.get("pwl_data"),
            Some(&PropertyValue::String("0 0 2n 1".to_owned()))
        );
    }
}
