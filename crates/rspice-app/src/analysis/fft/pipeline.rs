//! FFT Input Preparation Pipeline
//!
//! Converts raw time-domain simulation output into a numerically robust,
//! uniformly-sampled sequence suitable for FFT processing.
//!
//! Key steps:
//! - Validate the complete authored record without dropping evidence
//! - Resample variable-step data to a uniform grid
//! - Anti-alias low-pass filter before decimation
//! - Optional strict point cap for responsive UI paths

use std::f64::consts::PI;
use thiserror::Error;

/// Minimum usable sample count for FFT processing.
pub const MIN_FFT_SAMPLES: usize = super::data::MIN_FFT_DATA_SAMPLES;

/// Default point cap for interactive FFT computation.
pub const DEFAULT_MAX_FFT_POINTS: usize = 65_536;

/// Maximum reference-quality FFT preparation point count.
///
/// This keeps memory/time bounded while still preserving far more detail than
/// the interactive cap when users need analysis-grade fidelity.
pub const MAX_REFERENCE_RESAMPLE_POINTS: usize =
    rspice_core::numerics::rustfft_qualification::MAX_QUALIFIED_RUSTFFT_LENGTH;

const GRID_RELATIVE_TOLERANCE: f64 = 1.0e-9;
const MAX_QUALIFIED_DECIMATION_FACTOR: usize = 32;
const FIR_TAPS_PER_DECIMATION_FACTOR: usize = 32;
const MIN_FIR_TAPS: usize = 63;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FftInputAllocationStage {
    Name,
    SelectedSamples,
    ResampledSamples,
    FirCoefficients,
    DecimatedSamples,
    ExactSumPartials,
}

impl FftInputAllocationStage {
    const fn label(self) -> &'static str {
        match self {
            Self::Name => "FFT input name",
            Self::SelectedSamples => "selected FFT input samples",
            Self::ResampledSamples => "resampled FFT input",
            Self::FirCoefficients => "anti-alias FIR coefficients",
            Self::DecimatedSamples => "decimated FFT input",
            Self::ExactSumPartials => "exact DC-mean residuals",
        }
    }
}

/// A transient record could not be prepared without altering or fabricating
/// numerical evidence.
#[derive(Debug, Clone, PartialEq, Error)]
#[non_exhaustive]
pub enum FftInputError {
    #[error("FFT input time/value lengths differ ({time_count} time points, {value_count} values)")]
    LengthMismatch {
        time_count: usize,
        value_count: usize,
    },
    #[error("FFT input requires at least {minimum} samples; received {length}")]
    InsufficientSamples { length: usize, minimum: usize },
    #[error("FFT input has {length} samples; the qualified preparation limit is {maximum}")]
    SampleLimit { length: usize, maximum: usize },
    #[error("FFT input time {index} is non-finite ({value:?})")]
    NonFiniteTime { index: usize, value: f64 },
    #[error("FFT input sample {index} is non-finite ({value:?})")]
    NonFiniteValue { index: usize, value: f64 },
    #[error(
        "FFT input time must increase strictly at index {index} ({previous:?} then {current:?})"
    )]
    NonIncreasingTime {
        index: usize,
        previous: f64,
        current: f64,
    },
    #[error("interactive FFT point cap must be between {minimum} and {maximum}; received {value}")]
    InvalidPointCap {
        value: usize,
        minimum: usize,
        maximum: usize,
    },
    #[error("FFT target sample count must be between {minimum} and {maximum}; received {value}")]
    InvalidTargetCount {
        value: usize,
        minimum: usize,
        maximum: usize,
    },
    #[error("FFT target sample count {target} exceeds the interactive point cap {point_cap}")]
    TargetExceedsPointCap { target: usize, point_cap: usize },
    #[error("FFT time window must have finite ordered bounds; received [{start:?}, {end:?}]")]
    InvalidTimeWindow { start: f64, end: f64 },
    #[error("FFT time window [{start:?}, {end:?}] retains only {retained} authored samples")]
    InsufficientWindowSamples {
        start: f64,
        end: f64,
        retained: usize,
    },
    #[error(
        "explicit FFT resampling from {source_count} to {target_count} samples would reduce rate without a qualified anti-alias filter"
    )]
    UnqualifiedRateReduction {
        source_count: usize,
        target_count: usize,
    },
    #[error(
        "FFT decimation factor {factor} exceeds the qualified single-stage anti-alias limit of {maximum}"
    )]
    DecimationQualification { factor: usize, maximum: usize },
    #[error("FFT input could not reserve {requested} units for {stage}")]
    Allocation {
        stage: &'static str,
        requested: usize,
    },
    #[error("FFT timebase cannot be represented at interval {index}: {reason}")]
    Timebase { index: usize, reason: &'static str },
    #[error(
        "FFT timestamp resolution {resolution:?} s at interval {index} cannot qualify nominal interval {nominal_interval:?} s"
    )]
    TimestampResolution {
        index: usize,
        resolution: f64,
        nominal_interval: f64,
    },
    #[error("FFT sample rate is not representable as a finite positive value ({sample_rate:?})")]
    InvalidSampleRate { sample_rate: f64 },
    #[error(
        "FFT interpolation at output {output_index}, source segment {segment} failed: {reason}"
    )]
    Interpolation {
        output_index: usize,
        segment: usize,
        reason: &'static str,
    },
    #[error("FFT numerical {stage} failed at index {index}: {reason}")]
    Numerical {
        stage: &'static str,
        index: usize,
        reason: &'static str,
    },
    #[error(
        "FFT prepared-output invariant failed (length {length}, sample rate {sample_rate:?}, decimation {decimation_factor})"
    )]
    OutputInvariant {
        length: usize,
        sample_rate: f64,
        decimation_factor: usize,
    },
}

/// FFT input preparation policy.
///
/// - `Reference`: preserve available time-domain detail (no implicit
///   decimation), bounded by `MAX_REFERENCE_RESAMPLE_POINTS` for safety.
/// - `Interactive`: enforce a hard point cap for responsiveness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FftInputPolicy {
    Reference,
    Interactive { max_points: usize },
}

impl FftInputPolicy {
    pub const fn reference() -> Self {
        Self::Reference
    }

    pub const fn interactive_default() -> Self {
        Self::Interactive {
            max_points: DEFAULT_MAX_FFT_POINTS,
        }
    }

    fn point_cap(self) -> Result<Option<usize>, FftInputError> {
        match self {
            Self::Reference => Ok(None),
            Self::Interactive { max_points } => {
                if !(MIN_FFT_SAMPLES..=MAX_REFERENCE_RESAMPLE_POINTS).contains(&max_points) {
                    return Err(FftInputError::InvalidPointCap {
                        value: max_points,
                        minimum: MIN_FFT_SAMPLES,
                        maximum: MAX_REFERENCE_RESAMPLE_POINTS,
                    });
                }
                Ok(Some(max_points))
            }
        }
    }
}

/// Optional time-domain bounds for FFT input selection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FftTimeWindow {
    /// Inclusive lower bound (seconds).
    pub start: f64,
    /// Inclusive upper bound (seconds).
    pub end: f64,
}

impl FftTimeWindow {
    pub const fn new(start: f64, end: f64) -> Self {
        Self { start, end }
    }

    fn validated(self) -> Result<Self, FftInputError> {
        if self.start.is_finite() && self.end.is_finite() && self.end > self.start {
            Ok(self)
        } else {
            Err(FftInputError::InvalidTimeWindow {
                start: self.start,
                end: self.end,
            })
        }
    }
}

/// End-to-end FFT input preparation options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FftInputOptions {
    /// Data fidelity/performance policy.
    pub policy: FftInputPolicy,
    /// Optional selected time-domain window.
    pub time_window: Option<FftTimeWindow>,
    /// Optional target sample count for explicit resampling.
    pub target_samples: Option<usize>,
}

