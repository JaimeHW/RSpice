//! Frozen save candidates, publication admission and continuation authority.

use super::{BrowserOperationContext, ProjectLifecycle, ProjectLifecycleError, SaveScope};
use crate::ProjectFile;
use crate::persistence::browser::{BrowserBinding, BrowserWriteIntent};
use crate::persistence::{BrowserBindingBackend, BrowserBindingReceipt, serialized_project};
use crate::registry::ProjectDocumentId;
use rspice_app_types::product::{ContentDigest, TransactionId};
use std::path::PathBuf;

/// Serialized content captured before binding lookup or platform interaction.
pub struct StagedBrowserSave {
    transaction: TransactionId,
    context: BrowserOperationContext,
    candidate: ProjectFile,
    scope: SaveScope,
    saved_document: ProjectDocumentId,
    project_copy: bool,
    bytes: Vec<u8>,
    staged_digest: ContentDigest,
}

impl StagedBrowserSave {
    pub fn new(
        transaction: TransactionId,
        context: BrowserOperationContext,
        mut candidate: ProjectFile,
        scope: SaveScope,
        saved_document: ProjectDocumentId,
        project_copy: bool,
        suggested_name: &str,
    ) -> Result<Self, ProjectLifecycleError> {
        if project_copy {
            candidate.workspace.project = candidate
                .workspace
                .project
                .fork_copy_at(PathBuf::from(suggested_name));
        } else {
            candidate.workspace.project.path = None;
        }
        let (bytes, staged_digest) = serialized_project(&candidate)?;
        Ok(Self {
            transaction,
            context,
            candidate,
            scope,
            saved_document,
            project_copy,
            bytes,
            staged_digest,
        })
    }

    /// Resolve binding expectations only after serialization has succeeded.
    /// Platform capability discovery is needed only for a new binding.
    pub fn bind(
        self,
        existing: Option<&BrowserBinding>,
        fresh_backend: impl FnOnce() -> BrowserBindingBackend,
    ) -> Result<PreparedBrowserSave, ProjectLifecycleError> {
        let project_id = self.candidate.workspace.project.id().to_string();
        let intent = if let Some(binding) = existing {
            binding.prepare_write(project_id)?
        } else {
            BrowserWriteIntent::fresh(project_id, fresh_backend())
        };
        Ok(PreparedBrowserSave {
            staged: self,
            intent,
        })
    }
}

/// One frozen candidate paired with the exact expectations sent to storage.
pub struct PreparedBrowserSave {
    staged: StagedBrowserSave,
    intent: BrowserWriteIntent,
}

impl PreparedBrowserSave {
    pub fn transaction(&self) -> TransactionId {
        self.staged.transaction
    }
    pub fn context(&self) -> &BrowserOperationContext {
        &self.staged.context
    }
    pub fn candidate(&self) -> &ProjectFile {
        &self.staged.candidate
    }
    pub fn scope(&self) -> SaveScope {
        self.staged.scope
    }
    /// Active-document continuation must still refer to the tab captured here.
    pub fn saved_document(&self) -> &ProjectDocumentId {
        &self.staged.saved_document
    }
    pub fn is_project_copy(&self) -> bool {
        self.staged.project_copy
    }
    pub fn intent(&self) -> &BrowserWriteIntent {
        &self.intent
    }
    pub fn take_bytes(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.staged.bytes)
    }
}

pub struct BrowserSavePublication {
    pub receipt: BrowserBindingReceipt,
    pub display_name: String,
    pub durable: bool,
}

/// A verified canonical publication ready for adoption with its editor session.
pub struct BrowserCanonicalSave {
    pub candidate: ProjectFile,
    pub binding: BrowserBinding,
    pub scope: SaveScope,
}

impl ProjectLifecycle {
    /// Return no canonical content for an independent copy. Release the host
    /// handle before cancelling a copy or refused publication. A stale result
    /// must leave any newer transaction intact; successful canonical adoption
    /// finishes the transaction after the host applies this returned content.
    pub fn complete_browser_save(
        &mut self,
        prepared: PreparedBrowserSave,
        project_id: &str,
        current_receipt: Option<&BrowserBindingReceipt>,
        publication: BrowserSavePublication,
        release_handle: impl FnOnce(),
    ) -> Result<Option<BrowserCanonicalSave>, ProjectLifecycleError> {
        let PreparedBrowserSave { staged, intent } = prepared;
        if !self.is_current_transaction(staged.transaction)
            || !self.browser_operation_context_is_current(
                &staged.context,
                project_id,
                current_receipt,
            )
        {
            release_handle();
            return Err(ProjectLifecycleError::TransactionInProgress);
        }
        if staged.project_copy {
            release_handle();
            self.cancel_transaction();
            return Ok(None);
        }
        let binding = match intent.accept_publication(
            staged.staged_digest,
            publication.receipt,
            publication.display_name,
            publication.durable,
        ) {
            Ok(binding) => binding,
            Err(error) => {
                release_handle();
                self.cancel_transaction();
                return Err(error);
            }
        };
        Ok(Some(BrowserCanonicalSave {
            candidate: staged.candidate,
            binding,
            scope: staged.scope,
        }))
    }
}

impl SaveScope {
    /// Saving and continuing are distinct. Newer edits or a different active
    /// document require another review instead of authorizing their discard.
    pub fn authorizes_continuation(
        self,
        saved_document: &ProjectDocumentId,
        active_document: impl FnOnce() -> ProjectDocumentId,
        has_unsaved_changes: impl FnOnce() -> bool,
        active_document_is_dirty: impl FnOnce() -> bool,
    ) -> bool {
        match self {
            Self::AllDocuments => !has_unsaved_changes(),
            Self::ActiveDocument => {
                active_document() == *saved_document && !active_document_is_dirty()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaveContinuationEvent {
    Saved(TransactionId),
    SavedWithNewerChanges(TransactionId),
    Cancelled(TransactionId),
    Conflict(TransactionId),
    Failed(TransactionId, String),
    PublishedButNotAdopted(TransactionId, String),
}

impl SaveContinuationEvent {
    pub fn transaction(&self) -> TransactionId {
        match self {
            Self::Saved(transaction)
            | Self::SavedWithNewerChanges(transaction)
            | Self::Cancelled(transaction)
            | Self::Conflict(transaction)
            | Self::Failed(transaction, _)
            | Self::PublishedButNotAdopted(transaction, _) => *transaction,
        }
    }

    pub fn authorizes_destructive_action(&self) -> bool {
        matches!(self, Self::Saved(_))
    }

    pub fn needs_another_save(&self) -> bool {
        matches!(self, Self::SavedWithNewerChanges(_))
    }

    pub fn failure_message(&self) -> Option<&str> {
        match self {
            Self::Cancelled(_) => Some("The canonical save was cancelled."),
            Self::Conflict(_) => Some(
                "The canonical project changed outside RSpice; reopen it or save an independent project copy.",
            ),
            Self::Failed(_, message) | Self::PublishedButNotAdopted(_, message) => Some(message),
            Self::Saved(_) | Self::SavedWithNewerChanges(_) => None,
        }
    }
}

#[cfg(test)]
mod tests;
