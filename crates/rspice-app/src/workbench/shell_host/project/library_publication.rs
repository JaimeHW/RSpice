//! The Publish project and Restore project from publication dialogs'
//! composition over the whole application, and the transactions they run.

use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use egui::Context;
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_project::workbench::surfaces::project::library_publication::PublicationAction;
#[cfg(target_arch = "wasm32")]
use rspice_app_project::workbench::surfaces::project::library_publication::{
    BROWSER_PUBLICATION_COMPLETIONS, BROWSER_RESTORE_CHECKPOINTS, BROWSER_RESTORE_COMPLETIONS,
    BrowserPublicationCompletion, BrowserRestoreCheckpoint, start_browser_restore_picker,
};
use rspice_app_project::workbench::surfaces::project::library_publication::{
    PROJECT_ARTIFACT_EXTENSION, PUBLICATION_DIALOG_ID, ProjectLibraryPublicationCandidate,
    PublicationDraft, PublicationReadiness, RESTORE_DIALOG_ID, RestoreDraft, clear_restore_file,
    publication_fields, remember_identity, restore_fields, restore_publication_selector,
    safe_file_stem, selected_receipt, transaction_status,
};
use rspice_app_services::workbench::lifecycle::project_checkpoint::ProjectCheckpointReason;
use rspice_app_services::workbench::lifecycle::project_lifecycle::{
    ProjectDocumentId, dirty_documents,
};
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_services::workbench::workflows::export_workflow::SaveDialogConfig;
use rspice_app_state::diagnostics::{LogSeverity, LogSource};
use rspice_app_state::product::ContentDigest;
use rspice_app_state::state::workspace::ProjectLibraryPublicationReceipt;
use rspice_app_workbench::ui::widgets::{Button, Dialog, DialogChoice, DialogSize};
use rspice_ui_kit::panels::{property_card, property_row};

pub(crate) fn show(ctx: &Context, app: &mut RSpiceApp) {
    #[cfg(target_arch = "wasm32")]
    {
        poll_browser_publication_completions(ctx, app);
        poll_browser_restore_completions(ctx, app);
        poll_browser_restore_checkpoints(ctx, app);
    }
    show_publication_dialog(ctx, app);
    show_restore_dialog(ctx, app);
}

fn show_publication_dialog(ctx: &Context, app: &mut RSpiceApp) {
    let id = egui::Id::new(PUBLICATION_DIALOG_ID);
    let Some(mut draft) = ctx.data(|data| data.get_temp::<PublicationDraft>(id)) else {
        return;
    };
    let live_project_revision = app.state.model.workspace.content.project.revision().get();
    let live_library_revision = app.state.model.library_manager.revision();
    let stale = live_project_revision != draft.source_project_revision
        || live_library_revision != draft.source_library_revision;
    let lifecycle_error = publication_lifecycle_error(&app.state);
    let field_error = draft.field_error();
    let primary_enabled =
        !draft.busy && lifecycle_error.is_none() && field_error.is_none() && !stale;
    let choice = Dialog::new(
        "Publish project",
        if draft.busy {
            "Publishing\u{2026}"
        } else {
            "Publish"
        },
    )
    .description(
        "Write the project as it stands to a file and record the publication, so the project can be restored to it later.",
    )
    .size(DialogSize::Manager)
    .secondary("Cancel")
    .primary_enabled(primary_enabled)
    .show(ctx, |ui| {
        property_card(ui, "Project", |ui| {
            property_row(
                ui,
                "Project",
                app.state.model.workspace.content.project.display_name(),
            );
            property_row(
                ui,
                "Revision",
                &draft.source_project_revision.to_string(),
            );
        });
        ui.add_space(8.0);
        publication_fields(ui, &mut draft);
        if stale {
            transaction_status(
                ui,
                "The project changed while this dialog was open. Close it and publish again.",
                true,
            );
        } else if let Some(error) = draft.error.as_deref().or(lifecycle_error).or(field_error.as_deref()) {
            transaction_status(ui, error, true);
        } else if draft.busy {
            transaction_status(ui, "Waiting for the browser to save the file.", false);
        }
    });

    match choice {
        DialogChoice::Primary if primary_enabled => {
            draft.error = None;
            remember_identity(ctx, &draft.actor_id, &draft.authority_id);
            #[cfg(not(target_arch = "wasm32"))]
            match publish_native(app, &draft) {
                Ok(PublicationAction::Published(message)) => {
                    app.state
                        .record(LogSeverity::Info, LogSource::Studio, message);
                    ctx.data_mut(|data| {
                        data.remove::<PublicationDraft>(id);
                    });
                    return;
                }
                Ok(PublicationAction::Cancelled) => {
                    draft.error =
                        Some("Publishing was cancelled; nothing was recorded.".to_owned());
                }
                Err(error) => draft.error = Some(error),
            }
            #[cfg(target_arch = "wasm32")]
            match start_browser_publication(ctx, app, &draft) {
                Ok(publication_id) => {
                    draft.busy = true;
                    draft.active_publication_id = Some(publication_id);
                }
                Err(error) => draft.error = Some(error),
            }
        }
        DialogChoice::Secondary | DialogChoice::Cancelled => {
            ctx.data_mut(|data| {
                data.remove::<PublicationDraft>(id);
            });
            return;
        }
        DialogChoice::None | DialogChoice::Ghost | DialogChoice::Primary => {}
    }
    ctx.data_mut(|data| data.insert_temp(id, draft));
}

