//! Sealing host dependencies and validating resolved source waveforms.
//!
//! This is the only place a manual deck's `.include`, `.inc` and `.lib`
//! directives are resolved against the host filesystem before a run, so it is
//! also the only place the project's ordered include search chain has to be
//! applied. It lives beside the preparation it serves rather than inside it
//! because that module is already over the file budget.

use std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
use rspice_core::netlist::IncludeProcessor;

use super::{absolute_source_identity, path_identity};
use crate::simulation::execution::{PreparationError, PreparationStage};

pub(super) fn expand_manual_dependencies(
    source: &str,
    origin: Option<&Path>,
    include_search: &crate::state::IncludeSearchChain,
    sealed_sources: &crate::state::model_library::SealedModelExecutionSources,
) -> Result<
    (
        String,
        Option<String>,
        Vec<rspice_core::netlist::ResolvedIncludeDependency>,
    ),
    PreparationError,
> {
    let Some(origin) = origin else {
        return Ok((source.to_owned(), None, Vec::new()));
    };
    let absolute_origin = absolute_source_identity(origin)?;

    #[cfg(target_arch = "wasm32")]
    {
        // The browser resolves only through the authenticated bundle, which
        // carries its own edges; a host directory chain has nothing to add.
        let _ = include_search;
        let (expanded, dependencies) = sealed_sources
            .expand_root_dependencies(
                &absolute_origin,
                source,
                &rspice_core::abort_signal::NoAbort,
            )
            .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))?;
        Ok((
            expanded,
            Some(path_identity(&absolute_origin)),
            dependencies,
        ))
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = sealed_sources;
        let mut processor = IncludeProcessor::new(&absolute_origin);
        include_search.apply_to(&mut processor);
        let expanded = processor
            .expand_content(source, &absolute_origin)
            .map_err(|error| {
                PreparationError::new(
                    PreparationStage::SourceChecks,
                    format!("Could not seal manual deck dependencies: {error}"),
                )
            })?;
        Ok((
            expanded,
            Some(path_identity(&absolute_origin)),
            processor.resolved_dependencies().to_vec(),
        ))
    }
}

/// Resolve the same waveform grammar and instance parameters the engine uses
/// before any dispatch proof is built. File-backed waveforms require sealing;
/// their node or parameter names cannot identify a dependency by themselves.
pub(super) fn validated_executable_hierarchy(
    executable_netlist: &str,
) -> Result<(rspice_core::Netlist, rspice_core::netlist::FlattenedNetlist), PreparationError> {
    let parsed = rspice_core::netlist::parse_netlist(executable_netlist).map_err(|error| {
        PreparationError::new(
            PreparationStage::ModelBindings,
            format!("Executable source cannot authenticate project model use: {error}"),
        )
    })?;
    let flattened =
        rspice_core::netlist::flatten_netlist_with_models(&parsed).map_err(|error| {
            PreparationError::new(
                PreparationStage::ModelBindings,
                format!("Executable hierarchy cannot authenticate project model use: {error}"),
            )
        })?;
    for element in &flattened.elements {
        let dependency = rspice_core::netlist::independent_source_file_dependency(&element.kind)
            .map_err(|error| {
                PreparationError::new(PreparationStage::SourceChecks, error.to_string())
            })?;
        if let Some(path) = dependency {
            return Err(PreparationError::new(
                PreparationStage::SourceChecks,
                format!(
                    "Executable netlist contains an unsealed external dependency (file-backed PWL source) on element '{}': {path}",
                    element.name
                ),
            ));
        }
    }
    Ok((parsed, flattened))
}
