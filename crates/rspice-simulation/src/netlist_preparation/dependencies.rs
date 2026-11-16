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

/// `source` with each include directive it writes itself naming, by absolute
/// path, the file expanding it read, so the deck reads the same files from
/// any folder and without the search chain that found them.
/// `dependencies` are what expanding `source` resolved. A directive inside an
/// included file resolves against that file, so it is left as written.
#[cfg(not(target_arch = "wasm32"))]
pub fn name_resolved_includes(
    source: &str,
    dependencies: &[rspice_core::netlist::ResolvedIncludeDependency],
) -> String {
    let included = dependencies
        .iter()
        .map(rspice_core::netlist::ResolvedIncludeDependency::resolved_path)
        .collect::<std::collections::HashSet<_>>();
    let own = dependencies
        .iter()
        .filter(|dependency| !included.contains(dependency.owner_path()))
        .map(|dependency| (dependency.directive_line(), dependency))
        .collect::<std::collections::HashMap<_, _>>();
    if own.is_empty() {
        return source.to_owned();
    }
    let mut named = String::with_capacity(source.len());
    for (index, line) in source.split_inclusive('\n').enumerate() {
        let Some(dependency) = own.get(&(index + 1)) else {
            named.push_str(line);
            continue;
        };
        // The path the search chose, before links resolve; a search entry
        // that is itself relative leaves only the canonical path absolute.
        let chosen = dependency.resolution().resolved();
        let path = if chosen.is_absolute() {
            chosen
        } else {
            dependency.resolved_path()
        };
        let quoted = format!("\"{}\"", path.to_string_lossy().replace('"', "\\\""));
        // The path follows the directive's keyword.
        let keyword = line.split_whitespace().next().unwrap_or("");
        let after = line.find(keyword).map_or(0, |start| start + keyword.len());
        let (head, rest) = line.split_at(after);
        let requested = dependency.requested_path();
        let token = [
            format!("\"{requested}\""),
            format!("'{requested}'"),
            requested.to_owned(),
        ]
        .into_iter()
        .find(|token| rest.contains(token.as_str()));
        named.push_str(head);
        match token {
            Some(token) => named.push_str(&rest.replacen(&token, &quoted, 1)),
            None => named.push_str(rest),
        }
    }
    named
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

/// A manual deck expanded through its include chain.
pub struct ExpandedManualDependencies {
    pub expanded: String,
    /// The deck's own path as the host names it, when it has one.
    pub canonical_origin: Option<String>,
    pub dependencies: Vec<rspice_core::netlist::ResolvedIncludeDependency>,
    /// The file and line that wrote each expanded line, when the expansion
    /// included a file.
    pub source_lines: Option<std::sync::Arc<rspice_core::netlist::ExpandedLineMap>>,
}

/// Expand the manual deck `source` saved at `origin` through its include
/// chain. `rewrite` names the line each written line becomes, by the file and
/// line that wrote it: a sealed Verilog-A source line an included file
/// writes. The expansion's line map comes back when it included a file.
pub fn expand_manual_dependencies(
    source: &str,
    origin: Option<&Path>,
    include_search: &IncludeSearchChain,
    sealed_sources: &SealedModelExecutionSources,
    rewrite: &dyn Fn(&rspice_core::netlist::NetlistSourceLocation) -> Option<String>,
) -> Result<ExpandedManualDependencies, PreparationError> {
    let Some(origin) = origin else {
        return Ok(ExpandedManualDependencies {
            expanded: source.to_owned(),
            canonical_origin: None,
            dependencies: Vec::new(),
            source_lines: None,
        });
    };
    let absolute_origin = absolute_source_identity(origin)?;

    #[cfg(target_arch = "wasm32")]
    {
        // The browser resolves only through the authenticated bundle, which
        // carries its own edges; a host directory chain has nothing to add,
        // and nothing read from the host was sealed to rewrite.
        let _ = (include_search, rewrite);
        let (expanded, lines, dependencies) = sealed_sources
            .expand_root_dependencies_with_line_map(
                &absolute_origin,
                source,
                &rspice_core::abort_signal::NoAbort,
            )
            .map_err(|error| PreparationError::new(PreparationStage::SourceChecks, error))?;
        Ok(ExpandedManualDependencies {
            expanded,
            canonical_origin: Some(path_identity(&absolute_origin)),
            dependencies,
            source_lines: crate::execution::included_line_map(lines),
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = sealed_sources;
        let mut processor = IncludeProcessor::new(&absolute_origin);
        include_search.apply_to(&mut processor);
        let (expanded, lines) = processor
            .expand_content_rewriting_with_line_map_and_abort(
                source,
                &absolute_origin,
                &mut |location, _| rewrite(location),
                &rspice_core::abort_signal::NoAbort,
            )
            .map_err(|error| {
                PreparationError::new(
                    PreparationStage::SourceChecks,
                    format!("Could not seal manual deck dependencies: {error}"),
                )
            })?;
        Ok(ExpandedManualDependencies {
            expanded,
            canonical_origin: Some(path_identity(&absolute_origin)),
            dependencies: processor.resolved_dependencies().to_vec(),
            source_lines: crate::execution::included_line_map(lines),
        })
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    /// A deck whose library the run found beside its schematic and through a
    /// library folder names both by absolute path, reads the same models
    /// from another folder with no search chain, and leaves the include the
    /// library itself writes as written.
    #[test]
    fn a_deck_names_the_files_its_includes_resolved_to() {
        let root =
            std::env::temp_dir().join(format!("rspice_named_includes_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let (project, library, elsewhere) = (
            root.join("project"),
            root.join("lib"),
            root.join("elsewhere"),
        );
        for folder in [&project, &library, &elsewhere] {
            std::fs::create_dir_all(folder).unwrap();
        }
        std::fs::write(
            project.join("device.lib"),
            ".lib tt\n.include \"inner.inc\"\n.endl tt\n.lib ff\nRff a 0 4k\n.endl ff\n",
        )
        .unwrap();
        std::fs::write(project.join("inner.inc"), "Rtt a 0 9k\n").unwrap();
        std::fs::write(library.join("load.inc"), "Rload a 0 1k\n").unwrap();
        let deck = "deck\n.lib \"device.lib\" tt\n.include load.inc\nV1 a 0 1\n.op\n.end\n";
        let search = crate::netlist_preparation::IncludeSearchChain::resolve(
            std::slice::from_ref(&library),
            None,
        );
        let (expanded, dependencies) = expand_generated_dependencies_with_sealed_sources(
            deck,
            Some(&project.join("top.rsch")),
            &search,
            None,
        )
        .unwrap();

        let named = name_resolved_includes(deck, &dependencies);

        let quoted = |path: std::path::PathBuf| format!("\"{}\"", path.display());
        assert_eq!(
            named,
            format!(
                "deck\n.lib {} tt\n.include {}\nV1 a 0 1\n.op\n.end\n",
                quoted(project.join("device.lib")),
                quoted(library.join("load.inc"))
            )
        );
        let (moved, _) = expand_generated_dependencies_with_sealed_sources(
            &named,
            Some(&elsewhere.join("deck.cir")),
            &crate::netlist_preparation::IncludeSearchChain::default(),
            None,
        )
        .unwrap();
        assert_eq!(moved, expanded);
        assert!(
            moved.contains("Rtt a 0 9k") && moved.contains("Rload a 0 1k"),
            "{moved}"
        );
        let _ = std::fs::remove_dir_all(&root);
    }
}
