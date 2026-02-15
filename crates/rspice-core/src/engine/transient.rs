//! Transient Time-Domain Analysis
//!
//! This module provides time-domain simulation using:
//! - Adaptive timestep control with LTE estimation
//! - TrapGear method switching for stability
//! - Optional waveform compression for long simulations
//! - Cooperative abort for responsive cancellation

use super::{Engine, SimulationError, TransientResult};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::transient::{
    BreakpointManager, CompanionCoefficients, IntegrationMethod, LteEstimator, TimestepController,
    TrapGearController,
};
use crate::analysis::waveform::{CompressionConfig, TransientResultCompressed, WaveformRecorder};
use crate::netlist::AnalysisCommand;
use crate::{Netlist, Value};

/// Maximum voltage limit for solution values (matching DC solver)
///
/// Commercial simulators like Spectre/HSPICE use similar limits to prevent
/// Newton-Raphson divergence on stiff nonlinear circuits (e.g., BJT exponential I-V).
/// This value matches the DC solver's MAX_VOLTAGE in convergence.rs for consistency.
const MAX_VOLTAGE: Value = 1000.0;
/// Maximum allowed per-iteration node update during Newton damping.
///
/// This bound controls nonlinear solve trust-region size.
const MAX_NEWTON_ITER_DELTA_V: Value = 5e-2;
/// Maximum allowed node update when committing force-accepted steps.
///
/// This remains tight to avoid committing nonphysical jumps into reactive history.
const MAX_FORCE_ACCEPT_DELTA_V: Value = 5e-2;

#[derive(Debug, Clone, Default)]
struct JfetTransientHistory {
    vgs_prev: Vec<Value>,
    vgs_prev_prev: Vec<Value>,
    igs_prev: Vec<Value>,
    vgd_prev: Vec<Value>,
    vgd_prev_prev: Vec<Value>,
    igd_prev: Vec<Value>,
}

impl Engine {
    #[inline]
    fn transient_source_step_hint(netlist: &Netlist, max_step: Value) -> Value {
        if let Some(step) = netlist.analyses.iter().find_map(|analysis| match analysis {
            AnalysisCommand::Tran { step, .. } if step.is_finite() && *step > 0.0 => Some(*step),
            _ => None,
        }) {
            step
        } else if max_step.is_finite() && max_step > 0.0 {
            (max_step / 10.0).max(1e-12)
        } else {
            1e-12
        }
    }

    /// Run transient time-domain analysis
    ///
    /// Uses adaptive integration with automatic method switching (TrapGear).
    /// Trapezoidal integration is used normally for efficiency, but switches
    /// to Gear2/BDF2 when oscillations are detected for stability.
    ///
    /// For cancellable simulations, use [`run_tran_with_abort`] instead.
    pub fn run_tran(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
    ) -> Result<TransientResult, SimulationError> {
        self.run_tran_with_abort(netlist, tstop, max_step, &NoAbort)
    }

    #[inline]
    fn max_abs_delta_prefix(a: &[Value], b: &[Value], count: usize) -> Value {
        a.iter()
            .zip(b.iter())
            .take(count)
            .map(|(x, y)| (x - y).abs())
            .fold(0.0, Value::max)
    }

    #[inline]
    fn is_stale_step(
        previous_solution: &[Value],
        candidate_solution: &[Value],
        expected_source_delta: Value,
        num_nodes: usize,
    ) -> bool {
        // No externally-driven movement expected for this step.
        if expected_source_delta <= 1e-12 {
            return false;
        }

        // If the entire solution moves orders of magnitude less than the source
        // should have moved, the solver likely accepted a stale state.
        let observed_delta =
            Self::max_abs_delta_prefix(previous_solution, candidate_solution, num_nodes);
        observed_delta <= expected_source_delta * 1e-3
    }

    #[inline]
    fn is_unbounded_step(
        previous_solution: &[Value],
        candidate_solution: &[Value],
        expected_source_delta: Value,
        num_nodes: usize,
    ) -> bool {
        let observed_delta =
            Self::max_abs_delta_prefix(previous_solution, candidate_solution, num_nodes);
        // Guard only truly explosive step jumps. We intentionally allow bounded
        // multi-volt recovery movement here because force-accept paths may need
        // larger-than-source-following corrections to escape stiff NR stalls.
        let drive_scale = expected_source_delta.max(1e-6);
        let threshold = (drive_scale * 1e5).max(50.0);
        observed_delta > threshold
    }

    #[inline]
    fn is_clipped_force_candidate(
        previous_solution: &[Value],
        candidate_solution: &[Value],
        num_nodes: usize,
    ) -> bool {
        let clip_threshold = MAX_FORCE_ACCEPT_DELTA_V * 0.99;
        let mut clipped = 0usize;

        for (old_v, new_v) in previous_solution
            .iter()
            .zip(candidate_solution.iter())
            .take(num_nodes)
        {
            let delta = *new_v - *old_v;
            if delta.abs() >= clip_threshold {
                clipped += 1;
            }
        }

        if clipped < 2 {
            return false;
        }

        let min_clipped_nodes = (num_nodes / 2).max(2);
        clipped >= min_clipped_nodes
    }

