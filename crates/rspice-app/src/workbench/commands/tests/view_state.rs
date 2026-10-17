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
    app.state.schematic.session.editor.snap_engine.enabled = true;
    app.state
        .schematic
        .session
        .editor
        .snap_engine
        .snap_to_wire_segments = false;
    app.state.ui.schematic_snap = app.state.schematic.session.editor.snap_engine.clone();

    assert_eq!(Command::CycleGrid.stable_id(), "cycle-grid");
    assert_eq!(Command::CycleGrid.spec().label, "Cycle grid display");

    Command::CycleGrid.execute(&mut app);
    assert_eq!(app.state.ui.grid, crate::state::GridStyle::Lines);
    assert!(app.state.schematic.session.editor.snap_engine.enabled);
    assert!(app.state.ui.schematic_snap.enabled);
    assert!(
        !app.state
            .schematic
            .session
            .editor
            .snap_engine
            .snap_to_wire_segments,
        "display cycling must preserve detailed snap target choices"
    );

    Command::CycleGrid.execute(&mut app);
    assert_eq!(app.state.ui.grid, crate::state::GridStyle::Off);
    assert!(app.state.schematic.session.editor.snap_engine.enabled);

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
    app.state.ui.symbol.editor.show_grid = true;
    app.state.ui.symbol.editor.snap_to_grid = false;

    Command::CycleGrid.execute(&mut app);
    assert!(!app.state.ui.symbol.editor.show_grid);
    assert!(!app.state.ui.symbol.editor.snap_to_grid);

    Command::CycleGrid.execute(&mut app);
    assert!(app.state.ui.symbol.editor.show_grid);
    assert!(!app.state.ui.symbol.editor.snap_to_grid);
}

#[test]
fn schematic_zoom_commands_match_the_mockup_bounds_and_request_a_real_fit() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    app.state.schematic.session.editor.needs_fit = false;
    app.state.schematic.session.editor.needs_drawing_sheet_fit = false;

    // The canvas carries a zoom command out on its next frame, pivoting on
    // its middle (audit E-1), so each step draws one.
    app.state.schematic.session.editor.zoom = 1.0;
    Command::ZoomIn.execute(&mut app);
    draw_schematic_canvas(&mut app.state, 800.0, 600.0);
    assert!((app.state.schematic.session.editor.zoom - 1.2).abs() < f64::EPSILON);

    app.state.schematic.session.editor.zoom = 1.0;
    app.execute_shortcut_command(Command::ZoomOut);
    draw_schematic_canvas(&mut app.state, 800.0, 600.0);
    assert!((app.state.schematic.session.editor.zoom - (1.0 / 1.2)).abs() < f64::EPSILON);

    app.state.schematic.session.editor.zoom = 0.251;
    Command::ZoomOut.execute(&mut app);
    draw_schematic_canvas(&mut app.state, 800.0, 600.0);
    assert_eq!(app.state.schematic.session.editor.zoom, 0.25);

    app.state.schematic.session.editor.zoom = 7.99;
    Command::ZoomIn.execute(&mut app);
    draw_schematic_canvas(&mut app.state, 800.0, 600.0);
    assert_eq!(app.state.schematic.session.editor.zoom, 8.0);

    app.state.schematic.session.editor.zoom = 3.5;
    app.state.schematic.session.editor.pan = (127.0, -81.0);
    app.state.schematic.session.editor.needs_fit = false;
    app.state.schematic.session.editor.needs_drawing_sheet_fit = false;
    Command::ZoomFit.execute(&mut app);
    assert_eq!(app.state.schematic.session.editor.zoom, 3.5);
    assert_eq!(app.state.schematic.session.editor.pan, (127.0, -81.0));
    assert!(!app.state.schematic.session.editor.needs_fit);
    assert!(app.state.schematic.session.editor.needs_drawing_sheet_fit);

    Command::FitSchematicContent.execute(&mut app);
    assert!(app.state.schematic.session.editor.needs_fit);
    assert!(!app.state.schematic.session.editor.needs_drawing_sheet_fit);
}

/// Draw the schematic canvas for one frame in a `width` x `height` panel, so
/// requests the commands leave for the canvas are carried out with its rect.
fn draw_schematic_canvas(state: &mut AppState, width: f32, height: f32) {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width, height),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    crate::schematic::view::render_schematic_view(ui, state, None);
                });
        },
    );
}

/// The world point under the middle of a `width` x `height` canvas.
fn world_at_canvas_centre(app: &RSpiceApp, width: f64, height: f64) -> (f64, f64) {
    let editor = &app.state.schematic.session.editor;
    (
        (width / 2.0 - editor.pan.0) / editor.zoom,
        (height / 2.0 - editor.pan.1) / editor.zoom,
    )
}

