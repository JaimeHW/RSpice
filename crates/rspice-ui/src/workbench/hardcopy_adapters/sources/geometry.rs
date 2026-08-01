//! Placing model coordinates onto a printable page.
//!
//! Bounds, layer mappings, and plot geometry are derived from the semantic
//! document rather than from anything the screen did, and every path is
//! clipped to the axis rectangle it belongs to before it is emitted. A
//! degenerate range is widened explicitly instead of being divided by, so a
//! flat trace prints as a flat line rather than as nothing at all.

use super::*;

pub(super) fn layer_mapping(
    stable_id: impl Into<String>,
    display_name: impl Into<String>,
    screen_style: impl Into<String>,
) -> Result<PrintMappingEntry, HardcopySourceError> {
    mapping_entry(
        PrintObjectKind::Layer,
        stable_id,
        display_name,
        screen_style,
        PrintColor::GrayPercent(60),
        PrintRedundancy::SourceStyle,
        true,
    )
}

pub(super) fn mapping_entry(
    kind: PrintObjectKind,
    stable_id: impl Into<String>,
    display_name: impl Into<String>,
    screen_style: impl Into<String>,
    color: PrintColor,
    redundancy: PrintRedundancy,
    include_in_legend: bool,
) -> Result<PrintMappingEntry, HardcopySourceError> {
    let identity = PrintObjectIdentity::try_new(kind, stable_id, display_name, screen_style)
        .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?;
    PrintMappingEntry::try_new(identity, color, redundancy, include_in_legend)
        .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))
}

pub(super) fn compact_display(text: &str, fallback: &str) -> String {
    let compact = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let compact = compact.chars().take(96).collect::<String>();
    if compact.is_empty() {
        fallback.to_owned()
    } else {
        compact
    }
}

pub(super) fn semantic_is_empty(schematic: &SemanticSchematic) -> bool {
    schematic.components.is_empty()
        && schematic.wires.is_empty()
        && schematic.buses.is_empty()
        && schematic.bus_taps.is_empty()
        && schematic.junctions.is_empty()
        && schematic.net_labels.is_empty()
        && schematic.design_notes.is_empty()
        && schematic.documentation_shapes.is_empty()
}

#[derive(Default)]
pub(super) struct BoundsAccumulator {
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
    initialized: bool,
}

impl BoundsAccumulator {
    fn include(&mut self, point: Point) {
        self.include_i64(i64::from(point.x), i64::from(point.y));
    }

    fn include_i64(&mut self, x: i64, y: i64) {
        if self.initialized {
            self.min_x = self.min_x.min(x);
            self.min_y = self.min_y.min(y);
            self.max_x = self.max_x.max(x);
            self.max_y = self.max_y.max(y);
        } else {
            self.min_x = x;
            self.min_y = y;
            self.max_x = x;
            self.max_y = y;
            self.initialized = true;
        }
    }

    fn finish(self, allowance_units: i64) -> Result<SemanticBounds, HardcopySourceError> {
        if !self.initialized {
            return Err(HardcopySourceError::EmptyContent);
        }
        let min_x = self
            .min_x
            .checked_sub(allowance_units)
            .and_then(|value| value.checked_mul(SCHEMATIC_UNIT_UM))
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        let min_y = self
            .min_y
            .checked_sub(allowance_units)
            .and_then(|value| value.checked_mul(SCHEMATIC_UNIT_UM))
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        let max_x = self
            .max_x
            .checked_add(allowance_units)
            .and_then(|value| value.checked_mul(SCHEMATIC_UNIT_UM))
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        let max_y = self
            .max_y
            .checked_add(allowance_units)
            .and_then(|value| value.checked_mul(SCHEMATIC_UNIT_UM))
            .ok_or(HardcopySourceError::CoordinateOverflow)?;
        SemanticBounds::try_new(
            SemanticPoint::new(min_x, min_y),
            SemanticPoint::new(max_x, max_y),
        )
    }
}

