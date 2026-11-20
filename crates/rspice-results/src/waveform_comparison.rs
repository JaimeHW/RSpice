//! Comparing two runs of one analysis, signal by signal, read by reference.
//!
//! A comparison reads each run's X axis once, then each chosen signal of both
//! runs in turn, from wherever the run keeps it. A stored run's columns are
//! read for the comparison alone and never kept by the run, so nothing of
//! either run is held once the comparison ends, and comparing three signals of
//! a twenty-signal run reads those three. Before any sample is compared the two
//! runs are brought onto one set of rows under the alignment the reader chose,
//! and the receipt records that alignment, its interpolation and resampling.

use std::borrow::Cow;

use crate::analysis_result::AnalysisResult;
#[cfg(test)]
use crate::analysis_type::AnalysisType;
use crate::interpolation::linear_between;
use crate::run::SimulationRun;
use crate::visualization_document::{
    ComparisonAlignmentMethod, ComparisonDisposition, ComparisonExecutionContract,
    ComparisonExtrapolationPolicy, ComparisonInterpolationPolicy, ComparisonPolicy,
    ComparisonPrecisionPolicy, ComparisonReceipt, ComparisonResamplingPolicy, ErrorTally,
    NumericTolerance, RowAlignmentPolicy,
};
use crate::waveform::{RetainedWaveform, WaveformValues};
use rspice_vocabulary::product::DatasetBinding;

/// How the baseline and candidate series are brought onto one X axis before
/// they are compared.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ComparisonAlignmentDraft {
    #[default]
    FirstThresholdCrossing,
    AbsoluteXAxis,
    CrossCorrelation,
}

impl ComparisonAlignmentDraft {
    pub const ALL: [Self; 3] = [
        Self::FirstThresholdCrossing,
        Self::AbsoluteXAxis,
        Self::CrossCorrelation,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::FirstThresholdCrossing => "First threshold crossing",
            Self::AbsoluteXAxis => "Absolute X axis",
            Self::CrossCorrelation => "Cross-correlation alignment",
        }
    }
}

pub fn matching_comparison_analysis<'a, A, W>(
    active: &AnalysisResult<W>,
    run: &'a SimulationRun<A>,
) -> Option<&'a A>
where
    A: AsRef<AnalysisResult<W>>,
    W: AsRef<RetainedWaveform>,
{
    if let Some(source_id) = active
        .provenance
        .as_ref()
        .map(|provenance| provenance.authored_source_instance_id())
    {
        return run
            .find_analysis_by_source_instance(source_id)
            .filter(|analysis| analysis.as_ref().analysis_type == active.analysis_type);
    }
    let mut exact = run.analyses.iter().filter(|analysis| {
        let analysis: &AnalysisResult<W> = analysis.as_ref();
        analysis.provenance.is_none()
            && analysis.analysis_type == active.analysis_type
            && analysis.label == active.label
    });
    let candidate = exact.next()?;
    exact.next().is_none().then_some(candidate)
}

/// One run of a comparison: the dataset it is, and its analysis being
/// compared.
#[derive(Debug)]
pub struct ComparisonSide<'a, W> {
    pub binding: DatasetBinding,
    pub analysis: &'a AnalysisResult<W>,
}

impl<W> Clone for ComparisonSide<'_, W> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<W> Copy for ComparisonSide<'_, W> {}

fn validate_strict_axis(name: &str, axis: &[f64]) -> Result<(), String> {
    if axis.is_empty() {
        return Err(format!("{name} has no X axis samples."));
    }
    if axis.iter().any(|value| !value.is_finite()) {
        return Err(format!(
            "{name} has a missing X axis sample, so it cannot be compared."
        ));
    }
    if axis.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(format!(
            "{name} has an X axis that does not rise from sample to sample (nonmonotonic), so it cannot be compared."
        ));
    }
    Ok(())
}

fn exact_axis(left: &[f64], right: &[f64]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| left.to_bits() == right.to_bits())
}

fn unique_waveform<'a, W: AsRef<RetainedWaveform>>(
    analysis: &'a AnalysisResult<W>,
    signal_name: &str,
    run: &str,
) -> Result<&'a RetainedWaveform, String> {
    let mut matching = analysis
        .waveforms
        .iter()
        .map(AsRef::as_ref)
        .filter(|waveform| waveform.name == signal_name);
    let waveform = matching
        .next()
        .ok_or_else(|| format!("The {run} holds no signal named {signal_name}."))?;
    if matching.next().is_some() {
        return Err(format!(
            "The {run} holds two signals named {signal_name}, so which to compare is ambiguous."
        ));
    }
    Ok(waveform)
}

