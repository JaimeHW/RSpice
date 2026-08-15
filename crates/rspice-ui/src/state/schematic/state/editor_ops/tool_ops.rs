//! The armed-tool lifecycle.
//!
//! Arming, cancelling, and stepping back out of a tool are transitions of the
//! schematic document, not application commands: each one reads and writes
//! only this state. They previously lived in the workbench command dispatcher,
//! which put the schematic editor in the position of reaching up through the
//! whole application shell to change its own armed tool.

use super::super::*;

impl SchematicState {
    /// Whether changing grid, snap, or routing policy would alter an
    /// interaction that is already in progress.
    ///
    /// Merely arming a placement tool is safe: the next placement should use
    /// the newly applied policy. In contrast, routed conductors,
    /// multi-click documentation shapes, keyboard-driven shape placement,
    /// rubber-band selection, and pending drag transactions have already
    /// captured canvas coordinates under the current policy and must finish
    /// or be cancelled first.
    pub fn canvas_settings_change_blocked(&self) -> bool {
        self.wire_drawing.active
            || self.bus_drawing.active
            || !self.documentation_shape_drawing.points.is_empty()
            || self.documentation_shape_drawing.keyboard_active
            || self.selection_rect.is_active()
            || self.has_pending_operation()
    }

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
        // A model card is armed for one exact device kind. Re-arming any
        // other tool — including a different device — retires it, so a
        // resistor can never be placed carrying a diode's card.
        if Some(tool) != self.pending_part_model.as_ref().map(|armed| armed.tool) {
            self.pending_part_model = None;
        }
        self.tool = tool;
    }

    /// Arm one part published by a distributed model pack.
    ///
    /// Returns the label a receipt or toast names. The two placement shapes
    /// differ in what the schematic has to remember: a macromodel is a cell
    /// instance and carries its whole interface in the binding, while a model
    /// card is a native device that has to be told which card it holds and
    /// which of the family's symbol skins to wear.
    pub fn arm_pack_part(&mut self, placement: crate::state::model_hub::PartPlacement) -> String {
        match placement {
            crate::state::model_hub::PartPlacement::CellInstance(binding) => {
                let label = format!("{}/{}", binding.library, binding.cell);
                self.pending_library_cell = Some(*binding);
                self.arm_tool(Tool::Place(ComponentType::CellInstance));
                label
            }
            crate::state::model_hub::PartPlacement::NativeDevice {
                component_type,
                variant,
                model,
            } => {
                self.pending_library_cell = None;
                self.arm_tool(Tool::Place(component_type));
                self.pending_part_model = Some(PendingPartModel {
                    tool: Tool::Place(component_type),
                    model: model.clone(),
                    variant,
                });
                model
            }
        }
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
        self.pending_part_model = None;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_settings_are_blocked_only_by_in_progress_canvas_interactions() {
        let mut schematic = SchematicState::default();
        schematic.arm_tool(Tool::Place(ComponentType::Resistor));
        assert!(
            !schematic.canvas_settings_change_blocked(),
            "an armed tool has not committed any coordinates yet"
        );

        schematic.wire_drawing.active = true;
        assert!(schematic.canvas_settings_change_blocked());
        schematic.wire_drawing.clear();

        schematic.documentation_shape_drawing.keyboard_active = true;
        assert!(schematic.canvas_settings_change_blocked());
        schematic.documentation_shape_drawing.clear();

        schematic.selection_rect.start_at(Point::origin());
        assert!(schematic.canvas_settings_change_blocked());
        schematic.selection_rect.cancel();

        schematic.begin_operation("drag selection");
        assert!(schematic.canvas_settings_change_blocked());
        schematic.cancel_operation();
        assert!(!schematic.canvas_settings_change_blocked());
    }
}
