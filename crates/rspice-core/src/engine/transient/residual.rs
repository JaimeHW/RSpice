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
    /// Xyce's OneStep method uses a backward-Euler reactive companion for
    /// both order-one and order-two steps. Order two additionally splits the
    /// static DAE contribution, represented by `xyce_one_step_order2` below.
    pub(super) xyce_one_step: bool,
    /// Xyce's OneStep order-2 residual uses a backward-Euler Q difference
    /// together with a trapezoid-weighted static F term.  The accepted
    /// previous static residual is carried separately because it is not part
    /// of the reactive companion histories.
    pub(super) xyce_one_step_order2: bool,
    /// Use the bounded Picard form for shared Xyce LEVEL=2 cores. The
    /// residual and physical constitutive endpoint remain unchanged; only
    /// the local Jacobian is stabilized for this coupled topology.
    pub(super) xyce_static_history: Option<&'a [Value]>,
    pub(super) bsim4_trnqs_coeff: &'a CompanionCoefficients,
    pub(super) bjt_history: &'a BjtTransientHistory,
    pub(super) jfet_history: &'a JfetTransientHistory,
    pub(super) diode_history: &'a DiodeTransientHistory,
    pub(super) diode_companion_slots: &'a [TwoTerminalStampSlots],
    pub(super) mosfet_history: &'a MosfetTransientHistory,
    pub(super) mosfet_companion_slots: &'a [[TwoTerminalStampSlots; 5]],
    pub(super) vdmos_history: &'a VdmosTransientHistory,
    pub(super) vdmos_companion_slots: &'a [[TwoTerminalStampSlots; 7]],
    pub(super) b3soi_history: &'a B3SoiTransientHistory,
    pub(super) b3soi_zero_first_transient_charge_derivative: bool,
    pub(super) bsim3_history: &'a Bsim3TransientHistory,
    pub(super) bsim4_history: &'a Bsim4TransientHistory,
    pub(super) ekv26_history: &'a Ekv26TransientHistory,
    pub(super) suppress_gate_charge: bool,
    pub(super) baseline_diag_gmin: Value,
    pub(super) tline_dc_refs: &'a [(Value, Value)],
    pub(super) coupled_tline_refs: &'a [CoupledTlineReferenceState],
    #[cfg_attr(
        not(any(feature = "veriloga", feature = "veriloga-builtins-base")),
        allow(dead_code)
    )]
    pub(super) analysis_initial_step: bool,
    #[cfg_attr(
        not(any(feature = "veriloga", feature = "veriloga-builtins-base")),
        allow(dead_code)
    )]
    pub(super) analysis_final_step: bool,
}

/// Two-level numeric cache for the classic-MOS transient fast path.
///
/// `static_*` contains the run-invariant linear network and nodal GMIN.
/// `attempt_*` additionally contains sources evaluated at the trial time and
/// ordinary capacitor companions for the current integration coefficients.
/// Each Newton/proof stamp restores the latter with contiguous copies, then
/// adds only MOS charge and conduction terms.
#[derive(Default)]
pub(super) struct ClassicMosTransientStampCache {
    static_values: Vec<Value>,
    static_rhs: Vec<Value>,
    attempt_values: Vec<Value>,
    attempt_rhs: Vec<Value>,
}

impl ClassicMosTransientStampCache {
    #[inline]
    fn capture_static(&mut self, matrix: &mut crate::solver::StaticMatrix, rhs: &[Value]) {
        self.static_values.clear();
        self.static_values.extend_from_slice(matrix.values_mut());
        self.static_rhs.clear();
        self.static_rhs.extend_from_slice(rhs);
    }

    #[inline]
    fn restore_static(&self, matrix: &mut crate::solver::StaticMatrix, rhs: &mut [Value]) {
        debug_assert_eq!(matrix.values_mut().len(), self.static_values.len());
        debug_assert_eq!(rhs.len(), self.static_rhs.len());
        matrix.values_mut().copy_from_slice(&self.static_values);
        rhs.copy_from_slice(&self.static_rhs);
    }

    #[inline]
    fn capture_attempt(&mut self, matrix: &mut crate::solver::StaticMatrix, rhs: &[Value]) {
        self.attempt_values.clear();
        self.attempt_values.extend_from_slice(matrix.values_mut());
        self.attempt_rhs.clear();
        self.attempt_rhs.extend_from_slice(rhs);
    }

    #[inline]
    fn restore_attempt(&self, matrix: &mut crate::solver::StaticMatrix, rhs: &mut [Value]) {
        debug_assert_eq!(matrix.values_mut().len(), self.attempt_values.len());
        debug_assert_eq!(rhs.len(), self.attempt_rhs.len());
        matrix.values_mut().copy_from_slice(&self.attempt_values);
        rhs.copy_from_slice(&self.attempt_rhs);
    }
}

impl Engine {
    /// Capture the run-invariant part of the restricted classic-MOS transient
    /// topology. The caller has already checked the fail-closed capability.
    pub(super) fn initialize_classic_mos_transient_stamp_cache(
        &self,
        circuit: &crate::circuit::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        baseline_diag_gmin: Value,
    ) -> ClassicMosTransientStampCache {
        debug_assert!(circuit.has_cacheable_classic_mos_transient_base());
        matrix.clear_values();
        rhs.fill(0.0);
        Self::stamp_nodal_gmin(circuit, matrix, baseline_diag_gmin.max(0.0));
        circuit.stamp_transient_linear_direct(matrix, rhs);

        let mut cache = ClassicMosTransientStampCache::default();
        cache.capture_static(matrix, rhs);
        cache
    }

