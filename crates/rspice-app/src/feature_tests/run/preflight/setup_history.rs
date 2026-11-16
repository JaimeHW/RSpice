//! Undo and Redo on the Simulate setup pages: each page's committed edit is
//! one step of the setup's own history, put back exactly, and Undo follows
//! the focus: on a Simulate page it takes back the setup's last edit, on the
//! schematic the schematic's, whichever was made last.

use super::engine::app_with_a_moded_device;
use super::setup_page::{Page, stored_options};
use super::*;
use rspice_app_state::state::SimulationPage;
use rspice_app_workbench::workbench::framework::command_registry::ids;

/// What the first part on the sheet holds, as the schematic's own history
/// restores it.
fn first_value(app: &RSpiceApp) -> String {
    app.state.model.schematic.document().components[0]
        .value
        .clone()
}

/// A Solver edit, then a newer schematic edit. On the Solver page Undo takes
/// back the Solver edit and leaves the schematic's standing, and with no
/// setup step left it reaches no further; on the schematic Undo takes back
/// the schematic's edit and leaves the setup alone. Redo follows the focus
/// the same way.
#[test]
fn undo_takes_back_the_edit_of_the_place_it_is_pressed_in() {
    let mut page = Page::open_on(app_with_a_moded_device("").0, SimulationPage::Solver);
    let setup_before = stored_options(&page.app);
    page.type_slowly("relative update", "2e-3");
    let setup_after = stored_options(&page.app);
    assert_ne!(setup_before, setup_after);

    page.app.state.workbench.activate(Workspace::Design);
    let drawn = first_value(&page.app);
    page.app
        .state
        .model
        .schematic
        .with_undo("change value", |schematic| {
            schematic.document_mut_for_test().components[0].value = "7".to_owned();
        });
    assert_eq!(first_value(&page.app), "7");

    page.app.state.workbench.activate(Workspace::Simulate);
    page.app.execute_command_key(ids::UNDO);
    assert_eq!(
        stored_options(&page.app),
        setup_before,
        "the setup's edit is undone"
    );
    assert_eq!(
        first_value(&page.app),
        "7",
        "the newer schematic edit stands"
    );
    page.app.execute_command_key(ids::UNDO);
    assert_eq!(
        first_value(&page.app),
        "7",
        "a Simulate page never reaches past the setup into the design"
    );

    page.app.state.workbench.activate(Workspace::Design);
    page.app.execute_command_key(ids::UNDO);
    assert_eq!(
        first_value(&page.app),
        drawn,
        "the schematic's edit is undone"
    );
    assert_eq!(
        stored_options(&page.app),
        setup_before,
        "the setup is left as it is"
    );

    page.app.execute_command_key(ids::REDO);
    assert_eq!(first_value(&page.app), "7");
    assert_eq!(stored_options(&page.app), setup_before);
    page.app.state.workbench.activate(Workspace::Simulate);
    page.app.execute_command_key(ids::REDO);
    assert_eq!(stored_options(&page.app), setup_after);
    assert_eq!(first_value(&page.app), "7");
}

/// The Solver page commits through the same plan transaction, so its edits
/// are steps of the same history: a value typed a keystroke at a time is one
/// step, Undo puts back exactly the options it replaced, and Redo the ones it
/// wrote.
#[test]
fn a_solver_value_typed_a_keystroke_at_a_time_is_one_step_of_undo() {
    let mut page = Page::open_on(app_with_a_moded_device("").0, SimulationPage::Solver);
    let before = stored_options(&page.app);
    page.type_slowly("relative update", "2e-3");
    assert_eq!(page.app.state.model.sim_setup.options.reltol, 2e-3);
    let after = stored_options(&page.app);

    page.app.execute_command_key(ids::UNDO);
    assert_eq!(
        stored_options(&page.app),
        before,
        "one Undo takes the whole value back, not its last keystroke"
    );
    page.pass(Vec::new());
    assert_ne!(
        page.control("relative update").value.as_deref(),
        Some("2e-3"),
        "the page shows what the step restored"
    );
    page.app.execute_command_key(ids::REDO);
    assert_eq!(stored_options(&page.app), after);
}
