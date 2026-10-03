//! Exact comparison alignment and difference derivation over borrowed retained results.

use crate::analysis_result::AnalysisResult;
use crate::analysis_type::AnalysisType;
use crate::result_digest::ResultDigestEncoding;
use crate::run::SimulationRun;
use crate::studio_presentation::{ComparisonAlignmentDraft, MAX_DIFFERENCE_TRACE_NUMERIC_VALUES};
use crate::visualization_document::{
    ColumnRole, ComparisonAlignmentMethod, ComparisonExecutionContract,
    ComparisonExtrapolationPolicy, ComparisonInterpolationPolicy, ComparisonPolicy,
    ComparisonPrecisionPolicy, ComparisonReceipt, ComparisonRequest, ComparisonResamplingPolicy,
    NumericTolerance, RowAlignmentPolicy, SourceColumn, SourceDataset, SourceRow, TypedValue,
    ValueType, compare_source_datasets,
};
use crate::waveform::RetainedWaveform;
use rspice_app_types::product::DatasetBinding;

/// The shared coordinate axis, then the baseline and candidate series
/// resampled onto it.
type AlignedComparisonSeries = (Vec<f64>, Vec<Vec<f64>>, Vec<Vec<f64>>);

struct PreparedComparisonSources {
    baseline: SourceDataset,
    candidate: SourceDataset,
    signal_names: Vec<String>,
    coordinates: Vec<f64>,
    baseline_values: Vec<Vec<f64>>,
    candidate_values: Vec<Vec<f64>>,
    coordinate_unit: Option<String>,
    execution: ComparisonExecutionContract,
}

#[derive(Debug, Clone)]
pub struct DifferenceTrace {
    pub baseline: DatasetBinding,
    pub candidate: DatasetBinding,
    pub signal_key: String,
    pub signal_label: String,
    pub coordinate_unit: Option<String>,
    pub coordinates: Vec<f64>,
    pub absolute: Vec<f64>,
    pub relative: Vec<f64>,
    pub normalized: Vec<f64>,
    pub execution: ComparisonExecutionContract,
    pub tolerance: NumericTolerance,
}

#[derive(Debug)]
pub struct ComparisonExecution {
    pub receipt: ComparisonReceipt,
    pub difference_traces: Vec<DifferenceTrace>,
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

fn validate_strict_axis(name: &str, axis: &[f64]) -> Result<(), String> {
    if axis.is_empty() {
        return Err(format!("Waveform '{name}' has no coordinate samples."));
    }
    if axis.iter().any(|value| !value.is_finite()) {
        return Err(format!(
            "Waveform '{name}' contains a non-finite coordinate and cannot be compared."
        ));
    }
    if axis.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(format!(
            "Waveform '{name}' has a nonmonotonic coordinate axis and cannot be compared."
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
) -> Result<&'a RetainedWaveform, String> {
    let mut matching = analysis
        .waveforms
        .iter()
        .map(AsRef::as_ref)
        .filter(|waveform| waveform.name == signal_name);
    let waveform = matching
        .next()
        .ok_or_else(|| format!("Waveform '{signal_name}' is unavailable."))?;
    if matching.next().is_some() {
        return Err(format!(
            "Waveform name '{signal_name}' is ambiguous in the retained analysis."
        ));
    }
    Ok(waveform)
}

fn validated_analysis_axis<'a, W: AsRef<RetainedWaveform>>(
    analysis: &'a AnalysisResult<W>,
    signal_names: &[String],
) -> Result<&'a [f64], String> {
    let first_name = signal_names
        .first()
        .ok_or_else(|| "The selected analyses have no common waveform quantities.".to_owned())?;
    let reference = unique_waveform(analysis, first_name)?;
    if reference.y.len() != reference.x.len() {
        return Err(format!(
            "Waveform '{}' has mismatched coordinate and sample counts.",
            reference.name
        ));
    }
    validate_strict_axis(&reference.name, &reference.x)?;
    if reference.y.iter().any(|value| !value.is_finite()) {
        return Err(format!(
            "Waveform '{}' contains a non-finite sample and cannot be compared.",
            reference.name
        ));
    }
    for signal_name in signal_names.iter().skip(1) {
        let waveform = unique_waveform(analysis, signal_name)?;
        if waveform.y.len() != waveform.x.len()
            || !exact_axis(&waveform.x, &reference.x)
            || waveform.y.iter().any(|value| !value.is_finite())
        {
            return Err(format!(
                "Waveform '{signal_name}' does not share the analysis comparison axis or contains unsupported samples."
            ));
        }
    }
    Ok(&reference.x)
}

