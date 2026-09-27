//! Bounded native bundle container, digest, and versioned JSON decoding.

use super::{NativeBundleError, NativeBundleKind};
use serde::Deserialize;
use std::collections::HashSet;
use std::io::{Cursor, Read};

/// Bounds supplied by the importing transaction.
#[derive(Debug, Clone, Copy)]
pub struct NativeBundleReadLimits {
    pub max_members: usize,
    pub max_expanded_bytes: u64,
    pub max_member_bytes: u64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeBundleManifest {
    schema: String,
    dataset_member: String,
    dataset_sha256: String,
}

/// Decoded file fields; the caller applies result-domain and signal policies.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeDataset {
    schema: String,
    pub analysis: String,
    pub coordinate: NativeCoordinate,
    pub signals: Vec<NativeSignal>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeCoordinate {
    pub name: String,
    pub values: Vec<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeSignal {
    pub name: String,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub values: Option<Vec<f64>>,
    #[serde(default)]
    pub real: Option<Vec<f64>>,
    #[serde(default)]
    pub imag: Option<Vec<f64>>,
}

/// Verify the bounded container and digest before decoding the versioned file fields.
pub fn decode_native_bundle(
    bytes: &[u8],
    kind: NativeBundleKind,
    limits: NativeBundleReadLimits,
) -> Result<NativeDataset, NativeBundleError> {
    let max_members = limits.max_members;
    let max_expanded_bytes = limits.max_expanded_bytes;
    let mut archive =
        zip::ZipArchive::new(Cursor::new(bytes)).map_err(|error| NativeBundleError::Zip {
            context: "invalid ZIP container".into(),
            source: error,
        })?;
    if archive.len() > max_members {
        return Err(NativeBundleError::InvalidData(format!(
            "archive has {} members; the limit is {max_members}",
            archive.len()
        )));
    }
    let mut names = HashSet::with_capacity(archive.len());
    let mut expanded = 0_u64;
    for index in 0..archive.len() {
        let file = archive
            .by_index(index)
            .map_err(|error| NativeBundleError::Zip {
                context: format!("invalid member {index}"),
                source: error,
            })?;
        expanded = expanded.checked_add(file.size()).ok_or_else(|| {
            NativeBundleError::InvalidData("archive expanded-size accounting overflow".into())
        })?;
        if expanded > max_expanded_bytes {
            return Err(NativeBundleError::InvalidData(format!(
                "archive expands to {expanded} bytes; the limit is {max_expanded_bytes}"
            )));
        }
        let name = file.name().to_owned();
        if !names.insert(name.clone()) {
            return Err(NativeBundleError::InvalidData(format!(
                "archive repeats member '{name}'"
            )));
        }
        if file.is_dir() || name.starts_with('/') || name.contains("..") || name.contains('\\') {
            return Err(NativeBundleError::InvalidData(format!(
                "unsafe or unsupported archive member '{name}'"
            )));
        }
    }
    let manifest_bytes = read_zip_member(&mut archive, "manifest.json", limits.max_member_bytes)?;
    let manifest: NativeBundleManifest =
        serde_json::from_slice(&manifest_bytes).map_err(|error| NativeBundleError::Json {
            context: "manifest.json is invalid",
            source: error,
        })?;
    let expected_schema = kind.manifest_schema();
    if manifest.schema != expected_schema {
        return Err(NativeBundleError::InvalidData(format!(
            "manifest schema '{}' is not supported; expected '{expected_schema}'",
            manifest.schema
        )));
    }
    if manifest.dataset_member != "dataset.json" {
        return Err(NativeBundleError::InvalidData(
            "manifest must bind the canonical dataset.json member".into(),
        ));
    }
    let dataset_bytes = read_zip_member(
        &mut archive,
        &manifest.dataset_member,
        limits.max_member_bytes,
    )?;
    use sha2::Digest as _;
    let digest = format!("{:x}", sha2::Sha256::digest(&dataset_bytes));
    if !manifest.dataset_sha256.eq_ignore_ascii_case(&digest) {
        return Err(NativeBundleError::InvalidData(
            "dataset.json SHA-256 does not match the signed manifest identity".into(),
        ));
    }
    let dataset: NativeDataset =
        serde_json::from_slice(&dataset_bytes).map_err(|error| NativeBundleError::Json {
            context: "dataset.json is invalid",
            source: error,
        })?;
    if dataset.schema != "rspice-waveform-dataset/1" {
        return Err(NativeBundleError::InvalidData(format!(
            "unsupported dataset schema '{}'",
            dataset.schema
        )));
    }
    Ok(dataset)
}

pub(super) fn read_zip_member(
    archive: &mut zip::ZipArchive<Cursor<&[u8]>>,
    name: &str,
    max_member_bytes: u64,
) -> Result<Vec<u8>, NativeBundleError> {
    let file = archive
        .by_name(name)
        .map_err(|error| NativeBundleError::Zip {
            context: format!("missing '{name}'"),
            source: error,
        })?;
    if file.size() > max_member_bytes {
        return Err(NativeBundleError::InvalidData(format!(
            "'{name}' exceeds the byte limit"
        )));
    }
    let mut bytes = Vec::with_capacity(usize::try_from(file.size()).unwrap_or(0));
    file.take(max_member_bytes.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|error| NativeBundleError::Io {
            context: format!("could not decode '{name}'"),
            source: error,
        })?;
    if bytes.len() as u64 > max_member_bytes {
        return Err(NativeBundleError::InvalidData(format!(
            "'{name}' exceeds the byte limit"
        )));
    }
    Ok(bytes)
}
