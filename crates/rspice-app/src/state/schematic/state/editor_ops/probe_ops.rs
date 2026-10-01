//! Probe edits with the same document-local history and selection semantics.

use crate::product::{SavedOutputId, SimulationPlanId};
use crate::state::{Point, SchematicState};

impl SchematicState {
    pub(crate) fn bind_probe_saved_output(
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

    pub(crate) fn place_schematic_probe(
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
        match edit.value {
            Some(id) if edit.committed && id != 0 => Ok(id),
            _ => Err("the probe marker did not change the active schematic".to_owned()),
        }
    }

    pub(crate) fn edit_probe_intent(
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
}