impl Default for FftInputOptions {
    fn default() -> Self {
        Self {
            policy: FftInputPolicy::interactive_default(),
            time_window: None,
            target_samples: None,
        }
    }
}

impl FftInputOptions {
    pub fn with_policy(policy: FftInputPolicy) -> Self {
        Self {
            policy,
            ..Self::default()
        }
    }

    pub fn with_time_window(mut self, time_window: Option<FftTimeWindow>) -> Self {
        self.time_window = time_window;
        self
    }

    pub fn with_target_samples(mut self, target_samples: Option<usize>) -> Self {
        self.target_samples = target_samples;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PreparedFftInput {
    /// Source label (typically waveform name).
    pub name: String,
    /// Uniformly sampled signal values.
    pub samples: Vec<f64>,
    /// Effective sample rate (Hz) for `samples`.
    pub sample_rate: f64,
    /// Original valid sample count before processing.
    pub original_count: usize,
    /// Final decimation factor (1 means no decimation).
    pub decimation_factor: usize,
}

#[derive(Debug, Clone)]
struct UniformSeries {
    samples: Vec<f64>,
    sample_rate: f64,
}
/// Prepare a waveform for FFT analysis using explicit options.
///
/// The authored record is validated as a whole. Invalid samples, invalid
/// configuration, unqualified rate reduction, allocation failure, and
/// unrepresentable derived values are errors rather than silently discarded
/// evidence.
pub fn prepare_fft_input_with_options(
    name: &str,
    time: &[f64],
    values: &[f64],
    options: FftInputOptions,
) -> Result<PreparedFftInput, FftInputError> {
    let point_cap = options.policy.point_cap()?;
    let target_samples = validate_target_count(options.target_samples)?;
    if let (Some(point_cap), Some(target)) = (point_cap, target_samples)
        && target > point_cap
    {
        return Err(FftInputError::TargetExceedsPointCap { target, point_cap });
    }
    validate_source(time, values)?;
    let windowed = select_time_window(time, values, options.time_window)?;
    let original_count = windowed.len();

    let explicit_decimation = match target_samples {
        Some(target_count) if target_count < original_count => {
            if !original_count.is_multiple_of(target_count) {
                return Err(FftInputError::UnqualifiedRateReduction {
                    source_count: original_count,
                    target_count,
                });
            }
            let factor = original_count / target_count;
            if factor > MAX_QUALIFIED_DECIMATION_FACTOR {
                return Err(FftInputError::DecimationQualification {
                    factor,
                    maximum: MAX_QUALIFIED_DECIMATION_FACTOR,
                });
            }
            Some(factor)
        }
        _ => None,
    };

    let mut uniform = if let Some(target) = target_samples {
        if explicit_decimation.is_some() {
            // Preserve the authored point count before applying the qualified
            // anti-alias filter. Nonuniform evidence is first projected onto
            // an equal-count uniform grid, never directly onto the lower rate.
            if is_uniform_timeline(&windowed)? {
                uniform_from_cleaned(&windowed)?
            } else {
                resample_to_uniform(&windowed, windowed.len())?
            }
        } else if target == windowed.len() && is_uniform_timeline(&windowed)? {
            uniform_from_cleaned(&windowed)?
        } else {
            resample_to_uniform(&windowed, target)?
        }
    } else if is_uniform_timeline(&windowed)? {
        uniform_from_cleaned(&windowed)?
    } else {
        // Preserve the source rate first. Any later rate reduction happens
        // only through the qualified anti-alias path below.
        resample_to_uniform(&windowed, windowed.len())?
    };

    remove_dc_offset(&mut uniform.samples)?;

    let mut decimation_factor = 1usize;
    if let Some(factor) = explicit_decimation {
        uniform = anti_alias_decimate(&uniform, factor)?;
        decimation_factor = factor;
        remove_dc_offset(&mut uniform.samples)?;
    } else if let Some(max_points) = point_cap
        && uniform.samples.len() > max_points
    {
        decimation_factor = ceil_div(uniform.samples.len(), max_points);
        uniform = anti_alias_decimate(&uniform, decimation_factor)?;
        // Edge extension and finite FIR arithmetic can introduce a small
        // residual mean even when the source was centered. Authenticate
        // the actual retained record, not the pre-filter intermediate.
        remove_dc_offset(&mut uniform.samples)?;
    }

    if uniform.samples.len() < MIN_FFT_SAMPLES
        || uniform.samples.len() > MAX_REFERENCE_RESAMPLE_POINTS
        || !uniform.sample_rate.is_finite()
        || uniform.sample_rate <= 0.0
        || decimation_factor == 0
        || uniform.samples.iter().any(|value| !value.is_finite())
    {
        return Err(FftInputError::OutputInvariant {
            length: uniform.samples.len(),
            sample_rate: uniform.sample_rate,
            decimation_factor,
        });
    }

    Ok(PreparedFftInput {
        name: try_owned_name(name)?,
        samples: uniform.samples,
        sample_rate: uniform.sample_rate,
        original_count,
        decimation_factor,
    })
}

fn validate_target_count(target: Option<usize>) -> Result<Option<usize>, FftInputError> {
    let Some(value) = target else {
        return Ok(None);
    };
    if !(MIN_FFT_SAMPLES..=MAX_REFERENCE_RESAMPLE_POINTS).contains(&value) {
        return Err(FftInputError::InvalidTargetCount {
            value,
            minimum: MIN_FFT_SAMPLES,
            maximum: MAX_REFERENCE_RESAMPLE_POINTS,
        });
    }
    Ok(Some(value))
}

fn validate_source(time: &[f64], values: &[f64]) -> Result<(), FftInputError> {
    if time.len() != values.len() {
        return Err(FftInputError::LengthMismatch {
            time_count: time.len(),
            value_count: values.len(),
        });
    }
    if time.len() < MIN_FFT_SAMPLES {
        return Err(FftInputError::InsufficientSamples {
            length: time.len(),
            minimum: MIN_FFT_SAMPLES,
        });
    }
    for (index, (&t, &value)) in time.iter().zip(values).enumerate() {
        if !t.is_finite() {
            return Err(FftInputError::NonFiniteTime { index, value: t });
        }
        if !value.is_finite() {
            return Err(FftInputError::NonFiniteValue { index, value });
        }
        if index > 0 && t <= time[index - 1] {
            return Err(FftInputError::NonIncreasingTime {
                index,
                previous: time[index - 1],
                current: t,
            });
        }
    }
    Ok(())
}

fn validate_retained_count(retained: usize) -> Result<(), FftInputError> {
    if retained > MAX_REFERENCE_RESAMPLE_POINTS {
        return Err(FftInputError::SampleLimit {
            length: retained,
            maximum: MAX_REFERENCE_RESAMPLE_POINTS,
        });
    }
    Ok(())
}

fn select_time_window(
    time: &[f64],
    values: &[f64],
    time_window: Option<FftTimeWindow>,
) -> Result<Vec<(f64, f64)>, FftInputError> {
    let (start_index, end_index, diagnostic_bounds) = match time_window {
        Some(window) => {
            let window = window.validated()?;
            let start = time.partition_point(|value| *value < window.start);
            let end = time.partition_point(|value| *value <= window.end);
            (start, end, Some(window))
        }
        None => (0, time.len(), None),
    };
    let retained = end_index.saturating_sub(start_index);
    if retained < MIN_FFT_SAMPLES {
        let window = diagnostic_bounds.unwrap_or(FftTimeWindow {
            start: time[0],
            end: time[time.len() - 1],
        });
        return Err(FftInputError::InsufficientWindowSamples {
            start: window.start,
            end: window.end,
            retained,
        });
    }
    validate_retained_count(retained)?;

    let mut selected = Vec::new();
    try_reserve_exact(
        &mut selected,
        retained,
        FftInputAllocationStage::SelectedSamples,
    )?;
    selected.extend(
        time[start_index..end_index]
            .iter()
            .copied()
            .zip(values[start_index..end_index].iter().copied()),
    );
    Ok(selected)
}

fn is_uniform_timeline(data: &[(f64, f64)]) -> Result<bool, FftInputError> {
    let (nominal_interval, _) =
        qualified_grid_metrics(data[0].0, data[data.len() - 1].0, data.len())?;
    let mut uniform = true;
    for (index, pair) in data.windows(2).enumerate() {
        let ratio = qualified_interval_ratio(pair[0].0, pair[1].0, index, nominal_interval)?;
        if (ratio - 1.0).abs() > GRID_RELATIVE_TOLERANCE + 8.0 * f64::EPSILON {
            uniform = false;
        }
    }
    Ok(uniform)
}

fn uniform_from_cleaned(data: &[(f64, f64)]) -> Result<UniformSeries, FftInputError> {
    let (_, sample_rate) = qualified_grid_metrics(data[0].0, data[data.len() - 1].0, data.len())?;
    let mut samples = Vec::new();
    try_reserve_exact(
        &mut samples,
        data.len(),
        FftInputAllocationStage::ResampledSamples,
    )?;
    samples.extend(data.iter().map(|(_, value)| *value));
    Ok(UniformSeries {
        samples,
        sample_rate,
    })
}

fn resample_to_uniform(
    data: &[(f64, f64)],
    target_count: usize,
) -> Result<UniformSeries, FftInputError> {
    let t_start = data[0].0;
    let t_end = data[data.len() - 1].0;
    let (nominal_interval, sample_rate) = qualified_grid_metrics(t_start, t_end, target_count)?;
    let interval_count = target_count - 1;
    let interval_count_value =
        exactly_represented_usize(interval_count).ok_or(FftInputError::Timebase {
            index: interval_count,
            reason: "resampling interval count is not exactly representable",
        })?;

    let mut samples = Vec::new();
    try_reserve_exact(
        &mut samples,
        target_count,
        FftInputAllocationStage::ResampledSamples,
    )?;
    let mut source_upper = 0usize;
    let mut previous_time = None;
    for output_index in 0..target_count {
        let time = if output_index == 0 {
            t_start
        } else if output_index == interval_count {
            t_end
        } else {
            let index_value =
                exactly_represented_usize(output_index).ok_or(FftInputError::Timebase {
                    index: output_index,
                    reason: "resampling grid index is not exactly representable",
                })?;
            let fraction = index_value / interval_count_value;
            qualified_affine(t_start, t_end, fraction).map_err(|reason| {
                FftInputError::Interpolation {
                    output_index,
                    segment: source_upper.saturating_sub(1),
                    reason,
                }
            })?
        };
        if let Some(previous) = previous_time {
            if time <= previous {
                return Err(FftInputError::Timebase {
                    index: output_index,
                    reason: "uniform destination time is not representable after its predecessor",
                });
            }
            let ratio =
                qualified_interval_ratio(previous, time, output_index - 1, nominal_interval)?;
            if (ratio - 1.0).abs() > GRID_RELATIVE_TOLERANCE + 8.0 * f64::EPSILON {
                return Err(FftInputError::Timebase {
                    index: output_index - 1,
                    reason: "generated uniform interval exceeds the qualified grid tolerance",
                });
            }
        }
        previous_time = Some(time);
        while source_upper < data.len() && data[source_upper].0 < time {
            source_upper = source_upper.checked_add(1).ok_or(FftInputError::Timebase {
                index: output_index,
                reason: "source cursor exceeds this platform",
            })?;
        }
        if source_upper >= data.len() {
            return Err(FftInputError::Interpolation {
                output_index,
                segment: data.len() - 2,
                reason: "destination time could not be bracketed",
            });
        }
        let value = if data[source_upper].0 == time {
            data[source_upper].1
        } else {
            if source_upper == 0 {
                return Err(FftInputError::Interpolation {
                    output_index,
                    segment: 0,
                    reason: "destination time precedes the source grid",
                });
            }
            interpolate_segment(data, source_upper - 1, time, output_index)?
        };
        if !value.is_finite() {
            return Err(FftInputError::Numerical {
                stage: "linear interpolation",
                index: output_index,
                reason: "result is non-finite",
            });
        }
        samples.push(value);
    }

    Ok(UniformSeries {
        samples,
        sample_rate,
    })
}

fn qualified_interval_ratio(
    previous: f64,
    current: f64,
    index: usize,
    nominal_interval: f64,
) -> Result<f64, FftInputError> {
    let actual = scaled_positive_difference(current, previous).ok_or(FftInputError::Timebase {
        index,
        reason: "sample interval cannot be scaled",
    })?;
    let nominal = scaled_positive_value(nominal_interval).ok_or(FftInputError::Timebase {
        index,
        reason: "nominal interval cannot be scaled",
    })?;
    let resolution = value_ulp(previous).max(value_ulp(current));
    let resolution_scaled =
        scaled_positive_value(resolution).ok_or(FftInputError::TimestampResolution {
            index,
            resolution,
            nominal_interval,
        })?;
    let relative_resolution = scaled_positive_ratio(resolution_scaled, actual).ok_or(
        FftInputError::TimestampResolution {
            index,
            resolution,
            nominal_interval,
        },
    )?;
    if !resolution.is_finite() || relative_resolution > GRID_RELATIVE_TOLERANCE {
        return Err(FftInputError::TimestampResolution {
            index,
            resolution,
            nominal_interval,
        });
    }
    scaled_positive_ratio(actual, nominal).ok_or(FftInputError::Timebase {
        index,
        reason: "sample-interval ratio is not representable",
    })
}

fn anti_alias_decimate(
    input: &UniformSeries,
    factor: usize,
) -> Result<UniformSeries, FftInputError> {
    if factor <= 1 {
        let mut samples = Vec::new();
        try_reserve_exact(
            &mut samples,
            input.samples.len(),
            FftInputAllocationStage::DecimatedSamples,
        )?;
        samples.extend_from_slice(&input.samples);
        return Ok(UniformSeries {
            samples,
            sample_rate: input.sample_rate,
        });
    }
    if factor > MAX_QUALIFIED_DECIMATION_FACTOR {
        return Err(FftInputError::DecimationQualification {
            factor,
            maximum: MAX_QUALIFIED_DECIMATION_FACTOR,
        });
    }
    if input.samples.len() < MIN_FFT_SAMPLES
        || !input.sample_rate.is_finite()
        || input.sample_rate <= 0.0
    {
        return Err(FftInputError::OutputInvariant {
            length: input.samples.len(),
            sample_rate: input.sample_rate,
            decimation_factor: factor,
        });
    }

    let taps = fir_tap_count(factor)?;
    let cutoff = 0.45 / factor as f64;
    let fir = design_lowpass_fir(taps, cutoff)?;
    let decimated = apply_decimating_fir(&input.samples, &fir, factor)?;
    if decimated.len() < MIN_FFT_SAMPLES {
        return Err(FftInputError::InsufficientSamples {
            length: decimated.len(),
            minimum: MIN_FFT_SAMPLES,
        });
    }

    let sample_rate = input.sample_rate / factor as f64;
    if !sample_rate.is_finite() || sample_rate <= 0.0 {
        return Err(FftInputError::InvalidSampleRate { sample_rate });
    }

    Ok(UniformSeries {
        samples: decimated,
        sample_rate,
    })
}

fn ceil_div(n: usize, d: usize) -> usize {
    n / d + usize::from(!n.is_multiple_of(d))
}

fn fir_tap_count(factor: usize) -> Result<usize, FftInputError> {
    let mut taps = FIR_TAPS_PER_DECIMATION_FACTOR
        .checked_mul(factor)
        .ok_or(FftInputError::DecimationQualification {
            factor,
            maximum: MAX_QUALIFIED_DECIMATION_FACTOR,
        })?
        .max(MIN_FIR_TAPS);
    if taps.is_multiple_of(2) {
        taps = taps
            .checked_add(1)
            .ok_or(FftInputError::DecimationQualification {
                factor,
                maximum: MAX_QUALIFIED_DECIMATION_FACTOR,
            })?;
    }
    Ok(taps)
}

fn design_lowpass_fir(taps: usize, cutoff: f64) -> Result<Vec<f64>, FftInputError> {
    if taps < 3 || taps.is_multiple_of(2) || !cutoff.is_finite() || !(0.0..0.5).contains(&cutoff) {
        return Err(FftInputError::Numerical {
            stage: "anti-alias filter design",
            index: taps,
            reason: "tap count or cutoff is outside the qualified domain",
        });
    }
    let m = (taps - 1) as f64;
    let center = m * 0.5;
    let mut coeffs = Vec::new();
    try_reserve_exact(&mut coeffs, taps, FftInputAllocationStage::FirCoefficients)?;

    for n in 0..taps {
        let k = n as f64 - center;
        let sinc = if k == 0.0 {
            2.0 * cutoff
        } else {
            (2.0 * PI * cutoff * k).sin() / (PI * k)
        };

        // Blackman window for strong stopband suppression.
        let w =
            0.42 - 0.5 * (2.0 * PI * n as f64 / m).cos() + 0.08 * (4.0 * PI * n as f64 / m).cos();
        let coefficient = sinc * w;
        if !coefficient.is_finite() {
            return Err(FftInputError::Numerical {
                stage: "anti-alias filter design",
                index: n,
                reason: "coefficient is non-finite",
            });
        }
        coeffs.push(coefficient);
    }

    let mut coefficient_sum =
        ExactFloatSum::with_capacity(taps, FftInputAllocationStage::ExactSumPartials)?;
    for &coefficient in &coeffs {
        coefficient_sum.add(coefficient)?;
    }
    let sum = coefficient_sum.finish()?;
    if !sum.is_finite() || sum == 0.0 {
        return Err(FftInputError::Numerical {
            stage: "anti-alias filter normalization",
            index: taps,
            reason: "coefficient sum is zero or non-finite",
        });
    }
    for (index, coefficient) in coeffs.iter_mut().enumerate() {
        let unnormalized = *coefficient;
        *coefficient /= sum;
        if !coefficient.is_finite() || (unnormalized != 0.0 && *coefficient == 0.0) {
            return Err(FftInputError::Numerical {
                stage: "anti-alias filter normalization",
                index,
                reason: "normalization erased or overflowed a coefficient",
            });
        }
    }
    Ok(coeffs)
}

fn apply_decimating_fir(
    samples: &[f64],
    coeffs: &[f64],
    factor: usize,
) -> Result<Vec<f64>, FftInputError> {
    if samples.is_empty() || coeffs.is_empty() || factor == 0 {
        return Err(FftInputError::Numerical {
            stage: "anti-alias filtering",
            index: 0,
            reason: "samples or coefficients are empty, or decimation is zero",
        });
    }

    let half = (coeffs.len() / 2) as isize;
    let last = samples.len().saturating_sub(1) as isize;
    let scale = samples
        .iter()
        .fold(0.0_f64, |max, value| max.max(value.abs()));
    if !scale.is_finite() {
        return Err(FftInputError::Numerical {
            stage: "anti-alias input scaling",
            index: 0,
            reason: "source scale is non-finite",
        });
    }
    let output_count = ceil_div(samples.len(), factor);
    let mut out = Vec::new();
    try_reserve_exact(
        &mut out,
        output_count,
        FftInputAllocationStage::DecimatedSamples,
    )?;
    if scale == 0.0 {
        out.resize(output_count, 0.0);
        return Ok(out);
    }

    for output_index in 0..output_count {
        let center_index = output_index
            .checked_mul(factor)
            .ok_or(FftInputError::Numerical {
                stage: "anti-alias filtering",
                index: output_index,
                reason: "decimated source index exceeds this platform",
            })?;
        let mut sum = 0.0;
        let mut compensation = 0.0;
        for (tap_idx, &c) in coeffs.iter().enumerate() {
            let src = center_index as isize + tap_idx as isize - half;
            let src_idx = src.clamp(0, last) as usize;
            let normalized = samples[src_idx] / scale;
            if !normalized.is_finite() || (samples[src_idx] != 0.0 && normalized == 0.0) {
                return Err(FftInputError::Numerical {
                    stage: "anti-alias input scaling",
                    index: src_idx,
                    reason: "normalization erased or overflowed a sample",
                });
            }
            let product = normalized * c;
            if !product.is_finite() || (normalized != 0.0 && c != 0.0 && product == 0.0) {
                return Err(FftInputError::Numerical {
                    stage: "anti-alias convolution",
                    index: output_index,
                    reason: "a filter product is unrepresentable",
                });
            }
            let next = sum + product;
            if !next.is_finite() {
                return Err(FftInputError::Numerical {
                    stage: "anti-alias convolution",
                    index: output_index,
                    reason: "filter accumulation is non-finite",
                });
            }
            let residual = if sum.abs() >= product.abs() {
                (sum - next) + product
            } else {
                (product - next) + sum
            };
            compensation += residual;
            if !compensation.is_finite() {
                return Err(FftInputError::Numerical {
                    stage: "anti-alias convolution",
                    index: output_index,
                    reason: "filter compensation is non-finite",
                });
            }
            sum = next;
        }
        let normalized_output = sum + compensation;
        if !normalized_output.is_finite()
            || (normalized_output == 0.0 && (sum != 0.0 || compensation != 0.0))
        {
            return Err(FftInputError::Numerical {
                stage: "anti-alias convolution",
                index: output_index,
                reason: "compensated output is unrepresentable",
            });
        }
        let output = normalized_output * scale;
        if !output.is_finite() || (normalized_output != 0.0 && output == 0.0) {
            return Err(FftInputError::Numerical {
                stage: "anti-alias output scaling",
                index: output_index,
                reason: "restoring the source scale erased or overflowed the output",
            });
        }
        out.push(if output == 0.0 { 0.0 } else { output });
    }
    Ok(out)
}

fn remove_dc_offset(samples: &mut [f64]) -> Result<(), FftInputError> {
    if samples.is_empty() {
        return Err(FftInputError::InsufficientSamples {
            length: 0,
            minimum: MIN_FFT_SAMPLES,
        });
    }
    if samples.iter().all(|value| *value == samples[0]) {
        samples.fill(0.0);
        return Ok(());
    }
    let scale = samples
        .iter()
        .fold(0.0_f64, |max, value| max.max(value.abs()));
    if !scale.is_finite() || scale == 0.0 {
        return Err(FftInputError::Numerical {
            stage: "DC centering",
            index: 0,
            reason: "nonconstant record has an invalid scale",
        });
    }
    let mut sum =
        ExactFloatSum::with_capacity(samples.len(), FftInputAllocationStage::ExactSumPartials)?;
    for (index, &sample) in samples.iter().enumerate() {
        let normalized = sample / scale;
        if !normalized.is_finite() || (sample != 0.0 && normalized == 0.0) {
            return Err(FftInputError::Numerical {
                stage: "DC input scaling",
                index,
                reason: "normalization erased or overflowed a sample",
            });
        }
        sum.add(normalized)?;
    }
    let total = sum.finish()?;
    let count = exactly_represented_usize(samples.len()).ok_or(FftInputError::Numerical {
        stage: "DC centering",
        index: samples.len(),
        reason: "sample count is not exactly representable",
    })?;
    let mean = total / count;
    if !mean.is_finite() || !(-1.0..=1.0).contains(&mean) || (total != 0.0 && mean == 0.0) {
        return Err(FftInputError::Numerical {
            stage: "DC centering",
            index: samples.len(),
            reason: "normalized mean is unrepresentable",
        });
    }
    let represented_mean = mean * scale;
    if !represented_mean.is_finite() || (mean != 0.0 && represented_mean == 0.0) {
        return Err(FftInputError::Numerical {
            stage: "DC centering",
            index: samples.len(),
            reason: "restoring the mean scale erased or overflowed the mean",
        });
    }
    for (index, sample) in samples.iter_mut().enumerate() {
        let normalized = *sample / scale;
        let (high, low) =
            error_free_sum(normalized, -mean).map_err(|reason| FftInputError::Numerical {
                stage: "DC subtraction",
                index,
                reason,
            })?;
        let centered = high + low;
        if !centered.is_finite()
            || (centered == 0.0 && (high != 0.0 || low != 0.0 || *sample != represented_mean))
        {
            return Err(FftInputError::Numerical {
                stage: "DC subtraction",
                index,
                reason: "centering erased a nonzero difference",
            });
        }
        let restored = centered * scale;
        if !restored.is_finite() || (centered != 0.0 && restored == 0.0) {
            return Err(FftInputError::Numerical {
                stage: "DC output scaling",
                index,
                reason: "restoring the source scale erased or overflowed a sample",
            });
        }
        *sample = if restored == 0.0 { 0.0 } else { restored };
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct ScaledPositive {
    mantissa: f64,
    exponent: i32,
}

fn qualified_grid_metrics(
    start: f64,
    end: f64,
    sample_count: usize,
) -> Result<(f64, f64), FftInputError> {
    let interval_count = sample_count.checked_sub(1).ok_or(FftInputError::Timebase {
        index: 0,
        reason: "sample count has no interval",
    })?;
    let count = exactly_represented_usize(interval_count).ok_or(FftInputError::Timebase {
        index: interval_count,
        reason: "interval count is not exactly representable",
    })?;
    let span = scaled_positive_difference(end, start).ok_or(FftInputError::Timebase {
        index: interval_count.saturating_sub(1),
        reason: "record span is not representable in scaled form",
    })?;
    let count_scaled = scaled_positive_value(count).ok_or(FftInputError::Timebase {
        index: interval_count,
        reason: "interval count cannot be scaled",
    })?;
    let interval = scaled_positive_ratio(span, count_scaled).ok_or(FftInputError::Timebase {
        index: interval_count,
        reason: "nominal sample interval is outside finite representable range",
    })?;
    let sample_rate = scaled_positive_ratio(count_scaled, span)
        .ok_or(FftInputError::InvalidSampleRate { sample_rate: 0.0 })?;
    if !interval.is_finite() || interval <= 0.0 {
        return Err(FftInputError::Timebase {
            index: interval_count,
            reason: "nominal sample interval is not finite and positive",
        });
    }
    if !sample_rate.is_finite() || sample_rate <= 0.0 {
        return Err(FftInputError::InvalidSampleRate { sample_rate });
    }
    Ok((interval, sample_rate))
}

fn interpolate_segment(
    data: &[(f64, f64)],
    segment: usize,
    time: f64,
    output_index: usize,
) -> Result<f64, FftInputError> {
    let (t0, value0) = data[segment];
    let (t1, value1) = data[segment + 1];
    let numerator = scaled_positive_difference(time, t0).ok_or(FftInputError::Interpolation {
        output_index,
        segment,
        reason: "query offset is not representable in scaled form",
    })?;
    let denominator = scaled_positive_difference(t1, t0).ok_or(FftInputError::Interpolation {
        output_index,
        segment,
        reason: "source interval is not representable in scaled form",
    })?;
    let fraction =
        scaled_positive_ratio(numerator, denominator).ok_or(FftInputError::Interpolation {
            output_index,
            segment,
            reason: "interpolation fraction is not representable",
        })?;
    if !fraction.is_finite() || fraction <= 0.0 || fraction >= 1.0 {
        return Err(FftInputError::Interpolation {
            output_index,
            segment,
            reason: "interior interpolation fraction is not strictly inside (0, 1)",
        });
    }
    qualified_affine(value0, value1, fraction).map_err(|reason| FftInputError::Interpolation {
        output_index,
        segment,
        reason,
    })
}

fn qualified_affine(start: f64, end: f64, fraction: f64) -> Result<f64, &'static str> {
    if !start.is_finite() || !end.is_finite() {
        return Err("affine endpoints are non-finite");
    }
    if !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) {
        return Err("affine fraction is outside [0, 1]");
    }
    if fraction == 0.0 || start == end {
        return Ok(start);
    }
    if fraction == 1.0 {
        return Ok(end);
    }
    let (complement, complement_residual) = error_free_sum(1.0, -fraction)?;
    if complement + complement_residual <= 0.0 {
        return Err("affine complement is unrepresentable");
    }
    let mut expansion = BoundedExactFloatSum::<6>::new();
    for (weight, endpoint) in [
        (complement, start),
        (complement_residual, start),
        (fraction, end),
    ] {
        let (product, residual) = error_free_product(weight, endpoint)?;
        expansion.add(product)?;
        expansion.add(residual)?;
    }
    let result = expansion.finish()?;
    if !result.is_finite() || result <= start.min(end) || result >= start.max(end) {
        return Err("interior affine result is not representable strictly between its endpoints");
    }
    Ok(result)
}

struct BoundedExactFloatSum<const CAPACITY: usize> {
    partials: [f64; CAPACITY],
    len: usize,
}

impl<const CAPACITY: usize> BoundedExactFloatSum<CAPACITY> {
    const fn new() -> Self {
        Self {
            partials: [0.0; CAPACITY],
            len: 0,
        }
    }

    fn add(&mut self, mut value: f64) -> Result<(), &'static str> {
        if !value.is_finite() {
            return Err("floating-point expansion component is non-finite");
        }
        if value == 0.0 {
            return Ok(());
        }
        let mut retained = 0usize;
        for index in 0..self.len {
            let mut partial = self.partials[index];
            if value.abs() < partial.abs() {
                std::mem::swap(&mut value, &mut partial);
            }
            let high = value + partial;
            if !high.is_finite() {
                return Err("floating-point expansion accumulation is non-finite");
            }
            let low = partial - (high - value);
            if low != 0.0 {
                self.partials[retained] = low;
                retained += 1;
            }
            value = high;
        }
        if value != 0.0 {
            if retained == CAPACITY {
                return Err("floating-point expansion exceeded its fixed capacity");
            }
            self.partials[retained] = value;
            retained += 1;
        }
        self.len = retained;
        Ok(())
    }

    fn finish(self) -> Result<f64, &'static str> {
        let mut result = 0.0;
        for partial in self.partials.into_iter().take(self.len) {
            let next = result + partial;
            if !next.is_finite() {
                return Err("floating-point expansion result is non-finite");
            }
            result = next;
        }
        Ok(if result == 0.0 { 0.0 } else { result })
    }
}

