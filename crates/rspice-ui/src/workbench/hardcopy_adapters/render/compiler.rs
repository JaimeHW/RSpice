//! Compiling a semantic document into printable scene primitives.
//!
//! The compiler is the only place model coordinates become page coordinates,
//! and it lays content out in exact micrometres rather than device pixels — so
//! the same document compiles to the same scene at any resolution. Text that
//! does not fit is wrapped or reported, never clipped silently, because a
//! truncated value on a printed page is indistinguishable from a real one.

use super::*;

pub(super) struct SemanticSceneCompiler<'a> {
    pub(super) bounds: SemanticBounds,
    pub(super) extent: ContentExtent,
    pub(super) mapping: &'a PrintMappingTable,
    pub(super) primitives: Vec<ScenePrimitive>,
    pub(super) legend: Vec<LegendEntry>,
    pub(super) coordinate_offset: ScenePoint,
    pub(super) aggregate_sections: Vec<AggregateSection>,
    pub(super) mapping_ordinal: Option<u32>,
}

impl<'a> SemanticSceneCompiler<'a> {
    pub(super) fn new(
        bounds: SemanticBounds,
        extent: ContentExtent,
        mapping: &'a PrintMappingTable,
    ) -> Self {
        Self {
            bounds,
            extent,
            mapping,
            primitives: Vec::new(),
            legend: Vec::new(),
            coordinate_offset: ScenePoint::new(Length::ZERO, Length::ZERO),
            aggregate_sections: Vec::new(),
            mapping_ordinal: None,
        }
    }

    fn semantic_point(&self, point: SemanticPoint) -> Result<ScenePoint, HardcopyRenderError> {
        self.signed_micrometre_point(point.x_um, point.y_um)
    }

    fn schematic_point(&self, point: SchematicPoint) -> Result<ScenePoint, HardcopyRenderError> {
        let x = i64::from(point.x)
            .checked_mul(SCHEMATIC_UNIT_UM)
            .ok_or_else(|| conversion_error("schematic X coordinate overflow"))?;
        let y = i64::from(point.y)
            .checked_mul(SCHEMATIC_UNIT_UM)
            .ok_or_else(|| conversion_error("schematic Y coordinate overflow"))?;
        self.signed_micrometre_point(x, y)
    }

    fn signed_micrometre_point(&self, x: i64, y: i64) -> Result<ScenePoint, HardcopyRenderError> {
        let x = x
            .checked_sub(self.bounds.minimum.x_um)
            .and_then(|value| u64::try_from(value).ok())
            .ok_or_else(|| conversion_error("source X coordinate precedes frozen bounds"))?;
        let y = y
            .checked_sub(self.bounds.minimum.y_um)
            .and_then(|value| u64::try_from(value).ok())
            .ok_or_else(|| conversion_error("source Y coordinate precedes frozen bounds"))?;
        let x = x
            .checked_add(self.coordinate_offset.x.micrometres())
            .ok_or_else(|| conversion_error("aggregate X placement overflow"))?;
        let y = y
            .checked_add(self.coordinate_offset.y.micrometres())
            .ok_or_else(|| conversion_error("aggregate Y placement overflow"))?;
        if x > self.extent.width().micrometres() || y > self.extent.height().micrometres() {
            return Err(conversion_error(
                "source coordinate exceeds its authenticated physical bounds",
            ));
        }
        Ok(ScenePoint::new(
            Length::from_micrometres(x),
            Length::from_micrometres(y),
        ))
    }

    fn compile_document(
        &mut self,
        document: &HardcopySemanticDocument,
    ) -> Result<(), HardcopyRenderError> {
        match document {
            HardcopySemanticDocument::Schematic(schematic) => self.schematic(schematic),
            HardcopySemanticDocument::Symbol(symbol) => self.symbol_document(symbol, None),
            HardcopySemanticDocument::Plot(plot) => self.plot(plot),
            HardcopySemanticDocument::ResultSummary(summary) => self.result_summary(summary),
            HardcopySemanticDocument::Report(report) => self.report(report),
            HardcopySemanticDocument::Aggregate(_) => Err(conversion_error(
                "nested semantic aggregates are not supported",
            )),
        }
    }

    pub(super) fn aggregate(
        &mut self,
        aggregate: &SemanticAggregate,
    ) -> Result<(), HardcopyRenderError> {
        if aggregate.children.is_empty() || aggregate.children.len() > 4_096 {
            return Err(HardcopyRenderError::ResourceLimit {
                scope: "aggregate children",
                maximum: 4_096,
            });
        }
        for (index, child) in aggregate.children.iter().enumerate() {
            let expected_ordinal =
                u32::try_from(index).map_err(|_| conversion_error("aggregate ordinal overflow"))?;
            if child.ordinal != expected_ordinal {
                return Err(conversion_error(
                    "aggregate children are not in canonical ordinal order",
                ));
            }
            let offset_x = child
                .placement_origin
                .x_um
                .checked_sub(self.bounds.minimum.x_um)
                .and_then(|value| u64::try_from(value).ok())
                .ok_or_else(|| conversion_error("aggregate child X placement precedes bounds"))?;
            let offset_y = child
                .placement_origin
                .y_um
                .checked_sub(self.bounds.minimum.y_um)
                .and_then(|value| u64::try_from(value).ok())
                .ok_or_else(|| conversion_error("aggregate child Y placement precedes bounds"))?;
            let mut compiler = Self {
                bounds: child.local_bounds,
                extent: self.extent,
                mapping: self.mapping,
                primitives: Vec::new(),
                legend: Vec::new(),
                coordinate_offset: ScenePoint::new(
                    Length::from_micrometres(offset_x),
                    Length::from_micrometres(offset_y),
                ),
                aggregate_sections: Vec::new(),
                mapping_ordinal: Some(child.ordinal),
            };
            compiler.compile_document(&child.document)?;
            let primitive_start = self.primitives.len();
            let primitive_end = primitive_start
                .checked_add(compiler.primitives.len())
                .ok_or(HardcopyRenderError::ResourceLimit {
                    scope: "scene primitives",
                    maximum: MAX_SCENE_PRIMITIVES as u64,
                })?;
            let width = child
                .local_bounds
                .maximum
                .x_um
                .checked_sub(child.local_bounds.minimum.x_um)
                .and_then(|value| u64::try_from(value).ok())
                .ok_or_else(|| conversion_error("aggregate child width is invalid"))?;
            let height = child
                .local_bounds
                .maximum
                .y_um
                .checked_sub(child.local_bounds.minimum.y_um)
                .and_then(|value| u64::try_from(value).ok())
                .ok_or_else(|| conversion_error("aggregate child height is invalid"))?;
            let extent = ContentExtent::try_new(
                Length::from_micrometres(width),
                Length::from_micrometres(height),
            )
            .map_err(|error| conversion_error(error.to_string()))?;
            self.aggregate_sections.push(AggregateSection {
                ordinal: child.ordinal,
                source_key: child.source_key.clone(),
                display_name: child.display_name.clone(),
                content_digest: child.content_digest,
                origin: compiler.coordinate_offset,
                extent,
                page_break_before: child.page_break_before,
                primitive_start,
                primitive_end,
            });
            self.primitives.extend(compiler.primitives);
            for legend in compiler.legend {
                if !self.legend.iter().any(|existing| existing == &legend) {
                    self.legend.push(legend);
                }
            }
        }
        Ok(())
    }

