//! Canonical browser content admission, separate from handles and storage APIs.

use super::*;
use crate::persistence::ProjectBytes;
use crate::{DecodedProject, decode_project_text};
use rspice_design::hierarchy::HierarchySourceFiles;

impl BrowserBinding {
    pub fn open(
        bytes: ProjectBytes,
        display_name: String,
        source_files: impl HierarchySourceFiles,
    ) -> Result<(DecodedProject, Self), String> {
        let digest = bytes.digest();
        let text = bytes
            .into_text()
            .map_err(|error| format!("selected project is not valid UTF-8: {error}"))?;
        let project =
            decode_project_text(&text, None, source_files).map_err(|error| error.to_string())?;
        let binding = Self {
            receipt: BrowserBindingReceipt {
                binding_id: uuid::Uuid::new_v4(),
                backend: BrowserBindingBackend::ExternalFile,
                project_id: project.file.workspace.project.id().to_string(),
                accepted_generation: 1,
                accepted_digest: digest,
            },
            display_name,
            persisted_generation: None,
        };
        Ok((project, binding))
    }
}

/// Only an owned invalid record may be evicted from binding storage.
#[derive(Debug)]
pub enum BrowserRestoreMetadataError {
    Unowned(String),
    InvalidOwned(String),
}

#[derive(Debug)]
pub enum BrowserRestoreIssue {
    ReconnectRequired(Box<BrowserBinding>),
    Conflict {
        binding: Box<BrowserBinding>,
        observed_digest: ContentDigest,
        reason: String,
    },
    Retryable(String),
}

/// Metadata ownership is established before the host accesses or evicts its
/// handle. Permission and byte checks retain the session's exact receipt.
#[derive(Debug)]
pub struct BrowserRestoreCandidate {
    metadata: BrowserBindingMetadata,
    receipt: BrowserBindingReceipt,
    metadata_digest: ContentDigest,
}

impl BrowserRestoreCandidate {
    pub fn new(
        metadata: BrowserBindingMetadata,
        receipt: &BrowserBindingReceipt,
    ) -> Result<Self, BrowserRestoreMetadataError> {
        validate_browser_binding_identity(&metadata, receipt)
            .map_err(BrowserRestoreMetadataError::Unowned)?;
        let metadata_digest =
            metadata
                .accepted_digest
                .parse::<ContentDigest>()
                .map_err(|error| {
                    BrowserRestoreMetadataError::InvalidOwned(format!(
                        "browser binding digest is invalid: {error}"
                    ))
                })?;
        Ok(Self {
            metadata,
            receipt: receipt.clone(),
            metadata_digest,
        })
    }

    fn binding_from_receipt(&self) -> Box<BrowserBinding> {
        Box::new(BrowserBinding {
            receipt: self.receipt.clone(),
            display_name: self.metadata.display_name.clone(),
            persisted_generation: Some(self.metadata.accepted_generation),
        })
    }

    /// Generation conflict takes precedence over a reconnect decision, after
    /// the host has finished querying permission and before it reads bytes.
    pub fn check_permission(&self, permission: &str) -> Result<(), BrowserRestoreIssue> {
        if validate_browser_binding_metadata(&self.metadata, &self.receipt).is_err() {
            return Err(BrowserRestoreIssue::Conflict {
                binding: self.binding_from_receipt(),
                observed_digest: self.metadata_digest,
                reason: format!(
                    "another tab committed generation {} while this session accepted generation {}",
                    self.metadata.accepted_generation, self.receipt.accepted_generation,
                ),
            });
        }
        if browser_permission_decision(permission) == BrowserPermissionDecision::Reconnect {
            return Err(BrowserRestoreIssue::ReconnectRequired(
                self.binding_from_receipt(),
            ));
        }
        Ok(())
    }

    pub fn decode(
        self,
        bytes: Vec<u8>,
        permission: &str,
        source_files: impl HierarchySourceFiles,
    ) -> Result<(DecodedProject, BrowserBinding), BrowserRestoreIssue> {
        let bytes = ProjectBytes::from_bytes(bytes).map_err(|_| {
            BrowserRestoreIssue::Retryable(
                "browser project exceeds the supported project-size limit".to_owned(),
            )
        })?;
        let actual_digest = bytes.digest();
        let accepted_digest = validate_browser_restore_facts(
            &self.metadata,
            &self.receipt,
            permission,
            actual_digest,
        )
        .map_err(|reason| BrowserRestoreIssue::Conflict {
            binding: self.binding_from_receipt(),
            observed_digest: actual_digest,
            reason,
        })?;
        let text = bytes.into_text().map_err(|error| {
            BrowserRestoreIssue::Retryable(format!(
                "canonical browser project is not UTF-8: {error}"
            ))
        })?;
        let mut baseline = decode_project_text(&text, None, source_files).map_err(|error| {
            BrowserRestoreIssue::Retryable(format!("canonical browser project is invalid: {error}"))
        })?;
        if baseline.file.workspace.project.id().to_string() != self.receipt.project_id {
            return Err(BrowserRestoreIssue::Conflict {
                binding: self.binding_from_receipt(),
                observed_digest: actual_digest,
                reason: "canonical browser project identity no longer matches its binding"
                    .to_owned(),
            });
        }
        baseline.file.workspace.project.path = None;
        let binding = BrowserBinding {
            receipt: BrowserBindingReceipt {
                accepted_digest,
                ..self.receipt.clone()
            },
            display_name: self.metadata.display_name,
            persisted_generation: Some(self.receipt.accepted_generation),
        };
        Ok((baseline, binding))
    }
}

#[cfg(test)]
mod tests;
