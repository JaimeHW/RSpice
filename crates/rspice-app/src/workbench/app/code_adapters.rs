//! What the code feature's work asks of the session past the code feature:
//! placing a compiled Verilog-A device on a schematic, and bringing its
//! cell's symbol into step and opening it in the symbol editor. These
//! reach design's navigation and symbol editor, so they stay in the shell
//! when the code feature moves into a crate of its own.

use crate::workbench::app::actions::veriloga_placement::take_offered_veriloga_placement;
use crate::workbench::app::actions::veriloga_symbol::{
    active_veriloga_symbol, update_active_veriloga_symbol_in,
};
use crate::workbench::app_state::AppState;

impl AppState {
    /// Take the offered placement: switch to its schematic and arm the
    /// device there, as the shelf arms a cell.
    pub(crate) fn place_offered_veriloga_device(&mut self) -> Result<(), String> {
        let (target, binding) = take_offered_veriloga_placement(
            &mut self.code.code_workspace.veriloga.receipt,
            &self.model,
        )?;
        self.navigation().arm_cell_in_schematic(target, binding);
        Ok(())
    }
}

impl AppState {
    /// Update the open Verilog-A view's symbol, say what happened, and take
    /// its author to it with the added pins selected, ready to be placed.
    pub(crate) fn update_active_veriloga_symbol(&mut self) {
        if let Some((symbol, added)) =
            update_active_veriloga_symbol_in(&mut self.model, &mut self.workbench)
        {
            self.navigation()
                .open_symbol_with_pins_selected(symbol, added);
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
