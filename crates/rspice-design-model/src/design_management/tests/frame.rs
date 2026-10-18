//! A sheet's frame follows the drawing-format standard of its series: ISO
//! 5457 for ISO A and JIS B sheets, the ASME Y14.1 format for ANSI and ARCH.

use super::*;

const MM: i64 = 1_000;

fn sheet(
    standard: DrawingSheetStandard,
    orientation: SchematicPageOrientation,
) -> SchematicSheetFormat {
    SchematicSheetFormat::from_standard(standard, orientation)
}

fn rect(x_mm: f64, y_mm: f64, width_mm: f64, height_mm: f64) -> DrawingSheetRect {
    let um = |mm: f64| (mm * 1_000.0).round() as i64;
    DrawingSheetRect {
        x_um: um(x_mm),
        y_um: um(y_mm),
        width_um: um(width_mm) as u64,
        height_um: um(height_mm) as u64,
    }
}

fn layout(format: &SchematicSheetFormat) -> DrawingSheetZoneLayout {
    format
        .zone_layout(&format.geometry().unwrap())
        .expect("the sheet has zones")
}

/// A point in sheet micrometres.
type At = (i64, i64);

/// A frame line: where it starts and ends, and its width.
type FrameLine = (At, At, u64);

fn lines(artwork: &DrawingSheetFrameArtwork, layer: DrawingSheetFrameLayer) -> Vec<FrameLine> {
    artwork
        .layer(layer)
        .filter_map(|mark| match mark {
            DrawingSheetFrameMark::Line { from, to, width_um } => Some((*from, *to, *width_um)),
            _ => None,
        })
        .collect()
}

fn texts(
    artwork: &DrawingSheetFrameArtwork,
    layer: DrawingSheetFrameLayer,
) -> Vec<((i64, i64), String, u64)> {
    artwork
        .layer(layer)
        .filter_map(|mark| match mark {
            DrawingSheetFrameMark::Text {
                centre,
                text,
                lettering_um,
            } => Some((*centre, text.clone(), *lettering_um)),
            _ => None,
        })
        .collect()
}

#[test]
fn an_iso_frame_bounds_the_drawing_space_iso_5457_table_1_gives() {
    // IS 10711 Table 1: the drawing space inside the frame, A4 upright and
    // A3 to A0 in the landscape orientation 4.1 uses them in.
    for (standard, orientation, space_mm) in [
        (
            DrawingSheetStandard::IsoA4,
            SchematicPageOrientation::Portrait,
            (180.0, 277.0),
        ),
        (
            DrawingSheetStandard::IsoA3,
            SchematicPageOrientation::Landscape,
            (390.0, 277.0),
        ),
        (
            DrawingSheetStandard::IsoA2,
            SchematicPageOrientation::Landscape,
            (564.0, 400.0),
        ),
        (
            DrawingSheetStandard::IsoA1,
            SchematicPageOrientation::Landscape,
            (811.0, 574.0),
        ),
        (
            DrawingSheetStandard::IsoA0,
            SchematicPageOrientation::Landscape,
            (1_159.0, 821.0),
        ),
    ] {
        let geometry = sheet(standard, orientation).geometry().unwrap();
        assert_eq!(
            geometry.frame_placement,
            DrawingSheetFramePlacement::Iso5457
        );
        // The frame sits at the 20 mm filing border and 10 mm elsewhere.
        assert_eq!(geometry.drawing_area.x_um, 20 * MM);
        assert_eq!(geometry.drawing_area.y_um, 10 * MM);
        assert_eq!(
            (
                geometry.drawing_area.width_um,
                geometry.drawing_area.height_um
            ),
            ((space_mm.0 * 1_000.0) as u64, (space_mm.1 * 1_000.0) as u64),
            "{standard:?}"
        );
    }
    let a4 = sheet(
        DrawingSheetStandard::IsoA4,
        SchematicPageOrientation::Portrait,
    );
    let geometry = a4.geometry().unwrap();
    assert_eq!(geometry.drawing_area, rect(20.0, 10.0, 180.0, 277.0));
    // The grid reference border closes the 5 mm band outside the frame, 5 mm
    // from the trimmed edge and 15 mm on the filing side.
    assert_eq!(geometry.zone_band, rect(15.0, 5.0, 190.0, 287.0));
    assert_eq!(geometry.border_band_um, 5_000);
    assert_eq!(geometry.content_area, geometry.drawing_area);
}

