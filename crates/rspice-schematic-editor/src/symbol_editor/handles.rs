//! The handles a selected symbol shape is edited by in place, and what
//! dragging each one does to the shape.

use super::interaction::radius_through;
use super::session::bearing;
use rspice_design::symbol::{SymbolCurve, SymbolShape, rectangle_corners};
use rspice_design_model::Point;

/// One handle on a shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolHandle {
    /// A line's or a polygon's point, by its index.
    Vertex(usize),
    /// The middle of the edge from point `n` to the next: dragging it puts a
    /// new point there.
    EdgeMiddle(usize),
    /// A rectangle's corner, clockwise from its top-left as
    /// [`rectangle_corners`] lists them.
    Corner(usize),
    /// The middle of a rectangle's side: top, right, bottom, left.
    Side(usize),
    /// The point on a circle's east side that sets its radius.
    Radius,
    /// Where an arc starts, as it was drawn.
    ArcStart,
    /// Where an arc ends, as it was drawn.
    ArcEnd,
    /// The point a text, a dot, an arrow or a drawing's curve stands at.
    Anchor,
}

/// A handle on one of the document's shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymbolHandleRef {
    pub shape: usize,
    pub handle: SymbolHandle,
}

/// A handle being dragged, or armed by Stretch to follow the pointer: the
/// handle it has become (an edge's middle becomes the point it put there)
/// and the point its drag holds still, such as a rectangle's opposite
/// corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymbolHandleDrag {
    pub shape: usize,
    pub handle: SymbolHandle,
    pub fixed: Point,
}

/// The fewest points a line keeps, and a polygon.
pub const LINE_POINTS: usize = 2;
pub const POLYGON_CORNERS: usize = 3;

/// Every handle `shape` has, with where it stands in symbol coordinates. An
/// edge's middle can fall between whole units, so positions are exact.
pub fn shape_handles(shape: &SymbolShape) -> Vec<(SymbolHandle, [f64; 2])> {
    let at = |point: Point| [f64::from(point.x), f64::from(point.y)];
    let middle = |a: Point, b: Point| {
        [
            (f64::from(a.x) + f64::from(b.x)) / 2.0,
            (f64::from(a.y) + f64::from(b.y)) / 2.0,
        ]
    };
    match shape {
        SymbolShape::Polyline { points, closed, .. } => {
            let mut handles: Vec<(SymbolHandle, [f64; 2])> = points
                .iter()
                .enumerate()
                .map(|(index, point)| (SymbolHandle::Vertex(index), at(*point)))
                .collect();
            let edges = if *closed && points.len() > 2 {
                points.len()
            } else {
                points.len().saturating_sub(1)
            };
            handles.extend((0..edges).map(|index| {
                let next = points[(index + 1) % points.len()];
                (SymbolHandle::EdgeMiddle(index), middle(points[index], next))
            }));
            handles
        }
        SymbolShape::Rectangle { min, max, .. } => {
            let corners = rectangle_corners(*min, *max);
            let mut handles: Vec<(SymbolHandle, [f64; 2])> = corners
                .iter()
                .enumerate()
                .map(|(index, corner)| (SymbolHandle::Corner(index), at(*corner)))
                .collect();
            handles.extend((0..4).map(|index| {
                (
                    SymbolHandle::Side(index),
                    middle(corners[index], corners[(index + 1) % 4]),
                )
            }));
            handles
        }
        SymbolShape::Circle { center, radius, .. } => {
            vec![(SymbolHandle::Radius, at(*center + Point::new(*radius, 0)))]
        }
        SymbolShape::Arc {
            center,
            radius,
            start_degrees,
            sweep_degrees,
            ..
        } => {
            let circle = SymbolCurve::of_arc(*center, *radius, 0, 360);
            vec![
                (
                    SymbolHandle::ArcStart,
                    circle.point_at(f64::from(*start_degrees)),
                ),
                (
                    SymbolHandle::ArcEnd,
                    circle.point_at(f64::from(*start_degrees) + f64::from(*sweep_degrees)),
                ),
            ]
        }
        SymbolShape::Text { anchor, .. } | SymbolShape::Label { anchor, .. } => {
            vec![(SymbolHandle::Anchor, at(*anchor))]
        }
        SymbolShape::Dot { center, .. } => vec![(SymbolHandle::Anchor, at(*center))],
        SymbolShape::Arrow { tip, .. } => vec![(SymbolHandle::Anchor, at(*tip))],
        SymbolShape::Curve { curve, .. } => vec![(SymbolHandle::Anchor, curve.center)],
    }
}

