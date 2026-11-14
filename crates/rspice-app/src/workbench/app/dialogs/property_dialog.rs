//! Property dialog host.
//!
//! The floating schema-driven component editor backed by the
//! `PropertyEditorSchema`. Inline inspection lives in the workbench inspector
//! (`crate::workbench::docks::inspector`).

use std::collections::HashMap;

use crate::properties::{
    ComponentPropertyContext, PropertyEditorSchema, TabbedPropertyDialogState,
    render_tabbed_property_dialog,
};
use crate::state::{Component, ComponentType, PropertySheet, PropertyValue};
use crate::workbench::app::actions::workspace::sync_active_schematic;
use crate::workbench::app::dialogs::design_request::DesignRequest;
use crate::workbench::app::dialogs::stimulus_link::{
    StimulusLinkDialogState, StimulusLinkMode, open_stimulus_link, show_stimulus_definition,
};
use crate::workbench::app_state::Model;
use crate::workbench::state::WorkbenchSession;
use rspice_schematic_editor::component_properties::{
    ComponentEditorContext, ComponentPropertyDialogResult, StimulusEditorContext,
};

/// What the schematic component editor draws over and changes: its draft,
/// the property sheets, the stimulus link it hands an instance to, the
/// definition it shows in the Stimulus Library, the model and the session.
pub(crate) struct PropertyDialog<'a> {
    pub(crate) dialog: &'a mut TabbedPropertyDialogState,
    pub(crate) registry: &'a mut PropertyEditorSchema,
    pub(crate) stimulus_link: &'a mut StimulusLinkDialogState,
    pub(crate) selected_stimulus_definition: &'a mut Option<String>,
    pub(crate) model: &'a mut Model,
    pub(crate) workbench: &'a mut WorkbenchSession,
}

