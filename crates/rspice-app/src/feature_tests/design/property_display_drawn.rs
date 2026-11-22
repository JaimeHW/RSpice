//! A Property display note shows a title-block field of its sheet, drawn
//! through real frames: the canvas letters the field's value, and the new
//! value once the field changes.

use super::gestures_drawn::Fixture;
use rspice_app_sheets::workbench::app::drawing_sheet_edit::transaction::{
    DrawingSheetEdit, apply_drawing_sheet_edit_on,
};
use rspice_app_state::state::{DesignNote, DesignNoteKind, DrawingSheetTitleFieldId, Point};

/// Whether the frame painted `text`.
fn painted(output: &egui::FullOutput, text: &str) -> bool {
    fn find(shape: &egui::Shape, text: &str) -> bool {
        match shape {
            egui::Shape::Text(run) => run.galley.text() == text,
            egui::Shape::Vec(shapes) => shapes.iter().any(|shape| find(shape, text)),
            _ => false,
        }
    }
    output
        .shapes
        .iter()
        .any(|clipped| find(&clipped.shape, text))
}

#[test]
fn a_note_bound_to_the_sheet_title_shows_the_title_and_follows_it() {
    let mut fixture = Fixture::new(false);
    let note = DesignNote::new(
        70,
        Point::new(100, 150),
        DesignNoteKind::PropertyDisplay,
        "Title: ${TITLE}",
    )
    .expect("the sheet title is a field a note can show");
    fixture
        .app
        .state
        .model
        .schematic
        .document_mut_for_test()
        .design_notes
        .push(note);

    // Untitled, the block letters the design's name, and so does the note.
    let design = fixture
        .app
        .state
        .model
        .workspace
        .content
        .active_view
        .cell
        .clone();
    let output = fixture.frame_output(vec![], true);
    assert!(
        painted(&output, &format!("Title: {design}")),
        "the note letters the sheet's title"
    );
    assert!(!painted(&output, "Title: ${TITLE}"));

    let state = &mut fixture.app.state;
    apply_drawing_sheet_edit_on(
        &mut state.model,
        &state.workbench,
        DrawingSheetEdit::TitleField(DrawingSheetTitleFieldId::SheetTitle, "Power stage".into()),
    )
    .expect("the sheet takes the title");
    let output = fixture.frame_output(vec![], true);
    assert!(
        painted(&output, "Title: Power stage"),
        "the note letters the new title"
    );
}
