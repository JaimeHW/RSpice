//! Opening, creating, validating and saving a project model in the editor.

use super::*;
use crate::workbench::state::WorkbenchSession;
use rspice_app_runs::simulation::SimulationController;
use rspice_app_runs::workbench::app_state::run_preflight::PreflightDialogState;
use rspice_app_services::workbench::slots::platform::PlatformState;
use rspice_app_state::workbench::app_state::model::Model;

/// What the model commands' selection rules read: the device-model editor,
/// the catalog's selected model, the model and the platform services' state,
/// which says whether a project is open.
///
/// Whether a model can be opened, copied or saved is a question about those
/// parts alone, so the rules take them rather than the application state, and
/// a feature's availability can answer them from what it reads.
#[derive(Clone, Copy)]
pub struct ModelEditorReads<'a> {
    pub(crate) editor: &'a ModelEditorState,
    pub(crate) selected_model: &'a Option<String>,
    pub(crate) model: &'a Model,
    pub(crate) platform: &'a PlatformState,
}

/// What opening a model in the editor writes: the editor, the catalog's
/// selected model, the model, which the editor's revision is published to
/// and whose library becomes the selected one, and the session, which
/// routes to the editor; beside the platform state it reads.
///
/// Opening a model or copying one into the project reads and writes those
/// parts alone, so the operations take them rather than the application
/// state, and a feature's command can run them in its context.
pub struct ModelEditorParts<'a> {
    pub(crate) editor: &'a mut ModelEditorState,
    pub(crate) selected_model: &'a mut Option<String>,
    pub(crate) model: &'a mut Model,
    pub(crate) workbench: &'a mut WorkbenchSession,
    pub(crate) platform: &'a PlatformState,
}

/// Copy one selected read-only catalog model into a new project-owned revision
/// and open that exact committed revision in the transactional editor.
///
/// Candidate construction and editor resolution happen before the guarded
/// project publication. A parse, metadata, identity, or editor error therefore
/// leaves both the project and global Undo history unchanged.
pub fn create_editable_project_copy_and_open(
    parts: &mut ModelEditorParts<'_>,
    source_library_name: &str,
    source_model_name: &str,
) -> Result<(String, String), String> {
    creation_refusal(parts, "an editable model copy")?;

    let project_library_name =
        available_project_copy_library_name(&parts.model.model_library_manager, source_model_name);
    let mut candidate = parts.model.model_library_manager.clone();
    let commit = candidate.create_editable_project_copy(
        source_library_name,
        source_model_name,
        &project_library_name,
    )?;
    let project_model_name = commit.model_name.clone();
    let expected_project_revision = parts
        .model
        .workspace
        .content
        .project
        .next_revision()
        .map_err(|error| error.to_string())?;
    let mut editor = ModelEditorState::default();
    editor.open(
        &candidate,
        &project_library_name,
        &project_model_name,
        expected_project_revision,
    )?;

    parts.model.publish_project_model_candidate(
        candidate,
        commit,
        format!(
            "create editable project model {source_library_name}/{source_model_name} as {project_library_name}/{project_model_name}"
        ),
    )?;
    parts
        .workbench
        .navigate(
            SurfaceRoute::surface(SurfaceId::ModelEditor),
            RouteTransitionSource::User,
        )
        .map_err(|error| error.to_string())?;
    *parts.editor = editor;
    crate::workbench::models::workspace_state::select_library_or_prompt(
        parts.model,
        parts.workbench,
        &project_library_name,
    );
    *parts.selected_model = Some(project_model_name.clone());
    Ok((project_library_name, project_model_name))
}

/// Attach `library` to the active plan, as the Simulate Models page's
/// Attach does and as one step of the setup's history, unless the plan binds
/// it already: a model made in the project is one the project's runs read.
fn attach_library_to_active_plan(model: &mut Model, library: &str) -> Result<(), String> {
    if model
        .sim_setup
        .model_bindings
        .iter()
        .any(|binding| binding.library_name.eq_ignore_ascii_case(library))
    {
        return Ok(());
    }
    let before = model
        .setup_snapshot(crate::workbench::app_state::design_history::SetupParts::MODEL_BINDINGS);
    let mut candidate = model.sim_setup.clone();
    candidate.model_bindings.push(
        model
            .model_library_manager
            .simulation_plan_binding(library)?,
    );
    model
        .model_library_manager
        .validate_simulation_plan_bindings(&candidate.model_bindings)?;
    let receipt = candidate
        .commit_active_plan_configuration_change(format!(
            "Attached model library {library} to the plan."
        ))
        .map_err(|error| error.to_string())?;
    model.sim_setup = candidate;
    model.record_setup_change(crate::state::SimulationPage::Models.label(), before);
    model.record_plan_receipt(receipt.status_line());
    Ok(())
}

