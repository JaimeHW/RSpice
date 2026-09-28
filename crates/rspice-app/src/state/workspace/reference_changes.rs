//! Exact live-reference deltas shared by component and annotation transactions.

use super::design_reference_changes::DesignReferenceChanges;
use super::{DocumentOccurrence, ProjectWorkspace};
use crate::product::{SavedOutputId, SimulationPlanId};
use crate::state::{CellViewRef, Component, ConfigurationSetCatalog, SavedOutput};

#[derive(Debug, Clone, Default)]
pub(crate) struct ReferenceChanges {
    design: DesignReferenceChanges,
    outputs: Vec<OutputChange>,
}

#[derive(Debug, Clone)]
struct OutputChange {
    plan: SimulationPlanId,
    id: SavedOutputId,
    before: String,
    after: String,
}

#[derive(Clone)]
pub(crate) struct PreparedReferences {
    configurations: ConfigurationSetCatalog,
    outputs: Vec<(SimulationPlanId, SavedOutput)>,
    occurrences: Vec<(CellViewRef, DocumentOccurrence)>,
}

impl ReferenceChanges {
    pub(crate) fn reversed(mut self) -> Self {
        self.design = self.design.reversed();
        for change in &mut self.outputs {
            std::mem::swap(&mut change.before, &mut change.after);
        }
        self
    }

    pub(crate) fn between(
        workspace: &ProjectWorkspace,
        configurations: &ConfigurationSetCatalog,
        outputs: Vec<(SimulationPlanId, SavedOutput)>,
    ) -> Self {
        Self {
            design: DesignReferenceChanges::between(&workspace.configuration_sets, configurations),
            outputs: outputs
                .into_iter()
                .map(|(plan, output)| {
                    let before = workspace
                        .plan_data(plan)
                        .expect("source plan")
                        .saved_outputs
                        .iter()
                        .find(|candidate| candidate.id == output.id)
                        .expect("source output");
                    OutputChange {
                        plan,
                        id: output.id,
                        before: before.source_expression.clone(),
                        after: output.source_expression,
                    }
                })
                .collect(),
        }
    }

    pub(crate) fn add_instance_renames(
        &mut self,
        document: &CellViewRef,
        before: &[Component],
        after: &[Component],
    ) {
        self.design.add_instance_renames(document, before, after);
    }

    pub(crate) fn matches(&self, workspace: &ProjectWorkspace, forward: bool) -> bool {
        self.design
            .checked_configurations(&workspace.configuration_sets, forward)
            .is_some()
            && self.outputs_match(workspace, forward)
    }

    fn outputs_match(&self, workspace: &ProjectWorkspace, forward: bool) -> bool {
        self.outputs.iter().all(|change| {
            workspace
                .plan_data(change.plan)
                .and_then(|payload| {
                    payload
                        .saved_outputs
                        .iter()
                        .find(|output| output.id == change.id)
                })
                .is_some_and(|output| {
                    output.source_expression
                        == *if forward {
                            &change.before
                        } else {
                            &change.after
                        }
                })
        })
    }

    /// Finish revision arithmetic and validation before any owner changes.
    pub(crate) fn prepare(
        &self,
        workspace: &ProjectWorkspace,
        forward: bool,
    ) -> Result<PreparedReferences, String> {
        let Some(configurations) = self
            .design
            .checked_configurations(&workspace.configuration_sets, forward)
            .filter(|_| self.outputs_match(workspace, forward))
        else {
            return Err(
                "The configuration or saved-output references changed before commit.".to_owned(),
            );
        };
        let configurations = configurations.prepare()?;
        let mut outputs = Vec::with_capacity(self.outputs.len());
        for change in &self.outputs {
            let mut output = workspace
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
            occurrences: self.design.prepare_occurrences(
                workspace
                    .open_views
                    .iter()
                    .map(|open| (&open.reference, &open.occurrence)),
                forward,
            )?,
        })
    }
}

impl PreparedReferences {
    pub(crate) fn publish(self, workspace: &mut ProjectWorkspace) {
        workspace.replace_document_occurrences(self.occurrences);
        workspace.configuration_sets = self.configurations;
        for (plan, replacement) in self.outputs {
            let target = workspace
                .plan_data_mut(plan)
                .expect("guarded plan")
                .saved_outputs
                .iter_mut()
                .find(|output| output.id == replacement.id)
                .expect("guarded output");
            *target = replacement;
        }
        workspace.project_metadata_dirty = true;
    }
}
