//! Property dialog host.
//!
//! The floating schema-driven component editor backed by the
//! `PropertyRegistry`. Inline inspection lives in the workbench inspector
//! (`crate::workbench::docks::inspector`).

use std::collections::HashMap;

use crate::properties::{
    ComponentEditorContext, ComponentModelContext, ComponentOperatingPointContext,
    ComponentTerminalContext, TabbedDialogResult, render_tabbed_property_dialog,
};
use crate::simulation::netlist_gen::{HierarchySource, projection_nets};
use crate::state::{Component, ComponentType, PropertySheet, PropertyValue};
use crate::workbench::app_state::AppState;
use crate::workbench::state::{ModelsPage, Workspace};

/// Render the floating schematic component editor.
/// Call this from the main app update loop
pub fn render_property_dialog(ctx: &egui::Context, state: &mut AppState) -> TabbedDialogResult {
    if state.tabbed_property_dialog.open {
        state.tabbed_property_dialog.session_error = component_property_session_error(state);
    }
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let number_locale = state.ui.number_locale;
    let commit_policy = state.schematic.document_policy.property_commit;
    let editor_context = component_editor_context(state);
    let result = render_tabbed_property_dialog(
        ctx,
        &mut state.tabbed_property_dialog,
        &editor_context,
        &state.property_registry,
        &state.model_library_manager,
        quantity_policy,
        number_locale,
        commit_policy,
    );

    // Handle dialog result - apply changes back to component
    if matches!(
        result,
        TabbedDialogResult::Applied | TabbedDialogResult::AppliedAndClose
    ) && let Some(comp_id) = state.tabbed_property_dialog.component_id
    {
        let close_after_commit = result == TabbedDialogResult::AppliedAndClose;
        if let Some(error) = component_property_session_error(state) {
            state.tabbed_property_dialog.open = true;
            state.tabbed_property_dialog.session_error = Some(error);
            return TabbedDialogResult::None;
        }
        let committed = state.tabbed_property_dialog.take_prepared_commit();
        let committed_names: Vec<String> = committed.keys().cloned().collect();
        // The bridge serializes the full component property map. Merge the
        // validated delta onto the last committed baseline so a rejected
        // partial field cannot erase an unrelated existing parameter.
        let mut values = state.tabbed_property_dialog.original_values.clone();
        values.extend(committed);
        let Some(component) = state.schematic.components.iter().find(|c| c.id == comp_id) else {
            state.tabbed_property_dialog.open = true;
            state.tabbed_property_dialog.session_error = Some(
                "The selected component no longer exists. Close and reopen Object properties."
                    .to_owned(),
            );
            return TabbedDialogResult::None;
        };
        let mut candidate = component.clone();
        crate::properties::property_bridge::apply_properties_to_component(
            &mut candidate,
            &values,
            &state.property_registry,
        );
        if let Err(error) = validate_component_identity(state, comp_id, &candidate) {
            state.tabbed_property_dialog.open = true;
            state
                .tabbed_property_dialog
                .validation_errors
                .insert("name".to_owned(), error);
            state.tabbed_property_dialog.commit_error =
                Some("Correct the instance reference before applying.".to_owned());
            return TabbedDialogResult::None;
        }
        if let Err(error) = validate_model_binding_authority(state, &candidate, &values) {
            state.tabbed_property_dialog.open = true;
            state.tabbed_property_dialog.commit_error =
                Some(format!("The model binding was not changed: {error}"));
            return TabbedDialogResult::None;
        }
        let Some(property_sheet) = state.property_registry.get(candidate.kind) else {
            state.tabbed_property_dialog.open = true;
            state.tabbed_property_dialog.commit_error = Some(
                "The component was not changed because its property schema is unavailable."
                    .to_owned(),
            );
            return TabbedDialogResult::None;
        };
        if let Err(error) = validate_component_contract(&candidate, &values, property_sheet) {
            state.tabbed_property_dialog.open = true;
            state.tabbed_property_dialog.commit_error =
                Some(format!("The component was not changed: {error}"));
            return TabbedDialogResult::None;
        }
        if candidate.kind == crate::state::ComponentType::Port
            && let Err(error) = state
                .schematic
                .validate_edited_port_contract(comp_id, &candidate)
        {
            state.tabbed_property_dialog.open = true;
            state.tabbed_property_dialog.commit_error =
                Some(format!("The interface contract was not changed: {error}."));
            return TabbedDialogResult::None;
        }
        let changed_port_contract =
            candidate.kind == crate::state::ComponentType::Port && &candidate != component;
        if &candidate != component {
            let before = crate::state::SchematicSnapshot::capture(&state.schematic);
            if let Some(component) = state
                .schematic
                .components
                .iter_mut()
                .find(|component| component.id == comp_id)
            {
                *component = candidate;
            }
            state.schematic.is_dirty = true;
            state.schematic.bump_topology_version();
            state.schematic.commit_undo_from(before, "edit properties");
        }
        if changed_port_contract {
            state.sync_active_schematic_to_workspace();
        }

        // The mockup primary closes after the exact transaction completes.
        if close_after_commit {
            state.tabbed_property_dialog.clear_after_apply();
        } else if state.tabbed_property_dialog.open {
            state.tabbed_property_dialog.component_baseline = state
                .schematic
                .components
                .iter()
                .find(|component| component.id == comp_id)
                .cloned();
            state
                .tabbed_property_dialog
                .mark_fields_applied(committed_names);
        }
    }

    match result {
        TabbedDialogResult::OpenModel | TabbedDialogResult::OpenQualification => {
            let model = editor_context
                .model
                .as_ref()
                .map(|model| model.name.clone());
            let library = editor_context
                .model
                .as_ref()
                .and_then(|model| model.library.clone());
            state.tabbed_property_dialog.close();
            state.workbench.activate(Workspace::Models);
            state.workbench.models_page = if result == TabbedDialogResult::OpenQualification {
                ModelsPage::Qualification
            } else {
                ModelsPage::Models
            };
            if let Some(model) = model {
                state.workbench.selected_model = Some(model);
            }
            if let Some(library) = library {
                state.model_library_manager.select_library(&library);
            }
        }
        TabbedDialogResult::CrossProbe => {
            state.tabbed_property_dialog.close();
            state.workbench.activate(Workspace::Results);
        }
        _ => {}
    }

    if !state.tabbed_property_dialog.open {
        state.property_registry.clear_cell_instance_sheet();
    }

    result
}

