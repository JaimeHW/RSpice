//! The drawing-sheet preview over a whole application.

use crate::state::{
    DesignNote, DesignNoteKind, DocumentationShape, DocumentationShapeGeometry,
    DrawingSheetFrameLayer, DrawingSheetFrameMark, Junction, NetLabel, Point, SchematicProbe,
    SchematicSheetFormat,
};
use crate::workbench::app::dialogs::drawing_sheet_preview::*;
use crate::workbench::app_state::AppState;
use egui::{Pos2, Rect, Shape, vec2};
use rspice_schematic_editor::view::drawing_sheet::DrawingSheetPrintablePreviewKind;

#[test]
fn the_preview_letters_the_zones_the_sheet_model_letters() {
    // The preview keeps no alphabet of its own: it letters the
    // references the canvas and the plot letter, from the sheet model.
    let format = SchematicSheetFormat::from_standard(
        crate::state::DrawingSheetStandard::IsoA0,
        crate::state::SchematicPageOrientation::Landscape,
    );
    let geometry = format.geometry().unwrap();
    let expected = format
        .frame_artwork(&geometry)
        .layer(DrawingSheetFrameLayer::ZoneLabels)
        .filter_map(|mark| match mark {
            DrawingSheetFrameMark::Text { text, .. } => Some(text.clone()),
            _ => None,
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert!(expected.contains("Q") && expected.contains("R") && expected.contains("24"));
    // Wide enough that the 5 mm band carries legible references.
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(6_000.0, 4_500.0))),
        ..Default::default()
    };
    let output = egui::Context::default().run_ui(input, |ui| {
        drawing_sheet_preview(ui, &format, 4_000.0, "");
    });
    let lettered = output
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            Shape::Text(text) => Some(text.galley.text().to_owned()),
            _ => None,
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert!(
        expected.is_subset(&lettered),
        "the preview letters {lettered:?}, not the model's {expected:?}"
    );
}

#[test]
fn preview_covers_every_hardcopy_printable_class_but_not_probe_flags() {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .junctions
        .push(Junction::new(51, Point::new(1_100, 100)));
    state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(52, Point::new(1_100, 140), "OUTSIDE"));
    state
        .model
        .schematic
        .document_mut_for_test()
        .design_notes
        .push(
            DesignNote::new(
                53,
                Point::new(1_100, 180),
                DesignNoteKind::PlainText,
                "outside note",
            )
            .expect("valid note"),
        );
    state
        .model
        .schematic
        .document_mut_for_test()
        .documentation_shapes
        .push(
            DocumentationShape::new(
                54,
                DocumentationShapeGeometry::Line {
                    start: Point::new(1_100, 220),
                    end: Point::new(1_140, 240),
                },
            )
            .expect("valid documentation line"),
        );
    state.model.schematic.document_mut_for_test().probes.push(
        SchematicProbe::new(
            55,
            Point::new(1_100, 260),
            "V(out)",
            Some("V(out)".to_owned()),
        )
        .expect("valid probe"),
    );

    let content = DrawingSheetPreviewContent::from_model(
        &state.model,
        state.design.schematic_visibility.review_markers,
    );
    let drawing = SchematicSheetFormat::default()
        .geometry()
        .expect("default sheet geometry")
        .drawing_area;

    assert_eq!(content.objects.len(), 4);
    assert_eq!(content.outside_drawing_count(drawing), 4);
    assert_eq!(
        content
            .objects
            .iter()
            .map(|object| object.kind)
            .collect::<Vec<_>>(),
        vec![
            DrawingSheetPrintablePreviewKind::Junction,
            DrawingSheetPrintablePreviewKind::NetLabel,
            DrawingSheetPrintablePreviewKind::DesignNote,
            DrawingSheetPrintablePreviewKind::DocumentationShape,
        ]
    );
}
