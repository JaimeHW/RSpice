//! Canonical browser binding metadata, restoration and generation admission.

use super::{BrowserBindingBackend, BrowserBindingReceipt};
use crate::lifecycle::ProjectLifecycleError;
use rspice_app_types::product::ContentDigest;

mod load;
pub use load::{BrowserRestoreCandidate, BrowserRestoreIssue, BrowserRestoreMetadataError};

mod write;
pub use write::{BrowserProjectStorage, BrowserWriteError, BrowserWriteOperation};

pub const BROWSER_BINDING_SCHEMA_VERSION: u32 = 2;
pub const MAX_EXACT_BROWSER_GENERATION: u64 = (1_u64 << 53) - 1;

pub const fn browser_generation_has_restart_authority(
    accepted_generation: u64,
    persisted_generation: Option<u64>,
) -> bool {
    match persisted_generation {
        Some(persisted) => persisted == accepted_generation,
        None => false,
    }
}

pub const fn persisted_generation_after_browser_write(
    durable: bool,
    accepted_generation: u64,
    prior_persisted_generation: Option<u64>,
) -> Option<u64> {
    if durable {
        Some(accepted_generation)
    } else {
        prior_persisted_generation
    }
}

/// Accepted browser byte identity and its independently persisted generation.
/// Live handles and permission checks belong to the host adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserBinding {
    pub receipt: BrowserBindingReceipt,
    pub display_name: String,
    /// Last generation committed to binding storage, which can lag behind
    /// verified canonical bytes when storage is unavailable.
    pub persisted_generation: Option<u64>,
}

impl BrowserBinding {
    pub fn durable_receipt(&self) -> Option<BrowserBindingReceipt> {
        browser_generation_has_restart_authority(
            self.receipt.accepted_generation,
            self.persisted_generation,
        )
        .then(|| self.receipt.clone())
    }

    pub fn prepare_write(
        &self,
        project_id: String,
    ) -> Result<BrowserWriteIntent, ProjectLifecycleError> {
        if self.receipt.project_id != project_id {
            return Err(ProjectLifecycleError::InvalidState(
                "browser binding belongs to a different logical project".to_owned(),
            ));
        }
        Ok(BrowserWriteIntent {
            binding_id: self.receipt.binding_id,
            backend: self.receipt.backend,
            project_id,
            accepted_generation: self.receipt.accepted_generation.saturating_add(1).max(1),
            expected_digest: Some(self.receipt.accepted_digest),
            persisted_generation: self.persisted_generation,
        })
    }
}

/// Byte-write expectations, separate from a live browser handle. Library
/// publication also uses these expectations without establishing a project binding.
#[derive(Debug, Clone)]
pub struct BrowserWriteIntent {
    pub binding_id: uuid::Uuid,
    pub backend: BrowserBindingBackend,
    pub project_id: String,
    pub accepted_generation: u64,
    pub expected_digest: Option<ContentDigest>,
    pub persisted_generation: Option<u64>,
}

impl BrowserWriteIntent {
    pub fn fresh(project_id: String, backend: BrowserBindingBackend) -> Self {
        Self {
            binding_id: uuid::Uuid::new_v4(),
            backend,
            project_id,
            accepted_generation: 1,
            expected_digest: None,
            persisted_generation: None,
        }
    }

    pub fn validate_generation(&self) -> Result<(), String> {
        if !browser_generation_is_exact(self.accepted_generation)
            || self
                .persisted_generation
                .is_some_and(|generation| !browser_generation_is_exact(generation))
        {
            return Err(
                "browser binding generation must be a nonzero JavaScript-exact integer".to_owned(),
            );
        }
        Ok(())
    }

