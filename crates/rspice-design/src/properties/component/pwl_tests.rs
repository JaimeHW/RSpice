//! Component property mapping scenarios.

use super::*;
use crate::properties::PropertyCatalog;
use rspice_design_model::Point;

#[test]
fn new_pwl_sources_seed_the_registry_waveform_default() {
    let registry = PropertyCatalog::new();

    for (kind, expected) in [
        (ComponentType::VoltageSourcePwl, "0 0 1u 1 2u 0"),
        (ComponentType::CurrentSourcePwl, "0 0 1u 1m 2u 0"),
    ] {
        let component = Component::new(1, kind, Point::origin());
        let properties = collect_properties_with_sheet(&component, registry.get(component.kind));

        assert_eq!(
            properties.get("pwl_data"),
            Some(&PropertyValue::String(expected.to_owned()))
        );
    }
}

#[test]
fn authored_pwl_source_uses_the_schema_string_type() {
    let registry = PropertyCatalog::new();
    let component = Component::new(1, ComponentType::VoltageSourcePwl, Point::origin())
        .with_name_value("V1", "0 0 2n 1");

    let properties = collect_properties_with_sheet(&component, registry.get(component.kind));

    assert_eq!(
        properties.get("pwl_data"),
        Some(&PropertyValue::String("0 0 2n 1".to_owned()))
    );
}

/// A number-typed field authored with its unit is a number, not an
/// expression. `1ms` used to reach the sheet as `Expression("1ms")` —
/// the notation parser refused the `s` — so a period typed in the form
/// every deck uses stopped being a quantity the moment it was written.
#[test]
fn a_number_field_authored_with_its_unit_stays_a_number() {
    let registry = PropertyCatalog::new();
    let parsed = property_value_from_schema(
        "per",
        "1ms".to_owned(),
        registry.get(ComponentType::VoltageSourcePulse),
    );

    assert_eq!(parsed.as_number(), Some(1e-3));
}

#[test]
fn property_round_trip_retains_flags_quoted_extensions_and_expression_groups() {
    let registry = PropertyCatalog::new();
    let mut component = Component::new(
        1,
        ComponentType::Diode,
        rspice_design_model::Point::origin(),
    );
    component.params = r#"off note="[\"a  b\" \"C:\\my data\"]" expr={V(a,b) + 1}"#.to_owned();
    let original = parse_params_string(&component.params);
    let mut properties = collect_properties_with_sheet(&component, registry.get(component.kind));
    properties.insert("name".to_owned(), PropertyValue::String("D9".to_owned()));
    let sheet = registry.get(component.kind);
    apply_properties_with_sheet(&mut component, &properties, sheet).unwrap();
    assert_eq!(component.name, "D9");
    let round_trip = parse_params_string(&component.params);
    for (key, value) in original {
        assert_eq!(round_trip.get(&key), Some(&value), "{key}");
    }
    let values = collect_properties_with_sheet(&component, registry.get(component.kind));
    assert_eq!(values.get("note"), properties.get("note"));
}
