//! App transaction and model-placement coordination for editor tool changes.

use super::super::*;

impl SchematicState {
    pub fn canvas_settings_change_blocked(&self) -> bool {
        self.session.editor.canvas_settings_change_blocked() || self.has_pending_operation()
    }

    pub fn arm_tool(&mut self, tool: Tool) {
        self.session.editor.arm_tool(tool);
    }

    pub fn cancel_tool(&mut self) {
        self.session.editor.cancel_tool();
    }

    /// A pending document transaction owns Escape before local draft cancellation.
    pub fn cancel_interaction_step(&mut self) {
        if self.cancel_operation() {
            return;
        }
        self.session.editor.cancel_interaction_step();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_schematic_editor::session::placement::PendingPartModel;

    #[test]
    fn canvas_settings_are_blocked_only_by_in_progress_canvas_interactions() {
        let mut schematic = SchematicState::default();
        schematic.arm_tool(Tool::Place(ComponentType::Resistor));
        assert!(
            !schematic.canvas_settings_change_blocked(),
            "an armed tool has not committed any coordinates yet"
        );

        schematic.session.editor.wire_drawing.active = true;
        assert!(schematic.canvas_settings_change_blocked());
        schematic.session.editor.wire_drawing.clear();

        schematic
            .session
            .editor
            .documentation_shape_drawing
            .keyboard_active = true;
        assert!(schematic.canvas_settings_change_blocked());
        schematic.session.editor.documentation_shape_drawing.clear();

        schematic
            .session
            .editor
            .selection_rect
            .start_at(Point::origin());
        assert!(schematic.canvas_settings_change_blocked());
        schematic.session.editor.selection_rect.cancel();

        schematic.begin_operation("drag selection");
        assert!(schematic.canvas_settings_change_blocked());
        schematic.cancel_operation();
        assert!(!schematic.canvas_settings_change_blocked());
    }

    /// Arming the plain part of a kind retires a payload armed for that same
    /// kind.
    ///
    /// The reader who picks "Voltage source (SIN)" off the shelf, the toolbar
    /// or the command palette has asked for a default source; the reader who
    /// picks a model card or a stimulus definition has asked for that one.
    /// Both arrive as `Place(kind)`, so a payload that survived a matching
    /// re-arm silently answered the second question when the first was asked.
    #[test]
    fn arming_the_plain_part_of_a_kind_retires_a_payload_armed_for_it() {
        use crate::state::stimulus_library::definition::StimulusDefinition;

        let mut schematic = SchematicState::default();
        schematic.session.editor.pending_part_model = Some(PendingPartModel {
            tool: Tool::Place(ComponentType::Diode),
            model: "RSPICE_ZENER".to_owned(),
            variant: None,
        });
        schematic.arm_tool(Tool::Place(ComponentType::Diode));
        assert!(
            schematic.session.editor.pending_part_model.is_none(),
            "the plain diode is not the zener"
        );

        let definition = StimulusDefinition::new(
            "sensor_drive",
            ComponentType::VoltageSourceSin,
            crate::state::stimulus_library::now_unix_ms,
        )
        .expect("ok");
        schematic.session.editor.pending_stimulus = Some(PendingStimulusPlacement::of(&definition));
        schematic.arm_tool(Tool::Place(ComponentType::VoltageSourceSin));
        assert!(
            schematic.session.editor.pending_stimulus.is_none(),
            "the plain sine source is not the definition"
        );
        let placed =
            schematic.add_armed_component(ComponentType::VoltageSourceSin, Point::new(4, 4));
        assert!(
            schematic.design.document().components[0]
                .stimulus_provenance
                .is_none(),
            "so the next placement is a default instance ({placed})"
        );
    }

    /// Placing does not re-arm, so an armed payload survives every click until
    /// the reader arms something else.
    #[test]
    fn an_armed_definition_survives_repeated_placement() {
        use crate::state::stimulus_library::definition::StimulusDefinition;

        let mut definition = StimulusDefinition::new(
            "sensor_drive",
            ComponentType::VoltageSourceSin,
            crate::state::stimulus_library::now_unix_ms,
        )
        .expect("ok");
        definition.params = "freq=1k".to_owned();
        let mut schematic = SchematicState::default();
        schematic.arm_tool(Tool::Place(ComponentType::VoltageSourceSin));
        schematic.session.editor.pending_stimulus = Some(PendingStimulusPlacement::of(&definition));

        for at in [Point::new(4, 4), Point::new(40, 4)] {
            schematic.add_armed_component(ComponentType::VoltageSourceSin, at);
        }
        assert_eq!(schematic.design.document().components.len(), 2);
        assert!(
            schematic
                .design
                .document()
                .components
                .iter()
                .all(|component| component.stimulus_provenance.is_some()),
            "both clicks placed an adopter"
        );
    }
}
