//! Exact algebraic closure of a constant first-order descriptor.
//!
//! Derivative columns are eliminated before finite coordinates. Every new
//! algebraic constraint is differentiated and fed back into the descriptor,
//! exposing hidden constraints without a sampled Jacobian or a rank cutoff.
use super::*;

pub(in crate::engine) enum ConstraintDisposition {
    /// The analysis owner evaluates this original constitutive equation.
    RetainAtOwner,
    /// A different constitutive/state formulation must own the descriptor.
    Defer,
}

/// Close `E*x' + A*x = b` using coordinates `1..=size` and derivative
/// coordinates `size + 1..=2*size`. Symbolic right-hand-side forcing stays
/// in `values` with its authored sign. The caller supplies differentiation of symbols
/// and the disposition of residual equations containing only forcing.
///
/// `retained_words` reserves caller-owned storage, including the pending-row
/// vector's headers/capacity. Row coefficients and both reducers are charged
/// here. The returned reducer retains that reservation through its limit.
pub(in crate::engine) fn close_descriptor<K: Ord + Copy>(
    size: usize,
    mut rows: Vec<ExactRow<K>>,
    limits: crate::resource::ResourceLimits,
    retained_words: usize,
    abort: &dyn AbortSignal,
    differentiated: impl Fn(K) -> Result<K, SimulationError>,
    mut remainder: impl FnMut(ExactRow<K>) -> Result<ConstraintDisposition, SimulationError>,
) -> Result<Option<ExactElimination<K>>, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let invalid = || SimulationError::Circuit("invalid exact descriptor coordinates".to_owned());
    let columns = size
        .checked_mul(2)
        .and_then(|value| value.checked_add(1))
        .ok_or_else(invalid)?;
    if size == 0 {
        return Err(invalid());
    }
    let mut pending_words = 0usize;
    for row in &rows {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if row.query.sign() != Sign::NoSign
            || row.nodes.keys().any(|&node| node == 0 || node >= columns)
        {
            return Err(invalid());
        }
        pending_words = pending_words.saturating_add(row.words());
    }
    ExactElimination::<K>::ensure_words(
        retained_words.saturating_add(pending_words).saturating_add(
            columns
                .saturating_add(size.saturating_add(1))
                .saturating_mul(2),
        ),
        limits.max_result_values,
    )?;
    let mut dynamic = ExactElimination::<K>::new(columns, limits)?;
    let mut algebraic = ExactElimination::<K>::new(size + 1, limits)?;
    while let Some(row) = rows.pop() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        pending_words -= row.words();
        dynamic.limits.max_result_values = limits.max_result_values.saturating_sub(
            retained_words
                .saturating_add(pending_words)
                .saturating_add(algebraic.retained_words),
        );
        let Some(row) = dynamic.admit(row, size + 1, abort)? else {
            continue;
        };
        algebraic.limits.max_result_values = limits.max_result_values.saturating_sub(
            retained_words
                .saturating_add(pending_words)
                .saturating_add(dynamic.retained_words),
        );
        if let Some(row) = algebraic.admit(row, 1, abort)? {
            if !row.values.is_empty() && matches!(remainder(row)?, ConstraintDisposition::Defer) {
                return Ok(None);
            }
            continue;
        }
        let row = &algebraic.rows.last().unwrap().1;
        algebraic.check_cost(row.words().saturating_mul(3))?;
        let mut derivative = ExactRow {
            nodes: row
                .nodes
                .iter()
                .map(|(&node, value)| (size + node, value.clone()))
                .collect(),
            values: BTreeMap::new(),
            query: BigInt::default(),
        };
        for (&source, value) in &row.values {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            // Different symbols may share a derivative. Preserve their
            // combined equation instead of overwriting an earlier term.
            ExactElimination::<K>::add_integer(
                &mut derivative.values,
                differentiated(source)?,
                value.clone(),
            );
        }
        pending_words = pending_words.saturating_add(derivative.words());
        rows.push(derivative);
    }
    drop(dynamic);
    drop(rows);
    algebraic.limits.max_result_values = limits.max_result_values.saturating_sub(retained_words);
    Ok(Some(algebraic))
}

#[cfg(test)]
mod tests;
