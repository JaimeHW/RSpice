//! Immutable, content-addressed project-library publication receipts.
//!
//! Publication freezes the complete validated project file that existed
//! immediately before the receipt was appended. Artifact persistence belongs
//! to the caller (native file, browser download, or a trusted repository);
//! the project retains the exact digest, size, revisions, identity, authority
//! declaration, and hash-chain linkage needed to validate that artifact.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use uuid::Uuid;

use crate::product::{ContentDigest, ObjectRevision, ProjectId};

use super::{ProjectDescriptor, ProjectDescriptorError, validate_library_audit_text};

pub const PROJECT_LIBRARY_PUBLICATION_RECEIPT_SCHEMA_VERSION: u16 = 1;
pub const MAX_PROJECT_LIBRARY_PUBLICATION_RECEIPTS: usize = 4_096;
const MAX_PROJECT_LIBRARY_PUBLICATION_BYTES: u64 = 16 * 1024 * 1024 * 1024;

/// Validated caller input for one exact publication transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectLibraryPublicationDraft {
    pub(crate) publication_id: Uuid,
    pub(crate) label: String,
    pub(crate) actor_id: String,
    pub(crate) authority_id: String,
    pub(crate) reason: String,
    pub(crate) created_unix_ms: u64,
    pub(crate) library_revision: u64,
    pub(crate) snapshot_digest: ContentDigest,
    pub(crate) snapshot_byte_len: u64,
}

impl ProjectLibraryPublicationDraft {
    pub(crate) fn validate(&self) -> Result<(), ProjectDescriptorError> {
        if self.publication_id.is_nil() {
            return Err(ProjectDescriptorError::LibraryPublicationCorrupted(
                "publication identity must not be nil".to_owned(),
            ));
        }
        for (field, value) in [
            ("publication.label", self.label.as_str()),
            ("publication.actor_id", self.actor_id.as_str()),
            ("publication.authority_id", self.authority_id.as_str()),
            ("publication.reason", self.reason.as_str()),
        ] {
            validate_library_audit_text(field, value)?;
        }
        if self.created_unix_ms == 0 {
            return Err(ProjectDescriptorError::LibraryPublicationCorrupted(
                "publication creation time must be greater than zero".to_owned(),
            ));
        }
        if self.snapshot_byte_len == 0
            || self.snapshot_byte_len > MAX_PROJECT_LIBRARY_PUBLICATION_BYTES
        {
            return Err(ProjectDescriptorError::LibraryPublicationCorrupted(
                format!(
                    "publication artifact size must be between 1 and {MAX_PROJECT_LIBRARY_PUBLICATION_BYTES} bytes"
                ),
            ));
        }
        Ok(())
    }
}

/// Immutable hash-chained evidence for one exported project-library snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectLibraryPublicationReceipt {
    schema_version: u16,
    sequence: u64,
    publication_id: Uuid,
    project_id: ProjectId,
    label: String,
    actor_id: String,
    authority_id: String,
    reason: String,
    created_unix_ms: u64,
    source_project_revision: ObjectRevision,
    receipt_project_revision: ObjectRevision,
    library_revision: u64,
    snapshot_digest: ContentDigest,
    snapshot_byte_len: u64,
    previous_receipt_digest: Option<ContentDigest>,
    receipt_digest: ContentDigest,
}

#[derive(Serialize)]
struct ProjectLibraryPublicationReceiptPayload<'a> {
    schema_version: u16,
    sequence: u64,
    publication_id: Uuid,
    project_id: ProjectId,
    label: &'a str,
    actor_id: &'a str,
    authority_id: &'a str,
    reason: &'a str,
    created_unix_ms: u64,
    source_project_revision: ObjectRevision,
    receipt_project_revision: ObjectRevision,
    library_revision: u64,
    snapshot_digest: ContentDigest,
    snapshot_byte_len: u64,
    previous_receipt_digest: Option<ContentDigest>,
}

