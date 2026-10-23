//! The model an instance runs, as the component inspector states it.

use rspice_ui_kit::panels::Problem;

use crate::state::model_library::ModelConsumerScope;
use crate::state::{CellViewRef, Component, ViewType, explicit_component_model};
use crate::workbench::AppState;

/// The model an instance runs, and what is wrong with that binding when
/// something is. A binding that resolves says nothing beyond the model's name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ComponentModelEvidence {
    /// The model card or module the deck gives the instance. `None` for a
    /// device that takes no model, or whose model the netlister writes itself.
    pub(super) model: Option<String>,
    /// The binding's problem, worded as the problem.
    pub(super) problem: Option<(Problem, String)>,
}

impl ComponentModelEvidence {
    pub(super) fn resolved(model: impl Into<String>) -> Self {
        Self {
            model: Some(model.into()),
            problem: None,
        }
    }

    fn failed(model: impl Into<String>, problem: Problem, text: String) -> Self {
        Self {
            model: Some(model.into()),
            problem: Some((problem, text)),
        }
    }
}

/// What an instance of a project-owned Verilog-A view runs: the module its
/// view names, and whether its source has a compile that is still current.
/// No model catalog is involved, so none is reported as unverified.
fn project_module_evidence(
    state: &AppState,
    binding: &crate::state::LibraryCellInstance,
) -> Option<ComponentModelEvidence> {
    let reference = CellViewRef::new(&binding.library, &binding.cell, &binding.view);
    let view = state
        .model
        .library_manager
        .get_library(&reference.library)?
        .get_cell(&reference.cell)?
        .get_view(&reference.view)?;
    if view.view_type != ViewType::VerilogA {
        return None;
    }
    let model = view
        .metadata
        .get("veriloga.module")
        .cloned()
        .unwrap_or_else(|| binding.cell.clone());
    let bundle = state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&crate::state::ProjectSourceOwner::cell_view(reference));
    Some(match bundle {
        Some(bundle) if bundle.validation_is_current() => ComponentModelEvidence::resolved(model),
        Some(_) => ComponentModelEvidence::failed(
            model,
            Problem::Warning,
            "Recompile needed: the module changed since its compile".to_owned(),
        ),
        None => ComponentModelEvidence::failed(
            model,
            Problem::Failure,
            "No Verilog-A source for this view".to_owned(),
        ),
    })
}

