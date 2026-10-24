//! Waiving a finding from the workbench.
//!
//! The model edits the schematic that stores a waiver; the symbol in front of
//! the reader stores its own through its editor. Either way the run gate's
//! report is dropped and every published finding is stated again.

use crate::workbench::app_state::AppState;
use crate::workbench::app_state::waivers::{StoredWaiverEdit, StoredWaivers, WaiverTarget};

impl AppState {
    /// State the findings again where a history step changed the waivers
    /// stored as they were in `before`. Returns whether a schematic's
    /// changed: those are the waivers the run gate applies.
    ///
    /// An undo restores the document whole, a waiver with it, and nothing
    /// about that tells the published findings: without this a finding whose
    /// waiver was undone would still be listed waived, and still not count.
    // Application half of a model operation; it becomes a request.
    pub(crate) fn restate_waived_findings(&mut self, before: &StoredWaivers) -> bool {
        let after = self.model.stored_waivers();
        if after.symbol_changed_since(before) {
            // The pin checks read the symbol document, waivers and all.
            self.run_active_symbol_pin_checks();
        }
        let schematics = after.schematics_changed_since(before);
        if schematics {
            self.restate_design_checks();
        }
        schematics
    }

    /// Waive the finding at `target` for `reason`, as an edit of the document
    /// that stores it, and state every published finding again.
    // Application half of a model operation; it becomes a request.
    pub(crate) fn waive_finding(
        &mut self,
        target: &WaiverTarget,
        reason: &str,
    ) -> Result<(), String> {
        let edit = StoredWaiverEdit::waive(target, reason)?;
        self.edit_waivers(&target.cell_view, edit)
    }

    /// Remove the waiver stored at `target`, as an edit of the document that
    /// stores it, and state every published finding again.
    // Application half of a model operation; it becomes a request.
    pub(crate) fn remove_finding_waiver(&mut self, target: &WaiverTarget) -> Result<(), String> {
        self.edit_waivers(&target.cell_view, StoredWaiverEdit::remove(target))
    }

    fn edit_waivers(
        &mut self,
        cell_view: &crate::state::CellViewRef,
        edit: StoredWaiverEdit,
    ) -> Result<(), String> {
        if let Some(edit) = self.model.edit_held_waivers(cell_view, edit)? {
            self.edit_active_symbol_waivers(edit)?;
            // The pin checks read the symbol document, waivers and all,
            // so they are asked again rather than left stale.
            self.run_active_symbol_pin_checks();
        }
        self.runs.preflight.invalidate();
        self.restate_design_checks();
        Ok(())
    }

    fn edit_active_symbol_waivers(&mut self, edit: StoredWaiverEdit) -> Result<(), String> {
        let before = self.model.load_active_symbol_document()?;
        let mut document = before.clone();
        edit.apply(&mut document.waivers)?;
        if document == before {
            return Ok(());
        }
        self.store_active_symbol_document(&document)?;
        self.record_symbol_edit(&before);
        Ok(())
    }
}
