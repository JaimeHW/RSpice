//! Charge/flux event equations and separate finite-current reconstruction.
//!
//! This operator consumes a prepared charge-incidence topology and physical
//! F/Q stamps. It does not infer device support or modify accepted history.
//! Non-nodal flux equations preserve physical linkage at finite-voltage
//! events. Voltage impulses and controlled-source impulse paths still need
//! a descriptor transition with independently prepared topology.

use super::{AbortSignal, SimulationError, Value};
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};
use crate::solver::{SolverOptions, StaticMatrix};

mod stamp;
pub(super) use stamp::{EventSample, EventStamp};
mod rows;
pub(super) use rows::EventBranchEquation;
mod solve;
#[cfg(test)]
mod tests;

type Result<T> = std::result::Result<T, SimulationError>;

fn error(message: impl Into<String>) -> SimulationError {
    SimulationError::Circuit(format!("charge event: {}", message.into()))
}

fn check_abort(abort: &dyn AbortSignal) -> Result<()> {
    if abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(())
    }
}

fn sum(terms: impl Iterator<Item = (Value, Value)> + Clone) -> Result<Value> {
    let value = rspice_veriloga_runtime::arithmetic::sum_products(terms)
        .map_err(|_| error("unrepresentable equation sum"))?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(error("nonfinite equation sum"))
    }
}

#[derive(Clone, Copy)]
pub(super) struct EventVoltageSource {
    pub positive: usize,
    pub negative: usize,
    /// Zero-based coordinate in the original MNA solution.
    pub branch: usize,
    pub value: Value,
    pub slope: Value,
}

pub(super) struct EventOptions {
    pub limits: ResourceLimits,
    pub solver: SolverOptions,
    pub iterations: usize,
    pub backtracks: usize,
    pub voltage_tolerance: Value,
    pub current_tolerance: Value,
    pub charge_tolerance: Value,
    pub relative_tolerance: Value,
}

impl EventOptions {
    fn validate(&self) -> Result<()> {
        if self.iterations == 0
            || self.backtracks == 0
            || ![
                self.voltage_tolerance,
                self.current_tolerance,
                self.charge_tolerance,
                self.relative_tolerance,
            ]
            .into_iter()
            .all(|value| value.is_finite() && value > 0.0)
        {
            return Err(error("invalid tolerances or iteration budgets"));
        }
        Ok(())
    }
}

/// The returned solution contains finite source currents. Integrated source
/// impulses are separate coulomb values, in the source descriptor order.
pub(super) struct ChargeEventState {
    pub solution: Vec<Value>,
    pub source_impulses: Vec<Value>,
    /// Rates for node and non-source constitutive coordinates. This solve
    /// does not determine derivatives of ideal-source branch currents.
    pub coordinate_rates: Vec<Option<Value>>,
    pub iterations: usize,
}

pub(super) struct ChargeEventTopology {
    nodes: usize,
    size: usize,
    sources: Vec<EventVoltageSource>,
    roots: Vec<usize>,
    groups: Vec<Vec<usize>>,
    source_columns: Vec<bool>,
    /// Sparse source incidence for nodal audits; scanning every source for
    /// every node would make validation quadratic on source-rich circuits.
    source_incidence: Vec<Vec<(usize, Value)>>,
    branch_equations: Vec<EventBranchEquation>,
}

impl ChargeEventTopology {
    pub(super) fn new(
        nodes: usize,
        size: usize,
        charge_ports: &[(usize, usize)],
        sources: Vec<EventVoltageSource>,
        branch_equations: Vec<EventBranchEquation>,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self> {
        check_abort(abort)?;
        options.validate()?;
        ResourceLimitError::ensure(
            ResourceKind::MatrixUnknowns,
            size,
            options.limits.max_matrix_unknowns,
        )?;
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            size.saturating_mul(64)
                .saturating_add(charge_ports.len().saturating_mul(2))
                .saturating_add(sources.len().saturating_mul(8)),
            options.limits.max_result_values,
        )?;
        if size == 0
            || nodes > size
            || branch_equations.len() != size - nodes
            || !branch_equations.iter().all(|row| row.valid())
        {
            return Err(error("invalid dimensions, tolerances or iteration budgets"));
        }
        let mut source_columns = vec![false; size];
        let mut source_incidence = vec![Vec::new(); nodes];
        for (index, source) in sources.iter().enumerate() {
            if index % 64 == 0 {
                check_abort(abort)?;
            }
            if source.positive > nodes
                || source.negative > nodes
                || source.branch < nodes
                || source.branch >= size
                || source_columns[source.branch]
                || branch_equations[source.branch - nodes]
                    .flux_tolerance()
                    .is_some()
                || !source.value.is_finite()
                || !source.slope.is_finite()
            {
                return Err(error("invalid or duplicate voltage-source descriptor"));
            }
            source_columns[source.branch] = true;
            for (node, sign) in [(source.positive, 1.0), (source.negative, -1.0)] {
                if node != 0 {
                    source_incidence[node - 1].push((source.branch, sign));
                }
            }
        }
        let mut parents: Vec<_> = (0..=nodes).collect();
        fn root(parents: &mut [usize], mut node: usize) -> usize {
            while parents[node] != node {
                parents[node] = parents[parents[node]];
                node = parents[node];
            }
            node
        }
        for (index, (p, n)) in charge_ports
            .iter()
            .copied()
            .chain(
                sources
                    .iter()
                    .map(|source| (source.positive, source.negative)),
            )
            .enumerate()
        {
            if index % 64 == 0 {
                check_abort(abort)?;
            }
            if p > nodes || n > nodes {
                return Err(error("charge port outside the node population"));
            }
            let p = root(&mut parents, p);
            let n = root(&mut parents, n);
            parents[p.max(n)] = p.min(n);
        }
        let mut roots = Vec::with_capacity(nodes + 1);
        for node in 0..=nodes {
            if node % 64 == 0 {
                check_abort(abort)?;
            }
            roots.push(root(&mut parents, node));
        }
        let mut groups = vec![Vec::new(); nodes + 1];
        for node in 1..=nodes {
            if node % 64 == 0 {
                check_abort(abort)?;
            }
            groups[roots[node]].push(node - 1);
        }
        Ok(Self {
            nodes,
            size,
            sources,
            roots,
            groups,
            source_columns,
            source_incidence,
            branch_equations,
        })
    }

