//! Baking a symbol for drawing.
//!
//! Flattens a parsed symbol into the primitives the painter draws, with the
//! instance transform already applied, so the canvas does not re-resolve
//! geometry every frame.

use egui::{Painter, Pos2, Shape, Stroke, Vec2};

use rspice_design::symbol_artwork::Symbol;

// =============================================================================
// Baked symbols — flattened once, painted with one multiply-add per point
// =============================================================================

/// A symbol flattened for fast painting: beziers subdivided, view scaling,
/// mirroring and rotation applied, and corner-fill dots precomputed.
/// Coordinates are in schematic units relative to the component center, so
/// painting is `center + point * zoom` per vertex — no trig, no bezier
/// evaluation, no per-path Vec rebuilds.
#[derive(Debug, Clone, Default)]
pub struct BakedSymbol {
    polylines: Vec<Vec<Vec2>>,
    /// Closed filled shapes (arrowheads, junction bars) painted in the
    /// stroke color.
    filled: Vec<Vec<Vec2>>,
    corner_dots: Vec<Vec2>,
}

/// Flatten a symbol into caller-selected schematic dimensions without
/// mutating the immutable embedded asset.
pub fn bake_symbol_with_dimensions(
    symbol: &Symbol,
    target_width: f32,
    target_height: f32,
    rotation_degrees: i32,
    mirror_h: bool,
    mirror_v: bool,
) -> BakedSymbol {
    let mut baked = BakedSymbol::default();
    for run in symbol.placed_runs(
        target_width,
        target_height,
        rotation_degrees,
        mirror_h,
        mirror_v,
    ) {
        let points: Vec<Vec2> = run.points.iter().map(|&(x, y)| Vec2::new(x, y)).collect();
        if run.filled {
            baked.filled.push(points);
        } else {
            collect_corner_dots(&points, &mut baked.corner_dots);
            baked.polylines.push(points);
        }
    }
    baked
}

/// Record corner-fill dot positions at sharp direction changes — these fill
/// the joint gaps that separate line segments would otherwise show.
fn collect_corner_dots(points: &[Vec2], dots: &mut Vec<Vec2>) {
    for i in 1..points.len().saturating_sub(1) {
        let prev = points[i - 1];
        let curr = points[i];
        let next = points[i + 1];
        let d1 = curr - prev;
        let d2 = next - curr;
        let len1 = d1.length();
        let len2 = d2.length();
        if len1 > 0.001 && len2 > 0.001 {
            let dot = (d1 / len1).dot(d2 / len2);
            if dot.abs() < 0.87 {
                dots.push(curr);
            }
        }
    }
}

/// The terminals a symbol's leads end on, for painting those leads as the
/// ends of the wires that reach them.
///
/// A wire is snapped to the pixel grid by its own stroke, and a symbol's
/// stroke can be a different width with a different parity, so a lead painted
/// as artwork lands half a pixel off the wire it meets and smears. A lead here
/// is an axis-aligned stroke end on a terminal; it takes the wire's width and
/// the terminal's own column or row, so egui snaps it exactly as it snaps the
/// wire. Curves, diagonals and everything else stay artwork.
#[derive(Debug, Clone, Copy)]
pub struct PinLeads<'a> {
    terminals: &'a [Pos2],
    wire_width: f32,
    reach: f32,
}

impl PinLeads<'_> {
    /// No terminals: every stroke paints as artwork, for a symbol no wire
    /// reaches, a preview or a thumbnail.
    pub const NONE: PinLeads<'static> = PinLeads {
        terminals: &[],
        wire_width: 0.0,
        reach: 0.0,
    };
}

impl<'a> PinLeads<'a> {
    /// `terminals` in screen points, for wires `wire_width` points wide at
    /// `zoom` points per world unit.
    pub fn new(terminals: &'a [Pos2], wire_width: f32, zoom: f32) -> Self {
        Self {
            terminals,
            wire_width,
            // Half a world unit: art lies on the grid, or close by rounding.
            reach: 0.5 * zoom,
        }
    }

    /// The lead `end`-`inner` is, from its terminal to `inner` on the
    /// terminal's own column or row, if it is one.
    fn lead(&self, end: Pos2, inner: Pos2) -> Option<[Pos2; 2]> {
        const STRAIGHT: f32 = 0.01;
        let terminal = *self
            .terminals
            .iter()
            .find(|terminal| (**terminal - end).length() <= self.reach)?;
        let (across, along) = ((end.x - inner.x).abs(), (end.y - inner.y).abs());
        if across <= STRAIGHT && along > STRAIGHT {
            Some([terminal, Pos2::new(terminal.x, inner.y)])
        } else if along <= STRAIGHT && across > STRAIGHT {
            Some([terminal, Pos2::new(inner.x, terminal.y)])
        } else {
            None
        }
    }

