//! Persistent Visualization Studio surface.
//!
//! This is the authoring projection of the same immutable result datasets
//! rendered by the Results quick view. It owns no solver samples and never
//! copies a result into a parallel data store. Viewer compatibility is
//! fail-closed: a catalog entry is selectable only when both its data contract
//! and a real Rust renderer are available.

mod actions;
mod chrome;
mod dock;

use actions::*;
use chrome::*;
mod sections;
mod stage;
mod viewers;
use viewers::show as viewers_section;

use rspice_results_ui::studio::{
    dock::{VisualizationDock, compact_dock_geometry},
    inspector::OperationState,
    session::VisualizationStudioState,
    stage::{ExactSourceRow, ResultEntityRow},
};
use sections::*;
use stage::*;

use dock::{active_family_sample_selection, dock_body};
use rspice_results_ui::studio::{
    COMPACT_BREAKPOINT, TOUCH_DOCK_HEIGHT,
    chrome::{compact_section_picker, section_navigation, touch_dock},
};

use std::collections::{BTreeMap, BTreeSet, HashSet};

use egui::{Align, Id, Layout, Ui, vec2};

use crate::analysis::calculator;
use crate::diagnostics::ConsoleMessage;
use crate::product::{AnalysisInstanceId, DatasetBinding, DatasetId, ResultDocumentId};
#[cfg(test)]
use crate::results::viewer_catalog::VIEWER_DOCUMENTS;
use crate::results::viewer_catalog::{
    ViewerArt, ViewerCapabilities, ViewerCompatibility, ViewerDocumentDefinition,
    viewer_compatibility, viewer_document,
};
use crate::results::visualization_document::{
    AccessibleColorPalette, AxisOrientation, ComparisonReceipt, CursorId, DocumentEdit, EntityRef,
    FamilyAggregationMethod, FamilyAggregationPolicy, FamilyDimension as DocumentFamilyDimension,
    FamilyEncodingMap, FamilyPresentationPolicy, FamilyXDimension, FamilyXOrdering, LinkKind,
    MissingPointPolicy, PageUpdatePolicy, PaneId, SourceDataset, TypedValue, ValueType,
    VisualizationDocument, VisualizationTransactionReceipt,
};
use crate::state::{
    AnalysisResult, AnalysisResultPayload, AnalysisType, SensitivityResultMode,
    SensitivityResultRow, SimulationRun,
};
use crate::ui::tokens::Tokens;
use crate::workbench::{AppState, RSpiceApp};

use crate::workbench::{
    ChoicePreference, ResultViewer, RouteTransitionSource, ScalarPreference, SurfaceId,
    SurfaceRoute,
    state::{Workspace, WorkspaceDocumentId},
};

use rspice_results::studio_presentation::{
    ComparisonAlignmentDraft, MAX_DIFFERENCE_TRACE_NUMERIC_VALUES, MAX_DIFFERENCE_TRACE_SETS,
    REPORT_PAGE_TEMPLATES,
};
pub use rspice_results::studio_presentation::{
    ComplexProjection, DisplayLodPolicy, ViewerTool, VisualizationAnnotation,
    VisualizationAutoscale, VisualizationDifferenceKind, VisualizationDifferenceSeries,
    VisualizationDifferenceTraceSet, VisualizationMarker, VisualizationMeasurement,
    VisualizationPane, VisualizationPanePlacement, VisualizationReportPagePolicy,
    VisualizationSection, VisualizationStudioPresentation, VisualizationTouchPane,
};

use super::result_document;
use rspice_results::family_projection::{FamilyManifest, FamilyValueKind, SourceSampleSelection};

const fn document_pane_kind(art: ViewerArt) -> crate::results::visualization_document::PaneKind {
    use crate::results::visualization_document::PaneKind;
    match art {
        ViewerArt::Smith => PaneKind::Smith,
        ViewerArt::Polar => PaneKind::Polar,
        ViewerArt::Histogram => PaneKind::Histogram,
        ViewerArt::Table => PaneKind::Table,
        _ => PaneKind::Cartesian,
    }
}

fn report_visualization_commit(app: &mut RSpiceApp, result: Result<(), String>) -> bool {
    match result {
        Ok(()) => true,
        Err(error) => {
            app.state.push_user_message(ConsoleMessage::error(error));
            false
        }
    }
}

fn commit_visualization_revision(app: &mut RSpiceApp) -> bool {
    let result = app.state.workbench.visualization_studio.commit_revision();
    report_visualization_commit(app, result)
}

fn active_project_visualization_document_id(state: &AppState) -> Option<ResultDocumentId> {
    match state.workbench.documents.active(Workspace::Results) {
        Some(WorkspaceDocumentId::VisualizationDocument(document_id)) => Some(*document_id),
        _ => None,
    }
}

fn transact_active_project_document(
    app: &mut RSpiceApp,
    edits: Vec<DocumentEdit>,
) -> Result<VisualizationTransactionReceipt, String> {
    let document_id = active_project_visualization_document_id(&app.state)
        .ok_or_else(|| "Open a project-owned result document before editing it.".to_owned())?;
    let revision = app
        .state
        .workspace
        .content
        .visualization_document(document_id)
        .ok_or_else(|| "The active result document is no longer retained.".to_owned())?
        .revision();
    app.state
        .workspace
        .content
        .transact_visualization_document(document_id, revision, edits)
        .map_err(|error| error.to_string())
}

fn real_cursor_position(cursor: &crate::results::visualization_document::Cursor) -> Option<f64> {
    match &cursor.position {
        TypedValue::Real(position) => Some(*position),
        _ => None,
    }
}

fn same_cursor_position(left: f64, right: f64) -> bool {
    left.to_bits() == right.to_bits()
}

fn canonical_cursor_pair(
    document: &VisualizationDocument,
    pane_id: PaneId,
) -> Result<(Option<f64>, Option<f64>), String> {
    let mut pair = (None, None);
    for cursor in document
        .cursors()
        .iter()
        .filter(|cursor| cursor.pane_id == pane_id)
    {
        let slot = match cursor.label.as_str() {
            "A" => &mut pair.0,
            "B" => &mut pair.1,
            _ => continue,
        };
        if slot.is_some() {
            return Err(format!(
                "Pane {} contains more than one retained {} cursor.",
                pane_id.get(),
                cursor.label
            ));
        }
        *slot = Some(real_cursor_position(cursor).ok_or_else(|| {
            format!(
                "Retained {} cursor {} is not a real-valued horizontal cursor.",
                cursor.label,
                cursor.id.get()
            )
        })?);
    }
    Ok(pair)
}

fn canonical_cursor_pair_edits(
    document: &VisualizationDocument,
    pane_id: PaneId,
    desired: (Option<f64>, Option<f64>),
) -> Result<Vec<DocumentEdit>, String> {
    document
        .panes()
        .iter()
        .find(|pane| pane.id == pane_id)
        .ok_or_else(|| format!("Result-document pane {} no longer exists.", pane_id.get()))?;
    let axis_id = document
        .axes()
        .iter()
        .find(|axis| axis.pane_id == pane_id && axis.orientation == AxisOrientation::Horizontal)
        .map(|axis| axis.id)
        .ok_or_else(|| "The active result pane has no horizontal cursor axis.".to_owned())?;
    if desired
        .0
        .into_iter()
        .chain(desired.1)
        .any(|position| !position.is_finite())
    {
        return Err("A retained cursor position must be finite.".to_owned());
    }

    let mut removed_groups = BTreeSet::new();
    let mut removed_cursors = BTreeSet::new();
    let mut moves = BTreeMap::<CursorId, f64>::new();
    let mut additions = Vec::new();

    for (label, target) in [("A", desired.0), ("B", desired.1)] {
        let existing = document
            .cursors()
            .iter()
            .filter(|cursor| cursor.pane_id == pane_id && cursor.label == label)
            .collect::<Vec<_>>();
        if existing.len() > 1 {
            return Err(format!(
                "Pane {} contains duplicate retained {label} cursors.",
                pane_id.get()
            ));
        }
        match (existing.first().copied(), target) {
            (Some(cursor), Some(position)) => {
                let mut targets = BTreeSet::from([cursor.id]);
                for group in document.link_groups().iter().filter(|group| {
                    group.kind == LinkKind::CursorPosition
                        && group.members.contains(&EntityRef::Cursor(cursor.id))
                }) {
                    for member in &group.members {
                        if let EntityRef::Cursor(cursor_id) = member {
                            targets.insert(*cursor_id);
                        }
                    }
                }
                for cursor_id in targets {
                    let retained = document
                        .cursors()
                        .iter()
                        .find(|candidate| candidate.id == cursor_id)
                        .ok_or_else(|| {
                            format!("Linked cursor {} no longer exists.", cursor_id.get())
                        })?;
                    let retained_position = real_cursor_position(retained).ok_or_else(|| {
                        format!("Linked cursor {} is not real-valued.", cursor_id.get())
                    })?;
                    if !same_cursor_position(retained_position, position) {
                        moves.insert(cursor_id, position);
                    }
                }
            }
            (Some(cursor), None) => {
                let linked_groups = document
                    .link_groups()
                    .iter()
                    .filter(|group| {
                        group.kind == LinkKind::CursorPosition
                            && group.members.contains(&EntityRef::Cursor(cursor.id))
                    })
                    .collect::<Vec<_>>();
                if linked_groups.is_empty() {
                    removed_cursors.insert(cursor.id);
                } else {
                    for group in linked_groups {
                        removed_groups.insert(group.id);
                        for member in &group.members {
                            if let EntityRef::Cursor(cursor_id) = member {
                                removed_cursors.insert(*cursor_id);
                            }
                        }
                    }
                }
            }
            (None, Some(position)) => additions.push(DocumentEdit::AddCursor {
                pane_id,
                axis_id,
                position: TypedValue::Real(position),
                label: label.to_owned(),
            }),
            (None, None) => {}
        }
    }

    let mut edits = removed_groups
        .into_iter()
        .map(|group_id| DocumentEdit::Remove(EntityRef::LinkGroup(group_id)))
        .collect::<Vec<_>>();
    edits.extend(
        moves
            .into_iter()
            .filter(|(cursor_id, _)| !removed_cursors.contains(cursor_id))
            .map(|(cursor_id, position)| DocumentEdit::MoveCursor {
                cursor_id,
                position: TypedValue::Real(position),
            }),
    );
    edits.extend(
        removed_cursors
            .into_iter()
            .map(|cursor_id| DocumentEdit::Remove(EntityRef::Cursor(cursor_id))),
    );
    edits.extend(additions);
    Ok(edits)
}

