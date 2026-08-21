//! Schematic View for egui Application
//!
//! The main schematic canvas using egui's painter for vectorized rendering.
//! This will be optimized for 60fps with direct GPU rendering.

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use egui::{Sense, Ui, WidgetInfo, WidgetType};

use crate::state::{
    Component, ComponentType, Point, ResolvedCellSymbol, SchematicState, SymbolResolver,
};
use crate::workbench::app_state::AppState;

use super::symbols::SymbolLibrary;

mod array_interaction;
mod bus_interaction;
mod context_menu;
mod coordinates;
mod design_notes;
mod documentation_shapes;
mod drawing;
pub(crate) mod drawing_sheet;
mod grid;
mod interaction;
mod keyboard_navigation;
mod mobile_controls;
mod navigation;
mod net_labels;
mod preview;
pub(crate) mod resolved_symbol_render;
mod scene;
pub(crate) mod selection_layout;
pub(crate) mod sheet_visibility;
mod shelf_drag;
mod snap_resolution;
mod stretch_interaction;
mod symbol_primitives;
mod viewport;
pub(crate) mod violations;

use self::coordinates::viewport_from_state;
use self::drawing_sheet::ActiveDrawingSheet;
use self::interaction::handle_tool_interactions;
use self::keyboard_navigation::handle_keyboard_object_navigation;
use self::navigation::handle_viewport_navigation;
use self::preview::{draw_interaction_previews, draw_shelf_drag_preview};
use self::resolved_symbol_render::resolved_symbol_world_bounds;
use self::scene::draw_scene;
use self::shelf_drag::{
    ShelfDropOutcome, can_accept_shelf_drop, commit_shelf_drop, handle_placement_transform_keys,
};

pub(crate) use self::interaction::{
    ensure_probe_visible_with_feedback, ensure_retained_probe_visible_with_feedback,
    toggle_probe_with_feedback,
};
pub(crate) use self::mobile_controls::show as show_mobile_canvas_controls;
pub(crate) use self::scene::wrapped_signal_name;
pub(crate) use self::sheet_visibility::retain_selection_on_active_sheet;
pub(crate) use self::shelf_drag::{
    SchematicShelfDragPayload, handle_pre_render_placement_transform,
};

const SCHEMATIC_CANVAS_INTERACTION_ID: &str = "rspice-schematic-canvas-interaction";

/// Whether the typed component-shelf payload is currently over the schematic
/// canvas rectangle captured during the previous settled frame.
///
/// Application shortcut resolution runs before this frame's canvas response
/// exists, so it deliberately uses egui's retained response for the stable
/// canvas id. This closes the single-frame ordering gap for R/M drag-ghost
/// transforms without accepting a drop outside the canvas.
pub(crate) fn shelf_drag_over_schematic_canvas(ctx: &egui::Context) -> bool {
    egui::DragAndDrop::has_payload_of_type::<SchematicShelfDragPayload>(ctx)
        && schematic_canvas_contains_pointer(ctx)
}

pub(super) fn schematic_canvas_contains_pointer(ctx: &egui::Context) -> bool {
    ctx.pointer_hover_pos().is_some_and(|pointer| {
        ctx.read_response(egui::Id::new(SCHEMATIC_CANVAS_INTERACTION_ID))
            .is_some_and(|response| response.rect.contains(pointer))
    })
}

/// Transfer keyboard ownership to the single active schematic canvas. Modal
/// schematic commands call this when they arm so arrow/Enter entry is
/// immediately available without consuming keys from unrelated controls.
pub(crate) fn request_schematic_canvas_focus(ctx: &egui::Context) {
    ctx.memory_mut(|memory| {
        memory.request_focus(egui::Id::new(SCHEMATIC_CANVAS_INTERACTION_ID));
    });
}

#[derive(Default)]
pub(crate) struct SchematicSymbolContext {
    resolved_by_component_id: HashMap<u64, ResolvedCellSymbol>,
    resolved_by_binding: Vec<(crate::state::LibraryCellInstance, ResolvedCellSymbol)>,
    pending_library_symbol: Option<ResolvedCellSymbol>,
    revision: u64,
}

impl SchematicSymbolContext {
    pub(crate) fn from_state(state: &AppState) -> Self {
        let resolver =
            SymbolResolver::new(&state.library_manager, &state.workspace.schematic_buffers);
        let mut resolved_by_component_id = HashMap::new();
        let mut resolved_by_binding = Vec::new();
        for component in state
            .schematic
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::CellInstance)
        {
            let Some(binding) = component.library_cell.as_ref() else {
                continue;
            };
            let Some(resolved) = resolver.resolve_binding(binding) else {
                continue;
            };
            resolved_by_component_id.insert(component.id, resolved.clone());
            if !resolved_by_binding
                .iter()
                .any(|(candidate, _)| candidate == binding)
            {
                resolved_by_binding.push((binding.clone(), resolved));
            }
        }
        let pending_library_symbol = state
            .schematic
            .pending_library_cell
            .as_ref()
            .and_then(|binding| resolver.resolve_binding(binding));
        let revision = symbol_context_revision(state);

