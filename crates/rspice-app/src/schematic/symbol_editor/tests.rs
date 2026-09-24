//! Cases for the symbol editing surface.

use super::*;

#[test]
fn symbol_editor_has_no_direct_product_key_bypass() {
    let source = include_str!("../symbol_editor.rs");
    assert!(!source.contains(concat!("input.", "key_pressed(")));
    assert!(!source.contains(concat!("ctx.input_mut(|input| input.", "consume_key")));
    assert!(!source.contains(concat!("Use S", " for select")));
    assert!(!source.contains(concat!("Escape", " to cancel")));
}

#[test]
fn finishing_pending_polyline_creates_one_selected_shape() {
    let mut state = AppState::default();
    let mut document = SymbolDocument::default();
    state.ui.symbol.pending_polyline = vec![Point::new(0, 0), Point::new(10, 0)];

    assert!(finish_pending_polyline(&mut state, &mut document));

    assert_eq!(document.body.len(), 1);
    assert_eq!(state.ui.symbol.selected_shape, Some(0));
    assert!(state.ui.symbol.selection.shapes.contains(&0));
    assert!(state.ui.symbol.pending_polyline.is_empty());
}

#[test]
fn place_pin_without_available_pin_does_not_record_undo() {
    let mut state = AppState::default();
    let mut document = SymbolDocument::default();

    assert!(!place_selected_pin(
        &mut state,
        &mut document,
        Point::new(10, 0)
    ));

    assert!(!state.can_undo_active_symbol_document());
    assert!(document.pins.is_empty());
}

#[test]
fn drag_symbol_edit_records_one_undo_snapshot_per_gesture() {
    let mut state = AppState::default();
    let document = SymbolDocument::default();

    record_drag_symbol_edit(&mut state, &document);
    record_drag_symbol_edit(&mut state, &document);

    let key = state.workspace.active_key();
    assert_eq!(
        state.ui.symbol.undo_stacks.get(&key).map(Vec::len),
        Some(1),
        "a drag must create one undo transaction no matter how many snap buckets it crosses"
    );
    assert!(state.ui.symbol.drag_undo_recorded);

    state.ui.symbol.clear_drag_state();

    assert!(!state.ui.symbol.drag_undo_recorded);
}

#[test]
fn smooth_scroll_zoom_factor_is_proportional_not_binary() {
    let tiny = symbol_scroll_zoom_factor(1.0).expect("tiny smooth scroll should zoom");
    assert!(
        tiny > 1.0 && tiny < 1.01,
        "tiny smooth-scroll residue must not apply a full wheel notch: {tiny}"
    );

    let wheel = symbol_scroll_zoom_factor(120.0).expect("wheel scroll should zoom");
    assert!(
        wheel > tiny && wheel < 1.2,
        "a normal wheel notch should be noticeable but restrained: {wheel}"
    );
}

#[test]
fn preview_viewport_fits_large_symbols_inside_tile_body() {
    let document = SymbolDocument {
        body: vec![SymbolShape::Polyline {
            points: vec![
                Point::new(-300, -220),
                Point::new(300, -220),
                Point::new(300, 220),
                Point::new(-300, 220),
            ],
            closed: true,
        }],
        ..SymbolDocument::default()
    };
    let body_rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(168.0, 102.0));

    let viewport = preview_viewport_for_tile(body_rect, &document);
    let (min, max) = document_bounds(&document);
    let min_screen = viewport.world_to_screen(min);
    let max_screen = viewport.world_to_screen(max);

    assert!(
        body_rect.shrink(10.0).contains(min_screen),
        "min={min_screen:?}"
    );
    assert!(
        body_rect.shrink(10.0).contains(max_screen),
        "max={max_screen:?}"
    );
    assert!(
        viewport.zoom < 0.25,
        "large authored symbols must be scaled down for preview: {}",
        viewport.zoom
    );
}

#[test]
fn preview_viewport_fits_nonzero_origin_as_placed_symbol() {
    let document = SymbolDocument {
        origin: Point::new(200, 100),
        name_anchor: Point::new(200, 70),
        value_anchor: Point::new(200, 130),
        body: vec![SymbolShape::Polyline {
            points: vec![
                Point::new(160, 80),
                Point::new(240, 80),
                Point::new(240, 120),
                Point::new(160, 120),
            ],
            closed: true,
        }],
        ..SymbolDocument::default()
    };
    let body_rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(168.0, 102.0));

    let viewport = preview_viewport_for_tile(body_rect, &document);
    let (min, max) = document_bounds(&document);
    let min_screen = viewport.world_to_screen(min - document.origin);
    let max_screen = viewport.world_to_screen(max - document.origin);

    assert!(
        body_rect.shrink(10.0).contains(min_screen),
        "effective min={min_screen:?}"
    );
    assert!(
        body_rect.shrink(10.0).contains(max_screen),
        "effective max={max_screen:?}"
    );
}

#[test]
fn preview_tile_uses_larger_size_when_canvas_allows() {
    let canvas = Rect::from_min_size(pos2(0.0, 0.0), vec2(960.0, 640.0));

    let tile = preview_tile_rect(canvas);

    assert!(
        tile.width() >= 220.0 && tile.height() >= 156.0,
        "preview tile should be large enough for readable symbols: {tile:?}"
    );
    assert!(canvas.contains(tile.min));
    assert!(canvas.contains(tile.max));
}

