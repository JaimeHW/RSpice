//! Distributional transitions of a regular constant linear descriptor.
//!
//! E*x' + A*x = b(t), with incoming charge/flux and a regular outgoing
//! forcing jet. All coefficients of delta and its derivatives remain separate
//! from finite coordinates. No integration interval regularizes an impulse.
use super::*;
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};

mod audit;
mod storage;

type Result<T> = std::result::Result<T, SimulationError>;
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

fn failure(detail: &str) -> SimulationError {
    SimulationError::Circuit(format!("linear descriptor transition: {detail}"))
}

fn check_abort(abort: &dyn AbortSignal) -> Result<()> {
    if abort.is_aborted() {
        Err(SimulationError::Aborted)
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

pub(super) struct PreparedTransition {
    size: usize,
    impulse_orders: usize,
    forms: Vec<Vec<(Input, Value)>>,
    forcing_orders: Vec<usize>,
    retained_words: usize,
    audit_words: usize,
    constraints: ExactElimination<Input>,
    storage_constraints: ExactElimination<()>,
    a: Terms,
    e: Terms,
}

pub(super) struct Transition {
    size: usize,
    impulse_orders: usize,
    values: Vec<Value>,
}

impl Transition {
    pub(super) fn finite(&self) -> &[Value] {
        &self.values[..self.size]
    }
    pub(super) fn rates(&self) -> &[Value] {
        &self.values[(self.impulse_orders + 1) * self.size..]
    }
    pub(super) fn impulse(&self, order: usize) -> Option<&[Value]> {
        (order < self.impulse_orders)
            .then(|| &self.values[(order + 1) * self.size..(order + 2) * self.size])
    }
}

impl PreparedTransition {
    pub(super) fn new(
        size: usize,
        a: &[(usize, usize, Value)],
        e: &[(usize, usize, Value)],
        limits: ResourceLimits,
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
            .saturating_add(a.len().saturating_add(e.len()).saturating_mul(16));
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
        let mut forms = Vec::with_capacity(unknowns);
        let mut forcing_orders = vec![0; size];
        let mut audit_words = audit::SCRATCH_WORDS;
        let algebraic_bits = algebraic.rows.iter().map(|(_, row)| {
            row.nodes
                .values()
                .chain(row.values.values())
                .map(BigInt::bits)
                .max()
                .unwrap_or(0)
        });
        let storage_bits = storage_constraints
            .rows
            .iter()
            .map(|(_, row)| row.nodes.values().map(BigInt::bits).max().unwrap_or(0));
        for bits in algebraic_bits.chain(storage_bits) {
            check_abort(abort)?;
            audit_words = audit_words.max(
                usize::try_from(bits.div_ceil(64))
                    .unwrap_or(usize::MAX)
                    .saturating_mul(16)
                    .saturating_add(1024),
            );
        }
        for (_, row) in &algebraic.rows {
            for input in row.values.keys() {
                if let Input::Forcing { row, order } = input {
                    forcing_orders[*row] = forcing_orders[*row].max(order + 1);
                }
            }
        }
        for coordinate in 1..=unknowns {
            check_abort(abort)?;
            let mut query = ExactRow {
                query: BigInt::from(1),
                ..ExactRow::default()
            };
            query.nodes.insert(coordinate, BigInt::from(1));
            projector.reduce(&mut query, abort)?;
            if !query.nodes.is_empty() {
                return Err(failure("incomplete transition mapping"));
            }
            projector.check_cost(query.words().saturating_mul(3))?;
            projector.reserve_retained_words(query.values.len().saturating_mul(6))?;
            let mut form = Vec::with_capacity(query.values.len());
            for (input, value) in query.values {
                check_abort(abort)?;
                if let Input::Forcing { row, order } = input {
                    forcing_orders[row] = forcing_orders[row].max(order);
                }
                let weight = -coefficient_ratio(&value, &query.query)
                    .ok_or_else(|| failure("transition projection exceeds finite precision"))?;
                form.push((input, weight));
            }
            forms.push(form);
        }
        let retained_words = overhead
            .saturating_add(algebraic.retained_words)
            .saturating_add(storage_constraints.retained_words)
            .saturating_add(forms.capacity().saturating_mul(4))
            .saturating_add(forcing_orders.capacity())
            .saturating_add(forms.iter().fold(0usize, |sum, form| {
                sum.saturating_add(form.capacity().saturating_mul(4))
            }));
        ExactElimination::<Input>::ensure_words(retained_words, limits.max_result_values)?;
        Ok(Self {
            size,
            impulse_orders,
            forms,
            forcing_orders,
            retained_words,
            audit_words,
            constraints: algebraic,
            storage_constraints,
            a,
            e,
        })
    }

    /// Use accepted nodal charge and branch flux, including authored per-device
    /// startup ICs. No arbitrary finite voltage/current guess replaces storage.
    pub(super) fn evaluate(
        &self,
        storage: &[Value],
        mut forcing: impl FnMut(usize, usize) -> Result<Value>,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Transition> {
        check_abort(abort)?;
        if storage.len() != self.size || storage.iter().any(|value| !value.is_finite()) {
            return Err(failure("invalid incoming charge or flux"));
        }
        ResourceLimitError::ensure(
            ResourceKind::MatrixUnknowns,
            self.forms.len(),
            limits.max_matrix_unknowns,
        )?;
        let jet_values = self.forcing_orders.iter().fold(0usize, |sum, order| {
            sum.saturating_add(order.saturating_add(1))
        });
        ExactElimination::<Input>::ensure_words(
            self.retained_words
                .saturating_add(jet_values.saturating_mul(2))
                .saturating_add(self.size.saturating_mul(4))
                .saturating_add(self.forms.len().saturating_mul(2))
                .saturating_add(self.audit_words),
            limits.max_result_values,
        )?;
        self.audit_storage(storage, abort)?;
        let mut jets = Vec::with_capacity(self.size);
        for (row, &last_order) in self.forcing_orders.iter().enumerate() {
            let mut jet = Vec::with_capacity(last_order + 1);
            for order in 0..=last_order {
                check_abort(abort)?;
                let value = forcing(row, order)?;
                if !value.is_finite() {
                    return Err(failure("nonfinite outgoing forcing derivative"));
                }
                jet.push(value);
            }
            jets.push(jet);
        }
        let mut values = Vec::with_capacity(self.forms.len());
        for form in &self.forms {
            check_abort(abort)?;
            let cancelled = std::cell::Cell::new(false);
            let value = rspice_veriloga_runtime::arithmetic::sum_products(
                form.iter()
                    .take_while(|_| {
                        if cancelled.get() {
                            return false;
                        }
                        if abort.is_aborted() {
                            cancelled.set(true);
                            return false;
                        }
                        true
                    })
                    .map(|&(input, weight)| {
                        let value = match input {
                            Input::Storage(row) => storage[row],
                            Input::Forcing { row, order } => jets[row][order],
                        };
                        (value, weight)
                    }),
            );
            if cancelled.get() {
                return Err(SimulationError::Aborted);
            }
            let value = value.map_err(|_| failure("unrepresentable transition evaluation"))?;
            if !value.is_finite() {
                return Err(failure("nonfinite transition evaluation"));
            }
            values.push(value);
        }
        let transition = Transition {
            size: self.size,
            impulse_orders: self.impulse_orders,
            values,
        };
        self.audit(storage, &jets, &transition, abort)?;
        Ok(transition)
    }
}

#[cfg(test)]
mod tests;