/// A column's samples for this comparison alone. A stored window is read
/// here and dropped with the borrow; the waveform never keeps it.
fn read_column<'v>(values: &'v WaveformValues, what: &str) -> Result<Cow<'v, [f64]>, String> {
    values
        .read_range(0..values.len())
        .map_err(|failure| format!("{what} could not be read: {failure}"))
}

/// Whether two columns are the same samples, known without reading either:
/// one allocation, or the same window of a stored column whose store states
/// its identity.
fn same_column(left: &WaveformValues, right: &WaveformValues) -> bool {
    WaveformValues::ptr_eq(left, right)
        || match (left.stored_window(), right.stored_window()) {
            (Some((left_source, left_window)), Some((right_source, right_window))) => {
                left_window == right_window
                    && left_source.identity().is_some()
                    && left_source.identity() == right_source.identity()
            }
            _ => false,
        }
}

/// Whether `x` looks like the axis `reference` is, as far as can be told
/// without reading either: the same column, or one of the same length that
/// starts and ends where it does. The comparison checks the samples.
fn shares_axis(x: &WaveformValues, reference: &WaveformValues) -> bool {
    same_column(x, reference) || (x.len() == reference.len() && x.ends() == reference.ends())
}

fn unit_phrase(unit: Option<&str>) -> &str {
    unit.unwrap_or("no stated unit")
}

fn require_same_x_unit<W: AsRef<RetainedWaveform>>(
    candidate: &AnalysisResult<W>,
    baseline: &AnalysisResult<W>,
) -> Result<(), String> {
    let (here, there) = (
        candidate.waveform_coordinate_unit(),
        baseline.waveform_coordinate_unit(),
    );
    if here != there {
        return Err(format!(
            "The runs sweep different quantities: the X axis is in {} here and in {} in the baseline.",
            unit_phrase(here),
            unit_phrase(there)
        ));
    }
    Ok(())
}

/// The signals two analyses can be compared on: each waveform on the
/// candidate's X axis that the baseline holds once, on its own axis, with a
/// sample for every X. It is read from names, units and lengths alone, so it
/// reads no sample of either run; the samples are checked as a comparison
/// reads them.
pub fn comparison_signal_names<W: AsRef<RetainedWaveform>>(
    candidate: &AnalysisResult<W>,
    baseline: &AnalysisResult<W>,
) -> Result<Vec<String>, String> {
    require_same_x_unit(candidate, baseline)?;
    let reference = candidate
        .waveforms
        .first()
        .map(AsRef::as_ref)
        .ok_or_else(|| "The result holds no signals to compare.".to_owned())?;
    let mut baseline_axis: Option<&WaveformValues> = None;
    let mut names = Vec::new();
    for waveform in candidate
        .waveforms
        .iter()
        .map(AsRef::as_ref)
        .filter(|waveform| {
            waveform.y.len() == waveform.x.len() && shares_axis(&waveform.x, &reference.x)
        })
    {
        let mut matching = baseline
            .waveforms
            .iter()
            .map(AsRef::as_ref)
            .filter(|other| other.name == waveform.name);
        let Some(other) = matching.next() else {
            continue;
        };
        if matching.next().is_some() {
            return Err(format!(
                "The baseline holds two signals named {}, so which to compare is ambiguous.",
                waveform.name
            ));
        }
        if other.y.len() != other.x.len() {
            continue;
        }
        let axis = *baseline_axis.get_or_insert(&other.x);
        if !shares_axis(&other.x, axis) {
            continue;
        }
        if waveform.unit != other.unit {
            return Err(format!(
                "{} is in {} here and in {} in the baseline; compare samples in the same unit.",
                waveform.name,
                unit_phrase(waveform.unit.as_deref()),
                unit_phrase(other.unit.as_deref()),
            ));
        }
        if !names.contains(&waveform.name) {
            names.push(waveform.name.clone());
        }
    }
    names.sort();
    if names.is_empty() {
        return Err("The runs share no signal on this X axis.".to_owned());
    }
    Ok(names)
}

/// Where a target sits on a strictly ascending axis: on a sample, or between
/// one and the next.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Bracket {
    On(usize),
    Between { lower: usize, target: f64 },
}

/// Find `target` on `axis`, which ascends strictly. A target outside the
/// axis is refused: nothing is extrapolated.
pub(crate) fn bracket(axis: &[f64], target: f64) -> Result<Bracket, String> {
    if axis.is_empty() || !target.is_finite() || target < axis[0] || target > axis[axis.len() - 1] {
        return Err(
            "Interpolation target lies outside the retained monotone source axis.".to_owned(),
        );
    }
    match axis.binary_search_by(|value| value.total_cmp(&target)) {
        Ok(index) => Ok(Bracket::On(index)),
        Err(upper) if upper > 0 && upper < axis.len() => Ok(Bracket::Between {
            lower: upper - 1,
            target,
        }),
        _ => Err("Interpolation target is not bracketed by retained samples.".to_owned()),
    }
}

