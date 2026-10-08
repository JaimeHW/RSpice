//! A constraint-consistent Newton seed; it does not accept charge or state.
use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Input {
    Source(usize),
    Incoming(usize),
}

pub(super) struct VoltageSeed {
    forms: Vec<(usize, Vec<(Input, Value)>)>,
    pub retained_values: usize,
}

impl VoltageSeed {
    pub(super) fn new(
        nodes: usize,
        sources: &[EventVoltageSource],
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self> {
        Self::build(nodes, sources, None, options, abort)?
            .ok_or_else(|| error("dependent voltage constraints in physical event"))
    }

    /// Local Newton seed only. These derivatives never become the topology's
    /// authored coefficients, conserved rows or acceptance certificate.
    pub(super) fn linearized(
        nodes: usize,
        sources: &[EventVoltageSource],
        sample: &EventSample,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Self>> {
        Self::build(nodes, sources, Some(sample), options, abort)
    }

    fn build(
        nodes: usize,
        sources: &[EventVoltageSource],
        sample: Option<&EventSample>,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Self>> {
        let mut reducer = ExactElimination::<Input>::new(nodes.saturating_add(1), options.limits)?;
        reducer.reserve_retained_words(
            nodes
                .saturating_mul(8)
                .saturating_add(sources.len().saturating_mul(SOURCE_STORAGE_VALUES)),
        )?;
        for (index, source) in sources.iter().enumerate() {
            check_abort(abort)?;
            let affine = source.voltage_terms();
            let physical = sample.filter(|_| affine.is_none());
            if affine.is_none() && physical.is_none() {
                // A nonlinear constraint is solved by the physical Newton
                // owner. It cannot enter an exact constant-coefficient seed.
                continue;
            }
            let mut row = ExactRow::default();
            for (node, coefficient) in
                affine
                    .into_iter()
                    .flatten()
                    .chain(physical.into_iter().flat_map(|sample| {
                        sample.f.rows[source.branch]
                            .iter()
                            .map(|&(column, coefficient)| (column + 1, coefficient))
                    }))
            {
                if node == 0 {
                    continue;
                }
                if node > nodes {
                    return Err(error("nonlinear voltage seed has a non-nodal dependency"));
                }
                reducer.check_cost(row.words().saturating_mul(3).saturating_add(160))?;
                ExactElimination::<Input>::add_integer(
                    &mut row.nodes,
                    node,
                    integer_coefficient(coefficient)
                        .ok_or_else(|| error("invalid voltage seed coefficient"))?,
                );
            }
            // ExactRow stores the right-hand side. The query projection
            // already negates the eliminated residual coefficients.
            row.values
                .insert(Input::Source(index), integer_coefficient(1.0).unwrap());
            if reducer.admit(row, 1, abort)?.is_some() {
                return Ok(None);
            }
        }
        let constrained: Vec<_> = (1..=nodes)
            .filter(|&node| reducer.pivots[node].is_some())
            .collect();
        for node in 1..=nodes {
            check_abort(abort)?;
            if reducer.pivots[node].is_some() {
                continue;
            }
            let mut row = ExactRow::default();
            row.nodes.insert(node, BigInt::from(1));
            row.values
                .insert(Input::Incoming(node - 1), BigInt::from(1));
            reducer.admit(row, 1, abort)?;
        }
        let mut forms = Vec::with_capacity(constrained.len());
        for node in constrained {
            check_abort(abort)?;
            let mut row = ExactRow {
                query: BigInt::from(1),
                ..ExactRow::default()
            };
            row.nodes.insert(node, BigInt::from(1));
            reducer.reduce(&mut row, abort)?;
            reducer.check_cost(row.words().saturating_mul(3))?;
            let words = row.values.len().saturating_mul(6).saturating_add(4);
            reducer.reserve_retained_words(words)?;
            let mut form = Vec::with_capacity(row.values.len());
            for (input, coefficient) in row.values {
                let Some(weight) = coefficient_ratio(&coefficient, &row.query) else {
                    if sample.is_some() {
                        return Ok(None);
                    }
                    return Err(error("voltage seed projection exceeds finite precision"));
                };
                let weight = -weight;
                form.push((input, weight));
            }
            forms.push((node - 1, form));
        }
        let retained_values = forms
            .capacity()
            .saturating_mul(4)
            .saturating_add(4)
            .saturating_add(forms.iter().fold(0usize, |sum, (_, form)| {
                sum.saturating_add(form.capacity().saturating_mul(3))
            }));
        Ok(Some(Self {
            forms,
            retained_values,
        }))
    }

    pub(super) fn project(
        &self,
        incoming: &[Value],
        sources: &[EventVoltageSource],
        trial: &mut [Value],
        abort: &dyn AbortSignal,
    ) -> Result<()> {
        self.project_values(incoming, trial, abort, |index| {
            sources[index]
                .affine()
                .map_or(Value::NAN, |(value, _, _)| value)
        })
    }

    pub(super) fn project_values(
        &self,
        incoming: &[Value],
        trial: &mut [Value],
        abort: &dyn AbortSignal,
        source_value: impl Fn(usize) -> Value,
    ) -> Result<()> {
        for (node, form) in &self.forms {
            check_abort(abort)?;
            trial[*node] = sum(form.iter().map(|&(input, weight)| {
                let value = match input {
                    Input::Source(index) => source_value(index),
                    Input::Incoming(index) => incoming[index],
                };
                (value, weight)
            }))?;
        }
        Ok(())
    }
}

impl ChargeEventTopology {
    fn sampled_voltage_error(
        &self,
        physical: &EventSample,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Value> {
        let mut norm: Value = 0.0;
        for source in &self.sources {
            check_abort(abort)?;
            if source.affine().is_some() {
                continue;
            }
            let row = source.branch;
            let scale =
                options.voltage_tolerance + options.relative_tolerance * physical.f.scales[row];
            let value = if scale.is_finite() && scale > 0.0 {
                physical.f.values[row].abs() / scale
            } else {
                Value::INFINITY
            };
            norm = norm.max(value);
        }
        Ok(norm)
    }

    /// Improve only the initial guess. Re-evaluate the actual constraints at
    /// each local Newton step and backtrack invalid/worse candidates. The
    /// coupled solve still owns all charge, flux, rank and current acceptance.
    pub(in crate::engine::transient::charge_event) fn project_nonlinear_voltage_seed(
        &self,
        incoming: &[Value],
        trial: &mut Vec<Value>,
        options: &EventOptions,
        abort: &dyn AbortSignal,
        sample: &mut impl FnMut(&[Value], &dyn AbortSignal) -> Result<EventSample>,
    ) -> Result<()> {
        if self.sources.iter().all(|source| source.affine().is_some()) {
            return Ok(());
        }
        for _ in 0..options.iterations.min(8) {
            check_abort(abort)?;
            let physical = sample(trial, abort)?;
            if physical.nonfinite(self.size)? {
                return Ok(());
            }
            self.validate_sample(&physical, abort)?;
            let norm = self.sampled_voltage_error(&physical, options, abort)?;
            if norm <= 1.0 {
                return Ok(());
            }
            // Source values and two candidate vectors coexist with physical
            // F/Q stamps and the exact elimination workspace.
            let retained = self.size.saturating_mul(64).saturating_add(
                [&physical.f, &physical.q]
                    .into_iter()
                    .flat_map(|stamp| &stamp.rows)
                    .fold(0usize, |sum, row| {
                        sum.saturating_add(row.capacity().saturating_mul(2))
                    }),
            );
            let seed = with_retained_values(options, retained, |bounded| {
                VoltageSeed::linearized(self.nodes, &self.sources, &physical, bounded, abort)
            })?;
            let Some(seed) = seed else {
                return Ok(());
            };
            let mut values = Vec::with_capacity(self.sources.len());
            for source in &self.sources {
                check_abort(abort)?;
                let value = if let Some((value, _, _)) = source.affine() {
                    value
                } else {
                    // J*x_new = J*x - F(x), with held physical time.
                    let value = sum(physical.f.rows[source.branch]
                        .iter()
                        .map(|&(column, coefficient)| (trial[column], coefficient))
                        .chain([(physical.f.values[source.branch], -1.0)]));
                    let Ok(value) = value else {
                        return Ok(());
                    };
                    value
                };
                values.push(value);
            }
            let mut proposal = trial.clone();
            match seed.project_values(incoming, &mut proposal, abort, |index| values[index]) {
                // This local linearization can exceed finite precision even
                // when the original incoming chart remains usable.
                Err(SimulationError::Circuit(_)) => return Ok(()),
                result => result?,
            }
            // Release derivative/elimination storage before another physical
            // sample is allocated by the line search.
            drop((physical, seed, values));
            let mut accepted = false;
            let mut fraction = 1.0;
            let mut candidate = trial.clone();
            for _ in 0..options.backtracks {
                check_abort(abort)?;
                for node in 0..self.nodes {
                    candidate[node] =
                        sum(
                            [(trial[node], 1.0 - fraction), (proposal[node], fraction)].into_iter()
                        )?;
                }
                let physical = sample(&candidate, abort)?;
                if !physical.nonfinite(self.size)? {
                    self.validate_sample(&physical, abort)?;
                    if self.sampled_voltage_error(&physical, options, abort)? < norm {
                        std::mem::swap(trial, &mut candidate);
                        accepted = true;
                        break;
                    }
                }
                fraction *= 0.5;
            }
            if !accepted {
                return Ok(());
            }
        }
        Ok(())
    }
}
