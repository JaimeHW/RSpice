//! Dimensioned tolerances for the prepared event equations.
use super::*;
use crate::analysis::quasi_periodic::SpectralEnvelopeEventEquation as Row;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct EnvelopeEventTolerances {
    pub charge_coulombs: Value,
    pub flux_webers: Value,
    pub current_amperes: Value,
    pub voltage_volts: Value,
    pub current_rate_amperes_per_second: Value,
    pub voltage_rate_volts_per_second: Value,
}

impl Default for EnvelopeEventTolerances {
    fn default() -> Self {
        Self {
            charge_coulombs: 1e-15,
            flux_webers: 1e-15,
            current_amperes: 1e-12,
            voltage_volts: 1e-9,
            current_rate_amperes_per_second: 1e-6,
            voltage_rate_volts_per_second: 1e-3,
        }
    }
}

impl EnvelopeEventTolerances {
    pub fn validate(&self) -> Result<(), SimulationError> {
        if [
            self.charge_coulombs,
            self.flux_webers,
            self.current_amperes,
            self.voltage_volts,
            self.current_rate_amperes_per_second,
            self.voltage_rate_volts_per_second,
        ]
        .iter()
        .any(|v| !v.is_finite() || *v <= 0.0)
        {
            return Err(invalid("event tolerances must be positive and finite"));
        }
        Ok(())
    }
}

impl PreparedSpectralEnvelope {
    /// Resolve physical units from structural row ownership before a mission
    /// starts. In particular, an algebraic KCL derivative uses A/s, whereas
    /// an outgoing charge equation uses A. No artificial timestep converts
    /// one tolerance into the other.
    pub fn event_config_with_abort(
        &self,
        tolerance: &EnvelopeEventTolerances,
        abort: &dyn AbortSignal,
    ) -> Result<SpectralEnvelopeEventConfig, SimulationError> {
        check_abort(abort)?;
        tolerance.validate()?;
        let unknowns = self.carrier_sources.len();
        let limits = source_workspace_limits(unknowns, self.grid(), &self.limits)
            .map_err(numerical_error)?;
        crate::ResourceLimitError::ensure(
            crate::ResourceKind::ResultValues,
            unknowns.saturating_mul(18).saturating_add(1),
            limits.max_result_values,
        )?;
        // No event equations are consumed on an event-free mission. Avoid
        // restricting smooth nonlinear solves to the event topology subset.
        let has_events = self.next_source_event_after(0.0)?.is_some()
            || self.source_events_at(0.0)?.next().is_some();
        let rows = if has_events {
            Some(event_topology::prepare(&self.circuit, unknowns, abort)?)
        } else {
            None
        };
        let mut charge_tolerances = Vec::with_capacity(unknowns);
        let mut rate_tolerances = Vec::with_capacity(unknowns);
        for row in 0..unknowns {
            check_abort(abort)?;
            let nodal = row < self.node_names().len();
            charge_tolerances.push(if nodal {
                tolerance.charge_coulombs
            } else {
                tolerance.flux_webers
            });
            rate_tolerances.push(match rows.as_ref().map(|rows| &rows[row]) {
                Some(Row::VoltageSource { .. }) => tolerance.voltage_rate_volts_per_second,
                Some(Row::Algebraic(_)) if nodal => tolerance.current_rate_amperes_per_second,
                Some(Row::Algebraic(_)) => tolerance.voltage_rate_volts_per_second,
                _ if nodal => tolerance.current_amperes,
                _ => tolerance.voltage_volts,
            });
        }
        Ok(SpectralEnvelopeEventConfig {
            solver: self.config.solver.clone(),
            charge_tolerances,
            rate_tolerances,
        })
    }
}
