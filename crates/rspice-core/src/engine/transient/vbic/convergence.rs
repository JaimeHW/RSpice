//! VBIC local convergence and external-bias matching predicates.

#![allow(clippy::needless_range_loop)]

use super::*;

impl Engine {
    /// ngspice-style hidden-state delta check used for device-local bypass
    /// diagnostics. In ngspice, these `hat` comparisons decide whether the
    /// device can reuse the previous linearization; they are not a standalone
    /// Newton acceptance gate after the internal nodes have been reduced out of
    /// the global system.
    #[inline]
    pub(in crate::engine::transient) fn vbic_snapshot_convergence_met(
        bjt: &crate::device::Bjt,
        previous_external: [Value; BJT_EXTERNAL_STATE_DIM],
        previous_snapshot: &BjtChargeSnapshot,
        current_external: [Value; BJT_EXTERNAL_STATE_DIM],
        current_snapshot: &BjtChargeSnapshot,
        criteria: NonlinearConvergenceCriteria,
    ) -> bool {
        let previous = bjt.vbic_transient_convergence_state_for_snapshot(
            previous_external[0],
            previous_external[1],
            previous_external[2],
            previous_external[3],
            previous_snapshot,
        );
        let current = bjt.vbic_transient_convergence_state_for_snapshot(
            current_external[0],
            current_external[1],
            current_external[2],
            current_external[3],
            current_snapshot,
        );
        let reltol = criteria.relative_tolerance();
        let voltage_tol = criteria.voltage_tolerance();
        let current_tol = criteria.current_tolerance();

        let voltages_converged = current.voltages.iter().zip(previous.voltages.iter()).all(
            |(current_voltage, previous_voltage)| {
                let diff = (current_voltage - previous_voltage).abs();
                let tol = reltol * current_voltage.abs().max(previous_voltage.abs()) + voltage_tol;
                diff <= tol
            },
        );
        if !voltages_converged {
            return false;
        }

        let mut delta_internal = [0.0; BJT_INTERNAL_STATE_DIM];
        for idx in 0..BJT_INTERNAL_STATE_DIM {
            delta_internal[idx] = current_snapshot.reduction.internal_voltages[idx]
                - previous_snapshot.reduction.internal_voltages[idx];
        }

        (0..VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT).all(|branch_idx| {
            // Mirror ngspice's VBIC load-time bypass check: compare the full
            // predicted branch current against the candidate branch current,
            // including the hidden excess-phase xf2 contribution in iciei.
            // Excluding that term can accept a stale delayed-transport state
            // even when the candidate misses the device-local predictor tolerances.
            let predicted = previous.currents[branch_idx]
                + previous.d_currents_d_internal[branch_idx]
                    .iter()
                    .zip(delta_internal.iter())
                    .enumerate()
                    .filter(|(idx, _)| *idx != BJT_THERMAL_STATE_INDEX)
                    .map(|(_, (derivative, delta))| derivative * delta)
                    .sum::<Value>();
            let actual = current.currents[branch_idx];
            let tol = reltol * predicted.abs().max(actual.abs()) + current_tol;
            (predicted - actual).abs() <= tol
        })
    }

    #[inline]
    pub(in crate::engine::transient) fn vbic_local_candidate_is_acceptable(
        bjt: &crate::device::Bjt,
        previous_external: [Value; BJT_EXTERNAL_STATE_DIM],
        previous_snapshot: &BjtChargeSnapshot,
        candidate_snapshot: &BjtChargeSnapshot,
        candidate_linearization: &VbicTransientLinearization,
    ) -> bool {
        if Self::vbic_dynamic_snapshot_solution_is_acceptable(
            candidate_linearization,
            &candidate_snapshot.reduction.external_voltages,
            &candidate_snapshot.reduction.internal_voltages,
        ) {
            return true;
        }

        // ngspice's VBIC path ultimately accepts or bypasses local updates based on
        // branch/voltage predictor tolerances (`*_hat` checks), not on a separate
        // reduced hidden-state residual. During our local continuation fallback, a
        // candidate that meets those ngspice-style device tolerances should be
        // accepted even when the reduced internal equations are stricter.
        Self::vbic_snapshot_convergence_met(
            bjt,
            previous_external,
            previous_snapshot,
            candidate_snapshot.reduction.external_voltages,
            candidate_snapshot,
            NonlinearConvergenceCriteria::default(),
        )
    }

