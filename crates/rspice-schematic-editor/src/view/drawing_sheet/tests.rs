use super::*;
use rspice_design::schematic::{
    component::Component,
    component_type::ComponentType,
    design_note::{DesignNote, DesignNoteKind},
    document::SchematicDocument,
    documentation_shape::{DocumentationShape, DocumentationShapeGeometry},
    net_label::{Junction, NetLabel},
    wire::Wire,
};
use rspice_design_model::{SchematicPageOrientation, SchematicPageSize};

fn test_sheet() -> ActiveDrawingSheet {
    let format = SchematicSheetFormat::default();
    ActiveDrawingSheet {
        geometry: DrawingSheetGeometry::from_format(&format),
        format,
        sheet_name: "top".to_owned(),
        page_label: "1 of 1".to_owned(),
    }
}
fn test_view(document: &SchematicDocument) -> DesignView<'_> {
    DesignView {
        document,
        canvas_cache: None,
        sheet_catalog: None,
        review_markers: Default::default(),
    }
}

#[test]
fn a4_landscape_geometry_uses_the_mockup_origin_and_exact_calibration() {
    let geometry = DrawingSheetGeometry::from_format(&SchematicSheetFormat::standard(
        SchematicPageSize::A4,
        SchematicPageOrientation::Landscape,
    ));

    assert_eq!(
        geometry.physical.paper,
        DrawingSheetRect {
            x_um: 0,
            y_um: 0,
            width_um: 297_000,
            height_um: 210_000,
        }
    );
    assert_eq!(
        geometry.physical.printable,
        DrawingSheetRect {
            x_um: 20_000,
            y_um: 10_000,
            width_um: 267_000,
            height_um: 190_000,
        }
    );
    assert_eq!(
        geometry.physical.drawing_area,
        DrawingSheetRect {
            x_um: 25_000,
            y_um: 15_000,
            width_um: 257_000,
            height_um: 180_000,
        }
    );
    assert_eq!(geometry.paper.min_x, -140.0);
    assert_eq!(geometry.paper.min_y, -40.0);
    assert_eq!(geometry.paper.max_x, 1_048.0);
    assert_eq!(geometry.paper.max_y, 800.0);
    assert_eq!(
        geometry.physical.zones,
        Some(DrawingSheetZoneGrid {
            columns: 4,
            rows: 4,
            labels: DrawingSheetZoneLabels::AlphaNumeric,
            edges: DrawingSheetZoneEdges::All,
        })
    );
}

#[test]
fn letter_dimensions_retain_fractional_world_edges_without_rounding() {
    let geometry = DrawingSheetGeometry::from_format(&SchematicSheetFormat::standard(
        SchematicPageSize::UsLetter,
        SchematicPageOrientation::Portrait,
    ));

    assert_eq!(geometry.physical.paper.width_um, 215_900);
    assert!((geometry.paper.max_x - 723.6).abs() < f64::EPSILON);
    assert!((geometry.paper.max_y - 1_077.6).abs() < f64::EPSILON);
}

#[test]
fn level_of_detail_is_driven_by_physical_pixel_density() {
    assert_eq!(
        DrawingSheetLod::resolve(0.1, 1.0, test_lettering()).tier,
        DrawingSheetLodTier::Thumbnail
    );
    assert_eq!(
        DrawingSheetLod::resolve(0.25, 1.0, test_lettering()).tier,
        DrawingSheetLodTier::Outline
    );
    assert_eq!(
        DrawingSheetLod::resolve(0.5, 1.0, test_lettering()).tier,
        DrawingSheetLodTier::Working
    );
    assert_eq!(
        DrawingSheetLod::resolve(1.5, 1.0, test_lettering()).tier,
        DrawingSheetLodTier::Detail
    );
}

#[test]
fn zone_reference_and_sheet_coordinates_share_one_origin() {
    let geometry = DrawingSheetGeometry::from_format(&SchematicSheetFormat::default());
    assert_eq!(geometry.world_to_sheet_mm(-140.0, -40.0), (0.0, 0.0));
    assert_eq!(
        geometry.zone_reference(
            geometry.drawing_area.min_x + 1.0,
            geometry.drawing_area.min_y + 1.0
        ),
        Some("A1".to_owned())
    );
}

