//! The component inspector: identity, model binding, parameters, terminals.
//!
//! Every editable field validates before it commits and reports the rejection
//! in place, so an invalid value is never written and never silently reverted
//! either. Operating-point and SoA rows are shown only when the retained run
//! actually matches the current design — a stale result is reported as stale
//! rather than displayed as this component's state.

use super::*;
use crate::diagnostics::ConsoleMessage;

pub(super) fn component_panel(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    id: u64,
    sheet: &SheetConnectivity,
) {
    let Some(component) = app
        .state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .cloned()
    else {
        sheet_panel(ui, app);
        return;
    };
    let evidence = component_model_evidence(&app.state, &component);
    // The design root is implicit: on the top sheet the path says nothing.
    let occurrence = containing_occurrence_path(&app.state);
    object_title(
        ui,
        &component.name,
        Some(&component_title_kind(&component)),
        (occurrence != "/").then_some(occurrence.as_str()),
    );

    identity_section(ui, app, &component);
    terminals_section(ui, app, &component, sheet);
    parameters_section(ui, app, &component, &evidence);
    operating_point(ui, app, &component);
    component_checks(ui, app, &component);
}

/// The occurrence the inspected component sits in.
///
/// The title names the component beside this, so the path carries the
/// enclosing instances and stops there — and the design root is implicit, so an
/// inspected component on the top sheet reports `/`.
pub(super) fn containing_occurrence_path(state: &AppState) -> String {
    state.workspace.content.occurrence_path().to_string()
}

/// What a component is, for its title: a built-in's catalog name, a generated
/// module's name, a cell instance's cell, else its device kind.
pub(super) fn component_title_kind(component: &Component) -> String {
    let binding = component.library_cell.as_ref();
    if let Some(descriptor) = binding
        .filter(|binding| binding.is_builtin_xspice())
        .and_then(|binding| crate::state::validate_builtin_xspice_binding(binding).ok())
    {
        return descriptor.display_name.to_owned();
    }
    if let Some(descriptor) = binding
        .filter(|binding| binding.is_generated_veriloga())
        .and_then(|binding| crate::state::validate_generated_veriloga_binding(binding).ok())
    {
        return descriptor.module_name.to_owned();
    }
    binding.map_or_else(
        || component.kind.display_name().to_owned(),
        |binding| binding.cell.clone(),
    )
}

/// The view a cell instance binds, by its name. A primitive binds none: its
/// symbol is the device's own.
pub(super) fn instance_view(component: &Component) -> Option<&str> {
    component
        .library_cell
        .as_ref()
        .map(|binding| binding.view.trim())
        .filter(|view| !view.is_empty())
}