fn commit_active_project_cursor_pair(
    app: &mut RSpiceApp,
    pane_id: u64,
    desired: (Option<f64>, Option<f64>),
) -> bool {
    let Some(document_id) = active_project_visualization_document_id(&app.state) else {
        return false;
    };
    let plan = app
        .state
        .workspace
        .content
        .visualization_document(document_id)
        .ok_or_else(|| "The active result document is no longer retained.".to_owned())
        .and_then(|document| {
            let pane_id = document
                .panes()
                .iter()
                .find(|pane| pane.id.get() == pane_id)
                .map(|pane| pane.id)
                .ok_or_else(|| "The active result pane no longer exists.".to_owned())?;
            canonical_cursor_pair_edits(document, pane_id, desired)
        });
    let edits = match plan {
        Ok(edits) => edits,
        Err(error) => {
            app.state.push_user_message(ConsoleMessage::error(error));
            app.state
                .workbench
                .visualization_studio
                .pane_cursor_positions
                .remove(&pane_id);
            app.state.workbench.visualization_studio.applied_link_pane = None;
            reconcile_document(app);
            return true;
        }
    };
    if edits.is_empty() {
        return true;
    }
    let result = transact_active_project_document(app, edits).map(|_| ());
    let committed = report_visualization_commit(app, result);
    if committed {
        reconcile_document(app);
    } else {
        app.state.workbench.visualization_studio.applied_link_pane = None;
        reconcile_document(app);
    }
    true
}

fn set_active_project_cursor_links(app: &mut RSpiceApp, enabled: bool) -> bool {
    let Some(document_id) = active_project_visualization_document_id(&app.state) else {
        return false;
    };
    let active_pane_id = app.state.workbench.visualization_studio.active_pane;
    let plan = app
        .state
        .workspace
        .content
        .visualization_document(document_id)
        .ok_or_else(|| "The active result document is no longer retained.".to_owned())
        .and_then(|document| {
            let mut edits = document
                .link_groups()
                .iter()
                .filter(|group| group.kind == LinkKind::CursorPosition)
                .map(|group| DocumentEdit::Remove(EntityRef::LinkGroup(group.id)))
                .collect::<Vec<_>>();
            if enabled {
                let mut added = 0_usize;
                for label in ["A", "B"] {
                    let candidates = document
                        .cursors()
                        .iter()
                        .filter(|cursor| cursor.label == label)
                        .collect::<Vec<_>>();
                    let Some(reference) = candidates
                        .iter()
                        .copied()
                        .find(|cursor| Some(cursor.pane_id.get()) == active_pane_id)
                        .or_else(|| candidates.first().copied())
                    else {
                        continue;
                    };
                    let reference_axis = document
                        .axes()
                        .iter()
                        .find(|axis| axis.id == reference.axis_id)
                        .ok_or_else(|| {
                            format!("Cursor {} has no retained axis.", reference.id.get())
                        })?;
                    let compatible = candidates
                        .into_iter()
                        .filter(|cursor| {
                            document
                                .axes()
                                .iter()
                                .find(|axis| axis.id == cursor.axis_id)
                                .is_some_and(|axis| {
                                    axis.orientation == AxisOrientation::Horizontal
                                        && axis.scale == reference_axis.scale
                                        && axis.unit == reference_axis.unit
                                })
                        })
                        .collect::<Vec<_>>();
                    if compatible.len() >= 2 {
                        let reference_position =
                            real_cursor_position(reference).ok_or_else(|| {
                                format!("Cursor {} is not real-valued.", reference.id.get())
                            })?;
                        for cursor in &compatible {
                            let position = real_cursor_position(cursor).ok_or_else(|| {
                                format!("Cursor {} is not real-valued.", cursor.id.get())
                            })?;
                            if !same_cursor_position(position, reference_position) {
                                edits.push(DocumentEdit::MoveCursor {
                                    cursor_id: cursor.id,
                                    position: TypedValue::Real(reference_position),
                                });
                            }
                        }
                        edits.push(DocumentEdit::AddLinkGroup {
                            label: format!("{label} cursor link"),
                            kind: LinkKind::CursorPosition,
                            members: compatible
                                .into_iter()
                                .map(|cursor| EntityRef::Cursor(cursor.id))
                                .collect(),
                        });
                        added += 1;
                    }
                }
                if added == 0 {
                    return Err(
                        "Place the same A or B cursor on at least two panes before linking it."
                            .to_owned(),
                    );
                }
            }
            Ok(edits)
        });
    let edits = match plan {
        Ok(edits) => edits,
        Err(error) => {
            app.state.push_user_message(ConsoleMessage::warning(error));
            return true;
        }
    };
    if edits.is_empty() {
        return true;
    }
    let result = transact_active_project_document(app, edits).map(|_| ());
    if report_visualization_commit(app, result) {
        reconcile_document(app);
    }
    true
}

fn active_project_pane_and_trace(
    state: &AppState,
    preferred_signal: Option<&str>,
) -> Result<
    (
        crate::results::visualization_document::PaneId,
        crate::results::visualization_document::TraceId,
    ),
    String,
> {
    let document_id = active_project_visualization_document_id(state).ok_or_else(|| {
        "Open a project-owned result document before authoring entities.".to_owned()
    })?;
    let document = state
        .workspace
        .content
        .visualization_document(document_id)
        .ok_or_else(|| "The active result document is no longer retained.".to_owned())?;
    let pane_id = state
        .workbench
        .visualization_studio
        .active_pane
        .and_then(|active| {
            document
                .panes()
                .iter()
                .find(|pane| pane.id.get() == active)
                .map(|pane| pane.id)
        })
        .or_else(|| document.panes().first().map(|pane| pane.id))
        .ok_or_else(|| "The active result document has no pane.".to_owned())?;
    let trace = preferred_signal
        .and_then(|signal| {
            document.traces().iter().find(|trace| {
                trace.pane_id == pane_id && (trace.signal_key == signal || trace.label == signal)
            })
        })
        .or_else(|| {
            document
                .traces()
                .iter()
                .find(|trace| trace.pane_id == pane_id)
        })
        .ok_or_else(|| "The selected result pane has no retained trace.".to_owned())?;
    Ok((pane_id, trace.id))
}

pub(crate) fn open(app: &mut RSpiceApp) {
    if let Err(error) = navigate_to_visualization_studio(app) {
        app.state.push_user_message(ConsoleMessage::warning(error));
    }
}

fn navigate_to_visualization_studio(app: &mut RSpiceApp) -> Result<(), String> {
    let route = SurfaceRoute::surface(SurfaceId::VisualizationStudio);
    app.state
        .workbench
        .navigate(route, RouteTransitionSource::User)
        .map_err(|error| error.to_string())?;
    app.state.workbench.workspace = Workspace::Results;
    app.state.workbench.visualization_studio.normalize();
    app.state
        .workbench
        .specialist_tool_browser
        .record_recent(SurfaceId::VisualizationStudio);
    Ok(())
}

pub(crate) fn open_add_pane(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::AddPane);
}

pub(crate) fn open_trace_manager(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::TraceManager);
}

pub(crate) fn open_cursor_manager(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::CursorManager);
}

pub(crate) fn open_document_properties(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::DocumentProperties);
}

pub(crate) fn open_measurement_editor(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::Measurement);
}