#[test]
fn margins_narrower_than_the_band_keep_the_frame_inset() {
    let tight = sheet(
        DrawingSheetStandard::IsoA4,
        SchematicPageOrientation::Landscape,
    )
    .try_update(|draft| {
        draft.margins = DrawingSheetMargins {
            top_um: 4_000,
            right_um: 10_000,
            bottom_um: 10_000,
            left_um: 20_000,
        };
    })
    .unwrap();
    let geometry = tight.geometry().unwrap();
    assert_eq!(geometry.frame_placement, DrawingSheetFramePlacement::Inset);
    assert_eq!(geometry.zone_band, geometry.printable);
    assert_eq!(geometry.drawing_area, rect(25.0, 9.0, 257.0, 186.0));

    let plain = SchematicSheetFormat::default()
        .try_update(|draft| draft.apply_border_template(DrawingSheetBorderTemplate::Plain))
        .unwrap();
    assert_eq!(
        plain.geometry().unwrap().frame_placement,
        DrawingSheetFramePlacement::Inset
    );
}

#[test]
fn iso_zone_counts_are_iso_5457_table_2() {
    // IS 10711 Table 2, fields along the long side × the short side.
    for (standard, long, short) in [
        (DrawingSheetStandard::IsoA0, 24, 16),
        (DrawingSheetStandard::IsoA1, 16, 12),
        (DrawingSheetStandard::IsoA2, 12, 8),
        (DrawingSheetStandard::IsoA3, 8, 6),
        (DrawingSheetStandard::IsoA4, 6, 4),
    ] {
        let landscape = sheet(standard, SchematicPageOrientation::Landscape);
        let zones = landscape.geometry().unwrap().zones.unwrap();
        assert_eq!((zones.columns, zones.rows), (long, short), "{standard:?}");
        let portrait = sheet(standard, SchematicPageOrientation::Portrait);
        let zones = portrait.geometry().unwrap().zones.unwrap();
        assert_eq!((zones.columns, zones.rows), (short, long), "{standard:?}");
        // The 50 mm-from-centre rule every other ISO 5457 sheet is zoned by
        // is the rule the table follows.
        let (width_um, height_um) = standard.portrait_dimensions_um();
        assert_eq!(drawing_sheet_iso_5457_field_count(width_um), short);
        assert_eq!(drawing_sheet_iso_5457_field_count(height_um), long);
    }
}

#[test]
fn iso_fields_are_50_mm_from_the_axes_of_symmetry() {
    let a3 = layout(&sheet(
        DrawingSheetStandard::IsoA3,
        SchematicPageOrientation::Landscape,
    ));
    assert_eq!(a3.columns.len(), 8);
    assert_eq!(a3.rows.len(), 6);
    assert_eq!(
        a3.column_boundaries_um().collect::<Vec<_>>(),
        [60, 110, 160, 210, 260, 310, 360].map(|mm| mm * MM)
    );
    assert_eq!(
        a3.row_boundaries_um().collect::<Vec<_>>(),
        [48_500, 98_500, 148_500, 198_500, 248_500]
    );
    // The corner fields take up the difference, to the frame.
    assert_eq!(
        (a3.columns[0].start_um, a3.columns[0].end_um),
        (20 * MM, 60 * MM)
    );
    assert_eq!(
        (a3.columns[7].start_um, a3.columns[7].end_um),
        (360 * MM, 410 * MM)
    );
    // Rows are lettered from the top, columns numbered from the left.
    assert_eq!(a3.columns[0].label, "1");
    assert_eq!(a3.columns[7].label, "8");
    assert_eq!(a3.rows[0].label, "A");
    assert_eq!(a3.rows[5].label, "F");
    assert_eq!(a3.reference_at(30 * MM, 20 * MM).as_deref(), Some("A1"));
    assert_eq!(a3.reference_at(400 * MM, 280 * MM).as_deref(), Some("F8"));
    assert_eq!(a3.reference_at(10 * MM, 20 * MM), None, "outside the frame");

    // A4 portrait: columns 35/50/50/45 mm and rows 38.5/50/50/50/50/38.5 mm
    // inside the frame.
    let a4 = layout(&sheet(
        DrawingSheetStandard::IsoA4,
        SchematicPageOrientation::Portrait,
    ));
    let widths = |fields: &[DrawingSheetZoneField]| {
        fields
            .iter()
            .map(|field| field.end_um - field.start_um)
            .collect::<Vec<_>>()
    };
    assert_eq!(widths(&a4.columns), [35_000, 50_000, 50_000, 45_000]);
    assert_eq!(
        widths(&a4.rows),
        [38_500, 50_000, 50_000, 50_000, 50_000, 38_500]
    );
}

