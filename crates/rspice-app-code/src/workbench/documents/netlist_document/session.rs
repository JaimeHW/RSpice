//! What a session keeps of the Netlist workspace: the tabs open, the one in
//! front, and each document's view in its editor. The next launch opens the
//! same tabs, each as it was left; a deck whose file has gone opens as the
//! band that says where the project expected it.

use std::collections::HashSet;
use std::path::Path;

use rspice_app_runs::workbench::slots::runs::RunsState;
use rspice_app_state::workbench::app_state::model::Model;
use serde::{Deserialize, Serialize};

use super::{CompareSide, NetlistDocumentState, NetlistRoot, NetlistTab};
use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{DeckId, NetlistDocumentId};
use crate::workbench::documents::netlist_tabs::RunTarget;
use crate::workbench::documents::virtual_text_editor::EditorView;
use crate::workbench::state::WorkspaceDocumentId;

/// The Netlist workspace as a session keeps it.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NetlistSession {
    /// The project's deck open in the workspace, or the one that could not
    /// be read.
    deck: Option<DeckId>,
    /// Whether the reader had closed the deck's tab.
    deck_tab_closed: bool,
    /// The files open beside the deck, by the path the deck reads them by.
    deck_includes: Vec<String>,
    /// The files open beside the generated netlist, by the name it reads
    /// them by.
    generated_includes: Vec<String>,
    /// The run input open, by run and point.
    run_input: Option<(u64, Option<usize>)>,
    /// The two sides of the comparison open.
    comparison: Option<(CompareSide, CompareSide)>,
    /// The tab in front, once the page has put one there.
    front: Option<NetlistTab>,
    /// The tabs closing the comparison and closing the run input bring back.
    comparison_return: NetlistTab,
    run_input_return: NetlistTab,
    /// The deck Run runs, when Run does not run the plan.
    run_deck: Option<DeckId>,
    /// Each open document's view in its editor.
    views: Vec<(NetlistTab, EditorView)>,
}

