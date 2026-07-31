//! Project-owned engineering report authoring.
//!
//! The surface authors and saves the project-owned, versioned
//! [`ReportDocument`] graph. Route availability remains fail-closed until the
//! complete report workflow is ready for production use.

use std::borrow::Cow;

use egui::{Align2, Color32, Rect, ScrollArea, Sense, Stroke, Ui, Vec2};
use egui_extras::{Column, TableBuilder};

use crate::results::report_document::{
    DataTableBlock, DatasheetBlock, DatasheetField, EvidenceBlock, FigureSizing,
    FrozenReportArtifact, PlotFigureBlock, ProseBlock, ProseStyle, ReportBlockId, ReportBlockKind,
    ReportBlockedGateTextPolicy, ReportDocument, ReportEdit, ReportEntityRef,
    ReportPageEvidenceBinding, ReportPageId, ReportPageInclusion, ReportPageUpdatePolicy,
    ReportReferenceMode, ReportReferenceSnapshot, ReportSourceId, ReportTemplate,
    RequirementDisposition, RequirementEntry, RequirementsBlock, ReviewNoteBlock, ReviewNoteStatus,
    SpecificationDisposition, SpecificationEntry, SpecificationsBlock, TableCell, TableColumn,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogInitialFocus, input_row, select};
use crate::workbench::{AppState, RSpiceApp};

use super::super::commands::vocabulary::Command;
use super::super::design_system::{
    WorkbenchIcon, code_inspector_property_list, code_inspector_section, code_workspace_heading,
    icon_button, property_row, workspace_title_row,
};
use super::super::{RouteTransitionSource, SurfaceId, SurfaceRoute};

const DESKTOP_BREAKPOINT: f32 = 1_020.0;
const STACK_BREAKPOINT: f32 = 820.0;
const OUTLINE_DESKTOP_WIDTH: f32 = 250.0;
const OUTLINE_TABLET_WIDTH: f32 = 180.0;
const INSPECTOR_WIDTH: f32 = 300.0;
const PANEL_GAP: f32 = 0.0;
const OUTLINE_HEADER_HEIGHT: f32 = 39.0;
const OUTLINE_ROW_HEIGHT: f32 = 34.0;
const PREVIEW_MIN_HEIGHT: f32 = 420.0;
const INSPECTOR_SECTION_HEIGHT: f32 = 29.0;
const INSPECTOR_ROW_HEIGHT: f32 = 29.0;
const INSPECTOR_SECTION_PADDING: f32 = 17.0;
const PAPER: Color32 = Color32::from_rgb(255, 255, 255);
const PAPER_PANEL: Color32 = Color32::from_rgb(246, 247, 247);
const PAPER_TEXT: Color32 = Color32::from_rgb(32, 36, 40);
const PAPER_MUTED: Color32 = Color32::from_rgb(82, 89, 94);
const PAPER_FAINT: Color32 = Color32::from_rgb(98, 105, 110);
const PAPER_BORDER: Color32 = Color32::from_rgb(205, 210, 213);
const PAPER_ACCENT: Color32 = Color32::from_rgb(122, 93, 0);

fn paper_switch(ui: &mut Ui, value: &mut bool) -> egui::Response {
    const TRACK_SIZE: Vec2 = Vec2::new(32.0, 18.0);
    const HIT_SIZE: Vec2 = Vec2::new(40.0, 28.0);
    let (rect, mut response) = ui.allocate_exact_size(HIT_SIZE, Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Checkbox,
            ui.is_enabled(),
            *value,
            "Include report element",
        )
    });
    if response.clicked() {
        *value = !*value;
        response.mark_changed();
    }

    let track = Rect::from_center_size(rect.center(), TRACK_SIZE);
    let fill = if *value {
        PAPER_ACCENT
    } else if response.hovered() {
        Color32::from_rgb(225, 228, 229)
    } else {
        PAPER_PANEL
    };
    ui.painter().rect(
        track,
        TRACK_SIZE.y * 0.5,
        fill,
        Stroke::new(1.0, if *value { PAPER_ACCENT } else { PAPER_BORDER }),
        egui::StrokeKind::Inside,
    );
    let knob_x = if *value {
        track.right() - 7.0
    } else {
        track.left() + 7.0
    };
    ui.painter().circle_filled(
        egui::pos2(knob_x, track.center().y),
        5.5,
        if *value { PAPER } else { PAPER_MUTED },
    );
    theme::paint_focus_ring(ui, &response, rect);
    response
}

fn paint_dashed_rect(ui: &Ui, rect: Rect, color: Color32) {
    const DASH: f32 = 4.0;
    const GAP: f32 = 3.0;
    let stroke = Stroke::new(1.0, color);
    let painter = ui.painter();

    let mut x = rect.left();
    while x < rect.right() {
        let end = (x + DASH).min(rect.right());
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(end, rect.top())],
            stroke,
        );
        painter.line_segment(
            [egui::pos2(x, rect.bottom()), egui::pos2(end, rect.bottom())],
            stroke,
        );
        x += DASH + GAP;
    }

    let mut y = rect.top();
    while y < rect.bottom() {
        let end = (y + DASH).min(rect.bottom());
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.left(), end)],
            stroke,
        );
        painter.line_segment(
            [egui::pos2(rect.right(), y), egui::pos2(rect.right(), end)],
            stroke,
        );
        y += DASH + GAP;
    }
}

const INITIAL_PAGES: [(&str, &str); 7] = [
    ("1", "Executive summary"),
    ("2", "Design and configuration"),
    ("3", "Nominal results"),
    ("4", "PVT and yield"),
    ("5", "Reliability and regression"),
    ("6", "Physical DRC and waivers"),
    ("A", "Run manifests"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComposerLayout {
    ThreeColumn,
    TwoColumnInspectorBelow,
    Stacked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PageMoveDirection {
    Earlier,
    Later,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PageSettingEdit {
    Title(String),
    Inclusion(ReportPageInclusion),
    EvidenceBinding(ReportPageEvidenceBinding),
    BlockedGateText(ReportBlockedGateTextPolicy),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct PaneSeparators {
    top: bool,
    right: bool,
    bottom: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ComposerPaneHeights {
    outline: f32,
    preview: f32,
    inspector: f32,
}

impl ComposerLayout {
    fn resolve(width: f32) -> Self {
        if width > DESKTOP_BREAKPOINT {
            Self::ThreeColumn
        } else if width > STACK_BREAKPOINT {
            Self::TwoColumnInspectorBelow
        } else {
            Self::Stacked
        }
    }

    fn separators(self) -> [PaneSeparators; 3] {
        match self {
            Self::ThreeColumn => [
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
            ],
            Self::TwoColumnInspectorBelow => [
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
                PaneSeparators {
                    top: true,
                    ..PaneSeparators::default()
                },
            ],
            Self::Stacked => [
                PaneSeparators {
                    bottom: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators {
                    bottom: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
            ],
        }
    }
}

fn composer_pane_heights(
    layout: ComposerLayout,
    available_height: f32,
    page_count: usize,
    page_selected: bool,
) -> ComposerPaneHeights {
    let viewport_height = if available_height.is_finite() {
        available_height.max(1.0)
    } else {
        PREVIEW_MIN_HEIGHT
    };
    let outline_content = OUTLINE_HEADER_HEIGHT + OUTLINE_ROW_HEIGHT * page_count as f32;
    let inspector_sections = if page_selected { 2.0 } else { 1.0 };
    let inspector_content = inspector_sections
        * (INSPECTOR_SECTION_HEIGHT + INSPECTOR_ROW_HEIGHT * 5.0 + INSPECTOR_SECTION_PADDING);

    match layout {
        ComposerLayout::ThreeColumn => ComposerPaneHeights {
            outline: viewport_height,
            preview: viewport_height,
            inspector: viewport_height,
        },
        ComposerLayout::TwoColumnInspectorBelow => {
            let top_content = outline_content.max(PREVIEW_MIN_HEIGHT);
            let base_height = top_content + inspector_content;
            let surplus = (viewport_height - base_height).max(0.0);
            ComposerPaneHeights {
                outline: top_content + surplus * 0.65,
                preview: top_content + surplus * 0.65,
                inspector: inspector_content + surplus * 0.35,
            }
        }
        ComposerLayout::Stacked => {
            let base_height = outline_content + PREVIEW_MIN_HEIGHT + inspector_content;
            let surplus = (viewport_height - base_height).max(0.0);
            ComposerPaneHeights {
                outline: outline_content,
                preview: PREVIEW_MIN_HEIGHT + surplus * 0.70,
                inspector: inspector_content + surplus * 0.30,
            }
        }
    }
}

pub(crate) fn open(app: &mut RSpiceApp) {
    let route = SurfaceRoute::surface(SurfaceId::ReportAuthoring);
    if let Err(error) = app
        .state
        .workbench
        .navigate(route, RouteTransitionSource::User)
    {
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                error.to_string(),
            ));
    }
}

pub(crate) fn save_document(app: &mut RSpiceApp) {
    if !report_mutation_allowed(&app.state) {
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                report_mutation_block_reason(&app.state),
            ));
        return;
    }
    let invalid = app
        .state
        .workspace
        .report_documents
        .iter()
        .find_map(|document| document.validate().err());
    if let Some(error) = invalid {
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                "Report document save was blocked before publication: {error}"
            )));
        return;
    }
    Command::Save.execute(app);
}

pub(crate) fn open_add_page(app: &mut RSpiceApp) {
    if active_document(&app.state).is_none() || !report_mutation_allowed(&app.state) {
        return;
    }
    let next =
        active_document(&app.state).map_or(1, |document| document.pages().len().saturating_add(1));
    let editor = &mut app.state.workbench.report_authoring;
    editor.add_page_title = format!("Report page {next}");
    editor.transaction_error = None;
    editor.add_page_open = true;
}

pub(crate) fn open_page_properties(app: &mut RSpiceApp) {
    if !report_mutation_allowed(&app.state) {
        return;
    }
    let Some(document) = active_document(&app.state) else {
        return;
    };
    let Some(page) = selected_page_id(&app.state, document).and_then(|id| document.page(id)) else {
        return;
    };
    let page_id = page.id();
    let title = page.title().to_owned();
    let template = report_template_index(document.template());
    let update_policy = page_update_policy_index(page.update_policy());
    let editor = &mut app.state.workbench.report_authoring;
    editor.page_properties_page = Some(page_id);
    editor.page_title_draft = title;
    editor.report_template_draft = template;
    editor.page_update_policy_draft = update_policy;
    editor.transaction_error = None;
    editor.page_properties_open = true;
}

fn can_move_selected_page(state: &AppState, direction: PageMoveDirection) -> bool {
    if !report_mutation_allowed(state) {
        return false;
    }
    let Some(document) = active_document(state) else {
        return false;
    };
    let Some(page_id) = selected_page_id(state, document) else {
        return false;
    };
    let Some(index) = document
        .pages()
        .iter()
        .position(|page| page.id() == page_id)
    else {
        return false;
    };
    match direction {
        PageMoveDirection::Earlier => index > 0,
        PageMoveDirection::Later => index + 1 < document.pages().len(),
    }
}

fn move_selected_page(app: &mut RSpiceApp, direction: PageMoveDirection) {
    if !report_mutation_allowed(&app.state) {
        let reason = report_mutation_block_reason(&app.state).to_owned();
        app.state.workbench.report_authoring.transaction_error = Some(reason.clone());
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::warning(reason));
        return;
    }
    let selected_page =
        active_document(&app.state).and_then(|document| selected_page_id(&app.state, document));
    let Some(page_id) = selected_page else {
        return;
    };
    let revision_note = match direction {
        PageMoveDirection::Earlier => "Move report page earlier",
        PageMoveDirection::Later => "Move report page later",
    };
    let result = active_document_mut(&mut app.state).and_then(|document| {
        let Some(index) = document
            .pages()
            .iter()
            .position(|page| page.id() == page_id)
        else {
            return Err("The selected report page no longer exists.".to_owned());
        };
        let expected_page_revision = document.pages()[index].revision();
        let before = match direction {
            PageMoveDirection::Earlier if index > 0 => Some(document.pages()[index - 1].id()),
            PageMoveDirection::Later if index + 1 < document.pages().len() => {
                document.pages().get(index + 2).map(|page| page.id())
            }
            _ => return Ok(false),
        };
        document
            .transact_with_context(
                document.revision(),
                vec![ReportEdit::MovePage {
                    page_id,
                    expected_page_revision,
                    before,
                }],
                timestamp_unix_ms(),
                "rspice-local-session",
                revision_note,
            )
            .map(|_| true)
            .map_err(|error| error.to_string())
    });
    match result {
        Ok(changed) => {
            app.state.workbench.report_authoring.selected_page = Some(page_id);
            app.state.workbench.report_authoring.preview_block_page = 0;
            app.state.workbench.report_authoring.transaction_error = None;
            app.state.workspace.report_documents_dirty |= changed;
        }
        Err(error) => {
            app.state.workbench.report_authoring.transaction_error = Some(error.clone());
            app.state
                .push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
        }
    }
}

fn commit_page_setting(app: &mut RSpiceApp, page_id: ReportPageId, setting: PageSettingEdit) {
    if !report_mutation_allowed(&app.state) {
        let reason = report_mutation_block_reason(&app.state).to_owned();
        app.state.workbench.report_authoring.transaction_error = Some(reason.clone());
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::warning(reason));
        return;
    }
    if let PageSettingEdit::Title(title) = &setting
        && !valid_page_title(title)
    {
        app.state.workbench.report_authoring.transaction_error = Some(
            "The page title must be trimmed, non-empty, single-line text of at most 512 characters."
                .to_owned(),
        );
        return;
    }
    let result = active_document_mut(&mut app.state).and_then(|document| {
        let page = document
            .page(page_id)
            .ok_or_else(|| "The selected report page no longer exists.".to_owned())?;
        let expected_page_revision = page.revision();
        let (edit, revision_note) = match setting {
            PageSettingEdit::Title(title) if page.title() != title => (
                ReportEdit::UpdatePageTitle {
                    page_id,
                    expected_page_revision,
                    title,
                },
                "Update report page title",
            ),
            PageSettingEdit::Inclusion(inclusion) if page.inclusion() != inclusion => (
                ReportEdit::SetPageInclusion {
                    page_id,
                    expected_page_revision,
                    inclusion,
                },
                "Update report page inclusion",
            ),
            PageSettingEdit::EvidenceBinding(evidence_binding)
                if page.evidence_binding() != evidence_binding =>
            {
                (
                    ReportEdit::SetPageEvidenceBinding {
                        page_id,
                        expected_page_revision,
                        evidence_binding,
                    },
                    "Update report page evidence binding",
                )
            }
            PageSettingEdit::BlockedGateText(policy)
                if page.blocked_gate_text_policy() != policy =>
            {
                (
                    ReportEdit::SetPageBlockedGateTextPolicy {
                        page_id,
                        expected_page_revision,
                        policy,
                    },
                    "Update report blocked-gate text policy",
                )
            }
            _ => return Ok(false),
        };
        document
            .transact_with_context(
                document.revision(),
                vec![edit],
                timestamp_unix_ms(),
                "rspice-local-session",
                revision_note,
            )
            .map(|_| true)
            .map_err(|error| error.to_string())
    });
    match result {
        Ok(changed) => {
            app.state.workbench.report_authoring.selected_page = Some(page_id);
            app.state.workbench.report_authoring.preview_block_page = 0;
            app.state.workbench.report_authoring.transaction_error = None;
            app.state.workspace.report_documents_dirty |= changed;
        }
        Err(error) => {
            app.state.workbench.report_authoring.transaction_error = Some(error.clone());
            app.state
                .push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
        }
    }
}