pub(super) fn identity_section(ui: &mut Ui, app: &mut RSpiceApp, component: &Component) {
    let editable = !app.state.schematic_edit_read_only();
    let properties = section_header_action(ui, "Identity", "Properties…", editable);
    if properties.clicked() {
        crate::workbench::app::open_property_editor(&mut app.state, component.id);
    }
    properties.on_disabled_hover_text("the active document is read-only");
    let instance_rejection = edit_row(ui, app, component, InlineEditField::Instance, "Instance");
    // A built-in or a generated module runs the model its contract names, and
    // the Model row under Simulation parameters states it; it has no value.
    let runs_contract = component.library_cell.as_ref().is_some_and(|binding| {
        binding.builtin_xspice.is_some() || binding.generated_veriloga.is_some()
    });
    let value_rejection = if runs_contract {
        None
    } else {
        edit_row(ui, app, component, InlineEditField::Value, "Value")
    };
    if let Some(binding) = component.library_cell.as_ref() {
        property_row(
            ui,
            "Library cell",
            &format!("{}/{}", binding.library, binding.cell),
        );
    }
    if let Some(view) = instance_view(component) {
        property_row(ui, "View", view);
    }
    sheet_row(ui, &app.state, component.id);
    if editable {
        rejection_slot(
            ui,
            editing_identity_field(&app.state, component.id),
            instance_rejection.as_deref().or(value_rejection.as_deref()),
        );
    }

    // Identity actions exist only when the design can actually perform
    // them: the editable-properties transaction always, the symbol
    // cellview when the instance is bound to one, and the exact catalog
    // model when its source identity resolves.
    let symbol_view = component
        .library_cell
        .as_ref()
        .map(|binding| {
            CellViewRef::new(
                binding.library.clone(),
                binding.cell.clone(),
                "symbol".to_owned(),
            )
        })
        .filter(|reference| {
            app.state
                .library_manager
                .get_library(&reference.library)
                .and_then(|library| library.get_cell(&reference.cell))
                .and_then(|cell| cell.get_view(&reference.view))
                .is_some()
        });
    let symbol_writable = symbol_view.as_ref().is_some_and(|reference| {
        app.state
            .library_manager
            .get_library(&reference.library)
            .is_some_and(|library| !library.read_only)
    });
    let model_source = component_model_source_target(&app.state, component);
    let hierarchy_master = app
        .state
        .hierarchy_master_for_component(component.id)
        .map(|(_, reference)| reference);
    // The deck the instance emits is part of its identity, so the jump to it
    // belongs beside the symbol, master and model routes rather than in a
    // section of its own. It appears only once a deck states the instance.
    let netlist_card = Command::ShowInNetlist.is_enabled(app);
    if symbol_view.is_some() || hierarchy_master.is_some() || model_source.is_some() || netlist_card
    {
        action_stack(ui, |ui| {
            if let Some(reference) = symbol_view {
                let response = Button::new(if symbol_writable {
                    "Edit symbol…"
                } else {
                    "Open symbol…"
                })
                .ghost()
                .show(ui);
                if response.clicked() {
                    app.state.open_workspace_view(reference);
                }
            }

            if let Some(reference) = hierarchy_master.as_ref() {
                let label = format!("Descend into {}", reference.cell);
                if Button::new(&label).ghost().show(ui).clicked() {
                    app.state.open_selected_instance_master();
                }
            }

            if netlist_card && Button::new("Show in netlist").ghost().show(ui).clicked() {
                Command::ShowInNetlist.execute(app);
            }

            if let Some(target) = model_source.as_ref() {
                // A module's source is its cell view, which opens as a tab, so
                // the button says so in the canvas menu's words.
                let label = match target {
                    ComponentModelSourceTarget::Catalog { .. } => "Open model source…",
                    ComponentModelSourceTarget::VerilogA(_) => "Open Verilog-A view",
                };
                let response = Button::new(label).ghost().show(ui);
                if response.clicked() {
                    if !app.state.commit_pending_inspector_edit() {
                        return;
                    }
                    match target {
                        ComponentModelSourceTarget::Catalog { library, model } => {
                            app.state.select_model_library(library.as_str());
                            app.state.workbench.selected_model = Some(model.clone());
                            app.state.workbench.models_page = ModelsPage::Models;
                            app.state.workbench.activate(Workspace::Models);
                        }
                        ComponentModelSourceTarget::VerilogA(reference) => {
                            app.state.open_workspace_view(reference.clone());
                            app.state.workbench.activate(Workspace::Design);
                        }
                    }
                }
            }
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ComponentModelSourceTarget {
    Catalog { library: String, model: String },
    VerilogA(CellViewRef),
}

pub(super) fn component_model_source_target(
    state: &AppState,
    component: &Component,
) -> Option<ComponentModelSourceTarget> {
    if let Some(binding) = component.library_cell.as_ref() {
        let reference = CellViewRef::new(
            binding.library.clone(),
            binding.cell.clone(),
            binding.view.clone(),
        );
        let is_veriloga = state
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .is_some_and(|view| view.view_type == crate::state::ViewType::VerilogA);
        if is_veriloga {
            return Some(ComponentModelSourceTarget::VerilogA(reference));
        }
    }
    catalog_model_location(state, component)
        .map(|(library, model)| ComponentModelSourceTarget::Catalog { library, model })
}

pub(super) fn catalog_model_location(
    state: &AppState,
    component: &Component,
) -> Option<(String, String)> {
    if let Some(binding) = component.library_cell.as_ref()
        && let Some(library) = state.model_library_manager.get_library(&binding.library)
    {
        let candidates = [
            binding.module_name.as_deref(),
            Some(binding.cell.as_str()),
            (!component.value.trim().is_empty()).then_some(component.value.trim()),
        ];
        if let Some(model) = candidates.into_iter().flatten().find_map(|candidate| {
            library
                .models
                .values()
                .find(|model| model.name.eq_ignore_ascii_case(candidate))
        }) {
            return Some((library.name.clone(), model.name.clone()));
        }
    }

    // The jump follows the provider a run would execute, so a contested
    // definition the project has already resolved still has a source to open.
    // Requiring exactly one name match made the action vanish precisely when
    // an engineer most needs to see which copy wins.
    let model_name = crate::state::explicit_component_model(component)?;
    let provider = state
        .model_library_manager
        .effective_definition_provider(
            crate::state::model_library::ModelConsumerScope::PrimitiveModel,
            &model_name,
        )
        .ok()
        .flatten()?;
    let library = state.model_library_manager.get_library(&provider.library)?;
    let model = library
        .models
        .values()
        .find(|model| model.name == provider.definition)?;
    Some((library.name.clone(), model.name.clone()))
}

/// The instance parameters a classic SPICE MOSFET reads: the geometry, the
/// diffusion parasitics it sizes the junctions from, the instance
/// temperature and the `UIC` initial conditions.
///
/// LEVELs 1 to 6 — Shichman-Hodges through the legacy BSIM1/BSIM2 cards —
/// all reach the same builder and read the same instance set.
const CLASSIC_MOS_INSTANCE_PARAMETERS: &[&str] = &[
    "w", "l", "m", "ad", "as", "pd", "ps", "nrd", "nrs", "temp", "dtemp", "ic_vds", "ic_vgs",
    "ic_vbs",
];

/// The SPICE `LEVEL` of the model this instance runs.
///
/// The card it binds, or — for a bare placement, which the netlister resolves
/// onto its family's foundation card — that card's level.
fn bound_model_spice_level(state: &AppState, component: &Component) -> Option<u32> {
    let (library, model) = catalog_model_location(state, component)
        .or_else(|| foundation_default_model_location(state, component))?;
    let model = state
        .model_library_manager
        .get_library(&library)?
        .models
        .values()
        .find(|candidate| candidate.name == model)?;
    model.spice_level.or(match model.level {
        crate::state::model_library::ModelLevel::SpiceLevel1 => Some(1),
        crate::state::model_library::ModelLevel::SpiceLevel3 => Some(3),
        _ => None,
    })
}

/// Where the foundation card a bare bulk-MOS placement runs on lives.
fn foundation_default_model_location(
    state: &AppState,
    component: &Component,
) -> Option<(String, String)> {
    let family = match component.kind {
        ComponentType::Nmos => rspice_core::library::FoundationDeviceFamily::Nmos,
        ComponentType::Pmos => rspice_core::library::FoundationDeviceFamily::Pmos,
        _ => return None,
    };
    let provider = state
        .model_library_manager
        .effective_definition_provider(
            crate::state::model_library::ModelConsumerScope::PrimitiveModel,
            family.default_model_name(),
        )
        .ok()
        .flatten()?;
    Some((provider.library, provider.definition))
}

/// The instance parameters the bound model's level reads, or `None` when the
/// level is unknown or reads the sheet whole.
///
/// The MOSFET property sheet is the union of every MOS level's instance set,
/// so without this a LEVEL=1 inverter offers BSIM4 geometry modes, stress
/// distances, mismatch terms and body squares that nothing in its deck reads.
fn level_supported_instance_parameters(
    state: &AppState,
    component: &Component,
) -> Option<&'static [&'static str]> {
    if !matches!(component.kind, ComponentType::Nmos | ComponentType::Pmos) {
        return None;
    }
    matches!(bound_model_spice_level(state, component)?, 1..=6)
        .then_some(CLASSIC_MOS_INSTANCE_PARAMETERS)
}

pub(super) fn bound_model_choices(
    state: &AppState,
    component: &Component,
    current: &str,
) -> Vec<(String, String)> {
    let Some(binding) = component
        .library_cell
        .as_ref()
        .filter(|binding| binding.netlist_template.is_some())
    else {
        return Vec::new();
    };
    let Some(library) = state.model_library_manager.get_library(&binding.library) else {
        return Vec::new();
    };
    let Some(current_model) = library
        .models
        .values()
        .find(|model| model.name.eq_ignore_ascii_case(current))
    else {
        return Vec::new();
    };
    let mut models = library
        .models
        .values()
        .filter(|model| {
            crate::state::model_library::models_have_compatible_device_family(current_model, model)
        })
        .map(|model| model.name.clone())
        .collect::<Vec<_>>();
    models.sort_by_key(|name| name.to_ascii_lowercase());
    models.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    models
        .into_iter()
        .map(|model| (model.clone(), model))
        .collect()
}

pub(super) fn bound_model_section_choices(
    state: &AppState,
    component: &Component,
    selected_model: &str,
) -> Vec<(String, String)> {
    let Some(binding) = component.library_cell.as_ref() else {
        return Vec::new();
    };
    let Some(library) = state.model_library_manager.get_library(&binding.library) else {
        return Vec::new();
    };
    let mut sections = library
        .model_definition_metadata
        .iter()
        .find(|(model, _)| model.eq_ignore_ascii_case(selected_model))
        .map(|(_, metadata)| {
            metadata
                .sections
                .iter()
                .map(|section| section.name.clone())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if let Some(section) = binding
        .model_section
        .as_deref()
        .filter(|section| !section.trim().is_empty())
        && !sections
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(section))
    {
        sections.push(section.to_owned());
    }
    sections.sort_by_key(|section| section.to_ascii_lowercase());
    sections.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    let mut choices = vec![(String::new(), "default".to_owned())];
    choices.extend(sections.into_iter().map(|section| {
        let display = section.clone();
        (section, display)
    }));
    choices
}

pub(crate) fn apply_bound_model_choice(
    app: &mut RSpiceApp,
    component_id: u64,
    selected_model: &str,
) -> Result<bool, String> {
    app.state.commit_inline_component_edit()?;
    let component = app
        .state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .ok_or_else(|| "The selected instance no longer exists.".to_owned())?;
    let library_name = component
        .library_cell
        .as_ref()
        .ok_or_else(|| "The selected instance is not bound to a library cell.".to_owned())?
        .library
        .clone();

    super::super::validate_component_model_catalog_binding(
        &app.state,
        component_id,
        &library_name,
        selected_model,
    )?;

    let library = app
        .state
        .model_library_manager
        .get_library(&library_name)
        .ok_or_else(|| format!("Model library '{library_name}' is no longer loaded."))?;
    let candidate = library
        .models
        .values()
        .find(|model| model.name.eq_ignore_ascii_case(selected_model))
        .ok_or_else(|| {
            format!("Model '{selected_model}' is no longer present in library '{library_name}'.")
        })?;
    let candidate_name = candidate.name.clone();
    let candidate_source = candidate
        .file_path
        .clone()
        .or_else(|| library.root_path.clone());
    let changed =
        app.state
            .schematic
            .select_instance_model(component_id, candidate_name, candidate_source);
    if changed {
        app.invalidate_simulation_preflight();
    }
    Ok(changed)
}

pub(super) fn apply_bound_model_section(
    app: &mut RSpiceApp,
    component_id: u64,
    selected_section: &str,
) {
    if !app.state.commit_pending_inspector_edit() {
        return;
    }
    let Some(component) = app
        .state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
    else {
        return;
    };
    let selected_model = component
        .library_cell
        .as_ref()
        .and_then(|binding| binding.module_name.as_deref())
        .unwrap_or(component.value.as_str());
    let allowed = bound_model_section_choices(&app.state, &component, selected_model);
    if !allowed
        .iter()
        .any(|(value, _)| value.eq_ignore_ascii_case(selected_section))
    {
        return;
    }
    let selected = (!selected_section.trim().is_empty()).then(|| selected_section.to_owned());
    if app
        .state
        .schematic
        .select_instance_model_section(component_id, selected)
    {
        app.invalidate_simulation_preflight();
    }
}

pub(super) fn parameters_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    component: &Component,
    evidence: &ComponentModelEvidence,
) {
    let editable = !app.state.schematic_edit_read_only();
    section_header(
        ui,
        "Simulation parameters",
        (!editable).then_some("read-only"),
    );
    // A Verilog-A instance runs its view's module: that is the one fact to
    // state here, and only when the module is not the cell the Library cell
    // row already names. A model card and a library section are model-library
    // ideas, and the deck gives a module neither.
    let runs_module = crate::workbench::app::instantiates_veriloga_view(&app.state, component);
    let model_choices = evidence.model.as_deref().map_or_else(Vec::new, |model| {
        bound_model_choices(&app.state, component, model)
    });
    let mut selected_model = component
        .library_cell
        .as_ref()
        .and_then(|binding| binding.module_name.clone())
        .or_else(|| evidence.model.clone())
        .unwrap_or_default();
    if runs_module {
        if let Some(module) = evidence.model.as_deref().filter(|module| {
            component
                .library_cell
                .as_ref()
                .is_none_or(|binding| !binding.cell.eq_ignore_ascii_case(module))
        }) {
            property_row(ui, "Module", module);
        }
    } else if model_choices.len() > 1 {
        if property_row_combo(
            ui,
            "Model",
            ("design-inspector-model", component.id),
            &mut selected_model,
            &model_choices,
            editable,
        ) && let Err(error) = apply_bound_model_choice(app, component.id, &selected_model)
        {
            app.state
                .push_user_message(ConsoleMessage::error(error.clone()));
            app.state
                .ui
                .toasts
                .error_with_title(ui.ctx(), "Model binding rejected", error);
        }
    } else if let Some(model) = evidence.model.as_deref() {
        property_row(ui, "Model", model);
    }
    if let Some((problem, text)) = evidence.problem.as_ref() {
        crate::workbench::design_system::problem_line(ui, *problem, text);
    }

    let section_choices = if runs_module {
        Vec::new()
    } else {
        bound_model_section_choices(&app.state, component, &selected_model)
    };
    let mut selected_section = component
        .library_cell
        .as_ref()
        .and_then(|binding| binding.model_section.clone())
        .unwrap_or_default();
    if section_choices.len() > 1
        && property_row_combo(
            ui,
            "Section",
            ("design-inspector-model-section", component.id),
            &mut selected_section,
            &section_choices,
            editable,
        )
    {
        apply_bound_model_section(app, component.id, &selected_section);
    }

    // Instance temperature is the `temp` parameter. Left empty it inherits
    // the reference PVT point, which the row reports as its placeholder.
    let declared = crate::state::parse_params_string(&component.params);
    let inherited = format!(
        "inherit · {} °C",
        app.state.sim_setup.reference_pvt.temperature_celsius
    );
    let mut rejection = supports_instance_temperature(&app.state, component)
        .then(|| {
            edit_row_with_hint(
                ui,
                app,
                component,
                InlineEditField::Parameter(TEMPERATURE_PARAM.to_owned()),
                "Temperature",
                &inherited,
                PropertyRowCaption::default(),
            )
        })
        .flatten();

    // Every parameter declared by the authoritative family sheet is present
    // even on a freshly placed instance whose durable parameter string is
    // still empty. Authored extension fields are appended and never hidden.
    let parameter_contract = inline_parameter_contract(&app.state, component, &declared);
    let raw_field = InlineEditField::Parameters;
    let raw_editor_active = app
        .state
        .workbench
        .inline_edit
        .buffer_for(component.id, &raw_field)
        .is_some();
    if parameter_contract.is_empty() || raw_editor_active {
        let raw_rejection = edit_row_with_hint(
            ui,
            app,
            component,
            raw_field,
            "Parameters",
            "instance value",
            PropertyRowCaption::default(),
        );
        rejection = rejection.or(raw_rejection);
        if editable {
            rejection_slot(
                ui,
                editing_parameter_field(&app.state, component.id),
                rejection.as_deref(),
            );
        }
        return;
    }
    let mut removed = None;
    for parameter in parameter_contract {
        let caption = PropertyRowCaption {
            unit: parameter.unit.as_deref(),
            description: parameter.description.as_deref(),
        };
        if parameter.undeclared {
            let value = declared
                .get(&parameter.key.to_ascii_lowercase())
                .map_or("", String::as_str);
            if editable {
                if property_row_flagged(
                    ui,
                    &parameter.label,
                    value,
                    UNDECLARED_PARAMETER_NOTE,
                    Some(Problem::Warning),
                    REMOVE_UNDECLARED_PARAMETER,
                ) {
                    removed = Some(parameter.key);
                }
            } else {
                crate::workbench::design_system::schematic_property_row_problem(
                    ui,
                    &parameter.label,
                    &format!("{value} {UNDECLARED_PARAMETER_NOTE}"),
                    Some(Problem::Warning),
                );
            }
        } else if parameter.editable {
            let row_rejection = edit_row_with_hint(
                ui,
                app,
                component,
                InlineEditField::Parameter(parameter.key.clone()),
                &parameter.label,
                &parameter.hint,
                caption,
            );
            rejection = rejection.or(row_rejection);
        } else {
            let value = declared
                .get(&parameter.key)
                .map_or(parameter.hint.as_str(), String::as_str);
            if caption.unit.is_some() || caption.description.is_some() {
                property_row_captioned(ui, &parameter.label, value, caption);
            } else {
                property_row(ui, &parameter.label, value);
            }
        }
    }
    if editable {
        rejection_slot(
            ui,
            editing_parameter_field(&app.state, component.id),
            rejection.as_deref(),
        );
    }
    if let Some(key) = removed
        && app.state.commit_pending_inspector_edit()
    {
        match app
            .state
            .remove_undeclared_instance_parameter(component.id, &key)
        {
            Ok(true) => app.invalidate_simulation_preflight(),
            Ok(false) => {}
            Err(error) => app.state.push_user_message(ConsoleMessage::warning(format!(
                "{key} was not removed from {}: {error}",
                component.name
            ))),
        }
    }
}

/// What an undeclared parameter's row says after its value.
pub(super) const UNDECLARED_PARAMETER_NOTE: &str = "not in module";
/// The action that takes an undeclared parameter's value off the instance.
pub(super) const REMOVE_UNDECLARED_PARAMETER: &str = "Remove";

/// Whether `component` takes the SPICE instance temperature `temp`.
///
/// A built-in takes it when its compiled contract names it. An instance of a
/// Verilog-A view never does: a module reads `$temperature`, and takes `temp`
/// only as a parameter it declares, which its own typed row then offers.
pub(super) fn supports_instance_temperature(state: &AppState, component: &Component) -> bool {
    if crate::workbench::app::instantiates_veriloga_view(state, component) {
        return false;
    }
    component
        .library_cell
        .as_ref()
        .filter(|binding| binding.is_executable_builtin())
        .is_none_or(|binding| {
            binding
                .parameter_order
                .iter()
                .any(|name| name.eq_ignore_ascii_case(TEMPERATURE_PARAM))
        })
}

/// `true` while one of the identity group's editable fields holds the open
/// inline-edit session.
pub(super) fn editing_identity_field(state: &AppState, component: u64) -> bool {
    matches!(
        state.workbench.inline_edit.editing_field(component),
        Some(InlineEditField::Instance | InlineEditField::Value)
    )
}

/// `true` while one of the parameter group's editable fields — the instance
/// temperature, a typed parameter, or the raw override string — holds the
/// open inline-edit session.
pub(super) fn editing_parameter_field(state: &AppState, component: u64) -> bool {
    matches!(
        state.workbench.inline_edit.editing_field(component),
        Some(InlineEditField::Parameters | InlineEditField::Parameter(_))
    )
}

/// SPICE instance-temperature parameter.
pub(super) const TEMPERATURE_PARAM: &str = "temp";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct InlineParameterContract {
    pub(super) key: String,
    pub(super) label: String,
    pub(super) hint: String,
    pub(super) editable: bool,
    /// The unit the sheet states the value in.
    pub(super) unit: Option<String>,
    /// What the sheet says the parameter is, shown on the label's hover.
    pub(super) description: Option<String>,
    /// A value the instance sets for a parameter its compiled module no
    /// longer declares: shown, flagged, and offered for removal, because the
    /// netlister refuses it rather than emitting or dropping it.
    pub(super) undeclared: bool,
}

pub(super) fn inline_parameter_contract(
    state: &AppState,
    component: &Component,
    declared: &HashMap<String, String>,
) -> Vec<InlineParameterContract> {
    let primary = crate::properties::property_bridge::get_primary_property_name(component.kind);
    // `temp` has its own row where the instance takes the SPICE instance
    // temperature; anywhere else a `temp` the sheet declares is an ordinary
    // parameter row.
    let temperature_row = supports_instance_temperature(state, component);
    let is_temperature = |key: &str| temperature_row && key.eq_ignore_ascii_case(TEMPERATURE_PARAM);
    let compiled = crate::workbench::app::has_compiled_parameter_sheet(state, component);
    // A parameter the bound model's level does not read has no row, unless
    // the instance sets it anyway: then it is the instance's own value and
    // belongs on the panel whatever the card would do with it.
    let supported = level_supported_instance_parameters(state, component);
    let read_by_the_model = |name: &str| {
        supported.is_none_or(|set| {
            set.iter()
                .any(|supported| supported.eq_ignore_ascii_case(name))
                || declared.contains_key(&name.to_ascii_lowercase())
        })
    };
    let mut rows = Vec::new();
    let mut seen = HashSet::new();
    if let Ok(Some(sheet)) =
        crate::workbench::app::authoritative_component_property_sheet(state, component)
    {
        for definition in sheet.iter().filter(|definition| {
            definition.display_mode != DisplayMode::Hidden
                && definition.name != "name"
                && definition.name != "symbol"
                && !definition.name.eq_ignore_ascii_case(primary)
                && !is_temperature(&definition.name)
                && read_by_the_model(&definition.name)
        }) {
            let hint = definition.default_hint.clone().unwrap_or_else(|| {
                match &definition.default_value {
                    PropertyValue::Expression(source) => source.clone(),
                    value => value.display_string(),
                }
            });
            seen.insert(definition.name.to_ascii_lowercase());
            rows.push(InlineParameterContract {
                key: definition.name.clone(),
                label: definition.display_name.clone(),
                hint,
                editable: !definition.read_only && definition.display_mode != DisplayMode::Readonly,
                unit: definition.unit.clone().filter(|unit| !unit.is_empty()),
                description: Some(definition.description.clone())
                    .filter(|description| !description.trim().is_empty()),
                undeclared: false,
            });
        }
    }

    // A compiled module's sheet is its current contract; the placement's
    // copy of an older one names nothing the sheet does not.
    if let Some(binding) = component.library_cell.as_ref().filter(|_| !compiled) {
        for key in &binding.parameter_order {
            let normalized = key.to_ascii_lowercase();
            if normalized == primary || is_temperature(&normalized) || !seen.insert(normalized) {
                continue;
            }
            rows.push(InlineParameterContract {
                key: key.clone(),
                label: key.clone(),
                hint: "inherit".to_owned(),
                editable: true,
                unit: None,
                description: None,
                undeclared: false,
            });
        }
    }

    let mut extensions = declared
        .keys()
        .filter(|key| {
            !is_temperature(key)
                && !key.eq_ignore_ascii_case(primary)
                && !seen.contains(&key.to_ascii_lowercase())
        })
        .cloned()
        .collect::<Vec<_>>();
    extensions.sort_unstable_by_key(|key| key.to_ascii_lowercase());
    rows.extend(extensions.into_iter().map(|key| InlineParameterContract {
        label: key.clone(),
        key,
        hint: "instance value".to_owned(),
        editable: !compiled,
        unit: None,
        description: None,
        undeclared: compiled,
    }));
    rows
}

/// Icon box, then the gap before the pin name. The icon box starts on the
/// panel inset, with the section header's title and the property rows' label
/// column.
pub(super) const TERMINAL_ROW_ICON_W: f32 = 15.0;
pub(super) const TERMINAL_ROW_ICON_GAP: f32 = 6.0;

/// Per-pin terminal table: every declared terminal with the net it binds,
/// clickable to select that conductor. Unbound pins read `open` and are
/// not clickable, because there is no net to select.
pub(super) fn terminal_row(ui: &mut Ui, pin: &str, net: Option<&str>) -> TreeRowResult {
    ui.add_enabled_ui(net.is_some(), |ui| {
        let t = Tokens::get(ui.ctx());
        let meta = net.unwrap_or("open");
        let accessible_label = format!("{pin}, {meta}");
        let inset = panel_inset(ui);
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(
                ui.available_width(),
                crate::workbench::design_system::property_row_height(ui),
            ),
            egui::Sense::click(),
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::SelectableLabel,
                ui.is_enabled(),
                &accessible_label,
            )
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::TreeItem);
            // The pins are a flat list under the section heading; no row of
            // this table has a parent row to be nested beneath.
            node.set_level(1);
        });

        if ui.is_rect_visible(rect) {
            if response.hovered() && ui.is_enabled() {
                ui.painter()
                    .rect_filled(panel_bleed(ui, rect), 0.0, t.color.bg_hover);
            }

            // The list has no parent row and no pin ever expands, so the
            // mockup's nesting inset and blank caret column would only push
            // the whole table away from the panel edge. The status icon takes
            // the panel inset instead.
            let icon_center = egui::pos2(
                rect.left() + inset + TERMINAL_ROW_ICON_W * 0.5,
                rect.center().y,
            );
            if net.is_some() {
                ui.painter().hline(
                    egui::Rangef::new(icon_center.x - 5.5, icon_center.x + 5.5),
                    icon_center.y,
                    egui::Stroke::new(1.25, t.color.wire),
                );
            } else {
                ui.painter().circle_stroke(
                    icon_center,
                    4.0,
                    egui::Stroke::new(1.0, t.color.text_dim),
                );
            }

            let meta_galley = ui.painter().layout_no_wrap(
                meta.to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
            );
            let meta_left = rect.right() - inset - meta_galley.size().x;
            let label_left = rect.left() + inset + TERMINAL_ROW_ICON_W + TERMINAL_ROW_ICON_GAP;
            ui.painter()
                .with_clip_rect(egui::Rect::from_x_y_ranges(
                    label_left..=(meta_left - 8.0).max(label_left),
                    rect.y_range(),
                ))
                .text(
                    egui::pos2(label_left, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    pin,
                    theme::mono(tokens::FS_1, FontWeight::Regular),
                    if net.is_some() {
                        t.color.text_dim
                    } else {
                        t.color.text_faint
                    },
                );
            ui.painter().galley(
                egui::pos2(meta_left, rect.center().y - meta_galley.size().y * 0.5),
                meta_galley,
                t.color.text_faint,
            );
            theme::paint_focus_ring(ui, &response, rect);
        }

        TreeRowResult {
            response,
            switch_changed: false,
        }
    })
    .inner
}

