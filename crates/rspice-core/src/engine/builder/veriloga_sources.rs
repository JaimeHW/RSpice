//! Resolve a deck's Verilog sources before allocating mixed instances.
//!
//! Group module selections by source so one active closure supplies their
//! compilation and connection rules. Freeze discovery before executable
//! elaboration, bound retained source closures, and key runtime variants by
//! selected rules and the original model specialization.

use super::connect_modules::DesignConnectRules;
use super::veriloga_cache::{
    CachedVerilogAModel, canonicalize_for_cache, lookup_cached_veriloga_with_limits_and_abort,
    lookup_registered_connection_library, prepare_veriloga_source,
};
use crate::abort_signal::AbortSignal;
use crate::netlist::VerilogAInclude;
use crate::{ElaborationError, ElaborationErrorKind, ResourceLimits, SimulationError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Every refusal here is about one `.VERILOGA` source, so the source is the
/// subject and the span points at the file. No instance has been reached yet:
/// sources resolve once for the whole design, before any X-card is bound.
fn source_refusal(
    path: &Path,
    kind: ElaborationErrorKind,
    detail: impl Into<String>,
) -> SimulationError {
    ElaborationError::new(kind, detail).in_source(path).into()
}

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
                    return Err(source_refusal(
                        path,
                        ElaborationErrorKind::MissingSource,
                        format!(
                            "this dependency changed while sources '{}' and '{}' were being elaborated; retry with a stable source snapshot",
                            first_root.display(),
                            root.display()
                        ),
                    ));
                }
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert((identity, root.to_path_buf()));
            }
        }
        Ok(())
    }
}