pub(crate) fn open_annotation_editor(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::Annotation);
}

pub(crate) fn open_family_slicing(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::FamilySlice);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ResultsComparisonSource {
    dataset_id: DatasetId,
    analysis_sequence: u64,
    viewer: ResultViewer,
}

fn retained_document_analysis(
    run: &SimulationRun,
    source_id: AnalysisInstanceId,
) -> Option<&AnalysisResult> {
    run.analyses.iter().find(|analysis| {
        analysis.provenance().map_or_else(
            || {
                let name = format!("legacy-analysis-v1/{}", analysis.id);
                AnalysisInstanceId::from_namespace(run.dataset_id.as_uuid(), name.as_bytes())
                    == source_id
            },
            |provenance| {
                provenance.source_instance_id() == source_id
                    || provenance.authored_source_instance_id() == source_id
            },
        )
    })
}

fn active_results_comparison_source(state: &AppState) -> Result<ResultsComparisonSource, String> {
    if state.workbench.workspace != Workspace::Results {
        return Err("Open a result document before comparing datasets.".to_owned());
    }
    let document = crate::workbench::chrome::document_bar::active_document_id(state)
        .ok_or_else(|| "No active result document is available.".to_owned())?;
    let mut authored_analysis = None;
    let mut viewer = state.ui.results.session.viewer;
    let (dataset_id, expected_digest) = match document {
        WorkspaceDocumentId::ResultDataset(dataset_id) => {
            state
                .simulation
                .runs
                .iter()
                .find(|run| run.dataset_id == dataset_id)
                .ok_or_else(|| "The active result dataset is no longer retained.".to_owned())?;
            (dataset_id, None)
        }
        WorkspaceDocumentId::VisualizationDocument(document_id) => {
            let document = state
                .workspace
                .content
                .visualization_document(document_id)
                .ok_or_else(|| "The active result document is no longer retained.".to_owned())?;
            if let Some((pane, pane_binding)) = document
                .panes()
                .iter()
                .find_map(|pane| pane.binding.map(|binding| (pane, binding)))
            {
                authored_analysis = Some(pane_binding.analysis_id);
                viewer = ResultViewer::from_viewer_document_id(&pane.viewer_id)
                    .unwrap_or(ResultViewer::Waves);
                (
                    pane_binding.dataset.dataset_id,
                    Some(pane_binding.dataset.content_digest),
                )
            } else {
                let binding = document
                    .datasets()
                    .first()
                    .map(SourceDataset::binding)
                    .ok_or_else(|| {
                        "The active result document has no immutable dataset binding.".to_owned()
                    })?;
                (binding.dataset_id, Some(binding.content_digest))
            }
        }
        _ => return Err("The active document is not a result dataset.".to_owned()),
    };
    let run = state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == dataset_id)
        .ok_or_else(|| "The active result document's dataset is no longer retained.".to_owned())?;
    if expected_digest.is_some_and(|digest| run.dataset_content_digest() != digest) {
        return Err(
            "The active result document's immutable dataset digest does not match retained data."
                .to_owned(),
        );
    }
    let analysis = if let Some(authored_analysis) = authored_analysis {
        retained_document_analysis(run, authored_analysis).ok_or_else(|| {
            "The active result document's authored analysis is no longer retained.".to_owned()
        })?
    } else if state
        .simulation
        .active_run()
        .is_some_and(|active| active.dataset_id == dataset_id)
    {
        state
            .simulation
            .active_analysis()
            .or_else(|| run.analyses.first())
            .ok_or_else(|| "The active result dataset contains no analysis.".to_owned())?
    } else {
        run.analyses
            .first()
            .ok_or_else(|| "The active result dataset contains no analysis.".to_owned())?
    };
    viewer = result_document::project_viewer_for_analysis(viewer, analysis);
    Ok(ResultsComparisonSource {
        dataset_id,
        analysis_sequence: analysis.id,
        viewer,
    })
}

pub(crate) fn results_comparison_available(state: &AppState) -> bool {
    active_results_comparison_source(state).is_ok_and(|source| {
        !dock::compatible_comparison_dataset_ids(state, source.dataset_id, source.analysis_sequence)
            .is_empty()
    })
}

fn select_results_comparison_source(
    app: &mut RSpiceApp,
    source: ResultsComparisonSource,
) -> Result<(), String> {
    let run_index = app
        .state
        .simulation
        .runs
        .iter()
        .position(|run| run.dataset_id == source.dataset_id)
        .ok_or_else(|| "The candidate dataset is no longer retained.".to_owned())?;
    let analysis_index = app.state.simulation.runs[run_index]
        .analyses
        .iter()
        .position(|analysis| analysis.id == source.analysis_sequence)
        .ok_or_else(|| "The candidate analysis is no longer retained.".to_owned())?;
    if !app.state.simulation.select_run(run_index)
        || !app.state.simulation.select_analysis(analysis_index)
    {
        return Err("The candidate dataset could not be activated.".to_owned());
    }
    Ok(())
}

fn bind_comparison_owner(
    app: &mut RSpiceApp,
    source: ResultsComparisonSource,
) -> Result<(), String> {
    let viewer_document = source
        .viewer
        .viewer_document_id()
        .ok_or_else(|| {
            "Dataset-native result projections cannot be bound as Visualization Studio panes"
                .to_owned()
        })?
        .to_owned();
    let studio = &mut app.state.workbench.visualization_studio;
    studio.normalize();
    let pane_id = studio
        .panes
        .iter()
        .find(|pane| {
            pane.dataset_id == source.dataset_id
                && pane.analysis_sequence == source.analysis_sequence
                && pane.viewer == source.viewer
        })
        .map(|pane| pane.id);
    let pane_id = if let Some(pane_id) = pane_id {
        pane_id
    } else {
        studio.transact(|studio| {
            let pane_id = studio
                .allocate_identity()
                .ok_or_else(|| "Visualization pane identity space is exhausted.".to_owned())?;
            studio.panes.push(VisualizationPane {
                id: pane_id,
                viewer: source.viewer,
                viewer_document_id: viewer_document.clone(),
                dataset_id: source.dataset_id,
                analysis_sequence: source.analysis_sequence,
                x_link: None,
                cursor_group: None,
                page: "Engineering".to_owned(),
                placement: VisualizationPanePlacement::BelowSelected,
            });
            Ok(pane_id)
        })?
    };
    studio.active_pane = Some(pane_id);
    studio.selected_viewer_document = viewer_document;
    studio.applied_link_pane = None;
    studio.section = VisualizationSection::Viewers;
    app.state.ui.results.session.viewer = source.viewer;
    Ok(())
}

pub(crate) fn open_results_comparison(app: &mut RSpiceApp) {
    if let Err(error) = open_results_comparison_inner(app) {
        app.state.push_user_message(ConsoleMessage::warning(error));
    }
}

fn open_results_comparison_inner(app: &mut RSpiceApp) -> Result<(), String> {
    let source = active_results_comparison_source(&app.state)?;
    let comparison_datasets = dock::compatible_comparison_dataset_ids(
        &app.state,
        source.dataset_id,
        source.analysis_sequence,
    );
    if comparison_datasets.is_empty() {
        return Err(
            "A second compatible immutable dataset with an exact matching analysis and coordinate axis is required."
                .to_owned(),
        );
    }
    navigate_to_visualization_studio(app)?;
    select_results_comparison_source(app, source)?;
    bind_comparison_owner(app, source)?;
    let comparison_data_version = app.state.simulation.data_version;
    initialize_comparison_dock(
        &mut app.state.workbench.visualization_studio,
        comparison_datasets,
        comparison_data_version,
    );
    Ok(())
}

pub(crate) fn export_document(app: &mut RSpiceApp) {
    open_dock(app, VisualizationDock::Export);
}

fn initialize_comparison_dock(
    studio: &mut VisualizationStudioState,
    comparison_datasets: Vec<DatasetId>,
    comparison_data_version: u64,
) {
    studio.draft_comparison_dataset = comparison_datasets.first().copied();
    studio.draft_comparison_candidates = comparison_datasets;
    studio.draft_comparison_data_version = comparison_data_version;
    studio.draft_comparison_absolute_tolerance = 0.0;
    studio.draft_comparison_relative_tolerance = 0.0;
    studio.draft_comparison_alignment = ComparisonAlignmentDraft::default();
    studio.draft_comparison_alignment_signal.clear();
    studio.draft_comparison_threshold = 0.0;
    studio.draft_comparison_maximum_lag_samples = 128;
    studio.draft_comparison_difference_trace = true;
    studio.dock = Some(VisualizationDock::Comparison);
}

