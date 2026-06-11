use std::path::Path;

use crate::common::app::{AppState, ConsoleMessage};
use crate::io::{ProjectFile, ProjectIoError};

pub(crate) fn create_new_project(state: &mut AppState) {
    let mut library_manager = crate::state::LibraryManager::with_primitives();
    let mut workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut library_manager);
    let schematic = workspace
        .active_schematic()
        .cloned()
        .unwrap_or_else(crate::state::SchematicState::default);
    workspace.save_active_schematic(&schematic);

    state.library_manager = library_manager;
    state.workspace = workspace;
    state.schematic = schematic;
    state.simulation.clear_runs();
    state.push_user_message(ConsoleMessage::info("Created new project"));
}

pub(crate) fn save_project_to_path(state: &mut AppState, path: &Path) -> bool {
    state.sync_active_schematic_to_workspace();
    let mut workspace = state.workspace.clone();
    workspace.project.set_path(path.to_path_buf());

    let file = ProjectFile::new(workspace.clone(), state.library_manager.clone());
    match crate::io::save_project_file(&file, path) {
        Ok(()) => {
            state.workspace = workspace;
            state.workspace.mark_all_clean();
            state.schematic.is_dirty = false;
            state.remember_recent_file(crate::common::app::RecentKind::Project, path);
            state.push_user_message(ConsoleMessage::info(format!(
                "Saved project: {}",
                path.display()
            )));
            true
        }
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Project save failed: {}",
                error
            )));
            false
        }
    }
}

pub(crate) fn save_project(state: &mut AppState) -> bool {
    if let Some(path) = state.workspace.project.path.clone() {
        save_project_to_path(state, &path)
    } else {
        save_project_as(state)
    }
}

pub(crate) fn save_project_as(state: &mut AppState) -> bool {
    let default_name = state
        .workspace
        .project
        .path
        .as_ref()
        .and_then(|path| path.file_name())
        .map(|name| name.to_string_lossy().to_string())
        .or_else(|| Some("untitled.rspiceproj".to_string()));

    match crate::io::show_save_project_dialog(default_name.as_deref()) {
        Ok(path) => save_project_to_path(state, &path),
        Err(ProjectIoError::Cancelled) => false,
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Project Save As failed: {}",
                error
            )));
            false
        }
    }
}

pub(crate) fn load_project_from_path(state: &mut AppState, path: &Path) -> bool {
    match crate::io::load_project_file(path) {
        Ok(mut project) => {
            project
                .workspace
                .ensure_library_model(&mut project.libraries);
            state.library_manager = project.libraries;
            state.workspace = project.workspace;
            state.restore_active_schematic_from_workspace();
            state.simulation.clear_runs();
            state.remember_recent_file(crate::common::app::RecentKind::Project, path);
            state.push_user_message(ConsoleMessage::info(format!(
                "Opened project: {}",
                path.display()
            )));
            true
        }
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Project open failed: {}",
                error
            )));
            false
        }
    }
}

pub(crate) fn open_project(state: &mut AppState) -> bool {
    match crate::io::show_open_project_dialog() {
        Ok(path) => load_project_from_path(state, &path),
        Err(ProjectIoError::Cancelled) => false,
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Project open failed: {}",
                error
            )));
            false
        }
    }
}
