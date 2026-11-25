//! Probe edits with the same document-local history and selection semantics.

use crate::product::{SavedOutputId, SimulationPlanId};
use crate::state::{DifferenceEnd, Point, SchematicState};

impl SchematicState {
    pub fn bind_probe_saved_output(
        &mut self,
        existing_id: u64,
        binding: Option<(SimulationPlanId, SavedOutputId)>,
    ) {
        if self.session.read_only {
            return;
        }
        let edit = self.design.bind_probe_saved_output(existing_id, binding);
        if edit.value || edit.committed {
            self.finish_document_edit(edit.committed);
        }
    }

    pub fn place_schematic_probe(
        &mut self,
        position: Point,
        label: Option<String>,
        source_expression: Option<String>,
        binding: Option<(SimulationPlanId, SavedOutputId)>,
    ) -> Result<u64, String> {
        if self.session.read_only {
            return Err("the probe marker did not change the active schematic".to_owned());
        }
        let edit = self
            .design
            .place_schematic_probe(position, label, source_expression, binding);
        if let Some(id) = edit.value {
            self.session.editor.selection.select_only_probe(id);
        }
        if edit.value.is_some() || edit.committed {
            self.finish_document_edit(edit.committed);
        }
        // Inside a caller's open operation the marker joins it, and the
        // caller's step commits it.
        match edit.value {
            Some(id) if (edit.committed || self.has_pending_operation()) && id != 0 => Ok(id),
            _ => Err("the probe marker did not change the active schematic".to_owned()),
        }
    }

    pub fn edit_probe_intent(
        &mut self,
        id: u64,
        enabled: bool,
        plot_on_materialization: bool,
    ) -> bool {
        if self.session.read_only {
            return false;
        }
        let edit = self
            .design
            .edit_probe_intent(id, enabled, plot_on_materialization);
        if edit.value || edit.committed {
            self.finish_document_edit(edit.committed);
        }
        edit.committed
    }

    /// Make probe `id` the `end` of a two-net probe. Its net is saved for
    /// the voltage between the ends and is not plotted on its own.
    pub fn mark_probe_difference_end(&mut self, id: u64, end: DifferenceEnd) -> bool {
        if self.session.read_only {
            return false;
        }
        let edit = self.design.mark_probe_difference_end(id, end);
        if edit.value || edit.committed {
            self.finish_document_edit(edit.committed);
        }
        edit.value
    }
}
