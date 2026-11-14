//! The library publication dialogs' composition over the whole
//! application, and the publication transactions they run.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::product::ContentDigest;
use crate::state::workspace::ProjectLibraryPublicationReceipt;
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::design_system::{property_card, property_row};
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::surfaces::project::library_publication::PublicationAction;
#[cfg(target_arch = "wasm32")]
use crate::workbench::surfaces::project::library_publication::{
    BROWSER_PUBLICATION_COMPLETIONS, BROWSER_ROLLBACK_COMPLETIONS, BrowserPublicationCompletion,
    start_browser_rollback_picker,
};
use crate::workbench::surfaces::project::library_publication::{
    PROJECT_ARTIFACT_EXTENSION, PUBLICATION_DIALOG_ID, ProjectLibraryPublicationCandidate,
    PublicationDraft, ROLLBACK_DIALOG_ID, RollbackDraft, clear_rollback_artifact,
    publication_fields, rollback_fields, rollback_receipt_selector, safe_file_stem,
    selected_receipt, transaction_status,
};
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::workflows::export_workflow::SaveDialogConfig;
use egui::Context;

pub(crate) fn show(ctx: &Context, app: &mut RSpiceApp) {
    #[cfg(target_arch = "wasm32")]
    {
        poll_browser_publication_completions(ctx, app);
        poll_browser_rollback_completions(ctx, app);
    }
    show_publication_dialog(ctx, app);
    show_rollback_dialog(ctx, app);
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
    let lifecycle_error = publication_lifecycle_error(&app.state, stale);
    let field_error = draft.field_error();
    let primary_enabled =
        !draft.busy && lifecycle_error.is_none() && field_error.is_none() && !stale;
    let choice = Dialog::new(
        "Publish project library",
        if draft.busy {
            "Publishing\u{2026}"
        } else {
            "Publish durable artifact"
        },
    )
    .description(
        "Freeze the validated project and library catalog, write that copy to disk, then record the publication in the project's history.",
    )
    .size(DialogSize::Manager)
    .secondary("Cancel")
    .primary_enabled(primary_enabled)
    .show(ctx, |ui| {
        property_card(ui, "Publication boundary", |ui| {
            property_row(
                ui,
                "Project",
                &format!(
                    "{} \u{00b7} {}",
                    app.state.model.workspace.content.project.display_name(),
                    app.state.model.workspace.content.project.id()
                ),
            );
            property_row(
                ui,
                "Captured revisions",
                &format!(
                    "project {} \u{00b7} library {}",
                    draft.source_project_revision, draft.source_library_revision
                ),
            );
            property_row(
                ui,
                "Writer contract",
                if cfg!(target_arch = "wasm32") {
                    "File System Access write + read-back check"
                } else {
                    "atomic synchronized file publication"
                },
            );
        });
        ui.add_space(8.0);
        publication_fields(ui, &mut draft);
        ui.add_space(8.0);
        property_card(ui, "Authority declaration", |ui| {
            property_row(
                ui,
                "Identity scope",
                "entered by the operator; no organization identity is assumed",
            );
            property_row(
                ui,
                "Commit order",
                "prepare file \u{2192} write to disk \u{2192} record publication",
            );
            property_row(
                ui,
                "Failure behavior",
                "cancelled, failed, stale, or download-only writes append no receipt",
            );
        });
        if stale {
            transaction_status(
                ui,
                "The project or library catalog changed after this review opened. Cancel and reopen publication.",
                true,
            );
        } else if let Some(error) = draft.error.as_deref().or(lifecycle_error).or(field_error) {
            transaction_status(ui, error, true);
        } else if draft.busy {
            transaction_status(
                ui,
                "Waiting for the browser writer. Keep this project open.",
                false,
            );
        }
    });

    match choice {
        DialogChoice::Primary if primary_enabled => {
            draft.error = None;
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
                    draft.error = Some(
                        "Publication was cancelled; no artifact or receipt was committed."
                            .to_owned(),
                    );
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

fn show_rollback_dialog(ctx: &Context, app: &mut RSpiceApp) {
    let id = egui::Id::new(ROLLBACK_DIALOG_ID);
    let Some(mut draft) = ctx.data(|data| data.get_temp::<RollbackDraft>(id)) else {
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
            data.remove::<RollbackDraft>(id);
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
        clear_rollback_artifact(&mut draft);
    }
    let lifecycle_error = rollback_lifecycle_error(&app.state);
    let field_error = draft.field_error();
    let primary_enabled = !draft.busy && lifecycle_error.is_none() && field_error.is_none();
    let mut choose_artifact = false;
    let choice = Dialog::new("Rollback library publication", "Restore publication")
    .description(
        "Restore one saved full-project artifact. Its digest, size, project identity, revisions, publication history, technology binding, locks, and audit record are checked again before it replaces the project.",
    )
    .size(DialogSize::Manager)
    .secondary("Cancel")
    .primary_enabled(primary_enabled)
    .destructive()
    .show(ctx, |ui| {
        rollback_receipt_selector(ui, &receipts, &mut draft);
        ui.add_space(8.0);
        rollback_fields(ui, &mut draft);
        ui.add_space(8.0);
        property_card(ui, "Artifact", |ui| {
            let selected = selected_receipt(&receipts, draft.publication_id)
                .expect("selected publication remains retained");
            property_row(
                ui,
                "Expected",
                &format!(
                    "{} bytes \u{00b7} {}",
                    selected.snapshot_byte_len(),
                    selected.snapshot_digest()
                ),
            );
            property_row(
                ui,
                "Selected file",
                draft.artifact_name.as_deref().unwrap_or("none"),
            );
            let choose = Button::new(if draft.artifact_bytes.is_some() {
                "Replace artifact\u{2026}"
            } else {
                "Choose artifact\u{2026}"
            })
            .enabled(!draft.busy)
            .show(ui);
            if choose.clicked() {
                choose_artifact = true;
            }
        });
        if let Some(error) = draft.error.as_deref().or(lifecycle_error).or(field_error) {
            transaction_status(ui, error, true);
        } else if draft.busy {
            transaction_status(ui, "Waiting for the browser file picker.", false);
        }
    });

    if choose_artifact {
        draft.error = None;
        #[cfg(not(target_arch = "wasm32"))]
        match pick_native_rollback_artifact(app, &draft) {
            Ok(Some((name, bytes))) => {
                draft.artifact_name = Some(name);
                draft.artifact_bytes = Some(bytes);
            }
            Ok(None) => {
                draft.error =
                    Some("Artifact selection was cancelled; no project state changed.".to_owned());
            }
            Err(error) => draft.error = Some(error),
        }
        #[cfg(target_arch = "wasm32")]
        match start_browser_rollback_picker(ctx) {
            Ok(picker_id) => {
                draft.busy = true;
                draft.active_picker_id = Some(picker_id);
            }
            Err(error) => draft.error = Some(error),
        }
    }

    match choice {
        DialogChoice::Primary if primary_enabled => {
            let bytes = draft
                .artifact_bytes
                .as_deref()
                .expect("enabled rollback owns validated artifact");
            match rollback_project_library_publication(
                &mut app.state,
                draft.publication_id,
                bytes,
                draft.actor_id.clone(),
                draft.authority_id.clone(),
                draft.reason.clone(),
            ) {
                Ok(()) => {
                    app.state.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        format!(
                            "Rolled the project library back to {}; the rollback is in the audit history.",
                            draft
                                .artifact_name
                                .as_deref()
                                .unwrap_or("the chosen publication")
                        ),
                    );
                    ctx.data_mut(|data| {
                        data.remove::<RollbackDraft>(id);
                    });
                    return;
                }
                Err(error) => {
                    draft.error = Some(error);
                    clear_rollback_artifact(&mut draft);
                }
            }
        }
        DialogChoice::Secondary | DialogChoice::Cancelled => {
            ctx.data_mut(|data| {
                data.remove::<RollbackDraft>(id);
            });
            return;
        }
        DialogChoice::None | DialogChoice::Ghost | DialogChoice::Primary => {}
    }
    ctx.data_mut(|data| data.insert_temp(id, draft));
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn publish_native(
    app: &mut RSpiceApp,
    draft: &PublicationDraft,
) -> Result<PublicationAction, String> {
    if !app.export_workflow_io.saved_paths_are_reopenable() {
        return Err(
            "The active writer cannot prove a durable reopenable publication; no receipt was committed."
                .to_owned(),
        );
    }
    let default_name = publication_filename(&app.state, &draft.label);
    let Some(mut path) = app.export_workflow_io.show_save_dialog(SaveDialogConfig {
        title: "Publish project library",
        default_name: &default_name,
        filter_name: "RSpice project publication",
        filter_extensions: &[PROJECT_ARTIFACT_EXTENSION],
    })?
    else {
        return Ok(PublicationAction::Cancelled);
    };
    crate::io::file_exchange::ensure_file_extension(&mut path, PROJECT_ARTIFACT_EXTENSION);
    let destination = app.export_workflow_io.observe_destination(&path)?;
    let candidate = prepare_project_library_publication(
        &app.state,
        draft.label.clone(),
        draft.actor_id.clone(),
        draft.authority_id.clone(),
        draft.reason.clone(),
    )?;
    app.export_workflow_io.write_bytes_file_observed(
        &destination,
        candidate.artifact_bytes(),
        "application/vnd.rspice.project+json",
    )?;
    let receipt = commit_project_library_publication(&mut app.state, candidate)?;
    Ok(PublicationAction::Published(format!(
        "Published the project library to {} (publication #{:04}, {} bytes).",
        destination.path().display(),
        receipt.sequence(),
        receipt.snapshot_byte_len()
    )))
}

#[cfg(target_arch = "wasm32")]
fn start_browser_publication(
    ctx: &Context,
    app: &RSpiceApp,
    draft: &PublicationDraft,
) -> Result<uuid::Uuid, String> {
    use crate::workbench::lifecycle::project_lifecycle::{
        BrowserBindingBackend, BrowserWriteTarget, start_browser_write,
    };

    let candidate = prepare_project_library_publication(
        &app.state,
        draft.label.clone(),
        draft.actor_id.clone(),
        draft.authority_id.clone(),
        draft.reason.clone(),
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
    use crate::workbench::lifecycle::project_lifecycle::{
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
                        "The published file did not read back as written; no receipt was committed."
                            .to_owned(),
                    )
                } else {
                    commit_project_library_publication(&mut app.state, completion.candidate)
                        .map(|receipt| {
                            format!(
                                "Published the project library to {display_name} (publication #{:04}, {} bytes).",
                                receipt.sequence(),
                                receipt.snapshot_byte_len()
                            )
                        })
                }
            }
            BrowserWriteResult::Cancelled => {
                Err("Publication was cancelled; no receipt was committed.".to_owned())
            }
            BrowserWriteResult::ExternalChange { .. } => Err(
                "The chosen publication file changed before it was committed; no receipt was committed."
                    .to_owned(),
            ),
            BrowserWriteResult::Failed(error) => Err(format!(
                "Publishing the project library failed: {error}. No receipt was committed."
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
fn pick_native_rollback_artifact(
    app: &RSpiceApp,
    draft: &RollbackDraft,
) -> Result<Option<(String, Vec<u8>)>, String> {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("RSpice project publication", &[PROJECT_ARTIFACT_EXTENSION])
        .set_title("Choose library publication artifact")
        .pick_file()
    else {
        return Ok(None);
    };
    let metadata = std::fs::metadata(&path).map_err(|error| {
        format!(
            "Could not inspect publication artifact '{}': {error}",
            path.display()
        )
    })?;
    if metadata.len() > crate::io::project_io::MAX_PROJECT_FILE_BYTES {
        return Err(format!(
            "Publication artifact '{}' exceeds the supported {}-byte project limit",
            path.display(),
            crate::io::project_io::MAX_PROJECT_FILE_BYTES
        ));
    }
    let bytes = std::fs::read(&path).map_err(|error| {
        format!(
            "Could not read publication artifact '{}': {error}",
            path.display()
        )
    })?;
    validate_rollback_artifact(&app.state, draft.publication_id, &bytes)?;
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .map(str::to_owned)
        .unwrap_or_else(|| path.display().to_string());
    Ok(Some((name, bytes)))
}

#[cfg(target_arch = "wasm32")]
fn poll_browser_rollback_completions(ctx: &Context, app: &mut RSpiceApp) {
    use crate::workbench::lifecycle::project_lifecycle::{
        BrowserOpenResult, release_browser_handle,
    };
    use sha2::Digest as _;

    let completions =
        BROWSER_ROLLBACK_COMPLETIONS.with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    for completion in completions {
        let id = egui::Id::new(ROLLBACK_DIALOG_ID);
        let mut draft = ctx.data(|data| data.get_temp::<RollbackDraft>(id));
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
                match validate_rollback_artifact(&app.state, current.publication_id, &bytes) {
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
                            "Browser artifact read digest was internally inconsistent; no rollback is allowed."
                                .to_owned(),
                        );
                    }
                    Err(error) => {
                        current.error = Some(error);
                    }
                }
            }
            BrowserOpenResult::Cancelled => {
                current.error =
                    Some("Artifact selection was cancelled; no project state changed.".to_owned());
            }
            BrowserOpenResult::Failed(error) => {
                current.error = Some(format!("Browser artifact selection failed: {error}"));
            }
        }
        ctx.data_mut(|data| data.insert_temp(id, current));
    }
}

fn publication_lifecycle_error(state: &AppState, stale: bool) -> Option<&'static str> {
    if !state.platform.project_lifecycle.is_open() {
        Some("Open a project before publishing its library.")
    } else if state.model.safe_mode.project_read_only() {
        Some("Publication is unavailable while the project is open read-only.")
    } else if state.model.simulation.execution.has_active_execution() {
        Some("Publication is unavailable while a simulation is running.")
    } else if stale {
        Some("The publication review is stale.")
    } else {
        None
    }
}

