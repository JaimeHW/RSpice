//! A title-block field typed with the sheet's number and count letters each
//! sheet's own, as the canvas and print letter the block.

use super::apply_drawing_sheet_edit;
use crate::workbench::app_state::AppState;
use rspice_app_sheets::workbench::app::actions::sheets::{
    activate_sheet, new_sheet, sheet_entries,
};
use rspice_app_sheets::workbench::app::drawing_sheet_edit::DrawingSheetEdit;
use rspice_app_state::state::{Component, ComponentType, DrawingSheetTitleFieldId, Point};

/// The Title the open sheet's block letters.
fn lettered_title(state: &AppState) -> String {
    let sheet = state.model.active_drawing_sheet();
    state
        .model
        .title_block_fields(&sheet)
        .into_iter()
        .find(|field| field.id == DrawingSheetTitleFieldId::SheetTitle)
        .map(|field| field.value)
        .expect("the block letters a Title")
}

#[test]
fn a_title_typed_with_the_sheet_number_reads_each_sheets_own_number() {
    let mut state = AppState::default();
    state.model.schematic.document_mut_for_test().components = vec![Component::new(
        10,
        ComponentType::Resistor,
        Point::new(10, 10),
    )];
    new_sheet(&mut state.model, &mut state.workbench, &mut *state.design)
        .expect("the first new sheet creates the catalog");
    // A sheet's Title is its own, so each sheet is given the same text.
    let title = |state: &mut AppState| {
        apply_drawing_sheet_edit(
            state,
            DrawingSheetEdit::TitleField(
                DrawingSheetTitleFieldId::SheetTitle,
                "Sheet ${#} of ${##}".to_owned(),
            ),
        )
        .expect("the sheet takes the title");
    };
    title(&mut state);
    assert_eq!(lettered_title(&state), "Sheet 2 of 2", "the new sheet");

    let first = sheet_entries(&state)[0].id;
    activate_sheet(
        &mut state.model,
        &mut state.workbench,
        &mut *state.design,
        first,
    )
    .expect("activation publishes");
    title(&mut state);
    assert_eq!(lettered_title(&state), "Sheet 1 of 2", "the first sheet");
}
