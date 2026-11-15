//! Compiling an authored symbol view into printable scene primitives.
//!
//! One symbol reaches a page two ways — as the cellview itself, and as an
//! instance placed on a schematic — and both come through here, so a printed
//! instance can never carry different artwork from the printed master.
//!
//! Authored text is one of the body's shapes, so it prints with the artwork
//! it letters, hung off its anchor as the canvas hangs it.

use super::*;
use rspice_design::schematic::component_display::SchematicParameterLabelVisibility;
use rspice_design::schematic::label_layout::{LayoutPoint, PlacedText};

impl SemanticSceneCompiler<'_> {
    /// What an imported wire draws in place of a line: strokes in the
    /// sheet's symbol ink, and its name.
    pub(super) fn wire_bridge(
        &mut self,
        first: SchematicPoint,
        bridge: &rspice_design::schematic::wire::WireBridge,
    ) -> Result<(), HardcopyRenderError> {
        let stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:schematic-components",
            StrokeStyle {
                color: SemanticColor::Exact(super::schematic_marks::SYMBOL_INK),
                width: super::schematic_marks::width_um(drawing_metrics::SYMBOL_WIDTH_WORLD),
                ..StrokeStyle::default()
            },
        );
        for [from, to] in &bridge.lines {
            let points = vec![
                self.schematic_point(first + *from)?,
                self.schematic_point(first + *to)?,
            ];
            self.primitives.push(ScenePrimitive::Polyline {
                points,
                closed: false,
                stroke,
                fill: None,
            });
        }
        for curve in &bridge.curves {
            let placed = curve.placed(first, SchematicPoint::new(1, 0), SchematicPoint::new(0, 1));
            self.symbol_curve(&placed, None, stroke)?;
        }
        if let Some(text) = crate::sources::wire_bridge_text(first, bridge) {
            self.schematic_text(&text)?;
        }
        Ok(())
    }

    pub(in crate::render) fn symbol_document(
        &mut self,
        symbol: &SymbolDocument,
        component: Option<&Component>,
    ) -> Result<(), HardcopyRenderError> {
        let body_stable_id = if component.is_some() {
            "layer:schematic-components"
        } else {
            "layer:symbol-body"
        };
        let pin_stable_id = if component.is_some() {
            "layer:schematic-components"
        } else {
            "layer:symbol-pins"
        };
        // An instance on a sheet takes the sheet's symbol ink, as the SVG
        // export draws it; a printed symbol view keeps its own.
        let body_fallback = if component.is_some() {
            StrokeStyle {
                color: SemanticColor::Exact(super::schematic_marks::SYMBOL_INK),
                width: super::schematic_marks::width_um(drawing_metrics::SYMBOL_WIDTH_WORLD),
                ..StrokeStyle::default()
            }
        } else {
            StrokeStyle::default()
        };
        let body_stroke = self.mapped_stroke(PrintObjectKind::Layer, body_stable_id, body_fallback);
        let pin_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            pin_stable_id,
            StrokeStyle::try_new(
                SemanticColor::Accent,
                Length::from_micrometres(220),
                StrokePattern::Solid,
                None,
            )?,
        );
        // The symbol is held by its origin: an instance stands that point on
        // its position, as the canvas and the wires it connects do.
        let held = |point: SchematicPoint| point - symbol.origin;
        let world = |point: SchematicPoint| {
            if let Some(component) = component {
                let local = component.transform_point(held(point));
                SchematicPoint::new(
                    component.pos.x.saturating_add(local.x),
                    component.pos.y.saturating_add(local.y),
                )
            } else {
                held(point)
            }
        };
        let convert = |this: &Self, point: SchematicPoint| this.schematic_point(world(point));
        for shape in &symbol.body {
            let style = shape.style().unwrap_or_default();
            let stroke = styled_print_stroke(body_stroke, style);
            // The inside takes the outline's ink, through the same mapping a
            // printed dot's does.
            let fill = if shape.is_filled() {
                self.mapped_fill(
                    PrintObjectKind::Layer,
                    body_stable_id,
                    Some(SceneFill::solid(body_stroke.color)),
                )
            } else {
                None
            };
            match shape {
                SymbolShape::Polyline { points, closed, .. } => {
                    if points.len() >= if *closed { 3 } else { 2 } {
                        self.primitives.push(ScenePrimitive::Polyline {
                            points: points
                                .iter()
                                .copied()
                                .map(|point| convert(self, point))
                                .collect::<Result<_, _>>()?,
                            closed: *closed,
                            stroke,
                            fill,
                        });
                    }
                }
                SymbolShape::Rectangle { min, max, .. } => {
                    self.primitives.push(ScenePrimitive::Polyline {
                        points: rectangle_corners(*min, *max)
                            .into_iter()
                            .map(|point| convert(self, point))
                            .collect::<Result<_, _>>()?,
                        closed: true,
                        stroke,
                        fill,
                    });
                }
                SymbolShape::Circle { center, radius, .. } => {
                    self.primitives.push(ScenePrimitive::Circle {
                        center: convert(self, *center)?,
                        radius: Length::from_micrometres(
                            u64::from(radius.unsigned_abs()) * SCHEMATIC_UNIT_UM as u64,
                        ),
                        stroke: Some(stroke),
                        fill,
                    });
                }
                SymbolShape::Dot { center, radius } => {
                    self.primitives.push(ScenePrimitive::Circle {
                        center: convert(self, *center)?,
                        radius: Length::from_micrometres(
                            u64::from(radius.unsigned_abs()) * SCHEMATIC_UNIT_UM as u64,
                        ),
                        stroke: None,
                        fill: self.mapped_fill(
                            PrintObjectKind::Layer,
                            body_stable_id,
                            Some(SceneFill::solid(body_stroke.color)),
                        ),
                    });
                }
                SymbolShape::Arc {
                    center,
                    radius,
                    start_degrees,
                    sweep_degrees,
                    ..
                } => {
                    let arc =
                        SymbolCurve::of_arc(held(*center), *radius, *start_degrees, *sweep_degrees);
                    self.symbol_curve(&arc, component, stroke)?;
                }
                SymbolShape::Curve { curve, .. } => {
                    let mut curve = *curve;
                    curve.center[0] -= f64::from(symbol.origin.x);
                    curve.center[1] -= f64::from(symbol.origin.y);
                    self.symbol_curve(&curve, component, stroke)?;
                }
                SymbolShape::Arrow {
                    tip,
                    rotation_quarters,
                } => {
                    self.primitives.push(ScenePrimitive::Polyline {
                        points: rspice_design::symbol::arrow_corners(*tip, *rotation_quarters)
                            .into_iter()
                            .map(|point| convert(self, point))
                            .collect::<Result<_, _>>()?,
                        closed: true,
                        stroke: body_stroke,
                        fill: self.mapped_fill(
                            PrintObjectKind::Layer,
                            body_stable_id,
                            Some(SceneFill::solid(body_stroke.color)),
                        ),
                    });
                }
                SymbolShape::Text {
                    anchor,
                    text,
                    size,
                    align,
                    rotation,
                } => {
                    self.placed_lines(
                        placed_run(world(*anchor), *align, *rotation, component),
                        text,
                        SceneFont::Monospace,
                        size.height() as f32,
                        body_stroke.color,
                    )?;
                }
                // An instance letters its own value of the parameter, or the
                // default where it sets none, in its value's ink as the canvas
                // letters it; the symbol printed alone letters the default in
                // its body's ink.
                SymbolShape::Label {
                    anchor,
                    parameter,
                    default,
                    size,
                    align,
                    rotation,
                    shown,
                } => {
                    // A part that hides its value hides the labels that
                    // letter its values, as the canvas hides them.
                    let value_hidden = component.is_some_and(|component| {
                        !component
                            .display_mode
                            .show_value(SchematicParameterLabelVisibility::NamesAndValues)
                    });
                    if !*shown || value_hidden {
                        continue;
                    }
                    let value = component
                        .and_then(|component| {
                            rspice_design::parameters::parse_params_string(&component.params)
                                .remove(&parameter.to_lowercase())
                        })
                        .unwrap_or_else(|| default.clone());
                    if value.is_empty() {
                        continue;
                    }
                    let ink = if component.is_some() {
                        SemanticColor::Exact(super::schematic_text::VALUE_INK)
                    } else {
                        body_stroke.color
                    };
                    self.placed_lines(
                        placed_run(world(*anchor), *align, *rotation, component),
                        &rspice_design::symbol::symbol_label_text(parameter, &value),
                        SceneFont::Monospace,
                        size.height() as f32,
                        ink,
                    )?;
                }
            }
        }
        for pin in &symbol.pins {
            let Some(position) = pin.position else {
                continue;
            };
            // A pin's lead and the marks its style puts where the lead meets
            // the body print in the body's ink: one symbol weight.
            let drawing =
                rspice_design::symbol::pin_drawing(position, pin.side, pin.length, pin.style);
            if let Some((from, to)) = drawing.lead {
                self.primitives.push(ScenePrimitive::Line {
                    from: convert(self, from)?,
                    to: convert(self, to)?,
                    stroke: body_stroke,
                });
            }
            if let Some((centre, radius)) = drawing.bubble {
                self.primitives.push(ScenePrimitive::Circle {
                    center: convert(self, centre)?,
                    radius: Length::from_micrometres(
                        u64::from(radius.unsigned_abs()) * SCHEMATIC_UNIT_UM as u64,
                    ),
                    stroke: Some(body_stroke),
                    fill: None,
                });
            }
            if let Some(wedge) = drawing.wedge {
                self.primitives.push(ScenePrimitive::Polyline {
                    points: wedge
                        .into_iter()
                        .map(|point| convert(self, point))
                        .collect::<Result<_, _>>()?,
                    closed: false,
                    stroke: body_stroke,
                    fill: None,
                });
            }
            let position = convert(self, position)?;
            self.primitives.push(ScenePrimitive::Circle {
                center: position,
                radius: Length::from_micrometres(500),
                stroke: Some(pin_stroke),
                fill: None,
            });
            if component.is_none() {
                let label = self.offset_scene_point(position, 1_000, -800)?;
                self.add_text(label, &pin.name, SceneFont::Sans, 2_300, pin_stroke.color)?;
            }
        }
        if let Some(component) = component {
            self.symbol_pin_names(symbol, component)?;
        }
        self.add_mapping_legend(PrintObjectKind::Layer, body_stable_id, body_stroke)?;
        if pin_stable_id != body_stable_id {
            self.add_mapping_legend(PrintObjectKind::Layer, pin_stable_id, pin_stroke)?;
        }
        Ok(())
    }

    /// One curve of a symbol, placed unrounded and printed as a true curve.
    fn symbol_curve(
        &mut self,
        curve: &SymbolCurve,
        component: Option<&Component>,
        stroke: StrokeStyle,
    ) -> Result<(), HardcopyRenderError> {
        let placed = component.map_or(*curve, |component| {
            let origin = component.transform_point(SchematicPoint::origin());
            curve.placed(
                component.pos + origin,
                component.transform_point(SchematicPoint::new(1, 0)) - origin,
                component.transform_point(SchematicPoint::new(0, 1)) - origin,
            )
        });
        let micrometres = |value: f64| value * SCHEMATIC_UNIT_UM as f64;
        let center = self.signed_micrometre_point(
            micrometres(placed.center[0]).round() as i64,
            micrometres(placed.center[1]).round() as i64,
        )?;
        let length = |value: f64| Length::from_micrometres(micrometres(value).round() as u64);
        self.primitives.push(ScenePrimitive::Ellipse {
            center,
            radius_x: length(placed.radii[0]),
            radius_y: length(placed.radii[1]),
            start_millidegrees: (placed.start_degrees * 1000.0).round() as i32,
            sweep_millidegrees: (placed.sweep_degrees * 1000.0).round() as i32,
            stroke,
        });
        Ok(())
    }
}