    fn is_group_row(&self, row: usize) -> bool {
        row < self.nodes && self.roots[row + 1] == row + 1
    }

    fn storage_tolerance(&self, row: usize, options: &EventOptions) -> Option<Value> {
        if row < self.nodes {
            Some(options.charge_tolerance)
        } else {
            self.branch_equations[row - self.nodes].flux_tolerance()
        }
    }

    fn physical_probe(&self, trial: &[Value]) -> Vec<Value> {
        let mut state = trial.to_vec();
        for source in &self.sources {
            state[source.branch] = 0.0;
        }
        state
    }

    fn validate_sample(&self, sample: &EventSample, abort: &dyn AbortSignal) -> Result<()> {
        sample.validate(self.size)?;
        for (index, stamp) in [&sample.f, &sample.q].into_iter().enumerate() {
            for (row, entries) in stamp.rows.iter().enumerate() {
                if row % 64 == 0 {
                    check_abort(abort)?;
                }
                if entries
                    .iter()
                    .any(|&(column, value)| self.source_columns[column] && value != 0.0)
                {
                    return Err(error(
                        "physical F/Q must exclude ideal-source branch-current dependencies",
                    ));
                }
                if index == 1
                    && row >= self.nodes
                    && self.branch_equations[row - self.nodes]
                        .flux_tolerance()
                        .is_none()
                    && (stamp.values[row] != 0.0
                        || sample.q_time[row] != 0.0
                        || entries.iter().any(|(_, value)| *value != 0.0))
                {
                    return Err(error("non-nodal storage has no prepared flux equation"));
                }
            }
        }
        Ok(())
    }

