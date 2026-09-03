//! One target-neutral typed result document for every core result family.
//!
//! The CLI, Python, WebAssembly, and engine-adapter surfaces format this
//! document. They do not define their own. A surface that needs a different
//! shape formats *from* this one, so a channel, unit, validity mask, or
//! provenance field cannot be lost by being invented twice.
//!
//! # JSON schema
//!
//! The encoding is a single JSON object. Keys are `camelCase`; every object in
//! the document rejects unknown fields, and decoding rejects a
//! `schemaVersion` this build does not implement *before* any field is
//! decoded. Every field below is required unless it is described as nullable.
//!
//! ```text
//! {
//!   "schema":        "rspice-analysis-result"   fixed identifier
//!   "schemaVersion": 1                          this build's exact version
//!   "resultKind":    "op" | "dc" | "ac" | "tran" | "noise" | "sp" |
//!                    "port-noise" | "distortion" | "tf" | "stb" |
//!                    "sensitivity" | "pole-zero" | "fourier" | "fft" |
//!                    "monte-carlo" | "pss" | "pac" | "pnoise" | "hb" |
//!                    "envelope"
//!   "analysis":      { "kind": <analysis tag>, "ordinal": <u32, 0-based>,
//!                      "tag": "<kind>-<ordinal+1, 3 digits>" }
//!   "parentAnalysis": same shape or null. Required for "fft" and "fourier",
//!                     which point at the transient they post-processed;
//!                     optional for "pac"/"pnoise" (their PSS) and
//!                     "envelope" (its HB carrier); null everywhere else.
//!   "coordinate":    null, or
//!                    { "id": { "semantic": <32 lower-case hex chars>,
//!                              "occurrence": <u32, 0-based> },
//!                      "ordinal": <usize>, "label": <non-empty string>,
//!                      "assignments": [ { "kind": "alter"|"data"|"step"|
//!                                                 "temperature",
//!                                         "name": <axis name>,
//!                                         "valueIndex": <usize>,
//!                                         "value": <run-axis value>,
//!                                         "stepTarget": <step target|null> } ] }
//!   "topologyFingerprint": null, or 64 lower-case hex characters
//!   "namespaces":    null, or { "output": <string>, "checkpoint": <string> }
//!   "pointCount":    <usize>  length of every axis and signal series; 0 for
//!                             families whose data is scalars and payload only
//!   "axes":          [ { "name", "displayName",
//!                        "kind": "time"|"frequency"|"offset-frequency"|
//!                                "sweep-value"|"temperature"|"trial-index"|
//!                                "harmonic-index"|"sideband"|"bin-index"|
//!                                "port-index"|"phase"|"index",
//!                        "unit": <signal unit>,
//!                        "values": { "representation": "real",    "values": [f64] }
//!                               or { "representation": "integer", "values": [i64] } } ]
//!   "signals":       [ { "descriptor": { "canonicalName", "displayName",
//!                                        "kind", "unit", "valueType",
//!                                        "shape", "owner" },
//!                        "qualifier": null | { "kind": "distortion-fundamental",
//!                                              "tone": "f1"|"f2" }
//!                                          | { "kind": "distortion-product",
//!                                              "product": <product tag> }
//!                                          | { "kind": "pac-sideband",
//!                                              "sideband": <i32> },
//!                        "availability": "available" | "not-projected"
//!                                      | "absent-at-coordinate",
//!                        "values": { "representation": "real",
//!                                    "samples": [f64|null] }
//!                                or { "representation": "complex",
//!                                     "samples": [{"real","imaginary"}|null] }
//!                                or { "representation": "logic",
//!                                     "samples": [{"state","strength"}|null] } } ]
//!   "scalars":       [ { "name", "displayName", "unit": <signal unit|null>,
//!                        "value": { "representation": "real",    "value": f64|null }
//!                               or { "representation": "complex", "value": {...}|null }
//!                               or { "representation": "integer", "value": i64 }
//!                               or { "representation": "count",   "value": u64 }
//!                               or { "representation": "boolean", "value": bool }
//!                               or { "representation": "text",    "value": string } } ]
//!   "deviceStates":  [ { "deviceName", "deviceKind": <string|null>,
//!                        "regions": [] or [string|null] one per point,
//!                        "parameters": [ { "name", "unit": <unit|null>,
//!                                          "values": [f64|null] } ] } ]
//!   "payload":       { "family": <result kind tag>, ... }  see below
//! }
//! ```
//!
//! A signal unit is `{"unit": "volt"|"ampere"|"ohm"|"siemens"|"watt"|"hertz"|
//! "second"|"degree"|"radian"|"dimensionless"|"logic"}` or
//! `{"unit": "custom", "symbol": "<text>"}`.
//!
//! A run-axis value is `{"kind":"numeric","value":f64}`,
//! `{"kind":"data_row","bindings":[{"name","value"}]}`, or
//! `{"kind":"alter_variant","label","materialization_digest":<64 hex>}`.
//!
//! # Payload
//!
//! `payload.family` repeats `resultKind`, and decoding rejects a document
//! whose two disagree. The remaining fields are family-specific:
//!
//! ```text
//! op          observables[]                      name, unit|null, value|null
//! dc          sweepVariable, observables[]        one value per sweep point
//! ac          (no further fields)
//! tran        stepSizes[], storeTraces[],
//!             digitalTraces[], realTraces[],
//!             fftChildren[], compression|null     analysis + outputName per child
//! noise       contributionCatalog[],
//!             mechanismsUnavailable[],
//!             contributions[]                     identity, mechanismKind, and
//!                                                 output/input/percentage series
//! sp          referenceImpedance, ports[],
//!             angularFrequencies[]                ports: number, +node, -node, z0
//! port-noise  portCount
//! distortion  f2OverF1|null, products[]           product tag, order, frequencies[]
//! tf          output, input
//! stb         success, warnings[], nyquist[]      frequency, real, imaginary
//! sensitivity output, entries[]                   vectorName, element, elementKind,
//!                                                 parameter, nominalValue,
//!                                                 absolute, normalized
//! pole-zero   input, output, poles[], zeros[],
//!             poleEvidence, zeroEvidence,
//!             dcGain|null, highFrequencyGain|null
//! fourier     output
//! fft         source, outputName, physicalType,
//!             startTime, stopTime, sampleInterval,
//!             sampleCount, accurateSampling,
//!             coefficientFormat, compatibilityMode,
//!             window, windowName, alpha,
//!             coherentGain, frequencyResolution,
//!             fundamentalBin, minimumMetricBin,
//!             maximumMetricBin, metrics|null      metrics carry the ranked,
//!                                                 deliberately ragged harmonics
//! monte-carlo statistics[]                        name, samples[], mean|null,
//!                                                 standardDeviation|null,
//!                                                 minimum|null, maximum|null,
//!                                                 histogram[], binEdges[]
//! pss         floquetMultipliers[], floquetEvidence,
//!             floquetOrbitKind,
//!             trivialFloquetMultiplierIndex|null
//! pac         fundamentalFrequency, sidebandMinimum,
//!             sidebandMaximum, inputSource|null,
//!             outputNode|null, iterations, residual,
//!             sidebands[], conversionMatrix|null
//! pnoise      outputNode, jitterBandwidth|null,
//!             contributors[]                      each keeps its own offset grid
//! hb          tones[], reactiveSpectra[],
//!             continuationLimitations[]
//! envelope    continuation, carrier, transient     transient is the tran payload
//!                                                  of the continued run
//! ```
//!
//! # Missingness
//!
//! A sample that does not exist is `null`, never `0.0`. A whole series that
//! was deliberately not retained keeps its descriptor and unit and sets
//! `availability`; every one of its samples is then `null`. Reading a window
//! yields a parallel `validity` mask so a numeric transfer never has to invent
//! a placeholder meaning.