/// A frozen preparation or authenticated cache registration. Discover every
/// group's rules before compiling any hierarchy which could need those rules.
struct SourceGroup {
    indices: Vec<usize>,
    prepared: Option<rspice_veriloga::PreparedRuntimeSource>,
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
    let mut frozen = Vec::new();
    let mut retained_source_bytes = 0usize;
    let mut admit_source = |bytes: usize| -> Result<(), SimulationError> {
        retained_source_bytes = retained_source_bytes.saturating_add(bytes);
        crate::ResourceLimitError::ensure(
            crate::ResourceKind::ExpandedSourceBytes,
            retained_source_bytes,
            limits.max_expanded_source_bytes,
        )?;
        Ok(())
    };
    for group in groups {
        super::check_build_abort(abort)?;
        let path = &includes[group[0]].file_path;
        if let Some(library) = lookup_registered_connection_library(path, limits, abort)? {
            for &index in &group {
                if let Some(module) = &includes[index].selected_module {
                    return Err(source_refusal(
                        path,
                        ElaborationErrorKind::ModuleNotSelected,
                        format!(
                            "this source is a registered connection library; device module '{module}' cannot be selected"
                        ),
                    ));
                }
            }
            admit_source(library.preprocessed_source().len())?;
            let specification = library
                .connect_specification()
                .map_err(|error| source_refusal(path, ElaborationErrorKind::ConnectRule, error))?;
            rules.register(path, specification)?;
            continue;
        }
        let mut needs_preparation = false;
        let mut cached_identity = None;
        for &index in &group {
            let include = &includes[index];
            models[index] = lookup_cached_veriloga_with_limits_and_abort(
                path,
                include.selected_module.as_deref(),
                limits,
                abort,
            )?;
            match models[index]
                .as_ref()
                .and_then(|entry| entry.canonical_ir.as_deref())
            {
                Some(artifact) => {
                    needs_preparation |= cached_identity
                        .as_ref()
                        .is_some_and(|identity| identity != &artifact.metadata.source_identity);
                    cached_identity = Some(artifact.metadata.source_identity.clone());
                }
                None => {
                    needs_preparation |= !models[index]
                        .as_ref()
                        .is_some_and(|entry| entry.dependencies.is_empty());
                }
            }
        }
        let prepared = if needs_preparation {
            let prepared = prepare_veriloga_source(path, limits, abort)?;
            admit_source(prepared.preprocessed_source().len())?;
            for dependency in prepared.dependencies() {
                dependency_versions.record(path, &dependency.path, dependency.content_identity)?;
            }
            let specification = prepared.connect_specification();
            let has_modules = specification.declares_module;
            if !has_modules && !prepared.is_connect_library() {
                return Err(source_refusal(
                    path,
                    ElaborationErrorKind::UnknownModule,
                    "this source declares neither a device module nor a connection library",
                ));
            }
            rules.register(path, specification)?;
            if !has_modules {
                for &index in &group {
                    if let Some(module) = &includes[index].selected_module {
                        return Err(source_refusal(
                            path,
                            ElaborationErrorKind::UnknownModule,
                            format!(
                                "this source declares no device module, so module '{module}' cannot be selected"
                            ),
                        ));
                    }
                    models[index] = None;
                }
                continue;
            }
            // A partial cache hit must not mix source snapshots. Compile all
            // selected modules from this frozen preparation in the second pass.
            for &index in &group {
                models[index] = None;
            }
            Some(prepared)
        } else {
            let mut admitted = false;
            for &index in &group {
                if let Some(entry) = &models[index] {
                    if let Some(artifact) = entry.canonical_ir.as_deref() {
                        if !admitted {
                            admit_source(
                                artifact
                                    .connections
                                    .source()
                                    .or(artifact.parameter_source.as_deref())
                                    .map_or(0, str::len),
                            )?;
                            admitted = true;
                        }
                        rules.register_artifact(path, artifact)?;
                    }
                    for dependency in &entry.dependencies {
                        dependency_versions.record(
                            path,
                            &dependency.canonical_path,
                            dependency.content_hash,
                        )?;
                    }
                }
            }
            None
        };
        frozen.push(SourceGroup {
            indices: group,
            prepared,
        });
    }
    super::check_build_abort(abort)?;
    let configuration = rules.configuration()?;
    let compiler = rspice_veriloga::VerilogACompiler::new(
        super::veriloga_cache::deck_include_compiler_options(),
    );
    let control = super::veriloga_cache::VerilogACompileControl { abort };
    for group in frozen {
        let path = &includes[group.indices[0]].file_path;
        let mut compiled_selections = HashMap::new();
        for index in group.indices {
            super::check_build_abort(abort)?;
            let selected = includes[index].selected_module.as_deref();
            if let Some(cached) = compiled_selections.get(&includes[index].selected_module) {
                models[index] = Some(CachedVerilogAModel::clone(cached));
                continue;
            }
            let original = models[index].as_ref();
            let artifact = original.and_then(|entry| entry.canonical_ir.as_deref());
            let needs_configuration = configuration.as_deref().is_some_and(|configuration| {
                artifact.is_none_or(|artifact| {
                    artifact.connections.source().is_some()
                        && artifact.connections.configuration() != Some(configuration)
                })
            });
            if let Some(original) = original
                && !needs_configuration
            {
                compiled_selections
                    .insert(includes[index].selected_module.clone(), original.clone());
                continue;
            }
            // Contextual hits use the authenticated source and root assignments,
            // never only the path of a potentially replaced registration.
            if let Some(configuration) = configuration.as_deref() {
                let identity = match (&group.prepared, artifact) {
                    (Some(prepared), _) => {
                        prepared
                            .runtime_source_identity(selected)
                            .map_err(|error| {
                                source_refusal(
                                    path,
                                    ElaborationErrorKind::ModuleNotSelected,
                                    error.to_string(),
                                )
                            })?
                    }
                    (None, Some(artifact)) => artifact.runtime_source_identity(),
                    _ => {
                        return Err(source_refusal(
                            path,
                            ElaborationErrorKind::MissingSource,
                            "configured runtime requires retained source",
                        ));
                    }
                };
                if let Some(cached) = super::veriloga_cache::lookup_configured_veriloga(
                    path,
                    selected,
                    super::veriloga_cache::configuration_identity(identity, configuration),
                    limits,
                    abort,
                )? {
                    // A cold preparation and a disk entry must agree on every
                    // dependency version already frozen in the first pass.
                    for dependency in &cached.dependencies {
                        dependency_versions.record(
                            path,
                            &dependency.canonical_path,
                            dependency.content_hash,
                        )?;
                    }
                    compiled_selections
                        .insert(includes[index].selected_module.clone(), cached.clone());
                    models[index] = Some(cached);
                    continue;
                }
            }
            let replay;
            let prepared = if let Some(prepared) = &group.prepared {
                prepared
            } else {
                let artifact = artifact.ok_or_else(|| {
                    source_refusal(
                        path,
                        ElaborationErrorKind::MissingSource,
                        "configured runtime requires retained source",
                    )
                })?;
                replay = compiler
                    .prepare_artifact_runtime_source(artifact, &control)
                    .map_err(|error| {
                        if abort.is_aborted() {
                            SimulationError::Aborted
                        } else {
                            source_refusal(
                                path,
                                ElaborationErrorKind::CompileRefusal,
                                error.to_string(),
                            )
                        }
                    })?;
                &replay
            };
            let entry = super::veriloga_cache::compile_and_cache_prepared_with_connections(
                path,
                selected,
                prepared,
                configuration.as_deref(),
                original.map_or(&[], |entry| entry.dependencies.as_slice()),
                limits,
                abort,
            )?;
            compiled_selections.insert(includes[index].selected_module.clone(), entry.clone());
            models[index] = Some(entry);
        }
    }
    Ok(models)
}
