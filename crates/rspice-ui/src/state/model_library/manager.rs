//! The model library.
//!
//! Owns every model a project can resolve, from every source, and seals the
//! exact set a run executed against. Sealing is by content digest, so a
//! library that changes underneath a completed run is detectable rather
//! than silently assumed identical.

mod bin_audit;
mod corner_binding;
mod project_models;
mod sealing;
pub use bin_audit::{
    ModelBinAuditAxisRange, ModelBinAuditDraft, ModelBinAuditFinding, ModelBinAuditFindingKind,
    ModelBinAuditReceipt,
};
pub use corner_binding::{
    CornerBindingInspection, CornerBindingInspectionRow, CornerSectionInspection,
};

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rspice_core::library::{
    CatalogDefinitionPreview, CatalogSubcircuitInterface, SpiceLibraryIndex,
};

#[cfg(not(target_arch = "wasm32"))]
use super::is_foreign_platform_absolute_path;
use super::{
    DeviceModel, FiniteF64, ModelCorrelationState, ModelDefinitionMetadata, ModelFileIdentity,
    ModelLevel, ModelLibrary, ModelQualificationState, ModelSectionQualification,
    ModelSourceAuthority, ModelSourceContent, ModelSourceEdge, ModelSourceEvidenceBinding,
    ModelSourcePin, ModelSubcircuitInterface, ModelType, ParameterDataType, ParameterDefinition,
    ParameterSource, ParameterValue, ProcessCorner, ProjectModelDefinition,
    ProjectModelRevisionDefinition, first_unreachable_source, subcircuit_interface_key,
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
    pdk_process_bindings: Vec<crate::state::pdk_config::SealedPdkModelProcessBinding>,
    pdk_veriloga_artifacts: Vec<crate::state::pdk_config::SealedPdkVerilogAArtifact>,
    pdk_veriloga_bindings: Vec<crate::state::pdk_config::SealedPdkVerilogABinding>,
    pdk_identity: Option<(
        crate::state::pdk_config::PdkTechnologyBinding,
        ContentDigest,
    )>,
}

#[derive(Debug, Clone)]
struct SealedExecutionLibrary {
    name: String,
    root_path: PathBuf,
    corners: Vec<ProcessCorner>,
}

impl SealedModelExecutionSources {
    /// Add the exact signed-PDK model closure selected by the project binding.
    ///
    /// This rebuilds the same sealed bundle used by ordinary model libraries.
    /// It never introduces a search path and refuses identity or edge
    /// conflicts instead of choosing one source by precedence.
    pub(crate) fn with_pdk_model_sources(
        mut self,
        pdk: crate::state::pdk_config::SealedPdkModelSources,
    ) -> Result<Self, String> {
        if self.pdk_identity.is_some() {
            return Err("A sealed model snapshot already contains a signed PDK binding".to_owned());
        }

        let mut sources_by_key = self
            .sources
            .iter()
            .map(|(path, source)| (portable_path_key(path), (path.clone(), source.clone())))
            .collect::<BTreeMap<_, _>>();
        for (path, source) in pdk.sources {
            let key = portable_path_key(&path);
            match sources_by_key.entry(key) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert((path, source));
                }
                std::collections::btree_map::Entry::Occupied(entry) if entry.get().1 != source => {
                    return Err(format!(
                        "Signed PDK source '{}' conflicts with an authenticated model-library source",
                        path.display()
                    ));
                }
                std::collections::btree_map::Entry::Occupied(_) => {}
            }
        }

        let mut edges_by_key = self
            .edges
            .iter()
            .map(|edge| {
                (
                    (portable_path_key(&edge.owner), edge.requested_path.clone()),
                    edge.clone(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        for edge in pdk.edges {
            let key = (portable_path_key(&edge.owner), edge.requested_path.clone());
            match edges_by_key.entry(key) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert(edge);
                }
                std::collections::btree_map::Entry::Occupied(entry)
                    if portable_path_key(&entry.get().target)
                        != portable_path_key(&edge.target) =>
                {
                    return Err(format!(
                        "Signed PDK and model libraries disagree on dependency '{}' in '{}'",
                        edge.requested_path,
                        edge.owner.display()
                    ));
                }
                std::collections::btree_map::Entry::Occupied(_) => {}
            }
        }

        self.sources = sources_by_key.into_values().collect();
        self.sources.sort_by(|left, right| left.0.cmp(&right.0));
        self.edges = edges_by_key.into_values().collect();
        self.edges.sort_by(|left, right| {
            left.owner
                .cmp(&right.owner)
                .then_with(|| left.requested_path.cmp(&right.requested_path))
                .then_with(|| left.target.cmp(&right.target))
        });
        self.bundle = rspice_core::netlist::SealedSourceBundle::try_new_with_edges(
            self.sources.clone(),
            self.edges.clone(),
        )
        .map_err(|error| format!("Failed to merge signed PDK model sources: {error}"))?;
        self.pdk_process_bindings = pdk.process_bindings;
        self.pdk_veriloga_artifacts = pdk.veriloga_artifacts;
        self.pdk_veriloga_bindings = pdk.veriloga_bindings;
        self.pdk_identity = Some((pdk.binding, pdk.archive_digest));
        Ok(self)
    }

    /// Exact signed package identity participating in this model snapshot.
    #[must_use]
    pub(crate) fn pdk_model_identity(&self) -> Option<(String, ContentDigest)> {
        self.pdk_identity.as_ref().map(|(binding, archive_digest)| {
            (
                format!(
                    "signed-pdk:{}@{}:manifest:{}",
                    binding.package_id, binding.revision, binding.manifest_digest
                ),
                *archive_digest,
            )
        })
    }

    #[must_use]
    pub(crate) fn pdk_veriloga_authority(
        &self,
    ) -> Option<(
        &crate::state::pdk_config::PdkTechnologyBinding,
        ContentDigest,
        &[crate::state::pdk_config::SealedPdkVerilogAArtifact],
        &[crate::state::pdk_config::SealedPdkVerilogABinding],
    )> {
        let (binding, archive_digest) = self.pdk_identity.as_ref()?;
        Some((
            binding,
            *archive_digest,
            &self.pdk_veriloga_artifacts,
            &self.pdk_veriloga_bindings,
        ))
    }

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
        if self.libraries.is_empty() && self.pdk_process_bindings.is_empty() {
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
                let corner = library
                    .corners
                    .iter()
                    .find(|corner| corner.name.eq_ignore_ascii_case(keyword))
                    .cloned();
                if corner.is_none()
                    && (*process != CornerProcess::TT || !library.corners.is_empty())
                {
                    return Err(format!(
                        "Model library '{}' does not define the {} process section",
                        library.name, keyword
                    ));
                }
                match corner.as_ref() {
                    Some(corner) => {
                        for section in self.materialize_library_corner(library, corner)? {
                            let binding = CornerModelBinding {
                                process: *process,
                                source_label: section.source_label,
                                section: Some(section.section),
                                materialized_model_cards: section.materialized_model_cards,
                            };
                            binding.validate()?;
                            bindings.push(binding);
                        }
                    }
                    None => {
                        let mut processor = rspice_core::netlist::IncludeProcessor::new_sealed(
                            &library.root_path,
                            self.bundle.clone(),
                        );
                        let materialized_model_cards = processor
                            .process_sealed_root(&library.root_path, None)
                            .map_err(|error| {
                                format!(
                                    "Failed to materialize sealed model library '{}' from '{}': {error}",
                                    library.name,
                                    library.root_path.display()
                                )
                            })?;
                        let binding = CornerModelBinding {
                            process: *process,
                            source_label: library.root_path.display().to_string(),
                            section: None,
                            materialized_model_cards,
                        };
                        binding.validate()?;
                        bindings.push(binding);
                    }
                }
            }

            if !self.pdk_process_bindings.is_empty() {
                let pdk_process = pdk_model_process(*process);
                let selected = self
                    .pdk_process_bindings
                    .iter()
                    .filter(|binding| binding.process == pdk_process)
                    .collect::<Vec<_>>();
                if selected.is_empty() {
                    let package = self
                        .pdk_identity
                        .as_ref()
                        .map(|(binding, _)| format!("{} {}", binding.package_id, binding.revision))
                        .unwrap_or_else(|| "signed PDK".to_owned());
                    return Err(format!(
                        "{package} does not supply an explicit {} model-source contract",
                        process.as_keyword()
                    ));
                }
                for source in selected {
                    let mut processor = rspice_core::netlist::IncludeProcessor::new_sealed(
                        &source.root_path,
                        self.bundle.clone(),
                    );
                    let materialized_model_cards = processor
                        .process_sealed_root(&source.root_path, source.section.as_deref())
                        .map_err(|error| {
                            format!(
                                "Failed to materialize signed PDK {} source '{}' from '{}': {error}",
                                process.as_keyword(),
                                source.source_id,
                                source.artifact_path
                            )
                        })?;
                    let package = self
                        .pdk_identity
                        .as_ref()
                        .map(|(binding, digest)| {
                            format!(
                                "{} {} / {} / {} / artifact {} / archive {}",
                                binding.package_id,
                                binding.revision,
                                source.domain.label(),
                                source.source_id,
                                source.artifact_digest,
                                digest
                            )
                        })
                        .unwrap_or_else(|| source.source_id.clone());
                    let binding = CornerModelBinding {
                        process: *process,
                        source_label: package,
                        section: source.section.clone(),
                        materialized_model_cards,
                    };
                    binding.validate()?;
                    bindings.push(binding);
                }
            }
        }
        Ok(bindings)
    }

    fn materialize_library_corner(
        &self,
        library: &SealedExecutionLibrary,
        corner: &ProcessCorner,
    ) -> Result<Vec<MaterializedCornerSection>, String> {
        if let Err(errors) = corner.validate_contract() {
            return Err(format!(
                "Model library '{}' corner '{}' has an invalid section contract: {}",
                library.name,
                corner.name,
                errors.join("; ")
            ));
        }
        let source_path = corner
            .file_path
            .as_deref()
            .unwrap_or(library.root_path.as_path());
        let bindings = corner.effective_section_bindings();
        if bindings.is_empty() {
            return Err(format!(
                "Model library '{}' corner '{}' has no executable section binding",
                library.name, corner.name
            ));
        }

        let mut domains_by_section =
            BTreeMap::<(PathBuf, String), Vec<super::CornerSectionDomain>>::new();
        for binding in bindings {
            domains_by_section
                .entry((source_path.to_path_buf(), binding.section))
                .or_default()
                .push(binding.domain);
        }

        let mut sections = Vec::with_capacity(domains_by_section.len());
        for ((path, section), mut domains) in domains_by_section {
            domains.sort();
            domains.dedup();
            let mut processor =
                rspice_core::netlist::IncludeProcessor::new_sealed(&path, self.bundle.clone());
            let materialized_model_cards = processor
                .process_sealed_root(&path, Some(&section))
                .map_err(|error| {
                    format!(
                        "Failed to materialize {} section '{}' for model library '{}' corner '{}' from '{}': {error}",
                        domains
                            .iter()
                            .map(|domain| domain.label())
                            .collect::<Vec<_>>()
                            .join(" + "),
                        section,
                        library.name,
                        corner.name,
                        path.display()
                    )
                })?;
            sections.push(MaterializedCornerSection {
                source_label: format!(
                    "{} [{}] ({})",
                    path.display(),
                    section,
                    domains
                        .iter()
                        .map(|domain| domain.label())
                        .collect::<Vec<_>>()
                        .join(" + ")
                ),
                path,
                section,
                domains,
                materialized_model_cards,
            });
        }
        Ok(sections)
    }
}