struct ExactFloatSum {
    partials: Vec<f64>,
}

impl ExactFloatSum {
    fn with_capacity(
        capacity: usize,
        stage: FftInputAllocationStage,
    ) -> Result<Self, FftInputError> {
        let mut partials = Vec::new();
        try_reserve_exact(&mut partials, capacity, stage)?;
        Ok(Self { partials })
    }

    fn add(&mut self, mut value: f64) -> Result<(), FftInputError> {
        if !value.is_finite() {
            return Err(FftInputError::Numerical {
                stage: "exact accumulation",
                index: self.partials.len(),
                reason: "component is non-finite",
            });
        }
        if value == 0.0 {
            return Ok(());
        }
        let existing = self.partials.len();
        let mut retained = 0usize;
        for index in 0..existing {
            let mut partial = self.partials[index];
            if value.abs() < partial.abs() {
                std::mem::swap(&mut value, &mut partial);
            }
            let high = value + partial;
            if !high.is_finite() {
                return Err(FftInputError::Numerical {
                    stage: "exact accumulation",
                    index,
                    reason: "partial sum is non-finite",
                });
            }
            let low = partial - (high - value);
            if low != 0.0 {
                self.partials[retained] = low;
                retained += 1;
            }
            value = high;
        }
        self.partials.truncate(retained);
        if value != 0.0 {
            self.partials.push(value);
        }
        Ok(())
    }

