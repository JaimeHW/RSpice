//! The model library.
//!
//! Owns every model a project can resolve, from every source, and seals the
//! exact set a run executed against. Sealing is by content digest, so a
//! library that changes underneath a completed run is detectable rather
//! than silently assumed identical.

mod project_models;
mod sealing;

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rspice_core::library::SpiceLibraryIndex;

#[cfg(not(target_arch = "wasm32"))]
use super::is_foreign_platform_absolute_path;
use super::{
    DeviceModel, FiniteF64, ModelCorrelationState, ModelDefinitionMetadata, ModelFileIdentity,
    ModelLevel, ModelLibrary, ModelQualificationState, ModelSectionQualification,
    ModelSourceAuthority, ModelSourceContent, ModelSourceEdge, ModelSourceEvidenceBinding,
    ModelSourcePin, ModelType, ParameterDataType, ParameterDefinition, ParameterSource,
    ParameterValue, ProcessCorner, ProjectModelDefinition, ProjectModelRevisionDefinition,
    first_unreachable_source,
};
use crate::product::{ContentDigest, ModelSourceId, ObjectRevision};
use crate::services::simulation_runner::{CornerModelBinding, CornerProcess};

/// Published result of one atomic project-model definition transaction.
#[derive(Debug, Clone)]
pub struct ProjectModelCommit {
    pub library_name: String,
    pub model_name: String,
    pub before: Option<ModelLibrary>,
    pub after: ModelLibrary,
    /// Definition/source changes invalidate downstream execution; evidence-
    /// only commits do not alter the executable model closure.
    pub affects_execution: bool,
}

/// One immutable, authenticated model-source snapshot for a simulation run.
/// The exact bytes are intentionally transient and are never serialized into
/// project/session state.
#[derive(Debug, Clone)]
pub struct SealedModelExecutionSources {
    bundle: rspice_core::netlist::SealedSourceBundle,
    sources: Vec<(PathBuf, String)>,
    edges: Vec<rspice_core::netlist::SealedSourceEdge>,
    libraries: Vec<SealedExecutionLibrary>,
}

#[derive(Debug, Clone)]
struct SealedExecutionLibrary {
    name: String,
    root_path: PathBuf,
    sections: Vec<String>,
}

impl SealedModelExecutionSources {
    /// Build a source bundle that adds one active root buffer to the exact
    /// authenticated model-library closure.
    ///
    /// Root include edges are accepted only when their portable lexical target
    /// names one retained source exactly. This deliberately does not guess by
    /// basename or consult a host search path: unresolved and ambiguous deck
    /// references fail closed in browser execution.
    pub(crate) fn bundle_for_root(
        &self,
        root_path: &Path,
        root_source: &str,
    ) -> Result<rspice_core::netlist::SealedSourceBundle, String> {
        if !super::is_portable_absolute_path(root_path) {
            return Err(format!(
                "Authenticated browser source root must have an absolute portable identity: {}",
                root_path.display()
            ));
        }

        let root_key = portable_path_key(root_path);
        let matching_roots = self
            .sources
            .iter()
            .filter(|(path, _)| portable_path_key(path) == root_key)
            .collect::<Vec<_>>();
        if matching_roots.len() > 1 {
            return Err(format!(
                "Authenticated model sources contain an ambiguous root identity '{}'",
                root_path.display()
            ));
        }

        let mut sources = self.sources.clone();
        let mut edges = self.edges.clone();
        if let Some((accepted_path, accepted_source)) = matching_roots.first().copied() {
            if accepted_source != root_source {
                return Err(format!(
                    "Active source '{}' conflicts with an authenticated model-source member",
                    root_path.display()
                ));
            }
            if accepted_path != root_path {
                for (path, _) in &mut sources {
                    if path == accepted_path {
                        *path = root_path.to_path_buf();
                    }
                }
                for edge in &mut edges {
                    if edge.owner == *accepted_path {
                        edge.owner = root_path.to_path_buf();
                    }
                    if edge.target == *accepted_path {
                        edge.target = root_path.to_path_buf();
                    }
                }
            }
        } else {
            sources.push((root_path.to_path_buf(), root_source.to_owned()));
        }

        let mut root_edge_keys = HashSet::new();
        for requested_path in root_external_source_paths(root_source) {
            let requested_path = rspice_core::netlist::normalize_source_path_literal(
                &requested_path,
            )
            .map_err(|error| {
                format!(
                    "Source '{}' has an invalid external dependency path: {error}",
                    root_path.display()
                )
            })?;
            if !root_edge_keys.insert(requested_path.clone()) {
                continue;
            }
            if edges.iter().any(|edge| {
                portable_path_key(&edge.owner) == root_key && edge.requested_path == requested_path
            }) {
                continue;
            }

            let target_key = portable_dependency_target_key(root_path, &requested_path)?;
            let candidates = self
                .sources
                .iter()
                .filter(|(path, _)| portable_path_key(path) == target_key)
                .map(|(path, _)| path)
                .collect::<Vec<_>>();
            let target = match candidates.as_slice() {
                [target] => (*target).clone(),
                [] => {
                    return Err(format!(
                        "Dependency '{}' referenced by '{}' is not present in the authenticated model-source closure",
                        requested_path,
                        root_path.display()
                    ));
                }
                _ => {
                    return Err(format!(
                        "Dependency '{}' referenced by '{}' has an ambiguous authenticated source identity",
                        requested_path,
                        root_path.display()
                    ));
                }
            };
            edges.push(rspice_core::netlist::SealedSourceEdge {
                owner: root_path.to_path_buf(),
                requested_path,
                target,
            });
        }

        rspice_core::netlist::SealedSourceBundle::try_new_with_edges(sources, edges)
            .map_err(|error| format!("Failed to authorize active source dependencies: {error}"))
    }

