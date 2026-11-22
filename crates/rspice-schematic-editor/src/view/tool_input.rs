//! Canvas event interpretation and typed pointer actions.
//!
//! The host reconciles retained gestures and modal/tool ownership first, then
//! applies these actions synchronously through its checked edit services.

use egui::{Pos2, Response, Ui};
use rspice_design::schematic::component_type::ComponentType;
use rspice_design_model::Point;

use super::{
    coordinates::screen_to_schematic,
    design_view::DesignView,
    drawing::nearest_bus_hit,
    pointer_target::{PointerHit, PointerQuery},
    snap_resolution::{
        nearest_active_wire_screen_hit, resolve_grid_pointer, resolve_target_pointer,
        target_acquisition_radius,
    },
    symbol_context::SchematicSymbolContext,
    viewport::Viewport,
};
use crate::session::{
    snap::{SnapEngine, SnapTargetType},
    tool::Tool,
};

/// Events captured before any tool action changes the editor in this frame.
pub struct ToolInput {
    pub finish_route: bool,
    /// Backspace on a live wire or bus route: take back its last vertex.
    pub remove_route_point: bool,
    /// "/" with the wire or bus tool: flip the corner of the route between
    /// horizontal first and vertical first.
    pub flip_route_corner: bool,
    pub shape_double_click: Option<Pos2>,
    pub primary_click: Option<Pos2>,
    pub select_double_click: Option<Pos2>,
}

impl ToolInput {
    pub fn read(ui: &Ui, response: &Response, tool: Tool, route_active: bool) -> Self {
        let shape_double_click = tool == Tool::DocumentationShape
            && response.double_clicked_by(egui::PointerButton::Primary);
        // The route tools own a double-click whole. With a route live it ends
        // the route. With none, the first click already ended one on a
        // conductor, and the second must not start a new route there.
        let conductor_double_click = matches!(tool, Tool::Wire | Tool::Bus)
            && response.double_clicked_by(egui::PointerButton::Primary);
        let route_double_click = conductor_double_click && route_active;
        let route_enter = response.has_focus()
            && route_active
            && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
        let route_tool = matches!(tool, Tool::Wire | Tool::Bus);
        let remove_route_point = route_tool
            && response.has_focus()
            && route_active
            && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Backspace));
        // KiCad's posture key. It works with the tool armed and no route
        // live too, so the first bend can be chosen before the first click.
        let flip_route_corner = route_tool
            && response.has_focus()
            && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Slash));
        Self {
            finish_route: route_double_click || route_enter,
            remove_route_point,
            flip_route_corner,
            shape_double_click: shape_double_click
                .then(|| response.interact_pointer_pos())
                .flatten(),
            primary_click: (response.clicked_by(egui::PointerButton::Primary)
                && !shape_double_click
                && !conductor_double_click)
                .then(|| response.interact_pointer_pos())
                .flatten(),
            select_double_click: (tool == Tool::Select
                && response.double_clicked_by(egui::PointerButton::Primary))
            .then(|| response.interact_pointer_pos())
            .flatten(),
        }
    }
}

pub struct ToolPointerView<'a> {
    pub design: DesignView<'a>,
    pub snap_engine: &'a SnapEngine,
    pub can_edit: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum ToolPointerAction {
    ReadOnly,
    Place(ComponentType, Point),
    Wire {
        position: Option<Point>,
        /// The click joins a conductor or lands on a pin, which ends a live
        /// route there.
        ends_route: bool,
    },
    Bus {
        position: Point,
        /// The bus the click lands on, which ends a live route there and
        /// joins the two.
        joins: Option<u64>,
    },
    BusTap {
        position: Point,
        radius: i32,
    },
    Junction(Point),
    DesignNote(Point),
    DocumentationShape(Point),
    Select(PointerQuery),
    Probe(Point),
    NetLabel(Point),
}

