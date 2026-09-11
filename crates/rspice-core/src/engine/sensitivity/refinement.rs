//! Shared finite-difference refinement for real and complex circuit probes.

use super::{
    Engine, SimulationError, sensitivity_ratio, sensitivity_three_point, sensitivity_trial,
};
use crate::abort_signal::AbortSignal;
use crate::{Complex64, Value};
use rspice_veriloga_runtime::arithmetic::ScaledValue;

const RELATIVE_AGREEMENT: Value = 1e-4;
const MAX_REFINEMENTS: usize = 12;
const ARITHMETIC_ROUNDOFF: Value = 64.0 * Value::EPSILON;

struct Sample {
    coordinate: Value,
    values: Vec<Complex64>,
}

#[derive(Default)]
enum Trial {
    Sample(Sample),
    OutsideDomain,
    #[default]
    Unresolved,
}

impl Trial {
    fn as_ref(&self) -> Option<&Sample> {
        match self {
            Self::Sample(sample) => Some(sample),
            Self::OutsideDomain | Self::Unresolved => None,
        }
    }

    fn outside_domain(&self) -> bool {
        matches!(self, Self::OutsideDomain)
    }
}

#[derive(Default)]
struct Pair {
    positive: Trial,
    negative: Trial,
}

#[derive(Clone, Copy)]
struct Estimate {
    value: Value,
    // The leading interpolation error is proportional to this signed product.
    // Retaining actual offsets also handles rounded, nonuniform coordinates.
    error_factor: ScaledValue,
}

fn component_value(value: Complex64, index: usize) -> Value {
    if index == 0 { value.re } else { value.im }
}

fn difference(left: Value, right: Value) -> ScaledValue {
    ScaledValue::new(left).plus(ScaledValue::new(-right))
}

fn estimate(
    coordinate: Value,
    nominal: Value,
    first: Option<&Sample>,
    second: Option<&Sample>,
    index: usize,
    component: usize,
) -> Result<Option<Estimate>, SimulationError> {
    let (Some(first), Some(second)) = (first, second) else {
        return Ok(None);
    };
    if first.coordinate == second.coordinate {
        return Ok(None);
    }
    Ok(Some(Estimate {
        value: sensitivity_three_point(
            [coordinate, first.coordinate, second.coordinate],
            [
                nominal,
                component_value(first.values[index], component),
                component_value(second.values[index], component),
            ],
        )?,
        error_factor: difference(first.coordinate, coordinate)
            .multiply(difference(second.coordinate, coordinate)),
    }))
}

fn extrapolate(coarse: Estimate, fine: Estimate) -> Result<Value, SimulationError> {
    let one = ScaledValue::new(1.0);
    sensitivity_ratio(
        [
            [ScaledValue::new(fine.value), coarse.error_factor, one],
            [ScaledValue::new(-coarse.value), fine.error_factor, one],
        ]
        .into_iter(),
        [
            [coarse.error_factor, one, one],
            [fine.error_factor.negated(), one, one],
        ]
        .into_iter(),
    )
}

fn agrees(left: Value, right: Value, roundoff: ScaledValue) -> bool {
    let tolerance = ScaledValue::new(left.abs().max(right.abs()))
        .multiply(ScaledValue::new(RELATIVE_AGREEMENT))
        .plus(roundoff);
    let error = difference(left, right);
    error.is_zero() || (!tolerance.is_zero() && error.divide(tolerance).binary64().abs() <= 1.0)
}

