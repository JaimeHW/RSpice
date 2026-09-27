//! Complete PDK runtime validation and exact executable source closure sealing.

use rspice_app_types::product::ContentDigest;
use rspice_model_library::pdk::contracts::*;
use rspice_model_library::pdk::manifest::{
    PdkTechnologyManifest, package_path_to_host_path, signed_model_virtual_root,
    validate_package_path,
};
use rspice_model_library::pdk::package::{
    PdkTechnologyPackageMetadata, authenticate_archive, decode_bounded,
};
use rspice_model_library::pdk::{PdkPublisherTrustStore, PdkTechnologyError, content_digest};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

mod veriloga_sources;
use veriloga_sources::seal_pdk_veriloga_sources;

/// Package whose callbacks and executable source closures passed installation
/// validation. Registry deserialization still drops the runtime validation cache.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ValidatedPdkTechnologyPackage(PdkTechnologyPackageMetadata);

impl ValidatedPdkTechnologyPackage {
    #[must_use]
    pub fn manifest(&self) -> &PdkTechnologyManifest {
        self.0.manifest()
    }

    #[must_use]
    pub const fn manifest_digest(&self) -> ContentDigest {
        self.0.manifest_digest()
    }

    #[must_use]
    pub const fn archive_digest(&self) -> ContentDigest {
        self.0.archive_digest()
    }

    #[must_use]
    pub fn artifact_digests(&self) -> &BTreeMap<String, ContentDigest> {
        self.0.artifact_digests()
    }

    /// Signed technology symbols materialized against this archive's exact
    /// content-addressed model-source paths.
    #[must_use]
    pub fn symbol_definitions(
        &self,
    ) -> &[rspice_model_library::symbol::ModelBoundSymbolDefinition] {
        self.0.symbol_definitions()
    }

    pub fn runtime_compatibility(&self) -> Result<(), String> {
        validate_runtime_compatibility(self.manifest()).map_err(|error| error.to_string())
    }

    #[must_use]
    pub fn binding(&self) -> PdkTechnologyBinding {
        self.0.binding()
    }

    /// Recheck and compile this package's exact archived source closure.
    pub fn seal_model_sources(
        &self,
        archive: &SignedPdkTechnologyArchive,
    ) -> Result<SealedPdkModelSources, PdkTechnologyError> {
        seal_pdk_model_sources(archive, self)
    }

    /// Execute the selected callback against the exact validated archive.
    pub fn execute_callback(
        &self,
        archive: &SignedPdkTechnologyArchive,
        callback_id: &str,
        input: &rspice_model_library::pdk::callback::PdkCallbackExecutionInput,
    ) -> Result<
        rspice_model_library::pdk::callback::PdkCallbackExecutionReceipt,
        rspice_model_library::pdk::callback::PdkCallbackError,
    > {
        super::callback::execute_signed_callback(archive, self, callback_id, input)
    }
}

/// Runtime-only, exact source closure produced from one currently trusted
/// signed package. Every path is a content-addressed virtual identity; no
/// worker or browser execution path reopens a host file.
#[derive(Debug, Clone)]
pub struct SealedPdkModelSources(PdkModelSourceParts);

impl SealedPdkModelSources {
    pub fn as_parts(&self) -> &PdkModelSourceParts {
        &self.0
    }

    pub fn into_parts(self) -> PdkModelSourceParts {
        self.0
    }
}

/// Owned source data released by a sealed closure. Constructing these parts
/// alone cannot construct a seal or authorize a registry publication.
#[derive(Debug, Clone)]
pub struct PdkModelSourceParts {
    pub binding: PdkTechnologyBinding,
    pub archive_digest: ContentDigest,
    pub sources: Vec<(PathBuf, String)>,
    pub edges: Vec<rspice_core::netlist::SealedSourceEdge>,
    pub process_bindings: Vec<SealedPdkModelProcessBinding>,
    pub veriloga_artifacts: Vec<SealedPdkVerilogAArtifact>,
    pub veriloga_bindings: Vec<SealedPdkVerilogABinding>,
}

/// One source/section selected by a typed process contract inside a sealed
/// PDK model-source closure.
#[derive(Debug, Clone)]
pub struct SealedPdkModelProcessBinding {
    pub process: PdkModelProcess,
    pub source_id: String,
    pub domain: PdkModelDomain,
    pub root_path: PathBuf,
    pub artifact_path: String,
    pub artifact_digest: ContentDigest,
    pub section: Option<String>,
}

/// Exact UTF-8 Verilog-A artifact retained from one authenticated archive.
#[derive(Debug, Clone)]
pub struct SealedPdkVerilogAArtifact {
    pub path: String,
    pub source: String,
    pub digest: ContentDigest,
}

