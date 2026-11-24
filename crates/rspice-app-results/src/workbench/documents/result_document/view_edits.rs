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
//! run is not the reader's edit, so neither is a step. A step holds only
//! what its edit changed, so undo grows with the edits made, not with the
//! size of the view times the steps.
//!
//! Renaming, duplicating and deleting a saved document are steps too, kept
//! beside the views' own. Undo and Redo take whichever is latest: the shown
//! view's step or the documents' step.

use std::collections::HashMap;

use crate::product::{ObjectRevision, ResultDocumentId};
use crate::results::visualization_document::{
    DocumentEdit, ViewStep, VisualizationDocument, VisualizationTransactionReceipt,
};
use crate::state::workspace::VisualizationDocumentPersistenceError;
use crate::workbench::state::{Workspace, WorkspaceDocumentId};

use super::{ResultsParts, ResultsReads};

/// The undo and redo steps of every view the session has edited, and of
/// the saved documents renamed, duplicated or deleted.
#[derive(Debug, Clone, Default)]
pub(crate) struct ViewHistories {
    views: HashMap<ResultDocumentId, ViewHistory>,
    documents: History<DocumentStep>,
    /// Counts the steps recorded and taken, so Undo and Redo take the
    /// latest of the shown view's steps and the documents' steps.
    clock: u64,
}

impl ViewHistories {
    /// The stamp of a step recorded or taken now, later than every other.
    fn tick(&mut self) -> u64 {
        self.clock += 1;
        self.clock
    }
}

/// Undo and redo stacks, each step stamped with when it was recorded or
/// last taken.
#[derive(Debug, Clone)]
struct History<T> {
    undo: Vec<(u64, T)>,
    redo: Vec<(u64, T)>,
}

impl<T> Default for History<T> {
    fn default() -> Self {
        Self {
            undo: Vec::new(),
            redo: Vec::new(),
        }
    }
}

impl<T> History<T> {
    fn stack_mut(&mut self, direction: Step) -> &mut Vec<(u64, T)> {
        match direction {
            Step::Undo => &mut self.undo,
            Step::Redo => &mut self.redo,
        }
    }

    /// When the step `direction` takes next was recorded or last taken.
    fn latest(&self, direction: Step) -> Option<u64> {
        let stack = match direction {
            Step::Undo => &self.undo,
            Step::Redo => &self.redo,
        };
        stack.last().map(|(stamp, _)| *stamp)
    }
}

type ViewHistory = History<ViewStep>;

/// What taking a document step does: each step taken leaves the step that
/// reverses it on the other stack.
#[derive(Debug, Clone)]
enum DocumentStep {
    /// Give the document `document` the title `to`.
    Rename {
        document: ResultDocumentId,
        to: String,
    },
    /// Take the document `document` out of the project, then show `show`:
    /// the document a duplicate was made of.
    Remove {
        document: ResultDocumentId,
        show: Option<ResultDocumentId>,
    },
    /// Put `document` back at place `at` with its view's own steps; taking
    /// it out again shows `then`.
    Insert {
        document: Box<VisualizationDocument>,
        at: usize,
        history: Option<ViewHistory>,
        then: Option<ResultDocumentId>,
    },
}

impl Step {
    const fn other(self) -> Self {
        match self {
            Self::Undo => Self::Redo,
            Self::Redo => Self::Undo,
        }
    }
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
            | DocumentEdit::FollowRun { .. }
            | DocumentEdit::AddViewPane(_)
            | DocumentEdit::SetTracking(_)
    )
}

/// Whether an edit leaves what following a run reads as it stood: all but
/// those that change the view's tracking or the runs its panes read.
fn follows_alike(edit: &DocumentEdit) -> bool {
    !matches!(
        edit,
        DocumentEdit::RetargetTrackedDataset { .. }
            | DocumentEdit::FollowRun { .. }
            | DocumentEdit::AddViewPane(_)
            | DocumentEdit::SetTracking(_)
    )
}

