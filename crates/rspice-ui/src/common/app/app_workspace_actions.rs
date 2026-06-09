use crate::common::app::{AppState, ConsoleMessage, RSpiceApp};
use crate::state::{CellViewRef, ComponentType, SchematicState, ViewType};

fn view_type_for_reference(state: &AppState, reference: &CellViewRef) -> ViewType {
    state
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .map(|view| view.view_type)
        .unwrap_or(ViewType::Schematic)
}

fn schematic_for_workspace(state: &mut AppState, reference: &CellViewRef) -> SchematicState {
    state
        .workspace
        .schematic_buffers
        .get(&reference.key())
        .cloned()
        .unwrap_or_default()
}

impl AppState {
    pub(crate) fn sync_active_schematic_to_workspace(&mut self) {
        self.workspace.save_active_schematic(&self.schematic);
    }

    pub(crate) fn restore_active_schematic_from_workspace(&mut self) {
        self.workspace
            .ensure_library_model(&mut self.library_manager);
        let reference = self.workspace.active_view.clone();
        self.schematic = schematic_for_workspace(self, &reference);
        self.library_manager
            .select_view(&reference.library, &reference.cell, &reference.view);
    }

    pub(crate) fn open_workspace_view(&mut self, reference: CellViewRef) {
        self.sync_active_schematic_to_workspace();
        if self.workspace.active_view == reference {
            return;
        }
        let view_type = view_type_for_reference(self, &reference);
        self.workspace.open_as_root(reference.clone(), view_type);
        self.library_manager
            .select_view(&reference.library, &reference.cell, &reference.view);
        if matches!(view_type, ViewType::Schematic | ViewType::Testbench) {
            self.schematic = schematic_for_workspace(self, &reference);
        }
        self.push_user_message(ConsoleMessage::info(format!(
            "Opened {}",
            reference.display_path()
        )));
    }

    pub(crate) fn enter_workspace_view(&mut self, reference: CellViewRef) {
        self.sync_active_schematic_to_workspace();
        let view_type = view_type_for_reference(self, &reference);
        self.workspace.enter_hierarchy(reference.clone(), view_type);
        self.library_manager
            .select_view(&reference.library, &reference.cell, &reference.view);
        if matches!(view_type, ViewType::Schematic | ViewType::Testbench) {
            self.schematic = schematic_for_workspace(self, &reference);
        }
        self.push_user_message(ConsoleMessage::info(format!(
            "Entered {}",
            reference.display_path()
        )));
    }

    pub(crate) fn focus_workspace_breadcrumb(&mut self, index: usize) {
        self.sync_active_schematic_to_workspace();
        if let Some(reference) = self.workspace.focus_breadcrumb(index) {
            self.library_manager
                .select_view(&reference.library, &reference.cell, &reference.view);
            self.schematic = schematic_for_workspace(self, &reference);
        }
    }

    pub(crate) fn open_selected_instance_master(&mut self) {
        let Some(component_id) = self.schematic.selection.single_component() else {
            self.push_user_message(ConsoleMessage::warning(
                "Select one hierarchical instance first",
            ));
            return;
        };

        let Some(component) = self
            .schematic
            .components
            .iter()
            .find(|component| component.id == component_id)
        else {
            return;
        };

        if component.kind != ComponentType::CellInstance {
            self.push_user_message(ConsoleMessage::warning(
                "Selected component is not a hierarchical instance",
            ));
            return;
        }

        let Some(binding) = component.library_cell.as_ref() else {
            self.push_user_message(ConsoleMessage::warning(
                "Selected instance has no Library/Cell/View binding",
            ));
            return;
        };

        self.enter_workspace_view(CellViewRef::new(
            binding.library.clone(),
            binding.cell.clone(),
            binding.view.clone(),
        ));
    }
}

impl RSpiceApp {
    pub(crate) fn restore_workspace_after_project_load(&mut self) {
        self.state.restore_active_schematic_from_workspace();
        self.state.clear_transient_specialized_viewer_data();
    }
}
