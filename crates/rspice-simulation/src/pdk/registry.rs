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

use super::{
    SealedPdkModelSources, ValidatedPdkTechnologyPackage, validate_archive, validate_archive_bytes,
    validate_runtime_compatibility,
};
use rspice_app_types::product::ContentDigest;
use rspice_model_library::pdk::contracts::*;
use rspice_model_library::pdk::manifest::{PdkTechnologyManifest, validate_version};
use rspice_model_library::pdk::package::decode_bounded;
use rspice_model_library::pdk::{
    PdkAdministrativeAuthority, PdkPublisherTrustStore, PdkTechnologyError, content_digest,
    validate_identifier, validate_text,
};
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use std::collections::BTreeSet;

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
    pub(super) fn take_worker_validated_packages(&mut self) -> Vec<ValidatedPdkTechnologyPackage> {
        std::mem::take(&mut self.validated_packages)
    }

    #[cfg(target_arch = "wasm32")]
    pub(super) fn restore_worker_validated_packages(
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
    pub fn take_archives_for_browser_persistence(&mut self) -> Vec<SignedPdkTechnologyArchive> {
        self.validated_packages.clear();
        self.validation_errors.clear();
        std::mem::take(&mut self.archives)
    }

    /// Reattach exact signed archive payloads loaded from the browser object
    /// store. This deliberately restores no runtime trust; the caller must
    /// run `revalidate_installed` against the current publisher trust store.
    pub fn restore_archives_from_browser_persistence(
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
    pub fn seal_model_sources_for_binding(
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
        input: &rspice_model_library::pdk::callback::PdkCallbackExecutionInput,
    ) -> Result<
        rspice_model_library::pdk::callback::PdkCallbackExecutionReceipt,
        rspice_model_library::pdk::callback::PdkCallbackError,
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

    #[allow(
        clippy::too_many_arguments,
        reason = "Keep the before and after bindings explicit at each audited operation"
    )]
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
mod tests;
