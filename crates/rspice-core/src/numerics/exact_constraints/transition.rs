//! Distributional transitions of a regular constant linear descriptor.
//!
//! E*x' + A*x = b(t), with incoming charge/flux and a regular outgoing
//! forcing jet. All coefficients of delta and its derivatives remain separate
//! from finite coordinates. No integration interval regularizes an impulse.
use super::*;
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};

mod storage;

type Result<T> = std::result::Result<T, ConstraintError>;
type Terms = Vec<Vec<(usize, Value)>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Input {
    Storage(usize),
    Forcing { row: usize, order: usize },
}

impl Input {
    fn differentiated(self) -> Result<Self> {
        match self {
            Self::Forcing { row, order } => Ok(Self::Forcing {
                row,
                order: order
                    .checked_add(1)
                    .ok_or_else(|| failure("forcing order overflow"))?,
            }),
            Self::Storage(_) => Err(failure("incoming storage is not prescribed forcing")),
        }
    }
}

fn failure(detail: &str) -> ConstraintError {
    ConstraintError::Invalid(format!("linear descriptor transition: {detail}"))
}

fn check_abort(abort: &dyn AbortSignal) -> Result<()> {
    if abort.is_aborted() {
        Err(ConstraintError::Aborted)
    } else {
        Ok(())
    }
}

fn dimension(value: Option<usize>) -> Result<usize> {
    value.ok_or_else(|| failure("descriptor dimensions overflow"))
}

fn terms(size: usize, entries: &[(usize, usize, Value)], abort: &dyn AbortSignal) -> Result<Terms> {
    let mut rows = vec![Vec::new(); size];
    for &(row, column, value) in entries {
        check_abort(abort)?;
        if row >= size || column >= size || !value.is_finite() {
            return Err(failure("invalid authored matrix coefficient"));
        }
        if value != 0.0 {
            rows[row].push((column, value));
        }
    }
    Ok(rows)
}

fn add(
    reducer: &ExactElimination<Input>,
    row: &mut ExactRow<Input>,
    entries: &[(usize, Value)],
    offset: usize,
    abort: &dyn AbortSignal,
) -> Result<()> {
    for &(column, coefficient) in entries {
        check_abort(abort)?;
        reducer.check_cost(row.words().saturating_mul(3).saturating_add(160))?;
        let value =
            integer_coefficient(coefficient).ok_or_else(|| failure("nonfinite coefficient"))?;
        ExactElimination::<Input>::add_integer(&mut row.nodes, offset + column + 1, value);
    }
    Ok(())
}

fn admit(
    reducer: &mut ExactElimination<Input>,
    storage: &mut ExactElimination<()>,
    row: ExactRow<Input>,
    abort: &dyn AbortSignal,
) -> Result<()> {
    if let Some(row) = reducer.admit(row, 1, abort)? {
        if !row.nodes.is_empty() {
            return Err(failure("inconsistent or singular transition descriptor"));
        }
        // A charge vector is restricted to range(E). Redundant jump rows
        // may therefore leave a storage identity instead of literal zero.
        let mut constraint = storage::constraint(row)?;
        storage.limits.max_result_values = reducer.limits.max_result_values.saturating_sub(
            reducer
                .retained_words
                .saturating_sub(storage.retained_words),
        );
        storage.reduce(&mut constraint, abort)?;
        if !constraint.nodes.is_empty() {
            return Err(failure(
                "transition imposes an unexplained storage constraint",
            ));
        }
    }
    Ok(())
}

/// Exact finite, impulse and rate equations before projection into binary64.
struct TransitionSystem {
    size: usize,
    impulse_orders: usize,
    overhead: usize,
    a: Terms,
    e: Terms,
    algebraic: ExactElimination<Input>,
    storage_constraints: ExactElimination<()>,
    projector: ExactElimination<Input>,
}