/// What restoring the project from `receipt` replaces in the open project:
/// the views with unsaved changes, by name, the model libraries attached
/// now, and the runs recorded since the publication.
pub(crate) struct RestoreReplaces {
    pub(crate) unsaved_views: Vec<String>,
    pub(crate) model_libraries: usize,
    pub(crate) newer_runs: usize,
}

pub(crate) fn restore_replaces(
    state: &AppState,
    receipt: &ProjectLibraryPublicationReceipt,
) -> RestoreReplaces {
    let published_seconds = receipt.created_unix_ms() as f64 / 1000.0;
    RestoreReplaces {
        unsaved_views: dirty_documents(state)
            .into_iter()
            .filter_map(|document| match document {
                ProjectDocumentId::CellView(reference) => Some(reference.display_path()),
                _ => None,
            })
            .collect(),
        model_libraries: state.model.model_library_manager.library_count(),
        newer_runs: state
            .model
            .simulation
            .retained
            .runs
            .iter()
            .filter(|run| run.timestamp > published_seconds)
            .count(),
    }
}

fn restore_replaces_card(ui: &mut egui::Ui, replaces: &RestoreReplaces) {
    property_card(ui, "Replaced in the open project", |ui| {
        if replaces.unsaved_views.is_empty() {
            property_row(ui, "Unsaved views", "None");
        } else {
            for view in &replaces.unsaved_views {
                property_row(ui, "Unsaved view", view);
            }
        }
        property_row(ui, "Simulation setup", "Replaced by the published setup");
        property_row(
            ui,
            "Model libraries",
            &format!(
                "{} now, replaced by the published ones",
                replaces.model_libraries
            ),
        );
        property_row(
            ui,
            "Newer results",
            &match replaces.newer_runs {
                0 => "None".to_owned(),
                1 => "1 run since the publication is removed".to_owned(),
                runs => format!("{runs} runs since the publication are removed"),
            },
        );
    });
}

