//! Read-only comparisons for the netlist editor, and a run's input.
//!
//! A comparison is a view of two texts: the deck against its file on disk,
//! against the last run's input or against the generated netlist, and the
//! generated netlist against its previous build. Each shows the line diff
//! from one to the other on the comparison tab, which nobody edits, and
//! remembers the tab to go back to.

// Through the parent rather than `crate::workbench`: the module's edge onto
// `app_state` is being retired, and a submodule of it does not need its own.
use super::{CompareSide, Model, NetlistDocumentState, NetlistTab};
use crate::workbench::state::WorkbenchSession;
use rspice_app_runs::workbench::slots::runs::RunsState;

/// Reopen the already-materialized comparison document. Creating a new
/// comparison remains an explicit revision workflow; a tab switch is only a
/// presentation transition.
pub fn open_netlist_comparison(model: &mut Model, netlist: &mut NetlistDocumentState) -> bool {
    let Some((left, right)) = netlist.comparison_sides else {
        return false;
    };
    netlist.front = NetlistTab::Compare { left, right };
    netlist.front = netlist.front.root();
    netlist.active_document_initialized = true;
    model.simulation.source.netlist_content = netlist.generated_diff_source.clone();
    netlist.completion_open = false;
    netlist.completion_dismissed_at = None;
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
    true
}

/// Compare the generated netlist's build `index` builds back with the
/// generated netlist now.
pub fn compare_generated_revision(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    runs: &RunsState,
    index: usize,
) -> Result<(), String> {
    compare(
        model,
        netlist,
        runs,
        CompareSide::PreviousGenerated(index),
        CompareSide::Generated,
    )
}

/// Put the comparison of `left` with `right` in front: the line diff from
/// `left`'s text to `right`'s, on the comparison tab. Why there is nothing
/// to compare, when one side has no text.
pub fn compare(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    runs: &RunsState,
    left: CompareSide,
    right: CompareSide,
) -> Result<(), String> {
    let (before, before_label) = side_text(model, netlist, runs, left)?;
    let (after, after_label) = side_text(model, netlist, runs, right)?;
    let diff = similar::TextDiff::from_lines(before.as_str(), after.as_str())
        .unified_diff()
        .context_radius(3)
        .header(&before_label, &after_label)
        .to_string();
    netlist.generated_diff_source = if diff.is_empty() {
        format!("--- {before_label}\n+++ {after_label}\n No changes\n")
    } else {
        diff
    };
    // Closing the view goes back to the tab it was opened from; a view
    // opened over a run's input goes back to the tab that input was opened
    // over, as the input is no one's working text.
    if netlist.front.is_run_input() {
        netlist.comparison_return = netlist.run_input_return.clone();
    } else if !netlist.front.is_comparison() {
        netlist.comparison_return = netlist.front.clone();
    }
    put_comparison_in_front(netlist, left, right);
    netlist.active_document_initialized = true;
    model.simulation.source.netlist_content = netlist.generated_diff_source.clone();
    netlist.requested_line = None;
    netlist.cursor_line = 0;
    netlist.revision = netlist.revision.wrapping_add(1);
    netlist.completion_open = false;
    model.invalidate_source_evidence();
    Ok(())
}

/// The text of one side of a comparison and the words its header names it
/// by, or why it has none.
fn side_text(
    model: &Model,
    netlist: &NetlistDocumentState,
    runs: &RunsState,
    side: CompareSide,
) -> Result<(String, String), String> {
    let open_deck = |id| {
        netlist
            .open_deck
            .as_ref()
            .filter(|deck| deck.id == id)
            .ok_or_else(|| "The deck is no longer open.".to_owned())
    };
    match side {
        CompareSide::Generated => model
            .netlist_artifacts
            .generated_document
            .as_ref()
            .map(|_| {
                (
                    model.netlist_artifacts.generated_source.clone(),
                    "Generated netlist".to_owned(),
                )
            })
            .ok_or_else(|| "There is no generated netlist to compare.".to_owned()),
        CompareSide::PreviousGenerated(index) => netlist
            .generated_history
            .get(index)
            .map(|previous| (previous.source().to_owned(), "Previous build".to_owned()))
            .ok_or_else(|| "That build of the generated netlist is no longer kept.".to_owned()),
        CompareSide::Deck(id) => {
            let deck = open_deck(id)?;
            Ok((deck.root.text.clone(), deck.root.name()))
        }
        CompareSide::DiskFile(id) => {
            let deck = open_deck(id)?;
            Ok((disk_text(deck)?, format!("{} on disk", deck.root.name())))
        }
        CompareSide::RunInput(run) => Ok((
            run_input_text(model, runs, run)?,
            format!("Run {run} input"),
        )),
    }
}

/// The text of the open deck's file as it is on disk now.
#[cfg(not(target_arch = "wasm32"))]
fn disk_text(deck: &super::super::deck_files::OpenDeck) -> Result<String, String> {
    if deck.is_stored() {
        return Err(format!(
            "{} is kept in the project, with no file to compare.",
            deck.root.name()
        ));
    }
    let bytes = std::fs::read(&deck.root.path)
        .map_err(|error| format!("{} cannot be read: {error}", deck.root.name()))?;
    Ok(super::super::deck_files::OpenFile::from_bytes(deck.root.path.clone(), &bytes)?.text)
}

