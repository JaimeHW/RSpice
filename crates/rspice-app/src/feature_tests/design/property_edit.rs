//! Opening and saving a placed instance's property editor through the whole application:
//! open_property_editor and the validated save answer over AppState.

use crate::workbench::app::project_transactions::WholeStateValidation;
use crate::workbench::app_state::AppState;
use rspice_app_design::workbench::app::actions::property_edit::selected_object_properties_available_in;
use rspice_app_state::state::{
    Component, GeneratedSymbolViews, Library, LibraryCellInstance, ModelBoundSymbolDefinition,
    ParameterInheritance, Point, PortDirection, PropertyType, SymbolElectricalType,
    SymbolGraphicTemplate, SymbolIdentity, SymbolModelReference, SymbolNetlistBinding,
    SymbolParameterConstraints, SymbolParameterDefault, SymbolParameterField, SymbolParameterForm,
    SymbolParameterSection, SymbolParameterVisibility, SymbolPinDefinition, SymbolPinSide,
    SymbolSourceContract,
};
use rspice_app_state::state::{ComponentType, MODEL_BOUND_SYMBOL_METADATA_KEY, PropertyValue};

fn model_bound_definition() -> ModelBoundSymbolDefinition {
    let model = SymbolModelReference::new("vendor_cmos", "nmos_core").with_source_path(
        std::env::current_dir()
            .expect("workspace")
            .join("models/nmos.lib")
            .display()
            .to_string(),
    );
    let pins = [
        ("D", PortDirection::InOut, SymbolPinSide::Right),
        ("G", PortDirection::In, SymbolPinSide::Left),
        ("S", PortDirection::InOut, SymbolPinSide::Right),
        ("B", PortDirection::InOut, SymbolPinSide::Left),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (name, direction, side))| {
        SymbolPinDefinition::new(
            name,
            SymbolElectricalType::Analog,
            direction,
            side,
            index + 1,
        )
    })
    .collect::<Vec<_>>();
    let ports = pins.iter().map(SymbolPinDefinition::port_spec).collect();
    let number_field = |key: &str, label: &str, default: &str| SymbolParameterField {
        key: key.to_owned(),
        label: label.to_owned(),
        help: format!("Per-instance {label} override."),
        property_type: PropertyType::Number,
        default: SymbolParameterDefault::Number {
            engineering: default.to_owned(),
            unit: Some("m".to_owned()),
        },
        unit: Some("m".to_owned()),
        constraints: SymbolParameterConstraints {
            minimum: Some("1n".to_owned()),
            maximum: Some("1m".to_owned()),
            enum_values: Vec::new(),
            max_length: None,
        },
        inheritance: ParameterInheritance::InstanceOverride,
        visibility: SymbolParameterVisibility::Visible,
        required: true,
        aliases: Vec::new(),
    };
    let form = SymbolParameterForm {
        revision: 1,
        sections: vec![SymbolParameterSection {
            key: "geometry".to_owned(),
            label: "Geometry".to_owned(),
            help: "Instance geometry passed to the MOS implementation.".to_owned(),
            fields: vec![
                number_field("w", "Width", "1u"),
                number_field("l", "Length", "180n"),
            ],
        }],
    };
    ModelBoundSymbolDefinition::new(
        SymbolIdentity::new("model_cells", "nmos_core", 1, "model-cells-nmos-core-v1"),
        SymbolSourceContract::model(model.clone(), ports),
        pins,
        SymbolGraphicTemplate::RectangularIc,
        form,
        SymbolNetlistBinding {
            device_prefix: "M".to_owned(),
            model: Some(model),
            template: "M{name} {nodes} {model} {params}".to_owned(),
            parameter_order: vec!["w".to_owned(), "l".to_owned()],
        },
        GeneratedSymbolViews::default(),
    )
}

fn install_definition(
    state: &mut AppState,
    definition: &ModelBoundSymbolDefinition,
) -> LibraryCellInstance {
    let mut library = Library::new(&definition.identity.library);
    rspice_app_state::state::prepare_symbol_construction(definition, &library)
        .expect("construction plan")
        .commit(&mut library)
        .expect("definition publication");
    state.model.library_manager.add_library(library);

    let implementation_view = match &definition.source {
        SymbolSourceContract::Model { model, .. } => model.implementation_view.view_name(),
        SymbolSourceContract::ExistingSchematicPins { schematic_view, .. } => schematic_view,
        SymbolSourceContract::BlankExplicitContract => "symbol",
    };
    let mut binding = LibraryCellInstance::new(
        &definition.identity.library,
        &definition.identity.cell,
        implementation_view,
    );
    let ports = definition
        .pins
        .iter()
        .map(SymbolPinDefinition::port_spec)
        .collect::<Vec<_>>();
    binding.bind_interface(&ports);
    binding.netlist_template = definition
        .netlist
        .is_executable()
        .then(|| definition.netlist.template.clone());
    binding.reference_prefix = (!definition.netlist.device_prefix.is_empty())
        .then(|| definition.netlist.device_prefix.clone());
    binding.parameter_order = definition.netlist.parameter_order.clone();
    binding
}