pub(super) fn schematic_bounds(
    schematic: &SemanticSchematic,
) -> Result<SemanticBounds, HardcopySourceError> {
    let mut bounds = BoundsAccumulator::default();
    for component in &schematic.components {
        if let Some(symbol) = &component.resolved_symbol {
            include_symbol_document_bounds(&mut bounds, symbol, Some(&component.component));
        } else {
            let (min_x, min_y, max_x, max_y) = component.component.bounding_box();
            bounds.include(Point::new(min_x, min_y));
            bounds.include(Point::new(max_x, max_y));
        }
        // Component labels are semantic document content and can extend past
        // the body. The edge allowance covers the font ascent and short names;
        // long labels add deterministic horizontal width.
        let label_chars = component
            .component
            .name
            .len()
            .max(component.component.value.len());
        let label_width = i64::try_from(label_chars)
            .unwrap_or(i64::MAX)
            .saturating_mul(2);
        bounds.include_i64(
            i64::from(component.component.pos.x).saturating_add(label_width),
            i64::from(component.component.pos.y),
        );
    }
    for wire in &schematic.wires {
        wire.points
            .iter()
            .copied()
            .for_each(|point| bounds.include(point));
    }
    for bus in &schematic.buses {
        bus.points
            .iter()
            .copied()
            .for_each(|point| bounds.include(point));
    }
    for tap in &schematic.bus_taps {
        bounds.include(tap.bus_point);
        bounds.include(tap.connection_point);
    }
    for junction in &schematic.junctions {
        bounds.include(junction.pos);
    }
    for label in &schematic.net_labels {
        bounds.include(label.pos);
        bounds.include_i64(
            i64::from(label.pos.x).saturating_add(
                i64::try_from(label.name.len())
                    .unwrap_or(i64::MAX)
                    .saturating_mul(2),
            ),
            i64::from(label.pos.y),
        );
    }
    for note in &schematic.design_notes {
        bounds.include(note.pos);
        let lines = note.text.lines().collect::<Vec<_>>();
        let width = lines.iter().map(|line| line.len()).max().unwrap_or(1);
        bounds.include_i64(
            i64::from(note.pos.x)
                .saturating_add(i64::try_from(width).unwrap_or(i64::MAX).saturating_mul(2)),
            i64::from(note.pos.y).saturating_add(
                i64::try_from(lines.len())
                    .unwrap_or(i64::MAX)
                    .saturating_mul(4),
            ),
        );
    }
    for shape in &schematic.documentation_shapes {
        // Documentation arcs can sweep well beyond their three authored
        // control points. Use the shape authority's exact circular-extrema
        // bounds so hardcopy Ask/Extend decisions and authenticated renderer
        // bounds agree with the geometry that is actually emitted.
        let (minimum, maximum) = shape.bounds();
        bounds.include(minimum);
        bounds.include(maximum);
    }
    bounds.finish(SCHEMATIC_EDGE_ALLOWANCE_UNITS)
}

pub(crate) fn authored_sheet_bounds(
    format: &SchematicSheetFormat,
) -> Result<SemanticBounds, HardcopySourceError> {
    let origin_x = SCHEMATIC_SHEET_ORIGIN_X_UNITS
        .checked_mul(SCHEMATIC_UNIT_UM)
        .ok_or(HardcopySourceError::CoordinateOverflow)?;
    let origin_y = SCHEMATIC_SHEET_ORIGIN_Y_UNITS
        .checked_mul(SCHEMATIC_UNIT_UM)
        .ok_or(HardcopySourceError::CoordinateOverflow)?;
    let geometry = format
        .geometry()
        .map_err(|error| HardcopySourceError::InvalidSheetPartition(error.to_string()))?;
    // Authored-sheet clipping is governed by the physical paper edge. Bleed
    // is export artwork outside that edge and must never make off-sheet
    // schematic content appear to be on the authored sheet.
    let paper = geometry.paper;
    let minimum_x = origin_x
        .checked_add(paper.x_um)
        .ok_or(HardcopySourceError::CoordinateOverflow)?;
    let minimum_y = origin_y
        .checked_add(paper.y_um)
        .ok_or(HardcopySourceError::CoordinateOverflow)?;
    let width_um =
        i64::try_from(paper.width_um).map_err(|_| HardcopySourceError::CoordinateOverflow)?;
    let height_um =
        i64::try_from(paper.height_um).map_err(|_| HardcopySourceError::CoordinateOverflow)?;
    SemanticBounds::try_new(
        SemanticPoint::new(minimum_x, minimum_y),
        SemanticPoint::new(
            minimum_x
                .checked_add(width_um)
                .ok_or(HardcopySourceError::CoordinateOverflow)?,
            minimum_y
                .checked_add(height_um)
                .ok_or(HardcopySourceError::CoordinateOverflow)?,
        ),
    )
}

pub(super) fn union_bounds(first: SemanticBounds, second: SemanticBounds) -> SemanticBounds {
    SemanticBounds {
        minimum: SemanticPoint::new(
            first.minimum.x_um.min(second.minimum.x_um),
            first.minimum.y_um.min(second.minimum.y_um),
        ),
        maximum: SemanticPoint::new(
            first.maximum.x_um.max(second.maximum.x_um),
            first.maximum.y_um.max(second.maximum.y_um),
        ),
    }
}

