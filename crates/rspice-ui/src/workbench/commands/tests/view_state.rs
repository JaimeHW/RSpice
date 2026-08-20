//! Commands that change what a workspace shows, and the ones that reset it.
//!
//! None of these edits a document. Cycling the grid is display state and must
//! not rewrite the snap configuration behind it; a reset captures the exact
//! workspace it will restore and puts that up for review before the host
//! window or the dock layout moves.

use super::*;

#[test]
fn canvas_grid_command_cycles_display_without_mutating_snap_configuration() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    app.state.ui.set_grid_style(crate::state::GridStyle::Dots);
    app.state.schematic.snap_engine.enabled = true;
    app.state.schematic.snap_engine.snap_to_wire_segments = false;
    app.state.ui.schematic_snap = app.state.schematic.snap_engine.clone();

    assert_eq!(Command::CycleGrid.stable_id(), "cycle-grid");
    assert_eq!(Command::CycleGrid.spec().label, "Cycle grid display");

    Command::CycleGrid.execute(&mut app);
    assert_eq!(app.state.ui.grid, crate::state::GridStyle::Lines);
    assert!(app.state.schematic.snap_engine.enabled);
    assert!(app.state.ui.schematic_snap.enabled);
    assert!(
        !app.state.schematic.snap_engine.snap_to_wire_segments,
        "display cycling must preserve detailed snap target choices"
    );

    Command::CycleGrid.execute(&mut app);
    assert_eq!(app.state.ui.grid, crate::state::GridStyle::Off);
    assert!(app.state.schematic.snap_engine.enabled);

    Command::CycleGrid.execute(&mut app);
    assert_eq!(app.state.ui.grid, crate::state::GridStyle::Dots);
}

#[test]
fn canvas_grid_command_does_not_reinterpret_symbol_snap_policy() {
    let mut app = app_with_selected_authored_symbol();
    app.state
        .open_workspace_view(crate::state::CellViewRef::new(
            "command_test",
            "amp",
            "symbol",
        ));
    assert!(active_symbol_editor(&app));
    app.state.ui.symbol.show_grid = true;
    app.state.ui.symbol.snap_to_grid = false;

    Command::CycleGrid.execute(&mut app);
    assert!(!app.state.ui.symbol.show_grid);
    assert!(!app.state.ui.symbol.snap_to_grid);

    Command::CycleGrid.execute(&mut app);
    assert!(app.state.ui.symbol.show_grid);
    assert!(!app.state.ui.symbol.snap_to_grid);
}

#[test]
fn schematic_zoom_commands_match_the_mockup_bounds_and_request_a_real_fit() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);

    app.state.schematic.zoom = 1.0;
    Command::ZoomIn.execute(&mut app);
    assert!((app.state.schematic.zoom - 1.2).abs() < f64::EPSILON);

    app.state.schematic.zoom = 1.0;
    app.execute_shortcut_command(Command::ZoomOut);
    assert!((app.state.schematic.zoom - (1.0 / 1.2)).abs() < f64::EPSILON);

    app.state.schematic.zoom = 0.251;
    Command::ZoomOut.execute(&mut app);
    assert_eq!(app.state.schematic.zoom, 0.25);

    app.state.schematic.zoom = 7.99;
    Command::ZoomIn.execute(&mut app);
    assert_eq!(app.state.schematic.zoom, 8.0);

    app.state.schematic.zoom = 3.5;
    app.state.schematic.pan = (127.0, -81.0);
    app.state.schematic.needs_fit = false;
    app.state.schematic.needs_drawing_sheet_fit = false;
    Command::ZoomFit.execute(&mut app);
    assert_eq!(app.state.schematic.zoom, 3.5);
    assert_eq!(app.state.schematic.pan, (127.0, -81.0));
    assert!(!app.state.schematic.needs_fit);
    assert!(app.state.schematic.needs_drawing_sheet_fit);

    Command::FitSchematicContent.execute(&mut app);
    assert!(app.state.schematic.needs_fit);
    assert!(!app.state.schematic.needs_drawing_sheet_fit);
}

#[test]
fn only_exactly_implemented_reset_actions_are_discoverable() {
    let searchable = command_catalog().collect::<Vec<_>>();
    assert!(vocabulary::COMMAND_REGISTRY.contains(&Command::ResetActiveView));
    assert!(searchable.contains(&Command::ResetActiveView));
    assert!(vocabulary::COMMAND_REGISTRY.contains(&Command::ResetLayout));
    assert!(searchable.contains(&Command::ResetLayout));
    for command in [Command::PreviousWorkspace, Command::NextWorkspace] {
        assert!(!vocabulary::COMMAND_REGISTRY.contains(&command));
        assert!(!searchable.contains(&command));
    }
}

#[test]
fn every_workspace_exposes_the_mockup_reset_active_view_workflow() {
    for workspace in Workspace::ALL {
        assert!(
            reset_active_view_available(workspace),
            "{workspace:?} has implemented reset behavior"
        );
    }
}

#[test]
fn full_screen_command_opens_review_before_host_or_layout_mutation() {
    let mut app = RSpiceApp::test_instance();
    assert!(!app.state.workbench.full_screen_presentation);
    assert_eq!(app.state.ui.take_full_screen_request(), None);

    Command::ToggleFullScreen.execute(&mut app);

    assert!(app.state.dialogs.view_operation.open);
    assert_eq!(
        app.state.dialogs.view_operation.operation,
        crate::workbench::app::ViewOperation::FullScreen
    );
    assert!(app.state.dialogs.application_modal_open());
    assert!(!app.state.workbench.full_screen_presentation);
    assert_eq!(app.state.ui.take_full_screen_request(), None);
}

#[test]
fn reset_active_view_command_captures_the_exact_workspace_for_review() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Results);

    Command::ResetActiveView.execute(&mut app);

    assert!(app.state.dialogs.view_operation.open);
    assert_eq!(
        app.state.dialogs.view_operation.operation,
        crate::workbench::app::ViewOperation::ResetActiveView
    );
    assert_eq!(
        app.state.dialogs.view_operation.workspace,
        Workspace::Results
    );
    assert!(app.state.dialogs.application_modal_open());
}

#[test]
fn resetting_the_simulation_view_clears_the_narrowing_and_not_the_plan() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Simulate);
    app.state.workbench.saved_output_filter = "db20".to_owned();
    app.state.workbench.specification_filter = "bw(".to_owned();
    app.state.workbench.specification_evidence_filter =
        crate::workbench::state::SpecificationEvidenceFilter::Failing;
    app.state.workbench.selected_specification = Some("bw(vout)".to_owned());

    super::reset_active_view(&mut app);

    assert!(app.state.workbench.saved_output_filter.is_empty());
    assert!(app.state.workbench.specification_filter.is_empty());
    assert_eq!(
        app.state.workbench.specification_evidence_filter,
        crate::workbench::state::SpecificationEvidenceFilter::All
    );
    // A view reset restores what the reader can see, not what the plan says
    // or which record they were reading.
    assert_eq!(
        app.state.workbench.selected_specification.as_deref(),
        Some("bw(vout)")
    );
}

#[test]
fn resetting_one_view_leaves_every_other_workspace_navigator_filter_alone() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    app.state.workbench.set_navigator_filter("vout");
    app.state.workbench.activate(Workspace::Models);
    app.state.workbench.set_navigator_filter("nch");

    super::reset_active_view(&mut app);

    assert!(app.state.workbench.navigator_filter().is_empty());
    app.state.workbench.activate(Workspace::Design);
    assert_eq!(app.state.workbench.navigator_filter(), "vout");
}
