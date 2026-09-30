//! Persisted PDK technology authoring drafts.
//!
//! Installed packages remain immutable and executable only after signature
//! validation. A draft is an unsigned candidate derived from one exact source
//! package. It may be temporarily invalid while edited, but it cannot be
//! exported for signing until the complete candidate manifest passes the same
//! validation used for installed packages.

use serde::{Deserialize, Serialize};

use super::{
    PdkTechnologyError,
    contracts::{
        PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION, PdkTechnologyArchiveFile, PdkTechnologyBinding,
        SignedPdkTechnologyArchive,
    },
    manifest::{PdkTechnologyManifest, validate_manifest},
    package::PdkTechnologyPackageMetadata,
};

pub const PDK_TECHNOLOGY_DRAFT_SCHEMA_VERSION: u32 = 1;
pub const PDK_TECHNOLOGY_AUTHORING_BUNDLE_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyDraftBaseline {
    pub binding: PdkTechnologyBinding,
    pub archive_digest: rspice_app_types::product::ContentDigest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyDraft {
    pub schema_version: u32,
    pub draft_id: String,
    pub baseline: PdkTechnologyDraftBaseline,
    pub manifest: PdkTechnologyManifest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnsignedPdkTechnologyAuthoringBundle {
    pub schema_version: u32,
    pub draft_id: String,
    pub baseline: PdkTechnologyDraftBaseline,
    pub candidate_manifest: PdkTechnologyManifest,
    /// Exact source-package files retained for an external publisher signing
    /// step. RSpice never accepts or persists a publisher private key.
    pub source_files: Vec<PdkTechnologyArchiveFile>,
}

impl PdkTechnologyDraft {
    #[must_use]
    pub fn from_package(package: &PdkTechnologyPackageMetadata) -> Self {
        let mut manifest = package.manifest().clone();
        manifest.schema_version = PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION;
        Self {
            schema_version: PDK_TECHNOLOGY_DRAFT_SCHEMA_VERSION,
            draft_id: format!("{}-authoring", manifest.package_id),
            baseline: PdkTechnologyDraftBaseline {
                binding: package.binding(),
                archive_digest: package.archive_digest(),
            },
            manifest,
        }
    }

    /// Change the candidate revision and keep revision-bound signed symbol
    /// model references internally consistent.
    pub fn set_revision(&mut self, revision: String) {
        self.manifest.revision.clone_from(&revision);
        for definition in &mut self.manifest.symbol_definitions {
            let crate::symbol::SymbolSourceContract::Model { model, .. } = &mut definition.source
            else {
                continue;
            };
            model.revision = Some(revision.clone());
            if let Some(netlist_model) = definition.netlist.model.as_mut() {
                netlist_model.revision = Some(revision.clone());
            }
        }
    }

    pub fn validate_candidate(
        &self,
        baseline: &PdkTechnologyPackageMetadata,
    ) -> Result<(), PdkTechnologyError> {
        if self.schema_version != PDK_TECHNOLOGY_DRAFT_SCHEMA_VERSION {
            return Err(PdkTechnologyError::UnsupportedSchema {
                object: "technology draft",
                actual: self.schema_version,
                supported: PDK_TECHNOLOGY_DRAFT_SCHEMA_VERSION,
            });
        }
        if self.draft_id.trim().is_empty() || self.draft_id.len() > 128 {
            return Err(PdkTechnologyError::InvalidField(
                "technology draft ID must contain 1..=128 bytes".to_owned(),
            ));
        }
        if self.baseline.binding != baseline.binding()
            || self.baseline.archive_digest != baseline.archive_digest()
        {
            return Err(PdkTechnologyError::InvalidReference(
                "technology draft baseline no longer resolves to the exact trusted source package"
                    .to_owned(),
            ));
        }
        if !self
            .manifest
            .package_id
            .eq_ignore_ascii_case(&baseline.manifest().package_id)
            || self.manifest.publisher_id != baseline.manifest().publisher_id
            || self.manifest.signing_key_id != baseline.manifest().signing_key_id
        {
            return Err(PdkTechnologyError::InvalidReference(
                "technology draft cannot change package lineage or publisher signing identity"
                    .to_owned(),
            ));
        }
        if self.manifest.revision == self.baseline.binding.revision {
            return Err(PdkTechnologyError::ImmutableRevision(format!(
                "candidate revision must differ from immutable baseline {}",
                self.baseline.binding.revision
            )));
        }
        if self.manifest.schema_version != PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION {
            return Err(PdkTechnologyError::UnsupportedSchema {
                object: "candidate manifest",
                actual: self.manifest.schema_version,
                supported: PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION,
            });
        }
        if self.manifest.artifacts != baseline.manifest().artifacts {
            return Err(PdkTechnologyError::InvalidReference(
                "this authoring workflow cannot change source-package artifact identity; import a separately built signed package when rule or model bytes change"
                    .to_owned(),
            ));
        }
        validate_manifest(&self.manifest)
    }

    pub fn authoring_bundle(
        &self,
        baseline: &PdkTechnologyPackageMetadata,
        source_archive: &SignedPdkTechnologyArchive,
    ) -> Result<UnsignedPdkTechnologyAuthoringBundle, PdkTechnologyError> {
        self.validate_candidate(baseline)?;
        Ok(UnsignedPdkTechnologyAuthoringBundle {
            schema_version: PDK_TECHNOLOGY_AUTHORING_BUNDLE_SCHEMA_VERSION,
            draft_id: self.draft_id.clone(),
            baseline: self.baseline.clone(),
            candidate_manifest: self.manifest.clone(),
            source_files: source_archive.files.clone(),
        })
    }
}
