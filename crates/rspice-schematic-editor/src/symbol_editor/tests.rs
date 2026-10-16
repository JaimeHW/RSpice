//! Existing symbol canvas geometry and viewport regressions.

use super::*;

/// The symbol editor's two surfaces, written to PNGs for a human to look at:
/// the inspector with one pin selected, and the canvas with its check note
/// and its unpublished-changes strip.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn print_symbol_editor_renders_for_review() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();

    let reference = CellViewRef::new("work", "amp", "symbol");
    // One bus pin selected, one pin the schematic does not declare, and one
    // declared port with no pin: every row the panel can show at once.
    let mut document = SymbolDocument::default();
    document.body.push(SymbolShape::Polyline {
        points: vec![
            Point::new(-40, -30),
            Point::new(40, -30),
            Point::new(40, 30),
            Point::new(-40, 30),
        ],
        closed: true,
    });
    document.pins = vec![
        SymbolPin::new("DATA[3:0]", PortDirection::In, Some(Point::new(-40, -10))),
        SymbolPin::new("SPARE", PortDirection::Out, Some(Point::new(40, 10))),
    ];
    let metadata = SymbolEditorMetadata::for_document(&document);
    let ports = [
        PortSpec {
            name: "DATA[3:0]".to_owned(),
            direction: PortDirection::In,
        },
        PortSpec {
            name: "EN".to_owned(),
            direction: PortDirection::In,
        },
    ];
    let source = interaction::SymbolRequestSource {
        project: rspice_app_types::product::ProjectId::new(),
        document: reference.clone(),
        occurrence: None,
        design_epoch: 1,
        document_epoch: 1,
        library_revision: 1,
        window: 1,
    };

    let mut session = SymbolEditorSession::default();
    session.select_pin("DATA[3:0]");
    interaction::bind_canvas_source(&mut session, source.clone());

    let inspector = rspice_ui_kit::raster::render_themed(
        rspice_ui_kit::Theme::default(),
        vec2(360.0, 620.0),
        |ui, _background| {
            rspice_ui_kit::panels::panel_body(ui, |ui| {
                let _ = inspector::show(
                    ui,
                    source.clone(),
                    document.clone(),
                    metadata.clone(),
                    &ports,
                    &session,
                    interaction::SymbolEditCapabilities { edit: true },
                );
            });
        },
    );
    let path = directory.join("symbol-inspector.png");
    let height = inspector.content_height().max(1);
    std::fs::write(&path, inspector.png(height)).expect("write inspector render");
    writeln!(
        report,
        "{} {}x{}",
        path.display(),
        inspector.width(),
        height
    )
    .expect("write raster report");

    let canvas = rspice_ui_kit::raster::render_themed(
        rspice_ui_kit::Theme::default(),
        vec2(980.0, 560.0),
        |ui, _background| {
            surface::unpublished_changes_strip(ui, true);
            let stage = ui.available_rect_before_wrap();
            let (rect, response) =
                ui.allocate_exact_size(stage.size(), egui::Sense::click_and_drag());
            let mut session = session.clone();
            let viewport = update_viewport(ui, &mut session, rect, &document, &response);
            draw_canvas(
                ui, viewport, &document, &metadata, &ports, &session, &reference,
            );
            surface::canvas_check_note(ui.ctx(), &document, &ports, stage);
        },
    );
    let path = directory.join("symbol-canvas.png");
    let height = canvas.content_height().max(1);
    std::fs::write(&path, canvas.png(height)).expect("write canvas render");
    writeln!(report, "{} {}x{}", path.display(), canvas.width(), height)
        .expect("write raster report");
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
    let state = SymbolEditorSession::default();

    assert_eq!(state.grid_spacing, SymbolGridSpacing::Ten);
    assert_eq!(
        SymbolGridSpacing::TwoPointFive.label(),
        "grid 2.5 \u{00b7} fine"
    );
}