fn open_dock(app: &mut RSpiceApp, dock: VisualizationDock) {
    if dock == VisualizationDock::CursorManager
        && app.state.ui.results.session.viewer != ResultViewer::Waves
    {
        app.state.push_user_message(ConsoleMessage::warning(
            "Exact source cursor management is available in the waveform renderer.",
        ));
        return;
    }
    let active_pane = app
        .state
        .workbench
        .visualization_studio
        .active_pane()
        .cloned();
    let active_binding = app.state.simulation.active_run().and_then(|run| {
        app.state
            .simulation
            .active_analysis()
            .map(|analysis| (run.dataset_id, analysis.id))
    });
    let trace_source = app.state.simulation.active_run().and_then(|run| {
        app.state
            .simulation
            .active_analysis()
            .map(|analysis| (run.dataset_id, analysis.id, analysis.waveforms.clone()))
    });
    let phase_continuous = app.state.ui.results.session.phase_continuous;
    let active_viewer = app.state.ui.results.session.viewer;
    let family_manifest = app.state.simulation.active_analysis().and_then(|analysis| {
        FamilyManifest::from_metadata(analysis.analysis_type, analysis.family_metadata.as_ref())
            .ok()
            .flatten()
    });
    let active_family_policy = active_pane.as_ref().and_then(|pane| {
        app.state
            .workbench
            .visualization_studio
            .family_policies
            .get(&pane.id)
            .cloned()
    });
    let comparison_datasets = if dock == VisualizationDock::Comparison {
        active_binding.map_or_else(Vec::new, |(dataset, analysis)| {
            dock::compatible_comparison_dataset_ids(&app.state, dataset, analysis)
        })
    } else {
        Vec::new()
    };
    let comparison_data_version = app.state.simulation.data_version;
    let studio = &mut app.state.workbench.visualization_studio;
    match dock {
        VisualizationDock::AddPane => {
            studio.draft_viewer = active_viewer;
            studio.draft_dataset_id = active_binding.map(|binding| binding.0);
            studio.draft_analysis_sequence = active_binding.map(|binding| binding.1);
            studio.draft_pane_placement = VisualizationPanePlacement::BelowSelected;
            let page_count = studio
                .panes
                .iter()
                .map(|pane| pane.page.as_str())
                .collect::<HashSet<_>>()
                .len();
            studio.draft_page_title = format!("Page {}", page_count.saturating_add(1));
        }
        VisualizationDock::TraceManager => {
            if let Some((dataset_id, analysis_id, waveforms)) = trace_source {
                studio.draft_trace_dataset = Some(dataset_id);
                studio.draft_trace_analysis = Some(analysis_id);
                studio.draft_trace_visibility = waveforms
                    .into_iter()
                    .map(|waveform| (waveform.data.name, waveform.visible))
                    .collect();
            } else {
                studio.draft_trace_dataset = None;
                studio.draft_trace_analysis = None;
                studio.draft_trace_visibility.clear();
            }
        }
        VisualizationDock::DocumentProperties => {
            studio.draft_significant_digits = Some(studio.significant_digits);
            studio.draft_phase_continuous = Some(phase_continuous);
        }
        VisualizationDock::ReorderPanes => {
            studio.draft_pane_order = studio.panes.iter().map(|pane| pane.id).collect();
        }
        VisualizationDock::LinkGroups => {
            studio.draft_link_pane = active_pane.as_ref().map(|pane| pane.id);
            studio.draft_x_link = active_pane
                .as_ref()
                .and_then(|pane| pane.x_link)
                .unwrap_or_default();
            studio.draft_cursor_group = active_pane
                .as_ref()
                .and_then(|pane| pane.cursor_group)
                .unwrap_or_default();
        }
        VisualizationDock::PageEditor => {
            studio.draft_page_pane = active_pane.as_ref().map(|pane| pane.id);
            studio.draft_page = active_pane.map_or_else(String::new, |pane| pane.page);
            if let Some(policy) = studio
                .presentation
                .report_page_policies
                .get(&studio.draft_page)
            {
                studio.draft_report_template = policy.template.clone();
                studio.draft_report_freeze =
                    policy.update_policy == PageUpdatePolicy::FreezeFigureRevision;
            } else {
                studio.draft_report_template = REPORT_PAGE_TEMPLATES[0].to_owned();
                studio.draft_report_freeze = false;
            }
        }
        VisualizationDock::Measurement => {}
        VisualizationDock::FamilySlice => {
            initialize_family_draft(
                studio,
                family_manifest.as_ref(),
                active_family_policy.as_ref(),
            );
            if active_family_policy.is_none() {
                studio.family_query = studio.family_query.replacen(" and ", " · ", 1);
            }
        }
        VisualizationDock::FamilyFilter => {
            initialize_family_draft(
                studio,
                family_manifest.as_ref(),
                active_family_policy.as_ref(),
            );
        }
        VisualizationDock::FamilyEncoding => {
            initialize_family_draft(
                studio,
                family_manifest.as_ref(),
                active_family_policy.as_ref(),
            );
        }
        VisualizationDock::Comparison => {
            initialize_comparison_dock(studio, comparison_datasets, comparison_data_version);
        }
        VisualizationDock::CursorManager
        | VisualizationDock::Annotation
        | VisualizationDock::Export => {}
    }
    if dock != VisualizationDock::Comparison {
        studio.dock = Some(dock);
    }
}

fn initialize_family_draft(
    studio: &mut VisualizationStudioState,
    manifest: Option<&FamilyManifest>,
    policy: Option<&FamilyPresentationPolicy>,
) {
    let Some(manifest) = manifest else {
        studio.draft_family_x_dimension.clear();
        studio.draft_family_dimension.clear();
        studio.draft_family_color_dimension.clear();
        studio.draft_family_dash_dimension.clear();
        studio.draft_family_marker_dimension.clear();
        studio.family_query.clear();
        return;
    };

    let preferred_x = manifest
        .dimensions
        .iter()
        .find(|dimension| {
            dimension.kind == FamilyValueKind::Number
                && !matches!(dimension.id.as_str(), "temperature" | "sample")
        })
        .or_else(|| {
            manifest.dimensions.iter().find(|dimension| {
                matches!(
                    dimension.kind,
                    FamilyValueKind::Number | FamilyValueKind::Integer
                ) && dimension.id != "temperature"
            })
        })
        .map(|dimension| dimension.id.clone())
        .unwrap_or_else(|| "sample".to_owned());
    studio.draft_family_x_dimension = policy
        .map(|policy| policy.x_dimension.dimension.key.clone())
        .unwrap_or(preferred_x);

    let preferred_family = manifest
        .dimension("process")
        .or_else(|| {
            manifest.dimensions.iter().find(|dimension| {
                dimension.id != studio.draft_family_x_dimension
                    && !matches!(dimension.id.as_str(), "sample" | "status")
            })
        })
        .or_else(|| manifest.dimension("sample"))
        .map(|dimension| dimension.id.clone())
        .unwrap_or_default();
    studio.draft_family_dimension = policy
        .and_then(|policy| policy.family_dimensions.first())
        .map(|dimension| dimension.key.clone())
        .unwrap_or(preferred_family);

    let encoding_dimension = |predicate: fn(&FamilyEncodingMap) -> bool| {
        policy.and_then(|policy| {
            policy
                .encodings
                .iter()
                .find(|encoding| predicate(encoding))
                .map(|encoding| encoding.dimension().key.clone())
        })
    };
    studio.draft_family_color_dimension =
        encoding_dimension(|encoding| matches!(encoding, FamilyEncodingMap::Color { .. }))
            .unwrap_or_else(|| studio.draft_family_dimension.clone());
    studio.draft_family_dash_dimension =
        encoding_dimension(|encoding| matches!(encoding, FamilyEncodingMap::Dash { .. }))
            .or_else(|| {
                manifest
                    .dimension("temperature")
                    .map(|dimension| dimension.id.clone())
            })
            .unwrap_or_default();
    studio.draft_family_marker_dimension =
        encoding_dimension(|encoding| matches!(encoding, FamilyEncodingMap::Marker { .. }))
            .unwrap_or_else(|| studio.draft_family_color_dimension.clone());
    studio.draft_family_exclude_missing = policy.is_some_and(|policy| {
        policy.missing_points == MissingPointPolicy::ExcludeWithOmissionRecord
    });
    studio.family_query = policy
        .and_then(|policy| policy.filter.as_ref())
        .map(|filter| filter.source.clone())
        .unwrap_or_else(|| {
            if manifest.dimension("temperature").is_some() {
                "temperature in {27,125} and status != not-run".to_owned()
            } else {
                "status != not-run".to_owned()
            }
        });
}

pub(crate) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    reconcile_document(app);
    synchronize_runtime_policies(app);
    let compact = ui.available_width() <= COMPACT_BREAKPOINT
        || app.state.workbench.coarse_pointer
        || ui.ctx().input(|input| input.has_touch_screen());
    let t = Tokens::get(ui.ctx());
    let rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);

    workspace_header(ui, app);
    status_strip(ui, app);
    if !compact {
        section_navigation(ui, &mut app.state.workbench.visualization_studio.section);
    }

    let dock_height = if compact { TOUCH_DOCK_HEIGHT } else { 0.0 };
    let body_height = (ui.available_height() - dock_height).max(1.0);
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), body_height),
        Layout::top_down(Align::Min),
        |ui| {
            if compact
                && app.state.workbench.visualization_studio.touch_pane
                    == VisualizationTouchPane::Sections
            {
                let studio: &mut VisualizationStudioPresentation =
                    &mut app.state.workbench.visualization_studio;
                compact_section_picker(ui, &mut studio.section, &mut studio.touch_pane);
                return;
            }
            show_active_section(ui, app, compact);
        },
    );
    if compact {
        touch_dock(ui, &mut app.state.workbench.visualization_studio.touch_pane);
    }
    show_dock_if_open(ui, app, compact);
}

