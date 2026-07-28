//! Driving a cooperative qualification pass from the editor.
//!
//! The pass runs one vector at a time so the UI stays responsive, and its
//! results are committed to the draft only once a platform record is complete
//! and validated. A partial pass — cancelled, or stopped by a failure — never
//! reaches the draft, so the editor cannot show evidence for a suite that did
//! not finish.

use super::*;

pub(super) fn parse_frequency_axis(value: &str) -> Result<Vec<FiniteValue>, String> {
    let tokens = value
        .split(|character: char| character == ',' || character.is_whitespace())
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();
    if tokens.is_empty() {
        return Err("Frequency axis requires at least one exact frequency in hertz".to_owned());
    }
    tokens
        .into_iter()
        .enumerate()
        .map(|(index, token)| {
            FiniteValue::new(parse_finite(token, &format!("Frequency point {index}"))?)
                .map_err(|error| error.to_string())
        })
        .collect()
}

pub(super) fn parse_sample_index(value: &str, label: &str) -> Result<usize, String> {
    value
        .trim()
        .parse::<usize>()
        .map_err(|error| format!("{label} is invalid: {error}"))
}

pub(super) fn parse_finite(value: &str, label: &str) -> Result<f64, String> {
    let parsed = value
        .trim()
        .parse::<f64>()
        .map_err(|error| format!("{label} is invalid: {error}"))?;
    if parsed.is_finite() {
        Ok(parsed)
    } else {
        Err(format!("{label} must be finite"))
    }
}

pub(super) fn parse_non_negative(value: &str, label: &str) -> Result<f64, String> {
    let parsed = parse_finite(value, label)?;
    if parsed >= 0.0 {
        Ok(parsed)
    } else {
        Err(format!("{label} must be non-negative"))
    }
}

/// Open an exact project-owned model revision in the transactional editor.
pub fn open_project_model(
    app: &mut RSpiceApp,
    library_name: &str,
    model_name: &str,
) -> Result<(), String> {
    if app
        .state
        .workbench
        .model_editor
        .qualification_execution
        .is_some()
    {
        return Err(
            "Finish or cancel the active model qualification run before opening another model"
                .to_owned(),
        );
    }
    if let Some(open) = app.state.workbench.model_editor.draft.as_ref()
        && open.is_dirty()
    {
        if open.library_name.eq_ignore_ascii_case(library_name)
            && open.model_name.eq_ignore_ascii_case(model_name)
        {
            return Ok(());
        }
        return Err(format!(
            "Unsaved model candidate '{}/{}' is open; save or discard it before opening '{library_name}/{model_name}'",
            open.library_name, open.model_name
        ));
    }
    let mut editor = ModelEditorState::default();
    editor.open(
        &app.state.model_library_manager,
        library_name,
        model_name,
        app.state.workspace.project.revision(),
    )?;
    app.state
        .workbench
        .navigate(
            SurfaceRoute::surface(SurfaceId::ModelEditor),
            RouteTransitionSource::User,
        )
        .map_err(|error| error.to_string())?;
    app.state.workbench.model_editor = editor;
    app.state.model_library_manager.select_library(library_name);
    app.state.workbench.selected_model = Some(model_name.to_owned());
    Ok(())
}

/// Validate the open model draft and bind the resulting evidence to its exact
/// model-source and project revisions.
pub fn validate_open_candidate(app: &mut RSpiceApp) -> bool {
    let valid = app.state.workbench.model_editor.validate_candidate(
        &app.state.model_library_manager,
        app.state.workspace.project.revision(),
    );
    let message = if valid {
        ConsoleMessage::info("Model candidate is valid and bound to the current source revision.")
    } else {
        let summary = app
            .state
            .workbench
            .model_editor
            .diagnostics
            .first()
            .map_or("Model candidate is invalid.", |diagnostic| {
                diagnostic.message.as_str()
            });
        ConsoleMessage::warning(format!("Model candidate validation failed: {summary}"))
    };
    app.state.push_user_message(message);
    valid
}