fn validate_model_binding_authority(
    state: &AppState,
    component: &Component,
    values: &HashMap<String, PropertyValue>,
) -> Result<(), String> {
    use crate::state::model_library::ModelConsumerScope;

    let params = crate::state::parse_params_string(&component.params);
    let declared = params.get("model").cloned().or_else(|| {
        model_is_component_value(component.kind)
            .then(|| component.value.trim().to_owned())
            .filter(|value| !value.is_empty())
    });
    let Some(declared) = declared else {
        return Ok(());
    };
    let selected_library = values
        .get("model_library")
        .map(PropertyValue::display_string)
        .or_else(|| params.get("model_library").cloned())
        .map(|library| library.trim().to_owned())
        .filter(|library| !library.is_empty());

    let provider_libraries = state
        .model_library_manager
        .definition_providers(ModelConsumerScope::PrimitiveModel, &declared)
        .into_iter()
        .map(|provider| provider.library)
        .collect::<Vec<_>>();
    if provider_libraries.is_empty() {
        return if let Some(library_name) = selected_library {
            Err(format!(
                "catalog library '{library_name}' does not provide executable model '{declared}'"
            ))
        } else {
            // A primitive implemented directly by the engine has no catalog
            // provider and therefore needs no project-global decision.
            Ok(())
        };
    }

    let effective = state
        .model_library_manager
        .effective_definition_provider(ModelConsumerScope::PrimitiveModel, &declared)?
        .expect("a non-empty provider set has one effective provider");
    if let Some(library_name) = selected_library.as_deref()
        && !effective.library.eq_ignore_ascii_case(library_name)
    {
        return Err(format!(
            "model '{declared}' executes from project-global provider '{}', not instance metadata '{}'; resolve the provider globally or select '{}'",
            effective.library, library_name, effective.library
        ));
    }
    Ok(())
}

fn validate_component_identity(
    state: &AppState,
    component_id: u64,
    candidate: &Component,
) -> Result<(), String> {
    if !candidate.kind.spice_prefix().is_empty() {
        candidate
            .validate_reference_designator(candidate.name.trim())
            .map_err(|error| format!("The instance reference was not changed: {error}"))?;
        if state.schematic.components.iter().any(|component| {
            component.id != component_id
                && component.name.eq_ignore_ascii_case(candidate.name.trim())
        }) {
            return Err(format!(
                "The instance reference was not changed: '{}' is already used on this sheet.",
                candidate.name.trim()
            ));
        }
    }
    Ok(())
}

