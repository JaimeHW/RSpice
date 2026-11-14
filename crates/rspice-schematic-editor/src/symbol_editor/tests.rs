//! Existing symbol canvas geometry and viewport regressions.

use super::*;

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
/// The display lattice is body artwork's business. A terminal that landed on
/// a fine grid point is a terminal a parent schematic cannot wire to, so the
/// pin snap is deliberately not the one the toolbar shows.
#[test]
fn terminal_snap_holds_at_the_terminal_pitch_on_a_fine_display_grid() {
    assert_eq!(
        snap_point(Point::new(12, -13), SymbolGridSpacing::Two),
        Point::new(12, -14),
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
    let state = SymbolEditorSession::default();

    assert_eq!(state.grid_spacing, SymbolGridSpacing::Ten);
    assert_eq!(
        SymbolGridSpacing::ALL.map(SymbolGridSpacing::label),
        ["grid 10", "grid 5", "grid 2", "grid 1"]
    );
}

/// Coordinates are whole units, so a pitch is offered only if every point
/// snapped to it lands on its lattice: a 2.5 grid put points on 0, 3, 5, 8.
#[test]
fn every_grid_pitch_lands_points_on_its_own_lattice() {
    for spacing in SymbolGridSpacing::ALL {
        let step = spacing.model_step();
        for value in -25..=25 {
            let snapped = snap_point(Point::new(value, -value), spacing);
            assert_eq!(
                snapped.x % step,
                0,
                "{spacing:?} took {value} to {}",
                snapped.x
            );
            assert_eq!(
                snapped.y % step,
                0,
                "{spacing:?} took {value} to {}",
                snapped.y
            );
            assert!((snapped.x - value).abs() * 2 <= step, "{spacing:?} {value}");
        }
    }
}
