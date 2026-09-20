//! Explicit signed modulation coordinates, independent of the circuit truncation.
use super::*;
use std::collections::HashMap;

impl QuasiPeriodicNoiseProjector {
    pub(super) fn validate_modulation(
        &self,
        modulation: &[Complex64],
        tuples: &[Vec<i32>],
        outputs: usize,
        abort: &dyn AbortSignal,
    ) -> Result<(), Error> {
        if modulation.len() != tuples.len() {
            return Err(invalid(
                "colored modulation coefficients differ from their explicit tone tuples",
            ));
        }
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            tuples.len(),
            self.limits.max_analysis_points,
        )?;
        self.check_values(
            outputs,
            tuples
                .len()
                .saturating_mul(self.grid.dimensions().len() + 16),
        )?;
        let mut indices = HashMap::with_capacity(tuples.len());
        for (i, (tuple, a)) in tuples.iter().zip(modulation).enumerate() {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            if tuple.len() != self.grid.dimensions().len()
                || !finite(*a)
                || indices.insert(tuple.as_slice(), i).is_some()
            {
                return Err(invalid(
                    "colored modulation requires unique exact tuples and finite coefficients",
                ));
            }
        }
        for (i, (tuple, a)) in tuples.iter().zip(modulation).enumerate() {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            let opposite = tuple
                .iter()
                .map(|k| {
                    k.checked_neg().ok_or_else(|| {
                        invalid("modulation conjugate tuple exceeds signed coordinate range")
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            // An omitted coefficient is exactly zero, allowing sparse mode lists.
            let reflected = indices
                .get(opposite.as_slice())
                .map_or(Complex64::ZERO, |j| modulation[*j].conj());
            let scale =
                a.re.abs()
                    .max(a.im.abs())
                    .max(reflected.re.abs())
                    .max(reflected.im.abs());
            if scale > 0.0 && (*a / scale - reflected / scale).norm() > 128.0 * Value::EPSILON {
                return Err(invalid(
                    "physical colored-noise modulation must be real on independent phases",
                ));
            }
        }
        Ok(())
    }
}
