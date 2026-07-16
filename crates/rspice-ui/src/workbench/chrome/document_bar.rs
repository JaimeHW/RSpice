//! Workspace-scoped document tabs. Stable project identities drive every tab;
//! the shell never fabricates datasets, source files, or verification records.

use egui::containers::menu::MenuButton;
use egui::{Context, Frame, Layout, Sense, TopBottomPanel, Vec2};

use crate::common::{AppState, RSpiceApp};
use crate::state::ViewType;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::design_system::WorkbenchIcon;
use super::super::layout::LayoutSpec;
use super::super::state::{Workspace, WorkspaceDocumentId};

const TAB_MAX_WIDTH: f32 = 190.0;
const TAB_PADDING_X: f32 = 10.0;
const TAB_ICON_SIZE: f32 = 16.0;
const TAB_CONTENT_GAP: f32 = 7.0;
const TAB_CLOSE_SIZE: f32 = 16.0;
const TAB_DIRTY_SIZE: f32 = 6.0;
const TAB_ACTIVE_EDGE: f32 = 2.0;
const OVERFLOW_WIDTH: f32 = 31.0;

pub fn show(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    reconcile_document_registry(&mut app.state);
    let documents = visible_documents(&app.state);
    if !document_strip_visible(documents.len()) {
        return;
    }

    let t = Tokens::get(ctx);
    TopBottomPanel::top("workbench.document_bar")
        .exact_height(layout.document_bar_height)
        .frame(Frame::new().fill(t.color.bg_app))
        .show_separator_line(false)
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let rect = ui.max_rect();
            let overflow_width = if t.metrics.ctl_h >= 44.0 {
                44.0
            } else {
                OVERFLOW_WIDTH
            };
            ui.painter().hline(
                rect.x_range(),
                (rect.bottom() - 0.5).max(rect.top()),
                egui::Stroke::new(1.0, t.color.border),
            );
            ui.horizontal(|ui| {
                let tabs_width = (ui.available_width() - overflow_width).max(1.0);
                ui.allocate_ui_with_layout(
                    Vec2::new(tabs_width, layout.document_bar_height),
                    Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        egui::ScrollArea::horizontal()
                            .id_salt("workbench.document_tabs.scroll")
                            .scroll_bar_visibility(
                                egui::scroll_area::ScrollBarVisibility::AlwaysHidden,
                            )
                            .show(ui, |ui| {
                                let tabs = ui.horizontal(|ui| {
                                    document_tabs(
                                        ui,
                                        &mut app.state,
                                        &documents,
                                        layout.document_bar_height,
                                    );
                                });
                                ui.ctx().accesskit_node_builder(tabs.response.id, |node| {
                                    node.set_role(egui::accesskit::Role::TabList);
                                    node.set_label("Open documents");
                                });
                            });
                    },
                );
                document_overflow(
                    ui,
                    &mut app.state,
                    &documents,
                    layout.document_bar_height,
                    overflow_width,
                );
            });
        });
}

pub(in crate::workbench) fn is_visible(app: &RSpiceApp) -> bool {
    document_strip_visible(visible_documents(&app.state).len())
}

#[derive(Debug, Clone)]
struct WorkspaceDocument {
    id: WorkspaceDocumentId,
    label: String,
    icon: WorkbenchIcon,
    dirty: bool,
}

