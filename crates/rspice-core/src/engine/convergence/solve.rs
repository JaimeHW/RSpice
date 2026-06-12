//! Linear, nonlinear, and transient operating-point solve entry points.

use super::*;

impl Engine {
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
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        self.stamp_dc_direct(circuit, matrix, &mut rhs, gmin_floor);

        let direct_result = matrix.solve(&rhs);
        if let Ok(sol) = direct_result {
            return Ok(sol);
        }

        let mut last_err = direct_result.expect_err("checked Err branch");
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

    /// Solve nonlinear DC with optional node-voltage hint overrides.
    ///
    /// Performs a linear pre-solve to get a warm-start initial guess, which
    /// helps convergence especially for BJT circuits where starting from 0V
    /// puts the transistor in an unphysical state. `node_hints` entries are
    /// `(node_id, voltage)` with node IDs using the standard 1-based
    /// non-ground circuit numbering.
    pub(crate) fn solve_nonlinear_with_node_hints_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        node_hints: &[(usize, Value)],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let mut initial_guess = self
            .linear_presolve_for_guess(circuit, matrix)
            .unwrap_or_else(|| vec![0.0; size]);

        for &(node_id, voltage) in node_hints {
            if !voltage.is_finite() || node_id == 0 || node_id > circuit.num_nodes() {
                continue;
            }
            initial_guess[node_id - 1] = voltage;
        }

