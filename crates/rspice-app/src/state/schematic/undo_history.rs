//! Editor selection and dirty-state restoration for cancelled document edits.
pub use super::committed_history::{SchematicSnapshot, UndoSequence, next_undo_sequence};

#[derive(Debug, Clone)]
pub(super) struct OperationCancelState {
    operation_id: u64,
    selection: super::selection::Selection,
    was_dirty: bool,
}

impl OperationCancelState {
    pub(super) fn capture(
        operation_id: u64,
        selection: &super::selection::Selection,
        was_dirty: bool,
    ) -> Self {
        Self {
            operation_id,
            selection: selection.clone(),
            was_dirty,
        }
    }

    pub(super) fn operation_id(&self) -> u64 {
        self.operation_id
    }
    pub(super) fn was_dirty(&self) -> bool {
        self.was_dirty
    }
    pub(super) fn set_was_dirty(&mut self, was_dirty: bool) {
        self.was_dirty = was_dirty;
    }

    pub(super) fn restore(self, state: &mut super::state::SchematicState) {
        state.session.editor.selection = self.selection;
        state.session.is_dirty = self.was_dirty;
    }
}

#[cfg(test)]
mod tests {
    use super::super::component_type::ComponentType;
    use super::super::state::SchematicState;
    use super::*;
    use rspice_design_model::Point;

    fn capture(state: &SchematicState) -> SchematicSnapshot {
        SchematicSnapshot::capture(&state.design.document())
    }

    // -------------------------------------------------------------------------
    // SchematicState wrappers (with_undo / undo / redo / dirty flag)
    // -------------------------------------------------------------------------

    #[test]
    fn with_undo_records_entry_and_undo_restores_state_exactly() {
        let mut state = SchematicState::default();
        state.init_undo_history();
        let baseline = capture(&state);

        let changed = state.with_undo("Add resistor", |s| {
            s.add_component(ComponentType::Resistor, Point::new(10, 20));
        });
        assert!(changed);
        assert_eq!(state.design.document().components.len(), 1);
        assert!(state.can_undo());

        assert!(state.undo());
        assert!(state.design.document().components.is_empty());
        assert!(capture(&state).is_equal(&baseline));
        assert!(!state.can_undo());
        assert!(state.can_redo());
    }

    #[test]
    fn state_redo_reapplies_the_operation() {
        let mut state = SchematicState::default();
        state.init_undo_history();

        state.with_undo("Add resistor", |s| {
            s.add_component(ComponentType::Resistor, Point::new(10, 20));
        });
        let after = capture(&state);

        assert!(state.undo());
        assert!(state.redo());
        assert_eq!(state.design.document().components.len(), 1);
        assert!(capture(&state).is_equal(&after));
        assert!(state.can_undo());
        assert!(!state.can_redo());
    }

    #[test]
    fn with_undo_returns_false_when_nothing_changed() {
        let mut state = SchematicState::default();
        state.init_undo_history();

        let changed = state.with_undo("Nothing", |_| {});
        assert!(!changed);
        assert!(!state.can_undo());
    }

    #[test]
    fn complete_component_properties_and_connections_participate_in_undo_equality() {
        let mut state = SchematicState::default();
        let id = state.add_component(ComponentType::Resistor, Point::new(10, 20));
        state.clear_undo_history();

        assert!(state.with_undo("Edit component properties", |schematic| {
            let component = schematic
                .design
                .document_mut_for_test()
                .components
                .iter_mut()
                .find(|component| component.id == id)
                .unwrap();
            component.name = "R99".to_owned();
            component.params = "tc1=0.001".to_owned();
        }));
        assert!(state.can_undo());
        assert!(state.undo());
        let component = state
            .design
            .document()
            .components
            .iter()
            .find(|component| component.id == id)
            .unwrap();
        assert_ne!(component.name, "R99");
        assert!(component.params.is_empty());
    }