    #[inline]
    fn node_voltage(solution: &[Value], node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            solution.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    #[inline]
    fn differential_voltage(solution: &[Value], node_pos: usize, node_neg: usize) -> Value {
        Self::node_voltage(solution, node_pos) - Self::node_voltage(solution, node_neg)
    }

    #[inline]
    fn stamp_tline_port(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        node_pos: usize,
        node_neg: usize,
        g: Value,
        i_eq: Value,
    ) {
        if node_pos > 0 {
            matrix.add(node_pos - 1, node_pos - 1, g);
            if node_neg > 0 {
                matrix.add(node_pos - 1, node_neg - 1, -g);
            }
            rhs[node_pos - 1] += i_eq;
        }
        if node_neg > 0 {
            if node_pos > 0 {
                matrix.add(node_neg - 1, node_pos - 1, -g);
            }
            matrix.add(node_neg - 1, node_neg - 1, g);
            rhs[node_neg - 1] -= i_eq;
        }
    }

    #[inline]
    fn stamp_two_terminal_companion(
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        node_pos: usize,
        node_neg: usize,
        geq: Value,
        i_eq: Value,
    ) {
        if node_pos > 0 {
            matrix.add(node_pos - 1, node_pos - 1, geq);
            if node_neg > 0 {
                matrix.add(node_pos - 1, node_neg - 1, -geq);
            }
            rhs[node_pos - 1] += i_eq;
        }
        if node_neg > 0 {
            if node_pos > 0 {
                matrix.add(node_neg - 1, node_pos - 1, -geq);
            }
            matrix.add(node_neg - 1, node_neg - 1, geq);
            rhs[node_neg - 1] -= i_eq;
        }
    }

    #[inline]
    fn initialize_jfet_history(
        circuit: &crate::circuit::Circuit,
        solution: &[Value],
    ) -> JfetTransientHistory {
        let n = circuit.jfets.len();
        let mut history = JfetTransientHistory {
            vgs_prev: Vec::with_capacity(n),
            vgs_prev_prev: Vec::with_capacity(n),
            igs_prev: Vec::with_capacity(n),
            vgd_prev: Vec::with_capacity(n),
            vgd_prev_prev: Vec::with_capacity(n),
            igd_prev: Vec::with_capacity(n),
        };

        for jfet in &circuit.jfets {
            let vg = Self::node_voltage(solution, jfet.gate);
            let vd = Self::node_voltage(solution, jfet.drain);
            let vs = Self::node_voltage(solution, jfet.source);
            let vgs = vg - vs;
            let vgd = vg - vd;
            history.vgs_prev.push(vgs);
            history.vgs_prev_prev.push(vgs);
            history.igs_prev.push(0.0);
            history.vgd_prev.push(vgd);
            history.vgd_prev_prev.push(vgd);
            history.igd_prev.push(0.0);
        }

        history
    }

    #[inline]
    fn stamp_jfet_transient_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        dt: Value,
        coeff: &CompanionCoefficients,
        history: &JfetTransientHistory,
    ) {
        for (idx, jfet) in circuit.jfets.iter().enumerate() {
            let vg = Self::node_voltage(voltages, jfet.gate);
            let vd = Self::node_voltage(voltages, jfet.drain);
            let vs = Self::node_voltage(voltages, jfet.source);
            let vgs = vg - vs;
            let vgd = vg - vd;
            let pol = jfet.jfet_type.polarity();
            let (cgs, cgd) = jfet.capacitances(pol * vgs, pol * vgd);

            if cgs.is_finite() && cgs > 0.0 {
                let geq = coeff.capacitor_geq(cgs, dt);
                let ieq = coeff.capacitor_ieq(
                    cgs,
                    dt,
                    history.vgs_prev[idx],
                    history.vgs_prev_prev[idx],
                    history.igs_prev[idx],
                );
                Self::stamp_two_terminal_companion(matrix, rhs, jfet.gate, jfet.source, geq, ieq);
            }

            if cgd.is_finite() && cgd > 0.0 {
                let geq = coeff.capacitor_geq(cgd, dt);
                let ieq = coeff.capacitor_ieq(
                    cgd,
                    dt,
                    history.vgd_prev[idx],
                    history.vgd_prev_prev[idx],
                    history.igd_prev[idx],
                );
                Self::stamp_two_terminal_companion(matrix, rhs, jfet.gate, jfet.drain, geq, ieq);
            }
        }
    }

    #[inline]
    fn stamp_tline_companions(
        circuit: &crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        time: Value,
    ) {
        for tl in &circuit.tlines {
            let g = tl.conductance();
            let i_eq_p1 = g * tl.delayed_backward_at(time);
            let i_eq_p2 = g * tl.delayed_forward_at(time);

            Self::stamp_tline_port(matrix, rhs, tl.node1_pos, tl.node1_neg, g, i_eq_p1);
            Self::stamp_tline_port(matrix, rhs, tl.node2_pos, tl.node2_neg, g, i_eq_p2);
        }
    }

    #[inline]
    fn initialize_tline_history(
        circuit: &mut crate::circuit::Circuit,
        initial_solution: &[Value],
        initial_time: Value,
    ) {
        for tl in &mut circuit.tlines {
            tl.reset();
            let g = tl.conductance();
            let v1 = Self::differential_voltage(initial_solution, tl.node1_pos, tl.node1_neg);
            let v2 = Self::differential_voltage(initial_solution, tl.node2_pos, tl.node2_neg);

            // Start with zero incoming delayed waves at t=0.
            let i1 = g * v1;
            let i2 = g * v2;
            tl.update_history(initial_time, v1, i1, v2, i2);
        }
    }

    #[inline]
    fn update_reactive_history(
        circuit: &mut crate::circuit::Circuit,
        accepted_solution: &[Value],
        accepted_time: Value,
        dt: Value,
        method: IntegrationMethod,
        jfet_history: &mut JfetTransientHistory,
    ) {
        for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            let np = cap.pp.row;
            let nn = cap.nn.row;
            let v_new = Self::differential_voltage(accepted_solution, np, nn);

            // Compute new capacitor current from OLD history before rotating it.
            let coeff_update = CompanionCoefficients::for_method(method);
            let geq = coeff_update.capacitor_geq(circuit.capacitors.capacitances[cap_idx], dt);
            let ieq = coeff_update.capacitor_ieq(
                circuit.capacitors.capacitances[cap_idx],
                dt,
                circuit.capacitors.v_prev[cap_idx],
                circuit.capacitors.v_prev_prev[cap_idx],
                circuit.capacitors.i_prev[cap_idx],
            );
            let i_new = geq * v_new - ieq;

            let v_old = circuit.capacitors.v_prev[cap_idx];
            circuit.capacitors.v_prev_prev[cap_idx] = v_old;
            circuit.capacitors.v_prev[cap_idx] = v_new;
            circuit.capacitors.i_prev[cap_idx] = i_new;
        }

        for l_idx in 0..circuit.inductors.names.len() {
            let br = circuit.inductors.branch_indices[l_idx];
            if br > 0 {
                let br_idx = circuit.num_nodes() + br - 1;
                let i_new = accepted_solution[br_idx];
                circuit.inductors.i_prev_prev[l_idx] = circuit.inductors.i_prev[l_idx];
                circuit.inductors.i_prev[l_idx] = i_new;

                let np = circuit.inductors.node_pos[l_idx];
                let nn = circuit.inductors.node_neg[l_idx];
                let v_new = Self::differential_voltage(accepted_solution, np, nn);
                circuit.inductors.v_prev[l_idx] = v_new;
            }
        }
        circuit.refresh_jiles_atherton_inductances(accepted_solution);

        // Update transmission-line delayed-wave history from the accepted state.
        for tl in &mut circuit.tlines {
            let g = tl.conductance();
            let v1 = Self::differential_voltage(accepted_solution, tl.node1_pos, tl.node1_neg);
            let v2 = Self::differential_voltage(accepted_solution, tl.node2_pos, tl.node2_neg);
            let incoming_port1 = tl.delayed_backward_at(accepted_time);
            let incoming_port2 = tl.delayed_forward_at(accepted_time);
            let i1 = g * v1 - g * incoming_port1;
            let i2 = g * v2 - g * incoming_port2;
            tl.update_history(accepted_time, v1, i1, v2, i2);
        }

