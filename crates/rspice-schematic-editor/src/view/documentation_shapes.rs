//! Canvas presentation and exact interaction geometry for documentation shapes.

use egui::{Color32, Painter, Pos2, Rect, Stroke};

use rspice_design::schematic::documentation_shape::{
    DocumentationShape, DocumentationShapeGeometry, SymbolDash, SymbolStyle, arc_parameters,
};
use rspice_design_model::Point;
use rspice_ui_kit::palette::Palette;

use super::drawing::draw_hover_outline;
use super::lod::{self, Emphasis, StrokeRole};
use super::symbol_style::{broken_outline, inside_color, paint_fill};
use super::viewport::Viewport;

const HIT_TOLERANCE_POINTS: f32 = 6.0;
const RECTANGLE_CORNER_POINTS: f32 = 6.0;

pub fn world_bounds(shape: &DocumentationShape) -> (Point, Point) {
    shape.bounds()
}

fn shape_ink(selected: bool, palette: &Palette) -> Color32 {
    if selected {
        palette.canvas_select
    } else {
        palette.text_faint
    }
}

/// Paints the inside of a closed shape its style fills, beneath every wire,
/// part and line of lettering, so a shaded box sets off what it encloses
/// rather than hiding it.
pub fn draw_documentation_shape_fill(
    painter: &Painter,
    viewport: &Viewport,
    shape: &DocumentationShape,
    selected: bool,
    palette: &Palette,
) {
    if !shape.kind().is_closed() {
        return;
    }
    let Some(fill) = inside_color(
        shape.style.fill,
        shape_ink(selected, palette),
        palette.canvas_bg,
    ) else {
        return;
    };
    let rounded = |first: Point, opposite: Point| {
        let rect = Rect::from_two_pos(
            viewport.schematic_to_screen(first),
            viewport.schematic_to_screen(opposite),
        );
        painter.rect_filled(rect, corner_radius(viewport), fill);
    };
    match &shape.geometry {
        DocumentationShapeGeometry::Rectangle { first, opposite } => rounded(*first, *opposite),
        DocumentationShapeGeometry::Callout {
            elbow, box_corner, ..
        } => rounded(*elbow, *box_corner),
        DocumentationShapeGeometry::Polygon { points } => {
            let screen = points
                .iter()
                .map(|point| viewport.schematic_to_screen(*point))
                .collect::<Vec<_>>();
            paint_fill(painter, &screen, Some(fill));
        }
        DocumentationShapeGeometry::Line { .. } | DocumentationShapeGeometry::Arc { .. } => {}
    }
}

fn corner_radius(viewport: &Viewport) -> f32 {
    (RECTANGLE_CORNER_POINTS * viewport.zoom).clamp(3.0, 9.0)
}

pub fn draw_documentation_shape(
    painter: &Painter,
    viewport: &Viewport,
    shape: &DocumentationShape,
    selected: bool,
    hovered: bool,
    palette: &Palette,
) {
    let color = shape_ink(selected, palette);
    if hovered && !selected {
        let (min, max) = shape.bounds();
        draw_hover_outline(
            painter,
            Rect::from_two_pos(
                viewport.schematic_to_screen(min),
                viewport.schematic_to_screen(max),
            ),
        );
    }
    let stroke = Stroke::new(
        lod::stroke_pt(
            StrokeRole::Documentation,
            viewport.zoom,
            painter.pixels_per_point(),
            Emphasis::raised_if(selected),
        ) * shape.style.stroke.scale(),
        color,
    );
    draw_styled_geometry(painter, viewport, &shape.geometry, stroke, shape.style);
    if selected {
        for point in shape.geometry.points() {
            let screen = viewport.schematic_to_screen(point);
            painter.circle_filled(screen, 3.0, palette.canvas_bg);
            painter.circle_stroke(screen, 3.0, Stroke::new(1.0, palette.canvas_select));
        }
    }
}

