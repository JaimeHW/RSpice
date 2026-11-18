//! The document bar's Netlist tabs: the generated and owned netlists with
//! their open includes, the comparison and the run's deck; which of them is
//! active; and their activation.

use std::collections::HashSet;

use crate::workbench::MessageCatalog;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::documents::netlist_document::{
    ActiveNetlistDocument, NetlistDocumentState, run_deck_snapshot_run_id_in,
};
use crate::workbench::state::WorkspaceDocumentId;
use rspice_app_runs::workbench::slots::runs::RunsState;
use rspice_app_state::workbench::app_state::model::Model;

use crate::workbench::state::workbench_session::WorkspaceDocument;

/// What the Netlist tabs read: the model, the netlist editor's documents,
/// the includes the reader opened, the runs that sealed a deck and the
/// message catalogue.
#[derive(Clone, Copy)]
pub struct NetlistTabsReads<'a> {
    pub(crate) model: &'a Model,
    pub(crate) netlist: &'a NetlistDocumentState,
    pub(crate) open_documents: &'a HashSet<WorkspaceDocumentId>,
    pub(crate) runs: &'a RunsState,
    pub(crate) messages: MessageCatalog,
}

pub fn netlist_workspace_documents(reads: NetlistTabsReads<'_>) -> Vec<WorkspaceDocument> {
    let NetlistTabsReads {
        model,
        netlist,
        open_documents,
        runs,
        messages,
    } = reads;
    let mut documents = Vec::new();
    if let Some(generated) = model.netlist_artifacts.generated_document.as_ref() {
        documents.push(WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistGenerated(generated.id()),
            label: "Generated netlist".to_owned(),
            icon: WorkbenchIcon::Netlist,
            dirty: false,
        });
        documents.extend(generated.dependencies().iter().filter_map(|dependency| {
            dependency.source()?;
            let id = WorkspaceDocumentId::NetlistDependency {
                root: generated.id(),
                logical_identity: dependency.locator().logical_identity().to_owned(),
            };
            if !open_documents.contains(&id) {
                return None;
            }
            Some(WorkspaceDocument {
                id,
                label: format!("{} · generated", dependency.locator().display_name()),
                icon: WorkbenchIcon::Code,
                dirty: false,
            })
        }));
    }

    if let Some(deck) = netlist.open_deck.as_ref() {
        let root = deck.tab_id();
        documents.push(WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistOwned(root),
            label: deck.root.name(),
            icon: WorkbenchIcon::Netlist,
            dirty: deck.root.dirty,
        });
        documents.extend(deck.includes.iter().filter_map(|file| {
            let id = WorkspaceDocumentId::NetlistDependency {
                root,
                logical_identity: file.key(),
            };
            open_documents.contains(&id).then(|| WorkspaceDocument {
                id,
                label: file.name(),
                icon: WorkbenchIcon::Code,
                dirty: file.dirty,
            })
        }));
    }

    if !netlist.generated_diff_source.is_empty() {
        documents.push(WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistComparison,
            label: "Netlist comparison".to_owned(),
            icon: WorkbenchIcon::Code,
            dirty: false,
        });
    }
    if let Some(run_id) = run_deck_snapshot_run_id_in(runs) {
        documents.push(WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistRunSnapshot,
            label: messages.format(
                crate::workbench::MessageId::NetlistRunSnapshotDocument,
                &[("id", &run_id.to_string())],
            ),
            icon: WorkbenchIcon::Results,
            dirty: false,
        });
    }
    if documents.is_empty() {
        documents.push(code_workspace_document(model));
    }
    documents
}

/// The deck's own tab, when the Netlist workspace has no other document to
/// list.
fn code_workspace_document(model: &Model) -> WorkspaceDocument {
    WorkspaceDocument {
        id: WorkspaceDocumentId::NetlistSource,
        label: crate::workbench::documents::code_workspace::active_document_label(&model.workspace),
        icon: WorkbenchIcon::Netlist,
        dirty: model.workspace.content.decks_dirty,
    }
}

