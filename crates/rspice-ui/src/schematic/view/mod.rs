//! Schematic View for egui Application
//!
//! The main schematic canvas using egui's painter for vectorized rendering.
//! This will be optimized for 60fps with direct GPU rendering.

use std::collections::HashMap;

use egui::{Sense, Ui, WidgetInfo, WidgetType};

use crate::common::app::AppState;
use crate::state::{
    Component, ComponentType, Point, ResolvedCellSymbol, SchematicState, SymbolResolver,
};

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
use self::resolved_symbol_render::resolved_symbol_world_bounds;
use self::scene::draw_scene;

pub(crate) struct SchematicSymbolContext {
    resolved_by_component_id: HashMap<u64, ResolvedCellSymbol>,
    pending_library_symbol: Option<ResolvedCellSymbol>,
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
        let pending_library_symbol = state
            .schematic
            .pending_library_cell
            .as_ref()
            .and_then(|binding| resolver.resolve_binding(binding));

        Self {
            resolved_by_component_id,
            pending_library_symbol,
        }
    }

    pub(super) fn resolved_symbol(&self, component: &Component) -> Option<&ResolvedCellSymbol> {
        self.resolved_by_component_id.get(&component.id)
    }

    pub(super) fn pending_library_symbol(&self) -> Option<&ResolvedCellSymbol> {
        self.pending_library_symbol.as_ref()
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
            .find(|component| self.terminal_points(component).contains(&pos))
            .map(|component| component.id)
    }

    pub(super) fn component_at_resolved_symbol(
        &self,
        components: &[Component],
        pos: Point,
    ) -> Option<u64> {
        self.component_at_resolved_terminal(components, pos)
            .or_else(|| {
                components
                    .iter()
                    .map(|component| (component.id, self.component_bounds(component)))
                    .find(|(_, (min, max))| {
                        pos.x >= min.x && pos.x <= max.x && pos.y >= min.y && pos.y <= max.y
                    })
                    .map(|(id, _)| id)
            })
    }

    pub(super) fn component_bounds(&self, component: &Component) -> (Point, Point) {
        if let Some(symbol) = self.resolved_symbol(component)
            && let Some(bounds) = resolved_symbol_world_bounds(component, symbol)
        {
            return bounds;
        }
        let (min_x, min_y, max_x, max_y) = component.bounding_box();
        (Point::new(min_x, min_y), Point::new(max_x, max_y))
    }

    pub(super) fn content_bounds(
        &self,
        schematic: &SchematicState,
    ) -> Option<(i32, i32, i32, i32)> {
        if schematic.components.is_empty()
            && schematic.wires.is_empty()
            && schematic.junctions.is_empty()
        {
            return None;
        }

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        let mut include = |min: Point, max: Point| {
            min_x = min_x.min(min.x);
            min_y = min_y.min(min.y);
            max_x = max_x.max(max.x);
            max_y = max_y.max(max.y);
        };

        for component in &schematic.components {
            let (min, max) = self.component_bounds(component);
            include(min, max);
        }

        for wire in &schematic.wires {
            for point in &wire.points {
                include(*point, *point);
            }
        }

        for junction in &schematic.junctions {
            include(junction.pos, junction.pos);
        }

        Some((min_x, min_y, max_x, max_y))
    }

    pub(super) fn select_in_rect(
        &self,
        schematic: &mut SchematicState,
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
        add_to_selection: bool,
    ) -> usize {
        if !add_to_selection {
            schematic.selection.clear();
        }

        let mut count = 0;

        for component in &schematic.components {
            let (min, max) = self.component_bounds(component);
            if rects_intersect(min, max, min_x, min_y, max_x, max_y)
                && !schematic.selection.has_component(component.id)
            {
                schematic.selection.select_component(component.id);
                count += 1;
            }
        }

        for wire in &schematic.wires {
            let wire_in_rect = wire
                .points
                .iter()
                .any(|point| point_in_rect(*point, min_x, min_y, max_x, max_y));
            if wire_in_rect && !schematic.selection.has_wire(wire.id) {
                schematic.selection.select_wire(wire.id);
                count += 1;
            }
        }

        for junction in &schematic.junctions {
            if point_in_rect(junction.pos, min_x, min_y, max_x, max_y)
                && !schematic.selection.has_junction(junction.pos)
            {
                schematic.selection.select_junction(junction.pos);
                count += 1;
            }
        }

        count
    }
}

