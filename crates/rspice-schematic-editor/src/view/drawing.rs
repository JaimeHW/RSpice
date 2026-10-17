//! Canvas painting.
//!
//! Draws the schematic itself: the grid, wires, junctions, labels, and
//! placed instances, in the order that puts selection and highlight on top.

use std::collections::HashMap;

use egui::emath::GuiRounding as _;
use egui::{Painter, Pos2, Rect, Stroke, Vec2};

use rspice_design::resolved_symbol::ResolvedSymbolSource;
use rspice_design::schematic::{
    bus::{Bus, BusTap},
    component::Component,
    component_display::SchematicParameterLabelVisibility,
    component_type::ComponentType,
    label_layout::{self, LayoutPoint, TextEdge},
    probe::SchematicProbe,
    wire::Wire,
};
use rspice_design_model::{
    Point,
    port::{PortDirection, PortSpec},
};

use super::super::symbols::{SymbolLibrary, draw_baked};
use super::lod::{self, Emphasis, StrokeRole, TextClass, TextLod};
use super::resolved_symbol_render::{
    draw_resolved_symbol_with_visibility, resolved_symbol_world_bounds,
};
use super::symbol_context::SchematicSymbolContext;
use super::viewport::Viewport;

pub(crate) const PROBE_RADIUS: f32 = 9.0;
pub(crate) const PROBE_CROSSHAIR_HALF_SPAN: f32 = 13.0;
/// Where a probe's label, a bus declaration and a tap slice stand, in world
/// units, from the shared layout the SVG and the printed page read too.
pub(crate) const PROBE_LABEL_OFFSET_WU: Vec2 = layout_vec(label_layout::PROBE_LABEL_OFFSET);
pub(crate) const BUS_DECLARATION_OFFSET_WU: Vec2 = layout_vec(label_layout::BUS_DECLARATION_OFFSET);
pub(crate) const TAP_SLICE_OFFSET_WU: Vec2 = layout_vec(label_layout::TAP_SLICE_OFFSET);
pub(crate) const SMALL_TEXT_WORLD: f32 = label_layout::SMALL_TEXT_WORLD;
/// Mono advance per glyph at [`SMALL_TEXT_WORLD`], for a greeked bar.
const SMALL_TEXT_GLYPH_WORLD: f32 = 6.7 * SMALL_TEXT_WORLD / 11.0;
pub(crate) const INSTANCE_NAME_WORLD: f32 = label_layout::INSTANCE_NAME_WORLD;
pub(crate) const INSTANCE_VALUE_WORLD: f32 = label_layout::INSTANCE_VALUE_WORLD;

const fn layout_vec(point: LayoutPoint) -> Vec2 {
    Vec2::new(point.x, point.y)
}

pub(crate) const fn layout_pos(point: LayoutPoint) -> Pos2 {
    Pos2::new(point.x, point.y)
}

/// Draw a wire on the canvas
pub fn draw_wire(painter: &Painter, viewport: &Viewport, wire: &Wire, ink: ConductorInk) {
    draw_conductor(painter, viewport, &wire.points, ink);
}

/// The state a conductor is drawn in. Variants are in precedence order: when
/// several apply, the later one wins, and a junction takes the ink of the
/// most emphasized conductor it sits on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConductorInk {
    /// The `wire` token.
    Plain,
    /// A net-class colour (ground, supply, clock).
    Class(egui::Color32),
    /// The net is highlighted.
    Highlighted,
    Selected,
}

impl ConductorInk {
    /// Precedence: plain, class colour, highlighted, selected. Painting in
    /// this order keeps emphasis on top.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Plain => 0,
            Self::Class(_) => 1,
            Self::Highlighted => 2,
            Self::Selected => 3,
        }
    }

    /// The more emphasized of two inks.
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }

    pub fn colour(self, palette: &rspice_ui_kit::palette::Palette) -> egui::Color32 {
        match self {
            Self::Plain => palette.wire,
            Self::Class(colour) => colour,
            Self::Highlighted => palette.canvas_net_highlight,
            Self::Selected => palette.canvas_select,
        }
    }

    /// Selection and a highlight raise the stroke; a class colour is a
    /// colour, not emphasis.
    pub const fn emphasis(self) -> Emphasis {
        match self {
            Self::Plain | Self::Class(_) => Emphasis::Plain,
            Self::Highlighted | Self::Selected => Emphasis::Raised,
        }
    }
}

/// Draw one conductor polyline: a wire, or several wires chained end to end
/// by [`chain_conductors`].
pub fn draw_conductor(painter: &Painter, viewport: &Viewport, points: &[Point], ink: ConductorInk) {
    let palette = rspice_ui_kit::tokens::active_palette();
    let width = conductor_width_pt(
        viewport.zoom,
        painter.pixels_per_point(),
        ink.emphasis() == Emphasis::Raised,
    );

    let points = points
        .iter()
        .map(|point| viewport.schematic_to_screen(*point))
        .collect();
    paint_conductor(painter, points, Stroke::new(width, ink.colour(&palette)));
}

/// The drawn width of a wire, in points: raised when it is selected or its
/// net is highlighted. A junction dot on it is sized from this.
pub fn conductor_width_pt(zoom: f32, pixels_per_point: f32, raised: bool) -> f32 {
    lod::stroke_pt(
        StrokeRole::Wire,
        zoom,
        pixels_per_point,
        Emphasis::raised_if(raised),
    )
}

/// Screen-space paths for a conductor polyline: one mitered path, split
/// wherever the polyline folds straight back on itself.
///
/// Painting a polyline one segment at a time leaves a notch at every corner:
/// two butt ends meet at the vertex and the outer square between them is never
/// painted. One path gives every interior vertex a mitered join. Ends stay
/// butt ends: egui anti-aliases with alpha-blended feather bands, so two
/// strokes whose edges coincide blend to a lighter hairline, and a cap that
/// closed a corner or hid inside another conductor would put its end edge on
/// that conductor's edge. Corners two conductors form are closed by
/// [`chain_conductors`] instead, and a butt end on another conductor's
/// centreline sits fully inside its opaque core. A fold-back is split because
/// the tessellator flares a 180° join to double width.
///
/// Axis-aligned segments are snapped to the pixel grid exactly as egui snaps
/// a `LineSegment`, so a one-pixel wire at 100% zoom stays crisp and lines up
/// with the pin leads the symbol painter still draws as segments.
fn conductor_shapes(
    mut points: Vec<Pos2>,
    stroke: Stroke,
    pixels_per_point: f32,
) -> Vec<egui::Shape> {
    points.dedup();
    let mut shapes = Vec::new();
    for mut run in split_at_folds(points) {
        snap_conductor(&mut run, &stroke, pixels_per_point);
        run.dedup();
        if run.len() >= 2 {
            shapes.push(egui::Shape::line(run, stroke));
        }
    }
    shapes
}

/// Split a deduplicated polyline into runs that never reverse direction at a
/// vertex. Each fold vertex ends one run and starts the next.
fn split_at_folds(points: Vec<Pos2>) -> Vec<Vec<Pos2>> {
    let mut runs = Vec::new();
    let mut run: Vec<Pos2> = Vec::with_capacity(points.len());
    for point in points {
        if run.len() >= 2 {
            let (a, b) = (run[run.len() - 2], run[run.len() - 1]);
            let (u, v) = (b - a, point - b);
            let folds_back = (u.x * v.y - u.y * v.x).abs() <= f32::EPSILON && u.dot(v) < 0.0;
            if folds_back {
                runs.push(std::mem::replace(&mut run, vec![b]));
            }
        }
        run.push(point);
    }
    if run.len() >= 2 {
        runs.push(run);
    }
    runs
}

/// egui's `LineSegment` pixel snapping, applied to a whole polyline: the
/// across-axis coordinate of every axis-aligned segment goes to a pixel
/// centre or edge by stroke parity, and each open end is pulled a quarter
/// pixel inward before rounding so it does not gain a pixel at the fence post.
fn snap_conductor(points: &mut [Pos2], stroke: &Stroke, pixels_per_point: f32) {
    if points.len() < 2 {
        return;
    }
    for index in 0..points.len() - 1 {
        let (a, b) = (points[index], points[index + 1]);
        if a.x == b.x {
            let mut x = a.x;
            stroke.round_center_to_pixel(pixels_per_point, &mut x);
            points[index].x = x;
            points[index + 1].x = x;
        } else if a.y == b.y {
            let mut y = a.y;
            stroke.round_center_to_pixel(pixels_per_point, &mut y);
            points[index].y = y;
            points[index + 1].y = y;
        }
    }
    let quarter_pixel = 0.25 / pixels_per_point;
    let last = points.len() - 1;
    let inward = points[1];
    snap_open_end(&mut points[0], inward, quarter_pixel, pixels_per_point);
    let inward = points[last - 1];
    snap_open_end(&mut points[last], inward, quarter_pixel, pixels_per_point);
}

