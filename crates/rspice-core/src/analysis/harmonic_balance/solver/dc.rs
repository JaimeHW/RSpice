//! DC operating-point solve used to seed harmonic-balance Newton iteration.

use super::*;
use crate::solver::convergence::SourceStepper;
use crate::solver::{SolverError, StaticMatrix};

#[derive(Debug, Clone)]
struct HbDcCheckpoint {
    node_voltages: Vec<Value>,
    branch_currents: Vec<Value>,
}

impl HbSolver {
    /// Solve DC operating point before full HB iteration
    ///
    /// This method solves only the DC component (k=0) of the HB problem to establish
    /// the nonlinear device operating points. The DC solution provides a much better
    /// initial guess for the full HB Newton iteration.
    ///
    /// Uses the same convergence aids as the full HB solver:
    /// - GMIN stepping for ill-conditioned circuits
    /// - Source stepping as fallback
    ///
    /// Returns the DC node voltages if successful.
    pub fn solve_dc_operating_point(
        &mut self,
        state: &mut HbSolverState,
    ) -> Result<Vec<Value>, HbError> {
        self.solve_dc_operating_point_with_abort(state, &NoAbort)
    }

    /// Solve the HB DC seed while polling a cooperative abort signal.
    pub fn solve_dc_operating_point_with_abort(
        &mut self,
        state: &mut HbSolverState,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, HbError> {
        if abort.is_aborted() {
            return Err(HbError::Aborted);
        }
        let branch_count = self.validate_dc_exact_mna_registry()?;
        state.try_prepare_mna_branches(branch_count, self.num_harmonics)?;
        self.validate_dc_state(state)?;
        // The DC seed is part of the same authenticated HB solve. Respect the
        // caller's tolerances instead of silently weakening them.
        let dc_reltol = self.config.tolerance;
        let dc_abstol = self.config.abstol;
        if !dc_reltol.is_finite() || dc_reltol <= 0.0 {
            return Err(HbError::InvalidCircuit(format!(
                "HB relative tolerance must be finite and positive, got {dc_reltol}"
            )));
        }
        if !dc_abstol.is_finite() || dc_abstol <= 0.0 {
            return Err(HbError::InvalidCircuit(format!(
                "HB absolute tolerance must be finite and positive, got {dc_abstol}"
            )));
        }

        // DC-specific iteration limit
        let dc_max_iter = self.config.max_iterations.max(150);

        // For linear circuits, DC is just a linear solve at k=0
        if !self.has_nonlinear_devices() {
            self.solve_dc_linear(state)?;
            return Ok(self.extract_dc_solution(state));
        }

        // GMIN stepping is a homotopy only. The returned seed must satisfy the
        // physical zero-GMIN DC equations.
        let target_gmin = 0.0;
        let homotopy_floor = 1.0e-12;

        // Initialize diode voltages with forward bias estimate (0.6V per diode)
        // This gives Newton a much better starting point than V=0
        self.initialize_diode_voltages(state);

        // Step 1: Try direct DC Newton with minimal GMIN
        if self.dc_newton_inner_loop(
            state,
            target_gmin,
            dc_max_iter,
            dc_reltol,
            dc_abstol,
            1.0,
            abort,
        )? {
            return Ok(self.extract_dc_solution(state));
        }

        // Step 2: GMIN stepping - progressively increase GMIN until convergence,
        // then refine back down
        for gmin_level in [1e-9, 1e-6, 1e-4, 1e-2, 0.1, 1.0] {
            if self.dc_newton_inner_loop(
                state,
                gmin_level,
                dc_max_iter,
                dc_reltol * 10.0, // Relaxed tolerance during stepping
                dc_abstol * 10.0,
                1.0,
                abort,
            )? {
                // Converged at this GMIN level - refine to target
                let mut current_gmin = gmin_level;
                let mut last_good_state = self.capture_dc_checkpoint(state)?;
                let mut last_good_gmin = current_gmin;
                let mut refine_failures = 0;

                while current_gmin > homotopy_floor {
                    current_gmin /= 2.0;
                    if self.dc_newton_inner_loop(
                        state,
                        current_gmin,
                        dc_max_iter,
                        dc_reltol,
                        dc_abstol,
                        1.0,
                        abort,
                    )? {
                        last_good_state = self.capture_dc_checkpoint(state)?;
                        last_good_gmin = current_gmin;
                        refine_failures = 0; // Reset failure count on success
                    } else {
                        refine_failures += 1;
                        // Restore last good state and keep trying with smaller steps
                        self.restore_dc_checkpoint(state, &last_good_state)?;
                        // After too many consecutive failures, try slower reduction
                        if refine_failures > 3 {
                            break;
                        }
                        // Try 10% reduction instead of 50%
                        current_gmin = last_good_gmin * 0.9;
                    }
                }

                // A high-GMIN solution is only a warm start. Accept the seed
                // only after a full solve on the unmodified DC equations.
                if self.dc_newton_inner_loop(
                    state,
                    target_gmin,
                    dc_max_iter,
                    dc_reltol,
                    dc_abstol,
                    1.0,
                    abort,
                )? {
                    return Ok(self.extract_dc_solution(state));
                }
            }
        }

        // Step 3: Source stepping - ramp every independent DC source, including
        // exact ideal-voltage KVL rows, from zero to full strength.
        for node in 0..self.num_nodes {
            if node < state.x.len() && !state.x[node].is_empty() {
                state.x[node][0] = Complex64::new(0.0, 0.0);
            }
        }
        for spectrum in &mut state.mna_branch_currents {
            spectrum[0] = Complex64::new(0.0, 0.0);
        }

        // Use SourceStepper for DC sources
        let mut source_stepper = SourceStepper::new();
        let max_steps = 50;
        let mut step_count = 0;

        while !source_stepper.is_complete() && step_count < max_steps {
            let factor = source_stepper.factor();
            step_count += 1;
            let checkpoint = self.capture_dc_checkpoint(state)?;

            if self.dc_newton_inner_loop(
                state,
                1e-6,
                dc_max_iter / 2,
                dc_reltol * 10.0,
                dc_abstol * 10.0,
                factor,
                abort,
            )? {
                source_stepper.advance_on_success();
            } else {
                self.restore_dc_checkpoint(state, &checkpoint)?;
                if !source_stepper.reduce_on_failure() {
                    break;
                }
            }
        }

        // Final DC solve with full sources
        if source_stepper.is_complete()
            && self.dc_newton_inner_loop(
                state,
                target_gmin,
                dc_max_iter,
                dc_reltol,
                dc_abstol,
                1.0,
                abort,
            )?
        {
            return Ok(self.extract_dc_solution(state));
        }

        // DC solve failed - return what we have
        Err(HbError::ConvergenceFailed {
            iterations: step_count,
            residual: state.residual_norm,
        })
    }

    /// Solve DC for linear circuit (no nonlinear devices)
    fn solve_dc_linear(&self, state: &mut HbSolverState) -> Result<(), HbError> {
        let n = self.num_nodes;
        let branches = self.exact_mna_branches();
        let total_unknowns = n.checked_add(branches.len()).ok_or_else(|| {
            HbError::InvalidCircuit("HB DC MNA dimension exceeds this platform".to_string())
        })?;
        let mut g_dc = vec![vec![0.0; total_unknowns]; total_unknowns];

        for &(row, col, g) in &self.g_matrix {
            if row < n && col < n {
                g_dc[row][col] += g;
            }
        }

        if branches.is_empty() {
            // Compatibility for direct node-only solver clients. Production
            // exact-MNA HB registers every inductor branch and never reaches
            // this surrogate path.
            for &(row, col, l) in &self.l_matrix {
                if row < n && col < n && l.abs() > 1e-30 {
                    g_dc[row][col] += inductor_dc_short_admittance(l);
                }
            }
        }

        let mut rhs = vec![0.0; total_unknowns];
        for (node, slot) in rhs.iter_mut().take(n).enumerate() {
            *slot = self.dc_nodal_source(node, 1.0)?;
        }
        for (branch_index, branch) in branches.iter().enumerate() {
            let row = n + branch_index;
            let (node_pos, node_neg) = Self::dc_branch_terminals(branch);
            if node_pos > 0 {
                g_dc[node_pos - 1][row] += 1.0;
                g_dc[row][node_pos - 1] += 1.0;
            }
            if node_neg > 0 {
                g_dc[node_neg - 1][row] -= 1.0;
                g_dc[row][node_neg - 1] -= 1.0;
            }
            rhs[row] = self.dc_branch_source(branch, 1.0)?;
        }

        let solution = self.solve_real_linear_system(&g_dc, &rhs)?;

        for (node, &voltage) in solution.iter().take(n).enumerate() {
            state.x[node][0] = Complex64::new(voltage, 0.0);
        }
        for (branch, &current) in state
            .mna_branch_currents
            .iter_mut()
            .zip(solution.iter().skip(n))
        {
            branch[0] = Complex64::new(current, 0.0);
        }

        Ok(())
    }

    /// DC Newton inner loop - solves DC component only
    ///
    /// Uses the same algorithm as the full HB Newton but operates only on harmonic 0.
    fn dc_newton_inner_loop(
        &mut self,
        state: &mut HbSolverState,
        gmin: Value,
        max_iterations: usize,
        tol: Value,
        abstol: Value,
        source_scale: Value,
        abort: &dyn AbortSignal,
    ) -> Result<bool, HbError> {
        if !source_scale.is_finite() || !(0.0..=1.0).contains(&source_scale) {
            return Err(HbError::InvalidCircuit(format!(
                "HB DC source scale must be finite and within [0, 1], got {source_scale:e}"
            )));
        }
        for iteration in 0..max_iterations {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            state.iteration = iteration;

            // Compute DC residual
            self.compute_dc_residual(state, gmin, source_scale)?;

            // Check convergence per KCL row: |res| <= abstol + reltol*scale
            // with the scale built from that row's own current contributions.
            // Any circuit-wide reference (a norm, the max source current)
            // lets a microamp imbalance at a high-impedance node hide under
            // the amp scale of stiff Norton source rows.
            if state.dc_rows_converged_with_branch_tolerances(tol, abstol, crate::constants::VNTOL)
            {
                return Ok(true);
            }

            // Build DC Jacobian
            let jacobian = self.build_dc_jacobian(state, gmin)?;

            // Solve for delta_x: J * delta = -residual (standard Newton-Raphson)
            // We need -R because: R(x) = 0, Taylor: R(x+delta) ≈ R(x) + J*delta = 0
            // So J*delta = -R
            let mut neg_residual = Vec::new();
            neg_residual
                .try_reserve_exact(self.num_nodes + state.mna_branch_residual.len())
                .map_err(|error| {
                    HbError::InvalidCircuit(format!(
                        "HB DC residual-vector allocation failed: {error}"
                    ))
                })?;
            neg_residual.extend(state.residual.iter().map(|row| -row[0].re));
            neg_residual.extend(state.mna_branch_residual.iter().map(|row| -row[0].re));

            let delta_x = match self.solve_real_linear_system(&jacobian, &neg_residual) {
                Ok(d) => d,
                Err(_) => return Ok(false), // Singular Jacobian
            };

            // Line search with DC voltage limiting
            self.apply_dc_line_search(
                state,
                &delta_x,
                gmin,
                tol,
                abstol,
                crate::constants::VNTOL,
                source_scale,
            )?;
        }

        Ok(false)
    }

    /// Compute DC residual: R = I_source_dc - G*V_dc - I_nonlinear(V_dc) - gmin*V_dc
    fn compute_dc_residual(
        &mut self,
        state: &mut HbSolverState,
        gmin: Value,
        source_scale: Value,
    ) -> Result<(), HbError> {
        let n = self.num_nodes;

        // Initialize residual with DC sources; the per-row scale accumulates
        // |contribution| alongside every term for the KCL convergence test.
        for node in 0..n {
            if node < state.residual.len() && !state.residual[node].is_empty() {
                let source = self.dc_nodal_source(node, source_scale)?;
                state.residual[node][0] = Complex64::new(source, 0.0);
                if !state.residual_scale[node].is_empty() {
                    state.residual_scale[node][0] = source.abs();
                }
            }
        }

        // Extract DC voltages
        let v_dc: Vec<Value> = (0..n)
            .map(|node| {
                state
                    .x
                    .get(node)
                    .and_then(|x| x.first())
                    .map(|c| c.re)
                    .unwrap_or(0.0)
            })
            .collect();

        // Subtract linear contributions: G * V_dc
        for &(row, col, g) in &self.g_matrix {
            if row < n && col < n && row < state.residual.len() {
                let contribution = g * v_dc[col];
                state.residual[row][0] -= Complex64::new(contribution, 0.0);
                state.residual_scale[row][0] += contribution.abs();
            }
        }

        if self.exact_mna_branches().is_empty() {
            // Compatibility for direct node-only solver users.
            for &(row, col, l) in &self.l_matrix {
                if row < n && col < n && row < state.residual.len() && l.abs() > 1e-30 {
                    let y_l = inductor_dc_short_admittance(l);
                    state.residual[row][0] -= Complex64::new(y_l * v_dc[col], 0.0);
                    state.residual_scale[row][0] += y_l.abs() * v_dc[col].abs();
                }
            }
        }

        // Exact branch currents enter only the terminal KCL rows. GMIN and
        // nonlinear voltage limiting remain node-only homotopies.
        for (branch_index, branch) in self.exact_mna_branches().iter().enumerate() {
            let current = state.mna_branch_currents[branch_index][0].re;
            let (node_pos, node_neg) = Self::dc_branch_terminals(branch);
            if node_pos > 0 {
                state.residual[node_pos - 1][0] -= Complex64::new(current, 0.0);
                state.residual_scale[node_pos - 1][0] += current.abs();
            }
            if node_neg > 0 {
                state.residual[node_neg - 1][0] += Complex64::new(current, 0.0);
                state.residual_scale[node_neg - 1][0] += current.abs();
            }
        }

        // Subtract GMIN: gmin * V_dc (diagonal)
        for node in 0..n {
            if node < state.residual.len() && !state.residual[node].is_empty() {
                state.residual[node][0] -= Complex64::new(gmin * v_dc[node], 0.0);
                state.residual_scale[node][0] += gmin * v_dc[node].abs();
            }
        }

        // Add nonlinear device currents (device returns current INTO each node)
        // KCL: sum of currents INTO node = 0
        // R = I_source + I_device_into - G*V - gmin*V
        for device in &self.nonlinear_devices {
            let currents = device.evaluate(&v_dc);
            for (node, current) in currents {
                if node < state.residual.len() && !state.residual[node].is_empty() {
                    state.residual[node][0] += Complex64::new(current, 0.0);
                    state.residual_scale[node][0] += current.abs();
                }
            }
        }

        // Verilog-A devices participate in the DC seed too; skipping them
        // here let the seed "converge" on a circuit without their currents.
        #[cfg(feature = "veriloga")]
        if !self.veriloga_nonlinear_devices.is_empty() {
            for device in &mut self.veriloga_nonlinear_devices {
                device.device.update_all_voltages(&v_dc);
                let values = device.try_evaluate("DC residual evaluation")?;
                for (program_idx, value) in values.iter().enumerate() {
                    let Some(rows) = device.rhs_rows.get(program_idx) else {
                        continue;
                    };
                    for &(row, sign) in rows {
                        if row < n && row < state.residual.len() {
                            state.residual[row][0] += Complex64::new(sign * *value, 0.0);
                            state.residual_scale[row][0] += value.abs();
                        }
                    }
                }
            }
        }

        // Exact DC KVL: ideal sources retain their authored DC voltage while
        // inductors enforce a literal zero voltage drop, never a conductance
        // surrogate. These are voltage rows with their own convergence scale.
        for (branch_index, branch) in self.exact_mna_branches().iter().enumerate() {
            let (node_pos, node_neg) = Self::dc_branch_terminals(branch);
            let voltage =
                Self::dc_node_voltage(&v_dc, node_pos) - Self::dc_node_voltage(&v_dc, node_neg);
            let source = self.dc_branch_source(branch, source_scale)?;
            state.mna_branch_residual[branch_index][0] = Complex64::new(source - voltage, 0.0);
            state.mna_branch_residual_scale[branch_index][0] = source
                .abs()
                .max(Self::dc_node_voltage(&v_dc, node_pos).abs())
                .max(Self::dc_node_voltage(&v_dc, node_neg).abs());
        }

        self.validate_dc_residual_state(state)?;
        let diagnostic_norm: Value = state
            .residual
            .iter()
            .chain(state.mna_branch_residual.iter())
            .fold(0.0, |norm, row| norm.hypot(row[0].re));
        state.residual_norm = if diagnostic_norm.is_finite() {
            diagnostic_norm
        } else {
            Value::MAX
        };
        Ok(())
    }

    /// Build DC Jacobian: J = -G - dI_nonlinear/dV - gmin*I
    fn build_dc_jacobian(
        &mut self,
        state: &HbSolverState,
        gmin: Value,
    ) -> Result<Vec<Vec<Value>>, HbError> {
        let n = self.num_nodes;
        let total_unknowns = n
            .checked_add(self.exact_mna_branches().len())
            .ok_or_else(|| {
                HbError::InvalidCircuit(
                    "HB DC Jacobian dimension exceeds this platform".to_string(),
                )
            })?;
        let mut jacobian = vec![vec![0.0; total_unknowns]; total_unknowns];

        // Linear contribution: -G
        for &(row, col, g) in &self.g_matrix {
            if row < n && col < n {
                jacobian[row][col] -= g;
            }
        }

        if self.exact_mna_branches().is_empty() {
            for &(row, col, l) in &self.l_matrix {
                if row < n && col < n && l.abs() > 1e-30 {
                    jacobian[row][col] -= inductor_dc_short_admittance(l);
                }
            }
        }

        // GMIN contribution: -gmin on diagonal
        for i in 0..n {
            jacobian[i][i] -= gmin;
        }

        // Nonlinear device Jacobians
        let v_dc: Vec<Value> = (0..n)
            .map(|node| {
                state
                    .x
                    .get(node)
                    .and_then(|x| x.first())
                    .map(|c| c.re)
                    .unwrap_or(0.0)
            })
            .collect();

        // Nonlinear device Jacobians
        // Device returns MNA conductance stamps (+gd on diagonal for diode)
        // But dI_into/dV = -gd (more voltage → more current leaving → less current into)
        // Since J = dR/dV = dI_into/dV - G - gmin, we need to subtract the device stamps
        // J = -G_device - G_linear - gmin
        for device in &self.nonlinear_devices {
            let jac_entries = device.jacobian(&v_dc);
            for ((row, col), value) in jac_entries {
                if row < n && col < n {
                    // Subtract device conductance stamp to get correct dI_into/dV
                    jacobian[row][col] -= value;
                }
            }
        }

        #[cfg(feature = "veriloga")]
        if !self.veriloga_nonlinear_devices.is_empty() {
            for device in &mut self.veriloga_nonlinear_devices {
                device.device.update_all_voltages(&v_dc);
                let jac_entries = device.try_compute_jacobian("DC Jacobian evaluation")?;
                for entry in jac_entries {
                    let Some(prog_locs) = device.jacobian_locs.get(entry.program_idx) else {
                        continue;
                    };
                    let Some(&(row, col)) = prog_locs.get(entry.jacobian_idx) else {
                        continue;
                    };
                    if let (Some(i), Some(j)) = (row, col)
                        && i < n
                        && j < n
                    {
                        jacobian[i][j] -= entry.value;
                    }
                }
            }
        }

        for (branch_index, branch) in self.exact_mna_branches().iter().enumerate() {
            let branch_coordinate = n + branch_index;
            let (node_pos, node_neg) = Self::dc_branch_terminals(branch);
            if node_pos > 0 {
                jacobian[node_pos - 1][branch_coordinate] -= 1.0;
                jacobian[branch_coordinate][node_pos - 1] -= 1.0;
            }
            if node_neg > 0 {
                jacobian[node_neg - 1][branch_coordinate] += 1.0;
                jacobian[branch_coordinate][node_neg - 1] += 1.0;
            }
        }

        Ok(jacobian)
    }

    /// Apply DC line search with voltage limiting
    fn apply_dc_line_search(
        &mut self,
        state: &mut HbSolverState,
        delta_x: &[Value],
        gmin: Value,
        reltol: Value,
        current_abstol: Value,
        voltage_abstol: Value,
        source_scale: Value,
    ) -> Result<(), HbError> {
        let n = self.num_nodes;
        let expected_unknowns =
            n.checked_add(state.mna_branch_currents.len())
                .ok_or_else(|| {
                    HbError::InvalidCircuit(
                        "HB DC line-search dimension exceeds this platform".to_string(),
                    )
                })?;
        if delta_x.len() != expected_unknowns || delta_x.iter().any(|value| !value.is_finite()) {
            return Err(HbError::InvalidCircuit(format!(
                "HB DC line-search update has {} finite-qualified coordinates; expected {expected_unknowns}",
                delta_x.len()
            )));
        }
        let mut alpha = 1.0;
        let min_alpha = 0.001;
        let armijo_c = 1e-4;

        let checkpoint = self.capture_dc_checkpoint(state)?;
        let original_merit =
            state.certificate_merit(reltol, current_abstol, voltage_abstol, true)?;

        let mut best_alpha = alpha;
        let mut best_merit = f64::INFINITY;

        while alpha >= min_alpha {
            if !Self::apply_dc_trial_update(state, &checkpoint, delta_x, alpha) {
                alpha *= 0.5;
                continue;
            }

            // Compute new residual
            self.compute_dc_residual(state, gmin, source_scale)?;
            let trial_merit =
                state.certificate_merit(reltol, current_abstol, voltage_abstol, true)?;

            // Track best result
            if trial_merit < best_merit {
                best_merit = trial_merit;
                best_alpha = alpha;
            }

            // Dimensionless sufficient decrease across separately normalized
            // KCL-current and KVL-voltage row certificates.
            let sufficient_decrease = original_merit * (1.0 - armijo_c * alpha);
            if trial_merit <= sufficient_decrease {
                return Ok(()); // Accepted step
            }

            alpha *= 0.5;
        }

        if !best_merit.is_finite()
            || !Self::apply_dc_trial_update(state, &checkpoint, delta_x, best_alpha)
        {
            self.restore_dc_checkpoint(state, &checkpoint)?;
            return Err(HbError::InvalidCircuit(
                "HB DC line search found no finite branch-inclusive trial state".to_string(),
            ));
        }
        self.compute_dc_residual(state, gmin, source_scale)?;
        Ok(())
    }

    fn validate_dc_exact_mna_registry(&self) -> Result<usize, HbError> {
        let branches = self.exact_mna_branches();
        let names = self.exact_mna_branch_names();
        if names.len() != branches.len() {
            return Err(HbError::InvalidCircuit(format!(
                "HB DC exact-MNA registry has {} descriptors and {} names",
                branches.len(),
                names.len()
            )));
        }
        if self.voltage_source_branches.len() != self.num_branches {
            return Err(HbError::InvalidCircuit(format!(
                "HB DC voltage-source storage has {} descriptors for {} registered branch unknowns",
                self.voltage_source_branches.len(),
                self.num_branches
            )));
        }
        if branches.is_empty() && !self.voltage_source_branches.is_empty() {
            return Err(HbError::InvalidCircuit(
                "HB DC has ideal-voltage branch descriptors but no canonical exact-MNA registry"
                    .to_string(),
            ));
        }
        if !branches.is_empty()
            && !self.l_matrix.is_empty()
            && branches
                .iter()
                .any(|branch| matches!(branch, ExactMnaBranch::Inductor { .. }))
        {
            return Err(HbError::InvalidCircuit(
                "HB DC exact inductor branches cannot coexist with nodal inductor admittance stamps"
                    .to_string(),
            ));
        }

        let mut seen_names = std::collections::HashSet::new();
        seen_names.try_reserve(names.len()).map_err(|error| {
            HbError::InvalidCircuit(format!(
                "HB DC exact-MNA identity allocation failed: {error}"
            ))
        })?;
        let mut seen_voltage_sources = std::collections::HashSet::new();
        seen_voltage_sources
            .try_reserve(self.voltage_source_branches.len())
            .map_err(|error| {
                HbError::InvalidCircuit(format!(
                    "HB DC voltage-source identity allocation failed: {error}"
                ))
            })?;
        for (index, (branch, name)) in branches.iter().zip(names).enumerate() {
            if name.is_empty() || name.trim() != name || !seen_names.insert(name.to_uppercase()) {
                return Err(HbError::InvalidCircuit(format!(
                    "HB DC exact-MNA branch name '{name}' is empty, non-canonical, or duplicated"
                )));
            }
            let expected_ordinal = index + 1;
            let (branch_ordinal, node_pos, node_neg) = match branch {
                ExactMnaBranch::VoltageSource {
                    branch_ordinal,
                    node_pos,
                    node_neg,
                    source_index,
                    source,
                } => {
                    if !seen_voltage_sources.insert(*source_index) {
                        return Err(HbError::InvalidCircuit(format!(
                            "HB DC ideal-voltage source index {source_index} is registered more than once"
                        )));
                    }
                    let source = source.as_ref().ok_or_else(|| {
                        HbError::InvalidCircuit(format!(
                            "HB DC ideal-voltage branch '{name}' has no large-signal source evidence"
                        ))
                    })?;
                    if self.voltage_source_branches.get(*source_index) != Some(source)
                        || source.node_pos != *node_pos
                        || source.node_neg != *node_neg
                        || !source.dc_voltage.is_finite()
                        || source
                            .ac_harmonics
                            .iter()
                            .any(|(_, value)| !value.re.is_finite() || !value.im.is_finite())
                    {
                        return Err(HbError::InvalidCircuit(format!(
                            "HB DC ideal-voltage branch '{name}' has inconsistent source evidence"
                        )));
                    }
                    (*branch_ordinal, *node_pos, *node_neg)
                }
                ExactMnaBranch::Inductor {
                    branch_ordinal,
                    node_pos,
                    node_neg,
                    inductance,
                } => {
                    if !inductance.is_finite() || *inductance == 0.0 {
                        return Err(HbError::InvalidCircuit(format!(
                            "HB DC inductor branch '{name}' has invalid inductance {inductance:e}"
                        )));
                    }
                    (*branch_ordinal, *node_pos, *node_neg)
                }
            };
            if branch_ordinal != expected_ordinal
                || node_pos > self.num_nodes
                || node_neg > self.num_nodes
                || node_pos == node_neg
            {
                return Err(HbError::InvalidCircuit(format!(
                    "HB DC exact-MNA branch '{name}' has ordinal {branch_ordinal} and terminals ({node_pos}, {node_neg}); expected ordinal {expected_ordinal} within {} nodes",
                    self.num_nodes
                )));
            }
        }
        if seen_voltage_sources.len() != self.voltage_source_branches.len() {
            return Err(HbError::InvalidCircuit(format!(
                "HB DC exact-MNA registry covers {} of {} ideal-voltage source descriptors",
                seen_voltage_sources.len(),
                self.voltage_source_branches.len()
            )));
        }
        Ok(branches.len())
    }

    fn validate_dc_state(&self, state: &HbSolverState) -> Result<(), HbError> {
        let width = self.num_harmonics.checked_add(1).ok_or_else(|| {
            HbError::InvalidCircuit("HB DC harmonic width exceeds this platform".to_string())
        })?;
        if state.x.len() != self.num_nodes
            || state.residual.len() != self.num_nodes
            || state.residual_scale.len() != self.num_nodes
        {
            return Err(HbError::InvalidCircuit(
                "HB DC node-state cardinality does not match the solver".to_string(),
            ));
        }
        for node in 0..self.num_nodes {
            if state.x[node].len() != width
                || state.residual[node].len() != width
                || state.residual_scale[node].len() != width
            {
                return Err(HbError::InvalidCircuit(format!(
                    "HB DC node {node} has an inconsistent harmonic-state width"
                )));
            }
            if state.x[node]
                .iter()
                .chain(&state.residual[node])
                .any(|value| !value.re.is_finite() || !value.im.is_finite())
                || state.residual_scale[node]
                    .iter()
                    .any(|value| !value.is_finite() || *value < 0.0)
                || state.x[node][0].im != 0.0
            {
                return Err(HbError::InvalidCircuit(format!(
                    "HB DC node {node} contains non-finite state or a nonzero imaginary DC component"
                )));
            }
        }
        Ok(())
    }

    fn validate_dc_residual_state(&self, state: &HbSolverState) -> Result<(), HbError> {
        for (node, (residual, scale)) in
            state.residual.iter().zip(&state.residual_scale).enumerate()
        {
            if !residual[0].re.is_finite()
                || residual[0].im != 0.0
                || !scale[0].is_finite()
                || scale[0] < 0.0
            {
                return Err(HbError::InvalidCircuit(format!(
                    "HB DC KCL row {node} produced a non-finite residual certificate"
                )));
            }
        }
        for (branch, (residual, scale)) in state
            .mna_branch_residual
            .iter()
            .zip(&state.mna_branch_residual_scale)
            .enumerate()
        {
            if !residual[0].re.is_finite()
                || residual[0].im != 0.0
                || !scale[0].is_finite()
                || scale[0] < 0.0
            {
                return Err(HbError::InvalidCircuit(format!(
                    "HB DC KVL row {branch} produced a non-finite residual certificate"
                )));
            }
        }
        Ok(())
    }

    fn dc_nodal_source(&self, node: usize, source_scale: Value) -> Result<Value, HbError> {
        let source = self
            .source_spectra
            .get(node)
            .and_then(|spectrum| spectrum.first())
            .copied()
            .unwrap_or(Complex64::ZERO);
        if !source.re.is_finite() || !source.im.is_finite() || source.im != 0.0 {
            return Err(HbError::InvalidCircuit(format!(
                "HB DC nodal source row {node} has an invalid harmonic-zero coefficient"
            )));
        }
        let scaled = source.re * source_scale;
        if !scaled.is_finite() {
            return Err(HbError::InvalidCircuit(format!(
                "HB DC nodal source row {node} overflows at source scale {source_scale:e}"
            )));
        }
        Ok(scaled)
    }

    fn dc_branch_source(
        &self,
        branch: &ExactMnaBranch,
        source_scale: Value,
    ) -> Result<Value, HbError> {
        let source = match branch {
            ExactMnaBranch::VoltageSource {
                source: Some(source),
                ..
            } => source.dc_voltage * source_scale,
            ExactMnaBranch::VoltageSource { source: None, .. } => {
                return Err(HbError::InvalidCircuit(
                    "HB DC ideal-voltage branch lacks large-signal source evidence".to_string(),
                ));
            }
            ExactMnaBranch::Inductor { .. } => 0.0,
        };
        if !source.is_finite() {
            return Err(HbError::InvalidCircuit(
                "HB DC exact-MNA branch source is non-finite".to_string(),
            ));
        }
        Ok(source)
    }

    fn dc_branch_terminals(branch: &ExactMnaBranch) -> (usize, usize) {
        match branch {
            ExactMnaBranch::VoltageSource {
                node_pos, node_neg, ..
            }
            | ExactMnaBranch::Inductor {
                node_pos, node_neg, ..
            } => (*node_pos, *node_neg),
        }
    }

    fn dc_node_voltage(voltages: &[Value], node: usize) -> Value {
        if node == 0 { 0.0 } else { voltages[node - 1] }
    }

    fn capture_dc_checkpoint(&self, state: &HbSolverState) -> Result<HbDcCheckpoint, HbError> {
        self.validate_dc_state(state)?;
        Ok(HbDcCheckpoint {
            node_voltages: state.x.iter().map(|spectrum| spectrum[0].re).collect(),
            branch_currents: state
                .mna_branch_currents
                .iter()
                .map(|spectrum| spectrum[0].re)
                .collect(),
        })
    }

    fn restore_dc_checkpoint(
        &self,
        state: &mut HbSolverState,
        checkpoint: &HbDcCheckpoint,
    ) -> Result<(), HbError> {
        if checkpoint.node_voltages.len() != state.x.len()
            || checkpoint.branch_currents.len() != state.mna_branch_currents.len()
            || checkpoint
                .node_voltages
                .iter()
                .chain(&checkpoint.branch_currents)
                .any(|value| !value.is_finite())
        {
            return Err(HbError::InvalidCircuit(
                "HB DC checkpoint does not match the branch-inclusive state".to_string(),
            ));
        }
        for (spectrum, &voltage) in state.x.iter_mut().zip(&checkpoint.node_voltages) {
            spectrum[0] = Complex64::new(voltage, 0.0);
        }
        for (spectrum, &current) in state
            .mna_branch_currents
            .iter_mut()
            .zip(&checkpoint.branch_currents)
        {
            spectrum[0] = Complex64::new(current, 0.0);
        }
        Ok(())
    }

    fn apply_dc_trial_update(
        state: &mut HbSolverState,
        checkpoint: &HbDcCheckpoint,
        delta: &[Value],
        alpha: Value,
    ) -> bool {
        let node_count = checkpoint.node_voltages.len();
        for (node, (&original, &update)) in checkpoint
            .node_voltages
            .iter()
            .zip(delta.iter().take(node_count))
            .enumerate()
        {
            let limited_step = (alpha * update).clamp(-0.5, 0.5);
            let voltage = (original + limited_step).clamp(-1000.0, 1000.0);
            if !voltage.is_finite() {
                return false;
            }
            state.x[node][0] = Complex64::new(voltage, 0.0);
        }
        for (branch, (&original, &update)) in checkpoint
            .branch_currents
            .iter()
            .zip(delta.iter().skip(node_count))
            .enumerate()
        {
            let current = original + alpha * update;
            if !current.is_finite() {
                return false;
            }
            state.mna_branch_currents[branch][0] = Complex64::new(current, 0.0);
        }
        true
    }

    /// Solve and certify a real linear system with scale-aware sparse LU.
    pub(super) fn solve_real_linear_system(
        &self,
        a: &[Vec<Value>],
        b: &[Value],
    ) -> Result<Vec<Value>, HbError> {
        let n = b.len();
        if n == 0 {
            return if a.is_empty() {
                Ok(Vec::new())
            } else {
                Err(HbError::InvalidCircuit(format!(
                    "HB real linear system has {} matrix rows but an empty RHS",
                    a.len()
                )))
            };
        }
        if a.len() != n {
            return Err(HbError::InvalidCircuit(format!(
                "HB real linear-system dimension mismatch: matrix has {} rows, RHS has {n}",
                a.len()
            )));
        }
        if b.iter().any(|value| !value.is_finite()) {
            return Err(HbError::InvalidCircuit(
                "HB real linear-system RHS contains a non-finite value".to_string(),
            ));
        }

        let mut triplets = Vec::with_capacity(n);
        for (row_index, row) in a.iter().enumerate() {
            if row.len() != n {
                return Err(HbError::InvalidCircuit(format!(
                    "HB real linear-system row {row_index} has {} columns; expected {n}",
                    row.len()
                )));
            }
            for (col_index, &value) in row.iter().enumerate() {
                if !value.is_finite() {
                    return Err(HbError::InvalidCircuit(format!(
                        "HB real linear-system coefficient ({row_index}, {col_index}) is non-finite"
                    )));
                }
                if row_index == col_index || value != 0.0 {
                    triplets.push((row_index, col_index, value));
                }
            }
        }

        let mut matrix =
            StaticMatrix::from_triplets(n, n, &triplets).map_err(Self::map_linear_solve_error)?;
        match matrix.solve(b) {
            Ok(solution) => Ok(solution),
            Err(SolverError::InaccurateSolution(_)) if n <= 64 => matrix
                .solve_dense_extended(b)
                .map_err(Self::map_linear_solve_error),
            Err(error) => Err(Self::map_linear_solve_error(error)),
        }
    }

    /// Extract DC solution as vector of real voltages
    fn extract_dc_solution(&self, state: &HbSolverState) -> Vec<Value> {
        (0..self.num_nodes)
            .map(|node| {
                state
                    .x
                    .get(node)
                    .and_then(|x| x.first())
                    .map(|c| c.re)
                    .unwrap_or(0.0)
            })
            .collect()
    }

    /// Initialize node voltages for diode circuits
    /// Propagates ~0.6V per diode through the chain to help Newton converge
    fn initialize_diode_voltages(&self, state: &mut HbSolverState) {
        let n = self.num_nodes;

        // Find grounded nodes (nodes with large conductance to ground)
        let mut ground_conductance = vec![0.0; n];
        for &(row, col, g) in &self.g_matrix {
            if row == col && row < n {
                ground_conductance[row] += g;
            }
        }

        // Find the most grounded node as reference
        let reference_node = ground_conductance
            .iter()
            .enumerate()
            .max_by(|a, b| {
                let a_val = if a.1.is_finite() {
                    *a.1
                } else {
                    f64::NEG_INFINITY
                };
                let b_val = if b.1.is_finite() {
                    *b.1
                } else {
                    f64::NEG_INFINITY
                };
                a_val.total_cmp(&b_val)
            })
            .map(|(i, _)| i)
            .unwrap_or(n - 1);

        // Build diode adjacency: for each node, track connected diodes with polarity
        // (neighbor, is_this_node_anode) - True if current node is anode of diode to neighbor
        // Also include BJT B-E junctions as they behave like diodes
        let mut node_diodes: Vec<Vec<(usize, bool)>> = vec![vec![]; n];
        for device in &self.nonlinear_devices {
            match device.device_type {
                NonlinearDeviceType::Diode => {
                    let anode = device.terminals[0];
                    let cathode = device.terminals[1];
                    if anode < n && cathode < n {
                        node_diodes[anode].push((cathode, true)); // anode connects to cathode
                        node_diodes[cathode].push((anode, false)); // cathode connects to anode
                    }
                }
                NonlinearDeviceType::NpnBjt => {
                    // NPN: B-E junction is like diode with base as anode, emitter as cathode
                    let base = device.terminals[1];
                    let emitter = device.terminals[2];
                    if base < n && emitter < n {
                        node_diodes[base].push((emitter, true));
                        node_diodes[emitter].push((base, false));
                    }
                }
                NonlinearDeviceType::PnpBjt => {
                    // PNP: E-B junction is like diode with emitter as anode, base as cathode
                    let base = device.terminals[1];
                    let emitter = device.terminals[2];
                    if base < n && emitter < n {
                        node_diodes[emitter].push((base, true));
                        node_diodes[base].push((emitter, false));
                    }
                }
                _ => {} // MOSFETs don't have junction diodes for DC init
            }
        }

        // First pass: Set source nodes to estimated voltage based on diode distance to ground
        // and set reference node to 0V
        let mut node_voltage = vec![f64::NAN; n];
        node_voltage[reference_node] = 0.0;

        for node in 0..n {
            let source_current = self
                .source_spectra
                .get(node)
                .and_then(|s| s.first())
                .map(|c| c.re)
                .unwrap_or(0.0);

            // Get self-conductance to ground for this node (Norton equivalent)
            let self_conductance: f64 = self
                .g_matrix
                .iter()
                .filter(|&&(r, c, _)| r == node && c == node)
                .map(|&(_, _, g)| g)
                .sum();

            // Check if this node has diode connections (should use diode init instead)
            let has_diode_connection = node_diodes
                .get(node)
                .map(|v| !v.is_empty())
                .unwrap_or(false);

            if source_current > 0.0 && self_conductance > 0.1 && !has_diode_connection {
                // Use Norton equivalent: V = I/G for supply nodes without diode connections
                // Only apply for positive supplies with significant conductance (>0.1S)
                // This is critical for MOSFET circuits where supply rails must be correct
                let norton_v = source_current / self_conductance;
                if norton_v > 0.5 {
                    // Only use for supplies > 0.5V to avoid overriding near-ground estimates
                    node_voltage[node] = norton_v;
                }
            } else if source_current > 0.0 {
                // Positive current: estimate based on diode chain
                let diode_count = self.count_diodes_from_node(node, reference_node);
                node_voltage[node] = (diode_count as f64 * 0.6).max(0.1);
            } else if source_current < 0.0 {
                // Negative current: reverse bias
                node_voltage[node] = -0.1;
            }
        }

        // Second pass: BFS from known nodes to propagate through diode chain
        let mut queue = std::collections::VecDeque::new();
        for node in 0..n {
            if !node_voltage[node].is_nan() {
                queue.push_back(node);
            }
        }

        while let Some(current) = queue.pop_front() {
            let current_v = node_voltage[current];

            for &(neighbor, is_anode) in &node_diodes[current] {
                if node_voltage[neighbor].is_nan() {
                    // Propagate voltage through diode
                    // If current is anode, neighbor (cathode) is ~0.6V lower
                    // If current is cathode, neighbor (anode) is ~0.6V higher
                    let neighbor_v = if is_anode {
                        current_v - 0.6 // current is anode, neighbor is cathode
                    } else {
                        current_v + 0.6 // current is cathode, neighbor is anode
                    };
                    node_voltage[neighbor] = neighbor_v;
                    queue.push_back(neighbor);
                }
            }
        }

        // Apply voltages to state, using small default for any unvisited nodes
        for node in 0..n {
            if node < state.x.len() && !state.x[node].is_empty() {
                let v = if node_voltage[node].is_nan() {
                    0.1 // Default for unconnected nodes
                } else {
                    node_voltage[node]
                };
                state.x[node][0] = Complex64::new(v, 0.0);
            }
        }
    }

    /// Count diodes in path from node to reference (simple heuristic)
    fn count_diodes_from_node(&self, from: usize, to: usize) -> usize {
        let n = self.num_nodes;
        if from >= n || to >= n {
            return 0;
        }

        // Build adjacency for diodes
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        for device in &self.nonlinear_devices {
            if device.device_type == NonlinearDeviceType::Diode {
                let a = device.terminals[0];
                let c = device.terminals[1];
                if a < n && c < n {
                    adj[a].push(c);
                    adj[c].push(a);
                }
            }
        }

        // BFS to find shortest path through diodes
        let mut visited = vec![false; n];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((from, 0usize));
        visited[from] = true;

        while let Some((current, dist)) = queue.pop_front() {
            if current == to {
                return dist;
            }
            for &neighbor in &adj[current] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }

        0 // No path found
    }

    // =========================================================================
    // End DC Operating Point Solver
    // =========================================================================
}

#[cfg(test)]
mod linear_solve_tests {
    use super::*;
    use crate::analysis::harmonic_balance::HbConfig;

