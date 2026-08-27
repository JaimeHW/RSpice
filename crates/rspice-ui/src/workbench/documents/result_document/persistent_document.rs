//! Project-owned Results document projection.
//!
//! This module never invents a runtime document. It resolves each retained
//! page and pane directly from [`VisualizationDocument`], verifies the pane's
//! immutable dataset digest and analysis identity, and then delegates only to
//! an existing renderer with the same canonical viewer contract. A document
//! can only be created with a registered renderer; migrated or damaged
//! documents carrying an unknown viewer identity fail closed.

use egui::{Align, Layout, Rect, RichText, Sense, Ui, UiBuilder, pos2, vec2};

use crate::product::{
    AnalysisInstanceId, ContentDigest, ObjectRevision, ResultDocumentId, SimulationPlanId,
};
use crate::results::viewer_catalog::{ViewerReleaseClass, viewer_document};
use crate::results::visualization_document::{
    Annotation, Axis, AxisOrientation, AxisRange, Cursor, DocumentEdit, EntityRef, Marker,
    Measurement, Page, PageLayout, Pane, PaneDataBinding, PanePlacement, Trace, TypedValue,
};
use crate::state::{AnalysisResult, AnalysisType, SimulationRun};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::{AppState, RSpiceApp};

use super::ResultViewer;

#[derive(Debug, Clone)]
struct DocumentProjection {
    pages: Vec<PageProjection>,
}

#[derive(Debug, Clone)]
struct PageProjection {
    page: Page,
    panes: Vec<PaneProjection>,
}

#[derive(Debug, Clone)]
struct PaneProjection {
    document_id: ResultDocumentId,
    pane: Pane,
    axes: Vec<Axis>,
    traces: Vec<Trace>,
    cursors: Vec<Cursor>,
    markers: Vec<Marker>,
    measurements: Vec<Measurement>,
    annotations: Vec<Annotation>,
}

impl std::ops::Deref for PaneProjection {
    type Target = Pane;

    fn deref(&self) -> &Self::Target {
        &self.pane
    }
}

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp, document_id: ResultDocumentId) {
    // A document that cannot advance onto the newest run is still a document
    // about a real dataset. Blanking it withheld the evidence the reader
    // already had over a decision about which evidence to show next.
    match refresh_latest_binding(&mut app.state, document_id) {
        LatestBinding::Current => {}
        LatestBinding::Degraded(reason) => tracking_banner(ui, &reason),
        LatestBinding::Missing(reason) => {
            unavailable_surface(ui, "Result document unavailable", &reason);
            return;
        }
    }
    let Some(document) = projection(&app.state, document_id) else {
        unavailable_surface(
            ui,
            "Result document unavailable",
            "The selected project result document no longer exists.",
        );
        return;
    };
    let Some(first_page) = document.pages.first() else {
        unavailable_surface(
            ui,
            "Result document unavailable",
            "The document contains no renderable page.",
        );
        return;
    };
    let selected_page_id = app
        .state
        .ui
        .results
        .persistent_document_page(document_id)
        .filter(|selected| document.pages.iter().any(|page| page.page.id == *selected))
        .unwrap_or(first_page.page.id);

    app.state
        .ui
        .results
        .select_persistent_document_page(document_id, selected_page_id);
    let Some(page) = document
        .pages
        .iter()
        .find(|page| page.page.id == selected_page_id)
    else {
        unavailable_surface(
            ui,
            "Result document unavailable",
            "The selected page no longer exists in this result document.",
        );
        return;
    };

    if page.panes.is_empty() {
        unavailable_surface(
            ui,
            &page.page.title,
            "The selected page contains no result panes.",
        );
        return;
    }

    let requested_active_pane = app.state.workbench.visualization_studio.active_pane;
    let active_pane_id = resolved_active_pane_id(
        requested_active_pane,
        page.panes.iter().map(|pane| pane.id.get()),
    )
    .expect("a non-empty persistent page always resolves an active pane");
    app.state.workbench.visualization_studio.active_pane = Some(active_pane_id);
    let active_pane = page
        .panes
        .iter()
        .find(|pane| pane.id.get() == active_pane_id)
        .expect("the resolved active pane belongs to the selected page");
    if let Err(reason) = select_pane_binding(&mut app.state, active_pane) {
        unavailable_surface(ui, &active_pane.title, &reason);
        return;
    }
    super::prepare_viewer_state(app);
    if let Some(viewer) = compatible_active_viewer(&app.state, &page.page.title, active_pane) {
        select_global_viewer(&mut app.state, viewer);
    }
    super::show_persistent_docbar(ui, app, &page.page.title);
    let active_viewer = compatible_active_viewer(&app.state, &page.page.title, active_pane);
    if let Some(viewer) = active_viewer {
        select_global_viewer(&mut app.state, viewer);
        if let Err(reason) = project_pane_presentation(&mut app.state, active_pane, viewer) {
            unavailable_surface(ui, &active_pane.title, &reason);
            return;
        }
        if super::viewer_has_sheet_bar(viewer) {
            super::show_sheet_bar(ui, &mut app.state);
        }
        crate::ui::plot::set_interaction_mode(
            ui.ctx(),
            app.state.ui.results.plot_tool.interaction_mode(),
        );
    }

    // Persistent pages own exactly one stage-level readout, just like quick
    // Results. Individual panes never reserve their own instrument chrome.
    let strip_height = active_viewer.map_or(0.0, |_| super::readout_strip_height(&app.state));
    let available = ui.available_rect_before_wrap();
    let well_height = (available.height() - strip_height).max(0.0);
    let activated_pane_id = ui
        .allocate_ui(vec2(available.width(), well_height), |ui| {
            ui.set_min_height(well_height);
            if active_viewer == Some(ResultViewer::Manifest) {
                // Manifest is a dataset-native document, so it replaces the
                // page projection instead of masquerading as one retained
                // visualization pane.
                super::show_persistent_pane_viewer(ui, app, ResultViewer::Manifest);
                None
            } else {
                render_page(ui, app, page, active_pane_id, active_viewer)
            }
        })
        .inner;

    // Painting inactive panes temporarily selects their immutable bindings and
    // retained renderers. Restore the actual active pane so the shared readout,
    // navigator, inspector, and commands all observe one exact context.
    let restored_pane_id = activated_pane_id.unwrap_or(active_pane_id);
    let restored_pane = page
        .panes
        .iter()
        .find(|pane| pane.id.get() == restored_pane_id)
        .expect("a pane activated from this page remains in this page");
    if select_pane_binding(&mut app.state, restored_pane).is_ok() {
        app.state.workbench.visualization_studio.active_pane = Some(restored_pane_id);
        let restored_viewer = if activated_pane_id.is_some() {
            ResultViewer::from_viewer_document_id(&restored_pane.viewer_id)
        } else {
            active_viewer
        };
        if let Some(viewer) = restored_viewer {
            app.state.ui.results.viewer = viewer;
            if let Some(viewer_document_id) = viewer.viewer_document_id() {
                app.state
                    .workbench
                    .visualization_studio
                    .selected_viewer_document = if activated_pane_id.is_some() {
                    restored_pane.viewer_id.clone()
                } else {
                    viewer_document_id.to_owned()
                };
            }
        }
    }
    if let Some(fresh_pane) = projection(&app.state, document_id).and_then(|projection| {
        projection
            .pages
            .into_iter()
            .flat_map(|page| page.panes)
            .find(|pane| pane.id.get() == restored_pane_id)
    }) && let Some(viewer) = ResultViewer::from_viewer_document_id(&fresh_pane.viewer_id)
    {
        let viewer = bound_viewer_projection(&app.state, viewer);
        let _ = project_pane_presentation(&mut app.state, &fresh_pane, viewer);
    }
    if strip_height > 0.0 {
        super::waves::readout_strip(ui, &mut app.state, strip_height);
    }
}

fn resolved_active_pane_id(
    requested: Option<u64>,
    pane_ids: impl IntoIterator<Item = u64>,
) -> Option<u64> {
    let mut first = None;
    for pane_id in pane_ids {
        first.get_or_insert(pane_id);
        if requested == Some(pane_id) {
            return Some(pane_id);
        }
    }
    first
}

fn compatible_active_viewer(
    state: &AppState,
    family_label: &str,
    pane: &Pane,
) -> Option<ResultViewer> {
    let selected = state.ui.results.viewer;
    if super::family_allows_viewer(family_label, selected)
        && super::viewer_is_available(state, selected)
    {
        return Some(selected);
    }
    ResultViewer::from_viewer_document_id(&pane.viewer_id).filter(|viewer| {
        super::family_allows_viewer(family_label, *viewer)
            && super::viewer_is_available(state, *viewer)
    })
}

fn select_global_viewer(state: &mut AppState, viewer: ResultViewer) {
    state.ui.results.viewer = viewer;
    if let Some(viewer_document_id) = viewer.viewer_document_id() {
        state
            .workbench
            .visualization_studio
            .selected_viewer_document = viewer_document_id.to_owned();
    }
}

fn current_result_source_digest(state: &AppState) -> Option<ContentDigest> {
    let netlist = &state.ui.netlist;
    (netlist.generation_error.is_none()
        && netlist.generated_input_digest.is_some()
        && netlist.generated_input_digest == netlist.current_generation_input_digest
        && !state.simulation.netlist_content.trim().is_empty())
    .then(|| {
        crate::workbench::documents::netlist_document::source_content_digest(
            &state.simulation.netlist_content,
        )
    })
}

fn run_matches_current_authority(
    run: &SimulationRun,
    plan_id: SimulationPlanId,
    project_revision: ObjectRevision,
    source_digest: Option<ContentDigest>,
) -> bool {
    run.lifecycle == crate::state::SimulationRunLifecycle::Completed
        && run.success
        && run.prepared_receipt().is_some_and(|receipt| {
            receipt.simulation_plan_id() == Some(plan_id)
                && receipt.project_revision() == project_revision
                && source_digest.is_some_and(|digest| receipt.source_content_digest() == digest)
        })
}

fn latest_successful_authored_analysis(
    run: &SimulationRun,
    authored_analysis_id: AnalysisInstanceId,
) -> Option<&AnalysisResult> {
    run.find_analysis_by_source_instance(authored_analysis_id)
        .filter(|analysis| analysis.success)
}