    /// Extend the static cache with the solution-independent terms that are
    /// fixed for one attempted timepoint.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn prepare_classic_mos_transient_attempt(
        &self,
        cache: &mut ClassicMosTransientStampCache,
        circuit: &crate::circuit::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        time: Value,
        dt: Value,
        coeff: &CompanionCoefficients,
        xyce_one_step: bool,
    ) {
        debug_assert!(circuit.has_cacheable_classic_mos_transient_base());
        cache.restore_static(matrix, rhs);

        let num_nodes = circuit.num_nodes();
        circuit
            .voltage_sources
            .update_transient_rhs(rhs, time, |br_ordinal| num_nodes + br_ordinal);
        circuit.current_sources.update_transient_rhs(rhs, time);
        let companion_coeff = if xyce_one_step {
            CompanionCoefficients::backward_euler()
        } else {
            *coeff
        };
        circuit
            .capacitors
            .stamp_transient_companion(matrix, rhs, dt, &companion_coeff, num_nodes);
        cache.capture_attempt(matrix, rhs);
    }

    /// Assemble one classic-MOS Newton or exact-candidate residual system from
    /// the prepared base. This is equation-for-equation equivalent to the
    /// general assembler for the guarded topology.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn stamp_classic_mos_transient_system_from_cache(
        &self,
        cache: &ClassicMosTransientStampCache,
        circuit: &mut crate::circuit::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        dt: Value,
        ctx: &TransientSystemContext<'_>,
        refresh_nonlinear: bool,
        evaluation_mode: crate::device::veriloga_builtins::GeneratedEvaluationMode,
        caps_cache_out: Option<&mut Vec<(Value, Value, Value)>>,
    ) -> Result<(), SimulationError> {
        debug_assert!(circuit.has_cacheable_classic_mos_transient_base());
        debug_assert!(!ctx.xyce_one_step_order2);
        cache.restore_attempt(matrix, rhs);

        if refresh_nonlinear {
            self.update_transient_nonlinear_devices(circuit, solution)?;
        }
        let companion_coeff = if ctx.xyce_one_step {
            CompanionCoefficients::backward_euler()
        } else {
            *ctx.coeff
        };
        Self::stamp_mosfet_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            &companion_coeff,
            dt,
            ctx.mosfet_history,
            ctx.suppress_gate_charge,
            ctx.mosfet_companion_slots,
            caps_cache_out,
        );
        if evaluation_mode == crate::device::veriloga_builtins::GeneratedEvaluationMode::StaticProbe
        {
            circuit
                .mosfets
                .stamp_all_static_probe_direct(matrix, rhs, solution);
        } else {
            circuit.mosfets.stamp_all_direct(matrix, rhs, solution);
        }
        Ok(())
    }

    /// Refresh nonlinear trial state, parallelizing the model-batched classic
    /// MOS family only when its population amortizes bounded-pool dispatch.
    pub(super) fn update_transient_nonlinear_devices(
        &self,
        circuit: &mut crate::circuit::CircuitData,
        solution: &[Value],
    ) -> Result<(), SimulationError> {
        #[cfg(feature = "parallel")]
        {
            const PARALLEL_CLASSIC_MOS_THRESHOLD: usize = 2_048;
            // Level-1/2/3 MOS evaluation is a short, memory-bound kernel.
            // Host-wide task fanout regressed MOS4096 on a 28-thread machine;
            // one coarse task per 512 contiguous instances reached the best
            // measured width while still scaling larger arrays upward.
            const CLASSIC_MOS_INSTANCES_PER_WORKER: usize = 512;
            let instance_count = circuit.mosfets.len();
            let useful_workers = instance_count.div_ceil(CLASSIC_MOS_INSTANCES_PER_WORKER);
            let worker_count = self
                .parallel_worker_count(instance_count)
                .min(useful_workers.max(1));
            if instance_count >= PARALLEL_CLASSIC_MOS_THRESHOLD && worker_count > 1 {
                return self.install_classic_mos_parallel(|| {
                    circuit.update_nonlinear_parallel_classic_mos(solution, worker_count);
                });
            }
        }
        circuit.update_nonlinear(solution);
        Ok(())
    }

    /// Test the nonlinear residual with the active transient solver's native
    /// norm. Xyce 7.10 transient Newton uses an unscaled infinity norm of the
    /// assembled RHS with an independent `RHSTOL` (default `1e-2`).
    /// Native/ngspice modes retain RSpice's row-scaled residual policy.
    pub(super) fn transient_residual_convergence_met(
        &self,
        circuit: &crate::circuit::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        solution: &[Value],
        rhs: &[Value],
    ) -> bool {
        if self.config.spice_dialect != SpiceDialect::Xyce {
            return self.residual_convergence_met(circuit, matrix, solution, rhs);
        }

        if circuit.xyce_core_trial_invalid() {
            return false;
        }

        matrix
            .raw_residual_inf_norm(solution, rhs)
            .is_ok_and(|norm| norm.is_finite() && norm < self.transient_nonlinear_rhstol())
    }

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
        circuit: &mut crate::circuit::CircuitData,
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
    ) -> Result<(), SimulationError> {
        self.stamp_transient_system_with_generated_mode(
            circuit,
            matrix,
            rhs,
            solution,
            time,
            dt,
            ctx,
            vbic_snapshot_cache,
            vbic_reuse,
            refresh_nonlinear,
            extra_diag_gmin,
            crate::device::veriloga_builtins::GeneratedEvaluationMode::NewtonLimited,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn stamp_transient_system_with_generated_mode(
        &self,
        circuit: &mut crate::circuit::CircuitData,
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
        evaluation_mode: crate::device::veriloga_builtins::GeneratedEvaluationMode,
    ) -> Result<(), SimulationError> {
        let num_nodes = circuit.num_nodes();
        matrix.clear_values();
        rhs.fill(0.0);

        let diag_gmin = ctx.baseline_diag_gmin.max(0.0) + extra_diag_gmin;
        Self::stamp_nodal_gmin(circuit, matrix, diag_gmin);

        // Stamp linear devices (R, V, I) for transient; tline transient
        // behavior is stamped separately via companions.
        circuit.stamp_transient_linear_direct(matrix, rhs);
        circuit
            .voltage_sources
            .update_transient_rhs(rhs, time, |br_ordinal| num_nodes + br_ordinal);
        circuit.current_sources.update_transient_rhs(rhs, time);

        // Xyce's OneStep order-2 residual scales the complete static DAE
        // contribution by one half.  The reactive Q contribution is stamped
        // below with an order-1 (backward-Euler) companion instead.
        if ctx.xyce_one_step_order2 {
            for value in matrix.values_mut() {
                *value *= 0.5;
            }
            for value in rhs.iter_mut() {
                *value *= 0.5;
            }
        }

        let companion_coeff = if ctx.xyce_one_step {
            CompanionCoefficients::backward_euler()
        } else {
            *ctx.coeff
        };
        let bsim4_companion_coeff = if ctx.xyce_one_step {
            companion_coeff
        } else {
            *ctx.bsim4_trnqs_coeff
        };

        circuit.refresh_jiles_atherton_inductances(solution);
        circuit.set_b3soi_operating_point_mode(false);
        circuit.set_xyce_memristor_operating_point_mode(false);
        if refresh_nonlinear && circuit.has_nonlinear_devices() {
            self.update_transient_nonlinear_devices(circuit, solution)?;
            if evaluation_mode
                == crate::device::veriloga_builtins::GeneratedEvaluationMode::StaticProbe
            {
                // Native semiconductor Newton stamps apply local voltage
                // limiting and retain the corresponding linearization cache.
                // A convergence proof must instead evaluate every physical
                // equation at the candidate itself, just as generated
                // Verilog-A devices do in StaticProbe mode.
                circuit.update_bjt_static_linearizations(solution);
                circuit.update_b3soi_static_linearizations(solution);
                circuit.update_jfet_static_linearizations(solution);
            }
        }
        Self::refresh_jfet2_transient_linearizations(circuit, solution, dt, ctx.jfet_history);

        circuit
            .capacitors
            .stamp_transient_companion(matrix, rhs, dt, &companion_coeff, num_nodes);
        circuit
            .capacitors
            .stamp_solution_dependent_transient_companion(
                matrix,
                rhs,
                solution,
                time,
                dt,
                &companion_coeff,
            )
            .map_err(SimulationError::Circuit)?;
        circuit.stamp_transient_inductor_companions(matrix, rhs, dt, &companion_coeff, num_nodes);
        if ctx.xyce_one_step_order2 {
            circuit
                .inductors
                .scale_transient_kcl_rows(matrix, num_nodes, 0.5);
        }
        circuit.stamp_xyce_core_transient_companion(
            matrix,
            rhs,
            solution,
            dt,
            &companion_coeff,
            ctx.xyce_one_step_order2,
            evaluation_mode
                == crate::device::veriloga_builtins::GeneratedEvaluationMode::NewtonLimited
                && refresh_nonlinear,
        );
        circuit.stamp_coupled_inductor_pairs_transient(matrix, rhs, dt, &companion_coeff);
        circuit.stamp_multi_winding_transformers_transient(matrix, rhs, dt, &companion_coeff);

        Self::stamp_bjt_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            &companion_coeff,
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
            &companion_coeff,
            dt,
            ctx.jfet_history,
            ctx.suppress_gate_charge,
        );
        Self::stamp_diode_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            &companion_coeff,
            dt,
            ctx.diode_history,
            ctx.diode_companion_slots,
        );
        Self::stamp_mosfet_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            &companion_coeff,
            dt,
            ctx.mosfet_history,
            ctx.suppress_gate_charge,
            ctx.mosfet_companion_slots,
            None,
        );
        Self::stamp_vdmos_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            &companion_coeff,
            dt,
            ctx.vdmos_history,
            ctx.vdmos_companion_slots,
        );
        if !ctx.b3soi_zero_first_transient_charge_derivative {
            Self::stamp_b3soi_transient_companions(
                circuit,
                matrix,
                rhs,
                solution,
                &companion_coeff,
                dt,
                ctx.b3soi_history,
            );
        }
        Self::stamp_bsim3_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            &companion_coeff,
            dt,
            ctx.bsim3_history,
        );
        Self::stamp_bsim4_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            &companion_coeff,
            &bsim4_companion_coeff,
            dt,
            ctx.bsim4_history,
        );
        Self::stamp_ekv26_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            &companion_coeff,
            dt,
            ctx.ekv26_history,
        );
        Self::stamp_tline_companions(circuit, matrix, rhs, time, ctx.tline_dc_refs);
        Self::stamp_coupled_tline_companions(
            circuit,
            matrix,
            rhs,
            time,
            dt,
            ctx.coupled_tline_refs,
        );
        let generic_switch_snapshot = ctx
            .xyce_one_step_order2
            .then(|| (matrix.values_mut().to_vec(), rhs.to_vec()));
        circuit.stamp_generic_switches_with_solution(matrix, rhs, solution, time);
        if let Some((before_values, before_rhs)) = generic_switch_snapshot {
            for (value, before) in matrix.values_mut().iter_mut().zip(before_values) {
                *value = before + 0.5 * (*value - before);
            }
            for (value, before) in rhs.iter_mut().zip(before_rhs) {
                *value = before + 0.5 * (*value - before);
            }
        }

        if circuit.has_nonlinear_devices() {
            #[cfg(feature = "veriloga")]
            if circuit.has_veriloga_devices() {
                circuit
                    .prepare_veriloga_timepoint(
                        time,
                        dt,
                        &companion_coeff,
                        ctx.analysis_initial_step,
                        ctx.analysis_final_step,
                    )
                    .map_err(SimulationError::Circuit)?;
            }
            #[cfg(feature = "veriloga-builtins-base")]
            if circuit.has_generated_veriloga_devices() {
                circuit.prepare_generated_veriloga_timepoint(
                    time,
                    dt,
                    &companion_coeff,
                    ctx.analysis_initial_step,
                    ctx.analysis_final_step,
                );
            }
            let static_nonlinear_snapshot = ctx
                .xyce_one_step_order2
                .then(|| (matrix.values_mut().to_vec(), rhs.to_vec()));
            if evaluation_mode
                == crate::device::veriloga_builtins::GeneratedEvaluationMode::StaticProbe
            {
                circuit
                    .try_stamp_static_probe_nonlinear(matrix, rhs, solution)
                    .map_err(SimulationError::Circuit)?;
            } else {
                circuit
                    .stamp_nonlinear(matrix, rhs, solution)
                    .map_err(SimulationError::Circuit)?;
            }
            circuit
                .stamp_behavioral_with_generated_mode(
                    matrix,
                    rhs,
                    solution,
                    time,
                    crate::xspice::AnalysisType::Transient,
                    evaluation_mode,
                )
                .map_err(SimulationError::Circuit)?;
            if let Some((before_values, before_rhs)) = static_nonlinear_snapshot {
                for (value, before) in matrix.values_mut().iter_mut().zip(before_values) {
                    *value = before + 0.5 * (*value - before);
                }
                for (value, before) in rhs.iter_mut().zip(before_rhs) {
                    *value = before + 0.5 * (*value - before);
                }
            }
        } else {
            let static_behavioral_snapshot = ctx
                .xyce_one_step_order2
                .then(|| (matrix.values_mut().to_vec(), rhs.to_vec()));
            circuit.stamp_behavioral_sources(matrix, rhs, solution, time);
            if let Some((before_values, before_rhs)) = static_behavioral_snapshot {
                for (value, before) in matrix.values_mut().iter_mut().zip(before_values) {
                    *value = before + 0.5 * (*value - before);
                }
                for (value, before) in rhs.iter_mut().zip(before_rhs) {
                    *value = before + 0.5 * (*value - before);
                }
            }
        }

        if circuit.has_xspice_devices() {
            circuit.stamp_xspice_transient_trial_with_coefficients(
                matrix,
                rhs,
                time,
                dt,
                solution,
                &companion_coeff,
                ctx.xyce_one_step_order2,
            );
        }
        if ctx.xyce_one_step_order2 {
            if let Some(history) = ctx.xyce_static_history {
                debug_assert_eq!(history.len(), rhs.len());
                for (rhs_value, previous_residual) in rhs.iter_mut().zip(history) {
                    // The assembled system is A*x = b, while OneStep adds
                    // +0.5*(F_prev-B_prev) to its residual A*x-b.
                    *rhs_value -= 0.5 * previous_residual;
                }
            }
        }
        Ok(())
    }

    /// Capture the accepted physical static residual F(x)-B(t) used by
    /// Xyce's OneStep order-2 history term.  The transient companion matrix
    /// is deliberately omitted: it is represented by the per-device Q
    /// histories and is reassembled with the attempted step size.
    pub(super) fn capture_xyce_static_residual(
        &self,
        circuit: &mut crate::circuit::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        solution: &[Value],
        time: Value,
        baseline_diag_gmin: Value,
    ) -> Result<Vec<Value>, SimulationError> {
        self.update_transient_nonlinear_devices(circuit, solution)?;
        circuit.update_bjt_static_linearizations(solution);
        circuit.update_b3soi_static_linearizations(solution);
        circuit.update_jfet_static_linearizations(solution);

        let num_nodes = circuit.num_nodes();
        matrix.with_probe_values(|probe, probe_rhs| {
            Self::stamp_nodal_gmin(circuit, probe, baseline_diag_gmin.max(0.0));
            circuit.stamp_transient_linear_direct(probe, probe_rhs);
            circuit
                .voltage_sources
                .update_transient_rhs(probe_rhs, time, |br_ordinal| num_nodes + br_ordinal);
            circuit
                .current_sources
                .update_transient_rhs(probe_rhs, time);
            circuit.stamp_generic_switches_with_solution(probe, probe_rhs, solution, time);
            circuit.stamp_xyce_core_static_residual(probe, solution);
            if circuit.has_nonlinear_devices() {
                circuit
                    .try_stamp_static_probe_nonlinear(probe, probe_rhs, solution)
                    .map_err(SimulationError::Circuit)?;
                circuit
                    .stamp_behavioral_static_probe(
                        probe,
                        probe_rhs,
                        solution,
                        time,
                        crate::xspice::AnalysisType::Transient,
                    )
                    .map_err(SimulationError::Circuit)?;
            } else {
                circuit.stamp_behavioral_sources(probe, probe_rhs, solution, time);
            }
            if circuit.has_xspice_devices() {
                // OneStep's order-2 history term applies to the complete
                // static residual, including finite-impedance XSPICE output
                // conductances.  The accepted evaluation has already queued
                // a static-only stamp set, so consume it without reevaluating
                // event-driven models at the breakpoint.
                circuit.stamp_xspice_static_residual(solution, probe_rhs);
            }
            let residual = probe
                .residual_vector(solution, probe_rhs)
                .map_err(|error| SimulationError::Circuit(error.to_string()))?;
            Ok(residual)
        })
    }

    /// Restamp the full system at `solution` and test the true nonlinear
    /// residual against the engine's convergence scaling.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn transient_nonlinear_residual_converged(
        &self,
        circuit: &mut crate::circuit::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        dt: Value,
        ctx: &TransientSystemContext<'_>,
        classic_mos_cache: Option<&ClassicMosTransientStampCache>,
        vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
        classic_mos_caps_out: Option<&mut Vec<(Value, Value, Value)>>,
    ) -> Result<bool, SimulationError> {
        if solution.iter().any(|value| !value.is_finite()) {
            return Ok(false);
        }

        let refresh_nonlinear = !circuit.has_classic_mos_only_transient_nonlinearity();
        if let Some(cache) = classic_mos_cache {
            self.stamp_classic_mos_transient_system_from_cache(
                cache,
                circuit,
                matrix,
                rhs,
                solution,
                dt,
                ctx,
                refresh_nonlinear,
                crate::device::veriloga_builtins::GeneratedEvaluationMode::StaticProbe,
                classic_mos_caps_out,
            )?;
        } else {
            self.stamp_transient_system_with_generated_mode(
                circuit,
                matrix,
                rhs,
                solution,
                time,
                dt,
                ctx,
                vbic_snapshot_cache,
                VbicCachedSnapshotReuse::SeedOnly,
                refresh_nonlinear,
                0.0,
                crate::device::veriloga_builtins::GeneratedEvaluationMode::StaticProbe,
            )?;
        }
        Ok(self.transient_residual_convergence_met(circuit, matrix, solution, rhs))
    }

    #[inline]
    pub(super) fn should_prefer_dense_transient_solver(
        is_strictly_linear_transient: bool,
        size: usize,
        has_xspice_devices: bool,
    ) -> bool {
        // Linear transformer companion matrices can be severely ill-scaled as
        // the integration timestep changes. The equilibrated sparse path both
        // solves them faster and satisfies the componentwise backward-error
        // gate where unscaled dense Gaussian elimination can reject an
        // otherwise finite linear point.
        if is_strictly_linear_transient {
            return false;
        }

        // Small nonlinear systems used to route to dense LU because the
        // general sparse machinery dominates at this scale. The KLU-class
        // values-only refactorization has no such overhead — measured on
        // ring51 (~60 unknowns), the dense path costs ~100 us per Newton
        // solve against single-digit microseconds for the refactor — so
        // Dense remains only for explicit `RSPICE_SOLVER=faer` runs.
        !has_xspice_devices && size <= 64 && !crate::solver::klu_backend_enabled()
    }
}

