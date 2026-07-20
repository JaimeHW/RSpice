//! Cross-document design undo/redo.
//!
//! Ordinary schematic history intentionally owns one buffer. Hierarchy
//! extraction also creates a library cell, a child buffer, a generated symbol,
//! and a navigation transition, so it is retained here as one guarded project
//! transaction. Guards fail closed if either document or the target cell has
//! changed; no snapshot ever overwrites later work.

use crate::state::{
    Cell, CellViewRef, ComponentType, OpenCellView, SchematicSnapshot, SchematicState,
};

use super::AppState;

const MAX_PROJECT_DESIGN_STEPS: usize = 32;

#[derive(Debug, Clone, Default)]
pub(crate) struct ProjectDesignHistory {
    undo: Vec<HierarchyExtractionRecord>,
    redo: Vec<HierarchyExtractionRecord>,
}

#[derive(Debug, Clone)]
struct HierarchyExtractionRecord {
    description: String,
    parent_ref: CellViewRef,
    target_schematic_ref: CellViewRef,
    target_open_ref: CellViewRef,
    before_parent: SchematicSnapshot,
    after_parent: SchematicSnapshot,
    child: SchematicSnapshot,
    child_template: SchematicState,
    target_cell: Cell,
    open_views_before: Vec<OpenCellView>,
    hierarchy_stack_before: Vec<CellViewRef>,
    hierarchy_instances_before: Vec<String>,
    open_views_after: Vec<OpenCellView>,
    hierarchy_stack_after: Vec<CellViewRef>,
    hierarchy_instances_after: Vec<String>,
}

pub(crate) struct HierarchyExtractionHistoryEntry {
    pub(crate) parent_ref: CellViewRef,
    pub(crate) target_schematic_ref: CellViewRef,
    pub(crate) target_open_ref: CellViewRef,
    pub(crate) before_parent: SchematicState,
    pub(crate) after_parent: SchematicState,
    pub(crate) child: SchematicState,
    pub(crate) target_cell: Cell,
    pub(crate) open_views_before: Vec<OpenCellView>,
    pub(crate) hierarchy_stack_before: Vec<CellViewRef>,
    pub(crate) hierarchy_instances_before: Vec<String>,
    pub(crate) open_views_after: Vec<OpenCellView>,
    pub(crate) hierarchy_stack_after: Vec<CellViewRef>,
    pub(crate) hierarchy_instances_after: Vec<String>,
}

impl AppState {
    pub(crate) fn clear_project_design_history(&mut self) {
        self.project_design_history = ProjectDesignHistory::default();
    }

    pub(crate) fn record_hierarchy_extraction(&mut self, entry: HierarchyExtractionHistoryEntry) {
        self.project_design_history
            .undo
            .push(HierarchyExtractionRecord {
                description: "create hierarchical cell".to_owned(),
                parent_ref: entry.parent_ref,
                target_schematic_ref: entry.target_schematic_ref,
                target_open_ref: entry.target_open_ref,
                before_parent: SchematicSnapshot::capture(&entry.before_parent),
                after_parent: SchematicSnapshot::capture(&entry.after_parent),
                child: SchematicSnapshot::capture(&entry.child),
                child_template: entry.child,
                target_cell: entry.target_cell,
                open_views_before: entry.open_views_before,
                hierarchy_stack_before: entry.hierarchy_stack_before,
                hierarchy_instances_before: entry.hierarchy_instances_before,
                open_views_after: entry.open_views_after,
                hierarchy_stack_after: entry.hierarchy_stack_after,
                hierarchy_instances_after: entry.hierarchy_instances_after,
            });
        if self.project_design_history.undo.len() > MAX_PROJECT_DESIGN_STEPS {
            self.project_design_history.undo.remove(0);
        }
        self.project_design_history.redo.clear();
    }

    pub(crate) fn can_undo_project_design(&self) -> bool {
        self.project_design_history
            .undo
            .last()
            .is_some_and(|record| {
                record.after_design_matches(self)
                    && record.validate_mutation(self, "undone").is_ok()
            })
    }

    pub(crate) fn can_redo_project_design(&self) -> bool {
        self.project_design_history
            .redo
            .last()
            .is_some_and(|record| {
                record.before_design_matches(self)
                    && record.validate_mutation(self, "redone").is_ok()
            })
    }

