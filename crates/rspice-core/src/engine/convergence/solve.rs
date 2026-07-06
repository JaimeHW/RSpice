//! Linear, nonlinear, and transient operating-point solve entry points.

use super::*;
use crate::SpiceDialect;

/// Name the matrix rows behind a singular linear system so the user sees
/// which node or branch carries no constraining equation instead of a bare
/// "matrix is singular".
pub(crate) fn singular_system_message(circuit: &CircuitData, matrix: &StaticMatrix) -> String {
    let rows = matrix.deficient_rows();
    if rows.is_empty() {
        return "matrix is singular with no structurally empty rows; the usual causes are \
                loops of ideal voltage sources/inductors or duplicate constraints on one \
                node pair (run `rspice check` for a topology report)"
            .to_string();
    }
    let node_names = circuit.node_names_sorted();
    let branch_names = circuit.branch_names_sorted();
    let num_nodes = circuit.num_nodes();
    let mut names: Vec<String> = rows
        .iter()
        .take(8)
        .map(|&row| {
            if row < num_nodes {
                match node_names.get(row) {
                    Some(name) => format!("node '{name}'"),
                    None => format!("node #{}", row + 1),
                }
            } else {
                match branch_names.get(row - num_nodes) {
                    Some(name) => format!("branch '{name}'"),
                    None => format!("branch #{}", row - num_nodes + 1),
                }
            }
        })
        .collect();
    if rows.len() > 8 {
        names.push(format!("... {} more", rows.len() - 8));
    }
    format!(
        "matrix is singular: no equation constrains {} — the node(s) are floating or the \
         element wiring is inconsistent",
        names.join(", ")
    )
}

impl Engine {
    /// Solve a linear circuit (no nonlinear devices)
    pub(crate) fn solve_linear(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let mut rhs = vec![0.0; size];

        matrix.clear_values();
        rhs.fill(0.0);
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        self.stamp_dc_direct(circuit, matrix, &mut rhs, gmin_floor);
        if !circuit.behavioral_sources.is_empty()
            && !circuit.behavioral_sources.has_solution_dependent_sources()
        {
            let zero_solution = vec![0.0; size];
            circuit.stamp_behavioral_sources(matrix, &mut rhs, &zero_solution, 0.0);
        }

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
            return self.source_stepping(circuit, matrix).map_err(|err| {
                if matches!(err, crate::solver::SolverError::SingularMatrix) {
                    SimulationError::Circuit(singular_system_message(circuit, matrix))
                } else {
                    SimulationError::Solver(err)
                }
            });
        }

        if matches!(last_err, crate::solver::SolverError::SingularMatrix) {
            return Err(SimulationError::Circuit(singular_system_message(
                circuit, matrix,
            )));
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

        Self::apply_bjt_initial_guess_correction(&mut initial_guess, circuit);
        Self::apply_b3soi_pd_initial_guess_correction(&mut initial_guess, circuit);
        Self::apply_vbic_internal_initial_guess_correction(&mut initial_guess, circuit);
        for &(node_id, voltage) in node_hints {
            if !voltage.is_finite() || node_id == 0 || node_id > circuit.num_nodes() {
                continue;
            }
            initial_guess[node_id - 1] = voltage;
        }

        if !node_hints.is_empty() {
            match self.solve_nonlinear_nodeset_dc_startup_with_abort(
                circuit,
                matrix,
                &initial_guess,
                node_hints,
                abort,
            ) {
                Ok(nodeset_solution) => {
                    initial_guess = nodeset_solution;
                }
                Err(SimulationError::Aborted) => return Err(SimulationError::Aborted),
                Err(err) => {
                    log::debug!(
                        "NODESET-constrained DC startup did not converge: {err}; continuing from hinted seed."
                    );
                }
            }
        }

        self.solve_nonlinear_with_guess_and_abort(circuit, matrix, Some(&initial_guess), abort)
    }

