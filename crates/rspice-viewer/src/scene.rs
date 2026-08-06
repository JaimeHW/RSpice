//! Sealed-scene painting.
//!
//! Draws a contract [`Scene`] into an egui pane with the same geometry the
//! page's static SVG committed to: identical dash arrays, arc flattening
//! against a fixed chord tolerance, and role colors resolved through the
//! shared [`crate::theme`] palettes. Schematic panes add pan and zoom —
//! the reason a sheet hydrates at all.

use egui::epaint::TextShape;
use egui::{Align2, FontId, Pos2, Rect, Sense, Shape, Stroke as EguiStroke, Vec2};
use rspice_publication_contract::{
    PathSegment, Point, Primitive, Scene, StrokePattern, TextAnchor, TextFont,
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
                            stroke.pattern,
                            stroke.width_um as f32 * scale,
                        )
                    });
                    let mut subpath: Vec<Pos2> = Vec::new();
                    let emit = |points: &mut Vec<Pos2>| {
                        if points.len() >= 2 {
                            if let Some(fill) = path.fill {
                                painter.add(Shape::convex_polygon(
                                    points.clone(),
                                    palette.resolve(fill),
                                    EguiStroke::NONE,
                                ));
                            }
                            if let Some((stroke, pattern, width)) = &stroke {
                                let array = dash_pattern(*pattern);
                                if array.is_empty() {
                                    painter.add(Shape::line(points.clone(), *stroke));
                                } else {
                                    let scaled: Vec<f32> = array
                                        .iter()
                                        .map(|multiple| (multiple * width).max(0.5))
                                        .collect();
                                    for run in dash_polyline(points, &scaled) {
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
                    let font = match text.font {
                        TextFont::Sans | TextFont::SansSemibold => {
                            FontId::proportional(text.height_um as f32 * scale)
                        }
                        TextFont::Monospace => FontId::monospace(text.height_um as f32 * scale),
                    };
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
        *camera = SceneCamera::default();
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
}