fn set_report_block_enabled(
    app: &mut RSpiceApp,
    document_id: crate::product::ResultDocumentId,
    block_id: ReportBlockId,
    enabled: bool,
) {
    if !report_mutation_allowed(&app.state) {
        let reason = report_mutation_block_reason(&app.state).to_owned();
        app.state.workbench.report_authoring.transaction_error = Some(reason.clone());
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::warning(reason));
        return;
    }
    let timestamp = timestamp_unix_ms();
    let result = active_document_mut(&mut app.state).and_then(|document| {
        if document.id() != document_id {
            return Err(
                "The report document changed before the element update committed.".to_owned(),
            );
        }
        let block = document
            .block(block_id)
            .ok_or_else(|| "The selected report element no longer exists.".to_owned())?;
        if block.enabled() == enabled {
            return Ok(false);
        }
        document
            .transact_with_context(
                document.revision(),
                vec![ReportEdit::SetBlockEnabled {
                    block_id,
                    expected_block_revision: block.revision(),
                    enabled,
                }],
                timestamp,
                "rspice-local-session",
                if enabled {
                    "Include report page element"
                } else {
                    "Exclude report page element"
                },
            )
            .map(|_| true)
            .map_err(|error| error.to_string())
    });
    match result {
        Ok(changed) => {
            app.state.workbench.report_authoring.selected_report_block = Some(block_id);
            app.state.workbench.report_authoring.preview_block_page = 0;
            app.state.workbench.report_authoring.transaction_error = None;
            app.state.workspace.report_documents_dirty |= changed;
        }
        Err(error) => {
            app.state.workbench.report_authoring.transaction_error = Some(error.clone());
            app.state
                .push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
        }
    }
}

fn evidence_binding_label(state: &AppState, evidence_binding: ReportPageEvidenceBinding) -> String {
    match evidence_binding {
        ReportPageEvidenceBinding::Unbound => "Unbound — select evidence".to_owned(),
        ReportPageEvidenceBinding::LatestAcceptedRun => {
            "Latest accepted run — resolve on draft build".to_owned()
        }
        ReportPageEvidenceBinding::ExactDataset { binding } => state
            .simulation
            .runs
            .iter()
            .find(|run| {
                run.dataset_id == binding.dataset_id
                    && run.dataset_content_digest() == binding.content_digest
            })
            .map_or_else(
                || {
                    let dataset_id = binding.dataset_id.to_string();
                    format!(
                        "Dataset {}… · immutable",
                        dataset_id.get(..8).unwrap_or(&dataset_id)
                    )
                },
                |run| format!("Run {} · immutable", run.id),
            ),
    }
}

fn evidence_binding_options(
    state: &AppState,
    current: ReportPageEvidenceBinding,
) -> Vec<(String, ReportPageEvidenceBinding)> {
    let mut options = state
        .simulation
        .runs
        .iter()
        .filter(|run| !run.analyses.is_empty())
        .map(|run| {
            let binding =
                crate::product::DatasetBinding::new(run.dataset_id, run.dataset_content_digest());
            (
                format!("Run {} · immutable", run.id),
                ReportPageEvidenceBinding::ExactDataset { binding },
            )
        })
        .collect::<Vec<_>>();
    if !options.iter().any(|(_, option)| *option == current)
        && matches!(current, ReportPageEvidenceBinding::ExactDataset { .. })
    {
        options.push((evidence_binding_label(state, current), current));
    }
    options.push((
        "Latest accepted run — resolve on draft build".to_owned(),
        ReportPageEvidenceBinding::LatestAcceptedRun,
    ));
    options.push((
        "Unbound — select evidence".to_owned(),
        ReportPageEvidenceBinding::Unbound,
    ));
    options
}

fn open_create_document(app: &mut RSpiceApp) {
    if !report_mutation_allowed(&app.state) {
        return;
    }
    let editor = &mut app.state.workbench.report_authoring;
    editor.create_document_title = "Verification report".to_owned();
    editor.create_document_template = report_template_index(ReportTemplate::ReleaseVerification42);
    editor.transaction_error = None;
    editor.create_document_open = true;
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        ui.set_width(ui.available_width());

        if !app.state.project_lifecycle.project_open {
            workspace_title_row(ui, |ui| {
                code_workspace_heading(
                    ui,
                    "REPORT AUTHORING · NO OPEN PROJECT",
                    "Engineering report composer",
                    "Open a project before creating a project-owned report document.",
                );
            });
            return;
        }

        synchronize_report_selection(&mut app.state);

        workspace_title_row(ui, |ui| {
            code_workspace_heading(
                ui,
                "REPORT AUTHORING · ENGINEERING DRAFT",
                "Engineering report composer",
                "Author and save the versioned report document, its page order, and page properties.",
            );
        });

        let Some(document) = active_document(&app.state).cloned() else {
            empty_report_workspace(ui, app);
            return;
        };
        let selected_page = selected_page_id(&app.state, &document);
        let available = ui.available_size();
        let layout = ComposerLayout::resolve(available.x);
        let heights = composer_pane_heights(
            layout,
            available.y,
            document.pages().len(),
            selected_page.is_some(),
        );
        let [outline_separators, preview_separators, inspector_separators] = layout.separators();
        match layout {
            ComposerLayout::ThreeColumn => {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = PANEL_GAP;
                    ui.allocate_ui_with_layout(
                        Vec2::new(OUTLINE_DESKTOP_WIDTH, available.y),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            outline(
                                ui,
                                app,
                                &document,
                                selected_page,
                                outline_separators,
                            )
                        },
                    );
                    let preview_width = (available.x
                        - OUTLINE_DESKTOP_WIDTH
                        - INSPECTOR_WIDTH
                        - PANEL_GAP * 2.0)
                        .max(1.0);
                    ui.allocate_ui_with_layout(
                        Vec2::new(preview_width, available.y),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| preview(ui, app, &document, selected_page, preview_separators),
                    );
                    ui.allocate_ui_with_layout(
                        Vec2::new(INSPECTOR_WIDTH, available.y),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| inspector(ui, &document, selected_page, inspector_separators),
                    );
                });
            }
            ComposerLayout::TwoColumnInspectorBelow => {
                ScrollArea::vertical()
                    .id_salt("report-authoring.tablet")
                    .show(ui, |ui| {
                        let local_width = ui.available_width().max(1.0);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = PANEL_GAP;
                            ui.allocate_ui_with_layout(
                                Vec2::new(OUTLINE_TABLET_WIDTH, heights.outline),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    outline(
                                        ui,
                                        app,
                                        &document,
                                        selected_page,
                                        outline_separators,
                                    )
                                },
                            );
                            ui.allocate_ui_with_layout(
                                Vec2::new(
                                    (local_width - OUTLINE_TABLET_WIDTH).max(1.0),
                                    heights.preview,
                                ),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    preview(
                                        ui,
                                        app,
                                        &document,
                                        selected_page,
                                        preview_separators,
                                    )
                                },
                            );
                        });
                        ui.allocate_ui_with_layout(
                            Vec2::new(local_width, heights.inspector),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                inspector(
                                    ui,
                                    &document,
                                    selected_page,
                                    inspector_separators,
                                )
                            },
                        );
                    });
            }
            ComposerLayout::Stacked => {
                ScrollArea::vertical()
                    .id_salt("report-authoring.compact")
                    .show(ui, |ui| {
                        let local_width = ui.available_width().max(1.0);
                        ui.allocate_ui_with_layout(
                            Vec2::new(local_width, heights.outline),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                outline(
                                    ui,
                                    app,
                                    &document,
                                    selected_page,
                                    outline_separators,
                                )
                            },
                        );
                        ui.allocate_ui_with_layout(
                            Vec2::new(local_width, heights.preview),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                preview(
                                    ui,
                                    app,
                                    &document,
                                    selected_page,
                                    preview_separators,
                                )
                            },
                        );
                        ui.allocate_ui_with_layout(
                            Vec2::new(local_width, heights.inspector),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                inspector(
                                    ui,
                                    &document,
                                    selected_page,
                                    inspector_separators,
                                )
                            },
                        );
                    });
            }
        }
    });

    create_document_dialog(ui.ctx(), app);
    add_page_dialog(ui.ctx(), app);
    page_properties_dialog(ui.ctx(), app);
    remove_report_block_dialog(ui.ctx(), app);
    insert_result_document_dialog(ui.ctx(), app);
    add_report_element_dialog(ui.ctx(), app);
}

fn empty_report_workspace(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let available = ui.available_size();
    let writable = report_mutation_allowed(&app.state);
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .show(ui, |ui| {
            ui.set_min_size(available.max(Vec2::new(1.0, 1.0)));
            ui.add_space((available.y * 0.24).clamp(36.0, 180.0));
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("No report document")
                        .font(theme::sans(18.0, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.add_space(7.0);
                ui.label(
                    egui::RichText::new(
                        "Plan an explicit project-owned report artifact before authoring pages and evidence.",
                    )
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
                ui.add_space(16.0);
                if Button::new("Plan report artifact...")
                    .accent()
                    .enabled(writable)
                    .show(ui)
                    .clicked()
                {
                    open_create_document(app);
                }
                if !writable {
                    ui.add_space(8.0);
                    ui.colored_label(
                        t.color.err,
                        report_mutation_block_reason(&app.state),
                    );
                }
            });
        });
}

fn outline(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    document: &ReportDocument,
    selected_page: Option<ReportPageId>,
    separators: PaneSeparators,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let height = ui.available_height();
    let pane = egui::Frame::new().fill(t.color.bg_panel).show(ui, |ui| {
        ui.set_min_size(Vec2::new(width.max(1.0), height.max(1.0)));
        let (head, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 39.0), Sense::hover());
        ui.painter().hline(
            head.x_range(),
            head.bottom(),
            Stroke::new(1.0, t.color.border),
        );
        ui.painter().text(
            head.left_center() + Vec2::new(10.0, 0.0),
            Align2::LEFT_CENTER,
            "Report outline",
            theme::sans(tokens::FS_2, FontWeight::SemiBold),
            t.color.text,
        );
        const CONTROL_SIZE: f32 = 29.0;
        const CONTROL_GAP: f32 = 2.0;
        const CONTROL_COUNT: f32 = 4.0;
        let controls_width = CONTROL_SIZE * CONTROL_COUNT + CONTROL_GAP * (CONTROL_COUNT - 1.0);
        let controls_rect = Rect::from_center_size(
            head.right_center() - Vec2::new(5.0 + controls_width * 0.5, 0.0),
            Vec2::new(controls_width, CONTROL_SIZE),
        );
        let mut controls = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(controls_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        controls.spacing_mut().item_spacing.x = CONTROL_GAP;
        let writable = report_mutation_allowed(&app.state);
        let add_response = controls
            .add_enabled_ui(writable, |ui| {
                icon_button(
                    ui,
                    WorkbenchIcon::Add,
                    "Add report page",
                    false,
                    Vec2::splat(CONTROL_SIZE),
                )
            })
            .inner
            .on_disabled_hover_text(report_mutation_block_reason(&app.state));
        if add_response.clicked() {
            open_add_page(app);
        }

        let move_earlier_enabled = can_move_selected_page(&app.state, PageMoveDirection::Earlier);
        let move_earlier_response = controls
            .add_enabled_ui(move_earlier_enabled, |ui| {
                icon_button(
                    ui,
                    WorkbenchIcon::ArrowLeft,
                    "Move page earlier",
                    false,
                    Vec2::splat(CONTROL_SIZE),
                )
            })
            .inner
            .on_disabled_hover_text(if writable {
                "The selected page is already first."
            } else {
                report_mutation_block_reason(&app.state)
            });
        if move_earlier_response.clicked() {
            move_selected_page(app, PageMoveDirection::Earlier);
        }

        let move_later_enabled = can_move_selected_page(&app.state, PageMoveDirection::Later);
        let move_later_response = controls
            .add_enabled_ui(move_later_enabled, |ui| {
                icon_button(
                    ui,
                    WorkbenchIcon::ArrowRight,
                    "Move page later",
                    false,
                    Vec2::splat(CONTROL_SIZE),
                )
            })
            .inner
            .on_disabled_hover_text(if writable {
                "The selected page is already last."
            } else {
                report_mutation_block_reason(&app.state)
            });
        if move_later_response.clicked() {
            move_selected_page(app, PageMoveDirection::Later);
        }

        let properties_enabled = writable && selected_page.is_some();
        let properties_response = controls
            .add_enabled_ui(properties_enabled, |ui| {
                icon_button(
                    ui,
                    WorkbenchIcon::Sliders,
                    "Page properties",
                    false,
                    Vec2::splat(CONTROL_SIZE),
                )
            })
            .inner
            .on_disabled_hover_text(report_mutation_block_reason(&app.state));
        if properties_response.clicked() {
            open_page_properties(app);
        }

        ScrollArea::vertical()
            .id_salt("report-authoring.outline")
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                for (index, page) in document.pages().iter().enumerate() {
                    let marker = page_marker(index, page.title());
                    let selected = Some(page.id()) == selected_page;
                    if outline_row(ui, marker, page.title(), selected).clicked() {
                        app.state.workbench.report_authoring.selected_page = Some(page.id());
                        app.state.workbench.report_authoring.selected_report_block = None;
                        app.state.workbench.report_authoring.preview_block_page = 0;
                    }
                }
                page_settings(ui, app, document, selected_page);
            });
    });
    paint_pane_separators(ui, pane.response.rect, separators, t.color.border);
}

