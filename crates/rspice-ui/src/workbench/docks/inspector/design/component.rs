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
        .components
        .iter()
        .find(|component| component.id == id)
        .cloned()
    else {
        sheet_panel(ui, app, &sheet.nets);
        return;
    };
    let evidence = component_model_evidence(&app.state, &component);
    let editable = !app.state.schematic_edit_read_only();
    let builtin_descriptor = component
        .library_cell
        .as_ref()
        .filter(|binding| binding.is_builtin_xspice())
        .and_then(|binding| crate::state::validate_builtin_xspice_binding(binding).ok());
    let builtin_model = component
        .library_cell
        .as_ref()
        .and_then(|binding| binding.builtin_xspice.as_ref())
        .map(|contract| contract.model_type.as_str());
    let generated_descriptor = component
        .library_cell
        .as_ref()
        .filter(|binding| binding.is_generated_veriloga())
        .and_then(|binding| crate::state::validate_generated_veriloga_binding(binding).ok());

    hero(
        ui,
        app,
        Hero {
            preview: HeroPreview::Symbol(component.kind),
            eyebrow: format!(
                "{} · {}",
                component.name,
                containing_occurrence_path(&app.state)
            ),
            title: if let Some(descriptor) = builtin_descriptor {
                descriptor.display_name.to_owned()
            } else if let Some(descriptor) = generated_descriptor {
                descriptor.model_name.to_owned()
            } else if component.value.trim().is_empty() {
                component.kind.display_name().to_owned()
            } else {
                component.value.clone()
            },
            subtitle: builtin_model.map_or_else(
                || {
                    generated_descriptor.map_or_else(
                        || component.kind.display_name().to_owned(),
                        |descriptor| format!("Generated Verilog-A · {}", descriptor.module_name),
                    )
                },
                |model| format!("Built-in XSPICE · {model}"),
            ),
            statuses: vec![
                (
                    evidence.status.clone(),
                    evidence.tone.color(&Tokens::get(ui.ctx())),
                ),
                (
                    app.state
                        .sim_setup
                        .reference_pvt
                        .process
                        .short_name()
                        .to_owned(),
                    Tokens::get(ui.ctx()).color.text_dim,
                ),
            ],
            open_properties: editable.then_some(component.id),
        },
    );

    identity_section(ui, app, &component);
    terminals_section(ui, app, &component, sheet);
    parameters_section(ui, app, &component, &evidence);
    operating_point(ui, app, &component);
    component_checks(ui, app, &component);
}

/// The occurrence the inspected component sits in.
///
/// The hero names the component beside this, so the path carries the enclosing
/// instances and stops there — and the design root is implicit, so an inspected
/// component on the top sheet reports `/`.
pub(super) fn containing_occurrence_path(state: &AppState) -> String {
    state.workspace.occurrence_path().to_string()
}

pub(super) fn component_view_contract(component: &Component) -> String {
    component.library_cell.as_ref().map_or_else(
        || "symbol · spice".to_owned(),
        |binding| {
            let view = binding.view.trim();
            if view.is_empty() {
                "symbol · spice".to_owned()
            } else if view.eq_ignore_ascii_case("spice") {
                "spice".to_owned()
            } else {
                format!("{view} · spice")
            }
        },
    )
}

