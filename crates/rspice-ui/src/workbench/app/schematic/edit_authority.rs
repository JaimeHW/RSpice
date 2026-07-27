//! Immutable authority shared by mockup-owned schematic edit workflows.
//!
//! A modal may collect options and then hand control to the canvas. Both
//! phases validate this complete snapshot so a command can never commit into
//! a different document, view, grid policy, selection, or geometry revision.

use crate::state::{SchematicDocumentPolicy, SchematicSnapshot, Selection};

use crate::workbench::app::AppState;

#[derive(Debug, Clone)]
pub(crate) struct SchematicEditAuthority {
    pub(crate) design_execution_epoch: u64,
    pub(crate) active_schematic_epoch: u64,
    pub(crate) topology_version: u64,
    pub(crate) view_path: String,
    pub(crate) grid_size: i32,
    pub(crate) document_policy: SchematicDocumentPolicy,
    pub(crate) snapshot: SchematicSnapshot,
    pub(crate) selection: Selection,
}

impl SchematicEditAuthority {
    pub(crate) fn capture(state: &AppState) -> Self {
        Self {
            design_execution_epoch: state.design_execution_epoch,
            active_schematic_epoch: state.active_schematic_epoch,
            topology_version: state.schematic.topology_version(),
            view_path: state.workspace.active_view.display_path(),
            grid_size: state.schematic.grid_size,
            document_policy: state.schematic.document_policy,
            snapshot: SchematicSnapshot::capture(&state.schematic),
            selection: state.schematic.selection.clone(),
        }
    }

    pub(crate) fn validate(&self, state: &AppState, command: &str) -> Result<(), String> {
        if state.schematic.read_only || state.active_view_read_only() {
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
        let reopen = |reason: &str| format!("{reason}. Close and reopen {command}.");
        if self.design_execution_epoch != state.design_execution_epoch {
            return Err(reopen("The design document changed"));
        }
        if self.active_schematic_epoch != state.active_schematic_epoch {
            return Err(reopen("The active schematic buffer changed"));
        }
        if self.topology_version != state.schematic.topology_version() {
            return Err(reopen("The schematic topology changed"));
        }
        if self.view_path != state.workspace.active_view.display_path() {
            return Err(reopen("The active cell/view changed"));
        }
        if self.grid_size != state.schematic.grid_size
            || self.document_policy != state.schematic.document_policy
        {
            return Err(reopen("The schematic grid or editing policy changed"));
        }
        if self.selection != state.schematic.selection {
            return Err(reopen("The selected-object set changed"));
        }
        if !self.snapshot.is_equal_state(&state.schematic) {
            return Err(reopen("The schematic geometry changed"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ComponentType, Point};

    #[test]
    fn authority_rejects_geometry_selection_and_view_drift() {
        let mut state = AppState::default();
        let id = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        state.schematic.selection.select_only_component(id);
        let authority = SchematicEditAuthority::capture(&state);
        assert!(authority.validate(&state, "Move selection").is_ok());

        state.schematic.components[0].value = "2k".to_owned();
        assert!(authority.validate(&state, "Move selection").is_err());
        state.schematic.components[0].value = "1k".to_owned();
        state.schematic.selection.clear();
        assert!(authority.validate(&state, "Move selection").is_err());
    }
}
