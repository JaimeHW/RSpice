use super::*;

#[derive(Clone, Copy)]
enum CoordinatePolicy<'a> {
    Project,
    Continuous,
    OperatingPoint,
    Integrated(&'a [Value]),
}

#[derive(Clone, Copy)]
struct RateReference<'a> {
    solution: &'a [Value],
    // None is an authenticated static balance. Some is an integration
    // predictor which must fit the original physical equation budgets.
    storage_currents: Option<&'a [Value]>,
}

impl ChargeEventTopology {
    /// Solve the outgoing charge/flux constraints and its finite rate/current
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
        self.solve_coordinates(
            (incoming, incoming_q, CoordinatePolicy::Project),
            options,
            abort,
            &mut sample,
        )
    }

    /// Audit an invariant physical coordinate limit without projecting it to
    /// a nearby Newton solution. The caller must establish unchanged jump
    /// constraints and a C1 constitutive domain independently. Factorization,
    /// residual/update checks and all original storage/KCL audits still run.
    /// Source branch slots become finite outgoing currents only in finish.
    pub(in crate::engine::transient) fn solve_continuous(
        &self,
        incoming: &[Value],
        incoming_q: &[Value],
        options: &EventOptions,
        abort: &dyn AbortSignal,
        mut sample: impl FnMut(&[Value], &dyn AbortSignal) -> Result<EventSample>,
    ) -> Result<ChargeEventState> {
        self.solve_coordinates(
            (incoming, incoming_q, CoordinatePolicy::Continuous),
            options,
            abort,
            &mut sample,
        )
    }

    /// Initialize rates from an authenticated, unchanged operating point.
    /// Its static balance is an accepted equation, not a tiny excitation to
    /// divide by a potentially much smaller capacitance. Only rate RHS values
    /// use that balance; original charge, rank, KCL and flux audits remain.
    /// The caller must separately exclude released ICs and changed forcing.
    pub(in crate::engine::transient) fn solve_operating_point(
        &self,
        incoming: &[Value],
        incoming_q: &[Value],
        options: &EventOptions,
        abort: &dyn AbortSignal,
        mut sample: impl FnMut(&[Value], &dyn AbortSignal) -> Result<EventSample>,
    ) -> Result<ChargeEventState> {
        self.solve_coordinates(
            (incoming, incoming_q, CoordinatePolicy::OperatingPoint),
            options,
            abort,
            &mut sample,
        )
    }

    /// Retain a continuous incoming state and prefer its integration current
    /// reference wherever the original physical equations permit it. This
    /// prevents residual cancellation from exciting tiny storage modes while
    /// retaining real finite currents, source slopes, and every storage row.
    /// The caller must establish unchanged authored and delayed forcing.
    pub(in crate::engine::transient) fn solve_integrated(
        &self,
        incoming: (&[Value], &[Value]),
        currents: &[Value],
        options: &EventOptions,
        abort: &dyn AbortSignal,
        mut sample: impl FnMut(&[Value], &dyn AbortSignal) -> Result<EventSample>,
    ) -> Result<ChargeEventState> {
        check_abort(abort)?;
        if currents.len() != self.size {
            return Err(error("invalid integrated event current population"));
        }
        for (index, value) in currents.iter().enumerate() {
            if index.is_multiple_of(64) {
                check_abort(abort)?;
            }
            if !value.is_finite() {
                return Err(error("nonfinite integrated event current"));
            }
        }
        self.solve_coordinates(
            (
                incoming.0,
                incoming.1,
                CoordinatePolicy::Integrated(currents),
            ),
            options,
            abort,
            &mut sample,
        )
    }

    fn solve_coordinates(
        &self,
        coordinates: (&[Value], &[Value], CoordinatePolicy<'_>),
        options: &EventOptions,
        abort: &dyn AbortSignal,
        sample: &mut impl FnMut(&[Value], &dyn AbortSignal) -> Result<EventSample>,
    ) -> Result<ChargeEventState> {
        with_retained_values(
            options,
            self.weighted
                .as_ref()
                .map_or(0, |basis| basis.retained_values),
            |bounded| self.solve_coordinates_inner(coordinates, bounded, abort, sample),
        )
    }

    fn solve_coordinates_inner(
        &self,
        coordinates: (&[Value], &[Value], CoordinatePolicy<'_>),
        options: &EventOptions,
        abort: &dyn AbortSignal,
        sample: &mut impl FnMut(&[Value], &dyn AbortSignal) -> Result<EventSample>,
    ) -> Result<ChargeEventState> {
        let (incoming, incoming_q, policy) = coordinates;
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
        if incoming_q[self.nodes..]
            .iter()
            .zip(&self.branch_equations)
            .any(|(&value, row)| value != 0.0 && row.flux_tolerance().is_none())
        {
            return Err(error(
                "incoming non-nodal storage has no prepared flux equation",
            ));
        }
        let mut trial = self.physical_probe(incoming);
        if matches!(policy, CoordinatePolicy::Project) {
            self.project_current_controlled_voltage_seed(incoming, &mut trial, abort)?;
        }
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
                self.audit_storage(&trial, incoming_q, &physical, options, abort)?;
                return self.finish(
                    trial,
                    physical,
                    iteration + 1,
                    options,
                    abort,
                    match policy {
                        CoordinatePolicy::OperatingPoint => Some(RateReference {
                            solution: incoming,
                            storage_currents: None,
                        }),
                        CoordinatePolicy::Integrated(currents) => Some(RateReference {
                            solution: incoming,
                            storage_currents: Some(currents),
                        }),
                        _ => None,
                    },
                );
            }
            if !matches!(policy, CoordinatePolicy::Project) {
                return Err(error(
                    "continuous event limit fails the unchanged jump equations",
                ));
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

    fn audit_storage(
        &self,
        trial: &[Value],
        incoming_q: &[Value],
        physical: &EventSample,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<()> {
        for (row, &old_charge) in incoming_q.iter().enumerate() {
            if row % 64 == 0 {
                check_abort(abort)?;
            }
            let Some(absolute) = self.storage_tolerance(row, options) else {
                continue;
            };
            let impulses = self
                .source_incidence
                .get(row)
                .into_iter()
                .flatten()
                .map(|&(column, sign)| (trial[column], sign));
            let residual = sum([(physical.q.values[row], 1.0), (old_charge, -1.0)]
                .into_iter()
                .chain(impulses))?;
            let tolerance = absolute
                + options.relative_tolerance * physical.q.scales[row].max(old_charge.abs());
            if residual.abs() > tolerance {
                let kind = if row < self.nodes {
                    "nodal charge"
                } else {
                    "branch flux"
                };
                return Err(error(format!("{kind} conservation failed at row {row}")));
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
        reference: Option<RateReference<'_>>,
    ) -> Result<ChargeEventState> {
        let mut equations = self.rate_equations(&physical, options, abort)?;
        let group_budget =
            if reference.is_some_and(|reference| reference.storage_currents.is_some()) {
                self.integration_reference_budgets(&physical, options, abort)?
            } else {
                Vec::new()
            };
        if let Some(RateReference {
            solution,
            storage_currents: currents,
        }) = reference
        {
            for row in 0..self.size {
                if row % 64 == 0 {
                    check_abort(abort)?;
                }
                if !self.is_group_row(row) && self.storage_tolerance(row, options).is_some() {
                    // An OP balances F against source currents; a continuous
                    // integration reference also contains finite storage current.
                    // Neither changes the matrix or the original equation audit.
                    let reference = sum(self
                        .source_incidence
                        .get(row)
                        .into_iter()
                        .flatten()
                        .map(|&(column, sign)| (solution[column], -sign))
                        .chain([
                            (physical.q_time[row], 1.0),
                            (currents.map_or(0.0, |currents| currents[row]), -1.0),
                        ]))?;
                    let difference =
                        sum([(reference, 1.0), (equations.values[row], -1.0)].into_iter())?;
                    let mut tolerance = equations.absolute[row]
                        + options.relative_tolerance
                            * reference
                                .abs()
                                .max(equations.values[row].abs())
                                .max(physical.f.scales[row]);
                    if currents.is_some() && row < self.nodes {
                        tolerance = tolerance.min(group_budget[row]);
                    }
                    if currents.is_none() || difference.abs() <= tolerance {
                        equations.values[row] = reference;
                    }
                }
            }
        }
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
            // Audit the original sampled equation, including any flux row.
            // A rate reference does not authorize a new current/voltage floor.
            let value = if reference.is_some()
                && !self.is_group_row(row)
                && self.storage_tolerance(row, options).is_some()
            {
                sum([(physical.f.values[row], 1.0), (physical.q_time[row], 1.0)].into_iter())?
            } else {
                equations.values[row]
            };
            let residual = sum(products.clone().chain([(value, 1.0)]))?;
            let mut scale = products.fold(value.abs(), |old, (a, b)| old.max((a * b).abs()));
            if !self.is_group_row(row) && self.storage_tolerance(row, options).is_some() {
                // A storage rate row is original physical KCL (or flux voltage).
                // Its static terms can cancel; preserve their current/voltage
                // scale exactly as the separate original KCL audit does below.
                // Differentiated algebraic rows retain their own per-second scale.
                scale = scale.max(physical.f.scales[row]);
            }
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
