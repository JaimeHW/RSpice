//! Physical values and analytic derivatives of admitted expression equations.
use super::*;
use crate::expr::{CompiledExpr, Context, TimeDerivativeError, TimeDerivatives};

pub(super) fn unsupported(name: &str) -> SimulationError {
    error(format!(
        "behavioral source '{name}' requires a smooth prescribed physical time equation or a smooth nodal equation; unqualified branch-current controls, switched and stateful event providers remain unavailable"
    ))
}

impl PreparedEventCircuit<'_> {
    fn expression_retained_values(&self) -> usize {
        self.circuit
            .matrix_size()
            .saturating_mul(64)
            .saturating_add(self.circuit.capacitors.len().saturating_mul(6))
            .saturating_add(
                self.models
                    .len()
                    .saturating_mul(std::mem::size_of::<Bjt>().div_ceil(8)),
            )
            .saturating_add(self.ports.len().saturating_mul(2))
            .saturating_add(
                self.constant_sources
                    .len()
                    .saturating_add(self.circuit.voltage_sources.len())
                    .saturating_add(self.circuit.behavioral_sources.voltage_sources.len())
                    .saturating_mul(SOURCE_STORAGE_VALUES),
            )
    }

    pub(super) fn behavioral_sample(
        &self,
        name: &str,
        time: Value,
        options: &EventOptions,
        sample: impl FnOnce(
            usize,
        ) -> std::result::Result<
            crate::device::behavioral::PhysicalSample,
            TimeDerivativeError,
        >,
    ) -> Result<crate::device::behavioral::PhysicalSample> {
        let retained = self.expression_retained_values();
        sample(options.limits.max_result_values.saturating_sub(retained)).map_err(|failure| {
            match failure {
                TimeDerivativeError::Aborted => SimulationError::Aborted,
                TimeDerivativeError::Resource(mut failure) => {
                    failure.requested = failure.requested.saturating_add(retained);
                    failure.limit = options.limits.max_result_values;
                    SimulationError::ResourceLimit(failure)
                }
                failure => error(format!(
                    "behavioral source '{name}' at t={time:e}: {failure}"
                )),
            }
        })
    }

    pub(super) fn behavioral_time_values(
        &self,
        name: &str,
        program: Option<(&CompiledExpr, Context<'_>)>,
        time: Value,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<[Value; 2]> {
        check_abort(abort)?;
        let program = program.ok_or_else(|| unsupported(name))?;
        self.physical_time_values("behavioral source", name, program, time, options, abort)
    }

    pub(in crate::engine::transient) fn capacitor_values(
        &self,
        index: usize,
        time: Value,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<[Value; 2]> {
        check_abort(abort)?;
        let caps = &self.circuit.capacitors;
        let scale = caps.capacitances[index];
        let Some(expression) = &caps.value_expressions[index] else {
            return Ok([scale, 0.0]);
        };
        let program = expression.physical_time_program().ok_or_else(|| {
            error(format!(
                "capacitor '{}' requires a smooth prescribed event equation",
                caps.names[index]
            ))
        })?;
        let values = self.physical_time_values(
            "capacitor",
            &caps.names[index],
            program,
            time,
            options,
            abort,
        )?;
        let capacitance = sum([(scale, values[0])].into_iter())?;
        let slope = sum([(scale, values[1])].into_iter())?;
        if capacitance < 0.0 {
            return Err(error(format!(
                "capacitor '{}' has negative capacitance at t={time:e}",
                caps.names[index]
            )));
        }
        Ok([capacitance, slope])
    }

    fn physical_time_values(
        &self,
        kind: &str,
        name: &str,
        program: (&CompiledExpr, Context<'_>),
        time: Value,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<[Value; 2]> {
        let (program, mut context) = program;
        context.time = time;
        // Expression workspace is transient but coexists with prepared model,
        // topology and equation state; do not grant it the full limit again.
        let retained = self.expression_retained_values();
        let available = options.limits.max_result_values.saturating_sub(retained);
        let values = TimeDerivatives::new(program, 1, available, abort)
            .and_then(|plan| plan.evaluate(&context, abort))
            .map_err(|failure| match failure {
                TimeDerivativeError::Aborted => SimulationError::Aborted,
                TimeDerivativeError::Resource(mut failure) => {
                    failure.requested = failure.requested.saturating_add(retained);
                    failure.limit = options.limits.max_result_values;
                    SimulationError::ResourceLimit(failure)
                }
                failure => error(format!("{kind} '{name}' at t={time:e}: {failure}")),
            })?;
        Ok([values[0], values[1]])
    }
}
