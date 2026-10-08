//! Bounded native bundle container, digest, and versioned JSON decoding.

use super::{NativeBundleError, NativeBundleKind};
use crate::numeric::{DecodedNumericDataset, DecodedNumericSignal};
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

/// Versioned file fields, retained internally until waveform interpretation.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeDataset {
    schema: String,
    analysis: String,
    coordinate: NativeCoordinate,
    signals: Vec<NativeSignal>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeCoordinate {
    name: String,
    #[serde(default)]
    unit: Option<String>,
    values: Vec<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeSignal {
    name: String,
    #[serde(default)]
    unit: Option<String>,
    #[serde(default)]
    values: Option<Vec<f64>>,
    #[serde(default)]
    real: Option<Vec<f64>>,
    #[serde(default)]
    imag: Option<Vec<f64>>,
}

/// Verify the bounded container and digest before decoding the versioned file fields.
pub fn decode_native_bundle(
    bytes: &[u8],
    kind: NativeBundleKind,
    limits: NativeBundleReadLimits,
) -> Result<DecodedNumericDataset, NativeBundleError> {
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
    if !matches!(
        dataset.schema.as_str(),
        "rspice-waveform-dataset/1" | "rspice-waveform-dataset/2"
    ) {
        return Err(NativeBundleError::InvalidData(format!(
            "unsupported dataset schema '{}'",
            dataset.schema
        )));
    }
    if dataset.schema == "rspice-waveform-dataset/1" && dataset.coordinate.unit.is_some() {
        return Err(NativeBundleError::InvalidData(
            "coordinate units require rspice-waveform-dataset/2".into(),
        ));
    }
    let domain = dataset
        .analysis
        .parse::<crate::WaveformDomain>()
        .map_err(NativeBundleError::AnalysisDomain)?;
    let mut signals = Vec::with_capacity(dataset.signals.len());
    for signal in dataset.signals {
        let (real, imag) = match (signal.values, signal.real, signal.imag) {
            (Some(values), None, None) => (values, None),
            (None, Some(real), Some(imag)) => (real, Some(imag)),
            (values, real, imag) => {
                return Err(NativeBundleError::SignalRepresentation {
                    name: signal.name,
                    values: values.is_some(),
                    real: real.is_some(),
                    imag: imag.is_some(),
                });
            }
        };
        signals.push(DecodedNumericSignal {
            name: signal.name,
            real,
            imag,
            unit: signal.unit,
        });
    }
    let mut decoded = DecodedNumericDataset {
        coordinate_unit: dataset.coordinate.unit,
        domain,
        coordinate_name: dataset.coordinate.name,
        coordinate: dataset.coordinate.values,
        signals,
    };
    decoded
        .normalize_coordinate_unit()
        .map_err(NativeBundleError::InvalidData)?;
    Ok(decoded)
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
