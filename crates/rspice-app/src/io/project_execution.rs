//! Application adapters for portable project execution context.
//!
//! The project crate owns persisted data, migration and validation. These
//! adapters capture workbench state and report host source availability.

use crate::product::ProjectId;
use crate::state::model_library::{ModelLibraryManager, ModelSourceAuthority};
#[cfg(not(target_arch = "wasm32"))]
use crate::state::model_library::{first_unreachable_source, is_foreign_platform_absolute_path};
use crate::workbench::app_state::SimSetupState;

use rspice_project::{ProjectExecutionContext, ProjectModelLibrary};

/// Capture persisted execution inputs without storing editor or catalog-browser state.
pub(crate) fn capture_execution_context(
    simulation_plan: &SimSetupState,
    model_libraries: &ModelLibraryManager,
) -> Result<ProjectExecutionContext, String> {
    ProjectExecutionContext::from_setup(
        simulation_plan.setup.clone(),
        model_libraries
            .libraries_sorted()
            .into_iter()
            .map(ProjectModelLibrary::from)
            .collect(),
        model_libraries.owned_model_resolution_records(),
        model_libraries.model_validation_receipt().cloned(),
    )
}

/// Restore editor/catalog state after portable migration and validation succeed.
pub(crate) fn restore_execution_context(
    mut context: ProjectExecutionContext,
    project_id: ProjectId,
) -> Result<(SimSetupState, ModelLibraryManager, Vec<String>), String> {
    context.migrate_to_current(project_id)?;
    context.validate()?;
    let warnings = model_source_warnings(&context.model_libraries);
    let mut manager = ModelLibraryManager::new();
    for library in context.model_libraries {
        manager.add_library(library.into_model_library());
    }
    manager.restore_model_resolution_records(context.model_resolution_records)?;
    manager.restore_model_validation_receipt(context.model_validation_receipt)?;
    let mut simulation_plan = SimSetupState {
        setup: context.simulation_plan,
        session: Default::default(),
    };
    simulation_plan.prepare_after_restore();
    Ok((simulation_plan, manager, warnings))
}

#[cfg(not(target_arch = "wasm32"))]
fn model_source_warnings(libraries: &[ProjectModelLibrary]) -> Vec<String> {
    let mut warnings = Vec::new();
    for library in libraries {
        if library.source_authority != ModelSourceAuthority::External {
            continue;
        }
        let Some(path) = library.root_path.as_deref() else {
            continue;
        };
        if library.source_closure.is_empty() {
            warnings.push(format!(
                "Model library '{}' was restored with its persisted catalog and source binding, but the legacy binding is not content-pinned; refresh or re-import '{}' before simulation",
                library.name,
                path.display()
            ));
            continue;
        }
        if is_foreign_platform_absolute_path(path)
            || library
                .source_closure
                .iter()
                .any(|source| is_foreign_platform_absolute_path(&source.path))
        {
            warnings.push(format!(
                "Model library '{}' retains a foreign-platform source binding rooted at '{}'; it is unavailable on this host and simulations that require it remain blocked until it is re-imported or repaired",
                library.name,
                path.display()
            ));
            continue;
        }
        if library.source_closure.len() > 1 && library.source_edges.is_empty() {
            warnings.push(format!(
                "Model library '{}' was restored with schema-2 content pins but no authenticated dependency-resolution graph; refresh or re-import '{}' before simulation",
                library.name,
                path.display()
            ));
            continue;
        }
        if !library.source_edges.is_empty()
            && let Some(unreachable) =
                first_unreachable_source(path, &library.source_closure, &library.source_edges)
        {
            warnings.push(format!(
                "Model library '{}' retains source '{}' that is not reachable from its authenticated root; simulations remain blocked until the library is refreshed or re-imported",
                library.name,
                unreachable.display()
            ));
            continue;
        }
        for source in &library.source_closure {
            if !source.path.is_file() {
                warnings.push(format!(
                    "Model library '{}' was restored with its exact persisted source closure, but dependency '{}' is unavailable; simulations that require it remain blocked",
                    library.name,
                    source.path.display()
                ));
                break;
            }
            match ModelLibraryManager::calculate_source_digest(&source.path) {
                Ok(actual_digest) if actual_digest == source.digest => {}
                Ok(_) => {
                    warnings.push(format!(
                        "Model library '{}' was restored with its persisted catalog, but dependency '{}' differs from the explicitly accepted SHA-256 identity; simulation remains blocked until an explicit refresh or re-import accepts the new source closure",
                        library.name,
                        source.path.display()
                    ));
                    break;
                }
                Err(error) => {
                    warnings.push(format!(
                        "Model library '{}' was restored with its persisted catalog, but dependency '{}' could not be verified ({error}); simulation remains blocked",
                        library.name,
                        source.path.display()
                    ));
                    break;
                }
            }
        }
    }
    warnings
}

#[cfg(target_arch = "wasm32")]
fn model_source_warnings(libraries: &[ProjectModelLibrary]) -> Vec<String> {
    libraries
        .iter()
        .filter(|library| library.source_authority == ModelSourceAuthority::External)
        .filter_map(|library| {
            library.root_path.as_ref().map(|path| {
                format!(
                    "Model library '{}' keeps its persisted source closure rooted at '{}', but browser builds cannot verify desktop file paths; simulations that require it remain blocked until the source is re-imported through an available browser workflow",
                    library.name,
                    path.display()
                )
            })
        })
        .collect()
}

#[cfg(test)]
mod tests;
