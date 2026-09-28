//! Application entry points for document-owned structural-reference edits.

#[cfg(test)]
use super::ComponentType;
#[cfg(test)]
use super::reference_edit::PreparedCopyReferences;
use super::{Component, SchematicState, reference_edit};

impl SchematicState {
    #[cfg(test)]
    pub(crate) fn prepare_component_edit(
        &self,
        expected: &Component,
        candidate: Component,
    ) -> Result<Vec<Component>, String> {
        reference_edit::prepare_component_edit(&self.document, expected, candidate)
    }

    pub(crate) fn prepare_component_renames(
        &self,
        names: &std::collections::BTreeMap<u64, String>,
    ) -> Result<Vec<Component>, String> {
        reference_edit::prepare_component_renames(&self.document, names)
    }
}

#[cfg(test)]
mod tests;

/// One validated component/reference candidate bound to its current document.
pub(crate) struct PreparedComponentTransaction<'a> {
    schematic: &'a mut SchematicState,
    components: Vec<Component>,
}

impl PreparedComponentTransaction<'_> {
    pub(crate) fn changes_document(&self) -> bool {
        self.schematic.document.components != self.components
    }

    pub(crate) fn has_pending_operation(&self) -> bool {
        self.schematic.has_pending_operation()
    }

    pub(crate) fn commit_local(self, description: &str) {
        let before = super::SchematicSnapshot::capture(&self.schematic.document);
        self.schematic.document.components = self.components;
        self.schematic.is_dirty = true;
        self.schematic.bump_topology_version();
        self.schematic.commit_undo_from(before, description);
    }

    pub(crate) fn into_reference_candidate(self) -> SchematicState {
        let mut candidate = self.schematic.clone();
        candidate.document.components = self.components;
        candidate.is_dirty = true;
        candidate.bump_topology_version();
        candidate
    }
}

/// Probe values already remapped and validated for an upper reference transaction.
pub(crate) struct PreparedProbeReferences {
    probes: Vec<super::SchematicProbe>,
}

impl PreparedProbeReferences {
    pub(crate) fn apply_to(self, candidate: &mut SchematicState) {
        candidate.document.probes = self.probes;
        candidate.is_dirty = true;
    }
}

impl SchematicState {
    pub(crate) fn prepare_component_transaction(
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

    pub(crate) fn renamed_reference_candidate(
        &self,
        names: &std::collections::BTreeMap<u64, String>,
    ) -> Result<Self, String> {
        let mut candidate = self.clone();
        candidate.document.components = self.prepare_component_renames(names)?;
        candidate.is_dirty = true;
        candidate.bump_topology_version();
        Ok(candidate)
    }

    pub(crate) fn reference_history_candidate(&self, target: &super::SchematicSnapshot) -> Self {
        let mut candidate = self.clone();
        candidate.apply_snapshot(target);
        // A restored edit keeps current probe occurrences until their references are remapped.
        candidate.document.probes.clone_from(&self.document.probes);
        candidate
    }

    pub(crate) fn prepare_probe_reference_update(
        &self,
        mappings: &rspice_design::references::PathMappings,
    ) -> Result<Option<PreparedProbeReferences>, String> {
        rspice_design::references::remap_schematic_probes(&self.document.probes, mappings)
            .map(|probes| probes.map(|probes| PreparedProbeReferences { probes }))
    }
}
