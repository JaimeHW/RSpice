//! Opening the property editor for a schematic component.

use crate::state::{
    Component, ComponentType, MODEL_BOUND_SYMBOL_METADATA_KEY, ModelBoundSymbolDefinition,
    PropertyDefinition, PropertySheet, PropertyType, PropertyValue, SymbolSourceContract,
};
use crate::workbench::app_state::AppState;
use rspice_core::xspice::{CodeModelRegistry, ParamSpec, ParamType};

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
    if binding.is_builtin_xspice() {
        return builtin_xspice_property_sheet(binding).map(Some);
    }
    if binding.is_generated_veriloga() {
        return generated_veriloga_property_sheet(binding).map(Some);
    }
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

/// Exact property contract used by both the modal editor and inline
/// inspector. Dynamic catalog/library cells must never fall back to the
/// generic `CellInstance` registry sheet.
pub(crate) fn authoritative_component_property_sheet(
    state: &AppState,
    component: &Component,
) -> Result<Option<PropertySheet>, String> {
    if component.kind == ComponentType::CellInstance {
        authoritative_cell_instance_sheet(state, component)
    } else {
        Ok(state.property_registry.get(component.kind).cloned())
    }
}

fn builtin_xspice_property_sheet(
    binding: &crate::state::LibraryCellInstance,
) -> Result<PropertySheet, String> {
    let contract = binding
        .builtin_xspice
        .as_ref()
        .ok_or_else(|| "built-in XSPICE binding is missing its executable contract".to_owned())?;
    let descriptor = crate::state::validate_builtin_xspice_binding(binding)?;

    let registry = CodeModelRegistry::with_builtins();
    let model = registry
        .get(&contract.model_type)
        .ok_or_else(|| format!("XSPICE model '{}' is unavailable", contract.model_type))?;
    let mut sheet = cell_instance_identity_sheet();
    for (index, parameter) in model.parameters().iter().enumerate() {
        let (property_type, mut default_value) = xspice_parameter_property(parameter);
        if parameter.required {
            default_value = match parameter.param_type {
                ParamType::String | ParamType::StringVector => PropertyValue::string(""),
                _ => PropertyValue::expression(""),
            };
        }
        // The label is the model's exact parameter key: it is what the user
        // writes on the `.MODEL` card, so a derived prose label would only
        // introduce a second name for the same thing.
        let mut definition = PropertyDefinition::new(&parameter.name)
            .with_description(if parameter.description.trim().is_empty() {
                format!(
                    "{} model parameter ({:?}).",
                    descriptor.display_name, parameter.param_type
                )
            } else {
                parameter.description.clone()
            })
            .with_type(property_type)
            .with_default(default_value)
            .with_max_length(512)
            .with_order(index as i32)
            .with_category("Model parameters");
        if !parameter.min_is_soft {
            definition.min_value = parameter.min;
        }
        if !parameter.max_is_soft {
            definition.max_value = parameter.max;
        }
        if parameter.required {
            definition = definition.required();
        }
        sheet.add(definition);
    }
    Ok(sheet)
}