fn snap_open_end(end: &mut Pos2, inward: Pos2, quarter_pixel: f32, pixels_per_point: f32) {
    if end.x == inward.x && end.y != inward.y {
        let shrunk = end.y + quarter_pixel * (inward.y - end.y).signum();
        end.y = shrunk.round_to_pixel_center(pixels_per_point);
    } else if end.y == inward.y && end.x != inward.x {
        let shrunk = end.x + quarter_pixel * (inward.x - end.x).signum();
        end.x = shrunk.round_to_pixel_center(pixels_per_point);
    }
}

/// Paint a conductor polyline as one mitered path.
pub fn paint_conductor(painter: &Painter, points: Vec<Pos2>, stroke: Stroke) {
    for shape in conductor_shapes(points, stroke, painter.pixels_per_point()) {
        painter.add(shape);
    }
}

/// Chain conductors that meet end to end into single polylines, so the corner
/// two wires form is one mitered join rather than two butt ends with a notch
/// between them. Ends chain only where exactly two conductor ends meet, in a
/// corner or a collinear continuation; a T-junction, a pin, a dangling end and
/// an end on another conductor's interior vertex keep their butt end, which
/// is either open or hidden inside the through conductor. A continuation that
/// folds straight back is never chained.
pub fn chain_conductors<'a>(polylines: impl IntoIterator<Item = &'a [Point]>) -> Vec<Vec<Point>> {
    let mut chains: Vec<Vec<Point>> = polylines
        .into_iter()
        .map(|points| {
            let mut points = points.to_vec();
            points.dedup();
            points
        })
        .filter(|points| points.len() >= 2)
        .collect();
    let mut ends: HashMap<Point, Vec<(usize, bool)>> = HashMap::new();
    for (index, chain) in chains.iter().enumerate() {
        ends.entry(chain[0]).or_default().push((index, false));
        ends.entry(chain[chain.len() - 1])
            .or_default()
            .push((index, true));
    }
    let mut consumed = vec![false; chains.len()];
    let mut output = Vec::with_capacity(chains.len());
    for start in 0..chains.len() {
        if consumed[start] {
            continue;
        }
        consumed[start] = true;
        let mut chain = std::mem::take(&mut chains[start]);
        // Grow from the tail, then turn the chain around and grow again, so
        // both ends are extended with one piece of bookkeeping.
        for _ in 0..2 {
            loop {
                let tail = chain[chain.len() - 1];
                let Some(incident) = ends.get(&tail) else {
                    break;
                };
                if incident.len() != 2 {
                    break;
                }
                let Some(&(other, at_end)) = incident.iter().find(|(index, _)| !consumed[*index])
                else {
                    break;
                };
                let candidate = &chains[other];
                let second = if at_end {
                    candidate[candidate.len() - 2]
                } else {
                    candidate[1]
                };
                if folds_back(chain[chain.len() - 2], tail, second) {
                    break;
                }
                let mut next = std::mem::take(&mut chains[other]);
                if at_end {
                    next.reverse();
                }
                consumed[other] = true;
                chain.extend_from_slice(&next[1..]);
            }
            chain.reverse();
        }
        output.push(chain);
    }
    output
}

/// Whether `b -> c` reverses straight back along `a -> b`.
fn folds_back(a: Point, b: Point, c: Point) -> bool {
    let (ux, uy) = (
        i64::from(b.x) - i64::from(a.x),
        i64::from(b.y) - i64::from(a.y),
    );
    let (vx, vy) = (
        i64::from(c.x) - i64::from(b.x),
        i64::from(c.y) - i64::from(b.y),
    );
    ux * vy == uy * vx && ux * vx + uy * vy < 0
}

/// Draw a typed multi-conductor bus. Buses deliberately use the same
/// conductor color as scalar nets, with the mockup's three parallel strokes
/// so type remains legible in monochrome exports and color-vision variants.
pub fn draw_bus(painter: &Painter, viewport: &Viewport, bus: &Bus, selected: bool) {
    let palette = rspice_ui_kit::tokens::active_palette();
    let color = if selected {
        palette.canvas_select
    } else {
        palette.wire
    };
    let ppp = painter.pixels_per_point();
    let emphasis = Emphasis::raised_if(selected);
    let centerline: Vec<Pos2> = bus
        .points
        .iter()
        .map(|point| viewport.schematic_to_screen(*point))
        .collect();
    if lod::bus_collapses(viewport.zoom, ppp) {
        // The strands would merge into a smear: one stroke says "bus".
        painter.add(egui::Shape::line(
            centerline,
            Stroke::new(
                lod::stroke_pt(StrokeRole::BusCollapsed, viewport.zoom, ppp, emphasis),
                color,
            ),
        ));
    } else {
        let width = lod::stroke_pt(StrokeRole::BusStrand, viewport.zoom, ppp, emphasis);
        for offset in [-lod::BUS_STRAND_OFFSET_WU, 0.0, lod::BUS_STRAND_OFFSET_WU] {
            painter.add(egui::Shape::line(
                offset_polyline(&centerline, offset * viewport.zoom),
                Stroke::new(width, color),
            ));
        }
    }

    if let (Some(declaration), Some(anchor)) = (&bus.declaration, bus.points.last()) {
        draw_small_electrical_text(
            painter,
            viewport,
            *anchor,
            BUS_DECLARATION_OFFSET_WU,
            &declaration.to_string(),
            // Text never takes the selection colour.
            palette.net_label,
        );
    }
}

/// A bus declaration or tap slice: 9-wu mono text whose bottom-left corner
/// sits `offset_world` from `anchor`. Below the legibility floor it greeks
/// into a bar in the same place, so the reader still sees a name there.
fn draw_small_electrical_text(
    painter: &Painter,
    viewport: &Viewport,
    anchor: Point,
    offset_world: Vec2,
    text: &str,
    color: egui::Color32,
) {
    let ppp = painter.pixels_per_point();
    let corner = viewport.schematic_to_screen(anchor) + offset_world * viewport.zoom;
    match lod::text_lod(SMALL_TEXT_WORLD, viewport.zoom, ppp, TextClass::Electrical) {
        TextLod::Draw(size) => {
            painter.text(
                corner,
                egui::Align2::LEFT_BOTTOM,
                text,
                rspice_ui_kit::theme::mono(size, rspice_ui_kit::theme::FontWeight::Medium),
                color,
            );
        }
        TextLod::Greek(bar) => {
            let line_height = SMALL_TEXT_WORLD * 13.0 / 11.0 * viewport.zoom;
            let width = lod::greek_bar_width_pt(
                text.chars().count(),
                SMALL_TEXT_GLYPH_WORLD,
                viewport.zoom,
                ppp,
            );
            let middle = corner.y - line_height * 0.5;
            painter.rect_filled(
                Rect::from_min_size(
                    Pos2::new(corner.x, middle - bar.height_pt * 0.5),
                    Vec2::new(width, bar.height_pt),
                ),
                0.0,
                color.gamma_multiply(lod::GREEK_ALPHA),
            );
        }
        TextLod::Hide => {}
    }
}

/// Draw a typed bus breakout. The selector is part of the durable electrical
/// intent and therefore appears beside the tap in the canvas and SVG.
pub fn draw_bus_tap(painter: &Painter, viewport: &Viewport, tap: &BusTap, selected: bool) {
    let palette = rspice_ui_kit::tokens::active_palette();
    let color = if selected {
        palette.canvas_select
    } else {
        palette.wire
    };
    let width = lod::stroke_pt(
        StrokeRole::BusTap,
        viewport.zoom,
        painter.pixels_per_point(),
        Emphasis::raised_if(selected),
    );
    let route: Vec<Pos2> = crate::bus_geometry::bus_tap_route_points(tap)
        .into_iter()
        .map(|point| viewport.schematic_to_screen(point))
        .collect();
    painter.add(egui::Shape::line(route, Stroke::new(width, color)));

    draw_small_electrical_text(
        painter,
        viewport,
        tap.connection_point,
        TAP_SLICE_OFFSET_WU,
        &tap.slice.to_string(),
        palette.net_label,
    );
}

