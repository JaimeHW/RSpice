use super::*;

mod audit;

pub(crate) struct PreparedTransition {
    size: usize,
    impulse_orders: usize,
    forms: Vec<ExactRow<Input>>,
    forcing_orders: Vec<usize>,
    retained_words: usize,
    audit_words: usize,
    constraints: ExactElimination<Input>,
    storage_constraints: ExactElimination<()>,
    a: Terms,
    e: Terms,
}

pub(crate) struct Transition {
    size: usize,
    impulse_orders: usize,
    values: Vec<Value>,
}

impl Transition {
    pub(crate) fn impulse_count(&self) -> usize {
        self.impulse_orders
    }
    pub(crate) fn value_count(&self) -> usize {
        self.values.len()
    }

    pub(crate) fn finite(&self) -> &[Value] {
        &self.values[..self.size]
    }
    pub(crate) fn rates(&self) -> &[Value] {
        &self.values[(self.impulse_orders + 1) * self.size..]
    }
    pub(crate) fn impulse(&self, order: usize) -> Option<&[Value]> {
        (order < self.impulse_orders)
            .then(|| &self.values[(order + 1) * self.size..(order + 2) * self.size])
    }
}

impl PreparedTransition {
    pub(crate) fn retained_words(&self) -> usize {
        self.retained_words
    }

    pub(crate) fn charge_from_coordinates(
        &self,
        coordinates: &[Value],
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>> {
        check_abort(abort)?;
        if coordinates.len() != self.size || coordinates.iter().any(|v| !v.is_finite()) {
            return Err(failure("invalid storage coordinates"));
        }
        ExactElimination::<Input>::ensure_words(
            self.retained_words
                .saturating_add(self.size.saturating_mul(2))
                .saturating_add(audit::SCRATCH_WORDS),
            limits.max_result_values,
        )?;
        let mut charges = Vec::with_capacity(self.size);
        for row in &self.e {
            check_abort(abort)?;
            let cancelled = std::cell::Cell::new(false);
            let value = rspice_veriloga_runtime::arithmetic::sum_products(
                row.iter()
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
                    .map(|&(column, coefficient)| (coordinates[column], coefficient)),
            );
            if cancelled.get() {
                return Err(ConstraintError::Aborted);
            }
            charges.push(value.map_err(|_| failure("unrepresentable incoming storage"))?);
        }
        Ok(charges)
    }

    pub(crate) fn new(
        size: usize,
        a: &[(usize, usize, Value)],
        e: &[(usize, usize, Value)],
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self> {
        let TransitionSystem {
            size,
            impulse_orders,
            overhead,
            a,
            e,
            algebraic,
            storage_constraints,
            mut projector,
        } = TransitionSystem::new(size, a, e, limits, 0, abort)?;
        let unknowns = projector.pivots.len() - 1;
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
            projector.reserve_retained_words(query.words().saturating_add(16))?;
            // Retain exact projection ratios. Rounding each weight before
            // cancellation can invent an action when incoming storage already
            // agrees with the outgoing source (for example C=2p, Rm=3).
            let bits = query
                .values
                .values()
                .chain(std::iter::once(&query.query))
                .map(BigInt::bits)
                .max()
                .unwrap_or(0);
            audit_words = audit_words.max(
                usize::try_from(
                    bits.saturating_add(2098 + u64::from(usize::BITS))
                        .div_ceil(64),
                )
                .unwrap_or(usize::MAX)
                .saturating_mul(16)
                .saturating_add(1024),
            );
            for input in query.values.keys() {
                check_abort(abort)?;
                if let Input::Forcing { row, order } = *input {
                    forcing_orders[row] = forcing_orders[row].max(order);
                }
            }
            forms.push(query);
        }
        let retained_words = overhead
            .saturating_add(algebraic.retained_words)
            .saturating_add(storage_constraints.retained_words)
            .saturating_add(forms.capacity().saturating_mul(16))
            .saturating_add(forcing_orders.capacity())
            .saturating_add(
                forms
                    .iter()
                    .fold(0usize, |sum, form| sum.saturating_add(form.words())),
            );
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
    pub(crate) fn evaluate(
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
            let mut numerator = BigInt::default();
            for (input, weight) in &form.values {
                check_abort(abort)?;
                let value = match *input {
                    Input::Storage(row) => storage[row],
                    Input::Forcing { row, order } => jets[row][order],
                };
                numerator += weight * integer_coefficient(value).unwrap();
            }
            check_abort(abort)?;
            // Inputs are exact integers in units of 2^-1074; divide only
            // after the complete signed sum, rounding the output once.
            let value = coefficient_ratio(&-numerator, &(&form.query << 1074usize))
                .ok_or_else(|| failure("unrepresentable transition evaluation"))?;
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
