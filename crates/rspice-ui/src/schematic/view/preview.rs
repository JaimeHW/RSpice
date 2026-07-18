use egui::{Painter, Rect, Response, Stroke, Vec2};

use crate::common::app::AppState;
use crate::state::{Bus, BusTap, Component, ComponentType, Point, ResolvedCellSymbol, Tool};

use super::super::symbols::{SymbolLibrary, draw_symbol};
use super::SchematicSymbolContext;
use super::bus_interaction::resolve_bus_tap_candidate;
use super::coordinates::{screen_to_grid, screen_to_schematic, screen_to_wire_grid};
use super::drawing::{draw_bus, draw_bus_tap};
use super::resolved_symbol_render::draw_resolved_symbol;
use super::symbol_primitives::{
    draw_capacitor_symbol, draw_diode_symbol, draw_ground_symbol, draw_inductor_symbol,
    draw_isource_symbol, draw_nmos_symbol, draw_npn_symbol, draw_pmos_symbol, draw_pnp_symbol,
    draw_resistor_symbol, draw_vsource_symbol, rotation_to_index,
};
use super::viewport::Viewport;

const WIRE_PREVIEW_STROKE_WIDTH: f32 = 1.5;
const COMPONENT_PREVIEW_GHOST_ALPHA: f32 = 0.55;

pub(super) fn draw_interaction_previews(
    painter: &Painter,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
    symbol_library: Option<&SymbolLibrary>,
) {
    draw_bus_preview(painter, response, state, viewport);
    draw_wire_preview(painter, response, state, viewport, symbol_context);
    draw_bus_tap_preview(painter, response, state, viewport);
    draw_junction_preview(painter, response, state, viewport);
    draw_component_preview(
        painter,
        response,
        state,
        viewport,
        symbol_context,
        symbol_library,
    );
    draw_selection_rect(painter, state, viewport);
}

fn draw_junction_preview(
    painter: &Painter,
    response: &Response,
    state: &AppState,
    viewport: &Viewport,
) {
    if state.schematic.read_only || state.schematic.tool != Tool::Junction {
        return;
    }
    let Some(hover_pos) = response.hover_pos() else {
        return;
    };

    let requested = screen_to_wire_grid(viewport, state.schematic.grid_size, hover_pos);
    let candidate = state
        .schematic
        .nearest_junction_candidate(requested, state.schematic.grid_size);
    let preview = candidate.unwrap_or(requested);
    let pos = viewport.schematic_to_screen(preview);
    let palette = crate::ui::tokens::active_palette();
    let mixed_bus = candidate.is_some_and(|point| {
        state
            .schematic
            .buses
            .iter()
            .any(|bus| bus.contains_point(point))
    });
    let color = match candidate {
        Some(_) if mixed_bus => palette.err,
        Some(point) if state.schematic.has_junction(point) => palette.warn,
        Some(_) => palette.accent,
        None => palette.err,
    };
    let radius = (4.0 * viewport.zoom).max(3.0);
    painter.circle_stroke(pos, radius, Stroke::new(1.0, color));
    if !mixed_bus && candidate.is_some_and(|point| !state.schematic.has_junction(point)) {
        painter.circle_filled(pos, (1.75 * viewport.zoom).max(1.5), color);
    }
}

fn draw_bus_preview(
    painter: &Painter,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
) {
    if state.schematic.tool != Tool::Bus || !state.schematic.bus_drawing.active {
        return;
    }
    if let Some(hover) = response.hover_pos() {
        let position = screen_to_wire_grid(viewport, state.schematic.grid_size, hover);
        state.schematic.update_bus_preview(position);
    }

    let mut points = state.schematic.bus_drawing.points.clone();
    let preview = state.schematic.bus_drawing.preview_path();
    points.extend(preview.into_iter().skip(1));
    if points.len() < 2 {
        if let Some(start) = points.first() {
            painter.circle_stroke(
                viewport.schematic_to_screen(*start),
                (5.0 * viewport.zoom).max(3.0),
                Stroke::new(1.0, crate::ui::tokens::active_palette().accent),
            );
        }
        return;
    }
    let bus = Bus {
        id: 0,
        points,
        declaration: state.schematic.bus_drawing.declaration.clone(),
    };
    draw_bus(painter, viewport, &bus, true);
}