fn validate_component_contract(
    component: &Component,
    values: &HashMap<String, PropertyValue>,
    sheet: &PropertySheet,
) -> Result<(), String> {
    let params = crate::state::parse_params_string(&component.params);
    let number = |name: &str| {
        values
            .get(name)
            .or_else(|| sheet.get(name).map(|definition| &definition.default_value))
            .and_then(property_value_as_number)
            .or_else(|| {
                params
                    .get(name)
                    .and_then(|value| crate::quantity::parse_engineering_value(value).ok())
            })
    };
    let ordered = |lower: &str, upper: &str, label: &str| -> Result<(), String> {
        if let (Some(lower), Some(upper)) = (number(lower), number(upper))
            && lower > upper
        {
            return Err(format!("{label} minimum must not exceed its maximum"));
        }
        Ok(())
    };
    match component.kind {
        ComponentType::VoltageSourcePulse | ComponentType::CurrentSourcePulse => {
            if let (Some(rise), Some(fall), Some(width), Some(period)) =
                (number("tr"), number("tf"), number("pw"), number("per"))
                && rise + width + fall > period
            {
                return Err(
                    "pulse rise time, width, and fall time must fit within one period".to_owned(),
                );
            }
        }
        ComponentType::VoltageSourceExp | ComponentType::CurrentSourceExp => {
            if let (Some(first), Some(second)) = (number("td1"), number("td2"))
                && second < first
            {
                return Err("the second exponential transition cannot precede the first".to_owned());
            }
        }
        ComponentType::CurrentSourceNoise => {
            let amplitude = number("na").unwrap_or_else(|| {
                crate::quantity::parse_engineering_value(&component.value).unwrap_or(0.0)
            });
            let flicker_amplitude = number("namp").unwrap_or(0.0);
            if (amplitude != 0.0 || flicker_amplitude != 0.0)
                && number("nt").is_some_and(|interval| interval <= 0.0)
            {
                return Err(
                    "noise sample interval must be positive when noise is enabled".to_owned(),
                );
            }
            if flicker_amplitude != 0.0
                && number("nalpha").is_some_and(|alpha| !(0.0..2.0).contains(&alpha))
            {
                return Err(
                    "flicker-noise exponent must be greater than 0 and less than 2".to_owned(),
                );
            }
        }
        ComponentType::VSwitch | ComponentType::ISwitch | ComponentType::GenericSwitch => {
            if let (Some(on), Some(off)) = (number("ron"), number("roff"))
                && (on <= 0.0 || off <= on)
            {
                return Err(
                    "switch resistance requires 0 < on resistance < off resistance".to_owned(),
                );
            }
        }
        ComponentType::Memristor => {
            if let (Some(on), Some(off)) = (number("ron"), number("roff"))
                && (on <= 0.0 || off <= on)
            {
                return Err(
                    "memristor resistance requires 0 < on resistance < off resistance".to_owned(),
                );
            }
        }
        ComponentType::Vcvs | ComponentType::Ccvs => {
            ordered("vmin", "vmax", "output-voltage limit")?;
        }
        ComponentType::Vccs | ComponentType::Cccs => {
            ordered("imin", "imax", "output-current limit")?;
        }
        ComponentType::XspiceLimiter
        | ComponentType::XspiceIntegrator
        | ComponentType::XspiceDifferentiator => {
            ordered("out_lower_limit", "out_upper_limit", "output limit")?;
        }
        ComponentType::XspiceAdcBridge => {
            ordered("in_low", "in_high", "ADC input threshold")?;
        }
        ComponentType::XspiceDacBridge => {
            ordered("out_low", "out_high", "DAC output level")?;
        }
        _ => {}
    }
    Ok(())
}

fn property_value_as_number(value: &PropertyValue) -> Option<f64> {
    value.as_number().or_else(|| {
        let displayed = value.display_string();
        let literal = displayed
            .strip_prefix('{')
            .and_then(|displayed| displayed.strip_suffix('}'))
            .unwrap_or(&displayed);
        crate::quantity::parse_engineering_value(literal).ok()
    })
}

fn component_editor_context(state: &AppState) -> ComponentEditorContext {
    let Some(component_id) = state.tabbed_property_dialog.component_id else {
        return ComponentEditorContext::default();
    };
    let Some(component) = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
    else {
        return ComponentEditorContext::default();
    };

    let instance_path = format!(
        "{}/{}",
        state.workspace.active_view.display_path(),
        component.name
    );
    let library_cell = component
        .library_cell
        .as_ref()
        .map(|binding| format!("{}/{}/{}", binding.library, binding.cell, binding.view))
        .unwrap_or_else(|| {
            format!(
                "{} / {}",
                component.kind.spice_prefix(),
                component.kind.display_name()
            )
        });

    ComponentEditorContext {
        glyph: component_editor_glyph(component.kind).to_owned(),
        subtitle: component
            .library_cell
            .as_ref()
            .map(|binding| format!("{} · {} view", component.kind.display_name(), binding.view))
            .unwrap_or_else(|| format!("{} · SPICE primitive", component.kind.display_name())),
        instance_path,
        library_cell,
        family: format!(
            "{} · {}",
            component.kind.display_name(),
            if component.library_cell.is_some() {
                "library"
            } else {
                "primitive"
            }
        ),
        model: component_model_context(state, component),
        operating_point: component_operating_point_context(state, component),
        terminals: component_terminal_context(state, component),
    }
}

