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

#[test]
fn a_preset_sealed_on_the_retired_compact_block_opens_on_its_standard_block() {
    // The compact block is retired: a preset written on it opens on the
    // standard block of the series its size was started from, while its
    // package still verifies against what was written, its frame-reference
    // manifest and review rows naming `compact`.
    let (_, trust, _) = signed_organization_package();
    for (fixture, trust) in [
        (PROJECT, PdkPublisherTrustStore::default()),
        (PROJECT_REVIEW, PdkPublisherTrustStore::default()),
        (ORGANIZATION, trust),
    ] {
        let package = parse_package(fixture, &trust).unwrap();
        let template = |id: &str| {
            package
                .presets
                .iter()
                .find(|preset| preset.stable_id == id)
                .unwrap()
                .format
                .title_block
                .template
        };
        assert_eq!(
            template("compact-left"),
            DrawingSheetTitleBlockTemplate::Standard
        );
        assert_eq!(
            template("ansi-top"),
            DrawingSheetTitleBlockTemplate::AsmeY141
        );
        assert_eq!(
            template("iso-strip"),
            DrawingSheetTitleBlockTemplate::Standard
        );
        assert!(
            package
                .builtin_frame_references
                .contains(&BuiltinFrameReference {
                    kind: "title-block".to_owned(),
                    identity: "compact".to_owned(),
                })
        );
    }
    // Sealed again, the package is written with today's blocks throughout.
    let project = parse_package(PROJECT, &PdkPublisherTrustStore::default()).unwrap();
    let presets = project.presets.iter().map(|preset| DrawingSheetPreset {
        id: preset.stable_id.clone(),
        name: preset.name.clone(),
        scope: preset.source_scope,
        format: preset.format.clone(),
    });
    let resealed = encode_package(&build_package(presets).unwrap()).unwrap();
    assert!(!resealed.contains(r#""template": "compact""#), "{resealed}");
    assert!(!resealed.contains(r#""identity": "compact""#), "{resealed}");
    assert!(
        resealed.contains(r#""template": "asme-y14-1""#),
        "{resealed}"
    );
}

#[test]
fn a_preset_sealed_with_a_retired_placement_opens_upright_in_the_lower_right() {
    // Anchors other than the lower right corner, turned blocks and offsets
    // are retired: each preset opens with its block upright in the lower
    // right corner of its drawing area, while its package still verifies
    // against the placement that was written.
    let (_, trust, _) = signed_organization_package();
    for (fixture, trust) in [
        (PROJECT, PdkPublisherTrustStore::default()),
        (PROJECT_REVIEW, PdkPublisherTrustStore::default()),
        (ORGANIZATION, trust),
    ] {
        let package = parse_package(fixture, &trust).unwrap();
        let mut placed = 0;
        for preset in &package.presets {
            let geometry = preset.format.geometry().unwrap();
            let Some(block) = geometry.title_block else {
                continue;
            };
            let area = geometry.drawing_area;
            assert_eq!(
                block.x_um + block.width_um as i64,
                area.x_um + area.width_um as i64,
                "{}",
                preset.stable_id
            );
            assert_eq!(
                block.y_um + block.height_um as i64,
                area.y_um + area.height_um as i64,
                "{}",
                preset.stable_id
            );
            placed += 1;
        }
        assert_eq!(placed, package.presets.len());
    }
    let project = parse_package(PROJECT, &PdkPublisherTrustStore::default()).unwrap();
    let presets = project.presets.iter().map(|preset| DrawingSheetPreset {
        id: preset.stable_id.clone(),
        name: preset.name.clone(),
        scope: preset.source_scope,
        format: preset.format.clone(),
    });
    let resealed = encode_package(&build_package(presets).unwrap()).unwrap();
    for retired in [
        r#""anchor""#,
        r#""rotation""#,
        r#""offset_x_um""#,
        r#""offset_y_um""#,
    ] {
        assert!(!resealed.contains(retired), "{retired}: {resealed}");
    }
}
