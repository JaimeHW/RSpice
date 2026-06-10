//! Shared transient system assembly and residual reconstruction.
//!
//! The Newton loop, the restamp residual check, and the gmin-continuation
//! rescue all need the same equations: one assembly routine stamps the full
//! transient system (linear, companion, nonlinear, behavioral, XSPICE) at a
//! given iterate so the three consumers can never drift apart.

use super::*;

/// Per-step invariants of the transient system assembly. Holds borrows of
/// the integration coefficients and the per-device-family histories, so a
/// context is constructed locally at each use site (the histories are
/// mutated when a step is accepted) and dropped before the commit walks.
pub(super) struct TransientSystemContext<'a> {
    pub(super) coeff: &'a CompanionCoefficients,
    pub(super) method: IntegrationMethod,
    pub(super) trap_order: u8,
    pub(super) bjt_history: &'a BjtTransientHistory,
    pub(super) jfet_history: &'a JfetTransientHistory,
    pub(super) mosfet_history: &'a MosfetTransientHistory,
    pub(super) b3soi_history: &'a B3SoiTransientHistory,
    pub(super) suppress_gate_charge: bool,
    pub(super) tline_dc_refs: &'a [(Value, Value)],
    pub(super) coupled_tline_refs: &'a [CoupledTlineReferenceState],
}

impl Engine {
    /// Assemble the complete transient system `A(x)·x = b(x)` at `solution`
    /// for the timepoint `time` reached with step `dt`.
    ///
    /// `extra_diag_gmin` adds a continuation shunt on top of the baseline
    /// GMIN; both land only on node-voltage equations — branch-current
    /// equations (voltage source/inductor branches) must not receive the
    /// shunt or transient references are biased. `refresh_nonlinear` lets
    /// the Newton loop skip the device re-evaluation when its state already
    /// matches `solution`.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn stamp_transient_system(
        &self,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        dt: Value,
        ctx: &TransientSystemContext<'_>,
        vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
        vbic_reuse: VbicCachedSnapshotReuse,
        refresh_nonlinear: bool,
        extra_diag_gmin: Value,
    ) {
        let num_nodes = circuit.num_nodes();
        matrix.clear_values();
        rhs.fill(0.0);

        let diag_gmin = self.config.convergence_config.gmin_target.max(0.0) + extra_diag_gmin;
        if diag_gmin > 0.0 {
            for i in 0..num_nodes {
                matrix.add(i, i, diag_gmin);
            }
        }

        // Stamp linear devices (R, V, I) for transient; tline transient
        // behavior is stamped separately via companions.
        circuit.stamp_transient_linear_direct(matrix, rhs);
        circuit
            .voltage_sources
            .update_transient_rhs(rhs, time, |br_ordinal| num_nodes + br_ordinal);
        circuit.current_sources.update_transient_rhs(rhs, time);

        circuit.refresh_jiles_atherton_inductances(solution);
        if refresh_nonlinear && circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(solution);
        }

        circuit
            .capacitors
            .stamp_transient_companion(matrix, rhs, dt, ctx.coeff);
        circuit
            .inductors
            .stamp_transient_companion(matrix, rhs, dt, ctx.coeff, num_nodes);
        circuit.stamp_coupled_inductor_pairs_transient(matrix, rhs, dt, ctx.coeff);
        circuit.stamp_multi_winding_transformers_transient(matrix, rhs, dt, ctx.coeff);

        Self::stamp_bjt_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            ctx.method,
            ctx.trap_order,
            dt,
            ctx.bjt_history,
            vbic_snapshot_cache,
            vbic_reuse,
            self.voltage_abstol(),
            self.voltage_reltol(),
        );
        Self::stamp_jfet_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            ctx.method,
            ctx.trap_order,
            dt,
            ctx.jfet_history,
            ctx.suppress_gate_charge,
        );
        Self::stamp_mosfet_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            ctx.method,
            ctx.trap_order,
            dt,
            ctx.mosfet_history,
            ctx.suppress_gate_charge,
        );
        Self::stamp_b3soi_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            ctx.method,
            ctx.trap_order,
            dt,
            ctx.b3soi_history,
        );
        Self::stamp_tline_companions(circuit, matrix, rhs, time, ctx.tline_dc_refs);
        Self::stamp_coupled_tline_companions(circuit, matrix, rhs, time, dt, ctx.coupled_tline_refs);

        if circuit.has_nonlinear_devices() {
            circuit.stamp_nonlinear(matrix, rhs, solution);
            circuit.stamp_behavioral(matrix, rhs, solution, time);
        }

        if circuit.has_xspice_devices() {
            circuit.evaluate_xspice_with_timestep(time, dt, solution);
            circuit.stamp_xspice(matrix, rhs);
        }
    }

    /// Restamp the full system at `solution` and test the true nonlinear
    /// residual against the engine's convergence scaling.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn transient_nonlinear_residual_converged(
        &self,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        dt: Value,
        ctx: &TransientSystemContext<'_>,
        vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
    ) -> bool {
        if solution.iter().any(|value| !value.is_finite()) {
            return false;
        }

        self.stamp_transient_system(
            circuit,
            matrix,
            rhs,
            solution,
            time,
            dt,
            ctx,
            vbic_snapshot_cache,
            VbicCachedSnapshotReuse::SeedOnly,
            true,
            0.0,
        );
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
