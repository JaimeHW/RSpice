//! Packages sealed before every sheet took its series' standard frame still
//! open, and their digests and signatures still verify.

use super::*;

/// An organization package in its human-review form as the build before the
/// standard frames sealed and signed it, byte for byte. One preset is an ANSI
/// frame with a 0.75 in binding margin; one an organization border on margins
/// of its own, with coordinate labels lettered on the bottom and right and
/// registration and folding marks; one a plain border on 10 mm margins with a
/// custom 7 × 5 grid, numbered down the top and left, and folding marks.
const SIGNED_REVIEW_WITH_OLD_FRAMES: &str = r##"{
  "document": "rspice-sheet-formats-human-review",
  "schema": {
    "identity": "rspice-sheet-formats",
    "version": 1,
    "representation": "human-review",
    "exact_dimension_unit": "micrometre"
  },
  "package": {
    "schema": "rspice-sheet-formats",
    "version": 1,
    "source_digest_sha256": "34b0d823a056b9ac290425a11a48df82abecccfbf5c544c60bd1a9c6b2d7414a",
    "includes_builtin_frame_references": true,
    "includes_source_metadata": true,
    "builtin_frame_references": [
      {
        "kind": "border",
        "identity": "plain"
      },
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
      "signature_base64": "zJYwv30tcKEYEpxuNbo7Hy1rGG+9zbF6XAm1WqtUO5gxqZvDXVp9oScSG80pFCU1ka7FyN6WsmFH7tMOh49QDw=="
    },
    "presets": [
      {
        "stable_id": "bound-ansi",
        "name": "Bound ANSI",
        "source_scope": "organization",
        "portrait_width_um": 250000,
        "portrait_height_um": 400000,
        "format": {
          "size": {
            "kind": "custom",
            "name": "Bound ANSI",
            "portrait_width_um": 250000,
            "portrait_height_um": 400000
          },
          "orientation": "portrait",
          "authored_size": {
            "kind": "custom",
            "snapshot": {
              "preset_id": "bound-ansi",
              "name": "Bound ANSI",
              "portrait_width_um": 250000,
              "portrait_height_um": 400000,
              "source_preset_unavailable": false,
              "started_from": "ansi"
            }
          },
          "display_unit": "millimetres",
          "margins": {
            "top_um": 12700,
            "right_um": 12700,
            "bottom_um": 12700,
            "left_um": 19050
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
            "registration": false,
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
              },
              "language": {
                "value": "",
                "visible": false
              }
            }
          },
          "inheritance": "explicit"
        }
      },
      {
        "stable_id": "organization-frame",
        "name": "Organization frame",
        "source_scope": "organization",
        "portrait_width_um": 250000,
        "portrait_height_um": 400000,
        "format": {
          "size": {
            "kind": "custom",
            "name": "Organization frame",
            "portrait_width_um": 250000,
            "portrait_height_um": 400000
          },
          "orientation": "portrait",
          "authored_size": {
            "kind": "custom",
            "snapshot": {
              "preset_id": "organization-frame",
              "name": "Organization frame",
              "portrait_width_um": 250000,
              "portrait_height_um": 400000,
              "source_preset_unavailable": false
            }
          },
          "display_unit": "millimetres",
          "margins": {
            "top_um": 15000,
            "right_um": 12000,
            "bottom_um": 12000,
            "left_um": 25000
          },
          "bleed_um": 0,
          "border": "organization-managed",
          "zones": {
            "mode": "automatic",
            "custom_columns": null,
            "custom_rows": null,
            "labels": "coordinates",
            "edges": "bottom-and-right"
          },
          "marks": {
            "registration": true,
            "folding": true
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
              },
              "language": {
                "value": "",
                "visible": false
              }
            }
          },
          "inheritance": "explicit"
        }
      },
      {
        "stable_id": "plain-custom-zones",
        "name": "Plain custom zones",
        "source_scope": "organization",
        "portrait_width_um": 250000,
        "portrait_height_um": 400000,
        "format": {
          "size": {
            "kind": "custom",
            "name": "Plain custom zones",
            "portrait_width_um": 250000,
            "portrait_height_um": 400000
          },
          "orientation": "portrait",
          "authored_size": {
            "kind": "custom",
            "snapshot": {
              "preset_id": "plain-custom-zones",
              "name": "Plain custom zones",
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
            "left_um": 10000
          },
          "bleed_um": 0,
          "border": "plain",
          "zones": {
            "mode": "custom",
            "custom_columns": 7,
            "custom_rows": 5,
            "labels": "numeric-alpha",
            "edges": "top-and-left"
          },
          "marks": {
            "registration": false,
            "folding": true
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
              },
              "language": {
                "value": "",
                "visible": false
              }
            }
          },
          "inheritance": "explicit"
        }
      }
    ]
  },
  "review": [
    {
      "stable_id": "bound-ansi",
      "name": "Bound ANSI",
      "source_scope": "organization",
      "portrait_dimensions": "250 × 400 mm",
      "portrait_width_um": 250000,
      "portrait_height_um": 400000,
      "dependencies": [
        "border:builtin/standard",
        "title-block:builtin/standard"
      ]
    },
    {
      "stable_id": "organization-frame",
      "name": "Organization frame",
      "source_scope": "organization",
      "portrait_dimensions": "250 × 400 mm",
      "portrait_width_um": 250000,
      "portrait_height_um": 400000,
      "dependencies": [
        "border:organization-managed",
        "title-block:builtin/standard"
      ]
    },
    {
      "stable_id": "plain-custom-zones",
      "name": "Plain custom zones",
      "source_scope": "organization",
      "portrait_dimensions": "250 × 400 mm",
      "portrait_width_um": 250000,
      "portrait_height_um": 400000,
      "dependencies": [
        "border:builtin/plain",
        "title-block:builtin/standard"
      ]
    }
  ]
}"##;