fn reconcile_document(app: &mut RSpiceApp) {
    let active_dataset = app.state.simulation.active_run().map(|run| run.dataset_id);
    let active_analysis_sequence = app
        .state
        .simulation
        .active_analysis()
        .map(|analysis| analysis.id);
    let requested_viewer = app.state.ui.results.session.viewer;
    let (viewer, viewer_document_id) = requested_viewer.viewer_document_id().map_or_else(
        || (ResultViewer::Waves, "viewer-waveform".to_owned()),
        |document_id| (requested_viewer, document_id.to_owned()),
    );
    if let Some(document_id) = active_project_visualization_document_id(&app.state) {
        let projected = app
            .state
            .workspace
            .content
            .visualization_document(document_id)
            .map(|document| {
                let analysis_sequence_for =
                    |dataset_id: DatasetId, analysis_id: AnalysisInstanceId| {
                        app.state
                            .simulation
                            .runs
                            .iter()
                            .find(|run| run.dataset_id == dataset_id)
                            .and_then(|run| retained_document_analysis(run, analysis_id))
                            .map(|analysis| analysis.id)
                    };
                let mut ordered_panes = document.panes().iter().collect::<Vec<_>>();
                let x_links = document
                    .link_groups()
                    .iter()
                    .filter(|group| group.kind == LinkKind::HorizontalViewport)
                    .flat_map(|group| {
                        group.members.iter().filter_map(move |member| {
                            let crate::results::visualization_document::EntityRef::Axis(axis_id) =
                                member
                            else {
                                return None;
                            };
                            document
                                .axes()
                                .iter()
                                .find(|axis| axis.id == *axis_id)
                                .map(|axis| (axis.pane_id.get(), group.id.get()))
                        })
                    })
                    .collect::<BTreeMap<_, _>>();
                ordered_panes.sort_by_key(|pane| {
                    (
                        document
                            .pages()
                            .iter()
                            .position(|page| page.id == pane.page_id)
                            .unwrap_or(usize::MAX),
                        pane.order,
                        pane.id,
                    )
                });
                let panes = ordered_panes
                    .into_iter()
                    .map(|pane| {
                        let viewer = ResultViewer::from_viewer_document_id(&pane.viewer_id)
                            .unwrap_or(ResultViewer::Waves);
                        let dataset_id = pane
                            .binding
                            .map(|binding| binding.dataset.dataset_id)
                            .or_else(|| {
                                document
                                    .datasets()
                                    .first()
                                    .map(|dataset| dataset.binding().dataset_id)
                            })
                            .unwrap_or_else(DatasetId::new);
                        let analysis_sequence = pane
                            .binding
                            .and_then(|binding| {
                                app.state
                                    .simulation
                                    .runs
                                    .iter()
                                    .find(|run| run.dataset_id == dataset_id)
                                    .and_then(|run| {
                                        retained_document_analysis(run, binding.analysis_id)
                                    })
                                    .map(|analysis| analysis.id)
                            })
                            .unwrap_or_default();
                        let page = document
                            .pages()
                            .iter()
                            .find(|page| page.id == pane.page_id)
                            .map_or_else(|| "Engineering".to_owned(), |page| page.title.clone());
                        let placement = match pane.placement {
                            crate::results::visualization_document::PanePlacement::RightOf {
                                ..
                            } => VisualizationPanePlacement::RightOfSelected,
                            crate::results::visualization_document::PanePlacement::Primary
                            | crate::results::visualization_document::PanePlacement::Below {
                                ..
                            } => VisualizationPanePlacement::BelowSelected,
                        };
                        VisualizationPane {
                            id: pane.id.get(),
                            viewer,
                            viewer_document_id: pane.viewer_id.clone(),
                            dataset_id,
                            analysis_sequence,
                            x_link: x_links.get(&pane.id.get()).copied(),
                            cursor_group: None,
                            page,
                            placement,
                        }
                    })
                    .collect::<Vec<_>>();
                let markers = document
                    .markers()
                    .iter()
                    .filter_map(|marker| {
                        let trace = document
                            .traces()
                            .iter()
                            .find(|trace| trace.id == marker.trace_id)?;
                        let pane = document
                            .panes()
                            .iter()
                            .find(|pane| pane.id == marker.pane_id)?;
                        let binding = pane.binding?;
                        let dataset_id = trace.binding.dataset_id;
                        let analysis_sequence =
                            analysis_sequence_for(dataset_id, binding.analysis_id)?;
                        let TypedValue::Real(x) = &marker.coordinate else {
                            return None;
                        };
                        let x = *x;
                        let waveform = app
                            .state
                            .simulation
                            .runs
                            .iter()
                            .find(|run| run.dataset_id == dataset_id)?
                            .analyses
                            .iter()
                            .find(|analysis| analysis.id == analysis_sequence)?
                            .waveforms
                            .iter()
                            .find(|waveform| waveform.name == trace.label)?;
                        let sample_index = waveform
                            .x
                            .iter()
                            .position(|candidate| candidate.to_bits() == x.to_bits())?;
                        let y = *waveform.y.get(sample_index)?;
                        Some(VisualizationMarker {
                            id: marker.id.get(),
                            dataset_id,
                            analysis_sequence,
                            waveform_name: trace.label.clone(),
                            sample_index,
                            x,
                            y,
                            label: marker.label.clone(),
                        })
                    })
                    .collect::<Vec<_>>();
                let measurements = document
                    .measurements()
                    .iter()
                    .filter_map(|measurement| {
                        let expression = measurement.expression.clone()?;
                        let value = measurement.value?;
                        let trace = measurement.trace_ids.first().and_then(|trace_id| {
                            document.traces().iter().find(|trace| trace.id == *trace_id)
                        })?;
                        let pane = document
                            .panes()
                            .iter()
                            .find(|pane| pane.id == measurement.pane_id)?;
                        let analysis_id = pane.binding?.analysis_id;
                        let dataset_id = trace.binding.dataset_id;
                        let analysis_sequence = analysis_sequence_for(dataset_id, analysis_id)?;
                        Some(VisualizationMeasurement {
                            id: measurement.id.get(),
                            dataset_id,
                            analysis_sequence,
                            expression,
                            value,
                        })
                    })
                    .collect::<Vec<_>>();
                let annotations = document
                    .annotations()
                    .iter()
                    .filter_map(|annotation| {
                        let crate::results::visualization_document::AnnotationAnchor::Trace {
                            trace_id,
                            coordinate: TypedValue::Real(x),
                        } = &annotation.anchor
                        else {
                            return None;
                        };
                        let trace_id = *trace_id;
                        let x = *x;
                        let trace = document
                            .traces()
                            .iter()
                            .find(|trace| trace.id == trace_id)?;
                        let pane = document
                            .panes()
                            .iter()
                            .find(|pane| pane.id == annotation.pane_id)?;
                        let analysis_id = pane.binding?.analysis_id;
                        let dataset_id = trace.binding.dataset_id;
                        let analysis_sequence = analysis_sequence_for(dataset_id, analysis_id)?;
                        Some(VisualizationAnnotation {
                            id: annotation.id.get(),
                            dataset_id,
                            analysis_sequence,
                            x,
                            text: annotation.text.clone(),
                        })
                    })
                    .collect::<Vec<_>>();
                let family_policies = document
                    .panes()
                    .iter()
                    .filter_map(|pane| {
                        pane.family_policy
                            .clone()
                            .map(|policy| (pane.id.get(), policy))
                    })
                    .collect::<BTreeMap<_, _>>();
                let report_page_policies = document
                    .pages()
                    .iter()
                    .map(|page| {
                        (
                            page.title.clone(),
                            VisualizationReportPagePolicy {
                                template: page.template_id.clone(),
                                update_policy: page.update_policy,
                                revision: document.revision().get(),
                            },
                        )
                    })
                    .collect::<BTreeMap<_, _>>();
                let cursor_pairs = document
                    .panes()
                    .iter()
                    .map(|pane| {
                        let mut pair = (None, None);
                        for cursor in document
                            .cursors()
                            .iter()
                            .filter(|cursor| cursor.pane_id == pane.id)
                        {
                            let TypedValue::Real(position) = &cursor.position else {
                                continue;
                            };
                            match cursor.label.as_str() {
                                "A" => pair.0 = Some(*position),
                                "B" => pair.1 = Some(*position),
                                _ => {}
                            }
                        }
                        (pane.id.get(), pair)
                    })
                    .collect::<BTreeMap<_, _>>();
                let cursors_linked = document
                    .link_groups()
                    .iter()
                    .any(|group| group.kind == LinkKind::CursorPosition);
                (
                    panes,
                    markers,
                    measurements,
                    annotations,
                    family_policies,
                    report_page_policies,
                    document.comparisons().to_vec(),
                    document.revision().get(),
                    document.presentation(),
                    cursor_pairs,
                    cursors_linked,
                )
            });
        if let Some((
            panes,
            markers,
            measurements,
            annotations,
            family_policies,
            report_page_policies,
            comparison_receipts,
            revision,
            presentation,
            cursor_pairs,
            cursors_linked,
        )) = projected
        {
            let studio = &mut app.state.workbench.visualization_studio;
            let previous_active = studio.active_pane;
            studio.panes = panes;
            studio.markers = markers;
            studio.measurements = measurements;
            studio.annotations = annotations;
            studio.family_policies = family_policies;
            studio.report_page_policies = report_page_policies;
            studio.comparison_receipts = comparison_receipts;
            studio.active_pane = previous_active
                .filter(|active| studio.panes.iter().any(|pane| pane.id == *active))
                .or_else(|| studio.panes.first().map(|pane| pane.id));
            studio.next_identity = studio
                .panes
                .iter()
                .map(|pane| pane.id)
                .max()
                .unwrap_or_default()
                .saturating_add(1);
            studio.revision = revision;
            studio.significant_digits = presentation.significant_digits;
            if studio.pane_cursor_positions != cursor_pairs {
                studio.pane_cursor_positions = cursor_pairs;
                studio.applied_link_pane = None;
            }
            studio.linked_cursor_positions.clear();
            app.state.ui.results.session.phase_continuous = presentation.phase_continuous;
            app.state.ui.results.session.linked_cursors = cursors_linked;
        }
        if let Some(active_pane_id) = app.state.workbench.visualization_studio.active_pane {
            let visibility = app
                .state
                .workspace
                .content
                .visualization_document(document_id)
                .map(|document| {
                    document
                        .traces()
                        .iter()
                        .filter(|trace| trace.pane_id.get() == active_pane_id)
                        .map(|trace| (trace.label.clone(), trace.visible))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let projection = app
                .state
                .workbench
                .visualization_studio
                .panes
                .iter()
                .find(|pane| pane.id == active_pane_id)
                .cloned()
                .and_then(|pane| {
                    let run = app
                        .state
                        .simulation
                        .runs
                        .iter()
                        .find(|run| run.dataset_id == pane.dataset_id)?;
                    let analysis = run
                        .analyses
                        .iter()
                        .find(|analysis| analysis.id == pane.analysis_sequence)?;
                    let analysis_key =
                        result_document::AnalysisPresentationKey::new(run.dataset_id, analysis);
                    let traces = analysis
                        .waveforms
                        .iter()
                        .filter_map(|waveform| {
                            visibility
                                .iter()
                                .find(|(label, _)| label == &waveform.name)
                                .map(|(_, visible)| {
                                    (waveform.name.clone(), waveform.visible, *visible)
                                })
                        })
                        .collect::<Vec<_>>();
                    (!traces.is_empty()).then_some((analysis_key, traces))
                });
            if let Some((analysis_key, traces)) = projection {
                app.state
                    .ui
                    .results
                    .project_waveform_visibility(analysis_key, traces);
            }
        }
    }
    let studio = &mut app.state.workbench.visualization_studio;
    studio.normalize();
    if studio.panes.is_empty()
        && let Some(dataset_id) = active_dataset
        && let Some(id) = studio.allocate_identity()
    {
        studio.panes.push(VisualizationPane {
            id,
            viewer,
            viewer_document_id: viewer_document_id.clone(),
            dataset_id,
            analysis_sequence: active_analysis_sequence.unwrap_or_default(),
            x_link: Some(1),
            cursor_group: Some(1),
            page: "Engineering".to_owned(),
            placement: VisualizationPanePlacement::BelowSelected,
        });
        studio.active_pane = Some(id);
        studio.selected_viewer_document = viewer_document_id;
    }
    if let Some(pane) = studio.active_pane_mut() {
        app.state.ui.results.session.viewer = pane.viewer;
    }
    normalize_fit_policy_for_renderer(&mut studio.autoscale, app.state.ui.results.session.viewer);
    let binding = studio
        .active_pane
        .and_then(|id| studio.panes.iter().find(|pane| pane.id == id))
        .map(|pane| (pane.dataset_id, pane.analysis_sequence));
    if let Some((dataset_id, _)) = binding
        && app
            .state
            .simulation
            .active_run()
            .is_none_or(|run| run.dataset_id != dataset_id)
        && let Some(run_index) = app
            .state
            .simulation
            .runs
            .iter()
            .position(|run| run.dataset_id == dataset_id)
    {
        let _ = app.state.simulation.select_run(run_index);
    }
    if let Some((dataset_id, analysis_sequence)) = binding
        && let Some(run) = app.state.simulation.active_run()
        && run.dataset_id == dataset_id
        && app
            .state
            .simulation
            .active_analysis()
            .is_none_or(|analysis| analysis.id != analysis_sequence)
        && let Some(analysis_index) = run
            .analyses
            .iter()
            .position(|analysis| analysis.id == analysis_sequence)
    {
        let _ = app.state.simulation.select_analysis(analysis_index);
    }
    apply_active_link_state(app);
}

fn apply_active_link_state(app: &mut RSpiceApp) {
    let Some(pane) = app
        .state
        .workbench
        .visualization_studio
        .active_pane()
        .cloned()
    else {
        return;
    };
    if app.state.workbench.visualization_studio.applied_link_pane == Some(pane.id) {
        return;
    }
    let x_range = pane.x_link.and_then(|group| {
        app.state
            .workbench
            .visualization_studio
            .linked_x_ranges
            .get(&group)
            .copied()
    });
    let x_range = x_range.or_else(|| {
        app.state
            .workbench
            .visualization_studio
            .pane_x_ranges
            .get(&pane.id)
            .copied()
    });
    let cursors = pane.cursor_group.and_then(|group| {
        app.state
            .workbench
            .visualization_studio
            .linked_cursor_positions
            .get(&group)
            .copied()
    });
    let cursors = cursors.or_else(|| {
        app.state
            .workbench
            .visualization_studio
            .pane_cursor_positions
            .get(&pane.id)
            .copied()
    });
    if let Some(x_range) = x_range {
        result_document::request_view_gesture(
            &mut app.state,
            rspice_results_ui::session::ViewGesture::SetRanges {
                x: Some(x_range),
                y: None,
            },
        );
    }
    if let Some((a, b)) = cursors {
        app.state.ui.results.session.cursors.a = a;
        app.state.ui.results.session.cursors.b = b;
    } else {
        app.state.ui.results.session.cursors.clear();
    }
    app.state.workbench.visualization_studio.applied_link_pane = Some(pane.id);
}

fn capture_active_link_state(ctx: &egui::Context, app: &mut RSpiceApp) {
    let Some(pane) = app
        .state
        .workbench
        .visualization_studio
        .active_pane()
        .cloned()
    else {
        return;
    };
    let x_range = result_document::active_renderer_axis_range(
        ctx,
        &mut app.state,
        rspice_results_ui::session::PaneAxis::X,
    );
    let requested_cursors = (
        app.state.ui.results.session.cursors.a,
        app.state.ui.results.session.cursors.b,
    );
    commit_active_project_cursor_pair(app, pane.id, requested_cursors);
    let cursors = (
        app.state.ui.results.session.cursors.a,
        app.state.ui.results.session.cursors.b,
    );
    let studio = &mut app.state.workbench.visualization_studio;
    if let Some(x_range) = x_range {
        studio.pane_x_ranges.insert(pane.id, x_range);
        if let Some(group) = pane.x_link {
            studio.linked_x_ranges.insert(group, x_range);
        }
    } else {
        studio.pane_x_ranges.remove(&pane.id);
        if let Some(group) = pane.x_link {
            studio.linked_x_ranges.remove(&group);
        }
    }
    studio.pane_cursor_positions.insert(pane.id, cursors);
    if let Some(group) = pane.cursor_group {
        studio.linked_cursor_positions.insert(group, cursors);
    }
}

fn synchronize_runtime_policies(app: &mut RSpiceApp) {
    let studio = &app.state.workbench.visualization_studio;
    let complex_projection = match studio.complex_projection {
        ComplexProjection::MagnitudePhase => 0,
        ComplexProjection::RealImaginary => 1,
    };
    let display_lod = match studio.display_lod {
        DisplayLodPolicy::EnvelopePreserving => 0,
        DisplayLodPolicy::UniformSampling => 1,
        DisplayLodPolicy::ExactVisibleSamples => 2,
    };
    let significant_digits = u32::from(studio.significant_digits);
    let tile_memory_mib = studio.tile_memory_mib;

    if let Err(error) = app
        .state
        .ui
        .preferences
        .set_choice(ChoicePreference::ComplexNumberDisplay, complex_projection)
    {
        app.state.push_user_message(ConsoleMessage::error(error));
    }
    if let Err(error) = app
        .state
        .ui
        .preferences
        .set_choice(ChoicePreference::LargeDatasetDisplay, display_lod)
    {
        app.state.push_user_message(ConsoleMessage::error(error));
    }
    if let Err(error) = app.state.ui.preferences.set_scalar(
        ScalarPreference::DisplayedSignificantDigits,
        significant_digits,
    ) {
        app.state.push_user_message(ConsoleMessage::error(error));
    }
    app.state
        .ui
        .results
        .session
        .cache
        .set_memory_budget_mib(tile_memory_mib);
}

fn resolved_viewer_availability_for_binding(
    state: &AppState,
    definition: &ViewerDocumentDefinition,
    dataset_id: Option<DatasetId>,
    analysis_sequence: Option<u64>,
) -> Result<ResultViewer, String> {
    let dataset_id = dataset_id.ok_or_else(|| "Select a retained dataset".to_owned())?;
    let analysis_sequence =
        analysis_sequence.ok_or_else(|| "Select a retained analysis".to_owned())?;
    let run = state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == dataset_id)
        .ok_or_else(|| "The selected dataset is no longer retained".to_owned())?;
    let analysis_index = run
        .analyses
        .iter()
        .position(|analysis| analysis.id == analysis_sequence)
        .ok_or_else(|| "The selected analysis is no longer retained".to_owned())?;
    let analysis = &run.analyses[analysis_index];
    let analysis_ids = [analysis_manifest_id(analysis.analysis_type)];
    match viewer_compatibility(
        definition.id,
        ViewerCapabilities {
            analysis_ids: &analysis_ids,
            external_capabilities: &[],
        },
    ) {
        ViewerCompatibility::Compatible => {}
        ViewerCompatibility::MissingAnalysis {
            accepted_analysis_ids,
        } => {
            return Err(format!(
                "Requires {} analysis data",
                accepted_analysis_ids.join(" / ")
            ));
        }
        ViewerCompatibility::MissingExternalCapability { capability_id } => {
            return Err(format!("Requires {capability_id} result capability"));
        }
        ViewerCompatibility::UnknownDocument => {
            return Err("Viewer identity is not registered".to_owned());
        }
    }
    let viewer = ResultViewer::from_viewer_document_id(definition.id)
        .ok_or_else(|| "No exact Rust renderer is registered for this viewer".to_owned())?;
    let viewer = result_document::project_viewer_for_analysis(viewer, analysis);
    let binding_is_active = state
        .simulation
        .active_run()
        .is_some_and(|active| active.dataset_id == dataset_id)
        && state
            .simulation
            .active_analysis()
            .is_some_and(|active| active.id == analysis_sequence);
    let available = match viewer {
        ResultViewer::Waves | ResultViewer::DcSweep => !analysis.waveforms.is_empty(),
        ResultViewer::Bode => result_document::bode_analysis_is_renderable(analysis),
        ResultViewer::Fft | ResultViewer::Eye => {
            crate::simulation::SimulationController::analysis_supports_transient_derivation(
                analysis.analysis_type,
            ) && !analysis.waveforms.is_empty()
        }
        ResultViewer::HarmonicBalance => {
            result_document::harmonic_balance_analysis_is_renderable(analysis)
        }
        ResultViewer::PhaseNoise => result_document::phase_noise_analysis_is_renderable(analysis),
        ResultViewer::Specs => {
            !analysis.measurements.is_empty() || !state.workspace.content.specs.is_empty()
        }
        // The table lists retained samples and payload-only periodic spectra,
        // including zero-dynamic-mode results.
        ResultViewer::Table => {
            !analysis.waveforms.is_empty()
                || matches!(
                    analysis.result_payload,
                    Some(
                        AnalysisResultPayload::PssFloquet { .. }
                            | AnalysisResultPayload::Pstb { .. }
                    )
                )
        }
        ResultViewer::PoleZero => retained_pole_zero_payload(analysis).is_some(),
        ResultViewer::Contribution => {
            retained_sensitivity_study(analysis).is_some()
                || retained_sensitivity_payload(analysis).is_some()
        }
        ResultViewer::TransferFunction => analysis.result_payload.as_ref().is_some_and(|payload| {
            matches!(payload, AnalysisResultPayload::TransferFunction { .. })
                && payload.validate_for(analysis.analysis_type).is_ok()
        }),
        ResultViewer::Smith => result_document::smith_analysis_is_renderable(analysis),
        ResultViewer::NetworkMatrix => result_document::view_context::analysis_supports_viewer(viewer, analysis),
        ResultViewer::Hist
        | ResultViewer::Op
        | ResultViewer::NoiseContrib
        | ResultViewer::Nyquist
        // These three read the active selection's own retained evidence
        // through state-aware gates — the population memo and the workspace's
        // requirements — exactly as the histogram does.
        | ResultViewer::Polar
        | ResultViewer::Scatter
        | ResultViewer::BoxViolin => {
            binding_is_active && result_document::viewer_is_available(state, viewer)
        }
        // Dataset-native Results projections, which therefore can never be
        // resolved from a Visualization Studio document definition.
        ResultViewer::Manifest | ResultViewer::Events | ResultViewer::Soa | ResultViewer::Optimization => false,
    };
    if !available {
        return Err(if binding_is_active {
            result_document::viewer_unavailability_reason(state, viewer)
                .unwrap_or("The selected analysis does not satisfy this renderer contract")
                .to_owned()
        } else {
            "This renderer requires derived state owned by the currently active analysis".to_owned()
        });
    }
    Ok(viewer)
}

fn active_studio_exact_export_available(state: &AppState) -> bool {
    let Some(pane) = state.workbench.visualization_studio.active_pane() else {
        return false;
    };
    let Some(run) = state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == pane.dataset_id)
    else {
        return false;
    };
    if run.lifecycle != crate::state::SimulationRunLifecycle::Completed {
        return false;
    }
    let Some(analysis) = run
        .analyses
        .iter()
        .find(|analysis| analysis.id == pane.analysis_sequence)
    else {
        return false;
    };
    if !analysis.success {
        return false;
    }
    pane.viewer
        .viewer_document_id()
        .and_then(viewer_document)
        .is_some_and(|definition| {
            resolved_viewer_availability_for_binding(
                state,
                definition,
                Some(pane.dataset_id),
                Some(pane.analysis_sequence),
            )
            .is_ok()
        })
}

