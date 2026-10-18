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

fn margins(top_um: u64, right_um: u64, bottom_um: u64, left_um: u64) -> serde_json::Value {
    serde_json::json!({
        "top_um": top_um,
        "right_um": right_um,
        "bottom_um": bottom_um,
        "left_um": left_um,
    })
}

#[test]
fn a_sheet_saved_with_its_own_margins_loads_on_its_standards() {
    let iso = DrawingSheetFrameConvention::Iso5457.margins(false);
    for saved in [
        margins(5_000, 5_000, 5_000, 5_000),
        margins(25_000, 25_000, 25_000, 25_000),
        margins(10_000, 10_000, 10_000, 19_050),
    ] {
        let format = saved_with(|wire| wire["margins"] = saved.clone());
        assert_eq!(format.margins(), iso, "{saved}");
        assert!(!format.binding_margin, "an ISO sheet is never bound");
        let geometry = format.geometry().unwrap();
        assert_eq!(
            (geometry.drawing_area.x_um, geometry.drawing_area.y_um),
            (20_000, 10_000)
        );
        let written = serde_json::to_value(&format).unwrap();
        assert!(written.get("margins").is_none());
        assert!(written.get("binding_margin").is_none());
    }
}

#[test]
fn an_asme_sheet_saved_with_the_binding_margin_stays_bound() {
    let ansi_b = || {
        serde_json::to_value(SchematicSheetFormat::from_standard(
            DrawingSheetStandard::AnsiB,
            SchematicPageOrientation::Landscape,
        ))
        .unwrap()
    };
    let load = |mut wire: serde_json::Value, saved: serde_json::Value| {
        wire["margins"] = saved;
        serde_json::from_value::<SchematicSheetFormat>(wire).unwrap()
    };
    // Before ANSI sheets took Y14.1's 0.50 in on every side they were saved
    // with a 0.75 in left margin, which is the binding margin.
    let bound = load(ansi_b(), margins(12_700, 12_700, 12_700, 19_050));
    assert!(bound.binding_margin);
    assert_eq!(bound.margins().left_um, 19_050);
    assert_eq!(bound.margins().right_um, 12_700);
    let written = serde_json::to_value(&bound).unwrap();
    assert_eq!(written["binding_margin"], serde_json::json!(true));
    assert!(written.get("margins").is_none());
    assert_eq!(
        serde_json::from_value::<SchematicSheetFormat>(written).unwrap(),
        bound
    );
    // ARCH sheets were saved with a 1 in left margin, which is no option of
    // the standard's: they load on its 0.50 in.
    let wide = load(ansi_b(), margins(12_700, 12_700, 12_700, 25_400));
    assert!(!wide.binding_margin);
    assert_eq!(
        wide.margins(),
        DrawingSheetFrameConvention::AsmeY141.margins(false)
    );
}

#[test]
fn a_sheet_too_small_for_its_standards_margins_loads_without_a_frame() {
    let mut wire = serde_json::to_value(
        SchematicSheetFormat::try_custom("Tag", 52_000, 52_000, SchematicPageOrientation::Portrait)
            .unwrap(),
    )
    .unwrap();
    // Framed on 5 mm margins, it held a 32 mm drawing area inside a 5 mm
    // band; ISO 5457's 20 and 10 mm borders leave it 22 mm.
    wire["border"] = serde_json::json!("standard");
    wire["margins"] = margins(5_000, 5_000, 5_000, 5_000);
    wire["zones"] = serde_json::json!({
        "mode": "automatic",
        "custom_columns": null,
        "custom_rows": null,
        "labels": "alpha-numeric",
        "edges": "all",
    });
    let format: SchematicSheetFormat = serde_json::from_value(wire).unwrap();
    assert_eq!(format.border, DrawingSheetBorderTemplate::None);
    assert_eq!(format.zones.mode, DrawingSheetZoneMode::None);
    assert_eq!(format.geometry().unwrap().drawing_area.width_um, 52_000);
}

#[test]
fn a_plain_border_loads_as_the_standard_frame_without_zones() {
    let standard = SchematicSheetFormat::default()
        .try_update(|draft| draft.zones = DrawingSheetZones::none())
        .unwrap();
    let plain = saved_with(|wire| {
        wire["border"] = serde_json::json!("plain");
        wire["margins"] = margins(10_000, 10_000, 10_000, 10_000);
        wire["zones"] = serde_json::json!({
            "mode": "automatic",
            "custom_columns": null,
            "custom_rows": null,
            "labels": "alpha-numeric",
            "edges": "all",
        });
    });
    assert_eq!(plain.border, DrawingSheetBorderTemplate::Standard);
    assert_eq!(plain.zones.mode, DrawingSheetZoneMode::None);
    assert_eq!(plain.geometry().unwrap(), standard.geometry().unwrap());
    let written = serde_json::to_value(&plain).unwrap();
    assert_eq!(written["border"], serde_json::json!("standard"));
}

#[test]
fn an_organization_border_loads_as_the_standard_frame() {
    let standard = SchematicSheetFormat::default();
    for mode in ["automatic", "none"] {
        let managed = saved_with(|wire| {
            wire["border"] = serde_json::json!("organization-managed");
            wire["zones"] = serde_json::json!({
                "mode": mode,
                "custom_columns": null,
                "custom_rows": null,
                "labels": "alpha-numeric",
                "edges": "all",
            });
            wire["marks"] = serde_json::json!({ "registration": true, "folding": true });
        });
        assert_eq!(managed.border, DrawingSheetBorderTemplate::Standard);
        // Its zones and marks are kept as it was saved with them.
        assert_eq!(
            managed.zones.mode == DrawingSheetZoneMode::Automatic,
            mode == "automatic"
        );
        assert!(managed.marks.centring);
        assert_eq!(
            managed.geometry().unwrap().drawing_area,
            standard.geometry().unwrap().drawing_area
        );
        let written = serde_json::to_value(&managed).unwrap();
        assert_eq!(written["border"], serde_json::json!("standard"));
    }
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
