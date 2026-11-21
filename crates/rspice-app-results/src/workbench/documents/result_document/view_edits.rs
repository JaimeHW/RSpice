//! The one writer of Results view state.
//!
//! Every change the Results workspace makes to a view, whether a reader's
//! edit, a captured viewport or the view following a new run, is one
//! transaction applied here. Nothing else in the workspace transacts a
//! visualization document, so what a view holds, and what undo can step
//! back, has exactly one way in.
//!
//! Each view keeps its own undo and redo. A step is a transaction that holds
//! at least one edit the reader made; moving the viewport or a cursor is
//! navigation, which zoom's own back and forward covers, and following a new
//! run is not the reader's edit, so neither is a step.

use std::collections::HashMap;

use crate::product::{ObjectRevision, ResultDocumentId};
use crate::results::visualization_document::{
    DocumentEdit, ViewSnapshot, VisualizationDocument, VisualizationTransactionReceipt,
};
use crate::state::workspace::VisualizationDocumentPersistenceError;

use super::{ResultsParts, ResultsReads};

/// The undo and redo steps of every view the session has edited.
#[derive(Debug, Clone, Default)]
pub(crate) struct ViewHistories {
    views: HashMap<ResultDocumentId, ViewHistory>,
}

#[derive(Debug, Clone, Default)]
struct ViewHistory {
    undo: Vec<ViewSnapshot>,
    redo: Vec<ViewSnapshot>,
}

/// Which way a history step goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Step {
    Undo,
    Redo,
}

/// Whether an edit is one the reader made, as opposed to navigation or the
/// view following a run.
fn is_reader_edit(edit: &DocumentEdit) -> bool {
    !matches!(
        edit,
        DocumentEdit::SetAxisRange { .. }
            | DocumentEdit::MoveCursor { .. }
            | DocumentEdit::AddCursor { .. }
            | DocumentEdit::RetargetTrackedDataset { .. }
    )
}

/// Apply `edits` to the view `document_id` as one transaction at `revision`.
pub(crate) fn apply(
    state: &mut ResultsParts<'_>,
    document_id: ResultDocumentId,
    revision: ObjectRevision,
    edits: Vec<DocumentEdit>,
) -> Result<VisualizationTransactionReceipt, VisualizationDocumentPersistenceError> {
    let before = edits
        .iter()
        .any(is_reader_edit)
        .then(|| {
            state
                .model
                .workspace
                .content
                .visualization_document(document_id)
                .map(VisualizationDocument::view_snapshot)
        })
        .flatten();
    let receipt = state
        .model
        .workspace
        .content
        .transact_visualization_document(document_id, revision, edits)?;
    if let Some(before) = before {
        let history = state
            .results
            .view_edits
            .views
            .entry(document_id)
            .or_default();
        history.undo.push(before);
        history.redo.clear();
    }
    Ok(receipt)
}

/// Whether the view `document_id` has a step to take in `direction`.
pub(crate) fn can_step(
    state: ResultsReads<'_>,
    document_id: ResultDocumentId,
    direction: Step,
) -> bool {
    state
        .results
        .view_edits
        .views
        .get(&document_id)
        .is_some_and(|history| match direction {
            Step::Undo => !history.undo.is_empty(),
            Step::Redo => !history.redo.is_empty(),
        })
}

/// Take one undo or redo step of the view `document_id`. A step the view can
/// no longer take, because it names a run the view no longer reads, is
/// dropped and the reason returned.
pub(crate) fn step(
    state: &mut ResultsParts<'_>,
    document_id: ResultDocumentId,
    direction: Step,
) -> Result<(), String> {
    let Some(current) = state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .map(|document| (document.revision(), document.view_snapshot()))
    else {
        state.results.view_edits.views.remove(&document_id);
        return Err("The result view is no longer in the project.".to_owned());
    };
    let Some(history) = state.results.view_edits.views.get_mut(&document_id) else {
        return Ok(());
    };
    let popped = match direction {
        Step::Undo => history.undo.pop(),
        Step::Redo => history.redo.pop(),
    };
    let Some(target) = popped else {
        return Ok(());
    };
    let (revision, snapshot) = current;
    if let Err(error) = state
        .model
        .workspace
        .content
        .restore_visualization_document_view(document_id, revision, &target)
    {
        return Err(match direction {
            Step::Undo => format!("This step can no longer be undone: {error}"),
            Step::Redo => format!("This step can no longer be redone: {error}"),
        });
    }
    let history = state
        .results
        .view_edits
        .views
        .entry(document_id)
        .or_default();
    match direction {
        Step::Undo => history.redo.push(snapshot),
        Step::Redo => history.undo.push(snapshot),
    }
    Ok(())
}

/// The view Ctrl+Z and Ctrl+Y step in Results: the one the workspace shows.
pub(crate) fn shown_view(state: ResultsReads<'_>) -> Option<ResultDocumentId> {
    super::document_edit::active_document_id(state)
}

/// Whether Undo or Redo in Results would take a step.
pub(crate) fn shown_view_can_step(state: ResultsReads<'_>, direction: Step) -> bool {
    shown_view(state).is_some_and(|document_id| can_step(state, document_id, direction))
}

/// Take one undo or redo step of the view Results shows, saying on the
/// status bar why when it cannot.
pub(crate) fn step_shown_view(state: &mut ResultsParts<'_>, direction: Step) {
    let Some(document_id) = shown_view(state.results_reads()) else {
        return;
    };
    if let Err(reason) = step(state, document_id, direction) {
        state.prompt(reason);
    }
}
