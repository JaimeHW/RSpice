//! Fallible copies of native BJT accepted state, including transport records.

use super::*;

fn reserve_values<T>(count: usize, object: &'static str) -> Result<Vec<T>, SimulationError> {
    let mut result = Vec::new();
    #[cfg(test)]
    if count != 0 {
        copy_reservation_attempt()
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
    static COPY_RESERVATIONS_BEFORE_FAILURE: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
fn copy_reservation_attempt() -> Result<(), std::collections::TryReserveError> {
    COPY_RESERVATIONS_BEFORE_FAILURE.with(|remaining| match remaining.get() {
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
mod tests {
    use super::*;
    use crate::{SimulationConfig, SimulationErrorCategory, SimulationErrorCode};

    fn fail_copy_after<T>(count: usize, operation: impl FnOnce() -> T) -> T {
        struct Reset;
        impl Drop for Reset {
            fn drop(&mut self) {
                COPY_RESERVATIONS_BEFORE_FAILURE.set(None);
            }
        }
        COPY_RESERVATIONS_BEFORE_FAILURE.set(Some(count));
        let _reset = Reset;
        operation()
    }

    fn assert_allocation(error: &SimulationError) {
        assert!(
            matches!(error, SimulationError::Allocation { .. }),
            "{error}"
        );
        let descriptor = error.descriptor();
        assert_eq!(descriptor.code, SimulationErrorCode::AllocationFailed);
        assert_eq!(descriptor.code.as_str(), "allocation_failed");
        assert_eq!(descriptor.category, SimulationErrorCategory::ResourceLimit);
        assert!(descriptor.resource_limit.is_none());
        assert!(!descriptor.retryable);
        assert!(std::error::Error::source(error).is_some());
    }

    #[test]
    fn impossible_history_reservation_keeps_the_allocator_failure_typed() {
        let error = reserve_values::<u8>(usize::MAX, "BJT test history").unwrap_err();
        assert_allocation(&error);
    }

    #[test]
    fn public_capture_and_restore_preserve_allocation_failures_and_accepted_state() {
        let netlist = Netlist::parse("phase copy failures\nVC c 0 2\nVB b 0 .7\nQ1 c b 0 qm\n.model qm NPN IS=1e-16 BF=100 TF=1n PTF=90\n.end\n").unwrap();
        let engine = Engine::new(SimulationConfig::default());
        // Count the startup copies through the same public run route, so each
        // capture fault lands in final checkpoint materialization rather than
        // accidentally testing an earlier physical-startup copy.
        let (baseline, startup_copies) = fail_copy_after(usize::MAX, || {
            let result = engine.run_tran(&netlist, 1e-10, 1e-12).unwrap();
            (
                result,
                usize::MAX - COPY_RESERVATIONS_BEFORE_FAILURE.get().unwrap(),
            )
        });
        let (_, checkpoint) = engine
            .run_tran_checkpointed(&netlist, 1e-10, 1e-12)
            .unwrap();
        let encoded = checkpoint.to_text();
        let mut limited_config = SimulationConfig::default();
        limited_config.resource_limits.max_transport_history_bytes = 0;
        let limited = Engine::new(limited_config);
        let policy_error = fail_copy_after(0, || {
            let result = limited.run_tran_resume(&netlist, &checkpoint, 2e-10, 1e-12);
            assert_eq!(COPY_RESERVATIONS_BEFORE_FAILURE.get(), Some(0));
            result.unwrap_err()
        });
        assert!(matches!(policy_error, SimulationError::ResourceLimit(error)
            if error.resource == crate::ResourceKind::TransportHistoryBytes));

        let (continued, _) = engine
            .run_tran_resume(&netlist, &checkpoint, 2e-10, 1e-12)
            .unwrap();
        for count in [0, 1, 5, 12] {
            let failed_capture = fail_copy_after(startup_copies + count, || {
                engine.run_tran_checkpointed(&netlist, 1e-10, 1e-12)
            });
            assert_allocation(&failed_capture.unwrap_err());
            let failed_restore = fail_copy_after(count, || {
                engine.run_tran_resume(&netlist, &checkpoint, 2e-10, 1e-12)
            });
            assert_allocation(&failed_restore.unwrap_err());
            assert_eq!(checkpoint.to_text(), encoded);
        }
        let rerun = engine.run_tran(&netlist, 1e-10, 1e-12).unwrap();
        assert_eq!(rerun.time, baseline.time);
        assert_eq!(rerun.voltages, baseline.voltages);
        assert_eq!(rerun.branch_currents, baseline.branch_currents);
        let (resumed, _) = engine
            .run_tran_resume(&netlist, &checkpoint, 2e-10, 1e-12)
            .unwrap();
        assert_eq!(resumed.time, continued.time);
        assert_eq!(resumed.voltages, continued.voltages);
        assert_eq!(resumed.branch_currents, continued.branch_currents);
    }
}
