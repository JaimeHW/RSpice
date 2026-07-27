//! The armed-tool lifecycle.
//!
//! Arming, cancelling, and stepping back out of a tool are transitions of the
//! schematic document, not application commands: each one reads and writes
//! only this state. They previously lived in the workbench command dispatcher,
//! which put the schematic editor in the position of reaching up through the
//! whole application shell to change its own armed tool.

use super::super::*;

impl SchematicState {
    /// Arm one tool through the single conductor-lifecycle boundary.
    ///
    /// Switching tools cancels every incompatible unfinished route, and
    /// leaving typed placement retires every incompatible runtime
    /// configuration.
    pub fn arm_tool(&mut self, tool: Tool) {
        if self.tool != tool {
            self.cancel_routing_gestures();
        }
        if tool != Tool::BusTap {
            self.pending_bus_tap = None;
        }
        if tool != Tool::Place(ComponentType::Port) {
            self.pending_port = None;
        }
        if tool != Tool::DesignNote {
            self.pending_design_note = None;
        }
        if tool != Tool::DocumentationShape {
            self.pending_documentation_shape = None;
            self.documentation_shape_drawing.clear();
        }
        self.tool = tool;
    }

    /// Cancel the armed tool outright.
    ///
    /// Escape/cancel is stronger than re-arming Select: it also clears any
    /// inconsistent hidden route restored from legacy or interrupted state.
    pub fn cancel_tool(&mut self) {
        self.cancel_routing_gestures();
        self.pending_bus_tap = None;
        self.pending_port = None;
        self.pending_design_note = None;
        self.pending_documentation_shape = None;
        self.documentation_shape_drawing.clear();
        self.tool = Tool::Select;
    }

    /// Apply one level of the schematic Escape contract.
    ///
    /// Repeated Escape presses walk back an unfinished route, then the armed
    /// authoring tool, then the selection. Keeping these stages separate
    /// prevents a route cancellation from unexpectedly discarding the user's
    /// selection.
    pub fn cancel_interaction_step(&mut self) {
        if self.wire_drawing.active || self.bus_drawing.active {
            self.cancel_routing_gestures();
            return;
        }
        if self.tool != Tool::Select
            || self.pending_bus_tap.is_some()
            || self.pending_port.is_some()
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
