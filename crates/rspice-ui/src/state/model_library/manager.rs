use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

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

    /// Create a new single-card model whose exact source is owned by the
    /// project. The candidate is rendered, parsed, and checked completely
    /// before the manager is mutated.
    pub fn create_project_model(
        &mut self,
        library_name: &str,
        definition: &ProjectModelDefinition,
    ) -> Result<ProjectModelCommit, String> {
        validate_project_library_name(library_name)?;
        if let Some(existing) = self
            .libraries
            .keys()
            .find(|existing| existing.eq_ignore_ascii_case(library_name))
        {
            return Err(format!(
                "Model library '{library_name}' conflicts with existing library '{existing}'"
            ));
        }

        let source_id = ModelSourceId::new();
        let revision = ObjectRevision::INITIAL;
        let root = super::project_owned_source_path(source_id);
        let after = Self::build_project_model_library(
            library_name,
            None,
            source_id,
            revision,
            root,
            definition,
        )?;
        let model_name = definition.name.clone();
        self.libraries
            .insert(library_name.to_owned(), after.clone());
        Ok(ProjectModelCommit {
            library_name: library_name.to_owned(),
            model_name,
            before: None,
            after,
            affects_execution: true,
        })
    }

    /// Create one complete project-owned model revision. The base card,
    /// process sections, typed schema, statistical definition, and temperature
    /// laws are validated and published with one source identity or not at
    /// all.
    pub fn create_project_model_revision(
        &mut self,
        library_name: &str,
        definition: &ProjectModelRevisionDefinition,
        qualification: &ModelQualificationState,
    ) -> Result<ProjectModelCommit, String> {
        validate_project_library_name(library_name)?;
        if let Some(existing) = self
            .libraries
            .keys()
            .find(|existing| existing.eq_ignore_ascii_case(library_name))
        {
            return Err(format!(
                "Model library '{library_name}' conflicts with existing library '{existing}'"
            ));
        }

        let source_id = ModelSourceId::new();
        let revision = ObjectRevision::INITIAL;
        let root = super::project_owned_source_path(source_id);
        let after = Self::build_project_model_revision_library(
            library_name,
            None,
            source_id,
            revision,
            root,
            definition,
            qualification,
        )?;
        let model_name = definition.base.name.clone();
        self.libraries
            .insert(library_name.to_owned(), after.clone());
        Ok(ProjectModelCommit {
            library_name: library_name.to_owned(),
            model_name,
            before: None,
            after,
            affects_execution: true,
        })
    }

    /// Replace one editable project model using optimistic source-revision
    /// guards. External, built-in, multi-card, and stale sources fail closed.
    pub fn replace_project_model(
        &mut self,
        library_name: &str,
        expected_source_id: ModelSourceId,
        expected_revision: ObjectRevision,
        definition: &ProjectModelDefinition,
    ) -> Result<ProjectModelCommit, String> {
        let before = self
            .libraries
            .get(library_name)
            .cloned()
            .ok_or_else(|| format!("Model library '{library_name}' does not exist"))?;
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            ..
        } = before.source_authority
        else {
            return Err(format!(
                "Model library '{library_name}' is not project-owned; create an editable project copy before changing it"
            ));
        };
        if source_id != expected_source_id || revision != expected_revision {
            return Err(format!(
                "Model library '{library_name}' changed after this candidate was opened; reload or compare before saving"
            ));
        }
        if before.models.len() != 1
            || before.source_closure.len() != 1
            || before.source_contents.len() != 1
            || !before.source_edges.is_empty()
            || !before.corners.is_empty()
        {
            return Err(format!(
                "Model library '{library_name}' is not an editable single-card definition"
            ));
        }
        let next_revision = revision
            .next()
            .map_err(|error| format!("Cannot revise model library '{library_name}': {error}"))?;
        let root = before.root_path.clone().ok_or_else(|| {
            format!("Project-owned model library '{library_name}' has no source identity")
        })?;
        let after = Self::build_project_model_library(
            library_name,
            Some(&before),
            source_id,
            next_revision,
            root,
            definition,
        )?;
        if before.source_contents[0].bytes == after.source_contents[0].bytes {
            return Err("Model candidate has no source changes to save".to_owned());
        }
        let model_name = definition.name.clone();
        self.libraries
            .insert(library_name.to_owned(), after.clone());
        Ok(ProjectModelCommit {
            library_name: library_name.to_owned(),
            model_name,
            before: Some(before),
            after,
            affects_execution: true,
        })
    }

    /// Replace one complete project-owned model revision using optimistic
    /// source-revision guards. Validation and canonical source parsing finish
    /// before the live manager is mutated.
    pub fn replace_project_model_revision(
        &mut self,
        library_name: &str,
        expected_source_id: ModelSourceId,
        expected_revision: ObjectRevision,
        definition: &ProjectModelRevisionDefinition,
        qualification: &ModelQualificationState,
    ) -> Result<ProjectModelCommit, String> {
        let before = self
            .libraries
            .get(library_name)
            .cloned()
            .ok_or_else(|| format!("Model library '{library_name}' does not exist"))?;
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            ..
        } = before.source_authority
        else {
            return Err(format!(
                "Model library '{library_name}' is not project-owned; create an editable project copy before changing it"
            ));
        };
        if source_id != expected_source_id || revision != expected_revision {
            return Err(format!(
                "Model library '{library_name}' changed after this candidate was opened; reload or compare before saving"
            ));
        }
        if before.source_closure.len() != 1
            || before.source_contents.len() != 1
            || !before.source_edges.is_empty()
        {
            return Err(format!(
                "Model library '{library_name}' is not a complete editable project-model revision"
            ));
        }
        let previous_model_name = before.models.keys().next().ok_or_else(|| {
            format!("Project-owned model library '{library_name}' has no model projection")
        })?;
        if before.models.len() != 1
            || !before
                .model_definition_metadata
                .contains_key(previous_model_name)
        {
            return Err(format!(
                "Model library '{library_name}' does not have one coherent editable definition"
            ));
        }
        let display_name = before
            .model_definition_metadata
            .get(previous_model_name)
            .and_then(|metadata| metadata.sections.first())
            .and_then(|section| section.model_files.first())
            .map_or_else(
                || format!("{library_name}.model"),
                |identity| identity.display_name.clone(),
            );
        let current_identity_candidate = definition
            .clone()
            .bind_project_source_identity(source_id, revision, display_name)
            .map_err(|error| format!("Project model revision is invalid: {error}"))?;
        let current_identity_source = current_identity_candidate
            .canonical_source()
            .map_err(|error| format!("Project model source is invalid: {error}"))?;
        if before.source_contents[0].bytes == current_identity_source.into_bytes()
            && before.model_definition_metadata.get(previous_model_name)
                == Some(&current_identity_candidate.metadata)
            && before
                .model_qualification
                .get(previous_model_name)
                .cloned()
                .unwrap_or_default()
                == *qualification
        {
            return Err("Model candidate has no semantic changes to save".to_owned());
        }

        let next_revision = revision
            .next()
            .map_err(|error| format!("Cannot revise model library '{library_name}': {error}"))?;
        let root = before.root_path.clone().ok_or_else(|| {
            format!("Project-owned model library '{library_name}' has no source identity")
        })?;
        let after = Self::build_project_model_revision_library(
            library_name,
            Some(&before),
            source_id,
            next_revision,
            root,
            definition,
            qualification,
        )?;
        let model_name = definition.base.name.clone();
        self.libraries
            .insert(library_name.to_owned(), after.clone());
        Ok(ProjectModelCommit {
            library_name: library_name.to_owned(),
            model_name,
            before: Some(before),
            after,
            affects_execution: true,
        })
    }

    /// Replace one exact canonical model revision inside a project-owned
    /// multi-model/include source closure. Every untouched byte and graph edge
    /// is retained verbatim. The transaction fails unless the selected old
    /// revision occurs exactly once in the source member recorded by its model
    /// projection, so editing one card can never rewrite an adjacent model.
    pub fn replace_project_model_revision_in_library(
        &mut self,
        library_name: &str,
        expected_source_id: ModelSourceId,
        expected_library_revision: ObjectRevision,
        expected_model_revision: ObjectRevision,
        expected_model_name: &str,
        expected_model_digest: ContentDigest,
        definition: &ProjectModelRevisionDefinition,
        qualification: &ModelQualificationState,
    ) -> Result<ProjectModelCommit, String> {
        let before = self
            .libraries
            .get(library_name)
            .cloned()
            .ok_or_else(|| format!("Model library '{library_name}' does not exist"))?;
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            digest: root_digest,
        } = before.source_authority
        else {
            return Err(format!(
                "Model library '{library_name}' is not project-owned; create an editable project copy before changing it"
            ));
        };
        if source_id != expected_source_id || revision != expected_library_revision {
            return Err(format!(
                "Model library '{library_name}' changed after this candidate was opened; reload or compare before saving"
            ));
        }
        validate_project_owned_retained_closure(&before, root_digest)?;
        let old_model = before.models.get(expected_model_name).ok_or_else(|| {
            format!("Model '{expected_model_name}' no longer exists in library '{library_name}'")
        })?;
        let old_metadata = before
            .model_definition_metadata
            .get(expected_model_name)
            .ok_or_else(|| {
                format!(
                    "Model '{expected_model_name}' has no typed project-owned definition metadata"
                )
            })?;
        let old_definition = ProjectModelRevisionDefinition::new(
            ProjectModelDefinition::from_device_model(old_model),
            old_metadata.clone(),
        );
        let old_source = old_definition
            .canonical_source()
            .map_err(|error| format!("Retained model revision is invalid: {error}"))?;
        let actual_model_digest =
            ContentDigest::from_bytes(Sha256::digest(old_source.as_bytes()).into());
        let old_identity = old_definition
            .project_source_identity()
            .map_err(|error| format!("Project model source identity is invalid: {error}"))?;
        let actual_model_revision = old_identity
            .as_ref()
            .map_or(revision, |identity| identity.revision);
        if old_identity
            .as_ref()
            .is_some_and(|identity| identity.source_id != source_id)
            || actual_model_revision != expected_model_revision
            || actual_model_digest != expected_model_digest
        {
            return Err(format!(
                "Model '{expected_model_name}' changed after this candidate was opened; reload or compare before saving"
            ));
        }
        let source_path = old_model.file_path.as_ref().ok_or_else(|| {
            format!("Model '{expected_model_name}' has no retained source-file projection")
        })?;
        let content_index = before
            .source_contents
            .iter()
            .position(|content| content.path == *source_path)
            .ok_or_else(|| {
                format!(
                    "Model '{expected_model_name}' points outside the retained source closure at '{}'",
                    source_path.display()
                )
            })?;
        let pin_index = before
            .source_closure
            .iter()
            .position(|pin| pin.path == *source_path)
            .ok_or_else(|| {
                format!(
                    "Retained model source '{}' has no authenticated pin",
                    source_path.display()
                )
            })?;
        let old_bytes = &before.source_contents[content_index].bytes;
        let offsets = exact_subslice_offsets(old_bytes, old_source.as_bytes());
        let [offset] = offsets.as_slice() else {
            return Err(format!(
                "Model '{expected_model_name}' canonical revision must occur exactly once in retained source '{}' (found {})",
                source_path.display(),
                offsets.len()
            ));
        };

        let next_library_revision = revision
            .next()
            .map_err(|error| format!("Cannot revise model library '{library_name}': {error}"))?;
        let next_model_revision = actual_model_revision
            .next()
            .map_err(|error| format!("Cannot revise model '{expected_model_name}': {error}"))?;
        let display_name = old_metadata
            .sections
            .first()
            .and_then(|section| section.model_files.first())
            .map(|identity| identity.display_name.clone())
            .or_else(|| {
                source_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(str::to_owned)
            })
            .unwrap_or_else(|| format!("{library_name}.model"));
        let mut bound = definition
            .clone()
            .bind_project_source_identity(source_id, next_model_revision, display_name)
            .map_err(|error| format!("Project model revision is invalid: {error}"))?;
        let new_source = bound
            .canonical_source()
            .map_err(|error| format!("Project model source is invalid: {error}"))?;
        bound
            .verify_source_round_trip(&new_source)
            .map_err(|error| format!("Project model source is invalid: {error}"))?;
        let identity = bound
            .project_source_identity()
            .map_err(|error| format!("Project model source identity is invalid: {error}"))?
            .ok_or_else(|| "Project model source identity was not bound".to_owned())?;
        for section in &mut bound.metadata.sections {
            if !matches!(
                section.qualification,
                ModelSectionQualification::Unqualified
            ) {
                section.qualification = ModelSectionQualification::Unqualified;
            }
        }
        qualification
            .validate_for_model(&bound.base.name)
            .map_err(|error| format!("Project model qualification is invalid: {error}"))?;
        let current_source = ModelSourceEvidenceBinding::try_new_project_bound(
            &bound.base.name,
            source_id,
            identity.content_digest,
            next_model_revision,
        )
        .map_err(|error| format!("Project model source identity is invalid: {error}"))?;
        let retained_qualification = qualification
            .reconcile_after_source_revision(&current_source)
            .map_err(|error| {
                format!("Project model qualification migration is invalid: {error}")
            })?;
        validate_section_qualification_evidence(
            &bound.metadata,
            &retained_qualification,
            &current_source,
        )?;
        if bound.base.name != expected_model_name && before.models.contains_key(&bound.base.name) {
            return Err(format!(
                "Model '{}' already exists in library '{library_name}'",
                bound.base.name
            ));
        }
        if bound.base.name != expected_model_name
            && before
                .model_qualification
                .get(expected_model_name)
                .is_some_and(|retained| *retained != ModelQualificationState::default())
        {
            return Err(
                "A qualified model cannot be renamed without an explicit release-lineage migration"
                    .to_owned(),
            );
        }
        if bound.base.name != expected_model_name
            && before
                .model_correlation
                .get(expected_model_name)
                .is_some_and(|retained| *retained != ModelCorrelationState::default())
        {
            return Err(
                "A model with correlation history cannot be renamed without an explicit evidence-lineage migration"
                    .to_owned(),
            );
        }

        let mut parser = rspice_core::library::LibParser::new(
            source_path.parent().unwrap_or_else(|| Path::new("/")),
        );
        let parsed = parser.parse_string(&new_source);
        if !parsed.is_ok() || parsed.top_level_models.len() != 1 {
            return Err(format!(
                "Project model source could not be projected: {}",
                parsed
                    .errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
            ));
        }
        let mut device_model = Self::convert_parsed_model(&parsed.top_level_models[0], source_path);
        device_model.spice_type = Some(bound.base.spice_type.to_ascii_uppercase());
        device_model.description = bound.base.description.clone();
        device_model.file_path = Some(source_path.clone());
        device_model.source_line = parsed.top_level_models[0].source_line.map(|relative_line| {
            old_bytes[..*offset]
                .iter()
                .filter(|byte| **byte == b'\n')
                .count()
                + relative_line
        });

        let mut after = before.clone();
        let content = &mut after.source_contents[content_index].bytes;
        content.splice(*offset..(*offset + old_source.len()), new_source.bytes());
        let changed_member_digest =
            ContentDigest::from_bytes(Sha256::digest(content.as_slice()).into());
        after.source_closure[pin_index].digest = changed_member_digest;
        let next_root_digest = if after.root_path.as_ref() == Some(source_path) {
            changed_member_digest
        } else {
            root_digest
        };
        after.source_authority = ModelSourceAuthority::ProjectOwned {
            source_id,
            revision: next_library_revision,
            digest: next_root_digest,
        };
        after.models.remove(expected_model_name);
        after.models.insert(bound.base.name.clone(), device_model);
        after.model_definition_metadata.remove(expected_model_name);
        after
            .model_definition_metadata
            .insert(bound.base.name.clone(), bound.metadata.clone());
        after.model_qualification.remove(expected_model_name);
        if retained_qualification != ModelQualificationState::default() {
            after
                .model_qualification
                .insert(bound.base.name.clone(), retained_qualification);
        }
        if bound.base.name != expected_model_name {
            after.model_correlation.remove(expected_model_name);
        }
        after.version = next_library_revision.get().to_string();
        self.libraries
            .insert(library_name.to_owned(), after.clone());
        Ok(ProjectModelCommit {
            library_name: library_name.to_owned(),
            model_name: bound.base.name,
            before: Some(before),
            after,
            affects_execution: true,
        })
    }

    /// Replace only the qualification/release aggregate for an exact
    /// project-owned model source. The source identity, bytes, digest, and
    /// revision remain unchanged so newly produced evidence does not become
    /// stale as a side effect of persisting it.
    pub fn replace_project_model_qualification(
        &mut self,
        library_name: &str,
        expected_source_id: ModelSourceId,
        expected_library_revision: ObjectRevision,
        expected_model_revision: ObjectRevision,
        expected_model_digest: ContentDigest,
        model_name: &str,
        qualification: &ModelQualificationState,
    ) -> Result<ProjectModelCommit, String> {
        let before = self
            .libraries
            .get(library_name)
            .cloned()
            .ok_or_else(|| format!("Model library '{library_name}' does not exist"))?;
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            digest: root_digest,
        } = before.source_authority
        else {
            return Err(format!(
                "Model library '{library_name}' is not project-owned; create an editable project copy before changing it"
            ));
        };
        if source_id != expected_source_id || revision != expected_library_revision {
            return Err(format!(
                "Model library '{library_name}' changed after qualification began; rerun against the current source revision"
            ));
        }
        validate_project_owned_retained_closure(&before, root_digest)?;
        let Some(model) = before.models.get(model_name) else {
            return Err(format!(
                "Model library '{library_name}' does not contain model '{model_name}'"
            ));
        };
        let metadata = before
            .model_definition_metadata
            .get(model_name)
            .ok_or_else(|| {
                format!("Model '{model_name}' has no typed project-owned definition metadata")
            })?;
        let definition = ProjectModelRevisionDefinition::new(
            ProjectModelDefinition::from_device_model(model),
            metadata.clone(),
        );
        let canonical = definition
            .canonical_source()
            .map_err(|error| format!("Retained model revision is invalid: {error}"))?;
        let model_digest = ContentDigest::from_bytes(Sha256::digest(canonical.as_bytes()).into());
        let model_identity = definition
            .project_source_identity()
            .map_err(|error| format!("Project model source identity is invalid: {error}"))?;
        let model_revision = model_identity
            .as_ref()
            .map_or(revision, |identity| identity.revision);
        if model_identity
            .as_ref()
            .is_some_and(|identity| identity.source_id != source_id)
            || model_revision != expected_model_revision
            || model_digest != expected_model_digest
        {
            return Err(format!(
                "Model '{model_name}' changed after qualification began; rerun against the current source revision"
            ));
        }
        let source_path = model.file_path.as_ref().ok_or_else(|| {
            format!("Model '{model_name}' has no retained source-file projection")
        })?;
        let source_bytes = before
            .source_contents
            .iter()
            .find(|content| content.path == *source_path)
            .map(|content| content.bytes.as_slice())
            .ok_or_else(|| {
                format!(
                    "Model '{model_name}' points outside the retained source closure at '{}'",
                    source_path.display()
                )
            })?;
        let occurrences = exact_subslice_offsets(source_bytes, canonical.as_bytes()).len();
        if occurrences != 1 {
            return Err(format!(
                "Model '{model_name}' canonical revision must occur exactly once in retained source '{}' (found {occurrences})",
                source_path.display()
            ));
        }
        qualification
            .validate_for_model(model_name)
            .map_err(|error| format!("Project model qualification is invalid: {error}"))?;
        let current_source = ModelSourceEvidenceBinding::try_new_project_bound(
            model_name,
            source_id,
            model_digest,
            model_revision,
        )
        .map_err(|error| format!("Project model source identity is invalid: {error}"))?;
        validate_section_qualification_evidence(metadata, qualification, &current_source)?;
        let retained = before
            .model_qualification
            .get(model_name)
            .cloned()
            .unwrap_or_default();
        if retained == *qualification {
            return Err("Model qualification has no semantic changes to save".to_owned());
        }

        let mut after = before.clone();
        if *qualification == ModelQualificationState::default() {
            after.model_qualification.remove(model_name);
        } else {
            after
                .model_qualification
                .insert(model_name.to_owned(), qualification.clone());
        }
        self.libraries
            .insert(library_name.to_owned(), after.clone());
        Ok(ProjectModelCommit {
            library_name: library_name.to_owned(),
            model_name: model_name.to_owned(),
            before: Some(before),
            after,
            affects_execution: false,
        })
    }

    /// Replace only the measurement-correlation aggregate for an exact
    /// project-owned model source. Source bytes and revisions remain
    /// unchanged; historical suites may remain retained while every new suite
    /// binds the exact source revision selected by its author.
    pub fn replace_project_model_correlation(
        &mut self,
        library_name: &str,
        expected_source_id: ModelSourceId,
        expected_library_revision: ObjectRevision,
        expected_model_revision: ObjectRevision,
        expected_model_digest: ContentDigest,
        model_name: &str,
        correlation: &ModelCorrelationState,
    ) -> Result<ProjectModelCommit, String> {
        let before = self
            .libraries
            .get(library_name)
            .cloned()
            .ok_or_else(|| format!("Model library '{library_name}' does not exist"))?;
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            digest: root_digest,
        } = before.source_authority
        else {
            return Err(format!(
                "Model library '{library_name}' is not project-owned; create an editable project copy before changing it"
            ));
        };
        if source_id != expected_source_id || revision != expected_library_revision {
            return Err(format!(
                "Model library '{library_name}' changed after correlation review began; reload the current source revision"
            ));
        }
        validate_project_owned_retained_closure(&before, root_digest)?;
        let model = before.models.get(model_name).ok_or_else(|| {
            format!("Model library '{library_name}' does not contain model '{model_name}'")
        })?;
        let metadata = before
            .model_definition_metadata
            .get(model_name)
            .ok_or_else(|| {
                format!("Model '{model_name}' has no typed project-owned definition metadata")
            })?;
        let definition = ProjectModelRevisionDefinition::new(
            ProjectModelDefinition::from_device_model(model),
            metadata.clone(),
        );
        let canonical = definition
            .canonical_source()
            .map_err(|error| format!("Retained model revision is invalid: {error}"))?;
        let model_digest = ContentDigest::from_bytes(Sha256::digest(canonical.as_bytes()).into());
        let model_identity = definition
            .project_source_identity()
            .map_err(|error| format!("Project model source identity is invalid: {error}"))?;
        let model_revision = model_identity
            .as_ref()
            .map_or(revision, |identity| identity.revision);
        if model_identity
            .as_ref()
            .is_some_and(|identity| identity.source_id != source_id)
            || model_revision != expected_model_revision
            || model_digest != expected_model_digest
        {
            return Err(format!(
                "Model '{model_name}' changed after correlation review began; reload the current source revision"
            ));
        }
        let source_path = model.file_path.as_ref().ok_or_else(|| {
            format!("Model '{model_name}' has no retained source-file projection")
        })?;
        let source_bytes = before
            .source_contents
            .iter()
            .find(|content| content.path == *source_path)
            .map(|content| content.bytes.as_slice())
            .ok_or_else(|| {
                format!(
                    "Model '{model_name}' points outside the retained source closure at '{}'",
                    source_path.display()
                )
            })?;
        let occurrences = exact_subslice_offsets(source_bytes, canonical.as_bytes()).len();
        if occurrences != 1 {
            return Err(format!(
                "Model '{model_name}' canonical revision must occur exactly once in retained source '{}' (found {occurrences})",
                source_path.display()
            ));
        }
        correlation
            .validate_for_model(model_name)
            .map_err(|error| format!("Project model correlation state is invalid: {error}"))?;
        let current_source = ModelSourceEvidenceBinding::try_new_project_bound(
            model_name,
            source_id,
            model_digest,
            model_revision,
        )
        .map_err(|error| format!("Project model source identity is invalid: {error}"))?;
        let retained = before
            .model_correlation
            .get(model_name)
            .cloned()
            .unwrap_or_default();
        if retained == *correlation {
            return Err("Model correlation has no semantic changes to save".to_owned());
        }
        for existing in &retained.suites {
            let replacement = correlation
                .suites
                .iter()
                .find(|candidate| {
                    candidate.id.eq_ignore_ascii_case(&existing.id)
                        && candidate.revision == existing.revision
                })
                .ok_or_else(|| {
                    format!(
                        "Correlation suite '{}@{}' is immutable and cannot be removed",
                        existing.id,
                        existing.revision.get()
                    )
                })?;
            if replacement != existing {
                return Err(format!(
                    "Correlation suite '{}@{}' is immutable and cannot be replaced",
                    existing.id,
                    existing.revision.get()
                ));
            }
        }
        for existing in &retained.evidence {
            let replacement = correlation
                .evidence
                .iter()
                .find(|candidate| candidate.id.eq_ignore_ascii_case(&existing.id))
                .ok_or_else(|| {
                    format!(
                        "Correlation evidence '{}' is immutable and cannot be removed",
                        existing.id
                    )
                })?;
            if replacement != existing {
                return Err(format!(
                    "Correlation evidence '{}' is immutable and cannot be replaced",
                    existing.id
                ));
            }
        }
        for suite in correlation.suites.iter().filter(|candidate| {
            !retained.suites.iter().any(|existing| {
                existing.id.eq_ignore_ascii_case(&candidate.id)
                    && existing.revision == candidate.revision
            })
        }) {
            if suite.source != current_source {
                return Err(format!(
                    "New correlation suite '{}@{}' must bind the exact current model source revision",
                    suite.id,
                    suite.revision.get()
                ));
            }
            if retained
                .suites
                .iter()
                .filter(|existing| existing.id.eq_ignore_ascii_case(&suite.id))
                .any(|existing| existing.revision >= suite.revision)
            {
                return Err(format!(
                    "New correlation suite revision '{}@{}' does not advance retained history",
                    suite.id,
                    suite.revision.get()
                ));
            }
        }
        for evidence in correlation.evidence.iter().filter(|candidate| {
            !retained
                .evidence
                .iter()
                .any(|existing| existing.id.eq_ignore_ascii_case(&candidate.id))
        }) {
            if evidence.source != current_source {
                return Err(format!(
                    "New correlation evidence '{}' must bind the exact current model source revision",
                    evidence.id
                ));
            }
        }

        let mut after = before.clone();
        if *correlation == ModelCorrelationState::default() {
            after.model_correlation.remove(model_name);
        } else {
            after
                .model_correlation
                .insert(model_name.to_owned(), correlation.clone());
        }
        self.libraries
            .insert(library_name.to_owned(), after.clone());
        Ok(ProjectModelCommit {
            library_name: library_name.to_owned(),
            model_name: model_name.to_owned(),
            before: Some(before),
            after,
            affects_execution: false,
        })
    }

    /// Create a project-owned single-card copy of one selected external or
    /// built-in model without changing its source library.
    pub fn copy_model_to_project(
        &mut self,
        source_library: &str,
        model_name: &str,
        target_library: &str,
    ) -> Result<ProjectModelCommit, String> {
        let definition = {
            let library = self
                .libraries
                .get(source_library)
                .ok_or_else(|| format!("Model library '{source_library}' does not exist"))?;
            let model = library.models.get(model_name).ok_or_else(|| {
                format!("Model '{model_name}' does not exist in library '{source_library}'")
            })?;
            ProjectModelDefinition::from_device_model(model)
        };
        self.create_project_model(target_library, &definition)
    }

    fn build_project_model_library(
        library_name: &str,
        previous: Option<&ModelLibrary>,
        source_id: ModelSourceId,
        revision: ObjectRevision,
        root: PathBuf,
        definition: &ProjectModelDefinition,
    ) -> Result<ModelLibrary, String> {
        let source = definition.canonical_source()?;
        let bytes = source.into_bytes();
        let digest = ContentDigest::from_bytes(Sha256::digest(&bytes).into());
        let mut parser =
            rspice_core::library::LibParser::new(root.parent().unwrap_or_else(|| Path::new("/")));
        let parsed = parser.parse_string(
            rspice_core::netlist::decode_source_bytes(&bytes)
                .map_err(|error| format!("Project model source cannot be decoded: {error}"))?
                .as_str(),
        );
        if !parsed.is_ok() {
            return Err(format!(
                "Project model source is invalid: {}",
                parsed
                    .errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
            ));
        }
        if parsed.top_level_models.len() != 1
            || parsed.model_count() != 1
            || parsed.subcircuit_count() != 0
            || !parsed.sections.is_empty()
        {
            return Err(
                "Project model source must contain exactly one top-level .model card and no sections or subcircuits"
                    .to_owned(),
            );
        }
        let parsed_model = &parsed.top_level_models[0];
        if parsed_model.name != definition.name {
            return Err(format!(
                "Parsed model identity '{}' does not match candidate '{}'",
                parsed_model.name, definition.name
            ));
        }
        verify_project_model_round_trip(definition, parsed_model)?;

        let mut device_model = Self::convert_parsed_model(parsed_model, &root);
        device_model.spice_type = Some(definition.spice_type.to_ascii_uppercase());
        device_model.description = definition.description.clone();
        device_model.source_line = Some(
            definition
                .description
                .lines()
                .filter(|line| !line.trim().is_empty())
                .count()
                + 1,
        );

        let mut library = ModelLibrary::new(library_name);
        let previous_model_name = previous
            .and_then(|library| library.models.keys().next())
            .cloned();
        if let Some(previous) = previous {
            library.pdk_name = previous.pdk_name.clone();
            library.technology_node = previous.technology_node.clone();
            library.expanded = previous.expanded;
        }
        library.root_path = Some(root.clone());
        library.source_authority = ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            digest,
        };
        library.source_closure = vec![ModelSourcePin {
            path: root.clone(),
            digest,
        }];
        library.source_contents = vec![ModelSourceContent { path: root, bytes }];
        library.source_edges.clear();
        library.models.clear();
        library
            .models
            .insert(device_model.name.clone(), device_model);
        let previous_metadata = previous_model_name.as_deref().and_then(|model_name| {
            previous.and_then(|library| library.model_definition_metadata.get(model_name))
        });
        let mut metadata = reconcile_project_model_metadata(definition, previous_metadata)?;
        metadata.source_identity = Some(ModelFileIdentity {
            source_id: source_id.to_string(),
            revision: revision.get(),
            content_digest: digest.to_string(),
            display_name: format!("{library_name}.model"),
        });
        library
            .model_definition_metadata
            .insert(definition.name.clone(), metadata);
        if let Some(previous) = previous
            && let Some(previous_model_name) = previous_model_name.as_deref()
            && let Some(qualification) = previous.model_qualification.get(previous_model_name)
        {
            if previous_model_name != definition.name && *qualification != Default::default() {
                return Err(
                    "A qualified model cannot be renamed without an explicit release-lineage migration"
                        .to_owned(),
                );
            }
            if previous_model_name == definition.name {
                library
                    .model_qualification
                    .insert(definition.name.clone(), qualification.clone());
            }
        }
        if let Some(previous) = previous
            && let Some(previous_model_name) = previous_model_name.as_deref()
            && let Some(correlation) = previous.model_correlation.get(previous_model_name)
        {
            if previous_model_name != definition.name
                && *correlation != ModelCorrelationState::default()
            {
                return Err(
                    "A model with correlation history cannot be renamed without an explicit evidence-lineage migration"
                        .to_owned(),
                );
            }
            if previous_model_name == definition.name {
                library
                    .model_correlation
                    .insert(definition.name.clone(), correlation.clone());
            }
        }
        library.corners.clear();
        library.selected_corner = None;
        library.version = revision.get().to_string();
        Ok(library)
    }

    fn build_project_model_revision_library(
        library_name: &str,
        previous: Option<&ModelLibrary>,
        source_id: ModelSourceId,
        revision: ObjectRevision,
        root: PathBuf,
        definition: &ProjectModelRevisionDefinition,
        qualification: &ModelQualificationState,
    ) -> Result<ModelLibrary, String> {
        qualification
            .validate_for_model(&definition.base.name)
            .map_err(|error| format!("Project model qualification is invalid: {error}"))?;
        let mut bound = definition
            .clone()
            .bind_project_source_identity(source_id, revision, format!("{library_name}.model"))
            .map_err(|error| format!("Project model revision is invalid: {error}"))?;
        let source = bound
            .canonical_source()
            .map_err(|error| format!("Project model source is invalid: {error}"))?;
        bound
            .verify_source_round_trip(&source)
            .map_err(|error| format!("Project model source is invalid: {error}"))?;
        let identity = bound
            .project_source_identity()
            .map_err(|error| format!("Project model source identity is invalid: {error}"))?
            .ok_or_else(|| "Project model source identity was not bound".to_owned())?;
        let current_source = ModelSourceEvidenceBinding::try_new_project_bound(
            &bound.base.name,
            source_id,
            identity.content_digest,
            revision,
        )
        .map_err(|error| format!("Project model source identity is invalid: {error}"))?;
        let source_changed = previous.is_some_and(|library| {
            !matches!(
                library.source_authority,
                ModelSourceAuthority::ProjectOwned {
                    source_id: previous_source_id,
                    revision: previous_revision,
                    digest: previous_digest,
                } if previous_source_id == source_id
                    && previous_revision == revision
                    && previous_digest == identity.content_digest
            )
        });
        if source_changed {
            for section in &mut bound.metadata.sections {
                if !matches!(
                    section.qualification,
                    ModelSectionQualification::Unqualified
                ) {
                    section.qualification = ModelSectionQualification::Unqualified;
                }
            }
        }
        let retained_qualification = qualification
            .reconcile_after_source_revision(&current_source)
            .map_err(|error| {
                format!("Project model qualification migration is invalid: {error}")
            })?;
        validate_section_qualification_evidence(
            &bound.metadata,
            &retained_qualification,
            &current_source,
        )?;
        let bytes = source.into_bytes();

        let mut parser =
            rspice_core::library::LibParser::new(root.parent().unwrap_or_else(|| Path::new("/")));
        let parsed = parser.parse_string(
            rspice_core::netlist::decode_source_bytes(&bytes)
                .map_err(|error| format!("Project model source cannot be decoded: {error}"))?
                .as_str(),
        );
        if !parsed.is_ok() || parsed.top_level_models.len() != 1 {
            let details = parsed
                .errors
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; ");
            return Err(if details.is_empty() {
                "Project model source did not produce one top-level model".to_owned()
            } else {
                format!("Project model source could not be projected: {details}")
            });
        }

        let mut device_model = Self::convert_parsed_model(&parsed.top_level_models[0], &root);
        device_model.spice_type = Some(bound.base.spice_type.to_ascii_uppercase());
        device_model.description = bound.base.description.clone();
        device_model.source_line = Some(
            bound
                .base
                .description
                .lines()
                .filter(|line| !line.trim().is_empty())
                .count()
                + 1,
        );

        let previous_model_name = previous
            .and_then(|library| library.models.keys().next())
            .cloned();
        let mut library = ModelLibrary::new(library_name);
        if let Some(previous) = previous {
            library.pdk_name = previous.pdk_name.clone();
            library.technology_node = previous.technology_node.clone();
            library.expanded = previous.expanded;
        }
        library.root_path = Some(root.clone());
        library.source_authority = ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            digest: identity.content_digest,
        };
        library.source_closure = vec![ModelSourcePin {
            path: root.clone(),
            digest: identity.content_digest,
        }];
        library.source_contents = vec![ModelSourceContent {
            path: root.clone(),
            bytes,
        }];
        library.source_edges.clear();
        library.models.clear();
        library.models.insert(bound.base.name.clone(), device_model);
        library.model_definition_metadata.clear();
        library
            .model_definition_metadata
            .insert(bound.base.name.clone(), bound.metadata.clone());
        library.model_qualification.clear();
        if let Some(previous_model_name) = previous_model_name.as_deref() {
            if previous_model_name != bound.base.name && *qualification != Default::default() {
                return Err(
                    "A qualified model cannot be renamed without an explicit release-lineage migration"
                        .to_owned(),
                );
            }
        }
        if retained_qualification != Default::default() {
            library
                .model_qualification
                .insert(bound.base.name.clone(), retained_qualification);
        }
        if let Some(previous) = previous
            && let Some(previous_model_name) = previous_model_name.as_deref()
            && let Some(correlation) = previous.model_correlation.get(previous_model_name)
        {
            if previous_model_name != bound.base.name
                && *correlation != ModelCorrelationState::default()
            {
                return Err(
                    "A model with correlation history cannot be renamed without an explicit evidence-lineage migration"
                        .to_owned(),
                );
            }
            if previous_model_name == bound.base.name {
                library
                    .model_correlation
                    .insert(bound.base.name.clone(), correlation.clone());
            }
        }

        library.corners.clear();
        let selected_corner = bound
            .metadata
            .sections
            .iter()
            .find(|section| section.name.eq_ignore_ascii_case("tt"))
            .or_else(|| bound.metadata.sections.first())
            .map(|section| section.name.clone());
        for section in &bound.metadata.sections {
            let mut corner = ProcessCorner::new(&section.name);
            corner.description = format!("Project model section {}", section.name);
            corner.nmos_corner = section.name.to_ascii_lowercase();
            corner.pmos_corner = section.name.to_ascii_lowercase();
            corner.file_path = Some(root.clone());
            corner.is_default = selected_corner.as_deref() == Some(section.name.as_str());
            library.corners.insert(section.name.clone(), corner);
        }
        library.selected_corner = selected_corner;
        library.version = revision.get().to_string();
        Ok(library)
    }

    /// Compute the canonical SHA-256 identity used to pin an external model
    /// source. Callers compare this value with the digest stored by the last
    /// explicit load/refresh; computing it never accepts new content.
    pub fn calculate_source_digest(
        path: impl AsRef<std::path::Path>,
    ) -> Result<crate::product::ContentDigest, String> {
        let path = path.as_ref();
        let bytes = std::fs::read(path)
            .map_err(|error| format!("Failed to read '{}': {error}", path.display()))?;
        Ok(crate::product::ContentDigest::from_bytes(
            Sha256::digest(&bytes).into(),
        ))
    }

    /// Prove that the project-owned technology attachment still names the
    /// exact live execution catalog entry accepted at attachment time.
    pub fn validate_attached_technology(
        &self,
        binding: Option<&crate::state::ProjectTechnologyBinding>,
    ) -> Result<(), String> {
        let Some(binding) = binding else {
            return Ok(());
        };
        let library = self.get_library(binding.model_library()).ok_or_else(|| {
            format!(
                "Attached technology library '{}' was removed; reattach an authenticated model library before simulation",
                binding.model_library()
            )
        })?;
        binding.validate_model_library(library).map_err(|error| {
            format!(
                "Attached technology contract is stale: {error}. Reattach the current model library before simulation"
            )
        })
    }

    /// Build one all-or-nothing source snapshot for a simulation run.
    ///
    /// Every unique pinned member is read exactly once. The digest is computed
    /// over those same bytes, and only the authenticated UTF-8 content is
    /// published to the in-memory resolver.
    pub fn seal_execution_sources(&self) -> Result<SealedModelExecutionSources, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.seal_execution_sources_with_reader(|path| {
                std::fs::read(path).map_err(|error| error.to_string())
            })
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.seal_execution_sources_with_reader(|path| {
                Err(format!(
                    "browser execution cannot authenticate external model path '{}'",
                    path.display()
                ))
            })
        }
    }

    fn seal_execution_sources_with_reader<F>(
        &self,
        mut read_external: F,
    ) -> Result<SealedModelExecutionSources, String>
    where
        F: FnMut(&Path) -> Result<Vec<u8>, String>,
    {
        let mut libraries: Vec<&ModelLibrary> = self
            .libraries
            .values()
            .filter(|library| library.source_authority.has_execution_source())
            .collect();
        libraries.sort_by(|left, right| left.name.cmp(&right.name));

        // The final flag selects retained project bytes (`true`) or a live,
        // re-authenticated external read (`false`). A path can never mix the
        // two authorities across libraries.
        let mut expected_sources =
            BTreeMap::<PathBuf, (crate::product::ContentDigest, Vec<String>, bool)>::new();
        let mut retained_sources = BTreeMap::<PathBuf, Vec<u8>>::new();
        let mut expected_edges = BTreeMap::<(PathBuf, String), PathBuf>::new();
        let mut sealed_libraries = Vec::with_capacity(libraries.len());
        for library in libraries {
            let root_path = library.root_path.as_ref().ok_or_else(|| {
                format!(
                    "Model library '{}' declares source authority but has no root identity",
                    library.name
                )
            })?;
            let project_owned = library.source_authority.is_project_owned();
            if library.source_closure.is_empty() {
                return Err(format!(
                    "Model library '{}' is not content-pinned; refresh or re-import '{}' before simulation",
                    library.name,
                    root_path.display()
                ));
            }
            if !library
                .source_closure
                .iter()
                .any(|source| source.path == *root_path)
            {
                return Err(format!(
                    "Model library '{}' has a corrupt source closure that does not contain its root '{}'; refresh or re-import it before simulation",
                    library.name,
                    root_path.display()
                ));
            }

            let source_paths = library
                .source_closure
                .iter()
                .map(|source| source.path.clone())
                .collect::<HashSet<_>>();

            for source in &library.source_closure {
                #[cfg(not(target_arch = "wasm32"))]
                if !project_owned && is_foreign_platform_absolute_path(&source.path) {
                    return Err(format!(
                        "Model library '{}' retains foreign-platform dependency '{}', which is unavailable on this host; re-import or repair the binding before simulation",
                        library.name,
                        source.path.display()
                    ));
                }
                if !super::is_portable_absolute_path(&source.path) {
                    return Err(format!(
                        "Model library '{}' has a non-canonical dependency path '{}'; refresh or re-import it before simulation",
                        library.name,
                        source.path.display()
                    ));
                }
                match expected_sources.entry(source.path.clone()) {
                    std::collections::btree_map::Entry::Vacant(entry) => {
                        entry.insert((source.digest, vec![library.name.clone()], project_owned));
                    }
                    std::collections::btree_map::Entry::Occupied(mut entry) => {
                        if entry.get().0 != source.digest {
                            return Err(format!(
                                "Model libraries disagree on the accepted SHA-256 for shared dependency '{}'",
                                source.path.display()
                            ));
                        }
                        if entry.get().2 != project_owned {
                            return Err(format!(
                                "Model libraries disagree on source authority for shared dependency '{}'",
                                source.path.display()
                            ));
                        }
                        entry.get_mut().1.push(library.name.clone());
                    }
                }
            }
            if library.source_edges.is_empty()
                && let Some(unresolved) = library
                    .source_closure
                    .iter()
                    .find(|source| source.path != *root_path)
            {
                return Err(format!(
                    "Model library '{}' dependency '{}' has no authenticated resolution edge; refresh or re-import the library before simulation",
                    library.name,
                    unresolved.path.display()
                ));
            }
            if !library.source_edges.is_empty()
                && let Some(unreachable) = first_unreachable_source(
                    root_path,
                    &library.source_closure,
                    &library.source_edges,
                )
            {
                return Err(format!(
                    "Model library '{}' dependency '{}' is not reachable from root '{}' by authenticated resolution edges; refresh or re-import the library before simulation",
                    library.name,
                    unreachable.display(),
                    root_path.display()
                ));
            }
            for edge in &library.source_edges {
                if !source_paths.contains(&edge.owner) || !source_paths.contains(&edge.target) {
                    return Err(format!(
                        "Model library '{}' source edge '{}' -> '{}' references a source outside that library's pinned closure",
                        library.name,
                        edge.owner.display(),
                        edge.target.display()
                    ));
                }
                let requested_path =
                    rspice_core::netlist::normalize_source_path_literal(&edge.requested_path)
                        .map_err(|error| {
                            format!(
                                "Model library '{}' has an invalid source edge: {error}",
                                library.name
                            )
                        })?;
                let key = (edge.owner.clone(), requested_path);
                if let Some(existing) = expected_edges.get(&key) {
                    if existing != &edge.target {
                        return Err(format!(
                            "Model libraries disagree on dependency resolution for '{}' in '{}'",
                            key.1,
                            key.0.display()
                        ));
                    }
                } else {
                    expected_edges.insert(key, edge.target.clone());
                }
            }
            if !library.source_contents.is_empty() {
                if library.source_contents.len() != library.source_closure.len() {
                    return Err(format!(
                        "Model library '{}' does not retain exact bytes for every pinned source; refresh or re-import it",
                        library.name
                    ));
                }
                for (pin, content) in library.source_closure.iter().zip(&library.source_contents) {
                    if pin.path != content.path {
                        return Err(format!(
                            "Model library '{}' retained source-byte identity does not match '{}'",
                            library.name,
                            pin.path.display()
                        ));
                    }
                    let actual = crate::product::ContentDigest::from_bytes(
                        Sha256::digest(&content.bytes).into(),
                    );
                    if actual != pin.digest {
                        return Err(format!(
                            "Model library '{}' retained bytes for '{}' do not match the accepted digest",
                            library.name,
                            pin.path.display()
                        ));
                    }
                    match retained_sources.entry(content.path.clone()) {
                        std::collections::btree_map::Entry::Vacant(entry) => {
                            entry.insert(content.bytes.clone());
                        }
                        std::collections::btree_map::Entry::Occupied(entry)
                            if entry.get() != &content.bytes =>
                        {
                            return Err(format!(
                                "Model libraries retain different bytes for shared dependency '{}'",
                                content.path.display()
                            ));
                        }
                        std::collections::btree_map::Entry::Occupied(_) => {}
                    }
                }
            }
            if project_owned {
                let ModelSourceAuthority::ProjectOwned { digest, .. } = library.source_authority
                else {
                    unreachable!("project-owned authority was checked above")
                };
                if library.source_contents.len() != library.source_closure.len() {
                    return Err(format!(
                        "Project-owned model library '{}' must retain exact bytes for every authenticated source member",
                        library.name
                    ));
                }
                if library
                    .source_closure
                    .iter()
                    .find(|source| source.path == *root_path)
                    .is_none_or(|source| source.digest != digest)
                {
                    return Err(format!(
                        "Project-owned model library '{}' root source digest does not match its revision authority",
                        library.name
                    ));
                }
            }

            let mut sections = library
                .corners
                .values()
                .filter(|corner| corner.file_path.is_some())
                .map(|corner| corner.name.clone())
                .collect::<Vec<_>>();
            sections.sort_by_key(|section| section.to_ascii_lowercase());
            sections.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
            sealed_libraries.push(SealedExecutionLibrary {
                name: library.name.clone(),
                root_path: root_path.clone(),
                sections,
            });
        }

        for ((owner, requested_path), target) in &expected_edges {
            if !expected_sources.contains_key(owner) || !expected_sources.contains_key(target) {
                return Err(format!(
                    "Model source edge '{}' -> '{}' for '{}' references a source outside the pinned closure",
                    owner.display(),
                    target.display(),
                    requested_path
                ));
            }
        }

        let mut authenticated_sources = Vec::with_capacity(expected_sources.len());
        for (path, (expected_digest, owners, project_owned)) in expected_sources {
            let bytes = if project_owned {
                retained_sources.remove(&path).ok_or_else(|| {
                    format!(
                        "Project-owned model dependency '{}' (used by {}) has no retained source bytes",
                        path.display(),
                        owners.join(", ")
                    )
                })?
            } else {
                read_external(&path).map_err(|error| {
                    format!(
                        "Model library dependency is unavailable at '{}' (used by {}): {error}",
                        path.display(),
                        owners.join(", ")
                    )
                })?
            };
            let actual_digest =
                crate::product::ContentDigest::from_bytes(Sha256::digest(&bytes).into());
            if actual_digest != expected_digest {
                return Err(if project_owned {
                    format!(
                        "Project-owned model dependency changed at '{}'; the retained bytes no longer match the accepted SHA-256 identity",
                        path.display()
                    )
                } else {
                    format!(
                        "Model library dependency changed at '{}'; refresh or re-import the library to explicitly accept the new source closure before simulation",
                        path.display()
                    )
                });
            }
            let content = rspice_core::netlist::decode_source_bytes(&bytes).map_err(|error| {
                format!(
                    "Pinned model dependency '{}' cannot be decoded with the supported source encoding policy: {error}",
                    path.display(),
                )
            })?;
            authenticated_sources.push((path, content));
        }

        let edges = expected_edges
            .into_iter()
            .map(
                |((owner, requested_path), target)| rspice_core::netlist::SealedSourceEdge {
                    owner,
                    requested_path,
                    target,
                },
            )
            .collect::<Vec<_>>();
        let bundle = rspice_core::netlist::SealedSourceBundle::try_new_with_edges(
            authenticated_sources.clone(),
            edges.clone(),
        )
        .map_err(|error| format!("Failed to seal model source bundle: {error}"))?;
        Ok(SealedModelExecutionSources {
            bundle,
            sources: authenticated_sources,
            edges,
            libraries: sealed_libraries,
        })
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
            level: Self::convert_model_level(model.level),
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

    fn convert_model_level(level: Option<u32>) -> ModelLevel {
        match level {
            Some(1) => ModelLevel::SpiceLevel1,
            Some(3) => ModelLevel::SpiceLevel3,
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
}
