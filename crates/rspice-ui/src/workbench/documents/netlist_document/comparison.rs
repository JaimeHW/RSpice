//! Immutable revision comparison and restore for the netlist deck.
//!
//! Comparing two retained revisions, restoring one as a new working revision,
//! and the read-only documents those transactions project are one concern:
//! each of them replaces the visible source with text nobody may edit and
//! remembers which deck to return to. The deck a completed run consumed is
//! the third such document and lives beside them here.

// Through the parent rather than `crate::workbench`: the module's edge onto
// `app_state` is being retired, and a submodule of it does not need its own.
use super::{ActiveNetlistDocument, AppState, invalidate_source_evidence};

/// Reopen the already-materialized comparison document. Creating a new
/// comparison remains an explicit revision workflow; a tab switch is only a
/// presentation transition.
pub(crate) fn open_netlist_comparison(state: &mut AppState) -> bool {
    if state.ui.netlist.generated_diff_source.is_empty() {
        return false;
    }
    state.ui.netlist.active_document = ActiveNetlistDocument::GeneratedDiff;
    state.ui.netlist.active_dependency_identity = None;
    state.ui.netlist.active_dependency_root = None;
    state.ui.netlist.active_document_initialized = true;
    state.simulation.netlist_content = state.ui.netlist.generated_diff_source.clone();
    state.ui.netlist.completion_open = false;
    state.ui.netlist.completion_dismissed_at = None;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut state.ui.netlist);
    true
}

pub fn compare_generated_revision(state: &mut AppState, index: usize) -> Result<(), String> {
    let previous = state
        .ui
        .netlist
        .generated_history
        .get(index)
        .ok_or_else(|| "The selected generated revision is no longer retained.".to_owned())?;
    let current = state
        .ui
        .netlist
        .generated_document
        .as_ref()
        .and_then(crate::state::NetlistDocument::generated_artifact)
        .ok_or_else(|| "No current generated artifact is available.".to_owned())?;
    let previous_label = format!("generated-{}", short_digest(previous.content_digest()));
    let current_label = format!("generated-{}", short_digest(current.content_digest()));
    let diff = similar::TextDiff::from_lines(previous.source(), current.source())
        .unified_diff()
        .context_radius(3)
        .header(&previous_label, &current_label)
        .to_string();
    state.ui.netlist.generated_diff_source = if diff.is_empty() {
        format!("--- {previous_label}\n+++ {current_label}\n No source changes\n")
    } else {
        diff
    };
    state.ui.netlist.comparison_return_document = ActiveNetlistDocument::Generated;
    state.ui.netlist.active_dependency_identity = None;
    state.ui.netlist.active_dependency_root = None;
    state.ui.netlist.active_document = ActiveNetlistDocument::GeneratedDiff;
    state.ui.netlist.active_document_initialized = true;
    state.simulation.netlist_content = state.ui.netlist.generated_diff_source.clone();
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    state.ui.netlist.completion_open = false;
    invalidate_source_evidence(&mut state.ui.netlist);
    Ok(())
}

pub fn compare_owned_revision(state: &mut AppState, index: usize) -> Result<(), String> {
    let snapshot = state
        .workspace
        .netlist_descriptor
        .as_ref()
        .and_then(|descriptor| descriptor.revision_history.get(index))
        .ok_or_else(|| "The selected owned-source revision is no longer retained.".to_owned())?;
    let current = state
        .ui
        .netlist
        .owned_document
        .as_ref()
        .ok_or_else(|| "No current owned source document is available.".to_owned())?;
    let previous_label = format!(
        "owned-r{}-{}",
        snapshot.document_revision,
        short_digest(snapshot.content_digest)
    );
    let current_label = format!(
        "owned-r{}-{}",
        current.revision().get(),
        short_digest(current.content_digest())
    );
    let diff = similar::TextDiff::from_lines(snapshot.source.as_str(), current.source())
        .unified_diff()
        .context_radius(3)
        .header(&previous_label, &current_label)
        .to_string();
    state.ui.netlist.generated_diff_source = if diff.is_empty() {
        format!("--- {previous_label}\n+++ {current_label}\n No source changes\n")
    } else {
        diff
    };
    state.ui.netlist.comparison_return_document = ActiveNetlistDocument::OwnedSource;
    state.ui.netlist.active_dependency_identity = None;
    state.ui.netlist.active_dependency_root = None;
    state.ui.netlist.active_document = ActiveNetlistDocument::GeneratedDiff;
    state.ui.netlist.active_document_initialized = true;
    state.simulation.netlist_content = state.ui.netlist.generated_diff_source.clone();
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    state.ui.netlist.completion_open = false;
    invalidate_source_evidence(&mut state.ui.netlist);
    Ok(())
}