#[test]
fn symbol_canvas_accessibility_label_reports_editing_contract() {
    let mut state = AppState::default();
    state.ui.symbol.tool = SymbolTool::Circle;
    state.ui.symbol.select_shape(0);
    let document = SymbolDocument {
        pins: vec![SymbolPin::new(
            "OUT",
            PortDirection::Out,
            Some(Point::new(20, 0)),
        )],
        body: vec![SymbolShape::Circle {
            center: Point::origin(),
            radius: 10,
        }],
        ..SymbolDocument::default()
    };

    let label = symbol_canvas_accessibility_label(
        &document,
        &state,
        false,
        crate::workbench::commands::vocabulary::CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    );

    assert!(label.starts_with(
        "Symbol editor canvas. 1 shape; 1 pin, 1 pin placed; 1 item selected. Active tool: Circle. Editable."
    ));
    assert!(label.contains("P: Place symbol pin"));
    assert!(label.contains("Escape: Cancel active command"));
}

/// The display lattice is body artwork's business. A terminal that landed on
/// a 2.5 grid point is a terminal a parent schematic cannot wire to, so the
/// pin snap is deliberately not the one the toolbar shows.
#[test]
fn terminal_snap_holds_at_the_terminal_pitch_on_a_fine_display_grid() {
    assert_eq!(
        snap_point(Point::new(12, -13), SymbolGridSpacing::TwoPointFive),
        Point::new(13, -13),
        "body geometry follows the display lattice the author selected"
    );
    assert_eq!(
        snap_to_terminal_grid(Point::new(12, -13)),
        Point::new(10, -10)
    );
    assert_eq!(
        snap_to_terminal_grid(Point::new(16, -16)),
        Point::new(20, -20)
    );
    assert_eq!(
        snap_to_terminal_grid(Point::new(-14, 5)),
        Point::new(-10, 10)
    );
}

#[test]
fn symbol_grid_defaults_to_the_terminal_pitch() {
    let state = AppState::default();

    assert_eq!(state.ui.symbol.grid_spacing, SymbolGridSpacing::Ten);
    assert_eq!(
        SymbolGridSpacing::TwoPointFive.label(),
        "grid 2.5 \u{00b7} fine"
    );
}

/// The row used to state a count nothing computed. Every row now answers
/// from the document, and the off-grid row answers from the same finding the
/// symbol check publishes.
#[test]
fn save_checks_compute_the_off_grid_row_from_the_document() {
    let ports = [
        PortSpec {
            name: "IN".to_owned(),
            direction: PortDirection::In,
        },
        PortSpec {
            name: "OUT".to_owned(),
            direction: PortDirection::Out,
        },
    ];
    let document = SymbolDocument {
        pins: vec![
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 0))),
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(43, 0))),
        ],
        ..SymbolDocument::default()
    };

    let checks = symbol_save_checks(None, "amp", &document, &ports);
    let labels: Vec<&str> = checks.iter().map(|check| check.label).collect();

    assert!(
        !labels.contains(&"Hidden power pins"),
        "a row that states a count nothing computes is not a check: {labels:?}"
    );
    let off_grid = checks
        .iter()
        .find(|check| check.label == "Off-grid terminals")
        .expect("the off-grid row is computed");
    assert_eq!(off_grid.observed, "1");
    assert!(!off_grid.passed);
    assert!(off_grid.refusal().contains("off-grid terminals"));
}

#[test]
fn save_checks_pass_a_symbol_that_matches_its_interface() {
    let ports = [PortSpec {
        name: "IN".to_owned(),
        direction: PortDirection::In,
    }];
    let document = SymbolDocument {
        pins: vec![SymbolPin::new(
            "IN",
            PortDirection::In,
            Some(Point::new(-40, 0)),
        )],
        ..SymbolDocument::default()
    };

    let checks = symbol_save_checks(None, "amp", &document, &ports);

    assert!(
        checks.iter().all(|check| check.passed),
        "{:?}",
        checks
            .iter()
            .filter(|check| !check.passed)
            .map(|check| check.label)
            .collect::<Vec<_>>()
    );
}

/// Shift-click grows the selection instead of replacing it, which is the
/// only way a group drag can be armed at all.
#[test]
fn a_multi_object_grab_moves_the_whole_selection_as_one_edit() {
    let mut state = AppState::default();
    let mut document = SymbolDocument {
        pins: vec![SymbolPin::new(
            "IN",
            PortDirection::In,
            Some(Point::new(-40, 0)),
        )],
        body: vec![SymbolShape::Circle {
            center: Point::new(0, 0),
            radius: 10,
        }],
        ..SymbolDocument::default()
    };
    let mut editor = SymbolEditorMetadata::for_document(&document);
    let mut selection = SymbolSelection::single_pin("IN");
    selection.toggle_shape(0);
    assert_eq!(selection.len(), 2);
    state.ui.symbol.set_selection(selection);

    record_drag_symbol_edit(&mut state, &document);
    translate_selection(&mut state, &mut document, &mut editor, Point::new(10, 20));
    record_drag_symbol_edit(&mut state, &document);

    assert_eq!(
        document.pin("IN").and_then(|pin| pin.position),
        Some(Point::new(-30, 20))
    );
    assert!(matches!(
        document.body.first(),
        Some(SymbolShape::Circle { center, .. }) if *center == Point::new(10, 20)
    ));
    let key = state.workspace.active_key();
    assert_eq!(
        state.ui.symbol.undo_stacks.get(&key).map(Vec::len),
        Some(1),
        "a group drag is one undo transaction, not one per moved object"
    );
}
