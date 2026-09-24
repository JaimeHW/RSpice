//! Evaluating one correlation metric against aligned observations.
//!
//! Alignment happens before comparison and is explicit about what it did — a
//! point that had to be interpolated or unit-converted is recorded as such,
//! and a point outside the metric's declared domain is excluded rather than
//! extrapolated to. Residuals are therefore always attributable to a specific
//! observation and the transform that placed it.

use super::*;

pub(super) fn evaluate_metric(
    metric: &CorrelationMetricDefinition,
    reference: &CorrelationDatasetRevision,
    simulation: &CorrelationDatasetRevision,
    dispositions: &BTreeMap<(String, String), &CorrelationOutlierDisposition>,
) -> CorrelationResult<CorrelationMetricOutcome> {
    let mut eligible = Vec::new();
    for observation in reference
        .observations
        .iter()
        .filter(|observation| observation.quantity.eq_ignore_ascii_case(&metric.quantity))
    {
        if observation_inside_domain(observation, metric.domain.as_ref())? {
            eligible.push(observation);
        }
    }
    eligible.sort_by_key(|observation| normalized(&observation.id));
    if eligible.is_empty() {
        return Err(CorrelationError::new(
            CorrelationErrorCode::MetricInvalid,
            format!("metric.{}.quantity", metric.id),
            "metric selects no reference observations inside its declared domain",
        ));
    }
    let candidates = simulation
        .observations
        .iter()
        .filter(|observation| observation.quantity.eq_ignore_ascii_case(&metric.quantity))
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        return Err(CorrelationError::new(
            CorrelationErrorCode::MetricInvalid,
            format!("metric.{}.simulation_dataset_id", metric.id),
            "simulation dataset has no observations for the metric quantity",
        ));
    }
    let alignment_index = AlignmentIndex::try_new(&candidates, &metric.alignment)?;

    let mut residuals = Vec::new();
    let mut excluded_points = 0;
    for observation in &eligible {
        let disposition = dispositions.get(&(normalized(&metric.id), normalized(&observation.id)));
        let excluded = disposition
            .is_some_and(|disposition| disposition.decision != CorrelationOutlierDecision::Retain);
        let aligned = align_observation(observation, &alignment_index)?;
        let (metric_error, metric_uncertainty) =
            metric_error(metric.calculation, observation, &aligned)?;
        let effective_limit =
            metric.limit.get() + metric.uncertainty_multiplier.get() * metric_uncertainty;
        if !effective_limit.is_finite() || effective_limit <= 0.0 {
            return Err(CorrelationError::new(
                CorrelationErrorCode::MetricInvalid,
                format!("metric.{}.limit", metric.id),
                "effective metric limit must remain finite and greater than zero",
            ));
        }
        let normalized_error = metric_error / effective_limit;
        if !normalized_error.is_finite() || normalized_error < 0.0 {
            return Err(CorrelationError::new(
                CorrelationErrorCode::InvalidNumber,
                format!("metric.{}.normalized_error", metric.id),
                "metric produced a non-finite normalized residual",
            ));
        }
        residuals.push(CorrelationResidualPoint {
            id: format!("{}:{}", metric.id, observation.id),
            metric_id: metric.id.clone(),
            reference_observation_id: observation.id.clone(),
            reference_value: observation.value,
            simulated_value: finite("residual.simulated_value", aligned.value)?,
            simulation_observation_ids: aligned.observation_ids.clone(),
            alignment_evidence: aligned.evidence,
            metric_error: non_negative("residual.metric_error", metric_error)?,
            effective_limit: non_negative("residual.effective_limit", effective_limit)?,
            normalized_error: non_negative("residual.normalized_error", normalized_error)?,
            weight: non_negative("residual.weight", observation.weight.get() * aligned.weight)?,
            condition_group: condition_group_key(
                observation,
                match &metric.alignment {
                    CorrelationAlignmentPolicy::ExactOnly => None,
                    CorrelationAlignmentPolicy::MonotoneInterpolation { axis, .. } => {
                        Some(axis.as_str())
                    }
                },
            )?,
            excluded,
            exclusion_disposition_id: disposition
                .filter(|disposition| disposition.decision != CorrelationOutlierDecision::Retain)
                .map(|disposition| disposition.id.clone()),
        });
        excluded_points += usize::from(excluded);
    }
    if residuals.iter().all(|residual| residual.excluded) {
        return Err(CorrelationError::new(
            CorrelationErrorCode::MetricInvalid,
            format!("metric.{}.residuals", metric.id),
            "every eligible observation is excluded; no numerical correlation result exists",
        ));
    }
    residuals.sort_by_key(|residual| normalized(&residual.reference_observation_id));
    let (aggregate_error, aggregate_normalized_error) =
        aggregate_residuals(&residuals, metric.calculation, metric.aggregation);
    let covered_points = residuals.len().saturating_sub(excluded_points);
    let coverage = covered_points as f64 / eligible.len() as f64;
    let passed = aggregate_normalized_error <= 1.0 && coverage >= metric.minimum_coverage.get();
    Ok(CorrelationMetricOutcome {
        metric_id: metric.id.clone(),
        release_role: metric.release_role,
        evaluated_points: covered_points,
        excluded_points,
        coverage: non_negative("metric_outcome.coverage", coverage)?,
        minimum_coverage: metric.minimum_coverage,
        aggregate_error: non_negative("metric_outcome.aggregate_error", aggregate_error)?,
        aggregate_normalized_error: non_negative(
            "metric_outcome.aggregate_normalized_error",
            aggregate_normalized_error,
        )?,
        passed,
        residuals,
    })
}