pub fn restore_owned_revision(state: &mut AppState, index: usize) -> Result<(), String> {
    let snapshot = state
        .workspace
        .netlist_descriptor
        .as_ref()
        .and_then(|descriptor| descriptor.revision_history.get(index))
        .cloned()
        .ok_or_else(|| "The selected owned-source revision is no longer retained.".to_owned())?;
    let current = state
        .ui
        .netlist
        .owned_document
        .as_ref()
        .cloned()
        .ok_or_else(|| "No current owned source document is available.".to_owned())?;
    if current.content_digest() == snapshot.content_digest
        && current.dependencies() == snapshot.dependencies
    {
        return Err("The selected revision is already the current owned source.".to_owned());
    }

    let mut next_document = current.clone();
    next_document
        .replace_editable_source(
            next_document.content_digest(),
            snapshot.source.as_bytes().to_vec(),
        )
        .map_err(|error| error.to_string())?;
    next_document
        .acknowledge_dependencies(
            next_document.content_digest(),
            snapshot.dependencies.clone(),
        )
        .map_err(|error| error.to_string())?;

    let mut descriptor = state
        .workspace
        .netlist_descriptor
        .as_ref()
        .cloned()
        .ok_or_else(|| "Owned source metadata is unavailable.".to_owned())?;
    descriptor.retain_revision(&current, "Working state before revision restore")?;
    descriptor.source_encoding = snapshot.source_encoding;
    descriptor.source_line_ending = snapshot.source_line_ending;
    descriptor.owned_includes = snapshot.owned_includes.clone();
    descriptor.retain_revision(
        &next_document,
        format!("Restored revision {}", snapshot.document_revision),
    )?;

    let mut candidate = state.clone();
    candidate.workspace.netlist_source = Some(snapshot.source.clone());
    candidate.workspace.netlist_source_dirty = true;
    candidate.workspace.netlist_document = Some(next_document.clone());
    candidate.workspace.netlist_descriptor = Some(descriptor);
    candidate.ui.netlist.owned_document = Some(next_document);
    candidate.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    candidate.ui.netlist.active_dependency_identity = None;
    candidate.ui.netlist.active_dependency_root = None;
    candidate.ui.netlist.active_document_initialized = true;
    candidate.simulation.netlist_content = snapshot.source;
    candidate.ui.netlist.generated_diff_source.clear();
    candidate.ui.netlist.revision = candidate.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut candidate.ui.netlist);
    candidate
        .workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    *state = candidate;
    Ok(())
}

/// Put the working deck back in the editor after a read-only document.
///
/// The working deck is the retained project source, not the document that was
/// on screen: leaving a viewer must never overwrite an edit the engineer has
/// not saved yet.
fn return_to_working_deck(state: &mut AppState, root: ActiveNetlistDocument) {
    state.ui.netlist.active_dependency_identity = None;
    state.ui.netlist.active_dependency_root = None;
    match (root, state.workspace.netlist_source.clone()) {
        (ActiveNetlistDocument::OwnedSource, Some(source)) => {
            state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
            state.simulation.netlist_content = source;
        }
        _ => {
            state.ui.netlist.active_document = ActiveNetlistDocument::Generated;
            state.simulation.netlist_content = state.ui.netlist.generated_source.clone();
        }
    }
    state.ui.netlist.active_document_initialized = true;
    state.ui.netlist.completion_open = false;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut state.ui.netlist);
}

pub fn close_revision_comparison(state: &mut AppState) -> bool {
    return_to_working_deck(state, state.ui.netlist.comparison_return_document);
    state.ui.netlist.generated_diff_source.clear();
    true
}

fn short_digest(digest: crate::product::ContentDigest) -> String {
    digest.to_string().chars().take(12).collect()
}

/// The run a reopenable deck snapshot belongs to.
///
/// Both halves are required: a run binding without its sealed text is a
/// snapshot that cannot show a deck, and text without a run is a deck that
/// cannot name what executed it.
pub(crate) fn run_deck_snapshot_run_id(state: &AppState) -> Option<u64> {
    let run_id = state.ui.netlist.last_run_id?;
    state
        .ui
        .netlist
        .last_run_buffer
        .as_ref()
        .is_some_and(|buffer| !buffer.is_empty())
        .then_some(run_id)
}

/// The synthetic filename the run snapshot is filed under. It is an
/// identifier, not translated copy, and it names the run so two snapshots from
/// different runs can never be mistaken for one document.
pub(crate) fn run_deck_snapshot_artifact_name(state: &AppState) -> String {
    format!(
        "run-{}.sp",
        state.ui.netlist.last_run_id.unwrap_or_default()
    )
}

