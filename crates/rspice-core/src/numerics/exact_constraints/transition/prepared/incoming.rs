//! Incoming storage in exact binary-product units, before any projection.
use super::*;

pub(crate) struct TransitionStorage {
    pub(super) values: Vec<BigInt>,
}

impl TransitionStorage {
    pub(crate) fn retained_words(&self) -> usize {
        self.values
            .iter()
            .fold(self.values.capacity().saturating_mul(8), |sum, value| {
                sum.saturating_add(usize::try_from(value.bits().div_ceil(64)).unwrap_or(usize::MAX))
            })
    }
}

impl PreparedTransition {
    /// Each term is (row, physical coefficient, accepted device coordinate).
    /// Keeping C*V and L*I exact avoids inventing actions from rounding Q first.
    pub(crate) fn storage_from_products(
        &self,
        products: impl Iterator<Item = (usize, Value, Value)>,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<TransitionStorage> {
        check_abort(abort)?;
        // A binary64 product uses at most 4196 bits; summing at most usize::MAX
        // terms adds usize::BITS. Include vector, integer and scratch overhead.
        let row_words = (4196 + usize::BITS as usize).div_ceil(64) + 16;
        ExactElimination::<Input>::ensure_words(
            self.retained_words
                .saturating_add(self.size.saturating_mul(row_words))
                .saturating_add(audit::SCRATCH_WORDS),
            limits.max_result_values,
        )?;
        let mut values = vec![BigInt::default(); self.size];
        for (row, coefficient, coordinate) in products {
            check_abort(abort)?;
            let value = values
                .get_mut(row)
                .ok_or_else(|| failure("invalid storage row"))?;
            let coefficient = integer_coefficient(coefficient)
                .ok_or_else(|| failure("nonfinite storage coefficient"))?;
            let coordinate = integer_coefficient(coordinate)
                .ok_or_else(|| failure("nonfinite storage coordinate"))?;
            *value += coefficient * coordinate;
        }
        Ok(TransitionStorage { values })
    }

    pub(crate) fn charge_from_coordinates(
        &self,
        coordinates: &[Value],
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<TransitionStorage> {
        check_abort(abort)?;
        if coordinates.len() != self.size || coordinates.iter().any(|v| !v.is_finite()) {
            return Err(failure("invalid storage coordinates"));
        }
        self.storage_from_products(
            self.e.iter().enumerate().flat_map(|(row, terms)| {
                terms
                    .iter()
                    .map(move |&(column, coefficient)| (row, coefficient, coordinates[column]))
            }),
            limits,
            abort,
        )
    }
}