pub fn comparison_signal_names<W: AsRef<RetainedWaveform>>(
    candidate_analysis: &AnalysisResult<W>,
    baseline_analysis: &AnalysisResult<W>,
) -> Result<Vec<String>, String> {
    let candidate_reference = candidate_analysis
        .waveforms
        .first()
        .map(AsRef::as_ref)
        .ok_or_else(|| "The candidate analysis has no waveform quantities.".to_owned())?;
    if candidate_reference.y.len() != candidate_reference.x.len() {
        return Err(format!(
            "Waveform '{}' has mismatched coordinate and sample counts.",
            candidate_reference.name
        ));
    }
    validate_strict_axis(&candidate_reference.name, &candidate_reference.x)?;
    let candidate_axis_family = candidate_analysis
        .waveforms
        .iter()
        .map(AsRef::as_ref)
        .filter(|waveform| exact_axis(&waveform.x, &candidate_reference.x))
        .collect::<Vec<_>>();
    let mut baseline_reference = None;
    for waveform in &candidate_axis_family {
        let matching = baseline_analysis
            .waveforms
            .iter()
            .map(AsRef::as_ref)
            .filter(|baseline| baseline.name == waveform.name)
            .collect::<Vec<_>>();
        if matching.len() > 1 {
            return Err(format!(
                "Waveform name '{}' is ambiguous in the retained baseline analysis.",
                waveform.name
            ));
        }
        if let Some(baseline) = matching.first().copied()
            && baseline.y.len() == baseline.x.len()
        {
            baseline_reference = Some(baseline);
            break;
        }
    }
    let baseline_reference = baseline_reference.ok_or_else(|| {
        "The selected analyses have no common waveform quantities on the active coordinate axis."
            .to_owned()
    })?;
    validate_strict_axis(&baseline_reference.name, &baseline_reference.x)?;
    let mut signal_names = Vec::new();
    for candidate in candidate_axis_family {
        let Ok(baseline) = unique_waveform(baseline_analysis, &candidate.name) else {
            continue;
        };
        if !exact_axis(&baseline.x, &baseline_reference.x) {
            continue;
        }
        if candidate.y.len() != candidate.x.len()
            || baseline.y.len() != baseline.x.len()
            || candidate.y.iter().any(|value| !value.is_finite())
            || baseline.y.iter().any(|value| !value.is_finite())
        {
            return Err(format!(
                "Waveform '{}' contains unsupported or non-finite samples.",
                candidate.name
            ));
        }
        signal_names.push(candidate.name.clone());
    }
    signal_names.sort();
    if signal_names.is_empty() {
        return Err("The selected analyses have no common waveform quantities.".to_owned());
    }
    validated_analysis_axis(candidate_analysis, &signal_names)?;
    validated_analysis_axis(baseline_analysis, &signal_names)?;
    Ok(signal_names)
}

fn comparison_axis_unit(analysis_type: AnalysisType) -> &'static str {
    match analysis_type {
        AnalysisType::Ac | AnalysisType::Noise | AnalysisType::Pnoise => "Hz",
        AnalysisType::Transient | AnalysisType::Soa => "s",
        AnalysisType::DcSweep => "V",
        _ => "",
    }
}

fn comparison_source_dataset<A, W>(
    run: &SimulationRun<A>,
    signal_names: &[String],
    coordinates: &[f64],
    values: &[Vec<f64>],
    coordinate_unit: Option<&str>,
) -> Result<SourceDataset, String>
where
    A: AsRef<AnalysisResult<W>>,
    W: AsRef<RetainedWaveform>,
{
    if values.len() != signal_names.len()
        || values
            .iter()
            .any(|signal_values| signal_values.len() != coordinates.len())
    {
        return Err("Aligned comparison values do not match their declared shape.".to_owned());
    }
    let mut columns = vec![
        SourceColumn::new(
            "x",
            "X coordinate",
            ValueType::Real,
            ColumnRole::Coordinate,
            coordinate_unit.map(str::to_owned),
        )
        .map_err(|error| error.to_string())?,
    ];
    for (index, signal_name) in signal_names.iter().enumerate() {
        columns.push(
            SourceColumn::new(
                format!("signal:{index}"),
                signal_name,
                ValueType::Real,
                ColumnRole::Signal,
                // Preserve the comparison projection's unknown signal units.
                None,
            )
            .map_err(|error| error.to_string())?,
        );
    }
    let rows = coordinates
        .iter()
        .enumerate()
        .map(|(row, x)| {
            let mut row_values = vec![TypedValue::Real(*x)];
            row_values.extend(
                values
                    .iter()
                    .map(|signal_values| TypedValue::Real(signal_values[row])),
            );
            SourceRow::new(row_values)
        })
        .collect();
    SourceDataset::new(
        DatasetBinding::new(
            run.dataset_id,
            run.dataset_content_digest_with_encoding(ResultDigestEncoding::CURRENT),
        ),
        columns,
        rows,
    )
    .map_err(|error| error.to_string())
}