/// Zoom In, Zoom Out and Zoom 1:1 used to scale about the world origin, so a
/// sheet whose content sits far from (0, 0) slid off the screen one press at
/// a time. They pivot on the middle of the canvas, as the wheel pivots on
/// the pointer.
#[test]
fn zoom_commands_keep_the_middle_of_the_canvas_where_it_was() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    app.state.schematic.session.editor.needs_fit = false;
    app.state.schematic.session.editor.needs_drawing_sheet_fit = false;
    app.state.schematic.session.editor.zoom = 0.5;
    app.state.schematic.session.editor.pan = (400.0 - 2000.0 * 0.5, 300.0 - 100.0 * 0.5);
    draw_schematic_canvas(&mut app.state, 800.0, 600.0);
    assert_eq!(world_at_canvas_centre(&app, 800.0, 600.0), (2000.0, 100.0));

    for (command, zoom) in [
        (Command::ZoomIn, 0.6),
        (Command::ZoomIn, 0.72),
        (Command::ZoomOut, 0.6),
        (Command::ZoomOneToOne, 1.0),
    ] {
        app.execute_shortcut_command(command);
        draw_schematic_canvas(&mut app.state, 800.0, 600.0);
        let editor = &app.state.schematic.session.editor;
        assert!(
            (editor.zoom - zoom).abs() < 1e-9,
            "{command:?}: zoom {}",
            editor.zoom
        );
        let (x, y) = world_at_canvas_centre(&app, 800.0, 600.0);
        assert!(
            (x - 2000.0).abs() < 1e-6 && (y - 100.0).abs() < 1e-6,
            "{command:?} moved the middle of the canvas to ({x}, {y})"
        );
    }
}

/// The 25 % floor could not show a whole sheet on a small canvas: Fit Sheet
/// stopped short and cropped the paper. The floor is now the lower of 25 %
/// and the zoom that fits the sheet, so Fit Sheet shows all of it and Zoom
/// Out reaches the same framing.
#[test]
fn fit_sheet_shows_the_whole_sheet_on_a_small_canvas() {
    let (width, height) = (240.0_f32, 180.0_f32);
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    let paper = crate::schematic::view::drawing_sheet::resolve_active_drawing_sheet(&app.state)
        .geometry
        .paper
        .as_tuple();
    let fits_on_screen = |app: &RSpiceApp| {
        let editor = &app.state.schematic.session.editor;
        let left = editor.pan.0 + paper.0 * editor.zoom;
        let top = editor.pan.1 + paper.1 * editor.zoom;
        let right = editor.pan.0 + paper.2 * editor.zoom;
        let bottom = editor.pan.1 + paper.3 * editor.zoom;
        left >= -1e-6
            && top >= -1e-6
            && right <= f64::from(width) + 1e-6
            && bottom <= f64::from(height) + 1e-6
    };

    Command::ZoomFit.execute(&mut app);
    draw_schematic_canvas(&mut app.state, width, height);
    assert!(
        fits_on_screen(&app),
        "Fit Sheet left the paper cropped at zoom {}",
        app.state.schematic.session.editor.zoom
    );
    let fit_zoom = app.state.schematic.session.editor.zoom;
    assert!(fit_zoom < 0.25, "a small canvas needs less than 25 %");

    app.state.schematic.session.editor.zoom = 0.25;
    for _ in 0..20 {
        Command::ZoomOut.execute(&mut app);
        draw_schematic_canvas(&mut app.state, width, height);
    }
    assert!(
        (app.state.schematic.session.editor.zoom - fit_zoom).abs() < 1e-9,
        "Zoom Out stops at the zoom that fits the sheet, not above or below it"
    );
}

/// Zoom to Selection frames what is selected; Zoom Box frames a rectangle
/// dragged on the canvas. Neither existed.
#[test]
fn zoom_to_selection_and_zoom_box_are_commands() {
    for id in ["zoom-to-selection", "zoom-box"] {
        let command = Command::from_stable_id(id);
        assert!(command.is_some(), "{id} is not a command");
        assert!(
            command_catalog().any(|candidate| Some(candidate) == command),
            "{id} is not offered in the palette"
        );
    }
}