    #[test]
    fn project_portable_grid_policy_participates_in_undo_and_redo() {
        let mut state = SchematicState::default();
        state.init_undo_history();
        let original_policy = state.design.document().document_policy;
        let original_grid_size = state.design.document().grid_size;

        assert!(state.with_undo("change schematic grid pitch", |schematic| {
            schematic
                .design
                .document_mut_for_test()
                .document_policy
                .grid_pitch = super::super::document_policy::SchematicGridPitch::Metric;
            schematic.design.document_mut_for_test().grid_size = schematic
                .design
                .document()
                .document_policy
                .grid_pitch
                .canvas_grid_size();
        }));
        assert_ne!(state.design.document().document_policy, original_policy);
        assert_ne!(state.design.document().grid_size, original_grid_size);

        assert!(state.undo());
        assert_eq!(state.design.document().document_policy, original_policy);
        assert_eq!(state.design.document().grid_size, original_grid_size);
        assert!(state.redo());
        assert_eq!(
            state.design.document().document_policy.grid_pitch,
            super::super::document_policy::SchematicGridPitch::Metric
        );
        assert_eq!(
            state.design.document().grid_size,
            super::super::document_policy::SchematicGridPitch::Metric.canvas_grid_size()
        );
    }

    #[test]
    fn undo_and_redo_preserve_exact_runtime_wire_and_bus_routing() {
        let mut state = SchematicState::default();
        state.session.editor.wire_drawing.routing_mode =
            super::super::wire::WireRoutingMode::VerticalFirst;
        state.session.editor.bus_drawing.routing_mode =
            super::super::wire::WireRoutingMode::VerticalFirst;
        state.init_undo_history();

        assert!(state.with_undo("Add resistor", |schematic| {
            schematic.add_component(ComponentType::Resistor, Point::new(0, 0));
        }));
        assert!(state.undo());
        assert_eq!(
            state.session.editor.wire_drawing.routing_mode,
            super::super::wire::WireRoutingMode::VerticalFirst
        );
        assert_eq!(
            state.session.editor.bus_drawing.routing_mode,
            super::super::wire::WireRoutingMode::VerticalFirst
        );

        assert!(state.redo());
        assert_eq!(
            state.session.editor.wire_drawing.routing_mode,
            super::super::wire::WireRoutingMode::VerticalFirst
        );
        assert_eq!(
            state.session.editor.bus_drawing.routing_mode,
            super::super::wire::WireRoutingMode::VerticalFirst
        );
    }

    #[test]
    fn with_undo_is_skipped_on_read_only_state() {
        let mut state = SchematicState::default();
        state.init_undo_history();
        state.session.read_only = true;

        let changed = state.with_undo("Add resistor", |s| {
            s.add_component(ComponentType::Resistor, Point::new(0, 0));
        });

        // The closure must not run at all on a read-only view.
        assert!(!changed);
        assert!(state.design.document().components.is_empty());
        assert!(!state.can_undo());
    }

    #[test]
    fn state_undo_on_empty_history_returns_false() {
        let mut state = SchematicState::default();
        state.init_undo_history();

        assert!(!state.undo());
        assert!(!state.redo());
    }

    #[test]
    fn undo_marks_state_dirty_and_clears_selection() {
        let mut state = SchematicState::default();
        state.init_undo_history();

        let id = {
            let mut id = 0;
            state.with_undo("Add resistor", |s| {
                id = s.add_component(ComponentType::Resistor, Point::new(10, 20));
            });
            id
        };
        state.session.is_dirty = false;
        state.session.editor.selection.select_component(id);
        assert!(!state.session.editor.selection.is_empty());

        assert!(state.undo());
        assert!(state.session.is_dirty);
        assert!(state.session.editor.selection.is_empty());
    }

    #[test]
    fn begin_operation_auto_initializes_history() {
        let mut state = SchematicState::default();
        assert!(!state.history().is_initialized());

        state.begin_operation("Add resistor");
        state.add_component(ComponentType::Resistor, Point::new(0, 0));
        assert!(state.end_operation());

        assert!(state.history().is_initialized());
        assert_eq!(state.history().undo_count(), 1);
    }

    #[test]
    fn nested_helpers_extend_one_outer_atomic_undo_transaction() {
        let mut state = SchematicState::default();
        state.init_undo_history();

        state.begin_operation("outer edit");
        state.add_component(ComponentType::Resistor, Point::new(0, 0));
        assert!(!state.with_undo("nested helper", |schematic| {
            schematic.add_component(ComponentType::Capacitor, Point::new(20, 0));
        }));
        assert!(state.has_pending_operation());
        assert!(state.end_operation());

        assert_eq!(state.history().undo_count(), 1);
        assert_eq!(state.undo_description(), Some("outer edit"));
        assert!(state.undo());
        assert!(state.design.document().components.is_empty());
    }

