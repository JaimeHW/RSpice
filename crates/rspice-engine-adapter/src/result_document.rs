//! Versioned, typed analog result document emitted by the engine adapter.
//!
//! The CSV artifacts remain a compatibility view. This document is the
//! lossless machine contract: it names the analysis and directive, carries
//! units and ownership, preserves complex values, and represents a projected
//! or unavailable sample as JSON `null` instead of silently omitting it.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Stable schema identity, independent of its version.
pub const RESULT_DOCUMENT_SCHEMA: &str = "rspice-analog-result";
/// The only schema version this build can write or read.
pub const RESULT_DOCUMENT_VERSION: u32 = 1;
/// MIME type declared for version-1 result artifacts.
pub const RESULT_DOCUMENT_CONTENT_TYPE: &str =
    "application/vnd.rspice.analog-result+json;version=1";

/// One executed analog directive and all data retained for it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalogResultDocument {
    pub schema: String,
    pub schema_version: u32,
    pub analysis: AnalysisIdentity,
    pub point_count: usize,
    pub axes: Vec<AxisDocument>,
    pub signals: Vec<SignalDocument>,
    pub device_states: Vec<DeviceStateSeries>,
}

impl AnalogResultDocument {
    /// Construct a version-1 document with a stable one-based directive ID.
    pub fn new(kind: AnalogAnalysisKind, request_kind: &str, ordinal: usize) -> Self {
        Self {
            schema: RESULT_DOCUMENT_SCHEMA.to_owned(),
            schema_version: RESULT_DOCUMENT_VERSION,
            analysis: AnalysisIdentity {
                id: format!("{}-{ordinal:03}", kind.id_prefix()),
                kind,
                request_kind: request_kind.to_owned(),
                ordinal,
            },
            point_count: 0,
            axes: Vec::new(),
            signals: Vec::new(),
            device_states: Vec::new(),
        }
    }

    /// Validate and serialize a document written by this build.
    pub fn to_json(&self) -> Result<String, ResultDocumentError> {
        self.validate()?;
        serde_json::to_string_pretty(self).map_err(ResultDocumentError::InvalidJson)
    }

    /// Read the current schema and reject future versions before attempting
    /// strict decoding. This makes forward incompatibility explicit even if a
    /// future writer also added fields that version 1 does not recognize.
    pub fn from_json(json: &str) -> Result<Self, ResultDocumentError> {
        #[derive(Deserialize)]
        struct Header {
            schema: String,
            schema_version: u32,
        }

        let header: Header =
            serde_json::from_str(json).map_err(ResultDocumentError::InvalidJson)?;
        if header.schema != RESULT_DOCUMENT_SCHEMA {
            return Err(ResultDocumentError::WrongSchema(header.schema));
        }
        if header.schema_version != RESULT_DOCUMENT_VERSION {
            return Err(ResultDocumentError::UnsupportedVersion {
                found: header.schema_version,
                current: RESULT_DOCUMENT_VERSION,
            });
        }
        let document: Self =
            serde_json::from_str(json).map_err(ResultDocumentError::InvalidJson)?;
        document.validate()?;
        Ok(document)
    }

