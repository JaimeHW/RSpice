//! GMIN stepping, source stepping, pseudo-transient, and arc-length continuation.

use super::*;

impl Engine {
    pub(in crate::engine::convergence) fn build_descending_schedule(
        mut start: Value,
        mut end: Value,
    ) -> Vec<Value> {
        if !start.is_finite() || start <= 0.0 {
            start = 1e-3;
        }
        if !end.is_finite() || end < 0.0 {
            end = 0.0;
        }
        if start < end {
            std::mem::swap(&mut start, &mut end);
        }

        let mut values = Vec::with_capacity(16);
        let mut current = start;
        values.push(current);

        while current > end && values.len() < 64 {
            let next = (current / 10.0).max(end);
            if (next - current).abs() < Value::EPSILON {
                break;
            }
            values.push(next);
            current = next;
        }

        if values.last().is_none_or(|&v| v > end) {
            values.push(end);
        }

        values
    }

    #[inline]
    pub(in crate::engine::convergence) fn gmin_linear_schedule(&self) -> Vec<Value> {
        let conv = &self.config.convergence_config;
        let start = conv.gmin_initial.max(1e-2);
        let end = conv.gmin_target.max(0.0);
        Self::build_descending_schedule(start, end)
    }

    #[inline]
    pub(in crate::engine) fn gmin_nonlinear_schedule(&self) -> Vec<Value> {
        let conv = &self.config.convergence_config;
        let start = conv.gmin_initial.max(1e-3);
        let end = conv.gmin_target.max(0.0);
        Self::build_descending_schedule(start, end)
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
        let gmin_values = self.gmin_linear_schedule();

        let mut solution = None;

        for (idx, &gmin) in gmin_values.iter().enumerate() {
            match self.try_solve_with_gmin(circuit, matrix, gmin) {
                Ok(sol) => {
                    solution = Some(sol);
                    // Continue to try smaller GMIN for better accuracy
                }
                Err(_) if solution.is_some() => {
                    // Can't solve with smaller GMIN, use the last successful one
                    break;
                }
                Err(e) if idx == gmin_values.len() - 1 => {
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
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);

        let size = circuit.matrix_size();
        let mut solution = vec![0.0; size]; // Start from zero

        for &scale in SOURCE_SCALES {
            let mut rhs = vec![0.0; size];

            matrix.clear_values();
            rhs.fill(0.0);

            self.stamp_dc_scaled(circuit, matrix, &mut rhs, gmin_floor, scale);

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

    /// Source stepping for nonlinear circuits.
    ///
    /// Source stepping ramps sources from 0% to 100%, which helps find
    /// operating points in difficult circuits with strong nonlinearities. The
    /// adaptive implementation only advances after a converged corrector solve
    /// at the target scale; failed trial states are rolled back and retried
    /// with a smaller source increment.
    pub(crate) fn source_stepping_nonlinear_with_guess_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        const INITIAL_SOURCE_STEP: Value = 0.02;
        const MAX_SOURCE_STEP: Value = 0.2;
        const MIN_SOURCE_STEP: Value = 1.0e-4;
        const SOURCE_STEP_GROWTH: Value = 1.35;
        const SOURCE_STEP_SHRINK: Value = 0.5;
        const SOURCE_SCALE_EPS: Value = 1.0e-12;
        const MAX_SOURCE_ATTEMPTS: usize = 512;

        let size = circuit.matrix_size();

        let zero_guess = vec![0.0; size];
        let mut solution =
            self.prefer_lower_merit_scaled_seed(circuit, matrix, initial_guess, &zero_guess, 0.0);
        let mut damping_state = NewtonDampingState::default();
        let source_iterations = self.continuation_iteration_budget(20, 16);
        let mut total_iterations = 0usize;
        let (bootstrap_solution, bootstrap_converged, bootstrap_iterations) = self
            .solve_scaled_nonlinear_corrector(
                circuit,
                matrix,
                0.0,
                &solution,
                &mut damping_state,
                source_iterations,
                abort,
            );
        total_iterations += bootstrap_iterations;
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !bootstrap_converged {
            return Err(SimulationError::ConvergenceFailed(
                total_iterations.max(source_iterations),
            ));
        }
        solution = bootstrap_solution;

        let mut accepted_scale = 0.0;
        let mut source_step = INITIAL_SOURCE_STEP;
        let mut attempts = 0usize;

        while accepted_scale < 1.0 - SOURCE_SCALE_EPS {
            if attempts >= MAX_SOURCE_ATTEMPTS {
                return Err(SimulationError::ConvergenceFailed(total_iterations.max(1)));
            }
            if Self::should_abort_iteration(abort, attempts) {
                return Err(SimulationError::Aborted);
            }

            let target_scale = (accepted_scale + source_step).min(1.0);
            if target_scale <= accepted_scale + SOURCE_SCALE_EPS {
                return Err(SimulationError::ConvergenceFailed(total_iterations.max(1)));
            }

            let accepted_state = circuit.nonlinear_state_snapshot();
            let mut trial_damping_state = damping_state;
            let (candidate, converged, used_iterations) = self.solve_scaled_nonlinear_corrector(
                circuit,
                matrix,
                target_scale,
                &solution,
                &mut trial_damping_state,
                source_iterations,
                abort,
            );
            total_iterations = total_iterations.saturating_add(used_iterations);

            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }

            if converged {
                solution = candidate;
                accepted_scale = target_scale;
                damping_state = trial_damping_state;
                source_step = (source_step * SOURCE_STEP_GROWTH).min(MAX_SOURCE_STEP);
            } else {
                circuit.restore_nonlinear_state(accepted_state);
                source_step *= SOURCE_STEP_SHRINK;
                if source_step < MIN_SOURCE_STEP {
                    return Err(SimulationError::ConvergenceFailed(total_iterations.max(1)));
                }
            }

            attempts += 1;
        }

        let (polished_solution, converged, polish_iterations) = self
            .solve_scaled_nonlinear_corrector(
                circuit,
                matrix,
                1.0,
                &solution,
                &mut damping_state,
                source_iterations,
                abort,
            );
        total_iterations = total_iterations.saturating_add(polish_iterations);
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !converged {
            return Err(SimulationError::ConvergenceFailed(total_iterations.max(1)));
        }

        Ok(polished_solution)
    }

    /// Pseudo-transient continuation for difficult nonlinear circuits.
    ///
    /// Uses pseudo-capacitor anchoring (`Gpseudo * (x - x_prev)`) and grows the
    /// pseudo timestep as the solution stabilizes. This is a robust fallback for
    /// hard DC operating points where direct Newton/source/GMIN struggle.
    pub(crate) fn pseudo_transient_nonlinear_with_guess_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let mut solution = Self::normalize_initial_guess(initial_guess, size);
        let node_count = circuit.num_nodes().min(size);
        if Self::is_suspicious_solution(&solution, node_count) {
            solution.fill(0.0);
        }
        let mut anchor_solution = solution.clone();
        Self::clamp_solution_to_physical_bounds(&mut solution, node_count);
        Self::clamp_solution_to_physical_bounds(&mut anchor_solution, node_count);

        let mut pseudo = PseudoTransient::new();
        let mut damping_state = NewtonDampingState::default();
        let pseudo_iterations = self.continuation_iteration_budget(12, 16);

        let mut stage = 0usize;
        while !pseudo.is_complete() {
            if Self::should_abort_iteration(abort, stage) {
                return Err(SimulationError::Aborted);
            }
            let pseudo_conductance = pseudo.conductance(0);
            let mut stage_converged = false;

            for iteration in 0..pseudo_iterations {
                if Self::should_abort_iteration(abort, iteration) {
                    return Err(SimulationError::Aborted);
                }
                let mut rhs = vec![0.0; size];
                matrix.clear_values();

                for i in 0..size {
                    matrix.add(i, i, 1e-12 + pseudo_conductance);
                    rhs[i] += pseudo.current(anchor_solution[i]);
                }

                circuit.stamp_dc_direct(matrix, &mut rhs);
                self.stamp_nonlinear_devices_for_dc(circuit, matrix, &mut rhs, &solution);

                let raw_solution = match matrix.solve(&rhs) {
                    Ok(sol) => sol,
                    Err(_) => break,
                };

                    let mut new_solution = self.apply_damping_strategy_with_junction_ownership(
                        &solution,
                        &raw_solution,
                        &mut damping_state,
                        Self::junction_limiting_owns_newton_steps(circuit)
                            || self.b3soi_limiter_owns_global_damping(circuit),
                        |trial| {
                            self.nonlinear_merit_with_pseudo_transient(
                                circuit,
                                matrix,
                            trial,
                            &anchor_solution,
                            pseudo_conductance,
                        )
                    },
                );
                Self::clamp_solution_to_physical_bounds(&mut new_solution, node_count);

                let converged = self.node_voltage_convergence_met(
                    &solution,
                    &new_solution,
                    circuit.num_nodes(),
                );
                self.update_device_states_for_dc(circuit, &new_solution);
                solution = new_solution;
                let device_converged =
                    circuit.nonlinear_converged(self.device_convergence_criteria());
                let nonlinear_residual_converged = converged
                    && device_converged
                    && self.nonlinear_residual_converged_with_pseudo_transient(
                        circuit,
                        matrix,
                        &solution,
                        &anchor_solution,
                        pseudo_conductance,
                    );

                if converged && device_converged && nonlinear_residual_converged {
                    stage_converged = true;
                    break;
                }
            }

            if stage_converged {
                anchor_solution = solution.clone();
                pseudo.advance_on_success();
            } else if !pseudo.reduce_on_failure() {
                return Err(SimulationError::ConvergenceFailed(pseudo_iterations));
            }
            stage += 1;
        }

        Ok(solution)
    }

    /// Arc-length continuation fallback for strongly nonlinear or multi-solution circuits.
    ///
    /// Uses predictor-corrector continuation on source scale (lambda: 0 -> 1) with
    /// adaptive arc-length control. This improves robustness near turning points.
    pub(crate) fn arc_length_continuation_nonlinear_with_guess_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let mut current_solution = Self::normalize_initial_guess(initial_guess, size);
        let node_count = circuit.num_nodes().min(size);
        if Self::is_suspicious_solution(&current_solution, node_count) {
            current_solution.fill(0.0);
        }
        Self::clamp_solution_to_physical_bounds(&mut current_solution, node_count);
        let arc_newton_iters = self.continuation_iteration_budget(8, 16);

        let mut arc_cfg = ArcLengthConfig {
            tolerance: self.config.tolerance,
            max_steps: Self::ARC_LENGTH_MAX_STEPS,
            max_newton_iters: arc_newton_iters,
            ..ArcLengthConfig::default()
        };
        // Keep arc steps conservative for DC fallback stability.
        arc_cfg.initial_step = arc_cfg.initial_step.min(0.15);
        arc_cfg.max_step = arc_cfg.max_step.min(0.25);

        let mut arc = ArcLengthContinuation::with_config(size, arc_cfg.clone());
        let mut damping_state = NewtonDampingState::default();
        // Bootstrap to a consistent lambda=0 operating point before continuation.
        let (bootstrap_solution, _, _) = self.solve_scaled_nonlinear_corrector(
            circuit,
            matrix,
            0.0,
            &current_solution,
            &mut damping_state,
            arc_newton_iters,
            abort,
        );
        current_solution = bootstrap_solution;
        arc.initialize(&current_solution);

        let mut arc_step = 0usize;
        while !arc.is_complete() && !arc.is_failed() {
            if Self::should_abort_iteration(abort, arc_step) {
                return Err(SimulationError::Aborted);
            }
            let (predicted_solution, target_lambda) = arc.predict(&current_solution);
            let (corrected_solution, converged, newton_iters) = self
                .solve_scaled_nonlinear_corrector(
                    circuit,
                    matrix,
                    target_lambda,
                    &predicted_solution,
                    &mut damping_state,
                    arc_cfg.max_newton_iters,
                    abort,
                );

            if converged {
                arc.accept_step(&corrected_solution, target_lambda, newton_iters);
                current_solution = corrected_solution;
            } else if !arc.reject_step() {
                break;
            }
            arc_step += 1;
        }

        if arc.is_complete() {
            Ok(current_solution)
        } else {
            log::warn!(
                "Arc-length continuation did not reach lambda=1. Falling back to monotonic source continuation."
            );
            self.source_stepping_nonlinear_with_guess_and_abort(
                circuit,
                matrix,
                &current_solution,
                abort,
            )
        }
    }