    fn jump_equations(
        &self,
        trial: &[Value],
        incoming_q: &[Value],
        sample: &EventSample,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Equations> {
        self.validate_sample(sample, abort)?;
        let mut equations = Equations::new(self.size, options);
        for (row, &old_charge) in incoming_q.iter().enumerate() {
            if row % 64 == 0 {
                check_abort(abort)?;
            }
            if self.is_group_row(row) {
                for &node in &self.groups[row + 1] {
                    equations.add_row(row, &sample.f, node)?;
                }
                equations.absolute[row] = options.current_tolerance;
            } else if let Some(tolerance) = self.storage_tolerance(row, options) {
                equations.add_row(row, &sample.q, row)?;
                equations.values[row] =
                    sum([(sample.q.values[row], 1.0), (old_charge, -1.0)].into_iter())?;
                equations.scales[row] = sample.q.scales[row].max(old_charge.abs());
                equations.absolute[row] = tolerance;
            } else {
                equations.add_row(row, &sample.f, row)?;
                equations.absolute[row] = self.branch_equations[row - self.nodes].jump_tolerance();
            }
        }
        for source in &self.sources {
            for (node, sign) in [(source.positive, 1.0), (source.negative, -1.0)] {
                if node != 0 && !self.is_group_row(node - 1) {
                    equations.add(node - 1, source.branch, sign)?;
                    equations.add_value(node - 1, sign * trial[source.branch])?;
                }
            }
            equations.source_row(source, source.value, options.voltage_tolerance)?;
            equations.values[source.branch] = sum([
                (voltage(trial, source.positive), 1.0),
                (voltage(trial, source.negative), -1.0),
                (source.value, -1.0),
            ]
            .into_iter())?;
        }
        Ok(equations)
    }

    fn rate_equations(
        &self,
        sample: &EventSample,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Equations> {
        self.validate_sample(sample, abort)?;
        let mut equations = Equations::new(self.size, options);
        for row in 0..self.size {
            if row % 64 == 0 {
                check_abort(abort)?;
            }
            if self.is_group_row(row) {
                for &node in &self.groups[row + 1] {
                    equations.add_row(row, &sample.f, node)?;
                }
                equations.values[row] = sum(self.groups[row + 1]
                    .iter()
                    .map(|&node| (sample.f_time[node], 1.0)))?;
            } else if self.storage_tolerance(row, options).is_some() {
                equations.add_row(row, &sample.q, row)?;
                equations.values[row] =
                    sum([(sample.f.values[row], 1.0), (sample.q_time[row], 1.0)].into_iter())?;
                equations.absolute[row] = if row < self.nodes {
                    options.current_tolerance
                } else {
                    self.branch_equations[row - self.nodes].rate_tolerance()
                };
            } else {
                equations.add_row(row, &sample.f, row)?;
                equations.values[row] = sample.f_time[row];
            }
        }
        for source in &self.sources {
            for (node, sign) in [(source.positive, 1.0), (source.negative, -1.0)] {
                if node != 0 && !self.is_group_row(node - 1) {
                    equations.add(node - 1, source.branch, sign)?;
                }
            }
            // A differentiated constraint has units per second; its audit
            // uses componentwise relative backward error, not volt/amp floors.
            equations.source_row(source, source.slope, 0.0)?;
            equations.values[source.branch] = -source.slope;
        }
        Ok(equations)
    }
}

fn voltage(state: &[Value], node: usize) -> Value {
    if node == 0 { 0.0 } else { state[node - 1] }
}

struct Equations {
    rows: Vec<Vec<(usize, Value)>>,
    values: Vec<Value>,
    scales: Vec<Value>,
    absolute: Vec<Value>,
    terms: usize,
    limit: usize,
}

impl Equations {
    fn new(size: usize, options: &EventOptions) -> Self {
        Self {
            rows: vec![Vec::new(); size],
            values: vec![0.0; size],
            scales: vec![0.0; size],
            absolute: vec![0.0; size],
            terms: 0,
            limit: options
                .limits
                .max_result_values
                .saturating_sub(size.saturating_mul(64))
                / 256,
        }
    }
    fn add(&mut self, row: usize, column: usize, value: Value) -> Result<()> {
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            self.terms.saturating_add(1).saturating_mul(256),
            self.limit.saturating_mul(256),
        )?;
        self.rows[row].push((column, value));
        self.terms += 1;
        Ok(())
    }
    fn add_value(&mut self, row: usize, value: Value) -> Result<()> {
        self.values[row] = sum([(self.values[row], 1.0), (value, 1.0)].into_iter())?;
        self.scales[row] = self.scales[row].max(value.abs());
        Ok(())
    }
    fn add_row(&mut self, row: usize, stamp: &EventStamp, source: usize) -> Result<()> {
        self.add_value(row, stamp.values[source])?;
        self.scales[row] = self.scales[row].max(stamp.scales[source]);
        for &(column, value) in &stamp.rows[source] {
            self.add(row, column, value)?;
        }
        Ok(())
    }
    fn source_row(
        &mut self,
        source: &EventVoltageSource,
        value: Value,
        tolerance: Value,
    ) -> Result<()> {
        let row = source.branch;
        self.terms -= self.rows[row].len();
        self.rows[row].clear();
        if source.positive != 0 {
            self.add(row, source.positive - 1, 1.0)?;
        }
        if source.negative != 0 {
            self.add(row, source.negative - 1, -1.0)?;
        }
        self.scales[row] = value.abs();
        self.absolute[row] = tolerance;
        Ok(())
    }
    fn solve(&self, options: &EventOptions, abort: &dyn AbortSignal) -> Result<Vec<Value>> {
        check_abort(abort)?;
        let entries: Vec<_> = self
            .rows
            .iter()
            .enumerate()
            .flat_map(|(row, entries)| {
                entries
                    .iter()
                    .map(move |&(column, value)| (row, column, value))
            })
            .collect();
        let mut matrix = StaticMatrix::from_triplets_with_options(
            self.values.len(),
            self.values.len(),
            &entries,
            options.solver,
        )
        .map_err(|failure| error(failure.to_string()))?;
        let rhs: Vec<_> = self.values.iter().map(|value| -value).collect();
        let result = matrix
            .solve(&rhs)
            .map_err(|failure| error(failure.to_string()))?;
        check_abort(abort)?;
        Ok(result)
    }
    fn norm(&self, options: &EventOptions) -> Result<Value> {
        let mut norm: Value = 0.0;
        for row in 0..self.values.len() {
            let denominator = self.absolute[row] + options.relative_tolerance * self.scales[row];
            if !denominator.is_finite() || !self.values[row].is_finite() {
                return Err(error("nonfinite residual scale"));
            }
            let value = if denominator == 0.0 {
                if self.values[row] == 0.0 {
                    0.0
                } else {
                    Value::INFINITY
                }
            } else {
                self.values[row].abs() / denominator
            };
            norm = norm.max(value);
        }
        Ok(norm)
    }
}
