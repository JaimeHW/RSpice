//! Bounded Richardson refinement of private Volterra operator probes.
//!
//! MNA source currents can dominate direction normalization even when the
//! nonlinear voltage variation is tiny. A fixed step then differentiates
//! rounding noise in large cancelling linear/reactive operator products.
use super::{AbortSignal, Complex64, SimulationError, Value, check_abort};

pub(super) struct OperatorSample {
    pub values: Vec<Complex64>,
    /// Estimated rounding error before cancellation in each matrix/vector row.
    pub roundoff: Vec<Value>,
}

pub(super) struct DerivativeEstimate {
    values: Vec<Complex64>,
    roundoff: Vec<Value>,
}

impl DerivativeEstimate {
    pub(super) fn from_stencil(samples: &[(&OperatorSample, Value)], factor: Value) -> Self {
        let size = samples[0].0.values.len();
        let mut values = Vec::with_capacity(size);
        let mut roundoff = Vec::with_capacity(size);
        for row in 0..size {
            let mut value = Complex64::new(0.0, 0.0);
            let mut noise = 0.0;
            for &(sample, weight) in samples {
                value += weight * sample.values[row];
                noise += weight.abs() * sample.roundoff[row];
            }
            values.push(factor * value);
            roundoff.push(factor.abs() * noise);
        }
        Self { values, roundoff }
    }
}

/// Keep the lowest consistent truncation-plus-roundoff estimate independently
/// for each physical equation. Wider probes resolve cancellation; the nearby
/// estimates remain available when larger steps leave a constitutive domain.
/// A fixed number of stencils bounds work and every probe retains cancellation.
pub(super) fn refine(
    initial_step: Value,
    abort: &dyn AbortSignal,
    mut sample: impl FnMut(Value) -> Result<DerivativeEstimate, SimulationError>,
) -> Result<Vec<Complex64>, SimulationError> {
    let mut fine = sample(initial_step * 0.5)?;
    let mut best = vec![Complex64::new(0.0, 0.0); fine.values.len()];
    let mut errors = vec![Value::INFINITY; best.len()];
    let mut step = initial_step;
    // Twenty doublings explore six decades of step size. Strongly reactive
    // directions can put the nonlinear voltage several decades below the
    // source-current normalization; nearby estimates still protect curved
    // constitutive laws against the larger probes.
    for _ in 0..=20 {
        check_abort(abort)?;
        let coarse = match sample(step) {
            Ok(estimate) => estimate,
            Err(
                SimulationError::Circuit(_)
                | SimulationError::NonFiniteTrial(_)
                | SimulationError::ParameterDomain(_)
                | SimulationError::Solver(_)
                | SimulationError::ConvergenceFailed(_),
            ) if errors.iter().all(|error| error.is_finite()) => break,
            Err(failure) => return Err(failure),
        };
        for row in 0..best.len() {
            let correction = (fine.values[row] - coarse.values[row]) / 3.0;
            let value = fine.values[row] + correction;
            let error = correction.norm() + (4.0 * fine.roundoff[row] + coarse.roundoff[row]) / 3.0;
            // A distant saturated/flat region can appear more precise than
            // the local derivative. Preserve resolved local evidence instead
            // of accepting an unrelated derivative with a smaller error bar.
            let consistent = !errors[row].is_finite()
                || (value - best[row]).norm() <= 8.0 * (error + errors[row]);
            if value.re.is_finite() && value.im.is_finite() && error < errors[row] && consistent {
                best[row] = value;
                errors[row] = error;
            }
        }
        fine = coarse;
        step *= 2.0;
    }
    check_abort(abort)?;
    if errors.iter().any(|error| !error.is_finite()) {
        return Err(SimulationError::Circuit(
            "Volterra derivative refinement exceeded finite precision".to_owned(),
        ));
    }
    Ok(best)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::NoAbort;

    fn exponential_derivative(h: Value) -> DerivativeEstimate {
        let plus = OperatorSample {
            values: vec![Complex64::new(h.exp(), 0.0)],
            roundoff: vec![16.0 * Value::EPSILON * h.exp()],
        };
        let minus = OperatorSample {
            values: vec![Complex64::new((-h).exp(), 0.0)],
            roundoff: vec![16.0 * Value::EPSILON * (-h).exp()],
        };
        DerivativeEstimate::from_stencil(&[(&plus, 1.0), (&minus, -1.0)], 0.5 / h)
    }

    #[test]
    fn outer_domain_failure_preserves_a_local_curvature_estimate() {
        let mut reached_boundary = false;
        let result = refine(Value::EPSILON.cbrt(), &NoAbort, |h| {
            if h > 1e-3 {
                reached_boundary = true;
                Err(SimulationError::ParameterDomain("outer probe".into()))
            } else {
                Ok(exponential_derivative(h))
            }
        })
        .unwrap();
        assert!(reached_boundary);
        assert!((result[0].re - 1.0).abs() < 1e-9);
        assert_eq!(result[0].im, 0.0);
    }

    #[test]
    fn fatal_probe_errors_preserve_their_type_after_a_valid_estimate() {
        use crate::resource::{ResourceKind, ResourceLimitError};
        let resource = ResourceLimitError {
            resource: ResourceKind::ResultValues,
            requested: 2,
            limit: 1,
        };
        for (index, failure) in [
            SimulationError::Aborted,
            SimulationError::ResourceLimit(resource),
        ]
        .into_iter()
        .enumerate()
        {
            let mut calls = 0;
            let mut failure = Some(failure);
            let result = refine(Value::EPSILON.cbrt(), &NoAbort, |h| {
                calls += 1;
                if calls == 3 {
                    Err(failure.take().unwrap())
                } else {
                    Ok(exponential_derivative(h))
                }
            });
            assert_eq!(calls, 3);
            match result {
                Err(SimulationError::Aborted) => assert_eq!(index, 0),
                Err(SimulationError::ResourceLimit(error)) => {
                    assert_eq!(index, 1);
                    assert_eq!(error, resource);
                }
                other => panic!("fatal probe failure changed: {other:?}"),
            }
        }
    }

    #[test]
    fn cancellation_between_stencils_cannot_publish_an_earlier_estimate() {
        use crate::abort_signal::AtomicAbort;
        let abort = AtomicAbort::new();
        let mut calls = 0;
        let result = refine(Value::EPSILON.cbrt(), &abort, |h| {
            calls += 1;
            if calls == 2 {
                abort.set();
            }
            Ok(exponential_derivative(h))
        });
        assert_eq!(calls, 2);
        assert!(matches!(result, Err(SimulationError::Aborted)));
    }

    #[test]
    fn an_unresolvable_stencil_cannot_publish_zero() {
        let result = refine(Value::EPSILON.cbrt(), &NoAbort, |_| {
            Ok(DerivativeEstimate {
                values: vec![Complex64::new(Value::NAN, 0.0)],
                roundoff: vec![Value::INFINITY],
            })
        });
        assert!(matches!(result, Err(SimulationError::Circuit(_))));
    }
}
