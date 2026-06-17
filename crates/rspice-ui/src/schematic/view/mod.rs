//! Schematic View for egui Application
//!
//! The main schematic canvas using egui's painter for vectorized rendering.
//! This will be optimized for 60fps with direct GPU rendering.

use std::collections::HashMap;

use egui::{Sense, Ui};

use crate::common::app::AppState;
use crate::state::{Component, ComponentType, Point, ResolvedCellSymbol, SymbolResolver};

use super::symbols::SymbolLibrary;

mod context_menu;
mod coordinates;
mod drawing;
mod grid;
mod interaction;
mod navigation;
mod preview;
pub(crate) mod resolved_symbol_render;
mod scene;
mod symbol_primitives;
mod viewport;
pub(crate) mod violations;

use self::coordinates::viewport_from_state;
use self::interaction::handle_tool_interactions;
use self::navigation::handle_viewport_navigation;
use self::preview::draw_interaction_previews;
use self::scene::draw_scene;

pub(crate) struct SchematicSymbolContext {
    resolved_by_component_id: HashMap<u64, ResolvedCellSymbol>,
}

impl SchematicSymbolContext {
    pub(crate) fn from_state(state: &AppState) -> Self {
        let resolver =
            SymbolResolver::new(&state.library_manager, &state.workspace.schematic_buffers);
        let resolved_by_component_id = state
            .schematic
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::CellInstance)
            .filter_map(|component| {
                let binding = component.library_cell.as_ref()?;
                let resolved = resolver.resolve_binding(binding)?;
                Some((component.id, resolved))
            })
            .collect();

        Self {
            resolved_by_component_id,
        }
    }

    pub(super) fn resolved_symbol(&self, component: &Component) -> Option<&ResolvedCellSymbol> {
        self.resolved_by_component_id.get(&component.id)
    }

    pub(crate) fn terminal_points(&self, component: &Component) -> Vec<Point> {
        component
            .terminal_positions_resolved(self.resolved_symbol(component))
            .into_iter()
            .map(|(_, position)| position)
            .collect()
    }

    pub(super) fn component_at_resolved_terminal(
        &self,
        components: &[Component],
        pos: Point,
    ) -> Option<u64> {
        components
            .iter()
            .find(|component| {
                self.terminal_points(component)
                    .iter()
                    .any(|terminal_pos| *terminal_pos == pos)
            })
            .map(|component| component.id)
    }
}

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
    if let Some(target) = state.schematic.center_request.take() {
        state
            .schematic
            .center_view_on(target, available.width() as f64, available.height() as f64);
    }

    let response = ui.allocate_rect(available, Sense::click_and_drag());
    let painter = ui.painter_at(available);

    // Input first, painting second. Pan/zoom and tool edits apply BEFORE
    // the camera is built and the scene is painted — the old order drew
    // last frame's state, so the canvas trailed the cursor by a full
    // frame during pans and drags.
    handle_viewport_navigation(ui, &response, available, state);
    let viewport = viewport_from_state(state, available, ui.ctx().pixels_per_point());
    let symbol_context = SchematicSymbolContext::from_state(state);
    // Right-click owns two meanings: finishing a live wire run (inside the
    // tool handler) and the context menu (here). Capture whether a run was
    // live before the tool handler so the click that finishes a wire can
    // never also open the menu.
    let wire_was_active = state.schematic.wire_drawing.active;
    handle_tool_interactions(ui, &response, state, &viewport, &symbol_context);
    context_menu::handle_context_menu(
        &response,
        state,
        &viewport,
        wire_was_active,
        &symbol_context,
    );

    // Refresh the frame-coherent canvas cache (culling bounds + hover
    // hit-test index) after interactions may have edited topology.
    state.schematic.ensure_canvas_cache();

    draw_scene(
        &painter,
        available,
        &viewport,
        state,
        symbol_library,
        &symbol_context,
    );
    draw_interaction_previews(
        &painter,
        &response,
        state,
        &viewport,
        &symbol_context,
        symbol_library,
    );

    // Report the cursor position in grid units; the shell status bar shows it.
    let to_grid_units = |pos: egui::Pos2, state: &AppState| {
        let grid = f64::from(state.schematic.grid_size.max(1));
        let x = ((f64::from(pos.x - available.min.x)) - state.schematic.pan.0)
            / state.schematic.zoom
            / grid;
        let y = ((f64::from(pos.y - available.min.y)) - state.schematic.pan.1)
            / state.schematic.zoom
            / grid;
        (x, y)
    };
    state.shell.canvas_hover = response
        .hover_pos()
        .map(|cursor| to_grid_units(cursor, state));
    state.shell.canvas_view_center = Some(to_grid_units(available.center(), state));
}

/// Paint a component symbol centered in `rect` — used by the component
/// browser's preview pane. Pure presentation: no state access.
///
/// Uses the same SVG symbol the canvas renders (scaled to fit the rect);
/// procedural primitives are only the no-library fallback.
pub fn draw_symbol_preview(
    painter: &egui::Painter,
    rect: egui::Rect,
    kind: crate::state::ComponentType,
    color: egui::Color32,
    symbol_library: Option<&crate::schematic::symbols::SymbolLibrary>,
) {
    let stroke = egui::Stroke::new(1.6, color);

    if let Some(library) = symbol_library
        && let Some((symbol, rotation)) = library.get_with_rotation_variant(kind, 0, None)
    {
        // Fit the symbol's grid-unit box into the preview rect.
        let fit = ((rect.width() - 12.0) / symbol.target_width.max(0.001))
            .min((rect.height() - 8.0) / symbol.target_height.max(0.001));
        crate::schematic::symbols::draw_symbol(
            painter,
            symbol,
            rect.center(),
            fit,
            rotation,
            false,
            false,
            stroke,
        );
        return;
    }

    preview::draw_procedural_component_preview(painter, kind, rect.center(), 0.9, 0, stroke);
}
