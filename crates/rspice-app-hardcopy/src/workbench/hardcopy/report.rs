//! The report editor's commands, and what both they and the report route's
//! surface read: the open report document, its selected page, and whether the
//! project lets a report change now.

use rspice_results_ui::report::session::{
    ReportAuthoringState, page_update_policy_index, report_template_index,
};

use crate::results::report_document::{ReportDocument, ReportPageId};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::state::WorkbenchSession;
use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};

/// Show the report route, or say on the status bar why it cannot be shown.
pub(crate) fn open(wb: &mut WorkbenchSession) {
    let route = SurfaceRoute::surface(SurfaceId::ReportAuthoring);
    if let Err(error) = wb.navigate(route, RouteTransitionSource::User) {
        wb.prompt(error.to_string());
    }
}

/// What a report toolbar action acts on beyond an open report it may change.
#[derive(Clone, Copy)]
pub(crate) enum ReportNeed {
    Report,
    UnsavedChanges,
    SelectedPage,
}

/// Why a report toolbar action cannot run now, or `None` when it can.
pub(crate) fn action_block(
    editor: &ReportAuthoringState,
    model: &Model,
    wb: &WorkbenchSession,
    platform: &PlatformState,
    need: ReportNeed,
) -> Option<&'static str> {
    if wb.working_route().surface_id() != SurfaceId::ReportAuthoring {
        return Some("the report editor is not open");
    }
    if !platform.project_lifecycle.is_open() {
        return Some("no project is open");
    }
    if model.safe_mode.project_read_only() {
        return Some("the project is read-only");
    }
    if crate::workbench::lifecycle::project_lifecycle::operation_in_progress(platform) {
        return Some("a project operation is in progress");
    }
    let Some(document) = active_document(editor, model) else {
        return Some("no report is open");
    };
    match need {
        ReportNeed::Report => None,
        ReportNeed::UnsavedChanges => (!model.workspace.content.report_documents_dirty)
            .then_some("the report has no unsaved changes"),
        ReportNeed::SelectedPage => selected_page_id(editor, document)
            .is_none()
            .then_some("no report page is selected"),
    }
}

/// Whether the report documents may be saved with the project now. When
/// they may not, the status bar says why and nothing is saved.
pub(crate) fn ready_to_save(
    model: &Model,
    wb: &mut WorkbenchSession,
    platform: &PlatformState,
) -> bool {
    if !mutation_allowed(model, platform) {
        wb.prompt(mutation_block_reason(model, platform));
        return false;
    }
    let invalid = model
        .workspace
        .content
        .report_documents
        .iter()
        .find_map(|document| document.validate().err());
    if let Some(error) = invalid {
        wb.prompt(format!("The report was not saved: {error}"));
        return false;
    }
    true
}

pub(crate) fn open_add_page(
    editor: &mut ReportAuthoringState,
    model: &Model,
    platform: &PlatformState,
) {
    if active_document(editor, model).is_none() || !mutation_allowed(model, platform) {
        return;
    }
    let next = active_document(editor, model)
        .map_or(1, |document| document.pages().len().saturating_add(1));
    editor.add_page_title = format!("Report page {next}");
    editor.transaction_error = None;
    editor.add_page_open = true;
}

pub(crate) fn open_page_properties(
    editor: &mut ReportAuthoringState,
    model: &Model,
    platform: &PlatformState,
) {
    if !mutation_allowed(model, platform) {
        return;
    }
    let Some(document) = active_document(editor, model) else {
        return;
    };
    let Some(page) = selected_page_id(editor, document).and_then(|id| document.page(id)) else {
        return;
    };
    let page_id = page.id();
    let title = page.title().to_owned();
    let template = report_template_index(document.template());
    let update_policy = page_update_policy_index(page.update_policy());
    editor.page_properties_page = Some(page_id);
    editor.page_title_draft = title;
    editor.report_template_draft = template;
    editor.page_update_policy_draft = update_policy;
    editor.transaction_error = None;
    editor.page_properties_open = true;
}

/// The report document the editor has open.
pub fn active_document<'m>(
    editor: &ReportAuthoringState,
    model: &'m Model,
) -> Option<&'m ReportDocument> {
    let documents = &model.workspace.content.report_documents;
    let id = editor.selected_document?;
    documents.iter().find(|document| document.id() == id)
}

/// The page of `document` the editor has selected, or its first page.
pub(crate) fn selected_page_id(
    editor: &ReportAuthoringState,
    document: &ReportDocument,
) -> Option<ReportPageId> {
    editor
        .selected_page
        .filter(|id| document.page(*id).is_some())
        .or_else(|| document.pages().first().map(|page| page.id()))
}

/// Whether the project lets a report change now: one is open, it is not
/// read-only, and no operation on it is in progress.
pub(crate) fn mutation_allowed(model: &Model, platform: &PlatformState) -> bool {
    platform.project_lifecycle.is_open()
        && !model.safe_mode.project_read_only()
        && !crate::workbench::lifecycle::project_lifecycle::operation_in_progress(platform)
}

/// Why the project does not let a report change now.
pub(crate) fn mutation_block_reason(model: &Model, platform: &PlatformState) -> &'static str {
    if !platform.project_lifecycle.is_open() {
        "Open a project before changing its report document."
    } else if model.safe_mode.project_read_only() {
        "Report changes are unavailable because the active project is read-only."
    } else if crate::workbench::lifecycle::project_lifecycle::operation_in_progress(platform) {
        "Wait for the current project operation to finish before changing the report."
    } else {
        "Report changes are unavailable in the current application state."
    }
}
