//! Deterministic native RSpice waveform bundle publication.
//!
//! Both public bundle identities deliberately share one embedded waveform
//! dataset schema. The manifest schema distinguishes the artifact contract;
//! its digest binds the exact canonical `dataset.json` bytes.

use serde::Serialize;
use sha2::Digest as _;
use std::collections::HashSet;
use std::io::{Cursor, Write as _};

use super::result_import_workflow::MAX_RESULT_DATASET_BYTES;

const DATASET_SCHEMA: &str = "rspice-waveform-dataset/1";
const DATASET_MEMBER: &str = "dataset.json";
const MANIFEST_MEMBER: &str = "manifest.json";
const MAX_COLUMNS: usize = 1_024;
const MAX_ROWS: usize = 1_000_000;
const MAX_VALUES: usize = MAX_RESULT_DATASET_BYTES as usize / std::mem::size_of::<f64>();
const MAX_NAME_BYTES: usize = 1_024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NativeBundleKind {
    Result,
    Dataset,
}

impl NativeBundleKind {
    pub(crate) const fn manifest_schema(self) -> &'static str {
        match self {
            Self::Result => "rspice-result-bundle/1",
            Self::Dataset => "rspice-dataset-bundle/1",
        }
    }

    pub(crate) const fn extension(self) -> &'static str {
        match self {
            Self::Result => "rspiceresult",
            Self::Dataset => "rspicedata",
        }
    }

    pub(crate) const fn media_type(self) -> &'static str {
        match self {
            Self::Result => "application/vnd.rspice.result+zip",
            Self::Dataset => "application/vnd.rspice.dataset+zip",
        }
    }

    pub(crate) const fn display_name(self) -> &'static str {
        match self {
            Self::Result => "RSpice Result Bundle",
            Self::Dataset => "RSpice Dataset Bundle",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NativeBundleAnalysis {
    Transient,
    Ac,
    DcSweep,
}

impl NativeBundleAnalysis {
    const fn schema_name(self) -> &'static str {
        match self {
            Self::Transient => "transient",
            Self::Ac => "ac",
            Self::DcSweep => "dc_sweep",
        }
    }
}

#[derive(Debug)]
pub(crate) struct NativeBundleDataset<'a> {
    pub(crate) analysis: NativeBundleAnalysis,
    pub(crate) coordinate_name: &'a str,
    pub(crate) coordinate: &'a [f64],
    pub(crate) signals: Vec<NativeBundleSignal<'a>>,
}

#[derive(Debug)]
pub(crate) struct NativeBundleSignal<'a> {
    pub(crate) name: &'a str,
    pub(crate) unit: Option<&'a str>,
    pub(crate) values: NativeBundleSignalValues<'a>,
}

#[derive(Debug)]
pub(crate) enum NativeBundleSignalValues<'a> {
    Real(&'a [f64]),
    Complex { real: &'a [f64], imag: &'a [f64] },
}

#[derive(Serialize)]
struct Manifest<'a> {
    schema: &'a str,
    dataset_member: &'static str,
    dataset_sha256: String,
}

#[derive(Serialize)]
struct DatasetDocument<'a> {
    schema: &'static str,
    analysis: &'static str,
    coordinate: CoordinateDocument<'a>,
    signals: Vec<SignalDocument<'a>>,
}

#[derive(Serialize)]
struct CoordinateDocument<'a> {
    name: &'a str,
    values: &'a [f64],
}

#[derive(Serialize)]
#[serde(untagged)]
enum SignalDocument<'a> {
    Real {
        name: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        unit: Option<&'a str>,
        values: &'a [f64],
    },
    Complex {
        name: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        unit: Option<&'a str>,
        real: &'a [f64],
        imag: &'a [f64],
    },
}

/// Encode a deterministic, self-verifying native bundle.
pub(crate) fn encode_native_bundle(
    kind: NativeBundleKind,
    dataset: &NativeBundleDataset<'_>,
) -> Result<Vec<u8>, String> {
    validate_dataset(dataset)?;
    let signals = dataset
        .signals
        .iter()
        .map(|signal| match signal.values {
            NativeBundleSignalValues::Real(values) => SignalDocument::Real {
                name: signal.name,
                unit: signal.unit,
                values,
            },
            NativeBundleSignalValues::Complex { real, imag } => SignalDocument::Complex {
                name: signal.name,
                unit: signal.unit,
                real,
                imag,
            },
        })
        .collect();
    let document = DatasetDocument {
        schema: DATASET_SCHEMA,
        analysis: dataset.analysis.schema_name(),
        coordinate: CoordinateDocument {
            name: dataset.coordinate_name,
            values: dataset.coordinate,
        },
        signals,
    };
    let dataset_bytes = serde_json::to_vec(&document)
        .map_err(|error| format!("native dataset serialization failed: {error}"))?;
    if dataset_bytes.len() as u64 > MAX_RESULT_DATASET_BYTES {
        return Err(format!(
            "native dataset JSON is {} bytes; the limit is {MAX_RESULT_DATASET_BYTES}",
            dataset_bytes.len()
        ));
    }
    let manifest = Manifest {
        schema: kind.manifest_schema(),
        dataset_member: DATASET_MEMBER,
        dataset_sha256: format!("{:x}", sha2::Sha256::digest(&dataset_bytes)),
    };
    let manifest_bytes = serde_json::to_vec(&manifest)
        .map_err(|error| format!("native manifest serialization failed: {error}"))?;

    let cursor = Cursor::new(Vec::new());
    let mut archive = zip::ZipWriter::new(cursor);
    // Stored members and ZIP's fixed default timestamp make publication
    // byte-for-byte deterministic across repeated encodes of the same data.
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o644);
    archive
        .start_file(MANIFEST_MEMBER, options)
        .map_err(|error| format!("could not begin native manifest member: {error}"))?;
    archive
        .write_all(&manifest_bytes)
        .map_err(|error| format!("could not write native manifest member: {error}"))?;
    archive
        .start_file(DATASET_MEMBER, options)
        .map_err(|error| format!("could not begin native dataset member: {error}"))?;
    archive
        .write_all(&dataset_bytes)
        .map_err(|error| format!("could not write native dataset member: {error}"))?;
    let bytes = archive
        .finish()
        .map_err(|error| format!("could not finish native bundle: {error}"))?
        .into_inner();
    if bytes.len() as u64 > MAX_RESULT_DATASET_BYTES {
        return Err(format!(
            "native bundle is {} bytes; the import limit is {MAX_RESULT_DATASET_BYTES}",
            bytes.len()
        ));
    }
    Ok(bytes)
}

