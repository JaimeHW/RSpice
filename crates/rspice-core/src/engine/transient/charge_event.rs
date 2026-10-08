//! Charge/flux event equations and separate finite-current reconstruction.
//!
//! This operator consumes a prepared charge-incidence topology and physical
//! F/Q stamps. It does not infer device support or modify accepted history.
//! Non-nodal flux equations preserve physical linkage at finite-voltage
//! events. CCCS fanout has weighted current/impulse conservation. Voltage
//! impulses and CCVS constraints still need a full descriptor transition.

use super::{AbortSignal, SimulationError, Value};
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};
use crate::solver::{SolverOptions, StaticMatrix};

mod stamp;
pub(super) use stamp::{EventSample, EventStamp};
mod conservation;
mod flux;
use flux::FluxConservation;
mod rows;
use conservation::{CurrentConservation, EventCurrentControl, with_retained_values};
pub(super) use rows::EventBranchEquation;
use std::sync::Arc;
pub(super) mod circuit;
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

#[derive(Clone, Copy, PartialEq)]
pub(super) struct EventVoltageControl {
    pub positive: usize,
    pub negative: usize,
    pub gain: Value,
}

// Descriptor plus sparse incidence and allocator headroom, in Value units.
const SOURCE_STORAGE_VALUES: usize = 16;

#[derive(Clone, Copy)]
pub(super) enum EventVoltageEquation {
    /// V(out) - gain * V(control) = value. Slope differentiates only the
    /// prescribed right-hand side; control-coordinate rates remain unknowns.
    Affine {
        value: Value,
        slope: Value,
        control: Option<EventVoltageControl>,
    },
    /// The physical sampler owns the complete nonlinear voltage residual,
    /// nodal Jacobian and explicit time partial in this source's branch row.
    Sampled,
}

#[derive(Clone, Copy)]
pub(super) struct EventVoltageSource {
    pub positive: usize,
    pub negative: usize,
    /// Zero-based coordinate in the original MNA solution.
    pub branch: usize,
    pub equation: EventVoltageEquation,
}

impl EventVoltageSource {
    fn affine(&self) -> Option<(Value, Value, Option<EventVoltageControl>)> {
        match self.equation {
            EventVoltageEquation::Affine {
                value,
                slope,
                control,
            } => Some((value, slope, control)),
            EventVoltageEquation::Sampled => None,
        }
    }

    fn valid(&self, nodes: usize) -> bool {
        self.affine().is_none_or(|(value, slope, control)| {
            value.is_finite()
                && slope.is_finite()
                && control.is_none_or(|control| {
                    control.positive <= nodes
                        && control.negative <= nodes
                        && control.gain.is_finite()
                })
        })
    }

    fn voltage_terms(&self) -> Option<impl Iterator<Item = (usize, Value)> + Clone> {
        let (_, _, control) = self.affine()?;
        Some(
            [(self.positive, 1.0), (self.negative, -1.0)]
                .into_iter()
                .chain(control.into_iter().flat_map(|control| {
                    [
                        (control.positive, -control.gain),
                        (control.negative, control.gain),
                    ]
                })),
        )
    }
}

#[derive(Clone)]
pub(super) struct EventOptions {
    pub limits: ResourceLimits,
    pub solver: SolverOptions,
    /// The actual transient numerical conductance on electrical nodal rows.
    pub nodal_gmin: Value,
    pub iterations: usize,
    pub backtracks: usize,
    pub voltage_tolerance: Value,
    pub current_tolerance: Value,
    pub charge_tolerance: Value,
    pub relative_tolerance: Value,
}

