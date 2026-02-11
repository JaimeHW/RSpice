use std::path::Path;

use egui::Context;

use super::{AppState, ConsoleMessage, RSpiceApp};

fn persist_pdk_config(state: &mut AppState, config: crate::state::pdk_config::PdkConfig) {
    state.pdk_config = config;
    let _ = state.pdk_config.save();
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
    let _ = state.pdk_config.save();

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
                persist_pdk_config(&mut self.state, config);
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
                        emit_pdk_file_load_error_message(
                            &mut self.state,
                            path.as_path(),
                            error.to_string(),
                        );
                    }
                }
            }
            crate::panels::PdkSettingsDialogResult::Cancelled => {}
            crate::panels::PdkSettingsDialogResult::None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_emit_pdk_apply_messages_success() {
        let mut state = AppState::default();
        state.clear_primary_log();

        emit_pdk_apply_messages(&mut state, Ok(2));

        assert_eq!(state.console_messages.len(), 1);
        assert_eq!(
            state.console_messages[0].level,
            super::super::ConsoleLevel::Info
        );
        assert_eq!(
            state.console_messages[0].message,
            "PDK settings applied: 2 libraries loaded".to_string()
        );
    }

    #[test]
    fn test_emit_pdk_apply_messages_with_errors() {
        let mut state = AppState::default();
        state.clear_primary_log();

        emit_pdk_apply_messages(
            &mut state,
            Err(vec![
                "bad include path".to_string(),
                "parse error".to_string(),
            ]),
        );

        assert_eq!(state.console_messages.len(), 3);
        assert_eq!(
            state.console_messages[0].level,
            super::super::ConsoleLevel::Warning
        );
        assert_eq!(
            state.console_messages[0].message,
            "PDK settings applied with 2 errors".to_string()
        );
        assert_eq!(
            state.console_messages[1].level,
            super::super::ConsoleLevel::Error
        );
        assert_eq!(
            state.console_messages[1].message,
            "bad include path".to_string()
        );
        assert_eq!(
            state.console_messages[2].level,
            super::super::ConsoleLevel::Error
        );
        assert_eq!(state.console_messages[2].message, "parse error".to_string());
    }

    #[test]
    fn test_emit_pdk_file_load_success_message_with_stats() {
        let mut state = AppState::default();
        state.clear_primary_log();
        let path = PathBuf::from("models/pdk.lib");

        emit_pdk_file_load_success_message(&mut state, &path, "my_pdk", Some((10, 3)));

        assert_eq!(state.console_messages.len(), 2);
        assert_eq!(
            state.console_messages[0].level,
            super::super::ConsoleLevel::Info
        );
        assert_eq!(
            state.console_messages[0].message,
            "Loaded library 'my_pdk' from models/pdk.lib".to_string()
        );
        assert_eq!(
            state.console_messages[1].message,
            "  10 models, 3 corners available".to_string()
        );
    }

    #[test]
    fn test_emit_pdk_file_load_error_message() {
        let mut state = AppState::default();
        state.clear_primary_log();
        let path = PathBuf::from("models/missing.lib");

        emit_pdk_file_load_error_message(&mut state, &path, "file not found");

        assert_eq!(state.console_messages.len(), 1);
        assert_eq!(
            state.console_messages[0].level,
            super::super::ConsoleLevel::Error
        );
        assert_eq!(
            state.console_messages[0].message,
            "Failed to load models/missing.lib: file not found".to_string()
        );
    }
}
