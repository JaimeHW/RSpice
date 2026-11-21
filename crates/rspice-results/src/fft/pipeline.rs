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

use crate::fft::binary_parts::{
    materialize_binary_parts, normalize_binary_parts, positive_binary_parts,
};

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
pub enum FftInputError {
    #[error("FFT input could not be read: {0}")]
    Unreadable(String),
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

/// The record a spectrum transforms: from `start`, inclusive, to `end`,
/// exclusive, taken as one period of the signal, as ViVA's `dft` takes it.
/// A record set to a whole number of periods is then coherent.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FftTimeWindow {
    /// The record's first instant (seconds), inclusive.
    pub start: f64,
    /// The record's end (seconds), exclusive: one period after `start`.
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

/// How the transform's samples are made where the record holds none on its
/// grid: Eldo's resampling choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Resampling {
    /// A straight line between the neighbouring samples.
    #[default]
    Linear,
    /// The last sample at or before the grid point, held.
    SampleAndHold,
}

impl Resampling {
    /// Both choices, in the order a menu lists them.
    pub const ALL: [Self; 2] = [Self::Linear, Self::SampleAndHold];

    /// The choice as a menu names it.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Linear => "Interpolate",
            Self::SampleAndHold => "Sample and hold",
        }
    }
}

/// End-to-end FFT input preparation options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FftInputOptions {
    /// Data fidelity/performance policy.
    pub policy: FftInputPolicy,
    /// The record to transform; `None` is the whole record, its last sample
    /// taken as the end of its one period.
    pub time_window: Option<FftTimeWindow>,
    /// The transform's sample count `N`; `None` takes as many as the record
    /// holds inside its window.
    pub target_samples: Option<usize>,
    /// How samples off the record's own grid are made.
    pub resampling: Resampling,
}

