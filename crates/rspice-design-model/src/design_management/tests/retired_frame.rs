//! Sheets, defaults and presets saved with frame options the drawing-format
//! standards do not have still load, onto the standard frame, and are written
//! back without them.

use super::*;

/// A sheet as an earlier RSpice wrote it: today's default sheet with `edit`
/// applied to its wire form.
fn saved_with(edit: impl FnOnce(&mut serde_json::Value)) -> SchematicSheetFormat {
    let mut wire = serde_json::to_value(SchematicSheetFormat::default()).unwrap();
    edit(&mut wire);
    serde_json::from_value(wire).expect("a sheet saved with a retired option loads")
}

#[test]
fn custom_counts_label_schemes_and_edges_load_as_the_standards_zones() {
    let standard = SchematicSheetFormat::default().geometry().unwrap();
    for (mode, columns, rows, labels, edges) in [
        ("custom", "7", "5", "numeric-alpha", "top-and-left"),
        ("custom", "24", "2", "coordinates", "bottom-and-right"),
        ("automatic", "null", "null", "coordinates", "top-and-right"),
        ("automatic", "null", "null", "numeric-alpha", "all"),
    ] {
        let format = saved_with(|wire| {
            wire["zones"] = serde_json::from_str(&format!(
                r#"{{"mode":"{mode}","custom_columns":{columns},"custom_rows":{rows},"labels":"{labels}","edges":"{edges}"}}"#
            ))
            .unwrap();
        });
        assert_eq!(format.zones.mode, DrawingSheetZoneMode::Automatic);
        let geometry = format.geometry().unwrap();
        assert_eq!(geometry.zones, standard.zones, "{mode} {labels} {edges}");
        let zones = format.zone_layout(&geometry).unwrap();
        // ISO 5457: rows lettered from the top, columns numbered from the
        // left, a reference quoted for every point in the frame.
        assert_eq!(zones.rows[0].label, "A");
        assert_eq!(zones.columns[0].label, "1");
        assert_eq!(zones.reference_at(30_000, 20_000).as_deref(), Some("A1"));
        assert_eq!(
            serde_json::to_value(&format).unwrap()["zones"],
            serde_json::json!({ "mode": "automatic" }),
            "no count, label scheme or edge is written back"
        );
    }
    let without = saved_with(|wire| {
        wire["zones"] = serde_json::json!({
            "mode": "none",
            "custom_columns": null,
            "custom_rows": null,
            "labels": "alpha-numeric",
            "edges": "all",
        });
    });
    assert_eq!(without.zones.mode, DrawingSheetZoneMode::None);
    assert_eq!(without.geometry().unwrap().zones, None);
}

#[test]
fn registration_and_folding_flags_load_as_the_centring_marks_option() {
    for (registration, folding) in [(true, true), (true, false), (false, true), (false, false)] {
        let format = saved_with(|wire| {
            wire["marks"] = serde_json::json!({
                "registration": registration,
                "folding": folding,
            });
        });
        assert_eq!(format.marks.centring, registration);
        let written = serde_json::to_value(&format).unwrap();
        assert_eq!(
            written["marks"],
            serde_json::json!({ "centring": registration }),
            "neither retired flag is written back"
        );
    }
}