    fn apply_node_voltage_constraints(
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        node_hints: &[(usize, Value)],
    ) -> Result<(), SimulationError> {
        for &(node_id, voltage) in node_hints {
            if !voltage.is_finite() || node_id == 0 || node_id > circuit.num_nodes() {
                continue;
            }
            let row = node_id - 1;
            if row >= rhs.len() {
                continue;
            }
            matrix
                .force_identity_row(row)
                .map_err(SimulationError::Solver)?;
            rhs[row] = voltage;
        }
        Ok(())
    }

    fn enforce_node_voltage_hints(
        circuit: &CircuitData,
        solution: &mut [Value],
        node_hints: &[(usize, Value)],
    ) {
        for &(node_id, voltage) in node_hints {
            if !voltage.is_finite() || node_id == 0 || node_id > circuit.num_nodes() {
                continue;
            }
            if let Some(slot) = solution.get_mut(node_id - 1) {
                *slot = voltage;
            }
        }
    }

    fn constrained_dc_residual_converged(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        node_hints: &[(usize, Value)],
    ) -> Result<bool, SimulationError> {
        let snapshot = circuit.nonlinear_state_snapshot();
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        let junction_gmin =
            self.effective_device_junction_gmin(self.config.convergence_config.gmin_target);
        let result = matrix.with_probe_values(|probe, rhs| -> Result<bool, SimulationError> {
            let node_count = circuit.num_nodes().min(rhs.len());
            for i in 0..node_count {
                probe.add(i, i, gmin_floor);
            }
            circuit.stamp_dc_direct(probe, rhs);
            self.try_stamp_static_probe_nonlinear_devices_for_dc_with_junction_gmin(
                circuit,
                probe,
                rhs,
                solution,
                junction_gmin,
            )?;
            Self::apply_node_voltage_constraints(circuit, probe, rhs, node_hints)?;
            Ok(self.residual_probe_fixed_point_converged(circuit, probe, solution, rhs))
        });
        circuit.restore_nonlinear_state(snapshot);
        result
    }

    fn constrained_transient_op_residual_converged(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        time: Value,
        node_hints: &[(usize, Value)],
    ) -> Result<bool, SimulationError> {
        let snapshot = circuit.nonlinear_state_snapshot();
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        let junction_gmin = self.effective_device_junction_gmin(gmin_floor);
        let result = matrix.with_probe_values(|probe, rhs| -> Result<bool, SimulationError> {
            circuit.refresh_jiles_atherton_inductances(solution);
            Self::stamp_transient_operating_point_linear(
                circuit, probe, rhs, time, gmin_floor, false,
            );
            self.try_stamp_nonlinear_devices_for_operating_point(
                circuit,
                probe,
                rhs,
                solution,
                time,
                crate::xspice::AnalysisType::Transient,
                junction_gmin,
            )?;
            Self::apply_node_voltage_constraints(circuit, probe, rhs, node_hints)?;
            Ok(self.residual_probe_fixed_point_converged(circuit, probe, solution, rhs))
        });
        circuit.restore_nonlinear_state(snapshot);
        result
    }

    fn solve_nonlinear_nodeset_dc_startup_with_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
        node_hints: &[(usize, Value)],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let node_count = circuit.num_nodes().min(size);
        let mut solution = Self::sanitize_initial_guess(initial_guess, size, node_count);
        Self::enforce_node_voltage_hints(circuit, &mut solution, node_hints);
        self.prime_operating_point_seed(circuit, &solution, 0.0, crate::xspice::AnalysisType::DcOp);

        let mut rhs = vec![0.0; size];
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        let max_iterations = self.continuation_iteration_budget(1, 64);