pub(super) fn observation_inside_domain(
    observation: &CorrelationObservation,
    domain: Option<&CorrelationMetricDomain>,
) -> CorrelationResult<bool> {
    let Some(domain) = domain else {
        return Ok(true);
    };
    let coordinate = observation
        .coordinates
        .iter()
        .find(|coordinate| coordinate.dimension.eq_ignore_ascii_case(&domain.axis))
        .ok_or_else(|| {
            CorrelationError::new(
                CorrelationErrorCode::MetricInvalid,
                format!("observation.{}.coordinates", observation.id),
                format!(
                    "metric domain axis '{}' is missing from a selected reference observation",
                    domain.axis
                ),
            )
        })?;
    let value = convert_value(coordinate.value.get(), &coordinate.unit, &domain.unit)?;
    Ok(value >= domain.minimum.get() && value <= domain.maximum.get())
}

#[derive(Debug, Clone)]
pub(super) struct AlignedObservation {
    pub(super) value: f64,
    pub(super) uncertainty: f64,
    pub(super) weight: f64,
    pub(super) observation_ids: Vec<String>,
    pub(super) evidence: CorrelationAlignmentEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct CoordinateKeyPart {
    dimension: String,
    physical_dimension: UnitDimension,
    canonical_value_bits: u64,
}

pub(super) type CoordinateKey = Vec<CoordinateKeyPart>;

#[derive(Debug, Clone, Copy)]
pub(super) struct IndexedAxisPoint<'a> {
    axis_value: f64,
    observation: &'a CorrelationObservation,
}

#[derive(Debug)]
pub(super) enum AlignmentIndex<'a> {
    Exact {
        dimensions: BTreeMap<String, UnitDimension>,
        groups: BTreeMap<CoordinateKey, Vec<&'a CorrelationObservation>>,
    },
    Monotone {
        axis: String,
        axis_dimension: UnitDimension,
        dimensions: BTreeMap<String, UnitDimension>,
        groups: BTreeMap<CoordinateKey, Vec<IndexedAxisPoint<'a>>>,
        extrapolation: CorrelationExtrapolationPolicy,
    },
}