/// Take hold of `handle` on `shape`. An edge's middle puts a new point there
/// first, which the drag then carries; a rectangle's corner or side holds
/// the opposite one still.
pub fn grab(shape: &mut SymbolShape, index: usize, handle: SymbolHandle) -> SymbolHandleDrag {
    let mut drag = SymbolHandleDrag {
        shape: index,
        handle,
        fixed: Point::origin(),
    };
    match (shape, handle) {
        (SymbolShape::Polyline { points, .. }, SymbolHandle::EdgeMiddle(edge))
            if edge < points.len() =>
        {
            let next = points[(edge + 1) % points.len()];
            let start = points[edge];
            let middle = Point::new(
                (start.x + next.x).div_euclid(2),
                (start.y + next.y).div_euclid(2),
            );
            points.insert(edge + 1, middle);
            drag.handle = SymbolHandle::Vertex(edge + 1);
        }
        (SymbolShape::Rectangle { min, max, .. }, SymbolHandle::Corner(corner)) => {
            drag.fixed = rectangle_corners(*min, *max)[(corner + 2) % 4];
        }
        (SymbolShape::Rectangle { min, max, .. }, SymbolHandle::Side(side)) => {
            drag.fixed = rectangle_corners(*min, *max)[(side + 2) % 4];
        }
        _ => {}
    }
    drag
}

/// `shape` with the handle `drag` holds brought to `point`; `None` where
/// that would leave it with nothing to draw (a rectangle with no width or
/// height, a circle with no radius, an arc that turns no way) or put a
/// point on the one beside it.
pub fn moved(shape: &SymbolShape, drag: &SymbolHandleDrag, point: Point) -> Option<SymbolShape> {
    let mut moved = shape.clone();
    match (&mut moved, drag.handle) {
        (SymbolShape::Polyline { points, closed, .. }, SymbolHandle::Vertex(index)) => {
            let count = points.len();
            if index >= count {
                return None;
            }
            let previous = (index > 0 || *closed).then(|| points[(index + count - 1) % count]);
            let next = (index + 1 < count || *closed).then(|| points[(index + 1) % count]);
            if previous == Some(point) || next == Some(point) {
                return None;
            }
            points[index] = point;
        }
        (SymbolShape::Rectangle { .. }, SymbolHandle::Corner(_)) => {
            if point.x == drag.fixed.x || point.y == drag.fixed.y {
                return None;
            }
            moved = SymbolShape::rectangle(drag.fixed, point);
        }
        (SymbolShape::Rectangle { min, max, .. }, SymbolHandle::Side(side)) => {
            // Top and bottom move in y only, right and left in x only: the
            // corner carried is the one across from the corner held.
            let corner = match side {
                0 => Point::new(min.x, point.y),
                1 => Point::new(point.x, min.y),
                2 => Point::new(max.x, point.y),
                _ => Point::new(point.x, max.y),
            };
            let held = drag.fixed;
            let across = if side % 2 == 0 {
                corner.y == held.y
            } else {
                corner.x == held.x
            };
            if across {
                return None;
            }
            moved = SymbolShape::rectangle(held, corner);
        }
        (SymbolShape::Circle { center, radius, .. }, SymbolHandle::Radius) => {
            *radius = radius_through(*center, point);
            if *radius < 1 {
                return None;
            }
        }
        (
            SymbolShape::Arc {
                center,
                radius,
                start_degrees,
                sweep_degrees,
                ..
            },
            SymbolHandle::ArcStart | SymbolHandle::ArcEnd,
        ) => {
            if point == *center {
                return None;
            }
            let turned = bearing(*center, point).round() as i32;
            let clockwise = *sweep_degrees > 0;
            let (from, to) = if drag.handle == SymbolHandle::ArcStart {
                (turned, *start_degrees + *sweep_degrees)
            } else {
                (*start_degrees, turned)
            };
            // The arc keeps the way it turns, and the end not dragged stays
            // at its bearing.
            let sweep = if clockwise {
                (to - from).rem_euclid(360)
            } else {
                -(from - to).rem_euclid(360)
            };
            if sweep == 0 {
                return None;
            }
            *radius = radius_through(*center, point).max(1);
            *start_degrees = from.rem_euclid(360);
            *sweep_degrees = sweep;
        }
        (_, SymbolHandle::Anchor) => {
            let anchor = super::arrange::shape_anchor(shape);
            moved.translate(point - anchor);
        }
        _ => return None,
    }
    // A rectangle resized from its corner is built anew; it keeps the
    // stroke, dash and fill it was drawn with.
    if let (Some(style), Some(kept)) = (shape.style(), moved.style_mut()) {
        *kept = style;
    }
    (moved != *shape).then_some(moved)
}

