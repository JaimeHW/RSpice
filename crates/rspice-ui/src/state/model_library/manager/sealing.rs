//! Sealing the model sources a run will execute against.
//!
//! A seal pins each source by content digest and closes over its dependency
//! edges, so what a run executes is fixed at seal time and cannot be changed
//! by editing a file afterwards. Digests are computed, never accepted: a
//! source whose content no longer matches its stored digest fails the seal
//! rather than being silently re-pinned to the new bytes.

use super::*;

impl ModelLibraryManager {
    /// Compute the canonical SHA-256 identity used to pin an external model
    /// source. Callers compare this value with the digest stored by the last
    /// explicit load/refresh; computing it never accepts new content.
    #[cfg(not(target_arch = "wasm32"))]
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

    pub(super) fn seal_execution_sources_with_reader<F>(
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
                if !super::super::is_portable_absolute_path(&source.path) {
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
                root_path: root_path.clone(),
                corners,
                selected_corner: library.selected_corner.clone(),
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
            let actual_digest =
                crate::product::ContentDigest::from_bytes(Sha256::digest(&bytes).into());
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
        })
    }
}