pub(super) fn symbol_bounds(
    document: &SymbolDocument,
) -> Result<SemanticBounds, HardcopySourceError> {
    let mut bounds = BoundsAccumulator::default();
    include_symbol_document_bounds(&mut bounds, document, None);
    bounds.finish(SYMBOL_EDGE_ALLOWANCE_UNITS)
}

pub(super) fn include_symbol_document_bounds(
    bounds: &mut BoundsAccumulator,
    document: &SymbolDocument,
    component: Option<&Component>,
) {
    let map = |point: Point| {
        let effective = point - document.origin;
        component.map_or(effective, |component| {
            component.pos + component.transform_point(effective)
        })
    };
    for shape in &document.body {
        match shape {
            SymbolShape::Polyline { points, .. } => {
                points
                    .iter()
                    .copied()
                    .map(map)
                    .for_each(|point| bounds.include(point));
            }
            SymbolShape::Circle { center, radius } | SymbolShape::Dot { center, radius } => {
                let center = map(*center);
                let radius = radius.unsigned_abs().min(i32::MAX as u32) as i32;
                bounds.include(Point::new(
                    center.x.saturating_sub(radius),
                    center.y.saturating_sub(radius),
                ));
                bounds.include(Point::new(
                    center.x.saturating_add(radius),
                    center.y.saturating_add(radius),
                ));
            }
            SymbolShape::Arc { center, radius, .. } => {
                // Conservative full-circle bounds avoid platform-dependent
                // trigonometric extrema while never clipping an authored arc.
                let center = map(*center);
                let radius = radius.unsigned_abs().min(i32::MAX as u32) as i32;
                bounds.include(Point::new(
                    center.x.saturating_sub(radius),
                    center.y.saturating_sub(radius),
                ));
                bounds.include(Point::new(
                    center.x.saturating_add(radius),
                    center.y.saturating_add(radius),
                ));
            }
            SymbolShape::Arrow { tip, .. } => bounds.include(map(*tip)),
        }
    }
    for pin in &document.pins {
        if let Some(point) = pin.position {
            bounds.include(map(point));
        }
    }
    bounds.include(map(document.name_anchor));
    bounds.include(map(document.value_anchor));
}

pub(super) fn map_result_coordinate(
    value: f64,
    minimum: f64,
    span: f64,
    inset: i64,
    physical_span: i64,
) -> Result<i64, HardcopySourceError> {
    let normalized = (value - minimum) / span;
    let mapped = inset as f64 + normalized * physical_span as f64;
    if !mapped.is_finite() || mapped < i64::MIN as f64 || mapped > i64::MAX as f64 {
        return Err(HardcopySourceError::CoordinateOverflow);
    }
    Ok(mapped.round() as i64)
}

pub(super) fn clipped_plot_paths(
    points: &[(f64, f64)],
    x_minimum: f64,
    x_maximum: f64,
    y_minimum: f64,
    y_maximum: f64,
    plot_width: i64,
    plot_height: i64,
) -> Result<Vec<Vec<SemanticPoint>>, HardcopySourceError> {
    if points
        .iter()
        .any(|point| !point.0.is_finite() || !point.1.is_finite())
    {
        return Err(HardcopySourceError::NonFiniteResultSample);
    }
    let x_span = x_maximum - x_minimum;
    let y_span = y_maximum - y_minimum;
    if points.len() == 1 {
        let point = points[0];
        if point.0 < x_minimum || point.0 > x_maximum || point.1 < y_minimum || point.1 > y_maximum
        {
            return Ok(Vec::new());
        }
        return Ok(vec![vec![map_plot_point(
            point.0,
            point.1,
            x_minimum,
            y_minimum,
            x_span,
            y_span,
            plot_width,
            plot_height,
        )?]]);
    }

    let mut paths: Vec<Vec<SemanticPoint>> = Vec::new();
    for pair in points.windows(2) {
        let Some(((start_x, start_y), (end_x, end_y))) =
            clip_line_to_axis_rect(pair[0], pair[1], x_minimum, x_maximum, y_minimum, y_maximum)
        else {
            continue;
        };
        let start = map_plot_point(
            start_x,
            start_y,
            x_minimum,
            y_minimum,
            x_span,
            y_span,
            plot_width,
            plot_height,
        )?;
        let end = map_plot_point(
            end_x,
            end_y,
            x_minimum,
            y_minimum,
            x_span,
            y_span,
            plot_width,
            plot_height,
        )?;
        if let Some(path) = paths.last_mut()
            && path.last() == Some(&start)
        {
            if path.last() != Some(&end) {
                path.push(end);
            }
        } else {
            paths.push(if start == end {
                vec![start]
            } else {
                vec![start, end]
            });
        }
    }
    Ok(paths)
}

