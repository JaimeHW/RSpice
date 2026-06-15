use egui::{Painter, Rect, Response, Stroke, Vec2};

use crate::common::app::AppState;
use crate::state::{ComponentType, Point, Tool};

use super::super::symbols::{SymbolLibrary, draw_symbol};
use super::coordinates::{screen_to_grid, screen_to_wire_grid};
use super::symbol_primitives::{
    draw_capacitor_symbol, draw_diode_symbol, draw_ground_symbol, draw_inductor_symbol,
    draw_isource_symbol, draw_nmos_symbol, draw_npn_symbol, draw_pmos_symbol, draw_pnp_symbol,
    draw_resistor_symbol, draw_vsource_symbol, rotation_to_index,
};
use super::viewport::Viewport;

pub(super) fn draw_interaction_previews(
    painter: &Painter,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_library: Option<&SymbolLibrary>,
) {
    draw_wire_preview(painter, response, state, viewport);
    draw_component_preview(painter, response, state, viewport, symbol_library);
    draw_selection_rect(painter, state, viewport);
}

fn draw_wire_preview(
    painter: &Painter,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
) {
    let wire_active = state.schematic.wire_drawing.active;

    if wire_active && let Some(hover_pos) = response.hover_pos() {
        let grid_pos = screen_to_wire_grid(viewport, state.schematic.grid_size, hover_pos);
        state.schematic.update_wire_preview(grid_pos);
    }

    if wire_active {
        let wire_points: Vec<Point> = state.schematic.wire_drawing.points.clone();
        let preview_pos_opt = state.schematic.wire_drawing.preview_pos;

        if !wire_points.is_empty() {
            let wire_color = crate::ui::tokens::active_palette().accent;
            let stroke = Stroke::new(1.0 * viewport.zoom, wire_color);

            for segment in wire_points.windows(2) {
                let p1 = viewport.schematic_to_screen(segment[0]);
                let p2 = viewport.schematic_to_screen(segment[1]);
                painter.line_segment([p1, p2], stroke);
            }

            if let Some(preview) = preview_pos_opt
                && let Some(last) = wire_points.last()
            {
                let p1 = viewport.schematic_to_screen(*last);
                let p2 = viewport.schematic_to_screen(preview);
                painter.line_segment(
                    [p1, p2],
                    Stroke::new(1.0 * viewport.zoom, wire_color.gamma_multiply(0.6)),
                );
            }

            if let Some(start) = wire_points.first() {
                let start_screen = viewport.schematic_to_screen(*start);
                painter.circle_filled(start_screen, 4.0 * viewport.zoom, wire_color);
            }
        }
    }
}

fn draw_component_preview(
    painter: &Painter,
    response: &Response,
    state: &AppState,
    viewport: &Viewport,
    symbol_library: Option<&SymbolLibrary>,
) {
    let preview_tool = state.schematic.tool;
    let preview_rotation_degrees = state.schematic.preview_rotation.degrees();
    let preview_rotation_index = rotation_to_index(state.schematic.preview_rotation);

    if let Tool::Place(component_type) = preview_tool
        && let Some(hover_pos) = response.hover_pos()
    {
        let grid_pos = screen_to_grid(viewport, state.schematic.grid_size, hover_pos);
        let preview_pos = viewport.schematic_to_screen(grid_pos);

        // Ghost the symbol in dimmed accent until it is placed.
        let preview_stroke = Stroke::new(
            1.0 * viewport.zoom,
            crate::ui::tokens::active_palette()
                .accent
                .gamma_multiply(0.7),
        );

        let svg_rendered = if let Some(library) = symbol_library {
            if let Some((symbol, adjusted_rotation)) =
                library.get_with_rotation_variant(component_type, preview_rotation_degrees, None)
            {
                draw_symbol(
                    painter,
                    symbol,
                    preview_pos,
                    viewport.zoom,
                    adjusted_rotation,
                    false,
                    false,
                    preview_stroke,
                );
                true
            } else {
                false
            }
        } else {
            false
        };

        if !svg_rendered {
            draw_procedural_component_preview(
                painter,
                component_type,
                preview_pos,
                viewport.zoom,
                preview_rotation_index,
                preview_stroke,
            );
        }
    }
}

pub(super) fn draw_procedural_component_preview(
    painter: &Painter,
    component_type: ComponentType,
    preview_pos: egui::Pos2,
    zoom: f32,
    rotation_index: i32,
    preview_stroke: Stroke,
) {
    match component_type {
        ComponentType::Resistor => {
            draw_resistor_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::Capacitor => {
            draw_capacitor_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::Inductor => {
            draw_inductor_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::VoltageSource => {
            draw_vsource_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::CurrentSource => {
            draw_isource_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::Ground => draw_ground_symbol(painter, preview_pos, zoom, preview_stroke),
        ComponentType::Diode => {
            draw_diode_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::Nmos => {
            draw_nmos_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::Pmos => {
            draw_pmos_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::NpnBjt => {
            draw_npn_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        ComponentType::PnpBjt => {
            draw_pnp_symbol(painter, preview_pos, zoom, rotation_index, preview_stroke)
        }
        _ => {
            let rect = Rect::from_center_size(preview_pos, Vec2::splat(30.0 * zoom));
            painter.rect_stroke(rect, 2.0, preview_stroke);
        }
    }
}

fn draw_selection_rect(painter: &Painter, state: &AppState, tool_viewport: &Viewport) {
    if state.schematic.selection_rect.is_active() {
        let (min_x, min_y, max_x, max_y) = state.schematic.selection_rect.bounds();
        let top_left = tool_viewport.schematic_to_screen(Point::new(min_x, min_y));
        let bottom_right = tool_viewport.schematic_to_screen(Point::new(max_x, max_y));

        let selection_rect = Rect::from_min_max(top_left, bottom_right);

        let accent = crate::ui::tokens::active_palette().accent;
        painter.rect_filled(selection_rect, 0.0, accent.gamma_multiply(0.14));
        painter.rect_stroke(selection_rect, 0.0, Stroke::new(1.0, accent));
    }
}
