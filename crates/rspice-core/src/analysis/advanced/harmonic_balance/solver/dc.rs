//! DC operating-point solve used to seed harmonic-balance Newton iteration.

use super::*;
use crate::solver::convergence::SourceStepper;

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
        // DC tolerances (more realistic than HB defaults)
        // For DC analysis, we're solving KCL: sum of currents = 0
        // Typical circuit currents are in mA-ÂµA range, so abstol should be ~pA
        let dc_reltol = self.config.tolerance.max(1e-3); // At least 0.1% relative
        let dc_abstol = self.config.abstol.max(1e-9); // At least 1 pA absolute

        // DC-specific iteration limit
        let dc_max_iter = self.config.max_iterations.max(150);

        // For linear circuits, DC is just a linear solve at k=0
        if !self.has_nonlinear_devices() {
            self.solve_dc_linear(state)?;
            return Ok(self.extract_dc_solution(state));
        }

        // Target GMIN for final solution
        let target_gmin = 1e-12;

        // Initialize diode voltages with forward bias estimate (0.6V per diode)
        // This gives Newton a much better starting point than V=0
        self.initialize_diode_voltages(state);

        // Step 1: Try direct DC Newton with minimal GMIN
        if self.dc_newton_inner_loop(state, target_gmin, dc_max_iter, dc_reltol, dc_abstol) {
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
            ) {
                // Converged at this GMIN level - refine to target
                let mut current_gmin = gmin_level;
                let mut last_good_x = self.extract_dc_solution(state);
                let mut last_good_gmin = current_gmin;
                let mut refine_failures = 0;

                while current_gmin > target_gmin {
                    current_gmin /= 2.0;
                    if self.dc_newton_inner_loop(
                        state,
                        current_gmin,
                        dc_max_iter,
                        dc_reltol,
                        dc_abstol,
                    ) {
                        last_good_x = self.extract_dc_solution(state);
                        last_good_gmin = current_gmin;
                        refine_failures = 0; // Reset failure count on success
                    } else {
                        refine_failures += 1;
                        // Restore last good state and keep trying with smaller steps
                        for (node, &v) in last_good_x.iter().enumerate() {
                            if node < state.x.len() && !state.x[node].is_empty() {
                                state.x[node][0] = Complex64::new(v, 0.0);
                            }
                        }
                        // After too many consecutive failures, try slower reduction
                        if refine_failures > 3 {
                            break;
                        }
                        // Try 10% reduction instead of 50%
                        current_gmin = last_good_gmin * 0.9;
                    }
                }

                // Verify final residual at the best achievable GMIN
                self.compute_dc_residual(state, last_good_gmin.max(target_gmin));
                let rel_norm = state.residual_norm / (self.dc_solution_norm(state) + dc_abstol);
                if state.residual_norm < dc_abstol || rel_norm < dc_reltol {
                    return Ok(self.extract_dc_solution(state));
                }
            }
        }

        // Step 3: Source stepping - ramp DC sources from 0 to full
        let original_sources = self.source_spectra.clone();

        // Reset DC to zero
        for node in 0..self.num_nodes {
            if node < state.x.len() && !state.x[node].is_empty() {
                state.x[node][0] = Complex64::new(0.0, 0.0);
            }
        }

        // Use SourceStepper for DC sources
        let mut source_stepper = SourceStepper::new();
        let max_steps = 50;
        let mut step_count = 0;

        while !source_stepper.is_complete() && step_count < max_steps {
            let factor = source_stepper.factor();
            step_count += 1;

            // Scale DC sources only (harmonic 0)
            for node in 0..self.num_nodes.min(self.source_spectra.len()) {
                if !self.source_spectra[node].is_empty() {
                    self.source_spectra[node][0] = original_sources
                        .get(node)
                        .and_then(|s| s.first())
                        .copied()
                        .unwrap_or(Complex64::ZERO)
                        * factor;
                }
            }

            if self.dc_newton_inner_loop(
                state,
                1e-6,
                dc_max_iter / 2,
                dc_reltol * 10.0,
                dc_abstol * 10.0,
            ) {
                source_stepper.advance_on_success();
            } else if !source_stepper.reduce_on_failure() {
                break;
            }
        }

        // Restore original sources
        self.source_spectra = original_sources;

        // Final DC solve with full sources
        if source_stepper.is_complete()
            && self.dc_newton_inner_loop(state, target_gmin, dc_max_iter, dc_reltol, dc_abstol)
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
        // Build DC conductance matrix (G plus inductor DC shorts)
        let n = self.num_nodes;
        let mut g_dc = vec![vec![0.0; n]; n];

        // Add conductances
        for &(row, col, g) in &self.g_matrix {
            if row < n && col < n {
                g_dc[row][col] += g;
            }
        }

        // Inductors are DC shorts, with the same conductance the
        // full-spectrum residual uses; leaving them open would seed Newton
        // with the operating point of a different circuit.
        for &(row, col, l) in &self.l_matrix {
            if row < n && col < n && l.abs() > 1e-30 {
                g_dc[row][col] += DC_SHORT_CONDUCTANCE;
            }
        }

        // Add small GMIN on diagonal for invertibility
        let gmin = 1e-12;
        for i in 0..n {
            g_dc[i][i] += gmin;
        }

        // Build DC RHS (source currents at DC)
        let rhs: Vec<Value> = (0..n)
            .map(|node| {
                self.source_spectra
                    .get(node)
                    .and_then(|s| s.first())
                    .map(|c| c.re)
                    .unwrap_or(0.0)
            })
            .collect();

        // Solve G * V = I
        let solution = self.solve_real_linear_system(&g_dc, &rhs)?;

        // Store DC solution
        for (node, &v) in solution.iter().enumerate() {
            if node < state.x.len() && !state.x[node].is_empty() {
                state.x[node][0] = Complex64::new(v, 0.0);
            }
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
    ) -> bool {
        for iteration in 0..max_iterations {
            state.iteration = iteration;

            // Compute DC residual
            self.compute_dc_residual(state, gmin);

            // Check convergence using standard criteria:
            // - Absolute: residual < abstol (current tolerance)
            // - Relative: residual < reltol * max_current_in_circuit
            // The relative tolerance should be against CURRENT, not voltage
            let max_source_current: f64 = self
                .source_spectra
                .iter()
                .filter_map(|s| s.first())
                .map(|c| c.re.abs())
                .fold(0.0, |a: f64, b| a.max(b));

            // Scale for relative convergence: max of source currents or minimum threshold
            let current_scale = max_source_current.max(abstol);
            let rel_current_norm = state.residual_norm / current_scale;

            // Converge if:
            // 1. Absolute residual is tiny (< 1e-9 A), OR
            // 2. Both: relative residual is small (<1%) AND absolute residual is reasonable (<1mA)
            // This prevents declaring convergence with large absolute current imbalance
            let converged = state.residual_norm < abstol
                || (rel_current_norm < tol && state.residual_norm < 1e-3);
            if converged {
                return true;
            }

            // Build DC Jacobian
            let jacobian = self.build_dc_jacobian(state, gmin);

            // Solve for delta_x: J * delta = -residual (standard Newton-Raphson)
            // We need -R because: R(x) = 0, Taylor: R(x+delta) â‰ˆ R(x) + J*delta = 0
            // So J*delta = -R
            let neg_residual: Vec<Value> = (0..self.num_nodes)
                .map(|node| {
                    -state
                        .residual
                        .get(node)
                        .and_then(|r| r.first())
                        .map(|c| c.re)
                        .unwrap_or(0.0)
                })
                .collect();

            let delta_x = match self.solve_real_linear_system(&jacobian, &neg_residual) {
                Ok(d) => d,
                Err(_) => return false, // Singular Jacobian
            };

            // Line search with DC voltage limiting
            self.apply_dc_line_search(state, &delta_x, gmin, tol);
        }

        false
    }

    /// Compute DC residual: R = I_source_dc - G*V_dc - I_nonlinear(V_dc) - gmin*V_dc
    fn compute_dc_residual(&mut self, state: &mut HbSolverState, gmin: Value) {
        let n = self.num_nodes;

        // Initialize residual with DC sources
        for node in 0..n {
            if node < state.residual.len() && !state.residual[node].is_empty() {
                let source = self
                    .source_spectra
                    .get(node)
                    .and_then(|s| s.first())
                    .map(|c| c.re)
                    .unwrap_or(0.0);
                state.residual[node][0] = Complex64::new(source, 0.0);
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
            }
        }

        // Inductor DC shorts, consistent with the full-spectrum residual.
        for &(row, col, l) in &self.l_matrix {
            if row < n && col < n && row < state.residual.len() && l.abs() > 1e-30 {
                state.residual[row][0] -=
                    Complex64::new(DC_SHORT_CONDUCTANCE * v_dc[col], 0.0);
            }
        }

        // Subtract GMIN: gmin * V_dc (diagonal)
        for node in 0..n {
            if node < state.residual.len() && !state.residual[node].is_empty() {
                state.residual[node][0] -= Complex64::new(gmin * v_dc[node], 0.0);
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
                }
            }
        }

        // Verilog-A devices participate in the DC seed too; skipping them
        // here let the seed "converge" on a circuit without their currents.
        #[cfg(feature = "veriloga")]
        if !self.veriloga_nonlinear_devices.is_empty() {
            for device in &mut self.veriloga_nonlinear_devices {
                device.device.update_all_voltages(&v_dc);
                let values = device.device.evaluate();
                for (program_idx, value) in values.iter().enumerate() {
                    let Some(rows) = device.rhs_rows.get(program_idx) else {
                        continue;
                    };
                    for &(row, sign) in rows {
                        if row < n && row < state.residual.len() {
                            state.residual[row][0] += Complex64::new(sign * *value, 0.0);
                        }
                    }
                }
            }
        }

        // Compute residual norm (DC only)
        let norm_sq: Value = (0..n)
            .map(|node| {
                state
                    .residual
                    .get(node)
                    .and_then(|r| r.first())
                    .map(|c| c.re * c.re)
                    .unwrap_or(0.0)
            })
            .sum();
        state.residual_norm = norm_sq.sqrt();
    }

    /// Build DC Jacobian: J = -G - dI_nonlinear/dV - gmin*I
    fn build_dc_jacobian(&mut self, state: &HbSolverState, gmin: Value) -> Vec<Vec<Value>> {
        let n = self.num_nodes;
        let mut jacobian = vec![vec![0.0; n]; n];

        // Linear contribution: -G
        for &(row, col, g) in &self.g_matrix {
            if row < n && col < n {
                jacobian[row][col] -= g;
            }
        }

        // Inductor DC shorts, matching compute_dc_residual.
        for &(row, col, l) in &self.l_matrix {
            if row < n && col < n && l.abs() > 1e-30 {
                jacobian[row][col] -= DC_SHORT_CONDUCTANCE;
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
        // But dI_into/dV = -gd (more voltage â†’ more current leaving â†’ less current into)
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
                let jac_entries = device.device.compute_jacobian();
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

        jacobian
    }

    /// Apply DC line search with voltage limiting
    fn apply_dc_line_search(
        &mut self,
        state: &mut HbSolverState,
        delta_x: &[Value],
        gmin: Value,
        _tol: Value,
    ) {
        let n = self.num_nodes;
        let mut alpha = 1.0;
        let min_alpha = 0.001;
        let armijo_c = 1e-4;

        // Save original DC voltages and residual
        let orig_x: Vec<Value> = (0..n)
            .map(|node| {
                state
                    .x
                    .get(node)
                    .and_then(|x| x.first())
                    .map(|c| c.re)
                    .unwrap_or(0.0)
            })
            .collect();
        let orig_residual = state.residual_norm;

        // Compute expected improvement
        let grad_dot_delta: Value = (0..n)
            .map(|i| {
                let r = state
                    .residual
                    .get(i)
                    .and_then(|r| r.first())
                    .map(|c| c.re)
                    .unwrap_or(0.0);
                r * delta_x.get(i).copied().unwrap_or(0.0)
            })
            .sum();

        let mut best_alpha = alpha;
        let mut best_residual = f64::INFINITY;

        while alpha >= min_alpha {
            // Apply update with voltage limiting
            for node in 0..n {
                if node < state.x.len() && !state.x[node].is_empty() && node < delta_x.len() {
                    let mut new_v = orig_x[node] + alpha * delta_x[node];

                    // PN junction voltage limiting
                    // Limit voltage changes at PN junctions to prevent overflow
                    let max_delta_v = 0.5; // Maximum voltage change per iteration
                    if (new_v - orig_x[node]).abs() > max_delta_v {
                        new_v = orig_x[node] + max_delta_v * (new_v - orig_x[node]).signum();
                    }

                    // Clamp to reasonable range
                    new_v = new_v.clamp(-1000.0, 1000.0);

                    state.x[node][0] = Complex64::new(new_v, 0.0);
                }
            }

            // Compute new residual
            self.compute_dc_residual(state, gmin);

            // Track best result
            if state.residual_norm < best_residual {
                best_residual = state.residual_norm;
                best_alpha = alpha;
            }

            // Armijo condition
            if state.residual_norm <= orig_residual + armijo_c * alpha * grad_dot_delta {
                return; // Accepted step
            }

            alpha *= 0.5;
        }

        // Use best alpha found
        for node in 0..n {
            if node < state.x.len() && !state.x[node].is_empty() && node < delta_x.len() {
                let mut new_v = orig_x[node] + best_alpha * delta_x[node];
                new_v = new_v.clamp(-1000.0, 1000.0);
                state.x[node][0] = Complex64::new(new_v, 0.0);
            }
        }
        self.compute_dc_residual(state, gmin);
    }

    /// Solve real linear system using Gaussian elimination with partial pivoting
    pub(super) fn solve_real_linear_system(
        &self,
        a: &[Vec<Value>],
        b: &[Value],
    ) -> Result<Vec<Value>, HbError> {
        let n = a.len();
        if n == 0 || b.len() != n {
            return Err(HbError::InvalidCircuit("Invalid system size".to_string()));
        }

        // Create augmented matrix
        let mut aug: Vec<Vec<Value>> = a
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let mut new_row = row.clone();
                new_row.push(b[i]);
                new_row
            })
            .collect();

        // Forward elimination with partial pivoting
        for col in 0..n {
            // Find pivot
            let mut max_row = col;
            let mut max_val = aug[col][col].abs();
            for row in (col + 1)..n {
                if aug[row][col].abs() > max_val {
                    max_val = aug[row][col].abs();
                    max_row = row;
                }
            }

            if max_val < 1e-15 {
                return Err(HbError::SingularMatrix);
            }

            // Swap rows
            aug.swap(col, max_row);

            // Eliminate
            let pivot = aug[col][col];
            for row in (col + 1)..n {
                let factor = aug[row][col] / pivot;
                for j in col..=n {
                    aug[row][j] -= factor * aug[col][j];
                }
            }
        }

        // Back substitution
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = aug[i][n];
            for j in (i + 1)..n {
                sum -= aug[i][j] * x[j];
            }
            x[i] = sum / aug[i][i];
        }

        Ok(x)
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

    /// Compute DC solution norm
    fn dc_solution_norm(&self, state: &HbSolverState) -> Value {
        let sum_sq: Value = (0..self.num_nodes)
            .map(|node| {
                state
                    .x
                    .get(node)
                    .and_then(|x| x.first())
                    .map(|c| c.re * c.re)
                    .unwrap_or(0.0)
            })
            .sum();
        sum_sq.sqrt()
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
