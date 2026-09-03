//! Lossless, versioned transient `.FFT` result contract.
//!
//! This is the one typed artifact the adapter still owns. Every other family
//! publishes `rspice_core::execution::AnalysisResultDocument`; the FFT family
//! cannot yet, because the shared `fft` result document must be named by an
//! `AnalysisInstanceId` and `DeckPlan` mints none for a `.FFT` post-process
//! (`AnalysisInstanceId` also has no public constructor). Until core assigns
//! post-process identities, deleting this bundle would drop every authored
//! `.FFT` spectrum from the response, so it stays and names its parent
//! transient by that transient's canonical analysis tag.

use std::collections::HashSet;

use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::engine::{
    TransientFftBin, TransientFftHarmonic, TransientFftMetrics, TransientFftResult,
    transient_fft_window_coherent_gain,
};
use rspice_core::execution::bounded_io::{BoundedAbortWriter, BoundedWriteFailure};
use rspice_core::netlist::{FftAnalysis, FftFormat, FftOutput, FftWindow, XyceFftMode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::wire::MAX_ENGINE_RETAINED_RESULT_BYTES;

/// Stable schema identity for a bundle of transient FFT results.
pub const FFT_RESULT_DOCUMENT_SCHEMA: &str = "rspice-transient-fft-result";
/// The only FFT result schema version this build writes or reads.
pub const FFT_RESULT_DOCUMENT_VERSION: u32 = 1;
/// MIME type declared for version-1 FFT result artifacts.
pub const FFT_RESULT_DOCUMENT_CONTENT_TYPE: &str =
    "application/vnd.rspice.transient-fft-result+json;version=1";

const FFT_DB_REPORTING_FLOOR: f64 = 1.0e-10;
const MAX_RANKED_HARMONICS: usize = 30;

/// Every `.FFT` result belonging to one executed transient directive.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransientFftResultDocument {
    pub schema: String,
    pub schema_version: u32,
    /// Canonical analysis tag of the transient these spectra were taken from,
    /// exactly as `AnalysisInstanceId::tag` spells it (for example `tran-001`).
    pub parent_analysis: String,
    pub result_count: usize,
    /// Source order is significant and matches the deck's `.FFT` directives.
    pub results: Vec<FftResultDocument>,
}

impl TransientFftResultDocument {
    /// Map engine-owned transient FFT results into the public adapter schema.
    /// Unsupported physical types and inconsistent data fail closed.
    pub fn from_engine_results(
        parent_analysis: String,
        results: &[TransientFftResult],
        authored: &[FftAnalysis],
        authored_mode: XyceFftMode,
    ) -> Result<Self, FftResultDocumentError> {
        Self::from_engine_results_with_abort(
            parent_analysis,
            results,
            authored,
            authored_mode,
            &NoAbort,
        )
    }

    pub fn from_engine_results_with_abort(
        parent_analysis: String,
        results: &[TransientFftResult],
        authored: &[FftAnalysis],
        authored_mode: XyceFftMode,
        abort: &dyn AbortSignal,
    ) -> Result<Self, FftResultDocumentError> {
        if results.len() != authored.len() || results.is_empty() {
            return Err(invalid(
                "FFT engine results must match a nonempty authored directive sequence",
            ));
        }
        let mut mapped = Vec::new();
        mapped
            .try_reserve_exact(results.len())
            .map_err(|_| invalid("cannot allocate the typed FFT result document sequence"))?;
        for (index, (result, authored)) in results.iter().zip(authored).enumerate() {
            check_abort(abort)?;
            mapped.push(FftResultDocument::from_engine(
                result,
                authored,
                authored_mode,
                index + 1,
                abort,
            )?);
        }
        let mut results = mapped;
        for result in &mut results {
            result.parent_analysis_id = parent_analysis.clone();
        }
        let document = Self {
            schema: FFT_RESULT_DOCUMENT_SCHEMA.to_owned(),
            schema_version: FFT_RESULT_DOCUMENT_VERSION,
            parent_analysis,
            result_count: results.len(),
            results,
        };
        document.validate_with_abort(abort)?;
        Ok(document)
    }

    /// Validate and serialize a current-version document.
    pub fn to_json(&self) -> Result<String, FftResultDocumentError> {
        self.to_json_with_abort(&NoAbort, MAX_ENGINE_RETAINED_RESULT_BYTES)
    }

    pub fn to_json_with_abort(
        &self,
        abort: &dyn AbortSignal,
        byte_limit: u64,
    ) -> Result<String, FftResultDocumentError> {
        self.validate_with_abort(abort)?;
        check_abort(abort)?;
        let mut writer = BoundedAbortWriter::new(abort, byte_limit);
        if let Err(error) = serde_json::to_writer_pretty(&mut writer, self) {
            return Err(match writer.failure() {
                Some(BoundedWriteFailure::Aborted) => FftResultDocumentError::Aborted,
                Some(BoundedWriteFailure::ByteLimitExceeded { limit_bytes }) => {
                    FftResultDocumentError::ArtifactTooLarge { limit_bytes }
                }
                Some(BoundedWriteFailure::AllocationFailed) => {
                    invalid(&format!("cannot allocate FFT JSON: {error}"))
                }
                None => FftResultDocumentError::InvalidJson(error),
            });
        }
        check_abort(abort)?;
        writer
            .into_string()
            .map_err(|error| invalid(&format!("FFT JSON serialization was not UTF-8: {error}")))
    }

    /// Decode only this build's exact schema and reject future versions before
    /// strict field decoding.
    pub fn from_json(json: &str) -> Result<Self, FftResultDocumentError> {
        self::TransientFftResultDocument::from_json_with_abort(
            json,
            &NoAbort,
            MAX_ENGINE_RETAINED_RESULT_BYTES,
        )
    }

    /// Decode one bounded current-version FFT artifact cooperatively.
    pub fn from_json_with_abort(
        json: &str,
        abort: &dyn AbortSignal,
        byte_limit: u64,
    ) -> Result<Self, FftResultDocumentError> {
        check_abort(abort)?;
        if json.len() as u128 > byte_limit as u128 {
            return Err(FftResultDocumentError::ArtifactTooLarge {
                limit_bytes: byte_limit,
            });
        }
        #[derive(Deserialize)]
        struct Header {
            schema: String,
            schema_version: u32,
        }

        let header: Header =
            serde_json::from_str(json).map_err(FftResultDocumentError::InvalidJson)?;
        if header.schema != FFT_RESULT_DOCUMENT_SCHEMA {
            return Err(FftResultDocumentError::WrongSchema(header.schema));
        }
        if header.schema_version != FFT_RESULT_DOCUMENT_VERSION {
            return Err(FftResultDocumentError::UnsupportedVersion {
                found: header.schema_version,
                current: FFT_RESULT_DOCUMENT_VERSION,
            });
        }
        let document: Self =
            serde_json::from_str(json).map_err(FftResultDocumentError::InvalidJson)?;
        check_abort(abort)?;
        document.validate_with_abort(abort)?;
        Ok(document)
    }

    /// Enforce parent/child identity, source order, units, shapes, and all
    /// transform invariants needed for a lossless reader.
    pub fn validate(&self) -> Result<(), FftResultDocumentError> {
        self.validate_with_abort(&NoAbort)
    }

