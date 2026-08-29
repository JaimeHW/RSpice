//! Typed failures produced while constructing an FFT record.

use std::fmt;

use rspice_core::numerics::rustfft_qualification::RustfftQualificationError;
use thiserror::Error;

/// Maximum harmonic order accepted by the synchronous spectrum analyzer.
///
/// This shares the qualified transform-length ceiling: no authenticated
/// one-sided spectrum can contain more independently resolvable orders, and
/// malformed state cannot turn a UI recompute into an unbounded loop.
pub(crate) const MAX_SPECTRUM_HARMONIC_ORDER: usize =
    rspice_core::numerics::rustfft_qualification::MAX_QUALIFIED_RUSTFFT_LENGTH;

/// Minimum record length accepted by the low-level spectrum builder.
///
/// The interactive input pipeline and the direct builder share this boundary,
/// so no lower-level caller can manufacture a spectrum the UI would reject.
pub(crate) const MIN_FFT_DATA_SAMPLES: usize = 16;

/// Explicit allocation site in the FFT construction pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FftAllocationStage {
    /// Owned spectrum label.
    Name,
    /// Window coefficient vector.
    WindowCoefficients,
    /// Windowed complex transform input.
    TransformBuffer,
    /// Caller-owned RustFFT scratch workspace.
    TransformScratch,
    /// One-sided spectrum points.
    SpectrumPoints,
}

/// Explicit allocation site in post-transform spectrum analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpectrumAnalysisAllocationStage {
    /// Bins excluded from broadband-noise metrics.
    NoiseExclusionMask,
    /// Bins excluded while locating the largest spur.
    SpurExclusionMask,
    /// Authenticated harmonic measurements.
    Harmonics,
    /// Per-bin levels used for the median noise floor.
    NoiseLevelBins,
}

impl fmt::Display for SpectrumAnalysisAllocationStage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::NoiseExclusionMask => "spectrum-analysis noise exclusion mask",
            Self::SpurExclusionMask => "spectrum-analysis spur exclusion mask",
            Self::Harmonics => "spectrum-analysis harmonics",
            Self::NoiseLevelBins => "spectrum-analysis noise-level bins",
        })
    }
}

/// A transformed record could not be analyzed without fabricating or
/// silently truncating a spectral metric.
#[derive(Debug, Clone, PartialEq, Error)]
#[non_exhaustive]
pub enum SpectrumAnalysisError {
    /// The requested harmonic order is outside the qualified synchronous
    /// analysis contract.
    #[error("spectrum harmonic order must be between {minimum} and {maximum}; received {value}")]
    InvalidHarmonicOrder {
        /// Rejected order.
        value: usize,
        /// Smallest accepted order.
        minimum: usize,
        /// Largest accepted order.
        maximum: usize,
    },
    /// A top-level spectrum relationship is not valid for analysis.
    #[error("spectrum analysis input invariant failed: {reason}")]
    InvalidSpectrum {
        /// Stable diagnostic for the rejected invariant.
        reason: &'static str,
    },
    /// A stored spectrum point is not a finite physical bin.
    #[error(
        "spectrum analysis rejected bin {bin} (frequency {frequency:?}, magnitude {magnitude:?}, phase {phase:?}): {reason}"
    )]
    InvalidSpectrumPoint {
        /// Rejected bin.
        bin: usize,
        /// Stored frequency.
        frequency: f64,
        /// Stored nonnegative magnitude.
        magnitude: f64,
        /// Stored phase.
        phase: f64,
        /// Stable diagnostic for the rejected relationship.
        reason: &'static str,
    },
    /// A bounded post-transform allocation failed.
    #[error("spectrum analysis could not reserve {requested} units for {stage}")]
    Allocation {
        /// Analysis stage requesting memory.
        stage: SpectrumAnalysisAllocationStage,
        /// Number of additional elements requested.
        requested: usize,
    },
    /// A mathematically finite metric could not be materialized as `f64`.
    #[error(
        "spectrum metric {metric} is not representable (natural-log amplitude ratio {log_amplitude_ratio:?})"
    )]
    UnrepresentableMetric {
        /// Metric being materialized.
        metric: &'static str,
        /// Scale-safe logarithmic ratio used to derive it.
        log_amplitude_ratio: f64,
    },
}

impl fmt::Display for FftAllocationStage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Name => "spectrum name",
            Self::WindowCoefficients => "window coefficients",
            Self::TransformBuffer => "transform buffer",
            Self::TransformScratch => "transform scratch",
            Self::SpectrumPoints => "spectrum points",
        })
    }
}