/// Paint a retained probe flag using the upgraded mockup's circle, crosshair,
/// and exact source/reference label. The marker scales with the drawing so it
/// stays anchored to its authored schematic coordinate at every zoom.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeVisualStatus {
    Materialized,
    Hidden,
    Pending,
    Unavailable,
    Disabled,
}

impl ProbeVisualStatus {
    const fn suffix(self) -> &'static str {
        match self {
            Self::Materialized => "",
            Self::Hidden => " · hidden",
            Self::Pending => " · pending next run",
            Self::Unavailable => " · unavailable",
            Self::Disabled => " · disabled",
        }
    }
}

pub fn draw_probe(
    painter: &Painter,
    viewport: &Viewport,
    probe: &SchematicProbe,
    status: ProbeVisualStatus,
    selected: bool,
    hovered: bool,
) {
    let palette = rspice_ui_kit::tokens::active_palette();
    let status_color = match status {
        ProbeVisualStatus::Materialized => palette.ok,
        ProbeVisualStatus::Hidden => palette.text_faint,
        ProbeVisualStatus::Pending => palette.warn,
        ProbeVisualStatus::Unavailable => palette.err,
        ProbeVisualStatus::Disabled => palette.text_faint,
    };
    let ppp = painter.pixels_per_point();
    let center = viewport.schematic_to_screen(probe.position);
    let radius = PROBE_RADIUS * viewport.zoom;
    let half_span = PROBE_CROSSHAIR_HALF_SPAN * viewport.zoom;
    let ring = lod::stroke_pt(
        StrokeRole::Probe,
        viewport.zoom,
        ppp,
        Emphasis::raised_if(selected),
    );
    let crosshair = lod::stroke_pt(StrokeRole::Probe, viewport.zoom, ppp, Emphasis::Plain);
    if selected {
        painter.circle_filled(center, radius + 4.0 * viewport.zoom, palette.canvas_select);
    } else if hovered {
        painter.circle_stroke(
            center,
            radius + 4.0 * viewport.zoom,
            Stroke::new(lod::mark_outline_pt(ppp), palette.canvas_select),
        );
    }
    painter.circle_filled(center, radius, palette.canvas_bg);
    painter.circle_stroke(center, radius, Stroke::new(ring, status_color));
    painter.line_segment(
        [
            center - Vec2::new(half_span, 0.0),
            center + Vec2::new(half_span, 0.0),
        ],
        Stroke::new(crosshair, status_color),
    );
    painter.line_segment(
        [
            center - Vec2::new(0.0, half_span),
            center + Vec2::new(0.0, half_span),
        ],
        Stroke::new(crosshair, status_color),
    );
    // Below the floor the label hides and the marker stays.
    if let Some(size) = lod::annotation_size(SMALL_TEXT_WORLD, viewport.zoom, ppp) {
        painter.text(
            center + PROBE_LABEL_OFFSET_WU * viewport.zoom,
            egui::Align2::LEFT_BOTTOM,
            format!("{}{}", probe.reference, status.suffix()),
            rspice_ui_kit::theme::mono(size, rspice_ui_kit::theme::FontWeight::Medium),
            status_color,
        );
    }
}

/// Conservative zoom-independent authored bounds for fitting and culling.
/// The label font scales with the schematic, so its screen-space advance maps
/// to a stable world-space estimate just like component labels.
pub fn probe_world_bounds(probe: &SchematicProbe) -> (Point, Point) {
    probe.world_bounds()
}