/// What Latest tracking could do for one document on this frame.
enum LatestBinding {
    /// Nothing to report: pinned, already current, or freshly retargeted.
    Current,
    /// The document keeps the last binding that worked, and the reader is
    /// told why it is not advancing.
    Degraded(String),
    /// The document itself is gone, so there is nothing to render.
    Missing(String),
}

/// The banner text for a retarget the document could not complete.
///
/// It names the cause and the control that settles it, because the reader's
/// two useful answers are "fix the design so the run resolves again" and "pin
/// this document to the dataset it was built on".
fn degraded_tracking_reason(cause: &str) -> String {
    format!(
        "{cause} Showing the dataset this document last resolved. \
         Use the Latest / Pinned control in the toolbar to pin it here."
    )
}

fn refresh_latest_binding(state: &mut AppState, document_id: ResultDocumentId) -> LatestBinding {
    use crate::results::visualization_document::{DocumentEdit, ResultDocumentTrackingMode};

    let Some(document) = state.workspace.visualization_document(document_id) else {
        return LatestBinding::Missing(
            "The selected project result document no longer exists.".to_owned(),
        );
    };
    let tracking = document.tracking();
    if tracking.mode != ResultDocumentTrackingMode::Latest {
        state.ui.results.clear_latest_retarget_failure(document_id);
        return LatestBinding::Current;
    }
    // A tracking claim the document cannot substantiate is a reason to stop
    // advancing, not a reason to stop rendering: the immutable binding the
    // document already holds is still exactly what it says it is.
    let (Some(plan_id), Some(authored_analysis_id)) =
        (tracking.simulation_plan_id, tracking.authored_analysis_id)
    else {
        return LatestBinding::Degraded(degraded_tracking_reason(
            "This document tracks the latest run but retains no exact simulation-plan and \
             authored-analysis identity to follow.",
        ));
    };
    let Some(previous) = document
        .panes()
        .iter()
        .filter_map(|pane| pane.binding)
        .next()
        .map(|binding| binding.dataset)
    else {
        return LatestBinding::Degraded(degraded_tracking_reason(
            "This document retains no pane binding for Latest tracking to advance.",
        ));
    };
    let project_revision = state.workspace.project.revision();
    let source_digest = current_result_source_digest(state);

    let Some(candidate) = state
        .simulation
        .runs
        .iter()
        .filter(|run| run_matches_current_authority(run, plan_id, project_revision, source_digest))
        .filter_map(|run| {
            latest_successful_authored_analysis(run, authored_analysis_id)
                .map(|analysis| (run, analysis))
        })
        .max_by(|(left, _), (right, _)| {
            left.id
                .cmp(&right.id)
                .then_with(|| left.timestamp.total_cmp(&right.timestamp))
                .then_with(|| left.dataset_id.as_uuid().cmp(&right.dataset_id.as_uuid()))
        })
        .map(|(run, analysis)| (run.dataset_id, run.dataset_content_digest(), analysis.id))
    else {
        // Latest tracking is non-destructive. When no authenticated current
        // producer exists, keep the last immutable binding readable; the
        // Results toolbar marks it Historical until a matching completed run
        // can advance it again.
        return LatestBinding::Current;
    };
    let (candidate_dataset, candidate_digest, _) = candidate;
    let next = crate::product::DatasetBinding::new(candidate_dataset, candidate_digest);
    if next == previous {
        state.ui.results.clear_latest_retarget_failure(document_id);
        return LatestBinding::Current;
    }
    // Retargeting rebuilds the retained source dataset from the run. Re-trying
    // a retarget that has already failed would pay for that rebuild on every
    // frame and report the same refusal every frame, so a failure is held
    // against the exact candidate that produced it and re-tried only when a
    // different run becomes the candidate.
    if let Some(reason) = state
        .ui
        .results
        .latest_retarget_failure(document_id, candidate_dataset)
    {
        return LatestBinding::Degraded(reason.to_owned());
    }

    let prepared = state
        .simulation
        .runs
        .iter()
        .find(|run| run.dataset_id == candidate_dataset)
        .and_then(|run| {
            latest_successful_authored_analysis(run, authored_analysis_id).map(|analysis| {
                (
                    super::create_document::source_dataset(run, analysis),
                    analysis_identity(run, analysis),
                )
            })
        });
    let Some((source, analysis_id)) = prepared else {
        return LatestBinding::Current;
    };
    let refuse = |state: &mut AppState, cause: String| {
        let reason = degraded_tracking_reason(&cause);
        state.ui.results.record_latest_retarget_failure(
            document_id,
            candidate_dataset,
            reason.clone(),
        );
        LatestBinding::Degraded(reason)
    };
    let next_source = match source {
        Ok(source) => source,
        Err(error) => {
            return refuse(
                state,
                format!(
                    "The newest run of this document's analysis no longer builds its retained source: {error}."
                ),
            );
        }
    };
    let Some(revision) = state
        .workspace
        .visualization_document(document_id)
        .map(|document| document.revision())
    else {
        return LatestBinding::Missing(
            "The selected project result document is no longer retained.".to_owned(),
        );
    };
    if let Err(error) = state.workspace.transact_visualization_document(
        document_id,
        revision,
        vec![DocumentEdit::RetargetTrackedDataset {
            previous,
            next: next_source,
            analysis_id,
        }],
    ) {
        return refuse(
            state,
            format!("This document could not be retargeted onto the newest run: {error}."),
        );
    }
    state.ui.results.clear_latest_retarget_failure(document_id);
    LatestBinding::Current
}

pub(super) fn activate(state: &mut AppState, document_id: ResultDocumentId) -> bool {
    let Some(document) = state.workspace.visualization_document(document_id) else {
        return false;
    };
    let selected_page_id = state
        .ui
        .results
        .persistent_document_page(document_id)
        .filter(|selected| document.pages().iter().any(|page| page.id == *selected))
        .or_else(|| document.pages().first().map(|page| page.id));
    let Some(page) = selected_page_id
        .and_then(|page_id| document.pages().iter().find(|page| page.id == page_id))
    else {
        return false;
    };
    let panes = ordered_page_panes(document.panes(), page);
    let active_pane_id = resolved_active_pane_id(
        state.workbench.visualization_studio.active_pane,
        panes.iter().map(|pane| pane.id.get()),
    );
    let Some(pane) =
        active_pane_id.and_then(|pane_id| panes.into_iter().find(|pane| pane.id.get() == pane_id))
    else {
        return false;
    };
    let pane = pane.clone();
    if select_pane_binding(state, &pane).is_err() {
        return false;
    }
    state.workbench.visualization_studio.active_pane = Some(pane.id.get());
    if let Some(viewer) = ResultViewer::from_viewer_document_id(&pane.viewer_id) {
        state.ui.results.viewer = viewer;
    }
    state
        .workbench
        .visualization_studio
        .selected_viewer_document = pane.viewer_id;
    true
}

fn projection(state: &AppState, document_id: ResultDocumentId) -> Option<DocumentProjection> {
    let document = state.workspace.visualization_document(document_id)?;
    let pages = document
        .pages()
        .iter()
        .cloned()
        .map(|page| PageProjection {
            panes: ordered_page_panes(document.panes(), &page)
                .into_iter()
                .map(|pane| PaneProjection {
                    document_id,
                    pane: pane.clone(),
                    axes: document
                        .axes()
                        .iter()
                        .filter(|axis| axis.pane_id == pane.id)
                        .cloned()
                        .collect(),
                    traces: document
                        .traces()
                        .iter()
                        .filter(|trace| trace.pane_id == pane.id)
                        .cloned()
                        .collect(),
                    cursors: document
                        .cursors()
                        .iter()
                        .filter(|cursor| cursor.pane_id == pane.id)
                        .cloned()
                        .collect(),
                    markers: document
                        .markers()
                        .iter()
                        .filter(|marker| marker.pane_id == pane.id)
                        .cloned()
                        .collect(),
                    measurements: document
                        .measurements()
                        .iter()
                        .filter(|measurement| measurement.pane_id == pane.id)
                        .cloned()
                        .collect(),
                    annotations: document
                        .annotations()
                        .iter()
                        .filter(|annotation| annotation.pane_id == pane.id)
                        .cloned()
                        .collect(),
                })
                .collect(),
            page,
        })
        .collect();
    Some(DocumentProjection { pages })
}

fn ordered_page_panes<'a>(panes: &'a [Pane], page: &Page) -> Vec<&'a Pane> {
    let mut panes = panes
        .iter()
        .filter(|pane| pane.page_id == page.id)
        .collect::<Vec<_>>();
    panes.sort_by_key(|pane| (pane.order, placement_rank(pane.placement), pane.id.get()));
    panes
}

const fn placement_rank(placement: PanePlacement) -> u8 {
    match placement {
        PanePlacement::Primary => 0,
        PanePlacement::Below { .. } => 1,
        PanePlacement::RightOf { .. } => 2,
    }
}

fn render_page(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    page: &PageProjection,
    active_pane_id: u64,
    active_viewer: Option<ResultViewer>,
) -> Option<u64> {
    if page.page.layout == PageLayout::SinglePane && page.panes.len() != 1 {
        unavailable_surface(
            ui,
            &page.page.title,
            "A single-pane page must resolve to exactly one retained pane.",
        );
        return None;
    }

    // Reserve the stage exactly once. Pane viewers render into clipped slots
    // inside that fixed rectangle, so their own minimum sizes and scroll areas
    // cannot enlarge the Results document or create recursive scroll growth.
    let size = finite_stage_size(ui.available_size());
    let (stage_rect, _) = ui.allocate_exact_size(size, Sense::hover());
    let slots = bounded_pane_slots(stage_rect, page.page.layout, page.panes.len());
    let mut activated_pane_id = None;
    for (pane, rect) in page.panes.iter().zip(slots) {
        let mut pane_ui = ui.new_child(
            UiBuilder::new()
                .id_salt(("persistent-result-pane-slot", pane.id.get()))
                .max_rect(rect)
                .layout(Layout::top_down(Align::Min)),
        );
        pane_ui.set_clip_rect(pane_ui.clip_rect().intersect(rect));
        let activated = render_pane(&mut pane_ui, app, pane, active_pane_id, active_viewer);
        activated_pane_id = activated.or(activated_pane_id);
    }
    activated_pane_id
}

const PANE_GAP: f32 = 1.0;