#[cfg(test)]
mod tests {
    //! Whole-system Newton consistency: the stamped transient matrix must be
    //! the Jacobian of the true assembled residual `F(x) = A(x)·x − b(x)`.
    //!
    //! The device-level FD tests pin each model's static Jacobian and charge
    //! derivatives in isolation; what they cannot catch is an inconsistency
    //! *between* the pieces as assembled — a companion source folded at a
    //! different bias than its conductance, a sign flipped on one row of a
    //! charge branch, or a derivative stamped onto the wrong matrix column.
    //! Any such defect makes Newton iterate against a matrix that is not the
    //! derivative of the residual it is reducing, which shows up as the
    //! non-contracting mV-scale limit cycles seen on saturation knife edges.
    //!
    //! Consistency holds exactly (up to FD truncation) whenever the junction
    //! limiters are pass-through, i.e. when the device update state is
    //! settled at the probe base and the probes are small.

    use super::*;
    use crate::Netlist;

    #[test]
    fn xyce_transient_residual_honors_nonlin_tran_rhstol() {
        let mut engine = Engine::default();
        engine.config.spice_dialect = SpiceDialect::Xyce;
        engine.config.transient_nonlinear_rhstol = Some(0.25);
        let circuit = crate::circuit::CircuitData::default();
        let mut matrix = crate::solver::StaticMatrix::from_triplets(1, 1, &[(0, 0, 1.0)])
            .expect("identity matrix");

        assert!(engine.transient_residual_convergence_met(
            &circuit,
            &mut matrix,
            &[0.0],
            &[0.249_999],
        ));
        assert!(
            !engine.transient_residual_convergence_met(&circuit, &mut matrix, &[0.0], &[0.25],)
        );
    }