pub(super) fn terminals_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    component: &Component,
    sheet: &SheetConnectivity,
) {
    let empty = Vec::new();
    let bindings = sheet.terminals.get(&component.id).unwrap_or(&empty);
    if bindings.is_empty() {
        return;
    }
    let open_pins = bindings.iter().filter(|(_, net)| net.is_none()).count();
    // The rows count themselves; the header speaks only for the open ones.
    let open = (open_pins > 0).then(|| format!("{open_pins} open"));
    section_header(ui, "Terminals", open.as_deref());
    let mut select: Option<&str> = None;
    for (pin, net) in bindings {
        let row = terminal_row(ui, pin, net.as_deref());
        match net {
            Some(name) => {
                if row.response.clicked() {
                    select = Some(name.as_str());
                }
                row.response
                    .on_hover_text(format!("Select net {name} on the sheet"));
            }
            None => {
                row.response
                    .on_disabled_hover_text("Unconnected pin · wire it or run connectivity checks");
            }
        }
    }
    if let Some(name) = select
        && let Some(net) = sheet.nets.iter().find(|net| net.name == name)
    {
        select_net(app, net);
    }
}

/// A device class's designer set of operating-point quantities, in the order
/// a designer reads them: the reported parameter, its row label, its unit.
type OpQuantitySet = &'static [(&'static str, &'static str, &'static str)];

