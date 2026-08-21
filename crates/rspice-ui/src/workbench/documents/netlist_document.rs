//! Netlist document engine.
//!
//! This module owns editor diagnostics, syntax highlighting, completion,
//! parameter tuning, and run-to-run diff state. It has no application-chrome
//! chrome; the canonical Netlist workspace composes it as a document surface.

use std::collections::{HashMap, HashSet};

use super::{CodeSourceFileAction, CodeWorkspacePage};
use crate::state::same_include_graph;
use crate::workbench::AppState;

const MAX_OPEN_NETLIST_SECONDARY_DOCUMENTS: usize = 128;

mod baseline;
mod comparison;
mod completion;
pub(crate) mod diagnostics;
mod editor;
mod highlight;
pub(crate) mod language;
mod param_scan;
mod relink;
pub(crate) use relink::{
    NetlistDependencyRelinkState, begin_dependency_relink, cancel_dependency_relink,
    commit_dependency_relink,
};

pub use comparison::{
    close_revision_comparison, compare_generated_revision, compare_owned_revision,
    compare_run_deck_snapshot, open_run_deck_snapshot, restore_owned_revision,
    reveal_executed_deck,
};
pub(crate) use comparison::{
    close_run_deck_snapshot, open_netlist_comparison, run_deck_snapshot_artifact_name,
    run_deck_snapshot_run_id,
};
pub use diagnostics::{
    Diagnostic, DiagnosticSeverity, NetlistDiagnosticCollection, control_disposition_summary,
};
pub(crate) use editor::editor_id;
pub use editor::show as show_editor;

/// Stable identity for the exact UTF-8 source bytes visible in the editor.
pub fn source_content_digest(source: &str) -> crate::product::ContentDigest {
    crate::state::content_digest(source)
}

/// Exact, derived state of one project-owned netlist publication.
///
/// Project dirtiness, external publication, validation, and run eligibility
/// are independent facts. Keeping the projection here prevents the inspector,
/// toolbar, and execution gate from inventing contradictory interpretations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OwnedNetlistPublicationState {
    pub project_modified: bool,
    pub source_modified: bool,
    pub published_before: bool,
    pub saved: bool,
    pub validated: bool,
    pub external_change_pending: bool,
    pub run_eligible: bool,
}

impl OwnedNetlistPublicationState {
    #[must_use]
    pub const fn label(self) -> &'static str {
        if self.external_change_pending {
            "external change · review required"
        } else if !self.published_before && self.validated {
            "validated · not externally synchronized"
        } else if !self.published_before {
            "owned · validation required"
        } else if self.source_modified && self.validated {
            "modified · validated"
        } else if self.source_modified {
            "modified · validation required"
        } else if self.saved && self.validated && self.project_modified {
            "externally synchronized · validated · project modified"
        } else if self.saved && self.validated {
            "externally synchronized · validated"
        } else if self.saved {
            "externally synchronized · validation required"
        } else {
            "owned · validation required"
        }
    }
}

#[must_use]
pub(crate) fn owned_netlist_publication_state(
    state: &AppState,
    digest: crate::product::ContentDigest,
) -> OwnedNetlistPublicationState {
    let saved_digest = state.ui.netlist.externally_saved_content_digest;
    let saved = saved_digest == Some(digest);
    let validated = state.ui.netlist.validation.as_ref().is_some_and(|receipt| {
        receipt.visible_content_digest == digest
            && receipt.project_revision == state.workspace.project.revision().get()
    });
    OwnedNetlistPublicationState {
        project_modified: state.workspace.netlist_source_dirty,
        source_modified: saved_digest.is_some() && !saved,
        published_before: saved_digest.is_some(),
        saved,
        validated,
        external_change_pending: state.ui.netlist.external_change.is_some(),
        // Execution consumes the exact validated in-memory snapshot. External
        // synchronization is deliberately independent from run readiness.
        run_eligible: validated,
    }
}

/// The outline and line offsets of the buffer the editor is showing.
///
/// Parsing costs the whole deck and the navigator reads the outline on every
/// frame, so a large netlist would spend the frame budget re-deriving a
/// structure that did not change. The index is rebuilt only when the bytes
/// differ from the ones it was built from: that comparison is a memcmp, and
/// unlike a revision counter it cannot go stale behind a writer that forgets
/// to bump it.
pub(crate) fn visible_source_index(
    state: &mut AppState,
) -> std::sync::Arc<crate::state::NetlistSourceIndex> {
    let source = state.simulation.netlist_content.as_str();
    if !state.ui.netlist.source_index.describes(source) {
        state.ui.netlist.source_index =
            std::sync::Arc::new(crate::state::NetlistSourceIndex::parse(source));
    }
    std::sync::Arc::clone(&state.ui.netlist.source_index)
}

