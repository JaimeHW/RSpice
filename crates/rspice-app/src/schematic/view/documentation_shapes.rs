//! Canvas presentation and exact interaction geometry for documentation shapes.

use egui::{Color32, Painter, Pos2, Rect, Stroke};

use crate::state::{DocumentationShape, DocumentationShapeGeometry, Point, arc_parameters};

use super::viewport::Viewport;

const HIT_TOLERANCE_POINTS: f32 = 6.0;
const RECTANGLE_CORNER_POINTS: f32 = 6.0;

pub(super) fn world_bounds(shape: &DocumentationShape) -> (Point, Point) {
    shape.bounds()
}

pub(super) fn draw_documentation_shape(
    painter: &Painter,
    viewport: &Viewport,
    shape: &DocumentationShape,
    selected: bool,
    hovered: bool,
) {
    let palette = crate::ui::tokens::active_palette();
    let color = if selected {
        palette.accent
    } else if hovered {
        palette.text
    } else {
        palette.text_faint
    };
    let stroke = Stroke::new(if selected { 1.7 } else { 1.2 }, color);
    draw_geometry(painter, viewport, &shape.geometry, stroke);
    if selected {
        for point in shape.geometry.points() {
            let screen = viewport.schematic_to_screen(point);
            painter.circle_filled(screen, 3.0, palette.canvas_bg);
            painter.circle_stroke(screen, 3.0, Stroke::new(1.0, palette.accent));
        }
    }
}

pub(super) fn draw_geometry(
    painter: &Painter,
    viewport: &Viewport,
    geometry: &DocumentationShapeGeometry,
    stroke: Stroke,
) {
    match geometry {
        DocumentationShapeGeometry::Rectangle { first, opposite } => {
            let rect = Rect::from_two_pos(
                viewport.schematic_to_screen(*first),
                viewport.schematic_to_screen(*opposite),
            );
            painter.rect_stroke(
                rect,
                (RECTANGLE_CORNER_POINTS * viewport.zoom).clamp(3.0, 9.0),
                stroke,
                egui::StrokeKind::Middle,
            );
        }
        DocumentationShapeGeometry::Line { start, end } => {
            painter.line_segment(
                [
                    viewport.schematic_to_screen(*start),
                    viewport.schematic_to_screen(*end),
                ],
                stroke,
            );
        }
        DocumentationShapeGeometry::Polygon { points } => {
            let mut screen: Vec<Pos2> = points
                .iter()
                .map(|point| viewport.schematic_to_screen(*point))
                .collect();
            if let Some(first) = screen.first().copied() {
                screen.push(first);
            }
            painter.add(egui::Shape::line(screen, stroke));
        }
        DocumentationShapeGeometry::Arc {
            start,
            through,
            end,
        } => {
            painter.add(egui::Shape::line(
                arc_screen_points(viewport, *start, *through, *end),
                stroke,
            ));
        }
        DocumentationShapeGeometry::Callout {
            tip,
            elbow,
            box_corner,
        } => {
            painter.line_segment(
                [
                    viewport.schematic_to_screen(*tip),
                    viewport.schematic_to_screen(*elbow),
                ],
                stroke,
            );
            let rect = Rect::from_two_pos(
                viewport.schematic_to_screen(*elbow),
                viewport.schematic_to_screen(*box_corner),
            );
            painter.rect_stroke(
                rect,
                (RECTANGLE_CORNER_POINTS * viewport.zoom).clamp(3.0, 9.0),
                stroke,
                egui::StrokeKind::Middle,
            );
        }
    }
}