impl Default for FftInputOptions {
    fn default() -> Self {
        Self {
            policy: FftInputPolicy::interactive_default(),
            time_window: None,
            target_samples: None,
            resampling: Resampling::Linear,
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

    pub fn with_resampling(mut self, resampling: Resampling) -> Self {
        self.resampling = resampling;
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
    /// The record's own samples inside the record window.
    pub original_count: usize,
    /// Final decimation factor (1 means no decimation).
    pub decimation_factor: usize,
    /// The record's mean over its window, taken out before the transform.
    pub dc_level: f64,
    /// The record transformed, `[start, end)`.
    pub record: FftTimeWindow,
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
    let (point_cap, target_samples) = validate_options(options)?;
    validate_source(time, values)?;
    let windowed = select_time_window(time, values, options.time_window)?;
    prepare_windowed(
        name,
        windowed,
        point_cap,
        target_samples,
        options.resampling,
    )
}

/// Prepare a waveform for FFT analysis as [`prepare_fft_input_with_options`] prepares the same
/// samples, reading its columns rather than slices: the complete record is validated a block at a
/// time as it is read, and only the rows of the time window are held. A record kept in a results
/// store is prepared whatever its length; a block that cannot be read is
/// [`FftInputError::Unreadable`].
pub fn prepare_fft_input_from_columns(
    name: &str,
    time: &crate::waveform::WaveformValues,
    values: &crate::waveform::WaveformValues,
    options: FftInputOptions,
) -> Result<PreparedFftInput, FftInputError> {
    let (point_cap, target_samples) = validate_options(options)?;
    validate_source_columns(time, values)?;
    let windowed = select_time_window_columns(time, values, options.time_window)?;
    prepare_windowed(
        name,
        windowed,
        point_cap,
        target_samples,
        options.resampling,
    )
}

/// The point cap and target count `options` ask for, once they agree.
fn validate_options(
    options: FftInputOptions,
) -> Result<(Option<usize>, Option<usize>), FftInputError> {
    let point_cap = options.policy.point_cap()?;
    let target_samples = validate_target_count(options.target_samples)?;
    if let (Some(point_cap), Some(target)) = (point_cap, target_samples)
        && target > point_cap
    {
        return Err(FftInputError::TargetExceedsPointCap { target, point_cap });
    }
    Ok((point_cap, target_samples))
}

/// The pipeline from the validated rows of the time window on.
fn prepare_windowed(
    name: &str,
    windowed: WindowedRows,
    point_cap: Option<usize>,
    target_samples: Option<usize>,
    resampling: Resampling,
) -> Result<PreparedFftInput, FftInputError> {
    let WindowedRows {
        rows,
        start,
        stop,
        in_window,
    } = windowed;
    let original_count = in_window;

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

    // The record's own density first: a rate reduction happens only through
    // the qualified anti-alias path below, never by resampling straight onto
    // the lower rate.
    let grid_count = match (explicit_decimation, target_samples) {
        (Some(_), _) | (None, None) => original_count,
        (None, Some(target)) => target,
    };
    let mut uniform = record_on_grid(&rows, start, stop, grid_count, resampling)?;

    let mut dc_level = remove_dc_offset(&mut uniform.samples)?;

    let mut decimation_factor = 1usize;
    if let Some(factor) = explicit_decimation {
        uniform = anti_alias_decimate(&uniform, factor)?;
        decimation_factor = factor;
        dc_level += remove_dc_offset(&mut uniform.samples)?;
    } else if let Some(max_points) = point_cap
        && uniform.samples.len() > max_points
    {
        decimation_factor = ceil_div(uniform.samples.len(), max_points);
        uniform = anti_alias_decimate(&uniform, decimation_factor)?;
        // Edge extension and finite FIR arithmetic can introduce a small
        // residual mean even when the source was centered. Authenticate
        // the actual retained record, not the pre-filter intermediate.
        dc_level += remove_dc_offset(&mut uniform.samples)?;
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
        dc_level,
        record: FftTimeWindow::new(start, stop),
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
    validate_source_counts(time.len(), values.len())?;
    let mut previous = None;
    for (index, (&t, &value)) in time.iter().zip(values).enumerate() {
        validate_source_sample(index, t, value, previous)?;
        previous = Some(t);
    }
    Ok(())
}

/// [`validate_source`] over columns, a block of each at a time.
fn validate_source_columns(
    time: &crate::waveform::WaveformValues,
    values: &crate::waveform::WaveformValues,
) -> Result<(), FftInputError> {
    validate_source_counts(time.len(), values.len())?;
    let mut previous = None;
    let mut index = 0;
    column_pairs(time, values, 0..time.len(), |t, value| {
        validate_source_sample(index, t, value, previous)?;
        previous = Some(t);
        index += 1;
        Ok(())
    })
}

/// Each pair of samples of `time` and `values` over `rows`, in order, read a block of each column
/// at a time; the columns' blocks need not line up.
fn column_pairs(
    time: &crate::waveform::WaveformValues,
    values: &crate::waveform::WaveformValues,
    rows: std::ops::Range<usize>,
    mut each: impl FnMut(f64, f64) -> Result<(), FftInputError>,
) -> Result<(), FftInputError> {
    let unreadable = |failure: crate::waveform::ColumnReadFailure| {
        FftInputError::Unreadable(failure.to_string())
    };
    let (mut times, mut samples) = (time.blocks(rows.clone()), values.blocks(rows));
    let (mut time_block, mut time_at) = (std::borrow::Cow::Borrowed(&[][..]), 0);
    let (mut value_block, mut value_at) = (std::borrow::Cow::Borrowed(&[][..]), 0);
    loop {
        if time_at == time_block.len() {
            let Some(block) = times.next() else {
                return Ok(());
            };
            (time_block, time_at) = (block.map_err(unreadable)?, 0);
        }
        if value_at == value_block.len() {
            let Some(block) = samples.next() else {
                return Ok(());
            };
            (value_block, value_at) = (block.map_err(unreadable)?, 0);
        }
        let run = (time_block.len() - time_at).min(value_block.len() - value_at);
        for (&t, &value) in time_block[time_at..time_at + run]
            .iter()
            .zip(&value_block[value_at..value_at + run])
        {
            each(t, value)?;
        }
        time_at += run;
        value_at += run;
    }
}

fn validate_source_counts(time_count: usize, value_count: usize) -> Result<(), FftInputError> {
    if time_count != value_count {
        return Err(FftInputError::LengthMismatch {
            time_count,
            value_count,
        });
    }
    if time_count < MIN_FFT_SAMPLES {
        return Err(FftInputError::InsufficientSamples {
            length: time_count,
            minimum: MIN_FFT_SAMPLES,
        });
    }
    Ok(())
}

/// Sample `index` of the record, `t` and `value`, after the time `previous`.
fn validate_source_sample(
    index: usize,
    t: f64,
    value: f64,
    previous: Option<f64>,
) -> Result<(), FftInputError> {
    if !t.is_finite() {
        return Err(FftInputError::NonFiniteTime { index, value: t });
    }
    if !value.is_finite() {
        return Err(FftInputError::NonFiniteValue { index, value });
    }
    if let Some(previous) = previous
        && t <= previous
    {
        return Err(FftInputError::NonIncreasingTime {
            index,
            previous,
            current: t,
        });
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

/// The rows of a record window and the window itself: every row from the
/// last at or before its start to the first at or after its end, so each
/// point of the transform's grid in `[start, stop)` is bracketed.
struct WindowedRows {
    rows: Vec<(f64, f64)>,
    start: f64,
    stop: f64,
    /// The rows inside `[start, stop)`.
    in_window: usize,
}

/// The record `[start, stop)` a window asks of a record from `first` to
/// `last`: the whole record when none is set, and otherwise the window kept
/// inside the record.
fn record_bounds(
    first: f64,
    last: f64,
    time_window: Option<FftTimeWindow>,
) -> Result<(f64, f64), FftInputError> {
    let Some(window) = time_window else {
        return Ok((first, last));
    };
    let window = window.validated()?;
    let (start, stop) = (window.start.max(first), window.end.min(last));
    if stop > start {
        Ok((start, stop))
    } else {
        Err(FftInputError::InsufficientWindowSamples {
            start: window.start,
            end: window.end,
            retained: 0,
        })
    }
}

/// How close to the record's end a written sample is taken to be at it: a
/// millionth of a step, the slack a time accumulated step by step can miss
/// the end it was meant to land on by.
fn end_tolerance(start: f64, stop: f64, samples: usize) -> f64 {
    let samples = samples.max(1) as f64;
    (stop / samples - start / samples) * 1.0e-6
}

/// Whether the instant `t` is the record's end `stop`, or past it.
fn at_or_past(t: f64, stop: f64, tolerance: f64) -> bool {
    t >= stop - tolerance
}

/// The index range of the rows a record `[start, stop)` reads, bracketing
/// rows included, and how many rows lie inside it, from the count of rows
/// before an instant.
fn window_rows(
    len: usize,
    start: f64,
    stop: f64,
    rows_before: impl Fn(&dyn Fn(f64) -> bool) -> usize,
) -> (std::ops::Range<usize>, usize) {
    let through_start = rows_before(&|t| t <= start);
    let before_start = rows_before(&|t| t < start);
    let tolerance = end_tolerance(start, stop, rows_before(&|t| t < stop) - before_start);
    let before_stop = rows_before(&|t| !at_or_past(t, stop, tolerance));
    let first = through_start.saturating_sub(1);
    let end = (before_stop + 1).min(len);
    (first..end, before_stop.saturating_sub(before_start))
}

fn select_time_window(
    time: &[f64],
    values: &[f64],
    time_window: Option<FftTimeWindow>,
) -> Result<WindowedRows, FftInputError> {
    let (start, stop) = record_bounds(time[0], time[time.len() - 1], time_window)?;
    let (range, in_window) = window_rows(time.len(), start, stop, |before| {
        time.partition_point(|t| before(*t))
    });
    validate_window_rows(in_window, Some(FftTimeWindow::new(start, stop)), || {
        FftTimeWindow::new(start, stop)
    })?;

    let mut selected = Vec::new();
    try_reserve_exact(
        &mut selected,
        range.len(),
        FftInputAllocationStage::SelectedSamples,
    )?;
    selected.extend(
        time[range.clone()]
            .iter()
            .copied()
            .zip(values[range].iter().copied()),
    );
    Ok(WindowedRows {
        rows: selected,
        start,
        stop,
        in_window,
    })
}

/// [`select_time_window`] over columns: the window's bounds found by search, and only its rows
/// read, a block of each column at a time.
fn select_time_window_columns(
    time: &crate::waveform::WaveformValues,
    values: &crate::waveform::WaveformValues,
    time_window: Option<FftTimeWindow>,
) -> Result<WindowedRows, FftInputError> {
    let first = time.sample(0).unwrap_or(f64::NAN);
    let last = time.sample(time.len() - 1).unwrap_or(f64::NAN);
    let (start, stop) = record_bounds(first, last, time_window)?;
    let (range, in_window) = window_rows(time.len(), start, stop, |before| {
        time.partition_point(|t| before(t))
    });
    validate_window_rows(in_window, Some(FftTimeWindow::new(start, stop)), || {
        FftTimeWindow::new(start, stop)
    })?;

    let mut selected = Vec::new();
    try_reserve_exact(
        &mut selected,
        range.len(),
        FftInputAllocationStage::SelectedSamples,
    )?;
    column_pairs(time, values, range, |t, value| {
        selected.push((t, value));
        Ok(())
    })?;
    Ok(WindowedRows {
        rows: selected,
        start,
        stop,
        in_window,
    })
}

/// Whether a window of `retained` rows can be prepared: `window`, or the whole record's span,
/// names it when it holds too few.
fn validate_window_rows(
    retained: usize,
    window: Option<FftTimeWindow>,
    record: impl FnOnce() -> FftTimeWindow,
) -> Result<(), FftInputError> {
    if retained < MIN_FFT_SAMPLES {
        let window = window.unwrap_or_else(record);
        return Err(FftInputError::InsufficientWindowSamples {
            start: window.start,
            end: window.end,
            retained,
        });
    }
    validate_retained_count(retained)
}

/// The record `[start, stop)` as `count` samples on its uniform grid,
/// `start + k·(stop − start)/count`, at the rate `count/(stop − start)`.
///
/// When the record's own samples inside the window are that grid, they are
/// taken as they stand. Otherwise each grid point is made from the record
/// as `resampling` says: a record whose steps are not uniform, as every
/// transient with a switching edge is, is resampled rather than refused.
fn record_on_grid(
    rows: &[(f64, f64)],
    start: f64,
    stop: f64,
    count: usize,
    resampling: Resampling,
) -> Result<UniformSeries, FftInputError> {
    let interval_count = count.checked_add(1).ok_or(FftInputError::Timebase {
        index: count,
        reason: "sample count exceeds this platform",
    })?;
    let (nominal_interval, sample_rate) = qualified_grid_metrics(start, stop, interval_count)?;
    let first_inside = rows.partition_point(|(t, _)| *t < start);
    let tolerance = end_tolerance(start, stop, count);
    let inside = rows[first_inside..]
        .iter()
        .take_while(|(t, _)| !at_or_past(*t, stop, tolerance))
        .count();
    if inside == count
        && let Some(samples) = sample_grid(start, stop, count, nominal_interval, |index, time| {
            let (t, value) = rows[first_inside + index];
            Ok(((t - time).abs() <= GRID_RELATIVE_TOLERANCE * nominal_interval).then_some(value))
        })?
    {
        return Ok(UniformSeries {
            samples,
            sample_rate,
        });
    }
    let mut upper = 0usize;
    let samples = sample_grid(start, stop, count, nominal_interval, |index, time| {
        while upper < rows.len() && rows[upper].0 < time {
            upper += 1;
        }
        value_at(rows, upper, time, index, resampling).map(Some)
    })?
    .ok_or(FftInputError::Timebase {
        index: 0,
        reason: "resampling left a grid point without a value",
    })?;
    Ok(UniformSeries {
        samples,
        sample_rate,
    })
}

/// The `count` points of the grid `start + k·(stop − start)/count`, each
/// checked to stand one nominal interval after the last, and the sample
/// `sample` gives at each: `None` once it gives none.
fn sample_grid(
    start: f64,
    stop: f64,
    count: usize,
    nominal_interval: f64,
    mut sample: impl FnMut(usize, f64) -> Result<Option<f64>, FftInputError>,
) -> Result<Option<Vec<f64>>, FftInputError> {
    let count_value = exactly_represented_usize(count).ok_or(FftInputError::Timebase {
        index: count,
        reason: "sample count is not exactly representable",
    })?;
    let mut samples = Vec::new();
    try_reserve_exact(
        &mut samples,
        count,
        FftInputAllocationStage::ResampledSamples,
    )?;
    let mut previous_time = None;
    for output_index in 0..count {
        let time = if output_index == 0 {
            start
        } else {
            let index_value =
                exactly_represented_usize(output_index).ok_or(FftInputError::Timebase {
                    index: output_index,
                    reason: "resampling grid index is not exactly representable",
                })?;
            qualified_affine(start, stop, index_value / count_value).map_err(|reason| {
                FftInputError::Interpolation {
                    output_index,
                    segment: output_index.saturating_sub(1),
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
        let Some(value) = sample(output_index, time)? else {
            return Ok(None);
        };
        samples.push(value);
    }
    Ok(Some(samples))
}

/// The record's value at the grid instant `time`, `upper` the first row at
/// or after it.
fn value_at(
    rows: &[(f64, f64)],
    upper: usize,
    time: f64,
    output_index: usize,
    resampling: Resampling,
) -> Result<f64, FftInputError> {
    if upper >= rows.len() {
        return Err(FftInputError::Interpolation {
            output_index,
            segment: rows.len().saturating_sub(2),
            reason: "destination time could not be bracketed",
        });
    }
    let value = if rows[upper].0 == time {
        rows[upper].1
    } else if upper == 0 {
        return Err(FftInputError::Interpolation {
            output_index,
            segment: 0,
            reason: "destination time precedes the source grid",
        });
    } else {
        match resampling {
            Resampling::Linear => interpolate_segment(rows, upper - 1, time, output_index)?,
            Resampling::SampleAndHold => rows[upper - 1].1,
        }
    };
    if !value.is_finite() {
        return Err(FftInputError::Numerical {
            stage: "resampling",
            index: output_index,
            reason: "result is non-finite",
        });
    }
    Ok(value)
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

/// Take the record's mean out of it, and return the mean.
fn remove_dc_offset(samples: &mut [f64]) -> Result<f64, FftInputError> {
    if samples.is_empty() {
        return Err(FftInputError::InsufficientSamples {
            length: 0,
            minimum: MIN_FFT_SAMPLES,
        });
    }
    if samples.iter().all(|value| *value == samples[0]) {
        let level = samples[0];
        samples.fill(0.0);
        return Ok(level);
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
    Ok(represented_mean)
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
    // Neighbouring values a few units in the last place apart can round onto
    // an endpoint; the straight line between them is then that endpoint.
    qualified_affine(value0, value1, fraction).or_else(|_| {
        let value = value0 * (1.0 - fraction) + value1 * fraction;
        if value.is_finite() {
            Ok(value.clamp(value0.min(value1), value0.max(value1)))
        } else {
            Err(FftInputError::Interpolation {
                output_index,
                segment,
                reason: "interpolated value is not finite",
            })
        }
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
        // One period over the record's `length − 1` steps, the last sample
        // ending it.
        let values = (0..length)
            .map(|index| (2.0 * PI * index as f64 / (length - 1) as f64).sin() + 4.0)
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

        // One period [0, 4) of a record sampled each second holds 4 samples.
        let short_window =
            FftInputOptions::default().with_time_window(Some(FftTimeWindow::new(0.0, 4.0)));
        assert!(matches!(
            prepare_fft_input_with_options("window", &time, &values, short_window),
            Err(FftInputError::InsufficientWindowSamples { retained: 4, .. })
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
        // The last sample ends the record's one period: 31 samples over
        // [0, 7.75) at 4 per second.
        assert_eq!(prepared.original_count, 31);
        assert_eq!(prepared.samples.len(), 31);
        assert_eq!(prepared.decimation_factor, 1);
        assert_eq!(prepared.sample_rate, 4.0);
        assert_eq!(prepared.record, FftTimeWindow::new(0.0, 7.75));
        assert!(prepared.samples.iter().all(|value| value.is_finite()));
        assert!(prepared.samples.iter().sum::<f64>().abs() <= 32.0 * f64::EPSILON);
    }

    #[test]
    fn finite_extreme_constants_center_to_exact_zero() {
        let time = (0..=MIN_FFT_SAMPLES)
            .map(|index| index as f64)
            .collect::<Vec<_>>();
        for constant in [f64::MAX, f64::from_bits(1)] {
            let values = vec![constant; MIN_FFT_SAMPLES + 1];
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
        let time = (0..=MIN_FFT_SAMPLES)
            .map(|index| index as f64)
            .collect::<Vec<_>>();
        let mut values = vec![f64::MAX; MIN_FFT_SAMPLES + 1];
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
    fn nonuniform_resampling_is_linear_over_one_period_from_the_start() {
        let data = (0..=MIN_FFT_SAMPLES)
            .map(|index| {
                let time = (index * index) as f64;
                (time, 3.0 * time - 7.0)
            })
            .collect::<Vec<_>>();
        let (start, stop) = (data[0].0, data[MIN_FFT_SAMPLES].0);
        let resampled = record_on_grid(&data, start, stop, MIN_FFT_SAMPLES, Resampling::Linear)
            .expect("linear data should resample exactly on a qualified grid");
        assert_eq!(resampled.samples.len(), MIN_FFT_SAMPLES);
        assert_eq!(resampled.samples[0], data[0].1);
        assert_eq!(resampled.sample_rate, MIN_FFT_SAMPLES as f64 / stop);
        for (index, value) in resampled.samples.iter().enumerate() {
            let fraction = index as f64 / MIN_FFT_SAMPLES as f64;
            let time =
                qualified_affine(start, stop, fraction).expect("test grid must be representable");
            let expected = 3.0 * time - 7.0;
            assert!((*value - expected).abs() <= 2.0 * value_ulp(expected));
        }

        // Sample and hold keeps the last sample at or before each point.
        let held = record_on_grid(
            &data,
            start,
            stop,
            MIN_FFT_SAMPLES,
            Resampling::SampleAndHold,
        )
        .expect("held data on a qualified grid");
        for (index, value) in held.samples.iter().enumerate() {
            let time = start + (stop - start) * index as f64 / MIN_FFT_SAMPLES as f64;
            let before = data.iter().rev().find(|(t, _)| *t <= time).expect("a row");
            assert_eq!(*value, before.1, "grid point {index} at {time}");
        }
    }

    #[test]
    fn overflowing_raw_time_span_still_has_a_qualified_finite_rate() {
        let interval_count = MIN_FFT_SAMPLES as f64;
        let time = (0..=MIN_FFT_SAMPLES)
            .map(|index| {
                if index == 0 {
                    -f64::MAX
                } else if index == MIN_FFT_SAMPLES {
                    f64::MAX
                } else {
                    qualified_affine(-f64::MAX, f64::MAX, index as f64 / interval_count)
                        .expect("the extreme uniform time grid must remain representable")
                }
            })
            .collect::<Vec<_>>();
        let values = vec![f64::MAX; MIN_FFT_SAMPLES + 1];
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
        let time = (0..=MIN_FFT_SAMPLES)
            .map(|index| start + 2.0 * index as f64)
            .collect::<Vec<_>>();
        let values = vec![1.0; MIN_FFT_SAMPLES + 1];
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
        let time = (0..=MIN_FFT_SAMPLES)
            .map(|index| start + interval * index as f64)
            .collect::<Vec<_>>();
        let values = (0..=MIN_FFT_SAMPLES)
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
        let (time, values) = uniform_record(65, 0.0, 1.0);
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

    /// A column kept in a store, read a block at a time.
    struct Stored(Vec<f64>);

    impl crate::waveform::ColumnSource for Stored {
        fn name(&self) -> &str {
            "column"
        }

        fn store(&self) -> String {
            "run.rsr".to_owned()
        }

        fn rows(&self) -> u64 {
            self.0.len() as u64
        }

        fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
            let first = first as usize;
            out.copy_from_slice(&self.0[first..first + out.len()]);
            Ok(())
        }
    }

    fn stored(values: &[f64]) -> crate::waveform::WaveformValues {
        crate::waveform::WaveformValues::stored(
            std::sync::Arc::new(Stored(values.to_vec())),
            crate::waveform::StoredWindow {
                first: 0,
                rows: values.len() as u64,
                lead: crate::waveform::WindowLead::None,
            },
        )
        .expect("the window fits the column")
    }

    /// Columns prepare what slices of the same samples prepare: a record of several blocks cut by
    /// a time window, in memory, in a store and one of each, and each refusal at the same row.
    #[test]
    fn columns_prepare_what_slices_of_the_same_samples_prepare() {
        use crate::waveform::{BLOCK_SAMPLES, WaveformValues};
        let rows = 3 * BLOCK_SAMPLES + 17;
        let (time, values) = uniform_record(rows, 0.0, 1.0e-9);
        let options = FftInputOptions::with_policy(FftInputPolicy::interactive_default())
            .with_time_window(Some(FftTimeWindow {
                start: 1.0e-6,
                end: (rows - 500) as f64 * 1.0e-9,
            }));
        let prepared = prepare_fft_input_with_options("v", &time, &values, options);
        assert!(prepared.is_ok(), "{prepared:?}");
        let columns = [
            (
                WaveformValues::from(time.clone()),
                WaveformValues::from(values.clone()),
            ),
            (stored(&time), stored(&values)),
            (WaveformValues::from(time.clone()), stored(&values)),
        ];
        for (time, values) in &columns {
            assert_eq!(
                prepare_fft_input_from_columns("v", time, values, options),
                prepared
            );
        }

        let mut repeated = time.clone();
        repeated[2 * BLOCK_SAMPLES + 5] = repeated[2 * BLOCK_SAMPLES + 4];
        let refused = prepare_fft_input_with_options("v", &repeated, &values, options);
        assert!(
            matches!(refused, Err(FftInputError::NonIncreasingTime { index, .. }) if index == 2 * BLOCK_SAMPLES + 5),
            "{refused:?}"
        );
        assert_eq!(
            prepare_fft_input_from_columns("v", &stored(&repeated), &stored(&values), options),
            refused
        );
        let mut gap = values.clone();
        gap[BLOCK_SAMPLES + 3] = f64::NAN;
        assert!(matches!(
            prepare_fft_input_from_columns("v", &stored(&time), &stored(&gap), options),
            Err(FftInputError::NonFiniteValue { index, .. }) if index == BLOCK_SAMPLES + 3
        ));
        let narrow = options.with_time_window(Some(FftTimeWindow {
            start: 1.0e-6,
            end: 1.0e-6 + 2.0e-9,
        }));
        assert_eq!(
            prepare_fft_input_from_columns("v", &stored(&time), &stored(&values), narrow),
            prepare_fft_input_with_options("v", &time, &values, narrow)
        );
    }

    /// The record is N samples over [start, stop), one period of the signal,
    /// as ViVA's `dft` takes it. A transient written to an exact whole number
    /// of periods, 127 cycles of 1 kHz at 4096 steps (4097 points, 0 to
    /// 127 ms inclusive), is then coherent: the 4096 samples are numpy's
    /// coherent record, and its rectangular spectrum holds the tone in bin 127
    /// with every other bin at rounding. Taking the end sample into the
    /// period, as the record once did, leaked the tone and read SNR 29 dB.
    #[test]
    fn a_whole_number_of_periods_is_a_coherent_record() {
        use crate::fft::data::{FftData, SpectrumAnalysis, SpectrumNormalization};
        use crate::fft::window::WindowFunction;
        let (cycles, n) = (127.0_f64, 4096_usize);
        let step = cycles * 1.0e-3 / n as f64;
        let time = (0..=n).map(|k| k as f64 * step).collect::<Vec<_>>();
        let values = time
            .iter()
            .map(|t| (2.0 * PI * 1.0e3 * t).sin())
            .collect::<Vec<_>>();
        for window in [None, Some(FftTimeWindow::new(0.0, cycles * 1.0e-3))] {
            let options =
                FftInputOptions::with_policy(FftInputPolicy::Reference).with_time_window(window);
            let prepared = prepare_fft_input_with_options("V(out)", &time, &values, options)
                .expect("a coherent record");
            assert_eq!(prepared.samples.len(), n);
            assert!((prepared.sample_rate - n as f64 / (cycles * 1.0e-3)).abs() < 1.0e-6);
            let data = FftData::from_time_domain_with_normalization(
                "V(out)",
                &prepared.samples,
                prepared.sample_rate,
                WindowFunction::Rectangular,
                SpectrumNormalization::Rms,
            )
            .expect("a spectrum");
            let analysis = SpectrumAnalysis::analyze(&data, 10).expect("its figures");
            assert_eq!(analysis.fundamental_bin, Some(127));
            assert!((analysis.fundamental_frequency.unwrap() - 1.0e3).abs() < 1.0e-9);
            assert!((analysis.fundamental_db.unwrap() + 3.010_299_956_639_812).abs() < 1.0e-9);
            assert!(analysis.snr_db.unwrap() > 250.0, "{analysis:?}");
            assert!(analysis.sfdr_db.unwrap() > 250.0, "{analysis:?}");
            assert!(analysis.thd_percent.unwrap() < 1.0e-10, "{analysis:?}");
        }
    }

    /// The steady state of a 1 kHz, 0 to 1 V square wave through an RC low
    /// pass of τ = 100 µs, as a transient writes it: fine steps at each edge,
    /// coarse between. Each half period is an exponential towards the rail,
    /// `v(t) = rail + (v₀ − rail)·e^(−t/τ)`, starting from the level the last
    /// one ended at, `v₀ = 1/(1 + e^(−T/2τ))` after a rising edge.
    fn rc_square_wave(cycles: usize) -> (Vec<f64>, Vec<f64>) {
        let (period, tau) = (1.0e-3_f64, 1.0e-4_f64);
        let half = period / 2.0;
        let high_start = 1.0 / (1.0 + (-half / tau).exp());
        let low_start = 1.0 - high_start;
        let level = |t: f64| {
            let phase = t.rem_euclid(period);
            if phase < half {
                1.0 + (low_start - 1.0) * (-phase / tau).exp()
            } else {
                high_start * (-(phase - half) / tau).exp()
            }
        };
        let mut time = vec![0.0];
        let mut t = 0.0_f64;
        let end = cycles as f64 * period;
        while t < end {
            let since_edge = t.rem_euclid(half);
            // 1 ns steps for 20 ns after each edge, then growing to 10 µs.
            let step = if since_edge < 2.0e-8 {
                1.0e-9
            } else {
                (since_edge * 0.05).clamp(1.0e-9, 1.0e-5)
            };
            let next_edge = (t / half).floor() * half + half;
            t = (t + step).min(next_edge).min(end);
            time.push(t);
        }
        let values = time.iter().map(|t| level(*t)).collect();
        (time, values)
    }

    /// Auto N resamples a record whose steps are not uniform, rather than
    /// refusing it on a timestamp check: a transient's switching edges take
    /// steps a billionth of their timestamps, and the record once got no
    /// spectrum at all. The THD through HD10 is the Fourier series' answer,
    /// 20.27 %: the odd harmonics `4/(πk)` of the square wave, each through
    /// the low pass `1/√(1 + (2πk·f₀τ)²)`.
    #[test]
    fn auto_n_resamples_a_transient_and_reads_its_fourier_series() {
        use crate::fft::data::{FftData, SpectrumAnalysis, SpectrumNormalization};
        use crate::fft::window::WindowFunction;
        let (time, values) = rc_square_wave(20);
        let prepared = prepare_fft_input_with_options(
            "V(out)",
            &time,
            &values,
            FftInputOptions::with_policy(FftInputPolicy::Reference),
        )
        .expect("a transient record resamples");
        let data = FftData::from_time_domain_with_normalization(
            "V(out)",
            &prepared.samples,
            prepared.sample_rate,
            WindowFunction::Hanning,
            SpectrumNormalization::Rms,
        )
        .expect("a spectrum");
        let analysis = SpectrumAnalysis::analyze(&data, 10).expect("its figures");
        let amplitude = |k: f64| 4.0 / (PI * k) / (1.0 + (2.0 * PI * k * 0.1).powi(2)).sqrt();
        let thd = 100.0
            * [3.0, 5.0, 7.0, 9.0]
                .iter()
                .map(|k| amplitude(*k).powi(2))
                .sum::<f64>()
                .sqrt()
            / amplitude(1.0);
        assert!((thd - 20.27).abs() < 0.01, "the closed form itself: {thd}");
        let measured = analysis.thd_percent.expect("THD");
        assert!(
            (measured - thd).abs() < 0.03,
            "THD {measured} % against the Fourier series' {thd} %"
        );
        // The fundamental's rms level, 0.5 of the square's 4/π, through the
        // low pass.
        let level = 20.0 * (amplitude(1.0) / 2.0 / std::f64::consts::SQRT_2).log10();
        assert!(
            (analysis.fundamental_db.unwrap() - level).abs() < 0.01,
            "{} against {level}",
            analysis.fundamental_db.unwrap()
        );
        // The record's mean, taken out before the transform, is its DC.
        assert!(
            (prepared.dc_level - 0.5).abs() < 1.0e-3,
            "{}",
            prepared.dc_level
        );
    }
}