    fn solver() -> HbSolver {
        HbSolver::new(HbConfig::new(1.0e9), 1)
    }

    #[test]
    fn real_solve_rejects_singular_systems() {
        assert!(matches!(
            solver().solve_real_linear_system(&[vec![0.0]], &[1.0]),
            Err(HbError::SingularMatrix)
        ));
    }

    #[test]
    fn real_solve_preserves_tiny_physical_coefficients() {
        let solution = solver()
            .solve_real_linear_system(&[vec![1.0e-18]], &[1.0])
            .expect("1e18-ohm scalar system is nonsingular");
        assert!((solution[0] - 1.0e18).abs() <= 1.0e3);
    }

    #[test]
    fn linear_dc_seed_contains_no_implicit_shunt() {
        let mut solver = solver();
        solver.add_resistor(0, 1, 1.0e18);
        solver.add_dc_source(0, 1.0);
        let mut state = HbSolverState::new(1, solver.num_harmonics());

        let solution = solver
            .solve_dc_operating_point(&mut state)
            .expect("physical high-impedance DC seed solves");
        assert!((solution[0] - 1.0e18).abs() <= 1.0e3);
    }

    #[test]
    fn dc_seed_rejects_invalid_caller_tolerances() {
        let mut config = HbConfig::new(1.0e9);
        config.tolerance = f64::NAN;
        let mut solver = HbSolver::new(config, 1);
        let mut state = HbSolverState::new(1, solver.num_harmonics());

        assert!(matches!(
            solver.solve_dc_operating_point(&mut state),
            Err(HbError::InvalidCircuit(_))
        ));
    }

