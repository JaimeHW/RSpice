//! What the code feature's work asks of the session past the code feature:
//! placing a compiled Verilog-A device on a schematic, and bringing its
//! cell's symbol into step and opening it in the symbol editor. These take
//! the session whole, so they stay in the shell when the code feature
//! moves into a crate of its own.

use crate::state::{CellViewRef, VerilogAPlacementHandoff, library_cell_placement_candidates};
use crate::workbench::app::actions::veriloga_placement::veriloga_placement_target;
use crate::workbench::app::actions::veriloga_symbol::{
    active_veriloga_symbol, update_veriloga_symbol,
};
use crate::workbench::app_state::AppState;
use crate::workbench::documents::code_workspace::{VerilogACellSymbol, symbol_standing};
use rspice_schematic_editor::symbol_editor::session::SymbolSelection;

impl AppState {
    /// The placement the open Verilog-A view's compile result offers: the
    /// device and where it would go.
    pub(crate) fn offered_veriloga_placement(
        &self,
    ) -> Option<(VerilogAPlacementHandoff, CellViewRef)> {
        let handoff = self
            .code
            .code_workspace
            .veriloga
            .receipt
            .as_ref()?
            .placement
            .clone()?;
        let target = veriloga_placement_target(&self.model, &handoff)?;
        Some((handoff, target))
    }

    /// Take the offered placement: switch to its schematic and arm the
    /// device there, as the shelf arms a cell.
    pub(crate) fn place_offered_veriloga_device(&mut self) -> Result<(), String> {
        let (handoff, target) = self
            .offered_veriloga_placement()
            .ok_or_else(|| "No schematic is open to place the device in.".to_owned())?;
        let candidate =
            library_cell_placement_candidates(&self.model.library_manager, &self.model.workspace)
                .into_iter()
                .find(|candidate| {
                    candidate.library == handoff.view.library
                        && candidate.cell == handoff.view.cell
                        && candidate.view == handoff.view.view
                })
                .ok_or_else(|| {
                    format!("{} is no longer a cell view.", handoff.view.display_path())
                })?;
        if !candidate.ready {
            return Err(format!(
                "{} cannot be placed yet: {}.",
                handoff.view.display_path(),
                candidate.unavailable_reason
            ));
        }
        if let Some(receipt) = self.code.code_workspace.veriloga.receipt.as_mut() {
            receipt.placement = None;
        }
        self.open_workspace_view(target.clone());
        self.workbench
            .activate(crate::workbench::state::Workspace::Design);
        rspice_app_state::workbench::app::schematic::placement::arm_cell_instance(
            &mut self.model,
            candidate.binding,
        );
        Ok(())
    }
}

impl AppState {
    /// Update the open Verilog-A view's symbol, say what happened, and take
    /// its author to it with the added pins selected, ready to be placed.
    pub(crate) fn update_active_veriloga_symbol(&mut self) {
        let reference = self.model.workspace.content.active_view.clone();
        let added = match symbol_standing(&self.model.library_manager, &reference) {
            Some(VerilogACellSymbol::Drifted(drift)) => drift.missing,
            _ => Vec::new(),
        };
        match update_veriloga_symbol(&mut self.model, &reference) {
            Ok(message) => {
                self.prompt(message);
                open_veriloga_symbol(self);
                self.design.symbol.editor.set_selection(SymbolSelection {
                    pins: added.into_iter().collect(),
                    ..SymbolSelection::default()
                });
            }
            Err(message) => self.prompt(message),
        }
    }
}

/// Open the symbol of the cell the open Verilog-A view defines in the symbol
/// editor, where it is finished by hand.
pub(in crate::workbench) fn open_veriloga_symbol(state: &mut AppState) {
    if let Some(symbol) = active_veriloga_symbol(&state.model) {
        state.open_workspace_view(symbol);
    }
}