/// MOS-like devices, field-effect transistors of every model family: the bias
/// point, then the small-signal parameters.
const MOS_OP_SET: OpQuantitySet = &[
    ("id", "Id", "A"),
    ("vgs", "Vgs", "V"),
    ("vds", "Vds", "V"),
    ("vth", "Vth", "V"),
    ("vdsat", "Vdsat", "V"),
    ("gm", "gm", "S"),
    ("gds", "gds", "S"),
];
const BJT_OP_SET: OpQuantitySet = &[
    ("ic", "Ic", "A"),
    ("vbe", "Vbe", "V"),
    ("vce", "Vce", "V"),
    ("gm", "gm", "S"),
    ("beta", "β", ""),
];
const DIODE_OP_SET: OpQuantitySet = &[
    ("id", "Id", "A"),
    ("vd", "Vd", "V"),
    ("power", "Power", "W"),
];

/// The designer set for a reported device family, or `None` for a family
/// without one, whose currents, terminal voltages and power are shown as the
/// report gives them.
fn op_quantity_set(device_kind: &str) -> Option<OpQuantitySet> {
    match device_kind {
        "MOSFET" | "MOS9" | "BSIM3" | "BSIM4" | "B3SOIFD" | "B3SOIDD" | "B3SOIPD" | "EKV26"
        | "EKV3" | "VDMOS" | "JFET" | "JFET2" | "JFET2_XYCE" | "MESFET" | "HFET1" | "HFET2" => {
            Some(MOS_OP_SET)
        }
        "BJT" => Some(BJT_OP_SET),
        "DIODE" => Some(DIODE_OP_SET),
        _ => None,
    }
}