        for iteration in 0..max_iterations {
            if Self::should_abort_iteration(abort, iteration) {
                return Err(SimulationError::Aborted);
            }

            matrix.clear_values();
            rhs.fill(0.0);
            for i in 0..node_count {
                matrix.add(i, i, gmin_floor);
            }
            circuit.stamp_dc_direct(matrix, &mut rhs);
            self.try_stamp_nonlinear_devices_for_dc(circuit, matrix, &mut rhs, &solution)?;
            Self::apply_node_voltage_constraints(circuit, matrix, &mut rhs, node_hints)?;

            let mut new_solution = matrix.solve(&rhs).map_err(SimulationError::Solver)?;
            Self::clamp_solution_to_physical_bounds(&mut new_solution, node_count);
            Self::enforce_node_voltage_hints(circuit, &mut new_solution, node_hints);

            let voltage_converged =
                self.node_voltage_convergence_met(&solution, &new_solution, node_count);
            self.update_device_states_for_dc(circuit, &new_solution);
            let device_converged = circuit.nonlinear_converged(self.device_convergence_criteria());
            let nonlinear_residual_converged = voltage_converged
                && device_converged
                && self.constrained_dc_residual_converged(
                    circuit,
                    matrix,
                    &new_solution,
                    node_hints,
                )?;

            solution = new_solution;
            if voltage_converged && device_converged && nonlinear_residual_converged {
                return Ok(solution);
            }
        }

        Err(SimulationError::ConvergenceFailed(max_iterations))
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
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        let requires_conservative_nonlinear_limiting =
            circuit.requires_conservative_solution_damping();
        // ngspice's flat Newton: when junction devices replace their own
        // iterate voltages (pnjlim), the full node step IS the algorithm and
        // merit-based step shrinking livelocks turn-on (the raw residual
        // transiently rises along the convergent direction). The +/-1kV node
        // containment below stays active regardless.
        let junction_owns_steps = Self::junction_limiting_owns_newton_steps(circuit)
            || self.b3soi_limiter_owns_global_damping(circuit);
        // Use 10x more iterations for DC nonlinear since damping limits voltage change per step
        // With MAX_DELTA_V=2V and standard max_iterations=50, we can only move 100V
        // Need 500+ iterations to traverse the full +/-1000V range if starting from a poor guess
        let dc_max_iterations = self.nonlinear_iteration_budget(10);
        let mut direct_iterations = 0usize;
        let mut residual_stall_iterations = 0usize;
        let mut residual_stalled = false;
        let mut direct_solver_error = None;
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
            self.try_stamp_nonlinear_devices_for_dc(circuit, matrix, &mut rhs, &solution)?;
            // Solve linearized system
            let raw_solution = match matrix.solve(&rhs) {
                Ok(solution) => solution,
                Err(err) => {
                    direct_solver_error = Some(SimulationError::Solver(err));
                    break;
                }
            };
            // Voltage-limiting style damping is critical for strongly-coupled
            // semiconductor nonlinearities, but it can unnecessarily throttle
            // behavioral-only fixed-point updates (e.g., B-source macros that
            // legitimately require kilovolt-level solution jumps).
            let mut new_solution =
                if requires_conservative_nonlinear_limiting && !junction_owns_steps {
                    self.apply_damping_strategy_for_circuit(
                        circuit.has_b3soi_devices(),
                        &solution,
                        &raw_solution,
                        &mut damping_state,
                        junction_owns_steps,
                        |trial| self.nonlinear_merit(circuit, matrix, trial),
                    )
                } else {
                    raw_solution
                };
            circuit.enforce_dc_ideal_voltage_constraints(&mut new_solution);
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
                self.residual_convergence_met(circuit, matrix, &new_solution, &rhs);
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
                && self.try_nonlinear_residual_converged(circuit, matrix, &new_solution)?;
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

