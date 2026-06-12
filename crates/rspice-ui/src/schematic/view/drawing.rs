use egui::{Painter, Pos2, Rect, Stroke, Vec2};

use crate::common::app::AppState;
use crate::state::{Component, ComponentType, Point, Wire};

use super::super::symbols::{SymbolLibrary, draw_baked};
use super::symbol_primitives::{
    draw_capacitor_symbol, draw_cccs_symbol, draw_ccvs_symbol, draw_diode_symbol,
    draw_ground_symbol, draw_inductor_symbol, draw_isource_symbol, draw_nmos_symbol,
    draw_npn_symbol, draw_pmos_symbol, draw_pnp_symbol, draw_resistor_symbol, draw_vccs_symbol,
    draw_vcvs_symbol, draw_vsource_symbol, rotation_to_index,
};
use super::viewport::Viewport;

/// Draw a wire on the canvas
pub(super) fn draw_wire(
    painter: &Painter,
    viewport: &Viewport,
    wire: &Wire,
    selected: bool,
    highlighted: bool, // Net highlighting - orange color for connected net
) {
    // Wire is a polyline - draw all segments
    // Priority: selected > highlighted > default
    let palette = crate::ui::tokens::active_palette();
    let (color, width) = if selected {
        (palette.accent, 2.0) // Accent for selected
    } else if highlighted {
        (palette.warn, 2.0) // Net highlight (cross-probe)
    } else {
        (palette.wire, 1.0)
    };

    // Draw each segment of the wire polyline
    for segment in wire.points.windows(2) {
        let start = viewport.schematic_to_screen(segment[0]);
        let end = viewport.schematic_to_screen(segment[1]);
        painter.line_segment([start, end], Stroke::new(width * viewport.zoom, color));
    }
}