    /// Homotopy for stiff JFET-family gate generation-recombination branches.
    ///
    /// The final accepted point is always solved and validated with the full
    /// branch strength. Intermediate scales only provide a smoother path to the
    /// same operating-point equations.
    pub(crate) fn gate_generation_stepping_nonlinear_with_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        if !circuit.has_jfet_gate_generation_branches() {
            return Err(SimulationError::ConvergenceFailed(0));
        }

        const INITIAL_STEP: Value = 0.02;
        const MAX_STEP: Value = 0.2;
        const MIN_STEP: Value = 1.0e-4;
        const STEP_GROWTH: Value = 1.35;
        const STEP_SHRINK: Value = 0.5;
        const SCALE_EPS: Value = 1.0e-12;
        const MAX_ATTEMPTS: usize = 512;

        let size = circuit.matrix_size();
        let node_count = circuit.num_nodes().min(size);
        let mut solution = Self::sanitize_initial_guess(initial_guess, size, node_count);
        let mut damping_state = NewtonDampingState::default();
        let corrector_iterations = self.continuation_iteration_budget(8, 16);
        let mut total_iterations = 0usize;

        circuit.set_jfet_gate_generation_scale(0.0);
        let (relaxed_solution, relaxed_converged, relaxed_iterations) = self
            .solve_scaled_nonlinear_corrector_with_seed_mode(
                circuit,
                matrix,
                1.0,
                &solution,
                &mut damping_state,
                corrector_iterations,
                abort,
                super::fallback::CorrectorSeedMode::Limited,
            );
        total_iterations = total_iterations.saturating_add(relaxed_iterations);
        if abort.is_aborted() {
            circuit.set_jfet_gate_generation_scale(1.0);
            return Err(SimulationError::Aborted);
        }
        if !relaxed_converged {
            circuit.set_jfet_gate_generation_scale(1.0);
            return Err(SimulationError::ConvergenceFailed(total_iterations.max(1)));
        }
        solution = relaxed_solution;