/// Begin an exact, cooperative qualification pass for every persisted suite
/// on the current runtime. Desktop and WebAssembly records rendezvous through
/// persisted project state; neither target fabricates the other target's run.
pub fn start_qualification_execution(app: &mut RSpiceApp) -> Result<(), String> {
    start_qualification_execution_for(app, None)
}

/// Begin a cooperative rerun for one exact retained suite. The suite must
/// belong to the current project-owned source revision.
pub fn start_qualification_suite_execution(
    app: &mut RSpiceApp,
    suite_id: &str,
) -> Result<(), String> {
    if suite_id.trim().is_empty() {
        return Err("Qualification suite ID is required".to_owned());
    }
    start_qualification_execution_for(app, Some(suite_id))
}

/// Rerun the complete suite that owns a selected vector. Platform runs and
/// evidence are suite-complete contracts, so a vector action never publishes
/// a misleading partial suite result.
pub fn start_qualification_vector_execution(
    app: &mut RSpiceApp,
    suite_id: &str,
    vector_id: &str,
) -> Result<(), String> {
    app.state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .ok_or_else(|| "No project-owned model candidate is open".to_owned())?
        .qualification
        .qualification_vector(suite_id, vector_id)
        .map_err(|error| error.to_string())?;
    start_qualification_suite_execution(app, suite_id)
}

pub(super) fn start_qualification_execution_for(
    app: &mut RSpiceApp,
    selected_suite_id: Option<&str>,
) -> Result<(), String> {
    if !app.state.project_lifecycle.project_open {
        return Err("Qualification requires an open project".to_owned());
    }
    if app.state.workbench.safe_mode.project_read_only() {
        return Err("Qualification cannot run while the project is read-only".to_owned());
    }
    if !validate_open_candidate(app) {
        return Err(
            "Qualification cannot run until the exact model candidate validates".to_owned(),
        );
    }
    let editor = &mut app.state.workbench.model_editor;
    if editor.qualification_execution.is_some() {
        return Err("A model qualification run is already active".to_owned());
    }
    let draft = editor
        .draft
        .as_ref()
        .ok_or_else(|| "No project-owned model candidate is open".to_owned())?;
    if draft.definition_is_dirty() {
        return Err(
            "Save the model definition before producing source-bound qualification evidence"
                .to_owned(),
        );
    }
    draft
        .qualification
        .validate_for_model(&draft.model_name)
        .map_err(|error| format!("Qualification plan is invalid: {error}"))?;
    let source = ModelSourceEvidenceBinding::try_new_project_bound(
        draft.model_name.clone(),
        draft.source_id,
        draft.base_source_digest,
        draft.base_source_revision,
    )
    .map_err(|error| error.to_string())?;
    let exact_suites = draft
        .qualification
        .exact_suites_for_source(&source)
        .map_err(|error| format!("Qualification plan is invalid: {error}"))?;
    let selected_suites = if let Some(suite_id) = selected_suite_id {
        let suite = exact_suites
            .iter()
            .copied()
            .find(|suite| suite.id.eq_ignore_ascii_case(suite_id))
            .ok_or_else(|| {
                format!(
                    "Qualification suite {suite_id:?} is missing or is not bound to the current model source revision"
                )
            })?;
        vec![suite]
    } else {
        exact_suites
    };
    let first_suite = selected_suites.first().copied().ok_or_else(|| {
        "Add at least one executable qualification suite for the current model source revision before running"
            .to_owned()
    })?;
    let suite_ids = selected_suites
        .iter()
        .map(|suite| suite.id.clone())
        .collect::<Vec<_>>();
    let session = QualificationExecutionSession::try_new(first_suite, &source)
        .map_err(|error| error.to_string())?;
    let progress = session.progress();
    let platform = qualification_platform_label(progress.platform);
    editor.qualification_execution = Some(ModelQualificationExecution {
        suite_ids,
        suite_index: 0,
        source,
        session,
        progress,
        assembled_evidence: 0,
    });
    editor.qualification_execution_notice = Some(format!(
        "Running {platform} qualification. Completed vectors are retained only after each full suite finishes."
    ));
    Ok(())
}

