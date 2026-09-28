//! Canonical browser binding metadata, restoration and generation admission.

use super::{BrowserBindingBackend, BrowserBindingReceipt};
use rspice_app_types::product::ContentDigest;

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
        assert_eq!(
            persisted_generation_after_browser_write(false, 8, Some(7)),
            Some(7)
        );
        assert!(!browser_generation_has_restart_authority(8, Some(7)));
        assert_eq!(
            persisted_generation_after_browser_write(false, 1, None),
            None
        );
        assert!(!browser_generation_has_restart_authority(1, None));

        assert_eq!(
            persisted_generation_after_browser_write(true, 8, Some(7)),
            Some(8)
        );
        assert!(browser_generation_has_restart_authority(8, Some(8)));
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
