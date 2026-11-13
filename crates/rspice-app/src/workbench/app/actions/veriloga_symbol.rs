//! Open symbol: a Verilog-A cell's symbol in the symbol editor. Update symbol:
//! a hand-edited symbol brought back into step with the module that defines
//! its cell's ports, and opened with the pins it gained selected to be placed.
//!
//! The symbol is a library cell's document, not the open tab's, so the update
//! is one project undo step: the same record an imported or published symbol
//! definition makes. Pins the module no longer declares stay for the author
//! to remove, because removing one would disconnect whatever is wired to it.

use crate::state::CellViewRef;
use crate::workbench::app_state::{Model, publish_symbol_definition_candidate};
use crate::workbench::documents::code_workspace::{
    VerilogACellSymbol, reconciled_symbol, symbol_standing,
};
use rspice_design::library::ViewType;
use rspice_design::symbol::generated::GENERATED_SYMBOL_VIEW;

/// The symbol of the cell the open Verilog-A view defines, when it has one.
pub fn active_veriloga_symbol(model: &Model) -> Option<CellViewRef> {
    let active = &model.workspace.content.active_view;
    let cell = model
        .library_manager
        .get_library(&active.library)?
        .get_cell(&active.cell)?;
    (cell.get_view(&active.view)?.view_type == ViewType::VerilogA
        && cell.get_view(GENERATED_SYMBOL_VIEW).is_some())
    .then(|| CellViewRef::new(&active.library, &active.cell, GENERATED_SYMBOL_VIEW))
}

/// Whether the open Verilog-A view's cell has a symbol to open: a compile
/// draws one.
pub(in crate::workbench) fn can_open_veriloga_symbol(model: &Model) -> bool {
    active_veriloga_symbol(model).is_some()
}

/// Reconcile the hand-edited symbol of the cell owning the Verilog-A view at
/// `reference` with the module's ports, as one undo step. Returns the
/// sentence describing what changed.
pub fn update_veriloga_symbol(
    model: &mut Model,
    reference: &CellViewRef,
) -> Result<String, String> {
    let drift = match symbol_standing(&model.library_manager, reference) {
        Some(VerilogACellSymbol::Drifted(drift)) => drift,
        _ => return Err("The symbol already matches the module's ports.".to_owned()),
    };
    let document = reconciled_symbol(&model.library_manager, reference)?;
    let mut candidate = model.library_manager.clone();
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
        model,
        candidate,
        &reference.library,
        &reference.cell,
        "update symbol from Verilog-A module ports",
    )?;
    let mut message = format!(
        "Updated the {}/{} symbol from the module's ports ({} added, {} changed direction).",
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