#[derive(Debug, Clone)]
struct MaterializedCornerSection {
    source_label: String,
    path: PathBuf,
    section: String,
    domains: Vec<super::CornerSectionDomain>,
    materialized_model_cards: String,
}

const fn pdk_model_process(process: CornerProcess) -> crate::state::pdk_config::PdkModelProcess {
    match process {
        CornerProcess::TT => crate::state::pdk_config::PdkModelProcess::Tt,
        CornerProcess::SS => crate::state::pdk_config::PdkModelProcess::Ss,
        CornerProcess::FF => crate::state::pdk_config::PdkModelProcess::Ff,
        CornerProcess::SF => crate::state::pdk_config::PdkModelProcess::Sf,
        CornerProcess::FS => crate::state::pdk_config::PdkModelProcess::Fs,
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
    /// Project-persisted, content-addressed model-bin audit evidence.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    model_bin_audit_receipts: Vec<ModelBinAuditReceipt>,
    /// Explicit, project-owned provider choices for contested model names.
    ///
    /// The execution materializer converts these choices into a validated
    /// first-definition precedence order. Missing, stale, same-library, and
    /// cyclic choices fail closed before any source is sealed.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    definition_resolutions: BTreeMap<String, ModelDefinitionResolution>,
    /// Session-local ownership of libraries populated by the host PDK search
    /// configuration.
    ///
    /// This is deliberately not project-persisted. Once a project records an
    /// external source it is an explicit project dependency; a later host
    /// search-path edit must not silently delete it after restore. During the
    /// live session, however, this set lets Apply reconcile the previous PDK
    /// scan exactly instead of only accumulating newly discovered libraries.
    #[serde(skip)]
    pdk_config_libraries: BTreeSet<String>,
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

/// One bounded page from the complete addressable shipped-parts catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackCatalogPage {
    pub total_matches: usize,
    pub hits: Vec<PackModelHit>,
}

/// Exact executable source and ordered interface accepted for a shipped
/// top-level subcircuit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedPackSubcircuit {
    pub library: String,
    pub name: String,
    pub ports: Vec<String>,
    pub source_path: PathBuf,
}

/// One active parsed provider of a case-insensitive model name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelDefinitionProvider {
    pub library: String,
    pub model: String,
    pub source: Option<PathBuf>,
    pub source_line: Option<usize>,
}

/// A model name with more than one active provider.
///
/// No provider is selected implicitly. Until an explicit resolution contract
/// exists, execution fails closed rather than depending on map or include
/// iteration order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelDefinitionConflict {
    pub normalized_name: String,
    pub providers: Vec<ModelDefinitionProvider>,
}

/// Durable provider selection for one contested, case-insensitive model name.
///
/// This is a precedence contract, not a waiver: the selected provider must
/// still exist exactly, every other active provider remains visible, and the
/// complete set of selections must admit one deterministic library order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelDefinitionResolution {
    pub normalized_name: String,
    pub provider_library: String,
    pub provider_model: String,
}