fn outline_row(ui: &mut Ui, marker: &str, label: &str, selected: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 34.0), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Button, ui.is_enabled(), selected, label)
    });
    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
        ui.painter().rect_filled(
            Rect::from_min_max(rect.left_top(), rect.left_bottom() + Vec2::new(2.0, 0.0)),
            0.0,
            t.color.accent,
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().text(
        rect.left_center() + Vec2::new(10.0, 0.0),
        Align2::LEFT_CENTER,
        marker,
        theme::mono(tokens::FS_1, FontWeight::Medium),
        if selected {
            t.color.accent
        } else {
            t.color.text_dim
        },
    );
    ui.painter().text(
        rect.left_center() + Vec2::new(37.0, 0.0),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_1, FontWeight::Regular),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn page_settings(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    document: &ReportDocument,
    selected_page: Option<ReportPageId>,
) {
    let Some(page) = selected_page.and_then(|page_id| document.page(page_id)) else {
        return;
    };
    let page_id = page.id();
    {
        let editor = &mut app.state.workbench.report_authoring;
        if editor.inline_page_settings_page != Some(page_id) {
            editor.inline_page_settings_page = Some(page_id);
            editor.inline_page_title_draft = page.title().to_owned();
            editor.transaction_error = None;
        }
    }
    let t = Tokens::get(ui.ctx());
    code_inspector_section(ui, "Page settings", None, |ui| {
        egui::Frame::new()
            .inner_margin(egui::Margin {
                left: 10,
                right: 10,
                top: 8,
                bottom: 10,
            })
            .show(ui, |ui| {
                ui.set_width(ui.available_width().max(1.0));
                report_form_label(ui, "Page title");
                let title_response = ui.add_sized(
                    Vec2::new(ui.available_width(), t.metrics.ctl_h),
                    egui::TextEdit::singleline(
                        &mut app.state.workbench.report_authoring.inline_page_title_draft,
                    )
                    .font(theme::mono(tokens::FS_1, FontWeight::Regular)),
                );
                if title_response.has_focus()
                    && ui.input(|input| input.key_pressed(egui::Key::Escape))
                {
                    app.state.workbench.report_authoring.inline_page_title_draft =
                        page.title().to_owned();
                    title_response.surrender_focus();
                } else if title_response.lost_focus() {
                    let title = app
                        .state
                        .workbench
                        .report_authoring
                        .inline_page_title_draft
                        .trim()
                        .to_owned();
                    app.state.workbench.report_authoring.inline_page_title_draft = title.clone();
                    commit_page_setting(app, page_id, PageSettingEdit::Title(title));
                }

                ui.add_space(8.0);
                report_form_label(ui, "Include in artifact");
                const INCLUSION_OPTIONS: [(ReportPageInclusion, &str); 3] = [
                    (ReportPageInclusion::Included, "Included"),
                    (
                        ReportPageInclusion::ExcludedFromDraft,
                        "Excluded from draft",
                    ),
                    (ReportPageInclusion::AppendixOnly, "Appendix only"),
                ];
                let inclusion_labels = INCLUSION_OPTIONS
                    .iter()
                    .map(|(_, label)| (*label).to_owned())
                    .collect::<Vec<_>>();
                let inclusion_current = report_page_inclusion_label(page.inclusion());
                if let Some(index) = select(
                    ui,
                    "report-page-inclusion",
                    "Include report page in artifact",
                    inclusion_current,
                    &inclusion_labels,
                    ui.available_width(),
                ) && let Some((inclusion, _)) = INCLUSION_OPTIONS.get(index)
                {
                    commit_page_setting(app, page_id, PageSettingEdit::Inclusion(*inclusion));
                }

                ui.add_space(8.0);
                report_form_label(ui, "Evidence binding");
                let evidence_options =
                    evidence_binding_options(&app.state, page.evidence_binding());
                let evidence_labels = evidence_options
                    .iter()
                    .map(|(label, _)| label.clone())
                    .collect::<Vec<_>>();
                let evidence_current = evidence_binding_label(&app.state, page.evidence_binding());
                if let Some(index) = select(
                    ui,
                    "report-page-evidence-binding",
                    "Report page evidence binding",
                    &evidence_current,
                    &evidence_labels,
                    ui.available_width(),
                ) && let Some((_, evidence_binding)) = evidence_options.get(index)
                {
                    commit_page_setting(
                        app,
                        page_id,
                        PageSettingEdit::EvidenceBinding(*evidence_binding),
                    );
                }

                ui.add_space(8.0);
                report_form_label(ui, "Blocked-gate text");
                const GATE_TEXT_OPTIONS: [(ReportBlockedGateTextPolicy, &str); 2] = [
                    (
                        ReportBlockedGateTextPolicy::VerbatimFromSource,
                        "State verbatim from source",
                    ),
                    (
                        ReportBlockedGateTextPolicy::SummarizeWithLink,
                        "Summarize with link",
                    ),
                ];
                let gate_text_labels = GATE_TEXT_OPTIONS
                    .iter()
                    .map(|(_, label)| (*label).to_owned())
                    .collect::<Vec<_>>();
                let gate_text_current =
                    report_blocked_gate_text_policy_label(page.blocked_gate_text_policy());
                if let Some(index) = select(
                    ui,
                    "report-page-gate-text",
                    "Blocked-gate text policy",
                    gate_text_current,
                    &gate_text_labels,
                    ui.available_width(),
                ) && let Some((policy, _)) = GATE_TEXT_OPTIONS.get(index)
                {
                    commit_page_setting(app, page_id, PageSettingEdit::BlockedGateText(*policy));
                }

                if let Some(error) = app
                    .state
                    .workbench
                    .report_authoring
                    .transaction_error
                    .as_deref()
                {
                    ui.add_space(8.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(error)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.err),
                        )
                        .wrap(),
                    );
                }
            });
    });
}

fn report_form_label(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.add_space(4.0);
}

fn preview(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    document: &ReportDocument,
    selected_page: Option<ReportPageId>,
    separators: PaneSeparators,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let height = ui.available_height();
    let pane = egui::Frame::new()
        .fill(PAPER_PANEL)
        .show(ui, |ui| {
            ui.set_min_size(Vec2::new(width.max(1.0), height.max(1.0)));
            ScrollArea::vertical()
                .id_salt("report-authoring.preview")
                .show(ui, |ui| {
                    let compact = ui.available_width() < 560.0;
                    let horizontal_margin = if compact { 18.0 } else { 42.0 };
                    let top_margin = if compact { 24.0 } else { 36.0 };
                    egui::Frame::new()
                        .inner_margin(egui::Margin {
                            left: horizontal_margin as i8,
                            right: horizontal_margin as i8,
                            top: top_margin as i8,
                            bottom: 36,
                        })
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            paper_label(
                                ui,
                                "PROJECT REPORT DOCUMENT",
                                theme::mono(tokens::FS_0, FontWeight::Medium),
                                PAPER_MUTED,
                            );
                            ui.add_space(8.0);
                            paper_label(
                                ui,
                                document.title(),
                                theme::sans(26.0, FontWeight::SemiBold),
                                PAPER_TEXT,
                            );
                            ui.add_space(4.0);
                            paper_label(
                                ui,
                                &format!(
                                    "Document revision {} · {}",
                                    document.revision().get(),
                                    report_template_label(document.template()),
                                ),
                                theme::sans(tokens::FS_1, FontWeight::Regular),
                                PAPER_MUTED,
                            );

                            let page = selected_page.and_then(|id| document.page(id));
                            let page_index = page.and_then(|page| {
                                document
                                    .pages()
                                    .iter()
                                    .position(|candidate| candidate.id() == page.id())
                            });
                            let marker = page_index
                                .map(|index| page_marker(index, page.map_or("", |p| p.title())))
                                .unwrap_or("—");
                            let title = page.map_or("No report page selected", |page| page.title());
                            let description = page.map_or_else(
                                || "Select a page from the report outline.".to_owned(),
                                |page| {
                                    format!(
                                        "Page revision {} · {}",
                                        page.revision().get(),
                                        page_update_policy_label(page.update_policy())
                                    )
                                },
                            );
                            ui.add_space(24.0);
                            section_heading(ui, marker, title, &description);
                            ui.add_space(28.0);
                            summary_grid(ui, document, page, compact);
                            if let Some(page) = page {
                                ui.add_space(28.0);
                                page_elements(ui, app, document, page, compact);
                            }
                            ui.add_space(24.0);
                            paper_label(
                                ui,
                                "Document source",
                                theme::sans(tokens::FS_3, FontWeight::SemiBold),
                                PAPER_TEXT,
                            );
                            ui.add_space(7.0);
                            paper_label(
                                ui,
                                "This source is the canonical project-owned ReportDocument. Page and document changes are applied as validated, revision-checked transactions and persisted with the project.",
                                theme::sans(tokens::FS_1, FontWeight::Regular),
                                PAPER_MUTED,
                            );
                        });
                });
        });
    paint_pane_separators(ui, pane.response.rect, separators, t.color.border);
}

fn section_heading(ui: &mut Ui, marker: &str, title: &str, description: &str) {
    let (line, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), Sense::hover());
    ui.painter().hline(
        line.x_range(),
        line.center().y,
        Stroke::new(1.0, PAPER_BORDER),
    );
    ui.add_space(13.0);
    ui.horizontal_top(|ui| {
        ui.set_width(ui.available_width());
        ui.allocate_ui_with_layout(
            Vec2::new(34.0, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                paper_label(
                    ui,
                    marker,
                    theme::mono(tokens::FS_2, FontWeight::SemiBold),
                    PAPER_ACCENT,
                );
            },
        );
        ui.add_space(10.0);
        ui.vertical(|ui| {
            paper_label(
                ui,
                title,
                theme::sans(15.0, FontWeight::SemiBold),
                PAPER_TEXT,
            );
            ui.add_space(4.0);
            paper_label(
                ui,
                description,
                theme::sans(tokens::FS_1, FontWeight::Regular),
                PAPER_MUTED,
            );
        });
    });
}

fn summary_grid(
    ui: &mut Ui,
    document: &ReportDocument,
    page: Option<&crate::results::report_document::ReportPage>,
    compact: bool,
) {
    let page_sections = page.map_or(0, |page| page.sections().len());
    let page_blocks = page.map_or(0, |page| {
        page.sections()
            .iter()
            .map(|section| section.blocks().len())
            .sum()
    });
    let cells = [
        (document.pages().len().to_string(), "document pages"),
        (page_sections.to_string(), "selected-page sections"),
        (page_blocks.to_string(), "selected-page blocks"),
    ];
    if compact {
        for (value, label) in cells {
            summary_cell(ui, &value, label);
            ui.add_space(8.0);
        }
    } else {
        let width = ui.available_width();
        let cell_width = ((width - 16.0) / 3.0).max(1.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            for (value, label) in cells {
                ui.allocate_ui_with_layout(
                    Vec2::new(cell_width, 76.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| summary_cell(ui, &value, label),
                );
            }
        });
    }
}

fn summary_cell(ui: &mut Ui, value: &str, label: &str) {
    egui::Frame::new()
        .fill(PAPER)
        .stroke(Stroke::new(1.0, PAPER_BORDER))
        .inner_margin(egui::Margin::same(14))
        .show(ui, |ui| {
            ui.set_min_width((ui.available_width() - 28.0).max(1.0));
            paper_label(
                ui,
                value,
                theme::sans(20.0, FontWeight::SemiBold),
                PAPER_TEXT,
            );
            ui.add_space(4.0);
            paper_label(
                ui,
                label,
                theme::sans(tokens::FS_0, FontWeight::Regular),
                PAPER_FAINT,
            );
        });
}

fn page_elements(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    document: &ReportDocument,
    page: &crate::results::report_document::ReportPage,
    compact: bool,
) {
    paper_label(
        ui,
        "Page elements",
        theme::sans(tokens::FS_3, FontWeight::SemiBold),
        PAPER_TEXT,
    );
    ui.add_space(8.0);

    let blocks = page
        .sections()
        .iter()
        .flat_map(|section| section.blocks())
        .collect::<Vec<_>>();
    if blocks.is_empty() {
        egui::Frame::new()
            .fill(PAPER)
            .stroke(Stroke::new(1.0, PAPER_BORDER))
            .inner_margin(egui::Margin::same(14))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                paper_label(
                    ui,
                    "No page elements. Add authored content or insert an immutable result document.",
                    theme::sans(tokens::FS_1, FontWeight::Regular),
                    PAPER_MUTED,
                );
            });
    } else {
        ScrollArea::horizontal()
            .id_salt(("report-page-elements", page.id()))
            .show(ui, |ui| {
                ui.set_min_width(if compact {
                    680.0
                } else {
                    ui.available_width().max(680.0)
                });
                let previous_faint_background = ui.visuals().faint_bg_color;
                ui.visuals_mut().faint_bg_color = Color32::from_rgb(249, 249, 247);
                TableBuilder::new(ui)
                    .id_salt(("report-page-elements-table", page.id()))
                    .striped(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::remainder().at_least(150.0))
                    .column(Column::initial(110.0).at_least(96.0))
                    .column(Column::remainder().at_least(180.0))
                    .column(Column::initial(104.0).at_least(90.0))
                    .column(Column::initial(46.0).at_least(42.0))
                    .header(27.0, |mut header| {
                        for label in ["Element", "Type", "Bound source", "State", "On"] {
                            header.col(|ui| {
                                ui.painter().rect_filled(
                                    ui.max_rect(),
                                    0.0,
                                    Color32::from_rgb(244, 243, 239),
                                );
                                ui.label(
                                    egui::RichText::new(label)
                                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                                        .color(PAPER_MUTED),
                                );
                            });
                        }
                    })
                    .body(|mut body| {
                        for block in blocks {
                            body.row(24.0, |mut row| {
                                row.col(|ui| {
                                    let selected =
                                        app.state.workbench.report_authoring.selected_report_block
                                            == Some(block.id());
                                    if ui
                                        .selectable_label(
                                            selected,
                                            egui::RichText::new(report_block_element_title(
                                                block.kind(),
                                            ))
                                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                                            .color(PAPER_TEXT),
                                        )
                                        .clicked()
                                    {
                                        app.state
                                            .workbench
                                            .report_authoring
                                            .selected_report_block = Some(block.id());
                                    }
                                });
                                row.col(|ui| {
                                    ui.label(
                                        egui::RichText::new(report_block_kind_label(block.kind()))
                                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                            .color(PAPER_MUTED),
                                    );
                                });
                                row.col(|ui| {
                                    ui.label(
                                        egui::RichText::new(report_block_bound_source(
                                            block.kind(),
                                        ))
                                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                        .color(PAPER_MUTED),
                                    );
                                });
                                row.col(|ui| {
                                    let (state, color) = report_block_state(&app.state, block);
                                    ui.label(
                                        egui::RichText::new(state)
                                            .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                                            .color(color),
                                    );
                                });
                                row.col(|ui| {
                                    let mut enabled = block.enabled();
                                    let response = ui
                                        .add_enabled_ui(report_mutation_allowed(&app.state), |ui| {
                                            paper_switch(ui, &mut enabled)
                                        })
                                        .inner
                                        .on_disabled_hover_text(report_mutation_block_reason(
                                            &app.state,
                                        ));
                                    if response.changed() {
                                        set_report_block_enabled(
                                            app,
                                            document.id(),
                                            block.id(),
                                            enabled,
                                        );
                                    }
                                });
                            });
                        }
                    });
                ui.visuals_mut().faint_bg_color = previous_faint_background;
            });
    }

    ui.add_space(10.0);
    let writable = report_mutation_allowed(&app.state);
    let selected_block_exists = app
        .state
        .workbench
        .report_authoring
        .selected_report_block
        .is_some_and(|block_id| document.block(block_id).is_some());
    ui.horizontal_wrapped(|ui| {
        let add = Button::new("Add element…")
            .enabled(writable)
            .show(ui)
            .on_disabled_hover_text(report_mutation_block_reason(&app.state));
        if add.clicked() {
            open_add_report_element(app);
        }
        let remove = Button::new("Remove")
            .enabled(writable && selected_block_exists)
            .show(ui)
            .on_disabled_hover_text(if writable {
                "Select a page element to remove."
            } else {
                report_mutation_block_reason(&app.state)
            });
        if remove.clicked() {
            app.state
                .workbench
                .report_authoring
                .remove_report_block_open = true;
            app.state.workbench.report_authoring.transaction_error = None;
        }
        let insert = Button::new("Insert result document…")
            .enabled(writable && !app.state.workspace.visualization_documents.is_empty())
            .show(ui)
            .on_disabled_hover_text(if writable {
                "Create or retain a result document before inserting it into this report page."
            } else {
                report_mutation_block_reason(&app.state)
            });
        if insert.clicked() {
            open_insert_result_document(app);
        }
    });
    let banner = egui::Frame::new()
        .fill(PAPER)
        .outer_margin(egui::Margin::same(8))
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            paper_label(
                ui,
                "Release closure validates this artifact against exact immutable result revisions and dataset bindings before building a signed package. Report elements may state blocked gates but cannot change source-owned gate state.",
                theme::sans(tokens::FS_1, FontWeight::Regular),
                PAPER_TEXT,
            );
        });
    paint_dashed_rect(ui, banner.response.rect, PAPER_BORDER);
}

