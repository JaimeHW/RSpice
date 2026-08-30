//! Canvas painting.
//!
//! Draws the schematic itself: the grid, wires, junctions, labels, and
//! placed instances, in the order that puts selection and highlight on top.

use egui::{Painter, Pos2, Rect, Stroke, Vec2};

use crate::state::{
    Bus, BusTap, Component, ComponentType, Point, PortDirection, PortSpec, ResolvedSymbolSource,
    SchematicProbe, Wire,
};
use crate::workbench::app_state::AppState;

use super::super::symbols::{SymbolLibrary, draw_baked};
use super::SchematicSymbolContext;
use super::resolved_symbol_render::{
    draw_resolved_symbol_with_visibility, resolved_symbol_world_bounds,
};
use super::viewport::Viewport;

const DEFAULT_WIRE_STROKE_WIDTH: f32 = 1.1;
const SELECTED_WIRE_STROKE_WIDTH: f32 = 2.0;
const HIGHLIGHTED_WIRE_STROKE_WIDTH: f32 = 2.0;
const BUS_CONDUCTOR_OFFSET: f32 = 4.0;
const DEFAULT_BUS_STROKE_WIDTH: f32 = 1.2;
const SELECTED_BUS_STROKE_WIDTH: f32 = 1.6;
const DEFAULT_BUS_TAP_STROKE_WIDTH: f32 = 2.0;
const SELECTED_BUS_TAP_STROKE_WIDTH: f32 = 2.4;
const PROBE_RADIUS: f32 = 9.0;
const PROBE_CROSSHAIR_HALF_SPAN: f32 = 13.0;

const _: [(); 1] = [(); (DEFAULT_WIRE_STROKE_WIDTH.to_bits() == 1.1f32.to_bits()) as usize];
const _: [(); 1] = [(); (SELECTED_WIRE_STROKE_WIDTH.to_bits() == 2.0f32.to_bits()) as usize];
const _: [(); 1] = [(); (HIGHLIGHTED_WIRE_STROKE_WIDTH.to_bits() == 2.0f32.to_bits()) as usize];

/// Draw a wire on the canvas
pub(super) fn draw_wire(
    painter: &Painter,
    viewport: &Viewport,
    wire: &Wire,
    selected: bool,
    highlight_color: Option<egui::Color32>,
) {
    // Wire is a polyline - draw all segments
    // Priority: selected > highlighted > default
    let palette = crate::ui::tokens::active_palette();
    let (color, width) = if selected {
        (palette.accent, SELECTED_WIRE_STROKE_WIDTH) // Accent for selected
    } else if let Some(color) = highlight_color {
        (color, HIGHLIGHTED_WIRE_STROKE_WIDTH)
    } else {
        (palette.wire, DEFAULT_WIRE_STROKE_WIDTH)
    };

    // Draw each segment of the wire polyline
    for segment in wire.points.windows(2) {
        let start = viewport.schematic_to_screen(segment[0]);
        let end = viewport.schematic_to_screen(segment[1]);
        painter.line_segment([start, end], Stroke::new(width * viewport.zoom, color));
    }
}

/// Draw a typed multi-conductor bus. Buses deliberately use the same
/// conductor color as scalar nets, with the mockup's three parallel strokes
/// so type remains legible in monochrome exports and color-vision variants.
pub(super) fn draw_bus(painter: &Painter, viewport: &Viewport, bus: &Bus, selected: bool) {
    let palette = crate::ui::tokens::active_palette();
    let color = if selected {
        palette.accent
    } else {
        palette.wire
    };
    let width = if selected {
        SELECTED_BUS_STROKE_WIDTH
    } else {
        DEFAULT_BUS_STROKE_WIDTH
    };
    let centerline: Vec<Pos2> = bus
        .points
        .iter()
        .map(|point| viewport.schematic_to_screen(*point))
        .collect();
    for offset in [-BUS_CONDUCTOR_OFFSET, 0.0, BUS_CONDUCTOR_OFFSET] {
        painter.add(egui::Shape::line(
            offset_polyline(&centerline, offset * viewport.zoom),
            Stroke::new(width * viewport.zoom, color),
        ));
    }

    if let (Some(declaration), Some(anchor)) = (&bus.declaration, bus.points.last()) {
        painter.text(
            viewport.schematic_to_screen(*anchor) + Vec2::new(8.0, -8.0),
            egui::Align2::LEFT_BOTTOM,
            declaration.to_string(),
            crate::ui::theme::mono(
                crate::ui::tokens::FS_0,
                crate::ui::theme::FontWeight::Medium,
            ),
            if selected {
                palette.accent
            } else {
                palette.net_label
            },
        );
    }
}

