//! Tool and draft lifecycle owned by the per-document editor session.

use super::{EditorSession, tool::Tool};
use rspice_design::schematic::component_type::ComponentType;

impl EditorSession {
    /// An unfinished gesture has captured coordinates under the current grid/snap policy.
    pub fn canvas_settings_change_blocked(&self) -> bool {
        self.wire_drawing.active
            || self.bus_drawing.active
            || !self.documentation_shape_drawing.points.is_empty()
            || self.documentation_shape_drawing.keyboard_active
            || self.selection_rect.is_active()
    }

    /// Switch tools, cancelling incompatible routes and retiring typed placement payloads.
    pub fn arm_tool(&mut self, tool: Tool) {
        if self.tool != tool {
            self.cancel_routing_gestures();
        }
        if tool != Tool::BusTap {
            self.pending_bus_tap = None;
        }
        if tool != Tool::Place(ComponentType::Port) {
            self.pending_port_sequence = None;
        }
        if tool != Tool::DesignNote {
            self.pending_design_note = None;
        }
        if tool != Tool::DocumentationShape {
            self.pending_documentation_shape = None;
            self.documentation_shape_drawing.clear();
        }
        // Plain re-arming retires model/stimulus payloads, even for the same kind.
        // Payload-aware callers install their new binding after arming.
        self.pending_part_model = None;
        self.pending_stimulus = None;
        self.tool = tool;
    }

    /// Retire both conductor drafts without changing document geometry.
    fn cancel_routing_gestures(&mut self) {
        self.wire_drawing.clear();
        self.bus_drawing.cancel();
    }

    /// Cancel the tool and every unfinished route, including hidden stale routes.
    pub fn cancel_tool(&mut self) {
        self.cancel_routing_gestures();
        self.pending_bus_tap = None;
        self.pending_port_sequence = None;
        self.pending_design_note = None;
        self.pending_documentation_shape = None;
        self.documentation_shape_drawing.clear();
        self.pending_part_model = None;
        self.pending_stimulus = None;
        self.tool = Tool::Select;
    }

    /// Handle one local Escape stage when the host has no pending document transaction.
    pub fn cancel_interaction_step(&mut self) {
        if self.wire_drawing.active || self.bus_drawing.active {
            self.cancel_routing_gestures();
            return;
        }
        if self.tool != Tool::Select
            || self.pending_bus_tap.is_some()
            || self.pending_port_sequence.is_some()
            || self.pending_design_note.is_some()
            || self.pending_documentation_shape.is_some()
            || !self.documentation_shape_drawing.points.is_empty()
        {
            self.cancel_tool();
            return;
        }
        self.selection.clear();
        self.selection_rect.cancel();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::bus::{PendingBusTap, PendingBusTapPlacement};
    use rspice_design::schematic::bus::{BusDeclaration, BusSlice, BusTapOrientation};
    use rspice_design_model::Point;

    #[test]
    fn switching_conductor_tools_cancels_incompatible_routes_and_tap_state() {
        let mut schematic = EditorSession::default();
        schematic.arm_tool(Tool::Wire);
        schematic.wire_drawing.active = true;
        schematic.wire_drawing.points.push(Point::origin());
        assert!(schematic.wire_drawing.active);

        schematic.arm_tool(Tool::Bus);
        assert!(!schematic.wire_drawing.active);
        schematic.bus_drawing.start(Point::new(2, 3), None);
        assert!(schematic.bus_drawing.active);

        schematic.pending_bus_tap = Some(PendingBusTapPlacement::new(
            PendingBusTap::new(
                BusDeclaration::parse("DATA[15:0]").unwrap(),
                BusSlice::parse("DATA[7:0]").unwrap(),
                BusTapOrientation::Automatic,
            )
            .unwrap(),
            crate::requests::EditorRequestSource {
                project: rspice_app_types::product::ProjectId::new(),
                document: rspice_design_model::cell_view::CellViewRef::new(
                    "work",
                    "cell",
                    "schematic",
                ),
                occurrence: None,
                design_epoch: 0,
                document_epoch: 0,
                content_version: 0,
                topology_version: 0,
                symbol_revision: 0,
                sheet: None,
            },
        ));
        schematic.arm_tool(Tool::BusTap);
        assert!(!schematic.bus_drawing.active);
        assert!(schematic.pending_bus_tap.is_some());

        schematic.arm_tool(Tool::Wire);
        assert!(schematic.pending_bus_tap.is_none());
    }

    #[test]
    fn cancel_clears_even_hidden_conductor_routes() {
        let mut schematic = EditorSession::default();
        schematic.wire_drawing.active = true;
        schematic.wire_drawing.points.push(Point::origin());
        schematic.bus_drawing.start(Point::new(4, 5), None);
        assert!(schematic.wire_drawing.active);
        assert!(schematic.bus_drawing.active);

        schematic.cancel_tool();

        assert_eq!(schematic.tool, Tool::Select);
        assert!(!schematic.wire_drawing.active);
        assert!(schematic.wire_drawing.points.is_empty());
        assert!(!schematic.bus_drawing.active);
        assert!(schematic.bus_drawing.points.is_empty());
        assert!(schematic.pending_bus_tap.is_none());
    }
}