    #[test]
    fn cached_classic_mos_assembly_matches_canonical_static_probe_exactly() {
        const DECK: &str = "\
Classic MOS transient cache equivalence
VDD vdd 0 5
VIN g 0 DC 0 PULSE(0 2 1n 100p 100p 4n 10n)
IBIAS d 0 DC 0 PULSE(0 10u 1.1n 100p 100p 4n 10n)
R1 vdd d 2k
C1 d 0 2p
M1 d g 0 0 NM W=10u L=1u
.MODEL NM NMOS LEVEL=1 VTO=0.7 KP=100u LAMBDA=0.02 CGSO=2e-10 CGDO=1e-10
.END
";
        let netlist = Netlist::parse(DECK).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        assert!(circuit.has_cacheable_classic_mos_transient_base());
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);

        let dt = 2.0e-10;
        let time = 1.3e-9;
        circuit.voltage_sources.set_transient_context(dt, 20.0e-9);
        circuit.current_sources.set_transient_context(dt, 20.0e-9);
        let mut solution = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("operating point converges");
        solution[circuit.get_node_by_name("g").expect("gate node") - 1] = 1.4;
        solution[circuit.get_node_by_name("d").expect("drain node") - 1] = 2.7;
        circuit.update_nonlinear(&solution);
        circuit.update_nonlinear(&solution);

