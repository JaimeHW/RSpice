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

/// A deck's run was drawn from no schematic, so its operating point never
/// says the schematic changed since it ran, whatever the schematic in front.
#[test]
fn a_decks_operating_point_says_nothing_of_a_changed_schematic() {
    let run = rspice_app_runs::simulation::controller::test_execution::run_manual_batch(
        "* divider
V1 in 0 1
R1 in out 1k
R2 out 0 1k
.op
.end
",
    );
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state.model.simulation.retained.runs.push(run);
    let index = app.state.model.simulation.retained.runs.len() - 1;
    assert!(app.state.model.simulation.select_run(index));

    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut words = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::Panel::right("inspector").show(ctx, |ui| {
                rspice_app_results::workbench::documents::result_document::op_inspector::right_panel(
                    ui,
                    &mut app.state.results_parts(),
                );
            });
        });
        words = output
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
                _ => None,
            })
            .collect::<Vec<_>>();
    }
    assert!(
        words.iter().any(|words| words == "Conditions"),
        "the pane shows the run's operating point: {words:?}"
    );
    assert!(
        !words.iter().any(|words| words.contains("cross-probing")),
        "a deck's run says the schematic changed: {words:?}"
    );
}