fn component_editor_glyph(kind: ComponentType) -> &'static str {
    match kind {
        ComponentType::OpAmp => "△",
        ComponentType::VoltageSourceAc
        | ComponentType::VoltageSourceSin
        | ComponentType::CurrentSourceAc
        | ComponentType::CurrentSourceSin => "~",
        ComponentType::VoltageSourcePulse | ComponentType::CurrentSourcePulse => "∿",
        ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl => "⌁",
        ComponentType::VoltageSource => "+",
        ComponentType::CurrentSource => "I",
        ComponentType::Ground => "0",
        ComponentType::Port | ComponentType::RfPort => "P",
        ComponentType::CellInstance => "▣",
        ComponentType::XspiceInverter | ComponentType::XspiceBuffer => "⊳",
        kind if kind.is_xspice() => "⊞",
        _ => kind.spice_prefix(),
    }
}

fn component_model_context(
    state: &AppState,
    component: &Component,
) -> Option<ComponentModelContext> {
    let params = crate::state::parse_params_string(&component.params);
    let declared = params
        .get("model")
        .cloned()
        .or_else(|| {
            model_is_component_value(component.kind)
                .then(|| component.value.trim().to_owned())
                .filter(|value| !value.is_empty())
        })
        .or_else(|| component.library_cell.as_ref()?.module_name.clone())
        .or_else(|| {
            component
                .library_cell
                .as_ref()
                .map(|binding| binding.cell.clone())
        });
    let Some(declared) = declared else {
        if let Some(name) = generated_inline_model_name(component) {
            return Some(ComponentModelContext {
                name,
                library: None,
                source: "Netlist generator".to_owned(),
                section: "not applicable".to_owned(),
                status: "inline model - generated from instance parameters".to_owned(),
                can_open: false,
                can_qualify: false,
            });
        }
        return builtin_primitive_model_name(component.kind).map(|name| ComponentModelContext {
            name,
            library: None,
            source: "Built-in device kernel".to_owned(),
            section: "not sectioned".to_owned(),
            status: "built-in exact stamp".to_owned(),
            can_open: false,
            can_qualify: false,
        });
    };

    let selected_library = params
        .get("model_library")
        .map(|library| library.trim().to_owned())
        .filter(|library| !library.is_empty());
    let mut matches = state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .filter_map(|library| {
            library
                .models
                .values()
                .find(|model| model.name.eq_ignore_ascii_case(&declared))
                .map(|model| {
                    (
                        library.name.clone(),
                        model.name.clone(),
                        model
                            .file_path
                            .as_ref()
                            .or(library.root_path.as_ref())
                            .map(|path| path.display().to_string())
                            .unwrap_or_else(|| "in-memory catalog".to_owned()),
                    )
                })
        })
        .collect::<Vec<_>>();
    matches.sort_by(|left, right| left.0.cmp(&right.0));

    let binding = component.library_cell.as_ref();
    let effective_provider = state
        .model_library_manager
        .effective_definition_provider(
            crate::state::model_library::ModelConsumerScope::PrimitiveModel,
            &declared,
        )
        .ok()
        .flatten();
    let metadata_conflicts = selected_library.as_deref().is_some_and(|selected| {
        effective_provider
            .as_ref()
            .is_some_and(|provider| !provider.library.eq_ignore_ascii_case(selected))
    });
    let catalog_match = effective_provider.as_ref().and_then(|provider| {
        matches.iter().find(|(library, model, _)| {
            library == &provider.library && model == &provider.definition
        })
    });
    if let Some((library, model, source)) = catalog_match {
        let status = if metadata_conflicts {
            "project provider resolved · instance metadata conflicts"
        } else if matches.len() > 1 {
            "project-global catalog provider resolved"
        } else if matches.len() == 1 {
            "unique catalog binding resolved"
        } else {
            "provider selection requires catalog governance"
        };
        let section = binding
            .and_then(|binding| binding.model_section.clone())
            .or_else(|| params.get("section").cloned())
            .unwrap_or_else(|| {
                format!(
                    "{} process corner",
                    state.sim_setup.reference_pvt.process.short_name()
                )
            });
        Some(ComponentModelContext {
            name: model.clone(),
            library: Some(library.clone()),
            source: format!("{library} · {source}"),
            section,
            status: status.to_owned(),
            can_open: true,
            can_qualify: true,
        })
    } else {
        let source = selected_library
            .as_ref()
            .map(|library| format!("{library} · unresolved catalog selection"))
            .or_else(|| {
                binding
                    .and_then(|binding| binding.source_path.as_ref())
                    .map(|path| path.display().to_string())
            })
            .or_else(|| {
                binding
                    .map(|binding| format!("{}/{}/{}", binding.library, binding.cell, binding.view))
            })
            .unwrap_or_else(|| "No catalog source resolved".to_owned());
        Some(ComponentModelContext {
            name: declared,
            library: selected_library,
            source,
            section: binding
                .and_then(|binding| binding.model_section.clone())
                .or_else(|| params.get("section").cloned())
                .unwrap_or_else(|| {
                    format!(
                        "{} process corner",
                        state.sim_setup.reference_pvt.process.short_name()
                    )
                }),
            status: "declared binding · catalog unverified".to_owned(),
            can_open: false,
            can_qualify: false,
        })
    }
}

