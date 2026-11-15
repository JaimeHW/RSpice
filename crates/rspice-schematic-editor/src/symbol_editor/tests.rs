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

/// An arrowhead is drawn in symbol units: twice the zoom draws it twice as
/// long, as the body it points along grows.
#[test]
fn an_arrowhead_keeps_its_size_against_the_body_at_every_zoom() {
    use rspice_design::symbol::ARROW_LENGTH;

    let document = SymbolDocument {
        body: vec![SymbolShape::Arrow {
            tip: Point::origin(),
            rotation_quarters: 0,
        }],
        ..SymbolDocument::default()
    };
    let drawn_length = |zoom: f32| {
        let viewport = SymbolViewport::new(
            Rect::from_min_size(Pos2::ZERO, vec2(320.0, 320.0)),
            zoom,
            Vec2::ZERO,
        );
        let canvas = rspice_ui_kit::raster::render(vec2(320.0, 320.0), |ui, _| {
            draw_body(
                &ui.painter().clone(),
                viewport,
                &document,
                Color32::from_rgb(255, 64, 64),
                &Default::default(),
                Color32::WHITE,
            );
        });
        // The tip lands at (160, 160) and the head reaches back left of it.
        (0..160)
            .filter(|x| {
                let column = Rect::from_min_max(pos2(*x as f32, 0.0), pos2(*x as f32 + 1.0, 320.0));
                canvas
                    .pixels_in(column)
                    .any(|pixel| pixel != canvas.background())
            })
            .count() as f32
    };
    for zoom in [2.0, 4.0] {
        let length = drawn_length(zoom);
        let wanted = ARROW_LENGTH as f32 * zoom;
        assert!(
            (length - wanted).abs() <= 2.0,
            "at zoom {zoom} the head is {length} px long; {wanted} px is {ARROW_LENGTH} units"
        );
    }
}

/// A run turned to read upward is drawn upward on the canvas and picked
/// where it is drawn: above the anchor it starts at, not to its right.
#[test]
fn the_canvas_draws_and_picks_a_run_read_upward() {
    use rspice_design::symbol::{SymbolTextRotation, SymbolTextSize};

    let viewport = SymbolViewport::new(
        Rect::from_min_size(Pos2::ZERO, vec2(320.0, 320.0)),
        4.0,
        Vec2::ZERO,
    );
    // The anchor lands at (160, 160).
    let document = |rotation| SymbolDocument {
        body: vec![SymbolShape::Text {
            anchor: Point::origin(),
            text: "AMP".to_owned(),
            size: SymbolTextSize::Normal,
            align: SymbolTextAlign::Left,
            rotation,
        }],
        ..SymbolDocument::default()
    };
    let canvas = |rotation| {
        let document = document(rotation);
        rspice_ui_kit::raster::render(vec2(320.0, 320.0), |ui, _| {
            draw_body(
                &ui.painter().clone(),
                viewport,
                &document,
                Color32::from_rgb(255, 64, 64),
                &Default::default(),
                Color32::WHITE,
            );
        })
    };
    let painted = |canvas: &rspice_ui_kit::raster::Canvas, rect: Rect| {
        canvas
            .pixels_in(rect)
            .filter(|pixel| *pixel != canvas.background())
            .count()
    };
    let span = |canvas: &rspice_ui_kit::raster::Canvas, rect: Rect| {
        let columns = (rect.left() as i32..rect.right() as i32)
            .filter(|x| {
                let strip = Rect::from_min_max(
                    pos2(*x as f32, rect.top()),
                    pos2(*x as f32 + 1.0, rect.bottom()),
                );
                painted(canvas, strip) > 0
            })
            .count();
        let rows = (rect.top() as i32..rect.bottom() as i32)
            .filter(|y| {
                let strip = Rect::from_min_max(
                    pos2(rect.left(), *y as f32),
                    pos2(rect.right(), *y as f32 + 1.0),
                );
                painted(canvas, strip) > 0
            })
            .count();
        (columns, rows)
    };
    let above = Rect::from_min_max(pos2(120.0, 40.0), pos2(200.0, 158.0));
    let right = Rect::from_min_max(pos2(200.0, 120.0), pos2(320.0, 200.0));

    let across = canvas(SymbolTextRotation::R0);
    assert!(painted(&across, right) > 0);
    let upward = canvas(SymbolTextRotation::R90);
    assert_eq!(painted(&upward, right), 0, "nothing reads rightward");
    let (columns, rows) = span(&upward, above);
    assert!(
        columns > 0 && rows > columns,
        "read upward, the glyphs stack: {columns} columns by {rows} rows"
    );

    // Ten units above the anchor is on the run read upward and off the one
    // read across; ten to its right the other way round.
    let over = pos2(160.0, 120.0);
    let beside = pos2(200.0, 160.0);
    assert_eq!(
        hit_shape(&document(SymbolTextRotation::R90), viewport, over),
        Some(0)
    );
    assert_eq!(
        hit_shape(&document(SymbolTextRotation::R90), viewport, beside),
        None
    );
    assert_eq!(
        hit_shape(&document(SymbolTextRotation::R0), viewport, beside),
        Some(0)
    );
    assert_eq!(
        hit_shape(&document(SymbolTextRotation::R0), viewport, over),
        None
    );
}

