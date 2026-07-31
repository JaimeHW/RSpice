//! PDK setup workflow.

use std::path::Path;

use egui::Context;

use crate::diagnostics::ConsoleMessage;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::design_history::{
    publish_model_library_candidate, publish_model_library_set_candidate,
};
use crate::workbench::app_state::session::pdk_settings::PdkSettingsDialogResult;

#[cfg(target_arch = "wasm32")]
type BrowserModelImport = Result<Option<(String, Vec<u8>)>, String>;

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_MODEL_IMPORTS: std::cell::RefCell<std::collections::VecDeque<BrowserModelImport>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

fn apply_pdk_configuration_with_persistence(
    state: &mut AppState,
    mut config: crate::state::pdk_config::PdkConfig,
    mut persist: impl FnMut(&crate::state::pdk_config::PdkConfig) -> Result<(), String>,
) -> Result<usize, Vec<String>> {
    if state.project_lifecycle.project_open && state.workbench.safe_mode.project_read_only() {
        return Err(vec![
            "PDK settings cannot change while the project is read-only.".to_owned(),
        ]);
    }

    let previous = state.pdk_config.clone();
    let mut candidate = state.model_library_manager.clone();
    let loaded = candidate.replace_from_pdk_config(Some(&previous), &mut config)?;
    candidate
        .validate_attached_technology(state.workspace.project.technology_binding())
        .map_err(|error| {
            vec![format!(
                "PDK settings would invalidate the attached project technology: {error}"
            )]
        })?;
    persist(&config).map_err(|error| {
        vec![format!(
            "PDK settings were not applied because the configuration could not be persisted: {error}"
        )]
    })?;

    let publication = if state.project_lifecycle.project_open {
        publish_model_library_set_candidate(state, candidate, "apply configured PDK model sources")
            .map(|_| ())
    } else {
        state.model_library_manager = candidate;
        Ok(())
    };
    if let Err(error) = publication {
        let rollback = persist(&previous).err();
        let mut errors = vec![error];
        if let Some(rollback) = rollback {
            errors.push(format!(
                "PDK configuration rollback also failed; verify the persisted settings before restarting: {rollback}"
            ));
        }
        return Err(errors);
    }

    state.pdk_config = config;
    Ok(loaded)
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
                "PDK settings were not applied; no configured model libraries changed ({} errors)",
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
    pub(in crate::workbench) fn process_pdk_settings_dialog(&mut self, ctx: &Context) {
        #[cfg(target_arch = "wasm32")]
        poll_browser_model_imports(ctx, &mut self.state);
        let result = super::pdk_settings::render_pdk_settings_dialog(
            ctx,
            &mut self.state.pdk_settings_dialog,
        );
        match result {
            PdkSettingsDialogResult::Applied(config) => {
                let load_result =
                    apply_pdk_configuration_with_persistence(&mut self.state, config, |config| {
                        config.save().map_err(|error| error.to_string())
                    });
                emit_pdk_apply_messages(&mut self.state, load_result);
            }
            PdkSettingsDialogResult::LoadFile(path) => {
                let mut candidate = self.state.model_library_manager.clone();
                match candidate.load_library_file(&path, None) {
                    Ok(library_name) => {
                        let library_stats = candidate
                            .get_library(&library_name)
                            .map(|lib| (lib.model_count(), lib.corner_count()));
                        let publication = if self.state.project_lifecycle.project_open {
                            publish_model_library_candidate(
                                &mut self.state,
                                candidate,
                                &library_name,
                                format!("load model library {}", path.display()),
                            )
                            .map(|_| ())
                        } else {
                            self.state.model_library_manager = candidate;
                            Ok(())
                        };
                        match publication {
                            Ok(()) => emit_pdk_file_load_success_message(
                                &mut self.state,
                                path.as_path(),
                                &library_name,
                                library_stats,
                            ),
                            Err(error) => emit_pdk_file_load_error_message(
                                &mut self.state,
                                path.as_path(),
                                error,
                            ),
                        }
                    }
                    Err(error) => {
                        emit_pdk_file_load_error_message(&mut self.state, path.as_path(), &error);
                    }
                }
            }
            PdkSettingsDialogResult::ImportBrowserFile => {
                #[cfg(target_arch = "wasm32")]
                start_browser_model_import(ctx);
                #[cfg(not(target_arch = "wasm32"))]
                self.state.push_user_message(ConsoleMessage::error(
                    "Browser model import is unavailable on this platform".to_owned(),
                ));
            }
            PdkSettingsDialogResult::Cancelled => {}
            PdkSettingsDialogResult::None => {}
        }
    }
}