impl<'a> AlignmentIndex<'a> {
    fn try_new(
        candidates: &[&'a CorrelationObservation],
        policy: &CorrelationAlignmentPolicy,
    ) -> CorrelationResult<Self> {
        let first = candidates.first().copied().ok_or_else(|| {
            CorrelationError::new(
                CorrelationErrorCode::AlignmentInvalid,
                "alignment.candidates",
                "alignment requires at least one simulation observation",
            )
        })?;
        match policy {
            CorrelationAlignmentPolicy::ExactOnly => {
                let dimensions = coordinate_dimensions(first, None)?;
                let mut groups = BTreeMap::<CoordinateKey, Vec<&'a CorrelationObservation>>::new();
                for candidate in candidates {
                    let key = coordinate_key(candidate, None, &dimensions)?;
                    groups.entry(key).or_default().push(candidate);
                }
                Ok(Self::Exact { dimensions, groups })
            }
            CorrelationAlignmentPolicy::MonotoneInterpolation {
                axis,
                extrapolation,
            } => {
                let dimensions = coordinate_dimensions(first, Some(axis))?;
                let first_axis = coordinate(first, axis)?;
                let (axis_dimension, _) = canonical_coordinate(first, first_axis)?;
                let mut groups = BTreeMap::<CoordinateKey, Vec<IndexedAxisPoint<'a>>>::new();
                for candidate in candidates {
                    let key = coordinate_key(candidate, Some(axis), &dimensions)?;
                    let candidate_axis = coordinate(candidate, axis)?;
                    let (candidate_dimension, axis_value) =
                        canonical_coordinate(candidate, candidate_axis)?;
                    if candidate_dimension != axis_dimension {
                        return Err(CorrelationError::new(
                            CorrelationErrorCode::UnitMismatch,
                            format!("observation.{}.coordinates", candidate.id),
                            format!(
                                "alignment axis '{axis}' changes physical dimension across simulation observations"
                            ),
                        ));
                    }
                    groups.entry(key).or_default().push(IndexedAxisPoint {
                        axis_value,
                        observation: candidate,
                    });
                }
                for points in groups.values_mut() {
                    points.sort_by(|left, right| left.axis_value.total_cmp(&right.axis_value));
                    for pair in points.windows(2) {
                        if pair[0].axis_value >= pair[1].axis_value {
                            return Err(CorrelationError::new(
                                CorrelationErrorCode::AlignmentInvalid,
                                "alignment.candidates",
                                "candidate interpolation axes must be strictly increasing and unique within each condition group",
                            ));
                        }
                    }
                }
                Ok(Self::Monotone {
                    axis: axis.clone(),
                    axis_dimension,
                    dimensions,
                    groups,
                    extrapolation: *extrapolation,
                })
            }
        }
    }
}

pub(super) fn align_observation(
    reference: &CorrelationObservation,
    index: &AlignmentIndex<'_>,
) -> CorrelationResult<AlignedObservation> {
    match index {
        AlignmentIndex::Exact { dimensions, groups } => {
            let key = coordinate_key(reference, None, dimensions)?;
            let matches = groups.get(&key).map_or(&[][..], Vec::as_slice);
            if matches.len() != 1 {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::AlignmentInvalid,
                    format!("observation.{}", reference.id),
                    format!(
                        "exact alignment requires one candidate point; found {}",
                        matches.len()
                    ),
                ));
            }
            converted_candidate(reference, matches[0])
        }
        AlignmentIndex::Monotone {
            axis,
            axis_dimension,
            dimensions,
            groups,
            extrapolation,
        } => {
            let key = coordinate_key(reference, Some(axis), dimensions)?;
            let points = groups.get(&key).ok_or_else(|| {
                CorrelationError::new(
                    CorrelationErrorCode::AlignmentInvalid,
                    format!("observation.{}", reference.id),
                    "no simulation observations share the reference condition key",
                )
            })?;
            if points.len() < 2 {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::AlignmentInvalid,
                    format!("observation.{}", reference.id),
                    "monotone interpolation requires at least two compatible candidate points",
                ));
            }
            let reference_axis = coordinate(reference, axis)?;
            let (reference_dimension, axis_value) =
                canonical_coordinate(reference, reference_axis)?;
            if reference_dimension != *axis_dimension {
                return Err(CorrelationError::new(
                    CorrelationErrorCode::UnitMismatch,
                    format!("observation.{}.coordinates", reference.id),
                    format!(
                        "reference alignment axis '{axis}' has a different physical dimension from the simulation axis"
                    ),
                ));
            }
            match points.binary_search_by(|point| point.axis_value.total_cmp(&axis_value)) {
                Ok(index) => converted_candidate(reference, points[index].observation),
                Err(index) if index > 0 && index < points.len() => interpolate(
                    (
                        points[index - 1].axis_value,
                        converted_candidate(reference, points[index - 1].observation)?,
                    ),
                    (
                        points[index].axis_value,
                        converted_candidate(reference, points[index].observation)?,
                    ),
                    axis_value,
                    CorrelationAlignmentEvidence::Interpolated,
                ),
                Err(index) => {
                    let first = points.first().expect("two indexed points");
                    let last = points.last().expect("two indexed points");
                    let span = last.axis_value - first.axis_value;
                    let distance = if index == 0 {
                        first.axis_value - axis_value
                    } else {
                        axis_value - last.axis_value
                    };
                    match extrapolation {
                        CorrelationExtrapolationPolicy::Forbid => Err(CorrelationError::new(
                            CorrelationErrorCode::ExtrapolationForbidden,
                            format!("observation.{}", reference.id),
                            "reference coordinate lies outside the candidate domain and extrapolation is forbidden",
                        )),
                        CorrelationExtrapolationPolicy::Limited {
                            max_axis_span_fraction,
                        } if span > 0.0
                            && distance <= span * max_axis_span_fraction.get()
                            && max_axis_span_fraction.get() > 0.0 =>
                        {
                            let (left, right) = if index == 0 {
                                (&points[0], &points[1])
                            } else {
                                let length = points.len();
                                (&points[length - 2], &points[length - 1])
                            };
                            interpolate(
                                (
                                    left.axis_value,
                                    converted_candidate(reference, left.observation)?,
                                ),
                                (
                                    right.axis_value,
                                    converted_candidate(reference, right.observation)?,
                                ),
                                axis_value,
                                CorrelationAlignmentEvidence::Extrapolated,
                            )
                        }
                        CorrelationExtrapolationPolicy::Limited { .. } => {
                            Err(CorrelationError::new(
                                CorrelationErrorCode::ExtrapolationForbidden,
                                format!("observation.{}", reference.id),
                                "reference coordinate exceeds the declared limited-extrapolation envelope",
                            ))
                        }
                    }
                }
            }
        }
    }
}

