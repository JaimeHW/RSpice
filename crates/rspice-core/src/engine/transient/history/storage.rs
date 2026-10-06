//! Fallible copies of native BJT accepted state, including transport records.

use super::*;

fn copy_values<T: Copy>(source: &[T]) -> Result<Vec<T>, String> {
    let mut result = Vec::new();
    result
        .try_reserve_exact(source.len())
        .map_err(|error| format!("BJT accepted-history allocation failed: {error}"))?;
    result.extend_from_slice(source);
    Ok(result)
}

impl BjtTransientHistory {
    pub(in crate::engine::transient) fn try_clone(&self) -> Result<Self, String> {
        let mut phase = Vec::new();
        phase
            .try_reserve_exact(self.phase.len())
            .map_err(|error| format!("BJT phase-history owner allocation failed: {error}"))?;
        for history in &self.phase {
            phase.push(
                history
                    .as_ref()
                    .map(|history| history.try_clone())
                    .transpose()
                    .map_err(|error| {
                        format!("BJT phase-history copy allocation failed: {error}")
                    })?,
            );
        }
        Ok(Self {
            phase,
            weil_phase: copy_values(&self.weil_phase)?,
            phase_outgoing_slopes: copy_values(&self.phase_outgoing_slopes)?,
            vbe_prev: copy_values(&self.vbe_prev)?,
            vbe_prev_prev: copy_values(&self.vbe_prev_prev)?,
            ibe_prev: copy_values(&self.ibe_prev)?,
            vbc_prev: copy_values(&self.vbc_prev)?,
            vbc_prev_prev: copy_values(&self.vbc_prev_prev)?,
            ibc_prev: copy_values(&self.ibc_prev)?,
            vcs_prev: copy_values(&self.vcs_prev)?,
            vcs_prev_prev: copy_values(&self.vcs_prev_prev)?,
            ics_prev: copy_values(&self.ics_prev)?,
            charge_q_prev: copy_values(&self.charge_q_prev)?,
            charge_q_prev_prev: copy_values(&self.charge_q_prev_prev)?,
            charge_q_prev_prev_prev: copy_values(&self.charge_q_prev_prev_prev)?,
            charge_cq_prev: copy_values(&self.charge_cq_prev)?,
            accepted_external_bc_current: copy_values(&self.accepted_external_bc_current)?,
            accepted_terminal_currents: copy_values(&self.accepted_terminal_currents)?,
            dynamic_internal_prev: copy_values(&self.dynamic_internal_prev)?,
            dynamic_internal_prev_prev: copy_values(&self.dynamic_internal_prev_prev)?,
            dynamic_linear_prev: copy_values(&self.dynamic_linear_prev)?,
            dynamic_linear_prev_prev: copy_values(&self.dynamic_linear_prev_prev)?,
            accepted_dt_prev: self.accepted_dt_prev,
            accepted_dt_prev_prev: self.accepted_dt_prev_prev,
        })
    }
}
