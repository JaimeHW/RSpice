//! Packages sealed before the product's title-block fields last changed
//! still open, and their digests and signatures still verify.

use base64::Engine as _;
use ed25519_dalek::Signer as _;
use sha2::Digest as _;

use super::*;

/// An organization package as the build before the language code sealed and
/// signed it, byte for byte.
const SIGNED_WITHOUT_LANGUAGE: &str = r#"{
  "schema": "rspice-sheet-formats",
  "version": 1,
  "source_digest_sha256": "c34870b03da0821790a382c6ea89f29a3c40372e83c45aebd3a59074c8feebc1",
  "includes_builtin_frame_references": true,
  "includes_source_metadata": true,
  "builtin_frame_references": [
    {
      "kind": "border",
      "identity": "standard"
    },
    {
      "kind": "title-block",
      "identity": "standard"
    }
  ],
  "source_metadata": {
    "producer": "RSpice",
    "content": "drawing-sheet-format-definitions",
    "exact_dimension_unit": "micrometre",
    "source_scopes": [
      "organization"
    ]
  },
  "publisher_signature": {
    "publisher_id": "rspice-test-publisher",
    "signing_key_id": "sheet-formats-2026",
    "signature_base64": "IyXiOeHt0EVr2fCr5qZjASmFIzAjeBJqA7Fkjqpkr5z3pgXwsv5255tb6/cgiluTtUX8gbBEqYfpaQBVKmNcCQ=="
  },
  "presets": [
    {
      "stable_id": "organization-panel",
      "name": "Organization panel",
      "source_scope": "organization",
      "portrait_width_um": 250000,
      "portrait_height_um": 400000,
      "format": {
        "size": {
          "kind": "custom",
          "name": "Organization panel",
          "portrait_width_um": 250000,
          "portrait_height_um": 400000
        },
        "orientation": "portrait",
        "authored_size": {
          "kind": "custom",
          "snapshot": {
            "preset_id": "organization-panel",
            "name": "Organization panel",
            "portrait_width_um": 250000,
            "portrait_height_um": 400000,
            "source_preset_unavailable": false
          }
        },
        "display_unit": "millimetres",
        "margins": {
          "top_um": 10000,
          "right_um": 10000,
          "bottom_um": 10000,
          "left_um": 20000
        },
        "bleed_um": 0,
        "border": "standard",
        "zones": {
          "mode": "automatic",
          "custom_columns": null,
          "custom_rows": null,
          "labels": "alpha-numeric",
          "edges": "all"
        },
        "marks": {
          "registration": true,
          "folding": false
        },
        "title_block": {
          "template": "standard",
          "anchor": "bottom-right",
          "rotation": "upright",
          "offset_x_um": 0,
          "offset_y_um": 0,
          "scale": {
            "kind": "ratio",
            "drawing_units": 1,
            "reality_units": 1
          },
          "fields": {
            "project": {
              "value": "",
              "visible": false
            },
            "cell-view": {
              "value": "",
              "visible": true
            },
            "sheet-title": {
              "value": "",
              "visible": true
            },
            "page": {
              "value": "",
              "visible": true
            },
            "revision": {
              "value": "",
              "visible": true
            },
            "format": {
              "value": "",
              "visible": false
            },
            "drawn-by": {
              "value": "",
              "visible": true
            },
            "checked-by": {
              "value": "",
              "visible": false
            },
            "approved-by": {
              "value": "",
              "visible": true
            },
            "date": {
              "value": "",
              "visible": true
            },
            "organization": {
              "value": "",
              "visible": true
            },
            "document-id": {
              "value": "",
              "visible": true
            },
            "classification": {
              "value": "",
              "visible": false
            },
            "document-type": {
              "value": "",
              "visible": true
            },
            "document-status": {
              "value": "",
              "visible": true
            }
          }
        },
        "inheritance": "explicit"
      }
    }
  ]
}"#;