impl Bracket {
    /// The value of `values` here: the sample itself, or the straight line
    /// between the two samples around it.
    pub(crate) fn value(self, axis: &[f64], values: &[f64]) -> Result<f64, String> {
        match self {
            Self::On(index) => Ok(values[index]),
            Self::Between { lower, target } => {
                let value = linear_between(
                    axis[lower],
                    axis[lower + 1],
                    values[lower],
                    values[lower + 1],
                    target,
                );
                value.is_finite().then_some(value).ok_or_else(|| {
                    "Monotone linear interpolation produced a non-finite value.".to_owned()
                })
            }
        }
    }

    /// [`Self::value`] drawn under `warp`: straight in X, or straight in
    /// log X between two samples of a sweep spaced by a constant ratio.
    fn value_warped(self, warp: AxisWarp, axis: &[f64], values: &[f64]) -> Result<f64, String> {
        match (self, warp) {
            (Self::Between { lower, target }, AxisWarp::Log) => {
                let value = linear_between(
                    axis[lower].ln(),
                    axis[lower + 1].ln(),
                    values[lower],
                    values[lower + 1],
                    target.ln(),
                );
                value.is_finite().then_some(value).ok_or_else(|| {
                    "Monotone log-X interpolation produced a non-finite value.".to_owned()
                })
            }
            _ => self.value(axis, values),
        }
    }

    /// Where on the axis this is.
    fn x(self, axis: &[f64]) -> f64 {
        match self {
            Self::On(index) => axis[index],
            Self::Between { target, .. } => target,
        }
    }
}

/// How a resampled signal runs between two of its samples: straight in X, or
/// straight in log X on a sweep spaced by a constant ratio, as a decade sweep
/// is, where a straight line in X would bow between the points.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisWarp {
    Linear,
    Log,
}

impl AxisWarp {
    /// The warp `axis` is spaced by: log when every X is positive and each
    /// sample is the same ratio past the one before, rather than the same
    /// step.
    #[must_use]
    pub fn of(axis: &[f64]) -> Self {
        let count = axis.len();
        if count < 3 || axis.iter().any(|x| !(x.is_finite() && *x > 0.0)) {
            return Self::Linear;
        }
        let (first_ratio, last_ratio) = (axis[1] / axis[0], axis[count - 1] / axis[count - 2]);
        let (first_step, last_step) = (axis[1] - axis[0], axis[count - 1] - axis[count - 2]);
        let geometric =
            (first_ratio - last_ratio).abs() <= 1e-6 * first_ratio && first_ratio > 1.0 + 1e-9;
        let arithmetic = (first_step - last_step).abs() <= 1e-6 * first_step.abs();
        if geometric && !arithmetic {
            Self::Log
        } else {
            Self::Linear
        }
    }

    /// What a trace says of it.
    #[must_use]
    pub const fn words(self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::Log => "log X",
        }
    }
}

/// `values`, sampled on `source`, at each X of `target`, by the comparison's
/// own monotone interpolation under `warp`. A target outside `source`, or
/// beside a missing sample, is a missing sample: nothing is extrapolated.
/// `source` must ascend strictly; see [`check_resampling_axis`].
#[must_use]
pub fn resample_onto(source: &[f64], values: &[f64], target: &[f64], warp: AxisWarp) -> Vec<f64> {
    if source.len() != values.len() {
        return vec![f64::NAN; target.len()];
    }
    target
        .iter()
        .map(|&x| {
            bracket(source, x)
                .and_then(|at| at.value_warped(warp, source, values))
                .unwrap_or(f64::NAN)
        })
        .collect()
}

/// Whether `axis` can be resampled from: it holds samples, every one
/// present, each past the one before. `name` names it in the refusal.
pub fn check_resampling_axis(name: &str, axis: &[f64]) -> Result<(), String> {
    validate_strict_axis(name, axis)
}

#[cfg(test)]
fn interpolate_monotone(axis: &[f64], values: &[f64], target: f64) -> Result<f64, String> {
    if axis.len() != values.len() {
        return Err(
            "Interpolation target lies outside the retained monotone source axis.".to_owned(),
        );
    }
    bracket(axis, target)?.value(axis, values)
}