/// Invalidate byte-bound review evidence after any ownership or source edit.
pub fn invalidate_source_evidence(document: &mut NetlistDocumentState) {
    document.validation = None;
    document.validation_error = None;
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
pub(crate) struct OwnedNetlistReplacement {
    target: OwnedNetlistReplacementTarget,
    original: String,
    expected_digest: crate::product::ContentDigest,
    replacement: String,
    replacement_count: usize,
}

impl OwnedNetlistReplacement {
    #[must_use]
    pub(crate) fn root(original: &str, replacement: String, replacement_count: usize) -> Self {
        Self {
            target: OwnedNetlistReplacementTarget::Root,
            original: original.to_owned(),
            expected_digest: source_content_digest(original),
            replacement,
            replacement_count,
        }
    }

    #[must_use]
    pub(crate) fn dependency(
        logical_identity: impl Into<String>,
        original: &str,
        replacement: String,
        replacement_count: usize,
    ) -> Self {
        Self {
            target: OwnedNetlistReplacementTarget::Dependency(logical_identity.into()),
            original: original.to_owned(),
            expected_digest: source_content_digest(original),
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
            expected_digest: source_content_digest(original),
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
            expected_digest: source_content_digest(original),
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
            expected_digest: source_content_digest(&self.replacement),
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
pub(crate) fn replace_owned_sources_atomically(
    state: &mut AppState,
    replacements: Vec<OwnedNetlistReplacement>,
) -> Result<usize, String> {
    replace_owned_sources_atomically_impl(state, replacements, true)
}

fn replace_owned_sources_atomically_impl(
    state: &mut AppState,
    replacements: Vec<OwnedNetlistReplacement>,
    record_edit: bool,
) -> Result<usize, String> {
    if let Some(holder) = state.workbench.live_write_locks.netlist.as_deref() {
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
    let root_source = state
        .workspace
        .netlist_source
        .as_ref()
        .ok_or_else(|| "The project-owned root deck is no longer available.".to_owned())?;
    let current_document = state
        .ui
        .netlist
        .owned_document
        .as_ref()
        .or(state.workspace.netlist_document.as_ref())
        .ok_or_else(|| "The canonical project-owned netlist document is unavailable.".to_owned())?;
    if current_document.source() != root_source {
        return Err(
            "The project root and canonical netlist document differ; no source was changed."
                .to_owned(),
        );
    }
    let mut next_document = current_document.clone();
    let original_dependencies = current_document.dependencies().to_vec();
    let original_includes = current_document.include_directives().to_vec();
    let mut next_dependencies = original_dependencies.clone();
    let mut next_descriptor = state
        .workspace
        .netlist_descriptor
        .clone()
        .ok_or_else(|| "Owned netlist metadata is unavailable.".to_owned())?;
    let mut next_retained_decks = state.workspace.retained_netlist_decks.clone();
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
                if source_content_digest(root_source) != edit.expected_digest
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
                        format!("Owned include {identity:?} is no longer replacement-eligible.")
                    })?;
                let index = original_dependencies
                    .iter()
                    .position(|dependency| dependency.locator().logical_identity() == identity)
                    .ok_or_else(|| {
                        format!(
                            "Owned include {identity:?} is no longer in the dependency closure."
                        )
                    })?;
                let original = original_dependencies[index]
                    .source()
                    .ok_or_else(|| format!("Owned include {identity:?} is no longer resolved."))?;
                if source_content_digest(original) != edit.expected_digest
                    || include.content_digest != edit.expected_digest
                {
                    return Err(format!(
                        "Owned include {identity:?} changed after search results were produced; no source was changed."
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
                    .ok_or_else(|| format!("Owned include {identity:?} revision overflowed."))?;
                include.content_digest = source_content_digest(&edit.replacement);
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
                    || source_content_digest(deck.document.source()) != edit.expected_digest
                {
                    return Err(format!(
                        "Retained top deck {:?} changed after search results were produced; no source was changed.",
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
                        "Workspace-wide replacement would change the include graph of {:?}. Edit that top deck directly, then resolve its dependencies before retrying.",
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
                            "Owned include {logical_identity:?} is no longer in retained top deck {:?}.",
                            deck.descriptor.artifact_name
                        )
                    })?;
                let original = deck.document.dependencies()[dependency_index]
                    .source()
                    .ok_or_else(|| {
                        format!(
                            "Owned include {logical_identity:?} in retained top deck {:?} is no longer resolved.",
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
                            "Owned include {logical_identity:?} in retained top deck {:?} is no longer replacement-eligible.",
                            deck.descriptor.artifact_name
                        )
                    })?;
                if source_content_digest(&original) != edit.expected_digest
                    || include.content_digest != edit.expected_digest
                {
                    return Err(format!(
                        "Owned include {logical_identity:?} in retained top deck {:?} changed after search results were produced; no source was changed.",
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
                    format!("Owned include {logical_identity:?} revision overflowed.")
                })?;
                include.content_digest = source_content_digest(&edit.replacement);
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

    let mut candidate = state.clone();
    if root_changed
        && !candidate
            .workspace
            .replace_editable_netlist_source(next_root_source.clone())
    {
        return Err("The project-owned root deck could not be replaced.".to_owned());
    }
    candidate.workspace.netlist_descriptor = Some(next_descriptor);
    candidate.workspace.netlist_document = Some(next_document.clone());
    candidate.workspace.retained_netlist_decks = next_retained_decks;
    candidate.workspace.netlist_source_dirty = true;
    candidate.ui.netlist.owned_document = Some(next_document.clone());
    if candidate.ui.netlist.active_document == ActiveNetlistDocument::OwnedSource {
        candidate.simulation.netlist_content = candidate
            .ui
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
    candidate.ui.netlist.revision = candidate.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut candidate.ui.netlist);
    candidate
        .workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    if record_edit && !applied_replacements.is_empty() {
        let reverse = applied_replacements
            .iter()
            .rev()
            .map(OwnedNetlistReplacement::reversed)
            .collect();
        record_netlist_edit(
            &mut candidate.ui.netlist,
            NetlistEditJournalEntry {
                description: format!("replace {changed_count} netlist match(es)"),
                forward: applied_replacements,
                reverse,
            },
        );
    }
    *state = candidate;
    Ok(changed_count)
}

#[must_use]
pub(crate) fn can_undo_netlist_edit(state: &AppState) -> bool {
    !state.ui.netlist.edit_undo.is_empty()
}

#[must_use]
pub(crate) fn can_redo_netlist_edit(state: &AppState) -> bool {
    !state.ui.netlist.edit_redo.is_empty()
}

pub(crate) fn undo_netlist_edit(state: &mut AppState) -> Result<Option<String>, String> {
    let Some(entry) = state.ui.netlist.edit_undo.pop() else {
        return Ok(None);
    };
    if let Err(error) = replace_owned_sources_atomically_impl(state, entry.reverse.clone(), false) {
        state.ui.netlist.edit_undo.push(entry);
        return Err(error);
    }
    let description = entry.description.clone();
    state.ui.netlist.edit_redo.push(entry);
    Ok(Some(description))
}

pub(crate) fn redo_netlist_edit(state: &mut AppState) -> Result<Option<String>, String> {
    let Some(entry) = state.ui.netlist.edit_redo.pop() else {
        return Ok(None);
    };
    if let Err(error) = replace_owned_sources_atomically_impl(state, entry.forward.clone(), false) {
        state.ui.netlist.edit_redo.push(entry);
        return Err(error);
    }
    let description = entry.description.clone();
    state.ui.netlist.edit_undo.push(entry);
    Ok(Some(description))
}

/// Atomically replace the exact project-owned source across the canonical
/// document, persisted project projection, and visible editor buffer.
pub fn replace_owned_source(state: &mut AppState, source: String) -> bool {
    if let Some(holder) = state.workbench.live_write_locks.netlist.clone() {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
            "{holder} holds the netlist write lease; no source was changed."
        )));
        return false;
    }
    replace_owned_source_unlocked(state, source)
}

fn replace_root_document_preserving_dependencies(
    document: &crate::state::NetlistDocument,
    source: &str,
) -> Result<crate::state::NetlistDocument, String> {
    let next_dependencies = dependencies_for_rewritten_root(document, source)?;
    let mut next = document.clone();
    next.replace_editable_source(next.content_digest(), source.as_bytes().to_vec())
        .map_err(|error| error.to_string())?;
    next.acknowledge_dependencies(next.content_digest(), next_dependencies)
        .map_err(|error| error.to_string())?;
    Ok(next)
}

/// Rebind unchanged direct include edges to their new source-order indices and
/// retain only transitive members still reachable from those edges. New or
/// renamed include cards enter the closure unresolved and can then use the
/// ordinary relink/resolution workflow. This makes adding, ordering, and
/// removing include cards a normal atomic editor operation instead of
/// discarding every unaffected dependency or leaving stale owned authority.
fn dependencies_for_rewritten_root(
    document: &crate::state::NetlistDocument,
    source: &str,
) -> Result<Vec<crate::state::DependencyMetadata>, String> {
    let includes = crate::state::parse_include_directives(source);
    let original = document.dependencies();
    let mut next = Vec::new();
    let mut reachable = HashSet::new();

    for (index, include) in includes.iter().enumerate() {
        let requested = include.locator();
        if let Some(existing) = original.iter().find(|dependency| {
            dependency.direct_include_index().is_some()
                && dependency.parent().is_none()
                && dependency.requested_locator() == requested
        }) {
            let retained = crate::state::DependencyMetadata::unresolved_direct_to(
                index,
                requested,
                existing.locator().clone(),
            )
            .map_err(|error| error.to_string())?
            .with_authority(existing.authority())
            .with_resolution(existing.resolution().clone())
            .map_err(|error| error.to_string())?;
            reachable.insert(retained.locator().logical_identity().to_owned());
            next.push(retained);
        } else {
            let display_name = std::path::Path::new(requested).file_name().map_or_else(
                || requested.to_owned(),
                |name| name.to_string_lossy().into_owned(),
            );
            let locator = crate::state::SourceLocator::try_new(requested, display_name)
                .map_err(|error| error.to_string())?;
            reachable.insert(locator.logical_identity().to_owned());
            next.push(
                crate::state::DependencyMetadata::unresolved_direct_to(index, requested, locator)
                    .map_err(|error| error.to_string())?,
            );
        }
    }

    loop {
        let before = next.len();
        let newly_reachable = original
            .iter()
            .filter(|dependency| {
                dependency
                    .parent()
                    .is_some_and(|parent| reachable.contains(parent.logical_identity()))
                    && !reachable.contains(dependency.locator().logical_identity())
            })
            .cloned()
            .collect::<Vec<_>>();
        for dependency in newly_reachable {
            reachable.insert(dependency.locator().logical_identity().to_owned());
            next.push(dependency);
        }
        if next.len() == before {
            break;
        }
    }
    Ok(next)
}

fn prune_owned_include_authority(
    descriptor: &mut crate::state::OwnedNetlistDescriptor,
    document: &crate::state::NetlistDocument,
) {
    let retained = document
        .dependencies()
        .iter()
        .map(|dependency| dependency.locator().logical_identity())
        .collect::<HashSet<_>>();
    descriptor
        .owned_includes
        .retain(|include| retained.contains(include.logical_identity.as_str()));
}

/// Apply a live-session-authorized owned-source update independently of the
/// currently visible netlist document. Repeated delivery is convergence.
pub(crate) fn apply_live_owned_source(state: &mut AppState, source: String) -> bool {
    let Some(current) = state.workspace.netlist_source.as_deref() else {
        return false;
    };
    if current == source {
        return true;
    }
    let next_document = if let Some(document) = &state.workspace.netlist_document {
        let Ok(next) = replace_root_document_preserving_dependencies(document, &source) else {
            return false;
        };
        Some(next)
    } else {
        None
    };
    let mut candidate = state.clone();
    if !candidate
        .workspace
        .replace_editable_netlist_source(source.clone())
    {
        return false;
    }
    if candidate.ui.netlist.active_document == ActiveNetlistDocument::OwnedSource
        && candidate.ui.netlist.active_dependency_identity.is_none()
    {
        candidate.simulation.netlist_content = source;
    }
    if let Some(document) = next_document {
        if let Some(descriptor) = candidate.workspace.netlist_descriptor.as_mut() {
            prune_owned_include_authority(descriptor, &document);
        }
        candidate.workspace.netlist_document = Some(document.clone());
        candidate.ui.netlist.owned_document = Some(document);
    }
    candidate.ui.netlist.revision = candidate.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut candidate.ui.netlist);
    clear_netlist_edit_journal(&mut candidate.ui.netlist);
    if candidate
        .workspace
        .validate_simulation_configuration()
        .is_err()
    {
        return false;
    }
    *state = candidate;
    true
}

fn replace_owned_source_unlocked(state: &mut AppState, source: String) -> bool {
    if state.ui.netlist.active_document != ActiveNetlistDocument::OwnedSource
        || state.ui.netlist.active_dependency_identity.is_some()
    {
        return false;
    }
    let next_document = if let Some(document) = &state.ui.netlist.owned_document {
        let Ok(next) = replace_root_document_preserving_dependencies(document, &source) else {
            return false;
        };
        Some(next)
    } else {
        None
    };
    let mut candidate = state.clone();
    if !candidate
        .workspace
        .replace_editable_netlist_source(source.clone())
    {
        return false;
    }
    candidate.simulation.netlist_content = source;
    if let Some(document) = next_document {
        if let Some(descriptor) = candidate.workspace.netlist_descriptor.as_mut() {
            prune_owned_include_authority(descriptor, &document);
        }
        candidate.workspace.netlist_document = Some(document.clone());
        candidate.ui.netlist.owned_document = Some(document);
    }
    candidate.ui.netlist.revision = candidate.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut candidate.ui.netlist);
    clear_netlist_edit_journal(&mut candidate.ui.netlist);
    let Err(error) = candidate.workspace.validate_simulation_configuration() else {
        *state = candidate;
        return true;
    };
    state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
        "The netlist edit was not applied because it would invalidate the owned source graph: {error}"
    )));
    false
}

