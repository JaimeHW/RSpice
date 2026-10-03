//! Project-owned engineering report authoring.
//!
//! The surface authors and saves the project-owned, versioned
//! [`ReportDocument`] graph. Route availability remains fail-closed until the
//! complete report workflow is ready for production use.

mod presentation;
mod result_insert;
use rspice_results_ui::report::composer::{PageMoveDirection, PageSettingEdit};
use rspice_results_ui::report::inspector::DocumentPublicationEdit;
use rspice_results_ui::report::session::{
    page_update_policy_from_index, page_update_policy_index, report_template_from_index,
    report_template_index, valid_title,
};
use rspice_results_ui::report::{self, INITIAL_PAGES, report_template_label};

use egui::Ui;

use crate::results::report_document::{
    DataTableBlock, DatasheetBlock, DatasheetField, EvidenceBlock, ProseBlock, ProseStyle,
    ReportBlockId, ReportBlockKind, ReportDocument, ReportEdit, ReportEntityRef,
    ReportPageEvidenceBinding, ReportPageId, ReportReferenceMode, ReportReferenceSnapshot,
    ReportSourceId, ReportTemplate, RequirementDisposition, RequirementEntry, RequirementsBlock,
    ReviewNoteBlock, ReviewNoteStatus, SpecificationDisposition, SpecificationEntry,
    SpecificationsBlock, TableCell, TableColumn,
};
use crate::workbench::{AppState, RSpiceApp};

use super::super::commands::vocabulary::Command;
use super::super::{RouteTransitionSource, SurfaceId, SurfaceRoute};

#[cfg(test)]
use crate::results::report_document::ReportFigureSourceLocator;
use result_insert::{
    commit_insert_result_document, open_insert_result_document, report_figure_options,
};

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
        && !valid_title(title)
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
    report::composer::show(ui, &mut presentation::ReportHost(app));
    report::dialogs::show(ui.ctx(), &mut presentation::ReportHost(app));
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
    if !valid_title(&title) {
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
            editor.create_document_open = false;
            editor.transaction_error = None;
        }
        Err(error) => app.state.workbench.report_authoring.transaction_error = Some(error),
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
            app.state.workbench.report_authoring.add_page_open = false;
            app.state.workbench.report_authoring.transaction_error = None;
            app.state.workspace.content.report_documents_dirty = true;
        }
        Err(error) => app.state.workbench.report_authoring.transaction_error = Some(error),
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
        }
        if !selected_block_is_on_page {
            state.workbench.report_authoring.selected_report_block = None;
        }
    } else {
        state.workbench.report_authoring.selected_page = None;
        state.workbench.report_authoring.selected_report_block = None;
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

fn timestamp_unix_ms() -> u64 {
    u64::try_from(crate::time_compat::unix_epoch().as_millis()).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests;