fn report_block_element_title(kind: &ReportBlockKind) -> Cow<'_, str> {
    match kind {
        ReportBlockKind::PlotFigure(block) => Cow::Borrowed(&block.caption),
        ReportBlockKind::DataTable(block) => Cow::Borrowed(&block.title),
        ReportBlockKind::Datasheet(block) => Cow::Borrowed(&block.title),
        ReportBlockKind::Requirements(block) => Cow::Borrowed(&block.title),
        ReportBlockKind::Specifications(block) => Cow::Borrowed(&block.title),
        ReportBlockKind::Prose(block) => {
            let (text, _) = bounded_text_preview(&block.markdown, 56);
            text
        }
        ReportBlockKind::ReviewNote(block) => Cow::Owned(format!("Review note · {}", block.author)),
        ReportBlockKind::Evidence(block) => Cow::Borrowed(&block.title),
    }
}

fn report_block_bound_source(kind: &ReportBlockKind) -> String {
    kind.reference().map_or_else(
        || "Authored report source".to_owned(),
        |reference| {
            format!(
                "{} · {}",
                report_source_label(&reference.snapshot().source),
                if reference.is_frozen() {
                    "frozen"
                } else {
                    "linked"
                }
            )
        },
    )
}

fn report_block_state(
    state: &AppState,
    block: &crate::results::report_document::ReportBlock,
) -> (&'static str, Color32) {
    if !block.enabled() {
        ("excluded", PAPER_FAINT)
    } else if block
        .kind()
        .reference()
        .is_some_and(ReportReferenceMode::is_frozen)
    {
        ("frozen", Color32::from_rgb(72, 122, 78))
    } else if block
        .kind()
        .reference()
        .is_some_and(|reference| !report_reference_resolves(state, reference))
    {
        ("source missing", Color32::from_rgb(177, 64, 52))
    } else if block.kind().reference().is_some() {
        ("bound", Color32::from_rgb(72, 122, 78))
    } else {
        ("authored", PAPER_MUTED)
    }
}

fn report_reference_resolves(state: &AppState, reference: &ReportReferenceMode) -> bool {
    let snapshot = reference.snapshot();
    match &snapshot.source {
        ReportSourceId::VisualizationDocument { document_id } => state
            .workspace
            .visualization_documents
            .iter()
            .find(|document| document.id() == *document_id)
            .is_some_and(|document| {
                snapshot.source_revision == Some(document.revision())
                    && document
                        .content_digest()
                        .is_ok_and(|digest| digest == snapshot.content_digest)
            }),
        ReportSourceId::Dataset { dataset_id } => state.simulation.runs.iter().any(|run| {
            run.dataset_id == *dataset_id
                && run.dataset_content_digest() == snapshot.content_digest
                && snapshot.dataset_bindings.iter().any(|binding| {
                    binding.dataset_id == *dataset_id
                        && binding.content_digest == snapshot.content_digest
                })
        }),
        ReportSourceId::VerificationEvidence { .. } => {
            snapshot.dataset_bindings.iter().all(|binding| {
                state.simulation.runs.iter().any(|run| {
                    run.dataset_id == binding.dataset_id
                        && run.dataset_content_digest() == binding.content_digest
                })
            })
        }
        ReportSourceId::ExternalRecord { .. } => true,
    }
}

fn bounded_text_preview(value: &str, maximum_characters: usize) -> (Cow<'_, str>, bool) {
    value.char_indices().nth(maximum_characters).map_or_else(
        || (Cow::Borrowed(value), false),
        |(byte_index, _)| {
            let mut preview = String::with_capacity(byte_index.saturating_add(1));
            preview.push_str(&value[..byte_index]);
            preview.push('…');
            (Cow::Owned(preview), true)
        },
    )
}

fn report_source_label(source: &ReportSourceId) -> String {
    match source {
        ReportSourceId::VisualizationDocument { document_id } => {
            format!("visualization {document_id}")
        }
        ReportSourceId::Dataset { dataset_id } => format!("dataset {dataset_id}"),
        ReportSourceId::VerificationEvidence { evidence_id } => {
            format!("verification evidence {evidence_id}")
        }
        ReportSourceId::ExternalRecord { namespace, key } => {
            format!("external {namespace}:{key}")
        }
    }
}

fn report_block_kind_label(kind: &ReportBlockKind) -> &'static str {
    match kind {
        ReportBlockKind::PlotFigure(_) => "PLOT FIGURE",
        ReportBlockKind::DataTable(_) => "DATA TABLE",
        ReportBlockKind::Datasheet(_) => "DATASHEET",
        ReportBlockKind::Requirements(_) => "REQUIREMENTS",
        ReportBlockKind::Specifications(_) => "SPECIFICATIONS",
        ReportBlockKind::Prose(_) => "PROSE",
        ReportBlockKind::ReviewNote(_) => "REVIEW NOTE",
        ReportBlockKind::Evidence(_) => "EVIDENCE",
    }
}

fn paper_label(ui: &mut Ui, text: &str, font: egui::FontId, color: Color32) {
    ui.add(
        egui::Label::new(egui::RichText::new(text).font(font).color(color))
            .wrap()
            .selectable(true),
    );
}

fn inspector(
    ui: &mut Ui,
    document: &ReportDocument,
    selected_page: Option<ReportPageId>,
    separators: PaneSeparators,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let height = ui.available_height();
    let pane = egui::Frame::new().fill(t.color.bg_panel).show(ui, |ui| {
        ui.set_min_size(Vec2::new(width.max(1.0), height.max(1.0)));
        ScrollArea::vertical()
            .id_salt("report-authoring.inspector")
            .show(ui, |ui| {
                code_inspector_section(ui, "Report document", None, |ui| {
                    code_inspector_property_list(ui, |ui| {
                        property_row(ui, "Title", document.title());
                        property_row(ui, "Identity", &document.id().to_string());
                        property_row(ui, "Revision", &document.revision().get().to_string());
                        property_row(ui, "Template", report_template_label(document.template()));
                        property_row(ui, "Pages", &document.pages().len().to_string());
                    });
                });
                if let Some(page) = selected_page.and_then(|id| document.page(id)) {
                    code_inspector_section(ui, "Selected page", None, |ui| {
                        code_inspector_property_list(ui, |ui| {
                            property_row(ui, "Title", page.title());
                            property_row(ui, "Identity", &page.id().to_string());
                            property_row(ui, "Revision", &page.revision().get().to_string());
                            property_row(
                                ui,
                                "Update policy",
                                page_update_policy_label(page.update_policy()),
                            );
                            property_row(ui, "Sections", &page.sections().len().to_string());
                        });
                    });
                }
            });
    });
    paint_pane_separators(ui, pane.response.rect, separators, t.color.border);
}

fn paint_pane_separators(ui: &Ui, rect: Rect, separators: PaneSeparators, color: Color32) {
    let stroke = Stroke::new(1.0, color);
    if separators.top {
        let y = rect.top() + 0.5;
        ui.painter().line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            stroke,
        );
    }
    if separators.right {
        let x = rect.right() - 0.5;
        ui.painter().line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            stroke,
        );
    }
    if separators.bottom {
        let y = rect.bottom() - 0.5;
        ui.painter().line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            stroke,
        );
    }
}

fn create_document_dialog(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.workbench.report_authoring.create_document_open {
        return;
    }
    const TEMPLATE_LABELS: [&str; 3] = [
        "Release verification 4.2",
        "Design review",
        "Model qualification",
    ];
    let valid = valid_document_title(&app.state.workbench.report_authoring.create_document_title);
    let writable = report_mutation_allowed(&app.state);
    let error = app
        .state
        .workbench
        .report_authoring
        .transaction_error
        .clone();
    let choice = Dialog::new(
        "REPORT AUTHORING · TRACEABLE DERIVED EVIDENCE",
        "Plan report artifact",
        "Create report document",
    )
    .description(
        "Create one explicit project-owned report source and its mockup-specified page outline.",
    )
    .ghost("Cancel")
    .primary_enabled(valid && writable)
    .initial_focus(DialogInitialFocus::BodyControl)
    .show_with_initial_body_focus(ctx, |ui| {
        let response = input_row(
            ui,
            "Report title",
            &mut app
                .state
                .workbench
                .report_authoring
                .create_document_title,
        );
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.set_width(ui.available_width());
            let label_width = 130.0_f32.min(ui.available_width() * 0.32);
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Template"),
            );
            let selected = app
                .state
                .workbench
                .report_authoring
                .create_document_template
                .min(TEMPLATE_LABELS.len() - 1);
            let options = TEMPLATE_LABELS
                .iter()
                .map(|label| (*label).to_owned())
                .collect::<Vec<_>>();
            if let Some(index) = select(
                ui,
                "report-document-template",
                "Report document template",
                TEMPLATE_LABELS[selected],
                &options,
                ui.available_width(),
            ) {
                app.state
                    .workbench
                    .report_authoring
                    .create_document_template = index;
            }
        });
        ui.add_space(8.0);
        ui.label(
            "The document is created only when this transaction commits; opening Report Authoring never changes the project.",
        );
        if !writable {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.err,
                report_mutation_block_reason(&app.state),
            );
        }
        if let Some(error) = &error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
        Some(response.id)
    });
    match choice {
        DialogChoice::Primary if valid && writable => commit_create_document(app),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state.workbench.report_authoring.create_document_open = false;
            app.state.workbench.report_authoring.transaction_error = None;
        }
        _ => {}
    }
}

fn commit_create_document(app: &mut RSpiceApp) {
    if !report_mutation_allowed(&app.state) {
        app.state.workbench.report_authoring.transaction_error =
            Some(report_mutation_block_reason(&app.state).to_owned());
        return;
    }
    let title = app
        .state
        .workbench
        .report_authoring
        .create_document_title
        .to_owned();
    if !valid_document_title(&title) {
        app.state.workbench.report_authoring.transaction_error = Some(
            "The report title must be trimmed, non-empty, single-line text of at most 512 characters."
                .to_owned(),
        );
        return;
    }
    let template = report_template_from_index(
        app.state
            .workbench
            .report_authoring
            .create_document_template,
    );
    let initial_evidence_binding = app
        .state
        .simulation
        .active_run()
        .filter(|run| !run.analyses.is_empty())
        .or_else(|| {
            app.state
                .simulation
                .newest_retained_result_run_index()
                .and_then(|index| app.state.simulation.runs.get(index))
        })
        .map(|run| ReportPageEvidenceBinding::ExactDataset {
            binding: crate::product::DatasetBinding::new(
                run.dataset_id,
                run.dataset_content_digest(),
            ),
        });
    let result = ReportDocument::new_with_template(title, template)
        .map_err(|error| error.to_string())
        .and_then(|mut document| {
            let edits = INITIAL_PAGES
                .iter()
                .map(|(_, title)| ReportEdit::AddPage {
                    title: (*title).to_owned(),
                })
                .collect();
            document
                .transact_with_context(
                    document.revision(),
                    edits,
                    timestamp_unix_ms(),
                    "rspice-local-session",
                    format!("Create {} report outline", report_template_label(template)),
                )
                .map_err(|error| error.to_string())?;
            if let Some(evidence_binding) = initial_evidence_binding {
                let edits = document
                    .pages()
                    .iter()
                    .map(|page| ReportEdit::SetPageEvidenceBinding {
                        page_id: page.id(),
                        expected_page_revision: page.revision(),
                        evidence_binding,
                    })
                    .collect();
                document
                    .transact_with_context(
                        document.revision(),
                        edits,
                        timestamp_unix_ms(),
                        "rspice-local-session",
                        "Bind initial report pages to active result dataset",
                    )
                    .map_err(|error| error.to_string())?;
            }
            Ok(document)
        });
    match result {
        Ok(document) => {
            let document_id = document.id();
            let page_id = document.pages().first().map(|page| page.id());
            app.state.workspace.report_documents.push(document);
            app.state.workspace.report_documents_dirty = true;
            let editor = &mut app.state.workbench.report_authoring;
            editor.selected_document = Some(document_id);
            editor.selected_page = page_id;
            editor.preview_block_page = 0;
            editor.create_document_open = false;
            editor.transaction_error = None;
        }
        Err(error) => app.state.workbench.report_authoring.transaction_error = Some(error),
    }
}

fn add_page_dialog(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.workbench.report_authoring.add_page_open {
        return;
    }
    let valid = valid_page_title(&app.state.workbench.report_authoring.add_page_title);
    let error = app
        .state
        .workbench
        .report_authoring
        .transaction_error
        .clone();
    let choice = Dialog::new(
        "REPORTING · DOCUMENT COMPOSITION",
        "Add report page",
        "Add page",
    )
    .description("Add one versioned page to the project-owned report document.")
    .ghost("Cancel")
    .primary_enabled(valid)
    .initial_focus(DialogInitialFocus::BodyControl)
    .show_with_initial_body_focus(ctx, |ui| {
        let response = input_row(
            ui,
            "Page title",
            &mut app.state.workbench.report_authoring.add_page_title,
        );
        if let Some(error) = &error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
        Some(response.id)
    });
    match choice {
        DialogChoice::Primary if valid => commit_add_page(app),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state.workbench.report_authoring.add_page_open = false;
            app.state.workbench.report_authoring.transaction_error = None;
        }
        _ => {}
    }
}

fn commit_add_page(app: &mut RSpiceApp) {
    if !report_mutation_allowed(&app.state) {
        app.state.workbench.report_authoring.transaction_error =
            Some(report_mutation_block_reason(&app.state).to_owned());
        return;
    }
    let title = app
        .state
        .workbench
        .report_authoring
        .add_page_title
        .trim()
        .to_owned();
    let timestamp = timestamp_unix_ms();
    let result = active_document_mut(&mut app.state).and_then(|document| {
        document
            .transact_with_context(
                document.revision(),
                vec![ReportEdit::AddPage { title }],
                timestamp,
                "rspice-local-session",
                "Add report page",
            )
            .map_err(|error| error.to_string())
    });
    match result {
        Ok(receipt) => {
            let created_page = receipt.created.iter().find_map(|entity| match entity {
                ReportEntityRef::Page(id) => Some(*id),
                _ => None,
            });
            app.state.workbench.report_authoring.selected_page = created_page;
            app.state.workbench.report_authoring.preview_block_page = 0;
            app.state.workbench.report_authoring.add_page_open = false;
            app.state.workbench.report_authoring.transaction_error = None;
            app.state.workspace.report_documents_dirty = true;
        }
        Err(error) => app.state.workbench.report_authoring.transaction_error = Some(error),
    }
}