/// One signed manifest contract whose dependency closure compiled while the
/// package was validated. Execution recompiles these exact retained bytes and
/// checks the resulting runtime at the ordinary prepared-run boundary.
#[derive(Debug, Clone)]
pub struct SealedPdkVerilogABinding {
    pub source_id: String,
    pub root_artifact_path: String,
    pub root_artifact_digest: ContentDigest,
    pub module_name: String,
    pub netlist_alias: String,
}

pub fn validate_archive_bytes(
    bytes: &[u8],
    trust_store: &PdkPublisherTrustStore,
) -> Result<(SignedPdkTechnologyArchive, ValidatedPdkTechnologyPackage), PdkTechnologyError> {
    if bytes.len() > MAX_PDK_ARCHIVE_BYTES {
        return Err(PdkTechnologyError::ArchiveTooLarge {
            actual: bytes.len(),
            maximum: MAX_PDK_ARCHIVE_BYTES,
        });
    }
    let archive: SignedPdkTechnologyArchive = serde_json::from_slice(bytes)
        .map_err(|error| PdkTechnologyError::ArchiveParse(error.to_string()))?;
    let package = validate_archive(&archive, trust_store)?;
    Ok((archive, package))
}

pub fn validate_archive(
    archive: &SignedPdkTechnologyArchive,
    trust_store: &PdkPublisherTrustStore,
) -> Result<ValidatedPdkTechnologyPackage, PdkTechnologyError> {
    let package = ValidatedPdkTechnologyPackage(authenticate_archive(archive, trust_store)?);
    super::callback::validate_signed_callbacks(archive, &package)
        .map_err(PdkTechnologyError::CallbackValidation)?;
    // Executable model contracts are part of package validation, not a
    // deferred simulation-time best effort. This proves section existence,
    // package-relative dependency closure, source encoding, and reachability
    // using the exact decoded artifact bytes covered by the signed manifest.
    let _ = seal_pdk_model_sources(archive, &package)?;
    Ok(package)
}