fn waveform_matrix<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
    signal_names: &[String],
) -> Result<Vec<Vec<f64>>, String> {
    signal_names
        .iter()
        .map(|signal_name| {
            unique_waveform(analysis, signal_name)
                .map(|waveform| waveform.y.iter().copied().collect())
        })
        .collect()
}

fn interpolate_monotone(axis: &[f64], values: &[f64], target: f64) -> Result<f64, String> {
    if axis.len() != values.len()
        || axis.is_empty()
        || !target.is_finite()
        || target < axis[0]
        || target > axis[axis.len() - 1]
    {
        return Err(
            "Interpolation target lies outside the retained monotone source axis.".to_owned(),
        );
    }
    match axis.binary_search_by(|value| value.total_cmp(&target)) {
        Ok(index) => Ok(values[index]),
        Err(upper) if upper > 0 && upper < axis.len() => {
            let lower = upper - 1;
            let fraction = (target - axis[lower]) / (axis[upper] - axis[lower]);
            let value = values[lower] + fraction * (values[upper] - values[lower]);
            value.is_finite().then_some(value).ok_or_else(|| {
                "Monotone linear interpolation produced a non-finite value.".to_owned()
            })
        }
        _ => Err("Interpolation target is not bracketed by retained samples.".to_owned()),
    }
}

fn interpolate_matrix(
    source_axis: &[f64],
    source_values: &[Vec<f64>],
    targets: &[f64],
) -> Result<Vec<Vec<f64>>, String> {
    source_values
        .iter()
        .map(|values| {
            targets
                .iter()
                .map(|target| interpolate_monotone(source_axis, values, *target))
                .collect()
        })
        .collect()
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
            let fraction = (threshold - left) / (right - left);
            let crossing = axis[index] + fraction * (axis[index + 1] - axis[index]);
            if crossing.is_finite() {
                return Ok(crossing);
            }
        }
    }
    if values[values.len() - 1].to_bits() == threshold.to_bits() {
        return Ok(axis[axis.len() - 1]);
    }
    Err("The alignment signal has no finite threshold crossing in the retained data.".to_owned())
}

fn exact_intersection(
    baseline_axis: &[f64],
    candidate_axis: &[f64],
    baseline_values: &[Vec<f64>],
    candidate_values: &[Vec<f64>],
) -> Result<AlignedComparisonSeries, String> {
    let mut coordinates = Vec::new();
    let mut baseline_aligned = vec![Vec::new(); baseline_values.len()];
    let mut candidate_aligned = vec![Vec::new(); candidate_values.len()];
    let (mut baseline_index, mut candidate_index) = (0_usize, 0_usize);
    while baseline_index < baseline_axis.len() && candidate_index < candidate_axis.len() {
        let baseline_x = baseline_axis[baseline_index];
        let candidate_x = candidate_axis[candidate_index];
        if baseline_x.to_bits() == candidate_x.to_bits() {
            coordinates.push(candidate_x);
            for signal in 0..baseline_values.len() {
                baseline_aligned[signal].push(baseline_values[signal][baseline_index]);
                candidate_aligned[signal].push(candidate_values[signal][candidate_index]);
            }
            baseline_index += 1;
            candidate_index += 1;
        } else if baseline_x < candidate_x {
            baseline_index += 1;
        } else {
            candidate_index += 1;
        }
    }
    if coordinates.is_empty() {
        return Err(
            "Absolute X-axis comparison found no exact coordinate intersection.".to_owned(),
        );
    }
    Ok((coordinates, baseline_aligned, candidate_aligned))
}

