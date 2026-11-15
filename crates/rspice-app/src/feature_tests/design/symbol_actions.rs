//! The symbol editor's actions run through a whole application.

use crate::feature_tests::design::symbol_actions_drawn::open_symbol;
use crate::workbench::app_state::AppState;
use crate::workbench::commands::vocabulary::Command;
use rspice_app_design::workbench::app::actions::symbol::*;
use rspice_app_design::workbench::design::commands::DesignCommand;
use rspice_app_state::state::Point;
use rspice_app_state::state::{CellViewRef, SymbolEditorMetadata, SymbolShape};
use rspice_design::symbol::{SYMBOL_EDITOR_METADATA_KEY, SymbolAttributeKind};
use rspice_schematic_editor::symbol_editor::commands::{SymbolEditAction, SymbolTransform};
use rspice_schematic_editor::symbol_editor::interaction::bind_canvas_source;
use rspice_schematic_editor::symbol_editor::session::{
    SymbolSelection, SymbolTextDraft, SymbolTool,
};

fn metadata_encoding(state: &AppState) -> Option<String> {
    state
        .model
        .library_manager
        .get_library("work")
        .unwrap()
        .get_cell("amp")
        .unwrap()
        .get_view("symbol")
        .unwrap()
        .metadata
        .get(SYMBOL_EDITOR_METADATA_KEY)
        .cloned()
}

/// Duplicate in the symbol editor copied the selection over the user's
/// clipboard and pasted it from there, so a copy made before was lost. It
/// leaves the clipboard alone, as the schematic's Duplicate does (audit E-15).
#[test]
fn duplicate_leaves_the_symbol_clipboard_alone() {
    let mut app = open_symbol();
    let mut circle = SymbolSelection::default();
    circle.shapes.insert(0);
    app.state.design.symbol.editor.set_selection(circle);
    Command::Copy.execute(&mut app);
    let clipboard = app.state.design.symbol.editor.clipboard.clone();
    assert_eq!(clipboard.shapes.len(), 1);

    let mut dot = SymbolSelection::default();
    dot.shapes.insert(1);
    app.state.design.symbol.editor.set_selection(dot);
    Command::Duplicate.execute(&mut app);

    let document = app.state.model.load_active_symbol_document().unwrap();
    assert_eq!(document.body.len(), 3, "the dot was duplicated");
    assert!(matches!(document.body[2], SymbolShape::Dot { .. }));
    assert_eq!(app.state.design.symbol.editor.clipboard, clipboard);
}

#[test]
fn cut_paste_commands_preserve_order_unique_pin_names_and_undo() {
    let mut app = open_symbol();
    let before = app.state.model.load_active_symbol_document().unwrap();
    let mut selection = SymbolSelection::single_pin("IN");
    selection.shapes.extend([0, 1]);
    app.state.design.symbol.editor.set_selection(selection);
    Command::Cut.execute(&mut app);
    assert_eq!(app.state.design.symbol.editor.clipboard.shapes, before.body);
    assert_eq!(
        app.state.design.symbol.editor.clipboard.pins,
        vec![before.pins[0].clone()]
    );
    assert!(
        app.state
            .model
            .load_active_symbol_document()
            .unwrap()
            .body
            .is_empty()
    );
    assert!(
        app.state
            .design
            .symbol
            .editor
            .effective_selection()
            .is_empty()
    );
    assert!(app.state.undo_active_symbol_document().unwrap());
    assert_eq!(
        app.state.model.load_active_symbol_document().unwrap(),
        before
    );

    // The clipboard's bounds center is (-9, 1); hover wins over viewport center.
    app.state.design.canvas_hover = Some((31.0, 21.0));
    app.state.design.canvas_view_center = Some((100.0, 100.0));
    Command::Paste.execute(&mut app);
    let pasted = app.state.model.load_active_symbol_document().unwrap();
    assert_eq!(pasted.body.len(), 4);
    // The copy takes the next free name, not a copy suffix.
    let pin = pasted.pin("IN1").unwrap();
    assert_eq!(pin.position, Some(Point::new(0, 30)));
    assert_eq!(pin.side, before.pins[0].side);
    assert_eq!(pin.length, before.pins[0].length);
    assert_eq!(
        app.state.design.symbol.editor.effective_selection().shapes,
        [2, 3].into()
    );
    assert_eq!(
        app.state.design.symbol.editor.effective_selection().pins,
        ["IN1".to_owned()].into()
    );
    assert!(app.state.undo_active_symbol_document().unwrap());
    assert_eq!(
        app.state.model.load_active_symbol_document().unwrap(),
        before
    );
    assert!(app.state.redo_active_symbol_document().unwrap());
    assert_eq!(
        app.state.model.load_active_symbol_document().unwrap(),
        pasted
    );
}

