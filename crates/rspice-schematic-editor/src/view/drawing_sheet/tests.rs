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
use rspice_design_model::design_management::{
    DrawingSheetStandard, DrawingSheetZoneEdges, DrawingSheetZoneGrid, DrawingSheetZones,
};
use rspice_design_model::{SchematicPageOrientation, SchematicPageSize};

fn test_sheet() -> ActiveDrawingSheet {
    let format = SchematicSheetFormat::default();
    ActiveDrawingSheet {
        geometry: DrawingSheetGeometry::from_format(&format),
        format,
        design_name: "top".to_owned(),
        page_label: "1 of 1".to_owned(),
    }
}
fn test_view(document: &SchematicDocument) -> DesignView<'_> {
    DesignView {
        document,
        canvas_cache: None,
        sheet_catalog: None,
        review_markers: Default::default(),
        parameter_labels: Default::default(),
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
    // ISO 5457: the frame at the borders, the grid reference band outside.
    assert_eq!(geometry.physical.drawing_area, geometry.physical.printable);
    assert_eq!(geometry.paper.min_x, -140.0);
    assert_eq!(geometry.paper.min_y, -40.0);
    assert_eq!(geometry.paper.max_x, 1_048.0);
    assert_eq!(geometry.paper.max_y, 800.0);
    assert_eq!(
        geometry.physical.zones,
        Some(DrawingSheetZoneGrid {
            columns: 6,
            rows: 4,
            edges: DrawingSheetZoneEdges::TopAndRight,
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
fn a_frame_without_zones_letters_no_references_and_quotes_none() {
    let format = SchematicSheetFormat::default()
        .try_update(|draft| draft.zones = DrawingSheetZones::none())
        .expect("a frame may carry no zones");
    let sheet = ActiveDrawingSheet {
        geometry: DrawingSheetGeometry::from_format(&format),
        format,
        design_name: "top".to_owned(),
        page_label: "1 of 1".to_owned(),
    };
    let point = (
        sheet.geometry.drawing_area.min_x + 1.0,
        sheet.geometry.drawing_area.min_y + 1.0,
    );

    let shapes = painted_frame(&sheet, 1.5);
    let ticks = sheet
        .geometry
        .frame
        .layer(DrawingSheetFrameLayer::Zones)
        .count();
    assert_eq!(ticks, 0, "no field boundary is ruled");
    assert_eq!(
        sheet
            .geometry
            .frame
            .layer(DrawingSheetFrameLayer::Border)
            .count(),
        1,
        "only the frame is ruled: with no zones there is no band for a grid reference border to close"
    );
    assert_eq!(
        lettered_text(&shapes),
        ["A4"],
        "no zone reference is lettered, only the sheet size"
    );
    assert_eq!(sheet.geometry.zone_reference(point.0, point.1), None);
    assert!(
        !sheet.cursor_status(point.0, point.1).contains("zone"),
        "a frame without zones must not append a zone reference to status text"
    );
}

/// The lettering of the default A4 sheet, for tests that only need some.
fn test_lettering() -> DrawingSheetLettering {
    SchematicSheetFormat::default().authored_size.lettering()
}

fn active(format: SchematicSheetFormat) -> ActiveDrawingSheet {
    ActiveDrawingSheet {
        geometry: DrawingSheetGeometry::from_format(&format),
        format,
        design_name: "top".to_owned(),
        page_label: "1 of 1".to_owned(),
    }
}

/// The shapes the canvas paints for `sheet`'s frame at `zoom`, with the
/// sheet's top left at the screen origin.
fn painted_frame(sheet: &ActiveDrawingSheet, zoom: f32) -> Vec<Shape> {
    let viewport = Viewport {
        offset: pos2(
            -(DRAWING_SHEET_ORIGIN.0 as f32) * zoom,
            -(DRAWING_SHEET_ORIGIN.1 as f32) * zoom,
        ),
        zoom,
        bounds: Rect::from_min_size(Pos2::ZERO, vec2(4_000.0, 4_000.0)),
    };
    let context = egui::Context::default();
    context
        .run_ui(Default::default(), |context| {
            let painter = context.layer_painter(egui::LayerId::background());
            let lod = DrawingSheetLod::resolve(zoom, 1.0, test_lettering());
            let palette = DrawingSheetPalette::resolve(&Tokens::get(context));
            draw_frame(
                &painter,
                &viewport,
                sheet,
                lod,
                DrawingSheetLayerVisibility::default(),
                palette,
            );
        })
        .shapes
        .into_iter()
        .map(|clipped| clipped.shape)
        .collect()
}

#[test]
fn the_canvas_draws_centring_marks_and_no_margin_guide_on_an_iso_frame() {
    let a4 = active(SchematicSheetFormat::default());
    assert!(a4.format.marks.centring);
    let marks = a4
        .geometry
        .frame
        .layer(DrawingSheetFrameLayer::CentringMarks)
        .count();
    assert_eq!(marks, 4);
    // The frame lies on the margins, so no dashed guide is drawn for them.
    assert_eq!(
        a4.geometry.physical.drawing_area,
        a4.geometry.physical.printable
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
        design_name: "top".to_owned(),
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
fn overflow_advisories_remain_visible_in_thumbnail_views() {
    let thumbnail = DrawingSheetLod::resolve(0.1, 1.0, test_lettering());
    assert_eq!(thumbnail.tier, DrawingSheetLodTier::Thumbnail);
    assert!(!thumbnail.show_grid);
    assert!(overflow_advisories_visible(thumbnail.tier));
}

/// Every shape the canvas paints for `sheet`'s title block at one world unit
/// per point, lettering `fields`.
fn painted_title_block(
    sheet: &ActiveDrawingSheet,
    fields: &[ResolvedDrawingSheetTitleField],
) -> Vec<Shape> {
    let world = sheet
        .geometry
        .title_block
        .expect("the sheet carries a title block");
    let block = Rect::from_min_max(
        pos2(world.min_x as f32, world.min_y as f32),
        pos2(world.max_x as f32, world.max_y as f32),
    );
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    let frame = || {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(2_000.0, 2_000.0))),
                ..Default::default()
            },
            |ui| {
                draw_title_block(
                    &ui.ctx().layer_painter(egui::LayerId::background()),
                    block,
                    DrawingSheetLod::resolve(1.0, 1.0, DRAWING_SHEET_TITLE_BLOCK_LETTERING),
                    1.0,
                    DrawingSheetPalette::resolve(&Tokens::get(ui.ctx())),
                    || fields.to_vec(),
                    sheet,
                );
            },
        )
    };
    // Fonts are built on the first pass and laid out from the second.
    drop(frame());
    frame()
        .shapes
        .into_iter()
        .map(|clipped| clipped.shape)
        .collect()
}

/// The text of every line `shapes` letter, in paint order.
fn lettered_text(shapes: &[Shape]) -> Vec<String> {
    shapes
        .iter()
        .filter_map(|shape| match shape {
            Shape::Text(text) => Some(text.galley.text().to_owned()),
            _ => None,
        })
        .collect()
}

/// The left and right edges of the glyphs of `galley` lettering a character
/// `of` picks, in the galley's own frame.
fn glyph_span(galley: &egui::Galley, of: impl Fn(char) -> bool) -> (f32, f32) {
    galley
        .rows
        .iter()
        .flat_map(|row| row.row.glyphs.iter().map(move |glyph| (row.pos.x, glyph)))
        .filter(|(_, glyph)| of(glyph.chr))
        .fold(
            (f32::INFINITY, f32::NEG_INFINITY),
            |(left, right), (x, glyph)| {
                (
                    left.min(x + glyph.pos.x),
                    right.max(x + glyph.pos.x + glyph.advance_width),
                )
            },
        )
}

#[test]
fn a_title_typed_in_arabic_is_lettered_in_the_order_it_is_read() {
    // Arabic reads from the right, and the sheet number inside the title
    // reads left to right where it stands: last, so at the left of the line.
    let sheet = test_sheet();
    let fields = rspice_design_model::design_management::resolve_drawing_sheet_title_fields(
        &sheet.format,
        &[(
            rspice_design_model::design_management::DrawingSheetTitleFieldId::SheetTitle,
            "مخطط المضخم 12".to_owned(),
        )]
        .into_iter()
        .collect(),
    );
    let shapes = painted_title_block(&sheet, &fields);
    let title = shapes
        .iter()
        .find_map(|shape| match shape {
            Shape::Text(text) if text.galley.text().contains("12") => Some(&text.galley),
            _ => None,
        })
        .expect("the title is lettered");
    let arabic = |character: char| ('\u{0600}'..='\u{06FF}').contains(&character);
    let (number_left, number_right) = glyph_span(title, |character| character.is_ascii_digit());
    let (arabic_left, arabic_right) = glyph_span(title, arabic);
    assert!(number_left < number_right && arabic_left < arabic_right);
    // Glyphs are seated on whole pixels, so neighbours may overlap by less
    // than one.
    assert!(
        number_right < arabic_left + 1.0,
        "the number stands left of the words: {number_left}..{number_right} \
         against {arabic_left}..{arabic_right}"
    );
    // "12" keeps its own order inside the line.
    let one = glyph_span(title, |character| character == '1');
    let two = glyph_span(title, |character| character == '2');
    assert!(one.1 < two.0 + 1.0, "{one:?} {two:?}");
}

#[test]
fn sheet_one_of_several_on_the_asme_form_paints_the_same_revision_note() {
    let format = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::AnsiB,
        SchematicPageOrientation::Landscape,
    );
    let painted = |page: &str| {
        let sheet = ActiveDrawingSheet {
            geometry: DrawingSheetGeometry::from_format(&format),
            format: format.clone(),
            design_name: "top".to_owned(),
            page_label: page.to_owned(),
        };
        let viewport = Viewport {
            offset: Pos2::ZERO,
            zoom: 1.0,
            bounds: Rect::from_min_size(Pos2::ZERO, vec2(2_000.0, 2_000.0)),
        };
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        let frame = || {
            ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(2_000.0, 2_000.0))),
                    ..Default::default()
                },
                |ui| {
                    draw_same_revision_note(
                        &ui.ctx().layer_painter(egui::LayerId::background()),
                        &viewport,
                        DrawingSheetLod::resolve(1.0, 1.0, DRAWING_SHEET_TITLE_BLOCK_LETTERING),
                        DrawingSheetPalette::resolve(&Tokens::get(ui.ctx())),
                        &sheet,
                    );
                },
            )
        };
        drop(frame());
        let shapes = frame()
            .shapes
            .into_iter()
            .map(|clipped| clipped.shape)
            .collect::<Vec<_>>();
        lettered_text(&shapes)
    };
    assert_eq!(
        painted("1 of 3"),
        ["ALL SHEETS ARE THE SAME REVISION LEVEL"]
    );
    assert!(painted("2 of 3").is_empty());
    assert!(painted("1 of 1").is_empty());
}

