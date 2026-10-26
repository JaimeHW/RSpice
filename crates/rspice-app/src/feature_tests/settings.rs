//! The settings feature observed through a whole application: Preferences
//! and the keyboard shortcut workflows, in the cases that read the product's
//! own commands and bindings or another feature's state.

mod preference_libraries;
mod preference_page_defaults;
mod preference_pages;
mod preferences;
mod preferences_file;
#[cfg(not(target_arch = "wasm32"))]
mod preferences_file_store;
mod preferences_windows;
mod shortcut_artifact_files;
mod shortcut_artifact_import;
mod shortcut_editor;
mod shortcut_library;
mod shortcut_library_persistence;
mod shortcut_library_store;
mod shortcut_markdown;
mod shortcut_merge;
mod shortcut_merge_rollback;
#[cfg(not(target_arch = "wasm32"))]
mod shortcut_pdf;
#[cfg(not(target_arch = "wasm32"))]
mod shortcut_pdf_reference;
mod shortcut_portability;
mod shortcut_portability_dialogs;
mod shortcut_preferences;
mod shortcut_profile;
mod shortcut_profile_registry;
mod shortcut_profile_staging;
mod shortcut_profile_workflow;
mod shortcut_projection;
mod shortcut_reference_model;
mod shortcut_schema;
mod shortcut_vscode;
mod shortcut_vscode_import;

use crate::workbench::app::dialogs::preferences::PreferencesDialogs;
use crate::workbench::app_state::AppState;
use crate::workbench::commands::vocabulary::command_registry;

/// Preferences and the windows it opens, over the application state's
/// parts, for a test that draws them without the feature's context.
pub(crate) fn preferences_dialogs(state: &mut AppState) -> PreferencesDialogs<'_> {
    PreferencesDialogs {
        preferences_open: &mut state.settings.preferences_open,
        workspace_layout_manager: &mut state.settings.workspace_layout_manager,
        shortcut_portability: &mut state.settings.shortcut_portability,
        shortcut_editor: &mut state.settings.shortcut_editor,
        shortcut_policy_candidate: &mut state.settings.shortcut_policy_candidate,
        browser_spoken_feedback: &mut state.settings.browser_spoken_feedback,
        shortcut_library_persistence: &mut state.settings.shortcut_library_persistence,
        shortcut_library_publication_continuation: &mut state
            .settings
            .shortcut_library_publication_continuation,
        preferences_file: &mut state.settings.preferences_file,
        model: &mut state.model,
        workbench: &mut state.workbench,
        platform: &mut state.platform,
        cloud: &*state.cloud,
        commands: command_registry(),
    }
}
