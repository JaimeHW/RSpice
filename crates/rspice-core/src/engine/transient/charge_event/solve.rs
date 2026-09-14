use super::*;

impl ChargeEventTopology {
    /// Solve the outgoing charge constraints and its finite rate/current
    /// equations at one physical time. The sampler operates on private model
    /// scratch and a fixed, independently solved incoming history endpoint.
    pub(in crate::engine::transient) fn solve(
        &self,
        incoming: &[Value],
        incoming_q: &[Value],
        options: &EventOptions,
        abort: &dyn AbortSignal,
        mut sample: impl FnMut(&[Value], &dyn AbortSignal) -> Result<EventSample>,
    ) -> Result<ChargeEventState> {
        check_abort(abort)?;
        options.validate()?;
        ResourceLimitError::ensure(
            ResourceKind::MatrixUnknowns,
            self.size,
            options.limits.max_matrix_unknowns,
        )?;
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            self.size.saturating_mul(64),
            options.limits.max_result_values,
        )?;
        if incoming.len() != self.size
            || incoming_q.len() != self.size
            || !incoming
                .iter()
                .chain(incoming_q)
                .all(|value| value.is_finite())
        {
            return Err(error("invalid incoming physical state/charge"));
        }
        if incoming_q[self.nodes..].iter().any(|value| *value != 0.0) {
            return Err(error(
                "incoming non-nodal storage requires a flux/descriptor event operator",
            ));
        }
        let mut trial = self.physical_probe(incoming);
        for iteration in 0..options.iterations {
            check_abort(abort)?;
            let physical = sample(&self.physical_probe(&trial), abort)?;
            let equations = self.jump_equations(&trial, incoming_q, &physical, options, abort)?;
            // Factor even an exactly zero residual: a singular system must
            // not certify an arbitrary unconstrained initial guess.
            let correction = equations.solve(options, abort)?;
            let residual = equations.norm(options)?;
            let mut update: Value = 0.0;
            for (column, (&value, &change)) in trial.iter().zip(&correction).enumerate() {
                let absolute = if self.source_columns[column] {
                    options.charge_tolerance
                } else if column < self.nodes {
                    options.voltage_tolerance
                } else {
                    options.current_tolerance
                };
                let scale = absolute + options.relative_tolerance * value.abs();
                update = update.max(change.abs() / scale);
            }
            if residual <= 1.0 && update <= 1.0 {
                self.audit_charge(&trial, incoming_q, &physical, options, abort)?;
                return self.finish(trial, physical, iteration + 1, options, abort);
            }
            let mut accepted = None;
            let mut alpha: Value = 1.0;
            for _ in 0..options.backtracks {
                check_abort(abort)?;
                let next: Vec<_> = trial
                    .iter()
                    .zip(&correction)
                    .map(|(&value, &change)| alpha.mul_add(change, value))
                    .collect();
                if next.iter().all(|value| value.is_finite()) {
                    let next_sample = sample(&self.physical_probe(&next), abort)?;
                    if next_sample.nonfinite(self.size)? {
                        alpha *= 0.5;
                        continue;
                    }
                    let next_equations =
                        self.jump_equations(&next, incoming_q, &next_sample, options, abort)?;
                    let next_norm = next_equations.norm(options)?;
                    if next_norm < residual || (next_norm <= 1.0 && update * alpha <= 1.0) {
                        accepted = Some(next);
                        break;
                    }
                }
                alpha *= 0.5;
            }
            trial = accepted.ok_or_else(|| {
                error("Newton line search did not reduce the physical event residual")
            })?;
        }
        Err(SimulationError::ConvergenceFailed(options.iterations))
    }

    fn audit_charge(
        &self,
        trial: &[Value],
        incoming_q: &[Value],
        physical: &EventSample,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<()> {
        for (row, &old_charge) in incoming_q.iter().take(self.nodes).enumerate() {
            if row % 64 == 0 {
                check_abort(abort)?;
            }
            let impulses = self.source_incidence[row]
                .iter()
                .map(|&(column, sign)| (trial[column], sign));
            let residual = sum([(physical.q.values[row], 1.0), (old_charge, -1.0)]
                .into_iter()
                .chain(impulses))?;
            let tolerance = options.charge_tolerance
                + options.relative_tolerance * physical.q.scales[row].max(old_charge.abs());
            if residual.abs() > tolerance {
                return Err(error(format!(
                    "nodal charge conservation failed at row {row}"
                )));
            }
        }
        Ok(())
    }

    fn finish(
        &self,
        mut trial: Vec<Value>,
        physical: EventSample,
        iterations: usize,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<ChargeEventState> {
        let equations = self.rate_equations(&physical, options, abort)?;
        let rates = equations.solve(options, abort)?;
        // Audit the differentiated constraints in their own units. No time
        // interval converts an amp/volt floor into an invented rate tolerance.
        for row in 0..self.size {
            if row % 64 == 0 {
                check_abort(abort)?;
            }
            let products = equations.rows[row]
                .iter()
                .map(|&(column, value)| (value, rates[column]));
            let residual = sum(products.clone().chain([(equations.values[row], 1.0)]))?;
            let scale = products.fold(equations.values[row].abs(), |old, (a, b)| {
                old.max((a * b).abs())
            });
            let tolerance = equations.absolute[row] + options.relative_tolerance * scale;
            if !tolerance.is_finite() || residual.abs() > tolerance {
                return Err(error(format!("finite-rate equation failed at row {row}")));
            }
        }
        // Recheck every original KCL row, including the component row that
        // the square rate system replaced by differentiated algebraic KCL.
        for row in 0..self.nodes {
            if row % 64 == 0 {
                check_abort(abort)?;
            }
            let current = sum(physical.q.rows[row]
                .iter()
                .map(|&(column, value)| (value, rates[column])))?;
            let injections = self.source_incidence[row]
                .iter()
                .map(|&(column, sign)| (rates[column], sign));
            let residual = sum([
                (physical.f.values[row], 1.0),
                (physical.q_time[row], 1.0),
                (current, 1.0),
            ]
            .into_iter()
            .chain(injections))?;
            let scale = physical.f.scales[row]
                .max(physical.q_time[row].abs())
                .max(current.abs());
            if residual.abs() > options.current_tolerance + options.relative_tolerance * scale {
                return Err(error(format!("finite-current KCL failed at row {row}")));
            }
        }
        let source_impulses = self
            .sources
            .iter()
            .map(|source| trial[source.branch])
            .collect();
        for source in &self.sources {
            trial[source.branch] = rates[source.branch];
        }
        let coordinate_rates = rates
            .into_iter()
            .enumerate()
            .map(|(column, rate)| (!self.source_columns[column]).then_some(rate))
            .collect();
        Ok(ChargeEventState {
            solution: trial,
            source_impulses,
            coordinate_rates,
            iterations,
        })
    }
}