            if voltage_converged && device_converged && !nonlinear_residual_converged {
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
        if let Some(err) = direct_solver_error.as_ref() {
            log::warn!(
                "DC Newton-Raphson linear solve failed after {} iteration(s): {}. Trying configured convergence aids...",
                direct_iterations.max(1),
                err
            );
        } else if hit_voltage_limit {
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
            && !circuit.has_b3soi_devices()
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
            if let Some(err) = direct_solver_error {
                return Err(err);
            }
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
        let mut fallback_seed = if circuit.has_b3soi_devices() {
            Self::sanitize_initial_guess(
                &solution,
                solution.len(),
                circuit.num_nodes().min(solution.len()),
            )
        } else {
            self.prefer_lower_merit_scaled_seed(circuit, matrix, &solution, &zero_seed, 1.0)
        };
        let prefer_gate_generation_aids = circuit.has_jfet_gate_generation_branches();
        let prefer_gmin_aids = prefer_gate_generation_aids || !circuit.b3soi.is_empty();
        let mut gmin_attempted = false;

        if prefer_gmin_aids && allow_gmin {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            gmin_attempted = true;
            let gmin_state = circuit.nonlinear_state_snapshot();
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
                    circuit.restore_nonlinear_state(gmin_state);
                    log::warn!(
                        "Early GMIN stepping for weakly anchored nonlinear devices failed with {}. Continuing with configured aids.",
                        e
                    );
                }
            }
        }

        if circuit.has_b3soi_self_heating() && allow_source {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let thermal_state = circuit.nonlinear_state_snapshot();
            circuit.set_b3soi_self_heating_startup_disabled(true);
            circuit.reset_b3soi_operating_point_history();
            let mut electrical_seed = fallback_seed.clone();
            circuit.zero_b3soi_self_heating_temperature_guess(&mut electrical_seed);
            let node_count = circuit.num_nodes().min(electrical_seed.len());
            electrical_seed[node_count..].fill(0.0);
            let mut electrical_result = self.source_stepping_nonlinear_with_guess_and_abort(
                circuit,
                matrix,
                &electrical_seed,
                abort,
            );
            if electrical_result.is_err() && !abort.is_aborted() {
                circuit.reset_b3soi_operating_point_history();
                if let Some(direct_electrical_solution) =
                    self.warm_restart_after_fallback(circuit, matrix, &electrical_seed, abort)
                {
                    electrical_result = Ok(direct_electrical_solution);
                }
            }
            circuit.restore_nonlinear_state(thermal_state);
            match electrical_result {
                Ok(electrical_solution) => {
                    let mut coupled_seed = electrical_solution;
                    circuit.seed_b3soi_self_heating_temperature_guess(&mut coupled_seed);
                    circuit.prime_b3soi_operating_point_from_solution(&coupled_seed);
                    if let Some(restarted) =
                        self.warm_restart_after_fallback(circuit, matrix, &coupled_seed, abort)
                    {
                        log::info!(
                            "B3SOI electrothermal startup warmed the nonlinear state; direct Newton restart accepted."
                        );
                        return Ok(restarted);
                    }
                    if let Some(restarted) = self.static_probe_restart_after_fallback(
                        circuit,
                        matrix,
                        &coupled_seed,
                        abort,
                    ) {
                        log::info!(
                            "B3SOI electrothermal startup required static-probe polishing and is now accepted."
                        );
                        return Ok(restarted);
                    }
                    fallback_seed = self.prefer_lower_merit_scaled_seed(
                        circuit,
                        matrix,
                        &fallback_seed,
                        &coupled_seed,
                        1.0,
                    );
                }
                Err(e) => {
                    if abort.is_aborted() {
                        return Err(SimulationError::Aborted);
                    }
                    log::warn!(
                        "B3SOI electrothermal startup failed with {}. Continuing with configured aids.",
                        e
                    );
                }
            }
        }

