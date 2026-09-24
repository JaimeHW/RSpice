//! The model library.
//!
//! Owns every model a project can resolve, from every source, and seals the
//! exact set a run executed against. Sealing is by content digest, so a
//! library that changes underneath a completed run is detectable rather
//! than silently assumed identical.

mod catalog_identity;
mod project_models;
mod sealing;
mod source_bundle;

pub(crate) use catalog_identity::model_library_source_digest;

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rspice_core::library::SpiceLibraryIndex;
#[cfg(test)]
use rspice_model_library::ModelValidationFindingSeverity;
use rspice_model_library::{
    MODEL_RESOLUTION_RECORD_SCHEMA_VERSION, ModelConsumerScope, ModelResolutionRecord,
    ModelValidationFinding, ModelValidationReceipt, ModelValidationReceiptInput,
    SimulationPlanModelBinding,
};

#[cfg(not(target_arch = "wasm32"))]
use super::is_foreign_platform_absolute_path;
use super::{
    DeviceModel, ModelCorrelationState, ModelLevel, ModelLibrary, ModelQualificationState,
    ModelSectionQualification, ModelSourceAuthority, ModelSourceContent, ModelSourceEdge,
    ModelSourceEvidenceBinding, ModelSourcePin, ModelType, ProcessCorner, ProjectModelDefinition,
    ProjectModelRevisionDefinition, first_unreachable_source,
};
use crate::product::{ContentDigest, ModelSourceId, ObjectRevision};
use rspice_app_types::product::ProcessCorner as CornerProcess;
use rspice_model_library::{
    CornerModelBinding, MaterializedPlanBinding, ModelExecutionPlan,
    resolve_materialized_definition_namespace,
};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ModelDefinitionProvider {
    pub library: String,
    pub definition: String,
    pub source_digest: ContentDigest,
}

const fn model_validation_platform() -> &'static str {
    if cfg!(target_arch = "wasm32") {
        "browser-wasm32"
    } else if cfg!(target_os = "windows") {
        "desktop-windows"
    } else if cfg!(target_os = "macos") {
        "desktop-macos"
    } else if cfg!(target_os = "linux") {
        "desktop-linux"
    } else {
        "desktop-unsupported"
    }
}

/// One immutable, authenticated model-source snapshot for a simulation run.
/// The exact bytes are intentionally transient and are never serialized into
/// project/session state.
#[derive(Debug, Clone)]
pub struct SealedModelExecutionSources {
    bundle: rspice_core::netlist::SealedSourceBundle,
    sources: Vec<(PathBuf, String)>,
    edges: Vec<rspice_core::netlist::SealedSourceEdge>,
    model_library_source_paths: Vec<PathBuf>,
    libraries: Vec<SealedExecutionLibrary>,
    pdk_process_bindings: Vec<crate::state::pdk_config::SealedPdkModelProcessBinding>,
    pdk_veriloga_artifacts: Vec<crate::state::pdk_config::SealedPdkVerilogAArtifact>,
    pdk_veriloga_bindings: Vec<crate::state::pdk_config::SealedPdkVerilogABinding>,
    pdk_identity: Option<(
        crate::state::pdk_config::PdkTechnologyBinding,
        ContentDigest,
    )>,
    resolution_records: Vec<ModelResolutionRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SealedModelLibraryVerilogARoot {
    pub(crate) path: PathBuf,
    pub(crate) netlist_alias: Option<String>,
    pub(crate) selected_module: Option<String>,
}

/// Exact model-library bytes and AHDL roots authenticated by one run seal.
/// Signed-PDK artifacts are intentionally excluded; they have their own
/// manifest-governed authority and compiler path.
#[derive(Debug, Clone)]
pub(crate) struct SealedModelLibraryVerilogAAuthority {
    pub(crate) closure_digest: ContentDigest,
    pub(crate) sources: Vec<(PathBuf, String)>,
    pub(crate) roots: Vec<SealedModelLibraryVerilogARoot>,
}

#[derive(Debug, Clone)]
struct SealedExecutionLibrary {
    name: String,
    /// What every model block sealed from this library is labelled as. Settled
    /// at sealing time from the library's own provenance, because the sealed
    /// bundle is all a materializer has and a bundle path names bytes.
    provenance: String,
    root_path: PathBuf,
    source_digest: ContentDigest,
    corners: Vec<ProcessCorner>,
    selected_corner: Option<String>,
    allows_selected_section_override: bool,
}

/// One corner section materialized out of the sealed bundle, with the identity
/// needed to label it and the domains it covers.
struct MaterializedCornerSection {
    source_label: String,
    section: String,
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

fn hash_validation_source_field(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_le_bytes());
    hasher.update(value);
}

impl SealedModelExecutionSources {
    /// Build a source bundle that adds one active root buffer to the exact
    /// authenticated model-library closure.
    ///
    /// Root include edges are accepted only when their portable lexical target
    /// names one retained source exactly. This deliberately does not guess by
    /// basename or consult a host search path: unresolved and ambiguous deck
    /// references fail closed in browser execution.
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

    /// Freeze the exact model namespace for the nominal/reference run.
    ///
    /// An explicit per-library selection is authoritative for that library.
    /// The reference process supplies the conventional fallback and the signed
    /// PDK process contract. Process sweeps deliberately use
    /// [`Self::corner_model_bindings`] instead, because every sweep point owns
    /// its explicit process independently of the nominal selection.
    pub fn reference_model_execution_plan(
        &self,
        process: crate::product::ProcessCorner,
    ) -> Result<ModelExecutionPlan, String> {
        let materialized = self.bindings_for_processes(&[process], true)?;
        let selected_library_corners = self
            .libraries
            .iter()
            .map(|library| {
                let selected = library.selected_corner.clone().or_else(|| {
                    library
                        .corners
                        .iter()
                        .find(|corner| corner.name.eq_ignore_ascii_case(process.short_name()))
                        .map(|corner| corner.name.clone())
                });
                (library.name.clone(), selected)
            })
            .collect::<Vec<_>>();

        ModelExecutionPlan::try_new(
            process,
            selected_library_corners,
            materialized,
            &self.resolution_records,
        )
    }