        self.solve_nonlinear_with_guess_and_abort(circuit, matrix, Some(&initial_guess), abort)
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
    pub(crate) fn solve_nonlinear_with_guess_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: Option<&[Value]>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        // Sanitize any warm-start seed before Newton so pathological presolve
        // artifacts do not launch the iteration from physically impossible rails.
        let mut solution = match initial_guess {
            Some(guess) => Self::sanitize_initial_guess(guess, size, circuit.num_nodes().min(size)),
            None => {
                let mut guess = vec![0.0; size];
                Self::apply_bjt_initial_guess_correction(&mut guess, circuit);
                guess
            }
        };
        let startup_seed = solution.clone();
        self.prime_operating_point_seed(circuit, &solution, 0.0, crate::xspice::AnalysisType::DcOp);
        let mut rhs = vec![0.0; size];
        // Newton-Raphson iteration
        let mut hit_voltage_limit = false;
        let mut limited_nodes: Vec<usize> = Vec::new();
        let mut damping_state = NewtonDampingState::default();
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        let requires_conservative_nonlinear_limiting =
            circuit.requires_conservative_solution_damping();
        // ngspice's flat Newton: when junction devices replace their own
        // iterate voltages (pnjlim), the full node step IS the algorithm and
        // merit-based step shrinking livelocks turn-on (the raw residual
        // transiently rises along the convergent direction). The +/-1kV node
        // containment below stays active regardless.
        let junction_owns_steps = Self::junction_limiting_owns_newton_steps(circuit);
        // Use 10x more iterations for DC nonlinear since damping limits voltage change per step
        // With MAX_DELTA_V=2V and standard max_iterations=50, we can only move 100V
        // Need 500+ iterations to traverse the full +/-1000V range if starting from a poor guess
        let dc_max_iterations = self.nonlinear_iteration_budget(10);
        let mut direct_iterations = 0usize;
        let mut residual_stall_iterations = 0usize;
        let mut residual_stalled = false;
        for iteration in 0..dc_max_iterations {
            direct_iterations = iteration + 1;
            if Self::should_abort_iteration(abort, iteration) {
                return Err(SimulationError::Aborted);
            }
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
            let node_count = circuit.num_nodes().min(size);
            for i in 0..node_count {
                matrix.add(i, i, gmin_floor);
            }
            // Stamp linear devices
            circuit.stamp_dc_direct(matrix, &mut rhs);
            // Update nonlinear/behavioral/XSPICE devices with current solution and stamp
            self.stamp_nonlinear_devices_for_dc(circuit, matrix, &mut rhs, &solution);
            // Solve linearized system
            let raw_solution = matrix.solve(&rhs).map_err(SimulationError::Solver)?;
            // Voltage-limiting style damping is critical for strongly-coupled
            // semiconductor nonlinearities, but it can unnecessarily throttle
            // behavioral-only fixed-point updates (e.g., B-source macros that
            // legitimately require kilovolt-level solution jumps).
            let mut new_solution = if requires_conservative_nonlinear_limiting
                && !junction_owns_steps
            {
                self.apply_damping_strategy(&solution, &raw_solution, &mut damping_state, |trial| {
                    self.nonlinear_merit(circuit, matrix, trial)
                })
            } else {
                raw_solution
            };
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
                } else if i < node_count
                    && requires_conservative_nonlinear_limiting
                    && v.abs() > Self::MAX_NODE_VOLTAGE
                {
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
                self.node_voltage_convergence_met(&solution, &new_solution, node_count);
            let linearized_residual_converged =
                self.residual_convergence_met(matrix, &new_solution, &rhs);
            // Device convergence must be checked at the candidate iterate, not the prior iterate.
            self.update_device_states_for_dc(circuit, &new_solution);
            let device_converged = circuit.nonlinear_converged(self.device_convergence_criteria());
            if std::env::var("RSPICE_DC_TRACE").as_deref() == Ok("1") {
                let max_dv = solution
                    .iter()
                    .zip(new_solution.iter())
                    .map(|(a, b)| (a - b).abs())
                    .fold(0.0_f64, f64::max);
                let limited = circuit
                    .bjts
                    .devices
                    .iter()
                    .filter(|bjt| bjt.legacy_junction_limited_for_trace())
                    .count();
                let nl_res = self.nonlinear_merit(circuit, matrix, &new_solution);
                eprintln!(
                    "DCTRACE iter={iteration} max_dv={max_dv:.3e} vconv={voltage_converged} dconv={device_converged} linres={linearized_residual_converged} limited_bjts={limited} merit={nl_res:?}"
                );
            }
            let nonlinear_residual_converged = voltage_converged
                && device_converged
                && self.nonlinear_residual_converged(circuit, matrix, &new_solution);
            solution = new_solution;
            if voltage_converged && device_converged && nonlinear_residual_converged {
                if hit_voltage_limit {
                    log::info!(
                        "DC operating point converged after {} iterations (voltage limiting was triggered)",
                        iteration + 1
                    );
                }
                if let Some(refined) =
                    self.refine_fallback_candidate(circuit, matrix, &solution, abort)
                {
                    return Ok(refined);
                }
                return Ok(solution);
            }

            if voltage_converged
                && device_converged
                && !(linearized_residual_converged || nonlinear_residual_converged)
            {
                residual_stall_iterations += 1;
                if residual_stall_iterations >= Self::DC_RESIDUAL_STALL_LIMIT {
                    residual_stalled = true;
                    break;
                }
            } else {
                residual_stall_iterations = 0;
            }
        }
        // Log diagnostic information when falling back to convergence aids.
        if hit_voltage_limit {
            log::warn!(
                "DC Newton-Raphson did not converge after {} iterations. \
                Voltage limiting triggered on {} node(s). Trying configured convergence aids...",
                direct_iterations.max(1),
                limited_nodes.len()
            );
        } else if residual_stalled {
            log::info!(
                "DC Newton-Raphson residual checks stalled after {} iterations. Trying configured convergence aids...",
                direct_iterations.max(1)
            );
        } else {
            log::info!(
                "DC Newton-Raphson did not converge after {} iterations. Trying configured convergence aids...",
                dc_max_iterations
            );
        }

        if residual_stalled
            && let Some(refined) = self.refine_fallback_candidate(circuit, matrix, &solution, abort)
        {
            log::info!(
                "Residual-stalled DC Newton candidate accepted after static-device polishing."
            );
            return Ok(refined);
        }

        let conv_cfg = &self.config.convergence_config;
        let allow_source = conv_cfg.source_stepping;
        let allow_pseudo = conv_cfg.pseudo_transient;
        let allow_gmin = conv_cfg.gmin_stepping;
        let allow_arc = conv_cfg.arc_length;
        if !allow_source && !allow_pseudo && !allow_gmin && !allow_arc {
            return Err(SimulationError::ConvergenceFailed(dc_max_iterations));
        }

