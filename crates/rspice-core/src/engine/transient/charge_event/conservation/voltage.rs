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
        let mut reducer = ExactElimination::<Input>::new(nodes.saturating_add(1), options.limits)?;
        reducer.reserve_retained_words(
            nodes
                .saturating_mul(8)
                .saturating_add(sources.len().saturating_mul(SOURCE_STORAGE_VALUES)),
        )?;
        for (index, source) in sources.iter().enumerate() {
            check_abort(abort)?;
            let mut row = ExactRow::default();
            for (node, coefficient) in source.voltage_terms() {
                if node == 0 {
                    continue;
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
                return Err(error("dependent voltage constraints in physical event"));
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
                let weight = -coefficient_ratio(&coefficient, &row.query)
                    .ok_or_else(|| error("voltage seed projection exceeds finite precision"))?;
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
        Ok(Self {
            forms,
            retained_values,
        })
    }

    pub(super) fn project(
        &self,
        incoming: &[Value],
        sources: &[EventVoltageSource],
        trial: &mut [Value],
        abort: &dyn AbortSignal,
    ) -> Result<()> {
        for (node, form) in &self.forms {
            check_abort(abort)?;
            trial[*node] = sum(form.iter().map(|&(input, weight)| {
                let value = match input {
                    Input::Source(index) => sources[index].value,
                    Input::Incoming(index) => incoming[index],
                };
                (value, weight)
            }))?;
        }
        Ok(())
    }
}
