//! Project-owned Results document projection.
//!
//! This module never invents a runtime document. It resolves each retained
//! page and pane directly from [`VisualizationDocument`], verifies the pane's
//! immutable dataset digest and analysis identity, and then delegates only to
//! an existing renderer with the same canonical viewer contract. A document
//! can only be created with a registered renderer; migrated or damaged
//! documents carrying an unknown viewer identity fail closed.

use egui::{Align, Layout, RichText, Sense, Ui, UiBuilder, vec2};

use crate::product::{AnalysisInstanceId, ResultDocumentId};
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

    let Some((run, analysis)) = state
        .simulation
        .runs
        .iter()
        .filter(|run| {
            run.lifecycle.is_terminal()
                && run.success
                && run
                    .prepared_receipt()
                    .and_then(|receipt| receipt.simulation_plan_id())
                    == Some(plan_id)
        })
        .filter_map(|run| {
            run.analyses
                .iter()
                .find(|analysis| {
                    analysis.success
                        && analysis.provenance().is_some_and(|provenance| {
                            provenance.source_instance_id() == authored_analysis_id
                        })
                })
                .map(|analysis| (run, analysis))
        })
        .max_by(|(left, _), (right, _)| {
            left.id
                .cmp(&right.id)
                .then_with(|| left.timestamp.total_cmp(&right.timestamp))
                .then_with(|| left.dataset_id.as_uuid().cmp(&right.dataset_id.as_uuid()))
        })
    else {
        return Err(
            "No terminal retained run matches this document's exact plan and authored analysis."
                .to_owned(),
        );
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
                analysis_id: authored_analysis_id,
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
    match page.page.layout {
        PageLayout::SinglePane if page.panes.len() == 1 => {
            render_single(ui, app, &page.panes[0], active_pane_id, active_viewer)
        }
        PageLayout::SinglePane => {
            unavailable_surface(
                ui,
                &page.page.title,
                "A single-pane page must resolve to exactly one retained pane.",
            );
            None
        }
        PageLayout::Rows => render_rows(ui, app, &page.panes, active_pane_id, active_viewer),
        PageLayout::Columns => render_columns(ui, app, &page.panes, active_pane_id, active_viewer),
        PageLayout::Grid { columns } => render_grid(
            ui,
            app,
            &page.panes,
            usize::from(columns.max(1)),
            active_pane_id,
            active_viewer,
        ),
    }
}

fn render_single(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    pane: &Pane,
    active_pane_id: u64,
    active_viewer: Option<ResultViewer>,
) -> Option<u64> {
    let size = ui.available_size();
    ui.allocate_ui_with_layout(size, Layout::top_down(Align::Min), |ui| {
        render_pane(ui, app, pane, active_pane_id, active_viewer)
    })
    .inner
}

fn render_rows(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    panes: &[Pane],
    active_pane_id: u64,
    active_viewer: Option<ResultViewer>,
) -> Option<u64> {
    let count = panes.len().max(1) as f32;
    let gap = 1.0;
    let pane_height = ((ui.available_height() - gap * (count - 1.0)) / count).max(80.0);
    let width = ui.available_width();
    let mut activated_pane_id = None;
    for (index, pane) in panes.iter().enumerate() {
        let activated = ui
            .allocate_ui_with_layout(
                vec2(width, pane_height),
                Layout::top_down(Align::Min),
                |ui| render_pane(ui, app, pane, active_pane_id, active_viewer),
            )
            .inner;
        activated_pane_id = activated.or(activated_pane_id);
        if index + 1 < panes.len() {
            ui.allocate_space(vec2(width, gap));
        }
    }
    activated_pane_id
}

fn render_columns(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    panes: &[Pane],
    active_pane_id: u64,
    active_viewer: Option<ResultViewer>,
) -> Option<u64> {
    let count = panes.len().max(1) as f32;
    let gap = 1.0;
    let pane_width = ((ui.available_width() - gap * (count - 1.0)) / count).max(120.0);
    let height = ui.available_height();
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        let mut activated_pane_id = None;
        for pane in panes {
            let activated = ui
                .allocate_ui_with_layout(
                    vec2(pane_width, height),
                    Layout::top_down(Align::Min),
                    |ui| render_pane(ui, app, pane, active_pane_id, active_viewer),
                )
                .inner;
            activated_pane_id = activated.or(activated_pane_id);
        }
        activated_pane_id
    })
    .inner
}

fn render_grid(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    panes: &[Pane],
    columns: usize,
    active_pane_id: u64,
    active_viewer: Option<ResultViewer>,
) -> Option<u64> {
    let rows = panes.len().div_ceil(columns).max(1);
    let horizontal_gap = 1.0;
    let vertical_gap = 1.0;
    let pane_width = ((ui.available_width() - horizontal_gap * (columns.saturating_sub(1) as f32))
        / columns as f32)
        .max(120.0);
    let pane_height = ((ui.available_height() - vertical_gap * (rows.saturating_sub(1) as f32))
        / rows as f32)
        .max(80.0);
    ui.spacing_mut().item_spacing.y = vertical_gap;
    let mut activated_pane_id = None;
    for row in panes.chunks(columns) {
        let activated = ui
            .horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = horizontal_gap;
                let mut activated_pane_id = None;
                for pane in row {
                    let activated = ui
                        .allocate_ui_with_layout(
                            vec2(pane_width, pane_height),
                            Layout::top_down(Align::Min),
                            |ui| render_pane(ui, app, pane, active_pane_id, active_viewer),
                        )
                        .inner;
                    activated_pane_id = activated.or(activated_pane_id);
                }
                activated_pane_id
            })
            .inner;
        activated_pane_id = activated.or(activated_pane_id);
    }
    activated_pane_id
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
            .max_rect(rect.shrink(1.0))
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
        state.ui.results.viewer = viewer;
    }
    Ok(())
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
        ResultViewer::Waves => "viewer-waveform",
        ResultViewer::Bode | ResultViewer::NoiseContrib | ResultViewer::Nyquist => "viewer-bode",
        ResultViewer::Fft => "viewer-spectrum",
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
            analysis.analysis_type == AnalysisType::Transient && !analysis.waveforms.is_empty()
        }
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

    #[test]
    fn exact_renderer_mapping_never_substitutes_unimplemented_catalog_viewers() {
        assert_eq!(
            renderer_for_viewer_document("viewer-waveform"),
            Some(ResultViewer::Waves)
        );
        assert_eq!(
            renderer_for_viewer_document("viewer-table"),
            Some(ResultViewer::Table)
        );
        assert_eq!(renderer_for_viewer_document("viewer-phase-noise"), None);
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
    fn interactive_result_viewers_keep_canonical_document_identity() {
        for viewer in [
            ResultViewer::Waves,
            ResultViewer::Bode,
            ResultViewer::Fft,
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