        let tline_dc_refs = Engine::initialize_tline_history(&mut circuit, &solution, 0.0);
        let coupled_tline_refs =
            Engine::initialize_coupled_tline_history(&mut circuit, &solution, 0.0);
        let bjt_history = Engine::initialize_bjt_history(&circuit, &solution);
        let jfet_history = Engine::initialize_jfet_history(&circuit, &solution);
        let diode_history = Engine::initialize_diode_history(&circuit, &solution);
        let mosfet_history = Engine::initialize_mosfet_history(&circuit, &solution);
        let vdmos_history = Engine::initialize_vdmos_history(&circuit, &solution);
        let b3soi_history = Engine::initialize_b3soi_history(&circuit, &solution);
        let bsim3_history = Engine::initialize_bsim3_history(&circuit, &solution);
        let bsim4_history = Engine::initialize_bsim4_history(&circuit, &solution);
        let ekv26_history = Engine::initialize_ekv26_history(&circuit, &solution);
        let diode_companion_slots = Engine::link_diode_companion_slots(&circuit, &matrix);
        let mosfet_companion_slots = Engine::link_mosfet_companion_slots(&circuit, &matrix);
        let vdmos_companion_slots = Engine::link_vdmos_companion_slots(&circuit, &matrix);
        let coeff = CompanionCoefficients::backward_euler();
        let baseline_diag_gmin = engine.config.convergence_config.gmin_target.max(0.0);
        let ctx = TransientSystemContext {
            coeff: &coeff,
            xyce_one_step: false,
            xyce_one_step_order2: false,
            xyce_static_history: None,
            bsim4_trnqs_coeff: &coeff,
            bjt_history: &bjt_history,
            jfet_history: &jfet_history,
            diode_history: &diode_history,
            diode_companion_slots: &diode_companion_slots,
            mosfet_history: &mosfet_history,
            mosfet_companion_slots: &mosfet_companion_slots,
            vdmos_history: &vdmos_history,
            vdmos_companion_slots: &vdmos_companion_slots,
            b3soi_history: &b3soi_history,
            b3soi_zero_first_transient_charge_derivative: false,
            bsim3_history: &bsim3_history,
            bsim4_history: &bsim4_history,
            ekv26_history: &ekv26_history,
            suppress_gate_charge: false,
            baseline_diag_gmin,
            tline_dc_refs: &tline_dc_refs,
            coupled_tline_refs: &coupled_tline_refs,
            analysis_initial_step: false,
            analysis_final_step: false,
        };
        let mut rhs = vec![0.0; circuit.matrix_size()];
        let mut vbic_snapshot_cache = Vec::new();

        engine
            .stamp_transient_system_with_generated_mode(
                &mut circuit,
                &mut matrix,
                &mut rhs,
                &solution,
                time,
                dt,
                &ctx,
                &mut vbic_snapshot_cache,
                VbicCachedSnapshotReuse::SeedOnly,
                false,
                0.0,
                crate::device::veriloga_builtins::GeneratedEvaluationMode::StaticProbe,
            )
            .expect("canonical assembly succeeds");
        let canonical_values = matrix.values_mut().to_vec();
        let canonical_rhs = rhs.clone();

        let mut cache = engine.initialize_classic_mos_transient_stamp_cache(
            &circuit,
            &mut matrix,
            &mut rhs,
            baseline_diag_gmin,
        );
        engine.prepare_classic_mos_transient_attempt(
            &mut cache,
            &circuit,
            &mut matrix,
            &mut rhs,
            time,
            dt,
            &coeff,
            false,
        );
        engine
            .stamp_classic_mos_transient_system_from_cache(
                &cache,
                &mut circuit,
                &mut matrix,
                &mut rhs,
                &solution,
                dt,
                &ctx,
                false,
                crate::device::veriloga_builtins::GeneratedEvaluationMode::StaticProbe,
                None,
            )
            .expect("cached assembly succeeds");

        assert_eq!(matrix.values_mut(), canonical_values);
        assert_eq!(rhs, canonical_rhs);
    }

    /// The vbic_excess_phase_oracle testbench: the full diffamp N1 card
    /// (Kull epi resistance, avalanche, parasitic transistor, TD=2e-11)
    /// in a stable common-emitter stage. Exercises the promoted VBIC
    /// internal nodes, the xf delay states, and their charge companions.
    const VBIC_XF_DECK: &str = "\
VBIC whole-system Jacobian consistency testbench

V1 VCC 0 3.3
VIN B 0 DC 0.8 SIN(0.8 0.05 1G 0 0)
RC VCC C 1k
RE E 0 100
Q1 C B E 0 N1

.MODEL N1 NPN LEVEL=4
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 ISP=1e-15 RCX=10
+ RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3 ITF=8e-2
+ XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 CJCP=4e-13 VO=2
+ GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 TR=100e-12 TD=2e-11 RTH=300

.END
";

    /// Legacy Gummel-Poon BJT through the snapshot companion path.
    const GP_BJT_DECK: &str = "\
Legacy GP whole-system Jacobian consistency testbench

V1 VCC 0 5
VIN B 0 DC 0.7
RC VCC C 2k
RE E 0 100
Q1 C B E 0 QN

.MODEL QN NPN IS=1e-15 BF=100 BR=2 CJE=2e-12 CJC=1e-12 TF=3e-10 TR=5e-9