    fn finish(self) -> Result<f64, FftInputError> {
        let mut result = 0.0;
        for (index, partial) in self.partials.into_iter().enumerate() {
            let next = result + partial;
            if !next.is_finite() {
                return Err(FftInputError::Numerical {
                    stage: "exact accumulation",
                    index,
                    reason: "final sum is non-finite",
                });
            }
            result = next;
        }
        Ok(if result == 0.0 { 0.0 } else { result })
    }
}

fn error_free_sum(left: f64, right: f64) -> Result<(f64, f64), &'static str> {
    let sum = left + right;
    if !sum.is_finite() {
        return Err("floating-point sum is non-finite");
    }
    let right_virtual = sum - left;
    let residual = (left - (sum - right_virtual)) + (right - right_virtual);
    if !residual.is_finite() {
        return Err("floating-point sum residual is non-finite");
    }
    Ok((sum, residual))
}

fn error_free_product(left: f64, right: f64) -> Result<(f64, f64), &'static str> {
    let product = left * right;
    if !product.is_finite() {
        return Err("floating-point product is non-finite");
    }
    if left != 0.0 && right != 0.0 && product == 0.0 {
        return Err("floating-point product underflowed");
    }
    let residual = left.mul_add(right, -product);
    if !residual.is_finite() {
        return Err("floating-point product residual is non-finite");
    }
    if !product_is_exact(left, right, product) && (residual == 0.0 || residual.is_subnormal()) {
        return Err("floating-point product has an uncertified underflow-scale residual");
    }
    Ok((product, residual))
}