impl ProjectLibraryPublicationReceipt {
    fn calculate_digest(&self) -> Result<ContentDigest, ProjectDescriptorError> {
        let bytes = serde_json::to_vec(&ProjectLibraryPublicationReceiptPayload {
            schema_version: self.schema_version,
            sequence: self.sequence,
            publication_id: self.publication_id,
            project_id: self.project_id,
            label: &self.label,
            actor_id: &self.actor_id,
            authority_id: &self.authority_id,
            reason: &self.reason,
            created_unix_ms: self.created_unix_ms,
            source_project_revision: self.source_project_revision,
            receipt_project_revision: self.receipt_project_revision,
            library_revision: self.library_revision,
            snapshot_digest: self.snapshot_digest,
            snapshot_byte_len: self.snapshot_byte_len,
            previous_receipt_digest: self.previous_receipt_digest,
        })
        .map_err(|error| ProjectDescriptorError::Serialization(error.to_string()))?;
        Ok(ContentDigest::from_bytes(Sha256::digest(bytes).into()))
    }

    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn publication_id(&self) -> Uuid {
        self.publication_id
    }

    #[must_use]
    pub const fn project_id(&self) -> ProjectId {
        self.project_id
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub fn actor_id(&self) -> &str {
        &self.actor_id
    }

    #[must_use]
    pub fn authority_id(&self) -> &str {
        &self.authority_id
    }

    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }

    #[must_use]
    pub const fn created_unix_ms(&self) -> u64 {
        self.created_unix_ms
    }

    #[must_use]
    pub const fn source_project_revision(&self) -> ObjectRevision {
        self.source_project_revision
    }

    #[must_use]
    pub const fn receipt_project_revision(&self) -> ObjectRevision {
        self.receipt_project_revision
    }

    #[must_use]
    pub const fn library_revision(&self) -> u64 {
        self.library_revision
    }

    #[must_use]
    pub const fn snapshot_digest(&self) -> ContentDigest {
        self.snapshot_digest
    }

    #[must_use]
    pub const fn snapshot_byte_len(&self) -> u64 {
        self.snapshot_byte_len
    }

    #[must_use]
    pub const fn previous_receipt_digest(&self) -> Option<ContentDigest> {
        self.previous_receipt_digest
    }

    #[must_use]
    pub const fn receipt_digest(&self) -> ContentDigest {
        self.receipt_digest
    }
}

impl ProjectDescriptor {
    #[must_use]
    pub fn library_publications(&self) -> &[ProjectLibraryPublicationReceipt] {
        &self.library_publications
    }

    pub(crate) fn publish_library_snapshot(
        &mut self,
        draft: ProjectLibraryPublicationDraft,
    ) -> Result<ProjectLibraryPublicationReceipt, ProjectDescriptorError> {
        self.validate_library_publications()?;
        draft.validate()?;
        if self.library_publications.len() >= MAX_PROJECT_LIBRARY_PUBLICATION_RECEIPTS {
            return Err(ProjectDescriptorError::LibraryPublicationLimit {
                maximum: MAX_PROJECT_LIBRARY_PUBLICATION_RECEIPTS,
            });
        }
        if self
            .library_publications
            .iter()
            .any(|receipt| receipt.publication_id == draft.publication_id)
        {
            return Err(ProjectDescriptorError::LibraryPublicationCorrupted(
                "publication identity is already retained".to_owned(),
            ));
        }
        if self
            .library_publications
            .iter()
            .any(|receipt| receipt.label.eq_ignore_ascii_case(&draft.label))
        {
            return Err(ProjectDescriptorError::LibraryPublicationCorrupted(
                "publication label is already retained".to_owned(),
            ));
        }

        let receipt_project_revision = self.revision.next()?;
        let sequence = u64::try_from(self.library_publications.len())
            .ok()
            .and_then(|value| value.checked_add(1))
            .ok_or(ProjectDescriptorError::LibraryPublicationSequenceExhausted)?;
        let mut receipt = ProjectLibraryPublicationReceipt {
            schema_version: PROJECT_LIBRARY_PUBLICATION_RECEIPT_SCHEMA_VERSION,
            sequence,
            publication_id: draft.publication_id,
            project_id: self.id,
            label: draft.label,
            actor_id: draft.actor_id,
            authority_id: draft.authority_id,
            reason: draft.reason,
            created_unix_ms: draft.created_unix_ms,
            source_project_revision: self.revision,
            receipt_project_revision,
            library_revision: draft.library_revision,
            snapshot_digest: draft.snapshot_digest,
            snapshot_byte_len: draft.snapshot_byte_len,
            previous_receipt_digest: self
                .library_publications
                .last()
                .map(ProjectLibraryPublicationReceipt::receipt_digest),
            receipt_digest: ContentDigest::from_bytes([0; 32]),
        };
        receipt.receipt_digest = receipt.calculate_digest()?;

        let mut candidate = self.clone();
        candidate.revision = receipt_project_revision;
        candidate.library_publications.push(receipt.clone());
        candidate.validate()?;
        *self = candidate;
        Ok(receipt)
    }

