//! Model-provider authority for the frozen design hierarchy.
//!
//! Checks retained instance bindings against project catalog decisions and
//! resolves signed package authority only when a referenced provider needs it.

use std::path::Path;

use crate::pdk::{ValidatedPdkTechnologyPackage, project_signed_technology_package};
use crate::preparation::{PreparationError, PreparationStage};
use rspice_design::library::LibraryCatalog;
use rspice_model_library::ProjectTechnologyBinding;
use rspice_model_library::{ModelCatalog, ModelResolutionRecords};

/// Revalidate every model-bearing instance in the frozen hierarchy against
/// the project-global provider decision. Editor properties are not an
/// execution authority: restored projects and older symbol revisions must
/// pass this boundary immediately before their sources are sealed.
pub fn validate_projected_model_binding_authority(
    models: &ModelCatalog,
    resolutions: &ModelResolutionRecords,
    libraries: &LibraryCatalog,
    technology_binding: Option<&ProjectTechnologyBinding>,
    packages: &[ValidatedPdkTechnologyPackage],
    projection: &rspice_design::projection::ConfigurationExecutionProjection,
) -> Result<(), PreparationError> {
    use rspice_model_library::ModelConsumerScope;

    for (view, schematic) in projection.schematic_buffers() {
        for component in &schematic.document().components {
            let params = rspice_design::parameters::parse_params_string(&component.params);
            let model_bound_cell = component.library_cell.as_ref().filter(|binding| {
                binding.netlist_template.is_some() && !binding.is_executable_builtin()
            });
            let (scope, definition) = if let Some(binding) = model_bound_cell {
                let definition = binding
                    .module_name
                    .as_deref()
                    .map(str::trim)
                    .filter(|definition| !definition.is_empty())
                    .ok_or_else(|| {
                        PreparationError::new(
                            PreparationStage::ModelBindings,
                            format!(
                                "Model-bound instance '{}:{}' has no executable model or subcircuit name",
                                view, component.name
                            ),
                        )
                    })?;
                let is_subcircuit = binding
                    .effective_reference_prefix()
                    .is_some_and(|prefix| prefix.eq_ignore_ascii_case("X"))
                    || binding
                        .netlist_template
                        .as_deref()
                        .is_some_and(|template| template.trim_start().starts_with("X{name}"));
                (
                    if is_subcircuit {
                        ModelConsumerScope::Subcircuit
                    } else {
                        ModelConsumerScope::PrimitiveModel
                    },
                    definition.to_owned(),
                )
            } else if let Some(definition) = params
                .get("model")
                .map(String::as_str)
                .map(str::trim)
                .filter(|definition| !definition.is_empty())
                .or_else(|| {
                    component_value_is_model_name(component.kind)
                        .then_some(component.value.as_str())
                        .map(str::trim)
                        .filter(|definition| !definition.is_empty())
                })
            {
                (ModelConsumerScope::PrimitiveModel, definition.to_owned())
            } else {
                continue;
            };

            let symbol_provider_library = model_bound_cell
                .map(|binding| {
                    bound_symbol_provider_library(libraries, binding, view, &component.name)
                })
                .transpose()?
                .flatten();
            let selected_library = params
                .get("model_library")
                .map(String::as_str)
                .map(str::trim)
                .filter(|library| !library.is_empty())
                .map(str::to_owned)
                .or(symbol_provider_library);
            let providers = models.definition_providers(scope, &definition);
            if providers.is_empty() {
                if let (Some(binding), Some(selected_library)) =
                    (model_bound_cell, selected_library.as_deref())
                    && selected_library.starts_with("signed-pdk:")
                    && signed_pdk_symbol_binding_matches(
                        technology_binding,
                        packages,
                        selected_library,
                        &definition,
                        binding.source_path.as_deref(),
                    )?
                {
                    continue;
                }
                if model_bound_cell.is_some() || selected_library.is_some() {
                    return Err(PreparationError::new(
                        PreparationStage::ModelBindings,
                        format!(
                            "Instance '{}:{}' declares {} '{}' but its retained catalog provider is unavailable",
                            view,
                            component.name,
                            scope.label(),
                            definition
                        ),
                    ));
                }
                // Engine-native primitive models have no project catalog
                // provider. Their ordinary unresolved-model validation still
                // runs against the completed executable deck below.
                continue;
            }
            let effective = resolutions
                .effective_definition_provider(models, scope, &definition)
                .map_err(|error| {
                    PreparationError::new(
                        PreparationStage::ModelBindings,
                        format!(
                            "Instance '{}:{}' cannot resolve: {error}",
                            view, component.name
                        ),
                    )
                })?
                .expect("a non-empty provider set has one effective provider");
            if let Some(selected_library) = selected_library.as_deref()
                && !effective.library.eq_ignore_ascii_case(selected_library)
            {
                return Err(PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!(
                        "Instance '{}:{}' records model library '{}' but {} '{}' executes from project-global provider '{}'; review and rebind the instance",
                        view,
                        component.name,
                        selected_library,
                        scope.label(),
                        definition,
                        effective.library
                    ),
                ));
            }

            if let Some(binding) = model_bound_cell {
                let source_path = binding.source_path.as_deref().ok_or_else(|| {
                    PreparationError::new(
                        PreparationStage::ModelBindings,
                        format!(
                            "Model-bound instance '{}:{}' has no retained implementation source",
                            view, component.name
                        ),
                    )
                })?;
                let provider = models
                    .get_library(&effective.library)
                    .expect("the effective provider belongs to the live catalog");
                let source_matches = provider
                    .root_path
                    .as_deref()
                    .is_some_and(|path| model_source_paths_match(path, source_path))
                    || provider
                        .source_closure
                        .iter()
                        .any(|pin| model_source_paths_match(&pin.path, source_path));
                if !source_matches {
                    return Err(PreparationError::new(
                        PreparationStage::ModelBindings,
                        format!(
                            "Model-bound instance '{}:{}' retains source '{}' but project-global provider '{}' authenticates a different source; recreate or rebind the symbol",
                            view,
                            component.name,
                            source_path.display(),
                            effective.library
                        ),
                    ));
                }
            }
        }
    }
    Ok(())
}