        Self {
            resolved_by_component_id,
            resolved_by_binding,
            pending_library_symbol,
            revision,
        }
    }

    pub(super) fn resolved_symbol(&self, component: &Component) -> Option<&ResolvedCellSymbol> {
        self.resolved_by_component_id
            .get(&component.id)
            .or_else(|| {
                let binding = component.library_cell.as_ref()?;
                self.resolved_by_binding
                    .iter()
                    .find_map(|(candidate, symbol)| (candidate == binding).then_some(symbol))
            })
    }

    pub(super) fn pending_library_symbol(&self) -> Option<&ResolvedCellSymbol> {
        self.pending_library_symbol.as_ref()
    }

    pub(super) const fn revision(&self) -> u64 {
        self.revision
    }

    pub(crate) fn terminal_points(&self, component: &Component) -> Vec<Point> {
        component
            .terminal_positions_resolved(self.resolved_symbol(component))
            .into_iter()
            .map(|(_, position)| position)
            .collect()
    }

    pub(crate) fn named_terminal_points(&self, component: &Component) -> Vec<(String, Point)> {
        component
            .terminal_positions_resolved(self.resolved_symbol(component))
            .into_iter()
            .map(|(name, position)| (name.to_owned(), position))
            .collect()
    }

    /// Authoritative world bounds of the rendered instance, including custom
    /// symbol artwork. Geometry editors use this same extent so validation can
    /// never route through shapes that the user can see on the canvas.
    pub(crate) fn component_bounds_tuple(&self, component: &Component) -> (i32, i32, i32, i32) {
        let (min, max) = self.component_bounds(component);
        (min.x, min.y, max.x, max.y)
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
            && schematic.buses.is_empty()
            && schematic.bus_taps.is_empty()
            && schematic.junctions.is_empty()
            && schematic.net_labels.is_empty()
            && schematic.design_notes.is_empty()
            && schematic.documentation_shapes.is_empty()
            && schematic.probes.is_empty()
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

        for bus in &schematic.buses {
            for point in &bus.points {
                include(*point, *point);
            }
        }

        for tap in &schematic.bus_taps {
            for point in crate::schematic::bus_geometry::bus_tap_route_points(tap) {
                include(point, point);
            }
        }

        for junction in &schematic.junctions {
            include(junction.pos, junction.pos);
        }

        for label in &schematic.net_labels {
            let (min, max) = net_labels::world_bounds(label);
            include(min, max);
        }

        for note in &schematic.design_notes {
            let (min, max) = design_notes::conservative_world_bounds(note);
            include(min, max);
        }

        for shape in &schematic.documentation_shapes {
            let (min, max) = documentation_shapes::world_bounds(shape);
            include(min, max);
        }

        for probe in &schematic.probes {
            let (min, max) = drawing::probe_world_bounds(probe);
            include(min, max);
        }

        Some((min_x, min_y, max_x, max_y))
    }

    pub(super) fn select_in_rect(
        &self,
        schematic: &mut SchematicState,
        window: SelectionWindow,
        add_to_selection: bool,
    ) -> usize {
        let SelectionWindow {
            min_x,
            min_y,
            max_x,
            max_y,
            enclosed_only,
        } = window;
        if !add_to_selection {
            schematic.selection.clear();
        }

        let mut count = 0;

        for component in &schematic.components {
            let (min, max) = self.component_bounds(component);
            let matches = if enclosed_only {
                rect_contains_rect(min, max, min_x, min_y, max_x, max_y)
            } else {
                rects_intersect(min, max, min_x, min_y, max_x, max_y)
            };
            if matches && !schematic.selection.has_component(component.id) {
                schematic.selection.select_component(component.id);
                count += 1;
            }
        }

        for wire in &schematic.wires {
            let wire_in_rect = if enclosed_only {
                wire.points
                    .iter()
                    .all(|point| point_in_rect(*point, min_x, min_y, max_x, max_y))
            } else {
                wire.points.windows(2).any(|points| {
                    segment_intersects_rect(points[0], points[1], min_x, min_y, max_x, max_y)
                })
            };
            if wire_in_rect && !schematic.selection.has_wire(wire.id) {
                schematic.selection.select_wire(wire.id);
                count += 1;
            }
        }

        for bus in &schematic.buses {
            let bus_in_rect = if enclosed_only {
                bus.points
                    .iter()
                    .all(|point| point_in_rect(*point, min_x, min_y, max_x, max_y))
            } else {
                bus.points.windows(2).any(|points| {
                    segment_intersects_rect(points[0], points[1], min_x, min_y, max_x, max_y)
                })
            };
            if bus_in_rect && !schematic.selection.has_bus(bus.id) {
                schematic.selection.select_bus(bus.id);
                count += 1;
            }
        }

        for tap in &schematic.bus_taps {
            let route = crate::schematic::bus_geometry::bus_tap_route_points(tap);
            let tap_in_rect = if enclosed_only {
                route
                    .iter()
                    .all(|point| point_in_rect(*point, min_x, min_y, max_x, max_y))
            } else {
                route.windows(2).any(|segment| {
                    segment_intersects_rect(segment[0], segment[1], min_x, min_y, max_x, max_y)
                })
            };
            if tap_in_rect && !schematic.selection.has_bus_tap(tap.id) {
                schematic.selection.select_bus_tap(tap.id);
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

        for label in &schematic.net_labels {
            let (min, max) = net_labels::world_bounds(label);
            let matches = if enclosed_only {
                rect_contains_rect(min, max, min_x, min_y, max_x, max_y)
            } else {
                rects_intersect(min, max, min_x, min_y, max_x, max_y)
            };
            if matches && !schematic.selection.has_net_label(label.id) {
                schematic.selection.net_labels.insert(label.id);
                count += 1;
            }
        }

        for note in &schematic.design_notes {
            let (min, max) = design_notes::conservative_world_bounds(note);
            let matches = if enclosed_only {
                rect_contains_rect(min, max, min_x, min_y, max_x, max_y)
            } else {
                rects_intersect(min, max, min_x, min_y, max_x, max_y)
            };
            if matches && !schematic.selection.has_design_note(note.id) {
                schematic.selection.select_design_note(note.id);
                count += 1;
            }
        }

        for shape in &schematic.documentation_shapes {
            let matches = documentation_shapes::shape_intersects_rect(
                shape,
                min_x,
                min_y,
                max_x,
                max_y,
                enclosed_only,
            );
            if matches && !schematic.selection.has_documentation_shape(shape.id) {
                schematic.selection.select_documentation_shape(shape.id);
                count += 1;
            }
        }

        for probe in &schematic.probes {
            let (min, max) = probe.world_bounds();
            let matches = if enclosed_only {
                rect_contains_rect(min, max, min_x, min_y, max_x, max_y)
            } else {
                rects_intersect(min, max, min_x, min_y, max_x, max_y)
            };
            if matches && !schematic.selection.has_probe(probe.id) {
                schematic.selection.select_probe(probe.id);
                count += 1;
            }
        }

        count
    }
}

fn symbol_context_revision(state: &AppState) -> u64 {
    let mut hasher = DefaultHasher::new();
    state.library_manager.revision().hash(&mut hasher);
    state.workspace.schematic_buffers.len().hash(&mut hasher);
    let mut folded_xor = 0_u64;
    let mut folded_sum = 0_u64;
    for (key, buffer) in &state.workspace.schematic_buffers {
        let mut entry = DefaultHasher::new();
        key.hash(&mut entry);
        buffer.topology_version().hash(&mut entry);
        let entry = entry.finish();
        folded_xor ^= entry;
        folded_sum = folded_sum.wrapping_add(entry.rotate_left(17));
    }
    folded_xor.hash(&mut hasher);
    folded_sum.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SelectionWindow {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
    enclosed_only: bool,
}

impl SelectionWindow {
    pub(super) const fn new(
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
        enclosed_only: bool,
    ) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
            enclosed_only,
        }
    }
}

