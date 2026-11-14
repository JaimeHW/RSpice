//! What the code feature's work asks of the session past the code feature:
//! opening its Verilog-A cell's symbol in the symbol editor. This reaches
//! design's navigation, so it stays in the shell when the code feature
//! moves into a crate of its own.

use crate::workbench::app::actions::veriloga_symbol::active_veriloga_symbol;
use crate::workbench::app_state::AppState;

/// Open the symbol of the cell the open Verilog-A view defines in the symbol
/// editor, where it is finished by hand.
pub(in crate::workbench) fn open_veriloga_symbol(state: &mut AppState) {
    if let Some(symbol) = active_veriloga_symbol(&state.model) {
        state.open_workspace_view(symbol);
    }
}