fn page_properties_dialog(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.workbench.report_authoring.page_properties_open {
        return;
    }
    let valid = valid_page_title(&app.state.workbench.report_authoring.page_title_draft);
    let error = app
        .state
        .workbench
        .report_authoring
        .transaction_error
        .clone();
    let Some(document) = active_document(&app.state).cloned() else {
        app.state.workbench.report_authoring.page_properties_open = false;
        return;
    };
    let page_options = document
        .pages()
        .iter()
        .enumerate()
        .map(|(index, page)| format!("{} · {}", page_marker(index, page.title()), page.title()))
        .collect::<Vec<_>>();
    let page_index = app
        .state
        .workbench
        .report_authoring
        .page_properties_page
        .and_then(|page_id| {
            document
                .pages()
                .iter()
                .position(|page| page.id() == page_id)
        })
        .unwrap_or_default();
    const TEMPLATE_LABELS: [&str; 3] = [
        "Release verification 4.2",
        "Design review",
        "Model qualification",
    ];
    const UPDATE_POLICY_LABELS: [&str; 2] = [
        "Refresh linked figures automatically",
        "Freeze selected figure revision",
    ];
    let choice = Dialog::new(
        "REPORTING · DOCUMENT COMPOSITION",
        "Report page properties",
        "Save page properties",
    )
    .description("Edit the selected page through one revision-checked report transaction.")
    .ghost("Cancel")
    .primary_enabled(valid)
    .initial_focus(DialogInitialFocus::BodyControl)
    .show_with_initial_body_focus(ctx, |ui| {
        let label_width = 130.0_f32.min(ui.available_width() * 0.32);
        ui.horizontal(|ui| {
            ui.set_width(ui.available_width());
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Template"),
            );
            let selected_index = app
                .state
                .workbench
                .report_authoring
                .report_template_draft
                .min(TEMPLATE_LABELS.len() - 1);
            let options = TEMPLATE_LABELS
                .iter()
                .map(|label| (*label).to_owned())
                .collect::<Vec<_>>();
            if let Some(index) = select(
                ui,
                "report-page-template",
                "Report template",
                TEMPLATE_LABELS[selected_index],
                &options,
                ui.available_width(),
            ) {
                app.state.workbench.report_authoring.report_template_draft = index;
            }
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.set_width(ui.available_width());
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Page"),
            );
            if let Some(index) = select(
                ui,
                "report-page-selection",
                "Report page",
                page_options
                    .get(page_index)
                    .map_or("No page", String::as_str),
                &page_options,
                ui.available_width(),
            ) && let Some(page) = document.pages().get(index)
            {
                let editor = &mut app.state.workbench.report_authoring;
                editor.page_properties_page = Some(page.id());
                editor.page_title_draft = page.title().to_owned();
                editor.page_update_policy_draft = page_update_policy_index(page.update_policy());
                editor.transaction_error = None;
            }
        });
        ui.add_space(8.0);
        let response = input_row(
            ui,
            "Page title",
            &mut app.state.workbench.report_authoring.page_title_draft,
        );
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.set_width(ui.available_width());
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Update policy"),
            );
            let selected_index = app
                .state
                .workbench
                .report_authoring
                .page_update_policy_draft
                .min(UPDATE_POLICY_LABELS.len() - 1);
            let options = UPDATE_POLICY_LABELS
                .iter()
                .map(|label| (*label).to_owned())
                .collect::<Vec<_>>();
            if let Some(index) = select(
                ui,
                "report-page-update-policy",
                "Report page update policy",
                UPDATE_POLICY_LABELS[selected_index],
                &options,
                ui.available_width(),
            ) {
                app.state
                    .workbench
                    .report_authoring
                    .page_update_policy_draft = index;
            }
        });
        if let Some(error) = &error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
        Some(response.id)
    });
    match choice {
        DialogChoice::Primary if valid => commit_page_properties(app),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state.workbench.report_authoring.page_properties_open = false;
            app.state.workbench.report_authoring.transaction_error = None;
        }
        _ => {}
    }
}

fn open_add_report_element(app: &mut RSpiceApp) {
    if !report_mutation_allowed(&app.state) || active_document(&app.state).is_none() {
        return;
    }
    let editor = &mut app.state.workbench.report_authoring;
    editor.add_report_element_kind = 0;
    editor.add_report_element_title = "Engineering summary".to_owned();
    editor.add_report_element_primary =
        "Describe the conclusion and its supporting evidence.".to_owned();
    editor.add_report_element_secondary.clear();
    editor.add_report_element_tertiary.clear();
    editor.add_report_element_style = 0;
    editor.add_report_element_status = 0;
    editor.add_report_element_source_run = 0;
    editor.add_report_element_open = true;
    editor.transaction_error = None;
}

fn add_report_element_dialog(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.workbench.report_authoring.add_report_element_open {
        return;
    }
    const KIND_LABELS: [&str; 7] = [
        "Authored prose",
        "Data table",
        "Datasheet field",
        "Requirement statement",
        "Specification result",
        "Review note",
        "Verification evidence",
    ];
    const PROSE_STYLE_LABELS: [&str; 5] = [
        "Body",
        "Executive summary",
        "Method",
        "Conclusion",
        "Warning",
    ];
    let run_options = app
        .state
        .simulation
        .runs
        .iter()
        .filter(|run| !run.analyses.is_empty())
        .map(|run| format!("Run {} · immutable dataset", run.id))
        .collect::<Vec<_>>();
    let kind_index = app
        .state
        .workbench
        .report_authoring
        .add_report_element_kind
        .min(KIND_LABELS.len() - 1);
    let source_required = matches!(kind_index, 1 | 2 | 3 | 4 | 6);
    let valid = valid_add_report_element_draft(&app.state, !run_options.is_empty());
    let writable = report_mutation_allowed(&app.state);
    let error = app
        .state
        .workbench
        .report_authoring
        .transaction_error
        .clone();
    let choice = Dialog::new(
        "REPORT AUTHORING · PAGE ELEMENT CATALOG",
        "Add report element",
        "Add element",
    )
    .description(
        "Create one validated report element. Source-derived elements bind to one exact immutable dataset; result plots use Insert result document.",
    )
    .ghost("Cancel")
    .primary_enabled(valid && writable)
    .initial_focus(DialogInitialFocus::BodyControl)
    .show_with_initial_body_focus(ctx, |ui| {
        let label_width = 134.0_f32.min(ui.available_width() * 0.34);
        ui.horizontal(|ui| {
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Element type"),
            );
            let options = KIND_LABELS
                .iter()
                .map(|label| (*label).to_owned())
                .collect::<Vec<_>>();
            if let Some(index) = select(
                ui,
                "report-add-element-kind",
                "Report element type",
                KIND_LABELS[kind_index],
                &options,
                ui.available_width(),
            ) {
                reset_add_report_element_kind(app, index);
            }
        });
        ui.add_space(8.0);
        let focus = input_row(
            ui,
            if kind_index == 5 { "Author" } else { "Element title" },
            &mut app
                .state
                .workbench
                .report_authoring
                .add_report_element_title,
        );

        let (primary_label, secondary_label, tertiary_label) =
            add_report_element_field_labels(kind_index);
        ui.add_space(8.0);
        if matches!(kind_index, 0 | 3 | 5 | 6) {
            dialog_text_area(
                ui,
                primary_label,
                &mut app
                    .state
                    .workbench
                    .report_authoring
                    .add_report_element_primary,
            );
        } else {
            input_row(
                ui,
                primary_label,
                &mut app
                    .state
                    .workbench
                    .report_authoring
                    .add_report_element_primary,
            );
        }
        if let Some(label) = secondary_label {
            ui.add_space(8.0);
            input_row(
                ui,
                label,
                &mut app
                    .state
                    .workbench
                    .report_authoring
                    .add_report_element_secondary,
            );
        }
        if let Some(label) = tertiary_label {
            ui.add_space(8.0);
            input_row(
                ui,
                label,
                &mut app
                    .state
                    .workbench
                    .report_authoring
                    .add_report_element_tertiary,
            );
        }

        if kind_index == 0 {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add_sized(
                    Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                    egui::Label::new("Prose style"),
                );
                let style_index = app
                    .state
                    .workbench
                    .report_authoring
                    .add_report_element_style
                    .min(PROSE_STYLE_LABELS.len() - 1);
                let labels = PROSE_STYLE_LABELS
                    .iter()
                    .map(|label| (*label).to_owned())
                    .collect::<Vec<_>>();
                if let Some(index) = select(
                    ui,
                    "report-add-prose-style",
                    "Report prose style",
                    PROSE_STYLE_LABELS[style_index],
                    &labels,
                    ui.available_width(),
                ) {
                    app.state
                        .workbench
                        .report_authoring
                        .add_report_element_style = index;
                }
            });
        }
        if matches!(kind_index, 3 | 4 | 5) {
            ui.add_space(8.0);
            let status_labels: &[&str] = match kind_index {
                3 => &["Not evaluated", "Passed", "Failed", "Waived"],
                4 => &[
                    "Not evaluated",
                    "In specification",
                    "Out of specification",
                    "Informational",
                ],
                _ => &["Open", "Addressed", "Accepted"],
            };
            ui.horizontal(|ui| {
                ui.add_sized(
                    Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                    egui::Label::new("Status"),
                );
                let status_index = app
                    .state
                    .workbench
                    .report_authoring
                    .add_report_element_status
                    .min(status_labels.len() - 1);
                let labels = status_labels
                    .iter()
                    .map(|label| (*label).to_owned())
                    .collect::<Vec<_>>();
                if let Some(index) = select(
                    ui,
                    "report-add-element-status",
                    "Report element status",
                    status_labels[status_index],
                    &labels,
                    ui.available_width(),
                ) {
                    app.state
                        .workbench
                        .report_authoring
                        .add_report_element_status = index;
                }
            });
        }
        if source_required {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add_sized(
                    Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                    egui::Label::new("Bound source"),
                );
                let run_index = app
                    .state
                    .workbench
                    .report_authoring
                    .add_report_element_source_run
                    .min(run_options.len().saturating_sub(1));
                let current = run_options
                    .get(run_index)
                    .map_or("No immutable dataset retained", String::as_str);
                if let Some(index) = select(
                    ui,
                    "report-add-element-source",
                    "Immutable dataset source",
                    current,
                    &run_options,
                    ui.available_width(),
                ) {
                    app.state
                        .workbench
                        .report_authoring
                        .add_report_element_source_run = index;
                }
            });
            if run_options.is_empty() {
                ui.colored_label(
                    Tokens::get(ui.ctx()).color.warn,
                    "Run and retain an analysis before adding this source-derived element.",
                );
            }
        }
        if !writable {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.err,
                report_mutation_block_reason(&app.state),
            );
        }
        if let Some(error) = &error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
        Some(focus.id)
    });
    match choice {
        DialogChoice::Primary if valid && writable => commit_add_report_element(app),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state.workbench.report_authoring.add_report_element_open = false;
            app.state.workbench.report_authoring.transaction_error = None;
        }
        _ => {}
    }
}

fn dialog_text_area(ui: &mut Ui, label: &str, value: &mut String) -> egui::Response {
    let label_width = 134.0_f32.min(ui.available_width() * 0.34);
    ui.horizontal_top(|ui| {
        ui.add_sized(Vec2::new(label_width, 82.0), egui::Label::new(label));
        ui.add_sized(
            Vec2::new(ui.available_width(), 82.0),
            egui::TextEdit::multiline(value)
                .desired_rows(4)
                .font(theme::mono(tokens::FS_1, FontWeight::Regular)),
        )
    })
    .inner
}

fn add_report_element_field_labels(
    kind_index: usize,
) -> (&'static str, Option<&'static str>, Option<&'static str>) {
    match kind_index {
        1 => (
            "Column heading",
            Some("Cell value"),
            Some("Unit (optional)"),
        ),
        2 => ("Field label", Some("Field value"), Some("Unit (optional)")),
        3 => (
            "Requirement statement",
            Some("Requirement ID"),
            Some("Evidence label (optional)"),
        ),
        4 => (
            "Expression",
            Some("Limit"),
            Some("Measured value (optional)"),
        ),
        5 => ("Review message", None, None),
        6 => ("Evidence summary", None, None),
        _ => ("Report text", None, None),
    }
}

fn reset_add_report_element_kind(app: &mut RSpiceApp, kind_index: usize) {
    let editor = &mut app.state.workbench.report_authoring;
    editor.add_report_element_kind = kind_index.min(6);
    let (title, primary, secondary) = match editor.add_report_element_kind {
        1 => ("Data table", "Value", "0"),
        2 => ("Datasheet", "Parameter", "Value"),
        3 => ("Requirement", "State the requirement.", "REQ-1"),
        4 => ("Specification", "V(out)", "<= 1 V"),
        5 => ("rspice-local-session", "Review note.", ""),
        6 => (
            "Verification evidence",
            "Summarize the retained evidence.",
            "",
        ),
        _ => (
            "Engineering summary",
            "Describe the conclusion and its supporting evidence.",
            "",
        ),
    };
    editor.add_report_element_title = title.to_owned();
    editor.add_report_element_primary = primary.to_owned();
    editor.add_report_element_secondary = secondary.to_owned();
    editor.add_report_element_tertiary.clear();
    editor.add_report_element_style = 0;
    editor.add_report_element_status = 0;
    editor.transaction_error = None;
}

fn valid_add_report_element_draft(state: &AppState, source_available: bool) -> bool {
    let editor = &state.workbench.report_authoring;
    let kind = editor.add_report_element_kind.min(6);
    let title = editor.add_report_element_title.trim();
    let primary = editor.add_report_element_primary.trim();
    let secondary = editor.add_report_element_secondary.trim();
    let source_valid = !matches!(kind, 1 | 2 | 3 | 4 | 6) || source_available;
    let title_limit = if kind == 5 { 256 } else { 512 };
    let primary_limit = match kind {
        1 | 2 => 256,
        4 => 4_096,
        _ => 65_536,
    };
    let secondary_valid = match kind {
        1 | 2 => !secondary.is_empty() && secondary.len() <= 16_384,
        3 => {
            !secondary.is_empty()
                && secondary.len() <= 256
                && !secondary.chars().any(|character| {
                    character.is_control()
                        || character.is_whitespace()
                        || matches!(character, '/' | '\\')
                })
        }
        4 => !secondary.is_empty() && secondary.len() <= 4_096,
        _ => true,
    };
    let tertiary_valid = match kind {
        1 | 2 => editor.add_report_element_tertiary.trim().len() <= 64,
        3 => editor.add_report_element_tertiary.trim().len() <= 512,
        4 => editor.add_report_element_tertiary.trim().len() <= 4_096,
        _ => editor.add_report_element_tertiary.trim().is_empty(),
    };
    !title.is_empty()
        && title.len() <= title_limit
        && !title.chars().any(char::is_control)
        && !primary.is_empty()
        && primary.len() <= primary_limit
        && !primary
            .chars()
            .any(|ch| ch.is_control() && ch != '\n' && ch != '\t')
        && secondary_valid
        && tertiary_valid
        && source_valid
}

fn report_dataset_snapshot(
    state: &AppState,
    filtered_run_index: usize,
) -> Result<(ReportReferenceSnapshot, crate::product::DatasetBinding), String> {
    let run = state
        .simulation
        .runs
        .iter()
        .filter(|run| !run.analyses.is_empty())
        .nth(filtered_run_index)
        .ok_or_else(|| "The selected immutable dataset is no longer retained.".to_owned())?;
    let binding = crate::product::DatasetBinding::new(run.dataset_id, run.dataset_content_digest());
    let snapshot = ReportReferenceSnapshot::new(
        ReportSourceId::Dataset {
            dataset_id: binding.dataset_id,
        },
        None,
        binding.content_digest,
        vec![binding],
    )
    .map_err(|error| error.to_string())?;
    Ok((snapshot, binding))
}

