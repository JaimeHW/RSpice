//! The polynomial part of a transfer is the boundary response to a unit step.
use super::*;

#[derive(Debug, thiserror::Error)]
pub(crate) enum AsymptoteError {
    #[error(transparent)]
    Constraint(#[from] ConstraintError),
    #[error("the finite high-frequency gain is not representable in binary64")]
    Unrepresentable,
}

/// Combine the observed coordinate and the selected step forcing exactly.
/// Incoming perturbation storage and all regular forcing derivatives are zero.
fn observed_coefficient(
    projector: &ExactElimination<Input>,
    offset: usize,
    input: &[Value],
    output: &[Value],
    abort: &dyn AbortSignal,
) -> Result<(BigInt, BigInt)> {
    let mut query = ExactRow {
        query: integer_coefficient(1.0).unwrap(),
        ..ExactRow::default()
    };
    for (column, &weight) in output.iter().enumerate() {
        check_abort(abort)?;
        if weight != 0.0 {
            projector.check_cost(query.words().saturating_mul(3).saturating_add(160))?;
            query
                .nodes
                .insert(offset + column + 1, integer_coefficient(weight).unwrap());
        }
    }
    projector.reduce(&mut query, abort)?;
    if !query.nodes.is_empty() {
        return Err(failure("incomplete observed transition mapping"));
    }
    // Bound products, accumulator growth, denominator and rational-rounding
    // temporaries before materializing them. One binary64 input has <=2098 bits.
    let bits = query
        .values
        .values()
        .chain(std::iter::once(&query.query))
        .map(BigInt::bits)
        .max()
        .unwrap_or(0)
        .saturating_add(2098)
        .saturating_add(u64::from(usize::BITS));
    let words = usize::try_from(bits.div_ceil(64)).unwrap_or(usize::MAX);
    projector.check_cost(
        query
            .words()
            .saturating_mul(3)
            .saturating_add(words.saturating_mul(8))
            .saturating_add(160),
    )?;
    let mut numerator = BigInt::default();
    for (source, coefficient) in &query.values {
        check_abort(abort)?;
        if let Input::Forcing { row, order: 0 } = source {
            numerator -= coefficient * integer_coefficient(input[*row]).unwrap();
        }
    }
    let denominator = query.query * integer_coefficient(1.0).unwrap();
    Ok((numerator, denominator))
}

/// Return H(infinity) for H(s)=L*(G+sC)^-1*B. None proves a nonzero
/// positive-power term; unavailable precision is a distinct error.
pub(crate) fn high_frequency_gain(
    g: &[Vec<Value>],
    c: &[Vec<Value>],
    input: &[Value],
    output: &[Value],
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> std::result::Result<Option<Value>, AsymptoteError> {
    check_abort(abort)?;
    let size = g.len();
    if size == 0
        || c.len() != size
        || input.len() != size
        || output.len() != size
        || g.iter().chain(c).any(|row| row.len() != size)
        || input.iter().chain(output).any(|value| !value.is_finite())
    {
        return Err(failure("invalid transfer observation dimensions or weights").into());
    }
    let caller_retained = size
        .saturating_mul(size)
        .saturating_mul(8)
        .saturating_add(size.saturating_mul(64));
    let mut counts = [0usize; 2];
    for (count, matrix) in counts.iter_mut().zip([g, c]) {
        for row in matrix {
            check_abort(abort)?;
            *count = count.saturating_add(row.iter().filter(|&&value| value != 0.0).count());
        }
    }
    ExactElimination::<Input>::ensure_words(
        caller_retained
            .saturating_add(size.saturating_mul(64))
            .saturating_add(counts[0].saturating_add(counts[1]).saturating_mul(16)),
        limits.max_result_values,
    )?;
    let mut entries = [Vec::with_capacity(counts[0]), Vec::with_capacity(counts[1])];
    for (entries, matrix) in entries.iter_mut().zip([g, c]) {
        for (row, values) in matrix.iter().enumerate() {
            check_abort(abort)?;
            entries.extend(
                values
                    .iter()
                    .enumerate()
                    .filter_map(|(column, &value)| (value != 0.0).then_some((row, column, value))),
            );
        }
    }
    let TransitionSystem {
        size,
        impulse_orders,
        overhead,
        a,
        e,
        algebraic,
        storage_constraints,
        mut projector,
    } = TransitionSystem::new(
        size,
        &entries[0],
        &entries[1],
        limits,
        caller_retained,
        abort,
    )?;
    // PZ observes the exact equations once. It does not need the physical
    // evaluator's retained audits, floating projection forms or storage maps.
    let released = overhead
        .saturating_add(algebraic.retained_words)
        .saturating_add(storage_constraints.retained_words)
        .saturating_add((projector.pivots.len() - 1).saturating_mul(16));
    drop((entries, a, e, algebraic, storage_constraints));
    projector.retained_words = projector
        .retained_words
        .checked_sub(released)
        .ok_or_else(|| failure("inconsistent transition workspace accounting"))?;
    projector.reserve_retained_words(caller_retained)?;
    for order in 0..impulse_orders {
        let (numerator, _) =
            observed_coefficient(&projector, (order + 1) * size, input, output, abort)?;
        if numerator.sign() != Sign::NoSign {
            // Classify before projecting: even an unrepresentably small
            // nonzero impulse coefficient proves an improper transfer.
            check_abort(abort)?;
            return Ok(None);
        }
    }
    let (numerator, denominator) = observed_coefficient(&projector, 0, input, output, abort)?;
    let gain =
        coefficient_ratio(&numerator, &denominator).ok_or(AsymptoteError::Unrepresentable)?;
    check_abort(abort)?;
    Ok(Some(gain))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn high_frequency_preparation_preserves_workspace_and_inner_cancellation_errors() {
        let g = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let c = vec![vec![1.0, -1.0], vec![-1.0, 1.0]];
        let port = [1.0, 0.0];
        let limits = ResourceLimits {
            max_result_values: 300,
            ..ResourceLimits::default()
        };
        let error = high_frequency_gain(&g, &c, &port, &port, limits, &crate::NoAbort).unwrap_err();
        let AsymptoteError::Constraint(ConstraintError::ResourceLimit(error)) = error else {
            panic!("{error:?}");
        };
        assert_eq!(error.resource, ResourceKind::ResultValues);
        assert_eq!(error.limit, 300);
        assert!(error.requested > error.limit);

        struct StopAfter(AtomicUsize);
        impl AbortSignal for StopAfter {
            fn is_aborted(&self) -> bool {
                self.0.fetch_add(1, Ordering::Relaxed) + 1 >= 25
            }
        }
        let abort = StopAfter(AtomicUsize::new(0));
        let error = high_frequency_gain(&g, &c, &port, &port, ResourceLimits::default(), &abort)
            .unwrap_err();
        assert!(matches!(
            error,
            AsymptoteError::Constraint(ConstraintError::Aborted)
        ));
        assert_eq!(abort.0.load(Ordering::Relaxed), 25);
    }
}
