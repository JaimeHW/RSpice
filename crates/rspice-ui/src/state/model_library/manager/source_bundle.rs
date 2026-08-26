//! Authenticating a named set of in-memory sources into one model library.
//!
//! Two callers hand RSpice a bundle rather than a path: a browser upload,
//! which has no filesystem to read, and a Model Hub pack part, whose bytes
//! were proved as an archive and only *happen* to be expanded on a disk when
//! the store is a filesystem one. Both arrive as `(portable name, bytes)`
//! pairs with one executable root, so both are authenticated here and nowhere
//! else — which is what makes a project built in a browser and the same
//! project built on a desktop the same document.
//!
//! # A member identity is not a location
//!
//! Retained sources are addressed under `/rspice-browser/model-sources/`
//! followed by the digest of the whole reachable bundle, composed with forward
//! slashes on every host. Nothing ever opens these paths — the bytes travel
//! with the project — so the identity has to be stable across machines rather
//! than openable on any one of them.
//!
//! # Unreachable members are dropped, not retained
//!
//! Reachability is computed from the selected root through `.include`,
//! sectioned `.lib`, and Verilog-A edges, and only what it reaches is decoded,
//! validated, or kept. A pack that ships a licence file and a datasheet beside
//! its models therefore contributes neither to the project nor to its digest.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};

use sha2::{Digest as _, Sha256};

use super::ModelLibraryManager;
use crate::product::ContentDigest;
use crate::state::model_library::{
    ModelLibrary, ModelSourceAuthority, ModelSourceContent, ModelSourceEdge, ModelSourcePin,
    ProcessCorner,
};

pub(crate) fn normalize_browser_bundle_member_path(path: &str) -> Result<String, String> {
    if path.is_empty() {
        return Err("the relative path is empty".to_owned());
    }
    if path.chars().any(char::is_control) {
        return Err("the relative path contains a control character".to_owned());
    }

    let normalized = path.replace('\\', "/");
    if normalized.starts_with('/') {
        return Err("absolute paths are not allowed".to_owned());
    }

    let mut components = Vec::new();
    for component in normalized.split('/') {
        if component.is_empty() {
            return Err("empty path components are not allowed".to_owned());
        }
        if matches!(component, "." | "..") {
            return Err("'.' and '..' components are not allowed in member identities".to_owned());
        }
        if component.contains(':') {
            return Err("':' is not allowed in a portable member identity".to_owned());
        }
        components.push(component);
    }
    Ok(components.join("/"))
}

fn resolve_browser_bundle_dependency(owner: &str, requested_path: &str) -> Result<String, String> {
    let requested = rspice_core::netlist::normalize_source_path_literal(requested_path)
        .map_err(|error| error.to_string())?;
    if requested.starts_with('/') {
        return Err("absolute paths are not allowed".to_owned());
    }

    let mut components = owner.split('/').collect::<Vec<_>>();
    let owner_file = components
        .pop()
        .ok_or_else(|| "the owning source has no file name".to_owned())?;
    if owner_file.is_empty() {
        return Err("the owning source has no file name".to_owned());
    }

    for component in requested.split('/') {
        match component {
            "" => return Err("empty path components are not allowed".to_owned()),
            "." => {}
            ".." => {
                if components.pop().is_none() {
                    return Err("the dependency escapes the selected source tree".to_owned());
                }
            }
            component if component.contains(':') => {
                return Err("':' is not allowed in a portable dependency path".to_owned());
            }
            component => components.push(component),
        }
    }
    if components.is_empty() {
        return Err("the dependency does not identify a source file".to_owned());
    }
    Ok(components.join("/"))
}

fn browser_bundle_source_candidate(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "lib" | "model" | "mod" | "spice" | "cir" | "inc" | "scs"
            )
        })
}

fn browser_bundle_veriloga_member(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "va" | "vams" | "vh"
            )
        })
}

fn browser_veriloga_include(line: &str) -> Option<String> {
    let remainder = line.trim().strip_prefix("`include")?.trim_start();
    let remainder = remainder.strip_prefix('"')?;
    let end = remainder.find('"')?;
    Some(remainder[..end].to_owned())
}

