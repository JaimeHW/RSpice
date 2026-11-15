//! A symbol shape's style on screen: the weight and pattern of its outline,
//! and the ink inside a closed one.
//!
//! The symbol editor's canvas and every placed instance draw a style through
//! here, so the shape being edited and the shape on a sheet cannot disagree.

use egui::{Color32, Painter, Pos2, Shape, Stroke, epaint::Mesh};
use rspice_design::symbol::{SymbolDash, SymbolFill, SymbolStyle};

/// The stroke an outline in `style` is drawn with, from the weight the
/// renderer draws a body at.
pub(crate) fn styled_stroke(stroke: Stroke, style: SymbolStyle) -> Stroke {
    Stroke::new(stroke.width * style.stroke.scale(), stroke.color)
}

/// Paints the inside of the closed outline `points` in `color` when `style`
/// fills it. The outline itself is drawn afterwards, over the fill's edge.
pub(crate) fn paint_fill(painter: &Painter, points: &[Pos2], style: SymbolStyle, color: Color32) {
    if style.fill != SymbolFill::Solid {
        return;
    }
    let mut outline = points.to_vec();
    outline.dedup();
    if outline.len() > 2 && outline.first() == outline.last() {
        outline.pop();
    }
    let triangles = triangulate(&outline);
    if triangles.is_empty() {
        return;
    }
    let mut mesh = Mesh::default();
    for point in &outline {
        mesh.colored_vertex(*point, color);
    }
    for [a, b, c] in triangles {
        mesh.add_triangle(a as u32, b as u32, c as u32);
    }
    painter.add(Shape::mesh(mesh));
}

/// The pieces of `points` drawn in `style`'s pattern, `scale` screen points
/// to the symbol unit, so a pattern keeps its proportion at every zoom.
/// `None` for a whole line, which each renderer draws as it always has.
pub(crate) fn broken_outline(
    points: &[Pos2],
    closed: bool,
    stroke: Stroke,
    style: SymbolStyle,
    scale: f32,
) -> Option<Vec<Shape>> {
    let (drawn, skipped) = style.dash.pattern()?;
    let mut path = points.to_vec();
    if closed && let Some(first) = points.first() {
        path.push(*first);
    }
    Some(match style.dash {
        // Round dots, one stroke wide, a whole step apart.
        SymbolDash::Dotted => Shape::dotted_line(
            &path,
            stroke.color,
            ((drawn + skipped) * scale).max(1.0),
            (stroke.width / 2.0).max(0.5),
        ),
        SymbolDash::Dashed | SymbolDash::Solid => Shape::dashed_line(
            &path,
            stroke,
            (drawn * scale).max(1.0),
            (skipped * scale).max(1.0),
        ),
    })
}

/// Whether `pos` lies inside the closed outline `points`, by the even-odd
/// rule a fill paints with.
pub(crate) fn inside_outline(points: &[Pos2], pos: Pos2) -> bool {
    let mut inside = false;
    for (index, a) in points.iter().enumerate() {
        let b = points[(index + 1) % points.len()];
        if (a.y > pos.y) != (b.y > pos.y) && pos.x < a.x + (pos.y - a.y) * (b.x - a.x) / (b.y - a.y)
        {
            inside = !inside;
        }
    }
    inside
}

