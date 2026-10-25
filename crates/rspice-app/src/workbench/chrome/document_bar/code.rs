//! The document bar's Netlist tabs: the generated and owned netlists with
//! their open includes, the comparison and the run's deck, or the one
//! document a code page shows; which of them is active; and their activation.

use crate::workbench::AppState;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::documents::netlist_document::run_deck_snapshot_run_id;
use crate::workbench::state::WorkspaceDocumentId;

use crate::workbench::state::workbench_session::WorkspaceDocument;

pub(in crate::workbench) fn netlist_workspace_documents(
    state: &AppState,
) -> Vec<WorkspaceDocument> {
    use crate::workbench::documents::code_workspace::CodeWorkspacePage;

    if state.code.code_workspace.page != CodeWorkspacePage::Netlist {
        return vec![code_workspace_document(state)];
    }
    let mut documents = Vec::new();
    if let Some(generated) = state.model.netlist_artifacts.generated_document.as_ref() {
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
            if !state.code.netlist_open_documents.contains(&id) {
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

    if let Some(owned) = state.code.netlist.owned_document.as_ref() {
        let owned_label = state
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .map(|descriptor| descriptor.artifact_name.clone())
            .filter(|label| !label.trim().is_empty())
            .unwrap_or_else(|| "Project netlist".to_owned());
        documents.push(WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistOwned(owned.id()),
            label: owned_label,
            icon: WorkbenchIcon::Netlist,
            dirty: state.model.workspace.content.netlist_source_dirty,
        });
        documents.extend(owned.dependencies().iter().filter_map(|dependency| {
            dependency.source()?;
            let identity = dependency.locator().logical_identity();
            let id = WorkspaceDocumentId::NetlistDependency {
                root: owned.id(),
                logical_identity: identity.to_owned(),
            };
            if !state.code.netlist_open_documents.contains(&id) {
                return None;
            }
            let project_owned = state
                .model
                .workspace
                .content
                .netlist_descriptor
                .as_ref()
                .and_then(|descriptor| descriptor.owned_include(identity))
                .is_some();
            Some(WorkspaceDocument {
                id,
                label: if project_owned {
                    dependency.locator().display_name().to_owned()
                } else {
                    format!("{} · external", dependency.locator().display_name())
                },
                icon: WorkbenchIcon::Code,
                dirty: project_owned && state.model.workspace.content.netlist_source_dirty,
            })
        }));
    }

    if !state.code.netlist.generated_diff_source.is_empty() {
        documents.push(WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistComparison,
            label: "Netlist comparison".to_owned(),
            icon: WorkbenchIcon::Code,
            dirty: false,
        });
    }
    if let Some(run_id) = run_deck_snapshot_run_id(state) {
        documents.push(WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistRunSnapshot,
            label: state.workbench.messages().format(
                crate::workbench::MessageId::NetlistRunSnapshotDocument,
                &[("id", &run_id.to_string())],
            ),
            icon: WorkbenchIcon::Results,
            dirty: false,
        });
    }
    if documents.is_empty() {
        documents.push(code_workspace_document(state));
    }
    documents
}

/// The Code & Automation workspace shows one document at a time and the page
/// decides which. Only the netlist page has a dirty bit to report: Automation
/// sources are a project-owned bundle whose dirtiness belongs to the bundle,
/// not to this tab.
fn code_workspace_document(state: &AppState) -> WorkspaceDocument {
    use crate::workbench::documents::code_workspace::CodeWorkspacePage;

    let page = state.code.code_workspace.page;
    WorkspaceDocument {
        id: WorkspaceDocumentId::NetlistSource,
        label: crate::workbench::documents::code_workspace::active_document_label(
            &state.model.workspace,
            &state.code.code_workspace,
            state.workbench.messages(),
        ),
        icon: match page {
            CodeWorkspacePage::Netlist => WorkbenchIcon::Netlist,
            CodeWorkspacePage::Automation => WorkbenchIcon::Terminal,
        },
        dirty: page == CodeWorkspacePage::Netlist
            && state.model.workspace.content.netlist_source_dirty,
    }
}

pub(in crate::workbench) fn active_netlist_document_id(
    state: &AppState,
) -> Option<WorkspaceDocumentId> {
    use crate::workbench::documents::code_workspace::CodeWorkspacePage;
    use crate::workbench::documents::netlist_document::ActiveNetlistDocument;

    if state.code.code_workspace.page != CodeWorkspacePage::Netlist {
        return Some(WorkspaceDocumentId::NetlistSource);
    }
    let netlist = &state.code.netlist;
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
        ActiveNetlistDocument::Generated => {
            state.model.netlist_artifacts.generated_document.as_ref()
        }
        ActiveNetlistDocument::OwnedSource => netlist.owned_document.as_ref(),
        ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => None,
    }?;
    if let Some(logical_identity) = netlist.active_dependency_identity.as_ref() {
        Some(WorkspaceDocumentId::NetlistDependency {
            root: document.id(),
            logical_identity: logical_identity.clone(),
        })
    } else {
        Some(match root {
            ActiveNetlistDocument::Generated => {
                WorkspaceDocumentId::NetlistGenerated(document.id())
            }
            ActiveNetlistDocument::OwnedSource => WorkspaceDocumentId::NetlistOwned(document.id()),
            ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => {
                return None;
            }
        })
    }
}

/// Show the generated netlist, when it is the one this tab names.
pub(in crate::workbench) fn activate_generated_netlist(
    state: &mut AppState,
    document_id: &crate::state::NetlistDocumentId,
) -> bool {
    let available = state
        .model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .is_some_and(|document| document.id() == *document_id);
    available
        && ((state.code.netlist.active_document
            == crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated
            && state.code.netlist.active_dependency_identity.is_none())
            || crate::workbench::documents::netlist_document::open_generated_primary(state))
}

/// Show the project's own netlist, when it is the one this tab names.
pub(in crate::workbench) fn activate_owned_netlist(
    state: &mut AppState,
    document_id: &crate::state::NetlistDocumentId,
) -> bool {
    let available = state
        .code
        .netlist
        .owned_document
        .as_ref()
        .is_some_and(|document| document.id() == *document_id);
    available && crate::workbench::documents::netlist_document::open_owned_primary(state)
}

/// Show an include of the generated or the owned netlist.
pub(in crate::workbench) fn activate_netlist_dependency(
    state: &mut AppState,
    root: &crate::state::NetlistDocumentId,
    logical_identity: &str,
) -> bool {
    use crate::workbench::documents::netlist_document::ActiveNetlistDocument;
    let root_kind = if state
        .model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .is_some_and(|document| document.id() == *root)
    {
        Some(ActiveNetlistDocument::Generated)
    } else if state
        .code
        .netlist
        .owned_document
        .as_ref()
        .is_some_and(|document| document.id() == *root)
    {
        Some(ActiveNetlistDocument::OwnedSource)
    } else {
        None
    };
    root_kind.is_some_and(|root_kind| {
        crate::workbench::documents::netlist_document::open_netlist_dependency_from_root(
            state,
            root_kind,
            logical_identity,
        )
        .is_ok()
    })
}

/// Show the generated netlist's comparison.
pub(in crate::workbench) fn activate_netlist_comparison(state: &mut AppState) -> bool {
    crate::workbench::documents::netlist_document::open_netlist_comparison(state)
}

/// Show the deck the last manual run consumed.
pub(in crate::workbench) fn activate_netlist_run_snapshot(state: &mut AppState) -> bool {
    crate::workbench::documents::netlist_document::open_run_deck_snapshot(state)
}