    fn schematic_polyline(
        &self,
        points: &[SchematicPoint],
    ) -> Result<Vec<ScenePoint>, HardcopyRenderError> {
        points
            .iter()
            .copied()
            .map(|point| self.schematic_point(point))
            .collect()
    }

    pub(super) fn mapped_stroke(
        &self,
        kind: PrintObjectKind,
        stable_id: &str,
        fallback: StrokeStyle,
    ) -> StrokeStyle {
        let mapped_id = self.mapping_stable_id(stable_id);
        let Some(entry) =
            self.mapping.entries().iter().find(|entry| {
                entry.object().kind() == kind && entry.object().stable_id() == mapped_id
            })
        else {
            return fallback;
        };
        let color = SemanticColor::Exact(print_color_rgb(entry.print_color()));
        let (width, pattern, exact_dash, exact_dot_spacing) = match entry.redundancy() {
            PrintRedundancy::SolidLine { width } => (width, StrokePattern::Solid, None, None),
            PrintRedundancy::DashedLine { width, dash, gap } => {
                (width, StrokePattern::Dashed, Some((dash, gap)), None)
            }
            PrintRedundancy::DottedLeader { width, spacing } => {
                (width, StrokePattern::Dotted, None, Some(spacing))
            }
            PrintRedundancy::CrossHatch { line_width, .. } => {
                (line_width, StrokePattern::Solid, None, None)
            }
            PrintRedundancy::TriangleWithId { .. }
            | PrintRedundancy::SolidFill
            | PrintRedundancy::SourceStyle => (
                fallback.width,
                fallback.pattern,
                fallback.exact_dash,
                fallback.exact_dot_spacing,
            ),
        };
        StrokeStyle {
            color,
            width,
            pattern,
            series_index: fallback.series_index,
            exact_dash,
            exact_dot_spacing,
        }
    }

    fn mapped_redundancy(&self, kind: PrintObjectKind, stable_id: &str) -> Option<PrintRedundancy> {
        let mapped_id = self.mapping_stable_id(stable_id);
        self.mapping
            .entries()
            .iter()
            .find(|entry| entry.object().kind() == kind && entry.object().stable_id() == mapped_id)
            .map(|entry| entry.redundancy())
    }

    fn mapped_fill(
        &self,
        kind: PrintObjectKind,
        stable_id: &str,
        fallback: Option<SceneFill>,
    ) -> Option<SceneFill> {
        let mapped_id = self.mapping_stable_id(stable_id);
        let entry = self.mapping.entries().iter().find(|entry| {
            entry.object().kind() == kind && entry.object().stable_id() == mapped_id
        })?;
        let color = SemanticColor::Exact(print_color_rgb(entry.print_color()));
        match entry.redundancy() {
            PrintRedundancy::SolidFill => Some(SceneFill::solid(color)),
            PrintRedundancy::CrossHatch {
                line_width,
                spacing,
            } => Some(SceneFill::CrossHatch {
                color,
                line_width,
                spacing,
            }),
            _ => fallback,
        }
    }

    fn add_mapping_legend(
        &mut self,
        kind: PrintObjectKind,
        stable_id: &str,
        stroke: StrokeStyle,
    ) -> Result<(), HardcopyRenderError> {
        let mapped_id = self.mapping_stable_id(stable_id);
        let Some(entry) = self.mapping.entries().iter().find(|entry| {
            entry.object().kind() == kind
                && entry.object().stable_id() == mapped_id
                && entry.include_in_legend()
        }) else {
            return Ok(());
        };
        if self
            .legend
            .iter()
            .any(|existing| existing.label == entry.object().display_name())
        {
            return Ok(());
        }
        let display_name = entry.object().display_name().to_owned();
        let fill = self.mapped_fill(kind, stable_id, None);
        self.legend
            .push(LegendEntry::try_new_with_fill(display_name, stroke, fill)?);
        Ok(())
    }

    pub(super) fn mapping_stable_id(&self, stable_id: &str) -> String {
        let Some(ordinal) = self.mapping_ordinal else {
            return stable_id.to_owned();
        };
        let stable_digest = Sha256::digest(
            [
                b"rspice-aggregate-print-object-v1:".as_slice(),
                &ordinal.to_be_bytes(),
                stable_id.as_bytes(),
            ]
            .concat(),
        );
        let stable_suffix = stable_digest[..12]
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        format!("aggregate:{ordinal}:{stable_suffix}")
    }

