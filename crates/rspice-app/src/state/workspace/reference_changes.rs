//! Exact live-reference deltas shared by component and annotation transactions.

use super::*;
use crate::product::{SavedOutputId, SimulationPlanId};
use crate::state::workspace::DocumentOccurrence;
use crate::state::{
    Component, ConfigurationSetCatalog, ConfigurationSetDefinition, ConfigurationSetId,
    InstancePath, SavedOutput,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct ReferenceChanges {
    configurations: Vec<ConfigurationChange>,
    outputs: Vec<OutputChange>,
    instances: Vec<InstanceNameChange>,
}

#[derive(Debug, Clone)]
struct InstanceNameChange {
    document: CellViewRef,
    before: String,
    after: String,
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

#[derive(Clone)]
pub(crate) struct PreparedReferences {
    configurations: ConfigurationSetCatalog,
    outputs: Vec<(SimulationPlanId, SavedOutput)>,
    occurrences: Vec<(CellViewRef, DocumentOccurrence)>,
}

impl ReferenceChanges {
    pub(crate) fn reversed(mut self) -> Self {
        for change in &mut self.configurations {
            std::mem::swap(&mut change.before, &mut change.after);
        }
        for change in &mut self.outputs {
            std::mem::swap(&mut change.before, &mut change.after);
        }
        for change in &mut self.instances {
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
            instances: Vec::new(),
            configurations: configurations
                .configurations()
                .iter()
                .filter_map(|after| {
                    let before = workspace.configuration_sets.find(after.id())?;
                    (before.definition() != after.definition()).then(|| ConfigurationChange {
                        id: after.id(),
                        before: before.definition().clone(),
                        after: after.definition().clone(),
                    })
                })
                .collect(),
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
        let candidates: BTreeMap<_, _> = after
            .iter()
            .map(|component| (component.id, component))
            .collect();
        for component in before {
            if component.kind != ComponentType::CellInstance {
                continue;
            }
            if let Some(candidate) = candidates.get(&component.id)
                && component.name != candidate.name
            {
                self.instances.push(InstanceNameChange {
                    document: document.clone(),
                    before: component.name.clone(),
                    after: candidate.name.clone(),
                });
            }
        }
    }

    fn prepare_occurrences(
        &self,
        workspace: &ProjectWorkspace,
        forward: bool,
    ) -> Result<Vec<(CellViewRef, DocumentOccurrence)>, String> {
        let mut names = BTreeMap::new();
        for change in &self.instances {
            let (from, to) = if forward {
                (&change.before, &change.after)
            } else {
                (&change.after, &change.before)
            };
            let key = (
                change.document.key().to_ascii_lowercase(),
                from.to_ascii_lowercase(),
            );
            if let Some(previous) = names.insert(key, to)
                && previous != to
            {
                return Err("The renamed instance has ambiguous hierarchy occurrences.".to_owned());
            }
        }
        if names.is_empty() {
            return Ok(Vec::new());
        }
        let mut updates = Vec::new();
        for open in &workspace.open_views {
            let mut occurrence = open.occurrence.clone();
            let mut parent = occurrence.root.clone();
            let mut changed = false;
            for step in &mut occurrence.steps {
                if let Some(name) = names.get(&(
                    parent.key().to_ascii_lowercase(),
                    step.instance_name.to_ascii_lowercase(),
                )) {
                    changed |= step.instance_name != **name;
                    step.instance_name.clone_from(name);
                }
                parent.clone_from(&step.master);
            }
            if changed {
                let mut path = InstancePath::root();
                for step in &occurrence.steps {
                    path = path
                        .child(&step.instance_name)
                        .map_err(|error| error.to_string())?;
                }
                updates.push((open.reference.clone(), occurrence));
            }
        }
        Ok(updates)
    }

    pub(crate) fn matches(&self, workspace: &ProjectWorkspace, forward: bool) -> bool {
        self.configurations.iter().all(|change| {
            workspace
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
        }) && self.outputs.iter().all(|change| {
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
        if !self.matches(workspace, forward) {
            return Err(
                "The configuration or saved-output references changed before commit.".to_owned(),
            );
        }
        let mut configurations = workspace.configuration_sets.clone();
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
            occurrences: self.prepare_occurrences(workspace, forward)?,
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
