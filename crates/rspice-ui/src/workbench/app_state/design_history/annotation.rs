//! Prepare annotation names and their references before project publication.

use super::reference_preparation::validate_reference_document;
use super::*;

#[cfg(test)]
mod tests;

impl AppState {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn retain_annotation_history_after_save_descriptor(
        &mut self,
        published: &crate::state::ProjectDescriptor,
    ) {
        let Some(path) = &published.path else {
            return;
        };
        let before = self.workspace.project.revision();
        if before == published.revision() {
            return;
        }
        let mut expected = self.workspace.project.clone();
        expected.set_path(path.clone());
        // A first save can name an untitled project. Only that exact descriptor
        // transition may carry the guards forward; delayed saves or unrelated
        // descriptor changes must not acquire authority over newer edits.
        if matches!(
            (serde_json::to_value(&expected), serde_json::to_value(published)),
            (Ok(expected), Ok(published)) if expected == published
        ) {
            self.reanchor_annotation_history_revision(before, published.revision());
        }
    }

    /// A checked annotation compensation restores an earlier project state at
    /// a new monotonic revision. Carry that exact revision relationship into
    /// retained annotation/rename records. Unrelated external revision changes
    /// still fail their guards; they never call this history-only operation.
    pub(super) fn reanchor_annotation_history_revision(
        &mut self,
        previous: ObjectRevision,
        replacement: ObjectRevision,
    ) {
        if previous == replacement {
            return;
        }
        for record in self
            .project_design_history
            .undo
            .iter_mut()
            .chain(&mut self.project_design_history.redo)
        {
            match &mut record.body {
                ProjectDesignBody::DesignManagement(record) => {
                    for revision in [
                        &mut record.before_project_revision,
                        &mut record.undo_guard_revision,
                    ] {
                        if *revision == previous {
                            *revision = replacement;
                        }
                    }
                    if record.redo_guard_revision == Some(previous) {
                        record.redo_guard_revision = Some(replacement);
                    }
                }
                ProjectDesignBody::ComponentRename(record) => {
                    record.reanchor_annotation_revision(previous, replacement)
                }
                _ => {}
            }
        }
    }

    pub(crate) fn prepare_design_management_schematic_transaction(
        &self,
        candidate: &DesignManagementCatalog,
    ) -> Result<SchematicReferenceTransaction, String> {
        candidate.validate().map_err(|error| error.to_string())?;
        let projected = self.schematic_reference_sources();
        let existing = self
            .workspace
            .design_management
            .annotation()
            .effective_mappings();
        let mut names: BTreeMap<String, BTreeMap<u64, String>> = BTreeMap::new();
        for (object, mapping) in candidate.annotation().effective_mappings() {
            if existing.get(&object) == Some(&mapping) {
                continue;
            }
            let key = projected
                .keys()
                .find(|key| key.eq_ignore_ascii_case(object.cell_view_key()))
                .ok_or_else(|| {
                    format!(
                        "Annotation schematic '{}' is unavailable.",
                        object.cell_view_key()
                    )
                })?;
            let component = projected[key]
                .components
                .iter()
                .find(|component| component.id == object.object_id())
                .ok_or_else(|| {
                    format!(
                        "Annotation object {} no longer exists in '{key}'.",
                        object.object_id()
                    )
                })?;
            if component.name == mapping.new_reference {
                continue;
            }
            if component.name != mapping.old_reference {
                return Err(format!(
                    "Annotation cannot be published because {} in '{key}' changed from '{}'.",
                    component.name, mapping.old_reference
                ));
            }
            names
                .entry(key.clone())
                .or_default()
                .insert(component.id, mapping.new_reference);
        }
        let mut before = BTreeMap::new();
        let mut after = BTreeMap::new();
        for (key, names) in names {
            let source = projected[&key];
            validate_reference_document(self, &key, source)?;
            let mut candidate = source.clone();
            candidate.components = source.prepare_component_renames(&names)?;
            candidate.is_dirty = true;
            candidate.bump_topology_version();
            before.insert(key.clone(), source.clone());
            after.insert(key, candidate);
        }
        self.prepare_schematic_reference_transaction(before, after)
    }

    pub(crate) fn apply_design_management_schematic_transaction(
        &mut self,
        transaction: &SchematicReferenceTransaction,
    ) {
        transaction.prepared_references.clone().publish(self);
        let active_key = self.workspace.active_schematic_reference().key();
        for (key, schematic) in &transaction.after {
            if key.eq_ignore_ascii_case(&active_key) {
                self.schematic = schematic.clone();
                self.workspace
                    .schematic_buffers
                    .insert(active_key.clone(), schematic.clone());
            } else if let Some(existing_key) = self
                .workspace
                .schematic_buffers
                .keys()
                .find(|candidate| candidate.eq_ignore_ascii_case(key))
                .cloned()
            {
                self.workspace
                    .schematic_buffers
                    .insert(existing_key, schematic.clone());
            }
            if let Some(open) = self
                .workspace
                .open_views
                .iter_mut()
                .find(|open| open.reference.key().eq_ignore_ascii_case(key))
            {
                open.dirty = true;
            }
        }
    }
}