#[test]
fn symbol_transforms_keep_geometry_and_metadata_in_one_undo_entry() {
    let mut app = open_symbol();
    let mut before = app.state.model.load_active_symbol_document().unwrap();
    before.origin = Point::new(10, 20);
    before.name_anchor = Point::new(30, 40);
    let metadata = SymbolEditorMetadata::for_document(&before);
    app.state
        .store_active_symbol_editor_bundle(&before, &metadata)
        .unwrap();
    let encoded = metadata_encoding(&app.state);
    let mut selection = SymbolSelection::single_pin("IN");
    selection.shapes.insert(0);
    selection.attributes.insert(SymbolAttributeKind::Reference);
    app.state.design.symbol.editor.set_selection(selection);
    let modal_open = app.state.application_modal_open();
    let state = &mut app.state;
    edit_selected_symbol_items(
        &mut state.design.symbol,
        &mut state.model,
        &mut state.workbench,
        modal_open,
        SymbolEditAction::Transform(SymbolTransform::Rotate),
    );
    let after = app.state.model.load_active_symbol_document().unwrap();
    // The selection spans x -40 to 30 and y -10 to 40, so it turns about
    // (0, 20), the terminal grid point nearest its middle, not the origin.
    assert_eq!(after.pin("IN").unwrap().position, Some(Point::new(10, -20)));
    // The pin turns with the selection: a pin on the left comes up on top.
    assert_eq!(
        after.pin("IN").unwrap().side,
        before.pin("IN").unwrap().side.turned_clockwise()
    );
    assert_eq!(after.name_anchor, Point::new(-20, 50));
    assert_eq!(
        rspice_app_design::workbench::app::actions::workspace::symbol_document::load_active_symbol_editor_metadata(&app.state.model, &after)
            .unwrap()
            .attribute(SymbolAttributeKind::Reference)
            .unwrap()
            .position,
        after.name_anchor
    );
    assert_eq!(
        app.state
            .design
            .symbol
            .history
            .undo_depth(&app.state.model.workspace.content.active_key()),
        1
    );
    assert!(app.state.undo_active_symbol_document().unwrap());
    assert_eq!(
        app.state.model.load_active_symbol_document().unwrap(),
        before
    );
    assert_eq!(metadata_encoding(&app.state), encoded);

    // A selected pin turns and flips with the Arrange commands the
    // schematic uses; there is no pin-only pair beside them. A lone pin
    // turns about its own tip: it stays where it is wired and its lead
    // turns.
    app.state.design.symbol.editor.select_pin("IN");
    let pin_at = |app: &crate::workbench::RSpiceApp| {
        let pin = app
            .state
            .model
            .load_active_symbol_document()
            .unwrap()
            .pin("IN")
            .unwrap()
            .clone();
        (pin.position, pin.side)
    };
    let (placed, _) = pin_at(&app);
    app.execute_command_key(DesignCommand::RotateSelection.key());
    assert_eq!(
        pin_at(&app),
        (placed, rspice_app_state::state::SymbolPinSide::Top),
        "Rotate turns the selected pin's lead about its tip"
    );
    app.execute_command_key(DesignCommand::MirrorSelectionVertical.key());
    assert_eq!(
        pin_at(&app),
        (placed, rspice_app_state::state::SymbolPinSide::Bottom),
        "Mirror flips the selected pin's lead across its tip"
    );
    assert!(app.state.undo_active_symbol_document().unwrap());
    assert_eq!(
        pin_at(&app),
        (placed, rspice_app_state::state::SymbolPinSide::Top)
    );
}