fn first_threshold_crossing(axis: &[f64], values: &[f64], threshold: f64) -> Result<f64, String> {
    if axis.len() != values.len() || axis.len() < 2 || !threshold.is_finite() {
        return Err(
            "Threshold alignment requires at least two finite coordinate/value samples.".to_owned(),
        );
    }
    for index in 0..axis.len() - 1 {
        let left = values[index];
        let right = values[index + 1];
        if left.to_bits() == threshold.to_bits() {
            return Ok(axis[index]);
        }
        if (left < threshold && right >= threshold) || (left > threshold && right <= threshold) {
            if left == right {
                continue;
            }
            // The crossing interpolates the axis as a function of the values.
            let crossing = linear_between(left, right, axis[index], axis[index + 1], threshold);
            if crossing.is_finite() {
                return Ok(crossing);
            }
        }
    }
    if values[values.len() - 1].to_bits() == threshold.to_bits() {
        return Ok(axis[axis.len() - 1]);
    }
    Err("The alignment signal never crosses the threshold.".to_owned())
}

/// The rows both axes hold exactly, as each run's sample index.
fn exact_intersection(
    baseline_axis: &[f64],
    candidate_axis: &[f64],
) -> Result<(Vec<usize>, Vec<usize>), String> {
    let (mut baseline_rows, mut candidate_rows) = (Vec::new(), Vec::new());
    let (mut baseline_index, mut candidate_index) = (0_usize, 0_usize);
    while baseline_index < baseline_axis.len() && candidate_index < candidate_axis.len() {
        let baseline_x = baseline_axis[baseline_index];
        let candidate_x = candidate_axis[candidate_index];
        if baseline_x.to_bits() == candidate_x.to_bits() {
            baseline_rows.push(baseline_index);
            candidate_rows.push(candidate_index);
            baseline_index += 1;
            candidate_index += 1;
        } else if baseline_x < candidate_x {
            baseline_index += 1;
        } else {
            candidate_index += 1;
        }
    }
    if candidate_rows.is_empty() {
        return Err(
            "The runs share no X value exactly; align by threshold or correlation instead."
                .to_owned(),
        );
    }
    Ok((baseline_rows, candidate_rows))
}

fn uniform_grid(start: f64, end: f64, count: usize) -> Result<(Vec<f64>, f64), String> {
    if !start.is_finite() || !end.is_finite() || start >= end || count < 3 {
        return Err("The runs' X axes do not overlap.".to_owned());
    }
    let interval = (end - start) / (count - 1) as f64;
    if !interval.is_finite() || interval <= 0.0 {
        return Err("The uniform comparison interval is not representable.".to_owned());
    }
    let mut grid = (0..count)
        .map(|index| start + interval * index as f64)
        .collect::<Vec<_>>();
    grid[count - 1] = end;
    Ok((grid, interval))
}

fn correlation_score(baseline: &[f64], candidate: &[f64], lag: i64) -> Result<f64, String> {
    let (baseline_start, candidate_start) = if lag >= 0 {
        (0_usize, lag as usize)
    } else {
        (lag.unsigned_abs() as usize, 0_usize)
    };
    let count = baseline
        .len()
        .saturating_sub(baseline_start)
        .min(candidate.len().saturating_sub(candidate_start));
    if count < 3 {
        return Err(
            "Cross-correlation lag leaves fewer than three overlapping samples.".to_owned(),
        );
    }
    let baseline_slice = &baseline[baseline_start..baseline_start + count];
    let candidate_slice = &candidate[candidate_start..candidate_start + count];
    let baseline_mean = baseline_slice.iter().sum::<f64>() / count as f64;
    let candidate_mean = candidate_slice.iter().sum::<f64>() / count as f64;
    let mut covariance = 0.0;
    let mut baseline_energy = 0.0;
    let mut candidate_energy = 0.0;
    for (baseline, candidate) in baseline_slice.iter().zip(candidate_slice) {
        let baseline_centered = baseline - baseline_mean;
        let candidate_centered = candidate - candidate_mean;
        covariance += baseline_centered * candidate_centered;
        baseline_energy += baseline_centered * baseline_centered;
        candidate_energy += candidate_centered * candidate_centered;
    }
    let denominator = (baseline_energy * candidate_energy).sqrt();
    if !denominator.is_finite() || denominator == 0.0 {
        return Err(
            "The alignment signal is flat, so correlation cannot find where the runs line up."
                .to_owned(),
        );
    }
    let score = (covariance / denominator).clamp(-1.0, 1.0);
    score
        .is_finite()
        .then_some(score)
        .ok_or_else(|| "Cross-correlation produced a non-finite coefficient.".to_owned())
}