fn signed_pdk_symbol_binding_matches(
    technology_binding: Option<&ProjectTechnologyBinding>,
    packages: &[ValidatedPdkTechnologyPackage],
    provider_library: &str,
    definition: &str,
    source_path: Option<&Path>,
) -> Result<bool, PreparationError> {
    let Some(source_path) = source_path else {
        return Ok(false);
    };
    let package = project_signed_technology_package(packages, technology_binding)
        .map_err(|error| {
            PreparationError::new(
                PreparationStage::ModelBindings,
                format!("Signed technology symbol authority is unavailable: {error}"),
            )
        })?
        .ok_or_else(|| {
            PreparationError::new(
                PreparationStage::ModelBindings,
                "A signed-PDK symbol is bound but the project has no signed technology package",
            )
        })?;
    Ok(package.symbol_definitions().iter().any(|symbol| {
        symbol.netlist.model.as_ref().is_some_and(|model| {
            model.library.eq_ignore_ascii_case(provider_library)
                && model.model.eq_ignore_ascii_case(definition)
                && model.source_path.as_deref().is_some_and(|expected| {
                    model_source_paths_match(Path::new(expected), source_path)
                })
        })
    }))
}

fn bound_symbol_provider_library(
    libraries: &LibraryCatalog,
    binding: &rspice_design::schematic::component::LibraryCellInstance,
    view: &str,
    instance: &str,
) -> Result<Option<String>, PreparationError> {
    let Some(cell) = libraries
        .get_library(&binding.library)
        .and_then(|library| library.get_cell(&binding.cell))
    else {
        return Ok(None);
    };
    let preferred = cell.get_view(&binding.view).into_iter();
    let remaining = cell
        .views_sorted()
        .into_iter()
        .filter(|candidate| !candidate.name.eq_ignore_ascii_case(&binding.view));
    for candidate in preferred.chain(remaining) {
        let definition = rspice_design::model_bound_symbol::load_model_bound_symbol(candidate).map_err(|error| {
            PreparationError::new(
                PreparationStage::ModelBindings,
                format!(
                    "Model-bound instance '{}:{}' has an invalid retained symbol contract: {error}",
                    view, instance
                ),
            )
        })?;
        if let Some(definition) = definition {
            return Ok(definition
                .netlist
                .model
                .as_ref()
                .map(|model| model.library.clone()));
        }
    }
    Ok(None)
}

fn model_source_paths_match(left: &Path, right: &Path) -> bool {
    if cfg!(windows) {
        left.to_string_lossy()
            .eq_ignore_ascii_case(&right.to_string_lossy())
    } else {
        left == right
    }
}

fn component_value_is_model_name(
    kind: rspice_design::schematic::component_type::ComponentType,
) -> bool {
    use rspice_design::schematic::component_type::ComponentType;

    matches!(
        kind,
        ComponentType::Diode
            | ComponentType::Nmos
            | ComponentType::Pmos
            | ComponentType::NVdmos
            | ComponentType::PVdmos
            | ComponentType::NmosSoi
            | ComponentType::PmosSoi
            | ComponentType::NpnBjt
            | ComponentType::PnpBjt
            | ComponentType::NpnBjt4
            | ComponentType::PnpBjt4
            | ComponentType::NpnBjt5
            | ComponentType::PnpBjt5
            | ComponentType::Njfet
            | ComponentType::Pjfet
            | ComponentType::Nmesfet
            | ComponentType::Pmesfet
    )
}
