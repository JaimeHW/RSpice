//! Bounded fraction-free constraint elimination shared by analysis owners.
//! Authored binary64 coefficients retain their exact rank during preparation.

use crate::{AbortSignal, Value};
use num_bigint::{BigInt, BigUint, Sign};
use std::collections::BTreeMap;

/// Failures owned by the numerical kernel, independent of analysis dispatch.
#[derive(Debug, thiserror::Error)]
pub(crate) enum ConstraintError {
    #[error("exact constraint elimination was aborted")]
    Aborted,
    #[error("{0}")]
    Invalid(String),
    #[error(transparent)]
    ResourceLimit(#[from] crate::resource::ResourceLimitError),
}

mod descriptor;
pub(crate) mod finite;
pub(crate) mod hurwitz;
pub(crate) mod transition;
pub(crate) use descriptor::{ConstraintDisposition, close_descriptor};

/// Scratch words for binary64 product operands, exact signed/absolute sums,
/// and a rounded ratio. Products use at most 4196 bits and accumulation adds
/// at most usize::BITS. The caller charges this reusable per-equation space.
pub(crate) const RESIDUAL_SCRATCH_WORDS: usize = 1536;

/// Componentwise backward error of sum(a*x)=0, retaining products and
/// cancellation exactly before rounding the normalized residual upward.
pub(crate) fn equation_backward_error(
    terms: impl IntoIterator<Item = (Value, Value)>,
    abort: &dyn AbortSignal,
) -> Result<Value, ConstraintError> {
    let mut residual = BigInt::default();
    let mut magnitude = BigUint::default();
    for (coefficient, value) in terms {
        if abort.is_aborted() {
            return Err(ConstraintError::Aborted);
        }
        if !coefficient.is_finite() || !value.is_finite() {
            return Err(ConstraintError::Invalid(
                "nonfinite original-equation audit term".into(),
            ));
        }
        if coefficient == 0.0 || value == 0.0 {
            continue;
        }
        let product =
            integer_coefficient(coefficient).unwrap() * integer_coefficient(value).unwrap();
        magnitude += product.magnitude();
        residual += product;
    }
    if abort.is_aborted() {
        return Err(ConstraintError::Aborted);
    }
    if residual.sign() == Sign::NoSign {
        return Ok(0.0);
    }
    // |sum| <= sum(|terms|), so an unrepresentable ratio can only be a
    // positive underflow. Retain a conservative nonzero upper bound.
    Ok(coefficient_ratio(
        &BigInt::from(residual.magnitude().clone()),
        &BigInt::from(magnitude),
    )
    .map_or(Value::from_bits(1), Value::next_up))
}

/// Prove full rank of a square matrix without rounding its coefficients.
/// Entries must be unique and ordered by column; absent entries are zero.
/// Eliminate columns as rows of the transpose so only one incoming sparse
/// column is retained beside the bounded elimination workspace.
pub(crate) fn has_full_rank(
    size: usize,
    entries: impl IntoIterator<Item = (usize, usize, Value)>,
    limits: crate::resource::ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<bool, ConstraintError> {
    if abort.is_aborted() {
        return Err(ConstraintError::Aborted);
    }
    crate::resource::ResourceLimitError::ensure(
        crate::resource::ResourceKind::MatrixUnknowns,
        size,
        limits.max_matrix_unknowns,
    )?;
    let mut reducer = ExactElimination::<()>::new(size, limits)?;
    let mut column = 0;
    let mut equation = ExactRow::default();
    for (row, next_column, value) in entries {
        if abort.is_aborted() {
            return Err(ConstraintError::Aborted);
        }
        if row >= size || next_column >= size || next_column < column || !value.is_finite() {
            return Err(ConstraintError::Invalid(
                "invalid exact-rank matrix entry".to_owned(),
            ));
        }
        if next_column != column {
            if next_column != column + 1 || reducer.admit(equation, 0, abort)?.is_some() {
                return Ok(false);
            }
            equation = ExactRow::default();
            column = next_column;
        }
        if value != 0.0 {
            // A binary64 integer coefficient has at most 2098 bits. Account
            // for it and map overhead before materializing the next entry.
            reducer.check_cost(equation.words().saturating_add(160))?;
            equation
                .nodes
                .insert(row, integer_coefficient(value).unwrap());
        }
    }
    if size == 0 {
        return Ok(true);
    }
    if column != size - 1 || reducer.admit(equation, 0, abort)?.is_some() {
        return Ok(false);
    }
    Ok(reducer.rows.len() == size)
}

#[derive(Debug, Clone)]
pub(crate) struct ExactRow<K> {
    pub(crate) nodes: BTreeMap<usize, BigInt>,
    pub(crate) values: BTreeMap<K, BigInt>,
    /// Coefficient of the queried coordinate, absent on physical equations.
    pub(crate) query: BigInt,
}

/// Fraction-free sparse elimination preserves the exact rank of the authored
/// binary64 gains. Rounding a product before subtracting another control path
/// can otherwise create or remove a physical coordinate or conservation law.
#[derive(Debug, Clone)]
pub(crate) struct ExactElimination<K> {
    pub(crate) rows: Vec<(usize, ExactRow<K>)>,
    pub(crate) pivots: Vec<Option<usize>>,
    pub(crate) limits: crate::resource::ResourceLimits,
    pub(crate) retained_words: usize,
}

impl<K> Default for ExactRow<K> {
    fn default() -> Self {
        Self {
            nodes: BTreeMap::new(),
            values: BTreeMap::new(),
            query: BigInt::default(),
        }
    }
}

/// Represent a finite binary64 number in units of 2^-1074 without rounding.
pub(crate) fn integer_coefficient(value: Value) -> Option<BigInt> {
    if !value.is_finite() {
        return None;
    }
    let bits = value.to_bits();
    let exponent = (bits >> 52) & 0x7ff;
    let mantissa = (bits & ((1_u64 << 52) - 1)) | if exponent == 0 { 0 } else { 1_u64 << 52 };
    let coefficient = BigInt::from(mantissa) << exponent.saturating_sub(1) as usize;
    Some(if value.is_sign_negative() {
        -coefficient
    } else {
        coefficient
    })
}

/// Round a rational coefficient, including subnormal ties. Refuse a nonzero
/// edge that cannot survive projection into finite binary64 coefficients.
pub(crate) fn coefficient_ratio(numerator: &BigInt, denominator: &BigInt) -> Option<Value> {
    if numerator.sign() == Sign::NoSign {
        return Some(0.0);
    }
    if denominator.sign() == Sign::NoSign {
        return None;
    }
    let mut n = numerator.magnitude().clone();
    let mut d = denominator.magnitude().clone();
    let exponent = i64::try_from(n.bits()).ok()? - i64::try_from(d.bits()).ok()?;
    if !(-1075..=1024).contains(&exponent) {
        return None;
    }
    let below = if exponent >= 0 {
        n < (&d << exponent as usize)
    } else {
        (&n << (-exponent) as usize) < d
    };
    let exponent = exponent - i64::from(below);
    let shift = (52 - exponent).min(1074);
    if shift >= 0 {
        n <<= shift as usize;
    } else {
        d <<= (-shift) as usize;
    }
    let mut quotient = &n / &d;
    let remainder = (&n % &d) << 1;
    if remainder > d || (remainder == d && quotient.bit(0)) {
        quotient += 1_u32;
    }
    let digits = quotient.to_u64_digits();
    if digits.len() != 1 {
        return None;
    }
    let value = libm::scalbn(digits[0] as Value, -shift as i32);
    if !value.is_finite() || value == 0.0 {
        return None;
    }
    Some(if numerator.sign() != denominator.sign() {
        -value
    } else {
        value
    })
}

impl<K: Ord + Copy> ExactRow<K> {
    pub(crate) fn words(&self) -> usize {
        self.nodes
            .values()
            .chain(self.values.values())
            .chain(std::iter::once(&self.query))
            .fold(0_usize, |sum, value| {
                sum.saturating_add(
                    usize::try_from(value.bits().div_ceil(64))
                        .unwrap_or(usize::MAX)
                        .saturating_add(16),
                )
            })
    }

    pub(crate) fn normalize(&mut self, abort: &dyn AbortSignal) -> Result<(), ConstraintError> {
        let mut divisor = BigUint::default();
        for value in self
            .nodes
            .values()
            .chain(self.values.values())
            .chain(std::iter::once(&self.query))
        {
            let mut other = value.magnitude().clone();
            if abort.is_aborted() {
                return Err(ConstraintError::Aborted);
            }
            while other != BigUint::default() {
                if abort.is_aborted() {
                    return Err(ConstraintError::Aborted);
                }
                let remainder = &divisor % &other;
                divisor = other;
                other = remainder;
            }
            if divisor == BigUint::from(1_u32) {
                return Ok(());
            }
        }
        if divisor != BigUint::default() {
            let divisor = BigInt::from(divisor);
            for value in self
                .nodes
                .values_mut()
                .chain(self.values.values_mut())
                .chain(std::iter::once(&mut self.query))
            {
                *value /= &divisor;
            }
        }
        Ok(())
    }
}

impl<K: Ord + Copy> ExactElimination<K> {
    pub(crate) fn new(
        nodes: usize,
        limits: crate::resource::ResourceLimits,
    ) -> Result<Self, ConstraintError> {
        Self::ensure_words(nodes.saturating_mul(2), limits.max_result_values)?;
        Ok(Self {
            rows: Vec::new(),
            pivots: vec![None; nodes],
            limits,
            retained_words: nodes.saturating_mul(2),
        })
    }

    pub(crate) fn ensure_words(words: usize, limit: usize) -> Result<(), ConstraintError> {
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::ResultValues,
            words,
            limit,
        )?;
        Ok(())
    }

    pub(crate) fn check_cost(&self, extra: usize) -> Result<(), ConstraintError> {
        Self::ensure_words(
            self.retained_words.saturating_add(extra),
            self.limits.max_result_values,
        )
    }

    pub(crate) fn reserve_retained_words(&mut self, words: usize) -> Result<(), ConstraintError> {
        self.check_cost(words)?;
        self.retained_words = self.retained_words.saturating_add(words);
        Ok(())
    }

    pub(crate) fn add_integer<J: Ord>(terms: &mut BTreeMap<J, BigInt>, key: J, value: BigInt) {
        let sum = terms.get(&key).cloned().unwrap_or_default() + value;
        if sum.sign() == Sign::NoSign {
            terms.remove(&key);
        } else {
            terms.insert(key, sum);
        }
    }

    pub(crate) fn reduce(
        &self,
        row: &mut ExactRow<K>,
        abort: &dyn AbortSignal,
    ) -> Result<(), ConstraintError> {
        self.check_cost(row.words().saturating_mul(3))?;
        while let Some((node, pivot)) = row
            .nodes
            .keys()
            .find_map(|&node| self.pivots[node].map(|pivot| (node, pivot)))
        {
            if abort.is_aborted() {
                return Err(ConstraintError::Aborted);
            }
            let base = &self.rows[pivot].1;
            // Bound the fraction-free products before allocating them. Map
            // overhead and both input/product temporaries are included.
            let coefficients = row
                .nodes
                .len()
                .saturating_add(row.values.len())
                .saturating_add(base.nodes.len())
                .saturating_add(base.values.len())
                .saturating_add(1);
            let bits = row
                .nodes
                .values()
                .chain(row.values.values())
                .chain(std::iter::once(&row.query))
                .map(BigInt::bits)
                .max()
                .unwrap_or(0)
                .saturating_add(
                    base.nodes
                        .values()
                        .chain(base.values.values())
                        .map(BigInt::bits)
                        .max()
                        .unwrap_or(0),
                )
                .saturating_add(1);
            let words = usize::try_from(bits.div_ceil(64))
                .unwrap_or(usize::MAX)
                .saturating_add(16);
            self.check_cost(coefficients.saturating_mul(words).saturating_mul(3))?;
            let factor = row.nodes.remove(&node).unwrap();
            let scale = &base.nodes[&node];
            for value in row
                .nodes
                .values_mut()
                .chain(row.values.values_mut())
                .chain(std::iter::once(&mut row.query))
            {
                *value *= scale;
            }
            for (&column, value) in &base.nodes {
                if column != node {
                    Self::add_integer(&mut row.nodes, column, -(&factor * value));
                }
            }
            for (&column, value) in &base.values {
                Self::add_integer(&mut row.values, column, -(&factor * value));
            }
            row.normalize(abort)?;
        }
        Ok(())
    }

    /// Eliminate known pivots, then retain a pivot in the requested variable
    /// block. A remaining row belongs to the complementary constraint space.
    pub(crate) fn admit(
        &mut self,
        mut row: ExactRow<K>,
        first_pivot: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Option<ExactRow<K>>, ConstraintError> {
        row.normalize(abort)?;
        self.reduce(&mut row, abort)?;
        let Some((&pivot, _)) = row
            .nodes
            .range(first_pivot..)
            .max_by(|a, b| a.1.magnitude().cmp(b.1.magnitude()))
        else {
            return Ok(Some(row));
        };
        self.reserve_retained_words(row.words())?;
        self.pivots[pivot] = Some(self.rows.len());
        self.rows.push((pivot, row));
        Ok(None)
    }
}

#[cfg(test)]
mod rank_tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, NoAbort};
    use crate::resource::{ResourceKind, ResourceLimits};

