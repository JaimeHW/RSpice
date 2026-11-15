//! Sealed-scene painting.
//!
//! Draws a contract [`Scene`] into an egui pane with the same geometry the
//! page's static SVG committed to: identical dash arrays, arc flattening
//! against a fixed chord tolerance, and role colors resolved through the
//! shared [`crate::theme`] palettes. Schematic panes add pan and zoom —
//! the reason a sheet hydrates at all.

use egui::epaint::{Mesh, TextShape};
use egui::{Align2, Color32, Pos2, Rect, Sense, Shape, Stroke as EguiStroke, Vec2};
use rspice_publication_contract::{
    PathSegment, Point, Primitive, Scene, Stroke, StrokePattern, TextAnchor,
};

use crate::theme::Palette;

/// Dash array in stroke-width multiples, identical to the SVG renderer's
/// `stroke-dasharray` geometry.
#[must_use]
pub fn dash_pattern(pattern: StrokePattern) -> &'static [f32] {
    match pattern {
        StrokePattern::Solid => &[],
        StrokePattern::Dashed => &[4.0, 2.0],
        StrokePattern::Dotted => &[1.0, 3.0],
        StrokePattern::DashDot => &[4.0, 2.0, 1.0, 2.0],
    }
}

/// The on and off lengths `stroke` is drawn in on screen, `scale` pixels to
/// the micrometre: the exact pattern its author drew where it carries one,
/// else the pattern's own. Exact lengths are for round caps; these runs end
/// square, so each dash takes the caps' reach of one width and each gap
/// gives it up. Empty for a whole line.
#[must_use]
pub fn dash_array(stroke: &Stroke, scale: f32) -> Vec<f32> {
    let width = stroke.width_um as f32 * scale;
    match stroke.dash_um {
        Some([on, off]) => vec![
            (on as f32 * scale + width).max(0.5),
            (off as f32 * scale - width).max(0.5),
        ],
        None => dash_pattern(stroke.pattern)
            .iter()
            .map(|multiple| (multiple * width).max(0.5))
            .collect(),
    }
}

/// Split a polyline into the "on" runs of a repeating dash array.
///
/// Lengths are in the same unit as the points. The array alternates
/// on/off starting with "on", exactly like `stroke-dasharray`.
#[must_use]
pub fn dash_polyline(points: &[Pos2], array: &[f32]) -> Vec<Vec<Pos2>> {
    let total: f32 = array.iter().sum();
    if points.len() < 2 || array.is_empty() || total <= 0.0 {
        return vec![points.to_vec()];
    }
    let mut runs = Vec::new();
    let mut run: Vec<Pos2> = Vec::new();
    let mut slot = 0usize;
    let mut remaining = array[0];
    let mut cursor = points[0];
    if slot.is_multiple_of(2) {
        run.push(cursor);
    }
    for &target in &points[1..] {
        let mut segment = target - cursor;
        let mut length = segment.length();
        while length > remaining {
            let boundary = cursor + segment * (remaining / length);
            if slot.is_multiple_of(2) {
                run.push(boundary);
                runs.push(core::mem::take(&mut run));
            } else {
                run.push(boundary);
            }
            run.clear();
            cursor = boundary;
            segment = target - cursor;
            length = segment.length();
            slot = (slot + 1) % array.len();
            remaining = array[slot];
            if slot.is_multiple_of(2) {
                run.push(cursor);
            }
        }
        remaining -= length;
        cursor = target;
        if slot.is_multiple_of(2) {
            run.push(cursor);
        }
    }
    if run.len() >= 2 {
        runs.push(run);
    }
    runs
}

/// Flatten one arc into a polyline against a fixed chord tolerance in
/// pixels, bounded to keep hostile radii from exploding the point count.
fn flatten_arc(
    into: &mut Vec<Pos2>,
    center: Pos2,
    radius: f32,
    start_millideg: i32,
    sweep_millideg: i32,
) {
    let sweep_radians = (sweep_millideg as f32 / 1000.0).to_radians();
    let chord_tolerance = 0.25f32;
    let steps = if radius <= chord_tolerance {
        1
    } else {
        let per_full_turn =
            (core::f32::consts::TAU / (2.0 * (1.0 - chord_tolerance / radius).acos())).ceil();
        ((sweep_radians.abs() / core::f32::consts::TAU) * per_full_turn).ceil() as u32
    }
    .clamp(1, 512);
    for step in 0..=steps {
        let angle = (start_millideg as f32 / 1000.0).to_radians()
            + sweep_radians * step as f32 / steps as f32;
        into.push(Pos2::new(
            center.x + radius * angle.cos(),
            center.y - radius * angle.sin(),
        ));
    }
}

