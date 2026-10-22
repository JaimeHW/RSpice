//! Create hierarchy, undone and redone as one project step.
//!
//! Extraction rewrites the parent schematic, publishes a new library cell and
//! opens its child schematic. Each direction refuses unless the parent, the
//! child, the generated cell and the open tabs are still the ones the step
//! recorded, and unless no other schematic has come to reference the master.

use super::*;

/// Creating a previously absent master can silently resolve an authored
/// dangling instance in any open schematic. That is outside the extraction
/// transaction, so fail before publishing the new cell.
pub(crate) fn validate_hierarchy_target_unreferenced(
    state: &AppState,
    target: &CellViewRef,
) -> Result<(), String> {
    let references_target = |schematic: &rspice_design::schematic::document::SchematicDocument| {
        schematic_references_master(schematic, &target.library, &target.cell)
    };
    if references_target(state.model.schematic.document())
        || state
            .model
            .workspace
            .content
            .schematic_buffers
            .values()
            .any(|schematic| references_target(schematic.document()))
    {
        return Err(format!(
            "Cell '{}/{}' is already referenced by an open schematic and cannot be created implicitly by hierarchy extraction.",
            target.library, target.cell
        ));
    }
    Ok(())
}

impl HierarchyExtractionRecord {
    /// The design content and the tab set decide whether this step still
    /// describes the session — never which tab is in front. Undo brings the
    /// document it restores forward itself, so requiring it to already be
    /// there would refuse the one gesture the operator just made: ascend, then
    /// undo the ascent.
    pub(super) fn after_design_matches(&self, state: &AppState) -> bool {
        schematic_matches(state, &self.parent_ref, &self.after_parent)
            && schematic_matches(state, &self.target_schematic_ref, &self.child)
            && target_cell_matches(state, &self.target_schematic_ref, &self.target_cell)
            && open_documents_match(state, &self.open_views_after)
    }

    pub(super) fn before_design_matches(&self, state: &AppState) -> bool {
        schematic_matches(state, &self.parent_ref, &self.before_parent)
            && state
                .model
                .library_manager
                .get_library(&self.target_schematic_ref.library)
                .and_then(|library| library.get_cell(&self.target_schematic_ref.cell))
                .is_none()
            && !state
                .model
                .workspace
                .content
                .schematic_buffers
                .contains_key(&self.target_schematic_ref.key())
            && open_documents_match(state, &self.open_views_before)
    }

    pub(super) fn validate_mutation(
        &self,
        state: &AppState,
        operation: &str,
    ) -> Result<(), String> {
        if state.model.safe_mode.project_read_only() {
            return Err(format!(
                "Create hierarchy cannot be {operation} while the project is open read-only."
            ));
        }
        for library_name in [&self.parent_ref.library, &self.target_schematic_ref.library] {
            let library = state
                .model
                .library_manager
                .get_library(library_name)
                .ok_or_else(|| format!("Library '{library_name}' no longer exists."))?;
            if library.read_only {
                return Err(format!(
                    "Create hierarchy cannot be {operation} because library '{library_name}' is read-only."
                ));
            }
        }
        if schematic_read_only(state, &self.parent_ref)
            || (self.after_design_matches(state)
                && schematic_read_only(state, &self.target_schematic_ref))
        {
            return Err(format!(
                "Create hierarchy cannot be {operation} because an affected schematic is read-only."
            ));
        }
        if has_external_master_reference(state, self) {
            return Err(format!(
                "Create hierarchy cannot be {operation} because another schematic now references {}/{}.",
                self.target_schematic_ref.library, self.target_schematic_ref.cell
            ));
        }
        Ok(())
    }

