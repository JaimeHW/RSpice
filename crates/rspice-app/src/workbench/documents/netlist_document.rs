//! Netlist document engine.
//!
//! This module owns editor diagnostics, syntax highlighting, completion,
//! parameter tuning, and run-to-run diff state. It has no application-chrome
//! chrome; the canonical Netlist workspace composes it as a document surface.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use std::collections::{HashMap, HashSet};

use super::{CodeSourceFileAction, CodeWorkspacePage};
use crate::state::netlist_diagnostics::NetlistDiagnosticCollection;
use crate::state::same_include_graph;
use rspice_app_runs::workbench::slots::runs::RunsState;
use rspice_app_state::workbench::app_state::model::{
    Model, prune_owned_include_authority, replace_root_document_preserving_dependencies,
};

pub(crate) use deck_transaction::DeckTransaction;

const MAX_OPEN_NETLIST_SECONDARY_DOCUMENTS: usize = 128;

mod baseline;
mod comparison;
mod completion;
mod deck_transaction;
pub(crate) mod diagnostics;
mod editor;
mod executed_deck;
mod highlight;
mod import_review;
pub(crate) mod language;
mod navigation;
mod owned_source;
mod param_scan;
mod relink;
mod source_index;
pub use import_review::{
    NetlistImportIssue, NetlistImportIssueSeverity, NetlistImportOperation,
    NetlistImportReviewState,
};
pub use navigation::{open_diagnostic_location, open_source_location};
pub(crate) use owned_source::open_owned_source_in;
pub(crate) use relink::{
    NetlistDependencyRelinkState, begin_dependency_relink, cancel_dependency_relink,
    commit_dependency_relink,
};

pub use comparison::{close_revision_comparison, restore_owned_revision, reveal_executed_deck};
pub use comparison::{
    close_run_deck_snapshot, compare_run_deck_snapshot_in, open_executed_deck_in,
    open_netlist_comparison, open_run_deck_snapshot_in, run_deck_snapshot_artifact_name_in,
    run_deck_snapshot_run_id_in,
};
pub use comparison::{compare_generated_revision, compare_owned_revision};
pub use diagnostics::{DiagnosticSeverity, control_disposition_summary};
pub use editor::show as show_editor;
pub use editor::{commit_edit as commit_editor_edit, editor_id_in};
pub(crate) use executed_deck::run_snapshot_source_in;
pub use executed_deck::{ExecutedDeckSelection, ExecutedDeckVerification};
pub use source_index::{root_deck_source_index, visible_source_index_in};

/// Exact, derived state of one project-owned netlist publication.
///
/// Project dirtiness, external publication, validation, and run eligibility
/// are independent facts. Keeping the projection here prevents the inspector,
/// toolbar, and execution gate from inventing contradictory interpretations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnedNetlistPublicationState {
    pub project_modified: bool,
    pub source_modified: bool,
    pub published_before: bool,
    pub saved: bool,
    pub validated: bool,
    pub external_change_pending: bool,
    pub run_eligible: bool,
}

impl OwnedNetlistPublicationState {
    /// What to say about the deck, or nothing when there is nothing to say.
    ///
    /// The seven fields above keep every distinction the toolbar, the
    /// inspector and the execution gate need. This is only the one line a
    /// reader sees, so it follows the panel rule: a state renders when it is
    /// not the normal one. A validated deck, a deck saved and validated, and a
    /// deck edited since its last save are all ordinary — the last is already
    /// marked by the document tab — so they say nothing at all.
    #[must_use]
    pub const fn label(self) -> Option<&'static str> {
        if self.external_change_pending {
            Some("Changed on disk since you opened it")
        } else if self.validated {
            None
        } else {
            Some("Not checked yet")
        }
    }
}