        let coeff_update = CompanionCoefficients::for_method(method);
        for (idx, jfet) in circuit.jfets.iter().enumerate() {
            let vg = Self::node_voltage(accepted_solution, jfet.gate);
            let vd = Self::node_voltage(accepted_solution, jfet.drain);
            let vs = Self::node_voltage(accepted_solution, jfet.source);
            let vgs = vg - vs;
            let vgd = vg - vd;
            let pol = jfet.jfet_type.polarity();
            let (cgs, cgd) = jfet.capacitances(pol * vgs, pol * vgd);

            if cgs.is_finite() && cgs > 0.0 {
                let geq = coeff_update.capacitor_geq(cgs, dt);
                let ieq = coeff_update.capacitor_ieq(
                    cgs,
                    dt,
                    jfet_history.vgs_prev[idx],
                    jfet_history.vgs_prev_prev[idx],
                    jfet_history.igs_prev[idx],
                );
                let i_new = geq * vgs - ieq;
                let v_old = jfet_history.vgs_prev[idx];
                jfet_history.vgs_prev_prev[idx] = v_old;
                jfet_history.vgs_prev[idx] = vgs;
                jfet_history.igs_prev[idx] = i_new;
            } else {
                jfet_history.vgs_prev_prev[idx] = jfet_history.vgs_prev[idx];
                jfet_history.vgs_prev[idx] = vgs;
                jfet_history.igs_prev[idx] = 0.0;
            }

            if cgd.is_finite() && cgd > 0.0 {
                let geq = coeff_update.capacitor_geq(cgd, dt);
                let ieq = coeff_update.capacitor_ieq(
                    cgd,
                    dt,
                    jfet_history.vgd_prev[idx],
                    jfet_history.vgd_prev_prev[idx],
                    jfet_history.igd_prev[idx],
                );
                let i_new = geq * vgd - ieq;
                let v_old = jfet_history.vgd_prev[idx];
                jfet_history.vgd_prev_prev[idx] = v_old;
                jfet_history.vgd_prev[idx] = vgd;
                jfet_history.igd_prev[idx] = i_new;
            } else {
                jfet_history.vgd_prev_prev[idx] = jfet_history.vgd_prev[idx];
                jfet_history.vgd_prev[idx] = vgd;
                jfet_history.igd_prev[idx] = 0.0;
            }
        }
    }

    /// Run transient analysis with abort signal for cancellation
    ///
    /// This method supports cooperative cancellation via the `AbortSignal` trait.
    /// The abort signal is checked every 1000 iterations for minimal overhead.
    ///
    /// # Arguments
    ///
    /// * `netlist` - The circuit netlist to simulate
    /// * `tstop` - Stop time for the simulation
    /// * `max_step` - Maximum timestep size
    /// * `abort` - Abort signal for cancellation (use `&NoAbort` if not needed)
    ///
    /// # Returns
    ///
    /// Returns simulation results up to the point of abort, or an error if aborted.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use rspice_core::{Engine, AtomicAbort, AbortSignal};
    /// use std::sync::Arc;
    ///
    /// let abort = Arc::new(AtomicAbort::new());
    /// let abort_clone = Arc::clone(&abort);
    ///
    /// // In another thread: abort_clone.set();
    ///
    /// let result = engine.run_tran_with_abort(&netlist, 10e-3, 1e-6, &*abort);
    /// ```
    pub fn run_tran_with_abort(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<TransientResult, SimulationError> {
        let mut circuit = self.build_circuit(netlist)?;
        let mut matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        let source_step_hint = Self::transient_source_step_hint(netlist, max_step);
        circuit
            .voltage_sources
            .set_transient_context(source_step_hint, tstop);
        circuit
            .current_sources
            .set_transient_context(source_step_hint, tstop);

        // Get DC operating point as initial condition.
        let mut solution = self.solve_dc_operating_point(netlist, &mut circuit, &mut matrix)?;
        let applied_ic = self.apply_initial_condition_overrides(netlist, &circuit, &mut solution);
        circuit.refresh_jiles_atherton_inductances(&solution);

        let num_nodes = circuit.num_nodes();
        let size = circuit.matrix_size();

        // Initialize timestep controller
        // Use a practical min_timestep based on user's max_step request
        // This prevents timestep from shrinking to impractically small values for stiff circuits
        let initial_step = (max_step / 10.0).min(tstop / 100.0);
        let practical_min = (max_step / 1000.0).max(self.config.min_timestep);
        let mut timestep = TimestepController::new(initial_step, practical_min, max_step);
        let mut breakpoints = BreakpointManager::new();
        let mut lte_estimator =
            LteEstimator::with_tolerances(self.voltage_reltol(), self.voltage_abstol());

        // Initialize TrapGear controller for automatic method switching
        let mut trapgear = TrapGearController::new();

        // Track integration method order for LTE scaling
        let method_order = |method: IntegrationMethod| -> u32 {
            match method {
                IntegrationMethod::BackwardEuler => 1,
                _ => 2, // Trapezoidal and Gear2 are both order 2
            }
        };

        // Initialize result storage with actual node names from netlist
        let node_names = circuit.node_names_sorted();

        // Debug: log node names and their indices to verify alignment
        log::info!("Node mapping (index -> name, DC voltage):");
        for (i, name) in node_names.iter().enumerate() {
            let dc_v = solution.get(i).copied().unwrap_or(0.0);
            log::info!("  Node[{}] = '{}', V_dc = {:.4}", i, name, dc_v);
        }
        if applied_ic > 0 {
            log::info!(
                "Applied {} .IC node override(s) to transient initial state",
                applied_ic
            );
        }

        let mut result = TransientResult {
            time: vec![0.0],
            voltages: (0..num_nodes)
                .map(|i| vec![solution.get(i).copied().unwrap_or(0.0)])
                .collect(),
            num_nodes,
            node_names,
        };
        let mut t = 0.0;

        // Initialize capacitor voltage history from DC solution
        for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            let np = cap.pp.row;
            let nn = cap.nn.row;
            let v_dc = if np == 0 { 0.0 } else { solution[np - 1] }
                - if nn == 0 { 0.0 } else { solution[nn - 1] };
            circuit.capacitors.v_prev[cap_idx] = v_dc;
            circuit.capacitors.v_prev_prev[cap_idx] = v_dc;
            log::info!(
                "Capacitor {} init: v_dc={:.4}, np={}, nn={}",
                circuit.capacitors.names[cap_idx],
                v_dc,
                np,
                nn
            );
        }

        // Initialize inductor current and voltage history from DC solution
        for l_idx in 0..circuit.inductors.names.len() {
            let np = circuit.inductors.node_pos[l_idx];
            let nn = circuit.inductors.node_neg[l_idx];
            let br = circuit.inductors.branch_indices[l_idx];

            // Initialize voltage across inductor from DC solution
            let v_dc = if np == 0 { 0.0 } else { solution[np - 1] }
                - if nn == 0 { 0.0 } else { solution[nn - 1] };
            circuit.inductors.v_prev[l_idx] = v_dc;

            // Initialize branch currents from DC solution
            if br > 0 {
                let br_idx = circuit.num_nodes() + br - 1;
                let i_dc = solution[br_idx];
                circuit.inductors.i_prev[l_idx] = i_dc;
                circuit.inductors.i_prev_prev[l_idx] = i_dc;
            }
        }
        Self::initialize_tline_history(&mut circuit, &solution, 0.0);
        let mut jfet_history = Self::initialize_jfet_history(&circuit, &solution);

        // Main transient loop
        let mut retry_count = 0;
        let mut total_iterations = 0;
        let mut stale_accept_count = 0;
        let mut force_accept_cooldown = 0_usize; // Steps to skip dt reduction after force-accept
        const MAX_RETRIES: usize = 20; // Maximum retries per timepoint before force-accept
        const MAX_WALL_TIME_SECS: u64 = 300; // Wall-clock timeout (5 minutes - use abort for earlier cancellation)
        const ABORT_CHECK_INTERVAL: usize = 1000; // Check abort every N iterations for performance
        let estimated_steps = ((tstop / max_step).ceil().max(1.0) as usize).saturating_add(1);
        let max_total_iterations = estimated_steps.saturating_mul(40).max(10_000_000);
        let wall_start = std::time::Instant::now();
        let mut last_progress_log = std::time::Instant::now();

        while t < tstop && total_iterations < max_total_iterations {
            // Wall-clock timeout check
            if wall_start.elapsed().as_secs() > MAX_WALL_TIME_SECS {
                log::warn!(
                    "Transient simulation wall-clock timeout after {}s at t={:.3e}s ({:.1}% complete)",
                    MAX_WALL_TIME_SECS,
                    t,
                    (t / tstop) * 100.0
                );
                break;
            }

            // Progress logging every 2 seconds
            if last_progress_log.elapsed().as_secs() >= 2 {
                log::info!(
                    "Transient progress: t={:.3e}s / {:.3e}s ({:.1}%), {} iterations",
                    t,
                    tstop,
                    (t / tstop) * 100.0,
                    total_iterations
                );
                last_progress_log = std::time::Instant::now();
            }

            // Abort check - check every ABORT_CHECK_INTERVAL iterations for minimal overhead
            if total_iterations % ABORT_CHECK_INTERVAL == 0 {
                let is_aborted = abort.is_aborted();
                if total_iterations == 0 {
                    log::debug!("First abort check, is_aborted={}", is_aborted);
                }
                if is_aborted {
                    log::info!(
                        "Transient simulation aborted at t={:.3e}s ({:.1}% complete, {} iterations)",
                        t,
                        (t / tstop) * 100.0,
                        total_iterations
                    );
                    // Return error indicating abort - partial results are lost
                    return Err(SimulationError::Aborted);
                }
            }

            total_iterations += 1;
            let (dt, _at_breakpoint) = breakpoints.limit_step(t, timestep.dt());
            let dt = dt.min(tstop - t); // Don't overshoot tstop
            let expected_source_delta = circuit.voltage_sources.max_expected_delta(t, t + dt);

            // Prepare for Newton iteration at this timestep
            let mut new_solution = solution.clone();
            let mut rhs = vec![0.0; size];
            let mut had_solver_candidate = true;

            // Newton-Raphson iteration for this timestep.
            // Transient nonlinear regions (e.g., BJT turn-on) often need more
            // iterations than DC. Use a higher budget here to reduce force-accept.
            let tran_max_iterations = (self.config.max_iterations.saturating_mul(4)).min(400);
            let mut converged = false;
            for _iter in 0..tran_max_iterations {
                matrix.clear_values();
                rhs.fill(0.0);

                // Add GMIN diagonal
                for i in 0..size {
                    matrix.add(i, i, 1e-12);
                }

                // Stamp linear devices (R, V, I) - this stamps DC values initially
                circuit.stamp_dc_direct(&mut matrix, &mut rhs);

                // Update voltage source RHS values for time-varying sources (PULSE, SIN, etc.)
                let num_nodes = circuit.num_nodes();
                circuit.voltage_sources.update_transient_rhs(
                    &mut rhs,
                    t + dt, // Evaluate at target time point
                    |br_ordinal| num_nodes + br_ordinal,
                );
                circuit.current_sources.update_transient_rhs(&mut rhs, t + dt);

                // Get current integration method from TrapGear controller
                let current_method = trapgear.current_method();
                let coeff = CompanionCoefficients::for_method(current_method);
                circuit.refresh_jiles_atherton_inductances(&new_solution);

                // Stamp capacitor companion models for transient
                circuit
                    .capacitors
                    .stamp_transient_companion(&mut matrix, &mut rhs, dt, &coeff);

                // Stamp inductor companion models for transient
                circuit.inductors.stamp_transient_companion(
                    &mut matrix,
                    &mut rhs,
                    dt,
                    &coeff,
                    num_nodes,
                );
                Self::stamp_jfet_transient_companions(
                    &circuit,
                    &mut matrix,
                    &mut rhs,
                    &new_solution,
                    dt,
                    &coeff,
                    &jfet_history,
                );
                Self::stamp_tline_companions(&circuit, &mut matrix, &mut rhs, t + dt);

                // Stamp nonlinear devices if present
                if circuit.has_nonlinear_devices() {
                    circuit.update_nonlinear(&new_solution);
                    circuit.stamp_nonlinear(&mut matrix, &mut rhs, &new_solution);
                    circuit.stamp_behavioral(&mut matrix, &mut rhs, &new_solution, t + dt);
                }

                // Evaluate and stamp XSPICE code models
                if circuit.has_xspice_devices() {
                    circuit.evaluate_xspice_with_timestep(t + dt, dt, &new_solution);
                    circuit.stamp_xspice(&mut matrix, &mut rhs);
                }

                // Solve and check convergence
                match matrix.solve(&rhs) {
                    Ok(mut sol) => {
                        had_solver_candidate = true;
                        // Sanity check: detect and handle NaN/Inf/excessive values.
                        // IMPORTANT: Preserve the newest valid candidate when possible.
                        // If we keep the previous timestep guess here, force-accept can
                        // propagate a stale state and flatten non-source traces.
                        let mut has_bad_values = false;
                        let mut logged_divergence = false;

                        for (i, v) in sol.iter_mut().enumerate() {
                            if !v.is_finite() {
                                if !logged_divergence {
                                    log::debug!(
                                        "Transient: Newton divergence at t={:.3e}s, node {}: {:.3e} - reducing timestep",
                                        t + dt,
                                        i,
                                        *v
                                    );
                                    logged_divergence = true;
                                }
                                // Non-finite values cannot be used; fall back to prior guess.
                                *v = new_solution[i];
                                has_bad_values = true;
                            } else if v.abs() > MAX_VOLTAGE {
                                if !logged_divergence {
                                    log::debug!(
                                        "Transient: Newton divergence at t={:.3e}s, node {}: {:.3e} - reducing timestep",
                                        t + dt,
                                        i,
                                        *v
                                    );
                                    logged_divergence = true;
                                }
                                // Soft-limit finite overflow around previous Newton guess
                                // instead of hard-clamping to +/-MAX_VOLTAGE. Hard rail
                                // clamps can be force-accepted and then contaminate dynamic
                                // history with nonphysical state.
                                let old = new_solution[i];
                                let delta = *v - old;
                                if delta.is_finite() {
                                    *v = old + delta.signum() * MAX_NEWTON_ITER_DELTA_V;
                                } else {
                                    *v = old;
                                }
                                has_bad_values = true;
                            }
                        }

                        // Apply Newton damping to node voltages every iteration.
                        // This keeps transient NR inside a trust region in stiff nonlinear zones
                        // (e.g., BJT turn-on) and mirrors commercial solver stabilization.
                        for i in 0..num_nodes {
                            let old = new_solution[i];
                            let delta = sol[i] - old;
                            if delta.is_finite() && delta.abs() > MAX_NEWTON_ITER_DELTA_V {
                                sol[i] = old + delta.signum() * MAX_NEWTON_ITER_DELTA_V;
                            }
                        }

                        // If this Newton step was numerically bad, keep the sanitized
                        // candidate and continue Newton iterations.
                        if has_bad_values {
                            new_solution = sol;
                            continue;
                        }

                        let voltage_converged = self.voltage_convergence_met(&new_solution, &sol);
                        let linearized_residual_converged =
                            self.residual_convergence_met(&matrix, &sol, &rhs);

                        // CRITICAL: Update new_solution BEFORE checking device convergence
                        // Otherwise, BJT vbe/vbc are based on old guess, not new solve
                        new_solution = sol;

                        // Update nonlinear device state to new solution for accurate convergence check
                        if circuit.has_nonlinear_devices() {
                            circuit.update_nonlinear(&new_solution);
                        }

                        let device_converged = !circuit.has_nonlinear_devices()
                            || circuit.nonlinear_converged(self.device_convergence_tolerance());

                        if voltage_converged && device_converged && linearized_residual_converged {
                            converged = true;
                            break;
                        }
                    }
                    Err(e) => {
                        log::debug!(
                            "Transient solve failed at t={:.6e}, dt={:.3e}: {}",
                            t + dt,
                            dt,
                            e
                        );
                        break;
                    }
                }
            }

            if !converged {
                retry_count += 1;

                // Diagnostic logging for debugging convergence issues
                static CONV_LOG_COUNT: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                let count = CONV_LOG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if count < 10 {
                    // Check what specifically didn't converge
                    let v_conv = self.voltage_convergence_met(&solution, &new_solution);
                    let d_conv = !circuit.has_nonlinear_devices()
                        || circuit.nonlinear_converged(self.device_convergence_tolerance());
                    let r_conv = self.residual_convergence_met(&matrix, &new_solution, &rhs);
                    let max_dv = Self::max_abs_delta_prefix(&solution, &new_solution, num_nodes);
                    log::warn!(
                        "Newton non-converge at t={:.6e}, dt={:.3e}: voltage_conv={}, device_conv={}, residual_conv={}, max_dv={:.3e}, iter={}",
                        t,
                        dt,
                        v_conv,
                        d_conv,
                        r_conv,
                        max_dv,
                        total_iterations
                    );
                }

                // Diagnostic logging for debugging timestep issues
                if total_iterations < 100 || total_iterations % 10000 == 0 {
                    log::debug!(
                        "Newton non-convergence at t={:.3e}s, iter={}, dt={:.3e}s, reducing to {:.3e}s",
                        t,
                        total_iterations,
                        dt,
                        dt * 0.25
                    );
                }

                // Convergence failed - reduce timestep significantly (4x) and retry
                // BUT skip reduction if we're in post-force-accept cooldown to avoid ping-pong
                if force_accept_cooldown > 0 {
                    force_accept_cooldown -= 1;
                    // During cooldown, keep timestep at current level (don't shrink)
                } else {
                    timestep.force_step(dt * 0.25);
                }

                // Force accept when recovery is unlikely:
                // - After MAX_RETRIES attempts (regardless of timestep state), OR
                // - At minimum timestep AND at least MIN_RETRIES_AT_MIN have been tried
                // This prevents both infinite loops and force-accept floods
                const MIN_RETRIES_AT_MIN: usize = 3; // Give a few attempts at min dt
                let at_min_dt = timestep.is_at_minimum();
                let exhausted_retries = retry_count >= MAX_RETRIES;
                let exhausted_at_min = at_min_dt && retry_count >= MIN_RETRIES_AT_MIN;

                if exhausted_retries || exhausted_at_min {
                    let unbounded_force_candidate = Self::is_unbounded_step(
                        &solution,
                        &new_solution,
                        expected_source_delta,
                        num_nodes,
                    );
                    let stale_force_candidate = Self::is_stale_step(
                        &solution,
                        &new_solution,
                        expected_source_delta,
                        num_nodes,
                    );

                    if unbounded_force_candidate || !had_solver_candidate || stale_force_candidate {
                        stale_accept_count += 1;
                        let boosted = (dt * 4.0).min(max_step);
                        if boosted > dt {
                            timestep.force_step(boosted);
                        }
                        if stale_accept_count >= 8 {
                            if unbounded_force_candidate {
                                log::error!(
                                    "Transient diverged at t={:.6e}s: repeated unbounded force-accept candidates",
                                    t
                                );
                            } else {
                                log::error!(
                                    "Transient stalled near t={:.6e}s: stale force-accept candidates with active sources",
                                    t
                                );
                            }
                            return Err(SimulationError::ConvergenceFailed(total_iterations));
                        }
                        continue;
                    }
                    let clipped_force_candidate =
                        Self::is_clipped_force_candidate(&solution, &new_solution, num_nodes);
                    if clipped_force_candidate {
                        trapgear.force_method(IntegrationMethod::Gear2);
                        timestep.force_step((dt * 0.5).min(max_step));
                    }
                    stale_accept_count = 0;

                    t += dt;

                    // FORCE-ACCEPT: Use the unconverged Newton result as-is.
                    // While not fully converged, the Newton result is still a valid
                    // approximation that respects circuit equations (just with residual error).
                    // This is the standard SPICE approach for force-accept.
                    // Voltage sources are enforced to ensure correct input values.
                    circuit
                        .voltage_sources
                        .enforce_voltage_constraints(&mut new_solution, t);

                    // Keep forced movement tightly bounded to prevent long-run drift
                    // when a region requires repeated force-accepts.
                    for i in 0..num_nodes {
                        let old = solution[i];
                        let delta = new_solution[i] - old;
                        if delta.is_finite() && delta.abs() > MAX_FORCE_ACCEPT_DELTA_V {
                            new_solution[i] = old + delta.signum() * MAX_FORCE_ACCEPT_DELTA_V;
                        }
                    }
                    circuit
                        .voltage_sources
                        .enforce_voltage_constraints(&mut new_solution, t);

                    lte_estimator.record(&new_solution, dt);
                    lte_estimator.set_method_order(method_order(trapgear.current_method()));
                    trapgear.update(&new_solution, dt);
                    Self::update_reactive_history(
                        &mut circuit,
                        &new_solution,
                        t,
                        dt,
                        trapgear.current_method(),
                        &mut jfet_history,
                    );
                    if circuit.has_xspice_devices() {
                        circuit.accept_xspice_timestep();
                    }

                    solution = new_solution;
                    result.time.push(t);
                    for (i, voltages) in result.voltages.iter_mut().enumerate() {
                        voltages.push(solution.get(i).copied().unwrap_or(0.0));
                    }

                    // Debug: log force-accept values for node 0
                    let v0_force = solution.get(0).copied().unwrap_or(0.0);
                    static FORCE_LOG_COUNT: std::sync::atomic::AtomicUsize =
                        std::sync::atomic::AtomicUsize::new(0);
                    let count = FORCE_LOG_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if count < 20 {
                        log::warn!(
                            "FORCE-ACCEPT at t={:.6e}: v0={:.4}, retry_count={}",
                            t,
                            v0_force,
                            retry_count
                        );
                    }

                    retry_count = 0; // Reset for next timepoint

                    // Set cooldown to prevent immediate timestep shrinkage on next failure
                    // This gives the simulation time to progress through the difficult region
                    force_accept_cooldown = 50;

                    // Grow timestep after force-accept to escape minimum
                    let new_dt = (dt * 2.0).min(max_step);
                    timestep.force_step(new_dt);
                }
                continue;
            }

            // Check LTE for physics accuracy
            let (lte, accept) = lte_estimator.estimate(&new_solution, dt);
            if !accept {
                retry_count += 1;
                let scale = lte_estimator.recommend_scale(lte);
                timestep.adjust(lte / scale);

                // Force accept when recovery is unlikely:
                // - After MAX_RETRIES attempts (regardless of timestep state), OR
                // - At minimum timestep AND at least MIN_RETRIES_AT_MIN have been tried
                // This prevents both infinite loops and force-accept floods
                const MIN_RETRIES_AT_MIN: usize = 3;
                let at_min_dt = timestep.is_at_minimum();
                let exhausted_retries = retry_count >= MAX_RETRIES;
                let exhausted_at_min = at_min_dt && retry_count >= MIN_RETRIES_AT_MIN;

                if exhausted_retries || exhausted_at_min {
                    let unbounded_force_candidate = Self::is_unbounded_step(
                        &solution,
                        &new_solution,
                        expected_source_delta,
                        num_nodes,
                    );
                    let stale_force_candidate = Self::is_stale_step(
                        &solution,
                        &new_solution,
                        expected_source_delta,
                        num_nodes,
                    );

                    if unbounded_force_candidate || !had_solver_candidate || stale_force_candidate {
                        stale_accept_count += 1;
                        let boosted = (dt * 4.0).min(max_step);
                        if boosted > dt {
                            timestep.force_step(boosted);
                        }
                        if stale_accept_count >= 8 {
                            if unbounded_force_candidate {
                                log::error!(
                                    "Transient diverged at t={:.6e}s: repeated unbounded LTE force-accept candidates",
                                    t
                                );
                            } else {
                                log::error!(
                                    "Transient stalled near t={:.6e}s: stale LTE force-accept candidates with active sources",
                                    t
                                );
                            }
                            return Err(SimulationError::ConvergenceFailed(total_iterations));
                        }
                        continue;
                    }
                    let clipped_force_candidate =
                        Self::is_clipped_force_candidate(&solution, &new_solution, num_nodes);
                    if clipped_force_candidate {
                        trapgear.force_method(IntegrationMethod::Gear2);
                        timestep.force_step((dt * 0.5).min(max_step));
                    }
                    stale_accept_count = 0;

                    t += dt;
                    circuit
                        .voltage_sources
                        .enforce_voltage_constraints(&mut new_solution, t);

                    for i in 0..num_nodes {
                        let old = solution[i];
                        let delta = new_solution[i] - old;
                        if delta.is_finite() && delta.abs() > MAX_FORCE_ACCEPT_DELTA_V {
                            new_solution[i] = old + delta.signum() * MAX_FORCE_ACCEPT_DELTA_V;
                        }
                    }
                    circuit
                        .voltage_sources
                        .enforce_voltage_constraints(&mut new_solution, t);

                    lte_estimator.record(&new_solution, dt);
                    lte_estimator.set_method_order(method_order(trapgear.current_method()));
                    trapgear.update(&new_solution, dt);
                    Self::update_reactive_history(
                        &mut circuit,
                        &new_solution,
                        t,
                        dt,
                        trapgear.current_method(),
                        &mut jfet_history,
                    );
                    if circuit.has_xspice_devices() {
                        circuit.accept_xspice_timestep();
                    }

                    solution = new_solution;
                    result.time.push(t);
                    for (i, voltages) in result.voltages.iter_mut().enumerate() {
                        voltages.push(solution.get(i).copied().unwrap_or(0.0));
                    }
                    retry_count = 0; // Reset for next timepoint
                    force_accept_cooldown = 50; // Same stability window as primary path

                    // Grow timestep after force-accept (same as primary path)
                    let new_dt = (dt * 2.0).min(max_step);
                    timestep.force_step(new_dt);
                }
                continue;
            }

            // Success - reset retry counter
            retry_count = 0;

            // Keep ideal source constraints exact before LTE and state updates.
            circuit
                .voltage_sources
                .enforce_voltage_constraints(&mut new_solution, t + dt);

            if Self::is_stale_step(&solution, &new_solution, expected_source_delta, num_nodes) {
                stale_accept_count += 1;
                let boosted = (dt * 2.0).min(max_step);
                if boosted > dt {
                    timestep.force_step(boosted);
                }
                if stale_accept_count >= 8 {
                    log::error!(
                        "Transient stalled near t={:.6e}s: repeated stale accepted steps with active sources",
                        t
                    );
                    return Err(SimulationError::ConvergenceFailed(total_iterations));
                }
                continue;
            }
            stale_accept_count = 0;

            // Accept this timestep
            t += dt;
            lte_estimator.record(&new_solution, dt);
            lte_estimator.set_method_order(method_order(trapgear.current_method()));
            trapgear.update(&new_solution, dt);

            // CRITICAL: Grow timestep back after successful convergence
            // This prevents staying stuck at minimum timestep after force-accepts
            // LTE estimator recommends scale based on error - apply it to timestep
            let (lte, _) = lte_estimator.estimate(&new_solution, dt);
            let scale = lte_estimator.recommend_scale(lte);
            if scale > 1.0 {
                // Grow timestep gradually (limit to 1.5x per successful step)
                let new_dt = (dt * scale.min(1.5)).min(max_step);
                timestep.force_step(new_dt);
            }
            Self::update_reactive_history(
                &mut circuit,
                &new_solution,
                t,
                dt,
                trapgear.current_method(),
                &mut jfet_history,
            );

            // Accept XSPICE timestep (commit state changes)
            if circuit.has_xspice_devices() {
                circuit.accept_xspice_timestep();
            }

            solution = new_solution;

            // Store results
            result.time.push(t);
            for (i, voltages) in result.voltages.iter_mut().enumerate() {
                voltages.push(solution.get(i).copied().unwrap_or(0.0));
            }

            let scale = lte_estimator.recommend_scale(lte);
            timestep.adjust(lte / scale);
        }

        if t < tstop {
            log::error!(
                "Transient terminated early at t={:.6e}s / {:.6e}s after {} iterations",
                t,
                tstop,
                total_iterations
            );
            return Err(SimulationError::ConvergenceFailed(total_iterations));
        }

        log::info!(
            "Transient complete: {} time points computed",
            result.time.len()
        );

        // Debug: verify stored voltage range for node 0 (SIN source)
        if let Some(node0_voltages) = result.voltages.get(0) {
            let v_min = node0_voltages.iter().cloned().fold(f64::INFINITY, f64::min);
            let v_max = node0_voltages
                .iter()
                .cloned()
                .fold(f64::NEG_INFINITY, f64::max);
            log::info!(
                "Stored voltages for node0 (SIN source): {} points, y_min={:.4}, y_max={:.4}",
                node0_voltages.len(),
                v_min,
                v_max
            );
        }

        Ok(result)
    }

    /// Run transient analysis with waveform compression
    ///
    /// Uses the `WaveformRecorder` to achieve 10-100x memory reduction for long
    /// simulations. The compression uses linear interpolation-based point decimation
    /// that preserves all significant signal transitions.
    pub fn run_tran_compressed(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        compression: CompressionConfig,
    ) -> Result<TransientResultCompressed, SimulationError> {
        // Reuse the robust transient solver path, then apply waveform compression
        // during result marshaling. This keeps compressed and uncompressed physics
        // behavior identical, avoiding divergence between solver implementations.
        let result = self.run_tran(netlist, tstop, max_step)?;

        if result.time.is_empty() {
            return Ok(TransientResultCompressed {
                time: Vec::new(),
                voltages: vec![Vec::new(); result.num_nodes],
                num_nodes: result.num_nodes,
                compression_ratio: 1.0,
                input_points: 0,
            });
        }

        let initial_values: Vec<Value> = result
            .voltages
            .iter()
            .map(|wave| wave.first().copied().unwrap_or(0.0))
            .collect();
        let mut recorder = WaveformRecorder::new(
            result.num_nodes,
            result.time[0],
            &initial_values,
            compression,
        );

        for point_idx in 1..result.time.len() {
            let values: Vec<Value> = result
                .voltages
                .iter()
                .map(|wave| wave.get(point_idx).copied().unwrap_or(0.0))
                .collect();
            recorder.record(result.time[point_idx], &values);
        }

        let final_values: Vec<Value> = result
            .voltages
            .iter()
            .map(|wave| wave.last().copied().unwrap_or(0.0))
            .collect();
        recorder.finalize(*result.time.last().unwrap_or(&tstop), &final_values);

        Ok(recorder.to_transient_result())
    }
}