    #[test]
    fn exact_mna_dc_seed_solves_voltage_source_and_inductor_branch_unknowns() {
        let mut solver = HbSolver::new(HbConfig::new(1.0e3).with_harmonics(1), 2);
        solver.add_resistor(1, 2, 1.0e3);
        // A negligible nonlinear branch forces the production DC Newton path
        // without materially changing the simple one-milliamp oracle.
        solver.add_diode(1, 2, 1.0e-30, 1.0);
        let source_index = solver
            .try_add_named_voltage_source_branch_harmonics(1, 0, 1.0, &[], "V1")
            .expect("exact source descriptor");
        solver
            .try_add_periodic_voltage_source_branch(1, 0, source_index, 1, "V1")
            .expect("canonical source branch");
        solver
            .try_add_periodic_inductor_branch(1, 2, 1.0e-3, 2, "L1")
            .expect("canonical inductor branch");
        let mut state = HbSolverState::new(2, solver.num_harmonics());

        let voltages = solver
            .solve_dc_operating_point(&mut state)
            .expect("branch-aware nonlinear DC seed converges");

        assert!((voltages[0] - 1.0).abs() <= 1.0e-10);
        assert!((voltages[1] - 1.0).abs() <= 1.0e-10);
        assert_eq!(state.mna_branch_currents.len(), 2);
        let source_current = state.mna_branch_currents[0][0];
        let inductor_current = state.mna_branch_currents[1][0];
        assert_eq!(source_current.im, 0.0);
        assert_eq!(inductor_current.im, 0.0);
        assert!((source_current.re + inductor_current.re).abs() <= 1.0e-10);
        assert!((inductor_current.re - 1.0e-3).abs() <= 1.0e-9);
        assert!(state.dc_rows_converged_with_branch_tolerances(
            solver.config.tolerance,
            solver.config.abstol,
            crate::constants::VNTOL,
        ));
    }