/// The publication state of the owned deck whose visible content has this
/// digest: whether it is saved, validated against the project's revision and
/// eligible to run. It reads the model and the netlist editor's state.
#[must_use]
pub fn owned_netlist_publication_state_in(
    model: &Model,
    netlist: &NetlistDocumentState,
    digest: crate::product::ContentDigest,
) -> OwnedNetlistPublicationState {
    let saved_digest = netlist.externally_saved_content_digest;
    let saved = saved_digest == Some(digest);
    let validated = model
        .netlist_artifacts
        .validation
        .as_ref()
        .is_some_and(|receipt| {
            receipt.visible_content_digest == digest
                && receipt.project_revision == model.workspace.content.project.revision().get()
        });
    OwnedNetlistPublicationState {
        project_modified: model.workspace.content.netlist_source_dirty,
        source_modified: saved_digest.is_some() && !saved,
        published_before: saved_digest.is_some(),
        saved,
        validated,
        external_change_pending: netlist.external_change.is_some(),
        // Execution consumes the exact validated in-memory snapshot. External
        // synchronization is deliberately independent from run readiness.
        run_eligible: validated,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum OwnedNetlistReplacementTarget {
    Root,
    Dependency(String),
    RetainedRoot(uuid::Uuid),
    RetainedDependency {
        deck_id: uuid::Uuid,
        logical_identity: String,
    },
}

/// One revision-bound source replacement prepared before a workspace-wide
/// mutation begins. The original digest prevents a stale search result from
/// changing newer source bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedNetlistReplacement {
    target: OwnedNetlistReplacementTarget,
    original: String,
    expected_digest: crate::product::ContentDigest,
    replacement: String,
    replacement_count: usize,
}

impl OwnedNetlistReplacement {
    #[must_use]
    pub fn root(original: &str, replacement: String, replacement_count: usize) -> Self {
        Self {
            target: OwnedNetlistReplacementTarget::Root,
            original: original.to_owned(),
            expected_digest: crate::state::content_digest(original),
            replacement,
            replacement_count,
        }
    }

    #[must_use]
    pub fn dependency(
        logical_identity: impl Into<String>,
        original: &str,
        replacement: String,
        replacement_count: usize,
    ) -> Self {
        Self {
            target: OwnedNetlistReplacementTarget::Dependency(logical_identity.into()),
            original: original.to_owned(),
            expected_digest: crate::state::content_digest(original),
            replacement,
            replacement_count,
        }
    }

    #[must_use]
    pub(crate) fn retained_root(
        deck_id: uuid::Uuid,
        original: &str,
        replacement: String,
        replacement_count: usize,
    ) -> Self {
        Self {
            target: OwnedNetlistReplacementTarget::RetainedRoot(deck_id),
            original: original.to_owned(),
            expected_digest: crate::state::content_digest(original),
            replacement,
            replacement_count,
        }
    }

    #[must_use]
    pub(crate) fn retained_dependency(
        deck_id: uuid::Uuid,
        logical_identity: impl Into<String>,
        original: &str,
        replacement: String,
        replacement_count: usize,
    ) -> Self {
        Self {
            target: OwnedNetlistReplacementTarget::RetainedDependency {
                deck_id,
                logical_identity: logical_identity.into(),
            },
            original: original.to_owned(),
            expected_digest: crate::state::content_digest(original),
            replacement,
            replacement_count,
        }
    }
}

impl OwnedNetlistReplacement {
    fn reversed(&self) -> Self {
        Self {
            target: self.target.clone(),
            original: self.replacement.clone(),
            expected_digest: crate::state::content_digest(&self.replacement),
            replacement: self.original.clone(),
            replacement_count: self.replacement_count,
        }
    }
}

#[derive(Debug, Clone)]
struct NetlistEditJournalEntry {
    description: String,
    forward: Vec<OwnedNetlistReplacement>,
    reverse: Vec<OwnedNetlistReplacement>,
}

const MAX_NETLIST_EDIT_JOURNAL_ENTRIES: usize = 32;

fn record_netlist_edit(state: &mut NetlistDocumentState, entry: NetlistEditJournalEntry) {
    state.edit_redo.clear();
    state.edit_undo.push(entry);
    if state.edit_undo.len() > MAX_NETLIST_EDIT_JOURNAL_ENTRIES {
        state.edit_undo.remove(0);
    }
}

fn clear_netlist_edit_journal(state: &mut NetlistDocumentState) {
    state.edit_undo.clear();
    state.edit_redo.clear();
}

/// Publish a set of root/include replacements as one all-or-nothing workspace
/// transition. No active-document switching occurs while the transaction is
/// prepared, and every original digest is checked before the cloned candidate
/// replaces live state.
pub fn replace_owned_sources_atomically(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
    replacements: Vec<OwnedNetlistReplacement>,
) -> Result<usize, String> {
    replace_owned_sources_atomically_impl(model, netlist, open_documents, replacements, true)
}

fn replace_owned_sources_atomically_impl(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
    replacements: Vec<OwnedNetlistReplacement>,
    record_edit: bool,
) -> Result<usize, String> {
    if let Some(holder) = model.live_write_locks.netlist.as_deref() {
        return Err(format!(
            "{holder} holds the netlist write lease; no source was changed."
        ));
    }
    if replacements.is_empty() {
        return Ok(0);
    }

    let applied_replacements = replacements
        .iter()
        .filter(|replacement| {
            replacement.replacement_count > 0 && replacement.original != replacement.replacement
        })
        .cloned()
        .collect::<Vec<_>>();
    let root_source = model
        .workspace
        .content
        .netlist_source
        .as_ref()
        .ok_or_else(|| "The project-owned root deck is no longer available.".to_owned())?;
    let current_document = netlist
        .owned_document
        .as_ref()
        .or(model.workspace.content.netlist_document.as_ref())
        .ok_or_else(|| "The project's netlist document is unavailable.".to_owned())?;
    if current_document.source() != root_source {
        return Err(
            "The project's root deck and its netlist document differ; no source was changed."
                .to_owned(),
        );
    }
    let mut next_document = current_document.clone();
    let original_dependencies = current_document.dependencies().to_vec();
    let original_includes = current_document.include_directives().to_vec();
    let mut next_dependencies = original_dependencies.clone();
    let mut next_descriptor = model
        .workspace
        .content
        .netlist_descriptor
        .clone()
        .ok_or_else(|| "Owned netlist metadata is unavailable.".to_owned())?;
    let mut next_retained_decks = model.workspace.content.retained_netlist_decks.clone();
    let mut next_root_source = root_source.clone();
    let mut root_seen = false;
    let mut dependency_seen = HashSet::new();
    let mut retained_roots_seen = HashSet::new();
    let mut retained_dependencies_seen = HashSet::new();
    let mut changed_count = 0usize;
    let mut root_changed = false;
    let mut dependencies_changed = false;

    for edit in replacements {
        if edit.replacement_count == 0 {
            continue;
        }
        match edit.target {
            OwnedNetlistReplacementTarget::Root => {
                if root_seen {
                    return Err("The root deck was included twice in one replacement.".to_owned());
                }
                root_seen = true;
                if crate::state::content_digest(root_source) != edit.expected_digest
                    || current_document.content_digest() != edit.expected_digest
                {
                    return Err(
                        "The root deck changed after search results were produced; no source was changed."
                            .to_owned(),
                    );
                }
                if edit.replacement == *root_source {
                    continue;
                }
                next_document
                    .replace_editable_source(
                        edit.expected_digest,
                        edit.replacement.as_bytes().to_vec(),
                    )
                    .map_err(|error| error.to_string())?;
                if !same_include_graph(next_document.include_directives(), &original_includes) {
                    return Err(
                        "Workspace-wide replacement would change the include graph. Edit the root deck directly, then resolve its dependencies before retrying."
                            .to_owned(),
                    );
                }
                next_root_source = edit.replacement;
                root_changed = true;
                changed_count = changed_count
                    .checked_add(edit.replacement_count)
                    .ok_or_else(|| "Replacement count overflowed.".to_owned())?;
            }
            OwnedNetlistReplacementTarget::Dependency(identity) => {
                if identity.trim().is_empty() || !dependency_seen.insert(identity.clone()) {
                    return Err(
                        "An owned include was missing or included twice in one replacement."
                            .to_owned(),
                    );
                }
                let include = next_descriptor
                    .owned_includes
                    .iter_mut()
                    .find(|include| include.logical_identity == identity)
                    .ok_or_else(|| {
                        format!("Owned include '{identity}' is no longer eligible for replacement.")
                    })?;
                let index = original_dependencies
                    .iter()
                    .position(|dependency| dependency.locator().logical_identity() == identity)
                    .ok_or_else(|| {
                        format!(
                            "Owned include '{identity}' is no longer among the deck's dependencies."
                        )
                    })?;
                let original = original_dependencies[index]
                    .source()
                    .ok_or_else(|| format!("Owned include '{identity}' is no longer resolved."))?;
                if crate::state::content_digest(original) != edit.expected_digest
                    || include.content_digest != edit.expected_digest
                {
                    return Err(format!(
                        "Owned include '{identity}' changed after search results were produced; no source was changed."
                    ));
                }
                if edit.replacement == original {
                    continue;
                }
                next_dependencies[index] = original_dependencies[index]
                    .clone()
                    .resolve_utf8(edit.replacement.as_bytes().to_vec())
                    .map_err(|error| error.to_string())?;
                include.revision = include
                    .revision
                    .checked_add(1)
                    .ok_or_else(|| format!("Owned include '{identity}' revision overflowed."))?;
                include.content_digest = crate::state::content_digest(&edit.replacement);
                dependencies_changed = true;
                changed_count = changed_count
                    .checked_add(edit.replacement_count)
                    .ok_or_else(|| "Replacement count overflowed.".to_owned())?;
            }
            OwnedNetlistReplacementTarget::RetainedRoot(deck_id) => {
                if !retained_roots_seen.insert(deck_id) {
                    return Err(format!(
                        "Retained top deck {deck_id} was included twice in one replacement."
                    ));
                }
                let deck = next_retained_decks
                    .iter_mut()
                    .find(|deck| deck.descriptor.deck_id == deck_id)
                    .ok_or_else(|| {
                        format!("Retained top deck {deck_id} is no longer available.")
                    })?;
                if deck.document.content_digest() != edit.expected_digest
                    || crate::state::content_digest(deck.document.source()) != edit.expected_digest
                {
                    return Err(format!(
                        "Retained top deck '{}' changed after search results were produced; no source was changed.",
                        deck.descriptor.artifact_name
                    ));
                }
                if edit.replacement == deck.document.source() {
                    continue;
                }
                let original_includes = deck.document.include_directives().to_vec();
                let original_dependencies = deck.document.dependencies().to_vec();
                deck.document
                    .replace_editable_source(
                        edit.expected_digest,
                        edit.replacement.as_bytes().to_vec(),
                    )
                    .map_err(|error| error.to_string())?;
                if !same_include_graph(deck.document.include_directives(), &original_includes) {
                    return Err(format!(
                        "Workspace-wide replacement would change the include graph of '{}'. Edit that top deck directly, then resolve its dependencies before retrying.",
                        deck.descriptor.artifact_name
                    ));
                }
                deck.document
                    .acknowledge_dependencies(deck.document.content_digest(), original_dependencies)
                    .map_err(|error| error.to_string())?;
                changed_count = changed_count
                    .checked_add(edit.replacement_count)
                    .ok_or_else(|| "Replacement count overflowed.".to_owned())?;
            }
            OwnedNetlistReplacementTarget::RetainedDependency {
                deck_id,
                logical_identity,
            } => {
                if logical_identity.trim().is_empty()
                    || !retained_dependencies_seen.insert((deck_id, logical_identity.clone()))
                {
                    return Err(format!(
                        "An owned include in retained top deck {deck_id} was missing or included twice in one replacement."
                    ));
                }
                let deck = next_retained_decks
                    .iter_mut()
                    .find(|deck| deck.descriptor.deck_id == deck_id)
                    .ok_or_else(|| {
                        format!("Retained top deck {deck_id} is no longer available.")
                    })?;
                let dependency_index = deck
                    .document
                    .dependencies()
                    .iter()
                    .position(|dependency| {
                        dependency.locator().logical_identity() == logical_identity
                    })
                    .ok_or_else(|| {
                        format!(
                            "Owned include '{logical_identity}' is no longer in retained top deck '{}'.",
                            deck.descriptor.artifact_name
                        )
                    })?;
                let original = deck.document.dependencies()[dependency_index]
                    .source()
                    .ok_or_else(|| {
                        format!(
                            "Owned include '{logical_identity}' in retained top deck '{}' is no longer resolved.",
                            deck.descriptor.artifact_name
                        )
                    })?
                    .to_owned();
                let include = deck
                    .descriptor
                    .owned_includes
                    .iter_mut()
                    .find(|include| include.logical_identity == logical_identity)
                    .ok_or_else(|| {
                        format!(
                            "Owned include '{logical_identity}' in retained top deck '{}' is no longer eligible for replacement.",
                            deck.descriptor.artifact_name
                        )
                    })?;
                if crate::state::content_digest(&original) != edit.expected_digest
                    || include.content_digest != edit.expected_digest
                {
                    return Err(format!(
                        "Owned include '{logical_identity}' in retained top deck '{}' changed after search results were produced; no source was changed.",
                        deck.descriptor.artifact_name
                    ));
                }
                if edit.replacement == original {
                    continue;
                }
                let mut dependencies = deck.document.dependencies().to_vec();
                dependencies[dependency_index] = dependencies[dependency_index]
                    .clone()
                    .resolve_utf8(edit.replacement.as_bytes().to_vec())
                    .map_err(|error| error.to_string())?;
                deck.document
                    .acknowledge_dependencies(deck.document.content_digest(), dependencies)
                    .map_err(|error| error.to_string())?;
                include.revision = include.revision.checked_add(1).ok_or_else(|| {
                    format!("Owned include '{logical_identity}' revision overflowed.")
                })?;
                include.content_digest = crate::state::content_digest(&edit.replacement);
                changed_count = changed_count
                    .checked_add(edit.replacement_count)
                    .ok_or_else(|| "Replacement count overflowed.".to_owned())?;
            }
        }
    }

    if changed_count == 0 {
        return Ok(0);
    }
    if root_changed || dependencies_changed {
        next_document
            .acknowledge_dependencies(next_document.content_digest(), next_dependencies)
            .map_err(|error| error.to_string())?;
    }

    let mut candidate = DeckTransaction::begin(model, netlist, open_documents);
    if root_changed
        && !candidate
            .model
            .workspace
            .content
            .replace_editable_netlist_source(next_root_source.clone())
    {
        return Err("The project-owned root deck could not be replaced.".to_owned());
    }
    candidate.model.workspace.content.netlist_descriptor = Some(next_descriptor);
    candidate.model.workspace.content.netlist_document = Some(next_document.clone());
    candidate.model.workspace.content.retained_netlist_decks = next_retained_decks;
    candidate.model.workspace.content.netlist_source_dirty = true;
    candidate.netlist.owned_document = Some(next_document.clone());
    if candidate.netlist.active_document == ActiveNetlistDocument::OwnedSource {
        candidate.model.simulation.source.netlist_content = candidate
            .netlist
            .active_dependency_identity
            .as_deref()
            .and_then(|identity| {
                next_document
                    .dependencies()
                    .iter()
                    .find(|dependency| dependency.locator().logical_identity() == identity)
                    .and_then(crate::state::DependencyMetadata::source)
                    .map(str::to_owned)
            })
            .unwrap_or(next_root_source);
    }
    candidate.netlist.revision = candidate.netlist.revision.wrapping_add(1);
    candidate.model.invalidate_source_evidence();
    candidate
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    if record_edit && !applied_replacements.is_empty() {
        let reverse = applied_replacements
            .iter()
            .rev()
            .map(OwnedNetlistReplacement::reversed)
            .collect();
        record_netlist_edit(
            &mut candidate.netlist,
            NetlistEditJournalEntry {
                description: format!("replace {changed_count} netlist match(es)"),
                forward: applied_replacements,
                reverse,
            },
        );
    }
    candidate.commit_deck(model, netlist, open_documents);
    Ok(changed_count)
}

#[must_use]
pub fn can_undo_netlist_edit(netlist: &NetlistDocumentState) -> bool {
    !netlist.edit_undo.is_empty()
}

#[must_use]
pub fn can_redo_netlist_edit(netlist: &NetlistDocumentState) -> bool {
    !netlist.edit_redo.is_empty()
}

pub fn undo_netlist_edit(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
) -> Result<Option<String>, String> {
    let Some(entry) = netlist.edit_undo.pop() else {
        return Ok(None);
    };
    if let Err(error) = replace_owned_sources_atomically_impl(
        model,
        netlist,
        open_documents,
        entry.reverse.clone(),
        false,
    ) {
        netlist.edit_undo.push(entry);
        return Err(error);
    }
    let description = entry.description.clone();
    netlist.edit_redo.push(entry);
    Ok(Some(description))
}

pub fn redo_netlist_edit(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
) -> Result<Option<String>, String> {
    let Some(entry) = netlist.edit_redo.pop() else {
        return Ok(None);
    };
    if let Err(error) = replace_owned_sources_atomically_impl(
        model,
        netlist,
        open_documents,
        entry.forward.clone(),
        false,
    ) {
        netlist.edit_redo.push(entry);
        return Err(error);
    }
    let description = entry.description.clone();
    netlist.edit_undo.push(entry);
    Ok(Some(description))
}

/// Atomically replace the exact project-owned source across the canonical
/// document, persisted project projection, and visible editor buffer.
pub fn replace_owned_source(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
    workbench: &mut crate::workbench::state::WorkbenchSession,
    source: String,
) -> bool {
    if let Some(holder) = model.live_write_locks.netlist.clone() {
        workbench.prompt(format!(
            "{holder} holds the netlist write lease; no source was changed."
        ));
        return false;
    }
    replace_owned_source_unlocked(model, netlist, open_documents, workbench, source)
}

/// Adopt the project's own netlist source a live session replaced: the root
/// document rewritten over it, when the project keeps one, becomes the
/// editor's, and the editor's revision and undo history move past the source
/// they were taken from.
pub fn adopt_live_owned_source(
    netlist: &mut NetlistDocumentState,
    document: Option<crate::state::NetlistDocument>,
) {
    if let Some(document) = document {
        netlist.owned_document = Some(document);
    }
    netlist.revision = netlist.revision.wrapping_add(1);
    clear_netlist_edit_journal(netlist);
}

fn replace_owned_source_unlocked(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
    workbench: &mut crate::workbench::state::WorkbenchSession,
    source: String,
) -> bool {
    if netlist.active_document != ActiveNetlistDocument::OwnedSource
        || netlist.active_dependency_identity.is_some()
    {
        return false;
    }
    let next_document = if let Some(document) = &netlist.owned_document {
        let Ok(next) = replace_root_document_preserving_dependencies(document, &source) else {
            return false;
        };
        Some(next)
    } else {
        None
    };
    let mut candidate = DeckTransaction::begin(model, netlist, open_documents);
    if !candidate
        .model
        .workspace
        .content
        .replace_editable_netlist_source(source.clone())
    {
        return false;
    }
    candidate.model.simulation.source.netlist_content = source;
    if let Some(document) = next_document {
        if let Some(descriptor) = candidate
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_mut()
        {
            prune_owned_include_authority(descriptor, &document);
        }
        candidate.model.workspace.content.netlist_document = Some(document.clone());
        candidate.netlist.owned_document = Some(document);
    }
    candidate.netlist.revision = candidate.netlist.revision.wrapping_add(1);
    candidate.model.invalidate_source_evidence();
    clear_netlist_edit_journal(&mut candidate.netlist);
    let Err(error) = candidate
        .model
        .workspace
        .content
        .validate_simulation_configuration()
    else {
        candidate.commit_deck(model, netlist, open_documents);
        return true;
    };
    workbench.prompt(format!(
        "The netlist edit was not applied because it would break the project's sources: {error}"
    ));
    false
}

/// The canonical document a root names: the generated primary, or the
/// netlist editor's owned document; revision comparisons and run snapshots
/// have none. It reads the model and the netlist editor's state.
fn canonical_root_document_in<'a>(
    model: &'a Model,
    netlist: &'a NetlistDocumentState,
    root: ActiveNetlistDocument,
) -> Option<&'a crate::state::NetlistDocument> {
    match root {
        ActiveNetlistDocument::Generated => model.netlist_artifacts.generated_document.as_ref(),
        ActiveNetlistDocument::OwnedSource => netlist.owned_document.as_ref(),
        ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => None,
    }
}