fn draw_bus_tap_preview(
    painter: &Painter,
    response: &Response,
    state: &AppState,
    viewport: &Viewport,
) {
    if state.schematic.read_only || state.schematic.tool != Tool::BusTap {
        return;
    }
    let Some(hover) = response.hover_pos() else {
        return;
    };
    let requested = screen_to_schematic(viewport, hover);
    let hit_radius = (6.0 / viewport.zoom.max(0.1)).ceil() as i32;
    match resolve_bus_tap_candidate(&state.schematic, requested, hit_radius) {
        Ok(candidate) => {
            let Some(pending) = state.schematic.pending_bus_tap.as_ref() else {
                return;
            };
            let tap = BusTap {
                id: 0,
                bus_id: candidate.bus_id,
                bus_point: candidate.bus_point,
                connection_point: candidate.connection_point,
                slice: pending.slice.clone(),
                orientation: candidate.orientation,
            };
            draw_bus_tap(painter, viewport, &tap, true);
        }
        Err(_) => {
            painter.circle_stroke(
                viewport.schematic_to_screen(requested),
                (4.0 * viewport.zoom).max(3.0),
                Stroke::new(1.0, crate::ui::tokens::active_palette().err),
            );
        }
    }
}

fn draw_wire_preview(
    painter: &Painter,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) {
    let wire_active = state.schematic.wire_drawing.active;

    if wire_active && let Some(hover_pos) = response.hover_pos() {
        let grid_pos = wire_preview_snap_position(
            state,
            symbol_context,
            screen_to_wire_grid(viewport, state.schematic.grid_size, hover_pos),
        );
        state.schematic.update_wire_preview(grid_pos);
    }

    if wire_active {
        let wire_points: Vec<Point> = state.schematic.wire_drawing.points.clone();
        let preview_pos_opt = state.schematic.wire_drawing.preview_pos;

        if !wire_points.is_empty() {
            let wire_color = crate::ui::tokens::active_palette().accent;
            let stroke = Stroke::new(WIRE_PREVIEW_STROKE_WIDTH * viewport.zoom, wire_color);

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
                    Stroke::new(
                        WIRE_PREVIEW_STROKE_WIDTH * viewport.zoom,
                        wire_color.gamma_multiply(0.6),
                    ),
                );
            }

            if let Some(start) = wire_points.first() {
                let start_screen = viewport.schematic_to_screen(*start);
                painter.circle_filled(start_screen, 4.0 * viewport.zoom, wire_color);
            }
        }
    }
}

fn wire_preview_snap_position(
    state: &AppState,
    symbol_context: &SchematicSymbolContext,
    grid_pos: Point,
) -> Point {
    state
        .schematic
        .snap_engine
        .find_snap_target_resolved(
            grid_pos,
            &state.schematic.components,
            &state.schematic.wires,
            &state.schematic.junctions,
            |component| symbol_context.resolved_symbol(component),
        )
        .snapped_position
}

fn pending_library_cell_preview<'a>(
    state: &AppState,
    symbol_context: &'a SchematicSymbolContext,
    grid_pos: Point,
) -> Option<(Component, &'a ResolvedCellSymbol)> {
    let binding = state.schematic.pending_library_cell.clone()?;
    let symbol = symbol_context.pending_library_symbol()?;
    let component = Component::new(0, ComponentType::CellInstance, grid_pos)
        .with_rotation(state.schematic.preview_rotation)
        .with_library_cell(binding);
    Some((component, symbol))
}