impl TransitionSystem {
    fn new(
        size: usize,
        a: &[(usize, usize, Value)],
        e: &[(usize, usize, Value)],
        limits: ResourceLimits,
        caller_retained_words: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Self> {
        check_abort(abort)?;
        if size == 0 {
            return Err(failure("empty descriptor"));
        }
        let closure_columns = dimension(size.checked_mul(2).and_then(|n| n.checked_add(1)))?;
        ResourceLimitError::ensure(
            ResourceKind::MatrixUnknowns,
            size,
            limits.max_matrix_unknowns,
        )?;
        let overhead = size
            .saturating_mul(64)
            .saturating_add(a.len().saturating_add(e.len()).saturating_mul(16))
            .saturating_add(caller_retained_words);
        ExactElimination::<Input>::ensure_words(overhead, limits.max_result_values)?;
        let a = terms(size, a, abort)?;
        let e = terms(size, e, abort)?;
        let mut accounting = ExactElimination::<Input>::new(closure_columns, limits)?;
        accounting.reserve_retained_words(overhead)?;
        let mut rows = Vec::with_capacity(size);
        for index in 0..size {
            check_abort(abort)?;
            let mut row = ExactRow::default();
            add(&accounting, &mut row, &a[index], 0, abort)?;
            add(&accounting, &mut row, &e[index], size, abort)?;
            accounting.check_cost(row.words().saturating_mul(3).saturating_add(160))?;
            row.values.insert(
                Input::Forcing {
                    row: index,
                    order: 0,
                },
                integer_coefficient(1.0).unwrap(),
            );
            accounting.reserve_retained_words(row.words())?;
            rows.push(row);
        }
        drop(accounting);
        let algebraic = close_descriptor(
            size,
            rows,
            limits,
            overhead,
            abort,
            Input::differentiated,
            |_| Err(failure("singular original descriptor")),
        )?
        .ok_or_else(|| failure("descriptor closure was deferred"))?;
        let impulse_orders = algebraic
            .rows
            .iter()
            .flat_map(|(_, row)| row.values.keys())
            .filter_map(|input| {
                if let Input::Forcing { order, .. } = input {
                    Some(*order)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);
        let blocks = dimension(impulse_orders.checked_add(2))?;
        let unknowns = dimension(size.checked_mul(blocks))?;
        ResourceLimitError::ensure(
            ResourceKind::MatrixUnknowns,
            unknowns,
            limits.max_matrix_unknowns,
        )?;
        let mut storage_constraints = storage::prepare(
            &e,
            limits,
            overhead.saturating_add(algebraic.retained_words),
            abort,
        )?;
        let mut projector =
            ExactElimination::<Input>::new(dimension(unknowns.checked_add(1))?, limits)?;
        projector.reserve_retained_words(
            overhead
                .saturating_add(algebraic.retained_words)
                .saturating_add(unknowns.saturating_mul(16))
                .saturating_add(storage_constraints.retained_words),
        )?;
        let rate_offset = (impulse_orders + 1) * size;
        for (_, row) in &algebraic.rows {
            check_abort(abort)?;
            projector.check_cost(row.words().saturating_mul(4))?;
            admit(&mut projector, &mut storage_constraints, row.clone(), abort)?;
            let derivative = ExactRow {
                nodes: row
                    .nodes
                    .iter()
                    .map(|(&column, value)| (rate_offset + column, value.clone()))
                    .collect(),
                values: row
                    .values
                    .iter()
                    .map(|(&input, value)| Ok((input.differentiated()?, value.clone())))
                    .collect::<Result<_>>()?,
                query: BigInt::default(),
            };
            admit(&mut projector, &mut storage_constraints, derivative, abort)?;
        }
        for index in 0..size {
            check_abort(abort)?;
            // Integrated original equation: E*x+ + A*xi_0 = q-.
            let mut jump = ExactRow::default();
            add(&projector, &mut jump, &e[index], 0, abort)?;
            jump.values
                .insert(Input::Storage(index), integer_coefficient(1.0).unwrap());
            if impulse_orders != 0 {
                add(&projector, &mut jump, &a[index], size, abort)?;
            }
            admit(&mut projector, &mut storage_constraints, jump, abort)?;
            // Delta-derivative coefficients: E*xi_j + A*xi_(j+1) = 0,
            // terminated by E*xi_last = 0. There is no hidden cutoff.
            for order in 0..impulse_orders {
                let mut coefficient = ExactRow::default();
                add(
                    &projector,
                    &mut coefficient,
                    &e[index],
                    (order + 1) * size,
                    abort,
                )?;
                if order + 1 < impulse_orders {
                    add(
                        &projector,
                        &mut coefficient,
                        &a[index],
                        (order + 2) * size,
                        abort,
                    )?;
                }
                admit(&mut projector, &mut storage_constraints, coefficient, abort)?;
            }
            let mut finite = ExactRow::default();
            add(&projector, &mut finite, &a[index], 0, abort)?;
            add(&projector, &mut finite, &e[index], rate_offset, abort)?;
            projector.check_cost(finite.words().saturating_mul(3).saturating_add(160))?;
            finite.values.insert(
                Input::Forcing {
                    row: index,
                    order: 0,
                },
                integer_coefficient(1.0).unwrap(),
            );
            admit(&mut projector, &mut storage_constraints, finite, abort)?;
        }
        if projector.rows.len() != unknowns {
            return Err(failure("undetermined finite or impulsive coordinate"));
        }
        Ok(Self {
            size,
            impulse_orders,
            overhead,
            a,
            e,
            algebraic,
            storage_constraints,
            projector,
        })
    }
}

// The physical event evaluator and its audits remain test-gated until circuit
// integration owns all required accepted state. PZ consumes the exact system.
#[cfg(test)]
mod prepared;
