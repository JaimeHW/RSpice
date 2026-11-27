//! Bounded native bundle container, digest, and versioned JSON decoding.

use super::{NativeBundleError, NativeBundleKind, NativeBundleMember};
use crate::numeric::{DecodedNumericDataset, DecodedNumericSignal};
use serde::Deserialize;
use std::collections::HashSet;
use std::io::{Cursor, Read};

mod admission;
mod json_samples;

/// Bounds supplied by the importing transaction.
#[derive(Debug, Clone, Copy)]
pub struct NativeBundleReadLimits {
    pub max_members: usize,
    pub max_expanded_bytes: u64,
    pub max_member_bytes: u64,
    /// Maximum samples in any coordinate or signal component.
    pub max_rows: usize,
    /// Maximum signals plus the shared coordinate column.
    pub max_columns: usize,
    /// Retained values, including the magnitude beside both complex components.
    pub max_numeric_values: usize,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeBundleManifest {
    schema: String,
    dataset_member: String,
    dataset_sha256: String,
    /// Every further member of a `.step` sweep or Monte Carlo study the bundle holds, in family
    /// order: `dataset_member` is the first.
    #[serde(default)]
    family: Vec<NativeBundleFamilyEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeBundleFamilyEntry {
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
    #[serde(default)]
    member: Option<NativeBundleMember>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeCoordinate {
    name: String,
    #[serde(default)]
    unit: Option<String>,
    #[serde(deserialize_with = "json_samples::coordinates")]
    values: Vec<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeSignal {
    name: String,
    #[serde(default)]
    unit: Option<String>,
    #[serde(default, deserialize_with = "json_samples::optional_signal")]
    values: Option<Vec<f64>>,
    #[serde(default, deserialize_with = "json_samples::optional_signal")]
    real: Option<Vec<f64>>,
    #[serde(default, deserialize_with = "json_samples::optional_signal")]
    imag: Option<Vec<f64>>,
}

/// Verify the bounded container and digest before decoding the versioned file fields.
/// Numeric source spellings are checked before conversion: nonzero decimals
/// cannot underflow to zero, and integer literals must be exactly representable
/// in binary64. Explicit null signal samples remain unavailable.
pub fn decode_native_bundle(
    bytes: &[u8],
    kind: NativeBundleKind,
    limits: NativeBundleReadLimits,
) -> Result<DecodedNumericDataset, NativeBundleError> {
    decode_native_bundle_with_member(bytes, kind, limits).map(|(dataset, _)| dataset)
}

/// [`decode_native_bundle`], with the member of a `.step` sweep or Monte Carlo study the dataset
/// is, when the bundle names one.
pub fn decode_native_bundle_with_member(
    bytes: &[u8],
    kind: NativeBundleKind,
    limits: NativeBundleReadLimits,
) -> Result<(DecodedNumericDataset, Option<NativeBundleMember>), NativeBundleError> {
    read_native_bundle(bytes, kind, limits).map(|read| (read.dataset, read.member))
}

/// A bundle, read.
#[derive(Debug)]
pub struct NativeBundleRead {
    pub dataset: DecodedNumericDataset,
    /// The member of a `.step` sweep or Monte Carlo study the dataset is, when the bundle names
    /// one.
    pub member: Option<NativeBundleMember>,
    /// Every further member of the family a bundle of every member holds, in family order.
    pub family: Vec<NativeBundleFamilyRead>,
    /// The archive's members besides the manifest and the datasets, in name order: what a later
    /// version wrote that this one does not read.
    pub unread_members: Vec<String>,
}

/// One further member of a bundle of every member of a family.
#[derive(Debug)]
pub struct NativeBundleFamilyRead {
    pub dataset: DecodedNumericDataset,
    pub member: Option<NativeBundleMember>,
}

/// [`decode_native_bundle_with_member`], with the members the bundle holds that this version
/// does not read, so the import can say they were left out.
pub fn read_native_bundle(
    bytes: &[u8],
    kind: NativeBundleKind,
    limits: NativeBundleReadLimits,
) -> Result<NativeBundleRead, NativeBundleError> {
    let max_members = limits.max_members;
    let max_expanded_bytes = limits.max_expanded_bytes;
    let mut archive =
        crate::zip::reader::open_unique_archive(bytes, max_members).map_err(|error| {
            use crate::zip::reader::ArchiveReadError;
            match error {
                ArchiveReadError::Zip(source) => NativeBundleError::Zip {
                    context: "the bundle is not a readable ZIP archive".into(),
                    source,
                },
                ArchiveReadError::MemberCount { members, limit } => NativeBundleError::InvalidData(
                    format!("archive has {members} members; the limit is {limit}"),
                ),
                ArchiveReadError::DuplicateMember(name) => {
                    NativeBundleError::InvalidData(format!("archive repeats member '{name}'"))
                }
            }
        })?;
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
        let name = file
            .name()
            .map_err(|error| NativeBundleError::Zip {
                context: format!("invalid member {index} name"),
                source: error,
            })?
            .into_owned();
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
    if manifest.dataset_member != "dataset.json"
        || manifest
            .family
            .iter()
            .any(|entry| entry.dataset_member == "dataset.json")
    {
        return Err(NativeBundleError::InvalidData(
            "manifest must bind the canonical dataset.json member".into(),
        ));
    }
    let mut unread_members = names
        .into_iter()
        .filter(|name| {
            name != "manifest.json"
                && *name != manifest.dataset_member
                && !manifest
                    .family
                    .iter()
                    .any(|entry| entry.dataset_member == *name)
        })
        .collect::<Vec<_>>();
    unread_members.sort();
    let (dataset, member) = read_dataset(
        &mut archive,
        &manifest.dataset_member,
        &manifest.dataset_sha256,
        limits,
    )?;
    let mut family = Vec::with_capacity(manifest.family.len());
    for entry in &manifest.family {
        let (dataset, member) = read_dataset(
            &mut archive,
            &entry.dataset_member,
            &entry.dataset_sha256,
            limits,
        )?;
        family.push(NativeBundleFamilyRead { dataset, member });
    }
    Ok(NativeBundleRead {
        dataset,
        member,
        family,
        unread_members,
    })
}

/// The dataset `name` of `archive`, checked against the digest the manifest binds it to.
fn read_dataset(
    archive: &mut zip::ZipArchive<Cursor<&[u8]>>,
    name: &str,
    sha256: &str,
    limits: NativeBundleReadLimits,
) -> Result<(DecodedNumericDataset, Option<NativeBundleMember>), NativeBundleError> {
    if name.starts_with('/') || name.contains("..") || name.contains('\\') {
        return Err(NativeBundleError::InvalidData(format!(
            "unsafe or unsupported archive member '{name}'"
        )));
    }
    let dataset_bytes = read_zip_member(archive, name, limits.max_member_bytes)?;
    use sha2::Digest as _;
    let digest = format!("{:x}", sha2::Sha256::digest(&dataset_bytes));
    if !sha256.eq_ignore_ascii_case(&digest) {
        return Err(NativeBundleError::InvalidData(format!(
            "{name} SHA-256 does not match the manifest's dataset_sha256"
        )));
    }
    admission::check(&dataset_bytes, limits)?;
    let dataset: NativeDataset =
        serde_json::from_slice(&dataset_bytes).map_err(|error| NativeBundleError::Json {
            context: "dataset.json is invalid",
            source: error,
        })?;
    if !matches!(
        dataset.schema.as_str(),
        "rspice-waveform-dataset/1"
            | "rspice-waveform-dataset/2"
            | "rspice-waveform-dataset/3"
            | "rspice-waveform-dataset/4"
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
    if let Some(member) = &dataset.member {
        if dataset.schema != "rspice-waveform-dataset/4" {
            return Err(NativeBundleError::InvalidData(
                "a .step or Monte Carlo member requires rspice-waveform-dataset/4".into(),
            ));
        }
        member.validate().map_err(NativeBundleError::InvalidData)?;
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
        if real.len() != dataset.coordinate.values.len()
            || imag.as_ref().is_some_and(|imag| imag.len() != real.len())
        {
            return Err(NativeBundleError::InvalidData(format!(
                "signal '{}' sample counts do not match the coordinate",
                signal.name
            )));
        }
        if imag.as_ref().is_some_and(|imag| {
            real.iter()
                .zip(imag)
                .any(|(real, imag)| real.is_nan() != imag.is_nan())
        }) {
            return Err(NativeBundleError::InvalidData(format!(
                "signal '{}' has inconsistent complex sample availability",
                signal.name
            )));
        }
        if matches!(
            dataset.schema.as_str(),
            "rspice-waveform-dataset/1" | "rspice-waveform-dataset/2"
        ) && real.iter().any(|value| value.is_nan())
        {
            return Err(NativeBundleError::InvalidData(
                "unavailable samples require rspice-waveform-dataset/3".into(),
            ));
        }
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
    Ok((decoded, dataset.member))
}

pub(super) fn read_zip_member(
    archive: &mut zip::ZipArchive<Cursor<&[u8]>>,
    name: &str,
    max_member_bytes: u64,
) -> Result<Vec<u8>, NativeBundleError> {
    let file = archive
        .by_name(name)
        .map_err(|error| NativeBundleError::Zip {
            context: format!("the bundle holds no {name}"),
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
