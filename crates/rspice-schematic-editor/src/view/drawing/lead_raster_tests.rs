//! A wire and the pin lead it meets are one straight line on the raster: the
//! same device pixels across, none of them partly painted, at every pixel
//! density and zoom a display meets and wherever the pan leaves the part.

use egui::{Color32, Context, Rect, pos2, vec2};
use rspice_design::{
    schematic::component_display::{ComponentDisplayMode, SchematicParameterLabelVisibility},
    symbol::{SymbolDocument, SymbolPin, SymbolShape},
};
use rspice_design_model::port::PortDirection;
use rspice_ui_kit::{
    Theme,
    raster::{Canvas, display_input, rasterize},
    tokens::Tokens,
};

use super::*;
use crate::view::symbol_context::SchematicSymbolContext;

const SIZE: Vec2 = vec2(420.0, 420.0);
const DENSITIES: [f32; 4] = [1.0, 1.25, 1.5, 2.0];
const ZOOMS: [f32; 6] = [0.75, 1.0, 1.37, 1.75, 2.0, 2.45];
const PANS: [f32; 4] = [0.0, 0.25, 0.5, 0.77];

/// `component` with a wire leaving its `terminal` straight out, rendered.
fn render(
    component: &Component,
    terminal: Point,
    outward: (i32, i32),
    viewport: &Viewport,
    ppp: f32,
) -> Canvas {
    let library = SymbolLibrary::load_embedded().expect("the symbols load");
    let context = SchematicSymbolContext::default();
    let palette = Tokens::default().color;
    let far = terminal + Point::new(outward.0 * 40, outward.1 * 40);
    let ctx = Context::default();
    Theme::default().apply(&ctx);
    let mut output = None;
    for _ in 0..3 {
        output = Some(ctx.run_ui(display_input(SIZE, ppp), |ui| {
            let painter = ui.painter().clone();
            draw_conductor(
                &painter,
                viewport,
                &[terminal, far],
                ConductorInk::Plain,
                &palette,
            );
            draw_component(
                &painter,
                viewport,
                component,
                false,
                Some(&library),
                &context,
                SchematicParameterLabelVisibility::Hidden,
                &palette,
            );
        }));
    }
    rasterize(&ctx, SIZE, &output.expect("three frames"))
}

/// The device pixels a straight stroke paints across at `along`, near
/// `around`; none may be partly painted.
fn painted_across(
    canvas: &Canvas,
    upright: bool,
    along: usize,
    around: usize,
    what: &str,
) -> Vec<usize> {
    let background = canvas.background();
    let ink = |colour: Color32| {
        (i32::from(colour.r()) - i32::from(background.r()))
            .abs()
            .max((i32::from(colour.g()) - i32::from(background.g())).abs())
            .max((i32::from(colour.b()) - i32::from(background.b())).abs())
    };
    let across = around.saturating_sub(10)..around + 10;
    let inks = across
        .clone()
        .map(|place| {
            let (x, y) = if upright {
                (place, along)
            } else {
                (along, place)
            };
            ink(canvas.pixel(x, y).expect("on the canvas"))
        })
        .collect::<Vec<_>>();
    let full = inks.iter().copied().max().unwrap_or_default();
    assert!(full > 20, "{what}: nothing is painted: {inks:?}");
    assert!(
        inks.iter().all(|ink| *ink <= 3 || *ink >= full - 3),
        "{what} smears: {inks:?}"
    );
    across
        .zip(inks)
        .filter(|(_, ink)| *ink > 3)
        .map(|(place, _)| place)
        .collect()
}