/// Draw a component on the canvas
pub(super) fn draw_component(
    painter: &Painter,
    viewport: &Viewport,
    component: &Component,
    selected: bool,
    symbol_library: Option<&SymbolLibrary>,
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

    // SVG symbols expect rotation in degrees (0, 90, 180, 270)
    // Procedural drawing uses rotation_to_delta() which expects index (0-3)
    let rotation_degrees = component.rotation.degrees();
    let rotation_index = rotation_to_index(component.rotation);

    // Try to use SVG symbol if available — via the library's baked
    // (pre-flattened, pre-transformed) geometry: per frame this is one
    // multiply-add per vertex instead of bezier + trig per vertex.
    let svg_rendered = if let Some(library) = symbol_library {
        if let Some((symbol, adjusted_rotation)) = library.get_with_rotation_variant(
            component.kind,
            rotation_degrees,
            component.symbol_variant.as_deref(),
        ) {
            let baked = library.baked(
                symbol,
                adjusted_rotation,
                component.mirror_h,
                component.mirror_v,
            );
            draw_baked(painter, &baked, pos, scale, stroke);
            true
        } else {
            // No SVG available for this component type, will use procedural fallback
            false
        }
    } else {
        // Symbol library not available, procedural fallback will be used
        false
    };

    // Fall back to procedural drawing if SVG not available
    if !svg_rendered {
        // Using procedural drawing fallback
        match component.kind {
            ComponentType::Resistor => {
                draw_resistor_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Capacitor => {
                draw_capacitor_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Inductor => {
                draw_inductor_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::VoltageSource => {
                draw_vsource_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::CurrentSource => {
                draw_isource_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Ground => {
                draw_ground_symbol(painter, pos, scale, stroke);
            }
            ComponentType::Port => {
                draw_port_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Diode => {
                draw_diode_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Nmos => {
                draw_nmos_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Pmos => {
                draw_pmos_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::NpnBjt => {
                draw_npn_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::PnpBjt => {
                draw_pnp_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Vcvs => {
                draw_vcvs_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Vccs => {
                draw_vccs_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Ccvs => {
                draw_ccvs_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::Cccs => {
                draw_cccs_symbol(painter, pos, scale, rotation_index, stroke);
            }
            ComponentType::CellInstance => {
                draw_cell_instance_symbol(painter, pos, scale, rotation_index, component, stroke);
            }
            _ => {
                // Generic component: draw a rectangle
                let rect = Rect::from_center_size(pos, Vec2::splat(30.0 * scale));
                painter.rect_stroke(rect, 2.0, stroke);
            }
        }
    }

    // Smart label placement based on component type, rotation, and dimensions
    // Commercial EDA tools (Cadence Virtuoso) place labels to avoid overlapping
    // terminals and component body, with name/value on opposite sides
    draw_component_labels(painter, pos, scale, component);
}

/// Interface port: a flag whose tip is the attachment point at (-10, 0).
/// It must read as "this net leaves the cell", not as a floating label —
/// the filled tip distinguishes it from a net label at a glance.
fn draw_port_symbol(painter: &Painter, pos: Pos2, scale: f32, rotation_index: i32, stroke: Stroke) {
    let rotate = |dx: f32, dy: f32| -> Pos2 {
        let (x, y) = match rotation_index.rem_euclid(4) {
            0 => (dx, dy),
            1 => (-dy, dx),
            2 => (-dx, -dy),
            _ => (dy, -dx),
        };
        Pos2::new(pos.x + x * scale, pos.y + y * scale)
    };
    let outline = [
        rotate(-10.0, 0.0),
        rotate(-4.0, -6.0),
        rotate(10.0, -6.0),
        rotate(10.0, 6.0),
        rotate(-4.0, 6.0),
    ];
    for i in 0..outline.len() {
        painter.line_segment([outline[i], outline[(i + 1) % outline.len()]], stroke);
    }
    painter.circle_filled(rotate(-10.0, 0.0), 1.6 * scale, stroke.color);
}

/// Hierarchical cell instance: a block body with pin stubs matching the
/// declared terminal offsets, and the master cell's name inside — the
/// symbol must read as "descend into me", not as an anonymous square.
fn draw_cell_instance_symbol(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    rotation_index: i32,
    component: &Component,
    stroke: Stroke,
) {
    // Local geometry per symbol_dimensions (60 × 40): body ±20 × ±15,
    // pin stubs out to the ±30 terminals.
    let rotate = |dx: f32, dy: f32| -> Pos2 {
        let (x, y) = match rotation_index.rem_euclid(4) {
            0 => (dx, dy),
            1 => (-dy, dx),
            2 => (-dx, -dy),
            _ => (dy, -dx),
        };
        Pos2::new(pos.x + x * scale, pos.y + y * scale)
    };

    let corners = [
        rotate(-20.0, -15.0),
        rotate(20.0, -15.0),
        rotate(20.0, 15.0),
        rotate(-20.0, 15.0),
    ];
    for i in 0..4 {
        painter.line_segment([corners[i], corners[(i + 1) % 4]], stroke);
    }
    // Pin stubs with a small connection dot at the terminal.
    for side in [-1.0f32, 1.0] {
        painter.line_segment([rotate(side * 30.0, 0.0), rotate(side * 20.0, 0.0)], stroke);
        painter.circle_filled(rotate(side * 30.0, 0.0), 1.6 * scale, stroke.color);
    }

    // Master cell name inside the body, elided to fit; legible from ~60 %
    // zoom like the component labels.
    let cell_name = component
        .library_cell
        .as_ref()
        .map(|binding| binding.cell.as_str())
        .unwrap_or("cell");
    let font_size = 9.0 * scale;
    if font_size >= 4.0 {
        let max_chars = 8usize;
        let display: String = if cell_name.chars().count() > max_chars {
            let mut text: String = cell_name.chars().take(max_chars - 1).collect();
            text.push('…');
            text
        } else {
            cell_name.to_owned()
        };
        painter.text(
            pos,
            egui::Align2::CENTER_CENTER,
            display,
            crate::ui::theme::mono(font_size, crate::ui::theme::FontWeight::Medium),
            stroke.color,
        );
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
    let (width, height) = component.kind.symbol_dimensions();
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
            | crate::state::ComponentType::Nmos
            | crate::state::ComponentType::Pmos
            | crate::state::ComponentType::Njfet
            | crate::state::ComponentType::Pjfet
            | crate::state::ComponentType::NVdmos
            | crate::state::ComponentType::PVdmos
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
    if !component.name.is_empty() {
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
    if !value_text.is_empty() {
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

    let palette = crate::ui::tokens::active_palette();
    if is_hovered {
        // Draw larger highlight ring when hovered
        let highlight_radius = radius * 2.5;
        painter.circle_stroke(
            pos,
            highlight_radius,
            Stroke::new(1.0 * viewport.zoom, palette.accent),
        );
    }

    painter.circle_filled(pos, radius, palette.wire);
}

fn manhattan_distance(a: Point, b: Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

#[inline]
pub(super) fn nearest_terminal(
    terminals: &[(&'static str, Point)],
    target: Point,
) -> Option<(&'static str, Point)> {
    terminals
        .iter()
        .copied()
        .min_by_key(|(_, pos)| manhattan_distance(*pos, target))
}
