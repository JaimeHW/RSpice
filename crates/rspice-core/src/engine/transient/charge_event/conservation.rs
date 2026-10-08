//! Weighted conservation laws for ideal-source current and impulse fanout.
use super::*;
use crate::engine::exact_constraints::{
    ExactElimination, ExactRow, coefficient_ratio, integer_coefficient,
};
use num_bigint::BigInt;
use std::sync::Arc;

mod voltage;
use voltage::VoltageSeed;

#[derive(Clone, Copy)]
pub(super) struct EventCurrentControl {
    pub positive: usize,
    pub negative: usize,
    /// Zero-based original MNA controlling coordinate.
    pub branch: usize,
    pub gain: Value,
}

/// Identity of the authored constraint, excluding its time-varying forcing.
#[derive(Clone, Copy, PartialEq)]
struct VoltageBinding {
    positive: usize,
    negative: usize,
    branch: usize,
    control: Option<EventVoltageControl>,
    sampled: bool,
}

impl From<&EventVoltageSource> for VoltageBinding {
    fn from(source: &EventVoltageSource) -> Self {
        Self {
            positive: source.positive,
            negative: source.negative,
            branch: source.branch,
            control: source.affine().and_then(|(_, _, control)| control),
            sampled: source.affine().is_none(),
        }
    }
}

pub(super) struct CurrentConservation {
    pub rows: Vec<Vec<(usize, Value)>>,
    pub incidence: Arc<Vec<Vec<(usize, Value)>>>,
    pub source_columns: Vec<bool>,
    pub retained_values: usize,
    voltage_seed: VoltageSeed,
    pub(super) charge_roots: Vec<usize>,
    bindings: Vec<VoltageBinding>,
    size: usize,
}

