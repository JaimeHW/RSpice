//! Compiling a semantic document into printable scene primitives.
//!
//! The compiler is the only place model coordinates become page coordinates,
//! and it lays content out in exact micrometres rather than device pixels — so
//! the same document compiles to the same scene at any resolution. Text that
//! does not fit is wrapped or reported, never clipped silently, because a
//! truncated value on a printed page is indistinguishable from a real one.

use super::*;
use rspice_design::schematic::{component_type::ComponentType, drawing_metrics};

pub(super) struct SemanticSceneCompiler<'a> {
    pub(super) bounds: SemanticBounds,
    pub(super) extent: ContentExtent,
    pub(super) mapping: &'a PrintMappingTable,
    pub(super) schematic_output: SchematicHardcopySetup,
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
        schematic_output: SchematicHardcopySetup,
    ) -> Self {
        Self {
            bounds,
            extent,
            mapping,
            schematic_output,
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
                schematic_output: self.schematic_output,
                primitives: Vec::new(),
                legend: Vec::new(),
                coordinate_offset: ScenePoint::new(
                    Length::from_micrometres(offset_x),
                    Length::from_micrometres(offset_y),
                ),
                aggregate_sections: Vec::new(),
                mapping_ordinal: Some(child.ordinal),
            };
            match (
                child.document.as_ref(),
                child.publication_page_label.as_deref(),
            ) {
                (HardcopySemanticDocument::Schematic(schematic), Some(page_label)) => {
                    compiler.schematic_with_page_label(schematic, Some(page_label))?;
                }
                _ => compiler.compile_document(&child.document)?,
            }
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
        self.schematic_with_page_label(schematic, None)
    }

    pub(super) fn schematic_with_page_label(
        &mut self,
        schematic: &SemanticSchematic,
        page_label: Option<&str>,
    ) -> Result<(), HardcopyRenderError> {
        if let Some(format) = &schematic.drawing_sheet {
            if let Some(page_label) = page_label {
                let mut title_values = schematic.drawing_sheet_title_values.clone();
                title_values.insert(DrawingSheetTitleFieldId::Page, page_label.to_owned());
                self.drawing_sheet(format, &title_values, schematic.grid_pitch_units)?;
            } else {
                self.drawing_sheet(
                    format,
                    &schematic.drawing_sheet_title_values,
                    schematic.grid_pitch_units,
                )?;
            }
        }
        // The SVG export's inks and nominal weights (D1 sections 1 and 11);
        // a print mapping entry for the layer still overrides them.
        let print_stroke = |ink: Rgb8, world: f32| {
            StrokeStyle::try_new(
                SemanticColor::Exact(ink),
                schematic_marks::width_um(world),
                StrokePattern::Solid,
                None,
            )
        };
        let component_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:schematic-components",
            print_stroke(
                schematic_marks::SYMBOL_INK,
                drawing_metrics::SYMBOL_WIDTH_WORLD,
            )?,
        );
        let component_fill = self.mapped_fill(
            PrintObjectKind::Layer,
            "layer:schematic-components",
            Some(SceneFill::solid(component_stroke.color)),
        );
        let wire_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:schematic-wiring",
            print_stroke(schematic_marks::WIRE_INK, drawing_metrics::WIRE_WIDTH_WORLD)?,
        );
        let bus_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:schematic-buses",
            print_stroke(
                schematic_marks::WIRE_INK,
                drawing_metrics::BUS_STRAND_WIDTH_WORLD,
            )?,
        );
        let tap_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:schematic-buses",
            print_stroke(
                schematic_marks::WIRE_INK,
                drawing_metrics::BUS_TAP_WIDTH_WORLD,
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
            self.schematic_bus(bus, bus_stroke)?;
        }
        for tap in &schematic.bus_taps {
            self.schematic_bus_tap(tap, tap_stroke)?;
        }
        for junction in &schematic.junctions {
            self.primitives.push(ScenePrimitive::Circle {
                center: self.schematic_point(junction.pos)?,
                radius: schematic_marks::width_um(drawing_metrics::JUNCTION_RADIUS_WORLD),
                stroke: None,
                fill: self.mapped_fill(
                    PrintObjectKind::Layer,
                    "layer:schematic-wiring",
                    Some(SceneFill::solid(wire_stroke.color)),
                ),
            });
        }
        for label in &schematic.net_labels {
            self.schematic_net_label(label)?;
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
                if semantic.component.kind == ComponentType::Port {
                    self.port_direction_marks(&library, &semantic.component, component_stroke)?;
                }
            }
            self.instance_labels(semantic)?;
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

    fn drawing_sheet(
        &mut self,
        format: &SchematicSheetFormat,
        title_values: &BTreeMap<DrawingSheetTitleFieldId, String>,
        grid_pitch_units: i32,
    ) -> Result<(), HardcopyRenderError> {
        let geometry = format
            .geometry()
            .map_err(|error| conversion_error(format!("drawing-sheet geometry: {error}")))?;
        let paper_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:drawing-sheet-paper",
            StrokeStyle::try_new(
                SemanticColor::Foreground,
                Length::from_micrometres(180),
                StrokePattern::Solid,
                None,
            )?,
        );
        let title_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:drawing-sheet-title-block",
            StrokeStyle::try_new(
                SemanticColor::Foreground,
                Length::from_micrometres(geometry.effective_title_block_template.outline_um()),
                StrokePattern::Solid,
                None,
            )?,
        );
        let grid_stroke = self.mapped_stroke(
            PrintObjectKind::Layer,
            "layer:schematic-grid",
            StrokeStyle::try_new(
                SemanticColor::Secondary,
                Length::from_micrometres(60),
                StrokePattern::Dotted,
                None,
            )?,
        );

        if self.schematic_output.includes_paper() && geometry.bleed != geometry.paper {
            self.primitives.push(ScenePrimitive::Rect {
                rect: self.drawing_sheet_rect(geometry.bleed)?,
                stroke: Some(StrokeStyle::try_new(
                    SemanticColor::Secondary,
                    Length::from_micrometres(100),
                    StrokePattern::Dotted,
                    None,
                )?),
                fill: None,
            });
        }
        if self.schematic_output.includes_paper() {
            self.primitives.push(ScenePrimitive::Rect {
                rect: self.drawing_sheet_rect(geometry.paper)?,
                stroke: Some(paper_stroke),
                fill: None,
            });
        }
        let frame_stroke = self.drawing_sheet_frame(format, &geometry)?;
        if self.schematic_output.includes_grid() {
            self.drawing_sheet_grid(geometry.drawing_area, grid_pitch_units, grid_stroke)?;
        }
        if self.schematic_output.includes_title_block()
            && let Some(title_block) = geometry.title_block
        {
            self.drawing_sheet_title_block(
                format,
                geometry.effective_title_block_template,
                title_values,
                title_block,
                title_stroke,
            )?;
            // Sheet one of several on the ASME form says every sheet is at
            // the same revision, where the canvas letters it.
            if let Some(note) = title_values
                .get(&DrawingSheetTitleFieldId::Page)
                .and_then(|page| format.same_revision_note(&geometry, page))
            {
                let baseline_um =
                    note.seat_um + drawing_sheet_mono_baseline_offset_um(note.lettering_um) as i64;
                self.add_text_rotated(
                    self.drawing_sheet_point(note.x_um, baseline_um)?,
                    note.text,
                    SceneFont::Monospace,
                    drawing_sheet_mono_em_um(note.lettering_um),
                    title_stroke.color,
                    SceneTextRotation::Upright,
                )?;
            }
        }
        if self.schematic_output.includes_paper() {
            self.add_mapping_legend(
                PrintObjectKind::Layer,
                "layer:drawing-sheet-paper",
                paper_stroke,
            )?;
        }
        if self.schematic_output.includes_border()
            || self.schematic_output.includes_zones()
            || self.schematic_output.crop_marks()
        {
            self.add_mapping_legend(
                PrintObjectKind::Layer,
                "layer:drawing-sheet-frame",
                frame_stroke,
            )?;
        }
        if self.schematic_output.includes_title_block() {
            self.add_mapping_legend(
                PrintObjectKind::Layer,
                "layer:drawing-sheet-title-block",
                title_stroke,
            )?;
        }
        if self.schematic_output.includes_grid() {
            self.add_mapping_legend(PrintObjectKind::Layer, "layer:schematic-grid", grid_stroke)?;
        }
        Ok(())
    }

    fn drawing_sheet_point(&self, x_um: i64, y_um: i64) -> Result<ScenePoint, HardcopyRenderError> {
        let origin_x = SCHEMATIC_SHEET_ORIGIN_X_UNITS
            .checked_mul(SCHEMATIC_UNIT_UM)
            .ok_or_else(|| conversion_error("drawing-sheet X origin overflow"))?;
        let origin_y = SCHEMATIC_SHEET_ORIGIN_Y_UNITS
            .checked_mul(SCHEMATIC_UNIT_UM)
            .ok_or_else(|| conversion_error("drawing-sheet Y origin overflow"))?;
        self.signed_micrometre_point(
            origin_x
                .checked_add(x_um)
                .ok_or_else(|| conversion_error("drawing-sheet X coordinate overflow"))?,
            origin_y
                .checked_add(y_um)
                .ok_or_else(|| conversion_error("drawing-sheet Y coordinate overflow"))?,
        )
    }

    fn drawing_sheet_rect(&self, rect: DrawingSheetRect) -> Result<SceneRect, HardcopyRenderError> {
        let origin = self.drawing_sheet_point(rect.x_um, rect.y_um)?;
        SceneRect::try_new(
            origin.x,
            origin.y,
            Length::from_micrometres(rect.width_um),
            Length::from_micrometres(rect.height_um),
        )
    }

    fn drawing_sheet_grid(
        &mut self,
        drawing_area: DrawingSheetRect,
        grid_pitch_units: i32,
        stroke: StrokeStyle,
    ) -> Result<(), HardcopyRenderError> {
        let pitch_um = i64::from(grid_pitch_units)
            .checked_mul(SCHEMATIC_UNIT_UM)
            .filter(|pitch| *pitch > 0)
            .ok_or_else(|| conversion_error("schematic grid pitch must be positive"))?;
        let right = drawing_area
            .x_um
            .checked_add(
                i64::try_from(drawing_area.width_um)
                    .map_err(|_| conversion_error("drawing-sheet grid width overflow"))?,
            )
            .ok_or_else(|| conversion_error("drawing-sheet grid width overflow"))?;
        let bottom = drawing_area
            .y_um
            .checked_add(
                i64::try_from(drawing_area.height_um)
                    .map_err(|_| conversion_error("drawing-sheet grid height overflow"))?,
            )
            .ok_or_else(|| conversion_error("drawing-sheet grid height overflow"))?;
        let columns = drawing_area.width_um / pitch_um as u64 + 2;
        let rows = drawing_area.height_um / pitch_um as u64 + 2;
        const MAX_GRID_RULES: u64 = 100_000;
        if columns.saturating_add(rows) > MAX_GRID_RULES {
            return Err(HardcopyRenderError::ResourceLimit {
                scope: "schematic grid rules",
                maximum: MAX_GRID_RULES,
            });
        }

        let sheet_origin_x = SCHEMATIC_SHEET_ORIGIN_X_UNITS
            .checked_mul(SCHEMATIC_UNIT_UM)
            .ok_or_else(|| conversion_error("drawing-sheet grid X origin overflow"))?;
        let sheet_origin_y = SCHEMATIC_SHEET_ORIGIN_Y_UNITS
            .checked_mul(SCHEMATIC_UNIT_UM)
            .ok_or_else(|| conversion_error("drawing-sheet grid Y origin overflow"))?;
        let first_x = drawing_area.x_um
            + (pitch_um - (sheet_origin_x + drawing_area.x_um).rem_euclid(pitch_um))
                .rem_euclid(pitch_um);
        let first_y = drawing_area.y_um
            + (pitch_um - (sheet_origin_y + drawing_area.y_um).rem_euclid(pitch_um))
                .rem_euclid(pitch_um);

        let mut x = first_x;
        while x <= right {
            self.primitives.push(ScenePrimitive::Line {
                from: self.drawing_sheet_point(x, drawing_area.y_um)?,
                to: self.drawing_sheet_point(x, bottom)?,
                stroke,
            });
            x = x
                .checked_add(pitch_um)
                .ok_or_else(|| conversion_error("drawing-sheet grid X overflow"))?;
        }
        let mut y = first_y;
        while y <= bottom {
            self.primitives.push(ScenePrimitive::Line {
                from: self.drawing_sheet_point(drawing_area.x_um, y)?,
                to: self.drawing_sheet_point(right, y)?,
                stroke,
            });
            y = y
                .checked_add(pitch_um)
                .ok_or_else(|| conversion_error("drawing-sheet grid Y overflow"))?;
        }
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
        self.add_text_placed(
            origin,
            text,
            font,
            size_um,
            color,
            (TextAnchor::Start, SceneTextRotation::Upright),
        )
    }

    fn add_text_rotated(
        &mut self,
        origin: ScenePoint,
        text: &str,
        font: SceneFont,
        size_um: u64,
        color: SemanticColor,
        rotation: SceneTextRotation,
    ) -> Result<(), HardcopyRenderError> {
        self.add_text_placed(
            origin,
            text,
            font,
            size_um,
            color,
            (TextAnchor::Start, rotation),
        )
    }

    fn add_text_placed(
        &mut self,
        origin: ScenePoint,
        text: &str,
        font: SceneFont,
        size_um: u64,
        color: SemanticColor,
        (anchor, rotation): (TextAnchor, SceneTextRotation),
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
            anchor,
            rotation,
        });
        Ok(())
    }

    fn library_component(
        &mut self,
        library: &SymbolLibrary,
        component: &Component,
        stroke: StrokeStyle,
        mapped_fill: Option<SceneFill>,
    ) -> Result<(), HardcopyRenderError> {
        let builtin_asset = component
            .library_cell
            .as_ref()
            .and_then(|binding| binding.builtin_xspice.as_ref())
            .and_then(|contract| {
                let (width, height) = component.artwork_dimensions();
                let offsets = component.artwork_pin_offsets();
                library
                    .asset_matches_terminal_offsets(
                        &contract.symbol_asset,
                        width as f32,
                        height as f32,
                        &offsets,
                    )
                    .then(|| {
                        library.get_asset(&contract.symbol_asset).map(|symbol| {
                            (
                                symbol,
                                component.rotation.degrees(),
                                width as f32,
                                height as f32,
                            )
                        })
                    })
                    .flatten()
            });
        let drew_builtin_asset = builtin_asset.is_some();
        let (symbol, rotation, target_width, target_height) = if let Some(asset) = builtin_asset {
            asset
        } else if component
            .library_cell
            .as_ref()
            .is_some_and(|binding| binding.is_executable_builtin())
        {
            return self.generated_cell_instance(component, stroke);
        } else {
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
            (symbol, rotation, symbol.target_width, symbol.target_height)
        };
        let (cx, cy) = symbol.center();
        let scale_x = f64::from(target_width / symbol.width().max(0.001));
        let scale_y = f64::from(target_height / symbol.height().max(0.001));
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
        if drew_builtin_asset {
            self.artwork_lead_extensions(component, stroke)?;
        }
        Ok(())
    }

    fn generated_cell_instance(
        &mut self,
        component: &Component,
        stroke: StrokeStyle,
    ) -> Result<(), HardcopyRenderError> {
        let world = |local: rspice_design_model::Point| {
            let transformed = component.transform_point(local);
            SchematicPoint::new(
                component.pos.x.saturating_add(transformed.x),
                component.pos.y.saturating_add(transformed.y),
            )
        };
        let block = component.instance_block();
        let (min, max) = block.body;
        let corners = [
            world(min),
            world(rspice_design_model::Point::new(max.x, min.y)),
            world(max),
            world(rspice_design_model::Point::new(min.x, max.y)),
        ];
        self.primitives.push(ScenePrimitive::Polyline {
            points: corners
                .into_iter()
                .map(|point| self.schematic_point(point))
                .collect::<Result<_, _>>()?,
            closed: true,
            stroke,
            fill: None,
        });

        for pin in block.pins {
            let inner = rspice_design::symbol_generation::lead_inner(
                pin.offset,
                pin.side,
                Some(block.body),
            );
            self.primitives.push(ScenePrimitive::Line {
                from: self.schematic_point(world(pin.offset))?,
                to: self.schematic_point(world(inner))?,
                stroke,
            });
            self.primitives.push(ScenePrimitive::Circle {
                center: self.schematic_point(world(pin.offset))?,
                radius: Length::from_micrometres(500),
                stroke: Some(stroke),
                fill: None,
            });
            if !pin.name.is_empty() {
                let anchor = rspice_design::symbol_generation::pin_label_anchor(
                    pin.offset,
                    pin.side,
                    Some(block.body),
                );
                self.add_text(
                    self.schematic_point(world(anchor))?,
                    &rspice_design::symbol_generation::fit_pin_name(&pin.name),
                    SceneFont::Sans,
                    1_900,
                    stroke.color,
                )?;
            }
        }
        Ok(())
    }

    /// Carry artwork leads out to terminals the drawing itself does not reach.
    fn artwork_lead_extensions(
        &mut self,
        component: &Component,
        stroke: StrokeStyle,
    ) -> Result<(), HardcopyRenderError> {
        let world = |local: rspice_design_model::Point| {
            let transformed = component.transform_point(local);
            SchematicPoint::new(
                component.pos.x.saturating_add(transformed.x),
                component.pos.y.saturating_add(transformed.y),
            )
        };
        for (edge, terminal) in component.artwork_lead_extensions() {
            self.primitives.push(ScenePrimitive::Line {
                from: self.schematic_point(world(edge))?,
                to: self.schematic_point(world(terminal))?,
                stroke,
            });
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
        self.plot_grid(plot, frame)?;
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
                if let [point] = path.as_slice() {
                    self.primitives.push(ScenePrimitive::Circle {
                        center: self.semantic_point(*point)?,
                        radius: Length::from_micrometres(500),
                        stroke: Some(stroke),
                        fill: None,
                    });
                } else if path.len() >= 2 {
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
        for cursor in &plot.cursors {
            let start = self.semantic_point(cursor.start)?;
            let end = self.semantic_point(cursor.end)?;
            let stable_id = format!("cursor:{}", cursor.cursor_id);
            let stroke = self.mapped_stroke(
                PrintObjectKind::Marker,
                &stable_id,
                StrokeStyle::try_new(
                    SemanticColor::Accent,
                    Length::from_micrometres(200),
                    StrokePattern::Dashed,
                    None,
                )?,
            );
            self.primitives.push(ScenePrimitive::Polyline {
                points: vec![start, end],
                closed: false,
                stroke,
                fill: None,
            });
            let label = self.offset_scene_point(end, 600, 2_400)?;
            self.add_text(
                label,
                &cursor.label,
                SceneFont::Monospace,
                2_300,
                stroke.color,
            )?;
            self.add_mapping_legend(PrintObjectKind::Marker, &stable_id, stroke)?;
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
        for caption in &plot.captions {
            self.add_text(
                self.semantic_point(caption.position)?,
                &caption.text,
                SceneFont::Sans,
                2_300,
                SemanticColor::Secondary,
            )?;
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
                        block,
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
        block: &ReportBlock,
        figure: Option<&SemanticReportFigure>,
        mut y: u64,
        page_bottom: u64,
        stroke: StrokeStyle,
        page_title: &str,
    ) -> Result<u64, HardcopyRenderError> {
        let block_id = block.id();
        let mut lines = Vec::<String>::new();
        match block.kind() {
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

mod plot_grid;
mod schematic_marks;
mod schematic_text;
mod sheet_frame;
mod symbol_document;
mod title_block;