    pub fn validate_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<(), FftResultDocumentError> {
        if self.schema != FFT_RESULT_DOCUMENT_SCHEMA {
            return Err(FftResultDocumentError::WrongSchema(self.schema.clone()));
        }
        if self.schema_version != FFT_RESULT_DOCUMENT_VERSION {
            return Err(FftResultDocumentError::UnsupportedVersion {
                found: self.schema_version,
                current: FFT_RESULT_DOCUMENT_VERSION,
            });
        }
        if !valid_transient_analysis_tag(&self.parent_analysis) {
            return Err(invalid(
                "parent analysis must be the canonical transient directive tag",
            ));
        }
        if self.results.is_empty() || self.result_count != self.results.len() {
            return Err(invalid(
                "FFT result_count must equal a nonempty result sequence",
            ));
        }

        let mut identities = HashSet::new();
        for (index, result) in self.results.iter().enumerate() {
            check_abort(abort)?;
            let ordinal = index + 1;
            if result.ordinal != ordinal
                || result.analysis_id != format!("fft-{ordinal:03}")
                || result.parent_analysis_id != self.parent_analysis
                || !identities.insert(result.analysis_id.clone())
            {
                return Err(invalid("FFT identity or source ordering is invalid"));
            }
            result.validate_with_abort(abort)?;
        }
        Ok(())
    }
}

/// Complete typed result of one source-authored `.FFT` directive.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FftResultDocument {
    pub analysis_id: String,
    pub parent_analysis_id: String,
    pub ordinal: usize,
    pub source: FftSourceDocument,
    pub authored: FftAuthoredControlsDocument,
    pub signal: FftSignalDocument,
    pub sampling: FftSamplingDocument,
    pub transform: FftTransformDocument,
    pub spectrum: FftSpectrumDocument,
    pub metrics: Option<FftMetricsDocument>,
}

impl FftResultDocument {
    fn from_engine(
        result: &TransientFftResult,
        authored: &FftAnalysis,
        authored_mode: XyceFftMode,
        ordinal: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Self, FftResultDocumentError> {
        if result.output != authored.output {
            return Err(invalid(
                "FFT engine result output does not match its authored directive",
            ));
        }
        let (source_kind, source_text, authored_output) = match &result.output {
            FftOutput::Probe(probe) => (FftSourceKind::Probe, probe.clone(), probe.clone()),
            FftOutput::Expression(expression) => (
                FftSourceKind::Expression,
                expression.clone(),
                format!("{{{expression}}}"),
            ),
        };
        let physical_type = match result.physical_type {
            "voltage" => FftPhysicalType::Voltage,
            "current" => FftPhysicalType::Current,
            "parameter" => FftPhysicalType::Parameter,
            unsupported => {
                return Err(invalid(&format!(
                    "unsupported FFT physical type {unsupported:?}"
                )));
            }
        };
        let format = FftCoefficientFormat::from(result.format);
        let value_unit = physical_type.value_unit(format);
        let mut bins = Vec::new();
        bins.try_reserve_exact(result.bins.len())
            .map_err(|_| invalid("cannot allocate typed FFT bins"))?;
        for bin in &result.bins {
            if bin.index.is_multiple_of(256) {
                check_abort(abort)?;
            }
            bins.push(FftBinDocument::from(bin));
        }
        let document = Self {
            analysis_id: format!("fft-{ordinal:03}"),
            // The enclosing bundle assigns the exact parent during mapping.
            parent_analysis_id: String::new(),
            ordinal,
            source: FftSourceDocument {
                kind: source_kind,
                text: source_text,
                authored_output,
            },
            authored: FftAuthoredControlsDocument::from_analysis(authored, authored_mode),
            signal: FftSignalDocument {
                name: result.output_name.clone(),
                physical_type,
                unit: value_unit,
            },
            sampling: FftSamplingDocument {
                start_time_seconds: result.start_time,
                stop_time_seconds: result.stop_time,
                sample_interval_seconds: result.sample_interval,
                point_count: result.point_count,
                accurate_sampling: result.accurate_sampling,
            },
            transform: FftTransformDocument {
                format,
                mode: result.mode.into(),
                window: result.window.into(),
                window_name: result.window_name.clone(),
                alpha: result.alpha,
                coherent_gain: result.coherent_gain,
                frequency_resolution_hertz: result.frequency_resolution,
                fundamental_bin: result.fundamental_bin,
                minimum_metric_bin: result.minimum_metric_bin,
                maximum_metric_bin: result.maximum_metric_bin,
                sfdr_search_minimum_bin: if authored.minimum_frequency.is_none()
                    && result.maximum_metric_bin >= result.fundamental_bin
                {
                    result.fundamental_bin
                } else {
                    result.minimum_metric_bin
                },
            },
            spectrum: FftSpectrumDocument {
                frequency_unit: FftUnit::Hertz,
                value_unit,
                phase_unit: FftUnit::Degree,
                complex_representation: FftComplexRepresentation::Cartesian,
                bins,
            },
            metrics: result
                .metrics
                .as_ref()
                .map(|metrics| FftMetricsDocument::from_engine(metrics, value_unit, abort))
                .transpose()?,
        };
        document.validate_with_abort(abort)?;
        Ok(document)
    }

    fn validate_with_abort(&self, abort: &dyn AbortSignal) -> Result<(), FftResultDocumentError> {
        if self.ordinal == 0
            || self.analysis_id != format!("fft-{:03}", self.ordinal)
            || self.source.text.trim().is_empty()
            || self.source.authored_output.trim().is_empty()
            || self.signal.name.trim().is_empty()
            || self.signal.unit != self.signal.physical_type.value_unit(self.transform.format)
        {
            return Err(invalid("FFT source, signal, identity, or unit is invalid"));
        }
        match self.source.kind {
            FftSourceKind::Probe if self.source.authored_output != self.source.text => {
                return Err(invalid("FFT probe identity is not preserved exactly"));
            }
            FftSourceKind::Expression
                if self.source.authored_output != format!("{{{}}}", self.source.text) =>
            {
                return Err(invalid("FFT expression identity is not preserved exactly"));
            }
            _ => {}
        }

        self.sampling.validate()?;
        if self.spectrum.bins.len() != self.sampling.point_count / 2 + 1 {
            return Err(invalid(
                "FFT spectrum shape does not match its declared point count",
            ));
        }
        self.transform.validate(self.sampling.point_count, abort)?;
        self.authored.validate()?;
        self.validate_authored_consistency()?;
        let expected_resolution =
            1.0 / (self.sampling.stop_time_seconds - self.sampling.start_time_seconds);
        if !approximately_equal(
            self.transform.frequency_resolution_hertz,
            expected_resolution,
        ) {
            return Err(invalid(
                "FFT frequency resolution is inconsistent with its sample window",
            ));
        }
        self.spectrum.validate(
            self.sampling.point_count,
            self.signal.unit,
            &self.transform,
            abort,
        )?;
        if let Some(metrics) = &self.metrics {
            metrics.validate(
                self.signal.unit,
                &self.spectrum.bins,
                &self.transform,
                abort,
            )?;
        }
        Ok(())
    }

    fn validate_authored_consistency(&self) -> Result<(), FftResultDocumentError> {
        let effective_default_format = match self.transform.mode {
            FftCompatibilityMode::HspiceCompatible => FftCoefficientFormat::Normalized,
            FftCompatibilityMode::SpectreCompatible => FftCoefficientFormat::Unnormalized,
        };
        if self.authored.compatibility_mode != self.transform.mode
            || self.authored.point_count != self.sampling.point_count
            || !approximately_equal(
                self.authored.start_time_seconds.unwrap_or(0.0),
                self.sampling.start_time_seconds,
            )
            || self
                .authored
                .stop_time_seconds
                .is_some_and(|stop| !approximately_equal(stop, self.sampling.stop_time_seconds))
            || self.authored.format.unwrap_or(effective_default_format) != self.transform.format
            || self.authored.window != self.transform.window
            || self.authored.window_name != self.transform.window_name
            || !approximately_equal(self.authored.alpha, self.transform.alpha)
        {
            return Err(invalid(
                "effective FFT transform does not match its authored controls",
            ));
        }
        let last_bin = self.sampling.point_count / 2;
        let fundamental = authored_frequency_bin(
            self.authored.fundamental_frequency_hertz,
            1,
            self.transform.frequency_resolution_hertz,
            last_bin,
            false,
        )?;
        let minimum = authored_frequency_bin(
            self.authored.minimum_frequency_hertz,
            1,
            self.transform.frequency_resolution_hertz,
            last_bin,
            true,
        )?;
        let maximum = authored_frequency_bin(
            self.authored.maximum_frequency_hertz,
            last_bin,
            self.transform.frequency_resolution_hertz,
            last_bin,
            true,
        )?;
        let sfdr_search_minimum =
            if self.authored.minimum_frequency_hertz.is_none() && maximum >= fundamental {
                fundamental
            } else {
                minimum
            };
        if fundamental != self.transform.fundamental_bin
            || minimum != self.transform.minimum_metric_bin
            || maximum != self.transform.maximum_metric_bin
            || sfdr_search_minimum != self.transform.sfdr_search_minimum_bin
        {
            return Err(invalid(
                "effective FFT frequency bins do not match authored FREQ/FMIN/FMAX",
            ));
        }
        Ok(())
    }
}

/// Authored output identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FftSourceDocument {
    pub kind: FftSourceKind,
    /// Probe spelling or unbraced expression text retained by the parser.
    pub text: String,
    /// Complete source form, including braces around an expression.
    pub authored_output: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FftSourceKind {
    Probe,
    Expression,
}

/// Source-authored transform controls before transient-dependent defaults are
/// resolved. Retaining these alongside the effective transform makes repeated
/// directives with the same output distinguishable and auditable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FftAuthoredControlsDocument {
    /// Effective deck-level `.OPTIONS FFT FFT_MODE`, including its default.
    pub compatibility_mode: FftCompatibilityMode,
    pub start_time_seconds: Option<f64>,
    pub stop_time_seconds: Option<f64>,
    pub point_count: usize,
    pub format: Option<FftCoefficientFormat>,
    pub window: FftWindowKind,
    pub window_name: String,
    pub alpha: f64,
    pub fundamental_frequency_hertz: Option<f64>,
    pub minimum_frequency_hertz: Option<f64>,
    pub maximum_frequency_hertz: Option<f64>,
}

