//! Original-equation certificates, independent of the compiled projections.
use super::*;

// Binary64 integers have at most 2098 bits. A product has at most 4196;
// accumulation adds at most usize::BITS. Include conversion operands,
// signed/absolute accumulators and the exact tolerance comparison products.
// This workspace is reused one equation at a time, without a dense matrix.
pub(super) const SCRATCH_WORDS: usize = 1536;

fn equation(terms: impl Iterator<Item = (Value, Value)>, abort: &dyn AbortSignal) -> Result<()> {
    let mut residual = BigInt::default();
    let mut magnitude = BigUint::default();
    let mut count = 0usize;
    for (coefficient, value) in terms {
        check_abort(abort)?;
        if coefficient == 0.0 || value == 0.0 {
            continue;
        }
        let coefficient = integer_coefficient(coefficient)
            .ok_or_else(|| failure("nonfinite audit coefficient"))?;
        let value =
            integer_coefficient(value).ok_or_else(|| failure("nonfinite audit coordinate"))?;
        let product = coefficient * value;
        magnitude += product.magnitude();
        residual += product;
        count = count.saturating_add(1);
    }
    verify(residual, magnitude, count, abort)
}

fn exact_equation<'a>(
    terms: impl Iterator<Item = (&'a BigInt, Value)>,
    abort: &dyn AbortSignal,
) -> Result<()> {
    let mut residual = BigInt::default();
    let mut magnitude = BigUint::default();
    let mut count = 0usize;
    for (coefficient, value) in terms {
        check_abort(abort)?;
        if coefficient.sign() == Sign::NoSign || value == 0.0 {
            continue;
        }
        let value =
            integer_coefficient(value).ok_or_else(|| failure("nonfinite constraint coordinate"))?;
        let product = coefficient * value;
        magnitude += product.magnitude();
        residual += product;
        count = count.saturating_add(1);
    }
    verify(residual, magnitude, count, abort)
}

fn verify(
    residual: BigInt,
    magnitude: BigUint,
    count: usize,
    abort: &dyn AbortSignal,
) -> Result<()> {
    if residual.sign() == Sign::NoSign {
        return Ok(());
    }
    // Componentwise backward error, with no current/charge floor or invented
    // time scale. Exact products retain cancellation below binary64 products.
    let tolerance = 128.0 * Value::EPSILON * (count.saturating_add(1) as Value);
    if !tolerance.is_finite() || tolerance >= 1.0 {
        return Err(failure("audit term population cannot resolve an equation"));
    }
    check_abort(abort)?;
    let unit = integer_coefficient(1.0).unwrap();
    let tolerance = integer_coefficient(tolerance).unwrap();
    if residual.magnitude() * unit.magnitude() > magnitude * tolerance.magnitude() {
        return Err(failure(
            "original distributional equation failed its backward-error audit",
        ));
    }
    Ok(())
}

impl PreparedTransition {
    pub(super) fn audit_storage(&self, storage: &[Value], abort: &dyn AbortSignal) -> Result<()> {
        for (_, row) in &self.storage_constraints.rows {
            check_abort(abort)?;
            exact_equation(
                row.nodes
                    .iter()
                    .map(|(&index, coefficient)| (coefficient, storage[index - 1])),
                abort,
            )?;
        }
        Ok(())
    }

    pub(super) fn audit(
        &self,
        storage: &[Value],
        jets: &[Vec<Value>],
        result: &Transition,
        abort: &dyn AbortSignal,
    ) -> Result<()> {
        for row in 0..self.size {
            check_abort(abort)?;
            equation(
                self.a[row]
                    .iter()
                    .map(|&(column, coefficient)| (coefficient, result.finite()[column]))
                    .chain(
                        self.e[row]
                            .iter()
                            .map(|&(column, coefficient)| (coefficient, result.rates()[column])),
                    )
                    .chain([(-1.0, jets[row][0])]),
                abort,
            )?;
            equation(
                self.e[row]
                    .iter()
                    .map(|&(column, coefficient)| (coefficient, result.finite()[column]))
                    .chain([(-1.0, storage[row])])
                    .chain(result.impulse(0).into_iter().flat_map(|impulse| {
                        self.a[row]
                            .iter()
                            .map(move |&(column, coefficient)| (coefficient, impulse[column]))
                    })),
                abort,
            )?;
            for order in 0..self.impulse_orders {
                let impulse = result.impulse(order).unwrap();
                equation(
                    self.e[row]
                        .iter()
                        .map(|&(column, coefficient)| (coefficient, impulse[column]))
                        .chain(result.impulse(order + 1).into_iter().flat_map(|next| {
                            self.a[row]
                                .iter()
                                .map(move |&(column, coefficient)| (coefficient, next[column]))
                        })),
                    abort,
                )?;
            }
        }
        // Original E*x' + A*x does not contain rates of every algebraic
        // current. Its hidden constraints and their derivatives must also
        // qualify the published finite jet.
        for (_, row) in &self.constraints.rows {
            for extra in 0..=1 {
                let values = if extra == 0 {
                    result.finite()
                } else {
                    result.rates()
                };
                exact_equation(
                    row.nodes
                        .iter()
                        .map(|(&column, coefficient)| (coefficient, values[column - 1]))
                        .chain(row.values.iter().map(|(input, coefficient)| {
                            let value = match *input {
                                Input::Forcing { row, order } => jets[row][order + extra],
                                Input::Storage(_) => {
                                    unreachable!("closure differentiates only prescribed forcing")
                                }
                            };
                            (coefficient, -value)
                        })),
                    abort,
                )?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NoAbort;

    #[test]
    fn audit_retains_overflowing_and_subnormal_products_without_false_floors() {
        equation([(1e300, 1e300), (-1e300, 1e300)].into_iter(), &NoAbort).unwrap();
        assert!(equation([(1e300, 1e300), (-1e300, 0.99e300)].into_iter(), &NoAbort).is_err());
        let small = Value::from_bits(1);
        equation([(small, small), (-small, small)].into_iter(), &NoAbort).unwrap();
        assert!(equation([(small, small)].into_iter(), &NoAbort).is_err());
    }
}