/// The outline of `geometry` in `style`'s pattern: whole as
/// [`draw_geometry`] draws it, or broken into dashes or dots that keep their
/// proportion against the drawing at every zoom.
fn draw_styled_geometry(
    painter: &Painter,
    viewport: &Viewport,
    geometry: &DocumentationShapeGeometry,
    stroke: Stroke,
    style: SymbolStyle,
) {
    if style.dash == SymbolDash::Solid {
        draw_geometry(painter, viewport, geometry, stroke);
        return;
    }
    let screen = |point: &Point| viewport.schematic_to_screen(*point);
    let paths: Vec<(Vec<Pos2>, bool)> = match geometry {
        DocumentationShapeGeometry::Rectangle { first, opposite } => vec![(
            rounded_rect_path(
                Rect::from_two_pos(screen(first), screen(opposite)),
                corner_radius(viewport),
            ),
            true,
        )],
        DocumentationShapeGeometry::Line { start, end } => {
            vec![(vec![screen(start), screen(end)], false)]
        }
        DocumentationShapeGeometry::Polygon { points } => {
            vec![(points.iter().map(screen).collect(), true)]
        }
        DocumentationShapeGeometry::Arc {
            start,
            through,
            end,
        } => vec![(arc_screen_points(viewport, *start, *through, *end), false)],
        DocumentationShapeGeometry::Callout {
            tip,
            elbow,
            box_corner,
        } => vec![
            (vec![screen(tip), screen(elbow)], false),
            (
                rounded_rect_path(
                    Rect::from_two_pos(screen(elbow), screen(box_corner)),
                    corner_radius(viewport),
                ),
                true,
            ),
        ],
    };
    for (path, closed) in paths {
        if let Some(pieces) = broken_outline(&path, closed, stroke, style, viewport.zoom) {
            painter.extend(pieces);
        }
    }
}

/// The outline of `rect` with corners rounded to `radius`, as a closed path
/// whose last point leads back to its first.
fn rounded_rect_path(rect: Rect, radius: f32) -> Vec<Pos2> {
    const CORNER_STEPS: usize = 4;
    let radius = radius.min(rect.width() * 0.5).min(rect.height() * 0.5);
    let corners = [
        (rect.right_top() + egui::vec2(-radius, radius), -90.0_f32),
        (rect.right_bottom() + egui::vec2(-radius, -radius), 0.0),
        (rect.left_bottom() + egui::vec2(radius, -radius), 90.0),
        (rect.left_top() + egui::vec2(radius, radius), 180.0),
    ];
    corners
        .into_iter()
        .flat_map(|(center, from)| {
            (0..=CORNER_STEPS).map(move |step| {
                let angle = (from + 90.0 * step as f32 / CORNER_STEPS as f32).to_radians();
                center + radius * egui::vec2(angle.cos(), angle.sin())
            })
        })
        .collect()
}

pub fn draw_geometry(
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
                corner_radius(viewport),
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
                corner_radius(viewport),
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

pub fn documentation_shape_at(
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

pub fn shape_intersects_rect(
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
        .any(|(start, end)| {
            super::geometry::segment_intersects_rect(start, end, min_x, min_y, max_x, max_y)
        })
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

pub fn preview_stroke(valid: bool, palette: &Palette) -> Stroke {
    Stroke::new(
        1.4,
        if valid {
            palette.canvas_select.gamma_multiply(PREVIEW_ALPHA)
        } else {
            palette.err
        },
    )
}

pub fn preview_anchor_color(valid: bool, palette: &Palette) -> Color32 {
    if valid {
        palette.canvas_select.gamma_multiply(PREVIEW_ALPHA)
    } else {
        palette.err
    }
}

/// Route, placement and shape previews are the selection colour, slightly
/// held back so a committed selection still reads above them.
const PREVIEW_ALPHA: f32 = 0.85;

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::schematic::documentation_shape::{
        DocumentationShape, DocumentationShapeGeometry,
    };

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
            documentation_shape_at(
                &viewport,
                std::slice::from_ref(&shape),
                Pos2::new(50.0, 11.0)
            ),
            Some(3)
        );
        assert_eq!(
            documentation_shape_at(&viewport, &[shape], Pos2::new(50.0, 35.0)),
            None
        );
    }
}