    #[inline]
    pub(in crate::engine::transient) fn vbic_continuation_candidate_is_acceptable(
        bjt: &crate::device::Bjt,
        previous_external: [Value; BJT_EXTERNAL_STATE_DIM],
        previous_snapshot: &BjtChargeSnapshot,
        candidate_snapshot: &BjtChargeSnapshot,
        candidate_linearization: &VbicTransientLinearization,
    ) -> bool {
        Self::vbic_local_candidate_is_acceptable(
            bjt,
            previous_external,
            previous_snapshot,
            candidate_snapshot,
            candidate_linearization,
        )
    }

    #[inline]
    pub(in crate::engine::transient) fn vbic_homotopy_candidate_is_acceptable(
        bjt: &crate::device::Bjt,
        external: [Value; BJT_EXTERNAL_STATE_DIM],
        previous_internal: [Value; BJT_INTERNAL_STATE_DIM],
        candidate_snapshot: &BjtChargeSnapshot,
        candidate_linearization: &VbicTransientLinearization,
    ) -> bool {
        let previous_snapshot = bjt.charge_snapshot_for_dynamic_state(
            external[0],
            external[1],
            external[2],
            external[3],
            previous_internal,
        );
        Self::vbic_local_candidate_is_acceptable(
            bjt,
            external,
            &previous_snapshot,
            candidate_snapshot,
            candidate_linearization,
        )
    }

    #[inline]
    pub(in crate::engine::transient) fn transient_static_device_convergence_met(
        &self,
        circuit: &crate::circuit::Circuit,
    ) -> bool {
        let criteria = self.device_convergence_criteria();
        #[cfg(feature = "veriloga-builtins")]
        let generated_veriloga_converged = circuit.generated_veriloga_devices.all_converged();
        #[cfg(not(feature = "veriloga-builtins"))]
        let generated_veriloga_converged = true;
        #[cfg(feature = "veriloga")]
        let dynamic_veriloga_converged = circuit.veriloga_devices.all_converged();
        #[cfg(not(feature = "veriloga"))]
        let dynamic_veriloga_converged = true;

        circuit.diodes.all_converged(criteria)
            && circuit.mosfets.all_converged(criteria)
            && circuit.jfets.iter().all(|jfet| jfet.is_converged(criteria))
            && circuit.vswitches.iter().all(|sw| sw.is_converged(criteria))
            && circuit.iswitches.iter().all(|sw| sw.is_converged(criteria))
            && circuit.xspice_converged(criteria.voltage_tolerance())
            && circuit.bjts.all_converged(criteria)
            && dynamic_veriloga_converged
            && generated_veriloga_converged
    }

    #[inline]
    pub(in crate::engine::transient) fn vbic_snapshot_matches_external_bias(
        snapshot: &BjtChargeSnapshot,
        external: &[Value; BJT_EXTERNAL_STATE_DIM],
        voltage_abstol: Value,
        reltol: Value,
    ) -> bool {
        Self::check_voltage_convergence_with_tolerances(
            &snapshot.reduction.external_voltages,
            external,
            voltage_abstol,
            reltol,
        )
    }

    #[inline]
    pub(in crate::engine::transient) fn vbic_snapshot_matches_external_bias_exact(
        snapshot: &BjtChargeSnapshot,
        external: &[Value; BJT_EXTERNAL_STATE_DIM],
    ) -> bool {
        snapshot
            .reduction
            .external_voltages
            .iter()
            .zip(external.iter())
            .all(|(cached, expected)| (*cached - *expected).abs() <= 1e-18)
    }

    #[inline]
    pub(in crate::engine::transient) fn vbic_runtime_snapshot_reuse_tolerances(
        voltage_abstol: Value,
        reltol: Value,
    ) -> (Value, Value) {
        (voltage_abstol, reltol)
    }
}