fn available_documents(state: &AppState) -> Vec<WorkspaceDocument> {
    match state.workbench.workspace {
        Workspace::Project => vec![WorkspaceDocument {
            id: WorkspaceDocumentId::Project,
            label: "Project overview".to_owned(),
            icon: WorkbenchIcon::Project,
            dirty: false,
        }],
        Workspace::Design => state
            .workspace
            .open_views
            .iter()
            .map(|document| WorkspaceDocument {
                id: WorkspaceDocumentId::CellView(document.reference.clone()),
                label: format!("{} · {}", document.reference.cell, document.reference.view),
                icon: icon_for_view(document.view_type),
                dirty: document.dirty,
            })
            .collect(),
        Workspace::Simulate => {
            let mut documents = vec![WorkspaceDocument {
                id: WorkspaceDocumentId::SimulationPlan,
                label: "Simulation plan".to_owned(),
                icon: WorkbenchIcon::Simulate,
                dirty: false,
            }];
            if let Ok(plan) = state.sim_setup.stable_analysis_plan() {
                documents.extend(plan.instances().iter().map(|instance| WorkspaceDocument {
                    id: WorkspaceDocumentId::AnalysisSetup(instance.id()),
                    label: format!(
                        "{} · setup",
                        instance.kind().stable_id().to_ascii_uppercase()
                    ),
                    icon: WorkbenchIcon::Sliders,
                    dirty: false,
                }));
            }
            documents
        }
        Workspace::Results => state
            .simulation
            .runs
            .iter()
            .map(|run| WorkspaceDocument {
                id: WorkspaceDocumentId::ResultDataset(run.dataset_id),
                label: run.label.clone(),
                icon: WorkbenchIcon::Results,
                dirty: false,
            })
            .collect(),
        Workspace::Verify => vec![WorkspaceDocument {
            id: WorkspaceDocumentId::Verification,
            label: "Verification".to_owned(),
            icon: WorkbenchIcon::Verify,
            dirty: false,
        }],
        Workspace::Models => vec![WorkspaceDocument {
            id: WorkspaceDocumentId::Models,
            label: "Model & Library Manager".to_owned(),
            icon: WorkbenchIcon::Models,
            dirty: false,
        }],
        Workspace::Netlist => vec![WorkspaceDocument {
            id: WorkspaceDocumentId::NetlistSource,
            label: netlist_document_label(state),
            icon: WorkbenchIcon::Netlist,
            dirty: state.workspace.netlist_source_dirty,
        }],
    }
}

fn netlist_document_label(state: &AppState) -> String {
    state
        .workspace
        .netlist_source_path
        .as_deref()
        .and_then(std::path::Path::file_name)
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| format!("{}.sp · generated", state.workspace.project.top_cell))
}

fn authoritative_active_document(
    state: &AppState,
    available: &[WorkspaceDocument],
) -> Option<WorkspaceDocumentId> {
    let candidate = match state.workbench.workspace {
        Workspace::Design => Some(WorkspaceDocumentId::CellView(
            state.workspace.active_view.clone(),
        )),
        Workspace::Results => state
            .simulation
            .active_run()
            .map(|run| WorkspaceDocumentId::ResultDataset(run.dataset_id)),
        workspace => state.workbench.documents.active(workspace).cloned(),
    };
    candidate
        .filter(|candidate| available.iter().any(|document| document.id == *candidate))
        .or_else(|| available.first().map(|document| document.id.clone()))
}

fn visible_documents(state: &AppState) -> Vec<WorkspaceDocument> {
    let available = available_documents(state);
    let active = authoritative_active_document(state, &available);
    available
        .into_iter()
        .enumerate()
        .filter(|(index, document)| {
            *index == 0
                || !state.workbench.documents.is_closed(&document.id)
                || active.as_ref() == Some(&document.id)
        })
        .map(|(_, document)| document)
        .collect()
}

fn reconcile_document_registry(state: &mut AppState) {
    let available = available_documents(state);
    let workspace = state.workbench.workspace;
    state.workbench.documents.retain_available(
        workspace,
        available.iter().map(|document| document.id.clone()),
    );
    if let Some(active) = authoritative_active_document(state, &available) {
        state.workbench.documents.activate(active);
    }
}

fn document_strip_visible(document_count: usize) -> bool {
    document_count > 1
}