    /// Expand an active root through the authenticated model source closure
    /// without consulting a filesystem.
    #[cfg(any(target_arch = "wasm32", test))]
    pub(crate) fn expand_root_dependencies(
        &self,
        root_path: &Path,
        root_source: &str,
        abort: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> Result<(String, Vec<rspice_core::netlist::ResolvedIncludeDependency>), String> {
        let bundle = self.bundle_for_root(root_path, root_source)?;
        let mut processor = rspice_core::netlist::IncludeProcessor::new_sealed(root_path, bundle);
        let expanded = processor
            .expand_content_with_abort(root_source, root_path, abort)
            .map_err(|error| {
                format!(
                    "Could not expand authenticated dependencies for '{}': {error}",
                    root_path.display()
                )
            })?;
        Ok((expanded, processor.resolved_dependencies().to_vec()))
    }

    /// Materialize the exact model cards for the nominal/reference process.
    pub fn reference_process_model_cards(
        &self,
        process: crate::simulation::dialog::corner::ProcessCorner,
    ) -> Result<Vec<String>, String> {
        let process = match process {
            crate::simulation::dialog::corner::ProcessCorner::TT => CornerProcess::TT,
            crate::simulation::dialog::corner::ProcessCorner::SS => CornerProcess::SS,
            crate::simulation::dialog::corner::ProcessCorner::FF => CornerProcess::FF,
            crate::simulation::dialog::corner::ProcessCorner::SF => CornerProcess::SF,
            crate::simulation::dialog::corner::ProcessCorner::FS => CornerProcess::FS,
        };
        self.bindings_for_processes(&[process]).map(|bindings| {
            bindings
                .into_iter()
                .map(|binding| {
                    format!(
                        "* RSpice sealed model source: {}\n{}",
                        binding.source_label, binding.materialized_model_cards
                    )
                })
                .collect()
        })
    }

    /// Materialize every model section required by a process-corner run from
    /// this same immutable snapshot.
    pub fn corner_model_bindings(
        &self,
        processes: &[CornerProcess],
    ) -> Result<Vec<CornerModelBinding>, String> {
        self.bindings_for_processes(processes)
    }

    fn bindings_for_processes(
        &self,
        processes: &[CornerProcess],
    ) -> Result<Vec<CornerModelBinding>, String> {
        if self.libraries.is_empty() {
            if let Some(process) = processes
                .iter()
                .find(|process| **process != CornerProcess::TT)
            {
                return Err(format!(
                    "{} requires a PDK model library with an explicit process section",
                    process.as_keyword()
                ));
            }
            return Ok(Vec::new());
        }

        let mut bindings = Vec::new();
        for process in processes {
            for library in &self.libraries {
                let keyword = process.as_keyword();
                let section = library
                    .sections
                    .iter()
                    .find(|section| section.eq_ignore_ascii_case(keyword))
                    .cloned();
                if section.is_none()
                    && (*process != CornerProcess::TT || !library.sections.is_empty())
                {
                    return Err(format!(
                        "Model library '{}' does not define the {} process section",
                        library.name, keyword
                    ));
                }

                let mut processor = rspice_core::netlist::IncludeProcessor::new_sealed(
                    &library.root_path,
                    self.bundle.clone(),
                );
                let materialized_model_cards = processor
                    .process_sealed_root(&library.root_path, section.as_deref())
                    .map_err(|error| {
                        format!(
                            "Failed to materialize sealed model library '{}' from '{}': {error}",
                            library.name,
                            library.root_path.display()
                        )
                    })?;
                let source_label = match section.as_deref() {
                    Some(section) => format!("{} [{}]", library.root_path.display(), section),
                    None => library.root_path.display().to_string(),
                };
                let binding = CornerModelBinding {
                    process: *process,
                    source_label,
                    section,
                    materialized_model_cards,
                };
                binding.validate()?;
                bindings.push(binding);
            }
        }
        Ok(bindings)
    }
}

fn root_external_source_paths(source: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut inline_library_depth = 0usize;
    for line in source.lines() {
        let directive = line.split_whitespace().next().unwrap_or_default();
        if directive.eq_ignore_ascii_case(".endl") {
            inline_library_depth = inline_library_depth.saturating_sub(1);
            continue;
        }
        if let Some((path, section)) = rspice_core::netlist::parse_lib_directive(line) {
            if section.is_none() {
                inline_library_depth = inline_library_depth.saturating_add(1);
            } else if inline_library_depth == 0 {
                paths.push(path);
            }
            continue;
        }
        if inline_library_depth == 0
            && let Some(path) = rspice_core::netlist::parse_include_directive(line)
        {
            paths.push(path);
        }
    }
    paths
}

fn portable_dependency_target_key(
    root_path: &Path,
    requested_path: &str,
) -> Result<String, String> {
    let requested = normalize_portable_path_text(requested_path)?;
    if is_portable_absolute_text(&requested) {
        return Ok(portable_text_key(&requested));
    }
    let root = normalize_portable_path_text(&root_path.to_string_lossy())?;
    let mut parent = root.rsplit_once('/').map_or("", |(parent, _)| parent);
    if parent.is_empty() && root.starts_with('/') {
        parent = "/";
    }
    let joined = if parent.is_empty() {
        requested
    } else if parent == "/" {
        format!("/{requested}")
    } else {
        format!("{parent}/{requested}")
    };
    normalize_portable_path_text(&joined).map(|path| portable_text_key(&path))
}

fn portable_path_key(path: &Path) -> String {
    normalize_portable_path_text(&path.to_string_lossy())
        .map(|path| portable_text_key(&path))
        .unwrap_or_else(|_| path.to_string_lossy().replace('\\', "/"))
}

fn portable_text_key(path: &str) -> String {
    let mut key = path.to_owned();
    if is_windows_absolute_text(path) {
        key.make_ascii_lowercase();
    }
    key
}

fn is_portable_absolute_text(path: &str) -> bool {
    path.starts_with('/') || is_windows_absolute_text(path)
}

fn is_windows_absolute_text(path: &str) -> bool {
    let candidate = path
        .strip_prefix("//?/")
        .or_else(|| path.strip_prefix("//./"))
        .unwrap_or(path);
    let candidate = candidate
        .strip_prefix("UNC/")
        .or_else(|| candidate.strip_prefix("unc/"))
        .unwrap_or(candidate);
    let bytes = candidate.as_bytes();
    (bytes.len() >= 3 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && bytes[2] == b'/')
        || (path.starts_with("//")
            && candidate
                .split('/')
                .filter(|component| !component.is_empty())
                .take(2)
                .count()
                == 2)
}

fn normalize_portable_path_text(path: &str) -> Result<String, String> {
    let mut path = path.trim().replace('\\', "/");
    if path.is_empty() || path.chars().any(char::is_control) {
        return Err("source path is empty or contains a control character".to_owned());
    }
    if let Some(unprefixed) = path.strip_prefix("//?/") {
        path = if let Some(unc) = unprefixed.strip_prefix("UNC/") {
            format!("//{unc}")
        } else {
            unprefixed.to_owned()
        };
    }

    let prefix_len = if path.starts_with("//") {
        2
    } else if path.starts_with('/') {
        1
    } else if is_windows_absolute_text(&path) {
        3
    } else {
        0
    };
    let prefix = &path[..prefix_len];
    let mut components = Vec::new();
    for component in path[prefix_len..].split('/') {
        match component {
            "" | "." => {}
            ".." if components.pop().is_some() => {}
            ".." if prefix_len == 0 => components.push(component),
            ".." => {
                return Err(format!("absolute source path escapes its root: {path}"));
            }
            _ => components.push(component),
        }
    }
    let body = components.join("/");
    Ok(match prefix {
        "//" => format!("//{body}"),
        "/" => format!("/{body}"),
        _ if prefix_len == 3 => format!("{prefix}{body}"),
        _ => body,
    })
}

/// Manager for all model libraries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelLibraryManager {
    /// All libraries
    libraries: HashMap<String, ModelLibrary>,
    /// Currently selected library
    pub selected_library: Option<String>,
    /// Search filter
    pub filter_text: String,
    /// Filter by model type
    pub filter_type: Option<ModelType>,
    /// Index over the shipped model packs, when one was found on disk.
    ///
    /// Held rather than loaded: the packs carry around 199,000 definitions, so
    /// materializing them as `DeviceModel`s would cost far more memory than the
    /// catalogue view needs. Queries stream the on-disk index instead.
    ///
    /// Not serialized. It is a view of what is installed on this machine, so it
    /// is rediscovered on load rather than restored from a project file that may
    /// have been written elsewhere.
    #[serde(skip)]
    spice_packs: Option<Arc<SpiceLibraryIndex>>,
}