pub(super) fn identity_section(ui: &mut Ui, app: &mut RSpiceApp, component: &Component) {
    let editable = !app.state.schematic_edit_read_only();
    let properties = schematic_section_header_action(ui, "Identity", "Properties…", editable);
    if properties.clicked() {
        crate::workbench::app::open_property_editor(&mut app.state, component.id);
    }
    properties.on_disabled_hover_text("the active document is read-only");
    let instance_rejection = edit_row(ui, app, component, InlineEditField::Instance, "Instance");
    let value_rejection = if let Some(contract) = component
        .library_cell
        .as_ref()
        .and_then(|binding| binding.builtin_xspice.as_ref())
    {
        property_row(ui, "Executable model", &contract.model_type);
        None
    } else if let Some(contract) = component
        .library_cell
        .as_ref()
        .and_then(|binding| binding.generated_veriloga.as_ref())
    {
        property_row(ui, "Executable model", &contract.model_name);
        None
    } else {
        edit_row(ui, app, component, InlineEditField::Value, "Value")
    };
    let library_cell = component.library_cell.as_ref().map_or_else(
        || format!("primitives/{}", component.kind.display_name()),
        |binding| format!("{}/{}", binding.library, binding.cell),
    );
    property_row(ui, "Library cell", &library_cell);
    property_row(ui, "View", &component_view_contract(component));
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
                let response = Button::new("Open model source…").ghost().show(ui);
                if response.clicked() {
                    match target {
                        ComponentModelSourceTarget::Catalog { library, model } => {
                            app.state
                                .model_library_manager
                                .select_library(library.as_str());
                            app.state.workbench.selected_model = Some(model.clone());
                            app.state.workbench.models_page = ModelsPage::Models;
                            app.state.workbench.activate(Workspace::Models);
                        }
                        ComponentModelSourceTarget::VerilogA(reference) => {
                            app.state.open_workspace_view(reference.clone());
                            app.state.ui.code_workspace.page =
                                crate::workbench::documents::code_workspace::CodeWorkspacePage::VerilogA;
                            app.state.workbench.activate(Workspace::Netlist);
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

    let model_name = super::super::explicit_component_model(component)?;
    let matches = state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .filter_map(|library| {
            library
                .models
                .values()
                .find(|model| model.name.eq_ignore_ascii_case(&model_name))
                .map(|model| (library.name.clone(), model.name.clone()))
        })
        .collect::<Vec<_>>();
    (matches.len() == 1).then(|| matches[0].clone())
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
    let component = app
        .state
        .schematic
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
    let before = crate::state::SchematicSnapshot::capture(&app.state.schematic);
    let component = app
        .state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.id == component_id)
        .expect("the validated component remains present until mutation");
    let binding = component
        .library_cell
        .as_mut()
        .expect("the validated library binding remains present until mutation");
    let mut changed = binding.module_name.as_deref() != Some(candidate_name.as_str());
    binding.module_name = Some(candidate_name);
    if candidate_source.is_some() && binding.source_path != candidate_source {
        binding.source_path = candidate_source;
        binding.model_section = None;
        changed = true;
    }
    if changed {
        app.state.schematic.is_dirty = true;
        app.state.schematic.bump_topology_version();
        app.state
            .schematic
            .commit_undo_from(before, "select instance model");
        app.invalidate_simulation_preflight();
    }
    Ok(changed)
}

pub(super) fn apply_bound_model_section(
    app: &mut RSpiceApp,
    component_id: u64,
    selected_section: &str,
) {
    let Some(component) = app
        .state
        .schematic
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
    let before = crate::state::SchematicSnapshot::capture(&app.state.schematic);
    let Some(binding) = app
        .state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.id == component_id)
        .and_then(|component| component.library_cell.as_mut())
    else {
        return;
    };
    if binding.model_section == selected {
        return;
    }
    binding.model_section = selected;
    app.state.schematic.is_dirty = true;
    app.state.schematic.bump_topology_version();
    app.state
        .schematic
        .commit_undo_from(before, "select instance model section");
    app.invalidate_simulation_preflight();
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
        Some(if editable { "editable" } else { "read-only" }),
    );
    let model_choices = bound_model_choices(&app.state, component, &evidence.model);
    let mut selected_model = component
        .library_cell
        .as_ref()
        .and_then(|binding| binding.module_name.clone())
        .unwrap_or_else(|| evidence.model.clone());
    if model_choices.len() > 1 {
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
    } else {
        property_row(ui, "Model", &evidence.model);
    }

    let section_choices = bound_model_section_choices(&app.state, component, &selected_model);
    let mut selected_section = component
        .library_cell
        .as_ref()
        .and_then(|binding| binding.model_section.clone())
        .unwrap_or_default();
    if section_choices.len() > 1 {
        if property_row_combo(
            ui,
            "Section",
            ("design-inspector-model-section", component.id),
            &mut selected_section,
            &section_choices,
            editable,
        ) {
            apply_bound_model_section(app, component.id, &selected_section);
        }
    } else {
        property_row(ui, "Section", &evidence.section);
    }

    // Instance temperature is the `temp` parameter. Left empty it inherits
    // the reference PVT point, which the row reports as its placeholder.
    let declared = crate::state::parse_params_string(&component.params);
    let inherited = format!(
        "inherit · {} °C",
        app.state.sim_setup.reference_pvt.temperature_celsius
    );
    let supports_temperature = component
        .library_cell
        .as_ref()
        .filter(|binding| binding.is_executable_builtin())
        .is_none_or(|binding| {
            binding
                .parameter_order
                .iter()
                .any(|name| name.eq_ignore_ascii_case(TEMPERATURE_PARAM))
        });
    let mut rejection = supports_temperature
        .then(|| {
            edit_row_with_hint(
                ui,
                app,
                component,
                InlineEditField::Parameter(TEMPERATURE_PARAM.to_owned()),
                "Temperature",
                &inherited,
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
    for parameter in parameter_contract {
        if parameter.editable {
            let row_rejection = edit_row_with_hint(
                ui,
                app,
                component,
                InlineEditField::Parameter(parameter.key),
                &parameter.label,
                &parameter.hint,
            );
            rejection = rejection.or(row_rejection);
        } else {
            let value = declared
                .get(&parameter.key)
                .map_or(parameter.hint.as_str(), String::as_str);
            property_row(ui, &parameter.label, value);
        }
    }
    if editable {
        rejection_slot(
            ui,
            editing_parameter_field(&app.state, component.id),
            rejection.as_deref(),
        );
    }
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
}

pub(super) fn inline_parameter_contract(
    state: &AppState,
    component: &Component,
    declared: &HashMap<String, String>,
) -> Vec<InlineParameterContract> {
    let primary = crate::properties::property_bridge::get_primary_property_name(component.kind);
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
                && definition.name != TEMPERATURE_PARAM
        }) {
            let hint = match &definition.default_value {
                PropertyValue::Expression(source) => source.clone(),
                value => value.display_string(),
            };
            seen.insert(definition.name.to_ascii_lowercase());
            rows.push(InlineParameterContract {
                key: definition.name.clone(),
                label: definition.display_name.clone(),
                hint,
                editable: !definition.read_only && definition.display_mode != DisplayMode::Readonly,
            });
        }
    }

    if let Some(binding) = component.library_cell.as_ref() {
        for key in &binding.parameter_order {
            let normalized = key.to_ascii_lowercase();
            if normalized == primary || normalized == TEMPERATURE_PARAM || !seen.insert(normalized)
            {
                continue;
            }
            rows.push(InlineParameterContract {
                key: key.clone(),
                label: key.clone(),
                hint: "inherit".to_owned(),
                editable: true,
            });
        }
    }

    let mut extensions = declared
        .keys()
        .filter(|key| {
            key.as_str() != TEMPERATURE_PARAM
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
        editable: true,
    }));
    rows
}

