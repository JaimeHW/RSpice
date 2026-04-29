use egui::{Color32, Painter, Rect, Stroke};

use crate::common::app::AppState;
use crate::state::Point;

use super::super::symbols::SymbolLibrary;
use super::drawing::{draw_component, draw_junction, draw_wire};
use super::grid::draw_grid;
use super::viewport::Viewport;

pub(super) fn draw_scene(
    painter: &Painter,
    available: Rect,
    viewport: &Viewport,
    state: &AppState,
    symbol_library: Option<&SymbolLibrary>,
) {
    painter.rect_filled(available, 0.0, state.theme.canvas_bg);
    draw_grid(painter, available, state);

    let preview_bounds = if state.schematic.selection_rect.is_active() {
        let (min_x, min_y, max_x, max_y) = state.schematic.selection_rect.bounds();
        Some((min_x, min_y, max_x, max_y))
    } else {
        None
    };

    for wire in &state.schematic.wires {
        let mut is_selected = state.schematic.selection.wires.contains(&wire.id);

        if !is_selected && let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
            is_selected = wire
                .points
                .iter()
                .any(|p| p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y);
        }

        let is_highlighted = state.schematic.net_highlight.is_wire_highlighted(wire.id);
        draw_wire(painter, viewport, wire, is_selected, is_highlighted, state);
    }

    for component in &state.schematic.components {
        let mut is_selected = state.schematic.selection.components.contains(&component.id);

        if !is_selected && let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
            is_selected = component.pos.x >= min_x
                && component.pos.x <= max_x
                && component.pos.y >= min_y
                && component.pos.y <= max_y;
        }

        draw_component(
            painter,
            viewport,
            component,
            is_selected,
            state,
            symbol_library,
        );
    }

    for junction in &state.schematic.junctions {
        draw_junction(painter, viewport, junction.pos, state);
    }

    if let Some((hx, hy)) = state.dialogs.interaction.hover_wire_vertex {
        let hover_pos = Point::new(hx, hy);
        let is_junction = state.schematic.junctions.iter().any(|j| j.pos == hover_pos);
        if !is_junction {
            let pos = viewport.schematic_to_screen(hover_pos);
            let radius = 3.0 * viewport.zoom;
            painter.circle_stroke(
                pos,
                radius,
                Stroke::new(1.0 * viewport.zoom, Color32::from_rgb(100, 200, 255)),
            );
        }
    }
}
