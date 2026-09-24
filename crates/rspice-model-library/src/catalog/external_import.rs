//! Validating captured external sources and publishing complete catalog imports.
//!
//! The caller acquires files and preprocesses HDL; this boundary owns source
//! consistency, dependency edges, content pins, limits and atomic publication.

use super::ModelCatalog;
use crate::source_bundle::ImportLimits;
use crate::{
    ModelLibrary, ModelSourceAuthority, ModelSourceContent, ModelSourceEdge, ModelSourcePin,
    first_unreachable_source,
};
use rspice_app_types::product::ContentDigest;
use rspice_core::library::{LibParseResult, ResolvedLibDependency, ResolvedLibSource};
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeSet, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Provider-owned HDL documents and edges returned by the host preprocessor.
/// Compiler built-in headers are omitted from `sources`.
pub struct CapturedHdlSources {
    pub sources: Vec<(PathBuf, String)>,
    pub dependencies: Vec<ResolvedLibDependency>,
}

impl ModelCatalog {
    /// Validate one captured parse and publish it only after all source and
    /// projection checks succeed. The HDL callback acquires each discovered root.
    pub fn import_external_library(
        &mut self,
        path: &Path,
        mut result: LibParseResult,
        section: Option<&str>,
        limits: ImportLimits,
        mut capture_hdl: impl FnMut(&Path) -> Result<CapturedHdlSources, String>,
    ) -> Result<String, String> {
        let lib_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unnamed")
            .to_string();

        if !result.is_ok() {
            return Err(format!(
                "Model library '{}' contains parse or dependency errors: {}",
                path.display(),
                result
                    .errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
            ));
        }
        extend_hdl_closure(&mut result, &mut capture_hdl)?;
        result
            .resolved_sources
            .sort_by(|left, right| left.path.cmp(&right.path));
        result.resolved_dependencies.sort();
        result.resolved_dependencies.dedup();
        let total_bytes = result
            .resolved_sources
            .iter()
            .try_fold(0usize, |total, source| {
                total.checked_add(source.bytes.len())
            })
            .ok_or_else(|| "Model source closure size overflowed".to_owned())?;
        if result.resolved_sources.len() > limits.max_files || total_bytes > limits.max_total_bytes
        {
            return Err(format!(
                "Model source closure including Verilog-A dependencies exceeds the project limit ({} files / {} bytes)",
                limits.max_files, limits.max_total_bytes
            ));
        }
        let mut source_closure = result
            .resolved_sources
            .iter()
            .map(|source| ModelSourcePin {
                path: source.path.clone(),
                digest: ContentDigest::from_bytes(Sha256::digest(source.bytes.as_ref()).into()),
            })
            .collect::<Vec<_>>();
        source_closure.sort_by(|left, right| left.path.cmp(&right.path));
        let mut source_contents = result
            .resolved_sources
            .iter()
            .map(|source| ModelSourceContent {
                path: source.path.clone(),
                bytes: source.bytes.as_ref().to_vec(),
            })
            .collect::<Vec<_>>();
        source_contents.sort_by(|left, right| left.path.cmp(&right.path));
        for content in &source_contents {
            let source =
                rspice_core::netlist::decode_source_bytes(&content.bytes).map_err(|error| {
                    format!(
                        "Model source '{}' cannot be decoded for dialect validation: {error}",
                        content.path.display()
                    )
                })?;
            Self::validate_model_source_dialect(&content.path, &source)?;
        }
        if source_closure.is_empty() {
            return Err(format!(
                "Model library '{}' produced an empty source dependency closure",
                path.display()
            ));
        }
        let mut source_edges = result
            .resolved_dependencies
            .iter()
            .map(|edge| ModelSourceEdge {
                owner: edge.owner.clone(),
                requested_path: edge.requested_path.clone(),
                target: edge.target.clone(),
            })
            .collect::<Vec<_>>();
        source_edges.sort();
        source_edges.dedup();
        if let Some(unreachable) = first_unreachable_source(path, &source_closure, &source_edges) {
            return Err(format!(
                "Model library '{}' captured dependency '{}' that is not reachable from its root by authenticated resolution edges",
                path.display(),
                unreachable.display()
            ));
        }

        if let Some(existing) = self.get_library(&lib_name)
            && existing.root_path.as_deref() != Some(path)
        {
            return Err(format!(
                "Cannot load '{}': library name '{}' is already owned by a different model source",
                path.display(),
                lib_name
            ));
        }

        // Build a complete replacement and publish it only after every parse
        // and section check succeeds. A failed refresh never leaves a partly
        // updated model catalog behind.
        let mut library = self
            .get_library(&lib_name)
            .cloned()
            .unwrap_or_else(|| ModelLibrary::new(&lib_name));
        library.root_path = Some(path.to_path_buf());
        library.source_authority = ModelSourceAuthority::External;
        library.source_closure = source_closure;
        library.source_contents = source_contents;
        library.source_edges = source_edges;
        let library =
            library.with_parsed_catalog(&result, path, section, &path.display().to_string())?;

        self.add_library(library);
        Ok(lib_name)
    }

