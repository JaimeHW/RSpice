//! Project incarnation, accepted-generation and single-operation authority.

mod dirty;
mod save;
pub use save::{
    BrowserCanonicalSave, BrowserSavePublication, PreparedBrowserSave, SaveContinuationEvent,
    StagedBrowserSave,
};

use crate::persistence::BrowserBindingReceipt;
use crate::{
    AcceptedProject,
    registry::{DocumentRegistry, ProjectDocumentId, ResultFingerprintCache},
};

use rspice_app_types::product::{ContentDigest, TransactionId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveScope {
    ActiveDocument,
    AllDocuments,
}

#[derive(Debug, thiserror::Error)]
pub enum ProjectLifecycleError {
    #[error("no project is open")]
    NoProject,
    #[error("safe mode opened this project read-only; project writes are blocked for this launch")]
    SafeModeReadOnly,
    #[error(
        "a local simulation is running; stop it before replacing project-owned plan, model, or result state"
    )]
    ActiveRun,
    #[error("the active document has no accepted project baseline to restore")]
    NoAcceptedBaseline,
    #[error(
        "the active document cannot be closed because it is the only presented design document"
    )]
    LastPresentedDocument,
    #[error(
        "the project changed while the file picker was open; the replacement was cancelled to preserve newer edits"
    )]
    ReplacementChanged,
    #[error("another project lifecycle operation is already in progress")]
    TransactionInProgress,
    #[cfg(not(target_arch = "wasm32"))]
    #[error(
        "the canonical project could not be accepted at startup ({0}); use Save as project copy to preserve it"
    )]
    UnreadableCanonical(String),
    #[cfg(not(target_arch = "wasm32"))]
    #[error(
        "a project copy cannot replace the active project's canonical file; choose a different destination"
    )]
    CopyDestinationIsCanonical,
    #[error(
        "browser canonical-binding restoration or promotion is still in progress; wait for it to finish before saving"
    )]
    BrowserBindingRestorePending,
    #[error(
        "the canonical browser project changed outside RSpice; reopen it or save an independent project copy"
    )]
    BrowserExternalChange,
    #[error(
        "the active document or accepted project changed while the revert review was open; nothing was reverted"
    )]
    RevertReviewStale,
    #[error("project state is invalid: {0}")]
    InvalidState(String),
    #[error(transparent)]
    Persistence(#[from] crate::persistence::PersistenceError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserOperationContext {
    epoch: u64,
    operation_generation: u64,
    project_id: String,
    binding_receipt: Option<BrowserBindingReceipt>,
    accepted_generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevertReviewToken {
    document: ProjectDocumentId,
    accepted_generation: u64,
}

impl RevertReviewToken {
    pub fn document(&self) -> &ProjectDocumentId {
        &self.document
    }

    pub fn document_label(&self) -> String {
        self.document.label()
    }
}

#[derive(Debug, Clone)]
struct LifecycleTransaction {
    id: TransactionId,
    // Only replacement transactions carry a content guard. Save transactions
    // cannot authorize replacement, even if their ID is supplied to validation.
    replacement_guard: Option<ContentDigest>,
}

#[derive(Debug, Clone)]
pub struct ProjectLifecycle {
    // Incarnation changes invalidate callbacks across New and Close.
    epoch: u64,
    project_open: bool,
    transaction: Option<LifecycleTransaction>,
    accepted_generation: u64,
    registry: DocumentRegistry,
    result_fingerprints: ResultFingerprintCache,
    browser_operation_generation: u64,
    browser_restore_pending: bool,
    browser_promotion_pending: bool,
}

impl Default for ProjectLifecycle {
    fn default() -> Self {
        Self {
            epoch: 1,
            project_open: true,
            transaction: None,
            accepted_generation: 0,
            registry: DocumentRegistry::default(),
            result_fingerprints: ResultFingerprintCache::default(),
            browser_operation_generation: 1,
            browser_restore_pending: false,
            browser_promotion_pending: false,
        }
    }
}

impl ProjectLifecycle {
    pub const fn epoch(&self) -> u64 {
        self.epoch
    }
    pub const fn is_open(&self) -> bool {
        self.project_open
    }
    pub const fn accepted_generation(&self) -> u64 {
        self.accepted_generation
    }

    pub fn open_session(&mut self) {
        self.project_open = true;
    }

    pub fn require_open_project(&self) -> Result<(), ProjectLifecycleError> {
        self.project_open
            .then_some(())
            .ok_or(ProjectLifecycleError::NoProject)
    }

    pub fn reset_for_new_project(&mut self) {
        let next_epoch = self.epoch.wrapping_add(1).max(1);
        *self = Self {
            epoch: next_epoch,
            ..Self::default()
        };
    }

    pub fn close_project(&mut self) {
        self.reset_for_new_project();
        self.project_open = false;
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn restore_accepted_content(&mut self) {
        self.accepted_generation = 1;
    }

    pub fn accept_content(&mut self) {
        self.accepted_generation = self.accepted_generation.wrapping_add(1).max(1);
    }

    pub fn operation_in_progress(&self) -> bool {
        self.transaction.is_some() || self.browser_binding_pending()
    }

    /// Check before the application captures working content; beginning the
    /// transaction rechecks the same authority after capture.
    pub fn ensure_replacement_available(&self) -> Result<(), ProjectLifecycleError> {
        if self.operation_in_progress() {
            return Err(ProjectLifecycleError::TransactionInProgress);
        }
        Ok(())
    }

    pub fn begin_replacement(
        &mut self,
        content: ContentDigest,
    ) -> Result<TransactionId, ProjectLifecycleError> {
        self.ensure_replacement_available()?;
        let id = TransactionId::new();
        self.transaction = Some(LifecycleTransaction {
            id,
            replacement_guard: Some(content),
        });
        Ok(id)
    }

    /// Capture content only after the exact replacement transaction is found.
    pub fn validate_replacement(
        &self,
        id: TransactionId,
        current_content: impl FnOnce() -> Result<ContentDigest, ProjectLifecycleError>,
    ) -> Result<(), ProjectLifecycleError> {
        let transaction = self
            .transaction
            .as_ref()
            .filter(|transaction| transaction.id == id)
            .ok_or(ProjectLifecycleError::ReplacementChanged)?;
        let expected = transaction
            .replacement_guard
            .ok_or(ProjectLifecycleError::ReplacementChanged)?;
        let actual = current_content()?;
        if actual != expected {
            return Err(ProjectLifecycleError::ReplacementChanged);
        }
        Ok(())
    }

    pub fn begin_save(&mut self) -> Result<TransactionId, ProjectLifecycleError> {
        self.require_browser_binding_ready()?;
        if self.transaction.is_some() {
            return Err(ProjectLifecycleError::TransactionInProgress);
        }
        let id = TransactionId::new();
        self.transaction = Some(LifecycleTransaction {
            id,
            replacement_guard: None,
        });
        Ok(id)
    }

    pub fn cancel_transaction(&mut self) {
        self.transaction = None;
    }

    pub fn is_current_transaction(&self, id: TransactionId) -> bool {
        self.transaction
            .as_ref()
            .is_some_and(|transaction| transaction.id == id)
    }

    pub fn cancel_transaction_if(&mut self, id: TransactionId) -> bool {
        if self.is_current_transaction(id) {
            self.transaction = None;
            true
        } else {
            false
        }
    }

    pub fn prepare_revert(
        &self,
        document: ProjectDocumentId,
        accepted: Option<&AcceptedProject>,
    ) -> Result<RevertReviewToken, ProjectLifecycleError> {
        self.require_open_project()?;
        accepted.ok_or(ProjectLifecycleError::NoAcceptedBaseline)?;
        Ok(RevertReviewToken {
            document,
            accepted_generation: self.accepted_generation,
        })
    }

    pub fn validate_revert(
        &self,
        token: &RevertReviewToken,
        active_document: &ProjectDocumentId,
    ) -> Result<(), ProjectLifecycleError> {
        self.require_open_project()?;
        if *active_document != token.document
            || self.accepted_generation != token.accepted_generation
        {
            return Err(ProjectLifecycleError::RevertReviewStale);
        }
        Ok(())
    }

    fn browser_binding_pending(&self) -> bool {
        self.browser_restore_pending || self.browser_promotion_pending
    }

    pub fn require_browser_binding_ready(&self) -> Result<(), ProjectLifecycleError> {
        if self.browser_binding_pending() {
            return Err(ProjectLifecycleError::BrowserBindingRestorePending);
        }
        Ok(())
    }

    pub fn begin_browser_restore(&mut self) {
        self.browser_restore_pending = true;
    }

    pub fn finish_browser_restore(&mut self) {
        self.browser_restore_pending = false;
    }

    pub fn finish_browser_promotion(&mut self) {
        self.browser_promotion_pending = false;
    }

    pub fn clear_browser_pending(&mut self) {
        self.browser_restore_pending = false;
        self.browser_promotion_pending = false;
    }

    pub fn browser_operation_context(
        &self,
        project_id: &str,
        receipt: Option<&BrowserBindingReceipt>,
    ) -> BrowserOperationContext {
        BrowserOperationContext {
            epoch: self.epoch,
            operation_generation: self.browser_operation_generation,
            project_id: project_id.to_owned(),
            binding_receipt: receipt.cloned(),
            accepted_generation: self.accepted_generation,
        }
    }

    pub fn browser_operation_context_is_current(
        &self,
        context: &BrowserOperationContext,
        project_id: &str,
        receipt: Option<&BrowserBindingReceipt>,
    ) -> bool {
        operation_context_matches(
            context,
            self.epoch,
            self.browser_operation_generation,
            project_id,
            receipt,
            self.accepted_generation,
        )
    }

    pub fn begin_browser_promotion(&mut self) {
        self.browser_promotion_pending = true;
    }

    /// None means idle; Some reports whether a pending restore was cancelled.
    /// Every cancellation invalidates promises the platform cannot abort.
    pub fn cancel_browser_operation(&mut self) -> Option<bool> {
        let restore_was_pending = self.browser_restore_pending;
        if !self.operation_in_progress() {
            return None;
        }
        self.transaction = None;
        self.clear_browser_pending();
        self.browser_operation_generation =
            self.browser_operation_generation.wrapping_add(1).max(1);
        Some(restore_was_pending)
    }
}

fn operation_context_matches(
    context: &BrowserOperationContext,
    epoch: u64,
    operation_generation: u64,
    project_id: &str,
    receipt: Option<&BrowserBindingReceipt>,
    accepted_generation: u64,
) -> bool {
    context.epoch == epoch
        && context.operation_generation == operation_generation
        && context.project_id == project_id
        && context.binding_receipt.as_ref() == receipt
        && context.accepted_generation == accepted_generation
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence;
    #[test]
    fn browser_completion_context_rejects_every_authority_change() {
        let digest = persistence::digest_bytes(b"accepted browser bytes");
        let receipt = BrowserBindingReceipt {
            binding_id: uuid::Uuid::from_u128(0xf20c_f308_17a1_4fc4_8b0d_8f09_eab7_35c2),
            project_id: "logical-project".to_owned(),
            accepted_generation: 4,
            accepted_digest: digest,
            backend: persistence::BrowserBindingBackend::Opfs,
        };
        let context = BrowserOperationContext {
            epoch: 11,
            operation_generation: 3,
            project_id: receipt.project_id.clone(),
            binding_receipt: Some(receipt.clone()),
            accepted_generation: 9,
        };

        assert!(operation_context_matches(
            &context,
            11,
            3,
            "logical-project",
            Some(&receipt),
            9,
        ));
        assert!(!operation_context_matches(
            &context,
            12,
            3,
            "logical-project",
            Some(&receipt),
            9,
        ));
        assert!(!operation_context_matches(
            &context,
            11,
            3,
            "replacement-project",
            Some(&receipt),
            9,
        ));
        assert!(!operation_context_matches(
            &context,
            11,
            3,
            "logical-project",
            None,
            9,
        ));
        assert!(!operation_context_matches(
            &context,
            11,
            3,
            "logical-project",
            Some(&receipt),
            10,
        ));
        assert!(!operation_context_matches(
            &context,
            11,
            4,
            "logical-project",
            Some(&receipt),
            9,
        ));
    }
}