#[test]
fn coordinates_only_zones_retain_bands_but_suppress_labels_and_references() {
    let format = SchematicSheetFormat::default()
        .try_update(|draft| draft.zones.labels = DrawingSheetZoneLabels::Coordinates)
        .expect("coordinates-only zones are valid");
    let sheet = ActiveDrawingSheet {
        geometry: DrawingSheetGeometry::from_format(&format),
        format,
        sheet_name: "top".to_owned(),
        page_label: "1 of 1".to_owned(),
    };
    let point = (
        sheet.geometry.drawing_area.min_x + 1.0,
        sheet.geometry.drawing_area.min_y + 1.0,
    );

    assert!(!zone_edge_labels_visible(
        DrawingSheetZoneLabels::Coordinates
    ));
    let context = egui::Context::default();
    let output = context.run_ui(Default::default(), |context| {
        draw_zone_band(
            &context.layer_painter(egui::LayerId::background()),
            Rect::from_min_max(pos2(0.0, 0.0), pos2(100.0, 100.0)),
            Rect::from_min_max(pos2(10.0, 10.0), pos2(90.0, 90.0)),
            DrawingSheetZoneGrid {
                columns: 4,
                rows: 4,
                labels: DrawingSheetZoneLabels::Coordinates,
                edges: DrawingSheetZoneEdges::All,
            },
            DrawingSheetLod::resolve(1.5, 1.0, test_lettering()),
            1.5,
            DrawingSheetPalette::resolve(&Tokens::get(context)),
            test_lettering(),
        );
    });
    assert_eq!(
        output.shapes.len(),
        12,
        "coordinates-only mode retains all twelve ruled band segments"
    );
    assert_eq!(sheet.geometry.zone_reference(point.0, point.1), None);
    assert!(
        !sheet.cursor_status(point.0, point.1).contains("zone"),
        "coordinates-only mode must not append a zone reference to status text"
    );
}

/// The lettering of the default A4 sheet, for tests that only need some.
fn test_lettering() -> DrawingSheetLettering {
    SchematicSheetFormat::default().authored_size.lettering()
}

#[test]
fn drawing_sheet_lettering_keeps_its_authored_height_on_the_page() {
    let data = authored_lettering(test_lettering().data_um);
    assert_eq!(data, 10.0, "2.5 mm is ten world units on A4");

    // A larger sheet letters larger, as ISO 3098 requires: text legible on A4
    // at arm's length is not legible on a sheet four times the size.
    let a0 = DrawingSheetLettering::for_sheet(841_000, 1_189_000);
    assert!(a0.data_um > test_lettering().data_um);

    // Sheet lettering is paper annotation: it magnifies with the sheet and is
    // never pinned to a screen size, because a pinned size overruns the cell
    // whose character budget was derived from the authored height.
    for zoom in [0.25_f32, 0.55, 1.0, 2.0, 4.0] {
        let painted = drawing_sheet_font_size(zoom, data);
        assert!(
            (painted - data * zoom).abs() <= 0.25,
            "at {zoom}x the painted height {painted} must track the authored {data}"
        );
    }
    assert!(
        drawing_sheet_font_size(0.25, data) < drawing_sheet_font_size(0.55, data),
        "lettering must shrink with the sheet rather than hold a floor"
    );

    // A caption never outgrows the value it names, at any zoom.
    let caption = authored_lettering(test_lettering().caption_um);
    for zoom in [0.25_f32, 1.0, 4.0] {
        assert!(drawing_sheet_font_size(zoom, caption) < drawing_sheet_font_size(zoom, data));
    }
}

#[test]
fn title_block_fields_are_withheld_until_their_lettering_is_legible() {
    let legible = DrawingSheetLod::resolve(1.0, 1.0, test_lettering());
    assert!(
        legible.show_title_block_fields,
        "2.5 mm lettering at four pixels per millimetre is ten pixels and must be drawn"
    );
    let illegible = DrawingSheetLod::resolve(0.3, 1.0, test_lettering());
    assert!(
        !illegible.show_title_block_fields,
        "2.5 mm lettering at three pixels is unreadable and must be withheld, not inflated"
    );
    assert!(
        illegible.show_border,
        "the block stays ruled when its text is withheld"
    );
}

#[test]
fn cursor_status_uses_the_configured_unit_and_explicit_sheet_context() {
    let format = SchematicSheetFormat::default()
        .try_update(|draft| draft.display_unit = DrawingSheetDisplayUnit::Inches)
        .expect("inch display unit is valid");
    let sheet = ActiveDrawingSheet {
        geometry: DrawingSheetGeometry::from_format(&format),
        format,
        sheet_name: "top".to_owned(),
        page_label: "1 of 1".to_owned(),
    };
    let one_inch = 25.4 * DRAWING_SHEET_UNITS_PER_MM;

    assert_eq!(
        sheet.cursor_status(
            DRAWING_SHEET_ORIGIN.0 + one_inch,
            DRAWING_SHEET_ORIGIN.1 + one_inch
        ),
        "x 1 · y 1 in · zone A1"
    );
    assert_eq!(
        sheet.cursor_status(DRAWING_SHEET_ORIGIN.0 - one_inch, DRAWING_SHEET_ORIGIN.1),
        "x -1 · y 0 in · off sheet"
    );
}