/// One definition found in the shipped packs rather than in a loaded library.
///
/// Deliberately not a [`DeviceModel`]: nothing here has been parsed, and
/// presenting an unparsed catalogue row as a loaded model would overstate what
/// the application knows about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackModelHit {
    /// Definition name as written in the source.
    pub name: String,
    /// `model` or `subckt`.
    pub kind: String,
    /// Canonical device class, such as `diode` or `mosfet-n`.
    pub device: String,
    /// Owning pack identifier.
    pub pack: String,
    /// Human-readable pack title.
    pub pack_name: String,
    /// Absolute path to the defining file, when the pack is known.
    pub source: Option<PathBuf>,
    /// 1-based line of the definition.
    pub line: usize,
    /// Whether RSpice has established the right to redistribute the pack.
    pub redistributable: bool,
}

impl ModelLibraryManager {
    /// Create a new manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a library
    pub fn add_library(&mut self, library: ModelLibrary) {
        self.libraries.insert(library.name.clone(), library);
    }

    /// Remove a library
    pub fn remove_library(&mut self, name: &str) -> Option<ModelLibrary> {
        self.libraries.remove(name)
    }

    /// Get a library
    pub fn get_library(&self, name: &str) -> Option<&ModelLibrary> {
        self.libraries.get(name)
    }

    /// Get mutable library
    pub fn get_library_mut(&mut self, name: &str) -> Option<&mut ModelLibrary> {
        self.libraries.get_mut(name)
    }

    /// Select a library
    pub fn select_library(&mut self, name: &str) {
        if self.libraries.contains_key(name) {
            self.selected_library = Some(name.to_string());
        }
    }

    /// Get current library
    pub fn current_library(&self) -> Option<&ModelLibrary> {
        self.selected_library
            .as_ref()
            .and_then(|name| self.libraries.get(name))
    }

    /// Canonical identities of every project-owned model definition admitted
    /// to the executable model closure.
    ///
    /// Prepared simulation receipts retain these typed identities so later
    /// engineering evidence can prove that an exact model revision was present
    /// in the immutable run snapshot instead of trusting a display name or a
    /// user-entered digest.
    pub(crate) fn project_model_definition_identities(
        &self,
    ) -> Result<Vec<(ModelSourceId, String, ObjectRevision, ContentDigest)>, String> {
        let mut identities = Vec::new();
        for library in self
            .libraries
            .values()
            .filter(|library| library.source_authority.is_project_owned())
        {
            let ModelSourceAuthority::ProjectOwned {
                source_id,
                revision: library_revision,
                ..
            } = library.source_authority
            else {
                continue;
            };
            for (model_name, model) in &library.models {
                let metadata = library
                    .model_definition_metadata
                    .get(model_name)
                    .cloned()
                    .ok_or_else(|| {
                        format!(
                            "Project model '{}/{}' has no typed definition metadata",
                            library.name, model_name
                        )
                    })?;
                let definition = ProjectModelRevisionDefinition::new(
                    ProjectModelDefinition::from_device_model(model),
                    metadata,
                );
                let canonical = definition.canonical_source().map_err(|error| {
                    format!(
                        "Project model '{}/{}' cannot be authenticated for execution: {error}",
                        library.name, model_name
                    )
                })?;
                let definition_identity =
                    definition.project_source_identity().map_err(|error| {
                        format!(
                            "Project model '{}/{}' has invalid source identity: {error}",
                            library.name, model_name
                        )
                    })?;
                let revision = definition_identity
                    .as_ref()
                    .map_or(library_revision, |identity| identity.revision);
                let digest = ContentDigest::from_bytes(Sha256::digest(canonical.as_bytes()).into());
                if let Some(identity) = definition_identity
                    && (identity.source_id != source_id || identity.content_digest != digest)
                {
                    return Err(format!(
                        "Project model '{}/{}' definition identity does not match its retained source",
                        library.name, model_name
                    ));
                }
                identities.push((source_id, model_name.clone(), revision, digest));
            }
        }
        identities.sort_by(|left, right| {
            left.0
                .as_uuid()
                .cmp(&right.0.as_uuid())
                .then_with(|| {
                    left.1
                        .to_ascii_lowercase()
                        .cmp(&right.1.to_ascii_lowercase())
                })
                .then_with(|| left.2.cmp(&right.2))
                .then_with(|| left.3.cmp(&right.3))
        });
        identities.dedup();
        Ok(identities)
    }