/// The rows an operating-point entry shows: its region, its class's designer
/// set in engineering notation with units, and the temperature the run
/// solved it at. A quantity the report does not carry has no row.
pub(super) fn operating_point_rows(
    entry: &rspice_core::circuit::DeviceOpEntry,
) -> Vec<(String, String)> {
    use super::super::compact_readout;

    let value = |key: &str| {
        entry
            .params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(key))
            .map(|(_, value)| *value)
    };
    let mut rows = Vec::new();
    if let Some(region) = entry.region {
        rows.push(("Region".to_owned(), region.to_owned()));
    }
    match op_quantity_set(entry.device_kind) {
        Some(set) => {
            for (key, label, unit) in set {
                if let Some(value) = value(key) {
                    rows.push(((*label).to_owned(), compact_readout(value, unit)));
                }
            }
        }
        None => {
            for (name, value) in &entry.params {
                let unit = if *name == "power" {
                    "W"
                } else if name.starts_with('i') {
                    "A"
                } else if name.starts_with('v') && !name.ends_with("_intrinsic") {
                    "V"
                } else {
                    continue;
                };
                let mut label = (*name).to_owned();
                if let Some(first) = label.get_mut(..1) {
                    first.make_ascii_uppercase();
                }
                rows.push((label, compact_readout(*value, unit)));
            }
        }
    }
    // The device's own temperature, which is the run's: the setup may have
    // moved on since.
    if let Some(temperature) = value("temp") {
        rows.push((
            "Temperature".to_owned(),
            format!(
                "{} °C",
                crate::quantity::engineering::format_engineering_display_with(
                    temperature,
                    crate::quantity::EngineeringPrecision::Adaptive,
                )
            ),
        ));
    }
    rows
}