fn rect_contains_rect(
    min: Point,
    max: Point,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
) -> bool {
    min.x >= min_x && min.y >= min_y && max.x <= max_x && max.y <= max_y
}

fn point_in_rect(point: Point, min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> bool {
    point.x >= min_x && point.x <= max_x && point.y >= min_y && point.y <= max_y
}

fn rects_intersect(min: Point, max: Point, min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> bool {
    max.x >= min_x && min.x <= max_x && max.y >= min_y && min.y <= max_y
}

fn segment_intersects_rect(
    start: Point,
    end: Point,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
) -> bool {
    if point_in_rect(start, min_x, min_y, max_x, max_y)
        || point_in_rect(end, min_x, min_y, max_x, max_y)
    {
        return true;
    }

    let dx = f64::from(end.x) - f64::from(start.x);
    let dy = f64::from(end.y) - f64::from(start.y);
    let mut enter = 0.0_f64;
    let mut leave = 1.0_f64;
    for (p, q) in [
        (-dx, f64::from(start.x) - f64::from(min_x)),
        (dx, f64::from(max_x) - f64::from(start.x)),
        (-dy, f64::from(start.y) - f64::from(min_y)),
        (dy, f64::from(max_y) - f64::from(start.y)),
    ] {
        if p == 0.0 {
            if q < 0.0 {
                return false;
            }
            continue;
        }
        let ratio = q / p;
        if p < 0.0 {
            enter = enter.max(ratio);
        } else {
            leave = leave.min(ratio);
        }
        if enter > leave {
            return false;
        }
    }
    true
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

fn schematic_accessibility_label() -> &'static str {
    "Schematic canvas"
}

fn schematic_accessibility_description(
    state: &AppState,
    platform: crate::workbench::commands::vocabulary::CommandPlatform,
    operating_system: egui::os::OperatingSystem,
) -> String {
    use crate::ui::accessibility::counted;
    use crate::workbench::commands::vocabulary::Command;
    let schematic = &state.schematic;
    let tool = if schematic.tool.is_place_tool() {
        format!("Place {}", schematic.tool.display_name())
    } else {
        schematic.tool.display_name().to_owned()
    };
    let shortcuts = crate::workbench::app_state::accessibility_shortcut_summary(
        state.ui.preferences.shortcuts(),
        platform,
        operating_system,
        &[
            Command::SelectTool,
            Command::PlaceWire,
            Command::PlaceBus,
            Command::PlaceBusTap,
            Command::PlaceJunction,
            Command::PlaceProbe,
            Command::PlacePin,
            Command::PlaceText,
            Command::PlaceShape,
            Command::MoveSelection,
            Command::StretchSelection,
            Command::ArraySelection,
            Command::ZoomFit,
            Command::Cancel,
        ],
    );
    let traversal_instruction = if schematic.tool == crate::state::Tool::DocumentationShape {
        " Arrow keys move the exact shape cursor. Space places a point. Enter completes a legal polygon or places the current point. Backspace removes the last point. Escape cancels."
    } else if !state
        .ui
        .preferences
        .toggle(crate::workbench::TogglePreference::CanvasKeyboardNavigation)
        || !schematic_keyboard_navigation_has_objects(state)
    {
        ""
    } else {
        " Arrow keys select the nearest eligible schematic object in each direction."
    };
    format!(
        "{}, {}, {}, {}, {}, {}, {}, {}, {}.{traversal_instruction} Active tool: {}.{shortcuts}",
        counted(schematic.components.len(), "component", "components"),
        counted(schematic.wires.len(), "wire", "wires"),
        counted(schematic.buses.len(), "bus", "buses"),
        counted(schematic.bus_taps.len(), "bus tap", "bus taps"),
        counted(schematic.junctions.len(), "junction", "junctions"),
        counted(schematic.net_labels.len(), "net label", "net labels"),
        counted(schematic.design_notes.len(), "design note", "design notes"),
        counted(
            schematic.documentation_shapes.len(),
            "documentation shape",
            "documentation shapes"
        ),
        counted(schematic.probes.len(), "probe flag", "probe flags"),
        tool,
    )
}

fn schematic_selection_accessibility_status(state: &AppState) -> String {
    if let Some(component) = state.schematic.selection.single_component().and_then(|id| {
        state
            .schematic
            .components
            .iter()
            .find(|component| component.id == id)
    }) {
        let name = component.name.trim();
        let name = if name.is_empty() { "unnamed" } else { name };
        let value = component.value.trim();
        let value = if !value.is_empty() {
            value
        } else {
            component.library_cell.as_ref().map_or_else(
                || component.kind.display_name(),
                |binding| {
                    binding
                        .module_name
                        .as_deref()
                        .unwrap_or(binding.cell.as_str())
                },
            )
        };
        return format!("Selected instance {name}, {value}.");
    }

    if let Some(focus) = state
        .dialogs
        .interaction
        .schematic_keyboard_focus
        .filter(|focus| scene::keyboard_focus_matches_selection(state, *focus))
    {
        return format!("Selected {}.", schematic_keyboard_focus_label(state, focus));
    }

    match state.schematic.selection.count() {
        0 => "No schematic object selected.".to_owned(),
        1 => "One schematic object selected.".to_owned(),
        count => format!("{count} schematic objects selected."),
    }
}

fn schematic_keyboard_navigation_has_objects(state: &AppState) -> bool {
    let filter = state.ui.schematic_selection_filter;
    (filter.instances && !state.schematic.components.is_empty())
        || (filter.wires
            && (!state.schematic.wires.is_empty()
                || !state.schematic.buses.is_empty()
                || !state.schematic.bus_taps.is_empty()
                || !state.schematic.junctions.is_empty()))
        || (filter.labels && !state.schematic.net_labels.is_empty())
        || (filter.annotations
            && (!state.schematic.design_notes.is_empty()
                || !state.schematic.documentation_shapes.is_empty()
                || !state.schematic.probes.is_empty()))
}

fn schematic_keyboard_focus_label(
    state: &AppState,
    focus: crate::workbench::app_state::SchematicKeyboardFocus,
) -> String {
    use crate::workbench::app_state::SchematicKeyboardFocus;
    let id = match focus {
        SchematicKeyboardFocus::Component(id)
        | SchematicKeyboardFocus::Wire(id)
        | SchematicKeyboardFocus::Bus(id)
        | SchematicKeyboardFocus::BusTap(id)
        | SchematicKeyboardFocus::Junction(id)
        | SchematicKeyboardFocus::NetLabel(id)
        | SchematicKeyboardFocus::Probe(id)
        | SchematicKeyboardFocus::DesignNote(id)
        | SchematicKeyboardFocus::DocumentationShape(id) => id,
    };
    match focus {
        SchematicKeyboardFocus::Component(_) => format!("component {id}"),
        SchematicKeyboardFocus::Wire(_) => format!("wire {id}"),
        SchematicKeyboardFocus::Bus(_) => format!("bus {id}"),
        SchematicKeyboardFocus::BusTap(_) => format!("bus tap {id}"),
        SchematicKeyboardFocus::Junction(_) => format!("junction {id}"),
        SchematicKeyboardFocus::NetLabel(_) => state
            .schematic
            .net_labels
            .iter()
            .find(|label| label.id == id)
            .map_or_else(
                || format!("net label {id}"),
                |label| format!("net label {}", label.name),
            ),
        SchematicKeyboardFocus::Probe(_) => state
            .schematic
            .probes
            .iter()
            .find(|probe| probe.id == id)
            .map_or_else(
                || format!("probe {id}"),
                |probe| format!("probe {}", probe.reference),
            ),
        SchematicKeyboardFocus::DesignNote(_) => format!("design note {id}"),
        SchematicKeyboardFocus::DocumentationShape(_) => {
            format!("documentation shape {id}")
        }
    }
}

/// Why a result signal could not be located on the schematic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LocateSignalError {
    /// The signal is an expression or a device current, not a node voltage,
    /// so no single conductor carries it.
    NotANet,
    /// No retained cross-probe map matches the drawing as it stands, so the
    /// net's geometry is unknown.
    NoCurrentMap,
    /// The map is current but knows no conductor by that name.
    UnknownNet(String),
    /// The signal is read inside another instance than the one this tab is
    /// editing, so no conductor on screen carries it.
    OtherOccurrence(String),
}

impl LocateSignalError {
    pub(crate) fn message(&self, signal: &str) -> String {
        match self {
            Self::NotANet => {
                format!("{signal} is derived, not a node voltage — no single conductor carries it.")
            }
            Self::NoCurrentMap => {
                "The schematic changed since this result was produced; run again to cross-probe it."
                    .to_owned()
            }
            Self::UnknownNet(net) => {
                format!("The open sheet has no conductor named {net}.")
            }
            Self::OtherOccurrence(occurrence) => {
                format!(
                    "{signal} is read inside {occurrence}; descend into that instance to see it."
                )
            }
        }
    }
}

/// Select the conductor a result signal names, closing the probe loop from
/// the results workspace back to the drawing.
///
/// Fails closed rather than guessing: the retained cross-probe map must
/// belong to the open cell at its current topology, or the geometry it
/// holds describes a different drawing.
///
/// A trace name is an address, not a string: `V(x1.n1)` names the leaf `n1`
/// inside `/X1`, and only the leaf is drawn on a sheet. The scope is therefore
/// resolved against the tab's own occurrence and the leaf is what the geometry
/// is looked up by, so a node read in another instance says so instead of
/// selecting a same-named conductor here.
pub(crate) fn select_signal_conductor(
    state: &mut AppState,
    signal: &str,
) -> Result<String, LocateSignalError> {
    let (net, points) = locate_signal_conductor(state, signal)?;

    let wires: Vec<u64> = wires_touching(state, &points);
    state.schematic.selection.clear();
    for wire in &wires {
        state.schematic.selection.select_wire(*wire);
    }
    state
        .schematic
        .net_highlight
        .highlight_wires(wires.into_iter().collect());
    state.schematic.center_request = points
        .iter()
        .copied()
        .min_by_key(|point| (point.y, point.x));
    Ok(net)
}

/// Resolve one signal to the conductor geometry the open sheet draws for it.
///
/// The address rules live here so every caller reads the same map the same
/// way, whether it is locating one probed node or every node a failed run
/// named.
fn locate_signal_conductor(
    state: &AppState,
    signal: &str,
) -> Result<(String, Vec<crate::state::Point>), LocateSignalError> {
    let wrapped = wrapped_signal_name(signal, 'V').ok_or(LocateSignalError::NotANet)?;
    let target =
        crate::state::ProbeTarget::parse_legacy(wrapped).map_err(|_| LocateSignalError::NotANet)?;
    if !result_mapping_is_current(state) {
        return Err(LocateSignalError::NoCurrentMap);
    }
    let occurrence = state.workspace.occurrence_path();
    if target.scope.fold_key() != occurrence.fold_key() {
        return Err(LocateSignalError::OtherOccurrence(target.scope.to_string()));
    }
    // Report the net as the design spells it, not as the trace happened to.
    state
        .simulation
        .cross_probe
        .net_to_points
        .iter()
        .find(|(name, points)| name.eq_ignore_ascii_case(&target.leaf) && !points.is_empty())
        .map(|(name, points)| (name.clone(), points.clone()))
        .ok_or(LocateSignalError::UnknownNet(target.leaf.clone()))
}

/// Whether the retained cross-probe map belongs to the open cell as it is
/// drawn right now.
fn result_mapping_is_current(state: &AppState) -> bool {
    state.simulation.cross_probe.is_current_for(
        &state.workspace.active_view,
        state.schematic.topology_version(),
    )
}

fn wires_touching(state: &AppState, points: &[crate::state::Point]) -> Vec<u64> {
    state
        .schematic
        .wires
        .iter()
        .filter(|wire| points.iter().any(|point| wire.contains_point(*point)))
        .map(|wire| wire.id)
        .collect()
}

/// What a request to mark a failed run's objects could actually mark.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct FailureSiteSelection {
    /// Nets marked, spelled as the design spells them.
    pub(crate) nets: Vec<String>,
    /// Devices marked, spelled as the schematic spells them.
    pub(crate) devices: Vec<String>,
    /// Names the run attributed that this sheet does not draw — a node in
    /// another occurrence, or one the current drawing no longer carries.
    pub(crate) unlocated: Vec<String>,
}