fn arc_screen_points(viewport: &Viewport, start: Point, through: Point, end: Point) -> Vec<Pos2> {
    let Some((cx, cy, radius, start_angle, sweep)) = arc_parameters(start, through, end) else {
        return vec![
            viewport.schematic_to_screen(start),
            viewport.schematic_to_screen(end),
        ];
    };
    let segments = arc_segment_count(radius, sweep, viewport.zoom);
    (0..=segments)
        .map(|index| {
            let angle = start_angle + sweep * index as f64 / segments as f64;
            Pos2::new(
                viewport.bounds.min.x
                    + viewport.offset.x
                    + (cx + radius * angle.cos()) as f32 * viewport.zoom,
                viewport.bounds.min.y
                    + viewport.offset.y
                    + (cy + radius * angle.sin()) as f32 * viewport.zoom,
            )
        })
        .collect()
}

fn arc_world_points(start: Point, through: Point, end: Point, zoom: f32) -> Vec<Point> {
    let Some((cx, cy, radius, start_angle, sweep)) = arc_parameters(start, through, end) else {
        return vec![start, end];
    };
    let segments = arc_segment_count(radius, sweep, zoom);
    (0..=segments)
        .map(|index| {
            let angle = start_angle + sweep * index as f64 / segments as f64;
            Point::new(
                (cx + radius * angle.cos()).round() as i32,
                (cy + radius * angle.sin()).round() as i32,
            )
        })
        .collect()
}

fn arc_segment_count(radius: f64, sweep: f64, zoom: f32) -> usize {
    ((sweep.abs() * (radius * f64::from(zoom)).sqrt() * 0.8).ceil() as usize).clamp(12, 512)
}

pub(super) fn documentation_shape_at(
    viewport: &Viewport,
    shapes: &[DocumentationShape],
    pointer: Pos2,
) -> Option<u64> {
    shapes
        .iter()
        .rev()
        .find(|shape| hit_geometry(viewport, &shape.geometry, pointer, HIT_TOLERANCE_POINTS))
        .map(|shape| shape.id)
}

fn hit_geometry(
    viewport: &Viewport,
    geometry: &DocumentationShapeGeometry,
    pointer: Pos2,
    tolerance: f32,
) -> bool {
    let segment_hit = |a: Pos2, b: Pos2| distance_to_segment(pointer, a, b) <= tolerance;
    match geometry {
        DocumentationShapeGeometry::Rectangle { first, opposite } => {
            let rect = Rect::from_two_pos(
                viewport.schematic_to_screen(*first),
                viewport.schematic_to_screen(*opposite),
            );
            let [a, b, c, d] = [
                rect.left_top(),
                rect.right_top(),
                rect.right_bottom(),
                rect.left_bottom(),
            ];
            segment_hit(a, b) || segment_hit(b, c) || segment_hit(c, d) || segment_hit(d, a)
        }
        DocumentationShapeGeometry::Line { start, end } => segment_hit(
            viewport.schematic_to_screen(*start),
            viewport.schematic_to_screen(*end),
        ),
        DocumentationShapeGeometry::Polygon { points } => points
            .iter()
            .zip(points.iter().cycle().skip(1))
            .take(points.len())
            .any(|(a, b)| {
                segment_hit(
                    viewport.schematic_to_screen(*a),
                    viewport.schematic_to_screen(*b),
                )
            }),
        DocumentationShapeGeometry::Arc {
            start,
            through,
            end,
        } => arc_screen_points(viewport, *start, *through, *end)
            .windows(2)
            .any(|points| segment_hit(points[0], points[1])),
        DocumentationShapeGeometry::Callout {
            tip,
            elbow,
            box_corner,
        } => {
            let rect = Rect::from_two_pos(
                viewport.schematic_to_screen(*elbow),
                viewport.schematic_to_screen(*box_corner),
            );
            let [a, b, c, d] = [
                rect.left_top(),
                rect.right_top(),
                rect.right_bottom(),
                rect.left_bottom(),
            ];
            segment_hit(
                viewport.schematic_to_screen(*tip),
                viewport.schematic_to_screen(*elbow),
            ) || segment_hit(a, b)
                || segment_hit(b, c)
                || segment_hit(c, d)
                || segment_hit(d, a)
        }
    }
}

