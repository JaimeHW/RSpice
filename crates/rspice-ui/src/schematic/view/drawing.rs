use egui::{Painter, Pos2, Rect, Stroke, Vec2};

use crate::common::app::AppState;
use crate::state::{Component, ComponentType, Point, Wire};

use super::super::symbols::{SymbolLibrary, draw_symbol};
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
    state: &AppState,
) {
    // Wire is a polyline - draw all segments
    // Priority: selected > highlighted > default
    let (color, width) = if selected {
        (state.theme.accent, 2.0) // Accent for selected
    } else if highlighted {
        (crate::ui::tokens::active_palette().warn, 2.0) // Net highlight (cross-probe)
    } else {
        (state.theme.wire_default, 1.0)
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
    state: &AppState,
    symbol_library: Option<&SymbolLibrary>,
) {
    // Component uses `pos` not `position`, `kind` not `component_type`
    let pos = viewport.schematic_to_screen(component.pos);
    let scale = viewport.zoom;

    // Grid lines now visible through components (no opaque background)

    let outline_color = if selected {
        state.theme.accent // Accent for selected
    } else {
        state.theme.component_outline
    };

    let stroke = Stroke::new(if selected { 1.5 } else { 1.0 } * scale, outline_color);

    // SVG symbols expect rotation in degrees (0, 90, 180, 270)
    // Procedural drawing uses rotation_to_delta() which expects index (0-3)
    let rotation_degrees = component.rotation.degrees();
    let rotation_index = rotation_to_index(component.rotation);

    // Try to use SVG symbol if available
    let svg_rendered = if let Some(library) = symbol_library {
        if let Some((symbol, adjusted_rotation)) = library.get_with_rotation_variant(
            component.kind,
            rotation_degrees,
            component.symbol_variant.as_deref(),
        ) {
            // Note: SVG symbol is being used (removed per-frame logging)
            draw_symbol(
                painter,
                symbol,
                pos,
                scale,
                adjusted_rotation,
                component.mirror_h,
                component.mirror_v,
                stroke,
            );
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
    draw_component_labels(painter, pos, scale, component, state);
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
/// This is more robust than width/height heuristics for square symbols (e.g. capacitor).
fn infer_terminal_axis(component: &Component) -> TerminalAxis {
    let terminals = component.terminal_positions();
    if terminals.len() < 2 {
        return TerminalAxis::Mixed;
    }

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for (_, p) in terminals {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
    }

    let span_x = max_x - min_x;
    let span_y = max_y - min_y;

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

fn draw_component_labels(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    component: &Component,
    state: &AppState,
) {
    // Skip labels for Ground (too small, clutters schematic)
    if matches!(component.kind, crate::state::ComponentType::Ground) {
        return;
    }

    let name_font = egui::FontId::proportional(10.0 * scale);
    let value_font = egui::FontId::proportional(9.0 * scale);

    let layout = compute_label_layout(pos, scale, component);

    // Draw component name (reference designator)
    if !component.name.is_empty() {
        painter.text(
            layout.name_pos,
            layout.name_align,
            &component.name,
            name_font,
            state.theme.text_primary,
        );
    }

    // Draw component value
    let value_text = super::super::source_labels::component_value_label(component);
    if !value_text.is_empty() {
        painter.text(
            layout.value_pos,
            layout.value_align,
            value_text.as_ref(),
            value_font,
            state.theme.text_secondary,
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

    if is_hovered {
        // Draw larger highlight ring when hovered
        let highlight_radius = radius * 2.5;
        painter.circle_stroke(
            pos,
            highlight_radius,
            Stroke::new(1.0 * viewport.zoom, state.theme.accent),
        );
    }

    painter.circle_filled(pos, radius, state.theme.wire_default);
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