fn commit_add_report_element(app: &mut RSpiceApp) {
    if !report_mutation_allowed(&app.state) {
        app.state.workbench.report_authoring.transaction_error =
            Some(report_mutation_block_reason(&app.state).to_owned());
        return;
    }
    let editor = &app.state.workbench.report_authoring;
    let kind_index = editor.add_report_element_kind.min(6);
    let title = editor.add_report_element_title.trim().to_owned();
    let primary = editor.add_report_element_primary.trim().to_owned();
    let secondary = editor.add_report_element_secondary.trim().to_owned();
    let tertiary = editor.add_report_element_tertiary.trim().to_owned();
    let style = editor.add_report_element_style;
    let status = editor.add_report_element_status;
    let source_index = editor.add_report_element_source_run;
    let reference = if matches!(kind_index, 1 | 2 | 3 | 4 | 6) {
        match report_dataset_snapshot(&app.state, source_index) {
            Ok((snapshot, _)) => Some(ReportReferenceMode::Linked { snapshot }),
            Err(error) => {
                app.state.workbench.report_authoring.transaction_error = Some(error);
                return;
            }
        }
    } else {
        None
    };
    let timestamp = timestamp_unix_ms();
    let kind = match kind_index {
        1 => ReportBlockKind::DataTable(DataTableBlock {
            title,
            columns: vec![TableColumn {
                key: report_field_key(&primary),
                heading: primary,
                unit: (!tertiary.is_empty()).then_some(tertiary),
            }],
            rows: vec![vec![TableCell::Text(secondary)]],
            reference: reference.expect("source-derived table has reference"),
        }),
        2 => ReportBlockKind::Datasheet(DatasheetBlock {
            title,
            fields: vec![DatasheetField {
                key: report_field_key(&primary),
                label: primary,
                value: secondary,
                unit: (!tertiary.is_empty()).then_some(tertiary),
            }],
            reference: reference.expect("source-derived datasheet has reference"),
        }),
        3 => ReportBlockKind::Requirements(RequirementsBlock {
            title,
            entries: vec![RequirementEntry {
                requirement_id: secondary,
                statement: primary,
                disposition: match status {
                    1 => RequirementDisposition::Passed,
                    2 => RequirementDisposition::Failed,
                    3 => RequirementDisposition::Waived,
                    _ => RequirementDisposition::NotEvaluated,
                },
                evidence_label: (!tertiary.is_empty()).then_some(tertiary),
            }],
            reference: reference.expect("source-derived requirement has reference"),
        }),
        4 => ReportBlockKind::Specifications(SpecificationsBlock {
            title,
            entries: vec![SpecificationEntry {
                expression: primary,
                limit: secondary,
                measured: (!tertiary.is_empty()).then_some(tertiary),
                disposition: match status {
                    1 => SpecificationDisposition::InSpecification,
                    2 => SpecificationDisposition::OutOfSpecification,
                    3 => SpecificationDisposition::Informational,
                    _ => SpecificationDisposition::NotEvaluated,
                },
            }],
            reference: reference.expect("source-derived specification has reference"),
        }),
        5 => ReportBlockKind::ReviewNote(ReviewNoteBlock {
            author: title,
            status: match status {
                1 => ReviewNoteStatus::Addressed,
                2 => ReviewNoteStatus::Accepted,
                _ => ReviewNoteStatus::Open,
            },
            message: primary,
            created_at_unix_ms: timestamp,
            resolved_at_unix_ms: (status != 0).then_some(timestamp),
        }),
        6 => {
            let (_, binding) = match report_dataset_snapshot(&app.state, source_index) {
                Ok(value) => value,
                Err(error) => {
                    app.state.workbench.report_authoring.transaction_error = Some(error);
                    return;
                }
            };
            let mut identity_material = Vec::with_capacity(48);
            identity_material.extend_from_slice(binding.dataset_id.as_uuid().as_bytes());
            identity_material.extend_from_slice(binding.content_digest.as_bytes());
            let evidence_id = crate::product::VerificationEvidenceId::from_namespace(
                uuid::Uuid::from_bytes([
                    0x6f, 0x34, 0x1a, 0x28, 0xca, 0x3e, 0x4e, 0x62, 0x9e, 0x4c, 0x77, 0xa0, 0x72,
                    0x44, 0x1b, 0x0f,
                ]),
                &identity_material,
            );
            let dataset_reference = reference.expect("source-derived evidence has reference");
            let snapshot = dataset_reference.snapshot();
            let evidence_snapshot = match ReportReferenceSnapshot::new(
                ReportSourceId::VerificationEvidence { evidence_id },
                None,
                snapshot.content_digest,
                snapshot.dataset_bindings.clone(),
            ) {
                Ok(snapshot) => snapshot,
                Err(error) => {
                    app.state.workbench.report_authoring.transaction_error =
                        Some(error.to_string());
                    return;
                }
            };
            ReportBlockKind::Evidence(EvidenceBlock {
                title,
                summary: primary,
                reference: ReportReferenceMode::Linked {
                    snapshot: evidence_snapshot,
                },
            })
        }
        _ => ReportBlockKind::Prose(ProseBlock {
            style: match style {
                1 => ProseStyle::ExecutiveSummary,
                2 => ProseStyle::Method,
                3 => ProseStyle::Conclusion,
                4 => ProseStyle::Warning,
                _ => ProseStyle::Body,
            },
            markdown: format!("## {title}\n\n{primary}"),
        }),
    };
    let Some(page_id) =
        active_document(&app.state).and_then(|document| selected_page_id(&app.state, document))
    else {
        app.state.workbench.report_authoring.transaction_error =
            Some("Select a report page before adding an element.".to_owned());
        return;
    };
    let result = active_document_mut(&mut app.state).and_then(|document| {
        let page = document
            .page(page_id)
            .ok_or_else(|| "The selected report page no longer exists.".to_owned())?;
        document
            .transact_with_context(
                document.revision(),
                vec![ReportEdit::AddBlockToPage {
                    page_id,
                    expected_page_revision: page.revision(),
                    kind,
                }],
                timestamp,
                "rspice-local-session",
                "Add report page element",
            )
            .map_err(|error| error.to_string())
    });
    match result {
        Ok(receipt) => {
            app.state.workbench.report_authoring.selected_report_block =
                receipt.created.iter().find_map(|entity| match entity {
                    ReportEntityRef::Block(id) => Some(*id),
                    _ => None,
                });
            app.state.workbench.report_authoring.add_report_element_open = false;
            app.state.workbench.report_authoring.preview_block_page = 0;
            app.state.workbench.report_authoring.transaction_error = None;
            app.state.workspace.report_documents_dirty = true;
        }
        Err(error) => app.state.workbench.report_authoring.transaction_error = Some(error),
    }
}

fn report_field_key(label: &str) -> String {
    let mut key = String::with_capacity(label.len().min(128));
    let mut pending_separator = false;
    for ch in label.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending_separator && !key.is_empty() {
                key.push('_');
            }
            pending_separator = false;
            key.push(ch.to_ascii_lowercase());
        } else {
            pending_separator = true;
        }
        if key.len() >= 128 {
            break;
        }
    }
    if key.is_empty() {
        "value".to_owned()
    } else {
        key
    }
}

fn remove_report_block_dialog(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app
        .state
        .workbench
        .report_authoring
        .remove_report_block_open
    {
        return;
    }
    let block_id = app.state.workbench.report_authoring.selected_report_block;
    let block_title = block_id
        .and_then(|id| active_document(&app.state)?.block(id))
        .map(|block| report_block_element_title(block.kind()).into_owned())
        .unwrap_or_else(|| "Unavailable report element".to_owned());
    let valid = block_id.is_some_and(|id| {
        active_document(&app.state).is_some_and(|document| document.block(id).is_some())
    });
    let writable = report_mutation_allowed(&app.state);
    let error = app
        .state
        .workbench
        .report_authoring
        .transaction_error
        .clone();
    let choice = Dialog::new(
        "REPORT AUTHORING · PAGE ELEMENT",
        "Remove page element",
        "Remove",
    )
    .description(
        "Remove the selected element from this report revision. Its stable identity is retained as a tombstone in the report audit history.",
    )
    .ghost("Cancel")
    .primary_enabled(valid && writable)
    .initial_focus(DialogInitialFocus::Primary)
    .show(ctx, |ui| {
        ui.label(format!("Element: {block_title}"));
        if !writable {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.err,
                report_mutation_block_reason(&app.state),
            );
        }
        if let Some(error) = &error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
    });
    match choice {
        DialogChoice::Primary if valid && writable => commit_remove_report_block(app),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state
                .workbench
                .report_authoring
                .remove_report_block_open = false;
            app.state.workbench.report_authoring.transaction_error = None;
        }
        _ => {}
    }
}

fn commit_remove_report_block(app: &mut RSpiceApp) {
    if !report_mutation_allowed(&app.state) {
        app.state.workbench.report_authoring.transaction_error =
            Some(report_mutation_block_reason(&app.state).to_owned());
        return;
    }
    let Some(block_id) = app.state.workbench.report_authoring.selected_report_block else {
        app.state.workbench.report_authoring.transaction_error =
            Some("Select a report page element before removing it.".to_owned());
        return;
    };
    let result = active_document_mut(&mut app.state).and_then(|document| {
        let block = document
            .block(block_id)
            .ok_or_else(|| "The selected report element no longer exists.".to_owned())?;
        document
            .transact_with_context(
                document.revision(),
                vec![ReportEdit::Remove {
                    entity: ReportEntityRef::Block(block_id),
                    expected_entity_revision: block.revision(),
                }],
                timestamp_unix_ms(),
                "rspice-local-session",
                "Remove report page element",
            )
            .map_err(|error| error.to_string())
    });
    match result {
        Ok(_) => {
            app.state.workbench.report_authoring.selected_report_block = None;
            app.state
                .workbench
                .report_authoring
                .remove_report_block_open = false;
            app.state.workbench.report_authoring.preview_block_page = 0;
            app.state.workbench.report_authoring.transaction_error = None;
            app.state.workspace.report_documents_dirty = true;
        }
        Err(error) => app.state.workbench.report_authoring.transaction_error = Some(error),
    }
}

fn open_insert_result_document(app: &mut RSpiceApp) {
    if !report_mutation_allowed(&app.state)
        || app.state.workspace.visualization_documents.is_empty()
    {
        return;
    }
    let source = &app.state.workspace.visualization_documents[0];
    let editor = &mut app.state.workbench.report_authoring;
    editor.insert_result_document_index = 0;
    editor.insert_result_caption = source.title().to_owned();
    editor.insert_result_alternative_text = format!(
        "Result document {} at immutable revision {}.",
        source.title(),
        source.revision().get()
    );
    editor.insert_result_sizing = 0;
    editor.insert_result_frozen = false;
    editor.insert_result_document_open = true;
    editor.transaction_error = None;
}

fn insert_result_document_dialog(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app
        .state
        .workbench
        .report_authoring
        .insert_result_document_open
    {
        return;
    }
    const SIZING_LABELS: [&str; 3] = ["Fit width", "Fit page", "Natural size"];
    let source_options = app
        .state
        .workspace
        .visualization_documents
        .iter()
        .map(|document| {
            format!(
                "{} · revision {} · immutable source",
                document.title(),
                document.revision().get()
            )
        })
        .collect::<Vec<_>>();
    let source_index = app
        .state
        .workbench
        .report_authoring
        .insert_result_document_index
        .min(source_options.len().saturating_sub(1));
    let caption = app
        .state
        .workbench
        .report_authoring
        .insert_result_caption
        .trim();
    let alternative_text = app
        .state
        .workbench
        .report_authoring
        .insert_result_alternative_text
        .trim();
    let valid = !source_options.is_empty()
        && !caption.is_empty()
        && caption.len() <= 2_048
        && !caption.chars().any(char::is_control)
        && !alternative_text.is_empty()
        && alternative_text.len() <= 8_192
        && !alternative_text
            .chars()
            .any(|ch| ch.is_control() && ch != '\n' && ch != '\t');
    let writable = report_mutation_allowed(&app.state);
    let error = app
        .state
        .workbench
        .report_authoring
        .transaction_error
        .clone();
    let choice = Dialog::new(
        "REPORT AUTHORING · IMMUTABLE RESULT SOURCE",
        "Insert result document",
        "Insert result document",
    )
    .description(
        "Insert one exact visualization-document revision with all immutable dataset bindings retained for publication audit.",
    )
    .ghost("Cancel")
    .primary_enabled(valid && writable)
    .initial_focus(DialogInitialFocus::BodyControl)
    .show_with_initial_body_focus(ctx, |ui| {
        let label_width = 132.0_f32.min(ui.available_width() * 0.34);
        ui.horizontal(|ui| {
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Result document"),
            );
            let current = source_options
                .get(source_index)
                .map_or("No retained result documents", String::as_str);
            if let Some(index) = select(
                ui,
                "report-insert-result-source",
                "Immutable result document",
                current,
                &source_options,
                ui.available_width(),
            ) {
                app.state
                    .workbench
                    .report_authoring
                    .insert_result_document_index = index;
            }
        });
        ui.add_space(8.0);
        let focus = input_row(
            ui,
            "Caption",
            &mut app
                .state
                .workbench
                .report_authoring
                .insert_result_caption,
        );
        ui.add_space(8.0);
        input_row(
            ui,
            "Alternative text",
            &mut app
                .state
                .workbench
                .report_authoring
                .insert_result_alternative_text,
        );
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.add_sized(
                Vec2::new(label_width, Tokens::get(ui.ctx()).metrics.ctl_h),
                egui::Label::new("Sizing"),
            );
            let sizing_index = app
                .state
                .workbench
                .report_authoring
                .insert_result_sizing
                .min(SIZING_LABELS.len() - 1);
            let labels = SIZING_LABELS
                .iter()
                .map(|label| (*label).to_owned())
                .collect::<Vec<_>>();
            if let Some(index) = select(
                ui,
                "report-insert-result-sizing",
                "Result figure sizing",
                SIZING_LABELS[sizing_index],
                &labels,
                ui.available_width(),
            ) {
                app.state
                    .workbench
                    .report_authoring
                    .insert_result_sizing = index;
            }
        });
        ui.add_space(8.0);
        ui.checkbox(
            &mut app
                .state
                .workbench
                .report_authoring
                .insert_result_frozen,
            "Freeze self-contained source payload in this report revision",
        );
        ui.label(if app
            .state
            .workbench
            .report_authoring
            .insert_result_frozen
        {
            "The exact validated visualization source is embedded and digest-authenticated."
        } else {
            "The block remains linked to one exact immutable visualization revision."
        });
        if !writable {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.err,
                report_mutation_block_reason(&app.state),
            );
        }
        if let Some(error) = &error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
        Some(focus.id)
    });
    match choice {
        DialogChoice::Primary if valid && writable => commit_insert_result_document(app),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state
                .workbench
                .report_authoring
                .insert_result_document_open = false;
            app.state.workbench.report_authoring.transaction_error = None;
        }
        _ => {}
    }
}