        let mut accepted_scale = 0.0;
        let mut scale_step = INITIAL_STEP;
        let mut attempts = 0usize;

        while accepted_scale < 1.0 - SCALE_EPS {
            if attempts >= MAX_ATTEMPTS {
                circuit.set_jfet_gate_generation_scale(1.0);
                return Err(SimulationError::ConvergenceFailed(total_iterations.max(1)));
            }
            if Self::should_abort_iteration(abort, attempts) {
                circuit.set_jfet_gate_generation_scale(1.0);
                return Err(SimulationError::Aborted);
            }

            let target_scale = (accepted_scale + scale_step).min(1.0);
            if target_scale <= accepted_scale + SCALE_EPS {
                circuit.set_jfet_gate_generation_scale(1.0);
                return Err(SimulationError::ConvergenceFailed(total_iterations.max(1)));
            }

            let accepted_state = circuit.nonlinear_state_snapshot();
            let mut trial_damping_state = damping_state;
            circuit.set_jfet_gate_generation_scale(target_scale);
            let (candidate, converged, used_iterations) = self
                .solve_scaled_nonlinear_corrector_with_seed_mode(
                    circuit,
                    matrix,
                    1.0,
                    &solution,
                    &mut trial_damping_state,
                    corrector_iterations,
                    abort,
                    super::fallback::CorrectorSeedMode::Limited,
                );
            total_iterations = total_iterations.saturating_add(used_iterations);

            if abort.is_aborted() {
                circuit.set_jfet_gate_generation_scale(1.0);
                return Err(SimulationError::Aborted);
            }

            if converged {
                solution = candidate;
                accepted_scale = target_scale;
                damping_state = trial_damping_state;
                scale_step = (scale_step * STEP_GROWTH).min(MAX_STEP);
            } else {
                circuit.restore_nonlinear_state(accepted_state);
                scale_step *= STEP_SHRINK;
                if scale_step < MIN_STEP {
                    circuit.set_jfet_gate_generation_scale(1.0);
                    return Err(SimulationError::ConvergenceFailed(total_iterations.max(1)));
                }
            }

            attempts += 1;
        }