/// Resolve a retained probe marker in screen space. The minimum radius keeps
/// markers usable when zoomed far out while the upper range still follows the
/// authored drawing scale.
pub fn probe_at_screen(
    viewport: &Viewport,
    probes: &[SchematicProbe],
    pointer: Pos2,
) -> Option<u64> {
    let radius = (PROBE_RADIUS * viewport.zoom).max(8.0) + 4.0;
    probes
        .iter()
        .enumerate()
        .filter_map(|(index, probe)| {
            let distance_sq = viewport
                .schematic_to_screen(probe.position)
                .distance_sq(pointer);
            (distance_sq <= radius * radius).then_some((distance_sq, index, probe.id))
        })
        .min_by(|left, right| {
            left.0
                .total_cmp(&right.0)
                .then_with(|| left.1.cmp(&right.1))
        })
        .map(|(_, _, id)| id)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WireScreenHit {
    pub wire_id: u64,
    /// Integer schematic attachment on the conductor when representable,
    /// quantized along the conductor to the grid pitch when one is given. A
    /// visual hit on a malformed/non-integral conductor deliberately has no
    /// attachment.
    pub attachment: Option<Point>,
    distance_sq: f32,
    authored_index: usize,
}

/// Nearest conductor under a screen-space pointer, with an attachment point
/// only when one is representable in integer schematic coordinates.
///
/// With a `grid_pitch`, the coordinate along an axis-aligned conductor is
/// quantized to that pitch (the conductor's own position fixes the other
/// coordinate, so the attachment stays on its body), and the nearer of that
/// grid point and the segment's endpoints wins. Without one the exact pointer
/// projection is the attachment, which is the `Free` grid contract.
pub fn nearest_wire_screen_hit(
    viewport: &Viewport,
    wires: &[Wire],
    pointer: Pos2,
    radius: f32,
    grid_pitch: Option<i32>,
) -> Option<WireScreenHit> {
    let radius_sq = radius.max(0.0).powi(2);
    wires
        .iter()
        .enumerate()
        .flat_map(|(authored_index, wire)| {
            wire.points.windows(2).filter_map(move |segment| {
                let start = viewport.schematic_to_screen(segment[0]);
                let end = viewport.schematic_to_screen(segment[1]);
                let vector = end - start;
                let length_sq = vector.length_sq();
                if length_sq <= f32::EPSILON {
                    return None;
                }
                let t = ((pointer - start).dot(vector) / length_sq).clamp(0.0, 1.0);
                let closest = start + vector * t;
                let distance_sq = closest.distance_sq(pointer);
                if distance_sq > radius_sq {
                    return None;
                }

                let a = segment[0];
                let b = segment[1];
                let world_x =
                    (closest.x - viewport.bounds.min.x - viewport.offset.x) / viewport.zoom;
                let world_y =
                    (closest.y - viewport.bounds.min.y - viewport.offset.y) / viewport.zoom;
                let quantize = |value: f32| match grid_pitch {
                    Some(pitch) if pitch > 0 => (value / pitch as f32).round() * pitch as f32,
                    _ => value.round(),
                };
                let distance_to = |point: Point| {
                    (point.x as f32 - world_x).powi(2) + (point.y as f32 - world_y).powi(2)
                };
                let attachment = if a.y == b.y || a.x == b.x {
                    let horizontal = a.y == b.y;
                    let (low, high) = if horizontal {
                        (a.x.min(b.x), a.x.max(b.x))
                    } else {
                        (a.y.min(b.y), a.y.max(b.y))
                    };
                    let along = quantize(if horizontal { world_x } else { world_y });
                    let on_segment = (along >= low as f32 && along <= high as f32).then(|| {
                        if horizontal {
                            Point::new(along as i32, a.y)
                        } else {
                            Point::new(a.x, along as i32)
                        }
                    });
                    // The grid point leads so it wins a tie against an endpoint
                    // it coincides with; an endpoint wins only when the pointer
                    // is genuinely nearer to it.
                    on_segment
                        .into_iter()
                        .chain([a, b])
                        .min_by(|left, right| distance_to(*left).total_cmp(&distance_to(*right)))
                } else {
                    let on_grid = grid_pitch
                        .map(|_| Point::new(quantize(world_x) as i32, quantize(world_y) as i32));
                    let exact = Point::new(world_x.round() as i32, world_y.round() as i32);
                    on_grid
                        .into_iter()
                        .chain([exact])
                        .find(|candidate| wire.contains_point(*candidate))
                };
                Some(WireScreenHit {
                    wire_id: wire.id,
                    attachment,
                    distance_sq,
                    authored_index,
                })
            })
        })
        .min_by(|left, right| {
            left.distance_sq
                .total_cmp(&right.distance_sq)
                .then_with(|| left.authored_index.cmp(&right.authored_index))
        })
}

fn offset_polyline(points: &[Pos2], offset: f32) -> Vec<Pos2> {
    if points.len() < 2 || offset == 0.0 {
        return points.to_vec();
    }
    let normal = |start: Pos2, end: Pos2| {
        let direction = end - start;
        let length = direction.length().max(f32::EPSILON);
        Vec2::new(-direction.y / length, direction.x / length)
    };
    let mut result = Vec::with_capacity(points.len());
    for (index, point) in points.iter().copied().enumerate() {
        let shifted = if index == 0 {
            point + normal(points[0], points[1]) * offset
        } else if index + 1 == points.len() {
            point + normal(points[index - 1], points[index]) * offset
        } else {
            let previous = normal(points[index - 1], points[index]);
            let next = normal(points[index], points[index + 1]);
            let sum = previous + next;
            if sum.length_sq() <= f32::EPSILON {
                point + next * offset
            } else {
                let miter = sum.normalized();
                let scale = offset / miter.dot(next).abs().max(0.25);
                point + miter * scale
            }
        };
        result.push(shifted);
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BusHit {
    pub bus_id: u64,
    pub point: Point,
    pub segment_start: Point,
    pub segment_end: Point,
}

/// Find the nearest bus segment and project the cursor onto it. The search is
/// deterministic (distance then durable id), so overlapping malformed buses
/// never cause frame-to-frame target flicker.
pub fn nearest_bus_hit(buses: &[Bus], requested: Point, radius: i32) -> Option<BusHit> {
    let radius_sq = i128::from(radius.max(0)).pow(2);
    buses
        .iter()
        .flat_map(|bus| {
            bus.points.windows(2).filter_map(move |segment| {
                let point = rspice_design::schematic::bus::nearest_lattice_point_on_segment(
                    requested, segment[0], segment[1],
                );
                let dx = i128::from(point.x) - i128::from(requested.x);
                let dy = i128::from(point.y) - i128::from(requested.y);
                let distance_sq = dx * dx + dy * dy;
                (distance_sq <= radius_sq).then_some((
                    distance_sq,
                    bus.id,
                    BusHit {
                        bus_id: bus.id,
                        point,
                        segment_start: segment[0],
                        segment_end: segment[1],
                    },
                ))
            })
        })
        .min_by_key(|(distance, bus_id, _)| (*distance, *bus_id))
        .map(|(_, _, hit)| hit)
}

pub fn bus_tap_at(taps: &[BusTap], requested: Point, radius: i32) -> Option<u64> {
    let radius_sq = i128::from(radius.max(0)).pow(2);
    taps.iter()
        .filter_map(|tap| {
            crate::bus_geometry::bus_tap_route_points(tap)
                .windows(2)
                .map(|segment| {
                    let point = rspice_design::schematic::bus::nearest_lattice_point_on_segment(
                        requested, segment[0], segment[1],
                    );
                    let dx = i128::from(point.x) - i128::from(requested.x);
                    let dy = i128::from(point.y) - i128::from(requested.y);
                    dx * dx + dy * dy
                })
                .min()
                .filter(|distance_sq| *distance_sq <= radius_sq)
                .map(|distance_sq| (distance_sq, tap.id))
        })
        .min()
        .map(|(_, id)| id)
}

/// Draw a component on the canvas
pub fn draw_component(
    painter: &Painter,
    viewport: &Viewport,
    component: &Component,
    selected: bool,
    symbol_library: Option<&SymbolLibrary>,
    symbol_context: &SchematicSymbolContext,
    parameter_labels: SchematicParameterLabelVisibility,
) {
    // Component uses `pos` not `position`, `kind` not `component_type`
    let pos = viewport.schematic_to_screen(component.pos);
    let scale = viewport.zoom;

    // Grid lines now visible through components (no opaque background)

    let palette = rspice_ui_kit::tokens::active_palette();
    let outline_color = if selected {
        palette.canvas_select
    } else {
        palette.symbol
    };

    let ppp = painter.pixels_per_point();
    let stroke = Stroke::new(
        lod::stroke_pt(
            StrokeRole::Symbol,
            scale,
            ppp,
            Emphasis::raised_if(selected),
        ),
        outline_color,
    );

    let rotation_degrees = component.rotation.degrees();
    let resolved_cell_symbol = if component.kind == ComponentType::CellInstance {
        symbol_context.resolved_symbol(component)
    } else {
        None
    };

    // Authored symbols are the sole component-body source. Missing library or
    // instance resolution is rendered as an explicit error marker; it must
    // never silently change the circuit's visual semantics.
    let symbol_drew_its_own_labels = if component.kind == ComponentType::CellInstance {
        if symbol_library
            .and_then(|library| compatible_builtin_xspice_asset(component, library))
            .and_then(|(library, filename, width, height)| {
                library.baked_asset(
                    filename,
                    width,
                    height,
                    rotation_degrees,
                    component.mirror_h,
                    component.mirror_v,
                )
            })
            .is_some_and(|baked| {
                draw_baked(painter, &baked, pos, scale, stroke);
                draw_artwork_lead_extensions(painter, pos, scale, component, stroke);
                true
            })
        {
            false
        } else if let Some(symbol) = resolved_cell_symbol
            && symbol.source() == ResolvedSymbolSource::Authored
            && resolved_symbol_world_bounds(component, symbol).is_some()
        {
            draw_resolved_symbol_with_visibility(
                painter,
                pos,
                scale,
                component,
                symbol,
                stroke,
                parameter_labels,
            );
            true
        } else {
            draw_symbol_resolution_error(painter, pos, scale, component.kind, "unresolved cell");
            false
        }
    } else if let Some((library, symbol, adjusted_rotation)) = symbol_library.and_then(|library| {
        library
            .get_with_rotation_variant(
                component.kind,
                rotation_degrees,
                component.symbol_variant.as_deref(),
            )
            .map(|(symbol, rotation)| (library, symbol, rotation))
    }) {
        let symbol_stroke = if component.kind == ComponentType::Port {
            port_symbol_stroke(stroke, scale, ppp, selected, component.port_spec().as_ref())
        } else {
            stroke
        };
        let baked = library.baked(
            symbol,
            adjusted_rotation,
            component.mirror_h,
            component.mirror_v,
        );
        draw_baked(painter, &baked, pos, scale, symbol_stroke);
        if component.kind == ComponentType::Port {
            draw_port_direction_overlay(
                painter,
                pos,
                scale,
                adjusted_rotation,
                component.mirror_h,
                component.mirror_v,
                component
                    .port_spec()
                    .map(|port| port.direction)
                    .unwrap_or_default(),
                symbol_stroke,
            );
        }
        false
    } else {
        draw_symbol_resolution_error(painter, pos, scale, component.kind, "missing canonical SVG");
        false
    };

    // Smart label placement based on component type, rotation, and dimensions
    // Commercial EDA tools (Cadence Virtuoso) place labels to avoid overlapping
    // terminals and component body, with name/value on opposite sides.
    //
    // A resolved authored cell symbol prints its own name and value against
    // the anchors in its symbol document. Canonical catalog artwork and error
    // states use the ordinary instance labels here.
    if !symbol_drew_its_own_labels && parameter_labels != SchematicParameterLabelVisibility::Hidden
    {
        draw_component_labels(painter, pos, scale, component, parameter_labels);
    }
}

pub fn port_symbol_stroke(
    symbol_stroke: Stroke,
    zoom: f32,
    pixels_per_point: f32,
    selected: bool,
    spec: Option<&PortSpec>,
) -> Stroke {
    if spec.is_none_or(|port| port.vector().is_none()) {
        return symbol_stroke;
    }
    Stroke::new(
        lod::stroke_pt(
            StrokeRole::VectorPort,
            zoom,
            pixels_per_point,
            Emphasis::raised_if(selected),
        ),
        symbol_stroke.color,
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "Independent canvas transform, port direction, and paint inputs."
)]
pub fn draw_port_direction_overlay(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    rotation_degrees: i32,
    mirror_h: bool,
    mirror_v: bool,
    direction: PortDirection,
    stroke: Stroke,
) {
    let screen_point = |point| {
        let point =
            crate::port_overlay::transform_point(point, rotation_degrees, mirror_h, mirror_v);
        Pos2::new(pos.x + point.x * scale, pos.y + point.y * scale)
    };
    for segment in crate::port_overlay::direction_segments(direction) {
        painter.line_segment(
            [screen_point(segment.start), screen_point(segment.end)],
            stroke,
        );
    }
}

pub fn draw_symbol_resolution_error(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    kind: ComponentType,
    reason: &str,
) {
    let color = egui::Color32::from_rgb(220, 70, 70);
    let half = 12.0 * scale.max(0.25);
    let rect = Rect::from_center_size(pos, Vec2::splat(half * 2.0));
    let stroke = Stroke::new(
        lod::stroke_pt(
            StrokeRole::Symbol,
            scale,
            painter.pixels_per_point(),
            Emphasis::Raised,
        ),
        color,
    );
    painter.rect_stroke(rect, 1.0, stroke, egui::StrokeKind::Inside);
    painter.line_segment([rect.left_top(), rect.right_bottom()], stroke);
    painter.line_segment([rect.right_top(), rect.left_bottom()], stroke);
    painter.text(
        pos + egui::vec2(0.0, half + 4.0 * scale.max(0.5)),
        egui::Align2::CENTER_TOP,
        format!("{}: {reason}", kind.display_name()),
        egui::FontId::monospace(9.0 * scale.max(0.75)),
        color,
    );
}

/// Resolve authored artwork only when every visible lead anchor is exactly
/// the same point as the frozen executable terminal layout. A mismatch uses
/// another authored cell symbol when available, otherwise an explicit error.
pub fn compatible_builtin_xspice_asset<'a>(
    component: &'a Component,
    library: &'a SymbolLibrary,
) -> Option<(&'a SymbolLibrary, &'a str, f32, f32)> {
    let contract = component.library_cell.as_ref()?.builtin_xspice.as_ref()?;
    let (width, height) = component.artwork_dimensions();
    let offsets = component.artwork_pin_offsets();
    library
        .asset_matches_terminal_offsets(
            &contract.symbol_asset,
            width as f32,
            height as f32,
            &offsets,
        )
        .then_some((
            library,
            contract.symbol_asset.as_str(),
            width as f32,
            height as f32,
        ))
}

/// Carry artwork leads out to terminals the drawing itself does not reach,
/// which happens when a long interface widens the block past the size the
/// artwork was authored for.
pub fn draw_artwork_lead_extensions(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    component: &Component,
    stroke: Stroke,
) {
    for (edge, terminal) in component.artwork_lead_extensions() {
        let to_screen = |point: Point| {
            let transformed = component.transform_point(point);
            Pos2::new(
                pos.x + transformed.x as f32 * scale,
                pos.y + transformed.y as f32 * scale,
            )
        };
        painter.line_segment([to_screen(edge), to_screen(terminal)], stroke);
    }
}

/// Where a primitive's name and value sit and how they hang off their
/// anchors. Linear in `pos` and `scale`, so the SVG evaluates it in its own
/// units.
#[derive(Debug, Clone, Copy)]
pub(crate) struct LabelLayout {
    pub(crate) name_pos: Pos2,
    pub(crate) name_align: egui::Align2,
    pub(crate) value_pos: Pos2,
    pub(crate) value_align: egui::Align2,
}

/// How a run of the shared layout's text hangs off its anchor, as egui
/// aligns a galley.
pub(crate) const fn text_align(horizontal: TextEdge, vertical: TextEdge) -> egui::Align2 {
    const fn align(edge: TextEdge) -> egui::Align {
        match edge {
            TextEdge::Start => egui::Align::Min,
            TextEdge::Center => egui::Align::Center,
            TextEdge::End => egui::Align::Max,
        }
    }
    egui::Align2([align(horizontal), align(vertical)])
}

/// The shared instance label layout (`label_layout::instance_label_layout`)
/// evaluated at `pos` with `scale` points per world unit.
pub(crate) fn compute_label_layout(pos: Pos2, scale: f32, component: &Component) -> LabelLayout {
    let layout =
        label_layout::instance_label_layout(LayoutPoint::new(pos.x, pos.y), scale, component);
    LabelLayout {
        name_pos: layout_pos(layout.name.anchor),
        name_align: text_align(layout.name.horizontal, layout.name.vertical),
        value_pos: layout_pos(layout.value.anchor),
        value_align: text_align(layout.value.horizontal, layout.value.vertical),
    }
}

fn draw_component_labels(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    component: &Component,
    visibility: SchematicParameterLabelVisibility,
) {
    // Skip labels for Ground (too small, clutters schematic)
    if matches!(component.kind, ComponentType::Ground) {
        return;
    }

    // Below the legibility floor labels are unreadable smudge — commercial
    // EDA tools hide them; so do we, and zoomed-out paint cost drops with them.
    let ppp = painter.pixels_per_point();
    let layout = compute_label_layout(pos, scale, component);
    let palette = rspice_ui_kit::tokens::active_palette();

    // Draw component name (reference designator)
    if let Some(size) = lod::annotation_size(INSTANCE_NAME_WORLD, scale, ppp)
        && component.display_mode.show_name(visibility)
        && !component.name.is_empty()
    {
        painter.text(
            layout.name_pos,
            layout.name_align,
            &component.name,
            rspice_ui_kit::theme::sans(size, rspice_ui_kit::theme::FontWeight::Medium),
            palette.text,
        );
    }

    // Draw component value, or a port's name, which is an electrical name.
    if !component.display_mode.show_value(visibility) {
        return;
    }
    match label_layout::instance_value_class(component) {
        label_layout::ValueClass::Value => {
            if let Some(size) = lod::annotation_size(INSTANCE_VALUE_WORLD, scale, ppp) {
                let value_text =
                    super::super::source_labels::component_value_label_cached(component);
                if !value_text.is_empty() {
                    painter.text(
                        layout.value_pos,
                        layout.value_align,
                        value_text.as_ref(),
                        rspice_ui_kit::theme::sans(size, rspice_ui_kit::theme::FontWeight::Regular),
                        palette.text_dim,
                    );
                }
            }
        }
        label_layout::ValueClass::NetName => {
            let name = super::super::source_labels::component_value_label_cached(component);
            if !name.is_empty() {
                draw_electrical_name(
                    painter,
                    layout.value_pos,
                    layout.value_align,
                    name.as_ref(),
                    scale,
                    palette.net_label,
                );
            }
        }
    }
}

/// A port's name: an electrical name in a net label's type (Plex Mono Medium,
/// 11 wu), hung off `at` as `align` says. Below the legibility floor it
/// greeks into a bar where the text would be, as a net label's name does, so
/// a zoomed-out sheet still shows that a name is there (D1 section 1).
fn draw_electrical_name(
    painter: &Painter,
    at: Pos2,
    align: egui::Align2,
    text: &str,
    zoom: f32,
    color: egui::Color32,
) {
    let ppp = painter.pixels_per_point();
    let world = label_layout::ValueClass::NetName.size_world();
    match lod::text_lod(world, zoom, ppp, TextClass::Electrical) {
        TextLod::Draw(size) => {
            painter.text(
                at,
                align,
                text,
                rspice_ui_kit::theme::mono(size, rspice_ui_kit::theme::FontWeight::Medium),
                color,
            );
        }
        TextLod::Greek(bar) => {
            let width = lod::greek_bar_width_pt(
                text.chars().count(),
                label_layout::NET_LABEL_GLYPH_WIDTH_WORLD,
                zoom,
                ppp,
            );
            let text_box = align.anchor_size(
                at,
                Vec2::new(width, label_layout::LINE_BOX_EM * world * zoom),
            );
            painter.rect_filled(
                Rect::from_center_size(text_box.center(), Vec2::new(width, bar.height_pt)),
                0.0,
                color.gamma_multiply(lod::GREEK_ALPHA),
            );
        }
        TextLod::Hide => {}
    }
}

/// How a junction dot is painted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JunctionPaint {
    /// The junction itself is selected: selection fill and a ring.
    pub selected: bool,
    pub hovered: bool,
    /// The most emphasized conductor that meets the dot. The dot takes its
    /// colour and is sized from its drawn width, so a selected or
    /// highlighted wire never swallows its own junction.
    pub incident: ConductorInk,
}

impl JunctionPaint {
    pub const fn plain() -> Self {
        Self {
            selected: false,
            hovered: false,
            incident: ConductorInk::Plain,
        }
    }

    /// The dot's fill: the junction's own selection, else the ink of the
    /// conductors it joins.
    pub fn fill(&self, palette: &rspice_ui_kit::palette::Palette) -> egui::Color32 {
        if self.selected {
            palette.canvas_select
        } else {
            self.incident.colour(palette)
        }
    }
}

/// Draw a junction (net connection point).
pub fn draw_junction(
    painter: &Painter,
    viewport: &Viewport,
    position: Point,
    paint: JunctionPaint,
) {
    let ppp = painter.pixels_per_point();
    let pos = viewport.schematic_to_screen(position);
    let incident_width = conductor_width_pt(
        viewport.zoom,
        ppp,
        paint.incident.emphasis() == Emphasis::Raised,
    );
    let radius = lod::junction_radius_pt(viewport.zoom, incident_width);

    let palette = rspice_ui_kit::tokens::active_palette();
    if paint.selected {
        painter.circle_stroke(
            pos,
            radius + 2.0,
            Stroke::new(lod::mark_outline_pt(ppp), palette.canvas_select),
        );
    } else if paint.hovered {
        painter.circle_stroke(
            pos,
            radius + 3.0,
            Stroke::new(lod::hover_ring_pt(ppp), palette.canvas_select),
        );
    }

    painter.circle_filled(pos, radius, paint.fill(&palette));
}

/// How far a hover outline stands off what it outlines, in points.
const HOVER_OUTSET: f32 = 3.0;
const HOVER_DASH: f32 = 3.0;
const HOVER_GAP: f32 = 2.0;
const HOVER_ALPHA: f32 = 0.85;
/// The hover halo under a conductor: this much wider than the stroke, in
/// points, at this alpha.
const HOVER_HALO_WIDEN: f32 = 4.0;
const HOVER_HALO_ALPHA: f32 = 0.35;

/// The hover pre-highlight of an object with an area (an instance's pick
/// bounds, a label's decoration rect, a note or shape's bounds): a dashed
/// 1-dpx outline in the selection colour, 3 pt outside `rect`, with square
/// corners. Dashed, so it reads apart from both selection (a solid
/// recolour) and a window band (a solid edge).
pub fn draw_hover_outline(painter: &Painter, rect: Rect) {
    let ppp = painter.pixels_per_point();
    let rect = rect.expand(HOVER_OUTSET);
    let crisp = |value: f32| ((value * ppp).floor() + 0.5) / ppp;
    let rect = Rect::from_min_max(
        Pos2::new(crisp(rect.min.x), crisp(rect.min.y)),
        Pos2::new(crisp(rect.max.x), crisp(rect.max.y)),
    );
    let stroke = Stroke::new(
        1.0 / ppp,
        rspice_ui_kit::tokens::active_palette()
            .canvas_select
            .gamma_multiply(HOVER_ALPHA),
    );
    painter.extend(egui::Shape::dashed_line(
        &[
            rect.left_top(),
            rect.right_top(),
            rect.right_bottom(),
            rect.left_bottom(),
            rect.left_top(),
        ],
        stroke,
        HOVER_DASH,
        HOVER_GAP,
    ));
}

/// The hover pre-highlight of a conductor: a halo beneath it along the same
/// path, 4 pt wider than `width_pt`, in the selection colour at alpha 0.35.
pub fn draw_hover_halo(painter: &Painter, viewport: &Viewport, points: &[Point], width_pt: f32) {
    let points = points
        .iter()
        .map(|point| viewport.schematic_to_screen(*point))
        .collect();
    paint_conductor(
        painter,
        points,
        Stroke::new(
            width_pt + HOVER_HALO_WIDEN,
            rspice_ui_kit::tokens::active_palette()
                .canvas_select
                .gamma_multiply(HOVER_HALO_ALPHA),
        ),
    );
}

/// An instance's open pins: a hollow square centred on each terminal nothing
/// meets, 3 wu a side held between 4 and 7 pt, outlined 1 pt in the
/// instance's own stroke colour. A connected pin has no mark. It is an
/// editing aid, so only the scene draws it: never a ghost, an export or a
/// print.
pub fn draw_open_pin_marks(
    painter: &Painter,
    viewport: &Viewport,
    open: &[Point],
    colour: egui::Color32,
) {
    let ppp = painter.pixels_per_point().max(f32::EPSILON);
    // A whole, odd number of device pixels between the outline's centres:
    // with the centre on the pixel corner a lead's crisp stroke is centred
    // on, the 1-dpx edges then land on pixel centres, square about the lead.
    let side_px = (lod::open_pin_side_pt(viewport.zoom) * ppp)
        .round()
        .max(1.0);
    let side = (side_px + 1.0 - side_px % 2.0) / ppp;
    let stroke = Stroke::new(lod::mark_outline_pt(ppp), colour);
    for terminal in open {
        let centre = viewport.schematic_to_screen(*terminal);
        let snap = |value: f32| (value * ppp).round() / ppp;
        painter.rect_stroke(
            Rect::from_center_size(Pos2::new(snap(centre.x), snap(centre.y)), Vec2::splat(side)),
            0.0,
            stroke,
            egui::StrokeKind::Middle,
        );
    }
}

/// The drawn width of a bus as one band, in points: its outer strands'
/// span, or its collapsed stroke.
pub fn bus_band_width_pt(zoom: f32, pixels_per_point: f32) -> f32 {
    if lod::bus_collapses(zoom, pixels_per_point) {
        lod::stroke_pt(
            StrokeRole::BusCollapsed,
            zoom,
            pixels_per_point,
            Emphasis::Plain,
        )
    } else {
        2.0 * lod::BUS_STRAND_OFFSET_WU * zoom
            + lod::stroke_pt(
                StrokeRole::BusStrand,
                zoom,
                pixels_per_point,
                Emphasis::Plain,
            )
    }
}

/// The drawn width of a bus tap, in points.
pub fn bus_tap_width_pt(zoom: f32, pixels_per_point: f32) -> f32 {
    lod::stroke_pt(StrokeRole::BusTap, zoom, pixels_per_point, Emphasis::Plain)
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Context, Id, LayerId, Order, Shape};
    use rspice_design::schematic::bus::BusTapOrientation;
    use rspice_design_model::bus::{BusDeclaration, BusSlice};

    fn wire_stroke_width(selected: bool, highlighted: bool, zoom: f32) -> f32 {
        let ink = if selected {
            ConductorInk::Selected
        } else if highlighted {
            ConductorInk::Highlighted
        } else {
            ConductorInk::Plain
        };
        wire_ink_width(ink, zoom)
    }

    fn wire_ink_width(ink: ConductorInk, zoom: f32) -> f32 {
        let ctx = Context::default();
        let painter = Painter::new(
            ctx,
            LayerId::new(Order::Foreground, Id::new("wire-stroke-test")),
            Rect::from_min_size(Pos2::ZERO, Vec2::splat(100.0)),
        );
        let viewport = Viewport {
            offset: Pos2::ZERO,
            zoom,
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::splat(100.0)),
        };
        let wire = Wire::segment(1, Point::new(0, 0), Point::new(10, 0));

        draw_wire(&painter, &viewport, &wire, ink);

        let mut widths = Vec::new();
        painter.for_each_shape(|shape| {
            if let Shape::Path(path) = &shape.shape {
                widths.push(path.stroke.width);
            }
        });

        assert_eq!(widths.len(), 1);
        widths[0]
    }

    /// The painted wire is the level-of-detail width: whole device pixels
    /// while thin, one point wider when selected or highlighted.
    #[test]
    fn painted_wires_take_the_level_of_detail_width() {
        for (zoom, plain, raised) in [
            (0.25, 1.0, 2.0),
            (1.0, 1.0, 2.0),
            (2.0, 2.0, 3.0),
            (8.0, 3.0, 4.0),
        ] {
            assert_eq!(wire_stroke_width(false, false, zoom), plain, "zoom {zoom}");
            assert_eq!(wire_stroke_width(true, false, zoom), raised, "zoom {zoom}");
            assert_eq!(wire_stroke_width(false, true, zoom), raised, "zoom {zoom}");
            let class = ConductorInk::Class(egui::Color32::from_rgb(1, 2, 3));
            assert_eq!(
                wire_ink_width(class, zoom),
                plain,
                "a class colour is not emphasis"
            );
        }
    }

    /// The precedence for one conductor and for the dot on it: selected,
    /// then highlighted, then a class colour, then the wire. The dot takes
    /// the colour of the most emphasized conductor it joins and is sized
    /// from its width, so a selected wire no longer swallows its junction.
    #[test]
    fn a_junction_takes_the_ink_and_width_of_its_most_emphasized_wire() {
        let palette = rspice_ui_kit::palette::INSTRUMENT_DARK;
        let class = ConductorInk::Class(palette.net_class_ground);
        let highlighted = ConductorInk::Plain
            .max(class)
            .max(ConductorInk::Highlighted);
        assert_eq!(highlighted, ConductorInk::Highlighted);
        assert_eq!(
            highlighted.max(ConductorInk::Selected),
            ConductorInk::Selected
        );
        assert_eq!(ConductorInk::Selected.max(class), ConductorInk::Selected);
        assert_eq!(ConductorInk::Plain.max(class), class);

        let paint = |selected, incident| JunctionPaint {
            selected,
            hovered: false,
            incident,
        };
        assert_eq!(
            paint(true, ConductorInk::Plain).fill(&palette),
            palette.canvas_select
        );
        assert_eq!(
            paint(false, ConductorInk::Selected).fill(&palette),
            palette.canvas_select
        );
        assert_eq!(
            paint(false, ConductorInk::Highlighted).fill(&palette),
            palette.canvas_net_highlight
        );
        assert_eq!(paint(false, class).fill(&palette), palette.net_class_ground);
        assert_eq!(
            paint(false, ConductorInk::Plain).fill(&palette),
            palette.wire
        );

        // Painted: a dot on a selected wire at zoom 2 is 2.5 wires across.
        let ctx = Context::default();
        let painter = Painter::new(
            ctx,
            LayerId::new(Order::Foreground, Id::new("junction-paint-test")),
            Rect::from_min_size(Pos2::ZERO, Vec2::splat(100.0)),
        );
        let viewport = Viewport {
            offset: Pos2::new(50.0, 50.0),
            zoom: 2.0,
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::splat(100.0)),
        };
        draw_junction(
            &painter,
            &viewport,
            Point::new(0, 0),
            paint(false, ConductorInk::Selected),
        );
        let mut dots = Vec::new();
        painter.for_each_shape(|shape| {
            if let Shape::Circle(circle) = &shape.shape {
                dots.push((circle.radius, circle.fill));
            }
        });
        let selected_wire = wire_ink_width(ConductorInk::Selected, 2.0);
        assert_eq!(dots.len(), 1);
        assert!(
            2.0 * dots[0].0 >= 2.5 * selected_wire,
            "{dots:?} on {selected_wire}"
        );
        assert_eq!(
            dots[0].1,
            rspice_ui_kit::tokens::active_palette().canvas_select
        );
    }

    #[test]
    fn canonical_vector_port_body_and_overlay_use_bus_weight() {
        // At 2x and zoom 2 the bus weight and the symbol weight land on
        // different whole device pixels (5 against 4).
        let (zoom, ppp) = (2.0, 2.0);
        let width = lod::stroke_pt(StrokeRole::Symbol, zoom, ppp, Emphasis::Plain);
        let symbol = Stroke::new(width, rspice_ui_kit::tokens::active_palette().symbol);
        let spec = |name: &str| PortSpec {
            name: name.to_owned(),
            direction: PortDirection::InOut,
        };
        let vector = spec("DATA[7:0]");

        assert_eq!(width, 2.0);
        assert_eq!(
            port_symbol_stroke(symbol, zoom, ppp, false, Some(&vector)).width,
            2.5
        );
        assert_eq!(
            port_symbol_stroke(symbol, zoom, ppp, true, Some(&vector)).width,
            lod::stroke_pt(StrokeRole::VectorPort, zoom, ppp, Emphasis::Raised)
        );
        for scalar in ["EN", "DATA[3]", "bias_1"] {
            assert_eq!(
                port_symbol_stroke(symbol, zoom, ppp, false, Some(&spec(scalar))),
                symbol,
                "{scalar}"
            );
        }
        assert_eq!(port_symbol_stroke(symbol, zoom, ppp, false, None), symbol);
    }

    #[test]
    fn catalog_svg_is_used_only_when_its_leads_match_every_electrical_terminal() {
        let library = SymbolLibrary::load_embedded().expect("symbol library");
        let component = |model_type: &str| {
            let descriptor = rspice_design::schematic::device_catalog::engine_only_xspice_devices()
                .iter()
                .find(|descriptor| descriptor.model_type == model_type)
                .expect("catalog descriptor");
            let binding = rspice_design::schematic::device_catalog::builtin_xspice_library_binding(
                descriptor,
            )
            .expect("catalog binding");
            Component::new(1, ComponentType::CellInstance, Point::origin())
                .with_library_cell(binding)
        };

        assert!(compatible_builtin_xspice_asset(&component("astate"), &library).is_some());
        assert!(
            compatible_builtin_xspice_asset(&component("nco"), &library).is_none(),
            "the compact NCO bus artwork must not impersonate seven scalar wire terminals"
        );
    }

    /// Every catalog device whose artwork was authored for its own interface
    /// must keep drawing that artwork. Interface-derived terminal spacing must
    /// never cost a device its glyph, and must never stretch one.
    #[test]
    fn every_catalog_device_with_matching_artwork_still_draws_it_undistorted() {
        const ARTWORK_DEVICES: [&str; 25] = [
            "astate",
            "cmeter",
            "d_fdiv",
            "d_open_c",
            "d_open_e",
            "d_osc",
            "d_pwm",
            "d_source",
            "d_xnor",
            "file_source",
            "hyst",
            "lmeter",
            "pspice_d_stim",
            "pwl",
            "pwlts",
            "real_gain",
            "real_to_v",
            "s_xfer",
            "sine",
            "slew",
            "square",
            "table2d",
            "triangle",
            "v_to_real",
            "xfer",
        ];

        let library = SymbolLibrary::load_embedded().expect("symbol library");
        let mut matched = Vec::new();
        for descriptor in rspice_design::schematic::device_catalog::engine_only_xspice_devices() {
            let Ok(binding) =
                rspice_design::schematic::device_catalog::builtin_xspice_library_binding(
                    descriptor,
                )
            else {
                continue;
            };
            let component = Component::new(1, ComponentType::CellInstance, Point::origin())
                .with_library_cell(binding);
            if compatible_builtin_xspice_asset(&component, &library).is_none() {
                continue;
            }
            matched.push(descriptor.model_type);
            let (artwork_width, artwork_height) = component.artwork_dimensions();
            let (_, height) = component.symbol_dimensions();
            assert_eq!(
                (artwork_width, artwork_height),
                (rspice_design::symbol_generation::GENERATED_WIDTH, height),
                "{} artwork must be drawn in the box it was authored for",
                descriptor.model_type
            );
        }

        assert_eq!(
            matched, ARTWORK_DEVICES,
            "the set of catalog devices drawn with authored artwork changed"
        );
    }

    /// A block widened for its pin names still carries its artwork out to the
    /// terminals, so nothing is left floating off the drawing.
    #[test]
    fn artwork_leads_extend_to_terminals_the_drawing_does_not_reach() {
        let descriptor = rspice_design::schematic::device_catalog::engine_only_xspice_devices()
            .iter()
            .find(|descriptor| descriptor.model_type == "d_fdiv")
            .expect("catalog descriptor");
        let binding =
            rspice_design::schematic::device_catalog::builtin_xspice_library_binding(descriptor)
                .expect("catalog binding");
        let component = Component::new(1, ComponentType::CellInstance, Point::origin())
            .with_library_cell(binding);

        let (width, _) = component.symbol_dimensions();
        assert!(
            width > rspice_design::symbol_generation::GENERATED_WIDTH,
            "freq_in/freq_out do not fit the nominal body"
        );
        let extensions = component.artwork_lead_extensions();
        assert_eq!(extensions.len(), 2, "{extensions:?}");
        for (edge, terminal) in extensions {
            assert_eq!(edge.y, terminal.y, "a lead extension must run straight");
            assert_eq!(
                edge.x.abs(),
                rspice_design::symbol_generation::GENERATED_WIDTH / 2
            );
            assert_eq!(terminal.x.abs(), width / 2);
        }
    }

    #[test]
    fn retained_probe_bounds_include_crosshair_and_reference_label() {
        let probe =
            SchematicProbe::new(8, Point::new(100, 50), "V(OUT)", Some("V(OUT)".to_owned()))
                .unwrap();
        let (min, max) = probe_world_bounds(&probe);

        assert!(min.x < probe.position.x);
        assert!(min.y < probe.position.y);
        assert!(max.y > probe.position.y);
        assert!(
            max.x - probe.position.x
                > i32::try_from(probe.reference.chars().count()).unwrap_or(i32::MAX),
            "fit and culling bounds must retain the complete reference label"
        );
    }

    #[test]
    fn probe_hit_target_remains_usable_at_low_and_high_zoom() {
        let probe = SchematicProbe::new(8, Point::new(10, 20), "V(OUT)", None).unwrap();
        for zoom in [0.1, 3.0] {
            let viewport = Viewport {
                offset: Pos2::new(7.0, 11.0),
                zoom,
                bounds: Rect::from_min_size(Pos2::new(100.0, 200.0), Vec2::splat(400.0)),
            };
            let pointer = viewport.schematic_to_screen(probe.position) + Vec2::new(10.0, 0.0);

            assert_eq!(
                probe_at_screen(&viewport, std::slice::from_ref(&probe), pointer),
                Some(8)
            );
        }
    }

    #[test]
    fn a_conductor_is_one_path_snapped_like_a_line_segment() {
        // A one-pixel stroke snaps its centreline to pixel centres, and each
        // open end is pulled a quarter pixel inward before rounding: exactly
        // what egui does to a `LineSegment`, so wires and pin leads agree.
        let stroke = Stroke::new(1.0, egui::Color32::WHITE);
        let shapes = conductor_shapes(
            vec![
                Pos2::new(10.0, 10.0),
                Pos2::new(10.0, 10.0),
                Pos2::new(10.0, 50.0),
                Pos2::new(50.0, 50.0),
            ],
            stroke,
            1.0,
        );
        let [Shape::Path(path)] = shapes.as_slice() else {
            panic!("a conductor is one path, never a segment per vertex pair");
        };
        assert!(!path.closed);
        assert_eq!(
            path.points,
            vec![
                Pos2::new(10.5, 10.5),
                Pos2::new(10.5, 50.5),
                Pos2::new(49.5, 50.5)
            ]
        );
        assert!((path.stroke.width - 1.0).abs() < f32::EPSILON);

        assert!(
            conductor_shapes(vec![Pos2::new(3.0, 3.0), Pos2::new(3.0, 3.0)], stroke, 1.0)
                .is_empty(),
            "a degenerate conductor paints nothing"
        );

        // A path that folds straight back is two runs, never one flared join.
        let folded = conductor_shapes(
            vec![
                Pos2::new(10.0, 10.0),
                Pos2::new(10.0, 50.0),
                Pos2::new(10.0, 30.0),
            ],
            stroke,
            1.0,
        );
        assert_eq!(folded.len(), 2);
    }

    #[test]
    fn conductors_chain_only_where_exactly_two_ends_meet() {
        let corner = [Point::new(0, 0), Point::new(0, 10)];
        let continuation = [Point::new(0, 10), Point::new(10, 10)];
        let chained = chain_conductors([&corner[..], &continuation[..]]);
        assert_eq!(
            chained,
            vec![vec![
                Point::new(0, 0),
                Point::new(0, 10),
                Point::new(10, 10)
            ]]
        );

        // Authoring direction does not matter: a continuation drawn towards
        // the shared end is turned around.
        let reversed = [Point::new(10, 10), Point::new(0, 10)];
        let chained = chain_conductors([&corner[..], &reversed[..]]);
        assert_eq!(
            chained,
            vec![vec![
                Point::new(0, 0),
                Point::new(0, 10),
                Point::new(10, 10)
            ]]
        );

        // Three ends at a point are a junction; nothing chains there.
        let third = [Point::new(0, 10), Point::new(0, 20)];
        let chained = chain_conductors([&corner[..], &continuation[..], &third[..]]);
        assert_eq!(chained.len(), 3);

        // An end on another conductor's interior vertex is not a shared end.
        let bent = [Point::new(0, 0), Point::new(0, 10), Point::new(10, 10)];
        let onto_vertex = [Point::new(-10, 10), Point::new(0, 10)];
        assert_eq!(chain_conductors([&bent[..], &onto_vertex[..]]).len(), 2);

        // A continuation that folds straight back keeps both conductors apart.
        let fold = [Point::new(0, 10), Point::new(0, 5)];
        assert_eq!(chain_conductors([&corner[..], &fold[..]]).len(), 2);

        // Collinear pieces, a wire split at a former junction, become one run.
        let left = [Point::new(0, 0), Point::new(10, 0)];
        let right = [Point::new(10, 0), Point::new(20, 0)];
        assert_eq!(
            chain_conductors([&left[..], &right[..]]),
            vec![vec![Point::new(0, 0), Point::new(10, 0), Point::new(20, 0)]]
        );
    }

    #[test]
    fn wire_screen_hit_quantizes_along_the_conductor_to_the_grid_pitch() {
        let viewport = Viewport {
            offset: Pos2::ZERO,
            zoom: 2.0,
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::splat(400.0)),
        };

        let on_grid = Wire::segment(5, Point::new(0, 10), Point::new(40, 10));
        let pointer = viewport.schematic_to_screen(Point::new(17, 10)) + Vec2::new(0.0, 2.0);
        let hit = nearest_wire_screen_hit(&viewport, &[on_grid], pointer, 4.0, Some(10)).unwrap();
        assert_eq!(hit.attachment, Some(Point::new(20, 10)));

        // Only the coordinate along the conductor is quantized: a conductor
        // that is itself off the grid keeps the attachment on its body.
        let off_grid = Wire::segment(6, Point::new(0, 13), Point::new(40, 13));
        let pointer = viewport.schematic_to_screen(Point::new(17, 13));
        let hit = nearest_wire_screen_hit(&viewport, &[off_grid], pointer, 4.0, Some(10)).unwrap();
        assert_eq!(hit.attachment, Some(Point::new(20, 13)));

        // An endpoint the pointer is nearer to beats the grid point.
        let stub = Wire::segment(7, Point::new(3, 0), Point::new(3, 4));
        let pointer = viewport.schematic_to_screen(Point::new(3, 4)) + Vec2::new(0.0, 6.0);
        let hit = nearest_wire_screen_hit(&viewport, &[stub], pointer, 8.0, Some(10)).unwrap();
        assert_eq!(hit.attachment, Some(Point::new(3, 4)));

        // A diagonal conductor between grid points attaches on a grid point
        // of its own body.
        let diagonal = Wire::segment(8, Point::new(0, 0), Point::new(40, 40));
        let pointer = viewport.schematic_to_screen(Point::new(18, 18));
        let hit = nearest_wire_screen_hit(&viewport, &[diagonal], pointer, 4.0, Some(10)).unwrap();
        assert_eq!(hit.attachment, Some(Point::new(20, 20)));
    }

    #[test]
    fn wire_screen_hit_respects_viewport_origin_and_rejects_fractional_diagonal_attachment() {
        let viewport = Viewport {
            offset: Pos2::new(17.0, 23.0),
            zoom: 2.0,
            bounds: Rect::from_min_size(Pos2::new(100.0, 200.0), Vec2::splat(400.0)),
        };
        let horizontal = Wire::segment(5, Point::new(0, 10), Point::new(20, 10));
        let pointer = viewport.schematic_to_screen(Point::new(7, 10)) + Vec2::new(0.0, 2.0);

        let hit = nearest_wire_screen_hit(&viewport, &[horizontal], pointer, 4.0, None).unwrap();
        assert_eq!(hit.wire_id, 5);
        assert_eq!(hit.attachment, Some(Point::new(7, 10)));

        let diagonal = Wire::segment(6, Point::new(0, 0), Point::new(2, 1));
        let fractional = viewport.schematic_to_screen(Point::origin()) + Vec2::new(2.0, 1.0);
        let hit = nearest_wire_screen_hit(&viewport, &[diagonal], fractional, 1.0, None).unwrap();
        assert_eq!(hit.wire_id, 6);
        assert_eq!(hit.attachment, None);
    }

    #[test]
    fn nearest_lattice_point_handles_extreme_diagonal_coordinates() {
        assert_eq!(
            rspice_design::schematic::bus::nearest_lattice_point_on_segment(
                Point::origin(),
                Point::new(i32::MIN, i32::MIN),
                Point::new(i32::MAX, i32::MAX),
            ),
            Point::origin()
        );
    }

    #[test]
    fn bus_hit_testing_uses_radius_projection_and_durable_tie_breaking() {
        let declaration = BusDeclaration::parse("DATA[7:0]").unwrap();
        let buses = vec![
            Bus::segment(
                9,
                Point::new(0, 0),
                Point::new(20, 0),
                Some(declaration.clone()),
            )
            .unwrap(),
            Bus::segment(4, Point::new(0, 0), Point::new(20, 0), Some(declaration)).unwrap(),
        ];

        assert_eq!(
            nearest_bus_hit(&buses, Point::new(7, 2), 2),
            Some(BusHit {
                bus_id: 4,
                point: Point::new(7, 0),
                segment_start: Point::new(0, 0),
                segment_end: Point::new(20, 0),
            })
        );
        assert!(nearest_bus_hit(&buses, Point::new(7, 2), 1).is_none());
    }

    #[test]
    fn bus_tap_hit_testing_covers_the_routed_path_and_breaks_ties_by_id() {
        let bus = Bus::segment(
            1,
            Point::new(0, 0),
            Point::new(20, 0),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        let make_tap = |id| {
            BusTap::new(
                id,
                &bus,
                Point::new(10, 0),
                Point::new(10, 10),
                BusSlice::parse("DATA[3]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap()
        };
        let taps = vec![make_tap(8), make_tap(3)];

        assert_eq!(bus_tap_at(&taps, Point::new(11, 6), 1), Some(3));
        assert_eq!(bus_tap_at(&taps, Point::new(12, 6), 1), None);
    }
}