fn load_persisted_pdk_sources_with_persistence(
    state: &mut AppState,
    mut persist: impl FnMut(&crate::state::pdk_config::PdkConfig) -> Result<(), String>,
) -> Result<usize, Vec<String>> {
    let previous = state.pdk_config.clone();
    let mut config = previous.clone();
    let mut candidate = state.model_library_manager.clone();
    let loaded = candidate.replace_from_pdk_config(Some(&previous), &mut config)?;
    candidate
        .validate_attached_technology(state.workspace.project.technology_binding())
        .map_err(|error| {
            vec![format!(
                "Persisted PDK settings would invalidate the attached project technology: {error}"
            )]
        })?;
    persist(&config).map_err(|error| {
        vec![format!(
            "Persisted PDK source ownership could not be recorded: {error}"
        )]
    })?;
    state.model_library_manager = candidate;
    state.pdk_config = config;
    Ok(loaded)
}

pub(in crate::workbench) fn load_persisted_pdk_sources_at_startup(state: &mut AppState) {
    let result = load_persisted_pdk_sources_with_persistence(state, |config| {
        config.save().map_err(|error| error.to_string())
    });
    match result {
        Ok(count) if count > 0 => {
            state.push_user_message(ConsoleMessage::info(format!(
                "Loaded {count} persisted configured model libraries"
            )));
        }
        Ok(_) => {}
        Err(errors) => {
            state.push_user_message(ConsoleMessage::warning(
                "Persisted PDK model sources were not loaded; the previous model-library state was retained."
                    .to_owned(),
            ));
            for error in errors {
                state.push_user_message(ConsoleMessage::error(error));
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn start_browser_model_import(ctx: &Context) {
    let repaint = ctx.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let picked = rfd::AsyncFileDialog::new()
            .add_filter("SPICE model library", &["lib", "model", "spice", "cir"])
            .pick_file()
            .await;
        let result = match picked {
            None => Ok(None),
            Some(file) => {
                let name = file.file_name();
                let size = file.inner().size();
                if !size.is_finite()
                    || size < 0.0
                    || size > crate::io::project_io::MAX_PROJECT_FILE_BYTES as f64
                {
                    Err(format!(
                        "Selected model library exceeds the supported {}-byte limit",
                        crate::io::project_io::MAX_PROJECT_FILE_BYTES
                    ))
                } else {
                    let bytes = file.read().await;
                    if bytes.len() as u64 > crate::io::project_io::MAX_PROJECT_FILE_BYTES {
                        Err(
                            "Selected model library grew beyond the supported size limit"
                                .to_owned(),
                        )
                    } else {
                        Ok(Some((name, bytes)))
                    }
                }
            }
        };
        BROWSER_MODEL_IMPORTS.with(|queue| queue.borrow_mut().push_back(result));
        repaint.request_repaint();
    });
}

#[cfg(target_arch = "wasm32")]
fn poll_browser_model_imports(ctx: &Context, state: &mut AppState) {
    let completions =
        BROWSER_MODEL_IMPORTS.with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    for completion in completions {
        let result = completion.and_then(|picked| {
            let Some((name, bytes)) = picked else {
                return Ok(None);
            };
            let mut candidate = state.model_library_manager.clone();
            let library = candidate
                .load_library_bytes(&name, bytes, None)
                .map_err(|error| error.to_string())?;
            if state.project_lifecycle.project_open {
                publish_model_library_candidate(
                    state,
                    candidate,
                    &library,
                    format!("import browser model library {name}"),
                )?;
            } else {
                state.model_library_manager = candidate;
            }
            Ok(Some(library))
        });
        match result {
            Ok(Some(library)) => {
                let message = format!(
                    "Imported browser model library '{library}' with exact retained source bytes"
                );
                state.push_user_message(ConsoleMessage::info(message.clone()));
                state
                    .ui
                    .toasts
                    .success(ctx, "Model library imported", message);
            }
            Ok(None) => {}
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(error.clone()));
                state
                    .ui
                    .toasts
                    .error_with_title(ctx, "Model import failed", error);
            }
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::state::pdk_config::PdkConfig;

    fn configured_root(label: &str, file_name: &str, model_name: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "rspice-pdk-workflow-{label}-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&root).expect("create configured source root");
        std::fs::write(
            root.join(file_name),
            format!(".model {model_name} NMOS (LEVEL=1)\n"),
        )
        .expect("write configured model source");
        root
    }

    #[test]
    fn configured_sources_publish_and_unload_as_one_guarded_project_transaction() {
        let alpha_root = configured_root("alpha", "alpha.lib", "alpha_n");
        let beta_root = configured_root("beta", "beta.lib", "beta_n");
        let mut state = AppState::default();
        state.project_lifecycle.project_open = true;
        state.model_library_manager.clear();
        state.pdk_config = PdkConfig::new();
        let initial_revision = state.workspace.project.revision();
        let initial_epoch = state.design_execution_epoch;
        let mut config = PdkConfig::new();
        config.add_library_path(alpha_root.to_string_lossy().into_owned());
        config.add_library_path(beta_root.to_string_lossy().into_owned());

        assert_eq!(
            apply_pdk_configuration_with_persistence(&mut state, config, |_| Ok(()))
                .expect("configured sources publish"),
            2
        );
        let first_revision = state.workspace.project.revision();
        assert!(first_revision > initial_revision);
        assert_eq!(state.design_execution_epoch, initial_epoch.wrapping_add(1));
        assert!(state.workspace.project_metadata_dirty);
        assert!(state.model_library_manager.get_library("alpha").is_some());
        assert!(state.model_library_manager.get_library("beta").is_some());
        assert_eq!(state.pdk_config.managed_model_sources.len(), 2);
        assert!(state.can_undo_project_design());

        let mut disabled = state.pdk_config.clone();
        disabled.toggle_path_enabled(0);
        assert_eq!(
            apply_pdk_configuration_with_persistence(&mut state, disabled, |_| Ok(()))
                .expect("disabled source publishes"),
            1
        );
        assert!(state.workspace.project.revision() > first_revision);
        assert_eq!(state.design_execution_epoch, initial_epoch.wrapping_add(2));
        assert!(state.model_library_manager.get_library("alpha").is_none());
        assert!(state.model_library_manager.get_library("beta").is_some());

        assert_eq!(
            state
                .undo_project_design()
                .expect("undo configured source set"),
            Some("apply configured PDK model sources".to_owned())
        );
        assert!(state.model_library_manager.get_library("alpha").is_some());
        assert!(state.model_library_manager.get_library("beta").is_some());
        assert_eq!(state.design_execution_epoch, initial_epoch.wrapping_add(3));

        std::fs::remove_dir_all(alpha_root).expect("remove alpha source root");
        std::fs::remove_dir_all(beta_root).expect("remove beta source root");
    }

    #[test]
    fn persistence_failure_leaves_configuration_manager_and_project_unchanged() {
        let root = configured_root("persistence", "persisted.lib", "persisted_n");
        let mut state = AppState::default();
        state.project_lifecycle.project_open = true;
        state.model_library_manager.clear();
        state.pdk_config = PdkConfig::new();
        let revision = state.workspace.project.revision();
        let epoch = state.design_execution_epoch;
        let mut config = PdkConfig::new();
        config.add_library_path(root.to_string_lossy().into_owned());

        let errors = apply_pdk_configuration_with_persistence(&mut state, config, |_| {
            Err("storage unavailable".to_owned())
        })
        .expect_err("failed persistence rejects the candidate");

        assert!(
            errors
                .iter()
                .any(|error| error.contains("storage unavailable"))
        );
        assert!(
            state
                .model_library_manager
                .get_library("persisted")
                .is_none()
        );
        assert!(state.pdk_config.library_paths().is_empty());
        assert_eq!(state.workspace.project.revision(), revision);
        assert_eq!(state.design_execution_epoch, epoch);
        assert!(!state.can_undo_project_design());

        std::fs::remove_dir_all(root).expect("remove persistence source root");
    }

    #[test]
    fn startup_service_hydrates_persisted_sources_without_dirtying_a_closed_project() {
        let root = configured_root("startup", "startup.lib", "startup_n");
        let mut state = AppState::default();
        state.model_library_manager.clear();
        let revision = state.workspace.project.revision();
        let epoch = state.design_execution_epoch;
        let mut config = PdkConfig::new();
        config.add_library_path(root.to_string_lossy().into_owned());

        assert_eq!(
            {
                state.pdk_config = config;
                load_persisted_pdk_sources_with_persistence(&mut state, |_| Ok(()))
            }
            .expect("startup configured source hydration"),
            1
        );
        assert!(state.model_library_manager.get_library("startup").is_some());
        assert_eq!(state.workspace.project.revision(), revision);
        assert_eq!(state.design_execution_epoch, epoch);
        assert!(!state.workspace.project_metadata_dirty);
        assert!(!state.can_undo_project_design());

        std::fs::remove_dir_all(root).expect("remove startup source root");
    }
}
