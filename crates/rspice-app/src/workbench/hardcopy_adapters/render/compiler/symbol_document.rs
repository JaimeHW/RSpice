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
    pub(in crate::workbench::hardcopy_adapters::render) fn symbol_document(
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
        let body_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            body_stable_id,
            StrokeStyle::default(),
        );
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
                    let count = (sweep_degrees.unsigned_abs() / 5).clamp(8, 144) as usize;
                    let mut points = Vec::with_capacity(count + 1);
                    for index in 0..=count {
                        let angle = f64::from(*start_degrees)
                            + f64::from(*sweep_degrees) * index as f64 / count as f64;
                        let radians = angle.to_radians();
                        points.push(
                            SchematicPoint::new(
                                center.x.saturating_add(
                                    (f64::from(*radius) * radians.cos()).round() as i32,
                                ),
                                center.y.saturating_add(
                                    (f64::from(*radius) * radians.sin()).round() as i32,
                                ),
                            ),
                        );
                    }
                    self.primitives.push(ScenePrimitive::Polyline {
                        points: points
                            .into_iter()
                            .map(|point| convert(self, point))
                            .collect::<Result<_, _>>()?,
                        closed: false,
                        stroke: body_stroke,
                        fill: None,
                    });
                }
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
                        placed,
                        SceneTextRotation::Upright,
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
        self.add_mapping_legend(PrintObjectKind::Layer, body_stable_id, body_stroke)?;
        if pin_stable_id != body_stable_id {
            self.add_mapping_legend(PrintObjectKind::Layer, pin_stable_id, pin_stroke)?;
        }
        Ok(())
    }
}