/// Advance at most one qualification vector. Calling this once per frame keeps
/// desktop and browser shells responsive while preserving atomic run records.
pub fn advance_qualification_execution(app: &mut RSpiceApp) {
    let Some(mut execution) = app
        .state
        .workbench
        .model_editor
        .qualification_execution
        .take()
    else {
        return;
    };

    match execution.session.step(&rspice_core::NoAbort) {
        Ok(QualificationExecutionStep::InProgress(progress)) => {
            execution.progress = progress;
            app.state.workbench.model_editor.qualification_execution = Some(execution);
        }
        Ok(QualificationExecutionStep::Complete { progress, run }) => {
            execution.progress = progress;
            let suite_id = execution.current_suite_id().to_owned();
            let outcome = (|| -> Result<(), String> {
                let draft = app
                    .state
                    .workbench
                    .model_editor
                    .draft
                    .as_mut()
                    .ok_or_else(|| "The model candidate closed during qualification".to_owned())?;
                draft
                    .qualification
                    .upsert_platform_run_atomically(run)
                    .map_err(|error| error.to_string())?;
                let evidence_id = qualification_evidence_id(&suite_id, &execution.source);
                match draft.qualification.assemble_and_upsert_evidence_atomically(
                    evidence_id,
                    &suite_id,
                    &execution.source,
                ) {
                    Ok(evidence) => {
                        execution.assembled_evidence += 1;
                        let covered_sections = draft
                            .qualification
                            .suites
                            .iter()
                            .find(|suite| suite.id.eq_ignore_ascii_case(&suite_id))
                            .into_iter()
                            .flat_map(|suite| suite.vectors.iter())
                            .filter_map(|vector| vector.model_section.as_deref())
                            .map(str::to_ascii_lowercase)
                            .collect::<BTreeSet<_>>();
                        if !covered_sections.is_empty() {
                            let evidence_digest = evidence
                                .content_digest()
                                .map_err(|error| error.to_string())?;
                            if evidence.passed {
                                for section_name in &covered_sections {
                                    draft
                                        .qualification
                                        .validate_exact_section_evidence_digest(
                                            &execution.source,
                                            section_name,
                                            evidence_digest,
                                        )
                                        .map_err(|error| error.to_string())?;
                                }
                            }
                            for section in &mut draft.metadata.sections {
                                if !covered_sections.contains(&section.name.to_ascii_lowercase()) {
                                    continue;
                                }
                                if evidence.passed {
                                    section.qualification = ModelSectionQualification::Qualified {
                                        evidence_digest: Some(evidence_digest.to_string()),
                                    };
                                } else {
                                    section.qualification = ModelSectionQualification::Failed {
                                        summary: format!(
                                            "Qualification suite {suite_id} failed one or more declared Desktop/WebAssembly vector gates"
                                        ),
                                    };
                                }
                            }
                        }
                    }
                    Err(error)
                        if error.code == QualificationErrorCode::EvidenceCoverageMismatch =>
                    {
                        // The other real runtime has not published its exact run yet.
                    }
                    Err(error) => return Err(error.to_string()),
                }
                Ok(())
            })();
            if let Err(error) = outcome {
                app.state
                    .workbench
                    .model_editor
                    .qualification_execution_notice = Some(format!(
                    "Qualification stopped without publishing partial evidence: {error}"
                ));
                return;
            }

            execution.suite_index += 1;
            if execution.suite_index < execution.suite_ids.len() {
                let next_id = execution.current_suite_id().to_owned();
                let next_suite = app
                    .state
                    .workbench
                    .model_editor
                    .draft
                    .as_ref()
                    .and_then(|draft| {
                        draft
                            .qualification
                            .suites
                            .iter()
                            .find(|suite| suite.id.eq_ignore_ascii_case(&next_id))
                    })
                    .cloned();
                let Some(next_suite) = next_suite else {
                    app.state.workbench.model_editor.qualification_execution_notice = Some(
                        "Qualification plan changed during execution; remaining suites were not run."
                            .to_owned(),
                    );
                    return;
                };
                match QualificationExecutionSession::try_new(&next_suite, &execution.source) {
                    Ok(session) => {
                        execution.progress = session.progress();
                        execution.session = session;
                        app.state.workbench.model_editor.qualification_execution = Some(execution);
                    }
                    Err(error) => {
                        app.state
                            .workbench
                            .model_editor
                            .qualification_execution_notice = Some(format!(
                            "Qualification stopped before suite '{next_id}': {error}"
                        ));
                    }
                }
            } else {
                let platform = qualification_platform_label(execution.progress.platform);
                app.state
                    .workbench
                    .model_editor
                    .qualification_execution_notice = Some(format!(
                    "{platform} qualification completed for {} suite(s); {} exact Desktop/WebAssembly evidence record(s) were assembled. Save the qualification revision to retain the results.",
                    execution.total_suites(),
                    execution.assembled_evidence,
                ));
            }
        }
        Err(error) => {
            app.state
                .workbench
                .model_editor
                .qualification_execution_notice = Some(format!(
                "Qualification stopped without publishing a partial suite: {error}"
            ));
        }
    }
}

