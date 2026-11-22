//! The component inspector: identity, model binding, parameters, terminals.
//!
//! Every editable field validates before it commits and reports the rejection
//! in place, so an invalid value is never written and never silently reverted
//! either. Operating-point and SoA rows are shown only when the retained run
//! actually matches the current design — a stale result is reported as stale
//! rather than displayed as this component's state.

use super::*;
use crate::workbench::framework::command_registry::ids;

pub fn component_panel(
    ui: &mut Ui,
    cx: &mut Cx<'_, DesignFeature>,
    id: u64,
    sheet: &SheetConnectivity,
) {
    let Some(component) = cx
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .cloned()
    else {
        cx.state.inspector_yields_to_sheet = true;
        return;
    };
    let evidence = component_model_evidence(cx.model, &component);
    // The design root is implicit: on the top sheet the path says nothing.
    let occurrence = containing_occurrence_path(cx.model);
    object_title(
        ui,
        &component.name,
        Some(&component_title_kind(&component)),
        (occurrence != "/").then_some(occurrence.as_str()),
    );

    super::variant::variant_section(ui, cx, &component);
    identity_section(ui, cx, &component);
    terminals_section(ui, cx, &component, sheet);
    parameters_section(ui, cx, &component, &evidence);
    operating_point(ui, cx, &component);
    component_checks(ui, cx, &component);
}

/// The occurrence the inspected component sits in.
///
/// The title names the component beside this, so the path carries the
/// enclosing instances and stops there — and the design root is implicit, so an
/// inspected component on the top sheet reports `/`.
pub(super) fn containing_occurrence_path(model: &Model) -> String {
    model.workspace.content.occurrence_path().to_string()
}