/// A label is lettered on the canvas with its default at its true size,
/// faint while hidden on instances, and picked anywhere on that text.
#[test]
fn the_canvas_letters_a_label_with_its_default() {
    use rspice_design::symbol::{SymbolTextRotation, SymbolTextSize};

    let viewport = SymbolViewport::new(
        Rect::from_min_size(Pos2::ZERO, vec2(320.0, 320.0)),
        4.0,
        Vec2::ZERO,
    );
    // "W=1u" from the anchor at (160, 160): four glyphs, 20 units, 80 px.
    let document = |shown| SymbolDocument {
        body: vec![SymbolShape::Label {
            anchor: Point::origin(),
            parameter: "W".to_owned(),
            default: "1u".to_owned(),
            size: SymbolTextSize::Normal,
            align: SymbolTextAlign::Left,
            rotation: SymbolTextRotation::R0,
            shown,
        }],
        ..SymbolDocument::default()
    };
    let weight = |shown| {
        let document = document(shown);
        let canvas = rspice_ui_kit::raster::render(vec2(320.0, 320.0), |ui, _| {
            draw_body(
                &ui.painter().clone(),
                viewport,
                &document,
                Color32::from_rgb(255, 64, 64),
                &Default::default(),
                Color32::WHITE,
            );
        });
        let background = canvas.background();
        canvas
            .pixels_in(Rect::from_min_max(pos2(160.0, 140.0), pos2(250.0, 180.0)))
            .map(|pixel| {
                u64::from(pixel.r().abs_diff(background.r()))
                    + u64::from(pixel.g().abs_diff(background.g()))
                    + u64::from(pixel.b().abs_diff(background.b()))
            })
            .sum::<u64>()
    };
    let (shown, hidden) = (weight(true), weight(false));
    assert!(hidden > 0, "a hidden label is still drawn");
    assert!(
        hidden < shown,
        "faint while hidden: {hidden} against {shown}"
    );

    assert_eq!(
        hit_shape(&document(true), viewport, pos2(230.0, 160.0)),
        Some(0),
        "the end of the lettering picks it"
    );
    assert_eq!(
        hit_shape(&document(true), viewport, pos2(270.0, 160.0)),
        None
    );
}