pub(super) fn converted_candidate(
    reference: &CorrelationObservation,
    candidate: &CorrelationObservation,
) -> CorrelationResult<AlignedObservation> {
    Ok(AlignedObservation {
        value: convert_value(candidate.value.get(), &candidate.unit, &reference.unit)?,
        uncertainty: convert_delta(
            candidate.uncertainty.get(),
            &candidate.unit,
            &reference.unit,
        )?,
        weight: candidate.weight.get(),
        observation_ids: vec![candidate.id.clone()],
        evidence: CorrelationAlignmentEvidence::Exact,
    })
}

pub(super) fn interpolate(
    left: (f64, AlignedObservation),
    right: (f64, AlignedObservation),
    x: f64,
    evidence: CorrelationAlignmentEvidence,
) -> CorrelationResult<AlignedObservation> {
    let fraction = (x - left.0) / (right.0 - left.0);
    let lerp = |a: f64, b: f64| a + fraction * (b - a);
    let value = lerp(left.1.value, right.1.value);
    let (uncertainty, weight) = if evidence == CorrelationAlignmentEvidence::Extrapolated {
        // Extrapolation uses the absolute linear coefficients as a
        // worst-direction uncertainty bound. It can never cancel or shrink
        // below both endpoint uncertainties as an ordinary linear blend can.
        let left_coefficient = 1.0 - fraction;
        let right_coefficient = fraction;
        (
            left_coefficient.abs() * left.1.uncertainty
                + right_coefficient.abs() * right.1.uncertainty,
            left.1.weight.min(right.1.weight),
        )
    } else {
        (
            lerp(left.1.uncertainty, right.1.uncertainty).abs(),
            lerp(left.1.weight, right.1.weight),
        )
    };
    if !value.is_finite() || !uncertainty.is_finite() || !weight.is_finite() || weight <= 0.0 {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            "alignment",
            "interpolation produced a non-finite value, uncertainty, or weight",
        ));
    }
    Ok(AlignedObservation {
        value,
        uncertainty,
        weight,
        observation_ids: vec![
            left.1.observation_ids[0].clone(),
            right.1.observation_ids[0].clone(),
        ],
        evidence,
    })
}

pub(super) fn coordinate<'a>(
    observation: &'a CorrelationObservation,
    dimension: &str,
) -> CorrelationResult<&'a CorrelationCoordinate> {
    observation
        .coordinates
        .iter()
        .find(|coordinate| coordinate.dimension.eq_ignore_ascii_case(dimension))
        .ok_or_else(|| {
            CorrelationError::new(
                CorrelationErrorCode::AlignmentInvalid,
                format!("observation.{}.coordinates", observation.id),
                format!("required alignment axis '{dimension}' is missing"),
            )
        })
}

pub(super) fn coordinate_dimensions(
    observation: &CorrelationObservation,
    ignored_axis: Option<&str>,
) -> CorrelationResult<BTreeMap<String, UnitDimension>> {
    let mut dimensions = BTreeMap::new();
    for coordinate in observation.coordinates.iter().filter(|coordinate| {
        ignored_axis.is_none_or(|axis| !coordinate.dimension.eq_ignore_ascii_case(axis))
    }) {
        let name = normalized(&coordinate.dimension);
        let physical_dimension = unit_spec(&coordinate.unit)?.dimension;
        if dimensions
            .insert(name.clone(), physical_dimension)
            .is_some()
        {
            return Err(CorrelationError::new(
                CorrelationErrorCode::AlignmentInvalid,
                format!("observation.{}.coordinates", observation.id),
                format!("condition dimension '{}' is repeated", coordinate.dimension),
            ));
        }
    }
    Ok(dimensions)
}