fn show_restore_dialog(ctx: &Context, app: &mut RSpiceApp) {
    let id = egui::Id::new(RESTORE_DIALOG_ID);
    let Some(mut draft) = ctx.data(|data| data.get_temp::<RestoreDraft>(id)) else {
        return;
    };
    let receipts = app
        .state
        .model
        .workspace
        .content
        .project
        .library_publications()
        .to_vec();
    if receipts.is_empty() {
        ctx.data_mut(|data| {
            data.remove::<RestoreDraft>(id);
        });
        return;
    }
    if !receipts
        .iter()
        .any(|receipt| receipt.publication_id() == draft.publication_id)
    {
        draft.publication_id = receipts
            .last()
            .expect("nonempty publication list")
            .publication_id();
        clear_restore_file(&mut draft);
    }
    let replaces = restore_replaces(
        &app.state,
        selected_receipt(&receipts, draft.publication_id).expect("the publication is retained"),
    );
    let lifecycle_error = restore_lifecycle_error(&app.state);
    let field_error = draft.field_error();
    let primary_enabled = !draft.busy && lifecycle_error.is_none() && field_error.is_none();
    let mut choose_file = false;
    let choice = Dialog::new("Restore project from publication", "Restore project")
        .description(
            "Replace the open project with the copy saved when it was published. A checkpoint of the project as it stands is taken first, so the Recovery page can put it back.",
        )
        .size(DialogSize::Manager)
        .secondary("Cancel")
        .primary_enabled(primary_enabled)
        .destructive()
        .show(ctx, |ui| {
            restore_publication_selector(ui, &receipts, &mut draft);
            ui.add_space(8.0);
            restore_replaces_card(ui, &replaces);
            ui.add_space(8.0);
            property_card(ui, "Published file", |ui| {
                let selected = selected_receipt(&receipts, draft.publication_id)
                    .expect("selected publication remains retained");
                property_row(
                    ui,
                    "Size",
                    &format!("{} bytes", selected.snapshot_byte_len()),
                );
                property_row(
                    ui,
                    "Chosen file",
                    draft.artifact_name.as_deref().unwrap_or("None"),
                );
                let choose = Button::new(if draft.artifact_bytes.is_some() {
                    "Choose another file\u{2026}"
                } else {
                    "Choose file\u{2026}"
                })
                .enabled(!draft.busy)
                .show(ui);
                if choose.clicked() {
                    choose_file = true;
                }
            });
            ui.add_space(8.0);
            restore_fields(ui, &mut draft);
            if let Some(error) = draft.error.as_deref().or(lifecycle_error).or(field_error.as_deref()) {
                transaction_status(ui, error, true);
            } else if draft.busy {
                transaction_status(ui, "Waiting for the browser.", false);
            }
        });

    if choose_file {
        draft.error = None;
        #[cfg(not(target_arch = "wasm32"))]
        match pick_native_restore_file(app, &draft) {
            Ok(Some((name, bytes))) => {
                draft.artifact_name = Some(name);
                draft.artifact_bytes = Some(bytes);
            }
            Ok(None) => {
                draft.error = Some("No file was chosen; the project is unchanged.".to_owned());
            }
            Err(error) => draft.error = Some(error),
        }
        #[cfg(target_arch = "wasm32")]
        match start_browser_restore_picker(ctx) {
            Ok(picker_id) => {
                draft.busy = true;
                draft.active_picker_id = Some(picker_id);
            }
            Err(error) => draft.error = Some(error),
        }
    }

    match choice {
        DialogChoice::Primary if primary_enabled => {
            remember_identity(ctx, &draft.actor_id, &draft.authority_id);
            #[cfg(not(target_arch = "wasm32"))]
            match restore_with_checkpoint(app, &draft) {
                Ok(message) => {
                    app.state
                        .record(LogSeverity::Info, LogSource::Studio, message);
                    ctx.data_mut(|data| {
                        data.remove::<RestoreDraft>(id);
                    });
                    return;
                }
                Err(error) => {
                    draft.error = Some(error);
                    clear_restore_file(&mut draft);
                }
            }
            #[cfg(target_arch = "wasm32")]
            match start_browser_restore_checkpoint(ctx, app, &draft) {
                Ok(()) => {
                    draft.busy = true;
                    draft.error = None;
                }
                Err(error) => draft.error = Some(error),
            }
        }
        DialogChoice::Secondary | DialogChoice::Cancelled => {
            ctx.data_mut(|data| {
                data.remove::<RestoreDraft>(id);
            });
            return;
        }
        DialogChoice::None | DialogChoice::Ghost | DialogChoice::Primary => {}
    }
    ctx.data_mut(|data| data.insert_temp(id, draft));
}

/// Take a checkpoint of the project as it stands, then restore it from the
/// chosen publication. No checkpoint, no restore.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn restore_with_checkpoint(
    app: &mut RSpiceApp,
    draft: &RestoreDraft,
) -> Result<String, String> {
    let bytes = draft
        .artifact_bytes
        .as_deref()
        .ok_or_else(|| "Choose the published file.".to_owned())?;
    let checkpoint = rspice_app_services::workbench::lifecycle::project_checkpoint::create(
        &mut app.state,
        ProjectCheckpointReason::PublicationRestore,
    )
    .map_err(|error| {
        format!("The project was not restored: its checkpoint could not be taken ({error}).")
    })?;
    app.state.project.project_checkpoint_recovery.invalidate();
    restore_after_checkpoint(app, draft, bytes, checkpoint.project_revision())
}