    /// Search for models by name
    pub fn search_models(&self, pattern: &str) -> Vec<(&ModelLibrary, &DeviceModel)> {
        let pattern_lower = pattern.to_lowercase();
        let mut results = Vec::new();

        for lib in self.libraries.values() {
            for model in lib.models.values() {
                if model.name.to_lowercase().contains(&pattern_lower)
                    || model.description.to_lowercase().contains(&pattern_lower)
                {
                    if let Some(filter_type) = self.filter_type {
                        if model.model_type == filter_type {
                            results.push((lib, model));
                        }
                    } else {
                        results.push((lib, model));
                    }
                }
            }
        }

        results
    }

    /// Locate the shipped model packs, if this installation has them.
    ///
    /// Absence is normal, not a failure: the built-in library is compiled into
    /// the binary, and the browser build has no filesystem to find packs on.
    pub fn discover_spice_packs(&mut self) {
        self.spice_packs = match SpiceLibraryIndex::discover() {
            Ok(index) => index.map(Arc::new),
            Err(error) => {
                log::warn!("shipped SPICE model packs could not be read: {error}");
                None
            }
        };
    }

    /// The discovered pack index, when one is present.
    pub fn spice_packs(&self) -> Option<&SpiceLibraryIndex> {
        self.spice_packs.as_deref()
    }

    /// Selectable parts across the shipped packs, or zero when none were found.
    ///
    /// The addressable count rather than the raw definition total: two thirds
    /// of the definitions in the catalog are helper cards inside macromodel
    /// bodies, and offering those as parts would be a promise the netlist
    /// cannot keep.
    pub fn pack_definition_count(&self) -> usize {
        self.spice_packs.as_ref().map_or(0, |index| index.part_count())
    }

    /// Search the shipped packs for definitions whose name contains `query`.
    ///
    /// Bounded by `limit` because a short query matches tens of thousands of
    /// rows; the catalogue view is a browser, not a dump. An empty query
    /// returns nothing rather than everything, so opening the tab does not
    /// stream a 16 MB index off disk.
    pub fn search_pack_models(&self, query: &str, limit: usize) -> Vec<PackModelHit> {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            return Vec::new();
        }
        let Some(index) = self.spice_packs.as_ref() else {
            return Vec::new();
        };

        let entries = match index.search_parts(trimmed, limit) {
            Ok(entries) => entries,
            Err(error) => {
                log::warn!("shipped SPICE model catalog could not be searched: {error}");
                return Vec::new();
            }
        };