fn browser_bundle_direct_dependencies(
    owner: &str,
    bytes: &[u8],
) -> Result<Vec<(usize, String)>, String> {
    let source = rspice_core::netlist::decode_source_bytes(bytes)
        .map_err(|error| format!("Uploaded model source '{owner}' cannot be decoded: {error}"))?;
    if browser_bundle_veriloga_member(owner) {
        return Ok(source
            .lines()
            .enumerate()
            .filter_map(|(line, source)| {
                browser_veriloga_include(source).map(|path| (line + 1, path))
            })
            .collect());
    }

    let projection = rspice_core::library::adapt_spectre_model_library(Path::new(owner), &source)
        .map_err(|error| {
        format!(
            "{owner}:{} cannot be imported as an executable model library: {}",
            error.line, error.message
        )
    })?;
    Ok(projection
        .lines()
        .enumerate()
        .filter_map(|(line, source)| {
            rspice_core::netlist::parse_include_directive(source)
                .or_else(|| {
                    rspice_core::netlist::parse_lib_directive(source)
                        .and_then(|(path, section)| section.map(|_| path))
                })
                .or_else(|| {
                    rspice_core::netlist::parse_veriloga_source_directive(source)
                        .map(|include| include.file_path.to_string_lossy().into_owned())
                })
                .map(|path| (line + 1, path))
        })
        .collect())
}

fn infer_browser_bundle_root(
    members: &BTreeMap<String, Vec<u8>>,
    case_folded: &HashMap<String, String>,
) -> Result<Option<String>, String> {
    if members.len() == 1 {
        return Ok(members.keys().next().cloned());
    }
    let candidates = members
        .keys()
        .filter(|path| browser_bundle_source_candidate(path))
        .cloned()
        .collect::<Vec<_>>();
    let mut dependency_targets = HashSet::new();
    for owner in &candidates {
        let Ok(dependencies) = browser_bundle_direct_dependencies(owner, &members[owner]) else {
            continue;
        };
        for (_, requested) in dependencies {
            let Ok(normalized) = resolve_browser_bundle_dependency(owner, &requested) else {
                continue;
            };
            if let Some(target) = case_folded.get(&normalized.to_ascii_lowercase()) {
                dependency_targets.insert(target.clone());
            }
        }
    }
    let mut roots = candidates
        .into_iter()
        .filter(|candidate| !dependency_targets.contains(candidate))
        .collect::<Vec<_>>();
    Ok((roots.len() == 1).then(|| roots.pop().expect("one inferred root")))
}

fn reachable_browser_bundle_members(
    root: &str,
    members: &BTreeMap<String, Vec<u8>>,
    case_folded: &HashMap<String, String>,
) -> Result<HashSet<String>, String> {
    let mut reachable = HashSet::new();
    let mut pending = VecDeque::from([root.to_owned()]);
    while let Some(owner) = pending.pop_front() {
        if !reachable.insert(owner.clone()) {
            continue;
        }
        let bytes = members
            .get(&owner)
            .ok_or_else(|| format!("Selected model source root '{owner}' disappeared"))?;
        for (line, requested) in browser_bundle_direct_dependencies(&owner, bytes)? {
            let normalized =
                resolve_browser_bundle_dependency(&owner, &requested).map_err(|error| {
                    format!("{owner}:{line} has an invalid dependency path '{requested}': {error}")
                })?;
            let target = case_folded
                .get(&normalized.to_ascii_lowercase())
                .ok_or_else(|| {
                    format!(
                        "{owner}:{line} dependency '{requested}' is missing from the selected browser bundle"
                    )
                })?
                .clone();
            pending.push_back(target);
        }
    }
    Ok(reachable)
}

