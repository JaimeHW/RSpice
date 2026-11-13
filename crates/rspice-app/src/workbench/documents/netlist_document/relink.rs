//! The dependency relink transaction: rebinding one retained include to a
//! newly picked source without letting a stale picker commit over a document
//! that moved on.

use super::*;
use crate::workbench::state::WorkspaceDocumentId;

/// Revision-bound authority for one native or browser dependency relink
/// picker. It is transient UI state; exact relinked bytes and locator evidence
/// are persisted only after the guarded transaction commits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistDependencyRelinkState {
    pub root: ActiveNetlistDocument,
    pub logical_identity: String,
    pub expected_document_id: crate::state::NetlistDocumentId,
    pub expected_revision: crate::product::ObjectRevision,
}

pub(crate) fn begin_dependency_relink(
    model: &Model,
    netlist: &mut NetlistDocumentState,
    logical_identity: &str,
) -> Result<(), String> {
    if netlist.dependency_relink.is_some() {
        return Err("A dependency relink is already in progress.".to_owned());
    }
    let root = netlist
        .active_dependency_root
        .unwrap_or(netlist.active_document);
    let document = canonical_root_document_in(model, netlist, root)
        .ok_or_else(|| "The selected source has no dependency list.".to_owned())?;
    if !document
        .dependencies()
        .iter()
        .any(|dependency| dependency.locator().logical_identity() == logical_identity)
    {
        return Err("The dependency is no longer among the source's dependencies.".to_owned());
    }
    netlist.dependency_relink = Some(NetlistDependencyRelinkState {
        root,
        logical_identity: logical_identity.to_owned(),
        expected_document_id: document.id(),
        expected_revision: document.revision(),
    });
    Ok(())
}

pub(crate) fn cancel_dependency_relink(netlist: &mut NetlistDocumentState) {
    netlist.dependency_relink = None;
}

pub(crate) fn commit_dependency_relink(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    source: String,
    display_name: String,
    native_origin: Option<String>,
) -> Result<(), String> {
    let transaction = netlist
        .dependency_relink
        .clone()
        .ok_or_else(|| "No dependency relink transaction is active.".to_owned())?;
    let current = canonical_root_document_in(model, netlist, transaction.root)
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
        .ok_or_else(|| "The dependency is no longer among the source's dependencies.".to_owned())?;
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

    let mut candidate = DeckTransaction::begin(model, netlist, open_documents);
    match transaction.root {
        ActiveNetlistDocument::Generated => {
            candidate.model.netlist_artifacts.generated_document = Some(next.clone());
            candidate.model.netlist_artifacts.generated_source = next.source().to_owned();
        }
        ActiveNetlistDocument::OwnedSource => {
            if let Some(include) = candidate
                .model
                .workspace
                .content
                .netlist_descriptor
                .as_mut()
                .and_then(|descriptor| {
                    descriptor
                        .owned_includes
                        .iter_mut()
                        .find(|include| include.logical_identity == transaction.logical_identity)
                })
            {
                include.revision = include
                    .revision
                    .checked_add(1)
                    .ok_or_else(|| "Owned include revision overflowed.".to_owned())?;
                include.content_digest = crate::state::content_digest(&source);
            }
            candidate.model.workspace.content.netlist_document = Some(next.clone());
            candidate.model.workspace.content.netlist_source_dirty = true;
            candidate.netlist.owned_document = Some(next);
            candidate
                .model
                .workspace
                .content
                .validate_simulation_configuration()
                .map_err(|error| error.to_string())?;
        }
        ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => {
            return Err("Read-only revision documents cannot own a relink transaction.".to_owned());
        }
    }
    candidate.netlist.dependency_relink = None;
    candidate.netlist.active_document = transaction.root;
    candidate.netlist.active_dependency_root = Some(transaction.root);
    candidate.netlist.active_dependency_identity = Some(transaction.logical_identity.clone());
    candidate.model.simulation.source.netlist_content = source;
    candidate.netlist.revision = candidate.netlist.revision.wrapping_add(1);
    candidate.netlist.completion_open = false;
    candidate.model.invalidate_source_evidence();
    candidate.commit_deck(model, netlist, open_documents);
    runs.manual_run.edited_lines.clear();
    Ok(())
}
