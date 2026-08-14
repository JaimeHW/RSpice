//! Deterministic comparison of validated PDK technology revisions.
//!
//! A revision label is not a migration assessment. This module compares the
//! exact validated manifests and archive identities and classifies every
//! changed contract before a consuming project considers migration.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use super::technology_package::{
    PdkTechnologyArtifact, PdkTechnologyArtifactKind, PdkTechnologyBinding, PdkTechnologyLayer,
    ValidatedPdkTechnologyPackage,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdkTechnologyDiffArea {
    Identity,
    Compatibility,
    ModelSource,
    Symbol,
    Layer,
    StreamMap,
    Connectivity,
    Recognition,
    Extraction,
    Callback,
    Artifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdkTechnologyDiffKind {
    Added,
    Removed,
    Changed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdkTechnologyDiffImpact {
    Informational,
    ReviewRequired,
    Breaking,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyDiffEntry {
    pub area: PdkTechnologyDiffArea,
    pub identity: String,
    pub kind: PdkTechnologyDiffKind,
    pub impact: PdkTechnologyDiffImpact,
    pub before: Option<String>,
    pub after: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyRevisionDiff {
    pub baseline: PdkTechnologyBinding,
    pub baseline_archive_digest: crate::product::ContentDigest,
    pub candidate: PdkTechnologyBinding,
    pub candidate_archive_digest: crate::product::ContentDigest,
    pub same_package_lineage: bool,
    pub entries: Vec<PdkTechnologyDiffEntry>,
}

pub const PDK_TECHNOLOGY_MIGRATION_EVIDENCE_SCHEMA_VERSION: u16 = 1;

/// Compact, immutable proof of the exact signed-revision comparison reviewed
/// before a project technology replacement. The full diff is reproducible
/// from the retained content-addressed packages; the project receipt retains
/// its digest and impact cardinalities without copying megabytes of manifest
/// data into every project revision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyMigrationEvidence {
    schema_version: u16,
    baseline: PdkTechnologyBinding,
    baseline_archive_digest: crate::product::ContentDigest,
    candidate: PdkTechnologyBinding,
    candidate_archive_digest: crate::product::ContentDigest,
    diff_digest: crate::product::ContentDigest,
    entry_count: u32,
    breaking_count: u32,
    review_required_count: u32,
    informational_count: u32,
}

impl PdkTechnologyMigrationEvidence {
    pub fn from_diff(diff: &PdkTechnologyRevisionDiff) -> Result<Self, PdkTechnologyDiffError> {
        let encoded = serde_json::to_vec(diff)
            .map_err(|error| PdkTechnologyDiffError::Serialization(error.to_string()))?;
        let count = |impact| {
            u32::try_from(diff.count(impact)).map_err(|_| {
                PdkTechnologyDiffError::InvalidEvidence(
                    "revision diff contains more than u32::MAX entries".to_owned(),
                )
            })
        };
        let evidence = Self {
            schema_version: PDK_TECHNOLOGY_MIGRATION_EVIDENCE_SCHEMA_VERSION,
            baseline: diff.baseline.clone(),
            baseline_archive_digest: diff.baseline_archive_digest,
            candidate: diff.candidate.clone(),
            candidate_archive_digest: diff.candidate_archive_digest,
            diff_digest: crate::product::ContentDigest::from_bytes(Sha256::digest(encoded).into()),
            entry_count: u32::try_from(diff.entries.len()).map_err(|_| {
                PdkTechnologyDiffError::InvalidEvidence(
                    "revision diff contains more than u32::MAX entries".to_owned(),
                )
            })?,
            breaking_count: count(PdkTechnologyDiffImpact::Breaking)?,
            review_required_count: count(PdkTechnologyDiffImpact::ReviewRequired)?,
            informational_count: count(PdkTechnologyDiffImpact::Informational)?,
        };
        evidence.validate()?;
        Ok(evidence)
    }

    pub fn validate(&self) -> Result<(), PdkTechnologyDiffError> {
        if self.schema_version != PDK_TECHNOLOGY_MIGRATION_EVIDENCE_SCHEMA_VERSION {
            return Err(PdkTechnologyDiffError::InvalidEvidence(format!(
                "unsupported migration-evidence schema {}",
                self.schema_version
            )));
        }
        let classified = self
            .breaking_count
            .checked_add(self.review_required_count)
            .and_then(|count| count.checked_add(self.informational_count))
            .ok_or_else(|| {
                PdkTechnologyDiffError::InvalidEvidence(
                    "migration-evidence impact counts overflow".to_owned(),
                )
            })?;
        if classified != self.entry_count {
            return Err(PdkTechnologyDiffError::InvalidEvidence(format!(
                "migration evidence classifies {classified} entries but declares {}",
                self.entry_count
            )));
        }
        if self.diff_digest == crate::product::ContentDigest::from_bytes([0; 32]) {
            return Err(PdkTechnologyDiffError::InvalidEvidence(
                "migration-evidence diff digest is the zero sentinel".to_owned(),
            ));
        }
        Ok(())
    }

    pub fn matches_diff(&self, diff: &PdkTechnologyRevisionDiff) -> bool {
        Self::from_diff(diff).as_ref() == Ok(self)
    }

    pub const fn baseline(&self) -> &PdkTechnologyBinding {
        &self.baseline
    }

    pub const fn baseline_archive_digest(&self) -> crate::product::ContentDigest {
        self.baseline_archive_digest
    }

    pub const fn candidate(&self) -> &PdkTechnologyBinding {
        &self.candidate
    }

    pub const fn candidate_archive_digest(&self) -> crate::product::ContentDigest {
        self.candidate_archive_digest
    }

    pub const fn diff_digest(&self) -> crate::product::ContentDigest {
        self.diff_digest
    }

    pub const fn entry_count(&self) -> u32 {
        self.entry_count
    }

    pub const fn breaking_count(&self) -> u32 {
        self.breaking_count
    }

    pub const fn review_required_count(&self) -> u32 {
        self.review_required_count
    }

    pub const fn informational_count(&self) -> u32 {
        self.informational_count
    }
}

impl PdkTechnologyRevisionDiff {
    pub fn between(
        baseline: &ValidatedPdkTechnologyPackage,
        candidate: &ValidatedPdkTechnologyPackage,
    ) -> Result<Self, PdkTechnologyDiffError> {
        let before = baseline.manifest();
        let after = candidate.manifest();
        let mut entries = Vec::new();

        scalar_change(
            &mut entries,
            "manifest schema",
            &before.schema_version,
            &after.schema_version,
            PdkTechnologyDiffImpact::Breaking,
        )?;
        scalar_change(
            &mut entries,
            "package ID",
            &before.package_id,
            &after.package_id,
            PdkTechnologyDiffImpact::Breaking,
        )?;
        scalar_change(
            &mut entries,
            "technology name",
            &before.technology_name,
            &after.technology_name,
            PdkTechnologyDiffImpact::ReviewRequired,
        )?;
        scalar_change(
            &mut entries,
            "revision",
            &before.revision,
            &after.revision,
            PdkTechnologyDiffImpact::Informational,
        )?;
        scalar_change(
            &mut entries,
            "publisher ID",
            &before.publisher_id,
            &after.publisher_id,
            PdkTechnologyDiffImpact::ReviewRequired,
        )?;
        scalar_change(
            &mut entries,
            "signing key ID",
            &before.signing_key_id,
            &after.signing_key_id,
            PdkTechnologyDiffImpact::ReviewRequired,
        )?;
        scalar_change(
            &mut entries,
            "license SPDX",
            &before.license_spdx,
            &after.license_spdx,
            PdkTechnologyDiffImpact::ReviewRequired,
        )?;
        scalar_change(
            &mut entries,
            "process node (nm)",
            &before.process_node_nm,
            &after.process_node_nm,
            PdkTechnologyDiffImpact::Breaking,
        )?;
        if before.database_unit_meters.to_bits() != after.database_unit_meters.to_bits() {
            push_change(
                &mut entries,
                PdkTechnologyDiffArea::Identity,
                "database unit (m)",
                PdkTechnologyDiffKind::Changed,
                PdkTechnologyDiffImpact::Breaking,
                Some(canonical_json(&before.database_unit_meters)?),
                Some(canonical_json(&after.database_unit_meters)?),
            );
        }
        scalar_change(
            &mut entries,
            "stack name",
            &before.stack_name,
            &after.stack_name,
            PdkTechnologyDiffImpact::Breaking,
        )?;
        if before.compatibility != after.compatibility {
            push_change(
                &mut entries,
                PdkTechnologyDiffArea::Compatibility,
                "runtime compatibility",
                PdkTechnologyDiffKind::Changed,
                PdkTechnologyDiffImpact::Breaking,
                Some(canonical_json(&before.compatibility)?),
                Some(canonical_json(&after.compatibility)?),
            );
        }
        diff_keyed(
            &mut entries,
            PdkTechnologyDiffArea::ModelSource,
            &before.model_sources,
            &after.model_sources,
            |contract| contract.process.keyword().to_ascii_lowercase(),
            |_, _, _| PdkTechnologyDiffImpact::Breaking,
        )?;
        diff_keyed(
            &mut entries,
            PdkTechnologyDiffArea::Symbol,
            &before.symbol_definitions,
            &after.symbol_definitions,
            |definition| {
                format!(
                    "{}/{}",
                    definition.identity.library.to_ascii_lowercase(),
                    definition.identity.cell.to_ascii_lowercase()
                )
            },
            |_, _, _| PdkTechnologyDiffImpact::Breaking,
        )?;

        diff_keyed(
            &mut entries,
            PdkTechnologyDiffArea::Layer,
            &before.layers,
            &after.layers,
            |layer| layer.name.to_ascii_lowercase(),
            layer_impact,
        )?;
        diff_keyed(
            &mut entries,
            PdkTechnologyDiffArea::Layer,
            &before.layer_aliases,
            &after.layer_aliases,
            |alias| alias.alias.to_ascii_lowercase(),
            |_, _, _| PdkTechnologyDiffImpact::Breaking,
        )?;
        diff_keyed(
            &mut entries,
            PdkTechnologyDiffArea::StreamMap,
            &before.stream_map,
            &after.stream_map,
            |entry| {
                format!(
                    "{}/{}",
                    entry.layer.to_ascii_lowercase(),
                    entry.purpose.to_ascii_lowercase()
                )
            },
            |_, _, _| PdkTechnologyDiffImpact::Breaking,
        )?;
        diff_keyed(
            &mut entries,
            PdkTechnologyDiffArea::Connectivity,
            &before.connectivity,
            &after.connectivity,
            |edge| {
                format!(
                    "{} -> {} -> {}",
                    edge.from_layer.to_ascii_lowercase(),
                    edge.through_layer.to_ascii_lowercase(),
                    edge.to_layer.to_ascii_lowercase()
                )
            },
            |_, _, _| PdkTechnologyDiffImpact::Breaking,
        )?;
        diff_keyed(
            &mut entries,
            PdkTechnologyDiffArea::Connectivity,
            &before.vias,
            &after.vias,
            |via| via.via_id.to_ascii_lowercase(),
            |_, _, _| PdkTechnologyDiffImpact::Breaking,
        )?;
        diff_keyed(
            &mut entries,
            PdkTechnologyDiffArea::Recognition,
            &before.recognition,
            &after.recognition,
            |contract| contract.contract_id.to_ascii_lowercase(),
            |_, _, _| PdkTechnologyDiffImpact::Breaking,
        )?;
        diff_keyed(
            &mut entries,
            PdkTechnologyDiffArea::Extraction,
            &before.extraction,
            &after.extraction,
            |contract| contract.contract_id.to_ascii_lowercase(),
            |_, _, _| PdkTechnologyDiffImpact::Breaking,
        )?;
        diff_keyed(
            &mut entries,
            PdkTechnologyDiffArea::Callback,
            &before.callbacks,
            &after.callbacks,
            |contract| contract.callback_id.to_ascii_lowercase(),
            |_, _, _| PdkTechnologyDiffImpact::Breaking,
        )?;
        diff_keyed(
            &mut entries,
            PdkTechnologyDiffArea::Artifact,
            &before.artifacts,
            &after.artifacts,
            |artifact| artifact.path.to_ascii_lowercase(),
            artifact_impact,
        )?;

        if baseline.manifest_digest() != candidate.manifest_digest()
            && before.revision == after.revision
        {
            push_change(
                &mut entries,
                PdkTechnologyDiffArea::Identity,
                "manifest digest under unchanged revision",
                PdkTechnologyDiffKind::Changed,
                PdkTechnologyDiffImpact::Breaking,
                Some(canonical_json(&baseline.manifest_digest())?),
                Some(canonical_json(&candidate.manifest_digest())?),
            );
        }
        if baseline.archive_digest() != candidate.archive_digest() {
            push_change(
                &mut entries,
                PdkTechnologyDiffArea::Artifact,
                "signed archive digest",
                PdkTechnologyDiffKind::Changed,
                PdkTechnologyDiffImpact::ReviewRequired,
                Some(canonical_json(&baseline.archive_digest())?),
                Some(canonical_json(&candidate.archive_digest())?),
            );
        }

        Ok(Self {
            baseline: baseline.binding(),
            baseline_archive_digest: baseline.archive_digest(),
            candidate: candidate.binding(),
            candidate_archive_digest: candidate.archive_digest(),
            same_package_lineage: before.package_id.eq_ignore_ascii_case(&after.package_id),
            entries,
        })
    }

    #[must_use]
    pub fn count(&self, impact: PdkTechnologyDiffImpact) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.impact == impact)
            .count()
    }

    #[must_use]
    pub fn migration_requires_review(&self) -> bool {
        !self.entries.is_empty()
    }

    #[must_use]
    pub fn has_breaking_changes(&self) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.impact == PdkTechnologyDiffImpact::Breaking)
    }
}