fn product_is_exact(left: f64, right: f64, product: f64) -> bool {
    if left == 0.0 || right == 0.0 {
        return product == 0.0;
    }
    let (left_significand, left_exponent) = finite_binary_components(left);
    let (right_significand, right_exponent) = finite_binary_components(right);
    let exact_significand = u128::from(left_significand) * u128::from(right_significand);
    let exact_exponent = left_exponent + right_exponent;
    let (rounded_significand, rounded_exponent) = finite_binary_components(product);
    canonical_dyadic(exact_significand, exact_exponent)
        == canonical_dyadic(u128::from(rounded_significand), rounded_exponent)
}

fn canonical_dyadic(significand: u128, exponent: i32) -> (u128, i32) {
    debug_assert!(significand != 0);
    let trailing_zeros = significand.trailing_zeros();
    (
        significand >> trailing_zeros,
        exponent + trailing_zeros as i32,
    )
}

fn finite_binary_components(value: f64) -> (u64, i32) {
    const FRACTION_BITS: u64 = (1_u64 << 52) - 1;
    let bits = value.to_bits() & !(1_u64 << 63);
    let exponent_bits = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & FRACTION_BITS;
    if exponent_bits == 0 {
        (fraction, -1074)
    } else {
        ((1_u64 << 52) | fraction, exponent_bits - 1023 - 52)
    }
}

