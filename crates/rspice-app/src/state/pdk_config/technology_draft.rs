//! Persisted PDK technology authoring drafts.
//!
//! Installed packages remain immutable and executable only after signature
//! validation. A draft is an unsigned candidate derived from one exact source
//! package. It may be temporarily invalid while edited, but it cannot be
//! exported for signing until the complete candidate manifest passes the same
//! validation used for installed packages.

use serde::{Deserialize, Serialize};

use super::technology_package::{
    PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION, PdkTechnologyArchiveFile, PdkTechnologyBinding,
    PdkTechnologyError, PdkTechnologyManifest, SignedPdkTechnologyArchive,
    ValidatedPdkTechnologyPackage, validate_manifest,
};

pub const PDK_TECHNOLOGY_DRAFT_SCHEMA_VERSION: u32 = 1;
pub const PDK_TECHNOLOGY_AUTHORING_BUNDLE_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyDraftBaseline {
    pub binding: PdkTechnologyBinding,
    pub archive_digest: crate::product::ContentDigest,
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
    pub fn from_package(package: &ValidatedPdkTechnologyPackage) -> Self {
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
            let crate::state::SymbolSourceContract::Model { model, .. } = &mut definition.source
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
        baseline: &ValidatedPdkTechnologyPackage,
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
        baseline: &ValidatedPdkTechnologyPackage,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draft_requires_a_new_revision_and_preserves_exact_artifact_authority() {
        let (bytes, trust, _) = super::super::technology_package::tests::fixture_archive();
        let (_, package) = super::super::technology_package::validate_archive_bytes(&bytes, &trust)
            .expect("fixture validates");
        let mut draft = PdkTechnologyDraft::from_package(&package);
        assert!(matches!(
            draft.validate_candidate(&package),
            Err(PdkTechnologyError::ImmutableRevision(_))
        ));

        draft.set_revision("2.4.0".to_owned());
        draft
            .validate_candidate(&package)
            .expect("candidate validates");
        draft.manifest.artifacts.clear();
        assert!(matches!(
            draft.validate_candidate(&package),
            Err(PdkTechnologyError::InvalidReference(_))
        ));
    }

    #[test]
    fn authoring_bundle_carries_source_files_without_private_signing_material() {
        let (bytes, trust, _) = super::super::technology_package::tests::fixture_archive();
        let (archive, package) =
            super::super::technology_package::validate_archive_bytes(&bytes, &trust)
                .expect("fixture validates");
        let mut draft = PdkTechnologyDraft::from_package(&package);
        draft.set_revision("2.4.0".to_owned());
        let bundle = draft
            .authoring_bundle(&package, &archive)
            .expect("bundle builds");
        assert_eq!(bundle.source_files, archive.files);
        let json = serde_json::to_value(bundle).expect("bundle serializes");
        assert!(json.get("signature_base64").is_none());
    }

    #[test]
    fn persisted_pdk_config_round_trips_an_invalid_in_progress_draft_without_authority() {
        let (bytes, trust, _) = super::super::technology_package::tests::fixture_archive();
        let (_, package) = super::super::technology_package::validate_archive_bytes(&bytes, &trust)
            .expect("fixture validates");
        let draft = PdkTechnologyDraft::from_package(&package);
        assert!(draft.validate_candidate(&package).is_err());

        let mut config = super::super::PdkConfig::default();
        config.technology_draft = Some(draft.clone());
        let json = serde_json::to_string(&config).expect("PDK config serializes");
        let restored: super::super::PdkConfig =
            serde_json::from_str(&json).expect("PDK config restores");
        assert_eq!(restored.technology_draft, Some(draft));
        assert!(restored.technology_registry.validated_packages().is_empty());
    }
}
