//! Component property mapping scenarios.

use super::*;
use crate::state::Point;
use rspice_design::properties::PropertyCatalog;

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
    let registry = PropertyCatalog::new();
    let value = 1.234_567_890_123_456_7e-6;
    let mut component = Component::new(1, ComponentType::Resistor, Point::origin());
    let properties = HashMap::from([("r".to_owned(), PropertyValue::number(value))]);

    let sheet = registry.get(component.kind);
    apply_properties_with_sheet(&mut component, &properties, sheet).unwrap();

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
    let registry = PropertyCatalog::new();
    let mut component = Component::new(1, ComponentType::Resistor, Point::origin());
    let properties = HashMap::from([(
        "name".to_owned(),
        PropertyValue::String("  R42  ".to_owned()),
    )]);

    let sheet = registry.get(component.kind);
    apply_properties_with_sheet(&mut component, &properties, sheet).unwrap();

    assert_eq!(component.name, "R42");
}

#[test]
fn component_bridge_does_not_drop_a_small_nonzero_default_delta() {
    let registry = PropertyCatalog::new();
    let value = 5.0e-16;
    let mut component = Component::new(1, ComponentType::Resistor, Point::origin());
    let properties = HashMap::from([("tc1".to_owned(), PropertyValue::number(value))]);

    assert!(!property_values_equal(
        &PropertyValue::number(value),
        &PropertyValue::number(0.0)
    ));
    let sheet = registry.get(component.kind);
    apply_properties_with_sheet(&mut component, &properties, sheet).unwrap();

    let serialized = parse_params_string(&component.params);
    let tc1 = serialized.get("tc1").expect("non-default tc1 is retained");
    assert_eq!(tc1.parse::<f64>().expect("serialized tc1"), value);
}

#[test]
fn ac_source_magnitude_updates_the_emitted_primary_value() {
    let registry = PropertyCatalog::new();
    for kind in [
        ComponentType::VoltageSourceAc,
        ComponentType::CurrentSourceAc,
    ] {
        let mut component = Component::new(1, kind, Point::origin()).with_name_value("SRC1", "1");
        let mut properties =
            collect_properties_with_sheet(&component, registry.get(component.kind));
        properties.insert("ac".to_owned(), PropertyValue::number(2.5));

        let sheet = registry.get(component.kind);
        apply_properties_with_sheet(&mut component, &properties, sheet).unwrap();

        assert_eq!(component.value, "2.5");
        assert!(!parse_params_string(&component.params).contains_key("ac"));
    }
}

#[test]
fn mos_width_never_replaces_the_model_binding() {
    let registry = PropertyCatalog::new();
    for kind in [ComponentType::Nmos, ComponentType::Pmos] {
        let mut component =
            Component::new(1, kind, Point::origin()).with_name_value("M1", "core_model");
        let mut properties =
            collect_properties_with_sheet(&component, registry.get(component.kind));
        properties.insert("w".to_owned(), PropertyValue::number(2e-6));

        let sheet = registry.get(component.kind);
        apply_properties_with_sheet(&mut component, &properties, sheet).unwrap();

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
    let registry = PropertyCatalog::new();
    let mut component =
        Component::new(1, ComponentType::OpAmp, Point::origin()).with_name_value("E1", "100000");
    let mut properties = collect_properties_with_sheet(&component, registry.get(component.kind));
    properties.insert("gain".to_owned(), PropertyValue::number(250000.0));

    let sheet = registry.get(component.kind);
    apply_properties_with_sheet(&mut component, &properties, sheet).unwrap();

    assert_eq!(component.value, "250000");
    assert!(!parse_params_string(&component.params).contains_key("gain"));
}

#[test]
fn blank_optional_source_defaults_are_omitted_from_durable_parameters() {
    let registry = PropertyCatalog::new();
    let mut component = Component::new(1, ComponentType::VoltageSource, Point::origin());
    let properties = HashMap::from([
        (
            "pacdbm".to_owned(),
            PropertyValue::Expression(String::new()),
        ),
        ("rp".to_owned(), PropertyValue::Expression(String::new())),
    ]);

    let sheet = registry.get(component.kind);
    apply_properties_with_sheet(&mut component, &properties, sheet).unwrap();

    assert!(component.params.is_empty());
    assert!(!component.params.contains("inf"));
}

#[test]
fn legacy_port_contract_is_materialized_without_losing_extension_metadata() {
    let registry = PropertyCatalog::new();
    let mut component =
        Component::new(7, ComponentType::Port, Point::origin()).with_name_value("", "BIAS_EN");
    component.params = "dir=input vendor_role=calibration".to_owned();

    let properties = collect_properties_with_sheet(&component, registry.get(component.kind));

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

    let sheet = registry.get(component.kind);
    apply_properties_with_sheet(&mut component, &properties, sheet).unwrap();
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
fn cell_instance_multiplicity_round_trips_through_the_typed_field() {
    let registry = PropertyCatalog::new();
    let mut component = Component::new(1, ComponentType::CellInstance, Point::origin());
    component.params = "wp=2u".to_owned();

    let mut properties = collect_properties_with_sheet(&component, registry.get(component.kind));
    assert_eq!(
        properties.get(InstanceMultiplicity::PARAMETER_NAME),
        Some(&PropertyValue::number(1.0)),
        "an instance standing for itself reads as one, not as nothing"
    );

    properties.insert(
        InstanceMultiplicity::PARAMETER_NAME.to_owned(),
        PropertyValue::number(4.0),
    );
    let sheet = registry.get(component.kind);
    apply_properties_with_sheet(&mut component, &properties, sheet).unwrap();
    assert_eq!(
        component.multiplicity.map(InstanceMultiplicity::value),
        Some(4.0)
    );
    let params = parse_params_string(&component.params);
    assert!(
        !params.contains_key(InstanceMultiplicity::PARAMETER_NAME),
        "the reserved multiplier must never land in parameter text: {}",
        component.params
    );
    assert_eq!(params.get("wp").map(String::as_str), Some("2u"));

    properties.insert(
        InstanceMultiplicity::PARAMETER_NAME.to_owned(),
        PropertyValue::number(1.0),
    );
    let sheet = registry.get(component.kind);
    apply_properties_with_sheet(&mut component, &properties, sheet).unwrap();
    assert!(component.multiplicity.is_none());

    properties.insert(
        InstanceMultiplicity::PARAMETER_NAME.to_owned(),
        PropertyValue::number(-2.0),
    );
    let sheet = registry.get(component.kind);
    apply_properties_with_sheet(&mut component, &properties, sheet).unwrap();
    assert!(
        component.multiplicity.is_none(),
        "a value the engine refuses is not stored on the instance"
    );
}

#[test]
fn a_primitive_keeps_its_own_m_parameter_in_parameter_text() {
    let registry = PropertyCatalog::new();
    let mut component = Component::new(1, ComponentType::Nmos, Point::origin());
    let properties = HashMap::from([(
        InstanceMultiplicity::PARAMETER_NAME.to_owned(),
        PropertyValue::number(4.0),
    )]);

    let sheet = registry.get(component.kind);
    apply_properties_with_sheet(&mut component, &properties, sheet).unwrap();

    assert!(component.multiplicity.is_none());
    assert_eq!(
        parse_params_string(&component.params)
            .get(InstanceMultiplicity::PARAMETER_NAME)
            .map(String::as_str),
        Some("4"),
        "on a device card `m` is an ordinary device parameter"
    );
}