impl ToolPointerView<'_> {
    pub fn selection_query(&self, viewport: &Viewport, position: Pos2) -> PointerQuery {
        let grid = resolve_grid_pointer(
            self.snap_engine,
            self.design.document.grid_size,
            viewport,
            position,
        )
        .snapped_position;
        PointerQuery {
            hit: PointerHit::new(grid, screen_to_schematic(viewport, position)),
            radius: target_acquisition_radius(viewport),
            position,
        }
    }

    pub fn action(
        &self,
        tool: Tool,
        viewport: &Viewport,
        symbols: &SchematicSymbolContext,
        position: Pos2,
    ) -> Option<ToolPointerAction> {
        let grid_position = || {
            resolve_grid_pointer(
                self.snap_engine,
                self.design.document.grid_size,
                viewport,
                position,
            )
            .snapped_position
        };
        let target_position = || {
            resolve_target_pointer(&self.design, self.snap_engine, symbols, viewport, position)
                .snapped_position
        };
        Some(match tool {
            Tool::Place(_)
            | Tool::Wire
            | Tool::Bus
            | Tool::BusTap
            | Tool::Junction
            | Tool::DesignNote
            | Tool::DocumentationShape
            | Tool::Label
            | Tool::OffSheetConnector
                if !self.can_edit =>
            {
                ToolPointerAction::ReadOnly
            }
            Tool::Place(kind) => ToolPointerAction::Place(kind, grid_position()),
            Tool::Wire => {
                let hit = nearest_active_wire_screen_hit(
                    &self.design,
                    self.snap_engine,
                    viewport,
                    position,
                );
                let target = resolve_target_pointer(
                    &self.design,
                    self.snap_engine,
                    symbols,
                    viewport,
                    position,
                );
                let on_pin = matches!(target.target_type(), Some(SnapTargetType::Terminal { .. }));
                // A visual conductor owns the click. An unrepresentable exact
                // attachment must not fall back to a disconnected grid point.
                ToolPointerAction::Wire {
                    position: hit.map_or(Some(target.snapped_position), |hit| hit.attachment),
                    ends_route: hit.is_some() || on_pin,
                }
            }
            Tool::Bus => {
                // A click on a bus on this sheet lands on it, as a wire click
                // lands on a conductor: the point is the grid point carried
                // onto the bus, so the route ends exactly on it.
                let grid = grid_position();
                let buses = self
                    .design
                    .objects_on_active_sheet(&self.design.document.buses, |bus| bus.id);
                match nearest_bus_hit(
                    buses.as_ref(),
                    screen_to_schematic(viewport, position),
                    target_acquisition_radius(viewport),
                ) {
                    Some(hit) => ToolPointerAction::Bus {
                        position: rspice_design::schematic::bus::nearest_lattice_point_on_segment(
                            grid,
                            hit.segment_start,
                            hit.segment_end,
                        ),
                        joins: Some(hit.bus_id),
                    },
                    None => ToolPointerAction::Bus {
                        position: grid,
                        joins: None,
                    },
                }
            }
            Tool::BusTap => ToolPointerAction::BusTap {
                position: screen_to_schematic(viewport, position),
                radius: target_acquisition_radius(viewport),
            },
            Tool::Junction => ToolPointerAction::Junction(grid_position()),
            Tool::DesignNote => ToolPointerAction::DesignNote(grid_position()),
            Tool::DocumentationShape => ToolPointerAction::DocumentationShape(grid_position()),
            Tool::Select => ToolPointerAction::Select(self.selection_query(viewport, position)),
            Tool::MoveSelection | Tool::StretchSelection | Tool::ArraySelection => return None,
            Tool::Probe => ToolPointerAction::Probe(target_position()),
            Tool::Label | Tool::OffSheetConnector => ToolPointerAction::NetLabel(target_position()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What one frame over a focused canvas reads with `key` pressed.
    fn read_with_key(tool: Tool, key: egui::Key) -> ToolInput {
        let ctx = egui::Context::default();
        let mut input = None;
        for events in [
            Vec::new(),
            vec![egui::Event::Key {
                key,
                physical_key: Some(key),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
        ] {
            let _ = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(400.0, 300.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        // accessibility-pointer-shim: synthetic canvas for the key test.
                        let response = ui.interact(
                            ui.max_rect(),
                            egui::Id::new("tool-input-canvas"),
                            egui::Sense::click(),
                        );
                        response.request_focus();
                        input = Some(ToolInput::read(ui, &response, tool, true));
                    });
                },
            );
        }
        input.expect("the canvas ran")
    }

    /// "/" flips the route's bend with the wire or the bus tool, and
    /// Backspace takes back a bus point as it does a wire point.
    #[test]
    fn slash_and_backspace_belong_to_both_route_tools() {
        for tool in [Tool::Wire, Tool::Bus] {
            assert!(
                read_with_key(tool, egui::Key::Slash).flip_route_corner,
                "{tool:?}"
            );
            assert!(
                read_with_key(tool, egui::Key::Backspace).remove_route_point,
                "{tool:?}"
            );
        }
        assert!(!read_with_key(Tool::Select, egui::Key::Slash).flip_route_corner);
    }
}
