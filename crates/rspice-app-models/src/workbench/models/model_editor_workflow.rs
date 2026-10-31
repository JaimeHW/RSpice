//! The model editor's workflow requests and the rules that say which
//! model its commands act on.
//!
//! A model command opens, copies or saves exactly one coherent, project-owned
//! definition, or it is unavailable. The review surfaces the editor's toolbar
//! commands open are requested here, as UI-local state beside the persisted
//! model draft, which remains the sole source of engineering truth.

use std::cell::RefCell;

use crate::workbench::documents::model_editor::ModelEditorReads;
use rspice_app_state::workbench::app_state::model::Model;

pub(super) fn model_library_rescan_diagnostic(model: &Model) -> (String, bool) {
    let discovered_files = model.pdk_config.discovered_files.len();
    let scan_errors = &model.pdk_config.scan_errors;
    let pack_definitions = model.model_library_manager.pack_definition_count();
    let mut summary = format!(
        "Found {discovered_files} configured model file(s) and {pack_definitions} shipped pack \
         part(s)"
    );
    let pack_error = model.model_library_manager.spice_pack_error();
    if let Some(error) = pack_error {
        summary = format!("{summary}; {error}");
    }
    if !scan_errors.is_empty() {
        summary = format!(
            "{summary}; {} configured path error(s): {}",
            scan_errors.len(),
            scan_errors.join("; ")
        );
    }
    (summary, pack_error.is_some() || !scan_errors.is_empty())
}

/// Resolve the catalog's single selection only when it is an exact,
/// authenticated project-owned revision accepted by the model editor.
///
/// Command enablement is deliberately fail-closed. A stale catalog selection,
/// mismatched retained bytes, or a partial source closure must never expose an
/// editor command that can only fail after navigation.
pub fn selected_project_model_for_editor<'a>(
    reads: ModelEditorReads<'a>,
) -> Result<(&'a str, &'a str), &'static str> {
    if !reads.platform.project_lifecycle.is_open() {
        return Err("open a project before editing a device model");
    }
    let library_name = reads
        .model
        .model_library_manager
        .selected_library
        .as_deref()
        .ok_or("select one model in Model & library catalog")?;
    let model_name = reads
        .selected_model
        .as_deref()
        .ok_or("select one model in Model & library catalog")?;
    let library = reads
        .model
        .model_library_manager
        .get_library(library_name)
        .ok_or("the selected model library no longer exists")?;
    match library.source_authority {
        crate::state::model_library::ModelSourceAuthority::ProjectOwned { .. } => {}
        crate::state::model_library::ModelSourceAuthority::BuiltIn => {
            return Err("the selected model is built-in; create an editable project copy first");
        }
        crate::state::model_library::ModelSourceAuthority::External => {
            return Err("the selected model is external; create an editable project copy first");
        }
        crate::state::model_library::ModelSourceAuthority::RetainedImport { .. } => {
            return Err(
                "the selected model is a read-only retained import; create an editable project copy first",
            );
        }
    }
    crate::workbench::documents::model_editor::resolve_project_model_for_editor(
        &reads.model.model_library_manager,
        library_name,
        model_name,
    )
    .map_err(
        |_| "the selected project model retained source or typed definition is inconsistent",
    )?;
    Ok((library_name, model_name))
}

/// Resolve one selected external or built-in model that can become a new
/// project-owned revision. Mutation authority and editor lifecycle checks live
/// here so every command surface exposes the same fail-closed availability.
pub(super) fn selected_model_for_project_copy<'a>(
    reads: ModelEditorReads<'a>,
) -> Result<(&'a str, &'a str), &'static str> {
    if !reads.platform.project_lifecycle.is_open() {
        return Err("open a project before creating an editable model copy");
    }
    let library_name = reads
        .model
        .model_library_manager
        .selected_library
        .as_deref()
        .ok_or("select one external or built-in model in Model & library catalog")?;
    let model_name = reads
        .selected_model
        .as_deref()
        .ok_or("select one external or built-in model in Model & library catalog")?;
    let library = reads
        .model
        .model_library_manager
        .get_library(library_name)
        .ok_or("the selected model library no longer exists")?;
    if !library.models.contains_key(model_name) {
        return Err("the selected model no longer exists in its library");
    }
    if library.source_authority.is_project_owned() {
        return Err("the selected model is already an editable project copy");
    }
    if reads.model.safe_mode.project_read_only() {
        return Err("the project is open read-only");
    }
    if reads.editor.qualification_execution.is_some() {
        return Err("a model qualification run is active");
    }
    if reads
        .editor
        .draft
        .as_ref()
        .is_some_and(|draft| draft.is_dirty())
    {
        return Err("save or discard the open model candidate first");
    }
    Ok((library_name, model_name))
}

/// Model-editor toolbar commands open governed review surfaces before they
/// perform an operation. This state is deliberately UI-local: the persisted
/// model draft remains the sole source of engineering truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ModelEditorWorkflow {
    SaveRevision,
    ValidateCandidate,
    RunQualification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ModelEditorWorkflowRequest {
    pub(crate) workflow: ModelEditorWorkflow,
    pub(crate) library_name: String,
    pub(crate) model_name: String,
    pub(crate) prepared: bool,
    pub(crate) error: Option<String>,
}

thread_local! {
    static MODEL_EDITOR_WORKFLOW: RefCell<Option<ModelEditorWorkflowRequest>> = const {
        RefCell::new(None)
    };
}

pub(crate) fn request_model_editor_workflow(
    reads: ModelEditorReads<'_>,
    workflow: ModelEditorWorkflow,
) -> bool {
    let Some(draft) = reads.editor.draft.as_ref() else {
        return false;
    };
    MODEL_EDITOR_WORKFLOW.with(|state| {
        *state.borrow_mut() = Some(ModelEditorWorkflowRequest {
            workflow,
            library_name: draft.library_name.clone(),
            model_name: draft.model_name.clone(),
            prepared: false,
            error: None,
        });
    });
    true
}

pub(crate) fn active_model_editor_workflow(
    reads: ModelEditorReads<'_>,
) -> Option<ModelEditorWorkflowRequest> {
    let identity = reads
        .editor
        .draft
        .as_ref()
        .map(|draft| (draft.library_name.as_str(), draft.model_name.as_str()));
    MODEL_EDITOR_WORKFLOW.with(|state| {
        let mut state = state.borrow_mut();
        let matches_open_candidate = state.as_ref().is_some_and(|request| {
            identity.is_some_and(|(library, model)| {
                request.library_name.eq_ignore_ascii_case(library)
                    && request.model_name.eq_ignore_ascii_case(model)
            })
        });
        if matches_open_candidate {
            state.clone()
        } else {
            *state = None;
            None
        }
    })
}

pub(crate) fn prepare_model_editor_workflow() {
    MODEL_EDITOR_WORKFLOW.with(|state| {
        if let Some(request) = state.borrow_mut().as_mut() {
            request.prepared = true;
        }
    });
}

pub(crate) fn set_model_editor_workflow_error(error: impl Into<String>) {
    MODEL_EDITOR_WORKFLOW.with(|state| {
        if let Some(request) = state.borrow_mut().as_mut() {
            request.error = Some(error.into());
        }
    });
}

pub(crate) fn close_model_editor_workflow() {
    MODEL_EDITOR_WORKFLOW.with(|state| {
        *state.borrow_mut() = None;
    });
}
