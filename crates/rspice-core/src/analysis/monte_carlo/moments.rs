//! Shared sample moments, retaining range until the publication boundary.

use crate::Value;
use crate::abort_signal::AbortSignal;
use crate::analysis::error::SimulationError;
use rspice_veriloga_runtime::arithmetic::ScaledValue;

#[derive(Default)]
struct CompensatedSum {
    sum: Value,
    correction: Value,
}

impl CompensatedSum {
    fn add(&mut self, value: Value) {
        crate::numerics::compensated_add(&mut self.sum, &mut self.correction, value);
    }

    fn total(self) -> Value {
        self.sum + self.correction
    }
}

pub(super) fn sample_mean(samples: impl ExactSizeIterator<Item = Value> + Clone) -> Value {
    let count = samples.len() as Value;
    // Scaling each input first can erase a representable remainder when large
    // terms cancel. Sum the original values exactly and round only the ratio.
    // A nonempty finite population always has a representable mean; invalid
    // inputs remain unavailable rather than publishing a partial estimate.
    rspice_veriloga_runtime::arithmetic::sum_products_ratio(
        samples.map(|value| (value, 1.0)),
        std::iter::once((count, 1.0)),
    )
    .unwrap_or(Value::NAN)
}

pub(super) fn statistical_location_scale(
    samples: &[Value],
    min: Value,
    max: Value,
) -> (Value, Value) {
    // Center only a tightly clustered, single-sign population. Those
    // differences are exact by Sterbenz's lemma; centering a population that
    // spans zero can erase small values before compensation can recover them.
    let anchor = if (min > 0.0 && min >= max * 0.5) || (max < 0.0 && max <= min * 0.5) {
        samples[0]
    } else {
        0.0
    };
    (anchor, (min - anchor).abs().max((max - anchor).abs()))
}

/// The sample standard deviation of a validated, nonempty finite population.
/// Keep its exponent: a representable interval can have an overflowing sample
/// deviation, or a width and center smaller than the least binary64 value.
pub(super) fn standard_deviation(
    samples: &[Value],
    anchor: Value,
    scale: Value,
    abort: &dyn AbortSignal,
) -> Result<ScaledValue, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::from_abort(abort));
    }
    if scale == 0.0 {
        return Ok(ScaledValue::new(0.0));
    }
    let n = samples.len() as Value;
    let mut sum = CompensatedSum::default();
    for chunk in samples.chunks(64) {
        if abort.is_aborted() {
            return Err(SimulationError::from_abort(abort));
        }
        for value in chunk {
            sum.add((value - anchor) / scale);
        }
    }
    let normalized_mean = sum.total() / n;
    let mut sum = CompensatedSum::default();
    for chunk in samples.chunks(64) {
        if abort.is_aborted() {
            return Err(SimulationError::from_abort(abort));
        }
        for value in chunk {
            sum.add(((value - anchor) / scale - normalized_mean).powi(2));
        }
    }
    let variance = sum.total() / (n - 1.0).max(1.0);
    Ok(ScaledValue::new(variance.sqrt()).multiply(ScaledValue::new(scale)))
}