fn point_in_rect(point: Point, min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> bool {
    point.x >= min_x && point.x <= max_x && point.y >= min_y && point.y <= max_y
}

fn rects_intersect(min: Point, max: Point, min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> bool {
    max.x >= min_x && min.x <= max_x && max.y >= min_y && min.y <= max_y
}

fn refresh_symbol_context_after_interactions(
    state: &AppState,
    symbol_context: &mut SchematicSymbolContext,
    before_topology_version: u64,
) -> bool {
    if state.schematic.topology_version() == before_topology_version {
        return false;
    }
    *symbol_context = SchematicSymbolContext::from_state(state);
    true
}

fn schematic_accessibility_label(
    state: &AppState,
    platform: crate::workbench::commands::CommandPlatform,
    operating_system: egui::os::OperatingSystem,
) -> String {
    use crate::ui::accessibility::counted;
    use crate::workbench::commands::Command;
    let schematic = &state.schematic;
    let tool = if schematic.tool.is_place_tool() {
        format!("Place {}", schematic.tool.display_name())
    } else {
        schematic.tool.display_name().to_owned()
    };
    let shortcuts = crate::common::app::accessibility_shortcut_summary(
        state.ui.preferences.shortcuts(),
        platform,
        operating_system,
        &[
            Command::SelectTool,
            Command::PlaceWire,
            Command::PlaceProbe,
            Command::ZoomFit,
            Command::Cancel,
        ],
    );
    format!(
        "Schematic canvas. {}, {}, {}, {}; {}. Active tool: {}.{shortcuts}",
        counted(schematic.components.len(), "component", "components"),
        counted(schematic.wires.len(), "wire", "wires"),
        counted(schematic.junctions.len(), "junction", "junctions"),
        counted(schematic.net_labels.len(), "net label", "net labels"),
        counted(
            schematic.selection.count(),
            "item selected",
            "items selected"
        ),
        tool,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Cell, Library, LibraryCellInstance, PortDirection, PortSpec, SymbolDocument, SymbolPin,
        SymbolShape, View, ViewType,
    };

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    #[test]
    fn component_at_resolved_symbol_hits_authored_body_outside_generic_bounds() {
        let component = Component::new(1, ComponentType::CellInstance, Point::new(100, 50));
        let symbol = ResolvedCellSymbol::from_authored_document(
            SymbolDocument {
                body: vec![SymbolShape::Polyline {
                    points: vec![Point::new(80, -10), Point::new(120, 10)],
                    closed: false,
                }],
                pins: vec![SymbolPin::new(
                    "OUT",
                    PortDirection::Out,
                    Some(Point::new(120, 0)),
                )],
                ..SymbolDocument::default()
            },
            &[port("OUT", PortDirection::Out)],
        );
        let mut resolved_by_component_id = HashMap::new();
        resolved_by_component_id.insert(component.id, symbol);
        let context = SchematicSymbolContext {
            resolved_by_component_id,
            pending_library_symbol: None,
        };

        assert_eq!(
            context.component_at_resolved_symbol(&[component], Point::new(200, 50)),
            Some(1)
        );
    }

    #[test]
    fn content_bounds_include_authored_symbol_body() {
        let component = Component::new(1, ComponentType::CellInstance, Point::new(100, 50));
        let symbol = ResolvedCellSymbol::from_authored_document(
            SymbolDocument {
                body: vec![SymbolShape::Polyline {
                    points: vec![Point::new(80, -10), Point::new(120, 10)],
                    closed: false,
                }],
                pins: vec![SymbolPin::new(
                    "OUT",
                    PortDirection::Out,
                    Some(Point::new(120, 0)),
                )],
                ..SymbolDocument::default()
            },
            &[port("OUT", PortDirection::Out)],
        );
        let mut resolved_by_component_id = HashMap::new();
        resolved_by_component_id.insert(component.id, symbol);
        let context = SchematicSymbolContext {
            resolved_by_component_id,
            pending_library_symbol: None,
        };
        let mut schematic = SchematicState::default();
        schematic.components.push(component);

        assert_eq!(context.content_bounds(&schematic), Some((80, 10, 220, 90)));
    }

    #[test]
    fn select_in_rect_commits_authored_body_intersections() {
        let component = Component::new(1, ComponentType::CellInstance, Point::new(100, 50));
        let symbol = ResolvedCellSymbol::from_authored_document(
            SymbolDocument {
                body: vec![SymbolShape::Polyline {
                    points: vec![Point::new(80, -10), Point::new(120, 10)],
                    closed: false,
                }],
                pins: vec![SymbolPin::new(
                    "OUT",
                    PortDirection::Out,
                    Some(Point::new(120, 0)),
                )],
                ..SymbolDocument::default()
            },
            &[port("OUT", PortDirection::Out)],
        );
        let mut resolved_by_component_id = HashMap::new();
        resolved_by_component_id.insert(component.id, symbol);
        let context = SchematicSymbolContext {
            resolved_by_component_id,
            pending_library_symbol: None,
        };
        let mut schematic = SchematicState::default();
        schematic.components.push(component);

        let selected = context.select_in_rect(&mut schematic, 190, 40, 210, 60, false);

        assert_eq!(selected, 1);
        assert!(schematic.selection.has_component(1));
    }

    #[test]
    fn second_refresh_resolves_cell_added_after_initial_interaction_refresh() {
        let mut state = AppState::default();
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        SymbolDocument {
            pins: vec![SymbolPin::new(
                "OUT",
                PortDirection::Out,
                Some(Point::new(40, 0)),
            )],
            ..SymbolDocument::default()
        }
        .store_in_view(&mut symbol_view)
        .expect("symbol stores");
        cell.add_view(symbol_view);
        library.add_cell(cell);
        state.library_manager.add_library(library);

        let mut context = SchematicSymbolContext::from_state(&state);
        let before_interactions_topology = state.schematic.topology_version();
        assert!(!refresh_symbol_context_after_interactions(
            &state,
            &mut context,
            before_interactions_topology,
        ));

        // The context-menu pass runs after the tool-interaction refresh. A
        // duplicate/place action can therefore mutate topology between the
        // first refresh and painting.
        let before_context_menu_topology = state.schematic.topology_version();
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[port("OUT", PortDirection::Out)]);
        let id = state
            .schematic
            .add_library_cell_component(Point::new(100, 50), binding);

        let refreshed = refresh_symbol_context_after_interactions(
            &state,
            &mut context,
            before_context_menu_topology,
        );
        let component = state
            .schematic
            .components
            .iter()
            .find(|component| component.id == id)
            .expect("component placed");

        assert!(refreshed);
        assert!(context.resolved_symbol(component).is_some());
    }

    #[test]
    fn accessibility_label_summarizes_scene_selection_and_tool() {
        use crate::state::{Junction, NetLabel, Wire};

        let mut state = AppState::default();
        state.schematic.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::new(0, 0),
        ));
        state
            .schematic
            .wires
            .push(Wire::new(2, vec![Point::new(0, 0), Point::new(10, 0)]));
        state
            .schematic
            .junctions
            .push(Junction::new(3, Point::new(10, 0)));
        state
            .schematic
            .net_labels
            .push(NetLabel::new(4, Point::new(10, 0), "OUT"));
        state.schematic.selection.select_component(1);
        state.schematic.tool = crate::state::Tool::Wire;

        let label = schematic_accessibility_label(
            &state,
            crate::workbench::commands::CommandPlatform::Desktop,
            egui::os::OperatingSystem::Windows,
        );

        assert!(label.starts_with(
            "Schematic canvas. 1 component, 1 wire, 1 junction, 1 net label; 1 item selected."
        ));
        assert!(label.contains("Active tool: Wire."));
        assert!(label.contains("Escape: Cancel active command"));
    }

    #[test]
    fn schematic_canvas_has_no_static_shortcut_prose() {
        let source = include_str!("mod.rs");
        assert!(!source.contains(concat!("Use S", " for select")));
        assert!(!source.contains(concat!("Escape", " to cancel")));
    }
}

