//! A compiled Verilog-A module's parameters as an instance's typed properties.
//!
//! A clean compile states its module's parameters as the view's
//! `veriloga.parameter_contract`. This turns that contract into the property
//! sheet the instance editor and the inspector's rows share, and names the
//! values an instance still carries for parameters its module no longer
//! declares, which the netlister refuses rather than emits.
//!
//! Every module parameter is an instance property, whatever scope the module
//! gives it: an instance of a module takes all of them on its own line. A
//! bound that names another parameter is stated in the description and left
//! to the compiled module, which checks it once a run binds the values.

use rspice_design::library::metadata::{
    CellParameter, ParameterDeclaration, ParameterScope, ParameterValueType, RangeBound,
    RangeValue, cell_parameter_contract, declares_parameter_contract,
};

use crate::state::{
    Cell, Component, ComponentType, InstanceMultiplicity, MODEL_BOUND_SYMBOL_METADATA_KEY,
    PropertyDefinition, PropertySheet, PropertyType, PropertyValue, View, ViewType,
};
use crate::workbench::app_state::AppState;

/// What an empty field of a parameter whose module derives its default from
/// other parameters stands for.
pub(crate) const COMPUTED_DEFAULT: &str = "computed";

/// Whether `view` of `cell` is a Verilog-A view whose own compile states the
/// parameters an instance takes. A cell that publishes a model-bound symbol
/// presents that symbol's form instead.
pub(super) fn states_compiled_parameters(cell: &Cell, view: &View) -> bool {
    view.view_type == ViewType::VerilogA
        && !cell.metadata.contains_key(MODEL_BOUND_SYMBOL_METADATA_KEY)
        && declares_parameter_contract(cell, Some(view))
}

/// The library view `component` is an instance of, when it is a project cell.
fn bound_view<'a>(state: &'a AppState, component: &Component) -> Option<(&'a Cell, &'a View)> {
    if component.kind != ComponentType::CellInstance {
        return None;
    }
    let binding = component
        .library_cell
        .as_ref()
        .filter(|binding| !binding.is_executable_builtin())?;
    let cell = state
        .library_manager
        .get_library(&binding.library)?
        .get_cell(&binding.cell)?;
    Some((cell, cell.get_view(&binding.view)?))
}

/// Whether `component` is an instance of a Verilog-A view, compiled or not.
///
/// Such an instance has no SPICE instance temperature: `temp` reaches it only
/// as a parameter its module declares, which its own row then offers.
pub(crate) fn instantiates_veriloga_view(state: &AppState, component: &Component) -> bool {
    bound_view(state, component).is_some_and(|(_, view)| view.view_type == ViewType::VerilogA)
}

/// Whether `component`'s property sheet is its module's compiled contract.
pub(crate) fn has_compiled_parameter_sheet(state: &AppState, component: &Component) -> bool {
    bound_view(state, component).is_some_and(|(cell, view)| states_compiled_parameters(cell, view))
}

/// Values `component` sets for parameters its compiled module does not
/// declare, as `(key, value)` in key order.
///
/// The netlister refuses each one, so none is ever emitted; the design check
/// reports them and the inspector offers to remove them, so none is ever
/// dropped without the reader seeing it either.
pub(crate) fn undeclared_instance_parameters(
    state: &AppState,
    component: &Component,
) -> Vec<(String, String)> {
    let Some((cell, view)) =
        bound_view(state, component).filter(|(cell, view)| states_compiled_parameters(cell, view))
    else {
        return Vec::new();
    };
    let Ok(contract) = cell_parameter_contract(cell, Some(view)) else {
        return Vec::new();
    };
    // `m` written as text is refused by name as the instance's multiplicity,
    // which is a different repair from this one.
    let mut undeclared = crate::state::parse_params_string(&component.params)
        .into_iter()
        .filter(|(key, _)| {
            !is_instance_multiplicity(component, key)
                && !contract
                    .iter()
                    .any(|parameter| parameter.name.eq_ignore_ascii_case(key))
        })
        .collect::<Vec<_>>();
    undeclared.sort();
    undeclared
}

impl AppState {
    /// Take the value instance `component_id` sets for `key`, a parameter its
    /// module no longer declares, off the instance as one undoable edit. A key
    /// the module still declares is refused here: that one is edited, not
    /// removed. Returns whether the design changed.
    pub(crate) fn remove_undeclared_instance_parameter(
        &mut self,
        component_id: u64,
        key: &str,
    ) -> Result<bool, String> {
        let component = self
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == component_id)
            .cloned()
            .ok_or_else(|| "The selected instance no longer exists.".to_owned())?;
        if !undeclared_instance_parameters(self, &component)
            .iter()
            .any(|(undeclared, _)| undeclared.eq_ignore_ascii_case(key))
        {
            return Err(format!("{key} is a parameter its module declares"));
        }
        let mut candidate = component.clone();
        candidate.params =
            crate::state::params_string::set_parameter_value(&component.params, key, "")?;
        self.edit_component_transaction(&component, candidate, &format!("remove {key}"))
    }
}

