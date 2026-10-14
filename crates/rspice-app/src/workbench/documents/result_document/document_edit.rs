//! Edits the result viewer makes to the project result document it shows.
//!
//! The pane header's context menu and the docbar's pickers lead here, as do
//! the small forms of [`rspice_results_ui::document_dialogs`]. Every edit is
//! one transaction against the document at its current revision. A form keeps
//! only its draft: what it offers is resolved from the retained datasets on
//! the frame it draws, and its commit resolves them again, so a run pruned
//! under an open form refuses instead of binding.

use egui::Context;
use rspice_results_ui::chrome::persistent::PaneMenu;
use rspice_results_ui::document_dialogs::{
    ResultDocumentDialog,
    add_pane::{self, AddPaneChoices, AddPaneDraft},
};
use rspice_ui_kit::widgets::DialogChoice;

use crate::diagnostics::ConsoleMessage;
use crate::product::{DatasetBinding, ResultDocumentId};
use crate::results::viewer_catalog::{VIEWER_DOCUMENTS, ViewerReleaseClass, viewer_document};
use crate::results::visualization_document::{
    DocumentEdit, EntityRef, NewPane, PageLayout, PaneDataBinding, PaneId, PanePlacement,
    VisualizationDocument, VisualizationTransactionReceipt,
};
use crate::state::AnalysisResult;
use crate::workbench::state::{Workspace, WorkspaceDocumentId};

use super::{AppState, RSpiceApp, ResultViewer};

/// The project result document the Results workspace is showing, if any.
pub(super) fn active_document_id(state: &AppState) -> Option<ResultDocumentId> {
    match state.workbench.documents.active(Workspace::Results) {
        Some(WorkspaceDocumentId::VisualizationDocument(document_id)) => Some(*document_id),
        _ => None,
    }
}

fn project_document(
    state: &AppState,
    document_id: ResultDocumentId,
) -> Result<&VisualizationDocument, String> {
    state
        .workspace
        .content
        .visualization_document(document_id)
        .ok_or_else(|| "The result document is no longer in the project.".to_owned())
}

/// Commit `edits` against the document's current revision.
fn transact(
    state: &mut AppState,
    document_id: ResultDocumentId,
    edits: Vec<DocumentEdit>,
) -> Result<VisualizationTransactionReceipt, String> {
    let revision = project_document(state, document_id)?.revision();
    state
        .workspace
        .content
        .transact_visualization_document(document_id, revision, edits)
        .map_err(|error| error.to_string())
}

fn report(state: &mut AppState, result: Result<(), String>) {
    if let Err(error) = result {
        state.push_user_message(ConsoleMessage::error(error));
    }
}