fn uniform_grid(start: f64, end: f64, count: usize) -> Result<(Vec<f64>, f64), String> {
    if !start.is_finite() || !end.is_finite() || start >= end || count < 3 {
        return Err("A uniform comparison grid requires a finite non-empty overlap.".to_owned());
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
            "Cross-correlation is undefined for a constant or non-finite alignment signal."
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
        return Err(
            "Cross-correlation requires equal finite grids and a positive maximum lag.".to_owned(),
        );
    }
    let maximum_lag = maximum_lag_samples as usize;
    if maximum_lag > baseline.len().saturating_sub(3) {
        return Err(format!(
            "Maximum lag {maximum_lag_samples} leaves fewer than three overlapping samples; reduce it."
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

fn prepared_comparison_sources<A, W>(
    candidate_run: &SimulationRun<A>,
    candidate_analysis: &AnalysisResult<W>,
    baseline_run: &SimulationRun<A>,
    alignment: ComparisonAlignmentDraft,
    alignment_signal: &str,
    threshold: f64,
    maximum_lag_samples: u32,
) -> Result<PreparedComparisonSources, String>
where
    A: AsRef<AnalysisResult<W>>,
    W: AsRef<RetainedWaveform>,
{
    let baseline_analysis = matching_comparison_analysis(candidate_analysis, baseline_run)
        .ok_or_else(|| "The comparison dataset has no unambiguous matching analysis.".to_owned())?;
    let baseline_analysis = baseline_analysis.as_ref();
    let signal_names = comparison_signal_names(candidate_analysis, baseline_analysis)?;
    let candidate_axis = validated_analysis_axis(candidate_analysis, &signal_names)?;
    let baseline_axis = validated_analysis_axis(baseline_analysis, &signal_names)?;
    let candidate_source_values = waveform_matrix(candidate_analysis, &signal_names)?;
    let baseline_source_values = waveform_matrix(baseline_analysis, &signal_names)?;
    let signal_index = signal_names
        .iter()
        .position(|signal| signal == alignment_signal);
    let (coordinates, baseline_values, candidate_values, execution) = match alignment {
        ComparisonAlignmentDraft::AbsoluteXAxis => {
            let (coordinates, baseline_values, candidate_values) = exact_intersection(
                baseline_axis,
                candidate_axis,
                &baseline_source_values,
                &candidate_source_values,
            )?;
            (
                coordinates,
                baseline_values,
                candidate_values,
                ComparisonExecutionContract {
                    alignment: ComparisonAlignmentMethod::AbsoluteXAxis,
                    interpolation: ComparisonInterpolationPolicy::NoneExactOnly,
                    resampling: ComparisonResamplingPolicy::ExactCoordinateIntersection,
                    extrapolation: ComparisonExtrapolationPolicy::Forbid,
                    precision: ComparisonPrecisionPolicy::SourceF64NoRounding,
                },
            )
        }
        ComparisonAlignmentDraft::FirstThresholdCrossing => {
            let signal_index = signal_index.ok_or_else(|| {
                "Select a common waveform quantity for threshold alignment.".to_owned()
            })?;
            let baseline_crossing = first_threshold_crossing(
                baseline_axis,
                &baseline_source_values[signal_index],
                threshold,
            )?;
            let candidate_crossing = first_threshold_crossing(
                candidate_axis,
                &candidate_source_values[signal_index],
                threshold,
            )?;
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
            let candidate_indices = candidate_shifted
                .iter()
                .enumerate()
                .filter_map(|(index, coordinate)| {
                    (*coordinate >= overlap_start && *coordinate <= overlap_end).then_some(index)
                })
                .collect::<Vec<_>>();
            if candidate_indices.len() < 2 {
                return Err(
                    "Threshold-aligned datasets have fewer than two candidate samples in their common support."
                        .to_owned(),
                );
            }
            let coordinates = candidate_indices
                .iter()
                .map(|index| candidate_shifted[*index])
                .collect::<Vec<_>>();
            let baseline_values =
                interpolate_matrix(&baseline_shifted, &baseline_source_values, &coordinates)?;
            let candidate_values = candidate_source_values
                .iter()
                .map(|values| {
                    candidate_indices
                        .iter()
                        .map(|index| values[*index])
                        .collect()
                })
                .collect();
            (
                coordinates,
                baseline_values,
                candidate_values,
                ComparisonExecutionContract {
                    alignment: ComparisonAlignmentMethod::FirstThresholdCrossing {
                        signal_key: format!("signal:{signal_index}"),
                        threshold,
                        baseline_crossing,
                        candidate_crossing,
                    },
                    interpolation: ComparisonInterpolationPolicy::MonotoneLinear,
                    resampling: ComparisonResamplingPolicy::BaselineOntoCandidateGrid,
                    extrapolation: ComparisonExtrapolationPolicy::Forbid,
                    precision: ComparisonPrecisionPolicy::SourceF64NoRounding,
                },
            )
        }
        ComparisonAlignmentDraft::CrossCorrelation => {
            let signal_index = signal_index.ok_or_else(|| {
                "Select a common waveform quantity for cross-correlation alignment.".to_owned()
            })?;
            let overlap_start = baseline_axis[0].max(candidate_axis[0]);
            let overlap_end = baseline_axis[baseline_axis.len() - 1]
                .min(candidate_axis[candidate_axis.len() - 1]);
            let sample_count = baseline_axis.len().max(candidate_axis.len());
            let (correlation_grid, sample_interval) =
                uniform_grid(overlap_start, overlap_end, sample_count)?;
            let baseline_alignment = interpolate_matrix(
                baseline_axis,
                &[baseline_source_values[signal_index].clone()],
                &correlation_grid,
            )?
            .remove(0);
            let candidate_alignment = interpolate_matrix(
                candidate_axis,
                &[candidate_source_values[signal_index].clone()],
                &correlation_grid,
            )?
            .remove(0);
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
                return Err(
                    "Cross-correlation shift leaves no common coordinate support.".to_owned(),
                );
            }
            let final_count =
                (((aligned_end - aligned_start) / sample_interval).floor() as usize) + 1;
            if final_count < 3 {
                return Err(
                    "Cross-correlation shift leaves fewer than three samples in common support."
                        .to_owned(),
                );
            }
            let coordinates = (0..final_count)
                .map(|index| aligned_start + sample_interval * index as f64)
                .collect::<Vec<_>>();
            let baseline_targets = coordinates
                .iter()
                .map(|coordinate| coordinate - baseline_shift)
                .collect::<Vec<_>>();
            let baseline_values =
                interpolate_matrix(baseline_axis, &baseline_source_values, &baseline_targets)?;
            let candidate_values =
                interpolate_matrix(candidate_axis, &candidate_source_values, &coordinates)?;
            (
                coordinates,
                baseline_values,
                candidate_values,
                ComparisonExecutionContract {
                    alignment: ComparisonAlignmentMethod::CrossCorrelation {
                        signal_key: format!("signal:{signal_index}"),
                        maximum_lag_samples,
                        selected_lag_samples,
                        sample_interval,
                        coefficient,
                        baseline_shift,
                    },
                    interpolation: ComparisonInterpolationPolicy::MonotoneLinear,
                    resampling: ComparisonResamplingPolicy::UniformOverlapGrid,
                    extrapolation: ComparisonExtrapolationPolicy::Forbid,
                    precision: ComparisonPrecisionPolicy::SourceF64NoRounding,
                },
            )
        }
    };
    execution.validate().map_err(|error| error.to_string())?;
    let axis_unit = comparison_axis_unit(candidate_analysis.analysis_type);
    let coordinate_unit = (!axis_unit.is_empty()).then_some(axis_unit.to_owned());
    let baseline = comparison_source_dataset(
        baseline_run,
        &signal_names,
        &coordinates,
        &baseline_values,
        coordinate_unit.as_deref(),
    )?;
    let candidate = comparison_source_dataset(
        candidate_run,
        &signal_names,
        &coordinates,
        &candidate_values,
        coordinate_unit.as_deref(),
    )?;
    Ok(PreparedComparisonSources {
        baseline,
        candidate,
        signal_names,
        coordinates,
        baseline_values,
        candidate_values,
        coordinate_unit,
        execution,
    })
}

fn draft_difference_traces(
    prepared: &PreparedComparisonSources,
    tolerance: NumericTolerance,
) -> Result<Vec<DifferenceTrace>, String> {
    let retained_values = prepared
        .coordinates
        .len()
        .checked_mul(prepared.signal_names.len())
        .and_then(|values| values.checked_mul(4))
        .ok_or_else(|| "Difference-trace retained-value count overflowed.".to_owned())?;
    if retained_values > MAX_DIFFERENCE_TRACE_NUMERIC_VALUES {
        return Err(format!(
            "Difference traces require {retained_values} retained numeric values, exceeding the {MAX_DIFFERENCE_TRACE_NUMERIC_VALUES}-value document bound."
        ));
    }
    prepared
        .signal_names
        .iter()
        .enumerate()
        .map(|(signal_index, signal_label)| {
            let mut absolute = Vec::with_capacity(prepared.coordinates.len());
            let mut relative = Vec::with_capacity(prepared.coordinates.len());
            let mut normalized = Vec::with_capacity(prepared.coordinates.len());
            for (baseline, candidate) in prepared.baseline_values[signal_index]
                .iter()
                .zip(&prepared.candidate_values[signal_index])
            {
                let difference = (candidate - baseline).abs();
                let scale = baseline.abs().max(candidate.abs());
                let relative_difference = if scale == 0.0 {
                    0.0
                } else {
                    difference / scale
                };
                let allowed = tolerance.absolute + tolerance.relative * baseline.abs();
                let normalized_difference = if allowed == 0.0 {
                    if difference == 0.0 {
                        0.0
                    } else {
                        return Err(format!(
                            "Signal '{signal_label}' has a non-zero difference that cannot be normalized under zero tolerance."
                        ));
                    }
                } else {
                    difference / allowed
                };
                if !difference.is_finite()
                    || !relative_difference.is_finite()
                    || !normalized_difference.is_finite()
                {
                    return Err(format!(
                        "Difference derivation for signal '{signal_label}' produced a non-finite value."
                    ));
                }
                absolute.push(difference);
                relative.push(relative_difference);
                normalized.push(normalized_difference);
            }
            Ok(DifferenceTrace {
                baseline: prepared.baseline.binding(),
                candidate: prepared.candidate.binding(),
                signal_key: format!("signal:{signal_index}"),
                signal_label: signal_label.clone(),
                coordinate_unit: prepared.coordinate_unit.clone(),
                coordinates: prepared.coordinates.clone(),
                absolute,
                relative,
                normalized,
                execution: prepared.execution.clone(),
                tolerance,
            })
        })
        .collect()
}

/// Explicit policies for a single immutable comparison; no application selection state.
pub struct ComparisonOptions<'a> {
    pub alignment: ComparisonAlignmentDraft,
    pub alignment_signal: &'a str,
    pub threshold: f64,
    pub maximum_lag_samples: u32,
    pub absolute_tolerance: f64,
    pub relative_tolerance: f64,
    pub difference_trace: bool,
}

/// Align and compare exact retained samples without mutating either source.
pub fn execute_comparison<A, W>(
    active_run: &SimulationRun<A>,
    active_analysis: &AnalysisResult<W>,
    baseline_run: &SimulationRun<A>,
    options: ComparisonOptions<'_>,
) -> Result<ComparisonExecution, String>
where
    A: AsRef<AnalysisResult<W>>,
    W: AsRef<RetainedWaveform>,
{
    let prepared = prepared_comparison_sources(
        active_run,
        active_analysis,
        baseline_run,
        options.alignment,
        options.alignment_signal,
        options.threshold,
        options.maximum_lag_samples,
    )?;
    let signal_keys = (0..prepared.signal_names.len())
        .map(|index| format!("signal:{index}"))
        .collect();
    let tolerance = NumericTolerance::new(options.absolute_tolerance, options.relative_tolerance)
        .map_err(|error| error.to_string())?;
    let request = ComparisonRequest {
        baseline: prepared.baseline.binding(),
        candidate: prepared.candidate.binding(),
        signal_keys,
        policy: ComparisonPolicy {
            row_alignment: RowAlignmentPolicy::RequireIdentical,
            tolerance,
            require_identical_units: true,
            execution: prepared.execution.clone(),
        },
    };
    let receipt = compare_source_datasets(&prepared.baseline, &prepared.candidate, &request)
        .map_err(|error| error.to_string())?;
    let difference_traces = if options.difference_trace {
        draft_difference_traces(&prepared, tolerance)?
    } else {
        Vec::new()
    };
    Ok(ComparisonExecution {
        receipt,
        difference_traces,
    })
}

#[cfg(test)]
mod tests;
