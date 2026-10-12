//! Fallible initialization and copies of native BJT accepted state.

use super::*;

pub(in crate::engine::transient) fn reserve_values<T>(
    count: usize,
    object: &'static str,
) -> Result<Vec<T>, SimulationError> {
    let mut result = Vec::new();
    #[cfg(test)]
    if count != 0 {
        history_reservation_attempt()
            .map_err(|source| SimulationError::Allocation { object, source })?;
    }
    result
        .try_reserve_exact(count)
        .map_err(|source| SimulationError::Allocation { object, source })?;
    Ok(result)
}

fn copy_values<T: Copy>(source: &[T]) -> Result<Vec<T>, SimulationError> {
    let mut result = reserve_values(source.len(), "BJT accepted history")?;
    result.extend_from_slice(source);
    Ok(result)
}

impl BjtTransientHistory {
    #[cfg(test)]
    pub(in crate::engine) fn with_allocation_failure_after<T>(
        count: usize,
        operation: impl FnOnce() -> T,
    ) -> T {
        struct Reset;
        impl Drop for Reset {
            fn drop(&mut self) {
                HISTORY_RESERVATIONS_BEFORE_FAILURE.set(None);
            }
        }
        HISTORY_RESERVATIONS_BEFORE_FAILURE.set(Some(count));
        let _reset = Reset;
        operation()
    }

    /// Reserve every fixed lane before reading bias or changing accepted state.
    /// Numeric lanes are filled by `seed_bjt_history`; phase owners and accepted
    /// external current already have their zero/empty initial values.
    pub(in crate::engine) fn try_unseeded(count: usize) -> Result<Self, SimulationError> {
        fn lane<T>(count: usize) -> Result<Vec<T>, SimulationError> {
            reserve_values(count, "BJT state initialization")
        }
        let mut history = Self {
            phase: lane(count)?,
            weil_phase: lane(count)?,
            phase_outgoing_slopes: lane(count)?,
            vbe_prev: lane(count)?,
            vbe_prev_prev: lane(count)?,
            ibe_prev: lane(count)?,
            vbc_prev: lane(count)?,
            vbc_prev_prev: lane(count)?,
            ibc_prev: lane(count)?,
            vcs_prev: lane(count)?,
            vcs_prev_prev: lane(count)?,
            ics_prev: lane(count)?,
            charge_q_prev: lane(count)?,
            charge_q_prev_prev: lane(count)?,
            charge_q_prev_prev_prev: lane(count)?,
            charge_cq_prev: lane(count)?,
            accepted_external_bc_current: lane(count)?,
            accepted_terminal_currents: lane(count)?,
            dynamic_internal_prev: lane(count)?,
            dynamic_internal_prev_prev: lane(count)?,
            dynamic_linear_prev: lane(count)?,
            dynamic_linear_prev_prev: lane(count)?,
            accepted_dt_prev: 0.0,
            accepted_dt_prev_prev: 0.0,
        };
        history.phase.resize_with(count, || None);
        history.weil_phase.resize(count, None);
        history.phase_outgoing_slopes.resize(count, None);
        history.accepted_external_bc_current.resize(count, 0.0);
        Ok(history)
    }

    pub(in crate::engine::transient) fn try_clone(&self) -> Result<Self, SimulationError> {
        let mut phase = reserve_values(self.phase.len(), "BJT phase-history owners")?;
        for history in &self.phase {
            phase.push(
                history
                    .as_ref()
                    .map(|history| history.try_clone())
                    .transpose()
                    .map_err(|source| SimulationError::Allocation {
                        object: "BJT phase-history copy",
                        source,
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

#[cfg(test)]
thread_local! {
    static HISTORY_RESERVATIONS_BEFORE_FAILURE: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
fn history_reservation_attempt() -> Result<(), std::collections::TryReserveError> {
    HISTORY_RESERVATIONS_BEFORE_FAILURE.with(|remaining| match remaining.get() {
        Some(0) => {
            remaining.set(None);
            Vec::<u8>::new().try_reserve(usize::MAX)
        }
        Some(count) => {
            remaining.set(Some(count - 1));
            Ok(())
        }
        None => Ok(()),
    })
}

#[cfg(test)]
mod tests;