#[test]
fn a4_letters_its_grid_references_at_the_top_and_right_only() {
    for orientation in [
        SchematicPageOrientation::Portrait,
        SchematicPageOrientation::Landscape,
    ] {
        let a4 = sheet(DrawingSheetStandard::IsoA4, orientation);
        assert_eq!(a4.zones.edges, DrawingSheetZoneEdges::TopAndRight);
        let geometry = a4.geometry().unwrap();
        let zones = geometry.zones.unwrap();
        let labels = texts(
            &a4.frame_artwork(&geometry),
            DrawingSheetFrameLayer::ZoneLabels,
        );
        assert_eq!(labels.len(), usize::from(zones.columns + zones.rows));
        let area = geometry.drawing_area;
        let right = area.x_um + area.width_um as i64;
        for ((x, y), text, lettering_um) in labels {
            assert_eq!(
                lettering_um, 3_500,
                "ISO 5457 4.4 letters references 3.5 mm"
            );
            assert!(
                y < area.y_um || x > right,
                "{text} is lettered at ({x}, {y}), off the top and right bands"
            );
        }
        // Every other ISO size letters all four sides.
        let a3 = sheet(DrawingSheetStandard::IsoA3, orientation);
        assert_eq!(a3.zones.edges, DrawingSheetZoneEdges::All);
    }
}

#[test]
fn zone_letters_skip_only_i_and_o_and_double_past_z() {
    let letters = (0..24).map(drawing_sheet_zone_letters).collect::<String>();
    assert_eq!(letters, "ABCDEFGHJKLMNPQRSTUVWXYZ");
    assert_eq!(drawing_sheet_zone_letters(24), "AA");
    assert_eq!(drawing_sheet_zone_letters(25), "AB");
    assert_eq!(drawing_sheet_zone_letters(48), "BA");
    // A0 landscape's sixteen rows end P, Q, R.
    let a0 = layout(&sheet(
        DrawingSheetStandard::IsoA0,
        SchematicPageOrientation::Landscape,
    ));
    let last = a0
        .rows
        .iter()
        .rev()
        .take(3)
        .map(|row| row.label.as_str())
        .collect::<Vec<_>>();
    assert_eq!(last, ["R", "Q", "P"]);
}

#[test]
fn ansi_d_is_zoned_by_asme_y14_1_table_4_1_from_the_lower_right() {
    let ansi_d = sheet(
        DrawingSheetStandard::AnsiD,
        SchematicPageOrientation::Landscape,
    );
    assert_eq!(
        ansi_d.frame_convention(),
        DrawingSheetFrameConvention::AsmeY141
    );
    let geometry = ansi_d.geometry().unwrap();
    assert_eq!(
        geometry.frame_placement,
        DrawingSheetFramePlacement::AsmeY141
    );
    // The border line 0.50 in from the trim on every side, the whole margin
    // the zone band.
    assert_eq!(geometry.drawing_area, rect(12.7, 12.7, 838.2, 533.4));
    assert_eq!(geometry.zone_band, geometry.paper);
    assert_eq!(geometry.border_band_um, 12_700);

    let zones = layout(&ansi_d);
    assert_eq!((zones.columns.len(), zones.rows.len()), (8, 4));
    // Boundaries fall on whole zones from the trimmed edge: 4.25 in across,
    // 5.50 in down, so the first and last zones inside the border are 0.50 in
    // short.
    assert_eq!(
        zones.column_boundaries_um().collect::<Vec<_>>(),
        (1..8).map(|k| k * 107_950).collect::<Vec<_>>()
    );
    assert_eq!(
        zones.row_boundaries_um().collect::<Vec<_>>(),
        [139_700, 279_400, 419_100]
    );
    assert_eq!(
        zones.columns[7].end_um - zones.columns[7].start_um,
        107_950 - 12_700
    );
    // Numbered from the right, lettered from the bottom.
    let column_labels = zones
        .columns
        .iter()
        .map(|field| field.label.as_str())
        .collect::<Vec<_>>();
    assert_eq!(column_labels, ["8", "7", "6", "5", "4", "3", "2", "1"]);
    let row_labels = zones
        .rows
        .iter()
        .map(|field| field.label.as_str())
        .collect::<Vec<_>>();
    assert_eq!(row_labels, ["D", "C", "B", "A"]);
    // The title block's corner is zone A1.
    assert_eq!(zones.reference_at(850_000, 545_000).as_deref(), Some("A1"));
    // Zone references are lettered 6 mm high in the margin.
    let labels = texts(
        &ansi_d.frame_artwork(&geometry),
        DrawingSheetFrameLayer::ZoneLabels,
    );
    assert_eq!(labels.len(), 2 * (8 + 4));
    assert!(
        labels
            .iter()
            .all(|(_, _, lettering_um)| *lettering_um == 6_000)
    );
}