/// A deck in the browser is kept in the project, with no file to read.
#[cfg(target_arch = "wasm32")]
fn disk_text(deck: &super::super::deck_files::OpenDeck) -> Result<String, String> {
    Err(format!(
        "{} is kept in the project, with no file to compare.",
        deck.root.name()
    ))
}

/// The input run `run` read: the deck the last manual run read, or the one
/// point a run of one point read.
fn run_input_text(model: &Model, runs: &RunsState, run: u64) -> Result<String, String> {
    if runs.manual_run.last_run_id == Some(run)
        && let Some(buffer) = runs.manual_run.last_run_buffer.as_ref()
    {
        return Ok(buffer.clone());
    }
    let deck = model
        .simulation
        .retained
        .executed_decks
        .get(run)
        .ok_or_else(|| format!("The input of Run {run} is no longer kept."))?;
    match deck.points.as_slice() {
        [point] => Ok(point.deck.to_string()),
        _ => Err(format!(
            "Run {run} read {} points: open one point's input to compare it.",
            deck.points.len()
        )),
    }
}

/// Put the tab a read-only view was opened from back in front: the Added
/// lines, the deck or the generated netlist.
///
/// The deck is the open deck's text, not the document that was on screen:
/// leaving a view must never overwrite an edit the engineer has not saved
/// yet.
fn return_to_working_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    root: &NetlistTab,
) {
    let deck = netlist
        .open_deck
        .as_ref()
        .map(|deck| (deck.id, deck.root.text.clone()));
    match (root.root(), deck) {
        (NetlistTab::AddedLines, _) if super::show_added_lines(model, netlist) => return,
        (NetlistTab::Deck(id), Some((open, source))) if id == open => {
            netlist.front = NetlistTab::Deck(id);
            netlist.run_target = crate::workbench::documents::netlist_tabs::RunTarget::Deck(id);
            model.simulation.source.netlist_content = source;
        }
        _ => {
            netlist.front = NetlistTab::Generated;
            netlist.run_the_plan();
            model.simulation.source.netlist_content =
                model.netlist_artifacts.generated_source.clone();
        }
    }
    netlist.active_document_initialized = true;
    netlist.completion_open = false;
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
}

pub fn close_revision_comparison(model: &mut Model, netlist: &mut NetlistDocumentState) -> bool {
    let root = netlist.comparison_return.clone();
    return_to_working_deck(model, netlist, &root);
    netlist.generated_diff_source.clear();
    netlist.comparison_sides = None;
    netlist.comparison_include = None;
    true
}

/// Put the comparison of `left` to `right` in front, as the comparison tab.
fn put_comparison_in_front(
    netlist: &mut NetlistDocumentState,
    left: CompareSide,
    right: CompareSide,
) {
    netlist.comparison_sides = Some((left, right));
    netlist.comparison_include = None;
    netlist.front = NetlistTab::Compare { left, right };
}

/// The run a reopenable deck snapshot belongs to.
///
/// Both halves are required: a run binding without its sealed text is a
/// snapshot that cannot show a deck, and text without a run is a deck that
/// cannot name what executed it. It reads the runs.
pub fn run_deck_snapshot_run_id_in(runs: &RunsState) -> Option<u64> {
    let run_id = runs.manual_run.last_run_id?;
    runs.manual_run
        .last_run_buffer
        .as_ref()
        .is_some_and(|buffer| !buffer.is_empty())
        .then_some(run_id)
}

/// The synthetic filename the run snapshot is filed under. It is an
/// identifier, not translated copy, and it names the run so two snapshots from
/// different runs can never be mistaken for one document.
pub fn run_deck_snapshot_artifact_name_in(runs: &RunsState) -> String {
    format!("run-{}.sp", runs.manual_run.last_run_id.unwrap_or_default())
}

/// Open the deck a completed manual run consumed, pinned to that run.
///
/// The text is the sealed source the dispatch captured, not a re-derivation
/// from the live editor, so what this shows is what the engine read. It
/// writes the model and the netlist editor's state, and reads the runs that
/// sealed the deck.
pub fn open_run_deck_snapshot_in(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    runs: &RunsState,
) -> bool {
    if run_deck_snapshot_run_id_in(runs).is_none() {
        return false;
    }
    netlist.executed_deck_view = None;
    project_run_snapshot_in(model, netlist, runs);
    true
}

