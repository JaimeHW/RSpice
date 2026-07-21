//! Opening the property editor for a schematic component.

use super::AppState;
use crate::state::{
    Component, ComponentType, MODEL_BOUND_SYMBOL_METADATA_KEY, ModelBoundSymbolDefinition,
    PropertyDefinition, PropertySheet, PropertyType, PropertyValue, SymbolSourceContract,
};

/// Resolve the exact persistent parameter contract owned by a placed cell's
/// library master. Returning `None` is intentional for legacy and explicitly
/// unbound cells: they must not inherit whichever dynamic sheet was opened
/// most recently.
fn authoritative_cell_instance_sheet(
    state: &AppState,
    component: &Component,
) -> Result<Option<PropertySheet>, String> {
    if component.kind != ComponentType::CellInstance {
        return Ok(None);
    }
    let Some(binding) = component.library_cell.as_ref() else {
        return Ok(None);
    };
    let Some(cell) = state
        .library_manager
        .get_library(&binding.library)
        .and_then(|library| library.get_cell(&binding.cell))
    else {
        return Ok(None);
    };

    // Current publications keep the definition at cell scope. The view scan
    // is a deterministic compatibility path for early versioned documents;
    // conflicting view copies fail closed instead of guessing.
    let definition = if let Some(encoded) = cell.metadata.get(MODEL_BOUND_SYMBOL_METADATA_KEY) {
        Some(
            ModelBoundSymbolDefinition::from_json_bytes(encoded.as_bytes(), &cell.name)
                .map_err(|error| error.to_string())?,
        )
    } else {
        let mut found = None;
        for view in cell.views_sorted() {
            let Some(candidate) = ModelBoundSymbolDefinition::load_from_view(view)
                .map_err(|error| error.to_string())?
            else {
                continue;
            };
            if found
                .as_ref()
                .is_some_and(|current: &ModelBoundSymbolDefinition| current != &candidate)
            {
                return Err(format!(
                    "{}/{} contains conflicting versioned symbol definitions",
                    binding.library, binding.cell
                ));
            }
            found = Some(candidate);
        }
        found
    };
    let Some(definition) = definition else {
        return Ok(None);
    };
    if definition.identity.library != binding.library
        || definition.identity.cell != binding.cell
        || !definition.netlist.is_executable()
    {
        return Ok(None);
    }
    let implementation_view = match &definition.source {
        SymbolSourceContract::Model { model, .. } => model.implementation_view.view_name(),
        SymbolSourceContract::ExistingSchematicPins { schematic_view, .. } => schematic_view,
        SymbolSourceContract::BlankExplicitContract => return Ok(None),
    };
    if binding.view != implementation_view {
        return Ok(None);
    }

    let mut sheet = definition
        .parameter_form
        .to_property_sheet()
        .map_err(|error| error.to_string())?;
    sheet.add(
        PropertyDefinition::new("name")
            .with_display_name("Reference designator")
            .with_description("Unique instance name emitted at the start of the netlist line.")
            .with_type(PropertyType::String)
            .with_default(PropertyValue::string(""))
            .with_max_length(128)
            .with_order(-10_000)
            .with_category("Identity")
            .required(),
    );
    Ok(Some(sheet))
}

/// Single authority for the schematic Object properties command. Availability
/// requires a live target that the matching editor can actually open.
pub(crate) fn selected_object_properties_available(state: &AppState) -> bool {
    if state.schematic.read_only || state.active_view_read_only() {
        return false;
    }
    if let Some(id) = state.schematic.selection.single_component() {
        return state
            .schematic
            .components
            .iter()
            .find(|component| component.id == id)
            .is_some_and(|component| {
                if component.kind == ComponentType::CellInstance {
                    authoritative_cell_instance_sheet(state, component)
                        .is_ok_and(|sheet| sheet.is_some())
                } else {
                    state.property_registry.get(component.kind).is_some()
                }
            });
    }
    if let Some(id) = state.schematic.selection.single_net_label() {
        return state
            .schematic
            .net_labels
            .iter()
            .any(|label| label.id == id);
    }
    if let Some(id) = state.schematic.selection.single_design_note() {
        return state
            .schematic
            .design_notes
            .iter()
            .any(|note| note.id == id);
    }
    if let Some(id) = state.schematic.selection.single_documentation_shape() {
        return state
            .schematic
            .documentation_shapes
            .iter()
            .any(|shape| shape.id == id);
    }
    if let Some(id) = state.schematic.selection.single_bus_tap() {
        return state.schematic.bus_taps.iter().any(|tap| tap.id == id);
    }
    if let Some(id) = state.schematic.selection.single_bus() {
        return state.schematic.buses.iter().any(|bus| bus.id == id);
    }
    false
}

