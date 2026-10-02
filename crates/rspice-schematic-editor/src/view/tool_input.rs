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
    pointer_target::{PointerHit, PointerQuery},
    snap_resolution::{
        nearest_active_wire_screen_hit, resolve_grid_pointer, resolve_target_pointer,
        target_acquisition_radius,
    },
    symbol_context::SchematicSymbolContext,
    viewport::Viewport,
};
use crate::session::{snap::SnapEngine, tool::Tool};

/// Events captured before any tool action changes the editor in this frame.
pub struct ToolInput {
    pub finish_route: bool,
    pub shape_double_click: Option<Pos2>,
    pub primary_click: Option<Pos2>,
    pub select_double_click: Option<Pos2>,
}

impl ToolInput {
    pub fn read(ui: &Ui, response: &Response, tool: Tool, route_active: bool) -> Self {
        let shape_double_click = tool == Tool::DocumentationShape
            && response.double_clicked_by(egui::PointerButton::Primary);
        let route_double_click = matches!(tool, Tool::Wire | Tool::Bus)
            && response.double_clicked_by(egui::PointerButton::Primary)
            && route_active;
        let route_enter = response.has_focus()
            && route_active
            && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
        Self {
            finish_route: route_double_click || route_enter,
            shape_double_click: shape_double_click
                .then(|| response.interact_pointer_pos())
                .flatten(),
            primary_click: (response.clicked_by(egui::PointerButton::Primary)
                && !shape_double_click
                && !route_double_click)
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
        finish_on_conductor: bool,
    },
    Bus(Point),
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
            | Tool::Probe
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
                let fallback = target_position();
                // A visual conductor owns the click. An unrepresentable exact
                // attachment must not fall back to a disconnected grid point.
                ToolPointerAction::Wire {
                    position: hit.map_or(Some(fallback), |hit| hit.attachment),
                    finish_on_conductor: hit.is_some(),
                }
            }
            Tool::Bus => ToolPointerAction::Bus(grid_position()),
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