fn scalar_change<T>(
    entries: &mut Vec<PdkTechnologyDiffEntry>,
    identity: &str,
    before: &T,
    after: &T,
    impact: PdkTechnologyDiffImpact,
) -> Result<(), PdkTechnologyDiffError>
where
    T: PartialEq + Serialize,
{
    if before != after {
        push_change(
            entries,
            PdkTechnologyDiffArea::Identity,
            identity,
            PdkTechnologyDiffKind::Changed,
            impact,
            Some(canonical_json(before)?),
            Some(canonical_json(after)?),
        );
    }
    Ok(())
}

fn diff_keyed<T, Key, Impact>(
    entries: &mut Vec<PdkTechnologyDiffEntry>,
    area: PdkTechnologyDiffArea,
    before: &[T],
    after: &[T],
    key: Key,
    impact: Impact,
) -> Result<(), PdkTechnologyDiffError>
where
    T: PartialEq + Serialize,
    Key: Fn(&T) -> String,
    Impact: Fn(PdkTechnologyDiffKind, Option<&T>, Option<&T>) -> PdkTechnologyDiffImpact,
{
    let before = keyed(area, before, &key)?;
    let after = keyed(area, after, &key)?;
    let identities = before
        .keys()
        .chain(after.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    for identity in identities {
        match (before.get(&identity), after.get(&identity)) {
            (None, Some(after)) => push_change(
                entries,
                area,
                identity,
                PdkTechnologyDiffKind::Added,
                impact(PdkTechnologyDiffKind::Added, None, Some(*after)),
                None,
                Some(canonical_json(*after)?),
            ),
            (Some(before), None) => push_change(
                entries,
                area,
                identity,
                PdkTechnologyDiffKind::Removed,
                impact(PdkTechnologyDiffKind::Removed, Some(*before), None),
                Some(canonical_json(*before)?),
                None,
            ),
            (Some(before), Some(after)) if *before != *after => push_change(
                entries,
                area,
                identity,
                PdkTechnologyDiffKind::Changed,
                impact(PdkTechnologyDiffKind::Changed, Some(*before), Some(*after)),
                Some(canonical_json(*before)?),
                Some(canonical_json(*after)?),
            ),
            _ => {}
        }
    }
    Ok(())
}

fn keyed<'a, T, Key>(
    area: PdkTechnologyDiffArea,
    values: &'a [T],
    key: &Key,
) -> Result<BTreeMap<String, &'a T>, PdkTechnologyDiffError>
where
    Key: Fn(&T) -> String,
{
    let mut keyed = BTreeMap::new();
    for value in values {
        let identity = key(value);
        if keyed.insert(identity.clone(), value).is_some() {
            return Err(PdkTechnologyDiffError::DuplicateIdentity { area, identity });
        }
    }
    Ok(keyed)
}

