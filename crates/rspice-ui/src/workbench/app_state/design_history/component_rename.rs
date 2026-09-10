//! One publication boundary for a component name and the live references
//! carried with it. History owns exact affected content, never retained runs.

use super::*;
use crate::product::{SavedOutputId, SimulationPlanId};
use crate::state::{
    Component, ConfigurationSetCatalog, ConfigurationSetDefinition, ConfigurationSetId,
    SavedOutput, remap_instance_probes,
};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub(super) struct ComponentRenameRecord {
    document: CellViewRef,
    before: SchematicSnapshot,
    after: SchematicSnapshot,
    configurations: Vec<ConfigurationChange>,
    outputs: Vec<OutputChange>,
}

#[derive(Debug, Clone)]
struct ConfigurationChange {
    id: ConfigurationSetId,
    before: ConfigurationSetDefinition,
    after: ConfigurationSetDefinition,
}

#[derive(Debug, Clone)]
struct OutputChange {
    plan: SimulationPlanId,
    id: SavedOutputId,
    before: String,
    after: String,
}

/// Fallible work is completed before either the schematic or metadata moves.
struct PreparedReferences {
    configurations: ConfigurationSetCatalog,
    outputs: Vec<(SimulationPlanId, SavedOutput)>,
}

impl AppState {
    pub(crate) fn rename_component_transaction(
        &mut self,
        expected: &Component,
        name: String,
    ) -> Result<bool, String> {
        let document = self.workspace.active_schematic_reference();
        let before = SchematicSnapshot::capture(&self.schematic);
        let mut after = before.clone();
        after.components = self
            .schematic
            .prepare_component_rename(expected, name.clone())?;
        if before.is_equal(&after) {
            return Ok(false);
        }
        let occurrence = self.workspace.occurrence_path();
        let from = occurrence
            .child(&expected.name)
            .map_err(|error| error.to_string())?;
        let to = occurrence.child(&name).map_err(|error| error.to_string())?;
        // Primitive current probes name the emitted SPICE card, which may
        // differ from a legacy/imported component's display name.
        let probe_from = if expected.kind == ComponentType::CellInstance {
            from.clone()
        } else {
            occurrence
                .child(&expected.emitted_instance_name())
                .map_err(|error| error.to_string())?
        };
        let root = self
            .workspace
            .active_occurrence()
            .map_or_else(|| document.clone(), |occurrence| occurrence.root.clone());
        let mut configurations = self.workspace.configuration_sets.clone();
        configurations
            .remap_instance_paths_in_root(&root, &from, &to)
            .map_err(|error| error.to_string())?;
        let configurations = configurations
            .configurations()
            .iter()
            .filter_map(|after| {
                let before = self.workspace.configuration_sets.find(after.id())?;
                (before.definition() != after.definition()).then(|| ConfigurationChange {
                    id: after.id(),
                    before: before.definition().clone(),
                    after: after.definition().clone(),
                })
            })
            .collect();
        let mut outputs = Vec::new();
        if root
            .key()
            .eq_ignore_ascii_case(&self.workspace.simulation_root_reference().key())
        {
            for record in &self.workspace.simulation_plan_payloads {
                for output in &record.payload.saved_outputs {
                    if let Some(after) =
                        remap_instance_probes(&output.source_expression, &probe_from, &to)?
                    {
                        outputs.push(OutputChange {
                            plan: record.plan_id,
                            id: output.id,
                            before: output.source_expression.clone(),
                            after,
                        });
                    }
                }
            }
        }
        for probe in &mut after.probes {
            if let Some(expression) = &probe.source_expression
                && let Some(rewritten) = remap_instance_probes(expression, &probe_from, &to)?
            {
                if probe.reference == *expression {
                    probe.reference = rewritten.clone();
                }
                probe.source_expression = Some(rewritten);
                probe.validate()?;
            }
        }
        let record = ComponentRenameRecord {
            document: document.clone(),
            before,
            after,
            configurations,
            outputs,
        };
        let prepared = record.prepare(self, true)?;
        record.publish(self, true, prepared);
        self.schematic.undo_history.clear_redo();
        self.workspace.save_active_schematic(&self.schematic);
        self.push_project_record(
            RecordHeader::committed(
                vec![DocumentCompensation::naming(document.clone())],
                Some(document.clone()),
                Some(document),
            ),
            ProjectDesignBody::ComponentRename(Box::new(record)),
        );
        Ok(true)
    }
}