fn cross_correlation_lag(
    baseline: &[f64],
    candidate: &[f64],
    maximum_lag_samples: u32,
) -> Result<(i64, f64), String> {
    if baseline.len() != candidate.len() || baseline.len() < 3 || maximum_lag_samples == 0 {
        return Err("Max lag must be at least one sample.".to_owned());
    }
    let maximum_lag = maximum_lag_samples as usize;
    if maximum_lag > baseline.len().saturating_sub(3) {
        return Err(format!(
            "Max lag {maximum_lag_samples} leaves fewer than three overlapping samples; reduce it."
        ));
    }
    let work = baseline
        .len()
        .checked_mul(maximum_lag.saturating_mul(2).saturating_add(1))
        .ok_or_else(|| "Cross-correlation work estimate overflowed.".to_owned())?;
    const MAX_CORRELATION_WORK: usize = 32_000_000;
    if work > MAX_CORRELATION_WORK {
        return Err(format!(
            "Cross-correlation would evaluate {work} sample pairs; reduce maximum lag below the {MAX_CORRELATION_WORK}-pair execution bound."
        ));
    }
    let mut best: Option<(i64, f64)> = None;
    for lag in -(i64::from(maximum_lag_samples))..=i64::from(maximum_lag_samples) {
        let score = correlation_score(baseline, candidate, lag)?;
        let replace = best.is_none_or(|(best_lag, best_score)| {
            score > best_score + 1.0e-15
                || ((score - best_score).abs() <= 1.0e-15
                    && (lag.unsigned_abs(), lag) < (best_lag.unsigned_abs(), best_lag))
        });
        if replace {
            best = Some((lag, score));
        }
    }
    best.ok_or_else(|| "No valid cross-correlation lag was evaluated.".to_owned())
}

/// How one run's samples become the compared rows.
enum RowSource {
    /// Sample `rows[i]` of the run at row `i`.
    Samples(Vec<usize>),
    /// The run interpolated at each row on `axis`: the run's own axis, moved
    /// as the alignment moved it.
    Interpolated { axis: Vec<f64>, at: Vec<Bracket> },
}

impl RowSource {
    fn len(&self) -> usize {
        match self {
            Self::Samples(rows) => rows.len(),
            Self::Interpolated { at, .. } => at.len(),
        }
    }

    /// Row `row` of `values`, the run's samples of one signal.
    fn value(&self, row: usize, values: &[f64]) -> Result<f64, String> {
        match self {
            Self::Samples(rows) => Ok(values[rows[row]]),
            Self::Interpolated { axis, at } => at[row].value(axis, values),
        }
    }

    /// Where row `row` falls on the run's own axis, `own_axis`.
    fn x(&self, row: usize, own_axis: &[f64]) -> f64 {
        match self {
            Self::Samples(rows) => own_axis[rows[row]],
            Self::Interpolated { axis, at } => at[row].x(axis),
        }
    }
}

/// The two runs on one set of rows, and the contract that put them there.
struct Alignment {
    baseline: RowSource,
    candidate: RowSource,
    execution: ComparisonExecutionContract,
}

/// Explicit policies for a single immutable comparison; no application selection state.
pub struct ComparisonOptions<'a> {
    pub alignment: ComparisonAlignmentDraft,
    pub alignment_signal: &'a str,
    pub threshold: f64,
    pub maximum_lag_samples: u32,
    pub absolute_tolerance: f64,
    pub relative_tolerance: f64,
}

/// One place a compared signal parts from its baseline by more than the
/// tolerance: a stretch of consecutive compared rows, on the candidate's own
/// X axis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComparisonPlace {
    /// The signal, by its place in [`ComparisonOutcome::signals`].
    pub signal: usize,
    /// Where the stretch starts and ends.
    pub first: f64,
    pub last: f64,
    /// Where in it the runs part most, and by how much: candidate minus
    /// baseline, in the signal's unit.
    pub worst_x: f64,
    pub worst_error: f64,
}

/// What a comparison found: its receipt, the signals it compared with their
/// units, and every place they part from the baseline.
#[derive(Debug, Clone, PartialEq)]
pub struct ComparisonOutcome {
    pub receipt: ComparisonReceipt,
    /// The compared signals; the receipt keys each by its place here.
    pub signals: Vec<String>,
    pub units: Vec<Option<String>>,
    /// The unit of the candidate's X axis.
    pub coordinate_unit: Option<String>,
    /// Every place a signal parts from its baseline, in signal order and
    /// then along X.
    pub places: Vec<ComparisonPlace>,
}

/// One signal's columns on both runs, checked before any sample is read.
struct ComparedSignal<'a> {
    candidate: &'a RetainedWaveform,
    baseline: &'a RetainedWaveform,
}