pub(super) fn coordinate_key(
    observation: &CorrelationObservation,
    ignored_axis: Option<&str>,
    expected_dimensions: &BTreeMap<String, UnitDimension>,
) -> CorrelationResult<CoordinateKey> {
    let mut parts = Vec::with_capacity(expected_dimensions.len());
    for coordinate in observation.coordinates.iter().filter(|coordinate| {
        ignored_axis.is_none_or(|axis| !coordinate.dimension.eq_ignore_ascii_case(axis))
    }) {
        let dimension = normalized(&coordinate.dimension);
        let expected = expected_dimensions.get(&dimension).ok_or_else(|| {
            CorrelationError::new(
                CorrelationErrorCode::AlignmentInvalid,
                format!("observation.{}.coordinates", observation.id),
                format!(
                    "condition dimension '{}' is not part of the indexed alignment key",
                    coordinate.dimension
                ),
            )
        })?;
        let (physical_dimension, canonical_value) = canonical_coordinate(observation, coordinate)?;
        if physical_dimension != *expected {
            return Err(CorrelationError::new(
                CorrelationErrorCode::UnitMismatch,
                format!("observation.{}.coordinates", observation.id),
                format!(
                    "condition dimension '{}' changes physical unit dimension",
                    coordinate.dimension
                ),
            ));
        }
        parts.push(CoordinateKeyPart {
            dimension,
            physical_dimension,
            canonical_value_bits: canonical_value.to_bits(),
        });
    }
    parts.sort();
    if parts.len() != expected_dimensions.len() {
        let present = parts
            .iter()
            .map(|part| part.dimension.as_str())
            .collect::<BTreeSet<_>>();
        let missing = expected_dimensions
            .keys()
            .filter(|dimension| !present.contains(dimension.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        return Err(CorrelationError::new(
            CorrelationErrorCode::AlignmentInvalid,
            format!("observation.{}.coordinates", observation.id),
            format!(
                "alignment key is missing required condition dimension(s): {}",
                missing.join(", ")
            ),
        ));
    }
    Ok(parts)
}

pub(super) fn canonical_coordinate(
    observation: &CorrelationObservation,
    coordinate: &CorrelationCoordinate,
) -> CorrelationResult<(UnitDimension, f64)> {
    let unit = unit_spec(&coordinate.unit)?;
    let value = coordinate.value.get() * unit.scale + unit.bias;
    if !value.is_finite() {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            format!("observation.{}.coordinates", observation.id),
            format!(
                "condition dimension '{}' cannot be converted to a finite canonical value",
                coordinate.dimension
            ),
        ));
    }
    Ok((unit.dimension, if value == 0.0 { 0.0 } else { value }))
}

pub(super) fn condition_group_key(
    observation: &CorrelationObservation,
    ignored_axis: Option<&str>,
) -> CorrelationResult<String> {
    let dimensions = coordinate_dimensions(observation, ignored_axis)?;
    let key = coordinate_key(observation, ignored_axis, &dimensions)?;
    if key.is_empty() {
        Ok("all-conditions".to_owned())
    } else {
        let mut hasher = Sha256::new();
        for part in key {
            let length = u64::try_from(part.dimension.len()).map_err(|_| {
                CorrelationError::new(
                    CorrelationErrorCode::ResourceLimit,
                    "condition_group",
                    "condition dimension identity exceeds the supported range",
                )
            })?;
            hasher.update(length.to_le_bytes());
            hasher.update(part.dimension.as_bytes());
            hasher.update([part.physical_dimension.stable_tag()]);
            hasher.update(part.canonical_value_bits.to_le_bytes());
        }
        Ok(format!(
            "conditions:{}",
            ContentDigest::from_bytes(hasher.finalize().into())
        ))
    }
}

pub(super) fn metric_error(
    calculation: CorrelationCalculation,
    reference: &CorrelationObservation,
    simulated: &AlignedObservation,
) -> CorrelationResult<(f64, f64)> {
    let reference_value = reference.value.get();
    let reference_uncertainty = reference.uncertainty.get();
    let rss = |left: f64, right: f64| left.hypot(right);
    match calculation {
        CorrelationCalculation::AbsoluteLinear => Ok((
            (simulated.value - reference_value).abs(),
            rss(reference_uncertainty, simulated.uncertainty),
        )),
        CorrelationCalculation::AbsoluteDecibels => {
            let unit = unit_spec(&reference.unit)?;
            if unit.dimension == UnitDimension::Decibel {
                Ok((
                    (simulated.value - reference_value).abs(),
                    rss(reference_uncertainty, simulated.uncertainty),
                ))
            } else {
                if reference_value == 0.0 || simulated.value == 0.0 {
                    return Err(CorrelationError::new(
                        CorrelationErrorCode::MetricInvalid,
                        format!("observation.{}", reference.id),
                        "decibel error from linear values requires non-zero magnitudes",
                    ));
                }
                let factor = if unit.dimension == UnitDimension::Power {
                    10.0
                } else {
                    20.0
                };
                let error =
                    (factor * (simulated.value.abs() / reference_value.abs()).log10()).abs();
                let uncertainty = factor / std::f64::consts::LN_10
                    * rss(
                        reference_uncertainty / reference_value.abs(),
                        simulated.uncertainty / simulated.value.abs(),
                    );
                Ok((error, uncertainty))
            }
        }
        CorrelationCalculation::Relative => {
            let scale = reference_value.abs().max(f64::MIN_POSITIVE);
            Ok((
                (simulated.value - reference_value).abs() / scale,
                rss(reference_uncertainty, simulated.uncertainty) / scale,
            ))
        }
        CorrelationCalculation::WeightedRelative => {
            let denominator = reference_value
                .abs()
                .max(simulated.value.abs())
                .max(f64::MIN_POSITIVE);
            Ok((
                (simulated.value - reference_value).abs() / denominator,
                rss(reference_uncertainty, simulated.uncertainty) / denominator,
            ))
        }
        CorrelationCalculation::PhaseWrappedDegrees => {
            let reference_degrees = convert_value(reference_value, &reference.unit, "deg")?;
            let simulated_degrees = convert_value(simulated.value, &reference.unit, "deg")?;
            let mut delta = (simulated_degrees - reference_degrees) % 360.0;
            if delta > 180.0 {
                delta -= 360.0;
            } else if delta < -180.0 {
                delta += 360.0;
            }
            let reference_uncertainty =
                convert_delta(reference_uncertainty, &reference.unit, "deg")?;
            let simulated_uncertainty =
                convert_delta(simulated.uncertainty, &reference.unit, "deg")?;
            Ok((
                delta.abs(),
                rss(reference_uncertainty, simulated_uncertainty),
            ))
        }
    }
}