/// Open the tabbed property editor for `component_id`, populated from the
/// component's registry sheet and current values. Refused on read-only
/// views — the editor is an edit path.
pub(crate) fn open_property_editor(state: &mut AppState, component_id: u64) {
    if state.tabbed_property_dialog.open
        || state.dialogs.object_properties.open
        || state.dialogs.rename_selection.open
    {
        return;
    }
    if state.deny_read_only_edit() {
        return;
    }
    let Some(component) = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
    else {
        return;
    };
    let component_type = component.kind;
    if component_type == ComponentType::CellInstance {
        match authoritative_cell_instance_sheet(state, &component) {
            Ok(Some(sheet)) => {
                if let Err(error) = state.property_registry.install_cell_instance_sheet(sheet) {
                    state.property_registry.clear_cell_instance_sheet();
                    log::warn!("Cannot open model-bound instance properties: {error}");
                    return;
                }
            }
            Ok(None) => {
                state.property_registry.clear_cell_instance_sheet();
                log::warn!("No authoritative property form found for cell instance");
                return;
            }
            Err(error) => {
                state.property_registry.clear_cell_instance_sheet();
                log::warn!("Cannot load model-bound instance properties: {error}");
                return;
            }
        }
    } else {
        state.property_registry.clear_cell_instance_sheet();
    }
    let properties = crate::properties::property_bridge::collect_properties_from_component(
        &component,
        &state.property_registry,
    );
    let component_name = component.name.clone();
    if let Some(sheet) = state.property_registry.get(component_type) {
        state.tabbed_property_dialog.open_for_component(
            component_id,
            component_name,
            component_type,
            sheet,
            properties,
            crate::properties::ComponentPropertySession::new(
                component,
                state.design_execution_epoch,
                state.active_schematic_epoch,
                state.workspace.active_view.display_path(),
            ),
        );
    } else {
        log::warn!("No property sheet found for {:?}", component_type);
    }
}