#[test]
fn zoom_to_selection_frames_what_is_selected() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    app.state.schematic.session.editor.needs_fit = false;
    app.state.schematic.session.editor.needs_drawing_sheet_fit = false;
    let resistor = crate::state::Component::new(
        41,
        crate::state::ComponentType::Resistor,
        crate::state::Point::new(3000, 2000),
    );
    app.state
        .schematic
        .document_mut_for_test()
        .components
        .push(resistor.clone());
    let (min, max) =
        crate::schematic::view::schematic_symbol_context(&app.state).component_bounds(&resistor);
    let (min_x, min_y, max_x, max_y) = (min.x, min.y, max.x, max.y);
    draw_schematic_canvas(&mut app.state, 800.0, 600.0);
    assert!(
        !Command::ZoomToSelection.is_enabled(&app),
        "nothing is selected"
    );

    app.state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(41);
    assert!(Command::ZoomToSelection.is_enabled(&app));
    Command::ZoomToSelection.execute(&mut app);
    draw_schematic_canvas(&mut app.state, 800.0, 600.0);

    let (x, y) = world_at_canvas_centre(&app, 800.0, 600.0);
    let centre = (
        f64::from(min_x + max_x) / 2.0,
        f64::from(min_y + max_y) / 2.0,
    );
    assert!((x - centre.0).abs() < 1e-6 && (y - centre.1).abs() < 1e-6);
    assert!(
        app.state.schematic.session.editor.zoom > 1.0,
        "one part fills the canvas"
    );
}

/// One frame of the canvas with these pointer events, on a context that
/// lives across frames so egui can tell a drag from a click.
fn drive_schematic_canvas(state: &mut AppState, ctx: &egui::Context, events: Vec<egui::Event>) {
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 600.0),
            )),
            events,
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    crate::schematic::view::render_schematic_view(ui, state, None);
                });
        },
    );
}

#[test]
fn zoom_box_frames_the_dragged_rectangle_and_lets_go() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    app.state.schematic.session.editor.needs_fit = false;
    app.state.schematic.session.editor.needs_drawing_sheet_fit = false;
    app.state.schematic.session.editor.zoom = 1.0;
    app.state.schematic.session.editor.pan = (0.0, 0.0);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    drive_schematic_canvas(&mut app.state, &ctx, Vec::new());

    app.execute_shortcut_command(Command::ZoomBox);
    assert!(app.state.schematic.session.editor.zoom_box.armed);
    let (from, to) = (egui::pos2(100.0, 100.0), egui::pos2(300.0, 250.0));
    let button = |pos, pressed| egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    for events in [
        vec![egui::Event::PointerMoved(from)],
        vec![button(from, true)],
        vec![egui::Event::PointerMoved(egui::pos2(200.0, 175.0))],
        vec![egui::Event::PointerMoved(to)],
        vec![button(to, false)],
        Vec::new(),
    ] {
        drive_schematic_canvas(&mut app.state, &ctx, events);
    }

    let editor = &app.state.schematic.session.editor;
    assert!(!editor.zoom_box.armed, "the box is one-shot");
    assert!(
        editor.zoom > 3.0,
        "the 200 x 150 box fills the canvas, zoom {}",
        editor.zoom
    );
    let (x, y) = world_at_canvas_centre(&app, 800.0, 600.0);
    assert!(
        (x - 200.0).abs() < 1.0 && (y - 175.0).abs() < 1.0,
        "the box's middle is the canvas's middle, not ({x}, {y})"
    );
    assert!(
        app.state.schematic.session.editor.selection.is_empty(),
        "the drag drew a zoom box, not a selection marquee"
    );
}

#[test]
fn escape_lets_go_of_an_armed_zoom_box() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    Command::ZoomBox.execute(&mut app);
    assert!(app.state.schematic.session.editor.zoom_box.armed);
    app.execute_shortcut_command(Command::Cancel);
    assert!(!app.state.schematic.session.editor.zoom_box.armed);
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
fn full_screen_command_enters_in_the_same_frame_with_no_dialog() {
    let mut app = RSpiceApp::test_instance();
    assert!(!app.state.workbench.full_screen_presentation);
    assert_eq!(app.state.ui.take_full_screen_request(), None);

    Command::ToggleFullScreen.execute(&mut app);

    assert!(!app.state.dialogs.application_modal_open());
    assert!(app.state.workbench.full_screen_presentation);
    assert!(app.state.workbench.full_screen);
    assert_eq!(app.state.ui.take_full_screen_request(), Some(true));

    Command::ToggleFullScreen.execute(&mut app);

    assert!(!app.state.workbench.full_screen_presentation);
    assert!(!app.state.workbench.full_screen);
    assert_eq!(app.state.ui.take_full_screen_request(), Some(false));
}

#[test]
fn reset_active_view_command_resets_and_reports_in_the_same_frame() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);

    Command::ResetActiveView.execute(&mut app);

    assert!(!app.state.dialogs.application_modal_open());
    assert_eq!(app.state.prompt_text(), Some("Design view reset."));
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