impl FailureSiteSelection {
    pub(crate) fn is_empty(&self) -> bool {
        self.nets.is_empty() && self.devices.is_empty()
    }
}

/// Mark every design object a failed run named, and centre on the set.
///
/// Multi-object by construction. A convergence failure names the nodes it
/// could not settle; marking only the first would say the failure was about
/// that node, which is a different and false claim.
///
/// One refusal governs the whole request, because currency is a property of
/// the map and not of any one name in it. Names the current sheet does not
/// draw are reported rather than refused, so the caller can say how much of
/// the set it was able to show.
pub(crate) fn select_failure_sites(
    state: &mut AppState,
    nets: &[String],
    devices: &[String],
) -> Result<FailureSiteSelection, LocateSignalError> {
    if !result_mapping_is_current(state) {
        return Err(LocateSignalError::NoCurrentMap);
    }

    let mut selection = FailureSiteSelection::default();
    let mut wires: Vec<u64> = Vec::new();
    let mut points: Vec<crate::state::Point> = Vec::new();
    for net in nets {
        let signal = format!("V({net})");
        match locate_signal_conductor(state, &signal) {
            Ok((name, net_points)) => {
                wires.extend(wires_touching(state, &net_points));
                points.extend(net_points);
                selection.nets.push(name);
            }
            // The map is current — that was settled above — so a name that
            // does not resolve is one this sheet does not draw, not a reason
            // to abandon the names that do.
            Err(_) => selection.unlocated.push(net.clone()),
        }
    }

    let components: Vec<(u64, String, crate::state::Point)> = devices
        .iter()
        .filter_map(|device| {
            state
                .schematic
                .components
                .iter()
                .find(|component| component.spice_instance_name().eq_ignore_ascii_case(device))
                .map(|component| (component.id, component.spice_instance_name(), component.pos))
                .or_else(|| {
                    selection.unlocated.push(device.clone());
                    None
                })
        })
        .collect();

    state.schematic.selection.clear();
    for wire in &wires {
        state.schematic.selection.select_wire(*wire);
    }
    for (id, name, position) in components {
        state.schematic.selection.select_component(id);
        selection.devices.push(name);
        points.push(position);
    }
    state
        .schematic
        .net_highlight
        .highlight_wires(wires.into_iter().collect());
    state.schematic.center_request = points
        .iter()
        .copied()
        .min_by_key(|point| (point.y, point.x));
    Ok(selection)
}