fn active_studio_figure_export_available(state: &AppState) -> bool {
    active_studio_exact_export_available(state)
        && crate::workbench::hardcopy_adapters::sources::active_app_hardcopy_source_available(state)
}

fn available_analysis_ids(state: &AppState) -> Vec<&'static str> {
    state
        .simulation
        .active_analysis()
        .map(|analysis| vec![analysis_manifest_id(analysis.analysis_type)])
        .unwrap_or_default()
}

const fn analysis_manifest_id(analysis: crate::state::AnalysisType) -> &'static str {
    use crate::state::AnalysisType;
    match analysis {
        AnalysisType::DcOp => "op",
        AnalysisType::DcSweep | AnalysisType::Parametric => "dc",
        AnalysisType::Ac => "ac",
        AnalysisType::Disto => "disto",
        AnalysisType::Transient => "tran",
        AnalysisType::Noise => "noise",
        AnalysisType::PoleZero => "pz",
        AnalysisType::Tf => "xf",
        AnalysisType::Sensitivity => "sens",
        AnalysisType::Pac => "pac",
        AnalysisType::Pnoise => "pnoise",
        AnalysisType::Pxf => "pxf",
        AnalysisType::Pstb => "pstb",
        AnalysisType::Stb => "stb",
        AnalysisType::MonteCarlo => "mc",
        AnalysisType::Corner => "corner",
        AnalysisType::Optimization => "opt",
        AnalysisType::Soa => "soa",
        AnalysisType::SParameter => "sp",
        AnalysisType::Envelope => "envelope",
        AnalysisType::Fourier => "fourier",
        AnalysisType::HarmonicBalance => "hb",
        AnalysisType::Pss => "pss",
        AnalysisType::Qpss => "qpss",
        AnalysisType::Hbsp => "hbsp",
        AnalysisType::Hbnoise => "hbnoise",
        AnalysisType::Psp => "psp",
        AnalysisType::Qpac => "qpac",
        AnalysisType::Qpnoise => "qpnoise",
        AnalysisType::Qpxf => "qpxf",
        AnalysisType::TransientNoise => "tnoise",
        AnalysisType::DcMismatch => "dcmatch",
    }
}