fn finite_stage_size(size: egui::Vec2) -> egui::Vec2 {
    vec2(finite_extent(size.x), finite_extent(size.y))
}

fn finite_extent(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

fn bounded_axis_cells(extent: f32, count: usize) -> (f32, f32) {
    let count = count.max(1);
    let total_gap = (PANE_GAP * count.saturating_sub(1) as f32).min(extent);
    ((extent - total_gap) / count as f32, total_gap)
}

fn bounded_pane_slots(stage: Rect, layout: PageLayout, pane_count: usize) -> Vec<Rect> {
    if pane_count == 0 {
        return Vec::new();
    }
    let stage = Rect::from_min_size(stage.min, finite_stage_size(stage.size()));
    let (columns, rows) = match layout {
        PageLayout::SinglePane => (1, 1),
        PageLayout::Rows => (1, pane_count),
        PageLayout::Columns => (pane_count, 1),
        PageLayout::Grid { columns } => {
            let columns = usize::from(columns.max(1)).min(pane_count);
            (columns, pane_count.div_ceil(columns))
        }
    };
    let (pane_width, horizontal_gap_budget) = bounded_axis_cells(stage.width(), columns);
    let (pane_height, vertical_gap_budget) = bounded_axis_cells(stage.height(), rows);
    let horizontal_gap = if columns > 1 {
        horizontal_gap_budget / (columns - 1) as f32
    } else {
        0.0
    };
    let vertical_gap = if rows > 1 {
        vertical_gap_budget / (rows - 1) as f32
    } else {
        0.0
    };

    (0..pane_count)
        .map(|index| {
            let column = index % columns;
            let row = index / columns;
            let min = pos2(
                stage.left() + column as f32 * (pane_width + horizontal_gap),
                stage.top() + row as f32 * (pane_height + vertical_gap),
            );
            Rect::from_min_size(min, vec2(pane_width, pane_height)).intersect(stage)
        })
        .collect()
}

fn bounded_inset(rect: Rect, requested: f32) -> Rect {
    let inset = requested
        .max(0.0)
        .min(rect.width() * 0.5)
        .min(rect.height() * 0.5);
    rect.shrink(inset)
}

fn render_pane(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    pane: &PaneProjection,
    active_pane_id: u64,
    active_viewer: Option<ResultViewer>,
) -> Option<u64> {
    let t = Tokens::get(ui.ctx());
    let rect = ui.available_rect_before_wrap();
    let is_active = pane.id.get() == active_pane_id;
    let viewer = pane_viewer(is_active, active_viewer, &pane.viewer_id);
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    ui.painter().rect_stroke(
        rect,
        0.0,
        egui::Stroke::new(
            if is_active { 2.0 } else { 1.0 },
            if is_active {
                t.color.accent
            } else {
                t.color.border
            },
        ),
        egui::StrokeKind::Inside,
    );
    ui.scope_builder(
        UiBuilder::new()
            .id_salt(("persistent-result-pane", pane.id.get()))
            .max_rect(bounded_inset(rect, 1.0))
            .layout(Layout::top_down(Align::Min)),
        |ui| {
            let activated_pane_id =
                pane_header(ui, app, pane, viewer, is_active).then_some(pane.id.get());
            retained_evidence_bar(ui, pane);
            let viewer_height = ui.available_height().max(1.0);
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), viewer_height),
                Layout::top_down(Align::Min),
                |ui| render_pane_viewer(ui, app, pane, viewer, is_active),
            );
            activated_pane_id
        },
    )
    .inner
}

fn retained_evidence_bar(ui: &mut Ui, pane: &PaneProjection) {
    if pane.measurements.is_empty() && pane.annotations.is_empty() {
        return;
    }
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(6, 3))
        .show(ui, |ui| {
            egui::ScrollArea::horizontal()
                .id_salt(("persistent-pane-evidence", pane.id.get()))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for measurement in &pane.measurements {
                            let value = measurement.value.map_or_else(
                                || "unevaluated".to_owned(),
                                |value| format!("{value:.6}"),
                            );
                            ui.label(
                                RichText::new(format!("{} = {value}", measurement.label))
                                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text),
                            )
                            .on_hover_text(
                                measurement
                                    .expression
                                    .as_deref()
                                    .unwrap_or("Retained document measurement"),
                            );
                        }
                        for annotation in &pane.annotations {
                            ui.label(
                                RichText::new(format!("Note: {}", annotation.text))
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text_dim),
                            )
                            .on_hover_text("Retained visualization annotation");
                        }
                    });
                });
        });
}

fn pane_viewer(
    is_active: bool,
    active_viewer: Option<ResultViewer>,
    retained_viewer_id: &str,
) -> Option<ResultViewer> {
    if is_active {
        active_viewer
    } else {
        ResultViewer::from_viewer_document_id(retained_viewer_id)
    }
}

fn pane_header(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    pane: &PaneProjection,
    viewer: Option<ResultViewer>,
    is_active: bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let viewer_title = viewer.map_or_else(
        || pane.viewer_id.as_str(),
        |viewer| {
            viewer
                .viewer_document_id()
                .and_then(viewer_document)
                .map_or(viewer.tab_label(), |document| document.title)
        },
    );
    let display_title = pane.title.as_str();
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 25.0), Sense::click());
    ui.painter().rect_filled(
        rect,
        0.0,
        if is_active {
            t.color.accent_dim
        } else {
            t.color.bg_panel
        },
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.scope_builder(
        UiBuilder::new()
            .max_rect(rect.shrink2(vec2(8.0, 3.0)))
            .layout(Layout::left_to_right(Align::Center)),
        |ui| {
            ui.label(
                RichText::new(display_title)
                    .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                    .color(t.color.text),
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(
                    RichText::new(viewer_title)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
            });
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    let tab_label = format!("{display_title}, {viewer_title}");
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            is_active,
            tab_label.clone(),
        )
    });
    let clicked = response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .on_hover_text(format!(
            "{} axes · {} traces · {} cursors · {} markers · {} measurements · {} annotations",
            pane.axes.len(),
            pane.traces.len(),
            pane.cursors.len(),
            pane.markers.len(),
            pane.measurements.len(),
            pane.annotations.len()
        ))
        .clicked();
    clicked && select_pane_context(&mut app.state, pane).is_ok()
}

fn render_pane_viewer(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    pane: &PaneProjection,
    viewer: Option<ResultViewer>,
    is_active: bool,
) {
    if let Err(reason) = select_pane_binding(&mut app.state, pane) {
        unavailable_surface(ui, &pane.title, &reason);
        return;
    }
    let Some(viewer) = viewer else {
        unavailable_surface(
            ui,
            &pane.title,
            &format!(
                "The pane references unregistered viewer identity {:?}. The retained dataset remains unchanged.",
                pane.viewer_id
            ),
        );
        return;
    };
    let Some(definition) = viewer_document(&pane.viewer_id) else {
        unavailable_surface(
            ui,
            &pane.title,
            "The pane's viewer identity is absent from the canonical Results contract.",
        );
        return;
    };
    if definition.release != ViewerReleaseClass::ReleaseTarget {
        unavailable_surface(ui, &pane.title, definition.release.unavailable_reason());
        return;
    }
    let viewer = bound_viewer_projection(&app.state, viewer);
    if !super::viewer_is_available(&app.state, viewer) {
        unavailable_surface(
            ui,
            &pane.title,
            super::viewer_unavailability_reason(&app.state, viewer)
                .unwrap_or("The retained analysis does not satisfy this viewer contract."),
        );
        return;
    }
    app.state
        .workbench
        .visualization_studio
        .selected_viewer_document = if is_active {
        let Some(viewer_document_id) = viewer.viewer_document_id() else {
            unavailable_surface(
                ui,
                &pane.title,
                "Dataset-native views cannot be retained as visualization panes.",
            );
            return;
        };
        viewer_document_id.to_owned()
    } else {
        pane.viewer_id.clone()
    };
    if let Err(reason) = project_pane_presentation(&mut app.state, pane, viewer) {
        unavailable_surface(ui, &pane.title, &reason);
        return;
    }
    super::show_persistent_pane_viewer(ui, app, viewer);
    capture_pane_presentation(&mut app.state, pane, viewer);
}

fn project_pane_presentation(
    state: &mut AppState,
    pane: &PaneProjection,
    viewer: ResultViewer,
) -> Result<(), String> {
    let run = state
        .simulation
        .active_run()
        .ok_or_else(|| "The pane's retained dataset is not active.".to_owned())?;
    let analysis = state
        .simulation
        .active_analysis()
        .ok_or_else(|| "The pane's retained analysis is not active.".to_owned())?;
    let analysis_key = super::AnalysisPresentationKey::new(run.dataset_id, analysis);
    let visibility = analysis
        .waveforms
        .iter()
        .filter_map(|waveform| {
            pane.traces
                .iter()
                .find(|trace| trace.label == waveform.name)
                .map(|trace| (waveform.name.clone(), waveform.visible, trace.visible))
        })
        .collect::<Vec<_>>();
    state
        .ui
        .results
        .enter_persistent_pane(pane.document_id, pane.id, analysis_key);
    state
        .ui
        .results
        .project_waveform_visibility(analysis_key, visibility);

    let axis_range = |orientation| {
        pane.axes
            .iter()
            .find(|axis| axis.orientation == orientation)
            .and_then(|axis| axis.range)
            .map(|range| (range.minimum, range.maximum))
    };
    state.ui.results.project_persistent_plot_view(
        viewer,
        axis_range(AxisOrientation::Horizontal),
        axis_range(AxisOrientation::VerticalLeft),
    );

    let cursor_position = |label: &str| {
        pane.cursors
            .iter()
            .find(|cursor| cursor.label == label)
            .and_then(|cursor| match cursor.position {
                TypedValue::Real(position) => Some(position),
                _ => None,
            })
    };
    state.ui.results.cursors.a = cursor_position("A");
    state.ui.results.cursors.b = cursor_position("B");
    state.ui.results.cursor_strip = state.simulation.active_analysis_idx;
    // Retained markers project into their own overlay. They are entities of
    // this document, not quick-view annotations of the dataset, so they carry
    // the document's full-width serial and never enter the project's
    // quick-marker list or its id allocator.
    let mut markers = Vec::with_capacity(pane.markers.len());
    for marker in &pane.markers {
        let trace = pane
            .traces
            .iter()
            .find(|trace| trace.id == marker.trace_id)
            .ok_or_else(|| {
                format!(
                    "Marker {} references an unavailable trace.",
                    marker.id.get()
                )
            })?;
        let TypedValue::Real(x) = &marker.coordinate else {
            return Err(format!(
                "Marker {} has a non-real plot coordinate.",
                marker.id.get()
            ));
        };
        let (analysis, anchor) = super::waves::source_waveform_anchor(state, &trace.label)
            .ok_or_else(|| {
                format!(
                    "Marker {} cannot resolve retained signal '{}'.",
                    marker.id.get(),
                    trace.label
                )
            })?;
        markers.push(super::DocumentMarker {
            document_id: pane.document_id,
            pane_id: pane.id,
            retained_id: marker.id,
            analysis,
            anchor,
            trace_name: trace.label.clone(),
            x: *x,
            kind: super::marker_kind_of_retained(marker.kind),
            note: marker.label.clone(),
        });
    }
    state.ui.results.project_document_markers(markers);
    Ok(())
}