/// Authenticate a named set of in-memory sources into one library value.
///
/// This owns everything an import decides — which member is the root, which
/// members it can reach, what the retained closure and its resolution edges
/// are — and decides nothing about the catalog it will join. Publication is
/// the caller's, because the two callers answer a name collision
/// differently: a browser upload refuses to replace a library it did not
/// import, while re-adding the same pinned pack part is the same bytes
/// under the same identity and reuses what it already produced.
pub(super) fn build(
    display_name: &str,
    root_member: Option<&str>,
    files: Vec<(String, Vec<u8>)>,
    section: Option<&str>,
) -> Result<(String, ModelLibrary), String> {
    use rspice_core::library::{LibParser, ResolvedLibDependency};

    if files.is_empty() {
        return Err("Model source bundle contains no files".to_owned());
    }
    if files.len() > crate::state::MAX_PROJECT_SOURCE_FILES {
        return Err(format!(
            "Model source bundle contains {} files; the limit is {}",
            files.len(),
            crate::state::MAX_PROJECT_SOURCE_FILES
        ));
    }
    let total_bytes = files.iter().try_fold(0usize, |total, (_, bytes)| {
        total
            .checked_add(bytes.len())
            .ok_or_else(|| "Model source bundle size overflowed".to_owned())
    })?;
    if total_bytes > crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES {
        return Err(format!(
            "Model source bundle contains {total_bytes} bytes; the limit is {}",
            crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES
        ));
    }

    let mut members = BTreeMap::<String, Vec<u8>>::new();
    let mut case_folded = HashMap::<String, String>::new();
    for (name, bytes) in files {
        let safe_name = normalize_browser_bundle_member_path(&name)
            .map_err(|error| format!("Model source bundle member '{name}' is invalid: {error}"))?;
        if case_folded
            .insert(safe_name.to_ascii_lowercase(), safe_name.clone())
            .is_some()
        {
            return Err(format!(
                "Model source bundle repeats portable path '{safe_name}' ignoring case"
            ));
        }
        members.insert(safe_name, bytes);
    }

    let requested_root = root_member
        .map(normalize_browser_bundle_member_path)
        .transpose()
        .map_err(|error| format!("Model source bundle root is invalid: {error}"))?;
    let selected_root = if let Some(requested_root) = requested_root {
        case_folded
            .get(&requested_root.to_ascii_lowercase())
            .cloned()
            .ok_or_else(|| {
                format!("Model source bundle does not contain selected root '{requested_root}'")
            })?
    } else {
        let display_root = normalize_browser_bundle_member_path(display_name)
            .ok()
            .and_then(|display_member| {
                case_folded
                    .get(&display_member.to_ascii_lowercase())
                    .cloned()
            });
        display_root
            .or(infer_browser_bundle_root(&members, &case_folded)?)
            .ok_or_else(|| {
            "Model source bundle has more than one possible executable root; select the entry file explicitly"
                .to_owned()
            })?
    };

    let reachable = reachable_browser_bundle_members(&selected_root, &members, &case_folded)?;
    members.retain(|name, _| reachable.contains(name));
    case_folded.retain(|_, name| reachable.contains(name));

    let mut bundle_hasher = Sha256::new();
    for (name, bytes) in &members {
        bundle_hasher.update((name.len() as u64).to_be_bytes());
        bundle_hasher.update(name.as_bytes());
        bundle_hasher.update((bytes.len() as u64).to_be_bytes());
        bundle_hasher.update(bytes);
    }
    let bundle_digest = ContentDigest::from_bytes(bundle_hasher.finalize().into());
    let base = format!("/rspice-browser/model-sources/{bundle_digest}");
    // The identity is composed with forward slashes rather than joined as
    // a host path. Nothing ever opens these — they name authenticated
    // bytes the project already carries — so a host separator inside one
    // would be the single thing making the same retained source look
    // different on Windows than it does anywhere else.
    let member_paths = members
        .keys()
        .map(|name| (name.clone(), PathBuf::from(format!("{base}/{name}"))))
        .collect::<HashMap<_, _>>();
    let mut dependencies = Vec::<ResolvedLibDependency>::new();
    let mut decoded_members = BTreeMap::<String, String>::new();
    let mut veriloga_roots = BTreeSet::<String>::new();
    for (owner_name, bytes) in &members {
        let source = rspice_core::netlist::decode_source_bytes(bytes).map_err(|error| {
            format!("Uploaded model source '{owner_name}' cannot be decoded: {error}")
        })?;
        let dependency_projection =
            rspice_core::library::adapt_spectre_model_library(Path::new(owner_name), &source)
                .map_err(|error| {
                    format!(
                        "{owner_name}:{} cannot be imported as an executable model library: {}",
                        error.line, error.message
                    )
                })?;
        decoded_members.insert(owner_name.clone(), source.clone());
        let owner = member_paths
            .get(owner_name)
            .expect("every source member has a virtual identity")
            .clone();
        for (line_index, line) in dependency_projection.lines().enumerate() {
            let dependency = rspice_core::netlist::parse_include_directive(line)
                .map(|path| (path, false))
                .or_else(|| {
                    rspice_core::netlist::parse_lib_directive(line)
                        .and_then(|(path, section)| section.map(|_| (path, false)))
                })
                .or_else(|| {
                    rspice_core::netlist::parse_veriloga_source_directive(line)
                        .map(|include| (include.file_path.to_string_lossy().into_owned(), true))
                });
            let Some((requested_path, is_veriloga)) = dependency else {
                continue;
            };
            let normalized = resolve_browser_bundle_dependency(owner_name, &requested_path)
                .map_err(|error| {
                format!(
                    "{owner_name}:{} has an invalid dependency path '{requested_path}': {error}",
                    line_index + 1
                )
            })?;
            let target_name = case_folded
                .get(&normalized.to_ascii_lowercase())
                .ok_or_else(|| {
                format!(
                    "{owner_name}:{} dependency '{requested_path}' is missing from the selected browser bundle",
                    line_index + 1
                )
            })?;
            let target = member_paths
                .get(target_name)
                .expect("case-folded member identity belongs to the virtual bundle");
            if is_veriloga {
                veriloga_roots.insert(target_name.clone());
            }
            dependencies.push(ResolvedLibDependency {
                owner: owner.clone(),
                requested_path,
                target: target.clone(),
            });
        }
    }

    let virtual_files = decoded_members
        .iter()
        .map(|(path, source)| rspice_veriloga::VirtualSourceFile::new(path, source))
        .collect::<Vec<_>>();
    let veriloga_limits = rspice_veriloga::VirtualCompileLimits {
        max_files: crate::state::MAX_PROJECT_SOURCE_FILES,
        max_path_bytes: crate::state::MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES,
        max_file_bytes: crate::state::MAX_PROJECT_CODE_SOURCE_BYTES,
        max_total_source_bytes: crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES,
        max_include_depth: crate::state::MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH,
        max_expanded_bytes: crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES.saturating_mul(2),
        max_module_name_bytes: 128,
    };
    for veriloga_root in &veriloga_roots {
        let bundle =
            rspice_veriloga::VirtualSourceBundle::new(veriloga_root, virtual_files.iter().cloned())
                .map_err(|error| {
                    format!(
                        "Uploaded Verilog-A bundle rooted at '{veriloga_root}' is invalid: {error}"
                    )
                })?;
        let discovery = rspice_veriloga::VerilogACompiler::default()
            .discover_virtual_modules(&bundle, veriloga_limits)
            .map_err(|error| {
                format!(
                    "Uploaded Verilog-A bundle rooted at '{veriloga_root}' cannot be compiled: {error}"
                )
            })?;
        for include in discovery.include_graph {
            let Some(owner_name) = case_folded.get(&include.including_path.to_ascii_lowercase())
            else {
                // Compiler-owned standard headers never become project
                // model-library artifacts.
                continue;
            };
            let Some(target_name) = case_folded.get(&include.included_path.to_ascii_lowercase())
            else {
                continue;
            };
            let owner = member_paths
                .get(owner_name)
                .expect("Verilog-A owner belongs to the selected bundle");
            let target = member_paths
                .get(target_name)
                .expect("Verilog-A dependency belongs to the selected bundle");
            dependencies.push(ResolvedLibDependency {
                owner: owner.clone(),
                requested_path: include.requested_path,
                target: target.clone(),
            });
        }
    }
    dependencies.sort();
    dependencies.dedup();

    let sources = members
        .iter()
        .map(|(name, bytes)| (member_paths[name].clone(), bytes.clone()))
        .collect::<Vec<_>>();
    let root_name = selected_root;
    let root = member_paths[&root_name].clone();
    let root_bytes = sources
        .iter()
        .find_map(|(path, bytes)| (path == &root).then_some(bytes))
        .expect("the authenticated bundle contains its root");
    let root_digest = ContentDigest::from_bytes(Sha256::digest(root_bytes).into());
    let lib_name = Path::new(&root_name)
        .file_stem()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or("uploaded-models")
        .to_owned();

    let authenticated_dependencies = dependencies.clone();
    let mut parser = LibParser::new(root.parent().unwrap_or(Path::new("/")));
    let result = parser
        .parse_authenticated_closure(root.clone(), sources.clone(), dependencies)
        .map_err(|error| format!("Uploaded model bundle could not be authenticated: {error}"))?;
    if !result.is_ok() {
        return Err(format!(
            "Uploaded model bundle contains parse or unresolved dependency errors: {}",
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
        digest: root_digest,
    };
    library.source_closure = sources
        .iter()
        .map(|(path, bytes)| ModelSourcePin {
            path: path.clone(),
            digest: ContentDigest::from_bytes(Sha256::digest(bytes).into()),
        })
        .collect();
    library
        .source_closure
        .sort_by(|left, right| left.path.cmp(&right.path));
    library.source_contents = sources
        .into_iter()
        .map(|(path, bytes)| ModelSourceContent { path, bytes })
        .collect();
    library
        .source_contents
        .sort_by(|left, right| left.path.cmp(&right.path));
    library.source_edges = authenticated_dependencies
        .iter()
        .map(|dependency| ModelSourceEdge {
            owner: dependency.owner.clone(),
            requested_path: dependency.requested_path.clone(),
            target: dependency.target.clone(),
        })
        .collect();
    library.source_edges.sort();
    // The bundle's own sections are the whole corner catalog. Catalog and
    // selection are cleared together: a library pointing at a corner it does
    // not define fails projection later with "selected corner does not
    // exist". A section below re-selects when the source declares one.
    library.corners.clear();
    library.selected_corner = None;
    for section_name in result.section_names() {
        let mut corner = ProcessCorner::from_composite_section(section_name, root.clone(), false);
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
        let device_model = ModelLibraryManager::convert_parsed_model(model, &root);
        library
            .top_level_models
            .insert(device_model.name.clone(), device_model.clone());
        library
            .models
            .insert(device_model.name.clone(), device_model);
    }
    ModelLibraryManager::insert_parsed_subcircuits(
        &mut library,
        &result.top_level_subcircuits,
        &root,
        None,
    )?;
    for lib_section in &result.sections {
        ModelLibraryManager::insert_parsed_subcircuits(
            &mut library,
            &lib_section.subcircuits,
            &root,
            Some(&lib_section.name),
        )?;
    }
    for lib_section in &result.sections {
        let section_models = library
            .section_models
            .entry(lib_section.name.clone())
            .or_default();
        for model in &lib_section.models {
            let device_model = ModelLibraryManager::convert_parsed_model_in_section(
                model,
                &root,
                Some(&lib_section.name),
            );
            section_models.insert(device_model.name.clone(), device_model);
        }
    }
    if let Some(section_name) = selected_section.as_deref() {
        let lib_section = result.get_section(section_name).ok_or_else(|| {
            format!(
                "Section '{section_name}' not found. Available: {:?}",
                result.section_names()
            )
        })?;
        library.selected_corner = Some(lib_section.name.clone());
        if let Some(corner) = library.corners.get_mut(&lib_section.name) {
            corner.is_default = true;
        }
    }
    library.refresh_effective_model_projection();
    // A macromodel library legitimately declares only `.subckt`
    // definitions, so an empty model map alone is not an empty library.
    if library.top_level_models.is_empty()
        && library.section_models.values().all(HashMap::is_empty)
        && library.subcircuits.is_empty()
    {
        return Err(format!(
            "Model library '{lib_name}' contains no supported device models or addressable subcircuits"
        ));
    }
    Ok((lib_name, library))
}