/// Keep `document`, a new view, in the project: a run's working view, or a
/// document saved from one.
pub(crate) fn create(
    state: &mut ResultsParts<'_>,
    document: VisualizationDocument,
) -> Result<ResultDocumentId, VisualizationDocumentPersistenceError> {
    state
        .model
        .workspace
        .content
        .insert_visualization_document(document)
}

/// Forget the view `document_id` and its steps: a run's working view once
/// nothing can draw it again.
pub(crate) fn forget(state: &mut ResultsParts<'_>, document_id: ResultDocumentId) {
    state.results.working_view_plotting.forget(document_id);
    state
        .model
        .workspace
        .content
        .remove_visualization_document(document_id);
    state.results.view_edits.views.remove(&document_id);
}

/// Apply `edits` to the view `document_id` as one transaction at `revision`.
pub(crate) fn apply(
    state: &mut ResultsParts<'_>,
    document_id: ResultDocumentId,
    revision: ObjectRevision,
    edits: Vec<DocumentEdit>,
) -> Result<VisualizationTransactionReceipt, VisualizationDocumentPersistenceError> {
    settle(state, document_id);
    let followed = edits.iter().all(follows_alike)
        && super::working_view::following_holds(state.results_reads(), document_id);
    if !edits.iter().any(is_reader_edit) {
        let receipt = state
            .model
            .workspace
            .content
            .transact_visualization_document(document_id, revision, edits)?;
        super::working_view::still_followed(state, followed, receipt.committed_revision);
        return Ok(receipt);
    }
    let (receipt, undo) = state
        .model
        .workspace
        .content
        .transact_visualization_document_with_undo(document_id, revision, edits)?;
    super::working_view::still_followed(state, followed, receipt.committed_revision);
    let stamp = state.results.view_edits.tick();
    let history = state
        .results
        .view_edits
        .views
        .entry(document_id)
        .or_default();
    history.undo.push((stamp, undo));
    history.redo.clear();
    Ok(receipt)
}

/// Apply `edits` the view makes of itself, following its run (taking in
/// the signals a run plots, say): no step of the reader's to undo.
pub(crate) fn apply_following(
    state: &mut ResultsParts<'_>,
    document_id: ResultDocumentId,
    revision: ObjectRevision,
    edits: Vec<DocumentEdit>,
) -> Result<VisualizationTransactionReceipt, VisualizationDocumentPersistenceError> {
    settle(state, document_id);
    state
        .model
        .workspace
        .content
        .transact_visualization_document(document_id, revision, edits)
}

/// Have `document_id` take in the traces being made for it first, so a
/// change of the view comes after the reader's change they belong to.
fn settle(state: &mut ResultsParts<'_>, document_id: ResultDocumentId) {
    if state.results.working_view_plotting.view() == Some(document_id) {
        super::working_view::settle(state);
    }
}

/// Take one undo or redo step of the view `document_id`. A step addresses
/// the view, not the run it read: one taken before the view followed a
/// newer run is put back on the run shown now. A step the view can no
/// longer take is dropped and the reason returned.
pub(crate) fn step(
    state: &mut ResultsParts<'_>,
    document_id: ResultDocumentId,
    direction: Step,
) -> Result<(), String> {
    settle(state, document_id);
    let Some(revision) = state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .map(VisualizationDocument::revision)
    else {
        state.results.view_edits.views.remove(&document_id);
        return Err("The result view is no longer in the project.".to_owned());
    };
    let Some(history) = state.results.view_edits.views.get_mut(&document_id) else {
        return Ok(());
    };
    let Some((_, target)) = history.stack_mut(direction).pop() else {
        return Ok(());
    };
    // A step takes back or puts back the reader's own edits, which change
    // nothing following the run reads.
    let followed = super::working_view::following_holds(state.results_reads(), document_id);
    let reverse = match state
        .model
        .workspace
        .content
        .take_visualization_document_step(document_id, revision, target)
    {
        Ok((receipt, reverse)) => {
            super::working_view::still_followed(state, followed, receipt.committed_revision);
            reverse
        }
        Err(error) => {
            return Err(match direction {
                Step::Undo => format!("This step can no longer be undone: {error}"),
                Step::Redo => format!("This step can no longer be redone: {error}"),
            });
        }
    };
    let stamp = state.results.view_edits.tick();
    state
        .results
        .view_edits
        .views
        .entry(document_id)
        .or_default()
        .stack_mut(direction.other())
        .push((stamp, reverse));
    Ok(())
}