/// Draw a typed bus breakout. The selector is part of the durable electrical
/// intent and therefore appears beside the tap in the canvas and SVG.
pub(super) fn draw_bus_tap(painter: &Painter, viewport: &Viewport, tap: &BusTap, selected: bool) {
    let palette = crate::ui::tokens::active_palette();
    let color = if selected {
        palette.accent
    } else {
        palette.wire
    };
    let width = if selected {
        SELECTED_BUS_TAP_STROKE_WIDTH
    } else {
        DEFAULT_BUS_TAP_STROKE_WIDTH
    };
    let connection = viewport.schematic_to_screen(tap.connection_point);
    let route: Vec<Pos2> = crate::schematic::bus_geometry::bus_tap_route_points(tap)
        .into_iter()
        .map(|point| viewport.schematic_to_screen(point))
        .collect();
    painter.add(egui::Shape::line(
        route,
        Stroke::new(width * viewport.zoom, color),
    ));

    painter.text(
        connection + Vec2::new(5.0, -7.0),
        egui::Align2::LEFT_BOTTOM,
        tap.slice.to_string(),
        crate::ui::theme::mono(
            crate::ui::tokens::FS_0,
            crate::ui::theme::FontWeight::Medium,
        ),
        if selected {
            palette.accent
        } else {
            palette.net_label
        },
    );
}

/// Paint a retained probe flag using the upgraded mockup's circle, crosshair,
/// and exact source/reference label. The marker scales with the drawing so it
/// stays anchored to its authored schematic coordinate at every zoom.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ProbeVisualStatus {
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

pub(super) fn draw_probe(
    painter: &Painter,
    viewport: &Viewport,
    probe: &SchematicProbe,
    status: ProbeVisualStatus,
    selected: bool,
    hovered: bool,
) {
    let palette = crate::ui::tokens::active_palette();
    let status_color = match status {
        ProbeVisualStatus::Materialized => palette.ok,
        ProbeVisualStatus::Hidden => palette.text_faint,
        ProbeVisualStatus::Pending => palette.warn,
        ProbeVisualStatus::Unavailable => palette.err,
        ProbeVisualStatus::Disabled => palette.text_faint,
    };
    let center = viewport.schematic_to_screen(probe.position);
    let radius = PROBE_RADIUS * viewport.zoom;
    let half_span = PROBE_CROSSHAIR_HALF_SPAN * viewport.zoom;
    if selected {
        painter.circle_filled(center, radius + 4.0 * viewport.zoom, palette.accent);
    } else if hovered {
        painter.circle_stroke(
            center,
            radius + 4.0 * viewport.zoom,
            Stroke::new(1.5 * viewport.zoom, palette.accent),
        );
    }
    painter.circle_filled(center, radius, palette.canvas_bg);
    painter.circle_stroke(
        center,
        radius,
        Stroke::new(
            if selected { 2.5 } else { 2.0 } * viewport.zoom,
            status_color,
        ),
    );
    painter.line_segment(
        [
            center - Vec2::new(half_span, 0.0),
            center + Vec2::new(half_span, 0.0),
        ],
        Stroke::new(2.0 * viewport.zoom, status_color),
    );
    painter.line_segment(
        [
            center - Vec2::new(0.0, half_span),
            center + Vec2::new(0.0, half_span),
        ],
        Stroke::new(2.0 * viewport.zoom, status_color),
    );
    painter.text(
        center + Vec2::new(13.0, -9.0) * viewport.zoom,
        egui::Align2::LEFT_BOTTOM,
        format!("{}{}", probe.reference, status.suffix()),
        crate::ui::theme::mono(
            crate::ui::tokens::FS_0 * viewport.zoom,
            crate::ui::theme::FontWeight::Medium,
        ),
        status_color,
    );
}

