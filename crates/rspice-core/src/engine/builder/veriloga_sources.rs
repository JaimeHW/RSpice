//! Resolve a deck's Verilog sources before allocating mixed instances.
//!
//! Group module selections by source so one active closure supplies their
//! compilation and connection rules. Freeze discovery before executable
//! elaboration, bound retained source closures, and key runtime variants by
//! selected rules and the original model specialization. The sources of a
//! design with several are one compilation unit, read in deck order.

use super::connect_modules::DesignConnectRules;
use super::veriloga_cache::{
    CachedVerilogAModel, canonicalize_for_cache, is_sealed_veriloga_virtual_path,
    lookup_cached_veriloga_with_limits_and_abort, lookup_registered_connection_library,
    prepare_veriloga_source,
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
    library_map: Option<&Path>,
    rules: &mut DesignConnectRules,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<Option<CachedVerilogAModel>>, SimulationError> {
    // A library map (IEEE 1364-2005 section 13.2) places the unit's files in
    // libraries and brings in files of its own.
    let library_map = library_map
        .map(super::hdl_library_map::LibraryMap::read)
        .transpose()?;
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
    // The HDL sources of a run are one compilation unit (IEEE 1364-2005
    // clause 19), except a sealed runtime, which is registered whole, and a
    // connection library, which is a library of its own that a design selects
    // by source. A unit of one file is that file, as it always was, unless a
    // library map places it.
    let (unit_groups, groups): (Vec<_>, Vec<_>) = groups.into_iter().partition(|group| {
        let path = &includes[group[0]].file_path;
        !is_sealed_veriloga_virtual_path(path) && !declares_connect_items(path)
    });
    let (unit_groups, groups) =
        if unit_groups.len() > 1 || (library_map.is_some() && !unit_groups.is_empty()) {
            (unit_groups, groups)
        } else {
            let mut groups = [unit_groups, groups].concat();
            groups.sort_by_key(|group| group[0]);
            (Vec::new(), groups)
        };
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
            // selected modules from this frozen preparation in the second pass,
            // except a registered bytecode-only model: it has no canonical
            // artifact to recompile or compare, and it is the registration's
            // own authority for this path. It is kept only while every file it
            // was registered against is the version this preparation read.
            for &index in &group {
                if let Some(entry) = models[index]
                    .as_ref()
                    .filter(|entry| entry.canonical_ir.is_none())
                {
                    for dependency in &entry.dependencies {
                        dependency_versions.record(
                            path,
                            &dependency.canonical_path,
                            dependency.content_hash,
                        )?;
                    }
                } else {
                    models[index] = None;
                }
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
    let unit = if unit_groups.is_empty() {
        None
    } else {
        Some(unit_first_pass(
            includes,
            unit_groups,
            library_map.as_ref(),
            rules,
            UnitState {
                models: &mut models,
                dependency_versions: &mut dependency_versions,
                admit_source: &mut admit_source,
            },
            limits,
            abort,
        )?)
    };
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
                    None,
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
                None,
                original.map_or(&[], |entry| entry.dependencies.as_slice()),
                limits,
                abort,
            )?;
            compiled_selections.insert(includes[index].selected_module.clone(), entry.clone());
            models[index] = Some(entry);
        }
    }
    if let Some(unit) = unit {
        unit_second_pass(
            includes,
            unit,
            configuration.as_deref(),
            &mut models,
            &mut dependency_versions,
            limits,
            abort,
        )?;
    }
    Ok(models)
}

/// Whether a source declares a connect module or a `connectrules` block.
///
/// Such a file is a library of its own, which a design selects by source and
/// whose names may repeat another library's, so it stays out of the design's
/// compilation unit and compiles alone as before. The test is lexical and
/// errs towards that: a file that only mentions the keywords, in a comment,
/// compiles alone too.
fn declares_connect_items(path: &Path) -> bool {
    let Ok(text) = std::fs::read_to_string(path) else {
        return false;
    };
    let identifier =
        |character: char| character.is_ascii_alphanumeric() || "_$".contains(character);
    ["connectmodule", "connectrules"].iter().any(|word| {
        text.match_indices(word).any(|(at, _)| {
            !text[..at].chars().next_back().is_some_and(identifier)
                && !text[at + word.len()..]
                    .chars()
                    .next()
                    .is_some_and(identifier)
        })
    })
}

