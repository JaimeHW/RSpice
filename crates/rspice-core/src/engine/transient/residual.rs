//! Residual reconstruction helpers for transient nonlinear convergence.

use super::*;

impl Engine {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub(super) fn transient_nonlinear_residual_converged(
        &self,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        dt: Value,
        coeff: &CompanionCoefficients,
        current_method: IntegrationMethod,
        step_trap_order: u8,
        bjt_history: &BjtTransientHistory,
        vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
        jfet_history: &JfetTransientHistory,
        mosfet_history: &MosfetTransientHistory,
        suppress_gate_charge: bool,
        tline_dc_refs: &[(Value, Value)],
        coupled_tline_refs: &[CoupledTlineReferenceState],
    ) -> bool {
        if solution.iter().any(|value| !value.is_finite()) {
            return false;
        }

        let num_nodes = circuit.num_nodes();
        matrix.clear_values();
        rhs.fill(0.0);

        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        if gmin_floor > 0.0 {
            for i in 0..num_nodes {
                matrix.add(i, i, gmin_floor);
            }
        }

        circuit.stamp_transient_linear_direct(matrix, rhs);
        circuit
            .voltage_sources
            .update_transient_rhs(rhs, time, |br_ordinal| num_nodes + br_ordinal);
        circuit.current_sources.update_transient_rhs(rhs, time);
        circuit.refresh_jiles_atherton_inductances(solution);

        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(solution);
        }

        circuit
            .capacitors
            .stamp_transient_companion(matrix, rhs, dt, coeff);
        circuit
            .inductors
            .stamp_transient_companion(matrix, rhs, dt, coeff, num_nodes);
        circuit.stamp_coupled_inductor_pairs_transient(matrix, rhs, dt, coeff);
        circuit.stamp_multi_winding_transformers_transient(matrix, rhs, dt, coeff);

        Self::stamp_bjt_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            current_method,
            step_trap_order,
            dt,
            bjt_history,
            vbic_snapshot_cache,
            VbicCachedSnapshotReuse::SeedOnly,
            self.voltage_abstol(),
            self.voltage_reltol(),
        );
        Self::stamp_jfet_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            current_method,
            step_trap_order,
            dt,
            jfet_history,
            suppress_gate_charge,
        );
        Self::stamp_mosfet_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            current_method,
            step_trap_order,
            dt,
            mosfet_history,
            suppress_gate_charge,
        );
        Self::stamp_tline_companions(circuit, matrix, rhs, time, tline_dc_refs);
        Self::stamp_coupled_tline_companions(circuit, matrix, rhs, time, coupled_tline_refs);

        if circuit.has_nonlinear_devices() {
            circuit.stamp_nonlinear(matrix, rhs, solution);
            circuit.stamp_behavioral(matrix, rhs, solution, time);
        }

        if circuit.has_xspice_devices() {
            circuit.evaluate_xspice_with_timestep(time, dt, solution);
            circuit.stamp_xspice(matrix, rhs);
        }

        self.residual_convergence_met(matrix, solution, rhs)
    }

    #[inline]
    pub(super) fn should_prefer_dense_transient_solver(
        is_strictly_linear_transient: bool,
        size: usize,
        has_transformer_or_coupled_inductor: bool,
        has_xspice_devices: bool,
    ) -> bool {
        if is_strictly_linear_transient {
            return size <= 160 && has_transformer_or_coupled_inductor;
        }

        // Small nonlinear systems benefit from dense LU due to reduced sparse
        // symbolic/indirection overhead at this scale.
        !has_xspice_devices && size <= 64
    }
}
