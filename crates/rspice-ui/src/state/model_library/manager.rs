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

    /// Definition count across the shipped packs, or zero when none were found.
    pub fn pack_definition_count(&self) -> usize {
        self.spice_packs
            .as_ref()
            .map_or(0, |index| index.definition_count())
    }

    /// Search the shipped packs for definitions whose name contains `query`.
    ///
    /// Bounded by `limit` because a short query matches tens of thousands of
    /// rows; the catalogue view is a browser, not a dump. An empty query
    /// returns nothing rather than everything, so opening the tab does not
    /// stream a 19 MB index off disk.
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

    /// Get available sections/corners from a .lib file without fully loading it
    pub fn peek_library_sections(path: impl AsRef<std::path::Path>) -> Result<Vec<String>, String> {
        rspice_core::library::LibraryManager::peek_lib_sections(path)
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
mod tests {
    use super::*;
    use crate::state::model_library::{
        CorrelationDatasetClass, CorrelationDatasetRevision, CorrelationSimulationProvenance,
        CorrelationSuite,
    };
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn model_fixture() -> (std::path::PathBuf, std::path::PathBuf) {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "rspice-model-manager-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir_all(&directory).expect("create model fixture directory");
        let path = directory.join("foundry.lib");
        fs::write(
            &path,
            ".lib TT\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n.lib FF\n.model nch NMOS (LEVEL=1 KP=2e-3)\n.endl FF\n",
        )
        .expect("write model fixture");
        (directory, path)
    }

    #[test]
    fn byte_backed_import_retains_exact_execution_authority() {
        let bytes = b".model nch NMOS (LEVEL=1 KP=1e-3)\n".to_vec();
        let mut manager = ModelLibraryManager::new();
        let name = manager
            .load_library_bytes("browser-models.lib", bytes.clone(), None)
            .expect("self-contained byte source imports");
        let library = manager.get_library(&name).expect("library retained");
        assert_eq!(library.source_closure.len(), 1);
        assert_eq!(library.source_contents.len(), 1);
        assert_eq!(library.source_contents[0].bytes, bytes);
        let binding = crate::state::ProjectTechnologyBinding::from_model_library(library)
            .expect("byte-backed library is attachable");
        manager
            .validate_attached_technology(Some(&binding))
            .expect("unchanged byte-backed catalog matches attachment");
    }

    #[test]
    fn authenticated_root_expands_retained_model_include_closure_without_filesystem_lookup() {
        let (directory, path) = model_fixture();
        let child = directory.join("device.inc");
        fs::write(&child, ".model sealed_n NMOS (LEVEL=1 KP=7e-3)\n")
            .expect("write nested model source");
        fs::write(
            &path,
            ".include device.inc\n.lib TT\n.model root_n NMOS (LEVEL=1)\n.endl TT\n",
        )
        .expect("write model root");

        let mut manager = ModelLibraryManager::new();
        manager
            .load_library_file(&path, Some("TT"))
            .expect("import authenticated model closure");
        let sealed = manager
            .seal_execution_sources()
            .expect("seal exact model bytes");
        let deck = directory.join("browser-root.cir");
        let source = "browser root\n.lib \"foundry.lib\" TT\nM1 d g 0 0 sealed_n\n.end\n";

        let (expanded, dependencies) = sealed
            .expand_root_dependencies(&deck, source, &rspice_core::abort_signal::NoAbort)
            .expect("expand through authenticated bundle");

        assert!(expanded.contains("sealed_n"), "{expanded}");
        assert!(expanded.lines().all(|line| {
            rspice_core::netlist::parse_include_directive(line).is_none()
                && !rspice_core::netlist::parse_lib_directive(line)
                    .is_some_and(|(_, section)| section.is_some())
        }));
        assert_eq!(dependencies.len(), 2);

        fs::remove_dir_all(directory).expect("remove authenticated expansion fixture");
    }

    #[test]
    fn authenticated_root_rejects_missing_or_tampered_retained_sources() {
        let (directory, path) = model_fixture();
        let mut manager = ModelLibraryManager::new();
        let name = manager
            .load_library_file(&path, None)
            .expect("import authenticated model source");
        manager
            .get_library_mut(&name)
            .expect("library exists")
            .source_contents[0]
            .bytes
            .push(b' ');
        let tamper = manager
            .seal_execution_sources()
            .expect_err("retained byte tamper must fail closed");
        assert!(
            tamper.contains("do not match the accepted digest"),
            "{tamper}"
        );

        let mut manager = ModelLibraryManager::new();
        manager
            .load_library_file(&path, None)
            .expect("re-import clean source");
        let sealed = manager.seal_execution_sources().expect("seal clean source");
        let missing = sealed
            .bundle_for_root(
                &directory.join("browser-root.cir"),
                "browser root\n.include missing.lib\n.end\n",
            )
            .expect_err("unretained dependency must fail closed");
        assert!(
            missing.contains("not present in the authenticated"),
            "{missing}"
        );

        fs::remove_dir_all(directory).expect("remove authenticated failure fixture");
    }

    #[test]
    fn loaded_sections_resolve_to_exact_reference_and_corner_bindings() {
        let (directory, path) = model_fixture();
        let mut manager = ModelLibraryManager::new();
        manager
            .load_library_file(&path, None)
            .expect("load sectioned model library");

        let reference = manager
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::FF)
            .expect("FF binding exists");
        let bindings = manager
            .corner_model_bindings(&[CornerProcess::TT, CornerProcess::FF])
            .expect("TT and FF bindings exist");

        assert_eq!(reference.len(), 1);
        assert!(reference[0].contains("RSpice sealed model source"));
        assert!(reference[0].contains("KP=2e-3"));
        assert!(reference[0].lines().all(|line| {
            rspice_core::netlist::parse_lib_directive(line).is_none()
                && rspice_core::netlist::parse_include_directive(line).is_none()
        }));
        assert_eq!(bindings.len(), 2);
        assert_eq!(bindings[0].process, CornerProcess::TT);
        assert_eq!(bindings[0].section.as_deref(), Some("TT"));
        assert!(bindings[0].materialized_model_cards.contains("KP=1e-3"));
        assert_eq!(bindings[1].process, CornerProcess::FF);
        assert_eq!(bindings[1].section.as_deref(), Some("FF"));
        assert!(bindings[1].materialized_model_cards.contains("KP=2e-3"));

        let error = manager
            .corner_model_bindings(&[CornerProcess::SS])
            .expect_err("undefined SS section must fail closed");
        assert!(error.contains("does not define the SS process section"));
        fs::remove_dir_all(directory).expect("remove model fixture directory");
    }

    #[test]
    fn failed_section_refresh_is_transactional() {
        let (directory, path) = model_fixture();
        let mut manager = ModelLibraryManager::new();
        let name = manager
            .load_library_file(&path, Some("TT"))
            .expect("load TT section");
        let before = manager
            .get_library(&name)
            .expect("loaded library exists")
            .clone();

        let error = manager
            .load_library_file(&path, Some("MISSING"))
            .expect_err("missing section must fail");

        assert!(error.contains("Section 'MISSING' not found"));
        let after = manager.get_library(&name).expect("library remains loaded");
        assert_eq!(after.selected_corner, before.selected_corner);
        assert_eq!(after.models.len(), before.models.len());
        assert_eq!(after.source_closure, before.source_closure);
        assert_eq!(after.source_edges, before.source_edges);
        fs::remove_dir_all(directory).expect("remove model fixture directory");
    }

    #[test]
    fn explicit_refresh_atomically_accepts_new_source_closure() {
        let (directory, path) = model_fixture();
        let mut manager = ModelLibraryManager::new();
        let name = manager
            .load_library_file(&path, Some("TT"))
            .expect("load original source");
        let original = manager
            .get_library(&name)
            .expect("original library exists")
            .source_closure
            .clone();
        assert_eq!(original.len(), 1);

        fs::write(
            &path,
            ".lib TT\n.model nch NMOS (LEVEL=1 KP=7e-3)\n.endl TT\n.lib FF\n.model nch NMOS (LEVEL=1 KP=8e-3)\n.endl FF\n",
        )
        .expect("replace source content");
        let blocked = manager
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect_err("unaccepted source change must block");
        assert!(blocked.contains("dependency changed at"));

        manager
            .load_library_file(&path, Some("TT"))
            .expect("explicit refresh accepts replacement");
        let refreshed = manager
            .get_library(&name)
            .expect("refreshed library exists")
            .source_closure
            .clone();

        assert_ne!(refreshed, original);
        assert_eq!(
            refreshed[0].digest,
            ModelLibraryManager::calculate_source_digest(&path)
                .expect("current source digest computes")
        );
        manager
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect("refreshed source binds");

        fs::remove_dir_all(directory).expect("remove model fixture directory");
    }

    #[test]
    fn transitive_include_change_blocks_until_explicit_refresh() {
        let (directory, path) = model_fixture();
        let dependency = directory.join("device.inc");
        fs::write(&dependency, ".model included_nch NMOS (LEVEL=1 KP=1e-3)\n")
            .expect("write included source");
        fs::write(
            &path,
            ".include \"device.inc\"\n.lib TT\n.model nch NMOS (LEVEL=1 KP=2e-3)\n.endl TT\n",
        )
        .expect("write root with include");

        let mut manager = ModelLibraryManager::new();
        let name = manager
            .load_library_file(&path, Some("TT"))
            .expect("load transitive source closure");
        let accepted = manager
            .get_library(&name)
            .expect("library exists")
            .source_closure
            .clone();
        assert_eq!(accepted.len(), 2);
        assert_eq!(
            manager
                .get_library(&name)
                .expect("library exists")
                .source_edges
                .len(),
            1
        );

        fs::write(&dependency, ".model included_nch NMOS (LEVEL=1 KP=9e-3)\n")
            .expect("change only included source");
        let blocked = manager
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect_err("changed transitive dependency must block");
        assert!(blocked.contains("device.inc"));
        assert!(blocked.contains("dependency changed"));

        manager
            .load_library_file(&path, Some("TT"))
            .expect("explicit refresh accepts new dependency closure");
        let refreshed = &manager
            .get_library(&name)
            .expect("refreshed library exists")
            .source_closure;
        assert_ne!(refreshed, &accepted);
        manager
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect("refreshed transitive source binds");

        fs::remove_dir_all(directory).expect("remove model fixture directory");
    }

    #[test]
    fn external_lib_section_dependency_is_part_of_the_pinned_closure() {
        let (directory, path) = model_fixture();
        let dependency = directory.join("sectioned models.lib");
        fs::write(
            &dependency,
            ".lib TT\n.model child_nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n",
        )
        .expect("write external library dependency");
        fs::write(&path, ".lib \"sectioned models.lib\" TT\n")
            .expect("write external library wrapper");

        let mut manager = ModelLibraryManager::new();
        let name = manager
            .load_library_file(&path, Some("TT"))
            .expect("load external library dependency");
        let accepted = &manager
            .get_library(&name)
            .expect("library exists")
            .source_closure;
        assert_eq!(accepted.len(), 2);
        assert!(
            accepted
                .iter()
                .any(|source| source.path.ends_with("sectioned models.lib"))
        );

        fs::write(
            &dependency,
            ".lib TT\n.model child_nch NMOS (LEVEL=1 KP=8e-3)\n.endl TT\n",
        )
        .expect("change external library dependency");
        let blocked = manager
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect_err("changed external .lib dependency must block");
        assert!(blocked.contains("sectioned models.lib"));
        assert!(blocked.contains("dependency changed"));

        fs::remove_dir_all(directory).expect("remove model fixture directory");
    }

    #[test]
    fn cyclic_include_is_rejected_with_owning_source_and_no_partial_library() {
        let (directory, path) = model_fixture();
        let dependency = directory.join("cycle.inc");
        fs::write(
            &path,
            ".include \"cycle.inc\"\n.lib TT\n.model nch NMOS (LEVEL=1)\n.endl TT\n",
        )
        .expect("write root cycle member");
        fs::write(&dependency, ".include \"foundry.lib\"\n")
            .expect("write dependency cycle member");

        let mut manager = ModelLibraryManager::new();
        let error = manager
            .load_library_file(&path, Some("TT"))
            .expect_err("cycle must fail closed");

        assert!(error.contains("Cyclic include dependency"));
        assert!(error.contains("cycle.inc:1"));
        assert_eq!(manager.library_count(), 0);

        fs::remove_dir_all(directory).expect("remove model fixture directory");
    }

    #[test]
    fn sealed_snapshot_survives_mutation_and_deletion_without_reopening_sources() {
        let (directory, path) = model_fixture();
        let dependency = directory.join("device.inc");
        fs::write(&dependency, ".model sealed_n NMOS (LEVEL=1 KP=1e-3)\n")
            .expect("write sealed dependency");
        fs::write(
            &path,
            ".include \"device.inc\"\n.lib TT\n.model root_n NMOS (LEVEL=1 KP=2e-3)\n.endl TT\n",
        )
        .expect("write sealed root");

        let mut manager = ModelLibraryManager::new();
        manager
            .load_library_file(&path, Some("TT"))
            .expect("load sealed fixture");
        let snapshot = manager
            .seal_execution_sources()
            .expect("authenticate one immutable run snapshot");

        fs::write(&dependency, ".model sealed_n NMOS (LEVEL=1 KP=9e-3)\n")
            .expect("mutate dependency after sealing");
        fs::remove_file(&path).expect("delete root after sealing");

        let cards = snapshot
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect("materialization uses only sealed bytes");
        let cards = cards.join("\n");
        assert!(cards.contains("KP=1e-3"), "{cards}");
        assert!(cards.contains("KP=2e-3"), "{cards}");
        assert!(!cards.contains("KP=9e-3"), "{cards}");
        rspice_core::Netlist::parse(&format!("sealed worker deck\n{cards}\n.end\n"))
            .expect("self-contained sealed cards parse without source files");

        let fresh_error = manager
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect_err("a new run snapshot must reject the changed/deleted closure");
        assert!(
            fresh_error.contains("changed") || fresh_error.contains("unavailable"),
            "{fresh_error}"
        );

        fs::remove_dir_all(directory).expect("remove sealed fixture directory");
    }

    #[test]
    fn existing_dependency_without_authenticated_edge_is_rejected() {
        let (directory, path) = model_fixture();
        let dependency = directory.join("device.inc");
        fs::write(&dependency, ".model edge_n NMOS (LEVEL=1)\n").expect("write dependency");
        fs::write(&path, ".include device.inc\n.lib TT\n.endl TT\n").expect("write root");

        let mut manager = ModelLibraryManager::new();
        let name = manager
            .load_library_file(&path, Some("TT"))
            .expect("load dependency graph");
        manager
            .get_library_mut(&name)
            .expect("library exists")
            .source_edges
            .clear();

        let error = manager
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect_err("filesystem presence must not substitute for a missing edge");
        assert!(
            error.contains("no authenticated resolution edge"),
            "{error}"
        );
        assert!(dependency.is_file(), "dependency remains tempting on disk");

        fs::remove_dir_all(directory).expect("remove edge fixture directory");
    }

    #[test]
    fn disconnected_pinned_member_is_rejected_before_any_filesystem_probe() {
        let (directory, path) = model_fixture();
        let mut manager = ModelLibraryManager::new();
        let name = manager
            .load_library_file(&path, Some("TT"))
            .expect("load source root");
        let orphan = directory.join("must-never-be-probed.inc");
        assert!(!orphan.exists());

        let library = manager.get_library_mut(&name).expect("library exists");
        library.source_closure.push(ModelSourcePin {
            path: orphan.clone(),
            digest: crate::product::ContentDigest::from_bytes([0xa5; 32]),
        });
        library.source_edges.push(ModelSourceEdge {
            owner: orphan.clone(),
            requested_path: "must-never-be-probed.inc".to_owned(),
            target: orphan.clone(),
        });

        let error = manager
            .seal_execution_sources()
            .expect_err("a disconnected authenticated subgraph must fail closed");
        assert!(error.contains("not reachable from root"), "{error}");
        assert!(
            !error.contains("unavailable"),
            "reachability must be checked before filesystem availability: {error}"
        );

        fs::remove_dir_all(directory).expect("remove disconnected fixture directory");
    }

    #[test]
    fn captured_search_precedence_is_frozen_in_the_run_snapshot() {
        let (directory, path) = model_fixture();
        let subdirectory = directory.join("sub");
        fs::create_dir_all(&subdirectory).expect("create search-precedence directory");
        let first = subdirectory.join("first.inc");
        let local = subdirectory.join("shared.inc");
        let fallback = directory.join("shared.inc");
        fs::write(&first, ".incl shared.inc\n").expect("write nested include");
        fs::write(&local, ".model local_n NMOS (LEVEL=1 KP=1e-3)\n").expect("write local winner");
        fs::write(&fallback, ".model fallback_n NMOS (LEVEL=1 KP=9e-3)\n")
            .expect("write top-level fallback");
        fs::write(
            &path,
            ".include sub/first.inc\n.lib TT\n.model root_n NMOS (LEVEL=1)\n.endl TT\n",
        )
        .expect("write root");

        let mut manager = ModelLibraryManager::new();
        let name = manager
            .load_library_file(&path, Some("TT"))
            .expect("capture local-first resolution");
        let canonical_local = fs::canonicalize(&local).expect("canonical local path");
        assert!(
            manager
                .get_library(&name)
                .expect("library exists")
                .source_edges
                .iter()
                .any(|edge| edge.requested_path == "shared.inc" && edge.target == canonical_local)
        );
        let snapshot = manager
            .seal_execution_sources()
            .expect("seal captured precedence");
        fs::remove_file(&local).expect("remove original local winner");

        let cards = snapshot
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect("snapshot retains captured local winner")
            .join("\n");
        assert!(cards.contains("local_n"), "{cards}");
        assert!(!cards.contains("fallback_n"), "{cards}");

        fs::remove_dir_all(directory).expect("remove precedence fixture directory");
    }

    #[test]
    fn raw_byte_digest_and_supported_encoding_decode_share_one_read() {
        let (directory, path) = model_fixture();
        let source = ".lib TT\n.model utf16_n NMOS (LEVEL=1 KP=3e-3)\n.endl TT\n";
        let mut bytes = vec![0xff, 0xfe];
        bytes.extend(
            source
                .encode_utf16()
                .flat_map(u16::to_le_bytes)
                .collect::<Vec<_>>(),
        );
        fs::write(&path, &bytes).expect("write UTF-16LE model source");

        let mut manager = ModelLibraryManager::new();
        let name = manager
            .load_library_file(&path, Some("TT"))
            .expect("supported source encoding imports");
        let pin = manager
            .get_library(&name)
            .expect("library exists")
            .source_closure[0]
            .digest;
        assert_eq!(
            pin,
            crate::product::ContentDigest::from_bytes(Sha256::digest(&bytes).into())
        );
        let cards = manager
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect("verified raw bytes decode from memory")
            .join("\n");
        assert!(cards.contains("utf16_n"), "{cards}");

        fs::remove_dir_all(directory).expect("remove encoding fixture directory");
    }

    fn project_definition(vth0: f64, tag: &str) -> ProjectModelDefinition {
        ProjectModelDefinition {
            name: "owned_nch".to_owned(),
            spice_type: "NMOS".to_owned(),
            description: "Project-owned regression model".to_owned(),
            numeric_parameters: BTreeMap::from([
                ("level".to_owned(), 1.0),
                ("vth0".to_owned(), vth0),
            ]),
            string_parameters: BTreeMap::from([("revision_tag".to_owned(), tag.to_owned())]),
        }
    }

    fn current_model_source(
        library: &ModelLibrary,
    ) -> (
        ModelSourceId,
        ObjectRevision,
        ObjectRevision,
        ContentDigest,
        ModelSourceEvidenceBinding,
    ) {
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision: library_revision,
            ..
        } = library.source_authority
        else {
            panic!("fixture model must be project-owned");
        };
        let model = &library.models["owned_nch"];
        let metadata = library.model_definition_metadata["owned_nch"].clone();
        let definition = ProjectModelRevisionDefinition::new(
            ProjectModelDefinition::from_device_model(model),
            metadata,
        );
        let canonical = definition.canonical_source().unwrap();
        let model_digest = ContentDigest::from_bytes(Sha256::digest(canonical.as_bytes()).into());
        let model_revision = definition
            .project_source_identity()
            .unwrap()
            .expect("project source identity")
            .revision;
        let binding = ModelSourceEvidenceBinding::try_new_project_bound(
            "owned_nch",
            source_id,
            model_digest,
            model_revision,
        )
        .unwrap();
        (
            source_id,
            library_revision,
            model_revision,
            model_digest,
            binding,
        )
    }

    fn correlation_suite(
        source: ModelSourceEvidenceBinding,
        revision: ObjectRevision,
    ) -> CorrelationSuite {
        let reference_bytes = b"id,quantity,value,unit\nr1,gain,1,V\n".to_vec();
        let simulation_bytes = b"id,quantity,value,unit\ns1,gain,1,V\n".to_vec();
        let reference = CorrelationDatasetRevision::try_from_csv(
            "reference",
            ObjectRevision::INITIAL,
            "Reference",
            CorrelationDatasetClass::BenchMeasurement,
            "test authority",
            "lot-1",
            "fixture-1",
            "calibration-1",
            "reference.csv",
            reference_bytes,
            None,
        )
        .unwrap();
        let simulation_digest = ContentDigest::from_bytes(Sha256::digest(&simulation_bytes).into());
        let simulation = CorrelationDatasetRevision::try_from_csv_with_provenance(
            "simulation",
            ObjectRevision::INITIAL,
            "Simulation",
            CorrelationDatasetClass::ModelSimulation,
            "RSpice",
            "owned_nch",
            "retained-plan",
            "numeric-contract",
            "simulation.csv",
            simulation_bytes,
            Some(source.clone()),
            Some(CorrelationSimulationProvenance {
                run_id: "run-1".to_owned(),
                run_dataset_id: "dataset-1".to_owned(),
                analysis_id: 1,
                analysis_result_digest: ContentDigest::from_bytes([0x40; 32]),
                plan_id: "plan-1".to_owned(),
                project_revision: ObjectRevision::INITIAL,
                prepared_snapshot_digest: ContentDigest::from_bytes([0x41; 32]),
                source_content_digest: ContentDigest::from_bytes([0x42; 32]),
                task_config_digest: ContentDigest::from_bytes([0x43; 32]),
                execution_target: "Local desktop engine".to_owned(),
                export_digest: simulation_digest,
                model_source: source.clone(),
                executed_at_unix_ms: 1,
            }),
        )
        .unwrap();
        CorrelationSuite::try_new(
            "owned-nch-correlation",
            revision,
            "Owned NCH correlation",
            "model-owner",
            source,
            vec![reference, simulation],
            Vec::new(),
            Vec::new(),
        )
        .unwrap()
    }

    #[test]
    fn project_model_correlation_commit_is_guarded_append_only_and_source_bound() {
        let mut manager = ModelLibraryManager::new();
        let created = manager
            .create_project_model("owned_models", &project_definition(0.48, "r1"))
            .expect("create project model");
        let (source_id, library_revision, model_revision, model_digest, current_source) =
            current_model_source(&created.after);
        let first_suite = correlation_suite(current_source.clone(), ObjectRevision::INITIAL);
        let first_state =
            ModelCorrelationState::try_new(vec![first_suite.clone()], Vec::new()).unwrap();
        manager
            .replace_project_model_correlation(
                "owned_models",
                source_id,
                library_revision,
                model_revision,
                model_digest,
                "owned_nch",
                &first_state,
            )
            .expect("first correlation revision commits without changing model bytes");

        let second_revision = ObjectRevision::INITIAL.next().unwrap();
        let second_suite = correlation_suite(current_source.clone(), second_revision);
        let second_state = ModelCorrelationState::try_new(
            vec![first_suite.clone(), second_suite.clone()],
            Vec::new(),
        )
        .unwrap();
        let appended = manager
            .replace_project_model_correlation(
                "owned_models",
                source_id,
                library_revision,
                model_revision,
                model_digest,
                "owned_nch",
                &second_state,
            )
            .expect("suite history appends atomically");
        assert!(!appended.affects_execution);

        let deleted_history =
            ModelCorrelationState::try_new(vec![second_suite.clone()], Vec::new()).unwrap();
        let error = manager
            .replace_project_model_correlation(
                "owned_models",
                source_id,
                library_revision,
                model_revision,
                model_digest,
                "owned_nch",
                &deleted_history,
            )
            .unwrap_err();
        assert!(error.contains("immutable") && error.contains("cannot be removed"));

        let stale_source = ModelSourceEvidenceBinding::try_new_project_bound(
            "owned_nch",
            source_id,
            ContentDigest::from_bytes([0xee; 32]),
            model_revision,
        )
        .unwrap();
        let third_revision = second_revision.next().unwrap();
        let stale_suite = correlation_suite(stale_source, third_revision);
        let stale_state = ModelCorrelationState::try_new(
            vec![first_suite, second_suite, stale_suite],
            Vec::new(),
        )
        .unwrap();
        let error = manager
            .replace_project_model_correlation(
                "owned_models",
                source_id,
                library_revision,
                model_revision,
                model_digest,
                "owned_nch",
                &stale_state,
            )
            .unwrap_err();
        assert!(error.contains("exact current model source revision"));

        let wrong_library_revision = library_revision.next().unwrap();
        let error = manager
            .replace_project_model_correlation(
                "owned_models",
                source_id,
                wrong_library_revision,
                model_revision,
                model_digest,
                "owned_nch",
                &second_state,
            )
            .unwrap_err();
        assert!(error.contains("changed after correlation review began"));
    }

    #[test]
    fn project_model_create_and_replace_publish_exact_retained_execution_bytes() {
        let mut manager = ModelLibraryManager::new();
        let created = manager
            .create_project_model("owned_models", &project_definition(0.48, "r1"))
            .expect("create project model");
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            digest: first_digest,
        } = created.after.source_authority
        else {
            panic!("created model must be project-owned")
        };
        assert_eq!(revision, ObjectRevision::INITIAL);
        assert_eq!(
            created.after.models["owned_nch"].string_parameters["revision_tag"],
            "r1"
        );

        let sealed = manager
            .seal_execution_sources_with_reader(|path| {
                panic!(
                    "project-owned desktop sealing must not read {}",
                    path.display()
                )
            })
            .expect("retained project bytes seal");
        assert_eq!(sealed.sources.len(), 1);
        assert!(sealed.sources[0].1.contains("VTH0=0.48"));
        assert!(sealed.sources[0].1.contains("REVISION_TAG=\"r1\""));

        let replaced = manager
            .replace_project_model(
                "owned_models",
                source_id,
                revision,
                &project_definition(0.51, "r2"),
            )
            .expect("replace project model");
        let ModelSourceAuthority::ProjectOwned {
            revision: second_revision,
            digest: second_digest,
            ..
        } = replaced.after.source_authority
        else {
            panic!("replacement must remain project-owned")
        };
        assert_eq!(second_revision.get(), 2);
        assert_ne!(first_digest, second_digest);
        assert_eq!(replaced.after.models["owned_nch"].parameters["vth0"], 0.51);
        assert_eq!(
            replaced.after.models["owned_nch"].string_parameters["revision_tag"],
            "r2"
        );
    }

    #[test]
    fn project_model_replacement_is_guarded_and_atomic() {
        let mut manager = ModelLibraryManager::new();
        let created = manager
            .create_project_model("owned_models", &project_definition(0.48, "r1"))
            .expect("create project model");
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            ..
        } = created.after.source_authority
        else {
            panic!("created model must be project-owned")
        };
        let original = created.after.source_contents[0].bytes.clone();

        let stale = manager
            .replace_project_model(
                "owned_models",
                ModelSourceId::new(),
                revision,
                &project_definition(0.52, "r2"),
            )
            .expect_err("stale identity must fail");
        assert!(stale.contains("changed after this candidate was opened"));
        assert_eq!(
            manager.get_library("owned_models").unwrap().source_contents[0].bytes,
            original
        );

        let no_op = manager
            .replace_project_model(
                "owned_models",
                source_id,
                revision,
                &project_definition(0.48, "r1"),
            )
            .expect_err("unchanged source must not create a revision");
        assert!(no_op.contains("no source changes"));

        let mut invalid = project_definition(f64::NAN, "r2");
        invalid
            .string_parameters
            .insert("VTH0".to_owned(), "duplicate".to_owned());
        let invalid_error = manager
            .replace_project_model("owned_models", source_id, revision, &invalid)
            .expect_err("invalid candidate must fail before publication");
        assert!(
            invalid_error.contains("more than once") || invalid_error.contains("finite"),
            "{invalid_error}"
        );
        assert_eq!(
            manager.get_library("owned_models").unwrap().source_contents[0].bytes,
            original
        );
    }

    fn sectioned_project_revision(vth0: f64) -> ProjectModelRevisionDefinition {
        let base = project_definition(vth0, "r1");
        let metadata = reconcile_project_model_metadata(&base, None)
            .expect("synthesize typed project metadata");
        let mut definition = ProjectModelRevisionDefinition::new(base, metadata);
        definition
            .metadata
            .sections
            .push(crate::state::model_library::ModelSectionDefinition {
                name: "TT".to_owned(),
                parent: None,
                overrides: BTreeMap::from([(
                    "vth0".to_owned(),
                    ParameterValue::Numeric(FiniteF64::new(0.49).expect("finite fixture")),
                )]),
                model_files: Vec::new(),
                qualification: crate::state::model_library::ModelSectionQualification::Unqualified,
            });
        definition
    }

    #[test]
    fn complete_project_revision_publishes_sections_and_executes_selected_corner() {
        let mut manager = ModelLibraryManager::new();
        let created = manager
            .create_project_model_revision(
                "owned_sections",
                &sectioned_project_revision(0.48),
                &ModelQualificationState::default(),
            )
            .expect("create complete model revision");
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            digest,
        } = created.after.source_authority
        else {
            panic!("complete revision must be project-owned")
        };
        assert_eq!(revision, ObjectRevision::INITIAL);
        assert_eq!(created.after.selected_corner.as_deref(), Some("TT"));
        assert_eq!(created.after.corners.len(), 1);
        let metadata = &created.after.model_definition_metadata["owned_nch"];
        assert_eq!(metadata.sections[0].model_files.len(), 1);
        assert_eq!(metadata.sections[0].model_files[0].revision, 1);
        assert_eq!(
            metadata.sections[0].model_files[0].content_digest,
            digest.to_string()
        );

        let cards = manager
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect("materialize retained TT section")
            .join("\n");
        assert!(cards.contains("VTH0=0.49"), "{cards}");

        let mut metadata_only =
            ProjectModelRevisionDefinition::new(project_definition(0.48, "r1"), metadata.clone());
        metadata_only.metadata.parameters[0].unit = Some("dimensionless".to_owned());
        let replaced = manager
            .replace_project_model_revision(
                "owned_sections",
                source_id,
                revision,
                &metadata_only,
                &ModelQualificationState::default(),
            )
            .expect("metadata-only change creates a complete revision");
        assert_eq!(
            replaced.after.project_source_revision(),
            Some(ObjectRevision::new(2).expect("second revision"))
        );
        assert_eq!(
            replaced.after.source_contents[0].bytes,
            created.after.source_contents[0].bytes
        );
        assert_eq!(
            replaced.after.model_definition_metadata["owned_nch"].parameters[0]
                .unit
                .as_deref(),
            Some("dimensionless")
        );
    }

    #[test]
    fn project_model_tamper_fails_before_any_external_read() {
        let mut manager = ModelLibraryManager::new();
        manager
            .create_project_model("owned_models", &project_definition(0.48, "r1"))
            .expect("create project model");
        manager
            .get_library_mut("owned_models")
            .unwrap()
            .source_contents[0]
            .bytes
            .push(b' ');
        let error = manager
            .seal_execution_sources_with_reader(|path| {
                panic!(
                    "tampered project source must fail before reading {}",
                    path.display()
                )
            })
            .expect_err("tampered retained bytes must fail");
        assert!(
            error.contains("do not match the accepted digest"),
            "{error}"
        );
    }

    #[cfg(unix)]
    #[test]
    fn persisted_symlink_edge_survives_alias_removal_after_sealing() {
        use std::os::unix::fs::symlink;

        let (directory, path) = model_fixture();
        let target = directory.join("real.inc");
        let alias = directory.join("alias.inc");
        fs::write(&target, ".model symlink_n NMOS (LEVEL=1)\n").expect("write symlink target");
        symlink(&target, &alias).expect("create symlink alias");
        fs::write(&path, ".include alias.inc\n.lib TT\n.endl TT\n").expect("write symlink root");

        let mut manager = ModelLibraryManager::new();
        manager
            .load_library_file(&path, Some("TT"))
            .expect("capture symlink resolution");
        let snapshot = manager
            .seal_execution_sources()
            .expect("seal symlink target bytes");
        fs::remove_file(&alias).expect("remove alias after sealing");
        let cards = snapshot
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect("authenticated edge no longer needs symlink")
            .join("\n");
        assert!(cards.contains("symlink_n"), "{cards}");

        fs::remove_dir_all(directory).expect("remove symlink fixture directory");
    }

    //=========================================================================
    // Shipped model pack discovery
    //=========================================================================

    /// Open the repository's own model tree, so these tests do not depend on
    /// where the test binary happens to sit.
    fn repo_pack_manager() -> ModelLibraryManager {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../models/spice");
        let index = rspice_core::library::SpiceLibraryIndex::open(root)
            .expect("repository model tree opens");
        let mut manager = ModelLibraryManager::new();
        manager.spice_packs = Some(std::sync::Arc::new(index));
        manager
    }

    #[test]
    fn pack_search_finds_definitions_the_libraries_do_not_hold() {
        let manager = repo_pack_manager();
        assert!(manager.pack_definition_count() > 100_000);

        // Nothing is loaded, so a plain library search finds nothing...
        assert!(manager.search_models("2N3819").is_empty());
        // ...but the shipped packs carry it.
        let hits = manager.search_pack_models("2N3819", 50);
        assert!(!hits.is_empty(), "expected 2N3819 in the shipped packs");
        let hit = &hits[0];
        assert!(hit.source.as_ref().is_some_and(|p| p.is_file()));
        assert!(hit.line > 0);
    }

    #[test]
    fn pack_search_is_bounded_and_ignores_an_empty_query() {
        let manager = repo_pack_manager();
        // An empty query must not stream the whole 19 MB index.
        assert!(manager.search_pack_models("", 50).is_empty());
        assert!(manager.search_pack_models("   ", 50).is_empty());

        // A broad query is capped at the caller's limit.
        let hits = manager.search_pack_models("1N", 25);
        assert_eq!(hits.len(), 25);
    }

    #[test]
    fn pack_hits_carry_their_redistribution_status() {
        let manager = repo_pack_manager();
        let hits = manager.search_pack_models("nfet_01v8", 20);
        assert!(!hits.is_empty(), "expected sky130 devices in the packs");
        // sky130 is Apache-2.0, so its rows must not be flagged unlicensed.
        assert!(
            hits.iter().any(|hit| hit.redistributable),
            "expected at least one redistributable hit"
        );
    }

    #[test]
    fn missing_pack_tree_is_not_an_error() {
        // The browser build has no packs, and a source checkout may not have
        // synced them. Both must degrade to an empty search, not a failure.
        let manager = ModelLibraryManager::new();
        assert_eq!(manager.pack_definition_count(), 0);
        assert!(manager.search_pack_models("2N3904", 10).is_empty());
        assert!(manager.spice_packs().is_none());
    }
}
