//! Content-addressed PDK technology-package administration.
//!
//! A configured model search path is not a technology package.  This module
//! keeps those concepts separate and gives the PDK administration surface a
//! real authority boundary:
//!
//! - the manifest and every packaged artifact are size- and digest-verified;
//! - layer, purpose, stream-map, connectivity, callback, and platform
//!   contracts are validated before installation;
//! - installed revisions are immutable;
//! - installs are append-only, hash-chained transactions; and
//! - deserialized packages are never executable until they are revalidated.

use super::{
    SealedPdkModelSources, ValidatedPdkTechnologyPackage, validate_archive, validate_archive_bytes,
};
use rspice_model_library::pdk::contracts::*;
use rspice_model_library::pdk::manifest::{PdkTechnologyManifest, validate_version};
use rspice_model_library::pdk::package::decode_bounded;
use rspice_model_library::pdk::{
    PdkAdministrativeAuthority, PdkTechnologyError, content_digest, validate_identifier,
    validate_text,
};
use rspice_vocabulary::product::ContentDigest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyAuditReceipt {
    pub sequence: u64,
    pub actor_id: String,
    pub authority_id: String,
    pub reason: String,
    pub target: PdkTechnologyBinding,
    pub archive_digest: ContentDigest,
    pub previous_receipt_digest: Option<ContentDigest>,
    pub receipt_digest: ContentDigest,
}

#[derive(Serialize)]
struct PdkTechnologyAuditPayload<'a> {
    sequence: u64,
    actor_id: &'a str,
    authority_id: &'a str,
    reason: &'a str,
    target: &'a PdkTechnologyBinding,
    archive_digest: ContentDigest,
    previous_receipt_digest: Option<ContentDigest>,
}

impl PdkTechnologyAuditReceipt {
    fn calculate_digest(&self) -> Result<ContentDigest, PdkTechnologyError> {
        let payload = PdkTechnologyAuditPayload {
            sequence: self.sequence,
            actor_id: &self.actor_id,
            authority_id: &self.authority_id,
            reason: &self.reason,
            target: &self.target,
            archive_digest: self.archive_digest,
            previous_receipt_digest: self.previous_receipt_digest,
        };
        let bytes = serde_json::to_vec(&payload)
            .map_err(|error| PdkTechnologyError::Serialization(error.to_string()))?;
        Ok(content_digest(&bytes))
    }
}

/// Persisted PDK archives and administrative history.  `validated_packages`
/// is intentionally runtime-only: loading serialized state cannot restore
/// validation.  Call `revalidate_installed` before any package can be
/// consumed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyRegistry {
    #[serde(default)]
    archives: Vec<PdkTechnologyArchive>,
    #[serde(default)]
    audit: Vec<PdkTechnologyAuditReceipt>,
    #[serde(skip)]
    validated_packages: Vec<ValidatedPdkTechnologyPackage>,
    #[serde(skip)]
    validation_errors: Vec<String>,
}

impl PdkTechnologyRegistry {
    #[must_use]
    pub fn archives(&self) -> &[PdkTechnologyArchive] {
        &self.archives
    }

    /// Remove the PDK archive payloads from a persistence clone while
    /// retaining its bindings and audit history. Browser persistence stores
    /// these potentially large immutable payloads in a content-addressed
    /// object store instead of embedding them in one configuration record.
    pub fn take_archives_for_browser_persistence(&mut self) -> Vec<PdkTechnologyArchive> {
        self.validated_packages.clear();
        self.validation_errors.clear();
        std::mem::take(&mut self.archives)
    }

    /// Reattach exact PDK archive payloads loaded from the browser object
    /// store. This deliberately restores no runtime validation; the caller
    /// must run `revalidate_installed`.
    pub fn restore_archives_from_browser_persistence(
        &mut self,
        archives: Vec<PdkTechnologyArchive>,
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

    /// Seal the exact PDK model-source closure named by a project pin.
    ///
    /// The project binding, manifest digest, archive digest, decoded artifact
    /// bytes, package-relative dependency graph, and process-section contract
    /// are all checked again before runtime source authority is returned.
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

    pub fn install_archive_bytes(
        &mut self,
        bytes: &[u8],
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkTechnologyAuditReceipt, PdkTechnologyError> {
        authority.validate()?;
        validate_text("reason", reason, 1_024)?;
        self.validate_audit_chain()?;
        let (archive, package) = validate_archive_bytes(bytes)?;
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
                "{} {} is already installed; PDK revisions are immutable",
                binding.package_id, binding.revision
            )));
        }
        if self.archives.len() >= MAX_PDK_ARTIFACTS {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "technology package registry is limited to {MAX_PDK_ARTIFACTS} installed revisions"
            )));
        }

        let receipt = self.next_receipt(authority, reason, binding, package.archive_digest())?;
        self.archives.push(archive);
        self.validated_packages.push(package);
        self.sort_packages();
        self.audit.push(receipt.clone());
        Ok(receipt)
    }

    /// Rebuild all runtime validation from the exact persisted archives.
    /// Failure publishes neither a partial validated catalog nor executable
    /// authority.
    pub fn revalidate_installed(&mut self) -> Result<usize, Vec<String>> {
        if let Err(error) = self.validate_audit_chain() {
            self.validated_packages.clear();
            self.validation_errors = vec![error.to_string()];
            return Err(self.validation_errors.clone());
        }
        let mut packages = Vec::with_capacity(self.archives.len());
        let mut errors = Vec::new();
        for (index, archive) in self.archives.iter().enumerate() {
            match validate_archive(archive) {
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
                        "receipt[{index}] references package {} {} that is not installed with its exact PDK manifest",
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
            self.validated_packages = packages;
            self.validation_errors.clear();
            Ok(self.validated_packages.len())
        } else {
            self.validated_packages.clear();
            self.validation_errors = errors.clone();
            Err(errors)
        }
    }

    pub fn validate_audit_chain(&self) -> Result<(), PdkTechnologyError> {
        if self.audit.len() > MAX_PDK_AUDIT_RECEIPTS {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "technology audit exceeds {MAX_PDK_AUDIT_RECEIPTS} receipts"
            )));
        }
        let mut previous = None;
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
            previous = Some(receipt.receipt_digest);
        }
        Ok(())
    }

    fn next_receipt(
        &self,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
        target: PdkTechnologyBinding,
        archive_digest: ContentDigest,
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
            actor_id: authority.actor_id.clone(),
            authority_id: authority.authority_id.clone(),
            reason: reason.to_owned(),
            target,
            archive_digest,
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
    archive: &PdkTechnologyArchive,
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