/// Exact retained dependency currently projected as a first-class source
/// document. The dependency source remains canonical inside its root
/// `NetlistDocument`; this function never synthesizes a second source store.
/// It reads the model and the netlist editor's state.
pub(crate) fn active_dependency_in<'a>(
    model: &'a Model,
    netlist: &'a NetlistDocumentState,
) -> Option<&'a crate::state::DependencyMetadata> {
    let identity = netlist.active_dependency_identity.as_deref()?;
    let root = netlist.active_dependency_root?;
    canonical_root_document_in(model, netlist, root)?
        .dependencies()
        .iter()
        .find(|dependency| dependency.locator().logical_identity() == identity)
}

/// Whether the visible retained include has been explicitly copied into the
/// project and is therefore an editable project document. It reads the model
/// and the netlist editor's state.
#[must_use]
pub(crate) fn active_dependency_is_owned_in(model: &Model, netlist: &NetlistDocumentState) -> bool {
    let Some(identity) = netlist.active_dependency_identity.as_deref() else {
        return false;
    };
    netlist.active_dependency_root == Some(ActiveNetlistDocument::OwnedSource)
        && model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .and_then(|descriptor| descriptor.owned_include(identity))
            .is_some()
}

/// Whether editor mutations are authorized for the exact visible source. It
/// reads the model and the netlist editor's state.
#[must_use]
pub fn active_netlist_source_is_editable_in(model: &Model, netlist: &NetlistDocumentState) -> bool {
    if netlist.active_dependency_identity.is_some() {
        active_dependency_is_owned_in(model, netlist)
    } else {
        netlist.active_document == ActiveNetlistDocument::OwnedSource
            && model.workspace.content.has_editable_netlist_source()
    }
}

/// Which netlist document a manual deck run would take its source from.
///
/// Before the session has activated a document, a retained project-owned
/// source outranks the generated primary; afterwards the active document is
/// the answer and nothing infers one from the visible buffer.
///
/// It reads the model and the netlist editor's state.
pub(crate) fn effective_active_document_in(
    model: &Model,
    netlist: &NetlistDocumentState,
) -> ActiveNetlistDocument {
    if netlist.active_document_initialized {
        netlist.active_document
    } else if model.workspace.content.netlist_source.is_some() {
        ActiveNetlistDocument::OwnedSource
    } else {
        ActiveNetlistDocument::Generated
    }
}

/// The exact deck text a manual run would consume right now.
///
/// The run gate and the post-run strip both have to name the same bytes: one
/// decides whether the deck may run, the other whether the deck still matches
/// the run that already happened.
///
/// It reads the model and the netlist editor's state.
pub(crate) fn working_deck_source_in<'a>(
    model: &'a Model,
    netlist: &NetlistDocumentState,
) -> &'a str {
    if effective_active_document_in(model, netlist) == ActiveNetlistDocument::OwnedSource {
        model
            .workspace
            .content
            .netlist_source
            .as_deref()
            .unwrap_or(model.simulation.source.netlist_content.as_str())
    } else {
        model.simulation.source.netlist_content.as_str()
    }
}

/// Why Run on the Netlist workspace will not run the deck: what
/// [`manual_deck_run_block_reason_in`] finds, then a validation whose
/// authorization the run controller no longer holds, which is the run
/// tier's half of the answer.
pub fn manual_deck_run_block_reason_for_run(
    model: &Model,
    netlist: &NetlistDocumentState,
    messages: crate::workbench::MessageCatalog,
    controller: &rspice_app_runs::simulation::SimulationController,
) -> Option<String> {
    manual_deck_run_block_reason_in(model, netlist, messages).or_else(|| {
        rspice_app_runs::workbench::app_state::run_preflight::manual_deck_authorization_block_reason(
            model, controller,
        )
    })
}

/// User-facing reason the Netlist workspace cannot run the current deck. It
/// reads the model, the netlist editor's state and the message catalogue.
pub fn manual_deck_run_block_reason_in(
    model: &Model,
    netlist: &NetlistDocumentState,
    messages: crate::workbench::MessageCatalog,
) -> Option<String> {
    if model.simulation.execution.has_active_execution() {
        return Some("A simulation execution is already active".to_string());
    }
    let active_document = effective_active_document_in(model, netlist);
    if netlist.active_dependency_identity.is_some() {
        return Some(messages.text(crate::workbench::MessageId::NetlistDependencyCannotRun));
    }
    let source = working_deck_source_in(model, netlist);
    if source.trim().is_empty() {
        return Some("Enter a netlist before running".to_string());
    }
    if active_document == ActiveNetlistDocument::GeneratedDiff {
        return Some("Generated comparison documents cannot be executed".to_owned());
    }
    if active_document == ActiveNetlistDocument::RunSnapshot {
        return Some(
            "The deck a completed run used is sealed with that run; return to the working deck to run it again."
                .to_owned(),
        );
    }
    let current_digest = crate::state::content_digest(source);
    if active_document == ActiveNetlistDocument::Generated
        && (model.netlist_artifacts.generation_error.is_some()
            || model.netlist_artifacts.generated_input_digest
                != model.netlist_artifacts.current_generation_input_digest)
    {
        return Some(
            model
                .netlist_artifacts
                .generation_error
                .clone()
                .unwrap_or_else(|| "Regenerate the stale netlist before running".to_owned()),
        );
    }
    let publication = owned_netlist_publication_state_in(model, netlist, current_digest);
    if !publication.validated {
        return Some("Validate the current source and project revision before running".to_owned());
    }
    debug_assert!(publication.run_eligible);
    None
}

/// Open a resolved direct or transitive dependency without changing its root
/// document selection or ownership. Unresolved and stale identities fail
/// closed and leave the current buffer unchanged. It works over the model, the
/// netlist editor's state, the include tabs the reader opened and the runs.
pub(crate) fn open_netlist_dependency_in(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
    runs: &mut RunsState,
    logical_identity: &str,
) -> Result<(), String> {
    let root = netlist
        .active_dependency_root
        .unwrap_or(netlist.active_document);
    open_netlist_dependency_from_root_in(
        model,
        netlist,
        open_documents,
        runs,
        root,
        logical_identity,
    )
}

