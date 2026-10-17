//! Update symbol: a hand-edited symbol brought back into step with the
//! Verilog-A module that defines its cell's ports.
//!
//! The symbol is a library cell's document, not the open tab's, so the update
//! is one project undo step: the same record an imported or published symbol
//! definition makes. Pins the module no longer declares stay for the author
//! to remove, because removing one would disconnect whatever is wired to it.

use crate::state::CellViewRef;
use crate::workbench::app_state::{AppState, publish_symbol_definition_candidate};
use crate::workbench::documents::code_workspace::{
    VerilogACellSymbol, reconciled_symbol, symbol_standing,
};

impl AppState {
    /// Update the open Verilog-A view's symbol, and say what happened.
    pub(crate) fn update_active_veriloga_symbol(&mut self) {
        let reference = self.workspace.content.active_view.clone();
        match self.update_veriloga_symbol(&reference) {
            Ok(message) | Err(message) => self.prompt(message),
        }
    }

    /// Reconcile the hand-edited symbol of the cell owning the Verilog-A view
    /// at `reference` with the module's ports, as one undo step. Returns the
    /// sentence describing what changed.
    pub(crate) fn update_veriloga_symbol(
        &mut self,
        reference: &CellViewRef,
    ) -> Result<String, String> {
        let drift = match symbol_standing(&self.library_manager, reference) {
            Some(VerilogACellSymbol::Drifted(drift)) => drift,
            _ => return Err("The symbol already matches the module's ports.".to_owned()),
        };
        let document = reconciled_symbol(&self.library_manager, reference)?;
        let mut candidate = self.library_manager.clone();
        candidate
            .edit_library(&reference.library)
            .and_then(|mut library| {
                library.store_symbol_document(
                    &reference.cell,
                    rspice_design::symbol::generated::GENERATED_SYMBOL_VIEW,
                    &document,
                )
            })
            .ok_or_else(|| {
                format!(
                    "{}/{} has no symbol view",
                    reference.library, reference.cell
                )
            })??;
        publish_symbol_definition_candidate(
            self,
            candidate,
            &reference.library,
            &reference.cell,
            "update symbol from Verilog-A module ports",
        )?;
        let mut message = format!(
            "Updated the {}/{} symbol from the module's ports ({} added, {} redirected).",
            reference.library,
            reference.cell,
            drift.missing.len(),
            drift.turned.len()
        );
        match drift.extra.as_slice() {
            [] => {}
            [pin] => message.push_str(&format!(
                " Pin {pin} is not a module port and was kept; remove it in the symbol editor."
            )),
            pins => message.push_str(&format!(
                " Pins {} are not module ports and were kept; remove them in the symbol editor.",
                pins.join(", ")
            )),
        }
        Ok(message)
    }
}