mod builders;
mod payload;
#[cfg(test)]
mod tests;
mod wire;

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use payload::{
    AcPayload, AcSensitivityEntry, CompressionAlgorithmTag, CompressionObservationDocument,
    CompressionPolicyDocument, CompressionReportDocument, CompressionSampleDomainTag,
    CompressionSignalKindTag, DcSweepAxisDocument, DcSweepPayload, DigitalEventPoint,
    DigitalEventTrace, DigitalStateTag, DigitalStrengthTag, DistortionPayload,
    DistortionProductSeries, DistortionProductTag, DistortionTone, EnvelopeCarrierDocument,
    EnvelopeContinuationDocument, EnvelopeGuaranteeTag, EnvelopeNodeSpectrum, EnvelopePayload,
    FftChildReference, FftCoefficientFormatTag, FftCompatibilityModeTag, FftHarmonicDocument,
    FftMetricsDocument, FftPayload, FftSourceDocument, FftWindowTag, FloquetCertificateDocument,
    FloquetEvidenceDocument, FloquetOrbitTag, FourierPayload, HarmonicBalancePayload,
    HbContinuationLimitationTag, HbReactiveKindTag, HbReactiveSpectrumDocument, MonteCarloPayload,
    MonteCarloVariableStatistics, NamedObservable, NamedObservableSeries, NoiseContributionSeries,
    NoiseMechanismTag, NoisePayload, NoiseSourceIdentityDocument, NyquistSample,
    OperatingPointPayload, OscillatorPhaseNoiseDocument, PNoiseBandwidth, PNoiseContribution,
    PNoiseContributor, PNoisePayload, PacConversionEntry, PacConversionMatrixDocument, PacPayload,
    PacSidebandDescriptor, PoleZeroPayload, PortDocument, PortNoisePayload, PssPayload,
    RealEventPoint, RealEventTrace, ResultPayload, RootSetEvidenceDocument, SParameterPayload,
    SensitivityElementTag, SensitivityEntry, SensitivityPayload, SpectrumCertificateDocument,
    StabilityPayload, TransferFunctionPayload, TransientPayload,
};

use crate::abort_signal::{AbortSignal, NoAbort};
use crate::execution::bounded_io::{BoundedAbortWriter, BoundedWriteFailure};
use crate::execution::capability::{AnalysisResultKind, analysis_result_kind};
use crate::execution::plan::{
    AnalysisInstanceId, AnalysisKind, AxisKind, RunAxisValue, RunCoordinate, RunCoordinateId,
    StepAxisTarget,
};
use crate::execution::schema::{SignalDescriptor, SignalUnit, SignalValueType};
use crate::execution::topology::TopologyFingerprint;

/// Stable schema identifier written into every document.
pub const ANALYSIS_RESULT_DOCUMENT_SCHEMA: &str = "rspice-analysis-result";

/// Schema version this build produces and is the only one it decodes.
pub const ANALYSIS_RESULT_DOCUMENT_VERSION: u32 = 1;

/// How often long validation and serialization loops poll the abort source.
const ABORT_POLL_STRIDE: usize = 256;

//=============================================================================
// Document
//=============================================================================

/// One analysis result with its identity, provenance, schema, and values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AnalysisResultDocument {
    schema: String,
    schema_version: u32,
    #[serde(with = "wire::analysis_result_kind")]
    result_kind: AnalysisResultKind,
    #[serde(with = "wire::analysis_instance_id")]
    analysis: AnalysisInstanceId,
    #[serde(with = "wire::optional_analysis_instance_id")]
    parent_analysis: Option<AnalysisInstanceId>,
    coordinate: Option<ResultCoordinate>,
    #[serde(with = "wire::optional_topology_fingerprint")]
    topology_fingerprint: Option<TopologyFingerprint>,
    namespaces: Option<ResultNamespaces>,
    point_count: usize,
    axes: Vec<ResultAxis>,
    signals: Vec<ResultSignal>,
    scalars: Vec<ResultScalar>,
    device_states: Vec<DeviceStateSeries>,
    payload: ResultPayload,
}

impl AnalysisResultDocument {
    /// Start a document for one analysis instance and result family.
    pub fn builder(
        analysis: AnalysisInstanceId,
        payload: ResultPayload,
        point_count: usize,
    ) -> AnalysisResultDocumentBuilder {
        AnalysisResultDocumentBuilder {
            analysis,
            payload,
            point_count,
            parent_analysis: None,
            coordinate: None,
            topology_fingerprint: None,
            namespaces: None,
            axes: Vec::new(),
            signals: Vec::new(),
            scalars: Vec::new(),
            device_states: Vec::new(),
        }
    }

    /// Schema identifier, always [`ANALYSIS_RESULT_DOCUMENT_SCHEMA`].
    pub fn schema(&self) -> &str {
        &self.schema
    }

    /// Schema version this document declares.
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Result family this document represents.
    pub const fn result_kind(&self) -> AnalysisResultKind {
        self.result_kind
    }

    /// Identity of the authored analysis card that produced this result.
    pub const fn analysis(&self) -> AnalysisInstanceId {
        self.analysis
    }

    /// Identity of the analysis this result was post-processed from.
    pub const fn parent_analysis(&self) -> Option<AnalysisInstanceId> {
        self.parent_analysis
    }

    /// Shared-deck coordinate this result was produced at.
    pub const fn coordinate(&self) -> Option<&ResultCoordinate> {
        self.coordinate.as_ref()
    }

    /// Structural identity of the elaborated topology that was solved.
    pub const fn topology_fingerprint(&self) -> Option<TopologyFingerprint> {
        self.topology_fingerprint
    }

    /// Output and checkpoint namespaces this result was written under.
    pub const fn namespaces(&self) -> Option<&ResultNamespaces> {
        self.namespaces.as_ref()
    }

    /// Length of every axis and signal series in this document.
    pub const fn point_count(&self) -> usize {
        self.point_count
    }

    /// Typed coordinate axes.
    pub fn axes(&self) -> &[ResultAxis] {
        &self.axes
    }

    /// Typed signal series.
    pub fn signals(&self) -> &[ResultSignal] {
        &self.signals
    }

    /// Typed per-analysis scalars.
    pub fn scalars(&self) -> &[ResultScalar] {
        &self.scalars
    }

    /// Per-device operating state and parameter histories.
    pub fn device_states(&self) -> &[DeviceStateSeries] {
        &self.device_states
    }

    /// Family-specific data that does not fit an axis/signal/scalar shape.
    pub const fn payload(&self) -> &ResultPayload {
        &self.payload
    }

    /// Numerical values retained per point across every axis and signal.
    ///
    /// A complex sample counts as two values. Use this to size a window before
    /// requesting one.
    pub fn values_per_point(&self) -> usize {
        let axis_values = self.axes.len();
        let signal_values = self
            .signals
            .iter()
            .map(|signal| signal.values.numeric_columns())
            .fold(0, usize::saturating_add);
        axis_values.saturating_add(signal_values)
    }