/// Open a dependency of the given root document, over the model, the netlist
/// editor's state, the include tabs the reader opened and the runs, whose
/// manual run forgets the lines edited in the deck it leaves.
pub fn open_netlist_dependency_from_root_in(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
    runs: &mut RunsState,
    root: ActiveNetlistDocument,
    logical_identity: &str,
) -> Result<(), String> {
    if root == ActiveNetlistDocument::GeneratedDiff {
        return Err("Revision comparisons do not own dependency documents.".to_owned());
    }
    let document = canonical_root_document_in(model, netlist, root).ok_or_else(|| {
        "The selected dependency root is no longer available in this workspace.".to_owned()
    })?;
    let root_document_id = document.id();
    let source = document
        .dependencies()
        .iter()
        .find(|dependency| dependency.locator().logical_identity() == logical_identity)
        .and_then(crate::state::DependencyMetadata::source)
        .ok_or_else(|| {
            "The selected dependency is unresolved or no longer belongs to this source closure."
                .to_owned()
        })?
        .to_owned();

    let tab = crate::workbench::state::WorkspaceDocumentId::NetlistDependency {
        root: root_document_id,
        logical_identity: logical_identity.to_owned(),
    };
    if !open_documents.contains(&tab)
        && open_documents.len() >= MAX_OPEN_NETLIST_SECONDARY_DOCUMENTS
    {
        return Err(format!(
            "Close an include tab before opening another; the workspace retains at most {MAX_OPEN_NETLIST_SECONDARY_DOCUMENTS} secondary netlist documents."
        ));
    }
    netlist.active_document = root;
    netlist.active_dependency_root = Some(root);
    netlist.active_dependency_identity = Some(logical_identity.to_owned());
    open_documents.insert(tab);
    netlist.active_document_initialized = true;
    model.simulation.source.netlist_content = source;
    netlist.requested_line = None;
    netlist.cursor_line = 0;
    runs.manual_run.edited_lines.clear();
    netlist.completion_open = false;
    netlist.completion_dismissed_at = None;
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
    Ok(())
}

/// Return from an include document to its exact generated or owned root. It
/// writes the model, the netlist editor's state and the runs.
pub(crate) fn close_active_dependency_in(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    runs: &mut RunsState,
) -> bool {
    let Some(root) = netlist.active_dependency_root.take() else {
        return false;
    };
    netlist.active_dependency_identity = None;
    netlist.active_document = root;
    model.simulation.source.netlist_content = match root {
        ActiveNetlistDocument::Generated => model.netlist_artifacts.generated_source.clone(),
        ActiveNetlistDocument::OwnedSource => netlist
            .owned_document
            .as_ref()
            .map(|document| document.source().to_owned())
            .or_else(|| model.workspace.content.netlist_source.clone())
            .unwrap_or_default(),
        ActiveNetlistDocument::GeneratedDiff => netlist.generated_diff_source.clone(),
        ActiveNetlistDocument::RunSnapshot => run_snapshot_source_in(model, netlist, runs),
    };
    netlist.requested_line = None;
    netlist.cursor_line = 0;
    netlist.completion_open = false;
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
    refresh_diff_pips_from_baseline_in(model, runs);
    true
}

/// Promote the visible read-only dependency into explicit project ownership.
/// Its stable project identity is persisted, while its exact source bytes stay
/// in the canonical dependency closure used by execution and export.
pub fn copy_active_dependency_to_project(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
) -> Result<uuid::Uuid, String> {
    if netlist.active_dependency_root != Some(ActiveNetlistDocument::OwnedSource) {
        return Err(
            "Open the dependency from a project-owned root before copying it into the project."
                .to_owned(),
        );
    }
    let dependency = active_dependency_in(model, netlist)
        .cloned()
        .ok_or_else(|| "The selected dependency is no longer available.".to_owned())?;
    let identity = dependency.locator().logical_identity();
    if let Some(existing) = model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .and_then(|descriptor| descriptor.owned_include(identity))
    {
        return Ok(existing.document_id);
    }

    let include = crate::state::OwnedNetlistIncludeDescriptor::try_new(&dependency)?;
    let document_id = include.document_id;
    let mut candidate = DeckTransaction::begin(model, netlist, open_documents);
    candidate
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_mut()
        .ok_or_else(|| "Owned netlist metadata is unavailable.".to_owned())?
        .owned_includes
        .push(include);
    candidate.model.workspace.content.netlist_source_dirty = true;
    candidate
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    candidate.netlist.revision = candidate.netlist.revision.wrapping_add(1);
    candidate.netlist.completion_open = false;
    candidate.model.invalidate_source_evidence();
    candidate.commit_deck(model, netlist, open_documents);
    Ok(document_id)
}

/// Release editable project authority for the visible include while retaining
/// its exact bytes and include edge in the canonical closure. This is the
/// reversible counterpart to copy-to-project: the document becomes read-only
/// again, and no source text or external artifact is deleted.
pub fn release_active_dependency_from_project(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
) -> Result<(), String> {
    if netlist.active_dependency_root != Some(ActiveNetlistDocument::OwnedSource) {
        return Err(
            "Only an include opened from the owned top deck can release project ownership."
                .to_owned(),
        );
    }
    let identity = netlist
        .active_dependency_identity
        .clone()
        .ok_or_else(|| "No include document is active.".to_owned())?;
    let mut candidate = DeckTransaction::begin(model, netlist, open_documents);
    let descriptor = candidate
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_mut()
        .ok_or_else(|| "Owned netlist metadata is unavailable.".to_owned())?;
    let before = descriptor.owned_includes.len();
    descriptor
        .owned_includes
        .retain(|include| include.logical_identity != identity);
    if descriptor.owned_includes.len() == before {
        return Err("The active include is not project-owned.".to_owned());
    }
    candidate.model.workspace.content.netlist_source_dirty = true;
    candidate.netlist.revision = candidate.netlist.revision.wrapping_add(1);
    candidate.netlist.completion_open = false;
    candidate.model.invalidate_source_evidence();
    clear_netlist_edit_journal(&mut candidate.netlist);
    candidate
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    candidate.commit_deck(model, netlist, open_documents);
    Ok(())
}

/// Replace the exact bytes of an explicitly project-owned include. The root
/// deck text is unchanged; its canonical dependency closure, project
/// projection, ownership digest, validation evidence, and revision advance in
/// one validated transaction.
pub fn replace_owned_dependency_source(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
    source: String,
) -> bool {
    if !active_dependency_is_owned_in(model, netlist) {
        return false;
    }
    let Some(identity) = netlist.active_dependency_identity.clone() else {
        return false;
    };
    let Some(current_document) = netlist.owned_document.as_ref().cloned() else {
        return false;
    };
    let Some(index) = current_document
        .dependencies()
        .iter()
        .position(|dependency| dependency.locator().logical_identity() == identity)
    else {
        return false;
    };
    if current_document.dependencies()[index].source() == Some(source.as_str()) {
        return false;
    }

    let mut dependencies = current_document.dependencies().to_vec();
    let Ok(next_dependency) = dependencies[index]
        .clone()
        .resolve_utf8(source.as_bytes().to_vec())
    else {
        return false;
    };
    dependencies[index] = next_dependency;
    let mut next_document = current_document;
    if next_document
        .acknowledge_dependencies(next_document.content_digest(), dependencies)
        .is_err()
    {
        return false;
    }

    let mut candidate = DeckTransaction::begin(model, netlist, open_documents);
    let Some(descriptor) = candidate
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_mut()
    else {
        return false;
    };
    let Some(include) = descriptor
        .owned_includes
        .iter_mut()
        .find(|include| include.logical_identity == identity)
    else {
        return false;
    };
    let Some(next_revision) = include.revision.checked_add(1) else {
        return false;
    };
    include.revision = next_revision;
    include.content_digest = crate::state::content_digest(&source);
    candidate.model.workspace.content.netlist_document = Some(next_document.clone());
    candidate.model.workspace.content.netlist_source_dirty = true;
    candidate.netlist.owned_document = Some(next_document);
    candidate.model.simulation.source.netlist_content = source;
    candidate.netlist.revision = candidate.netlist.revision.wrapping_add(1);
    candidate.model.invalidate_source_evidence();
    clear_netlist_edit_journal(&mut candidate.netlist);
    if candidate
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .is_err()
    {
        return false;
    }
    candidate.commit_deck(model, netlist, open_documents);
    true
}

/// Take the reader to the generated deck from another workspace: its page in
/// the Netlist workspace, with the generated primary in front. It works over
/// the model, the netlist editor's state, the Code workspace's page and the
/// workbench.
pub fn show_generated_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    page: &mut CodeWorkspacePage,
    workbench: &mut crate::workbench::state::WorkbenchSession,
) {
    let _ = open_generated_primary_in(model, netlist);
    *page = CodeWorkspacePage::Netlist;
    if workbench.workspace != crate::workbench::state::Workspace::Netlist {
        workbench.activate(crate::workbench::state::Workspace::Netlist);
    }
}

/// Select the immutable generated primary without deleting or changing an
/// owned source or generated-diff document. This is an explicit document
/// transition; navigating among Code workspace pages must not call it. It
/// works over the model and the netlist editor's state.
pub fn open_generated_primary_in(model: &mut Model, netlist: &mut NetlistDocumentState) -> bool {
    if model.netlist_artifacts.generated_document.is_none()
        || model.netlist_artifacts.generated_source.trim().is_empty()
    {
        return false;
    }
    if netlist.active_document == ActiveNetlistDocument::Generated
        && netlist.active_dependency_identity.is_none()
    {
        return false;
    }
    netlist.active_dependency_identity = None;
    netlist.active_dependency_root = None;
    netlist.active_document = ActiveNetlistDocument::Generated;
    netlist.active_document_initialized = true;
    model.simulation.source.netlist_content = model.netlist_artifacts.generated_source.clone();
    netlist.completion_open = false;
    netlist.completion_dismissed_at = None;
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
    true
}