/// Render the schematic view (central canvas)
pub fn render_schematic_view(
    ui: &mut Ui,
    state: &mut AppState,
    symbol_library: Option<&SymbolLibrary>,
) {
    let available = ui.available_rect_before_wrap();
    let mut symbol_context = SchematicSymbolContext::from_state(state);
    let drawing_sheet = ActiveDrawingSheet::resolve(state);

    if state.schematic.needs_drawing_sheet_fit {
        state.schematic.needs_drawing_sheet_fit = false;
        state.schematic.needs_fit = false;
        drawing_sheet.geometry.fit_view(
            &mut state.schematic,
            available.width() as f64,
            available.height() as f64,
        );
    } else if state.schematic.needs_fit {
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

    let response = ui.interact(
        available,
        egui::Id::new(SCHEMATIC_CANVAS_INTERACTION_ID),
        Sense::click_and_drag(),
    );
    if response.clicked() || response.secondary_clicked() || response.drag_started() {
        state.dialogs.interaction.schematic_keyboard_focus = None;
    }
    ui.advance_cursor_after_rect(available);
    let painter = ui.painter_at(available);

    // Input first, painting second. Pan/zoom and tool edits apply BEFORE
    // the camera is built and the scene is painted — the old order drew
    // last frame's state, so the canvas trailed the cursor by a full
    // frame during pans and drags.
    handle_viewport_navigation(ui, &response, available, state);
    let viewport = viewport_from_state(state, available, ui.ctx().pixels_per_point());
    let shelf_drag = response.dnd_hover_payload::<SchematicShelfDragPayload>();
    let shelf_drag_position = shelf_drag
        .as_ref()
        .and_then(|_| ui.ctx().pointer_hover_pos());
    let shelf_drag_over_canvas = shelf_drag.is_some() && shelf_drag_position.is_some();
    handle_placement_transform_keys(&response, state, shelf_drag_over_canvas);
    if let Some(payload) = shelf_drag.as_deref() {
        ui.ctx()
            .set_cursor_icon(if can_accept_shelf_drop(state, payload) {
                egui::CursorIcon::Copy
            } else {
                egui::CursorIcon::NoDrop
            });
    }
    let dropped_payload = response.dnd_release_payload::<SchematicShelfDragPayload>();
    let before_interactions_topology = state.schematic.topology_version();
    // Keep the pre-interaction route state available to the context-menu
    // layer for diagnostics. Secondary click is exclusively a context-menu
    // gesture; routes commit with Enter or primary double-click.
    let routing_was_active = state.schematic.wire_drawing.active
        || state.schematic.bus_drawing.active
        || !state
            .schematic
            .documentation_shape_drawing
            .points
            .is_empty();
    if let (Some(payload), Some(position)) = (dropped_payload.as_deref(), shelf_drag_position) {
        let drop_position =
            snap_resolution::resolve_grid_pointer(state, &viewport, position).snapped_position;
        match commit_shelf_drop(state, payload, drop_position) {
            ShelfDropOutcome::Placed => {
                state.ui.toasts.success(
                    ui.ctx(),
                    "Component placed",
                    format!(
                        "{} was placed at ({}, {}).",
                        payload.component_type().display_name(),
                        drop_position.x,
                        drop_position.y
                    ),
                );
            }
            ShelfDropOutcome::ReadOnly => {
                state.ui.toasts.warn_with_title(
                    ui.ctx(),
                    "Drop not permitted",
                    "The active schematic is read-only; no component was placed.",
                );
            }
            ShelfDropOutcome::RequiresConfiguration => {
                state.ui.toasts.warn_with_title(
                    ui.ctx(),
                    "Configuration required",
                    "Use Place pin or port to define the interface contract before placement.",
                );
            }
        }
    } else {
        handle_tool_interactions(ui, &response, state, &viewport, &symbol_context);
    }
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
        routing_was_active,
        &symbol_context,
    );
    handle_keyboard_object_navigation(&response, state, &symbol_context);
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
        &drawing_sheet,
    );
    draw_interaction_previews(
        &painter,
        &response,
        state,
        &viewport,
        &symbol_context,
        symbol_library,
    );
    if dropped_payload.is_none()
        && let (Some(payload), Some(position)) = (shelf_drag.as_deref(), shelf_drag_position)
        && can_accept_shelf_drop(state, payload)
    {
        draw_shelf_drag_preview(
            &painter,
            state,
            &viewport,
            payload,
            position,
            symbol_library,
        );
    }
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
    state.ui.canvas_hover = shelf_drag_position
        .or_else(|| response.hover_pos())
        .map(|cursor| to_grid_units(cursor, state));
    state.ui.canvas_view_center = Some(to_grid_units(available.center(), state));

    let shortcut_platform = crate::workbench::app_state::runtime_command_platform(ui.ctx());
    let operating_system = ui.ctx().os();
    let accessibility_label = schematic_accessibility_label();
    let accessibility_description =
        schematic_accessibility_description(state, shortcut_platform, operating_system);
    response.widget_info(|| {
        WidgetInfo::labeled(WidgetType::Image, ui.is_enabled(), accessibility_label)
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Canvas);
        node.set_label(accessibility_label);
        node.set_description(accessibility_description);
    });
    let selection_status = schematic_selection_accessibility_status(state);
    let selection_status_response = ui.interact(
        egui::Rect::from_min_size(available.min, egui::Vec2::splat(1.0)),
        response.id.with("selection-status"),
        egui::Sense::hover(),
    );
    ui.ctx()
        .accesskit_node_builder(selection_status_response.id, |node| {
            node.set_role(egui::accesskit::Role::Status);
            node.set_label(selection_status);
            node.set_live(egui::accesskit::Live::Polite);
        });
    // The canvas takes keyboard focus for tool, nudge, and routing shortcuts,
    // so it owes a visible focus ring like every other custom click target.
    // Painted last so the schematic artwork does not cover it.
    crate::ui::theme::paint_focus_ring(ui, &response, available);
    crate::workbench::app_state::report_engineering_canvas_focus(
        &response,
        state.workspace.active_view_type(),
    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Cell, Library, LibraryCellInstance, NetLabel, PortDirection, PortSpec, SymbolDocument,
        SymbolPin, SymbolShape, View, ViewType,
    };

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    fn state_with_probed_wire() -> AppState {
        let mut state = AppState::default();
        let a = Point::new(0, 0);
        let b = Point::new(40, 0);
        state
            .schematic
            .wires
            .push(crate::state::Wire::new(1, vec![a, b]));
        state.simulation.cross_probe.update(
            state.workspace.active_view.clone(),
            std::collections::HashMap::from([(a, "OUT".to_owned()), (b, "OUT".to_owned())]),
            std::collections::HashMap::from([("OUT".to_owned(), vec![a, b])]),
            std::collections::HashMap::new(),
            state.schematic.topology_version(),
        );
        state
    }

    #[test]
    fn locating_a_node_voltage_selects_and_highlights_its_conductor() {
        let mut state = state_with_probed_wire();

        let net = select_signal_conductor(&mut state, "V(out)").expect("net resolves");

        assert_eq!(net, "OUT");
        assert!(state.schematic.selection.wires.contains(&1));
        assert!(state.schematic.net_highlight.is_wire_highlighted(1));
        assert_eq!(state.schematic.center_request, Some(Point::new(0, 0)));
    }

    #[test]
    fn a_derived_signal_explains_itself_instead_of_selecting_geometry() {
        let mut state = state_with_probed_wire();

        let error = select_signal_conductor(&mut state, "V(out)-V(in)")
            .expect_err("an expression is not a net");

        assert_eq!(error, LocateSignalError::NotANet);
        assert!(error.message("V(out)-V(in)").contains("derived"));
        assert!(state.schematic.selection.is_empty());
    }

    #[test]
    fn a_result_from_a_different_drawing_never_selects_stale_geometry() {
        let mut state = state_with_probed_wire();
        // Any structural edit invalidates the retained point map.
        state.schematic.bump_topology_version();

        let error = select_signal_conductor(&mut state, "V(out)")
            .expect_err("the map is no longer current");

        assert_eq!(error, LocateSignalError::NoCurrentMap);
        assert!(state.schematic.selection.is_empty());
    }

    #[test]
    fn a_scoped_trace_name_locates_the_conductor_by_its_leaf() {
        let mut state = AppState::default();
        state.workspace.descend_into(
            "X1".to_owned(),
            crate::state::CellViewRef::new("user", "amp", "schematic"),
            ViewType::Schematic,
        );
        let a = Point::new(0, 0);
        let b = Point::new(40, 0);
        state
            .schematic
            .wires
            .push(crate::state::Wire::new(1, vec![a, b]));
        state.simulation.cross_probe.update(
            state.workspace.active_view.clone(),
            std::collections::HashMap::from([(a, "OUT".to_owned()), (b, "OUT".to_owned())]),
            std::collections::HashMap::from([("OUT".to_owned(), vec![a, b])]),
            std::collections::HashMap::new(),
            state.schematic.topology_version(),
        );

        let net =
            select_signal_conductor(&mut state, "V(x1.out)").expect("the leaf is on this sheet");

        assert_eq!(net, "OUT");
        assert!(state.schematic.selection.wires.contains(&1));
    }

    #[test]
    fn a_signal_read_in_another_instance_never_selects_a_same_named_conductor() {
        let mut state = state_with_probed_wire();

        let error = select_signal_conductor(&mut state, "V(x1.out)")
            .expect_err("this tab is editing the design root");

        assert_eq!(error, LocateSignalError::OtherOccurrence("/x1".to_owned()));
        assert!(error.message("V(x1.out)").contains("/x1"));
        assert!(state.schematic.selection.is_empty());
    }

    #[test]
    fn an_unknown_net_is_reported_by_name() {
        let mut state = state_with_probed_wire();

        let error = select_signal_conductor(&mut state, "V(missing)")
            .expect_err("no conductor carries this net");

        assert_eq!(error, LocateSignalError::UnknownNet("missing".to_owned()));
        assert!(error.message("V(missing)").contains("missing"));
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
            resolved_by_binding: Vec::new(),
            pending_library_symbol: None,
            revision: 0,
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
            resolved_by_binding: Vec::new(),
            pending_library_symbol: None,
            revision: 0,
        };
        let mut schematic = SchematicState::default();
        schematic.components.push(component);

        assert_eq!(context.content_bounds(&schematic), Some((80, 10, 220, 90)));
    }

    #[test]
    fn content_bounds_and_marquee_selection_include_net_label_text() {
        let mut schematic = SchematicState::default();
        let label = NetLabel::new(77, Point::new(100, 80), "afe_out");
        let (min, max) = net_labels::world_bounds(&label);
        schematic.net_labels.push(label);
        let context = SchematicSymbolContext::default();

        assert_eq!(
            context.content_bounds(&schematic),
            Some((min.x, min.y, max.x, max.y))
        );
        assert_eq!(
            context.select_in_rect(
                &mut schematic,
                SelectionWindow::new(min.x + 2, min.y + 2, max.x - 2, max.y - 2, false),
                false,
            ),
            1
        );
        assert_eq!(schematic.selection.single_net_label(), Some(77));
    }

    #[test]
    fn content_bounds_and_marquee_selection_include_design_note_text() {
        let mut schematic = SchematicState::default();
        let note = crate::state::DesignNote::new(
            78,
            Point::new(100, 80),
            crate::state::DesignNoteKind::PlainText,
            "Bias network\nKeep clear",
        )
        .unwrap();
        let (min, max) = design_notes::conservative_world_bounds(&note);
        schematic.design_notes.push(note);
        let context = SchematicSymbolContext::default();

        assert_eq!(
            context.content_bounds(&schematic),
            Some((min.x, min.y, max.x, max.y))
        );
        assert_eq!(
            context.select_in_rect(
                &mut schematic,
                SelectionWindow::new(min.x, min.y, max.x, max.y, false),
                false,
            ),
            1
        );
        assert_eq!(schematic.selection.single_design_note(), Some(78));
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
            resolved_by_binding: Vec::new(),
            pending_library_symbol: None,
            revision: 0,
        };
        let mut schematic = SchematicState::default();
        schematic.components.push(component);

        let selected = context.select_in_rect(
            &mut schematic,
            SelectionWindow::new(190, 40, 210, 60, false),
            false,
        );

        assert_eq!(selected, 1);
        assert!(schematic.selection.has_component(1));
    }

    #[test]
    fn enclosed_selection_rejects_partial_component_intersections() {
        let mut schematic = SchematicState::default();
        schematic.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::new(100, 50),
        ));
        let context = SchematicSymbolContext::default();

        assert_eq!(
            context.select_in_rect(
                &mut schematic,
                SelectionWindow::new(95, 45, 105, 55, true),
                false,
            ),
            0
        );
        assert!(!schematic.selection.has_component(1));
    }

    #[test]
    fn intersecting_selection_detects_wire_crossing_without_an_inside_vertex() {
        let mut schematic = SchematicState::default();
        schematic.add_wire(vec![Point::new(0, 50), Point::new(100, 50)]);
        let wire_id = schematic.wires[0].id;
        let context = SchematicSymbolContext::default();

        assert_eq!(
            context.select_in_rect(
                &mut schematic,
                SelectionWindow::new(40, 40, 60, 60, false),
                false,
            ),
            1
        );
        assert!(schematic.selection.has_wire(wire_id));
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
        let mut generated_preview = component.clone();
        generated_preview.id = u64::MAX;
        assert!(
            context.resolved_symbol(&generated_preview).is_some(),
            "generated array members resolve authored symbols by immutable binding"
        );
    }

    #[test]
    fn symbol_context_revision_tracks_library_and_schematic_interface_sources() {
        let mut state = AppState::default();
        let baseline = symbol_context_revision(&state);

        state.library_manager.add_library(Library::new("work"));
        let library_changed = symbol_context_revision(&state);
        assert_ne!(library_changed, baseline);

        let buffer = state
            .workspace
            .schematic_buffers
            .entry("work/amp/schematic".to_owned())
            .or_default();
        buffer.add_component(ComponentType::Port, Point::origin());
        assert_ne!(symbol_context_revision(&state), library_changed);
    }

    #[test]
    fn accessibility_description_summarizes_scene_and_tool_without_selection_churn() {
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
        state.schematic.design_notes.push(
            crate::state::DesignNote::new(
                5,
                Point::new(20, 10),
                crate::state::DesignNoteKind::PlainText,
                "Bias network",
            )
            .unwrap(),
        );
        state.schematic.selection.select_component(1);
        state.schematic.tool = crate::state::Tool::Wire;

        let description = schematic_accessibility_description(
            &state,
            crate::workbench::commands::vocabulary::CommandPlatform::Desktop,
            egui::os::OperatingSystem::Windows,
        );

        assert!(description.starts_with(
            "1 component, 1 wire, 0 buses, 0 bus taps, 1 junction, 1 net label, 1 design note, 0 documentation shapes, 0 probe flags."
        ));
        assert!(description.contains("Active tool: Wire."));
        assert!(!description.contains("Selected instance"));
        assert!(!description.contains("item selected"));
        assert!(description.contains(
            "Arrow keys select the nearest eligible schematic object in each direction."
        ));
        assert!(description.contains("Escape: Cancel active command"));
        assert!(description.contains("Shift+T: Place text or note"));
        assert!(description.contains("S: Stretch selection"));
        assert_eq!(
            schematic_selection_accessibility_status(&state),
            "Selected instance unnamed, Resistor."
        );
        state.schematic.selection.clear();
        assert_eq!(
            schematic_accessibility_description(
                &state,
                crate::workbench::commands::vocabulary::CommandPlatform::Desktop,
                egui::os::OperatingSystem::Windows,
            ),
            description
        );
        assert_eq!(
            schematic_selection_accessibility_status(&state),
            "No schematic object selected."
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn canvas_accessibility_uses_a_stable_node_and_concise_live_selection_status() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut state = AppState::default();
        let mut component = Component::new(17, ComponentType::Resistor, Point::new(40, 20));
        component.name = "RGAIN".to_owned();
        component.value = "499 ohm".to_owned();
        state.schematic.components.push(component);
        state.schematic.selection.select_only_component(17);

        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| render_schematic_view(ui, &mut state, None));
            },
        );
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("schematic accessibility tree")
            .nodes;
        let canvas = nodes
            .iter()
            .find(|(_, node)| node.role() == egui::accesskit::Role::Canvas)
            .map(|(_, node)| node)
            .expect("schematic canvas node");
        assert_eq!(canvas.label(), Some("Schematic canvas"));
        assert!(
            canvas
                .description()
                .is_some_and(|description| description.contains(
                    "Arrow keys select the nearest eligible schematic object in each direction."
                ))
        );
        assert_eq!(canvas.live(), None);

        let status = nodes
            .iter()
            .find(|(_, node)| {
                node.role() == egui::accesskit::Role::Status
                    && node.label() == Some("Selected instance RGAIN, 499 ohm.")
            })
            .map(|(_, node)| node)
            .expect("concise schematic selection status node");
        assert_eq!(status.live(), Some(egui::accesskit::Live::Polite));
    }

    #[test]
    fn schematic_canvas_has_no_static_shortcut_prose() {
        let source = include_str!("view.rs");
        assert!(!source.contains(concat!("Use S", " for select")));
        assert!(!source.contains(concat!("Escape", " to cancel")));
    }
}