/// What a component is, for its title: a built-in's catalog name, a generated
/// module's name, a cell instance's cell, else its device kind.
pub fn component_title_kind(component: &Component) -> String {
    let binding = component.library_cell.as_ref();
    if let Some(descriptor) = binding
        .filter(|binding| binding.is_builtin_xspice())
        .and_then(|binding| crate::state::validate_builtin_xspice_binding(binding).ok())
    {
        return descriptor.display_name.to_owned();
    }
    if let Some(descriptor) = binding
        .filter(|binding| binding.is_builtin_functional())
        .and_then(|binding| crate::state::validate_builtin_functional_binding(binding).ok())
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

pub fn identity_section(ui: &mut Ui, cx: &mut Cx<'_, DesignFeature>, component: &Component) {
    // On a read-only document the editor opens with every field read-only.
    if section_header_action(ui, "Identity", "Properties…", true).clicked() {
        crate::workbench::app::actions::property_edit::open_property_editor_in(
            &mut cx.state.property_form_parts(),
            cx.model,
            cx.wb,
            component.id,
        );
    }
    let instance_rejection = edit_row(ui, cx, component, InlineEditField::Instance, "Instance");
    field_rejection_slot(
        ui,
        cx.wb,
        component.id,
        &InlineEditField::Instance,
        instance_rejection.as_deref(),
    );
    // A built-in or a generated module runs the model its contract names, and
    // the Model row under Parameters states it; it has no value. A value that
    // is a property of the part's sheet is that property's row, under its
    // own name, among the parameters.
    let runs_contract = component
        .library_cell
        .as_ref()
        .is_some_and(|binding| binding.is_executable_builtin());
    if !runs_contract && !value_is_a_parameter(cx.state, cx.model, component) {
        let value_rejection = edit_row(ui, cx, component, InlineEditField::Value, "Value");
        field_rejection_slot(
            ui,
            cx.wb,
            component.id,
            &InlineEditField::Value,
            value_rejection.as_deref(),
        );
    }
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
    sheet_row(ui, cx.model, component.id);

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
            cx.model
                .library_manager
                .get_library(&reference.library)
                .and_then(|library| library.get_cell(&reference.cell))
                .and_then(|cell| cell.get_view(&reference.view))
                .is_some()
        });
    let symbol_writable = symbol_view.as_ref().is_some_and(|reference| {
        cx.model
            .library_manager
            .get_library(&reference.library)
            .is_some_and(|library| !library.read_only)
    });
    let model_source = component_model_source_target(cx.model, component);
    let hierarchy_master =
        crate::workbench::app::hierarchy_master_for_component(cx.model, component.id)
            .map(|(_, reference)| reference);
    // The deck the instance emits is part of its identity, so the jump to it
    // belongs beside the symbol, master and model routes rather than in a
    // section of its own. It appears wherever the command can be taken: the
    // jump generates the deck first and says so when it states no card.
    let netlist_card = cx.foreign_available(ids::SHOW_IN_NETLIST);
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
                    cx.state
                        .navigation_parts(cx.model, cx.wb, cx.runs)
                        .open_workspace_view(reference);
                }
            }

            if let Some(reference) = hierarchy_master.as_ref() {
                let label = format!("Descend into {}", reference.cell);
                if Button::new(&label).ghost().show(ui).clicked() {
                    cx.state
                        .navigation_parts(cx.model, cx.wb, cx.runs)
                        .open_selected_instance_master();
                }
            }

            if netlist_card && Button::new("Show in netlist").ghost().show(ui).clicked() {
                cx.execute(ids::SHOW_IN_NETLIST);
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
                    if !commit_pending_inspector_edit(cx.model, cx.wb) {
                        return;
                    }
                    match target {
                        ComponentModelSourceTarget::Catalog { library, model } => {
                            cx.wb.outbox.push(RevealModel {
                                library: library.clone(),
                                model: model.clone(),
                            });
                        }
                        ComponentModelSourceTarget::VerilogA(reference) => {
                            cx.state
                                .navigation_parts(cx.model, cx.wb, cx.runs)
                                .open_workspace_view(reference.clone());
                            cx.wb.activate(Workspace::Design);
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
    model: &Model,
    component: &Component,
) -> Option<ComponentModelSourceTarget> {
    if let Some(binding) = component.library_cell.as_ref() {
        let reference = CellViewRef::new(
            binding.library.clone(),
            binding.cell.clone(),
            binding.view.clone(),
        );
        let is_veriloga = model
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .is_some_and(|view| view.view_type.is_hdl_source());
        if is_veriloga {
            return Some(ComponentModelSourceTarget::VerilogA(reference));
        }
    }
    catalog_model_location(model, component)
        .map(|(library, model)| ComponentModelSourceTarget::Catalog { library, model })
}

pub fn catalog_model_location(model: &Model, component: &Component) -> Option<(String, String)> {
    if let Some(binding) = component.library_cell.as_ref()
        && let Some(library) = model.model_library_manager.get_library(&binding.library)
    {
        let candidates = [
            binding.module_name.as_deref(),
            Some(binding.cell.as_str()),
            (!component.value.trim().is_empty()).then_some(component.value.trim()),
        ];
        // A binned family opens at its first bin, as below.
        if let Some(model) = candidates
            .into_iter()
            .flatten()
            .find_map(|candidate| library.executable_cards(candidate).into_iter().next())
        {
            return Some((library.name.clone(), model.name.clone()));
        }
    }

    // The jump follows the provider a run would execute, so a contested
    // definition the project has already resolved still has a source to open.
    // Requiring exactly one name match made the action vanish precisely when
    // an engineer most needs to see which copy wins.
    let model_name = crate::state::explicit_component_model(component)?;
    let provider = model
        .model_library_manager
        .effective_plan_definition_provider(
            &model.sim_setup.model_bindings,
            crate::state::model_library::ModelConsumerScope::PrimitiveModel,
            &model_name,
        )
        .ok()
        .flatten()?;
    let library = model.model_library_manager.get_library(&provider.library)?;
    // A binned family opens at its first bin; which bin runs is the
    // simulator's choice from the instance's geometry.
    let model = library
        .executable_cards(&provider.definition)
        .into_iter()
        .next()?;
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
fn bound_model_spice_level(model: &Model, component: &Component) -> Option<u32> {
    let (library, name) = catalog_model_location(model, component)
        .or_else(|| foundation_default_model_location(model, component))?;
    let model = model
        .model_library_manager
        .get_library(&library)?
        .models
        .values()
        .find(|candidate| candidate.name == name)?;
    model.spice_level.or(match model.level {
        crate::state::model_library::ModelLevel::SpiceLevel1 => Some(1),
        crate::state::model_library::ModelLevel::SpiceLevel3 => Some(3),
        _ => None,
    })
}

/// Where the foundation card a bare bulk-MOS placement runs on lives.
fn foundation_default_model_location(
    model: &Model,
    component: &Component,
) -> Option<(String, String)> {
    let family = match component.kind {
        ComponentType::Nmos => rspice_core::library::FoundationDeviceFamily::Nmos,
        ComponentType::Pmos => rspice_core::library::FoundationDeviceFamily::Pmos,
        _ => return None,
    };
    let provider = model
        .model_library_manager
        .effective_plan_definition_provider(
            &model.sim_setup.model_bindings,
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
pub fn level_supported_instance_parameters(
    model: &Model,
    component: &Component,
) -> Option<&'static [&'static str]> {
    if !matches!(component.kind, ComponentType::Nmos | ComponentType::Pmos) {
        return None;
    }
    matches!(bound_model_spice_level(model, component)?, 1..=6)
        .then_some(CLASSIC_MOS_INSTANCE_PARAMETERS)
}

/// The model cards the project's libraries state for `module`, by name.
pub fn model_cards_of_module(model: &Model, module: &str) -> Vec<String> {
    let mut cards = model
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .flat_map(|library| library.bindable_models())
        .filter(|model| {
            model.cards().iter().all(|card| {
                card.spice_type
                    .as_deref()
                    .is_some_and(|name| name.eq_ignore_ascii_case(module))
            })
        })
        .map(|model| model.name().to_owned())
        .collect::<Vec<_>>();
    cards.sort_unstable_by_key(|card| card.to_ascii_lowercase());
    cards.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    cards
}

pub fn bound_model_choices(
    model: &Model,
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
    let Some(library) = model.model_library_manager.get_library(&binding.library) else {
        return Vec::new();
    };
    let Some(current_model) = library.executable_cards(current).into_iter().next() else {
        return Vec::new();
    };
    // A binned family is one choice, by the name an instance binds; its bins
    // are what the simulator picks from by L and W, not choices beside it.
    let mut models = library
        .bindable_models()
        .into_iter()
        .filter(|model| {
            model.cards().iter().all(|card| {
                crate::state::model_library::models_have_compatible_device_family(
                    current_model,
                    card,
                )
            })
        })
        .map(|model| {
            use crate::state::model_library::BindableModel;
            let label = match &model {
                BindableModel::Card(card) => card.name.clone(),
                BindableModel::Family(family) if family.bins.len() == 1 => {
                    format!("{} · 1 bin", family.name)
                }
                BindableModel::Family(family) => {
                    format!("{} · {} bins", family.name, family.bins.len())
                }
            };
            (model.name().to_owned(), label)
        })
        .collect::<Vec<_>>();
    // An instance already pinned to one bin keeps that bin as its current
    // choice, so the row says what it runs; choosing the family unpins it.
    if !models
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case(current))
        && let Some(family) = crate::state::model_library::model_bin_family(&current_model.name)
            .and_then(|family| library.bin_family(family))
    {
        models.push((
            current_model.name.clone(),
            format!("{} · bin of {}", current_model.name, family.name),
        ));
    }
    models.sort_by_key(|(name, _)| name.to_ascii_lowercase());
    models.dedup_by(|left, right| left.0.eq_ignore_ascii_case(&right.0));
    models
}

pub(super) fn bound_model_section_choices(
    model: &Model,
    component: &Component,
    selected_model: &str,
) -> Vec<(String, String)> {
    let Some(binding) = component.library_cell.as_ref() else {
        return Vec::new();
    };
    let Some(library) = model.model_library_manager.get_library(&binding.library) else {
        return Vec::new();
    };
    let mut sections = super::model_evidence::model_sections(library, selected_model);
    sections.sort_by_key(|section| section.to_ascii_lowercase());
    sections.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    let mut choices = vec![(String::new(), "default".to_owned())];
    choices.extend(
        sections
            .iter()
            .map(|section| (section.clone(), section.clone())),
    );
    // A section the model does not define stays the row's value, so the row
    // says what the instance holds, and is marked as one the model lacks; the
    // model's problem line says what that means.
    if let Some(section) = binding
        .model_section
        .as_deref()
        .filter(|section| !section.trim().is_empty())
        && !sections
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(section))
    {
        choices.push((section.to_owned(), format!("{section} \u{b7} not defined")));
    }
    choices
}

/// Bind one selected schematic instance from the Models & PDKs workbench
/// through the same guarded schematic transaction used by the inspector.
/// Library-cell instances retain their exact library identity; primitive
/// instances retain provider provenance as editor metadata, while the binding
/// is accepted only when it agrees with the project-global executable provider.
pub fn bind_component_model_from_catalog(
    cx: &mut Cx<'_, DesignFeature>,
    component_id: u64,
    library_name: &str,
    model_name: &str,
) -> Result<(), String> {
    commit_inline_component_edit(cx.model, cx.wb)?;
    crate::workbench::docks::inspector::validate_component_model_catalog_binding(
        cx.model,
        component_id,
        library_name,
        model_name,
    )?;
    let component = cx
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
        .ok_or_else(|| "The selected instance no longer exists.".to_owned())?;
    if let Some(binding) = component.library_cell.as_ref() {
        if !binding.library.eq_ignore_ascii_case(library_name) {
            return Err(format!(
                "The selected cell instance is bound to library '{}'; cross-library rebinding requires the Library Manager.",
                binding.library
            ));
        }
        apply_bound_model_choice(cx, component_id, model_name)?;
        return Ok(());
    }

    let effective = cx
        .model
        .model_library_manager
        .effective_plan_definition_provider(
            &cx.model.sim_setup.model_bindings,
            crate::state::model_library::ModelConsumerScope::PrimitiveModel,
            model_name,
        )?
        .ok_or_else(|| format!("Model '{model_name}' has no executable catalog provider."))?;
    if !effective.library.eq_ignore_ascii_case(library_name) {
        return Err(format!(
            "Model '{model_name}' executes from project-global provider '{}', not selected library '{library_name}'. Resolve the project-global provider before binding.",
            effective.library
        ));
    }

    let params =
        crate::state::params_string::set_parameter_value(&component.params, "model", model_name)
            .and_then(|text| {
                crate::state::params_string::set_parameter_value(
                    &text,
                    "model_library",
                    &effective.library,
                )
            })?;
    let mut candidate = component.clone();
    candidate.params = params;
    cx.model
        .edit_component_transaction(&component, candidate, "bind instance model")?;
    rspice_app_runs::workbench::preflight::invalidate(
        &mut cx.runs.preflight,
        &mut *cx.handles.run_controller,
    );
    Ok(())
}

pub fn apply_bound_model_choice(
    cx: &mut Cx<'_, DesignFeature>,
    component_id: u64,
    selected_model: &str,
) -> Result<bool, String> {
    commit_inline_component_edit(cx.model, cx.wb)?;
    let component = cx
        .model
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

    crate::workbench::docks::inspector::validate_component_model_catalog_binding(
        cx.model,
        component_id,
        &library_name,
        selected_model,
    )?;

    let library = cx
        .model
        .model_library_manager
        .get_library(&library_name)
        .ok_or_else(|| format!("Model library '{library_name}' is no longer loaded."))?;
    let candidate = library.bindable_model(selected_model).ok_or_else(|| {
        format!("Model '{selected_model}' is no longer present in library '{library_name}'.")
    })?;
    let candidate_name = candidate.name().to_owned();
    // The file holding every card the name can run, or the library's root
    // when a family's bins are spread over more than one.
    let cards = candidate.cards();
    let candidate_source = cards
        .iter()
        .all(|card| card.file_path == cards[0].file_path)
        .then(|| cards[0].file_path.clone())
        .flatten()
        .or_else(|| library.root_path.clone());
    let changed =
        cx.model
            .schematic
            .select_instance_model(component_id, candidate_name, candidate_source);
    if changed {
        rspice_app_runs::workbench::preflight::invalidate(
            &mut cx.runs.preflight,
            &mut *cx.handles.run_controller,
        );
    }
    Ok(changed)
}

pub fn apply_bound_model_section(
    cx: &mut Cx<'_, DesignFeature>,
    component_id: u64,
    selected_section: &str,
) {
    if !commit_pending_inspector_edit(cx.model, cx.wb) {
        return;
    }
    let Some(component) = cx
        .model
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
    let allowed = bound_model_section_choices(cx.model, &component, selected_model);
    if !allowed
        .iter()
        .any(|(value, _)| value.eq_ignore_ascii_case(selected_section))
    {
        return;
    }
    let selected = (!selected_section.trim().is_empty()).then(|| selected_section.to_owned());
    if cx
        .model
        .schematic
        .select_instance_model_section(component_id, selected)
    {
        rspice_app_runs::workbench::preflight::invalidate(
            &mut cx.runs.preflight,
            &mut *cx.handles.run_controller,
        );
    }
}

pub fn parameters_section(
    ui: &mut Ui,
    cx: &mut Cx<'_, DesignFeature>,
    component: &Component,
    evidence: &ComponentModelEvidence,
) {
    let editable = !cx.model.schematic_edit_read_only();
    section_header(ui, "Parameters", (!editable).then_some("read-only"));
    source_function_row(ui, cx, component, editable);
    // Every parameter the component editor offers is present, in its groups,
    // even on a freshly placed instance whose parameter string is still
    // empty. Values the sheet does not declare follow, and are never hidden.
    let declared = crate::state::parse_params_string(&component.params);
    let parameter_contract = inline_parameter_contract(cx.state, cx.model, component, &declared);
    // Where the sheet has a model row, it binds the model in its group, with
    // Browse beside it as in the component editor; otherwise the binding is
    // stated above the rows.
    let model_row = (!cx.model.instantiates_veriloga_view(component))
        .then(|| {
            parameter_contract
                .iter()
                .position(|row| !row.undeclared && row.key.eq_ignore_ascii_case("model"))
        })
        .flatten();
    if model_row.is_none() {
        model_binding(ui, cx, component, evidence, None);
    }

    let raw_field = InlineEditField::Parameters;
    let raw_editor_active = cx
        .wb
        .inline_edit
        .buffer_for(component.id, &raw_field)
        .is_some();
    if parameter_contract.is_empty() || raw_editor_active {
        let rejection = edit_row_with_hint(
            ui,
            cx,
            component,
            raw_field.clone(),
            "Parameters",
            "instance value",
            PropertyRowCaption::default(),
        );
        field_rejection_slot(ui, cx.wb, component.id, &raw_field, rejection.as_deref());
        return;
    }

    // The open groups and the values beside them, then the fold, as the
    // component editor lays them out: opening the fold moves nothing above
    // it.
    let (open, folded): (Vec<_>, Vec<_>) = parameter_contract
        .iter()
        .enumerate()
        .partition(|(_, row)| !row.advanced);
    let mut removed = None;
    let mut rows = |ui: &mut Ui,
                    cx: &mut Cx<'_, DesignFeature>,
                    rows: &[(usize, &InlineParameterContract)]| {
        let mut group = None;
        for (index, row) in rows {
            if group != Some(row.group.as_str()) {
                group = Some(row.group.as_str());
                if !row.group.is_empty() {
                    crate::workbench::design_system::subsection_header(ui, &row.group);
                }
            }
            if model_row == Some(*index) {
                model_binding(ui, cx, component, evidence, Some(row));
            } else if let Some(key) = parameter_row(ui, cx, component, row) {
                removed = Some(key);
            }
        }
    };
    rows(ui, cx, &open);
    if !folded.is_empty() && advanced_fold(ui, cx, component, &folded) {
        rows(ui, cx, &folded);
    }

    if let Some(key) = removed
        && commit_pending_inspector_edit(cx.model, cx.wb)
    {
        match cx
            .model
            .remove_undeclared_instance_parameter(component.id, &key)
        {
            Ok(true) => rspice_app_runs::workbench::preflight::invalidate(
                &mut cx.runs.preflight,
                &mut *cx.handles.run_controller,
            ),
            Ok(false) => {}
            Err(error) => cx.wb.prompt(format!(
                "{key} was not removed from {}: {error}",
                component.name
            )),
        }
    }
}

/// What an independent source's Function row is called.
pub const SOURCE_FUNCTION: &str = "Function";

/// The functions a source of `kind`'s quantity can take, each as the part it
/// is and the keyword its card carries; `None` for anything that is not an
/// independent source.
pub fn source_functions(kind: ComponentType) -> Option<Vec<(ComponentType, &'static str)>> {
    use rspice_design::stimulus_library::definition::{StimulusFamily, StimulusKind};
    StimulusFamily::of(kind)?;
    let quantity = StimulusKind::of(kind)?;
    Some(
        StimulusFamily::ALL
            .iter()
            .map(|family| (family.component_type(quantity), family.label()))
            .collect(),
    )
}

/// An independent source's function, as a choice that swaps it in place.
fn source_function_row(
    ui: &mut Ui,
    cx: &mut Cx<'_, DesignFeature>,
    component: &Component,
    editable: bool,
) {
    let Some(functions) = source_functions(component.kind) else {
        return;
    };
    let mut selected = format!("{:?}", component.kind);
    let choices = functions
        .iter()
        .map(|(kind, label)| (format!("{kind:?}"), (*label).to_owned()))
        .collect::<Vec<_>>();
    if property_row_combo(
        ui,
        SOURCE_FUNCTION,
        ("design-inspector-source-function", component.id),
        &mut selected,
        &choices,
        editable,
    ) && let Some((target, _)) = functions
        .iter()
        .find(|(kind, _)| format!("{kind:?}") == selected)
    {
        change_source_function(cx, component, *target);
    }
}

/// The part `component` becomes when its function changes to `target`.
///
/// A value carries over where the new function takes a property of the
/// same name, since a SPICE source card names a quantity the same way in
/// every function that has it: the AC magnitude and phase, the parasitics,
/// the noise switch and the delay `TD` a pulse and a sine share, and a
/// pulse's `V1` into an exponential's. The part's value is the new
/// function's primary property, taken from the old value when the two
/// share it and from a parameter of that name otherwise, so a DC source's
/// level becomes an AC source's DC bias and back. Anything the new function
/// does not take is dropped, which the change reports and an undo restores.
/// This is how Virtuoso's source cell keeps its common fields when its
/// source type changes. The name, position, wiring and purchasing fields
/// are the instance's and stay.
pub fn source_function_target(
    registry: &crate::properties::PropertyEditorSchema,
    component: &Component,
    target: ComponentType,
) -> crate::state::SchematicReplacementTargetSpec {
    use crate::properties::property_bridge::get_primary_property_name;
    let source_primary = get_primary_property_name(component.kind);
    let target_primary = get_primary_property_name(target);
    let declared = crate::state::parse_params_string(&component.params);
    let sheet = registry.get(target);
    let declares = |name: &str| sheet.is_some_and(|sheet| sheet.get(name).is_some());
    let value = if source_primary == target_primary && !component.value.trim().is_empty() {
        component.value.clone()
    } else {
        declared
            .get(target_primary)
            .filter(|value| !value.trim().is_empty())
            .cloned()
            .unwrap_or_else(|| target.default_value().to_owned())
    };
    let parameters = sheet
        .map(|sheet| {
            sheet
                .iter()
                .filter(|definition| {
                    definition.name != "name"
                        && definition.name != "symbol"
                        && !definition.name.eq_ignore_ascii_case(target_primary)
                })
                .map(|definition| {
                    let parameter =
                        crate::state::SchematicReplacementParameter::new(definition.name.clone());
                    let default = definition.default_value.display_string();
                    if definition.required && !default.is_empty() {
                        parameter.required().with_default(default)
                    } else {
                        parameter
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let carried_value = if source_primary != target_primary
        && declares(source_primary)
        && !component.value.trim().is_empty()
        && !declared.contains_key(source_primary)
    {
        rspice_design::schematic::replacement::format_replacement_parameters(&HashMap::from([(
            source_primary.to_owned(),
            component.value.trim().to_owned(),
        )]))
    } else {
        String::new()
    };
    let symbol = component.symbol_variant.clone().filter(|variant| {
        sheet
            .and_then(|sheet| sheet.get("symbol"))
            .is_some_and(|definition| {
                matches!(&definition.default_value, PropertyValue::Enum { options, .. }
                    if options.contains(variant))
            })
    });
    crate::state::SchematicReplacementTargetSpec::primitive(target)
        .with_value(value)
        .with_default_params(carried_value)
        .with_symbol_variant(symbol)
        .with_parameters(parameters)
}

/// Change a source's function in place, as one undo step: the same part on
/// the same wires, now with the new function's card.
pub fn change_source_function(
    cx: &mut Cx<'_, DesignFeature>,
    component: &Component,
    target: ComponentType,
) {
    if target == component.kind || !commit_pending_inspector_edit(cx.model, cx.wb) {
        return;
    }
    let spec = source_function_target(&cx.state.property_registry, component, target);
    // The old value goes when the new function has no property of its name.
    let value_kept = {
        use crate::properties::property_bridge::get_primary_property_name;
        let primary = get_primary_property_name(component.kind);
        component.value.trim().is_empty()
            || primary == get_primary_property_name(target)
            || cx
                .state
                .property_registry
                .get(target)
                .is_some_and(|sheet| sheet.get(primary).is_some())
    };
    cx.model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(component.id);
    let changed = cx
        .model
        .schematic
        .replacement_authority()
        .and_then(|authority| {
            cx.model
                .schematic
                .replace_selected_instance(&authority, &spec)
        });
    match changed {
        Ok(impact) => {
            rspice_app_runs::workbench::preflight::invalidate(
                &mut cx.runs.preflight,
                &mut *cx.handles.run_controller,
            );
            let function = source_functions(target)
                .and_then(|functions| {
                    functions
                        .into_iter()
                        .find(|(kind, _)| *kind == target)
                        .map(|(_, label)| label)
                })
                .unwrap_or_default();
            let dropped = impact.dropped_parameters + usize::from(!value_kept);
            if dropped > 0 {
                let (values, were) = if dropped == 1 {
                    ("value", "was")
                } else {
                    ("values", "were")
                };
                cx.wb.prompt(format!(
                    "{} is now a {function} source. {dropped} {values} that {function} does not take {were} dropped.",
                    component.name,
                ));
            }
        }
        Err(error) => cx.wb.prompt(format!(
            "The function of {} was not changed: {error}",
            component.name
        )),
    }
}

/// The model the instance runs, and what is wrong with it.
///
/// A Verilog-A instance names its module and binds a card typed for it.
/// Otherwise the instance binds a card: chosen from the cards that fit when
/// its library offers several; typed, with Browse beside it, where the sheet
/// has a model row (`row`, in its group); and stated where nothing here can
/// change it. The binding's problem is said under it,
/// and a card with library sections offers them after it.
fn model_binding(
    ui: &mut Ui,
    cx: &mut Cx<'_, DesignFeature>,
    component: &Component,
    evidence: &ComponentModelEvidence,
    row: Option<&InlineParameterContract>,
) {
    let editable = !cx.model.schematic_edit_read_only();
    // A Verilog-A instance runs its view's module, named here only when it is
    // not the cell the Library cell row already names. Its Model row binds a
    // card from the project's libraries; a library section is a model-library
    // idea the deck does not give a module.
    let runs_module = cx.model.instantiates_veriloga_view(component);
    let model_choices = evidence.model.as_deref().map_or_else(Vec::new, |model| {
        bound_model_choices(cx.model, component, model)
    });
    let mut selected_model = component
        .library_cell
        .as_ref()
        .and_then(|binding| binding.module_name.clone())
        .or_else(|| evidence.model.clone())
        .unwrap_or_default();
    let label = row.map_or("Model", |row| row.label.as_str());
    if runs_module {
        if let Some(module) = evidence.model.as_deref().filter(|module| {
            component
                .library_cell
                .as_ref()
                .is_none_or(|binding| !binding.cell.eq_ignore_ascii_case(module))
        }) {
            property_row(ui, "Module", module);
        }
        let mut bound_card = component
            .library_cell
            .as_ref()
            .and_then(|binding| binding.model_card.clone())
            .unwrap_or_default();
        let cards = evidence
            .model
            .as_deref()
            .map_or_else(Vec::new, |module| model_cards_of_module(cx.model, module));
        let listed = |name: &str| cards.iter().any(|card| card.eq_ignore_ascii_case(name));
        if !cards.is_empty() || !bound_card.is_empty() {
            let choices = std::iter::once((String::new(), "None".to_owned()))
                .chain(cards.iter().map(|card| (card.clone(), card.clone())))
                .chain(
                    (!bound_card.is_empty() && !listed(&bound_card))
                        .then(|| (bound_card.clone(), bound_card.clone())),
                )
                .collect::<Vec<_>>();
            if property_row_combo(
                ui,
                "Model",
                ("design-inspector-model", component.id),
                &mut bound_card,
                &choices,
                editable,
            ) && commit_pending_inspector_edit(cx.model, cx.wb)
                && cx.model.schematic.select_instance_model_card(
                    component.id,
                    (!bound_card.is_empty()).then(|| bound_card.clone()),
                )
            {
                rspice_app_runs::workbench::preflight::invalidate(
                    &mut cx.runs.preflight,
                    &mut *cx.handles.run_controller,
                );
            }
            if !bound_card.is_empty() && !listed(&bound_card) {
                crate::workbench::design_system::problem_line(
                    ui,
                    Problem::Failure,
                    &format!("Model {bound_card} not found in the loaded libraries"),
                );
            }
        }
    } else if model_choices.len() > 1 {
        if property_row_combo(
            ui,
            label,
            ("design-inspector-model", component.id),
            &mut selected_model,
            &model_choices,
            editable,
        ) && let Err(error) = apply_bound_model_choice(cx, component.id, &selected_model)
        {
            cx.wb.prompt(error);
        }
    } else if let Some(row) = row {
        let field = row.field();
        let (rejection, browse) = edit_row_with_action(
            ui,
            cx,
            component,
            field.clone(),
            &row.label,
            crate::workbench::design_system::PropertyRowAction {
                icon: crate::workbench::design_system::WorkbenchIcon::Models,
                label: BROWSE_MODELS,
                enabled: true,
                disabled_reason: None,
            },
        );
        if browse {
            browse_component_model(cx, component.id);
        }
        field_rejection_slot(ui, cx.wb, component.id, &field, rejection.as_deref());
    } else if let Some(model) = evidence.model.as_deref() {
        property_row(ui, "Model", model);
    }
    if let Some((problem, text)) = evidence.problem.as_ref() {
        crate::workbench::design_system::problem_line(ui, *problem, text);
    }

    let section_choices = if runs_module {
        Vec::new()
    } else {
        bound_model_section_choices(cx.model, component, &selected_model)
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
        apply_bound_model_section(cx, component.id, &selected_section);
    }
}

/// What the Browse beside a model's name is called.
pub const BROWSE_MODELS: &str = "Browse models";

/// Open the component editor on `component` with its model browser over it,
/// as the editor's own Browse beside the model does: the model picked there
/// fills the editor's Model field, and OK binds it as one undo step.
fn browse_component_model(cx: &mut Cx<'_, DesignFeature>, component: u64) {
    crate::workbench::app::actions::property_edit::open_property_editor_in(
        &mut cx.state.property_form_parts(),
        cx.model,
        cx.wb,
        component,
    );
    if cx.state.tabbed_property_dialog.open {
        cx.state.tabbed_property_dialog.browse_model();
    }
}

/// One parameter row, edited as the component editor edits its property, with
/// the reason a refused value was turned away under it. Returns the key of an
/// undeclared value the reader chose to remove.
fn parameter_row(
    ui: &mut Ui,
    cx: &mut Cx<'_, DesignFeature>,
    component: &Component,
    row: &InlineParameterContract,
) -> Option<String> {
    let editable = !cx.model.schematic_edit_read_only();
    let field = row.field();
    let caption = PropertyRowCaption {
        unit: row.unit.as_deref(),
        description: row.description.as_deref(),
    };
    if row.undeclared {
        let value = field_value(component, &field);
        let note = if cx.model.model_card_owns_parameter(component, &row.key) {
            MODEL_CARD_PARAMETER_NOTE
        } else {
            UNDECLARED_PARAMETER_NOTE
        };
        if editable {
            return property_row_flagged(
                ui,
                &row.label,
                &value,
                note,
                Some(Problem::Warning),
                REMOVE_UNDECLARED_PARAMETER,
            )
            .then(|| row.key.clone());
        }
        crate::workbench::design_system::property_row_problem(
            ui,
            &row.label,
            &format!("{value} {note}"),
            Some(Problem::Warning),
        );
        return None;
    }
    if !row.editable {
        let value = field_value(component, &field);
        let shown = if value.is_empty() { &row.hint } else { &value };
        if caption.unit.is_some() || caption.description.is_some() {
            property_row_captioned(ui, &row.label, shown, caption);
        } else {
            property_row(ui, &row.label, shown);
        }
        return None;
    }
    let rejection = match &row.editor {
        InlineEditor::Check if editable => check_row(ui, cx, component, row),
        InlineEditor::Choice(options) if editable => choice_row(ui, cx, component, row, options),
        _ => edit_row_with_hint(
            ui,
            cx,
            component,
            field.clone(),
            &row.label,
            &row.hint,
            caption,
        ),
    };
    field_rejection_slot(ui, cx.wb, component.id, &field, rejection.as_deref());
    None
}

/// Whether a yes/no value's text says yes, in any spelling the gate takes.
fn switch_is_on(text: &str) -> Option<bool> {
    match text.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

/// A yes/no parameter's row: the component editor's check box, which commits
/// when it is clicked.
fn check_row(
    ui: &mut Ui,
    cx: &mut Cx<'_, DesignFeature>,
    component: &Component,
    row: &InlineParameterContract,
) -> Option<String> {
    let field = row.field();
    let on = switch_is_on(&field_value(component, &field))
        .or_else(|| switch_is_on(&row.hint))
        .unwrap_or(false);
    let definition = PropertyDefinition::new(row.key.clone())
        .with_display_name(row.label.clone())
        .with_type(PropertyType::Boolean);
    let quantity_policy = cx.model.preferences.quantity_presentation_policy();
    let number_locale = cx.wb.number_locale;
    let output = ui
        .push_id((component.id, &field), |ui| {
            crate::workbench::design_system::property_row_control(ui, &row.label, |ui| {
                let width = ui.available_width();
                rspice_schematic_editor::property_values::render_value_editor(
                    ui,
                    &definition,
                    &PropertyValue::Boolean(on),
                    None,
                    width,
                    quantity_policy,
                    number_locale,
                )
            })
        })
        .inner;
    if let Some(PropertyValue::Boolean(chosen)) = output.changed {
        commit_field_now(
            cx,
            component.id,
            field.clone(),
            if chosen { "1" } else { "0" }.to_owned(),
        );
    }
    cx.wb
        .inline_edit
        .error_for(component.id, &field)
        .map(str::to_owned)
}

/// A parameter with a fixed set of options: a drop-down of them, which
/// commits when one is chosen. A value outside the set stays the row's value,
/// so the row says what the instance holds.
fn choice_row(
    ui: &mut Ui,
    cx: &mut Cx<'_, DesignFeature>,
    component: &Component,
    row: &InlineParameterContract,
    options: &[String],
) -> Option<String> {
    let field = row.field();
    let current = field_value(component, &field);
    let current = if current.trim().is_empty() {
        row.hint.clone()
    } else {
        current
    };
    let mut selected = options
        .iter()
        .find(|option| option.eq_ignore_ascii_case(current.trim()))
        .cloned()
        .unwrap_or_else(|| current.trim().to_owned());
    let mut choices = options
        .iter()
        .map(|option| (option.clone(), option.clone()))
        .collect::<Vec<_>>();
    if !selected.is_empty() && !options.contains(&selected) {
        choices.push((selected.clone(), selected.clone()));
    }
    if property_row_combo(
        ui,
        &row.label,
        ("design-inspector-choice", component.id, &row.key),
        &mut selected,
        &choices,
        true,
    ) {
        commit_field_now(cx, component.id, field.clone(), selected);
    }
    cx.wb
        .inline_edit
        .error_for(component.id, &field)
        .map(str::to_owned)
}

/// The Advanced fold over `folded`, as the component editor folds them:
/// closed until opened, and open while one of its fields is being typed into
/// or holds a refused value, so a field that needs fixing is never folded
/// away. Its state is kept per kind of part. Returns whether it is open.
fn advanced_fold(
    ui: &mut Ui,
    cx: &Cx<'_, DesignFeature>,
    component: &Component,
    folded: &[(usize, &InlineParameterContract)],
) -> bool {
    let id = egui::Id::new(("design-inspector-advanced", component.kind));
    let needs_open = folded.iter().any(|(_, row)| {
        let field = row.field();
        cx.wb.inline_edit.is_editing(component.id, &field)
            || cx.wb.inline_edit.error_for(component.id, &field).is_some()
    });
    let shown = ui.data(|data| data.get_temp::<bool>(id)).unwrap_or(false) || needs_open;
    let mut open = shown;
    crate::workbench::design_system::section_block_gap(ui);
    // The disclosure's triangle starts on the column the group headings
    // start on, as a heading that opens.
    let inset = crate::workbench::design_system::panel_inset(ui);
    let indent = (inset - 1.0).max(0.0);
    let width = (ui.available_width() - indent - inset).max(1.0);
    let height = Tokens::get(ui.ctx()).metrics.row_h;
    let rect = egui::Rect::from_min_size(
        ui.cursor().min + egui::vec2(indent, 0.0),
        egui::vec2(width, height),
    );
    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        rspice_schematic_editor::component_properties::advanced_disclosure(
            ui,
            folded.len(),
            &mut open,
        );
    });
    if open != shown {
        ui.data_mut(|data| data.insert_temp(id, open));
    }
    open
}

/// The strip under one editable row that says why its value is refused:
/// while the row is being typed into, the reason the text is refused, and
/// after, the reason the value set aside in the row was. It is under the row
/// it speaks of, not at the foot of the section, and takes no room while
/// there is nothing to say.
fn field_rejection_slot(
    ui: &mut Ui,
    wb: &WorkbenchSession,
    component: u64,
    field: &InlineEditField,
    rejection: Option<&str>,
) {
    let live = wb
        .inline_edit
        .session()
        .filter(|_| wb.inline_edit.is_editing(component, field))
        .and_then(|session| session.error.as_deref());
    rejection_slot(ui, false, live.or(rejection));
}

/// What an undeclared parameter's row says after its value.
pub const UNDECLARED_PARAMETER_NOTE: &str = "not in module";
/// What the row of a value says when the instance's model card owns it.
pub const MODEL_CARD_PARAMETER_NOTE: &str = "model only";
/// The action that takes an undeclared parameter's value off the instance.
pub const REMOVE_UNDECLARED_PARAMETER: &str = "Remove";

/// Whether `component` takes the SPICE instance temperature `temp`.
///
/// A built-in takes it when its compiled contract names it. An instance of a
/// Verilog-A view never does: a module reads `$temperature`, and takes `temp`
/// only as a parameter it declares, which its own typed row then offers.
pub fn supports_instance_temperature(model: &Model, component: &Component) -> bool {
    if model.instantiates_veriloga_view(component) {
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

/// SPICE instance-temperature parameter.
pub const TEMPERATURE_PARAM: &str = "temp";

/// The heading over the values an instance sets that its sheet does not
/// declare.
pub const OTHER_PARAMETERS: &str = "Other parameters";

/// How a parameter row is edited: as the component editor edits the same
/// property.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineEditor {
    /// Typed text, held to the property's type and range.
    Text,
    /// A yes/no property: a check box.
    Check,
    /// One of a fixed set of options: a drop-down.
    Choice(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineParameterContract {
    pub key: String,
    pub label: String,
    pub hint: String,
    pub editable: bool,
    /// The unit the sheet states the value in.
    pub unit: Option<String>,
    /// What the sheet says the parameter is, shown on the label's hover.
    pub description: Option<String>,
    /// A value the instance sets for a parameter its compiled module no
    /// longer declares: shown, flagged, and offered for removal, because the
    /// netlister refuses it rather than emitting or dropping it.
    pub undeclared: bool,
    /// The heading the row stands under: its category on the sheet, or
    /// [`OTHER_PARAMETERS`] for a value the sheet does not declare. Empty
    /// when the rows have no sheet to group them.
    pub group: String,
    /// Folded under Advanced, as the component editor folds it.
    pub advanced: bool,
    /// The row is the part's value: the sheet's primary property, which the
    /// part stores as its value rather than as a parameter.
    pub primary: bool,
    pub editor: InlineEditor,
}

impl InlineParameterContract {
    /// The inline field the row edits.
    #[must_use]
    pub fn field(&self) -> InlineEditField {
        if self.primary {
            InlineEditField::Value
        } else {
            InlineEditField::Parameter(self.key.clone())
        }
    }
}

/// The property sheet the inspector reads for `component`: the family's, or
/// a cell instance's own contract. Borrowed where the registry holds it, as
/// it is read every frame.
fn inspector_property_sheet<'a>(
    design: &'a DesignState,
    model: &Model,
    component: &Component,
) -> Option<std::borrow::Cow<'a, crate::state::PropertySheet>> {
    if component.kind == ComponentType::CellInstance {
        crate::workbench::app::authoritative_component_property_sheet(
            &design.property_registry,
            model,
            component,
        )
        .ok()
        .flatten()
        .map(std::borrow::Cow::Owned)
    } else {
        design
            .property_registry
            .get(component.kind)
            .map(std::borrow::Cow::Borrowed)
    }
}

/// The sheet property the part's value stands for, when the parameter rows
/// edit it under its own name and unit: a resistor's resistance, a sine
/// source's offset, a transistor's model. A cell instance's value, a
/// built-in's contract and a ground's name are not properties of a sheet,
/// and the Identity group keeps the plain Value row for them.
fn primary_value_definition<'a>(
    sheet: &'a crate::state::PropertySheet,
    component: &Component,
) -> Option<&'a PropertyDefinition> {
    let runs_contract = component
        .library_cell
        .as_ref()
        .is_some_and(|binding| binding.is_executable_builtin());
    if runs_contract || component.kind == ComponentType::CellInstance {
        return None;
    }
    let primary = crate::properties::property_bridge::get_primary_property_name(component.kind);
    if primary == "name" {
        return None;
    }
    sheet
        .get(primary)
        .filter(|definition| definition.display_mode != DisplayMode::Hidden)
}

/// Whether the part's value is edited in the parameter rows, under its
/// property's name, rather than as a Value row of the Identity group.
pub fn value_is_a_parameter(design: &DesignState, model: &Model, component: &Component) -> bool {
    inspector_property_sheet(design, model, component)
        .is_some_and(|sheet| primary_value_definition(&sheet, component).is_some())
}

/// How the component editor edits `definition`.
fn inline_editor(definition: &PropertyDefinition) -> InlineEditor {
    match (&definition.prop_type, &definition.default_value) {
        (PropertyType::Boolean, _) => InlineEditor::Check,
        (PropertyType::Enum, PropertyValue::Enum { options, .. }) if !options.is_empty() => {
            InlineEditor::Choice(options.clone())
        }
        _ => InlineEditor::Text,
    }
}

/// The inspector's parameter rows for `component`, in the order and groups
/// the component editor shows them.
///
/// The rows the sheet declares come from
/// [`property_form`](rspice_schematic_editor::component_properties::property_form),
/// the function the component editor's form is built by, so the two offer
/// the same fields under the same headings and fold the same ones under
/// Advanced: a field waiting on a model, or one the bound card's level does
/// not read, is in neither. The part's value is the row of its primary
/// property. A value the instance sets that the sheet does not declare, or
/// whose row the sheet keeps hidden, follows under [`OTHER_PARAMETERS`],
/// since it is the instance's own and is never hidden.
pub fn inline_parameter_contract(
    design: &DesignState,
    model: &Model,
    component: &Component,
    declared: &HashMap<String, String>,
) -> Vec<InlineParameterContract> {
    let primary = crate::properties::property_bridge::get_primary_property_name(component.kind);
    // Where the instance takes the SPICE instance temperature, an empty
    // `temp` inherits the reference PVT point, and its row says so.
    let temperature_row = supports_instance_temperature(model, component);
    let is_temperature = |key: &str| temperature_row && key.eq_ignore_ascii_case(TEMPERATURE_PARAM);
    let reference = model.sim_setup.reference_pvt.temperature_celsius;
    let inherited = || format!("inherit · {reference} °C");
    let compiled = model.has_compiled_parameter_sheet(component);
    let sheet = inspector_property_sheet(design, model, component);
    let value_definition = sheet
        .as_deref()
        .and_then(|sheet| primary_value_definition(sheet, component));
    let is_value = |name: &str| value_definition.is_some_and(|value| value.name == name);
    let row = |definition: &PropertyDefinition, group: &str, advanced: bool| {
        let unit = definition.unit.clone().filter(|unit| !unit.is_empty());
        let hint = if is_temperature(&definition.name) {
            // A row that shows its unit says it once.
            if unit.is_some() {
                format!("inherit · {reference}")
            } else {
                inherited()
            }
        } else {
            definition
                .default_hint
                .clone()
                .unwrap_or_else(|| match &definition.default_value {
                    PropertyValue::Expression(source) => source.clone(),
                    value => value.display_string(),
                })
        };
        InlineParameterContract {
            key: definition.name.clone(),
            label: definition.display_name.clone(),
            hint,
            editable: !definition.read_only && definition.display_mode != DisplayMode::Readonly,
            unit,
            description: Some(definition.description.clone())
                .filter(|description| !description.trim().is_empty()),
            undeclared: false,
            group: group.to_owned(),
            advanced,
            primary: is_value(&definition.name),
            editor: inline_editor(definition),
        }
    };

    let mut rows = Vec::new();
    let mut seen = HashSet::new();
    if let Some(sheet) = sheet.as_deref() {
        let values = rspice_design::properties::component::collect_properties_with_sheet(
            component,
            Some(sheet),
        );
        let form = rspice_schematic_editor::component_properties::property_form(
            sheet,
            component.kind,
            &values,
            level_supported_instance_parameters(model, component),
        );
        // A PWL source's points are a table of their own in the component
        // editor, so its form leaves them out; here they are the value row
        // of their group.
        let mut value_pending =
            value_definition.filter(|value| !form.rows().any(|row| row.name == value.name));
        for (groups, advanced) in [(&form.groups, false), (&form.advanced, true)] {
            for group in groups {
                if let Some(value) =
                    value_pending.take_if(|value| !advanced && value.category == group.category)
                {
                    seen.insert(value.name.to_ascii_lowercase());
                    rows.push(row(value, &group.category, false));
                }
                for definition in &group.rows {
                    // A value that is not a sheet property has its own row
                    // in the Identity group.
                    if definition.name.eq_ignore_ascii_case(primary) && !is_value(&definition.name)
                    {
                        continue;
                    }
                    seen.insert(definition.name.to_ascii_lowercase());
                    rows.push(row(definition, &group.category, advanced));
                }
            }
        }
        if let Some(value) = value_pending {
            seen.insert(value.name.to_ascii_lowercase());
            rows.insert(0, row(value, &value.category, false));
        }
    }

    // A compiled module's sheet is its current contract; the placement's
    // copy of an older one names nothing the sheet does not.
    let mut others = Vec::new();
    if let Some(binding) = component.library_cell.as_ref().filter(|_| !compiled) {
        for key in &binding.parameter_order {
            let normalized = key.to_ascii_lowercase();
            if normalized == primary || !seen.insert(normalized) {
                continue;
            }
            let temperature = is_temperature(key);
            others.push(InlineParameterContract {
                key: key.clone(),
                label: if temperature {
                    "Temperature".to_owned()
                } else {
                    key.clone()
                },
                hint: if temperature {
                    inherited()
                } else {
                    "inherit".to_owned()
                },
                editable: true,
                unit: None,
                description: None,
                undeclared: false,
                group: String::new(),
                advanced: false,
                primary: false,
                editor: InlineEditor::Text,
            });
        }
    }

    let mut extensions = declared
        .keys()
        .filter(|key| {
            !key.eq_ignore_ascii_case(primary) && !seen.contains(&key.to_ascii_lowercase())
        })
        .cloned()
        .collect::<Vec<_>>();
    extensions.sort_unstable_by_key(|key| key.to_ascii_lowercase());
    others.extend(extensions.into_iter().map(|key| {
        // A value the sheet declares but its form does not show keeps the
        // sheet's name for it.
        match sheet.as_deref().and_then(|sheet| sheet.get(&key)) {
            Some(definition) if !compiled => row(definition, "", false),
            _ => InlineParameterContract {
                label: if is_temperature(&key) {
                    "Temperature".to_owned()
                } else {
                    key.clone()
                },
                hint: if is_temperature(&key) {
                    inherited()
                } else {
                    "instance value".to_owned()
                },
                key,
                editable: !compiled,
                unit: None,
                description: None,
                undeclared: compiled,
                group: String::new(),
                advanced: false,
                primary: false,
                editor: InlineEditor::Text,
            },
        }
    }));
    // Values beside the sheet's groups are headed as their own group; with
    // no sheet they are the whole list and need no heading.
    if !rows.is_empty() {
        for other in &mut others {
            OTHER_PARAMETERS.clone_into(&mut other.group);
        }
    }
    rows.extend(others);
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

pub fn terminals_section(
    ui: &mut Ui,
    cx: &mut Cx<'_, DesignFeature>,
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
        select_net(cx, net);
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
    policy: crate::quantity::QuantityPresentationPolicy,
) -> Vec<(String, String)> {
    use crate::workbench::docks::inspector::compact_readout;

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
            crate::quantity::temperature_label(temperature, policy),
        ));
    }
    rows
}

/// The engine's name for `component` at the occurrence being edited: the
/// element the engine knows on the top sheet, the dotted instance path below
/// it. The report names every device that way, so a bare name below the root
/// would read the operating point of a different instance of the same master.
pub(super) fn device_engine_name(model: &Model, component: &Component) -> Option<String> {
    model
        .workspace
        .content
        .occurrence_path()
        .engine_name_for(&component.engine_element_name())
        .ok()
}

/// The active run's operating point for `component`, when the run solved one
/// for it. With none the section is absent.
pub(super) fn operating_point(ui: &mut Ui, cx: &Cx<'_, DesignFeature>, component: &Component) {
    let Some(engine_name) = device_engine_name(cx.model, component) else {
        return;
    };
    let Some(run) = cx.model.simulation.active_run() else {
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
    if active_run_matches_design(cx.model) {
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
    let policy = cx.model.preferences.quantity_presentation_policy();
    for (label, value) in operating_point_rows(entry, policy) {
        property_row(ui, &label, &value);
    }
}

/// This instance's own findings from a check of the drawing as it stands,
/// and its stress against the safe operating area when a run measured it.
/// With neither, the section is absent: a clean or unchecked instance is the
/// normal state, and the sheet's Checks section owns the check itself.
pub fn component_checks(ui: &mut Ui, cx: &Cx<'_, DesignFeature>, component: &Component) {
    let findings = current_findings(cx.model, |violation| {
        violation_targets_component(violation, component)
    });
    let soa = retained_component_soa(cx.model, &component.name);
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
    // A finding inside an instantiated cellview names that cellview's
    // components, and `amp`'s `R1` is not this sheet's `R1` however alike the
    // two references read.
    if matches!(violation.location, DrcLocation::InsideInstance { .. }) {
        return false;
    }
    // A terminal finding belongs to the component that owns the terminal, and
    // so does a finding that involves the component or one of its terminals:
    // a loop through it, a name it shares, an output of it on a shorted net.
    // Each is matched by identity, never by name, so an instance with no
    // authored reference is found and two instances that share a name are
    // told apart.
    let this = |location: &DrcLocation| {
        matches!(
            location,
            DrcLocation::Component { id, .. } | DrcLocation::Terminal { component_id: id, .. }
                if *id == component.id
        )
    };
    this(&violation.location)
        || violation
            .related
            .iter()
            .any(|related| this(&related.location))
}

/// Resolve only current, source-attributed SOA evidence for the selected
/// instance. A retained run from another project revision is deliberately
/// ignored; showing its margin beside a changed schematic would be unsafe.
pub(super) fn retained_component_soa(
    model: &Model,
    component_name: &str,
) -> Option<ComponentSoaDisplay> {
    let run = model.simulation.active_run()?;
    let receipt = run.prepared_receipt()?;
    if receipt.project_revision() != model.workspace.content.project.revision() {
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