    /// Total numerical values this document retains, for resource budgets.
    pub fn total_value_count(&self) -> usize {
        let series = self.values_per_point().saturating_mul(self.point_count);
        let scalars = self.scalars.len();
        let device_states = self
            .device_states
            .iter()
            .map(DeviceStateSeries::value_count)
            .fold(0, usize::saturating_add);
        series
            .saturating_add(scalars)
            .saturating_add(device_states)
            .saturating_add(self.payload.value_count())
    }

    /// Copy a bounded window of every axis and signal.
    ///
    /// The window is `count` points starting at `start`. Missing samples come
    /// back as a zero placeholder paired with a zero validity byte, so a
    /// numeric transport never has to encode absence as a plausible value.
    pub fn window(&self, start: usize, count: usize) -> Result<ResultWindow, ResultDocumentError> {
        let end = start
            .checked_add(count)
            .filter(|end| *end <= self.point_count)
            .ok_or(ResultDocumentError::WindowOutOfBounds {
                start,
                count,
                point_count: self.point_count,
            })?;
        Ok(ResultWindow {
            schema_version: self.schema_version,
            analysis: self.analysis,
            coordinate_id: self.coordinate.as_ref().map(|coordinate| coordinate.id),
            start,
            count,
            point_count: self.point_count,
            // `validate` proves every axis and series is `point_count` long,
            // but a document reached through a deserializer has not been
            // validated yet, so a short column is refused rather than a panic.
            axes: self
                .axes
                .iter()
                .map(|axis| {
                    Some(AxisWindow {
                        name: axis.name.clone(),
                        values: axis.values.slice(start, end)?,
                    })
                })
                .collect::<Option<Vec<_>>>()
                .ok_or(ResultDocumentError::WindowOutOfBounds {
                    start,
                    count,
                    point_count: self.point_count,
                })?,
            signals: self
                .signals
                .iter()
                .map(|signal| {
                    Some(SignalWindow {
                        canonical_name: signal.descriptor.canonical_name().to_owned(),
                        qualifier: signal.qualifier.clone(),
                        values: signal.values.window(start, end)?,
                    })
                })
                .collect::<Option<Vec<_>>>()
                .ok_or(ResultDocumentError::WindowOutOfBounds {
                    start,
                    count,
                    point_count: self.point_count,
                })?,
        })
    }

    /// Validate every invariant this document declares.
    pub fn validate(&self) -> Result<(), ResultDocumentError> {
        self.validate_with_abort(&NoAbort)
    }

    /// Cancellable form of [`Self::validate`].
    pub fn validate_with_abort(&self, abort: &dyn AbortSignal) -> Result<(), ResultDocumentError> {
        check_abort(abort)?;
        if self.schema != ANALYSIS_RESULT_DOCUMENT_SCHEMA {
            return Err(ResultDocumentError::WrongSchema {
                found: self.schema.clone(),
            });
        }
        if self.schema_version != ANALYSIS_RESULT_DOCUMENT_VERSION {
            return Err(ResultDocumentError::UnsupportedVersion {
                found: self.schema_version,
                current: ANALYSIS_RESULT_DOCUMENT_VERSION,
            });
        }
        if self.payload.result_kind() != self.result_kind {
            return Err(ResultDocumentError::PayloadFamilyMismatch {
                declared: self.result_kind,
                payload: self.payload.result_kind(),
            });
        }
        self.validate_identity()?;
        self.validate_series(abort)?;
        self.validate_scalars()?;
        self.validate_device_states(abort)?;
        if let Some(coordinate) = &self.coordinate {
            coordinate.validate()?;
        }
        if let Some(namespaces) = &self.namespaces {
            require_name("output namespace", &namespaces.output)?;
            require_name("checkpoint namespace", &namespaces.checkpoint)?;
        }
        check_abort(abort)?;
        self.payload.validate()?;
        check_abort(abort)
    }

    fn validate_identity(&self) -> Result<(), ResultDocumentError> {
        let declared = analysis_result_kind(self.analysis.kind());
        let compatible = declared == self.result_kind
            || (self.result_kind == AnalysisResultKind::PortNoise
                && self.analysis.kind() == AnalysisKind::Sp);
        if !compatible {
            return Err(ResultDocumentError::AnalysisFamilyMismatch {
                declared: self.result_kind,
                analysis: declared,
            });
        }
        // A periodic small-signal card linearizes around whichever periodic
        // large-signal analysis preceded it. Shooting `.PSS` and harmonic
        // balance both produce that operating point, and the engine runs
        // `.PAC`/`.PNOISE` from either, so both are accepted parents.
        let required_parent: Option<(bool, &'static [AnalysisKind])> = match self.result_kind {
            AnalysisResultKind::Fft | AnalysisResultKind::Fourier => {
                Some((true, &[AnalysisKind::Tran]))
            }
            AnalysisResultKind::Pac | AnalysisResultKind::PNoise => {
                Some((false, &[AnalysisKind::Pss, AnalysisKind::HarmonicBalance]))
            }
            AnalysisResultKind::Envelope => Some((false, &[AnalysisKind::HarmonicBalance])),
            _ => None,
        };
        match (required_parent, self.parent_analysis) {
            (None, None) => Ok(()),
            (None, Some(_)) => Err(ResultDocumentError::UnexpectedParentAnalysis {
                result_kind: self.result_kind,
            }),
            (Some((true, _)), None) => Err(ResultDocumentError::MissingParentAnalysis {
                result_kind: self.result_kind,
            }),
            (Some((false, _)), None) => Ok(()),
            (Some((_, expected)), Some(parent)) => {
                if expected.contains(&parent.kind()) {
                    Ok(())
                } else {
                    Err(ResultDocumentError::WrongParentAnalysis {
                        result_kind: self.result_kind,
                        expected,
                        found: parent.kind(),
                    })
                }
            }
        }
    }

    fn validate_series(&self, abort: &dyn AbortSignal) -> Result<(), ResultDocumentError> {
        if self.point_count == 0 && !(self.axes.is_empty() && self.signals.is_empty()) {
            return Err(ResultDocumentError::Malformed {
                location: "point count",
                detail: "a zero-point document cannot carry axes or signals".to_owned(),
            });
        }
        let mut axis_names = BTreeSet::new();
        for (index, axis) in self.axes.iter().enumerate() {
            if index.is_multiple_of(ABORT_POLL_STRIDE) {
                check_abort(abort)?;
            }
            axis.validate()?;
            if axis.values.len() != self.point_count {
                return Err(ResultDocumentError::SeriesLength {
                    location: format!("axis '{}'", axis.name),
                    expected: self.point_count,
                    actual: axis.values.len(),
                });
            }
            if !axis_names.insert(axis.name.to_ascii_lowercase()) {
                return Err(ResultDocumentError::DuplicateSeries {
                    location: "axis",
                    name: axis.name.clone(),
                });
            }
        }

        let mut signal_identities = BTreeSet::new();
        for (index, signal) in self.signals.iter().enumerate() {
            if index.is_multiple_of(ABORT_POLL_STRIDE) {
                check_abort(abort)?;
            }
            signal.validate()?;
            if signal.values.len() != self.point_count {
                return Err(ResultDocumentError::SeriesLength {
                    location: format!("signal '{}'", signal.descriptor.canonical_name()),
                    expected: self.point_count,
                    actual: signal.values.len(),
                });
            }
            let identity = (
                signal.qualifier.as_ref().map(SeriesQualifier::identity),
                signal.descriptor.canonical_name().to_owned(),
            );
            if !signal_identities.insert(identity) {
                return Err(ResultDocumentError::DuplicateSeries {
                    location: "signal",
                    name: signal.descriptor.canonical_name().to_owned(),
                });
            }
        }
        Ok(())
    }