fn seal_pdk_model_sources(
    archive: &SignedPdkTechnologyArchive,
    package: &ValidatedPdkTechnologyPackage,
) -> Result<SealedPdkModelSources, PdkTechnologyError> {
    let binding = package.binding();
    let archive_bytes = serde_json::to_vec(archive)
        .map_err(|error| PdkTechnologyError::Serialization(error.to_string()))?;
    if content_digest(&archive_bytes) != package.archive_digest() {
        return Err(PdkTechnologyError::ModelMaterialization(format!(
            "{} {} archive bytes no longer match the validated archive digest",
            binding.package_id, binding.revision
        )));
    }

    let model_artifacts = package
        .manifest()
        .artifacts
        .iter()
        .filter(|artifact| artifact.kind == PdkTechnologyArtifactKind::Model)
        .map(|artifact| (artifact.path.to_ascii_lowercase(), artifact))
        .collect::<BTreeMap<_, _>>();
    let (veriloga_artifacts, veriloga_bindings) = seal_pdk_veriloga_sources(archive, package)?;
    if model_artifacts.is_empty() {
        return Ok(SealedPdkModelSources(PdkModelSourceParts {
            binding,
            archive_digest: package.archive_digest(),
            sources: Vec::new(),
            edges: Vec::new(),
            process_bindings: Vec::new(),
            veriloga_artifacts,
            veriloga_bindings,
        }));
    }

    let archive_files = archive
        .files
        .iter()
        .map(|file| (file.path.to_ascii_lowercase(), file))
        .collect::<BTreeMap<_, _>>();
    let virtual_root = signed_model_virtual_root(&package.archive_digest().to_string());
    let mut virtual_paths = BTreeMap::<String, PathBuf>::new();
    let mut sources = Vec::<(PathBuf, String)>::with_capacity(model_artifacts.len());
    for (key, artifact) in &model_artifacts {
        let file = archive_files
            .get(key)
            .ok_or_else(|| PdkTechnologyError::MissingArtifact(artifact.path.clone()))?;
        let bytes = decode_bounded(
            &format!("files[{}].content_base64", file.path),
            &file.content_base64,
            MAX_PDK_ARTIFACT_BYTES,
        )?;
        let actual_digest = content_digest(&bytes);
        if actual_digest != artifact.sha256
            || package.artifact_digests().get(&artifact.path) != Some(&actual_digest)
        {
            return Err(PdkTechnologyError::ArtifactDigestMismatch {
                path: artifact.path.clone(),
                declared: artifact.sha256,
                actual: actual_digest,
            });
        }
        let actual_size = bytes.len();
        if u64::try_from(actual_size).ok() != Some(artifact.size_bytes) {
            return Err(PdkTechnologyError::ArtifactSizeMismatch {
                path: artifact.path.clone(),
                declared: artifact.size_bytes,
                actual: actual_size,
            });
        }
        let source = rspice_core::netlist::decode_source_bytes(&bytes).map_err(|error| {
            PdkTechnologyError::ModelMaterialization(format!(
                "model artifact '{}' cannot be decoded with the supported source policy: {error}",
                artifact.path
            ))
        })?;
        let path = virtual_root.join(package_path_to_host_path(&artifact.path));
        virtual_paths.insert(key.clone(), path.clone());
        sources.push((path, source));
    }
    sources.sort_by(|left, right| left.0.cmp(&right.0));

    let source_by_path = sources
        .iter()
        .map(|(path, source)| (path.clone(), source.as_str()))
        .collect::<BTreeMap<_, _>>();
    let mut edge_keys = BTreeSet::<(PathBuf, String, PathBuf)>::new();
    for (artifact_key, owner_path) in &virtual_paths {
        let source = source_by_path
            .get(owner_path)
            .expect("every virtual model path has decoded source");
        let owner_artifact = model_artifacts
            .get(artifact_key)
            .expect("every virtual model path names an artifact");
        for requested_path in pdk_external_source_paths(source) {
            let target_artifact_path =
                resolve_package_dependency(&owner_artifact.path, &requested_path)?;
            let target_key = target_artifact_path.to_ascii_lowercase();
            let target = virtual_paths.get(&target_key).ok_or_else(|| {
                PdkTechnologyError::ModelMaterialization(format!(
                    "model artifact '{}' references '{}' which is not a signed model artifact in this package",
                    owner_artifact.path, requested_path
                ))
            })?;
            edge_keys.insert((owner_path.clone(), requested_path, target.clone()));
        }
    }
    let edges = edge_keys
        .into_iter()
        .map(
            |(owner, requested_path, target)| rspice_core::netlist::SealedSourceEdge {
                owner,
                requested_path,
                target,
            },
        )
        .collect::<Vec<_>>();
    let bundle = rspice_core::netlist::SealedSourceBundle::try_new_with_edges(
        sources.clone(),
        edges.clone(),
    )
    .map_err(|error| {
        PdkTechnologyError::ModelMaterialization(format!(
            "signed model-source bundle is invalid: {error}"
        ))
    })?;

    let mut process_bindings = Vec::new();
    let mut reachable_artifacts = BTreeSet::<String>::new();
    for contract in &package.manifest().model_sources {
        for source in &contract.sources {
            let artifact_key = source.artifact_path.to_ascii_lowercase();
            let artifact = model_artifacts.get(&artifact_key).ok_or_else(|| {
                PdkTechnologyError::InvalidReference(format!(
                    "model source '{}' references missing model artifact '{}'",
                    source.source_id, source.artifact_path
                ))
            })?;
            let root_path = virtual_paths
                .get(&artifact_key)
                .expect("validated model artifact has a virtual path")
                .clone();
            let mut processor =
                rspice_core::netlist::IncludeProcessor::new_sealed(&root_path, bundle.clone());
            let materialized = processor
                .process_sealed_root(&root_path, source.section.as_deref())
                .map_err(|error| {
                    PdkTechnologyError::ModelMaterialization(format!(
                        "{} {} process {} source '{}' from '{}' could not be materialized: {error}",
                        binding.package_id,
                        binding.revision,
                        contract.process.keyword(),
                        source.source_id,
                        source.artifact_path
                    ))
                })?;
            if materialized.trim().is_empty() {
                return Err(PdkTechnologyError::ModelMaterialization(format!(
                    "{} {} process {} source '{}' materializes no executable model cards",
                    binding.package_id,
                    binding.revision,
                    contract.process.keyword(),
                    source.source_id
                )));
            }
            reachable_artifacts.insert(artifact_key);
            for dependency in processor.resolved_dependencies() {
                let dependency_key = virtual_paths
                    .iter()
                    .find_map(|(key, path)| {
                        (path == dependency.resolved_path()).then(|| key.clone())
                    })
                    .ok_or_else(|| {
                        PdkTechnologyError::ModelMaterialization(format!(
                            "resolved dependency '{}' escaped the signed package closure",
                            dependency.resolved_path().display()
                        ))
                    })?;
                reachable_artifacts.insert(dependency_key);
            }
            process_bindings.push(SealedPdkModelProcessBinding {
                process: contract.process,
                source_id: source.source_id.clone(),
                domain: source.domain,
                root_path,
                artifact_path: artifact.path.clone(),
                artifact_digest: artifact.sha256,
                section: source.section.clone(),
            });
        }
    }
    process_bindings.sort_by(|left, right| {
        left.process
            .cmp(&right.process)
            .then_with(|| left.domain.cmp(&right.domain))
            .then_with(|| {
                left.source_id
                    .to_ascii_lowercase()
                    .cmp(&right.source_id.to_ascii_lowercase())
            })
            .then_with(|| left.source_id.cmp(&right.source_id))
    });
    let unreachable = model_artifacts
        .keys()
        .filter(|path| !reachable_artifacts.contains(*path))
        .cloned()
        .collect::<Vec<_>>();
    if !unreachable.is_empty() {
        return Err(PdkTechnologyError::ModelMaterialization(format!(
            "signed model artifacts are unreachable from every declared process contract: {}",
            unreachable.join(", ")
        )));
    }

    Ok(SealedPdkModelSources(PdkModelSourceParts {
        binding,
        archive_digest: package.archive_digest(),
        sources,
        edges,
        process_bindings,
        veriloga_artifacts,
        veriloga_bindings,
    }))
}

