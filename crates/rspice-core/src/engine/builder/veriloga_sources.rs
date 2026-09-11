//! Resolve a deck's Verilog sources before allocating mixed instances.
//!
//! Group module selections by source so one active closure supplies their
//! compilation and connection rules. Only one analyzed tree is retained at a
//! time; completed models keep using the existing bounded runtime cache.

use super::connect_modules::DesignConnectRules;
use super::veriloga_cache::{
    CachedVerilogAModel, canonicalize_for_cache, compile_and_cache_prepared_veriloga,
    lookup_cached_veriloga_with_limits_and_abort, lookup_registered_connection_library,
    prepare_veriloga_source,
};
use crate::abort_signal::AbortSignal;
use crate::netlist::VerilogAInclude;
use crate::{ResourceLimits, SimulationError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Default)]
struct DependencyVersions {
    observed: HashMap<PathBuf, ([u8; 32], PathBuf)>,
}

impl DependencyVersions {
    fn record(
        &mut self,
        root: &Path,
        path: &Path,
        identity: [u8; 32],
    ) -> Result<(), SimulationError> {
        match self.observed.entry(path.to_path_buf()) {
            std::collections::hash_map::Entry::Occupied(previous) => {
                let (first_identity, first_root) = previous.get();
                if first_identity != &identity {
                    return Err(SimulationError::Netlist(format!(
                        "Verilog-A dependency '{}' changed while sources '{}' and '{}' were being elaborated; retry with a stable source snapshot",
                        path.display(),
                        first_root.display(),
                        root.display()
                    )));
                }
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert((identity, root.to_path_buf()));
            }
        }
        Ok(())
    }
}

pub(super) fn resolve_includes(
    includes: &[VerilogAInclude],
    rules: &mut DesignConnectRules,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<Option<CachedVerilogAModel>>, SimulationError> {
    let mut group_indices = HashMap::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for (index, include) in includes.iter().enumerate() {
        super::check_build_abort(abort)?;
        let next_group = groups.len();
        let group = *group_indices
            .entry(canonicalize_for_cache(&include.file_path))
            .or_insert_with(|| {
                groups.push(Vec::new());
                next_group
            });
        groups[group].push(index);
    }
    let mut models = vec![None; includes.len()];
    let mut dependency_versions = DependencyVersions::default();
    for group in groups {
        let path = &includes[group[0]].file_path;
        if let Some(library) = lookup_registered_connection_library(path, limits, abort)? {
            for &index in &group {
                if let Some(module) = &includes[index].selected_module {
                    return Err(SimulationError::Netlist(format!(
                        "Verilog-A source '{}' is a registered connection library; device module '{}' cannot be selected",
                        path.display(),
                        module
                    )));
                }
            }
            let specification = library
                .connect_specification()
                .map_err(SimulationError::Netlist)?;
            super::check_build_abort(abort)?;
            rules.register(path, specification)?;
            continue;
        }
        let mut needs_preparation = false;
        let mut cached_identity = None;
        for &index in &group {
            let include = &includes[index];
            models[index] = lookup_cached_veriloga_with_limits_and_abort(
                &include.file_path,
                include.selected_module.as_deref(),
                limits,
                abort,
            )?;
            match models[index]
                .as_ref()
                .and_then(|entry| entry.canonical_ir.as_deref())
            {
                Some(artifact) => {
                    if cached_identity
                        .as_ref()
                        .is_some_and(|identity| identity != &artifact.metadata.source_identity)
                    {
                        needs_preparation = true;
                    }
                    cached_identity = Some(artifact.metadata.source_identity.clone());
                }
                None => {
                    // The legacy browser registration API can supply only a
                    // bytecode model. Its empty dependency set is fileless;
                    // there is no source to reopen for connection discovery.
                    needs_preparation |= !models[index]
                        .as_ref()
                        .is_some_and(|entry| entry.dependencies.is_empty());
                }
            }
        }
        if needs_preparation {
            let prepared = prepare_veriloga_source(path, limits, abort)?;
            for dependency in prepared.dependencies() {
                dependency_versions.record(path, &dependency.path, dependency.content_identity)?;
            }
            let specification = prepared.connect_specification();
            let source_identity = specification.source_identity.clone();
            let has_modules = specification.declares_module;
            if !has_modules && !prepared.is_connect_library() {
                return Err(SimulationError::Netlist(format!(
                    "Verilog-A source '{}' declares neither a device module nor a connection library",
                    path.display()
                )));
            }
            rules.register(path, specification)?;
            // Reuse the complete analyzed tree for every selected module, and
            // release it before preparing the next source group.
            let mut compiled_selections = HashMap::new();
            for &index in &group {
                let include = &includes[index];
                if !has_modules {
                    if let Some(module) = &include.selected_module {
                        return Err(SimulationError::Netlist(format!(
                            "Verilog-A source '{}' declares no device module, so module '{}' cannot be selected",
                            path.display(),
                            module
                        )));
                    }
                    models[index] = None;
                    continue;
                }
                // A file changed between cache lookups and preparation must
                // not mix artifacts from different snapshots of one source.
                let reuse = models[index].as_ref().is_some_and(|entry| {
                    entry.canonical_ir.as_deref().is_none_or(|artifact| {
                        artifact.metadata.source_identity.as_str() == source_identity
                    })
                });
                if !reuse {
                    let selected = include.selected_module.clone();
                    let model = match compiled_selections.entry(selected) {
                        std::collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
                        std::collections::hash_map::Entry::Vacant(entry) => {
                            entry.insert(compile_and_cache_prepared_veriloga(
                                &include.file_path,
                                include.selected_module.as_deref(),
                                &prepared,
                                limits,
                                abort,
                            )?)
                        }
                    };
                    models[index] = Some(model.clone());
                }
            }
        } else {
            for &index in &group {
                if let Some(artifact) = models[index]
                    .as_ref()
                    .and_then(|entry| entry.canonical_ir.as_deref())
                {
                    rules.register_artifact(&includes[index].file_path, artifact)?;
                }
            }
        }
        for index in group {
            if let Some(entry) = &models[index] {
                for dependency in &entry.dependencies {
                    dependency_versions.record(
                        path,
                        &dependency.canonical_path,
                        dependency.content_hash,
                    )?;
                }
            }
        }
    }
    Ok(models)
}
