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
use crate::results::viewer_catalog::viewer_document;
use crate::results::visualization_document::{
    Page, PageLayout, Pane, PaneDataBinding, PanePlacement,
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
    panes: Vec<Pane>,
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
            renderer_for_viewer_document(&restored_pane.viewer_id)
        } else {
            active_viewer
        };
        if let Some(viewer) = restored_viewer {
            app.state.ui.results.viewer = viewer;
            if let Some(viewer_document_id) = viewer_document_id(viewer) {
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
    renderer_for_viewer_document(&pane.viewer_id).filter(|viewer| {
        super::family_allows_viewer(family_label, *viewer)
            && super::viewer_is_available(state, *viewer)
    })
}

fn select_global_viewer(state: &mut AppState, viewer: ResultViewer) {
    state.ui.results.viewer = viewer;
    if let Some(viewer_document_id) = viewer_document_id(viewer) {
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
    let document = state
        .workspace
        .visualization_documents
        .iter_mut()
        .find(|document| document.id() == document_id)
        .ok_or_else(|| "The selected project result document is no longer retained.".to_owned())?;
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::RetargetTrackedDataset {
                previous,
                next: next_source,
                analysis_id: analysis_identity(run, analysis),
            }],
        )
        .map_err(|error| error.to_string())?;
    state.workspace.visualization_documents_dirty = true;
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
    if let Some(viewer) = renderer_for_viewer_document(&pane.viewer_id) {
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
                .cloned()
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
    pane: &Pane,
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

fn pane_viewer(
    is_active: bool,
    active_viewer: Option<ResultViewer>,
    retained_viewer_id: &str,
) -> Option<ResultViewer> {
    if is_active {
        active_viewer
    } else {
        renderer_for_viewer_document(retained_viewer_id)
    }
}

fn pane_header(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    pane: &Pane,
    viewer: Option<ResultViewer>,
    is_active: bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let viewer_title = viewer.map_or_else(
        || pane.viewer_id.as_str(),
        |viewer| {
            viewer_document_id(viewer)
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
        .clicked();
    clicked && select_pane_context(&mut app.state, pane).is_ok()
}

fn render_pane_viewer(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    pane: &Pane,
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
        let Some(viewer_document_id) = viewer_document_id(viewer) else {
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
    super::show_persistent_pane_viewer(ui, app, viewer);
}

fn select_pane_context(state: &mut AppState, pane: &Pane) -> Result<(), String> {
    select_pane_binding(state, pane)?;
    state.workbench.visualization_studio.active_pane = Some(pane.id.get());
    state
        .workbench
        .visualization_studio
        .selected_viewer_document = pane.viewer_id.clone();
    if let Some(viewer) = renderer_for_viewer_document(&pane.viewer_id) {
        state.ui.results.viewer = bound_viewer_projection(state, viewer);
    }
    Ok(())
}

fn bound_viewer_projection(state: &AppState, viewer: ResultViewer) -> ResultViewer {
    if viewer == ResultViewer::Waves
        && state
            .simulation
            .active_analysis()
            .is_some_and(|analysis| analysis.analysis_type == AnalysisType::DcSweep)
    {
        ResultViewer::DcSweep
    } else if viewer == ResultViewer::Fft
        && state
            .simulation
            .active_analysis()
            .is_some_and(|analysis| analysis.analysis_type == AnalysisType::HarmonicBalance)
    {
        ResultViewer::HarmonicBalance
    } else if viewer == ResultViewer::Bode
        && state
            .simulation
            .active_analysis()
            .is_some_and(super::bode::ordinary_noise_spectrum_is_renderable)
    {
        ResultViewer::NoiseContrib
    } else {
        viewer
    }
}

fn select_pane_binding(state: &mut AppState, pane: &Pane) -> Result<(), String> {
    let binding = pane
        .binding
        .ok_or_else(|| "This result pane has no immutable dataset binding.".to_owned())?;
    let (run_index, analysis_index) = resolve_binding(state, binding)?;
    if !state.simulation.select_run(run_index) {
        return Err("The retained dataset could not be selected.".to_owned());
    }
    if !state.simulation.select_analysis(analysis_index) {
        return Err("The retained analysis could not be selected.".to_owned());
    }
    Ok(())
}

const fn viewer_document_id(viewer: ResultViewer) -> Option<&'static str> {
    Some(match viewer {
        ResultViewer::Manifest => return None,
        ResultViewer::Waves | ResultViewer::DcSweep => "viewer-waveform",
        ResultViewer::Bode | ResultViewer::NoiseContrib | ResultViewer::Nyquist => "viewer-bode",
        ResultViewer::Fft | ResultViewer::HarmonicBalance => "viewer-spectrum",
        ResultViewer::PhaseNoise => "viewer-phase-noise",
        ResultViewer::Eye => "eye-viewer",
        ResultViewer::Hist => "viewer-histogram",
        ResultViewer::Op | ResultViewer::Specs | ResultViewer::Table => "viewer-table",
        ResultViewer::Contribution => "viewer-contribution",
        ResultViewer::TransferFunction => "viewer-transfer-function",
        ResultViewer::Smith => "viewer-smith",
        ResultViewer::PoleZero => "viewer-pz",
    })
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

pub(super) fn renderer_for_viewer_document(id: &str) -> Option<ResultViewer> {
    match id {
        "viewer-waveform" => Some(ResultViewer::Waves),
        "viewer-bode" => Some(ResultViewer::Bode),
        "viewer-spectrum" => Some(ResultViewer::Fft),
        "viewer-phase-noise" => Some(ResultViewer::PhaseNoise),
        "viewer-smith" => Some(ResultViewer::Smith),
        "viewer-table" => Some(ResultViewer::Table),
        "viewer-histogram" => Some(ResultViewer::Hist),
        "eye-viewer" => Some(ResultViewer::Eye),
        "viewer-pz" => Some(ResultViewer::PoleZero),
        "viewer-contribution" => Some(ResultViewer::Contribution),
        "viewer-transfer-function" => Some(ResultViewer::TransferFunction),
        _ => None,
    }
}

/// Whether the registered persistent renderer can truthfully present this
/// exact retained analysis. Catalog compatibility is intentionally broader:
/// one catalog document can own several specialist modes, while the current
/// native renderer may implement only a subset. Creation must never bind a
/// broader catalog promise to a narrower renderer and silently substitute the
/// resulting presentation.
pub(super) fn renderer_supports_analysis(id: &str, analysis: &AnalysisResult) -> bool {
    if renderer_for_viewer_document(id).is_none() {
        return false;
    }
    match id {
        "viewer-waveform" => {
            matches!(
                analysis.analysis_type,
                AnalysisType::Transient | AnalysisType::DcSweep
            ) && !analysis.waveforms.is_empty()
        }
        "viewer-bode" => {
            (analysis.analysis_type == AnalysisType::Ac && !analysis.waveforms.is_empty())
                || super::bode::ordinary_noise_spectrum_is_renderable(analysis)
        }
        // The native spectrum renderer currently derives an FFT from retained
        // transient samples. It does not reinterpret HB/PSS tone tables as an
        // FFT document.
        "viewer-spectrum" => {
            (analysis.analysis_type == AnalysisType::Transient && !analysis.waveforms.is_empty())
                || super::harmonic_balance_analysis_is_renderable(analysis)
        }
        "viewer-phase-noise" => super::phase_noise_analysis_is_renderable(analysis),
        "viewer-smith" => {
            analysis.analysis_type == AnalysisType::SParameter && !analysis.waveforms.is_empty()
        }
        "viewer-table" => !analysis.waveforms.is_empty(),
        "viewer-histogram" => analysis.analysis_type == AnalysisType::MonteCarlo,
        "eye-viewer" => {
            analysis.analysis_type == AnalysisType::Transient && !analysis.waveforms.is_empty()
        }
        "viewer-pz" => analysis.analysis_type == AnalysisType::PoleZero,
        "viewer-contribution" => matches!(
            analysis.analysis_type,
            AnalysisType::Sensitivity | AnalysisType::DcMismatch
        ),
        "viewer-transfer-function" => analysis.analysis_type == AnalysisType::Tf,
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

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
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
        run.restore_provenance(SimulationRunProvenance::Prepared(receipt))
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
            renderer_for_viewer_document("viewer-waveform"),
            Some(ResultViewer::Waves)
        );
        assert_eq!(
            renderer_for_viewer_document("viewer-table"),
            Some(ResultViewer::Table)
        );
        assert_eq!(
            renderer_for_viewer_document("viewer-phase-noise"),
            Some(ResultViewer::PhaseNoise)
        );
        assert_eq!(renderer_for_viewer_document("viewer-manifest"), None);
        assert_eq!(renderer_for_viewer_document("manifest"), None);
        assert_eq!(renderer_for_viewer_document("field-viewer-3d"), None);
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
        ] {
            let document_id =
                viewer_document_id(viewer).expect("interactive viewers have catalog identities");
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