/// What a drag of `handle` on `shape` is called as an undo step: "Move
/// vertex", "Add vertex", "Resize rectangle".
pub fn step_name(shape: &SymbolShape, handle: SymbolHandle) -> String {
    match handle {
        SymbolHandle::Vertex(_) => "Move vertex".to_owned(),
        SymbolHandle::EdgeMiddle(_) => "Add vertex".to_owned(),
        SymbolHandle::Corner(_) | SymbolHandle::Side(_) => "Resize rectangle".to_owned(),
        SymbolHandle::Radius => "Resize circle".to_owned(),
        SymbolHandle::ArcStart => "Move arc start".to_owned(),
        SymbolHandle::ArcEnd => "Move arc end".to_owned(),
        SymbolHandle::Anchor => {
            format!("Move {}", super::session::shape_kind(shape).to_lowercase())
        }
    }
}

/// Remove point `vertex` from the line or polygon `shape`, unless that would
/// leave fewer than it needs; why not, when it would.
pub fn remove_vertex(shape: &mut SymbolShape, vertex: usize) -> Result<(), &'static str> {
    let SymbolShape::Polyline { points, closed, .. } = shape else {
        return Err("Only a line or a polygon has points to delete.");
    };
    let fewest = if *closed {
        POLYGON_CORNERS
    } else {
        LINE_POINTS
    };
    if points.len() <= fewest {
        return Err(if *closed {
            "A polygon keeps at least three corners; delete the polygon instead."
        } else {
            "A line keeps at least two points; delete the line instead."
        });
    }
    if vertex >= points.len() {
        return Err("That point is no longer on the shape.");
    }
    points.remove(vertex);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drag(shape: &mut SymbolShape, handle: SymbolHandle) -> SymbolHandleDrag {
        grab(shape, 0, handle)
    }

    /// A rectangle's corner drag keeps the opposite corner where it is and
    /// the shape a rectangle, whichever way the corner crosses it.
    #[test]
    fn a_corner_drag_holds_the_opposite_corner() {
        let mut rectangle = SymbolShape::rectangle(Point::new(-20, -10), Point::new(20, 10));
        let held = drag(&mut rectangle, SymbolHandle::Corner(2));
        assert_eq!(held.fixed, Point::new(-20, -10));
        assert_eq!(
            moved(&rectangle, &held, Point::new(40, 30)),
            Some(SymbolShape::rectangle(
                Point::new(-20, -10),
                Point::new(40, 30)
            ))
        );
        assert_eq!(
            moved(&rectangle, &held, Point::new(-40, -30)),
            Some(SymbolShape::rectangle(
                Point::new(-40, -30),
                Point::new(-20, -10)
            ))
        );
        assert_eq!(moved(&rectangle, &held, Point::new(-20, 30)), None);
    }

    /// A resized rectangle keeps its stroke, dash and fill, from a corner
    /// and from a side alike.
    #[test]
    fn a_resize_keeps_the_rectangles_style() {
        use rspice_design::symbol::{SymbolDash, SymbolFill, SymbolStroke, SymbolStyle};
        let style = SymbolStyle {
            stroke: SymbolStroke::Heavy,
            dash: SymbolDash::Dashed,
            fill: SymbolFill::Solid,
        };
        let mut rectangle = SymbolShape::rectangle(Point::new(-20, -10), Point::new(20, 10));
        *rectangle.style_mut().unwrap() = style;
        for handle in [SymbolHandle::Corner(2), SymbolHandle::Side(1)] {
            let held = drag(&mut rectangle, handle);
            let resized = moved(&rectangle, &held, Point::new(40, 30)).unwrap();
            assert_eq!(resized.style(), Some(style), "{handle:?}");
        }
    }

    /// A side drag moves that side alone.
    #[test]
    fn a_side_drag_moves_one_side() {
        let mut rectangle = SymbolShape::rectangle(Point::new(-20, -10), Point::new(20, 10));
        let top = drag(&mut rectangle, SymbolHandle::Side(0));
        assert_eq!(
            moved(&rectangle, &top, Point::new(70, -30)),
            Some(SymbolShape::rectangle(
                Point::new(-20, -30),
                Point::new(20, 10)
            ))
        );
        let right = drag(&mut rectangle, SymbolHandle::Side(1));
        assert_eq!(
            moved(&rectangle, &right, Point::new(50, 99)),
            Some(SymbolShape::rectangle(
                Point::new(-20, -10),
                Point::new(50, 10)
            ))
        );
        assert_eq!(moved(&rectangle, &right, Point::new(-20, 0)), None);
    }

    /// An edge's middle puts a point there and the drag carries it.
    #[test]
    fn an_edge_middle_drag_inserts_a_point() {
        let mut polygon = SymbolShape::Polyline {
            points: vec![Point::new(0, 0), Point::new(20, 0), Point::new(20, 20)],
            closed: true,
            style: Default::default(),
        };
        let held = drag(&mut polygon, SymbolHandle::EdgeMiddle(2));
        assert_eq!(held.handle, SymbolHandle::Vertex(3));
        assert_eq!(
            polygon,
            SymbolShape::Polyline {
                points: vec![
                    Point::new(0, 0),
                    Point::new(20, 0),
                    Point::new(20, 20),
                    Point::new(10, 10),
                ],
                closed: true,
                style: Default::default(),
            }
        );
        assert_eq!(
            moved(&polygon, &held, Point::new(0, 20)),
            Some(SymbolShape::Polyline {
                points: vec![
                    Point::new(0, 0),
                    Point::new(20, 0),
                    Point::new(20, 20),
                    Point::new(0, 20),
                ],
                closed: true,
                style: Default::default(),
            })
        );
        // Onto the corner beside it is no point at all.
        assert_eq!(moved(&polygon, &held, Point::new(0, 0)), None);
    }

    /// An arc's end follows the pointer round its centre; its start stays,
    /// and it keeps turning the way it was drawn.
    #[test]
    fn an_arc_end_drag_moves_the_end_and_keeps_the_start() {
        let arc = SymbolShape::Arc {
            center: Point::origin(),
            radius: 10,
            start_degrees: 0,
            sweep_degrees: 90,
            style: Default::default(),
        };
        let held = SymbolHandleDrag {
            shape: 0,
            handle: SymbolHandle::ArcEnd,
            fixed: Point::origin(),
        };
        assert_eq!(
            moved(&arc, &held, Point::new(-20, 0)),
            Some(SymbolShape::Arc {
                center: Point::origin(),
                radius: 20,
                start_degrees: 0,
                sweep_degrees: 180,
                style: Default::default(),
            })
        );
        let start = SymbolHandleDrag {
            handle: SymbolHandle::ArcStart,
            ..held
        };
        assert_eq!(
            moved(&arc, &start, Point::new(0, -10)),
            Some(SymbolShape::Arc {
                center: Point::origin(),
                radius: 10,
                start_degrees: 270,
                sweep_degrees: 180,
                style: Default::default(),
            })
        );
        assert_eq!(moved(&arc, &held, Point::new(10, 0)), None);
    }

    /// A line keeps two points and a polygon three.
    #[test]
    fn a_vertex_is_removed_down_to_the_fewest_the_shape_needs() {
        let mut line = SymbolShape::Polyline {
            points: vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)],
            closed: false,
            style: Default::default(),
        };
        assert_eq!(remove_vertex(&mut line, 1), Ok(()));
        assert!(remove_vertex(&mut line, 0).is_err());
        let mut polygon = SymbolShape::Polyline {
            points: vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)],
            closed: true,
            style: Default::default(),
        };
        assert!(remove_vertex(&mut polygon, 0).is_err());
    }
}