/// Require observable probe changes to exceed the arithmetic noise relative to
/// the requested agreement. Keep evidence of variation: a larger step landing
/// on an equal value must not turn an unresolved response into a false zero.
fn response_resolved(
    pair: &Pair,
    nominal: &[Complex64],
    observed: &mut [[bool; 2]],
    reserve: Value,
    abort: &dyn AbortSignal,
) -> Result<bool, SimulationError> {
    let mut resolved = true;
    for (index, nominal) in nominal.iter().enumerate() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        for (component, has_varied) in observed[index].iter_mut().enumerate() {
            let nominal = component_value(*nominal, component);
            let values = [pair.positive.as_ref(), pair.negative.as_ref()]
                .into_iter()
                .flatten()
                .map(|sample| component_value(sample.values[index], component));
            let scale = values
                .clone()
                .map(Value::abs)
                .fold(nominal.abs(), Value::max);
            let mut response: Value = 0.0;
            for value in values {
                let change = difference(value, nominal);
                if !change.is_zero() {
                    *has_varied = true;
                    response =
                        response.max(change.divide(ScaledValue::new(scale)).binary64().abs());
                }
            }
            resolved &=
                !*has_varied || response >= reserve * ARITHMETIC_ROUNDOFF / RELATIVE_AGREEMENT;
        }
    }
    Ok(resolved)
}

/// Check expanded stencils against the original nearby response. Interpolate
/// changes from the nominal value so a large baseline does not cancel the
/// evidence, and retain exponents in both the weights and the noise bound.
fn preserves_nearby_response(
    coordinate: Value,
    nominal: Value,
    anchor: &Pair,
    samples: &[Option<&Sample>],
    index: usize,
    component: usize,
) -> bool {
    for nearby in [anchor.positive.as_ref(), anchor.negative.as_ref()]
        .into_iter()
        .flatten()
    {
        let actual = component_value(nearby.values[index], component);
        let mut prediction = ScaledValue::new(0.0);
        let mut noise_weight = ScaledValue::new(1.0);
        let mut scale = nominal.abs().max(actual.abs());
        for (i, sample) in samples.iter().enumerate() {
            let Some(sample) = sample else { continue };
            if samples[..i]
                .iter()
                .flatten()
                .any(|earlier| earlier.coordinate == sample.coordinate)
            {
                continue;
            }
            let mut weight = difference(nearby.coordinate, coordinate)
                .divide(difference(sample.coordinate, coordinate));
            for (j, other) in samples.iter().enumerate() {
                let Some(other) = other else { continue };
                if other.coordinate == sample.coordinate
                    || samples[..j]
                        .iter()
                        .flatten()
                        .any(|earlier| earlier.coordinate == other.coordinate)
                {
                    continue;
                }
                weight = weight.multiply(
                    difference(nearby.coordinate, other.coordinate)
                        .divide(difference(sample.coordinate, other.coordinate)),
                );
            }
            let value = component_value(sample.values[index], component);
            scale = scale.max(value.abs());
            prediction = prediction.plus(weight.multiply(difference(value, nominal)));
            // copysign retains the sign even if binary64 conversion underflows.
            noise_weight = noise_weight
                .plus(weight.multiply(ScaledValue::new(1.0_f64.copysign(weight.binary64()))));
        }
        let error = prediction.plus(difference(nominal, actual));
        let tolerance = ScaledValue::new(scale)
            .multiply(ScaledValue::new(ARITHMETIC_ROUNDOFF))
            .multiply(noise_weight);
        let relative_error = error.divide(tolerance).binary64().abs();
        if !error.is_zero() && (!relative_error.is_finite() || relative_error > 1.0) {
            return false;
        }
    }
    true
}