/// Leading inset of a terminal row's status icon, matching the section
/// header's title and the property rows' label column.
pub(super) const TERMINAL_ROW_PAD_X: f32 = 10.0;
/// Icon box, then the gap before the pin name.
pub(super) const TERMINAL_ROW_ICON_W: f32 = 15.0;
pub(super) const TERMINAL_ROW_ICON_GAP: f32 = 6.0;
/// Where a terminal row's pin name starts.
pub(super) const TERMINAL_ROW_LABEL_X: f32 =
    TERMINAL_ROW_PAD_X + TERMINAL_ROW_ICON_W + TERMINAL_ROW_ICON_GAP;

/// Per-pin terminal table: every declared terminal with the net it binds,
/// clickable to select that conductor. Unbound pins read `open` and are
/// not clickable, because there is no net to select.
pub(super) fn terminal_row(ui: &mut Ui, pin: &str, net: Option<&str>) -> TreeRowResult {
    ui.add_enabled_ui(net.is_some(), |ui| {
        let t = Tokens::get(ui.ctx());
        let meta = net.unwrap_or("open");
        let accessible_label = format!("{pin}, {meta}");
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), 24.0), egui::Sense::click());
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
                ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
            }

            // The list has no parent row and no pin ever expands, so the
            // mockup's nesting inset and blank caret column would only push
            // the whole table away from the panel edge. The status icon takes
            // the section's own 10 px inset instead.
            let icon_center = egui::pos2(
                rect.left() + TERMINAL_ROW_PAD_X + TERMINAL_ROW_ICON_W * 0.5,
                rect.center().y,
            );
            if net.is_some() {
                ui.painter().hline(
                    egui::Rangef::new(icon_center.x - 5.5, icon_center.x + 5.5),
                    icon_center.y,
                    egui::Stroke::new(1.25, t.color.wire),
                );
            } else {
                ui.painter()
                    .circle_stroke(icon_center, 4.0, egui::Stroke::new(1.0, t.color.err));
            }

            let meta_galley = ui.painter().layout_no_wrap(
                meta.to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
            );
            let meta_left = rect.right() - TERMINAL_ROW_PAD_X - meta_galley.size().x;
            let label_left = rect.left() + TERMINAL_ROW_LABEL_X;
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
            checkbox_changed: false,
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
    let open_pins = bindings.iter().filter(|(_, net)| net.is_none()).count();

    section_header(
        ui,
        "Terminals",
        Some(&if open_pins == 0 {
            format!("{} bound", bindings.len())
        } else {
            format!("{open_pins} open")
        }),
    );
    if bindings.is_empty() {
        muted_inspector_copy(ui, "This instance declares no terminals.");
        return;
    }
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