fn layer_impact(
    kind: PdkTechnologyDiffKind,
    before: Option<&PdkTechnologyLayer>,
    after: Option<&PdkTechnologyLayer>,
) -> PdkTechnologyDiffImpact {
    if kind == PdkTechnologyDiffKind::Changed
        && let (Some(before), Some(after)) = (before, after)
    {
        let presentation_only = before.name == after.name
            && before.order == after.order
            && before.kind == after.kind
            && before.purposes == after.purposes
            && before.role == after.role
            && before.display_rgba != after.display_rgba;
        if presentation_only {
            return PdkTechnologyDiffImpact::ReviewRequired;
        }
    }
    PdkTechnologyDiffImpact::Breaking
}

fn artifact_impact(
    _kind: PdkTechnologyDiffKind,
    before: Option<&PdkTechnologyArtifact>,
    after: Option<&PdkTechnologyArtifact>,
) -> PdkTechnologyDiffImpact {
    if before
        .into_iter()
        .chain(after)
        .all(|artifact| artifact_is_review_only(artifact.kind))
    {
        PdkTechnologyDiffImpact::ReviewRequired
    } else {
        PdkTechnologyDiffImpact::Breaking
    }
}

const fn artifact_is_review_only(kind: PdkTechnologyArtifactKind) -> bool {
    matches!(
        kind,
        PdkTechnologyArtifactKind::DisplayResource
            | PdkTechnologyArtifactKind::QualificationVector
            | PdkTechnologyArtifactKind::QualificationReference
            | PdkTechnologyArtifactKind::Documentation
    )
}