fn generated_veriloga_property_sheet(
    binding: &crate::state::LibraryCellInstance,
) -> Result<PropertySheet, String> {
    let descriptor = crate::state::validate_generated_veriloga_binding(binding)?;
    let mut sheet = cell_instance_identity_sheet();
    for (index, parameter) in descriptor.parameters.iter().enumerate() {
        let scope = generated_veriloga_parameter_scope_label(parameter.scope);
        let default = parameter
            .default
            .map(|value| {
                if parameter.is_integer {
                    PropertyValue::expression((value as i64).to_string())
                } else {
                    PropertyValue::expression(value.to_string())
                }
            })
            .unwrap_or_else(|| PropertyValue::expression(""));
        let mut description = format!(
            "Generated Verilog-A {scope} parameter for {}.",
            descriptor.model_name
        );
        if parameter.has_dynamic_constraints {
            description.push_str(
                " Its final range depends on other parameters; the compiled model validates the complete set.",
            );
        }
        if !parameter.aliases.is_empty() {
            description.push_str(&format!(" Aliases: {}.", parameter.aliases.join(", ")));
        }
        // As with code models, the compiled Verilog-A parameter key is the
        // name the deck and the compiled module both use.
        let mut definition = PropertyDefinition::new(parameter.name)
            .with_description(description)
            .with_type(PropertyType::Expression)
            .with_default(default)
            .with_max_length(512)
            .with_order(index as i32)
            .with_category(if scope == "model" {
                "Model parameters"
            } else {
                "Instance parameters"
            });
        if !parameter.has_dynamic_constraints {
            definition.min_value = parameter
                .minimum
                .filter(|bound| !bound.exclusive)
                .map(|bound| bound.value);
            definition.max_value = parameter
                .maximum
                .filter(|bound| !bound.exclusive)
                .map(|bound| bound.value);
        }
        sheet.add(definition);
    }
    Ok(sheet)
}

const fn generated_veriloga_parameter_scope_label(
    scope: rspice_core::device::veriloga_builtins::GeneratedVerilogAParameterScope,
) -> &'static str {
    match scope {
        rspice_core::device::veriloga_builtins::GeneratedVerilogAParameterScope::Model => "model",
        rspice_core::device::veriloga_builtins::GeneratedVerilogAParameterScope::Instance => {
            "instance"
        }
        rspice_core::device::veriloga_builtins::GeneratedVerilogAParameterScope::Dual => {
            "model/instance"
        }
    }
}

fn xspice_parameter_property(parameter: &ParamSpec) -> (PropertyType, PropertyValue) {
    match parameter.param_type {
        ParamType::Real => (
            PropertyType::Expression,
            PropertyValue::expression(parameter.default.to_string()),
        ),
        ParamType::Integer => (
            PropertyType::Expression,
            PropertyValue::expression((parameter.default as i64).to_string()),
        ),
        ParamType::Boolean => (
            PropertyType::Boolean,
            PropertyValue::boolean(parameter.default != 0.0),
        ),
        ParamType::Complex => {
            let value = parameter.complex_default.unwrap_or_default();
            (
                PropertyType::Expression,
                PropertyValue::expression(format!("<{} {}>", value.re, value.im)),
            )
        }
        ParamType::String => (
            PropertyType::String,
            PropertyValue::string(parameter.string_default.clone().unwrap_or_default()),
        ),
        ParamType::StringVector => (
            PropertyType::String,
            PropertyValue::string(format_string_vector(
                parameter
                    .string_vector_default
                    .as_deref()
                    .unwrap_or_default(),
            )),
        ),
        ParamType::RealVector => (
            PropertyType::Expression,
            PropertyValue::expression(format_numeric_vector(
                parameter.real_vector_default.as_deref().unwrap_or_default(),
            )),
        ),
        ParamType::IntegerVector => (
            PropertyType::Expression,
            PropertyValue::expression(format_numeric_vector(
                parameter
                    .integer_vector_default
                    .as_deref()
                    .unwrap_or_default(),
            )),
        ),
        ParamType::ComplexVector => (
            PropertyType::Expression,
            PropertyValue::expression(format_complex_vector(
                parameter
                    .complex_vector_default
                    .as_deref()
                    .unwrap_or_default(),
            )),
        ),
    }
}

fn format_numeric_vector<T: std::fmt::Display>(values: &[T]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn format_string_vector(values: &[String]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| format!("\"{}\"", value.replace('"', "\\\"")))
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn format_complex_vector(values: &[rspice_core::Complex64]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| format!("<{} {}>", value.re, value.im))
            .collect::<Vec<_>>()
            .join(" ")
    )
}

