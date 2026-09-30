//! Unsigned draft validation against installed packages and app configuration persistence.

use rspice_model_library::pdk::{PdkTechnologyError, technology_draft::*};

#[test]
fn draft_requires_a_new_revision_and_preserves_exact_artifact_authority() {
    let (bytes, trust, _) = super::technology_package::tests::fixture_archive();
    let (_, package) = super::technology_package::validate_archive_bytes(&bytes, &trust)
        .expect("fixture validates");
    let mut draft = PdkTechnologyDraft::from_package(package.metadata());
    assert!(matches!(
        draft.validate_candidate(package.metadata()),
        Err(PdkTechnologyError::ImmutableRevision(_))
    ));

    draft.set_revision("2.4.0".to_owned());
    draft
        .validate_candidate(package.metadata())
        .expect("candidate validates");
    draft.manifest.artifacts.clear();
    assert!(matches!(
        draft.validate_candidate(package.metadata()),
        Err(PdkTechnologyError::InvalidReference(_))
    ));
}

#[test]
fn authoring_bundle_carries_source_files_without_private_signing_material() {
    let (bytes, trust, _) = super::technology_package::tests::fixture_archive();
    let (archive, package) = super::technology_package::validate_archive_bytes(&bytes, &trust)
        .expect("fixture validates");
    let mut draft = PdkTechnologyDraft::from_package(package.metadata());
    draft.set_revision("2.4.0".to_owned());
    let bundle = draft
        .authoring_bundle(package.metadata(), &archive)
        .expect("bundle builds");
    assert_eq!(bundle.source_files, archive.files);
    let json = serde_json::to_value(bundle).expect("bundle serializes");
    assert!(json.get("signature_base64").is_none());
}

#[test]
fn persisted_pdk_config_round_trips_an_invalid_in_progress_draft_without_authority() {
    let (bytes, trust, _) = super::technology_package::tests::fixture_archive();
    let (_, package) = super::technology_package::validate_archive_bytes(&bytes, &trust)
        .expect("fixture validates");
    let draft = PdkTechnologyDraft::from_package(package.metadata());
    assert!(draft.validate_candidate(package.metadata()).is_err());

    let mut config = super::PdkConfig::default();
    config.technology_draft = Some(draft.clone());
    let json = serde_json::to_string(&config).expect("PDK config serializes");
    let restored: super::PdkConfig = serde_json::from_str(&json).expect("PDK config restores");
    assert_eq!(restored.technology_draft, Some(draft));
    assert!(restored.technology_registry.validated_packages().is_empty());
}
