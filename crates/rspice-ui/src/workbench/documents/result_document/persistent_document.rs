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
    if let Err(reason) = refresh_latest_binding(&mut app.state, document_id) {
        unavailable_surface(ui, "Result document tracking blocked", &reason);
        return;
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

fn refresh_latest_binding(
    state: &mut AppState,
    document_id: ResultDocumentId,
) -> Result<(), String> {
    use crate::results::visualization_document::{DocumentEdit, ResultDocumentTrackingMode};

    let document = state
        .workspace
        .visualization_document(document_id)
        .ok_or_else(|| "The selected project result document no longer exists.".to_owned())?;
    let tracking = document.tracking();
    if tracking.mode != ResultDocumentTrackingMode::Latest {
        return Ok(());
    }
    let plan_id = tracking
        .simulation_plan_id
        .ok_or_else(|| "Latest tracking has no exact simulation-plan identity.".to_owned())?;
    let authored_analysis_id = tracking
        .authored_analysis_id
        .ok_or_else(|| "Latest tracking has no exact authored-analysis identity.".to_owned())?;
    let previous = document
        .panes()
        .iter()
        .filter_map(|pane| pane.binding)
        .next()
        .ok_or_else(|| "The result document has no retained pane binding.".to_owned())?
        .dataset;
    let project_revision = state.workspace.project.revision();
    let source_digest = current_result_source_digest(state);

    let Some((run, analysis)) = state
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
    else {
        // Latest tracking is non-destructive. When no authenticated current
        // producer exists, keep the last immutable binding readable; the
        // Results toolbar marks it Historical until a matching completed run
        // can advance it again.
        return Ok(());
    };
    let next = crate::product::DatasetBinding::new(run.dataset_id, run.dataset_content_digest());
    if next == previous {
        return Ok(());
    }
    let next_source =
        super::create_document::source_dataset(run, analysis).map_err(|error| error.to_string())?;
    let revision = state
        .workspace
        .visualization_document(document_id)
        .ok_or_else(|| "The selected project result document is no longer retained.".to_owned())?
        .revision();
    state
        .workspace
        .transact_visualization_document(
            document_id,
            revision,
            vec![DocumentEdit::RetargetTrackedDataset {
                previous,
                next: next_source,
                analysis_id: analysis_identity(run, analysis),
            }],
        )
        .map_err(|error| error.to_string())?;
    Ok(())
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
        let id = u32::try_from(marker.id.get())
            .map_err(|_| "A retained marker identity exceeds the renderer range.".to_owned())?;
        let kind = match marker.kind {
            crate::results::visualization_document::PlotMarkerKind::PointNote
            | crate::results::visualization_document::PlotMarkerKind::MeasurementAnchor => {
                super::MarkerKind::Note
            }
            crate::results::visualization_document::PlotMarkerKind::Peak => super::MarkerKind::Peak,
            crate::results::visualization_document::PlotMarkerKind::SpecificationLine => {
                super::MarkerKind::Spec
            }
        };
        markers.push(super::ResultMarker {
            id,
            analysis,
            anchor,
            trace_name: trace.label.clone(),
            x: *x,
            kind,
            note: marker.label.clone(),
        });
    }
    state.ui.results.project_document_markers(markers);
    Ok(())
}

fn capture_pane_presentation(state: &mut AppState, pane: &PaneProjection, viewer: ResultViewer) {
    let view = state.ui.results.persistent_plot_view(viewer);
    let cursors = state.ui.results.cursors;
    let result_markers = state.ui.results.markers.clone();
    let Some(document) = state.workspace.visualization_document(pane.document_id) else {
        return;
    };
    let revision = document.revision();
    let mut edits = Vec::new();
    let mut new_marker_session_ids = Vec::new();
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
    let marker_kind = |kind| match kind {
        super::MarkerKind::Note => {
            crate::results::visualization_document::PlotMarkerKind::PointNote
        }
        super::MarkerKind::Peak => crate::results::visualization_document::PlotMarkerKind::Peak,
        super::MarkerKind::Spec => {
            crate::results::visualization_document::PlotMarkerKind::SpecificationLine
        }
    };
    for marker in document
        .markers()
        .iter()
        .filter(|marker| marker.pane_id == pane.id)
    {
        let projected_id = u32::try_from(marker.id.get()).ok();
        let Some(projected) =
            projected_id.and_then(|id| result_markers.iter().find(|projected| projected.id == id))
        else {
            edits.push(DocumentEdit::Remove(EntityRef::Marker(marker.id)));
            continue;
        };
        let kind = marker_kind(projected.kind);
        if marker.coordinate != TypedValue::Real(projected.x)
            || marker.label != projected.note
            || marker.kind != kind
        {
            edits.push(DocumentEdit::SetMarker {
                marker_id: marker.id,
                coordinate: TypedValue::Real(projected.x),
                label: projected.note.clone(),
                kind,
                scope: marker.scope,
                source_specification: marker.source_specification.clone(),
            });
        }
    }
    for marker in &result_markers {
        let already_retained = document.markers().iter().any(|retained| {
            retained.pane_id == pane.id && u32::try_from(retained.id.get()).ok() == Some(marker.id)
        });
        if already_retained {
            continue;
        }
        let Some(trace) = document
            .traces()
            .iter()
            .find(|trace| trace.pane_id == pane.id && trace.label == marker.trace_name)
        else {
            continue;
        };
        edits.push(DocumentEdit::AddTypedMarker {
            pane_id: pane.id,
            trace_id: trace.id,
            coordinate: TypedValue::Real(marker.x),
            label: marker.note.clone(),
            kind: marker_kind(marker.kind),
            scope: crate::results::visualization_document::PlotMarkerScope::Pane,
            source_specification: None,
        });
        new_marker_session_ids.push(marker.id);
    }
    if edits.is_empty() {
        return;
    }
    match state
        .workspace
        .transact_visualization_document(pane.document_id, revision, edits)
    {
        Ok(receipt) => {
            let retained_ids = receipt
                .created
                .into_iter()
                .filter_map(|entity| match entity {
                    EntityRef::Marker(id) => u32::try_from(id.get()).ok(),
                    _ => None,
                });
            for (session_id, retained_id) in new_marker_session_ids.into_iter().zip(retained_ids) {
                if let Some(marker) = state.ui.results.marker_mut(session_id) {
                    marker.id = retained_id;
                }
            }
        }
        Err(error) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "Could not retain Results pane presentation: {error}"
            )));
        }
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