/// Align and Snap to grid act on the symbol's own selection, each as one
/// undo step; Align with one object selected is unavailable and says why.
#[test]
fn align_and_snap_act_on_the_symbol_selection_as_one_step() {
    use crate::workbench::framework::command_registry::CommandAvailability;
    let mut app = open_symbol();
    let key = app.state.model.workspace.content.active_key();
    let before = app.state.model.load_active_symbol_document().unwrap();

    app.state.design.symbol.editor.select_shape(0);
    assert_eq!(
        app.command_availability(DesignCommand::AlignSelectionLeft.key()),
        CommandAvailability::Disabled("select at least two objects")
    );

    let mut selection = SymbolSelection::single_pin("IN");
    selection.shapes.insert(0);
    app.state.design.symbol.editor.set_selection(selection);
    app.execute_command_key(DesignCommand::AlignSelectionLeft.key());
    let aligned = app.state.model.load_active_symbol_document().unwrap();
    // The IN tip at x -40 is the leftmost edge: the circle comes to it.
    assert_eq!(
        aligned.body[0],
        SymbolShape::Circle {
            center: Point::new(-30, 0),
            radius: 10,
            style: Default::default(),
        }
    );
    assert_eq!(aligned.pins, before.pins);
    assert_eq!(
        app.state.design.symbol.history.undo_step(&key),
        Some("Align left")
    );
    assert!(app.state.undo_active_symbol_document().unwrap());
    assert_eq!(
        app.state.model.load_active_symbol_document().unwrap(),
        before
    );

    let mut off_grid = before.clone();
    off_grid.body[1] = SymbolShape::Dot {
        center: Point::new(23, 12),
        radius: 2,
    };
    app.state
        .store_active_symbol_editor_bundle(
            &off_grid,
            &SymbolEditorMetadata::for_document(&off_grid),
        )
        .unwrap();
    app.state.design.symbol.editor.select_shape(1);
    app.execute_command_key(DesignCommand::SnapSelectionToGrid.key());
    assert_eq!(
        app.state.model.load_active_symbol_document().unwrap().body[1],
        SymbolShape::Dot {
            center: Point::new(20, 10),
            radius: 2,
        }
    );
    assert_eq!(
        app.state.design.symbol.history.undo_step(&key),
        Some("Snap to grid")
    );
    assert_eq!(
        app.state.prompt_text(),
        Some("Snapped 1 object to the grid.")
    );
}

/// Escape lets go of the shape being drawn and leaves its tool armed; a
/// second Escape puts the tool down. The document never held the shape.
#[test]
fn escape_lets_go_of_the_shape_being_drawn_then_puts_the_tool_down() {
    let mut app = open_symbol();
    let before = app.state.model.load_active_symbol_document().unwrap();
    app.state.design.symbol.editor.tool = SymbolTool::Polygon;
    app.state.design.symbol.editor.pending_polyline =
        vec![Point::origin(), Point::new(10, 20), Point::new(20, 0)];

    app.execute_shortcut_command(Command::Cancel);

    assert_eq!(
        app.state.model.load_active_symbol_document().unwrap(),
        before
    );
    assert!(app.state.design.symbol.editor.pending_polyline.is_empty());
    assert_eq!(app.state.design.symbol.editor.tool, SymbolTool::Polygon);
    assert!(!app.state.can_undo_active_symbol_document());

    app.execute_shortcut_command(Command::Cancel);

    assert_eq!(app.state.design.symbol.editor.tool, SymbolTool::Select);
}

