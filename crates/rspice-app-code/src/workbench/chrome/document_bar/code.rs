//! The document bar's Netlist tabs: the generated and owned netlists with
//! their open includes, the comparison and the run's deck; which of them is
//! active; and their activation.

use std::collections::HashSet;

use crate::workbench::MessageCatalog;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::documents::netlist_document::{
    CompareSide, NetlistDocumentState, NetlistRoot, NetlistTab,
};
use crate::workbench::state::WorkspaceDocumentId;
use rspice_app_runs::workbench::slots::runs::RunsState;
use rspice_app_state::workbench::app_state::model::Model;

use crate::workbench::state::workbench_session::WorkspaceDocument;

/// What the Netlist tabs read: the model, the netlist editor's documents,
/// the includes the reader opened and the message catalogue.
#[derive(Clone, Copy)]
pub struct NetlistTabsReads<'a> {
    pub(crate) model: &'a Model,
    pub(crate) netlist: &'a NetlistDocumentState,
    pub(crate) open_documents: &'a HashSet<WorkspaceDocumentId>,
    pub(crate) messages: MessageCatalog,
}

/// The Netlist page's tabs, in order: the generated netlist and the Added
/// lines (pinned while there is a design to netlist), each include of the
/// generated netlist the reader opened, the deck and its open includes, the
/// open comparison and the open run input.
pub fn netlist_workspace_documents(reads: NetlistTabsReads<'_>) -> Vec<WorkspaceDocument> {
    let NetlistTabsReads {
        model,
        netlist,
        open_documents,
        messages,
    } = reads;
    let mut documents = Vec::new();
    if let Some(generated) = model.netlist_artifacts.generated_document.as_ref() {
        documents.push(WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistGenerated(generated.id()),
            label: messages.text(crate::workbench::MessageId::CodeGeneratedNetlist),
            icon: WorkbenchIcon::Netlist,
            dirty: false,
        });
        documents.push(WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistAddedLines,
            label: messages.text(crate::workbench::MessageId::NetlistAddedLinesDocument),
            icon: WorkbenchIcon::Code,
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
                label: messages.format(
                    crate::workbench::MessageId::NetlistGeneratedInclude,
                    &[("name", dependency.locator().display_name())],
                ),
                icon: WorkbenchIcon::Code,
                dirty: false,
            })
        }));
    }

    if let Some(deck) = netlist
        .open_deck
        .as_ref()
        .filter(|_| !netlist.deck_tab_closed)
    {
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

    if let Some((left, right)) = netlist.comparison_sides {
        documents.push(WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistComparison,
            label: comparison_label(model, netlist, messages, left, right),
            icon: WorkbenchIcon::Compare,
            dirty: false,
        });
    }
    if let Some((run, point)) = netlist.open_run_input {
        documents.push(WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistRunInput { run, point },
            label: run_input_label(model, messages, run, point),
            icon: WorkbenchIcon::Results,
            dirty: false,
        });
    }
    if documents.is_empty() {
        documents.push(code_workspace_document(model));
    }
    documents
}

/// A comparison's tab label, its two sides: "amp_tb.cir on disk vs
/// amp_tb.cir", "Run 12 input vs amp_tb.cir", "Previous build vs Generated
/// netlist".
fn comparison_label(
    model: &Model,
    netlist: &NetlistDocumentState,
    messages: MessageCatalog,
    left: CompareSide,
    right: CompareSide,
) -> String {
    use crate::workbench::MessageId;
    let deck_name = |id: crate::state::DeckId| {
        netlist
            .open_deck
            .as_ref()
            .filter(|deck| deck.id == id)
            .map(|deck| deck.root.name())
            .or_else(|| {
                model
                    .workspace
                    .content
                    .decks
                    .iter()
                    .find(|deck| deck.id == id)
                    .map(crate::state::ProjectDeck::name)
            })
            .unwrap_or_default()
    };
    // A comparison of one of the deck's includes with its file names the
    // include, not the deck that reads it.
    let file_name = |id: crate::state::DeckId| {
        netlist
            .comparison_include
            .as_deref()
            .and_then(|key| {
                netlist
                    .open_deck
                    .as_ref()
                    .filter(|deck| deck.id == id)?
                    .include(key)
            })
            .map_or_else(|| deck_name(id), |file| file.name())
    };
    let side = |side: CompareSide| match side {
        CompareSide::Generated => messages.text(MessageId::CodeGeneratedNetlist),
        CompareSide::PreviousGenerated(_) => messages.text(MessageId::NetlistComparePreviousBuild),
        CompareSide::Deck(id) => file_name(id),
        CompareSide::DiskFile(id) => {
            messages.format(MessageId::NetlistCompareOnDisk, &[("name", &file_name(id))])
        }
        CompareSide::RunInput(run) => messages.format(
            MessageId::NetlistRunSnapshotDocument,
            &[("id", &run.to_string())],
        ),
    };
    messages.format(
        MessageId::NetlistComparisonDocument,
        &[("left", &side(left)), ("right", &side(right))],
    )
}

