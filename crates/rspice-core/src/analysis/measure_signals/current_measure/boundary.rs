//! The antiderivative of c*delta^(n), n>0, is c*delta^(n-1).
//! Its regular part is zero. Its value at the support is not a scalar.
use super::*;
use rspice_veriloga_runtime::arithmetic::ScaledValue;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(in super::super) struct Boundary {
    pub time: Value,
    pub primitive_order: u32,
}

impl std::fmt::Display for Boundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "INTEG/AVG boundary at time {} contains a current impulse primitive of derivative order {}; no finite scalar exists there",
            self.time, self.primitive_order
        )
    }
}

fn derivative_at<'a>(
    term: CurrentImpulseContribution<'a>,
    time: Value,
    order: u32,
) -> Option<&'a crate::CurrentImpulseDerivative> {
    let points = &term.trace.derivatives;
    let index = points
        .partition_point(|point| point.time < time || (point.time == time && point.order < order));
    points.get(index).filter(|point| point.time == time)
}

/// Test the exact weighted distribution, not each owner independently: two
/// physical currents may cancel at the same time and derivative order.
pub(super) fn at<'a>(
    terms: impl Iterator<Item = CurrentImpulseContribution<'a>> + Clone,
    time: Value,
    abort: &dyn AbortSignal,
) -> Result<Option<Boundary>, CurrentObservationError> {
    let mut minimum_order = 1;
    if abort.is_aborted() {
        return Err(CurrentObservationError::Aborted);
    }
    loop {
        let mut order = None;
        for (index, term) in terms.clone().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(CurrentObservationError::Aborted);
            }
            if let Some(point) = derivative_at(term, time, minimum_order) {
                order = Some(order.map_or(point.order, |old: u32| old.min(point.order)));
            }
        }
        let Some(order) = order else { return Ok(None) };
        let one = ScaledValue::new(1.0);
        let sum = ScaledValue::sum_triple_products_ratio(
            terms.clone().enumerate().map(|(index, term)| {
                let coefficient = if index.is_multiple_of(64) && abort.is_aborted() {
                    Value::NAN
                } else {
                    derivative_at(term, time, order)
                        .filter(|point| point.order == order)
                        .map_or(0.0, |point| point.coefficient)
                };
                [
                    ScaledValue::new(coefficient),
                    ScaledValue::new(term.weight),
                    one,
                ]
            }),
            [[one; 3]].into_iter(),
        );
        if abort.is_aborted() {
            return Err(CurrentObservationError::Aborted);
        }
        let sum = sum.map_err(|_| {
            invalid("current integral boundary accumulation exceeds its numerical precision")
        })?;
        if !sum.is_zero() {
            return Ok(Some(Boundary {
                time,
                primitive_order: order - 1,
            }));
        }
        let Some(next) = order.checked_add(1) else {
            return Ok(None);
        };
        minimum_order = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integral_boundary_cancellation_is_bounded_inside_the_exact_sum() {
        let trace = crate::CurrentImpulseTrace {
            owner: crate::CurrentImpulseOwner::Branch {
                branch_name: "V1".into(),
            },
            complete: true,
            points: vec![],
            derivatives: vec![crate::CurrentImpulseDerivative {
                time: 0.5,
                order: u32::MAX,
                coefficient: 1.0,
            }],
        };
        let visits = std::cell::Cell::new(0);
        let terms = (0..4096).map(|_| {
            visits.set(visits.get() + 1);
            CurrentImpulseContribution {
                trace: &trace,
                weight: 1.0,
            }
        });
        // Entry plus 64 discovery polls succeed. Abort in exact accumulation,
        // before traversing a second complete set of terms.
        let abort = crate::abort_signal::CountingAbort::new(65);
        assert!(matches!(
            at(terms, 0.5, &abort),
            Err(CurrentObservationError::Aborted)
        ));
        assert!(visits.get() <= 4097, "visited {} terms", visits.get());
        assert_eq!(
            at(
                std::iter::once(CurrentImpulseContribution {
                    trace: &trace,
                    weight: 1.0
                }),
                0.5,
                &NoAbort
            )
            .unwrap(),
            Some(Boundary {
                time: 0.5,
                primitive_order: u32::MAX - 1
            })
        );
    }
}