/// How a run of the symbol's lettering hangs off its world `anchor` once
/// `component` (if any) has carried it: as the canvas hangs it, a run read
/// upward hung upright and turned a quarter counter-clockwise about its
/// anchor.
fn placed_run(
    anchor: SchematicPoint,
    align: SymbolTextAlign,
    rotation: SymbolTextRotation,
    component: Option<&Component>,
) -> PlacedText {
    let placement = align.placement(rotation, |point| {
        component.map_or(point, |c| c.transform_point(point))
    });
    let (horizontal, vertical) = rspice_design::symbol::symbol_text_hang(placement, rotation);
    PlacedText {
        anchor: LayoutPoint::new(anchor.x as f32, anchor.y as f32),
        horizontal,
        vertical,
        upward: matches!(rotation, SymbolTextRotation::R90),
    }
}

/// `stroke` weighted and broken as `style` says. A whole line keeps the
/// pattern the print mapping gave the body's layer; a broken one is broken
/// in symbol units, so the pattern keeps its proportion against the body on
/// every page size, as it does on screen.
///
/// Paper caps every dash round, half the line's width past each end, where
/// the canvas ends a dash square and draws a dot one width across. A dash is
/// set short by the width and a dot is a dash of next to nothing, so the
/// page shows the canvas's pattern at every weight.
fn styled_print_stroke(stroke: StrokeStyle, style: SymbolStyle) -> StrokeStyle {
    /// The dash a round cap turns into a dot one line width across.
    const DOT_UM: f64 = 1.0;
    let scaled = (stroke.width.micrometres() as f64 * f64::from(style.stroke.scale())).round();
    let width = Length::from_micrometres((scaled as u64).max(1));
    let Some((drawn, skipped)) = style.dash.pattern() else {
        return StrokeStyle { width, ..stroke };
    };
    let units = |value: f32| f64::from(value) * SCHEMATIC_UNIT_UM as f64;
    let step = units(drawn + skipped);
    let (pattern, dash) = match style.dash {
        SymbolDash::Dotted => (StrokePattern::Dotted, DOT_UM),
        _ => (
            StrokePattern::Dashed,
            (units(drawn) - width.micrometres() as f64).max(DOT_UM),
        ),
    };
    let length = |value: f64| Length::from_micrometres(value.round() as u64);
    StrokeStyle {
        width,
        pattern,
        exact_dash: Some((length(dash), length(step - dash))),
        exact_dot_spacing: None,
        ..stroke
    }
}