#[test]
fn the_canvas_letters_each_field_in_the_cell_the_sheet_lays_out() {
    // The canvas draws the cells hardcopy plots, so a value is cut short on
    // screen exactly where it is on paper and nowhere else.
    use rspice_design_model::design_management::DrawingSheetTitleFieldId as Field;
    let sheet = active(SchematicSheetFormat::from_standard(
        DrawingSheetStandard::IsoA4,
        SchematicPageOrientation::Landscape,
    ));
    let long = "a value far longer than any cell of the block can hold";
    let values = [
        (Field::CellView, long),
        (Field::Page, "12 of 12"),
        (Field::Revision, "C"),
        (Field::Date, "2026-10-18"),
        (Field::DocumentStatus, "Under approval"),
    ]
    .into_iter()
    .map(|(id, value)| (id, value.to_owned()))
    .collect();
    let fields = rspice_design_model::design_management::resolve_drawing_sheet_title_fields(
        &sheet.format,
        &values,
    );
    let layout = drawing_sheet_title_layout(&sheet.format, &sheet.geometry.physical)
        .expect("the sheet carries a title block");
    let text = lettered_text(&painted_title_block(&sheet, &fields));
    let lettering = layout.lettering(&fields);
    assert_eq!(
        text,
        lettering
            .iter()
            .map(|line| line.text.clone())
            .collect::<Vec<_>>(),
        "the canvas paints exactly the lines the layout letters"
    );
    for whole in ["12 of 12", "2026-10-18", "Under approval"] {
        assert!(
            text.iter().any(|line| line == whole),
            "{whole} is cut: {text:?}"
        );
    }
    // A cell / view path is cut at its start, where the ellipsis stands.
    assert!(
        text.iter()
            .any(|line| line.starts_with('…') && long.ends_with(line.trim_start_matches('…'))),
        "a value longer than its cell is cut with an ellipsis: {text:?}"
    );
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
    let sheet = active(SchematicSheetFormat::from_standard(
        DrawingSheetStandard::IsoA4,
        SchematicPageOrientation::Landscape,
    ));

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
    assert_eq!(instance.identity.to_string(), "R_OUT");
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

/// The halo meshes one canvas frame paints over `summary` in `viewport`.
fn painted_halos(
    ctx: &egui::Context,
    viewport: &Viewport,
    summary: &Arc<DrawingSheetOverflowSummary>,
) -> Vec<Arc<Mesh>> {
    let input = egui::RawInput {
        screen_rect: Some(viewport.bounds),
        ..Default::default()
    };
    ctx.run_ui(input, |ui| {
        let painter = ui.ctx().layer_painter(egui::LayerId::background());
        draw_overflow_advisories(&painter, viewport.bounds, viewport, summary);
    })
    .shapes
    .into_iter()
    .filter_map(|clipped| match clipped.shape {
        Shape::Mesh(mesh) => Some(mesh),
        _ => None,
    })
    .collect()
}

#[test]
fn overflow_halos_are_tessellated_once_while_the_advisories_and_view_stand_still() {
    let mut document = SchematicDocument::default();
    document.components.push(
        Component::new(11, ComponentType::Resistor, Point::new(1_100, 100))
            .with_name_value("R_OUT", "1k"),
    );
    document.wires.push(Wire::new(
        21,
        vec![Point::new(-100, 100), Point::new(-80, 100)],
    ));
    let summary = Arc::new(drawing_sheet_overflow_summary(
        &test_view(&document),
        &SchematicSymbolContext::default(),
        &test_sheet(),
    ));
    assert_eq!(summary.items.len(), 2);
    let ctx = egui::Context::default();
    let mut viewport = Viewport {
        offset: pos2(300.0, 200.0),
        zoom: 0.5,
        bounds: Rect::from_min_size(Pos2::ZERO, vec2(1_200.0, 900.0)),
    };

    let first = painted_halos(&ctx, &viewport, &summary);
    assert_eq!(first.len(), 1, "every halo is painted as one mesh");
    assert!(
        !first[0].indices.is_empty(),
        "the halos are tessellated into the mesh"
    );
    assert!(
        viewport.bounds.contains_rect(first[0].calc_bounds()),
        "every halo stays on the canvas"
    );

    let steady = painted_halos(&ctx, &viewport, &summary);
    assert!(
        Arc::ptr_eq(&first[0], &steady[0]),
        "a steady frame paints the mesh it painted last"
    );

    let found_again = Arc::new(summary.as_ref().clone());
    let refound = painted_halos(&ctx, &viewport, &found_again);
    assert!(
        !Arc::ptr_eq(&steady[0], &refound[0]),
        "advisories found afresh are dashed afresh"
    );

    viewport.offset.x += 40.0;
    let panned = painted_halos(&ctx, &viewport, &found_again);
    assert!(
        !Arc::ptr_eq(&refound[0], &panned[0]),
        "a moved view dashes the halos where they now stand"
    );
    let shift = panned[0].calc_bounds().min.x - refound[0].calc_bounds().min.x;
    assert!(
        (shift - 40.0).abs() < 1.0,
        "the halos move with the view, by {shift}"
    );
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

/// A PWL source of 64 points letters "PWL", its point count and its end
/// times, not its table: drawn well inside the sheet, it is not reported as
/// running off the paper by the length of the table it does not letter.
#[test]
fn a_long_pwl_source_is_measured_by_the_summary_it_letters() {
    let table = (0..64)
        .map(|k| format!("{k}.501u 0.123456789"))
        .collect::<Vec<_>>()
        .join(" ");
    let mut document = SchematicDocument::default();
    document.components.push(
        Component::new(62, ComponentType::VoltageSourcePwl, Point::new(20, 400))
            .with_name_value("vin", &table),
    );
    let symbol_context = SchematicSymbolContext::default();
    let sheet = test_sheet();
    let object = active_object_bounds(&test_view(&document), &symbol_context)
        .into_iter()
        .find(|object| object.target == DrawingSheetOverflowTarget::Component(62))
        .expect("the source is printable");
    assert!(
        sheet.geometry.drawing_area.contains_rect(object.bounds),
        "{:?} against {:?}",
        object.bounds,
        sheet.geometry.drawing_area
    );
    let report = drawing_sheet_overflow_summary(&test_view(&document), &symbol_context, &sheet);
    assert!(report.is_clear(), "{:?}", report.items);
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

#[test]
fn a_title_in_any_script_is_painted_cut_and_set_where_the_plot_sets_it() {
    use rspice_design_model::design_management::{
        DrawingSheetTitleFieldValueAuthority, resolve_drawing_sheet_title_fields,
    };
    // Arabic with its vowel marks, Hebrew with a number, a unit and
    // brackets, and Thai with its tone marks, each long enough to be cut;
    // and a Latin value, set on the left.
    let mut values = [
        "\u{645}\u{64e}\u{631}\u{652}\u{62d}\u{64e}\u{628}\u{64b}\u{627} \u{628}\u{650}\u{643}\u{64f}\u{645}\u{652} ",
        "\u{5de}\u{5d2}\u{5d1}\u{5e8} R1 = 10 k\u{3a9} (V+) ",
        "\u{e2a}\u{e27}\u{e31}\u{e2a}\u{e14}\u{e35}\u{e04}\u{e23}\u{e31}\u{e1a} ",
        "J. Whitfield ",
    ]
    .map(|phrase| phrase.repeat(6).trim_end().to_owned())
    .into_iter();
    let format = SchematicSheetFormat::default()
        .try_update(|draft| {
            for (field, state) in &mut draft.title_block.fields {
                if field.policy().value_authority == DrawingSheetTitleFieldValueAuthority::Authored
                    && !field.is_signature_date()
                    && state.visible
                    && let Some(value) = values.next()
                {
                    state.value = value;
                }
            }
        })
        .unwrap();
    let sheet = active(format);
    let fields = resolve_drawing_sheet_title_fields(&sheet.format, &Default::default());
    let layout = drawing_sheet_title_layout(&sheet.format, &sheet.geometry.physical).unwrap();
    let lettering = layout.lettering_measured(&fields, &DrawingSheetTitleWidth);
    assert!(lettering.iter().any(|line| line.text.ends_with('…')));
    assert!(
        lettering
            .iter()
            .any(|line| line.align == DrawingSheetTitleAlign::Right)
    );
    let shapes = painted_title_block(&sheet, &fields);
    // The canvas letters exactly the lines the layout cuts, by the width
    // print sets them at: a line holding no right-to-left text as it is,
    // and any other with the same characters in the order it is read, its
    // brackets mirrored inside a right-to-left run.
    let painted = lettered_text(&shapes);
    assert_eq!(painted.len(), lettering.len(), "{painted:?}");
    let characters = |text: &str| {
        let mut characters = text
            .chars()
            .map(|character| if character == ')' { '(' } else { character })
            .collect::<Vec<_>>();
        characters.sort_unstable();
        characters
    };
    for (painted, line) in painted.iter().zip(&lettering) {
        if line.text.is_ascii() {
            assert_eq!(*painted, line.text);
        } else {
            assert_eq!(characters(painted), characters(&line.text), "{painted:?}");
        }
    }
    // A cut value read right to left ends at the left, where it is read to.
    let title = painted
        .iter()
        .zip(&lettering)
        .find(|(_, line)| line.align == DrawingSheetTitleAlign::Right && line.text.ends_with('…'))
        .map(|(painted, _)| painted)
        .expect("a right-to-left value is cut");
    assert!(title.starts_with('…'), "{title:?}");
    // Each stands against the edge of its run the plot sets it on.
    let world = sheet.geometry.title_block.unwrap();
    let x_at = |x_um: u64| {
        world.min_x as f32
            + (world.max_x - world.min_x) as f32 * x_um as f32 / layout.width_um as f32
    };
    let texts = shapes.iter().filter_map(|shape| match shape {
        Shape::Text(text) => Some(text),
        _ => None,
    });
    for (text, line) in texts.zip(&lettering) {
        let (painted, laid_out) = match line.align {
            DrawingSheetTitleAlign::Left => (text.pos.x, x_at(line.x_um)),
            DrawingSheetTitleAlign::Right => (
                text.pos.x + text.galley.size().x,
                x_at(line.x_um + line.width_um),
            ),
        };
        assert!(
            (painted - laid_out).abs() < 0.5,
            "{:?} stands at {painted}, not {laid_out}",
            line.text
        );
    }
}
