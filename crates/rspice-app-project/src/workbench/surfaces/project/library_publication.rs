//! Publish project and Restore project from publication: their drafts and
//! fields.
//!
//! A publication is recorded only after the exact prepared project bytes
//! have crossed a writer boundary that can prove a durable native write or a
//! browser File System Access write with read-back verification. Browser
//! downloads are deliberately not treated as durable publication.

use super::*;
#[cfg(target_arch = "wasm32")]
use crate::product::ContentDigest;
use crate::product::ObjectRevision;
use crate::state::workspace::ProjectLibraryPublicationReceipt;
use crate::ui::widgets::FormRows;
use crate::workbench::app_state::Model;
use egui::Id;

pub const PUBLICATION_DIALOG_ID: &str = "workbench.project.library.publication";
pub const RESTORE_DIALOG_ID: &str = "workbench.project.library.publication.restore";
pub const PROJECT_ARTIFACT_EXTENSION: &str = "rspiceproj";
/// Who last published or restored, and who approved it, for this session:
/// the next dialog starts from them.
const IDENTITY_MEMORY_ID: &str = "workbench.project.library.publication.identity";

#[derive(Debug, Clone, Default)]
struct RememberedIdentity {
    actor_id: String,
    authority_id: String,
}

/// Keep `actor_id` and `authority_id` for the next Publish or Restore of
/// this session.
pub fn remember_identity(ctx: &Context, actor_id: &str, authority_id: &str) {
    ctx.data_mut(|data| {
        data.insert_temp(
            egui::Id::new(IDENTITY_MEMORY_ID),
            RememberedIdentity {
                actor_id: actor_id.trim().to_owned(),
                authority_id: authority_id.trim().to_owned(),
            },
        );
    });
}

fn remembered_identity(ctx: &Context) -> RememberedIdentity {
    ctx.data(|data| data.get_temp::<RememberedIdentity>(egui::Id::new(IDENTITY_MEMORY_ID)))
        .unwrap_or_default()
}

/// Why `value`, the field the reader knows as `label`, cannot be recorded:
/// each is required and on one line. Spaces around a value are dropped when
/// it is recorded.
fn field_refusal(label: &str, value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        Some(format!("Enter {label}."))
    } else if !audit_text_valid(value) {
        Some(format!("Put {label} on one line."))
    } else {
        None
    }
}