/// The inside of the closed outline `points` painted in `color`, as the
/// page's static figure fills it. A convex outline takes egui's own smoothed
/// polygon; a concave one (an arrowhead's notch, an L-shaped body) is cut
/// into ears, where a fan from its first corner would paint across the
/// notch and lay a tint twice where its triangles overlap.
fn fill_shape(points: &[Pos2], color: Color32) -> Option<Shape> {
    let mut outline = points.to_vec();
    outline.dedup();
    if outline.len() > 2 && outline.first() == outline.last() {
        outline.pop();
    }
    if outline.len() < 3 {
        return None;
    }
    let count = outline.len();
    let turns = (0..count).map(|at| {
        turn(
            outline[at],
            outline[(at + 1) % count],
            outline[(at + 2) % count],
        )
    });
    let (left, right) = turns.fold((false, false), |(left, right), turn| {
        (left || turn > 0.0, right || turn < 0.0)
    });
    if !(left && right) {
        return Some(Shape::convex_polygon(outline, color, EguiStroke::NONE));
    }
    let triangles = triangulate(&outline);
    if triangles.is_empty() {
        return None;
    }
    let mut mesh = Mesh::default();
    for point in &outline {
        mesh.colored_vertex(*point, color);
    }
    for [a, b, c] in triangles {
        mesh.add_triangle(a as u32, b as u32, c as u32);
    }
    Some(Shape::mesh(mesh))
}

/// Triangles covering the polygon `points`, as indices into it, clipped one
/// ear at a time. An outline that crosses itself may have no ear left to
/// clip; what remains of it is then fanned, so it is never left empty.
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
                        || !inside_triangle(points[other], pa, pb, pc, winding)
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
fn inside_triangle(point: Pos2, a: Pos2, b: Pos2, c: Pos2, winding: f32) -> bool {
    turn(a, b, point) * winding >= 0.0
        && turn(b, c, point) * winding >= 0.0
        && turn(c, a, point) * winding >= 0.0
}

/// Pan/zoom state for an interactive scene pane. Stored per figure.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SceneCamera {
    /// Extra zoom on top of fit-to-pane. 1.0 = fitted.
    pub zoom: f32,
    /// Pan offset in pane points.
    pub offset: Vec2,
}

impl Default for SceneCamera {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            offset: Vec2::ZERO,
        }
    }
}

const MAX_ZOOM: f32 = 64.0;

impl SceneCamera {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Zoom around the pane center. Pointer-wheel zoom uses its own pivot in
    /// [`scene_pane`], while these controls remain keyboard reachable.
    pub fn zoom_by(&mut self, factor: f32) {
        if !factor.is_finite() || factor <= 0.0 {
            return;
        }
        let previous = self.zoom;
        self.zoom = (self.zoom * factor).clamp(1.0, MAX_ZOOM);
        self.offset *= self.zoom / previous;
        if self.zoom <= 1.0 {
            self.offset = Vec2::ZERO;
        }
    }
}

/// Keyboard-accessible camera controls shown above an interactive sheet.
pub fn scene_toolbar(ui: &mut egui::Ui, camera: &mut SceneCamera) {
    ui.horizontal_wrapped(|ui| {
        if ui
            .button("Fit")
            .on_hover_text("Fit the sheet in the viewer")
            .clicked()
        {
            camera.reset();
        }
        if ui.button("−").on_hover_text("Zoom out").clicked() {
            camera.zoom_by(1.0 / 1.4);
        }
        if ui.button("+").on_hover_text("Zoom in").clicked() {
            camera.zoom_by(1.4);
        }
        ui.monospace(format!("{:.0}%", camera.zoom * 100.0));
        ui.weak("Drag to pan · wheel to zoom · double-click to fit");
    });
}