#[cfg(test)]
mod abort_tests {
    use super::*;
    use crate::Engine;
    use crate::abort_signal::{CountingAbort, ImmediateAbort, NoAbort};

    fn simple_rc_netlist() -> Netlist {
        // Simple RC circuit: V1 1 0 1V, R1 1 2 1k, C1 2 0 1u
        Netlist::parse(
            "Simple RC Circuit\n\
             V1 1 0 DC 1\n\
             R1 1 2 1k\n\
             C1 2 0 1u\n\
             .end",
        )
        .expect("Failed to parse netlist")
    }

    fn simple_bjt_amp_netlist() -> Netlist {
        Netlist::parse(
            "Simple BJT amplifier regression\n\
             V1 net3 0 SIN(0 2 1k 0 0 0)\n\
             CC1 net3 net5 1u\n\
             RR1 net5 0 1k\n\
             RR2 net4 net5 1k\n\
             RR3 net1 0 2.2k\n\
             RR4 net7 net1 1k\n\
             CC2 net7 0 10u\n\
             RR5 net4 net6 1k\n\
             V2 net4 0 DC 9\n\
             Q2 net6 net5 net1 npn_Q2\n\
             .MODEL npn_Q2 NPN (BF=100 IS=1e-15)\n\
             .end",
        )
        .expect("Failed to parse BJT amp netlist")
    }

