//! The operating-point inspector's refusals over a whole application.

use rspice_results_ui::op_inspector::OpAction;

use crate::workbench::AppState;
use rspice_app_results::workbench::documents::result_document::op_inspector::apply_action;

/// A locate that cannot be made is a refused gesture: the status bar
/// says why, and nothing is toasted.
#[test]
fn a_device_that_cannot_be_located_is_answered_on_the_status_bar() {
    let mut state = AppState::default();
    apply_action(&mut state.results_parts(), OpAction::LocateDevice(u64::MAX));
    state.drain_notices(&egui::Context::default());
    assert_eq!(
        state.prompt_text(),
        Some("This device no longer matches a part on the schematic.")
    );
    assert!(state.workbench.toasts.activity().is_empty());
}

/// A double-click the schematic cannot answer gets its reason on the same
/// line, and leaves the workspace where it was.
#[test]
fn a_refused_cross_probe_is_answered_on_the_status_bar() {
    let mut state = AppState::default();
    let workspace = state.workbench.workspace;
    apply_action(
        &mut state.results_parts(),
        OpAction::Refused("This retained node has no schematic mapping."),
    );
    state.drain_notices(&egui::Context::default());
    assert_eq!(
        state.prompt_text(),
        Some("This retained node has no schematic mapping.")
    );
    assert_eq!(state.workbench.workspace, workspace);
    assert!(state.workbench.toasts.activity().is_empty());
}

/// Locating a device placed on a sheet that is not in front brings that
/// sheet forward and selects the device there. It used to pan the sheet in
/// front to where the device sits on its own sheet, and the canvas then
/// dropped the selection it could not draw.
#[test]
fn locating_a_device_on_another_sheet_brings_its_sheet_forward() {
    use rspice_app_state::state::{Component, ComponentType, Point};

    let mut app = crate::workbench::RSpiceApp::test_instance();
    let mut device = Component::new(7, ComponentType::Resistor, Point::new(300, 140));
    device.name = "R7".to_owned();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(device);
    rspice_app_design::schematic::view::sheet_visibility::seed_two_sheets(
        &mut app.state.model,
        &[],
        &[7],
    );
    let first = app.state.model.active_sheet_id();

    apply_action(&mut app.state.results_parts(), OpAction::LocateDevice(7));
    app.after_contribution();

    let model = &mut app.state.model;
    assert_ne!(model.active_sheet_id(), first);
    assert_eq!(
        model.sheet_name(model.active_sheet_id().expect("a sheet is active")),
        "Sheet 2"
    );
    model.retain_selection_on_active_sheet();
    assert!(model.schematic.session.editor.selection.has_component(7));
    assert_eq!(
        model.schematic.session.editor.center_request,
        Some(Point::new(300, 140))
    );
    assert_eq!(
        app.state.workbench.workspace,
        rspice_app_state::workbench::workspace::Workspace::Design
    );
}
