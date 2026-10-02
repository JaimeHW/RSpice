//! Immutable authority shared by mockup-owned schematic edit workflows.
//!
//! A modal may collect options and then hand control to the canvas. Both
//! phases validate this complete snapshot so a command can never commit into
//! a different document, view, grid policy, selection, or geometry revision.

use rspice_schematic_editor::requests::EditorRequestSource;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::state::{SchematicDocumentPolicy, SchematicSnapshot, Selection};

use crate::workbench::app_state::AppState;

#[derive(Debug, Clone)]
pub(crate) struct SchematicEditAuthority {
    pub(crate) source: EditorRequestSource,
    pub(crate) view_path: String,
    pub(crate) grid_size: i32,
    pub(crate) document_policy: SchematicDocumentPolicy,
    pub(crate) snapshot: SchematicSnapshot,
    pub(crate) selection: Selection,
}

impl SchematicEditAuthority {
    pub(crate) fn capture(state: &AppState) -> Self {
        Self {
            source: schematic_editor_request_source(state),
            view_path: state.workspace.content.active_view.display_path(),
            grid_size: state.schematic.document().grid_size,
            document_policy: state.schematic.document().document_policy,
            snapshot: SchematicSnapshot::capture(&state.schematic.document()),
            selection: state.schematic.session.editor.selection.clone(),
        }
    }

    /// Geometry and scope are shared; each workflow validates the library masters it uses.
    pub(crate) fn matches_source(&self, current: &EditorRequestSource) -> bool {
        self.source.project == current.project
            && self.source.document == current.document
            && self.source.occurrence == current.occurrence
            && self.source.design_epoch == current.design_epoch
            && self.source.document_epoch == current.document_epoch
            && self.source.content_version == current.content_version
            && self.source.topology_version == current.topology_version
            && self.source.sheet == current.sheet
    }

    pub(crate) fn validate(&self, state: &AppState, command: &str) -> Result<(), String> {
        if state.schematic_edit_read_only() {
            return Err("The active schematic is read-only.".to_owned());
        }
        self.validate_presentation(state, command)
    }

    /// Validate an operation which changes only runtime presentation state.
    ///
    /// Selection is deliberately available in read-only cell views, but it
    /// must still be bound to the exact document, hierarchy owner, geometry,
    /// and prior selection reviewed by the user.
    pub(crate) fn validate_presentation(
        &self,
        state: &AppState,
        command: &str,
    ) -> Result<(), String> {
        let current = schematic_editor_request_source(state);
        let reopen = |reason: &str| format!("{reason}. Close and reopen {command}.");
        if self.source.design_epoch != current.design_epoch {
            return Err(reopen("The design document changed"));
        }
        if self.source.document_epoch != current.document_epoch {
            return Err(reopen("The active schematic buffer changed"));
        }
        if self.source.topology_version != current.topology_version {
            return Err(reopen("The schematic topology changed"));
        }
        if self.view_path != state.workspace.content.active_view.display_path() {
            return Err(reopen("The active cell/view changed"));
        }
        if !self.matches_source(&current) {
            return Err(reopen("The schematic source or editing scope changed"));
        }
        if self.grid_size != state.schematic.document().grid_size
            || self.document_policy != state.schematic.document().document_policy
        {
            return Err(reopen("The schematic grid or editing policy changed"));
        }
        if self.selection != state.schematic.session.editor.selection {
            return Err(reopen("The selected-object set changed"));
        }
        if !self.snapshot.is_equal_document(&state.schematic.document()) {
            return Err(reopen("The schematic geometry changed"));
        }
        Ok(())
    }
}

/// Source identity shared by editor requests and retained edit workflows.
pub(crate) fn schematic_editor_request_source(state: &AppState) -> EditorRequestSource {
    let document = state.workspace.content.active_schematic_reference();
    let document_key = document.key();
    let sheet = state
        .workspace
        .content
        .design_management
        .sheet_catalog(&document_key)
        .map(|catalog| (catalog.active_sheet_id(), catalog.revision()));
    EditorRequestSource {
        project: state.workspace.content.project.id(),
        document,
        occurrence: state.workspace.content.active_occurrence().cloned(),
        design_epoch: state.design_execution_epoch,
        document_epoch: state.active_schematic_epoch,
        content_version: state.schematic.content_version(),
        topology_version: state.schematic.topology_version(),
        symbol_revision: symbol_context_revision(state),
        sheet,
    }
}

