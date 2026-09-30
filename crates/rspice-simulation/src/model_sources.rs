//! Authentication and materialization of immutable model-source snapshots.

mod binding_authority;
pub use binding_authority::validate_projected_model_binding_authority;

mod catalog_sealing;
pub use catalog_sealing::{seal_catalog_execution_sources, seal_plan_execution_sources};

mod project_provenance;
pub use project_provenance::prepared_project_model_sources;

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use rspice_app_types::product::{ContentDigest, ProcessCorner as CornerProcess};
#[cfg(not(target_arch = "wasm32"))]
use rspice_model_library::is_foreign_platform_absolute_path;
use rspice_model_library::source_paths::{portable_dependency_target_key, portable_path_key};
use rspice_model_library::{
    CornerModelBinding, MaterializedPlanBinding, ModelExecutionPlan, ModelLibrary,
    ModelResolutionRecord, ModelSourceAuthority, ProcessCorner, first_unreachable_source,
    is_portable_absolute_path, resolve_materialized_definition_namespace,
};
use sha2::{Digest as _, Sha256};

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
    pdk_process_bindings: Vec<crate::pdk::SealedPdkModelProcessBinding>,
    pdk_veriloga_artifacts: Vec<crate::pdk::SealedPdkVerilogAArtifact>,
    pdk_veriloga_bindings: Vec<crate::pdk::SealedPdkVerilogABinding>,
    pdk_identity: Option<(
        rspice_model_library::pdk::contracts::PdkTechnologyBinding,
        ContentDigest,
    )>,
    resolution_records: Vec<ModelResolutionRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SealedModelLibraryVerilogARoot {
    pub path: PathBuf,
    pub netlist_alias: Option<String>,
    pub selected_module: Option<String>,
}

/// Exact model-library bytes and AHDL roots authenticated by one run seal.
/// Signed-PDK artifacts are intentionally excluded; they have their own
/// manifest-governed authority and compiler path.
#[derive(Debug, Clone)]
pub struct SealedModelLibraryVerilogAAuthority {
    closure_digest: ContentDigest,
    sources: Vec<(PathBuf, String)>,
    roots: Vec<SealedModelLibraryVerilogARoot>,
}

impl SealedModelLibraryVerilogAAuthority {
    pub fn closure_digest(&self) -> ContentDigest {
        self.closure_digest
    }

    pub fn sources(&self) -> &[(PathBuf, String)] {
        &self.sources
    }