pub(super) const OP_SUMMARY_MARGIN_X: f32 = 8.0;
pub(super) const OP_SUMMARY_PADDING: f32 = 9.0;
pub(super) const OP_SUMMARY_ROW_H: f32 = 22.0;
/// Gap between an annotation row's label and its value.
pub(super) const OP_SUMMARY_GAP: f32 = 8.0;
/// Widest share of a row the label may hold once the two columns compete.
pub(super) const OP_SUMMARY_LABEL_FRACTION: f32 = 0.45;

/// One measured row of the annotation card.
pub(super) struct OperatingPointRow {
    pub(super) label: std::sync::Arc<egui::Galley>,
    pub(super) value: std::sync::Arc<egui::Galley>,
    /// Top of each galley relative to the row, so the first line of a wrapped
    /// value sits on the same band as every single-line row.
    pub(super) label_top: f32,
    pub(super) value_top: f32,
    pub(super) height: f32,
}

/// Height of a laid-out column's first line.
fn first_line_height(galley: &egui::Galley) -> f32 {
    galley
        .rows
        .first()
        .map_or_else(|| galley.size().y, |row| row.rect().height())
}

/// How much taller than one line a wrapped column is.
fn wrapped_overflow(galley: &egui::Galley) -> f32 {
    (galley.size().y - first_line_height(galley)).max(0.0)
}

/// Lay a row out against the card's inner width.
///
/// The label keeps its measured width while both columns fit, and is capped
/// at [`OP_SUMMARY_LABEL_FRACTION`] once they compete. Whatever the value
/// then needs it wraps into, growing the row: retained evidence and the
/// reason a run does not apply are facts, and neither may be hidden behind an
/// ellipsis or painted over its own label.
pub(super) fn operating_point_row(
    ui: &Ui,
    inner_width: f32,
    label: &str,
    value: &str,
) -> OperatingPointRow {
    let t = Tokens::get(ui.ctx());
    let label_font = theme::sans(tokens::FS_2, FontWeight::Regular);
    let value_font = theme::mono(tokens::FS_2, FontWeight::Medium);
    let columns = (inner_width - OP_SUMMARY_GAP).max(1.0);
    let measure = |text: &str, font: &egui::FontId| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font.clone(), Color32::WHITE)
            .size()
            .x
    };
    let natural_label = measure(label, &label_font);
    let label_width = if natural_label + measure(value, &value_font) <= columns {
        natural_label
    } else {
        natural_label.min(columns * OP_SUMMARY_LABEL_FRACTION)
    };
    let value_width = (columns - label_width).max(1.0);

    let label_galley =
        ui.painter()
            .layout(label.to_owned(), label_font, t.color.text_dim, label_width);
    let mut value_job = egui::text::LayoutJob::simple(
        value.to_owned(),
        value_font,
        t.color.text,
        value_width.max(1.0),
    );
    value_job.halign = egui::Align::RIGHT;
    let value_galley = ui.fonts_mut(|fonts| fonts.layout_job(value_job));

    // Center each column's *first* line in the base band, then grow the row
    // by whatever the taller wrapped column spills past one line. Measuring
    // the galley rather than the font metric keeps a row of single-line
    // columns exactly one band tall: the two disagree by a fraction of a
    // pixel, and that fraction would compound down the card.
    let label_top = (OP_SUMMARY_ROW_H - first_line_height(&label_galley)) * 0.5;
    let value_top = (OP_SUMMARY_ROW_H - first_line_height(&value_galley)) * 0.5;
    let height =
        OP_SUMMARY_ROW_H + wrapped_overflow(&label_galley).max(wrapped_overflow(&value_galley));
    OperatingPointRow {
        label: label_galley,
        value: value_galley,
        label_top,
        value_top,
        height,
    }
}