fn pdk_external_source_paths(source: &str) -> Vec<String> {
    let mut paths = Vec::new();
    for line in source.lines() {
        if let Some((path, section)) = rspice_core::netlist::parse_lib_directive(line) {
            if section.is_some() {
                paths.push(path);
            }
            continue;
        }
        if let Some(path) = rspice_core::netlist::parse_include_directive(line) {
            paths.push(path);
        }
    }
    paths
}

fn resolve_package_dependency(
    owner_artifact_path: &str,
    requested_path: &str,
) -> Result<String, PdkTechnologyError> {
    let requested =
        rspice_core::netlist::normalize_source_path_literal(requested_path).map_err(|error| {
            PdkTechnologyError::ModelMaterialization(format!(
                "model artifact '{owner_artifact_path}' contains invalid dependency path '{requested_path}': {error}"
            ))
        })?;
    let requested = requested.replace('\\', "/");
    if requested.starts_with('/')
        || requested.starts_with("//")
        || requested
            .as_bytes()
            .get(1)
            .is_some_and(|separator| *separator == b':')
    {
        return Err(PdkTechnologyError::ModelMaterialization(format!(
            "model artifact '{owner_artifact_path}' requests external absolute dependency '{requested_path}'"
        )));
    }

    let mut components = owner_artifact_path
        .split('/')
        .map(str::to_owned)
        .collect::<Vec<_>>();
    components.pop();
    for component in requested.split('/') {
        match component {
            "" | "." => {}
            ".." => {
                if components.pop().is_none() {
                    return Err(PdkTechnologyError::ModelMaterialization(format!(
                        "model artifact '{owner_artifact_path}' dependency '{requested_path}' escapes the signed package root"
                    )));
                }
            }
            component => components.push(component.to_owned()),
        }
    }
    if components.is_empty() {
        return Err(PdkTechnologyError::ModelMaterialization(format!(
            "model artifact '{owner_artifact_path}' dependency '{requested_path}' has no package target"
        )));
    }
    let resolved = components.join("/");
    validate_package_path("model dependency target", &resolved)?;
    Ok(resolved)
}

pub fn validate_runtime_compatibility(
    manifest: &PdkTechnologyManifest,
) -> Result<(), PdkTechnologyError> {
    let current_target = current_execution_target();
    if !manifest.compatibility.targets.contains(&current_target) {
        return Err(PdkTechnologyError::IncompatibleRuntime(format!(
            "{} {} does not permit the current {current_target:?} execution target",
            manifest.package_id, manifest.revision
        )));
    }
    let current = semver::Version::parse(env!("CARGO_PKG_VERSION")).map_err(|error| {
        PdkTechnologyError::IncompatibleRuntime(format!("RSpice build version is invalid: {error}"))
    })?;
    for (component, minimum) in [
        ("engine", &manifest.compatibility.minimum_engine_version),
        ("viewer", &manifest.compatibility.minimum_viewer_version),
    ] {
        let minimum = semver::Version::parse(minimum).map_err(|error| {
            PdkTechnologyError::InvalidField(format!(
                "manifest.compatibility.minimum_{component}_version is invalid: {error}"
            ))
        })?;
        if current < minimum {
            return Err(PdkTechnologyError::IncompatibleRuntime(format!(
                "{} {} requires {component} {minimum} or newer; this build is {current}",
                manifest.package_id, manifest.revision
            )));
        }
    }
    Ok(())
}