    /// Materialize the exact model cards for the nominal/reference process.
    pub fn reference_process_model_cards(
        &self,
        process: crate::product::ProcessCorner,
    ) -> Result<Vec<String>, String> {
        self.reference_model_execution_plan(process)
            .map(|plan| plan.model_cards())
    }

    /// Materialize every model section required by a process-corner run from
    /// this same immutable snapshot.
    pub fn corner_model_bindings(
        &self,
        processes: &[CornerProcess],
    ) -> Result<Vec<CornerModelBinding>, String> {
        let mut resolved = Vec::new();
        for process in processes {
            let materialized = self.bindings_for_processes(&[*process], false)?;
            let (bindings, _) =
                resolve_materialized_definition_namespace(materialized, &self.resolution_records)?;
            resolved.extend(bindings);
        }
        Ok(resolved)
    }

    fn bindings_for_processes(
        &self,
        processes: &[CornerProcess],
        honor_nominal_selection: bool,
    ) -> Result<Vec<MaterializedPlanBinding>, String> {
        if self.libraries.is_empty() && self.pdk_process_bindings.is_empty() {
            if let Some(process) = processes
                .iter()
                .find(|process| **process != CornerProcess::TT)
            {
                return Err(format!(
                    "{} requires a PDK model library with an explicit process section",
                    process.short_name()
                ));
            }
            return Ok(Vec::new());
        }

        let mut bindings = Vec::new();
        for process in processes {
            for library in &self.libraries {
                let keyword = process.short_name();
                let requested_corner = honor_nominal_selection
                    .then_some(library.selected_corner.as_deref())
                    .flatten()
                    .unwrap_or(keyword);
                let corner = library
                    .corners
                    .iter()
                    .find(|corner| corner.name.eq_ignore_ascii_case(requested_corner))
                    .cloned();
                if corner.is_none()
                    && (library.selected_corner.is_some()
                        || *process != CornerProcess::TT
                        || !library.corners.is_empty())
                {
                    return Err(format!(
                        "Model library '{}' does not define selected corner '{}' for the {} reference process",
                        library.name, requested_corner, keyword
                    ));
                }
                match corner.as_ref() {
                    Some(corner) => {
                        for section in self.materialize_library_corner(library, corner)? {
                            let binding = MaterializedPlanBinding::try_new(
                                CornerModelBinding {
                                    process: *process,
                                    source_label: section.source_label,
                                    section: Some(section.section),
                                    materialized_model_cards: section.materialized_model_cards,
                                },
                                library.name.clone(),
                                library.source_digest,
                                library.allows_selected_section_override,
                            )?;
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
                        let binding = MaterializedPlanBinding::try_new(
                            CornerModelBinding {
                                process: *process,
                                source_label: library.provenance.clone(),
                                section: None,
                                materialized_model_cards,
                            },
                            library.name.clone(),
                            library.source_digest,
                            false,
                        )?;
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
                        process.short_name()
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
                                process.short_name(),
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
                    let binding = MaterializedPlanBinding::try_new(
                        CornerModelBinding {
                            process: *process,
                            source_label: package,
                            section: source.section.clone(),
                            materialized_model_cards,
                        },
                        format!("signed-pdk:{}", source.source_id),
                        source.artifact_digest,
                        false,
                    )?;
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
                    library.provenance,
                    section,
                    domains
                        .iter()
                        .map(|domain| domain.label())
                        .collect::<Vec<_>>()
                        .join(" + ")
                ),
                section,
                materialized_model_cards,
            });
        }
        Ok(sections)
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
    /// Durable project decisions for contested executable definitions. The
    /// map key is the canonical `scope:name` identity repeated by each value.
    #[serde(default)]
    resolution_records: BTreeMap<String, ModelResolutionRecord>,
    #[serde(default)]
    validation_receipt: Option<ModelValidationReceipt>,
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
    /// The individual source file is excluded from redistribution even when
    /// other files in the same pack are shippable.
    pub restricted: bool,
}

impl ModelLibraryManager {
    #[must_use]
    pub fn model_validation_receipt(&self) -> Option<&ModelValidationReceipt> {
        self.validation_receipt.as_ref()
    }

    pub(crate) fn invalidate_model_validation_receipt(&mut self) {
        self.validation_receipt = None;
    }

    pub(crate) fn restore_model_validation_receipt(
        &mut self,
        receipt: Option<ModelValidationReceipt>,
    ) -> Result<(), String> {
        if let Some(receipt) = receipt.as_ref() {
            receipt.verify()?;
        }
        self.validation_receipt = receipt;
        Ok(())
    }

    pub(crate) fn issue_model_validation_receipt(
        &mut self,
        project_revision: ObjectRevision,
        plan_digest: ContentDigest,
        pdk_archive_digest: Option<ContentDigest>,
        execution_schema_version: u32,
        findings: Vec<ModelValidationFinding>,
    ) -> Result<ModelValidationReceipt, String> {
        let (source_count, source_closure_digest) = self.model_validation_source_identity();
        let receipt = ModelValidationReceipt::issue(ModelValidationReceiptInput {
            project_revision,
            model_execution_plan_digest: plan_digest,
            execution_catalog_digest: self.execution_catalog_digest(),
            source_count,
            source_closure_digest,
            pdk_archive_digest,
            engine_version: env!("CARGO_PKG_VERSION").to_owned(),
            execution_schema_version,
            platform: model_validation_platform().to_owned(),
            findings,
            validated_at_unix_ms: crate::time_compat::checked_unix_time_ms().map_err(|error| {
                format!("system clock cannot timestamp model validation: {error}")
            })?,
        })?;
        self.validation_receipt = Some(receipt.clone());
        Ok(receipt)
    }

    pub(crate) fn validate_model_validation_receipt(
        &self,
        project_revision: ObjectRevision,
        plan_digest: ContentDigest,
        pdk_archive_digest: Option<ContentDigest>,
        execution_schema_version: u32,
    ) -> Result<&ModelValidationReceipt, String> {
        let receipt = self.validation_receipt.as_ref().ok_or_else(|| {
            "No durable model-validation receipt exists for this project revision.".to_owned()
        })?;
        receipt.verify()?;
        if receipt.project_revision != project_revision {
            return Err(
                "Model-validation receipt is stale after a project revision change.".to_owned(),
            );
        }
        if receipt.model_execution_plan_digest != plan_digest {
            return Err(
                "Model-validation receipt is stale after the execution plan changed.".to_owned(),
            );
        }
        if receipt.execution_catalog_digest != self.execution_catalog_digest() {
            return Err(
                "Model-validation receipt is stale after the model catalog changed.".to_owned(),
            );
        }
        if receipt.pdk_archive_digest != pdk_archive_digest {
            return Err(
                "Model-validation receipt is stale after the signed PDK changed.".to_owned(),
            );
        }
        if receipt.execution_schema_version != execution_schema_version
            || receipt.engine_version != env!("CARGO_PKG_VERSION")
            || receipt.platform != model_validation_platform()
        {
            return Err(
                "Model-validation receipt was produced by a different engine, schema, or platform."
                    .to_owned(),
            );
        }
        let (source_count, source_closure_digest) = self.model_validation_source_identity();
        if receipt.source_count != source_count
            || receipt.source_closure_digest != source_closure_digest
        {
            return Err(
                "Model-validation receipt is stale after source digests changed.".to_owned(),
            );
        }
        Ok(receipt)
    }

    fn model_validation_source_identity(&self) -> (u64, ContentDigest) {
        let mut identities = self
            .libraries_sorted()
            .into_iter()
            .flat_map(|library| {
                library
                    .source_closure
                    .iter()
                    .map(move |source| (library.name.clone(), source.digest.to_string()))
            })
            .collect::<Vec<_>>();
        identities.sort();
        let source_count = identities.len() as u64;
        let mut hasher = Sha256::new();
        hasher.update(b"rspice.model-validation-source-closure/v1\0");
        for (library, digest) in identities {
            hash_validation_source_field(&mut hasher, library.as_bytes());
            hash_validation_source_field(&mut hasher, digest.as_bytes());
        }
        (
            source_count,
            ContentDigest::from_bytes(hasher.finalize().into()),
        )
    }

    #[must_use]
    pub fn model_resolution_record(
        &self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> Option<&ModelResolutionRecord> {
        let normalized = definition.trim().to_ascii_lowercase();
        self.resolution_records.get(&scope.record_key(&normalized))
    }

    pub(crate) fn restore_model_resolution_records(
        &mut self,
        records: Vec<ModelResolutionRecord>,
    ) -> Result<(), String> {
        let mut restored = BTreeMap::new();
        for record in records {
            record.validate()?;
            let key = record.key();
            if restored.insert(key.clone(), record).is_some() {
                return Err(format!("model-resolution record '{key}' is repeated"));
            }
        }
        self.resolution_records = restored;
        self.validate_model_resolution_records_against_catalog()
    }

    #[must_use]
    pub(crate) fn owned_model_resolution_records(&self) -> Vec<ModelResolutionRecord> {
        self.resolution_records.values().cloned().collect()
    }

    pub(crate) fn definition_providers(
        &self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> Vec<ModelDefinitionProvider> {
        let normalized = definition.trim().to_ascii_lowercase();
        let mut providers = Vec::new();
        for library in self.libraries_sorted() {
            let active_sections = library.active_section_names();
            let names = match scope {
                ModelConsumerScope::PrimitiveModel => library
                    .models
                    .values()
                    .map(|model| model.name.as_str())
                    .collect::<Vec<_>>(),
                ModelConsumerScope::Subcircuit => library
                    .subcircuits
                    .values()
                    .filter(|subcircuit| {
                        subcircuit.section.as_deref().is_none_or(|section| {
                            active_sections
                                .iter()
                                .any(|active| active.eq_ignore_ascii_case(section))
                        })
                    })
                    .map(|subcircuit| subcircuit.name.as_str())
                    .collect::<Vec<_>>(),
            };
            for exact_name in names
                .into_iter()
                .filter(|name| name.to_ascii_lowercase() == normalized)
            {
                providers.push(ModelDefinitionProvider {
                    library: library.name.clone(),
                    definition: exact_name.to_owned(),
                    source_digest: model_library_source_digest(library),
                });
            }
        }
        providers.sort_by(|left, right| {
            left.library
                .cmp(&right.library)
                .then_with(|| left.definition.cmp(&right.definition))
                .then_with(|| {
                    left.source_digest
                        .to_string()
                        .cmp(&right.source_digest.to_string())
                })
        });
        providers
    }

    /// Resolve the one provider the flat executable SPICE namespace will use.
    ///
    /// Component properties may retain a library name for provenance, but that
    /// metadata is not an independent namespace selector. Every UI surface must
    /// consult this method so Properties, catalog binding, and the sealed run
    /// plan agree with the same project-global provider decision.
    pub(crate) fn effective_definition_provider(
        &self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> Result<Option<ModelDefinitionProvider>, String> {
        let providers = self.definition_providers(scope, definition);
        match providers.as_slice() {
            [] => Ok(None),
            [provider] => Ok(Some(provider.clone())),
            _ => {
                let normalized_name = definition.trim().to_ascii_lowercase();
                let Some(record) = self.model_resolution_record(scope, &normalized_name) else {
                    return Err(format!(
                        "{} '{}' has {} executable providers ({}); resolve the project-global provider before binding or editing an instance",
                        scope.label(),
                        definition.trim(),
                        providers.len(),
                        providers
                            .iter()
                            .map(|provider| provider.library.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                };
                let winners = providers
                    .into_iter()
                    .filter(|provider| {
                        provider.library == record.provider_library
                            && provider.definition == record.provider_definition
                            && provider.source_digest == record.provider_source_digest
                    })
                    .collect::<Vec<_>>();
                match winners.as_slice() {
                    [provider] => Ok(Some(provider.clone())),
                    [] => Err(format!(
                        "project-global provider decision for {} '{}' no longer matches an authenticated catalog definition",
                        scope.label(),
                        definition.trim()
                    )),
                    _ => Err(format!(
                        "{} '{}' is repeated inside resolved provider '{}'; repair that source before binding an instance",
                        scope.label(),
                        definition.trim(),
                        record.provider_library
                    )),
                }
            }
        }
    }

    pub fn resolve_definition_provider(
        &mut self,
        scope: ModelConsumerScope,
        definition: &str,
        provider_library: &str,
        audit_reason: &str,
    ) -> Result<ModelResolutionRecord, String> {
        let normalized_name = definition.trim().to_ascii_lowercase();
        let providers = self.definition_providers(scope, &normalized_name);
        if providers.len() < 2 {
            return Err(format!(
                "{} definition '{}' is not contested by multiple authenticated providers",
                scope.label(),
                definition.trim()
            ));
        }
        let provider = providers
            .iter()
            .find(|provider| provider.library == provider_library)
            .ok_or_else(|| {
                format!(
                    "'{provider_library}' is not an exact provider of contested {} '{}'",
                    scope.label(),
                    normalized_name
                )
            })?;
        if providers.iter().any(|candidate| {
            candidate != provider
                && candidate.library.eq_ignore_ascii_case(&provider.library)
                && candidate.source_digest == provider.source_digest
        }) {
            return Err(format!(
                "{} '{}' is defined more than once inside provider '{}'; repair the source because a provider decision cannot distinguish same-source duplicates",
                scope.label(),
                normalized_name,
                provider.library
            ));
        }
        let created_at_unix_ms = crate::time_compat::checked_unix_time_ms()
            .map_err(|error| format!("system clock cannot timestamp provider decision: {error}"))?;
        let record = ModelResolutionRecord {
            schema_version: MODEL_RESOLUTION_RECORD_SCHEMA_VERSION,
            consumer_scope: scope,
            normalized_name,
            provider_library: provider.library.clone(),
            provider_definition: provider.definition.clone(),
            provider_source_digest: provider.source_digest,
            audit_reason: audit_reason.to_owned(),
            created_at_unix_ms,
        };
        record.validate()?;
        self.resolution_records.insert(record.key(), record.clone());
        Ok(record)
    }

    pub fn clear_definition_provider(
        &mut self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> bool {
        let normalized = definition.trim().to_ascii_lowercase();
        self.resolution_records
            .remove(&scope.record_key(&normalized))
            .is_some()
    }

    fn validate_model_resolution_records_against_catalog(&self) -> Result<(), String> {
        for (key, record) in &self.resolution_records {
            record.validate()?;
            if record.key() != *key {
                return Err(format!(
                    "model-resolution map key '{key}' does not match its record identity '{}'",
                    record.key()
                ));
            }
            let provider = self.get_library(&record.provider_library).ok_or_else(|| {
                format!(
                    "provider decision for {} '{}' is stale because library '{}' was removed",
                    record.consumer_scope.label(),
                    record.normalized_name,
                    record.provider_library
                )
            })?;
            if model_library_source_digest(provider) != record.provider_source_digest {
                return Err(format!(
                    "provider decision for {} '{}' is stale because source '{}' changed digest",
                    record.consumer_scope.label(),
                    record.normalized_name,
                    record.provider_library
                ));
            }
            let exact_provider_exists = self
                .definition_providers(record.consumer_scope, &record.normalized_name)
                .into_iter()
                .any(|candidate| {
                    candidate.library == record.provider_library
                        && candidate.definition == record.provider_definition
                        && candidate.source_digest == record.provider_source_digest
                });
            if !exact_provider_exists {
                return Err(format!(
                    "provider decision for {} '{}' is stale because exact definition '{}/{}' is no longer available",
                    record.consumer_scope.label(),
                    record.normalized_name,
                    record.provider_library,
                    record.provider_definition
                ));
            }
        }
        Ok(())
    }

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

    /// Select a library, refusing a name this project no longer holds.
    ///
    /// It used to be a silent no-op, which is the worst of the three possible
    /// behaviours: every Models surface renders from the selection, so a route
    /// that named a library that had since gone left the *previous* one showing
    /// and read as a route that worked. Refusing by name lets the caller say so.
    pub fn select_library(&mut self, name: &str) -> Result<(), String> {
        if !self.libraries.contains_key(name) {
            return Err(format!(
                "Model library '{name}' is not loaded in this project, so the selection was not \
                 changed."
            ));
        }
        self.selected_library = Some(name.to_string());
        Ok(())
    }

    /// Libraries that declare `process`, in sorted order.
    ///
    /// The keyword is the one [`Self::reference_model_execution_plan`] looks a
    /// process section up by, so a route offered here lands on a library that
    /// plan would actually read. Answers "which library is this binding failure
    /// about" when the failure sentence itself names only the process.
    #[must_use]
    pub fn libraries_declaring_process(&self, process: crate::product::ProcessCorner) -> Vec<&str> {
        let keyword = process.short_name();
        self.libraries_sorted()
            .into_iter()
            .filter(|library| library.declares_process(keyword))
            .map(|library| library.name.as_str())
            .collect()
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
    /// Absence is normal, not a failure: the foundation library is compiled into
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
    /// The addressable count rather than the raw definition total, so private
    /// helper cards inside macromodel bodies are never offered as parts.
    pub fn pack_definition_count(&self) -> usize {
        self.spice_packs
            .as_ref()
            .map_or(0, |index| index.part_count())
    }

    /// Search the shipped packs for definitions whose name contains `query`.
    ///
    /// Bounded by `limit` because the same API can inspect an explicitly opened
    /// developer corpus. An empty query returns nothing rather than everything.
    #[cfg(test)]
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
                    restricted: entry.restricted,
                }
            })
            .collect()
    }

    /// Browse a bounded first page or a canonical device-class projection of
    /// the shipped corpus without loading every catalog row into memory.
    pub fn browse_pack_models(
        &self,
        query: &str,
        pack_filter: Option<&str>,
        device_filters: &[&str],
        offset: usize,
        limit: usize,
    ) -> Result<(usize, Vec<PackModelHit>), String> {
        let Some(index) = self.spice_packs.as_ref() else {
            return Ok((0, Vec::new()));
        };
        let (total, entries) = index
            .query_parts(query, pack_filter, device_filters, offset, limit)
            .map_err(|error| format!("Shipped model catalog could not be read: {error}"))?;

        let hits = entries
            .into_iter()
            .map(|entry| {
                let pack = index.pack(&entry.pack);
                PackModelHit {
                    name: entry.name.clone(),
                    kind: entry.kind.clone(),
                    device: entry.device.clone(),
                    pack_name: pack.map_or_else(|| entry.pack.clone(), |p| p.name.clone()),
                    redistributable: pack.is_some_and(|pack| pack.redistributable),
                    source: entry.source_path(index),
                    line: entry.line,
                    pack: entry.pack,
                    restricted: entry.restricted,
                }
            })
            .collect();
        Ok((total, hits))
    }

    /// Whether a pack's executable entry bytes are available to attach now.
    ///
    /// Browser builds embed discovery metadata only. They must not enable an
    /// attach action whose synthetic catalog path can never be opened.
    #[must_use]
    pub fn spice_pack_entry_available(&self, pack_id: &str) -> bool {
        self.spice_packs.as_ref().is_some_and(|index| {
            index.source_files_available()
                && index
                    .pack(pack_id)
                    .and_then(|pack| pack.entry_path(index.root()))
                    .is_some_and(|entry| entry.is_file())
        })
    }

    /// Load a redistributable pack's declared entry as one authenticated model
    /// library. The caller publishes the resulting manager candidate at the
    /// project transaction boundary.
    pub fn attach_spice_pack(&mut self, pack_id: &str) -> Result<String, String> {
        let index = self.spice_packs.as_ref().ok_or_else(|| {
            "The shipped model corpus is not installed on this machine.".to_owned()
        })?;
        let pack = index
            .pack(pack_id)
            .ok_or_else(|| format!("Model pack '{pack_id}' is no longer installed."))?;
        if !pack.redistributable {
            return Err(format!(
                "Model pack '{}' cannot be embedded in a project because its redistribution grant is not established.",
                pack.name
            ));
        }
        let entry = pack.entry_path(index.root()).ok_or_else(|| {
            format!(
                "Model pack '{}' has no declared entry file to attach.",
                pack.name
            )
        })?;
        let library_name = self.load_catalog_source_without_collision(&entry)?;
        self.retain_pack_library(&library_name, pack_id)?;
        Ok(library_name)
    }

    /// Retain one part's source from an installed Model Hub pack, recording
    /// which signed release the bytes came from.
    ///
    /// `files` is the release's expanded file set and `part_source` names the
    /// member that defines the part; everything unreachable from it is dropped
    /// rather than retained. The bytes are the argument, not a path, because a
    /// pack's sources are proved as bytes on every platform and only *happen*
    /// to be expanded on a disk when the store is a filesystem. Routing both
    /// stores through the one import is what makes a browser session's project
    /// and a desktop session's project the same document.
    ///
    /// What lands is the same retention an uploaded source goes through — the
    /// same closure, the same digest, the same read-only retained authority —
    /// with one fact added: the pin naming the release. The pin is evidence,
    /// never a resolution input; nothing about executing the library consults
    /// it, so a project whose pinned release has been uninstalled behaves
    /// exactly as it did the day it was saved.
    pub fn add_pack_release_part(
        &mut self,
        files: Vec<(String, Vec<u8>)>,
        part_source: &str,
        pin: super::PackPartPin,
    ) -> Result<String, String> {
        let (library_name, mut library) =
            source_bundle::build(part_source, Some(part_source), files, None)?;
        if let Some(existing) = self.libraries.get(&library_name) {
            // Different bytes under a name this project already uses is a
            // collision — unless the name is held by this same part of this
            // same pack, in which case it is the one thing it can honestly be:
            // another release of it. Adopting a release re-runs this path under
            // the newer version, and refusing there would leave detaching the
            // library — and losing the pin that says where it came from — as
            // the only way to move a pinned part forward.
            let same_pack_part = existing
                .pack_pin
                .as_ref()
                .is_some_and(|held| held.pack_id == pin.pack_id && held.part_id == pin.part_id);
            if existing.root_path != library.root_path && !same_pack_part {
                return Err(format!(
                    "Library name '{library_name}' is already owned by another model source; \
                     rename or detach it before adding '{}'.",
                    pin.part_id
                ));
            }
            // Identical bytes under an identity this project already minted:
            // keeping it means a second part from the same source file does
            // not renumber the source every surface already refers to.
            if let (
                ModelSourceAuthority::RetainedImport { source_id, .. },
                ModelSourceAuthority::RetainedImport {
                    source_id: replacement,
                    ..
                },
            ) = (existing.source_authority, &mut library.source_authority)
            {
                *replacement = source_id;
            }
        }
        let pack_id = pin.pack_id.clone();
        self.libraries.insert(library_name.clone(), library);
        self.retain_pack_library_pinned(&library_name, &pack_id, Some(pin))?;
        Ok(library_name)
    }

    fn retain_pack_library(&mut self, library_name: &str, pack_id: &str) -> Result<(), String> {
        self.retain_pack_library_pinned(library_name, pack_id, None)
    }

    fn retain_pack_library_pinned(
        &mut self,
        library_name: &str,
        pack_id: &str,
        pin: Option<super::PackPartPin>,
    ) -> Result<(), String> {
        let library = self.libraries.get_mut(library_name).ok_or_else(|| {
            format!("Attached pack library '{library_name}' disappeared before publication")
        })?;
        let root = library.root_path.as_ref().ok_or_else(|| {
            format!("Attached pack library '{library_name}' has no root identity")
        })?;
        let root_digest = library
            .source_closure
            .iter()
            .find(|pin| pin.path == *root)
            .map(|pin| pin.digest)
            .ok_or_else(|| {
                format!(
                    "Attached pack library '{library_name}' did not retain its root source bytes"
                )
            })?;
        let source_id = match library.source_authority {
            ModelSourceAuthority::RetainedImport { source_id, .. } => source_id,
            _ => ModelSourceId::new(),
        };
        library.source_authority = ModelSourceAuthority::RetainedImport {
            source_id,
            digest: root_digest,
        };
        library.pack_id = Some(pack_id.to_owned());
        if pin.is_some() {
            library.pack_pin = pin;
        }
        Ok(())
    }

    /// Explicitly refresh a shipped-pack snapshot from the currently installed
    /// corpus, then immediately return it to retained project authority.
    pub fn refresh_spice_pack(&mut self, pack_id: &str) -> Result<String, String> {
        let entry = {
            let index = self.spice_packs.as_ref().ok_or_else(|| {
                "The shipped model corpus is not installed on this machine.".to_owned()
            })?;
            let pack = index
                .pack(pack_id)
                .ok_or_else(|| format!("Model pack '{pack_id}' is no longer installed."))?;
            if !pack.redistributable {
                return Err(format!(
                    "Model pack '{}' can no longer be refreshed because its redistribution grant is not established.",
                    pack.name
                ));
            }
            pack.entry_path(index.root()).ok_or_else(|| {
                format!(
                    "Model pack '{}' has no declared entry file to refresh.",
                    pack.name
                )
            })?
        };
        let selected_corner = self
            .libraries
            .values()
            .find(|library| library.pack_id.as_deref() == Some(pack_id))
            .and_then(|library| library.selected_corner.clone());
        let library_name = self.load_library_file(&entry, selected_corner.as_deref())?;
        self.retain_pack_library(&library_name, pack_id)?;
        Ok(library_name)
    }

    /// Load the exact shipped source containing an addressable part. Restricted
    /// files fail closed instead of being silently copied into project data.
    pub fn add_spice_part(&mut self, pack_id: &str, part_name: &str) -> Result<String, String> {
        let index = self.spice_packs.as_ref().ok_or_else(|| {
            "The shipped model corpus is not installed on this machine.".to_owned()
        })?;
        let pack = index
            .pack(pack_id)
            .ok_or_else(|| format!("Model pack '{pack_id}' is no longer installed."))?;
        if !pack.redistributable {
            return Err(format!(
                "Part '{part_name}' cannot be copied from '{}' because its redistribution grant is not established.",
                pack.name
            ));
        }
        let matches = index
            .find_part(part_name)
            .map_err(|error| format!("Part '{part_name}' could not be resolved: {error}"))?;
        let entry = matches
            .into_iter()
            .find(|entry| entry.pack == pack_id)
            .ok_or_else(|| {
                format!(
                    "Part '{part_name}' is no longer present in pack '{}'.",
                    pack.name
                )
            })?;
        if entry.restricted {
            return Err(format!(
                "Part '{part_name}' is in a source file that is not licensed for project embedding."
            ));
        }
        let source = entry
            .source_path(index)
            .ok_or_else(|| format!("Part '{part_name}' has no installed source file."))?;
        let library_name = self.load_catalog_source_without_collision(&source)?;
        self.retain_pack_library(&library_name, pack_id)?;
        Ok(library_name)
    }

    fn load_catalog_source_without_collision(&mut self, path: &Path) -> Result<String, String> {
        let canonical = std::fs::canonicalize(path).map_err(|error| {
            format!(
                "Failed to resolve model source '{}': {error}",
                path.display()
            )
        })?;
        let library_name = canonical
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or_else(|| {
                format!(
                    "Model source '{}' has no valid file name.",
                    canonical.display()
                )
            })?
            .to_owned();
        if let Some(existing) = self.get_library(&library_name) {
            if existing.root_path.as_deref() == Some(canonical.as_path()) {
                return Ok(library_name);
            }
            return Err(format!(
                "Library name '{library_name}' is already owned by another source; rename or detach it before adding '{}'.",
                canonical.display()
            ));
        }
        self.load_library_file(&canonical, None)
    }

    /// Get libraries sorted by name
    pub fn libraries_sorted(&self) -> Vec<&ModelLibrary> {
        let mut libs: Vec<_> = self.libraries.values().collect();
        libs.sort_by(|a, b| a.name.cmp(&b.name));
        libs
    }

    /// Stable owned snapshot used by guarded multi-library project
    /// transactions. Presentation filters and the shipped-pack index remain
    /// manager state and are intentionally excluded.
    pub(crate) fn library_snapshot(&self) -> Vec<ModelLibrary> {
        self.libraries_sorted().into_iter().cloned().collect()
    }

    /// Replace the complete loaded-library set while preserving presentation
    /// state owned by this manager.
    pub(crate) fn replace_library_snapshot(
        &mut self,
        libraries: Vec<ModelLibrary>,
    ) -> Result<(), String> {
        let expanded = self
            .libraries
            .iter()
            .map(|(name, library)| (name.clone(), library.expanded))
            .collect::<HashMap<_, _>>();
        let mut replacement = HashMap::with_capacity(libraries.len());
        for mut library in libraries {
            if replacement.contains_key(&library.name) {
                return Err(format!(
                    "Model-library snapshot repeats library '{}'",
                    library.name
                ));
            }
            if let Some(retained) = expanded.get(&library.name) {
                library.expanded = *retained;
            }
            replacement.insert(library.name.clone(), library);
        }
        self.libraries = replacement;
        if self
            .selected_library
            .as_ref()
            .is_some_and(|name| !self.libraries.contains_key(name))
        {
            self.selected_library = None;
        }
        Ok(())
    }

    /// Enforce the model-library dialect boundary before any parsed
    /// projection is accepted. `.scs` sources admit the explicit
    /// `simulator lang=spice` interoperability profile and the fail-closed
    /// declarative Spectre model-library subset implemented by the core
    /// adapter. Unsupported native statements are errors, never discarded.
    pub(crate) fn validate_model_source_dialect(path: &Path, source: &str) -> Result<(), String> {
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

    #[cfg(not(target_arch = "wasm32"))]
    fn extend_native_veriloga_closure(
        result: &mut rspice_core::library::LibParseResult,
    ) -> Result<(), String> {
        let mut roots = BTreeSet::<PathBuf>::new();
        for resolved in &result.resolved_sources {
            let projected = rspice_core::library::adapt_spectre_model_library(
                &resolved.path,
                &resolved.content,
            )
            .map_err(|error| {
                format!(
                    "{}:{} cannot authenticate AHDL dependencies: {}",
                    resolved.path.display(),
                    error.line,
                    error.message
                )
            })?;
            for line in projected.lines() {
                let Some(include) = rspice_core::netlist::parse_veriloga_source_directive(line)
                else {
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

        let limits = rspice_veriloga::SourceProviderLimits {
            max_dependencies: crate::state::MAX_PROJECT_SOURCE_FILES.saturating_add(64),
            max_total_source_bytes: crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES.saturating_mul(2),
            max_include_depth: crate::state::MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH,
            max_expanded_bytes: crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES.saturating_mul(2),
        };
        for root in roots {
            let mut preprocessor = rspice_veriloga::Preprocessor::new();
            preprocessor
                .preprocess_file_with_limits(&root, limits)
                .map_err(|error| {
                    format!(
                        "Could not authenticate Verilog-A closure rooted at '{}': {error}",
                        root.display()
                    )
                })?;
            let documents = preprocessor.take_dependency_documents();
            let provider_paths = documents
                .iter()
                .filter(|document| {
                    document.origin == rspice_veriloga::SourceDocumentOrigin::Provider
                })
                .map(|document| document.logical_path.clone())
                .collect::<HashSet<_>>();
            for document in documents.into_iter().filter(|document| {
                document.origin == rspice_veriloga::SourceDocumentOrigin::Provider
            }) {
                if let Some(existing) = result
                    .resolved_sources
                    .iter()
                    .find(|source| source.path == document.logical_path)
                {
                    if existing.content.as_ref() != document.source.as_str() {
                        return Err(format!(
                            "Verilog-A dependency '{}' changed while its closure was captured",
                            document.logical_path.display()
                        ));
                    }
                    continue;
                }
                let bytes: Arc<[u8]> = Arc::from(document.source.as_bytes());
                let content: Arc<str> = Arc::from(document.source);
                result
                    .resolved_sources
                    .push(rspice_core::library::ResolvedLibSource {
                        path: document.logical_path,
                        bytes,
                        content,
                    });
            }
            for include in preprocessor.take_include_graph() {
                if !provider_paths.contains(&include.included_path) {
                    continue;
                }
                let dependency = rspice_core::library::ResolvedLibDependency {
                    owner: include.including_path,
                    requested_path: include.requested_path,
                    target: include.included_path,
                };
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
        if result.resolved_sources.len() > crate::state::MAX_PROJECT_SOURCE_FILES
            || total_bytes > crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES
        {
            return Err(format!(
                "Model source closure including Verilog-A dependencies exceeds the project limit ({} files / {} bytes)",
                crate::state::MAX_PROJECT_SOURCE_FILES,
                crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES
            ));
        }
        Ok(())
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
    #[cfg(test)]
    pub fn clear(&mut self) {
        self.libraries.clear();
        self.resolution_records.clear();
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
        #[cfg(not(target_arch = "wasm32"))]
        let result = {
            let mut result = result;
            Self::extend_native_veriloga_closure(&mut result)?;
            result
        };
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
        let library =
            library.with_parsed_catalog(&result, &path, section, &path.display().to_string())?;

        self.libraries.insert(lib_name.clone(), library);
        Ok(lib_name)
    }

    /// Import one self-contained model source from authenticated bytes.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub fn load_library_bytes(
        &mut self,
        file_name: &str,
        bytes: Vec<u8>,
        section: Option<&str>,
    ) -> Result<String, String> {
        self.load_library_bundle(file_name, vec![(file_name.to_owned(), bytes)], section)
    }

    /// Import a browser-selected source tree with one unambiguous executable
    /// root. Every reachable `.include`, external `.lib`, and Verilog-A edge is
    /// resolved relative to its owner; unreachable uploads are ignored. If a
    /// tree has multiple possible roots, callers must use
    /// [`Self::load_library_bundle_from_root`] to make the choice explicit.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub fn load_library_bundle(
        &mut self,
        display_name: &str,
        files: Vec<(String, Vec<u8>)>,
        section: Option<&str>,
    ) -> Result<String, String> {
        self.load_library_bundle_with_root(display_name, None, files, section)
    }

    /// Import a browser-selected source tree from one explicit executable
    /// entry. Unreachable members are neither decoded nor retained.
    pub fn load_library_bundle_from_root(
        &mut self,
        display_name: &str,
        root_member: &str,
        files: Vec<(String, Vec<u8>)>,
        section: Option<&str>,
    ) -> Result<String, String> {
        self.load_library_bundle_with_root(display_name, Some(root_member), files, section)
    }

    fn load_library_bundle_with_root(
        &mut self,
        display_name: &str,
        root_member: Option<&str>,
        files: Vec<(String, Vec<u8>)>,
        section: Option<&str>,
    ) -> Result<String, String> {
        let (lib_name, library) = source_bundle::build(display_name, root_member, files, section)?;
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
        process: crate::product::ProcessCorner,
    ) -> Result<Vec<String>, String> {
        self.seal_execution_sources()?
            .reference_process_model_cards(process)
    }

    /// Resolve all process-specific sources required by a PVT run.
    #[cfg(test)]
    pub fn corner_model_bindings(
        &self,
        processes: &[CornerProcess],
    ) -> Result<Vec<CornerModelBinding>, String> {
        self.seal_execution_sources()?
            .corner_model_bindings(processes)
    }

    /// Atomically replace the exact external libraries governed by a PDK
    /// configuration.
    ///
    /// Discovery always runs against `next`; cached dialog rows never
    /// authorize application. Every enabled file is canonicalized and parsed
    /// into an isolated manager candidate. Scan, include, parse, or
    /// name-collision failure leaves both this manager and the retained PDK
    /// ownership provenance unchanged.
    pub fn replace_from_pdk_config(
        &mut self,
        previous: Option<&crate::state::pdk_config::PdkConfig>,
        next: &mut crate::state::pdk_config::PdkConfig,
    ) -> Result<usize, Vec<String>> {
        next.discover_model_files();
        if !next.scan_errors.is_empty() {
            return Err(next.scan_errors.clone());
        }

        let mut candidate = self.clone();
        let mut previously_managed = previous
            .into_iter()
            .flat_map(|config| config.managed_model_sources.iter())
            .map(|path| portable_path_key(path))
            .collect::<BTreeSet<_>>();
        if previously_managed.is_empty() {
            previously_managed.extend(
                previous
                    .into_iter()
                    .flat_map(|config| config.discovered_files.iter())
                    .map(|file| portable_path_key(&file.path)),
            );
        }
        let removed = candidate
            .libraries
            .iter()
            .filter(|&(_name, library)| {
                (matches!(library.source_authority, ModelSourceAuthority::External)
                    && library
                        .root_path
                        .as_ref()
                        .is_some_and(|path| previously_managed.contains(&portable_path_key(path))))
            })
            .map(|(name, _library)| name.clone())
            .collect::<Vec<_>>();
        for name in removed {
            candidate.libraries.remove(&name);
        }
        if candidate
            .selected_library
            .as_ref()
            .is_some_and(|name| !candidate.libraries.contains_key(name))
        {
            candidate.selected_library = None;
        }

        let mut loaded = 0;
        let mut errors = Vec::new();
        let mut admitted = Vec::new();
        let mut seen = BTreeSet::new();
        for file in &next.discovered_files {
            if !crate::state::pdk_config::MODEL_FILE_EXTENSIONS.contains(&file.extension.as_str()) {
                continue;
            }
            let canonical = match std::fs::canonicalize(&file.path) {
                Ok(path) => path,
                Err(error) => {
                    errors.push(format!(
                        "{}: failed to resolve discovered model source: {error}",
                        file.path.display()
                    ));
                    continue;
                }
            };
            if !seen.insert(portable_path_key(&canonical)) {
                continue;
            }
            match candidate.load_library_file(&canonical, None) {
                Ok(_) => {
                    admitted.push(canonical);
                    loaded += 1;
                }
                Err(error) => errors.push(format!("{}: {error}", file.path.display())),
            }
        }

        if errors.is_empty() {
            next.managed_model_sources = admitted;
            *self = candidate;
            Ok(loaded)
        } else {
            Err(errors)
        }
    }

    /// Populate the UI catalog from the embedded RSpice foundation library.
    ///
    /// The cards are read through the `.lib` parser rather than the core
    /// library index, because the index reduces every card to one of nine
    /// coarse types and a foundation card's *declared* type is what the rest
    /// of the catalog reasons with: `models_have_compatible_device_family`
    /// tells an n-channel MESFET from a p-channel one by the `NMF`/`PMF`
    /// token, and a partially-depleted SOI card from a plain MOSFET by its
    /// `LEVEL=57`. Reduced to `OTHER` and `Unknown`, an SOI card would be
    /// offered to bulk MOSFETs and withheld from the SOI symbol it exists for.
    ///
    /// The declared token is also what decides the card's [`ModelType`], for
    /// the same reason and through the same vocabulary the whole catalog
    /// reads. Classifying through the core index instead lost every family the
    /// index has no type for — VDMOS and MESFET both — and, having a `Njfet`
    /// it could not spell, discarded JFETs as well.
    pub fn load_builtin_models(&mut self) {
        let core_manager = rspice_core::library::LibraryManager::new();
        let Some(content) = core_manager.get_library_content("foundation.lib") else {
            return;
        };
        let parsed = rspice_core::library::LibParser::new(".").parse_string(content);
        let library = self
            .libraries
            .entry("RSpice Foundation".to_owned())
            .or_insert_with(|| ModelLibrary::new("RSpice Foundation"));
        library.pack_id = Some("rspice-foundation".to_owned());

        for model in &parsed.top_level_models {
            let device_model = DeviceModel {
                name: model.name.clone(),
                // Built-in cards are compiled in at file scope; no `.lib`
                // section owns them.
                section: None,
                model_type: ModelType::from_name(&model.spice_type),
                spice_type: Some(model.spice_type.clone()),
                level: ModelLevel::from_spice_card(model.level, &model.spice_type),
                spice_level: model.level,
                model_version: model.version,
                description: model.description.clone().unwrap_or_default(),
                l_min: model.lmin,
                l_max: model.lmax,
                w_min: model.wmin,
                w_max: model.wmax,
                vdd: None,
                vth0: None,
                // Compiled in: there is no file on disk to reveal, so there is
                // no line in one either.
                file_path: None,
                parameters: model.parameters.clone(),
                string_parameters: model.string_params.clone(),
                source_line: None,
            };
            library.add_model(device_model);
        }
        for subcircuit in core_manager.subcircuits() {
            library.subcircuits.insert(
                subcircuit.name.clone(),
                super::ModelSubcircuitInterface {
                    name: subcircuit.name.clone(),
                    ports: subcircuit.pins.clone(),
                    parameter_defaults: BTreeMap::new(),
                    description: subcircuit.description.clone(),
                    file_path: None,
                    source_line: None,
                    section: None,
                },
            );
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

#[cfg(test)]
mod builtin_catalog_tests;
#[cfg(test)]
mod tests;