/// Text typed into the text tool's entry is placed as one edit that stores
/// only geometry; a request made before the view changed places nothing.
#[test]
fn placing_text_is_one_edit_and_a_stale_request_places_nothing() {
    let mut app = open_symbol();
    let before = app.state.model.load_active_symbol_document().unwrap();
    assert!(metadata_encoding(&app.state).is_none());
    let draft = || SymbolTextDraft {
        anchor: Point::new(0, -30),
        text: "GAIN".to_owned(),
        caret_placed: true,
    };
    app.state.design.symbol.editor.tool = SymbolTool::Text;
    app.state.design.symbol.editor.text_draft = Some(draft());

    // What the entry does on Enter.
    let request = symbol_text_request(
        &mut app.state.design.symbol,
        &app.state.model,
        &mut app.state.workbench,
    )
    .unwrap();
    let modal_open = app.state.application_modal_open();
    apply_symbol_edit_request(
        &mut app.state.design.symbol,
        &mut app.state.model,
        &mut app.state.workbench,
        modal_open,
        request,
    );

    let placed = app.state.model.load_active_symbol_document().unwrap();
    assert_eq!(
        placed.body.last(),
        Some(&SymbolShape::Text {
            anchor: Point::new(0, -30),
            text: "GAIN".to_owned(),
            size: Default::default(),
            align: Default::default(),
            rotation: Default::default(),
        })
    );
    assert_eq!(placed.body.len(), before.body.len() + 1);
    assert!(metadata_encoding(&app.state).is_none());
    assert!(app.state.design.symbol.editor.text_draft.is_none());
    assert_eq!(app.state.design.symbol.editor.tool, SymbolTool::Text);
    assert!(app.state.undo_active_symbol_document().unwrap());
    assert_eq!(
        app.state.model.load_active_symbol_document().unwrap(),
        before
    );

    let source =
        rspice_app_design::workbench::app::actions::workspace::symbol_document::symbol_editor_request_source(&app.state.model, &app.state.workbench);
    bind_canvas_source(&mut app.state.design.symbol.editor, source);
    app.state.design.symbol.editor.text_draft = Some(draft());
    let pending = symbol_edit_request(
        &app.state.design.symbol,
        &app.state.model,
        &app.state.workbench,
        SymbolEditAction::PlaceText {
            anchor: Point::new(0, -30),
            text: "GAIN".to_owned(),
        },
    );
    app.state
        .open_workspace_view(CellViewRef::new("work", "amp", "schematic"));
    app.state
        .open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    app.state.design.symbol.editor.text_draft = Some(draft());
    let modal_open = app.state.application_modal_open();
    apply_symbol_edit_request(
        &mut app.state.design.symbol,
        &mut app.state.model,
        &mut app.state.workbench,
        modal_open,
        pending,
    );
    assert_eq!(
        app.state.model.load_active_symbol_document().unwrap(),
        before
    );
    assert!(!app.state.can_undo_active_symbol_document());
}

/// The symbol canvas navigates as the schematic does: the zoom commands
/// share the schematic's range, Zoom box arms over the symbol, and Zoom to
/// selection frames what is selected.
#[test]
fn the_symbol_canvas_shares_the_schematic_navigation() {
    use crate::feature_tests::replay::Replay;
    use rspice_schematic_editor::view::navigation::SCHEMATIC_ZOOM_MAX;
    let mut replay = Replay::of(open_symbol());
    replay.frames(Vec::new());
    for _ in 0..30 {
        replay.app.execute_command_key(Command::ZoomIn.key());
        replay.frame(Vec::new());
    }
    assert_eq!(
        f64::from(replay.app.state.design.symbol.editor.zoom),
        SCHEMATIC_ZOOM_MAX,
        "Zoom in stops where the schematic's does"
    );

    replay.app.execute_command_key(Command::ZoomBox.key());
    assert!(replay.app.state.design.symbol.editor.zoom_box.armed);
    replay.app.state.design.symbol.editor.zoom_box.disarm();

    // The dot at (20, 10) is framed in the middle of the canvas.
    replay.app.state.design.symbol.editor.select_shape(1);
    replay.app.execute_command_key(Command::ZoomOut.key());
    replay.frame(Vec::new());
    replay
        .app
        .execute_command_key(Command::ZoomToSelection.key());
    replay.frame(Vec::new());
    let editor = &replay.app.state.design.symbol.editor;
    assert!(
        (editor.pan.0 + 20.0 * editor.zoom).abs() < 1.0
            && (editor.pan.1 + 10.0 * editor.zoom).abs() < 1.0,
        "zoom {} pan {:?}",
        editor.zoom,
        editor.pan
    );
}

/// A tool chosen on the toolbar hands the keyboard back to the canvas, so
/// the canvas's single keys work at once: G steps the grid.
#[test]
fn a_toolbar_tool_hands_the_keyboard_back_to_the_canvas() {
    use crate::feature_tests::replay::Replay;
    use rspice_schematic_editor::symbol_editor::session::SymbolTool;
    let mut replay = Replay::of(open_symbol());
    replay.click("Draw symbol rectangle (R)", egui::accesskit::Role::Button);
    assert_eq!(
        replay.app.state.design.symbol.editor.tool,
        SymbolTool::Rectangle
    );
    let shown = replay.app.state.design.symbol.editor.show_grid;
    replay.key(egui::Key::G, egui::Modifiers::NONE);
    assert_ne!(
        replay.app.state.design.symbol.editor.show_grid, shown,
        "G reached the canvas right after the toolbar"
    );
}