pub(crate) fn symbol_context_revision(state: &AppState) -> u64 {
    let mut hasher = DefaultHasher::new();
    state.library_manager.revision().hash(&mut hasher);
    state
        .workspace
        .content
        .schematic_buffers
        .len()
        .hash(&mut hasher);
    let mut folded_xor = 0_u64;
    let mut folded_sum = 0_u64;
    for (key, buffer) in &state.workspace.content.schematic_buffers {
        let mut entry = DefaultHasher::new();
        key.hash(&mut entry);
        buffer.topology_version().hash(&mut entry);
        let entry = entry.finish();
        folded_xor ^= entry;
        folded_sum = folded_sum.wrapping_add(entry.rotate_left(17));
    }
    folded_xor.hash(&mut hasher);
    folded_sum.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ComponentType, Point};
    use crate::workbench::state::LocalSafeModeOptions;

    #[test]
    fn authority_rejects_another_occurrence_or_sheet_of_the_same_master() {
        use crate::state::{
            CellViewRef, SheetDefinition, SheetPortPolicy, SheetTemplate, ViewType,
        };
        for occurrence_change in [true, false] {
            let mut state = AppState::default();
            let parent = state.workspace.content.active_view.clone();
            let master = CellViewRef::new(&parent.library, "authority_child", "schematic");
            state
                .workspace
                .insert_schematic_editor(master.key(), state.schematic.clone());
            state
                .workspace
                .descend_into("X1".to_owned(), master.clone(), ViewType::Schematic);
            let first = state
                .workspace
                .content
                .design_management
                .bootstrap_for_cell_view(&master.key(), "Sheet 1", [])
                .unwrap();
            let catalog = state
                .workspace
                .content
                .design_management
                .sheet_catalog_mut(&master.key())
                .unwrap();
            let second = catalog
                .create_sheet(
                    SheetDefinition {
                        name: "Sheet 2".to_owned(),
                        template: SheetTemplate::AnalogSchematic,
                        port_policy: SheetPortPolicy::TypedOffSheetPorts,
                        explicit_page_number: Some(2),
                    },
                    Some(first),
                )
                .unwrap();
            catalog.set_active(first).unwrap();
            let authority = SchematicEditAuthority::capture(&state);
            assert!(authority.validate(&state, "Create array").is_ok());

            if occurrence_change {
                state.workspace.ascend_one().unwrap();
                state
                    .workspace
                    .descend_into("X2".to_owned(), master, ViewType::Schematic);
            } else {
                state
                    .workspace
                    .content
                    .design_management
                    .sheet_catalog_mut(&master.key())
                    .unwrap()
                    .set_active(second)
                    .unwrap();
            }
            let current = schematic_editor_request_source(&state);
            assert_eq!(authority.source.document, current.document);
            assert_eq!(authority.source.design_epoch, current.design_epoch);
            assert_eq!(authority.source.document_epoch, current.document_epoch);
            assert_eq!(authority.source.topology_version, current.topology_version);
            assert!(
                authority
                    .snapshot
                    .is_equal_document(state.schematic.document())
            );
            assert!(authority.validate(&state, "Create array").is_err());
            assert!(
                authority
                    .validate_presentation(&state, "Inspect selection")
                    .is_err()
            );
        }
    }

    #[test]
    fn authority_rejects_geometry_selection_and_view_drift() {
        let mut state = AppState::default();
        let id = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(id);
        let authority = SchematicEditAuthority::capture(&state);
        assert!(authority.validate(&state, "Move selection").is_ok());

        state.schematic.document_mut_for_test().components[0].value = "2k".to_owned();
        assert!(authority.validate(&state, "Move selection").is_err());
        state.schematic.document_mut_for_test().components[0].value = "1k".to_owned();
        state.schematic.session.editor.selection.clear();
        assert!(authority.validate(&state, "Move selection").is_err());
    }

    #[test]
    fn authority_revalidates_safe_mode_at_commit_time() {
        let mut state = AppState::default();
        let authority = SchematicEditAuthority::capture(&state);
        state.workbench.safe_mode.activate(
            LocalSafeModeOptions {
                open_project_read_only: true,
                ..Default::default()
            },
            String::new(),
        );

        assert!(authority.validate(&state, "Edit schematic").is_err());
        assert!(
            authority
                .validate_presentation(&state, "Inspect schematic")
                .is_ok(),
            "presentation-only transactions remain available in safe mode"
        );
    }
}
