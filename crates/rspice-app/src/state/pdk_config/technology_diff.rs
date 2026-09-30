//! Application integration tests for signed-PDK revision comparisons.

pub use rspice_model_library::pdk::diff::*;

#[cfg(test)]
pub(crate) mod tests {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    use ed25519_dalek::{Signer as _, SigningKey};

    use super::*;
    use rspice_model_library::pdk::contracts::{PdkExecutionTarget, SignedPdkTechnologyArchive};
    use rspice_model_library::pdk::manifest::PdkTechnologyManifest;
    use rspice_model_library::pdk::{PdkAdministrativeAuthority, PdkPublisherTrustStore};
    use rspice_simulation::pdk::test_fixtures::{fixture_archive, fixture_signed_symbol};
    use rspice_simulation::pdk::{ValidatedPdkTechnologyPackage, validate_archive_bytes};

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
        let diff = PdkTechnologyRevisionDiff::between(package.metadata(), package.metadata())
            .expect("diff");

        assert!(diff.entries.is_empty());
        assert!(!diff.migration_requires_review());
        assert!(!diff.has_breaking_changes());
    }

    #[test]
    fn display_only_layer_change_requires_review_but_is_not_structurally_breaking() {
        let (baseline, candidate) = pair(|manifest| {
            manifest.layers[0].display_rgba = [12, 34, 56, 255];
        });
        let diff = PdkTechnologyRevisionDiff::between(baseline.metadata(), candidate.metadata())
            .expect("diff");
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
        let first = PdkTechnologyRevisionDiff::between(baseline.metadata(), candidate.metadata())
            .expect("first diff");
        let second = PdkTechnologyRevisionDiff::between(baseline.metadata(), candidate.metadata())
            .expect("second diff");

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
                    contract.process == rspice_model_library::pdk::contracts::PdkModelProcess::Tt
                })
                .expect("fixture supplies TT");
            tt.sources[0].domain = rspice_model_library::pdk::contracts::PdkModelDomain::Mos;
            tt.required_domains = vec![rspice_model_library::pdk::contracts::PdkModelDomain::Mos];
        });
        let diff = PdkTechnologyRevisionDiff::between(baseline.metadata(), candidate.metadata())
            .expect("diff");
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
        let diff = PdkTechnologyRevisionDiff::between(baseline.metadata(), candidate.metadata())
            .expect("diff");
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
        let diff = PdkTechnologyRevisionDiff::between(baseline.metadata(), candidate.metadata())
            .expect("diff");
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
        let mut tampered: serde_json::Value =
            serde_json::from_slice(&encoded).expect("encoded evidence");
        tampered["review_required_count"] =
            serde_json::json!(evidence.review_required_count().saturating_add(1));
        let tampered: PdkTechnologyMigrationEvidence =
            serde_json::from_value(tampered).expect("tampered evidence shape");
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
        let forward = PdkTechnologyRevisionDiff::between(baseline.metadata(), candidate.metadata())
            .expect("forward diff");
        let reverse = PdkTechnologyRevisionDiff::between(candidate.metadata(), baseline.metadata())
            .expect("reverse diff");

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
}