/// Restore the project from the publication `draft` names, once the
/// checkpoint of revision `checkpoint_revision` is taken.
fn restore_after_checkpoint(
    app: &mut RSpiceApp,
    draft: &RestoreDraft,
    bytes: &[u8],
    checkpoint_revision: u64,
) -> Result<String, String> {
    let label = app
        .state
        .model
        .workspace
        .content
        .project
        .library_publications()
        .iter()
        .find(|receipt| receipt.publication_id() == draft.publication_id)
        .map(|receipt| {
            format!(
                "#{:04} \u{201c}{}\u{201d}",
                receipt.sequence(),
                receipt.label()
            )
        })
        .unwrap_or_default();
    restore_project_from_publication(
        &mut app.state,
        draft.publication_id,
        bytes,
        draft.actor_id.trim(),
        draft.authority_id.trim(),
        draft.reason.trim(),
    )?;
    Ok(format!(
        "Restored the project from publication {label}. The checkpoint \u{201c}{}\u{201d} on the Recovery page holds revision {checkpoint_revision} as it was.",
        ProjectCheckpointReason::PublicationRestore.label()
    ))
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn publish_native(
    app: &mut RSpiceApp,
    draft: &PublicationDraft,
) -> Result<PublicationAction, String> {
    if !app.export_workflow_io.saved_paths_are_reopenable() {
        return Err(
            "This build cannot save a file that can be opened again; nothing was published."
                .to_owned(),
        );
    }
    let default_name = publication_filename(&app.state, &draft.label);
    let Some(mut path) = app.export_workflow_io.show_save_dialog(SaveDialogConfig {
        title: "Publish project",
        default_name: &default_name,
        filter_name: "RSpice project publication",
        filter_extensions: &[PROJECT_ARTIFACT_EXTENSION],
    })?
    else {
        return Ok(PublicationAction::Cancelled);
    };
    rspice_app_services::io::file_exchange::ensure_file_extension(
        &mut path,
        PROJECT_ARTIFACT_EXTENSION,
    );
    let destination = app.export_workflow_io.observe_destination(&path)?;
    let candidate = prepare_project_library_publication(
        &app.state,
        draft.label.trim(),
        draft.actor_id.trim(),
        draft.authority_id.trim(),
        draft.reason.trim(),
    )?;
    app.export_workflow_io.write_bytes_file_observed(
        &destination,
        candidate.artifact_bytes(),
        "application/vnd.rspice.project+json",
    )?;
    let receipt = commit_project_library_publication(&mut app.state, candidate)?;
    Ok(PublicationAction::Published(format!(
        "Published the project to {} as publication #{:04}.",
        destination.path().display(),
        receipt.sequence(),
    )))
}

#[cfg(target_arch = "wasm32")]
fn start_browser_publication(
    ctx: &Context,
    app: &RSpiceApp,
    draft: &PublicationDraft,
) -> Result<uuid::Uuid, String> {
    use rspice_app_services::workbench::lifecycle::project_lifecycle::{
        BrowserBindingBackend, BrowserWriteTarget, start_browser_write,
    };

    let candidate = prepare_project_library_publication(
        &app.state,
        draft.label.trim(),
        draft.actor_id.trim(),
        draft.authority_id.trim(),
        draft.reason.trim(),
    )?;
    let publication_id = candidate.publication_id();
    let bytes = candidate.artifact_bytes().to_vec();
    let suggested_name = publication_filename(&app.state, &draft.label);
    let target = BrowserWriteTarget {
        handle_id: None,
        intent: rspice_project::persistence::browser::BrowserWriteIntent::fresh(
            app.state.model.workspace.content.project.id().to_string(),
            BrowserBindingBackend::ExternalFile,
        ),
    };
    let repaint = ctx.clone();
    start_browser_write(target, false, &suggested_name, bytes, move |result| {
        BROWSER_PUBLICATION_COMPLETIONS.with(|queue| {
            queue
                .borrow_mut()
                .push_back(BrowserPublicationCompletion { candidate, result });
        });
        repaint.request_repaint();
    })?;
    Ok(publication_id)
}

#[cfg(target_arch = "wasm32")]
fn poll_browser_publication_completions(ctx: &Context, app: &mut RSpiceApp) {
    use rspice_app_services::workbench::lifecycle::project_lifecycle::{
        BrowserWriteResult, release_browser_handle,
    };

    let completions = BROWSER_PUBLICATION_COMPLETIONS
        .with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    for completion in completions {
        let publication_id = completion.candidate.publication_id();
        let cancelled = matches!(completion.result, BrowserWriteResult::Cancelled);
        let result = match completion.result {
            BrowserWriteResult::Saved {
                handle_id,
                display_name,
                digest,
                ..
            }
            | BrowserWriteResult::SavedSessionOnly {
                handle_id,
                display_name,
                digest,
                ..
            } => {
                release_browser_handle(handle_id);
                if digest != completion.candidate.snapshot_digest() {
                    Err(
                        "The saved file did not read back as written; nothing was recorded."
                            .to_owned(),
                    )
                } else {
                    commit_project_library_publication(&mut app.state, completion.candidate).map(
                        |receipt| {
                            format!(
                                "Published the project to {display_name} as publication #{:04}.",
                                receipt.sequence(),
                            )
                        },
                    )
                }
            }
            BrowserWriteResult::Cancelled => {
                Err("Publishing was cancelled; nothing was recorded.".to_owned())
            }
            BrowserWriteResult::ExternalChange { .. } => Err(
                "The chosen file changed while it was being saved; nothing was recorded."
                    .to_owned(),
            ),
            BrowserWriteResult::Failed(error) => Err(format!(
                "Publishing the project failed: {error}. Nothing was recorded."
            )),
        };
        match result {
            Ok(message) => {
                app.state
                    .record(LogSeverity::Info, LogSource::Studio, message);
                ctx.data_mut(|data| {
                    if data
                        .get_temp::<PublicationDraft>(egui::Id::new(PUBLICATION_DIALOG_ID))
                        .is_some_and(|draft| draft.active_publication_id == Some(publication_id))
                    {
                        data.remove::<PublicationDraft>(egui::Id::new(PUBLICATION_DIALOG_ID));
                    }
                });
            }
            Err(error) => {
                // The dialog may have been closed while the browser wrote, so
                // a failure is recorded as well as shown in it.
                if !cancelled {
                    app.state
                        .record(LogSeverity::Error, LogSource::Studio, error.clone());
                }
                ctx.data_mut(|data| {
                    let id = egui::Id::new(PUBLICATION_DIALOG_ID);
                    if let Some(mut draft) = data.get_temp::<PublicationDraft>(id)
                        && draft.active_publication_id == Some(publication_id)
                    {
                        draft.busy = false;
                        draft.active_publication_id = None;
                        draft.error = Some(error);
                        data.insert_temp(id, draft);
                    }
                });
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn pick_native_restore_file(
    app: &RSpiceApp,
    draft: &RestoreDraft,
) -> Result<Option<(String, Vec<u8>)>, String> {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("RSpice project publication", &[PROJECT_ARTIFACT_EXTENSION])
        .set_title("Choose the published project file")
        .pick_file()
    else {
        return Ok(None);
    };
    let metadata = std::fs::metadata(&path)
        .map_err(|error| format!("Could not read '{}': {error}", path.display()))?;
    if metadata.len() > rspice_app_services::io::project_io::MAX_PROJECT_FILE_BYTES {
        return Err(format!(
            "'{}' is larger than a project file can be ({} bytes).",
            path.display(),
            rspice_app_services::io::project_io::MAX_PROJECT_FILE_BYTES
        ));
    }
    let bytes = std::fs::read(&path)
        .map_err(|error| format!("Could not read '{}': {error}", path.display()))?;
    validate_restore_file(&app.state, draft.publication_id, &bytes)?;
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .map(str::to_owned)
        .unwrap_or_else(|| path.display().to_string());
    Ok(Some((name, bytes)))
}

#[cfg(target_arch = "wasm32")]
fn poll_browser_restore_completions(ctx: &Context, app: &mut RSpiceApp) {
    use rspice_app_services::workbench::lifecycle::project_lifecycle::{
        BrowserOpenResult, release_browser_handle,
    };
    use sha2::Digest as _;

    let completions =
        BROWSER_RESTORE_COMPLETIONS.with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    for completion in completions {
        let id = egui::Id::new(RESTORE_DIALOG_ID);
        let mut draft = ctx.data(|data| data.get_temp::<RestoreDraft>(id));
        let Some(mut current) = draft.take() else {
            if let BrowserOpenResult::Opened { handle_id, .. } = completion.result {
                release_browser_handle(handle_id);
            }
            continue;
        };
        if current.active_picker_id != Some(completion.picker_id) {
            if let BrowserOpenResult::Opened { handle_id, .. } = completion.result {
                release_browser_handle(handle_id);
            }
            continue;
        }
        current.busy = false;
        current.active_picker_id = None;
        match completion.result {
            BrowserOpenResult::Opened {
                handle_id,
                display_name,
                bytes,
            } => {
                release_browser_handle(handle_id);
                let (bytes, digest) = bytes.into_parts();
                match validate_restore_file(&app.state, current.publication_id, &bytes) {
                    Ok(())
                        if ContentDigest::from_bytes(sha2::Sha256::digest(&bytes).into())
                            == digest =>
                    {
                        current.artifact_name = Some(display_name);
                        current.artifact_bytes = Some(bytes);
                        current.error = None;
                    }
                    Ok(()) => {
                        current.error = Some(
                            "The browser read the file inconsistently; the project is unchanged."
                                .to_owned(),
                        );
                    }
                    Err(error) => {
                        current.error = Some(error);
                    }
                }
            }
            BrowserOpenResult::Cancelled => {
                current.error = Some("No file was chosen; the project is unchanged.".to_owned());
            }
            BrowserOpenResult::Failed(error) => {
                current.error = Some(format!("The browser could not open the file: {error}"));
            }
        }
        ctx.data_mut(|data| data.insert_temp(id, current));
    }
}

/// Start writing the checkpoint a browser restore takes first; the restore
/// runs when it is written.
#[cfg(target_arch = "wasm32")]
fn start_browser_restore_checkpoint(
    ctx: &Context,
    app: &RSpiceApp,
    draft: &RestoreDraft,
) -> Result<(), String> {
    let publication_id = draft.publication_id;
    let repaint = ctx.clone();
    rspice_app_services::workbench::lifecycle::project_checkpoint::start_create(
        &app.state,
        ProjectCheckpointReason::PublicationRestore,
        move |result| {
            BROWSER_RESTORE_CHECKPOINTS.with(|queue| {
                queue.borrow_mut().push_back(BrowserRestoreCheckpoint {
                    publication_id,
                    result,
                });
            });
            repaint.request_repaint();
        },
    )
}

#[cfg(target_arch = "wasm32")]
fn poll_browser_restore_checkpoints(ctx: &Context, app: &mut RSpiceApp) {
    let completions =
        BROWSER_RESTORE_CHECKPOINTS.with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    for completion in completions {
        let id = egui::Id::new(RESTORE_DIALOG_ID);
        let Some(mut draft) = ctx.data(|data| data.get_temp::<RestoreDraft>(id)) else {
            continue;
        };
        if !draft.busy || draft.publication_id != completion.publication_id {
            continue;
        }
        draft.busy = false;
        app.state.project.project_checkpoint_recovery.invalidate();
        let restored = completion
            .result
            .map_err(|error| {
                format!(
                    "The project was not restored: its checkpoint could not be taken ({error})."
                )
            })
            .and_then(|checkpoint| {
                let bytes = draft
                    .artifact_bytes
                    .clone()
                    .ok_or_else(|| "Choose the published file.".to_owned())?;
                restore_after_checkpoint(app, &draft, &bytes, checkpoint.project_revision())
            });
        match restored {
            Ok(message) => {
                app.state
                    .record(LogSeverity::Info, LogSource::Studio, message);
                ctx.data_mut(|data| {
                    data.remove::<RestoreDraft>(id);
                });
            }
            Err(error) => {
                draft.error = Some(error);
                clear_restore_file(&mut draft);
                ctx.data_mut(|data| data.insert_temp(id, draft));
            }
        }
    }
}

fn publication_lifecycle_error(state: &AppState) -> Option<&'static str> {
    PublicationReadiness::of(&state.model, state.platform.project_lifecycle.is_open())
        .publish_blocker()
}

fn restore_lifecycle_error(state: &AppState) -> Option<&'static str> {
    PublicationReadiness::of(&state.model, state.platform.project_lifecycle.is_open())
        .restore_blocker()
}

/// Whether `bytes` are the file publication `publication_id` recorded.
pub(crate) fn validate_restore_file(
    state: &AppState,
    publication_id: uuid::Uuid,
    bytes: &[u8],
) -> Result<(), String> {
    use sha2::Digest as _;

    let receipt = state
        .model
        .workspace
        .content
        .project
        .library_publications()
        .iter()
        .find(|receipt| receipt.publication_id() == publication_id)
        .ok_or_else(|| "The project no longer has that publication.".to_owned())?;
    let digest = ContentDigest::from_bytes(sha2::Sha256::digest(bytes).into());
    if bytes.len() as u64 != receipt.snapshot_byte_len() || digest != receipt.snapshot_digest() {
        return Err(format!(
            "The chosen file is not the file published as #{:04}.",
            receipt.sequence()
        ));
    }
    Ok(())
}

pub(crate) fn publication_filename(state: &AppState, label: &str) -> String {
    let project = safe_file_stem(state.model.workspace.content.project.name());
    let label = safe_file_stem(label);
    format!("{project}-{label}.{PROJECT_ARTIFACT_EXTENSION}")
}

/// Prepare the exact artifact and receipt candidate without changing live
/// project state. A native, browser, or repository writer must durably
/// publish `artifact_bytes()` before commit.
pub(crate) fn prepare_project_library_publication(
    state: &AppState,
    label: impl Into<String>,
    actor_id: impl Into<String>,
    authority_id: impl Into<String>,
    reason: impl Into<String>,
) -> Result<ProjectLibraryPublicationCandidate, String> {
    use sha2::Digest as _;

    if state.model.safe_mode.project_read_only() {
        return Err("The project is open read-only.".to_owned());
    }
    if state.model.simulation.execution.has_active_execution() {
        return Err("A simulation is running.".to_owned());
    }
    let snapshot = rspice_app_services::workbench::lifecycle::project_lifecycle::snapshot(state)
        .map_err(|error| format!("The project could not be read for publishing: {error}"))?;
    let serialized = rspice_app_services::io::project_io::serialize_project_file(&snapshot)
        .map_err(|error| format!("The project could not be written for publishing: {error}"))?;
    let bytes = serialized.into_bytes();
    let snapshot_byte_len = u64::try_from(bytes.len())
        .map_err(|_| "The project is too large to publish.".to_owned())?;
    let draft = rspice_app_state::state::workspace::ProjectLibraryPublicationDraft {
        publication_id: uuid::Uuid::new_v4(),
        label: label.into(),
        actor_id: actor_id.into(),
        authority_id: authority_id.into(),
        reason: reason.into(),
        created_unix_ms: rspice_app_state::time_compat::unix_epoch()
            .as_millis()
            .try_into()
            .unwrap_or(u64::MAX)
            .max(1),
        library_revision: state.model.library_manager.revision(),
        snapshot_digest: rspice_app_state::product::ContentDigest::from_bytes(
            sha2::Sha256::digest(&bytes).into(),
        ),
        snapshot_byte_len,
    };
    let mut descriptor_preflight = state.model.workspace.content.project.clone();
    descriptor_preflight
        .publish_library_snapshot(draft.clone())
        .map_err(|error| format!("The publication cannot be recorded: {error}"))?;
    Ok(ProjectLibraryPublicationCandidate {
        draft,
        artifact_bytes: bytes,
        source_project_revision: state.model.workspace.content.project.revision(),
    })
}

/// Commit a publication only after its exact artifact was durably
/// accepted. Any intervening project or catalog change rejects the
/// candidate and leaves live state untouched.
pub(crate) fn commit_project_library_publication(
    state: &mut AppState,
    candidate: ProjectLibraryPublicationCandidate,
) -> Result<ProjectLibraryPublicationReceipt, String> {
    use sha2::Digest as _;

    if state.model.safe_mode.project_read_only() {
        return Err("The project is open read-only.".to_owned());
    }
    if state.model.simulation.execution.has_active_execution() {
        return Err("A simulation is running.".to_owned());
    }
    if state.model.workspace.content.project.revision() != candidate.source_project_revision
        || state.model.library_manager.revision() != candidate.draft.library_revision
    {
        return Err(
            "The project changed while it was being published; publish it again.".to_owned(),
        );
    }
    let current_snapshot =
        rspice_app_services::workbench::lifecycle::project_lifecycle::snapshot(state)
            .map_err(|error| format!("The project could not be read again: {error}"))?;
    let current_serialized =
        rspice_app_services::io::project_io::serialize_project_file(&current_snapshot)
            .map_err(|error| format!("The project could not be written again: {error}"))?;
    let current_bytes = current_serialized.as_bytes();
    if current_bytes.len() as u64 != candidate.draft.snapshot_byte_len
        || rspice_app_state::product::ContentDigest::from_bytes(
            sha2::Sha256::digest(current_bytes).into(),
        ) != candidate.draft.snapshot_digest
    {
        return Err(
            "The project changed while it was being published; publish it again.".to_owned(),
        );
    }
    let receipt = state
        .model
        .workspace
        .content
        .project
        .publish_library_snapshot(candidate.draft)
        .map_err(|error| format!("The publication could not be recorded: {error}"))?;
    state.model.workspace.content.project_metadata_dirty = true;
    state.model.design_execution_epoch = state.model.design_execution_epoch.wrapping_add(1);
    state
        .model
        .netlist_artifacts
        .current_generation_input_digest = None;
    state.model.clear_project_design_history();
    Ok(receipt)
}

/// Replace the open project with the file a publication recorded, keeping
/// the project's identity, its publications and the audit history recorded
/// since. The restore is one new revision; a damaged, altered or foreign
/// file, or one from before a technology change, leaves the project as it
/// was.
pub(crate) fn restore_project_from_publication(
    state: &mut AppState,
    publication_id: uuid::Uuid,
    artifact_bytes: &[u8],
    actor_id: impl Into<String>,
    authority_id: impl Into<String>,
    reason: impl Into<String>,
) -> Result<(), String> {
    use sha2::Digest as _;

    if state.model.safe_mode.project_read_only() {
        return Err("The project is open read-only.".to_owned());
    }
    if state.model.simulation.execution.has_active_execution() {
        return Err("A simulation is running.".to_owned());
    }
    let receipt = state
        .model
        .workspace
        .content
        .project
        .library_publications()
        .iter()
        .find(|receipt| receipt.publication_id() == publication_id)
        .cloned()
        .ok_or_else(|| "The project no longer has that publication.".to_owned())?;
    if artifact_bytes.len() as u64 != receipt.snapshot_byte_len()
        || rspice_app_state::product::ContentDigest::from_bytes(
            sha2::Sha256::digest(artifact_bytes).into(),
        ) != receipt.snapshot_digest()
    {
        return Err(format!(
            "The chosen file is not the file published as #{:04}.",
            receipt.sequence()
        ));
    }
    let artifact_text = std::str::from_utf8(artifact_bytes)
        .map_err(|error| format!("The published file is not a project file: {error}"))?;
    let mut artifact = rspice_app_services::io::project_io::load_project_text(artifact_text, None)
        .map_err(|error| format!("The published file cannot be read: {error}"))?;
    if artifact.file.workspace.project.id() != receipt.project_id()
        || artifact.file.workspace.project.revision() != receipt.source_project_revision()
        || artifact.file.libraries.revision() != receipt.library_revision()
    {
        return Err("The published file belongs to another project or revision.".to_owned());
    }
    let expected_prior_publications = usize::try_from(receipt.sequence() - 1)
        .map_err(|_| "The publication's number is not valid.".to_owned())?;
    if artifact.file.workspace.project.library_publications().len() != expected_prior_publications
        || artifact
            .file
            .workspace
            .project
            .library_publications()
            .last()
            .map(ProjectLibraryPublicationReceipt::receipt_digest)
            != receipt.previous_receipt_digest()
    {
        return Err("The published file lacks the publications recorded before it.".to_owned());
    }
    if artifact.file.workspace.project.technology_binding()
        != state.model.workspace.content.project.technology_binding()
    {
        return Err(
            "The project's technology changed after this publication; a restore cannot undo that."
                .to_owned(),
        );
    }

    let mutation = rspice_app_state::state::ProjectLibraryMutation::RollbackPublication {
        publication_id,
        publication_label: receipt.label().to_owned(),
        snapshot_digest: receipt.snapshot_digest(),
        actor_id: actor_id.into(),
        authority_id: authority_id.into(),
        reason: reason.into(),
    };
    let prepared = state.model.preflight_project_library_mutation(mutation)?;

    let (simulation_plan, model_library_manager, execution_warnings) =
        match artifact.file.execution_context.take() {
            Some(context) => rspice_app_services::io::restore_execution_context(context).map_err(|error| {
                format!("The published simulation setup cannot be read: {error}")
            })?,
            None => (
                rspice_app_state::workbench::app_state::sim_setup::SimSetupState::new_with_user_preferences(
                    &state.model.preferences,
                ),
                rspice_app_state::workbench::workspace::default_model_library_manager(),
                vec![
                    "The publication has no simulation setup; the default setup was restored"
                        .to_owned(),
                ],
            ),
        };

    let mut candidate = state.clone();
    let mut current_project = candidate.model.workspace.content.project.clone();
    current_project.root_library = artifact.file.workspace.project.root_library.clone();
    current_project.top_cell = artifact.file.workspace.project.top_cell.clone();
    artifact.file.workspace.project = current_project;
    candidate.clear_design_execution_context();
    candidate
        .model
        .library_manager
        .replace_catalog_from_snapshot(&artifact.file.libraries)?;
    candidate.model.workspace = rspice_app_state::state::ProjectWorkspace::from_parts(
        artifact.file.workspace,
        artifact.workspace_session,
    );
    candidate.model.sim_setup = simulation_plan;
    candidate.model.model_library_manager = model_library_manager;
    candidate.restore_active_schematic_from_workspace();
    candidate.model.simulation = rspice_app_state::state::SimulationState::default();
    rspice_app_services::io::restore_simulation_results(
        artifact.file.simulation_results,
        &mut candidate.model.simulation,
    )
    .map_err(|error| format!("The published results cannot be read: {error}"))?;
    candidate.model.publish_project_library_mutation(prepared);
    candidate
        .model
        .workspace
        .content
        .project
        .validate()
        .map_err(|error| format!("The restored project is not valid: {error}"))?;
    state.commit_candidate(candidate);
    for warning in execution_warnings {
        state.record(
            rspice_app_state::diagnostics::LogSeverity::Warning,
            rspice_app_state::diagnostics::LogSource::Studio,
            warning,
        );
    }
    Ok(())
}