#[test]
fn validated_save_uses_each_masters_schema_without_borrowing_the_open_dialog() {
    use crate::workbench::app::dialogs::check_and_save_validation::CheckAndSaveValidationReport;

    let mut state = AppState::default();
    let first = model_bound_definition();
    let first_binding = install_definition(&mut state, &first);
    let mut second = model_bound_definition();
    second.identity.library = "large_geometry".to_owned();
    second.identity.binding_id = "large-geometry-v1".to_owned();
    let width = &mut second.parameter_form.sections[0].fields[0];
    width.default = SymbolParameterDefault::Number {
        engineering: "2m".to_owned(),
        unit: Some("m".to_owned()),
    };
    width.constraints.minimum = Some("1m".to_owned());
    width.constraints.maximum = Some("1".to_owned());
    let second_binding = install_definition(&mut state, &second);
    state.model.schematic.document_mut_for_test().components = vec![
        Component::new(1, ComponentType::CellInstance, Point::new(0, 0))
            .with_library_cell(first_binding)
            .with_name_value("M1", "nmos_core"),
        Component::new(2, ComponentType::CellInstance, Point::new(100, 0))
            .with_library_cell(second_binding)
            .with_name_value("M2", "nmos_core"),
    ];
    state.model.schematic.document_mut_for_test().components[0].params = "w=2u l=180n".to_owned();

    for (width, invalid) in [("2m", false), ("2u", true)] {
        state.model.schematic.document_mut_for_test().components[1].params =
            format!("w={width} l=180n");
        for owner in [None, Some(1), Some(2)] {
            state.design.tabbed_property_dialog.close();
            state.design.property_registry.clear_cell_instance_sheet();
            if let Some(owner) = owner {
                state.open_property_editor(owner);
                assert!(state.design.tabbed_property_dialog.open);
            }
            let original = state.model.schematic.document().components.clone();
            let draft = state.design.tabbed_property_dialog.draft.values.clone();
            let report = CheckAndSaveValidationReport::validate(&state).unwrap();
            let errors = report
                .blockers()
                .iter()
                .filter(|finding| finding.source == "parameters")
                .map(|finding| finding.message.as_str())
                .collect::<Vec<_>>();
            if invalid {
                assert_eq!(errors.len(), 1, "owner {owner:?}: {errors:?}");
                assert!(errors[0].contains("M2 property w:"), "{errors:?}");
            } else {
                assert!(errors.is_empty(), "owner {owner:?}: {errors:?}");
            }
            assert_eq!(state.model.schematic.document().components, original);
            assert_eq!(state.design.tabbed_property_dialog.draft.values, draft);
        }
    }

    // Invalid published metadata must not fall back to the other master's
    // retained form or to an identity-only schema.
    state.model.schematic.document_mut_for_test().components[1].params = "w=2m l=180n".to_owned();
    state
        .model
        .library_manager
        .get_library_mut(&first.identity.library)
        .unwrap()
        .get_cell_mut(&first.identity.cell)
        .unwrap()
        .metadata
        .insert(MODEL_BOUND_SYMBOL_METADATA_KEY.to_owned(), "{".to_owned());
    let report = CheckAndSaveValidationReport::validate(&state).unwrap();
    let errors = report
        .blockers()
        .iter()
        .filter(|finding| finding.source == "parameters")
        .map(|finding| finding.message.as_str())
        .collect::<Vec<_>>();
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].contains("M1 property schema:"), "{errors:?}");
}

#[test]
fn placed_model_bound_instance_opens_its_authoritative_typed_form() {
    let mut state = AppState::default();
    let definition = model_bound_definition();
    let binding = install_definition(&mut state, &definition);
    let mut instance = Component::new(44, ComponentType::CellInstance, Point::origin())
        .with_library_cell(binding)
        .with_name_value("M44", "nmos_core");
    instance.params = "w=2u l=220n".to_owned();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(instance);
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(44);

    assert!(selected_object_properties_available_in(
        &state.design.property_registry,
        &state.model
    ));
    state.open_property_editor(44);

    assert!(state.design.tabbed_property_dialog.open);
    assert_eq!(
        state.design.tabbed_property_dialog.draft.component_type,
        Some(ComponentType::CellInstance)
    );
    let sheet = state
        .design
        .property_registry
        .get(ComponentType::CellInstance)
        .expect("transaction-scoped cell schema");
    assert_eq!(
        sheet.get("w").map(|field| field.prop_type),
        Some(PropertyType::Number)
    );
    assert_eq!(
        sheet.get("l").map(|field| field.prop_type),
        Some(PropertyType::Number)
    );
    assert_eq!(
        state.design.tabbed_property_dialog.draft.values.get("w"),
        Some(&PropertyValue::Number {
            value: 2e-6,
            unit: Some("m".to_owned()),
        })
    );
    assert!(matches!(
        state.design.tabbed_property_dialog.draft.values.get("name"),
        Some(PropertyValue::String(name)) if name == "M44"
    ));
}