/// Cancel between vectors and discard all in-progress outcomes. Completed
/// suites already committed to the draft remain available for explicit save.
pub fn cancel_qualification_execution(app: &mut RSpiceApp) -> bool {
    let Some(mut execution) = app
        .state
        .workbench
        .model_editor
        .qualification_execution
        .take()
    else {
        return false;
    };
    execution.session.cancel();
    app.state
        .workbench
        .model_editor
        .qualification_execution_notice = Some(format!(
        "{} qualification cancelled before the next vector; no partial suite was published.",
        qualification_platform_label(execution.progress.platform),
    ));
    true
}

/// Apply a promotion transaction to the open candidate only when the project
/// remains writable. The subsequent explicit save publishes the immutable
/// release through the normal project revision history.
pub fn promote_open_candidate(app: &mut RSpiceApp, candidate_id: &str) -> bool {
    if !app.state.project_lifecycle.project_open {
        app.state.workbench.model_editor.promotion_error =
            Some("Model promotion requires an open project".to_owned());
        return false;
    }
    if app.state.workbench.safe_mode.project_read_only() {
        app.state.workbench.model_editor.promotion_error =
            Some("Model promotion is unavailable while the project is read-only".to_owned());
        return false;
    }
    if !validate_open_candidate(app) {
        app.state.workbench.model_editor.promotion_error = Some(
            "Model promotion requires a valid candidate bound to the current project revision"
                .to_owned(),
        );
        return false;
    }
    let Some((library_name, model_name)) = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .map(|draft| (draft.library_name.clone(), draft.model_name.clone()))
    else {
        app.state.workbench.model_editor.promotion_error =
            Some("No project-owned model candidate is open".to_owned());
        return false;
    };
    let Some(library) = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .find(|library| library.name.eq_ignore_ascii_case(&library_name))
    else {
        app.state.workbench.model_editor.promotion_error = Some(format!(
            "Model promotion cannot resolve project library '{library_name}'"
        ));
        return false;
    };
    let correlation = library
        .model_correlation
        .get(&model_name)
        .cloned()
        .unwrap_or_default();
    app.state
        .workbench
        .model_editor
        .commit_promotion(candidate_id, &correlation)
}

pub(super) fn qualification_evidence_id(
    suite_id: &str,
    source: &ModelSourceEvidenceBinding,
) -> String {
    format!(
        "{}-r{}-{}",
        suite_id,
        source.source_revision.get(),
        source.source_digest
    )
}