impl PropertyDialog<'_> {
    /// Render the floating schematic component editor. A re-adopted stimulus
    /// asks the shell to reopen the editor on the instance's new card.
    pub(crate) fn render_property_dialog(
        &mut self,
        ctx: &egui::Context,
    ) -> ComponentPropertyDialogResult {
        if self.dialog.open {
            self.dialog.session_error = component_property_session_error(self.dialog, self.model);
            refresh_source_contract_advisories(self.dialog, self.registry, self.model);
        }
        let quantity_policy = self.model.preferences.quantity_presentation_policy();
        let number_locale = self.workbench.number_locale;
        self.dialog.draft.standard_series = self.model.workspace.content.standard_value_series;
        let editor_context = component_editor_context(self.dialog, self.model);
        let result = render_tabbed_property_dialog(
            ctx,
            self.dialog,
            &editor_context,
            self.registry,
            &self.model.model_library_manager,
            quantity_policy,
            number_locale,
        );

        // Handle dialog result - apply changes back to component
        if matches!(
            result,
            ComponentPropertyDialogResult::Applied | ComponentPropertyDialogResult::AppliedAndClose
        ) && let Some(comp_id) = self.dialog.component_id
        {
            let close_after_commit = result == ComponentPropertyDialogResult::AppliedAndClose;
            if let Some(error) = component_property_session_error(self.dialog, self.model) {
                self.dialog.open = true;
                self.dialog.session_error = Some(error);
                return ComponentPropertyDialogResult::None;
            }
            let committed = self.dialog.draft.take_prepared_commit();
            let committed_names: Vec<String> = committed.keys().cloned().collect();
            // The bridge serializes the full component property map. Merge the
            // validated delta onto the last committed baseline so a rejected
            // partial field cannot erase an unrelated existing parameter.
            let mut values = self.dialog.draft.original_values.clone();
            values.extend(committed);
            let Some(component) = self
                .model
                .schematic
                .document()
                .components
                .iter()
                .find(|c| c.id == comp_id)
            else {
                self.dialog.open = true;
                self.dialog.session_error = Some(
                    "The selected component no longer exists. Close and reopen Object properties."
                        .to_owned(),
                );
                return ComponentPropertyDialogResult::None;
            };
            let mut candidate = component.clone();
            if let Err(error) = crate::properties::property_bridge::apply_properties_to_component(
                &mut candidate,
                &values,
                self.registry,
            ) {
                self.dialog.open = true;
                self.dialog.draft.commit_error = Some(format!(
                    "The component was not changed: {error} Correct its Parameters text in the inspector."
                ));
                return ComponentPropertyDialogResult::None;
            }
            if let Err(error) = validate_component_identity(self.model, comp_id, &candidate) {
                self.dialog.open = true;
                self.dialog
                    .draft
                    .validation_errors
                    .insert("name".to_owned(), error);
                self.dialog.draft.commit_error =
                    Some("Correct the instance reference before applying.".to_owned());
                return ComponentPropertyDialogResult::None;
            }
            if let Err(error) = validate_model_binding_authority(self.model, &candidate, &values) {
                self.dialog.open = true;
                self.dialog.draft.commit_error =
                    Some(format!("The model binding was not changed: {error}"));
                return ComponentPropertyDialogResult::None;
            }
            let Some(property_sheet) = self.registry.get(candidate.kind) else {
                self.dialog.open = true;
                self.dialog.draft.commit_error = Some(
                    "The component was not changed because its property schema is unavailable."
                        .to_owned(),
                );
                return ComponentPropertyDialogResult::None;
            };
            if let Err(error) = validate_component_contract(&candidate, &values, property_sheet) {
                self.dialog.open = true;
                self.dialog.draft.commit_error =
                    Some(format!("The component was not changed: {error}"));
                return ComponentPropertyDialogResult::None;
            }
            if candidate.kind == crate::state::ComponentType::Port
                && let Err(error) = self
                    .model
                    .schematic
                    .validate_edited_port_contract(comp_id, &candidate)
            {
                self.dialog.open = true;
                self.dialog.draft.commit_error =
                    Some(format!("The interface contract was not changed: {error}."));
                return ComponentPropertyDialogResult::None;
            }
            let changed_port_contract =
                candidate.kind == crate::state::ComponentType::Port && &candidate != component;
            let expected = component.clone();
            if let Err(error) =
                self.model
                    .edit_component_transaction(&expected, candidate, "edit properties")
            {
                self.dialog.open = true;
                self.dialog.draft.commit_error =
                    Some(format!("The component was not changed: {error}"));
                return ComponentPropertyDialogResult::None;
            }
            if changed_port_contract {
                sync_active_schematic(self.model, self.workbench);
            }

            // The mockup primary closes after the exact transaction completes.
            if close_after_commit {
                self.dialog.close();
            } else if self.dialog.open {
                let component = self
                    .model
                    .schematic
                    .document()
                    .components
                    .iter()
                    .find(|component| component.id == comp_id)
                    .expect("committed component retains its identity");
                let values = crate::properties::property_bridge::collect_properties_from_component(
                    component,
                    self.registry,
                );
                self.dialog.component_baseline = Some(component.clone());
                self.dialog.component_name = Some(component.name.clone());
                self.dialog.source = Some(crate::workbench::app::schematic_editor_request_source(
                    &*self.model,
                ));
                self.dialog.draft.mark_fields_applied(committed_names);
                self.dialog.draft.rebase_applied_values(
                    values,
                    self.registry.get(component.kind).expect("validated schema"),
                    quantity_policy,
                    number_locale,
                );
            }
        }

        match result {
            ComponentPropertyDialogResult::AdoptStimulus
            | ComponentPropertyDialogResult::ExtractStimulus => {
                // The link transaction is about the instance on the sheet, not
                // about this editor's draft, so the editor closes first: two modal
                // surfaces over one component would let a reader apply one edit
                // through each and see only the second.
                let mode = if result == ComponentPropertyDialogResult::AdoptStimulus {
                    StimulusLinkMode::Adopt
                } else {
                    StimulusLinkMode::Extract
                };
                if let Some(component_id) = self.dialog.component_id {
                    self.dialog.close();
                    if let Err(refusal) =
                        open_stimulus_link(self.stimulus_link, self.model, component_id, mode)
                    {
                        self.workbench.prompt(refusal);
                    }
                }
            }
            ComponentPropertyDialogResult::OpenStimulusDefinition => {
                let definition = editor_context
                    .editor
                    .stimulus
                    .as_ref()
                    .and_then(|stimulus| stimulus.definition.clone());
                self.dialog.close();
                if let Some(definition) = definition {
                    show_stimulus_definition(
                        self.selected_stimulus_definition,
                        self.workbench,
                        &definition,
                    );
                }
            }
            ComponentPropertyDialogResult::ReadoptStimulus => {
                let definition = editor_context
                    .editor
                    .stimulus
                    .as_ref()
                    .and_then(|stimulus| stimulus.definition.clone());
                if let (Some(component_id), Some(definition)) =
                    (self.dialog.component_id, definition)
                {
                    match self.model.readopt_stimulus(component_id, &definition) {
                        Ok(line) => {
                            self.workbench.prompt(line);
                            // Reopened rather than patched in place: the copy is a
                            // whole new card, and the draft the reader was typing
                            // into describes the one it replaced.
                            self.dialog.close();
                            // The shell reopens it straight after this step draws.
                            self.workbench
                                .outbox
                                .push(DesignRequest::ReopenPropertyEditor(component_id));
                        }
                        Err(refusal) => {
                            self.dialog.draft.commit_error = Some(refusal);
                        }
                    }
                }
            }
            _ => {}
        }

        if !self.dialog.open {
            self.registry.clear_cell_instance_sheet();
        }

        result
    }
}