fn validate_dataset(dataset: &NativeBundleDataset<'_>) -> Result<(), String> {
    validate_identity("coordinate", dataset.coordinate_name)?;
    if !(2..=MAX_ROWS).contains(&dataset.coordinate.len()) {
        return Err(format!(
            "native bundle coordinate has {} samples; supported range is 2..={MAX_ROWS}",
            dataset.coordinate.len()
        ));
    }
    if dataset.signals.is_empty() || dataset.signals.len() + 1 > MAX_COLUMNS {
        return Err(format!(
            "native bundle has {} signals; supported range is 1..={}",
            dataset.signals.len(),
            MAX_COLUMNS - 1
        ));
    }
    validate_coordinate(dataset.analysis, dataset.coordinate)?;

    let mut identities = HashSet::with_capacity(dataset.signals.len());
    let mut value_count = dataset.coordinate.len();
    for signal in &dataset.signals {
        validate_identity("signal", signal.name)?;
        if !identities.insert(signal.name.to_lowercase()) {
            return Err(format!(
                "native bundle repeats signal identity '{}'",
                signal.name
            ));
        }
        if signal
            .unit
            .is_some_and(|unit| unit.len() > MAX_NAME_BYTES || unit.chars().any(char::is_control))
        {
            return Err(format!(
                "native bundle signal '{}' has an invalid unit",
                signal.name
            ));
        }
        match signal.values {
            NativeBundleSignalValues::Real(values) => {
                validate_component(signal.name, "values", values, dataset.coordinate.len())?;
                value_count = value_count.checked_add(values.len()).ok_or_else(|| {
                    "native bundle numeric-value accounting overflowed".to_owned()
                })?;
            }
            NativeBundleSignalValues::Complex { real, imag } => {
                validate_component(signal.name, "real", real, dataset.coordinate.len())?;
                validate_component(signal.name, "imag", imag, dataset.coordinate.len())?;
                value_count = value_count
                    .checked_add(real.len())
                    .and_then(|count| count.checked_add(imag.len()))
                    .ok_or_else(|| {
                        "native bundle numeric-value accounting overflowed".to_owned()
                    })?;
            }
        }
        if value_count > MAX_VALUES {
            return Err(format!(
                "native bundle retains {value_count} numeric values; the limit is {MAX_VALUES}"
            ));
        }
    }
    Ok(())
}

fn validate_identity(kind: &str, identity: &str) -> Result<(), String> {
    if identity.trim().is_empty()
        || identity.len() > MAX_NAME_BYTES
        || identity.chars().any(char::is_control)
    {
        Err(format!("native bundle {kind} identity is empty or invalid"))
    } else {
        Ok(())
    }
}

fn validate_component(
    signal: &str,
    component: &str,
    values: &[f64],
    expected_len: usize,
) -> Result<(), String> {
    if values.len() != expected_len {
        return Err(format!(
            "native bundle signal '{signal}' has {} {component} samples; expected {expected_len}",
            values.len()
        ));
    }
    if let Some(index) = values.iter().position(|value| !value.is_finite()) {
        return Err(format!(
            "native bundle signal '{signal}' has a non-finite {component} sample at index {index}"
        ));
    }
    Ok(())
}

fn validate_coordinate(analysis: NativeBundleAnalysis, coordinate: &[f64]) -> Result<(), String> {
    if let Some(index) = coordinate.iter().position(|value| !value.is_finite()) {
        return Err(format!(
            "native bundle coordinate has a non-finite sample at index {index}"
        ));
    }
    let mut direction = None;
    for (index, pair) in coordinate.windows(2).enumerate() {
        let step = pair[1].total_cmp(&pair[0]);
        if step.is_eq() {
            return Err(format!(
                "native bundle coordinate repeats at sample {}",
                index + 1
            ));
        }
        if let Some(expected) = direction {
            if step != expected {
                return Err(format!(
                    "native bundle coordinate reverses at sample {}",
                    index + 1
                ));
            }
        } else {
            direction = Some(step);
        }
    }
    if analysis == NativeBundleAnalysis::Ac && coordinate.iter().any(|value| *value <= 0.0) {
        return Err("native AC bundle requires positive frequency coordinates".to_owned());
    }
    Ok(())
}