fn compared_signal<'a, W: AsRef<RetainedWaveform>>(
    candidate: &'a AnalysisResult<W>,
    baseline: &'a AnalysisResult<W>,
    name: &str,
) -> Result<ComparedSignal<'a>, String> {
    let signal = ComparedSignal {
        candidate: unique_waveform(candidate, name, "result")?,
        baseline: unique_waveform(baseline, name, "baseline")?,
    };
    if signal.candidate.unit != signal.baseline.unit {
        return Err(format!(
            "{name} is in {} here and in {} in the baseline; compare samples in the same unit.",
            unit_phrase(signal.candidate.unit.as_deref()),
            unit_phrase(signal.baseline.unit.as_deref()),
        ));
    }
    for (waveform, run) in [(signal.candidate, "result"), (signal.baseline, "baseline")] {
        if waveform.y.len() != waveform.x.len() {
            return Err(format!(
                "{name} in the {run} does not hold a sample for every X, so it cannot be compared."
            ));
        }
    }
    Ok(signal)
}

/// The X axis one run's chosen signals share, read once.
fn shared_axis<'a>(
    waveforms: impl IntoIterator<Item = &'a RetainedWaveform>,
    run: &str,
) -> Result<Cow<'a, [f64]>, String> {
    let mut waveforms = waveforms.into_iter();
    let reference = waveforms
        .next()
        .ok_or_else(|| "Choose at least one signal to compare.".to_owned())?;
    let axis = read_column(&reference.x, &format!("The {run}'s X axis"))?;
    validate_strict_axis(&format!("{} in the {run}", reference.name), &axis)?;
    for waveform in waveforms {
        if same_column(&waveform.x, &reference.x) {
            continue;
        }
        let shares = waveform.x.len() == axis.len()
            && exact_axis(
                &read_column(&waveform.x, &format!("The X axis of {}", waveform.name))?,
                &axis,
            );
        if !shares {
            return Err(format!(
                "{} in the {run} is not on the X axis of {}, so the two cannot be compared together.",
                waveform.name, reference.name
            ));
        }
    }
    Ok(axis)
}

/// Every sample of one signal of one run, read for this comparison alone.
fn signal_samples<'a>(waveform: &'a RetainedWaveform, run: &str) -> Result<Cow<'a, [f64]>, String> {
    let samples = read_column(&waveform.y, &format!("{} in the {run}", waveform.name))?;
    if samples.iter().any(|value| !value.is_finite()) {
        return Err(format!(
            "{} has a missing sample in the {run}, so it cannot be compared.",
            waveform.name
        ));
    }
    Ok(samples)
}