#[test]
fn every_ansi_size_takes_its_table_4_1_zone_count() {
    for (standard, landscape) in [
        (DrawingSheetStandard::AnsiA, (2, 2)),
        (DrawingSheetStandard::AnsiB, (4, 2)),
        (DrawingSheetStandard::AnsiC, (4, 4)),
        (DrawingSheetStandard::AnsiD, (8, 4)),
        (DrawingSheetStandard::AnsiE, (8, 8)),
    ] {
        let zones = sheet(standard, SchematicPageOrientation::Landscape)
            .geometry()
            .unwrap()
            .zones
            .unwrap();
        assert_eq!((zones.columns, zones.rows), landscape, "{standard:?}");
        let portrait = sheet(standard, SchematicPageOrientation::Portrait)
            .geometry()
            .unwrap()
            .zones
            .unwrap();
        assert_eq!(
            (portrait.columns, portrait.rows),
            (landscape.1, landscape.0)
        );
    }
    assert_eq!(
        DrawingSheetStandard::AnsiA.default_margins(),
        DrawingSheetMargins {
            top_um: 12_700,
            right_um: 12_700,
            bottom_um: 12_700,
            left_um: 12_700,
        },
        "a binding margin is an option, not the default"
    );
}

#[test]
fn arch_sheets_are_zoned_in_equal_zones_of_one_to_five_and_a_half_inches() {
    for standard in DrawingSheetStandard::ALL
        .into_iter()
        .filter(|standard| standard.series() == DrawingSheetStandardSeries::Architectural)
    {
        let format = sheet(standard, SchematicPageOrientation::Landscape);
        assert_eq!(
            format.frame_convention(),
            DrawingSheetFrameConvention::AsmeY141
        );
        let (width_um, height_um) = format.oriented_dimensions_um();
        let zones = format.geometry().unwrap().zones.unwrap();
        assert!(drawing_sheet_asme_zone_count_allowed(
            width_um,
            zones.columns
        ));
        assert!(drawing_sheet_asme_zone_count_allowed(height_um, zones.rows));
    }
    // ARCH D's 36 in edge takes ANSI D's eight zones; its 24 in edge, too
    // long for ANSI C's four, takes five.
    let arch_d = sheet(
        DrawingSheetStandard::ArchD,
        SchematicPageOrientation::Landscape,
    );
    let zones = arch_d.geometry().unwrap().zones.unwrap();
    assert_eq!((zones.columns, zones.rows), (8, 5));

    assert_eq!(
        DrawingSheetStandard::ArchE1.portrait_dimensions_um(),
        (762_000, 1_066_800)
    );
    assert_eq!(DrawingSheetStandard::ArchE1.label(), "ARCH E1");
    assert_eq!(
        serde_json::to_value(DrawingSheetStandard::ArchE1).unwrap(),
        serde_json::json!("arch-e1")
    );
}

#[test]
fn asme_zone_counts_must_keep_zones_between_one_and_five_and_a_half_inches() {
    assert!(drawing_sheet_asme_zone_count_allowed(279_400, 2));
    assert!(
        !drawing_sheet_asme_zone_count_allowed(279_400, 24),
        "0.46 in zones"
    );
    assert!(
        !drawing_sheet_asme_zone_count_allowed(1_117_600, 2),
        "22 in zones"
    );
    assert!(drawing_sheet_asme_zone_count_allowed(1_117_600, 8));
    // A sheet saved with such a count still loads and draws.
    let crowded = sheet(
        DrawingSheetStandard::AnsiA,
        SchematicPageOrientation::Landscape,
    )
    .try_update(|draft| {
        draft.zones.mode = DrawingSheetZoneMode::Custom;
        draft.zones.custom_columns = Some(24);
        draft.zones.custom_rows = Some(2);
    })
    .unwrap();
    assert_eq!(crowded.geometry().unwrap().zones.unwrap().columns, 24);
    // Its 0.46 in zones count from the trimmed edge, so the first and last
    // lie wholly in the 0.50 in margin and the frame shows zones 2 to 23.
    let columns = layout(&crowded).columns;
    assert_eq!(columns.len(), 22);
    assert_eq!(columns.first().unwrap().label, "23");
    assert_eq!(columns.last().unwrap().label, "2");
}

