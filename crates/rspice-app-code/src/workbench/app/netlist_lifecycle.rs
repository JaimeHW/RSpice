//! The netlist lifecycle choices the source document dialog raises: a
//! lifecycle choice on the netlist page's deck or include, and the top
//! deck the creation form drafted. Each is carried out over the netlist
//! editor, the netlist documents it has open, the Code workspace and the
//! model; a choice refused before it begins is prompted on the session's
//! status bar.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app::dialogs::source_document::close_netlist_lifecycle;
use crate::workbench::app_state::Model;
use crate::workbench::code::commands::NetlistLifecycleAction;
use crate::workbench::documents::code_workspace::CodeWorkspaceRuntimeState;
use crate::workbench::documents::netlist_document::{self, NetlistDocumentState};
use crate::workbench::state::WorkspaceDocumentId;
use crate::workbench::{MessageId, WorkbenchSession};
use std::collections::HashSet;

/// Carry out a lifecycle choice on the netlist page's deck or include that
/// the dialog raised, then close the dialog when the same frame closed it.
pub fn apply_netlist_lifecycle_action(
    code: &mut CodeWorkspaceRuntimeState,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    action: NetlistLifecycleAction,
    close: bool,
) {
    let messages = wb.messages();
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
            if let Err(error) =
                netlist_document::begin_netlist_lifecycle_action_in(model, netlist, file_action)
            {
                wb.prompt(error);
            }
        }
        NetlistLifecycleAction::Commit => {
            match netlist_document::commit_netlist_lifecycle_action(model, netlist, open_documents)
            {
                Ok(message) => {
                    model.record(LogSeverity::Info, LogSource::Studio, message);
                }
                Err(error) => {
                    if let Some(transaction) = netlist.lifecycle_dialog.transaction.as_mut() {
                        transaction.error = Some(error);
                    } else {
                        model.record(LogSeverity::Error, LogSource::Studio, error);
                    }
                }
            }
        }
        NetlistLifecycleAction::SelectDeck(deck_id) => {
            if let Err(error) =
                netlist_document::select_retained_top_deck(model, netlist, open_documents, deck_id)
            {
                wb.prompt(error);
            }
        }
        NetlistLifecycleAction::CopyIncludeOwnership => {
            match netlist_document::copy_active_dependency_to_project(
                model,
                netlist,
                open_documents,
            ) {
                Ok(_) => {
                    model.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        messages.text(MessageId::NetlistCopySucceeded),
                    );
                }
                Err(error) => {
                    model.record(LogSeverity::Error, LogSource::Studio, error);
                }
            }
        }
        NetlistLifecycleAction::ReleaseIncludeOwnership => {
            match netlist_document::release_active_dependency_from_project(
                model,
                netlist,
                open_documents,
            ) {
                Ok(()) => {
                    model.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        messages.text(MessageId::NetlistReleaseSucceeded),
                    );
                }
                Err(error) => {
                    model.record(LogSeverity::Error, LogSource::Studio, error);
                }
            }
        }
        NetlistLifecycleAction::BeginInclude(file_action) => {
            if let Err(error) =
                netlist_document::begin_owned_include_lifecycle_action(model, netlist, file_action)
            {
                wb.prompt(error);
            }
        }
        NetlistLifecycleAction::CommitInclude => {
            match netlist_document::commit_owned_include_lifecycle_action(
                model,
                netlist,
                open_documents,
            ) {
                Ok(message) => {
                    model.record(LogSeverity::Info, LogSource::Studio, message);
                }
                Err(error) => {
                    if let Some(transaction) = netlist.lifecycle_dialog.include_transaction.as_mut()
                    {
                        transaction.error = Some(error);
                    }
                }
            }
        }
        NetlistLifecycleAction::SaveSourceAs => {
            netlist_document::open_netlist_save_dialog_in(model, netlist, true);
            code.source_document_dialog = false;
        }
    }
    if close {
        close_netlist_lifecycle(code, netlist);
    }
}

/// Create the top deck the creation form drafted, which the form raised,
/// and close the dialog once it is made; a refusal stays in the form's
/// footer.
pub fn commit_new_top_deck(
    code: &mut CodeWorkspaceRuntimeState,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    model: &mut Model,
) {
    match netlist_document::commit_netlist_lifecycle_action(model, netlist, open_documents) {
        Ok(message) => {
            model.record(LogSeverity::Info, LogSource::Studio, message);
            code.source_document_dialog = false;
            netlist.lifecycle_dialog.transaction = None;
        }
        Err(error) => {
            if let Some(transaction) = netlist.lifecycle_dialog.transaction.as_mut() {
                transaction.error = Some(error);
            }
        }
    }
}