pub(super) fn component_model_evidence(
    state: &AppState,
    component: &Component,
) -> ComponentModelEvidence {
    if let Some(binding) = component.library_cell.as_ref() {
        if binding.is_builtin_xspice() {
            return match crate::state::validate_builtin_xspice_binding(binding) {
                Ok(descriptor) => ComponentModelEvidence::resolved(descriptor.model_type),
                Err(error) => ComponentModelEvidence::failed(
                    binding.cell.clone(),
                    Problem::Failure,
                    format!("Model invalid: {error}"),
                ),
            };
        }
        if binding.is_generated_veriloga() {
            return match crate::state::validate_generated_veriloga_binding(binding) {
                Ok(descriptor) => ComponentModelEvidence::resolved(descriptor.model_name),
                Err(error) => ComponentModelEvidence::failed(
                    binding.cell.clone(),
                    Problem::Failure,
                    format!("Model invalid: {error}"),
                ),
            };
        }
        if let Some(evidence) = project_module_evidence(state, binding) {
            return evidence;
        }
        if let Some(library) = state
            .model
            .model_library_manager
            .get_library(&binding.library)
        {
            let candidates = [
                binding.module_name.as_deref(),
                Some(binding.cell.as_str()),
                (!component.value.trim().is_empty()).then_some(component.value.trim()),
            ];
            if let Some(model) = candidates
                .into_iter()
                .flatten()
                .find_map(|candidate| library.bindable_model(candidate))
            {
                return bindable_model_evidence(library, &model);
            }
        }

        let model = binding
            .module_name
            .clone()
            .unwrap_or_else(|| binding.cell.clone());
        if binding.source_path.is_some() {
            // The deck includes the file the binding names; no loaded library
            // vouches for what is in it.
            let text = format!("Model {model} is not in a loaded library");
            return ComponentModelEvidence::failed(model, Problem::Warning, text);
        }

        let project_view_exists = state
            .model
            .library_manager
            .get_library(&binding.library)
            .and_then(|library| library.get_cell(&binding.cell))
            .and_then(|cell| cell.get_view(&binding.view))
            .is_some();
        return if project_view_exists {
            ComponentModelEvidence::resolved(model)
        } else {
            let text = format!(
                "Library cell {}/{} ({}) not found",
                binding.library, binding.cell, binding.view
            );
            ComponentModelEvidence::failed(model, Problem::Failure, text)
        };
    }

    if let Some(model_name) = explicit_component_model(component) {
        // Resolution is a project-global decision, not a per-surface scan.
        // Counting name matches reported "ambiguous" for every contested
        // definition — including ones the project had already resolved, which
        // is what execution and the Models workspace both actually run.
        let not_found = |model_name: String| {
            let text = format!("Model {model_name} not found in the loaded libraries");
            ComponentModelEvidence::failed(model_name, Problem::Failure, text)
        };
        return match state
            .model
            .model_library_manager
            .effective_definition_provider(ModelConsumerScope::PrimitiveModel, &model_name)
        {
            Ok(Some(provider)) => state
                .model
                .model_library_manager
                .get_library(&provider.library)
                .and_then(|library| {
                    library
                        .bindable_model(&provider.definition)
                        .map(|model| bindable_model_evidence(library, &model))
                })
                .unwrap_or_else(|| not_found(model_name)),
            Ok(None) => not_found(model_name),
            Err(reason) => {
                let text = format!("Model {model_name} ambiguous: {reason}");
                ComponentModelEvidence::failed(model_name, Problem::Failure, text)
            }
        };
    }

    // A native device, or one whose model card the netlister writes from the
    // instance itself: there is no model to name.
    ComponentModelEvidence {
        model: None,
        problem: None,
    }
}

/// A binned family is as sound as its least sound bin, since the instance's
/// geometry may select any of them.
fn bindable_model_evidence(
    library: &crate::state::model_library::ModelLibrary,
    model: &crate::state::model_library::BindableModel<'_>,
) -> ComponentModelEvidence {
    match model {
        crate::state::model_library::BindableModel::Card(card) => {
            catalog_model_evidence(library, card)
        }
        crate::state::model_library::BindableModel::Family(family) => family
            .bins
            .iter()
            .map(|bin| catalog_model_evidence(library, bin))
            .find(|evidence| evidence.problem.is_some())
            .unwrap_or_else(|| ComponentModelEvidence::resolved(family.name.to_owned())),
    }
}

fn catalog_model_evidence(
    library: &crate::state::model_library::ModelLibrary,
    model: &crate::state::model_library::DeviceModel,
) -> ComponentModelEvidence {
    let external = library.root_path.is_some() || model.file_path.is_some();
    if !external {
        return ComponentModelEvidence::resolved(model.name.clone());
    }

    let source = model.file_path.as_deref().or(library.root_path.as_deref());
    let recorded = library.root_path.as_ref().and_then(|_| {
        source.and_then(|source| library.source_closure.iter().find(|pin| pin.path == source))
    });
    match (source, recorded) {
        (Some(_), Some(_)) => ComponentModelEvidence::resolved(model.name.clone()),
        (Some(source), None) => {
            let text = format!(
                "Model file {} is not one of library {}'s files",
                source.display(),
                library.name
            );
            ComponentModelEvidence::failed(model.name.clone(), Problem::Failure, text)
        }
        (None, _) => {
            let text = format!(
                "Library {} does not say which file holds {}",
                library.name, model.name
            );
            ComponentModelEvidence::failed(model.name.clone(), Problem::Failure, text)
        }
    }
}

#[cfg(test)]
mod tests;