impl EventOptions {
    fn validate(&self) -> Result<()> {
        if !self.nodal_gmin.is_finite() || self.nodal_gmin < 0.0 {
            return Err(error("invalid transient nodal conditioning floor"));
        }
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

/// Structural coupling of a prescribed, charge-free nodal current to the
/// jump constraints. This describes equations, not their regularity or a
/// license to discard propagated events. Ideal-source finite currents may
/// jump even when the forcing cancels from the jump equations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::engine::transient) enum CurrentJumpCoupling {
    Cancels,
    Present,
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
    source_incidence: Arc<Vec<Vec<(usize, Value)>>>,
    weighted: Option<Arc<CurrentConservation>>,
    flux: Option<Arc<FluxConservation>>,
    branch_equations: Vec<EventBranchEquation>,
}

impl ChargeEventTopology {
    /// A current contributes +u and -u to its terminal KCL rows. Only
    /// ungrounded group sums retain F in the jump system; equal group roots
    /// cancel this incidence identically, including all its derivatives.
    /// No numerical zero, current magnitude or capacitance threshold enters
    /// this test. Different groups conservatively retain direct coupling.
    ///
    /// With unchanged source constraints and locally smooth, nonsingular
    /// jump/rate equations, cancellation lets a current discontinuity affect
    /// coordinate rates without changing nodal values. Singular systems and
    /// higher-index/unsupported descriptors need a separate analysis; the
    /// ordinary event solve still has to establish its local regularity.
    pub(in crate::engine::transient) fn current_jump_coupling(
        &self,
        positive: usize,
        negative: usize,
    ) -> Result<CurrentJumpCoupling> {
        if positive > self.nodes || negative > self.nodes {
            return Err(error("current event port outside the node population"));
        }
        // Equal charge components cancel in every weighted law. For other
        // components conservatively retain coupling; rounded coefficient
        // equality cannot certify an exact cancellation.
        let cancels = self.weighted.as_ref().map_or_else(
            || self.roots[positive] == self.roots[negative],
            |basis| {
                basis.charge_roots[positive] == basis.charge_roots[negative]
                    || basis.rows.iter().all(Vec::is_empty)
            },
        );
        Ok(if cancels {
            CurrentJumpCoupling::Cancels
        } else {
            CurrentJumpCoupling::Present
        })
    }