/// Activate the retained project-owned root without creating or promoting a
/// document, over the model and the netlist editor's state. Document tabs use
/// this exact transition so switching tabs never changes source ownership.
pub(crate) fn open_owned_primary_in(model: &mut Model, netlist: &mut NetlistDocumentState) -> bool {
    let Some(document) = netlist.owned_document.as_ref().or(model
        .workspace
        .content
        .netlist_document
        .as_ref())
    else {
        return false;
    };
    // Workspace/document-bar reconciliation may ask to activate the already
    // active tab again. That is presentation work, not a source transition:
    // advancing the editor revision and discarding the exact validation
    // receipt here also destroys the controller's one-shot authorization and
    // leaves a visibly validated manual deck unable to run.
    if netlist.active_document == ActiveNetlistDocument::OwnedSource
        && netlist.active_dependency_identity.is_none()
        && model.simulation.source.netlist_content == document.source()
    {
        return true;
    }
    let source = document.source().to_owned();
    netlist.active_document = ActiveNetlistDocument::OwnedSource;
    netlist.active_dependency_identity = None;
    netlist.active_dependency_root = None;
    netlist.active_document_initialized = true;
    model.simulation.source.netlist_content = source;
    netlist.completion_open = false;
    netlist.completion_dismissed_at = None;
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
    true
}

/// Code-workspace document currently projected into the central editor.
/// Generated and owned source are independent retained documents; switching
/// between them never deletes either artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActiveNetlistDocument {
    #[default]
    Generated,
    OwnedSource,
    GeneratedDiff,
    /// The exact deck a completed manual run consumed. Sealed with that run,
    /// so it is a viewer and never an editable deck.
    RunSnapshot,
}

/// User-selected reach of the Code workspace find surface. Replacement is
/// intentionally limited to project-owned source documents; project-reference
/// search is always find-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NetlistFindScope {
    #[default]
    CurrentDocument,
    Selection,
    OpenDocuments,
    ActiveLanguageProject,
    ProjectReferences,
}

/// Persistent, keyboard-reachable state for the mockup's Find and replace
/// surface. Match rows are derived from the current exact source every frame,
/// so they cannot become stale after an edit.
#[derive(Debug, Clone)]
pub struct NetlistFindState {
    pub open: bool,
    pub find: String,
    pub replacement: String,
    pub match_case: bool,
    pub whole_symbol: bool,
    pub regular_expression: bool,
    pub include_comments: bool,
    pub include_generated_references: bool,
    pub scope: NetlistFindScope,
    pub selected_match: usize,
    pub error: Option<String>,
    /// A press the window made this frame, which the shell carries out once
    /// the page has drawn. The session does not store it.
    pub pending: Option<NetlistFindStep>,
}

impl Default for NetlistFindState {
    fn default() -> Self {
        Self {
            open: false,
            find: String::new(),
            replacement: String::new(),
            match_case: false,
            whole_symbol: false,
            regular_expression: false,
            include_comments: true,
            include_generated_references: false,
            scope: NetlistFindScope::CurrentDocument,
            selected_match: 0,
            error: None,
            pending: None,
        }
    }
}

/// A press in the find window: going to a match, replacing it, or replacing
/// every match. Going to a match opens its document and a replacement
/// commits through a candidate copy of the whole state, so the shell carries
/// it out once the page has drawn.
#[derive(Debug, Clone)]
pub struct NetlistFindStep {
    pub(crate) action: Option<NetlistFindAction>,
    /// The options the matches were found with.
    pub(crate) options: crate::state::FindOptions,
    /// The match the press names.
    pub(crate) target: Option<NetlistFindTarget>,
    /// What Replace all planned, against the scan the window drew.
    pub(crate) replacements: Option<Vec<OwnedNetlistReplacement>>,
}

/// What a press in the find window asks for.
#[derive(Debug, Clone, Copy)]
pub(crate) enum NetlistFindAction {
    Select(usize),
    ReplaceNext,
    ReplaceAll,
}

/// The match a find step names, held independently of the scan it came from.
#[derive(Debug, Clone)]
pub(crate) struct NetlistFindTarget {
    pub(crate) index: usize,
    pub(crate) active_document: ActiveNetlistDocument,
    pub(crate) deck_id: Option<uuid::Uuid>,
    pub(crate) dependency_identity: Option<String>,
    pub(crate) editable: bool,
    /// The document's bytes, present only for a replacement.
    pub(crate) source: Option<String>,
    pub(crate) line: usize,
    pub(crate) byte_range: std::ops::Range<usize>,
}

#[derive(Debug, Clone)]
pub struct NetlistOwnershipDialogState {
    pub open: bool,
    pub artifact_name: String,
    pub strategy: crate::state::OwnedNetlistEditStrategy,
    pub error: Option<String>,
}