fn document_tabs(
    ui: &mut egui::Ui,
    state: &mut AppState,
    documents: &[WorkspaceDocument],
    height: f32,
) {
    let active = authoritative_active_document(state, documents);
    let mut open = None;
    let mut close = None;
    let mut focus_index = None;
    let mut tab_ids = Vec::with_capacity(documents.len());

    for (index, document) in documents.iter().enumerate() {
        let selected = active.as_ref() == Some(&document.id);
        let tab = ui
            .push_id(("document", &document.id), |ui| {
                document_tab(
                    ui,
                    DocumentTabPresentation {
                        icon: document.icon,
                        label: &document.label,
                        selected,
                        dirty: document.dirty,
                        closable: index > 0,
                        height,
                    },
                )
            })
            .inner;
        tab_ids.push(tab.response.id);

        if tab.close_clicked {
            close = Some((index, document.id.clone()));
        } else if tab.response.clicked() {
            focus_index = Some(index);
            if !selected {
                open = Some(document.id.clone());
            }
        }

        if tab.response.has_focus() {
            let key = ui.input_mut(|input| {
                if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
                    Some(TabKey::Previous)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
                    Some(TabKey::Next)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::Home) {
                    Some(TabKey::First)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::End) {
                    Some(TabKey::Last)
                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::Delete) {
                    Some(TabKey::Close)
                } else {
                    None
                }
            });
            match key {
                Some(TabKey::Previous) => {
                    focus_index = Some((index + documents.len() - 1) % documents.len())
                }
                Some(TabKey::Next) => focus_index = Some((index + 1) % documents.len()),
                Some(TabKey::First) => focus_index = Some(0),
                Some(TabKey::Last) => focus_index = Some(documents.len() - 1),
                Some(TabKey::Close) if index > 0 => close = Some((index, document.id.clone())),
                Some(TabKey::Close) | None => {}
            }
        }
    }

    if let Some(index) = focus_index {
        open = Some(documents[index].id.clone());
        ui.memory_mut(|memory| memory.request_focus(tab_ids[index]));
    }
    if let Some(document) = open {
        activate_document(state, &document);
    }
    if let Some((closed_index, document)) = close {
        let focus_after_close = if closed_index + 1 < documents.len() {
            closed_index + 1
        } else {
            closed_index.saturating_sub(1)
        };
        if close_document(state, &document, documents) {
            ui.memory_mut(|memory| memory.request_focus(tab_ids[focus_after_close]));
        }
    }
}

fn activate_document(state: &mut AppState, document: &WorkspaceDocumentId) -> bool {
    if document.workspace() != state.workbench.workspace {
        return false;
    }
    let activated = match document {
        WorkspaceDocumentId::CellView(reference) => {
            if state
                .workspace
                .open_views
                .iter()
                .any(|open| open.reference == *reference)
            {
                state.open_workspace_view(reference.clone());
                true
            } else {
                false
            }
        }
        WorkspaceDocumentId::AnalysisSetup(id) => {
            let Some(position) = state
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .and_then(|plan| {
                    plan.instances()
                        .iter()
                        .position(|instance| instance.id() == *id)
                })
            else {
                return false;
            };
            state.workbench.active_analysis_instance = Some(*id);
            state.workbench.active_analysis = position;
            true
        }
        WorkspaceDocumentId::ResultDataset(dataset_id) => state
            .simulation
            .runs
            .iter()
            .position(|run| run.dataset_id == *dataset_id)
            .is_some_and(|index| state.simulation.select_run(index)),
        WorkspaceDocumentId::Project
        | WorkspaceDocumentId::SimulationPlan
        | WorkspaceDocumentId::Verification
        | WorkspaceDocumentId::Models
        | WorkspaceDocumentId::NetlistSource => true,
    };
    if activated {
        state.workbench.documents.activate(document.clone());
    }
    activated
}

fn close_document(
    state: &mut AppState,
    document: &WorkspaceDocumentId,
    documents: &[WorkspaceDocument],
) -> bool {
    let Some(closed_index) = documents
        .iter()
        .position(|candidate| candidate.id == *document)
    else {
        return false;
    };
    if closed_index == 0 || document.workspace() != state.workbench.workspace {
        return false;
    }
    let fallback = documents
        .get(closed_index + 1)
        .or_else(|| documents.get(closed_index.saturating_sub(1)))
        .map(|document| document.id.clone());

    let closed = match document {
        WorkspaceDocumentId::CellView(_) => {
            if !activate_document(state, document) {
                return false;
            }
            crate::common::project_workflow::close_active_document(state)
        }
        _ => {
            state.workbench.documents.close(document);
            true
        }
    };
    if closed && let Some(fallback) = fallback {
        activate_document(state, &fallback);
    }
    closed
}

