//! Prepared physical circuit equations. Device families are admitted
//! explicitly; an absent sampler never means a zero contribution.

use super::*;
use crate::circuit::SourceTimeSide;
use crate::device::Bjt;
use crate::engine::periodic_capability::PeriodicDeviceFamily;
use rspice_veriloga_runtime::transport_delay::{DelayBuffer, DelayTimeSide};

mod coupling;
mod prepare;
mod sample;
#[cfg(test)]
mod tests;

#[derive(Clone, Copy)]
pub(in crate::engine::transient) struct EventPhase<'a> {
    pub history: &'a DelayBuffer,
    /// Forward current from the independently solved incoming state.
    pub endpoint: Value,
}

pub(in crate::engine::transient) struct PreparedEventCircuit<'a> {
    circuit: &'a crate::CircuitData,
    models: Vec<Bjt>,
    ports: Vec<(usize, usize)>,
    equations: Vec<EventBranchEquation>,
    constant_sources: Vec<EventVoltageSource>,
}

fn side(side: SourceTimeSide) -> Result<DelayTimeSide> {
    match side {
        SourceTimeSide::LeftLimit => Ok(DelayTimeSide::Incoming),
        SourceTimeSide::RightLimit => Ok(DelayTimeSide::Outgoing),
        SourceTimeSide::Published => Err(error(
            "physical event equations require an incoming or outgoing source side",
        )),
    }
}

impl PreparedEventCircuit<'_> {
    pub(in crate::engine::transient) fn topology(
        &self,
        time: Value,
        source_side: SourceTimeSide,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<ChargeEventTopology> {
        check_abort(abort)?;
        options.validate()?;
        ResourceLimitError::ensure(
            ResourceKind::MatrixUnknowns,
            self.circuit.matrix_size(),
            options.limits.max_matrix_unknowns,
        )?;
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            self.circuit
                .matrix_size()
                .saturating_mul(64)
                .saturating_add(self.ports.len().saturating_mul(2))
                .saturating_add(
                    self.constant_sources
                        .len()
                        .saturating_add(self.circuit.voltage_sources.len())
                        .saturating_mul(8),
                ),
            options.limits.max_result_values,
        )?;
        side(source_side)?;
        if !time.is_finite() || time < 0.0 {
            return Err(error("invalid physical event time"));
        }
        let mut sources = self.constant_sources.clone();
        let table = &self.circuit.voltage_sources;
        for index in 0..table.len() {
            if index % 64 == 0 {
                check_abort(abort)?;
            }
            let value = table.transient_value_at_on_side(index, time, source_side);
            let slope = table
                .time_derivative_at_on_side(index, time, 1, source_side)
                .ok_or_else(|| {
                    error(format!(
                        "source '{}' has no regular slope on the requested side",
                        table.names[index]
                    ))
                })?;
            sources.push(EventVoltageSource {
                positive: table.node_pos[index],
                negative: table.node_neg[index],
                branch: self.circuit.num_nodes() + table.branch_indices[index] - 1,
                value,
                slope,
            });
        }
        ChargeEventTopology::new(
            self.circuit.num_nodes(),
            self.circuit.matrix_size(),
            &self.ports,
            sources,
            self.equations.clone(),
            options,
            abort,
        )
    }

    pub(in crate::engine::transient) fn forward_inputs(
        &self,
        state: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Option<Value>>> {
        self.validate_state(state)?;
        let mut inputs = Vec::with_capacity(self.models.len());
        for model in &self.models {
            check_abort(abort)?;
            let current = if model.legacy_excess_phase_delay() == 0.0 {
                None
            } else {
                let branch = model
                    .legacy_forward_transport_branch(&model.mna_internal_state_at_solution(state))
                    .ok_or_else(|| {
                        error(format!(
                            "BJT '{}' has no GP forward-current equation",
                            model.name
                        ))
                    })?;
                if !branch.current.is_finite() {
                    return Err(error(format!(
                        "BJT '{}' has nonfinite forward current",
                        model.name
                    )));
                }
                Some(branch.current)
            };
            inputs.push(current);
        }
        Ok(inputs)
    }

    fn validate_state(&self, state: &[Value]) -> Result<()> {
        if state.len() != self.circuit.matrix_size() || state.iter().any(|value| !value.is_finite())
        {
            return Err(error("invalid physical circuit state"));
        }
        Ok(())
    }
}