/// Conservative zoom-independent authored bounds for fitting and culling.
/// The label font scales with the schematic, so its screen-space advance maps
/// to a stable world-space estimate just like component labels.
pub(super) fn probe_world_bounds(probe: &SchematicProbe) -> (Point, Point) {
    probe.world_bounds()
}

/// Resolve a retained probe marker in screen space. The minimum radius keeps
/// markers usable when zoomed far out while the upper range still follows the
/// authored drawing scale.
pub(super) fn probe_at_screen(
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
pub(super) struct WireScreenHit {
    pub(super) wire_id: u64,
    /// Exact integer schematic attachment when representable. A visual hit on
    /// a malformed/non-integral conductor deliberately has no attachment.
    pub(super) attachment: Option<Point>,
    distance_sq: f32,
    authored_index: usize,
}

/// Nearest conductor under a screen-space pointer, with an exact authored
/// attachment point only when that point is representable in integer
/// schematic coordinates.
pub(super) fn nearest_wire_screen_hit(
    viewport: &Viewport,
    wires: &[Wire],
    pointer: Pos2,
    radius: f32,
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
                let attachment = if a.y == b.y {
                    let x = ((closest.x - viewport.bounds.min.x - viewport.offset.x)
                        / viewport.zoom)
                        .round()
                        .clamp(a.x.min(b.x) as f32, a.x.max(b.x) as f32);
                    Some(Point::new(x as i32, a.y))
                } else if a.x == b.x {
                    let y = ((closest.y - viewport.bounds.min.y - viewport.offset.y)
                        / viewport.zoom)
                        .round()
                        .clamp(a.y.min(b.y) as f32, a.y.max(b.y) as f32);
                    Some(Point::new(a.x, y as i32))
                } else {
                    let world_x = ((closest.x - viewport.bounds.min.x - viewport.offset.x)
                        / viewport.zoom)
                        .round() as i32;
                    let world_y = ((closest.y - viewport.bounds.min.y - viewport.offset.y)
                        / viewport.zoom)
                        .round() as i32;
                    let candidate = Point::new(world_x, world_y);
                    wire.contains_point(candidate).then_some(candidate)
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
pub(super) struct BusHit {
    pub bus_id: u64,
    pub point: Point,
    pub segment_start: Point,
    pub segment_end: Point,
}

/// Find the nearest bus segment and project the cursor onto it. The search is
/// deterministic (distance then durable id), so overlapping malformed buses
/// never cause frame-to-frame target flicker.
pub(super) fn nearest_bus_hit(buses: &[Bus], requested: Point, radius: i32) -> Option<BusHit> {
    let radius_sq = i128::from(radius.max(0)).pow(2);
    buses
        .iter()
        .flat_map(|bus| {
            bus.points.windows(2).filter_map(move |segment| {
                let point = crate::state::nearest_lattice_point_on_segment(
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

pub(super) fn bus_tap_at(taps: &[BusTap], requested: Point, radius: i32) -> Option<u64> {
    let radius_sq = i128::from(radius.max(0)).pow(2);
    taps.iter()
        .filter_map(|tap| {
            crate::schematic::bus_geometry::bus_tap_route_points(tap)
                .windows(2)
                .map(|segment| {
                    let point = crate::state::nearest_lattice_point_on_segment(
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
pub(super) fn draw_component(
    painter: &Painter,
    viewport: &Viewport,
    component: &Component,
    selected: bool,
    symbol_library: Option<&SymbolLibrary>,
    symbol_context: &SchematicSymbolContext,
    parameter_labels: crate::state::SchematicParameterLabelVisibility,
) {
    // Component uses `pos` not `position`, `kind` not `component_type`
    let pos = viewport.schematic_to_screen(component.pos);
    let scale = viewport.zoom;

    // Grid lines now visible through components (no opaque background)

    let palette = crate::ui::tokens::active_palette();
    let outline_color = if selected {
        palette.accent // Accent for selected
    } else {
        palette.symbol
    };

    let stroke = Stroke::new(if selected { 1.5 } else { 1.0 } * scale, outline_color);

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
            port_symbol_stroke(stroke, scale, selected, component.port_spec().as_ref())
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
    if !symbol_drew_its_own_labels
        && parameter_labels != crate::state::SchematicParameterLabelVisibility::Hidden
    {
        draw_component_labels(painter, pos, scale, component, parameter_labels);
    }
}

pub(super) fn port_symbol_stroke(
    symbol_stroke: Stroke,
    scale: f32,
    selected: bool,
    spec: Option<&PortSpec>,
) -> Stroke {
    if spec.is_none_or(|port| port.vector().is_none()) {
        return symbol_stroke;
    }
    let width = if selected {
        SELECTED_BUS_STROKE_WIDTH
    } else {
        DEFAULT_BUS_STROKE_WIDTH
    };
    Stroke::new(width * scale, symbol_stroke.color)
}

#[expect(
    clippy::too_many_arguments,
    reason = "the overlay mirrors the symbol transform contract"
)]
pub(super) fn draw_port_direction_overlay(
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
        let point = crate::schematic::port_overlay::transform_point(
            point,
            rotation_degrees,
            mirror_h,
            mirror_v,
        );
        Pos2::new(pos.x + point.x * scale, pos.y + point.y * scale)
    };
    for segment in crate::schematic::port_overlay::direction_segments(direction) {
        painter.line_segment(
            [screen_point(segment.start), screen_point(segment.end)],
            stroke,
        );
    }
}

pub(super) fn draw_symbol_resolution_error(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    kind: ComponentType,
    reason: &str,
) {
    let color = egui::Color32::from_rgb(220, 70, 70);
    let half = 12.0 * scale.max(0.25);
    let rect = Rect::from_center_size(pos, Vec2::splat(half * 2.0));
    let stroke = Stroke::new(1.5 * scale.max(0.5), color);
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
pub(super) fn compatible_builtin_xspice_asset<'a>(
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
pub(super) fn draw_artwork_lead_extensions(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    component: &Component,
    stroke: Stroke,
) {
    for (edge, terminal) in component.artwork_lead_extensions() {
        let to_screen = |point: crate::state::Point| {
            let transformed = component.transform_point(point);
            Pos2::new(
                pos.x + transformed.x as f32 * scale,
                pos.y + transformed.y as f32 * scale,
            )
        };
        painter.line_segment([to_screen(edge), to_screen(terminal)], stroke);
    }
}

/// Smart label placement for components
///
/// Places name and value labels optimally based on:
/// - Component dimensions and aspect ratio
/// - Rotation (horizontal vs vertical orientation)
/// - Terminal positions (avoid overlapping terminals)
///
/// Matches commercial EDA tool behavior (Cadence Virtuoso style).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TerminalAxis {
    Horizontal,
    Vertical,
    Mixed,
}

#[derive(Debug, Clone, Copy)]
struct LabelLayout {
    name_pos: Pos2,
    name_align: egui::Align2,
    value_pos: Pos2,
    value_align: egui::Align2,
}

/// Infer the dominant terminal direction after mirror/rotation transforms.
/// This is more robust than width/height heuristics for square symbols
/// (e.g. capacitor). Works on the static offsets — mirroring never changes
/// an axis span and 90°/270° rotation just swaps the spans — so this runs
/// allocation-free every frame.
fn infer_terminal_axis(component: &Component) -> TerminalAxis {
    let offsets = component.kind.terminal_offsets();
    if offsets.len() < 2 {
        return TerminalAxis::Mixed;
    }

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for (_, p) in offsets {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
    }

    let mut span_x = max_x - min_x;
    let mut span_y = max_y - min_y;
    if component.rotation.is_vertical() {
        std::mem::swap(&mut span_x, &mut span_y);
    }

    // One-grid hysteresis to avoid ambiguous axis flapping near equal spans.
    const AXIS_HYSTERESIS: i32 = 10;
    if span_x > span_y + AXIS_HYSTERESIS {
        TerminalAxis::Horizontal
    } else if span_y > span_x + AXIS_HYSTERESIS {
        TerminalAxis::Vertical
    } else {
        TerminalAxis::Mixed
    }
}

fn compute_label_layout(pos: Pos2, scale: f32, component: &Component) -> LabelLayout {
    let (width, height) = component.symbol_dimensions();
    let is_rotated_vertical = component.rotation.is_vertical(); // R90 or R270

    // Determine effective dimensions after rotation.
    let (eff_w, eff_h) = if is_rotated_vertical {
        (height as f32, width as f32) // Swap for rotated
    } else {
        (width as f32, height as f32)
    };

    // Determine label placement based on component shape and transformed terminal orientation.
    // Horizontal terminal axis: labels above/below.
    // Vertical terminal axis: labels left/right.
    // Tall devices (BJTs, MOSFETs): name above, value on right side.
    let is_tall_device = matches!(
        component.kind,
        crate::state::ComponentType::NpnBjt
            | crate::state::ComponentType::PnpBjt
            | crate::state::ComponentType::NpnBjt4
            | crate::state::ComponentType::PnpBjt4
            | crate::state::ComponentType::NpnBjt5
            | crate::state::ComponentType::PnpBjt5
            | crate::state::ComponentType::Nmos
            | crate::state::ComponentType::Pmos
            | crate::state::ComponentType::Njfet
            | crate::state::ComponentType::Pjfet
            | crate::state::ComponentType::Nmesfet
            | crate::state::ComponentType::Pmesfet
            | crate::state::ComponentType::NVdmos
            | crate::state::ComponentType::PVdmos
            | crate::state::ComponentType::NmosSoi
            | crate::state::ComponentType::PmosSoi
    );

    // Calculate label margin from component edge.
    let margin = 4.0 * scale;
    let half_w = (eff_w / 2.0) * scale;
    let half_h = (eff_h / 2.0) * scale;
    let terminal_axis = infer_terminal_axis(component);

    if is_tall_device {
        LabelLayout {
            name_pos: Pos2::new(pos.x, pos.y - half_h - margin),
            name_align: egui::Align2::CENTER_BOTTOM,
            value_pos: Pos2::new(pos.x + half_w + margin, pos.y),
            value_align: egui::Align2::LEFT_CENTER,
        }
    } else {
        // Mixed axis falls back to geometric aspect ratio.
        let use_left_right = matches!(terminal_axis, TerminalAxis::Vertical)
            || (matches!(terminal_axis, TerminalAxis::Mixed) && eff_h > eff_w);

        if use_left_right {
            LabelLayout {
                name_pos: Pos2::new(pos.x - half_w - margin, pos.y),
                name_align: egui::Align2::RIGHT_CENTER,
                value_pos: Pos2::new(pos.x + half_w + margin, pos.y),
                value_align: egui::Align2::LEFT_CENTER,
            }
        } else {
            LabelLayout {
                name_pos: Pos2::new(pos.x, pos.y - half_h - margin),
                name_align: egui::Align2::CENTER_BOTTOM,
                value_pos: Pos2::new(pos.x, pos.y + half_h + margin),
                value_align: egui::Align2::CENTER_TOP,
            }
        }
    }
}

/// Quantize a zoom-scaled font size to quarter points: consecutive zoom
/// frames then reuse egui's cached galleys instead of re-shaping every
/// label on screen each frame (the dominant zoom-gesture cost).
fn quantize_font_size(size: f32) -> f32 {
    ((size * 4.0).round() * 0.25).max(1.0)
}

fn draw_component_labels(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    component: &Component,
    visibility: crate::state::SchematicParameterLabelVisibility,
) {
    // Skip labels for Ground (too small, clutters schematic)
    if matches!(component.kind, crate::state::ComponentType::Ground) {
        return;
    }

    // Below this size labels are unreadable smudge — commercial EDA tools
    // hide them; so do we, and zoomed-out paint cost drops with them.
    let name_size = quantize_font_size(10.0 * scale);
    if name_size < 4.0 {
        return;
    }
    let name_font = crate::ui::theme::sans(name_size, crate::ui::theme::FontWeight::Medium);
    let value_font = crate::ui::theme::sans(
        quantize_font_size(9.0 * scale),
        crate::ui::theme::FontWeight::Regular,
    );

    let layout = compute_label_layout(pos, scale, component);
    let palette = crate::ui::tokens::active_palette();

    // Draw component name (reference designator)
    if component.display_mode.show_name(visibility) && !component.name.is_empty() {
        painter.text(
            layout.name_pos,
            layout.name_align,
            &component.name,
            name_font,
            palette.text,
        );
    }

    // Draw component value
    let value_text = super::super::source_labels::component_value_label_cached(component);
    if component.display_mode.show_value(visibility) && !value_text.is_empty() {
        painter.text(
            layout.value_pos,
            layout.value_align,
            value_text.as_ref(),
            value_font,
            palette.text_dim,
        );
    }
}

/// Draw a junction (net connection point)
pub(super) fn draw_junction(
    painter: &Painter,
    viewport: &Viewport,
    position: Point,
    state: &AppState,
) {
    let pos = viewport.schematic_to_screen(position);
    let radius = 1.5 * viewport.zoom; // Match wire/symbol stroke width

    // Check if this junction is being hovered (for visual feedback)
    let is_hovered = state
        .dialogs
        .interaction
        .hover_wire_vertex
        .map(|(x, y)| x == position.x && y == position.y)
        .unwrap_or(false);
    let is_selected = state.schematic.selection.has_junction(position);

    let palette = crate::ui::tokens::active_palette();
    if is_hovered || is_selected {
        // Draw larger highlight ring when hovered
        let highlight_radius = radius * 2.5;
        painter.circle_stroke(
            pos,
            highlight_radius,
            Stroke::new(1.0 * viewport.zoom, palette.accent),
        );
    }

    painter.circle_filled(
        pos,
        radius,
        if is_selected {
            palette.accent
        } else {
            palette.wire
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{BusDeclaration, BusSlice, BusTapOrientation};
    use egui::{Context, Id, LayerId, Order, Shape};

    fn wire_stroke_width(selected: bool, highlighted: bool, zoom: f32) -> f32 {
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

        draw_wire(
            &painter,
            &viewport,
            &wire,
            selected,
            highlighted.then_some(crate::ui::tokens::active_palette().warn),
        );

        let mut widths = Vec::new();
        painter.for_each_shape(|shape| {
            if let Shape::LineSegment { stroke, .. } = &shape.shape {
                widths.push(stroke.width);
            }
        });

        assert_eq!(widths.len(), 1);
        widths[0]
    }

    #[test]
    fn wire_strokes_use_design_default_width_without_changing_state_widths() {
        let zoom = 2.0;

        assert!((wire_stroke_width(false, false, zoom) - 2.2).abs() < f32::EPSILON);
        assert!((wire_stroke_width(true, false, zoom) - 4.0).abs() < f32::EPSILON);
        assert!((wire_stroke_width(false, true, zoom) - 4.0).abs() < f32::EPSILON);
    }

    #[test]
    fn canonical_vector_port_body_and_overlay_use_bus_weight() {
        let scale = 1.5;
        let symbol = Stroke::new(scale, crate::ui::tokens::active_palette().symbol);
        let spec = |name: &str| PortSpec {
            name: name.to_owned(),
            direction: PortDirection::InOut,
        };
        let vector = spec("DATA[7:0]");

        assert_eq!(
            port_symbol_stroke(symbol, scale, false, Some(&vector)).width,
            DEFAULT_BUS_STROKE_WIDTH * scale
        );
        assert_eq!(
            port_symbol_stroke(symbol, scale, true, Some(&vector)).width,
            SELECTED_BUS_STROKE_WIDTH * scale
        );
        for scalar in ["EN", "DATA[3]", "bias_1"] {
            assert_eq!(
                port_symbol_stroke(symbol, scale, false, Some(&spec(scalar))),
                symbol,
                "{scalar}"
            );
        }
        assert_eq!(port_symbol_stroke(symbol, scale, false, None), symbol);
    }

    #[test]
    fn catalog_svg_is_used_only_when_its_leads_match_every_electrical_terminal() {
        let library = SymbolLibrary::load_embedded().expect("symbol library");
        let component = |model_type: &str| {
            let descriptor = crate::state::engine_only_xspice_devices()
                .iter()
                .find(|descriptor| descriptor.model_type == model_type)
                .expect("catalog descriptor");
            let binding =
                crate::state::builtin_xspice_library_binding(descriptor).expect("catalog binding");
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
        const ARTWORK_DEVICES: [&str; 23] = [
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
            "xfer",
        ];

        let library = SymbolLibrary::load_embedded().expect("symbol library");
        let mut matched = Vec::new();
        for descriptor in crate::state::engine_only_xspice_devices() {
            let Ok(binding) = crate::state::builtin_xspice_library_binding(descriptor) else {
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
                (crate::state::GENERATED_WIDTH, height),
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
        let descriptor = crate::state::engine_only_xspice_devices()
            .iter()
            .find(|descriptor| descriptor.model_type == "d_fdiv")
            .expect("catalog descriptor");
        let binding =
            crate::state::builtin_xspice_library_binding(descriptor).expect("catalog binding");
        let component = Component::new(1, ComponentType::CellInstance, Point::origin())
            .with_library_cell(binding);

        let (width, _) = component.symbol_dimensions();
        assert!(
            width > crate::state::GENERATED_WIDTH,
            "freq_in/freq_out do not fit the nominal body"
        );
        let extensions = component.artwork_lead_extensions();
        assert_eq!(extensions.len(), 2, "{extensions:?}");
        for (edge, terminal) in extensions {
            assert_eq!(edge.y, terminal.y, "a lead extension must run straight");
            assert_eq!(edge.x.abs(), crate::state::GENERATED_WIDTH / 2);
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
    fn wire_screen_hit_respects_viewport_origin_and_rejects_fractional_diagonal_attachment() {
        let viewport = Viewport {
            offset: Pos2::new(17.0, 23.0),
            zoom: 2.0,
            bounds: Rect::from_min_size(Pos2::new(100.0, 200.0), Vec2::splat(400.0)),
        };
        let horizontal = Wire::segment(5, Point::new(0, 10), Point::new(20, 10));
        let pointer = viewport.schematic_to_screen(Point::new(7, 10)) + Vec2::new(0.0, 2.0);

        let hit = nearest_wire_screen_hit(&viewport, &[horizontal], pointer, 4.0).unwrap();
        assert_eq!(hit.wire_id, 5);
        assert_eq!(hit.attachment, Some(Point::new(7, 10)));

        let diagonal = Wire::segment(6, Point::new(0, 0), Point::new(2, 1));
        let fractional = viewport.schematic_to_screen(Point::origin()) + Vec2::new(2.0, 1.0);
        let hit = nearest_wire_screen_hit(&viewport, &[diagonal], fractional, 1.0).unwrap();
        assert_eq!(hit.wire_id, 6);
        assert_eq!(hit.attachment, None);
    }

    #[test]
    fn nearest_lattice_point_handles_extreme_diagonal_coordinates() {
        assert_eq!(
            crate::state::nearest_lattice_point_on_segment(
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