        circuit.set_jfet_gate_generation_scale(1.0);
        let (polished, converged, polish_iterations) = self
            .solve_scaled_nonlinear_corrector_with_seed_mode(
                circuit,
                matrix,
                1.0,
                &solution,
                &mut damping_state,
                corrector_iterations,
                abort,
                super::fallback::CorrectorSeedMode::Limited,
            );
        total_iterations = total_iterations.saturating_add(polish_iterations);
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !converged {
            return Err(SimulationError::ConvergenceFailed(total_iterations.max(1)));
        }

        Ok(polished)
    }

    /// GMIN stepping for very difficult nonlinear circuits
    ///
    /// GMIN stepping starts with a large GMIN (1e-3) added to each node,
    /// then progressively reduces it to the normal value (1e-12).
    /// This helps BJT and other semiconductor devices find their operating
    /// point by initially providing a strong DC path that's gradually removed.
    /// This is a technique used in commercial SPICE simulators like Spectre/HSPICE.
    pub(crate) fn gmin_stepping_nonlinear_with_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let gmin_scales = self.gmin_nonlinear_schedule();

        let size = circuit.matrix_size();
        let node_count = circuit.num_nodes().min(size);

        // Check for suspicious values - not just clamped at ±999V but also
        // suspiciously uniform values that indicate failed source stepping.
        // Reset to zero if the guess looks like garbage.
        let node_guess_len = node_count.min(initial_guess.len());
        let is_garbage = Self::has_suspicious_uniformity(&initial_guess[..node_guess_len]);

        let mut solution: Vec<Value> = if is_garbage {
            log::debug!("GMIN stepping: resetting garbage initial guess to zero");
            vec![0.0; size]
        } else {
            initial_guess
                .iter()
                .enumerate()
                .map(|(idx, &v)| {
                    if idx < node_count && v.abs() >= 999.0 {
                        0.0
                    } else {
                        v
                    }
                })
                .collect()
        };
        self.prime_operating_point_seed(circuit, &solution, 0.0, crate::xspice::AnalysisType::DcOp);
        let mut damping_state = NewtonDampingState::default();
        let gmin_iterations = self.continuation_iteration_budget(10, 12);

        for (step, &gmin) in gmin_scales.iter().enumerate() {
            if Self::should_abort_iteration(abort, step) {
                return Err(SimulationError::Aborted);
            }
            let junction_gmin = self.effective_device_junction_gmin(gmin);
            log::debug!("GMIN stepping: step {} with GMIN = {:.2e}", step + 1, gmin);

            // Use more iterations for GMIN stepping to allow convergence
            for iteration in 0..gmin_iterations {
                if Self::should_abort_iteration(abort, iteration) {
                    return Err(SimulationError::Aborted);
                }
                let mut rhs = vec![0.0; size];

                matrix.clear_values();

                // Add current GMIN only to node-voltage equations.
                let node_count = circuit.num_nodes().min(size);
                for i in 0..node_count {
                    matrix.add(i, i, gmin);
                }

                // Stamp linear and nonlinear devices
                circuit.stamp_dc_direct(matrix, &mut rhs);
                self.stamp_nonlinear_devices_for_dc_with_junction_gmin(
                    circuit,
                    matrix,
                    &mut rhs,
                    &solution,
                    junction_gmin,
                );

                // Log RHS on first iteration of first GMIN step for debugging
                if step == 0 && iteration == 0 {
                    log::debug!(
                        "GMIN step 1 iter 1 - RHS: {:?}",
                        rhs.iter().map(|v| format!("{:.2e}", v)).collect::<Vec<_>>()
                    );
                }

                match matrix.solve(&rhs) {
                    Ok(raw_solution) => {
                        let mut new_solution = self.apply_damping_strategy_with_junction_ownership(
                            &solution,
                            &raw_solution,
                            &mut damping_state,
                            Self::junction_limiting_owns_newton_steps(circuit)
                                || self.b3soi_limiter_owns_global_damping(circuit),
                            |trial| self.nonlinear_merit_with_gmin(circuit, matrix, trial, gmin),
                        );
                        Self::clamp_solution_to_physical_bounds(&mut new_solution, node_count);

                        let converged = self.node_voltage_convergence_met(
                            &solution,
                            &new_solution,
                            circuit.num_nodes(),
                        );
                        self.update_device_states_for_dc_with_junction_gmin(
                            circuit,
                            &new_solution,
                            junction_gmin,
                        );
                        solution = new_solution;
                        let device_converged =
                            circuit.nonlinear_converged(self.device_convergence_criteria());
                        let nonlinear_residual_converged = converged
                            && device_converged
                            && self.nonlinear_residual_converged_with_gmin(
                                circuit, matrix, &solution, gmin,
                            );
                        if converged && device_converged && nonlinear_residual_converged {
                            break;
                        }
                    }
                    Err(e) if step == gmin_scales.len() - 1 => {
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
        let has_clamped = Self::has_clamped_values(&solution, node_count);

        // Check for suspicious uniformity (same issue as source stepping)
        let final_node_count = node_count.min(solution.len());
        let has_suspicious_uniformity =
            Self::has_suspicious_uniformity(&solution[..final_node_count]);

        if has_clamped {
            log::warn!(
                "GMIN stepping completed but solution still contains clamped values. \
                Circuit may need additional biasing or convergence aids."
            );
        } else if has_suspicious_uniformity {
            let unique_values: std::collections::HashSet<i32> =
                solution.iter().map(|v| (v * 100.0) as i32).collect();
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