fn canonical_root_document(
    state: &AppState,
    root: ActiveNetlistDocument,
) -> Option<&crate::state::NetlistDocument> {
    match root {
        ActiveNetlistDocument::Generated => state.ui.netlist.generated_document.as_ref(),
        ActiveNetlistDocument::OwnedSource => state.ui.netlist.owned_document.as_ref(),
        ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => None,
    }
}

/// Exact retained dependency currently projected as a first-class source
/// document. The dependency source remains canonical inside its root
/// `NetlistDocument`; this function never synthesizes a second source store.
pub fn active_dependency(state: &AppState) -> Option<&crate::state::DependencyMetadata> {
    let identity = state.ui.netlist.active_dependency_identity.as_deref()?;
    let root = state.ui.netlist.active_dependency_root?;
    canonical_root_document(state, root)?
        .dependencies()
        .iter()
        .find(|dependency| dependency.locator().logical_identity() == identity)
}

/// Open the exact source location owned by a canonical diagnostic. Problems,
/// inspector actions, and future validation reports all route through this
/// function so included-source diagnostics cannot be misrepresented as root
/// buffer lines.
pub(crate) fn open_diagnostic_location(
    state: &mut AppState,
    diagnostic_id: uuid::Uuid,
) -> Result<(), String> {
    let diagnostic = state
        .ui
        .netlist
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.canonical.diagnostic_id == diagnostic_id)
        .cloned()
        .ok_or_else(|| "The selected diagnostic is no longer available.".to_owned())?;
    if !diagnostic.is_current() {
        return Err(
            "The selected diagnostic belongs to stale source; validate the current document first."
                .to_owned(),
        );
    }

    let included_source = diagnostic
        .line
        .is_none()
        .then(|| diagnostic.source_path.as_deref())
        .flatten();
    open_source_location(
        state,
        included_source,
        diagnostic.line.or(diagnostic.source_line),
    )
}

/// Open one source location in the Netlist workspace.
///
/// `included_source`, when present, names a file inside the root document's
/// authenticated include closure; it is resolved through that closure so an
/// included-source location cannot be misrepresented as a root-buffer line,
/// and an unresolvable one refuses rather than landing somewhere plausible.
///
/// Problems rows and the simulation-preflight report both route here. That is
/// the point: a location has to mean the same thing wherever it was reported
/// from, and the preflight report used to have no route to the netlist at all.
pub(crate) fn open_source_location(
    state: &mut AppState,
    included_source: Option<&std::path::Path>,
    line: Option<usize>,
) -> Result<(), String> {
    let root = state
        .ui
        .netlist
        .active_dependency_root
        .unwrap_or(state.ui.netlist.active_document);
    if let Some(source_path) = included_source {
        let dependency_identity = canonical_root_document(state, root)
            .and_then(|document| {
                document.dependencies().iter().find(|dependency| {
                    let locator = dependency.locator();
                    locator.native_origin().is_some_and(|origin| {
                        std::path::Path::new(origin) == source_path
                    }) || std::path::Path::new(locator.logical_identity()) == source_path
                        || std::path::Path::new(locator.locator()) == source_path
                })
            })
            .map(|dependency| dependency.locator().logical_identity().to_owned())
            .ok_or_else(|| {
                format!(
                    "Diagnostic source {} is no longer present in the authenticated include closure.",
                    source_path.display()
                )
            })?;
        open_netlist_dependency_from_root(state, root, &dependency_identity)?;
    }

    state.ui.code_workspace.page = CodeWorkspacePage::Netlist;
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    if let Some(line) = line {
        state.ui.netlist.cursor_line = line;
        state.ui.netlist.requested_line = Some(line.saturating_add(1));
    }
    Ok(())
}

/// Whether the visible retained include has been explicitly copied into the
/// project and is therefore an editable project document.
#[must_use]
pub fn active_dependency_is_owned(state: &AppState) -> bool {
    let Some(identity) = state.ui.netlist.active_dependency_identity.as_deref() else {
        return false;
    };
    state.ui.netlist.active_dependency_root == Some(ActiveNetlistDocument::OwnedSource)
        && state
            .workspace
            .netlist_descriptor
            .as_ref()
            .and_then(|descriptor| descriptor.owned_include(identity))
            .is_some()
}

/// Whether editor mutations are authorized for the exact visible source.
#[must_use]
pub fn active_netlist_source_is_editable(state: &AppState) -> bool {
    if state.ui.netlist.active_dependency_identity.is_some() {
        active_dependency_is_owned(state)
    } else {
        state.ui.netlist.active_document == ActiveNetlistDocument::OwnedSource
            && state.workspace.has_editable_netlist_source()
    }
}