/// Triangles covering the polygon `points`, as indices into it.
///
/// Ears are clipped one at a time, which fills a concave outline — an
/// arrowhead's notch, an L-shaped body — where a fan would paint across the
/// notch. An outline that crosses itself may have no ear left to clip; what
/// remains of it is then fanned, so it is never left empty.
fn triangulate(points: &[Pos2]) -> Vec<[usize; 3]> {
    let count = points.len();
    if count < 3 {
        return Vec::new();
    }
    let twice_area: f32 = (0..count)
        .map(|index| {
            let (a, b) = (points[index], points[(index + 1) % count]);
            a.x * b.y - b.x * a.y
        })
        .sum();
    if twice_area == 0.0 {
        return Vec::new();
    }
    let winding = twice_area.signum();
    let mut remaining = (0..count).collect::<Vec<_>>();
    let mut triangles = Vec::with_capacity(count - 2);
    while remaining.len() > 3 {
        let corners = remaining.len();
        let around = |at: usize| {
            (
                remaining[(at + corners - 1) % corners],
                remaining[at],
                remaining[(at + 1) % corners],
            )
        };
        let ear = (0..corners).find(|&at| {
            let (a, b, c) = around(at);
            let (pa, pb, pc) = (points[a], points[b], points[c]);
            turn(pa, pb, pc) * winding > 0.0
                && remaining.iter().all(|&other| {
                    other == a
                        || other == b
                        || other == c
                        || !inside(points[other], pa, pb, pc, winding)
                })
        });
        let Some(at) = ear else {
            triangles.extend(
                (1..corners - 1).map(|next| [remaining[0], remaining[next], remaining[next + 1]]),
            );
            return triangles;
        };
        let (a, b, c) = around(at);
        triangles.push([a, b, c]);
        remaining.remove(at);
    }
    triangles.push([remaining[0], remaining[1], remaining[2]]);
    triangles
}

/// Twice the signed area of the triangle `a b c`: which way the path turns
/// at `b`.
fn turn(a: Pos2, b: Pos2, c: Pos2) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

/// Whether `point` lies in the triangle `a b c` wound the way `winding` says,
/// its edges included.
fn inside(point: Pos2, a: Pos2, b: Pos2, c: Pos2, winding: f32) -> bool {
    turn(a, b, point) * winding >= 0.0
        && turn(b, c, point) * winding >= 0.0
        && turn(c, a, point) * winding >= 0.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::pos2;

    fn area(points: &[Pos2], triangles: &[[usize; 3]]) -> f32 {
        triangles
            .iter()
            .map(|[a, b, c]| turn(points[*a], points[*b], points[*c]).abs() / 2.0)
            .sum()
    }

    /// A concave outline is covered exactly, the notch left bare: the
    /// triangles add up to the outline's own area, which a fan from the
    /// first corner of an L overshoots.
    #[test]
    fn a_concave_outline_fills_without_painting_its_notch() {
        let l_shape = [
            pos2(0.0, 0.0),
            pos2(40.0, 0.0),
            pos2(40.0, 10.0),
            pos2(10.0, 10.0),
            pos2(10.0, 40.0),
            pos2(0.0, 40.0),
        ];
        let triangles = triangulate(&l_shape);
        assert_eq!(triangles.len(), 4);
        assert!((area(&l_shape, &triangles) - 700.0).abs() < 1e-3);
        // Nothing lands in the notch's corner.
        let notch = pos2(30.0, 30.0);
        assert!(triangles.iter().all(|[a, b, c]| {
            !inside(notch, l_shape[*a], l_shape[*b], l_shape[*c], 1.0)
                && !inside(notch, l_shape[*a], l_shape[*b], l_shape[*c], -1.0)
        }));
    }

    #[test]
    fn a_point_in_a_concave_outlines_notch_is_outside_it() {
        let l_shape = [
            pos2(0.0, 0.0),
            pos2(40.0, 0.0),
            pos2(40.0, 10.0),
            pos2(10.0, 10.0),
            pos2(10.0, 40.0),
            pos2(0.0, 40.0),
        ];
        assert!(inside_outline(&l_shape, pos2(5.0, 30.0)));
        assert!(inside_outline(&l_shape, pos2(30.0, 5.0)));
        assert!(!inside_outline(&l_shape, pos2(30.0, 30.0)));
        assert!(!inside_outline(&l_shape, pos2(-1.0, 5.0)));
    }

    #[test]
    fn an_outline_wound_either_way_is_filled() {
        let triangle = [pos2(0.0, 0.0), pos2(10.0, 0.0), pos2(0.0, 10.0)];
        let reversed = [triangle[2], triangle[1], triangle[0]];
        assert_eq!(triangulate(&triangle).len(), 1);
        assert_eq!(triangulate(&reversed).len(), 1);
        assert!(triangulate(&[pos2(0.0, 0.0), pos2(5.0, 0.0), pos2(10.0, 0.0)]).is_empty());
    }
}