/// Open the source one point of a completed run was actually handed.
///
/// Any run, not only a manual one: the deck a corner point solved is the only
/// artifact that settles what that point solved, and every run seals one. The
/// text comes out of the session's executed-deck archive, so a run whose deck
/// this session no longer holds cannot be opened — and says so by refusing,
/// rather than by opening the working deck, which is a different document.
///
/// Read-only is a property of the tab, not of this route: the editor never
/// makes a run input editable and no save action accepts one.
///
/// The deck is checked against its run's receipt here, once, and the verdict
/// travels with the selection. Opening is the only moment the answer can
/// change, because both of its inputs are sealed. It writes the model and the
/// netlist editor's state, and reads the runs that sealed the deck.
pub fn open_executed_deck_in(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    runs: &RunsState,
    run_id: u64,
    point: usize,
) -> bool {
    let Some(deck) = model.simulation.retained.executed_decks.get(run_id) else {
        return false;
    };
    let point = point.min(deck.points.len().saturating_sub(1));
    let Some(verification) =
        crate::workbench::documents::netlist_document::ExecutedDeckVerification::of(
            model, run_id, point,
        )
    else {
        return false;
    };
    netlist.executed_deck_view = Some(
        crate::workbench::documents::netlist_document::ExecutedDeckSelection {
            run_id,
            point,
            verification,
        },
    );
    project_run_snapshot_in(model, netlist, runs);
    true
}

/// Open one point's executed deck and put the reader in front of it.
///
/// Callers ask this from the Verify workspace's corners row, from a run's
/// receipt in the inspector, and from the code workspace's own run strip, and
/// all of them are somewhere else when they ask — so the workspace change
/// belongs here rather than at each call site, where one of them would
/// eventually forget it.
///
/// # What a row elsewhere needs to offer this
///
/// Two things, and nothing more. The **run sequence** — `SimulationRun::id`,
/// not [`crate::product::RunId`] — because that is what the executed-deck
/// archive is keyed on; a surface holding a stable run identity resolves it
/// through the run it already has in hand. And the **point index**, which is a
/// position in [`crate::state::ExecutedDeck::points`]; `0` is the run's first
/// dispatched task and an index past the end resolves back into range rather
/// than opening nothing.
///
/// Whether to offer the route at all is
/// `state.simulation.executed_decks.get(sequence)`: `None` means this project
/// holds no deck for that run, and the row should say so rather than offer a
/// control that refuses. The same lookup gives `points.len()`, which is how
/// many tasks a row can step through.
///
/// This returns `false` in exactly that case, so a caller that offers the
/// route unconditionally still cannot open a deck that is not held.
pub fn reveal_executed_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    runs: &RunsState,
    workbench: &mut WorkbenchSession,
    run_id: u64,
    point: usize,
) -> bool {
    if !open_executed_deck_in(model, netlist, runs, run_id, point) {
        return false;
    }
    workbench.activate(crate::workbench::state::Workspace::Netlist);
    true
}

/// Projects whatever the run-snapshot document is currently bound to, over
/// the model, the netlist editor's state and the runs that sealed the deck.
fn project_run_snapshot_in(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    runs: &RunsState,
) {
    if !netlist.front.is_run_input() {
        netlist.run_input_return = netlist.front.clone();
    }
    let source = super::run_snapshot_source_in(model, netlist, runs);
    let (run, point) = match netlist.executed_deck_view {
        Some(selection) => (selection.run_id, Some(selection.point)),
        None => (runs.manual_run.last_run_id.unwrap_or_default(), None),
    };
    netlist.front = NetlistTab::RunInput { run, point };
    netlist.open_run_input = Some((run, point));
    netlist.active_document_initialized = true;
    model.simulation.source.netlist_content = source;
    netlist.requested_line = None;
    netlist.cursor_line = 0;
    netlist.completion_open = false;
    netlist.completion_dismissed_at = None;
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
}

/// Leave the run snapshot for the working deck it was opened from.
pub fn close_run_deck_snapshot(model: &mut Model, netlist: &mut NetlistDocumentState) -> bool {
    if !netlist.front.is_run_input() {
        return false;
    }
    netlist.executed_deck_view = None;
    netlist.open_run_input = None;
    let root = netlist.run_input_return.clone();
    return_to_working_deck(model, netlist, &root);
    true
}

/// Compare the last manual run's input with the deck or generated netlist
/// the input was opened over, else with the one in front. It writes the
/// model and the netlist editor's state, and reads the runs.
pub fn compare_run_deck_snapshot_in(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    runs: &RunsState,
) -> Result<(), String> {
    if netlist.executed_deck_view.is_some() {
        // Not a refusal to be worked around: an executed deck is the source a
        // point was handed after expansion, corner materialization and
        // per-point overrides. Diffing it against the deck somebody is editing
        // would report every one of those as an edit.
        return Err(
            "An executed deck has no working copy to compare against. Open the deck this \
             session last ran manually to compare that."
                .to_owned(),
        );
    }
    let run_id = run_deck_snapshot_run_id_in(runs).ok_or_else(|| {
        "No manual run has finished yet: there is no run input to compare.".to_owned()
    })?;
    // The deck or generated netlist the run input was opened over, else the
    // one in front.
    let working = if netlist.front.is_run_input() {
        netlist.run_input_return.root()
    } else {
        netlist.front.root()
    };
    let right = match working {
        NetlistTab::Deck(id) => CompareSide::Deck(id),
        _ => CompareSide::Generated,
    };
    compare(model, netlist, runs, CompareSide::RunInput(run_id), right)
}