#[derive(Clone, Copy)]
enum TabKey {
    Previous,
    Next,
    First,
    Last,
    Close,
}

struct DocumentTabResponse {
    response: egui::Response,
    close_clicked: bool,
}

struct DocumentTabPresentation<'a> {
    icon: WorkbenchIcon,
    label: &'a str,
    selected: bool,
    dirty: bool,
    closable: bool,
    height: f32,
}

fn document_tab(
    ui: &mut egui::Ui,
    presentation: DocumentTabPresentation<'_>,
) -> DocumentTabResponse {
    let DocumentTabPresentation {
        icon,
        label,
        selected,
        dirty,
        closable,
        height,
    } = presentation;
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let intrinsic_label_width = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font.clone(), t.color.text)
        .size()
        .x;
    let suffix_width = suffix_width(dirty, closable);
    let width = (TAB_PADDING_X
        + TAB_ICON_SIZE
        + TAB_CONTENT_GAP
        + intrinsic_label_width
        + suffix_width
        + TAB_PADDING_X)
        .min(TAB_MAX_WIDTH);
    let sense = if selected {
        Sense::click()
    } else {
        Sense::click().difference(Sense::focusable_noninteractive())
    };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), sense);
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            label,
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Tab);
        node.set_selected(selected);
        let description = match (dirty, closable) {
            (true, true) => "Unsaved changes. Press Delete to close.",
            (true, false) => "Unsaved changes.",
            (false, true) => "Press Delete to close.",
            (false, false) => "Pinned primary document.",
        };
        node.set_description(description);
    });

    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.right(), rect.top() + TAB_ACTIVE_EDGE),
            ),
            0.0,
            t.color.accent,
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().vline(
        rect.right(),
        rect.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );

    let icon_center_x = rect.left() + TAB_PADDING_X + TAB_ICON_SIZE * 0.5;
    icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(icon_center_x, rect.center().y),
            Vec2::splat(TAB_ICON_SIZE),
        ),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    let label_x = rect.left() + TAB_PADDING_X + TAB_ICON_SIZE + TAB_CONTENT_GAP;
    let suffix_left = rect.right() - TAB_PADDING_X - suffix_width;
    let label_clip = egui::Rect::from_min_max(
        egui::pos2(label_x, rect.top()),
        egui::pos2(suffix_left.max(label_x), rect.bottom()),
    );
    let painted_label =
        ellipsize_document_label(ui.painter(), label, &font, (suffix_left - label_x).max(0.0));
    let galley = ui
        .painter()
        .layout_no_wrap(painted_label, font, t.color.text);
    ui.painter().with_clip_rect(label_clip).galley(
        egui::pos2(label_x, rect.center().y - galley.size().y * 0.5),
        galley,
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );

    let mut suffix_x = suffix_left;
    if suffix_width > 0.0 {
        suffix_x += TAB_CONTENT_GAP;
    }
    if dirty {
        let center = egui::pos2(suffix_x + TAB_DIRTY_SIZE * 0.5, rect.center().y);
        ui.painter()
            .circle_filled(center, TAB_DIRTY_SIZE * 0.5, t.color.warn);
        suffix_x += TAB_DIRTY_SIZE;
        if closable {
            suffix_x += TAB_CONTENT_GAP;
        }
    }
    let mut close_clicked = false;
    if closable {
        let close_rect = egui::Rect::from_min_size(
            egui::pos2(suffix_x + 2.0, rect.center().y - TAB_CLOSE_SIZE * 0.5),
            Vec2::splat(TAB_CLOSE_SIZE),
        );
        let close_response = ui.interact(
            close_rect,
            response.id.with("close"),
            Sense::click().difference(Sense::focusable_noninteractive()),
        );
        if close_response.hovered() {
            ui.painter().rect_filled(close_rect, 2.0, t.color.bg_active);
        }
        ui.painter().text(
            close_rect.center(),
            egui::Align2::CENTER_CENTER,
            "×",
            theme::sans(14.0, FontWeight::Regular),
            if close_response.hovered() {
                t.color.text
            } else {
                t.color.text_faint
            },
        );
        close_clicked = close_response.clicked();
    }
    theme::paint_focus_ring(ui, &response, rect);
    DocumentTabResponse {
        response,
        close_clicked,
    }
}