fn add_viewer_pane(app: &mut RSpiceApp, document_id: &str, viewer: ResultViewer) {
    let Some(dataset_id) = app.state.simulation.active_run().map(|run| run.dataset_id) else {
        app.state.push_user_message(ConsoleMessage::warning(
            "A visualization pane requires an active immutable result dataset.",
        ));
        return;
    };
    let Some(analysis_sequence) = app
        .state
        .simulation
        .active_analysis()
        .map(|analysis| analysis.id)
    else {
        app.state.push_user_message(ConsoleMessage::warning(
            "A visualization pane requires a selected retained analysis.",
        ));
        return;
    };
    add_viewer_pane_bound(
        app,
        document_id,
        viewer,
        dataset_id,
        analysis_sequence,
        VisualizationPanePlacement::BelowSelected,
        String::new(),
    );
}

fn add_viewer_pane_bound(
    app: &mut RSpiceApp,
    document_id: &str,
    viewer: ResultViewer,
    dataset_id: DatasetId,
    analysis_sequence: u64,
    placement: VisualizationPanePlacement,
    requested_page_title: String,
) {
    let binding = app
        .state
        .simulation
        .runs
        .iter()
        .enumerate()
        .find_map(|(run_index, run)| {
            (run.dataset_id == dataset_id).then(|| {
                run.analyses
                    .iter()
                    .position(|analysis| analysis.id == analysis_sequence)
                    .map(|analysis_index| (run_index, analysis_index))
            })?
        });
    let Some((run_index, analysis_index)) = binding else {
        app.state.push_user_message(ConsoleMessage::warning(
            "The selected immutable dataset or analysis is no longer retained.",
        ));
        return;
    };
    let Some(definition) = viewer_document(document_id) else {
        app.state.push_user_message(ConsoleMessage::error(
            "The selected visualization viewer is not registered.",
        ));
        return;
    };
    if let Err(error) = resolved_viewer_availability_for_binding(
        &app.state,
        definition,
        Some(dataset_id),
        Some(analysis_sequence),
    ) {
        app.state.push_user_message(ConsoleMessage::warning(error));
        return;
    }
    let active_pane = app
        .state
        .workbench
        .visualization_studio
        .active_pane()
        .cloned();
    let page = if placement == VisualizationPanePlacement::NewWorksheetPage {
        requested_page_title.trim().to_owned()
    } else {
        active_pane
            .as_ref()
            .map_or_else(|| "Engineering".to_owned(), |pane| pane.page.clone())
    };
    if page.is_empty() {
        app.state.push_user_message(ConsoleMessage::warning(
            "A new worksheet page requires a non-blank title.",
        ));
        return;
    }
    if active_project_visualization_document_id(&app.state).is_some() {
        let analysis_id = {
            let run = &app.state.simulation.runs[run_index];
            let analysis = &run.analyses[analysis_index];
            analysis.provenance().map_or_else(
                || {
                    let name = format!("legacy-analysis-v1/{}", analysis.id);
                    AnalysisInstanceId::from_namespace(run.dataset_id.as_uuid(), name.as_bytes())
                },
                |provenance| provenance.source_instance_id(),
            )
        };
        let dataset_binding = {
            let run = &app.state.simulation.runs[run_index];
            DatasetBinding::new(run.dataset_id, run.dataset_content_digest())
        };
        let binding = crate::results::visualization_document::PaneDataBinding {
            analysis_id,
            dataset: dataset_binding,
        };
        let existing_dataset = active_project_visualization_document_id(&app.state)
            .and_then(|document_id| {
                app.state
                    .workspace
                    .content
                    .visualization_document(document_id)
            })
            .is_some_and(|document| {
                document
                    .datasets()
                    .iter()
                    .any(|dataset| dataset.binding() == dataset_binding)
            });
        let source = {
            let run = &app.state.simulation.runs[run_index];
            let analysis = &run.analyses[analysis_index];
            match result_document::visualization_source_dataset(run, analysis) {
                Ok(source) => source,
                Err(error) => {
                    app.state.push_user_message(ConsoleMessage::error(error));
                    return;
                }
            }
        };
        let mut edits = vec![if existing_dataset {
            DocumentEdit::MergeDatasetProjection(source)
        } else {
            DocumentEdit::AttachDataset(source)
        }];
        if placement == VisualizationPanePlacement::NewWorksheetPage {
            edits.push(DocumentEdit::AddPaneOnNewPage {
                page: crate::results::visualization_document::NewPage {
                    title: page,
                    layout: crate::results::visualization_document::PageLayout::Rows,
                    template_id: "engineering-dark".to_owned(),
                    update_policy: crate::results::visualization_document::PageUpdatePolicy::RefreshLinkedFigures,
                },
                pane: crate::results::visualization_document::NewPagePane {
                    title: definition.title.to_owned(),
                    kind: document_pane_kind(definition.art),
                    viewer_id: document_id.to_owned(),
                    binding: Some(binding),
                },
            });
        } else {
            let Some(active) = active_pane.as_ref() else {
                app.state.push_user_message(ConsoleMessage::warning(
                    "Select a result-document pane before inserting another pane.",
                ));
                return;
            };
            let anchor = app
                .state
                .workspace
                .content
                .visualization_document(
                    active_project_visualization_document_id(&app.state)
                        .expect("canonical branch has active document"),
                )
                .and_then(|document| {
                    document
                        .panes()
                        .iter()
                        .find(|pane| pane.id.get() == active.id)
                        .map(|pane| pane.id)
                });
            let Some(anchor) = anchor else {
                app.state.push_user_message(ConsoleMessage::error(
                    "The selected project result pane no longer exists.",
                ));
                return;
            };
            let placement = match placement {
                VisualizationPanePlacement::RightOfSelected => {
                    crate::results::visualization_document::PanePlacement::RightOf {
                        anchor_pane_id: anchor,
                    }
                }
                VisualizationPanePlacement::BelowSelected => {
                    crate::results::visualization_document::PanePlacement::Below {
                        anchor_pane_id: anchor,
                    }
                }
                VisualizationPanePlacement::NewWorksheetPage => unreachable!(),
            };
            let page_id = app
                .state
                .workspace
                .content
                .visualization_document(
                    active_project_visualization_document_id(&app.state)
                        .expect("canonical branch has active document"),
                )
                .and_then(|document| {
                    document
                        .panes()
                        .iter()
                        .find(|pane| pane.id.get() == active.id)
                        .map(|pane| pane.page_id)
                })
                .expect("resolved active pane owns a page");
            edits.push(DocumentEdit::AddBoundPane(
                crate::results::visualization_document::NewPane {
                    page_id,
                    title: definition.title.to_owned(),
                    kind: document_pane_kind(definition.art),
                    viewer_id: document_id.to_owned(),
                    binding: Some(binding),
                    placement,
                },
            ));
        }
        match transact_active_project_document(app, edits) {
            Ok(receipt) => {
                if let Some(pane_id) = receipt.created.iter().find_map(|entity| match entity {
                    crate::results::visualization_document::EntityRef::Pane(id) => Some(id.get()),
                    _ => None,
                }) {
                    app.state.workbench.visualization_studio.active_pane = Some(pane_id);
                }
                let _ = app.state.simulation.select_run(run_index);
                let _ = app.state.simulation.select_analysis(analysis_index);
                app.state.ui.results.session.viewer = viewer;
                reconcile_document(app);
            }
            Err(error) => app.state.push_user_message(ConsoleMessage::error(error)),
        }
        return;
    }
    let x_link = if placement == VisualizationPanePlacement::NewWorksheetPage {
        None
    } else {
        active_pane.as_ref().and_then(|pane| pane.x_link)
    };
    let cursor_group = if placement == VisualizationPanePlacement::NewWorksheetPage {
        None
    } else {
        active_pane.as_ref().and_then(|pane| pane.cursor_group)
    };
    let studio = &mut app.state.workbench.visualization_studio;
    let document_id = document_id.to_owned();
    let result = studio.transact(|studio| {
        let id = studio
            .allocate_identity()
            .ok_or_else(|| "Visualization pane identity space is exhausted".to_owned())?;
        let insertion_index = if placement == VisualizationPanePlacement::NewWorksheetPage {
            studio.panes.len()
        } else {
            studio
                .active_pane
                .and_then(|active| studio.panes.iter().position(|pane| pane.id == active))
                .map_or(studio.panes.len(), |index| index + 1)
        };
        studio.panes.insert(
            insertion_index,
            VisualizationPane {
                id,
                viewer,
                viewer_document_id: document_id.clone(),
                dataset_id,
                analysis_sequence,
                x_link,
                cursor_group,
                page,
                placement,
            },
        );
        studio.active_pane = Some(id);
        studio.selected_viewer_document = document_id;
        studio.applied_link_pane = None;
        Ok(id)
    });
    match result {
        Ok(_) => {
            let _ = app.state.simulation.select_run(run_index);
            let _ = app.state.simulation.select_analysis(analysis_index);
            app.state.ui.results.session.viewer = viewer;
        }
        Err(error) => app.state.push_user_message(ConsoleMessage::error(error)),
    }
}
#[cfg(test)]
mod integrity_scan_tests;
