//! Model binding changes after catalog validation, with their original undo semantics.

use crate::state::SchematicState;
use std::path::PathBuf;

impl SchematicState {
    pub(crate) fn select_instance_model(
        &mut self,
        component_id: u64,
        candidate_name: String,
        candidate_source: Option<PathBuf>,
    ) -> bool {
        let changed =
            self.design
                .select_instance_model(component_id, candidate_name, candidate_source);
        if changed {
            self.session.is_dirty = true;
        }
        changed
    }

    pub(crate) fn select_instance_model_section(
        &mut self,
        component_id: u64,
        selected: Option<String>,
    ) -> bool {
        let changed = self
            .design
            .select_instance_model_section(component_id, selected);
        if changed {
            self.session.is_dirty = true;
        }
        changed
    }
}