/// Revision-bound lifecycle transaction for the active project-owned top deck.
/// Native publication paths are deliberately not edited here: rename and move
/// operate on the portable project path, while Save As controls publication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistLifecycleTransaction {
    pub action: CodeSourceFileAction,
    pub project_id: crate::product::ProjectId,
    pub deck_id: Option<uuid::Uuid>,
    pub document_id: Option<crate::state::NetlistDocumentId>,
    pub document_revision: Option<crate::product::ObjectRevision>,
    pub content_digest: Option<crate::product::ContentDigest>,
    pub source_path: String,
    pub proposed_path: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct NetlistLifecycleDialogState {
    pub transaction: Option<NetlistLifecycleTransaction>,
    pub include_transaction: Option<NetlistIncludeLifecycleTransaction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistIncludeLifecycleTransaction {
    pub action: CodeSourceFileAction,
    pub project_id: crate::product::ProjectId,
    pub root_document_id: crate::state::NetlistDocumentId,
    pub root_revision: crate::product::ObjectRevision,
    pub logical_identity: String,
    pub proposed_identity: String,
    pub error: Option<String>,
}

/// Open the lifecycle transaction for a Rename or Move of the active owned
/// include, over the model and the netlist editor's state.
pub fn begin_owned_include_lifecycle_action(
    model: &Model,
    netlist: &mut NetlistDocumentState,
    action: CodeSourceFileAction,
) -> Result<(), String> {
    if !matches!(
        action,
        CodeSourceFileAction::Rename | CodeSourceFileAction::Move
    ) {
        return Err("Owned include lifecycle supports Rename and Move here.".to_owned());
    }
    if !active_dependency_is_owned_in(model, netlist) {
        return Err(
            "Copy the active include into the project before renaming or moving it.".to_owned(),
        );
    }
    let logical_identity = netlist
        .active_dependency_identity
        .clone()
        .ok_or_else(|| "No include document is active.".to_owned())?;
    let document = model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .ok_or_else(|| "The owned root document is unavailable.".to_owned())?;
    netlist.lifecycle_dialog.include_transaction = Some(NetlistIncludeLifecycleTransaction {
        action,
        project_id: model.workspace.content.project.id(),
        root_document_id: document.id(),
        root_revision: document.revision(),
        proposed_identity: logical_identity.clone(),
        logical_identity,
        error: None,
    });
    Ok(())
}

pub fn commit_owned_include_lifecycle_action(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
) -> Result<String, String> {
    let transaction = netlist
        .lifecycle_dialog
        .include_transaction
        .clone()
        .ok_or_else(|| "No owned-include lifecycle transaction is open.".to_owned())?;
    let document = model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .ok_or_else(|| "The owned root document is unavailable.".to_owned())?;
    if model.workspace.content.project.id() != transaction.project_id
        || document.id() != transaction.root_document_id
        || document.revision() != transaction.root_revision
        || netlist.active_dependency_identity.as_deref()
            != Some(transaction.logical_identity.as_str())
    {
        return Err(
            "The include closure changed while the lifecycle transaction was open; review it and retry."
                .to_owned(),
        );
    }

    let proposed = normalized_top_deck_path(&transaction.proposed_identity)?;
    if transaction.action == CodeSourceFileAction::Rename
        && top_deck_parent(&proposed) != top_deck_parent(&transaction.logical_identity)
    {
        return Err(
            "Rename changes only the include file name. Use Move to change its project folder."
                .to_owned(),
        );
    }
    let display_name = std::path::Path::new(&proposed).file_name().map_or_else(
        || proposed.clone(),
        |name| name.to_string_lossy().into_owned(),
    );
    let locator = crate::state::SourceLocator::try_new(proposed.clone(), display_name.clone())
        .map_err(|error| error.to_string())?;

    let mut candidate = DeckTransaction::begin(model, netlist, open_documents);
    let mut next_document = document.clone();
    next_document
        .rename_dependency_identity(
            transaction.root_revision,
            &transaction.logical_identity,
            locator,
        )
        .map_err(|error| error.to_string())?;
    let descriptor = candidate
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_mut()
        .ok_or_else(|| "Owned netlist metadata is unavailable.".to_owned())?;
    let include = descriptor
        .owned_includes
        .iter_mut()
        .find(|include| include.logical_identity == transaction.logical_identity)
        .ok_or_else(|| "The include is no longer project-owned.".to_owned())?;
    include.logical_identity.clone_from(&proposed);
    include.display_name = display_name;
    include.revision = include
        .revision
        .checked_add(1)
        .ok_or_else(|| "Owned include revision overflowed.".to_owned())?;

    let old_tab = crate::workbench::state::WorkspaceDocumentId::NetlistDependency {
        root: transaction.root_document_id,
        logical_identity: transaction.logical_identity.clone(),
    };
    let new_tab = crate::workbench::state::WorkspaceDocumentId::NetlistDependency {
        root: transaction.root_document_id,
        logical_identity: proposed.clone(),
    };
    if candidate.open_documents.remove(&old_tab) {
        candidate.open_documents.insert(new_tab);
    }
    candidate.model.workspace.content.netlist_document = Some(next_document.clone());
    candidate.model.workspace.content.netlist_source_dirty = true;
    candidate.netlist.owned_document = Some(next_document);
    candidate.netlist.active_dependency_identity = Some(proposed.clone());
    candidate.netlist.lifecycle_dialog.include_transaction = None;
    candidate.netlist.revision = candidate.netlist.revision.wrapping_add(1);
    candidate.model.invalidate_source_evidence();
    clear_netlist_edit_journal(&mut candidate.netlist);
    candidate
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    candidate.commit_deck(model, netlist, open_documents);
    Ok(format!(
        "{} project-owned include as {proposed} without changing its retained source bytes.",
        if transaction.action == CodeSourceFileAction::Rename {
            "Renamed"
        } else {
            "Moved"
        }
    ))
}

/// Open the lifecycle transaction for a New, Rename, Move, Duplicate or
/// Delete of the active top deck, with the path it proposes, over the model
/// and the netlist editor's state.
pub fn begin_netlist_lifecycle_action_in(
    model: &Model,
    netlist: &mut NetlistDocumentState,
    action: CodeSourceFileAction,
) -> Result<(), String> {
    if netlist.active_dependency_identity.is_some() {
        return Err(
            "Return to the owned root deck before changing a top-level deck lifecycle.".to_owned(),
        );
    }
    let descriptor = model.workspace.content.netlist_descriptor.as_ref();
    let document = model.workspace.content.netlist_document.as_ref();
    if descriptor.is_some() != document.is_some() {
        return Err("The active top-deck descriptor and document are inconsistent.".to_owned());
    }
    if descriptor.is_none() && action != CodeSourceFileAction::New {
        return Err("Create a project-owned top deck before changing its lifecycle.".to_owned());
    }
    let existing_artifact_name = descriptor.map(|descriptor| descriptor.artifact_name.clone());
    let proposed_path = match action {
        CodeSourceFileAction::New => unique_top_deck_path(model, "untitled.cir"),
        CodeSourceFileAction::Rename | CodeSourceFileAction::Move => existing_artifact_name
            .clone()
            .ok_or_else(|| "The active top-deck descriptor disappeared.".to_owned())?,
        CodeSourceFileAction::Duplicate => {
            let artifact_name = existing_artifact_name
                .as_deref()
                .ok_or_else(|| "The active top-deck descriptor disappeared.".to_owned())?;
            duplicate_top_deck_path(model, artifact_name)
        }
        CodeSourceFileAction::Delete => String::new(),
    };
    netlist.lifecycle_dialog.transaction = Some(NetlistLifecycleTransaction {
        action,
        project_id: model.workspace.content.project.id(),
        deck_id: descriptor.map(|descriptor| descriptor.deck_id),
        document_id: document.map(crate::state::NetlistDocument::id),
        document_revision: document.map(crate::state::NetlistDocument::revision),
        content_digest: document.map(crate::state::NetlistDocument::content_digest),
        source_path: descriptor
            .map_or_else(String::new, |descriptor| descriptor.artifact_name.clone()),
        proposed_path,
        error: None,
    });
    Ok(())
}

pub fn commit_netlist_lifecycle_action(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
) -> Result<String, String> {
    let transaction = netlist
        .lifecycle_dialog
        .transaction
        .clone()
        .ok_or_else(|| "No top-deck lifecycle transaction is open.".to_owned())?;
    let revision_matches = match (
        transaction.deck_id,
        transaction.document_id,
        transaction.document_revision,
        transaction.content_digest,
        model.workspace.content.netlist_descriptor.as_ref(),
        model.workspace.content.netlist_document.as_ref(),
    ) {
        (
            Some(deck_id),
            Some(document_id),
            Some(revision),
            Some(digest),
            Some(descriptor),
            Some(document),
        ) => {
            descriptor.deck_id == deck_id
                && document.id() == document_id
                && document.revision() == revision
                && document.content_digest() == digest
        }
        (None, None, None, None, None, None) => {
            transaction.action == CodeSourceFileAction::New
                && model.workspace.content.netlist_source.is_none()
        }
        _ => false,
    };
    if model.workspace.content.project.id() != transaction.project_id || !revision_matches {
        return Err(
            "The active top deck changed while the lifecycle dialog was open; review it and retry."
                .to_owned(),
        );
    }

    let mut candidate = DeckTransaction::begin(model, netlist, open_documents);
    let message = match transaction.action {
        CodeSourceFileAction::New => {
            let path = normalized_top_deck_path(&transaction.proposed_path)?;
            ensure_top_deck_path_available(&candidate.model, &path, None)?;
            let source = "* New RSpice top deck\n.end\n".to_owned();
            let next = crate::state::NetlistDocument::from_authored_source(
                crate::state::NetlistDocumentId::new(),
                source.as_bytes().to_vec(),
            )
            .map_err(|error| error.to_string())?;
            let mut next_descriptor = new_owned_netlist_descriptor(
                path.clone(),
                crate::state::OwnedNetlistEditStrategy::OwnedSource,
                &source,
            );
            next_descriptor.retain_revision(&next, "Created new top deck")?;
            if candidate.model.workspace.content.netlist_document.is_some() {
                retain_active_top_deck(&mut candidate.model)?;
            } else if candidate
                .model
                .workspace
                .content
                .netlist_descriptor
                .is_some()
                || candidate.model.workspace.content.netlist_source.is_some()
            {
                return Err("The empty netlist workspace is internally inconsistent.".to_owned());
            }
            install_active_top_deck(
                &mut candidate.model,
                &mut candidate.netlist,
                next_descriptor,
                next,
                None,
            );
            format!("Created and selected project top deck {path}.")
        }
        CodeSourceFileAction::Rename => {
            let path = normalized_top_deck_path(&transaction.proposed_path)?;
            if top_deck_parent(&path) != top_deck_parent(&transaction.source_path) {
                return Err(
                    "Rename changes only the file name. Use Move to change the project folder."
                        .to_owned(),
                );
            }
            ensure_top_deck_path_available(&candidate.model, &path, transaction.deck_id)?;
            candidate
                .model
                .workspace
                .content
                .netlist_descriptor
                .as_mut()
                .ok_or_else(|| "The active top-deck descriptor disappeared.".to_owned())?
                .artifact_name = path.clone();
            candidate.model.workspace.content.netlist_source_dirty = true;
            format!("Renamed the project top deck to {path}.")
        }
        CodeSourceFileAction::Move => {
            let path = normalized_top_deck_path(&transaction.proposed_path)?;
            ensure_top_deck_path_available(&candidate.model, &path, transaction.deck_id)?;
            candidate
                .model
                .workspace
                .content
                .netlist_descriptor
                .as_mut()
                .ok_or_else(|| "The active top-deck descriptor disappeared.".to_owned())?
                .artifact_name = path.clone();
            candidate.model.workspace.content.netlist_source_dirty = true;
            format!("Moved the project top deck to {path}.")
        }
        CodeSourceFileAction::Duplicate => {
            let path = normalized_top_deck_path(&transaction.proposed_path)?;
            ensure_top_deck_path_available(&candidate.model, &path, None)?;
            let current = candidate
                .model
                .workspace
                .content
                .netlist_document
                .as_ref()
                .ok_or_else(|| "The active top-deck document disappeared.".to_owned())?;
            let duplicate = current
                .create_editable_copy(
                    crate::state::NetlistDocumentId::new(),
                    current.content_digest(),
                )
                .map_err(|error| error.to_string())?;
            let mut duplicate_descriptor = candidate
                .model
                .workspace
                .content
                .netlist_descriptor
                .as_ref()
                .cloned()
                .ok_or_else(|| "The active top-deck descriptor disappeared.".to_owned())?;
            duplicate_descriptor.deck_id = uuid::Uuid::new_v4();
            duplicate_descriptor.artifact_name = path.clone();
            duplicate_descriptor.external_file_sha256 = None;
            duplicate_descriptor.save_history.clear();
            duplicate_descriptor.revision_history.clear();
            for include in &mut duplicate_descriptor.owned_includes {
                include.document_id = uuid::Uuid::new_v4();
            }
            duplicate_descriptor.retain_revision(&duplicate, "Duplicated top deck")?;
            retain_active_top_deck(&mut candidate.model)?;
            install_active_top_deck(
                &mut candidate.model,
                &mut candidate.netlist,
                duplicate_descriptor,
                duplicate,
                None,
            );
            format!("Duplicated and selected project top deck {path}.")
        }
        CodeSourceFileAction::Delete => {
            let removed = transaction.source_path.clone();
            if let Some(next) = candidate
                .model
                .workspace
                .content
                .retained_netlist_decks
                .pop()
            {
                candidate
                    .model
                    .workspace
                    .content
                    .return_to_generated_netlist();
                install_active_top_deck(
                    &mut candidate.model,
                    &mut candidate.netlist,
                    next.descriptor,
                    next.document,
                    next.source_path,
                );
            } else {
                candidate
                    .model
                    .workspace
                    .content
                    .return_to_generated_netlist();
                candidate.netlist.owned_document = None;
                candidate.model.netlist_artifacts.generated_source = candidate
                    .model
                    .netlist_artifacts
                    .generated_document
                    .as_ref()
                    .map_or_else(String::new, |generated| generated.source().to_owned());
                candidate.netlist.active_document = ActiveNetlistDocument::Generated;
                candidate.netlist.active_dependency_identity = None;
                candidate.netlist.active_dependency_root = None;
                candidate.model.simulation.source.netlist_content =
                    candidate.model.netlist_artifacts.generated_source.clone();
                candidate.netlist.externally_saved_content_digest = None;
            }
            format!(
                "Removed project top deck {removed}. Any separately published native file was not deleted."
            )
        }
    };
    candidate.netlist.lifecycle_dialog.transaction = None;
    candidate.netlist.revision = candidate.netlist.revision.wrapping_add(1);
    candidate.netlist.completion_open = false;
    candidate.netlist.selected_byte_range = None;
    candidate.model.invalidate_source_evidence();
    clear_netlist_edit_journal(&mut candidate.netlist);
    candidate
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    candidate.commit_deck(model, netlist, open_documents);
    Ok(message)
}

pub fn select_retained_top_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<crate::workbench::state::WorkspaceDocumentId>,
    deck_id: uuid::Uuid,
) -> Result<(), String> {
    let index = model
        .workspace
        .content
        .retained_netlist_decks
        .iter()
        .position(|deck| deck.descriptor.deck_id == deck_id)
        .ok_or_else(|| "The selected retained top deck no longer exists.".to_owned())?;
    let mut candidate = DeckTransaction::begin(model, netlist, open_documents);
    let next = candidate
        .model
        .workspace
        .content
        .retained_netlist_decks
        .remove(index);
    retain_active_top_deck(&mut candidate.model)?;
    install_active_top_deck(
        &mut candidate.model,
        &mut candidate.netlist,
        next.descriptor,
        next.document,
        next.source_path,
    );
    candidate.model.workspace.content.netlist_source_dirty = true;
    candidate.netlist.lifecycle_dialog.transaction = None;
    candidate.netlist.revision = candidate.netlist.revision.wrapping_add(1);
    candidate.model.invalidate_source_evidence();
    clear_netlist_edit_journal(&mut candidate.netlist);
    candidate
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    candidate.commit_deck(model, netlist, open_documents);
    Ok(())
}