fn scaled_positive_value(value: f64) -> Option<ScaledPositive> {
    let (mantissa, exponent) = positive_binary_parts(value)?;
    Some(ScaledPositive { mantissa, exponent })
}

fn scaled_positive_difference(high: f64, low: f64) -> Option<ScaledPositive> {
    if !high.is_finite() || !low.is_finite() || high <= low {
        return None;
    }
    let scale = high.abs().max(low.abs());
    if !scale.is_finite() || scale <= 0.0 {
        return None;
    }
    let normalized_difference = high / scale - low / scale;
    scaled_positive_product([normalized_difference, scale])
}

fn scaled_positive_product<const N: usize>(values: [f64; N]) -> Option<ScaledPositive> {
    let mut mantissa = 1.0;
    let mut exponent = 0_i32;
    for value in values {
        let (factor_mantissa, factor_exponent) = positive_binary_parts(value)?;
        mantissa *= factor_mantissa;
        exponent = exponent.checked_add(factor_exponent)?;
        normalize_binary_parts(&mut mantissa, &mut exponent)?;
    }
    Some(ScaledPositive { mantissa, exponent })
}

fn scaled_positive_ratio(numerator: ScaledPositive, denominator: ScaledPositive) -> Option<f64> {
    let mut mantissa = numerator.mantissa / denominator.mantissa;
    let mut exponent = numerator.exponent.checked_sub(denominator.exponent)?;
    normalize_binary_parts(&mut mantissa, &mut exponent)?;
    materialize_binary_parts(mantissa, exponent)
}

fn positive_binary_parts(value: f64) -> Option<(f64, i32)> {
    const FRACTION_MASK: u64 = (1_u64 << 52) - 1;
    const EXPONENT_ONE: u64 = 1023_u64 << 52;
    if !value.is_finite() || value <= 0.0 {
        return None;
    }
    let bits = value.to_bits();
    let raw_exponent = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & FRACTION_MASK;
    if raw_exponent != 0 {
        return Some((f64::from_bits(EXPONENT_ONE | fraction), raw_exponent - 1023));
    }
    let highest_bit = 63_i32 - fraction.leading_zeros() as i32;
    let shift = u32::try_from(52_i32 - highest_bit).ok()?;
    let normalized_significand = fraction.checked_shl(shift)?;
    Some((
        f64::from_bits(EXPONENT_ONE | (normalized_significand & FRACTION_MASK)),
        highest_bit - 1074,
    ))
}

fn normalize_binary_parts(mantissa: &mut f64, exponent: &mut i32) -> Option<()> {
    if !mantissa.is_finite() || *mantissa <= 0.0 {
        return None;
    }
    while *mantissa >= 2.0 {
        *mantissa *= 0.5;
        *exponent = exponent.checked_add(1)?;
    }
    while *mantissa < 1.0 {
        *mantissa *= 2.0;
        *exponent = exponent.checked_sub(1)?;
    }
    Some(())
}

fn materialize_binary_parts(mantissa: f64, exponent: i32) -> Option<f64> {
    const FRACTION_MASK: u64 = (1_u64 << 52) - 1;
    const MAX_MANTISSA: f64 = f64::from_bits((1023_u64 << 52) | FRACTION_MASK);
    if !(1.0..2.0).contains(&mantissa) || !(-1075..=1023).contains(&exponent) {
        return None;
    }
    if exponent == 1023 && mantissa > MAX_MANTISSA {
        return None;
    }
    let fraction = mantissa.to_bits() & FRACTION_MASK;
    if exponent >= -1022 {
        let raw_exponent = u64::try_from(exponent + 1023).ok()?;
        let value = f64::from_bits((raw_exponent << 52) | fraction);
        return (value.is_finite() && value > 0.0).then_some(value);
    }
    let significand = (1_u64 << 52) | fraction;
    let shift = u32::try_from(-1022 - exponent).ok()?;
    let truncated = significand >> shift;
    let remainder_mask = (1_u64 << shift) - 1;
    let remainder = significand & remainder_mask;
    let halfway = 1_u64 << (shift - 1);
    let round_up = remainder > halfway || (remainder == halfway && !truncated.is_multiple_of(2));
    let rounded = truncated.checked_add(u64::from(round_up))?;
    let value = f64::from_bits(rounded);
    (value > 0.0).then_some(value)
}

fn exactly_represented_usize(value: usize) -> Option<f64> {
    let represented = value as f64;
    (represented.is_finite() && represented as usize == value).then_some(represented)
}

fn value_ulp(value: f64) -> f64 {
    if !value.is_finite() {
        return f64::INFINITY;
    }
    let upward = (value.next_up() - value).abs();
    let downward = (value - value.next_down()).abs();
    match (upward.is_finite(), downward.is_finite()) {
        (true, true) => upward.max(downward),
        (true, false) => upward,
        (false, true) => downward,
        (false, false) => f64::INFINITY,
    }
}