    fn validate_scalars(&self) -> Result<(), ResultDocumentError> {
        let mut names = BTreeSet::new();
        for scalar in &self.scalars {
            scalar.validate()?;
            if !names.insert(scalar.name.to_ascii_lowercase()) {
                return Err(ResultDocumentError::DuplicateSeries {
                    location: "scalar",
                    name: scalar.name.clone(),
                });
            }
        }
        Ok(())
    }

    fn validate_device_states(&self, abort: &dyn AbortSignal) -> Result<(), ResultDocumentError> {
        let mut names = BTreeSet::new();
        for (index, state) in self.device_states.iter().enumerate() {
            if index.is_multiple_of(ABORT_POLL_STRIDE) {
                check_abort(abort)?;
            }
            state.validate(self.point_count)?;
            if !names.insert(state.device_name.to_ascii_lowercase()) {
                return Err(ResultDocumentError::DuplicateSeries {
                    location: "device state",
                    name: state.device_name.clone(),
                });
            }
        }
        Ok(())
    }

    /// Validate and encode this document as JSON.
    pub fn to_json(&self) -> Result<String, ResultDocumentError> {
        self.to_json_with_abort(&NoAbort, u64::MAX)
    }

    /// Validate and encode this document through a bounded, cancellable sink.
    pub fn to_json_with_abort(
        &self,
        abort: &dyn AbortSignal,
        byte_limit: u64,
    ) -> Result<String, ResultDocumentError> {
        self.validate_with_abort(abort)?;
        let mut writer = BoundedAbortWriter::new(abort, byte_limit);
        if let Err(error) = serde_json::to_writer(&mut writer, self) {
            return Err(match writer.failure() {
                Some(BoundedWriteFailure::Aborted) => ResultDocumentError::Aborted,
                Some(BoundedWriteFailure::ByteLimitExceeded { limit_bytes }) => {
                    ResultDocumentError::ArtifactTooLarge { limit_bytes }
                }
                Some(BoundedWriteFailure::AllocationFailed) => {
                    ResultDocumentError::AllocationFailed
                }
                None => ResultDocumentError::Json(error.to_string()),
            });
        }
        check_abort(abort)?;
        writer
            .into_string()
            .map_err(|error| ResultDocumentError::Json(format!("encoder emitted {error}")))
    }

    /// Decode a document this build's exact schema version produced.
    pub fn from_json(json: &str) -> Result<Self, ResultDocumentError> {
        Self::from_json_with_abort(json, &NoAbort, u64::MAX)
    }

    /// Bounded, cancellable form of [`Self::from_json`].
    ///
    /// The schema identifier and version are checked before any other field is
    /// decoded, so a future document is rejected as a version mismatch rather
    /// than as a pile of unknown-field errors.
    pub fn from_json_with_abort(
        json: &str,
        abort: &dyn AbortSignal,
        byte_limit: u64,
    ) -> Result<Self, ResultDocumentError> {
        check_abort(abort)?;
        if json.len() as u128 > u128::from(byte_limit) {
            return Err(ResultDocumentError::ArtifactTooLarge {
                limit_bytes: byte_limit,
            });
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Header {
            schema: String,
            schema_version: u32,
        }

        let header: Header = serde_json::from_str(json)
            .map_err(|error| ResultDocumentError::Json(error.to_string()))?;
        if header.schema != ANALYSIS_RESULT_DOCUMENT_SCHEMA {
            return Err(ResultDocumentError::WrongSchema {
                found: header.schema,
            });
        }
        if header.schema_version != ANALYSIS_RESULT_DOCUMENT_VERSION {
            return Err(ResultDocumentError::UnsupportedVersion {
                found: header.schema_version,
                current: ANALYSIS_RESULT_DOCUMENT_VERSION,
            });
        }
        check_abort(abort)?;
        let document: Self = serde_json::from_str(json)
            .map_err(|error| ResultDocumentError::Json(error.to_string()))?;
        document.validate_with_abort(abort)?;
        Ok(document)
    }
}

/// Staged, validating constructor for [`AnalysisResultDocument`].
#[derive(Debug, Clone)]
pub struct AnalysisResultDocumentBuilder {
    analysis: AnalysisInstanceId,
    payload: ResultPayload,
    point_count: usize,
    parent_analysis: Option<AnalysisInstanceId>,
    coordinate: Option<ResultCoordinate>,
    topology_fingerprint: Option<TopologyFingerprint>,
    namespaces: Option<ResultNamespaces>,
    axes: Vec<ResultAxis>,
    signals: Vec<ResultSignal>,
    scalars: Vec<ResultScalar>,
    device_states: Vec<DeviceStateSeries>,
}

impl AnalysisResultDocumentBuilder {
    /// Name the analysis this result was post-processed from.
    #[must_use]
    pub fn parent_analysis(mut self, parent: AnalysisInstanceId) -> Self {
        self.parent_analysis = Some(parent);
        self
    }

    /// Attach the shared-deck coordinate this result was produced at.
    #[must_use]
    pub fn coordinate(mut self, coordinate: ResultCoordinate) -> Self {
        self.coordinate = Some(coordinate);
        self
    }

    /// Attach the structural identity of the solved topology.
    #[must_use]
    pub fn topology_fingerprint(mut self, fingerprint: TopologyFingerprint) -> Self {
        self.topology_fingerprint = Some(fingerprint);
        self
    }

    /// Attach the output and checkpoint namespaces.
    #[must_use]
    pub fn namespaces(mut self, namespaces: ResultNamespaces) -> Self {
        self.namespaces = Some(namespaces);
        self
    }

    /// Append one coordinate axis.
    #[must_use]
    pub fn axis(mut self, axis: ResultAxis) -> Self {
        self.axes.push(axis);
        self
    }

    /// Append one signal series.
    #[must_use]
    pub fn signal(mut self, signal: ResultSignal) -> Self {
        self.signals.push(signal);
        self
    }

    /// Append several signal series.
    #[must_use]
    pub fn signals(mut self, signals: impl IntoIterator<Item = ResultSignal>) -> Self {
        self.signals.extend(signals);
        self
    }

    /// Append one typed scalar.
    #[must_use]
    pub fn scalar(mut self, scalar: ResultScalar) -> Self {
        self.scalars.push(scalar);
        self
    }

    /// Append several typed scalars.
    #[must_use]
    pub fn scalars(mut self, scalars: impl IntoIterator<Item = ResultScalar>) -> Self {
        self.scalars.extend(scalars);
        self
    }

    /// Append several device state histories.
    #[must_use]
    pub fn device_states(mut self, states: impl IntoIterator<Item = DeviceStateSeries>) -> Self {
        self.device_states.extend(states);
        self
    }

    /// Borrow the payload staged so far.
    pub const fn payload_ref(&self) -> &ResultPayload {
        &self.payload
    }