#[test]
fn a_plain_schematic_cell_gets_its_declared_parameters_and_a_typed_multiplicity() {
    let mut state = AppState::default();
    let mut cell = rspice_app_state::state::Cell::new("amp");
    cell.metadata.insert(
        "cdf.parameter_contract".to_owned(),
        r#"[{"name":"r","default":"1k"},{"name":"c","required":true,"aliases":["cap"]}]"#
            .to_owned(),
    );
    cell.add_view(rspice_app_state::state::View::new(
        "schematic",
        rspice_app_state::state::ViewType::Schematic,
    ));
    let mut library = Library::new("work");
    library.add_cell(cell);
    state.model.library_manager.add_library(library);
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(
            Component::new(9, ComponentType::CellInstance, Point::origin())
                .with_library_cell(LibraryCellInstance::new("work", "amp", "schematic"))
                .with_name_value("X9", "amp"),
        );
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(9);

    assert!(selected_object_properties_available_in(
        &state.design.property_registry,
        &state.model
    ));
    state.open_property_editor(9);
    assert!(state.design.tabbed_property_dialog.open);

    let sheet = state
        .design
        .property_registry
        .get(ComponentType::CellInstance)
        .expect("declared cell contract");
    assert!(sheet.get("name").is_some());
    assert_eq!(
        sheet.get("r").map(|field| field.prop_type),
        Some(PropertyType::Expression)
    );
    assert!(sheet.get("c").is_some_and(|field| field.required));
    let multiplicity = sheet.get("m").expect("typed multiplicity row");
    assert_eq!(multiplicity.prop_type, PropertyType::Number);
    assert_eq!(multiplicity.default_value, PropertyValue::number(1.0));
    assert!(multiplicity.validate(&PropertyValue::number(0.0)).is_err());
    assert!(multiplicity.validate(&PropertyValue::number(2.5)).is_ok());
}

#[test]
fn every_static_component_family_opens_its_registered_editor() {
    for (index, kind) in ComponentType::ALL
        .into_iter()
        .filter(|kind| *kind != ComponentType::CellInstance)
        .enumerate()
    {
        let mut state = AppState::default();
        state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .clear();
        let name = if kind.spice_prefix().is_empty() {
            if kind == ComponentType::Ground {
                "GND".to_owned()
            } else {
                String::new()
            }
        } else {
            format!("{}1", kind.spice_prefix())
        };
        let component =
            Component::new(index as u64 + 1, kind, Point::origin()).with_name_value(name, "");
        state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .push(component);

        state.open_property_editor(index as u64 + 1);

        assert!(
            state.design.tabbed_property_dialog.open,
            "{} did not open a component editor",
            kind.display_name()
        );
        assert_eq!(
            state.design.tabbed_property_dialog.draft.component_type,
            Some(kind)
        );
    }
}

#[test]
fn legacy_and_review_only_cells_get_isolated_identity_forms() {
    let mut state = AppState::default();
    let definition = model_bound_definition();
    let binding = install_definition(&mut state, &definition);
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(
            Component::new(1, ComponentType::CellInstance, Point::origin())
                .with_library_cell(binding)
                .with_name_value("M1", "nmos_core"),
        );
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(1);
    state.open_property_editor(1);
    assert!(state.design.tabbed_property_dialog.open);
    state.design.tabbed_property_dialog.close();
    assert!(
        state
            .design
            .property_registry
            .get(ComponentType::CellInstance)
            .is_some()
    );

    let mut legacy_library = Library::new("legacy");
    legacy_library.add_cell(rspice_app_state::state::Cell::new("opaque"));
    state.model.library_manager.add_library(legacy_library);
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(
            Component::new(2, ComponentType::CellInstance, Point::new(20, 0))
                .with_library_cell(LibraryCellInstance::new("legacy", "opaque", "schematic"))
                .with_name_value("X2", "opaque"),
        );
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(2);
    assert!(selected_object_properties_available_in(
        &state.design.property_registry,
        &state.model
    ));
    state.open_property_editor(2);
    assert!(state.design.tabbed_property_dialog.open);
    let legacy_sheet = state
        .design
        .property_registry
        .get(ComponentType::CellInstance)
        .expect("legacy identity form");
    assert!(legacy_sheet.get("name").is_some());
    assert!(legacy_sheet.get("w").is_none());
    state.design.tabbed_property_dialog.close();

    let mut review = model_bound_definition();
    review.identity.cell = "review_only".to_owned();
    review.identity.binding_id = "review-only-v1".to_owned();
    review.source = SymbolSourceContract::BlankExplicitContract;
    review.netlist = SymbolNetlistBinding::unbound();
    let review_binding = install_definition(&mut state, &review);
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(
            Component::new(3, ComponentType::CellInstance, Point::new(40, 0))
                .with_library_cell(review_binding)
                .with_name_value("X3", "review_only"),
        );
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(3);
    assert!(selected_object_properties_available_in(
        &state.design.property_registry,
        &state.model
    ));
    state.open_property_editor(3);
    assert!(state.design.tabbed_property_dialog.open);
    let review_sheet = state
        .design
        .property_registry
        .get(ComponentType::CellInstance)
        .expect("review identity form");
    assert!(review_sheet.get("name").is_some());
    assert!(review_sheet.get("w").is_none());
}