#[test]
fn fit_and_detail_contracts_match_the_permanent_canvas_mockup() {
    assert_eq!(FIT_SCREEN_INSET, 26.0);
    let detail = DrawingSheetLod::resolve(1.5, 1.0, test_lettering());
    assert!(detail.show_origin);
    assert!(detail.show_dimensions);
}

#[test]
fn overflow_advisories_remain_visible_in_thumbnail_views() {
    let thumbnail = DrawingSheetLod::resolve(0.1, 1.0, test_lettering());
    assert_eq!(thumbnail.tier, DrawingSheetLodTier::Thumbnail);
    assert!(!thumbnail.show_grid);
    assert!(overflow_advisories_visible(thumbnail.tier));
}

#[test]
fn title_block_rotation_preserves_center_and_swaps_complete_block_extent() {
    let target = Rect::from_center_size(pos2(200.0, 120.0), vec2(40.0, 100.0));
    let authored = Rect::from_center_size(target.center(), vec2(target.height(), target.width()));
    let angle = title_block_rotation_angle(DrawingSheetTitleBlockRotation::Clockwise90);
    let corners = [
        rotate_title_block_point(authored.left_top(), authored.center(), angle),
        rotate_title_block_point(authored.right_top(), authored.center(), angle),
        rotate_title_block_point(authored.right_bottom(), authored.center(), angle),
        rotate_title_block_point(authored.left_bottom(), authored.center(), angle),
    ];
    let bounds = Rect::from_points(&corners);

    assert_eq!(bounds.center(), target.center());
    assert!((bounds.width() - target.width()).abs() < f32::EPSILON);
    assert!((bounds.height() - target.height()).abs() < f32::EPSILON);
}

#[test]
fn title_block_text_uses_an_ellipsis_instead_of_a_clipped_final_glyph() {
    assert_eq!(truncate_title_text("ABCDEFG", 5), "ABCD…");
    assert_eq!(truncate_title_text("ABCD", 5), "ABCD");
    assert_eq!(truncate_title_text("ééééé", 4), "ééé…");
}

#[test]
fn overflow_report_retains_exact_item_identity_severity_coordinates_and_zone() {
    let mut document = SchematicDocument::default();
    document.components.push(
        Component::new(11, ComponentType::Resistor, Point::new(1_100, 100))
            .with_name_value("R_OUT", "1k"),
    );
    document.wires.push(Wire::new(
        21,
        vec![Point::new(-100, 100), Point::new(-80, 100)],
    ));
    document.wires.push(Wire::new(
        22,
        vec![Point::new(500, 650), Point::new(520, 650)],
    ));
    let symbol_context = SchematicSymbolContext::default();
    let sheet = test_sheet();

    let report = drawing_sheet_overflow_summary(&test_view(&document), &symbol_context, &sheet);

    assert_eq!(report.items.len(), 3);
    assert_eq!(report.off_paper, 1);
    assert_eq!(report.outside_border, 2);
    assert_eq!(report.title_block_collisions, 1);
    assert_eq!(report.finding_count(), 3);
    assert_eq!(
        report.first_target,
        Some(DrawingSheetOverflowTarget::Component(11))
    );
    let instance = &report.items[0];
    assert_eq!(instance.identity, "R_OUT");
    assert_eq!(instance.kind, "Instance");
    assert!(instance.outside_border);
    assert!(instance.off_paper);
    assert!(!instance.title_block_collision);
    assert_eq!(instance.severity, DrawingSheetOverflowSeverity::OffPaper);
    let border_wire = report
        .items
        .iter()
        .find(|item| item.target == DrawingSheetOverflowTarget::Wire(21))
        .expect("outside-border wire");
    assert_eq!(border_wire.sheet_coordinates_mm, (10.0, 35.0));
    assert_eq!(border_wire.zone, None);
    let title_wire = report
        .items
        .iter()
        .find(|item| item.target == DrawingSheetOverflowTarget::Wire(22))
        .expect("title-block wire");
    assert_eq!(
        title_wire.severity,
        DrawingSheetOverflowSeverity::TitleBlockCollision
    );
    assert!(title_wire.zone.is_some());
}