/// The digest the build before the standard frames sealed the package with.
const WRITTEN_DIGEST: &str = "34b0d823a056b9ac290425a11a48df82abecccfbf5c544c60bd1a9c6b2d7414a";

/// The review document as a value to change.
fn review_document() -> serde_json::Value {
    serde_json::from_str(SIGNED_REVIEW_WITH_OLD_FRAMES).unwrap()
}

/// The same package in its canonical form, which holds the package alone.
fn canonical_form() -> String {
    serde_json::to_string_pretty(&review_document()["package"]).unwrap()
}

#[test]
fn a_package_sealed_over_the_frames_before_the_standards_opens_and_verifies() {
    let (_, trust, signing_key) = signed_organization_package();
    let verifying_key = signing_key.verifying_key().to_bytes();
    for source in [SIGNED_REVIEW_WITH_OLD_FRAMES.to_owned(), canonical_form()] {
        let package = parse_package(&source, &trust).unwrap();
        assert_eq!(package.presets.len(), 3);
        let verification = verify_published_drawing_sheet_package(&source, &verifying_key).unwrap();
        assert_eq!(verification.source_digest_sha256, WRITTEN_DIGEST);
        assert_eq!(
            inspect_drawing_sheet_package(&source)
                .unwrap()
                .source_digest_sha256,
            WRITTEN_DIGEST
        );
    }
}

#[test]
fn tampering_with_a_frame_as_it_was_written_is_still_caught() {
    let (_, trust, _) = signed_organization_package();
    let tampered: [(usize, &str, serde_json::Value); 6] = [
        (0, "/margins/left_um", serde_json::json!(12_700)),
        (1, "/zones/labels", serde_json::json!("alpha-numeric")),
        (1, "/marks/folding", serde_json::json!(false)),
        (2, "/zones/custom_columns", serde_json::json!(8)),
        (2, "/zones/edges", serde_json::json!("all")),
        (2, "/border", serde_json::json!("standard")),
    ];
    for (preset, pointer, value) in tampered {
        let mut document = review_document();
        *document["package"]["presets"][preset]["format"]
            .pointer_mut(pointer)
            .unwrap() = value;
        assert!(
            parse_package(&document.to_string(), &trust).is_err(),
            "{pointer} of preset {preset}"
        );
        let package = serde_json::to_string(&document["package"]).unwrap();
        assert!(
            parse_package(&package, &trust).is_err(),
            "{pointer} of preset {preset}"
        );
    }
}