    pub(crate) fn project_undo_owns_active_document(&self) -> bool {
        self.project_design_history
            .undo
            .last()
            .is_some_and(|record| {
                let active = self.workspace.active_schematic_reference();
                active == record.parent_ref || active == record.target_schematic_ref
            })
    }

    pub(crate) fn project_redo_owns_active_document(&self) -> bool {
        self.project_design_history
            .redo
            .last()
            .is_some_and(|record| {
                let active = self.workspace.active_schematic_reference();
                active == record.parent_ref || active == record.target_schematic_ref
            })
    }

    pub(crate) fn undo_project_design(&mut self) -> Result<Option<String>, String> {
        let Some(record) = self.project_design_history.undo.last() else {
            return Ok(None);
        };
        if !record.after_design_matches(self) {
            return Ok(None);
        }
        record.validate_mutation(self, "undone")?;
        let mut record = self
            .project_design_history
            .undo
            .pop()
            .expect("the guarded project transaction remains present");
        record.apply_before(self)?;
        let description = record.description.clone();
        self.project_design_history.redo.push(record);
        Ok(Some(description))
    }

    pub(crate) fn redo_project_design(&mut self) -> Result<Option<String>, String> {
        let Some(record) = self.project_design_history.redo.last() else {
            return Ok(None);
        };
        if !record.before_design_matches(self) {
            return Ok(None);
        }
        record.validate_mutation(self, "redone")?;
        let record = self
            .project_design_history
            .redo
            .pop()
            .expect("the guarded project transaction remains present");
        record.apply_after(self)?;
        let description = record.description.clone();
        self.project_design_history.undo.push(record);
        Ok(Some(description))
    }
}

/// Creating a previously absent master can silently resolve an authored
/// dangling instance in any open schematic. That is outside the extraction
/// transaction, so fail before publishing the new cell.
pub(crate) fn validate_hierarchy_target_unreferenced(
    state: &AppState,
    target: &CellViewRef,
) -> Result<(), String> {
    let references_target = |schematic: &SchematicState| {
        schematic_references_master(schematic, &target.library, &target.cell)
    };
    if references_target(&state.schematic)
        || state
            .workspace
            .schematic_buffers
            .values()
            .any(references_target)
    {
        return Err(format!(
            "Cell '{}/{}' is already referenced by an open schematic and cannot be created implicitly by hierarchy extraction.",
            target.library, target.cell
        ));
    }
    Ok(())
}

impl HierarchyExtractionRecord {
    fn after_design_matches(&self, state: &AppState) -> bool {
        state.workspace.active_view == self.target_open_ref
            && schematic_matches(state, &self.parent_ref, &self.after_parent)
            && schematic_matches(state, &self.target_schematic_ref, &self.child)
            && target_cell_matches(state, &self.target_schematic_ref, &self.target_cell)
            && navigation_matches(
                state,
                &self.open_views_after,
                &self.hierarchy_stack_after,
                &self.hierarchy_instances_after,
            )
    }

    fn before_design_matches(&self, state: &AppState) -> bool {
        state.workspace.active_view == self.parent_ref
            && schematic_matches(state, &self.parent_ref, &self.before_parent)
            && state
                .library_manager
                .get_library(&self.target_schematic_ref.library)
                .and_then(|library| library.get_cell(&self.target_schematic_ref.cell))
                .is_none()
            && !state
                .workspace
                .schematic_buffers
                .contains_key(&self.target_schematic_ref.key())
            && navigation_matches(
                state,
                &self.open_views_before,
                &self.hierarchy_stack_before,
                &self.hierarchy_instances_before,
            )
    }

