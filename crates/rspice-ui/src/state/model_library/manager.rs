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
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rspice_core::library::SpiceLibraryIndex;

#[cfg(test)]
use super::ModelFileIdentity;
#[cfg(not(target_arch = "wasm32"))]
use super::is_foreign_platform_absolute_path;
use super::{
    DeviceModel, FiniteF64, ModelCorrelationState, ModelDefinitionMetadata, ModelLevel,
    ModelLibrary, ModelQualificationState, ModelSectionQualification, ModelSourceAuthority,
    ModelSourceContent, ModelSourceEdge, ModelSourceEvidenceBinding, ModelSourcePin, ModelType,
    ParameterDataType, ParameterDefinition, ParameterSource, ParameterValue, ProcessCorner,
    ProjectModelDefinition, ProjectModelRevisionDefinition, first_unreachable_source,
    subcircuit_interface_key,
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

/// Immutable, content-addressed model namespace used by one nominal run.
///
/// This is the semantic boundary shared by preflight, save validation, and
/// prepared-run construction.  It records the exact per-library corner that
/// was selected when sources were sealed and rejects a contested executable
/// namespace before the engine can fall back to first-definition lookup.
#[derive(Debug, Clone)]
pub struct ModelExecutionPlan {
    reference_process: crate::simulation::dialog::corner::ProcessCorner,
    selected_library_corners: Vec<(String, Option<String>)>,
    bindings: Vec<CornerModelBinding>,
    digest: ContentDigest,
}

impl ModelExecutionPlan {
    #[must_use]
    pub const fn reference_process(&self) -> crate::simulation::dialog::corner::ProcessCorner {
        self.reference_process
    }

    #[must_use]
    pub fn selected_library_corners(&self) -> &[(String, Option<String>)] {
        &self.selected_library_corners
    }

    #[must_use]
    pub fn bindings(&self) -> &[CornerModelBinding] {
        &self.bindings
    }

    #[must_use]
    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }

    #[must_use]
    pub fn model_cards(&self) -> Vec<String> {
        self.bindings
            .iter()
            .map(|binding| {
                format!(
                    "* RSpice sealed model source: {}\n{}",
                    binding.source_label, binding.materialized_model_cards
                )
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct SealedExecutionLibrary {
    name: String,
    root_path: PathBuf,
    corners: Vec<ProcessCorner>,
    selected_corner: Option<String>,
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

const fn simulation_corner_process(
    process: crate::simulation::dialog::corner::ProcessCorner,
) -> CornerProcess {
    match process {
        crate::simulation::dialog::corner::ProcessCorner::TT => CornerProcess::TT,
        crate::simulation::dialog::corner::ProcessCorner::SS => CornerProcess::SS,
        crate::simulation::dialog::corner::ProcessCorner::FF => CornerProcess::FF,
        crate::simulation::dialog::corner::ProcessCorner::SF => CornerProcess::SF,
        crate::simulation::dialog::corner::ProcessCorner::FS => CornerProcess::FS,
    }
}

fn hash_plan_field(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_le_bytes());
    hasher.update(value);
}

fn validate_materialized_definition_namespace_by_process(
    bindings: &[CornerModelBinding],
) -> Result<(), String> {
    for process in [
        CornerProcess::TT,
        CornerProcess::SS,
        CornerProcess::FF,
        CornerProcess::SF,
        CornerProcess::FS,
    ] {
        let process_bindings = bindings
            .iter()
            .filter(|binding| binding.process == process)
            .cloned()
            .collect::<Vec<_>>();
        if !process_bindings.is_empty() {
            validate_materialized_definition_namespace(&process_bindings)?;
        }
    }
    Ok(())
}

/// Reject a materialized namespace whose result would otherwise depend on the
/// first definition returned by the core engine.  SPICE model and subcircuit
/// names intentionally share this conservative top-level conflict index: an
/// explicit, persisted resolution record is required before such a contested
/// name can ever become executable.
fn validate_materialized_definition_namespace(
    bindings: &[CornerModelBinding],
) -> Result<(), String> {
    let mut providers = BTreeMap::<String, (String, Vec<String>)>::new();
    for binding in bindings {
        let source = format!(
            "{}{}",
            binding.source_label,
            binding
                .section
                .as_deref()
                .map(|section| format!(" [{section}]"))
                .unwrap_or_default()
        );
        let wrapped = format!(
            "RSpice sealed model namespace\n{}\n.end\n",
            binding.materialized_model_cards
        );
        let parsed = rspice_core::netlist::parse_netlist(&wrapped).map_err(|error| {
            format!("Cannot validate executable model namespace from '{source}': {error}")
        })?;
        for (name, kind) in parsed
            .models
            .iter()
            .map(|definition| (definition.name.as_str(), "model"))
            .chain(
                parsed
                    .subcircuits
                    .iter()
                    .map(|definition| (definition.name.as_str(), "subcircuit")),
            )
        {
            let entry = providers
                .entry(name.to_ascii_lowercase())
                .or_insert_with(|| (name.to_owned(), Vec::new()));
            entry.1.push(format!("{kind} in {source}"));
        }
    }

    let conflicts = providers
        .into_values()
        .filter(|(_, providers)| providers.len() > 1)
        .take(8)
        .map(|(name, providers)| format!("'{name}' from {}", providers.join(", ")))
        .collect::<Vec<_>>();
    if conflicts.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "Executable model namespace is contested and fails closed: {}. Remove or rename the duplicate definition before simulation.",
            conflicts.join("; ")
        ))
    }
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
        process: crate::simulation::dialog::corner::ProcessCorner,
    ) -> Result<ModelExecutionPlan, String> {
        let corner_process = simulation_corner_process(process);
        let bindings = self.bindings_for_processes(&[corner_process], true)?;
        validate_materialized_definition_namespace(&bindings)?;

        let selected_library_corners = self
            .libraries
            .iter()
            .map(|library| {
                let selected = library.selected_corner.clone().or_else(|| {
                    library
                        .corners
                        .iter()
                        .find(|corner| {
                            corner
                                .name
                                .eq_ignore_ascii_case(corner_process.as_keyword())
                        })
                        .map(|corner| corner.name.clone())
                });
                (library.name.clone(), selected)
            })
            .collect::<Vec<_>>();

        let mut hasher = Sha256::new();
        hasher.update(b"rspice.model-execution-plan/v1\0");
        hasher.update(corner_process.as_keyword().as_bytes());
        for (library, corner) in &selected_library_corners {
            hash_plan_field(&mut hasher, library.as_bytes());
            hash_plan_field(&mut hasher, corner.as_deref().unwrap_or("").as_bytes());
        }
        for binding in &bindings {
            hash_plan_field(&mut hasher, binding.source_label.as_bytes());
            hash_plan_field(
                &mut hasher,
                binding.section.as_deref().unwrap_or("").as_bytes(),
            );
            hash_plan_field(&mut hasher, binding.materialized_model_cards.as_bytes());
        }
        let digest = ContentDigest::from_bytes(hasher.finalize().into());

        Ok(ModelExecutionPlan {
            reference_process: process,
            selected_library_corners,
            bindings,
            digest,
        })
    }

    /// Materialize the exact model cards for the nominal/reference process.
    pub fn reference_process_model_cards(
        &self,
        process: crate::simulation::dialog::corner::ProcessCorner,
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
        let bindings = self.bindings_for_processes(processes, false)?;
        validate_materialized_definition_namespace_by_process(&bindings)?;
        Ok(bindings)
    }

    fn bindings_for_processes(
        &self,
        processes: &[CornerProcess],
        honor_nominal_selection: bool,
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
    /// Convert a parsed card that a `.lib` section owns, recording the section
    /// as execution provenance. Cards at file scope use
    /// [`Self::convert_parsed_model`], which leaves the section unset.
    pub(crate) fn convert_parsed_model_in_section(
        model: &rspice_core::library::ParsedModel,
        file_path: &Path,
        section: Option<&str>,
    ) -> DeviceModel {
        let mut converted = Self::convert_parsed_model(model, file_path);
        converted.section = section.map(str::to_owned);
        converted
    }

    /// Project one parsed subcircuit onto its callable interface. A subcircuit
    /// carries its own source file when it was reached through an include, so
    /// that path wins over the root being scanned.
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

    pub(crate) fn convert_parsed_subcircuit(
        subcircuit: &rspice_core::library::ParsedSubcircuit,
        file_path: &Path,
        section: Option<&str>,
    ) -> super::ModelSubcircuitInterface {
        super::ModelSubcircuitInterface {
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

    /// Stable identity of the persisted model catalogue relevant to source
    /// preparation. Browser filters, selection, shipped-pack indexes, and
    /// audit ledgers are deliberately excluded.
    pub(crate) fn execution_catalog_digest(&self) -> ContentDigest {
        let mut libraries = self.libraries.values().collect::<Vec<_>>();
        libraries.sort_by(|left, right| left.name.cmp(&right.name));
        let mut hasher = Sha256::new();
        hasher.update(b"rspice.model-execution-catalog/v4\0");
        for library in libraries {
            // A library owns several `HashMap` fields, so serializing it
            // directly emits their entries in per-instance iteration order and
            // yields a different digest for identical content. Route through
            // `serde_json::Value`, whose objects are key-sorted maps, so the
            // catalogue identity depends only on the content itself. A prepared
            // run compares this digest before dispatch; an order-dependent one
            // expires authorized runs at random.
            let bytes = serde_json::to_value(library)
                .and_then(|canonical| serde_json::to_vec(&canonical))
                .unwrap_or_else(|error| format!("serialization-error:{error}").into_bytes());
            hasher.update((bytes.len() as u64).to_le_bytes());
            hasher.update(bytes);
        }
        ContentDigest::from_bytes(hasher.finalize().into())
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
        self.spice_packs
            .as_ref()
            .map_or(0, |index| index.part_count())
    }

    /// Search the shipped packs for definitions whose name contains `query`.
    ///
    /// Bounded by `limit` because a short query matches tens of thousands of
    /// rows; the catalogue view is a browser, not a dump. An empty query
    /// returns nothing rather than everything, so opening the tab does not
    /// stream a 16 MB index off disk.
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
        device_filter: Option<&str>,
        limit: usize,
    ) -> Result<Vec<PackModelHit>, String> {
        let Some(index) = self.spice_packs.as_ref() else {
            return Ok(Vec::new());
        };
        let query = query.trim();
        let entries = if !query.is_empty() {
            index.search_parts(query, limit)
        } else if let Some(device) = device_filter {
            index.parts_by_device(device, limit)
        } else {
            index.browse_parts(limit)
        }
        .map_err(|error| format!("Shipped model catalog could not be read: {error}"))?;

        Ok(entries
            .into_iter()
            .filter(|entry| {
                device_filter.is_none_or(|device| {
                    let entry_device = entry.device.to_ascii_lowercase();
                    match device {
                        "mosfet" => entry_device.contains("mos"),
                        "bipolar" => {
                            entry_device.contains("bjt") || entry_device.contains("bipolar")
                        }
                        "jfet" => entry_device.contains("jfet") || entry_device.contains("hemt"),
                        "passive" => ["resistor", "capacitor", "inductor", "passive"]
                            .iter()
                            .any(|kind| entry_device.contains(kind)),
                        "subckt" => entry.kind.eq_ignore_ascii_case("subckt"),
                        other => entry_device.contains(other),
                    }
                })
            })
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
            .collect())
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
        self.load_catalog_source_without_collision(&entry)
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
        self.load_catalog_source_without_collision(&source)
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
            // Build the corner through its section contract rather than by
            // field assignment: a corner with no section binding materializes
            // to nothing, so a bare name would seal an empty corner.
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
        // Every section's interfaces are retained, not just the selected one:
        // a subcircuit is addressable by section-qualified identity, and a
        // library that declares only `.subckt` definitions is still a library.
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
    #[cfg(any(test, target_arch = "wasm32"))]
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
        // Imported bytes the project retained, not a definition the project
        // authored. The distinction is load-bearing: a project-owned library
        // carries a revision and typed definition metadata, and
        // `project_model_definition_identities` fails closed when a
        // project-owned model has none. An import has neither, so recording it
        // as project-owned makes that check demand metadata that cannot exist.
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
        // `ModelLibrary::new` seeds the standard corners and selects "tt".
        // Clearing the catalogue without clearing the selection leaves a
        // library pointing at a corner it no longer defines, which fails
        // projection later with "selected corner does not exist". A section
        // below re-selects when the source actually declares one.
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
                library
                    .models
                    .insert(device_model.name.clone(), device_model);
            }
            library.selected_corner = Some(lib_section.name.clone());
            if let Some(corner) = library.corners.get_mut(&lib_section.name) {
                corner.is_default = true;
            }
        }
        // A macromodel library legitimately declares only `.subckt`
        // definitions, so an empty model map alone is not an empty library.
        if library.models.is_empty() && library.subcircuits.is_empty() {
            return Err(format!(
                "Model library '{lib_name}' contains no supported device models or addressable subcircuits"
            ));
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
                    // Built-in cards are compiled in at file scope; no `.lib`
                    // section owns them.
                    section: None,
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
            // This conversion has no section context; a card parsed inside a
            // `.lib` is attributed by the caller that knows the section.
            section: None,
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

#[cfg(test)]
fn reconcile_project_model_metadata(
    definition: &ProjectModelDefinition,
    previous: Option<&ModelDefinitionMetadata>,
) -> Result<ModelDefinitionMetadata, String> {
    if previous.is_some_and(|metadata| !metadata.sections.is_empty()) {
        return Err(
            "A sectioned model must be changed through the complete project-model revision transaction"
                .to_owned(),
        );
    }
    reconcile_project_model_revision_metadata(definition, previous)
}

fn reconcile_project_model_revision_metadata(
    definition: &ProjectModelDefinition,
    previous: Option<&ModelDefinitionMetadata>,
) -> Result<ModelDefinitionMetadata, String> {
    let mut metadata = previous.cloned().unwrap_or_default();
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

#[cfg(test)]
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
