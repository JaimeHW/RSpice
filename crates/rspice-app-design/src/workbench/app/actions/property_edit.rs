//! Opening the property editor for a schematic component.

use crate::properties::{PropertyEditorSchema, TabbedPropertyDialogState};
use crate::simulation::plan::AnalysisDraft;
use crate::simulation::stimulus_realize::PreviewTiming;
use crate::state::{
    Component, ComponentType, MODEL_BOUND_SYMBOL_METADATA_KEY, ModelBoundSymbolDefinition,
    PropertyDefinition, PropertySheet, PropertyType, PropertyValue, SymbolSourceContract,
};
use crate::workbench::app::actions::workspace::{
    commit_pending_inspector_edit, deny_read_only_edit,
};
use crate::workbench::app::dialogs::object_properties::ObjectPropertiesDialogState;
use crate::workbench::app_state::Model;
use crate::workbench::app_state::model::{
    binds_model_card, cell_instance_identity_sheet, compiled_parameter_sheet,
    instance_multiplicity_definition, states_compiled_parameters,
};
use crate::workbench::state::WorkbenchSession;
use rspice_core::xspice::{CodeModelRegistry, ParamSpec, ParamType};

/// Resolve the exact persistent parameter contract owned by a placed cell's
/// library master. Returning `None` is intentional for legacy and explicitly
/// unbound cells: they must not inherit whichever dynamic sheet was opened
/// most recently.
fn authoritative_cell_instance_sheet(
    model: &Model,
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
    if binding.is_builtin_functional() {
        return functional_property_sheet(binding).map(Some);
    }
    if binding.is_generated_veriloga() {
        return generated_veriloga_property_sheet(binding).map(Some);
    }
    let Some(cell) = model
        .library_manager
        .get_library(&binding.library)
        .and_then(|library| library.get_cell(&binding.cell))
    else {
        return Ok(None);
    };
    let bound_view = cell.get_view(&binding.view);
    // A Verilog-A view whose compile stated its module's parameters: one
    // typed field each, read from the contract the netlister binds.
    if let Some(view) = bound_view.filter(|view| states_compiled_parameters(cell, view)) {
        return compiled_parameter_sheet(cell, view, binds_model_card(component)).map(Some);
    }

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
            let Some(candidate) =
                crate::state::load_model_bound_symbol(view).map_err(|error| error.to_string())?
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
        return cell_contract_property_sheet(cell, bound_view).map(Some);
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

/// Property contract for a placed cell whose master publishes no versioned
/// symbol definition: the parameters the cellview declares, plus the
/// multiplicity the instance line carries.
///
/// The multiplicity row is unconditional. `m` on an X line is the flattener's
/// physical multiplier rather than a subcircuit parameter, so leaving it to
/// free-text parameter entry is the one spelling mistake that silently stops
/// multiplying anything. A cell that declares nothing still owns it.
fn cell_contract_property_sheet(
    cell: &crate::state::Cell,
    view: Option<&crate::state::View>,
) -> Result<PropertySheet, String> {
    let mut sheet = cell_instance_identity_sheet();
    sheet.add(instance_multiplicity_definition());
    for (index, parameter) in rspice_design::library::metadata::cell_parameter_contract(cell, view)?
        .into_iter()
        .enumerate()
    {
        let mut description = format!("Instance override for the {} parameter.", parameter.name);
        if !parameter.aliases.is_empty() {
            description.push_str(&format!(" Aliases: {}.", parameter.aliases.join(", ")));
        }
        let mut definition = PropertyDefinition::new(&parameter.name)
            .with_description(description)
            .with_type(PropertyType::Expression)
            .with_default(PropertyValue::expression(
                parameter.default_value.unwrap_or_default(),
            ))
            .with_max_length(512)
            .with_order(index as i32)
            .with_category("Cell parameters");
        if parameter.required {
            definition = definition.required();
        }
        sheet.add(definition);
    }
    Ok(sheet)
}

/// Exact property contract used by both the modal editor and inline
/// inspector. Dynamic catalog/library cells must never fall back to the
/// generic `CellInstance` registry sheet.
pub fn authoritative_component_property_sheet(
    registry: &crate::properties::PropertyEditorSchema,
    model: &Model,
    component: &Component,
) -> Result<Option<PropertySheet>, String> {
    if component.kind == ComponentType::CellInstance {
        authoritative_cell_instance_sheet(model, component)
    } else {
        Ok(registry.get(component.kind).cloned())
    }
}

/// Resolve dynamic contracts without replacing the active editor's sheet.
pub(crate) fn component_property_findings(
    registry: &crate::properties::PropertyEditorSchema,
    model: &Model,
    component: &Component,
) -> Vec<(String, String)> {
    use crate::properties::property_bridge::validate_component_properties;

    if component.kind != ComponentType::CellInstance {
        return validate_component_properties(component, registry.get(component.kind));
    }
    let sheet = match authoritative_cell_instance_sheet(model, component) {
        Ok(sheet) => sheet.unwrap_or_else(cell_instance_identity_sheet),
        Err(error) => return vec![("schema".to_owned(), error)],
    };
    validate_component_properties(component, Some(&sheet))
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
                    "{} model parameter ({}).",
                    descriptor.display_name,
                    match parameter.param_type {
                        ParamType::Real => "real",
                        ParamType::Integer => "integer",
                        ParamType::Boolean => "boolean",
                        ParamType::Complex => "complex",
                        ParamType::String => "string",
                        ParamType::StringVector => "string vector",
                        ParamType::RealVector => "real vector",
                        ParamType::IntegerVector => "integer vector",
                        ParamType::ComplexVector => "complex vector",
                    }
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

/// A functional part's sheet: how many inputs it draws and its preset first,
/// where it has them, then one row per subcircuit parameter, labelled in
/// words, with its unit and the subcircuit's own default. A selector offers
/// its named settings.
fn functional_property_sheet(
    binding: &crate::state::LibraryCellInstance,
) -> Result<PropertySheet, String> {
    let descriptor = crate::state::validate_builtin_functional_binding(binding)?;
    let mut sheet = cell_instance_identity_sheet();
    if let Some(inputs) = descriptor.inputs {
        let counts = (inputs.fewest..=inputs.most)
            .map(|count| count.to_string())
            .collect::<Vec<_>>();
        sheet.add(
            PropertyDefinition::new(crate::state::FUNCTIONAL_INPUTS_SETTING)
                .with_display_name("Inputs")
                .with_description(
                    "How many inputs the gate draws. Inputs it leaves off are tied to ground, which the gate ignores.",
                )
                .with_type(PropertyType::Enum)
                .with_default(PropertyValue::enumeration(counts[0].clone(), counts))
                .with_order(-2)
                .with_category("Parameters"),
        );
    }
    if let Some(preset) = descriptor.preset {
        let labels = preset
            .choices
            .iter()
            .map(|choice| choice.label.to_owned())
            .collect::<Vec<_>>();
        let presets = preset
            .choices
            .iter()
            .map(|choice| {
                let values = choice
                    .values
                    .iter()
                    .map(|(name, value)| ((*name).to_owned(), PropertyValue::expression(*value)))
                    .collect();
                (choice.label.to_owned(), values)
            })
            .collect();
        sheet.add(
            PropertyDefinition::new(preset.name)
                .with_display_name(preset.label)
                .with_description(preset.description)
                .with_type(PropertyType::Enum)
                .with_default(PropertyValue::enumeration(labels[0].clone(), labels))
                .with_presets(presets)
                .with_order(-1)
                .with_category("Parameters"),
        );
    }
    for (index, parameter) in descriptor.parameters.iter().enumerate() {
        let (property_type, default) = if parameter.choices.is_empty() {
            (
                PropertyType::Expression,
                PropertyValue::expression(parameter.default),
            )
        } else {
            (
                PropertyType::Enum,
                PropertyValue::enumeration(
                    parameter.default,
                    parameter
                        .choices
                        .iter()
                        .map(|choice| choice.label.to_owned())
                        .collect(),
                ),
            )
        };
        let mut definition = PropertyDefinition::new(parameter.name)
            .with_display_name(parameter.label)
            .with_description(parameter.description)
            .with_type(property_type)
            .with_default(default)
            .with_max_length(512)
            .with_order(index as i32)
            .with_category("Parameters");
        if !parameter.unit.is_empty() {
            definition = definition.with_unit(parameter.unit);
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
        sheet.add(generated_veriloga_parameter_definition(
            parameter,
            index,
            descriptor.model_name,
        ));
    }
    Ok(sheet)
}

/// One built-in Verilog-A parameter's row, with the units and description its
/// source declares.
fn generated_veriloga_parameter_definition(
    parameter: &rspice_core::device::veriloga_builtins::GeneratedVerilogAParameterDescriptor,
    index: usize,
    model_name: &str,
) -> PropertyDefinition {
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
    // The source's own `desc` where it gives one.
    let mut description = if parameter.description.is_empty() {
        format!("Built-in Verilog-A {scope} parameter of {model_name}.")
    } else {
        parameter.description.to_owned()
    };
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
    if !parameter.units.is_empty() {
        definition.unit = Some(parameter.units.to_owned());
    }
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
    definition
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

/// Single authority for the schematic Object properties command, over the
/// property schema and the model. Availability requires a live target that the
/// matching editor can actually open.
pub fn selected_object_properties_available_in(
    registry: &PropertyEditorSchema,
    model: &Model,
) -> bool {
    if let Some(id) = model.schematic.session.editor.selection.single_probe() {
        return model
            .schematic
            .document()
            .probes
            .iter()
            .any(|probe| probe.id == id);
    }
    if model.schematic_edit_read_only() {
        return false;
    }
    if let Some(id) = model.schematic.session.editor.selection.single_component() {
        return model
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == id)
            .is_some_and(|component| {
                if component.kind == ComponentType::CellInstance {
                    authoritative_cell_instance_sheet(model, component).is_ok()
                } else {
                    registry.get(component.kind).is_some()
                }
            });
    }
    if let Some(id) = model.schematic.session.editor.selection.single_net_label() {
        return model
            .schematic
            .document()
            .net_labels
            .iter()
            .any(|label| label.id == id);
    }
    if let Some(id) = model
        .schematic
        .session
        .editor
        .selection
        .single_design_note()
    {
        return model
            .schematic
            .document()
            .design_notes
            .iter()
            .any(|note| note.id == id);
    }
    if let Some(id) = model
        .schematic
        .session
        .editor
        .selection
        .single_documentation_shape()
    {
        return model
            .schematic
            .document()
            .documentation_shapes
            .iter()
            .any(|shape| shape.id == id);
    }
    if let Some(id) = model.schematic.session.editor.selection.single_bus_tap() {
        return model
            .schematic
            .document()
            .bus_taps
            .iter()
            .any(|tap| tap.id == id);
    }
    if let Some(id) = model.schematic.session.editor.selection.single_bus() {
        return model
            .schematic
            .document()
            .buses
            .iter()
            .any(|bus| bus.id == id);
    }
    if model
        .schematic
        .session
        .editor
        .selection
        .has_any_wire_selection()
    {
        return crate::workbench::app::selected_named_net_target(model).is_some();
    }
    false
}

/// What opening a properties form reads and writes of the design feature's
/// state: the component editor, the generic form, the schema they draw
/// from, and whether the rename form holds the selection.
pub struct PropertyFormParts<'a> {
    pub(crate) tabbed_property_dialog: &'a mut TabbedPropertyDialogState,
    pub(crate) object_properties: &'a mut ObjectPropertiesDialogState,
    pub(crate) rename_open: bool,
    pub(crate) property_registry: &'a mut PropertyEditorSchema,
}

/// Open the schematic component editor for `component_id`, populated from the
/// component's registry sheet and current values, over the forms, the model
/// and the session. Refused on read-only views — the editor is an edit path.
pub fn open_property_editor_in(
    parts: &mut PropertyFormParts<'_>,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    component_id: u64,
) {
    if parts.tabbed_property_dialog.open || parts.object_properties.open || parts.rename_open {
        return;
    }
    if !commit_pending_inspector_edit(model, wb) {
        return;
    }
    if deny_read_only_edit(model, wb) {
        return;
    }
    let Some(component) = model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
    else {
        return;
    };
    let component_type = component.kind;
    if component_type == ComponentType::CellInstance {
        match authoritative_cell_instance_sheet(model, &component) {
            Ok(Some(sheet)) => {
                if let Err(error) = parts.property_registry.install_cell_instance_sheet(sheet) {
                    parts.property_registry.clear_cell_instance_sheet();
                    log::warn!("Cannot open model-bound instance properties: {error}");
                    return;
                }
            }
            Ok(None) => {
                if let Err(error) = parts
                    .property_registry
                    .install_cell_instance_sheet(cell_instance_identity_sheet())
                {
                    parts.property_registry.clear_cell_instance_sheet();
                    log::warn!("Cannot open cell-instance identity properties: {error}");
                    return;
                }
            }
            Err(error) => {
                parts.property_registry.clear_cell_instance_sheet();
                log::warn!("Cannot load model-bound instance properties: {error}");
                return;
            }
        }
    } else {
        parts.property_registry.clear_cell_instance_sheet();
    }
    let properties = crate::properties::property_bridge::collect_properties_from_component(
        &component,
        parts.property_registry,
    );
    let component_name = component.name.clone();
    if let Some(sheet) = parts.property_registry.get(component_type) {
        parts.tabbed_property_dialog.open_for_component(
            component_id,
            component_name,
            component_type,
            sheet,
            properties,
            crate::properties::ComponentPropertySession::new(
                component,
                crate::workbench::app::schematic_editor_request_source(&*model),
                model.workspace.content.active_view.display_path(),
            )
            .with_data_root(
                model
                    .workspace
                    .content
                    .project
                    .data_root()
                    .map(std::path::Path::to_path_buf),
            )
            .with_preview_timing(stimulus_preview_timing(model)),
        );
    } else {
        log::warn!("No property sheet found for {:?}", component_type);
    }
}

/// The transient the component editor's stimulus preview evaluates against.
///
/// Every omitted waveform field resolves against a stop time, so a preview has
/// to name one. The plan's first enabled transient is that one; a plan without
/// a transient yet gets the setup's own defaults, which is what a run would use
/// if the user added one and changed nothing, and the card says the numbers came
/// from there rather than from an analysis. The plan's compatibility dialect
/// and the program a drawing came from decide whose defaults fill the fields a
/// card leaves out, as they do for the run.
pub(in crate::workbench) fn stimulus_preview_timing(model: &Model) -> PreviewTiming {
    let authored = model.sim_setup.analysis_plan.as_ref().and_then(|plan| {
        plan.instances()
            .iter()
            .filter(|instance| instance.enabled())
            .find_map(|instance| match instance.draft() {
                AnalysisDraft::Transient(setup) => Some(setup.clone()),
                _ => None,
            })
    });
    let from_analysis = authored.is_some();
    let setup = authored.unwrap_or_default();
    let seconds = |text: &str| crate::quantity::parse_engineering_value(text).ok();
    let default = PreviewTiming::default();
    let options = &model.sim_setup.options;
    PreviewTiming {
        tstep: seconds(&setup.step)
            .filter(|step| step.is_finite() && *step > 0.0)
            .unwrap_or(default.tstep),
        tstop: seconds(&setup.stop)
            .filter(|stop| stop.is_finite() && *stop > 0.0)
            .unwrap_or(default.tstop),
        from_analysis,
        rules: rspice_core::netlist::SourceRules::new(
            options.compatibility.core_override().unwrap_or_default(),
            options.source_simulator.core_source(),
        ),
    }
}

/// Open the canonical properties workflow for the one selected editable
/// schematic object, over the forms, the model and the session. Component
/// instances retain their schema-driven editor; buses, taps, and wire-resolved
/// named nets use the topology-aware generic transaction specified by the
/// workbench mockup.
pub fn open_selected_object_properties_in(
    parts: &mut PropertyFormParts<'_>,
    model: &mut Model,
    wb: &mut WorkbenchSession,
) -> bool {
    if parts.tabbed_property_dialog.open || parts.object_properties.open || parts.rename_open {
        return false;
    }
    if let Some(id) = model.schematic.session.editor.selection.single_probe() {
        if model
            .schematic
            .document()
            .probes
            .iter()
            .all(|probe| probe.id != id)
        {
            return false;
        }
        wb.inspector_visible = true;
        wb.drawer = Some(crate::workbench::state::Drawer::Inspector);
        return true;
    }
    if deny_read_only_edit(model, wb)
        || !selected_object_properties_available_in(parts.property_registry, model)
    {
        return false;
    }
    if let Some(component_id) = model.schematic.session.editor.selection.single_component() {
        open_property_editor_in(parts, model, wb, component_id);
        return parts.tabbed_property_dialog.open;
    }
    let source = crate::workbench::app::schematic_editor_request_source(&*model);
    if let Some(label_id) = model.schematic.session.editor.selection.single_net_label()
        && let Some(label) = model
            .schematic
            .document()
            .net_labels
            .iter()
            .find(|label| label.id == label_id)
    {
        parts.object_properties.open_net_label(
            label,
            source,
            model.workspace.content.active_view.display_path(),
        );
        return true;
    }
    if model
        .schematic
        .session
        .editor
        .selection
        .has_any_wire_selection()
        && let Some(target) = crate::workbench::app::selected_named_net_target(model)
    {
        parts.object_properties.open_named_net(
            target,
            source,
            model.workspace.content.active_view.display_path(),
        );
        return true;
    }
    if let Some(note_id) = model
        .schematic
        .session
        .editor
        .selection
        .single_design_note()
        && let Some(note) = model
            .schematic
            .document()
            .design_notes
            .iter()
            .find(|note| note.id == note_id)
    {
        parts.object_properties.open_design_note(
            note,
            source,
            model.workspace.content.active_view.display_path(),
        );
        return true;
    }
    if let Some(shape_id) = model
        .schematic
        .session
        .editor
        .selection
        .single_documentation_shape()
        && let Some(shape) = model
            .schematic
            .document()
            .documentation_shapes
            .iter()
            .find(|shape| shape.id == shape_id)
    {
        parts.object_properties.open_documentation_shape(
            shape,
            source,
            model.workspace.content.active_view.display_path(),
        );
        return true;
    }
    if let Some(tap_id) = model.schematic.session.editor.selection.single_bus_tap()
        && let Some(tap) = model
            .schematic
            .document()
            .bus_taps
            .iter()
            .find(|tap| tap.id == tap_id)
    {
        parts.object_properties.open_bus_tap(
            tap,
            source,
            model.workspace.content.active_view.display_path(),
        );
        return true;
    }
    if let Some(bus_id) = model.schematic.session.editor.selection.single_bus()
        && let Some(bus) = model
            .schematic
            .document()
            .buses
            .iter()
            .find(|bus| bus.id == bus_id)
    {
        parts.object_properties.open_bus(
            bus,
            source,
            model.workspace.content.active_view.display_path(),
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
        DesignNoteKind, NetLabel, Point, SchematicProbe, Wire,
    };
    use crate::workbench::design::test_host::DesignTestHost;

    /// The preview's fallback timing names the plan's default transient, and
    /// the type that owns that default lives above the layer the preview does,
    /// so the two numbers are pinned together here — where both are in scope.
    #[test]
    fn the_preview_fallback_is_the_plans_own_default_transient() {
        let setup = crate::workbench::app_state::TranSetup::default();
        let default = PreviewTiming::default();

        assert_eq!(
            crate::quantity::parse_engineering_value(&setup.step).ok(),
            Some(default.tstep)
        );
        assert_eq!(
            crate::quantity::parse_engineering_value(&setup.stop).ok(),
            Some(default.tstop)
        );
        assert!(!default.from_analysis);
    }

    #[test]
    fn the_preview_reads_the_plans_transient_and_falls_back_when_there_is_none() {
        let mut state = DesignTestHost::default();
        let planned = stimulus_preview_timing(&state.model);
        assert!(
            planned.from_analysis,
            "a new project's plan opens with a transient in it"
        );

        state.model.sim_setup.analysis_plan = None;
        let fallback = stimulus_preview_timing(&state.model);
        assert_eq!(fallback, PreviewTiming::default());
        assert!(!fallback.from_analysis);
    }

    /// The preview reads omitted source fields as the plan's run will: under
    /// its compatibility dialect, with the defaults of the program a drawing
    /// came from.
    #[test]
    fn the_preview_reads_sources_by_the_plans_compatibility_and_source_program() {
        use rspice_simulation_contract::options::{SimulationCompatibility, SourceSimulator};
        let mut state = AppState::default();
        assert_eq!(
            stimulus_preview_timing(&state.model).rules,
            rspice_core::netlist::SourceRules::default()
        );
        state.model.sim_setup.options.compatibility = SimulationCompatibility::Ngspice;
        state.model.sim_setup.options.source_simulator = SourceSimulator::Ltspice;
        assert_eq!(
            stimulus_preview_timing(&state.model).rules,
            rspice_core::netlist::SourceRules::new(
                rspice_core::config::SpiceDialect::Ngspice,
                Some(rspice_core::netlist::SourceSimulator::Ltspice),
            )
        );
    }

    /// A built-in Verilog-A parameter's row carries the units and description
    /// its source declares.
    #[test]
    fn a_generated_veriloga_parameter_row_reads_its_source_units_and_description() {
        use rspice_core::device::veriloga_builtins::GeneratedVerilogAParameterDescriptor;

        let described = GeneratedVerilogAParameterDescriptor::model("rth", None)
            .described("K/W", "Thermal resistance");
        let row = generated_veriloga_parameter_definition(&described, 0, "bjt505t_va");
        assert_eq!(row.description, "Thermal resistance");
        assert_eq!(row.unit.as_deref(), Some("K/W"));

        let bare = GeneratedVerilogAParameterDescriptor::instance("w", None);
        let row = generated_veriloga_parameter_definition(&bare, 1, "bjt505t_va");
        assert_eq!(
            row.description,
            "Built-in Verilog-A instance parameter of bjt505t_va."
        );
        assert_eq!(row.unit, None);
    }

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

    #[test]
    fn selected_bus_and_tap_open_the_generic_transaction() {
        let mut state = DesignTestHost::default();
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
        state
            .model
            .schematic
            .document_mut_for_test()
            .buses
            .push(bus);
        state
            .model
            .schematic
            .document_mut_for_test()
            .bus_taps
            .push(tap);

        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_bus(41);
        assert!(open_selected_object_properties_in(
            &mut state.design.property_form_parts(),
            &mut state.model,
            &mut state.workbench,
        ));
        assert_eq!(
            state.design.object_properties.source,
            Some(crate::workbench::app::schematic_editor_request_source(
                &state
            ))
        );
        assert!(matches!(
            state.design.object_properties.draft,
            Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::Bus(_))
        ));
        state.design.object_properties.close();

        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_bus_tap(42);
        assert!(open_selected_object_properties_in(
            &mut state.design.property_form_parts(),
            &mut state.model,
            &mut state.workbench,
        ));
        assert_eq!(
            state.design.object_properties.source,
            Some(crate::workbench::app::schematic_editor_request_source(
                &state
            ))
        );
        assert!(matches!(
            state.design.object_properties.draft,
            Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::BusTap(_))
        ));
    }

    #[test]
    fn selected_net_label_opens_the_guarded_object_properties_transaction() {
        let mut state = DesignTestHost::default();
        let label = NetLabel::new(43, Point::new(-12, 34), "afe.out");
        state
            .model
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(label.clone());
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_net_label(label.id);

        assert!(open_selected_object_properties_in(
            &mut state.design.property_form_parts(),
            &mut state.model,
            &mut state.workbench,
        ));
        assert_eq!(
            state.design.object_properties.source,
            Some(crate::workbench::app::schematic_editor_request_source(
                &state
            ))
        );
        assert!(matches!(
            state.design.object_properties.draft,
            Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::NetLabel(ref draft))
                if draft.original == label
                    && draft.name == "afe.out"
                    && draft.x == "-12"
                    && draft.y == "34"
        ));
    }

    #[test]
    fn selected_named_wire_opens_net_scoped_properties_and_unnamed_wire_fails_closed() {
        let mut state = DesignTestHost::default();
        state
            .model
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::new(71, vec![Point::new(0, 0), Point::new(40, 0)]));
        state
            .model
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::new(72, Point::new(20, 0), "sense"));
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_wire(71);

        assert!(selected_object_properties_available_in(
            &state.design.property_registry,
            &state.model
        ));
        assert!(open_selected_object_properties_in(
            &mut state.design.property_form_parts(),
            &mut state.model,
            &mut state.workbench,
        ));
        assert_eq!(
            state.design.object_properties.source,
            Some(crate::workbench::app::schematic_editor_request_source(
                &state
            ))
        );
        assert!(matches!(
            state.design.object_properties.draft,
            Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::NamedNet(ref draft))
                if draft.original.name == "sense"
                    && draft.original.labels.iter().map(|label| label.id).collect::<Vec<_>>() == [72]
                    && draft.name == "sense"
        ));

        state.design.object_properties.close();
        state
            .model
            .schematic
            .document_mut_for_test()
            .net_labels
            .clear();
        assert!(!selected_object_properties_available_in(
            &state.design.property_registry,
            &state.model
        ));
        assert!(!open_selected_object_properties_in(
            &mut state.design.property_form_parts(),
            &mut state.model,
            &mut state.workbench,
        ));
    }

    #[test]
    fn interface_port_object_properties_preserve_the_full_typed_editor() {
        let mut state = DesignTestHost::default();
        state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .push(
                Component::new(73, ComponentType::Port, Point::origin())
                    .with_name_value("P73", "VIN"),
            );
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(73);

        assert!(selected_object_properties_available_in(
            &state.design.property_registry,
            &state.model
        ));
        assert!(open_selected_object_properties_in(
            &mut state.design.property_form_parts(),
            &mut state.model,
            &mut state.workbench,
        ));
        assert!(state.design.tabbed_property_dialog.open);
        assert!(!state.design.object_properties.open);
        assert_eq!(
            state.design.tabbed_property_dialog.draft.component_type,
            Some(ComponentType::Port)
        );
    }

    #[test]
    fn selected_design_note_opens_the_guarded_object_properties_transaction() {
        let mut state = DesignTestHost::default();
        let note = DesignNote::new(
            44,
            Point::new(-12, 34),
            DesignNoteKind::ReviewNote,
            "Review bias path",
        )
        .unwrap();
        state
            .model
            .schematic
            .document_mut_for_test()
            .design_notes
            .push(note.clone());
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_design_note(note.id);

        assert!(open_selected_object_properties_in(
            &mut state.design.property_form_parts(),
            &mut state.model,
            &mut state.workbench,
        ));
        assert_eq!(
            state.design.object_properties.source,
            Some(crate::workbench::app::schematic_editor_request_source(
                &state
            ))
        );
        assert!(matches!(
            state.design.object_properties.draft,
            Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::DesignNote(ref draft))
                if draft.original == note
                    && draft.kind == DesignNoteKind::ReviewNote
                    && draft.text == "Review bias path"
        ));
    }

    #[test]
    fn generic_properties_refuses_read_only_schematics() {
        let mut state = DesignTestHost::default();
        state
            .model
            .schematic
            .document_mut_for_test()
            .buses
            .push(Bus::segment(1, Point::new(0, 0), Point::new(10, 0), None).unwrap());
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_bus(1);
        state.model.schematic.session.read_only = true;

        assert!(!open_selected_object_properties_in(
            &mut state.design.property_form_parts(),
            &mut state.model,
            &mut state.workbench,
        ));
        assert!(!state.design.object_properties.open);
    }

    #[test]
    fn selected_probe_routes_to_the_inspector_even_in_read_only_mode() {
        let mut state = DesignTestHost::default();
        state.model.schematic.document_mut_for_test().probes.push(
            SchematicProbe::new(74, Point::new(10, 20), "V(out)", Some("V(out)".to_owned()))
                .unwrap(),
        );
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_probe(74);
        state.model.schematic.session.read_only = true;

        assert!(selected_object_properties_available_in(
            &state.design.property_registry,
            &state.model
        ));
        assert!(open_selected_object_properties_in(
            &mut state.design.property_form_parts(),
            &mut state.model,
            &mut state.workbench,
        ));
        assert!(state.workbench.inspector_visible);
        assert_eq!(
            state.workbench.drawer,
            Some(crate::workbench::state::Drawer::Inspector)
        );
        assert!(!state.design.object_properties.open);
        assert!(!state.design.tabbed_property_dialog.open);
    }

    #[test]
    fn stale_selected_object_ids_fail_closed_without_opening_a_dialog() {
        let mut state = DesignTestHost::default();
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_bus(9001);
        assert!(!open_selected_object_properties_in(
            &mut state.design.property_form_parts(),
            &mut state.model,
            &mut state.workbench,
        ));
        assert!(!state.design.object_properties.open);

        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_bus_tap(9002);
        assert!(!open_selected_object_properties_in(
            &mut state.design.property_form_parts(),
            &mut state.model,
            &mut state.workbench,
        ));
        assert!(!state.design.object_properties.open);

        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_net_label(9003);
        assert!(!open_selected_object_properties_in(
            &mut state.design.property_form_parts(),
            &mut state.model,
            &mut state.workbench,
        ));
        assert!(!state.design.object_properties.open);

        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_design_note(9004);
        assert!(!open_selected_object_properties_in(
            &mut state.design.property_form_parts(),
            &mut state.model,
            &mut state.workbench,
        ));
        assert!(!state.design.object_properties.open);
    }

    #[test]
    fn a_retained_property_transaction_cannot_be_overwritten_by_another_owner() {
        let mut state = DesignTestHost::default();
        let bus = Bus::segment(
            1,
            Point::new(0, 0),
            Point::new(10, 0),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        state
            .model
            .schematic
            .document_mut_for_test()
            .buses
            .push(bus);
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_bus(1);
        assert!(open_selected_object_properties_in(
            &mut state.design.property_form_parts(),
            &mut state.model,
            &mut state.workbench,
        ));

        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_bus(999);
        assert!(!open_selected_object_properties_in(
            &mut state.design.property_form_parts(),
            &mut state.model,
            &mut state.workbench,
        ));
        assert!(matches!(
            state.design.object_properties.draft,
            Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::Bus(ref draft)) if draft.original.id == 1
        ));
    }
}