/// What the unit's passes share with the per-file passes of the same design.
struct UnitState<'a> {
    models: &'a mut [Option<CachedVerilogAModel>],
    dependency_versions: &'a mut DependencyVersions,
    admit_source: &'a mut dyn FnMut(usize) -> Result<(), SimulationError>,
}

/// A compilation unit of several files, after its first pass.
struct UnitPass {
    /// The includes of each file, in deck order.
    groups: Vec<Vec<usize>>,
    identity: [u8; 32],
    /// The unit's preparation, unless every selection was a cache hit.
    prepared: Option<Box<rspice_veriloga::PreparedRuntimeSource>>,
}

/// The first pass over a design's files that form one compilation unit.
///
/// The files are the library `work`, read in deck order: macros and
/// `timescale` carry from one file into the next (IEEE 1364-2005 sections
/// 19.3 and 19.8), and a module of one file may instantiate a module of
/// another. A library map instead places each file in a library and adds the
/// files it names that the deck does not, after the deck's, in map order
/// (clause 13). Each selection is cached under the unit, so a design that
/// compiled before needs no preparation; otherwise the unit is prepared once
/// and its connect rules registered.
fn unit_first_pass(
    includes: &[VerilogAInclude],
    groups: Vec<Vec<usize>>,
    library_map: Option<&super::hdl_library_map::LibraryMap>,
    rules: &mut DesignConnectRules,
    state: UnitState<'_>,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<UnitPass, SimulationError> {
    use super::veriloga_cache::{lookup_unit_veriloga, prepare_veriloga_unit, unit_identity};
    let UnitState {
        models,
        dependency_versions,
        admit_source,
    } = state;
    let mut files: Vec<PathBuf> = groups
        .iter()
        .map(|group| includes[group[0]].file_path.clone())
        .collect();
    // A file the deck also names is read once, where the deck names it, in
    // the map's library.
    let libraries = match library_map {
        None => None,
        Some(map) => {
            let named = files
                .iter()
                .map(|path| canonicalize_for_cache(path))
                .collect();
            files.extend(map.other_files(&named));
            Some(rspice_veriloga::UnitLibraries {
                files: files
                    .iter()
                    .map(|path| map.library_of(path))
                    .collect::<Result<_, _>>()?,
                declared: map.declared(),
                configs: map.configs(),
            })
        }
    };
    let paths: Vec<&Path> = files.iter().map(PathBuf::as_path).collect();
    let identity = unit_identity(paths.iter().copied(), libraries.as_ref());
    let mut whole_hit = true;
    for &index in groups.iter().flatten() {
        super::check_build_abort(abort)?;
        let include = &includes[index];
        models[index] = lookup_unit_veriloga(
            &include.file_path,
            include.selected_module.as_deref(),
            identity,
            limits,
            abort,
        )?;
        whole_hit &= models[index].is_some();
    }
    // Hits are kept: each was compiled in this unit from files whose contents
    // are its recorded dependencies, and a file that changed since disagrees
    // with the preparation below on its version.
    let mut admitted = false;
    for &index in groups.iter().flatten() {
        let include = &includes[index];
        let Some(entry) = &models[index] else {
            continue;
        };
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
            rules.register_artifact(&include.file_path, artifact)?;
        }
        for dependency in &entry.dependencies {
            dependency_versions.record(
                &include.file_path,
                &dependency.canonical_path,
                dependency.content_hash,
            )?;
        }
    }
    let prepared = if whole_hit {
        None
    } else {
        let prepared = Box::new(prepare_veriloga_unit(
            &paths,
            libraries.as_ref(),
            limits,
            abort,
        )?);
        if !admitted {
            admit_source(prepared.preprocessed_source().len())?;
        }
        for dependency in prepared.dependencies() {
            dependency_versions.record(paths[0], &dependency.path, dependency.content_identity)?;
        }
        let specification = prepared.connect_specification();
        // The deck's sources; a file only the map names is no source a
        // design selects.
        for path in &paths[..groups.len()] {
            rules.register(path, specification.clone())?;
        }
        // A file of the unit may declare no module: the macros and
        // `timescale` later files read. It has no model.
        for (file, group) in prepared.unit_files().iter().zip(&groups) {
            if !file.modules.is_empty() {
                continue;
            }
            for &index in group {
                if let Some(module) = &includes[index].selected_module {
                    return Err(source_refusal(
                        &file.path,
                        ElaborationErrorKind::UnknownModule,
                        format!(
                            "this source declares no device module, so module '{module}' cannot be selected"
                        ),
                    ));
                }
            }
        }
        Some(prepared)
    };
    Ok(UnitPass {
        groups,
        identity,
        prepared,
    })
}

