//! Display-profile integration with installed packages and persisted audit tampering.

use super::{PdkAdministrativeAuthority, ValidatedPdkTechnologyPackage};
use crate::state::pdk_config::technology_package::tests::fixture_archive;
use rspice_model_library::pdk::display_profile::*;

fn fixture() -> (ValidatedPdkTechnologyPackage, PdkAdministrativeAuthority) {
    let (bytes, trust, authority) = fixture_archive();
    let mut registry = super::technology_package::PdkTechnologyRegistry::default();
    registry
        .install_archive_bytes(&bytes, &trust, &authority, "install display fixture")
        .expect("install");
    (registry.validated_packages()[0].clone(), authority)
}

#[test]
fn publication_is_immutable_versioned_and_exactly_package_bound() {
    let (package, authority) = fixture();
    let mut registry = PdkDisplayProfileRegistry::default();
    let mut draft =
        PdkDisplayProfileDraft::signed_defaults(package.metadata(), "layout-dark", "Layout dark");
    let first = registry
        .publish_and_activate(
            package.metadata(),
            draft.clone(),
            &authority,
            "initial profile",
        )
        .expect("publish");
    draft.entries[0].screen_rgba = [20, 200, 80, 255];
    let second = registry
        .publish_and_activate(
            package.metadata(),
            draft,
            &authority,
            "improve active contrast",
        )
        .expect("publish revision");

    assert_eq!(first.target.revision, 1);
    assert_eq!(second.target.revision, 2);
    assert_ne!(first.target.profile_digest, second.target.profile_digest);
    assert_eq!(registry.revisions().len(), 2);
    assert_eq!(
        registry
            .active_for_package(package.metadata())
            .map(|profile| profile.revision),
        Some(2)
    );
    registry.validate_audit_chain().expect("audit");
}

#[test]
fn incomplete_or_foreign_layer_contract_fails_closed() {
    let (package, authority) = fixture();
    let mut registry = PdkDisplayProfileRegistry::default();
    let mut missing =
        PdkDisplayProfileDraft::signed_defaults(package.metadata(), "layout-dark", "Layout dark");
    missing.entries.pop();
    assert!(matches!(
        registry.publish_and_activate(package.metadata(), missing, &authority, "missing row"),
        Err(PdkDisplayProfileError::MissingLayerPurposes(_))
    ));

    let mut foreign =
        PdkDisplayProfileDraft::signed_defaults(package.metadata(), "layout-dark", "Layout dark");
    foreign.entries[0].layer = "unknown".to_owned();
    assert!(matches!(
        registry.publish_and_activate(package.metadata(), foreign, &authority, "foreign row"),
        Err(PdkDisplayProfileError::UnknownLayerPurpose(_))
    ));
    assert!(registry.revisions().is_empty());
    assert!(registry.audit().is_empty());
}

#[test]
fn unavailable_project_and_organization_scopes_fail_closed() {
    let (package, authority) = fixture();
    let mut registry = PdkDisplayProfileRegistry::default();
    for scope in [
        PdkDisplayProfileScope::Project,
        PdkDisplayProfileScope::Organization,
    ] {
        let mut draft = PdkDisplayProfileDraft::signed_defaults(
            package.metadata(),
            "layout-dark",
            "Layout dark",
        );
        draft.scope = scope;
        assert!(matches!(
            registry.publish_and_activate(
                package.metadata(),
                draft,
                &authority,
                "reject unavailable scope"
            ),
            Err(PdkDisplayProfileError::InvalidField(_))
        ));
    }
    assert!(registry.revisions().is_empty());
    assert!(registry.audit().is_empty());
}

#[test]
fn rollback_requires_exact_previously_active_revision() {
    let (package, authority) = fixture();
    let mut registry = PdkDisplayProfileRegistry::default();
    let mut draft =
        PdkDisplayProfileDraft::signed_defaults(package.metadata(), "layout-dark", "Layout dark");
    registry
        .publish_and_activate(
            package.metadata(),
            draft.clone(),
            &authority,
            "revision one",
        )
        .expect("publish one");
    draft.label = "Layout dark adjusted".to_owned();
    registry
        .publish_and_activate(package.metadata(), draft, &authority, "revision two")
        .expect("publish two");
    let receipt = registry
        .rollback_to(
            package.metadata(),
            "layout-dark",
            1,
            &authority,
            "restore known display",
        )
        .expect("rollback");

    assert_eq!(receipt.action, PdkDisplayProfileAuditAction::Rollback);
    assert_eq!(receipt.target.revision, 1);
    assert_eq!(
        registry
            .active_for_package(package.metadata())
            .map(|profile| profile.revision),
        Some(1)
    );
}

#[test]
fn tampered_revision_or_receipt_is_rejected() {
    let (package, authority) = fixture();
    let mut registry = PdkDisplayProfileRegistry::default();
    registry
        .publish_and_activate(
            package.metadata(),
            PdkDisplayProfileDraft::signed_defaults(
                package.metadata(),
                "layout-dark",
                "Layout dark",
            ),
            &authority,
            "initial profile",
        )
        .expect("publish");
    let mut revision_tamper = serde_json::to_value(&registry).expect("serialize registry");
    revision_tamper["revisions"][0]["entries"][0]["visible"] = false.into();
    let revision_tamper: PdkDisplayProfileRegistry =
        serde_json::from_value(revision_tamper).expect("restore tampered registry");
    assert!(matches!(
        revision_tamper.validate_audit_chain(),
        Err(PdkDisplayProfileError::Corrupted(_))
    ));
    let mut receipt_tamper = serde_json::to_value(&registry).expect("serialize registry");
    receipt_tamper["audit"][0]["reason"] = "rewritten".into();
    let receipt_tamper: PdkDisplayProfileRegistry =
        serde_json::from_value(receipt_tamper).expect("restore tampered registry");
    assert!(matches!(
        receipt_tamper.validate_audit_chain(),
        Err(PdkDisplayProfileError::Corrupted(_))
    ));
}
