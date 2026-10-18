//! A plotted sheet frame is the standard's: its lines at the standard's
//! widths, its marks and its size designation where the standard puts them.

use super::*;
use rspice_design_model::design_management::DrawingSheetFrameLayer;

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