fn commit_insert_result_document(app: &mut RSpiceApp) {
    if !report_mutation_allowed(&app.state) {
        app.state.workbench.report_authoring.transaction_error =
            Some(report_mutation_block_reason(&app.state).to_owned());
        return;
    }
    let source_index = app
        .state
        .workbench
        .report_authoring
        .insert_result_document_index;
    let Some(source) = app
        .state
        .workspace
        .visualization_documents
        .get(source_index)
        .cloned()
    else {
        app.state.workbench.report_authoring.transaction_error =
            Some("The selected result document no longer exists.".to_owned());
        return;
    };
    let caption = app
        .state
        .workbench
        .report_authoring
        .insert_result_caption
        .trim()
        .to_owned();
    let alternative_text = app
        .state
        .workbench
        .report_authoring
        .insert_result_alternative_text
        .trim()
        .to_owned();
    let sizing = match app.state.workbench.report_authoring.insert_result_sizing {
        1 => FigureSizing::FitPage,
        2 => FigureSizing::Natural,
        _ => FigureSizing::FitWidth,
    };
    let source_digest = match source.content_digest() {
        Ok(digest) => digest,
        Err(error) => {
            app.state.workbench.report_authoring.transaction_error =
                Some(format!("The result document is invalid: {error}"));
            return;
        }
    };
    let dataset_bindings = source
        .datasets()
        .iter()
        .map(|dataset| dataset.binding())
        .collect::<Vec<_>>();
    let snapshot = match ReportReferenceSnapshot::new(
        ReportSourceId::VisualizationDocument {
            document_id: source.id(),
        },
        Some(source.revision()),
        source_digest,
        dataset_bindings,
    ) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            app.state.workbench.report_authoring.transaction_error = Some(error.to_string());
            return;
        }
    };
    let reference = if app.state.workbench.report_authoring.insert_result_frozen {
        let payload = match serde_json::to_vec(&source) {
            Ok(payload) => payload,
            Err(error) => {
                app.state.workbench.report_authoring.transaction_error =
                    Some(format!("The result document could not be frozen: {error}"));
                return;
            }
        };
        let artifact =
            match FrozenReportArtifact::new("application/vnd.rspice.visualization+json", payload) {
                Ok(artifact) => artifact,
                Err(error) => {
                    app.state.workbench.report_authoring.transaction_error =
                        Some(error.to_string());
                    return;
                }
            };
        ReportReferenceMode::Frozen { snapshot, artifact }
    } else {
        ReportReferenceMode::Linked { snapshot }
    };
    let Some(page_id) =
        active_document(&app.state).and_then(|document| selected_page_id(&app.state, document))
    else {
        app.state.workbench.report_authoring.transaction_error =
            Some("Select a report page before inserting a result document.".to_owned());
        return;
    };
    let result = active_document_mut(&mut app.state).and_then(|document| {
        let page = document
            .page(page_id)
            .ok_or_else(|| "The selected report page no longer exists.".to_owned())?;
        document
            .transact_with_context(
                document.revision(),
                vec![ReportEdit::AddBlockToPage {
                    page_id,
                    expected_page_revision: page.revision(),
                    kind: ReportBlockKind::PlotFigure(PlotFigureBlock {
                        caption,
                        alternative_text,
                        sizing,
                        reference,
                    }),
                }],
                timestamp_unix_ms(),
                "rspice-local-session",
                "Insert immutable result document into report page",
            )
            .map_err(|error| error.to_string())
    });
    match result {
        Ok(receipt) => {
            app.state.workbench.report_authoring.selected_report_block =
                receipt.created.iter().find_map(|entity| match entity {
                    ReportEntityRef::Block(id) => Some(*id),
                    _ => None,
                });
            app.state
                .workbench
                .report_authoring
                .insert_result_document_open = false;
            app.state.workbench.report_authoring.preview_block_page = 0;
            app.state.workbench.report_authoring.transaction_error = None;
            app.state.workspace.report_documents_dirty = true;
        }
        Err(error) => app.state.workbench.report_authoring.transaction_error = Some(error),
    }
}

fn commit_page_properties(app: &mut RSpiceApp) {
    if !report_mutation_allowed(&app.state) {
        app.state.workbench.report_authoring.transaction_error =
            Some(report_mutation_block_reason(&app.state).to_owned());
        return;
    }
    let Some(page_id) = app.state.workbench.report_authoring.page_properties_page else {
        app.state.workbench.report_authoring.transaction_error =
            Some("The selected report page no longer exists.".to_owned());
        return;
    };
    let title = app
        .state
        .workbench
        .report_authoring
        .page_title_draft
        .trim()
        .to_owned();
    let inline_title = title.clone();
    let timestamp = timestamp_unix_ms();
    let template =
        report_template_from_index(app.state.workbench.report_authoring.report_template_draft);
    let update_policy = page_update_policy_from_index(
        app.state
            .workbench
            .report_authoring
            .page_update_policy_draft,
    );
    let result = active_document_mut(&mut app.state).and_then(|document| {
        let page = document
            .page(page_id)
            .ok_or_else(|| "The selected report page no longer exists.".to_owned())?;
        let mut edits = Vec::with_capacity(3);
        if document.template() != template {
            edits.push(ReportEdit::SetTemplate { template });
        }
        let mut expected_page_revision = page.revision();
        if page.title() != title {
            edits.push(ReportEdit::UpdatePageTitle {
                page_id,
                expected_page_revision,
                title,
            });
            expected_page_revision = expected_page_revision
                .next()
                .map_err(|error| error.to_string())?;
        }
        if page.update_policy() != update_policy {
            edits.push(ReportEdit::SetPageUpdatePolicy {
                page_id,
                expected_page_revision,
                update_policy,
            });
        }
        if edits.is_empty() {
            return Ok(false);
        }
        document
            .transact_with_context(
                document.revision(),
                edits,
                timestamp,
                "rspice-local-session",
                "Update report page properties",
            )
            .map(|_| true)
            .map_err(|error| error.to_string())
    });
    match result {
        Ok(changed) => {
            app.state.workbench.report_authoring.selected_page = Some(page_id);
            app.state.workbench.report_authoring.preview_block_page = 0;
            app.state
                .workbench
                .report_authoring
                .inline_page_settings_page = Some(page_id);
            app.state.workbench.report_authoring.inline_page_title_draft = inline_title;
            app.state.workbench.report_authoring.page_properties_open = false;
            app.state.workbench.report_authoring.transaction_error = None;
            app.state.workspace.report_documents_dirty |= changed;
        }
        Err(error) => app.state.workbench.report_authoring.transaction_error = Some(error),
    }
}

fn synchronize_report_selection(state: &mut AppState) {
    let selected_is_valid = state
        .workbench
        .report_authoring
        .selected_document
        .is_some_and(|id| {
            state
                .workspace
                .report_documents
                .iter()
                .any(|doc| doc.id() == id)
        });
    if !selected_is_valid {
        state.workbench.report_authoring.selected_document = state
            .workspace
            .report_documents
            .first()
            .map(ReportDocument::id);
        state.workbench.report_authoring.preview_block_page = 0;
    }
    let current_page = state.workbench.report_authoring.selected_page;
    let current_block = state.workbench.report_authoring.selected_report_block;
    let selection = active_document(state).map(|document| {
        let selected_page = current_page
            .filter(|page_id| document.page(*page_id).is_some())
            .or_else(|| document.pages().first().map(|page| page.id()));
        let selected_block_is_on_page = current_block.is_some_and(|block_id| {
            selected_page
                .and_then(|page_id| document.page(page_id))
                .is_some_and(|page| {
                    page.sections()
                        .iter()
                        .flat_map(|section| section.blocks())
                        .any(|block| block.id() == block_id)
                })
        });
        (selected_page, selected_block_is_on_page)
    });
    if let Some((selected_page, selected_block_is_on_page)) = selection {
        if current_page != selected_page {
            state.workbench.report_authoring.selected_page = selected_page;
            state.workbench.report_authoring.preview_block_page = 0;
        }
        if !selected_block_is_on_page {
            state.workbench.report_authoring.selected_report_block = None;
        }
    } else {
        state.workbench.report_authoring.selected_page = None;
        state.workbench.report_authoring.selected_report_block = None;
        state.workbench.report_authoring.preview_block_page = 0;
        state.workbench.report_authoring.inline_page_settings_page = None;
        state
            .workbench
            .report_authoring
            .inline_page_title_draft
            .clear();
    }
}

fn active_document(state: &AppState) -> Option<&ReportDocument> {
    let id = state.workbench.report_authoring.selected_document?;
    state
        .workspace
        .report_documents
        .iter()
        .find(|document| document.id() == id)
}

fn active_document_mut(state: &mut AppState) -> Result<&mut ReportDocument, String> {
    let id = state
        .workbench
        .report_authoring
        .selected_document
        .ok_or_else(|| "No report document is selected.".to_owned())?;
    state
        .workspace
        .report_documents
        .iter_mut()
        .find(|document| document.id() == id)
        .ok_or_else(|| "The selected report document no longer exists.".to_owned())
}

fn selected_page_id(state: &AppState, document: &ReportDocument) -> Option<ReportPageId> {
    state
        .workbench
        .report_authoring
        .selected_page
        .filter(|id| document.page(*id).is_some())
        .or_else(|| document.pages().first().map(|page| page.id()))
}

fn valid_page_title(title: &str) -> bool {
    let trimmed = title.trim();
    !trimmed.is_empty()
        && trimmed == title
        && trimmed.len() <= 512
        && !trimmed.chars().any(char::is_control)
}

fn valid_document_title(title: &str) -> bool {
    valid_page_title(title)
}

fn report_mutation_allowed(state: &AppState) -> bool {
    state.project_lifecycle.project_open
        && !state.workbench.safe_mode.project_read_only()
        && !crate::workbench::lifecycle::project_lifecycle::operation_in_progress(state)
}

fn report_mutation_block_reason(state: &AppState) -> &'static str {
    if !state.project_lifecycle.project_open {
        "Open a project before changing its report document."
    } else if state.workbench.safe_mode.project_read_only() {
        "Report changes are unavailable because the active project is read-only."
    } else if crate::workbench::lifecycle::project_lifecycle::operation_in_progress(state) {
        "Wait for the current project operation to finish before changing the report."
    } else {
        "Report changes are unavailable in the current application state."
    }
}

fn report_template_index(template: ReportTemplate) -> usize {
    match template {
        ReportTemplate::ReleaseVerification42 => 0,
        ReportTemplate::DesignReview => 1,
        ReportTemplate::ModelQualification => 2,
    }
}

fn report_template_from_index(index: usize) -> ReportTemplate {
    match index {
        1 => ReportTemplate::DesignReview,
        2 => ReportTemplate::ModelQualification,
        _ => ReportTemplate::ReleaseVerification42,
    }
}

fn report_template_label(template: ReportTemplate) -> &'static str {
    match template {
        ReportTemplate::ReleaseVerification42 => "Release verification 4.2",
        ReportTemplate::DesignReview => "Design review",
        ReportTemplate::ModelQualification => "Model qualification",
    }
}

fn page_update_policy_index(policy: ReportPageUpdatePolicy) -> usize {
    match policy {
        ReportPageUpdatePolicy::RefreshLinkedAutomatically => 0,
        ReportPageUpdatePolicy::FreezeSelectedRevision => 1,
    }
}

fn page_update_policy_from_index(index: usize) -> ReportPageUpdatePolicy {
    match index {
        1 => ReportPageUpdatePolicy::FreezeSelectedRevision,
        _ => ReportPageUpdatePolicy::RefreshLinkedAutomatically,
    }
}

fn page_update_policy_label(policy: ReportPageUpdatePolicy) -> &'static str {
    match policy {
        ReportPageUpdatePolicy::RefreshLinkedAutomatically => "Refresh linked automatically",
        ReportPageUpdatePolicy::FreezeSelectedRevision => "Freeze selected revision",
    }
}

fn report_page_inclusion_label(inclusion: ReportPageInclusion) -> &'static str {
    match inclusion {
        ReportPageInclusion::Included => "Included",
        ReportPageInclusion::ExcludedFromDraft => "Excluded from draft",
        ReportPageInclusion::AppendixOnly => "Appendix only",
    }
}

fn report_blocked_gate_text_policy_label(policy: ReportBlockedGateTextPolicy) -> &'static str {
    match policy {
        ReportBlockedGateTextPolicy::VerbatimFromSource => "State verbatim from source",
        ReportBlockedGateTextPolicy::SummarizeWithLink => "Summarize with link",
    }
}

fn page_marker(_index: usize, title: &str) -> &str {
    INITIAL_PAGES
        .iter()
        .find(|(_, expected)| *expected == title)
        .map_or("+", |(marker, _)| *marker)
}