/// Why the editor cannot take a newly created project model now, naming
/// `what` is being created: no project, a read-only one, a project operation
/// in flight, or an unsaved candidate it would replace.
fn creation_refusal(parts: &ModelEditorParts<'_>, what: &str) -> Result<(), String> {
    if !parts.platform.project_lifecycle.is_open() {
        return Err(format!("Open a project before creating {what}"));
    }
    if parts.model.safe_mode.project_read_only() {
        return Err(format!(
            "The project is read-only, so {what} cannot be created"
        ));
    }
    if parts.platform.project_lifecycle.operation_in_progress() {
        return Err(format!(
            "Wait for the current project operation before creating {what}"
        ));
    }
    if let Some(open) = parts.editor.draft.as_ref()
        && open.is_dirty()
    {
        return Err(format!(
            "Unsaved model candidate '{}/{}' is open; save or discard it before creating {what}",
            open.library_name, open.model_name
        ));
    }
    Ok(())
}

/// Create a new project model of the device type `defaults` describes, named
/// `name`: its card states the type, its level when it has more than one,
/// and every parameter of the type at the engine's own default, each with
/// its unit and what it is. The model is published as one undoable project
/// revision in a library of its own and opened in the model editor, as an
/// editable copy is.
pub fn create_new_project_model_and_open(
    parts: &mut ModelEditorParts<'_>,
    defaults: &rspice_core::library::DeviceModelDefaults,
    name: &str,
) -> Result<(String, String), String> {
    creation_refusal(parts, "a model")?;
    let name = name.trim();
    let mut numeric_parameters = std::collections::BTreeMap::new();
    if let Some(level) = defaults.level {
        numeric_parameters.insert("LEVEL".to_owned(), f64::from(level));
    }
    for parameter in defaults.parameters {
        numeric_parameters.insert(parameter.name.to_owned(), parameter.default);
    }
    let base = ProjectModelDefinition {
        name: name.to_owned(),
        spice_type: defaults.spice_type.to_owned(),
        description: defaults.label.to_owned(),
        numeric_parameters,
        string_parameters: std::collections::BTreeMap::new(),
    };
    let mut metadata = base.reconcile_metadata(None)?;
    for parameter in &mut metadata.parameters {
        if parameter.name == "LEVEL" {
            parameter.description = "model level".to_owned();
            continue;
        }
        if let Some(default) = defaults
            .parameters
            .iter()
            .find(|default| default.name == parameter.name)
        {
            parameter.unit = (!default.unit.is_empty()).then(|| default.unit.to_owned());
            default.description.clone_into(&mut parameter.description);
        }
    }
    let definition = ProjectModelRevisionDefinition::new(base, metadata);

    let project_library_name =
        available_project_copy_library_name(&parts.model.model_library_manager, name);
    let mut candidate = parts.model.model_library_manager.clone();
    let commit = candidate.create_project_model_revision(&project_library_name, &definition)?;
    let project_model_name = commit.model_name.clone();
    let expected_project_revision = parts
        .model
        .workspace
        .content
        .project
        .next_revision()
        .map_err(|error| error.to_string())?;
    let mut editor = ModelEditorState::default();
    editor.open(
        &candidate,
        &project_library_name,
        &project_model_name,
        expected_project_revision,
    )?;
    parts.model.publish_project_model_candidate(
        candidate,
        commit,
        format!(
            "create {} model {project_library_name}/{project_model_name}",
            defaults.spice_type
        ),
    )?;
    if let Err(error) = attach_library_to_active_plan(parts.model, &project_library_name) {
        parts.model.record(
            LogSeverity::Warning,
            LogSource::Studio,
            format!("Model library '{project_library_name}' was not attached to the plan: {error}"),
        );
    }
    parts
        .workbench
        .navigate(
            SurfaceRoute::surface(SurfaceId::ModelEditor),
            RouteTransitionSource::User,
        )
        .map_err(|error| error.to_string())?;
    *parts.editor = editor;
    crate::workbench::models::workspace_state::select_library_or_prompt(
        parts.model,
        parts.workbench,
        &project_library_name,
    );
    *parts.selected_model = Some(project_model_name.clone());
    Ok((project_library_name, project_model_name))
}