        if allow_source {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let source_state = circuit.nonlinear_state_snapshot();
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
                    circuit.restore_nonlinear_state(source_state);
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
            let gmin_state = circuit.nonlinear_state_snapshot();
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
                    circuit.restore_nonlinear_state(gmin_state);
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

    pub(crate) fn solve_nonlinear_transient_op_startup_with_guess_and_hints_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        time: Value,
        initial_guess: &[Value],
        node_hints: &[(usize, Value)],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let node_count = circuit.num_nodes().min(size);
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        let junction_gmin = self.effective_device_junction_gmin(gmin_floor);
        let mut solution = Self::sanitize_initial_guess(initial_guess, size, node_count);
        Self::enforce_node_voltage_hints(circuit, &mut solution, node_hints);
        self.prime_operating_point_seed(
            circuit,
            &solution,
            time,
            crate::xspice::AnalysisType::Transient,
        );

        let mut rhs = vec![0.0; size];
        let max_iterations = self.continuation_iteration_budget(1, 64);

        for iteration in 0..max_iterations {
            if Self::should_abort_iteration(abort, iteration) {
                return Err(SimulationError::Aborted);
            }

            matrix.clear_values();
            rhs.fill(0.0);
            circuit.refresh_jiles_atherton_inductances(&solution);
            Self::stamp_transient_operating_point_linear(
                circuit, matrix, &mut rhs, time, gmin_floor, false,
            );
            self.try_stamp_nonlinear_devices_for_operating_point(
                circuit,
                matrix,
                &mut rhs,
                &solution,
                time,
                crate::xspice::AnalysisType::Transient,
                junction_gmin,
            )?;
            Self::apply_node_voltage_constraints(circuit, matrix, &mut rhs, node_hints)?;

            let mut new_solution = matrix.solve(&rhs).map_err(SimulationError::Solver)?;
            Self::clamp_solution_to_physical_bounds(&mut new_solution, node_count);
            Self::enforce_node_voltage_hints(circuit, &mut new_solution, node_hints);

            let voltage_converged =
                self.node_voltage_convergence_met(&solution, &new_solution, node_count);
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
                && self.constrained_transient_op_residual_converged(
                    circuit,
                    matrix,
                    &new_solution,
                    time,
                    node_hints,
                )?;

            solution = new_solution;
            if voltage_converged && device_converged && nonlinear_residual_converged {
                return Ok(solution);
            }
        }

        Err(SimulationError::ConvergenceFailed(max_iterations))
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
            self.dc_nodal_gmin_floor(circuit),
            true,
        );
        if !circuit.behavioral_sources.is_empty()
            && !circuit.behavioral_sources.has_solution_dependent_sources()
        {
            let zero_solution = vec![0.0; size];
            circuit.stamp_behavioral_sources(matrix, &mut rhs, &zero_solution, time);
        }
        if circuit.has_xspice_devices() {
            let zero_solution = vec![0.0; size];
            circuit.stamp_xspice_transient_trial(matrix, &mut rhs, time, 0.0, &zero_solution);
        }
        match matrix.solve(&rhs) {
            Ok(solution) if solution.iter().all(|value| value.is_finite()) => Ok(solution),
            Ok(_) | Err(_) if !circuit.inductors.is_empty() => {
                matrix.clear_values();
                rhs.fill(0.0);
                Self::stamp_transient_current_seed_linear(
                    circuit,
                    matrix,
                    &mut rhs,
                    time,
                    self.dc_nodal_gmin_floor(circuit),
                );
                if !circuit.behavioral_sources.is_empty()
                    && !circuit.behavioral_sources.has_solution_dependent_sources()
                {
                    let zero_solution = vec![0.0; size];
                    circuit.stamp_behavioral_sources(matrix, &mut rhs, &zero_solution, time);
                }
                if circuit.has_xspice_devices() {
                    let zero_solution = vec![0.0; size];
                    circuit.stamp_xspice_transient_trial(
                        matrix,
                        &mut rhs,
                        time,
                        0.0,
                        &zero_solution,
                    );
                }
                match matrix.solve(&rhs) {
                    Ok(solution) if solution.iter().all(|value| value.is_finite()) => Ok(solution),
                    Ok(_) => Err(SimulationError::Solver(
                        crate::solver::SolverError::SingularMatrix,
                    )),
                    Err(err) => Err(SimulationError::Solver(err)),
                }
            }
            Ok(_) => Err(SimulationError::Solver(
                crate::solver::SolverError::SingularMatrix,
            )),
            Err(err) => Err(SimulationError::Solver(err)),
        }
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
        let gmin_floor =
            if self.config.spice_dialect == SpiceDialect::Xyce && !node_hints.is_empty() {
                0.0
            } else {
                self.dc_nodal_gmin_floor(circuit)
            };
        let junction_gmin = self.effective_device_junction_gmin(gmin_floor);
        let mut solution = self
            .linear_presolve_for_guess_with_linear_stamp(circuit, matrix, |circuit, matrix, rhs| {
                Self::stamp_transient_operating_point_linear(
                    circuit, matrix, rhs, time, 0.0, false,
                );
            })
            .unwrap_or_else(|| vec![0.0; size]);