/// A time-domain record could not be converted into a trustworthy spectrum.
#[derive(Debug, Clone, PartialEq, Error)]
#[non_exhaustive]
pub enum FftBuildError {
    /// The direct builder received too few samples for a useful spectrum.
    #[error("FFT requires at least {minimum} samples; received {length}")]
    InsufficientSamples {
        /// Received record length.
        length: usize,
        /// Required record length.
        minimum: usize,
    },
    /// RustFFT planning would exceed the shared qualified resource policy.
    #[error("FFT record is outside the qualified planning policy: {0}")]
    PlanningQualification(#[from] RustfftQualificationError),
    /// The sampling frequency is unusable.
    #[error("FFT sample rate must be finite and positive; received {sample_rate:?}")]
    InvalidSampleRate {
        /// Rejected sample rate.
        sample_rate: f64,
    },
    /// A time-domain value is NaN or infinite.
    #[error("FFT input sample {index} is non-finite ({value:?})")]
    NonFiniteInputSample {
        /// Position in the input record.
        index: usize,
        /// Rejected value.
        value: f64,
    },
    /// Normalizing an extreme mixed-scale record erased a nonzero input.
    #[error("FFT input normalization erased sample {index} ({value:?} / {scale:?})")]
    ErasedInputScale {
        /// Position in the input record.
        index: usize,
        /// Original nonzero sample.
        value: f64,
        /// Record-wide normalization scale.
        scale: f64,
    },
    /// Applying a nonzero window coefficient erased a normalized sample.
    #[error(
        "FFT window multiplication erased sample {index} ({normalized_sample:?} * {coefficient:?})"
    )]
    ErasedWindowedSample {
        /// Position in the input record.
        index: usize,
        /// Nonzero normalized input.
        normalized_sample: f64,
        /// Nonzero window coefficient.
        coefficient: f64,
    },
    /// A bounded explicit allocation failed.
    #[error("FFT could not reserve {requested} units for {stage}")]
    Allocation {
        /// Construction stage that requested memory.
        stage: FftAllocationStage,
        /// Number of additional elements or bytes requested at that stage.
        requested: usize,
    },
    /// Window generation returned a different length than requested.
    #[error("FFT window generated {actual} coefficients for a {expected}-sample record")]
    WindowLengthMismatch {
        /// Requested transform length.
        expected: usize,
        /// Generated coefficient count.
        actual: usize,
    },
    /// A generated coefficient is NaN or infinite.
    #[error("FFT window coefficient {index} is non-finite ({value:?})")]
    NonFiniteWindowCoefficient {
        /// Position in the generated window.
        index: usize,
        /// Rejected coefficient.
        value: f64,
    },
    /// The actual finite-length window cannot calibrate amplitude and noise.
    #[error(
        "FFT window calibration is invalid (coherent gain {coherent_gain:?}, equivalent noise bandwidth {equivalent_noise_bandwidth_bins:?} bins)"
    )]
    InvalidWindowCalibration {
        /// Mean of the generated coefficients.
        coherent_gain: f64,
        /// Equivalent noise bandwidth of the generated coefficients.
        equivalent_noise_bandwidth_bins: f64,
    },
    /// RustFFT produced a NaN or infinite component.
    #[error("FFT transform bin {bin} is non-finite ({real:?} + j{imaginary:?})")]
    NonFiniteTransformBin {
        /// Transform-bin index.
        bin: usize,
        /// Real component.
        real: f64,
        /// Imaginary component.
        imaginary: f64,
    },
    /// A cached/planned transform does not match the qualified request.
    #[error(
        "FFT plan invariant failed for requested length {requested} (plan {plan_length}, buffer {buffer_length})"
    )]
    PlanInvariant {
        /// Qualified requested transform length.
        requested: usize,
        /// Length reported by the returned plan.
        plan_length: usize,
        /// Prepared complex input length.
        buffer_length: usize,
    },
    /// A positive-rate frequency grid cannot be represented monotonically.
    #[error(
        "FFT frequency bin {bin} is not representable as a finite increasing value (previous {previous_frequency:?}, current {frequency:?})"
    )]
    UnrepresentableFrequency {
        /// One-sided bin index.
        bin: usize,
        /// Previous represented frequency.
        previous_frequency: f64,
        /// Rejected frequency.
        frequency: f64,
    },
    /// A nonzero calibrated magnitude is outside finite `f64` range.
    #[error(
        "FFT magnitude at bin {bin} is not representable (normalized magnitude {normalized_magnitude:?}, source scale {source_scale:?}, numerator scale {numerator_scale:?}, denominator scale {denominator_scale:?})"
    )]
    UnrepresentableMagnitude {
        /// One-sided bin index.
        bin: usize,
        /// Magnitude of the normalized RustFFT coefficient.
        normalized_magnitude: f64,
        /// Record-wide input scale.
        source_scale: f64,
        /// One-sided and normalization multiplier.
        numerator_scale: f64,
        /// Transform-length and coherent-gain divisor.
        denominator_scale: f64,
    },
    /// A calibrated one-sided spectrum point cannot be represented finitely.
    #[error(
        "FFT spectrum bin {bin} is not finitely representable (frequency {frequency:?}, real {real:?}, imaginary {imaginary:?}, magnitude {magnitude:?}, phase {phase:?})"
    )]
    NonFiniteSpectrumPoint {
        /// One-sided spectrum-bin index.
        bin: usize,
        /// Frequency in hertz.
        frequency: f64,
        /// Transform real component used to derive phase.
        real: f64,
        /// Transform imaginary component used to derive phase.
        imaginary: f64,
        /// Linear magnitude.
        magnitude: f64,
        /// Phase in radians.
        phase: f64,
    },
}