        entries
            .into_iter()
            .map(|entry| {
                let pack = index.pack(&entry.pack);
                PackModelHit {
                    name: entry.name.clone(),
                    kind: entry.kind.clone(),
                    device: entry.device.clone(),
                    pack_name: pack.map_or_else(|| entry.pack.clone(), |p| p.name.clone()),
                    redistributable: pack.is_some_and(|p| p.redistributable),
                    source: entry.source_path(index),
                    line: entry.line,
                    pack: entry.pack,
                }
            })
            .collect()
    }

    /// Get libraries sorted by name
    pub fn libraries_sorted(&self) -> Vec<&ModelLibrary> {
        let mut libs: Vec<_> = self.libraries.values().collect();
        libs.sort_by(|a, b| a.name.cmp(&b.name));
        libs
    }

    /// Total library count
    pub fn library_count(&self) -> usize {
        self.libraries.len()
    }

    /// Total model count across all libraries
    pub fn total_model_count(&self) -> usize {
        self.libraries.values().map(|l| l.model_count()).sum()
    }

    /// Clear all
    pub fn clear(&mut self) {
        self.libraries.clear();
        self.selected_library = None;
    }

    /// Load a library from a .lib file
    ///
    /// Parses the file using the rspice-core library parser and adds models
    /// to a new library entry.
    pub fn load_library_file(
        &mut self,
        path: impl AsRef<std::path::Path>,
        section: Option<&str>,
    ) -> Result<String, String> {
        use rspice_core::library::LibParser;

        let path = std::fs::canonicalize(path.as_ref()).map_err(|error| {
            format!(
                "Failed to resolve model library '{}': {error}",
                path.as_ref().display()
            )
        })?;
        let base_dir = path.parent().unwrap_or(std::path::Path::new("."));
        let lib_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unnamed")
            .to_string();

        // The parser captures the exact root-plus-include bytes it consumes.
        // Hash that captured closure so parsing and pinning cannot observe
        // different file versions during an explicit refresh.
        let mut parser = LibParser::new(base_dir);
        let result = parser.parse_file(&path).map_err(|error| {
            format!(
                "Failed to parse model library '{}': {error}",
                path.display()
            )
        })?;
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
        let mut source_closure = result
            .resolved_sources
            .iter()
            .map(|source| ModelSourcePin {
                path: source.path.clone(),
                digest: crate::product::ContentDigest::from_bytes(
                    Sha256::digest(source.bytes.as_ref()).into(),
                ),
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
        if let Some(unreachable) = first_unreachable_source(&path, &source_closure, &source_edges) {
            return Err(format!(
                "Model library '{}' captured dependency '{}' that is not reachable from its root by authenticated resolution edges",
                path.display(),
                unreachable.display()
            ));
        }

        if let Some(existing) = self.libraries.get(&lib_name)
            && existing.root_path.as_deref() != Some(path.as_path())
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
            .libraries
            .get(&lib_name)
            .cloned()
            .unwrap_or_else(|| ModelLibrary::new(&lib_name));
        library.root_path = Some(path.clone());
        library.source_authority = ModelSourceAuthority::External;
        library.source_closure = source_closure;
        library.source_contents = source_contents;
        library.source_edges = source_edges;
        library.models.clear();
        library.model_definition_metadata.clear();
        library.model_qualification.clear();
        library.model_correlation.clear();
        library.corners.clear();
        library.selected_corner = None;

        for section_name in result.section_names() {
            let corner = ProcessCorner {
                name: section_name.to_string(),
                description: format!("Process corner from {}", lib_name),
                file_path: Some(path.clone()),
                is_default: false,
                ..ProcessCorner::default()
            };
            library.corners.insert(corner.name.clone(), corner);
        }

        let section_names = result.section_names();
        let selected_section = if let Some(section_name) = section {
            Some(section_name.to_owned())
        } else {
            section_names
                .iter()
                .find(|name| name.eq_ignore_ascii_case("tt"))
                .or_else(|| section_names.first())
                .map(|name| (*name).to_owned())
        };

        for model in &result.top_level_models {
            let device_model = Self::convert_parsed_model(model, &path);
            library
                .models
                .insert(device_model.name.clone(), device_model);
        }

        if let Some(section_name) = selected_section.as_deref() {
            if let Some(lib_section) = result.get_section(section_name) {
                for model in &lib_section.models {
                    let device_model = Self::convert_parsed_model(model, &path);
                    library
                        .models
                        .insert(device_model.name.clone(), device_model);
                }
                library.selected_corner = Some(lib_section.name.clone());
                if let Some(corner) = library.corners.get_mut(&lib_section.name) {
                    corner.is_default = true;
                }
            } else {
                return Err(format!(
                    "Section '{}' not found. Available: {:?}",
                    section_name,
                    result.section_names()
                ));
            }
        }

        self.libraries.insert(lib_name.clone(), library);
        Ok(lib_name)
    }

    /// Import one self-contained model source from authenticated bytes.
    ///
    /// This is the browser/mobile counterpart to `load_library_file`. Includes
    /// fail closed because a single-file picker cannot prove dependency bytes;
    /// multi-file libraries must arrive in a project whose complete retained
    /// source closure was captured by the desktop importer.
    pub fn load_library_bytes(
        &mut self,
        file_name: &str,
        bytes: Vec<u8>,
        section: Option<&str>,
    ) -> Result<String, String> {
        use rspice_core::library::LibParser;

        let safe_name = std::path::Path::new(file_name)
            .file_name()
            .and_then(|name| name.to_str())
            .filter(|name| !name.is_empty())
            .ok_or_else(|| "Model library upload has no valid file name".to_owned())?;
        let lib_name = std::path::Path::new(safe_name)
            .file_stem()
            .and_then(|name| name.to_str())
            .filter(|name| !name.is_empty())
            .unwrap_or("uploaded-models")
            .to_owned();
        let digest = crate::product::ContentDigest::from_bytes(Sha256::digest(&bytes).into());
        let root = PathBuf::from(format!(
            "/rspice-browser/model-sources/{digest}/{safe_name}"
        ));
        let content = rspice_core::netlist::decode_source_bytes(&bytes)
            .map_err(|error| format!("Uploaded model source cannot be decoded: {error}"))?;
        let mut parser = LibParser::new(root.parent().unwrap_or(std::path::Path::new("/")));
        let result = parser.parse_string(&content);
        if !result.is_ok() {
            return Err(format!(
                "Uploaded model library contains parse or unresolved dependency errors: {}",
                result
                    .errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
            ));
        }

        let mut library = ModelLibrary::new(&lib_name);
        library.root_path = Some(root.clone());
        library.source_authority = ModelSourceAuthority::ProjectOwned {
            source_id: crate::product::ModelSourceId::new(),
            revision: crate::product::ObjectRevision::INITIAL,
            digest,
        };
        library.source_closure = vec![ModelSourcePin {
            path: root.clone(),
            digest,
        }];
        library.source_contents = vec![ModelSourceContent {
            path: root.clone(),
            bytes,
        }];
        library.corners.clear();
        for section_name in result.section_names() {
            let corner = ProcessCorner {
                name: section_name.to_owned(),
                description: format!("Process corner from {lib_name}"),
                file_path: Some(root.clone()),
                is_default: false,
                ..ProcessCorner::default()
            };
            library.corners.insert(corner.name.clone(), corner);
        }
        let section_names = result.section_names();
        let selected_section = section.map(str::to_owned).or_else(|| {
            section_names
                .iter()
                .find(|name| name.eq_ignore_ascii_case("tt"))
                .or_else(|| section_names.first())
                .map(|name| (*name).to_owned())
        });
        for model in &result.top_level_models {
            let device_model = Self::convert_parsed_model(model, &root);
            library
                .models
                .insert(device_model.name.clone(), device_model);
        }
        if let Some(section_name) = selected_section.as_deref() {
            let lib_section = result.get_section(section_name).ok_or_else(|| {
                format!(
                    "Section '{section_name}' not found. Available: {:?}",
                    result.section_names()
                )
            })?;
            for model in &lib_section.models {
                let device_model = Self::convert_parsed_model(model, &root);
                library
                    .models
                    .insert(device_model.name.clone(), device_model);
            }
            library.selected_corner = Some(lib_section.name.clone());
            if let Some(corner) = library.corners.get_mut(&lib_section.name) {
                corner.is_default = true;
            }
        }
        if library.models.is_empty() {
            return Err("Uploaded model library contains no supported device models".to_owned());
        }
        if self.libraries.contains_key(&lib_name) {
            return Err(format!(
                "Model library '{lib_name}' already exists; remove it before importing replacement bytes"
            ));
        }
        self.libraries.insert(lib_name.clone(), library);
        Ok(lib_name)
    }



    /// Resolve deterministic, self-contained model cards for a nominal run.
    pub fn reference_process_model_cards(
        &self,
        process: crate::simulation::dialog::corner::ProcessCorner,
    ) -> Result<Vec<String>, String> {
        self.seal_execution_sources()?
            .reference_process_model_cards(process)
    }

    /// Resolve all process-specific sources required by a PVT run.
    pub fn corner_model_bindings(
        &self,
        processes: &[CornerProcess],
    ) -> Result<Vec<CornerModelBinding>, String> {
        self.seal_execution_sources()?
            .corner_model_bindings(processes)
    }

    /// Load models from all discovered files in a PdkConfig
    ///
    /// Scans all discovered .lib and .scs files and adds them as libraries.
    pub fn load_from_pdk_config(
        &mut self,
        pdk_config: &crate::state::pdk_config::PdkConfig,
    ) -> Result<usize, Vec<String>> {
        let mut loaded = 0;
        let mut errors = Vec::new();

        for file in &pdk_config.discovered_files {
            if file.extension != "lib" && file.extension != "scs" {
                continue;
            }

            match self.load_library_file(&file.path, None) {
                Ok(_) => loaded += 1,
                Err(e) => errors.push(format!("{}: {}", file.path.display(), e)),
            }
        }

        if errors.is_empty() {
            Ok(loaded)
        } else {
            Err(errors)
        }
    }

    /// Populate with built-in models from the core engine
    ///
    /// Loads the embedded model libraries (diode.lib, mosfet.lib, etc.)
    /// into UI-accessible libraries.
    pub fn load_builtin_models(&mut self) {
        let core_manager = rspice_core::library::LibraryManager::new();

        for model_type in core_manager.available_types() {
            let models = core_manager.models_of_type(model_type);
            if models.is_empty() {
                continue;
            }

            let lib_name = model_type.display_name().to_string();
            let library = self
                .libraries
                .entry(lib_name.clone())
                .or_insert_with(|| ModelLibrary::new(&lib_name));

            for model in models {
                let device_model = DeviceModel {
                    name: model.name.clone(),
                    model_type: Self::convert_core_model_type(model.model_type),
                    spice_type: Some(Self::core_model_type_token(model.model_type).to_owned()),
                    level: ModelLevel::Unknown,
                    spice_level: None,
                    model_version: None,
                    description: model.description.clone().unwrap_or_default(),
                    l_min: model.lmin,
                    l_max: model.lmax,
                    w_min: model.wmin,
                    w_max: model.wmax,
                    vdd: None,
                    vth0: None,
                    file_path: None,
                    parameters: HashMap::new(),
                    string_parameters: HashMap::new(),
                    source_line: None,
                };
                library
                    .models
                    .insert(device_model.name.clone(), device_model);
            }
        }
    }

    /// Convert a parsed model from the core library to UI DeviceModel
    pub(crate) fn convert_parsed_model(
        model: &rspice_core::library::ParsedModel,
        file_path: &std::path::Path,
    ) -> DeviceModel {
        let model_type = Self::convert_core_model_type(model.model_type);

        DeviceModel {
            name: model.name.clone(),
            model_type,
            spice_type: Some(model.spice_type.clone()),
            level: Self::convert_model_level(model.level, &model.spice_type),
            spice_level: model.level,
            model_version: model.version,
            description: model.description.clone().unwrap_or_default(),
            l_min: model.lmin,
            l_max: model.lmax,
            w_min: model.wmin,
            w_max: model.wmax,
            vdd: None,
            vth0: None,
            file_path: Some(
                model
                    .source_file
                    .as_deref()
                    .unwrap_or(file_path)
                    .to_path_buf(),
            ),
            parameters: model.parameters.clone(),
            string_parameters: model.string_params.clone(),
            source_line: model.source_line,
        }
    }

    /// Classify what a model card *claims to be* for browsing/filtering.
    /// The card's type keyword wins over the LEVEL number because several
    /// LEVEL values are overloaded across device families (e.g. 8 is
    /// BSIM3v3 on a MOS card but HICUM/L2 on a BJT card, and 2002 is MVSG
    /// on MOS but DIODE_CMC on a diode). This is not a statement of native
    /// engine support.
    fn convert_model_level(level: Option<u32>, spice_type: &str) -> ModelLevel {
        let type_token = spice_type.trim().to_ascii_uppercase();

        // Family-named cards classify regardless of LEVEL.
        let by_name = match type_token.as_str() {
            t if t.starts_with("BSIMSOI") || t.starts_with("BSIM-SOI") => Some(ModelLevel::BsimSoi),
            t if t.starts_with("BSIMCMG") => Some(ModelLevel::BsimCmg),
            t if t.starts_with("BSIMBULK") => Some(ModelLevel::BsimBulk),
            t if t.starts_with("BSIMIMG") => Some(ModelLevel::BsimImg),
            t if t.starts_with("PSP") => Some(ModelLevel::Psp),
            t if t.starts_with("EKV") => Some(ModelLevel::Ekv),
            t if t.starts_with("HISIM") => Some(ModelLevel::HiSim),
            t if t.starts_with("L_UTSOI") || t.starts_with("LUTSOI") => Some(ModelLevel::LUtsoi),
            "MOSVAR" => Some(ModelLevel::Mosvar),
            t if t.starts_with("MVSG") => Some(ModelLevel::Mvsg),
            t if t.starts_with("VDMOS") || t == "NVDMOS" || t == "PVDMOS" => {
                Some(ModelLevel::Vdmos)
            }
            t if t.starts_with("VBIC") => Some(ModelLevel::Vbic),
            t if t.starts_with("MEXTRAM")
                || t.starts_with("BJT505")
                || t.starts_with("BJTD505") =>
            {
                Some(ModelLevel::Mextram)
            }
            t if t.starts_with("HICUM") => Some(ModelLevel::Hicum),
            t if t.starts_with("ASMHEMT")
                || t.starts_with("ANGELOV")
                || t.starts_with("EPFL_HEMT")
                || t.starts_with("EPFLHEMT") =>
            {
                Some(ModelLevel::Hemt)
            }
            "JUNCAP200" => Some(ModelLevel::Juncap),
            "DIODE_CMC" => Some(ModelLevel::DiodeCmc),
            t if t.starts_with("R2_CMC")
                || t.starts_with("R3_CMC")
                || t == "R2"
                || t == "R3"
                || t == "R2_ET" =>
            {
                Some(ModelLevel::RCmc)
            }
            _ => None,
        };
        if let Some(family) = by_name {
            return family;
        }

        let is_bjt = matches!(type_token.as_str(), "NPN" | "PNP" | "LPNP");
        let is_mos = matches!(type_token.as_str(), "NMOS" | "PMOS");
        let is_diode = matches!(type_token.as_str(), "D" | "DIODE");
        let is_resistor = matches!(type_token.as_str(), "R" | "RES" | "RESISTOR");

        match level {
            Some(4 | 9 | 11 | 12 | 13) if is_bjt => ModelLevel::Vbic,
            Some(8 | 230 | 234) if is_bjt => ModelLevel::Hicum,
            Some(504 | 505) if is_bjt => ModelLevel::Mextram,
            Some(200) if is_diode => ModelLevel::Juncap,
            Some(2002) if is_diode => ModelLevel::DiodeCmc,
            Some(1002 | 1003) if is_resistor => ModelLevel::RCmc,
            Some(1) => ModelLevel::SpiceLevel1,
            Some(3) => ModelLevel::SpiceLevel3,
            Some(8 | 49) if is_mos => ModelLevel::Bsim3v3,
            Some(14 | 54) if is_mos => ModelLevel::Bsim4,
            Some(10 | 55..=57 | 70470) if is_mos => ModelLevel::BsimSoi,
            Some(107 | 108 | 110 | 111) if is_mos => ModelLevel::BsimCmg,
            Some(104) if is_mos => ModelLevel::Psp,
            Some(260 | 301) if is_mos => ModelLevel::Ekv,
            Some(10240) if is_mos => ModelLevel::LUtsoi,
            Some(1000) if is_mos => ModelLevel::Mosvar,
            Some(2002) if is_mos => ModelLevel::Mvsg,
            Some(18) if is_mos => ModelLevel::Vdmos,
            // Preserve the historical level-only classification for cards
            // whose type keyword was not recognized.
            Some(8 | 49) => ModelLevel::Bsim3v3,
            Some(14 | 54) => ModelLevel::Bsim4,
            _ => ModelLevel::Unknown,
        }
    }

    fn core_model_type_token(model_type: rspice_core::library::ModelType) -> &'static str {
        use rspice_core::library::ModelType as CoreType;
        match model_type {
            CoreType::Nmos => "NMOS",
            CoreType::Pmos => "PMOS",
            CoreType::NpnBjt => "NPN",
            CoreType::PnpBjt => "PNP",
            CoreType::Diode => "D",
            CoreType::Resistor => "R",
            CoreType::Capacitor => "C",
            CoreType::Njfet => "NJF",
            CoreType::Pjfet => "PJF",
            CoreType::Other => "OTHER",
        }
    }

    /// Convert core ModelType to UI ModelType
    fn convert_core_model_type(core_type: rspice_core::library::ModelType) -> ModelType {
        use rspice_core::library::ModelType as CoreType;
        match core_type {
            CoreType::Nmos => ModelType::Nmos,
            CoreType::Pmos => ModelType::Pmos,
            CoreType::NpnBjt => ModelType::Npn,
            CoreType::PnpBjt => ModelType::Pnp,
            CoreType::Diode => ModelType::Diode,
            CoreType::Resistor => ModelType::Resistor,
            CoreType::Capacitor => ModelType::Capacitor,
            CoreType::Njfet | CoreType::Pjfet => ModelType::Other,
            CoreType::Other => ModelType::Other,
        }
    }
}

