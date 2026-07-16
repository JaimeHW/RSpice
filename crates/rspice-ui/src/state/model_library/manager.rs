use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
use super::is_foreign_platform_absolute_path;
use super::{
    DeviceModel, ModelLevel, ModelLibrary, ModelSourceContent, ModelSourceEdge, ModelSourcePin,
    ModelType, ProcessCorner, first_unreachable_source,
};
use crate::services::simulation_runner::{CornerModelBinding, CornerProcess};

/// One immutable, authenticated model-source snapshot for a simulation run.
/// The exact bytes are intentionally transient and are never serialized into
/// project/session state.
#[derive(Debug, Clone)]
pub struct SealedModelExecutionSources {
    bundle: rspice_core::netlist::SealedSourceBundle,
    libraries: Vec<SealedExecutionLibrary>,
}

#[derive(Debug, Clone)]
struct SealedExecutionLibrary {
    name: String,
    root_path: PathBuf,
    sections: Vec<String>,
}

impl SealedModelExecutionSources {
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
        library.source_closure = source_closure;
        library.source_contents = source_contents;
        library.source_edges = source_edges;
        library.models.clear();
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
        let mut libraries: Vec<&ModelLibrary> = self
            .libraries
            .values()
            .filter(|library| library.root_path.is_some())
            .collect();
        libraries.sort_by(|left, right| left.name.cmp(&right.name));

        let mut expected_sources =
            BTreeMap::<PathBuf, (crate::product::ContentDigest, Vec<String>)>::new();
        let mut retained_sources = BTreeMap::<PathBuf, Vec<u8>>::new();
        let mut expected_edges = BTreeMap::<(PathBuf, String), PathBuf>::new();
        let mut sealed_libraries = Vec::with_capacity(libraries.len());
        for library in libraries {
            let root_path = library
                .root_path
                .as_ref()
                .expect("external model libraries have a source path");
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
                if is_foreign_platform_absolute_path(&source.path) {
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
                        entry.insert((source.digest, vec![library.name.clone()]));
                    }
                    std::collections::btree_map::Entry::Occupied(mut entry) => {
                        if entry.get().0 != source.digest {
                            return Err(format!(
                                "Model libraries disagree on the accepted SHA-256 for shared dependency '{}'",
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

        #[cfg(not(target_arch = "wasm32"))]
        let authenticated_sources = {
            let mut authenticated_sources = Vec::with_capacity(expected_sources.len());
            for (path, (expected_digest, owners)) in expected_sources {
                let bytes = std::fs::read(&path).map_err(|error| {
                    format!(
                        "Model library dependency is unavailable at '{}' (used by {}): {error}",
                        path.display(),
                        owners.join(", ")
                    )
                })?;
                let actual_digest =
                    crate::product::ContentDigest::from_bytes(Sha256::digest(&bytes).into());
                if actual_digest != expected_digest {
                    return Err(format!(
                        "Model library dependency changed at '{}'; refresh or re-import the library to explicitly accept the new source closure before simulation",
                        path.display()
                    ));
                }
                let content = rspice_core::netlist::decode_source_bytes(&bytes).map_err(|error| {
                    format!(
                        "Pinned model dependency '{}' cannot be decoded with the supported source encoding policy: {error}",
                        path.display(),
                    )
                })?;
                authenticated_sources.push((path, content));
            }
            authenticated_sources
        };

        #[cfg(target_arch = "wasm32")]
        let authenticated_sources = {
            let mut authenticated_sources = Vec::with_capacity(expected_sources.len());
            for (path, (expected_digest, owners)) in expected_sources {
                let bytes = retained_sources.remove(&path).ok_or_else(|| {
                    format!(
                        "Model library dependency '{}' (used by {}) has no retained browser source bytes; re-import the library",
                        path.display(),
                        owners.join(", ")
                    )
                })?;
                let actual_digest =
                    crate::product::ContentDigest::from_bytes(Sha256::digest(&bytes).into());
                if actual_digest != expected_digest {
                    return Err(format!(
                        "Retained browser model dependency '{}' no longer matches its accepted digest",
                        path.display()
                    ));
                }
                let content =
                    rspice_core::netlist::decode_source_bytes(&bytes).map_err(|error| {
                        format!(
                            "Pinned browser model dependency '{}' cannot be decoded: {error}",
                            path.display()
                        )
                    })?;
                authenticated_sources.push((path, content));
            }
            authenticated_sources
        };

        let edges = expected_edges
            .into_iter()
            .map(
                |((owner, requested_path), target)| rspice_core::netlist::SealedSourceEdge {
                    owner,
                    requested_path,
                    target,
                },
            );
        let bundle = rspice_core::netlist::SealedSourceBundle::try_new_with_edges(
            authenticated_sources,
            edges,
        )
        .map_err(|error| format!("Failed to seal model source bundle: {error}"))?;
        Ok(SealedModelExecutionSources {
            bundle,
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
                    level: ModelLevel::Unknown,
                    description: model.description.clone().unwrap_or_default(),
                    l_min: model.lmin,
                    l_max: model.lmax,
                    w_min: model.wmin,
                    w_max: model.wmax,
                    vdd: None,
                    vth0: None,
                    file_path: None,
                    parameters: HashMap::new(),
                };
                library
                    .models
                    .insert(device_model.name.clone(), device_model);
            }
        }
    }

    /// Convert a parsed model from the core library to UI DeviceModel
    fn convert_parsed_model(
        model: &rspice_core::library::ParsedModel,
        file_path: &std::path::Path,
    ) -> DeviceModel {
        let model_type = Self::convert_core_model_type(model.model_type);

        DeviceModel {
            name: model.name.clone(),
            model_type,
            level: ModelLevel::Unknown,
            description: model.description.clone().unwrap_or_default(),
            l_min: model.lmin,
            l_max: model.lmax,
            w_min: model.wmin,
            w_max: model.wmax,
            vdd: None,
            vth0: None,
            file_path: Some(file_path.to_path_buf()),
            parameters: model.parameters.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;
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