/// Paint a scene into the given rect, honoring the camera. Returns the
/// µm-per-point scale used, letting callers hit-test scene coordinates.
pub fn paint_scene(
    painter: &egui::Painter,
    rect: Rect,
    scene: &Scene,
    palette: &Palette,
    camera: SceneCamera,
) -> f32 {
    let scene_width = scene.width_um.max(1) as f32;
    let scene_height = scene.height_um.max(1) as f32;
    let fit = (rect.width() / scene_width).min(rect.height() / scene_height);
    let scale = fit * camera.zoom;
    let centered = Vec2::new(
        (rect.width() - scene_width * scale) * 0.5,
        (rect.height() - scene_height * scale) * 0.5,
    );
    let origin = rect.min + centered + camera.offset;
    let to_screen = |point: Point| {
        Pos2::new(
            origin.x + point.x_um as f32 * scale,
            origin.y + point.y_um as f32 * scale,
        )
    };

    for group in &scene.groups {
        for primitive in &group.primitives {
            match primitive {
                Primitive::Path(path) => {
                    let stroke = path.stroke.as_ref().map(|stroke| {
                        (
                            EguiStroke::new(
                                (stroke.width_um as f32 * scale).max(0.75),
                                palette.resolve(stroke.paint),
                            ),
                            dash_array(stroke, scale),
                        )
                    });
                    let mut subpath: Vec<Pos2> = Vec::new();
                    let emit = |points: &mut Vec<Pos2>| {
                        if points.len() >= 2 {
                            if let Some(fill) = path.fill
                                && let Some(inside) = fill_shape(points, palette.resolve(fill))
                            {
                                painter.add(inside);
                            }
                            if let Some((stroke, array)) = &stroke {
                                if array.is_empty() {
                                    painter.add(Shape::line(points.clone(), *stroke));
                                } else {
                                    for run in dash_polyline(points, array) {
                                        if run.len() >= 2 {
                                            painter.add(Shape::line(run, *stroke));
                                        }
                                    }
                                }
                            }
                        }
                        points.clear();
                    };
                    for segment in &path.segments {
                        match *segment {
                            PathSegment::MoveTo { to } => {
                                emit(&mut subpath);
                                subpath.push(to_screen(to));
                            }
                            PathSegment::LineTo { to } => subpath.push(to_screen(to)),
                            PathSegment::Arc {
                                center,
                                radius_um,
                                start_millideg,
                                sweep_millideg,
                            } => flatten_arc(
                                &mut subpath,
                                to_screen(center),
                                radius_um as f32 * scale,
                                start_millideg,
                                sweep_millideg,
                            ),
                            PathSegment::Close => {
                                if let Some(first) = subpath.first().copied() {
                                    subpath.push(first);
                                }
                                emit(&mut subpath);
                            }
                        }
                    }
                    emit(&mut subpath);
                }
                Primitive::Text(text) => {
                    let color = palette.resolve(text.paint);
                    let font = crate::fonts::text_font(text.font, text.height_um as f32 * scale);
                    let anchor = match text.anchor {
                        TextAnchor::Start => Align2::LEFT_BOTTOM,
                        TextAnchor::Middle => Align2::CENTER_BOTTOM,
                        TextAnchor::End => Align2::RIGHT_BOTTOM,
                    };
                    let origin = to_screen(text.origin);
                    if text.rotation_millideg == 0 {
                        painter.text(origin, anchor, &text.text, font, color);
                    } else {
                        let galley = painter.layout_no_wrap(text.text.clone(), font, color);
                        let anchored = anchor.anchor_size(origin, galley.size()).min;
                        let angle = -(text.rotation_millideg as f32 / 1000.0).to_radians();
                        painter.add(Shape::Text(
                            TextShape::new(anchored, galley, color).with_angle(angle),
                        ));
                    }
                }
            }
        }
    }
    scale
}