/// The property sheet of an instance of a compiled Verilog-A view: its
/// identity, its multiplicity, which a module reads as `$mfactor`, and one
/// typed field per module parameter.
pub(super) fn compiled_parameter_sheet(cell: &Cell, view: &View) -> Result<PropertySheet, String> {
    let mut sheet = super::property_edit::cell_instance_identity_sheet();
    sheet.add(super::property_edit::instance_multiplicity_definition());
    for (index, parameter) in cell_parameter_contract(cell, Some(view))?
        .iter()
        .enumerate()
    {
        sheet.add(parameter_property(
            parameter,
            i32::try_from(index).unwrap_or(i32::MAX),
        ));
    }
    Ok(sheet)
}

/// One module parameter as a typed property. The key is the parameter's
/// lower-case spelling, which is how an instance's parameter text stores it,
/// and the label is the module's own spelling.
fn parameter_property(parameter: &CellParameter, order: i32) -> PropertyDefinition {
    let definition = PropertyDefinition::new(parameter.name.to_ascii_lowercase())
        .with_display_name(&parameter.name)
        .with_max_length(512)
        .with_order(order);
    let Some(declaration) = parameter.declaration.as_ref() else {
        // An authored entry beside the compiled ones states a name and a
        // default, and nothing a typed field could enforce.
        let mut definition = definition
            .with_type(PropertyType::Expression)
            .with_default(PropertyValue::expression(
                parameter.default_value.clone().unwrap_or_default(),
            ))
            .with_description(format!(
                "Instance override for the {} parameter.",
                parameter.name
            ))
            .with_category("Cell parameters");
        if parameter.required {
            definition = definition.required();
        }
        return definition;
    };
    let mut definition = definition
        .with_category(match declaration.scope {
            ParameterScope::Instance => "Instance parameters",
            ParameterScope::Model => "Model parameters",
        })
        .with_description(parameter_description(parameter, declaration));
    if let Some(units) = declaration
        .units
        .as_deref()
        .filter(|units| !units.is_empty())
    {
        definition = definition.with_unit(units);
    }
    let default = parameter.default_value.as_deref();
    definition = match declaration.value_type {
        ParameterValueType::String => definition
            .with_type(PropertyType::String)
            .with_default(PropertyValue::string(default.unwrap_or_default())),
        ParameterValueType::Real | ParameterValueType::Integer => {
            match default.and_then(|text| text.parse::<f64>().ok()) {
                Some(value) => definition
                    .with_type(PropertyType::Number)
                    .with_default(PropertyValue::number(value))
                    .with_default_hint(default.unwrap_or_default()),
                // No number a field could hold stands for a computed default,
                // so the field is an optional expression and its empty state
                // says what it means.
                None => definition
                    .with_type(PropertyType::Expression)
                    .with_default(PropertyValue::expression(""))
                    .with_default_hint(COMPUTED_DEFAULT),
            }
        }
    };
    if declaration.value_type == ParameterValueType::Integer {
        definition = definition.integer();
    }
    let Some(range) = declaration.range.as_ref() else {
        return definition;
    };
    let constant = |bound: &RangeBound| match bound {
        RangeBound::Inclusive(RangeValue::Constant(value)) => Some((*value, false)),
        RangeBound::Exclusive(RangeValue::Constant(value)) => Some((*value, true)),
        _ => None,
    };
    if let Some((min, exclusive)) = constant(&range.lower) {
        definition = definition.with_min(min, exclusive);
    }
    if let Some((max, exclusive)) = constant(&range.upper) {
        definition = definition.with_max(max, exclusive);
    }
    for excluded in &range.exclude {
        if let RangeValue::Constant(value) = excluded {
            definition = definition.excluding(*value);
        }
    }
    definition
}

/// The module's own description, then what the field cannot show: the
/// parameter's type and scope, its default, and its declared range, with the
/// terms only the compiled module can check called out as such.
fn parameter_description(parameter: &CellParameter, declaration: &ParameterDeclaration) -> String {
    let mut text = String::new();
    if let Some(described) = declaration
        .description
        .as_deref()
        .map(str::trim)
        .filter(|described| !described.is_empty())
    {
        text.push_str(described);
        if !described.ends_with('.') {
            text.push('.');
        }
        text.push(' ');
    }
    let kind = match declaration.value_type {
        ParameterValueType::Real => "Real",
        ParameterValueType::Integer => "Integer",
        ParameterValueType::String => "String",
    };
    let scope = match declaration.scope {
        ParameterScope::Instance => "instance",
        ParameterScope::Model => "model",
    };
    match parameter.default_value.as_deref() {
        Some(default) => text.push_str(&format!("{kind} {scope} parameter, default {default}.")),
        None => text.push_str(&format!(
            "{kind} {scope} parameter; its module computes the default from other parameters."
        )),
    }
    if let Some(range) = declaration.range.as_ref() {
        text.push_str(&format!(" Declared {}.", range.source_text()));
        if range.has_dependent_terms() {
            text.push_str(
                " Terms that name other parameters are checked by the compiled module when a run binds their values.",
            );
        }
    }
    if !parameter.aliases.is_empty() {
        text.push_str(&format!(" Aliases: {}.", parameter.aliases.join(", ")));
    }
    text
}

/// Whether `key` is the multiplicity a cell instance carries as a typed
/// field rather than as parameter text.
pub(crate) fn is_instance_multiplicity(component: &Component, key: &str) -> bool {
    component.kind == ComponentType::CellInstance
        && key.eq_ignore_ascii_case(InstanceMultiplicity::PARAMETER_NAME)
}