#[must_use]
pub const fn qualification_platform_label(platform: QualificationPlatform) -> &'static str {
    match platform {
        QualificationPlatform::Desktop => "Desktop",
        QualificationPlatform::WebAssembly => "WebAssembly",
    }
}

/// Publish the open candidate as one guarded project/model revision and then
/// reopen the committed revision as the new immutable editing base.
pub fn save_open_candidate(app: &mut RSpiceApp) -> Result<ObjectRevision, String> {
    if !app.state.project_lifecycle.project_open {
        return Err("Model revision cannot be saved without an open project".to_owned());
    }
    if app.state.workbench.safe_mode.project_read_only() {
        return Err("Model revision cannot be saved while the project is read-only".to_owned());
    }
    if app
        .state
        .workbench
        .model_editor
        .qualification_execution
        .is_some()
    {
        return Err(
            "Model revision cannot be saved while qualification execution is active".to_owned(),
        );
    }
    let draft = app
        .state
        .workbench
        .model_editor
        .draft
        .clone()
        .ok_or_else(|| "No project-owned model candidate is open".to_owned())?;
    if !draft.is_dirty() {
        return Err("Model candidate has no semantic changes to save".to_owned());
    }
    if !validate_open_candidate(app) {
        return Err("Model revision was not saved because validation failed".to_owned());
    }
    let definition = draft.definition().map_err(|diagnostics| {
        diagnostics
            .into_iter()
            .map(|diagnostic| format!("{}: {}", diagnostic.field, diagnostic.message))
            .collect::<Vec<_>>()
            .join("; ")
    })?;
    let definition_changed = definition != draft.base_definition;
    let mut candidate = app.state.model_library_manager.clone();
    let commit = if definition_changed {
        candidate.replace_project_model_revision_in_library(
            &draft.library_name,
            draft.source_id,
            draft.base_library_revision,
            draft.base_source_revision,
            &draft.model_name,
            draft.base_source_digest,
            &definition,
            &draft.qualification,
        )?
    } else {
        candidate.replace_project_model_qualification(
            &draft.library_name,
            draft.source_id,
            draft.base_library_revision,
            draft.base_source_revision,
            draft.base_source_digest,
            &draft.model_name,
            &draft.qualification,
        )?
    };
    let description = if definition_changed {
        format!(
            "save model revision {}/{}",
            commit.library_name, commit.model_name
        )
    } else {
        format!(
            "save model qualification {}/{}",
            commit.library_name, commit.model_name
        )
    };
    let committed_revision =
        app.state
            .publish_project_model_candidate(candidate, commit, description)?;
    if definition_changed {
        app.invalidate_simulation_preflight();
    }
    app.state
        .model_library_manager
        .select_library(&draft.library_name);
    app.state.workbench.selected_model = Some(definition.base.name.clone());
    let reopen_result = app.state.workbench.model_editor.open(
        &app.state.model_library_manager,
        &draft.library_name,
        &definition.base.name,
        committed_revision,
    );
    if reopen_result.is_ok() {
        app.state
            .workbench
            .model_editor
            .validate_candidate(&app.state.model_library_manager, committed_revision);
    } else if let Err(error) = reopen_result {
        // Publication has already committed at this point. Never report that
        // the save failed or leave a stale editable draft behind merely
        // because the post-commit convenience reopen could not be rebuilt.
        app.state.workbench.model_editor = ModelEditorState::default();
        app.state.push_user_message(ConsoleMessage::warning(format!(
            "Saved the model revision, but the editor could not reopen it: {error}"
        )));
    }
    let saved_kind = if definition_changed {
        "model revision"
    } else {
        "model qualification"
    };
    app.state.push_user_message(ConsoleMessage::info(format!(
        "Saved {saved_kind} {}/{} at project revision {}.",
        draft.library_name,
        definition.base.name,
        committed_revision.get()
    )));
    Ok(committed_revision)
}
