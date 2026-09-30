//! Exact project model provenance in the executable source.

use crate::netlist_preparation::validated_executable_hierarchy;
use crate::preparation::{PreparationError, PreparationStage};
use std::collections::{HashMap, HashSet};

pub fn prepared_project_model_sources(
    catalog: &rspice_model_library::ModelCatalog,
    executable_netlist: &str,
) -> Result<Vec<rspice_results::run_receipt::PreparedModelSourceIdentity>, PreparationError> {
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
    let identities = catalog
        .project_model_definition_identities()
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?
        .into_iter()
        .filter(|(_, model_name, _, _)| referenced_names.contains(&model_name.to_ascii_lowercase()))
        .collect::<Vec<_>>();
    let canonical_models = canonical_project_model_definitions(catalog);
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
                catalog,
                source_id,
                &model_name,
                revision,
                content_digest,
            );
            rspice_results::run_receipt::PreparedModelSourceIdentity::new(
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
    catalog: &rspice_model_library::ModelCatalog,
    source_id: rspice_app_types::product::ModelSourceId,
    model_name: &str,
    revision: rspice_app_types::product::ObjectRevision,
    content_digest: rspice_app_types::product::ContentDigest,
) -> rspice_results::run_receipt::PreparedModelQualification {
    use rspice_model_library::ModelSourceAuthority;
    use rspice_results::run_receipt::PreparedModelQualification;

    let Some(library) = catalog.libraries_sorted().into_iter().find(|library| {
        matches!(
            library.source_authority,
            ModelSourceAuthority::ProjectOwned { source_id: owner, .. } if owner == source_id
        )
    }) else {
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
    catalog: &rspice_model_library::ModelCatalog,
) -> Vec<(
    rspice_app_types::product::ModelSourceId,
    String,
    rspice_core::netlist::ModelDef,
)> {
    let mut models = Vec::new();
    for library in catalog
        .libraries_sorted()
        .into_iter()
        .filter(|library| library.source_authority.is_project_owned())
    {
        let rspice_model_library::ModelSourceAuthority::ProjectOwned { source_id, .. } =
            library.source_authority
        else {
            continue;
        };
        for (model_name, model) in &library.models {
            let Some(metadata) = library.model_definition_metadata.get(model_name) else {
                continue;
            };
            let definition = rspice_model_library::ProjectModelRevisionDefinition::new(
                rspice_model_library::ProjectModelDefinition::from_device_model(model),
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

#[cfg(test)]
mod tests;