/// The engine's name for `component` at the occurrence being edited: the
/// card name on the top sheet, the dotted instance path below it. The report
/// names every device that way, so a bare name below the root would read the
/// operating point of a different instance of the same master.
pub(super) fn device_engine_name(state: &AppState, component: &Component) -> Option<String> {
    state
        .workspace
        .content
        .occurrence_path()
        .engine_name_for(&component.emitted_instance_name())
        .ok()
}

/// The active run's operating point for `component`, when the run solved one
/// for it. With none the section is absent.
pub(super) fn operating_point(ui: &mut Ui, app: &RSpiceApp, component: &Component) {
    let Some(engine_name) = device_engine_name(&app.state, component) else {
        return;
    };
    let Some(run) = app.state.simulation.active_run() else {
        return;
    };
    let Some(entry) = run
        .analyses
        .iter()
        .filter_map(|analysis| analysis.device_op.as_ref())
        .find_map(|report| {
            report
                .entries
                .iter()
                .find(|entry| entry.name.eq_ignore_ascii_case(&engine_name))
        })
    else {
        return;
    };
    if active_run_matches_design(&app.state) {
        section_header(ui, "Operating point", None);
    } else {
        section_header_explained(
            ui,
            "Operating point",
            "stale, rerun",
            &format!(
                "Run {} solved an earlier version of this schematic.",
                run.id
            ),
        );
    }
    for (label, value) in operating_point_rows(entry) {
        property_row(ui, &label, &value);
    }
}

