//! The sheets feature's commands on their own: what each answers and does
//! over the feature's state, the model and the session. What the shell's
//! prologue adds, and how the commands are listed and bound, is tested with
//! the shell's commands.

use super::*;
use crate::state::{SheetDefinition, SheetPortPolicy, SheetTemplate};
use crate::workbench::app_state::AppState;
use crate::workbench::framework::feature::harness::FeatureHarness;

const SHEET_COMMANDS: [SheetsCommand; 4] = [
    SheetsCommand::NextSheet,
    SheetsCommand::PreviousSheet,
    SheetsCommand::NewSheet,
    SheetsCommand::DeleteSheet,
];

/// The sheets feature over `state`'s model and session.
fn sheets_over(state: AppState) -> FeatureHarness<SheetsFeature> {
    FeatureHarness::with_model(state.model, *state.workbench)
}

/// A fresh session with a project open.
fn project_open() -> AppState {
    let mut state = AppState::default();
    state.open_project_session();
    state
}

/// The sheets feature on the design workspace, its drawing `count` sheets
/// long with the first one active.
fn sheets_on_a_drawing_of(count: usize) -> FeatureHarness<SheetsFeature> {
    let mut state = project_open();
    state.workbench.activate(Workspace::Design);
    let key = state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let first = state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Sheet 1", [10])
        .expect("first sheet");
    let catalog = state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .expect("sheet catalog");
    for ordinal in 2..=count {
        catalog
            .create_sheet(
                SheetDefinition {
                    name: format!("Sheet {ordinal}"),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(ordinal as u32),
                },
                None,
            )
            .expect("further sheet");
    }
    catalog.set_active(first).expect("active sheet");
    sheets_over(state)
}

/// The sheets feature on the design workspace, its drawing given a sheet
/// catalog of one sheet.
fn sheets_on_a_catalogued_drawing(read_only: bool) -> FeatureHarness<SheetsFeature> {
    let mut state = project_open();
    state.workbench.workspace = Workspace::Design;
    let key = state.model.workspace.content.active_key();
    state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Main", [])
        .unwrap();
    state.model.schematic.session.read_only = read_only;
    sheets_over(state)
}

#[test]
fn navigation_needs_a_second_sheet_and_authoring_needs_a_catalog() {
    let mut sheets = sheets_on_a_drawing_of(1);
    assert!(
        !sheets
            .availability(SheetsCommand::NextSheet.key())
            .is_available()
    );
    assert!(
        !sheets
            .availability(SheetsCommand::PreviousSheet.key())
            .is_available()
    );
    assert!(
        !sheets
            .availability(SheetsCommand::DeleteSheet.key())
            .is_available()
    );
    assert!(
        sheets
            .availability(SheetsCommand::NewSheet.key())
            .is_available()
    );

    sheets = sheets_on_a_drawing_of(2);
    for command in SHEET_COMMANDS {
        assert_eq!(
            sheets.availability(command.key()),
            CommandAvailability::Available,
            "{command:?}"
        );
    }
}

#[test]
fn paging_commands_move_the_active_sheet_and_wrap() {
    let mut sheets = sheets_on_a_drawing_of(3);
    let order = sheets
        .model
        .active_sheet_catalog()
        .map(|catalog| {
            catalog
                .sheets()
                .iter()
                .map(|sheet| sheet.id())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    assert_eq!(sheets.model.active_sheet_id(), Some(order[0]));

    sheets.execute(SheetsCommand::NextSheet.key());
    assert_eq!(sheets.model.active_sheet_id(), Some(order[1]));
    sheets.execute(SheetsCommand::PreviousSheet.key());
    sheets.execute(SheetsCommand::PreviousSheet.key());
    assert_eq!(
        sheets.model.active_sheet_id(),
        Some(order[2]),
        "paging wraps at the ends"
    );
}

#[test]
fn creating_and_deleting_sheets_run_their_real_transactions() {
    let mut sheets = sheets_on_a_drawing_of(2);
    sheets.execute(SheetsCommand::NewSheet.key());
    assert_eq!(sheet_count(&sheets.model), 3);

    let created = sheets
        .model
        .active_sheet_id()
        .expect("the new sheet is entered");
    sheets.execute(SheetsCommand::DeleteSheet.key());
    assert_eq!(sheet_count(&sheets.model), 2);
    assert_ne!(sheets.model.active_sheet_id(), Some(created));
}

/// Sheet properties show the sheet where it is edited: the selection is
/// cleared, so the inspector is the sheet's; the inspector opens where it was
/// closed; and the sheet panel is asked to bring its Sheet section into view.
#[test]
fn sheet_properties_open_the_inspector_on_the_sheet() {
    let mut state = project_open();
    rspice_app_state::workbench::examples::load_example(
        "RC Lowpass Filter",
        &mut state.model.schematic,
    );
    state.select_all_schematic_objects();
    state.workbench.inspector_visible = false;
    let mut sheets = sheets_over(state);
    assert!(
        sheets
            .availability(SheetsCommand::SheetProperties.key())
            .is_available()
    );

    sheets.execute(SheetsCommand::SheetProperties.key());

    assert!(sheets.model.schematic.session.editor.selection.is_empty());
    assert!(sheets.wb.inspector_visible);
    assert!(sheets.state.reveal_sheet_section);
}

#[test]
fn document_sheet_commands_open_their_real_surfaces_only_in_schematic_context() {
    let mut sheets = sheets_on_a_catalogued_drawing(false);

    assert!(
        sheets
            .availability(SheetsCommand::SheetFormatManager.key())
            .is_available()
    );
    assert!(
        sheets
            .availability(SheetsCommand::CustomSheetSizes.key())
            .is_available()
    );

    sheets.execute(SheetsCommand::SheetFormatManager.key());
    assert!(sheets.state.drawing_sheet_support.manager.open);

    sheets.execute(SheetsCommand::CustomSheetSizes.key());
    assert!(sheets.state.drawing_sheet_presets.any_open());

    // The project default is where a format reaches several sheets, so its
    // dialog is a command of its own.
    assert!(
        sheets
            .availability(SheetsCommand::SheetDefaults.key())
            .is_available()
    );
    sheets.execute(SheetsCommand::SheetDefaults.key());
    assert!(sheets.state.drawing_sheet_defaults.open);

    let mut results = project_open();
    results.workbench.workspace = Workspace::Results;
    let results = sheets_over(results);
    assert!(
        !results
            .availability(SheetsCommand::SheetProperties.key())
            .is_available()
    );
    assert!(
        !results
            .availability(SheetsCommand::SheetFormatManager.key())
            .is_available()
    );
    assert!(
        !results
            .availability(SheetsCommand::CustomSheetSizes.key())
            .is_available()
    );
    assert!(
        !results
            .availability(SheetsCommand::SheetDefaults.key())
            .is_available()
    );
}

#[test]
fn sheet_format_manager_requires_live_schematic_edit_authority() {
    let mut sheets = sheets_on_a_catalogued_drawing(true);

    assert_eq!(
        sheets.availability(SheetsCommand::SheetFormatManager.key()),
        CommandAvailability::Disabled("the active schematic is read-only")
    );
    sheets.execute(SheetsCommand::SheetFormatManager.key());
    assert!(!sheets.state.drawing_sheet_support.manager.open);
}