    pub(super) fn apply_before(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.after_design_matches(state) {
            return Err("Create hierarchy cannot be undone because its parent, child, or generated symbol changed.".to_owned());
        }
        self.validate_mutation(state, "undone")?;
        self.child_template = schematic_clone(state, &self.target_schematic_ref)
            .ok_or_else(|| "The generated child schematic is no longer open.".to_owned())?;
        self.target_cell = state
            .model
            .library_manager
            .get_library(&self.target_schematic_ref.library)
            .and_then(|library| library.get_cell(&self.target_schematic_ref.cell))
            .cloned()
            .ok_or_else(|| "The generated hierarchy cell no longer exists.".to_owned())?;
        let current_open_views = state.model.workspace.content.open_views.clone();
        let mut library = state
            .model
            .library_manager
            .edit_library(&self.target_schematic_ref.library)
            .ok_or_else(|| "The target library no longer exists.".to_owned())?;
        if !library.remove_cell(&self.target_schematic_ref.cell) {
            return Err("The generated hierarchy cell no longer exists.".to_owned());
        }
        state
            .model
            .workspace
            .remove_schematic_editor(&self.target_schematic_ref.key());
        apply_design_snapshot(state, &self.parent_ref, &self.before_parent)?;
        state.model.workspace.content.open_views = restored_open_views(
            &self.open_views_before,
            &current_open_views,
            [(&self.parent_ref, true)],
        );
        state
            .model
            .workspace
            .content
            .hierarchy_stack
            .clone_from(&self.hierarchy_stack_before);
        state
            .model
            .workspace
            .content
            .hierarchy_instances
            .clone_from(&self.hierarchy_instances_before);
        // The restored tabs carry their own occurrences, so the document this
        // step re-activates is already at the occurrence it was edited at. The
        // snapshot breadcrumb must not re-derive that: it describes where the
        // session was, not which document the record owns.
        state.model.workspace.content.active_view = self.parent_ref.clone();
        state.restore_active_schematic_from_workspace();
        Ok(())
    }