fn align(
    candidate_axis: &[f64],
    baseline_axis: &[f64],
    alignment_signal: Option<(&[f64], &[f64])>,
    options: &ComparisonOptions<'_>,
    alignment_key: String,
) -> Result<Alignment, String> {
    let exact = |alignment| ComparisonExecutionContract {
        alignment,
        interpolation: ComparisonInterpolationPolicy::NoneExactOnly,
        resampling: ComparisonResamplingPolicy::ExactCoordinateIntersection,
        extrapolation: ComparisonExtrapolationPolicy::Forbid,
        precision: ComparisonPrecisionPolicy::SourceF64NoRounding,
    };
    let interpolated = |alignment, resampling| ComparisonExecutionContract {
        alignment,
        interpolation: ComparisonInterpolationPolicy::MonotoneLinear,
        resampling,
        extrapolation: ComparisonExtrapolationPolicy::Forbid,
        precision: ComparisonPrecisionPolicy::SourceF64NoRounding,
    };
    match options.alignment {
        ComparisonAlignmentDraft::AbsoluteXAxis => {
            let (baseline_rows, candidate_rows) =
                exact_intersection(baseline_axis, candidate_axis)?;
            Ok(Alignment {
                baseline: RowSource::Samples(baseline_rows),
                candidate: RowSource::Samples(candidate_rows),
                execution: exact(ComparisonAlignmentMethod::AbsoluteXAxis),
            })
        }
        ComparisonAlignmentDraft::FirstThresholdCrossing => {
            let (candidate_values, baseline_values) = alignment_signal.ok_or_else(|| {
                "Choose the signal whose threshold crossing aligns the runs.".to_owned()
            })?;
            let threshold = options.threshold;
            let baseline_crossing =
                first_threshold_crossing(baseline_axis, baseline_values, threshold)?;
            let candidate_crossing =
                first_threshold_crossing(candidate_axis, candidate_values, threshold)?;
            let baseline_shifted = baseline_axis
                .iter()
                .map(|coordinate| coordinate - baseline_crossing)
                .collect::<Vec<_>>();
            let candidate_shifted = candidate_axis
                .iter()
                .map(|coordinate| coordinate - candidate_crossing)
                .collect::<Vec<_>>();
            let overlap_start = baseline_shifted[0].max(candidate_shifted[0]);
            let overlap_end = baseline_shifted[baseline_shifted.len() - 1]
                .min(candidate_shifted[candidate_shifted.len() - 1]);
            let candidate_rows = candidate_shifted
                .iter()
                .enumerate()
                .filter_map(|(index, coordinate)| {
                    (*coordinate >= overlap_start && *coordinate <= overlap_end).then_some(index)
                })
                .collect::<Vec<_>>();
            if candidate_rows.len() < 2 {
                return Err(
                    "Aligned at their threshold crossings, the runs overlap by fewer than two samples."
                        .to_owned(),
                );
            }
            let at = candidate_rows
                .iter()
                .map(|index| bracket(&baseline_shifted, candidate_shifted[*index]))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Alignment {
                baseline: RowSource::Interpolated {
                    axis: baseline_shifted,
                    at,
                },
                candidate: RowSource::Samples(candidate_rows),
                execution: interpolated(
                    ComparisonAlignmentMethod::FirstThresholdCrossing {
                        signal_key: alignment_key,
                        threshold,
                        baseline_crossing,
                        candidate_crossing,
                    },
                    ComparisonResamplingPolicy::BaselineOntoCandidateGrid,
                ),
            })
        }
        ComparisonAlignmentDraft::CrossCorrelation => {
            let (candidate_values, baseline_values) = alignment_signal
                .ok_or_else(|| "Choose the signal whose correlation aligns the runs.".to_owned())?;
            let maximum_lag_samples = options.maximum_lag_samples;
            let overlap_start = baseline_axis[0].max(candidate_axis[0]);
            let overlap_end = baseline_axis[baseline_axis.len() - 1]
                .min(candidate_axis[candidate_axis.len() - 1]);
            let sample_count = baseline_axis.len().max(candidate_axis.len());
            let (correlation_grid, sample_interval) =
                uniform_grid(overlap_start, overlap_end, sample_count)?;
            let on_grid = |axis: &[f64], values: &[f64]| {
                correlation_grid
                    .iter()
                    .map(|target| bracket(axis, *target)?.value(axis, values))
                    .collect::<Result<Vec<_>, _>>()
            };
            let baseline_alignment = on_grid(baseline_axis, baseline_values)?;
            let candidate_alignment = on_grid(candidate_axis, candidate_values)?;
            let (selected_lag_samples, coefficient) = cross_correlation_lag(
                &baseline_alignment,
                &candidate_alignment,
                maximum_lag_samples,
            )?;
            let baseline_shift = selected_lag_samples as f64 * sample_interval;
            let aligned_start = candidate_axis[0].max(baseline_axis[0] + baseline_shift);
            let aligned_end = candidate_axis[candidate_axis.len() - 1]
                .min(baseline_axis[baseline_axis.len() - 1] + baseline_shift);
            if aligned_start >= aligned_end {
                return Err("Shifted to line up, the runs no longer overlap.".to_owned());
            }
            let final_count =
                (((aligned_end - aligned_start) / sample_interval).floor() as usize) + 1;
            if final_count < 3 {
                return Err(
                    "Shifted to line up, the runs overlap by fewer than three samples.".to_owned(),
                );
            }
            let coordinates = (0..final_count)
                .map(|index| aligned_start + sample_interval * index as f64)
                .collect::<Vec<_>>();
            let baseline_at = coordinates
                .iter()
                .map(|coordinate| bracket(baseline_axis, coordinate - baseline_shift))
                .collect::<Result<Vec<_>, _>>()?;
            let candidate_at = coordinates
                .iter()
                .map(|coordinate| bracket(candidate_axis, *coordinate))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Alignment {
                baseline: RowSource::Interpolated {
                    axis: baseline_axis.to_vec(),
                    at: baseline_at,
                },
                candidate: RowSource::Interpolated {
                    axis: candidate_axis.to_vec(),
                    at: candidate_at,
                },
                execution: interpolated(
                    ComparisonAlignmentMethod::CrossCorrelation {
                        signal_key: alignment_key,
                        maximum_lag_samples,
                        selected_lag_samples,
                        sample_interval,
                        coefficient,
                        baseline_shift,
                    },
                    ComparisonResamplingPolicy::UniformOverlapGrid,
                ),
            })
        }
    }
}

