//! Packages sealed before the ASME form's title-block fields still open, and
//! their digests and signatures still verify.
//!
//! The fixtures are what the build at 6db8fbd44f, the commit the ASME form
//! was built on, wrote for three custom presets: a compact block anchored
//! bottom left, turned clockwise, offset and declaring a 2:1 scale; ISO
//! 7200's block in a bottom strip declaring no scale; and a compact block on
//! an ANSI-started size anchored top right, turned counter-clockwise and
//! offset. That build had no CAGE code, engineer, signature dates or
//! contract number.

use super::*;

const PROJECT: &str = include_str!("before_asme_form/project.json");
const PROJECT_REVIEW: &str = include_str!("before_asme_form/project-review.json");
const ORGANIZATION: &str = include_str!("before_asme_form/organization.json");

/// The fields the product gained with the ASME form.
const GAINED: [DrawingSheetTitleFieldId; 7] = [
    DrawingSheetTitleFieldId::CageCode,
    DrawingSheetTitleFieldId::Engineer,
    DrawingSheetTitleFieldId::DrawnDate,
    DrawingSheetTitleFieldId::CheckedDate,
    DrawingSheetTitleFieldId::EngineerDate,
    DrawingSheetTitleFieldId::ApprovedDate,
    DrawingSheetTitleFieldId::ContractNumber,
];

fn sealed_digest(package: &str) -> String {
    let wire = serde_json::from_str::<serde_json::Value>(package).unwrap();
    let wire = wire.get("package").unwrap_or(&wire);
    wire["source_digest_sha256"].as_str().unwrap().to_owned()
}

/// Every preset loads with the fields the product has today, the ones it
/// gained hidden and empty.
fn assert_gained_fields_supplied(package: &DrawingSheetPresetPackage) {
    assert_eq!(package.presets.len(), 3);
    for preset in &package.presets {
        let fields = &preset.format.title_block.fields;
        assert_eq!(fields.len(), DrawingSheetTitleFieldId::ALL.len());
        for field in GAINED {
            assert!(!fields[&field].visible, "{field:?}");
            assert!(fields[&field].value.is_empty(), "{field:?}");
        }
    }
}

#[test]
fn a_package_sealed_before_the_asme_fields_still_opens_and_verifies() {
    for fixture in [PROJECT, PROJECT_REVIEW, ORGANIZATION] {
        assert!(
            !fixture.contains("cage-code"),
            "sealed without the new fields"
        );
    }

    let project = parse_package(PROJECT, &PdkPublisherTrustStore::default()).unwrap();
    assert_gained_fields_supplied(&project);
    assert_eq!(
        inspect_drawing_sheet_package(PROJECT)
            .unwrap()
            .source_digest_sha256,
        sealed_digest(PROJECT)
    );
    let review = parse_package(PROJECT_REVIEW, &PdkPublisherTrustStore::default()).unwrap();
    assert_eq!(review, project);

    let (_, trust, signing_key) = signed_organization_package();
    let organization = parse_package(ORGANIZATION, &trust).unwrap();
    assert_gained_fields_supplied(&organization);
    let verification = verify_published_drawing_sheet_package(
        ORGANIZATION,
        &signing_key.verifying_key().to_bytes(),
    )
    .unwrap();
    assert_eq!(
        verification.source_digest_sha256,
        sealed_digest(ORGANIZATION)
    );
}

#[test]
fn tampering_with_a_package_sealed_before_the_asme_fields_is_still_caught() {
    let (_, trust, _) = signed_organization_package();
    for (fixture, trust) in [
        (PROJECT, PdkPublisherTrustStore::default()),
        (ORGANIZATION, trust),
    ] {
        let tampered = fixture.replacen("\"bleed_um\": 0", "\"bleed_um\": 1", 1);
        assert_ne!(tampered, fixture);
        assert!(parse_package(&tampered, &trust).is_err());
    }
}
