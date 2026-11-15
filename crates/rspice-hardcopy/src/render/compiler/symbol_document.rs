//! Compiling an authored symbol view into printable scene primitives.
//!
//! One symbol reaches a page two ways — as the cellview itself, and as an
//! instance placed on a schematic — and both come through here, so a printed
//! instance can never carry different artwork from the printed master.
//!
//! Authored text is one of the body's shapes, so it prints with the artwork
//! it letters. The scene sets a run along its reading, across or upward, which
//! is why an orientation that carries a run across its reading centres it on
//! its anchor instead.

use super::*;

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
        let convert = |this: &Self, point: SchematicPoint| {
            let world = if let Some(component) = component {
                let local = component.transform_point(point);
                SchematicPoint::new(
                    component.pos.x.saturating_add(local.x),
                    component.pos.y.saturating_add(local.y),
                )
            } else {
                point
            };
            this.schematic_point(world)
        };
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
                    let arc = SymbolCurve::of_arc(*center, *radius, *start_degrees, *sweep_degrees);
                    self.symbol_curve(&arc, component, stroke)?;
                }
                SymbolShape::Curve { curve, .. } => self.symbol_curve(curve, component, stroke)?,
                SymbolShape::Arrow {
                    tip,
                    rotation_quarters,
                } => {
                    let direction = rotation_quarters.rem_euclid(4);
                    let mut local = [
                        SchematicPoint::new(0, 0),
                        SchematicPoint::new(-8, -4),
                        SchematicPoint::new(-8, 4),
                    ];
                    for point in &mut local {
                        for _ in 0..direction {
                            *point = SchematicPoint::new(-point.y, point.x);
                        }
                        point.x = point.x.saturating_add(tip.x);
                        point.y = point.y.saturating_add(tip.y);
                    }
                    self.primitives.push(ScenePrimitive::Polyline {
                        points: local
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
                    let origin = convert(self, *anchor)?;
                    self.add_text_placed(
                        origin,
                        text,
                        SceneFont::Monospace,
                        u64::from(size.height().unsigned_abs()) * SCHEMATIC_UNIT_UM as u64,
                        body_stroke.color,
                        placed_run(*align, *rotation, component),
                    )?;
                }
                // An instance letters its own value of the parameter, or the
                // default where it sets none; the symbol printed alone
                // letters the default.
                SymbolShape::Label {
                    anchor,
                    parameter,
                    default,
                    size,
                    align,
                    rotation,
                    shown,
                } => {
                    let value = component
                        .and_then(|component| {
                            rspice_design::parameters::parse_params_string(&component.params)
                                .remove(&parameter.to_lowercase())
                        })
                        .unwrap_or_else(|| default.clone());
                    if !*shown || value.is_empty() {
                        continue;
                    }
                    let origin = convert(self, *anchor)?;
                    self.add_text_placed(
                        origin,
                        &rspice_design::symbol::symbol_label_text(parameter, &value),
                        SceneFont::Monospace,
                        u64::from(size.height().unsigned_abs()) * SCHEMATIC_UNIT_UM as u64,
                        body_stroke.color,
                        placed_run(*align, *rotation, component),
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

/// `stroke` weighted and broken as `style` says. A whole line keeps the
/// pattern the print mapping gave the body's layer; a broken one is broken
/// in symbol units, so the pattern keeps its proportion against the body on
/// every page size, as it does on screen.
/// Where the scene sets a run from and which way it reads, once `component`
/// (if any) has carried it. The scene sets a run along its reading only, so
/// an orientation that carried the run across that reading centres it
/// instead of hanging it off a side the print cannot express.
fn placed_run(
    align: SymbolTextAlign,
    rotation: SymbolTextRotation,
    component: Option<&Component>,
) -> (TextAnchor, SceneTextRotation) {
    let placement = align.placement(rotation, |point| {
        component.map_or(point, |c| c.transform_point(point))
    });
    let placed = match (rotation, placement) {
        (SymbolTextRotation::R0, SymbolTextPlacement::After)
        | (SymbolTextRotation::R90, SymbolTextPlacement::Above) => TextAnchor::Start,
        (SymbolTextRotation::R0, SymbolTextPlacement::Before)
        | (SymbolTextRotation::R90, SymbolTextPlacement::Below) => TextAnchor::End,
        _ => TextAnchor::Middle,
    };
    let turn = match rotation {
        SymbolTextRotation::R0 => SceneTextRotation::Upright,
        SymbolTextRotation::R90 => SceneTextRotation::CounterClockwise90,
    };
    (placed, turn)
}

fn styled_print_stroke(stroke: StrokeStyle, style: SymbolStyle) -> StrokeStyle {
    let scaled = (stroke.width.micrometres() as f64 * f64::from(style.stroke.scale())).round();
    let width = Length::from_micrometres((scaled as u64).max(1));
    let units = |value: f32| {
        Length::from_micrometres((f64::from(value) * SCHEMATIC_UNIT_UM as f64).round() as u64)
    };
    match (style.dash, style.dash.pattern()) {
        (SymbolDash::Dotted, Some((_, skipped))) => StrokeStyle {
            width,
            pattern: StrokePattern::Dotted,
            exact_dash: None,
            exact_dot_spacing: Some(units(skipped)),
            ..stroke
        },
        (_, Some((drawn, skipped))) => StrokeStyle {
            width,
            pattern: StrokePattern::Dashed,
            exact_dash: Some((units(drawn), units(skipped))),
            exact_dot_spacing: None,
            ..stroke
        },
        (_, None) => StrokeStyle { width, ..stroke },
    }
}