/// Identity-only contract for legacy, hierarchical, and review-only cell
/// instances whose master does not publish an executable typed parameter
/// form. The editor remains fully functional for the one property the
/// instance itself owns and preserves every opaque master parameter without
/// presenting guessed controls.
fn cell_instance_identity_sheet() -> PropertySheet {
    let mut sheet = PropertySheet::new();
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
    sheet
}

/// Single authority for the schematic Object properties command. Availability
/// requires a live target that the matching editor can actually open.
pub(crate) fn selected_object_properties_available(state: &AppState) -> bool {
    if let Some(id) = state.schematic.selection.single_probe() {
        return state.schematic.probes.iter().any(|probe| probe.id == id);
    }
    if state.schematic_edit_read_only() {
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
                    authoritative_cell_instance_sheet(state, component).is_ok()
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
    if state.schematic.selection.has_any_wire_selection() {
        return crate::workbench::app::selected_named_net_target(state).is_some();
    }
    false
}

/// Open the schematic component editor for `component_id`, populated from the
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
                if let Err(error) = state
                    .property_registry
                    .install_cell_instance_sheet(cell_instance_identity_sheet())
                {
                    state.property_registry.clear_cell_instance_sheet();
                    log::warn!("Cannot open cell-instance identity properties: {error}");
                    return;
                }
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
/// buses, taps, and wire-resolved named nets use the topology-aware generic
/// transaction specified by the workbench mockup.
pub(crate) fn open_selected_object_properties(state: &mut AppState) -> bool {
    if state.tabbed_property_dialog.open
        || state.dialogs.object_properties.open
        || state.dialogs.rename_selection.open
    {
        return false;
    }
    if let Some(id) = state.schematic.selection.single_probe() {
        if state.schematic.probes.iter().all(|probe| probe.id != id) {
            return false;
        }
        state.workbench.inspector_visible = true;
        state.workbench.drawer = Some(crate::workbench::state::Drawer::Inspector);
        return true;
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
    if state.schematic.selection.has_any_wire_selection()
        && let Some(target) = crate::workbench::app::selected_named_net_target(state)
    {
        state.dialogs.object_properties.open_named_net(
            target,
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
        PropertyType, SchematicProbe, SymbolElectricalType, SymbolGraphicTemplate, SymbolIdentity,
        SymbolModelReference, SymbolNetlistBinding, SymbolParameterConstraints,
        SymbolParameterDefault, SymbolParameterField, SymbolParameterForm, SymbolParameterSection,
        SymbolParameterVisibility, SymbolPinDefinition, SymbolPinSide, SymbolSourceContract, Wire,
    };

    #[test]
    fn generated_veriloga_scope_labels_cover_dual_storage() {
        use rspice_core::device::veriloga_builtins::GeneratedVerilogAParameterScope;

        assert_eq!(
            generated_veriloga_parameter_scope_label(GeneratedVerilogAParameterScope::Model),
            "model"
        );
        assert_eq!(
            generated_veriloga_parameter_scope_label(GeneratedVerilogAParameterScope::Instance),
            "instance"
        );
        assert_eq!(
            generated_veriloga_parameter_scope_label(GeneratedVerilogAParameterScope::Dual),
            "model/instance"
        );
    }

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
            Some(crate::workbench::app::ObjectPropertiesDraft::Bus(_))
        ));
        state.dialogs.object_properties.close();

        state.schematic.selection.select_only_bus_tap(42);
        assert!(open_selected_object_properties(&mut state));
        assert!(matches!(
            state.dialogs.object_properties.draft,
            Some(crate::workbench::app::ObjectPropertiesDraft::BusTap(_))
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
            Some(crate::workbench::app::ObjectPropertiesDraft::NetLabel(ref draft))
                if draft.original == label
                    && draft.name == "afe.out"
                    && draft.x == "-12"
                    && draft.y == "34"
        ));
    }

    #[test]
    fn selected_named_wire_opens_net_scoped_properties_and_unnamed_wire_fails_closed() {
        let mut state = AppState::default();
        state
            .schematic
            .wires
            .push(Wire::new(71, vec![Point::new(0, 0), Point::new(40, 0)]));
        state
            .schematic
            .net_labels
            .push(NetLabel::new(72, Point::new(20, 0), "sense"));
        state.schematic.selection.select_only_wire(71);

        assert!(selected_object_properties_available(&state));
        assert!(open_selected_object_properties(&mut state));
        assert!(matches!(
            state.dialogs.object_properties.draft,
            Some(crate::workbench::app::ObjectPropertiesDraft::NamedNet(ref draft))
                if draft.original.name == "sense"
                    && draft.original.labels.iter().map(|label| label.id).collect::<Vec<_>>() == [72]
                    && draft.name == "sense"
        ));

        state.dialogs.object_properties.close();
        state.schematic.net_labels.clear();
        assert!(!selected_object_properties_available(&state));
        assert!(!open_selected_object_properties(&mut state));
    }

    #[test]
    fn interface_port_object_properties_preserve_the_full_typed_editor() {
        let mut state = AppState::default();
        state.schematic.components.push(
            Component::new(73, ComponentType::Port, Point::origin()).with_name_value("P73", "VIN"),
        );
        state.schematic.selection.select_only_component(73);

        assert!(selected_object_properties_available(&state));
        assert!(open_selected_object_properties(&mut state));
        assert!(state.tabbed_property_dialog.open);
        assert!(!state.dialogs.object_properties.open);
        assert_eq!(
            state.tabbed_property_dialog.component_type,
            Some(ComponentType::Port)
        );
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
            Some(crate::workbench::app::ObjectPropertiesDraft::DesignNote(ref draft))
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
    fn selected_probe_routes_to_the_inspector_even_in_read_only_mode() {
        let mut state = AppState::default();
        state.schematic.probes.push(
            SchematicProbe::new(74, Point::new(10, 20), "V(out)", Some("V(out)".to_owned()))
                .unwrap(),
        );
        state.schematic.selection.select_only_probe(74);
        state.schematic.read_only = true;

        assert!(selected_object_properties_available(&state));
        assert!(open_selected_object_properties(&mut state));
        assert!(state.workbench.inspector_visible);
        assert_eq!(
            state.workbench.drawer,
            Some(crate::workbench::state::Drawer::Inspector)
        );
        assert!(!state.dialogs.object_properties.open);
        assert!(!state.tabbed_property_dialog.open);
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
            Some(crate::workbench::app::ObjectPropertiesDraft::Bus(ref draft)) if draft.original.id == 1
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
    fn every_static_component_family_opens_its_registered_editor() {
        for (index, kind) in ComponentType::ALL
            .into_iter()
            .filter(|kind| *kind != ComponentType::CellInstance)
            .enumerate()
        {
            let mut state = AppState::default();
            state.schematic.components.clear();
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
            state.schematic.components.push(component);

            open_property_editor(&mut state, index as u64 + 1);

            assert!(
                state.tabbed_property_dialog.open,
                "{} did not open a component editor",
                kind.display_name()
            );
            assert_eq!(state.tabbed_property_dialog.component_type, Some(kind));
        }
    }

    #[test]
    fn legacy_and_review_only_cells_get_isolated_identity_forms() {
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
        assert!(selected_object_properties_available(&state));
        open_property_editor(&mut state, 2);
        assert!(state.tabbed_property_dialog.open);
        let legacy_sheet = state
            .property_registry
            .get(ComponentType::CellInstance)
            .expect("legacy identity form");
        assert!(legacy_sheet.get("name").is_some());
        assert!(legacy_sheet.get("w").is_none());
        state.tabbed_property_dialog.close();

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
        assert!(selected_object_properties_available(&state));
        open_property_editor(&mut state, 3);
        assert!(state.tabbed_property_dialog.open);
        let review_sheet = state
            .property_registry
            .get(ComponentType::CellInstance)
            .expect("review identity form");
        assert!(review_sheet.get("name").is_some());
        assert!(review_sheet.get("w").is_none());
    }
}