    pub fn roots(&self) -> &[SealedModelLibraryVerilogARoot] {
        &self.roots
    }
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

const fn pdk_model_process(
    process: CornerProcess,
) -> rspice_model_library::pdk::contracts::PdkModelProcess {
    match process {
        CornerProcess::TT => rspice_model_library::pdk::contracts::PdkModelProcess::Tt,
        CornerProcess::SS => rspice_model_library::pdk::contracts::PdkModelProcess::Ss,
        CornerProcess::FF => rspice_model_library::pdk::contracts::PdkModelProcess::Ff,
        CornerProcess::SF => rspice_model_library::pdk::contracts::PdkModelProcess::Sf,
        CornerProcess::FS => rspice_model_library::pdk::contracts::PdkModelProcess::Fs,
    }
}

impl SealedModelExecutionSources {
    /// Merge an authenticated signed PDK closure into this source snapshot.
    pub fn with_pdk_model_sources(
        mut self,
        pdk: crate::pdk::SealedPdkModelSources,
    ) -> Result<Self, String> {
        let pdk = pdk.into_parts();
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
    pub fn pdk_model_identity(&self) -> Option<(String, ContentDigest)> {
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
    pub fn pdk_veriloga_authority(
        &self,
    ) -> Option<(
        &rspice_model_library::pdk::contracts::PdkTechnologyBinding,
        ContentDigest,
        &[crate::pdk::SealedPdkVerilogAArtifact],
        &[crate::pdk::SealedPdkVerilogABinding],
    )> {
        let (binding, archive_digest) = self.pdk_identity.as_ref()?;
        Some((
            binding,
            *archive_digest,
            &self.pdk_veriloga_artifacts,
            &self.pdk_veriloga_bindings,
        ))
    }

    /// Add the active root using exact portable identities from this sealed closure.
    /// Unresolved or ambiguous dependencies fail without consulting host search paths.
    pub fn bundle_for_root(
        &self,
        root_path: &Path,
        root_source: &str,
    ) -> Result<rspice_core::netlist::SealedSourceBundle, String> {
        if !is_portable_absolute_path(root_path) {
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
    pub fn expand_root_dependencies(
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
        process: CornerProcess,
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
        process: CornerProcess,
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
            BTreeMap::<(PathBuf, String), Vec<rspice_model_library::CornerSectionDomain>>::new();
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

/// Authenticate selected library bytes and freeze their materialization inputs.
///
/// The caller validates live catalog decisions and plan bindings before selecting
/// libraries. This checks the complete pinned byte closure through the supplied
/// host reader; it does not authorize publication or dispatch into a live project.
pub fn seal_model_sources<F>(
    libraries: Vec<(&ModelLibrary, Option<String>)>,
    resolution_records: &BTreeMap<String, ModelResolutionRecord>,
    mut read_external: F,
) -> Result<SealedModelExecutionSources, String>
where
    F: FnMut(&Path) -> Result<Vec<u8>, String>,
{
    // The final flag selects retained project bytes (`true`) or a live,
    // re-authenticated external read (`false`). A path can never mix the
    // two authorities across libraries.
    let mut expected_sources = BTreeMap::<PathBuf, (ContentDigest, Vec<String>, bool)>::new();
    let mut retained_sources = BTreeMap::<PathBuf, Vec<u8>>::new();
    let mut expected_edges = BTreeMap::<(PathBuf, String), PathBuf>::new();
    let mut sealed_libraries = Vec::with_capacity(libraries.len());
    for (library, selected_corner) in libraries {
        let root_path = library.root_path.as_ref().ok_or_else(|| {
            format!(
                "Model library '{}' declares source authority but has no root identity",
                library.name
            )
        })?;
        // Both a project-owned library and a retained import carry their bytes in
        // the project, so both are sealed from those bytes rather than re-read
        // from a host path that may not exist here.
        let retained = library.source_authority.uses_retained_bytes();
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
            if !retained && is_foreign_platform_absolute_path(&source.path) {
                return Err(format!(
                    "Model library '{}' retains foreign-platform dependency '{}', which is unavailable on this host; re-import or repair the binding before simulation",
                    library.name,
                    source.path.display()
                ));
            }
            if !is_portable_absolute_path(&source.path) {
                return Err(format!(
                    "Model library '{}' has a non-canonical dependency path '{}'; refresh or re-import it before simulation",
                    library.name,
                    source.path.display()
                ));
            }
            match expected_sources.entry(source.path.clone()) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert((source.digest, vec![library.name.clone()], retained));
                }
                std::collections::btree_map::Entry::Occupied(mut entry) => {
                    if entry.get().0 != source.digest {
                        return Err(format!(
                            "Model libraries disagree on the accepted SHA-256 for shared dependency '{}'",
                            source.path.display()
                        ));
                    }
                    if entry.get().2 != retained {
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
            && let Some(unreachable) =
                first_unreachable_source(root_path, &library.source_closure, &library.source_edges)
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
            let requested_path = rspice_core::netlist::normalize_source_path_literal(
                &edge.requested_path,
            )
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
                let actual = ContentDigest::from_bytes(Sha256::digest(&content.bytes).into());
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
        if retained {
            let Some(digest) = library.source_authority.retained_root_digest() else {
                unreachable!("retained authority was checked above")
            };
            if library.source_contents.len() != library.source_closure.len() {
                return Err(format!(
                    "Model library '{}' retains its bytes and must hold exact content for every authenticated source member",
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

        // Seal the corner definitions themselves, not just their names: a
        // corner carries the section bindings and source path that
        // materializing a process card needs, and re-deriving them from a
        // name after sealing would reopen the very state the seal fixes.
        let mut corners = library
            .corners
            .values()
            .filter(|corner| corner.file_path.is_some() || !corner.section_bindings.is_empty())
            .cloned()
            .collect::<Vec<_>>();
        corners.sort_by_key(|corner| corner.name.to_ascii_lowercase());
        if let Some(pair) = corners
            .windows(2)
            .find(|pair| pair[0].name.eq_ignore_ascii_case(&pair[1].name))
        {
            return Err(format!(
                "Model library '{}' defines case-insensitive duplicate corners '{}' and '{}'",
                library.name, pair[0].name, pair[1].name
            ));
        }
        for corner in &corners {
            let Some(source_path) = corner.file_path.as_ref() else {
                continue;
            };
            if !source_paths.contains(source_path) {
                return Err(format!(
                    "Model library '{}' corner '{}' binds source '{}' outside its authenticated closure",
                    library.name,
                    corner.name,
                    source_path.display()
                ));
            }
        }
        sealed_libraries.push(SealedExecutionLibrary {
            name: library.name.clone(),
            provenance: library.provenance_label(),
            root_path: root_path.clone(),
            source_digest: library
                .pinned_root_digest()
                .expect("the sealed source closure contains its root pin"),
            corners,
            selected_corner,
            allows_selected_section_override: matches!(
                library.source_authority,
                ModelSourceAuthority::ProjectOwned { .. }
            ),
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
    for (path, (expected_digest, owners, retained)) in expected_sources {
        let bytes = if retained {
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
        let actual_digest = ContentDigest::from_bytes(Sha256::digest(&bytes).into());
        if actual_digest != expected_digest {
            return Err(if retained {
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
        model_library_source_paths: authenticated_sources
            .iter()
            .map(|(path, _)| path.clone())
            .collect(),
        sources: authenticated_sources,
        edges,
        libraries: sealed_libraries,
        // A signed PDK contributes its bindings through
        // `with_pdk_model_sources` after sealing; a project with no PDK pin
        // seals with none.
        pdk_process_bindings: Vec::new(),
        pdk_veriloga_artifacts: Vec::new(),
        pdk_veriloga_bindings: Vec::new(),
        pdk_identity: None,
        resolution_records: resolution_records.values().cloned().collect(),
    })
}

impl SealedModelExecutionSources {
    pub fn model_library_veriloga_authority(
        &self,
    ) -> Result<Option<SealedModelLibraryVerilogAAuthority>, String> {
        let model_paths = self
            .model_library_source_paths
            .iter()
            .map(|path| portable_path_key(path))
            .collect::<HashSet<_>>();
        let mut sources = self
            .sources
            .iter()
            .filter(|(path, _)| model_paths.contains(&portable_path_key(path)))
            .cloned()
            .collect::<Vec<_>>();
        sources.sort_by(|left, right| left.0.cmp(&right.0));

        let mut roots = Vec::<SealedModelLibraryVerilogARoot>::new();
        for (owner, source) in &sources {
            let projected = rspice_core::library::adapt_spectre_model_library(owner, source)
                .map_err(|error| {
                    format!(
                        "Authenticated model source '{}':{} no longer satisfies the Spectre adapter: {}",
                        owner.display(),
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
                        "Authenticated Verilog-A dependency in '{}' is invalid: {error}",
                        owner.display()
                    )
                })?;
                let matches = self
                    .edges
                    .iter()
                    .filter(|edge| {
                        portable_path_key(&edge.owner) == portable_path_key(owner)
                            && rspice_core::netlist::normalize_source_path_literal(
                                &edge.requested_path,
                            )
                            .is_ok_and(|edge_requested| edge_requested == requested)
                    })
                    .collect::<Vec<_>>();
                let [edge] = matches.as_slice() else {
                    return Err(format!(
                        "Authenticated Verilog-A dependency '{}' in '{}' has {} exact resolution edges; refresh or re-import the model library",
                        requested,
                        owner.display(),
                        matches.len()
                    ));
                };
                if !model_paths.contains(&portable_path_key(&edge.target)) {
                    return Err(format!(
                        "Authenticated Verilog-A dependency '{}' resolves outside the sealed model-library authority",
                        requested
                    ));
                }
                roots.push(SealedModelLibraryVerilogARoot {
                    path: edge.target.clone(),
                    netlist_alias: include.model_name,
                    selected_module: include.selected_module,
                });
            }
        }
        roots.sort_by(|left, right| {
            left.path
                .cmp(&right.path)
                .then_with(|| left.selected_module.cmp(&right.selected_module))
                .then_with(|| {
                    left.netlist_alias
                        .as_deref()
                        .unwrap_or_default()
                        .cmp(right.netlist_alias.as_deref().unwrap_or_default())
                })
        });
        roots.dedup();
        for pair in roots.windows(2) {
            if portable_path_key(&pair[0].path) == portable_path_key(&pair[1].path)
                && pair[0].selected_module == pair[1].selected_module
                && pair[0].netlist_alias != pair[1].netlist_alias
            {
                return Err(format!(
                    "Verilog-A source '{}' is included with conflicting model aliases",
                    pair[1].path.display()
                ));
            }
        }
        if roots.is_empty() {
            return Ok(None);
        }

        let mut hasher = Sha256::new();
        hasher.update(b"rspice.sealed-model-library-veriloga/v2\0");
        for (path, source) in &sources {
            let path = portable_path_key(path);
            hasher.update((path.len() as u64).to_le_bytes());
            hasher.update(path.as_bytes());
            hasher.update((source.len() as u64).to_le_bytes());
            hasher.update(source.as_bytes());
        }
        for root in &roots {
            let path = portable_path_key(&root.path);
            hasher.update((path.len() as u64).to_le_bytes());
            hasher.update(path.as_bytes());
            if let Some(module) = &root.selected_module {
                hasher.update((module.len() as u64).to_le_bytes());
                hasher.update(module.as_bytes());
            } else {
                hasher.update(0_u64.to_le_bytes());
            }
            if let Some(alias) = &root.netlist_alias {
                hasher.update((alias.len() as u64).to_le_bytes());
                hasher.update(alias.as_bytes());
            } else {
                hasher.update(0_u64.to_le_bytes());
            }
        }
        Ok(Some(SealedModelLibraryVerilogAAuthority {
            closure_digest: ContentDigest::from_bytes(hasher.finalize().into()),
            sources,
            roots,
        }))
    }
}

#[cfg(test)]
mod tests;