impl Engine {
    /// Resolve observed probe changes before comparing successively halved
    /// central and one-sided stencils. An explicit step is an initial guess.
    /// This is a numerical consistency check, not a proof of model regularity.
    /// The nominal run is counted by the caller; all attempted trials share its
    /// counter, including failed physical-domain/convergence evaluations.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn refine_sensitivity(
        &self,
        name: &str,
        coordinate: Value,
        mut step: Value,
        nominal: &[Complex64],
        runs: &mut usize,
        abort: &dyn AbortSignal,
        mut evaluate: impl FnMut(Value) -> Result<Vec<Complex64>, SimulationError>,
    ) -> Result<Vec<Complex64>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !coordinate.is_finite()
            || !step.is_finite()
            || step <= 0.0
            || nominal
                .iter()
                .any(|value| !value.re.is_finite() || !value.im.is_finite())
        {
            return Err(SimulationError::Circuit(format!(
                "Sensitivity parameter '{name}' has an invalid nominal sample or step"
            )));
        }
        let mut last_failure = None;
        let mut sample_pair = |step: Value| -> Result<Pair, SimulationError> {
            let mut sample = |candidate: Value| -> Result<Trial, SimulationError> {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                if !candidate.is_finite() || candidate == coordinate {
                    return Ok(Trial::OutsideDomain);
                }
                *runs = runs.saturating_add(1);
                self.ensure_batch_runs(*runs)?;
                match sensitivity_trial(evaluate(candidate))? {
                    Ok(values) => {
                        if values.len() != nominal.len()
                            || values
                                .iter()
                                .any(|value| !value.re.is_finite() || !value.im.is_finite())
                        {
                            return Err(SimulationError::Circuit(format!(
                                "Sensitivity parameter '{name}' returned an invalid probe trace at {candidate}"
                            )));
                        }
                        Ok(Trial::Sample(Sample {
                            coordinate: candidate,
                            values,
                        }))
                    }
                    Err(error) => {
                        let domain = matches!(error, SimulationError::ParameterDomain(_));
                        last_failure = Some(format!("at {candidate}: {error}"));
                        Ok(if domain {
                            Trial::OutsideDomain
                        } else {
                            Trial::Unresolved
                        })
                    }
                }
            };
            Ok(Pair {
                positive: sample(coordinate + step)?,
                negative: sample(coordinate - step)?,
            })
        };

        let mut outer = sample_pair(step)?;
        let mut anchor = None;
        let mut observed = vec![[false; 2]; nominal.len()];
        // A tiny step at a zero-valued parameter can lose the response beneath
        // a nonzero probe baseline. Enlarge before fitting instead of allowing
        // the roundoff floor to excuse an inaccurate derivative. Reserve three
        // doublings to fit two resolved stencils, including a one-sided fit.
        for expansion in 0..MAX_REFINEMENTS - 3 {
            let reserve = if expansion == 0 { 16.0 } else { 1.0 };
            if response_resolved(&outer, nominal, &mut observed, reserve, abort)? {
                break;
            }
            let larger = step * 2.0;
            if !larger.is_finite()
                || (outer.positive.as_ref().is_none() && outer.negative.as_ref().is_none())
            {
                break;
            }
            step = larger;
            let previous = std::mem::replace(&mut outer, sample_pair(step)?);
            if anchor.is_none() {
                anchor = Some(previous);
            }
        }
        let mut older = Pair::default();
        let mut prepared = [None, None];
        if anchor.is_some()
            && (step * 8.0).is_finite()
            && response_resolved(&outer, nominal, &mut observed, 1.0, abort)?
        {
            // Measure the smallest response directly instead of assuming its
            // rate of decay. Reuse it and the next larger pair while refining;
            // smooth stationary responses can decay faster than quadratically.
            let first_inner = sample_pair(step * 2.0)?;
            let next_outer = sample_pair(step * 4.0)?;
            older = sample_pair(step * 8.0)?;
            prepared = [
                Some(first_inner),
                Some(std::mem::replace(&mut outer, next_outer)),
            ];
            step *= 4.0;
        }
        let mut prepared = prepared.into_iter().flatten();
        let mut previous_one_sided: Option<(bool, Vec<Complex64>)> = None;
        for _ in 0..MAX_REFINEMENTS {
            step *= 0.5;
            if step == 0.0
                || [
                    (coordinate + step, outer.positive.as_ref()),
                    (coordinate - step, outer.negative.as_ref()),
                ]
                .into_iter()
                .all(|(candidate, previous)| {
                    !candidate.is_finite()
                        || candidate == coordinate
                        || previous.is_some_and(|previous| previous.coordinate == candidate)
                })
            {
                break;
            }
            let inner = match prepared.next() {
                Some(pair) => pair,
                None => sample_pair(step)?,
            };
            let mut candidate = Vec::with_capacity(nominal.len());
            let mut qualified = response_resolved(&inner, nominal, &mut observed, 1.0, abort)?;
            let mut one_sided_direction = None;
            let mut all_extrapolated = true;
            for (index, nominal_value) in nominal.iter().enumerate() {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let mut projected = [0.0; 2];
                for (component, projected_value) in projected.iter_mut().enumerate() {
                    let nominal_value = component_value(*nominal_value, component);
                    let samples = [
                        older.positive.as_ref(),
                        older.negative.as_ref(),
                        outer.positive.as_ref(),
                        outer.negative.as_ref(),
                        inner.positive.as_ref(),
                        inner.negative.as_ref(),
                    ];
                    if let Some(anchor) = &anchor {
                        qualified &= preserves_nearby_response(
                            coordinate,
                            nominal_value,
                            anchor,
                            &samples,
                            index,
                            component,
                        );
                    }
                    let scale = samples
                        .iter()
                        .flatten()
                        .map(|sample| component_value(sample.values[index], component).abs())
                        .fold(nominal_value.abs(), Value::max);
                    // Use a conservative arithmetic floor in derivative units,
                    // without imposing an absolute voltage/current cutoff.
                    let spacing = samples
                        .iter()
                        .flatten()
                        .map(|sample| (sample.coordinate - coordinate).abs())
                        .fold(Value::INFINITY, Value::min);
                    let roundoff = ScaledValue::new(scale)
                        .multiply(ScaledValue::new(ARITHMETIC_ROUNDOFF))
                        .divide(ScaledValue::new(spacing));
                    let fit = |a: Option<&Sample>, b: Option<&Sample>| {
                        estimate(coordinate, nominal_value, a, b, index, component).map_err(
                            |error| {
                                SimulationError::Circuit(format!(
                                    "Sensitivity parameter '{name}': {error}"
                                ))
                            },
                        )
                    };
                    let extrapolate = |coarse, fine| {
                        extrapolate(coarse, fine).map_err(|error| {
                            SimulationError::Circuit(format!(
                                "Sensitivity parameter '{name}': {error}"
                            ))
                        })
                    };
                    let coarse = fit(outer.negative.as_ref(), outer.positive.as_ref())?;
                    let fine = fit(inner.negative.as_ref(), inner.positive.as_ref())?;
                    let left = fit(outer.negative.as_ref(), inner.negative.as_ref())?;
                    let right = fit(outer.positive.as_ref(), inner.positive.as_ref())?;
                    if let (Some(coarse), Some(fine), Some(left), Some(right)) =
                        (coarse, fine, left, right)
                    {
                        let value = extrapolate(coarse, fine)?;
                        let direct = agrees(coarse.value, fine.value, roundoff)
                            && agrees(left.value, right.value, roundoff)
                            && agrees(left.value, fine.value, roundoff)
                            && agrees(right.value, fine.value, roundoff);
                        let refined =
                            if let (Some(previous), Some(previous_left), Some(previous_right)) = (
                                fit(older.negative.as_ref(), older.positive.as_ref())?,
                                fit(older.negative.as_ref(), outer.negative.as_ref())?,
                                fit(older.positive.as_ref(), outer.positive.as_ref())?,
                            ) {
                                agrees(extrapolate(previous, coarse)?, value, roundoff)
                                    && agrees(extrapolate(previous_left, left)?, value, roundoff)
                                    && agrees(extrapolate(previous_right, right)?, value, roundoff)
                            } else {
                                false
                            };
                        qualified &= direct || refined;
                        *projected_value = value;
                    } else {
                        let direction = match (left, right) {
                            (Some(_), None)
                                if older.positive.outside_domain()
                                    && outer.positive.outside_domain()
                                    && inner.positive.outside_domain()
                                    && anchor
                                        .as_ref()
                                        .is_none_or(|pair| pair.positive.outside_domain()) =>
                            {
                                Some(false)
                            }
                            (None, Some(_))
                                if older.negative.outside_domain()
                                    && outer.negative.outside_domain()
                                    && inner.negative.outside_domain()
                                    && anchor
                                        .as_ref()
                                        .is_none_or(|pair| pair.negative.outside_domain()) =>
                            {
                                Some(true)
                            }
                            _ => None,
                        };
                        let Some(positive) = direction else {
                            qualified = false;
                            all_extrapolated = false;
                            continue;
                        };
                        one_sided_direction = Some(positive);
                        let (current, previous) = if positive {
                            (
                                right,
                                fit(older.positive.as_ref(), outer.positive.as_ref())?,
                            )
                        } else {
                            (left, fit(older.negative.as_ref(), outer.negative.as_ref())?)
                        };
                        let (Some(current), Some(previous)) = (current, previous) else {
                            qualified = false;
                            all_extrapolated = false;
                            continue;
                        };
                        let value = extrapolate(previous, current)?;
                        qualified &= agrees(previous.value, current.value, roundoff)
                            || previous_one_sided
                                .as_ref()
                                .is_some_and(|(direction, values)| {
                                    *direction == positive
                                        && agrees(
                                            component_value(values[index], component),
                                            value,
                                            roundoff,
                                        )
                                });
                        *projected_value = value;
                    }
                }
                candidate.push(Complex64::new(projected[0], projected[1]));
            }
            if qualified {
                return Ok(candidate);
            }
            // Store only real extrapolations, not the uncomputed placeholders
            // from the first one-sided stencil.
            previous_one_sided = one_sided_direction
                .filter(|_| all_extrapolated)
                .map(|positive| (positive, candidate));
            older = outer;
            outer = inner;
        }
        Err(SimulationError::Circuit(format!(
            "Sensitivity parameter '{name}' could not resolve a derivative: trial failures do not establish a parameter boundary, estimates disagree, or distinct finite coordinates are exhausted (possible discontinuity or insufficient numerical precision){}",
            last_failure
                .map(|failure| format!("; last trial failure {failure}"))
                .unwrap_or_default()
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::NoAbort;

    #[test]
    fn refinement_resolves_small_responses_without_a_probe_scale_floor() {
        for scale in [1e-200, 1.0, 1e200] {
            for one_sided in [false, true] {
                let mut largest: Value = 0.0;
                let value = Engine::default()
                    .refine_sensitivity(
                        "small response",
                        0.0,
                        1e-12,
                        &[Complex64::new(scale, 0.0)],
                        &mut 1,
                        &NoAbort,
                        |point| {
                            largest = largest.max(point.abs());
                            if one_sided && point < 0.0 {
                                Err(SimulationError::ParameterDomain(
                                    "nonnegative parameter".into(),
                                ))
                            } else {
                                Ok(vec![Complex64::new(scale * (1.0 + point), 0.0)])
                            }
                        },
                    )
                    .unwrap();
                assert!(
                    (value[0].re / scale - 1.0).abs() < 1e-5,
                    "scale={scale}, one_sided={one_sided}: {value:?}"
                );
                assert!(
                    largest > 1e-12,
                    "the weak response must trigger a larger step"
                );
            }
        }
    }

    #[test]
    fn refinement_expansion_cannot_replace_a_local_slope_with_a_distant_branch() {
        let result = Engine::default().refine_sensitivity(
            "nearby branch",
            0.0,
            1e-12,
            &[Complex64::new(1.0, 0.0)],
            &mut 1,
            &NoAbort,
            |point| {
                Ok(vec![Complex64::new(
                    1.0 + if point.abs() <= 1e-12 {
                        point
                    } else {
                        2.0 * point
                    },
                    0.0,
                )])
            },
        );
        assert!(
            result.is_err(),
            "the expanded stencil crossed the local branch: {result:?}"
        );
    }

    #[test]
    fn refinement_expansion_preserves_nearby_trial_failure_evidence() {
        let result = Engine::default().refine_sensitivity(
            "nearby failed trial",
            0.0,
            1e-12,
            &[Complex64::new(1.0, 0.0)],
            &mut 1,
            &NoAbort,
            |point| {
                if point < -1e-12 {
                    Err(SimulationError::ParameterDomain("outer domain".into()))
                } else if point < 0.0 {
                    Err(SimulationError::ConvergenceFailed(20))
                } else {
                    Ok(vec![Complex64::new(1.0 + point, 0.0)])
                }
            },
        );
        assert!(
            result.is_err(),
            "nearby failures remain unresolved: {result:?}"
        );
    }

    #[test]
    fn refinement_resolves_a_smooth_stationary_response_above_a_baseline() {
        for one_sided in [false, true] {
            let result = Engine::default().refine_sensitivity(
                "stationary response",
                0.0,
                1e-4,
                &[Complex64::new(1.0, 0.0)],
                &mut 1,
                &NoAbort,
                |point| {
                    if one_sided && point < 0.0 {
                        Err(SimulationError::ParameterDomain(
                            "nonnegative parameter".into(),
                        ))
                    } else {
                        Ok(vec![Complex64::new(1.0 + point.powi(3), 0.0)])
                    }
                },
            );
            assert!(result.is_ok(), "one_sided={one_sided}: {result:?}");
            assert!(result.unwrap()[0].re.abs() < 1e-9);
        }
    }

    #[test]
    fn refinement_does_not_erase_a_weak_response_when_larger_samples_are_equal() {
        let result = Engine::default().refine_sensitivity(
            "localized response",
            0.0,
            1e-12,
            &[Complex64::new(1.0, 0.0)],
            &mut 1,
            &NoAbort,
            |point| {
                Ok(vec![Complex64::new(
                    if point.abs() <= 1e-12 {
                        1.0 + point
                    } else {
                        1.0
                    },
                    0.0,
                )])
            },
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("could not resolve")
        );
    }

    #[test]
    fn refinement_expansion_observes_run_limits_and_fatal_failures() {
        let mut config = crate::engine::SimulationConfig::default();
        config.resource_limits.max_batch_runs = 5;
        let mut evaluations = 0;
        let error = Engine::new(config)
            .refine_sensitivity(
                "expansion budget",
                0.0,
                1e-12,
                &[Complex64::new(1.0, 0.0)],
                &mut 1,
                &NoAbort,
                |point| {
                    evaluations += 1;
                    Ok(vec![Complex64::new(1.0 + point, 0.0)])
                },
            )
            .unwrap_err();
        assert!(matches!(error, SimulationError::ResourceLimit(_)));
        assert_eq!(evaluations, 4);

        let mut evaluations = 0;
        let error = Engine::default()
            .refine_sensitivity(
                "expansion cancellation",
                0.0,
                1e-12,
                &[Complex64::new(1.0, 0.0)],
                &mut 1,
                &NoAbort,
                |point| {
                    evaluations += 1;
                    if evaluations == 3 {
                        Err(SimulationError::Aborted)
                    } else {
                        Ok(vec![Complex64::new(1.0 + point, 0.0)])
                    }
                },
            )
            .unwrap_err();
        assert!(matches!(error, SimulationError::Aborted));
        assert_eq!(evaluations, 3, "stop before the next expansion trial");
    }

    #[test]
    fn refinement_checks_multiple_scales_and_directional_consistency() {
        let engine = Engine::default();
        let mut samples = Vec::new();
        let values = engine
            .refine_sensitivity(
                "curvature",
                1.0,
                1e-3,
                &[Complex64::new(100.0_f64.exp(), 0.0)],
                &mut 1,
                &NoAbort,
                |point| {
                    samples.push(point);
                    Ok(vec![Complex64::new((100.0 * point).exp(), 0.0)])
                },
            )
            .unwrap();
        assert!((values[0].re / (100.0 * 100.0_f64.exp()) - 1.0).abs() < 1e-5);
        assert!(
            samples.len() > 4,
            "curvature requires more than the first pair of stencils"
        );
        assert!(samples.contains(&1.00025));

        let stationary = engine
            .refine_sensitivity(
                "cubic",
                0.0,
                1e-12,
                &[Complex64::new(0.0, 0.0)],
                &mut 1,
                &NoAbort,
                |point| Ok(vec![Complex64::new(point.powi(3), 0.0)]),
            )
            .unwrap();
        // Residual arithmetic noise is far below the h^2 truncation error.
        assert!(stationary[0].re.abs() < 1e-36, "{stationary:?}");
    }

    #[test]
    fn refinement_handles_one_sided_domains_without_accepting_an_infinite_slope() {
        let engine = Engine::default();
        for square_root in [false, true] {
            let result = engine.refine_sensitivity(
                "domain",
                0.0,
                1e-3,
                &[Complex64::new(0.0, 0.0)],
                &mut 1,
                &NoAbort,
                |point| {
                    if point < 0.0 {
                        Err(SimulationError::ParameterDomain(
                            "negative physical parameter".to_owned(),
                        ))
                    } else {
                        Ok(vec![Complex64::new(
                            if square_root {
                                point.sqrt()
                            } else {
                                1000.0 * point
                            },
                            0.0,
                        )])
                    }
                },
            );
            if square_root {
                assert!(
                    result
                        .unwrap_err()
                        .to_string()
                        .contains("could not resolve")
                );
            } else {
                assert!((result.unwrap()[0].re / 1000.0 - 1.0).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn refinement_does_not_treat_failed_trials_as_parameter_boundaries() {
        for (failure, direction) in
            (0..4).flat_map(|failure| [-1.0, 1.0].map(|side| (failure, side)))
        {
            let result = Engine::default().refine_sensitivity(
                "failed trial",
                0.0,
                1e-3,
                &[Complex64::new(0.0, 0.0)],
                &mut 1,
                &NoAbort,
                |point| {
                    if direction * point < 0.0 {
                        Err(match failure {
                            0 => SimulationError::ConvergenceFailed(20),
                            1 => SimulationError::Solver(
                                crate::solver::SolverError::ConvergenceFailed(20),
                            ),
                            2 => SimulationError::Circuit("device evaluation failed".into()),
                            _ => SimulationError::Netlist("parameter replay failed".into()),
                        })
                    } else {
                        Ok(vec![Complex64::new(point, 0.0)])
                    }
                },
            );
            assert!(result.is_err(), "failure {failure} produced {result:?}");
        }
    }

    #[test]
    fn refinement_recovers_two_sided_samples_after_failed_outer_trials() {
        let mut failures = 0;
        let value = Engine::default()
            .refine_sensitivity(
                "retry",
                0.0,
                1e-3,
                &[Complex64::new(0.0, 0.0)],
                &mut 1,
                &NoAbort,
                |point| {
                    if point < -1e-4 {
                        failures += 1;
                        Err(SimulationError::ConvergenceFailed(20))
                    } else {
                        Ok(vec![Complex64::new(point, 0.0)])
                    }
                },
            )
            .unwrap();
        assert_eq!(failures, 4);
        assert!((value[0].re - 1.0).abs() < 1e-12);
    }

    #[test]
    fn refinement_counts_trials_and_preserves_fatal_failures() {
        let mut config = crate::engine::SimulationConfig::default();
        config.resource_limits.max_batch_runs = 4;
        let engine = Engine::new(config);
        let mut evaluations = 0;
        let error = engine
            .refine_sensitivity(
                "budget",
                1.0,
                1e-3,
                &[Complex64::new(1.0, 0.0)],
                &mut 1,
                &NoAbort,
                |point| {
                    evaluations += 1;
                    Ok(vec![Complex64::new(point, 0.0)])
                },
            )
            .unwrap_err();
        assert!(matches!(error, SimulationError::ResourceLimit(_)));
        assert_eq!(
            evaluations, 3,
            "the budget is checked before the fourth trial"
        );

        let mut evaluations = 0;
        let error = Engine::default()
            .refine_sensitivity(
                "abort",
                1.0,
                1e-3,
                &[Complex64::new(1.0, 0.0)],
                &mut 1,
                &NoAbort,
                |_| {
                    evaluations += 1;
                    Err(SimulationError::Aborted)
                },
            )
            .unwrap_err();
        assert!(matches!(error, SimulationError::Aborted));
        assert_eq!(evaluations, 1);
    }
}
