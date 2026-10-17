//! Resolve and seal authored and generated source dependencies for the execution host.

use super::{IncludeSearchChain, contains_external_include_directive};
use crate::model_sources::SealedModelExecutionSources;
use crate::preparation::{PreparationError, PreparationStage};
#[cfg(not(target_arch = "wasm32"))]
use rspice_core::netlist::IncludeProcessor;
use std::path::{Path, PathBuf};

pub fn expand_generated_dependencies(
    source: &str,
    origin: Option<&Path>,
    include_search: &IncludeSearchChain,
    catalog: &rspice_model_library::ModelCatalog,
    resolution_records: &rspice_model_library::ModelResolutionRecords,
) -> Result<(String, Vec<rspice_core::netlist::ResolvedIncludeDependency>), PreparationError> {
    #[cfg(target_arch = "wasm32")]
    let sealed = crate::model_sources::seal_catalog_execution_sources(catalog, resolution_records)
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (catalog, resolution_records);

    expand_generated_dependencies_with_sealed_sources(source, origin, include_search, {
        #[cfg(target_arch = "wasm32")]
        {
            Some(&sealed)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            None
        }
    })
}

pub fn expand_generated_dependencies_with_sealed_sources(
    source: &str,
    origin: Option<&Path>,
    include_search: &IncludeSearchChain,
    sealed_sources: Option<&SealedModelExecutionSources>,
) -> Result<(String, Vec<rspice_core::netlist::ResolvedIncludeDependency>), PreparationError> {
    if !contains_external_include_directive(source) {
        return Ok((source.to_owned(), Vec::new()));
    }

    #[cfg(target_arch = "wasm32")]
    {
        // The browser resolves only through the authenticated bundle.
        let _ = include_search;
        let origin = origin.ok_or_else(|| {
            PreparationError::new(
                PreparationStage::SourceChecks,
                "Configured external SPICE sources require an imported root identity before browser execution",
            )
        })?;
        let origin = absolute_source_identity(origin)?;
        let sealed_sources = sealed_sources.ok_or_else(|| {
            PreparationError::new(
                PreparationStage::ModelBindings,
                "Configured external SPICE sources have no authenticated browser source bundle",
            )
        })?;
        sealed_sources
            .expand_root_dependencies(&origin, source, &rspice_core::abort_signal::NoAbort)
            .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = sealed_sources;
        let owner = match origin {
            Some(path) => absolute_source_identity(path)?,
            None => execution_current_directory()?.join("__rspice_generated_source__.cir"),
        };
        let mut processor = IncludeProcessor::new(&owner);
        include_search.apply_to(&mut processor);
        let expanded = processor.expand_content(source, &owner).map_err(|error| {
            PreparationError::new(
                PreparationStage::SourceChecks,
                format!("Could not seal configured source dependencies: {error}"),
            )
        })?;
        Ok((expanded, processor.resolved_dependencies().to_vec()))
    }
}

/// Refuse unless every host source `dependencies` names still reads as it did.
///
/// Expanding a deck's includes reads the host: each directive walks its
/// search chain, takes the first candidate that exists, and splices in what
/// that file decodes to. A deck prepared earlier is the deck the host gives
/// now only when every candidate a chain tried still exists or still does not,
/// and every file a chain chose still decodes to the text that was spliced in.
/// That is what this checks, file by file, without expanding anything. A
/// dependency resolved through an authenticated in-memory edge examined no
/// host path and is left alone.
pub fn verify_host_dependencies(
    dependencies: &[rspice_core::netlist::ResolvedIncludeDependency],
) -> Result<(), String> {
    use rspice_core::netlist::IncludeSearchStage;

    for dependency in dependencies {
        let resolution = dependency.resolution();
        if resolution.stage() == IncludeSearchStage::SealedEdge {
            continue;
        }
        for candidate in resolution.tried() {
            if candidate.path().exists() != candidate.exists() {
                return Err(format!(
                    "'{}' {} since '{}' was resolved",
                    candidate.path().display(),
                    if candidate.exists() {
                        "was removed"
                    } else {
                        "appeared"
                    },
                    dependency.requested_path()
                ));
            }
        }
        let path = dependency.resolved_path();
        let unchanged = std::fs::read(path)
            .ok()
            .and_then(|bytes| rspice_core::netlist::decode_source_bytes(&bytes).ok())
            .is_some_and(|source| source == dependency.source());
        if !unchanged {
            return Err(format!("'{}' changed since it was read", path.display()));
        }
    }
    Ok(())
}

/// The host's identity for a deck's file: what a manual deck's receipt names
/// and its includes are searched from, as the host resolves it now.
pub fn absolute_source_identity(path: &Path) -> Result<PathBuf, PreparationError> {
    #[cfg(target_arch = "wasm32")]
    if rspice_model_library::is_portable_absolute_path(path) {
        return Ok(path.to_path_buf());
    }
    if path.is_absolute() {
        return Ok(path.canonicalize().unwrap_or_else(|_| path.to_path_buf()));
    }
    let current = std::env::current_dir().map_err(|error| {
        PreparationError::new(
            PreparationStage::SourceChecks,
            format!("Could not resolve manual deck origin: {error}"),
        )
    })?;
    let joined = current.join(path);
    Ok(joined.canonicalize().unwrap_or(joined))
}

fn path_identity(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

pub fn execution_current_directory() -> Result<PathBuf, PreparationError> {
    match std::env::current_dir() {
        Ok(path) => Ok(path),
        #[cfg(target_arch = "wasm32")]
        Err(_) => Ok(PathBuf::from(".")),
        #[cfg(not(target_arch = "wasm32"))]
        Err(error) => Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!("Could not resolve the automatic export directory: {error}"),
        )),
    }
}

pub fn expand_manual_dependencies(
    source: &str,
    origin: Option<&Path>,
    include_search: &IncludeSearchChain,
    sealed_sources: &SealedModelExecutionSources,
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
