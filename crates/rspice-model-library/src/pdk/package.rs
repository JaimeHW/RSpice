//! Signed-archive authentication and portable package metadata.
//!
//! Source compilation, callback execution and registry publication belong above
//! this data boundary and cannot be authorized by deserialized metadata.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use rspice_app_types::product::ContentDigest;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::contracts::*;
use super::manifest::{
    PdkTechnologyManifest, package_path_to_host_path, signed_model_virtual_root, validate_manifest,
    validate_package_path,
};
use super::{PdkPublisherTrustStore, PdkTechnologyError, content_digest};

/// Manifest and artifact identities produced by signed-archive authentication.
///
/// This record carries no runtime authority. Installation also validates callback
/// modules and executable source closures. Deserializing this record does not
/// authenticate it or restore an installation's runtime validation cache.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyPackageMetadata {
    manifest: PdkTechnologyManifest,
    manifest_digest: ContentDigest,
    archive_digest: ContentDigest,
    artifact_digests: BTreeMap<String, ContentDigest>,
    symbol_definitions: Vec<crate::symbol::ModelBoundSymbolDefinition>,
}

impl PdkTechnologyPackageMetadata {
    #[must_use]
    pub fn manifest(&self) -> &PdkTechnologyManifest {
        &self.manifest
    }

    #[must_use]
    pub const fn manifest_digest(&self) -> ContentDigest {
        self.manifest_digest
    }

    #[must_use]
    pub const fn archive_digest(&self) -> ContentDigest {
        self.archive_digest
    }

    #[must_use]
    pub fn artifact_digests(&self) -> &BTreeMap<String, ContentDigest> {
        &self.artifact_digests
    }

    /// Signed technology symbols materialized against this archive's exact
    /// content-addressed model-source paths.
    #[must_use]
    pub fn symbol_definitions(&self) -> &[crate::symbol::ModelBoundSymbolDefinition] {
        &self.symbol_definitions
    }

    #[must_use]
    pub fn binding(&self) -> PdkTechnologyBinding {
        PdkTechnologyBinding {
            package_id: self.manifest.package_id.clone(),
            revision: self.manifest.revision.clone(),
            manifest_digest: self.manifest_digest,
        }
    }
}

/// Authenticate the exact manifest and every declared artifact, preserving their
/// normalized archive identity. Runtime validation remains the installer's duty.
pub fn authenticate_archive(
    archive: &SignedPdkTechnologyArchive,
    trust_store: &PdkPublisherTrustStore,
) -> Result<PdkTechnologyPackageMetadata, PdkTechnologyError> {
    if archive.schema_version != PDK_TECHNOLOGY_ARCHIVE_SCHEMA_VERSION {
        return Err(PdkTechnologyError::UnsupportedSchema {
            object: "archive",
            actual: archive.schema_version,
            supported: PDK_TECHNOLOGY_ARCHIVE_SCHEMA_VERSION,
        });
    }
    if archive.files.len() > MAX_PDK_ARTIFACTS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "archive has {} files; maximum is {MAX_PDK_ARTIFACTS}",
            archive.files.len()
        )));
    }
    let manifest_bytes = decode_bounded(
        "manifest_base64",
        &archive.manifest_base64,
        MAX_PDK_MANIFEST_BYTES,
    )?;
    let manifest: PdkTechnologyManifest = serde_json::from_slice(&manifest_bytes)
        .map_err(|error| PdkTechnologyError::ManifestParse(error.to_string()))?;
    validate_manifest(&manifest)?;

    let signature_bytes = decode_bounded("signature_base64", &archive.signature_base64, 64)?;
    trust_store.verify_publisher_signature(
        &manifest.publisher_id,
        &manifest.signing_key_id,
        &manifest_bytes,
        &signature_bytes,
    )?;

    let mut actual_files = BTreeMap::<String, (usize, ContentDigest)>::new();
    let mut total = 0usize;
    for (index, file) in archive.files.iter().enumerate() {
        validate_package_path(&format!("files[{index}].path"), &file.path)?;
        let normalized = file.path.to_ascii_lowercase();
        if actual_files.contains_key(&normalized) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "archive repeats case-insensitive path '{}'",
                file.path
            )));
        }
        let bytes = decode_bounded(
            &format!("files[{index}].content_base64"),
            &file.content_base64,
            MAX_PDK_ARTIFACT_BYTES,
        )?;
        total = total.checked_add(bytes.len()).ok_or_else(|| {
            PdkTechnologyError::LimitExceeded("archive byte count overflow".to_owned())
        })?;
        if total > MAX_PDK_TOTAL_ARTIFACT_BYTES {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "decoded artifact bytes exceed {MAX_PDK_TOTAL_ARTIFACT_BYTES}"
            )));
        }
        actual_files.insert(normalized, (bytes.len(), content_digest(&bytes)));
    }

    let mut artifact_digests = BTreeMap::new();
    for (index, artifact) in manifest.artifacts.iter().enumerate() {
        let key = artifact.path.to_ascii_lowercase();
        let Some((actual_size, actual_digest)) = actual_files.remove(&key) else {
            return Err(PdkTechnologyError::MissingArtifact(artifact.path.clone()));
        };
        let declared_size = usize::try_from(artifact.size_bytes).map_err(|_| {
            PdkTechnologyError::InvalidField(format!(
                "artifacts[{index}].size_bytes cannot be represented on this platform"
            ))
        })?;
        if declared_size != actual_size {
            return Err(PdkTechnologyError::ArtifactSizeMismatch {
                path: artifact.path.clone(),
                declared: artifact.size_bytes,
                actual: actual_size,
            });
        }
        if artifact.sha256 != actual_digest {
            return Err(PdkTechnologyError::ArtifactDigestMismatch {
                path: artifact.path.clone(),
                declared: artifact.sha256,
                actual: actual_digest,
            });
        }
        artifact_digests.insert(artifact.path.clone(), actual_digest);
    }
    if let Some((extra, _)) = actual_files.first_key_value() {
        return Err(PdkTechnologyError::UndeclaredArtifact(extra.clone()));
    }

    let archive_digest = content_digest(
        &serde_json::to_vec(archive)
            .map_err(|error| PdkTechnologyError::Serialization(error.to_string()))?,
    );
    let symbol_definitions = materialize_signed_symbol_definitions(&manifest, archive_digest)?;
    let package = PdkTechnologyPackageMetadata {
        manifest,
        manifest_digest: content_digest(&manifest_bytes),
        // JSON envelope whitespace is intentionally not part of package
        // identity. The signature binds the exact manifest bytes and each
        // manifest digest binds exact decoded artifact bytes. This digest
        // binds the complete normalized envelope identically before and after
        // persistence.
        archive_digest,
        artifact_digests,
        symbol_definitions,
    };
    Ok(package)
}