.END
";

    fn matrix_row_label(circuit: &crate::circuit::CircuitData, index: usize) -> String {
        let num_nodes = circuit.num_nodes();
        if index < num_nodes {
            let names = circuit.node_names_sorted();
            names
                .get(index)
                .cloned()
                .unwrap_or_else(|| format!("node#{}", index + 1))
        } else {
            format!("branch#{}", index - num_nodes + 1)
        }
    }

    /// Assemble the transient system at every coordinate-perturbed point and
    /// compare central FD columns of the true residual against the stamped
    /// matrix. Returns human-readable descriptions of every inconsistent
    /// entry; empty means the assembly is Newton-consistent at `base`.
    fn assembled_jacobian_fd_failures(
        deck: &str,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        kick: Option<(&str, Value)>,
    ) -> Vec<String> {
        let netlist = Netlist::parse(deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);
        circuit.voltage_sources.set_transient_context(dt, 1e-9);
        circuit.current_sources.set_transient_context(dt, 1e-9);

        let mut base = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("operating point converges");
        if let Some((node, delta)) = kick {
            let id = circuit
                .get_node_by_name(node)
                .expect("kick node exists in the deck");
            base[id - 1] += delta;
        }
        // Settle the device update state (junction-limiter references) at
        // the probe base so the limiters are pass-through for small probes.
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&base);
            circuit.update_nonlinear(&base);
        }

        let tline_dc_refs = Engine::initialize_tline_history(&mut circuit, &base, 0.0);
        let coupled_tline_refs = Engine::initialize_coupled_tline_history(&mut circuit, &base, 0.0);
        let mut bjt_history = Engine::initialize_bjt_history(&circuit, &base);
        bjt_history.accepted_dt_prev = dt;
        bjt_history.accepted_dt_prev_prev = dt;
        let mut jfet_history = Engine::initialize_jfet_history(&circuit, &base);
        jfet_history.accepted_dt_prev = dt;
        jfet_history.accepted_dt_prev_prev = dt;
        let mut diode_history = Engine::initialize_diode_history(&circuit, &base);
        diode_history.accepted_dt_prev = dt;
        diode_history.accepted_dt_prev_prev = dt;
        let mut mosfet_history = Engine::initialize_mosfet_history(&circuit, &base);
        mosfet_history.accepted_dt_prev = dt;
        mosfet_history.accepted_dt_prev_prev = dt;
        let mut vdmos_history = Engine::initialize_vdmos_history(&circuit, &base);
        vdmos_history.accepted_dt_prev = dt;
        vdmos_history.accepted_dt_prev_prev = dt;
        let mut b3soi_history = Engine::initialize_b3soi_history(&circuit, &base);
        b3soi_history.accepted_dt_prev = dt;
        b3soi_history.accepted_dt_prev_prev = dt;
        let mut bsim3_history = Engine::initialize_bsim3_history(&circuit, &base);
        bsim3_history.accepted_dt_prev = dt;
        bsim3_history.accepted_dt_prev_prev = dt;
        let mut bsim4_history = Engine::initialize_bsim4_history(&circuit, &base);
        bsim4_history.accepted_dt_prev = dt;
        bsim4_history.accepted_dt_prev_prev = dt;
        let mut ekv26_history = Engine::initialize_ekv26_history(&circuit, &base);
        ekv26_history.accepted_dt_prev = dt;
        ekv26_history.accepted_dt_prev_prev = dt;
        let mut vbic_snapshot_cache = vec![None; circuit.bjts.devices.len()];
        circuit.set_semiconductor_junction_gmin(
            engine.effective_device_junction_gmin(engine.config.convergence_config.gmin_target),
        );

        let coeff = CompanionCoefficients::for_method(Engine::effective_companion_method(
            method, trap_order,
        ));
        let diode_companion_slots = Engine::link_diode_companion_slots(&circuit, &matrix);
        let mosfet_companion_slots = Engine::link_mosfet_companion_slots(&circuit, &matrix);
        let vdmos_companion_slots = Engine::link_vdmos_companion_slots(&circuit, &matrix);
        let ctx = TransientSystemContext {
            coeff: &coeff,
            xyce_one_step: false,
            xyce_one_step_order2: false,
            xyce_static_history: None,
            bsim4_trnqs_coeff: &coeff,
            bjt_history: &bjt_history,
            jfet_history: &jfet_history,
            diode_history: &diode_history,
            diode_companion_slots: &diode_companion_slots,
            mosfet_history: &mosfet_history,
            mosfet_companion_slots: &mosfet_companion_slots,
            vdmos_history: &vdmos_history,
            vdmos_companion_slots: &vdmos_companion_slots,
            b3soi_history: &b3soi_history,
            b3soi_zero_first_transient_charge_derivative: false,
            bsim3_history: &bsim3_history,
            bsim4_history: &bsim4_history,
            ekv26_history: &ekv26_history,
            suppress_gate_charge: false,
            baseline_diag_gmin: engine.config.convergence_config.gmin_target.max(0.0),
            tline_dc_refs: &tline_dc_refs,
            coupled_tline_refs: &coupled_tline_refs,
            analysis_initial_step: false,
            analysis_final_step: false,
        };

        let size = circuit.matrix_size();
        let time = dt;
        let mut rhs = vec![0.0; size];

        // Stamp at the base point and capture the asserted Jacobian columns
        // A(base)·e_j before any probe overwrites the matrix values.
        engine
            .stamp_transient_system(
                &mut circuit,
                &mut matrix,
                &mut rhs,
                &base,
                time,
                dt,
                &ctx,
                &mut vbic_snapshot_cache,
                VbicCachedSnapshotReuse::SeedOnly,
                true,
                0.0,
            )
            .expect("base transient system stamps");
        let zeros = vec![0.0; size];
        let mut stamped_columns = Vec::with_capacity(size);
        for j in 0..size {
            let mut unit = vec![0.0; size];
            unit[j] = 1.0;
            stamped_columns.push(
                matrix
                    .residual_vector(&unit, &zeros)
                    .expect("matrix column extraction"),
            );
        }

        let mut failures = Vec::new();
        for j in 0..size {
            let h = 1e-7 * base[j].abs().max(1.0);
            let mut probe = base.clone();
            probe[j] += h;
            engine
                .stamp_transient_system(
                    &mut circuit,
                    &mut matrix,
                    &mut rhs,
                    &probe,
                    time,
                    dt,
                    &ctx,
                    &mut vbic_snapshot_cache,
                    VbicCachedSnapshotReuse::SeedOnly,
                    true,
                    0.0,
                )
                .expect("positive transient probe stamps");
            let f_plus = matrix
                .residual_vector(&probe, &rhs)
                .expect("residual at +h probe");

            probe[j] = base[j] - h;
            engine
                .stamp_transient_system(
                    &mut circuit,
                    &mut matrix,
                    &mut rhs,
                    &probe,
                    time,
                    dt,
                    &ctx,
                    &mut vbic_snapshot_cache,
                    VbicCachedSnapshotReuse::SeedOnly,
                    true,
                    0.0,
                )
                .expect("negative transient probe stamps");
            let f_minus = matrix
                .residual_vector(&probe, &rhs)
                .expect("residual at -h probe");

            let column_scale = stamped_columns[j]
                .iter()
                .fold(0.0_f64, |acc, value| acc.max(value.abs()));
            for i in 0..size {
                let fd = (f_plus[i] - f_minus[i]) / (2.0 * h);
                let stamped = stamped_columns[j][i];
                let error = (fd - stamped).abs();
                let gate = 1e-6 * column_scale.max(1e-12) + 1e-4 * fd.abs().max(stamped.abs());
                if error > gate {
                    failures.push(format!(
                        "d F[{}] / d x[{}]: stamped {:.9e} vs FD {:.9e} (err {:.3e})",
                        matrix_row_label(&circuit, i),
                        matrix_row_label(&circuit, j),
                        stamped,
                        fd,
                        error,
                    ));
                }
            }
        }
        failures
    }

    fn assert_consistent(
        deck: &str,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        kick: Option<(&str, Value)>,
    ) {
        let failures = assembled_jacobian_fd_failures(deck, method, trap_order, dt, kick);
        assert!(
            failures.is_empty(),
            "assembled transient matrix is not the Jacobian of the assembled residual \
             (method {:?}, trap order {}, dt {:.1e}, kick {:?}):\n{}",
            method,
            trap_order,
            dt,
            kick,
            failures.join("\n"),
        );
    }

    #[test]
    fn promoted_vbic_assembly_is_newton_consistent_at_small_dt() {
        assert_consistent(VBIC_XF_DECK, IntegrationMethod::Trapezoidal, 2, 2e-12, None);
    }

    #[test]
    fn promoted_vbic_assembly_is_newton_consistent_at_diffamp_scale_dt() {
        // dt/td = 5, the regime where the diffamp deck amplifies per
        // accepted step; trapezoidal and Gear2 both fail there, so both
        // discretizations must prove consistent.
        assert_consistent(VBIC_XF_DECK, IntegrationMethod::Trapezoidal, 2, 1e-10, None);
        assert_consistent(VBIC_XF_DECK, IntegrationMethod::Gear2, 2, 1e-10, None);
    }

    #[test]
    fn promoted_vbic_assembly_is_newton_consistent_at_backward_euler_startup() {
        // Trap order 1 resolves to the backward-Euler companion used on the
        // first accepted points and after every breakpoint restart.
        assert_consistent(VBIC_XF_DECK, IntegrationMethod::Trapezoidal, 1, 2e-12, None);
    }

    #[test]
    fn promoted_vbic_assembly_is_newton_consistent_off_equilibrium() {
        // A base-drive kick moves the BE junction off the solved manifold —
        // the consistency contract must hold at unconverged iterates too,
        // since that is where Newton actually navigates.
        assert_consistent(
            VBIC_XF_DECK,
            IntegrationMethod::Trapezoidal,
            2,
            1e-10,
            Some(("B", 0.03)),
        );
    }

    #[test]
    fn legacy_gp_snapshot_assembly_is_newton_consistent() {
        assert_consistent(GP_BJT_DECK, IntegrationMethod::Trapezoidal, 2, 1e-10, None);
        assert_consistent(GP_BJT_DECK, IntegrationMethod::Gear2, 2, 1e-10, None);
    }

    /// Spectral radius of the backward-Euler one-step map `M = I − A(dt)⁻¹G`
    /// of the system linearized at `base`, by power iteration. `G` is the
    /// pure-static assembly (huge-dt limit), `A(dt)` the BE assembly; for a
    /// generalized pole `s` of the (G, C) pencil the map's eigenvalue is
    /// `1/(1 − s·dt)`, so a radius above one at small `dt` certifies a
    /// right-half-plane pole of the linearization — instability of the
    /// continuous model, not of the discretization.
    #[allow(clippy::too_many_arguments)]
    fn be_one_step_map_spectral_radius(
        engine: &Engine,
        circuit: &mut crate::circuit::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        base: &[Value],
        ctx: &TransientSystemContext<'_>,
        vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
        dt: Value,
        iterations: usize,
    ) -> (Value, Vec<Value>) {
        let size = circuit.matrix_size();
        let mut rhs = vec![0.0; size];
        let zeros = vec![0.0; size];

        // Dense static matrix from the huge-dt assembly (companion
        // conductances scale as 1/dt and vanish).
        engine
            .stamp_transient_system(
                circuit,
                matrix,
                &mut rhs,
                base,
                dt,
                1e9,
                ctx,
                vbic_snapshot_cache,
                VbicCachedSnapshotReuse::SeedOnly,
                true,
                0.0,
            )
            .expect("static companion system stamps");
        let mut static_columns = Vec::with_capacity(size);
        for j in 0..size {
            let mut unit = vec![0.0; size];
            unit[j] = 1.0;
            static_columns.push(
                matrix
                    .residual_vector(&unit, &zeros)
                    .expect("static column extraction"),
            );
        }

        // BE assembly at the probe dt; the matrix now holds A(dt).
        engine
            .stamp_transient_system(
                circuit,
                matrix,
                &mut rhs,
                base,
                dt,
                dt,
                ctx,
                vbic_snapshot_cache,
                VbicCachedSnapshotReuse::SeedOnly,
                true,
                0.0,
            )
            .expect("dynamic companion system stamps");

        // Power iteration on M·z = z − A⁻¹·(G·z). A deterministic
        // pseudo-random start avoids accidental orthogonality to the
        // dominant mode; the radius estimate is the geometric mean of the
        // last few growth ratios, which converges for complex-pair
        // dominants as well.
        let mut z: Vec<Value> = (0..size)
            .map(|i| (((i * 2654435761) % 1000) as Value / 1000.0) - 0.5)
            .collect();
        let mut norm = z.iter().map(|v| v * v).sum::<Value>().sqrt();
        for v in &mut z {
            *v /= norm;
        }
        let mut log_growth_window = std::collections::VecDeque::new();
        for _ in 0..iterations {
            let mut gz = vec![0.0; size];
            for (j, column) in static_columns.iter().enumerate() {
                let zj = z[j];
                if zj == 0.0 {
                    continue;
                }
                for i in 0..size {
                    gz[i] += column[i] * zj;
                }
            }
            let correction = matrix.solve(&gz).expect("A solve in power iteration");
            for i in 0..size {
                z[i] -= correction[i];
            }
            norm = z.iter().map(|v| v * v).sum::<Value>().sqrt();
            if !norm.is_finite() || norm == 0.0 {
                return (norm, z);
            }
            log_growth_window.push_back(norm.ln());
            if log_growth_window.len() > 24 {
                log_growth_window.pop_front();
            }
            for v in &mut z {
                *v /= norm;
            }
        }
        let radius =
            (log_growth_window.iter().sum::<Value>() / log_growth_window.len() as Value).exp();
        (radius, z)
    }

    /// One application of the BE one-step map at `dt` to a fixed vector:
    /// returns `‖M(dt)·v‖ / ‖v‖`. Discriminates a genuine pencil eigenmode
    /// (the ratio must track `1/|1 − s·dt|` across dts for a fixed pole `s`)
    /// from a power-iteration dominant that is structural junk.
    #[allow(clippy::too_many_arguments)]
    fn be_one_step_map_gain_on_vector(
        engine: &Engine,
        circuit: &mut crate::circuit::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        base: &[Value],
        ctx: &TransientSystemContext<'_>,
        vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
        dt: Value,
        vector: &[Value],
    ) -> Value {
        let size = circuit.matrix_size();
        let mut rhs = vec![0.0; size];
        let zeros = vec![0.0; size];
        engine
            .stamp_transient_system(
                circuit,
                matrix,
                &mut rhs,
                base,
                dt,
                1e9,
                ctx,
                vbic_snapshot_cache,
                VbicCachedSnapshotReuse::SeedOnly,
                true,
                0.0,
            )
            .expect("static matrix-vector system stamps");
        let gv = matrix
            .residual_vector(vector, &zeros)
            .expect("static matvec");
        engine
            .stamp_transient_system(
                circuit,
                matrix,
                &mut rhs,
                base,
                dt,
                dt,
                ctx,
                vbic_snapshot_cache,
                VbicCachedSnapshotReuse::SeedOnly,
                true,
                0.0,
            )
            .expect("dynamic matrix-vector system stamps");
        let correction = matrix.solve(&gv).expect("A solve");
        let mapped_norm = vector
            .iter()
            .zip(&correction)
            .map(|(v, c)| (v - c) * (v - c))
            .sum::<Value>()
            .sqrt();
        let vector_norm = vector.iter().map(|v| v * v).sum::<Value>().sqrt();
        mapped_norm / vector_norm
    }

    /// Diagnostic probe for the vbic diffamp transient stall: measures the
    /// linearized one-step amplification at the solved operating point.
    /// Ignored by default — it needs the checked-in regression deck and a
    /// multi-second aided OP solve; run with
    /// `cargo test --release -p rspice-core --lib diffamp_one_step -- --ignored --nocapture`.
    #[test]
    #[ignore = "diagnostic probe over the diffamp regression deck"]
    fn diffamp_one_step_map_spectral_radius_probe() {
        let deck_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/vbic/diffamp.cir");
        let deck = std::fs::read_to_string(deck_path).expect("diffamp deck readable");
        // TD=2e-12 is the recorded transient discriminator (completes clean
        // where TD=2e-11 blows up after ~8 accepted steps at any dt); if the
        // radius collapses below one with it, the right-half-plane modes are
        // conclusively the excess-phase delay loop's.
        let deck_small_td = deck.replace("TD=2e-11", "TD=2e-12");
        assert_ne!(
            deck, deck_small_td,
            "TD replacement must hit the model cards"
        );
        // Model-term knockouts to localize the physical origin of any
        // right-half-plane modes: avalanche generation off, Kull epi
        // charge off.
        let deck_no_avalanche = deck.replace("AVC1=2 AVC2=15", "AVC1=0 AVC2=15");
        assert_ne!(deck, deck_no_avalanche);
        let deck_no_qco = deck.replace("QCO=1e-12", "QCO=0");
        assert_ne!(deck, deck_no_qco);
        let deck_no_parasitic = deck.replace("ISP=1e-15", "ISP=0");
        assert_ne!(deck, deck_no_parasitic);
        let deck_no_kull = deck.replace("RCI=60", "RCI=0");
        assert_ne!(deck, deck_no_kull);
        for (label, source) in [
            ("diffamp", deck.as_str()),
            ("diffamp-td2e-12", deck_small_td.as_str()),
            ("diffamp-no-avalanche", deck_no_avalanche.as_str()),
            ("diffamp-no-qco", deck_no_qco.as_str()),
            ("diffamp-no-parasitic", deck_no_parasitic.as_str()),
            ("diffamp-no-kull", deck_no_kull.as_str()),
            ("ce-control", VBIC_XF_DECK),
        ] {
            let netlist = Netlist::parse(source).expect("deck parses");
            let engine = Engine::default().resolved_for_netlist(&netlist);
            let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
            let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
            circuit.link_indices(&matrix);
            circuit.voltage_sources.set_transient_context(1e-9, 1e-6);
            circuit.current_sources.set_transient_context(1e-9, 1e-6);
            let (base, _mode) = engine
                .solve_transient_initial_solution(
                    &netlist,
                    &mut circuit,
                    &mut matrix,
                    &crate::abort_signal::NoAbort,
                )
                .expect("operating point");
            circuit.update_nonlinear(&base);
            circuit.update_nonlinear(&base);

            let tline_dc_refs = Engine::initialize_tline_history(&mut circuit, &base, 0.0);
            let coupled_tline_refs =
                Engine::initialize_coupled_tline_history(&mut circuit, &base, 0.0);
            let bjt_history = Engine::initialize_bjt_history(&circuit, &base);
            let jfet_history = Engine::initialize_jfet_history(&circuit, &base);
            let diode_history = Engine::initialize_diode_history(&circuit, &base);
            let mosfet_history = Engine::initialize_mosfet_history(&circuit, &base);
            let vdmos_history = Engine::initialize_vdmos_history(&circuit, &base);
            let b3soi_history = Engine::initialize_b3soi_history(&circuit, &base);
            let bsim3_history = Engine::initialize_bsim3_history(&circuit, &base);
            let bsim4_history = Engine::initialize_bsim4_history(&circuit, &base);
            let ekv26_history = Engine::initialize_ekv26_history(&circuit, &base);
            let mut vbic_snapshot_cache = vec![None; circuit.bjts.devices.len()];
            circuit.set_semiconductor_junction_gmin(
                engine.effective_device_junction_gmin(engine.config.convergence_config.gmin_target),
            );
            let coeff = CompanionCoefficients::for_method(IntegrationMethod::BackwardEuler);
            let diode_companion_slots = Engine::link_diode_companion_slots(&circuit, &matrix);
            let mosfet_companion_slots = Engine::link_mosfet_companion_slots(&circuit, &matrix);
            let vdmos_companion_slots = Engine::link_vdmos_companion_slots(&circuit, &matrix);
            let ctx = TransientSystemContext {
                coeff: &coeff,
                xyce_one_step: false,
                xyce_one_step_order2: false,
                xyce_static_history: None,
                bsim4_trnqs_coeff: &coeff,
                bjt_history: &bjt_history,
                jfet_history: &jfet_history,
                diode_history: &diode_history,
                diode_companion_slots: &diode_companion_slots,
                mosfet_history: &mosfet_history,
                mosfet_companion_slots: &mosfet_companion_slots,
                vdmos_history: &vdmos_history,
                vdmos_companion_slots: &vdmos_companion_slots,
                b3soi_history: &b3soi_history,
                b3soi_zero_first_transient_charge_derivative: false,
                bsim3_history: &bsim3_history,
                bsim4_history: &bsim4_history,
                ekv26_history: &ekv26_history,
                suppress_gate_charge: false,
                baseline_diag_gmin: engine.config.convergence_config.gmin_target.max(0.0),
                tline_dc_refs: &tline_dc_refs,
                coupled_tline_refs: &coupled_tline_refs,
                analysis_initial_step: false,
                analysis_final_step: false,
            };

            // Reference mode: the dt=1e-12 power-iteration dominant. If it
            // is a genuine pencil eigenmode with a fixed pole `s`, its gain
            // under the other dts' maps must track 1/|1 − s·dt|; structural
            // junk holds or grows instead.
            let (reference_radius, reference_vector) = be_one_step_map_spectral_radius(
                &engine,
                &mut circuit,
                &mut matrix,
                &base,
                &ctx,
                &mut vbic_snapshot_cache,
                1e-12,
                600,
            );
            let implied_pole = (1.0 - 1.0 / reference_radius) / 1e-12;
            println!(
                "{}: reference dominant at dt=1e-12: radius {:.6} (implied real pole {:.4e})",
                label, reference_radius, implied_pole
            );
            for dt in [1e-12, 1e-11, 1e-10, 1e-9, 1e-8] {
                let (radius, dominant) = be_one_step_map_spectral_radius(
                    &engine,
                    &mut circuit,
                    &mut matrix,
                    &base,
                    &ctx,
                    &mut vbic_snapshot_cache,
                    dt,
                    600,
                );
                let fixed_vector_gain = be_one_step_map_gain_on_vector(
                    &engine,
                    &mut circuit,
                    &mut matrix,
                    &base,
                    &ctx,
                    &mut vbic_snapshot_cache,
                    dt,
                    &reference_vector,
                );
                let pole_prediction = 1.0 / (1.0 - implied_pole * dt).abs();
                // Per-dt eigenmode certification: this dt's dominant has two
                // real-pole branches consistent with |mu| = radius; a
                // genuine fixed pole must reproduce the measured gain at
                // dt/10 on one of them. Junk directions track neither.
                let probe_dt = dt / 10.0;
                let own_gain_at_probe = be_one_step_map_gain_on_vector(
                    &engine,
                    &mut circuit,
                    &mut matrix,
                    &base,
                    &ctx,
                    &mut vbic_snapshot_cache,
                    probe_dt,
                    &dominant,
                );
                let pole_lo = (1.0 - 1.0 / radius) / dt;
                let pole_hi = (1.0 + 1.0 / radius) / dt;
                let prediction_lo = 1.0 / (1.0 - pole_lo * probe_dt).abs();
                let prediction_hi = 1.0 / (1.0 - pole_hi * probe_dt).abs();
                let mut ranked: Vec<(usize, Value)> = dominant
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(i, v)| (i, v.abs()))
                    .collect();
                ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                let support: Vec<String> = ranked
                    .iter()
                    .take(6)
                    .map(|(i, magnitude)| {
                        format!("{}={:.3}", matrix_row_label(&circuit, *i), magnitude)
                    })
                    .collect();
                println!(
                    "{}: dt={:.0e}: radius {:.6} | ref-vector gain {:.6} (pred {:.6}) | own dominant at dt/10: gain {:.6} (pole branches predict {:.6} / {:.6})\n    support: {}",
                    label,
                    dt,
                    radius,
                    fixed_vector_gain,
                    pole_prediction,
                    own_gain_at_probe,
                    prediction_lo,
                    prediction_hi,
                    support.join(", "),
                );
            }
        }
    }
}
