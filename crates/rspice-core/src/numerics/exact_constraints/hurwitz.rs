//! Exact asymptotic stability of rational state equations.
//!
//! Positive time scaling clears denominators without moving roots across the
//! imaginary axis. Strong components are independent spectral blocks; within
//! a block, Faddeev–LeVerrier produces integer characteristic coefficients and
//! a positively scaled Routh array decides strict Hurwitz stability. No root
//! tolerance or rounded eigenvector participates in the decision.
//!
//! Recurrences: Hou, SIAM Review 40(3), 1998, doi:10.1137/S003614459732076X;
//! Jeltsch, ETH SAM Report 1995-12, section 3 (fraction-free Routh).
use super::*;

type Row = ExactRow<usize>;

struct Workspace<'a> {
    retained: usize,
    limit: usize,
    abort: &'a dyn AbortSignal,
}

impl Workspace<'_> {
    fn poll(&self) -> Result<(), ConstraintError> {
        if self.abort.is_aborted() {
            Err(ConstraintError::Aborted)
        } else {
            Ok(())
        }
    }

    /// Bound every live integer slot and arithmetic temporary before growth.
    fn integers(&self, slots: usize, bits: u64) -> Result<(), ConstraintError> {
        self.poll()?;
        let words = usize::try_from(bits.div_ceil(64))
            .unwrap_or(usize::MAX)
            .saturating_add(16);
        ExactElimination::<usize>::ensure_words(
            self.retained.saturating_add(slots.saturating_mul(words)),
            self.limit,
        )
    }
}

fn gcd(mut a: BigUint, mut b: BigUint, work: &Workspace<'_>) -> Result<BigUint, ConstraintError> {
    while b != BigUint::default() {
        work.poll()?;
        let remainder = &a % &b;
        a = b;
        b = remainder;
    }
    Ok(a)
}