    /// Replace the staged payload, keeping every series already added.
    ///
    /// This exists for composite families: an envelope result reuses the
    /// transient projection for its continued waveforms and then declares the
    /// envelope family that owns it.
    #[must_use]
    pub fn replace_payload(mut self, payload: ResultPayload) -> Self {
        self.payload = payload;
        self
    }

    /// Validate and finish the document.
    pub fn build(self) -> Result<AnalysisResultDocument, ResultDocumentError> {
        self.build_with_abort(&NoAbort)
    }

    /// Cancellable form of [`Self::build`].
    pub fn build_with_abort(
        self,
        abort: &dyn AbortSignal,
    ) -> Result<AnalysisResultDocument, ResultDocumentError> {
        let document = AnalysisResultDocument {
            schema: ANALYSIS_RESULT_DOCUMENT_SCHEMA.to_owned(),
            schema_version: ANALYSIS_RESULT_DOCUMENT_VERSION,
            result_kind: self.payload.result_kind(),
            analysis: self.analysis,
            parent_analysis: self.parent_analysis,
            coordinate: self.coordinate,
            topology_fingerprint: self.topology_fingerprint,
            namespaces: self.namespaces,
            point_count: self.point_count,
            axes: self.axes,
            signals: self.signals,
            scalars: self.scalars,
            device_states: self.device_states,
            payload: self.payload,
        };
        document.validate_with_abort(abort)?;
        Ok(document)
    }
}

//=============================================================================
// Coordinate identity and namespaces
//=============================================================================

/// The shared-deck coordinate a result was produced at.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ResultCoordinate {
    #[serde(with = "wire::run_coordinate_id")]
    id: RunCoordinateId,
    ordinal: usize,
    label: String,
    assignments: Vec<ResultAxisAssignment>,
}

impl ResultCoordinate {
    /// Project one planned coordinate into the document.
    pub fn from_run_coordinate(coordinate: &RunCoordinate) -> Self {
        Self {
            id: coordinate.stable_id(),
            ordinal: coordinate.ordinal(),
            label: coordinate.stable_tag(),
            assignments: coordinate
                .assignments()
                .iter()
                .map(|assignment| ResultAxisAssignment {
                    kind: assignment.kind(),
                    name: assignment.name().to_owned(),
                    value_index: assignment.value_index(),
                    value: assignment.value().clone(),
                    step_target: assignment.step_target().cloned(),
                })
                .collect(),
        }
    }

    /// Stable coordinate identity.
    pub const fn id(&self) -> RunCoordinateId {
        self.id
    }

    /// Zero-based position in the Cartesian coordinate order.
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    /// Display label for this coordinate.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Typed axis assignments that define this coordinate.
    pub fn assignments(&self) -> &[ResultAxisAssignment] {
        &self.assignments
    }

    fn validate(&self) -> Result<(), ResultDocumentError> {
        require_name("coordinate label", &self.label)?;
        for assignment in &self.assignments {
            require_name("coordinate axis name", &assignment.name)?;
            if let RunAxisValue::Numeric(value) = &assignment.value {
                finite("coordinate axis value", *value)?;
            }
        }
        Ok(())
    }
}

/// One typed axis assignment inside a coordinate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ResultAxisAssignment {
    #[serde(with = "wire::axis_kind")]
    kind: AxisKind,
    name: String,
    value_index: usize,
    #[serde(with = "wire::run_axis_value")]
    value: RunAxisValue,
    #[serde(with = "wire::optional_step_axis_target")]
    step_target: Option<StepAxisTarget>,
}

impl ResultAxisAssignment {
    /// Which planning dimension produced this assignment.
    pub const fn kind(&self) -> AxisKind {
        self.kind
    }

    /// Canonical axis name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Zero-based index of the selected value inside its axis.
    pub const fn value_index(&self) -> usize {
        self.value_index
    }

    /// The selected typed axis value.
    pub const fn value(&self) -> &RunAxisValue {
        &self.value
    }

    /// Typed `.STEP` target, when this axis came from an authored `.STEP`.
    pub const fn step_target(&self) -> Option<&StepAxisTarget> {
        self.step_target.as_ref()
    }
}

/// The artifact namespaces a result was written under.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ResultNamespaces {
    /// Namespace for output artifacts.
    pub output: String,
    /// Namespace for checkpoint artifacts.
    pub checkpoint: String,
}

//=============================================================================
// Axes
//=============================================================================

/// What a coordinate axis measures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResultAxisKind {
    Time,
    Frequency,
    OffsetFrequency,
    SweepValue,
    Temperature,
    TrialIndex,
    HarmonicIndex,
    Sideband,
    BinIndex,
    PortIndex,
    Phase,
    Index,
}

/// One typed coordinate axis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ResultAxis {
    name: String,
    display_name: String,
    kind: ResultAxisKind,
    #[serde(with = "wire::signal_unit")]
    unit: SignalUnit,
    values: AxisValues,
}

impl ResultAxis {
    /// Build one axis, rejecting empty names and non-finite coordinates.
    pub fn new(
        name: impl Into<String>,
        display_name: impl Into<String>,
        kind: ResultAxisKind,
        unit: SignalUnit,
        values: AxisValues,
    ) -> Result<Self, ResultDocumentError> {
        let name = name.into();
        let display_name = display_name.into();
        require_name("axis name", &name)?;
        require_name("axis display name", &display_name)?;
        let axis = Self {
            name: name.trim().to_ascii_lowercase(),
            display_name: display_name.trim().to_owned(),
            kind,
            unit,
            values,
        };
        axis.validate()?;
        Ok(axis)
    }

    /// Canonical axis name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Display spelling of the axis name.
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// What this axis measures.
    pub const fn kind(&self) -> ResultAxisKind {
        self.kind
    }

    /// Physical unit of the axis coordinates.
    pub const fn unit(&self) -> &SignalUnit {
        &self.unit
    }

    /// The axis coordinates.
    pub const fn values(&self) -> &AxisValues {
        &self.values
    }

    fn validate(&self) -> Result<(), ResultDocumentError> {
        require_name("axis name", &self.name)?;
        require_name("axis display name", &self.display_name)?;
        match &self.values {
            AxisValues::Real { values } => finite_slice("axis coordinate", values),
            AxisValues::Integer { .. } => Ok(()),
        }
    }
}

/// Coordinates of one axis.
///
/// An axis is a coordinate, so it has no missing values: a point that was not
/// computed is simply not a point of this result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "representation", rename_all = "snake_case")]
pub enum AxisValues {
    Real { values: Vec<f64> },
    Integer { values: Vec<i64> },
}

impl AxisValues {
    /// Number of coordinates on this axis.
    pub fn len(&self) -> usize {
        match self {
            Self::Real { values } => values.len(),
            Self::Integer { values } => values.len(),
        }
    }

    /// Whether the axis has no coordinates.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The coordinates in `start..end`, or `None` when this axis is shorter
    /// than the document's declared point count claims.
    fn slice(&self, start: usize, end: usize) -> Option<Self> {
        Some(match self {
            Self::Real { values } => Self::Real {
                values: values.get(start..end)?.to_vec(),
            },
            Self::Integer { values } => Self::Integer {
                values: values.get(start..end)?.to_vec(),
            },
        })
    }
}

//=============================================================================
// Signals
//=============================================================================

/// Why a series carries no values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SeriesAvailability {
    /// The series was computed and retained.
    Available,
    /// Output projection deliberately did not retain this series. Its
    /// descriptor, unit, and owner are still evidence that it exists.
    NotProjected,
    /// The signal does not exist at this coordinate, for example because a
    /// conditional `.STEP` topology removed the node that owns it.
    AbsentAtCoordinate,
}

