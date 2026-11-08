//! Compiling an authored symbol view into printable scene primitives.
//!
//! One symbol reaches a page two ways — as the cellview itself, and as an
//! instance placed on a schematic — and both come through here, so a printed
//! instance can never carry different artwork from the printed master.
//!
//! Authored text is one of the body's shapes, so it prints with the artwork
//! it letters. The scene sets a run horizontally only, which is why an
//! orientation that stands one on end centres it on its anchor instead.

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
            match shape {
                SymbolShape::Polyline { points, closed } => {
                    if points.len() >= if *closed { 3 } else { 2 } {
                        self.primitives.push(ScenePrimitive::Polyline {
                            points: points
                                .iter()
                                .copied()
                                .map(|point| convert(self, point))
                                .collect::<Result<_, _>>()?,
                            closed: *closed,
                            stroke: body_stroke,
                            fill: None,
                        });
                    }
                }
                SymbolShape::Circle { center, radius } => {
                    self.primitives.push(ScenePrimitive::Circle {
                        center: convert(self, *center)?,
                        radius: Length::from_micrometres(
                            u64::from(radius.unsigned_abs()) * SCHEMATIC_UNIT_UM as u64,
                        ),
                        stroke: Some(body_stroke),
                        fill: None,
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
                } => {
                    let arc = SymbolCurve::of_arc(*center, *radius, *start_degrees, *sweep_degrees);
                    self.symbol_curve(&arc, component, body_stroke)?;
                }
                SymbolShape::Curve(curve) => self.symbol_curve(curve, component, body_stroke)?,
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
                } => {
                    let origin = convert(self, *anchor)?;
                    // The scene places a run horizontally only, so an
                    // orientation that stood it on end centres it instead of
                    // hanging it off a side the print cannot express.
                    let placed = match align
                        .placement(|point| component.map_or(point, |c| c.transform_point(point)))
                    {
                        SymbolTextPlacement::After => TextAnchor::Start,
                        SymbolTextPlacement::Before => TextAnchor::End,
                        SymbolTextPlacement::On
                        | SymbolTextPlacement::Below
                        | SymbolTextPlacement::Above => TextAnchor::Middle,
                    };
                    self.add_text_placed(
                        origin,
                        text,
                        SceneFont::Monospace,
                        u64::from(size.height().unsigned_abs()) * SCHEMATIC_UNIT_UM as u64,
                        body_stroke.color,
                        (placed, SceneTextRotation::Upright),
                    )?;
                }
            }
        }
        for pin in &symbol.pins {
            let Some(position) = pin.position else {
                continue;
            };
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