fn ellipsize_document_label(
    painter: &egui::Painter,
    value: &str,
    font: &egui::FontId,
    maximum_width: f32,
) -> String {
    if maximum_width <= 0.0 {
        return String::new();
    }
    if painter
        .layout_no_wrap(value.to_owned(), font.clone(), egui::Color32::WHITE)
        .size()
        .x
        <= maximum_width
    {
        return value.to_owned();
    }
    let ellipsis = "\u{2026}";
    if painter
        .layout_no_wrap(ellipsis.to_owned(), font.clone(), egui::Color32::WHITE)
        .size()
        .x
        > maximum_width
    {
        return String::new();
    }
    let characters = value.chars().collect::<Vec<_>>();
    let mut low = 0_usize;
    let mut high = characters.len();
    while low < high {
        let mid = (low + high).div_ceil(2);
        let candidate = characters[..mid]
            .iter()
            .copied()
            .chain(std::iter::once('\u{2026}'))
            .collect::<String>();
        if painter
            .layout_no_wrap(candidate, font.clone(), egui::Color32::WHITE)
            .size()
            .x
            <= maximum_width
        {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    characters[..low]
        .iter()
        .copied()
        .chain(std::iter::once('\u{2026}'))
        .collect()
}

fn suffix_width(dirty: bool, closable: bool) -> f32 {
    match (dirty, closable) {
        (false, false) => 0.0,
        (true, false) => TAB_CONTENT_GAP + TAB_DIRTY_SIZE,
        (false, true) => TAB_CONTENT_GAP + 2.0 + TAB_CLOSE_SIZE,
        (true, true) => TAB_CONTENT_GAP + TAB_DIRTY_SIZE + TAB_CONTENT_GAP + 2.0 + TAB_CLOSE_SIZE,
    }
}

fn document_overflow(
    ui: &mut egui::Ui,
    state: &mut AppState,
    documents: &[WorkspaceDocument],
    height: f32,
    width: f32,
) {
    let t = Tokens::get(ui.ctx());
    let active = authoritative_active_document(state, documents);
    let mut selected = None;
    let (response, _) = ui
        .scope(|ui| {
            ui.spacing_mut().button_padding = Vec2::ZERO;
            MenuButton::from_button(
                egui::Button::new("")
                    .frame(false)
                    .min_size(Vec2::new(width, height.max(t.metrics.ctl_h))),
            )
            .ui(ui, |ui| {
                ui.set_min_width(220.0);
                ui.spacing_mut().item_spacing.y = 0.0;
                for document in documents {
                    if ui
                        .add_sized(
                            [220.0, if height >= 44.0 { 44.0 } else { 29.0 }],
                            egui::Button::selectable(
                                active.as_ref() == Some(&document.id),
                                &document.label,
                            ),
                        )
                        .clicked()
                    {
                        selected = Some(document.id.clone());
                        ui.close();
                    }
                }
            })
        })
        .inner;
    if let Some(document) = selected {
        activate_document(state, &document);
    }
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            "Manage open documents",
        )
    });
    ui.painter().rect_filled(
        response.rect,
        0.0,
        if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_app
        },
    );
    ui.painter().vline(
        response.rect.left(),
        response.rect.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );
    WorkbenchIcon::ChevronDown.paint(
        ui.painter(),
        egui::Rect::from_center_size(response.rect.center(), Vec2::splat(16.0)),
        if response.hovered() {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, response.rect);
    response.on_hover_text("Manage open documents");
}

