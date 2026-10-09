//! Authored constant-linear circuit descriptor and regular source jets.
//! Nonlinear F/Q are never replaced by a frozen small-signal matrix here.
use super::*;
use crate::device::MatrixStamper;
use crate::numerics::exact_constraints::{
    ConstraintError,
    transition::{PreparedTransition, Transition, TransitionStorage},
};

#[derive(Clone, Copy)]
enum Forcing {
    Voltage(usize),
    Current(usize),
}

pub(in crate::engine::transient) struct LinearDescriptor {
    kernel: PreparedTransition,
    forcing: Vec<Vec<(Forcing, Value)>>,
    source_branches: Vec<usize>,
    retained_values: usize,
}

fn kernel_error(error: ConstraintError) -> SimulationError {
    match error {
        ConstraintError::Aborted => SimulationError::Aborted,
        ConstraintError::ResourceLimit(error) => error.into(),
        ConstraintError::Invalid(message) => super::error(message),
    }
}

impl LinearDescriptor {
    pub(super) fn prepare(
        sampler: &mut PreparedEventCircuit<'_>,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self> {
        let circuit = sampler.circuit;
        if !PreparedEventCircuit::supports_linear_events(circuit) {
            return Err(error(
                "constant-linear descriptor has nonlinear or unowned equations",
            ));
        }
        let size = circuit.matrix_size();
        let nodes = circuit.num_nodes();
        let forcing_count = circuit
            .voltage_sources
            .len()
            .saturating_add(circuit.current_sources.len().saturating_mul(2));
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            size.saturating_mul(64)
                .saturating_add(forcing_count.saturating_mul(8)),
            options.limits.max_result_values,
        )?;
        // This sampler supplies the original constant R/C/L/G/F coefficients,
        // including mutual linkage and the configured shunts. Ideal-source
        // rows and incidence are added explicitly below, before exact closure.
        let mut sample = sampler.sample(
            0.0,
            SourceTimeSide::RightLimit,
            &vec![0.0; size],
            &[],
            options,
            abort,
        )?;
        let mut source_branches = Vec::new();
        for source in &sampler.constant_sources {
            check_abort(abort)?;
            let row = source.branch + 1;
            source_branches.push(source.branch);
            sample.f.stamp(source.positive, row, 1.0);
            sample.f.stamp(source.negative, row, -1.0);
            for (node, coefficient) in source
                .voltage_terms()
                .ok_or_else(|| error("nonlinear source in constant descriptor"))?
            {
                sample.f.stamp(row, node, coefficient);
            }
        }
        let mut forcing = vec![Vec::new(); size];
        let voltage = &circuit.voltage_sources;
        for index in 0..voltage.len() {
            check_abort(abort)?;
            let row = nodes + voltage.branch_indices[index];
            source_branches.push(row - 1);
            let p = voltage.node_pos[index];
            let n = voltage.node_neg[index];
            sample.f.stamp(p, row, 1.0);
            sample.f.stamp(n, row, -1.0);
            sample.f.stamp(row, p, 1.0);
            sample.f.stamp(row, n, -1.0);
            forcing[row - 1].push((Forcing::Voltage(index), 1.0));
        }
        let ccvs = &circuit.ccvs;
        for index in 0..ccvs.len() {
            check_abort(abort)?;
            let row = nodes + ccvs.branch_indices[index];
            source_branches.push(row - 1);
            let p = ccvs.node_pos[index];
            let n = ccvs.node_neg[index];
            sample.f.stamp(p, row, 1.0);
            sample.f.stamp(n, row, -1.0);
            sample.f.stamp(row, p, 1.0);
            sample.f.stamp(row, n, -1.0);
            sample.f.stamp(
                row,
                nodes + ccvs.ctrl_branch[index],
                -ccvs.transresistances[index],
            );
        }
        let current = &circuit.current_sources;
        for index in 0..current.len() {
            check_abort(abort)?;
            for (node, sign) in [
                (current.node_pos[index], -1.0),
                (current.node_neg[index], 1.0),
            ] {
                if node != 0 {
                    forcing[node - 1].push((Forcing::Current(index), sign));
                }
            }
        }
        sample.validate(size)?;
        let entries = sample
            .f
            .rows
            .iter()
            .chain(&sample.q.rows)
            .fold(0usize, |count, row| count.saturating_add(row.len()));
        let retained_values = size
            .saturating_mul(4)
            .saturating_add(forcing_count.saturating_mul(8));
        let kernel = with_retained_values(
            options,
            size.saturating_mul(64)
                .saturating_add(entries.saturating_mul(8))
                .saturating_add(retained_values),
            |bounded| {
                let entries = |stamp: &EventStamp| {
                    stamp
                        .rows
                        .iter()
                        .enumerate()
                        .flat_map(|(row, terms)| {
                            terms
                                .iter()
                                .map(move |&(column, value)| (row, column, value))
                        })
                        .collect::<Vec<_>>()
                };
                PreparedTransition::new(
                    size,
                    &entries(&sample.f),
                    &entries(&sample.q),
                    bounded.limits,
                    abort,
                )
                .map_err(kernel_error)
            },
        )?;
        Ok(Self {
            kernel,
            forcing,
            source_branches,
            retained_values,
        })
    }

    pub(in crate::engine::transient) fn source_branches(&self) -> &[usize] {
        &self.source_branches
    }

    pub(in crate::engine::transient) fn storage(
        &self,
        state: &[Value],
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<TransitionStorage> {
        with_retained_values(options, self.retained_values, |bounded| {
            self.kernel
                .charge_from_coordinates(state, bounded.limits, abort)
                .map_err(kernel_error)
        })
    }

    pub(in crate::engine::transient) fn startup_storage(
        &self,
        products: impl Iterator<Item = (usize, Value, Value)>,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<TransitionStorage> {
        with_retained_values(options, self.retained_values, |bounded| {
            self.kernel
                .storage_from_products(products, bounded.limits, abort)
                .map_err(kernel_error)
        })
    }

    pub(in crate::engine::transient) fn evaluate(
        &self,
        circuit: &crate::CircuitData,
        time: Value,
        storage: &TransitionStorage,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Transition> {
        check_abort(abort)?;
        if !time.is_finite() || time < 0.0 {
            return Err(error("invalid descriptor event clock"));
        }
        with_retained_values(options, self.retained_values, |bounded| {
            self.kernel
                .evaluate_storage(
                    storage,
                    |row, order| {
                        let mut terms = Vec::with_capacity(self.forcing[row].len());
                        for &(source, coefficient) in &self.forcing[row] {
                            if abort.is_aborted() {
                                return Err(ConstraintError::Aborted);
                            }
                            terms.push((source.derivative(circuit, time, order)?, coefficient));
                        }
                        rspice_veriloga_runtime::arithmetic::sum_products(terms.into_iter())
                            .map_err(|_| {
                                ConstraintError::Invalid(
                                    "unrepresentable descriptor forcing".into(),
                                )
                            })
                    },
                    bounded.limits,
                    abort,
                )
                .map_err(kernel_error)
        })
    }

    pub(in crate::engine::transient) fn retained_values(&self) -> usize {
        self.retained_values
            .saturating_add(self.kernel.retained_words())
    }
}

impl Forcing {
    fn derivative(
        self,
        circuit: &crate::CircuitData,
        time: Value,
        order: usize,
    ) -> std::result::Result<Value, ConstraintError> {
        let side = SourceTimeSide::RightLimit;
        let (name, value) = match self {
            Self::Voltage(index) => {
                let table = &circuit.voltage_sources;
                (
                    &table.names[index],
                    table.time_derivative_at_on_side(index, time, order, side),
                )
            }
            Self::Current(index) => {
                let table = &circuit.current_sources;
                (
                    &table.names[index],
                    table.time_derivative_at_on_side(index, time, order, side),
                )
            }
        };
        value.filter(|value| value.is_finite()).ok_or_else(|| {
            ConstraintError::Invalid(format!(
                "source '{name}' has no finite outgoing derivative of order {order}"
            ))
        })
    }
}