fn validate_project_library_name(name: &str) -> Result<(), String> {
    if name.is_empty() || name.trim() != name || name.len() > 128 {
        return Err(
            "Project model library name must contain 1 to 128 characters without outer whitespace"
                .to_owned(),
        );
    }
    if name
        .chars()
        .any(|character| character.is_control() || matches!(character, '/' | '\\'))
    {
        return Err(format!(
            "Project model library name '{name}' contains an invalid path or control character"
        ));
    }
    Ok(())
}

fn exact_subslice_offsets(haystack: &[u8], needle: &[u8]) -> Vec<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return Vec::new();
    }
    haystack
        .windows(needle.len())
        .enumerate()
        .filter_map(|(offset, candidate)| (candidate == needle).then_some(offset))
        .collect()
}

fn validate_project_owned_retained_closure(
    library: &ModelLibrary,
    root_digest: ContentDigest,
) -> Result<(), String> {
    let root = library.root_path.as_ref().ok_or_else(|| {
        format!(
            "Project-owned model library '{}' has no retained root identity",
            library.name
        )
    })?;
    if library.source_closure.is_empty()
        || library.source_closure.len() != library.source_contents.len()
    {
        return Err(format!(
            "Project-owned model library '{}' has an incomplete retained source closure",
            library.name
        ));
    }
    let mut pins = BTreeMap::new();
    for pin in &library.source_closure {
        if pins.insert(pin.path.clone(), pin.digest).is_some() {
            return Err(format!(
                "Project-owned model library '{}' repeats retained source '{}'",
                library.name,
                pin.path.display()
            ));
        }
    }
    let mut contents = BTreeMap::new();
    for content in &library.source_contents {
        if contents
            .insert(content.path.clone(), &content.bytes)
            .is_some()
        {
            return Err(format!(
                "Project-owned model library '{}' repeats retained bytes for '{}'",
                library.name,
                content.path.display()
            ));
        }
    }
    if pins.keys().ne(contents.keys()) {
        return Err(format!(
            "Project-owned model library '{}' retained pins and bytes do not describe the same closure",
            library.name
        ));
    }
    for (path, expected) in &pins {
        let bytes = contents
            .get(path)
            .expect("pin/content key equality was checked above");
        let actual = ContentDigest::from_bytes(Sha256::digest(bytes).into());
        if actual != *expected {
            return Err(format!(
                "Project-owned model source '{}' fails its retained content digest",
                path.display()
            ));
        }
    }
    if pins.get(root) != Some(&root_digest) {
        return Err(format!(
            "Project-owned model library '{}' root digest is inconsistent with its revision authority",
            library.name
        ));
    }
    if library
        .source_edges
        .iter()
        .any(|edge| !pins.contains_key(&edge.owner) || !pins.contains_key(&edge.target))
        || first_unreachable_source(root, &library.source_closure, &library.source_edges).is_some()
    {
        return Err(format!(
            "Project-owned model library '{}' has an invalid retained include graph",
            library.name
        ));
    }
    Ok(())
}