/// Render the schematic view (central canvas)
pub fn render_schematic_view(
    ui: &mut Ui,
    state: &mut AppState,
    symbol_library: Option<&SymbolLibrary>,
) {
    let available = ui.available_rect_before_wrap();
    let mut symbol_context = SchematicSymbolContext::from_state(state);

    if state.schematic.needs_fit {
        state.schematic.needs_fit = false;
        let bounds = symbol_context.content_bounds(&state.schematic);
        state.schematic.zoom_to_fit_bounds(
            bounds,
            available.width() as f64,
            available.height() as f64,
        );
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
    let before_interactions_topology = state.schematic.topology_version();
    // Right-click owns two meanings: finishing a live wire run (inside the
    // tool handler) and the context menu (here). Capture whether a run was
    // live before the tool handler so the click that finishes a wire can
    // never also open the menu.
    let wire_was_active = state.schematic.wire_drawing.active;
    handle_tool_interactions(ui, &response, state, &viewport, &symbol_context);
    refresh_symbol_context_after_interactions(
        state,
        &mut symbol_context,
        before_interactions_topology,
    );
    let before_context_menu_topology = state.schematic.topology_version();
    context_menu::handle_context_menu(
        &response,
        state,
        &viewport,
        wire_was_active,
        &symbol_context,
    );
    refresh_symbol_context_after_interactions(
        state,
        &mut symbol_context,
        before_context_menu_topology,
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

    // Report the cursor position in grid units; the workbench status bar shows it.
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
    state.ui.canvas_hover = response
        .hover_pos()
        .map(|cursor| to_grid_units(cursor, state));
    state.ui.canvas_view_center = Some(to_grid_units(available.center(), state));

    let shortcut_platform = crate::common::app::runtime_command_platform(ui.ctx());
    let operating_system = ui.ctx().os();
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Image,
            ui.is_enabled(),
            schematic_accessibility_label(state, shortcut_platform, operating_system),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Canvas);
    });
    crate::common::app::report_engineering_canvas_focus(
        &response,
        state.workspace.active_view_type(),
    );
    crate::ui::theme::paint_focus_ring(ui, &response, available);
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