        if let Some(legacy_seed) = self.legacy_hfet_inverse_branch_seed(circuit, &startup_seed) {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if let Some(restarted) =
                self.warm_restart_after_fallback(circuit, matrix, &legacy_seed, abort)
            {
                log::info!(
                    "Legacy HFET inverse-branch restart accepted after direct Newton failed."
                );
                return Ok(restarted);
            }
        }

        let zero_seed = vec![0.0; solution.len()];
        let mut fallback_seed =
            self.prefer_lower_merit_scaled_seed(circuit, matrix, &solution, &zero_seed, 1.0);
        let prefer_gate_generation_aids = circuit.has_jfet_gate_generation_branches();
        let mut gmin_attempted = false;

        if prefer_gate_generation_aids && allow_gmin {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            gmin_attempted = true;
            match self.gmin_stepping_nonlinear_with_abort(circuit, matrix, &fallback_seed, abort) {
                Ok(gmin_solution) => {
                    if let Some(candidate) = self.evaluate_fallback_candidate(
                        circuit,
                        matrix,
                        gmin_solution.clone(),
                        "GMIN stepping",
                        abort,
                    ) {
                        return Ok(candidate);
                    }
                    fallback_seed = self.prefer_lower_merit_scaled_seed(
                        circuit,
                        matrix,
                        &fallback_seed,
                        &gmin_solution,
                        1.0,
                    );
                    if let Some(restarted) =
                        self.warm_restart_after_fallback(circuit, matrix, &fallback_seed, abort)
                    {
                        log::info!(
                            "GMIN stepping warmed the nonlinear state; direct Newton restart accepted."
                        );
                        return Ok(restarted);
                    }
                }
                Err(e) => {
                    log::warn!(
                        "Early GMIN stepping for gate generation branch failed with {}. Continuing with configured aids.",
                        e
                    );
                }
            }
        }

        if allow_source {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            match self.source_stepping_nonlinear_with_guess_and_abort(
                circuit,
                matrix,
                &fallback_seed,
                abort,
            ) {
                Ok(source_stepped) => {
                    log::info!(
                        "DC operating point after source stepping ({} nodes): {:?}",
                        source_stepped.len(),
                        source_stepped.iter().take(10).collect::<Vec<_>>()
                    );
                    if let Some(candidate) = self.evaluate_fallback_candidate(
                        circuit,
                        matrix,
                        source_stepped.clone(),
                        "Source stepping",
                        abort,
                    ) {
                        return Ok(candidate);
                    }
                    fallback_seed = self.prefer_lower_merit_scaled_seed(
                        circuit,
                        matrix,
                        &fallback_seed,
                        &source_stepped,
                        1.0,
                    );
                    if let Some(restarted) =
                        self.warm_restart_after_fallback(circuit, matrix, &fallback_seed, abort)
                    {
                        log::info!(
                            "Source stepping warmed the nonlinear state; direct Newton restart accepted."
                        );
                        return Ok(restarted);
                    }
                }
                Err(e) => {
                    if !allow_pseudo && !allow_gmin && !allow_arc {
                        return Err(e);
                    }
                    log::warn!(
                        "Source stepping failed with {}. Escalating to next configured aid.",
                        e
                    );
                }
            }
        }

        if allow_pseudo {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            match self.pseudo_transient_nonlinear_with_guess_and_abort(
                circuit,
                matrix,
                &fallback_seed,
                abort,
            ) {
                Ok(pseudo_solution) => {
                    log::info!(
                        "DC operating point after pseudo-transient continuation ({} nodes): {:?}",
                        pseudo_solution.len(),
                        pseudo_solution.iter().take(10).collect::<Vec<_>>()
                    );
                    if let Some(candidate) = self.evaluate_fallback_candidate(
                        circuit,
                        matrix,
                        pseudo_solution.clone(),
                        "Pseudo-transient continuation",
                        abort,
                    ) {
                        return Ok(candidate);
                    }
                    fallback_seed = self.prefer_lower_merit_scaled_seed(
                        circuit,
                        matrix,
                        &fallback_seed,
                        &pseudo_solution,
                        1.0,
                    );
                    if let Some(restarted) =
                        self.warm_restart_after_fallback(circuit, matrix, &fallback_seed, abort)
                    {
                        log::info!(
                            "Pseudo-transient continuation warmed the nonlinear state; direct Newton restart accepted."
                        );
                        return Ok(restarted);
                    }
                }
                Err(e) => {
                    if !allow_gmin && !allow_arc {
                        return Err(e);
                    }
                    log::warn!(
                        "Pseudo-transient continuation failed with {}. Escalating to next configured aid.",
                        e
                    );
                }
            }
        }

