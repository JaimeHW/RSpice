use super::{ConsoleMessage, RSpiceApp, VERILOGA_LIBRARY_NAME};

impl RSpiceApp {
    pub(in crate::common::app) fn process_pending_library_deletions(&mut self) {
        if let Some((lib_name, cell_name)) = self.state.pending_delete_cell.take() {
            let mut deleted = false;
            if let Some(lib) = self.state.library_manager.get_library_mut(&lib_name) {
                deleted = lib.remove_cell(&cell_name);
                if deleted {
                    self.state
                        .prune_workspace_after_cell_deleted(&lib_name, &cell_name);
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Deleted cell '{}' from library '{}'",
                        cell_name, lib_name
                    )));
                }
            }
            if deleted && lib_name == VERILOGA_LIBRARY_NAME {
                self.persist_global_veriloga_library_with_feedback();
            }
        }

        if let Some((lib_name, cell_name, view_name)) = self.state.pending_delete_view.take() {
            let mut deleted = false;
            if let Some(lib) = self.state.library_manager.get_library_mut(&lib_name)
                && let Some(cell) = lib.get_cell_mut(&cell_name)
            {
                deleted = cell.remove_view(&view_name);
                if deleted {
                    self.state
                        .prune_workspace_after_view_deleted(&lib_name, &cell_name, &view_name);
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Deleted view '{}' from cell '{}'",
                        view_name, cell_name
                    )));
                }
            }
            if deleted && lib_name == VERILOGA_LIBRARY_NAME {
                self.persist_global_veriloga_library_with_feedback();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Cell, CellViewRef, ComponentType, Library, LibraryCellInstance, OpenCellView, Point,
        SchematicState, View, ViewType,
    };

    fn app_with_state(state: crate::common::app::AppState) -> RSpiceApp {
        RSpiceApp {
            state,
            first_frame: false,
            #[cfg(not(target_arch = "wasm32"))]
            autosave_last: None,
            applied_theme: None,
            last_window_title: String::new(),
            symbol_library: None,
            simulation_controller: crate::simulation::SimulationController::new(),
            file_workflow_io: Box::new(crate::common::file_workflow::NativeFileWorkflowIo),
            export_workflow_io: Box::new(crate::common::export_workflow::NativeExportWorkflowIo),
        }
    }

    fn state_with_open_amp_cell() -> crate::common::app::AppState {
        let mut state = crate::common::app::AppState::default();
        let mut library = Library::new("work");
        let mut amp = Cell::new("amp");
        amp.add_view(View::new("schematic", ViewType::Schematic));
        amp.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(amp);
        let mut keep = Cell::new("keep");
        keep.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(keep);
        state.library_manager.add_library(library);

        let amp_ref = CellViewRef::new("work", "amp", "schematic");
        let keep_ref = CellViewRef::new("work", "keep", "schematic");
        let mut amp_schematic = SchematicState::default();
        amp_schematic.add_component(ComponentType::Resistor, Point::new(10, 10));
        state
            .workspace
            .schematic_buffers
            .insert(amp_ref.key(), amp_schematic.clone());
        state
            .workspace
            .schematic_buffers
            .insert(keep_ref.key(), SchematicState::default());
        state.workspace.open_views = vec![
            OpenCellView::new(keep_ref.clone(), ViewType::Schematic),
            OpenCellView::new(amp_ref.clone(), ViewType::Schematic),
        ];
        state.workspace.active_view = amp_ref.clone();
        state.workspace.hierarchy_stack = vec![keep_ref, amp_ref.clone()];
        state.workspace.hierarchy_instances = vec!["XAMP".to_string()];
        state.schematic = amp_schematic;
        state
    }

    fn state_with_active_leaf_under_amp() -> crate::common::app::AppState {
        let mut state = crate::common::app::AppState::default();
        let mut library = Library::new("work");
        for name in ["top", "amp", "leaf"] {
            let mut cell = Cell::new(name);
            cell.add_view(View::new("schematic", ViewType::Schematic));
            library.add_cell(cell);
        }
        state.library_manager.add_library(library);

        let top = CellViewRef::new("work", "top", "schematic");
        let amp = CellViewRef::new("work", "amp", "schematic");
        let leaf = CellViewRef::new("work", "leaf", "schematic");
        state
            .workspace
            .schematic_buffers
            .insert(top.key(), SchematicState::default());
        state
            .workspace
            .schematic_buffers
            .insert(amp.key(), SchematicState::default());
        state
            .workspace
            .schematic_buffers
            .insert(leaf.key(), SchematicState::default());
        state.workspace.open_views = vec![
            OpenCellView::new(top.clone(), ViewType::Schematic),
            OpenCellView::new(amp.clone(), ViewType::Schematic),
            OpenCellView::new(leaf.clone(), ViewType::Schematic),
        ];
        state.workspace.active_view = leaf.clone();
        state.workspace.hierarchy_stack = vec![top, amp, leaf.clone()];
        state.workspace.hierarchy_instances = vec!["XAMP".to_string(), "XLEAF".to_string()];
        state.schematic = SchematicState::default();
        state
    }

    fn default_project_with_active_keep_cell() -> crate::common::app::AppState {
        let mut state = crate::common::app::AppState::default();
        let keep_ref = CellViewRef::new("user", "keep", "schematic");
        if let Some(library) = state.library_manager.get_library_mut("user") {
            let mut keep = Cell::new("keep");
            keep.add_view(View::new("schematic", ViewType::Schematic));
            library.add_cell(keep);
        }
        state
            .workspace
            .schematic_buffers
            .insert(keep_ref.key(), SchematicState::default());
        state.workspace.open_views = vec![
            OpenCellView::new(
                CellViewRef::new("user", "top", "schematic"),
                ViewType::Schematic,
            ),
            OpenCellView::new(keep_ref.clone(), ViewType::Schematic),
        ];
        state.workspace.active_view = keep_ref.clone();
        state.workspace.hierarchy_stack = vec![keep_ref];
        state.workspace.hierarchy_instances.clear();
        state.schematic = SchematicState::default();
        state
    }

    fn default_project_with_active_top_instancing_amp() -> crate::common::app::AppState {
        let mut state = crate::common::app::AppState::default();
        let amp_ref = CellViewRef::new("user", "amp", "schematic");
        let top_ref = CellViewRef::new("user", "top", "schematic");

        if let Some(library) = state.library_manager.get_library_mut("user") {
            let mut amp = Cell::new("amp");
            amp.add_view(View::new("schematic", ViewType::Schematic));
            library.add_cell(amp);
        }

        let mut top = SchematicState::default();
        top.add_library_cell_component(
            Point::new(10, 10),
            LibraryCellInstance::new("user", "amp", "schematic"),
        );

        state
            .workspace
            .schematic_buffers
            .insert(top_ref.key(), top.clone());
        state
            .workspace
            .schematic_buffers
            .insert(amp_ref.key(), SchematicState::default());
        state.workspace.open_views = vec![OpenCellView::new(top_ref.clone(), ViewType::Schematic)];
        state.workspace.active_view = top_ref.clone();
        state.workspace.hierarchy_stack = vec![top_ref];
        state.workspace.hierarchy_instances.clear();
        state.schematic = top;
        state
    }

    #[test]
    fn deleting_open_cell_prunes_workspace_references_and_restores_valid_focus() {
        let mut app = app_with_state(state_with_open_amp_cell());
        app.state.pending_delete_cell = Some(("work".to_string(), "amp".to_string()));

        app.process_pending_library_deletions();

        assert!(
            app.state
                .library_manager
                .get_library("work")
                .and_then(|library| library.get_cell("amp"))
                .is_none()
        );
        assert!(
            app.state
                .workspace
                .open_views
                .iter()
                .all(|open| { open.reference.library != "work" || open.reference.cell != "amp" })
        );
        assert!(
            app.state
                .workspace
                .hierarchy_stack
                .iter()
                .all(|reference| reference.library != "work" || reference.cell != "amp")
        );
        assert!(
            !app.state
                .workspace
                .schematic_buffers
                .contains_key("work/amp/schematic")
        );
        assert_ne!(
            app.state.workspace.active_view,
            CellViewRef::new("work", "amp", "schematic")
        );
        assert!(app.state.workspace.active_context_schematic().is_some());
    }

    #[test]
    fn deleting_active_view_prunes_workspace_references_and_falls_back_to_schematic() {
        let mut state = state_with_open_amp_cell();
        let symbol_ref = CellViewRef::new("work", "amp", "symbol");
        state
            .workspace
            .open_views
            .push(OpenCellView::new(symbol_ref.clone(), ViewType::Symbol));
        state.workspace.active_view = symbol_ref.clone();
        state.workspace.hierarchy_stack = vec![symbol_ref.clone()];
        state.workspace.hierarchy_instances.clear();
        let mut app = app_with_state(state);
        app.state.pending_delete_view =
            Some(("work".to_string(), "amp".to_string(), "symbol".to_string()));

        app.process_pending_library_deletions();

        assert!(
            app.state
                .library_manager
                .get_library("work")
                .and_then(|library| library.get_cell("amp"))
                .and_then(|cell| cell.get_view("symbol"))
                .is_none()
        );
        assert!(
            app.state
                .workspace
                .open_views
                .iter()
                .all(|open| open.reference != symbol_ref)
        );
        assert!(
            !app.state
                .workspace
                .hierarchy_stack
                .iter()
                .any(|reference| reference == &symbol_ref)
        );
        assert_eq!(
            app.state.workspace.active_view,
            CellViewRef::new("work", "amp", "schematic")
        );
        assert!(app.state.workspace.active_context_schematic().is_some());
    }

    #[test]
    fn deleting_hierarchy_ancestor_resets_invalid_breadcrumb_path() {
        let leaf = CellViewRef::new("work", "leaf", "schematic");
        let mut app = app_with_state(state_with_active_leaf_under_amp());
        app.state.pending_delete_cell = Some(("work".to_string(), "amp".to_string()));

        app.process_pending_library_deletions();

        assert_eq!(app.state.workspace.active_view, leaf);
        assert_eq!(app.state.workspace.hierarchy_stack, vec![leaf]);
        assert!(app.state.workspace.hierarchy_instances.is_empty());
    }

    #[test]
    fn deleting_default_top_cell_does_not_recreate_deleted_design() {
        let mut app = app_with_state(crate::common::app::AppState::default());
        app.state.pending_delete_cell = Some(("user".to_string(), "top".to_string()));

        app.process_pending_library_deletions();

        assert!(
            app.state
                .library_manager
                .get_library("user")
                .and_then(|library| library.get_cell("top"))
                .is_none(),
            "deleted default top cell should not be recreated as fallback"
        );
        assert_ne!(
            app.state.workspace.active_view,
            CellViewRef::new("user", "top", "schematic")
        );
        assert!(app.state.workspace.active_context_schematic().is_some());
    }

    #[test]
    fn deleting_default_top_schematic_view_does_not_recreate_deleted_view() {
        let mut app = app_with_state(crate::common::app::AppState::default());
        app.state.pending_delete_view = Some((
            "user".to_string(),
            "top".to_string(),
            "schematic".to_string(),
        ));

        app.process_pending_library_deletions();

        assert!(
            app.state
                .library_manager
                .get_library("user")
                .and_then(|library| library.get_cell("top"))
                .and_then(|cell| cell.get_view("schematic"))
                .is_none(),
            "deleted default top schematic view should not be recreated as fallback"
        );
        assert_ne!(
            app.state.workspace.active_view,
            CellViewRef::new("user", "top", "schematic")
        );
        assert!(app.state.workspace.active_context_schematic().is_some());
    }

    #[test]
    fn deleting_non_active_project_top_cell_repoints_root_and_invalidates_runs() {
        let mut app = app_with_state(default_project_with_active_keep_cell());
        let original_epoch = app.state.design_execution_epoch;
        app.state.pending_delete_cell = Some(("user".to_string(), "top".to_string()));

        app.process_pending_library_deletions();

        assert!(
            app.state
                .library_manager
                .get_library("user")
                .and_then(|library| library.get_cell("top"))
                .is_none(),
            "deleted project top cell should stay deleted"
        );
        assert_eq!(app.state.workspace.active_view.cell, "keep");
        assert_eq!(app.state.workspace.project.root_library, "user");
        assert_eq!(app.state.workspace.project.top_cell, "keep");
        assert_ne!(app.state.design_execution_epoch, original_epoch);
    }

    #[test]
    fn deleting_non_active_project_top_schematic_repoints_root_and_invalidates_runs() {
        let mut app = app_with_state(default_project_with_active_keep_cell());
        let original_epoch = app.state.design_execution_epoch;
        app.state.pending_delete_view = Some((
            "user".to_string(),
            "top".to_string(),
            "schematic".to_string(),
        ));

        app.process_pending_library_deletions();

        assert!(
            app.state
                .library_manager
                .get_library("user")
                .and_then(|library| library.get_cell("top"))
                .and_then(|cell| cell.get_view("schematic"))
                .is_none(),
            "deleted project top schematic should stay deleted"
        );
        assert_eq!(app.state.workspace.active_view.cell, "keep");
        assert_eq!(app.state.workspace.project.root_library, "user");
        assert_eq!(app.state.workspace.project.top_cell, "keep");
        assert_ne!(app.state.design_execution_epoch, original_epoch);
    }

    #[test]
    fn deleting_instanced_non_active_cell_invalidates_runs() {
        let mut app = app_with_state(default_project_with_active_top_instancing_amp());
        let original_epoch = app.state.design_execution_epoch;
        app.state.pending_delete_cell = Some(("user".to_string(), "amp".to_string()));

        app.process_pending_library_deletions();

        assert_eq!(app.state.workspace.active_view.cell, "top");
        assert_eq!(app.state.workspace.project.root_library, "user");
        assert_eq!(app.state.workspace.project.top_cell, "top");
        assert_ne!(app.state.design_execution_epoch, original_epoch);
    }

    #[test]
    fn deleting_instanced_non_active_schematic_invalidates_runs() {
        let mut app = app_with_state(default_project_with_active_top_instancing_amp());
        let original_epoch = app.state.design_execution_epoch;
        app.state.pending_delete_view = Some((
            "user".to_string(),
            "amp".to_string(),
            "schematic".to_string(),
        ));

        app.process_pending_library_deletions();

        assert_eq!(app.state.workspace.active_view.cell, "top");
        assert_eq!(app.state.workspace.project.root_library, "user");
        assert_eq!(app.state.workspace.project.top_cell, "top");
        assert_ne!(app.state.design_execution_epoch, original_epoch);
    }
}
