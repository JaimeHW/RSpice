//! Project-owned engineering report authoring.
//!
//! The surface authors and saves the project-owned, versioned
//! [`ReportDocument`] graph. Route availability remains fail-closed until the
//! complete report workflow is ready for production use.

mod presentation;
mod result_insert;
use rspice_results_ui::report::inspector::DocumentPublicationEdit;
use rspice_results_ui::report::preview::report_block_element_title;
use rspice_results_ui::report::{
    self, INITIAL_PAGES, PaneSeparators, page_marker, paint_pane_separators, report_template_label,
};

use egui::{Align, Align2, Layout, Rect, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::results::report_document::{
    DataTableBlock, DatasheetBlock, DatasheetField, EvidenceBlock, ProseBlock, ProseStyle,
    ReportBlockId, ReportBlockKind, ReportBlockedGateTextPolicy, ReportDocument, ReportEdit,
    ReportEntityRef, ReportPageEvidenceBinding, ReportPageId, ReportPageInclusion,
    ReportPageUpdatePolicy, ReportReferenceMode, ReportReferenceSnapshot, ReportSourceId,
    ReportTemplate, RequirementDisposition, RequirementEntry, RequirementsBlock, ReviewNoteBlock,
    ReviewNoteStatus, SpecificationDisposition, SpecificationEntry, SpecificationsBlock, TableCell,
    TableColumn,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogInitialFocus, input_row, select};
use crate::workbench::{AppState, RSpiceApp};

use super::super::commands::vocabulary::Command;
use super::super::design_system::{
    WorkbenchIcon, code_inspector_section, code_workspace_heading, icon_button, workspace_title_row,
};
use super::super::{RouteTransitionSource, SurfaceId, SurfaceRoute};

#[cfg(test)]
use crate::results::report_document::ReportFigureSourceLocator;
#[cfg(test)]
use result_insert::commit_insert_result_document;
use result_insert::{
    insert_result_document_dialog, open_insert_result_document, report_figure_options,
};

const DESKTOP_BREAKPOINT: f32 = 1_020.0;
const STACK_BREAKPOINT: f32 = 820.0;
const OUTLINE_DESKTOP_WIDTH: f32 = 250.0;
const OUTLINE_TABLET_WIDTH: f32 = 180.0;
const INSPECTOR_WIDTH: f32 = 300.0;
const PANEL_GAP: f32 = 0.0;
const TITLE_ACTION_STACK_BREAKPOINT: f32 = 560.0;
const OUTLINE_HEADER_HEIGHT: f32 = 39.0;
const OUTLINE_ROW_HEIGHT: f32 = 34.0;
const PREVIEW_MIN_HEIGHT: f32 = 420.0;
const INSPECTOR_PUBLICATION_CONTENT_HEIGHT: f32 = 820.0;
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
    _page_selected: bool,
) -> ComposerPaneHeights {
    let viewport_height = if available_height.is_finite() {
        available_height.max(1.0)
    } else {
        PREVIEW_MIN_HEIGHT
    };
    let outline_content = OUTLINE_HEADER_HEIGHT + OUTLINE_ROW_HEIGHT * page_count as f32;
    let inspector_content = INSPECTOR_PUBLICATION_CONTENT_HEIGHT;

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

pub(crate) fn can_open(state: &AppState) -> bool {
    state.project_lifecycle.is_open()
}

pub(crate) fn can_save_document(state: &AppState) -> bool {
    state.workbench.current_route().surface_id() == SurfaceId::ReportAuthoring
        && report_mutation_allowed(state)
        && active_document(state).is_some()
        && state.workspace.content.report_documents_dirty
}

pub(crate) fn can_add_page(state: &AppState) -> bool {
    state.workbench.current_route().surface_id() == SurfaceId::ReportAuthoring
        && report_mutation_allowed(state)
        && active_document(state).is_some()
}

pub(crate) fn can_edit_page_properties(state: &AppState) -> bool {
    if state.workbench.current_route().surface_id() != SurfaceId::ReportAuthoring
        || !report_mutation_allowed(state)
    {
        return false;
    }
    active_document(state).is_some_and(|document| selected_page_id(state, document).is_some())
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
        .content
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
            app.state.workspace.content.report_documents_dirty |= changed;
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
            app.state.workspace.content.report_documents_dirty |= changed;
        }
        Err(error) => {
            app.state.workbench.report_authoring.transaction_error = Some(error.clone());
            app.state
                .push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
        }
    }
}