fn push_change(
    entries: &mut Vec<PdkTechnologyDiffEntry>,
    area: PdkTechnologyDiffArea,
    identity: impl Into<String>,
    kind: PdkTechnologyDiffKind,
    impact: PdkTechnologyDiffImpact,
    before: Option<String>,
    after: Option<String>,
) {
    entries.push(PdkTechnologyDiffEntry {
        area,
        identity: identity.into(),
        kind,
        impact,
        before,
        after,
    });
}

fn canonical_json(value: &impl Serialize) -> Result<String, PdkTechnologyDiffError> {
    serde_json::to_string(value)
        .map_err(|error| PdkTechnologyDiffError::Serialization(error.to_string()))
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PdkTechnologyDiffError {
    #[error("duplicate {area:?} identity '{identity}' in validated PDK package")]
    DuplicateIdentity {
        area: PdkTechnologyDiffArea,
        identity: String,
    },
    #[error("PDK revision diff serialization failed: {0}")]
    Serialization(String),
    #[error("invalid PDK migration evidence: {0}")]
    InvalidEvidence(String),
}

#[cfg(test)]
pub(crate) mod tests {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    use ed25519_dalek::{Signer as _, SigningKey};

    use super::*;
    use crate::state::pdk_config::technology_package::{
        PdkAdministrativeAuthority, PdkExecutionTarget, PdkPublisherTrustStore,
        PdkTechnologyManifest, SignedPdkTechnologyArchive,
        tests::{fixture_archive, fixture_signed_symbol},
        validate_archive_bytes,
    };

    pub(crate) fn fixture_revision_archives() -> (
        Vec<u8>,
        Vec<u8>,
        PdkPublisherTrustStore,
        PdkAdministrativeAuthority,
    ) {
        let (baseline, trust, authority) = fixture_archive();
        let candidate = resign_variant(&baseline, &trust, |manifest| {
            manifest.layers[0].display_rgba = [12, 34, 56, 255];
        });
        (baseline, candidate, trust, authority)
    }

    fn pair(
        mutate: impl FnOnce(&mut PdkTechnologyManifest),
    ) -> (ValidatedPdkTechnologyPackage, ValidatedPdkTechnologyPackage) {
        let (baseline_bytes, trust, _) = fixture_archive();
        let (_, baseline) =
            validate_archive_bytes(&baseline_bytes, &trust).expect("baseline package");
        let candidate_bytes = resign_variant(&baseline_bytes, &trust, mutate);
        let (_, candidate) =
            validate_archive_bytes(&candidate_bytes, &trust).expect("candidate package");
        (baseline, candidate)
    }

    fn resign_variant(
        archive_bytes: &[u8],
        _trust: &PdkPublisherTrustStore,
        mutate: impl FnOnce(&mut PdkTechnologyManifest),
    ) -> Vec<u8> {
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let mut archive: SignedPdkTechnologyArchive =
            serde_json::from_slice(archive_bytes).expect("archive fixture");
        let manifest_bytes = STANDARD
            .decode(&archive.manifest_base64)
            .expect("manifest base64");
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&manifest_bytes).expect("manifest fixture");
        manifest.revision = "2.4.0".to_owned();
        mutate(&mut manifest);
        let manifest_bytes = serde_json::to_vec(&manifest).expect("candidate manifest");
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        serde_json::to_vec(&archive).expect("candidate archive")
    }

    #[test]
    fn identical_validated_package_has_no_changes() {
        let (bytes, trust, _) = fixture_archive();
        let (_, package) = validate_archive_bytes(&bytes, &trust).expect("validated package");
        let diff = PdkTechnologyRevisionDiff::between(&package, &package).expect("diff");

        assert!(diff.entries.is_empty());
        assert!(!diff.migration_requires_review());
        assert!(!diff.has_breaking_changes());
    }

    #[test]
    fn display_only_layer_change_requires_review_but_is_not_structurally_breaking() {
        let (baseline, candidate) = pair(|manifest| {
            manifest.layers[0].display_rgba = [12, 34, 56, 255];
        });
        let diff = PdkTechnologyRevisionDiff::between(&baseline, &candidate).expect("diff");
        let layer = diff
            .entries
            .iter()
            .find(|entry| entry.area == PdkTechnologyDiffArea::Layer)
            .expect("layer change");

        assert_eq!(layer.identity, "active");
        assert_eq!(layer.kind, PdkTechnologyDiffKind::Changed);
        assert_eq!(layer.impact, PdkTechnologyDiffImpact::ReviewRequired);
        assert_eq!(diff.count(PdkTechnologyDiffImpact::Breaking), 0);
        assert!(diff.migration_requires_review());
    }

    #[test]
    fn runtime_and_process_contract_changes_are_breaking_and_deterministic() {
        let (baseline, candidate) = pair(|manifest| {
            manifest.process_node_nm = 130;
            manifest
                .compatibility
                .targets
                .retain(|target| *target != PdkExecutionTarget::Mobile);
        });
        let first = PdkTechnologyRevisionDiff::between(&baseline, &candidate).expect("first diff");
        let second =
            PdkTechnologyRevisionDiff::between(&baseline, &candidate).expect("second diff");

        assert_eq!(first, second);
        let encoded = serde_json::to_vec(&first).expect("serialize exact diff");
        let restored: PdkTechnologyRevisionDiff =
            serde_json::from_slice(&encoded).expect("deserialize exact diff");
        assert_eq!(restored, first);
        assert!(first.has_breaking_changes());
        assert!(first.entries.iter().any(|entry| {
            entry.identity == "process node (nm)"
                && entry.impact == PdkTechnologyDiffImpact::Breaking
        }));
        assert!(first.entries.iter().any(|entry| {
            entry.area == PdkTechnologyDiffArea::Compatibility
                && entry.impact == PdkTechnologyDiffImpact::Breaking
        }));
    }

    #[test]
    fn executable_model_source_contract_changes_are_breaking() {
        let (baseline, candidate) = pair(|manifest| {
            let tt = manifest
                .model_sources
                .iter_mut()
                .find(|contract| {
                    contract.process
                        == crate::state::pdk_config::technology_package::PdkModelProcess::Tt
                })
                .expect("fixture supplies TT");
            tt.sources[0].domain =
                crate::state::pdk_config::technology_package::PdkModelDomain::Mos;
            tt.required_domains =
                vec![crate::state::pdk_config::technology_package::PdkModelDomain::Mos];
        });
        let diff = PdkTechnologyRevisionDiff::between(&baseline, &candidate).expect("diff");
        assert!(diff.entries.iter().any(|entry| {
            entry.area == PdkTechnologyDiffArea::ModelSource
                && entry.identity == "tt"
                && entry.kind == PdkTechnologyDiffKind::Changed
                && entry.impact == PdkTechnologyDiffImpact::Breaking
        }));
    }

    #[test]
    fn signed_symbol_contract_addition_is_breaking() {
        let (baseline, candidate) = pair(|manifest| {
            let definition = fixture_signed_symbol(manifest);
            manifest.symbol_definitions.push(definition);
        });
        let diff = PdkTechnologyRevisionDiff::between(&baseline, &candidate).expect("diff");
        assert!(diff.entries.iter().any(|entry| {
            entry.area == PdkTechnologyDiffArea::Symbol
                && entry.identity == "demo180/nmos_demo"
                && entry.kind == PdkTechnologyDiffKind::Added
                && entry.impact == PdkTechnologyDiffImpact::Breaking
        }));
    }

    #[test]
    fn migration_evidence_is_exact_round_trippable_and_tamper_evident() {
        let (baseline, candidate) = pair(|manifest| {
            manifest.layers[0].display_rgba = [12, 34, 56, 255];
        });
        let diff = PdkTechnologyRevisionDiff::between(&baseline, &candidate).expect("diff");
        let evidence = PdkTechnologyMigrationEvidence::from_diff(&diff).expect("evidence");
        assert!(evidence.matches_diff(&diff));
        assert_eq!(
            evidence.entry_count(),
            evidence.breaking_count()
                + evidence.review_required_count()
                + evidence.informational_count()
        );

        let encoded = serde_json::to_vec(&evidence).expect("serialize evidence");
        let restored: PdkTechnologyMigrationEvidence =
            serde_json::from_slice(&encoded).expect("deserialize evidence");
        assert_eq!(restored, evidence);
        let mut tampered = evidence;
        tampered.review_required_count = tampered.review_required_count.saturating_add(1);
        assert!(tampered.validate().is_err());
        assert!(!tampered.matches_diff(&diff));
    }

    #[test]
    fn keyed_contract_removal_and_reverse_addition_are_explicit() {
        let (baseline, candidate) = pair(|manifest| {
            manifest.layers[2]
                .purposes
                .retain(|purpose| purpose != "pin");
            manifest
                .stream_map
                .retain(|entry| !(entry.layer == "metal1" && entry.purpose == "pin"));
        });
        let forward =
            PdkTechnologyRevisionDiff::between(&baseline, &candidate).expect("forward diff");
        let reverse =
            PdkTechnologyRevisionDiff::between(&candidate, &baseline).expect("reverse diff");

        assert!(forward.entries.iter().any(|entry| {
            entry.area == PdkTechnologyDiffArea::StreamMap
                && entry.identity == "metal1/pin"
                && entry.kind == PdkTechnologyDiffKind::Removed
                && entry.impact == PdkTechnologyDiffImpact::Breaking
        }));
        assert!(reverse.entries.iter().any(|entry| {
            entry.area == PdkTechnologyDiffArea::StreamMap
                && entry.identity == "metal1/pin"
                && entry.kind == PdkTechnologyDiffKind::Added
                && entry.impact == PdkTechnologyDiffImpact::Breaking
        }));
    }

    #[test]
    fn artifact_reclassification_is_breaking_if_either_side_is_executable() {
        let digest = crate::product::ContentDigest::from_bytes([7; 32]);
        let executable = PdkTechnologyArtifact {
            path: "models/device.lib".to_owned(),
            kind: PdkTechnologyArtifactKind::Model,
            size_bytes: 64,
            sha256: digest,
        };
        let documentation = PdkTechnologyArtifact {
            kind: PdkTechnologyArtifactKind::Documentation,
            ..executable.clone()
        };

        assert_eq!(
            artifact_impact(
                PdkTechnologyDiffKind::Changed,
                Some(&executable),
                Some(&documentation),
            ),
            PdkTechnologyDiffImpact::Breaking
        );
        assert_eq!(
            artifact_impact(
                PdkTechnologyDiffKind::Changed,
                Some(&documentation),
                Some(&executable),
            ),
            PdkTechnologyDiffImpact::Breaking
        );
    }

    #[test]
    fn callback_abi_contract_changes_are_explicit_and_breaking() {
        let (archive_bytes, _, _) = fixture_archive();
        let archive: SignedPdkTechnologyArchive =
            serde_json::from_slice(&archive_bytes).expect("archive fixture");
        let manifest_bytes = STANDARD
            .decode(&archive.manifest_base64)
            .expect("manifest base64");
        let manifest: PdkTechnologyManifest =
            serde_json::from_slice(&manifest_bytes).expect("manifest fixture");
        let before = manifest.callbacks;
        let mut after = before.clone();
        after[0].entrypoint = "derive_v2".to_owned();

        let mut entries = Vec::new();
        diff_keyed(
            &mut entries,
            PdkTechnologyDiffArea::Callback,
            &before,
            &after,
            |contract| contract.callback_id.to_ascii_lowercase(),
            |_, _, _| PdkTechnologyDiffImpact::Breaking,
        )
        .expect("callback diff");

        assert_eq!(entries.len(), 1);
        let entry = &entries[0];
        assert_eq!(entry.area, PdkTechnologyDiffArea::Callback);
        assert_eq!(entry.identity, before[0].callback_id);
        assert_eq!(entry.kind, PdkTechnologyDiffKind::Changed);
        assert_eq!(entry.impact, PdkTechnologyDiffImpact::Breaking);
        assert!(
            entry
                .before
                .as_deref()
                .is_some_and(|value| value.contains("\"entrypoint\":\"derive\""))
        );
        assert!(
            entry
                .after
                .as_deref()
                .is_some_and(|value| value.contains("\"entrypoint\":\"derive_v2\""))
        );
    }
}
