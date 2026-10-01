//! Application entry points for document-owned structural-reference edits.

#[cfg(test)]
use super::ComponentType;
#[cfg(test)]
use super::reference_edit;
#[cfg(test)]
use super::reference_edit::PreparedCopyReferences;
use super::{Component, SchematicEditorRef, SchematicState};

impl SchematicState {
    #[cfg(test)]
    pub(crate) fn prepare_component_edit(
        &self,
        expected: &Component,
        candidate: Component,
    ) -> Result<Vec<Component>, String> {
        reference_edit::prepare_component_edit(self.document(), expected, candidate)
    }

    #[cfg(test)]
    pub(crate) fn prepare_component_renames(
        &self,
        names: &std::collections::BTreeMap<u64, String>,
    ) -> Result<Vec<Component>, String> {
        reference_edit::prepare_component_renames(&self.design.document(), names)
    }
}

#[cfg(test)]
mod tests;

impl SchematicState {
    pub(crate) fn prepare_component_transaction(
        &mut self,
        expected: &Component,
        candidate: Component,
    ) -> Result<rspice_design::schematic::owned::references::PreparedComponentTransaction<'_>, String>
    {
        self.design
            .prepare_component_transaction(expected, candidate)
    }
}

impl SchematicEditorRef<'_> {
    pub(crate) fn renamed_reference_candidate(
        &self,
        names: &std::collections::BTreeMap<u64, String>,
    ) -> Result<SchematicState, String> {
        let design = self.design.renamed_reference_candidate(names)?;
        let mut candidate = self.with_design(design);
        candidate.session.is_dirty = true;
        Ok(candidate)
    }
    pub(crate) fn reference_history_candidate(
        &self,
        target: &super::SchematicSnapshot,
    ) -> SchematicState {
        let mut candidate = self.with_design(self.design.reference_history_candidate(target));
        candidate.session.is_dirty = true;
        candidate.session.editor.selection.clear();
        candidate
    }
}