fn validate_model_binding_authority(
    model: &Model,
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

    let provider_libraries = model
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

    let effective = model
        .model_library_manager
        .effective_plan_definition_provider(
            &model.sim_setup.model_bindings,
            ModelConsumerScope::PrimitiveModel,
            &declared,
        )?
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
    model: &Model,
    component_id: u64,
    candidate: &Component,
) -> Result<(), String> {
    if !candidate.kind.spice_prefix().is_empty() {
        candidate
            .validate_reference_designator(candidate.name.trim())
            .map_err(|error| format!("The instance reference was not changed: {error}"))?;
        if model
            .schematic
            .document()
            .components
            .iter()
            .any(|component| {
                component.id != component_id
                    && component.name.eq_ignore_ascii_case(candidate.name.trim())
            })
        {
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
    // Independent sources state their contract in one place, at two strengths,
    // and the gate that reads the refusals is shared with the inspector's
    // inline field editor, so the two commit paths cannot disagree about what
    // a field means. The advisories are surfaced beside the fields by
    // `refresh_source_contract_advisories` without blocking anything.
    if let Some(refusal) =
        crate::properties::property_bridge::source_commit_refusal(component, values, sheet)
    {
        return Err(refusal);
    }
    match component.kind {
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

/// Recompute the open editor's advisories against the current draft.
///
/// This runs every frame the dialog is open rather than on commit, because an
/// advisory exists to be read while the field is being typed into — by the time
/// a commit is attempted the decision it describes has already been made.
///
/// The per-frame work is bounded by one component: the editor's own draft values
/// and that instance's parameter string, re-parsed against its property sheet.
/// It reaches no other component and no net, and it runs only while a modal
/// editor is open — which is why it is not keyed on a revision.
fn refresh_source_contract_advisories(
    dialog: &mut TabbedPropertyDialogState,
    registry: &PropertyEditorSchema,
    model: &Model,
) {
    let advisories = dialog
        .component_id
        .and_then(|component_id| {
            model
                .schematic
                .document()
                .components
                .iter()
                .find(|component| component.id == component_id)
        })
        .and_then(|component| {
            let sheet = registry.get(component.kind)?;
            Some(
                crate::properties::property_bridge::component_source_contract(
                    component,
                    &dialog.draft.values,
                    sheet,
                )
                .into_iter()
                .filter(|finding| finding.strength == crate::state::ContractStrength::Advisory)
                .collect::<Vec<_>>(),
            )
        })
        .unwrap_or_default();
    dialog.source_advisories = advisories;
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

fn component_editor_context(
    dialog: &TabbedPropertyDialogState,
    model: &Model,
) -> ComponentPropertyContext {
    let Some(component_id) = dialog.component_id else {
        return ComponentPropertyContext::default();
    };
    let Some(component) = model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
    else {
        return ComponentPropertyContext::default();
    };

    // The master an instance stands for, and nothing for a primitive: the
    // engine is not a library, and `R / Resistor` beside `R1` said the word
    // the subtitle under it already says.
    let library_cell = component
        .library_cell
        .as_ref()
        .map(|binding| format!("{}/{}/{}", binding.library, binding.cell, binding.view))
        .unwrap_or_default();

    let mut editor = ComponentEditorContext {
        // One word for what this part is: the device for a primitive, the
        // cell and the view it is opened through for an instance.
        subtitle: component
            .library_cell
            .as_ref()
            .map(|binding| format!("{} ({})", binding.cell, binding.view))
            .unwrap_or_else(|| component.kind.display_name().to_owned()),
        library_cell,
        stimulus: None,
    };
    let (stimulus, retained_table) = component_stimulus_context(model, component);
    editor.stimulus = stimulus;
    ComponentPropertyContext {
        editor,
        retained_table,
    }
}

/// Where this instance stands with the project's stimulus library.
///
/// Read from the library on every frame, exactly as the model binding and the
/// terminals are: the library is project state, and an editor holding its own
/// copy of a lifecycle word is how a chip comes to disagree with the page that
/// computed it.
fn component_stimulus_context(
    model: &Model,
    component: &Component,
) -> (
    Option<StimulusEditorContext>,
    Option<crate::properties::RetainedTableFile>,
) {
    if !crate::simulation::stimulus_realize::is_independent_source(component.kind) {
        return (None, None);
    }
    let library = &model.workspace.content.stimulus_library;
    let definition = component
        .stimulus_provenance
        .as_ref()
        .map(|provenance| provenance.definition.clone());
    let editor = StimulusEditorContext {
        state: library.provenance_state(component),
        library_revision: definition
            .as_deref()
            .and_then(|name| library.get(name))
            .map(crate::state::stimulus_library::definition::StimulusDefinition::revision),
        definition,
        library_is_empty: library.is_empty(),
    };
    let retained_table = library.retained_pwl_table(component).and_then(|table| {
        Some(crate::properties::RetainedTableFile {
            reference: crate::simulation::stimulus_realize::data_file_reference(component)?,
            path: crate::simulation::table_route::materialized(table).ok()?,
        })
    });
    (Some(editor), retained_table)
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

pub(crate) fn component_property_session_error(
    dialog: &TabbedPropertyDialogState,
    model: &Model,
) -> Option<String> {
    if model.schematic_edit_read_only() {
        return Some("The active schematic is read-only; no properties can be applied.".to_owned());
    }
    let Some(source) = dialog.source.as_ref() else {
        return Some(
            "The property editor document context is unavailable. Close and reopen Object properties."
                .to_owned(),
        );
    };
    if source.design_epoch != model.design_execution_epoch {
        return Some(
            "The design document changed while properties were open. Close and reopen the current object."
                .to_owned(),
        );
    }
    if source.document_epoch != model.active_schematic_epoch {
        return Some(
            "The active schematic buffer changed while properties were open. Close and reopen the current object."
                .to_owned(),
        );
    }
    if dialog.view_path != model.workspace.content.active_view.display_path() {
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
    let Some(current) = model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == baseline.id)
    else {
        return Some(
            "The selected component no longer exists. Close and reopen Object properties."
                .to_owned(),
        );
    };
    if current != baseline {
        return Some("The selected component changed while properties were open. Close and reopen the current object.".to_owned());
    }
    if source != &crate::workbench::app::schematic_editor_request_source(model) {
        return Some(
            "The schematic source or editing scope changed while properties were open. Close and reopen the current object."
                .to_owned(),
        );
    }
    crate::state::params_string::validate_parameter_text(&current.params)
        .err()
        .map(|error| format!("Parameters text needs correction: {error} Close Object properties and correct it in the inspector."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, Point, PropertyValue};
    use crate::workbench::design::test_host::DesignTestHost;

    #[test]
    fn cross_field_validation_includes_unserialized_schema_defaults() {
        let registry = crate::properties::PropertyEditorSchema::new();
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
        )
        .unwrap();

        assert_eq!(
            validate_component_contract(&component, &values, sheet),
            Err("memristor resistance requires 0 < on resistance < off resistance".to_owned())
        );
    }

    /// Which library executes a model is a project-global decision, and the
    /// editor cannot commit an instance that contradicts it.
    ///
    /// The dialog no longer states the binding at all: the design inspector
    /// owns it, and the card the editor used to draw beside the form showed a
    /// "Section" of `TT process corner` — the project's reference corner — for
    /// every instance that named no section of its own.
    #[test]
    fn an_instance_cannot_override_the_project_global_model_provider() {
        use crate::state::model_library::{
            DeviceModel, ModelConsumerScope, ModelLibrary, ModelType,
        };

        let mut state = DesignTestHost::default();
        state.model.model_library_manager.clear();
        for library_name in ["alpha", "beta"] {
            let mut library = ModelLibrary::new(library_name);
            library.add_model(DeviceModel::new("shared_diode", ModelType::Diode));
            state.model.model_library_manager.add_library(library);
        }
        state
            .model
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

        let values = HashMap::from([(
            "model_library".to_owned(),
            PropertyValue::String("beta".to_owned()),
        )]);
        assert!(validate_model_binding_authority(&state.model, &component, &values).is_ok());
        let mut component_without_library = component.clone();
        component_without_library.params.clear();
        assert!(
            validate_model_binding_authority(
                &state.model,
                &component_without_library,
                &HashMap::new()
            )
            .is_ok(),
            "the project-global decision is authoritative even without redundant instance metadata"
        );

        let values = HashMap::from([(
            "model_library".to_owned(),
            PropertyValue::String("alpha".to_owned()),
        )]);
        let error = validate_model_binding_authority(&state.model, &component, &values)
            .expect_err("instance metadata cannot override the executable provider");
        assert!(error.contains("project-global provider 'beta'"));
    }

    /// A binned family is bound by its name, the way a foundry deck binds it:
    /// the library holding `nch.1` and `nch.2` provides `nch`, and the commit
    /// that names it with that library is accepted.
    #[test]
    fn a_binned_family_commits_under_its_own_name() {
        use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};

        let mut state = DesignTestHost::default();
        state.model.model_library_manager.clear();
        let mut kit = ModelLibrary::new("kit");
        for (bin, (l_min, l_max)) in [(1, (0.1e-6, 1e-6)), (2, (1e-6, 10e-6))] {
            kit.add_model(
                DeviceModel::new(format!("nch.{bin}"), ModelType::Nmos)
                    .with_geometry(l_min, l_max, 0.1e-6, 100e-6),
            );
        }
        state.model.model_library_manager.add_library(kit);
        let mut component =
            Component::new(1, ComponentType::Nmos, Point::origin()).with_name_value("M1", "nch");
        component.params = "model_library=kit".to_owned();
        let values = HashMap::from([(
            "model_library".to_owned(),
            PropertyValue::String("kit".to_owned()),
        )]);
        validate_model_binding_authority(&state.model, &component, &values)
            .expect("the kit provides the family it holds the bins of");

        let elsewhere = HashMap::from([(
            "model_library".to_owned(),
            PropertyValue::String("other".to_owned()),
        )]);
        let error = validate_model_binding_authority(&state.model, &component, &elsewhere)
            .expect_err("only the kit provides nch");
        assert!(error.contains("project-global provider 'kit'"), "{error}");

        component.value = "pch".to_owned();
        let error = validate_model_binding_authority(&state.model, &component, &values)
            .expect_err("no library provides a family it holds no bin of");
        assert!(
            error.contains("does not provide executable model 'pch'"),
            "{error}"
        );
    }
}