/// A row states query*x'[i] + sum(values[j]*x[j]) = 0. Its query is nonzero.
pub(crate) fn rational_rows_are_hurwitz(
    rows: &[Row],
    limits: crate::resource::ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<bool, ConstraintError> {
    let n = rows.len();
    if abort.is_aborted() {
        return Err(ConstraintError::Aborted);
    }
    crate::resource::ResourceLimitError::ensure(
        crate::resource::ResourceKind::MatrixUnknowns,
        n,
        limits.max_matrix_unknowns,
    )?;
    let entries = rows
        .iter()
        .fold(0usize, |count, row| count.saturating_add(row.values.len()));
    let retained = rows.iter().fold(
        n.saturating_mul(32)
            .saturating_add(entries.saturating_mul(8)),
        |count, row| count.saturating_add(row.words()),
    );
    let work = Workspace {
        retained,
        limit: limits.max_result_values,
        abort,
    };
    work.integers(0, 0)?;
    if rows.iter().any(|row| {
        !row.nodes.is_empty()
            || row.query.sign() == Sign::NoSign
            || row.values.keys().any(|&column| column >= n)
    }) {
        return Err(ConstraintError::Invalid(
            "invalid rational stability equation".into(),
        ));
    }

    // Iterative Kosaraju avoids call-stack growth for long feedforward chains.
    let mut reverse = vec![Vec::new(); n];
    for (row, equation) in rows.iter().enumerate() {
        for (&column, value) in &equation.values {
            work.poll()?;
            if value.sign() != Sign::NoSign {
                reverse[column].push(row);
            }
        }
    }
    let mut visited = vec![false; n];
    let mut finish = Vec::with_capacity(n);
    let mut stack = Vec::new();
    for root in 0..n {
        stack.push((root, false));
        while let Some((node, finished)) = stack.pop() {
            work.poll()?;
            if finished {
                finish.push(node);
            } else if !visited[node] {
                visited[node] = true;
                stack.push((node, true));
                for (&column, value) in rows[node].values.iter().rev() {
                    if !visited[column] && value.sign() != Sign::NoSign {
                        stack.push((column, false));
                    }
                }
            }
        }
    }
    visited.fill(false);
    let mut component = Vec::new();
    for root in finish.into_iter().rev() {
        if visited[root] {
            continue;
        }
        component.clear();
        visited[root] = true;
        stack.push((root, false));
        while let Some((node, _)) = stack.pop() {
            work.poll()?;
            component.push(node);
            for &column in &reverse[node] {
                if !visited[column] {
                    visited[column] = true;
                    stack.push((column, false));
                }
            }
        }
        if !component_is_hurwitz(rows, &component, &work)? {
            return Ok(false);
        }
    }
    Ok(true)
}

pub(crate) fn matrix_is_hurwitz(
    matrix: &[Vec<Value>],
    limits: crate::resource::ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<bool, ConstraintError> {
    let n = matrix.len();
    if abort.is_aborted() {
        return Err(ConstraintError::Aborted);
    }
    crate::resource::ResourceLimitError::ensure(
        crate::resource::ResourceKind::MatrixUnknowns,
        n,
        limits.max_matrix_unknowns,
    )?;
    let mut work = Workspace {
        retained: n.saturating_mul(32),
        limit: limits.max_result_values,
        abort,
    };
    work.integers(0, 0)?;
    let mut rows = Vec::with_capacity(n);
    for values in matrix {
        if values.len() != n {
            return Err(ConstraintError::Invalid(
                "invalid stability matrix dimensions".into(),
            ));
        }
        work.integers(1, 1075)?;
        let mut row = Row {
            query: integer_coefficient(1.0).unwrap(),
            ..Row::default()
        };
        for (column, &value) in values.iter().enumerate() {
            work.poll()?;
            if value != 0.0 {
                work.integers(row.values.len().saturating_add(4), 2098)?;
                row.values.insert(
                    column,
                    integer_coefficient(-value).ok_or_else(|| {
                        ConstraintError::Invalid("nonfinite stability matrix".into())
                    })?,
                );
            }
        }
        row.normalize(abort)?;
        work.retained = work.retained.saturating_add(row.words());
        rows.push(row);
    }
    rational_rows_are_hurwitz(&rows, limits, abort)
}

fn component_is_hurwitz(
    rows: &[Row],
    component: &[usize],
    work: &Workspace<'_>,
) -> Result<bool, ConstraintError> {
    let n = component.len();
    if n == 1 {
        let row = &rows[component[0]];
        return Ok(row.values.get(&component[0]).is_some_and(|diagonal| {
            diagonal.sign() != Sign::NoSign && diagonal.sign() == row.query.sign()
        }));
    }
    let slots = n
        .saturating_mul(n)
        .saturating_mul(4)
        .saturating_add(n.saturating_mul(8))
        .saturating_add(32);
    let mut denominator = BigInt::from(1);
    let mut coefficient_bits = 1;
    for &index in component {
        let row = &rows[index];
        let bits = denominator.bits().saturating_add(row.query.bits());
        work.integers(slots, bits)?;
        let divisor = gcd(
            denominator.magnitude().clone(),
            row.query.magnitude().clone(),
            work,
        )?;
        denominator =
            denominator / BigInt::from(divisor) * BigInt::from(row.query.magnitude().clone());
        coefficient_bits =
            coefficient_bits.max(row.values.values().map(BigInt::bits).max().unwrap_or(0));
    }
    let bits = denominator.bits().saturating_add(coefficient_bits);
    work.integers(slots, bits)?;
    let mut matrix = vec![vec![BigInt::default(); n]; n];
    for (row, &original) in component.iter().enumerate() {
        work.poll()?;
        let factor = &denominator / &rows[original].query;
        for (column, &source) in component.iter().enumerate() {
            work.poll()?;
            if let Some(value) = rows[original].values.get(&source) {
                matrix[row][column] = -value * &factor;
            }
        }
    }
    // The common denominator is positive: matrix and the exact rational block
    // have identical half-plane membership. No denominator remains live below.
    drop(denominator);
    integer_matrix_is_hurwitz(&matrix, slots, work)
}

fn integer_matrix_is_hurwitz(
    matrix: &[Vec<BigInt>],
    slots: usize,
    work: &Workspace<'_>,
) -> Result<bool, ConstraintError> {
    let n = matrix.len();
    let matrix_bits = matrix.iter().flatten().map(BigInt::bits).max().unwrap_or(0);
    let sum_bits = u64::from(usize::BITS - n.leading_zeros());
    work.integers(
        slots,
        matrix_bits.saturating_add(sum_bits).saturating_add(2),
    )?;
    let mut trace = BigInt::default();
    let mut dominant = true;
    let mut strict = false;
    for (index, row) in matrix.iter().enumerate() {
        work.poll()?;
        trace += &row[index];
        let off_diagonal: BigUint = row
            .iter()
            .enumerate()
            .filter(|(col, _)| *col != index)
            .map(|(_, value)| value.magnitude())
            .sum();
        dominant &= row[index].sign() == Sign::Minus && row[index].magnitude() >= &off_diagonal;
        strict |= row[index].magnitude() > &off_diagonal;
    }
    if trace.sign() != Sign::Minus {
        return Ok(false);
    }
    // Gershgorin plus irreducibility: weakly left-half-plane row disks and
    // one strictly interior disk exclude an imaginary-axis eigenvector.
    if dominant && strict {
        return Ok(true);
    }

    let mut coefficients = vec![BigInt::from(1)];
    let mut iterate = vec![vec![BigInt::default(); n]; n];
    for (index, row) in iterate.iter_mut().enumerate() {
        row[index] = BigInt::from(1);
    }
    for order in 1..=n {
        let iterate_bits = iterate
            .iter()
            .flatten()
            .map(BigInt::bits)
            .max()
            .unwrap_or(0);
        let bits = matrix_bits
            .saturating_add(iterate_bits)
            .saturating_add(sum_bits.saturating_mul(2))
            .saturating_add(2)
            .max(coefficients.iter().map(BigInt::bits).max().unwrap_or(0));
        work.integers(slots, bits)?;
        let mut product = vec![vec![BigInt::default(); n]; n];
        for (row, source) in matrix.iter().enumerate() {
            for (inner, weight) in source.iter().enumerate() {
                work.poll()?;
                if weight.sign() == Sign::NoSign {
                    continue;
                }
                for (target, value) in product[row].iter_mut().zip(&iterate[inner]) {
                    work.poll()?;
                    *target += weight * value;
                }
            }
        }
        let trace: BigInt = product.iter().enumerate().map(|(i, row)| &row[i]).sum();
        let divisor = BigInt::from(order);
        if (&trace % &divisor).sign() != Sign::NoSign {
            return Err(ConstraintError::Invalid(
                "nonexact characteristic polynomial division".into(),
            ));
        }
        let coefficient = -trace / divisor;
        // Every coefficient of a real monic Hurwitz polynomial is positive.
        if coefficient.sign() != Sign::Plus {
            return Ok(false);
        }
        for (index, row) in product.iter_mut().enumerate() {
            row[index] += &coefficient;
        }
        coefficients.push(coefficient);
        iterate = product;
    }
    drop(iterate);
    polynomial_is_hurwitz(&coefficients, matrix_bits, slots, work)
}

fn polynomial_is_hurwitz(
    coefficients: &[BigInt],
    matrix_bits: u64,
    slots: usize,
    work: &Workspace<'_>,
) -> Result<bool, ConstraintError> {
    let order = coefficients.len() - 1;
    if order == 0 {
        return Ok(true);
    }
    let retained_bits = coefficients
        .iter()
        .map(BigInt::bits)
        .max()
        .unwrap_or(0)
        .max(matrix_bits);
    work.integers(slots, retained_bits)?;
    let width = (order + 2) / 2;
    let mut upper = vec![BigInt::default(); width];
    let mut lower = vec![BigInt::default(); width];
    for (index, value) in coefficients.iter().enumerate() {
        if index % 2 == 0 {
            upper[index / 2] = value.clone();
        } else {
            lower[index / 2] = value.clone();
        }
    }
    for _ in 1..order {
        if lower[0].sign() != Sign::Plus {
            return Ok(false);
        }
        let bits = upper
            .iter()
            .chain(&lower)
            .map(BigInt::bits)
            .max()
            .unwrap_or(0)
            .saturating_mul(2)
            .saturating_add(1);
        work.integers(slots, bits.max(retained_bits))?;
        let mut next = vec![BigInt::default(); width];
        for column in 0..width - 1 {
            work.poll()?;
            next[column] = &lower[0] * &upper[column + 1] - &upper[0] * &lower[column + 1];
        }
        // Omitted division by lower[0] is a positive row scaling. Remove the
        // common positive gcd to bound growth without changing any signs.
        let mut divisor = BigUint::default();
        for value in &next {
            divisor = gcd(divisor, value.magnitude().clone(), work)?;
        }
        if divisor != BigUint::default() {
            let divisor = BigInt::from(divisor);
            for value in &mut next {
                *value /= &divisor;
            }
        }
        upper = lower;
        lower = next;
    }
    Ok(lower[0].sign() == Sign::Plus)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NoAbort, ResourceLimits, abort_signal::CountingAbort};

    fn stable(matrix: &[Vec<Value>]) -> bool {
        matrix_is_hurwitz(matrix, ResourceLimits::default(), &NoAbort).unwrap()
    }

    fn companion(coefficients: &[Value]) -> Vec<Vec<Value>> {
        let n = coefficients.len();
        let mut matrix = vec![vec![0.0; n]; n];
        for (index, row) in matrix.iter_mut().enumerate().take(n - 1) {
            row[index + 1] = 1.0;
        }
        for (target, coefficient) in matrix[n - 1].iter_mut().zip(coefficients.iter().rev()) {
            *target = -coefficient;
        }
        matrix
    }

    #[test]
    fn integer_second_order_grid_matches_trace_and_determinant() {
        for a in -2..=2 {
            for b in -2..=2 {
                for c in -2..=2 {
                    for d in -2..=2 {
                        let matrix = vec![
                            vec![f64::from(a), f64::from(b)],
                            vec![f64::from(c), f64::from(d)],
                        ];
                        assert_eq!(
                            stable(&matrix),
                            a + d < 0 && a * d - b * c > 0,
                            "{matrix:?}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn cubic_grid_matches_independent_hurwitz_inequalities() {
        for a in 1..=5 {
            for b in 1..=5 {
                for c in -1..=26 {
                    assert_eq!(
                        stable(&companion(&[f64::from(a), f64::from(b), f64::from(c)])),
                        c > 0 && a * b > c,
                        "s^3 + {a}s^2 + {b}s + {c}"
                    );
                }
            }
        }
    }

    #[test]
    fn repeated_negative_poles_and_nonnormal_similarity_keep_their_signs() {
        for order in 2..=12 {
            let mut coefficient = 1u64;
            let coefficients: Vec<_> = (1..=order)
                .map(|index| {
                    coefficient = coefficient * (order + 1 - index) / index;
                    coefficient as f64
                })
                .collect();
            assert!(stable(&companion(&coefficients)), "(s+1)^{order}");
        }
        for exponent in [0, 25, 100, 400] {
            let a = 2.0f64.powi(exponent);
            let damping = a * 2.0f64.powi(-48);
            for sign in [-1.0, 0.0, 1.0] {
                let shift = sign * damping;
                // A = -shift*I + N, with N^2 exactly zero in binary64.
                let matrix = vec![vec![-a - shift, a * a], vec![-1.0, a - shift]];
                assert_eq!(stable(&matrix), sign > 0.0, "scale={exponent}, sign={sign}");
            }
        }
    }

    #[test]
    fn smallest_damping_is_not_replaced_by_a_numerical_deadband() {
        for damping in [f64::from_bits(1), 2.0f64.powi(-500), 2.0f64.powi(-60), 0.5] {
            for sign in [-1.0, 0.0, 1.0] {
                let d = sign * damping;
                assert_eq!(stable(&[vec![-d, -1.0], vec![1.0, -d]]), sign > 0.0);
            }
        }
    }

    #[test]
    fn strong_components_ignore_feedforward_gain_and_retain_hidden_modes() {
        let mut matrix = vec![vec![0.0; 128]; 128];
        for (index, row) in matrix.iter_mut().enumerate() {
            row[index] = -1.0;
            if index > 0 {
                row[index - 1] = f64::MAX;
            }
        }
        assert!(stable(&matrix));
        matrix[61][61] = 0.0;
        assert!(!stable(&matrix));
    }

    #[test]
    fn proof_obeys_every_cancellation_poll_and_workspace_limit() {
        let matrix = companion(&[4.0, 6.0, 4.0, 1.0]);
        let count = CountingAbort::new(usize::MAX);
        assert!(matrix_is_hurwitz(&matrix, ResourceLimits::default(), &count).unwrap());
        for threshold in 0..count.count() {
            let abort = CountingAbort::new(threshold);
            assert!(matches!(
                matrix_is_hurwitz(&matrix, ResourceLimits::default(), &abort),
                Err(ConstraintError::Aborted)
            ));
            assert_eq!(abort.polls_after_abort(), 0);
        }
        for limit in [0, 100, 500, 1000] {
            let mut limits = ResourceLimits::default();
            limits.max_result_values = limit;
            assert!(matches!(
                matrix_is_hurwitz(&matrix, limits, &NoAbort),
                Err(ConstraintError::ResourceLimit(_))
            ));
        }
    }
}
