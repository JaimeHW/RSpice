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
mod status_bar;
mod symbol_primitives;
mod viewport;

use self::coordinates::viewport_from_state;
use self::interaction::handle_tool_interactions;
use self::navigation::handle_viewport_navigation;
use self::preview::draw_interaction_previews;
use self::scene::draw_scene;
use self::status_bar::draw_canvas_status_bar;

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

    draw_canvas_status_bar(&painter, available, state, response.hover_pos());
}
