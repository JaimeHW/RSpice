//! End-user project-library publication and rollback transactions.
//!
//! A publication receipt is committed only after the exact prepared project
//! bytes have crossed a writer boundary that can prove a durable native write
//! or a browser File System Access write with read-back verification. Browser
//! downloads are deliberately not treated as durable publication.

use super::*;
use crate::product::ContentDigest;
use crate::state::workspace::ProjectLibraryPublicationReceipt;
use crate::workbench::app_state::AppState;
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::workflows::export_workflow::SaveDialogConfig;

const PUBLICATION_DIALOG_ID: &str = "workbench.project.library.publication";
const ROLLBACK_DIALOG_ID: &str = "workbench.project.library.publication.rollback";
const PROJECT_ARTIFACT_EXTENSION: &str = "rspiceproj";

#[derive(Debug, Clone)]
struct PublicationDraft {
    source_project_revision: u64,
    source_library_revision: u64,
    label: String,
    actor_id: String,
    authority_id: String,
    reason: String,
    busy: bool,
    #[cfg(target_arch = "wasm32")]
    active_publication_id: Option<uuid::Uuid>,
    error: Option<String>,
}

impl PublicationDraft {
    fn new(state: &AppState) -> Self {
        Self {
            source_project_revision: state.workspace.project.revision().get(),
            source_library_revision: state.library_manager.revision(),
            label: format!(
                "{}-library-r{}",
                state.workspace.project.name(),
                state.workspace.project.revision().get()
            ),
            actor_id: String::new(),
            authority_id: String::new(),
            reason: String::new(),
            busy: false,
            #[cfg(target_arch = "wasm32")]
            active_publication_id: None,
            error: None,
        }
    }