        for &(node_id, voltage) in node_hints {
            if !voltage.is_finite() || node_id == 0 || node_id > circuit.num_nodes() {
                continue;
            }
            solution[node_id - 1] = voltage;
        }

        solution = Self::sanitize_initial_guess(&solution, size, circuit.num_nodes().min(size));
        let mut nodeset_startup_solution = None;
        if !node_hints.is_empty() {
            match self.solve_nonlinear_transient_op_startup_with_guess_and_hints_abort(
                circuit, matrix, time, &solution, node_hints, abort,
            ) {
                Ok(nodeset_solution) => {
                    solution = nodeset_solution;
                    nodeset_startup_solution = Some(solution.clone());
                }
                Err(SimulationError::Aborted) => return Err(SimulationError::Aborted),
                Err(err) => {
                    log::debug!(
                        "NODESET-constrained transient operating-point startup did not converge: {err}; continuing from hinted seed."
                    );
                }
            }
        }
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
        let junction_owns_steps = Self::junction_limiting_owns_newton_steps(circuit)
            || self.b3soi_limiter_owns_global_damping(circuit);
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
                circuit, matrix, &mut rhs, time, gmin_floor, false,
            );
            self.try_stamp_nonlinear_devices_for_operating_point(
                circuit,
                matrix,
                &mut rhs,
                &solution,
                time,
                crate::xspice::AnalysisType::Transient,
                junction_gmin,
            )?;

            let raw_solution = match matrix.solve(&rhs) {
                Ok(solution) => solution,
                Err(err) => {
                    if let Some(startup_solution) = nodeset_startup_solution.as_ref() {
                        log::debug!(
                            "Unconstrained transient operating-point solve failed after NODESET startup ({err}); using the NODESET startup state."
                        );
                        return Ok(startup_solution.clone());
                    }
                    return Err(SimulationError::Solver(err));
                }
            };
            let mut new_solution =
                if requires_conservative_nonlinear_limiting && !junction_owns_steps {
                    self.apply_damping_strategy_for_circuit(
                        circuit.has_b3soi_devices(),
                        &solution,
                        &raw_solution,
                        &mut damping_state,
                        junction_owns_steps,
                        |trial| {
                            self.nonlinear_merit_with_linear_stamp(
                                circuit,
                                matrix,
                                trial,
                                |circuit, matrix, rhs| {
                                    circuit.refresh_jiles_atherton_inductances(trial);
                                    Self::stamp_transient_operating_point_linear(
                                        circuit, matrix, rhs, time, gmin_floor, false,
                                    );
                                },
                            )
                        },
                    )
                } else {
                    raw_solution
                };
            circuit.enforce_ideal_voltage_constraints(&mut new_solution, time);
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
                            circuit, matrix, rhs, time, gmin_floor, false,
                        );
                    },
                );

            solution = new_solution;
            if voltage_converged && device_converged && nonlinear_residual_converged {
                return Ok(solution);
            }
        }

        if let Some(startup_solution) = nodeset_startup_solution {
            log::debug!(
                "Unconstrained transient operating-point solve did not converge after NODESET startup; using the NODESET startup state."
            );
            return Ok(startup_solution);
        }

        Err(SimulationError::ConvergenceFailed(tranop_max_iterations))
    }
}