/// Retain what the reader changed about this pane's viewport and cursors.
///
/// Markers are deliberately absent: every marker interaction — placement,
/// removal, and the purpose dialog's Apply — transacts against the document at
/// the moment it happens. Diffing them here instead meant a frame that
/// projected one pane and captured another could delete the second pane's
/// markers, and it forced the document's full-width entity serials through a
/// quick-view `u32` to make the two lists comparable at all.
fn capture_pane_presentation(state: &mut AppState, pane: &PaneProjection, viewer: ResultViewer) {
    let view = state.ui.results.persistent_plot_view(viewer);
    let cursors = state.ui.results.cursors;
    let Some(document) = state.workspace.visualization_document(pane.document_id) else {
        return;
    };
    let revision = document.revision();
    let mut edits = Vec::new();
    for (orientation, requested) in [
        (AxisOrientation::Horizontal, view.x),
        (AxisOrientation::VerticalLeft, view.y),
    ] {
        let Some(axis) = document
            .axes()
            .iter()
            .find(|axis| axis.pane_id == pane.id && axis.orientation == orientation)
        else {
            continue;
        };
        let requested =
            requested.and_then(|(minimum, maximum)| AxisRange::new(minimum, maximum).ok());
        if axis.range != requested {
            edits.push(DocumentEdit::SetAxisRange {
                axis_id: axis.id,
                range: requested,
            });
        }
    }
    let Some(horizontal_axis) = document
        .axes()
        .iter()
        .find(|axis| axis.pane_id == pane.id && axis.orientation == AxisOrientation::Horizontal)
    else {
        return;
    };
    for (label, requested) in [("A", cursors.a), ("B", cursors.b)] {
        let retained = document
            .cursors()
            .iter()
            .find(|cursor| cursor.pane_id == pane.id && cursor.label == label);
        match (retained, requested) {
            (Some(cursor), Some(position)) if cursor.position != TypedValue::Real(position) => {
                edits.push(DocumentEdit::MoveCursor {
                    cursor_id: cursor.id,
                    position: TypedValue::Real(position),
                });
            }
            (None, Some(position)) => edits.push(DocumentEdit::AddCursor {
                pane_id: pane.id,
                axis_id: horizontal_axis.id,
                position: TypedValue::Real(position),
                label: label.to_owned(),
            }),
            (Some(cursor), None) => edits.push(DocumentEdit::Remove(EntityRef::Cursor(cursor.id))),
            (Some(_), Some(_)) | (None, None) => {}
        }
    }
    if edits.is_empty() {
        return;
    }
    if let Err(error) =
        state
            .workspace
            .transact_visualization_document(pane.document_id, revision, edits)
    {
        state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
            "Could not retain Results pane presentation: {error}"
        )));
    }
}

fn select_pane_context(state: &mut AppState, pane: &Pane) -> Result<(), String> {
    select_pane_binding(state, pane)?;
    state.workbench.visualization_studio.active_pane = Some(pane.id.get());
    state
        .workbench
        .visualization_studio
        .selected_viewer_document = pane.viewer_id.clone();
    if let Some(viewer) = ResultViewer::from_viewer_document_id(&pane.viewer_id) {
        state.ui.results.viewer = bound_viewer_projection(state, viewer);
    }
    Ok(())
}

fn bound_viewer_projection(state: &AppState, viewer: ResultViewer) -> ResultViewer {
    state
        .simulation
        .active_analysis()
        .map_or(viewer, |analysis| {
            super::project_viewer_for_analysis(viewer, analysis)
        })
}

/// Bind the global simulation projection to one pane's immutable dataset.
///
/// Re-selecting the binding that is already active must be inert.
/// `select_run` resynchronizes the displayed waveform set, which advances the
/// simulation data version, and every version change retires cursors, the
/// selected trace, the active pane, pinned readouts and the renderer caches.
/// Every pane of every frame runs this path — several times, since painting
/// inactive panes temporarily selects their bindings and then restores the
/// active one — so an unguarded re-selection made all of those states
/// impossible to hold at all while a project-owned document was open. The
/// document bar's dataset activation carries the identical guard.
fn select_pane_binding(state: &mut AppState, pane: &Pane) -> Result<(), String> {
    let binding = pane
        .binding
        .ok_or_else(|| "This result pane has no immutable dataset binding.".to_owned())?;
    let (run_index, analysis_index) = resolve_binding(state, binding)?;
    if state.simulation.active_run_idx == Some(run_index)
        && state.simulation.active_analysis_idx == Some(analysis_index)
    {
        return Ok(());
    }
    if !state.simulation.select_run(run_index) {
        return Err("The retained dataset could not be selected.".to_owned());
    }
    if !state.simulation.select_analysis(analysis_index) {
        return Err("The retained analysis could not be selected.".to_owned());
    }
    Ok(())
}

fn resolve_binding(state: &AppState, binding: PaneDataBinding) -> Result<(usize, usize), String> {
    let run_index = state
        .simulation
        .runs
        .iter()
        .position(|run| run.dataset_id == binding.dataset.dataset_id)
        .ok_or_else(|| "The pane's immutable result dataset is no longer retained.".to_owned())?;
    let run = &state.simulation.runs[run_index];
    if run.dataset_content_digest() != binding.dataset.content_digest {
        return Err(
            "The retained dataset content does not match the pane's immutable binding.".to_owned(),
        );
    }
    let analysis_index = run
        .analyses
        .iter()
        .position(|analysis| analysis_identity(run, analysis) == binding.analysis_id)
        .ok_or_else(|| "The pane's bound analysis is no longer retained.".to_owned())?;
    Ok((run_index, analysis_index))
}

fn analysis_identity(run: &SimulationRun, analysis: &AnalysisResult) -> AnalysisInstanceId {
    analysis.provenance().map_or_else(
        || {
            let name = format!("legacy-analysis-v1/{}", analysis.id);
            AnalysisInstanceId::from_namespace(run.dataset_id.as_uuid(), name.as_bytes())
        },
        |provenance| provenance.source_instance_id(),
    )
}

/// Whether the registered persistent renderer can truthfully present this
/// exact retained analysis. Catalog compatibility is intentionally broader:
/// one catalog document can own several specialist modes, while the current
/// native renderer may implement only a subset. Creation must never bind a
/// broader catalog promise to a narrower renderer and silently substitute the
/// resulting presentation.
pub(super) fn renderer_supports_analysis(id: &str, analysis: &AnalysisResult) -> bool {
    if ResultViewer::from_viewer_document_id(id).is_none() {
        return false;
    }
    match id {
        "viewer-waveform" => {
            (analysis.analysis_type.is_time_domain()
                || analysis.analysis_type == AnalysisType::DcSweep)
                && !analysis.waveforms.is_empty()
        }
        "viewer-bode" => {
            super::bode_analysis_is_renderable(analysis)
                || super::bode::ordinary_noise_spectrum_is_renderable(analysis)
        }
        "viewer-spectrum" => {
            (analysis.analysis_type.is_time_domain() && !analysis.waveforms.is_empty())
                || super::harmonic_balance_analysis_is_renderable(analysis)
        }
        "viewer-phase-noise" => super::phase_noise_analysis_is_renderable(analysis),
        "viewer-smith" => super::smith::analysis_is_renderable(analysis),
        "viewer-table" => {
            !analysis.waveforms.is_empty()
                || super::view_context::analysis_supports_viewer(ResultViewer::Op, analysis)
        }
        "viewer-histogram" => analysis.analysis_type == AnalysisType::MonteCarlo,
        "eye-viewer" => analysis.analysis_type.is_time_domain() && !analysis.waveforms.is_empty(),
        "viewer-pz" => analysis.analysis_type == AnalysisType::PoleZero,
        "viewer-contribution" => matches!(
            analysis.analysis_type,
            AnalysisType::Sensitivity | AnalysisType::DcMismatch
        ),
        "viewer-transfer-function" => analysis.analysis_type == AnalysisType::Tf,
        "viewer-digital-events" => {
            super::view_context::analysis_supports_viewer(ResultViewer::Events, analysis)
        }
        "viewer-soa" => super::view_context::analysis_supports_viewer(ResultViewer::Soa, analysis),
        "viewer-optimization" => {
            super::view_context::analysis_supports_viewer(ResultViewer::Optimization, analysis)
        }
        // Reliability remains a typed canonical pane, but its producer and
        // numeric evidence are preview-classified by the current contract.
        // It must not become creatable merely because a quick sheet exists.
        "viewer-reliability" => false,
        _ => false,
    }
}

/// One-line strip above the document stating why Latest tracking is standing
/// still, without taking the document's own surface away from the reader.
fn tracking_banner(ui: &mut Ui, reason: &str) {
    let t = Tokens::get(ui.ctx());
    let height = ui.text_style_height(&egui::TextStyle::Body) + 10.0;
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), height), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    ui.painter().rect_filled(
        egui::Rect::from_min_size(rect.left_top(), egui::vec2(3.0, rect.height())),
        0.0,
        t.color.warn,
    );
    ui.painter().text(
        pos2(rect.left() + 13.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        reason,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text,
    );
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, reason));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
        node.set_label(reason);
    });
}