/// A run input's tab label: "Run 12 input" for the deck a manual run read
/// or a run of one point, "Run 12 · point 3/9" for one point of several.
fn run_input_label(
    model: &Model,
    messages: MessageCatalog,
    run: u64,
    point: Option<usize>,
) -> String {
    let id = run.to_string();
    let count = model
        .simulation
        .retained
        .executed_decks
        .get(run)
        .map_or(0, |deck| deck.points.len());
    match point {
        Some(point) if count > 1 => messages.format(
            crate::workbench::MessageId::NetlistRunInputPointDocument,
            &[
                ("id", &id),
                ("point", &(point + 1).to_string()),
                ("count", &count.to_string()),
            ],
        ),
        _ => messages.format(
            crate::workbench::MessageId::NetlistRunSnapshotDocument,
            &[("id", &id)],
        ),
    }
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
    let generated = || {
        model
            .netlist_artifacts
            .generated_document
            .as_ref()
            .map(|document| document.id())
    };
    let deck = |id: crate::state::DeckId| {
        netlist
            .open_deck
            .as_ref()
            .filter(|deck| deck.id == id)
            .map(|deck| deck.tab_id())
    };
    Some(match &netlist.front {
        NetlistTab::Generated => WorkspaceDocumentId::NetlistGenerated(generated()?),
        NetlistTab::AddedLines => WorkspaceDocumentId::NetlistAddedLines,
        NetlistTab::Deck(id) => WorkspaceDocumentId::NetlistOwned(deck(*id)?),
        NetlistTab::Include { deck: root, path } => WorkspaceDocumentId::NetlistDependency {
            root: match root {
                Some(id) => deck(*id)?,
                None => generated()?,
            },
            logical_identity: path.clone(),
        },
        NetlistTab::RunInput { run, point } => WorkspaceDocumentId::NetlistRunInput {
            run: *run,
            point: *point,
        },
        NetlistTab::Compare { .. } => WorkspaceDocumentId::NetlistComparison,
    })
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
        && ((netlist.front.shows_generated() && netlist.front.include().is_none())
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
        Some(NetlistRoot::Generated)
    } else if !netlist.deck_tab_closed
        && netlist
            .open_deck
            .as_ref()
            .is_some_and(|deck| deck.tab_id() == *root)
    {
        Some(NetlistRoot::Deck)
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

/// Show the Added lines of the plan's root design.
pub fn activate_added_lines(model: &mut Model, netlist: &mut NetlistDocumentState) -> bool {
    crate::workbench::documents::netlist_document::show_added_lines(model, netlist)
}

/// Show the input run `run` executed, for `point` of it or, with no point,
/// the deck a manual run read.
pub fn activate_netlist_run_input(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    runs: &RunsState,
    run: u64,
    point: Option<usize>,
) -> bool {
    match point {
        Some(point) => crate::workbench::documents::netlist_document::open_executed_deck_in(
            model, netlist, runs, run, point,
        ),
        None => crate::workbench::documents::netlist_document::open_run_deck_snapshot_in(
            model, netlist, runs,
        ),
    }
}

/// Close the deck's tab and its includes' tabs, and put the generated
/// netlist in front when the deck or one of its includes was: the plan is
/// what Run runs again. The deck stays open, with any edit not yet saved,
/// until it is shown again.
pub fn close_netlist_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
) {
    let Some(deck) = netlist.open_deck.as_ref() else {
        return;
    };
    let root = deck.tab_id();
    open_documents.retain(|tab| {
        !matches!(tab, WorkspaceDocumentId::NetlistDependency { root: of, .. } if *of == root)
    });
    netlist.deck_tab_closed = true;
    netlist.run_the_plan();
    if matches!(
        netlist.front,
        NetlistTab::Deck(_) | NetlistTab::Include { deck: Some(_), .. }
    ) {
        crate::workbench::documents::netlist_document::open_generated_primary_in(model, netlist);
    }
}

/// Close the run input's tab, and leave it when it is in front.
pub fn close_netlist_run_input(model: &mut Model, netlist: &mut NetlistDocumentState) {
    if !crate::workbench::documents::netlist_document::close_run_deck_snapshot(model, netlist) {
        netlist.open_run_input = None;
        netlist.executed_deck_view = None;
    }
}

/// Close the comparison's tab, and leave it when it is in front.
pub fn close_netlist_comparison(model: &mut Model, netlist: &mut NetlistDocumentState) {
    if netlist.front.is_comparison() {
        crate::workbench::documents::netlist_document::close_revision_comparison(model, netlist);
    } else {
        netlist.generated_diff_source.clear();
        netlist.comparison_sides = None;
        netlist.comparison_include = None;
    }
}

/// Close an include tab: the reader no longer has it open. The tab's
/// document stays in its root's source closure.
pub fn close_netlist_dependency(
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    document: &WorkspaceDocumentId,
) {
    open_documents.remove(document);
}