    /// Paints whichever ends of the open stroke `points` are leads, and
    /// answers the part of it that is still to paint as artwork.
    pub fn paint_leads<'p>(
        &self,
        painter: &Painter,
        points: &'p [Pos2],
        stroke: Stroke,
    ) -> &'p [Pos2] {
        let wire = Stroke::new(self.wire_width, stroke.color);
        let mut rest = points;
        if let [end, inner, ..] = rest
            && let Some(lead) = self.lead(*end, *inner)
        {
            painter.line_segment(lead, wire);
            rest = &rest[1..];
        }
        if let [.., inner, end] = rest
            && let Some(lead) = self.lead(*end, *inner)
        {
            painter.line_segment(lead, wire);
            rest = &rest[..rest.len() - 1];
        }
        rest
    }
}

/// Paint a baked symbol: one multiply-add per vertex, one shape per polyline,
/// its leads as the ends of the wires that reach them.
pub fn draw_baked(
    painter: &Painter,
    baked: &BakedSymbol,
    center: Pos2,
    scale: f32,
    stroke: Stroke,
    leads: &PinLeads<'_>,
) {
    let radius = stroke.width / 2.0;
    for dot in &baked.corner_dots {
        painter.circle_filled(center + *dot * scale, radius, stroke.color);
    }
    for line in &baked.polylines {
        if let [a, b] = line[..] {
            let points = [center + a * scale, center + b * scale];
            if leads.paint_leads(painter, &points, stroke).len() == 2 {
                painter.line_segment(points, stroke);
            }
        } else {
            let points: Vec<Pos2> = line.iter().map(|p| center + *p * scale).collect();
            let rest = leads.paint_leads(painter, &points, stroke);
            if rest.len() >= 2 {
                painter.add(Shape::line(rest.to_vec(), stroke));
            }
        }
    }
    for polygon in &baked.filled {
        let points: Vec<Pos2> = polygon.iter().map(|p| center + *p * scale).collect();
        painter.add(Shape::convex_polygon(
            points,
            stroke.color,
            Stroke::new(stroke.width * 0.5, stroke.color),
        ));
    }
}

// =============================================================================
// Direct (unbaked) rendering — used by one-off previews
// =============================================================================

/// Render a symbol to an egui painter with position, scale, rotation, mirroring, and stroke
#[expect(
    clippy::too_many_arguments,
    reason = "independent placement and stroke inputs"
)]
pub fn draw_symbol(
    painter: &Painter,
    symbol: &Symbol,
    center: Pos2,
    scale: f32,
    rotation_degrees: i32,
    mirror_h: bool,
    mirror_v: bool,
    stroke: Stroke,
) {
    draw_symbol_with_dimensions(
        painter,
        symbol,
        symbol.target_width,
        symbol.target_height,
        center,
        scale,
        rotation_degrees,
        mirror_h,
        mirror_v,
        stroke,
    );
}

/// Draw immutable asset geometry at placement-specific dimensions.
#[expect(
    clippy::too_many_arguments,
    reason = "independent asset dimensions, placement and stroke inputs"
)]
pub fn draw_symbol_with_dimensions(
    painter: &Painter,
    symbol: &Symbol,
    target_width: f32,
    target_height: f32,
    center: Pos2,
    scale: f32,
    rotation_degrees: i32,
    mirror_h: bool,
    mirror_v: bool,
    stroke: Stroke,
) {
    for run in symbol.placed_runs(
        target_width,
        target_height,
        rotation_degrees,
        mirror_h,
        mirror_v,
    ) {
        let points: Vec<Pos2> = run
            .points
            .iter()
            .map(|&(x, y)| center + Vec2::new(x, y) * scale)
            .collect();
        if run.filled {
            painter.add(Shape::convex_polygon(
                points,
                stroke.color,
                Stroke::new(stroke.width * 0.5, stroke.color),
            ));
        } else {
            draw_path_segment(painter, &points, stroke);
        }
    }
}

/// Draw a sequence of connected line segments using individual line calls
fn draw_path_segment(painter: &Painter, points: &[Pos2], stroke: Stroke) {
    if points.len() < 2 {
        return;
    }

    // Draw circles FIRST (behind lines) at vertices where direction changes
    // This fills gaps at corners without visible circle edges (lines cover them)
    let radius = stroke.width / 2.0;

    // Check middle vertices for direction changes
    for i in 1..points.len().saturating_sub(1) {
        let prev = points[i - 1];
        let curr = points[i];
        let next = points[i + 1];

        // Calculate direction vectors
        let d1 = (curr.x - prev.x, curr.y - prev.y);
        let d2 = (next.x - curr.x, next.y - curr.y);

        // Normalize and check if directions are different (not collinear)
        let len1 = (d1.0 * d1.0 + d1.1 * d1.1).sqrt();
        let len2 = (d2.0 * d2.0 + d2.1 * d2.1).sqrt();

        if len1 > 0.001 && len2 > 0.001 {
            let n1 = (d1.0 / len1, d1.1 / len1);
            let n2 = (d2.0 / len2, d2.1 / len2);

            // Dot product - if close to 1, segments are collinear
            let dot = n1.0 * n2.0 + n1.1 * n2.1;

            // Only draw at sharp corners (angle > ~30 degrees)
            if dot.abs() < 0.87 {
                painter.circle_filled(curr, radius, stroke.color);
            }
        }
    }

    // Draw line segments ON TOP of circles
    for pair in points.windows(2) {
        painter.line_segment([pair[0], pair[1]], stroke);
    }
}