    /// Match the host's verified publication to the exact prepared write.
    /// The caller must separately retain current transaction/context authority.
    pub fn accept_publication(
        &self,
        staged_digest: ContentDigest,
        receipt: BrowserBindingReceipt,
        display_name: String,
        durable: bool,
    ) -> Result<BrowserBinding, ProjectLifecycleError> {
        if receipt.binding_id != self.binding_id
            || receipt.backend != self.backend
            || receipt.project_id != self.project_id
            || receipt.accepted_generation != self.accepted_generation
            || receipt.accepted_digest != staged_digest
        {
            return Err(ProjectLifecycleError::InvalidState(
                "browser binding identity changed during save completion".to_owned(),
            ));
        }
        // A session-only publication retains the last durable generation so
        // the next save compares against the record actually in storage.
        let persisted_generation = persisted_generation_after_browser_write(
            durable,
            receipt.accepted_generation,
            self.persisted_generation,
        );
        Ok(BrowserBinding {
            receipt,
            display_name,
            persisted_generation,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserBindingMetadata {
    pub schema_version: u32,
    pub binding_id: String,
    pub project_id: String,
    pub accepted_generation: u64,
    pub accepted_digest: String,
    pub backend: BrowserBindingBackend,
    pub display_name: String,
}

pub fn validate_browser_restore_facts(
    metadata: &BrowserBindingMetadata,
    receipt: &BrowserBindingReceipt,
    permission: &str,
    actual_digest: ContentDigest,
) -> Result<ContentDigest, String> {
    let accepted = validate_browser_binding_metadata(metadata, receipt)?;
    if permission != "granted" {
        return Err(format!(
            "browser file permission is {permission}; select the canonical project again"
        ));
    }
    if actual_digest != accepted {
        return Err("canonical browser project changed outside RSpice".to_owned());
    }
    Ok(accepted)
}

pub fn validate_browser_binding_metadata(
    metadata: &BrowserBindingMetadata,
    receipt: &BrowserBindingReceipt,
) -> Result<ContentDigest, String> {
    if metadata.schema_version != BROWSER_BINDING_SCHEMA_VERSION {
        return Err(format!(
            "unsupported browser binding schema {}",
            metadata.schema_version
        ));
    }
    if metadata.binding_id != receipt.binding_id.to_string()
        || metadata.project_id != receipt.project_id
        || metadata.accepted_generation != receipt.accepted_generation
        || metadata.backend != receipt.backend
    {
        return Err("browser binding belongs to a different project identity".to_owned());
    }
    let accepted = metadata
        .accepted_digest
        .parse::<ContentDigest>()
        .map_err(|error| format!("browser binding digest is invalid: {error}"))?;
    if accepted != receipt.accepted_digest {
        return Err("browser binding receipt digest does not match IndexedDB".to_owned());
    }
    Ok(accepted)
}

pub fn validate_browser_binding_identity(
    metadata: &BrowserBindingMetadata,
    receipt: &BrowserBindingReceipt,
) -> Result<(), String> {
    if metadata.schema_version != BROWSER_BINDING_SCHEMA_VERSION {
        return Err(format!(
            "unsupported browser binding schema {}",
            metadata.schema_version
        ));
    }
    if metadata.binding_id != receipt.binding_id.to_string()
        || metadata.project_id != receipt.project_id
        || metadata.backend != receipt.backend
    {
        return Err("browser binding belongs to a different canonical identity".to_owned());
    }
    Ok(())
}

pub fn validate_binding_generation_commit(
    existing: Option<&BrowserBindingMetadata>,
    binding_id: uuid::Uuid,
    project_id: &str,
    backend: BrowserBindingBackend,
    expected_generation: Option<u64>,
    next_generation: u64,
) -> Result<(), String> {
    if !browser_generation_is_exact(next_generation)
        || expected_generation
            .is_some_and(|value| !browser_generation_is_exact(value) || next_generation <= value)
    {
        return Err("browser binding generation did not advance".to_owned());
    }
    match (existing, expected_generation) {
        (None, None) => Ok(()),
        (Some(metadata), Some(expected))
            if metadata.schema_version == BROWSER_BINDING_SCHEMA_VERSION
                && metadata.binding_id == binding_id.to_string()
                && metadata.project_id == project_id
                && metadata.backend == backend
                && metadata.accepted_generation == expected =>
        {
            Ok(())
        }
        _ => Err(
            "browser binding generation changed in another tab; reopen it or save a project copy"
                .to_owned(),
        ),
    }
}

pub const fn browser_generation_is_exact(generation: u64) -> bool {
    generation > 0 && generation <= MAX_EXACT_BROWSER_GENERATION
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserPermissionDecision {
    Verify,
    Reconnect,
}

pub fn browser_permission_decision(permission: &str) -> BrowserPermissionDecision {
    match permission {
        "granted" => BrowserPermissionDecision::Verify,
        _ => BrowserPermissionDecision::Reconnect,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowserBindingCommitOutcome {
    Durable,
    SessionOnly(String),
}

pub fn classify_browser_binding_commit(result: Result<(), String>) -> BrowserBindingCommitOutcome {
    match result {
        Ok(()) => BrowserBindingCommitOutcome::Durable,
        Err(error) => BrowserBindingCommitOutcome::SessionOnly(error),
    }
}

pub fn browser_backend_name(backend: BrowserBindingBackend) -> &'static str {
    match backend {
        BrowserBindingBackend::ExternalFile => "external-file",
        BrowserBindingBackend::Opfs => "opfs",
    }
}

pub fn browser_backend_from_name(value: &str) -> Result<BrowserBindingBackend, String> {
    match value {
        "external-file" => Ok(BrowserBindingBackend::ExternalFile),
        "opfs" => Ok(BrowserBindingBackend::Opfs),
        _ => Err(format!("unknown browser binding backend {value}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::digest_bytes;

    fn browser_receipt(project_id: &str, digest: ContentDigest) -> BrowserBindingReceipt {
        BrowserBindingReceipt {
            binding_id: uuid::Uuid::from_u128(0x7d9a_1db3_55f2_4da2_82e1_992f_6e65_0f42),
            project_id: project_id.to_owned(),
            accepted_generation: 7,
            accepted_digest: digest,
            backend: BrowserBindingBackend::ExternalFile,
        }
    }

    fn browser_metadata(
        receipt: &BrowserBindingReceipt,
        digest: ContentDigest,
    ) -> BrowserBindingMetadata {
        BrowserBindingMetadata {
            schema_version: BROWSER_BINDING_SCHEMA_VERSION,
            binding_id: receipt.binding_id.to_string(),
            project_id: receipt.project_id.clone(),
            accepted_generation: receipt.accepted_generation,
            accepted_digest: digest.to_string(),
            backend: receipt.backend,
            display_name: "project.rspiceproj".to_owned(),
        }
    }

    #[test]
    fn browser_restore_protocol_distinguishes_prompt_from_revocation() {
        assert_eq!(
            browser_permission_decision("granted"),
            BrowserPermissionDecision::Verify
        );
        assert_eq!(
            browser_permission_decision("prompt"),
            BrowserPermissionDecision::Reconnect
        );
        assert_eq!(
            browser_permission_decision("denied"),
            BrowserPermissionDecision::Reconnect
        );
        assert_eq!(
            browser_permission_decision("unexpected"),
            BrowserPermissionDecision::Reconnect
        );
    }

    #[test]
    fn browser_restore_protocol_rejects_wrong_schema_identity_and_digest() {
        let accepted = digest_bytes(b"accepted");
        let receipt = browser_receipt("project-id", accepted);
        let metadata = browser_metadata(&receipt, accepted);
        validate_browser_binding_identity(&metadata, &receipt)
            .expect("structural browser binding identity matches");
        assert_eq!(
            validate_browser_restore_facts(&metadata, &receipt, "granted", accepted).unwrap(),
            accepted
        );

        let mut wrong_schema = metadata.clone();
        wrong_schema.schema_version += 1;
        assert!(validate_browser_binding_metadata(&wrong_schema, &receipt).is_err());

        let mut wrong_identity = metadata.clone();
        wrong_identity.project_id = "other-project".to_owned();
        assert!(validate_browser_binding_metadata(&wrong_identity, &receipt).is_err());

        let mut newer_generation = metadata.clone();
        newer_generation.accepted_generation += 1;
        assert!(validate_browser_binding_metadata(&newer_generation, &receipt).is_err());

        let mut other_backend = metadata.clone();
        other_backend.backend = BrowserBindingBackend::Opfs;
        assert!(validate_browser_binding_identity(&other_backend, &receipt).is_err());
        assert!(validate_browser_binding_metadata(&other_backend, &receipt).is_err());

        assert!(
            validate_browser_restore_facts(
                &metadata,
                &receipt,
                "granted",
                digest_bytes(b"external change"),
            )
            .is_err()
        );
        assert!(validate_browser_restore_facts(&metadata, &receipt, "denied", accepted,).is_err());
    }

    #[test]
    fn browser_binding_generation_compare_exchange_is_fail_closed() {
        let digest = digest_bytes(b"accepted");
        let receipt = browser_receipt("project-id", digest);
        let metadata = browser_metadata(&receipt, digest);

        validate_binding_generation_commit(
            Some(&metadata),
            receipt.binding_id,
            &receipt.project_id,
            receipt.backend,
            Some(7),
            8,
        )
        .expect("matching generation advances");
        validate_binding_generation_commit(
            None,
            receipt.binding_id,
            &receipt.project_id,
            receipt.backend,
            None,
            1,
        )
        .expect("a fresh opaque binding can commit generation one");

        for (existing, expected, next) in [
            (Some(&metadata), Some(6), 8),
            (Some(&metadata), None, 8),
            (None, Some(7), 8),
            (Some(&metadata), Some(7), 7),
            (None, None, MAX_EXACT_BROWSER_GENERATION + 1),
            (
                Some(&metadata),
                Some(MAX_EXACT_BROWSER_GENERATION + 1),
                MAX_EXACT_BROWSER_GENERATION + 1,
            ),
        ] {
            assert!(
                validate_binding_generation_commit(
                    existing,
                    receipt.binding_id,
                    &receipt.project_id,
                    receipt.backend,
                    expected,
                    next,
                )
                .is_err()
            );
        }
    }

    #[test]
    fn verified_write_with_binding_failure_is_typed_session_only_success() {
        assert_eq!(
            classify_browser_binding_commit(Ok(())),
            BrowserBindingCommitOutcome::Durable
        );
        assert_eq!(
            classify_browser_binding_commit(Err("quota".to_owned())),
            BrowserBindingCommitOutcome::SessionOnly("quota".to_owned())
        );
    }

    #[test]
    fn session_only_browser_write_retains_last_durable_generation_without_restart_authority() {
        let accepted = BrowserBinding {
            receipt: browser_receipt("project-id", digest_bytes(b"generation 7")),
            display_name: "project.rspiceproj".to_owned(),
            persisted_generation: Some(7),
        };
        let next = accepted.prepare_write("project-id".to_owned()).unwrap();
        let written = BrowserBindingReceipt {
            accepted_generation: 8,
            accepted_digest: digest_bytes(b"generation 8"),
            ..accepted.receipt.clone()
        };
        let session = next
            .accept_publication(
                written.accepted_digest,
                written.clone(),
                accepted.display_name.clone(),
                false,
            )
            .unwrap();
        assert_eq!(session.persisted_generation, Some(7));
        assert!(session.durable_receipt().is_none());

        let retry = session.prepare_write("project-id".to_owned()).unwrap();
        assert_eq!(retry.accepted_generation, 9);
        assert_eq!(retry.expected_digest, Some(written.accepted_digest));
        assert_eq!(retry.persisted_generation, Some(7));
        let committed = BrowserBindingReceipt {
            accepted_generation: 9,
            accepted_digest: digest_bytes(b"generation 9"),
            ..written
        };
        let durable = retry
            .accept_publication(
                committed.accepted_digest,
                committed.clone(),
                session.display_name,
                true,
            )
            .unwrap();
        assert_eq!(durable.durable_receipt(), Some(committed));

        let fresh = BrowserWriteIntent::fresh("copy-id".to_owned(), BrowserBindingBackend::Opfs);
        fresh.validate_generation().unwrap();
        assert_eq!(fresh.accepted_generation, 1);
        assert_eq!(fresh.expected_digest, None);
        assert_ne!(fresh.binding_id, accepted.receipt.binding_id);
        let fresh_receipt = BrowserBindingReceipt {
            binding_id: fresh.binding_id,
            project_id: fresh.project_id.clone(),
            accepted_generation: 1,
            accepted_digest: digest_bytes(b"first save"),
            backend: fresh.backend,
        };
        let session = fresh
            .accept_publication(
                fresh_receipt.accepted_digest,
                fresh_receipt,
                "copy.rspiceproj".to_owned(),
                false,
            )
            .unwrap();
        assert_eq!(session.persisted_generation, None);
        assert!(session.durable_receipt().is_none());
    }

    #[test]
    fn browser_save_rejects_cross_binding_completions_and_unrepresentable_generations() {
        let binding = BrowserBinding {
            receipt: browser_receipt("project-id", digest_bytes(b"accepted")),
            display_name: "project.rspiceproj".to_owned(),
            persisted_generation: Some(7),
        };
        assert!(binding.prepare_write("other-project".to_owned()).is_err());
        let target = binding.prepare_write("project-id".to_owned()).unwrap();
        let staged_digest = digest_bytes(b"staged");
        let published = BrowserBindingReceipt {
            accepted_generation: 8,
            accepted_digest: staged_digest,
            ..binding.receipt.clone()
        };
        let mut mismatches = vec![published.clone(); 5];
        mismatches[0].binding_id = uuid::Uuid::nil();
        mismatches[1].backend = BrowserBindingBackend::Opfs;
        mismatches[2].project_id = "other-project".to_owned();
        mismatches[3].accepted_generation = 7;
        mismatches[4].accepted_digest = binding.receipt.accepted_digest;
        for receipt in mismatches {
            assert!(matches!(
                target.accept_publication(
                    staged_digest,
                    receipt,
                    binding.display_name.clone(),
                    true
                ),
                Err(ProjectLifecycleError::InvalidState(_))
            ));
        }
        assert_eq!(binding.durable_receipt(), Some(binding.receipt.clone()));
        for generation in [0, MAX_EXACT_BROWSER_GENERATION + 1, u64::MAX] {
            let mut invalid = target.clone();
            invalid.accepted_generation = generation;
            assert!(invalid.validate_generation().is_err());
            invalid.accepted_generation = 8;
            invalid.persisted_generation = Some(generation);
            assert!(invalid.validate_generation().is_err());
        }
        target.validate_generation().unwrap();
        assert_eq!(
            target
                .accept_publication(staged_digest, published.clone(), binding.display_name, true)
                .unwrap()
                .durable_receipt(),
            Some(published)
        );
    }

    #[test]
    fn mismatched_browser_record_never_establishes_eviction_ownership() {
        let digest = digest_bytes(b"accepted");
        let receipt = browser_receipt("project-a", digest);
        let mut other_project = browser_metadata(&receipt, digest);
        other_project.project_id = "project-b".to_owned();
        assert!(validate_browser_binding_identity(&other_project, &receipt).is_err());

        let mut other_binding = browser_metadata(&receipt, digest);
        other_binding.binding_id = uuid::Uuid::new_v4().to_string();
        assert!(validate_browser_binding_identity(&other_binding, &receipt).is_err());

        let owned = browser_metadata(&receipt, digest);
        validate_browser_binding_identity(&owned, &receipt)
            .expect("all durable identity fields prove record ownership");
    }
}