    /// Enforce the model-library dialect boundary before any parsed
    /// projection is accepted. `.scs` sources admit the explicit
    /// `simulator lang=spice` interoperability profile and the fail-closed
    /// declarative Spectre model-library subset implemented by the core
    /// adapter. Unsupported native statements are errors, never discarded.
    pub fn validate_model_source_dialect(path: &Path, source: &str) -> Result<(), String> {
        rspice_core::library::adapt_spectre_model_library(path, source)
            .map(|_| ())
            .map_err(|error| {
                format!(
                    "{}:{} cannot be imported as an executable model library: {}",
                    path.display(),
                    error.line,
                    error.message
                )
            })
    }
}

fn extend_hdl_closure(
    result: &mut LibParseResult,
    capture_hdl: &mut impl FnMut(&Path) -> Result<CapturedHdlSources, String>,
) -> Result<(), String> {
    let mut roots = BTreeSet::<PathBuf>::new();
    for resolved in &result.resolved_sources {
        let projected =
            rspice_core::library::adapt_spectre_model_library(&resolved.path, &resolved.content)
                .map_err(|error| {
                    format!(
                        "{}:{} cannot authenticate AHDL dependencies: {}",
                        resolved.path.display(),
                        error.line,
                        error.message
                    )
                })?;
        for line in projected.lines() {
            let Some(include) = rspice_core::netlist::parse_veriloga_source_directive(line) else {
                continue;
            };
            let requested = rspice_core::netlist::normalize_source_path_literal(
                &include.file_path.to_string_lossy(),
            )
            .map_err(|error| {
                format!(
                    "{} has an invalid Verilog-A dependency: {error}",
                    resolved.path.display()
                )
            })?;
            let matches = result
                .resolved_dependencies
                .iter()
                .filter(|dependency| {
                    dependency.owner == resolved.path
                        && rspice_core::netlist::normalize_source_path_literal(
                            &dependency.requested_path,
                        )
                        .is_ok_and(|candidate| candidate == requested)
                })
                .collect::<Vec<_>>();
            let [dependency] = matches.as_slice() else {
                return Err(format!(
                    "{} Verilog-A dependency '{}' has {} resolution edges",
                    resolved.path.display(),
                    requested,
                    matches.len()
                ));
            };
            roots.insert(dependency.target.clone());
        }
    }

    for root in roots {
        let captured = capture_hdl(&root)?;
        let provider_paths = captured
            .sources
            .iter()
            .map(|(path, _)| path.clone())
            .collect::<HashSet<_>>();
        for (path, source) in captured.sources {
            if let Some(existing) = result
                .resolved_sources
                .iter()
                .find(|existing| existing.path == path)
            {
                if existing.content.as_ref() != source.as_str() {
                    return Err(format!(
                        "Verilog-A dependency '{}' changed while its closure was captured",
                        path.display()
                    ));
                }
                continue;
            }
            result.resolved_sources.push(ResolvedLibSource {
                path,
                bytes: Arc::from(source.as_bytes()),
                content: Arc::from(source),
            });
        }
        for dependency in captured.dependencies {
            if !provider_paths.contains(&dependency.target) {
                continue;
            }
            if let Some(existing) = result.resolved_dependencies.iter().find(|existing| {
                existing.owner == dependency.owner
                    && existing.requested_path == dependency.requested_path
            }) {
                if existing.target != dependency.target {
                    return Err(format!(
                        "Verilog-A dependency '{}' in '{}' resolved inconsistently",
                        dependency.requested_path,
                        dependency.owner.display()
                    ));
                }
            } else {
                result.resolved_dependencies.push(dependency);
            }
        }
    }
    Ok(())
}
