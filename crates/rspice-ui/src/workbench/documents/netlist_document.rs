//! Netlist document engine.
//!
//! This module owns editor diagnostics, syntax highlighting, completion,
//! parameter tuning, and run-to-run diff state. It has no application-chrome
//! chrome; the canonical Netlist workspace composes it as a document surface.

use std::collections::{HashMap, HashSet};

use crate::workbench::AppState;

mod baseline;
mod completion;
pub(crate) mod diagnostics;
mod editor;
mod highlight;
mod param_scan;

pub use diagnostics::{Diagnostic, DiagnosticSeverity, NetlistDiagnosticCollection};
pub(crate) use editor::editor_id;
pub use editor::show as show_editor;

/// Stable identity for the exact UTF-8 source bytes visible in the editor.
pub fn source_content_digest(source: &str) -> crate::product::ContentDigest {
    crate::state::content_digest(source)
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
        let mut next = document.clone();
        if next
            .replace_editable_source(next.content_digest(), source.as_bytes().to_vec())
            .is_err()
        {
            return false;
        }
        Some(next)
    } else {
        None
    };
    if !state
        .workspace
        .replace_editable_netlist_source(source.clone())
    {
        return false;
    }
    if state.ui.netlist.active_document == ActiveNetlistDocument::OwnedSource
        && state.ui.netlist.active_dependency_identity.is_none()
    {
        state.simulation.netlist_content = source;
    }
    if let Some(document) = next_document {
        state.workspace.netlist_document = Some(document.clone());
        state.ui.netlist.owned_document = Some(document);
    }
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut state.ui.netlist);
    true
}

fn replace_owned_source_unlocked(state: &mut AppState, source: String) -> bool {
    if state.ui.netlist.active_document != ActiveNetlistDocument::OwnedSource
        || state.ui.netlist.active_dependency_identity.is_some()
    {
        return false;
    }
    let next_document = if let Some(document) = &state.ui.netlist.owned_document {
        let mut next = document.clone();
        if next
            .replace_editable_source(next.content_digest(), source.as_bytes().to_vec())
            .is_err()
        {
            return false;
        }
        Some(next)
    } else {
        None
    };
    if !state
        .workspace
        .replace_editable_netlist_source(source.clone())
    {
        return false;
    }
    state.simulation.netlist_content = source;
    if let Some(document) = next_document {
        state.workspace.netlist_document = Some(document.clone());
        state.ui.netlist.owned_document = Some(document);
    }
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut state.ui.netlist);
    true
}