    pub(super) fn validate_library_publications(&self) -> Result<(), ProjectDescriptorError> {
        if self.library_publications.len() > MAX_PROJECT_LIBRARY_PUBLICATION_RECEIPTS {
            return Err(ProjectDescriptorError::LibraryPublicationLimit {
                maximum: MAX_PROJECT_LIBRARY_PUBLICATION_RECEIPTS,
            });
        }

        let mut previous_receipt_digest = None;
        let mut previous_project_revision = None;
        let mut publication_ids = HashSet::with_capacity(self.library_publications.len());
        let mut labels = HashSet::with_capacity(self.library_publications.len());
        for (index, receipt) in self.library_publications.iter().enumerate() {
            let expected_sequence = u64::try_from(index)
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or(ProjectDescriptorError::LibraryPublicationSequenceExhausted)?;
            if receipt.schema_version != PROJECT_LIBRARY_PUBLICATION_RECEIPT_SCHEMA_VERSION {
                return Err(ProjectDescriptorError::LibraryPublicationCorrupted(
                    format!(
                        "receipt[{index}] schema {} is unsupported",
                        receipt.schema_version
                    ),
                ));
            }
            if receipt.sequence != expected_sequence {
                return Err(ProjectDescriptorError::LibraryPublicationCorrupted(
                    format!(
                        "receipt[{index}] sequence is {}, expected {expected_sequence}",
                        receipt.sequence
                    ),
                ));
            }
            if receipt.publication_id.is_nil()
                || receipt.project_id != self.id
                || !publication_ids.insert(receipt.publication_id)
            {
                return Err(ProjectDescriptorError::LibraryPublicationCorrupted(
                    format!("receipt[{index}] has an invalid or duplicated identity"),
                ));
            }
            if !labels.insert(receipt.label.to_lowercase()) {
                return Err(ProjectDescriptorError::LibraryPublicationCorrupted(
                    format!("receipt[{index}] has a duplicated label"),
                ));
            }
            for (field, value) in [
                ("publication.label", receipt.label.as_str()),
                ("publication.actor_id", receipt.actor_id.as_str()),
                ("publication.authority_id", receipt.authority_id.as_str()),
                ("publication.reason", receipt.reason.as_str()),
            ] {
                validate_library_audit_text(field, value)?;
            }
            if receipt.created_unix_ms == 0
                || receipt.snapshot_byte_len == 0
                || receipt.snapshot_byte_len > MAX_PROJECT_LIBRARY_PUBLICATION_BYTES
            {
                return Err(ProjectDescriptorError::LibraryPublicationCorrupted(
                    format!("receipt[{index}] has invalid artifact metadata"),
                ));
            }
            if receipt.source_project_revision.next()? != receipt.receipt_project_revision
                || receipt.receipt_project_revision > self.revision
            {
                return Err(ProjectDescriptorError::LibraryPublicationCorrupted(
                    format!("receipt[{index}] has invalid project revision lineage"),
                ));
            }
            if previous_project_revision
                .is_some_and(|previous| receipt.source_project_revision < previous)
            {
                return Err(ProjectDescriptorError::LibraryPublicationCorrupted(
                    format!("receipt[{index}] project revision regresses"),
                ));
            }
            if receipt.previous_receipt_digest != previous_receipt_digest
                || receipt.calculate_digest()? != receipt.receipt_digest
            {
                return Err(ProjectDescriptorError::LibraryPublicationCorrupted(
                    format!("receipt[{index}] has invalid content or chain linkage"),
                ));
            }
            previous_receipt_digest = Some(receipt.receipt_digest);
            previous_project_revision = Some(receipt.receipt_project_revision);
        }
        Ok(())
    }
}