    #[test]
    fn test_transient_no_abort_completes() {
        let engine = Engine::default();
        let netlist = simple_rc_netlist();

        // Should complete successfully with NoAbort
        let result = engine.run_tran_with_abort(&netlist, 1e-3, 1e-5, &NoAbort);
        assert!(result.is_ok(), "Transient should complete without abort");

        let result = result.unwrap();
        assert!(result.time.len() > 1, "Should have multiple time points");
        assert!(result.time.last().copied().unwrap_or(0.0) >= 1e-3 * 0.99);
    }

    #[test]
    fn test_transient_immediate_abort_returns_error() {
        let engine = Engine::default();
        let netlist = simple_rc_netlist();

        // ImmediateAbort should cause immediate termination
        let result = engine.run_tran_with_abort(&netlist, 1e-3, 1e-5, &ImmediateAbort);

        assert!(result.is_err(), "Should return error when aborted");
        match result {
            Err(SimulationError::Aborted) => {
                // Expected - simulation was aborted
            }
            Err(other) => panic!("Expected Aborted error, got: {:?}", other),
            Ok(_) => panic!("Expected error, got success"),
        }
    }

    #[test]
    fn test_transient_counting_abort_stops_at_threshold() {
        let engine = Engine::default();
        let netlist = simple_rc_netlist();

        // CountingAbort(5) will return true after 5 checks
        // Since we check every 1000 iterations, this tests the abort path
        let abort = CountingAbort::new(5);
        let result = engine.run_tran_with_abort(&netlist, 10e-3, 1e-7, &abort);

        // With a long simulation and small timestep, we should hit abort
        if abort.count() >= 5 {
            // If we checked 5 times, we should have aborted
            assert!(result.is_err(), "Should abort after threshold checks");
        } else {
            // If simulation finished before 5 checks, that's OK
            assert!(result.is_ok(), "Simulation finished before abort threshold");
        }
    }

