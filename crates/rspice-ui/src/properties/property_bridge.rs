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
use crate::state::{format_params_string, parse_params_string};
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
        ComponentType::VoltageSourceAc => "ac",
        ComponentType::VoltageSourcePulse => "v1",
        ComponentType::VoltageSourceSin => "vo",
        ComponentType::VoltageSourceExp => "v1",
        ComponentType::VoltageSourceSffm => "vo",
        ComponentType::CurrentSource => "dc",
        ComponentType::CurrentSourceAc => "ac",
        ComponentType::CurrentSourcePulse => "i1",
        ComponentType::CurrentSourceSin => "io",
        ComponentType::CurrentSourceExp => "i1",
        ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl => "pwl_data",
        ComponentType::Diode
        | ComponentType::Nmos
        | ComponentType::Pmos
        | ComponentType::NVdmos
        | ComponentType::PVdmos
        | ComponentType::NmosSoi
        | ComponentType::PmosSoi
        | ComponentType::NpnBjt
        | ComponentType::PnpBjt
        | ComponentType::NpnBjt4
        | ComponentType::PnpBjt4
        | ComponentType::NpnBjt5
        | ComponentType::PnpBjt5
        | ComponentType::Njfet
        | ComponentType::Pjfet
        | ComponentType::Nmesfet
        | ComponentType::Pmesfet => "model",
        ComponentType::Vcvs => "gain",
        ComponentType::Vccs => "gm",
        ComponentType::Ccvs => "rm",
        ComponentType::Cccs => "gain",
        ComponentType::OpAmp => "gain",
        ComponentType::CurrentSourceNoise => "na",
        ComponentType::Ground => "name",
        // Catch-all for any other component types
        _ => "value",
    }
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
/// Build the component editor's typed draft map.
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
        let value = if component.kind.is_pwl_source() || component.kind == ComponentType::Port {
            PropertyValue::String(component.value.clone())
        } else {
            PropertyValue::Expression(component.value.clone())
        };
        properties.insert(primary_prop.to_string(), value);
    }

    // A legacy Port may only carry `dir=`. Materialize the complete typed
    // contract at the property boundary so every schema field is editable and
    // applying the unchanged form upgrades it to the durable representation.
    // Canonical contract values intentionally win over legacy aliases such as
    // `input`, `digital`, or `real`.
    if component.kind == ComponentType::Port
        && let Some(contract) = component.port_contract()
    {
        for (key, value) in [
            ("dir", contract.direction.keyword().to_owned()),
            ("signal_type", contract.signal_type.keyword().to_owned()),
            ("discipline", contract.discipline.keyword().to_owned()),
            ("documentation", contract.documentation),
        ] {
            properties.insert(
                key.to_owned(),
                property_value_from_schema(component.kind, key, value, registry),
            );
        }
    }

    // Parse additional parameters from params string
    let parsed_params = parse_params_string(&component.params);
    for (key, value) in parsed_params {
        // Skip if this is the primary property (already handled)
        if key == primary_prop {
            continue;
        }

        if component.kind == ComponentType::Port && is_port_contract_property(&key) {
            continue;
        }

        let prop_value = property_value_from_schema(component.kind, &key, value, registry);

        properties.insert(key, prop_value);
    }

    properties
}