fn rollback_lifecycle_error(state: &AppState) -> Option<&'static str> {
    if !state.platform.project_lifecycle.is_open() {
        Some("Open the publication's project before rollback.")
    } else if state.model.safe_mode.project_read_only() {
        Some("Rollback is unavailable while the project is open read-only.")
    } else if state.model.simulation.execution.has_active_execution() {
        Some("Rollback is unavailable while a simulation is running.")
    } else {
        None
    }
}

pub(crate) fn validate_rollback_artifact(
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
        .ok_or_else(|| format!("Publication {publication_id} is no longer retained"))?;
    let digest = ContentDigest::from_bytes(sha2::Sha256::digest(bytes).into());
    if bytes.len() as u64 != receipt.snapshot_byte_len() || digest != receipt.snapshot_digest() {
        return Err(format!(
            "Selected artifact does not match publication #{:04}: expected {} bytes \u{00b7} {}, observed {} bytes \u{00b7} {}.",
            receipt.sequence(),
            receipt.snapshot_byte_len(),
            receipt.snapshot_digest(),
            bytes.len(),
            digest
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
        return Err(
            "project library publication is unavailable because the project is open read-only"
                .to_owned(),
        );
    }
    if state.model.simulation.execution.has_active_execution() {
        return Err(
            "project library publication is unavailable while a simulation is running".to_owned(),
        );
    }
    let snapshot = crate::workbench::lifecycle::project_lifecycle::snapshot(state)
        .map_err(|error| format!("project library publication snapshot failed: {error}"))?;
    let serialized = crate::io::project_io::serialize_project_file(&snapshot)
        .map_err(|error| format!("project library publication serialization failed: {error}"))?;
    let bytes = serialized.into_bytes();
    let snapshot_byte_len = u64::try_from(bytes.len())
        .map_err(|_| "project library publication artifact is too large".to_owned())?;
    let draft = crate::state::workspace::ProjectLibraryPublicationDraft {
        publication_id: uuid::Uuid::new_v4(),
        label: label.into(),
        actor_id: actor_id.into(),
        authority_id: authority_id.into(),
        reason: reason.into(),
        created_unix_ms: crate::time_compat::unix_epoch()
            .as_millis()
            .try_into()
            .unwrap_or(u64::MAX)
            .max(1),
        library_revision: state.model.library_manager.revision(),
        snapshot_digest: crate::product::ContentDigest::from_bytes(
            sha2::Sha256::digest(&bytes).into(),
        ),
        snapshot_byte_len,
    };
    let mut descriptor_preflight = state.model.workspace.content.project.clone();
    descriptor_preflight
        .publish_library_snapshot(draft.clone())
        .map_err(|error| format!("project library publication preflight failed: {error}"))?;
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
        return Err(
            "project library publication is unavailable because the project is open read-only"
                .to_owned(),
        );
    }
    if state.model.simulation.execution.has_active_execution() {
        return Err(
            "project library publication is unavailable while a simulation is running".to_owned(),
        );
    }
    if state.model.workspace.content.project.revision() != candidate.source_project_revision
        || state.model.library_manager.revision() != candidate.draft.library_revision
    {
        return Err(
            "project library publication candidate is stale; prepare and publish a new artifact"
                .to_owned(),
        );
    }
    let current_snapshot = crate::workbench::lifecycle::project_lifecycle::snapshot(state)
        .map_err(|error| format!("project library publication recheck failed: {error}"))?;
    let current_serialized = crate::io::project_io::serialize_project_file(&current_snapshot)
        .map_err(|error| {
            format!("project library publication recheck serialization failed: {error}")
        })?;
    let current_bytes = current_serialized.as_bytes();
    if current_bytes.len() as u64 != candidate.draft.snapshot_byte_len
        || crate::product::ContentDigest::from_bytes(sha2::Sha256::digest(current_bytes).into())
            != candidate.draft.snapshot_digest
    {
        return Err(
            "project library publication content changed after the artifact was prepared"
                .to_owned(),
        );
    }
    let receipt = state
        .model
        .workspace
        .content
        .project
        .publish_library_snapshot(candidate.draft)
        .map_err(|error| format!("project library publication failed: {error}"))?;
    state.model.workspace.content.project_metadata_dirty = true;
    state.model.design_execution_epoch = state.model.design_execution_epoch.wrapping_add(1);
    state
        .model
        .netlist_artifacts
        .current_generation_input_digest = None;
    state.model.clear_project_design_history();
    Ok(receipt)
}

/// Restore the exact complete project artifact named by an immutable
/// library publication while preserving the current project identity,
/// publication ledger, and intervening audit history. The rollback is one
/// new revision; malformed, tampered, foreign, stale-authority, or
/// technology-incompatible artifacts leave live state unchanged.
pub(crate) fn rollback_project_library_publication(
    state: &mut AppState,
    publication_id: uuid::Uuid,
    artifact_bytes: &[u8],
    actor_id: impl Into<String>,
    authority_id: impl Into<String>,
    reason: impl Into<String>,
) -> Result<(), String> {
    use sha2::Digest as _;

    if state.model.safe_mode.project_read_only() {
        return Err(
            "project library rollback is unavailable because the project is open read-only"
                .to_owned(),
        );
    }
    if state.model.simulation.execution.has_active_execution() {
        return Err(
            "project library rollback is unavailable while a simulation is running".to_owned(),
        );
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
        .ok_or_else(|| format!("project library publication {publication_id} is not retained"))?;
    if artifact_bytes.len() as u64 != receipt.snapshot_byte_len()
        || crate::product::ContentDigest::from_bytes(sha2::Sha256::digest(artifact_bytes).into())
            != receipt.snapshot_digest()
    {
        return Err(
            "project library rollback artifact does not match its publication receipt".to_owned(),
        );
    }
    let artifact_text = std::str::from_utf8(artifact_bytes)
        .map_err(|error| format!("project library rollback artifact is not UTF-8: {error}"))?;
    let mut artifact = crate::io::project_io::load_project_text(artifact_text, None)
        .map_err(|error| format!("project library rollback artifact is invalid: {error}"))?;
    if artifact.file.workspace.project.id() != receipt.project_id()
        || artifact.file.workspace.project.revision() != receipt.source_project_revision()
        || artifact.file.libraries.revision() != receipt.library_revision()
    {
        return Err(
            "project library rollback artifact identity or revision does not match its receipt"
                .to_owned(),
        );
    }
    let expected_prior_publications = usize::try_from(receipt.sequence() - 1)
        .map_err(|_| "project library publication sequence is invalid".to_owned())?;
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
        return Err(
            "project library rollback artifact is missing publications recorded before it"
                .to_owned(),
        );
    }
    if artifact.file.workspace.project.technology_binding()
        != state.model.workspace.content.project.technology_binding()
    {
        return Err(
            "project library rollback cannot undo a change to the project's technology binding"
                .to_owned(),
        );
    }

    let mutation = crate::state::ProjectLibraryMutation::RollbackPublication {
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
            Some(context) => crate::io::restore_execution_context(context).map_err(|error| {
                format!("project library rollback execution context is invalid: {error}")
            })?,
            None => (
                crate::workbench::app_state::SimSetupState::new_with_user_preferences(
                    &state.model.preferences,
                ),
                crate::workbench::app_state::default_model_library_manager(),
                vec![
                    "The publication predates durable simulation plans; documented defaults were restored"
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
    candidate.model.library_edit_locks = crate::state::ProjectLibraryLockAuthority::default();
    candidate.model.workspace = crate::state::ProjectWorkspace::from_parts(
        artifact.file.workspace,
        artifact.workspace_session,
    );
    candidate.model.sim_setup = simulation_plan;
    candidate.model.model_library_manager = model_library_manager;
    candidate.restore_active_schematic_from_workspace();
    candidate.model.simulation = crate::state::SimulationState::default();
    crate::io::restore_simulation_results(
        artifact.file.simulation_results,
        &mut candidate.model.simulation,
    )
    .map_err(|error| format!("project library rollback result history is invalid: {error}"))?;
    candidate.model.publish_project_library_mutation(prepared);
    candidate
        .model
        .workspace
        .content
        .project
        .validate()
        .map_err(|error| format!("project library rollback metadata is invalid: {error}"))?;
    state.commit_candidate(candidate);
    for warning in execution_warnings {
        state.record(
            crate::diagnostics::LogSeverity::Warning,
            crate::diagnostics::LogSource::Studio,
            warning,
        );
    }
    Ok(())
}