    /// Enforce shape, identity, finiteness, and the analysis/value-type rules
    /// that readers otherwise have to guess.
    pub fn validate(&self) -> Result<(), ResultDocumentError> {
        if self.schema != RESULT_DOCUMENT_SCHEMA {
            return Err(ResultDocumentError::WrongSchema(self.schema.clone()));
        }
        if self.schema_version != RESULT_DOCUMENT_VERSION {
            return Err(ResultDocumentError::UnsupportedVersion {
                found: self.schema_version,
                current: RESULT_DOCUMENT_VERSION,
            });
        }
        if self.analysis.ordinal == 0
            || self.analysis.id
                != format!(
                    "{}-{:03}",
                    self.analysis.kind.id_prefix(),
                    self.analysis.ordinal
                )
        {
            return Err(invalid("analysis ID does not match its kind and ordinal"));
        }
        if self.analysis.request_kind.trim().is_empty() || self.point_count == 0 {
            return Err(invalid(
                "analysis request kind and a nonzero point count are required",
            ));
        }
        let expected_axes = match self.analysis.kind {
            AnalogAnalysisKind::OperatingPoint => 0..=0,
            AnalogAnalysisKind::DcSweep => 1..=2,
            AnalogAnalysisKind::AcSmallSignal
            | AnalogAnalysisKind::Transient
            | AnalogAnalysisKind::Noise => 1..=1,
        };
        if !expected_axes.contains(&self.axes.len()) {
            return Err(invalid("axis count is not valid for the analysis kind"));
        }

        let mut names = HashSet::new();
        for axis in &self.axes {
            if axis.name.trim().is_empty()
                || axis.values.len() != self.point_count
                || !finite_real(&axis.values)
                || !names.insert(format!("axis:{}", axis.name.to_ascii_lowercase()))
            {
                return Err(invalid("axis name, shape, or value is invalid"));
            }
        }
        for signal in &self.signals {
            if signal.canonical_name.trim().is_empty()
                || signal.display_name.trim().is_empty()
                || !names.insert(format!(
                    "signal:{}",
                    signal.canonical_name.to_ascii_lowercase()
                ))
                || signal.values.len() != self.point_count
                || !signal.values.is_finite()
            {
                return Err(invalid("signal name, shape, or value is invalid"));
            }
            match signal.kind {
                AnalogSignalKind::Voltage if signal.unit != Some(SignalUnit::Volt) => {
                    return Err(invalid("voltage signals must be measured in volts"));
                }
                AnalogSignalKind::BranchCurrent if signal.unit != Some(SignalUnit::Ampere) => {
                    return Err(invalid(
                        "branch-current signals must be measured in amperes",
                    ));
                }
                _ => {}
            }
            let expects_complex = matches!(
                self.analysis.kind,
                AnalogAnalysisKind::AcSmallSignal | AnalogAnalysisKind::Noise
            ) && matches!(
                signal.kind,
                AnalogSignalKind::Voltage | AnalogSignalKind::BranchCurrent
            );
            if expects_complex != matches!(signal.values, SignalValues::Complex { .. }) {
                return Err(invalid(
                    "signal value representation is not valid for the analysis kind",
                ));
            }
        }
        for state in &self.device_states {
            if state.device_name.trim().is_empty()
                || state.regions.len() != self.point_count
                || !names.insert(format!("state:{}", state.device_name.to_ascii_lowercase()))
            {
                return Err(invalid("device-state name or shape is invalid"));
            }
        }
        Ok(())
    }
}

fn invalid(message: &str) -> ResultDocumentError {
    ResultDocumentError::InvalidDocument(message.to_owned())
}

fn finite_real(values: &[Option<f64>]) -> bool {
    values.iter().flatten().all(|value| value.is_finite())
}

/// Stable identity of the executed analysis directive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisIdentity {
    pub id: String,
    pub kind: AnalogAnalysisKind,
    pub request_kind: String,
    /// One-based ordinal within matching directives in the source deck.
    pub ordinal: usize,
}

/// Analog analysis classes executable by this adapter revision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalogAnalysisKind {
    OperatingPoint,
    DcSweep,
    AcSmallSignal,
    Transient,
    Noise,
}

impl AnalogAnalysisKind {
    const fn id_prefix(self) -> &'static str {
        match self {
            Self::OperatingPoint => "op",
            Self::DcSweep => "dc",
            Self::AcSmallSignal => "ac",
            Self::Transient => "tran",
            Self::Noise => "noise",
        }
    }
}

/// One independent coordinate shared by all signals in the document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AxisDocument {
    pub name: String,
    /// `None` means the producing engine has no sound physical-unit metadata.
    pub unit: Option<SignalUnit>,
    pub values: Vec<Option<f64>>,
}

/// One typed result signal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignalDocument {
    pub canonical_name: String,
    pub display_name: String,
    pub kind: AnalogSignalKind,
    pub owner: SignalOwner,
    /// `None` is explicit missing unit metadata, not a dimensionless claim.
    pub unit: Option<SignalUnit>,
    pub values: SignalValues,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalogSignalKind {
    Voltage,
    BranchCurrent,
    DeviceObservable,
    Scalar,
}