    #[test]
    fn test_run_tran_delegates_to_run_tran_with_abort() {
        let engine = Engine::default();
        let netlist = simple_rc_netlist();

        // Regular run_tran should work the same as run_tran_with_abort(&NoAbort)
        let result1 = engine.run_tran(&netlist, 1e-4, 1e-6);
        let result2 = engine.run_tran_with_abort(&netlist, 1e-4, 1e-6, &NoAbort);

        assert!(result1.is_ok() && result2.is_ok());
        let r1 = result1.unwrap();
        let r2 = result2.unwrap();

        // Results should be identical
        assert_eq!(r1.time.len(), r2.time.len());
        assert_eq!(r1.num_nodes, r2.num_nodes);
    }

    #[test]
    fn test_abort_signal_checked_periodically() {
        // Verify that abort is not checked every single iteration
        // by using a counting abort and verifying the check count
        let engine = Engine::default();
        let netlist = simple_rc_netlist();

        // Use a very high threshold so we don't actually abort
        let abort = CountingAbort::new(1_000_000);
        let _result = engine.run_tran_with_abort(&netlist, 1e-4, 1e-6, &abort);

        // The number of checks should be much less than total iterations
        // (we check every 1000 iterations)
        let check_count = abort.count();
        assert!(
            check_count > 0,
            "Should have checked abort at least once: {}",
            check_count
        );
        // For a short simulation, we shouldn't have too many checks
        assert!(
            check_count < 1000,
            "Check count {} seems too high for short simulation",
            check_count
        );
    }