fn builtin_primitive_model_name(kind: ComponentType) -> Option<String> {
    if matches!(
        kind,
        ComponentType::Ground | ComponentType::Port | ComponentType::CellInstance
    ) {
        return None;
    }
    let prefix = kind.spice_prefix();
    Some(if prefix.is_empty() {
        kind.display_name().to_owned()
    } else {
        format!("{prefix} primitive")
    })
}

fn model_is_component_value(kind: ComponentType) -> bool {
    matches!(
        kind,
        ComponentType::Diode
            | ComponentType::Nmos
            | ComponentType::Pmos
            | ComponentType::NVdmos
            | ComponentType::PVdmos
            | ComponentType::NmosSoi
            | ComponentType::PmosSoi
            | ComponentType::NpnBjt
            | ComponentType::PnpBjt
            | ComponentType::NpnBjt4
            | ComponentType::PnpBjt4
            | ComponentType::NpnBjt5
            | ComponentType::PnpBjt5
            | ComponentType::Njfet
            | ComponentType::Pjfet
            | ComponentType::Nmesfet
            | ComponentType::Pmesfet
    )
}

fn generated_inline_model_name(component: &Component) -> Option<String> {
    let prefix = match component.kind {
        ComponentType::Nmos => "nmos",
        ComponentType::Pmos => "pmos",
        ComponentType::NVdmos => "nvdmos",
        ComponentType::PVdmos => "pvdmos",
        ComponentType::NmosSoi => "nmossoi",
        ComponentType::PmosSoi => "pmossoi",
        ComponentType::NpnBjt | ComponentType::NpnBjt4 | ComponentType::NpnBjt5 => "npn",
        ComponentType::PnpBjt | ComponentType::PnpBjt4 | ComponentType::PnpBjt5 => "pnp",
        ComponentType::Njfet => "njf",
        ComponentType::Pjfet => "pjf",
        ComponentType::Nmesfet => "nmf",
        ComponentType::Pmesfet => "pmf",
        ComponentType::VSwitch => "sw",
        ComponentType::ISwitch => "isw",
        ComponentType::GenericSwitch => "sw",
        ComponentType::Diode => "d",
        ComponentType::SaturableInductor => "core",
        ComponentType::Memristor => "mem",
        ComponentType::CoupledTransmissionLine => "cpl",
        ComponentType::LossyTransmissionLine => {
            let params = crate::state::parse_params_string(&component.params);
            if params.get("kind").is_some_and(|kind| kind == "txl") {
                "txl"
            } else {
                "ltra"
            }
        }
        kind if kind.is_xspice() => return Some(format!("{}_model", component.name)),
        _ => return None,
    };
    Some(format!("{prefix}_{}", component.name))
}

fn component_operating_point_context(
    state: &AppState,
    component: &Component,
) -> Option<ComponentOperatingPointContext> {
    let run = state.simulation.active_run()?;
    let current = run
        .prepared_receipt()
        .is_some_and(|receipt| receipt.project_revision() == state.workspace.project.revision())
        && state.simulation.cross_probe.is_current_for(
            &state.workspace.active_view,
            state.schematic.topology_version(),
        );
    run.analyses.iter().find_map(|analysis| {
        analysis.device_op.as_ref().and_then(|report| {
            report
                .entries
                .iter()
                .find(|entry| entry.name.eq_ignore_ascii_case(&component.name))
                .map(|entry| {
                    let mut rows = Vec::new();
                    if let Some(region) = entry.region {
                        rows.push(("Region".to_owned(), region.to_owned()));
                    }
                    rows.extend(
                        entry
                            .params
                            .iter()
                            .take(5_usize.saturating_sub(rows.len()))
                            .map(|(name, value)| ((*name).to_owned(), format!("{value:.6e}"))),
                    );
                    rows.push((
                        "Temperature".to_owned(),
                        format!(
                            "{:.1} °C",
                            state.sim_setup.reference_pvt.temperature_celsius
                        ),
                    ));
                    ComponentOperatingPointContext {
                        run_id: run.id,
                        analysis: analysis.label.clone(),
                        current,
                        rows,
                    }
                })
        })
    })
}