fn draw_component_preview(
    painter: &Painter,
    response: &Response,
    state: &AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
    symbol_library: Option<&SymbolLibrary>,
) {
    if !component_preview_enabled(state.schematic.read_only) {
        return;
    }

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
                .gamma_multiply(COMPONENT_PREVIEW_GHOST_ALPHA),
        );

        if component_type == ComponentType::CellInstance
            && let Some((preview_component, symbol)) =
                pending_library_cell_preview(state, symbol_context, grid_pos)
        {
            draw_resolved_symbol(
                painter,
                preview_pos,
                viewport.zoom,
                &preview_component,
                symbol,
                preview_stroke,
            );
            return;
        }

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
            painter.rect_stroke(rect, 2.0, preview_stroke, egui::StrokeKind::Inside);
        }
    }
}

fn component_preview_enabled(read_only: bool) -> bool {
    !read_only
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_preview_ghost_uses_design_alpha_and_hides_on_read_only() {
        assert!((COMPONENT_PREVIEW_GHOST_ALPHA - 0.55).abs() < f32::EPSILON);
        assert!(component_preview_enabled(false));
        assert!(!component_preview_enabled(true));
    }

    #[test]
    fn wire_preview_stroke_width_matches_live_preview_spec() {
        assert!((WIRE_PREVIEW_STROKE_WIDTH - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn wire_preview_snap_position_uses_resolved_cell_terminals() {
        let mut state = AppState::default();
        let mut binding = crate::state::LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[crate::state::PortSpec {
            name: "IN".to_string(),
            direction: crate::state::PortDirection::In,
        }]);
        state.schematic.components.push(
            crate::state::Component::new(
                1,
                ComponentType::CellInstance,
                crate::state::Point::new(40, 40),
            )
            .with_library_cell(binding),
        );
        let symbol_context = crate::schematic::view::SchematicSymbolContext::from_state(&state);
        let component = &state.schematic.components[0];
        let terminal =
            component.terminal_positions_resolved(symbol_context.resolved_symbol(component))[0].1;
        let near_terminal = crate::state::Point::new(terminal.x + 1, terminal.y);

        assert_eq!(
            wire_preview_snap_position(&state, &symbol_context, near_terminal),
            terminal
        );
    }

    #[test]
    fn pending_library_cell_preview_uses_selected_authored_symbol() {
        let mut state = AppState::default();
        let mut library = crate::state::Library::new("work");
        let mut cell = crate::state::Cell::new("amp");
        cell.add_view(crate::state::View::new(
            "schematic",
            crate::state::ViewType::Schematic,
        ));
        let mut symbol_view = crate::state::View::new("symbol", crate::state::ViewType::Symbol);
        crate::state::SymbolDocument {
            pins: vec![crate::state::SymbolPin::new(
                "OUT",
                crate::state::PortDirection::Out,
                Some(Point::new(40, 0)),
            )],
            ..crate::state::SymbolDocument::default()
        }
        .store_in_view(&mut symbol_view)
        .expect("symbol stores");
        cell.add_view(symbol_view);
        library.add_cell(cell);
        state.library_manager.add_library(library);

        let mut binding = crate::state::LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[crate::state::PortSpec {
            name: "OUT".to_owned(),
            direction: crate::state::PortDirection::Out,
        }]);
        state.schematic.pending_library_cell = Some(binding);
        state.schematic.preview_rotation = crate::state::Rotation::R90;
        let context = SchematicSymbolContext::from_state(&state);

        let (component, symbol) =
            pending_library_cell_preview(&state, &context, Point::new(100, 50))
                .expect("pending library cell has preview symbol");

        assert_eq!(component.kind, ComponentType::CellInstance);
        assert_eq!(component.pos, Point::new(100, 50));
        assert_eq!(component.rotation, crate::state::Rotation::R90);
        assert_eq!(
            component
                .library_cell
                .as_ref()
                .map(|binding| (binding.library.as_str(), binding.cell.as_str())),
            Some(("work", "amp"))
        );
        assert_eq!(symbol.connectable_pins().count(), 1);
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
        painter.rect_stroke(
            selection_rect,
            0.0,
            Stroke::new(1.0, accent),
            egui::StrokeKind::Inside,
        );
    }
}
