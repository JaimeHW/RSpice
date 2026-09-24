//! Project-owned model provenance in a prepared executable deck.
//!
//! Correlates referenced executable cards with one exact authored source and
//! its immutable qualification release before recording run evidence.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use super::{AppState, PreparationError, PreparationStage, validated_executable_hierarchy};

/// Revalidate every model-bearing instance in the frozen hierarchy against
/// the project-global provider decision. Editor properties are not an
/// execution authority: restored projects and older symbol revisions must
/// pass this boundary immediately before their sources are sealed.
pub(super) fn validate_projected_model_binding_authority(
    state: &AppState,
    projection: &crate::state::workspace::ConfigurationExecutionProjection,
) -> Result<(), PreparationError> {
    use crate::state::model_library::ModelConsumerScope;

    for (view, schematic) in projection.schematic_buffers() {
        for component in &schematic.components {
            let params = crate::state::parse_params_string(&component.params);
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
                .map(|binding| bound_symbol_provider_library(state, binding, view, &component.name))
                .transpose()?
                .flatten();
            let selected_library = params
                .get("model_library")
                .map(String::as_str)
                .map(str::trim)
                .filter(|library| !library.is_empty())
                .map(str::to_owned)
                .or(symbol_provider_library);
            let providers = state
                .model_library_manager
                .definition_providers(scope, &definition);
            if providers.is_empty() {
                if let (Some(binding), Some(selected_library)) =
                    (model_bound_cell, selected_library.as_deref())
                    && selected_library.starts_with("signed-pdk:")
                    && signed_pdk_symbol_binding_matches(
                        state,
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
            let effective = state
                .model_library_manager
                .effective_definition_provider(scope, &definition)
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
                let provider = state
                    .model_library_manager
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
    state: &AppState,
    provider_library: &str,
    definition: &str,
    source_path: Option<&Path>,
) -> Result<bool, PreparationError> {
    let Some(source_path) = source_path else {
        return Ok(false);
    };
    let package = state
        .project_signed_technology_package()
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
    state: &AppState,
    binding: &crate::state::LibraryCellInstance,
    view: &str,
    instance: &str,
) -> Result<Option<String>, PreparationError> {
    let Some(cell) = state
        .library_manager
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
        let definition = crate::state::ModelBoundSymbolDefinition::load_from_view(candidate)
            .map_err(|error| {
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

fn component_value_is_model_name(kind: crate::state::ComponentType) -> bool {
    use crate::state::ComponentType;

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

pub(super) fn prepared_project_model_sources(
    state: &AppState,
    executable_netlist: &str,
) -> Result<Vec<crate::state::PreparedModelSourceIdentity>, PreparationError> {
    let (parsed, flattened) = validated_executable_hierarchy(executable_netlist)?;
    let referenced_names = flattened
        .elements
        .iter()
        .filter_map(element_model_name)
        .map(str::to_ascii_lowercase)
        .collect::<HashSet<_>>();
    let executable_models = parsed
        .models
        .iter()
        .chain(flattened.scoped_models.iter())
        .collect::<Vec<_>>();
    let identities = state
        .model_library_manager
        .project_model_definition_identities()
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?
        .into_iter()
        .filter(|(_, model_name, _, _)| referenced_names.contains(&model_name.to_ascii_lowercase()))
        .collect::<Vec<_>>();
    let canonical_models = canonical_project_model_definitions(state);
    let mut name_counts = HashMap::<String, usize>::new();
    for (_, model_name, _, _) in &identities {
        *name_counts
            .entry(model_name.to_ascii_lowercase())
            .or_default() += 1;
    }
    identities
        .into_iter()
        // A repeated semantic name cannot be traced back to one exact project
        // source from the executable SPICE instance. Omit every ambiguous
        // candidate so simulation remains available while correlation
        // evidence fails closed.
        .filter(|(_, model_name, _, _)| {
            name_counts.get(&model_name.to_ascii_lowercase()).copied() == Some(1)
        })
        .filter(|(source_id, model_name, _, _)| {
            let mut executable_matches = executable_models
                .iter()
                .copied()
                .filter(|model| model.name.eq_ignore_ascii_case(model_name));
            let Some(executable_model) = executable_matches.next() else {
                return false;
            };
            if executable_matches.next().is_some() {
                return false;
            }

            let mut canonical_matches =
                canonical_models
                    .iter()
                    .filter(|(candidate_source_id, candidate_name, _)| {
                        candidate_source_id == source_id
                            && candidate_name.eq_ignore_ascii_case(model_name)
                    });
            let Some((_, _, canonical_model)) = canonical_matches.next() else {
                return false;
            };
            canonical_matches.next().is_none()
                && model_definitions_match(canonical_model, executable_model)
        })
        .map(|(source_id, model_name, revision, content_digest)| {
            let qualification = model_qualification_at_preparation(
                state,
                source_id,
                &model_name,
                revision,
                content_digest,
            );
            crate::state::PreparedModelSourceIdentity::new(
                source_id,
                model_name,
                revision,
                content_digest,
                qualification,
            )
            .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))
        })
        .collect()
}

/// Whether an immutable release covers this exact model revision right now.
///
/// Matched on the exact source digest and revision, not on the model name: a
/// release authorizes the bytes it was measured against, so a model edited
/// after it was released is a different model and is not covered by it. That is
/// the whole value of recording the answer per run — the qualification state is
/// editable and the run is not.
fn model_qualification_at_preparation(
    state: &AppState,
    source_id: crate::product::ModelSourceId,
    model_name: &str,
    revision: crate::product::ObjectRevision,
    content_digest: crate::product::ContentDigest,
) -> crate::state::PreparedModelQualification {
    use crate::state::PreparedModelQualification;
    use crate::state::model_library::ModelSourceAuthority;

    let Some(library) = state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .find(|library| {
            matches!(
                library.source_authority,
                ModelSourceAuthority::ProjectOwned { source_id: owner, .. } if owner == source_id
            )
        })
    else {
        return PreparedModelQualification::Unqualified;
    };

    let released = library
        .model_qualification
        .iter()
        .filter(|(name, _)| name.eq_ignore_ascii_case(model_name))
        .any(|(_, qualification)| {
            qualification.releases.iter().any(|release| {
                release.source.source_id == Some(source_id)
                    && release.source.source_digest == content_digest
                    && release.source.source_revision == revision
            })
        });

    if released {
        PreparedModelQualification::Released
    } else {
        PreparedModelQualification::Unqualified
    }
}

fn canonical_project_model_definitions(
    state: &AppState,
) -> Vec<(
    crate::product::ModelSourceId,
    String,
    rspice_core::netlist::ModelDef,
)> {
    let mut models = Vec::new();
    for library in state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .filter(|library| library.source_authority.is_project_owned())
    {
        let crate::state::model_library::ModelSourceAuthority::ProjectOwned { source_id, .. } =
            library.source_authority
        else {
            continue;
        };
        for (model_name, model) in &library.models {
            let Some(metadata) = library.model_definition_metadata.get(model_name) else {
                continue;
            };
            let definition = crate::state::model_library::ProjectModelRevisionDefinition::new(
                crate::state::model_library::ProjectModelDefinition::from_device_model(model),
                metadata.clone(),
            );
            let Ok(source) = definition.qualification_model_source(None) else {
                continue;
            };
            let deck = format!("Authenticated project model\n{source}.end\n");
            let Ok(parsed) = rspice_core::netlist::parse_netlist(&deck) else {
                continue;
            };
            let mut matching = parsed
                .models
                .into_iter()
                .filter(|candidate| candidate.name.eq_ignore_ascii_case(model_name));
            let Some(canonical_model) = matching.next() else {
                continue;
            };
            if matching.next().is_none() {
                models.push((source_id, model_name.clone(), canonical_model));
            }
        }
    }
    models
}

fn model_definitions_match(
    canonical: &rspice_core::netlist::ModelDef,
    executable: &rspice_core::netlist::ModelDef,
) -> bool {
    canonical.name.eq_ignore_ascii_case(&executable.name)
        && canonical
            .model_type
            .eq_ignore_ascii_case(&executable.model_type)
        && named_model_fields_match(&canonical.params, &executable.params)
        && named_model_fields_match(&canonical.expr_params, &executable.expr_params)
        && named_model_fields_match(&canonical.string_params, &executable.string_params)
        && named_model_fields_match(
            &canonical.string_vector_params,
            &executable.string_vector_params,
        )
        && named_model_fields_match(
            &canonical.real_vector_params,
            &executable.real_vector_params,
        )
        && named_model_fields_match(
            &canonical.real_vector_expr_params,
            &executable.real_vector_expr_params,
        )
        && named_model_fields_match(
            &canonical.integer_vector_params,
            &executable.integer_vector_params,
        )
}

fn named_model_fields_match<T: PartialEq>(
    canonical: &[(String, T)],
    executable: &[(String, T)],
) -> bool {
    fn normalized<T>(fields: &[(String, T)]) -> Option<HashMap<String, &T>> {
        let mut normalized = HashMap::with_capacity(fields.len());
        for (name, value) in fields {
            if normalized
                .insert(name.to_ascii_lowercase(), value)
                .is_some()
            {
                return None;
            }
        }
        Some(normalized)
    }

    normalized(canonical)
        .zip(normalized(executable))
        .is_some_and(|(canonical, executable)| canonical == executable)
}

fn element_model_name(element: &rspice_core::netlist::Element) -> Option<&str> {
    use rspice_core::netlist::ElementKind as Kind;
    match &element.kind {
        Kind::Resistor { model, .. }
        | Kind::Capacitor { model, .. }
        | Kind::Inductor { model, .. }
        | Kind::TransmissionLine { model, .. } => model.as_deref(),
        Kind::JilesAthertonInductor { model, .. }
        | Kind::Diode { model, .. }
        | Kind::Bjt { model, .. }
        | Kind::Mosfet { model, .. }
        | Kind::Jfet { model, .. }
        | Kind::Mesfet { model, .. }
        | Kind::XyceMemristor { model, .. }
        | Kind::VSwitch { model, .. }
        | Kind::ISwitch { model, .. }
        | Kind::GenericSwitch { model, .. }
        | Kind::Xspice { model, .. } => Some(model),
        _ => None,
    }
}