fn distance_to_segment(point: Pos2, start: Pos2, end: Pos2) -> f32 {
    let segment = end - start;
    let length_squared = segment.length_sq();
    if length_squared <= f32::EPSILON {
        return point.distance(start);
    }
    let t = ((point - start).dot(segment) / length_squared).clamp(0.0, 1.0);
    point.distance(start + segment * t)
}

pub(super) fn shape_intersects_rect(
    shape: &DocumentationShape,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
    enclosed_only: bool,
) -> bool {
    let (min, max) = world_bounds(shape);
    if enclosed_only {
        return min.x >= min_x && min.y >= min_y && max.x <= max_x && max.y <= max_y;
    }
    if max.x < min_x || min.x > max_x || max.y < min_y || min.y > max_y {
        return false;
    }
    shape_segments(&shape.geometry)
        .into_iter()
        .any(|(start, end)| super::segment_intersects_rect(start, end, min_x, min_y, max_x, max_y))
}

fn shape_segments(geometry: &DocumentationShapeGeometry) -> Vec<(Point, Point)> {
    match geometry {
        DocumentationShapeGeometry::Rectangle { first, opposite } => {
            let a = *first;
            let c = *opposite;
            let b = Point::new(c.x, a.y);
            let d = Point::new(a.x, c.y);
            vec![(a, b), (b, c), (c, d), (d, a)]
        }
        DocumentationShapeGeometry::Line { start, end } => vec![(*start, *end)],
        DocumentationShapeGeometry::Polygon { points } => points
            .iter()
            .zip(points.iter().cycle().skip(1))
            .take(points.len())
            .map(|(a, b)| (*a, *b))
            .collect(),
        DocumentationShapeGeometry::Arc {
            start,
            through,
            end,
        } => arc_world_points(*start, *through, *end, 1.0)
            .windows(2)
            .map(|pair| (pair[0], pair[1]))
            .collect(),
        DocumentationShapeGeometry::Callout {
            tip,
            elbow,
            box_corner,
        } => {
            let a = *elbow;
            let c = *box_corner;
            let b = Point::new(c.x, a.y);
            let d = Point::new(a.x, c.y);
            vec![(*tip, *elbow), (a, b), (b, c), (c, d), (d, a)]
        }
    }
}

pub(super) fn preview_stroke(valid: bool) -> Stroke {
    let palette = crate::ui::tokens::active_palette();
    Stroke::new(1.4, if valid { palette.accent } else { palette.err })
}

pub(super) fn preview_anchor_color(valid: bool) -> Color32 {
    let palette = crate::ui::tokens::active_palette();
    if valid { palette.accent } else { palette.err }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{DocumentationShape, DocumentationShapeGeometry};

    #[test]
    fn three_point_arc_bounds_include_cardinal_extrema() {
        let shape = DocumentationShape::new(
            1,
            DocumentationShapeGeometry::Arc {
                start: Point::new(10, 0),
                through: Point::new(0, 10),
                end: Point::new(-10, 0),
            },
        )
        .unwrap();
        assert_eq!(
            world_bounds(&shape),
            (Point::new(-10, 0), Point::new(10, 10))
        );
    }

    #[test]
    fn rectangle_is_hit_on_outline_not_in_empty_interior() {
        let shape = DocumentationShape::new(
            3,
            DocumentationShapeGeometry::Rectangle {
                first: Point::new(10, 10),
                opposite: Point::new(90, 60),
            },
        )
        .unwrap();
        let viewport = Viewport {
            offset: Pos2::ZERO,
            zoom: 1.0,
            bounds: Rect::from_min_size(Pos2::ZERO, egui::vec2(200.0, 200.0)),
        };
        assert_eq!(
            documentation_shape_at(&viewport, &[shape.clone()], Pos2::new(50.0, 11.0)),
            Some(3)
        );
        assert_eq!(
            documentation_shape_at(&viewport, &[shape], Pos2::new(50.0, 35.0)),
            None
        );
    }
}