pub(super) fn aggregate_residuals(
    residuals: &[CorrelationResidualPoint],
    calculation: CorrelationCalculation,
    aggregation: CorrelationAggregation,
) -> (f64, f64) {
    let included = residuals
        .iter()
        .filter(|residual| !residual.excluded)
        .collect::<Vec<_>>();
    match aggregation {
        CorrelationAggregation::EveryPoint => included
            .iter()
            .map(|residual| (residual.metric_error.get(), residual.normalized_error.get()))
            .max_by(|left, right| left.1.total_cmp(&right.1))
            .expect("non-empty residuals"),
        CorrelationAggregation::WorstCondition => {
            let mut groups = BTreeMap::<&str, Vec<&CorrelationResidualPoint>>::new();
            for residual in &included {
                groups
                    .entry(residual.condition_group.as_str())
                    .or_default()
                    .push(residual);
            }
            groups
                .values()
                .map(|group| rms_residuals(group, false))
                .max_by(|left, right| left.1.total_cmp(&right.1))
                .expect("non-empty residual groups")
        }
        CorrelationAggregation::Percentile95 => {
            let mut ranked = included
                .iter()
                .map(|residual| (residual.metric_error.get(), residual.normalized_error.get()))
                .collect::<Vec<_>>();
            ranked.sort_by(|left, right| left.1.total_cmp(&right.1));
            let rank = ((ranked.len() as f64 * 0.95).ceil() as usize).clamp(1, ranked.len());
            ranked[rank - 1]
        }
        CorrelationAggregation::RootMeanSquare => rms_residuals(
            &included,
            calculation == CorrelationCalculation::WeightedRelative,
        ),
    }
}

