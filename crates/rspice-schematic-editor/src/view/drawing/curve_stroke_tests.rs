//! An imported part is drawn with RSpice's own pen: its curves show no
//! straight piece at any size, and its strokes are as wide as a native part's.

use egui::{Context, Rect, Shape as PaintedShape, pos2, vec2};
use rspice_design::{
    schematic::component_display::{ComponentDisplayMode, SchematicParameterLabelVisibility},
    symbol::{SymbolCurve, SymbolDocument, SymbolPin, SymbolShape},
};
use rspice_design_model::port::PortDirection;
use rspice_ui_kit::{Theme, raster::display_input, tokens::Tokens};

use super::*;
use crate::view::symbol_context::SchematicSymbolContext;

const SIZE: Vec2 = vec2(900.0, 900.0);

fn imported_inductor() -> Component {
    let pin = |name: &str, y: i32| {
        SymbolPin::new(name, PortDirection::InOut, Some(Point::new(0, y))).with_name_hidden(true)
    };
    let hump = |y: f64| {
        SymbolShape::curve(SymbolCurve {
            center: [0.0, y],
            radii: [10.0, 10.0],
            start_degrees: 200.0,
            sweep_degrees: 225.0,
        })
    };
    let mut inductor = Component::new(1, ComponentType::Inductor, Point::new(20, 20));
    inductor.display_mode = ComponentDisplayMode::Hidden;
    inductor.set_source_symbol(SymbolDocument {
        pins: vec![pin("+", 0), pin("-", 60)],
        body: vec![
            SymbolShape::Polyline {
                points: vec![Point::new(0, 0), Point::new(-10, 15)],
                closed: false,
                style: Default::default(),
            },
            hump(15.0),
            hump(30.0),
            hump(45.0),
        ],
        ..SymbolDocument::default()
    });
    inductor
}

/// Every shape `component` paints at `zoom` on a display of `ppp`.
fn painted(component: &Component, zoom: f32, ppp: f32) -> Vec<PaintedShape> {
    let library = SymbolLibrary::load_embedded().expect("the symbols load");
    let context = SchematicSymbolContext::default();
    let palette = Tokens::default().color;
    let viewport = Viewport {
        offset: pos2(40.0, 40.0),
        zoom,
        bounds: Rect::from_min_size(Pos2::ZERO, SIZE),
    };
    let ctx = Context::default();
    Theme::default().apply(&ctx);
    let mut shapes = Vec::new();
    let _ = ctx.run_ui(display_input(SIZE, ppp), |ui| {
        let painter = ui.painter().clone();
        draw_component(
            &painter,
            &viewport,
            component,
            false,
            Some(&library),
            &context,
            SchematicParameterLabelVisibility::Hidden,
            &palette,
        );
        shapes.clear();
        painter.for_each_shape(|shape| shapes.push(shape.shape.clone()));
    });
    shapes
}

#[test]
fn an_imported_inductor_shows_no_straight_piece_at_any_zoom() {
    let inductor = imported_inductor();
    for (zoom, ppp) in [(2.0, 1.0), (8.0, 1.0), (2.0, 2.0), (8.0, 1.5)] {
        let humps = painted(&inductor, zoom, ppp)
            .into_iter()
            .filter_map(|shape| match shape {
                PaintedShape::Path(path) if path.points.len() > 4 => Some(path.points),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(humps.len(), 3, "three humps at {zoom}x on {ppp}");
        for points in humps {
            for piece in points.windows(2) {
                let device = piece[0].distance(piece[1]) * ppp;
                assert!(device <= 3.0, "{device} device px at {zoom}x on {ppp}");
            }
        }
    }
}