fn materialize_signed_symbol_definitions(
    manifest: &PdkTechnologyManifest,
    archive_digest: ContentDigest,
) -> Result<Vec<crate::symbol::ModelBoundSymbolDefinition>, PdkTechnologyError> {
    let virtual_root = signed_model_virtual_root(&archive_digest.to_string());
    let mut definitions = Vec::with_capacity(manifest.symbol_definitions.len());
    for (index, signed) in manifest.symbol_definitions.iter().enumerate() {
        let mut definition = signed.clone();
        let crate::symbol::SymbolSourceContract::Model { model, .. } = &mut definition.source
        else {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] is not model-bound"
            )));
        };
        let package_path = model.source_path.as_deref().ok_or_else(|| {
            PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] has no model source path"
            ))
        })?;
        let source_path = virtual_root
            .join(package_path_to_host_path(package_path))
            .to_string_lossy()
            .into_owned();
        model.source_path = Some(source_path.clone());
        definition
            .netlist
            .model
            .as_mut()
            .ok_or_else(|| {
                PdkTechnologyError::InvalidField(format!(
                    "manifest.symbol_definitions[{index}] has no executable model binding"
                ))
            })?
            .source_path = Some(source_path);
        definition.validate().map_err(|error| {
            PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] is invalid after signed source materialization: {error}"
            ))
        })?;
        definitions.push(definition);
    }
    definitions.sort_by(|left, right| {
        left.identity
            .cell
            .to_ascii_lowercase()
            .cmp(&right.identity.cell.to_ascii_lowercase())
    });
    Ok(definitions)
}

pub fn decode_bounded(
    field: &str,
    value: &str,
    maximum: usize,
) -> Result<Vec<u8>, PdkTechnologyError> {
    let approximate = value.len().saturating_mul(3) / 4;
    if approximate > maximum.saturating_add(3) {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "{field} exceeds {maximum} decoded bytes"
        )));
    }
    let bytes = STANDARD
        .decode(value)
        .map_err(|error| PdkTechnologyError::InvalidBase64 {
            field: field.to_owned(),
            detail: error.to_string(),
        })?;
    if bytes.len() > maximum {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "{field} contains {} decoded bytes; maximum is {maximum}",
            bytes.len()
        )));
    }
    Ok(bytes)
}
