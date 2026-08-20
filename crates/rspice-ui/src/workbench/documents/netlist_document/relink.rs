//! The dependency relink transaction: rebinding one retained include to a
//! newly picked source without letting a stale picker commit over a document
//! that moved on.

use super::*;

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
        ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => {
            return Err("Read-only revision documents cannot own a relink transaction.".to_owned());
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