/// An interactive scene pane: fitted painting plus drag-pan and scroll-zoom.
pub fn scene_pane(ui: &mut egui::Ui, scene: &Scene, camera: &mut SceneCamera) {
    let available = ui.available_size();
    let aspect = scene.height_um.max(1) as f32 / scene.width_um.max(1) as f32;
    let size = Vec2::new(
        available.x,
        (available.x * aspect).min(available.y.max(120.0)),
    );
    let (rect, response) = ui.allocate_exact_size(size, Sense::click_and_drag());

    if response.dragged() {
        camera.offset += response.drag_delta();
    }
    if response.double_clicked() {
        camera.reset();
    }
    if response.hovered() {
        let scroll = ui.input(|input| input.smooth_scroll_delta.y);
        if scroll != 0.0 {
            let factor = (scroll * 0.003).exp();
            let zoomed = (camera.zoom * factor).clamp(1.0, MAX_ZOOM);
            let applied = zoomed / camera.zoom;
            if let Some(pointer) = response.hover_pos() {
                let pivot = pointer - rect.center();
                camera.offset = (camera.offset - pivot) * applied + pivot;
            }
            camera.zoom = zoomed;
        }
    }
    if camera.zoom <= 1.0 {
        camera.offset = Vec2::ZERO;
    }

    let painter = ui.painter_at(rect);
    let palette = Palette::for_dark_mode(ui.visuals().dark_mode);
    paint_scene(&painter, rect, scene, &palette, *camera);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dash_patterns_mirror_the_svg_renderer_geometry() {
        assert_eq!(dash_pattern(StrokePattern::Solid), &[] as &[f32]);
        assert_eq!(dash_pattern(StrokePattern::Dashed), &[4.0, 2.0]);
        assert_eq!(dash_pattern(StrokePattern::Dotted), &[1.0, 3.0]);
        assert_eq!(dash_pattern(StrokePattern::DashDot), &[4.0, 2.0, 1.0, 2.0]);
    }

    /// An outline carrying the pattern its author drew is drawn in exactly
    /// that pattern, its round-capped lengths read for square ends: a dot of
    /// next to nothing stands one width across. One without draws the
    /// pattern's own.
    #[test]
    fn an_outline_draws_the_exact_pattern_it_was_drawn_in() {
        let stroke = |dash_um| rspice_publication_contract::Stroke {
            width_um: 250,
            paint: rspice_publication_contract::Paint::Rgba([0, 0, 0, 255]),
            pattern: StrokePattern::Dotted,
            dash_um,
        };
        let near = |drawn: Vec<f32>, expected: [f32; 2]| {
            drawn.len() == 2
                && drawn
                    .iter()
                    .zip(expected)
                    .all(|(drawn, expected)| (drawn - expected).abs() < 1e-4)
        };
        assert!(near(
            dash_array(&stroke(Some([1, 1249])), 0.01),
            [2.51, 9.99]
        ));
        assert!(near(dash_array(&stroke(None), 0.01), [2.5, 7.5]));
    }

    /// A concave body is filled exactly, its notch left bare: the mesh's
    /// triangles add up to the outline's own area, which a fan from the
    /// first corner of an L overshoots. A convex one keeps egui's smoothed
    /// polygon.
    #[test]
    fn a_concave_fill_leaves_its_notch_bare() {
        let l_shape = [
            Pos2::new(0.0, 0.0),
            Pos2::new(40.0, 0.0),
            Pos2::new(40.0, 10.0),
            Pos2::new(10.0, 10.0),
            Pos2::new(10.0, 40.0),
            Pos2::new(0.0, 40.0),
            Pos2::new(0.0, 0.0),
        ];
        let Some(Shape::Mesh(mesh)) = fill_shape(&l_shape, Color32::RED) else {
            panic!("a concave outline fills as a mesh of ears");
        };
        let area: f32 = mesh
            .indices
            .chunks(3)
            .map(|corners| {
                let [a, b, c] = [0, 1, 2].map(|at| mesh.vertices[corners[at] as usize].pos);
                turn(a, b, c).abs() / 2.0
            })
            .sum();
        assert_eq!(area, 40.0 * 10.0 + 10.0 * 30.0);

        let square = [
            Pos2::new(0.0, 0.0),
            Pos2::new(10.0, 0.0),
            Pos2::new(10.0, 10.0),
            Pos2::new(0.0, 10.0),
            Pos2::new(0.0, 0.0),
        ];
        assert!(matches!(
            fill_shape(&square, Color32::RED),
            Some(Shape::Path(_))
        ));
    }

    #[test]
    fn dashing_a_line_produces_alternating_on_runs() {
        let points = [Pos2::new(0.0, 0.0), Pos2::new(10.0, 0.0)];
        let runs = dash_polyline(&points, &[2.0, 2.0]);
        assert_eq!(runs.len(), 3);
        assert_eq!(runs[0], vec![Pos2::new(0.0, 0.0), Pos2::new(2.0, 0.0)]);
        assert_eq!(runs[1], vec![Pos2::new(4.0, 0.0), Pos2::new(6.0, 0.0)]);
        assert_eq!(runs[2], vec![Pos2::new(8.0, 0.0), Pos2::new(10.0, 0.0)]);
    }

    #[test]
    fn dashing_never_drops_a_solid_pattern() {
        let points = [Pos2::new(0.0, 0.0), Pos2::new(3.0, 4.0)];
        assert_eq!(dash_polyline(&points, &[]), vec![points.to_vec()]);
    }

    #[test]
    fn dash_runs_follow_polyline_corners() {
        let points = [
            Pos2::new(0.0, 0.0),
            Pos2::new(3.0, 0.0),
            Pos2::new(3.0, 3.0),
        ];
        let runs = dash_polyline(&points, &[4.0, 1.0]);
        assert_eq!(runs.len(), 2);
        assert_eq!(
            runs[0],
            vec![
                Pos2::new(0.0, 0.0),
                Pos2::new(3.0, 0.0),
                Pos2::new(3.0, 1.0)
            ]
        );
        assert_eq!(runs[1], vec![Pos2::new(3.0, 2.0), Pos2::new(3.0, 3.0)]);
    }

    #[test]
    fn camera_controls_clamp_and_reset_without_losing_finite_state() {
        let mut camera = SceneCamera::default();
        camera.zoom_by(2.0);
        assert_eq!(camera.zoom, 2.0);
        camera.offset = Vec2::new(10.0, -4.0);
        camera.zoom_by(0.01);
        assert_eq!(camera, SceneCamera::default());
        camera.zoom_by(1_000.0);
        assert_eq!(camera.zoom, MAX_ZOOM);
        camera.reset();
        assert_eq!(camera, SceneCamera::default());
    }
}
