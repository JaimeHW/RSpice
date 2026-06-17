use egui::{Painter, Rect, Stroke};

use crate::common::app::AppState;
use crate::state::Point;

use super::super::symbols::SymbolLibrary;
use super::SchematicSymbolContext;
use super::drawing::{draw_component, draw_junction, draw_wire};
use super::grid::draw_grid;
use super::viewport::Viewport;

/// Culling margin in world units: symbols extend up to ~40 units from their
/// anchor and labels overhang further; generous slack keeps pop-in impossible
/// while still rejecting everything genuinely off-screen.
const CULL_MARGIN: f32 = 160.0;

pub(super) fn draw_scene(
    painter: &Painter,
    available: Rect,
    viewport: &Viewport,
    state: &AppState,
    symbol_library: Option<&SymbolLibrary>,
    symbol_context: &SchematicSymbolContext,
) {
    painter.rect_filled(
        available,
        0.0,
        crate::ui::tokens::active_palette().canvas_bg,
    );
    draw_grid(painter, available, state);

    // First-run guidance: an empty sheet says what to do next instead of
    // presenting a silent dot field.
    if state.schematic.components.is_empty() && state.schematic.wires.is_empty() {
        draw_empty_hint(painter, available);
    }

    let preview_bounds = if state.schematic.selection_rect.is_active() {
        let (min_x, min_y, max_x, max_y) = state.schematic.selection_rect.bounds();
        Some((min_x, min_y, max_x, max_y))
    } else {
        None
    };

    // Viewport culling: only elements whose bounds intersect the visible
    // world rect are transformed and tessellated.
    let (wx0, wy0, wx1, wy1) = viewport.visible_world_rect(CULL_MARGIN);
    let cache = state.schematic.canvas_cache();

    for (index, wire) in state.schematic.wires.iter().enumerate() {
        if let Some((min, max)) = cache.and_then(|c| c.wire_bounds.get(index)) {
            if (max.x as f32) < wx0
                || (min.x as f32) > wx1
                || (max.y as f32) < wy0
                || (min.y as f32) > wy1
            {
                continue;
            }
        }
        let mut is_selected = state.schematic.selection.wires.contains(&wire.id);

        if !is_selected && let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
            is_selected = wire
                .points
                .iter()
                .any(|p| p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y);
        }

        let is_highlighted = state.schematic.net_highlight.is_wire_highlighted(wire.id);
        draw_wire(painter, viewport, wire, is_selected, is_highlighted);
    }

    for component in &state.schematic.components {
        let (cx, cy) = (component.pos.x as f32, component.pos.y as f32);
        if cx < wx0 || cx > wx1 || cy < wy0 || cy > wy1 {
            continue;
        }
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
            symbol_library,
            symbol_context,
        );
    }

    for junction in &state.schematic.junctions {
        let (jx, jy) = (junction.pos.x as f32, junction.pos.y as f32);
        if jx < wx0 || jx > wx1 || jy < wy0 || jy > wy1 {
            continue;
        }
        draw_junction(painter, viewport, junction.pos, state);
    }

    if let Some((hx, hy)) = state.dialogs.interaction.hover_wire_vertex {
        let hover_pos = Point::new(hx, hy);
        let is_junction = match cache {
            Some(cache) => cache.junctions.contains(&hover_pos),
            None => state.schematic.junctions.iter().any(|j| j.pos == hover_pos),
        };
        if !is_junction {
            let pos = viewport.schematic_to_screen(hover_pos);
            let radius = 3.0 * viewport.zoom;
            painter.circle_stroke(
                pos,
                radius,
                Stroke::new(
                    1.0 * viewport.zoom,
                    crate::ui::tokens::active_palette().accent,
                ),
            );
        }
    }

    // Check results last — violation badges annotate everything below.
    super::violations::draw_violation_markers(painter, viewport, state);
}

/// Centered get-started hint for an empty sheet.
fn draw_empty_hint(painter: &Painter, available: Rect) {
    use crate::ui::theme::{self, FontWeight};

    let palette = crate::ui::tokens::active_palette();
    let center = available.center();
    painter.text(
        center - egui::vec2(0.0, 22.0),
        egui::Align2::CENTER_CENTER,
        "Empty schematic",
        theme::sans(15.0, FontWeight::Medium),
        palette.text_dim,
    );
    painter.text(
        center + egui::vec2(0.0, 2.0),
        egui::Align2::CENTER_CENTER,
        "Pick a part from the left panel, or press Shift+R · C · L · V · G to place one",
        theme::sans(12.0, FontWeight::Regular),
        palette.text_faint,
    );
    painter.text(
        center + egui::vec2(0.0, 22.0),
        egui::Align2::CENTER_CENTER,
        "File ▸ Open example loads a ready-to-run circuit",
        theme::sans(12.0, FontWeight::Regular),
        palette.text_faint,
    );
}
