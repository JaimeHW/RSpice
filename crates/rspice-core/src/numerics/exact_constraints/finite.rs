//! Finite dynamics of a homogeneous constant descriptor, with exact rank.
//!
//! Algebraic closure removes infinite chains before any floating-point
//! eigensolve. The surviving coordinates obey x' = A*x. No small-pivot or
//! frequency threshold decides which coordinates survive.
use super::*;
use crate::resource::ResourceLimits;

#[derive(Debug, thiserror::Error)]
pub(crate) enum FiniteDescriptorError {
    #[error(transparent)]
    Constraint(#[from] ConstraintError),
    #[error("the descriptor pencil is irregular")]
    Irregular,
}

pub(crate) struct FiniteDynamics {
    pub(crate) matrix: Vec<Vec<Value>>,
    /// Residual of G*T + C*T*A, where x=T*q spans the finite subspace.
    pub(crate) projection_error: Value,
    /// Strict Hurwitz classification before the exact finite equations round.
    pub(crate) asymptotically_stable: Option<bool>,
}

type Reducer = ExactElimination<usize>;
type Row = ExactRow<usize>;

fn invalid(message: &str) -> FiniteDescriptorError {
    ConstraintError::Invalid(message.to_owned()).into()
}

fn check_abort(abort: &dyn AbortSignal) -> Result<(), ConstraintError> {
    if abort.is_aborted() {
        Err(ConstraintError::Aborted)
    } else {
        Ok(())
    }
}

fn project_equation(
    mut row: Row,
    reducer: &Reducer,
    abort: &dyn AbortSignal,
) -> Result<Row, FiniteDescriptorError> {
    reducer.reduce(&mut row, abort)?;
    if !row.nodes.is_empty() {
        return Err(FiniteDescriptorError::Irregular);
    }
    Ok(row)
}

fn rounded_projection(row: &Row, count: usize) -> Result<Vec<Value>, FiniteDescriptorError> {
    let mut values = vec![0.0; count];
    for (&index, value) in &row.values {
        values[index] = -coefficient_ratio(value, &row.query)
            .ok_or_else(|| invalid("finite descriptor coefficient is not representable"))?;
    }
    Ok(values)
}

/// Input matrices and all exact/floating workspace share the owner's limit.
pub(crate) fn finite_dynamics(
    g: &[Vec<Value>],
    c: &[Vec<Value>],
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
    certify_stability: bool,
) -> Result<FiniteDynamics, FiniteDescriptorError> {
    check_abort(abort)?;
    let n = g.len();
    if n == 0 || c.len() != n || g.iter().chain(c).any(|row| row.len() != n) {
        return Err(invalid("invalid finite descriptor dimensions"));
    }
    // Matrices, row/vector headers, pivot maps, projections and their working
    // copies. Integer coefficients and multiplication temporaries are charged
    // separately by the shared elimination kernel.
    let overhead = n
        .saturating_mul(n)
        .saturating_mul(8)
        .saturating_add(n.saturating_mul(64))
        .saturating_add(16);
    Reducer::ensure_words(overhead, limits.max_result_values)?;
    let mut rows = Vec::with_capacity(n);
    let mut words = overhead;
    for (g_row, c_row) in g.iter().zip(c) {
        let mut row = Row::default();
        for (column, &value) in g_row.iter().chain(c_row).enumerate() {
            check_abort(abort)?;
            if value == 0.0 {
                continue;
            }
            Reducer::ensure_words(words.saturating_add(160), limits.max_result_values)?;
            let value = integer_coefficient(value)
                .ok_or_else(|| invalid("nonfinite descriptor coefficient"))?;
            words = words.saturating_add(16 + value.bits().div_ceil(64) as usize);
            row.nodes.insert(column + 1, value);
        }
        words = words.saturating_add(16); // zero query coordinate
        rows.push(row);
    }
    let mut algebraic =
        close_descriptor::<_, ConstraintError>(n, rows, limits, overhead, abort, Ok, |_| {
            Ok(ConstraintDisposition::RetainAtOwner)
        })?
        .expect("homogeneous closure never defers");
    let free: Vec<_> = (1..=n)
        .filter(|&node| algebraic.pivots[node].is_none())
        .collect();
    if free.is_empty() {
        // Full-rank homogeneous algebraic closure proves a regular pencil
        // with no finite coordinates, including higher-index infinite chains.
        return Ok(FiniteDynamics {
            matrix: Vec::new(),
            projection_error: 0.0,
            asymptotically_stable: certify_stability.then_some(true),
        });
    }
    let mut rate_limits = limits;
    rate_limits.max_result_values = limits
        .max_result_values
        .saturating_sub(overhead.saturating_add(algebraic.retained_words));
    let mut rates = Reducer::new(n + 1, rate_limits)?;
    // Derivatives obey every homogeneous algebraic constraint, including the
    // hidden constraints exposed by differentiated descriptor closure.
    for (_, row) in &algebraic.rows {
        rates.check_cost(row.words().saturating_mul(3))?;
        rates.admit(row.clone(), 1, abort)?;
    }
    for (g_row, c_row) in g.iter().zip(c) {
        let mut row = Row::default();
        for (column, (&static_value, &dynamic_value)) in g_row.iter().zip(c_row).enumerate() {
            check_abort(abort)?;
            rates.check_cost(row.words().saturating_add(320))?;
            if dynamic_value != 0.0 {
                row.nodes
                    .insert(column + 1, integer_coefficient(dynamic_value).unwrap());
            }
            if static_value != 0.0 {
                row.values
                    .insert(column + 1, integer_coefficient(static_value).unwrap());
            }
        }
        if let Some(remainder) = rates.admit(row, 1, abort)? {
            let mut equation = Row {
                nodes: remainder.values,
                ..Row::default()
            };
            algebraic.limits.max_result_values = limits
                .max_result_values
                .saturating_sub(overhead.saturating_add(rates.retained_words));
            algebraic.reduce(&mut equation, abort)?;
            if !equation.nodes.is_empty() {
                return Err(FiniteDescriptorError::Irregular);
            }
        }
    }
    if rates.rows.len() != n {
        return Err(FiniteDescriptorError::Irregular);
    }
    algebraic.limits.max_result_values = limits
        .max_result_values
        .saturating_sub(overhead.saturating_add(rates.retained_words));
    for (state, &node) in free.iter().enumerate() {
        let row = Row {
            nodes: BTreeMap::from([(node, BigInt::from(1))]),
            values: BTreeMap::from([(state, BigInt::from(-1))]),
            query: BigInt::default(),
        };
        algebraic.admit(row, 1, abort)?;
    }
    rates.limits.max_result_values = limits
        .max_result_values
        .saturating_sub(overhead.saturating_add(algebraic.retained_words));
    let mut matrix = Vec::with_capacity(free.len());
    let mut certificate_rows = Vec::new();
    let mut certificate_words = 0usize;
    for &node in &free {
        rates.limits.max_result_values = limits.max_result_values.saturating_sub(
            overhead
                .saturating_add(algebraic.retained_words)
                .saturating_add(certificate_words),
        );
        algebraic.limits.max_result_values = limits.max_result_values.saturating_sub(
            overhead
                .saturating_add(rates.retained_words)
                .saturating_add(certificate_words),
        );
        let mut query = Row {
            nodes: BTreeMap::from([(node, BigInt::from(-1))]),
            query: BigInt::from(1),
            ..Row::default()
        };
        rates.reduce(&mut query, abort)?;
        if !query.nodes.is_empty() {
            return Err(FiniteDescriptorError::Irregular);
        }
        // The rate equation's symbolic forcing is the original x vector.
        // Substitute its exact finite-state expansion before rounding once.
        query.nodes = std::mem::take(&mut query.values);
        let projected = project_equation(query, &algebraic, abort)?;
        matrix.push(rounded_projection(&projected, free.len())?);
        if certify_stability {
            certificate_words = certificate_words.saturating_add(projected.words());
            Reducer::ensure_words(
                overhead
                    .saturating_add(algebraic.retained_words)
                    .saturating_add(rates.retained_words)
                    .saturating_add(certificate_words),
                limits.max_result_values,
            )?;
            certificate_rows.push(projected);
        }
    }
    let asymptotically_stable = if certify_stability {
        let mut certificate_limits = limits;
        certificate_limits.max_result_values = limits.max_result_values.saturating_sub(
            overhead
                .saturating_add(algebraic.retained_words)
                .saturating_add(rates.retained_words),
        );
        Some(
            super::hurwitz::rational_rows_are_hurwitz(&certificate_rows, certificate_limits, abort)
                .map_err(|mut error| {
                    if let ConstraintError::ResourceLimit(resource) = &mut error
                        && resource.resource == crate::resource::ResourceKind::ResultValues
                    {
                        resource.requested = resource.requested.saturating_add(
                            limits.max_result_values - certificate_limits.max_result_values,
                        );
                        resource.limit = limits.max_result_values;
                    }
                    error
                })?,
        )
    } else {
        None
    };
    drop(certificate_rows);
    algebraic.limits.max_result_values = limits
        .max_result_values
        .saturating_sub(overhead.saturating_add(rates.retained_words));
    let mut basis = Vec::with_capacity(n);
    for node in 1..=n {
        let query = Row {
            nodes: BTreeMap::from([(node, BigInt::from(-1))]),
            query: BigInt::from(1),
            ..Row::default()
        };
        basis.push(rounded_projection(
            &project_equation(query, &algebraic, abort)?,
            free.len(),
        )?);
    }
    // Verify the projected finite invariant subspace against the original
    // binary64 pencil. Scale before products to avoid gratuitous overflow.
    let pencil_scale = g
        .iter()
        .chain(c)
        .flatten()
        .map(|v| v.abs())
        .fold(0.0_f64, Value::max);
    let basis_scale = basis
        .iter()
        .flatten()
        .map(|v| v.abs())
        .fold(1.0_f64, Value::max);
    let a_scale = matrix
        .iter()
        .flatten()
        .map(|v| v.abs())
        .fold(1.0_f64, Value::max);
    let norm = |rows: &[Vec<Value>], scale: Value| {
        rows.iter()
            .flatten()
            .fold(0.0_f64, |norm, value| norm.hypot(value / scale))
    };
    let denominator = (norm(g, pencil_scale) / a_scale
        + norm(c, pencil_scale) * norm(&matrix, a_scale))
        * norm(&basis, basis_scale);
    let mut residual_norm = 0.0_f64;
    let mut ct = vec![0.0; free.len()];
    for row in 0..n {
        check_abort(abort)?;
        for (state, value) in ct.iter_mut().enumerate() {
            *value = (0..n)
                .map(|column| {
                    (c[row][column] / pencil_scale) * (basis[column][state] / basis_scale)
                })
                .sum();
        }
        for state in 0..free.len() {
            check_abort(abort)?;
            let gt: Value = (0..n)
                .map(|column| {
                    (g[row][column] / pencil_scale) * (basis[column][state] / basis_scale) / a_scale
                })
                .sum();
            let cta: Value = ct
                .iter()
                .enumerate()
                .map(|(inner, value)| value * (matrix[inner][state] / a_scale))
                .sum();
            residual_norm = residual_norm.hypot(gt + cta);
        }
    }
    let projection_error = if denominator == 0.0 && residual_norm == 0.0 {
        0.0
    } else {
        residual_norm / denominator
    };
    check_abort(abort)?;
    Ok(FiniteDynamics {
        matrix,
        projection_error,
        asymptotically_stable,
    })
}
