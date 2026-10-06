//! Physical values and analytic slopes of admitted prescribed B-sources.
use super::*;
use crate::expr::{CompiledExpr, Context, TimeDerivativeError, TimeDerivatives};

pub(super) fn unsupported(name: &str) -> SimulationError {
    error(format!(
        "behavioral source '{name}' requires a smooth prescribed physical time equation; circuit-dependent, switched and stateful event providers remain unavailable"
    ))
}

impl PreparedEventCircuit<'_> {
    pub(super) fn behavioral_time_values(
        &self,
        name: &str,
        program: Option<(&CompiledExpr, Context<'_>)>,
        time: Value,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<[Value; 2]> {
        check_abort(abort)?;
        let (program, mut context) = program.ok_or_else(|| unsupported(name))?;
        context.time = time;
        // Expression workspace is transient but coexists with prepared model,
        // topology and equation state; do not grant it the full limit again.
        let retained = self
            .circuit
            .matrix_size()
            .saturating_mul(64)
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
                    .saturating_mul(8),
            );
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
                failure => error(format!(
                    "behavioral source '{name}' at t={time:e}: {failure}"
                )),
            })?;
        Ok([values[0], values[1]])
    }
}