/// The view Ctrl+Z and Ctrl+Y step in Results: the one the workspace shows,
/// a saved document or the working view of the run it shows.
pub(crate) fn shown_view(state: ResultsReads<'_>) -> Option<ResultDocumentId> {
    match state.workbench.documents.active(Workspace::Results) {
        Some(WorkspaceDocumentId::ResultDataset(dataset_id)) => {
            super::working_view::view_of_run(state, *dataset_id)
        }
        _ => super::document_edit::active_document_id(state),
    }
}

/// Whose step Undo or Redo takes next in Results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NextStep {
    /// The shown view's own.
    View(ResultDocumentId),
    /// A saved document's rename, duplicate or delete.
    Document,
}

/// The step Undo or Redo takes next in Results: the latest of the shown
/// view's and the documents'.
fn next_step(state: ResultsReads<'_>, direction: Step) -> Option<NextStep> {
    let histories = &state.results.view_edits;
    let shown = shown_view(state);
    // A change whose traces are still being made is the latest step, and
    // leaves the view nothing to redo.
    let making = shown.filter(|view| state.results.working_view_plotting.view() == Some(*view));
    if let (Some(view), Step::Undo) = (making, direction) {
        return Some(NextStep::View(view));
    }
    let view = shown.filter(|_| making.is_none()).and_then(|document_id| {
        histories
            .views
            .get(&document_id)
            .and_then(|history| history.latest(direction))
            .map(|stamp| (stamp, NextStep::View(document_id)))
    });
    let document = histories
        .documents
        .latest(direction)
        .map(|stamp| (stamp, NextStep::Document));
    view.into_iter()
        .chain(document)
        .max_by_key(|(stamp, _)| *stamp)
        .map(|(_, next)| next)
}

/// Whether Undo or Redo in Results would take a step.
pub(crate) fn shown_view_can_step(state: ResultsReads<'_>, direction: Step) -> bool {
    next_step(state, direction).is_some()
}

/// Take one undo or redo step in Results, the shown view's or a saved
/// document's, saying on the status bar why when it cannot.
pub(crate) fn step_shown_view(state: &mut ResultsParts<'_>, direction: Step) {
    let outcome = match next_step(state.results_reads(), direction) {
        Some(NextStep::View(document_id)) => step(state, document_id, direction),
        Some(NextStep::Document) => step_document(state, direction),
        None => return,
    };
    if let Err(reason) = outcome {
        state.prompt(reason);
    }
}

/// Note a document step the reader took, as one Undo takes back.
fn record(state: &mut ResultsParts<'_>, reverse: DocumentStep) {
    let stamp = state.results.view_edits.tick();
    let documents = &mut state.results.view_edits.documents;
    documents.undo.push((stamp, reverse));
    documents.redo.clear();
}

/// Give the saved document `document_id` the title `title`, a step Undo
/// takes back.
pub(crate) fn rename_document(
    state: &mut ResultsParts<'_>,
    document_id: ResultDocumentId,
    title: &str,
) -> Result<(), String> {
    let from = state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .ok_or_else(|| GONE.to_owned())?
        .title()
        .to_owned();
    state
        .model
        .workspace
        .content
        .rename_visualization_document(document_id, title)
        .map_err(|error| error.to_string())?;
    record(
        state,
        DocumentStep::Rename {
            document: document_id,
            to: from,
        },
    );
    Ok(())
}