/// Run the same complete schema, expression, numeric-draft, and PWL checks
/// used by the interactive property editor without mutating application state.
/// Keeping this adapter at the property boundary prevents save validation from
/// drifting into a weaker duplicate of the production editor contract.
pub(crate) fn validate_component_properties(
    component: &Component,
    registry: &PropertyRegistry,
) -> Vec<(String, String)> {
    let Some(sheet) = registry.get(component.kind) else {
        return vec![(
            "schema".to_owned(),
            format!(
                "{} has no registered property schema",
                component.kind.display_name()
            ),
        )];
    };
    let values = collect_properties_from_component(component, registry);
    let mut validator = crate::properties::TabbedPropertyDialogState::default();
    validator.open_for_component(
        component.id,
        component.name.clone(),
        component.kind,
        sheet,
        values,
        crate::properties::ComponentPropertySession::new(
            component.clone(),
            0,
            0,
            "detached property validation".to_owned(),
        ),
    );
    validator.validate_all(sheet);
    let mut errors = validator.validation_errors.into_iter().collect::<Vec<_>>();
    errors.sort_by(|left, right| left.0.cmp(&right.0));
    errors
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
        // Validation is performed against the normalized reference; persist
        // that same identity so whitespace cannot create visually identical
        // but electrically distinct instance names.
        component.name = name.trim().to_owned();
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
    let mut secondary_params: HashMap<String, String> = if component.kind == ComponentType::Port {
        // Port contracts can coexist with future/extension metadata. Preserve
        // fields unknown to this property sheet instead of erasing them while
        // upgrading a legacy contract.
        parse_params_string(&component.params)
    } else {
        HashMap::new()
    };

    for (key, value) in properties {
        // Skip name and primary property
        if key == "name" || key == primary_prop || key == "symbol" || key == "model_corner" {
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

        // Port contract fields stay explicit even at schema defaults. This
        // makes their durable meaning independent of defaults changing in a
        // future release and upgrades legacy `dir=`-only components safely.
        if !is_default || (component.kind == ComponentType::Port && is_port_contract_property(key))
        {
            secondary_params.insert(key.clone(), string_value);
        }
    }

    if component.kind == ComponentType::Port
        && let Some(contract) = component.port_contract()
    {
        secondary_params
            .entry("dir".to_owned())
            .or_insert_with(|| contract.direction.keyword().to_owned());
        secondary_params
            .entry("signal_type".to_owned())
            .or_insert_with(|| contract.signal_type.keyword().to_owned());
        secondary_params
            .entry("discipline".to_owned())
            .or_insert_with(|| contract.discipline.keyword().to_owned());
        secondary_params
            .entry("documentation".to_owned())
            .or_insert(contract.documentation);
    }

    // Format secondary parameters into params string
    component.params = format_params_string(&secondary_params);
}

fn is_port_contract_property(key: &str) -> bool {
    matches!(key, "dir" | "signal_type" | "discipline" | "documentation")
}

fn property_value_from_schema(
    kind: ComponentType,
    key: &str,
    value: String,
    registry: &PropertyRegistry,
) -> PropertyValue {
    let Some(definition) = registry
        .get(kind)
        .and_then(|sheet| sheet.iter().find(|definition| definition.name == key))
    else {
        return PropertyValue::Expression(value);
    };

    match &definition.default_value {
        PropertyValue::Number { .. } => {
            if let Ok(number) = crate::quantity::parse_engineering_value(&value) {
                PropertyValue::Number {
                    value: number,
                    unit: definition.unit.clone(),
                }
            } else {
                PropertyValue::Expression(value)
            }
        }
        PropertyValue::String(_) => PropertyValue::String(value),
        PropertyValue::Expression(_) => PropertyValue::Expression(value),
        PropertyValue::Boolean(_) => PropertyValue::Boolean(matches!(
            value.to_lowercase().as_str(),
            "true" | "1" | "yes" | "on"
        )),
        PropertyValue::Enum { options, .. } => PropertyValue::Enum {
            selected: value,
            options: options.clone(),
        },
    }
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
    fn component_bridge_persists_the_normalized_reference_identity() {
        let registry = PropertyRegistry::new();
        let mut component = Component::new(1, ComponentType::Resistor, Point::origin());
        let properties = HashMap::from([(
            "name".to_owned(),
            PropertyValue::String("  R42  ".to_owned()),
        )]);

        apply_properties_to_component(&mut component, &properties, &registry);

        assert_eq!(component.name, "R42");
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
    fn ac_source_magnitude_updates_the_emitted_primary_value() {
        let registry = PropertyRegistry::new();
        for kind in [
            ComponentType::VoltageSourceAc,
            ComponentType::CurrentSourceAc,
        ] {
            let mut component =
                Component::new(1, kind, Point::origin()).with_name_value("SRC1", "1");
            let mut properties = collect_properties_from_component(&component, &registry);
            properties.insert("ac".to_owned(), PropertyValue::number(2.5));

            apply_properties_to_component(&mut component, &properties, &registry);

            assert_eq!(component.value, "2.5");
            assert!(!parse_params_string(&component.params).contains_key("ac"));
        }
    }

    #[test]
    fn mos_width_never_replaces_the_model_binding() {
        let registry = PropertyRegistry::new();
        for kind in [ComponentType::Nmos, ComponentType::Pmos] {
            let mut component =
                Component::new(1, kind, Point::origin()).with_name_value("M1", "core_model");
            let mut properties = collect_properties_from_component(&component, &registry);
            properties.insert("w".to_owned(), PropertyValue::number(2e-6));

            apply_properties_to_component(&mut component, &properties, &registry);

            assert_eq!(component.value, "core_model");
            assert_eq!(
                parse_params_string(&component.params)
                    .get("w")
                    .map(String::as_str),
                Some("0.000002")
            );
        }
    }

    #[test]
    fn op_amp_gain_is_the_positional_primary_value() {
        let registry = PropertyRegistry::new();
        let mut component = Component::new(1, ComponentType::OpAmp, Point::origin())
            .with_name_value("E1", "100000");
        let mut properties = collect_properties_from_component(&component, &registry);
        properties.insert("gain".to_owned(), PropertyValue::number(250000.0));

        apply_properties_to_component(&mut component, &properties, &registry);

        assert_eq!(component.value, "250000");
        assert!(!parse_params_string(&component.params).contains_key("gain"));
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

    #[test]
    fn detached_validation_uses_the_production_property_contract() {
        let registry = PropertyRegistry::new();
        let valid =
            Component::new(1, ComponentType::Resistor, Point::origin()).with_name_value("R1", "1k");
        let invalid = Component::new(2, ComponentType::Resistor, Point::origin())
            .with_name_value("R2", "1k+");

        assert!(validate_component_properties(&valid, &registry).is_empty());
        assert!(
            validate_component_properties(&invalid, &registry)
                .iter()
                .any(|(field, _)| field == "r")
        );
    }

    #[test]
    fn legacy_port_contract_is_materialized_without_losing_extension_metadata() {
        let registry = PropertyRegistry::new();
        let mut component =
            Component::new(7, ComponentType::Port, Point::origin()).with_name_value("", "BIAS_EN");
        component.params = "dir=input vendor_role=calibration".to_owned();

        let properties = collect_properties_from_component(&component, &registry);

        assert_eq!(
            properties.get("value"),
            Some(&PropertyValue::String("BIAS_EN".to_owned()))
        );
        assert!(matches!(
            properties.get("dir"),
            Some(PropertyValue::Enum { selected, .. }) if selected == "in"
        ));
        assert!(matches!(
            properties.get("signal_type"),
            Some(PropertyValue::Enum { selected, .. }) if selected == "analog"
        ));
        assert!(matches!(
            properties.get("discipline"),
            Some(PropertyValue::Enum { selected, .. }) if selected == "electrical"
        ));
        assert!(matches!(
            properties.get("documentation"),
            Some(PropertyValue::String(value)) if !value.is_empty()
        ));

        apply_properties_to_component(&mut component, &properties, &registry);
        let encoded = parse_params_string(&component.params);
        assert_eq!(encoded.get("dir").map(String::as_str), Some("in"));
        assert_eq!(
            encoded.get("signal_type").map(String::as_str),
            Some("analog")
        );
        assert_eq!(
            encoded.get("discipline").map(String::as_str),
            Some("electrical")
        );
        assert_eq!(
            encoded.get("vendor_role").map(String::as_str),
            Some("calibration")
        );
    }

    #[test]
    fn typed_port_property_edit_preserves_order_and_updates_the_complete_contract() {
        let registry = PropertyRegistry::new();
        let mut state = crate::state::SchematicState::default();
        let pending = crate::state::PendingPortPlacement::new(
            "OUT",
            crate::state::PortDirectionType::OutputAnalog,
            crate::state::PortDiscipline::Electrical,
            state.topology_version(),
            state.next_interface_order(),
        );
        let id = state
            .place_pending_port(Point::origin(), pending)
            .expect("typed port places");
        let mut component = state
            .components
            .iter()
            .find(|component| component.id == id)
            .expect("port exists")
            .clone();
        let mut properties = collect_properties_from_component(&component, &registry);
        properties.insert(
            "discipline".to_owned(),
            PropertyValue::enumeration(
                "thermal",
                ["electrical", "logic", "wreal", "thermal"]
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
            ),
        );
        properties.insert(
            "documentation".to_owned(),
            PropertyValue::String("Thermal monitor output".to_owned()),
        );

        apply_properties_to_component(&mut component, &properties, &registry);

        let contract = component.port_contract().expect("typed contract remains");
        assert_eq!(contract.direction, crate::state::PortDirection::Out);
        assert_eq!(contract.signal_type, crate::state::PortSignalType::Analog);
        assert_eq!(contract.discipline, crate::state::PortDiscipline::Thermal);
        assert_eq!(contract.netlist_order, Some(1));
        assert_eq!(contract.documentation, "Thermal monitor output");
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
