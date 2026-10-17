//! Prepared structural-reference changes retain their live source borrow.
use super::super::{
    component::Component,
    component_references as reference_edit,
    find_replace::{self, FindField, FindMatch},
    history::SchematicSnapshot,
    probe::SchematicProbe,
};
use super::Schematic;
use std::collections::BTreeMap;
pub struct PreparedComponentTransaction<'a> {
    schematic: &'a mut Schematic,
    components: Vec<Component>,
}
impl PreparedComponentTransaction<'_> {
    pub fn changes_document(&self) -> bool {
        self.schematic.document.components != self.components
    }
    pub fn has_pending_operation(&self) -> bool {
        self.schematic.pending_operation_id().is_some()
    }
    pub fn commit_local(self, description: &str) {
        let before = SchematicSnapshot::capture(&self.schematic.document);
        self.schematic.document.components = self.components;
        self.schematic.invalidate_topology();
        self.schematic.begin_operation_from(before, description);
        self.schematic.end_operation();
    }
    pub fn into_reference_candidate(self) -> Schematic {
        let mut candidate = self.schematic.clone();
        candidate.document.components = self.components;
        candidate.invalidate_topology();
        candidate
    }
}

/// Probe values remapped and validated for an upper project reference transaction.
pub struct PreparedProbeReferences {
    probes: Vec<SchematicProbe>,
}
impl PreparedProbeReferences {
    pub fn apply_to(self, candidate: &mut Schematic) {
        candidate.document.probes = self.probes;
    }
}

impl Schematic {
    pub fn prepare_component_transaction(
        &mut self,
        expected: &Component,
        candidate: Component,
    ) -> Result<PreparedComponentTransaction<'_>, String> {
        let components =
            reference_edit::prepare_component_edit(&self.document, expected, candidate)?;
        Ok(PreparedComponentTransaction {
            schematic: self,
            components,
        })
    }
    pub fn renamed_reference_candidate(
        &self,
        names: &BTreeMap<u64, String>,
    ) -> Result<Self, String> {
        let mut candidate = self.clone();
        candidate.document.components =
            reference_edit::prepare_component_renames(&self.document, names)?;
        candidate.invalidate_topology();
        Ok(candidate)
    }
    /// The document a find and replace leaves, as a reference candidate:
    /// every listed text changed, and each instance rename carried into the
    /// other instances that name it, as a manual rename carries it. Returns
    /// the candidate and how many texts changed.
    pub fn replaced_text_reference_candidate(
        &self,
        matches: &[FindMatch],
    ) -> Result<(Self, usize), String> {
        if let Some(refusal) = find_replace::refusal(&self.document, matches) {
            return Err(refusal);
        }
        let (renames, texts): (Vec<FindMatch>, Vec<FindMatch>) = matches
            .iter()
            .cloned()
            .partition(|found| found.field == FindField::InstanceName);
        let mut candidate = self.clone();
        let mut changed = find_replace::replace_in_document(&mut candidate.document, &texts);
        let names: BTreeMap<u64, String> = renames
            .into_iter()
            .filter(|found| {
                candidate.document.components.iter().any(|component| {
                    component.id == found.object_id && component.name == found.before
                })
            })
            .map(|found| (found.object_id, found.after))
            .collect();
        changed += names.len();
        candidate.document.components =
            reference_edit::prepare_component_renames(&candidate.document, &names)?;
        candidate.invalidate_topology();
        Ok((candidate, changed))
    }
    pub fn reference_history_candidate(&self, target: &SchematicSnapshot) -> Self {
        let mut candidate = self.clone();
        candidate.apply_snapshot(target);
        candidate.document.probes.clone_from(&self.document.probes);
        candidate
    }
    pub fn prepare_probe_reference_update(
        &self,
        mappings: &crate::references::PathMappings,
    ) -> Result<Option<PreparedProbeReferences>, String> {
        crate::references::remap_schematic_probes(&self.document.probes, mappings)
            .map(|probes| probes.map(|probes| PreparedProbeReferences { probes }))
    }
}