impl SeriesAvailability {
    const fn requires_all_missing(self) -> bool {
        matches!(self, Self::NotProjected | Self::AbsentAtCoordinate)
    }
}

/// Which sub-result a signal belongs to, when one result family carries
/// several parallel response sets over the same axis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum SeriesQualifier {
    /// First-order distortion response to one authored tone.
    DistortionFundamental { tone: DistortionTone },
    /// One Volterra product response.
    DistortionProduct { product: DistortionProductTag },
    /// One PAC sideband spectrum.
    PacSideband { sideband: i32 },
}

impl SeriesQualifier {
    fn identity(&self) -> String {
        match self {
            Self::DistortionFundamental { tone } => format!("fundamental:{tone:?}"),
            Self::DistortionProduct { product } => format!("product:{}", product.label()),
            Self::PacSideband { sideband } => format!("sideband:{sideband}"),
        }
    }
}

/// One typed signal series keyed by its schema descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ResultSignal {
    #[serde(with = "wire::signal_descriptor")]
    descriptor: SignalDescriptor,
    qualifier: Option<SeriesQualifier>,
    availability: SeriesAvailability,
    values: SeriesValues,
}

impl ResultSignal {
    /// Build one series, checking value type, finiteness, and availability.
    pub fn new(
        descriptor: SignalDescriptor,
        qualifier: Option<SeriesQualifier>,
        availability: SeriesAvailability,
        values: SeriesValues,
    ) -> Result<Self, ResultDocumentError> {
        let signal = Self {
            descriptor,
            qualifier,
            availability,
            values,
        };
        signal.validate()?;
        Ok(signal)
    }

    /// Stable schema descriptor for this series.
    pub const fn descriptor(&self) -> &SignalDescriptor {
        &self.descriptor
    }

    /// Sub-result this series belongs to, when the family has several.
    pub const fn qualifier(&self) -> Option<&SeriesQualifier> {
        self.qualifier.as_ref()
    }

    /// Whether this series was retained, and if not, why.
    pub const fn availability(&self) -> SeriesAvailability {
        self.availability
    }

    /// The samples themselves.
    pub const fn values(&self) -> &SeriesValues {
        &self.values
    }

    /// Whether any sample of this series is present.
    pub fn has_any_sample(&self) -> bool {
        self.values.has_any_sample()
    }

    fn validate(&self) -> Result<(), ResultDocumentError> {
        let expected = self.values.value_type();
        if self.descriptor.value_type() != expected {
            return Err(ResultDocumentError::SignalValueType {
                name: self.descriptor.canonical_name().to_owned(),
                declared: self.descriptor.value_type(),
                encoded: expected,
            });
        }
        if self.availability.requires_all_missing() && self.values.has_any_sample() {
            return Err(ResultDocumentError::Malformed {
                location: "signal availability",
                detail: format!(
                    "'{}' declares that it was not retained but carries samples",
                    self.descriptor.canonical_name()
                ),
            });
        }
        self.values.validate(self.descriptor.canonical_name())
    }
}

/// Samples of one series, with explicit missingness.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "representation", rename_all = "snake_case")]
pub enum SeriesValues {
    Real { samples: Vec<Option<f64>> },
    Complex { samples: Vec<Option<ComplexSample>> },
    Logic { samples: Vec<Option<LogicSample>> },
}

impl SeriesValues {
    /// Number of points in this series.
    pub fn len(&self) -> usize {
        match self {
            Self::Real { samples } => samples.len(),
            Self::Complex { samples } => samples.len(),
            Self::Logic { samples } => samples.len(),
        }
    }

    /// Whether this series has no points.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Value type this representation encodes.
    pub const fn value_type(&self) -> SignalValueType {
        match self {
            Self::Real { .. } => SignalValueType::Real,
            Self::Complex { .. } => SignalValueType::Complex,
            Self::Logic { .. } => SignalValueType::Logic,
        }
    }

    /// Numeric columns one point of this series occupies.
    pub const fn numeric_columns(&self) -> usize {
        match self {
            Self::Real { .. } | Self::Logic { .. } => 1,
            Self::Complex { .. } => 2,
        }
    }

    /// Whether at least one sample is present.
    pub fn has_any_sample(&self) -> bool {
        match self {
            Self::Real { samples } => samples.iter().any(Option::is_some),
            Self::Complex { samples } => samples.iter().any(Option::is_some),
            Self::Logic { samples } => samples.iter().any(Option::is_some),
        }
    }

    fn validate(&self, name: &str) -> Result<(), ResultDocumentError> {
        match self {
            Self::Real { samples } => {
                for sample in samples.iter().flatten() {
                    if !sample.is_finite() {
                        return Err(ResultDocumentError::NonFinite {
                            location: format!("signal '{name}'"),
                            value: *sample,
                        });
                    }
                }
            }
            Self::Complex { samples } => {
                for sample in samples.iter().flatten() {
                    if !sample.real.is_finite() {
                        return Err(ResultDocumentError::NonFinite {
                            location: format!("signal '{name}' real part"),
                            value: sample.real,
                        });
                    }
                    if !sample.imaginary.is_finite() {
                        return Err(ResultDocumentError::NonFinite {
                            location: format!("signal '{name}' imaginary part"),
                            value: sample.imaginary,
                        });
                    }
                }
            }
            Self::Logic { .. } => {}
        }
        Ok(())
    }

    /// The samples in `start..end`, or `None` when this series is shorter
    /// than the document's declared point count claims.
    fn window(&self, start: usize, end: usize) -> Option<SeriesWindowValues> {
        Some(match self {
            Self::Real { samples } => {
                let samples = samples.get(start..end)?;
                SeriesWindowValues::Real {
                    values: samples.iter().map(|sample| sample.unwrap_or(0.0)).collect(),
                    validity: validity_mask(samples),
                }
            }
            Self::Complex { samples } => {
                let samples = samples.get(start..end)?;
                SeriesWindowValues::Complex {
                    real: samples
                        .iter()
                        .map(|sample| sample.map_or(0.0, |sample| sample.real))
                        .collect(),
                    imaginary: samples
                        .iter()
                        .map(|sample| sample.map_or(0.0, |sample| sample.imaginary))
                        .collect(),
                    validity: validity_mask(samples),
                }
            }
            Self::Logic { samples } => {
                let samples = samples.get(start..end)?;
                SeriesWindowValues::Logic {
                    samples: samples.to_vec(),
                    validity: validity_mask(samples),
                }
            }
        })
    }
}

fn validity_mask<T>(samples: &[Option<T>]) -> Vec<u8> {
    samples
        .iter()
        .map(|sample| u8::from(sample.is_some()))
        .collect()
}

/// One complex sample.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ComplexSample {
    pub real: f64,
    pub imaginary: f64,
}

impl ComplexSample {
    /// Build a sample from a core complex value.
    pub const fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }
}

impl From<num_complex::Complex64> for ComplexSample {
    fn from(value: num_complex::Complex64) -> Self {
        Self {
            real: value.re,
            imaginary: value.im,
        }
    }
}

impl From<ComplexSample> for num_complex::Complex64 {
    fn from(sample: ComplexSample) -> Self {
        Self::new(sample.real, sample.imaginary)
    }
}

