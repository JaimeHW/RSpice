//! PDK setup workflow.

use std::path::Path;

use egui::Context;

use crate::diagnostics::ConsoleMessage;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::session::pdk_settings::PdkSettingsDialogResult;

#[cfg(target_arch = "wasm32")]
type BrowserModelImport = Result<Option<Vec<(String, Vec<u8>)>>, String>;

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_MODEL_IMPORTS: std::cell::RefCell<std::collections::VecDeque<BrowserModelImport>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

#[cfg(not(target_arch = "wasm32"))]
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
    _ctx: &Context,
    app: &mut RSpiceApp,
    path: &Path,
    library_name: &str,
    library_stats: Option<(usize, usize)>,
) {
    let mut candidate = app.state.pdk_config.clone();
    candidate.add_recent_file(path);
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.state.pdk_config = candidate;
        if let Err(err) = app.state.pdk_config.save() {
            emit_pdk_save_warning(&mut app.state, "was updated", err.to_string());
        }
    }
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = app.start_browser_pdk_recent_file_publication(_ctx, candidate) {
        emit_pdk_save_warning(&mut app.state, "was updated", error);
    }

    app.state.push_user_message(ConsoleMessage::info(format!(
        "Loaded library '{}' from {}",
        library_name,
        path.display()
    )));

    if let Some((model_count, corner_count)) = library_stats {
        app.state.push_user_message(ConsoleMessage::info(format!(
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
    /// Publish one complete model-catalogue replacement as a project
    /// transaction. Every model/PDK authoring surface uses this path so a
    /// corner edit, pack activation, or import cannot leave prepared
    /// simulation state valid against superseded execution sources.
    pub(in crate::workbench) fn publish_model_library_candidate(
        &mut self,
        candidate: crate::state::ModelLibraryManager,
    ) -> Result<(), String> {
        let execution_changed = candidate.execution_catalog_digest()
            != self.state.model_library_manager.execution_catalog_digest();
        if execution_changed
            && self.state.project_lifecycle.project_open
            && self.state.workbench.safe_mode.project_read_only()
        {
            return Err("The project is read-only".to_owned());
        }
        if execution_changed && self.state.project_lifecycle.project_open {
            self.state
                .workspace
                .project
                .next_revision()
                .map_err(|error| error.to_string())?;
            self.state
                .workspace
                .project
                .advance_revision()
                .map_err(|error| error.to_string())?;
            self.state.workspace.project_metadata_dirty = true;
        }
        self.state.model_library_manager = candidate;
        if execution_changed {
            self.state.design_execution_epoch = self.state.design_execution_epoch.wrapping_add(1);
            self.state.ui.netlist.current_generation_input_digest = None;
            self.invalidate_simulation_preflight();
        }
        Ok(())
    }

    pub(in crate::workbench) fn process_pdk_settings_dialog(&mut self, ctx: &Context) {
        #[cfg(target_arch = "wasm32")]
        poll_browser_model_imports(ctx, self);
        let result = super::pdk_settings::render_pdk_settings_dialog(
            ctx,
            &mut self.state.pdk_settings_dialog,
        );
        match result {
            PdkSettingsDialogResult::Applied(config) => {
                let mut candidate = self.state.model_library_manager.clone();
                let load_result = match candidate.load_from_pdk_config(&config) {
                    #[cfg(not(target_arch = "wasm32"))]
                    Ok(count) => self
                        .publish_model_library_candidate(candidate)
                        .map(|()| count)
                        .map_err(|error| vec![error]),
                    #[cfg(target_arch = "wasm32")]
                    Ok(count) => Ok(count),
                    Err(errors) => Err(errors),
                };
                #[cfg(not(target_arch = "wasm32"))]
                {
                    if let Err(err) = persist_pdk_config(&mut self.state, config) {
                        emit_pdk_save_warning(&mut self.state, "was applied", err);
                    }
                    emit_pdk_apply_messages(&mut self.state, load_result);
                }
                #[cfg(target_arch = "wasm32")]
                {
                    let model_candidate = load_result.as_ref().ok().map(|_| candidate);
                    if let Err(error) = self.start_browser_pdk_settings_publication(
                        ctx,
                        config,
                        model_candidate,
                        load_result,
                    ) {
                        emit_pdk_save_warning(&mut self.state, "was applied", error);
                    }
                }
            }
            PdkSettingsDialogResult::LoadFile(path) => {
                let mut candidate = self.state.model_library_manager.clone();
                match candidate
                    .load_library_file(&path, None)
                    .and_then(|library_name| {
                        self.publish_model_library_candidate(candidate)?;
                        Ok(library_name)
                    }) {
                    Ok(library_name) => {
                        let library_stats = self
                            .state
                            .model_library_manager
                            .get_library(&library_name)
                            .map(|lib| (lib.model_count(), lib.corner_count()));
                        emit_pdk_file_load_success_message(
                            ctx,
                            self,
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

#[cfg(target_arch = "wasm32")]
fn start_browser_model_import(ctx: &Context) {
    let repaint = ctx.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let picked = rfd::AsyncFileDialog::new()
            .add_filter(
                "SPICE model bundle",
                &["lib", "model", "spice", "cir", "inc"],
            )
            .pick_files()
            .await;
        let result = match picked {
            None => Ok(None),
            Some(files) => {
                let mut imported = Vec::with_capacity(files.len());
                let mut total_bytes = 0_u64;
                let mut error = None;
                for file in files {
                    let name = file.file_name();
                    let size = file.inner().size();
                    if !size.is_finite()
                        || size < 0.0
                        || size > crate::io::project_io::MAX_PROJECT_FILE_BYTES as f64
                    {
                        error = Some(format!(
                            "Selected model source '{name}' exceeds the supported {}-byte limit",
                            crate::io::project_io::MAX_PROJECT_FILE_BYTES
                        ));
                        break;
                    }
                    let bytes = file.read().await;
                    if bytes.len() as u64 > crate::io::project_io::MAX_PROJECT_FILE_BYTES {
                        error = Some(format!(
                            "Selected model source '{name}' grew beyond the supported size limit"
                        ));
                        break;
                    }
                    total_bytes = total_bytes.saturating_add(bytes.len() as u64);
                    if total_bytes > crate::io::project_io::MAX_PROJECT_FILE_BYTES {
                        error = Some(format!(
                            "Selected model bundle exceeds the supported {}-byte aggregate limit",
                            crate::io::project_io::MAX_PROJECT_FILE_BYTES
                        ));
                        break;
                    }
                    imported.push((name, bytes));
                }
                if let Some(error) = error {
                    Err(error)
                } else {
                    Ok(Some(imported))
                }
            }
        };
        BROWSER_MODEL_IMPORTS.with(|queue| queue.borrow_mut().push_back(result));
        repaint.request_repaint();
    });
}

#[cfg(target_arch = "wasm32")]
fn poll_browser_model_imports(ctx: &Context, app: &mut RSpiceApp) {
    let completions =
        BROWSER_MODEL_IMPORTS.with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    for completion in completions {
        let result = completion.and_then(|picked| {
            let Some(files) = picked else {
                return Ok(None);
            };
            let mut candidate = app.state.model_library_manager.clone();
            let library = candidate
                .load_library_bundle_bytes(files, None)
                .map_err(|error| error.to_string())?;
            app.publish_model_library_candidate(candidate)?;
            Ok(Some(library))
        });
        match result {
            Ok(Some(library)) => {
                let message = format!(
                    "Imported browser model library '{library}' with its exact authenticated source bundle"
                );
                app.state
                    .push_user_message(ConsoleMessage::info(message.clone()));
                app.state
                    .ui
                    .toasts
                    .success(ctx, "Model library imported", message);
            }
            Ok(None) => {}
            Err(error) => {
                app.state
                    .push_user_message(ConsoleMessage::error(error.clone()));
                app.state
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

    #[test]
    fn publishing_imported_models_advances_project_revision_and_invalidates_execution() {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.project_open = true;
        let revision_before = app.state.workspace.project.revision();
        let epoch_before = app.state.design_execution_epoch;
        let mut candidate = app.state.model_library_manager.clone();
        candidate
            .load_library_bytes(
                "imported.lib",
                b".model imported_n NMOS (LEVEL=1 KP=1e-3)\n".to_vec(),
                None,
            )
            .expect("candidate imports");

        app.publish_model_library_candidate(candidate)
            .expect("candidate publishes");

        assert_eq!(
            app.state.workspace.project.revision(),
            revision_before.next().expect("revision advances")
        );
        assert!(app.state.workspace.project_metadata_dirty);
        assert_eq!(
            app.state.design_execution_epoch,
            epoch_before.wrapping_add(1)
        );
        assert!(
            app.state
                .model_library_manager
                .libraries_sorted()
                .iter()
                .any(|library| library.models.contains_key("imported_n"))
        );
        assert!(
            app.state
                .ui
                .netlist
                .current_generation_input_digest
                .is_none()
        );
    }
}