pub(super) fn canonical_marker_semantics(
    scene: &ResolvedCartesianLineScene,
    marker: &crate::results::visualization_document::Marker,
) -> SemanticPlotMarker {
    let source_x = typed_numeric_value(&marker.coordinate);
    let source_y = source_x.and_then(|x| {
        scene
            .traces()
            .iter()
            .find(|trace| trace.trace_id() == marker.trace_id)
            .and_then(|trace| trace_y_at_x(trace.points(), x))
    });
    SemanticPlotMarker {
        marker_id: marker.id.get(),
        label: marker.label.clone(),
        trace_id: Some(marker.trace_id.get()),
        source_x_bits: source_x.map(f64::to_bits),
        source_y_bits: source_y.map(f64::to_bits),
        position: source_x
            .zip(source_y)
            .and_then(|(x, y)| semantic_position_in_scene(scene, x, y).ok()),
    }
}

pub(super) fn canonical_annotation_semantics(
    scene: &ResolvedCartesianLineScene,
    annotation: &crate::results::visualization_document::Annotation,
) -> SemanticPlotAnnotation {
    let (trace_id, source_x, source_y, position) = match &annotation.anchor {
        AnnotationAnchor::Pane {
            x_fraction,
            y_fraction,
        } => {
            let x_fraction = f64::from(*x_fraction).clamp(0.0, 1.0);
            let y_fraction = f64::from(*y_fraction).clamp(0.0, 1.0);
            let x = scene.x_range().minimum
                + x_fraction * (scene.x_range().maximum - scene.x_range().minimum);
            let y = scene.y_range().maximum
                - y_fraction * (scene.y_range().maximum - scene.y_range().minimum);
            (
                None,
                Some(x),
                Some(y),
                semantic_position_in_scene(scene, x, y).ok(),
            )
        }
        AnnotationAnchor::Trace {
            trace_id,
            coordinate,
        } => {
            let x = typed_numeric_value(coordinate);
            let y = x.and_then(|x| {
                scene
                    .traces()
                    .iter()
                    .find(|trace| trace.trace_id() == *trace_id)
                    .and_then(|trace| trace_y_at_x(trace.points(), x))
            });
            (
                Some(trace_id.get()),
                x,
                y,
                x.zip(y)
                    .and_then(|(x, y)| semantic_position_in_scene(scene, x, y).ok()),
            )
        }
    };
    SemanticPlotAnnotation {
        annotation_id: annotation.id.get(),
        text: annotation.text.clone(),
        trace_id,
        source_x_bits: source_x.map(f64::to_bits),
        source_y_bits: source_y.map(f64::to_bits),
        position,
    }
}

pub(super) fn typed_numeric_value(value: &TypedValue) -> Option<f64> {
    match value {
        TypedValue::Real(value) => Some(*value),
        TypedValue::Integer(value) => Some(*value as f64),
        TypedValue::Boolean(_) | TypedValue::Text(_) => None,
    }
}

pub(super) fn trace_y_at_x(
    points: &[crate::results::visualization_raster::ResolvedRasterPoint],
    x: f64,
) -> Option<f64> {
    if let Some(point) = points
        .iter()
        .find(|point| point.x().to_bits() == x.to_bits())
    {
        return Some(point.y());
    }
    points.windows(2).find_map(|pair| {
        let left = pair[0];
        let right = pair[1];
        if (left.x() <= x && x <= right.x()) || (right.x() <= x && x <= left.x()) {
            let span = right.x() - left.x();
            if span == 0.0 {
                Some(left.y())
            } else {
                let fraction = (x - left.x()) / span;
                Some(left.y() + fraction * (right.y() - left.y()))
            }
        } else {
            None
        }
    })
}

pub(super) fn semantic_position_in_scene(
    scene: &ResolvedCartesianLineScene,
    x: f64,
    y: f64,
) -> Result<SemanticPoint, HardcopySourceError> {
    let x_range = scene.x_range();
    let y_range = scene.y_range();
    map_plot_point(
        x.clamp(x_range.minimum, x_range.maximum),
        y.clamp(y_range.minimum, y_range.maximum),
        x_range.minimum,
        y_range.minimum,
        x_range.maximum - x_range.minimum,
        y_range.maximum - y_range.minimum,
        PLOT_WIDTH_UM - 2 * PLOT_INSET_UM,
        PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM,
    )
}