/// Keep `copy`, a duplicate of the saved document `of`, in the project, a
/// step Undo takes back.
pub(crate) fn add_duplicate(
    state: &mut ResultsParts<'_>,
    copy: VisualizationDocument,
    of: ResultDocumentId,
) -> Result<ResultDocumentId, String> {
    let document = create(state, copy).map_err(|error| error.to_string())?;
    record(
        state,
        DocumentStep::Remove {
            document,
            show: Some(of),
        },
    );
    Ok(document)
}

/// Delete the saved document `document_id` and its view's steps, a step
/// Undo takes back: the document returns where it stood, with its steps.
pub(crate) fn delete_document(
    state: &mut ResultsParts<'_>,
    document_id: ResultDocumentId,
) -> Result<(), String> {
    let reverse = take_out(state, document_id, None)?;
    record(state, reverse);
    Ok(())
}

const GONE: &str = "The document is no longer in the project.";

/// Take the document `document_id` out of the project with its view's
/// steps, and return the step that puts both back; taking it out again
/// then shows `then`.
fn take_out(
    state: &mut ResultsParts<'_>,
    document_id: ResultDocumentId,
    then: Option<ResultDocumentId>,
) -> Result<DocumentStep, String> {
    let content = &mut state.model.workspace.content;
    let at = content
        .visualization_documents
        .iter()
        .position(|document| document.id() == document_id)
        .ok_or_else(|| GONE.to_owned())?;
    let document = Box::new(content.visualization_documents[at].clone());
    content.remove_visualization_document(document_id);
    let history = state.results.view_edits.views.remove(&document_id);
    Ok(DocumentStep::Insert {
        document,
        at,
        history,
        then,
    })
}

/// Take the documents' next step in `direction`, leave the step that
/// reverses it on the other stack, and show the document it leaves in
/// front. A step that can no longer be taken is dropped and the reason
/// returned.
fn step_document(state: &mut ResultsParts<'_>, direction: Step) -> Result<(), String> {
    let Some((_, next)) = state
        .results
        .view_edits
        .documents
        .stack_mut(direction)
        .pop()
    else {
        return Ok(());
    };
    let (reverse, show) = take_document_step(state, next).map_err(|error| match direction {
        Step::Undo => format!("This step can no longer be undone: {error}"),
        Step::Redo => format!("This step can no longer be redone: {error}"),
    })?;
    let stamp = state.results.view_edits.tick();
    state
        .results
        .view_edits
        .documents
        .stack_mut(direction.other())
        .push((stamp, reverse));
    if let Some(document_id) = show {
        crate::workbench::chrome::document_bar::results::show_visualization_document(
            state,
            &document_id,
        );
    }
    Ok(())
}

/// Do what `step` says, and return the step that reverses it with the
/// document to show after it.
fn take_document_step(
    state: &mut ResultsParts<'_>,
    step: DocumentStep,
) -> Result<(DocumentStep, Option<ResultDocumentId>), String> {
    match step {
        DocumentStep::Rename { document, to } => {
            let content = &mut state.model.workspace.content;
            let from = content
                .visualization_document(document)
                .ok_or_else(|| GONE.to_owned())?
                .title()
                .to_owned();
            content
                .rename_visualization_document(document, &to)
                .map_err(|error| error.to_string())?;
            Ok((DocumentStep::Rename { document, to: from }, Some(document)))
        }
        DocumentStep::Remove { document, show } => Ok((take_out(state, document, show)?, show)),
        DocumentStep::Insert {
            document,
            at,
            history,
            then,
        } => {
            let document_id = state
                .model
                .workspace
                .content
                .insert_visualization_document_at(*document, at)
                .map_err(|error| error.to_string())?;
            if let Some(history) = history {
                state.results.view_edits.views.insert(document_id, history);
            }
            Ok((
                DocumentStep::Remove {
                    document: document_id,
                    show: then,
                },
                Some(document_id),
            ))
        }
    }
}