    fn field_error(&self) -> Option<&'static str> {
        for (label, value) in [
            ("Publication label", self.label.as_str()),
            ("Actor identity", self.actor_id.as_str()),
            ("Authority identity", self.authority_id.as_str()),
            ("Publication reason", self.reason.as_str()),
        ] {
            if !audit_text_valid(value) {
                return Some(match label {
                    "Publication label" => {
                        "Publication label is required, must be trimmed, and must not exceed 240 characters."
                    }
                    "Actor identity" => {
                        "Actor identity is required, must be trimmed, and must not exceed 240 characters."
                    }
                    "Authority identity" => {
                        "Authority identity is required, must be trimmed, and must not exceed 240 characters."
                    }
                    _ => {
                        "Publication reason is required, must be trimmed, and must not exceed 240 characters."
                    }
                });
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
struct RollbackDraft {
    publication_id: uuid::Uuid,
    actor_id: String,
    authority_id: String,
    reason: String,
    artifact_name: Option<String>,
    artifact_bytes: Option<Vec<u8>>,
    busy: bool,
    #[cfg(target_arch = "wasm32")]
    active_picker_id: Option<uuid::Uuid>,
    error: Option<String>,
}

impl RollbackDraft {
    fn new(receipt: &ProjectLibraryPublicationReceipt) -> Self {
        Self {
            publication_id: receipt.publication_id(),
            actor_id: String::new(),
            authority_id: String::new(),
            reason: String::new(),
            artifact_name: None,
            artifact_bytes: None,
            busy: false,
            #[cfg(target_arch = "wasm32")]
            active_picker_id: None,
            error: None,
        }
    }

    fn field_error(&self) -> Option<&'static str> {
        for (label, value) in [
            ("Actor identity", self.actor_id.as_str()),
            ("Authority identity", self.authority_id.as_str()),
            ("Rollback reason", self.reason.as_str()),
        ] {
            if !audit_text_valid(value) {
                return Some(match label {
                    "Actor identity" => {
                        "Actor identity is required, must be trimmed, and must not exceed 240 characters."
                    }
                    "Authority identity" => {
                        "Authority identity is required, must be trimmed, and must not exceed 240 characters."
                    }
                    _ => {
                        "Rollback reason is required, must be trimmed, and must not exceed 240 characters."
                    }
                });
            }
        }
        if self.artifact_bytes.is_none() {
            return Some("Choose the exact retained publication artifact before rollback.");
        }
        None
    }
}

#[cfg(target_arch = "wasm32")]
struct BrowserPublicationCompletion {
    candidate: crate::ProjectLibraryPublicationCandidate,
    result: crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult,
}

#[cfg(target_arch = "wasm32")]
struct BrowserRollbackCompletion {
    picker_id: uuid::Uuid,
    result: crate::workbench::lifecycle::project_lifecycle::BrowserOpenResult,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_PUBLICATION_COMPLETIONS:
        std::cell::RefCell<std::collections::VecDeque<BrowserPublicationCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
    static BROWSER_ROLLBACK_COMPLETIONS:
        std::cell::RefCell<std::collections::VecDeque<BrowserRollbackCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

pub(super) fn open_publication(ctx: &Context, state: &AppState) {
    ctx.data_mut(|data| {
        data.remove::<RollbackDraft>(egui::Id::new(ROLLBACK_DIALOG_ID));
        data.insert_temp(
            egui::Id::new(PUBLICATION_DIALOG_ID),
            PublicationDraft::new(state),
        );
    });
}

pub(super) fn open_rollback(ctx: &Context, state: &AppState) {
    let Some(receipt) = state.workspace.project.library_publications().last() else {
        return;
    };
    ctx.data_mut(|data| {
        data.remove::<PublicationDraft>(egui::Id::new(PUBLICATION_DIALOG_ID));
        data.insert_temp(
            egui::Id::new(ROLLBACK_DIALOG_ID),
            RollbackDraft::new(receipt),
        );
    });
}

#[cfg(test)]
pub(super) fn publication_dialog_open(ctx: &Context) -> bool {
    ctx.data(|data| {
        data.get_temp::<PublicationDraft>(egui::Id::new(PUBLICATION_DIALOG_ID))
            .is_some()
    })
}

#[cfg(test)]
pub(super) fn rollback_dialog_open(ctx: &Context) -> bool {
    ctx.data(|data| {
        data.get_temp::<RollbackDraft>(egui::Id::new(ROLLBACK_DIALOG_ID))
            .is_some()
    })
}

pub(super) fn show(ctx: &Context, app: &mut RSpiceApp) {
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
    let live_project_revision = app.state.workspace.project.revision().get();
    let live_library_revision = app.state.library_manager.revision();
    let stale = live_project_revision != draft.source_project_revision
        || live_library_revision != draft.source_library_revision;
    let lifecycle_error = publication_lifecycle_error(&app.state, stale);
    let field_error = draft.field_error();
    let primary_enabled =
        !draft.busy && lifecycle_error.is_none() && field_error.is_none() && !stale;
    let choice = Dialog::new(
        "PROJECT \u{00b7} LIBRARY GOVERNANCE",
        "Publish project library",
        if draft.busy {
            "Publishing\u{2026}"
        } else {
            "Publish durable artifact"
        },
    )
    .description(
        "Freeze the complete validated project and library catalog, durably write those exact bytes, then append one immutable content-addressed receipt.",
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
                    app.state.workspace.project.display_name(),
                    app.state.workspace.project.id()
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
                    "File System Access write + exact read-back"
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
                "operator-provided evidence; no organization identity is inferred",
            );
            property_row(
                ui,
                "Commit order",
                "prepare bytes \u{2192} durable write \u{2192} exact receipt",
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
                "The browser writer is waiting for a verified durable result. Keep this project open.",
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
                    app.state.push_user_message(ConsoleMessage::info(message));
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
                Err(error) => {
                    app.state
                        .push_user_message(ConsoleMessage::error(error.clone()));
                    draft.error = Some(error);
                }
            }
            #[cfg(target_arch = "wasm32")]
            match start_browser_publication(ctx, app, &draft) {
                Ok(publication_id) => {
                    draft.busy = true;
                    draft.active_publication_id = Some(publication_id);
                }
                Err(error) => {
                    app.state
                        .push_user_message(ConsoleMessage::error(error.clone()));
                    draft.error = Some(error);
                }
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
        .workspace
        .project
        .library_publications()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
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
    let choice = Dialog::new(
        "PROJECT \u{00b7} LIBRARY GOVERNANCE",
        "Rollback library publication",
        "Restore exact publication",
    )
    .description(
        "Restore one exact retained full-project artifact. Digest, size, project identity, revisions, publication lineage, technology binding, locks, and audit authority are revalidated before replacement.",
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
        property_card(ui, "Exact artifact", |ui| {
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
                "Choose exact artifact\u{2026}"
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
            transaction_status(
                ui,
                "Waiting for the browser file picker. No project state has changed.",
                false,
            );
        } else if draft.artifact_bytes.is_some() {
            transaction_status(
                ui,
                "The chosen artifact matches the retained receipt digest and size. Final rollback validation runs on confirmation.",
                false,
            );
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
            Err(error) => {
                app.state
                    .push_user_message(ConsoleMessage::error(error.clone()));
                draft.error = Some(error);
            }
        }
        #[cfg(target_arch = "wasm32")]
        match start_browser_rollback_picker(ctx) {
            Ok(picker_id) => {
                draft.busy = true;
                draft.active_picker_id = Some(picker_id);
            }
            Err(error) => {
                app.state
                    .push_user_message(ConsoleMessage::error(error.clone()));
                draft.error = Some(error);
            }
        }
    }

    match choice {
        DialogChoice::Primary if primary_enabled => {
            let bytes = draft
                .artifact_bytes
                .as_deref()
                .expect("enabled rollback owns validated artifact");
            match app.rollback_project_library_publication(
                draft.publication_id,
                bytes,
                draft.actor_id.clone(),
                draft.authority_id.clone(),
                draft.reason.clone(),
            ) {
                Ok(()) => {
                    let message = format!(
                        "Library publication {} was restored atomically; a new rollback audit receipt was appended.",
                        draft.publication_id
                    );
                    app.state.push_user_message(ConsoleMessage::info(message));
                    ctx.data_mut(|data| {
                        data.remove::<RollbackDraft>(id);
                    });
                    return;
                }
                Err(error) => {
                    app.state
                        .push_user_message(ConsoleMessage::error(error.clone()));
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

fn publication_fields(ui: &mut Ui, draft: &mut PublicationDraft) {
    property_card(ui, "Receipt fields", |ui| {
        transaction_field(
            ui,
            "workbench.project.library.publication.label",
            "Publication label",
            &mut draft.label,
            "analog-core-1.0.0",
        );
        transaction_field(
            ui,
            "workbench.project.library.publication.actor",
            "Actor identity",
            &mut draft.actor_id,
            "engineer@organization",
        );
        transaction_field(
            ui,
            "workbench.project.library.publication.authority",
            "Authority identity",
            &mut draft.authority_id,
            "release-authority",
        );
        transaction_field(
            ui,
            "workbench.project.library.publication.reason",
            "Reason",
            &mut draft.reason,
            "Qualified handoff",
        );
    });
}

fn rollback_fields(ui: &mut Ui, draft: &mut RollbackDraft) {
    property_card(ui, "Rollback receipt", |ui| {
        transaction_field(
            ui,
            "workbench.project.library.rollback.actor",
            "Actor identity",
            &mut draft.actor_id,
            "engineer@organization",
        );
        transaction_field(
            ui,
            "workbench.project.library.rollback.authority",
            "Authority identity",
            &mut draft.authority_id,
            "release-authority",
        );
        transaction_field(
            ui,
            "workbench.project.library.rollback.reason",
            "Reason",
            &mut draft.reason,
            "Restore qualified handoff",
        );
    });
}

fn transaction_field(
    ui: &mut Ui,
    id: &'static str,
    label: &'static str,
    value: &mut String,
    hint: &'static str,
) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.set_min_height(t.metrics.ctl_h.max(32.0));
        ui.add_sized(
            [142.0, t.metrics.ctl_h],
            egui::Label::new(
                egui::RichText::new(label)
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text_dim),
            ),
        );
        let response = ui.add_sized(
            [ui.available_width().max(180.0), t.metrics.ctl_h],
            egui::TextEdit::singleline(value)
                .id_source(id)
                .hint_text(hint)
                .char_limit(240),
        );
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_label(label);
        });
    });
}

fn rollback_receipt_selector(
    ui: &mut Ui,
    receipts: &[ProjectLibraryPublicationReceipt],
    draft: &mut RollbackDraft,
) {
    property_card(ui, "Retained publication", |ui| {
        let selected = selected_receipt(receipts, draft.publication_id)
            .expect("rollback receipt is normalized before rendering");
        let prior = draft.publication_id;
        egui::ComboBox::from_id_salt("workbench.project.library.rollback.publication")
            .selected_text(format!(
                "#{:04} \u{00b7} {}",
                selected.sequence(),
                selected.label()
            ))
            .width(ui.available_width().max(220.0))
            .show_ui(ui, |ui| {
                for receipt in receipts.iter().rev() {
                    ui.selectable_value(
                        &mut draft.publication_id,
                        receipt.publication_id(),
                        format!("#{:04} \u{00b7} {}", receipt.sequence(), receipt.label()),
                    );
                }
            });
        if draft.publication_id != prior {
            clear_rollback_artifact(draft);
        }
        let selected = selected_receipt(receipts, draft.publication_id)
            .expect("selected rollback receipt remains retained");
        property_row(
            ui,
            "Publication identity",
            &selected.publication_id().to_string(),
        );
        property_row(
            ui,
            "Source revisions",
            &format!(
                "project {} \u{00b7} library {}",
                selected.source_project_revision().get(),
                selected.library_revision()
            ),
        );
        property_row(ui, "Published by", selected.actor_id());
        property_row(ui, "Authority", selected.authority_id());
    });
}

fn selected_receipt(
    receipts: &[ProjectLibraryPublicationReceipt],
    publication_id: uuid::Uuid,
) -> Option<&ProjectLibraryPublicationReceipt> {
    receipts
        .iter()
        .find(|receipt| receipt.publication_id() == publication_id)
}

fn publication_lifecycle_error(state: &AppState, stale: bool) -> Option<&'static str> {
    if !state.project_lifecycle.project_open {
        Some("Open a project before publishing its library.")
    } else if state.workbench.safe_mode.project_read_only() {
        Some("Publication is unavailable while the project is open read-only.")
    } else if state.simulation.is_running {
        Some("Publication is unavailable while a simulation is running.")
    } else if stale {
        Some("The publication review is stale.")
    } else {
        None
    }
}

fn rollback_lifecycle_error(state: &AppState) -> Option<&'static str> {
    if !state.project_lifecycle.project_open {
        Some("Open the publication's project before rollback.")
    } else if state.workbench.safe_mode.project_read_only() {
        Some("Rollback is unavailable while the project is open read-only.")
    } else if state.simulation.is_running {
        Some("Rollback is unavailable while a simulation is running.")
    } else {
        None
    }
}

fn audit_text_valid(value: &str) -> bool {
    !value.is_empty()
        && value == value.trim()
        && value.chars().count() <= 240
        && !value.chars().any(char::is_control)
}

fn transaction_status(ui: &mut Ui, message: &str, error: bool) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(8.0);
    egui::Frame::new()
        .fill(if error {
            t.color.err.gamma_multiply(0.16)
        } else {
            t.color.info.gamma_multiply(0.14)
        })
        .stroke(Stroke::new(
            1.0,
            if error { t.color.err } else { t.color.info },
        ))
        .corner_radius(4.0)
        .inner_margin(Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(message)
                    .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                    .color(if error { t.color.err } else { t.color.text }),
            );
        });
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
enum PublicationAction {
    Published(String),
    Cancelled,
}