fn try_owned_name(name: &str) -> Result<String, FftInputError> {
    let mut owned = String::new();
    owned
        .try_reserve_exact(name.len())
        .map_err(|_| FftInputError::Allocation {
            stage: FftInputAllocationStage::Name.label(),
            requested: name.len(),
        })?;
    owned.push_str(name);
    Ok(owned)
}

fn try_reserve_exact<T>(
    values: &mut Vec<T>,
    additional: usize,
    stage: FftInputAllocationStage,
) -> Result<(), FftInputError> {
    values
        .try_reserve_exact(additional)
        .map_err(|_| FftInputError::Allocation {
            stage: stage.label(),
            requested: additional,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn uniform_record(length: usize, start: f64, interval: f64) -> (Vec<f64>, Vec<f64>) {
        let time = (0..length)
            .map(|index| start + index as f64 * interval)
            .collect::<Vec<_>>();
        let values = (0..length)
            .map(|index| (2.0 * PI * index as f64 / length as f64).sin() + 4.0)
            .collect::<Vec<_>>();
        (time, values)
    }

    #[test]
    fn authored_grid_is_validated_without_truncation_or_sample_dropping() {
        let (time, values) = uniform_record(MIN_FFT_SAMPLES, 0.0, 1.0);
        assert!(matches!(
            prepare_fft_input_with_options(
                "short",
                &time[..MIN_FFT_SAMPLES - 1],
                &values[..MIN_FFT_SAMPLES - 1],
                FftInputOptions::default()
            ),
            Err(FftInputError::InsufficientSamples {
                length,
                minimum: MIN_FFT_SAMPLES
            }) if length == MIN_FFT_SAMPLES - 1
        ));
        assert!(matches!(
            prepare_fft_input_with_options(
                "mismatch",
                &time,
                &values[..values.len() - 1],
                FftInputOptions::default()
            ),
            Err(FftInputError::LengthMismatch { .. })
        ));

        for nonfinite in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut invalid_time = time.clone();
            invalid_time[6] = nonfinite;
            assert!(matches!(
                prepare_fft_input_with_options(
                    "nonfinite-time",
                    &invalid_time,
                    &values,
                    FftInputOptions::default()
                ),
                Err(FftInputError::NonFiniteTime { index: 6, .. })
            ));

            let mut invalid_values = values.clone();
            invalid_values[7] = nonfinite;
            assert!(matches!(
                prepare_fft_input_with_options(
                    "nonfinite-value",
                    &time,
                    &invalid_values,
                    FftInputOptions::default()
                ),
                Err(FftInputError::NonFiniteValue { index: 7, .. })
            ));
        }

        for current in [time[7], time[7] - 1.0] {
            let mut nonmonotonic = time.clone();
            nonmonotonic[8] = current;
            assert!(matches!(
                prepare_fft_input_with_options(
                    "nonmonotonic",
                    &nonmonotonic,
                    &values,
                    FftInputOptions::default()
                ),
                Err(FftInputError::NonIncreasingTime { index: 8, .. })
            ));
        }
    }

    #[test]
    fn invalid_configuration_is_rejected_instead_of_clamped_or_ignored() {
        let (time, values) = uniform_record(32, 0.0, 1.0);
        let invalid_cap = FftInputOptions::with_policy(FftInputPolicy::Interactive {
            max_points: MIN_FFT_SAMPLES - 1,
        });
        assert!(matches!(
            prepare_fft_input_with_options("cap", &time, &values, invalid_cap),
            Err(FftInputError::InvalidPointCap { .. })
        ));

        let invalid_target = FftInputOptions::default().with_target_samples(Some(0));
        assert!(matches!(
            prepare_fft_input_with_options("target", &time, &values, invalid_target),
            Err(FftInputError::InvalidTargetCount { .. })
        ));

        let conflicting_counts = FftInputOptions::with_policy(FftInputPolicy::Interactive {
            max_points: MIN_FFT_SAMPLES,
        })
        .with_target_samples(Some(MIN_FFT_SAMPLES + 1));
        assert!(matches!(
            prepare_fft_input_with_options("counts", &time, &values, conflicting_counts),
            Err(FftInputError::TargetExceedsPointCap { .. })
        ));

        let invalid_window =
            FftInputOptions::default().with_time_window(Some(FftTimeWindow::new(f64::NAN, 1.0)));
        assert!(matches!(
            prepare_fft_input_with_options("window", &time, &values, invalid_window),
            Err(FftInputError::InvalidTimeWindow { .. })
        ));

        let reversed_window =
            FftInputOptions::default().with_time_window(Some(FftTimeWindow::new(4.0, 3.0)));
        assert!(matches!(
            prepare_fft_input_with_options("window", &time, &values, reversed_window),
            Err(FftInputError::InvalidTimeWindow { .. })
        ));

        let short_window =
            FftInputOptions::default().with_time_window(Some(FftTimeWindow::new(0.0, 4.0)));
        assert!(matches!(
            prepare_fft_input_with_options("window", &time, &values, short_window),
            Err(FftInputError::InsufficientWindowSamples { retained: 5, .. })
        ));

        assert_eq!(
            validate_retained_count(MAX_REFERENCE_RESAMPLE_POINTS + 1),
            Err(FftInputError::SampleLimit {
                length: MAX_REFERENCE_RESAMPLE_POINTS + 1,
                maximum: MAX_REFERENCE_RESAMPLE_POINTS,
            })
        );
    }

    #[test]
    fn allocation_failures_preserve_every_preparation_stage() {
        for stage in [
            FftInputAllocationStage::Name,
            FftInputAllocationStage::SelectedSamples,
            FftInputAllocationStage::ResampledSamples,
            FftInputAllocationStage::FirCoefficients,
            FftInputAllocationStage::DecimatedSamples,
            FftInputAllocationStage::ExactSumPartials,
        ] {
            let mut bytes = Vec::<u8>::new();
            assert_eq!(
                try_reserve_exact(&mut bytes, usize::MAX, stage),
                Err(FftInputError::Allocation {
                    stage: stage.label(),
                    requested: usize::MAX,
                })
            );
        }

        assert!(matches!(
            ExactFloatSum::with_capacity(usize::MAX, FftInputAllocationStage::ExactSumPartials),
            Err(FftInputError::Allocation {
                stage: "exact DC-mean residuals",
                requested: usize::MAX
            })
        ));
    }

    #[test]
    fn uniform_record_preserves_rate_and_removes_dc() {
        let (time, values) = uniform_record(32, 0.0, 0.25);
        let prepared = prepare_fft_input_with_options(
            "ordinary",
            &time,
            &values,
            FftInputOptions::with_policy(FftInputPolicy::Reference),
        )
        .expect("ordinary uniform record should qualify");
        assert_eq!(prepared.name, "ordinary");
        assert_eq!(prepared.original_count, 32);
        assert_eq!(prepared.decimation_factor, 1);
        assert_eq!(prepared.sample_rate, 4.0);
        assert!(prepared.samples.iter().all(|value| value.is_finite()));
        assert!(prepared.samples.iter().sum::<f64>().abs() <= 32.0 * f64::EPSILON);
    }

    #[test]
    fn finite_extreme_constants_center_to_exact_zero() {
        let time = (0..MIN_FFT_SAMPLES)
            .map(|index| index as f64)
            .collect::<Vec<_>>();
        for constant in [f64::MAX, f64::from_bits(1)] {
            let values = vec![constant; MIN_FFT_SAMPLES];
            let prepared = prepare_fft_input_with_options(
                "constant",
                &time,
                &values,
                FftInputOptions::with_policy(FftInputPolicy::Reference),
            )
            .expect("a finite constant record has an exact zero centered signal");
            assert!(prepared.samples.iter().all(|value| *value == 0.0));
        }
    }

    #[test]
    fn mixed_scale_centering_fails_when_normalization_erases_evidence() {
        let time = (0..MIN_FFT_SAMPLES)
            .map(|index| index as f64)
            .collect::<Vec<_>>();
        let mut values = vec![f64::MAX; MIN_FFT_SAMPLES];
        values[3] = f64::from_bits(1);
        assert!(matches!(
            prepare_fft_input_with_options(
                "mixed",
                &time,
                &values,
                FftInputOptions::with_policy(FftInputPolicy::Reference)
            ),
            Err(FftInputError::Numerical {
                stage: "DC input scaling",
                index: 3,
                ..
            })
        ));
    }

    #[test]
    fn nonuniform_resampling_is_linear_and_retains_authored_endpoints() {
        let data = (0..MIN_FFT_SAMPLES)
            .map(|index| {
                let time = (index * index) as f64;
                (time, 3.0 * time - 7.0)
            })
            .collect::<Vec<_>>();
        let resampled = resample_to_uniform(&data, MIN_FFT_SAMPLES)
            .expect("linear data should resample exactly on a qualified grid");
        assert_eq!(resampled.samples[0], data[0].1);
        assert_eq!(
            resampled.samples[MIN_FFT_SAMPLES - 1],
            data[MIN_FFT_SAMPLES - 1].1
        );
        for (index, value) in resampled.samples.iter().enumerate() {
            let fraction = index as f64 / (MIN_FFT_SAMPLES - 1) as f64;
            let time = qualified_affine(data[0].0, data[MIN_FFT_SAMPLES - 1].0, fraction)
                .expect("test grid must be representable");
            let expected = 3.0 * time - 7.0;
            assert!((*value - expected).abs() <= 2.0 * value_ulp(expected));
        }
    }

    #[test]
    fn overflowing_raw_time_span_still_has_a_qualified_finite_rate() {
        let interval_count = (MIN_FFT_SAMPLES - 1) as f64;
        let time = (0..MIN_FFT_SAMPLES)
            .map(|index| {
                if index == 0 {
                    -f64::MAX
                } else if index == MIN_FFT_SAMPLES - 1 {
                    f64::MAX
                } else {
                    qualified_affine(-f64::MAX, f64::MAX, index as f64 / interval_count)
                        .expect("the extreme uniform time grid must remain representable")
                }
            })
            .collect::<Vec<_>>();
        let values = vec![f64::MAX; MIN_FFT_SAMPLES];
        let prepared = prepare_fft_input_with_options(
            "extreme-time",
            &time,
            &values,
            FftInputOptions::with_policy(FftInputPolicy::Reference),
        )
        .expect("raw span overflow is not a semantic failure when the rate is representable");
        assert!(prepared.sample_rate.is_finite() && prepared.sample_rate > 0.0);
        assert!(prepared.samples.iter().all(|value| *value == 0.0));
    }

    #[test]
    fn timestamp_ulp_must_be_small_relative_to_nominal_interval() {
        let start = 1.0e16;
        let time = (0..MIN_FFT_SAMPLES)
            .map(|index| start + 2.0 * index as f64)
            .collect::<Vec<_>>();
        let values = vec![1.0; MIN_FFT_SAMPLES];
        assert!(matches!(
            prepare_fft_input_with_options(
                "poor-resolution",
                &time,
                &values,
                FftInputOptions::with_policy(FftInputPolicy::Reference)
            ),
            Err(FftInputError::TimestampResolution { .. })
        ));
    }

    #[test]
    fn generated_resampling_grid_must_meet_the_timestamp_resolution_contract() {
        let start = 1.0e16;
        let interval = 4.0e9;
        let time = (0..MIN_FFT_SAMPLES)
            .map(|index| start + interval * index as f64)
            .collect::<Vec<_>>();
        let values = (0..MIN_FFT_SAMPLES)
            .map(|index| index as f64)
            .collect::<Vec<_>>();
        let options =
            FftInputOptions::with_policy(FftInputPolicy::Reference).with_target_samples(Some(1024));
        assert!(matches!(
            prepare_fft_input_with_options("quantized-grid", &time, &values, options),
            Err(FftInputError::TimestampResolution { .. })
        ));
    }

    #[test]
    fn integral_rate_reduction_is_filtered_and_unqualified_reductions_fail_closed() {
        let (time, values) = uniform_record(64, 0.0, 1.0);
        let qualified_reduction =
            FftInputOptions::with_policy(FftInputPolicy::Reference).with_target_samples(Some(32));
        let prepared =
            prepare_fft_input_with_options("reduce", &time, &values, qualified_reduction)
                .expect("an integral twofold reduction has a qualified anti-alias path");
        assert_eq!(prepared.samples.len(), 32);
        assert_eq!(prepared.decimation_factor, 2);

        let unqualified_reduction =
            FftInputOptions::with_policy(FftInputPolicy::Reference).with_target_samples(Some(31));
        assert!(matches!(
            prepare_fft_input_with_options("reduce", &time, &values, unqualified_reduction),
            Err(FftInputError::UnqualifiedRateReduction {
                source_count: 64,
                target_count: 31
            })
        ));

        let input = UniformSeries {
            samples: vec![0.0; MIN_FFT_SAMPLES * 33],
            sample_rate: 1.0,
        };
        assert!(matches!(
            anti_alias_decimate(&input, 33),
            Err(FftInputError::DecimationQualification {
                factor: 33,
                maximum: 32
            })
        ));
    }

    #[test]
    fn bounded_interactive_decimation_is_filtered_and_suppresses_aliases() {
        const SOURCE_COUNT: usize = 2048;
        const FACTOR: usize = 8;
        const OUTPUT_COUNT: usize = SOURCE_COUNT / FACTOR;
        const LOW_SOURCE_FREQUENCY: f64 = 0.4 / FACTOR as f64;
        const REJECTED_SOURCE_FREQUENCY: f64 = 0.55 / FACTOR as f64;
        const LOW_OUTPUT_FREQUENCY: f64 = 0.4;
        const ALIAS_OUTPUT_FREQUENCY: f64 = 0.45;

        let time = (0..SOURCE_COUNT)
            .map(|index| index as f64)
            .collect::<Vec<_>>();
        let values = (0..SOURCE_COUNT)
            .map(|index| {
                let index = index as f64;
                (2.0 * PI * LOW_SOURCE_FREQUENCY * index).sin()
                    + (2.0 * PI * REJECTED_SOURCE_FREQUENCY * index).sin()
            })
            .collect::<Vec<_>>();
        let options = FftInputOptions::with_policy(FftInputPolicy::Interactive {
            max_points: OUTPUT_COUNT,
        });
        let prepared = prepare_fft_input_with_options("decimated", &time, &values, options)
            .expect("eightfold interactive decimation is inside the qualified FIR bound");
        assert_eq!(prepared.decimation_factor, FACTOR);
        assert_eq!(prepared.samples.len(), OUTPUT_COUNT);
        assert_eq!(prepared.sample_rate, 1.0 / FACTOR as f64);
        assert!(prepared.samples.iter().all(|value| value.is_finite()));

        // Exclude the edge-extension region and use 200 retained points so
        // both 0.4 and 0.45 cycles/output-sample are coherent projections.
        let interior = &prepared.samples[28..228];
        let amplitude = |frequency: f64| {
            let (in_phase, quadrature) = interior.iter().enumerate().fold(
                (0.0, 0.0),
                |(in_phase, quadrature), (index, &sample)| {
                    let angle = 2.0 * PI * frequency * index as f64;
                    (
                        in_phase + sample * angle.cos(),
                        quadrature + sample * angle.sin(),
                    )
                },
            );
            2.0 * in_phase.hypot(quadrature) / interior.len() as f64
        };
        let retained_amplitude = amplitude(LOW_OUTPUT_FREQUENCY);
        let alias_amplitude = amplitude(ALIAS_OUTPUT_FREQUENCY);
        assert!(
            (0.95..=1.05).contains(&retained_amplitude),
            "passband amplitude {retained_amplitude:?} is outside the qualified tolerance"
        );
        assert!(
            alias_amplitude < 1.0e-3,
            "stopband alias amplitude {alias_amplitude:?} exceeds the qualified limit"
        );
    }
}