pub fn active_netlist_document_id(
    model: &Model,
    netlist: &NetlistDocumentState,
) -> Option<WorkspaceDocumentId> {
    if netlist.active_document == ActiveNetlistDocument::GeneratedDiff {
        return Some(WorkspaceDocumentId::NetlistComparison);
    }
    if netlist.active_document == ActiveNetlistDocument::RunSnapshot {
        return Some(WorkspaceDocumentId::NetlistRunSnapshot);
    }
    let root = netlist
        .active_dependency_root
        .unwrap_or(netlist.active_document);
    let document = match root {
        ActiveNetlistDocument::Generated => model
            .netlist_artifacts
            .generated_document
            .as_ref()
            .map(|document| document.id()),
        ActiveNetlistDocument::OwnedSource => netlist.open_deck.as_ref().map(|deck| deck.tab_id()),
        ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => None,
    }?;
    if let Some(logical_identity) = netlist.active_dependency_identity.as_ref() {
        Some(WorkspaceDocumentId::NetlistDependency {
            root: document,
            logical_identity: logical_identity.clone(),
        })
    } else {
        Some(match root {
            ActiveNetlistDocument::Generated => WorkspaceDocumentId::NetlistGenerated(document),
            ActiveNetlistDocument::OwnedSource => WorkspaceDocumentId::NetlistOwned(document),
            ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => {
                return None;
            }
        })
    }
}

/// Show the generated netlist, when it is the one this tab names.
pub fn activate_generated_netlist(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    document_id: &crate::state::NetlistDocumentId,
) -> bool {
    let available = model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .is_some_and(|document| document.id() == *document_id);
    available
        && ((netlist.active_document == ActiveNetlistDocument::Generated
            && netlist.active_dependency_identity.is_none())
            || crate::workbench::documents::netlist_document::open_generated_primary_in(
                model, netlist,
            ))
}

/// Show the open deck, when it is the one this tab names.
pub fn activate_owned_netlist(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    document_id: &crate::state::NetlistDocumentId,
) -> bool {
    let available = netlist
        .open_deck
        .as_ref()
        .is_some_and(|deck| deck.tab_id() == *document_id);
    available && crate::workbench::documents::deck_files::show_deck(model, netlist)
}

/// Show an include of the generated netlist or of the open deck, as a tab
/// the reader opened, leaving the manual run's edited lines behind.
pub fn activate_netlist_dependency(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    root: &crate::state::NetlistDocumentId,
    logical_identity: &str,
) -> bool {
    let root_kind = if model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .is_some_and(|document| document.id() == *root)
    {
        Some(ActiveNetlistDocument::Generated)
    } else if netlist
        .open_deck
        .as_ref()
        .is_some_and(|deck| deck.tab_id() == *root)
    {
        Some(ActiveNetlistDocument::OwnedSource)
    } else {
        None
    };
    root_kind.is_some_and(|root_kind| {
        crate::workbench::documents::netlist_document::open_netlist_dependency_from_root_in(
            model,
            netlist,
            open_documents,
            runs,
            root_kind,
            logical_identity,
        )
        .is_ok()
    })
}

/// Show the generated netlist's comparison.
pub fn activate_netlist_comparison(model: &mut Model, netlist: &mut NetlistDocumentState) -> bool {
    crate::workbench::documents::netlist_document::open_netlist_comparison(model, netlist)
}

/// Show the deck the last manual run consumed.
pub fn activate_netlist_run_snapshot(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    runs: &RunsState,
) -> bool {
    crate::workbench::documents::netlist_document::open_run_deck_snapshot_in(model, netlist, runs)
}

/// Close an include tab: the reader no longer has it open. The tab's
/// document stays in its root's source closure.
pub fn close_netlist_dependency(
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    document: &WorkspaceDocumentId,
) {
    open_documents.remove(document);
}
