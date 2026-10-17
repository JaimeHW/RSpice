//! Waiver edits with the same document-local history as every other edit of
//! the schematic that stores them.

use rspice_design::drc::DrcWaiver;

use crate::state::SchematicState;

impl SchematicState {
    /// Record `waiver` on this schematic as one undoable edit.
    pub(crate) fn waive(&mut self, waiver: DrcWaiver) -> Result<(), String> {
        if self.session.read_only {
            return Err("the schematic that stores this waiver is read-only".to_owned());
        }
        let edit = self
            .design
            .waive(waiver)
            .map_err(|refusal| refusal.to_string())?;
        self.finish_document_edit(edit.committed);
        Ok(())
    }

    /// Remove the waiver stored for `rule` on `object`, as one undoable
    /// edit. Returns whether there was one.
    pub(crate) fn remove_waiver(&mut self, rule: &str, object: &str) -> Result<bool, String> {
        if self.session.read_only {
            return Err("the schematic that stores this waiver is read-only".to_owned());
        }
        let edit = self
            .design
            .remove_waiver(rule, object)
            .map_err(|refusal| refusal.to_string())?;
        if edit.value {
            self.finish_document_edit(edit.committed);
        }
        Ok(edit.value)
    }
}