#[test]
fn frame_lines_are_drawn_at_the_widths_the_standards_give() {
    let a4 = sheet(
        DrawingSheetStandard::IsoA4,
        SchematicPageOrientation::Landscape,
    );
    let artwork = a4.frame_artwork(&a4.geometry().unwrap());
    let rects = artwork
        .layer(DrawingSheetFrameLayer::Border)
        .filter_map(|mark| match mark {
            DrawingSheetFrameMark::Rect { width_um, .. } => Some(*width_um),
            _ => None,
        })
        .collect::<Vec<_>>();
    // The frame 0.7 mm; the grid reference border 0.35 mm.
    assert_eq!(rects, [700, 350]);
    let ticks = lines(&artwork, DrawingSheetFrameLayer::Zones);
    assert!(!ticks.is_empty());
    assert!(ticks.iter().all(|(_, _, width)| *width == 350));

    let ansi_d = sheet(
        DrawingSheetStandard::AnsiD,
        SchematicPageOrientation::Landscape,
    );
    let artwork = ansi_d.frame_artwork(&ansi_d.geometry().unwrap());
    let rects = artwork
        .layer(DrawingSheetFrameLayer::Border)
        .filter_map(|mark| match mark {
            DrawingSheetFrameMark::Rect { width_um, .. } => Some(*width_um),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(rects, [600], "one 0.6 mm border line, no second frame");
    let ticks = lines(&artwork, DrawingSheetFrameLayer::Zones);
    assert!(ticks.iter().all(|(_, _, width)| *width == 300));
}

#[test]
fn an_iso_frame_draws_centring_marks_instead_of_corner_crosses() {
    let a3 = sheet(
        DrawingSheetStandard::IsoA3,
        SchematicPageOrientation::Landscape,
    );
    assert!(a3.marks.centring);
    let geometry = a3.geometry().unwrap();
    let artwork = a3.frame_artwork(&geometry);
    let marks = lines(&artwork, DrawingSheetFrameLayer::CentringMarks);
    // Four 0.7 mm marks on the axes of symmetry, from the grid reference
    // border to 10 mm beyond the frame.
    assert_eq!(
        marks,
        [
            ((210 * MM, 5 * MM), (210 * MM, 20 * MM), 700),
            ((210 * MM, 292 * MM), (210 * MM, 277 * MM), 700),
            ((15 * MM, 148_500), (30 * MM, 148_500), 700),
            ((415 * MM, 148_500), (400 * MM, 148_500), 700),
        ]
    );
    // On A4 portrait the title block spans the foot of the frame, and the
    // bottom mark stops at the frame rather than rule through its cells.
    let a4 = sheet(
        DrawingSheetStandard::IsoA4,
        SchematicPageOrientation::Portrait,
    );
    let marks = lines(
        &a4.frame_artwork(&a4.geometry().unwrap()),
        DrawingSheetFrameLayer::CentringMarks,
    );
    assert_eq!(marks[1], ((105 * MM, 292 * MM), (105 * MM, 287 * MM), 700));
    assert_eq!(marks[0], ((105 * MM, 5 * MM), (105 * MM, 20 * MM), 700));

    // On an ASME format the microfilm arrows are an option a new sheet leaves
    // off.
    let ansi_d = sheet(
        DrawingSheetStandard::AnsiD,
        SchematicPageOrientation::Landscape,
    );
    let artwork = ansi_d.frame_artwork(&ansi_d.geometry().unwrap());
    assert_eq!(
        artwork.layer(DrawingSheetFrameLayer::CentringMarks).count(),
        0
    );
    // Asked for, they are Fig 5-5 arrows centred in each margin, their points
    // on the border line.
    let arrowed = ansi_d
        .try_update(|draft| draft.marks.centring = true)
        .unwrap();
    let geometry = arrowed.geometry().unwrap();
    let heads = arrowed
        .frame_artwork(&geometry)
        .layer(DrawingSheetFrameLayer::CentringMarks)
        .filter_map(|mark| match mark {
            DrawingSheetFrameMark::FilledPolygon { points } => Some(points[0]),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        heads,
        [
            (431_800, 12_700),
            (431_800, 546_100),
            (12_700, 279_400),
            (850_900, 279_400),
        ]
    );
}

#[test]
fn the_size_designation_is_lettered_in_the_bottom_border_at_the_right_corner() {
    let a4 = sheet(
        DrawingSheetStandard::IsoA4,
        SchematicPageOrientation::Portrait,
    );
    let geometry = a4.geometry().unwrap();
    let designation = texts(
        &a4.frame_artwork(&geometry),
        DrawingSheetFrameLayer::SizeDesignation,
    );
    let [((x, y), text, lettering_um)] = designation.as_slice() else {
        panic!("one size designation, found {designation:?}");
    };
    assert_eq!(text, "A4");
    assert_eq!(*lettering_um, 3_500);
    // Below the grid reference border, right of the sheet's centre, inside
    // the frame's width and clear of the trimming mark in the corner.
    assert!(*y > 292 * MM && *y < 297 * MM);
    assert!(*x > 105 * MM && *x < 200 * MM - 3_000);
    // ASME formats letter the size in the title block instead.
    let ansi_d = sheet(
        DrawingSheetStandard::AnsiD,
        SchematicPageOrientation::Landscape,
    );
    let artwork = ansi_d.frame_artwork(&ansi_d.geometry().unwrap());
    assert_eq!(
        artwork
            .layer(DrawingSheetFrameLayer::SizeDesignation)
            .count(),
        0
    );
}

#[test]
fn iso_trimming_marks_are_two_overlapping_ten_by_five_rectangles() {
    let a4 = sheet(
        DrawingSheetStandard::IsoA4,
        SchematicPageOrientation::Portrait,
    );
    let artwork = a4.frame_artwork(&a4.geometry().unwrap());
    let rects = artwork
        .layer(DrawingSheetFrameLayer::TrimMarks)
        .map(|mark| match mark {
            DrawingSheetFrameMark::FilledRect { rect } => *rect,
            other => panic!("a trimming mark is filled, not {other:?}"),
        })
        .collect::<Vec<_>>();
    assert_eq!(rects.len(), 8);
    assert!(rects.contains(&rect(0.0, 0.0, 10.0, 5.0)));
    assert!(rects.contains(&rect(0.0, 0.0, 5.0, 10.0)));
    assert!(rects.contains(&rect(200.0, 292.0, 10.0, 5.0)));
    assert!(rects.contains(&rect(205.0, 287.0, 5.0, 10.0)));
}

#[test]
fn a_custom_size_keeps_the_frame_of_the_series_it_was_started_from() {
    let iso = crate::custom_format("Panel", 300_000, 500_000, crate::StartingFrame::IsoA).unwrap();
    assert_eq!(iso.frame_convention(), DrawingSheetFrameConvention::Iso5457);
    let ansi =
        crate::custom_format("Panel", 300_000, 500_000, crate::StartingFrame::AnsiA).unwrap();
    assert_eq!(
        ansi.frame_convention(),
        DrawingSheetFrameConvention::AsmeY141
    );
    assert_eq!(
        ansi.geometry().unwrap().frame_placement,
        DrawingSheetFramePlacement::AsmeY141
    );
    // A size started from no series is framed to ISO 5457, and one with no
    // record writes none, so a sheet saved before the record existed reads
    // back byte for byte.
    let plain = SchematicSheetFormat::try_custom(
        "Panel",
        300_000,
        500_000,
        SchematicPageOrientation::Portrait,
    )
    .unwrap();
    assert_eq!(
        plain.frame_convention(),
        DrawingSheetFrameConvention::Iso5457
    );
    let wire = serde_json::to_value(&plain).unwrap();
    assert!(
        wire["authored_size"]["snapshot"]
            .get("started_from")
            .is_none()
    );
    let wire = serde_json::to_value(&ansi).unwrap();
    assert_eq!(
        wire["authored_size"]["snapshot"]["started_from"],
        serde_json::json!("ansi")
    );
    let restored: SchematicSheetFormat = serde_json::from_value(wire).unwrap();
    assert_eq!(restored, ansi);
}