        if allow_gmin && !gmin_attempted {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            match self.gmin_stepping_nonlinear_with_abort(circuit, matrix, &fallback_seed, abort) {
                Ok(gmin_solution) => {
                    if let Some(candidate) = self.evaluate_fallback_candidate(
                        circuit,
                        matrix,
                        gmin_solution.clone(),
                        "GMIN stepping",
                        abort,
                    ) {
                        return Ok(candidate);
                    }
                    fallback_seed = self.prefer_lower_merit_scaled_seed(
                        circuit,
                        matrix,
                        &fallback_seed,
                        &gmin_solution,
                        1.0,
                    );
                    if let Some(restarted) =
                        self.warm_restart_after_fallback(circuit, matrix, &fallback_seed, abort)
                    {
                        log::info!(
                            "GMIN stepping warmed the nonlinear state; direct Newton restart accepted."
                        );
                        return Ok(restarted);
                    }
                }
                Err(e) => {
                    if !allow_arc {
                        return Err(e);
                    }
                    log::warn!(
                        "GMIN stepping failed with {}. Escalating to arc-length continuation.",
                        e
                    );
                }
            }
        }

        if circuit.has_jfet_gate_generation_branches() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            match self.gate_generation_stepping_nonlinear_with_abort(
                circuit,
                matrix,
                &fallback_seed,
                abort,
            ) {
                Ok(gate_solution) => {
                    log::info!(
                        "Gate generation continuation produced a DC operating-point candidate."
                    );
                    if let Some(candidate) = self.evaluate_fallback_candidate(
                        circuit,
                        matrix,
                        gate_solution.clone(),
                        "Gate generation continuation",
                        abort,
                    ) {
                        return Ok(candidate);
                    }
                    fallback_seed = self.prefer_lower_merit_scaled_seed(
                        circuit,
                        matrix,
                        &fallback_seed,
                        &gate_solution,
                        1.0,
                    );
                    if let Some(restarted) =
                        self.warm_restart_after_fallback(circuit, matrix, &fallback_seed, abort)
                    {
                        log::info!(
                            "Gate generation continuation warmed the nonlinear state; direct Newton restart accepted."
                        );
                        return Ok(restarted);
                    }
                }
                Err(e) => {
                    if !allow_arc {
                        return Err(e);
                    }
                    log::warn!(
                        "Gate generation continuation failed with {}. Escalating to arc-length continuation.",
                        e
                    );
                }
            }
        }

        if allow_arc {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let arc_solution = self.arc_length_continuation_nonlinear_with_guess_and_abort(
                circuit,
                matrix,
                &fallback_seed,
                abort,
            )?;
            if let Some(candidate) = self.evaluate_fallback_candidate(
                circuit,
                matrix,
                arc_solution.clone(),
                "Arc-length continuation",
                abort,
            ) {
                return Ok(candidate);
            }
            if let Some(restarted) =
                self.warm_restart_after_fallback(circuit, matrix, &arc_solution, abort)
            {
                log::info!(
                    "Arc-length continuation warmed the nonlinear state; direct Newton restart accepted."
                );
                return Ok(restarted);
            }
        }
        Err(SimulationError::ConvergenceFailed(dc_max_iterations))
    }

    pub(crate) fn solve_linear_transient_operating_point_with_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        time: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }

        let size = circuit.matrix_size();
        matrix.clear_values();
        let mut rhs = vec![0.0; size];
        Self::stamp_transient_operating_point_linear(
            circuit,
            matrix,
            &mut rhs,
            time,
            self.config.convergence_config.gmin_target.max(0.0),
        );
        matrix.solve(&rhs).map_err(SimulationError::Solver)
    }

    pub(crate) fn solve_nonlinear_transient_op_with_node_hints_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        time: Value,
        node_hints: &[(usize, Value)],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        let junction_gmin = self.effective_device_junction_gmin(gmin_floor);
        let mut solution = self
            .linear_presolve_for_guess_with_linear_stamp(circuit, matrix, |circuit, matrix, rhs| {
                Self::stamp_transient_operating_point_linear(circuit, matrix, rhs, time, 0.0);
            })
            .unwrap_or_else(|| vec![0.0; size]);

        for &(node_id, voltage) in node_hints {
            if !voltage.is_finite() || node_id == 0 || node_id > circuit.num_nodes() {
                continue;
            }
            solution[node_id - 1] = voltage;
        }

        solution = Self::sanitize_initial_guess(&solution, size, circuit.num_nodes().min(size));
        self.prime_operating_point_seed(
            circuit,
            &solution,
            time,
            crate::xspice::AnalysisType::Transient,
        );

        let requires_conservative_nonlinear_limiting =
            circuit.requires_conservative_solution_damping();
        let mut rhs = vec![0.0; size];
        let mut damping_state = NewtonDampingState::default();
        let junction_owns_steps = Self::junction_limiting_owns_newton_steps(circuit);
        // ngspice floors every NIiter call to 100 iterations (ITL1); the
        // per-iterate junction walk of pnjlim devices legitimately needs
        // tens of iterations on deep TTL chains before the residual settles.
        let tranop_max_iterations = if junction_owns_steps {
            self.continuation_iteration_budget(1, 100)
        } else {
            self.continuation_iteration_budget(1, 32)
        };

        for iteration in 0..tranop_max_iterations {
            if Self::should_abort_iteration(abort, iteration) {
                return Err(SimulationError::Aborted);
            }

            matrix.clear_values();
            rhs.fill(0.0);

            circuit.refresh_jiles_atherton_inductances(&solution);
            Self::stamp_transient_operating_point_linear(
                circuit, matrix, &mut rhs, time, gmin_floor,
            );
            self.stamp_nonlinear_devices_for_operating_point(
                circuit,
                matrix,
                &mut rhs,
                &solution,
                time,
                crate::xspice::AnalysisType::Transient,
                junction_gmin,
            );

            let raw_solution = matrix.solve(&rhs).map_err(SimulationError::Solver)?;
            let mut new_solution = if requires_conservative_nonlinear_limiting
                && !junction_owns_steps
            {
                self.apply_damping_strategy(&solution, &raw_solution, &mut damping_state, |trial| {
                    self.nonlinear_merit_with_linear_stamp(
                        circuit,
                        matrix,
                        trial,
                        |circuit, matrix, rhs| {
                            circuit.refresh_jiles_atherton_inductances(trial);
                            Self::stamp_transient_operating_point_linear(
                                circuit, matrix, rhs, time, gmin_floor,
                            );
                        },
                    )
                })
            } else {
                raw_solution
            };
            Self::clamp_solution_to_physical_bounds(
                &mut new_solution,
                circuit.num_nodes().min(size),
            );

            let voltage_converged =
                self.node_voltage_convergence_met(&solution, &new_solution, circuit.num_nodes());
            self.update_device_states_for_operating_point(
                circuit,
                &new_solution,
                time,
                crate::xspice::AnalysisType::Transient,
                junction_gmin,
            );
            let device_converged = circuit.nonlinear_converged(self.device_convergence_criteria());
            let nonlinear_residual_converged = voltage_converged
                && device_converged
                && self.nonlinear_residual_converged_with_linear_stamp(
                    circuit,
                    matrix,
                    &new_solution,
                    |circuit, matrix, rhs| {
                        circuit.refresh_jiles_atherton_inductances(&new_solution);
                        Self::stamp_transient_operating_point_linear(
                            circuit, matrix, rhs, time, gmin_floor,
                        );
                    },
                );

            solution = new_solution;
            if voltage_converged && device_converged && nonlinear_residual_converged {
                return Ok(solution);
            }
        }

        Err(SimulationError::ConvergenceFailed(tranop_max_iterations))
    }
}
