//! A plotted sheet frame is the standard's: its lines at the standard's
//! widths, its marks and its size designation where the standard puts them.

use super::*;
use rspice_design_model::design_management::{DrawingSheetFrameLayer, DrawingSheetRect};

/// Paper, border and zones, with crop marks as asked.
fn frame_output(crop_marks: bool) -> SchematicHardcopySetup {
    SchematicHardcopySetup::new(
        SchematicHardcopyExtent::AuthoredDrawingSheet,
        OutsideSheetContentPolicy::Ask,
        crop_marks,
        true,
        true,
        false,
        true,
        false,
    )
}

fn plotted(format: &SchematicSheetFormat, output: SchematicHardcopySetup) -> Vec<ScenePrimitive> {
    let source = resolved_blank_schematic(format);
    scene_from_resolved(
        &source,
        source.default_print_mapping(),
        output,
        resolved_metadata(),
    )
    .unwrap()
    .primitives()
    .to_vec()
}

fn stroke_widths(primitives: &[ScenePrimitive]) -> Vec<u64> {
    primitives
        .iter()
        .filter_map(|primitive| match primitive {
            ScenePrimitive::Line { stroke, .. } => Some(stroke.width.micrometres()),
            ScenePrimitive::Rect {
                stroke: Some(stroke),
                ..
            } => Some(stroke.width.micrometres()),
            _ => None,
        })
        .collect()
}

#[test]
fn an_iso_frame_plots_at_iso_5457_line_widths() {
    let a3 = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::IsoA3,
        SchematicPageOrientation::Landscape,
    );
    let primitives = plotted(&a3, frame_output(false));
    let widths = stroke_widths(&primitives);
    // 0.18 mm paper edge; 0.7 mm frame and centring marks; 0.35 mm grid
    // reference border and grid lines. Nothing else, and no dashed margin
    // rectangle.
    assert!(widths.contains(&700) && widths.contains(&350));
    assert!(
        widths.iter().all(|width| [180, 350, 700].contains(width)),
        "{widths:?}"
    );
    assert!(primitives.iter().all(|primitive| !matches!(
        primitive,
        ScenePrimitive::Rect { stroke: Some(stroke), .. } if stroke.pattern == StrokePattern::Dashed
    )));
    // Four 0.7 mm centring marks on the axes: two vertical at x = 210 mm and
    // two horizontal at y = 148.5 mm, in sheet coordinates.
    let geometry = a3.geometry().unwrap();
    let centring = a3
        .frame_artwork(&geometry)
        .layer(DrawingSheetFrameLayer::CentringMarks)
        .count();
    assert_eq!(centring, 4);
    let heavy_lines = primitives
        .iter()
        .filter(|primitive| {
            matches!(primitive, ScenePrimitive::Line { stroke, .. } if stroke.width.micrometres() == 700)
        })
        .count();
    assert_eq!(heavy_lines, 4, "four centring marks and no corner crosses");
}

#[test]
fn an_asme_border_plots_at_asme_y14_1_line_widths() {
    let ansi_d = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::AnsiD,
        SchematicPageOrientation::Landscape,
    );
    let widths = stroke_widths(&plotted(&ansi_d, frame_output(false)));
    // A 0.6 mm border line and 0.3 mm zone markers.
    assert!(widths.contains(&600) && widths.contains(&300));
    assert!(
        widths.iter().all(|width| [180, 300, 600].contains(width)),
        "{widths:?}"
    );
}

#[test]
fn the_size_designation_is_plotted_in_the_bottom_border() {
    let a4 = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::IsoA4,
        SchematicPageOrientation::Portrait,
    );
    let primitives = plotted(&a4, frame_output(false));
    let designation = primitives
        .iter()
        .find_map(|primitive| match primitive {
            ScenePrimitive::Text { origin, text, .. } if text == "A4" => Some(*origin),
            _ => None,
        })
        .expect("A4 is lettered on the sheet");
    // The sheet compiles at the authored paper's origin, so a baseline below
    // the 292 mm grid reference border is in the bottom border.
    let (paper_width_um, paper_height_um) = a4.oriented_dimensions_um();
    let baseline_um = designation.y.micrometres();
    assert!(baseline_um > 292_000 && baseline_um < paper_height_um);
    assert!(designation.x.micrometres() > paper_width_um / 2);
}

#[test]
fn iso_trimming_marks_are_filled_rectangles_and_only_when_asked() {
    let a4 = SchematicSheetFormat::default();
    let filled = |primitives: &[ScenePrimitive]| {
        primitives
            .iter()
            .filter(|primitive| {
                matches!(
                    primitive,
                    ScenePrimitive::Rect {
                        stroke: None,
                        fill: Some(_),
                        ..
                    }
                )
            })
            .count()
    };
    assert_eq!(filled(&plotted(&a4, frame_output(true))), 8);
    assert_eq!(filled(&plotted(&a4, frame_output(false))), 0);
}

/// The form prints black: an A3 sheet plotted 1:1 onto A3 has full black
/// ink on its frame line and on its title block's outline, where both once
/// printed 60 % grey (#666666).
#[test]
fn the_frame_and_title_block_print_black() {
    let a3 = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::IsoA3,
        SchematicPageOrientation::Landscape,
    );
    let source = resolved_blank_schematic(&a3);
    let dpi = 150;
    let base = setup(OutputFormat::Png { dpi }, false);
    let setup = HardcopySetup::try_new_with_schematic(
        PhysicalPageSetup::try_new(
            PaperSize::Standard(StandardPaper::A3),
            PageMargins::uniform(Length::ZERO),
            Bleed::None,
            Orientation::Landscape,
        )
        .unwrap(),
        ScaleMode::EngineeringOneToOne,
        base.tiling(),
        base.render().clone(),
        DecorationSetup::try_new(false, false, false, Watermark::None).unwrap(),
        SchematicHardcopySetup::new(
            SchematicHardcopyExtent::AuthoredDrawingSheet,
            OutsideSheetContentPolicy::Ask,
            false,
            false,
            true,
            true,
            true,
            false,
        ),
        source.default_print_mapping().clone(),
    )
    .unwrap();
    let plan =
        HardcopyPlan::compile(source.authority().clone(), setup, source.content_extent()).unwrap();
    assert_eq!(plan.pagination().pages().len(), 1);
    let png = HardcopyRenderer::render_resolved(&plan, &source, resolved_metadata()).unwrap();
    let decoder = png::Decoder::new(Cursor::new(png.single_part().unwrap().bytes()));
    let mut reader = decoder.read_info().unwrap();
    let mut rgba = vec![0; reader.output_buffer_size().unwrap()];
    let info = reader.next_frame(&mut rgba).unwrap();
    let width = usize::try_from(info.width).unwrap();
    let pixel = |x_um: i64, y_um: i64| {
        let to_px = |um: i64| usize::try_from(um).unwrap() * usize::from(dpi) / 25_400;
        let at = (to_px(y_um) * width + to_px(x_um)) * 4;
        [rgba[at], rgba[at + 1], rgba[at + 2]]
    };
    let geometry = a3.geometry().unwrap();
    let frame = geometry.drawing_area;
    let block = geometry.title_block.expect("an ISO 7200 block");
    let height = |rect: DrawingSheetRect| i64::try_from(rect.height_um).unwrap();
    assert_eq!(
        pixel(frame.x_um, frame.y_um + height(frame) / 2),
        [0, 0, 0],
        "the frame line"
    );
    assert_eq!(
        pixel(block.x_um, block.y_um + height(block) / 2),
        [0, 0, 0],
        "the title block's outline"
    );
}