/// The instance's pins with the net each one binds, read from the configured
/// design.
///
/// When that design does not resolve, every pin's net cell states the reason
/// instead. An empty cell would read as an open pin, which is a different and
/// much worse claim than "the configuration does not resolve".
fn component_terminal_context(
    state: &AppState,
    component: &Component,
) -> Vec<ComponentTerminalContext> {
    let projection = match state.workspace.design_projection(
        &state.library_manager,
        &state.workspace.active_view,
        &state.schematic,
    ) {
        Ok(projection) => projection,
        Err(error) => {
            let reason = error.to_string();
            return component
                .terminal_positions_resolved(None)
                .into_iter()
                .enumerate()
                .map(|(index, (pin, _))| ComponentTerminalContext {
                    direction: component_terminal_direction(component, index, &pin),
                    net: Some(reason.clone()),
                    pin,
                })
                .collect();
        }
    };
    let hierarchy = HierarchySource::from_design_projection(&state.library_manager, &projection);
    let nets = projection_nets(
        &state.library_manager,
        &projection,
        &state.workspace.active_view.key(),
    );
    let mut bound = HashMap::<(u64, String), String>::new();
    for net in nets.iter() {
        let isolated = net.terminals.len() == 1
            && net.wire_ids.is_empty()
            && net.port.is_none()
            && !net.authored_name;
        if isolated {
            continue;
        }
        for terminal in &net.terminals {
            bound.insert(
                (terminal.component_id, terminal.pin.to_ascii_lowercase()),
                net.name.clone(),
            );
        }
    }
    let resolved = component
        .library_cell
        .as_ref()
        .and_then(|binding| hierarchy.resolved_symbol_for(binding));
    component
        .terminal_positions_resolved(resolved.as_ref())
        .into_iter()
        .enumerate()
        .map(|(index, (pin, _))| ComponentTerminalContext {
            direction: component_terminal_direction(component, index, &pin),
            net: bound
                .get(&(component.id, pin.to_ascii_lowercase()))
                .cloned(),
            pin,
        })
        .collect()
}

fn component_terminal_direction(component: &Component, index: usize, pin: &str) -> String {
    if let Some(direction) = component
        .library_cell
        .as_ref()
        .and_then(|binding| binding.terminal_dirs.get(index))
    {
        return direction.keyword().to_owned();
    }
    if component.kind == ComponentType::Port {
        return component
            .port_contract()
            .map(|contract| contract.direction.keyword().to_owned())
            .unwrap_or_else(|| "inout".to_owned());
    }
    if component.kind.is_xspice() {
        return if matches!(pin.to_ascii_lowercase().as_str(), "out" | "q" | "qbar") {
            "output"
        } else {
            "input"
        }
        .to_owned();
    }
    match component.kind {
        ComponentType::OpAmp => {
            if index == 2 {
                "output"
            } else {
                "input"
            }
        }
        ComponentType::Vcvs | ComponentType::Vccs | ComponentType::Ccvs | ComponentType::Cccs => {
            if index < 2 {
                "output"
            } else {
                "input"
            }
        }
        ComponentType::VSwitch | ComponentType::ISwitch | ComponentType::GenericSwitch => {
            if index < 2 { "passive" } else { "input" }
        }
        ComponentType::Ground => "ground",
        kind if kind.is_source() => "output",
        kind if kind.is_semiconductor() => "inout",
        _ => "passive",
    }
    .to_owned()
}

