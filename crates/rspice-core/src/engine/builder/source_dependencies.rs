//! Source discovery for hosts that must reserve model inputs before publishing.
use super::*;

impl Engine {
    /// Discover active Verilog-A/AMS source files, including nested headers,
    /// using the same compiler options as circuit construction. This performs
    /// bounded preprocessing, without selecting, compiling, or executing a
    /// module. Paths are sorted and deduplicated; built-in headers and registered
    /// virtual sources have no filesystem dependency and are excluded.
    ///
    /// The result describes this read of the source tree, not an immutable
    /// snapshot of a later circuit build. Callers must keep their inputs stable.
    pub fn veriloga_source_dependencies_with_abort(
        &self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<PathBuf>, SimulationError> {
        self.ensure_valid_configuration()?;
        check_build_abort(abort)?;
        #[cfg(not(target_arch = "wasm32"))]
        let mut paths = BTreeSet::new();
        #[cfg(target_arch = "wasm32")]
        let paths = BTreeSet::<PathBuf>::new();
        let mut roots = HashSet::new();
        for include in &netlist.veriloga_includes {
            check_build_abort(abort)?;
            let path = &include.file_path;
            if veriloga_cache::is_sealed_veriloga_virtual_path(path)
                || !roots.insert(veriloga_cache::canonicalize_for_cache(path))
            {
                continue;
            }
            #[cfg(target_arch = "wasm32")]
            return Err(crate::ElaborationError::new(
                crate::ElaborationErrorKind::MissingSource,
                "filesystem source discovery is unavailable in the browser; register virtual sources",
            )
            .in_source(path)
            .into());
            #[cfg(not(target_arch = "wasm32"))]
            {
                use rspice_veriloga::preprocessor::{
                    BoundedFileSystemSourceProvider, SourceResource,
                };
                let limits = self.config.resource_limits;
                let control = veriloga_cache::VerilogACompileControl { abort };
                let provider = BoundedFileSystemSourceProvider::new(
                    rspice_veriloga::SourceProviderLimits {
                        max_dependencies: usize::MAX,
                        max_total_source_bytes: limits.max_dependency_source_bytes,
                        max_include_depth: limits.max_include_depth,
                        max_expanded_bytes: limits.max_expanded_source_bytes,
                    },
                    limits.max_dependency_source_bytes,
                    usize::MAX,
                    &control,
                );
                let compiler = rspice_veriloga::VerilogACompiler::new(
                    veriloga_cache::deck_include_compiler_options(),
                );
                let dependencies = compiler
                    .provider_source_dependencies(&provider, path)
                    .map_err(|error| {
                        if error.cancelled {
                            SimulationError::Aborted
                        } else if let Some(limit) = error.resource_limit {
                            SimulationError::ResourceLimit(ResourceLimitError {
                                resource: match limit.resource {
                                    SourceResource::IncludeDepth => ResourceKind::IncludeDepth,
                                    SourceResource::ExpandedBytes => {
                                        ResourceKind::ExpandedSourceBytes
                                    }
                                    SourceResource::RootLines => ResourceKind::NetlistLines,
                                    _ => ResourceKind::DependencySourceBytes,
                                },
                                requested: limit.requested,
                                limit: limit.limit,
                            })
                        } else {
                            crate::ElaborationError::new(
                                crate::ElaborationErrorKind::CompileRefusal,
                                format!("source dependency discovery failed: {error}"),
                            )
                            .in_source(path)
                            .into()
                        }
                    })?;
                paths.extend(dependencies);
            }
        }
        check_build_abort(abort)?;
        Ok(paths.into_iter().collect())
    }
}
