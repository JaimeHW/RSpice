//! Reversible configuration definitions and instance names for reference edits.

use super::document_occurrence::DocumentOccurrence;
use rspice_app_types::hierarchy_path::InstancePath;
use rspice_design::configuration_set::{
    ConfigurationSetCatalog, ConfigurationSetDefinition, ConfigurationSetId,
};
use rspice_design::schematic::{component::Component, component_type::ComponentType};
use rspice_design_model::cell_view::CellViewRef;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub(crate) struct DesignReferenceChanges {
    configurations: Vec<ConfigurationChange>,
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

/// A matched catalog borrowed until candidate preparation. This is not
/// authority to publish into a project or a different catalog.
pub(crate) struct CheckedConfigurationReferences<'a> {
    changes: &'a [ConfigurationChange],
    current: &'a ConfigurationSetCatalog,
    forward: bool,
}

impl DesignReferenceChanges {
    pub(crate) fn reversed(mut self) -> Self {
        for change in &mut self.configurations {
            std::mem::swap(&mut change.before, &mut change.after);
        }
        for change in &mut self.instances {
            std::mem::swap(&mut change.before, &mut change.after);
        }
        self
    }

    pub(crate) fn between(
        before: &ConfigurationSetCatalog,
        configurations: &ConfigurationSetCatalog,
    ) -> Self {
        Self {
            instances: Vec::new(),
            configurations: configurations
                .configurations()
                .iter()
                .filter_map(|after| {
                    let before = before.find(after.id())?;
                    (before.definition() != after.definition()).then(|| ConfigurationChange {
                        id: after.id(),
                        before: before.definition().clone(),
                        after: after.definition().clone(),
                    })
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

    pub(crate) fn prepare_occurrences<'a>(
        &self,
        occurrences: impl IntoIterator<Item = (&'a CellViewRef, &'a DocumentOccurrence)>,
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
        for (reference, source) in occurrences {
            let mut occurrence = source.clone();
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
                updates.push((reference.clone(), occurrence));
            }
        }
        Ok(updates)
    }

    /// Borrow the exact catalog whose definitions match this history direction.
    pub(crate) fn checked_configurations<'a>(
        &'a self,
        current: &'a ConfigurationSetCatalog,
        forward: bool,
    ) -> Option<CheckedConfigurationReferences<'a>> {
        if !self.configurations.iter().all(|change| {
            current.find(change.id).is_some_and(|current| {
                current.definition()
                    == if forward {
                        &change.before
                    } else {
                        &change.after
                    }
            })
        }) {
            return None;
        }
        Some(CheckedConfigurationReferences {
            changes: &self.configurations,
            current,
            forward,
        })
    }
}

impl CheckedConfigurationReferences<'_> {
    /// Prepare from the matched source, advancing its current revisions.
    pub(crate) fn prepare(self) -> Result<ConfigurationSetCatalog, String> {
        let mut configurations = self.current.clone();
        for change in self.changes {
            let revision = configurations
                .find(change.id)
                .expect("guarded configuration")
                .revision();
            configurations
                .update(
                    change.id,
                    revision,
                    if self.forward {
                        &change.after
                    } else {
                        &change.before
                    }
                    .clone(),
                )
                .map_err(|error| error.to_string())?;
        }
        Ok(configurations)
    }
}

#[cfg(test)]
mod tests;
