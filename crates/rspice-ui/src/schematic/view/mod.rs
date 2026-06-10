//! Schematic View for egui Application
//!
//! The main schematic canvas using egui's painter for vectorized rendering.
//! This will be optimized for 60fps with direct GPU rendering.

use egui::{Sense, Ui};

use crate::common::app::AppState;

use super::symbols::SymbolLibrary;

mod coordinates;
mod drawing;
mod grid;
mod interaction;
mod navigation;
mod preview;
mod scene;
mod symbol_primitives;
mod viewport;

use self::coordinates::viewport_from_state;
use self::interaction::handle_tool_interactions;
use self::navigation::handle_viewport_navigation;
use self::preview::draw_interaction_previews;
use self::scene::draw_scene;

/// Render the schematic view (central canvas)
pub fn render_schematic_view(
    ui: &mut Ui,
    state: &mut AppState,
    symbol_library: Option<&SymbolLibrary>,
) {
    let available = ui.available_rect_before_wrap();

    if state.schematic.needs_fit {
        state.schematic.needs_fit = false;
        state
            .schematic
            .zoom_to_fit(available.width() as f64, available.height() as f64);
    }

    let response = ui.allocate_rect(available, Sense::click_and_drag());
    let painter = ui.painter_at(available);
    let viewport = viewport_from_state(state, available);

    // Refresh the frame-coherent canvas cache (culling bounds + hover
    // hit-test index) before painting and interaction use it.
    state.schematic.ensure_canvas_cache();

    draw_scene(&painter, available, &viewport, state, symbol_library);
    handle_viewport_navigation(ui, &response, available, state);

    let tool_viewport = viewport_from_state(state, available);
    handle_tool_interactions(ui, &response, state, &tool_viewport);
    draw_interaction_previews(
        &painter,
        &response,
        state,
        &tool_viewport,
        &viewport,
        symbol_library,
    );

    // Report the cursor position in grid units; the shell status bar shows it.
    state.shell.canvas_hover = response.hover_pos().map(|cursor| {
        let grid = f64::from(state.schematic.grid_size.max(1));
        let x = ((f64::from(cursor.x - available.min.x)) - state.schematic.pan.0)
            / state.schematic.zoom
            / grid;
        let y = ((f64::from(cursor.y - available.min.y)) - state.schematic.pan.1)
            / state.schematic.zoom
            / grid;
        (x, y)
    });
}

/// Paint a component symbol centered in `rect` — used by the component
/// browser's preview pane. Pure presentation: no state access.
pub fn draw_symbol_preview(
    painter: &egui::Painter,
    rect: egui::Rect,
    kind: crate::state::ComponentType,
    color: egui::Color32,
) {
    let stroke = egui::Stroke::new(1.6, color);
    preview::draw_procedural_component_preview(painter, kind, rect.center(), 0.9, 0, stroke);
}
