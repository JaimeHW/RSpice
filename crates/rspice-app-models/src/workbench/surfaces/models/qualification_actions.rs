//! What the qualification page offers on the families it lists: the rail
//! that groups them, and the workflows it opens on the selected one. Every
//! blocked action states the reason it is blocked rather than being hidden,
//! because "no button" and "not qualified" are different facts to an engineer
//! looking for why.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum QualificationPageAction {
    ReviewVectors,
    ReviewReleaseBinding,
    RunSuite,
    CompareRelease,
    OpenCorrelation,
}

/// The band the engine-owned families are listed under.
pub(super) const ENGINE_OWNED_BAND: &str =
    "Engine-owned · exempt from the source-owned release gate";

/// One row of the qualification rail: a model, or the band above a group.
pub(super) enum QualificationRailRow<'a> {
    Band(&'static str),
    Model(&'a QualificationModelSummary),
}

/// The rail's order: the gate's own subjects first, then what it exempts.
///
/// Interleaving the two — which is what sorting by library name alone does —
/// leaves a reader scanning one flat list to tell "exempt" from "not yet
/// qualified" by badge colour, on a page whose entire subject is which models
/// a release may be signed against. The band says it once, in words, and the
/// group under it inherits the statement.
pub(super) fn qualification_rail_rows(
    summaries: &[QualificationModelSummary],
) -> Vec<QualificationRailRow<'_>> {
    let (exempt, subject): (Vec<_>, Vec<_>) = summaries
        .iter()
        .partition(|summary| !summary.gate.is_gate_subject());
    let mut rows = subject
        .into_iter()
        .map(QualificationRailRow::Model)
        .collect::<Vec<_>>();
    if !exempt.is_empty() {
        rows.push(QualificationRailRow::Band(ENGINE_OWNED_BAND));
        rows.extend(exempt.into_iter().map(QualificationRailRow::Model));
    }
    rows
}

pub(super) fn selected_qualification_summary<'a>(
    cx: &Cx<'_, ModelsFeature>,
    summaries: &'a [QualificationModelSummary],
) -> Option<&'a QualificationModelSummary> {
    let library = cx.model.model_library_manager.selected_library.as_deref()?;
    let model = cx.state.selected_model.as_deref()?;
    summaries.iter().find(|summary| {
        summary.library.eq_ignore_ascii_case(library) && summary.model.eq_ignore_ascii_case(model)
    })
}

pub(super) fn qualification_action_block_reason(
    cx: &Cx<'_, ModelsFeature>,
    selected: Option<&QualificationModelSummary>,
    action: QualificationPageAction,
) -> Option<String> {
    let Some(selected) = selected else {
        return Some("Select a model family first".to_owned());
    };
    if !cx.platform.project_lifecycle.is_open() {
        return Some("Open a project before using model qualification".to_owned());
    }
    // Asked before the source-owned refusal below, which would otherwise
    // report an engine-owned card as a project-owned revision the user failed
    // to select. There is nothing here for them to fix; the way to bring one
    // of these under the gate is to author a project copy of it.
    if !selected.gate.is_gate_subject() {
        return Some(
            "Engine-owned models are exempt from the source-owned release gate — author a \
             project copy to qualify one"
                .to_owned(),
        );
    }
    if selected.source_error.is_some() {
        return Some("Select a project-owned model revision".to_owned());
    }
    if cx.state.model_editor.qualification_execution.is_some() {
        return Some(
            "Finish or cancel the active model qualification run before changing workflows"
                .to_owned(),
        );
    }
    if let Some(draft) = cx.state.model_editor.draft.as_ref() {
        let same_model = draft.library_name.eq_ignore_ascii_case(&selected.library)
            && draft.model_name.eq_ignore_ascii_case(&selected.model);
        if draft.is_dirty() && !same_model {
            return Some(format!(
                "Save or discard unsaved model candidate '{}/{}' first",
                draft.library_name, draft.model_name
            ));
        }
        if same_model
            && draft.definition_is_dirty()
            && matches!(
                action,
                QualificationPageAction::RunSuite | QualificationPageAction::CompareRelease
            )
        {
            return Some(
                "Save the changed model definition before running or comparing qualification"
                    .to_owned(),
            );
        }
    }
    match action {
        QualificationPageAction::ReviewVectors
        | QualificationPageAction::ReviewReleaseBinding
        | QualificationPageAction::OpenCorrelation => None,
        QualificationPageAction::RunSuite if cx.model.safe_mode.project_read_only() => {
            Some("Qualification cannot run while the project is read-only".to_owned())
        }
        QualificationPageAction::RunSuite if selected.suites == 0 => {
            Some("Author at least one executable qualification suite first".to_owned())
        }
        QualificationPageAction::RunSuite => None,
        QualificationPageAction::CompareRelease if !selected.comparison_available => {
            Some("The selected model has no approved release to compare".to_owned())
        }
        QualificationPageAction::CompareRelease => None,
    }
}