fn component_property_session_error(state: &AppState) -> Option<String> {
    let dialog = &state.tabbed_property_dialog;
    if state.schematic.read_only || state.active_view_read_only() {
        return Some("The active schematic is read-only; no properties can be applied.".to_owned());
    }
    if dialog.design_execution_epoch != state.design_execution_epoch {
        return Some(
            "The design document changed while properties were open. Close and reopen the current object."
                .to_owned(),
        );
    }
    if dialog.active_schematic_epoch != state.active_schematic_epoch {
        return Some(
            "The active schematic buffer changed while properties were open. Close and reopen the current object."
                .to_owned(),
        );
    }
    if dialog.view_path != state.workspace.active_view.display_path() {
        return Some(
            "The active cell/view changed while properties were open. Close and reopen the current object."
                .to_owned(),
        );
    }
    let Some(baseline) = dialog.component_baseline.as_ref() else {
        return Some(
            "The selected component baseline is unavailable. Close and reopen Object properties."
                .to_owned(),
        );
    };
    let Some(current) = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == baseline.id)
    else {
        return Some(
            "The selected component no longer exists. Close and reopen Object properties."
                .to_owned(),
        );
    };
    (current != baseline).then(|| {
        "The selected component changed while properties were open. Close and reopen the current object."
            .to_owned()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        CellViewRef, Component, ComponentType, Point, PropertyCommitPolicy, PropertyValue,
    };
    use crate::workbench::app::open_property_editor;

    fn dialog_input(events: Vec<egui::Event>) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_100.0, 850.0),
            )),
            events,
            ..Default::default()
        }
    }

    fn key_event(key: egui::Key) -> egui::Event {
        key_event_with_modifiers(key, egui::Modifiers::NONE)
    }

    fn key_event_with_modifiers(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers,
        }
    }

    fn state_with_resistor() -> AppState {
        let mut state = AppState::default();
        state.schematic.components.clear();
        let mut component = Component::new(44, ComponentType::Resistor, Point::new(4, 8));
        component.name = "R1".to_owned();
        component.value = "1k".to_owned();
        state.schematic.components.push(component);
        state.schematic.clear_undo_history();
        state
    }

    #[test]
    fn cross_field_validation_includes_unserialized_schema_defaults() {
        let registry = crate::state::PropertyRegistry::new();
        let sheet = registry
            .get(ComponentType::Memristor)
            .expect("memristor schema");
        let mut component =
            Component::new(1, ComponentType::Memristor, Point::origin()).with_name_value("MR1", "");
        let values =
            HashMap::from([("ron".to_owned(), PropertyValue::Expression("2k".to_owned()))]);
        crate::properties::property_bridge::apply_properties_to_component(
            &mut component,
            &values,
            &registry,
        );

        assert_eq!(
            validate_component_contract(&component, &values, sheet),
            Err("memristor resistance requires 0 < on resistance < off resistance".to_owned())
        );
    }

    #[test]
    fn model_context_uses_the_project_global_provider_for_duplicate_names() {
        use crate::state::model_library::{
            DeviceModel, ModelConsumerScope, ModelLibrary, ModelType,
        };

        let mut state = AppState::default();
        state.model_library_manager.clear();
        for library_name in ["alpha", "beta"] {
            let mut library = ModelLibrary::new(library_name);
            library.add_model(DeviceModel::new("shared_diode", ModelType::Diode));
            state.model_library_manager.add_library(library);
        }
        state
            .model_library_manager
            .resolve_definition_provider(
                ModelConsumerScope::PrimitiveModel,
                "shared_diode",
                "beta",
                "Test selects the project-global executable provider.",
            )
            .expect("provider decision");
        let mut component = Component::new(1, ComponentType::Diode, Point::origin())
            .with_name_value("D1", "shared_diode");
        component.params = "model_library=beta".to_owned();

        let context = component_model_context(&state, &component).expect("model context");

        assert_eq!(context.library.as_deref(), Some("beta"));
        assert_eq!(context.name, "shared_diode");
        assert_eq!(context.section, "TT process corner");
        assert!(context.source.starts_with("beta ·"));
        let values = HashMap::from([(
            "model_library".to_owned(),
            PropertyValue::String("beta".to_owned()),
        )]);
        assert!(validate_model_binding_authority(&state, &component, &values).is_ok());
        let mut component_without_library = component.clone();
        component_without_library.params.clear();
        assert!(
            validate_model_binding_authority(&state, &component_without_library, &HashMap::new())
                .is_ok(),
            "the project-global decision is authoritative even without redundant instance metadata"
        );

        let values = HashMap::from([(
            "model_library".to_owned(),
            PropertyValue::String("alpha".to_owned()),
        )]);
        let error = validate_model_binding_authority(&state, &component, &values)
            .expect_err("instance metadata cannot override the executable provider");
        assert!(error.contains("project-global provider 'beta'"));
    }

    #[test]
    fn session_guard_rejects_read_only_stale_view_epoch_and_object() {
        let mut state = state_with_resistor();
        open_property_editor(&mut state, 44);
        assert!(component_property_session_error(&state).is_none());

        state.schematic.read_only = true;
        assert!(component_property_session_error(&state).is_some());
        state.schematic.read_only = false;

        state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
        assert!(component_property_session_error(&state).is_some());
        state.design_execution_epoch = state.tabbed_property_dialog.design_execution_epoch;

        state.schematic.components[0].name = "R2".to_owned();
        assert!(component_property_session_error(&state).is_some());
    }

    #[test]
    fn rendered_component_primary_commits_name_with_one_real_undo_record() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = state_with_resistor();
        open_property_editor(&mut state, 44);
        state
            .tabbed_property_dialog
            .set_value("name", PropertyValue::String("R99".to_owned()));

        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            render_property_dialog(ctx, &mut state);
        });

        assert!(!state.tabbed_property_dialog.open);
        assert_eq!(state.schematic.components[0].name, "R99");
        assert!(state.schematic.can_undo());
        assert!(state.schematic.undo());
        assert_eq!(state.schematic.components[0].name, "R1");
    }

    #[test]
    fn component_dirty_escape_discards_the_isolated_draft() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = state_with_resistor();
        open_property_editor(&mut state, 44);
        state
            .tabbed_property_dialog
            .set_value("name", PropertyValue::String("R99".to_owned()));

        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        assert!(!state.tabbed_property_dialog.open);
        assert_eq!(state.schematic.components[0].name, "R1");
    }

    #[test]
    fn returning_to_the_same_view_cannot_reauthorize_an_old_dialog() {
        let mut state = state_with_resistor();
        open_property_editor(&mut state, 44);
        let captured_epoch = state.tabbed_property_dialog.active_schematic_epoch;
        let original_view = state.workspace.active_view.clone();

        state.open_workspace_view(CellViewRef::new("work", "detour", "schematic"));
        state.open_workspace_view(original_view.clone());

        assert_eq!(state.workspace.active_view, original_view);
        assert_ne!(state.active_schematic_epoch, captured_epoch);
        assert!(component_property_session_error(&state).is_some());
    }

    #[test]
    fn escape_closes_nested_model_browser_before_dirty_parent_dialog() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = state_with_resistor();
        open_property_editor(&mut state, 44);
        state
            .tabbed_property_dialog
            .set_value("name", PropertyValue::String("R99".to_owned()));
        state.tabbed_property_dialog.model_browser.open = true;

        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            render_property_dialog(ctx, &mut state);
        });

        assert!(!state.tabbed_property_dialog.model_browser.open);
        assert!(state.tabbed_property_dialog.open);
        assert_eq!(state.schematic.components[0].name, "R1");
    }

    #[test]
    fn first_parameter_editor_retains_invalid_source_across_repaints() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = state_with_resistor();
        open_property_editor(&mut state, 44);
        let original = state
            .tabbed_property_dialog
            .numeric_text_draft("r")
            .expect("resistance source")
            .to_owned();

        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        let _ = ctx.run_ui(
            dialog_input(vec![
                // Move explicitly to the end. Ctrl+A is Select All on
                // Windows/Linux but an Emacs-style line-start command on
                // macOS, which made this retained-source contract depend on
                // the host platform rather than the editor behavior under
                // test.
                key_event(egui::Key::End),
                egui::Event::Text("1e".to_owned()),
            ]),
            |ctx| {
                render_property_dialog(ctx, &mut state);
            },
        );
        let expected = format!("{original}1e");
        assert_eq!(
            state.tabbed_property_dialog.numeric_text_draft("r"),
            Some(expected.as_str())
        );
        assert!(
            state
                .tabbed_property_dialog
                .validation_errors
                .contains_key("r")
        );

        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        assert_eq!(
            state.tabbed_property_dialog.numeric_text_draft("r"),
            Some(expected.as_str())
        );
    }

    #[test]
    fn rendered_partial_policy_commits_only_valid_fields_and_retains_invalid_draft() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = state_with_resistor();
        state.schematic.document_policy.property_commit = PropertyCommitPolicy::ApplyValidFields;
        open_property_editor(&mut state, 44);
        state
            .tabbed_property_dialog
            .set_value("name", PropertyValue::String("R99".to_owned()));
        state.tabbed_property_dialog.update_numeric_text_draft(
            "m",
            "0".to_owned(),
            Some("Multiplier must be at least 1".to_owned()),
        );

        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            render_property_dialog(ctx, &mut state);
        });

        assert!(state.tabbed_property_dialog.open);
        assert_eq!(state.schematic.components[0].name, "R99");
        assert_eq!(state.schematic.components[0].params, "");
        assert_eq!(state.schematic.undo_history.undo_count(), 1);
        assert_eq!(
            state.tabbed_property_dialog.numeric_text_draft("m"),
            Some("0")
        );
        assert!(state.tabbed_property_dialog.is_modified("m"));
        assert!(!state.tabbed_property_dialog.is_modified("name"));

        state
            .tabbed_property_dialog
            .update_numeric_text_draft("m", "2".to_owned(), None);
        state
            .tabbed_property_dialog
            .set_value("m", PropertyValue::number(2.0));
        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            render_property_dialog(ctx, &mut state);
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            render_property_dialog(ctx, &mut state);
        });

        assert!(!state.tabbed_property_dialog.open);
        assert_eq!(state.schematic.components[0].name, "R99");
        assert_eq!(state.schematic.components[0].params, "m=2");
        assert_eq!(state.schematic.undo_history.undo_count(), 2);

        assert!(state.schematic.undo());
        assert_eq!(state.schematic.components[0].name, "R99");
        assert_eq!(state.schematic.components[0].params, "");
        assert!(state.schematic.undo());
        assert_eq!(state.schematic.components[0].name, "R1");
    }
}