fn unresolved_definition_error(conflict: &ModelDefinitionConflict) -> String {
    let providers = conflict
        .providers
        .iter()
        .map(|provider| {
            let source = provider
                .source
                .as_deref()
                .map_or_else(|| "in-memory".to_owned(), |path| path.display().to_string());
            match provider.source_line {
                Some(line) => format!("{} ({source}:{line})", provider.library),
                None => format!("{} ({source})", provider.library),
            }
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "Contested model definition '{}' has {} active providers: {providers}. Select one exact provider or remove the overlap before simulation; RSpice will not choose by implicit include order",
        conflict.normalized_name,
        conflict.providers.len()
    )
}

impl ModelLibraryManager {
    /// Create a new manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a library
    pub fn add_library(&mut self, library: ModelLibrary) {
        self.libraries.insert(library.name.clone(), library);
        self.prune_inactive_definition_resolutions();
    }

    /// Remove a library
    pub fn remove_library(&mut self, name: &str) -> Option<ModelLibrary> {
        let removed = self.libraries.remove(name);
        self.pdk_config_libraries.remove(name);
        self.prune_inactive_definition_resolutions();
        removed
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

    /// Shared installed-pack index for a cancellable background catalog query.
    ///
    /// The project model manager can retain large authenticated source
    /// closures, so catalog workers clone only this index rather than cloning
    /// the complete manager.
    pub(crate) fn shared_spice_packs(&self) -> Option<Arc<SpiceLibraryIndex>> {
        self.spice_packs.clone()
    }

    /// Selectable model cards across the shipped packs, or zero when none were
    /// found.
    ///
    /// Subcircuits and nested helper cards are intentionally excluded from the
    /// Models page because they require a different symbol/interface workflow.
    pub fn pack_definition_count(&self) -> usize {
        self.spice_packs
            .as_ref()
            .map_or(0, |index| index.model_count())
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

        let entries = match index.search_model_cards(trimmed, limit) {
            Ok(entries) => entries,
            Err(error) => {
                log::warn!("shipped SPICE model catalog could not be searched: {error}");
                return Vec::new();
            }
        };

        entries
            .into_iter()
            .map(|entry| pack_model_hit(index, entry))
            .collect()
    }

    /// Query every addressable `.model` and `.subckt` definition in the
    /// shipped corpus. The index streams the catalog and returns a bounded
    /// page plus the complete match count.
    pub fn query_pack_parts(
        &self,
        query: &str,
        pack: Option<&str>,
        devices: &[&str],
        offset: usize,
        limit: usize,
    ) -> Result<PackCatalogPage, String> {
        let index = self.spice_packs.as_ref().ok_or_else(|| {
            "The shipped model-pack index is unavailable on this installation".to_owned()
        })?;
        Self::query_pack_parts_from_index(index, query, pack, devices, offset, limit, || false)
    }

    /// Query a shared installed-pack index without retaining or cloning project
    /// model-library state. Used by the UI background worker.
    pub(crate) fn query_pack_parts_from_index(
        index: &SpiceLibraryIndex,
        query: &str,
        pack: Option<&str>,
        devices: &[&str],
        offset: usize,
        limit: usize,
        is_cancelled: impl FnMut() -> bool,
    ) -> Result<PackCatalogPage, String> {
        let (total_matches, entries) = index
            .query_parts_cancellable(query, pack, devices, offset, limit, is_cancelled)
            .map_err(|error| format!("Could not query the shipped parts catalog: {error}"))?;
        Ok(PackCatalogPage {
            total_matches,
            hits: entries
                .into_iter()
                .map(|entry| pack_model_hit(index, entry))
                .collect(),
        })
    }

    /// Exact addressable part counts grouped by canonical device class.
    pub fn pack_device_counts(&self) -> Result<BTreeMap<String, usize>, String> {
        let index = self.spice_packs.as_ref().ok_or_else(|| {
            "The shipped model-pack index is unavailable on this installation".to_owned()
        })?;
        index
            .part_device_counts()
            .map_err(|error| format!("Could not count shipped part classes: {error}"))
    }

    /// Revalidate and read a bounded source preview for one exact catalog hit.
    pub fn preview_pack_part(
        &self,
        hit: &PackModelHit,
    ) -> Result<CatalogDefinitionPreview, String> {
        let index = self.spice_packs.as_ref().ok_or_else(|| {
            "The shipped model-pack index is unavailable on this installation".to_owned()
        })?;
        let entry = index
            .find_part(&hit.name)
            .map_err(|error| format!("Could not revalidate shipped part '{}': {error}", hit.name))?
            .into_iter()
            .find(|entry| {
                entry.pack == hit.pack
                    && entry.kind == hit.kind
                    && entry.line == hit.line
                    && entry.source_path(index).as_ref() == hit.source.as_ref()
            })
            .ok_or_else(|| {
                format!(
                    "Shipped part '{}' changed or disappeared after catalog lookup",
                    hit.name
                )
            })?;
        index
            .definition_preview(&entry)
            .map_err(|error| format!("Could not preview shipped part '{}': {error}", hit.name))
    }

    /// Whether this project currently executes any source owned by `pack_id`.
    #[must_use]
    pub fn is_pack_attached(&self, pack_id: &str) -> bool {
        let Some(index) = self.spice_packs.as_ref() else {
            return false;
        };
        let Some(pack) = index.pack(pack_id) else {
            return false;
        };
        let Ok(pack_root) = std::fs::canonicalize(index.root().join(&pack.path)) else {
            return false;
        };
        self.libraries.values().any(|library| {
            library
                .root_path
                .as_deref()
                .is_some_and(|root| root.starts_with(&pack_root))
        })
    }

    /// Attach the declared entry source of one redistributable shipped pack.
    ///
    /// Packs without an entry source remain browseable at definition
    /// granularity. The declared entry and pack root are canonicalized before
    /// parsing so a malformed manifest cannot escape the indexed pack tree.
    pub fn attach_pack(&mut self, pack_id: &str) -> Result<String, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = pack_id;
            return Err(
                "Installed model packs are unavailable in the browser; import authenticated source bytes instead"
                    .to_owned(),
            );
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let (entry, library_name, pack_name) = {
                let index = self.spice_packs.as_ref().ok_or_else(|| {
                    "The shipped model-pack index is unavailable on this installation".to_owned()
                })?;
                let pack = index
                    .pack(pack_id)
                    .ok_or_else(|| format!("Shipped pack '{pack_id}' is not indexed"))?;
                if !pack.redistributable {
                    return Err(format!(
                        "Pack '{}' is browse-only because redistribution authority is not established",
                        pack.name
                    ));
                }
                let relative_entry = pack.entry.as_ref().ok_or_else(|| {
                    format!(
                        "Pack '{}' has no declared entry source; browse and add an individual model instead",
                        pack.name
                    )
                })?;
                let pack_root =
                    std::fs::canonicalize(index.root().join(&pack.path)).map_err(|error| {
                        format!(
                            "Could not resolve the indexed root for shipped pack '{}': {error}",
                            pack.name
                        )
                    })?;
                let entry =
                    std::fs::canonicalize(pack_root.join(relative_entry)).map_err(|error| {
                        format!(
                            "Could not resolve the declared entry for shipped pack '{}': {error}",
                            pack.name
                        )
                    })?;
                if !entry.starts_with(&pack_root) {
                    return Err(format!(
                        "The declared entry for shipped pack '{}' escapes its indexed pack root",
                        pack.name
                    ));
                }
                let source_stem = entry
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .unwrap_or("entry")
                    .to_owned();
                (
                    entry,
                    pack_library_name(&pack.id, &source_stem),
                    pack.name.clone(),
                )
            };

            let previous = self.libraries.get(&library_name).cloned();
            let retained_source_id = previous.as_ref().and_then(|library| {
                if let ModelSourceAuthority::RetainedImport { source_id, .. } =
                    library.source_authority
                {
                    Some(source_id)
                } else {
                    None
                }
            });
            let loaded =
                self.load_library_file_with_name(&entry, None, Some(library_name.as_str()))?;
            if let Err(error) = self.retain_loaded_import(&loaded, retained_source_id) {
                if let Some(previous) = previous {
                    self.libraries.insert(library_name.clone(), previous);
                } else {
                    self.libraries.remove(&library_name);
                }
                return Err(error);
            }
            self.select_library(&loaded);
            log::info!("Attached shipped pack '{pack_name}' as executable library '{loaded}'");
            Ok(loaded)
        }
    }

    /// Detach every executable library loaded from one shipped pack.
    pub fn detach_pack(&mut self, pack_id: &str) -> Result<usize, String> {
        let index = self.spice_packs.as_ref().ok_or_else(|| {
            "The shipped model-pack index is unavailable on this installation".to_owned()
        })?;
        let pack = index
            .pack(pack_id)
            .ok_or_else(|| format!("Shipped pack '{pack_id}' is not indexed"))?;
        let pack_root = std::fs::canonicalize(index.root().join(&pack.path)).map_err(|error| {
            format!(
                "Could not resolve the indexed root for shipped pack '{}': {error}",
                pack.name
            )
        })?;
        let mut names = self
            .libraries
            .iter()
            .filter_map(|(name, library)| {
                library
                    .root_path
                    .as_deref()
                    .is_some_and(|root| root.starts_with(&pack_root))
                    .then(|| name.clone())
            })
            .collect::<Vec<_>>();
        names.sort();
        if names.is_empty() {
            return Err(format!(
                "No executable model library from pack '{}' is attached",
                pack.name
            ));
        }
        for name in &names {
            self.libraries.remove(name);
        }
        if self
            .selected_library
            .as_ref()
            .is_some_and(|selected| names.contains(selected))
        {
            self.selected_library = None;
        }
        Ok(names.len())
    }

    /// Materialize one exact shipped-pack model into an executable external
    /// library and select it.
    ///
    /// The catalogue hit is revalidated against the on-disk index before any
    /// state changes. Only top-level `.model` definitions from sources whose
    /// redistribution is established are activatable; subcircuits need their
    /// own symbol/interface workflow and restricted packs remain browse-only.
    pub fn activate_pack_model(&mut self, hit: &PackModelHit) -> Result<String, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = hit;
            return Err(
                "Shipped model packs are unavailable in the browser; import authenticated source bytes instead"
                    .to_owned(),
            );
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let (source, pack_name, library_name) = {
                let index = self.spice_packs.as_ref().ok_or_else(|| {
                    "The shipped model-pack index is unavailable on this installation".to_owned()
                })?;
                let entry = index
                    .find_part(&hit.name)
                    .map_err(|error| {
                        format!("Could not revalidate shipped model '{}': {error}", hit.name)
                    })?
                    .into_iter()
                    .find(|entry| {
                        entry.pack == hit.pack
                            && entry.line == hit.line
                            && entry.source_path(index).as_ref() == hit.source.as_ref()
                    })
                    .ok_or_else(|| {
                        format!(
                            "Shipped model '{}' changed or disappeared after catalogue lookup",
                            hit.name
                        )
                    })?;
                let pack = index.pack(&entry.pack).ok_or_else(|| {
                    format!("Shipped model pack '{}' is no longer indexed", entry.pack)
                })?;
                if entry.restricted || !pack.redistributable {
                    return Err(format!(
                        "Shipped model '{}' from '{}' is browse-only because redistribution authority is not established",
                        hit.name, pack.name
                    ));
                }
                if !entry.kind.eq_ignore_ascii_case("model") {
                    return Err(format!(
                        "Shipped part '{}' is a subcircuit; activate it through the symbol/subcircuit import workflow",
                        hit.name
                    ));
                }
                let source = entry.source_path(index).ok_or_else(|| {
                    format!("Shipped model '{}' has no resolvable source path", hit.name)
                })?;
                let source_stem = source
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .unwrap_or("models")
                    .to_owned();
                (
                    source,
                    pack.name.clone(),
                    pack_library_name(&entry.pack, &source_stem),
                )
            };

            let previous = self.libraries.get(&library_name).cloned();
            let retained_source_id = previous.as_ref().and_then(|library| {
                if let ModelSourceAuthority::RetainedImport { source_id, .. } =
                    library.source_authority
                {
                    Some(source_id)
                } else {
                    None
                }
            });
            let loaded =
                self.load_library_file_with_name(&source, None, Some(library_name.as_str()))?;
            let contains_target = self.libraries.get(&loaded).is_some_and(|library| {
                library
                    .models
                    .values()
                    .any(|model| model.name.eq_ignore_ascii_case(&hit.name))
            });
            if !contains_target {
                if let Some(previous) = previous {
                    self.libraries.insert(library_name.clone(), previous);
                } else {
                    self.libraries.remove(&library_name);
                }
                return Err(format!(
                    "Shipped source '{}' parsed, but model '{}' was not materialized as a supported top-level device",
                    source.display(),
                    hit.name
                ));
            }
            if let Err(error) = self.retain_loaded_import(&loaded, retained_source_id) {
                if let Some(previous) = previous {
                    self.libraries.insert(library_name.clone(), previous);
                } else {
                    self.libraries.remove(&library_name);
                }
                return Err(error);
            }
            self.select_library(&loaded);
            log::info!(
                "Activated shipped model '{}' from pack '{}' as library '{}'",
                hit.name,
                pack_name,
                loaded
            );
            Ok(loaded)
        }
    }

    /// Attach one redistributable shipped source and return its exact
    /// top-level subcircuit terminal contract for symbol construction.
    pub fn activate_pack_subcircuit(
        &mut self,
        hit: &PackModelHit,
    ) -> Result<ActivatedPackSubcircuit, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = hit;
            return Err(
                "Shipped model packs are unavailable in the browser; import authenticated source bytes instead"
                    .to_owned(),
            );
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let (interface, pack_name, library_name) = {
                let index = self.spice_packs.as_ref().ok_or_else(|| {
                    "The shipped model-pack index is unavailable on this installation".to_owned()
                })?;
                let entry = index
                    .find_part(&hit.name)
                    .map_err(|error| {
                        format!(
                            "Could not revalidate shipped subcircuit '{}': {error}",
                            hit.name
                        )
                    })?
                    .into_iter()
                    .find(|entry| {
                        entry.pack == hit.pack
                            && entry.line == hit.line
                            && entry.source_path(index).as_ref() == hit.source.as_ref()
                    })
                    .ok_or_else(|| {
                        format!(
                            "Shipped subcircuit '{}' changed or disappeared after catalogue lookup",
                            hit.name
                        )
                    })?;
                let pack = index.pack(&entry.pack).ok_or_else(|| {
                    format!("Shipped model pack '{}' is no longer indexed", entry.pack)
                })?;
                if entry.restricted || !pack.redistributable {
                    return Err(format!(
                        "Shipped subcircuit '{}' from '{}' is browse-only because redistribution authority is not established",
                        hit.name, pack.name
                    ));
                }
                if !entry.kind.eq_ignore_ascii_case("subckt") {
                    return Err(format!(
                        "Shipped part '{}' is not a top-level subcircuit",
                        hit.name
                    ));
                }
                let mut interface: CatalogSubcircuitInterface =
                    index.subcircuit_interface(&entry).map_err(|error| {
                        format!(
                            "Could not inspect shipped subcircuit '{}': {error}",
                            hit.name
                        )
                    })?;
                interface.source_path =
                    std::fs::canonicalize(&interface.source_path).map_err(|error| {
                        format!(
                            "Could not canonicalize shipped subcircuit source '{}': {error}",
                            interface.source_path.display()
                        )
                    })?;
                let source_stem = interface
                    .source_path
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .unwrap_or("subcircuits")
                    .to_owned();
                (
                    interface,
                    pack.name.clone(),
                    pack_library_name(&entry.pack, &source_stem),
                )
            };

            let previous = self.libraries.get(&library_name).cloned();
            let retained_source_id = previous.as_ref().and_then(|library| {
                if let ModelSourceAuthority::RetainedImport { source_id, .. } =
                    library.source_authority
                {
                    Some(source_id)
                } else {
                    None
                }
            });
            let loaded = self.load_library_file_with_name(
                &interface.source_path,
                None,
                Some(&library_name),
            )?;
            let materialized_error = self
                .libraries
                .get(&loaded)
                .and_then(|library| {
                    library
                        .subcircuits
                        .values()
                        .find(|candidate| {
                            candidate.name.eq_ignore_ascii_case(&interface.name)
                                && candidate.file_path.as_deref()
                                    == Some(interface.source_path.as_path())
                                && candidate.source_line == Some(interface.start_line)
                        })
                })
                .map_or_else(
                    || {
                        Some(format!(
                            "Shipped source '{}' parsed, but subcircuit '{}' was not materialized as a supported top-level definition",
                            interface.source_path.display(),
                            interface.name
                        ))
                    },
                    |candidate| {
                        if candidate.name != interface.name
                            || candidate.ports != interface.ports
                        {
                            Some(format!(
                                "Shipped subcircuit '{}' changed while its source interface was being activated",
                                interface.name
                            ))
                        } else {
                            None
                        }
                    },
                );
            if let Some(error) = materialized_error {
                if let Some(previous) = previous.clone() {
                    self.libraries.insert(library_name.clone(), previous);
                } else {
                    self.libraries.remove(&library_name);
                }
                return Err(error);
            }
            if let Err(error) = self.retain_loaded_import(&loaded, retained_source_id) {
                if let Some(previous) = previous {
                    self.libraries.insert(library_name.clone(), previous);
                } else {
                    self.libraries.remove(&library_name);
                }
                return Err(error);
            }
            self.select_library(&loaded);
            log::info!(
                "Activated shipped subcircuit '{}' from pack '{}' as library '{}'",
                interface.name,
                pack_name,
                loaded
            );
            Ok(ActivatedPackSubcircuit {
                library: loaded,
                name: interface.name,
                ports: interface.ports,
                source_path: interface.source_path,
            })
        }
    }

    fn retain_loaded_import(
        &mut self,
        library_name: &str,
        source_id: Option<crate::product::ModelSourceId>,
    ) -> Result<(), String> {
        let library = self
            .libraries
            .get_mut(library_name)
            .ok_or_else(|| format!("Loaded model library '{library_name}' disappeared"))?;
        let root = library.root_path.as_ref().ok_or_else(|| {
            format!("Loaded model library '{library_name}' has no root source identity")
        })?;
        let digest = library
            .source_closure
            .iter()
            .find(|source| &source.path == root)
            .map(|source| source.digest)
            .ok_or_else(|| {
                format!(
                    "Loaded model library '{library_name}' has no authenticated root source pin"
                )
            })?;
        if library.source_contents.len() != library.source_closure.len() {
            return Err(format!(
                "Loaded model library '{library_name}' does not retain its complete authenticated source closure"
            ));
        }
        library.source_authority = ModelSourceAuthority::RetainedImport {
            source_id: source_id.unwrap_or_else(crate::product::ModelSourceId::new),
            digest,
        };
        Ok(())
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

    /// Total addressable device-model and subcircuit definitions.
    pub fn total_definition_count(&self) -> usize {
        self.libraries
            .values()
            .map(|library| library.models.len() + library.subcircuits.len())
            .sum()
    }

    /// Stable identity of the persisted model catalogue relevant to source
    /// preparation. Browser filters, selection, shipped-pack indexes, and
    /// audit ledgers are deliberately excluded.
    pub(crate) fn execution_catalog_digest(&self) -> ContentDigest {
        let mut libraries = self.libraries.values().collect::<Vec<_>>();
        libraries.sort_by(|left, right| left.name.cmp(&right.name));
        let mut hasher = Sha256::new();
        hasher.update(b"rspice.model-execution-catalog/v2\0");
        for library in libraries {
            let bytes = serde_json::to_vec(library)
                .unwrap_or_else(|error| format!("serialization-error:{error}").into_bytes());
            hasher.update((bytes.len() as u64).to_le_bytes());
            hasher.update(bytes);
        }
        for resolution in self.definition_resolutions.values() {
            let bytes = serde_json::to_vec(resolution)
                .unwrap_or_else(|error| format!("serialization-error:{error}").into_bytes());
            hasher.update((bytes.len() as u64).to_le_bytes());
            hasher.update(bytes);
        }
        ContentDigest::from_bytes(hasher.finalize().into())
    }

    /// Case-insensitive model names currently provided by multiple executable
    /// libraries.
    #[must_use]
    pub fn definition_conflicts(&self) -> Vec<ModelDefinitionConflict> {
        let mut providers = BTreeMap::<String, Vec<ModelDefinitionProvider>>::new();
        for library in self.libraries_sorted() {
            let mut models = library.models.values().collect::<Vec<_>>();
            models.sort_by_key(|model| model.name.to_ascii_lowercase());
            for model in models {
                providers
                    .entry(model.name.to_ascii_lowercase())
                    .or_default()
                    .push(ModelDefinitionProvider {
                        library: library.name.clone(),
                        model: model.name.clone(),
                        source: model
                            .file_path
                            .clone()
                            .or_else(|| library.root_path.clone()),
                        source_line: model.source_line,
                    });
            }
        }
        providers
            .into_iter()
            .filter_map(|(normalized_name, mut providers)| {
                providers.sort_by(|left, right| {
                    left.library
                        .to_ascii_lowercase()
                        .cmp(&right.library.to_ascii_lowercase())
                        .then_with(|| left.model.cmp(&right.model))
                        .then_with(|| left.source.cmp(&right.source))
                        .then_with(|| left.source_line.cmp(&right.source_line))
                });
                (providers.len() > 1).then_some(ModelDefinitionConflict {
                    normalized_name,
                    providers,
                })
            })
            .collect()
    }

    /// Return the explicit provider choice for a normalized model name.
    #[must_use]
    pub fn definition_resolution(&self, model_name: &str) -> Option<&ModelDefinitionResolution> {
        self.definition_resolutions
            .get(&model_name.trim().to_ascii_lowercase())
    }

    /// Stable persisted provider choices.
    #[must_use]
    pub fn definition_resolutions(&self) -> Vec<&ModelDefinitionResolution> {
        self.definition_resolutions.values().collect()
    }

    /// Select one exact provider for a currently contested model name.
    pub fn resolve_definition_conflict(
        &mut self,
        model_name: &str,
        provider_library: &str,
        provider_model: &str,
    ) -> Result<(), String> {
        let normalized_name = model_name.trim().to_ascii_lowercase();
        let conflict = self
            .definition_conflicts()
            .into_iter()
            .find(|conflict| conflict.normalized_name == normalized_name)
            .ok_or_else(|| {
                format!(
                    "Model definition '{}' is not currently contested",
                    model_name.trim()
                )
            })?;
        if conflict.providers.iter().any(|left| {
            conflict
                .providers
                .iter()
                .any(|right| left != right && left.library.eq_ignore_ascii_case(&right.library))
        }) {
            return Err(format!(
                "Contested model definition '{}' has multiple case-colliding providers inside one library; repair that source before simulation",
                conflict.normalized_name
            ));
        }
        let provider = conflict
            .providers
            .iter()
            .find(|provider| {
                provider.library == provider_library && provider.model == provider_model
            })
            .ok_or_else(|| {
                format!(
                    "Provider '{provider_library}/{provider_model}' is not an exact active provider of contested model '{}'",
                    conflict.normalized_name
                )
            })?;
        self.definition_resolutions.insert(
            normalized_name.clone(),
            ModelDefinitionResolution {
                normalized_name,
                provider_library: provider.library.clone(),
                provider_model: provider.model.clone(),
            },
        );
        Ok(())
    }

    /// Remove one explicit provider selection.
    pub fn clear_definition_resolution(&mut self, model_name: &str) -> bool {
        self.definition_resolutions
            .remove(&model_name.trim().to_ascii_lowercase())
            .is_some()
    }

    /// Restore persisted resolution contracts without accepting stale or
    /// duplicate keys.
    pub(crate) fn restore_definition_resolutions(
        &mut self,
        resolutions: Vec<ModelDefinitionResolution>,
    ) -> Result<(), String> {
        let mut restored = BTreeMap::new();
        for resolution in resolutions {
            let normalized = resolution.normalized_name.trim().to_ascii_lowercase();
            if normalized.is_empty()
                || normalized != resolution.normalized_name
                || resolution.provider_library.trim().is_empty()
                || resolution.provider_model.trim().is_empty()
            {
                return Err(
                    "Model-definition resolution contains an invalid canonical provider identity"
                        .to_owned(),
                );
            }
            if restored.insert(normalized.clone(), resolution).is_some() {
                return Err(format!(
                    "Model-definition resolution '{}' is repeated",
                    normalized
                ));
            }
        }
        self.definition_resolutions = restored;
        self.validate_recorded_definition_resolutions()
    }

    /// Reject an executable catalog whose winner would otherwise depend on
    /// implicit include or map ordering.
    pub fn validate_definition_resolution(&self) -> Result<(), String> {
        self.definition_resolution_order().map(|_| ())
    }

    /// Clear all
    pub fn clear(&mut self) {
        self.libraries.clear();
        self.definition_resolutions.clear();
        self.pdk_config_libraries.clear();
        self.selected_library = None;
    }

    fn prune_inactive_definition_resolutions(&mut self) {
        let active = self
            .definition_conflicts()
            .into_iter()
            .map(|conflict| conflict.normalized_name)
            .collect::<HashSet<_>>();
        self.definition_resolutions
            .retain(|normalized_name, _| active.contains(normalized_name));
    }

    fn definition_resolution_order(&self) -> Result<Vec<String>, String> {
        let conflicts = self.definition_conflicts();
        self.validate_recorded_definition_resolutions()?;
        self.validate_active_subcircuit_definitions()?;

        let mut outgoing = BTreeMap::<String, BTreeSet<String>>::new();
        let mut indegree = BTreeMap::<String, usize>::new();
        for library in self.libraries_sorted() {
            outgoing.entry(library.name.clone()).or_default();
            indegree.entry(library.name.clone()).or_default();
        }
        for conflict in &conflicts {
            let resolution = self
                .definition_resolutions
                .get(&conflict.normalized_name)
                .ok_or_else(|| unresolved_definition_error(conflict))?;
            if resolution.normalized_name != conflict.normalized_name {
                return Err(format!(
                    "Model-definition resolution key '{}' does not match its canonical name '{}'",
                    conflict.normalized_name, resolution.normalized_name
                ));
            }
            let winner = conflict
                .providers
                .iter()
                .find(|provider| {
                    provider.library == resolution.provider_library
                        && provider.model == resolution.provider_model
                })
                .ok_or_else(|| {
                    format!(
                        "Resolution for contested model '{}' selects stale provider '{}/{}'; select one of the current exact providers",
                        conflict.normalized_name,
                        resolution.provider_library,
                        resolution.provider_model
                    )
                })?;
            for loser in &conflict.providers {
                if loser == winner {
                    continue;
                }
                if loser.library.eq_ignore_ascii_case(&winner.library) {
                    return Err(format!(
                        "Contested model definition '{}' has multiple case-colliding providers inside library '{}'; library precedence cannot resolve definitions within one source",
                        conflict.normalized_name, winner.library
                    ));
                }
                let inserted = outgoing
                    .entry(winner.library.clone())
                    .or_default()
                    .insert(loser.library.clone());
                if inserted {
                    *indegree.entry(loser.library.clone()).or_default() += 1;
                }
            }
        }

        let mut ready = indegree
            .iter()
            .filter_map(|(library, degree)| (*degree == 0).then_some(library.clone()))
            .collect::<BTreeSet<_>>();
        let mut order = Vec::with_capacity(indegree.len());
        while let Some(library) = ready.pop_first() {
            order.push(library.clone());
            for dependent in outgoing.get(&library).into_iter().flatten() {
                let degree = indegree
                    .get_mut(dependent)
                    .expect("every precedence target is an active library");
                *degree = degree.saturating_sub(1);
                if *degree == 0 {
                    ready.insert(dependent.clone());
                }
            }
        }
        if order.len() != indegree.len() {
            let cyclic = indegree
                .into_iter()
                .filter_map(|(library, degree)| (degree > 0).then_some(library))
                .collect::<Vec<_>>()
                .join(", ");
            return Err(format!(
                "Model-definition provider selections create a cyclic precedence contract across libraries: {cyclic}. Choose compatible providers or split the overlapping sources"
            ));
        }
        Ok(order)
    }

    fn validate_active_subcircuit_definitions(&self) -> Result<(), String> {
        let mut providers = BTreeMap::<String, Vec<String>>::new();
        for library in self.libraries_sorted() {
            if !library.source_authority.has_execution_source() {
                continue;
            }
            for subcircuit in library.subcircuits.values().filter(|subcircuit| {
                subcircuit.section.is_none()
                    || subcircuit.section.as_deref() == library.selected_corner.as_deref()
            }) {
                providers
                    .entry(subcircuit.name.to_ascii_lowercase())
                    .or_default()
                    .push(format!(
                        "{}/{}",
                        library.name,
                        subcircuit.section.as_deref().unwrap_or("top-level")
                    ));
            }
        }
        if let Some((name, mut providers)) = providers
            .into_iter()
            .find(|(_, providers)| providers.len() > 1)
        {
            providers.sort();
            return Err(format!(
                "Active subcircuit definition '{name}' has multiple providers ({}). Remove the overlap or select non-overlapping library sections before simulation; RSpice will not choose a subcircuit by implicit include order",
                providers.join(", ")
            ));
        }
        Ok(())
    }

    fn validate_recorded_definition_resolutions(&self) -> Result<(), String> {
        let conflicts = self.definition_conflicts();
        for (normalized_name, resolution) in &self.definition_resolutions {
            let conflict = conflicts
                .iter()
                .find(|conflict| conflict.normalized_name == *normalized_name)
                .ok_or_else(|| {
                    format!(
                        "Model-definition resolution '{normalized_name}' is stale because that name is no longer contested"
                    )
                })?;
            if resolution.normalized_name != *normalized_name {
                return Err(format!(
                    "Model-definition resolution key '{normalized_name}' does not match its canonical name '{}'",
                    resolution.normalized_name
                ));
            }
            if !conflict.providers.iter().any(|provider| {
                provider.library == resolution.provider_library
                    && provider.model == resolution.provider_model
            }) {
                return Err(format!(
                    "Resolution for contested model '{}' selects stale provider '{}/{}'; select one of the current exact providers",
                    conflict.normalized_name,
                    resolution.provider_library,
                    resolution.provider_model
                ));
            }
        }
        Ok(())
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
        self.load_library_file_with_name(path, section, None)
    }

    fn load_library_file_with_name(
        &mut self,
        path: impl AsRef<std::path::Path>,
        section: Option<&str>,
        preferred_name: Option<&str>,
    ) -> Result<String, String> {
        use rspice_core::library::LibParser;

        let path = std::fs::canonicalize(path.as_ref()).map_err(|error| {
            format!(
                "Failed to resolve model library '{}': {error}",
                path.as_ref().display()
            )
        })?;
        let base_dir = path.parent().unwrap_or(std::path::Path::new("."));
        let lib_name = preferred_name.map(str::to_owned).unwrap_or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unnamed")
                .to_string()
        });

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
        library.subcircuits.clear();
        library.model_definition_metadata.clear();
        library.model_qualification.clear();
        library.model_correlation.clear();
        library.corners.clear();
        library.selected_corner = None;

        for section_name in result.section_names() {
            let mut corner =
                ProcessCorner::from_composite_section(section_name, path.clone(), false);
            corner.description = format!("Process corner from {lib_name}");
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
        Self::insert_parsed_subcircuits(&mut library, &result.top_level_subcircuits, &path, None)?;
        for lib_section in &result.sections {
            Self::insert_parsed_subcircuits(
                &mut library,
                &lib_section.subcircuits,
                &path,
                Some(&lib_section.name),
            )?;
        }

        if let Some(section_name) = selected_section.as_deref() {
            if let Some(lib_section) = result.get_section(section_name) {
                for model in &lib_section.models {
                    let device_model = Self::convert_parsed_model_in_section(
                        model,
                        &path,
                        Some(&lib_section.name),
                    );
                    Self::insert_case_insensitive_active_model(&mut library.models, device_model);
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
        if library.models.is_empty() && library.subcircuits.is_empty() {
            return Err(format!(
                "Model library '{}' contains no supported device models or addressable subcircuits",
                path.display()
            ));
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
        library.source_authority = ModelSourceAuthority::RetainedImport {
            source_id: crate::product::ModelSourceId::new(),
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
        library.selected_corner = None;
        for section_name in result.section_names() {
            let mut corner =
                ProcessCorner::from_composite_section(section_name, root.clone(), false);
            corner.description = format!("Process corner from {lib_name}");
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
        Self::insert_parsed_subcircuits(&mut library, &result.top_level_subcircuits, &root, None)?;
        for lib_section in &result.sections {
            Self::insert_parsed_subcircuits(
                &mut library,
                &lib_section.subcircuits,
                &root,
                Some(&lib_section.name),
            )?;
        }
        if let Some(section_name) = selected_section.as_deref() {
            let lib_section = result.get_section(section_name).ok_or_else(|| {
                format!(
                    "Section '{section_name}' not found. Available: {:?}",
                    result.section_names()
                )
            })?;
            for model in &lib_section.models {
                let device_model =
                    Self::convert_parsed_model_in_section(model, &root, Some(&lib_section.name));
                Self::insert_case_insensitive_active_model(&mut library.models, device_model);
            }
            library.selected_corner = Some(lib_section.name.clone());
            if let Some(corner) = library.corners.get_mut(&lib_section.name) {
                corner.is_default = true;
            }
        }
        if library.models.is_empty() && library.subcircuits.is_empty() {
            return Err(
                "Uploaded model library contains no supported device models or addressable subcircuits"
                    .to_owned(),
            );
        }
        if self.libraries.contains_key(&lib_name) {
            return Err(format!(
                "Model library '{lib_name}' already exists; remove it before importing replacement bytes"
            ));
        }
        self.libraries.insert(lib_name.clone(), library);
        Ok(lib_name)
    }

    /// Import one complete browser/mobile model-source bundle.
    ///
    /// Browser file handles do not expose trustworthy host paths. Every
    /// selected member is therefore normalized to a sibling file in one
    /// content-addressed virtual directory. A unique root and every `.include`
    /// or external `.lib` edge must be provable from the selected bytes;
    /// missing, ambiguous, duplicate, nested-path, and unreachable members
    /// fail closed before the manager is mutated.
    pub fn load_library_bundle_bytes(
        &mut self,
        files: Vec<(String, Vec<u8>)>,
        section: Option<&str>,
    ) -> Result<String, String> {
        use rspice_core::library::{LibParser, ResolvedLibDependency};

        if files.is_empty() {
            return Err("Model library bundle contains no files".to_owned());
        }

        let mut normalized = Vec::<(String, Vec<u8>, String)>::with_capacity(files.len());
        let mut names = BTreeMap::<String, String>::new();
        for (file_name, bytes) in files {
            let safe_name = std::path::Path::new(&file_name)
                .file_name()
                .and_then(|name| name.to_str())
                .filter(|name| !name.is_empty())
                .ok_or_else(|| "Model library bundle contains an invalid file name".to_owned())?
                .to_owned();
            if safe_name != file_name {
                return Err(format!(
                    "Browser model bundle member '{file_name}' is not a plain sibling file name"
                ));
            }
            let key = safe_name.to_ascii_lowercase();
            if let Some(existing) = names.insert(key, safe_name.clone()) {
                return Err(format!(
                    "Browser model bundle repeats case-insensitive file name '{existing}'/'{safe_name}'"
                ));
            }
            let content = rspice_core::netlist::decode_source_bytes(&bytes).map_err(|error| {
                format!("Uploaded model source '{safe_name}' cannot be decoded: {error}")
            })?;
            normalized.push((safe_name, bytes, content));
        }
        normalized.sort_by(|left, right| {
            left.0
                .to_ascii_lowercase()
                .cmp(&right.0.to_ascii_lowercase())
                .then_with(|| left.0.cmp(&right.0))
        });

        let mut bundle_hasher = Sha256::new();
        bundle_hasher.update(b"rspice.browser-model-bundle/v1\0");
        for (name, bytes, _) in &normalized {
            bundle_hasher.update((name.len() as u64).to_le_bytes());
            bundle_hasher.update(name.as_bytes());
            bundle_hasher.update((bytes.len() as u64).to_le_bytes());
            bundle_hasher.update(Sha256::digest(bytes));
        }
        let bundle_digest =
            crate::product::ContentDigest::from_bytes(bundle_hasher.finalize().into());
        let virtual_directory =
            PathBuf::from(format!("/rspice-browser/model-sources/{bundle_digest}"));

        let paths = normalized
            .iter()
            .map(|(name, _, _)| (name.clone(), virtual_directory.join(name)))
            .collect::<BTreeMap<_, _>>();
        let mut dependencies = Vec::<ResolvedLibDependency>::new();
        let mut source_edges = Vec::<ModelSourceEdge>::new();
        let mut targets = BTreeSet::<PathBuf>::new();
        for (name, _, content) in &normalized {
            let owner = paths
                .get(name)
                .expect("normalized browser source has a virtual path")
                .clone();
            for requested_path in root_external_source_paths(content) {
                let requested = normalize_portable_path_text(&requested_path)?;
                if is_portable_absolute_text(&requested) || requested.contains('/') {
                    return Err(format!(
                        "Browser model source '{name}' requests '{requested_path}'. Multi-file browser import can authenticate sibling includes only; import this closure from desktop or flatten it without changing include names."
                    ));
                }
                let target = paths.get(&requested).cloned().ok_or_else(|| {
                    format!(
                        "Browser model source '{name}' requires missing sibling '{requested_path}'"
                    )
                })?;
                targets.insert(target.clone());
                dependencies.push(ResolvedLibDependency {
                    owner: owner.clone(),
                    requested_path: requested_path.clone(),
                    target: target.clone(),
                });
                source_edges.push(ModelSourceEdge {
                    owner: owner.clone(),
                    requested_path,
                    target,
                });
            }
        }
        dependencies.sort_by(|left, right| {
            left.owner
                .cmp(&right.owner)
                .then_with(|| left.requested_path.cmp(&right.requested_path))
                .then_with(|| left.target.cmp(&right.target))
        });
        dependencies.dedup_by(|left, right| {
            left.owner == right.owner
                && left.requested_path == right.requested_path
                && left.target == right.target
        });
        source_edges.sort();
        source_edges.dedup();

        let roots = paths
            .values()
            .filter(|path| !targets.contains(*path))
            .cloned()
            .collect::<Vec<_>>();
        let root = match roots.as_slice() {
            [root] => root.clone(),
            [] => {
                return Err(
                    "Browser model bundle has no unique root; its dependency graph is cyclic"
                        .to_owned(),
                );
            }
            roots => {
                return Err(format!(
                    "Browser model bundle has {} independent roots ({}); select one complete include closure at a time",
                    roots.len(),
                    roots
                        .iter()
                        .filter_map(|path| path.file_name())
                        .map(|name| name.to_string_lossy())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        };

        let mut source_closure = normalized
            .iter()
            .map(|(name, bytes, _)| ModelSourcePin {
                path: paths
                    .get(name)
                    .expect("normalized browser source has a virtual path")
                    .clone(),
                digest: crate::product::ContentDigest::from_bytes(Sha256::digest(bytes).into()),
            })
            .collect::<Vec<_>>();
        source_closure.sort_by(|left, right| left.path.cmp(&right.path));
        if let Some(unreachable) = first_unreachable_source(&root, &source_closure, &source_edges) {
            return Err(format!(
                "Browser model bundle contains unreachable member '{}'",
                unreachable.display()
            ));
        }
        let mut source_contents = normalized
            .into_iter()
            .map(|(name, bytes, _)| ModelSourceContent {
                path: paths
                    .get(&name)
                    .expect("normalized browser source has a virtual path")
                    .clone(),
                bytes,
            })
            .collect::<Vec<_>>();
        source_contents.sort_by(|left, right| left.path.cmp(&right.path));

        let mut parser = LibParser::new(&virtual_directory);
        let result = parser.parse_authenticated_closure(
            root.clone(),
            source_contents
                .iter()
                .map(|content| (content.path.clone(), content.bytes.clone())),
            dependencies,
        )?;
        if !result.is_ok() {
            return Err(format!(
                "Uploaded model bundle contains parse or dependency errors: {}",
                result
                    .errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
            ));
        }

        let lib_name = root
            .file_stem()
            .and_then(|name| name.to_str())
            .filter(|name| !name.is_empty())
            .unwrap_or("uploaded-models")
            .to_owned();
        if self.libraries.contains_key(&lib_name) {
            return Err(format!(
                "Model library '{lib_name}' already exists; remove it before importing replacement bytes"
            ));
        }
        let root_digest = source_closure
            .iter()
            .find(|pin| pin.path == root)
            .map(|pin| pin.digest)
            .expect("authenticated browser root belongs to the closure");
        let mut library = ModelLibrary::new(&lib_name);
        library.root_path = Some(root.clone());
        library.source_authority = ModelSourceAuthority::RetainedImport {
            source_id: crate::product::ModelSourceId::new(),
            digest: root_digest,
        };
        library.source_closure = source_closure;
        library.source_contents = source_contents;
        library.source_edges = source_edges;
        library.corners.clear();
        library.selected_corner = None;
        for section_name in result.section_names() {
            let mut corner =
                ProcessCorner::from_composite_section(section_name, root.clone(), false);
            corner.description = format!("Process corner from {lib_name}");
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
        Self::insert_parsed_subcircuits(&mut library, &result.top_level_subcircuits, &root, None)?;
        for lib_section in &result.sections {
            Self::insert_parsed_subcircuits(
                &mut library,
                &lib_section.subcircuits,
                &root,
                Some(&lib_section.name),
            )?;
        }
        if let Some(section_name) = selected_section.as_deref() {
            let lib_section = result.get_section(section_name).ok_or_else(|| {
                format!(
                    "Section '{section_name}' not found. Available: {:?}",
                    result.section_names()
                )
            })?;
            for model in &lib_section.models {
                let device_model =
                    Self::convert_parsed_model_in_section(model, &root, Some(&lib_section.name));
                Self::insert_case_insensitive_active_model(&mut library.models, device_model);
            }
            library.selected_corner = Some(lib_section.name.clone());
            if let Some(corner) = library.corners.get_mut(&lib_section.name) {
                corner.is_default = true;
            }
        }
        if library.models.is_empty() && library.subcircuits.is_empty() {
            return Err(
                "Uploaded model bundle contains no supported device models or addressable subcircuits"
                    .to_owned(),
            );
        }
        self.libraries.insert(lib_name.clone(), library);
        Ok(lib_name)
    }

    /// Rebuild the browseable active-card projection after a corner contract
    /// changes. The projection is derived only from the already authenticated
    /// closure; it never consults a search path or silently substitutes a
    /// typical section.
    pub(crate) fn rebuild_active_model_projection(
        &mut self,
        library_name: &str,
    ) -> Result<(), String> {
        use rspice_core::library::LibParser;

        let library = self
            .libraries
            .get(library_name)
            .cloned()
            .ok_or_else(|| format!("Model library '{library_name}' does not exist"))?;
        let root = library.root_path.as_ref().ok_or_else(|| {
            format!("Model library '{library_name}' has no authenticated source root")
        })?;
        if library.source_contents.is_empty() {
            return Err(format!(
                "Model library '{library_name}' has no retained authenticated bytes from which to rebuild its active model cards"
            ));
        }
        let sources = library
            .source_contents
            .iter()
            .map(|content| (content.path.clone(), content.bytes.clone()));
        let dependencies =
            library
                .source_edges
                .iter()
                .map(|edge| rspice_core::library::ResolvedLibDependency {
                    owner: edge.owner.clone(),
                    requested_path: edge.requested_path.clone(),
                    target: edge.target.clone(),
                });
        let mut parser = LibParser::new(root.parent().unwrap_or_else(|| std::path::Path::new(".")));
        let parsed = parser
            .parse_authenticated_closure(root.clone(), sources, dependencies)
            .map_err(|error| {
                format!(
                    "Model library '{library_name}' retained closure cannot be authenticated: {error}"
                )
            })?;
        if !parsed.is_ok() {
            return Err(format!(
                "Model library '{library_name}' retained closure does not parse: {}",
                parsed
                    .errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
            ));
        }

        let mut models = HashMap::new();
        for model in &parsed.top_level_models {
            let model = Self::convert_parsed_model(model, root);
            models.insert(model.name.clone(), model);
        }

        let section_names = active_model_section_names(&library)?;
        let mut section_definitions = HashMap::<String, (String, String)>::new();
        for requested_section in section_names {
            let matching_sections = parsed
                .sections
                .iter()
                .filter(|section| section.name.eq_ignore_ascii_case(&requested_section))
                .collect::<Vec<_>>();
            if matching_sections.is_empty() {
                return Err(format!(
                    "Model library '{library_name}' selected corner requires missing section '{requested_section}'"
                ));
            }
            for section in matching_sections {
                for parsed_model in &section.models {
                    let canonical = parsed_model.name.to_ascii_lowercase();
                    if let Some((first_section, first_name)) = section_definitions
                        .insert(canonical, (section.name.clone(), parsed_model.name.clone()))
                    {
                        return Err(format!(
                            "Model library '{library_name}' selected corner resolves model '{}' from both sections '{}' and '{}'; active section composition is ambiguous",
                            first_name, first_section, section.name
                        ));
                    }
                    let model = Self::convert_parsed_model_in_section(
                        parsed_model,
                        root,
                        Some(&section.name),
                    );
                    Self::insert_case_insensitive_active_model(&mut models, model);
                }
            }
        }
        if models.is_empty() && library.subcircuits.is_empty() {
            return Err(format!(
                "Model library '{library_name}' selected corner materializes no supported model cards or subcircuits"
            ));
        }

        self.libraries
            .get_mut(library_name)
            .expect("library identity was retained during projection rebuild")
            .models = models;
        Ok(())
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
    /// Reconcile every supported model source in the authoritative PDK scan.
    ///
    /// Libraries published by the previous scan are removed from the
    /// candidate first. Direct imports, retained pack sources, project-owned
    /// definitions, and project-restored external dependencies are not owned
    /// by this host configuration and remain untouched. The replacement is
    /// published only when every source parses successfully.
    pub fn load_from_pdk_config(
        &mut self,
        pdk_config: &crate::state::pdk_config::PdkConfig,
    ) -> Result<usize, Vec<String>> {
        let mut candidate = self.clone();
        let previously_managed = std::mem::take(&mut candidate.pdk_config_libraries);
        candidate.libraries.retain(|name, library| {
            !previously_managed.contains(name)
                || library.source_authority != ModelSourceAuthority::External
        });
        let mut loaded = 0;
        let mut errors = Vec::new();
        let mut sources = BTreeSet::new();

        for file in pdk_config.discovered_files() {
            if !crate::state::pdk_config::MODEL_FILE_EXTENSIONS.contains(&file.extension.as_str()) {
                continue;
            }
            match std::fs::canonicalize(&file.path) {
                Ok(path) => {
                    sources.insert(path);
                }
                Err(error) => errors.push(format!(
                    "{}: failed to resolve discovered PDK source: {error}",
                    file.path.display()
                )),
            }
        }

        for source in sources {
            let library_name = pdk_config_library_name(&candidate, &source);
            match candidate.load_library_file_with_name(&source, None, Some(&library_name)) {
                Ok(name) => {
                    candidate.pdk_config_libraries.insert(name);
                    loaded += 1;
                }
                Err(error) => errors.push(format!("{}: {error}", source.display())),
            }
        }

        if errors.is_empty() {
            if candidate
                .selected_library
                .as_ref()
                .is_some_and(|selected| !candidate.libraries.contains_key(selected))
            {
                candidate.selected_library = None;
            }
            candidate.prune_inactive_definition_resolutions();
            *self = candidate;
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
                    section: None,
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

    fn insert_case_insensitive_active_model(
        models: &mut HashMap<String, DeviceModel>,
        model: DeviceModel,
    ) {
        if let Some(existing) = models
            .keys()
            .find(|name| name.eq_ignore_ascii_case(&model.name))
            .cloned()
        {
            models.remove(&existing);
        }
        models.insert(model.name.clone(), model);
    }

    /// Convert a parsed model from the core library to UI DeviceModel
    pub(crate) fn convert_parsed_model(
        model: &rspice_core::library::ParsedModel,
        file_path: &std::path::Path,
    ) -> DeviceModel {
        Self::convert_parsed_model_in_section(model, file_path, None)
    }

    /// Convert a parsed card while retaining the exact active `.lib` section.
    pub(crate) fn convert_parsed_model_in_section(
        model: &rspice_core::library::ParsedModel,
        file_path: &std::path::Path,
        section: Option<&str>,
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
            section: section.map(str::to_owned),
            parameters: model.parameters.clone(),
            string_parameters: model.string_params.clone(),
            source_line: model.source_line,
        }
    }

    pub(crate) fn convert_parsed_subcircuit(
        subcircuit: &rspice_core::library::ParsedSubcircuit,
        file_path: &Path,
        section: Option<&str>,
    ) -> ModelSubcircuitInterface {
        ModelSubcircuitInterface {
            name: subcircuit.name.clone(),
            ports: subcircuit.pins.clone(),
            parameter_defaults: subcircuit
                .parameter_defaults
                .iter()
                .map(|(name, value)| (name.clone(), value.clone()))
                .collect(),
            description: subcircuit.description.clone(),
            file_path: Some(
                subcircuit
                    .source_file
                    .as_deref()
                    .unwrap_or(file_path)
                    .to_path_buf(),
            ),
            source_line: subcircuit.source_line,
            section: section.map(str::to_owned),
        }
    }

    fn insert_parsed_subcircuits(
        library: &mut ModelLibrary,
        parsed: &[rspice_core::library::ParsedSubcircuit],
        file_path: &Path,
        section: Option<&str>,
    ) -> Result<(), String> {
        for subcircuit in parsed {
            let interface = Self::convert_parsed_subcircuit(subcircuit, file_path, section);
            let key = subcircuit_interface_key(interface.section.as_deref(), &interface.name);
            if let Some(existing) = library
                .subcircuits
                .keys()
                .find(|existing| existing.eq_ignore_ascii_case(&key))
            {
                return Err(format!(
                    "Subcircuit '{}' is defined more than once in the same library section (first identity '{}')",
                    interface.name, existing
                ));
            }
            library.subcircuits.insert(key, interface);
        }
        Ok(())
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

fn active_model_section_names(library: &ModelLibrary) -> Result<Vec<String>, String> {
    if matches!(
        library.source_authority,
        ModelSourceAuthority::ProjectOwned { .. }
    ) && !library.model_definition_metadata.is_empty()
    {
        // Project-authored revisions retain one canonical top-level base card.
        // Their named-section overrides are typed authoring metadata and are
        // materialized for execution separately; replacing the base catalog
        // with an override card would break its source/evidence identity.
        return Ok(Vec::new());
    }
    let Some(selected_corner) = library.selected_corner.as_deref() else {
        return Ok(Vec::new());
    };
    let corner = library.corners.get(selected_corner).ok_or_else(|| {
        format!(
            "Model library '{}' selected corner '{}' does not exist",
            library.name, selected_corner
        )
    })?;
    let mut sections = BTreeMap::<String, String>::new();
    for binding in corner.effective_section_bindings() {
        sections
            .entry(binding.section.to_ascii_lowercase())
            .or_insert(binding.section);
    }
    if sections.is_empty() {
        return Err(format!(
            "Model library '{}' selected corner '{}' has no executable section bindings",
            library.name, selected_corner
        ));
    }
    Ok(sections.into_values().collect())
}

fn pdk_config_library_name(manager: &ModelLibraryManager, source: &Path) -> String {
    let base = source
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.trim().is_empty())
        .unwrap_or("pdk-models")
        .to_owned();
    let occupied = |candidate: &str| {
        manager
            .libraries
            .keys()
            .any(|name| name.eq_ignore_ascii_case(candidate))
    };
    if !occupied(&base) {
        return base;
    }

    let digest = format!("{:x}", Sha256::digest(portable_path_key(source).as_bytes()));
    for width in [12usize, 16, 24, 32, 64] {
        let candidate = format!("{base}@{}", &digest[..width.min(digest.len())]);
        if !occupied(&candidate) {
            return candidate;
        }
    }

    // SHA-256 collisions are not assumed impossible. Preserve a deterministic
    // and bounded fallback rather than overwriting an existing provider.
    for suffix in 2u32.. {
        let candidate = format!("{base}@{digest}-{suffix}");
        if !occupied(&candidate) {
            return candidate;
        }
    }
    unreachable!("the finite library catalog cannot occupy every u32 suffix")
}

fn pack_model_hit(
    index: &SpiceLibraryIndex,
    entry: rspice_core::library::CatalogEntry,
) -> PackModelHit {
    let pack = index.pack(&entry.pack);
    PackModelHit {
        name: entry.name.clone(),
        kind: entry.kind.clone(),
        device: entry.device.clone(),
        pack_name: pack.map_or_else(|| entry.pack.clone(), |pack| pack.name.clone()),
        redistributable: pack.is_some_and(|pack| pack.redistributable) && !entry.restricted,
        source: entry.source_path(index),
        line: entry.line,
        pack: entry.pack,
    }
}

fn pack_library_name(pack: &str, source_stem: &str) -> String {
    let canonical = |value: &str| {
        let mut result = String::with_capacity(value.len());
        let mut previous_separator = false;
        for character in value.chars() {
            if character.is_ascii_alphanumeric() {
                result.push(character.to_ascii_lowercase());
                previous_separator = false;
            } else if !previous_separator {
                result.push('-');
                previous_separator = true;
            }
        }
        result.trim_matches('-').to_owned()
    };
    let pack = canonical(pack);
    let source = canonical(source_stem);
    format!("pack-{pack}-{source}")
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