impl FftAuthoredControlsDocument {
    fn from_analysis(value: &FftAnalysis, mode: XyceFftMode) -> Self {
        Self {
            compatibility_mode: mode.into(),
            start_time_seconds: value.start,
            stop_time_seconds: value.stop,
            point_count: value.points,
            format: value.format.map(FftCoefficientFormat::from),
            window: FftWindowKind::from(value.window),
            window_name: value.window_name.clone(),
            alpha: value.alpha,
            fundamental_frequency_hertz: value.fundamental_frequency,
            minimum_frequency_hertz: value.minimum_frequency,
            maximum_frequency_hertz: value.maximum_frequency,
        }
    }

    fn validate(&self) -> Result<(), FftResultDocumentError> {
        if self.point_count < 4
            || !self.point_count.is_power_of_two()
            || !window_name_matches(self.window, &self.window_name)
            || !self.alpha.is_finite()
            || !(1.0..=20.0).contains(&self.alpha)
            || self
                .start_time_seconds
                .is_some_and(|value| !value.is_finite() || value < 0.0)
            || self
                .stop_time_seconds
                .is_some_and(|value| !value.is_finite() || value <= 0.0)
            || self
                .fundamental_frequency_hertz
                .is_some_and(|value| !value.is_finite() || value <= 0.0)
            || self
                .minimum_frequency_hertz
                .is_some_and(|value| !value.is_finite() || value < 0.0)
            || self
                .maximum_frequency_hertz
                .is_some_and(|value| !value.is_finite() || value <= 0.0)
        {
            return Err(invalid("authored FFT controls are invalid"));
        }
        if let (Some(start), Some(stop)) = (self.start_time_seconds, self.stop_time_seconds)
            && stop <= start
        {
            return Err(invalid("authored FFT STOP must exceed START"));
        }
        if let (Some(minimum), Some(maximum)) =
            (self.minimum_frequency_hertz, self.maximum_frequency_hertz)
            && minimum > maximum
        {
            return Err(invalid("authored FFT FMIN exceeds FMAX"));
        }
        Ok(())
    }
}

/// Resolved scalar signal and its physical dimension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FftSignalDocument {
    pub name: String,
    pub physical_type: FftPhysicalType,
    /// `None` is explicit unknown/parameter dimension, not dimensionless.
    pub unit: Option<FftUnit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FftPhysicalType {
    Voltage,
    Current,
    Parameter,
}

impl FftPhysicalType {
    const fn value_unit(self, format: FftCoefficientFormat) -> Option<FftUnit> {
        if matches!(format, FftCoefficientFormat::Normalized) {
            return Some(FftUnit::Dimensionless);
        }
        match self {
            Self::Voltage => Some(FftUnit::Volt),
            Self::Current => Some(FftUnit::Ampere),
            Self::Parameter => None,
        }
    }
}

/// Uniform resampling contract used by the transform.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FftSamplingDocument {
    pub start_time_seconds: f64,
    pub stop_time_seconds: f64,
    pub sample_interval_seconds: f64,
    pub point_count: usize,
    pub accurate_sampling: bool,
}

impl FftSamplingDocument {
    fn validate(&self) -> Result<(), FftResultDocumentError> {
        if self.point_count < 4
            || !self.point_count.is_power_of_two()
            || !self.start_time_seconds.is_finite()
            || !self.stop_time_seconds.is_finite()
            || !self.sample_interval_seconds.is_finite()
            || self.start_time_seconds < 0.0
            || self.stop_time_seconds <= self.start_time_seconds
            || self.sample_interval_seconds <= 0.0
        {
            return Err(invalid("FFT sampling metadata is invalid"));
        }
        let expected = (self.stop_time_seconds - self.start_time_seconds) / self.point_count as f64;
        if !approximately_equal(self.sample_interval_seconds, expected) {
            return Err(invalid(
                "FFT sample interval is inconsistent with its time span and point count",
            ));
        }
        Ok(())
    }
}

/// Effective transform controls after dialect defaults and normalization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FftTransformDocument {
    pub format: FftCoefficientFormat,
    pub mode: FftCompatibilityMode,
    pub window: FftWindowKind,
    /// Parser-retained spelling used by compatible report headers.
    pub window_name: String,
    pub alpha: f64,
    pub coherent_gain: f64,
    pub frequency_resolution_hertz: f64,
    pub fundamental_bin: usize,
    pub minimum_metric_bin: usize,
    pub maximum_metric_bin: usize,
    /// Effective inclusive lower bin used to search for the SFDR spur. This
    /// differs from FMIN when FMIN was omitted and FREQ selects a later bin.
    pub sfdr_search_minimum_bin: usize,
}

