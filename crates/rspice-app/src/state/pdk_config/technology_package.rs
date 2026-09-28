//! Signed, content-addressed PDK technology-package administration.
//!
//! A configured model search path is not a technology package.  This module
//! keeps those concepts separate and gives the PDK administration surface a
//! real authority boundary:
//!
//! - the publisher signature covers the exact manifest bytes;
//! - every packaged artifact is size- and digest-verified;
//! - layer, purpose, stream-map, connectivity, callback, and platform
//!   contracts are validated before installation;
//! - installed revisions are immutable;
//! - activation and rollback are append-only, hash-chained transactions; and
//! - deserialized packages are never executable until they are revalidated
//!   against the current trust store.

#[cfg(target_arch = "wasm32")]
use std::collections::BTreeSet;

#[cfg(test)]
use base64::{Engine as _, engine::general_purpose::STANDARD};
#[cfg(test)]
use rspice_model_library::pdk::manifest::{signed_model_virtual_root, validate_package_path};
#[cfg(test)]
use rspice_model_library::pdk::package::authenticate_archive;
use serde::{Deserialize, Serialize};

use rspice_simulation::pdk::{SealedPdkModelSources, validate_runtime_compatibility};
pub use rspice_simulation::pdk::{
    ValidatedPdkTechnologyPackage, validate_archive, validate_archive_bytes,
};

use crate::product::ContentDigest;
pub use rspice_model_library::pdk::contracts::*;
pub use rspice_model_library::pdk::manifest::PdkTechnologyManifest;
pub(super) use rspice_model_library::pdk::manifest::validate_manifest;
use rspice_model_library::pdk::manifest::validate_version;
use rspice_model_library::pdk::package::decode_bounded;
pub use rspice_model_library::pdk::{
    PdkAdministrativeAuthority, PdkPublisherTrustStore, PdkTechnologyError, PdkTrustAuditAction,
    PdkTrustAuditReceipt, TrustedPdkPublisherKey,
};
use rspice_model_library::pdk::{content_digest, validate_identifier, validate_text};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdkTechnologyAuditAction {
    Install,
    Activate,
    Rollback,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyAuditReceipt {
    pub sequence: u64,
    pub action: PdkTechnologyAuditAction,
    pub actor_id: String,
    pub authority_id: String,
    pub reason: String,
    pub target: PdkTechnologyBinding,
    pub archive_digest: ContentDigest,
    pub before_active: Option<PdkTechnologyBinding>,
    pub after_active: Option<PdkTechnologyBinding>,
    pub previous_receipt_digest: Option<ContentDigest>,
    pub receipt_digest: ContentDigest,
}

#[derive(Serialize)]
struct PdkTechnologyAuditPayload<'a> {
    sequence: u64,
    action: PdkTechnologyAuditAction,
    actor_id: &'a str,
    authority_id: &'a str,
    reason: &'a str,
    target: &'a PdkTechnologyBinding,
    archive_digest: ContentDigest,
    before_active: &'a Option<PdkTechnologyBinding>,
    after_active: &'a Option<PdkTechnologyBinding>,
    previous_receipt_digest: Option<ContentDigest>,
}

impl PdkTechnologyAuditReceipt {
    fn calculate_digest(&self) -> Result<ContentDigest, PdkTechnologyError> {
        let payload = PdkTechnologyAuditPayload {
            sequence: self.sequence,
            action: self.action,
            actor_id: &self.actor_id,
            authority_id: &self.authority_id,
            reason: &self.reason,
            target: &self.target,
            archive_digest: self.archive_digest,
            before_active: &self.before_active,
            after_active: &self.after_active,
            previous_receipt_digest: self.previous_receipt_digest,
        };
        let bytes = serde_json::to_vec(&payload)
            .map_err(|error| PdkTechnologyError::Serialization(error.to_string()))?;
        Ok(content_digest(&bytes))
    }
}

/// Persisted signed archives and administrative history.  `validated_packages`
/// is intentionally runtime-only: loading serialized state cannot restore
/// trust.  Call `revalidate_installed` with the current trust store before any
/// package can be activated or consumed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyRegistry {
    #[serde(default)]
    archives: Vec<SignedPdkTechnologyArchive>,
    #[serde(default)]
    active: Option<PdkTechnologyBinding>,
    #[serde(default)]
    audit: Vec<PdkTechnologyAuditReceipt>,
    #[serde(skip)]
    validated_packages: Vec<ValidatedPdkTechnologyPackage>,
    #[serde(skip)]
    validation_errors: Vec<String>,
}

impl PdkTechnologyRegistry {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn take_worker_validated_packages(&mut self) -> Vec<ValidatedPdkTechnologyPackage> {
        std::mem::take(&mut self.validated_packages)
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn restore_worker_validated_packages(
        &mut self,
        packages: Vec<ValidatedPdkTechnologyPackage>,
    ) -> Result<(), PdkTechnologyError> {
        if !self.validated_packages.is_empty() || !self.validation_errors.is_empty() {
            return Err(PdkTechnologyError::NotRuntimeValidated(
                "worker package publication requires an empty runtime validation cache".to_owned(),
            ));
        }
        if packages.len() != self.archives.len() {
            return Err(PdkTechnologyError::NotRuntimeValidated(format!(
                "worker validated {} packages for {} installed archives",
                packages.len(),
                self.archives.len()
            )));
        }
        let mut identities = BTreeSet::new();
        for package in &packages {
            let binding = package.binding();
            if !identities.insert((
                binding.package_id.to_ascii_lowercase(),
                binding.revision.clone(),
                binding.manifest_digest,
            )) {
                return Err(PdkTechnologyError::NotRuntimeValidated(
                    "worker returned duplicate validated package identities".to_owned(),
                ));
            }
            if !self.audit.iter().any(|receipt| {
                receipt.target == binding && receipt.archive_digest == package.archive_digest()
            }) {
                return Err(PdkTechnologyError::NotRuntimeValidated(format!(
                    "worker package {} {} has no matching authenticated installation receipt",
                    binding.package_id, binding.revision
                )));
            }
        }
        self.validated_packages = packages;
        Ok(())
    }

    #[must_use]
    pub fn archives(&self) -> &[SignedPdkTechnologyArchive] {
        &self.archives
    }

    /// Resolve the immutable archive behind an exact currently validated
    /// package. This is used by the unsigned authoring export to retain source
    /// artifact bytes without granting the draft executable authority.
    #[must_use]
    pub fn archive_for_package(
        &self,
        package: &ValidatedPdkTechnologyPackage,
    ) -> Option<&SignedPdkTechnologyArchive> {
        self.archives.iter().find(|archive| {
            serde_json::to_vec(archive)
                .ok()
                .is_some_and(|bytes| content_digest(&bytes) == package.archive_digest())
        })
    }

    /// Remove the signed archive payloads from a persistence clone while
    /// retaining its bindings and audit history. Browser persistence stores
    /// these potentially large immutable payloads in a content-addressed
    /// object store instead of embedding them in one configuration record.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub(super) fn take_archives_for_browser_persistence(
        &mut self,
    ) -> Vec<SignedPdkTechnologyArchive> {
        self.validated_packages.clear();
        self.validation_errors.clear();
        std::mem::take(&mut self.archives)
    }