    #[test]
    fn dc_source_homotopy_scales_exact_voltage_kvl_without_scaling_branch_current() {
        let mut solver = HbSolver::new(HbConfig::new(1.0e3).with_harmonics(1), 1);
        let source_index = solver
            .try_add_named_voltage_source_branch_harmonics(1, 0, 2.0, &[], "V1")
            .expect("exact source descriptor");
        solver
            .try_add_periodic_voltage_source_branch(1, 0, source_index, 1, "V1")
            .expect("canonical source branch");
        let mut state = HbSolverState::new(1, solver.num_harmonics());
        state
            .try_prepare_mna_branches(1, solver.num_harmonics())
            .expect("branch workspaces");

        solver
            .compute_dc_residual(&mut state, 0.0, 0.25)
            .expect("quarter-strength exact DC residual");

        assert_eq!(state.mna_branch_residual[0][0], Complex64::new(0.5, 0.0));
        assert_eq!(state.mna_branch_currents[0][0], Complex64::ZERO);
        assert_eq!(state.residual[0][0], Complex64::ZERO);
    }

    #[test]
    fn dc_seed_rejects_uncanonicalized_public_voltage_source_branches() {
        let mut solver = HbSolver::new(HbConfig::new(1.0e3).with_harmonics(1), 1);
        solver.add_voltage_source_branch(1, 0, 1.0);
        solver.add_diode(0, 1, 1.0e-30, 1.0);
        let mut state = HbSolverState::new(1, solver.num_harmonics());

        let error = solver
            .solve_dc_operating_point(&mut state)
            .expect_err("a partial legacy branch registry must fail closed");
        assert!(error.to_string().contains("canonical exact-MNA"), "{error}");
    }

    #[test]
    fn real_solve_rejects_malformed_and_nonfinite_input() {
        assert!(matches!(
            solver().solve_real_linear_system(&[], &[1.0]),
            Err(HbError::InvalidCircuit(_))
        ));
        assert!(matches!(
            solver().solve_real_linear_system(&[vec![f64::NAN]], &[1.0]),
            Err(HbError::InvalidCircuit(_))
        ));
        assert!(matches!(
            solver().solve_real_linear_system(&[vec![1.0]], &[f64::INFINITY]),
            Err(HbError::InvalidCircuit(_))
        ));
    }
}