/// The second pass over a compilation unit: compile what the cache did not
/// hold, or what the design's connect configuration changes.
fn unit_second_pass(
    includes: &[VerilogAInclude],
    unit: UnitPass,
    configuration: Option<&rspice_veriloga::ConnectionConfiguration>,
    models: &mut [Option<CachedVerilogAModel>],
    dependency_versions: &mut DependencyVersions,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    use super::veriloga_cache::{
        UnitSelection, configuration_identity, lookup_configured_veriloga,
    };
    let mut compiled_selections: HashMap<(usize, Option<String>), CachedVerilogAModel> =
        HashMap::new();
    for (file, indices) in unit.groups.iter().enumerate() {
        for &index in indices {
            super::check_build_abort(abort)?;
            let include = &includes[index];
            let path = include.file_path.as_path();
            let selected = include.selected_module.as_deref();
            let key = (file, include.selected_module.clone());
            if let Some(cached) = compiled_selections.get(&key) {
                models[index] = Some(CachedVerilogAModel::clone(cached));
                continue;
            }
            let original = models[index].take();
            let artifact = original
                .as_ref()
                .and_then(|entry| entry.canonical_ir.as_deref());
            let needs_configuration = configuration.is_some_and(|configuration| {
                artifact.is_none_or(|artifact| {
                    artifact.connections.source().is_some()
                        && artifact.connections.configuration() != Some(configuration)
                })
            });
            if let Some(original) = original
                && !needs_configuration
            {
                compiled_selections.insert(key, original.clone());
                models[index] = Some(original);
                continue;
            }
            // A whole hit reaches here only to apply a configuration, which
            // replays the unit from its preparation.
            let Some(prepared) = unit.prepared.as_deref() else {
                return Err(source_refusal(
                    path,
                    ElaborationErrorKind::MissingSource,
                    "configured runtime requires retained source",
                ));
            };
            let modules = &prepared.unit_files()[file].modules;
            if modules.is_empty() {
                continue; // Directives only: no model.
            }
            let module = match (selected, modules.as_slice()) {
                (Some(module), _) => module,
                (None, [module]) => module.as_str(),
                (None, modules) => {
                    return Err(source_refusal(
                        path,
                        ElaborationErrorKind::ModuleNotSelected,
                        format!(
                            "the file declares multiple modules: {}; select one by name",
                            modules
                                .iter()
                                .map(|module| module.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    ));
                }
            };
            if let Some(configuration) = configuration {
                let identity = prepared
                    .runtime_source_identity(Some(module))
                    .map_err(|error| {
                        source_refusal(
                            path,
                            ElaborationErrorKind::ModuleNotSelected,
                            error.to_string(),
                        )
                    })?;
                if let Some(cached) = lookup_configured_veriloga(
                    path,
                    selected,
                    configuration_identity(identity, configuration),
                    Some(unit.identity),
                    limits,
                    abort,
                )? {
                    for dependency in &cached.dependencies {
                        dependency_versions.record(
                            path,
                            &dependency.canonical_path,
                            dependency.content_hash,
                        )?;
                    }
                    compiled_selections.insert(key, cached.clone());
                    models[index] = Some(cached);
                    continue;
                }
            }
            let entry = super::veriloga_cache::compile_and_cache_prepared_with_connections(
                path,
                selected,
                prepared,
                configuration,
                Some(UnitSelection {
                    identity: unit.identity,
                    module,
                }),
                &[],
                limits,
                abort,
            )?;
            compiled_selections.insert(key, entry.clone());
            models[index] = Some(entry);
        }
    }
    Ok(())
}