    /// Reattach exact signed archive payloads loaded from the browser object
    /// store. This deliberately restores no runtime trust; the caller must
    /// run `revalidate_installed` against the current publisher trust store.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub(super) fn restore_archives_from_browser_persistence(
        &mut self,
        archives: Vec<SignedPdkTechnologyArchive>,
    ) -> Result<(), PdkTechnologyError> {
        if !self.archives.is_empty() {
            return Err(PdkTechnologyError::AuditCorrupted(
                "browser PDK metadata unexpectedly contains embedded archives".to_owned(),
            ));
        }
        if archives.len() > MAX_PDK_ARTIFACTS {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "technology package registry is limited to {MAX_PDK_ARTIFACTS} installed revisions"
            )));
        }
        self.archives = archives;
        self.validated_packages.clear();
        self.validation_errors.clear();
        Ok(())
    }

    #[must_use]
    pub fn active_binding(&self) -> Option<&PdkTechnologyBinding> {
        self.active.as_ref()
    }

    #[must_use]
    pub fn audit(&self) -> &[PdkTechnologyAuditReceipt] {
        &self.audit
    }

    #[must_use]
    pub fn validated_packages(&self) -> &[ValidatedPdkTechnologyPackage] {
        &self.validated_packages
    }

    #[must_use]
    pub fn validation_errors(&self) -> &[String] {
        &self.validation_errors
    }

    #[must_use]
    pub fn active_package(&self) -> Option<&ValidatedPdkTechnologyPackage> {
        let active = self.active.as_ref()?;
        self.validated_packages.iter().find(|package| {
            package
                .manifest()
                .package_id
                .eq_ignore_ascii_case(&active.package_id)
                && package.manifest().revision == active.revision
                && package.manifest_digest() == active.manifest_digest
        })
    }

    /// Seal the exact signed model-source closure named by a project pin.
    ///
    /// The project binding, manifest digest, archive digest, decoded artifact
    /// bytes, package-relative dependency graph, and process-section contract
    /// are all checked again before runtime source authority is returned.
    /// This method never falls back to the administratively active revision.
    pub(crate) fn seal_model_sources_for_binding(
        &self,
        binding: &PdkTechnologyBinding,
        expected_archive_digest: ContentDigest,
    ) -> Result<SealedPdkModelSources, PdkTechnologyError> {
        self.validate_audit_chain()?;
        if !self.validation_errors.is_empty() {
            return Err(PdkTechnologyError::NotRuntimeValidated(
                self.validation_errors.join("; "),
            ));
        }
        let package = self
            .validated_packages
            .iter()
            .find(|package| package.binding() == *binding)
            .ok_or_else(|| {
                PdkTechnologyError::NotRuntimeValidated(format!(
                    "{} {} does not resolve to its exact currently trusted manifest",
                    binding.package_id, binding.revision
                ))
            })?;
        if package.archive_digest() != expected_archive_digest {
            return Err(PdkTechnologyError::NotRuntimeValidated(format!(
                "{} {} resolves to archive {}, not the project-pinned archive {}",
                binding.package_id,
                binding.revision,
                package.archive_digest(),
                expected_archive_digest
            )));
        }
        package.runtime_compatibility().map_err(|detail| {
            PdkTechnologyError::IncompatibleRuntime(format!(
                "{} {}: {detail}",
                binding.package_id, binding.revision
            ))
        })?;

        let matching_archives = self
            .archives
            .iter()
            .filter(|archive| {
                archive_identity(archive).is_ok_and(|candidate| candidate == *binding)
                    && serde_json::to_vec(*archive)
                        .map(|bytes| content_digest(&bytes) == expected_archive_digest)
                        .unwrap_or(false)
            })
            .collect::<Vec<_>>();
        let archive = match matching_archives.as_slice() {
            [archive] => *archive,
            [] => {
                return Err(PdkTechnologyError::NotRuntimeValidated(format!(
                    "{} {} has no exact installed archive for project execution",
                    binding.package_id, binding.revision
                )));
            }
            _ => {
                return Err(PdkTechnologyError::AuditCorrupted(format!(
                    "{} {} resolves to more than one identical archive",
                    binding.package_id, binding.revision
                )));
            }
        };
        package.seal_model_sources(archive)
    }

    /// Execute one callback from the exact currently trusted archive selected
    /// by a project binding. Administrative activation is deliberately not an
    /// authority source for callback execution.
    pub fn execute_callback_for_binding(
        &self,
        binding: &PdkTechnologyBinding,
        expected_archive_digest: ContentDigest,
        callback_id: &str,
        input: &super::technology_callback::PdkCallbackExecutionInput,
    ) -> Result<
        super::technology_callback::PdkCallbackExecutionReceipt,
        super::technology_callback::PdkCallbackError,
    > {
        self.validate_audit_chain()?;
        if !self.validation_errors.is_empty() {
            return Err(
                PdkTechnologyError::NotRuntimeValidated(self.validation_errors.join("; ")).into(),
            );
        }
        let package = self
            .validated_packages
            .iter()
            .find(|package| package.binding() == *binding)
            .ok_or_else(|| {
                PdkTechnologyError::NotRuntimeValidated(format!(
                    "{} {} does not resolve to its exact currently trusted manifest",
                    binding.package_id, binding.revision
                ))
            })?;
        if package.archive_digest() != expected_archive_digest {
            return Err(PdkTechnologyError::NotRuntimeValidated(format!(
                "{} {} resolves to archive {}, not the project-pinned archive {}",
                binding.package_id,
                binding.revision,
                package.archive_digest(),
                expected_archive_digest
            ))
            .into());
        }
        package.runtime_compatibility().map_err(|detail| {
            PdkTechnologyError::IncompatibleRuntime(format!(
                "{} {}: {detail}",
                binding.package_id, binding.revision
            ))
        })?;
        let matching_archives = self
            .archives
            .iter()
            .filter(|archive| {
                archive_identity(archive).is_ok_and(|candidate| candidate == *binding)
                    && serde_json::to_vec(*archive)
                        .map(|bytes| content_digest(&bytes) == expected_archive_digest)
                        .unwrap_or(false)
            })
            .collect::<Vec<_>>();
        let archive = match matching_archives.as_slice() {
            [archive] => *archive,
            [] => {
                return Err(PdkTechnologyError::NotRuntimeValidated(format!(
                    "{} {} has no exact installed archive for callback execution",
                    binding.package_id, binding.revision
                ))
                .into());
            }
            _ => {
                return Err(PdkTechnologyError::AuditCorrupted(format!(
                    "{} {} resolves to more than one identical callback archive",
                    binding.package_id, binding.revision
                ))
                .into());
            }
        };
        package.execute_callback(archive, callback_id, input)
    }

    #[must_use]
    pub fn runtime_ready(&self) -> bool {
        self.validation_errors.is_empty()
            && (self.active.is_none() || self.active_package().is_some())
    }

    pub fn install_archive_bytes(
        &mut self,
        bytes: &[u8],
        trust_store: &PdkPublisherTrustStore,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkTechnologyAuditReceipt, PdkTechnologyError> {
        authority.validate()?;
        validate_text("reason", reason, 1_024)?;
        self.validate_audit_chain()?;
        let (archive, package) = validate_archive_bytes(bytes, trust_store)?;
        let binding = package.binding();
        if self.archives.iter().any(|installed| {
            archive_identity(installed).is_ok_and(|candidate| {
                candidate
                    .package_id
                    .eq_ignore_ascii_case(&binding.package_id)
                    && candidate.revision == binding.revision
            })
        }) {
            return Err(PdkTechnologyError::ImmutableRevision(format!(
                "{} {} is already installed; signed revisions are immutable",
                binding.package_id, binding.revision
            )));
        }
        if self.archives.len() >= MAX_PDK_ARTIFACTS {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "technology package registry is limited to {MAX_PDK_ARTIFACTS} installed revisions"
            )));
        }

        let receipt = self.next_receipt(
            PdkTechnologyAuditAction::Install,
            authority,
            reason,
            binding,
            package.archive_digest(),
            self.active.clone(),
            self.active.clone(),
        )?;
        self.archives.push(archive);
        self.validated_packages.push(package);
        self.sort_packages();
        self.audit.push(receipt.clone());
        Ok(receipt)
    }

    /// Rebuild all runtime trust from the exact persisted archives.  Failure
    /// publishes neither a partial validated catalog nor executable authority.
    pub fn revalidate_installed(
        &mut self,
        trust_store: &PdkPublisherTrustStore,
    ) -> Result<usize, Vec<String>> {
        if let Err(error) = trust_store.validate() {
            self.validated_packages.clear();
            self.validation_errors = vec![error.to_string()];
            return Err(self.validation_errors.clone());
        }
        if let Err(error) = self.validate_audit_chain() {
            self.validated_packages.clear();
            self.validation_errors = vec![error.to_string()];
            return Err(self.validation_errors.clone());
        }
        let mut packages = Vec::with_capacity(self.archives.len());
        let mut errors = Vec::new();
        for (index, archive) in self.archives.iter().enumerate() {
            match validate_archive(archive, trust_store) {
                Ok(package) => packages.push(package),
                Err(error) => errors.push(format!("archive[{index}]: {error}")),
            }
        }
        if errors.is_empty() {
            for (index, receipt) in self.audit.iter().enumerate() {
                let Some(package) = packages
                    .iter()
                    .find(|package| package.binding() == receipt.target)
                else {
                    errors.push(format!(
                        "receipt[{index}] references package {} {} that is not installed with its exact signed manifest",
                        receipt.target.package_id, receipt.target.revision
                    ));
                    continue;
                };
                if receipt.archive_digest != package.archive_digest() {
                    errors.push(format!(
                        "receipt[{index}] archive digest does not match package {} {}",
                        receipt.target.package_id, receipt.target.revision
                    ));
                }
            }
        }
        if errors.is_empty() {
            packages.sort_by(package_order);
            if let Some(active) = &self.active {
                match packages
                    .iter()
                    .find(|package| package.binding() == *active)
                {
                    None => errors.push(format!(
                        "active binding {} {} does not resolve to an exact currently trusted package",
                        active.package_id, active.revision
                    )),
                    Some(package) => {
                        if let Err(error) = validate_runtime_compatibility(package.manifest()) {
                            errors.push(format!(
                                "active binding {} {} is incompatible: {error}",
                                active.package_id, active.revision
                            ));
                        }
                    }
                }
            }
        }
        if errors.is_empty() {
            self.validated_packages = packages;
            self.validation_errors.clear();
            Ok(self.validated_packages.len())
        } else {
            self.validated_packages.clear();
            self.validation_errors = errors.clone();
            Err(errors)
        }
    }

    pub fn activate(
        &mut self,
        package_id: &str,
        revision: &str,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkTechnologyAuditReceipt, PdkTechnologyError> {
        self.activate_as(
            PdkTechnologyAuditAction::Activate,
            package_id,
            revision,
            authority,
            reason,
        )
    }

    pub fn rollback_to(
        &mut self,
        package_id: &str,
        revision: &str,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkTechnologyAuditReceipt, PdkTechnologyError> {
        let appeared_before = self.audit.iter().any(|receipt| {
            receipt.after_active.as_ref().is_some_and(|binding| {
                binding.package_id.eq_ignore_ascii_case(package_id) && binding.revision == revision
            })
        });
        if !appeared_before {
            return Err(PdkTechnologyError::InvalidTransition(format!(
                "{package_id} {revision} has never been an active trusted binding"
            )));
        }
        self.activate_as(
            PdkTechnologyAuditAction::Rollback,
            package_id,
            revision,
            authority,
            reason,
        )
    }

    pub fn validate_audit_chain(&self) -> Result<(), PdkTechnologyError> {
        if self.audit.len() > MAX_PDK_AUDIT_RECEIPTS {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "technology audit exceeds {MAX_PDK_AUDIT_RECEIPTS} receipts"
            )));
        }
        let mut previous = None;
        let mut expected_active = None;
        for (index, receipt) in self.audit.iter().enumerate() {
            let expected_sequence = u64::try_from(index).map_err(|_| {
                PdkTechnologyError::LimitExceeded("audit index overflow".to_owned())
            })? + 1;
            if receipt.sequence != expected_sequence {
                return Err(PdkTechnologyError::AuditCorrupted(format!(
                    "receipt[{index}] sequence is {}, expected {expected_sequence}",
                    receipt.sequence
                )));
            }
            if receipt.previous_receipt_digest != previous {
                return Err(PdkTechnologyError::AuditCorrupted(format!(
                    "receipt[{index}] does not bind the exact previous receipt"
                )));
            }
            validate_identifier(&format!("audit[{index}].actor_id"), &receipt.actor_id)?;
            validate_identifier(
                &format!("audit[{index}].authority_id"),
                &receipt.authority_id,
            )?;
            validate_text(&format!("audit[{index}].reason"), &receipt.reason, 1_024)?;
            validate_identifier(
                &format!("audit[{index}].target.package_id"),
                &receipt.target.package_id,
            )?;
            validate_version(
                &format!("audit[{index}].target.revision"),
                &receipt.target.revision,
            )?;
            if receipt.calculate_digest()? != receipt.receipt_digest {
                return Err(PdkTechnologyError::AuditCorrupted(format!(
                    "receipt[{index}] content digest does not match its payload"
                )));
            }
            if receipt.before_active != expected_active {
                return Err(PdkTechnologyError::AuditCorrupted(format!(
                    "receipt[{index}] before_active does not match the preceding transaction"
                )));
            }
            match receipt.action {
                PdkTechnologyAuditAction::Install => {
                    if receipt.after_active != receipt.before_active {
                        return Err(PdkTechnologyError::AuditCorrupted(format!(
                            "receipt[{index}] install transaction changes the active binding"
                        )));
                    }
                }
                PdkTechnologyAuditAction::Activate | PdkTechnologyAuditAction::Rollback => {
                    if receipt.after_active.as_ref() != Some(&receipt.target) {
                        return Err(PdkTechnologyError::AuditCorrupted(format!(
                            "receipt[{index}] activation target and resulting binding differ"
                        )));
                    }
                    if receipt.after_active == receipt.before_active {
                        return Err(PdkTechnologyError::AuditCorrupted(format!(
                            "receipt[{index}] records a no-op activation transition"
                        )));
                    }
                }
            }
            expected_active = receipt.after_active.clone();
            previous = Some(receipt.receipt_digest);
        }
        if self.active != expected_active {
            return Err(PdkTechnologyError::AuditCorrupted(
                "registry active binding does not match the final audit transaction".to_owned(),
            ));
        }
        Ok(())
    }

    fn activate_as(
        &mut self,
        action: PdkTechnologyAuditAction,
        package_id: &str,
        revision: &str,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkTechnologyAuditReceipt, PdkTechnologyError> {
        authority.validate()?;
        validate_text("reason", reason, 1_024)?;
        self.validate_audit_chain()?;
        let package = self
            .validated_packages
            .iter()
            .find(|candidate| {
                candidate
                    .manifest()
                    .package_id
                    .eq_ignore_ascii_case(package_id)
                    && candidate.manifest().revision == revision
            })
            .cloned()
            .ok_or_else(|| {
                PdkTechnologyError::NotRuntimeValidated(format!(
                    "{package_id} {revision} is not present in the current trusted runtime catalog"
                ))
            })?;
        let after = package.binding();
        validate_runtime_compatibility(package.manifest())?;
        if self.active.as_ref() == Some(&after) {
            return Err(PdkTechnologyError::InvalidTransition(format!(
                "{} {} is already active",
                after.package_id, after.revision
            )));
        }
        let before = self.active.clone();
        let receipt = self.next_receipt(
            action,
            authority,
            reason,
            after.clone(),
            package.archive_digest(),
            before,
            Some(after.clone()),
        )?;
        self.active = Some(after);
        self.audit.push(receipt.clone());
        Ok(receipt)
    }

    fn next_receipt(
        &self,
        action: PdkTechnologyAuditAction,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
        target: PdkTechnologyBinding,
        archive_digest: ContentDigest,
        before_active: Option<PdkTechnologyBinding>,
        after_active: Option<PdkTechnologyBinding>,
    ) -> Result<PdkTechnologyAuditReceipt, PdkTechnologyError> {
        if self.audit.len() >= MAX_PDK_AUDIT_RECEIPTS {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "technology audit is limited to {MAX_PDK_AUDIT_RECEIPTS} receipts"
            )));
        }
        let sequence = u64::try_from(self.audit.len())
            .map_err(|_| PdkTechnologyError::LimitExceeded("audit index overflow".to_owned()))?
            + 1;
        let mut receipt = PdkTechnologyAuditReceipt {
            sequence,
            action,
            actor_id: authority.actor_id.clone(),
            authority_id: authority.authority_id.clone(),
            reason: reason.to_owned(),
            target,
            archive_digest,
            before_active,
            after_active,
            previous_receipt_digest: self.audit.last().map(|receipt| receipt.receipt_digest),
            receipt_digest: content_digest(&[]),
        };
        receipt.receipt_digest = receipt.calculate_digest()?;
        Ok(receipt)
    }

    fn sort_packages(&mut self) {
        self.archives.sort_by(|left, right| {
            let left = archive_identity(left).ok();
            let right = archive_identity(right).ok();
            left.as_ref()
                .map(|binding| {
                    (
                        binding.package_id.to_ascii_lowercase(),
                        binding.revision.clone(),
                    )
                })
                .cmp(&right.as_ref().map(|binding| {
                    (
                        binding.package_id.to_ascii_lowercase(),
                        binding.revision.clone(),
                    )
                }))
        });
        self.validated_packages.sort_by(package_order);
    }
}