/// Which netlist document a manual deck run would take its source from.
///
/// Before the session has activated a document, a retained project-owned
/// source outranks the generated primary; afterwards the active document is
/// the answer and nothing infers one from the visible buffer.
pub(crate) fn effective_active_document(state: &AppState) -> ActiveNetlistDocument {
    if state.ui.netlist.active_document_initialized {
        state.ui.netlist.active_document
    } else if state.workspace.netlist_source.is_some() {
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
pub(crate) fn working_deck_source(state: &AppState) -> &str {
    if effective_active_document(state) == ActiveNetlistDocument::OwnedSource {
        state
            .workspace
            .netlist_source
            .as_deref()
            .unwrap_or(state.simulation.netlist_content.as_str())
    } else {
        state.simulation.netlist_content.as_str()
    }
}

/// Open a resolved direct or transitive dependency without changing its root
/// document selection or ownership. Unresolved and stale identities fail
/// closed and leave the current buffer unchanged.
pub fn open_netlist_dependency(state: &mut AppState, logical_identity: &str) -> Result<(), String> {
    let root = state
        .ui
        .netlist
        .active_dependency_root
        .unwrap_or(state.ui.netlist.active_document);
    open_netlist_dependency_from_root(state, root, logical_identity)
}

pub(crate) fn open_netlist_dependency_from_root(
    state: &mut AppState,
    root: ActiveNetlistDocument,
    logical_identity: &str,
) -> Result<(), String> {
    if root == ActiveNetlistDocument::GeneratedDiff {
        return Err("Revision comparisons do not own dependency documents.".to_owned());
    }
    let document = canonical_root_document(state, root).ok_or_else(|| {
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
    if !state.workbench.netlist_open_documents.contains(&tab)
        && state.workbench.netlist_open_documents.len() >= MAX_OPEN_NETLIST_SECONDARY_DOCUMENTS
    {
        return Err(format!(
            "Close an include tab before opening another; the workspace retains at most {MAX_OPEN_NETLIST_SECONDARY_DOCUMENTS} secondary netlist documents."
        ));
    }
    state.ui.netlist.active_document = root;
    state.ui.netlist.active_dependency_root = Some(root);
    state.ui.netlist.active_dependency_identity = Some(logical_identity.to_owned());
    state.workbench.netlist_open_documents.insert(tab);
    state.ui.netlist.active_document_initialized = true;
    state.simulation.netlist_content = source;
    state.ui.netlist.requested_line = None;
    state.ui.netlist.cursor_line = 0;
    state.ui.netlist.edited_lines.clear();
    state.ui.netlist.completion_open = false;
    state.ui.netlist.completion_dismissed_at = None;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut state.ui.netlist);
    Ok(())
}

/// Return from an include document to its exact generated or owned root.
pub fn close_active_dependency(state: &mut AppState) -> bool {
    let Some(root) = state.ui.netlist.active_dependency_root.take() else {
        return false;
    };
    state.ui.netlist.active_dependency_identity = None;
    state.ui.netlist.active_document = root;
    state.simulation.netlist_content = match root {
        ActiveNetlistDocument::Generated => state.ui.netlist.generated_source.clone(),
        ActiveNetlistDocument::OwnedSource => state
            .ui
            .netlist
            .owned_document
            .as_ref()
            .map(|document| document.source().to_owned())
            .or_else(|| state.workspace.netlist_source.clone())
            .unwrap_or_default(),
        ActiveNetlistDocument::GeneratedDiff => state.ui.netlist.generated_diff_source.clone(),
        ActiveNetlistDocument::RunSnapshot => run_snapshot_source(state),
    };
    state.ui.netlist.requested_line = None;
    state.ui.netlist.cursor_line = 0;
    state.ui.netlist.completion_open = false;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut state.ui.netlist);
    refresh_diff_pips_from_baseline(state);
    true
}

/// Promote the visible read-only dependency into explicit project ownership.
/// Its stable project identity is persisted, while its exact source bytes stay
/// in the canonical dependency closure used by execution and export.
pub fn copy_active_dependency_to_project(state: &mut AppState) -> Result<uuid::Uuid, String> {
    if state.ui.netlist.active_dependency_root != Some(ActiveNetlistDocument::OwnedSource) {
        return Err(
            "Open the dependency from a project-owned root before copying it into the project."
                .to_owned(),
        );
    }
    let dependency = active_dependency(state)
        .cloned()
        .ok_or_else(|| "The selected dependency is no longer available.".to_owned())?;
    let identity = dependency.locator().logical_identity();
    if let Some(existing) = state
        .workspace
        .netlist_descriptor
        .as_ref()
        .and_then(|descriptor| descriptor.owned_include(identity))
    {
        return Ok(existing.document_id);
    }

    let include = crate::state::OwnedNetlistIncludeDescriptor::try_new(&dependency)?;
    let document_id = include.document_id;
    let mut candidate = state.clone();
    candidate
        .workspace
        .netlist_descriptor
        .as_mut()
        .ok_or_else(|| "Owned netlist metadata is unavailable.".to_owned())?
        .owned_includes
        .push(include);
    candidate.workspace.netlist_source_dirty = true;
    candidate
        .workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    candidate.ui.netlist.revision = candidate.ui.netlist.revision.wrapping_add(1);
    candidate.ui.netlist.completion_open = false;
    invalidate_source_evidence(&mut candidate.ui.netlist);
    *state = candidate;
    Ok(document_id)
}

/// Release editable project authority for the visible include while retaining
/// its exact bytes and include edge in the canonical closure. This is the
/// reversible counterpart to copy-to-project: the document becomes read-only
/// again, and no source text or external artifact is deleted.
pub fn release_active_dependency_from_project(state: &mut AppState) -> Result<(), String> {
    if state.ui.netlist.active_dependency_root != Some(ActiveNetlistDocument::OwnedSource) {
        return Err(
            "Only an include opened from the owned top deck can release project ownership."
                .to_owned(),
        );
    }
    let identity = state
        .ui
        .netlist
        .active_dependency_identity
        .clone()
        .ok_or_else(|| "No include document is active.".to_owned())?;
    let mut candidate = state.clone();
    let descriptor = candidate
        .workspace
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
    candidate.workspace.netlist_source_dirty = true;
    candidate.ui.netlist.revision = candidate.ui.netlist.revision.wrapping_add(1);
    candidate.ui.netlist.completion_open = false;
    invalidate_source_evidence(&mut candidate.ui.netlist);
    clear_netlist_edit_journal(&mut candidate.ui.netlist);
    candidate
        .workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    *state = candidate;
    Ok(())
}

/// Replace the exact bytes of an explicitly project-owned include. The root
/// deck text is unchanged; its canonical dependency closure, project
/// projection, ownership digest, validation evidence, and revision advance in
/// one validated transaction.
pub fn replace_owned_dependency_source(state: &mut AppState, source: String) -> bool {
    if !active_dependency_is_owned(state) {
        return false;
    }
    let Some(identity) = state.ui.netlist.active_dependency_identity.clone() else {
        return false;
    };
    let Some(current_document) = state.ui.netlist.owned_document.as_ref().cloned() else {
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

    let mut candidate = state.clone();
    let Some(descriptor) = candidate.workspace.netlist_descriptor.as_mut() else {
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
    candidate.workspace.netlist_document = Some(next_document.clone());
    candidate.workspace.netlist_source_dirty = true;
    candidate.ui.netlist.owned_document = Some(next_document);
    candidate.simulation.netlist_content = source;
    candidate.ui.netlist.revision = candidate.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut candidate.ui.netlist);
    clear_netlist_edit_journal(&mut candidate.ui.netlist);
    if candidate
        .workspace
        .validate_simulation_configuration()
        .is_err()
    {
        return false;
    }
    *state = candidate;
    true
}

/// Select the immutable generated primary without deleting or changing an
/// owned source or generated-diff document. This is an explicit document
/// transition; navigating among Code workspace pages must not call it.
pub fn open_generated_primary(state: &mut AppState) -> bool {
    if state.ui.netlist.generated_document.is_none()
        || state.ui.netlist.generated_source.trim().is_empty()
    {
        return false;
    }
    if state.ui.netlist.active_document == ActiveNetlistDocument::Generated
        && state.ui.netlist.active_dependency_identity.is_none()
    {
        return false;
    }
    state.ui.netlist.active_dependency_identity = None;
    state.ui.netlist.active_dependency_root = None;
    state.ui.netlist.active_document = ActiveNetlistDocument::Generated;
    state.ui.netlist.active_document_initialized = true;
    state.simulation.netlist_content = state.ui.netlist.generated_source.clone();
    state.ui.netlist.completion_open = false;
    state.ui.netlist.completion_dismissed_at = None;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut state.ui.netlist);
    true
}

/// Activate the retained project-owned root without creating or promoting a
/// document. Document tabs use this exact transition so switching tabs never
/// changes source ownership.
pub(crate) fn open_owned_primary(state: &mut AppState) -> bool {
    let Some(document) = state
        .ui
        .netlist
        .owned_document
        .as_ref()
        .or(state.workspace.netlist_document.as_ref())
    else {
        return false;
    };
    // Workspace/document-bar reconciliation may ask to activate the already
    // active tab again. That is presentation work, not a source transition:
    // advancing the editor revision and discarding the exact validation
    // receipt here also destroys the controller's one-shot authorization and
    // leaves a visibly validated manual deck unable to run.
    if state.ui.netlist.active_document == ActiveNetlistDocument::OwnedSource
        && state.ui.netlist.active_dependency_identity.is_none()
        && state.simulation.netlist_content == document.source()
    {
        return true;
    }
    let source = document.source().to_owned();
    state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    state.ui.netlist.active_dependency_identity = None;
    state.ui.netlist.active_dependency_root = None;
    state.ui.netlist.active_document_initialized = true;
    state.simulation.netlist_content = source;
    state.ui.netlist.completion_open = false;
    state.ui.netlist.completion_dismissed_at = None;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut state.ui.netlist);
    true
}

/// Runtime evidence that the exact visible deck passed the same preparation
/// contract used by execution. The receipt is invalid as soon as the visible
/// content digest changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistValidationReceipt {
    pub visible_content_digest: crate::product::ContentDigest,
    pub executable_source_digest: crate::product::ContentDigest,
    pub prepared_snapshot_digest: crate::product::ContentDigest,
    pub project_revision: u64,
    pub task_count: usize,
    pub advisory_count: usize,
}