/// An attribute is lettered as a new instance letters it, at its true size,
/// and picked anywhere on that text: the end of a long value as surely as
/// its start, and nothing past it.
#[test]
fn an_attribute_is_lettered_at_true_size_and_picked_on_its_text() {
    use rspice_design::schematic::label_layout::{INSTANCE_NAME_WORLD, INSTANCE_VALUE_WORLD};

    let viewport = SymbolViewport::new(
        Rect::from_min_size(Pos2::ZERO, vec2(480.0, 320.0)),
        4.0,
        Vec2::ZERO,
    );
    let document = SymbolDocument {
        name_anchor: Point::new(0, -30),
        value_anchor: Point::new(0, 30),
        ..SymbolDocument::default()
    };
    let mut metadata = SymbolEditorMetadata::for_document(&document);
    for attribute in &mut metadata.attributes {
        if attribute.kind == SymbolAttributeKind::Value {
            attribute.default_value = "OPA_LONG_VALUE".to_owned();
        }
    }
    let session = SymbolEditorSession::default();
    let lettering = |kind| {
        let attribute = metadata
            .attributes
            .iter()
            .find(|attribute| attribute.kind == kind)
            .unwrap();
        attribute_lettering(&document, attribute, &session, viewport).unwrap()
    };

    let reference = lettering(SymbolAttributeKind::Reference);
    assert_eq!(reference.text, "U1");
    assert_eq!(reference.font.size, INSTANCE_NAME_WORLD * 4.0);
    let value = lettering(SymbolAttributeKind::Value);
    assert_eq!(value.text, "OPA_LONG_VALUE");
    assert_eq!(value.font.size, INSTANCE_VALUE_WORLD * 4.0);

    let rect = value.rect();
    assert!(rect.width() > 200.0, "{rect:?}");
    let hit = |pos| hit_label(&document, &metadata, &session, viewport, pos);
    assert_eq!(
        hit(pos2(rect.right() - 3.0, rect.center().y)),
        Some(SymbolAttributeKind::Value)
    );
    assert_eq!(
        hit(pos2(rect.left() + 3.0, rect.center().y)),
        Some(SymbolAttributeKind::Value)
    );
    assert_eq!(hit(pos2(rect.right() + 30.0, rect.center().y)), None);
    assert_eq!(
        hit(reference.rect().center()),
        Some(SymbolAttributeKind::Reference)
    );
}

#[test]
fn a_new_instances_reference_and_value_stand_in_for_the_defaults() {
    assert_eq!(reference_sample("R?"), "R1");
    assert_eq!(reference_sample("U?"), "U1");
    assert_eq!(reference_sample(""), "X1");
    assert_eq!(value_sample("VALUE", Some("amp")), "amp");
    assert_eq!(value_sample("", Some("amp")), "amp");
    assert_eq!(value_sample("10k", Some("amp")), "10k");
    assert_eq!(value_sample("VALUE", None), "VALUE");
}

/// Rotating a selection turns a text run in it with the drawing, standing a
/// run read across up to read upward.
#[test]
fn rotating_a_selection_turns_the_text_in_it() {
    use super::commands::{SymbolEditAction, SymbolTransform, apply_edit};
    use super::interaction::SymbolEditCapabilities;
    use rspice_design::symbol::{SymbolTextRotation, SymbolTextSize};

    let mut document = SymbolDocument {
        body: vec![SymbolShape::Text {
            anchor: Point::new(20, 0),
            text: "OTA".to_owned(),
            size: SymbolTextSize::Normal,
            align: SymbolTextAlign::Left,
            rotation: SymbolTextRotation::R0,
        }],
        ..SymbolDocument::default()
    };
    let mut metadata = SymbolEditorMetadata::for_document(&document);
    let mut session = SymbolEditorSession::default();
    session.select_shape(0);

    let outcome = apply_edit(
        &mut session,
        &mut document,
        Some(&mut metadata),
        SymbolEditAction::Transform(SymbolTransform::Rotate),
        SymbolEditCapabilities { edit: true },
    );

    assert!(outcome.changed);
    assert!(
        matches!(
            &document.body[0],
            SymbolShape::Text {
                rotation: SymbolTextRotation::R90,
                ..
            }
        ),
        "{:?}",
        document.body[0]
    );
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