    pub(super) fn apply_after(&self, state: &mut AppState) -> Result<(), String> {
        if !self.before_design_matches(state) {
            return Err(
                "Create hierarchy cannot be redone because the source or destination changed."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "redone")?;
        let current_open_views = state.model.workspace.content.open_views.clone();
        let mut library = state
            .model
            .library_manager
            .edit_library(&self.target_schematic_ref.library)
            .ok_or_else(|| "The target library no longer exists.".to_owned())?;
        library.add_cell(self.target_cell.clone());
        apply_design_snapshot(state, &self.parent_ref, &self.after_parent)?;
        state
            .model
            .workspace
            .insert_schematic_editor(self.target_schematic_ref.key(), self.child_template.clone());
        state.model.workspace.content.open_views = restored_open_views(
            &self.open_views_after,
            &current_open_views,
            [(&self.parent_ref, true), (&self.target_open_ref, true)],
        );
        state
            .model
            .workspace
            .content
            .hierarchy_stack
            .clone_from(&self.hierarchy_stack_after);
        state
            .model
            .workspace
            .content
            .hierarchy_instances
            .clone_from(&self.hierarchy_instances_after);
        state.model.workspace.content.active_view = self.target_open_ref.clone();
        state.restore_active_schematic_from_workspace();
        Ok(())
    }
}

fn schematic_matches(
    state: &AppState,
    reference: &CellViewRef,
    expected: &SchematicSnapshot,
) -> bool {
    if state.model.workspace.content.active_schematic_reference() == *reference {
        expected.is_equal_document(state.model.schematic.document())
    } else {
        state
            .model
            .workspace
            .content
            .schematic_buffers
            .get(&reference.key())
            .is_some_and(|schematic| expected.is_equal_document(schematic.document()))
    }
}

fn apply_design_snapshot(
    state: &mut AppState,
    reference: &CellViewRef,
    snapshot: &SchematicSnapshot,
) -> Result<(), String> {
    if state.model.workspace.content.active_schematic_reference() == *reference {
        state.model.schematic.apply_snapshot(snapshot);
        state
            .model
            .workspace
            .insert_schematic_editor(reference.key(), state.model.schematic.clone());
        return Ok(());
    }
    let mut schematic = state
        .model
        .workspace
        .schematic_editor_mut(&reference.key())
        .ok_or_else(|| {
            format!(
                "Schematic '{}' is no longer open.",
                reference.display_path()
            )
        })?;
    schematic.editor.apply_snapshot(snapshot);
    Ok(())
}

fn schematic_clone(state: &AppState, reference: &CellViewRef) -> Option<SchematicState> {
    if state.model.workspace.content.active_schematic_reference() == *reference {
        Some(state.model.schematic.clone())
    } else {
        state
            .model
            .workspace
            .clone_schematic_editor(&reference.key())
    }
}

fn restored_open_views<const N: usize>(
    expected: &[OpenCellView],
    current: &[OpenCellView],
    dirty_overrides: [(&CellViewRef, bool); N],
) -> Vec<OpenCellView> {
    expected
        .iter()
        .cloned()
        .map(|mut restored| {
            if let Some(actual) = current
                .iter()
                .find(|actual| actual.reference == restored.reference)
            {
                restored.dirty = actual.dirty;
            }
            if let Some((_, dirty)) = dirty_overrides
                .iter()
                .find(|(reference, _)| **reference == restored.reference)
            {
                restored.dirty = *dirty;
            }
            restored
        })
        .collect()
}

/// Whether the open documents are the ones this step was recorded against.
///
/// The breadcrumb is deliberately not part of it. `hierarchy_stack` and
/// `hierarchy_instances` are a runtime projection of the active tab's own
/// occurrence, so comparing them is comparing which tab is in front — and undo
/// brings the tab it restores forward before restoring anything. The step
/// still puts the breadcrumb back; it just does not refuse over it.
fn open_documents_match(state: &AppState, expected_views: &[OpenCellView]) -> bool {
    state.model.workspace.content.open_views.len() == expected_views.len()
        && state
            .model
            .workspace
            .content
            .open_views
            .iter()
            .zip(expected_views)
            .all(|(actual, expected)| {
                actual.reference == expected.reference && actual.view_type == expected.view_type
            })
}

fn has_external_master_reference(state: &AppState, record: &HierarchyExtractionRecord) -> bool {
    let target_library = record.target_schematic_ref.library.as_str();
    let target_cell = record.target_schematic_ref.cell.as_str();
    let is_external =
        |key: &str| key != record.parent_ref.key() && key != record.target_schematic_ref.key();
    let references_target = |schematic: &rspice_design::schematic::document::SchematicDocument| {
        schematic_references_master(schematic, target_library, target_cell)
    };

    state
        .model
        .workspace
        .content
        .schematic_buffers
        .iter()
        .any(|(key, schematic)| is_external(key) && references_target(schematic.document()))
        || (is_external(
            &state
                .model
                .workspace
                .content
                .active_schematic_reference()
                .key(),
        ) && references_target(state.model.schematic.document()))
}

fn schematic_references_master(
    schematic: &rspice_design::schematic::document::SchematicDocument,
    target_library: &str,
    target_cell: &str,
) -> bool {
    schematic.components.iter().any(|component| {
        component.kind == ComponentType::CellInstance
            && component.library_cell.as_ref().is_some_and(|binding| {
                binding.library == target_library && binding.cell == target_cell
            })
    })
}

fn target_cell_matches(state: &AppState, reference: &CellViewRef, expected: &Cell) -> bool {
    let Some(actual) = state
        .model
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
    else {
        return false;
    };
    normalized_cell(actual) == normalized_cell(expected)
}

fn normalized_cell(cell: &Cell) -> Option<serde_json::Value> {
    let mut cell = cell.clone();
    cell.expanded = false;
    for view in cell.views.values_mut() {
        view.modified = false;
        view.is_open = false;
        view.modified_time = None;
        view.file_path = None;
    }
    serde_json::to_value(cell).ok()
}