pub(super) fn map_plot_point(
    x: f64,
    y: f64,
    x_minimum: f64,
    y_minimum: f64,
    x_span: f64,
    y_span: f64,
    plot_width: i64,
    plot_height: i64,
) -> Result<SemanticPoint, HardcopySourceError> {
    let mapped_x = map_result_coordinate(x, x_minimum, x_span, PLOT_INSET_UM, plot_width)?;
    let mapped_y = map_result_coordinate(y, y_minimum, y_span, PLOT_INSET_UM, plot_height)?;
    Ok(SemanticPoint::new(mapped_x, PLOT_HEIGHT_UM - mapped_y))
}

/// Liang-Barsky clipping avoids raster-size-dependent results and retains
/// exact line intersections with the authored axis rectangle.
pub(super) fn clip_line_to_axis_rect(
    start: (f64, f64),
    end: (f64, f64),
    x_minimum: f64,
    x_maximum: f64,
    y_minimum: f64,
    y_maximum: f64,
) -> Option<((f64, f64), (f64, f64))> {
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let mut lower = 0.0_f64;
    let mut upper = 1.0_f64;
    for (p, q) in [
        (-dx, start.0 - x_minimum),
        (dx, x_maximum - start.0),
        (-dy, start.1 - y_minimum),
        (dy, y_maximum - start.1),
    ] {
        if p == 0.0 {
            if q < 0.0 {
                return None;
            }
            continue;
        }
        let ratio = q / p;
        if p < 0.0 {
            if ratio > upper {
                return None;
            }
            lower = lower.max(ratio);
        } else {
            if ratio < lower {
                return None;
            }
            upper = upper.min(ratio);
        }
    }
    Some((
        (start.0 + lower * dx, start.1 + lower * dy),
        (start.0 + upper * dx, start.1 + upper * dy),
    ))
}

/// Pin a resolved live document into a persistable source-set member.
///
/// This constructor stays with the adapters because it reads
/// [`ResolvedHardcopyDocument`], which resolves live schematic and result
/// documents. The record type itself is owned by `crate::hardcopy::sources`.
impl HardcopySourceSetMember {
    pub fn from_resolved(resolved: &ResolvedHardcopyDocument) -> Result<Self, HardcopySourceError> {
        Self::try_new(
            resolved.source_key(),
            resolved.authority().display_name(),
            resolved.authority().document_id(),
            resolved.authority().revision(),
            resolved.authority().content_digest(),
            resolved.authority().scope().clone(),
        )
    }
}

#[cfg(test)]
mod drawing_sheet_geometry_tests {
    use super::*;
    use crate::state::DocumentationShapeGeometry;

    #[test]
    fn authored_sheet_bounds_stop_at_paper_even_when_bleed_is_configured() {
        let format = SchematicSheetFormat::default()
            .try_update(|draft| draft.bleed_um = 5_000)
            .unwrap();
        let bounds = authored_sheet_bounds(&format).unwrap();
        let (width_um, height_um) = format.oriented_dimensions_um();
        assert_eq!(bounds.maximum.x_um - bounds.minimum.x_um, width_um as i64);
        assert_eq!(bounds.maximum.y_um - bounds.minimum.y_um, height_um as i64);
    }

    #[test]
    fn schematic_bounds_include_major_arc_cardinal_extrema() {
        let arc = DocumentationShape::new(
            1,
            DocumentationShapeGeometry::Arc {
                start: Point::new(10, 0),
                through: Point::new(0, -10),
                end: Point::new(0, 10),
            },
        )
        .unwrap();
        let schematic = SemanticSchematic {
            view_path: "library/cell/schematic".to_owned(),
            drawing_sheet: None,
            drawing_sheet_title_values: std::collections::BTreeMap::new(),
            grid_pitch_units: 10,
            components: Vec::new(),
            wires: Vec::new(),
            buses: Vec::new(),
            bus_taps: Vec::new(),
            junctions: Vec::new(),
            net_labels: Vec::new(),
            design_notes: Vec::new(),
            documentation_shapes: vec![arc],
        };
        let bounds = schematic_bounds(&schematic).unwrap();
        assert_eq!(
            bounds.minimum.x_um,
            (-10 - SCHEMATIC_EDGE_ALLOWANCE_UNITS) * SCHEMATIC_UNIT_UM
        );
        assert_eq!(
            bounds.maximum.x_um,
            (10 + SCHEMATIC_EDGE_ALLOWANCE_UNITS) * SCHEMATIC_UNIT_UM
        );
    }
}