#[test]
fn every_durable_printable_object_class_participates_in_off_paper_review() {
    let mut document = SchematicDocument::default();
    document
        .junctions
        .push(Junction::new(31, Point::new(1_100, 100)));
    document
        .net_labels
        .push(NetLabel::new(32, Point::new(1_100, 140), "OUTSIDE"));
    document.design_notes.push(
        DesignNote::new(
            33,
            Point::new(1_100, 180),
            DesignNoteKind::PlainText,
            "outside note",
        )
        .expect("valid design note"),
    );
    document.documentation_shapes.push(
        DocumentationShape::new(
            34,
            DocumentationShapeGeometry::Line {
                start: Point::new(1_100, 220),
                end: Point::new(1_140, 240),
            },
        )
        .expect("valid documentation line"),
    );
    let symbol_context = SchematicSymbolContext::default();
    let sheet = test_sheet();

    let report = drawing_sheet_overflow_summary(&test_view(&document), &symbol_context, &sheet);

    assert_eq!(report.items.len(), 4);
    assert_eq!(report.outside_border, 4);
    assert_eq!(report.off_paper, 4);
    assert_eq!(
        report
            .items
            .iter()
            .map(|item| item.target)
            .collect::<Vec<_>>(),
        vec![
            DrawingSheetOverflowTarget::Junction(31),
            DrawingSheetOverflowTarget::NetLabel(32),
            DrawingSheetOverflowTarget::DesignNote(33),
            DrawingSheetOverflowTarget::DocumentationShape(34),
        ]
    );
    assert!(
        report
            .items
            .iter()
            .all(|item| item.severity == DrawingSheetOverflowSeverity::OffPaper)
    );
}

#[test]
fn component_overflow_includes_the_hardcopy_name_and_value_extent() {
    let mut document = SchematicDocument::default();
    document.components.push(
        Component::new(61, ComponentType::Resistor, Point::new(1_000, 100))
            .with_name_value("R_INSTANCE_NAME_REACHES_BEYOND_THE_PAPER_EDGE", "1k"),
    );
    let symbol_context = SchematicSymbolContext::default();
    let sheet = test_sheet();
    let (body_min, body_max) = symbol_context.component_bounds(&document.components[0]);
    let body = WorldRect::from_points(body_min, body_max);
    let object = active_object_bounds(&test_view(&document), &symbol_context)
        .into_iter()
        .find(|object| object.target == DrawingSheetOverflowTarget::Component(61))
        .expect("component is printable");

    assert!(
        sheet.geometry.paper.contains_rect(body),
        "the symbol body itself remains on the authored paper"
    );
    assert!(
        !sheet.geometry.paper.contains_rect(object.bounds),
        "the retained hardcopy label extends the reviewed printable bounds"
    );
    let item = drawing_sheet_overflow_summary(&test_view(&document), &symbol_context, &sheet)
        .items
        .into_iter()
        .find(|item| item.target == DrawingSheetOverflowTarget::Component(61))
        .expect("label-only off-paper overflow is reported");
    assert_eq!(item.severity, DrawingSheetOverflowSeverity::OffPaper);
}

#[test]
fn conductor_title_collision_uses_segments_not_the_route_bounding_box() {
    let title_block = WorldRect {
        min_x: 10.0,
        min_y: 10.0,
        max_x: 20.0,
        max_y: 20.0,
    };
    let route = vec![Point::new(0, 15), Point::new(0, 0), Point::new(15, 0)];
    let route_bounds = points_bounds(&route).expect("the route has points");
    let mut document = SchematicDocument::default();
    document.wires.push(Wire::new(91, route));
    let symbol_context = SchematicSymbolContext::default();
    let object = active_object_bounds(&test_view(&document), &symbol_context)
        .into_iter()
        .find(|object| object.target == DrawingSheetOverflowTarget::Wire(91))
        .expect("the routed wire is printable on the active sheet");

    assert!(
        title_block.intersects(route_bounds),
        "the historical route-AABB implementation would report a collision"
    );
    assert!(
        !object.intersects(title_block),
        "neither segment of the L-shaped route intersects the title block"
    );
    assert_eq!(
        object.bounds, route_bounds,
        "navigation retains full bounds"
    );

    let mut geometry = DrawingSheetGeometry::from_format(&SchematicSheetFormat::default());
    geometry.paper = WorldRect {
        min_x: -100.0,
        min_y: -100.0,
        max_x: 100.0,
        max_y: 100.0,
    };
    geometry.content_area = geometry.paper;
    geometry.title_block = Some(title_block);
    assert!(
        overflow_items(&test_view(&document), &symbol_context, &geometry).is_empty(),
        "the overflow report must not contain the historical false-positive finding"
    );
}