    #[test]
    fn test_is_stale_step_false_when_source_static() {
        let prev = vec![1.0, 2.0, 3.0];
        let next = vec![1.0, 2.0, 3.0];
        assert!(!Engine::is_stale_step(&prev, &next, 0.0, prev.len()));
    }

    #[test]
    fn test_is_stale_step_true_when_solution_does_not_follow_source() {
        let prev = vec![0.5, 1.0, -2.0];
        let next = prev.clone();
        assert!(Engine::is_stale_step(&prev, &next, 1e-3, prev.len()));
    }

    #[test]
    fn test_is_stale_step_false_when_solution_moves_with_source() {
        let prev = vec![0.5, 1.0, -2.0];
        let next = vec![0.5002, 1.0001, -1.9999];
        assert!(!Engine::is_stale_step(&prev, &next, 1e-3, prev.len()));
    }

    #[test]
    fn test_is_unbounded_step_detects_runaway() {
        let prev = vec![1.0, 2.0];
        let next = vec![500.0, -400.0];
        assert!(Engine::is_unbounded_step(&prev, &next, 1e-4, prev.len()));
    }

    #[test]
    fn test_is_unbounded_step_false_for_reasonable_change() {
        let prev = vec![1.0, 2.0];
        let next = vec![1.05, 2.08];
        assert!(!Engine::is_unbounded_step(&prev, &next, 1e-3, prev.len()));
    }

