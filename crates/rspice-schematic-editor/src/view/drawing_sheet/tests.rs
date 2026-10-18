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
    DRAWING_SHEET_TITLE_OUTLINE_UM, DrawingSheetStandard, DrawingSheetZoneEdges,
    DrawingSheetZoneGrid, DrawingSheetZones,
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
fn a_frame_without_zones_letters_no_references_and_quotes_none() {
    let format = SchematicSheetFormat::default()
        .try_update(|draft| draft.zones = DrawingSheetZones::none())
        .expect("a frame may carry no zones");
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

#[test]
fn drawing_sheet_lettering_keeps_its_authored_height_on_the_page() {
    // The height the standard names is the height of the capitals, and the
    // size a font engine takes is an em, so these are not the same number:
    // 2.5 mm capitals need a 3.58 mm em. Ten world units of capitals is what
    // has to come back out.
    let data = authored_lettering(test_lettering().data_um);
    assert!(
        data > 10.0,
        "an em lettering 2.5 mm capitals is taller than the capitals"
    );
    assert!(
        (data * 0.698 - 10.0).abs() < 0.05,
        "an em of {data} world units must letter capitals ten world units tall"
    );

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

fn active(format: SchematicSheetFormat) -> ActiveDrawingSheet {
    ActiveDrawingSheet {
        geometry: DrawingSheetGeometry::from_format(&format),
        format,
        sheet_name: "top".to_owned(),
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

fn stroke_widths(shapes: &[Shape]) -> Vec<f32> {
    shapes
        .iter()
        .filter_map(|shape| match shape {
            Shape::LineSegment { stroke, .. } => Some(stroke.width),
            Shape::Rect(rect) => Some(rect.stroke.width),
            _ => None,
        })
        .collect()
}

#[test]
fn the_canvas_paints_the_frame_at_the_widths_the_plot_prints() {
    // Four world units per millimetre: at zoom 1.5 a 0.7 mm frame is 4.2
    // points and a 0.35 mm grid line 2.1.
    let a4 = active(SchematicSheetFormat::default());
    let widths = stroke_widths(&painted_frame(&a4, 1.5));
    assert!(widths.iter().any(|width| (width - 4.2).abs() < 1e-3));
    assert!(widths.iter().any(|width| (width - 2.1).abs() < 1e-3));
    assert!(
        widths
            .iter()
            .all(|width| (width - 4.2).abs() < 1e-3 || (width - 2.1).abs() < 1e-3),
        "ISO 5457 draws 0.7 mm and 0.35 mm lines only: {widths:?}"
    );
    // ASME Y14.1: a 0.6 mm border and 0.3 mm zone markers.
    let ansi_d = active(SchematicSheetFormat::from_standard(
        DrawingSheetStandard::AnsiD,
        SchematicPageOrientation::Landscape,
    ));
    let widths = stroke_widths(&painted_frame(&ansi_d, 1.5));
    assert!(widths.iter().any(|width| (width - 3.6).abs() < 1e-3));
    assert!(widths.iter().any(|width| (width - 1.8).abs() < 1e-3));
    // Zoomed far out a line keeps one physical pixel rather than vanishing.
    assert!(
        stroke_widths(&painted_frame(&a4, 0.1))
            .iter()
            .all(|width| *width >= 1.0)
    );
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
fn zone_references_letter_at_the_height_of_their_band_not_their_sheet() {
    let lettered = |sheet: &ActiveDrawingSheet| {
        painted_frame(sheet, 1.0)
            .into_iter()
            .filter_map(|shape| match shape {
                Shape::Text(text) => Some(text.galley.job.sections[0].format.font_id.size),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    // ISO 5457 letters a grid reference 3.5 mm high whatever the paper, and
    // a Standard band is 5 mm wide on A0 as on A4. Fourteen world units of
    // capitals is that 3.5 mm; the A4 size designation beside them is
    // lettered at the same height.
    let a4 = lettered(&test_sheet());
    let a0 = lettered(&active(SchematicSheetFormat::from_standard(
        DrawingSheetStandard::IsoA0,
        SchematicPageOrientation::Landscape,
    )));
    assert!(!a4.is_empty());
    for size in a4.iter().chain(&a0) {
        assert!(
            (size * 0.698 - 14.0).abs() < 0.2,
            "a {size}-unit em must letter 3.5 mm capitals"
        );
    }

    // ASME Y14.2 letters zones 6 mm high in the 0.50 in margin.
    let ansi_d = lettered(&active(SchematicSheetFormat::from_standard(
        DrawingSheetStandard::AnsiD,
        SchematicPageOrientation::Landscape,
    )));
    assert_eq!(ansi_d.len(), 2 * (8 + 4));
    assert!(ansi_d.iter().all(|size| (size * 0.698 - 24.0).abs() < 0.2));
}

#[test]
fn title_block_capitals_stand_at_the_height_the_standard_names() {
    // The arithmetic above is only worth as much as the assumption under it:
    // that the size egui takes is an em. Paint a capital at the size a 2.5 mm
    // field is lettered at and measure the ink it leaves. Ten world units of
    // capital is the 2.5 mm ISO 3098 names, and a size taken as a cap height
    // instead of an em would leave seven.
    let em = authored_lettering(test_lettering().data_um);
    let rows = rspice_ui_kit::raster::glyph_ink_rows("H", em);
    let (first, last) = (
        *rows.first().expect("a capital leaves ink"),
        *rows.last().expect("a capital leaves ink"),
    );
    let painted = (last - first + 1) as f32;
    assert!(
        (painted - 10.0).abs() <= 1.5,
        "a capital lettered at a {em}-unit em measured {painted} units, not the authored ten"
    );
}

#[test]
fn title_block_fields_are_withheld_until_their_lettering_is_legible() {
    let legible = DrawingSheetLod::resolve(1.0, 1.0, test_lettering());
    assert!(
        legible.show_title_block_fields,
        "2.5 mm capitals at four pixels per millimetre set a 14-pixel em and must be drawn"
    );
    let illegible = DrawingSheetLod::resolve(0.3, 1.0, test_lettering());
    assert!(
        !illegible.show_title_block_fields,
        "the same capitals at three tenths of that zoom set a 4-pixel em, which is \
         unreadable and must be withheld rather than inflated"
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

#[test]
fn an_automatic_field_is_lettered_like_any_other() {
    // ISO 7200 marks no field as filled in by the system, so the canvas
    // neither marks such a cell nor letters its caption any differently.
    let sheet = test_sheet();
    let fields = rspice_design_model::design_management::resolve_drawing_sheet_title_fields(
        &sheet.format,
        &[(
            rspice_design_model::design_management::DrawingSheetTitleFieldId::Page,
            "1 of 1".to_owned(),
        )]
        .into_iter()
        .collect(),
    );
    assert!(fields.iter().any(|field| {
        field.authority
            == rspice_design_model::design_management::DrawingSheetTitleFieldValueAuthority::Automatic
    }));
    let shapes = painted_title_block(&sheet, &fields);
    assert!(
        !shapes.iter().any(|shape| matches!(shape, Shape::Circle(_))),
        "an automatic cell carries no marker"
    );
    let text = lettered_text(&shapes);
    let layout = drawing_sheet_title_layout(&sheet.format, &sheet.geometry.physical)
        .expect("the sheet carries a title block");
    for field in &fields {
        let caption = layout.cell(field.id).unwrap().letter_caption();
        assert!(
            text.contains(&caption),
            "{:?} must be captioned with its form's wording: {text:?}",
            field.id
        );
    }
    // ISO 7200's own terms, as the plot letters them.
    for caption in [
        "Rev.",
        "Date of issue",
        "Sheet",
        "Title",
        "Identification number",
    ] {
        assert!(text.iter().any(|line| line == caption), "{text:?}");
    }
}

#[test]
fn an_empty_authored_field_is_painted_as_its_caption_alone() {
    // Created by and Approved by start on a new sheet empty: boxes to sign
    // on the print, with no dash or empty line of text in them.
    let sheet = test_sheet();
    let fields = rspice_design_model::design_management::resolve_drawing_sheet_title_fields(
        &sheet.format,
        &std::collections::BTreeMap::new(),
    );
    let blank = fields.iter().filter(|field| field.value.is_empty()).count();
    assert!(blank >= 2, "a new sheet starts with unsigned fields");
    let text = lettered_text(&painted_title_block(&sheet, &fields));
    assert!(
        text.iter().all(|line| !line.trim().is_empty()),
        "an empty value paints no text: {text:?}"
    );
    let layout = drawing_sheet_title_layout(&sheet.format, &sheet.geometry.physical)
        .expect("the sheet carries a title block");
    assert_eq!(text.len(), layout.cells.len() + fields.len() - blank);
    for caption in ["Created by", "Approved by"] {
        assert!(text.iter().any(|line| line == caption), "{text:?}");
    }
}

#[test]
fn title_block_lines_are_the_plots_widths_magnified_with_the_sheet() {
    // 0.7 mm outline and 0.35 mm rules at four world units per millimetre.
    assert!((title_line_width(DRAWING_SHEET_TITLE_OUTLINE_UM, 1.0, 1.0) - 2.8).abs() < 1e-4);
    assert!((title_line_width(DRAWING_SHEET_TITLE_RULE_UM, 1.0, 1.0) - 1.4).abs() < 1e-4);
    assert!((title_line_width(DRAWING_SHEET_TITLE_RULE_UM, 2.0, 1.0) - 2.8).abs() < 1e-4);
    // Never thinner than one physical pixel.
    assert!((title_line_width(DRAWING_SHEET_TITLE_RULE_UM, 0.1, 2.0) - 0.5).abs() < 1e-4);

    let sheet = test_sheet();
    let fields = rspice_design_model::design_management::resolve_drawing_sheet_title_fields(
        &sheet.format,
        &std::collections::BTreeMap::new(),
    );
    let layout = drawing_sheet_title_layout(&sheet.format, &sheet.geometry.physical).unwrap();
    let shapes = painted_title_block(&sheet, &fields);
    let rules = shapes
        .iter()
        .filter_map(|shape| match shape {
            Shape::LineSegment { stroke, .. } => Some(stroke.width),
            _ => None,
        })
        .collect::<Vec<_>>();
    // Each cell's own rules, each painted once, at the narrow width.
    assert_eq!(rules.len(), layout.rules.len());
    assert!(rules.iter().all(|width| (width - 1.4).abs() < 1e-4));
    let outline = shapes
        .iter()
        .find_map(|shape| match shape {
            Shape::Path(path) => Some(path.stroke.width),
            _ => None,
        })
        .expect("the outline is painted");
    assert!((outline - 2.8).abs() < 1e-4);
}

#[test]
fn an_asme_sheet_paints_the_forms_lines_and_lettering_the_plot_prints() {
    use rspice_design_model::design_management::{
        DrawingSheetTitleBlockTemplate, DrawingSheetTitleFieldId as Field,
    };
    for standard in [
        DrawingSheetStandard::AnsiB,
        DrawingSheetStandard::AnsiD,
        DrawingSheetStandard::ArchD,
    ] {
        let format =
            SchematicSheetFormat::from_standard(standard, SchematicPageOrientation::Landscape);
        assert_eq!(
            format.title_block.template,
            DrawingSheetTitleBlockTemplate::AsmeY141
        );
        let sheet = ActiveDrawingSheet {
            geometry: DrawingSheetGeometry::from_format(&format),
            format,
            sheet_name: "top".to_owned(),
            page_label: "1 of 3".to_owned(),
        };
        let fields = rspice_design_model::design_management::resolve_drawing_sheet_title_fields(
            &sheet.format,
            &[
                (Field::Page, "1 of 3".to_owned()),
                (Field::DocumentId, "RS-0042".to_owned()),
            ]
            .into_iter()
            .collect(),
        );
        let layout = drawing_sheet_title_layout(&sheet.format, &sheet.geometry.physical).unwrap();
        let shapes = painted_title_block(&sheet, &fields);
        // The 0.6 mm outline and each rule at its own width, magnified as
        // the plot's widths are.
        let outline = shapes
            .iter()
            .find_map(|shape| match shape {
                Shape::Path(path) => Some(path.stroke.width),
                _ => None,
            })
            .expect("the outline is painted");
        assert!((outline - title_line_width(600, 1.0, 1.0)).abs() < 1e-4);
        let mut painted = shapes
            .iter()
            .filter_map(|shape| match shape {
                Shape::LineSegment { stroke, .. } => Some(stroke.width),
                _ => None,
            })
            .collect::<Vec<_>>();
        let mut laid_out = layout
            .rules
            .iter()
            .map(|rule| title_line_width(rule.width_um, 1.0, 1.0))
            .collect::<Vec<_>>();
        painted.sort_by(f32::total_cmp);
        laid_out.sort_by(f32::total_cmp);
        assert_eq!(painted, laid_out, "{standard:?}");
        let text = lettered_text(&shapes);
        assert_eq!(
            text,
            layout
                .lettering(&fields)
                .into_iter()
                .map(|line| line.text)
                .collect::<Vec<_>>(),
            "{standard:?}"
        );
        for line in ["SHEET", "1 OF 3", "SCALE", "NONE", "RS-0042"] {
            assert!(
                text.iter().any(|text| text == line),
                "{standard:?} {text:?}"
            );
        }
    }
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
            sheet_name: "top".to_owned(),
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
    let sheet = test_sheet();
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

/// Every grid dot lands on a lattice point exactly where content on that
/// point is placed: the lattice is phased on the viewport's own offset, the
/// one `schematic_to_screen` uses, not on a separately passed pan, so a pin
/// end on the grid sits on its dot at every pan.
#[test]
fn grid_dots_register_with_the_content_lattice() {
    let ctx = egui::Context::default();
    let _ = ctx.run_ui(egui::RawInput::default(), |_| {});
    let bounds = Rect::from_min_size(egui::pos2(3.25, 7.5), egui::vec2(400.0, 300.0));
    let viewport = Viewport {
        // World 200..600 by 100..400: inside the A4 drawing area.
        offset: egui::pos2(-197.0, -103.0),
        zoom: 1.0,
        bounds,
    };
    let painter = Painter::new(
        ctx,
        egui::LayerId::new(egui::Order::Background, egui::Id::new("grid-phase")),
        bounds,
    );
    draw_base(
        &painter,
        bounds,
        &viewport,
        DrawingSheetCanvas {
            layers: DrawingSheetLayerVisibility::default(),
            grid: GridStyle::Dots,
            grid_size: 10,
        },
        &test_sheet(),
        Vec::new,
    );
    let grid_ink = rspice_ui_kit::tokens::active_palette().canvas_grid;
    let origin = viewport.schematic_to_screen(Point::new(0, 0));
    let mut dots = 0;
    painter.for_each_shape(|shape| {
        let egui::Shape::Mesh(mesh) = &shape.shape else {
            return;
        };
        for quad in mesh.vertices.chunks(4) {
            if quad.len() < 4 || quad[0].color != grid_ink {
                continue;
            }
            let centre = quad
                .iter()
                .fold(egui::Vec2::ZERO, |sum, vertex| sum + vertex.pos.to_vec2())
                / 4.0;
            let steps = (centre - origin.to_vec2()) / 10.0;
            assert!(
                (steps.x - steps.x.round()).abs() < 1e-3
                    && (steps.y - steps.y.round()).abs() < 1e-3,
                "a dot at {centre:?} is off the lattice through {origin:?}"
            );
            dots += 1;
        }
    });
    assert!(dots > 100, "the grid painted {dots} dots");
}