/// Entity that owns a result signal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SignalOwner {
    Node {
        name: String,
    },
    Branch {
        name: String,
    },
    Device {
        device: Option<String>,
        parameter: Option<String>,
        device_kind: Option<String>,
    },
    Analysis,
}

/// Physical-unit vocabulary understood by result readers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalUnit {
    Volt,
    Ampere,
    Second,
    Hertz,
    VoltSquaredPerHertz,
    Dimensionless,
    Ohm,
    Siemens,
    Watt,
    Farad,
    Henry,
    Coulomb,
    Meter,
    DegreeCelsius,
}

/// Real or complex samples. Every entry may be explicitly unavailable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "representation", rename_all = "snake_case")]
pub enum SignalValues {
    Real { samples: Vec<Option<f64>> },
    Complex { samples: Vec<Option<ComplexSample>> },
}

impl SignalValues {
    fn len(&self) -> usize {
        match self {
            Self::Real { samples } => samples.len(),
            Self::Complex { samples } => samples.len(),
        }
    }

    fn is_finite(&self) -> bool {
        match self {
            Self::Real { samples } => finite_real(samples),
            Self::Complex { samples } => samples
                .iter()
                .flatten()
                .all(|sample| sample.real.is_finite() && sample.imaginary.is_finite()),
        }
    }
}

/// JSON-safe representation of one complex sample.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComplexSample {
    pub real: f64,
    pub imaginary: f64,
}

/// Per-point operating region for a device. `None` preserves the distinction
/// between "the device has no region" and dropping device state entirely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeviceStateSeries {
    pub device_name: String,
    pub device_kind: Option<String>,
    pub regions: Vec<Option<String>>,
}

#[derive(Debug, Error)]
pub enum ResultDocumentError {
    #[error("invalid result JSON: {0}")]
    InvalidJson(#[source] serde_json::Error),
    #[error("unexpected result schema {0:?}")]
    WrongSchema(String),
    #[error("result schema version {found} is unsupported (current version is {current})")]
    UnsupportedVersion { found: u32, current: u32 },
    #[error("invalid result document: {0}")]
    InvalidDocument(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_version_round_trips_real_complex_and_missing_samples() {
        let mut document =
            AnalogResultDocument::new(AnalogAnalysisKind::AcSmallSignal, "ac_small_signal", 2);
        document.point_count = 2;
        document.axes.push(AxisDocument {
            name: "frequency".to_owned(),
            unit: Some(SignalUnit::Hertz),
            values: vec![Some(1.0), Some(10.0)],
        });
        document.signals.push(SignalDocument {
            canonical_name: "v(out)".to_owned(),
            display_name: "V(out)".to_owned(),
            kind: AnalogSignalKind::Voltage,
            owner: SignalOwner::Node {
                name: "out".to_owned(),
            },
            unit: Some(SignalUnit::Volt),
            values: SignalValues::Complex {
                samples: vec![
                    Some(ComplexSample {
                        real: 1.0,
                        imaginary: -0.25,
                    }),
                    None,
                ],
            },
        });
        document.signals.push(SignalDocument {
            canonical_name: "gain_squared".to_owned(),
            display_name: "gain squared".to_owned(),
            kind: AnalogSignalKind::Scalar,
            owner: SignalOwner::Analysis,
            unit: Some(SignalUnit::Dimensionless),
            values: SignalValues::Real {
                samples: vec![Some(1.0), None],
            },
        });

        let json = document.to_json().expect("serialize current schema");
        assert_eq!(AnalogResultDocument::from_json(&json).unwrap(), document);
    }

    #[test]
    fn future_version_is_rejected_before_unknown_fields_are_decoded() {
        let json = r#"{
            "schema":"rspice-analog-result",
            "schema_version":2,
            "new_required_contract":"future"
        }"#;
        assert!(matches!(
            AnalogResultDocument::from_json(json),
            Err(ResultDocumentError::UnsupportedVersion {
                found: 2,
                current: 1
            })
        ));
    }
}