/// This instance's own findings from a check of the drawing as it stands,
/// and its stress against the safe operating area when a run measured it.
/// With neither, the section is absent: a clean or unchecked instance is the
/// normal state, and the sheet's Checks section owns the check itself.
pub(super) fn component_checks(ui: &mut Ui, app: &RSpiceApp, component: &Component) {
    let findings = current_findings(&app.state, |violation| {
        violation_targets_component(violation, component)
    });
    let soa = retained_component_soa(&app.state, &component.name);
    if findings.is_empty() && soa.is_none() {
        return;
    }
    section_header(ui, "Checks", None);
    if !findings.is_empty() {
        property_row_problem(
            ui,
            "Connectivity",
            &count_label(findings.len(), "finding"),
            Some(findings_problem(&findings)),
        );
    }
    if let Some(soa) = soa {
        property_row_problem(ui, "Safe operating area", &soa.label, soa.problem);
    }
}

pub(super) fn violation_targets_component(violation: &DrcViolation, component: &Component) -> bool {
    // A terminal finding belongs to the component that owns the terminal. The
    // identity has to be read off the location rather than left to the related
    // items, because an instance with no authored reference is named by its id
    // in those and matches nothing.
    matches!(
        &violation.location,
        DrcLocation::Component { id, .. } | DrcLocation::Terminal { component_id: id, .. }
            if *id == component.id
    ) || violation
        .related_items
        .iter()
        .any(|item| item.eq_ignore_ascii_case(&component.name))
}