fn commit_document_publication_setting(
    app: &mut RSpiceApp,
    document_id: crate::product::ResultDocumentId,
    setting: DocumentPublicationEdit,
) {
    if !report_mutation_allowed(&app.state) {
        let reason = report_mutation_block_reason(&app.state).to_owned();
        app.state.workbench.report_authoring.transaction_error = Some(reason.clone());
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::warning(reason));
        return;
    }
    let result = active_document_mut(&mut app.state).and_then(|document| {
        if document.id() != document_id {
            return Err(
                "The report document changed before its publication policy committed.".to_owned(),
            );
        }
        let (edit, revision_note) = match setting {
            DocumentPublicationEdit::OutputFormats(output_formats)
                if document.output_formats() != output_formats =>
            {
                (
                    ReportEdit::SetOutputFormats { output_formats },
                    "Update report output formats",
                )
            }
            DocumentPublicationEdit::PublicationProfile(publication_profile)
                if document.publication_profile() != publication_profile =>
            {
                (
                    ReportEdit::SetPublicationProfile {
                        publication_profile,
                    },
                    "Update report publication profile",
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
            app.state.workspace.content.report_documents_dirty |= changed;
            app.state.workbench.report_authoring.transaction_error = None;
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
            app.state.workspace.content.report_documents_dirty |= changed;
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

        if !app.state.project_lifecycle.is_open() {
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

        report_title_row(ui, app);

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
                        |ui| outline(ui, app, &document, selected_page, outline_separators),
                    );
                    let preview_width =
                        (available.x - OUTLINE_DESKTOP_WIDTH - INSPECTOR_WIDTH - PANEL_GAP * 2.0)
                            .max(1.0);
                    ui.allocate_ui_with_layout(
                        Vec2::new(preview_width, available.y),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            report::preview::show(
                                ui,
                                &mut presentation::ReportHost(app),
                                &document,
                                selected_page,
                                preview_separators,
                            )
                        },
                    );
                    ui.allocate_ui_with_layout(
                        Vec2::new(INSPECTOR_WIDTH, available.y),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            report::inspector::show(
                                ui,
                                &mut presentation::ReportHost(app),
                                &document,
                                inspector_separators,
                            )
                        },
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
                                |ui| outline(ui, app, &document, selected_page, outline_separators),
                            );
                            ui.allocate_ui_with_layout(
                                Vec2::new(
                                    (local_width - OUTLINE_TABLET_WIDTH).max(1.0),
                                    heights.preview,
                                ),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    report::preview::show(
                                        ui,
                                        &mut presentation::ReportHost(app),
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
                                report::inspector::show(
                                    ui,
                                    &mut presentation::ReportHost(app),
                                    &document,
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
                            |ui| outline(ui, app, &document, selected_page, outline_separators),
                        );
                        ui.allocate_ui_with_layout(
                            Vec2::new(local_width, heights.preview),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                report::preview::show(
                                    ui,
                                    &mut presentation::ReportHost(app),
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
                                report::inspector::show(
                                    ui,
                                    &mut presentation::ReportHost(app),
                                    &document,
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

fn report_title_row(ui: &mut Ui, app: &mut RSpiceApp) {
    workspace_title_row(ui, |ui| {
        if ui.available_width() <= TITLE_ACTION_STACK_BREAKPOINT {
            report_title_heading(ui);
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let width = ((ui.available_width() - 6.0) * 0.5).max(1.0);
                report_release_button(ui, app, width);
                report_plan_button(ui, app, width);
            });
        } else {
            let release_width = report_title_button_width(ui, "Release closure", false);
            let plan_width = report_title_button_width(ui, "Plan report artifact…", true);
            let actions_width = release_width + 6.0 + plan_width;
            let heading_width = (ui.available_width() - actions_width - 12.0).max(1.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                ui.allocate_ui_with_layout(
                    Vec2::new(heading_width, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        ui.set_width(heading_width);
                        report_title_heading(ui);
                    },
                );
                ui.add_space(6.0);
                report_release_button(ui, app, 0.0);
                report_plan_button(ui, app, 0.0);
            });
        }
    });
}

fn report_title_heading(ui: &mut Ui) {
    code_workspace_heading(
        ui,
        "REPORT AUTHORING · ENGINEERING DRAFT",
        "Engineering report composer",
        "Author traceable pages, locked run references, publication plots and machine-readable appendices. Release closure alone owns package assembly and promotion.",
    );
}

fn report_release_button(ui: &mut Ui, app: &mut RSpiceApp, width: f32) {
    let mut button = Button::new("Release closure");
    if width > 0.0 {
        button = button.min_width(width).max_width(width);
    }
    if button.show(ui).clicked() {
        open_release_cockpit(app);
    }
}

fn report_plan_button(ui: &mut Ui, app: &mut RSpiceApp, width: f32) {
    let writable = report_mutation_allowed(&app.state);
    let mut button = Button::new("Plan report artifact…")
        .accent()
        .enabled(writable);
    if width > 0.0 {
        button = button.min_width(width).max_width(width);
    }
    let response = button
        .show(ui)
        .on_disabled_hover_text(report_mutation_block_reason(&app.state));
    if response.clicked() {
        open_create_document(app);
    }
}

fn report_title_button_width(ui: &Ui, label: &str, accent: bool) -> f32 {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(
        tokens::FS_0,
        if accent {
            FontWeight::SemiBold
        } else {
            FontWeight::Regular
        },
    );
    let content = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font, t.color.text)
        .size()
        .x
        + 20.0;
    content.max(if t.metrics.ctl_h >= 44.0 { 44.0 } else { 0.0 })
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
    let page_marker = document
        .pages()
        .iter()
        .position(|candidate| candidate.id() == page_id)
        .map(|index| page_marker(index, page.title()))
        .unwrap_or("+");
    let section_title = format!("Page settings · {page_marker}");
    code_inspector_section(ui, &section_title, None, |ui| {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ReportJointYield {
    passing: usize,
    total: usize,
}

impl ReportJointYield {
    fn from_results(results: &[rspice_results::yield_analysis::YieldResult]) -> Option<Self> {
        let total = results.first()?.total_runs;
        if total == 0
            || results
                .iter()
                .any(|result| result.total_runs != total || result.trail.len() != total)
        {
            return None;
        }
        let passing = (0..total)
            .filter(|index| results.iter().all(|result| result.trail[*index]))
            .count();
        Some(Self { passing, total })
    }

    fn percent(self) -> f64 {
        self.passing as f64 / self.total as f64 * 100.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ReportSummaryMetrics {
    checks_passing: usize,
    checks_total: usize,
    joint_yield: Option<ReportJointYield>,
    pvt_completed: Option<usize>,
    pvt_total: Option<usize>,
}

impl ReportSummaryMetrics {
    fn from_state(state: &AppState) -> Self {
        let run = state.simulation.active_run();
        let checks_total = state.workspace.content.specs.len();
        let checks_passing = state
            .workspace
            .content
            .specs
            .iter()
            .filter(|spec| {
                run.and_then(|run| {
                    run.analyses
                        .iter()
                        .rev()
                        .filter(|analysis| analysis.success && analysis.provenance.is_some())
                        .flat_map(|analysis| analysis.measurements.iter())
                        .find(|measurement| {
                            measurement.name.eq_ignore_ascii_case(&spec.measurement)
                        })
                        .filter(|measurement| measurement.passed && measurement.error.is_none())
                        .and_then(|measurement| {
                            measurement.value_in_unit(&spec.unit).ok().flatten()
                        })
                        .filter(|value| value.is_finite())
                })
                .is_some_and(|value| spec.passes(value))
            })
            .count();

        let joint_yield = state
            .simulation
            .yield_results_for_active_dataset()
            .and_then(ReportJointYield::from_results);

        let pvt_progress = run
            .and_then(|run| {
                run.analyses.iter().rev().find(|analysis| {
                    analysis.analysis_type == crate::state::AnalysisType::Corner
                        && analysis.success
                        && analysis.provenance.is_some()
                })
            })
            .and_then(|analysis| match analysis.family_metadata.as_ref() {
                Some(crate::state::AnalysisResultFamilyMetadata::Corner {
                    x_values,
                    failed_corners,
                    ..
                }) if !x_values.is_empty() && *failed_corners <= x_values.len() => {
                    Some((x_values.len() - *failed_corners, x_values.len()))
                }
                _ => None,
            });

        Self {
            checks_passing,
            checks_total,
            joint_yield,
            pvt_completed: pvt_progress.map(|(completed, _)| completed),
            pvt_total: pvt_progress.map(|(_, total)| total),
        }
    }
}

fn report_reference_resolves(state: &AppState, reference: &ReportReferenceMode) -> bool {
    let snapshot = reference.snapshot();
    match &snapshot.source {
        ReportSourceId::VisualizationDocument { document_id } => state
            .workspace
            .content
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

fn report_bound_result_label(state: &AppState, document: &ReportDocument) -> (String, bool) {
    let mut binding = None;
    for page in document.pages() {
        let ReportPageEvidenceBinding::ExactDataset {
            binding: page_binding,
        } = page.evidence_binding()
        else {
            return ("Exact immutable result required".to_owned(), false);
        };
        match binding {
            None => binding = Some(page_binding),
            Some(existing) if existing == page_binding => {}
            Some(_) => return ("Multiple immutable result bindings".to_owned(), false),
        }
    }
    let Some(binding) = binding else {
        return ("No report pages are bound".to_owned(), false);
    };
    let label = state
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
        );
    (label, true)
}

fn open_release_cockpit(app: &mut RSpiceApp) {
    if let Err(error) = app.state.workbench.navigate(
        SurfaceRoute::surface(SurfaceId::ReleaseCockpit),
        RouteTransitionSource::User,
    ) {
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                "Cannot open release package assembly: {error}"
            )));
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
            app.state.workspace.content.report_documents.push(document);
            app.state.workspace.content.report_documents_dirty = true;
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
            app.state.workspace.content.report_documents_dirty = true;
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
        if matches!(kind_index, 3..=5) {
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
            app.state.workspace.content.report_documents_dirty = true;
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
            app.state.workspace.content.report_documents_dirty = true;
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
            app.state.workspace.content.report_documents_dirty |= changed;
        }
        Err(error) => app.state.workbench.report_authoring.transaction_error = Some(error),
    }
}

fn synchronize_report_selection(state: &mut AppState) {
    let documents = &state.workspace.content.report_documents;
    let selected_is_valid = state
        .workbench
        .report_authoring
        .selected_document
        .is_some_and(|id| documents.iter().any(|doc| doc.id() == id));
    if !selected_is_valid {
        state.workbench.report_authoring.selected_document =
            documents.first().map(ReportDocument::id);
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
    let documents = &state.workspace.content.report_documents;
    let id = state.workbench.report_authoring.selected_document?;
    documents.iter().find(|document| document.id() == id)
}

fn active_document_mut(state: &mut AppState) -> Result<&mut ReportDocument, String> {
    let documents = &mut state.workspace.content.report_documents;
    let id = state
        .workbench
        .report_authoring
        .selected_document
        .ok_or_else(|| "No report document is selected.".to_owned())?;
    documents
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
    state.project_lifecycle.is_open()
        && !state.workbench.safe_mode.project_read_only()
        && !crate::workbench::lifecycle::project_lifecycle::operation_in_progress(state)
}

fn report_mutation_block_reason(state: &AppState) -> &'static str {
    if !state.project_lifecycle.is_open() {
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

fn timestamp_unix_ms() -> u64 {
    u64::try_from(crate::time_compat::unix_epoch().as_millis()).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests;