/// One digital sample: a logic state with its drive strength.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LogicSample {
    pub state: DigitalStateTag,
    pub strength: DigitalStrengthTag,
}

//=============================================================================
// Scalars
//=============================================================================

/// One typed per-analysis scalar, such as a stability margin or a residual.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ResultScalar {
    name: String,
    display_name: String,
    #[serde(with = "wire::optional_signal_unit")]
    unit: Option<SignalUnit>,
    value: ScalarValue,
}

impl ResultScalar {
    /// Build one scalar, rejecting empty names and non-finite values.
    pub fn new(
        name: impl Into<String>,
        display_name: impl Into<String>,
        unit: Option<SignalUnit>,
        value: ScalarValue,
    ) -> Result<Self, ResultDocumentError> {
        let name = name.into();
        let display_name = display_name.into();
        require_name("scalar name", &name)?;
        require_name("scalar display name", &display_name)?;
        let scalar = Self {
            name: name.trim().to_ascii_lowercase(),
            display_name: display_name.trim().to_owned(),
            unit,
            value,
        };
        scalar.validate()?;
        Ok(scalar)
    }

    /// Canonical scalar name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Display spelling of the scalar name.
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Unit, or `None` when the producing result declares none.
    pub const fn unit(&self) -> Option<&SignalUnit> {
        self.unit.as_ref()
    }

    /// The value itself.
    pub const fn value(&self) -> &ScalarValue {
        &self.value
    }

    fn validate(&self) -> Result<(), ResultDocumentError> {
        require_name("scalar name", &self.name)?;
        require_name("scalar display name", &self.display_name)?;
        match &self.value {
            ScalarValue::Real { value } => finite_optional("scalar", *value),
            ScalarValue::Complex { value } => {
                if let Some(sample) = value {
                    finite("scalar real part", sample.real)?;
                    finite("scalar imaginary part", sample.imaginary)?;
                }
                Ok(())
            }
            ScalarValue::Integer { .. }
            | ScalarValue::Count { .. }
            | ScalarValue::Boolean { .. }
            | ScalarValue::Unavailable { .. } => Ok(()),
            ScalarValue::Text { value } => require_name("scalar text value", value),
        }
    }
}

/// A typed scalar value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "representation", rename_all = "snake_case")]
pub enum ScalarValue {
    Real {
        value: Option<f64>,
    },
    Complex {
        value: Option<ComplexSample>,
    },
    Integer {
        value: i64,
    },
    Count {
        value: u64,
    },
    Boolean {
        value: bool,
    },
    Text {
        value: String,
    },
    /// A real quantity the analysis proved has no finite value, with the
    /// reason. A finite `ResultScalar` cannot hold `±inf`, and rounding one
    /// down to a large number would report a margin the loop does not have.
    Unavailable {
        reason: ScalarUnavailability,
    },
}

/// Why a real scalar has no finite value.
///
/// This is stronger than `Real { value: None }`, which says only that the
/// producing analysis did not compute the quantity. Each variant here is a
/// determination: the analysis ran, and the answer is not a number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ScalarUnavailability {
    /// The quantity diverges upward — the input impedance of an ideal voltage
    /// source, or the gain margin of a loop whose phase never reaches -180°.
    PositiveInfinity,
    /// The quantity diverges downward, such as the phase margin of a loop
    /// whose magnitude never falls below unity.
    NegativeInfinity,
    /// The quantity is only defined at a crossover the response never makes,
    /// so it has no value at all rather than an infinite one.
    NoCrossover,
}

impl ScalarUnavailability {
    /// Classify one non-finite real value, or `None` when it is finite.
    ///
    /// `NaN` is deliberately not classified: it is a defect in the producing
    /// computation, not a determination about the circuit, and the projection
    /// must keep rejecting it.
    pub const fn classify(value: f64) -> Option<Self> {
        if value == f64::INFINITY {
            Some(Self::PositiveInfinity)
        } else if value == f64::NEG_INFINITY {
            Some(Self::NegativeInfinity)
        } else {
            None
        }
    }

    /// Stable tag used by diagnostics and frontends.
    pub const fn tag(self) -> &'static str {
        match self {
            Self::PositiveInfinity => "positive_infinity",
            Self::NegativeInfinity => "negative_infinity",
            Self::NoCrossover => "no_crossover",
        }
    }
}

//=============================================================================
// Device state
//=============================================================================

/// Per-device operating state captured alongside a result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DeviceStateSeries {
    device_name: String,
    device_kind: Option<String>,
    /// Operating region at each point, or empty when the family reports none.
    regions: Vec<Option<String>>,
    parameters: Vec<DeviceParameterSeries>,
}

impl DeviceStateSeries {
    /// Build one device state history.
    pub fn new(
        device_name: impl Into<String>,
        device_kind: Option<String>,
        regions: Vec<Option<String>>,
        parameters: Vec<DeviceParameterSeries>,
    ) -> Result<Self, ResultDocumentError> {
        let device_name = device_name.into();
        require_name("device state name", &device_name)?;
        Ok(Self {
            device_name,
            device_kind,
            regions,
            parameters,
        })
    }

    /// Instance name as written in the netlist.
    pub fn device_name(&self) -> &str {
        &self.device_name
    }

    /// Device family label, when the producing result declares one.
    pub fn device_kind(&self) -> Option<&str> {
        self.device_kind.as_deref()
    }

    /// Operating region at each point, or empty.
    pub fn regions(&self) -> &[Option<String>] {
        &self.regions
    }

    /// Named operating-point parameter histories.
    pub fn parameters(&self) -> &[DeviceParameterSeries] {
        &self.parameters
    }

    fn value_count(&self) -> usize {
        self.parameters
            .iter()
            .map(|parameter| parameter.values.len())
            .fold(0, usize::saturating_add)
    }

    fn validate(&self, point_count: usize) -> Result<(), ResultDocumentError> {
        require_name("device state name", &self.device_name)?;
        if !self.regions.is_empty() && self.regions.len() != point_count {
            return Err(ResultDocumentError::SeriesLength {
                location: format!("device '{}' regions", self.device_name),
                expected: point_count,
                actual: self.regions.len(),
            });
        }
        let mut names = BTreeSet::new();
        for parameter in &self.parameters {
            require_name("device parameter name", &parameter.name)?;
            if parameter.values.len() != point_count {
                return Err(ResultDocumentError::SeriesLength {
                    location: format!(
                        "device '{}' parameter '{}'",
                        self.device_name, parameter.name
                    ),
                    expected: point_count,
                    actual: parameter.values.len(),
                });
            }
            for value in &parameter.values {
                finite_optional("device parameter", *value)?;
            }
            if !names.insert(parameter.name.to_ascii_lowercase()) {
                return Err(ResultDocumentError::DuplicateSeries {
                    location: "device parameter",
                    name: parameter.name.clone(),
                });
            }
        }
        Ok(())
    }
}

/// One named device operating-point parameter over the document's points.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DeviceParameterSeries {
    pub name: String,
    #[serde(with = "wire::optional_signal_unit")]
    pub unit: Option<SignalUnit>,
    pub values: Vec<Option<f64>>,
}

//=============================================================================
// Windows
//=============================================================================

