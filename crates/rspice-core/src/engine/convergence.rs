//! Convergence helpers for Newton-Raphson iteration
//!
//! This module provides:
//! - GMIN stepping for difficult circuits
//! - Source stepping for convergence
//! - Linear and nonlinear solver interfaces

use super::{DampingStrategy, Engine, SimulationError};
use crate::solver::{SolverError, StaticMatrix};
use crate::{CircuitData, Value};

#[derive(Debug, Clone, Copy)]
struct NewtonDampingState {
    bank_rose_alpha: Value,
    prev_step_norm: Option<Value>,
}

impl Default for NewtonDampingState {
    fn default() -> Self {
        Self {
            bank_rose_alpha: 1.0,
            prev_step_norm: None,
        }
    }
}

impl Engine {
    const MAX_NODE_VOLTAGE: Value = 1000.0;
    const MAX_DELTA_VOLTAGE_LIMIT: Value = 0.5;
    const BANK_ROSE_ALPHA_MIN: Value = 0.1;
    const BANK_ROSE_ALPHA_MAX: Value = 1.0;
    const ARMIJO_C1: Value = 1e-4;
    const LINE_SEARCH_BACKTRACK: Value = 0.5;
    const LINE_SEARCH_MAX_ITERS: usize = 8;

    #[inline]
    fn nonlinear_iteration_budget(&self, multiplier: usize) -> usize {
        self.config.max_iterations.saturating_mul(multiplier).max(1)
    }

    #[inline]
    fn has_clamped_values(solution: &[Value]) -> bool {
        solution.iter().any(|&v| !v.is_finite() || v.abs() >= 999.0)
    }

    #[inline]
    fn has_suspicious_uniformity(solution: &[Value]) -> bool {
        let unique_values: std::collections::HashSet<i32> =
            solution.iter().map(|v| (v * 100.0) as i32).collect();
        unique_values.len() <= 4 && solution.len() > 4
    }

    #[inline]
    fn is_suspicious_solution(solution: &[Value]) -> bool {
        Self::has_clamped_values(solution) || Self::has_suspicious_uniformity(solution)
    }

    #[inline]
    fn step_l2_norm(old: &[Value], new: &[Value]) -> Value {
        old.iter()
            .zip(new.iter())
            .map(|(&a, &b)| {
                let d = b - a;
                d * d
            })
            .sum::<Value>()
            .sqrt()
    }

    fn interpolate_solution(old: &[Value], proposal: &[Value], alpha: Value) -> Vec<Value> {
        old.iter()
            .zip(proposal.iter())
            .map(|(&old_v, &new_v)| old_v + alpha * (new_v - old_v))
            .collect()
    }

    fn limit_step_delta(old: &[Value], proposal: &[Value], max_delta: Value) -> Vec<Value> {
        old.iter()
            .zip(proposal.iter())
            .map(|(&old_v, &new_v)| {
                let delta = new_v - old_v;
                if delta.abs() > max_delta {
                    old_v + delta.signum() * max_delta
                } else {
                    new_v
                }
            })
            .collect()
    }

    fn update_bank_rose_alpha(damping_state: &mut NewtonDampingState, step_norm: Value) {
        let Some(prev_norm) = damping_state.prev_step_norm else {
            damping_state.prev_step_norm = Some(step_norm.max(1e-30));
            damping_state.bank_rose_alpha = 1.0;
            return;
        };

        let ratio = if prev_norm > 0.0 {
            step_norm / prev_norm
        } else {
            1.0
        };

        if ratio > 1.0 {
            damping_state.bank_rose_alpha *= 0.5;
        } else if ratio > 0.9 {
            damping_state.bank_rose_alpha *= 0.9;
        } else if ratio < 0.5 {
            damping_state.bank_rose_alpha *= 1.2;
        }

        damping_state.bank_rose_alpha = damping_state
            .bank_rose_alpha
            .clamp(Self::BANK_ROSE_ALPHA_MIN, Self::BANK_ROSE_ALPHA_MAX);
        damping_state.prev_step_norm = Some(step_norm.max(1e-30));
    }

    fn line_search_step<F>(old: &[Value], proposal: &[Value], merit: &mut F) -> Vec<Value>
    where
        F: FnMut(&[Value]) -> Option<Value>,
    {
        let base_merit = merit(old).unwrap_or(Value::INFINITY);
        let mut best_solution = proposal.to_vec();
        let mut best_merit = merit(proposal).unwrap_or(Value::INFINITY);

        if best_merit.is_finite() && (!base_merit.is_finite() || best_merit <= base_merit) {
            return best_solution;
        }

        let mut alpha = Self::LINE_SEARCH_BACKTRACK;
        for _ in 0..Self::LINE_SEARCH_MAX_ITERS {
            let trial = Self::interpolate_solution(old, proposal, alpha);
            if let Some(trial_merit) = merit(&trial) {
                if trial_merit < best_merit {
                    best_merit = trial_merit;
                    best_solution = trial.clone();
                }

                let armijo_ok = !base_merit.is_finite()
                    || trial_merit <= base_merit * (1.0 - Self::ARMIJO_C1 * alpha);
                if armijo_ok {
                    return trial;
                }
            }
            alpha *= Self::LINE_SEARCH_BACKTRACK;
        }

        best_solution
    }