impl NetlistSession {
    /// The workspace as `netlist` shows it, with the include tabs in
    /// `open_documents`; `generated` names the generated netlist the
    /// workspace holds, whose includes are kept. A session not yet shown
    /// since it was restored is kept as it was restored.
    #[must_use]
    pub fn of(
        netlist: &NetlistDocumentState,
        open_documents: &HashSet<WorkspaceDocumentId>,
        generated: Option<NetlistDocumentId>,
    ) -> Self {
        if let Some(pending) = &netlist.pending_session {
            return pending.clone();
        }
        let deck_root = netlist.open_deck.as_ref().map(|deck| deck.tab_id());
        let includes_of = |owner: Option<NetlistDocumentId>| {
            let mut paths = open_documents
                .iter()
                .filter_map(|document| match document {
                    WorkspaceDocumentId::NetlistDependency {
                        root,
                        logical_identity,
                    } if owner == Some(*root) => Some(logical_identity.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>();
            paths.sort();
            paths
        };
        let mut session = Self {
            deck: netlist
                .open_deck
                .as_ref()
                .map(|deck| deck.id)
                .or(netlist.unreadable_deck),
            deck_tab_closed: netlist.deck_tab_closed,
            deck_includes: includes_of(deck_root),
            generated_includes: includes_of(generated),
            run_input: netlist.open_run_input,
            comparison: netlist.comparison_sides,
            front: netlist
                .active_document_initialized
                .then(|| netlist.front.clone()),
            comparison_return: netlist.comparison_return.clone(),
            run_input_return: netlist.run_input_return.clone(),
            run_deck: match netlist.run_target() {
                RunTarget::Deck(deck) => Some(deck),
                RunTarget::Plan => None,
            },
            views: Vec::new(),
        };
        // The views of the documents drawn since launch, then those of the
        // documents still to be drawn as the last launch left them.
        for (tab, view) in netlist.editor_views.iter().chain(&netlist.restore_views) {
            if session.keeps(tab) && !session.views.iter().any(|(kept, _)| kept == tab) {
                session.views.push((tab.clone(), view.clone()));
            }
        }
        session
    }

    /// Whether `tab` is open, as this session has the workspace.
    fn keeps(&self, tab: &NetlistTab) -> bool {
        match tab {
            NetlistTab::Generated | NetlistTab::AddedLines => true,
            NetlistTab::Deck(deck) => self.deck == Some(*deck),
            NetlistTab::Include {
                deck: Some(deck),
                path,
            } => self.deck == Some(*deck) && self.deck_includes.contains(path),
            NetlistTab::Include { deck: None, path } => self.generated_includes.contains(path),
            NetlistTab::RunInput { run, point } => self.run_input == Some((*run, *point)),
            // A comparison is a list of lines: there is no editor to keep.
            NetlistTab::Compare { .. } => false,
        }
    }

    /// The view the session keeps of `tab`, for a test to read.
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn view(&self, tab: &NetlistTab) -> Option<&EditorView> {
        self.views
            .iter()
            .find_map(|(kept, view)| (kept == tab).then_some(view))
    }

    /// The tab in front, for a test to read.
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn front(&self) -> Option<&NetlistTab> {
        self.front.as_ref()
    }
}

/// Hold `session` for the Netlist page to reopen the next time it prepares.
pub fn restore(netlist: &mut NetlistDocumentState, session: NetlistSession) {
    netlist.pending_session = (session != NetlistSession::default()).then_some(session);
}

/// Whether a restored session waits for the page to reopen a deck: the
/// page then opens that deck rather than the project's first.
#[must_use]
pub fn restores_a_deck(netlist: &NetlistDocumentState) -> bool {
    netlist
        .pending_session
        .as_ref()
        .is_some_and(|session| session.deck.is_some())
}

/// Reopen the session the page was restored with, once: its deck and the
/// files open beside it and beside the generated netlist, its run input and
/// its comparison, with its tab in front and each document's view handed to
/// its editor. What can no longer be opened is said on the log, and the
/// rest still opens.
pub fn reopen(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
) {
    let Some(session) = netlist.pending_session.take() else {
        return;
    };
    // A deck the reader opened since the launch, before the page was shown,
    // is the reader's choice: it stays in front, and the last launch's tabs
    // give way to it.
    if netlist
        .open_deck
        .as_ref()
        .is_some_and(|open| Some(open.id) != session.deck)
    {
        return;
    }
    let mut refusals = Vec::new();
    if let Some(deck) = session.deck {
        let opened = crate::workbench::documents::deck_files::open_deck_by_id(
            model,
            netlist,
            open_documents,
            runs,
            deck,
        );
        if let Err(refusal) = opened {
            model.record(LogSeverity::Error, LogSource::Netlist, refusal);
        } else {
            for path in &session.deck_includes {
                if let Err(refusal) = crate::workbench::documents::deck_files::show_include(
                    model,
                    netlist,
                    open_documents,
                    runs,
                    Path::new(path),
                ) {
                    refusals.push(refusal);
                }
            }
        }
    }
    for path in &session.generated_includes {
        if let Err(refusal) = super::open_netlist_dependency_from_root_in(
            model,
            netlist,
            open_documents,
            runs,
            NetlistRoot::Generated,
            path,
        ) {
            refusals.push(refusal);
        }
    }
    match session.run_input {
        Some((run, Some(point))) => {
            super::comparison::open_executed_deck_in(model, netlist, runs, run, point);
        }
        Some((_, None)) => {
            super::comparison::open_run_deck_snapshot_in(model, netlist, runs);
        }
        None => {}
    }
    if let Some((left, right)) = session.comparison
        && let Err(refusal) = super::compare(model, netlist, runs, left, right)
    {
        refusals.push(refusal);
    }
    if let Some(front) = &session.front {
        bring_to_front(model, netlist, open_documents, runs, front);
    }
    let open_deck = netlist.open_deck.as_ref().map(|deck| deck.id);
    netlist.deck_tab_closed =
        session.deck_tab_closed && open_deck.is_some() && !netlist.front.shows_deck();
    netlist.run_target = match session.run_deck {
        Some(deck) if open_deck == Some(deck) => RunTarget::Deck(deck),
        _ => RunTarget::Plan,
    };
    netlist.comparison_return = session.comparison_return;
    netlist.run_input_return = session.run_input_return;
    netlist.restore_views = session.views;
    for refusal in refusals {
        model.record(LogSeverity::Warning, LogSource::Netlist, refusal);
    }
}

/// Put `tab` in front as its own route does, when it opened.
fn bring_to_front(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    tab: &NetlistTab,
) {
    match tab {
        NetlistTab::Generated => {
            if !super::open_generated_primary_in(model, netlist) {
                netlist.front = NetlistTab::Generated;
            }
        }
        NetlistTab::AddedLines => {
            super::show_added_lines(model, netlist);
        }
        NetlistTab::Deck(_) => {
            crate::workbench::documents::deck_files::show_deck(model, netlist);
        }
        NetlistTab::Include {
            deck: Some(_),
            path,
        } => {
            let _ = crate::workbench::documents::deck_files::show_include(
                model,
                netlist,
                open_documents,
                runs,
                Path::new(path),
            );
        }
        NetlistTab::Include { deck: None, path } => {
            let _ = super::open_netlist_dependency_from_root_in(
                model,
                netlist,
                open_documents,
                runs,
                NetlistRoot::Generated,
                path,
            );
        }
        NetlistTab::RunInput { run, point } => match point {
            Some(point) => {
                super::comparison::open_executed_deck_in(model, netlist, runs, *run, *point);
            }
            None => {
                super::comparison::open_run_deck_snapshot_in(model, netlist, runs);
            }
        },
        NetlistTab::Compare { .. } => {
            super::comparison::open_netlist_comparison(model, netlist);
        }
    }
    netlist.active_document_initialized = true;
}

/// Hand the editor `editor_id` the view the last launch left `tab` in, the
/// first time it draws `tab`.
pub(super) fn hand_back_view(
    ctx: &egui::Context,
    netlist: &mut NetlistDocumentState,
    tab: &NetlistTab,
    editor_id: egui::Id,
) {
    if let Some(index) = netlist
        .restore_views
        .iter()
        .position(|(kept, _)| kept == tab)
    {
        let (_, view) = netlist.restore_views.swap_remove(index);
        crate::workbench::documents::virtual_text_editor::queue_view(ctx, editor_id, view);
    }
}

/// Keep `view` as the view the editor shows of `tab`.
pub(super) fn keep_view(netlist: &mut NetlistDocumentState, tab: &NetlistTab, view: EditorView) {
    match netlist
        .editor_views
        .iter_mut()
        .find(|(kept, _)| kept == tab)
    {
        Some((_, kept)) => *kept = view,
        None => netlist.editor_views.push((tab.clone(), view)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::code::tests::harness;
    use crate::workbench::documents::deck_files::OpenDeck;

    /// A deck opened after the launch and before the Netlist page was shown
    /// stays open in front: the session the launch restored gives way.
    #[test]
    fn a_deck_opened_since_the_launch_outranks_the_restored_one() {
        let mut harness = harness();
        let opened = OpenDeck::in_memory("C:/p/new.cir", "new\n.end\n");
        let opened_id = opened.id;
        harness.state.netlist.open_deck = Some(opened);
        restore(
            &mut harness.state.netlist,
            NetlistSession {
                deck: Some(DeckId::new()),
                front: Some(NetlistTab::AddedLines),
                ..NetlistSession::default()
            },
        );

        reopen(
            &mut harness.model,
            &mut harness.state.netlist,
            &mut harness.state.netlist_open_documents,
            &mut harness.runs,
        );

        assert_eq!(
            harness.state.netlist.open_deck.as_ref().map(|deck| deck.id),
            Some(opened_id)
        );
        assert_ne!(harness.state.netlist.front, NetlistTab::AddedLines);
        assert!(harness.state.netlist.pending_session.is_none());
    }
}
