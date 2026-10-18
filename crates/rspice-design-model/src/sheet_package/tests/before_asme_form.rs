//! Packages sealed before the ASME form's title-block fields still open, and
//! their digests and signatures still verify.
//!
//! Each fixture set is what one build wrote for the same three custom
//! presets: a compact block anchored bottom left, turned clockwise, offset
//! and declaring a 2:1 scale; ISO 7200's block in a bottom strip declaring
//! no scale; and a compact block on an ANSI-started size anchored top right,
//! turned counter-clockwise and offset. The first set is from 6db8fbd44f, the
//! commit the ASME form was first built on, which also wrote each preset's
//! own margins, border, zones and marks; the second is from 329472e3f2, the
//! commit it was rebased onto, which wrote the standard frames. Neither
//! build had a CAGE code, engineer, signature dates or contract number.

use super::*;

/// One build's packages: a project package, its human-review form, and an
/// organization package signed with the test publisher's key.
struct Sealed {
    project: &'static str,
    review: &'static str,
    organization: &'static str,
}

const SEALED: [Sealed; 2] = [
    Sealed {
        project: include_str!("before_asme_form/project.json"),
        review: include_str!("before_asme_form/project-review.json"),
        organization: include_str!("before_asme_form/organization.json"),
    },
    Sealed {
        project: include_str!("before_asme_form/standard-frame/project.json"),
        review: include_str!("before_asme_form/standard-frame/project-review.json"),
        organization: include_str!("before_asme_form/standard-frame/organization.json"),
    },
];

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

/// Every package of every build, each with the trust it is opened under.
fn every_package() -> Vec<(&'static str, PdkPublisherTrustStore)> {
    let (_, trust, _) = signed_organization_package();
    SEALED
        .iter()
        .flat_map(|sealed| {
            [
                (sealed.project, PdkPublisherTrustStore::default()),
                (sealed.review, PdkPublisherTrustStore::default()),
                (sealed.organization, trust.clone()),
            ]
        })
        .collect()
}

/// `package`'s presets sealed again by this build, as a project package: an
/// organization's needs its publisher's signing tool.
fn resealed(package: &DrawingSheetPresetPackage) -> String {
    let presets = package.presets.iter().map(|preset| DrawingSheetPreset {
        id: preset.stable_id.clone(),
        name: preset.name.clone(),
        scope: DrawingSheetPresetScope::Project,
        format: preset.format.clone(),
    });
    encode_package(&build_package(presets).unwrap()).unwrap()
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
    let (_, trust, signing_key) = signed_organization_package();
    for sealed in &SEALED {
        for fixture in [sealed.project, sealed.review, sealed.organization] {
            assert!(
                !fixture.contains("cage-code"),
                "sealed without the new fields"
            );
        }

        let project = parse_package(sealed.project, &PdkPublisherTrustStore::default()).unwrap();
        assert_gained_fields_supplied(&project);
        assert_eq!(
            inspect_drawing_sheet_package(sealed.project)
                .unwrap()
                .source_digest_sha256,
            sealed_digest(sealed.project)
        );
        let review = parse_package(sealed.review, &PdkPublisherTrustStore::default()).unwrap();
        assert_eq!(review, project);

        let organization = parse_package(sealed.organization, &trust).unwrap();
        assert_gained_fields_supplied(&organization);
        let verification = verify_published_drawing_sheet_package(
            sealed.organization,
            &signing_key.verifying_key().to_bytes(),
        )
        .unwrap();
        assert_eq!(
            verification.source_digest_sha256,
            sealed_digest(sealed.organization)
        );
    }
}

#[test]
fn tampering_with_a_package_sealed_before_the_asme_fields_is_still_caught() {
    let (_, trust, _) = signed_organization_package();
    for sealed in &SEALED {
        for (fixture, trust) in [
            (sealed.project, PdkPublisherTrustStore::default()),
            (sealed.organization, trust.clone()),
        ] {
            let tampered = fixture.replacen("\"bleed_um\": 0", "\"bleed_um\": 1", 1);
            assert_ne!(tampered, fixture);
            assert!(parse_package(&tampered, &trust).is_err());
        }
    }
}

#[test]
fn a_preset_sealed_on_the_retired_compact_block_opens_on_its_standard_block() {
    // The compact block is retired: a preset written on it opens on the
    // standard block of the series its size was started from, while its
    // package still verifies against what was written, its frame-reference
    // manifest and review rows naming `compact`.
    for (fixture, trust) in every_package() {
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
        // Sealed again, the package is written with today's blocks
        // throughout.
        let resealed = resealed(&package);
        assert!(!resealed.contains(r#""template": "compact""#), "{resealed}");
        assert!(!resealed.contains(r#""identity": "compact""#), "{resealed}");
        assert!(
            resealed.contains(r#""template": "asme-y14-1""#),
            "{resealed}"
        );
    }
}

#[test]
fn a_preset_sealed_with_a_retired_placement_opens_upright_in_the_lower_right() {
    // Anchors other than the lower right corner, turned blocks and offsets
    // are retired: each preset opens with its block upright in the lower
    // right corner of its drawing area, while its package still verifies
    // against the placement that was written.
    for (fixture, trust) in every_package() {
        let package = parse_package(fixture, &trust).unwrap();
        for preset in &package.presets {
            let geometry = preset.format.geometry().unwrap();
            let block = geometry.title_block.expect("the block fits its sheet");
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
        }
        let resealed = resealed(&package);
        for retired in [
            r#""anchor""#,
            r#""rotation""#,
            r#""offset_x_um""#,
            r#""offset_y_um""#,
        ] {
            assert!(!resealed.contains(retired), "{retired}: {resealed}");
        }
    }
}

#[test]
fn a_preset_sealed_declaring_a_scale_opens_declaring_none() {
    // A title block no longer declares a scale: the presets declaring 2:1,
    // none and 1:1 open alike, while their packages still verify against the
    // scale each was written with (see the first test), and a package sealed
    // again declares none.
    for (fixture, trust) in every_package() {
        assert!(fixture.contains(r#""drawing_units": 2"#));
        assert!(fixture.contains(r#""kind": "not-to-scale""#));
        let package = parse_package(fixture, &trust).unwrap();
        for preset in &package.presets {
            let written = serde_json::to_value(&preset.format).unwrap();
            assert!(
                written["title_block"].get("scale").is_none(),
                "{}",
                preset.stable_id
            );
        }
        let resealed = resealed(&package);
        assert!(!resealed.contains(r#""scale""#), "{resealed}");
    }
}
