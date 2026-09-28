//! Hierarchy candidates preserve design authority without copying their documents again.
use super::super::hierarchy::{HierarchyDocument, HierarchySource};
use super::Schematic;
use super::history::EditHistory;
use std::collections::HashSet;

impl Schematic {
    pub fn hierarchy_source<'a>(
        &'a self,
        selected_components: &'a HashSet<u64>,
        selected_count: usize,
    ) -> HierarchySource<'a> {
        HierarchySource {
            document: &self.document,
            identity: &self.identity,
            topology_version: self.topology_version,
            selected_components,
            selected_count,
        }
    }

    pub fn clone_with_hierarchy_document(&self, candidate: HierarchyDocument) -> Self {
        Self {
            document: candidate.document,
            identity: candidate.identity,
            topology_version: self
                .topology_version
                .wrapping_add(candidate.topology_changes),
            content_version: self.content_version,
            history: self.history.clone(),
        }
    }

    pub fn from_hierarchy_document(candidate: HierarchyDocument) -> Self {
        Self {
            document: candidate.document,
            identity: candidate.identity,
            topology_version: candidate.topology_changes,
            content_version: 0,
            history: EditHistory::default(),
        }
    }
}
