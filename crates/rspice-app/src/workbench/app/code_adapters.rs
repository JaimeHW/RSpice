//! What the code feature's work asks of the session past the code feature:
//! placing a compiled Verilog-A device on a schematic, bringing its cell's
//! symbol into step and opening it in the symbol editor, opening the
//! source document and New Verilog-A device forms, and carrying out the
//! netlist lifecycle choices the source document dialog hands the shell.
//! These take the session whole, so they stay in the shell when the code
//! feature moves into a crate of its own.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{CellViewRef, VerilogAPlacementHandoff, library_cell_placement_candidates};
use crate::workbench::MessageId;
use crate::workbench::app::actions::veriloga_placement::veriloga_placement_target;
use crate::workbench::app::actions::veriloga_symbol::{
    active_veriloga_symbol, update_veriloga_symbol,
};
use crate::workbench::app::dialogs::library::new_veriloga_device::open_new_veriloga_device_form_in;
use crate::workbench::app::dialogs::source_document::{
    close_netlist_lifecycle, open_source_document_dialog_in,
};
use crate::workbench::app_state::AppState;
use crate::workbench::code::commands::NetlistLifecycleAction;
use crate::workbench::documents::code_workspace::{VerilogACellSymbol, symbol_standing};
use crate::workbench::documents::netlist_document;
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

/// Open New Verilog-A device… on the library holding the active design,
/// remembering the schematic it was opened from.
pub(crate) fn open_new_veriloga_device_form(state: &mut AppState) {
    open_new_veriloga_device_form_in(&state.model, &mut state.code.new_veriloga_device);
}

/// Bring the lifecycle dialog up for the page that is showing.
///
pub(crate) fn open_source_document_dialog(state: &mut AppState) -> Result<(), String> {
    open_source_document_dialog_in(
        state.workbench.workspace,
        &state.model,
        &mut state.code.code_workspace,
        &state.code.netlist,
        &state.platform,
    )
}

/// Carry out a lifecycle choice on the netlist page's deck or include that
/// the dialog handed the shell, then close the dialog when the same frame
/// closed it.
pub(in crate::workbench) fn apply_netlist_lifecycle_action(
    state: &mut AppState,
    action: NetlistLifecycleAction,
    close: bool,
) {
    let messages = state.workbench.messages();
    match action {
        // Nothing to carry out, or a choice the form applies over the Code
        // workspace's and the netlist editor's state and the model (Cancel,
        // OpenHistory, CancelInclude, SetIncludeSearchPaths): the form never
        // hands these over.
        NetlistLifecycleAction::None
        | NetlistLifecycleAction::Cancel
        | NetlistLifecycleAction::OpenHistory
        | NetlistLifecycleAction::CancelInclude
        | NetlistLifecycleAction::SetIncludeSearchPaths(_) => {}
        NetlistLifecycleAction::Begin(file_action) => {
            if let Err(error) = netlist_document::begin_netlist_lifecycle_action_in(
                &state.model,
                &mut state.code.netlist,
                file_action,
            ) {
                state.prompt(error);
            }
        }
        NetlistLifecycleAction::Commit => {
            match netlist_document::commit_netlist_lifecycle_action(
                &mut state.model,
                &mut state.code.netlist,
                &mut state.code.netlist_open_documents,
            ) {
                Ok(message) => {
                    state.record(LogSeverity::Info, LogSource::Studio, message);
                }
                Err(error) => {
                    if let Some(transaction) =
                        state.code.netlist.lifecycle_dialog.transaction.as_mut()
                    {
                        transaction.error = Some(error);
                    } else {
                        state.record(LogSeverity::Error, LogSource::Studio, error);
                    }
                }
            }
        }
        NetlistLifecycleAction::SelectDeck(deck_id) => {
            if let Err(error) = netlist_document::select_retained_top_deck(
                &mut state.model,
                &mut state.code.netlist,
                &mut state.code.netlist_open_documents,
                deck_id,
            ) {
                state.prompt(error);
            }
        }
        NetlistLifecycleAction::CopyIncludeOwnership => {
            match netlist_document::copy_active_dependency_to_project(
                &mut state.model,
                &mut state.code.netlist,
                &mut state.code.netlist_open_documents,
            ) {
                Ok(_) => {
                    state.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        messages.text(MessageId::NetlistCopySucceeded),
                    );
                }
                Err(error) => {
                    state.record(LogSeverity::Error, LogSource::Studio, error);
                }
            }
        }
        NetlistLifecycleAction::ReleaseIncludeOwnership => {
            match netlist_document::release_active_dependency_from_project(
                &mut state.model,
                &mut state.code.netlist,
                &mut state.code.netlist_open_documents,
            ) {
                Ok(()) => {
                    state.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        messages.text(MessageId::NetlistReleaseSucceeded),
                    );
                }
                Err(error) => {
                    state.record(LogSeverity::Error, LogSource::Studio, error);
                }
            }
        }
        NetlistLifecycleAction::BeginInclude(file_action) => {
            if let Err(error) = netlist_document::begin_owned_include_lifecycle_action(
                &state.model,
                &mut state.code.netlist,
                file_action,
            ) {
                state.prompt(error);
            }
        }
        NetlistLifecycleAction::CommitInclude => {
            match netlist_document::commit_owned_include_lifecycle_action(
                &mut state.model,
                &mut state.code.netlist,
                &mut state.code.netlist_open_documents,
            ) {
                Ok(message) => {
                    state.record(LogSeverity::Info, LogSource::Studio, message);
                }
                Err(error) => {
                    if let Some(transaction) = state
                        .code
                        .netlist
                        .lifecycle_dialog
                        .include_transaction
                        .as_mut()
                    {
                        transaction.error = Some(error);
                    }
                }
            }
        }
        NetlistLifecycleAction::SaveSourceAs => {
            netlist_document::open_netlist_save_dialog_in(
                &state.model,
                &mut state.code.netlist,
                true,
            );
            state.code.code_workspace.source_document_dialog = false;
        }
    }
    if close {
        close_netlist_lifecycle(&mut state.code.code_workspace, &mut state.code.netlist);
    }
}

/// Create the top deck the creation form drafted, which the form handed the
/// shell, and close the dialog once it is made; a refusal stays in the
/// form's footer.
pub(in crate::workbench) fn commit_new_top_deck(state: &mut AppState) {
    match netlist_document::commit_netlist_lifecycle_action(
        &mut state.model,
        &mut state.code.netlist,
        &mut state.code.netlist_open_documents,
    ) {
        Ok(message) => {
            state.record(LogSeverity::Info, LogSource::Studio, message);
            state.code.code_workspace.source_document_dialog = false;
            state.code.netlist.lifecycle_dialog.transaction = None;
        }
        Err(error) => {
            if let Some(transaction) = state.code.netlist.lifecycle_dialog.transaction.as_mut() {
                transaction.error = Some(error);
            }
        }
    }
}