pub(super) fn rms_residuals(residuals: &[&CorrelationResidualPoint], weighted: bool) -> (f64, f64) {
    let denominator = if weighted {
        residuals
            .iter()
            .map(|residual| residual.weight.get())
            .sum::<f64>()
    } else {
        residuals.len() as f64
    };
    let error_energy = residuals
        .iter()
        .map(|residual| {
            residual.metric_error.get().powi(2) * if weighted { residual.weight.get() } else { 1.0 }
        })
        .sum::<f64>();
    let normalized_energy = residuals
        .iter()
        .map(|residual| {
            residual.normalized_error.get().powi(2)
                * if weighted { residual.weight.get() } else { 1.0 }
        })
        .sum::<f64>();
    (
        (error_energy / denominator).sqrt(),
        (normalized_energy / denominator).sqrt(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum UnitDimension {
    Dimensionless,
    Voltage,
    Current,
    Resistance,
    Power,
    Frequency,
    Time,
    Temperature,
    Decibel,
    Angle,
    VoltageNoiseDensity,
    CurrentNoiseDensity,
}

impl UnitDimension {
    const fn stable_tag(self) -> u8 {
        match self {
            Self::Dimensionless => 0,
            Self::Voltage => 1,
            Self::Current => 2,
            Self::Resistance => 3,
            Self::Power => 4,
            Self::Frequency => 5,
            Self::Time => 6,
            Self::Temperature => 7,
            Self::Decibel => 8,
            Self::Angle => 9,
            Self::VoltageNoiseDensity => 10,
            Self::CurrentNoiseDensity => 11,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct UnitSpec {
    pub(super) dimension: UnitDimension,
    pub(super) scale: f64,
    pub(super) bias: f64,
}

pub(super) fn unit_spec(unit: &str) -> CorrelationResult<UnitSpec> {
    let normalized = normalized_unit(unit);
    let spec = match normalized.as_str() {
        "1" | "dimensionless" => UnitSpec::linear(UnitDimension::Dimensionless, 1.0),
        "v" | "volt" | "volts" => UnitSpec::linear(UnitDimension::Voltage, 1.0),
        "mv" => UnitSpec::linear(UnitDimension::Voltage, 1.0e-3),
        "uv" => UnitSpec::linear(UnitDimension::Voltage, 1.0e-6),
        "nv" => UnitSpec::linear(UnitDimension::Voltage, 1.0e-9),
        "v/sqrthz" | "v/rthz" => UnitSpec::linear(UnitDimension::VoltageNoiseDensity, 1.0),
        "mv/sqrthz" | "mv/rthz" => UnitSpec::linear(UnitDimension::VoltageNoiseDensity, 1.0e-3),
        "uv/sqrthz" | "uv/rthz" => UnitSpec::linear(UnitDimension::VoltageNoiseDensity, 1.0e-6),
        "nv/sqrthz" | "nv/rthz" => UnitSpec::linear(UnitDimension::VoltageNoiseDensity, 1.0e-9),
        "a" | "amp" | "amps" | "ampere" | "amperes" => {
            UnitSpec::linear(UnitDimension::Current, 1.0)
        }
        "ma" => UnitSpec::linear(UnitDimension::Current, 1.0e-3),
        "ua" => UnitSpec::linear(UnitDimension::Current, 1.0e-6),
        "na" => UnitSpec::linear(UnitDimension::Current, 1.0e-9),
        "pa" => UnitSpec::linear(UnitDimension::Current, 1.0e-12),
        "a/sqrthz" | "a/rthz" => UnitSpec::linear(UnitDimension::CurrentNoiseDensity, 1.0),
        "ma/sqrthz" | "ma/rthz" => UnitSpec::linear(UnitDimension::CurrentNoiseDensity, 1.0e-3),
        "ua/sqrthz" | "ua/rthz" => UnitSpec::linear(UnitDimension::CurrentNoiseDensity, 1.0e-6),
        "na/sqrthz" | "na/rthz" => UnitSpec::linear(UnitDimension::CurrentNoiseDensity, 1.0e-9),
        "pa/sqrthz" | "pa/rthz" => UnitSpec::linear(UnitDimension::CurrentNoiseDensity, 1.0e-12),
        "ohm" | "ohms" => UnitSpec::linear(UnitDimension::Resistance, 1.0),
        "kohm" | "kohms" => UnitSpec::linear(UnitDimension::Resistance, 1.0e3),
        "mohm" | "mohms" => UnitSpec::linear(UnitDimension::Resistance, 1.0e6),
        "w" | "watt" | "watts" => UnitSpec::linear(UnitDimension::Power, 1.0),
        "mw" => UnitSpec::linear(UnitDimension::Power, 1.0e-3),
        "hz" => UnitSpec::linear(UnitDimension::Frequency, 1.0),
        "khz" => UnitSpec::linear(UnitDimension::Frequency, 1.0e3),
        "mhz" => UnitSpec::linear(UnitDimension::Frequency, 1.0e6),
        "ghz" => UnitSpec::linear(UnitDimension::Frequency, 1.0e9),
        "s" | "sec" | "second" | "seconds" => UnitSpec::linear(UnitDimension::Time, 1.0),
        "ms" => UnitSpec::linear(UnitDimension::Time, 1.0e-3),
        "us" => UnitSpec::linear(UnitDimension::Time, 1.0e-6),
        "ns" => UnitSpec::linear(UnitDimension::Time, 1.0e-9),
        "ps" => UnitSpec::linear(UnitDimension::Time, 1.0e-12),
        "k" | "kelvin" => UnitSpec::linear(UnitDimension::Temperature, 1.0),
        "degc" | "celsius" => UnitSpec {
            dimension: UnitDimension::Temperature,
            scale: 1.0,
            bias: 273.15,
        },
        "db" => UnitSpec::linear(UnitDimension::Decibel, 1.0),
        "deg" | "degree" | "degrees" => UnitSpec::linear(UnitDimension::Angle, 1.0),
        "rad" | "radian" | "radians" => {
            UnitSpec::linear(UnitDimension::Angle, 180.0 / std::f64::consts::PI)
        }
        _ => {
            return Err(CorrelationError::new(
                CorrelationErrorCode::UnitMismatch,
                "unit",
                format!("unsupported correlation unit '{unit}'"),
            ));
        }
    };
    Ok(spec)
}

impl UnitSpec {
    const fn linear(dimension: UnitDimension, scale: f64) -> Self {
        Self {
            dimension,
            scale,
            bias: 0.0,
        }
    }
}

pub(super) fn convert_value(value: f64, from: &str, to: &str) -> CorrelationResult<f64> {
    let from = unit_spec(from)?;
    let to = unit_spec(to)?;
    if from.dimension != to.dimension {
        return Err(CorrelationError::new(
            CorrelationErrorCode::UnitMismatch,
            "unit",
            "correlation units describe different physical dimensions",
        ));
    }
    let converted = (value * from.scale + from.bias - to.bias) / to.scale;
    if !converted.is_finite() {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            "unit",
            "unit conversion produced a non-finite value",
        ));
    }
    Ok(if converted == 0.0 { 0.0 } else { converted })
}

pub(super) fn convert_delta(value: f64, from: &str, to: &str) -> CorrelationResult<f64> {
    let from = unit_spec(from)?;
    let to = unit_spec(to)?;
    if from.dimension != to.dimension {
        return Err(CorrelationError::new(
            CorrelationErrorCode::UnitMismatch,
            "unit",
            "correlation units describe different physical dimensions",
        ));
    }
    let converted = value * from.scale / to.scale;
    if !converted.is_finite() || converted < 0.0 {
        return Err(CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            "unit",
            "uncertainty conversion produced an invalid value",
        ));
    }
    Ok(converted)
}

pub(super) fn normalized_unit(unit: &str) -> String {
    unit.trim()
        .to_lowercase()
        .replace([' ', '_'], "")
        .replace('√', "sqrt")
        .replace(['µ', 'μ'], "u")
        .replace(['ω', 'Ω'], "ohm")
        .replace('°', "deg")
}

pub(super) fn find_ci<'a>(
    datasets: &[&'a CorrelationDatasetRevision],
    id: &str,
) -> Option<&'a CorrelationDatasetRevision> {
    datasets
        .iter()
        .copied()
        .find(|dataset| dataset.id.eq_ignore_ascii_case(id))
}

pub(super) fn require_schema(path: &str, value: u32) -> CorrelationResult<()> {
    if value == MODEL_CORRELATION_SCHEMA_VERSION {
        Ok(())
    } else {
        Err(CorrelationError::new(
            CorrelationErrorCode::UnsupportedSchema,
            path,
            format!(
                "expected correlation schema {MODEL_CORRELATION_SCHEMA_VERSION}, received {value}"
            ),
        ))
    }
}

pub(super) fn require_text(path: &str, value: &str) -> CorrelationResult<()> {
    if value.trim().is_empty() {
        Err(CorrelationError::new(
            CorrelationErrorCode::MissingValue,
            path,
            "value is required",
        ))
    } else {
        Ok(())
    }
}

pub(super) fn bounded_text(path: &str, value: &str) -> CorrelationResult<()> {
    if value.len() <= MAX_CORRELATION_TEXT_BYTES {
        Ok(())
    } else {
        Err(CorrelationError::new(
            CorrelationErrorCode::ResourceLimit,
            path,
            format!("text is limited to {MAX_CORRELATION_TEXT_BYTES} UTF-8 bytes"),
        ))
    }
}

pub(super) fn require_count(
    path: &str,
    count: usize,
    maximum: usize,
    description: &str,
) -> CorrelationResult<()> {
    if count <= maximum {
        Ok(())
    } else {
        Err(CorrelationError::new(
            CorrelationErrorCode::ResourceLimit,
            path,
            format!("{description} are limited to {maximum}; received {count}"),
        ))
    }
}

pub(super) fn checked_count_add(
    path: &str,
    left: usize,
    right: usize,
    description: &str,
) -> CorrelationResult<usize> {
    left.checked_add(right).ok_or_else(|| {
        CorrelationError::new(
            CorrelationErrorCode::ResourceLimit,
            path,
            format!("{description} exceed the supported count range"),
        )
    })
}

pub(super) fn finite(path: &str, value: f64) -> CorrelationResult<FiniteValue> {
    FiniteValue::new(value).map_err(|_| {
        CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            path,
            "value must be finite",
        )
    })
}

pub(super) fn non_negative(path: &str, value: f64) -> CorrelationResult<NonNegativeFinite> {
    NonNegativeFinite::new(value).map_err(|_| {
        CorrelationError::new(
            CorrelationErrorCode::InvalidNumber,
            path,
            "value must be finite and non-negative",
        )
    })
}

pub(super) fn digest(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

pub(super) fn normalized(value: &str) -> String {
    value.trim().to_lowercase()
}
