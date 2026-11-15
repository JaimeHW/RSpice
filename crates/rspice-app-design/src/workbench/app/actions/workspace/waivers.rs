//! Waiving a finding from the workbench.
//!
//! The model edits the schematic that stores a waiver; the symbol in front of
//! the reader stores its own through its editor. Either way the run gate's
//! report is dropped and every published finding is stated again.

use crate::workbench::app_state::waivers::StoredWaivers;
use crate::workbench::app_state::{Model, SlotFacts};

use super::symbol_document::run_active_symbol_pin_checks;

/// State the findings again where a history step changed the waivers
/// stored as they were in `before`. Returns whether a schematic's
/// changed: those are the waivers the run gate applies.
///
/// An undo restores the document whole, a waiver with it, and nothing
/// about that tells the published findings: without this a finding whose
/// waiver was undone would still be listed waived, and still not count.
pub(crate) fn restate_waived_findings(
    model: &mut Model,
    facts: SlotFacts,
    before: &StoredWaivers,
) -> bool {
    let after = model.stored_waivers();
    if after.symbol_changed_since(before) {
        // The pin checks read the symbol document, waivers and all.
        run_active_symbol_pin_checks(model, facts);
    }
    let schematics = after.schematics_changed_since(before);
    if schematics {
        model.restate_design_checks(facts);
    }
    schematics
}
