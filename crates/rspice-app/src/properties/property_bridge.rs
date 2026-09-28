//! Bind component property maps to the active editor schema and validation session.

use super::PropertyEditorSchema;
use crate::state::{Component, PropertySheet, PropertyValue};
use std::collections::HashMap;

pub use rspice_design::properties::component::get_primary_property_name;
use rspice_design::properties::component::{
    apply_properties_with_sheet, collect_properties_with_sheet,
};
pub(crate) use rspice_design::properties::component::{
    component_source_contract, property_value_to_string, source_commit_refusal,
};

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
    registry: &PropertyEditorSchema,
) -> HashMap<String, PropertyValue> {
    collect_properties_with_sheet(component, registry.get(component.kind))
}

/// Apply the current transaction's schema to its component field map.
pub fn apply_properties_to_component(
    component: &mut Component,
    properties: &HashMap<String, PropertyValue>,
    registry: &PropertyEditorSchema,
) -> Result<(), String> {
    let sheet = registry.get(component.kind);
    apply_properties_with_sheet(component, properties, sheet)
}

/// Run the same complete schema, expression, numeric-draft, and PWL checks
/// used by the interactive property editor without mutating application state.
/// Keeping this adapter at the property boundary prevents save validation from
/// drifting into a weaker duplicate of the production editor contract.
/// The caller supplies this instance's authoritative sheet; a dynamic sheet
/// retained by another component's open dialog is not validation authority.
pub(crate) fn validate_component_properties(
    component: &Component,
    sheet: Option<&PropertySheet>,
) -> Vec<(String, String)> {
    if let Err(error) = crate::state::params_string::validate_parameter_text(&component.params) {
        return vec![("parameters".to_owned(), error)];
    }
    let Some(sheet) = sheet else {
        return vec![(
            "schema".to_owned(),
            format!(
                "{} has no registered property schema",
                component.kind.display_name()
            ),
        )];
    };
    let values = collect_properties_with_sheet(component, Some(sheet));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::parse_params_string;
    use crate::state::{ComponentType, Point};

    #[test]
    fn detached_validation_uses_the_production_property_contract() {
        let registry = PropertyEditorSchema::new();
        let valid =
            Component::new(1, ComponentType::Resistor, Point::origin()).with_name_value("R1", "1k");
        let invalid = Component::new(2, ComponentType::Resistor, Point::origin())
            .with_name_value("R2", "1k+");

        assert!(validate_component_properties(&valid, registry.get(valid.kind)).is_empty());
        assert!(
            validate_component_properties(&invalid, registry.get(invalid.kind))
                .iter()
                .any(|(field, _)| field == "r")
        );
    }

    #[test]
    fn typed_port_property_edit_preserves_order_and_updates_the_complete_contract() {
        let registry = PropertyEditorSchema::new();
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
            .document()
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

        apply_properties_to_component(&mut component, &properties, &registry).unwrap();

        let contract = component.port_contract().expect("typed contract remains");
        assert_eq!(contract.direction, crate::state::PortDirection::Out);
        assert_eq!(contract.signal_type, crate::state::PortSignalType::Analog);
        assert_eq!(contract.discipline, crate::state::PortDiscipline::Thermal);
        assert_eq!(contract.netlist_order, Some(1));
        assert_eq!(contract.documentation, "Thermal monitor output");
    }

    /// The interface position is editable after placement, and clearing it
    /// really does return the port to document order — the Port branch seeds
    /// its parameter map from the existing params, so a defaulted field has to
    /// be removed rather than merely left uninserted.
    #[test]
    fn interface_order_is_editable_and_clears_back_to_document_order() {
        let registry = PropertyEditorSchema::new();
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
            .document()
            .components
            .iter()
            .find(|component| component.id == id)
            .expect("port exists")
            .clone();

        let mut properties = collect_properties_from_component(&component, &registry);
        assert_eq!(
            properties.get("interface_order"),
            Some(&PropertyValue::number(1.0)),
            "a placed port shows the position placement gave it"
        );

        properties.insert("interface_order".to_owned(), PropertyValue::number(4.0));
        apply_properties_to_component(&mut component, &properties, &registry).unwrap();
        assert_eq!(
            component.port_contract().expect("contract").netlist_order,
            Some(4)
        );
        // A whole number must not reach the params string as `4.0`; the
        // contract reader would see no position at all.
        assert_eq!(
            parse_params_string(&component.params)
                .get("interface_order")
                .map(String::as_str),
            Some("4")
        );

        properties.insert("interface_order".to_owned(), PropertyValue::number(0.0));
        apply_properties_to_component(&mut component, &properties, &registry).unwrap();
        assert_eq!(
            component.port_contract().expect("contract").netlist_order,
            None
        );
        assert!(
            !parse_params_string(&component.params).contains_key("interface_order"),
            "clearing the position removes the entry: {}",
            component.params
        );
    }
}

#[cfg(test)]
mod pwl_tests {
    use super::*;
    use crate::state::ComponentType;

    #[test]
    fn property_application_refuses_malformed_or_ambiguous_text_before_any_mutation() {
        let registry = PropertyEditorSchema::new();
        for source in ["note='unterminated", "temp=27 TEMP=85"] {
            let mut component =
                Component::new(1, ComponentType::Resistor, crate::state::Point::origin());
            component.params = source.to_owned();
            let original = component.clone();
            let values = HashMap::from([
                ("name".to_owned(), PropertyValue::String("R9".to_owned())),
                ("r".to_owned(), PropertyValue::Expression("2k".to_owned())),
            ]);
            assert!(apply_properties_to_component(&mut component, &values, &registry).is_err());
            assert_eq!(component, original);
            assert_eq!(
                validate_component_properties(&component, registry.get(component.kind))[0].0,
                "parameters"
            );
        }
    }
}