fn timestamp_unix_ms() -> u64 {
    u64::try_from(crate::time_compat::unix_epoch().as_millis()).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn responsive_report_builder_matches_mockup_breakpoints() {
        assert_eq!(OUTLINE_DESKTOP_WIDTH, 250.0);
        assert_eq!(OUTLINE_TABLET_WIDTH, 180.0);
        assert_eq!(INSPECTOR_WIDTH, 300.0);
        assert_eq!(
            ComposerLayout::resolve(1_280.0),
            ComposerLayout::ThreeColumn
        );
        assert_eq!(
            ComposerLayout::resolve(1_020.0),
            ComposerLayout::TwoColumnInspectorBelow
        );
        assert_eq!(
            ComposerLayout::resolve(821.0),
            ComposerLayout::TwoColumnInspectorBelow
        );
        assert_eq!(ComposerLayout::resolve(820.0), ComposerLayout::Stacked);
        assert_eq!(ComposerLayout::resolve(390.0), ComposerLayout::Stacked);
    }

    #[test]
    fn every_report_layout_assigns_each_internal_seam_to_one_pane() {
        assert_eq!(
            ComposerLayout::ThreeColumn.separators(),
            [
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
            ]
        );
        assert_eq!(
            ComposerLayout::TwoColumnInspectorBelow.separators(),
            [
                PaneSeparators {
                    right: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
                PaneSeparators {
                    top: true,
                    ..PaneSeparators::default()
                },
            ]
        );
        assert_eq!(
            ComposerLayout::Stacked.separators(),
            [
                PaneSeparators {
                    bottom: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators {
                    bottom: true,
                    ..PaneSeparators::default()
                },
                PaneSeparators::default(),
            ]
        );
    }

    #[test]
    fn tablet_and_stacked_pane_heights_follow_local_space_and_document_content() {
        let tablet_short = composer_pane_heights(
            ComposerLayout::TwoColumnInspectorBelow,
            640.0,
            INITIAL_PAGES.len(),
            true,
        );
        let tablet_tall = composer_pane_heights(
            ComposerLayout::TwoColumnInspectorBelow,
            1_000.0,
            INITIAL_PAGES.len(),
            true,
        );
        assert!(tablet_tall.preview > tablet_short.preview);
        assert!(tablet_tall.inspector > tablet_short.inspector);
        assert!(tablet_short.preview + tablet_short.inspector + 0.01 >= 640.0);
        assert!(tablet_tall.preview + tablet_tall.inspector + 0.01 >= 1_000.0);

        let compact_seven =
            composer_pane_heights(ComposerLayout::Stacked, 720.0, INITIAL_PAGES.len(), true);
        let compact_twelve = composer_pane_heights(ComposerLayout::Stacked, 720.0, 12, true);
        assert!(compact_twelve.outline > compact_seven.outline);
        assert_eq!(compact_seven.preview, PREVIEW_MIN_HEIGHT);
        assert!(compact_seven.inspector > 300.0);
    }

    #[test]
    fn incomplete_publication_contract_keeps_surface_and_commands_unexposed() {
        assert!(!crate::workbench::surface_availability(SurfaceId::ReportAuthoring).can_open());
        for command in [
            Command::ReportAuthoring,
            Command::SaveReportDocument,
            Command::AddReportPage,
            Command::ReportPageProperties,
        ] {
            assert!(!crate::workbench::commands::vocabulary::COMMAND_REGISTRY.contains(&command));
        }
    }

    #[test]
    fn opening_report_authoring_never_mutates_an_empty_project() {
        let mut state = AppState::default();
        assert!(state.workspace.report_documents.is_empty());
        assert!(!state.workspace.report_documents_dirty);

        synchronize_report_selection(&mut state);

        assert!(state.workspace.report_documents.is_empty());
        assert!(!state.workspace.report_documents_dirty);
        assert_eq!(state.workbench.report_authoring.selected_document, None);
        assert_eq!(state.workbench.report_authoring.selected_page, None);
    }

    #[test]
    fn explicit_report_plan_creates_the_exact_mockup_outline_once() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.report_authoring.preview_block_page = 7;
        app.state.workbench.report_authoring.create_document_title =
            "Verification report".to_owned();
        app.state
            .workbench
            .report_authoring
            .create_document_template =
            report_template_index(ReportTemplate::ReleaseVerification42);

        commit_create_document(&mut app);

        let document = active_document(&app.state).expect("active report");
        assert_eq!(document.pages().len(), INITIAL_PAGES.len());
        for (page, (_, expected)) in document.pages().iter().zip(INITIAL_PAGES) {
            assert_eq!(page.title(), expected);
        }
        assert!(app.state.workspace.report_documents_dirty);
        assert_eq!(app.state.workbench.report_authoring.preview_block_page, 0);
        let document_id = app.state.workbench.report_authoring.selected_document;
        let page_id = app.state.workbench.report_authoring.selected_page;
        synchronize_report_selection(&mut app.state);
        synchronize_report_selection(&mut app.state);
        assert_eq!(app.state.workspace.report_documents.len(), 1);
        assert_eq!(
            app.state.workbench.report_authoring.selected_document,
            document_id
        );
        assert_eq!(app.state.workbench.report_authoring.selected_page, page_id);
    }

    #[test]
    fn report_plan_binds_initial_pages_to_the_active_immutable_dataset() {
        let mut app = RSpiceApp::test_instance();
        let mut run = crate::state::SimulationRun::new(41);
        run.add_analysis(crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::Transient,
            "retained transient",
        ));
        let expected_binding =
            crate::product::DatasetBinding::new(run.dataset_id, run.dataset_content_digest());
        app.state.simulation.runs = vec![run];
        app.state.simulation.active_run_idx = Some(0);
        app.state.workbench.report_authoring.create_document_title =
            "Verification report".to_owned();

        commit_create_document(&mut app);

        let document = active_document(&app.state).expect("active report");
        assert!(document.pages().iter().all(|page| {
            page.evidence_binding()
                == ReportPageEvidenceBinding::ExactDataset {
                    binding: expected_binding,
                }
        }));
        assert_eq!(
            document
                .revision_history()
                .records()
                .last()
                .expect("binding revision")
                .revision_note(),
            "Bind initial report pages to active result dataset"
        );
    }

    #[test]
    fn report_page_settings_commit_canonical_revision_checked_transactions() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.report_authoring.create_document_title =
            "Verification report".to_owned();
        commit_create_document(&mut app);
        let page_id = app
            .state
            .workbench
            .report_authoring
            .selected_page
            .expect("selected page");
        let initial_revision = active_document(&app.state)
            .expect("active report")
            .revision();
        app.state.workspace.report_documents_dirty = false;

        commit_page_setting(
            &mut app,
            page_id,
            PageSettingEdit::Title("Decision and release summary".to_owned()),
        );
        commit_page_setting(
            &mut app,
            page_id,
            PageSettingEdit::Inclusion(ReportPageInclusion::AppendixOnly),
        );
        commit_page_setting(
            &mut app,
            page_id,
            PageSettingEdit::EvidenceBinding(ReportPageEvidenceBinding::LatestAcceptedRun),
        );
        commit_page_setting(
            &mut app,
            page_id,
            PageSettingEdit::BlockedGateText(ReportBlockedGateTextPolicy::SummarizeWithLink),
        );

        let document = active_document(&app.state).expect("active report");
        let page = document.page(page_id).expect("selected page");
        assert_eq!(page.title(), "Decision and release summary");
        assert_eq!(page.inclusion(), ReportPageInclusion::AppendixOnly);
        assert_eq!(
            page.evidence_binding(),
            ReportPageEvidenceBinding::LatestAcceptedRun
        );
        assert_eq!(
            page.blocked_gate_text_policy(),
            ReportBlockedGateTextPolicy::SummarizeWithLink
        );
        assert_eq!(document.revision().get(), initial_revision.get() + 4);
        assert!(app.state.workspace.report_documents_dirty);
        assert!(
            app.state
                .workbench
                .report_authoring
                .transaction_error
                .is_none()
        );
    }

    #[test]
    fn report_page_order_controls_commit_revision_checked_moves() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.report_authoring.create_document_title =
            "Verification report".to_owned();
        commit_create_document(&mut app);
        let page_ids = active_document(&app.state)
            .expect("active report")
            .pages()
            .iter()
            .map(|page| page.id())
            .collect::<Vec<_>>();
        let page_to_move = page_ids[1];
        app.state.workbench.report_authoring.selected_page = Some(page_to_move);
        app.state.workspace.report_documents_dirty = false;

        assert!(can_move_selected_page(
            &app.state,
            PageMoveDirection::Earlier
        ));
        assert!(can_move_selected_page(&app.state, PageMoveDirection::Later));
        move_selected_page(&mut app, PageMoveDirection::Earlier);

        let document = active_document(&app.state).expect("active report");
        assert_eq!(document.pages()[0].id(), page_to_move);
        assert_eq!(document.pages()[1].id(), page_ids[0]);
        assert_eq!(
            document
                .revision_history()
                .records()
                .last()
                .expect("move revision")
                .revision_note(),
            "Move report page earlier"
        );
        assert!(app.state.workspace.report_documents_dirty);
        assert_eq!(
            app.state.workbench.report_authoring.selected_page,
            Some(page_to_move)
        );

        app.state.workspace.report_documents_dirty = false;
        move_selected_page(&mut app, PageMoveDirection::Later);
        let document = active_document(&app.state).expect("active report");
        assert_eq!(
            document
                .pages()
                .iter()
                .map(|page| page.id())
                .collect::<Vec<_>>(),
            page_ids
        );
        assert_eq!(
            document
                .revision_history()
                .records()
                .last()
                .expect("move revision")
                .revision_note(),
            "Move report page later"
        );
        assert!(app.state.workspace.report_documents_dirty);
    }

    #[test]
    fn report_page_order_controls_fail_closed_at_document_boundaries() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.report_authoring.create_document_title =
            "Verification report".to_owned();
        commit_create_document(&mut app);
        let first_page = active_document(&app.state).expect("active report").pages()[0].id();
        app.state.workbench.report_authoring.selected_page = Some(first_page);
        app.state.workspace.report_documents_dirty = false;
        let revision = active_document(&app.state)
            .expect("active report")
            .revision();

        assert!(!can_move_selected_page(
            &app.state,
            PageMoveDirection::Earlier
        ));
        move_selected_page(&mut app, PageMoveDirection::Earlier);

        assert_eq!(
            active_document(&app.state)
                .expect("active report")
                .revision(),
            revision
        );
        assert!(!app.state.workspace.report_documents_dirty);
    }

    #[test]
    fn report_element_catalog_commits_every_non_plot_block_kind() {
        let mut app = RSpiceApp::test_instance();
        let mut run = crate::state::SimulationRun::new(83);
        run.add_analysis(crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::Transient,
            "retained transient",
        ));
        app.state.simulation.runs = vec![run];
        app.state.simulation.active_run_idx = Some(0);
        app.state.workbench.report_authoring.create_document_title =
            "Verification report".to_owned();
        commit_create_document(&mut app);

        for kind_index in 0..=6 {
            reset_add_report_element_kind(&mut app, kind_index);
            assert!(valid_add_report_element_draft(&app.state, true));
            commit_add_report_element(&mut app);
            assert!(
                app.state
                    .workbench
                    .report_authoring
                    .transaction_error
                    .is_none()
            );
        }

        let document = active_document(&app.state).expect("active report");
        let page_id = app
            .state
            .workbench
            .report_authoring
            .selected_page
            .expect("selected page");
        let page = document.page(page_id).expect("selected page");
        assert_eq!(page.sections().len(), 1);
        let blocks = page.sections()[0].blocks();
        assert_eq!(blocks.len(), 7);
        assert!(matches!(blocks[0].kind(), ReportBlockKind::Prose(_)));
        assert!(matches!(blocks[1].kind(), ReportBlockKind::DataTable(_)));
        assert!(matches!(blocks[2].kind(), ReportBlockKind::Datasheet(_)));
        assert!(matches!(blocks[3].kind(), ReportBlockKind::Requirements(_)));
        assert!(matches!(
            blocks[4].kind(),
            ReportBlockKind::Specifications(_)
        ));
        assert!(matches!(blocks[5].kind(), ReportBlockKind::ReviewNote(_)));
        assert!(matches!(blocks[6].kind(), ReportBlockKind::Evidence(_)));
        assert!(blocks.iter().all(|block| block.enabled()));
    }

    #[test]
    fn page_element_toggle_and_remove_are_canonical_transactions() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.report_authoring.create_document_title =
            "Verification report".to_owned();
        commit_create_document(&mut app);
        reset_add_report_element_kind(&mut app, 0);
        commit_add_report_element(&mut app);
        let document_id = active_document(&app.state).expect("active report").id();
        let block_id = app
            .state
            .workbench
            .report_authoring
            .selected_report_block
            .expect("new block selected");

        set_report_block_enabled(&mut app, document_id, block_id, false);
        assert!(
            !active_document(&app.state)
                .expect("active report")
                .block(block_id)
                .expect("block")
                .enabled()
        );
        app.state
            .workbench
            .report_authoring
            .remove_report_block_open = true;
        commit_remove_report_block(&mut app);
        let document = active_document(&app.state).expect("active report");
        assert!(document.block(block_id).is_none());
        assert!(
            document
                .tombstones()
                .iter()
                .any(|tombstone| tombstone.entity == ReportEntityRef::Block(block_id))
        );
        assert_eq!(
            app.state.workbench.report_authoring.selected_report_block,
            None
        );
    }

    #[test]
    fn inserting_result_document_binds_exact_revision_digest_and_dataset() {
        use crate::results::visualization_document::{
            ColumnRole, SourceColumn, SourceDataset, SourceRow, TypedValue, ValueType,
            VisualizationDocument,
        };

        let mut app = RSpiceApp::test_instance();
        app.state.workbench.report_authoring.create_document_title =
            "Verification report".to_owned();
        commit_create_document(&mut app);
        let binding = crate::product::DatasetBinding::new(
            crate::product::DatasetId::new(),
            crate::product::ContentDigest::from_bytes([0x83; 32]),
        );
        let dataset = SourceDataset::new(
            binding,
            vec![
                SourceColumn::new(
                    "time",
                    "Time",
                    ValueType::Real,
                    ColumnRole::Coordinate,
                    Some("s".to_owned()),
                )
                .unwrap(),
                SourceColumn::new(
                    "vout",
                    "V(out)",
                    ValueType::Real,
                    ColumnRole::Signal,
                    Some("V".to_owned()),
                )
                .unwrap(),
            ],
            vec![SourceRow::new(vec![
                TypedValue::Real(0.0),
                TypedValue::Real(1.0),
            ])],
        )
        .unwrap();
        let source = VisualizationDocument::new("Nominal response", vec![dataset]).unwrap();
        let source_id = source.id();
        let source_revision = source.revision();
        let source_digest = source.content_digest().unwrap();
        app.state.workspace.visualization_documents.push(source);

        open_insert_result_document(&mut app);
        commit_insert_result_document(&mut app);

        let block_id = app
            .state
            .workbench
            .report_authoring
            .selected_report_block
            .expect("inserted plot selected");
        let block = active_document(&app.state)
            .expect("active report")
            .block(block_id)
            .expect("inserted plot");
        let ReportBlockKind::PlotFigure(figure) = block.kind() else {
            panic!("inserted block must be a plot figure");
        };
        let snapshot = figure.reference.snapshot();
        assert_eq!(
            snapshot.source,
            ReportSourceId::VisualizationDocument {
                document_id: source_id
            }
        );
        assert_eq!(snapshot.source_revision, Some(source_revision));
        assert_eq!(snapshot.content_digest, source_digest);
        assert_eq!(snapshot.dataset_bindings, vec![binding]);
        assert!(report_reference_resolves(&app.state, &figure.reference));
    }

    #[test]
    fn report_page_markers_remain_semantically_stable_after_reordering() {
        assert_eq!(page_marker(0, "Executive summary"), "1");
        assert_eq!(page_marker(5, "Executive summary"), "1");
        assert_eq!(page_marker(0, "Run manifests"), "A");
        assert_eq!(page_marker(7, "Custom appendix"), "+");
    }

    #[test]
    fn direct_report_creation_fails_closed_in_read_only_safe_mode() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.safe_mode.activate(
            crate::workbench::state::LocalSafeModeOptions {
                open_project_read_only: true,
                ..crate::workbench::state::LocalSafeModeOptions::default()
            },
            "report authoring test".to_owned(),
        );
        app.state.workbench.report_authoring.create_document_title =
            "Verification report".to_owned();

        commit_create_document(&mut app);

        assert!(app.state.workspace.report_documents.is_empty());
        assert!(!app.state.workspace.report_documents_dirty);
        assert_eq!(
            app.state
                .workbench
                .report_authoring
                .transaction_error
                .as_deref(),
            Some("Report changes are unavailable because the active project is read-only.")
        );
    }

    #[test]
    fn report_page_title_validation_matches_domain_limits() {
        assert!(valid_page_title("PVT and yield"));
        assert!(!valid_page_title(""));
        assert!(!valid_page_title(" leading"));
        assert!(!valid_page_title("trailing "));
        assert!(!valid_page_title("bad\nlabel"));
        assert!(!valid_page_title(&"x".repeat(513)));
    }

    #[test]
    fn changing_active_run_does_not_mutate_the_project_report_document() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.report_authoring.create_document_title =
            "Verification report".to_owned();
        commit_create_document(&mut app);
        app.state.simulation.runs = vec![
            crate::state::SimulationRun::new(2),
            crate::state::SimulationRun::new(1),
        ];
        let document = active_document(&app.state)
            .expect("report document")
            .clone();
        app.state.simulation.active_run_idx = Some(0);
        app.state.simulation.active_run_idx = Some(1);
        assert_eq!(active_document(&app.state), Some(&document));
    }

    #[test]
    fn prose_preview_is_unicode_safe_and_bounded() {
        const MAXIMUM_CHARACTERS: usize = 4_096;
        let exact = "a".repeat(MAXIMUM_CHARACTERS);
        let (preview, truncated) = bounded_text_preview(&exact, MAXIMUM_CHARACTERS);
        assert!(!truncated);
        assert_eq!(preview, exact);

        let oversized = format!("{}é-tail", exact);
        let (preview, truncated) = bounded_text_preview(&oversized, MAXIMUM_CHARACTERS);
        assert!(truncated);
        assert_eq!(preview.chars().count(), MAXIMUM_CHARACTERS + 1);
        assert!(preview.ends_with('…'));
    }

    #[test]
    fn invalid_report_page_selection_resets_preview_pagination() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.report_authoring.create_document_title =
            "Verification report".to_owned();
        commit_create_document(&mut app);
        app.state.workbench.report_authoring.selected_page =
            Some(crate::results::report_document::ReportPageId::new());
        app.state.workbench.report_authoring.preview_block_page = 4;

        synchronize_report_selection(&mut app.state);

        assert_eq!(app.state.workbench.report_authoring.preview_block_page, 0);
        assert_eq!(
            app.state.workbench.report_authoring.selected_page,
            active_document(&app.state)
                .and_then(|document| document.pages().first())
                .map(|page| page.id())
        );
    }
}