/// Draw the open form, if any, and commit what it asks for.
pub(crate) fn show(ctx: &Context, app: &mut RSpiceApp) {
    let Some(dialog) = app.state.workbench.result_document_dialog.clone() else {
        return;
    };
    let (dialog, choice) = match dialog {
        ResultDocumentDialog::AddPane(mut draft) => {
            let choices = add_pane_choices(&app.state, &mut draft);
            let choice = add_pane::show(ctx, &mut draft, &choices);
            (ResultDocumentDialog::AddPane(draft), choice)
        }
        ResultDocumentDialog::Compare(mut draft) => {
            let choices = compare::choices(&app.state, &mut draft);
            let choice =
                rspice_results_ui::document_dialogs::compare::show(ctx, &mut draft, &choices);
            (ResultDocumentDialog::Compare(draft), choice)
        }
    };
    app.state.workbench.result_document_dialog = Some(dialog);
    match choice {
        DialogChoice::Primary => commit_open_dialog(&mut app.state),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state.workbench.result_document_dialog = None;
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

/// Commit the open form's draft. The form closes when its edit lands, or
/// moves on to what it shows next.
fn commit_open_dialog(state: &mut AppState) {
    let result = match state.workbench.result_document_dialog.clone() {
        Some(ResultDocumentDialog::AddPane(draft)) => add_pane(state, &draft).map(|()| None),
        Some(ResultDocumentDialog::Compare(draft)) => compare::commit(state, draft),
        None => return,
    };
    match result {
        Ok(next) => state.workbench.result_document_dialog = next,
        Err(error) => state.push_user_message(ConsoleMessage::error(error)),
    }
}

// ---------------------------------------------------------------------------
// Add and remove panes
// ---------------------------------------------------------------------------

/// Whether a pane can be added: a project result document is open and a
/// retained dataset exists to bind the pane to.
pub(crate) fn add_pane_available(state: &AppState) -> bool {
    active_document_id(state).is_some() && state.simulation.retained.has_results()
}

/// Open the Add pane form, seeded with the dataset, analysis and sheet the
/// selected pane shows.
pub(crate) fn open_add_pane(state: &mut AppState) {
    if !add_pane_available(state) {
        return;
    }
    let draft = AddPaneDraft {
        dataset: state.simulation.active_run().map(|run| run.dataset_id),
        analysis: state
            .simulation
            .active_analysis()
            .map(|analysis| analysis.id),
        viewer: Some(state.ui.results.session.viewer),
    };
    state.workbench.result_document_dialog = Some(ResultDocumentDialog::AddPane(draft));
}

/// The sheets a pane bound to `analysis` can draw: release viewers whose data
/// contract the analysis meets and whose native renderer presents it.
fn pane_viewers(analysis: &AnalysisResult) -> Vec<(ResultViewer, String)> {
    let mut viewers: Vec<(ResultViewer, String)> = Vec::new();
    for definition in VIEWER_DOCUMENTS.iter() {
        let Some(viewer) = ResultViewer::from_viewer_document_id(definition.id) else {
            continue;
        };
        if definition.release == ViewerReleaseClass::ReleaseTarget
            && super::create_document::analysis_matches_viewer(definition, analysis.analysis_type)
            && super::persistent_document::renderer_supports_analysis(definition.id, analysis)
            && !viewers.iter().any(|(candidate, _)| *candidate == viewer)
        {
            viewers.push((viewer, definition.title.to_owned()));
        }
    }
    viewers
}

/// Resolve the form's choices against the retained datasets, moving any
/// selection that no longer exists onto the first one that does.
fn add_pane_choices(state: &AppState, draft: &mut AddPaneDraft) -> AddPaneChoices {
    let runs = &state.simulation.retained.runs;
    let datasets = runs
        .iter()
        .map(|run| (run.dataset_id, format!("Run {} · {}", run.id, run.label)))
        .collect();
    let run = draft
        .dataset
        .and_then(|dataset| runs.iter().find(|run| run.dataset_id == dataset))
        .or_else(|| runs.iter().next());
    draft.dataset = run.map(|run| run.dataset_id);
    let analyses = run.map_or_else(Vec::new, |run| {
        run.analyses
            .iter()
            .map(|analysis| (analysis.id, analysis.label.clone()))
            .collect()
    });
    let analysis = run.and_then(|run| {
        draft
            .analysis
            .and_then(|id| run.analyses.iter().find(|analysis| analysis.id == id))
            .or_else(|| run.analyses.first())
    });
    draft.analysis = analysis.map(|analysis| analysis.id);
    let viewers = analysis.map_or_else(Vec::new, pane_viewers);
    if !draft
        .viewer
        .is_some_and(|viewer| viewers.iter().any(|(candidate, _)| *candidate == viewer))
    {
        draft.viewer = viewers.first().map(|(viewer, _)| *viewer);
    }
    let position = active_document_id(state).and_then(|document_id| {
        let (_, pane_id) = super::persistent_document::reading_position(state, document_id)?;
        let document = state
            .workspace
            .content
            .visualization_document(document_id)?;
        let pane = document.panes().iter().find(|pane| pane.id == pane_id)?;
        Some(format!("{} · after {}", document.title(), pane.title))
    });
    let refusal = if position.is_none() {
        Some("Open a result document to add a pane to it.".to_owned())
    } else if analysis.is_none() {
        Some("No retained analysis to show.".to_owned())
    } else if draft.viewer.is_none() {
        Some("No sheet can draw this analysis in a pane.".to_owned())
    } else {
        None
    };
    AddPaneChoices {
        anchor: position.unwrap_or_default(),
        datasets,
        analyses,
        viewers,
        refusal,
    }
}

/// Bind a new pane to the draft's dataset and analysis, after the selected
/// pane. A single-pane page becomes a page of rows, since it can only ever
/// hold one.
fn add_pane(state: &mut AppState, draft: &AddPaneDraft) -> Result<(), String> {
    let document_id =
        active_document_id(state).ok_or_else(|| "No result document is open.".to_owned())?;
    let (page_id, anchor) = super::persistent_document::reading_position(state, document_id)
        .ok_or_else(|| "The result document has no pane to add after.".to_owned())?;
    let run = draft
        .dataset
        .and_then(|dataset| {
            state
                .simulation
                .retained
                .runs
                .iter()
                .find(|run| run.dataset_id == dataset)
        })
        .ok_or_else(|| "The selected dataset is no longer retained.".to_owned())?;
    let analysis = draft
        .analysis
        .and_then(|id| run.analyses.iter().find(|analysis| analysis.id == id))
        .ok_or_else(|| "The selected analysis is no longer retained.".to_owned())?;
    let viewer = draft
        .viewer
        .filter(|viewer| {
            pane_viewers(analysis)
                .iter()
                .any(|(candidate, _)| candidate == viewer)
        })
        .ok_or_else(|| "The selected sheet cannot draw this analysis.".to_owned())?;
    let viewer_id = viewer
        .viewer_document_id()
        .ok_or_else(|| "The selected sheet cannot be kept as a pane.".to_owned())?;
    let definition = viewer_document(viewer_id)
        .ok_or_else(|| "The selected sheet is not a registered viewer.".to_owned())?;
    let source =
        super::create_document::source_dataset(run, analysis).map_err(|error| error.to_string())?;
    let binding = PaneDataBinding {
        analysis_id: super::create_document::source_analysis_identity(run, analysis),
        dataset: DatasetBinding::new(run.dataset_id, run.dataset_content_digest()),
    };
    let document = project_document(state, document_id)?;
    let page = document
        .pages()
        .iter()
        .find(|page| page.id == page_id)
        .ok_or_else(|| "The selected page no longer exists.".to_owned())?;
    let mut edits = Vec::new();
    if page.layout == PageLayout::SinglePane {
        edits.push(DocumentEdit::SetPageComposition {
            page_id,
            layout: PageLayout::Rows,
            template_id: page.template_id.clone(),
            update_policy: page.update_policy,
        });
    }
    let attached = document
        .datasets()
        .iter()
        .any(|dataset| dataset.binding().dataset_id == run.dataset_id);
    edits.push(if attached {
        DocumentEdit::MergeDatasetProjection(source)
    } else {
        DocumentEdit::AttachDataset(source)
    });
    edits.push(DocumentEdit::AddBoundPane(NewPane {
        page_id,
        title: definition.title.to_owned(),
        kind: super::create_document::pane_kind(definition.art),
        viewer_id: viewer_id.to_owned(),
        binding: Some(binding),
        placement: PanePlacement::Below {
            anchor_pane_id: anchor,
        },
    }));
    let receipt = transact(state, document_id, edits)?;
    if let Some(pane_id) = receipt.created.iter().find_map(|entity| match entity {
        EntityRef::Pane(id) => Some(*id),
        _ => None,
    }) {
        state
            .ui
            .results
            .session
            .select_persistent_document_pane(document_id, pane_id);
        state.ui.results.session.viewer = viewer;
    }
    Ok(())
}

/// The context-menu edits one pane's header offers.
pub(super) fn pane_menu(
    state: &AppState,
    document_id: ResultDocumentId,
    pane_id: PaneId,
) -> PaneMenu {
    let Some(document) = state.workspace.content.visualization_document(document_id) else {
        return PaneMenu::default();
    };
    let Some((position, page_panes)) = page_order(document, pane_id) else {
        return PaneMenu::default();
    };
    PaneMenu {
        earlier: position > 0,
        later: position + 1 < page_panes.len(),
        x_link: links::x_link_state(document, pane_id),
        removable: page_panes.len() > 1,
    }
}

/// Link or unlink one pane's X axis with the rest of its page.
pub(super) fn toggle_pane_x_link(
    state: &mut AppState,
    document_id: ResultDocumentId,
    pane_id: PaneId,
) {
    let result = links::toggle_x_link(state, document_id, pane_id);
    report(state, result);
}

/// Link or unlink A/B cursors. Over a project result document the document
/// keeps the links, so they hold across panes and through a save; over a
/// quick view the session's toggle links the viewer's own cursor rows.
pub(crate) fn toggle_linked_cursors(state: &mut AppState) {
    let Some(document_id) = active_document_id(state) else {
        state.ui.results.session.toggle_linked_cursors();
        return;
    };
    let result = project_document(state, document_id)
        .map(links::cursors_linked)
        .and_then(|linked| links::set_cursor_links(state, document_id, !linked));
    report(state, result);
}

/// The pane's place in its page's reading order, with that order.
fn page_order(document: &VisualizationDocument, pane_id: PaneId) -> Option<(usize, Vec<PaneId>)> {
    let pane = document.panes().iter().find(|pane| pane.id == pane_id)?;
    let page = document
        .pages()
        .iter()
        .find(|page| page.id == pane.page_id)?;
    let order = super::persistent_document::ordered_page_panes(document.panes(), page)
        .into_iter()
        .map(|pane| pane.id)
        .collect::<Vec<_>>();
    let position = order.iter().position(|candidate| *candidate == pane_id)?;
    Some((position, order))
}

/// Which way a pane moves in its page's order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PaneMove {
    Earlier,
    Later,
}

/// Change places with the pane's neighbour in its page's order. The page
/// keeps its layout; the panes fill its slots in the new order.
pub(super) fn move_pane(
    state: &mut AppState,
    document_id: ResultDocumentId,
    pane_id: PaneId,
    direction: PaneMove,
) {
    let result = move_pane_edit(state, document_id, pane_id, direction)
        .and_then(|edit| transact(state, document_id, vec![edit]).map(|_| ()));
    report(state, result);
}

fn move_pane_edit(
    state: &AppState,
    document_id: ResultDocumentId,
    pane_id: PaneId,
    direction: PaneMove,
) -> Result<DocumentEdit, String> {
    let document = project_document(state, document_id)?;
    let (position, mut order) = page_order(document, pane_id)
        .ok_or_else(|| "The pane is no longer in this document.".to_owned())?;
    let neighbour = match direction {
        PaneMove::Earlier => position.checked_sub(1),
        PaneMove::Later => Some(position + 1).filter(|next| *next < order.len()),
    }
    .ok_or_else(|| match direction {
        PaneMove::Earlier => "The pane is already first on its page.".to_owned(),
        PaneMove::Later => "The pane is already last on its page.".to_owned(),
    })?;
    order.swap(position, neighbour);
    let page_id = document
        .panes()
        .iter()
        .find(|pane| pane.id == pane_id)
        .map(|pane| pane.page_id)
        .ok_or_else(|| "The pane is no longer in this document.".to_owned())?;
    Ok(DocumentEdit::ReorderPagePanes {
        page_id,
        pane_ids: order,
    })
}

/// Take one pane out of the document, with its axes, traces, cursors,
/// markers, measurements and notes. Link groups lose the pane's members in
/// the same transaction, and a group left with one member goes too.
pub(super) fn remove_pane(state: &mut AppState, document_id: ResultDocumentId, pane_id: PaneId) {
    let result = remove_pane_edits(state, document_id, pane_id)
        .and_then(|edits| transact(state, document_id, edits).map(|_| ()));
    report(state, result);
}

fn remove_pane_edits(
    state: &AppState,
    document_id: ResultDocumentId,
    pane_id: PaneId,
) -> Result<Vec<DocumentEdit>, String> {
    let document = project_document(state, document_id)?;
    if !document.panes().iter().any(|pane| pane.id == pane_id) {
        return Err("The pane is no longer in this document.".to_owned());
    }
    if !pane_menu(state, document_id, pane_id).removable {
        return Err("A page keeps at least one pane.".to_owned());
    }
    let owned = document
        .axes()
        .iter()
        .filter(|axis| axis.pane_id == pane_id)
        .map(|axis| EntityRef::Axis(axis.id))
        .chain(
            document
                .cursors()
                .iter()
                .filter(|cursor| cursor.pane_id == pane_id)
                .map(|cursor| EntityRef::Cursor(cursor.id)),
        )
        .collect::<Vec<_>>();
    let mut edits = Vec::new();
    for group in document.link_groups() {
        let members = group
            .members
            .iter()
            .copied()
            .filter(|member| !owned.contains(member))
            .collect::<Vec<_>>();
        if members.len() == group.members.len() {
            continue;
        }
        edits.push(if members.len() < 2 {
            DocumentEdit::Remove(EntityRef::LinkGroup(group.id))
        } else {
            DocumentEdit::SetLinkMembers {
                link_group_id: group.id,
                members,
            }
        });
    }
    edits.push(DocumentEdit::Remove(EntityRef::Pane(pane_id)));
    Ok(edits)
}

mod compare;
pub(super) mod links;

pub(crate) use compare::{compare_available, open_compare};

#[cfg(test)]
mod tests;
