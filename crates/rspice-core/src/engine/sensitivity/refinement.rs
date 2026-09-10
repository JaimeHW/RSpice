//! Shared finite-difference refinement for real and complex circuit probes.

use super::{
    Engine, SimulationError, sensitivity_ratio, sensitivity_three_point, sensitivity_trial,
};
use crate::abort_signal::AbortSignal;
use crate::{Complex64, Value};
use rspice_veriloga_runtime::arithmetic::ScaledValue;

const RELATIVE_AGREEMENT: Value = 1e-4;
const MAX_REFINEMENTS: usize = 12;

struct Sample {
    coordinate: Value,
    values: Vec<Complex64>,
}

#[derive(Default)]
struct Pair {
    positive: Option<Sample>,
    negative: Option<Sample>,
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

impl Engine {
    /// Compare central and one-sided estimates over successively halved steps.
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
            let mut sample = |candidate: Value| -> Result<Option<Sample>, SimulationError> {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                if !candidate.is_finite() || candidate == coordinate {
                    return Ok(None);
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
                        Ok(Some(Sample {
                            coordinate: candidate,
                            values,
                        }))
                    }
                    Err(error) => {
                        last_failure = Some(format!("at {candidate}: {error}"));
                        Ok(None)
                    }
                }
            };
            Ok(Pair {
                positive: sample(coordinate + step)?,
                negative: sample(coordinate - step)?,
            })
        };

        let mut outer = sample_pair(step)?;
        let mut older = Pair::default();
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
            let inner = sample_pair(step)?;
            let mut candidate = Vec::with_capacity(nominal.len());
            let mut qualified = true;
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
                        .multiply(ScaledValue::new(64.0 * Value::EPSILON))
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
                                if outer.positive.is_none() && inner.positive.is_none() =>
                            {
                                Some(false)
                            }
                            (None, Some(_))
                                if outer.negative.is_none() && inner.negative.is_none() =>
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
            "Sensitivity parameter '{name}' could not resolve a derivative: one-sided or refined estimates disagree, or distinct finite coordinates are exhausted (possible discontinuity or insufficient numerical precision){}",
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
                        Err(SimulationError::Circuit(
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