/// A safe-operating-area verdict as a row states it: the worst margin, or the
/// violation count, with a problem's mark when the stress is near or past a
/// limit. A pass is neutral.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ComponentSoaDisplay {
    pub(crate) label: String,
    pub(crate) problem: Option<Problem>,
}

/// Resolve only current, source-attributed SOA evidence for the selected
/// instance. A retained run from another project revision is deliberately
/// ignored; showing its margin beside a changed schematic would be unsafe.
pub(super) fn retained_component_soa(
    state: &AppState,
    component_name: &str,
) -> Option<ComponentSoaDisplay> {
    let run = state.simulation.active_run()?;
    let receipt = run.prepared_receipt()?;
    if receipt.project_revision() != state.workspace.content.project.revision() {
        return None;
    }

    let mut evaluations = Vec::new();
    let mut violations = 0usize;
    for analysis in &run.analyses {
        if !analysis.success || analysis.provenance.is_none() {
            continue;
        }
        let Some(AnalysisResultPayload::Soa {
            source_history: _,
            evaluations: retained,
            violations: retained_violations,
        }) = analysis.result_payload.as_ref()
        else {
            continue;
        };
        evaluations.extend(
            retained
                .iter()
                .filter(|evaluation| evaluation.device_id.eq_ignore_ascii_case(component_name)),
        );
        violations += retained_violations
            .iter()
            .filter(|violation| violation.device_id.eq_ignore_ascii_case(component_name))
            .count();
    }

    component_soa_display(&evaluations, violations)
}

/// The worst verdict one SOA analysis holds across every device it checked,
/// or `None` when the analysis carries no evaluations to judge.
pub(crate) fn analysis_soa_display(
    analysis: &crate::state::AnalysisResult,
) -> Option<ComponentSoaDisplay> {
    let Some(AnalysisResultPayload::Soa {
        evaluations,
        violations,
        ..
    }) = analysis.result_payload.as_ref()
    else {
        return None;
    };
    component_soa_display(&evaluations.iter().collect::<Vec<_>>(), violations.len())
}

/// The verdict a set of evaluations and violations amounts to: the violation
/// count when any limit was crossed, else the worst margin. `None` when there
/// is nothing to judge.
pub(super) fn component_soa_display(
    evaluations: &[&crate::state::SoaEvaluationEvidence],
    violations: usize,
) -> Option<ComponentSoaDisplay> {
    if violations > 0 {
        return Some(ComponentSoaDisplay {
            label: count_label(violations, "violation"),
            problem: Some(Problem::Failure),
        });
    }
    if evaluations.is_empty() {
        return None;
    }

    let verdict = evaluations
        .iter()
        .map(|evaluation| evaluation.verdict)
        .max()
        .unwrap_or(SoaRuleVerdictEvidence::Pass);
    let margin_percent = evaluations
        .iter()
        .filter_map(|evaluation| {
            let limit = evaluation.limit_value.abs();
            (limit > f64::EPSILON)
                .then_some((limit - evaluation.worst_actual_value.abs()) / limit * 100.0)
        })
        .min_by(f64::total_cmp);
    let (problem, without_margin) = match verdict {
        SoaRuleVerdictEvidence::Pass => (None, "within limits"),
        SoaRuleVerdictEvidence::Warning => (Some(Problem::Warning), "near a limit"),
        SoaRuleVerdictEvidence::Violation | SoaRuleVerdictEvidence::Critical => {
            (Some(Problem::Failure), "past a limit")
        }
    };
    Some(ComponentSoaDisplay {
        label: margin_percent.map_or_else(
            || without_margin.to_owned(),
            |margin| format!("{margin:.1}% margin"),
        ),
        problem,
    })
}