fn validate_section_qualification_evidence(
    metadata: &ModelDefinitionMetadata,
    qualification: &ModelQualificationState,
    source: &ModelSourceEvidenceBinding,
) -> Result<(), String> {
    for (index, section) in metadata.sections.iter().enumerate() {
        let evidence_digest = match &section.qualification {
            ModelSectionQualification::Qualified {
                evidence_digest: Some(evidence_digest),
            } => evidence_digest,
            ModelSectionQualification::Qualified {
                evidence_digest: None,
            } => {
                return Err(format!(
                    "Model section {:?} is qualified without an evidence digest",
                    section.name
                ));
            }
            ModelSectionQualification::Unqualified
            | ModelSectionQualification::Pending
            | ModelSectionQualification::Failed { .. } => continue,
        };
        let evidence_digest = evidence_digest.parse::<ContentDigest>().map_err(|error| {
            format!(
                "Model section {:?} has an invalid qualification evidence digest: {error}",
                section.name
            )
        })?;
        qualification
            .validate_exact_section_evidence_digest(source, &section.name, evidence_digest)
            .map_err(|error| {
                format!(
                    "Model section {:?} qualification at sections[{index}] is not backed by exact retained evidence: {error}",
                    section.name
                )
            })?;
    }
    Ok(())
}