    fn validate_mutation(&self, state: &AppState, operation: &str) -> Result<(), String> {
        if state.workbench.safe_mode.project_read_only() {
            return Err(format!(
                "Create hierarchy cannot be {operation} while the project is open read-only."
            ));
        }
        for library_name in [&self.parent_ref.library, &self.target_schematic_ref.library] {
            let library = state
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

    fn apply_before(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.after_design_matches(state) {
            return Err("Create hierarchy cannot be undone because its parent, child, or generated symbol changed.".to_owned());
        }
        self.validate_mutation(state, "undone")?;
        self.child_template = schematic_clone(state, &self.target_schematic_ref)
            .ok_or_else(|| "The generated child schematic is no longer open.".to_owned())?;
        self.target_cell = state
            .library_manager
            .get_library(&self.target_schematic_ref.library)
            .and_then(|library| library.get_cell(&self.target_schematic_ref.cell))
            .cloned()
            .ok_or_else(|| "The generated hierarchy cell no longer exists.".to_owned())?;
        let current_open_views = state.workspace.open_views.clone();
        let library = state
            .library_manager
            .get_library_mut(&self.target_schematic_ref.library)
            .ok_or_else(|| "The target library no longer exists.".to_owned())?;
        if !library.remove_cell(&self.target_schematic_ref.cell) {
            return Err("The generated hierarchy cell no longer exists.".to_owned());
        }
        state
            .workspace
            .schematic_buffers
            .remove(&self.target_schematic_ref.key());
        apply_design_snapshot(state, &self.parent_ref, &self.before_parent)?;
        state.workspace.open_views = restored_open_views(
            &self.open_views_before,
            &current_open_views,
            [(&self.parent_ref, true)],
        );
        state
            .workspace
            .hierarchy_stack
            .clone_from(&self.hierarchy_stack_before);
        state
            .workspace
            .hierarchy_instances
            .clone_from(&self.hierarchy_instances_before);
        state.workspace.active_view = self.parent_ref.clone();
        state.restore_active_schematic_from_workspace();
        Ok(())
    }

    fn apply_after(&self, state: &mut AppState) -> Result<(), String> {
        if !self.before_design_matches(state) {
            return Err(
                "Create hierarchy cannot be redone because the source or destination changed."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "redone")?;
        let current_open_views = state.workspace.open_views.clone();
        let library = state
            .library_manager
            .get_library_mut(&self.target_schematic_ref.library)
            .ok_or_else(|| "The target library no longer exists.".to_owned())?;
        library.add_cell(self.target_cell.clone());
        apply_design_snapshot(state, &self.parent_ref, &self.after_parent)?;
        state
            .workspace
            .schematic_buffers
            .insert(self.target_schematic_ref.key(), self.child_template.clone());
        state.workspace.open_views = restored_open_views(
            &self.open_views_after,
            &current_open_views,
            [(&self.parent_ref, true), (&self.target_open_ref, true)],
        );
        state
            .workspace
            .hierarchy_stack
            .clone_from(&self.hierarchy_stack_after);
        state
            .workspace
            .hierarchy_instances
            .clone_from(&self.hierarchy_instances_after);
        state.workspace.active_view = self.target_open_ref.clone();
        state.restore_active_schematic_from_workspace();
        Ok(())
    }
}

fn schematic_matches(
    state: &AppState,
    reference: &CellViewRef,
    expected: &SchematicSnapshot,
) -> bool {
    if state.workspace.active_schematic_reference() == *reference {
        expected.is_equal_state(&state.schematic)
    } else {
        state
            .workspace
            .schematic_buffers
            .get(&reference.key())
            .is_some_and(|schematic| expected.is_equal_state(schematic))
    }
}

fn apply_design_snapshot(
    state: &mut AppState,
    reference: &CellViewRef,
    snapshot: &SchematicSnapshot,
) -> Result<(), String> {
    if state.workspace.active_schematic_reference() == *reference {
        snapshot.apply(&mut state.schematic);
        state
            .workspace
            .schematic_buffers
            .insert(reference.key(), state.schematic.clone());
        return Ok(());
    }
    let schematic = state
        .workspace
        .schematic_buffers
        .get_mut(&reference.key())
        .ok_or_else(|| {
            format!(
                "Schematic '{}' is no longer open.",
                reference.display_path()
            )
        })?;
    snapshot.apply(schematic);
    Ok(())
}

fn schematic_read_only(state: &AppState, reference: &CellViewRef) -> bool {
    if state.workspace.active_schematic_reference() == *reference {
        state.schematic.read_only
    } else {
        state
            .workspace
            .schematic_buffers
            .get(&reference.key())
            .is_some_and(|schematic| schematic.read_only)
    }
}

fn schematic_clone(state: &AppState, reference: &CellViewRef) -> Option<SchematicState> {
    if state.workspace.active_schematic_reference() == *reference {
        Some(state.schematic.clone())
    } else {
        state
            .workspace
            .schematic_buffers
            .get(&reference.key())
            .cloned()
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

fn navigation_matches(
    state: &AppState,
    expected_views: &[OpenCellView],
    expected_stack: &[CellViewRef],
    expected_instances: &[String],
) -> bool {
    state.workspace.open_views.len() == expected_views.len()
        && state
            .workspace
            .open_views
            .iter()
            .zip(expected_views)
            .all(|(actual, expected)| {
                actual.reference == expected.reference && actual.view_type == expected.view_type
            })
        && state.workspace.hierarchy_stack == expected_stack
        && state.workspace.hierarchy_instances == expected_instances
}

fn has_external_master_reference(state: &AppState, record: &HierarchyExtractionRecord) -> bool {
    let target_library = record.target_schematic_ref.library.as_str();
    let target_cell = record.target_schematic_ref.cell.as_str();
    let is_external =
        |key: &str| key != record.parent_ref.key() && key != record.target_schematic_ref.key();
    let references_target = |schematic: &SchematicState| {
        schematic_references_master(schematic, target_library, target_cell)
    };

    state
        .workspace
        .schematic_buffers
        .iter()
        .any(|(key, schematic)| is_external(key) && references_target(schematic))
        || (is_external(&state.workspace.active_schematic_reference().key())
            && references_target(&state.schematic))
}

fn schematic_references_master(
    schematic: &SchematicState,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Cell, ComponentType, LibraryCellInstance, Point, View, ViewType};
    use crate::workbench::state::LocalSafeModeOptions;

    fn state_with_hierarchy_record() -> (AppState, CellViewRef, CellViewRef) {
        let mut state = AppState::default();
        let parent_ref = state.workspace.active_view.clone();
        let before_parent = state.schematic.clone();
        let mut after_parent = before_parent.clone();
        after_parent.add_component(ComponentType::Resistor, Point::origin());
        let target = CellViewRef::new(&parent_ref.library, "child", "schematic");
        let mut child = SchematicState::default();
        child.add_component(ComponentType::Capacitor, Point::origin());
        let mut cell = Cell::new("child");
        cell.add_view(View::new("schematic", ViewType::Schematic));

        state
            .library_manager
            .get_library_mut(&target.library)
            .expect("target library")
            .add_cell(cell.clone());
        state
            .workspace
            .schematic_buffers
            .insert(parent_ref.key(), after_parent.clone());
        state
            .workspace
            .schematic_buffers
            .insert(target.key(), child.clone());
        state.schematic = child.clone();
        state.workspace.active_view = target.clone();
        let open_views_after = state.workspace.open_views.clone();
        let hierarchy_stack_after = state.workspace.hierarchy_stack.clone();
        let hierarchy_instances_after = state.workspace.hierarchy_instances.clone();
        state.record_hierarchy_extraction(HierarchyExtractionHistoryEntry {
            parent_ref: parent_ref.clone(),
            target_schematic_ref: target.clone(),
            target_open_ref: target.clone(),
            before_parent,
            after_parent,
            child,
            target_cell: cell,
            open_views_before: state.workspace.open_views.clone(),
            hierarchy_stack_before: state.workspace.hierarchy_stack.clone(),
            hierarchy_instances_before: state.workspace.hierarchy_instances.clone(),
            open_views_after,
            hierarchy_stack_after,
            hierarchy_instances_after,
        });
        (state, parent_ref, target)
    }

    #[test]
    fn guarded_history_refuses_to_overwrite_a_modified_child() {
        let (mut state, _, _) = state_with_hierarchy_record();
        state.schematic.components[0].value = "changed".to_owned();
        assert!(!state.can_undo_project_design());
        assert_eq!(state.undo_project_design().expect("guarded"), None);
    }

    #[test]
    fn hierarchy_history_refuses_read_only_library_without_mutation() {
        let (mut state, _, target) = state_with_hierarchy_record();
        state
            .library_manager
            .get_library_mut(&target.library)
            .expect("target library")
            .read_only = true;

        assert!(!state.can_undo_project_design());
        assert!(state.undo_project_design().is_err());
        assert!(
            state
                .library_manager
                .get_library(&target.library)
                .and_then(|library| library.get_cell(&target.cell))
                .is_some()
        );
    }

    #[test]
    fn hierarchy_history_refuses_project_read_only_safe_mode() {
        let (mut state, _, target) = state_with_hierarchy_record();
        state.workbench.safe_mode.activate(
            LocalSafeModeOptions {
                open_project_read_only: true,
                ..LocalSafeModeOptions::default()
            },
            "retained session".to_owned(),
        );

        assert!(!state.can_undo_project_design());
        assert!(state.undo_project_design().is_err());
        assert!(
            state
                .library_manager
                .get_library(&target.library)
                .and_then(|library| library.get_cell(&target.cell))
                .is_some()
        );
    }

    #[test]
    fn hierarchy_history_refuses_dangling_external_master_reference() {
        let (mut state, _, target) = state_with_hierarchy_record();
        let unrelated = CellViewRef::new("work", "other", "schematic");
        let mut schematic = SchematicState::default();
        let id = schematic.add_component(ComponentType::CellInstance, Point::origin());
        let component = schematic
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("instance");
        component.library_cell = Some(LibraryCellInstance::new(
            &target.library,
            &target.cell,
            "symbol",
        ));
        state
            .workspace
            .schematic_buffers
            .insert(unrelated.key(), schematic);

        assert!(!state.can_undo_project_design());
        assert!(state.undo_project_design().is_err());
        assert!(
            state
                .library_manager
                .get_library(&target.library)
                .and_then(|library| library.get_cell(&target.cell))
                .is_some()
        );
    }

    #[test]
    fn new_hierarchy_target_refuses_to_resolve_an_existing_dangling_instance() {
        let mut state = AppState::default();
        let target = CellViewRef::new("work", "child", "schematic");
        let id = state
            .schematic
            .add_component(ComponentType::CellInstance, Point::origin());
        state
            .schematic
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("instance")
            .library_cell = Some(LibraryCellInstance::new("work", "child", "symbol"));

        assert!(validate_hierarchy_target_unreferenced(&state, &target).is_err());
        assert!(
            state
                .library_manager
                .get_library("work")
                .and_then(|library| library.get_cell("child"))
                .is_none()
        );
    }

    #[test]
    fn project_undo_and_redo_preserve_parent_view_runtime() {
        let (mut state, parent, _) = state_with_hierarchy_record();
        let parent_buffer = state
            .workspace
            .schematic_buffers
            .get_mut(&parent.key())
            .expect("parent");
        parent_buffer.zoom = 2.75;
        parent_buffer.pan = (140.0, -35.0);

        assert!(state.undo_project_design().expect("undo").is_some());
        assert_eq!(state.schematic.zoom, 2.75);
        assert_eq!(state.schematic.pan, (140.0, -35.0));

        state.schematic.zoom = 1.5;
        state.schematic.pan = (-20.0, 85.0);
        state.sync_active_schematic_to_workspace();
        assert!(state.redo_project_design().expect("redo").is_some());
        let parent_buffer = state
            .workspace
            .schematic_buffers
            .get(&parent.key())
            .expect("parent");
        assert_eq!(parent_buffer.zoom, 1.5);
        assert_eq!(parent_buffer.pan, (-20.0, 85.0));
    }

    #[test]
    fn project_undo_and_redo_preserve_child_view_runtime() {
        let (mut state, _, _) = state_with_hierarchy_record();
        state.schematic.zoom = 3.25;
        state.schematic.pan = (75.0, -120.0);

        assert!(state.undo_project_design().expect("undo").is_some());
        assert!(state.redo_project_design().expect("redo").is_some());
        assert_eq!(state.schematic.zoom, 3.25);
        assert_eq!(state.schematic.pan, (75.0, -120.0));
    }

    #[test]
    fn project_history_preserves_unrelated_tab_dirty_state() {
        let (mut state, _, _) = state_with_hierarchy_record();
        let unrelated = OpenCellView::new(
            CellViewRef::new("work", "other", "schematic"),
            ViewType::Schematic,
        );
        state.workspace.open_views.push(unrelated.clone());
        let record = state
            .project_design_history
            .undo
            .last_mut()
            .expect("record");
        record.open_views_before.push(unrelated.clone());
        record.open_views_after.push(unrelated);
        state
            .workspace
            .open_views
            .last_mut()
            .expect("unrelated tab")
            .dirty = true;

        assert!(state.undo_project_design().expect("undo").is_some());
        assert!(
            state
                .workspace
                .open_views
                .iter()
                .find(|view| view.reference.cell == "other")
                .expect("restored unrelated tab")
                .dirty
        );
    }

    #[test]
    fn project_history_refuses_to_hijack_changed_active_focus() {
        let (mut state, parent, target) = state_with_hierarchy_record();
        state.workspace.active_view = parent;

        assert!(!state.can_undo_project_design());
        assert_eq!(state.undo_project_design().expect("guarded"), None);
        assert!(
            state
                .library_manager
                .get_library(&target.library)
                .and_then(|library| library.get_cell(&target.cell))
                .is_some()
        );
    }
}