/// Compare `signals` of the candidate with the baseline, reading each run's
/// columns as the comparison reaches them and keeping none. `progress` hears
/// how many signals are done of how many after each one, and stops the
/// comparison by answering `false`.
pub fn execute_comparison<W: AsRef<RetainedWaveform>>(
    candidate: ComparisonSide<'_, W>,
    baseline: ComparisonSide<'_, W>,
    signals: &[String],
    options: ComparisonOptions<'_>,
    progress: &mut dyn FnMut(usize, usize) -> bool,
) -> Result<ComparisonOutcome, String> {
    if signals.is_empty() {
        return Err("Choose at least one signal to compare.".to_owned());
    }
    if candidate.binding.dataset_id == baseline.binding.dataset_id {
        return Err("A run cannot be compared with itself; choose another baseline.".to_owned());
    }
    require_same_x_unit(candidate.analysis, baseline.analysis)?;
    let tolerance = NumericTolerance::new(options.absolute_tolerance, options.relative_tolerance)
        .map_err(|_| "Tolerances must be finite and not negative.".to_owned())?;
    let compared = signals
        .iter()
        .map(|name| compared_signal(candidate.analysis, baseline.analysis, name))
        .collect::<Result<Vec<_>, _>>()?;
    let aligning = options.alignment != ComparisonAlignmentDraft::AbsoluteXAxis;
    let alignment_signal = aligning
        .then(|| {
            compared_signal(
                candidate.analysis,
                baseline.analysis,
                options.alignment_signal,
            )
        })
        .transpose()
        .map_err(|error| format!("The alignment signal cannot be read: {error}"))?;
    let read = || compared.iter().chain(&alignment_signal);
    let candidate_axis = shared_axis(read().map(|signal| signal.candidate), "result")?;
    let baseline_axis = shared_axis(read().map(|signal| signal.baseline), "baseline")?;
    let alignment_key = alignment_signal
        .as_ref()
        .and_then(|_| {
            signals
                .iter()
                .position(|name| name == options.alignment_signal)
        })
        .map_or_else(|| "alignment".to_owned(), |index| format!("signal:{index}"));
    let alignment = {
        let samples = alignment_signal
            .as_ref()
            .map(|signal| {
                Ok::<_, String>((
                    signal_samples(signal.candidate, "result")?,
                    signal_samples(signal.baseline, "baseline")?,
                ))
            })
            .transpose()?;
        align(
            &candidate_axis,
            &baseline_axis,
            samples
                .as_ref()
                .map(|(candidate, baseline)| (&candidate[..], &baseline[..])),
            &options,
            alignment_key,
        )?
    };
    alignment
        .execution
        .validate()
        .map_err(|error| error.to_string())?;
    let rows = alignment.candidate.len();
    let mut tallies = Vec::with_capacity(compared.len());
    let mut places = Vec::new();
    for (index, signal) in compared.iter().enumerate() {
        let candidate_values = signal_samples(signal.candidate, "result")?;
        let baseline_values = signal_samples(signal.baseline, "baseline")?;
        let mut tally = ErrorTally::default();
        let mut open: Option<ComparisonPlace> = None;
        for row in 0..rows {
            let baseline_value = alignment.baseline.value(row, &baseline_values)?;
            let candidate_value = alignment.candidate.value(row, &candidate_values)?;
            if tally.add(baseline_value, candidate_value, tolerance) {
                let x = alignment.candidate.x(row, &candidate_axis);
                let error = candidate_value - baseline_value;
                let place = open.get_or_insert(ComparisonPlace {
                    signal: index,
                    first: x,
                    last: x,
                    worst_x: x,
                    worst_error: error,
                });
                place.last = x;
                if error.abs() > place.worst_error.abs() {
                    place.worst_x = x;
                    place.worst_error = error;
                }
            } else if let Some(place) = open.take() {
                places.push(place);
            }
        }
        places.extend(open);
        tallies.push(tally.into_signal(format!("signal:{index}")));
        if !progress(index + 1, compared.len()) {
            return Err("The comparison was stopped.".to_owned());
        }
    }
    let disposition = if tallies.iter().any(|signal| signal.failed_rows > 0) {
        ComparisonDisposition::Failed
    } else {
        ComparisonDisposition::Passed
    };
    let receipt = ComparisonReceipt {
        baseline: baseline.binding,
        candidate: candidate.binding,
        policy: ComparisonPolicy {
            row_alignment: RowAlignmentPolicy::RequireIdentical,
            tolerance,
            require_identical_units: true,
            execution: alignment.execution,
        },
        rows_compared: rows,
        signals: tallies,
        disposition,
    };
    receipt
        .validate_structure()
        .map_err(|error| error.to_string())?;
    Ok(ComparisonOutcome {
        receipt,
        signals: signals.to_vec(),
        units: compared
            .iter()
            .map(|signal| signal.candidate.unit.clone())
            .collect(),
        coordinate_unit: candidate
            .analysis
            .waveform_coordinate_unit()
            .map(str::to_owned),
        places,
    })
}

#[cfg(test)]
mod tests;