    #[test]
    fn test_is_clipped_force_candidate_detects_any_clipping() {
        let prev = vec![0.0; 6];
        let next = vec![0.5, -0.5, 0.5, -0.5, 0.0, 0.0];
        assert!(Engine::is_clipped_force_candidate(&prev, &next, 6));
    }

    #[test]
    fn test_is_clipped_force_candidate_false_when_below_clip_threshold() {
        let prev = vec![0.0; 6];
        let next = vec![0.01, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert!(!Engine::is_clipped_force_candidate(&prev, &next, 6));
    }

    #[test]
    fn test_bjt_amp_retains_tail_dynamics_after_stall_region() {
        let engine = Engine::default();
        let netlist = simple_bjt_amp_netlist();

        let result = engine
            .run_tran(&netlist, 100e-6, 20e-9)
            .expect("BJT amplifier transient should complete");

        let tail_start_idx = result
            .time
            .iter()
            .position(|&t| t >= 70e-6)
            .expect("tail start must exist");

        let mut dynamic_nodes = 0usize;
        for node in 0..result.num_nodes {
            let tail = &result.voltages[node][tail_start_idx..];
            let mut v_min = Value::INFINITY;
            let mut v_max = Value::NEG_INFINITY;
            for &v in tail {
                v_min = v_min.min(v);
                v_max = v_max.max(v);
            }
            if v_max - v_min > 1e-3 {
                dynamic_nodes += 1;
            }
        }

        assert!(
            dynamic_nodes >= 3,
            "Expected multiple dynamic traces in tail, got {}",
            dynamic_nodes
        );
    }

    #[test]
    fn test_bjt_amp_no_catastrophic_step_jumps_or_negative_runaway() {
        let engine = Engine::default();
        let netlist = simple_bjt_amp_netlist();

        let result = engine
            .run_tran(&netlist, 100e-6, 20e-9)
            .expect("BJT amplifier transient should complete");

        let global_min = result
            .voltages
            .iter()
            .flat_map(|v| v.iter().copied())
            .fold(Value::INFINITY, Value::min);
        assert!(
            global_min > -100.0,
            "Unexpected catastrophic negative runaway detected: min={}",
            global_min
        );

        for node in 1..result.num_nodes {
            let max_step = result.voltages[node]
                .windows(2)
                .map(|w| (w[1] - w[0]).abs())
                .fold(0.0, Value::max);
            assert!(
                max_step < 0.25,
                "Node {} has nonphysical force-accept jump: max_step={}",
                node,
                max_step
            );
        }
    }

    #[test]
    fn test_bjt_amp_bias_nodes_do_not_go_unphysically_negative() {
        let engine = Engine::default();
        let netlist = simple_bjt_amp_netlist();

        let result = engine
            .run_tran(&netlist, 100e-6, 20e-9)
            .expect("BJT amplifier transient should complete");

        // Nodes 1..5 are biased by resistive paths to supply/ground in this topology.
        // They should not run away to deep negative voltages during startup.
        for node in 1..result.num_nodes {
            let v_min = result.voltages[node]
                .iter()
                .copied()
                .fold(Value::INFINITY, Value::min);
            assert!(
                v_min > -0.5,
                "Node {} dropped to nonphysical negative voltage: {}",
                node,
                v_min
            );
        }
    }

    #[test]
    fn test_bjt_amp_output_peak_and_crossing_match_expected_envelope() {
        let engine = Engine::default();
        let netlist = simple_bjt_amp_netlist();

        // Long enough to capture positive peak and downward crossing against net7.
        let result = engine
            .run_tran(&netlist, 600e-6, 20e-9)
            .expect("BJT amplifier transient should complete");

        let net1 = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("net1"))
            .expect("net1 not found");
        let net7 = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("net7"))
            .expect("net7 not found");

        let out = &result.voltages[net1];
        let bypass = &result.voltages[net7];
        assert!(!out.is_empty() && out.len() == bypass.len());

        // Startup bias should be around the expected emitter DC level (definitely not 0V).
        assert!(
            out[0] > 3.0 && out[0] < 4.5,
            "Unexpected net1 startup bias: {}",
            out[0]
        );

        let (peak_idx, peak_v) = out
            .iter()
            .copied()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .expect("peak should exist");
        let peak_t = result.time[peak_idx];
        assert!(
            peak_v > 5.0 && peak_v < 5.6,
            "net1 peak out of expected range: {}",
            peak_v
        );
        assert!(
            peak_t > 150e-6 && peak_t < 300e-6,
            "net1 peak time out of expected range: {}",
            peak_t
        );

        // After peak, net1 should cross below the bypass node in the falling half-cycle.
        let crossing_t = (peak_idx + 1..out.len())
            .find(|&i| out[i] <= bypass[i])
            .map(|i| result.time[i])
            .expect("expected net1/net7 crossing after peak");
        assert!(
            crossing_t > 300e-6 && crossing_t < 520e-6,
            "net1/net7 crossing time out of expected range: {}",
            crossing_t
        );
    }
}