#[cfg(not(target_arch = "wasm32"))]
fn publish_native(
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
    crate::workbench::workflows::file_actions::ensure_file_extension(
        &mut path,
        PROJECT_ARTIFACT_EXTENSION,
    );
    let destination = app.export_workflow_io.observe_destination(&path)?;
    let candidate = app.prepare_project_library_publication(
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
    let receipt = app.commit_project_library_publication(candidate)?;
    Ok(PublicationAction::Published(format!(
        "Published library artifact '{}' and committed receipt #{:04} {} ({} bytes \u{00b7} {}).",
        destination.path().display(),
        receipt.sequence(),
        receipt.publication_id(),
        receipt.snapshot_byte_len(),
        receipt.snapshot_digest()
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

    let candidate = app.prepare_project_library_publication(
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
        binding_id: uuid::Uuid::new_v4(),
        backend: BrowserBindingBackend::ExternalFile,
        project_id: app.state.workspace.project.id().to_string(),
        accepted_generation: 1,
        expected_digest: None,
        persisted_generation: None,
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
                        "Browser publication read-back digest did not match the prepared artifact; no receipt was committed."
                            .to_owned(),
                    )
                } else {
                    app.commit_project_library_publication(completion.candidate)
                        .map(|receipt| {
                            format!(
                                "Published library artifact '{display_name}' and committed receipt #{:04} {} ({} bytes \u{00b7} {}).",
                                receipt.sequence(),
                                receipt.publication_id(),
                                receipt.snapshot_byte_len(),
                                receipt.snapshot_digest()
                            )
                        })
                }
            }
            BrowserWriteResult::Cancelled => {
                Err("Publication was cancelled; no receipt was committed.".to_owned())
            }
            BrowserWriteResult::ExternalChange { observed_digest } => Err(format!(
                "The chosen publication file changed before commit (observed {observed_digest}); no receipt was committed."
            )),
            BrowserWriteResult::Failed(error) => Err(format!(
                "Durable browser publication failed: {error}. No receipt was committed."
            )),
        };
        match result {
            Ok(message) => {
                app.state.push_user_message(ConsoleMessage::info(message));
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
                app.state
                    .push_user_message(ConsoleMessage::error(error.clone()));
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
        .set_title("Choose exact library publication artifact")
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
fn start_browser_rollback_picker(ctx: &Context) -> Result<uuid::Uuid, String> {
    use crate::workbench::lifecycle::project_lifecycle::start_browser_open;

    let picker_id = uuid::Uuid::new_v4();
    let repaint = ctx.clone();
    start_browser_open(move |result| {
        BROWSER_ROLLBACK_COMPLETIONS.with(|queue| {
            queue
                .borrow_mut()
                .push_back(BrowserRollbackCompletion { picker_id, result });
        });
        repaint.request_repaint();
    })?;
    Ok(picker_id)
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
                digest,
            } => {
                release_browser_handle(handle_id);
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
        if let Some(error) = current.error.as_ref() {
            app.state
                .push_user_message(ConsoleMessage::error(error.clone()));
        }
        ctx.data_mut(|data| data.insert_temp(id, current));
    }
}

fn validate_rollback_artifact(
    state: &AppState,
    publication_id: uuid::Uuid,
    bytes: &[u8],
) -> Result<(), String> {
    use sha2::Digest as _;

    let receipt = state
        .workspace
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

fn clear_rollback_artifact(draft: &mut RollbackDraft) {
    draft.artifact_name = None;
    draft.artifact_bytes = None;
}

fn publication_filename(state: &AppState, label: &str) -> String {
    let project = safe_file_stem(state.workspace.project.name());
    let label = safe_file_stem(label);
    format!("{project}-{label}.{PROJECT_ARTIFACT_EXTENSION}")
}

fn safe_file_stem(value: &str) -> String {
    let mut output = String::new();
    let mut separator = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') {
            output.push(character);
            separator = false;
        } else if !separator && !output.is_empty() {
            output.push('-');
            separator = true;
        }
    }
    let output = output.trim_matches(['-', '.']).to_owned();
    if output.is_empty() {
        "project-library".to_owned()
    } else {
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(target_arch = "wasm32"))]
    use crate::workbench::workflows::export_workflow::{
        ExportWorkflowIo, ObservedExportDestination,
    };
    #[cfg(not(target_arch = "wasm32"))]
    use std::cell::RefCell;
    #[cfg(not(target_arch = "wasm32"))]
    use std::path::{Path, PathBuf};
    #[cfg(not(target_arch = "wasm32"))]
    use std::rc::Rc;

    #[cfg(not(target_arch = "wasm32"))]
    #[derive(Debug, Clone)]
    struct PublicationIo {
        destination: PathBuf,
        reopenable: bool,
        writes: Rc<RefCell<Vec<(PathBuf, Vec<u8>)>>>,
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl ExportWorkflowIo for PublicationIo {
        fn show_save_dialog(
            &self,
            _config: SaveDialogConfig<'_>,
        ) -> Result<Option<PathBuf>, String> {
            Ok(Some(self.destination.clone()))
        }

        fn write_text_file(&self, _path: &Path, _contents: &str) -> Result<(), String> {
            Err("text export is outside this test".to_owned())
        }

        fn write_bytes_file_observed(
            &self,
            destination: &ObservedExportDestination,
            contents: &[u8],
            _mime_type: &str,
        ) -> Result<(), String> {
            self.writes
                .borrow_mut()
                .push((destination.path().to_path_buf(), contents.to_vec()));
            Ok(())
        }

        fn write_waveform_csv(
            &self,
            _dataset: &crate::io::WaveformDataset,
            _path: &Path,
        ) -> Result<(), String> {
            Err("waveform export is outside this test".to_owned())
        }

        fn saved_paths_are_reopenable(&self) -> bool {
            self.reopenable
        }
    }

    fn complete_publication_draft(app: &RSpiceApp) -> PublicationDraft {
        let mut draft = PublicationDraft::new(&app.state);
        draft.label = "analog-core-1.0.0".to_owned();
        draft.actor_id = "release-engineer@example.test".to_owned();
        draft.authority_id = "organization-release-authority".to_owned();
        draft.reason = "Qualified library handoff".to_owned();
        draft
    }

    #[test]
    fn publication_and_rollback_fields_are_strict_audit_text() {
        assert!(audit_text_valid("release-engineer@example.test"));
        assert!(!audit_text_valid(""));
        assert!(!audit_text_valid(" leading"));
        assert!(!audit_text_valid("trailing "));
        assert!(!audit_text_valid("line\nbreak"));
        assert!(!audit_text_valid(&"x".repeat(241)));

        let app = RSpiceApp::test_instance();
        let mut draft = PublicationDraft::new(&app.state);
        assert!(draft.field_error().is_some());
        draft = complete_publication_draft(&app);
        assert!(draft.field_error().is_none());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_writer_persists_exact_candidate_before_committing_receipt() {
        let mut app = RSpiceApp::test_instance();
        let draft = complete_publication_draft(&app);
        let writes = Rc::new(RefCell::new(Vec::new()));
        app.export_workflow_io = Box::new(PublicationIo {
            destination: PathBuf::from("release").join("analog-core"),
            reopenable: true,
            writes: Rc::clone(&writes),
        });

        let action = publish_native(&mut app, &draft).expect("publication succeeds");
        assert!(matches!(action, PublicationAction::Published(_)));
        let [receipt] = app.state.workspace.project.library_publications() else {
            panic!("exactly one publication receipt must commit");
        };
        let writes = writes.borrow();
        let [(path, bytes)] = writes.as_slice() else {
            panic!("exactly one durable writer call is expected");
        };
        assert_eq!(
            path,
            &PathBuf::from("release").join("analog-core.rspiceproj")
        );
        assert_eq!(bytes.len() as u64, receipt.snapshot_byte_len());
        validate_rollback_artifact(&app.state, receipt.publication_id(), bytes)
            .expect("written bytes match committed receipt");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn download_only_writer_fails_before_bytes_or_receipt_are_published() {
        let mut app = RSpiceApp::test_instance();
        let draft = complete_publication_draft(&app);
        let writes = Rc::new(RefCell::new(Vec::new()));
        app.export_workflow_io = Box::new(PublicationIo {
            destination: PathBuf::from("download.rspiceproj"),
            reopenable: false,
            writes: Rc::clone(&writes),
        });

        let error = publish_native(&mut app, &draft)
            .expect_err("download-only backend cannot authorize a receipt");

        assert!(error.contains("cannot prove a durable"));
        assert!(writes.borrow().is_empty());
        assert!(
            app.state
                .workspace
                .project
                .library_publications()
                .is_empty()
        );
    }

    #[test]
    fn rollback_artifact_validation_is_exact_and_fail_closed() {
        let mut app = RSpiceApp::test_instance();
        let candidate = app
            .prepare_project_library_publication(
                "analog-core-1.0.0",
                "release-engineer@example.test",
                "organization-release-authority",
                "Qualified library handoff",
            )
            .expect("candidate prepares");
        let bytes = candidate.artifact_bytes().to_vec();
        let receipt = app
            .commit_project_library_publication(candidate)
            .expect("receipt commits after test-owned durable boundary");

        validate_rollback_artifact(&app.state, receipt.publication_id(), &bytes)
            .expect("exact bytes pass");
        let mut tampered = bytes;
        tampered[0] ^= 0x01;
        assert!(
            validate_rollback_artifact(&app.state, receipt.publication_id(), &tampered)
                .expect_err("tampered bytes fail")
                .contains("does not match")
        );
        assert!(
            validate_rollback_artifact(&app.state, uuid::Uuid::new_v4(), &tampered)
                .expect_err("foreign receipt fails")
                .contains("no longer retained")
        );
    }

    #[test]
    fn governance_dialogs_render_accessible_fields_at_phone_width() {
        let mut app = RSpiceApp::test_instance();
        let candidate = app
            .prepare_project_library_publication(
                "analog-core-1.0.0",
                "release-engineer@example.test",
                "organization-release-authority",
                "Qualified library handoff",
            )
            .expect("candidate prepares");
        app.commit_project_library_publication(candidate)
            .expect("fixture receipt commits");
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        open_publication(&ctx, &app.state);
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(390.0, 844.0),
                )),
                ..Default::default()
            },
            |ctx| show(ctx, &mut app),
        );
        let labels = output
            .platform_output
            .accesskit_update
            .expect("publication dialog exposes AccessKit")
            .nodes
            .into_iter()
            .filter_map(|(_, node)| node.label().map(str::to_owned))
            .collect::<Vec<_>>();
        assert!(labels.iter().any(|label| label == "Publication label"));
        assert!(labels.iter().any(|label| label == "Actor identity"));
        assert!(labels.iter().any(|label| label == "Authority identity"));

        open_rollback(&ctx, &app.state);
        let output = ctx.run_ui(egui::RawInput::default(), |ctx| show(ctx, &mut app));
        let labels = output
            .platform_output
            .accesskit_update
            .expect("rollback dialog exposes AccessKit")
            .nodes
            .into_iter()
            .filter_map(|(_, node)| node.label().map(str::to_owned))
            .collect::<Vec<_>>();
        assert!(
            labels
                .iter()
                .any(|label| label.contains("Choose exact artifact"))
        );
        assert!(
            labels
                .iter()
                .any(|label| label.contains("Restore exact publication"))
        );
    }

    #[test]
    fn publication_filename_is_platform_safe_and_typed() {
        let app = RSpiceApp::test_instance();
        let filename = publication_filename(&app.state, "analog / core 1.0");
        assert!(filename.ends_with(".rspiceproj"));
        assert!(!filename.contains('/'));
        assert_eq!(safe_file_stem("***"), "project-library");
    }
}