/// The first field of `fields` the dialog cannot record, as the reader is
/// told it.
fn first_field_error(fields: &[(&'static str, &str)]) -> Option<String> {
    fields
        .iter()
        .find_map(|(label, value)| field_refusal(label, value))
}

/// Which of a form's fields the reader has left, and whether they have
/// asked to commit it. A required field reads as missing only after one of
/// the two: a form opened with empty fields is being filled in, not wrong.
#[derive(Debug, Clone, Copy, Default)]
pub struct Visited {
    left: u8,
    submitted: bool,
}

impl Visited {
    fn leave(&mut self, field: usize) {
        self.left |= 1 << field;
    }

    /// The reader asked to commit: every field now says what it lacks.
    pub fn submit(&mut self) {
        self.submitted = true;
    }

    fn shows(self, field: usize) -> bool {
        self.submitted || self.left & (1 << field) != 0
    }

    /// The first refusal among `fields` the reader has reached.
    fn first_error(self, fields: &[(&'static str, &str)]) -> Option<String> {
        fields
            .iter()
            .enumerate()
            .filter(|(index, _)| self.shows(*index))
            .find_map(|(_, (label, value))| field_refusal(label, value))
    }
}

#[derive(Debug, Clone)]
pub struct PublicationDraft {
    pub source_project_revision: u64,
    pub source_library_revision: u64,
    pub label: String,
    pub actor_id: String,
    pub authority_id: String,
    pub reason: String,
    pub busy: bool,
    #[cfg(target_arch = "wasm32")]
    pub active_publication_id: Option<uuid::Uuid>,
    pub error: Option<String>,
    pub visited: Visited,
}

impl PublicationDraft {
    pub fn new(model: &Model) -> Self {
        Self {
            source_project_revision: model.workspace.content.project.revision().get(),
            source_library_revision: model.library_manager.revision(),
            label: format!(
                "{}-r{}",
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
            visited: Visited::default(),
        }
    }

    fn fields(&self) -> [(&'static str, &str); 4] {
        [
            ("a label", &self.label),
            ("who publishes", &self.actor_id),
            ("who approved it", &self.authority_id),
            ("a reason", &self.reason),
        ]
    }

    /// Why the draft cannot be recorded, if it cannot.
    pub fn field_error(&self) -> Option<String> {
        first_field_error(&self.fields())
    }

    /// [`Self::field_error`] as far as the reader has reached: a field they
    /// have left, or any once they asked to publish.
    pub fn shown_field_error(&self) -> Option<String> {
        self.visited.first_error(&self.fields())
    }
}

#[derive(Debug, Clone)]
pub struct RestoreDraft {
    pub publication_id: uuid::Uuid,
    pub actor_id: String,
    pub authority_id: String,
    pub reason: String,
    pub artifact_name: Option<String>,
    pub artifact_bytes: Option<Vec<u8>>,
    pub busy: bool,
    #[cfg(target_arch = "wasm32")]
    pub active_picker_id: Option<uuid::Uuid>,
    pub error: Option<String>,
    pub visited: Visited,
}

impl RestoreDraft {
    fn new(receipt: &ProjectLibraryPublicationReceipt, identity: RememberedIdentity) -> Self {
        Self {
            publication_id: receipt.publication_id(),
            actor_id: identity.actor_id,
            authority_id: identity.authority_id,
            reason: String::new(),
            artifact_name: None,
            artifact_bytes: None,
            busy: false,
            #[cfg(target_arch = "wasm32")]
            active_picker_id: None,
            error: None,
            visited: Visited::default(),
        }
    }

    fn fields(&self) -> [(&'static str, &str); 3] {
        [
            ("who restores", &self.actor_id),
            ("who approved it", &self.authority_id),
            ("a reason", &self.reason),
        ]
    }

    fn file_error(&self) -> Option<String> {
        self.artifact_bytes
            .is_none()
            .then(|| "Choose the published file.".to_owned())
    }

    /// Why the restore cannot run, if it cannot.
    pub fn field_error(&self) -> Option<String> {
        first_field_error(&self.fields()).or_else(|| self.file_error())
    }

    /// [`Self::field_error`] as far as the reader has reached: a field they
    /// have left, or any, the file included, once they asked to restore.
    pub fn shown_field_error(&self) -> Option<String> {
        self.visited
            .first_error(&self.fields())
            .or_else(|| self.visited.submitted.then(|| self.file_error()).flatten())
    }
}

#[cfg(target_arch = "wasm32")]
pub struct BrowserPublicationCompletion {
    pub candidate: ProjectLibraryPublicationCandidate,
    pub result: crate::workbench::lifecycle::project_lifecycle::BrowserWriteResult,
}

#[cfg(target_arch = "wasm32")]
pub struct BrowserRestoreCompletion {
    pub picker_id: uuid::Uuid,
    pub result: crate::workbench::lifecycle::project_lifecycle::BrowserOpenResult,
}

/// The checkpoint a browser restore takes first, once it is written.
#[cfg(target_arch = "wasm32")]
pub struct BrowserRestoreCheckpoint {
    pub publication_id: uuid::Uuid,
    pub result:
        Result<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary, String>,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    pub static BROWSER_PUBLICATION_COMPLETIONS:
        std::cell::RefCell<std::collections::VecDeque<BrowserPublicationCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
    pub static BROWSER_RESTORE_COMPLETIONS:
        std::cell::RefCell<std::collections::VecDeque<BrowserRestoreCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
    pub static BROWSER_RESTORE_CHECKPOINTS:
        std::cell::RefCell<std::collections::VecDeque<BrowserRestoreCheckpoint>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

/// Whether this build can write and read a project file the user chooses.
/// A browser without the File System Access pickers cannot.
pub fn file_pickers_available() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        crate::workbench::lifecycle::project_lifecycle::browser_file_picker_supported()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        true
    }
}

/// What the project's state is, as Publish project and Restore project from
/// publication read it.
#[derive(Debug, Clone, Copy)]
pub struct PublicationReadiness {
    pub project_open: bool,
    pub file_pickers: bool,
    pub read_only: bool,
    pub simulation_running: bool,
    pub has_publications: bool,
}

impl PublicationReadiness {
    pub fn of(model: &Model, project_open: bool) -> Self {
        Self {
            project_open,
            file_pickers: file_pickers_available(),
            read_only: model.safe_mode.project_read_only(),
            simulation_running: model.simulation.execution.has_active_execution(),
            has_publications: !model
                .workspace
                .content
                .project
                .library_publications()
                .is_empty(),
        }
    }

    /// Why Publish project cannot run now, if it cannot.
    pub fn publish_blocker(self) -> Option<&'static str> {
        if !self.project_open {
            Some("Open a project to publish it.")
        } else if !self.file_pickers {
            Some("This browser cannot save a file to disk; publish from the desktop app.")
        } else if self.read_only {
            Some("The project is open read-only.")
        } else if self.simulation_running {
            Some("A simulation is running.")
        } else {
            None
        }
    }

    /// Why Restore project from publication cannot run now, if it cannot.
    pub fn restore_blocker(self) -> Option<&'static str> {
        if !self.project_open {
            Some("Open a project to restore it from a publication.")
        } else if !self.has_publications {
            Some("The project has no publications to restore from.")
        } else if !self.file_pickers {
            Some("This browser cannot open a file from disk; restore from the desktop app.")
        } else if self.read_only {
            Some("The project is open read-only.")
        } else if self.simulation_running {
            Some("A simulation is running.")
        } else {
            None
        }
    }
}

pub fn open_publication(ctx: &Context, model: &Model) {
    let identity = remembered_identity(ctx);
    let mut draft = PublicationDraft::new(model);
    draft.actor_id = identity.actor_id;
    draft.authority_id = identity.authority_id;
    ctx.data_mut(|data| {
        data.remove::<RestoreDraft>(egui::Id::new(RESTORE_DIALOG_ID));
        data.insert_temp(egui::Id::new(PUBLICATION_DIALOG_ID), draft);
    });
}

pub fn open_restore(ctx: &Context, model: &Model) {
    let Some(receipt) = model
        .workspace
        .content
        .project
        .library_publications()
        .last()
    else {
        return;
    };
    let draft = RestoreDraft::new(receipt, remembered_identity(ctx));
    ctx.data_mut(|data| {
        data.remove::<PublicationDraft>(egui::Id::new(PUBLICATION_DIALOG_ID));
        data.insert_temp(egui::Id::new(RESTORE_DIALOG_ID), draft);
    });
}

#[cfg(any(test, feature = "test-support"))]
pub fn publication_dialog_open(ctx: &Context) -> bool {
    ctx.data(|data| {
        data.get_temp::<PublicationDraft>(egui::Id::new(PUBLICATION_DIALOG_ID))
            .is_some()
    })
}

#[cfg(any(test, feature = "test-support"))]
pub fn restore_dialog_open(ctx: &Context) -> bool {
    ctx.data(|data| {
        data.get_temp::<RestoreDraft>(egui::Id::new(RESTORE_DIALOG_ID))
            .is_some()
    })
}

/// The Publish dialog's audit fields, as rows of its form, in the body face
/// down one label column. Returns the field the keyboard starts in and a
/// refusal marks: the first that cannot be recorded, else the first.
pub fn publication_fields(rows: &mut FormRows<'_>, draft: &mut PublicationDraft) -> Option<Id> {
    let ids = [
        "workbench.project.library.publication.label",
        "workbench.project.library.publication.actor",
        "workbench.project.library.publication.authority",
        "workbench.project.library.publication.reason",
    ]
    .map(Id::new);
    let principal = principal_field(&ids, &draft.fields());
    let PublicationDraft {
        label,
        actor_id,
        authority_id,
        reason,
        visited,
        ..
    } = draft;
    audit_fields(
        rows,
        visited,
        [
            ("Label", ids[0], label, "analog-core-1.0.0"),
            ("Published by", ids[1], actor_id, "Your name"),
            ("Approved by", ids[2], authority_id, "Who approved it"),
            ("Reason", ids[3], reason, "Handed to layout"),
        ],
    );
    principal
}

/// The Restore dialog's audit fields, as [`publication_fields`] sets
/// Publish's.
pub fn restore_fields(rows: &mut FormRows<'_>, draft: &mut RestoreDraft) -> Option<Id> {
    let ids = [
        "workbench.project.library.restore.actor",
        "workbench.project.library.restore.authority",
        "workbench.project.library.restore.reason",
    ]
    .map(Id::new);
    let principal = principal_field(&ids, &draft.fields());
    let RestoreDraft {
        actor_id,
        authority_id,
        reason,
        visited,
        ..
    } = draft;
    audit_fields(
        rows,
        visited,
        [
            ("Restored by", ids[0], actor_id, "Your name"),
            ("Approved by", ids[1], authority_id, "Who approved it"),
            ("Reason", ids[2], reason, "Back to the layout handoff"),
        ],
    );
    principal
}

/// The first of `ids` whose field cannot be recorded, else the first.
fn principal_field(ids: &[Id], fields: &[(&'static str, &str)]) -> Option<Id> {
    fields
        .iter()
        .position(|(label, value)| field_refusal(label, value).is_some())
        .and_then(|index| ids.get(index))
        .or(ids.first())
        .copied()
}

/// One text row a field, noting each field the reader leaves.
fn audit_fields<const N: usize>(
    rows: &mut FormRows<'_>,
    visited: &mut Visited,
    fields: [(&str, Id, &mut String, &str); N],
) {
    for (index, (label, id, value, hint)) in fields.into_iter().enumerate() {
        if rows.text_sans(label, id, value, hint).lost_focus() {
            visited.leave(index);
        }
    }
}

/// The publication a restore puts back, picked by its sequence and label,
/// with what was recorded when it was published.
pub fn restore_publication_selector(
    rows: &mut FormRows<'_>,
    receipts: &[ProjectLibraryPublicationReceipt],
    draft: &mut RestoreDraft,
) {
    let name = |receipt: &ProjectLibraryPublicationReceipt| {
        format!("#{:04} \u{00b7} {}", receipt.sequence(), receipt.label())
    };
    let newest_first = receipts.iter().rev().collect::<Vec<_>>();
    let selected = selected_receipt(receipts, draft.publication_id)
        .expect("the restore's publication is normalized before rendering");
    let options = newest_first
        .iter()
        .map(|receipt| name(receipt))
        .collect::<Vec<_>>();
    if let Some(picked) = rows.select(
        "Publication",
        "workbench.project.library.restore.publication",
        &name(selected),
        &options,
    ) && let Some(receipt) = newest_first.get(picked)
        && receipt.publication_id() != draft.publication_id
    {
        draft.publication_id = receipt.publication_id();
        clear_restore_file(draft);
    }
    let selected = selected_receipt(receipts, draft.publication_id)
        .expect("the selected publication remains retained");
    rows.value(
        "Revision",
        &selected.source_project_revision().get().to_string(),
    );
    rows.value("Published by", selected.actor_id());
    rows.value("Approved by", selected.authority_id());
    rows.value("Reason", selected.reason());
}

pub fn selected_receipt(
    receipts: &[ProjectLibraryPublicationReceipt],
    publication_id: uuid::Uuid,
) -> Option<&ProjectLibraryPublicationReceipt> {
    receipts
        .iter()
        .find(|receipt| receipt.publication_id() == publication_id)
}

pub fn audit_text_valid(value: &str) -> bool {
    !value.is_empty() && value == value.trim() && !value.chars().any(char::is_control)
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub enum PublicationAction {
    Published(String),
    Cancelled,
}

#[cfg(target_arch = "wasm32")]
pub fn start_browser_restore_picker(ctx: &Context) -> Result<uuid::Uuid, String> {
    use crate::workbench::lifecycle::project_lifecycle::start_browser_open;

    let picker_id = uuid::Uuid::new_v4();
    let repaint = ctx.clone();
    start_browser_open(move |result| {
        BROWSER_RESTORE_COMPLETIONS.with(|queue| {
            queue
                .borrow_mut()
                .push_back(BrowserRestoreCompletion { picker_id, result });
        });
        repaint.request_repaint();
    })?;
    Ok(picker_id)
}

pub fn clear_restore_file(draft: &mut RestoreDraft) {
    draft.artifact_name = None;
    draft.artifact_bytes = None;
}

pub fn safe_file_stem(value: &str) -> String {
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
        "project".to_owned()
    } else {
        output
    }
}

pub struct ProjectLibraryPublicationCandidate {
    pub draft: crate::state::workspace::ProjectLibraryPublicationDraft,
    pub artifact_bytes: Vec<u8>,
    pub source_project_revision: ObjectRevision,
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