impl FftTransformDocument {
    fn validate(
        &self,
        point_count: usize,
        abort: &dyn AbortSignal,
    ) -> Result<(), FftResultDocumentError> {
        let last_bin = point_count / 2;
        if !window_name_matches(self.window, &self.window_name)
            || !self.alpha.is_finite()
            || !(1.0..=20.0).contains(&self.alpha)
            || !self.coherent_gain.is_finite()
            || self.coherent_gain <= 0.0
            || !self.frequency_resolution_hertz.is_finite()
            || self.frequency_resolution_hertz <= 0.0
            || self.fundamental_bin == 0
            || self.fundamental_bin > last_bin
            || self.minimum_metric_bin > self.maximum_metric_bin
            || self.maximum_metric_bin > last_bin
            || self.sfdr_search_minimum_bin > self.maximum_metric_bin
            || (self.fundamental_bin == 1 && self.maximum_metric_bin < 2)
            || (self.fundamental_bin > 1 && self.maximum_metric_bin < 1)
        {
            return Err(invalid("FFT transform metadata is invalid"));
        }
        let expected_coherent_gain = transient_fft_window_coherent_gain(
            self.window.core_window(),
            self.mode.core_mode(),
            point_count,
            abort,
        )
        .map_err(|error| {
            if matches!(error, rspice_core::engine::SimulationError::Aborted) {
                FftResultDocumentError::Aborted
            } else {
                invalid(&format!("cannot validate FFT coherent gain: {error}"))
            }
        })?;
        if !approximately_equal(self.coherent_gain, expected_coherent_gain) {
            return Err(invalid(
                "FFT coherent gain does not match its window and compatibility mode",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FftCoefficientFormat {
    Normalized,
    Unnormalized,
}

impl From<FftFormat> for FftCoefficientFormat {
    fn from(value: FftFormat) -> Self {
        match value {
            FftFormat::Normalized => Self::Normalized,
            FftFormat::Unnormalized => Self::Unnormalized,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FftCompatibilityMode {
    HspiceCompatible,
    SpectreCompatible,
}

impl From<XyceFftMode> for FftCompatibilityMode {
    fn from(value: XyceFftMode) -> Self {
        match value {
            XyceFftMode::HspiceCompatible => Self::HspiceCompatible,
            XyceFftMode::SpectreCompatible => Self::SpectreCompatible,
        }
    }
}

impl FftCompatibilityMode {
    const fn core_mode(self) -> XyceFftMode {
        match self {
            Self::HspiceCompatible => XyceFftMode::HspiceCompatible,
            Self::SpectreCompatible => XyceFftMode::SpectreCompatible,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FftWindowKind {
    Rectangular,
    Bartlett,
    BartlettHann,
    Hamming,
    Hann,
    Blackman67Db,
    Blackman,
    BlackmanHarris,
    Nuttall,
    HalfCycleSine,
    HalfCycleSine3,
    HalfCycleSine6,
    Cosine2,
    Cosine4,
}

impl From<FftWindow> for FftWindowKind {
    fn from(value: FftWindow) -> Self {
        match value {
            FftWindow::Rectangular => Self::Rectangular,
            FftWindow::Bartlett => Self::Bartlett,
            FftWindow::BartlettHann => Self::BartlettHann,
            FftWindow::Hamming => Self::Hamming,
            FftWindow::Hann => Self::Hann,
            FftWindow::Blackman67Db => Self::Blackman67Db,
            FftWindow::Blackman => Self::Blackman,
            FftWindow::BlackmanHarris => Self::BlackmanHarris,
            FftWindow::Nuttall => Self::Nuttall,
            FftWindow::HalfCycleSine => Self::HalfCycleSine,
            FftWindow::HalfCycleSine3 => Self::HalfCycleSine3,
            FftWindow::HalfCycleSine6 => Self::HalfCycleSine6,
            FftWindow::Cosine2 => Self::Cosine2,
            FftWindow::Cosine4 => Self::Cosine4,
        }
    }
}

impl FftWindowKind {
    const fn core_window(self) -> FftWindow {
        match self {
            Self::Rectangular => FftWindow::Rectangular,
            Self::Bartlett => FftWindow::Bartlett,
            Self::BartlettHann => FftWindow::BartlettHann,
            Self::Hamming => FftWindow::Hamming,
            Self::Hann => FftWindow::Hann,
            Self::Blackman67Db => FftWindow::Blackman67Db,
            Self::Blackman => FftWindow::Blackman,
            Self::BlackmanHarris => FftWindow::BlackmanHarris,
            Self::Nuttall => FftWindow::Nuttall,
            Self::HalfCycleSine => FftWindow::HalfCycleSine,
            Self::HalfCycleSine3 => FftWindow::HalfCycleSine3,
            Self::HalfCycleSine6 => FftWindow::HalfCycleSine6,
            Self::Cosine2 => FftWindow::Cosine2,
            Self::Cosine4 => FftWindow::Cosine4,
        }
    }
}

/// One-sided DC-through-Nyquist spectrum.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FftSpectrumDocument {
    pub frequency_unit: FftUnit,
    pub value_unit: Option<FftUnit>,
    pub phase_unit: FftUnit,
    pub complex_representation: FftComplexRepresentation,
    pub bins: Vec<FftBinDocument>,
}

impl FftSpectrumDocument {
    fn validate(
        &self,
        point_count: usize,
        expected_value_unit: Option<FftUnit>,
        transform: &FftTransformDocument,
        abort: &dyn AbortSignal,
    ) -> Result<(), FftResultDocumentError> {
        if self.frequency_unit != FftUnit::Hertz
            || self.value_unit != expected_value_unit
            || self.phase_unit != FftUnit::Degree
            || self.complex_representation != FftComplexRepresentation::Cartesian
            || self.bins.len() != point_count / 2 + 1
        {
            return Err(invalid(
                "FFT spectrum units, representation, or shape is invalid",
            ));
        }
        for (index, bin) in self.bins.iter().enumerate() {
            if index.is_multiple_of(256) {
                check_abort(abort)?;
            }
            if bin.index != index || !bin.is_finite() || bin.frequency_hertz < 0.0 {
                return Err(invalid("FFT bins are not finite and contiguous"));
            }
            if !approximately_equal(
                bin.frequency_hertz,
                index as f64 * transform.frequency_resolution_hertz,
            ) || !(-180.0..=180.0).contains(&bin.phase_degrees)
                || bin.magnitude < 0.0
                || !approximately_equal(bin.magnitude, bin.real.hypot(bin.imaginary))
                || !angles_equal(
                    bin.phase_degrees,
                    bin.imaginary.atan2(bin.real).to_degrees(),
                )
            {
                return Err(invalid("FFT bin calibration is internally inconsistent"));
            }
        }
        if transform.format == FftCoefficientFormat::Normalized {
            let largest = self
                .bins
                .iter()
                .map(|bin| bin.magnitude)
                .fold(0.0, f64::max);
            if largest > 0.0 && !approximately_equal(largest, 1.0) {
                return Err(invalid(
                    "normalized FFT spectrum does not have unit peak magnitude",
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FftComplexRepresentation {
    Cartesian,
}

/// Unit vocabulary needed by spectra and distortion metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FftUnit {
    #[serde(rename = "Hz")]
    Hertz,
    #[serde(rename = "V")]
    Volt,
    #[serde(rename = "A")]
    Ampere,
    #[serde(rename = "degree")]
    Degree,
    #[serde(rename = "1")]
    Dimensionless,
    #[serde(rename = "dB")]
    Decibel,
    #[serde(rename = "bit")]
    Bit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FftBinDocument {
    pub index: usize,
    pub frequency_hertz: f64,
    pub real: f64,
    pub imaginary: f64,
    pub magnitude: f64,
    pub phase_degrees: f64,
}

impl FftBinDocument {
    fn is_finite(&self) -> bool {
        self.frequency_hertz.is_finite()
            && self.real.is_finite()
            && self.imaginary.is_finite()
            && self.magnitude.is_finite()
            && self.phase_degrees.is_finite()
    }
}

impl From<&TransientFftBin> for FftBinDocument {
    fn from(bin: &TransientFftBin) -> Self {
        Self {
            index: bin.index,
            frequency_hertz: bin.frequency,
            real: bin.real,
            imaginary: bin.imaginary,
            magnitude: bin.magnitude,
            phase_degrees: bin.phase_degrees,
        }
    }
}

/// Explicit units for every optional FFTOUT metric.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FftMetricUnits {
    pub fundamental_magnitude: Option<FftUnit>,
    pub thd_ratio: FftUnit,
    pub thd_db: FftUnit,
    pub sndr_db: FftUnit,
    pub enob_bits: FftUnit,
    pub snr_db: FftUnit,
    pub sfdr_db: FftUnit,
    pub sfdr_spur_frequency: FftUnit,
}

/// Xyce-compatible figures requested by `FFTOUT=1`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FftMetricsDocument {
    pub units: FftMetricUnits,
    pub fundamental_magnitude: f64,
    pub thd_ratio: f64,
    pub thd_db: f64,
    pub sndr_db: f64,
    pub enob_bits: f64,
    pub snr_db: f64,
    pub sfdr_db: f64,
    pub sfdr_spur_bin: Option<usize>,
    pub sfdr_spur_frequency_hertz: Option<f64>,
    pub largest_harmonics: Vec<FftHarmonicDocument>,
}

impl FftMetricsDocument {
    fn from_engine(
        metrics: &TransientFftMetrics,
        value_unit: Option<FftUnit>,
        abort: &dyn AbortSignal,
    ) -> Result<Self, FftResultDocumentError> {
        let mut largest_harmonics = Vec::new();
        largest_harmonics
            .try_reserve_exact(metrics.largest_harmonics.len())
            .map_err(|_| invalid("cannot allocate typed FFT ranked harmonics"))?;
        for harmonic in &metrics.largest_harmonics {
            check_abort(abort)?;
            largest_harmonics.push(FftHarmonicDocument::from(harmonic));
        }
        Ok(Self {
            units: FftMetricUnits {
                fundamental_magnitude: value_unit,
                thd_ratio: FftUnit::Dimensionless,
                thd_db: FftUnit::Decibel,
                sndr_db: FftUnit::Decibel,
                enob_bits: FftUnit::Bit,
                snr_db: FftUnit::Decibel,
                sfdr_db: FftUnit::Decibel,
                sfdr_spur_frequency: FftUnit::Hertz,
            },
            fundamental_magnitude: metrics.fundamental_magnitude,
            thd_ratio: metrics.thd_ratio,
            thd_db: metrics.thd_db,
            sndr_db: metrics.sndr_db,
            enob_bits: metrics.enob_bits,
            snr_db: metrics.snr_db,
            sfdr_db: metrics.sfdr_db,
            sfdr_spur_bin: metrics.sfdr_spur_bin,
            sfdr_spur_frequency_hertz: metrics.sfdr_spur_frequency,
            largest_harmonics,
        })
    }

    fn validate(
        &self,
        value_unit: Option<FftUnit>,
        bins: &[FftBinDocument],
        transform: &FftTransformDocument,
        abort: &dyn AbortSignal,
    ) -> Result<(), FftResultDocumentError> {
        let expected_units = FftMetricUnits {
            fundamental_magnitude: value_unit,
            thd_ratio: FftUnit::Dimensionless,
            thd_db: FftUnit::Decibel,
            sndr_db: FftUnit::Decibel,
            enob_bits: FftUnit::Bit,
            snr_db: FftUnit::Decibel,
            sfdr_db: FftUnit::Decibel,
            sfdr_spur_frequency: FftUnit::Hertz,
        };
        if self.units != expected_units
            || ![
                self.fundamental_magnitude,
                self.thd_ratio,
                self.thd_db,
                self.sndr_db,
                self.enob_bits,
                self.snr_db,
                self.sfdr_db,
            ]
            .iter()
            .all(|value| value.is_finite())
            || self.fundamental_magnitude <= FFT_DB_REPORTING_FLOOR
            || self.thd_ratio < 0.0
            || self
                .sfdr_spur_frequency_hertz
                .is_some_and(|value| !value.is_finite())
            || self.sfdr_spur_bin.is_some() != self.sfdr_spur_frequency_hertz.is_some()
            || self.largest_harmonics.len()
                != bins.len().saturating_sub(1).min(MAX_RANKED_HARMONICS)
        {
            return Err(invalid("FFT metric values or units are invalid"));
        }
        let Some(fundamental) = bins.get(transform.fundamental_bin) else {
            return Err(invalid(
                "FFT fundamental metric bin is outside the spectrum",
            ));
        };
        if !approximately_equal(self.fundamental_magnitude, fundamental.magnitude) {
            return Err(invalid(
                "FFT fundamental metric magnitude does not match its spectrum bin",
            ));
        }
        let mut distortion_power = 0.0;
        for bin in (transform.fundamental_bin.saturating_mul(2)..=transform.maximum_metric_bin)
            .step_by(transform.fundamental_bin)
        {
            if bin.is_multiple_of(256) {
                check_abort(abort)?;
            }
            distortion_power += bins[bin].magnitude.powi(2);
        }
        let expected_thd_ratio = distortion_power.sqrt() / fundamental.magnitude;
        let expected_thd_db = 20.0 * expected_thd_ratio.max(FFT_DB_REPORTING_FLOOR).log10();

        let mut noise_and_distortion_power = 0.0;
        for (index, bin) in bins.iter().enumerate().skip(1) {
            if index.is_multiple_of(256) {
                check_abort(abort)?;
            }
            if index != transform.fundamental_bin {
                noise_and_distortion_power += bin.magnitude.powi(2);
            }
        }
        let expected_sndr_db = 20.0
            * (fundamental.magnitude
                / noise_and_distortion_power
                    .sqrt()
                    .max(FFT_DB_REPORTING_FLOOR))
            .log10();
        let expected_enob_bits = (expected_sndr_db - 1.76) / 6.02;

        let signal_frequency_limit = transform.maximum_metric_bin.max(transform.fundamental_bin);
        let mut noise_power = 0.0;
        for (index, bin) in bins.iter().enumerate().skip(1) {
            if index.is_multiple_of(256) {
                check_abort(abort)?;
            }
            if index % transform.fundamental_bin != 0 || index > signal_frequency_limit {
                noise_power += bin.magnitude.powi(2);
            }
        }
        let expected_snr_db =
            20.0 * (fundamental.magnitude / noise_power.sqrt().max(FFT_DB_REPORTING_FLOOR)).log10();

        let mut expected_spur: Option<&FftBinDocument> = None;
        for (index, bin) in bins
            .iter()
            .enumerate()
            .take(transform.maximum_metric_bin + 1)
            .skip(transform.sfdr_search_minimum_bin)
        {
            if index.is_multiple_of(256) {
                check_abort(abort)?;
            }
            if index != transform.fundamental_bin
                && bin.magnitude > expected_spur.map_or(0.0, |spur| spur.magnitude)
            {
                expected_spur = Some(bin);
            }
        }
        let expected_sfdr_db = 20.0
            * (fundamental.magnitude
                / expected_spur
                    .map_or(0.0, |spur| spur.magnitude)
                    .max(FFT_DB_REPORTING_FLOOR))
            .log10();
        if !approximately_equal(self.thd_ratio, expected_thd_ratio)
            || !approximately_equal(self.thd_db, expected_thd_db)
            || !approximately_equal(self.sndr_db, expected_sndr_db)
            || !approximately_equal(self.enob_bits, expected_enob_bits)
            || !approximately_equal(self.snr_db, expected_snr_db)
            || !approximately_equal(self.sfdr_db, expected_sfdr_db)
            || self.sfdr_spur_bin != expected_spur.map(|spur| spur.index)
            || self.sfdr_spur_frequency_hertz != expected_spur.map(|spur| spur.frequency_hertz)
        {
            return Err(invalid(
                "FFT distortion metrics are inconsistent with the published spectrum",
            ));
        }
        let expected_len = bins.len().saturating_sub(1).min(MAX_RANKED_HARMONICS);
        let mut expected_harmonic_bins: Vec<usize> = Vec::new();
        expected_harmonic_bins
            .try_reserve_exact(expected_len)
            .map_err(|_| invalid("cannot allocate FFT harmonic validation state"))?;
        for index in 1..bins.len() {
            if index.is_multiple_of(256) {
                check_abort(abort)?;
            }
            let position = expected_harmonic_bins
                .iter()
                .position(|retained| {
                    bins[index].magnitude > bins[*retained].magnitude
                        || (bins[index].magnitude == bins[*retained].magnitude && index < *retained)
                })
                .unwrap_or(expected_harmonic_bins.len());
            if expected_harmonic_bins.len() < expected_len {
                expected_harmonic_bins.insert(position, index);
            } else if position < expected_len {
                expected_harmonic_bins.pop();
                expected_harmonic_bins.insert(position, index);
            }
        }
        let mut harmonic_bins = HashSet::new();
        for (index, harmonic) in self.largest_harmonics.iter().enumerate() {
            if index.is_multiple_of(256) {
                check_abort(abort)?;
            }
            let rank = index + 1;
            let Some(bin) = bins.get(harmonic.bin) else {
                return Err(invalid("FFT ranked harmonic is outside the spectrum"));
            };
            if harmonic.rank != rank
                || harmonic.bin == 0
                || !harmonic_bins.insert(harmonic.bin)
                || harmonic.bin != expected_harmonic_bins[index]
                || !harmonic.is_finite()
                || harmonic.magnitude < 0.0
                || !approximately_equal(harmonic.frequency_hertz, bin.frequency_hertz)
                || !approximately_equal(harmonic.magnitude, bin.magnitude)
                || !approximately_equal(
                    harmonic.magnitude_db,
                    20.0 * harmonic.magnitude.max(FFT_DB_REPORTING_FLOOR).log10(),
                )
                || !approximately_equal(harmonic.phase_degrees, bin.phase_degrees)
            {
                return Err(invalid("FFT ranked harmonic metadata is inconsistent"));
            }
            if index > 0 {
                let prior = &self.largest_harmonics[index - 1];
                if harmonic.magnitude > prior.magnitude
                    || (harmonic.magnitude == prior.magnitude && harmonic.bin < prior.bin)
                {
                    return Err(invalid(
                        "FFT ranked harmonics are not in stable magnitude order",
                    ));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FftHarmonicDocument {
    pub rank: usize,
    pub bin: usize,
    pub frequency_hertz: f64,
    pub magnitude: f64,
    pub magnitude_db: f64,
    pub phase_degrees: f64,
}

impl FftHarmonicDocument {
    fn is_finite(&self) -> bool {
        self.frequency_hertz.is_finite()
            && self.magnitude.is_finite()
            && self.magnitude_db.is_finite()
            && self.phase_degrees.is_finite()
    }
}

impl From<&TransientFftHarmonic> for FftHarmonicDocument {
    fn from(harmonic: &TransientFftHarmonic) -> Self {
        Self {
            rank: harmonic.rank,
            bin: harmonic.bin,
            frequency_hertz: harmonic.frequency,
            magnitude: harmonic.magnitude,
            magnitude_db: harmonic.magnitude_db,
            phase_degrees: harmonic.phase_degrees,
        }
    }
}

fn approximately_equal(left: f64, right: f64) -> bool {
    let scale = left.abs().max(right.abs());
    if scale == 0.0 {
        left == right
    } else {
        (left - right).abs() <= 1.0e-12 * scale
    }
}

fn angles_equal(left: f64, right: f64) -> bool {
    let difference = (left - right + 180.0).rem_euclid(360.0) - 180.0;
    difference.abs() <= 1.0e-10
}

fn authored_frequency_bin(
    requested: Option<f64>,
    default: usize,
    resolution: f64,
    last_bin: usize,
    allow_zero: bool,
) -> Result<usize, FftResultDocumentError> {
    let Some(requested) = requested else {
        return Ok(default);
    };
    let rounded = (requested / resolution).round();
    if !rounded.is_finite() || rounded < 0.0 || rounded > last_bin as f64 {
        return Err(invalid("authored FFT frequency is outside the spectrum"));
    }
    let bin = rounded as usize;
    if bin == 0 && !allow_zero {
        return Err(invalid("authored FFT frequency rounds below bin one"));
    }
    Ok(bin)
}

fn window_name_matches(window: FftWindowKind, name: &str) -> bool {
    matches!(
        (window, name),
        (FftWindowKind::Rectangular, "RECT" | "RECTANGULAR")
            | (FftWindowKind::Bartlett, "BART" | "BARTLETT")
            | (FftWindowKind::BartlettHann, "BARTLETTHANN")
            | (FftWindowKind::Hamming, "HAMM" | "HAMMING")
            | (FftWindowKind::Hann, "HANN" | "HANNING")
            | (FftWindowKind::Blackman67Db, "BLACK")
            | (FftWindowKind::Blackman, "BLACKMAN")
            | (FftWindowKind::BlackmanHarris, "HARRIS" | "BLACKMANHARRIS")
            | (FftWindowKind::Nuttall, "NUTTALL")
            | (FftWindowKind::HalfCycleSine, "HALFCYCLESINE")
            | (FftWindowKind::HalfCycleSine3, "HALFCYCLESINE3")
            | (FftWindowKind::HalfCycleSine6, "HALFCYCLESINE6")
            | (FftWindowKind::Cosine2, "COSINE2")
            | (FftWindowKind::Cosine4, "COSINE4")
    )
}

/// Whether a string is the canonical tag of an authored transient analysis.
///
/// The shape is `AnalysisInstanceId::tag`'s: the kind tag, a hyphen, and a
/// one-based ordinal of at least three digits with no leading zero beyond the
/// fixed width. Checking it here keeps the parent link verifiable by a reader
/// that never saw the plan.
fn valid_transient_analysis_tag(tag: &str) -> bool {
    let Some(ordinal) = tag.strip_prefix("tran-") else {
        return false;
    };
    ordinal.len() >= 3
        && ordinal.bytes().all(|byte| byte.is_ascii_digit())
        && (ordinal.len() == 3 || !ordinal.starts_with('0'))
        && ordinal.parse::<u64>().is_ok_and(|value| value >= 1)
}

fn check_abort(abort: &dyn AbortSignal) -> Result<(), FftResultDocumentError> {
    if abort.is_aborted() {
        Err(FftResultDocumentError::Aborted)
    } else {
        Ok(())
    }
}

fn invalid(message: &str) -> FftResultDocumentError {
    FftResultDocumentError::InvalidDocument(message.to_owned())
}

#[derive(Debug, Error)]
pub enum FftResultDocumentError {
    #[error("invalid FFT result JSON: {0}")]
    InvalidJson(#[source] serde_json::Error),
    #[error("unexpected FFT result schema {0:?}")]
    WrongSchema(String),
    #[error("FFT result schema version {found} is unsupported (current version is {current})")]
    UnsupportedVersion { found: u32, current: u32 },
    #[error("invalid FFT result document: {0}")]
    InvalidDocument(String),
    #[error("FFT result mapping or serialization was cancelled")]
    Aborted,
    #[error("FFT result artifact exceeds the {limit_bytes}-byte limit")]
    ArtifactTooLarge { limit_bytes: u64 },
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::{CountingAbort, NoAbort};
    use rspice_core::{Engine, Netlist, SimulationConfig};

    fn parent() -> String {
        "tran-002".to_owned()
    }

    #[test]
    fn only_a_canonical_transient_tag_names_a_parent() {
        for accepted in ["tran-001", "tran-002", "tran-1000"] {
            assert!(valid_transient_analysis_tag(accepted), "{accepted}");
        }
        for refused in [
            "tran-00",
            "tran-000",
            "tran-0001",
            "ac-001",
            "tran-",
            "tran-01a",
        ] {
            assert!(!valid_transient_analysis_tag(refused), "{refused}");
        }
    }

    fn engine_result(output: FftOutput, point_count: usize) -> TransientFftResult {
        let resolution = 10.0;
        let bins = (0..=point_count / 2)
            .map(|index| {
                let real = index as f64 * 0.25;
                let imaginary = -(index as f64) * 0.125;
                TransientFftBin {
                    index,
                    frequency: index as f64 * resolution,
                    real,
                    imaginary,
                    magnitude: real.hypot(imaginary),
                    phase_degrees: imaginary.atan2(real).to_degrees(),
                }
            })
            .collect();
        TransientFftResult {
            output,
            output_name: "V(out)".to_owned(),
            physical_type: "voltage",
            start_time: 0.0,
            stop_time: 0.1,
            sample_interval: 0.1 / point_count as f64,
            point_count,
            accurate_sampling: true,
            format: FftFormat::Unnormalized,
            mode: XyceFftMode::HspiceCompatible,
            window: FftWindow::Hann,
            window_name: "HANN".to_owned(),
            alpha: 3.0,
            coherent_gain: transient_fft_window_coherent_gain(
                FftWindow::Hann,
                XyceFftMode::HspiceCompatible,
                point_count,
                &NoAbort,
            )
            .expect("fixture coherent gain"),
            frequency_resolution: resolution,
            fundamental_bin: 1,
            minimum_metric_bin: 1,
            maximum_metric_bin: point_count / 2,
            bins,
            metrics: None,
        }
    }

    fn authored(result: &TransientFftResult) -> FftAnalysis {
        FftAnalysis {
            output: result.output.clone(),
            start: Some(result.start_time),
            stop: Some(result.stop_time),
            points: result.point_count,
            format: Some(result.format),
            window: result.window,
            window_name: result.window_name.clone(),
            alpha: result.alpha,
            fundamental_frequency: None,
            minimum_frequency: None,
            maximum_frequency: None,
        }
    }

    fn metric_document(fmin: Option<f64>) -> TransientFftResultDocument {
        let fmin = fmin.map_or_else(String::new, |value| format!(" fmin={value}"));
        let deck = format!(
            "metric validation\n\
             V1 out 0 SIN(0 1 2k)\n\
             R1 out 0 1k\n\
             .options fft fftout=1\n\
             .tran 1u 1m\n\
             .fft v(out) np=16 format=unorm window=rect freq=2k{fmin} fmax=4k\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("metric FFT fixture parses");
        let result = Engine::new(SimulationConfig::default())
            .run_tran_with_abort(&netlist, 1.0e-3, 1.0e-6, &NoAbort)
            .expect("metric FFT fixture executes");
        TransientFftResultDocument::from_engine_results(
            parent(),
            &result.fft_results,
            &netlist.fft_analyses,
            netlist.options.fft_mode.unwrap_or_default(),
        )
        .expect("metric FFT document maps")
    }

    #[test]
    fn current_schema_round_trips_ordered_ragged_spectra_and_source_identity() {
        let first = engine_result(FftOutput::Probe("V(out)".to_owned()), 8);
        let mut second = engine_result(FftOutput::Expression("V(out)*2".to_owned()), 16);
        second.output_name = "{V(out)*2}".to_owned();
        second.physical_type = "parameter";
        second.format = FftFormat::Normalized;
        let peak = second
            .bins
            .iter()
            .map(|bin| bin.magnitude)
            .fold(0.0, f64::max);
        for bin in &mut second.bins {
            bin.real /= peak;
            bin.imaginary /= peak;
            bin.magnitude /= peak;
        }
        let authored = [authored(&first), authored(&second)];

        let mut document = TransientFftResultDocument::from_engine_results(
            parent(),
            &[first.clone(), second.clone()],
            &authored,
            XyceFftMode::HspiceCompatible,
        )
        .expect("map FFT bundle");
        for result in &mut document.results {
            result.parent_analysis_id = document.parent_analysis.clone();
        }
        document.validate().expect("complete bundle validates");
        let json = document.to_json().expect("serialize FFT bundle");
        let decoded = TransientFftResultDocument::from_json(&json).expect("decode FFT bundle");

        assert_eq!(decoded, document);
        assert_eq!(decoded.results[0].analysis_id, "fft-001");
        assert_eq!(decoded.results[1].analysis_id, "fft-002");
        assert_eq!(decoded.results[0].spectrum.bins.len(), 5);
        assert_eq!(decoded.results[1].spectrum.bins.len(), 9);
        assert_eq!(decoded.results[0].source.authored_output, "V(out)");
        assert_eq!(decoded.results[1].source.authored_output, "{V(out)*2}");
        assert_eq!(decoded.results[0].signal.unit, Some(FftUnit::Volt));
        assert_eq!(decoded.results[1].signal.unit, Some(FftUnit::Dimensionless));
    }

    #[test]
    fn future_versions_and_lossy_physical_type_mappings_fail_closed() {
        let future = r#"{
            "schema":"rspice-transient-fft-result",
            "schema_version":2,
            "future_required_field":true
        }"#;
        assert!(matches!(
            TransientFftResultDocument::from_json(future),
            Err(FftResultDocumentError::UnsupportedVersion {
                found: 2,
                current: 1
            })
        ));

        let mut unsupported = engine_result(FftOutput::Probe("V(out)".to_owned()), 8);
        unsupported.physical_type = "power";
        let authored = authored(&unsupported);
        assert!(matches!(
            TransientFftResultDocument::from_engine_results(
                parent(),
                &[unsupported],
                &[authored],
                XyceFftMode::HspiceCompatible,
            ),
            Err(FftResultDocumentError::InvalidDocument(_))
        ));
    }

    #[test]
    fn malformed_shapes_units_and_metric_references_fail_closed() {
        let result = engine_result(FftOutput::Probe("V(out)".to_owned()), 8);
        let authored = authored(&result);
        let mut document = TransientFftResultDocument::from_engine_results(
            parent(),
            &[result],
            &[authored],
            XyceFftMode::HspiceCompatible,
        )
        .unwrap();
        document.results[0].parent_analysis_id = document.parent_analysis.clone();
        document.results[0].spectrum.bins.pop();
        assert!(matches!(
            document.validate(),
            Err(FftResultDocumentError::InvalidDocument(_))
        ));
    }

    #[test]
    fn normalized_and_unnormalized_units_are_distinct_and_strict() {
        let mut unnormalized = engine_result(FftOutput::Probe("V(out)".to_owned()), 8);
        let unnormalized_authored = authored(&unnormalized);
        let mut unnormalized_document = TransientFftResultDocument::from_engine_results(
            parent(),
            &[unnormalized.clone()],
            &[unnormalized_authored],
            XyceFftMode::HspiceCompatible,
        )
        .unwrap();
        assert_eq!(
            unnormalized_document.results[0].signal.unit,
            Some(FftUnit::Volt)
        );
        unnormalized_document.results[0].signal.unit = Some(FftUnit::Dimensionless);
        assert!(unnormalized_document.validate().is_err());

        unnormalized.format = FftFormat::Normalized;
        let peak = unnormalized
            .bins
            .iter()
            .map(|bin| bin.magnitude)
            .fold(0.0, f64::max);
        for bin in &mut unnormalized.bins {
            bin.real /= peak;
            bin.imaginary /= peak;
            bin.magnitude /= peak;
        }
        let normalized_authored = authored(&unnormalized);
        let mut normalized_document = TransientFftResultDocument::from_engine_results(
            parent(),
            &[unnormalized],
            &[normalized_authored],
            XyceFftMode::HspiceCompatible,
        )
        .unwrap();
        assert_eq!(
            normalized_document.results[0].signal.unit,
            Some(FftUnit::Dimensionless)
        );
        normalized_document.results[0].signal.unit = Some(FftUnit::Volt);
        assert!(normalized_document.validate().is_err());
    }

    #[test]
    fn strict_engine_invariants_reject_malformed_transform_and_spectrum_claims() {
        let mut normalized = engine_result(FftOutput::Probe("V(out)".to_owned()), 8);
        normalized.format = FftFormat::Normalized;
        let peak = normalized
            .bins
            .iter()
            .map(|bin| bin.magnitude)
            .fold(0.0, f64::max);
        for bin in &mut normalized.bins {
            bin.real /= peak;
            bin.imaginary /= peak;
            bin.magnitude /= peak;
        }
        let authored = authored(&normalized);
        let base = TransientFftResultDocument::from_engine_results(
            parent(),
            &[normalized],
            &[authored],
            XyceFftMode::HspiceCompatible,
        )
        .unwrap();

        let mut malformed = base.clone();
        malformed.results[0].sampling.point_count = 6;
        assert!(malformed.validate().is_err());
        let mut malformed = base.clone();
        malformed.results[0].sampling.point_count = 1usize << 30;
        malformed.results[0].sampling.sample_interval_seconds =
            (malformed.results[0].sampling.stop_time_seconds
                - malformed.results[0].sampling.start_time_seconds)
                / malformed.results[0].sampling.point_count as f64;
        malformed.results[0].authored.point_count = malformed.results[0].sampling.point_count;
        assert!(malformed.validate().is_err());
        let mut malformed = base.clone();
        malformed.results[0].transform.fundamental_bin = 0;
        assert!(malformed.validate().is_err());
        let mut malformed = base.clone();
        malformed.results[0].transform.alpha = 20.000_001;
        assert!(malformed.validate().is_err());
        let mut malformed = base.clone();
        malformed.results[0].transform.window_name = "RECT".to_owned();
        assert!(malformed.validate().is_err());
        let mut malformed = base.clone();
        malformed.results[0].transform.mode = FftCompatibilityMode::SpectreCompatible;
        assert!(malformed.validate().is_err());
        let mut malformed = base.clone();
        malformed.results[0].transform.coherent_gain += 0.125;
        assert!(malformed.validate().is_err());
        let mut malformed = base.clone();
        malformed.results[0].spectrum.bins[0].magnitude = -1.0e-300;
        assert!(malformed.validate().is_err());
        let mut malformed = base;
        for bin in &mut malformed.results[0].spectrum.bins {
            bin.real *= 0.5;
            bin.imaginary *= 0.5;
            bin.magnitude *= 0.5;
        }
        assert!(malformed.validate().is_err());
    }

    #[test]
    fn authored_controls_disambiguate_repeated_outputs_and_reject_mismatches() {
        let first = engine_result(FftOutput::Probe("V(out)".to_owned()), 8);
        let mut second = engine_result(FftOutput::Probe("V(out)".to_owned()), 16);
        second.window = FftWindow::Rectangular;
        second.window_name = "RECT".to_owned();
        second.coherent_gain = 1.0;
        let authored = [authored(&first), authored(&second)];
        let document = TransientFftResultDocument::from_engine_results(
            parent(),
            &[first.clone(), second.clone()],
            &authored,
            XyceFftMode::HspiceCompatible,
        )
        .expect("repeated output controls remain ordered");
        assert_eq!(document.results[0].authored.point_count, 8);
        assert_eq!(document.results[1].authored.point_count, 16);
        assert_eq!(
            document.results[1].authored.window,
            FftWindowKind::Rectangular
        );

        let mut mismatched = authored;
        mismatched[1].points = 8;
        assert!(
            TransientFftResultDocument::from_engine_results(
                parent(),
                &[first, second],
                &mismatched,
                XyceFftMode::HspiceCompatible,
            )
            .is_err()
        );
    }

    #[test]
    fn metric_families_are_recomputed_and_fmin_provenance_controls_sfdr_search() {
        let absent = metric_document(None);
        assert_eq!(absent.results[0].transform.fundamental_bin, 2);
        assert_eq!(absent.results[0].transform.sfdr_search_minimum_bin, 2);
        let explicit_zero = metric_document(Some(0.0));
        assert_eq!(explicit_zero.results[0].transform.minimum_metric_bin, 0);
        assert_eq!(
            explicit_zero.results[0].transform.sfdr_search_minimum_bin,
            0
        );

        let reject = |document: TransientFftResultDocument| {
            assert!(document.validate().is_err(), "metric mutation was accepted");
        };
        let mut mutated = absent.clone();
        mutated.results[0].metrics.as_mut().unwrap().thd_ratio += 0.01;
        reject(mutated);
        let mut mutated = absent.clone();
        mutated.results[0].metrics.as_mut().unwrap().thd_db += 1.0;
        reject(mutated);
        let mut mutated = absent.clone();
        mutated.results[0].metrics.as_mut().unwrap().sndr_db += 1.0;
        reject(mutated);
        let mut mutated = absent.clone();
        mutated.results[0].metrics.as_mut().unwrap().enob_bits += 1.0;
        reject(mutated);
        let mut mutated = absent.clone();
        mutated.results[0].metrics.as_mut().unwrap().snr_db += 1.0;
        reject(mutated);
        let mut mutated = absent.clone();
        mutated.results[0].metrics.as_mut().unwrap().sfdr_db += 1.0;
        reject(mutated);
        let mut mutated = absent;
        let fundamental = mutated.results[0].transform.fundamental_bin;
        let frequency = mutated.results[0].spectrum.bins[fundamental].frequency_hertz;
        let metrics = mutated.results[0].metrics.as_mut().unwrap();
        metrics.sfdr_spur_bin = Some(fundamental);
        metrics.sfdr_spur_frequency_hertz = Some(frequency);
        reject(mutated);
    }

    #[test]
    fn mapping_serialization_cancellation_and_byte_limits_fail_closed() {
        let result = engine_result(FftOutput::Probe("V(out)".to_owned()), 512);
        let authored = authored(&result);
        let mapping_abort = CountingAbort::new(3);
        assert!(matches!(
            TransientFftResultDocument::from_engine_results_with_abort(
                parent(),
                std::slice::from_ref(&result),
                std::slice::from_ref(&authored),
                XyceFftMode::HspiceCompatible,
                &mapping_abort,
            ),
            Err(FftResultDocumentError::Aborted)
        ));

        let document = TransientFftResultDocument::from_engine_results(
            parent(),
            &[result],
            &[authored],
            XyceFftMode::HspiceCompatible,
        )
        .unwrap();
        let validation_counter = CountingAbort::new(usize::MAX);
        document
            .validate_with_abort(&validation_counter)
            .expect("count validation abort checks");
        let serialization_abort = CountingAbort::new(validation_counter.count() + 2);
        assert!(matches!(
            document.to_json_with_abort(&serialization_abort, u64::MAX),
            Err(FftResultDocumentError::Aborted)
        ));

        let json = document.to_json().unwrap();
        assert!(
            document
                .to_json_with_abort(&NoAbort, json.len() as u64)
                .is_ok()
        );
        assert!(matches!(
            document.to_json_with_abort(&NoAbort, json.len() as u64 - 1),
            Err(FftResultDocumentError::ArtifactTooLarge { .. })
        ));
    }
}
