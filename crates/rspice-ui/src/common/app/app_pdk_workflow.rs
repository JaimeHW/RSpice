use std::path::Path;

use egui::Context;

use super::{AppState, ConsoleMessage, RSpiceApp};

fn persist_pdk_config(
    state: &mut AppState,
    config: crate::state::pdk_config::PdkConfig,
) -> Result<(), String> {
    state.pdk_config = config;
    state.pdk_config.save().map_err(|err| err.to_string())
}

fn emit_pdk_save_warning(state: &mut AppState, operation: &str, error: impl AsRef<str>) {
    state.push_user_message(ConsoleMessage::warning(format!(
        "PDK configuration {} but could not be persisted: {}",
        operation,
        error.as_ref()
    )));
}

fn emit_pdk_apply_messages(state: &mut AppState, load_result: Result<usize, Vec<String>>) {
    match load_result {
        Ok(count) => {
            state.push_user_message(ConsoleMessage::info(format!(
                "PDK settings applied: {} libraries loaded",
                count
            )));
        }
        Err(errors) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "PDK settings applied with {} errors",
                errors.len()
            )));
            for error in errors {
                state.push_user_message(ConsoleMessage::error(error));
            }
        }
    }
}

fn emit_pdk_file_load_success_message(
    state: &mut AppState,
    path: &Path,
    library_name: &str,
    library_stats: Option<(usize, usize)>,
) {
    state.pdk_config.add_recent_file(path);
    if let Err(err) = state.pdk_config.save() {
        emit_pdk_save_warning(state, "was updated", err.to_string());
    }

    state.push_user_message(ConsoleMessage::info(format!(
        "Loaded library '{}' from {}",
        library_name,
        path.display()
    )));

    if let Some((model_count, corner_count)) = library_stats {
        state.push_user_message(ConsoleMessage::info(format!(
            "  {} models, {} corners available",
            model_count, corner_count
        )));
    }
}

fn emit_pdk_file_load_error_message(state: &mut AppState, path: &Path, error: impl AsRef<str>) {
    state.push_user_message(ConsoleMessage::error(format!(
        "Failed to load {}: {}",
        path.display(),
        error.as_ref()
    )));
}

impl RSpiceApp {
    pub(super) fn process_pdk_settings_dialog(&mut self, ctx: &Context) {
        let result =
            crate::panels::render_pdk_settings_dialog(ctx, &mut self.state.pdk_settings_dialog);
        match result {
            crate::panels::PdkSettingsDialogResult::Applied(config) => {
                let load_result = self
                    .state
                    .model_library_manager
                    .load_from_pdk_config(&config);
                if let Err(err) = persist_pdk_config(&mut self.state, config) {
                    emit_pdk_save_warning(&mut self.state, "was applied", err);
                }
                emit_pdk_apply_messages(&mut self.state, load_result);
            }
            crate::panels::PdkSettingsDialogResult::LoadFile(path) => {
                match self
                    .state
                    .model_library_manager
                    .load_library_file(&path, None)
                {
                    Ok(library_name) => {
                        let library_stats = self
                            .state
                            .model_library_manager
                            .get_library(&library_name)
                            .map(|lib| (lib.model_count(), lib.corner_count()));
                        emit_pdk_file_load_success_message(
                            &mut self.state,
                            path.as_path(),
                            &library_name,
                            library_stats,
                        );
                    }
                    Err(error) => {
                        emit_pdk_file_load_error_message(&mut self.state, path.as_path(), &error);
                    }
                }
            }
            crate::panels::PdkSettingsDialogResult::Cancelled => {}
            crate::panels::PdkSettingsDialogResult::None => {}
        }
    }
}