fn archive_identity(
    archive: &SignedPdkTechnologyArchive,
) -> Result<PdkTechnologyBinding, PdkTechnologyError> {
    let bytes = decode_bounded(
        "manifest_base64",
        &archive.manifest_base64,
        MAX_PDK_MANIFEST_BYTES,
    )?;
    let manifest: PdkTechnologyManifest = serde_json::from_slice(&bytes)
        .map_err(|error| PdkTechnologyError::ManifestParse(error.to_string()))?;
    Ok(PdkTechnologyBinding {
        package_id: manifest.package_id,
        revision: manifest.revision,
        manifest_digest: content_digest(&bytes),
    })
}

fn package_order(
    left: &ValidatedPdkTechnologyPackage,
    right: &ValidatedPdkTechnologyPackage,
) -> std::cmp::Ordering {
    (
        left.manifest().package_id.to_ascii_lowercase(),
        left.manifest().revision.as_str(),
    )
        .cmp(&(
            right.manifest().package_id.to_ascii_lowercase(),
            right.manifest().revision.as_str(),
        ))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use ed25519_dalek::{Signer as _, SigningKey};
    use rspice_model_library::pdk::manifest::package_path_to_host_path;
    use std::path::PathBuf;

    pub(crate) fn fixture_archive() -> (Vec<u8>, PdkPublisherTrustStore, PdkAdministrativeAuthority)
    {
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let model_bytes = br#".lib TT
.model nmos_demo nmos level=1 vto=0.55
.endl TT
.lib SS
.model nmos_demo nmos level=1 vto=0.60
.endl SS
.lib FF
.model nmos_demo nmos level=1 vto=0.50
.endl FF
.lib SF
.model nmos_demo nmos level=1 vto=0.58
.endl SF
.lib FS
.model nmos_demo nmos level=1 vto=0.52
.endl FS
"#
        .to_vec();
        let callback_bytes = wat::parse_str(
            r#"(module
                (memory (export "memory") 1 2)
                (func (export "derive") (result i32)
                    i32.const 0))"#,
        )
        .unwrap();
        let manifest = PdkTechnologyManifest {
            schema_version: PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION,
            package_id: "demo180".to_owned(),
            technology_name: "Demo 180 nm".to_owned(),
            revision: "2.3.1".to_owned(),
            publisher_id: "rspice-foundry-demo".to_owned(),
            signing_key_id: "ceremony-01".to_owned(),
            license_spdx: "LicenseRef-RSpice-Demo-PDK".to_owned(),
            process_node_nm: 180,
            database_unit_meters: 1.0e-9,
            stack_name: "1P2M".to_owned(),
            compatibility: PdkTechnologyCompatibility {
                minimum_engine_version: "0.1.0".to_owned(),
                minimum_viewer_version: "0.1.0".to_owned(),
                targets: vec![
                    PdkExecutionTarget::Desktop,
                    PdkExecutionTarget::WebAssembly,
                    PdkExecutionTarget::Mobile,
                ],
            },
            model_sources: PdkModelProcess::ALL
                .into_iter()
                .map(|process| PdkModelProcessContract {
                    process,
                    sources: vec![PdkModelSectionSource {
                        source_id: format!(
                            "demo-models-{}",
                            process.keyword().to_ascii_lowercase()
                        ),
                        domain: PdkModelDomain::Composite,
                        artifact_path: "models/demo.lib".to_owned(),
                        section: Some(process.keyword().to_owned()),
                    }],
                    required_domains: vec![PdkModelDomain::Composite],
                })
                .collect(),
            veriloga_sources: Vec::new(),
            symbol_definitions: Vec::new(),
            layers: vec![
                PdkTechnologyLayer {
                    name: "active".to_owned(),
                    order: 0,
                    kind: PdkLayerKind::Active,
                    purposes: vec!["drawing".to_owned()],
                    role: "diffusion".to_owned(),
                    display_rgba: [64, 160, 96, 255],
                },
                PdkTechnologyLayer {
                    name: "cont".to_owned(),
                    order: 1,
                    kind: PdkLayerKind::Cut,
                    purposes: vec!["drawing".to_owned()],
                    role: "active to metal1".to_owned(),
                    display_rgba: [192, 192, 192, 255],
                },
                PdkTechnologyLayer {
                    name: "metal1".to_owned(),
                    order: 2,
                    kind: PdkLayerKind::Metal,
                    purposes: vec!["drawing".to_owned(), "pin".to_owned()],
                    role: "routing".to_owned(),
                    display_rgba: [64, 144, 208, 255],
                },
            ],
            layer_aliases: Vec::new(),
            stream_map: vec![
                PdkStreamMapEntry {
                    layer: "active".to_owned(),
                    purpose: "drawing".to_owned(),
                    stream_layer: 1,
                    stream_datatype: 0,
                },
                PdkStreamMapEntry {
                    layer: "cont".to_owned(),
                    purpose: "drawing".to_owned(),
                    stream_layer: 2,
                    stream_datatype: 0,
                },
                PdkStreamMapEntry {
                    layer: "metal1".to_owned(),
                    purpose: "drawing".to_owned(),
                    stream_layer: 3,
                    stream_datatype: 0,
                },
                PdkStreamMapEntry {
                    layer: "metal1".to_owned(),
                    purpose: "pin".to_owned(),
                    stream_layer: 3,
                    stream_datatype: 1,
                },
            ],
            connectivity: vec![PdkConnectivityEdge {
                from_layer: "active".to_owned(),
                through_layer: "cont".to_owned(),
                to_layer: "metal1".to_owned(),
            }],
            vias: Vec::new(),
            recognition: Vec::new(),
            extraction: Vec::new(),
            callbacks: vec![PdkCallbackContract {
                callback_id: "derive-device".to_owned(),
                artifact_path: "callbacks/derive.wasm".to_owned(),
                abi_version: PDK_CALLBACK_ABI_VERSION,
                entrypoint: "derive".to_owned(),
                capabilities: vec![
                    PdkCallbackCapability::ReadPackage,
                    PdkCallbackCapability::WriteDerivedMetadata,
                ],
            }],
            artifacts: vec![
                PdkTechnologyArtifact {
                    path: "models/demo.lib".to_owned(),
                    kind: PdkTechnologyArtifactKind::Model,
                    size_bytes: u64::try_from(model_bytes.len()).unwrap(),
                    sha256: content_digest(&model_bytes),
                },
                PdkTechnologyArtifact {
                    path: "callbacks/derive.wasm".to_owned(),
                    kind: PdkTechnologyArtifactKind::Callback,
                    size_bytes: u64::try_from(callback_bytes.len()).unwrap(),
                    sha256: content_digest(&callback_bytes),
                },
            ],
        };
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        let archive = SignedPdkTechnologyArchive {
            schema_version: PDK_TECHNOLOGY_ARCHIVE_SCHEMA_VERSION,
            manifest_base64: STANDARD.encode(&manifest_bytes),
            signature_base64: STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes()),
            files: vec![
                PdkTechnologyArchiveFile {
                    path: "models/demo.lib".to_owned(),
                    content_base64: STANDARD.encode(model_bytes),
                },
                PdkTechnologyArchiveFile {
                    path: "callbacks/derive.wasm".to_owned(),
                    content_base64: STANDARD.encode(callback_bytes),
                },
            ],
        };
        (
            serde_json::to_vec(&archive).unwrap(),
            {
                let mut trust = PdkPublisherTrustStore::default();
                trust.keys = vec![TrustedPdkPublisherKey {
                    publisher_id: "rspice-foundry-demo".to_owned(),
                    key_id: "ceremony-01".to_owned(),
                    verifying_key: signing_key.verifying_key().to_bytes(),
                    revoked: false,
                }];
                trust
            },
            PdkAdministrativeAuthority {
                actor_id: "cad-admin@example.com".to_owned(),
                authority_id: "role:pdk-administrator".to_owned(),
            },
        )
    }

    pub(crate) fn fixture_archive_with_veriloga()
    -> (Vec<u8>, PdkPublisherTrustStore, PdkAdministrativeAuthority) {
        fixture_archive_with_veriloga_source(
            br#"`include "parts/resistance.vams"
module pdk_resistor(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = `PDK_RESISTANCE;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
        )
    }

    pub(crate) fn fixture_archive_with_veriloga_source(
        root: &[u8],
    ) -> (Vec<u8>, PdkPublisherTrustStore, PdkAdministrativeAuthority) {
        let (bytes, trust, authority) = fixture_archive();
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        let dependency = b"`define PDK_RESISTANCE 250.0\n".to_vec();
        for (path, content) in [
            ("veriloga/pdk_resistor.va", root),
            ("veriloga/parts/resistance.vams", dependency.as_slice()),
        ] {
            manifest.artifacts.push(PdkTechnologyArtifact {
                path: path.to_owned(),
                kind: PdkTechnologyArtifactKind::VerilogASource,
                size_bytes: u64::try_from(content.len()).unwrap(),
                sha256: content_digest(content),
            });
            archive.files.push(PdkTechnologyArchiveFile {
                path: path.to_owned(),
                content_base64: STANDARD.encode(content),
            });
        }
        manifest.veriloga_sources = vec![PdkVerilogASourceContract {
            source_id: "pdk-resistor-runtime".to_owned(),
            root_artifact_path: "veriloga/pdk_resistor.va".to_owned(),
            module_name: "pdk_resistor".to_owned(),
            netlist_alias: "pdk_resistor_model".to_owned(),
        }];
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        (serde_json::to_vec(&archive).unwrap(), trust, authority)
    }

    #[test]
    fn schema_five_layer_aliases_and_via_definitions_are_typed_and_cross_validated() {
        let (bytes, _, _) = fixture_archive();
        let archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        manifest.layer_aliases.push(PdkLayerAlias {
            alias: "m1_drawing".to_owned(),
            layer: "metal1".to_owned(),
            purpose: "drawing".to_owned(),
        });
        manifest.vias.push(PdkViaDefinition {
            via_id: "cont_active_m1".to_owned(),
            lower_layer: "active".to_owned(),
            cut_layer: "cont".to_owned(),
            upper_layer: "metal1".to_owned(),
            cut_width_meters: 1.6e-7,
            cut_height_meters: 1.6e-7,
            lower_enclosure_meters: 5.0e-8,
            upper_enclosure_meters: 5.0e-8,
            maximum_rows: 8,
            maximum_columns: 8,
            maximum_rms_current_per_cut_amperes: Some(8.0e-3),
        });
        validate_manifest(&manifest).expect("schema-five physical contracts validate");

        let mut invalid_alias = manifest.clone();
        invalid_alias.layer_aliases[0].alias = "metal1".to_owned();
        assert!(matches!(
            validate_manifest(&invalid_alias),
            Err(PdkTechnologyError::Duplicate(_))
        ));

        let mut invalid_via = manifest;
        invalid_via.vias[0].cut_layer = "active".to_owned();
        assert!(matches!(
            validate_manifest(&invalid_via),
            Err(PdkTechnologyError::InvalidField(_)) | Err(PdkTechnologyError::InvalidReference(_))
        ));
    }

    #[test]
    fn via_connectivity_rejects_cut_or_marker_endpoints() {
        let (bytes, _, _) = fixture_archive();
        let archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        let metal = manifest
            .layers
            .iter_mut()
            .find(|layer| layer.name == "metal1")
            .expect("fixture metal layer");
        metal.kind = PdkLayerKind::Marker;
        let error = validate_manifest(&manifest).expect_err("marker is not a via endpoint");
        assert!(
            error.to_string().contains("not a conductor layer"),
            "{error}"
        );
    }

    pub(crate) fn fixture_signed_symbol(
        manifest: &PdkTechnologyManifest,
    ) -> rspice_model_library::symbol::ModelBoundSymbolDefinition {
        let mut model = rspice_model_library::symbol::SymbolModelReference::new(
            "signed-pdk:demo-models-tt",
            "nmos_demo",
        )
        .with_source_path("models/demo.lib");
        model.section = Some("TT".to_owned());
        model.revision = Some(manifest.revision.clone());
        let pins = [
            ("D", rspice_model_library::symbol::SymbolPinSide::Right),
            ("G", rspice_model_library::symbol::SymbolPinSide::Left),
            ("S", rspice_model_library::symbol::SymbolPinSide::Right),
            ("B", rspice_model_library::symbol::SymbolPinSide::Bottom),
        ]
        .into_iter()
        .enumerate()
        .map(|(index, (name, side))| {
            rspice_model_library::symbol::SymbolPinDefinition::new(
                name,
                rspice_model_library::symbol::SymbolElectricalType::Analog,
                crate::state::PortDirection::InOut,
                side,
                index + 1,
            )
        })
        .collect::<Vec<_>>();
        let ports = pins
            .iter()
            .map(rspice_model_library::symbol::SymbolPinDefinition::port_spec)
            .collect();
        rspice_model_library::symbol::ModelBoundSymbolDefinition::new(
            rspice_model_library::symbol::SymbolIdentity::new(
                &manifest.package_id,
                "nmos_demo",
                1,
                "signed-pdk:demo180/nmos_demo",
            ),
            rspice_model_library::symbol::SymbolSourceContract::model(model.clone(), ports),
            pins,
            rspice_model_library::symbol::SymbolGraphicTemplate::RectangularIc,
            rspice_model_library::symbol::SymbolParameterForm {
                revision: 1,
                sections: Vec::new(),
            },
            rspice_model_library::symbol::SymbolNetlistBinding {
                device_prefix: "M".to_owned(),
                model: Some(model),
                template: "M{name} {nodes} {model} {params}".to_owned(),
                parameter_order: Vec::new(),
            },
            rspice_model_library::symbol::GeneratedSymbolViews::default(),
        )
    }

    pub(crate) fn fixture_archive_with_symbols()
    -> (Vec<u8>, PdkPublisherTrustStore, PdkAdministrativeAuthority) {
        let (bytes, trust, authority) = fixture_archive();
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        manifest.symbol_definitions = vec![fixture_signed_symbol(&manifest)];
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        (serde_json::to_vec(&archive).unwrap(), trust, authority)
    }

    #[test]
    fn signed_symbols_materialize_exact_archive_bound_sources() {
        let (bytes, trust, _) = fixture_archive_with_symbols();
        let (_, package) = validate_archive_bytes(&bytes, &trust).expect("signed symbol validates");
        assert_eq!(package.manifest().symbol_definitions.len(), 1);
        assert_eq!(
            package.manifest().symbol_definitions[0]
                .netlist
                .model
                .as_ref()
                .and_then(|model| model.source_path.as_deref()),
            Some("models/demo.lib")
        );

        let definition = package
            .symbol_definitions()
            .first()
            .expect("runtime symbol");
        definition
            .validate()
            .expect("materialized symbol validates");
        let source = definition
            .netlist
            .model
            .as_ref()
            .and_then(|model| model.source_path.as_deref())
            .expect("materialized source");
        assert_eq!(
            PathBuf::from(source),
            signed_model_virtual_root(&package.archive_digest().to_string())
                .join(package_path_to_host_path("models/demo.lib"))
        );
    }

    #[test]
    fn signed_symbols_reject_provider_or_artifact_authority_mismatch() {
        let (bytes, trust, _) = fixture_archive_with_symbols();
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        let definition = manifest.symbol_definitions.first_mut().unwrap();
        let rspice_model_library::symbol::SymbolSourceContract::Model { model, .. } =
            &mut definition.source
        else {
            unreachable!()
        };
        model.library = "signed-pdk:unrelated-provider".to_owned();
        definition.netlist.model = Some(model.clone());
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());

        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&archive).unwrap(), &trust),
            Err(PdkTechnologyError::InvalidReference(_))
        ));
    }

    #[test]
    fn signed_model_sources_materialize_exact_process_sections_and_archive_identity() {
        let (bytes, trust, authority) = fixture_archive();
        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(
                &bytes,
                &trust,
                &authority,
                "Install executable model-source fixture",
            )
            .expect("signed package installs");
        let package = registry.validated_packages()[0].clone();
        let sealed = registry
            .seal_model_sources_for_binding(&package.binding(), package.archive_digest())
            .expect("exact project-bound model sources seal");
        assert_eq!(sealed.as_parts().sources.len(), 1);
        assert_eq!(sealed.as_parts().process_bindings.len(), 5);
        assert_eq!(sealed.as_parts().binding, package.binding());
        assert_eq!(sealed.as_parts().archive_digest, package.archive_digest());

        let combined = crate::state::model_library::ModelLibraryManager::new()
            .seal_execution_sources()
            .expect("empty ordinary model catalog seals")
            .with_pdk_model_sources(sealed)
            .expect("signed PDK closure merges");
        let tt = combined
            .reference_process_model_cards(crate::product::ProcessCorner::TT)
            .expect("TT materializes");
        assert_eq!(tt.len(), 1);
        assert!(tt[0].contains("vto=0.55"));
        assert!(!tt[0].contains("vto=0.60"));
        assert!(!tt[0].to_ascii_lowercase().contains(".lib "));

        let corner_bindings = combined
            .corner_model_bindings(&[
                rspice_app_types::product::ProcessCorner::SS,
                rspice_app_types::product::ProcessCorner::FF,
            ])
            .expect("explicit signed corner sections materialize");
        assert_eq!(corner_bindings.len(), 2);
        assert!(
            corner_bindings[0]
                .materialized_model_cards
                .contains("vto=0.60")
        );
        assert!(
            corner_bindings[1]
                .materialized_model_cards
                .contains("vto=0.50")
        );
        let (identity, digest) = combined
            .pdk_model_identity()
            .expect("prepared model snapshot binds signed package");
        assert!(identity.contains("demo180@2.3.1"));
        assert_eq!(digest, package.archive_digest());
    }

    #[test]
    fn signed_veriloga_closure_compiles_and_retains_exact_archive_authority() {
        let (bytes, trust, authority) = fixture_archive_with_veriloga();
        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(
                &bytes,
                &trust,
                &authority,
                "Install signed Verilog-A fixture",
            )
            .expect("signed Verilog-A package installs");
        let package = registry.validated_packages()[0].clone();
        let sealed = registry
            .seal_model_sources_for_binding(&package.binding(), package.archive_digest())
            .expect("exact signed Verilog-A closure seals");
        assert_eq!(sealed.as_parts().veriloga_artifacts.len(), 2);
        assert_eq!(sealed.as_parts().veriloga_bindings.len(), 1);
        let combined = crate::state::model_library::ModelLibraryManager::new()
            .seal_execution_sources()
            .unwrap()
            .with_pdk_model_sources(sealed)
            .unwrap();
        let (binding, archive_digest, artifacts, bindings) = combined
            .pdk_veriloga_authority()
            .expect("signed runtime authority retained");
        assert_eq!(binding, &package.binding());
        assert_eq!(archive_digest, package.archive_digest());
        let runtime = crate::simulation::veriloga::compile_signed_pdk_source_runtime(
            binding,
            archive_digest,
            artifacts,
            &bindings[0],
        )
        .expect("retained signed source recompiles");
        assert!(runtime.source_key().starts_with("__rspice_pdk__/"));
        assert_eq!(runtime.source_digest(), package.archive_digest());
        assert_eq!(runtime.module_name(), "pdk_resistor");
        assert_eq!(runtime.netlist_alias(), "pdk_resistor_model");
        assert_eq!(runtime.terminal_names().unwrap(), ["p", "n"]);
        let encoded = serde_json::to_vec(&runtime).unwrap();
        let restored: crate::simulation::veriloga::PreparedVerilogARuntime =
            serde_json::from_slice(&encoded).unwrap();
        restored.validate().expect("worker payload revalidates");
        assert_eq!(restored, runtime);
    }

    #[test]
    fn signed_veriloga_rejects_tampered_or_untyped_dependency_bytes() {
        let (bytes, trust, _) = fixture_archive_with_veriloga();
        let mut tampered: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        tampered
            .files
            .iter_mut()
            .find(|file| file.path.ends_with("resistance.vams"))
            .unwrap()
            .content_base64 = STANDARD.encode(b"`define PDK_RESISTANCE 251.0\n");
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&tampered).unwrap(), &trust),
            Err(PdkTechnologyError::ArtifactDigestMismatch { .. })
        ));

        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let mut untyped: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&untyped.manifest_base64).unwrap()).unwrap();
        manifest
            .artifacts
            .iter_mut()
            .find(|artifact| artifact.path.ends_with("resistance.vams"))
            .unwrap()
            .kind = PdkTechnologyArtifactKind::Documentation;
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        untyped.manifest_base64 = STANDARD.encode(&manifest_bytes);
        untyped.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&untyped).unwrap(), &trust),
            Err(PdkTechnologyError::ModelMaterialization(_))
        ));
    }

    #[test]
    fn signed_veriloga_manifest_rejects_alias_collisions_unreachable_sources_and_schema_downgrade()
    {
        let (bytes, trust, _) = fixture_archive_with_veriloga();
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);

        let mut collision: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&collision.manifest_base64).unwrap()).unwrap();
        let mut duplicate = manifest.veriloga_sources[0].clone();
        duplicate.source_id = "second-runtime".to_owned();
        duplicate.module_name = "second_module".to_owned();
        duplicate.netlist_alias = "PDK_RESISTOR_MODEL".to_owned();
        manifest.veriloga_sources.push(duplicate);
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        collision.manifest_base64 = STANDARD.encode(&manifest_bytes);
        collision.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&collision).unwrap(), &trust),
            Err(PdkTechnologyError::Duplicate(_))
        ));

        let mut unreachable: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&unreachable.manifest_base64).unwrap())
                .unwrap();
        let orphan = b"module orphan(p); inout p; electrical p; analog I(p) <+ 0.0; endmodule\n";
        manifest.artifacts.push(PdkTechnologyArtifact {
            path: "veriloga/orphan.va".to_owned(),
            kind: PdkTechnologyArtifactKind::VerilogASource,
            size_bytes: u64::try_from(orphan.len()).unwrap(),
            sha256: content_digest(orphan),
        });
        unreachable.files.push(PdkTechnologyArchiveFile {
            path: "veriloga/orphan.va".to_owned(),
            content_base64: STANDARD.encode(orphan),
        });
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        unreachable.manifest_base64 = STANDARD.encode(&manifest_bytes);
        unreachable.signature_base64 =
            STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        authenticate_archive(&unreachable, &trust).expect("archive bytes are authentic");
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&unreachable).unwrap(), &trust),
            Err(PdkTechnologyError::ModelMaterialization(_))
        ));

        let mut downgraded: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&downgraded.manifest_base64).unwrap()).unwrap();
        manifest.schema_version = 1;
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        downgraded.manifest_base64 = STANDARD.encode(&manifest_bytes);
        downgraded.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&downgraded).unwrap(), &trust),
            Err(PdkTechnologyError::InvalidField(_))
        ));
    }

    #[test]
    fn signed_model_sections_close_package_relative_dependencies_in_memory() {
        let (bytes, trust, authority) = fixture_archive();
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        let root = STANDARD.decode(&archive.files[0].content_base64).unwrap();
        let root = String::from_utf8(root)
            .unwrap()
            .replacen(".lib TT\n", ".lib TT\n.include \"parts/common.inc\"\n", 1)
            .into_bytes();
        archive.files[0].content_base64 = STANDARD.encode(&root);
        manifest.artifacts[0].size_bytes = u64::try_from(root.len()).unwrap();
        manifest.artifacts[0].sha256 = content_digest(&root);

        let dependency = b".model pdk_helper d is=1e-14\n".to_vec();
        manifest.artifacts.push(PdkTechnologyArtifact {
            path: "models/parts/common.inc".to_owned(),
            kind: PdkTechnologyArtifactKind::Model,
            size_bytes: u64::try_from(dependency.len()).unwrap(),
            sha256: content_digest(&dependency),
        });
        archive.files.push(PdkTechnologyArchiveFile {
            path: "models/parts/common.inc".to_owned(),
            content_base64: STANDARD.encode(&dependency),
        });
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        let signed = serde_json::to_vec(&archive).unwrap();

        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(
                &signed,
                &trust,
                &authority,
                "Install dependency-closed package",
            )
            .expect("package-relative dependency validates");
        let package = registry.validated_packages()[0].clone();
        let sealed = registry
            .seal_model_sources_for_binding(&package.binding(), package.archive_digest())
            .expect("dependency closure seals");
        assert_eq!(sealed.as_parts().sources.len(), 2);
        assert_eq!(sealed.as_parts().edges.len(), 1);
        let combined = crate::state::model_library::ModelLibraryManager::new()
            .seal_execution_sources()
            .unwrap()
            .with_pdk_model_sources(sealed)
            .unwrap();
        let cards = combined
            .reference_process_model_cards(crate::product::ProcessCorner::TT)
            .unwrap();
        assert!(cards[0].contains(".model pdk_helper d is=1e-14"));
        assert!(!cards[0].contains(".include"));
        assert!(!cards[0].contains("/rspice-pdk/"));
    }

    #[test]
    fn signed_model_contract_rejects_external_dependencies_and_missing_reference_process() {
        let (bytes, trust, _) = fixture_archive();
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();

        let external = b".include \"../../outside.lib\"\n.model nmos_demo nmos level=1".to_vec();
        archive.files[0].content_base64 = STANDARD.encode(&external);
        manifest.artifacts[0].size_bytes = u64::try_from(external.len()).unwrap();
        manifest.artifacts[0].sha256 = content_digest(&external);
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&archive).unwrap(), &trust),
            Err(PdkTechnologyError::ModelMaterialization(_))
        ));

        let (bytes, trust, _) = fixture_archive();
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        manifest
            .model_sources
            .retain(|contract| contract.process != PdkModelProcess::Tt);
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&archive).unwrap(), &trust),
            Err(PdkTechnologyError::InvalidReference(_))
        ));
    }

    #[test]
    fn project_model_sealing_rejects_post_validation_archive_mutation() {
        let (bytes, trust, authority) = fixture_archive();
        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(&bytes, &trust, &authority, "Install exact package")
            .expect("install");
        let package = registry.validated_packages()[0].clone();
        registry.archives[0].files[0].content_base64 =
            STANDARD.encode(b".model attacker nmos level=1");
        assert!(matches!(
            registry.seal_model_sources_for_binding(&package.binding(), package.archive_digest()),
            Err(PdkTechnologyError::NotRuntimeValidated(_))
        ));
    }

    #[test]
    fn signed_archive_verifies_every_exact_artifact_and_contract() {
        let (bytes, trust, _) = fixture_archive();
        let (_, package) = validate_archive_bytes(&bytes, &trust).expect("archive validates");

        assert_eq!(package.manifest().package_id, "demo180");
        assert_eq!(package.manifest().layers.len(), 3);
        assert_eq!(package.artifact_digests().len(), 2);
        let archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(
            package.archive_digest(),
            content_digest(&serde_json::to_vec(&archive).unwrap())
        );
    }

    #[test]
    fn recognition_and_extraction_contracts_are_typed_complete_and_source_bound() {
        let (bytes, trust, _) = fixture_archive();
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let manifest_bytes = STANDARD.decode(&archive.manifest_base64).unwrap();
        let mut manifest: PdkTechnologyManifest = serde_json::from_slice(&manifest_bytes).unwrap();

        let specialized = [
            (
                "recognition/nmos.json",
                PdkTechnologyArtifactKind::RecognitionMap,
                br#"{"device":"nmos"}"#.as_slice(),
            ),
            (
                "extraction/rc.json",
                PdkTechnologyArtifactKind::ExtractionRule,
                br#"{"quantities":["r","c"]}"#.as_slice(),
            ),
            (
                "qualification/nmos-layout.json",
                PdkTechnologyArtifactKind::QualificationVector,
                br#"{"shapes":[]}"#.as_slice(),
            ),
            (
                "qualification/rc-layout.json",
                PdkTechnologyArtifactKind::QualificationVector,
                br#"{"wires":[]}"#.as_slice(),
            ),
            (
                "qualification/rc-reference.json",
                PdkTechnologyArtifactKind::QualificationReference,
                br#"{"r":10.0,"c":1e-15}"#.as_slice(),
            ),
        ];
        for (path, kind, content) in specialized {
            manifest.artifacts.push(PdkTechnologyArtifact {
                path: path.to_owned(),
                kind,
                size_bytes: u64::try_from(content.len()).unwrap(),
                sha256: content_digest(content),
            });
            archive.files.push(PdkTechnologyArchiveFile {
                path: path.to_owned(),
                content_base64: STANDARD.encode(content),
            });
        }
        manifest.recognition = vec![PdkRecognitionContract {
            contract_id: "recognize-nmos".to_owned(),
            device_class: "nmos".to_owned(),
            rule_artifact_path: "recognition/nmos.json".to_owned(),
            terminals: vec![
                PdkRecognitionTerminal {
                    terminal_name: "source".to_owned(),
                    layer: "active".to_owned(),
                    purpose: "drawing".to_owned(),
                },
                PdkRecognitionTerminal {
                    terminal_name: "drain".to_owned(),
                    layer: "active".to_owned(),
                    purpose: "drawing".to_owned(),
                },
            ],
            qualification_vectors: vec![PdkRecognitionQualificationVector {
                vector_id: "recognize-nmos-positive".to_owned(),
                layout_artifact_path: "qualification/nmos-layout.json".to_owned(),
                expected_instance_count: 1,
            }],
        }];
        manifest.extraction = vec![PdkExtractionContract {
            contract_id: "extract-metal-rc".to_owned(),
            rule_artifact_path: "extraction/rc.json".to_owned(),
            quantities: vec![
                PdkExtractionQuantity::Resistance,
                PdkExtractionQuantity::Capacitance,
            ],
            layer_purposes: vec![PdkLayerPurposeRef {
                layer: "metal1".to_owned(),
                purpose: "drawing".to_owned(),
            }],
            qualification_vectors: vec![PdkExtractionQualificationVector {
                vector_id: "extract-metal-rc-reference".to_owned(),
                layout_artifact_path: "qualification/rc-layout.json".to_owned(),
                reference_artifact_path: "qualification/rc-reference.json".to_owned(),
            }],
        }];

        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        let signed = serde_json::to_vec(&archive).unwrap();
        let (_, package) =
            validate_archive_bytes(&signed, &trust).expect("typed contracts validate");
        assert_eq!(package.manifest().recognition.len(), 1);
        assert_eq!(package.manifest().extraction.len(), 1);

        let mut invalid = manifest.clone();
        invalid.recognition[0].terminals[0].purpose = "undeclared".to_owned();
        let invalid_bytes = serde_json::to_vec(&invalid).unwrap();
        archive.manifest_base64 = STANDARD.encode(&invalid_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&invalid_bytes).to_bytes());
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&archive).unwrap(), &trust),
            Err(PdkTechnologyError::InvalidReference(_))
        ));

        let mut duplicate = manifest;
        duplicate.extraction[0].qualification_vectors[0].layout_artifact_path =
            "qualification/nmos-layout.json".to_owned();
        let duplicate_bytes = serde_json::to_vec(&duplicate).unwrap();
        archive.manifest_base64 = STANDARD.encode(&duplicate_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&duplicate_bytes).to_bytes());
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&archive).unwrap(), &trust),
            Err(PdkTechnologyError::Duplicate(_))
        ));
    }

    #[test]
    fn project_pin_resolves_only_the_exact_currently_trusted_archive() {
        let (bytes, trust, authority) = fixture_archive();
        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(&bytes, &trust, &authority, "Install for project pin")
            .expect("install");
        let package = registry
            .validated_packages()
            .first()
            .expect("installed package validates");
        let pin = crate::state::workspace::ProjectSignedTechnologyPin::from_package_metadata(
            package.metadata(),
        )
        .expect("project pin");
        registry
            .validate_project_pin(&pin)
            .expect("exact trusted archive resolves");

        let json = serde_json::to_string(&pin).expect("pin serializes");
        let restored: crate::state::workspace::ProjectSignedTechnologyPin =
            serde_json::from_str(&json).expect("pin deserializes");
        assert_eq!(restored, pin);

        let mut revoked = trust;
        revoked.keys[0].revoked = true;
        registry
            .revalidate_installed(&revoked)
            .expect_err("revocation invalidates runtime packages");
        assert!(matches!(
            registry.validate_project_pin(&pin),
            Err(crate::state::workspace::TechnologyBindingError::SignedPackageUnavailable { .. })
        ));
    }

    #[test]
    fn publisher_key_provision_and_revocation_are_immutable_and_hash_chained() {
        let (bytes, fixture_trust, authority) = fixture_archive();
        let key = fixture_trust.keys[0].clone();
        let mut trust = PdkPublisherTrustStore::default();
        let provision = trust
            .provision_key(key.clone(), &authority, "Approve foundry ceremony key")
            .expect("provision");
        assert_eq!(provision.action, PdkTrustAuditAction::Provision);
        assert!(validate_archive_bytes(&bytes, &trust).is_ok());

        let before_duplicate = trust.clone();
        assert!(matches!(
            trust.provision_key(key.clone(), &authority, "Duplicate"),
            Err(PdkTechnologyError::ImmutableTrustKey(_))
        ));
        assert_eq!(trust, before_duplicate);

        let revoke = trust
            .revoke_key(
                &key.publisher_id,
                &key.key_id,
                &authority,
                "Publisher key retired",
            )
            .expect("revoke");
        assert_eq!(revoke.action, PdkTrustAuditAction::Revoke);
        assert_eq!(
            revoke.previous_receipt_digest,
            Some(provision.receipt_digest)
        );
        assert!(matches!(
            validate_archive_bytes(&bytes, &trust),
            Err(PdkTechnologyError::RevokedPublisherKey { .. })
        ));
        let before_second_revoke = trust.clone();
        assert!(matches!(
            trust.revoke_key(&key.publisher_id, &key.key_id, &authority, "Revoke again"),
            Err(PdkTechnologyError::ImmutableTrustKey(_))
        ));
        assert_eq!(trust, before_second_revoke);

        let json = serde_json::to_string(&trust).expect("serialize governed trust");
        let restored: PdkPublisherTrustStore =
            serde_json::from_str(&json).expect("deserialize governed trust");
        restored.validate().expect("restored audit validates");

        let mut tampered = serde_json::to_value(&restored).unwrap();
        tampered["audit"][0]["reason"] = serde_json::json!("altered");
        let tampered: PdkPublisherTrustStore = serde_json::from_value(tampered).unwrap();
        assert!(matches!(
            tampered.validate(),
            Err(PdkTechnologyError::TrustAuditCorrupted(_))
        ));
    }

    #[test]
    fn tampered_artifact_signature_and_unknown_key_fail_closed() {
        let (bytes, trust, _) = fixture_archive();
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        archive.files[0].content_base64 = STANDARD.encode(b"tampered");
        let tampered = serde_json::to_vec(&archive).unwrap();
        assert!(matches!(
            validate_archive_bytes(&tampered, &trust),
            Err(PdkTechnologyError::ArtifactSizeMismatch { .. })
                | Err(PdkTechnologyError::ArtifactDigestMismatch { .. })
        ));

        let mut signature_tampered: SignedPdkTechnologyArchive =
            serde_json::from_slice(&bytes).unwrap();
        signature_tampered.signature_base64 = STANDARD.encode([0_u8; 64]);
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&signature_tampered).unwrap(), &trust),
            Err(PdkTechnologyError::InvalidSignature { .. })
        ));

        assert!(matches!(
            validate_archive_bytes(&bytes, &PdkPublisherTrustStore::default()),
            Err(PdkTechnologyError::UntrustedPublisher { .. })
        ));
    }

    #[test]
    fn network_callbacks_and_incomplete_stream_maps_are_rejected() {
        let (bytes, trust, _) = fixture_archive();
        let archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let manifest_bytes = STANDARD.decode(&archive.manifest_base64).unwrap();
        let mut manifest: PdkTechnologyManifest = serde_json::from_slice(&manifest_bytes).unwrap();
        manifest.callbacks[0]
            .capabilities
            .push(PdkCallbackCapability::Network);
        assert!(matches!(
            validate_manifest(&manifest),
            Err(PdkTechnologyError::ForbiddenCapability(_))
        ));

        manifest.callbacks[0].capabilities.pop();
        manifest.stream_map.pop();
        assert!(matches!(
            validate_manifest(&manifest),
            Err(PdkTechnologyError::MissingMapping(_))
        ));

        let mut revoked = trust;
        revoked.keys[0].revoked = true;
        assert!(matches!(
            validate_archive_bytes(&bytes, &revoked),
            Err(PdkTechnologyError::RevokedPublisherKey { .. })
        ));
    }

    #[test]
    fn target_declarations_are_nonempty_and_activation_is_platform_scoped() {
        let (bytes, trust, authority) = fixture_archive();
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let manifest_bytes = STANDARD.decode(&archive.manifest_base64).unwrap();
        let mut manifest: PdkTechnologyManifest = serde_json::from_slice(&manifest_bytes).unwrap();
        manifest.compatibility.targets.clear();
        assert!(matches!(
            validate_manifest(&manifest),
            Err(PdkTechnologyError::InvalidField(_))
        ));

        manifest.compatibility.targets = vec![match current_execution_target() {
            PdkExecutionTarget::Desktop => PdkExecutionTarget::WebAssembly,
            PdkExecutionTarget::WebAssembly | PdkExecutionTarget::Mobile => {
                PdkExecutionTarget::Desktop
            }
        }];
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        let restricted_bytes = serde_json::to_vec(&archive).unwrap();

        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(
                &restricted_bytes,
                &trust,
                &authority,
                "Install platform-restricted package",
            )
            .expect("a platform-restricted package can be inspected");
        assert!(matches!(
            registry.activate(
                "demo180",
                "2.3.1",
                &authority,
                "Attempt incompatible activation"
            ),
            Err(PdkTechnologyError::IncompatibleRuntime(_))
        ));
        assert!(registry.active_binding().is_none());
    }

    #[test]
    fn install_activation_and_rollback_are_hash_chained_and_revalidated() {
        let (bytes, trust, authority) = fixture_archive();
        let mut registry = PdkTechnologyRegistry::default();
        let install = registry
            .install_archive_bytes(&bytes, &trust, &authority, "Install reviewed package")
            .expect("install");
        let activate = registry
            .activate(
                "demo180",
                "2.3.1",
                &authority,
                "Activate for new project bindings",
            )
            .expect("activate");
        assert_eq!(
            activate.previous_receipt_digest,
            Some(install.receipt_digest)
        );
        registry.validate_audit_chain().expect("audit chain");
        assert!(registry.active_package().is_some());

        let json = serde_json::to_string(&registry).unwrap();
        let mut restored: PdkTechnologyRegistry = serde_json::from_str(&json).unwrap();
        assert!(restored.active_package().is_none());
        assert!(!restored.runtime_ready());
        restored
            .revalidate_installed(&trust)
            .expect("revalidate persisted archive");
        assert!(restored.active_package().is_some());

        // The same active target is not a rollback. A future revision must be
        // activated before returning to this retained prior binding.
        assert!(matches!(
            restored.rollback_to("demo180", "2.3.1", &authority, "No intervening revision"),
            Err(PdkTechnologyError::InvalidTransition(_))
        ));
    }

    #[test]
    fn immutable_revision_and_tampered_audit_fail_before_mutation() {
        let (bytes, trust, authority) = fixture_archive();
        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(&bytes, &trust, &authority, "Install")
            .unwrap();
        let before = registry.clone();
        assert!(matches!(
            registry.install_archive_bytes(&bytes, &trust, &authority, "Install again"),
            Err(PdkTechnologyError::ImmutableRevision(_))
        ));
        assert_eq!(registry, before);

        registry.audit[0].reason = "altered".to_owned();
        let tampered = registry.clone();
        assert!(matches!(
            registry.activate("demo180", "2.3.1", &authority, "Activate"),
            Err(PdkTechnologyError::AuditCorrupted(_))
        ));
        assert_eq!(registry, tampered);
    }

    #[test]
    fn recomputed_hashes_cannot_disguise_impossible_transitions_or_wrong_archives() {
        let (bytes, trust, authority) = fixture_archive();
        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(&bytes, &trust, &authority, "Install")
            .unwrap();

        let target = registry.audit[0].target.clone();
        registry.audit[0].after_active = Some(target.clone());
        registry.audit[0].receipt_digest = registry.audit[0].calculate_digest().unwrap();
        registry.active = Some(target);
        assert!(matches!(
            registry.validate_audit_chain(),
            Err(PdkTechnologyError::AuditCorrupted(_))
        ));

        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(&bytes, &trust, &authority, "Install")
            .unwrap();
        registry.audit[0].archive_digest = content_digest(b"wrong archive");
        registry.audit[0].receipt_digest = registry.audit[0].calculate_digest().unwrap();
        let errors = registry
            .revalidate_installed(&trust)
            .expect_err("receipt must bind the installed archive");
        assert!(
            errors
                .iter()
                .any(|error| error.contains("archive digest does not match"))
        );
    }

    #[test]
    fn unknown_manifest_fields_and_unsafe_paths_are_rejected() {
        let (bytes, trust, _) = fixture_archive();
        let archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let manifest_bytes = STANDARD.decode(&archive.manifest_base64).unwrap();
        let mut value: serde_json::Value = serde_json::from_slice(&manifest_bytes).unwrap();
        value["unknown"] = serde_json::json!(true);
        assert!(
            serde_json::from_value::<PdkTechnologyManifest>(value)
                .unwrap_err()
                .to_string()
                .contains("unknown field")
        );
        assert!(matches!(
            validate_package_path("artifact", "../secret"),
            Err(PdkTechnologyError::InvalidField(_))
        ));
        assert!(trust.validate().is_ok());
    }
}