/// The text the run-snapshot document is showing right now.
///
/// One resolution, because the document is projected from three places — when
/// it is opened, when a tab returns to it, and when the surface re-reads it —
/// and a second spelling would let one of them show a different deck than the
/// header above it names.
///
/// An executed deck this session no longer holds resolves to nothing rather
/// than to the manual baseline. Silently substituting a different document
/// under the same header is the one outcome worse than an empty viewer.
pub(crate) fn run_snapshot_source(state: &AppState) -> String {
    let Some(selection) = state.ui.netlist.executed_deck_view else {
        return state.ui.netlist.last_run_buffer.clone().unwrap_or_default();
    };
    state
        .simulation
        .executed_decks
        .get(selection.run_id)
        .and_then(|deck| deck.point(selection.point))
        .map(|point| point.deck.to_string())
        .unwrap_or_default()
}

/// One point of one completed run, as the executed-deck viewer shows it.
///
/// A run and a point, and nothing else. The deck text itself is looked up in
/// the session archive every time it is needed rather than copied here: a
/// second copy could outlive the archive that holds the first, and would then
/// be a deck nobody could say was still the one that ran.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutedDeckSelection {
    pub run_id: u64,
    pub point: usize,
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
        }
    }
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
pub(crate) struct NetlistLifecycleTransaction {
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
pub(crate) struct NetlistLifecycleDialogState {
    pub transaction: Option<NetlistLifecycleTransaction>,
    pub include_transaction: Option<NetlistIncludeLifecycleTransaction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NetlistIncludeLifecycleTransaction {
    pub action: CodeSourceFileAction,
    pub project_id: crate::product::ProjectId,
    pub root_document_id: crate::state::NetlistDocumentId,
    pub root_revision: crate::product::ObjectRevision,
    pub logical_identity: String,
    pub proposed_identity: String,
    pub error: Option<String>,
}

pub(crate) fn begin_owned_include_lifecycle_action(
    state: &mut AppState,
    action: CodeSourceFileAction,
) -> Result<(), String> {
    if !matches!(
        action,
        CodeSourceFileAction::Rename | CodeSourceFileAction::Move
    ) {
        return Err("Owned include lifecycle supports Rename and Move here.".to_owned());
    }
    if !active_dependency_is_owned(state) {
        return Err(
            "Copy the active include into the project before renaming or moving it.".to_owned(),
        );
    }
    let logical_identity = state
        .ui
        .netlist
        .active_dependency_identity
        .clone()
        .ok_or_else(|| "No include document is active.".to_owned())?;
    let document = state
        .workspace
        .netlist_document
        .as_ref()
        .ok_or_else(|| "The owned root document is unavailable.".to_owned())?;
    state.ui.netlist.lifecycle_dialog.include_transaction =
        Some(NetlistIncludeLifecycleTransaction {
            action,
            project_id: state.workspace.project.id(),
            root_document_id: document.id(),
            root_revision: document.revision(),
            proposed_identity: logical_identity.clone(),
            logical_identity,
            error: None,
        });
    Ok(())
}

pub(crate) fn commit_owned_include_lifecycle_action(
    state: &mut AppState,
) -> Result<String, String> {
    let transaction = state
        .ui
        .netlist
        .lifecycle_dialog
        .include_transaction
        .clone()
        .ok_or_else(|| "No owned-include lifecycle transaction is open.".to_owned())?;
    let document = state
        .workspace
        .netlist_document
        .as_ref()
        .ok_or_else(|| "The owned root document is unavailable.".to_owned())?;
    if state.workspace.project.id() != transaction.project_id
        || document.id() != transaction.root_document_id
        || document.revision() != transaction.root_revision
        || state.ui.netlist.active_dependency_identity.as_deref()
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

    let mut candidate = state.clone();
    let mut next_document = document.clone();
    next_document
        .rename_dependency_identity(
            transaction.root_revision,
            &transaction.logical_identity,
            locator,
        )
        .map_err(|error| error.to_string())?;
    let descriptor = candidate
        .workspace
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
    if candidate.workbench.netlist_open_documents.remove(&old_tab) {
        candidate.workbench.netlist_open_documents.insert(new_tab);
    }
    candidate.workspace.netlist_document = Some(next_document.clone());
    candidate.workspace.netlist_source_dirty = true;
    candidate.ui.netlist.owned_document = Some(next_document);
    candidate.ui.netlist.active_dependency_identity = Some(proposed.clone());
    candidate.ui.netlist.lifecycle_dialog.include_transaction = None;
    candidate.ui.netlist.revision = candidate.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut candidate.ui.netlist);
    clear_netlist_edit_journal(&mut candidate.ui.netlist);
    candidate
        .workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    *state = candidate;
    Ok(format!(
        "{} project-owned include as {proposed} without changing its retained source bytes.",
        if transaction.action == CodeSourceFileAction::Rename {
            "Renamed"
        } else {
            "Moved"
        }
    ))
}

pub(crate) fn begin_netlist_lifecycle_action(
    state: &mut AppState,
    action: CodeSourceFileAction,
) -> Result<(), String> {
    if state.ui.netlist.active_dependency_identity.is_some() {
        return Err(
            "Return to the owned root deck before changing a top-level deck lifecycle.".to_owned(),
        );
    }
    let descriptor = state.workspace.netlist_descriptor.as_ref();
    let document = state.workspace.netlist_document.as_ref();
    if descriptor.is_some() != document.is_some() {
        return Err("The active top-deck descriptor and document are inconsistent.".to_owned());
    }
    if descriptor.is_none() && action != CodeSourceFileAction::New {
        return Err("Create a project-owned top deck before changing its lifecycle.".to_owned());
    }
    let existing_artifact_name = descriptor.map(|descriptor| descriptor.artifact_name.clone());
    let proposed_path = match action {
        CodeSourceFileAction::New => unique_top_deck_path(state, "untitled.cir"),
        CodeSourceFileAction::Rename | CodeSourceFileAction::Move => existing_artifact_name
            .clone()
            .ok_or_else(|| "The active top-deck descriptor disappeared.".to_owned())?,
        CodeSourceFileAction::Duplicate => {
            let artifact_name = existing_artifact_name
                .as_deref()
                .ok_or_else(|| "The active top-deck descriptor disappeared.".to_owned())?;
            duplicate_top_deck_path(state, artifact_name)
        }
        CodeSourceFileAction::Delete => String::new(),
    };
    state.ui.netlist.lifecycle_dialog.transaction = Some(NetlistLifecycleTransaction {
        action,
        project_id: state.workspace.project.id(),
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

pub(crate) fn commit_netlist_lifecycle_action(state: &mut AppState) -> Result<String, String> {
    let transaction = state
        .ui
        .netlist
        .lifecycle_dialog
        .transaction
        .clone()
        .ok_or_else(|| "No top-deck lifecycle transaction is open.".to_owned())?;
    let revision_matches = match (
        transaction.deck_id,
        transaction.document_id,
        transaction.document_revision,
        transaction.content_digest,
        state.workspace.netlist_descriptor.as_ref(),
        state.workspace.netlist_document.as_ref(),
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
                && state.workspace.netlist_source.is_none()
        }
        _ => false,
    };
    if state.workspace.project.id() != transaction.project_id || !revision_matches {
        return Err(
            "The active top deck changed while the lifecycle dialog was open; review it and retry."
                .to_owned(),
        );
    }

    let mut candidate = state.clone();
    let message = match transaction.action {
        CodeSourceFileAction::New => {
            let path = normalized_top_deck_path(&transaction.proposed_path)?;
            ensure_top_deck_path_available(&candidate, &path, None)?;
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
            if candidate.workspace.netlist_document.is_some() {
                retain_active_top_deck(&mut candidate)?;
            } else if candidate.workspace.netlist_descriptor.is_some()
                || candidate.workspace.netlist_source.is_some()
            {
                return Err("The empty netlist workspace is internally inconsistent.".to_owned());
            }
            install_active_top_deck(&mut candidate, next_descriptor, next, None);
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
            ensure_top_deck_path_available(&candidate, &path, transaction.deck_id)?;
            candidate
                .workspace
                .netlist_descriptor
                .as_mut()
                .ok_or_else(|| "The active top-deck descriptor disappeared.".to_owned())?
                .artifact_name = path.clone();
            candidate.workspace.netlist_source_dirty = true;
            format!("Renamed the project top deck to {path}.")
        }
        CodeSourceFileAction::Move => {
            let path = normalized_top_deck_path(&transaction.proposed_path)?;
            ensure_top_deck_path_available(&candidate, &path, transaction.deck_id)?;
            candidate
                .workspace
                .netlist_descriptor
                .as_mut()
                .ok_or_else(|| "The active top-deck descriptor disappeared.".to_owned())?
                .artifact_name = path.clone();
            candidate.workspace.netlist_source_dirty = true;
            format!("Moved the project top deck to {path}.")
        }
        CodeSourceFileAction::Duplicate => {
            let path = normalized_top_deck_path(&transaction.proposed_path)?;
            ensure_top_deck_path_available(&candidate, &path, None)?;
            let current = candidate
                .workspace
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
                .workspace
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
            retain_active_top_deck(&mut candidate)?;
            install_active_top_deck(&mut candidate, duplicate_descriptor, duplicate, None);
            format!("Duplicated and selected project top deck {path}.")
        }
        CodeSourceFileAction::Delete => {
            let removed = transaction.source_path.clone();
            if let Some(next) = candidate.workspace.retained_netlist_decks.pop() {
                candidate.workspace.return_to_generated_netlist();
                install_active_top_deck(
                    &mut candidate,
                    next.descriptor,
                    next.document,
                    next.source_path,
                );
            } else {
                candidate.workspace.return_to_generated_netlist();
                candidate.ui.netlist.owned_document = None;
                candidate.ui.netlist.generated_source = candidate
                    .ui
                    .netlist
                    .generated_document
                    .as_ref()
                    .map_or_else(String::new, |generated| generated.source().to_owned());
                candidate.ui.netlist.active_document = ActiveNetlistDocument::Generated;
                candidate.ui.netlist.active_dependency_identity = None;
                candidate.ui.netlist.active_dependency_root = None;
                candidate.simulation.netlist_content =
                    candidate.ui.netlist.generated_source.clone();
                candidate.ui.netlist.externally_saved_content_digest = None;
            }
            format!(
                "Removed project top deck {removed}. Any separately published native file was not deleted."
            )
        }
    };
    candidate.ui.netlist.lifecycle_dialog.transaction = None;
    candidate.ui.netlist.revision = candidate.ui.netlist.revision.wrapping_add(1);
    candidate.ui.netlist.completion_open = false;
    candidate.ui.netlist.selected_byte_range = None;
    invalidate_source_evidence(&mut candidate.ui.netlist);
    clear_netlist_edit_journal(&mut candidate.ui.netlist);
    candidate
        .workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    *state = candidate;
    Ok(message)
}

pub(crate) fn select_retained_top_deck(
    state: &mut AppState,
    deck_id: uuid::Uuid,
) -> Result<(), String> {
    let index = state
        .workspace
        .retained_netlist_decks
        .iter()
        .position(|deck| deck.descriptor.deck_id == deck_id)
        .ok_or_else(|| "The selected retained top deck no longer exists.".to_owned())?;
    let mut candidate = state.clone();
    let next = candidate.workspace.retained_netlist_decks.remove(index);
    retain_active_top_deck(&mut candidate)?;
    install_active_top_deck(
        &mut candidate,
        next.descriptor,
        next.document,
        next.source_path,
    );
    candidate.workspace.netlist_source_dirty = true;
    candidate.ui.netlist.lifecycle_dialog.transaction = None;
    candidate.ui.netlist.revision = candidate.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut candidate.ui.netlist);
    clear_netlist_edit_journal(&mut candidate.ui.netlist);
    candidate
        .workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    *state = candidate;
    Ok(())
}

pub(crate) fn retain_active_top_deck(state: &mut AppState) -> Result<(), String> {
    let descriptor = state
        .workspace
        .netlist_descriptor
        .take()
        .ok_or_else(|| "No active top-deck descriptor could be retained.".to_owned())?;
    let document = state
        .workspace
        .netlist_document
        .take()
        .ok_or_else(|| "No active top-deck document could be retained.".to_owned())?;
    if state.workspace.netlist_source.as_deref() != Some(document.source()) {
        return Err("The active top-deck source projection is inconsistent.".to_owned());
    }
    state.workspace.netlist_source = None;
    state
        .workspace
        .retained_netlist_decks
        .push(crate::state::RetainedOwnedNetlistDeck {
            descriptor,
            document,
            source_path: state.workspace.netlist_source_path.take(),
        });
    Ok(())
}

pub(crate) fn install_active_top_deck(
    state: &mut AppState,
    descriptor: crate::state::OwnedNetlistDescriptor,
    document: crate::state::NetlistDocument,
    source_path: Option<std::path::PathBuf>,
) {
    let source = document.source().to_owned();
    state.workspace.netlist_source = Some(source.clone());
    state.workspace.netlist_document = Some(document.clone());
    state.workspace.netlist_descriptor = Some(descriptor);
    state.workspace.netlist_source_path = source_path;
    state.workspace.netlist_source_dirty = true;
    state.ui.netlist.externally_saved_content_digest = document.saved_digest();
    state.ui.netlist.owned_document = Some(document);
    state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    state.ui.netlist.active_dependency_identity = None;
    state.ui.netlist.active_dependency_root = None;
    state.ui.netlist.active_document_initialized = true;
    state.simulation.netlist_content = source;
    if state.is_netlist_first_without_schematic() {
        // The bootstrap schematic is an implementation detail, not a second
        // design authority. Retaining its DRC receipt would leak stale
        // schematic findings into a project now owned by an authored deck.
        state.clear_all_design_checks();
        state.dialogs.drc_results = None;
        state.dialogs.drc_checked_version = 0;
        state.dialogs.drc_cycle = None;
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
    state: &AppState,
    path: &str,
    except: Option<uuid::Uuid>,
) -> Result<(), String> {
    let conflicts_with_active =
        state
            .workspace
            .netlist_descriptor
            .as_ref()
            .is_some_and(|descriptor| {
                Some(descriptor.deck_id) != except
                    && descriptor.artifact_name.eq_ignore_ascii_case(path)
            });
    let conflicts_with_retained = state.workspace.retained_netlist_decks.iter().any(|deck| {
        Some(deck.descriptor.deck_id) != except
            && deck.descriptor.artifact_name.eq_ignore_ascii_case(path)
    });
    if conflicts_with_active || conflicts_with_retained {
        return Err(format!("A project top deck already owns path {path}."));
    }
    Ok(())
}

fn unique_top_deck_path(state: &AppState, preferred: &str) -> String {
    if ensure_top_deck_path_available(state, preferred, None).is_ok() {
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
        if ensure_top_deck_path_available(state, &candidate, None).is_ok() {
            return candidate;
        }
    }
    format!("deck_{}.cir", uuid::Uuid::new_v4().simple())
}

pub(crate) fn available_top_deck_path(state: &AppState, preferred: &str) -> String {
    unique_top_deck_path(state, preferred)
}

fn duplicate_top_deck_path(state: &AppState, source: &str) -> String {
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
        state,
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
pub(crate) enum NetlistExternalChangeResolution {
    #[default]
    Merge,
    KeepLocal,
    ReloadExternal,
}

impl NetlistExternalChangeResolution {
    pub(crate) const ALL: [Self; 3] = [Self::Merge, Self::KeepLocal, Self::ReloadExternal];
}

#[derive(Debug, Clone)]
pub(crate) struct NetlistExternalChangeState {
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

pub(crate) fn open_netlist_save_dialog(state: &mut AppState, save_as: bool) {
    let encoding = state
        .workspace
        .netlist_descriptor
        .as_ref()
        .map_or(crate::state::NetlistTextEncoding::Utf8, |descriptor| {
            descriptor.source_encoding
        });
    state.ui.netlist.save_dialog.open = true;
    state.ui.netlist.save_dialog.save_as = save_as;
    state.ui.netlist.save_dialog.encoding = encoding;
    state.ui.netlist.save_dialog.error = None;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NetlistImportOperation {
    OpenProject,
    ImportIntoProject,
    RequalifyOwnedSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NetlistImportIssueSeverity {
    Advisory,
    Blocking,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NetlistImportIssue {
    pub severity: NetlistImportIssueSeverity,
    pub message: String,
}

/// Lossless, revision-guarded candidate presented before a netlist import is
/// allowed to replace project state. The candidate owns the decoded source
/// snapshot and raw-file identity; accepting the dialog never re-reads a
/// different file and cancellation never mutates project-owned documents.
#[derive(Debug, Clone)]
pub(crate) struct NetlistImportReviewState {
    pub transaction: crate::product::TransactionId,
    pub operation: NetlistImportOperation,
    pub display_name: String,
    /// Native picker origin re-read at commit to reject review races. Archive
    /// imports deliberately keep `source_path` empty so Save never overwrites
    /// a ZIP with plain deck bytes.
    pub selected_file_path: Option<std::path::PathBuf>,
    pub source_path: Option<std::path::PathBuf>,
    pub source: String,
    pub dependencies: Vec<crate::state::DependencyMetadata>,
    pub archive_import: bool,
    pub original_byte_count: usize,
    pub original_sha256: [u8; 32],
    pub encoding: crate::state::NetlistTextEncoding,
    pub line_ending: crate::state::NetlistLineEnding,
    pub detected_dialect: crate::state::NetlistSourceDialect,
    pub selected_dialect: crate::state::NetlistSourceDialect,
    pub detection_evidence: Vec<String>,
    pub transformations: Vec<String>,
    pub issues: Vec<NetlistImportIssue>,
    pub compatibility_accepted: bool,
    pub error: Option<String>,
}

impl NetlistImportReviewState {
    pub(crate) fn blocking_issue_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == NetlistImportIssueSeverity::Blocking)
            .count()
    }

    pub(crate) fn can_commit(&self) -> bool {
        self.blocking_issue_count() == 0
            && self.dialect_qualification().is_ok()
            && (!self.selected_dialect.requires_compatibility_review()
                || self.compatibility_accepted)
    }

    /// Prove that the selected import profile has an executable semantic
    /// implementation. A review checkbox acknowledges a qualified profile;
    /// it can never create an adapter or turn a vendor marker into canonical
    /// RSpice syntax. This check is deliberately repeated at commit.
    pub(crate) fn dialect_qualification(&self) -> Result<(), String> {
        use crate::state::NetlistSourceDialect;

        match self.selected_dialect {
            NetlistSourceDialect::RSpice => {
                if self.detected_dialect == NetlistSourceDialect::RSpice {
                    Ok(())
                } else {
                    Err(format!(
                        "Detected {} markers cannot be relabeled as RSpice canonical SPICE. Remove or explicitly convert every foreign construct before importing.",
                        self.detected_dialect.label()
                    ))
                }
            }
            NetlistSourceDialect::Spice3Ngspice => {
                if !matches!(
                    self.detected_dialect,
                    NetlistSourceDialect::RSpice | NetlistSourceDialect::Spice3Ngspice
                ) {
                    return Err(format!(
                        "Detected {} markers cannot be reclassified as SPICE3/ngspice. Convert the foreign statements before selecting another execution profile.",
                        self.detected_dialect.label()
                    ));
                }
                crate::state::NetlistExecutionProfile::Spice3NgspiceV2
                    .validate_source(&self.source)
                    .map_err(|error| format!(
                        "SPICE3/ngspice compatibility is not qualified: {error}. Only line-preserving declarative .control/.endc commands (op, dc, ac, sp, tran, save, and bounded meas/measure) are qualified; remove imperative commands such as wrdata and setplot before import."
                    ))?;
                Ok(())
            }
            NetlistSourceDialect::Pspice => {
                if !matches!(
                    self.detected_dialect,
                    NetlistSourceDialect::RSpice | NetlistSourceDialect::Pspice
                ) {
                    return Err(format!(
                        "Detected {} markers cannot be reclassified as PSpice. Convert the foreign statements before selecting another execution profile.",
                        self.detected_dialect.label()
                    ));
                }
                crate::state::NetlistExecutionProfile::PspiceDeclarativeV2
                    .validate_source(&self.source)
                    .map_err(|error| {
                        format!(
                            "PSpice declarative compatibility is not qualified: {error}. The v2 profile requires a pre-.END .PROBE, .PROBE64, or .PROBE/CSDF source marker; admits typed DEV/LOT, .DISTRIBUTION, selected-analysis .MC collation, bounded immutable .MC LIST/OUTPUT results, and exact minimum-order Type-I E/G CHEBYSHEV LP/HP/BP/BR sources; and rejects unsupported .STIMULUS or FREQ sources and unknown output-format commands."
                        )
                    })?;
                Ok(())
            }
            NetlistSourceDialect::Hspice => {
                if !matches!(
                    self.detected_dialect,
                    NetlistSourceDialect::RSpice | NetlistSourceDialect::Hspice
                ) {
                    return Err(format!(
                        "Detected {} markers cannot be reclassified as HSPICE. Convert the foreign statements before selecting another execution profile.",
                        self.detected_dialect.label()
                    ));
                }
                crate::state::NetlistExecutionProfile::HspiceDeclarativeV1
                    .adapt_source(&self.source)
                    .map_err(|error| {
                        format!(
                            "HSPICE declarative compatibility is not qualified: {error}. The v1 adapter requires a pre-.END .OPTION POST or .PROTECT/.UNPROTECT source marker and accepts only ordinary SPICE analyses plus those simulation-neutral presentation directives."
                        )
                    })?;
                Ok(())
            }
            NetlistSourceDialect::Spectre => {
                if !matches!(
                    self.detected_dialect,
                    NetlistSourceDialect::RSpice | NetlistSourceDialect::Spectre
                ) {
                    return Err(format!(
                        "Detected {} markers cannot be reclassified as Spectre SPICE interoperability source. Convert the foreign statements before selecting another execution profile.",
                        self.detected_dialect.label()
                    ));
                }
                crate::state::NetlistExecutionProfile::SpectreSpiceV1
                    .adapt_source(&self.source)
                    .map_err(|error| {
                        format!(
                            "Spectre SPICE interoperability is not qualified: {error}. The v1 adapter requires exact `simulator lang=spice` mode and rejects native Spectre parameters, AHDL includes, saveOptions, and language switches."
                        )
                    })?;
                Ok(())
            }
            NetlistSourceDialect::Ads => {
                if !matches!(
                    self.detected_dialect,
                    NetlistSourceDialect::RSpice | NetlistSourceDialect::Ads
                ) {
                    return Err(format!(
                        "Detected {} markers cannot be reclassified as an ADS SPICE export. Convert the foreign statements before selecting another execution profile.",
                        self.detected_dialect.label()
                    ));
                }
                crate::state::NetlistExecutionProfile::AdsSpiceExportV1
                    .adapt_source(&self.source)
                    .map_err(|error| {
                        format!(
                            "ADS SPICE-export compatibility is not qualified: {error}. The v1 adapter accepts the bounded ResourceUsage/UseNutmegFormat/TopDesignName header only; #uselib, define, simulatorOptions, native ADS devices, and unresolved library semantics remain rejected."
                        )
                    })?;
                Ok(())
            }
            NetlistSourceDialect::Unknown => Err(
                "Ambiguous or unknown dialect evidence has no executable compatibility profile. Resolve the dialect and convert unsupported constructs before importing."
                    .to_owned(),
            ),
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
    /// Canonical immutable document, including generated provenance,
    /// dependency identities, source map, outline, and validation state.
    pub generated_document: Option<crate::state::NetlistDocument>,
    /// Canonical project-owned document retained independently from the
    /// generated primary.
    pub owned_document: Option<crate::state::NetlistDocument>,
    /// Prior immutable artifacts retained for deterministic revision compare.
    pub generated_history: Vec<crate::state::GeneratedArtifact>,
    /// Read-only unified comparison document (`generated.diff`).
    pub generated_diff_source: String,
    /// Document selected before entering a read-only revision comparison.
    pub comparison_return_document: ActiveNetlistDocument,
    /// Runtime-generated primary artifact retained independently from any
    /// project-owned source document.
    pub generated_source: String,
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
    /// Exact project-input digest used to produce the retained generated
    /// artifact. `None` means the current bytes have no generated authority.
    pub generated_input_digest: Option<crate::product::ContentDigest>,
    /// Input digest observed this frame. A mismatch with
    /// `generated_input_digest` makes the retained artifact stale and blocks
    /// execution until generation succeeds.
    pub current_generation_input_digest: Option<crate::product::ContentDigest>,
    /// Exact diagnostic from the latest failed generation attempt.
    pub generation_error: Option<String>,
    /// Receipt for the latest exact visible source validation.
    pub validation: Option<NetlistValidationReceipt>,
    /// Exact validation failure retained until the bytes change or validation
    /// succeeds.
    pub validation_error: Option<String>,
    /// Content digest most recently published through Save Source/Save As.
    pub externally_saved_content_digest: Option<crate::product::ContentDigest>,
    /// Find/replace surface and selection state.
    pub find: NetlistFindState,
    /// Explicit multi-source replacement history. Ordinary editor keystrokes
    /// keep using the text editor's per-document history; these entries cover
    /// transactions that change one or more canonical project sources.
    edit_undo: Vec<NetlistEditJournalEntry>,
    edit_redo: Vec<NetlistEditJournalEntry>,
    pub(crate) rename_dialog: language::NetlistRenameDialogState,
    pub(crate) signature_help: language::NetlistSignatureHelpState,
    pub ownership_dialog: NetlistOwnershipDialogState,
    pub(crate) lifecycle_dialog: NetlistLifecycleDialogState,
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
    /// `ui.input(..).time` of the last edit, for parse debounce.
    last_edit_time: f64,
    /// Current parse diagnostics.
    pub diagnostics: std::sync::Arc<NetlistDiagnosticCollection>,
    /// Zero-based lines edited since the last completed manual-deck run.
    pub edited_lines: HashSet<usize>,
    /// Result data version last reconciled with the editor baseline.
    seen_data_version: u64,
    /// Exact editor buffer from the last successful manual-deck run.
    pub last_run_buffer: Option<String>,
    /// Run sequence `last_run_buffer` was sealed with. This is the only
    /// deck-to-run binding: the run's label, deck digest and revision are read
    /// back off that run's own receipt rather than copied here.
    pub last_run_id: Option<u64>,
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
    /// Numeric `.param` values captured from `last_run_buffer`.
    pub last_run_params: HashMap<String, f64>,
    /// Editor buffer captured when the current manual-deck run started.
    pub pending_run_buffer: Option<String>,
    /// Run id associated with `pending_run_buffer`.
    pub pending_manual_run_id: Option<u64>,
    /// Zero-based line containing the caret.
    pub cursor_line: usize,
    /// Unicode-scalar caret position in the visible source. Language actions
    /// use this exact editor result rather than guessing from the line.
    pub cursor_char_index: usize,
    /// Last non-empty editor selection expressed as an exact byte range in
    /// the visible source. Selection-scoped find never guesses from a line.
    pub selected_byte_range: Option<std::ops::Range<usize>>,
    /// A re-run requested while the engine was busy.
    pub rerun_queued: bool,
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
    /// Outline and line offsets of the visible buffer. Read through
    /// [`visible_source_index`], which is what keeps it current.
    source_index: std::sync::Arc<crate::state::NetlistSourceIndex>,
}

impl NetlistDocumentState {
    /// Whether a retained Code-workspace transaction owns exclusive input.
    /// This is queried before painting, so keyboard shortcuts cannot mutate
    /// the document behind a modal during its opening frame.
    pub(crate) fn application_modal_open(&self) -> bool {
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

/// Reconcile queued execution and diff state before rendering the document.
pub fn prepare(state: &mut AppState) {
    if state.ui.netlist.rerun_queued && !state.simulation.has_active_execution() {
        state.ui.netlist.rerun_queued = false;
        if let Some(reason) = state.manual_deck_run_block_reason() {
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                "Queued netlist re-run cancelled: {reason}"
            )));
        } else {
            state.request_netlist_manual_deck_run();
        }
    }

    let data_version = state.simulation.data_version;
    if state.ui.netlist.seen_data_version == data_version {
        return;
    }
    state.ui.netlist.seen_data_version = data_version;
    if let Some(baseline) = state.ui.netlist.last_run_buffer.as_deref() {
        state.ui.netlist.edited_lines =
            baseline::changed_lines_against_baseline(&state.simulation.netlist_content, baseline);
    }
}

pub(super) fn refresh_diff_pips_from_baseline(state: &mut AppState) {
    let Some(baseline) = state.ui.netlist.last_run_buffer.as_deref() else {
        return;
    };
    state.ui.netlist.edited_lines =
        baseline::changed_lines_against_baseline(&state.simulation.netlist_content, baseline);
}

#[cfg(test)]
mod tests;