    fn apply_damping_strategy<F>(
        &self,
        old: &[Value],
        proposal: &[Value],
        damping_state: &mut NewtonDampingState,
        mut merit: F,
    ) -> Vec<Value>
    where
        F: FnMut(&[Value]) -> Option<Value>,
    {
        match self.config.convergence_config.damping_strategy {
            DampingStrategy::None => proposal.to_vec(),
            DampingStrategy::LineSearch => Self::line_search_step(old, proposal, &mut merit),
            DampingStrategy::VoltageLimiting => {
                Self::limit_step_delta(old, proposal, Self::MAX_DELTA_VOLTAGE_LIMIT)
            }
            DampingStrategy::BankRose => {
                let step_norm = Self::step_l2_norm(old, proposal);
                Self::update_bank_rose_alpha(damping_state, step_norm);
                Self::interpolate_solution(old, proposal, damping_state.bank_rose_alpha)
            }
            DampingStrategy::Combined => {
                let limited = Self::limit_step_delta(old, proposal, Self::MAX_DELTA_VOLTAGE_LIMIT);
                let step_norm = Self::step_l2_norm(old, &limited);
                Self::update_bank_rose_alpha(damping_state, step_norm);
                let bank_rose_step =
                    Self::interpolate_solution(old, &limited, damping_state.bank_rose_alpha);
                Self::line_search_step(old, &bank_rose_step, &mut merit)
            }
        }
    }

    fn clamp_solution_to_physical_bounds(solution: &mut [Value]) {
        for v in solution.iter_mut() {
            if !v.is_finite() {
                *v = 0.0;
            } else if v.abs() > Self::MAX_NODE_VOLTAGE {
                *v = v.signum() * Self::MAX_NODE_VOLTAGE;
            }
        }
    }

    fn nonlinear_merit_with_linear_stamp<F>(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        mut linear_stamp: F,
    ) -> Option<Value>
    where
        F: FnMut(&mut CircuitData, &mut StaticMatrix, &mut [Value]),
    {
        let size = circuit.matrix_size();
        if solution.len() != size || solution.iter().any(|v| !v.is_finite()) {
            return None;
        }

        let mut rhs = vec![0.0; size];
        matrix.clear_values();
        linear_stamp(circuit, matrix, &mut rhs);
        circuit.update_nonlinear(solution);
        circuit.stamp_nonlinear(matrix, &mut rhs, solution);

        let next_solution = matrix.solve(&rhs).ok()?;
        Some(Self::step_l2_norm(solution, &next_solution))
    }

    fn nonlinear_merit(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
    ) -> Option<Value> {
        self.nonlinear_merit_with_linear_stamp(circuit, matrix, solution, |circuit, matrix, rhs| {
            for i in 0..rhs.len() {
                matrix.add(i, i, 1e-12);
            }
            circuit.stamp_dc_direct(matrix, rhs);
        })
    }

    fn nonlinear_merit_scaled(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        source_scale: Value,
    ) -> Option<Value> {
        self.nonlinear_merit_with_linear_stamp(circuit, matrix, solution, |circuit, matrix, rhs| {
            for i in 0..rhs.len() {
                matrix.add(i, i, 1e-12);
            }
            circuit.stamp_dc_direct_scaled(matrix, rhs, source_scale);
        })
    }

    fn nonlinear_merit_with_gmin(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        gmin: Value,
    ) -> Option<Value> {
        self.nonlinear_merit_with_linear_stamp(circuit, matrix, solution, |circuit, matrix, rhs| {
            for i in 0..rhs.len() {
                matrix.add(i, i, gmin);
            }
            circuit.stamp_dc_direct(matrix, rhs);
        })
    }

    fn validate_nonlinear_solution(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
    ) -> bool {
        let size = circuit.matrix_size();
        if solution.len() != size || solution.iter().any(|v| !v.is_finite()) {
            return false;
        }

        // Re-evaluate one Newton linearization at the candidate point and require
        // a small fixed-point update plus device-level convergence.
        let mut rhs = vec![0.0; size];
        matrix.clear_values();
        for i in 0..size {
            matrix.add(i, i, 1e-12);
        }
        circuit.stamp_dc_direct(matrix, &mut rhs);
        circuit.update_nonlinear(solution);
        circuit.stamp_nonlinear(matrix, &mut rhs, solution);

        let Ok(next_solution) = matrix.solve(&rhs) else {
            return false;
        };

        Self::check_voltage_convergence(solution, &next_solution, self.config.tolerance)
            && circuit.nonlinear_converged(self.config.tolerance)
    }

    /// Try solving with a specific GMIN value
    pub(crate) fn try_solve_with_gmin(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        gmin: Value,
    ) -> Result<Vec<Value>, SolverError> {
        let size = circuit.matrix_size();
        let mut rhs = vec![0.0; size];

        matrix.clear_values();
        rhs.fill(0.0);

        self.stamp_dc_direct(circuit, matrix, &mut rhs, gmin);
        matrix.solve(&rhs)
    }