/// Move the model's active top deck into its retained decks, leaving no
/// active deck.
pub(crate) fn retain_active_top_deck(model: &mut Model) -> Result<(), String> {
    let descriptor = model
        .workspace
        .content
        .netlist_descriptor
        .take()
        .ok_or_else(|| "No active top-deck descriptor could be retained.".to_owned())?;
    let document = model
        .workspace
        .content
        .netlist_document
        .take()
        .ok_or_else(|| "No active top-deck document could be retained.".to_owned())?;
    if model.workspace.content.netlist_source.as_deref() != Some(document.source()) {
        return Err("The active top-deck source projection is inconsistent.".to_owned());
    }
    model.workspace.content.netlist_source = None;
    model
        .workspace
        .content
        .retained_netlist_decks
        .push(crate::state::RetainedOwnedNetlistDeck {
            descriptor,
            document,
            source_path: model.workspace.content.netlist_source_path.take(),
        });
    Ok(())
}

/// Make this deck the model's active top deck and the netlist editor's owned
/// document, in front.
pub(crate) fn install_active_top_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    descriptor: crate::state::OwnedNetlistDescriptor,
    document: crate::state::NetlistDocument,
    source_path: Option<std::path::PathBuf>,
) {
    let source = document.source().to_owned();
    model.workspace.content.netlist_source = Some(source.clone());
    model.workspace.content.netlist_document = Some(document.clone());
    model.workspace.content.netlist_descriptor = Some(descriptor);
    model.workspace.content.netlist_source_path = source_path;
    model.workspace.content.netlist_source_dirty = true;
    netlist.externally_saved_content_digest = document.saved_digest();
    netlist.owned_document = Some(document);
    netlist.active_document = ActiveNetlistDocument::OwnedSource;
    netlist.active_dependency_identity = None;
    netlist.active_dependency_root = None;
    netlist.active_document_initialized = true;
    model.simulation.source.netlist_content = source;
    if model.is_netlist_first_without_schematic() {
        // The bootstrap schematic is an implementation detail, not a second
        // design authority. Retaining its DRC receipt would leak stale
        // schematic findings into a project now owned by an authored deck.
        model.clear_all_design_checks();
        model.design_checks.drc_results = None;
        model.design_checks.drc_checked_version = 0;
    }
}

fn new_owned_netlist_descriptor(
    artifact_name: String,
    strategy: crate::state::OwnedNetlistEditStrategy,
    source: &str,
) -> crate::state::OwnedNetlistDescriptor {
    crate::state::OwnedNetlistDescriptor {
        deck_id: uuid::Uuid::new_v4(),
        artifact_name,
        strategy,
        source_encoding: crate::state::NetlistTextEncoding::Utf8,
        source_line_ending: crate::state::NetlistLineEnding::detect(source),
        imported_dialect: None,
        compatibility_reviewed: false,
        execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
        external_file_sha256: None,
        save_history: Vec::new(),
        revision_history: Vec::new(),
        owned_includes: Vec::new(),
    }
}

fn normalized_top_deck_path(path: &str) -> Result<String, String> {
    let path = path.trim().replace('\\', "/");
    crate::state::validate_owned_netlist_artifact_path(&path)?;
    Ok(path)
}

fn top_deck_parent(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(parent, _)| parent)
}

fn ensure_top_deck_path_available(
    model: &Model,
    path: &str,
    except: Option<uuid::Uuid>,
) -> Result<(), String> {
    let conflicts_with_active = model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .is_some_and(|descriptor| {
            Some(descriptor.deck_id) != except
                && descriptor.artifact_name.eq_ignore_ascii_case(path)
        });
    let conflicts_with_retained =
        model
            .workspace
            .content
            .retained_netlist_decks
            .iter()
            .any(|deck| {
                Some(deck.descriptor.deck_id) != except
                    && deck.descriptor.artifact_name.eq_ignore_ascii_case(path)
            });
    if conflicts_with_active || conflicts_with_retained {
        return Err(format!("A project top deck already owns path {path}."));
    }
    Ok(())
}

/// The preferred top-deck path when no deck of the model owns it, else the
/// first free numbered variant of it.
pub(crate) fn unique_top_deck_path(model: &Model, preferred: &str) -> String {
    if ensure_top_deck_path_available(model, preferred, None).is_ok() {
        return preferred.to_owned();
    }
    let (parent, leaf) = preferred
        .rsplit_once('/')
        .map_or(("", preferred), |value| value);
    let path = std::path::Path::new(leaf);
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("deck");
    let extension = path.extension().and_then(|value| value.to_str());
    for suffix in 2..=crate::state::MAX_PROJECT_SOURCE_FILES {
        let leaf = extension.map_or_else(
            || format!("{stem}_{suffix}"),
            |extension| format!("{stem}_{suffix}.{extension}"),
        );
        let candidate = if parent.is_empty() {
            leaf
        } else {
            format!("{parent}/{leaf}")
        };
        if ensure_top_deck_path_available(model, &candidate, None).is_ok() {
            return candidate;
        }
    }
    format!("deck_{}.cir", uuid::Uuid::new_v4().simple())
}

fn duplicate_top_deck_path(model: &Model, source: &str) -> String {
    let (parent, leaf) = source.rsplit_once('/').map_or(("", source), |value| value);
    let path = std::path::Path::new(leaf);
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("deck");
    let extension = path.extension().and_then(|value| value.to_str());
    let candidate = extension.map_or_else(
        || format!("{stem}_copy"),
        |extension| format!("{stem}_copy.{extension}"),
    );
    unique_top_deck_path(
        model,
        &if parent.is_empty() {
            candidate
        } else {
            format!("{parent}/{candidate}")
        },
    )
}

#[derive(Debug, Clone, Default)]
pub struct NetlistComparisonDialogState {
    pub open: bool,
    pub selected_history_index: usize,
}

