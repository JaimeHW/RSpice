//! Editor scopes around design-owned sheet topology.
use crate::state::SchematicState;
impl SchematicState {
    pub(crate) fn with_active_wire_topology<R>(
        &mut self,
        hidden: impl Fn(u64) -> bool,
        operation: impl FnOnce(&mut SchematicState) -> R,
    ) -> R {
        let hidden = self.design.take_hidden_wire_topology(hidden);
        let result = operation(self);
        hidden.restore(&mut self.design);
        result
    }
    pub(crate) fn with_hidden_wire_topology_preserved<R>(
        &mut self,
        hidden: impl Fn(u64) -> bool,
        operation: impl FnOnce(&mut SchematicState) -> R,
    ) -> R {
        let hidden = self.design.capture_hidden_wire_topology(hidden);
        let result = operation(self);
        hidden.restore(&mut self.design);
        result
    }
}