    const REGULAR: [(usize, usize, Value); 4] = [
        (0, 0, 0.06666666666666668),
        (1, 0, 0.2),
        (0, 1, 0.1),
        (1, 1, 0.3),
    ];

    #[test]
    fn original_equation_error_retains_overflow_underflow_and_cancellation() {
        let error = |terms| equation_backward_error(terms, &NoAbort).unwrap();
        assert_eq!(error([(1e300, 1e300), (-1e300, 1e300)]), 0.0);
        assert!(error([(1e-300, 1e-300), (0.0, 0.0)]) >= 1.0);
        // Binary64 rounds 1.1*1.1 before subtraction, hiding this residual.
        assert!(error([(1.1, 1.1), (-1.1_f64.powi(2), 1.0)]) > 0.0);
        assert!(equation_backward_error([(Value::NAN, 0.0)], &NoAbort).is_err());
        let abort = CountingAbort::new(1);
        assert!(matches!(
            equation_backward_error([(1.0, 1.0), (-1.0, 1.0)], &abort),
            Err(ConstraintError::Aborted)
        ));
    }

    #[test]
    fn rank_preserves_sub_ulp_determinants_and_empty_columns() {
        let limits = ResourceLimits::default();
        assert!(has_full_rank(2, REGULAR, limits, &NoAbort).unwrap());
        for entries in [
            vec![(0, 0, 1.0), (1, 0, 2.0), (0, 1, 1.0), (1, 1, 2.0)],
            vec![(1, 1, 1.0)],
            vec![(0, 0, 1.0)],
            vec![],
        ] {
            assert!(!has_full_rank(2, entries, limits, &NoAbort).unwrap());
        }
    }

    #[test]
    fn rank_honors_abort_and_workspace_limits_during_elimination() {
        let limits = ResourceLimits::default();
        let counter = CountingAbort::new(usize::MAX);
        assert!(has_full_rank(2, REGULAR, limits, &counter).unwrap());
        for threshold in 0..counter.count() {
            let abort = CountingAbort::new(threshold);
            assert!(matches!(
                has_full_rank(2, REGULAR, limits, &abort),
                Err(ConstraintError::Aborted)
            ));
            assert_eq!(abort.count(), threshold + 1);
        }
        let mut limited = limits;
        limited.max_result_values = 100;
        assert!(matches!(
            has_full_rank(2, REGULAR, limited, &NoAbort),
            Err(ConstraintError::ResourceLimit(error))
                if error.resource == ResourceKind::ResultValues
        ));
        limited = limits;
        limited.max_matrix_unknowns = 1;
        assert!(matches!(
            has_full_rank(2, REGULAR, limited, &NoAbort),
            Err(ConstraintError::ResourceLimit(error))
                if error.resource == ResourceKind::MatrixUnknowns
        ));
    }
}