    #[test]
    fn cancelled_state_operation_leaves_no_entry() {
        let mut state = SchematicState::default();
        state.init_undo_history();

        state.begin_operation("Aborted drag");
        state.add_component(ComponentType::Resistor, Point::new(0, 0));
        state.cancel_operation();

        assert!(!state.has_pending_operation());
        assert!(!state.can_undo());
        assert!(state.design.document().components.is_empty());
    }

    #[test]
    fn cancelled_operation_restores_selection_dirty_state_and_redo_without_rewinding_caches() {
        for was_dirty in [false, true] {
            let mut state = SchematicState::default();
            let resistor = state.add_component(ComponentType::Resistor, Point::new(10, 20));
            state.with_undo("add capacitor", |state| {
                state.add_component(ComponentType::Capacitor, Point::new(100, 20));
            });
            assert!(state.undo());
            state
                .session
                .editor
                .selection
                .select_only_component(resistor);
            state.session.is_dirty = was_dirty;
            let before = SchematicSnapshot::capture(&state.design.document());
            let selection = state.session.editor.selection.clone();
            let content_version = state.content_version();
            state.begin_operation("drag selection");
            state.design.document_mut_for_test().components[0].pos = Point::new(80, 90);
            state.session.is_dirty = true;
            state.bump_topology_version();
            let dragged_topology = state.topology_version();
            state.session.editor.pan = (123.0, 0.0);

            assert!(state.cancel_operation());
            assert!(before.is_equal_document(&state.design.document()));
            assert_eq!(state.session.editor.selection, selection);
            assert_eq!(state.session.is_dirty, was_dirty);
            assert_eq!(state.session.editor.pan, (123.0, 0.0));
            assert_eq!(state.content_version(), content_version);
            assert_ne!(state.topology_version(), dragged_topology);
            assert_eq!(state.redo_description(), Some("add capacitor"));
            assert!(!state.can_undo());
            let cancelled_topology = state.topology_version();
            assert!(!state.cancel_operation());
            assert_eq!(state.topology_version(), cancelled_topology);
            assert!(state.redo());
            assert_eq!(state.design.document().components.len(), 2);
            assert_eq!(
                state.design.document().components[0].pos,
                Point::new(10, 20)
            );
        }
    }

    #[test]
    fn cancelled_nested_operation_restores_the_outer_baseline() {
        let mut state = SchematicState::default();
        state.session.is_dirty = false;
        state.begin_operation("outer gesture");
        state.add_component(ComponentType::Resistor, Point::origin());
        state.begin_operation("nested helper");
        state.add_component(ComponentType::Capacitor, Point::new(50, 0));
        assert!(state.cancel_operation());
        assert!(state.design.document().components.is_empty());
        assert!(!state.session.is_dirty);
        assert!(!state.has_pending_operation());
        assert!(!state.end_operation());
        assert!(!state.can_undo());
    }

    #[test]
    fn cancelled_no_op_preserves_selection_and_does_not_dirty_or_invalidate_the_document() {
        let mut state = SchematicState::default();
        let resistor = state.add_component(ComponentType::Resistor, Point::origin());
        state
            .session
            .editor
            .selection
            .select_only_component(resistor);
        state.session.is_dirty = false;
        let selection = state.session.editor.selection.clone();
        let topology = state.topology_version();
        state.begin_operation("stationary drag");
        assert!(state.cancel_operation());
        assert_eq!(state.session.editor.selection, selection);
        assert!(!state.session.is_dirty);
        assert_eq!(state.topology_version(), topology);
        assert!(!state.can_undo());
    }

    #[test]
    fn committed_history_does_not_retain_gesture_cancellation_state() {
        let mut state = SchematicState::default();
        state.with_undo("add component", |state| {
            state.add_component(ComponentType::Resistor, Point::origin());
        });
        assert!(state.session.operation_cancel.is_none());
        assert!(state.can_undo());
    }
}