    /// GMIN stepping: try progressively smaller GMIN values
    pub(crate) fn gmin_stepping(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SolverError> {
        // GMIN stepping sequence from large to small
        const GMIN_VALUES: &[Value] = &[1e-2, 1e-4, 1e-6, 1e-9, 1e-12];

        let mut solution = None;

        for &gmin in GMIN_VALUES {
            match self.try_solve_with_gmin(circuit, matrix, gmin) {
                Ok(sol) => {
                    solution = Some(sol);
                    // Continue to try smaller GMIN for better accuracy
                }
                Err(_) if solution.is_some() => {
                    // Can't solve with smaller GMIN, use the last successful one
                    break;
                }
                Err(e) if gmin == GMIN_VALUES[GMIN_VALUES.len() - 1] => {
                    // Last GMIN value failed and we have no solution
                    return Err(e);
                }
                Err(_) => {
                    // Try next GMIN value
                    continue;
                }
            }
        }

        solution.ok_or(SolverError::SingularMatrix)
    }

    /// Source stepping: ramp sources from 0 to 100%
    pub(crate) fn source_stepping(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SolverError> {
        // Source stepping sequence
        const SOURCE_SCALES: &[Value] = &[0.0, 0.1, 0.25, 0.5, 0.75, 1.0];
        const GMIN: Value = 1e-12;

        let size = circuit.matrix_size();
        let mut solution = vec![0.0; size]; // Start from zero

        for &scale in SOURCE_SCALES {
            let mut rhs = vec![0.0; size];

            matrix.clear_values();
            rhs.fill(0.0);

            self.stamp_dc_scaled(circuit, matrix, &mut rhs, GMIN, scale);

            match matrix.solve(&rhs) {
                Ok(sol) => {
                    solution = sol;
                }
                Err(e) if scale == 1.0 => {
                    return Err(e);
                }
                Err(_) => {
                    // Try to continue with the current solution
                    continue;
                }
            }
        }

        Ok(solution)
    }

    /// Solve a linear circuit (no nonlinear devices)
    pub(crate) fn solve_linear(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let mut rhs = vec![0.0; size];

        matrix.clear_values();
        rhs.fill(0.0);
        self.stamp_dc_direct(circuit, matrix, &mut rhs, 1e-12);

        let direct_result = matrix.solve(&rhs);
        if let Ok(sol) = direct_result {
            return Ok(sol);
        }

        let mut last_err = direct_result.err().expect("checked Err branch");
        let conv_cfg = &self.config.convergence_config;

        if conv_cfg.gmin_stepping {
            match self.gmin_stepping(circuit, matrix) {
                Ok(sol) => return Ok(sol),
                Err(e) => {
                    last_err = e;
                }
            }
        }

        if conv_cfg.source_stepping {
            return self
                .source_stepping(circuit, matrix)
                .map_err(SimulationError::Solver);
        }

        Err(SimulationError::Solver(last_err))
    }

    /// Solve a nonlinear circuit using Newton-Raphson iteration
    ///
    /// This performs a linear pre-solve to get a warm-start initial guess,
    /// which helps convergence especially for BJT circuits where starting
    /// from 0V puts the transistor in an unphysical state.
    pub(crate) fn solve_nonlinear(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SimulationError> {
        // First, try to get an initial guess from linear solve
        // This stamps only linear devices (resistors, voltage sources)
        // to get DC source voltages propagated through the resistor network
        let initial_guess = self.linear_presolve_for_guess(circuit, matrix);

        // Apply BJT-specific junction voltage correction
        // The linear presolve treats BJT terminals as disconnected, so
        // VBE could be huge (e.g., 9V). We need to set VBE ≈ 0.7V for
        // realistic forward active operation.
        let corrected_guess = initial_guess.map(|mut guess| {
            Self::apply_bjt_initial_guess_correction(&mut guess, circuit);
            guess
        });

        // Now run full nonlinear solve with the warm-start guess
        self.solve_nonlinear_with_guess(circuit, matrix, corrected_guess.as_deref())
    }

    /// Apply BJT-specific initial guess corrections
    ///
    /// The linear presolve doesn't include BJT connections, so the base
    /// and emitter may have unrealistic voltage differences. This function
    /// corrects the initial guess to place the BJT in forward-active region:
    /// - VBE ≈ 0.7V (typical forward bias)
    /// - VCE > VCE(sat) ≈ 0.2V (avoid saturation)
    fn apply_bjt_initial_guess_correction(guess: &mut [Value], circuit: &CircuitData) {
        const VBE_FORWARD: Value = 0.7; // Typical forward B-E voltage
        const VCE_SAT: Value = 0.2; // Saturation voltage

        for bjt in &circuit.bjts.devices {
            let collector_node = bjt.node_collector;
            let base_node = bjt.node_base;
            let emitter_node = bjt.node_emitter;

            if base_node == 0 || emitter_node == 0 {
                continue;
            }

            let vc = if collector_node > 0 {
                guess[collector_node - 1]
            } else {
                0.0
            };
            let vb = guess[base_node - 1];
            let ve = guess[emitter_node - 1];

            // Strategy: Start with emitter voltage from linear presolve (respects resistor network)
            // Adjust base to be VBE_FORWARD above emitter
            // Adjust collector to be above base for forward-active

            let is_npn = matches!(bjt.bjt_type, crate::device::BjtType::Npn);

            if is_npn {
                // NPN: Vc > Vb > Ve, VBE ≈ 0.7V, VCE > 0.2V
                // Keep emitter at linear presolve value (grounded through resistor)
                // Set base = emitter + 0.7V
                // Set collector to be above base (midpoint to VCC or similar)

                let ve_new = ve; // Keep emitter from linear presolve
                let vb_new = ve_new + VBE_FORWARD;
                let vc_new = (vb_new + vc.max(vb_new + VCE_SAT)) / 2.0; // Between base and original Vc
                let vc_new = vc_new.max(vb_new + VCE_SAT); // Ensure forward active

                if (vb - ve).abs() > 1.0 || vc < vb {
                    log::debug!(
                        "BJT {} (NPN): Correcting to forward active: Vc={:.2}->{:.2}, Vb={:.2}->{:.2}, Ve={:.2}->{:.2}",
                        bjt.name,
                        vc,
                        vc_new,
                        vb,
                        vb_new,
                        ve,
                        ve_new
                    );
                    guess[base_node - 1] = vb_new;
                    if collector_node > 0 {
                        guess[collector_node - 1] = vc_new;
                    }
                }
            } else {
                // PNP: Ve > Vb > Vc, VEB ≈ 0.7V, VEC > 0.2V
                let ve_new = ve;
                let vb_new = ve_new - VBE_FORWARD;
                let vc_new = vb_new - VCE_SAT;

                if (ve - vb).abs() > 1.0 || vc > vb {
                    log::debug!(
                        "BJT {} (PNP): Correcting to forward active: Vc={:.2}->{:.2}, Vb={:.2}->{:.2}, Ve={:.2}->{:.2}",
                        bjt.name,
                        vc,
                        vc_new,
                        vb,
                        vb_new,
                        ve,
                        ve_new
                    );
                    guess[base_node - 1] = vb_new;
                    if collector_node > 0 {
                        guess[collector_node - 1] = vc_new;
                    }
                }
            }
        }
    }

    /// Perform linear pre-solve to get initial voltage guess
    ///
    /// This solves the circuit with only linear devices (nonlinear devices
    /// replaced by very high resistances) to establish DC source voltages
    /// through the resistor network. This provides a much better starting
    /// point for Newton iteration than all zeros.
    fn linear_presolve_for_guess(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Option<Vec<Value>> {
        let size = circuit.matrix_size();
        let num_nodes = circuit.num_nodes();

        // Clear matrix and build linear-only stamp
        matrix.clear_values();
        let mut rhs = vec![0.0; size];

        // Stamp only linear devices
        circuit.stamp_dc_direct(matrix, &mut rhs);

        // Add small conductance to ground for each node to prevent floating nodes
        // This is especially important for BJT base/emitter nodes that would
        // otherwise be floating in the linear presolve
        for i in 0..num_nodes {
            if let Some(idx) = matrix.get_index(i, i) {
                matrix.stamp_direct(idx, 1e-9); // 1nS to ground
            }
        }

        // Try to solve
        match matrix.solve(&rhs) {
            Ok(solution) => {
                log::debug!("Linear presolve succeeded, using as initial guess");
                // Log initial guess voltages for debugging
                for (i, &v) in solution.iter().enumerate().take(num_nodes) {
                    log::debug!("  Presolve V({}) = {:.4} V", i + 1, v);
                }
                Some(solution)
            }
            Err(_) => {
                log::debug!("Linear presolve failed, starting from zero");
                None
            }
        }
    }

    /// Solve a nonlinear circuit using Newton-Raphson iteration with optional initial guess
    ///
    /// # Arguments
    /// * `circuit` - Circuit data with nonlinear devices
    /// * `matrix` - Sparse matrix structure for MNA
    /// * `initial_guess` - Optional initial solution vector (e.g., from previous DC sweep point)
    ///
    /// Using a good initial guess (like the previous sweep point solution) significantly
    /// improves convergence speed and robustness for nonlinear circuits.
    ///
    /// # Returns
    /// The converged solution vector, or error if Newton-Raphson fails to converge.
    pub(crate) fn solve_nonlinear_with_guess(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: Option<&[Value]>,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        // Use provided initial guess or start from zero
        let mut solution = match initial_guess {
            Some(guess) if guess.len() == size => guess.to_vec(),
            Some(guess) => {
                // Mismatched size - log warning and use zero
                // This can happen if circuit topology changed
                let mut sol = vec![0.0; size];
                // Copy as much as we can
                let copy_len = guess.len().min(size);
                sol[..copy_len].copy_from_slice(&guess[..copy_len]);
                sol
            }
            None => vec![0.0; size],
        };
        let mut rhs = vec![0.0; size];
        // Newton-Raphson iteration
        let mut hit_voltage_limit = false;
        let mut limited_nodes: Vec<usize> = Vec::new();
        let mut damping_state = NewtonDampingState::default();
        // Use 10x more iterations for DC nonlinear since damping limits voltage change per step
        // With MAX_DELTA_V=2V and standard max_iterations=50, we can only move 100V
        // Need 500+ iterations to traverse the full +/-1000V range if starting from a poor guess
        let dc_max_iterations = self.nonlinear_iteration_budget(10);
        for iteration in 0..dc_max_iterations {
            // Debug trace first few iterations
            if iteration < 5 {
                log::debug!(
                    "Newton iter {}: V = {:?}",
                    iteration,
                    solution
                        .iter()
                        .take(circuit.num_nodes())
                        .map(|v| format!("{:.2}", v))
                        .collect::<Vec<_>>()
                );
            }
            // Clear matrix and RHS for this iteration
            matrix.clear_values();
            rhs.fill(0.0);
            // Add GMIN to diagonal for numerical stability
            for i in 0..size {
                matrix.add(i, i, 1e-12);
            }
            // Stamp linear devices
            circuit.stamp_dc_direct(matrix, &mut rhs);
            // Update nonlinear devices with current solution and stamp
            circuit.update_nonlinear(&solution);
            circuit.stamp_nonlinear(matrix, &mut rhs, &solution);
            // Solve linearized system
            let raw_solution = matrix.solve(&rhs).map_err(SimulationError::Solver)?;
            let mut new_solution = self.apply_damping_strategy(
                &solution,
                &raw_solution,
                &mut damping_state,
                |trial| self.nonlinear_merit(circuit, matrix, trial),
            );
            // Solution limiting: prevent numerical blow-up by clamping extreme values
            // This is a critical convergence aid for circuits with strong nonlinearities
            for (i, v) in new_solution.iter_mut().enumerate() {
                if !v.is_finite() {
                    log::debug!(
                        "DC iter {}: NaN/Inf at node {}, resetting to 0",
                        iteration,
                        i + 1
                    );
                    *v = 0.0; // Replace NaN/Inf with zero
                } else if v.abs() > Self::MAX_NODE_VOLTAGE {
                    if !hit_voltage_limit {
                        hit_voltage_limit = true;
                        log::debug!(
                            "DC iter {}: Voltage limiting triggered - Newton-Raphson may struggle to converge",
                            iteration
                        );
                    }
                    if !limited_nodes.contains(&i) {
                        limited_nodes.push(i);
                        log::debug!(
                            "  Node {}: {:.2e}V -> clamped to {:.0}V",
                            i + 1,
                            *v,
                            v.signum() * Self::MAX_NODE_VOLTAGE
                        );
                    }
                    *v = v.signum() * Self::MAX_NODE_VOLTAGE;
                }
            }
            // Check convergence (both voltage change and device convergence)
            let voltage_converged =
                Self::check_voltage_convergence(&solution, &new_solution, self.config.tolerance);
            // Device convergence must be checked at the candidate iterate, not the prior iterate.
            circuit.update_nonlinear(&new_solution);
            let device_converged = circuit.nonlinear_converged(self.config.tolerance);
            solution = new_solution;
            if voltage_converged && device_converged {
                if hit_voltage_limit {
                    log::info!(
                        "DC operating point converged after {} iterations (voltage limiting was triggered)",
                        iteration + 1
                    );
                }
                return Ok(solution);
            }
        }
        // Log diagnostic information when falling back to source stepping
        if hit_voltage_limit {
            log::warn!(
                "DC Newton-Raphson did not converge after {} iterations. \
                Voltage limiting triggered on {} node(s). Trying source stepping...",
                dc_max_iterations,
                limited_nodes.len()
            );
        } else {
            log::info!(
                "DC Newton-Raphson did not converge after {} iterations. Trying source stepping...",
                dc_max_iterations
            );
        }
        let conv_cfg = &self.config.convergence_config;
        let allow_source = conv_cfg.source_stepping;
        let allow_gmin = conv_cfg.gmin_stepping;
        if !allow_source && !allow_gmin {
            return Err(SimulationError::ConvergenceFailed(dc_max_iterations));
        }
        let mut fallback_seed = solution.clone();
        if allow_source {
            match self.source_stepping_nonlinear_with_guess(circuit, matrix, &solution) {
                Ok(source_stepped) => {
                    log::info!(
                        "DC operating point after source stepping ({} nodes): {:?}",
                        source_stepped.len(),
                        source_stepped.iter().take(10).collect::<Vec<_>>()
                    );
                    let suspicious = Self::is_suspicious_solution(&source_stepped);
                    let validated = !suspicious
                        && self.validate_nonlinear_solution(circuit, matrix, &source_stepped);
                    if validated {
                        return Ok(source_stepped);
                    }
                    if suspicious {
                        if Self::has_clamped_values(&source_stepped) {
                            log::warn!(
                                "Source stepping produced clamped values; candidate rejected."
                            );
                        } else {
                            log::warn!(
                                "Source stepping produced suspiciously uniform values; candidate rejected."
                            );
                        }
                    } else {
                        log::warn!(
                            "Source stepping candidate failed convergence re-validation; candidate rejected."
                        );
                    }
                    fallback_seed = source_stepped;
                }
                Err(e) => {
                    if !allow_gmin {
                        return Err(e);
                    }
                    log::warn!(
                        "Source stepping failed with {}. Escalating to GMIN stepping.",
                        e
                    );
                }
            }
        }
        if allow_gmin {
            let gmin_solution = self.gmin_stepping_nonlinear(circuit, matrix, &fallback_seed)?;
            let suspicious = Self::is_suspicious_solution(&gmin_solution);
            let validated =
                !suspicious && self.validate_nonlinear_solution(circuit, matrix, &gmin_solution);
            if validated {
                return Ok(gmin_solution);
            }
            if suspicious {
                log::warn!("GMIN stepping candidate rejected due to suspicious/clamped values.");
            } else {
                log::warn!("GMIN stepping candidate failed convergence re-validation.");
            }
        }
        Err(SimulationError::ConvergenceFailed(dc_max_iterations))
    }

    /// Check if voltage solution has converged
    pub(crate) fn check_voltage_convergence(
        old: &[Value],
        new: &[Value],
        tolerance: Value,
    ) -> bool {
        if old.len() != new.len() {
            return false;
        }
        for (&v_old, &v_new) in old.iter().zip(new.iter()) {
            let abs_diff = (v_new - v_old).abs();
            let rel_diff = if v_new.abs() > tolerance {
                abs_diff / v_new.abs()
            } else {
                0.0
            };
            if abs_diff > tolerance && rel_diff > 1e-3 {
                return false;
            }
        }
        true
    }

    /// Source stepping for nonlinear circuits (starts from zero)
    #[allow(dead_code)]
    pub(crate) fn source_stepping_nonlinear(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let zero_guess = vec![0.0; size];
        self.source_stepping_nonlinear_with_guess(circuit, matrix, &zero_guess)
    }

    /// Source stepping for nonlinear circuits with initial guess
    ///
    /// # Arguments
    /// * `circuit` - Circuit data with nonlinear devices
    /// * `matrix` - Sparse matrix structure  
    /// * `initial_guess` - Starting solution (e.g., from failed Newton iteration or previous sweep point)
    ///
    /// Source stepping ramps sources from 0% to 100% in steps, which helps
    /// find operating points in difficult circuits with strong nonlinearities.
    /// Uses finer granularity (11 steps) for commercial-grade convergence handling.
    pub(crate) fn source_stepping_nonlinear_with_guess(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
    ) -> Result<Vec<Value>, SimulationError> {
        // Finer source stepping for difficult circuits (mirrors Spectre/HSPICE approach)
        // Smaller initial steps help with BJT/MOSFET turn-on regions
        const SOURCE_SCALES: &[Value] =
            &[0.0, 0.01, 0.05, 0.1, 0.2, 0.35, 0.5, 0.65, 0.8, 0.9, 1.0];

        let size = circuit.matrix_size();

        // Start from provided initial guess (scaled to first source level)
        let mut solution = if initial_guess.len() == size {
            initial_guess.to_vec()
        } else {
            vec![0.0; size]
        };
        let mut damping_state = NewtonDampingState::default();

        for &scale in SOURCE_SCALES {
            // Run Newton iterations at this source level
            // Use significantly more iterations for damped convergence (20x base)
            // With MAX_DELTA_V=2V damping, need 500+ iterations to traverse full range
            for _iteration in 0..self.nonlinear_iteration_budget(20) {
                let mut rhs = vec![0.0; size];

                matrix.clear_values();

                // Add GMIN
                for i in 0..size {
                    matrix.add(i, i, 1e-12);
                }

                // Stamp linear devices with scaled sources
                circuit.stamp_dc_direct_scaled(matrix, &mut rhs, scale);

                // Stamp nonlinear devices
                circuit.update_nonlinear(&solution);
                circuit.stamp_nonlinear(matrix, &mut rhs, &solution);

                match matrix.solve(&rhs) {
                    Ok(raw_solution) => {
                        let mut new_solution = self.apply_damping_strategy(
                            &solution,
                            &raw_solution,
                            &mut damping_state,
                            |trial| self.nonlinear_merit_scaled(circuit, matrix, trial, scale),
                        );
                        Self::clamp_solution_to_physical_bounds(&mut new_solution);

                        let converged = Self::check_voltage_convergence(
                            &solution,
                            &new_solution,
                            self.config.tolerance,
                        );
                        circuit.update_nonlinear(&new_solution);
                        solution = new_solution;
                        if converged && circuit.nonlinear_converged(self.config.tolerance) {
                            break;
                        }
                    }
                    Err(e) if scale == 1.0 => {
                        return Err(SimulationError::Solver(e));
                    }
                    Err(_) => {
                        break; // Try next scale
                    }
                }
            }
        }

        Ok(solution)
    }

    /// GMIN stepping for very difficult nonlinear circuits
    ///
    /// GMIN stepping starts with a large GMIN (1e-3) added to each node,
    /// then progressively reduces it to the normal value (1e-12).
    /// This helps BJT and other semiconductor devices find their operating
    /// point by initially providing a strong DC path that's gradually removed.
    /// This is a technique used in commercial SPICE simulators like Spectre/HSPICE.
    pub(crate) fn gmin_stepping_nonlinear(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
    ) -> Result<Vec<Value>, SimulationError> {
        // GMIN values from large to small (logarithmic progression)
        const GMIN_SCALES: &[Value] = &[
            1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 1e-8, 1e-9, 1e-10, 1e-11, 1e-12,
        ];

        let size = circuit.matrix_size();

        // Check for suspicious values - not just clamped at ±999V but also
        // suspiciously uniform values that indicate failed source stepping.
        // Reset to zero if the guess looks like garbage.
        let unique_values: std::collections::HashSet<i32> =
            initial_guess.iter().map(|v| (v * 100.0) as i32).collect();
        let is_garbage = unique_values.len() <= 4 && initial_guess.len() > 4;

        let mut solution: Vec<Value> = if is_garbage {
            log::debug!("GMIN stepping: resetting garbage initial guess to zero");
            vec![0.0; size]
        } else {
            initial_guess
                .iter()
                .map(|&v| if v.abs() >= 999.0 { 0.0 } else { v })
                .collect()
        };
        let mut damping_state = NewtonDampingState::default();

        for (step, &gmin) in GMIN_SCALES.iter().enumerate() {
            log::debug!("GMIN stepping: step {} with GMIN = {:.2e}", step + 1, gmin);

            // Use more iterations for GMIN stepping to allow convergence
            for _iteration in 0..self.nonlinear_iteration_budget(10) {
                let mut rhs = vec![0.0; size];

                matrix.clear_values();

                // Add current GMIN to diagonal
                for i in 0..size {
                    matrix.add(i, i, gmin);
                }

                // Stamp linear and nonlinear devices
                circuit.stamp_dc_direct(matrix, &mut rhs);
                circuit.update_nonlinear(&solution);
                circuit.stamp_nonlinear(matrix, &mut rhs, &solution);

                // Log RHS on first iteration of first GMIN step for debugging
                if step == 0 && _iteration == 0 {
                    log::debug!(
                        "GMIN step 1 iter 1 - RHS: {:?}",
                        rhs.iter().map(|v| format!("{:.2e}", v)).collect::<Vec<_>>()
                    );
                }

                match matrix.solve(&rhs) {
                    Ok(raw_solution) => {
                        let mut new_solution = self.apply_damping_strategy(
                            &solution,
                            &raw_solution,
                            &mut damping_state,
                            |trial| self.nonlinear_merit_with_gmin(circuit, matrix, trial, gmin),
                        );
                        Self::clamp_solution_to_physical_bounds(&mut new_solution);

                        let converged = Self::check_voltage_convergence(
                            &solution,
                            &new_solution,
                            self.config.tolerance,
                        );
                        circuit.update_nonlinear(&new_solution);
                        solution = new_solution;
                        if converged && circuit.nonlinear_converged(self.config.tolerance) {
                            break;
                        }
                    }
                    Err(e) if step == GMIN_SCALES.len() - 1 => {
                        log::error!("GMIN stepping failed at final step: {:?}", e);
                        return Err(SimulationError::Solver(e));
                    }
                    Err(_) => {
                        log::debug!(
                            "Matrix solve failed at GMIN step {}, continuing...",
                            step + 1
                        );
                        break; // Try next GMIN level
                    }
                }
            }
            // Log solution after each GMIN step for debugging
            log::debug!(
                "GMIN step {} (GMIN={:.0e}) solution: {:?}",
                step + 1,
                gmin,
                solution
                    .iter()
                    .take(8)
                    .map(|v| format!("{:.2}", v))
                    .collect::<Vec<_>>()
            );
        }

        // Log the actual GMIN stepping result for debugging
        log::info!(
            "DC operating point after GMIN stepping ({} nodes): {:?}",
            solution.len(),
            solution
                .iter()
                .take(10)
                .map(|v| format!("{:.2}", v))
                .collect::<Vec<_>>()
        );

        // Final check: detect both clamped values and suspicious uniformity
        let has_clamped = solution.iter().any(|&v| v.abs() >= 999.0);

        // Check for suspicious uniformity (same issue as source stepping)
        let unique_values: std::collections::HashSet<i32> =
            solution.iter().map(|v| (v * 100.0) as i32).collect();
        let has_suspicious_uniformity = unique_values.len() <= 4 && solution.len() > 4;

        if has_clamped {
            log::warn!(
                "GMIN stepping completed but solution still contains clamped values. \
                Circuit may need additional biasing or convergence aids."
            );
        } else if has_suspicious_uniformity {
            log::warn!(
                "GMIN stepping completed but solution has suspiciously uniform values ({} unique). \
                DC operating point may be incorrect.",
                unique_values.len()
            );
        } else {
            log::info!("GMIN stepping converged successfully");
        }

        Ok(solution)
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Test voltage convergence checking with identical vectors.
    #[test]
    fn test_voltage_convergence_identical() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test voltage convergence with small absolute differences.
    #[test]
    fn test_voltage_convergence_small_absolute() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0 + 1e-10, 2.0 + 1e-10, 3.0 + 1e-10];
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test voltage convergence fails with large differences.
    #[test]
    fn test_voltage_convergence_large_diff() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 4.0]; // 33% relative diff
        assert!(!Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test voltage convergence with near-zero values (special handling).
    #[test]
    fn test_voltage_convergence_near_zero() {
        let a = vec![1e-15, 0.0, 1e-14];
        let b = vec![0.0, 1e-15, 0.0];
        // Near-zero values should use absolute tolerance only
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test voltage convergence with mismatched lengths fails.
    #[test]
    fn test_voltage_convergence_mismatched_length() {
        let a = vec![1.0, 2.0];
        let b = vec![1.0, 2.0, 3.0];
        assert!(!Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test voltage convergence with empty vectors.
    #[test]
    fn test_voltage_convergence_empty() {
        let a: Vec<f64> = vec![];
        let b: Vec<f64> = vec![];
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test convergence at tolerance boundary.
    #[test]
    fn test_voltage_convergence_at_tolerance() {
        let tolerance: f64 = 1e-6;
        let a = vec![1.0];
        let b = vec![1.0 + tolerance * 0.9]; // Just under tolerance
        assert!(Engine::check_voltage_convergence(&a, &b, tolerance));
    }

    /// Test that relative tolerance kicks in for large values.
    #[test]
    fn test_voltage_convergence_relative_tolerance() {
        // For large values, relative tolerance of 1e-3 is used
        let a = vec![1000.0];
        let b = vec![1000.5]; // 0.05% difference, under 0.1%
        // 0.5 absolute diff > 1e-6 tolerance, but relative = 0.0005 < 1e-3
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-6));
    }

    /// Test voltage convergence with negative values.
    #[test]
    fn test_voltage_convergence_negative_values() {
        let a = vec![-5.0, -10.0];
        let b = vec![-5.0 + 1e-10, -10.0 - 1e-10];
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test convergence with mixed positive/negative.
    #[test]
    fn test_voltage_convergence_mixed_signs() {
        let a = vec![5.0, -5.0, 0.0];
        let b = vec![5.0, -5.0, 0.0];
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test that GMIN values are in decreasing order.
    #[test]
    fn test_gmin_stepping_values_order() {
        // The GMIN_VALUES constant should be in decreasing order
        // for proper stepping from large to small
        const GMIN_VALUES: &[f64] = &[1e-2, 1e-4, 1e-6, 1e-9, 1e-12];
        for i in 1..GMIN_VALUES.len() {
            assert!(
                GMIN_VALUES[i] < GMIN_VALUES[i - 1],
                "GMIN values should be decreasing"
            );
        }
    }

    /// Test that source stepping scales are in increasing order.
    #[test]
    fn test_source_stepping_values_order() {
        const SOURCE_SCALES: &[f64] = &[0.0, 0.1, 0.25, 0.5, 0.75, 1.0];
        for i in 1..SOURCE_SCALES.len() {
            assert!(
                SOURCE_SCALES[i] > SOURCE_SCALES[i - 1],
                "Source scales should be increasing"
            );
        }
        // Must start at 0 and end at 1
        assert!((SOURCE_SCALES[0] - 0.0).abs() < 1e-10);
        assert!((SOURCE_SCALES[SOURCE_SCALES.len() - 1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_nonlinear_iteration_budget_never_returns_zero() {
        let mut config = crate::engine::SimulationConfig::default();
        config.max_iterations = 0;
        let engine = Engine::new(config);

        assert_eq!(engine.nonlinear_iteration_budget(10), 1);
        assert_eq!(engine.nonlinear_iteration_budget(20), 1);
    }

    #[test]
    fn test_suspicious_solution_detection_flags_clamped_and_non_finite_values() {
        let clamped = vec![0.0, 1.0, 1000.0, -2.0];
        let non_finite = vec![0.0, f64::INFINITY, 1.0, 2.0];

        assert!(Engine::has_clamped_values(&clamped));
        assert!(Engine::has_clamped_values(&non_finite));
        assert!(Engine::is_suspicious_solution(&clamped));
        assert!(Engine::is_suspicious_solution(&non_finite));
    }

    #[test]
    fn test_suspicious_solution_detection_flags_uniform_stuck_vectors() {
        let stuck = vec![0.0, 0.0, 0.0, 0.01, 0.0, 0.0];
        let healthy = vec![0.0, 0.17, -0.32, 0.91, -1.4, 2.3];

        assert!(Engine::has_suspicious_uniformity(&stuck));
        assert!(!Engine::has_suspicious_uniformity(&healthy));
        assert!(Engine::is_suspicious_solution(&stuck));
        assert!(!Engine::is_suspicious_solution(&healthy));
    }

    #[test]
    fn test_limit_step_delta_clamps_large_updates() {
        let old = vec![0.0, -1.0, 0.25];
        let proposal = vec![2.0, -4.0, 0.50];
        let limited = Engine::limit_step_delta(&old, &proposal, 0.5);

        assert!((limited[0] - 0.5).abs() < 1e-12);
        assert!((limited[1] - (-1.5)).abs() < 1e-12);
        assert!((limited[2] - 0.50).abs() < 1e-12);
    }

    #[test]
    fn test_interpolate_solution_scales_newton_step() {
        let old = vec![1.0, -2.0];
        let proposal = vec![5.0, 2.0];
        let half_step = Engine::interpolate_solution(&old, &proposal, 0.5);
        let quarter_step = Engine::interpolate_solution(&old, &proposal, 0.25);

        assert!((half_step[0] - 3.0).abs() < 1e-12);
        assert!((half_step[1] - 0.0).abs() < 1e-12);
        assert!((quarter_step[0] - 2.0).abs() < 1e-12);
        assert!((quarter_step[1] - (-1.0)).abs() < 1e-12);
    }

    #[test]
    fn test_bank_rose_alpha_adapts_to_step_quality() {
        let mut state = NewtonDampingState::default();

        Engine::update_bank_rose_alpha(&mut state, 1.0);
        assert!((state.bank_rose_alpha - 1.0).abs() < 1e-12);

        // Larger step norm implies worse behavior -> alpha should shrink.
        Engine::update_bank_rose_alpha(&mut state, 2.0);
        assert!(state.bank_rose_alpha < 1.0);
        let reduced = state.bank_rose_alpha;

        // Improved step norm should recover alpha.
        Engine::update_bank_rose_alpha(&mut state, 0.5);
        assert!(state.bank_rose_alpha > reduced);
    }

    #[test]
    fn test_line_search_step_backtracks_to_lower_merit() {
        let old = vec![0.0];
        let proposal = vec![10.0];

        let mut merit = |x: &[Value]| Some((x[0] - 1.0).powi(2));
        let chosen = Engine::line_search_step(&old, &proposal, &mut merit);

        assert!(chosen[0] < proposal[0], "line search should backtrack");
        assert!((chosen[0] - 1.0).abs() < (proposal[0] - 1.0).abs());
    }

    #[test]
    fn test_apply_damping_strategy_none_preserves_proposal() {
        let mut config = crate::engine::SimulationConfig::default();
        config.convergence_config = crate::engine::ConvergenceConfig::fast()
            .with_damping(crate::engine::DampingStrategy::None);
        let engine = Engine::new(config);

        let old = vec![0.0, 0.0];
        let proposal = vec![2.0, -3.0];
        let mut state = NewtonDampingState::default();
        let damped = engine.apply_damping_strategy(&old, &proposal, &mut state, |_| Some(0.0));

        assert_eq!(damped, proposal);
    }

    #[test]
    fn test_apply_damping_strategy_voltage_limiting_clamps_delta() {
        let mut config = crate::engine::SimulationConfig::default();
        config.convergence_config = crate::engine::ConvergenceConfig::default()
            .with_damping(crate::engine::DampingStrategy::VoltageLimiting);
        let engine = Engine::new(config);

        let old = vec![0.0, 0.0];
        let proposal = vec![2.0, -3.0];
        let mut state = NewtonDampingState::default();
        let damped = engine.apply_damping_strategy(&old, &proposal, &mut state, |_| Some(0.0));

        assert!((damped[0] - Engine::MAX_DELTA_VOLTAGE_LIMIT).abs() < 1e-12);
        assert!((damped[1] + Engine::MAX_DELTA_VOLTAGE_LIMIT).abs() < 1e-12);
    }
}