impl CurrentConservation {
    /// Compile once per prepared circuit. The weights annihilate every
    /// charge-port column and complete source-current incidence column.
    /// Rank decisions use exact authored coefficients, never a DC Jacobian.
    pub(super) fn new(
        nodes: usize,
        size: usize,
        charge_ports: &[(usize, usize)],
        sources: &[EventVoltageSource],
        controls: &[EventCurrentControl],
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self> {
        check_abort(abort)?;
        if nodes > size {
            return Err(error("invalid current descriptor dimensions"));
        }
        let terms = sources
            .len()
            .saturating_add(controls.len())
            .saturating_mul(2);
        let overhead = size
            .saturating_mul(64)
            .saturating_add(nodes.saturating_mul(16))
            .saturating_add(sources.len().saturating_mul(24))
            .saturating_add(controls.len().saturating_mul(8))
            .saturating_add(terms.saturating_mul(16))
            .saturating_add(charge_ports.len().saturating_mul(2));
        ExactElimination::<usize>::ensure_words(overhead, options.limits.max_result_values)?;
        let mut reducer = ExactElimination::<usize>::new(nodes.saturating_add(1), options.limits)?;
        reducer.reserve_retained_words(overhead)?;
        let mut parents: Vec<_> = (0..=nodes).collect();
        let mut degree = vec![0usize; nodes + 1];
        fn root(parents: &mut [usize], mut node: usize) -> usize {
            while parents[node] != node {
                parents[node] = parents[parents[node]];
                node = parents[node];
            }
            node
        }
        for &(p, n) in charge_ports {
            check_abort(abort)?;
            if p > nodes || n > nodes {
                return Err(error("charge port outside current descriptor"));
            }
            if p != n {
                degree[p] += 1;
                degree[n] += 1;
            }
            let p = root(&mut parents, p);
            let n = root(&mut parents, n);
            parents[p.max(n)] = p.min(n);
        }
        let mut charge_roots = Vec::with_capacity(nodes + 1);
        for node in 0..=nodes {
            check_abort(abort)?;
            charge_roots.push(root(&mut parents, node));
        }
        // Keep small peripheral charge rows, as in the ordinary graph path.
        let mut representatives = charge_roots.clone();
        for node in 1..=nodes {
            let component = charge_roots[node];
            if component != 0 && degree[node] > degree[representatives[component]] {
                representatives[component] = node;
            }
        }
        for component in &mut charge_roots {
            *component = representatives[*component];
        }
        let mut source_columns = vec![false; size];
        let mut positions = vec![None; size];
        let mut columns = vec![Vec::new(); sources.len()];
        let mut bindings = Vec::with_capacity(sources.len());
        for (index, source) in sources.iter().enumerate() {
            check_abort(abort)?;
            if source.branch < nodes
                || source.branch >= size
                || source_columns[source.branch]
                || source.positive > nodes
                || source.negative > nodes
                || !source.valid(nodes)
            {
                return Err(error("invalid source in current descriptor"));
            }
            source_columns[source.branch] = true;
            positions[source.branch] = Some(index);
            bindings.push(VoltageBinding::from(source));
            columns[index].extend([(source.positive, 1.0), (source.negative, -1.0)]);
        }
        for control in controls {
            check_abort(abort)?;
            if control.positive > nodes || control.negative > nodes || !control.gain.is_finite() {
                return Err(error("invalid current-control descriptor"));
            }
            let index = positions
                .get(control.branch)
                .copied()
                .flatten()
                .ok_or_else(|| error("impulsive current control has no ideal-source owner"))?;
            columns[index].extend([
                (control.positive, control.gain),
                (control.negative, -control.gain),
            ]);
        }
        let mut incidence = vec![Vec::new(); nodes];
        for (index, column) in columns.into_iter().enumerate() {
            let mut row = ExactRow::<usize>::default();
            for (node, coefficient) in column {
                check_abort(abort)?;
                if node == 0 || coefficient == 0.0 {
                    continue;
                }
                incidence[node - 1].push((sources[index].branch, coefficient));
                let component = charge_roots[node];
                if component != 0 {
                    reducer.check_cost(row.words().saturating_mul(3).saturating_add(160))?;
                    ExactElimination::<usize>::add_integer(
                        &mut row.nodes,
                        component,
                        integer_coefficient(coefficient)
                            .ok_or_else(|| error("nonfinite incidence coefficient"))?,
                    );
                }
            }
            reducer.admit(row, 1, abort)?;
        }
        let free: Vec<_> = (1..=nodes)
            .filter(|&node| charge_roots[node] == node && reducer.pivots[node].is_none())
            .collect();
        for &node in &free {
            check_abort(abort)?;
            let mut row = ExactRow::default();
            row.nodes.insert(node, BigInt::from(1));
            row.values.insert(node, BigInt::from(-1));
            reducer.admit(row, 1, abort)?;
        }
        let mut members = vec![Vec::new(); nodes + 1];
        for node in 1..=nodes {
            members[charge_roots[node]].push(node - 1);
        }
        let mut rows = vec![Vec::new(); nodes];
        for (component, component_members) in members.iter().enumerate().skip(1) {
            check_abort(abort)?;
            if component_members.is_empty() {
                continue;
            }
            let mut row = ExactRow {
                query: BigInt::from(1),
                ..ExactRow::default()
            };
            row.nodes.insert(component, BigInt::from(1));
            reducer.reduce(&mut row, abort)?;
            if !row.nodes.is_empty() {
                return Err(error("incomplete current conservation basis"));
            }
            reducer.check_cost(row.words().saturating_mul(3))?;
            for (free, coefficient) in row.values {
                let weight = -coefficient_ratio(&coefficient, &row.query).ok_or_else(|| {
                    error("current conservation projection exceeds finite precision")
                })?;
                reducer.reserve_retained_words(component_members.len().saturating_mul(4))?;
                for &node in component_members {
                    rows[free - 1].push((node, weight));
                }
            }
        }
        let retained_values = nodes
            .saturating_mul(8)
            .saturating_add(size)
            .saturating_add(
                bindings
                    .capacity()
                    .saturating_mul(std::mem::size_of::<VoltageBinding>().div_ceil(8)),
            )
            .saturating_add(8)
            .saturating_add(rows.iter().chain(&incidence).fold(0usize, |sum, row| {
                sum.saturating_add(row.capacity().saturating_mul(2))
            }));
        // Elimination scratch must not coexist with a second uncharged
        // preparation budget. The retained physical basis remains alive.
        drop(reducer);
        drop((parents, degree, representatives, positions, members, free));
        let voltage_seed = with_retained_values(
            options,
            retained_values.saturating_add(size.saturating_mul(64)),
            |bounded| VoltageSeed::new(nodes, sources, bounded, abort),
        )?;
        let retained_values = retained_values.saturating_add(voltage_seed.retained_values);
        Ok(Self {
            rows,
            incidence: Arc::new(incidence),
            source_columns,
            retained_values,
            voltage_seed,
            charge_roots,
            bindings,
            size,
        })
    }
}

/// Account for immutable prepared storage alongside each disposable solve or
/// sample workspace, preserving diagnostics in the caller's original budget.
pub(super) fn with_retained_values<T>(
    options: &EventOptions,
    retained: usize,
    action: impl FnOnce(&EventOptions) -> Result<T>,
) -> Result<T> {
    if retained == 0 {
        return action(options);
    }
    ResourceLimitError::ensure(
        ResourceKind::ResultValues,
        retained,
        options.limits.max_result_values,
    )?;
    let mut bounded = options.clone();
    bounded.limits.max_result_values -= retained;
    action(&bounded).map_err(|failure| match failure {
        SimulationError::ResourceLimit(mut failure)
            if failure.resource == ResourceKind::ResultValues =>
        {
            failure.requested = failure.requested.saturating_add(retained);
            failure.limit = options.limits.max_result_values;
            SimulationError::ResourceLimit(failure)
        }
        failure => failure,
    })
}

impl ChargeEventTopology {
    pub(super) fn project_voltage_seed(
        &self,
        incoming: &[Value],
        trial: &mut [Value],
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<()> {
        if let Some(basis) = &self.weighted {
            basis
                .voltage_seed
                .project(incoming, &self.sources, trial, abort)?;
        } else if !self.sources.is_empty() {
            with_retained_values(options, self.size.saturating_mul(64), |bounded| {
                VoltageSeed::new(self.nodes, &self.sources, bounded, abort)?.project(
                    incoming,
                    &self.sources,
                    trial,
                    abort,
                )
            })?;
        }
        Ok(())
    }

    pub(super) fn install_current_conservation(
        &mut self,
        prepared: Arc<CurrentConservation>,
    ) -> Result<()> {
        if prepared.size != self.size
            || prepared.rows.len() != self.nodes
            || !prepared
                .bindings
                .iter()
                .copied()
                .eq(self.sources.iter().map(VoltageBinding::from))
        {
            return Err(error(
                "current conservation bindings do not match event topology",
            ));
        }
        self.source_incidence = Arc::clone(&prepared.incidence);
        self.weighted = Some(prepared);
        Ok(())
    }

    pub(super) fn conservation_terms(
        &self,
        row: usize,
    ) -> impl Iterator<Item = (usize, Value)> + Clone {
        let plain = if self.weighted.is_none() {
            self.groups[row + 1].as_slice()
        } else {
            &[]
        };
        let weighted = self
            .weighted
            .as_ref()
            .map_or(&[][..], |basis| basis.rows[row].as_slice());
        plain
            .iter()
            .map(|&node| (node, 1.0))
            .chain(weighted.iter().copied())
    }

    pub(super) fn integration_reference_budgets(
        &self,
        physical: &EventSample,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>> {
        let mut budgets = vec![Value::INFINITY; self.nodes];
        for omitted in 0..self.nodes {
            check_abort(abort)?;
            if !self.is_group_row(omitted) {
                continue;
            }
            let terms = self.conservation_terms(omitted);
            let residual = sum(terms.clone().flat_map(|(row, weight)| {
                [
                    (physical.f.values[row], weight),
                    (physical.q_time[row], weight),
                ]
            }))?;
            let tolerance =
                options.current_tolerance + options.relative_tolerance * physical.f.scales[omitted];
            // Each basis row has unit weight at its own omitted KCL and zero
            // weight at other omitted rows. Share the available physical KCL
            // budget across retained rows, reserving half for solve roundoff.
            let share =
                0.5 * (tolerance - residual.abs()).max(0.0) / terms.clone().count() as Value;
            for (row, weight) in terms {
                budgets[row] = budgets[row].min(share / weight.abs());
            }
        }
        Ok(budgets)
    }
}
