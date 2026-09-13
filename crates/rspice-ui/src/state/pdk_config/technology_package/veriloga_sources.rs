//! Authenticate and compile the exact Verilog-A closure of a signed PDK.
//!
//! Every declared source must be reachable from a runtime contract, and its
//! bytes must still match the signed artifact before compilation or sealing.

use super::*;

pub(super) fn seal_pdk_veriloga_sources(
    archive: &SignedPdkTechnologyArchive,
    package: &ValidatedPdkTechnologyPackage,
) -> Result<
    (
        Vec<SealedPdkVerilogAArtifact>,
        Vec<SealedPdkVerilogABinding>,
    ),
    PdkTechnologyError,
> {
    let declared = package
        .manifest
        .artifacts
        .iter()
        .filter(|artifact| artifact.kind == PdkTechnologyArtifactKind::VerilogASource)
        .map(|artifact| (artifact.path.to_ascii_lowercase(), artifact))
        .collect::<BTreeMap<_, _>>();
    if declared.is_empty() {
        return Ok((Vec::new(), Vec::new()));
    }

    let archive_files = archive
        .files
        .iter()
        .map(|file| (file.path.to_ascii_lowercase(), file))
        .collect::<BTreeMap<_, _>>();
    let mut artifacts = Vec::with_capacity(declared.len());
    for (key, artifact) in &declared {
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
            || package.artifact_digests.get(&artifact.path) != Some(&actual_digest)
        {
            return Err(PdkTechnologyError::ArtifactDigestMismatch {
                path: artifact.path.clone(),
                declared: artifact.sha256,
                actual: actual_digest,
            });
        }
        if u64::try_from(bytes.len()).ok() != Some(artifact.size_bytes) {
            return Err(PdkTechnologyError::ArtifactSizeMismatch {
                path: artifact.path.clone(),
                declared: artifact.size_bytes,
                actual: bytes.len(),
            });
        }
        let source = rspice_core::netlist::decode_source_bytes(&bytes).map_err(|error| {
            PdkTechnologyError::ModelMaterialization(format!(
                "Verilog-A artifact '{}' cannot be decoded with the supported source policy: {error}",
                artifact.path
            ))
        })?;
        artifacts.push(SealedPdkVerilogAArtifact {
            path: artifact.path.clone(),
            source,
            digest: actual_digest,
        });
    }
    artifacts.sort_by(|left, right| {
        left.path
            .to_ascii_lowercase()
            .cmp(&right.path.to_ascii_lowercase())
            .then_with(|| left.path.cmp(&right.path))
    });

    let virtual_files = artifacts
        .iter()
        .map(|artifact| rspice_veriloga::VirtualSourceFile::new(&artifact.path, &artifact.source))
        .collect::<Vec<_>>();
    let limits = rspice_veriloga::VirtualCompileLimits {
        max_files: MAX_PDK_ARTIFACTS,
        max_path_bytes: 1_024,
        max_file_bytes: MAX_PDK_ARTIFACT_BYTES,
        max_total_source_bytes: MAX_PDK_TOTAL_ARTIFACT_BYTES,
        max_include_depth: 64,
        max_expanded_bytes: MAX_PDK_TOTAL_ARTIFACT_BYTES.saturating_mul(2),
        max_module_name_bytes: 128,
    };
    let mut reachable = BTreeSet::<String>::new();
    let mut bindings = Vec::with_capacity(package.manifest.veriloga_sources.len());
    for contract in &package.manifest.veriloga_sources {
        let root_key = contract.root_artifact_path.to_ascii_lowercase();
        let root = declared.get(&root_key).ok_or_else(|| {
            PdkTechnologyError::InvalidReference(format!(
                "Verilog-A source '{}' references missing artifact '{}'",
                contract.source_id, contract.root_artifact_path
            ))
        })?;
        let bundle = rspice_veriloga::VirtualSourceBundle::new(
            &contract.root_artifact_path,
            virtual_files.clone(),
        )
        .map_err(|error| {
            PdkTechnologyError::ModelMaterialization(format!(
                "signed Verilog-A source bundle for '{}' is invalid: {error}",
                contract.source_id
            ))
        })?;
        let compilation = rspice_veriloga::VerilogACompiler::new(
            crate::state::model_library::compilation::unified_runtime_compiler_options(),
        )
        .compile_virtual_runtime(&bundle, &contract.module_name, limits)
        .map_err(|error| {
            PdkTechnologyError::ModelMaterialization(format!(
                "signed Verilog-A source '{}' module '{}' could not be compiled: {error}",
                contract.source_id, contract.module_name
            ))
        })?;
        compilation.validate_integrity().map_err(|error| {
            PdkTechnologyError::ModelMaterialization(format!(
                "compiled Verilog-A source '{}' failed its integrity check: {error}",
                contract.source_id
            ))
        })?;
        for dependency in &compilation.dependency_closure {
            let key = dependency.logical_path.to_ascii_lowercase();
            if declared.contains_key(&key) {
                reachable.insert(key);
            }
        }
        bindings.push(SealedPdkVerilogABinding {
            source_id: contract.source_id.clone(),
            root_artifact_path: root.path.clone(),
            root_artifact_digest: root.sha256,
            module_name: contract.module_name.clone(),
            netlist_alias: contract.netlist_alias.clone(),
        });
    }
    bindings.sort_by(|left, right| {
        left.source_id
            .to_ascii_lowercase()
            .cmp(&right.source_id.to_ascii_lowercase())
            .then_with(|| left.source_id.cmp(&right.source_id))
    });
    let unreachable = declared
        .keys()
        .filter(|path| !reachable.contains(*path))
        .cloned()
        .collect::<Vec<_>>();
    if !unreachable.is_empty() {
        return Err(PdkTechnologyError::ModelMaterialization(format!(
            "signed Verilog-A artifacts are unreachable from every declared runtime contract: {}",
            unreachable.join(", ")
        )));
    }
    Ok((artifacts, bindings))
}