#[derive(Debug, Clone)]
pub struct NetlistSaveDialogState {
    pub open: bool,
    pub save_as: bool,
    pub encoding: crate::state::NetlistTextEncoding,
    pub message: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NetlistExternalChangeResolution {
    #[default]
    Merge,
    KeepLocal,
    ReloadExternal,
}

impl NetlistExternalChangeResolution {
    pub(crate) const ALL: [Self; 3] = [Self::Merge, Self::KeepLocal, Self::ReloadExternal];
}

#[derive(Debug, Clone)]
pub struct NetlistExternalChangeState {
    pub path: std::path::PathBuf,
    pub expected_sha256: [u8; 32],
    pub observed_sha256: [u8; 32],
    pub local_source: String,
    pub external_source: String,
    pub base_source: Option<String>,
    pub merged_source: String,
    pub merge_conflict_count: usize,
    pub comparison: String,
    pub external_encoding: crate::state::NetlistTextEncoding,
    pub resolution: NetlistExternalChangeResolution,
    pub error: Option<String>,
}

impl Default for NetlistSaveDialogState {
    fn default() -> Self {
        Self {
            open: false,
            save_as: false,
            encoding: crate::state::NetlistTextEncoding::Utf8,
            message: "Update owned SPICE source".to_owned(),
            error: None,
        }
    }
}

/// Open the owned deck's save dialog, or its Save As, in the deck's own
/// encoding, over the model and the netlist editor's state.
pub fn open_netlist_save_dialog_in(
    model: &Model,
    netlist: &mut NetlistDocumentState,
    save_as: bool,
) {
    let encoding = model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .map_or(crate::state::NetlistTextEncoding::Utf8, |descriptor| {
            descriptor.source_encoding
        });
    netlist.save_dialog.open = true;
    netlist.save_dialog.save_as = save_as;
    netlist.save_dialog.encoding = encoding;
    netlist.save_dialog.error = None;
}

#[derive(Debug, Clone)]
pub struct NetlistExportDialogState {
    pub open: bool,
    pub format: crate::io::NetlistFormat,
    pub bundle_dependencies: bool,
    pub include_source_map: bool,
    pub error: Option<String>,
}

impl Default for NetlistExportDialogState {
    fn default() -> Self {
        Self {
            open: false,
            format: crate::io::NetlistFormat::Spice,
            bundle_dependencies: true,
            include_source_map: true,
            error: None,
        }
    }
}

impl Default for NetlistOwnershipDialogState {
    fn default() -> Self {
        Self {
            open: false,
            artifact_name: "top_override.sp".to_owned(),
            strategy: crate::state::OwnedNetlistEditStrategy::OwnedSource,
            error: None,
        }
    }
}

/// Transient state for one netlist document.
#[derive(Debug, Clone, Default)]
pub struct NetlistDocumentState {
    /// Canonical project-owned document retained independently from the
    /// generated primary.
    pub owned_document: Option<crate::state::NetlistDocument>,
    /// Prior immutable artifacts retained for deterministic revision compare.
    pub generated_history: Vec<crate::state::GeneratedArtifact>,
    /// Read-only unified comparison document (`generated.diff`).
    pub generated_diff_source: String,
    /// Document selected before entering a read-only revision comparison.
    pub comparison_return_document: ActiveNetlistDocument,
    /// Active central document.
    pub active_document: ActiveNetlistDocument,
    /// Logical identity of the exact retained dependency projected into the
    /// editor, or `None` when the selected generated/owned root is visible.
    pub active_dependency_identity: Option<String>,
    /// Root document that owns the active retained dependency. Stored
    /// separately so generated and owned closures cannot be confused during
    /// navigation or mutation.
    pub active_dependency_root: Option<ActiveNetlistDocument>,
    /// Whether initial active-document selection has been reconciled with a
    /// just-opened project's persisted owned source.
    pub active_document_initialized: bool,
    /// Input digest of the latest generation that failed. The same inputs
    /// fail the same way, so they are not rebuilt, and the failure is not
    /// recorded again, until one of them changes.
    pub failed_generation_input_digest: Option<crate::product::ContentDigest>,
    /// Content digest most recently published through Save Source/Save As.
    pub externally_saved_content_digest: Option<crate::product::ContentDigest>,
    /// Find/replace surface and selection state.
    pub find: NetlistFindState,
    /// Explicit multi-source replacement history. Ordinary editor keystrokes
    /// keep using the text editor's per-document history; these entries cover
    /// transactions that change one or more canonical project sources.
    edit_undo: Vec<NetlistEditJournalEntry>,
    edit_redo: Vec<NetlistEditJournalEntry>,
    pub rename_dialog: language::NetlistRenameDialogState,
    pub signature_help: language::NetlistSignatureHelpState,
    pub ownership_dialog: NetlistOwnershipDialogState,
    pub lifecycle_dialog: NetlistLifecycleDialogState,
    pub comparison_dialog: NetlistComparisonDialogState,
    pub save_dialog: NetlistSaveDialogState,
    /// Native external-file compare/merge transaction. Browser downloads have
    /// no reopenable path authority and therefore never create this state.
    pub external_change: Option<NetlistExternalChangeState>,
    pub export_dialog: NetlistExportDialogState,
    /// Review-before-commit state for a bounded netlist import. `Some` also
    /// owns the project lifecycle replacement transaction until Commit or
    /// Cancel, so no stale picker/review can replace newer project data.
    pub import_review: Option<NetlistImportReviewState>,
    /// Revision-bound source/permission reacquisition transaction for one
    /// retained include. Browser picker completion is non-abortable, so this
    /// lease must still match before any bytes can commit.
    pub dependency_relink: Option<NetlistDependencyRelinkState>,
    /// One-based source line requested by outline, diagnostics, or find. The
    /// editor consumes this exactly once and places the caret there.
    pub requested_line: Option<usize>,
    /// One-based line selected through the current schematic's authoritative
    /// generated source map. Unlike the caret this can remain highlighted
    /// while another editor interaction has focus.
    pub cross_probe_line: Option<usize>,
    /// Buffer revision, bumped on every edit.
    pub revision: u64,
    /// Revision the diagnostics were parsed for.
    diag_revision: Option<u64>,
    /// Library folders the diagnostics resolved includes through; a change
    /// in Preferences stales them as an edit does.
    diag_library_folders: Vec<std::path::PathBuf>,
    /// `ui.input(..).time` of the last edit, for parse debounce.
    last_edit_time: f64,
    /// Current parse diagnostics.
    pub diagnostics: std::sync::Arc<NetlistDiagnosticCollection>,
    /// Result data version last reconciled with the editor baseline.
    seen_data_version: u64,
    /// Working deck the run snapshot was opened from, so its comparison knows
    /// which revision to diff against once the snapshot itself is active.
    pub(crate) run_snapshot_return_document: ActiveNetlistDocument,
    /// Which executed deck the run-snapshot document is showing.
    ///
    /// Absent means it is showing this session's manual-deck baseline: the
    /// deck somebody typed and ran, which is the only one there is a working
    /// copy to compare against. Present means it is showing the source one
    /// point of a completed run was actually handed — a different artifact,
    /// for a different question, with nothing to diff it against.
    pub executed_deck_view: Option<ExecutedDeckSelection>,
    /// Zero-based line containing the caret.
    pub cursor_line: usize,
    /// Unicode-scalar caret position in the visible source. Language actions
    /// use this exact editor result rather than guessing from the line.
    pub cursor_char_index: usize,
    /// Last non-empty editor selection expressed as an exact byte range in
    /// the visible source. Selection-scoped find never guesses from a line.
    pub selected_byte_range: Option<std::ops::Range<usize>>,
    /// Whether the completion popover was open last frame.
    pub completion_open: bool,
    /// Selected completion row.
    pub completion_index: usize,
    /// Revision at which completion was dismissed.
    pub completion_dismissed_at: Option<u64>,
    /// Harvested `.model` and `.subckt` symbols.
    symbols: Vec<completion::SymbolEntry>,
    /// Revision-keyed semantic indexes for generated and project-owned source
    /// closures. Exact document revisions make stale indexes unobservable.
    project_indexes: HashMap<crate::state::NetlistDocumentId, language::CachedProjectIndex>,
    /// Buffer-revision-keyed value spans behind the editor's value hover.
    value_index: Option<language::CachedValueIndex>,
    /// What the parse of one buffer revision left unread in the buffer: the
    /// lines of an untaken `.if` branch or an unselected inline `.lib`
    /// section. Absent when that parse failed, or read a deck other than the
    /// buffer as its own root.
    unread_source: Option<(
        u64,
        std::sync::Arc<[rspice_core::netlist::UnreadSourceRegion]>,
    )>,
    /// Outline and line offsets of the visible buffer. Read through
    /// [`visible_source_index_in`], which is what keeps it current.
    source_index: std::sync::Arc<crate::state::NetlistSourceIndex>,
    /// The same for the project's top-level deck while the editor shows other
    /// text. Read through [`root_deck_source_index`].
    root_deck_index: std::sync::Arc<crate::state::NetlistSourceIndex>,
}

impl NetlistDocumentState {
    /// Take the schematic cross-probe's line: withdraw the request it made
    /// before while nothing has asked for another line, mark `line`, and ask
    /// for it while the generated deck is the document shown.
    pub fn project_cross_probe_line(&mut self, released: Option<usize>, line: Option<usize>) {
        if released.is_some() && self.requested_line == released {
            self.requested_line = None;
        }
        self.cross_probe_line = line;
        if line.is_some() && self.active_document == ActiveNetlistDocument::Generated {
            self.requested_line = line;
        }
    }

    /// The buffer revision the findings this editor holds were validated
    /// against, or `None` where they are no validation: nothing has been
    /// parsed yet, or the comparison is showing, for which the editor clears
    /// its findings without parsing anything.
    pub(crate) fn validated_revision(&self) -> Option<u64> {
        if self.active_document == ActiveNetlistDocument::GeneratedDiff {
            return None;
        }
        self.diag_revision
    }

    /// Record a settled parse of `revision`, as the editor does once it has
    /// validated the buffer.
    #[cfg(test)]
    pub fn mark_validated_for_test(&mut self, revision: u64) {
        self.diag_revision = Some(revision);
    }

    /// What the parse of `revision` left unread in the buffer, when that
    /// parse has run and read the buffer as its own root.
    pub(crate) fn unread_source_at(
        &self,
        revision: u64,
    ) -> Option<std::sync::Arc<[rspice_core::netlist::UnreadSourceRegion]>> {
        self.unread_source
            .as_ref()
            .filter(|(parsed, _)| *parsed == revision)
            .map(|(_, unread)| std::sync::Arc::clone(unread))
    }

    /// Whether a retained Code-workspace transaction owns exclusive input.
    /// This is queried before painting, so keyboard shortcuts cannot mutate
    /// the document behind a modal during its opening frame.
    pub fn application_modal_open(&self) -> bool {
        self.find.open
            || self.rename_dialog.open
            || self.ownership_dialog.open
            || self.lifecycle_dialog.transaction.is_some()
            || self.lifecycle_dialog.include_transaction.is_some()
            || self.comparison_dialog.open
            || self.save_dialog.open
            || self.external_change.is_some()
            || self.export_dialog.open
            || self.import_review.is_some()
            || self.dependency_relink.is_some()
    }
}

/// Reconcile queued execution and diff state before rendering the document,
/// over the model, the netlist editor's state, the runs and the session.
pub fn prepare(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    runs: &mut RunsState,
    workbench: &mut crate::workbench::state::WorkbenchSession,
) {
    if runs.manual_run.rerun_queued && !model.simulation.execution.has_active_execution() {
        runs.manual_run.rerun_queued = false;
        if let Some(reason) = manual_deck_run_block_reason_in(model, netlist, workbench.messages())
        {
            model.record(
                crate::diagnostics::LogSeverity::Warning,
                crate::diagnostics::LogSource::Run,
                format!("Queued netlist re-run cancelled: {reason}"),
            );
        } else {
            request_netlist_manual_deck_run_in(model, runs, workbench);
        }
    }

    let data_version = model.simulation.view.data_version;
    if netlist.seen_data_version == data_version {
        return;
    }
    netlist.seen_data_version = data_version;
    if let Some(baseline) = runs.manual_run.last_run_buffer.as_deref() {
        runs.manual_run.edited_lines = baseline::changed_lines_against_baseline(
            &model.simulation.source.netlist_content,
            baseline,
        );
    }
}

/// Run the Netlist workspace's deck: remember the page Run was pressed on,
/// and start the run, or queue it behind the one going. It writes the model,
/// the runs and the session.
pub fn request_netlist_manual_deck_run_in(
    model: &mut Model,
    runs: &mut RunsState,
    workbench: &mut crate::workbench::state::WorkbenchSession,
) {
    workbench.run_press_route = Some(workbench.current_route());
    runs.run_watch.finished_run = None;
    model.simulation.execution.run_intent = crate::state::SimulationRunIntent::ManualDeck;
    if model.simulation.execution.has_active_execution() {
        runs.manual_run.rerun_queued = true;
    } else {
        model.simulation.execution.request_manual_deck_run();
    }
}

/// Mark the deck's lines that differ from the last manual run's buffer, over
/// the model and the runs.
fn refresh_diff_pips_from_baseline_in(model: &Model, runs: &mut RunsState) {
    let Some(baseline) = runs.manual_run.last_run_buffer.as_deref() else {
        return;
    };
    runs.manual_run.edited_lines = baseline::changed_lines_against_baseline(
        &model.simulation.source.netlist_content,
        baseline,
    );
}

#[cfg(test)]
pub(crate) mod fixtures;
#[cfg(test)]
mod tests;