fn canonical_root_document(
    state: &AppState,
    root: ActiveNetlistDocument,
) -> Option<&crate::state::NetlistDocument> {
    match root {
        ActiveNetlistDocument::Generated => state.ui.netlist.generated_document.as_ref(),
        ActiveNetlistDocument::OwnedSource => state.ui.netlist.owned_document.as_ref(),
        ActiveNetlistDocument::GeneratedDiff => None,
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

/// Open a resolved direct or transitive dependency without changing its root
/// document selection or ownership. Unresolved and stale identities fail
/// closed and leave the current buffer unchanged.
pub fn open_netlist_dependency(state: &mut AppState, logical_identity: &str) -> Result<(), String> {
    let root = state
        .ui
        .netlist
        .active_dependency_root
        .unwrap_or(state.ui.netlist.active_document);
    if root == ActiveNetlistDocument::GeneratedDiff {
        return Err("Revision comparisons do not own dependency documents.".to_owned());
    }
    let source = canonical_root_document(state, root)
        .and_then(|document| {
            document
                .dependencies()
                .iter()
                .find(|dependency| dependency.locator().logical_identity() == logical_identity)
        })
        .and_then(crate::state::DependencyMetadata::source)
        .ok_or_else(|| {
            "The selected dependency is unresolved or no longer belongs to this source closure."
                .to_owned()
        })?
        .to_owned();

    state.ui.netlist.active_document = root;
    state.ui.netlist.active_dependency_root = Some(root);
    state.ui.netlist.active_dependency_identity = Some(logical_identity.to_owned());
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

/// Revision-bound authority for one native or browser dependency relink
/// picker. It is transient UI state; exact relinked bytes and locator evidence
/// are persisted only after the guarded transaction commits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NetlistDependencyRelinkState {
    pub root: ActiveNetlistDocument,
    pub logical_identity: String,
    pub expected_document_id: crate::state::NetlistDocumentId,
    pub expected_revision: crate::product::ObjectRevision,
}

pub(crate) fn begin_dependency_relink(
    state: &mut AppState,
    logical_identity: &str,
) -> Result<(), String> {
    if state.ui.netlist.dependency_relink.is_some() {
        return Err("A dependency relink is already in progress.".to_owned());
    }
    let root = state
        .ui
        .netlist
        .active_dependency_root
        .unwrap_or(state.ui.netlist.active_document);
    let document = canonical_root_document(state, root)
        .ok_or_else(|| "The selected source has no canonical dependency closure.".to_owned())?;
    if !document
        .dependencies()
        .iter()
        .any(|dependency| dependency.locator().logical_identity() == logical_identity)
    {
        return Err("The dependency is no longer in the canonical source closure.".to_owned());
    }
    state.ui.netlist.dependency_relink = Some(NetlistDependencyRelinkState {
        root,
        logical_identity: logical_identity.to_owned(),
        expected_document_id: document.id(),
        expected_revision: document.revision(),
    });
    Ok(())
}

pub(crate) fn cancel_dependency_relink(state: &mut AppState) {
    state.ui.netlist.dependency_relink = None;
}

pub(crate) fn commit_dependency_relink(
    state: &mut AppState,
    source: String,
    display_name: String,
    native_origin: Option<String>,
) -> Result<(), String> {
    let transaction = state
        .ui
        .netlist
        .dependency_relink
        .clone()
        .ok_or_else(|| "No dependency relink transaction is active.".to_owned())?;
    let current = canonical_root_document(state, transaction.root)
        .cloned()
        .ok_or_else(|| "The relink root document is no longer available.".to_owned())?;
    if current.id() != transaction.expected_document_id
        || current.revision() != transaction.expected_revision
    {
        return Err(
            "The dependency closure changed while the picker was open; review it and retry."
                .to_owned(),
        );
    }
    let existing = current
        .dependencies()
        .iter()
        .find(|dependency| dependency.locator().logical_identity() == transaction.logical_identity)
        .ok_or_else(|| "The dependency is no longer in the canonical closure.".to_owned())?;
    let locator = existing
        .locator()
        .clone()
        .with_relinked_origin(display_name, native_origin)
        .map_err(|error| error.to_string())?;
    let mut next = current;
    next.relink_dependency_source(
        transaction.expected_revision,
        &transaction.logical_identity,
        locator,
        source.as_bytes().to_vec(),
    )
    .map_err(|error| error.to_string())?;

    let mut candidate = state.clone();
    match transaction.root {
        ActiveNetlistDocument::Generated => {
            candidate.ui.netlist.generated_document = Some(next.clone());
            candidate.ui.netlist.generated_source = next.source().to_owned();
        }
        ActiveNetlistDocument::OwnedSource => {
            if let Some(include) =
                candidate
                    .workspace
                    .netlist_descriptor
                    .as_mut()
                    .and_then(|descriptor| {
                        descriptor.owned_includes.iter_mut().find(|include| {
                            include.logical_identity == transaction.logical_identity
                        })
                    })
            {
                include.revision = include
                    .revision
                    .checked_add(1)
                    .ok_or_else(|| "Owned include revision overflowed.".to_owned())?;
                include.content_digest = crate::state::content_digest(&source);
            }
            candidate.workspace.netlist_document = Some(next.clone());
            candidate.workspace.netlist_source_dirty = true;
            candidate.ui.netlist.owned_document = Some(next);
            candidate
                .workspace
                .validate_simulation_configuration()
                .map_err(|error| error.to_string())?;
        }
        ActiveNetlistDocument::GeneratedDiff => {
            return Err("Revision comparisons cannot own a relink transaction.".to_owned());
        }
    }
    candidate.ui.netlist.dependency_relink = None;
    candidate.ui.netlist.active_document = transaction.root;
    candidate.ui.netlist.active_dependency_root = Some(transaction.root);
    candidate.ui.netlist.active_dependency_identity = Some(transaction.logical_identity.clone());
    candidate.simulation.netlist_content = source;
    candidate.ui.netlist.revision = candidate.ui.netlist.revision.wrapping_add(1);
    candidate.ui.netlist.completion_open = false;
    candidate.ui.netlist.edited_lines.clear();
    invalidate_source_evidence(&mut candidate.ui.netlist);
    *state = candidate;
    Ok(())
}

/// Select the immutable generated primary without deleting or changing an
/// owned source or generated-diff document. This is an explicit document
/// transition; navigating among Code workspace pages must not call it.
pub fn open_generated_primary(state: &mut AppState) -> bool {
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
        .map(crate::state::NetlistDocument::generated_artifact)
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

pub fn close_revision_comparison(state: &mut AppState) -> bool {
    state.ui.netlist.active_dependency_identity = None;
    state.ui.netlist.active_dependency_root = None;
    match (
        state.ui.netlist.comparison_return_document,
        state.workspace.netlist_source.clone(),
    ) {
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
    state.ui.netlist.generated_diff_source.clear();
    state.ui.netlist.completion_open = false;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    invalidate_source_evidence(&mut state.ui.netlist);
    true
}

fn short_digest(digest: crate::product::ContentDigest) -> String {
    digest.to_string().chars().take(12).collect()
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

/// Code-workspace document currently projected into the central editor.
/// Generated and owned source are independent retained documents; switching
/// between them never deletes either artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActiveNetlistDocument {
    #[default]
    Generated,
    OwnedSource,
    GeneratedDiff,
}

/// User-selected reach of the Code workspace find surface. Replacement is
/// intentionally limited to project-owned source documents; project-reference
/// search is always find-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NetlistFindScope {
    #[default]
    CurrentDocument,
    AllOwnedSources,
    ProjectReferences,
}

/// Persistent, keyboard-reachable state for the mockup's Find and replace
/// surface. Match rows are derived from the current exact source every frame,
/// so they cannot become stale after an edit.
#[derive(Debug, Clone, Default)]
pub struct NetlistFindState {
    pub open: bool,
    pub find: String,
    pub replacement: String,
    pub match_case: bool,
    pub whole_symbol: bool,
    pub regular_expression: bool,
    pub scope: NetlistFindScope,
    pub selected_match: usize,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NetlistOwnershipDialogState {
    pub open: bool,
    pub artifact_name: String,
    pub strategy: crate::state::OwnedNetlistEditStrategy,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct NetlistComparisonDialogState {
    pub open: bool,
    pub selected_history_index: usize,
}

#[derive(Debug, Clone)]
pub struct NetlistSaveDialogState {
    pub open: bool,
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
            message: "Update owned SPICE source".to_owned(),
            error: None,
        }
    }
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
    pub ownership_dialog: NetlistOwnershipDialogState,
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
    /// Numeric `.param` values captured from `last_run_buffer`.
    pub last_run_params: HashMap<String, f64>,
    /// Editor buffer captured when the current manual-deck run started.
    pub pending_run_buffer: Option<String>,
    /// Run id associated with `pending_run_buffer`.
    pub pending_manual_run_id: Option<u64>,
    /// Zero-based line containing the caret.
    pub cursor_line: usize,
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
            || self.ownership_dialog.open
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
mod tests {
    use super::*;
    use crate::product::ObjectRevision;
    use crate::state::{
        DependencyMetadata, GeneratedArtifact, GeneratedProvenance, GenerationInput,
        NetlistDocument, NetlistDocumentId, SourceLocator,
    };

    const ROOT: &str = "owned dependency fixture\n.include \"models/resistor.inc\"\nV1 out 0 1\nR1 out 0 rmodel\n.op\n.end\n";
    const ORIGINAL_INCLUDE: &str = ".param rmodel=1k\n";
    const EDITED_INCLUDE: &str = ".param rmodel=2.2k\n";
    const INCLUDE_IDENTITY: &str = "models/resistor.inc";

    fn owned_dependency_state() -> AppState {
        let locator = SourceLocator::try_new(INCLUDE_IDENTITY, "resistor.inc").unwrap();
        let dependency = DependencyMetadata::unresolved_direct_to(0, INCLUDE_IDENTITY, locator)
            .unwrap()
            .resolve_utf8(ORIGINAL_INCLUDE.as_bytes().to_vec())
            .unwrap();
        let provenance = GeneratedProvenance::try_new(
            "rspice-owned-include-test",
            GenerationInput::new(
                ObjectRevision::INITIAL,
                crate::state::content_digest("owned-include-input"),
            ),
        )
        .unwrap();
        let artifact = GeneratedArtifact::try_from_utf8(
            provenance,
            ROOT.as_bytes().to_vec(),
            vec![dependency],
            Vec::new(),
        )
        .unwrap();
        let generated =
            NetlistDocument::from_generated(NetlistDocumentId::new(), artifact).unwrap();
        let owned = generated
            .create_editable_copy(NetlistDocumentId::new(), generated.content_digest())
            .unwrap();

        let mut state = AppState::default();
        state.workspace.netlist_source = Some(ROOT.to_owned());
        state.workspace.netlist_document = Some(owned.clone());
        state.workspace.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
            artifact_name: "owned.cir".to_owned(),
            strategy: crate::state::OwnedNetlistEditStrategy::OwnedSource,
            source_encoding: crate::state::NetlistTextEncoding::Utf8,
            source_line_ending: crate::state::NetlistLineEnding::Lf,
            imported_dialect: Some(crate::state::NetlistSourceDialect::RSpice),
            compatibility_reviewed: true,
            execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
            external_file_sha256: None,
            save_history: Vec::new(),
            revision_history: Vec::new(),
            owned_includes: Vec::new(),
        });
        state.ui.netlist.generated_source = ROOT.to_owned();
        state.ui.netlist.generated_document = Some(generated);
        state.ui.netlist.owned_document = Some(owned);
        state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
        state.ui.netlist.active_document_initialized = true;
        state.simulation.netlist_content = ROOT.to_owned();
        state.workspace.validate_simulation_configuration().unwrap();
        state
    }

    /// The navigator asks for the outline on every frame. Parsing costs the
    /// deck, so the answer has to be the same object until the bytes change —
    /// and a different one the moment they do, because a navigator listing
    /// declarations the buffer no longer contains is worse than a slow one.
    #[test]
    fn the_visible_outline_is_parsed_once_per_change_and_never_reused_across_one() {
        let mut state = owned_dependency_state();

        let first = visible_source_index(&mut state);
        let again = visible_source_index(&mut state);
        assert!(
            std::sync::Arc::ptr_eq(&first, &again),
            "an unchanged buffer must not be parsed twice"
        );
        assert!(first.describes(ROOT));
        assert_eq!(first.card(3), "V1 out 0 1");

        assert!(replace_owned_source(
            &mut state,
            format!("{ROOT}* one more card\n")
        ));
        let edited = visible_source_index(&mut state);
        assert!(!std::sync::Arc::ptr_eq(&first, &edited));
        assert_eq!(edited.line_count(), first.line_count() + 1);

        // Switching documents changes the buffer without editing it.
        assert!(open_generated_primary(&mut state));
        assert!(visible_source_index(&mut state).describes(ROOT));
    }

    #[test]
    fn dependency_is_read_only_until_copy_then_edits_the_execution_closure() {
        let mut state = owned_dependency_state();
        open_netlist_dependency(&mut state, INCLUDE_IDENTITY).unwrap();
        assert_eq!(state.simulation.netlist_content, ORIGINAL_INCLUDE);
        assert!(!active_dependency_is_owned(&state));
        assert!(!replace_owned_dependency_source(
            &mut state,
            EDITED_INCLUDE.to_owned()
        ));

        let document_id = copy_active_dependency_to_project(&mut state).unwrap();
        assert_eq!(
            copy_active_dependency_to_project(&mut state).unwrap(),
            document_id,
            "copy-to-project is idempotent and preserves stable identity"
        );
        assert!(active_dependency_is_owned(&state));
        assert!(replace_owned_dependency_source(
            &mut state,
            EDITED_INCLUDE.to_owned()
        ));

        let document = state.workspace.netlist_document.as_ref().unwrap();
        assert_eq!(
            document.source(),
            ROOT,
            "include edits never rewrite the root"
        );
        assert_eq!(document.dependencies()[0].source(), Some(EDITED_INCLUDE));
        let descriptor = state.workspace.netlist_descriptor.as_ref().unwrap();
        let include = descriptor.owned_include(INCLUDE_IDENTITY).unwrap();
        assert_eq!(include.document_id, document_id);
        assert_eq!(include.revision, 2);
        assert_eq!(
            include.content_digest,
            crate::state::content_digest(EDITED_INCLUDE)
        );
        state.workspace.validate_simulation_configuration().unwrap();

        let expanded = crate::state::expand_retained_netlist_dependencies(
            document.id(),
            document.source(),
            document.dependencies(),
        )
        .unwrap();
        assert!(expanded.source.contains(".param rmodel=2.2k"));
        assert!(!expanded.source.contains(".param rmodel=1k"));
    }

    #[test]
    fn owned_dependency_identity_and_bytes_survive_project_round_trip() {
        let mut state = owned_dependency_state();
        open_netlist_dependency(&mut state, INCLUDE_IDENTITY).unwrap();
        let document_id = copy_active_dependency_to_project(&mut state).unwrap();
        assert!(replace_owned_dependency_source(
            &mut state,
            EDITED_INCLUDE.to_owned()
        ));

        let bytes = serde_json::to_vec(&state.workspace).unwrap();
        let restored: crate::state::ProjectWorkspace = serde_json::from_slice(&bytes).unwrap();
        restored.validate_simulation_configuration().unwrap();
        let include = restored
            .netlist_descriptor
            .as_ref()
            .unwrap()
            .owned_include(INCLUDE_IDENTITY)
            .unwrap();
        assert_eq!(include.document_id, document_id);
        assert_eq!(include.revision, 2);
        assert_eq!(
            restored.netlist_document.as_ref().unwrap().dependencies()[0].source(),
            Some(EDITED_INCLUDE)
        );
    }

    #[test]
    fn revision_restore_restores_include_bytes_and_ownership_as_one_snapshot() {
        let mut state = owned_dependency_state();
        let baseline_document = state.ui.netlist.owned_document.as_ref().unwrap().clone();
        state
            .workspace
            .netlist_descriptor
            .as_mut()
            .unwrap()
            .retain_revision(&baseline_document, "Before include ownership")
            .unwrap();
        open_netlist_dependency(&mut state, INCLUDE_IDENTITY).unwrap();
        copy_active_dependency_to_project(&mut state).unwrap();
        assert!(replace_owned_dependency_source(
            &mut state,
            EDITED_INCLUDE.to_owned()
        ));

        restore_owned_revision(&mut state, 0).unwrap();

        assert!(
            state
                .workspace
                .netlist_descriptor
                .as_ref()
                .unwrap()
                .owned_includes
                .is_empty()
        );
        assert_eq!(
            state
                .workspace
                .netlist_document
                .as_ref()
                .unwrap()
                .dependencies()[0]
                .source(),
            Some(ORIGINAL_INCLUDE)
        );
        assert!(state.ui.netlist.active_dependency_identity.is_none());
        state.workspace.validate_simulation_configuration().unwrap();
    }

    #[test]
    fn dependency_transitions_fail_closed_for_stale_or_generated_ownership() {
        let mut state = owned_dependency_state();
        let before = state.simulation.netlist_content.clone();
        assert!(open_netlist_dependency(&mut state, "missing.inc").is_err());
        assert_eq!(state.simulation.netlist_content, before);
        assert!(state.ui.netlist.active_dependency_identity.is_none());

        open_generated_primary(&mut state);
        open_netlist_dependency(&mut state, INCLUDE_IDENTITY).unwrap();
        assert!(!active_dependency_is_owned(&state));
        assert!(copy_active_dependency_to_project(&mut state).is_err());
        assert!(!replace_owned_dependency_source(
            &mut state,
            EDITED_INCLUDE.to_owned()
        ));
    }

    #[test]
    fn project_validation_rejects_owned_include_digest_drift() {
        let mut state = owned_dependency_state();
        open_netlist_dependency(&mut state, INCLUDE_IDENTITY).unwrap();
        copy_active_dependency_to_project(&mut state).unwrap();
        state
            .workspace
            .netlist_descriptor
            .as_mut()
            .unwrap()
            .owned_includes[0]
            .content_digest = crate::product::ContentDigest::from_bytes([0x7f; 32]);
        assert!(state.workspace.validate_simulation_configuration().is_err());
    }

    #[test]
    fn relink_reacquires_origin_and_replaces_exact_retained_bytes() {
        let mut state = owned_dependency_state();
        begin_dependency_relink(&mut state, INCLUDE_IDENTITY).unwrap();
        commit_dependency_relink(
            &mut state,
            EDITED_INCLUDE.to_owned(),
            "resistor-qualified.lib".to_owned(),
            Some("C:/qualified/models/resistor-qualified.lib".to_owned()),
        )
        .unwrap();

        let dependency = active_dependency(&state).unwrap();
        assert_eq!(dependency.source(), Some(EDITED_INCLUDE));
        assert_eq!(
            dependency.locator().display_name(),
            "resistor-qualified.lib"
        );
        assert_eq!(
            dependency.locator().native_origin(),
            Some("C:/qualified/models/resistor-qualified.lib")
        );
        assert_eq!(
            dependency.locator().logical_identity(),
            INCLUDE_IDENTITY,
            "relink must preserve the canonical include edge"
        );
        assert!(state.workspace.netlist_source_dirty);
        state.workspace.validate_simulation_configuration().unwrap();
    }

    #[test]
    fn relink_rejects_late_picker_completion_after_document_revision_changes() {
        let mut state = owned_dependency_state();
        begin_dependency_relink(&mut state, INCLUDE_IDENTITY).unwrap();
        let document = state.ui.netlist.owned_document.as_mut().unwrap();
        document
            .replace_editable_source(
                document.content_digest(),
                ROOT.replace("V1 out 0 1", "V1 out 0 2").into_bytes(),
            )
            .unwrap();

        let error = commit_dependency_relink(
            &mut state,
            EDITED_INCLUDE.to_owned(),
            "late.lib".to_owned(),
            None,
        )
        .unwrap_err();

        assert!(error.contains("changed while the picker was open"));
        assert_eq!(
            state
                .workspace
                .netlist_document
                .as_ref()
                .unwrap()
                .dependencies()[0]
                .source(),
            Some(ORIGINAL_INCLUDE)
        );
    }

    #[test]
    fn generated_dependency_relink_updates_only_the_generated_artifact() {
        let mut state = owned_dependency_state();
        open_generated_primary(&mut state);
        begin_dependency_relink(&mut state, INCLUDE_IDENTITY).unwrap();
        commit_dependency_relink(
            &mut state,
            EDITED_INCLUDE.to_owned(),
            "browser-picked.inc".to_owned(),
            None,
        )
        .unwrap();

        assert_eq!(
            active_dependency(&state).unwrap().source(),
            Some(EDITED_INCLUDE)
        );
        assert_eq!(
            state
                .workspace
                .netlist_document
                .as_ref()
                .unwrap()
                .dependencies()[0]
                .source(),
            Some(ORIGINAL_INCLUDE),
            "relinking generated review authority must not mutate the owned root"
        );
    }
}