    pub(super) fn schematic(
        &mut self,
        schematic: &SemanticSchematic,
    ) -> Result<(), HardcopyRenderError> {
        let component_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:schematic-components",
            StrokeStyle::default(),
        );
        let component_fill = self.mapped_fill(
            PrintObjectKind::Layer,
            "layer:schematic-components",
            Some(SceneFill::solid(component_stroke.color)),
        );
        let wire_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:schematic-wiring",
            StrokeStyle::try_new(
                SemanticColor::Foreground,
                Length::from_micrometres(250),
                StrokePattern::Solid,
                None,
            )?,
        );
        let bus_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:schematic-buses",
            StrokeStyle::try_new(
                SemanticColor::Foreground,
                Length::from_micrometres(600),
                StrokePattern::Solid,
                None,
            )?,
        );
        for wire in &schematic.wires {
            if wire.points.len() >= 2 {
                self.primitives.push(ScenePrimitive::Polyline {
                    points: self.schematic_polyline(&wire.points)?,
                    closed: false,
                    stroke: wire_stroke,
                    fill: None,
                });
            }
        }
        for bus in &schematic.buses {
            self.primitives.push(ScenePrimitive::Polyline {
                points: self.schematic_polyline(&bus.points)?,
                closed: false,
                stroke: bus_stroke,
                fill: None,
            });
        }
        for tap in &schematic.bus_taps {
            self.primitives.push(ScenePrimitive::Line {
                from: self.schematic_point(tap.bus_point)?,
                to: self.schematic_point(tap.connection_point)?,
                stroke: bus_stroke,
            });
        }
        for junction in &schematic.junctions {
            self.primitives.push(ScenePrimitive::Circle {
                center: self.schematic_point(junction.pos)?,
                radius: Length::from_micrometres(900),
                stroke: None,
                fill: self.mapped_fill(
                    PrintObjectKind::Layer,
                    "layer:schematic-wiring",
                    Some(SceneFill::solid(wire_stroke.color)),
                ),
            });
        }
        for label in &schematic.net_labels {
            self.add_text(
                self.schematic_point(label.pos)?,
                &label.name,
                SceneFont::SansSemibold,
                2_800,
                wire_stroke.color,
            )?;
        }

        let library = SymbolLibrary::load_embedded()
            .map_err(|error| conversion_error(format!("embedded symbol library: {error}")))?;
        for semantic in &schematic.components {
            if let Some(symbol) = &semantic.resolved_symbol {
                self.symbol_document(symbol, Some(&semantic.component))?;
            } else {
                self.library_component(
                    &library,
                    &semantic.component,
                    component_stroke,
                    component_fill,
                )?;
            }
            let anchor = self.schematic_point(semantic.component.pos)?;
            self.add_text(
                anchor,
                &semantic.component.name,
                SceneFont::SansSemibold,
                2_700,
                component_stroke.color,
            )?;
            if !semantic.component.value.is_empty() {
                let value_anchor = self.offset_scene_point(anchor, 0, 3_500)?;
                self.add_text(
                    value_anchor,
                    &semantic.component.value,
                    SceneFont::Sans,
                    2_500,
                    component_stroke.color,
                )?;
            }
        }

        let annotation_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:drawing-annotation",
            StrokeStyle::try_new(
                SemanticColor::Secondary,
                Length::from_micrometres(200),
                StrokePattern::Dotted,
                None,
            )?,
        );
        for note in &schematic.design_notes {
            let mut origin = self.schematic_point(note.pos)?;
            for line in note.text.lines() {
                self.add_text(
                    origin,
                    line,
                    SceneFont::Sans,
                    2_800,
                    annotation_stroke.color,
                )?;
                origin = self.offset_scene_point(origin, 0, 3_400)?;
            }
        }
        let documentation_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:drawing-documentation",
            StrokeStyle::try_new(
                SemanticColor::Secondary,
                Length::from_micrometres(220),
                StrokePattern::Solid,
                None,
            )?,
        );
        for shape in &schematic.documentation_shapes {
            self.documentation_shape(&shape.geometry, documentation_stroke)?;
        }
        self.add_mapping_legend(
            PrintObjectKind::Layer,
            "layer:schematic-components",
            component_stroke,
        )?;
        self.add_mapping_legend(
            PrintObjectKind::Layer,
            "layer:schematic-wiring",
            wire_stroke,
        )?;
        self.add_mapping_legend(PrintObjectKind::Layer, "layer:schematic-buses", bus_stroke)?;
        self.add_mapping_legend(
            PrintObjectKind::Layer,
            "layer:drawing-annotation",
            annotation_stroke,
        )?;
        self.add_mapping_legend(
            PrintObjectKind::Layer,
            "layer:drawing-documentation",
            documentation_stroke,
        )?;
        Ok(())
    }

    fn offset_scene_point(
        &self,
        point: ScenePoint,
        dx: i64,
        dy: i64,
    ) -> Result<ScenePoint, HardcopyRenderError> {
        let axis = |value: u64, delta: i64, maximum: u64| {
            let adjusted = i128::from(value) + i128::from(delta);
            if adjusted < 0 || adjusted > i128::from(maximum) {
                None
            } else {
                Some(adjusted as u64)
            }
        };
        Ok(ScenePoint::new(
            Length::from_micrometres(
                axis(point.x.micrometres(), dx, self.extent.width().micrometres())
                    .ok_or_else(|| conversion_error("text anchor exceeds source bounds"))?,
            ),
            Length::from_micrometres(
                axis(
                    point.y.micrometres(),
                    dy,
                    self.extent.height().micrometres(),
                )
                .ok_or_else(|| conversion_error("text anchor exceeds source bounds"))?,
            ),
        ))
    }

    fn add_text(
        &mut self,
        origin: ScenePoint,
        text: &str,
        font: SceneFont,
        size_um: u64,
        color: SemanticColor,
    ) -> Result<(), HardcopyRenderError> {
        let normalized = text.replace(['\r', '\n', '\t'], " ");
        if normalized.trim().is_empty() {
            return Ok(());
        }
        validate_text("semantic source text", &normalized, 65_536)?;
        self.primitives.push(ScenePrimitive::Text {
            origin,
            text: normalized,
            font,
            size: Length::from_micrometres(size_um),
            color,
            anchor: TextAnchor::Start,
        });
        Ok(())
    }

    pub(super) fn symbol_document(
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

    fn library_component(
        &mut self,
        library: &SymbolLibrary,
        component: &Component,
        stroke: StrokeStyle,
        mapped_fill: Option<SceneFill>,
    ) -> Result<(), HardcopyRenderError> {
        let (symbol, rotation) = library
            .get_with_rotation_variant(
                component.kind,
                component.rotation.degrees(),
                component.symbol_variant.as_deref(),
            )
            .ok_or_else(|| {
                conversion_error(format!(
                    "no production symbol is registered for {:?}",
                    component.kind
                ))
            })?;
        let (cx, cy) = symbol.center();
        let scale_x = f64::from(symbol.target_width / symbol.width().max(0.001));
        let scale_y = f64::from(symbol.target_height / symbol.height().max(0.001));
        let radians = f64::from(rotation).to_radians();
        let (cosine, sine) = (radians.cos(), radians.sin());
        let transform = |x: f32, y: f32| {
            let mut x = (f64::from(x) - f64::from(cx)) * scale_x;
            let mut y = (f64::from(y) - f64::from(cy)) * scale_y;
            if component.mirror_h {
                x = -x;
            }
            if component.mirror_v {
                y = -y;
            }
            let rotated_x = x * cosine - y * sine;
            let rotated_y = x * sine + y * cosine;
            SchematicPoint::new(
                component.pos.x.saturating_add(rotated_x.round() as i32),
                component.pos.y.saturating_add(rotated_y.round() as i32),
            )
        };
        for path in &symbol.paths {
            let mut points = Vec::<SchematicPoint>::new();
            let mut current = (0.0_f32, 0.0_f32);
            let flush = |this: &mut Self,
                         points: &mut Vec<SchematicPoint>|
             -> Result<(), HardcopyRenderError> {
                if points.len() >= 2 {
                    let closed = points.first() == points.last() && points.len() >= 4;
                    if closed {
                        points.pop();
                    }
                    this.primitives.push(ScenePrimitive::Polyline {
                        points: points
                            .drain(..)
                            .map(|point| this.schematic_point(point))
                            .collect::<Result<_, _>>()?,
                        closed,
                        stroke,
                        fill: if path.filled { mapped_fill } else { None },
                    });
                } else {
                    points.clear();
                }
                Ok(())
            };
            for command in &path.commands {
                match command {
                    PathCommand::MoveTo(x, y) => {
                        flush(self, &mut points)?;
                        points.push(transform(*x, *y));
                        current = (*x, *y);
                    }
                    PathCommand::LineTo(x, y) => {
                        points.push(transform(*x, *y));
                        current = (*x, *y);
                    }
                    PathCommand::CurveTo { ctrl1, ctrl2, end } => {
                        for index in 1..=16 {
                            let t = index as f32 / 16.0;
                            let one_minus_t = 1.0 - t;
                            let x = one_minus_t.powi(3) * current.0
                                + 3.0 * one_minus_t.powi(2) * t * ctrl1.0
                                + 3.0 * one_minus_t * t.powi(2) * ctrl2.0
                                + t.powi(3) * end.0;
                            let y = one_minus_t.powi(3) * current.1
                                + 3.0 * one_minus_t.powi(2) * t * ctrl1.1
                                + 3.0 * one_minus_t * t.powi(2) * ctrl2.1
                                + t.powi(3) * end.1;
                            points.push(transform(x, y));
                        }
                        current = *end;
                    }
                    PathCommand::Close => {
                        if let Some(first) = points.first().copied() {
                            points.push(first);
                        }
                    }
                }
            }
            flush(self, &mut points)?;
        }
        Ok(())
    }

    fn documentation_shape(
        &mut self,
        geometry: &DocumentationShapeGeometry,
        stroke: StrokeStyle,
    ) -> Result<(), HardcopyRenderError> {
        match geometry {
            DocumentationShapeGeometry::Rectangle { first, opposite } => {
                let first = self.schematic_point(*first)?;
                let opposite = self.schematic_point(*opposite)?;
                let x = first.x.min(opposite.x);
                let y = first.y.min(opposite.y);
                let width = Length::from_micrometres(
                    first.x.micrometres().abs_diff(opposite.x.micrometres()),
                );
                let height = Length::from_micrometres(
                    first.y.micrometres().abs_diff(opposite.y.micrometres()),
                );
                self.primitives.push(ScenePrimitive::Rect {
                    rect: SceneRect::try_new(x, y, width, height)?,
                    stroke: Some(stroke),
                    fill: None,
                });
            }
            DocumentationShapeGeometry::Line { start, end } => {
                self.primitives.push(ScenePrimitive::Line {
                    from: self.schematic_point(*start)?,
                    to: self.schematic_point(*end)?,
                    stroke,
                });
            }
            DocumentationShapeGeometry::Polygon { points } => {
                self.primitives.push(ScenePrimitive::Polyline {
                    points: self.schematic_polyline(points)?,
                    closed: true,
                    stroke,
                    fill: None,
                });
            }
            DocumentationShapeGeometry::Arc {
                start,
                through,
                end,
            } => {
                let points = circular_arc_points(*start, *through, *end)?;
                self.primitives.push(ScenePrimitive::Polyline {
                    points: self.schematic_polyline(&points)?,
                    closed: false,
                    stroke,
                    fill: None,
                });
            }
            DocumentationShapeGeometry::Callout {
                tip,
                elbow,
                box_corner,
            } => {
                self.primitives.push(ScenePrimitive::Polyline {
                    points: self.schematic_polyline(&[*tip, *elbow])?,
                    closed: false,
                    stroke,
                    fill: None,
                });
                let first = self.schematic_point(*elbow)?;
                let opposite = self.schematic_point(*box_corner)?;
                self.primitives.push(ScenePrimitive::Rect {
                    rect: SceneRect::try_new(
                        first.x.min(opposite.x),
                        first.y.min(opposite.y),
                        Length::from_micrometres(
                            first.x.micrometres().abs_diff(opposite.x.micrometres()),
                        ),
                        Length::from_micrometres(
                            first.y.micrometres().abs_diff(opposite.y.micrometres()),
                        ),
                    )?,
                    stroke: Some(stroke),
                    fill: None,
                });
            }
        }
        Ok(())
    }

    pub(super) fn plot(&mut self, plot: &SemanticPlot) -> Result<(), HardcopyRenderError> {
        let frame_stroke = StrokeStyle::try_new(
            SemanticColor::Secondary,
            Length::from_micrometres(220),
            StrokePattern::Solid,
            None,
        )?;
        let frame = SceneRect::try_new(
            Length::from_micrometres(12_700),
            Length::from_micrometres(12_700),
            Length::from_micrometres(self.extent.width().micrometres().saturating_sub(25_400)),
            Length::from_micrometres(self.extent.height().micrometres().saturating_sub(25_400)),
        )?;
        self.primitives.push(ScenePrimitive::Rect {
            rect: frame,
            stroke: Some(frame_stroke),
            fill: None,
        });
        for division in 1..10_u64 {
            let x = self.extent.width().micrometres() * division / 10;
            self.primitives.push(ScenePrimitive::Line {
                from: ScenePoint::new(Length::from_micrometres(x), frame.y),
                to: ScenePoint::new(
                    Length::from_micrometres(x),
                    Length::from_micrometres(frame.y.micrometres() + frame.height.micrometres()),
                ),
                stroke: StrokeStyle::try_new(
                    SemanticColor::Grid,
                    Length::from_micrometres(120),
                    StrokePattern::Dotted,
                    None,
                )?,
            });
        }
        for (index, trace) in plot.traces.iter().enumerate() {
            let stable_id = format!("trace:{}", trace.trace_id);
            let stroke = self.mapped_stroke(
                PrintObjectKind::Trace,
                &stable_id,
                StrokeStyle::try_new(
                    SemanticColor::Trace(index as u16),
                    Length::from_micrometres(300),
                    auto_trace_pattern(index as u16),
                    Some(index as u16),
                )?,
            );
            for path in &trace.paths {
                if path.len() >= 2 {
                    self.primitives.push(ScenePrimitive::Polyline {
                        points: path
                            .iter()
                            .copied()
                            .map(|point| self.semantic_point(point))
                            .collect::<Result<_, _>>()?,
                        closed: false,
                        stroke,
                        fill: None,
                    });
                }
            }
            self.add_mapping_legend(PrintObjectKind::Trace, &stable_id, stroke)?;
            let mapped_id = self.mapping_stable_id(&stable_id);
            if !self.mapping.entries().iter().any(|entry| {
                entry.object().kind() == PrintObjectKind::Trace
                    && entry.object().stable_id() == mapped_id
            }) {
                self.legend
                    .push(LegendEntry::try_new(&trace.label, stroke)?);
            }
        }
        for marker in &plot.markers {
            let position = marker.position.ok_or_else(|| {
                conversion_error(format!(
                    "plot marker {} has no authenticated physical position",
                    marker.marker_id
                ))
            })?;
            let center = self.semantic_point(position)?;
            let stable_id = format!("marker:{}", marker.marker_id);
            let size = match self.mapped_redundancy(PrintObjectKind::Marker, &stable_id) {
                Some(PrintRedundancy::TriangleWithId { size }) => i64::try_from(size.micrometres())
                    .map_err(|_| conversion_error("mapped marker size exceeds signed geometry"))?,
                _ => 2_500_i64,
            };
            let points = [
                self.offset_scene_point(center, 0, -size)?,
                self.offset_scene_point(center, -size, size)?,
                self.offset_scene_point(center, size, size)?,
            ];
            let stroke = self.mapped_stroke(
                PrintObjectKind::Marker,
                &stable_id,
                StrokeStyle::try_new(
                    SemanticColor::Accent,
                    Length::from_micrometres(250),
                    StrokePattern::Solid,
                    None,
                )?,
            );
            self.primitives.push(ScenePrimitive::Polyline {
                points: points.to_vec(),
                closed: true,
                stroke,
                fill: self.mapped_fill(
                    PrintObjectKind::Marker,
                    &stable_id,
                    Some(SceneFill::solid(stroke.color)),
                ),
            });
            let label = self.offset_scene_point(center, size + 800, -size)?;
            let mapped_marker_id = marker.marker_id.to_string();
            self.add_text(
                label,
                if matches!(
                    self.mapped_redundancy(PrintObjectKind::Marker, &stable_id),
                    Some(PrintRedundancy::TriangleWithId { .. })
                ) {
                    &mapped_marker_id
                } else {
                    &marker.label
                },
                SceneFont::Monospace,
                2_300,
                stroke.color,
            )?;
            self.add_mapping_legend(PrintObjectKind::Marker, &stable_id, stroke)?;
        }
        for annotation in &plot.annotations {
            let position = annotation.position.ok_or_else(|| {
                conversion_error(format!(
                    "plot annotation {} has no authenticated physical position",
                    annotation.annotation_id
                ))
            })?;
            let origin = self.semantic_point(position)?;
            let label = self.offset_scene_point(origin, 4_000, -4_000)?;
            let stable_id = format!("annotation:{}", annotation.annotation_id);
            let stroke = self.mapped_stroke(
                PrintObjectKind::ReviewAnnotation,
                &stable_id,
                StrokeStyle::try_new(
                    SemanticColor::Secondary,
                    Length::from_micrometres(200),
                    StrokePattern::Dotted,
                    None,
                )?,
            );
            self.primitives.push(ScenePrimitive::Line {
                from: origin,
                to: label,
                stroke,
            });
            self.add_text(
                label,
                &annotation.text,
                SceneFont::Sans,
                2_400,
                stroke.color,
            )?;
        }
        Ok(())
    }

    pub(super) fn result_summary(
        &mut self,
        summary: &SemanticResultSummary,
    ) -> Result<(), HardcopyRenderError> {
        let stable_id = format!(
            "layer:result-summary:{}",
            summary.viewer.label().to_ascii_lowercase()
        );
        let stroke = self.mapped_stroke(PrintObjectKind::Layer, &stable_id, StrokeStyle::default());
        let mut y = 12_000_u64;
        self.add_text(
            ScenePoint::new(
                Length::from_micrometres(12_000),
                Length::from_micrometres(y),
            ),
            &summary.title,
            SceneFont::SansSemibold,
            4_500,
            stroke.color,
        )?;
        y += 9_000;
        for table in &summary.tables {
            y = self.semantic_table(table, y, stroke)?;
        }
        self.add_mapping_legend(PrintObjectKind::Layer, &stable_id, stroke)?;
        Ok(())
    }

    fn semantic_table(
        &mut self,
        table: &SemanticTable,
        mut y: u64,
        stroke: StrokeStyle,
    ) -> Result<u64, HardcopyRenderError> {
        let left = 12_000_u64;
        let right = self.extent.width().micrometres().saturating_sub(12_000);
        let columns = table.columns.len().max(1) as u64;
        let column_width = right.saturating_sub(left) / columns;
        self.add_text(
            ScenePoint::new(Length::from_micrometres(left), Length::from_micrometres(y)),
            &table.title,
            SceneFont::SansSemibold,
            3_200,
            stroke.color,
        )?;
        y += 5_000;
        for (column, heading) in table.columns.iter().enumerate() {
            self.primitives.push(ScenePrimitive::Rect {
                rect: SceneRect::try_new(
                    Length::from_micrometres(left + column as u64 * column_width),
                    Length::from_micrometres(y),
                    Length::from_micrometres(column_width),
                    Length::from_micrometres(4_500),
                )?,
                stroke: Some(stroke),
                fill: None,
            });
            self.add_text(
                ScenePoint::new(
                    Length::from_micrometres(left + column as u64 * column_width + 800),
                    Length::from_micrometres(y + 3_000),
                ),
                heading,
                SceneFont::SansSemibold,
                2_300,
                stroke.color,
            )?;
        }
        y += 4_500;
        for row in &table.rows {
            if y + 4_500 > self.extent.height().micrometres().saturating_sub(8_000) {
                return Err(conversion_error(format!(
                    "table '{}' does not fit its authenticated result page",
                    table.title
                )));
            }
            for (column, value) in row.iter().enumerate() {
                if column >= columns as usize {
                    break;
                }
                self.primitives.push(ScenePrimitive::Rect {
                    rect: SceneRect::try_new(
                        Length::from_micrometres(left + column as u64 * column_width),
                        Length::from_micrometres(y),
                        Length::from_micrometres(column_width),
                        Length::from_micrometres(4_500),
                    )?,
                    stroke: Some(stroke),
                    fill: None,
                });
                self.add_text(
                    ScenePoint::new(
                        Length::from_micrometres(left + column as u64 * column_width + 800),
                        Length::from_micrometres(y + 3_000),
                    ),
                    value,
                    SceneFont::Monospace,
                    2_100,
                    stroke.color,
                )?;
            }
            y += 4_500;
        }
        Ok(y + 5_000)
    }

    pub(super) fn report(&mut self, report: &SemanticReport) -> Result<(), HardcopyRenderError> {
        const PAGE_HEIGHT: u64 = 279_400;
        const PAGE_GAP: u64 = 5_000;
        let stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:report-content",
            StrokeStyle::default(),
        );
        for (page_index, page) in report.pages.iter().enumerate() {
            let page_top = page_index as u64 * (PAGE_HEIGHT + PAGE_GAP);
            let page_bottom = page_top + PAGE_HEIGHT;
            self.primitives.push(ScenePrimitive::Rect {
                rect: SceneRect::try_new(
                    Length::from_micrometres(0),
                    Length::from_micrometres(page_top),
                    Length::from_micrometres(self.extent.width().micrometres()),
                    Length::from_micrometres(PAGE_HEIGHT),
                )?,
                stroke: Some(stroke),
                fill: None,
            });
            let mut y = page_top + 16_000;
            self.add_text(
                ScenePoint::new(
                    Length::from_micrometres(16_000),
                    Length::from_micrometres(y),
                ),
                page.title(),
                SceneFont::SansSemibold,
                5_000,
                stroke.color,
            )?;
            y += 9_000;
            for section in page.sections() {
                self.ensure_report_room(y, 7_000, page_bottom, page.title())?;
                self.add_text(
                    ScenePoint::new(
                        Length::from_micrometres(16_000),
                        Length::from_micrometres(y),
                    ),
                    section.title(),
                    SceneFont::SansSemibold,
                    3_800,
                    stroke.color,
                )?;
                y += 7_000;
                for block in section.blocks() {
                    if let Some(reference) = block.kind().reference() {
                        let authenticated = report
                            .authenticated_references
                            .iter()
                            .find(|candidate| candidate.block_id == block.id())
                            .ok_or(HardcopyRenderError::UnauthenticatedReportReference)?;
                        if &authenticated.reference != reference {
                            return Err(HardcopyRenderError::UnauthenticatedReportReference);
                        }
                    }
                    y = self.report_block(
                        block.id(),
                        block.kind(),
                        report
                            .figures
                            .iter()
                            .find(|figure| figure.block_id == block.id()),
                        y,
                        page_bottom,
                        stroke,
                        page.title(),
                    )?;
                }
            }
        }
        self.add_mapping_legend(PrintObjectKind::Layer, "layer:report-content", stroke)?;
        Ok(())
    }

    fn ensure_report_room(
        &self,
        y: u64,
        required: u64,
        page_bottom: u64,
        title: &str,
    ) -> Result<(), HardcopyRenderError> {
        if y.saturating_add(required) > page_bottom.saturating_sub(12_000) {
            Err(conversion_error(format!(
                "authored report page '{title}' overflows its physical page"
            )))
        } else {
            Ok(())
        }
    }

    fn report_block(
        &mut self,
        block_id: ReportBlockId,
        block: &ReportBlockKind,
        figure: Option<&SemanticReportFigure>,
        mut y: u64,
        page_bottom: u64,
        stroke: StrokeStyle,
        page_title: &str,
    ) -> Result<u64, HardcopyRenderError> {
        let mut lines = Vec::<String>::new();
        match block {
            ReportBlockKind::PlotFigure(value) => {
                let figure =
                    figure.ok_or(HardcopyRenderError::UnsupportedAuthenticatedReportBlock(
                        "unresolved plot figure",
                    ))?;
                if figure.media_type != "image/png"
                    || figure.caption != value.caption
                    || figure.alternative_text != value.alternative_text
                    || figure.sizing != value.sizing
                {
                    return Err(HardcopyRenderError::UnsupportedAuthenticatedReportBlock(
                        "mismatched plot figure",
                    ));
                }
                let (pixel_width, pixel_height) = png_dimensions(&figure.payload)?;
                if pixel_width != figure.width_pixels || pixel_height != figure.height_pixels {
                    return Err(HardcopyRenderError::InvalidEmbeddedFigure(
                        "authenticated PNG dimensions changed after source resolution".to_owned(),
                    ));
                }
                self.ensure_report_room(y, 5_500, page_bottom, page_title)?;
                self.add_text(
                    ScenePoint::new(
                        Length::from_micrometres(20_000),
                        Length::from_micrometres(y),
                    ),
                    &value.caption,
                    SceneFont::SansSemibold,
                    2_900,
                    stroke.color,
                )?;
                y += 5_500;
                let maximum_width = self
                    .extent
                    .width()
                    .micrometres()
                    .checked_sub(40_000)
                    .ok_or_else(|| conversion_error("report figure page is too narrow"))?;
                let maximum_height = page_bottom.saturating_sub(18_000).saturating_sub(y);
                let natural_width = u64::from(pixel_width)
                    .checked_mul(MICROMETRES_PER_INCH)
                    .ok_or_else(|| conversion_error("report figure width overflow"))?
                    .div_ceil(96);
                let natural_height = u64::from(pixel_height)
                    .checked_mul(MICROMETRES_PER_INCH)
                    .ok_or_else(|| conversion_error("report figure height overflow"))?
                    .div_ceil(96);
                let scale = match value.sizing {
                    FigureSizing::Natural => 1.0_f64
                        .min(maximum_width as f64 / natural_width as f64)
                        .min(maximum_height as f64 / natural_height as f64),
                    FigureSizing::FitWidth => (maximum_width as f64 / natural_width as f64)
                        .min(maximum_height as f64 / natural_height as f64),
                    FigureSizing::FitPage => (maximum_width as f64 / natural_width as f64)
                        .min(maximum_height as f64 / natural_height as f64)
                        .min(1.0),
                };
                if !scale.is_finite() || scale <= 0.0 {
                    return Err(conversion_error(format!(
                        "authenticated plot figure {block_id} cannot fit its authored page"
                    )));
                }
                let width = (natural_width as f64 * scale).floor() as u64;
                let height = (natural_height as f64 * scale).floor() as u64;
                self.ensure_report_room(y, height.saturating_add(7_000), page_bottom, page_title)?;
                self.primitives.push(ScenePrimitive::RasterImage {
                    rect: SceneRect::try_new(
                        Length::from_micrometres(20_000),
                        Length::from_micrometres(y),
                        Length::from_micrometres(width),
                        Length::from_micrometres(height),
                    )?,
                    png: figure.payload.clone(),
                    content_digest: figure.artifact_digest,
                    alternative_text: value.alternative_text.clone(),
                });
                y = y.saturating_add(height).saturating_add(4_000);
                self.add_text(
                    ScenePoint::new(
                        Length::from_micrometres(20_000),
                        Length::from_micrometres(y),
                    ),
                    &value.alternative_text,
                    SceneFont::Sans,
                    2_200,
                    stroke.color,
                )?;
                return Ok(y + 5_000);
            }
            ReportBlockKind::DataTable(value) => {
                let table = SemanticTable {
                    title: value.title.clone(),
                    columns: value
                        .columns
                        .iter()
                        .map(|column| column.heading.clone())
                        .collect(),
                    rows: value
                        .rows
                        .iter()
                        .map(|row| row.iter().map(format_table_cell).collect())
                        .collect(),
                };
                let next = self.semantic_table(&table, y, stroke)?;
                if next > page_bottom.saturating_sub(12_000) {
                    return Err(conversion_error(format!(
                        "report table '{}' overflows authored page '{page_title}'",
                        value.title
                    )));
                }
                return Ok(next);
            }
            ReportBlockKind::Datasheet(value) => {
                let table = SemanticTable {
                    title: value.title.clone(),
                    columns: vec!["Field".to_owned(), "Value".to_owned()],
                    rows: value
                        .fields
                        .iter()
                        .map(|field| {
                            vec![
                                field.label.clone(),
                                format!(
                                    "{}{}",
                                    field.value,
                                    field
                                        .unit
                                        .as_deref()
                                        .map(|unit| format!(" {unit}"))
                                        .unwrap_or_default()
                                ),
                            ]
                        })
                        .collect(),
                };
                let next = self.semantic_table(&table, y, stroke)?;
                if next > page_bottom.saturating_sub(12_000) {
                    return Err(conversion_error(format!(
                        "report datasheet '{}' overflows authored page '{page_title}'",
                        value.title
                    )));
                }
                return Ok(next);
            }
            ReportBlockKind::Requirements(value) => {
                let table = SemanticTable {
                    title: value.title.clone(),
                    columns: vec![
                        "Requirement".to_owned(),
                        "Statement".to_owned(),
                        "Disposition".to_owned(),
                        "Evidence".to_owned(),
                    ],
                    rows: value
                        .entries
                        .iter()
                        .map(|entry| {
                            vec![
                                entry.requirement_id.clone(),
                                entry.statement.clone(),
                                format!("{:?}", entry.disposition),
                                entry.evidence_label.clone().unwrap_or_default(),
                            ]
                        })
                        .collect(),
                };
                let next = self.semantic_table(&table, y, stroke)?;
                if next > page_bottom.saturating_sub(12_000) {
                    return Err(conversion_error(format!(
                        "report requirements '{}' overflow authored page '{page_title}'",
                        value.title
                    )));
                }
                return Ok(next);
            }
            ReportBlockKind::Specifications(value) => {
                let table = SemanticTable {
                    title: value.title.clone(),
                    columns: vec![
                        "Expression".to_owned(),
                        "Limit".to_owned(),
                        "Measured".to_owned(),
                        "Disposition".to_owned(),
                    ],
                    rows: value
                        .entries
                        .iter()
                        .map(|entry| {
                            vec![
                                entry.expression.clone(),
                                entry.limit.clone(),
                                entry.measured.clone().unwrap_or_else(|| "—".to_owned()),
                                format!("{:?}", entry.disposition),
                            ]
                        })
                        .collect(),
                };
                let next = self.semantic_table(&table, y, stroke)?;
                if next > page_bottom.saturating_sub(12_000) {
                    return Err(conversion_error(format!(
                        "report specifications '{}' overflow authored page '{page_title}'",
                        value.title
                    )));
                }
                return Ok(next);
            }
            ReportBlockKind::Prose(value) => {
                return self.report_markdown(&value.markdown, y, page_bottom, stroke, page_title);
            }
            ReportBlockKind::ReviewNote(value) => {
                lines.push(format!("Review — {} [{:?}]", value.author, value.status));
                lines.extend(value.message.lines().map(str::to_owned));
            }
            ReportBlockKind::Evidence(_) => {
                return Err(HardcopyRenderError::UnsupportedAuthenticatedReportBlock(
                    "evidence",
                ));
            }
        }
        for line in lines {
            for wrapped in wrap_text(&line, 112) {
                self.ensure_report_room(y, 4_200, page_bottom, page_title)?;
                self.add_text(
                    ScenePoint::new(
                        Length::from_micrometres(20_000),
                        Length::from_micrometres(y),
                    ),
                    &wrapped,
                    SceneFont::Sans,
                    2_500,
                    stroke.color,
                )?;
                y += 4_200;
            }
        }
        Ok(y + 3_000)
    }

    fn report_markdown(
        &mut self,
        markdown: &str,
        mut y: u64,
        page_bottom: u64,
        stroke: StrokeStyle,
        page_title: &str,
    ) -> Result<u64, HardcopyRenderError> {
        let mut in_code_block = false;
        let mut paragraph = String::new();
        let flush_paragraph = |this: &mut Self,
                               paragraph: &mut String,
                               y: &mut u64|
         -> Result<(), HardcopyRenderError> {
            if paragraph.is_empty() {
                return Ok(());
            }
            let (text, font) = parse_supported_inline_markdown(paragraph)?;
            for wrapped in wrap_text(&text, 112) {
                this.ensure_report_room(*y, 4_200, page_bottom, page_title)?;
                this.add_text(
                    ScenePoint::new(
                        Length::from_micrometres(20_000),
                        Length::from_micrometres(*y),
                    ),
                    &wrapped,
                    font,
                    2_500,
                    stroke.color,
                )?;
                *y += 4_200;
            }
            *y += 1_500;
            paragraph.clear();
            Ok(())
        };
        for raw in markdown.lines() {
            let line = raw.trim_end();
            if line.trim() == "```" {
                flush_paragraph(self, &mut paragraph, &mut y)?;
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block {
                self.ensure_report_room(y, 4_000, page_bottom, page_title)?;
                self.add_text(
                    ScenePoint::new(
                        Length::from_micrometres(22_000),
                        Length::from_micrometres(y),
                    ),
                    if line.is_empty() { " " } else { line },
                    SceneFont::Monospace,
                    2_300,
                    stroke.color,
                )?;
                y += 4_000;
                continue;
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                flush_paragraph(self, &mut paragraph, &mut y)?;
                continue;
            }
            if trimmed.starts_with('>')
                || trimmed.starts_with("![")
                || trimmed.contains("](")
                || trimmed.starts_with('<')
                || trimmed.matches('|').count() >= 2
            {
                return Err(HardcopyRenderError::UnsupportedReportMarkdown);
            }
            let heading =
                ["#### ", "### ", "## ", "# "]
                    .iter()
                    .enumerate()
                    .find_map(|(index, prefix)| {
                        trimmed
                            .strip_prefix(prefix)
                            .map(|text| (text, 3_000 + (3 - index as u64) * 350))
                    });
            if let Some((text, size)) = heading {
                flush_paragraph(self, &mut paragraph, &mut y)?;
                let (text, _) = parse_supported_inline_markdown(text)?;
                self.ensure_report_room(y, 5_500, page_bottom, page_title)?;
                self.add_text(
                    ScenePoint::new(
                        Length::from_micrometres(20_000),
                        Length::from_micrometres(y),
                    ),
                    &text,
                    SceneFont::SansSemibold,
                    size,
                    stroke.color,
                )?;
                y += 5_500;
                continue;
            }
            let unordered = trimmed
                .strip_prefix("- ")
                .or_else(|| trimmed.strip_prefix("* "));
            let ordered = trimmed
                .find(". ")
                .filter(|index| *index > 0 && trimmed[..*index].chars().all(|c| c.is_ascii_digit()))
                .map(|index| (&trimmed[..index + 1], &trimmed[index + 2..]));
            if unordered.is_some() || ordered.is_some() {
                flush_paragraph(self, &mut paragraph, &mut y)?;
                let (prefix, content) = match (unordered, ordered) {
                    (Some(content), _) => ("•".to_owned(), content),
                    (_, Some((number, content))) => (number.to_owned(), content),
                    _ => unreachable!(),
                };
                let (content, font) = parse_supported_inline_markdown(content)?;
                for wrapped in wrap_text(&format!("{prefix} {content}"), 106) {
                    self.ensure_report_room(y, 4_200, page_bottom, page_title)?;
                    self.add_text(
                        ScenePoint::new(
                            Length::from_micrometres(24_000),
                            Length::from_micrometres(y),
                        ),
                        &wrapped,
                        font,
                        2_500,
                        stroke.color,
                    )?;
                    y += 4_200;
                }
                continue;
            }
            if !paragraph.is_empty() {
                paragraph.push(' ');
            }
            paragraph.push_str(trimmed);
        }
        if in_code_block {
            return Err(HardcopyRenderError::UnsupportedReportMarkdown);
        }
        flush_paragraph(self, &mut paragraph, &mut y)?;
        Ok(y + 3_000)
    }
}
