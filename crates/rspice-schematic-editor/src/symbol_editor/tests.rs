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
            style: Default::default(),
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
            style: Default::default(),
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
/// The preview keeps a column of its own at the canvas's right edge, large
/// enough to read a symbol in, so the drawing is laid out beside it and
/// never under it; a canvas too narrow to give a column up shows none.
#[test]
fn the_preview_keeps_a_readable_column_of_its_own() {
    let canvas = Rect::from_min_size(pos2(0.0, 0.0), vec2(960.0, 640.0));
    let width = preview_column_width(canvas.width());
    let column = Rect::from_min_max(pos2(canvas.right() - width, canvas.top()), canvas.max);

    let tile = preview_tile_rect(column);

    assert!(
        tile.width() >= 220.0 && tile.height() >= 156.0,
        "preview tile should be large enough for readable symbols: {tile:?}"
    );
    assert!(
        column.contains(tile.min) && column.contains(tile.max),
        "{tile:?} in {column:?}"
    );
    assert_eq!(
        preview_column_width(480.0),
        0.0,
        "a narrow canvas keeps its width"
    );
}

/// The tile's header states the scale the tile draws at, in the status
/// bar's terms. It said "100%" whatever the symbol's size: a large symbol
/// fitted into the tile at a fifth of that read as drawn at full size.
#[test]
fn the_preview_header_states_the_scale_it_draws_at() {
    let large = SymbolDocument {
        body: vec![SymbolShape::Polyline {
            points: vec![
                Point::new(-300, -220),
                Point::new(300, -220),
                Point::new(300, 220),
                Point::new(-300, 220),
            ],
            closed: true,
            style: Default::default(),
        }],
        ..SymbolDocument::default()
    };
    let body = Rect::from_min_size(pos2(0.0, 0.0), vec2(240.0, 142.0));
    let zoom = preview_viewport_for_tile(body, &large).zoom;

    assert!(zoom < 1.0, "{zoom}");
    assert_eq!(
        preview_scale_label(zoom),
        format!("AS PLACED \u{b7} {}%", (zoom * 100.0).round())
    );
    assert_ne!(preview_scale_label(zoom), "AS PLACED \u{b7} 100%");
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

/// The canvas inks a shape in its style, as a placed instance does: a filled
/// outline paints its inside, a dashed one leaves gaps along its length, a
/// heavy one weighs more than a thin one. A filled shape is picked anywhere
/// inside it; an unfilled one only on its edge.
#[test]
fn the_canvas_draws_and_picks_a_shape_in_its_style() {
    use rspice_design::symbol::{SymbolDash, SymbolFill, SymbolStroke, SymbolStyle};

    let ink_color = Color32::from_rgb(255, 64, 64);
    let viewport = SymbolViewport::new(
        Rect::from_min_size(Pos2::ZERO, vec2(320.0, 320.0)),
        4.0,
        Vec2::ZERO,
    );
    let document = |shape: SymbolShape| SymbolDocument {
        body: vec![shape],
        ..SymbolDocument::default()
    };
    let canvas = |shape: SymbolShape| {
        let document = document(shape);
        rspice_ui_kit::raster::render(vec2(320.0, 320.0), |ui, _| {
            draw_body(
                &ui.painter().clone(),
                viewport,
                &document,
                ink_color,
                &Default::default(),
                Color32::WHITE,
            );
        })
    };
    let ink = |canvas: &rspice_ui_kit::raster::Canvas, rect: Rect| -> (usize, u64) {
        let background = canvas.background();
        canvas
            .pixels_in(rect)
            .fold((0, 0), |(count, weight), pixel| {
                let distance = [
                    (pixel.r(), background.r()),
                    (pixel.g(), background.g()),
                    (pixel.b(), background.b()),
                ]
                .into_iter()
                .map(|(a, b)| u64::from(a.abs_diff(b)))
                .sum::<u64>();
                (count + usize::from(distance > 0), weight + distance)
            })
    };

    // The triangle's corners land at (80, 100), (240, 160) and (80, 220).
    let triangle = |style| SymbolShape::Polyline {
        points: vec![Point::new(-20, -15), Point::new(20, 0), Point::new(-20, 15)],
        closed: true,
        style,
    };
    let filled = SymbolStyle {
        fill: SymbolFill::Solid,
        ..SymbolStyle::default()
    };
    let inside = Rect::from_center_size(pos2(140.0, 160.0), vec2(16.0, 16.0));
    assert_eq!(ink(&canvas(triangle(SymbolStyle::default())), inside).0, 0);
    let painted = canvas(triangle(filled));
    assert!(painted.pixels_in(inside).all(|pixel| pixel == ink_color));
    assert_eq!(
        hit_shape(&document(triangle(filled)), viewport, pos2(140.0, 160.0)),
        Some(0)
    );
    assert_eq!(
        hit_shape(
            &document(triangle(SymbolStyle::default())),
            viewport,
            pos2(140.0, 160.0)
        ),
        None
    );

    // The line runs from x 40 to x 280 along y 160.
    let line = |style| SymbolShape::Polyline {
        points: vec![Point::new(-30, 0), Point::new(30, 0)],
        closed: false,
        style,
    };
    let along = Rect::from_min_max(pos2(60.0, 150.0), pos2(260.0, 170.0));
    let columns = |style| {
        let drawn = canvas(line(style));
        (60..260)
            .filter(|x| {
                let column =
                    Rect::from_min_max(pos2(*x as f32, 150.0), pos2(*x as f32 + 1.0, 170.0));
                ink(&drawn, column).0 > 0
            })
            .count()
    };
    assert_eq!(columns(SymbolStyle::default()), 200);
    for dash in [SymbolDash::Dashed, SymbolDash::Dotted] {
        let broken = columns(SymbolStyle {
            dash,
            ..SymbolStyle::default()
        });
        assert!(broken > 0 && broken < 160, "{dash:?}: {broken}");
    }
    let weight = |stroke| {
        ink(
            &canvas(line(SymbolStyle {
                stroke,
                ..SymbolStyle::default()
            })),
            along,
        )
        .1
    };
    let (thin, normal, heavy) = (
        weight(SymbolStroke::Thin),
        weight(SymbolStroke::Normal),
        weight(SymbolStroke::Heavy),
    );
    assert!(thin < normal && normal < heavy, "{thin} {normal} {heavy}");
}