/// Open the canonical properties workflow for the one selected editable
/// schematic object. Component instances retain their schema-driven editor;
/// buses and taps use the topology-aware generic transaction specified by the
/// workbench mockup.
pub(crate) fn open_selected_object_properties(state: &mut AppState) -> bool {
    if state.tabbed_property_dialog.open
        || state.dialogs.object_properties.open
        || state.dialogs.rename_selection.open
    {
        return false;
    }
    if state.deny_read_only_edit() || !selected_object_properties_available(state) {
        return false;
    }
    if let Some(component_id) = state.schematic.selection.single_component() {
        open_property_editor(state, component_id);
        return state.tabbed_property_dialog.open;
    }
    if let Some(label_id) = state.schematic.selection.single_net_label()
        && let Some(label) = state
            .schematic
            .net_labels
            .iter()
            .find(|label| label.id == label_id)
    {
        state.dialogs.object_properties.open_net_label(
            label,
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.schematic.topology_version(),
            state.workspace.active_view.display_path(),
        );
        return true;
    }
    if let Some(note_id) = state.schematic.selection.single_design_note()
        && let Some(note) = state
            .schematic
            .design_notes
            .iter()
            .find(|note| note.id == note_id)
    {
        state.dialogs.object_properties.open_design_note(
            note,
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.schematic.topology_version(),
            state.workspace.active_view.display_path(),
        );
        return true;
    }
    if let Some(shape_id) = state.schematic.selection.single_documentation_shape()
        && let Some(shape) = state
            .schematic
            .documentation_shapes
            .iter()
            .find(|shape| shape.id == shape_id)
    {
        state.dialogs.object_properties.open_documentation_shape(
            shape,
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.schematic.topology_version(),
            state.workspace.active_view.display_path(),
        );
        return true;
    }
    if let Some(tap_id) = state.schematic.selection.single_bus_tap()
        && let Some(tap) = state.schematic.bus_taps.iter().find(|tap| tap.id == tap_id)
    {
        state.dialogs.object_properties.open_bus_tap(
            tap,
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.schematic.topology_version(),
            state.workspace.active_view.display_path(),
        );
        return true;
    }
    if let Some(bus_id) = state.schematic.selection.single_bus()
        && let Some(bus) = state.schematic.buses.iter().find(|bus| bus.id == bus_id)
    {
        state.dialogs.object_properties.open_bus(
            bus,
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.schematic.topology_version(),
            state.workspace.active_view.display_path(),
        );
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Component, DesignNote,
        DesignNoteKind, GeneratedSymbolViews, Library, LibraryCellInstance,
        ModelBoundSymbolDefinition, NetLabel, ParameterInheritance, Point, PortDirection,
        PropertyType, SymbolElectricalType, SymbolGraphicTemplate, SymbolIdentity,
        SymbolModelReference, SymbolNetlistBinding, SymbolParameterConstraints,
        SymbolParameterDefault, SymbolParameterField, SymbolParameterForm, SymbolParameterSection,
        SymbolParameterVisibility, SymbolPinDefinition, SymbolPinSide, SymbolSourceContract,
    };

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
        definition
            .build_plan(&library)
            .expect("construction plan")
            .commit(&mut library)
            .expect("definition publication");
        state.library_manager.add_library(library);

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
    fn selected_bus_and_tap_open_the_generic_transaction() {
        let mut state = AppState::default();
        let bus = Bus::segment(
            41,
            Point::new(0, 0),
            Point::new(10, 0),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        let tap = BusTap::new(
            42,
            &bus,
            Point::new(5, 0),
            Point::new(5, 5),
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        state.schematic.buses.push(bus);
        state.schematic.bus_taps.push(tap);

        state.schematic.selection.select_only_bus(41);
        assert!(open_selected_object_properties(&mut state));
        assert!(matches!(
            state.dialogs.object_properties.draft,
            Some(super::super::ObjectPropertiesDraft::Bus(_))
        ));
        state.dialogs.object_properties.close();

        state.schematic.selection.select_only_bus_tap(42);
        assert!(open_selected_object_properties(&mut state));
        assert!(matches!(
            state.dialogs.object_properties.draft,
            Some(super::super::ObjectPropertiesDraft::BusTap(_))
        ));
    }

    #[test]
    fn selected_net_label_opens_the_guarded_object_properties_transaction() {
        let mut state = AppState::default();
        let label = NetLabel::new(43, Point::new(-12, 34), "afe.out");
        state.schematic.net_labels.push(label.clone());
        state.schematic.selection.select_only_net_label(label.id);

        assert!(open_selected_object_properties(&mut state));
        assert!(matches!(
            state.dialogs.object_properties.draft,
            Some(super::super::ObjectPropertiesDraft::NetLabel(ref draft))
                if draft.original == label
                    && draft.name == "afe.out"
                    && draft.x == "-12"
                    && draft.y == "34"
        ));
    }

    #[test]
    fn selected_design_note_opens_the_guarded_object_properties_transaction() {
        let mut state = AppState::default();
        let note = DesignNote::new(
            44,
            Point::new(-12, 34),
            DesignNoteKind::ReviewNote,
            "Review bias path",
        )
        .unwrap();
        state.schematic.design_notes.push(note.clone());
        state.schematic.selection.select_only_design_note(note.id);

        assert!(open_selected_object_properties(&mut state));
        assert!(matches!(
            state.dialogs.object_properties.draft,
            Some(super::super::ObjectPropertiesDraft::DesignNote(ref draft))
                if draft.original == note
                    && draft.kind == DesignNoteKind::ReviewNote
                    && draft.text == "Review bias path"
        ));
    }

    #[test]
    fn generic_properties_refuses_read_only_schematics() {
        let mut state = AppState::default();
        state
            .schematic
            .buses
            .push(Bus::segment(1, Point::new(0, 0), Point::new(10, 0), None).unwrap());
        state.schematic.selection.select_only_bus(1);
        state.schematic.read_only = true;

        assert!(!open_selected_object_properties(&mut state));
        assert!(!state.dialogs.object_properties.open);
    }

    #[test]
    fn stale_selected_object_ids_fail_closed_without_opening_a_dialog() {
        let mut state = AppState::default();
        state.schematic.selection.select_only_bus(9001);
        assert!(!open_selected_object_properties(&mut state));
        assert!(!state.dialogs.object_properties.open);

        state.schematic.selection.select_only_bus_tap(9002);
        assert!(!open_selected_object_properties(&mut state));
        assert!(!state.dialogs.object_properties.open);

        state.schematic.selection.select_only_net_label(9003);
        assert!(!open_selected_object_properties(&mut state));
        assert!(!state.dialogs.object_properties.open);

        state.schematic.selection.select_only_design_note(9004);
        assert!(!open_selected_object_properties(&mut state));
        assert!(!state.dialogs.object_properties.open);
    }

    #[test]
    fn a_retained_property_transaction_cannot_be_overwritten_by_another_owner() {
        let mut state = AppState::default();
        let bus = Bus::segment(
            1,
            Point::new(0, 0),
            Point::new(10, 0),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        state.schematic.buses.push(bus);
        state.schematic.selection.select_only_bus(1);
        assert!(open_selected_object_properties(&mut state));

        state.schematic.selection.select_only_bus(999);
        assert!(!open_selected_object_properties(&mut state));
        assert!(matches!(
            state.dialogs.object_properties.draft,
            Some(super::super::ObjectPropertiesDraft::Bus(ref draft)) if draft.original.id == 1
        ));
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
        state.schematic.components.push(instance);
        state.schematic.selection.select_only_component(44);

        assert!(selected_object_properties_available(&state));
        open_property_editor(&mut state, 44);

        assert!(state.tabbed_property_dialog.open);
        assert_eq!(
            state.tabbed_property_dialog.component_type,
            Some(ComponentType::CellInstance)
        );
        assert!(
            state
                .tabbed_property_dialog
                .tabs
                .iter()
                .any(|tab| tab.name == "Geometry" && tab.property_count == 2)
        );
        let sheet = state
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
            state.tabbed_property_dialog.values.get("w"),
            Some(&PropertyValue::Number {
                value: 2e-6,
                unit: Some("m".to_owned()),
            })
        );
        assert!(matches!(
            state.tabbed_property_dialog.values.get("name"),
            Some(PropertyValue::String(name)) if name == "M44"
        ));
    }

    #[test]
    fn legacy_and_review_only_cells_cannot_reuse_a_prior_dynamic_form() {
        let mut state = AppState::default();
        let definition = model_bound_definition();
        let binding = install_definition(&mut state, &definition);
        state.schematic.components.push(
            Component::new(1, ComponentType::CellInstance, Point::origin())
                .with_library_cell(binding)
                .with_name_value("M1", "nmos_core"),
        );
        state.schematic.selection.select_only_component(1);
        open_property_editor(&mut state, 1);
        assert!(state.tabbed_property_dialog.open);
        state.tabbed_property_dialog.close();
        assert!(
            state
                .property_registry
                .get(ComponentType::CellInstance)
                .is_some()
        );

        let mut legacy_library = Library::new("legacy");
        legacy_library.add_cell(crate::state::Cell::new("opaque"));
        state.library_manager.add_library(legacy_library);
        state.schematic.components.push(
            Component::new(2, ComponentType::CellInstance, Point::new(20, 0))
                .with_library_cell(LibraryCellInstance::new("legacy", "opaque", "schematic"))
                .with_name_value("X2", "opaque"),
        );
        state.schematic.selection.select_only_component(2);
        assert!(!selected_object_properties_available(&state));
        open_property_editor(&mut state, 2);
        assert!(!state.tabbed_property_dialog.open);
        assert!(
            state
                .property_registry
                .get(ComponentType::CellInstance)
                .is_none()
        );

        let mut review = model_bound_definition();
        review.identity.cell = "review_only".to_owned();
        review.identity.binding_id = "review-only-v1".to_owned();
        review.source = SymbolSourceContract::BlankExplicitContract;
        review.netlist = SymbolNetlistBinding::unbound();
        let review_binding = install_definition(&mut state, &review);
        state.schematic.components.push(
            Component::new(3, ComponentType::CellInstance, Point::new(40, 0))
                .with_library_cell(review_binding)
                .with_name_value("X3", "review_only"),
        );
        state.schematic.selection.select_only_component(3);
        assert!(!selected_object_properties_available(&state));
        open_property_editor(&mut state, 3);
        assert!(!state.tabbed_property_dialog.open);
        assert!(
            state
                .property_registry
                .get(ComponentType::CellInstance)
                .is_none()
        );
    }
}
