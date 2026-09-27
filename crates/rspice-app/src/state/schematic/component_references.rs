//! Application entry points for document-owned structural-reference edits.

#[cfg(test)]
use super::ComponentType;
#[cfg(test)]
use super::reference_edit::PreparedCopyReferences;
use super::{Component, SchematicState, reference_edit};

impl SchematicState {
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