fn available_project_copy_library_name(manager: &ModelLibraryManager, model_name: &str) -> String {
    for sequence in 1_u32.. {
        let suffix = if sequence == 1 {
            " project".to_owned()
        } else {
            format!(" project {sequence}")
        };
        let maximum_stem_bytes = 128_usize.saturating_sub(suffix.len());
        let mut stem = String::new();
        for character in model_name.chars() {
            if character.is_control() || matches!(character, '/' | '\\') {
                continue;
            }
            if stem.len() + character.len_utf8() > maximum_stem_bytes {
                break;
            }
            stem.push(character);
        }
        if stem.trim().is_empty() {
            stem.push_str("editable model");
        }
        let candidate = format!("{}{suffix}", stem.trim());
        if manager
            .libraries_sorted()
            .iter()
            .all(|library| !library.name.eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
    }
    unreachable!("u32 project-copy suffixes exhaust practical library identities")
}

/// Open an exact project-owned model revision in the transactional editor.
pub fn open_project_model(
    parts: &mut ModelEditorParts<'_>,
    library_name: &str,
    model_name: &str,
) -> Result<(), String> {
    if let Some(open) = parts.editor.draft.as_ref()
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
        &parts.model.model_library_manager,
        library_name,
        model_name,
        parts.model.workspace.content.project.revision(),
    )?;
    parts
        .workbench
        .navigate(
            SurfaceRoute::surface(SurfaceId::ModelEditor),
            RouteTransitionSource::User,
        )
        .map_err(|error| error.to_string())?;
    *parts.editor = editor;
    crate::workbench::models::workspace_state::select_library_or_prompt(
        parts.model,
        parts.workbench,
        library_name,
    );
    *parts.selected_model = Some(model_name.to_owned());
    Ok(())
}

/// Validate the open model draft and bind the resulting evidence to its exact
/// model-source and project revisions.
pub fn validate_open_candidate(parts: &mut ModelEditorParts<'_>) -> bool {
    let valid = parts.editor.validate_candidate(
        &parts.model.model_library_manager,
        parts.model.workspace.content.project.revision(),
    );
    let message = if valid {
        "Model candidate is valid against the current source revision.".to_owned()
    } else {
        let summary = parts
            .editor
            .diagnostics
            .first()
            .map_or("Model candidate is invalid.", |diagnostic| {
                diagnostic.message.as_str()
            });
        format!("Model candidate validation failed: {summary}")
    };
    parts.workbench.prompt(message);
    valid
}

/// Publish the open candidate as one guarded project/model revision and then
/// reopen the committed revision as the new immutable editing base. Saving
/// drops the run's preflight and the run it prepared.
pub fn save_open_candidate(
    parts: &mut ModelEditorParts<'_>,
    preflight: &mut PreflightDialogState,
    run_controller: &mut SimulationController,
) -> Result<ObjectRevision, String> {
    if !parts.platform.project_lifecycle.is_open() {
        return Err("Model revision cannot be saved without an open project".to_owned());
    }
    if parts.model.safe_mode.project_read_only() {
        return Err("Model revision cannot be saved while the project is read-only".to_owned());
    }
    let draft = parts
        .editor
        .draft
        .clone()
        .ok_or_else(|| "No project-owned model candidate is open".to_owned())?;
    if !draft.is_dirty() {
        return Err("Model candidate has no semantic changes to save".to_owned());
    }
    if !validate_open_candidate(parts) {
        return Err("Model revision was not saved because validation failed".to_owned());
    }
    let definition = draft.definition().map_err(|diagnostics| {
        diagnostics
            .into_iter()
            .map(|diagnostic| format!("{}: {}", diagnostic.field, diagnostic.message))
            .collect::<Vec<_>>()
            .join("; ")
    })?;
    let mut candidate = parts.model.model_library_manager.clone();
    let commit = candidate.replace_project_model_revision_in_library(
        &draft.library_name,
        draft.source_id,
        draft.base_library_revision,
        draft.base_source_revision,
        &draft.model_name,
        draft.base_source_digest,
        &definition,
    )?;
    let description = format!(
        "save model revision {}/{}",
        commit.library_name, commit.model_name
    );
    let committed_revision =
        parts
            .model
            .publish_project_model_candidate(candidate, commit, description)?;
    preflight.invalidate();
    run_controller.clear_prepared_run();
    crate::workbench::models::workspace_state::select_library_or_prompt(
        parts.model,
        parts.workbench,
        &draft.library_name,
    );
    *parts.selected_model = Some(definition.base.name.clone());
    let reopen_result = parts.editor.open(
        &parts.model.model_library_manager,
        &draft.library_name,
        &definition.base.name,
        committed_revision,
    );
    if reopen_result.is_ok() {
        parts
            .editor
            .validate_candidate(&parts.model.model_library_manager, committed_revision);
    } else if let Err(error) = reopen_result {
        // Publication has already committed at this point. Never report that
        // the save failed or leave a stale editable draft behind merely
        // because the post-commit convenience reopen could not be rebuilt.
        *parts.editor = ModelEditorState::default();
        parts.model.record(
            LogSeverity::Warning,
            LogSource::Studio,
            format!("Saved the model revision, but the editor could not reopen it: {error}"),
        );
    }
    parts.model.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!(
            "Saved model revision {}/{} at project revision {}.",
            draft.library_name,
            definition.base.name,
            committed_revision.get()
        ),
    );
    Ok(committed_revision)
}