pub(super) fn operating_point_summary(ui: &mut Ui, rows: &[(String, String)]) {
    let t = Tokens::get(ui.ctx());
    let inner_width =
        (ui.available_width() - 2.0 * OP_SUMMARY_MARGIN_X - 2.0 * OP_SUMMARY_PADDING).max(1.0);
    let measured = rows
        .iter()
        .map(|(label, value)| operating_point_row(ui, inner_width, label, value))
        .collect::<Vec<_>>();
    let height = OP_SUMMARY_PADDING * 2.0 + measured.iter().map(|row| row.height).sum::<f32>();
    let (outer, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    let card = egui::Rect::from_min_max(
        egui::pos2(outer.left() + OP_SUMMARY_MARGIN_X, outer.top()),
        egui::pos2(outer.right() - OP_SUMMARY_MARGIN_X, outer.bottom()),
    );
    ui.painter().rect(
        card,
        t.radius,
        t.color.bg_inset,
        egui::Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    let painter = ui.painter().with_clip_rect(card);
    let mut top = card.top() + OP_SUMMARY_PADDING;
    for row in measured {
        painter.galley(
            egui::pos2(card.left() + OP_SUMMARY_PADDING, top + row.label_top),
            row.label,
            t.color.text_dim,
        );
        painter.galley(
            egui::pos2(card.right() - OP_SUMMARY_PADDING, top + row.value_top),
            row.value,
            t.color.text,
        );
        top += row.height;
    }
}

pub(super) fn operating_point(ui: &mut Ui, app: &RSpiceApp, component: &Component) {
    let retained = app.state.simulation.active_run().and_then(|run| {
        run.analyses.iter().find_map(|analysis| {
            analysis.device_op.as_ref().and_then(|report| {
                report
                    .entries
                    .iter()
                    .find(|entry| entry.name.eq_ignore_ascii_case(&component.name))
                    .map(|entry| {
                        (
                            run.id,
                            analysis.label.clone(),
                            entry.region,
                            entry.params.clone(),
                            active_run_matches_design(&app.state),
                        )
                    })
            })
        })
    });
    if let Some((run_id, analysis, region, params, current)) = retained {
        section_header(
            ui,
            &format!("Operating point · Run {run_id}"),
            Some(if current { "current" } else { "stale" }),
        );
        let mut rows = Vec::new();
        if !current {
            rows.push((
                "Provenance".to_owned(),
                "Historical evidence · rerun for current schematic".to_owned(),
            ));
        }
        if let Some(region) = region {
            rows.push(("Region".to_owned(), region.to_owned()));
        }
        for (name, value) in params.into_iter().take(2_usize.saturating_sub(rows.len())) {
            rows.push((name.to_owned(), format!("{value:.6e}")));
        }
        rows.push((
            "Temperature".to_owned(),
            format!(
                "{:.1} °C",
                app.state.sim_setup.reference_pvt.temperature_celsius
            ),
        ));
        rows.push(("Analysis".to_owned(), analysis));
        operating_point_summary(ui, &rows);
    } else {
        section_header(ui, "Operating point", Some("no evidence"));
        operating_point_summary(
            ui,
            &[
                (
                    "Selection".to_owned(),
                    "No retained device operating point".to_owned(),
                ),
                (
                    "Required analysis".to_owned(),
                    "DC operating point".to_owned(),
                ),
            ],
        );
    }
}

pub(super) fn component_checks(ui: &mut Ui, app: &RSpiceApp, component: &Component) {
    let topology = app.state.schematic.topology_version();
    let current = checks_current(&app.state);
    let findings = current
        .then(|| {
            app.state.dialogs.drc_results.as_ref().map(|result| {
                result
                    .violations()
                    .iter()
                    .filter(|violation| violation_targets_component(violation, component))
                    .collect::<Vec<_>>()
            })
        })
        .flatten();
    section_header(ui, "Checks", Some(&checks_status(&app.state)));
    let t = Tokens::get(ui.ctx());
    let (connectivity, tone, mark) = if let Some(findings) = findings {
        if findings.is_empty() {
            ("checked".to_owned(), t.color.ok, StatusMark::Success)
        } else {
            let severity = findings
                .iter()
                .map(|finding| finding.severity)
                .max()
                .unwrap_or(DrcSeverity::Info);
            (
                format!(
                    "{} finding{}",
                    findings.len(),
                    if findings.len() == 1 { "" } else { "s" }
                ),
                if severity >= DrcSeverity::Error {
                    t.color.err
                } else {
                    t.color.warn
                },
                if severity >= DrcSeverity::Error {
                    StatusMark::Failure
                } else {
                    StatusMark::Warning
                },
            )
        }
    } else {
        (
            "pending recheck".to_owned(),
            t.color.warn,
            StatusMark::Warning,
        )
    };
    property_row_status(ui, "Connectivity", &connectivity, tone, mark);
    let soa = retained_component_soa(&app.state, &component.name);
    property_row_status(
        ui,
        "Safe operating area",
        &soa.label,
        match soa.tone {
            ComponentSoaTone::Pass => t.color.ok,
            ComponentSoaTone::Warning => t.color.warn,
            ComponentSoaTone::Failure => t.color.err,
            ComponentSoaTone::NoEvidence => t.color.text_dim,
        },
        match soa.tone {
            ComponentSoaTone::Pass => StatusMark::Success,
            ComponentSoaTone::Warning => StatusMark::Warning,
            ComponentSoaTone::Failure => StatusMark::Failure,
            ComponentSoaTone::NoEvidence => StatusMark::Neutral,
        },
    );
    property_row(
        ui,
        "Last checked",
        &if current {
            format!("topology revision {topology}")
        } else {
            format!("rerun for topology revision {topology}")
        },
    );
}

pub(super) fn violation_targets_component(violation: &DrcViolation, component: &Component) -> bool {
    matches!(
        &violation.location,
        DrcLocation::Component { id, .. } if *id == component.id
    ) || violation
        .related_items
        .iter()
        .any(|item| item.eq_ignore_ascii_case(&component.name))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ComponentSoaTone {
    Pass,
    Warning,
    Failure,
    NoEvidence,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct ComponentSoaDisplay {
    pub(super) label: String,
    pub(super) tone: ComponentSoaTone,
}

/// Resolve only current, source-attributed SOA evidence for the selected
/// instance. A retained run from another project revision is deliberately
/// ignored; showing its margin beside a changed schematic would be unsafe.
pub(super) fn retained_component_soa(
    state: &AppState,
    component_name: &str,
) -> ComponentSoaDisplay {
    let Some(run) = state.simulation.active_run() else {
        return no_component_soa();
    };
    let Some(receipt) = run.prepared_receipt() else {
        return no_component_soa();
    };
    if receipt.project_revision() != state.workspace.project.revision() {
        return no_component_soa();
    }

    let mut evaluations = Vec::new();
    let mut violations = 0usize;
    for analysis in &run.analyses {
        if !analysis.success || analysis.provenance.is_none() {
            continue;
        }
        let Some(AnalysisResultPayload::Soa {
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

pub(super) fn no_component_soa() -> ComponentSoaDisplay {
    ComponentSoaDisplay {
        label: "No retained device evidence".to_owned(),
        tone: ComponentSoaTone::NoEvidence,
    }
}

pub(super) fn component_soa_display(
    evaluations: &[&crate::state::SoaEvaluationEvidence],
    violations: usize,
) -> ComponentSoaDisplay {
    if violations > 0 {
        return ComponentSoaDisplay {
            label: format!(
                "{violations} retained violation{}",
                if violations == 1 { "" } else { "s" }
            ),
            tone: ComponentSoaTone::Failure,
        };
    }
    if evaluations.is_empty() {
        return no_component_soa();
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
    let margin = margin_percent
        .map(|value| format!(" · {value:.1}% margin"))
        .unwrap_or_default();
    let (label, tone) = match verdict {
        SoaRuleVerdictEvidence::Pass => (format!("pass{margin}"), ComponentSoaTone::Pass),
        SoaRuleVerdictEvidence::Warning => (format!("warning{margin}"), ComponentSoaTone::Warning),
        SoaRuleVerdictEvidence::Violation | SoaRuleVerdictEvidence::Critical => {
            (format!("failed{margin}"), ComponentSoaTone::Failure)
        }
    };
    ComponentSoaDisplay { label, tone }
}

// =============================================================================
// Net inspector
// =============================================================================

pub(super) const fn net_icon(class: NetClass) -> WorkbenchIcon {
    match class {
        NetClass::Ground | NetClass::Supply => WorkbenchIcon::Supply,
        NetClass::Signal => WorkbenchIcon::Wire,
    }
}