fn reconcile_project_model_metadata(
    definition: &ProjectModelDefinition,
    previous: Option<&ModelDefinitionMetadata>,
) -> Result<ModelDefinitionMetadata, String> {
    let mut metadata = previous.cloned().unwrap_or_default();
    if !metadata.sections.is_empty() {
        return Err(
            "A sectioned model must be changed through the complete project-model revision transaction"
                .to_owned(),
        );
    }
    let previous_parameters = metadata.parameters;
    let mut parameters = Vec::with_capacity(
        definition.numeric_parameters.len() + definition.string_parameters.len(),
    );
    for (name, value) in &definition.numeric_parameters {
        let mut parameter = previous_parameters
            .iter()
            .find(|parameter| parameter.name.eq_ignore_ascii_case(name))
            .cloned()
            .unwrap_or_else(|| ParameterDefinition {
                name: name.clone(),
                data_type: ParameterDataType::Numeric,
                value: ParameterValue::Numeric(
                    FiniteF64::new(*value).expect("project definitions reject non-finite values"),
                ),
                unit: None,
                bounds: None,
                source: ParameterSource::Declared {
                    source: "project model source".to_owned(),
                },
                description: format!("Project-owned {name} model parameter"),
            });
        if parameter.data_type != ParameterDataType::Numeric {
            return Err(format!(
                "Model parameter '{name}' cannot change from string to numeric without an explicit schema migration"
            ));
        }
        parameter.name = name.clone();
        parameter.value = ParameterValue::Numeric(
            FiniteF64::new(*value).expect("project definitions reject non-finite values"),
        );
        parameters.push(parameter);
    }
    for (name, value) in &definition.string_parameters {
        let mut parameter = previous_parameters
            .iter()
            .find(|parameter| parameter.name.eq_ignore_ascii_case(name))
            .cloned()
            .unwrap_or_else(|| ParameterDefinition {
                name: name.clone(),
                data_type: ParameterDataType::String,
                value: ParameterValue::String(value.clone()),
                unit: None,
                bounds: None,
                source: ParameterSource::Declared {
                    source: "project model source".to_owned(),
                },
                description: format!("Project-owned {name} model parameter"),
            });
        if parameter.data_type != ParameterDataType::String {
            return Err(format!(
                "Model parameter '{name}' cannot change from numeric to string without an explicit schema migration"
            ));
        }
        parameter.name = name.clone();
        parameter.value = ParameterValue::String(value.clone());
        parameters.push(parameter);
    }
    parameters.sort_by(|left, right| {
        left.name
            .to_ascii_lowercase()
            .cmp(&right.name.to_ascii_lowercase())
            .then_with(|| left.name.cmp(&right.name))
    });
    metadata.parameters = parameters;
    metadata
        .validate()
        .map_err(|error| format!("Project model metadata is invalid: {error}"))?;
    Ok(metadata)
}

fn verify_project_model_round_trip(
    definition: &ProjectModelDefinition,
    parsed: &rspice_core::library::ParsedModel,
) -> Result<(), String> {
    let expected_numeric = definition
        .numeric_parameters
        .iter()
        .map(|(name, value)| (name.to_ascii_lowercase(), *value))
        .collect::<HashMap<_, _>>();
    if parsed.parameters.len() != expected_numeric.len()
        || expected_numeric.iter().any(|(name, value)| {
            parsed
                .parameters
                .get(name)
                .is_none_or(|parsed| parsed.to_bits() != value.to_bits())
        })
    {
        return Err(
            "Project model numeric parameters did not survive canonical source parsing exactly"
                .to_owned(),
        );
    }
    let expected_strings = definition
        .string_parameters
        .iter()
        .map(|(name, value)| (name.to_ascii_lowercase(), value.as_str()))
        .collect::<HashMap<_, _>>();
    if parsed.string_params.len() != expected_strings.len()
        || expected_strings.iter().any(|(name, value)| {
            parsed
                .string_params
                .get(name)
                .is_none_or(|parsed| parsed != value)
        })
    {
        return Err(
            "Project model string parameters did not survive canonical source parsing exactly"
                .to_owned(),
        );
    }
    Ok(())
}


#[cfg(test)]
mod tests;
