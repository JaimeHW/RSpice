//! End-user project-library publication and rollback transactions.
//!
//! A publication receipt is committed only after the exact prepared project
//! bytes have crossed a writer boundary that can prove a durable native write
//! or a browser File System Access write with read-back verification. Browser
//! downloads are deliberately not treated as durable publication.

use super::*;
#[cfg(target_arch = "wasm32")]
use crate::product::ContentDigest;
use crate::product::ObjectRevision;
use crate::state::workspace::ProjectLibraryPublicationReceipt;
use crate::workbench::app_state::Model;

pub(crate) const PUBLICATION_DIALOG_ID: &str = "workbench.project.library.publication";
pub(crate) const ROLLBACK_DIALOG_ID: &str = "workbench.project.library.publication.rollback";
pub(crate) const PROJECT_ARTIFACT_EXTENSION: &str = "rspiceproj";

#[derive(Debug, Clone)]
pub(crate) struct PublicationDraft {
    pub(crate) source_project_revision: u64,
    pub(crate) source_library_revision: u64,
    pub(crate) label: String,
    pub(crate) actor_id: String,
    pub(crate) authority_id: String,
    pub(crate) reason: String,
    pub(crate) busy: bool,
    #[cfg(target_arch = "wasm32")]
    pub(crate) active_publication_id: Option<uuid::Uuid>,
    pub(crate) error: Option<String>,
}

impl PublicationDraft {
    pub(crate) fn new(model: &Model) -> Self {
        Self {
            source_project_revision: model.workspace.content.project.revision().get(),
            source_library_revision: model.library_manager.revision(),
            label: format!(
                "{}-library-r{}",
                model.workspace.content.project.name(),
                model.workspace.content.project.revision().get()
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

    pub(crate) fn field_error(&self) -> Option<&'static str> {
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
pub(crate) struct RollbackDraft {
    pub(crate) publication_id: uuid::Uuid,
    pub(crate) actor_id: String,
    pub(crate) authority_id: String,
    pub(crate) reason: String,
    pub(crate) artifact_name: Option<String>,
    pub(crate) artifact_bytes: Option<Vec<u8>>,
    pub(crate) busy: bool,
    #[cfg(target_arch = "wasm32")]
    pub(crate) active_picker_id: Option<uuid::Uuid>,
    pub(crate) error: Option<String>,
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

    pub(crate) fn field_error(&self) -> Option<&'static str> {
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
            return Some("Choose the saved publication artifact before rollback.");
        }
        None
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) struct BrowserPublicationCompletion {
    pub(crate) candidate: ProjectLibraryPublicationCandidate,
    pub(crate) result: crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult,
}

#[cfg(target_arch = "wasm32")]
pub(crate) struct BrowserRollbackCompletion {
    pub(crate) picker_id: uuid::Uuid,
    pub(crate) result: crate::workbench::lifecycle::project_lifecycle::BrowserOpenResult,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    pub(crate) static BROWSER_PUBLICATION_COMPLETIONS:
        std::cell::RefCell<std::collections::VecDeque<BrowserPublicationCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
    pub(crate) static BROWSER_ROLLBACK_COMPLETIONS:
        std::cell::RefCell<std::collections::VecDeque<BrowserRollbackCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

pub(crate) fn open_publication(ctx: &Context, model: &Model) {
    ctx.data_mut(|data| {
        data.remove::<RollbackDraft>(egui::Id::new(ROLLBACK_DIALOG_ID));
        data.insert_temp(
            egui::Id::new(PUBLICATION_DIALOG_ID),
            PublicationDraft::new(model),
        );
    });
}

pub(crate) fn open_rollback(ctx: &Context, model: &Model) {
    let Some(receipt) = model
        .workspace
        .content
        .project
        .library_publications()
        .last()
    else {
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
pub(crate) fn publication_dialog_open(ctx: &Context) -> bool {
    ctx.data(|data| {
        data.get_temp::<PublicationDraft>(egui::Id::new(PUBLICATION_DIALOG_ID))
            .is_some()
    })
}

#[cfg(test)]
pub(crate) fn rollback_dialog_open(ctx: &Context) -> bool {
    ctx.data(|data| {
        data.get_temp::<RollbackDraft>(egui::Id::new(ROLLBACK_DIALOG_ID))
            .is_some()
    })
}

pub(crate) fn publication_fields(ui: &mut Ui, draft: &mut PublicationDraft) {
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

pub(crate) fn rollback_fields(ui: &mut Ui, draft: &mut RollbackDraft) {
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

pub(crate) fn rollback_receipt_selector(
    ui: &mut Ui,
    receipts: &[ProjectLibraryPublicationReceipt],
    draft: &mut RollbackDraft,
) {
    property_card(ui, "Retained publication", |ui| {
        let selected = selected_receipt(receipts, draft.publication_id)
            .expect("rollback receipt is normalized before rendering");
        let prior = draft.publication_id;
        crate::ui::widgets::Combo::from_id_salt("workbench.project.library.rollback.publication")
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

pub(crate) fn selected_receipt(
    receipts: &[ProjectLibraryPublicationReceipt],
    publication_id: uuid::Uuid,
) -> Option<&ProjectLibraryPublicationReceipt> {
    receipts
        .iter()
        .find(|receipt| receipt.publication_id() == publication_id)
}

pub(crate) fn audit_text_valid(value: &str) -> bool {
    !value.is_empty()
        && value == value.trim()
        && value.chars().count() <= 240
        && !value.chars().any(char::is_control)
}

/// What stopped the transaction, or what it is waiting for.
///
/// A refusal is a problem line: the kit's cross in front of neutral words.
/// A wait is neither a problem nor news, so it is a line of dim text with no
/// frame around it — the blue box it used to sit in read as an alert for
/// something that is simply in progress.
pub(crate) fn transaction_status(ui: &mut Ui, message: &str, error: bool) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(8.0);
    if error {
        problem_line(ui, Problem::Failure, message);
    } else {
        ui.label(
            egui::RichText::new(message)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub(crate) enum PublicationAction {
    Published(String),
    Cancelled,
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn start_browser_rollback_picker(ctx: &Context) -> Result<uuid::Uuid, String> {
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

pub(crate) fn clear_rollback_artifact(draft: &mut RollbackDraft) {
    draft.artifact_name = None;
    draft.artifact_bytes = None;
}

pub(crate) fn safe_file_stem(value: &str) -> String {
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

pub(crate) struct ProjectLibraryPublicationCandidate {
    pub(crate) draft: crate::state::workspace::ProjectLibraryPublicationDraft,
    pub(crate) artifact_bytes: Vec<u8>,
    pub(crate) source_project_revision: ObjectRevision,
}

impl ProjectLibraryPublicationCandidate {
    #[must_use]
    pub fn artifact_bytes(&self) -> &[u8] {
        &self.artifact_bytes
    }

    #[cfg(target_arch = "wasm32")]
    #[must_use]
    pub fn publication_id(&self) -> uuid::Uuid {
        self.draft.publication_id
    }

    #[cfg(target_arch = "wasm32")]
    #[must_use]
    pub fn snapshot_digest(&self) -> ContentDigest {
        self.draft.snapshot_digest
    }
}