/// Every density, zoom and pan: the lead that meets the wire at the part's
/// first terminal lies on the wire's own pixels.
fn assert_lead_lies_on_its_wire(name: &str, component: &Component, outward: (i32, i32)) {
    let terminal = component
        .terminal_positions()
        .into_iter()
        .map(|(_, point)| point)
        .min_by_key(|point| (point.x * outward.0 + point.y * outward.1) * -1)
        .expect("a terminal");
    let upright = outward.0 == 0;
    for ppp in DENSITIES {
        for zoom in ZOOMS {
            for pan in PANS {
                let viewport = Viewport {
                    offset: pos2(80.0 + pan, 90.0 + pan),
                    zoom,
                    bounds: Rect::from_min_size(Pos2::ZERO, SIZE),
                };
                let canvas = render(component, terminal, outward, &viewport, ppp);
                let at = viewport.schematic_to_screen(terminal);
                let device = |offset_world: i32| {
                    let along = if upright {
                        at.y + (offset_world * outward.1) as f32 * zoom
                    } else {
                        at.x + (offset_world * outward.0) as f32 * zoom
                    };
                    (along * ppp) as usize
                };
                let around = (if upright { at.x } else { at.y } * ppp) as usize;
                let case = format!("{name} at {ppp}x, zoom {zoom}, pan {pan}");
                let wire = painted_across(
                    &canvas,
                    upright,
                    device(20),
                    around,
                    &format!("the wire of {case}"),
                );
                let lead = painted_across(
                    &canvas,
                    upright,
                    device(-4),
                    around,
                    &format!("the lead of {case}"),
                );
                assert_eq!(lead, wire, "{case}");
            }
        }
    }
}

#[test]
fn a_native_glyphs_upright_lead_lies_on_its_wire() {
    let mut source = Component::new(1, ComponentType::VoltageSource, Point::new(40, 60));
    source.display_mode = ComponentDisplayMode::Hidden;
    assert_lead_lies_on_its_wire("a voltage source", &source, (0, -1));
}

#[test]
fn a_native_glyphs_level_lead_lies_on_its_wire() {
    let mut resistor = Component::new(1, ComponentType::Resistor, Point::new(60, 60));
    resistor.display_mode = ComponentDisplayMode::Hidden;
    assert_lead_lies_on_its_wire("a resistor", &resistor, (-1, 0));
}

#[test]
fn an_imported_symbols_lead_lies_on_its_wire() {
    let pin = |name: &str, y: i32| {
        SymbolPin::new(name, PortDirection::InOut, Some(Point::new(0, y))).with_name_hidden(true)
    };
    let stroke = |points: Vec<Point>| SymbolShape::Polyline {
        points,
        closed: false,
    };
    let mut resistor = Component::new(1, ComponentType::Resistor, Point::new(60, 60));
    resistor.display_mode = ComponentDisplayMode::Hidden;
    resistor.set_source_symbol(SymbolDocument {
        pins: vec![pin("+", -30), pin("-", 30)],
        body: vec![
            stroke(vec![Point::new(0, -30), Point::new(0, -15)]),
            stroke(vec![Point::new(0, 30), Point::new(0, 15), Point::new(9, 9)]),
            stroke(vec![Point::new(0, -15), Point::new(9, 9)]),
        ],
        ..SymbolDocument::default()
    });
    assert_lead_lies_on_its_wire("an imported resistor", &resistor, (0, -1));
}

/// A diagonal stroke ending on a terminal is artwork: only straight leads
/// take the wire's pixels.
#[test]
fn a_slanted_stroke_on_a_terminal_stays_artwork() {
    let painter = Painter::new(
        Context::default(),
        egui::LayerId::background(),
        Rect::from_min_size(Pos2::ZERO, SIZE),
    );
    let terminals = [pos2(10.0, 10.0)];
    let leads = PinLeads::new(&terminals, 3.0, 1.0);
    let stroke = Stroke::new(1.0, Color32::WHITE);
    let slanted = [pos2(10.0, 10.0), pos2(20.0, 25.0)];
    assert_eq!(leads.paint_leads(&painter, &slanted, stroke), &slanted);
    let upright = [pos2(10.0, 10.0), pos2(10.0, 25.0), pos2(20.0, 25.0)];
    assert_eq!(leads.paint_leads(&painter, &upright, stroke), &upright[1..]);
    let mut widths = Vec::new();
    painter.for_each_shape(|shape| {
        if let egui::Shape::LineSegment { stroke, .. } = &shape.shape {
            widths.push(stroke.width);
        }
    });
    assert_eq!(widths, [3.0]);
}