fn unavailable_surface(ui: &mut Ui, title: &str, reason: &str) {
    let t = Tokens::get(ui.ctx());
    let rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    ui.scope_builder(
        UiBuilder::new()
            .max_rect(rect.shrink(20.0))
            .layout(Layout::top_down_justified(Align::Center)),
        |ui| {
            ui.add_space((ui.available_height() * 0.35).max(0.0));
            ui.label(
                RichText::new(title)
                    .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.label(
                RichText::new(reason)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisResultProvenance, AnalysisResultSourceDomain, PreparedRunReceipt,
        PreparedRunTaskReceipt, PreparedSourceCheckReceipt, SimulationRunLifecycle,
        SimulationRunProvenance,
    };
    use crate::workbench::state::CreateResultDocumentDialogState;

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    fn persistent_transient_fixture() -> (RSpiceApp, ResultDocumentId) {
        let mut app = RSpiceApp::test_instance();
        let mut run = SimulationRun::new(1);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
                crate::state::WaveformData::new(
                    "V(out)",
                    vec![0.0, 0.5, 1.0],
                    vec![0.0, 1.0, 0.0],
                    "#fff",
                ),
            ]),
        );
        let dataset_id = run.dataset_id;
        app.state.simulation.runs = vec![run];
        assert!(app.state.simulation.select_run(0));
        assert!(app.state.simulation.select_analysis(0));
        app.state.workbench.create_result_document = CreateResultDocumentDialogState {
            open: true,
            name: "Durable transient review".to_owned(),
            name_touched: true,
            dataset_id: Some(dataset_id),
            family_id: "waveform-worksheet".to_owned(),
            viewer_id: "viewer-waveform".to_owned(),
            layout_id: "single-pane".to_owned(),
            validation_error: None,
        };
        let document_id =
            super::super::create_document::commit(&mut app).expect("persistent document commits");
        (app, document_id)
    }

    /// Drive one whole frame of a surface, with the product theme applied so
    /// token lookups and font metrics resolve exactly as they do on screen.
    fn drive_frame(body: impl FnMut(&mut Ui)) {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut body = body;
        let _ = ctx.run_ui(egui::RawInput::default(), |ui| body(ui));
    }

    /// Re-selecting the binding a persistent pane already holds must be inert.
    ///
    /// `select_run` resynchronizes the displayed waveform set and advances the
    /// simulation data version; every version change retires cursors, the
    /// selected trace, the active pane, pinned readouts and the renderer
    /// caches. This path runs on every frame the document draws, so an
    /// unguarded re-selection made all of those states impossible to hold.
    #[test]
    fn idle_frames_of_an_open_persistent_document_never_advance_the_data_version() {
        let (mut app, document_id) = persistent_transient_fixture();
        drive_frame(|ui| show(ui, &mut app, document_id));
        let settled = app.state.simulation.data_version;

        for frame in 0..4 {
            drive_frame(|ui| show(ui, &mut app, document_id));
            assert_eq!(
                app.state.simulation.data_version, settled,
                "idle frame {frame} of an open persistent document advanced the data version"
            );
        }
    }

    /// The state an advancing data version retires has to survive an idle
    /// frame, or a reader can never hold a cursor, a trace selection or a
    /// pinned readout inside a project-owned document at all.
    #[test]
    fn idle_frames_of_an_open_persistent_document_hold_cursors_and_trace_selection() {
        let (mut app, document_id) = persistent_transient_fixture();
        drive_frame(|ui| show(ui, &mut app, document_id));
        let analysis = super::super::AnalysisPresentationKey::new(
            app.state.simulation.runs[0].dataset_id,
            &app.state.simulation.runs[0].analyses[0],
        );
        app.state.ui.results.selected_trace = Some(
            super::super::SelectedResultTrace::from_identity(analysis, "V(out)"),
        );
        app.state
            .ui
            .results
            .rf_pin
            .insert(ResultViewer::Smith, (0, 3));

        for frame in 0..3 {
            drive_frame(|ui| show(ui, &mut app, document_id));
            assert!(
                app.state.ui.results.selected_trace.is_some(),
                "idle frame {frame} cleared the selected trace"
            );
            assert!(
                app.state
                    .ui
                    .results
                    .rf_pin
                    .contains_key(&ResultViewer::Smith),
                "idle frame {frame} cleared the pinned readout"
            );
        }
    }

    // -----------------------------------------------------------------
    // marker ownership
    // -----------------------------------------------------------------

    /// Project the fixture's one pane, exactly as a drawn frame does, and
    /// hand back the pane so the test can keep transacting against it.
    fn projected_pane(app: &mut RSpiceApp, document_id: ResultDocumentId) -> PaneProjection {
        let mut projected = projection(&app.state, document_id).expect("document projection");
        let mut page = projected.pages.remove(0);
        let pane = page.panes.remove(0);
        select_pane_binding(&mut app.state, &pane).expect("pane binding");
        project_pane_presentation(&mut app.state, &pane, ResultViewer::Waves)
            .expect("presentation projects");
        pane
    }

    fn active_analysis_anchor(
        state: &mut AppState,
    ) -> (
        super::super::AnalysisPresentationKey,
        super::super::WaveformPresentationKey,
    ) {
        super::super::waves::source_waveform_anchor(state, "V(out)")
            .expect("the fixture retains V(out)")
    }

    /// The fixture's one trace's X samples, as the drawn pane knows them.
    static FIXTURE_SAMPLES: [f64; 3] = [0.0, 0.5, 1.0];

    fn placement(
        analysis: super::super::AnalysisPresentationKey,
        anchor: &super::super::WaveformPresentationKey,
        trace_name: &str,
        x: f64,
    ) -> super::super::MarkerPlacement<'static> {
        super::super::MarkerPlacement {
            analysis,
            anchor: anchor.clone(),
            trace_name: trace_name.to_owned(),
            x,
            samples: &FIXTURE_SAMPLES,
        }
    }

    fn retained_markers(app: &RSpiceApp, document_id: ResultDocumentId) -> Vec<Marker> {
        app.state
            .workspace
            .visualization_document(document_id)
            .expect("retained document")
            .markers()
            .to_vec()
    }

    /// (h.2) Projecting a pane that already holds a retained marker must not
    /// adopt it into the project's quick-view list.
    ///
    /// The projection used to write the document's markers straight into
    /// `ResultsState::markers` — the list the project file saves — so a
    /// document's own annotation was saved a second time as a dataset marker,
    /// under an id truncated out of the document's entity serial.
    #[test]
    fn projecting_a_retained_marker_never_adopts_it_into_the_quick_list() {
        let (mut app, document_id) = persistent_transient_fixture();
        let pane = projected_pane(&mut app, document_id);
        let (trace_id, revision) = {
            let document = app
                .state
                .workspace
                .visualization_document(document_id)
                .expect("retained document");
            (document.traces()[0].id, document.revision())
        };
        app.state
            .workspace
            .transact_visualization_document(
                document_id,
                revision,
                vec![DocumentEdit::AddTypedMarker {
                    pane_id: pane.id,
                    trace_id,
                    coordinate: TypedValue::Real(0.5),
                    label: "retained".to_owned(),
                    kind: crate::results::visualization_document::PlotMarkerKind::PointNote,
                    scope: crate::results::visualization_document::PlotMarkerScope::Pane,
                    source_specification: None,
                }],
            )
            .expect("the document retains a marker");

        let pane = projected_pane(&mut app, document_id);

        assert!(
            app.state.ui.results.markers.is_empty(),
            "a retained document marker leaked into the project's quick-view list: {:?}",
            app.state
                .ui
                .results
                .markers
                .iter()
                .map(|marker| (marker.id, marker.note.clone()))
                .collect::<Vec<_>>()
        );
        assert_eq!(app.state.ui.results.document_markers.len(), 1);
        assert_eq!(app.state.ui.results.document_markers[0].note, "retained");
        assert_eq!(app.state.ui.results.document_markers[0].pane_id, pane.id);
    }

    /// (h.7) A click inside a persistent pane is retained by the document,
    /// not copied into the project's quick-view marker list.
    #[test]
    fn placing_a_marker_on_a_persistent_pane_transacts_against_the_document() {
        let (mut app, document_id) = persistent_transient_fixture();
        let pane = projected_pane(&mut app, document_id);
        let (analysis, anchor) = active_analysis_anchor(&mut app.state);
        let revision_before = app
            .state
            .workspace
            .visualization_document(document_id)
            .expect("retained document")
            .revision();

        let selector =
            super::super::place_marker(&mut app.state, placement(analysis, &anchor, "V(out)", 0.5))
                .expect("the placement resolves a store");

        let super::super::MarkerSelector::Document {
            document_id: routed_document,
            pane_id,
            marker_id,
        } = selector
        else {
            panic!("a persistent pane must retain its own markers");
        };
        assert_eq!(routed_document, document_id);
        assert_eq!(pane_id, pane.id);
        let retained = retained_markers(&app, document_id);
        assert_eq!(retained.len(), 1);
        assert_eq!(retained[0].id, marker_id);
        assert_eq!(retained[0].coordinate, TypedValue::Real(0.5));
        assert_ne!(
            app.state
                .workspace
                .visualization_document(document_id)
                .expect("retained document")
                .revision(),
            revision_before,
            "retaining a marker advances the document revision"
        );
        assert!(
            app.state.ui.results.markers.is_empty(),
            "a document marker must never enter the project's quick-view list"
        );
        // It is addressable in the frame it was placed in, not one frame later.
        assert!(app.state.ui.results.document_marker(marker_id).is_some());
    }

    /// (h.7) A document cannot retain a marker on a trace it does not own, so
    /// the click still lands — on the dataset — and says so.
    #[test]
    fn a_trace_the_document_does_not_retain_falls_back_to_a_quick_marker() {
        let (mut app, document_id) = persistent_transient_fixture();
        let _pane = projected_pane(&mut app, document_id);
        let (analysis, anchor) = active_analysis_anchor(&mut app.state);

        let selector = super::super::place_marker(
            &mut app.state,
            placement(analysis, &anchor, "V(a)+V(b)", 0.25),
        )
        .expect("the placement resolves a store");

        assert!(matches!(selector, super::super::MarkerSelector::Quick(_)));
        assert_eq!(app.state.ui.results.markers.len(), 1);
        assert!(retained_markers(&app, document_id).is_empty());
        assert!(
            app.state
                .log_buffer
                .entries()
                .any(|entry| entry.message.contains("not a retained trace")),
            "the fallback has to be stated, not silent"
        );
    }

    /// (h.1) Quick markers are a fact about the dataset. Opening, drawing and
    /// leaving a project-owned document must not adopt, rewrite or drop them.
    #[test]
    fn quick_markers_are_untouched_by_opening_and_leaving_a_persistent_document() {
        let (mut app, document_id) = persistent_transient_fixture();
        let (analysis, anchor) = active_analysis_anchor(&mut app.state);
        let quick =
            app.state
                .ui
                .results
                .add_marker(analysis, anchor.clone(), "V(out)".to_owned(), 0.75);
        if let Some(marker) = app.state.ui.results.marker_mut(quick) {
            marker.note = "dataset note".to_owned();
        }
        let pane = projected_pane(&mut app, document_id);
        super::super::place_marker(&mut app.state, placement(analysis, &anchor, "V(out)", 0.5))
            .expect("the document retains its own marker");

        for _ in 0..3 {
            drive_frame(|ui| show(ui, &mut app, document_id));
        }

        assert_eq!(app.state.ui.results.markers.len(), 1);
        assert_eq!(app.state.ui.results.markers[0].id, quick);
        assert_eq!(app.state.ui.results.markers[0].note, "dataset note");
        assert_eq!(
            app.state.ui.results.document_markers.len(),
            1,
            "the drawn pane's retained markers project into the overlay"
        );
        assert_eq!(app.state.ui.results.document_markers[0].pane_id, pane.id);

        // Back on a quick surface the overlay is gone and the quick marker is
        // exactly what it was.
        drive_frame(|ui| super::super::show_compact_split(ui, &mut app));
        assert!(app.state.ui.results.document_markers.is_empty());
        assert!(app.state.ui.results.persistent_pane_context.is_none());
        assert_eq!(app.state.ui.results.markers.len(), 1);
        assert_eq!(app.state.ui.results.markers[0].note, "dataset note");
    }

    /// (h.8) The Studio stage embeds the renderer against the global
    /// projection, so it clears the pane context and its overlay too.
    #[test]
    fn the_studio_stage_clears_the_persistent_pane_context_and_overlay() {
        let (mut app, document_id) = persistent_transient_fixture();
        let (analysis, anchor) = active_analysis_anchor(&mut app.state);
        let _pane = projected_pane(&mut app, document_id);
        super::super::place_marker(&mut app.state, placement(analysis, &anchor, "V(out)", 0.5))
            .expect("the document retains its own marker");
        assert!(!app.state.ui.results.document_markers.is_empty());

        drive_frame(|ui| {
            super::super::show_embedded_with_sample_selection(ui, &mut app, None);
        });

        assert!(app.state.ui.results.persistent_pane_context.is_none());
        assert!(app.state.ui.results.document_markers.is_empty());
    }

    /// (h.2, h.3) The two stores never contend: a document serial cannot
    /// advance the quick allocator, and the project save carries exactly the
    /// quick markers.
    #[test]
    fn document_marker_serials_never_reach_the_project_save_or_the_quick_allocator() {
        let (mut app, document_id) = persistent_transient_fixture();
        let (analysis, anchor) = active_analysis_anchor(&mut app.state);
        let _pane = projected_pane(&mut app, document_id);
        let selector =
            super::super::place_marker(&mut app.state, placement(analysis, &anchor, "V(out)", 0.5))
                .expect("the document retains its own marker");
        let super::super::MarkerSelector::Document { marker_id, .. } = selector else {
            panic!("a persistent pane retains its own markers");
        };

        let quick = app
            .state
            .ui
            .results
            .add_marker(analysis, anchor, "V(out)".to_owned(), 0.9);

        assert_eq!(
            quick,
            1,
            "the quick allocator counts quick markers alone, whatever serial \
             the document handed its own marker ({})",
            marker_id.get()
        );
        assert_eq!(
            app.state.ui.results.markers.len(),
            1,
            "the project's saved marker list is exactly the quick markers"
        );
        assert_eq!(app.state.ui.results.markers[0].id, quick);
    }

    /// (h.5) Apply routes by the store the dialog opened on, even when a quick
    /// id and a document serial are the same number — the case the old shared
    /// `u32` space could not tell apart.
    #[test]
    fn the_marker_dialog_applies_to_the_store_it_opened_on() {
        let (mut app, document_id) = persistent_transient_fixture();
        let (analysis, anchor) = active_analysis_anchor(&mut app.state);
        let _pane = projected_pane(&mut app, document_id);
        let selector =
            super::super::place_marker(&mut app.state, placement(analysis, &anchor, "V(out)", 0.5))
                .expect("the document retains its own marker");
        let super::super::MarkerSelector::Document { marker_id, .. } = selector else {
            panic!("a persistent pane retains its own markers");
        };
        // Give the quick store a marker whose id is the document serial, so
        // the two identities collide as integers and can only be told apart
        // by the store they name.
        let colliding = u32::try_from(marker_id.get()).expect("a small test serial");
        app.state
            .ui
            .results
            .adopt_markers(vec![super::super::ResultMarker {
                id: colliding,
                analysis,
                anchor,
                trace_name: "V(out)".to_owned(),
                x: 0.9,
                kind: super::super::MarkerKind::Note,
                note: "quick".to_owned(),
            }]);

        super::super::commit_marker_edit(
            &mut app.state,
            selector,
            "retained",
            super::super::MarkerKind::Peak,
        )
        .expect("the document edit commits");

        let retained = retained_markers(&app, document_id);
        assert_eq!(retained.len(), 1);
        assert_eq!(retained[0].label, "retained");
        assert_eq!(
            retained[0].kind,
            crate::results::visualization_document::PlotMarkerKind::Peak
        );
        assert_eq!(
            app.state.ui.results.markers[0].note, "quick",
            "the quick marker that shares the number must be untouched"
        );
        assert_eq!(
            app.state.ui.results.markers[0].kind,
            super::super::MarkerKind::Note
        );

        super::super::commit_marker_edit(
            &mut app.state,
            super::super::MarkerSelector::Quick(colliding),
            "edited quick",
            super::super::MarkerKind::Spec,
        )
        .expect("the quick edit commits");

        assert_eq!(app.state.ui.results.markers[0].note, "edited quick");
        assert_eq!(
            retained_markers(&app, document_id)[0].label,
            "retained",
            "editing the quick marker must not reach the document"
        );
    }

    /// (h.6) Removing a row removes it from the store that owns it, and only
    /// a document row advances the document.
    #[test]
    fn removing_a_marker_row_reaches_only_the_store_that_owns_it() {
        let (mut app, document_id) = persistent_transient_fixture();
        let (analysis, anchor) = active_analysis_anchor(&mut app.state);
        let _pane = projected_pane(&mut app, document_id);
        let document_selector =
            super::super::place_marker(&mut app.state, placement(analysis, &anchor, "V(out)", 0.5))
                .expect("the document retains its own marker");
        let quick = app
            .state
            .ui
            .results
            .add_marker(analysis, anchor, "V(out)".to_owned(), 0.9);
        let revision_before = app
            .state
            .workspace
            .visualization_document(document_id)
            .expect("retained document")
            .revision();

        super::super::remove_marker(&mut app.state, super::super::MarkerSelector::Quick(quick));

        assert!(app.state.ui.results.markers.is_empty());
        assert_eq!(retained_markers(&app, document_id).len(), 1);
        assert_eq!(
            app.state
                .workspace
                .visualization_document(document_id)
                .expect("retained document")
                .revision(),
            revision_before,
            "removing a quick marker is not a change to the document"
        );

        super::super::remove_marker(&mut app.state, document_selector);

        assert!(retained_markers(&app, document_id).is_empty());
        assert!(app.state.ui.results.document_markers.is_empty());
        assert_ne!(
            app.state
                .workspace
                .visualization_document(document_id)
                .expect("retained document")
                .revision(),
            revision_before
        );
    }

    /// (h.4) A project saved before markers had one owner captured the
    /// document's markers into the quick list as well. Reopening must keep the
    /// genuine quick marker and drop the duplicate rather than draw both.
    #[test]
    fn a_saved_projection_of_a_retained_marker_is_dropped_on_load() {
        let (mut app, document_id) = persistent_transient_fixture();
        let (analysis, anchor) = active_analysis_anchor(&mut app.state);
        let _pane = projected_pane(&mut app, document_id);
        super::super::place_marker(&mut app.state, placement(analysis, &anchor, "V(out)", 0.5))
            .expect("the document retains its own marker");
        let marker_id = retained_markers(&app, document_id)[0].id;
        super::super::commit_marker_edit(
            &mut app.state,
            super::super::MarkerSelector::Document {
                document_id,
                pane_id: _pane.id,
                marker_id,
            },
            "overshoot",
            super::super::MarkerKind::Peak,
        )
        .expect("the document edit commits");

        let duplicate = super::super::ResultMarker {
            id: 4,
            analysis,
            anchor: anchor.clone(),
            trace_name: "V(out)".to_owned(),
            x: 0.5,
            kind: super::super::MarkerKind::Peak,
            note: "overshoot".to_owned(),
        };
        let genuine = super::super::ResultMarker {
            id: 5,
            analysis,
            anchor,
            trace_name: "V(out)".to_owned(),
            x: 0.9,
            kind: super::super::MarkerKind::Note,
            note: "settling".to_owned(),
        };

        super::super::restore_markers(&mut app.state, vec![duplicate, genuine]);

        assert_eq!(app.state.ui.results.markers.len(), 1);
        assert_eq!(app.state.ui.results.markers[0].note, "settling");
    }

    #[test]
    fn projection_carries_every_document_owned_pane_entity() {
        let (app, document_id) = persistent_transient_fixture();
        let projected = projection(&app.state, document_id).expect("document projection");
        let pane = &projected.pages[0].panes[0];
        let document = app
            .state
            .workspace
            .visualization_document(document_id)
            .expect("retained document");

        assert_eq!(pane.axes, document.axes());
        assert_eq!(pane.traces, document.traces());
        assert_eq!(pane.cursors, document.cursors());
        assert_eq!(pane.markers, document.markers());
        assert_eq!(pane.measurements, document.measurements());
        assert_eq!(pane.annotations, document.annotations());
    }

    #[test]
    fn persistent_trace_axis_and_cursor_interactions_commit_without_mutating_results() {
        let (mut app, document_id) = persistent_transient_fixture();
        let mut projected = projection(&app.state, document_id).expect("document projection");
        let mut page = projected.pages.remove(0);
        let pane = page.panes.remove(0);
        select_pane_binding(&mut app.state, &pane).expect("pane binding");
        project_pane_presentation(&mut app.state, &pane, ResultViewer::Waves)
            .expect("presentation projects");
        let retained_default = app.state.simulation.runs[0].analyses[0].waveforms[0].visible;

        super::super::waves::toggle_visibility(&mut app.state, 0, 0);
        assert_eq!(
            app.state.simulation.runs[0].analyses[0].waveforms[0].visible,
            retained_default
        );
        assert!(
            !app.state
                .workspace
                .visualization_document(document_id)
                .expect("document remains retained")
                .traces()[0]
                .visible
        );

        let analysis = super::super::AnalysisPresentationKey::new(
            app.state.simulation.runs[0].dataset_id,
            &app.state.simulation.runs[0].analyses[0],
        );
        let view =
            app.state
                .ui
                .results
                .analysis_plot_view_pane_mut(ResultViewer::Waves, analysis, 0);
        view.x = Some((0.2, 0.8));
        view.y = Some((-0.25, 1.25));
        app.state.ui.results.cursors.a = Some(0.5);
        capture_pane_presentation(&mut app.state, &pane, ResultViewer::Waves);

        let document = app
            .state
            .workspace
            .visualization_document(document_id)
            .expect("captured document");
        assert_eq!(
            document
                .axes()
                .iter()
                .find(|axis| axis.orientation == AxisOrientation::Horizontal)
                .and_then(|axis| axis.range),
            Some(AxisRange::new(0.2, 0.8).unwrap())
        );
        assert!(
            document
                .cursors()
                .iter()
                .any(|cursor| { cursor.label == "A" && cursor.position == TypedValue::Real(0.5) })
        );
        assert!(app.state.workspace.visualization_documents_dirty);
    }

    fn completed_prepared_run(
        plan_id: SimulationPlanId,
        project_revision: ObjectRevision,
        source_digest: ContentDigest,
    ) -> SimulationRun {
        let task = PreparedRunTaskReceipt::new(
            AnalysisInstanceId::new(),
            ObjectRevision::INITIAL,
            Vec::new(),
            5,
            digest(0x44),
        )
        .expect("task receipt");
        let receipt = PreparedRunReceipt::new(
            AnalysisResultSourceDomain::SimulationPlan,
            Some(plan_id),
            project_revision,
            digest(0x41),
            source_digest,
            PreparedSourceCheckReceipt::SchematicDrc(digest(0x43)),
            vec![task],
        )
        .expect("run receipt");
        let mut run = SimulationRun::new(1);
        run.restore_provenance(SimulationRunProvenance::Prepared(Box::new(receipt)))
            .expect("run provenance");
        run.mark_running().expect("running lifecycle");
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("completed lifecycle");
        run
    }

    // -----------------------------------------------------------------
    // Latest tracking
    // -----------------------------------------------------------------

    /// A document that follows the newest run of one authored analysis, plus
    /// the identities a second run needs to be a candidate for it.
    struct LatestFixture {
        app: RSpiceApp,
        document_id: ResultDocumentId,
        plan_id: SimulationPlanId,
        authored_analysis_id: AnalysisInstanceId,
        source_digest: ContentDigest,
    }

    fn authored_run(
        run_sequence: u64,
        plan_id: SimulationPlanId,
        project_revision: ObjectRevision,
        source_digest: ContentDigest,
        authored_analysis_id: AnalysisInstanceId,
        trace_name: &str,
    ) -> SimulationRun {
        let mut run = completed_prepared_run(plan_id, project_revision, source_digest);
        run.id = run_sequence;
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Transient")
                .with_waveforms(vec![crate::state::WaveformData::new(
                    trace_name,
                    vec![0.0, 0.5, 1.0],
                    vec![0.0, 1.0, 0.0],
                    "#fff",
                )])
                .with_provenance(
                    AnalysisResultProvenance::new(
                        authored_analysis_id,
                        ObjectRevision::INITIAL,
                        digest(0x51),
                        Vec::new(),
                    )
                    .expect("analysis provenance"),
                ),
        );
        run
    }

    fn latest_tracking_fixture() -> LatestFixture {
        use crate::results::visualization_document::{
            DocumentEdit, ResultDocumentTracking, ResultDocumentTrackingMode,
        };

        let mut app = RSpiceApp::test_instance();
        // Latest tracking only follows runs the current authored source
        // authorizes, so the fixture has to be that source.
        app.state.simulation.netlist_content = "* latest tracking\nV1 out 0 1\n.end\n".to_owned();
        let input_digest = digest(0x60);
        app.state.ui.netlist.generation_error = None;
        app.state.ui.netlist.generated_input_digest = Some(input_digest);
        app.state.ui.netlist.current_generation_input_digest = Some(input_digest);
        let source_digest = current_result_source_digest(&app.state)
            .expect("the fixture states a current authored source");

        let plan_id = SimulationPlanId::new();
        let authored_analysis_id = AnalysisInstanceId::new();
        let project_revision = app.state.workspace.project.revision();
        let run = authored_run(
            1,
            plan_id,
            project_revision,
            source_digest,
            authored_analysis_id,
            "V(out)",
        );
        let dataset_id = run.dataset_id;
        app.state.simulation.runs = vec![run];
        assert!(app.state.simulation.select_run(0));
        assert!(app.state.simulation.select_analysis(0));
        app.state.workbench.create_result_document = CreateResultDocumentDialogState {
            open: true,
            name: "Latest transient review".to_owned(),
            name_touched: true,
            dataset_id: Some(dataset_id),
            family_id: "waveform-worksheet".to_owned(),
            viewer_id: "viewer-waveform".to_owned(),
            layout_id: "single-pane".to_owned(),
            validation_error: None,
        };
        let document_id =
            super::super::create_document::commit(&mut app).expect("persistent document commits");
        let revision = app
            .state
            .workspace
            .visualization_document(document_id)
            .expect("retained document")
            .revision();
        app.state
            .workspace
            .transact_visualization_document(
                document_id,
                revision,
                vec![DocumentEdit::SetTracking(ResultDocumentTracking::for_plan(
                    ResultDocumentTrackingMode::Latest,
                    plan_id,
                    authored_analysis_id,
                ))],
            )
            .expect("the document tracks the latest run");

        LatestFixture {
            app,
            document_id,
            plan_id,
            authored_analysis_id,
            source_digest,
        }
    }

    /// A Latest document whose newest run renamed the net it plots keeps
    /// drawing the dataset it last resolved, under a banner naming the cause
    /// and the control that settles it. Blanking the surface withheld the
    /// evidence the reader already had.
    #[test]
    fn a_latest_document_that_cannot_retarget_keeps_its_last_good_binding() {
        let LatestFixture {
            mut app,
            document_id,
            plan_id,
            authored_analysis_id,
            source_digest,
        } = latest_tracking_fixture();
        let good_dataset = app.state.simulation.runs[0].dataset_id;
        let project_revision = app.state.workspace.project.revision();
        // The newest run of the same authored analysis no longer carries the
        // signal this document's traces name.
        app.state.simulation.runs.push(authored_run(
            2,
            plan_id,
            project_revision,
            source_digest,
            authored_analysis_id,
            "V(renamed)",
        ));
        let stale_dataset = app.state.simulation.runs[1].dataset_id;
        assert_ne!(good_dataset, stale_dataset);

        let LatestBinding::Degraded(reason) = refresh_latest_binding(&mut app.state, document_id)
        else {
            panic!("a document that cannot retarget must degrade, not advance or blank");
        };
        assert!(
            reason.contains("Latest / Pinned"),
            "the banner has to name the control that settles this: {reason}"
        );
        assert!(
            reason.contains("could not be retargeted onto the newest run")
                || reason.contains("no longer builds its retained source"),
            "the banner has to name the cause, not just the remedy: {reason}"
        );

        // The document still draws: the pane resolves, against the binding it
        // last had.
        drive_frame(|ui| show(ui, &mut app, document_id));
        let context = app
            .state
            .ui
            .results
            .persistent_pane_context
            .expect("the document still projects a pane");
        assert_eq!(context.document_id, document_id);
        assert_eq!(
            app.state
                .simulation
                .active_run()
                .expect("an active run")
                .dataset_id,
            good_dataset,
            "the last binding that resolved is the one still on screen"
        );
    }

    /// A refused retarget rebuilds the retained source dataset. Re-trying it
    /// every frame would pay for that rebuild every frame, so the refusal is
    /// held against the candidate that produced it.
    #[test]
    fn a_refused_latest_retarget_is_not_re_attempted_every_frame() {
        let LatestFixture {
            mut app,
            document_id,
            plan_id,
            authored_analysis_id,
            source_digest,
        } = latest_tracking_fixture();
        let project_revision = app.state.workspace.project.revision();
        app.state.simulation.runs.push(authored_run(
            2,
            plan_id,
            project_revision,
            source_digest,
            authored_analysis_id,
            "V(renamed)",
        ));
        let stale_dataset = app.state.simulation.runs[1].dataset_id;

        assert!(matches!(
            refresh_latest_binding(&mut app.state, document_id),
            LatestBinding::Degraded(_)
        ));
        let first = app
            .state
            .ui
            .results
            .latest_retarget_failure(document_id, stale_dataset)
            .expect("the refusal is held against the candidate that caused it")
            .to_owned();
        let revision_after_first = app
            .state
            .workspace
            .visualization_document(document_id)
            .expect("retained document")
            .revision();

        for _ in 0..3 {
            assert!(matches!(
                refresh_latest_binding(&mut app.state, document_id),
                LatestBinding::Degraded(_)
            ));
        }

        assert_eq!(
            app.state
                .ui
                .results
                .latest_retarget_failure(document_id, stale_dataset),
            Some(first.as_str())
        );
        assert_eq!(
            app.state
                .workspace
                .visualization_document(document_id)
                .expect("retained document")
                .revision(),
            revision_after_first,
            "a held refusal must not keep re-transacting against the document"
        );

        // A genuinely new candidate is tried again rather than inheriting the
        // previous refusal.
        assert_eq!(
            app.state
                .ui
                .results
                .latest_retarget_failure(document_id, crate::product::DatasetId::new()),
            None
        );
    }

    #[test]
    fn latest_candidate_requires_completed_success_and_current_project_source_authority() {
        let plan_id = SimulationPlanId::new();
        let revision = ObjectRevision::INITIAL;
        let source = digest(0x52);
        let mut run = completed_prepared_run(plan_id, revision, source);

        assert!(run_matches_current_authority(
            &run,
            plan_id,
            revision,
            Some(source)
        ));
        assert!(!run_matches_current_authority(
            &run,
            plan_id,
            ObjectRevision::new(revision.get() + 1).expect("next revision"),
            Some(source)
        ));
        assert!(!run_matches_current_authority(
            &run,
            plan_id,
            revision,
            Some(digest(0x53))
        ));
        assert!(!run_matches_current_authority(
            &run, plan_id, revision, None
        ));

        run.success = false;
        assert!(!run_matches_current_authority(
            &run,
            plan_id,
            revision,
            Some(source)
        ));
        run.success = true;
        run.lifecycle = SimulationRunLifecycle::Running;
        assert!(!run_matches_current_authority(
            &run,
            plan_id,
            revision,
            Some(source)
        ));
    }

    #[test]
    fn latest_authored_analysis_uses_the_final_expanded_execution_identity() {
        let authored = AnalysisInstanceId::new();
        let first_execution = AnalysisInstanceId::new();
        let final_execution = AnalysisInstanceId::new();
        let mut run = SimulationRun::new(1);
        for (label, execution) in [
            ("PVT point 1/2", first_execution),
            ("PVT point 2/2", final_execution),
        ] {
            run.add_analysis(
                AnalysisResult::new(1, AnalysisType::Transient, label).with_provenance(
                    AnalysisResultProvenance::new_with_authored_source_domain(
                        AnalysisResultSourceDomain::SimulationPlan,
                        execution,
                        authored,
                        ObjectRevision::INITIAL,
                        digest(0x61),
                        Vec::new(),
                    )
                    .expect("expanded analysis provenance"),
                ),
            );
        }

        let selected = latest_successful_authored_analysis(&run, authored)
            .expect("the authored analysis has expanded results");
        assert_eq!(selected.label, "PVT point 2/2");
        assert_eq!(analysis_identity(&run, selected), final_execution);
    }

    fn assert_slots_are_finite_and_bounded(stage: Rect, slots: &[Rect], expected: usize) {
        assert_eq!(slots.len(), expected);
        for slot in slots {
            for value in [
                slot.min.x,
                slot.min.y,
                slot.max.x,
                slot.max.y,
                slot.width(),
                slot.height(),
            ] {
                assert!(value.is_finite(), "pane slot must remain finite: {slot:?}");
            }
            assert!(slot.width() >= 0.0 && slot.height() >= 0.0, "{slot:?}");
            assert!(
                slot.left() >= stage.left() && slot.top() >= stage.top(),
                "{slot:?}"
            );
            assert!(
                slot.right() <= stage.right() && slot.bottom() <= stage.bottom(),
                "{slot:?} exceeds {stage:?}"
            );
        }
    }

    #[test]
    fn multi_pane_slots_never_exceed_narrow_or_short_result_stages() {
        let short = Rect::from_min_size(pos2(17.0, 23.0), vec2(480.0, 19.0));
        let rows = bounded_pane_slots(short, PageLayout::Rows, 7);
        assert_slots_are_finite_and_bounded(short, &rows, 7);

        let narrow = Rect::from_min_size(pos2(3.0, 5.0), vec2(13.0, 360.0));
        let columns = bounded_pane_slots(narrow, PageLayout::Columns, 8);
        assert_slots_are_finite_and_bounded(narrow, &columns, 8);

        let compact = Rect::from_min_size(pos2(11.0, 13.0), vec2(29.0, 17.0));
        let grid = bounded_pane_slots(compact, PageLayout::Grid { columns: 3 }, 11);
        assert_slots_are_finite_and_bounded(compact, &grid, 11);
    }

    #[test]
    fn pane_slot_geometry_sanitizes_nonfinite_available_extents() {
        let size = finite_stage_size(vec2(f32::INFINITY, f32::NAN));
        assert_eq!(size, vec2(0.0, 0.0));
        let stage = Rect::from_min_size(pos2(0.0, 0.0), size);
        let slots = bounded_pane_slots(stage, PageLayout::Grid { columns: 2 }, 4);
        assert_slots_are_finite_and_bounded(stage, &slots, 4);
    }

    #[test]
    fn exact_renderer_mapping_exposes_only_implemented_catalog_viewers() {
        assert_eq!(
            ResultViewer::from_viewer_document_id("viewer-waveform"),
            Some(ResultViewer::Waves)
        );
        assert_eq!(
            ResultViewer::from_viewer_document_id("viewer-table"),
            Some(ResultViewer::Table)
        );
        assert_eq!(
            ResultViewer::from_viewer_document_id("viewer-phase-noise"),
            Some(ResultViewer::PhaseNoise)
        );
        assert_eq!(
            ResultViewer::from_viewer_document_id("viewer-manifest"),
            None
        );
        assert_eq!(ResultViewer::from_viewer_document_id("manifest"), None);
        assert_eq!(
            ResultViewer::from_viewer_document_id("field-viewer-3d"),
            None
        );
    }

    #[test]
    fn every_release_target_has_an_exact_native_renderer_identity() {
        let release_targets = crate::results::viewer_catalog::VIEWER_DOCUMENTS
            .iter()
            .filter(|viewer| viewer.release == ViewerReleaseClass::ReleaseTarget)
            .collect::<Vec<_>>();
        assert_eq!(release_targets.len(), 13);
        for viewer in release_targets {
            let native = ResultViewer::from_viewer_document_id(viewer.id);
            assert!(
                native.is_some(),
                "release-target viewer {} has no native Results renderer",
                viewer.id
            );
            assert_eq!(
                native.and_then(ResultViewer::viewer_document_id),
                Some(viewer.id),
                "release-target viewer {} does not round-trip its canonical identity",
                viewer.id
            );
        }
    }

    #[test]
    fn active_pane_identity_is_retained_only_when_it_belongs_to_the_page() {
        assert_eq!(resolved_active_pane_id(Some(7), [3_u64, 7, 11]), Some(7));
        assert_eq!(resolved_active_pane_id(Some(99), [3_u64, 7, 11]), Some(3));
        assert_eq!(resolved_active_pane_id(None, [3_u64, 7, 11]), Some(3));
        assert_eq!(resolved_active_pane_id(Some(7), []), None);
    }

    #[test]
    fn inactive_panes_use_retained_viewers_and_never_the_global_viewer() {
        assert_eq!(
            pane_viewer(true, Some(ResultViewer::Eye), "viewer-waveform"),
            Some(ResultViewer::Eye)
        );
        assert_eq!(
            pane_viewer(false, Some(ResultViewer::Eye), "viewer-waveform"),
            Some(ResultViewer::Waves)
        );
        assert_eq!(
            pane_viewer(false, Some(ResultViewer::Eye), "viewer-manifest"),
            None
        );
    }

    #[test]
    fn retained_frequency_document_restores_its_noise_projection() {
        let noise = AnalysisResult::new(3, AnalysisType::Noise, "noise").with_waveforms(vec![
            crate::state::WaveformData::new(
                "inoise",
                vec![1.0, 10.0],
                vec![1.0e-9, 2.0e-9],
                "#fff",
            ),
        ]);
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(noise);
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));

        assert_eq!(
            bound_viewer_projection(&state, ResultViewer::Bode),
            ResultViewer::NoiseContrib
        );
    }

    #[test]
    fn interactive_result_viewers_keep_canonical_document_identity() {
        for viewer in [
            ResultViewer::Waves,
            ResultViewer::DcSweep,
            ResultViewer::Bode,
            ResultViewer::Fft,
            ResultViewer::HarmonicBalance,
            ResultViewer::PhaseNoise,
            ResultViewer::Eye,
            ResultViewer::Hist,
            ResultViewer::Op,
            ResultViewer::NoiseContrib,
            ResultViewer::Contribution,
            ResultViewer::TransferFunction,
            ResultViewer::Specs,
            ResultViewer::Table,
            ResultViewer::Nyquist,
            ResultViewer::Smith,
            ResultViewer::PoleZero,
            ResultViewer::Events,
            ResultViewer::Soa,
            ResultViewer::Reliability,
            ResultViewer::Optimization,
        ] {
            let document_id = viewer
                .viewer_document_id()
                .expect("interactive viewers have catalog identities");
            assert!(
                viewer_document(document_id).is_some(),
                "{viewer:?} mapped to unknown canonical document {document_id}"
            );
        }
    }

    #[test]
    fn persistent_renderer_contract_never_substitutes_a_broader_catalog_mode() {
        let transient =
            AnalysisResult::new(1, AnalysisType::Transient, "tran").with_waveforms(vec![
                crate::state::WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff"),
            ]);
        let hb = AnalysisResult::new(2, AnalysisType::HarmonicBalance, "hb").with_waveforms(vec![
            crate::state::WaveformData::new("V(out)", vec![1.0, 2.0], vec![1.0, 0.5], "#fff"),
        ]);
        let noise = AnalysisResult::new(3, AnalysisType::Noise, "noise").with_waveforms(vec![
            crate::state::WaveformData::new(
                "onoise",
                vec![1.0, 10.0],
                vec![1.0e-18, 1.0e-19],
                "#fff",
            ),
        ]);
        let invalid_noise = AnalysisResult::new(4, AnalysisType::Noise, "invalid noise")
            .with_waveforms(vec![crate::state::WaveformData::new(
                "onoise",
                vec![1.0, 10.0],
                vec![1.0e-18, -1.0e-19],
                "#fff",
            )]);

        assert!(renderer_supports_analysis("viewer-spectrum", &transient));
        assert!(!renderer_supports_analysis("viewer-spectrum", &hb));
        assert!(renderer_supports_analysis("viewer-bode", &noise));
        assert!(!renderer_supports_analysis("viewer-bode", &invalid_noise));
        assert!(!renderer_supports_analysis("viewer-phase-noise", &noise));
    }
}