    pub(in crate::engine::transient) fn source_branches(
        &self,
    ) -> impl ExactSizeIterator<Item = usize> + '_ {
        self.sources.iter().map(|source| source.branch)
    }

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
                .saturating_add(sources.len().saturating_mul(SOURCE_STORAGE_VALUES)),
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
                || !source.valid(nodes)
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
        let mut degree = vec![0usize; nodes + 1];
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
            if p != n {
                degree[p] += 1;
                degree[n] += 1;
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
        // Replace the charge row at a component's most connected node by
        // its algebraic KCL sum. Keeping the peripheral rows avoids forming
        // a tiny storage mode as the difference of two large hub charges
        // (for example GP Q_BE and a much smaller Q_BC). This is a structural
        // choice of equivalent equations, not a capacitance cutoff. Ground
        // remains the representative of every grounded component.
        let mut representatives = roots.clone();
        for node in 1..=nodes {
            if node % 64 == 0 {
                check_abort(abort)?;
            }
            let component = roots[node];
            if component != 0 && degree[node] > degree[representatives[component]] {
                representatives[component] = node;
            }
        }
        for (index, component) in roots.iter_mut().enumerate() {
            if index % 64 == 0 {
                check_abort(abort)?;
            }
            *component = representatives[*component];
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
            source_incidence: Arc::new(source_incidence),
            weighted: None,
            flux: None,
            branch_equations,
        })
    }

    fn is_group_row(&self, row: usize) -> bool {
        row < self.nodes
            && self.weighted.as_ref().map_or_else(
                || self.roots[row + 1] == row + 1,
                |basis| !basis.rows[row].is_empty(),
            )
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
                for (node, weight) in self.conservation_terms(row) {
                    equations.add_weighted_row(row, &sample.f, node, weight)?;
                }
                equations.absolute[row] = options.current_tolerance;
            } else if let Some(terms) = self.flux_constraint(row) {
                for &(source, weight) in terms {
                    check_abort(abort)?;
                    equations.add_weighted_row(row, &sample.f, source, weight)?;
                }
                equations.absolute[row] = sum(terms.iter().map(|&(source, weight)| {
                    (
                        self.branch_equations[source - self.nodes].rate_tolerance(),
                        weight.abs(),
                    )
                }))?;
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
        for row in 0..self.nodes {
            check_abort(abort)?;
            if !self.is_group_row(row) {
                for &(branch, coefficient) in &self.source_incidence[row] {
                    equations.add(row, branch, coefficient)?;
                    equations.add_value(row, sum([(trial[branch], coefficient)].into_iter())?)?;
                }
            }
        }
        for source in &self.sources {
            let Some((value, _, control)) = source.affine() else {
                // The sampler's nonlinear constraint and Jacobian already
                // occupy this row; never replace them with an affine proxy.
                equations.absolute[source.branch] = options.voltage_tolerance;
                continue;
            };
            equations.source_row(source, value, options.voltage_tolerance)?;
            equations.values[source.branch] = sum(source
                .voltage_terms()
                .into_iter()
                .flatten()
                .map(|(node, coefficient)| (voltage(trial, node), coefficient))
                .chain([(value, -1.0)]))?;
            if let Some(control) = control {
                let controlled = sum([
                    (voltage(trial, control.positive), control.gain),
                    (voltage(trial, control.negative), -control.gain),
                ]
                .into_iter())?;
                equations.scales[source.branch] =
                    equations.scales[source.branch].max(controlled.abs());
            }
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
                for (node, weight) in self.conservation_terms(row) {
                    equations.add_weighted_row(row, &sample.f, node, weight)?;
                }
                equations.values[row] = sum(self
                    .conservation_terms(row)
                    .map(|(node, weight)| (sample.f_time[node], weight)))?;
            } else if let Some(terms) = self.flux_constraint(row) {
                for &(source, weight) in terms {
                    check_abort(abort)?;
                    equations.add_weighted_row(row, &sample.f, source, weight)?;
                }
                // Differentiate the algebraic voltage constraint in rate
                // units. No timestep converts a voltage floor to this row.
                equations.values[row] = sum(terms
                    .iter()
                    .map(|&(source, weight)| (sample.f_time[source], weight)))?;
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
        for row in 0..self.nodes {
            check_abort(abort)?;
            if !self.is_group_row(row) {
                for &(branch, coefficient) in &self.source_incidence[row] {
                    equations.add(row, branch, coefficient)?;
                }
            }
        }
        for source in &self.sources {
            // A differentiated constraint has units per second; its audit
            // uses componentwise relative backward error, not volt/amp floors.
            if let Some((_, slope, _)) = source.affine() {
                equations.source_row(source, slope, 0.0)?;
                equations.values[source.branch] = -slope;
            } else {
                equations.absolute[source.branch] = 0.0;
            }
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
    fn add_weighted_row(
        &mut self,
        row: usize,
        stamp: &EventStamp,
        source: usize,
        weight: Value,
    ) -> Result<()> {
        if weight == 1.0 {
            return self.add_row(row, stamp, source);
        }
        self.add_value(row, sum([(stamp.values[source], weight)].into_iter())?)?;
        self.scales[row] =
            self.scales[row].max(sum([(stamp.scales[source], weight.abs())].into_iter())?);
        for &(column, value) in &stamp.rows[source] {
            self.add(row, column, sum([(value, weight)].into_iter())?)?;
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
        let terms = source
            .voltage_terms()
            .ok_or_else(|| error("nonlinear voltage equation has no affine source row"))?;
        for (node, coefficient) in terms {
            if node != 0 {
                self.add(row, node - 1, coefficient)?;
            }
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
        .map_err(SimulationError::Solver)?;
        let rhs: Vec<_> = self.values.iter().map(|value| -value).collect();
        let result = matrix.solve(&rhs).map_err(SimulationError::Solver)?;
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