impl ComponentRenameRecord {
    pub(super) fn after_design_matches(&self, state: &AppState) -> bool {
        self.matches(state, false)
    }
    pub(super) fn before_design_matches(&self, state: &AppState) -> bool {
        self.matches(state, true)
    }

    fn matches(&self, state: &AppState, forward: bool) -> bool {
        let expected = if forward { &self.before } else { &self.after };
        schematic_for_reference(state, &self.document)
            .is_some_and(|schematic| expected.is_equal_state(schematic))
            && self.configurations.iter().all(|change| {
                state
                    .workspace
                    .configuration_sets
                    .find(change.id)
                    .is_some_and(|current| {
                        current.definition()
                            == if forward {
                                &change.before
                            } else {
                                &change.after
                            }
                    })
            })
            && self.outputs.iter().all(|change| {
                state
                    .workspace
                    .plan_data(change.plan)
                    .and_then(|payload| {
                        payload
                            .saved_outputs
                            .iter()
                            .find(|output| output.id == change.id)
                    })
                    .is_some_and(|output| {
                        output.source_expression
                            == if forward {
                                &change.before
                            } else {
                                &change.after
                            }
                            .as_str()
                    })
            })
    }

    pub(super) fn validate_mutation(
        &self,
        state: &AppState,
        operation: &str,
    ) -> Result<(), String> {
        self.prepare(state, operation != "undone").map(|_| ())
    }

    fn prepare(&self, state: &AppState, forward: bool) -> Result<PreparedReferences, String> {
        if !state.project_lifecycle.project_open || document_read_only(state, &self.document) {
            return Err(
                "Component rename requires an open, writable project and document.".to_owned(),
            );
        }
        if !self.matches(state, forward) {
            return Err(
                "Component rename cannot be applied because its document or references changed."
                    .to_owned(),
            );
        }
        if schematic_for_reference(state, &self.document)
            .is_some_and(SchematicState::has_pending_operation)
        {
            return Err(
                "Finish or cancel the active schematic gesture before renaming.".to_owned(),
            );
        }
        let mut configurations = state.workspace.configuration_sets.clone();
        for change in &self.configurations {
            let revision = configurations
                .find(change.id)
                .expect("guarded configuration")
                .revision();
            configurations
                .update(
                    change.id,
                    revision,
                    if forward {
                        &change.after
                    } else {
                        &change.before
                    }
                    .clone(),
                )
                .map_err(|error| error.to_string())?;
        }
        let mut outputs = Vec::with_capacity(self.outputs.len());
        for change in &self.outputs {
            let mut output = state
                .workspace
                .plan_data(change.plan)
                .expect("guarded plan")
                .saved_outputs
                .iter()
                .find(|output| output.id == change.id)
                .expect("guarded output")
                .clone();
            output.source_expression = if forward {
                &change.after
            } else {
                &change.before
            }
            .clone();
            output.revision = output.revision.next().map_err(|error| error.to_string())?;
            output.validate()?;
            outputs.push((change.plan, output));
        }
        Ok(PreparedReferences {
            configurations,
            outputs,
        })
    }

    fn publish(&self, state: &mut AppState, forward: bool, prepared: PreparedReferences) {
        let schematic = if state.workspace.active_schematic_reference() == self.document {
            &mut state.schematic
        } else {
            state
                .workspace
                .schematic_buffers
                .get_mut(&self.document.key())
                .expect("guarded document")
        };
        // All object IDs are retained, so selection is still valid.
        let selection = schematic.selection.clone();
        if forward { &self.after } else { &self.before }.apply(schematic);
        schematic.selection = selection;
        if state.workspace.active_schematic_reference() == self.document {
            state.workspace.save_active_schematic(&state.schematic);
        }
        state.workspace.configuration_sets = prepared.configurations;
        for (plan, replacement) in prepared.outputs {
            let target = state
                .workspace
                .plan_data_mut(plan)
                .expect("guarded plan")
                .saved_outputs
                .iter_mut()
                .find(|output| output.id == replacement.id)
                .expect("guarded output");
            *target = replacement;
        }
        state.workspace.project_metadata_dirty = true;
        state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
        state.ui.netlist.current_generation_input_digest = None;
    }

    pub(super) fn apply_before(&mut self, state: &mut AppState) -> Result<(), String> {
        let prepared = self.prepare(state, false)?;
        self.publish(state, false, prepared);
        Ok(())
    }

    pub(super) fn apply_after(&mut self, state: &mut AppState) -> Result<(), String> {
        let prepared = self.prepare(state, true)?;
        self.publish(state, true, prepared);
        Ok(())
    }
}