/// Open the deck a completed manual run consumed, pinned to that run.
///
/// The text is the sealed source the dispatch captured, not a re-derivation
/// from the live editor, so what this shows is what the engine read.
pub fn open_run_deck_snapshot(state: &mut AppState) -> bool {
    if run_deck_snapshot_run_id(state).is_none() {
        return false;
    }
    state.ui.netlist.executed_deck_view = None;
    project_run_snapshot(state);
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
/// Read-only is a property of the document, not of this route: the run
/// snapshot is the one [`ActiveNetlistDocument`] the editor never makes
/// editable and no lifecycle, save, or ownership action accepts.
///
/// The deck is checked against its run's receipt here, once, and the verdict
/// travels with the selection. Opening is the only moment the answer can
/// change, because both of its inputs are sealed.
pub fn open_executed_deck(state: &mut AppState, run_id: u64, point: usize) -> bool {
    let Some(deck) = state.simulation.executed_decks.get(run_id) else {
        return false;
    };
    let point = point.min(deck.points.len().saturating_sub(1));
    let Some(verification) =
        crate::workbench::documents::netlist_document::ExecutedDeckVerification::of(
            state, run_id, point,
        )
    else {
        return false;
    };
    state.ui.netlist.executed_deck_view = Some(
        crate::workbench::documents::netlist_document::ExecutedDeckSelection {
            run_id,
            point,
            verification,
        },
    );
    project_run_snapshot(state);
    true
}

/// Open one point's executed deck and put the reader in front of it.
///
/// Two callers ask this — the Verify workspace's corners row and a run's
/// receipt — and both are somewhere else when they ask, so the workspace
/// change belongs here rather than at each call site where one of them would
/// eventually forget it.
pub fn reveal_executed_deck(state: &mut AppState, run_id: u64, point: usize) -> bool {
    if !open_executed_deck(state, run_id, point) {
        return false;
    }
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    true
}

/// Projects whatever the run-snapshot document is currently bound to.
fn project_run_snapshot(state: &mut AppState) {
    if state.ui.netlist.active_document != ActiveNetlistDocument::RunSnapshot {
        state.ui.netlist.run_snapshot_return_document = state.ui.netlist.active_document;
    }
    let source = super::run_snapshot_source(state);
    state.ui.netlist.active_document = ActiveNetlistDocument::RunSnapshot;
    state.ui.netlist.active_dependency_identity = None;
    state.ui.netlist.active_dependency_root = None;
    state.ui.netlist.active_document_initialized = true;
    state.simulation.netlist_content = source;
    state.ui.netlist.requested_line = None;
    state.ui.netlist.cursor_line = 0;
    state.ui.netlist.completion_open = false;
    state.ui.netlist.completion_dismissed_at = None;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut state.ui.netlist);
}

/// Leave the run snapshot for the working deck it was opened from.
pub(crate) fn close_run_deck_snapshot(state: &mut AppState) -> bool {
    if state.ui.netlist.active_document != ActiveNetlistDocument::RunSnapshot {
        return false;
    }
    state.ui.netlist.executed_deck_view = None;
    return_to_working_deck(state, state.ui.netlist.run_snapshot_return_document);
    true
}

/// Diff the run's sealed deck against the working revision it was opened from.
pub fn compare_run_deck_snapshot(state: &mut AppState) -> Result<(), String> {
    if state.ui.netlist.executed_deck_view.is_some() {
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
    let run_id = state
        .ui
        .netlist
        .last_run_id
        .ok_or_else(|| "No completed manual run owns a deck snapshot.".to_owned())?;
    let snapshot = state
        .ui
        .netlist
        .last_run_buffer
        .clone()
        .ok_or_else(|| "The deck this run used is no longer retained.".to_owned())?;
    let root = state.ui.netlist.run_snapshot_return_document;
    let (working, current_label) =
        match root {
            ActiveNetlistDocument::OwnedSource => {
                let document =
                    state.ui.netlist.owned_document.as_ref().ok_or_else(|| {
                        "No current owned source document is available.".to_owned()
                    })?;
                (
                    state
                        .workspace
                        .netlist_source
                        .clone()
                        .unwrap_or_else(|| document.source().to_owned()),
                    format!(
                        "owned-r{}-{}",
                        document.revision().get(),
                        short_digest(document.content_digest())
                    ),
                )
            }
            _ => {
                let document = state
                    .ui
                    .netlist
                    .generated_document
                    .as_ref()
                    .ok_or_else(|| "No current generated artifact is available.".to_owned())?;
                (
                    state.ui.netlist.generated_source.clone(),
                    format!("generated-{}", short_digest(document.content_digest())),
                )
            }
        };
    let previous_label = format!("run-{run_id}");
    let diff = similar::TextDiff::from_lines(snapshot.as_str(), working.as_str())
        .unified_diff()
        .context_radius(3)
        .header(&previous_label, &current_label)
        .to_string();
    state.ui.netlist.generated_diff_source = if diff.is_empty() {
        format!("--- {previous_label}\n+++ {current_label}\n No source changes\n")
    } else {
        diff
    };
    state.ui.netlist.comparison_return_document = root;
    state.ui.netlist.active_dependency_identity = None;
    state.ui.netlist.active_dependency_root = None;
    state.ui.netlist.active_document = ActiveNetlistDocument::GeneratedDiff;
    state.ui.netlist.active_document_initialized = true;
    state.simulation.netlist_content = state.ui.netlist.generated_diff_source.clone();
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    state.ui.netlist.completion_open = false;
    invalidate_source_evidence(&mut state.ui.netlist);
    Ok(())
}
