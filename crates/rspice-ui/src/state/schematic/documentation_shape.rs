//! Durable, non-electrical schematic presentation geometry.
//!
//! Documentation shapes are document data, but never conductor data. They
//! participate in selection, editing, persistence, recovery, and export while
//! remaining invisible to connectivity and generated SPICE.

use serde::{Deserialize, Serialize};

use super::{Point, SchematicState};

pub const MAX_DOCUMENTATION_POLYGON_POINTS: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentationShapeKind {
    #[default]
    Rectangle,
    Line,
    Polygon,
    Arc,
    Callout,
}

impl DocumentationShapeKind {
    pub const ALL: [Self; 5] = [
        Self::Rectangle,
        Self::Line,
        Self::Polygon,
        Self::Arc,
        Self::Callout,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Rectangle => "Rectangle",
            Self::Line => "Line",
            Self::Polygon => "Polygon",
            Self::Arc => "Arc",
            Self::Callout => "Callout",
        }
    }

    pub const fn minimum_points(self) -> usize {
        match self {
            Self::Rectangle | Self::Line => 2,
            Self::Polygon | Self::Arc | Self::Callout => 3,
        }
    }

    pub const fn commits_automatically(self) -> bool {
        !matches!(self, Self::Polygon)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentationShapeLayer {
    #[default]
    DrawingDocumentation,
}

impl DocumentationShapeLayer {
    pub const fn label(self) -> &'static str {
        match self {
            Self::DrawingDocumentation => "drawing / documentation",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DocumentationShapeGeometry {
    Rectangle {
        first: Point,
        opposite: Point,
    },
    Line {
        start: Point,
        end: Point,
    },
    Polygon {
        points: Vec<Point>,
    },
    /// Three-point arc. The unique circular sweep begins at `start`, passes
    /// through `through`, and terminates at `end`.
    Arc {
        start: Point,
        through: Point,
        end: Point,
    },
    /// Leader tip, elbow, and opposite box corner. The elbow is also the
    /// nearest corner of the rounded callout box.
    Callout {
        tip: Point,
        elbow: Point,
        box_corner: Point,
    },
}

impl DocumentationShapeGeometry {
    pub const fn kind(&self) -> DocumentationShapeKind {
        match self {
            Self::Rectangle { .. } => DocumentationShapeKind::Rectangle,
            Self::Line { .. } => DocumentationShapeKind::Line,
            Self::Polygon { .. } => DocumentationShapeKind::Polygon,
            Self::Arc { .. } => DocumentationShapeKind::Arc,
            Self::Callout { .. } => DocumentationShapeKind::Callout,
        }
    }

    pub fn validate(&self) -> Result<(), DocumentationShapeError> {
        match self {
            Self::Rectangle { first, opposite } => {
                if first.x == opposite.x || first.y == opposite.y {
                    return Err(DocumentationShapeError::DegenerateGeometry);
                }
            }
            Self::Line { start, end } => {
                if start == end {
                    return Err(DocumentationShapeError::DegenerateGeometry);
                }
            }
            Self::Polygon { points } => {
                if points.len() < 3 {
                    return Err(DocumentationShapeError::InsufficientPoints);
                }
                if points.len() > MAX_DOCUMENTATION_POLYGON_POINTS {
                    return Err(DocumentationShapeError::TooManyPoints);
                }
                if points.first() == points.last()
                    || points.windows(2).any(|pair| pair[0] == pair[1])
                    || signed_double_area(points) == 0
                    || polygon_self_intersects(points)
                {
                    return Err(DocumentationShapeError::DegenerateGeometry);
                }
            }
            Self::Arc {
                start,
                through,
                end,
            } => {
                if arc_parameters(*start, *through, *end).is_none() {
                    return Err(DocumentationShapeError::DegenerateGeometry);
                }
            }
            Self::Callout {
                tip,
                elbow,
                box_corner,
            } => {
                if tip == elbow || elbow.x == box_corner.x || elbow.y == box_corner.y {
                    return Err(DocumentationShapeError::DegenerateGeometry);
                }
            }
        }
        Ok(())
    }

    pub fn translate(&mut self, delta: Point) {
        let points = self.points();
        let min_x = points.iter().map(|point| point.x).min().unwrap_or(0);
        let max_x = points.iter().map(|point| point.x).max().unwrap_or(0);
        let min_y = points.iter().map(|point| point.y).min().unwrap_or(0);
        let max_y = points.iter().map(|point| point.y).max().unwrap_or(0);
        let delta = Point::new(
            delta.x.clamp(
                i32::MIN.saturating_sub(min_x),
                i32::MAX.saturating_sub(max_x),
            ),
            delta.y.clamp(
                i32::MIN.saturating_sub(min_y),
                i32::MAX.saturating_sub(max_y),
            ),
        );
        let translate = |point: &mut Point| {
            point.x += delta.x;
            point.y += delta.y;
        };
        match self {
            Self::Rectangle { first, opposite } => {
                translate(first);
                translate(opposite);
            }
            Self::Line { start, end } => {
                translate(start);
                translate(end);
            }
            Self::Polygon { points } => points.iter_mut().for_each(translate),
            Self::Arc {
                start,
                through,
                end,
            } => {
                translate(start);
                translate(through);
                translate(end);
            }
            Self::Callout {
                tip,
                elbow,
                box_corner,
            } => {
                translate(tip);
                translate(elbow);
                translate(box_corner);
            }
        }
    }

    pub fn points(&self) -> Vec<Point> {
        match self {
            Self::Rectangle { first, opposite } => vec![*first, *opposite],
            Self::Line { start, end } => vec![*start, *end],
            Self::Polygon { points } => points.clone(),
            Self::Arc {
                start,
                through,
                end,
            } => vec![*start, *through, *end],
            Self::Callout {
                tip,
                elbow,
                box_corner,
            } => vec![*tip, *elbow, *box_corner],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentationShape {
    pub id: u64,
    pub geometry: DocumentationShapeGeometry,
    pub layer: DocumentationShapeLayer,
}

impl DocumentationShape {
    pub fn new(
        id: u64,
        geometry: DocumentationShapeGeometry,
    ) -> Result<Self, DocumentationShapeError> {
        geometry.validate()?;
        Ok(Self {
            id,
            geometry,
            layer: DocumentationShapeLayer::DrawingDocumentation,
        })
    }

    pub const fn kind(&self) -> DocumentationShapeKind {
        self.geometry.kind()
    }

    pub fn validate(&self) -> Result<(), DocumentationShapeError> {
        if self.layer != DocumentationShapeLayer::DrawingDocumentation {
            return Err(DocumentationShapeError::InvalidLayer);
        }
        self.geometry.validate()
    }

    pub fn translate(&mut self, delta: Point) {
        self.geometry.translate(delta);
    }

    pub fn bounds(&self) -> (Point, Point) {
        if let DocumentationShapeGeometry::Arc {
            start,
            through,
            end,
        } = &self.geometry
            && let Some((cx, cy, radius, start_angle, sweep)) =
                arc_parameters(*start, *through, *end)
        {
            let mut samples = vec![
                (f64::from(start.x), f64::from(start.y)),
                (f64::from(end.x), f64::from(end.y)),
            ];
            for angle in [
                0.0,
                std::f64::consts::FRAC_PI_2,
                std::f64::consts::PI,
                std::f64::consts::PI + std::f64::consts::FRAC_PI_2,
            ] {
                if angle_on_sweep(angle, start_angle, sweep) {
                    samples.push((cx + radius * angle.cos(), cy + radius * angle.sin()));
                }
            }
            let min_x = samples
                .iter()
                .map(|point| point.0)
                .fold(f64::INFINITY, f64::min)
                .floor() as i32;
            let min_y = samples
                .iter()
                .map(|point| point.1)
                .fold(f64::INFINITY, f64::min)
                .floor() as i32;
            let max_x = samples
                .iter()
                .map(|point| point.0)
                .fold(f64::NEG_INFINITY, f64::max)
                .ceil() as i32;
            let max_y = samples
                .iter()
                .map(|point| point.1)
                .fold(f64::NEG_INFINITY, f64::max)
                .ceil() as i32;
            return (Point::new(min_x, min_y), Point::new(max_x, max_y));
        }
        let points = self.geometry.points();
        let first = points.first().copied().unwrap_or_else(Point::origin);
        points
            .iter()
            .skip(1)
            .fold((first, first), |(mut min, mut max), point| {
                min.x = min.x.min(point.x);
                min.y = min.y.min(point.y);
                max.x = max.x.max(point.x);
                max.y = max.y.max(point.y);
                (min, max)
            })
    }
}

/// Clamp one translation for an entire documentation-shape set so every
/// object preserves its relative position at the signed coordinate limits.
pub(crate) fn clamped_documentation_shape_translation<'a>(
    shapes: impl IntoIterator<Item = &'a DocumentationShape>,
    delta: Point,
) -> Point {
    let mut points = shapes.into_iter().flat_map(|shape| shape.geometry.points());
    let Some(first) = points.next() else {
        return delta;
    };
    let (min_x, max_x, min_y, max_y) = points.fold(
        (first.x, first.x, first.y, first.y),
        |(min_x, max_x, min_y, max_y), point| {
            (
                min_x.min(point.x),
                max_x.max(point.x),
                min_y.min(point.y),
                max_y.max(point.y),
            )
        },
    );
    Point::new(
        delta.x.clamp(
            i32::MIN.saturating_sub(min_x),
            i32::MAX.saturating_sub(max_x),
        ),
        delta.y.clamp(
            i32::MIN.saturating_sub(min_y),
            i32::MAX.saturating_sub(max_y),
        ),
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentationShapePlacementAuthority {
    pub design_execution_epoch: u64,
    pub active_schematic_epoch: u64,
    pub view_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingDocumentationShapePlacement {
    pub kind: DocumentationShapeKind,
    pub layer: DocumentationShapeLayer,
    pub topology_version: u64,
    pub expected_shapes: Vec<DocumentationShape>,
    pub document_authority: Option<DocumentationShapePlacementAuthority>,
}

impl PendingDocumentationShapePlacement {
    pub fn new(
        kind: DocumentationShapeKind,
        topology_version: u64,
        expected_shapes: &[DocumentationShape],
    ) -> Self {
        Self {
            kind,
            layer: DocumentationShapeLayer::DrawingDocumentation,
            topology_version,
            expected_shapes: expected_shapes.to_vec(),
            document_authority: None,
        }
    }

    pub fn with_document_authority(
        mut self,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        view_path: String,
    ) -> Self {
        self.document_authority = Some(DocumentationShapePlacementAuthority {
            design_execution_epoch,
            active_schematic_epoch,
            view_path,
        });
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DocumentationShapeDrawing {
    pub points: Vec<Point>,
    /// Grid-resolved placement cursor used when the focused canvas is driven
    /// without a pointing device.
    pub keyboard_cursor: Option<Point>,
    pub keyboard_active: bool,
}

impl DocumentationShapeDrawing {
    pub fn clear(&mut self) {
        self.points.clear();
        self.keyboard_cursor = None;
        self.keyboard_active = false;
    }

    pub fn add_point(
        &mut self,
        kind: DocumentationShapeKind,
        point: Point,
    ) -> Result<bool, DocumentationShapeError> {
        if self.points.last() == Some(&point) {
            return Ok(false);
        }
        if kind == DocumentationShapeKind::Polygon
            && self.points.len() >= MAX_DOCUMENTATION_POLYGON_POINTS
        {
            return Err(DocumentationShapeError::TooManyPoints);
        }
        self.points.push(point);
        Ok(kind.commits_automatically() && self.points.len() == kind.minimum_points())
    }

    pub fn geometry(
        &self,
        kind: DocumentationShapeKind,
    ) -> Result<DocumentationShapeGeometry, DocumentationShapeError> {
        geometry_from_points(kind, &self.points)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentationShapeError {
    InsufficientPoints,
    TooManyPoints,
    DegenerateGeometry,
    InvalidLayer,
    ReadOnly,
    StaleDocument,
}

impl std::fmt::Display for DocumentationShapeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientPoints => formatter.write_str("The shape needs more points."),
            Self::TooManyPoints => write!(
                formatter,
                "Polygons may contain at most {MAX_DOCUMENTATION_POLYGON_POINTS} points."
            ),
            Self::DegenerateGeometry => formatter
                .write_str("The shape has zero length, area, radius, sweep, or box extent."),
            Self::InvalidLayer => {
                formatter.write_str("Documentation shapes must remain on drawing / documentation.")
            }
            Self::ReadOnly => formatter.write_str("The active schematic is read-only."),
            Self::StaleDocument => formatter.write_str(
                "The schematic changed after the documentation-shape contract was prepared.",
            ),
        }
    }
}

impl std::error::Error for DocumentationShapeError {}

impl SchematicState {
    pub fn validate_pending_documentation_shape(
        &self,
        pending: &PendingDocumentationShapePlacement,
    ) -> Result<(), DocumentationShapeError> {
        if self.read_only {
            return Err(DocumentationShapeError::ReadOnly);
        }
        if pending.topology_version != self.topology_version()
            || pending.expected_shapes != self.documentation_shapes
        {
            return Err(DocumentationShapeError::StaleDocument);
        }
        if pending.layer != DocumentationShapeLayer::DrawingDocumentation {
            return Err(DocumentationShapeError::InvalidLayer);
        }
        Ok(())
    }

    pub fn commit_documentation_shape(
        &mut self,
        pending: PendingDocumentationShapePlacement,
        geometry: DocumentationShapeGeometry,
    ) -> Result<u64, DocumentationShapeError> {
        self.validate_pending_documentation_shape(&pending)?;
        if geometry.kind() != pending.kind {
            return Err(DocumentationShapeError::DegenerateGeometry);
        }
        let id = self.next_id();
        let shape = DocumentationShape::new(id, geometry)?;
        let changed = self.with_undo("draw documentation shape", |schematic| {
            schematic.documentation_shapes.push(shape);
            schematic.selection.clear();
            schematic.selection.select_documentation_shape(id);
            schematic.is_dirty = true;
        });
        if changed {
            Ok(id)
        } else {
            Err(DocumentationShapeError::ReadOnly)
        }
    }
}

pub fn geometry_from_points(
    kind: DocumentationShapeKind,
    points: &[Point],
) -> Result<DocumentationShapeGeometry, DocumentationShapeError> {
    let geometry = match kind {
        DocumentationShapeKind::Rectangle if points.len() == 2 => {
            DocumentationShapeGeometry::Rectangle {
                first: points[0],
                opposite: points[1],
            }
        }
        DocumentationShapeKind::Line if points.len() == 2 => DocumentationShapeGeometry::Line {
            start: points[0],
            end: points[1],
        },
        DocumentationShapeKind::Polygon if points.len() >= 3 => {
            DocumentationShapeGeometry::Polygon {
                points: points.to_vec(),
            }
        }
        DocumentationShapeKind::Arc if points.len() == 3 => DocumentationShapeGeometry::Arc {
            start: points[0],
            through: points[1],
            end: points[2],
        },
        DocumentationShapeKind::Callout if points.len() == 3 => {
            DocumentationShapeGeometry::Callout {
                tip: points[0],
                elbow: points[1],
                box_corner: points[2],
            }
        }
        _ => return Err(DocumentationShapeError::InsufficientPoints),
    };
    geometry.validate()?;
    Ok(geometry)
}

fn signed_double_area(points: &[Point]) -> i128 {
    points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .take(points.len())
        .map(|(left, right)| {
            i128::from(left.x) * i128::from(right.y) - i128::from(right.x) * i128::from(left.y)
        })
        .sum()
}

/// Center, radius, start angle, and signed sweep for a valid three-point arc.
pub fn arc_parameters(
    start: Point,
    through: Point,
    end: Point,
) -> Option<(f64, f64, f64, f64, f64)> {
    // Decide degeneracy in the exact integer domain. A floating-point
    // determinant can round a valid large-coordinate arc to zero (or admit a
    // collinear one) before the geometric solve even begins.
    let exact_orientation = (i128::from(through.x) - i128::from(start.x))
        * (i128::from(end.y) - i128::from(start.y))
        - (i128::from(through.y) - i128::from(start.y)) * (i128::from(end.x) - i128::from(start.x));
    if start == through || through == end || start == end || exact_orientation == 0 {
        return None;
    }
    let (ax, ay) = (f64::from(start.x), f64::from(start.y));
    let (bx, by) = (f64::from(through.x), f64::from(through.y));
    let (cx, cy) = (f64::from(end.x), f64::from(end.y));
    let determinant = 2.0 * (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by));
    if !determinant.is_finite() || determinant == 0.0 {
        return None;
    }
    let aa = ax * ax + ay * ay;
    let bb = bx * bx + by * by;
    let cc = cx * cx + cy * cy;
    let center_x = (aa * (by - cy) + bb * (cy - ay) + cc * (ay - by)) / determinant;
    let center_y = (aa * (cx - bx) + bb * (ax - cx) + cc * (bx - ax)) / determinant;
    let radius = ((ax - center_x).powi(2) + (ay - center_y).powi(2)).sqrt();
    if !radius.is_finite() || radius <= f64::EPSILON {
        return None;
    }
    let start_angle = (ay - center_y).atan2(ax - center_x);
    let end_angle = (cy - center_y).atan2(cx - center_x);
    let positive = positive_angle(end_angle - start_angle);
    // Exact orientation selects the only sweep that passes through the middle
    // control point, including major arcs where angle comparisons are most
    // vulnerable to rounding at the branch cut.
    let sweep = if exact_orientation > 0 {
        positive
    } else {
        positive - std::f64::consts::TAU
    };
    (sweep.abs() > f64::EPSILON).then_some((center_x, center_y, radius, start_angle, sweep))
}

fn positive_angle(angle: f64) -> f64 {
    angle.rem_euclid(std::f64::consts::TAU)
}

fn angle_on_sweep(angle: f64, start: f64, sweep: f64) -> bool {
    if sweep >= 0.0 {
        positive_angle(angle - start) <= sweep + 1e-10
    } else {
        positive_angle(start - angle) <= -sweep + 1e-10
    }
}

fn polygon_self_intersects(points: &[Point]) -> bool {
    let edge_count = points.len();
    for left in 0..edge_count {
        let left_next = (left + 1) % edge_count;
        for right in (left + 1)..edge_count {
            let right_next = (right + 1) % edge_count;
            if left == right || left_next == right || right_next == left {
                continue;
            }
            if segments_intersect(
                points[left],
                points[left_next],
                points[right],
                points[right_next],
            ) {
                return true;
            }
        }
    }
    false
}

fn segments_intersect(a: Point, b: Point, c: Point, d: Point) -> bool {
    let orient = |p: Point, q: Point, r: Point| {
        (i128::from(q.x) - i128::from(p.x)) * (i128::from(r.y) - i128::from(p.y))
            - (i128::from(q.y) - i128::from(p.y)) * (i128::from(r.x) - i128::from(p.x))
    };
    let (o1, o2, o3, o4) = (
        orient(a, b, c),
        orient(a, b, d),
        orient(c, d, a),
        orient(c, d, b),
    );
    if o1 == 0 && point_on_segment(c, a, b)
        || o2 == 0 && point_on_segment(d, a, b)
        || o3 == 0 && point_on_segment(a, c, d)
        || o4 == 0 && point_on_segment(b, c, d)
    {
        return true;
    }
    (o1 > 0) != (o2 > 0) && (o3 > 0) != (o4 > 0)
}

fn point_on_segment(point: Point, start: Point, end: Point) -> bool {
    point.x >= start.x.min(end.x)
        && point.x <= start.x.max(end.x)
        && point.y >= start.y.min(end.y)
        && point.y <= start.y.max(end.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_mockup_kind_has_valid_exact_geometry() {
        let cases = [
            (
                DocumentationShapeKind::Rectangle,
                vec![Point::new(0, 0), Point::new(20, 10)],
            ),
            (
                DocumentationShapeKind::Line,
                vec![Point::new(0, 0), Point::new(20, 10)],
            ),
            (
                DocumentationShapeKind::Polygon,
                vec![Point::new(0, 0), Point::new(20, 0), Point::new(10, 10)],
            ),
            (
                DocumentationShapeKind::Arc,
                vec![Point::new(0, 0), Point::new(10, 0), Point::new(0, 10)],
            ),
            (
                DocumentationShapeKind::Callout,
                vec![Point::new(0, 0), Point::new(10, 10), Point::new(30, 20)],
            ),
        ];
        for (kind, points) in cases {
            assert_eq!(geometry_from_points(kind, &points).unwrap().kind(), kind);
        }
    }

    #[test]
    fn degenerate_geometry_fails_closed() {
        assert_eq!(
            geometry_from_points(
                DocumentationShapeKind::Rectangle,
                &[Point::origin(), Point::new(10, 0)]
            ),
            Err(DocumentationShapeError::DegenerateGeometry)
        );
        assert_eq!(
            geometry_from_points(
                DocumentationShapeKind::Polygon,
                &[Point::origin(), Point::new(10, 0), Point::new(20, 0)]
            ),
            Err(DocumentationShapeError::DegenerateGeometry)
        );
    }

    #[test]
    fn three_point_arc_uses_exact_orientation_for_major_sweeps_and_extreme_collinearity() {
        let (_, _, radius, _, sweep) =
            arc_parameters(Point::new(10, 0), Point::new(0, -10), Point::new(0, 10))
                .expect("three distinct non-collinear points define one arc");
        assert!((radius - 10.0).abs() < 1e-9);
        assert!(
            sweep < -std::f64::consts::PI,
            "the selected arc must pass through the lower control point"
        );
        assert!(
            arc_parameters(
                Point::new(i32::MIN, i32::MIN),
                Point::origin(),
                Point::new(i32::MAX, i32::MAX),
            )
            .is_none()
        );
    }

    #[test]
    fn placement_is_non_electrical_and_one_undo_record() {
        let mut schematic = SchematicState::default();
        let topology = schematic.topology_version();
        let pending = PendingDocumentationShapePlacement::new(
            DocumentationShapeKind::Rectangle,
            topology,
            &schematic.documentation_shapes,
        );
        let id = schematic
            .commit_documentation_shape(
                pending,
                DocumentationShapeGeometry::Rectangle {
                    first: Point::origin(),
                    opposite: Point::new(20, 10),
                },
            )
            .unwrap();
        assert_eq!(schematic.documentation_shapes[0].id, id);
        assert!(schematic.components.is_empty());
        assert!(schematic.wires.is_empty());
        assert_eq!(schematic.topology_version(), topology);
        assert!(schematic.undo());
        assert!(schematic.documentation_shapes.is_empty());
    }

    #[test]
    fn armed_contract_rejects_documentation_shape_drift() {
        let mut schematic = SchematicState::default();
        let pending = PendingDocumentationShapePlacement::new(
            DocumentationShapeKind::Line,
            schematic.topology_version(),
            &schematic.documentation_shapes,
        );
        schematic.documentation_shapes.push(
            DocumentationShape::new(
                44,
                DocumentationShapeGeometry::Line {
                    start: Point::origin(),
                    end: Point::new(1, 1),
                },
            )
            .unwrap(),
        );
        assert_eq!(
            schematic.validate_pending_documentation_shape(&pending),
            Err(DocumentationShapeError::StaleDocument)
        );
    }
}