const fn icon_for_view(view_type: ViewType) -> WorkbenchIcon {
    match view_type {
        ViewType::Schematic | ViewType::Testbench => WorkbenchIcon::Design,
        ViewType::Symbol => WorkbenchIcon::Models,
        ViewType::Spice | ViewType::Verilog | ViewType::VerilogA => WorkbenchIcon::Netlist,
        _ => WorkbenchIcon::File,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::SimulationRun;

    #[test]
    fn strip_is_visible_only_for_multiple_open_documents() {
        assert!(!document_strip_visible(0));
        assert!(!document_strip_visible(1));
        assert!(document_strip_visible(2));
    }

    #[test]
    fn registry_scopes_active_documents_by_workspace() {
        let mut registry = super::super::super::state::WorkspaceDocumentRegistry::default();
        let analysis = crate::product::AnalysisInstanceId::new();
        let dataset = crate::product::DatasetId::new();

        registry.activate(WorkspaceDocumentId::AnalysisSetup(analysis));
        registry.activate(WorkspaceDocumentId::ResultDataset(dataset));

        assert_eq!(
            registry.active(Workspace::Simulate),
            Some(&WorkspaceDocumentId::AnalysisSetup(analysis))
        );
        assert_eq!(
            registry.active(Workspace::Results),
            Some(&WorkspaceDocumentId::ResultDataset(dataset))
        );
        assert!(registry.active(Workspace::Design).is_none());
    }

    #[test]
    fn activating_a_result_document_does_not_force_design_workspace() {
        let mut state = AppState::default();
        state.workbench.workspace = Workspace::Results;
        let run = SimulationRun::new(7);
        let dataset = run.dataset_id;
        state.simulation.runs.push(run);

        assert!(activate_document(
            &mut state,
            &WorkspaceDocumentId::ResultDataset(dataset)
        ));
        assert_eq!(state.workbench.workspace, Workspace::Results);
        assert_eq!(state.simulation.active_run_idx, Some(0));
    }

    #[test]
    fn closing_a_result_tab_keeps_dataset_and_selects_adjacent_document() {
        let mut state = AppState::default();
        state.workbench.workspace = Workspace::Results;
        let first = SimulationRun::new(1);
        let first_dataset = first.dataset_id;
        let second = SimulationRun::new(2);
        let second_dataset = second.dataset_id;
        state.simulation.runs = vec![first, second];
        assert!(state.simulation.select_run(1));
        let documents = visible_documents(&state);

        assert!(close_document(
            &mut state,
            &WorkspaceDocumentId::ResultDataset(second_dataset),
            &documents
        ));
        assert_eq!(
            state.simulation.runs.len(),
            2,
            "closing never deletes result data"
        );
        assert_eq!(state.simulation.active_run_idx, Some(0));
        assert_eq!(
            state.simulation.active_run().unwrap().dataset_id,
            first_dataset
        );
        assert!(
            state
                .workbench
                .documents
                .is_closed(&WorkspaceDocumentId::ResultDataset(second_dataset))
        );
        assert_eq!(visible_documents(&state).len(), 1);
    }

    #[test]
    fn document_tab_geometry_matches_the_mockup() {
        assert_eq!(TAB_MAX_WIDTH, 190.0);
        assert_eq!(TAB_PADDING_X, 10.0);
        assert_eq!(TAB_ICON_SIZE, 16.0);
        assert_eq!(TAB_CONTENT_GAP, 7.0);
        assert_eq!(TAB_CLOSE_SIZE, 16.0);
        assert_eq!(TAB_DIRTY_SIZE, 6.0);
        assert_eq!(OVERFLOW_WIDTH, 31.0);
        assert_eq!(tokens::FS_0, 11.0);
    }

    #[test]
    fn dirty_and_close_suffixes_never_overlap() {
        assert_eq!(suffix_width(false, false), 0.0);
        assert_eq!(suffix_width(true, false), 13.0);
        assert_eq!(suffix_width(false, true), 25.0);
        assert_eq!(suffix_width(true, true), 38.0);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn long_document_labels_are_measured_and_ellipsized() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut fitted = String::new();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let font = theme::sans(tokens::FS_0, FontWeight::Regular);
                fitted = ellipsize_document_label(
                    ui.painter(),
                    "precision_sensor_front_end · schematic",
                    &font,
                    72.0,
                );
                let width = ui
                    .painter()
                    .layout_no_wrap(fitted.clone(), font, egui::Color32::WHITE)
                    .size()
                    .x;
                assert!(width <= 72.0);
            });
        });
        assert!(fitted.ends_with('\u{2026}'));
        assert_ne!(fitted, "precision_sensor_front_end · schematic");
    }
}