/// A bounded copy of the document's series over a point range.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultWindow {
    pub schema_version: u32,
    #[serde(with = "wire::analysis_instance_id")]
    pub analysis: AnalysisInstanceId,
    #[serde(with = "wire::optional_run_coordinate_id")]
    pub coordinate_id: Option<RunCoordinateId>,
    pub start: usize,
    pub count: usize,
    pub point_count: usize,
    pub axes: Vec<AxisWindow>,
    pub signals: Vec<SignalWindow>,
}

/// One axis inside a window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisWindow {
    pub name: String,
    pub values: AxisValues,
}

/// One signal inside a window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalWindow {
    pub canonical_name: String,
    pub qualifier: Option<SeriesQualifier>,
    pub values: SeriesWindowValues,
}

/// Window samples, split into dense numeric columns and a validity mask.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "representation", rename_all = "snake_case")]
pub enum SeriesWindowValues {
    Real {
        values: Vec<f64>,
        validity: Vec<u8>,
    },
    Complex {
        real: Vec<f64>,
        imaginary: Vec<f64>,
        validity: Vec<u8>,
    },
    Logic {
        samples: Vec<Option<LogicSample>>,
        validity: Vec<u8>,
    },
}

//=============================================================================
// Shared validation helpers
//=============================================================================

fn check_abort(abort: &dyn AbortSignal) -> Result<(), ResultDocumentError> {
    if abort.is_aborted() {
        Err(ResultDocumentError::Aborted)
    } else {
        Ok(())
    }
}

fn require_name(location: &'static str, name: &str) -> Result<(), ResultDocumentError> {
    if name.trim().is_empty() {
        Err(ResultDocumentError::EmptyName { location })
    } else {
        Ok(())
    }
}

fn finite(location: &'static str, value: f64) -> Result<(), ResultDocumentError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(ResultDocumentError::NonFinite {
            location: location.to_owned(),
            value,
        })
    }
}

fn finite_optional(location: &'static str, value: Option<f64>) -> Result<(), ResultDocumentError> {
    match value {
        Some(value) => finite(location, value),
        None => Ok(()),
    }
}

fn finite_slice(location: &'static str, values: &[f64]) -> Result<(), ResultDocumentError> {
    for value in values {
        finite(location, *value)?;
    }
    Ok(())
}

//=============================================================================
// Errors
//=============================================================================

/// Why a result document could not be built, validated, encoded, or decoded.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum ResultDocumentError {
    /// The abort source fired before the operation completed.
    Aborted,
    /// The JSON declared a different schema identifier.
    WrongSchema { found: String },
    /// The JSON declared a schema version this build does not implement.
    UnsupportedVersion { found: u32, current: u32 },
    /// Encoding would have exceeded the declared byte limit.
    ArtifactTooLarge { limit_bytes: u64 },
    /// The encoder or decoder could not allocate.
    AllocationFailed,
    /// The JSON encoder or decoder rejected the document.
    Json(String),
    /// A required name was empty.
    EmptyName { location: &'static str },
    /// A value that must be finite was not.
    NonFinite { location: String, value: f64 },
    /// A series did not have the document's point count.
    SeriesLength {
        location: String,
        expected: usize,
        actual: usize,
    },
    /// Two axes, signals, scalars, or device states shared one identity.
    DuplicateSeries {
        location: &'static str,
        name: String,
    },
    /// A signal's descriptor and its encoded samples disagree.
    SignalValueType {
        name: String,
        declared: SignalValueType,
        encoded: SignalValueType,
    },
    /// The payload belongs to a different result family than the document.
    PayloadFamilyMismatch {
        declared: AnalysisResultKind,
        payload: AnalysisResultKind,
    },
    /// The analysis identity belongs to a different result family.
    AnalysisFamilyMismatch {
        declared: AnalysisResultKind,
        analysis: AnalysisResultKind,
    },
    /// A post-process family did not name the analysis it derived from.
    MissingParentAnalysis { result_kind: AnalysisResultKind },
    /// A family that is not a post-process named a parent analysis.
    UnexpectedParentAnalysis { result_kind: AnalysisResultKind },
    /// A post-process named a parent of the wrong analysis kind.
    WrongParentAnalysis {
        result_kind: AnalysisResultKind,
        /// Every parent family the result kind derives from.
        expected: &'static [AnalysisKind],
        found: AnalysisKind,
    },
    /// A window did not fit inside the document.
    WindowOutOfBounds {
        start: usize,
        count: usize,
        point_count: usize,
    },
    /// A source result could not be projected into the document.
    SourceResult {
        location: &'static str,
        detail: String,
    },
    /// A structural invariant of the document was violated.
    Malformed {
        location: &'static str,
        detail: String,
    },
}

impl fmt::Display for ResultDocumentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Aborted => formatter.write_str("result document work was cancelled"),
            Self::WrongSchema { found } => {
                write!(formatter, "unexpected result document schema {found:?}")
            }
            Self::UnsupportedVersion { found, current } => write!(
                formatter,
                "result document version {found} is unsupported (this build writes version {current})"
            ),
            Self::ArtifactTooLarge { limit_bytes } => write!(
                formatter,
                "result document exceeds its {limit_bytes}-byte limit"
            ),
            Self::AllocationFailed => {
                formatter.write_str("result document serialization could not allocate")
            }
            Self::Json(detail) => write!(formatter, "result document JSON error: {detail}"),
            Self::EmptyName { location } => write!(formatter, "{location} is empty"),
            Self::NonFinite { location, value } => {
                write!(formatter, "{location} has non-finite value {value}")
            }
            Self::SeriesLength {
                location,
                expected,
                actual,
            } => write!(
                formatter,
                "{location} has {actual} samples, expected {expected}"
            ),
            Self::DuplicateSeries { location, name } => {
                write!(formatter, "duplicate {location} identity {name:?}")
            }
            Self::SignalValueType {
                name,
                declared,
                encoded,
            } => write!(
                formatter,
                "signal {name:?} declares {declared:?} values but encodes {encoded:?}"
            ),
            Self::PayloadFamilyMismatch { declared, payload } => write!(
                formatter,
                "document declares result family {} but carries a {} payload",
                declared.tag(),
                payload.tag()
            ),
            Self::AnalysisFamilyMismatch { declared, analysis } => write!(
                formatter,
                "document declares result family {} but its analysis produces {}",
                declared.tag(),
                analysis.tag()
            ),
            Self::MissingParentAnalysis { result_kind } => write!(
                formatter,
                "{} results must name the analysis they post-processed",
                result_kind.tag()
            ),
            Self::UnexpectedParentAnalysis { result_kind } => write!(
                formatter,
                "{} results are not post-processed and cannot name a parent analysis",
                result_kind.tag()
            ),
            Self::WrongParentAnalysis {
                result_kind,
                expected,
                found,
            } => write!(
                formatter,
                "{} results derive from {} analyses, not {}",
                result_kind.tag(),
                expected
                    .iter()
                    .map(|kind| kind.tag())
                    .collect::<Vec<_>>()
                    .join(" or "),
                found.tag()
            ),
            Self::WindowOutOfBounds {
                start,
                count,
                point_count,
            } => write!(
                formatter,
                "window [{start}, {start}+{count}) is outside a {point_count}-point result"
            ),
            Self::SourceResult { location, detail } => {
                write!(formatter, "{location} cannot be projected: {detail}")
            }
            Self::Malformed { location, detail } => write!(formatter, "{location}: {detail}"),
        }
    }
}

impl std::error::Error for ResultDocumentError {}