/// A project package as the same build sealed it, byte for byte.
const UNSIGNED_WITHOUT_LANGUAGE: &str = r#"{
  "schema": "rspice-sheet-formats",
  "version": 1,
  "source_digest_sha256": "480b6c8b86509c7e2bda48c2c696483bb7150667166b25db5460c3c25f414779",
  "includes_builtin_frame_references": true,
  "includes_source_metadata": true,
  "builtin_frame_references": [
    {
      "kind": "border",
      "identity": "standard"
    },
    {
      "kind": "title-block",
      "identity": "standard"
    }
  ],
  "source_metadata": {
    "producer": "RSpice",
    "content": "drawing-sheet-format-definitions",
    "exact_dimension_unit": "micrometre",
    "source_scopes": [
      "project"
    ]
  },
  "presets": [
    {
      "stable_id": "custom-lab",
      "name": "Lab panel",
      "source_scope": "project",
      "portrait_width_um": 250001,
      "portrait_height_um": 400002,
      "format": {
        "size": {
          "kind": "custom",
          "name": "Lab panel",
          "portrait_width_um": 250001,
          "portrait_height_um": 400002
        },
        "orientation": "portrait",
        "authored_size": {
          "kind": "custom",
          "snapshot": {
            "preset_id": "custom-lab",
            "name": "Lab panel",
            "portrait_width_um": 250001,
            "portrait_height_um": 400002,
            "source_preset_unavailable": false
          }
        },
        "display_unit": "millimetres",
        "margins": {
          "top_um": 10000,
          "right_um": 10000,
          "bottom_um": 10000,
          "left_um": 20000
        },
        "bleed_um": 0,
        "border": "standard",
        "zones": {
          "mode": "automatic",
          "custom_columns": null,
          "custom_rows": null,
          "labels": "alpha-numeric",
          "edges": "all"
        },
        "marks": {
          "registration": true,
          "folding": false
        },
        "title_block": {
          "template": "standard",
          "anchor": "bottom-right",
          "rotation": "upright",
          "offset_x_um": 0,
          "offset_y_um": 0,
          "scale": {
            "kind": "ratio",
            "drawing_units": 1,
            "reality_units": 1
          },
          "fields": {
            "project": {
              "value": "",
              "visible": false
            },
            "cell-view": {
              "value": "",
              "visible": true
            },
            "sheet-title": {
              "value": "",
              "visible": true
            },
            "page": {
              "value": "",
              "visible": true
            },
            "revision": {
              "value": "",
              "visible": true
            },
            "format": {
              "value": "",
              "visible": false
            },
            "drawn-by": {
              "value": "",
              "visible": true
            },
            "checked-by": {
              "value": "",
              "visible": false
            },
            "approved-by": {
              "value": "",
              "visible": true
            },
            "date": {
              "value": "",
              "visible": true
            },
            "organization": {
              "value": "",
              "visible": true
            },
            "document-id": {
              "value": "",
              "visible": true
            },
            "classification": {
              "value": "",
              "visible": false
            },
            "document-type": {
              "value": "",
              "visible": true
            },
            "document-status": {
              "value": "",
              "visible": true
            }
          }
        },
        "inheritance": "explicit"
      }
    }
  ]
}"#;

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// One title-block field as the canonical contract writes it, preceded by
/// the comma that separates it from the field before.
fn canonical_entry(key: &str, visible: bool) -> String {
    format!(r#","{key}":{{"value":"","visible":{visible}}}"#)
}

/// `canonical` as a build without the language code wrote it: the field
/// written last is the one it lacks.
fn without_language(canonical: &[u8]) -> String {
    let today = String::from_utf8(canonical.to_vec()).unwrap();
    let entry = canonical_entry("language", false);
    assert!(today.contains(&entry), "{today}");
    today.replacen(&entry, "", 1)
}

/// `canonical` as a build from before the document type and status, and
/// with the retired scale field, wrote it.
fn as_an_older_build_wrote(canonical: &str) -> String {
    let format_entry = canonical_entry("format", false);
    let older = canonical
        .replacen(&canonical_entry("document-type", true), "", 1)
        .replacen(&canonical_entry("document-status", true), "", 1)
        .replacen(
            &format_entry,
            &format!("{format_entry}{}", canonical_entry("scale", true)),
            1,
        );
    assert_ne!(older, canonical);
    older
}

/// `fixture` with its preset's fields as the older build wrote them, sealed
/// with `digest` and, when signed, `signature`.
fn written_by_an_older_build(fixture: &str, digest: &str, signature: Option<&str>) -> String {
    let mut wire = serde_json::from_str::<serde_json::Value>(fixture).unwrap();
    let fields = wire["presets"][0]["format"]["title_block"]["fields"]
        .as_object_mut()
        .unwrap();
    fields.remove("document-type");
    fields.remove("document-status");
    fields.insert(
        "scale".to_owned(),
        serde_json::json!({ "value": "", "visible": true }),
    );
    wire["source_digest_sha256"] = serde_json::json!(digest);
    if let Some(signature) = signature {
        wire["publisher_signature"]["signature_base64"] = serde_json::json!(signature);
    }
    serde_json::to_string_pretty(&wire).unwrap()
}

#[test]
fn a_package_sealed_before_the_language_code_still_opens_and_verifies() {
    let (_, trust, signing_key) = signed_organization_package();
    let signed = parse_package(SIGNED_WITHOUT_LANGUAGE, &trust).unwrap();
    // It loads with the field the product has gained since, hidden.
    let fields = &signed.presets[0].format.title_block.fields;
    assert!(!fields[&DrawingSheetTitleFieldId::Language].visible);
    assert_eq!(fields.len(), DrawingSheetTitleFieldId::ALL.len());
    let verification = verify_published_drawing_sheet_package(
        SIGNED_WITHOUT_LANGUAGE,
        &signing_key.verifying_key().to_bytes(),
    )
    .unwrap();
    assert_eq!(
        verification.source_digest_sha256,
        "c34870b03da0821790a382c6ea89f29a3c40372e83c45aebd3a59074c8feebc1"
    );

    let unsigned = parse_package(
        UNSIGNED_WITHOUT_LANGUAGE,
        &PdkPublisherTrustStore::default(),
    )
    .unwrap();
    assert!(
        !unsigned.presets[0].format.title_block.fields[&DrawingSheetTitleFieldId::Language].visible
    );
    let inspection = inspect_drawing_sheet_package(UNSIGNED_WITHOUT_LANGUAGE).unwrap();
    assert_eq!(
        inspection.source_digest_sha256,
        "480b6c8b86509c7e2bda48c2c696483bb7150667166b25db5460c3c25f414779"
    );
}

/// The contract a build today writes for the same presets, less the field it
/// has gained, is exactly what the older build sealed: the fixtures are what
/// that build wrote, not what this one expects.
#[test]
fn todays_contract_less_the_gained_field_is_what_the_older_build_sealed() {
    let (organization, _, _) = signed_organization_package();
    let organization_contract = canonical_package_contract(&organization).unwrap();
    assert_eq!(
        sha256_hex(without_language(&organization_contract).as_bytes()),
        "c34870b03da0821790a382c6ea89f29a3c40372e83c45aebd3a59074c8feebc1"
    );
    assert_eq!(
        sha256_hex(
            without_language(&canonical_package_contract(&lab_panel_package()).unwrap()).as_bytes()
        ),
        "480b6c8b86509c7e2bda48c2c696483bb7150667166b25db5460c3c25f414779"
    );
}

/// The project package of [`UNSIGNED_WITHOUT_LANGUAGE`] as a build today
/// seals it.
fn lab_panel_package() -> DrawingSheetPresetPackage {
    build_package([DrawingSheetPreset {
        id: "custom-lab".to_owned(),
        name: "Lab panel".to_owned(),
        scope: DrawingSheetPresetScope::Project,
        format: custom_format("Lab panel", 250_001, 400_002, StartingFrame::IsoA).unwrap(),
    }])
    .unwrap()
}

#[test]
fn a_package_sealed_before_the_document_fields_and_with_scale_still_verifies() {
    let (organization, trust, signing_key) = signed_organization_package();
    let older = as_an_older_build_wrote(&without_language(
        &canonical_package_contract(&organization).unwrap(),
    ));
    let mut message = PACKAGE_SIGNATURE_DOMAIN.to_vec();
    message.extend_from_slice(older.as_bytes());
    let signature = STANDARD.encode(signing_key.sign(&message).to_bytes());
    let signed = written_by_an_older_build(
        SIGNED_WITHOUT_LANGUAGE,
        &sha256_hex(older.as_bytes()),
        Some(&signature),
    );
    let restored = parse_package(&signed, &trust).unwrap();
    verify_published_drawing_sheet_package(&signed, &signing_key.verifying_key().to_bytes())
        .unwrap();
    // It loads with every field the product has, and without the retired one.
    let fields = &restored.presets[0].format.title_block.fields;
    assert!(fields[&DrawingSheetTitleFieldId::DocumentType].visible);
    assert!(fields[&DrawingSheetTitleFieldId::DocumentStatus].visible);
    assert!(!fields.contains_key(&DrawingSheetTitleFieldId::Scale));

    let today = lab_panel_package();
    let older = as_an_older_build_wrote(&without_language(
        &canonical_package_contract(&today).unwrap(),
    ));
    let unsigned = written_by_an_older_build(
        UNSIGNED_WITHOUT_LANGUAGE,
        &sha256_hex(older.as_bytes()),
        None,
    );
    parse_package(&unsigned, &PdkPublisherTrustStore::default()).unwrap();
    // The human-review form of the same package opens as well: its review
    // rows name no field.
    let review =
        encode_package_with_format(&today, DrawingSheetPackageEncoding::HumanReviewJson).unwrap();
    let mut review = serde_json::from_str::<serde_json::Value>(&review).unwrap();
    review["package"] = serde_json::from_str(&unsigned).unwrap();
    parse_package(&review.to_string(), &PdkPublisherTrustStore::default()).unwrap();
}

#[test]
fn tampering_with_an_older_package_is_still_caught() {
    let (_, trust, _) = signed_organization_package();
    let checked_by_hidden = r#""checked-by": {
              "value": "",
              "visible": false"#;
    // A field it was written with, changed.
    let shown = SIGNED_WITHOUT_LANGUAGE.replacen(
        checked_by_hidden,
        &checked_by_hidden.replace("false", "true"),
        1,
    );
    assert_ne!(shown, SIGNED_WITHOUT_LANGUAGE);
    assert!(parse_package(&shown, &trust).is_err());
    // A field it was written with, removed.
    let mut wire = serde_json::from_str::<serde_json::Value>(UNSIGNED_WITHOUT_LANGUAGE).unwrap();
    wire["presets"][0]["format"]["title_block"]["fields"]
        .as_object_mut()
        .unwrap()
        .remove("checked-by");
    assert!(parse_package(&wire.to_string(), &PdkPublisherTrustStore::default()).is_err());
    // A field it was not written with, added: the contract it was sealed
    // over no longer holds.
    let mut wire = serde_json::from_str::<serde_json::Value>(SIGNED_WITHOUT_LANGUAGE).unwrap();
    wire["presets"][0]["format"]["title_block"]["fields"]["language"] =
        serde_json::json!({ "value": "", "visible": true });
    assert!(parse_package(&wire.to_string(), &trust).is_err());
}

#[test]
fn a_package_sealed_again_is_written_with_every_field() {
    let (_, trust, signing_key) = signed_organization_package();
    let published = publish_organization_drawing_sheet_package(
        UNSIGNED_WITHOUT_LANGUAGE,
        &[0x53; 32],
        "rspice-test-publisher",
        "sheet-formats-2026",
        true,
        DrawingSheetPackageEncoding::CanonicalSchema1,
    )
    .unwrap();
    let wire = serde_json::from_str::<serde_json::Value>(&published.encoded).unwrap();
    let fields = wire["presets"][0]["format"]["title_block"]["fields"]
        .as_object()
        .unwrap();
    assert_eq!(fields.len(), DrawingSheetTitleFieldId::ALL.len());
    assert!(fields.contains_key("language"));
    let resealed = parse_package(&published.encoded, &trust).unwrap();
    assert!(resealed.retired_spellings.written_title_fields.is_empty());
    verify_published_drawing_sheet_package(
        &published.encoded,
        &signing_key.verifying_key().to_bytes(),
    )
    .unwrap();
    assert_ne!(
        published.source_digest_sha256,
        "480b6c8b86509c7e2bda48c2c696483bb7150667166b25db5460c3c25f414779"
    );
}