pub(super) fn execute_qualification_action(
    cx: &mut Cx<'_, ModelsFeature>,
    action: QualificationPageAction,
) {
    let summaries = qualification_summaries(cx.model, &cx.state.model_editor);
    let selected = selected_qualification_summary(cx, &summaries);
    // The page disables a blocked action and names the blocker on hover.
    if qualification_action_block_reason(cx, selected, action).is_some() {
        return;
    }
    if action == QualificationPageAction::OpenCorrelation {
        if let Err(error) = cx.wb.navigate(
            SurfaceRoute::surface(SurfaceId::ModelCorrelation),
            RouteTransitionSource::User,
        ) {
            cx.wb
                .prompt(format!("Measurement correlation cannot be opened: {error}"));
        }
        return;
    }
    let Some(library) = cx.model.model_library_manager.selected_library.clone() else {
        cx.wb
            .prompt("Select a model family before opening qualification.");
        return;
    };
    let Some(model) = cx.state.selected_model.clone() else {
        cx.wb
            .prompt("Select a model family before opening qualification.");
        return;
    };
    if let Err(error) = model_editor::open_project_model(
        &mut cx.state.model_editor_parts(cx.model, cx.wb, cx.platform),
        &library,
        &model,
    ) {
        cx.wb
            .prompt(format!("Qualification cannot be opened: {error}"));
        return;
    }
    if let Err(error) = cx.wb.navigate(
        SurfaceRoute::surface(SurfaceId::ModelEditor),
        RouteTransitionSource::User,
    ) {
        cx.wb
            .prompt(format!("Qualification editor cannot be shown: {error}"));
        return;
    }

    match action {
        QualificationPageAction::ReviewVectors => {
            cx.state.model_editor.active_section = ModelEditorSection::Tests;
            cx.state.model_editor.qualification_plan_open = true;
        }
        QualificationPageAction::ReviewReleaseBinding => {
            cx.state.model_editor.active_section = ModelEditorSection::Release;
        }
        QualificationPageAction::RunSuite => {
            cx.state.model_editor.active_section = ModelEditorSection::Tests;
            model_editor::validate_open_candidate(&mut cx.state.model_editor_parts(
                cx.model,
                cx.wb,
                cx.platform,
            ));
            let command = ModelsCommand::ModelRunQualificationTests;
            let availability = crate::workbench::models::command_availability(command, cx);
            if availability.is_available() {
                cx.execute(command.key());
            } else if let CommandAvailability::Disabled(reason) = availability {
                cx.wb.prompt(format!("Qualification cannot run: {reason}."));
            }
        }
        QualificationPageAction::CompareRelease => {
            cx.state.model_editor.active_section = ModelEditorSection::Release;
            let command = ModelsCommand::ModelCompareRelease;
            let availability = crate::workbench::models::command_availability(command, cx);
            if availability.is_available() {
                cx.execute(command.key());
            } else if let CommandAvailability::Disabled(reason) = availability {
                cx.wb.prompt(format!(
                    "Approved-model comparison is unavailable: {reason}."
                ));
            }
        }
        QualificationPageAction::OpenCorrelation => {}
    }
}
