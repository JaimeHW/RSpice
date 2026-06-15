//! Full nonlinear harmonic-balance Newton iteration helpers.

use super::*;
use crate::solver::convergence::{PseudoTransient, SourceStepper};
use crate::solver::limit_pn_voltage;
use std::f64::consts::PI;

impl HbSolver {
    /// Full Newton-Raphson iteration for nonlinear HB analysis
    ///
    /// Advanced implementation following standard methodology:
    /// 1. Try direct Newton-Raphson first
    /// 2. If that fails, use source stepping (ramp sources from 0 to full)
    /// 3. Use GMIN as a constant stabilizer throughout
    ///
    /// The Jacobian includes:
    /// - Linear part: block-diagonal Y = G + jωC + 1/(jωL) per harmonic
    /// - Nonlinear part: FFT-based convolution of time-domain Jacobians
    /// - GMIN: diagonal conductance for numerical stability
    pub fn solve_newton(&mut self, state: &mut HbSolverState) -> Result<(), HbError> {
        let tol = self.config.tolerance;
        let abstol = self.config.abstol;

        // For linear circuits, use direct solve
        if !self.has_nonlinear_devices() {
            return self.solve_linear(state);
        }

        // Target GMIN for the converged solution: the SPICE-standard 1e-12.
        // The stabilizer stays in the final residual, so anything larger
        // would leave a visible leak at high-impedance nodes; difficult
        // circuits get their conditioning help from the GMIN ladder below,
        // which always refines back to this target.
        let gmin = 1e-12;

        // Step 0: Solve DC operating point first
        // This establishes the nonlinear device operating points and provides a much
        // better initial guess than starting from zero or a random guess.
        if let Ok(_dc_solution) = self.solve_dc_operating_point(state) {
            // DC solution is now stored in state.x[node][0]
            // Initialize harmonic components to zero (small-signal around DC)
            // This is the standard HB initialization approach
            for node in 0..self.num_nodes {
                if node < state.x.len() {
                    for k in 1..state.x[node].len() {
                        // Keep harmonics at zero - full Newton will find them
                        state.x[node][k] = Complex64::new(0.0, 0.0);
                    }
                }
            }
        }
        // If DC solve fails, continue with existing fallback strategy
        // The full Newton may still succeed with source stepping

        // Step 1: Try direct Newton first
        if self.newton_inner_loop(state, gmin, self.config.max_iterations, tol, abstol) {
            state.converged = true;
            return Ok(());
        }

        // Step 2: If direct Newton fails, try with progressively larger GMIN
        // This helps regularize ill-conditioned Jacobians
        // Include high GMIN levels (0.1, 1.0) for very difficult circuits
        for gmin_level in [1e-6, 1e-4, 1e-2, 0.1, 1.0] {
            if self.newton_inner_loop(
                state,
                gmin_level,
                self.config.max_iterations,
                tol * 10.0,
                abstol,
            ) {
                // Converged at higher GMIN - now refine with progressively lower GMIN
                // Save state before refinement in case we need to restore
                let mut last_good_state = state.x.clone();
                let mut last_good_residual = state.residual_norm;

                let mut current_gmin = gmin_level;
                while current_gmin > gmin {
                    // Use factor of 2 for very gradual refinement
                    current_gmin /= 2.0;
                    if self.newton_inner_loop(
                        state,
                        current_gmin,
                        self.config.max_iterations,
                        tol,
                        abstol,
                    ) {
                        // Success - update last good state
                        last_good_state = state.x.clone();
                        last_good_residual = state.residual_norm;
                    } else {
                        // Failed - restore last good state and stop refining
                        state.x = last_good_state;
                        state.residual_norm = last_good_residual;
                        break;
                    }
                }
                // Recompute residual with target GMIN to check tolerance
                self.compute_full_residual_with_gmin(state, gmin);
                if state.residual_norm < abstol || state.rows_converged(tol, abstol) {
                    state.converged = true;
                    return Ok(());
                }
            }
        }

        // Step 3: Try source stepping
        // Scale sources from 0 to full, using previous converged solution as starting point
        let original_sources = self.source_spectra.clone();
        let mut source_stepper = SourceStepper::new();
        let mut total_iterations = 0; // Reset for source stepping
        let max_total_iter = self.config.max_iterations * 20;

        // Reset state to zero - with sources=0, solution=0 trivially
        for node in 0..self.num_nodes {
            if node < state.x.len() {
                for k in 0..state.x[node].len() {
                    state.x[node][k] = Complex64::new(0.0, 0.0);
                }
            }
        }

        while !source_stepper.is_complete() && total_iterations < max_total_iter {
            let factor = source_stepper.factor();

            // Scale sources by current factor
            for node in 0..self.num_nodes {
                if node < self.source_spectra.len() {
                    for k in 0..self.source_spectra[node].len() {
                        self.source_spectra[node][k] = original_sources
                            .get(node)
                            .and_then(|s| s.get(k))
                            .copied()
                            .unwrap_or(Complex64::new(0.0, 0.0))
                            * factor;
                    }
                }
            }

            // Don't reset state.x - keep converged solution from previous source level

            // Try Newton at this source level (using previous solution as starting point)
            let converged = self.newton_inner_loop(
                state,
                gmin,
                self.config.max_iterations / 2,
                tol * 10.0,
                abstol,
            );

            total_iterations += state.iteration;

            if converged {
                // Keep state.x as-is for next step (converged solution)
                source_stepper.advance_on_success();
            } else {
                if !source_stepper.reduce_on_failure() {
                    break;
                }
            }
        }

        // Restore original sources
        self.source_spectra = original_sources;

        // If source stepping completed, do final Newton with original sources
        if source_stepper.is_complete()
            && self.newton_inner_loop(state, gmin, self.config.max_iterations, tol, abstol)
        {
            state.converged = true;
            state.iteration = total_iterations;
            return Ok(());
        }

        // Step 4: Try pseudo-transient
        // Add damping capacitors to each node and integrate to steady-state
        let mut ptran = PseudoTransient::new();
        let mut ptran_iterations = 0;
        let max_ptran_iter = self.config.max_iterations * 5;

        while !ptran.is_complete() && ptran_iterations < max_ptran_iter {
            // Pseudo-transient adds G_eq = C_pseudo/dt to each node diagonal
            // This damps oscillations and helps find DC solution
            let ptran_gmin = gmin + ptran.conductance(0);

            let converged = self.newton_inner_loop(
                state,
                ptran_gmin,
                self.config.max_iterations / 4,
                tol * 100.0, // Relaxed tolerance during stepping
                abstol,
            );

            ptran_iterations += state.iteration;

            if converged {
                ptran.advance_on_success();
            } else {
                if !ptran.reduce_on_failure() {
                    break;
                }
            }
        }

        // If pseudo-transient completed, do final high-accuracy Newton
        if ptran.is_complete()
            && self.newton_inner_loop(state, gmin, self.config.max_iterations, tol, abstol)
        {
            state.converged = true;
            state.iteration = total_iterations + ptran_iterations;
            return Ok(());
        }

        Err(HbError::ConvergenceFailed {
            iterations: total_iterations + ptran_iterations,
            residual: state.residual_norm,
        })
    }

    /// Inner Newton iteration loop at a fixed GMIN level
    fn newton_inner_loop(
        &mut self,
        state: &mut HbSolverState,
        gmin: Value,
        max_iter: usize,
        tol: Value,
        abstol: Value,
    ) -> bool {
        for iter in 0..max_iter {
            state.iteration = iter;
            state.total_iterations += 1;

            // 1. Compute full residual: linear + nonlinear + GMIN contributions
            self.compute_full_residual_with_gmin(state, gmin);

            // 2. Check convergence: per-row KCL test. A global norm hides a
            // microamp imbalance at a high-impedance node behind the amp
            // scale of stiff source rows, accepting grossly wrong bias.
            if state.residual_norm < abstol || state.rows_converged(tol, abstol) {
                return true;
            }

            // 3+4. Build the Jacobian and solve J * dX = -R. The exact path
            // carries the conjugate (Hankel) coupling in a real-split system
            // and restores quadratic convergence; the Toeplitz-only complex
            // path remains selectable for A/B comparison and for the large-
            // system Krylov fast path.
            let delta_x = if self.config.use_exact_jacobian {
                match self.solve_jacobian_system_exact(state, gmin) {
                    Ok(dx) => dx,
                    Err(_) => return false, // Singular matrix
                }
            } else {
                let jacobian = self.build_full_jacobian_with_gmin(state, gmin);
                match self.solve_jacobian_system(&jacobian, state) {
                    Ok(dx) => dx,
                    Err(_) => return false, // Singular matrix
                }
            };

            // 5. Apply line search for robust convergence
            if self
                .apply_line_search_with_gmin(state, &delta_x, gmin)
                .is_err()
            {
                return false;
            }
        }

        false // Max iterations reached
    }

    /// Compute full residual including GMIN contribution
    ///
    /// Residual = I_source - Y*V - gmin*V - I_nonlinear
    /// (KCL: sum of currents INTO node = 0)
    fn compute_full_residual_with_gmin(&mut self, state: &mut HbSolverState, gmin: Value) {
        // Start with linear residual (I_source - Y*V)
        self.compute_linear_residual(state);

        // Subtract GMIN contribution: I_gmin = gmin * V (current leaves node via GMIN)
        for node in 0..self.num_nodes {
            for k in 0..=self.num_harmonics {
                if node < state.residual.len() && k < state.residual[node].len() {
                    state.residual[node][k] -= gmin * state.x[node][k];
                    state.residual_scale[node][k] += gmin * state.x[node][k].norm();
                }
            }
        }

        // Subtract nonlinear device currents (evaluated in time domain via FFT)
        // Note: add_nonlinear_residual adds currents with correct sign already
        if self.has_nonlinear_devices() {
            self.add_nonlinear_residual(state);
        }
    }

    /// Build Jacobian with GMIN on diagonal
    ///
    /// Residual = I_source - Y*V - gmin*V, so J = ∂res/∂V = -Y - gmin
    fn build_full_jacobian_with_gmin(
        &mut self,
        state: &HbSolverState,
        gmin: Value,
    ) -> Vec<Vec<Complex64>> {
        let mut jac = self.build_full_jacobian(state);

        // Subtract GMIN from all diagonal entries (consistent with residual -= gmin*V)
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;
        for i in 0..n {
            for k in 0..h {
                let idx = i * h + k;
                if idx < jac.len() {
                    jac[idx][idx] -= gmin;
                }
            }
        }

        jac
    }

    /// Apply line search with GMIN and PN voltage limiting
    ///
    /// Advanced implementation following standard methodology:
    /// - Armijo backtracking line search
    /// - PN junction voltage limiting on DC component
    fn apply_line_search_with_gmin(
        &mut self,
        state: &mut HbSolverState,
        delta_x: &[Vec<Complex64>],
        gmin: Value,
    ) -> Result<(), HbError> {
        let initial_norm = state.residual_norm;
        let armijo_c = 1e-4;
        let min_alpha = 0.01;
        let vt = 0.02585; // Thermal voltage at 300K

        let mut alpha = 1.0;
        let mut best_alpha = alpha;
        let mut best_norm = f64::INFINITY;

        let x_orig: Vec<Vec<Complex64>> = state.x.clone();

        while alpha >= min_alpha {
            for (node, dx_node) in delta_x.iter().enumerate() {
                for (k, &dx) in dx_node.iter().enumerate() {
                    if node < state.x.len() && k < state.x[node].len() {
                        let v_old = x_orig[node][k].re;
                        let v_new_raw = v_old + alpha * dx.re;

                        // Apply PN voltage limiting to DC component only
                        let v_new = if k == 0 {
                            limit_pn_voltage(v_old, v_new_raw, vt)
                        } else {
                            v_new_raw
                        };

                        // Keep imaginary part updated normally
                        let im_new = x_orig[node][k].im + alpha * dx.im;
                        state.x[node][k] = Complex64::new(v_new, im_new);
                    }
                }
            }

            self.compute_full_residual_with_gmin(state, gmin);

            if state.residual_norm < initial_norm * (1.0 - armijo_c * alpha) {
                return Ok(());
            }

            if state.residual_norm < best_norm {
                best_norm = state.residual_norm;
                best_alpha = alpha;
            }

            alpha *= 0.5;
        }

        // Use best step found with voltage limiting
        for (node, dx_node) in delta_x.iter().enumerate() {
            for (k, &dx) in dx_node.iter().enumerate() {
                if node < state.x.len() && k < state.x[node].len() {
                    let v_old = x_orig[node][k].re;
                    let v_new_raw = v_old + best_alpha * dx.re;

                    let v_new = if k == 0 {
                        limit_pn_voltage(v_old, v_new_raw, vt)
                    } else {
                        v_new_raw
                    };

                    let im_new = x_orig[node][k].im + best_alpha * dx.im;
                    state.x[node][k] = Complex64::new(v_new, im_new);
                }
            }
        }
        self.compute_full_residual_with_gmin(state, gmin);

        Ok(())
    }

    /// Compute full residual including linear and nonlinear contributions

    /// Add nonlinear device contributions to residual
    fn add_nonlinear_residual(&mut self, state: &mut HbSolverState) {
        let n_time = self.fft.size();

        // Convert spectral voltages to time domain
        let v_time: Vec<Vec<Value>> = (0..self.num_nodes)
            .map(|node| self.fft.to_time_domain(&state.x[node]))
            .collect();

        // Accumulate nonlinear currents at each time point
        let mut i_time = vec![vec![0.0; n_time]; self.num_nodes];

        if !self.nonlinear_devices.is_empty() {
            let mut node_voltages = vec![0.0; self.num_nodes];
            for t in 0..n_time {
                for node in 0..self.num_nodes {
                    node_voltages[node] = v_time[node][t];
                }
                for device in &self.nonlinear_devices {
                    for (node, current) in device.evaluate(&node_voltages) {
                        if node < i_time.len() {
                            i_time[node][t] += current;
                        }
                    }
                }
            }
        }

        #[cfg(feature = "veriloga")]
        if !self.veriloga_nonlinear_devices.is_empty() {
            let mut circuit_voltages = vec![0.0; self.num_nodes];
            for t in 0..n_time {
                for node in 0..self.num_nodes {
                    circuit_voltages[node] = v_time[node][t];
                }
                for device in &mut self.veriloga_nonlinear_devices {
                    device.device.update_all_voltages(&circuit_voltages);
                    let values = device.device.evaluate();
                    for (program_idx, value) in values.iter().enumerate() {
                        let Some(rows) = device.rhs_rows.get(program_idx) else {
                            continue;
                        };
                        for &(row, sign) in rows {
                            if row < self.num_nodes {
                                i_time[row][t] += sign * *value;
                            }
                        }
                    }
                }
            }
        }

        // Convert nonlinear currents to frequency domain and ADD to residual
        // Device returns stamped contribution (current INTO node, already with correct sign)
        for node in 0..self.num_nodes {
            let i_spectrum = self.fft.to_frequency_domain(&i_time[node]);
            for (k, &i_k) in i_spectrum.iter().enumerate() {
                if k <= self.num_harmonics && node < state.residual.len() {
                    state.residual[node][k] += i_k;
                    state.residual_scale[node][k] += i_k.norm();
                }
            }
        }

        // Charge storage: the capacitive current delivered into a node is
        // d/dt of the delivered charge, i.e. jw_k * Q_k per harmonic. The
        // charge waveform comes from the same time grid as the resistive
        // currents, so charge and current stay phase-consistent.
        if self
            .nonlinear_devices
            .iter()
            .any(|d| d.has_charge_storage())
        {
            let omega0 = 2.0 * PI * self.config.fundamental_freq;
            let mut q_time = vec![vec![0.0; n_time]; self.num_nodes];
            let mut node_voltages = vec![0.0; self.num_nodes];
            for t in 0..n_time {
                for node in 0..self.num_nodes {
                    node_voltages[node] = v_time[node][t];
                }
                for device in &self.nonlinear_devices {
                    for (node, charge) in device.charge(&node_voltages) {
                        if node < q_time.len() {
                            q_time[node][t] += charge;
                        }
                    }
                }
            }

            for node in 0..self.num_nodes {
                let q_spectrum = self.fft.to_frequency_domain(&q_time[node]);
                for (k, &q_k) in q_spectrum.iter().enumerate() {
                    if k <= self.num_harmonics && node < state.residual.len() {
                        let omega_k = (k as f64) * omega0;
                        state.residual[node][k] += Complex64::new(0.0, omega_k) * q_k;
                        state.residual_scale[node][k] += omega_k * q_k.norm();
                    }
                }
            }
        }

        state.compute_residual_norm();
    }

    /// Build full Jacobian matrix for Newton iteration
    ///
    /// Structure: block matrix [node_i, k][node_j, l] where:
    /// - Diagonal blocks (k == l): linear admittance + linearized nonlinear
    /// - Off-diagonal blocks: nonlinear coupling via FFT convolution
    ///
    /// For efficiency, we flatten to a single [n*h x n*h] complex matrix
    fn build_full_jacobian(&mut self, state: &HbSolverState) -> Vec<Vec<Complex64>> {
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;
        let size = n * h;
        let omega0 = 2.0 * PI * self.config.fundamental_freq;

        // Initialize Jacobian
        let mut jac = vec![vec![Complex64::new(0.0, 0.0); size]; size];

        // --- Linear part: block-diagonal per harmonic ---
        // Residual = I_source - Y*V, so J = ∂res/∂V = -Y
        for k in 0..h {
            let omega_k = (k as f64) * omega0;

            // Conductance contribution: -G (negative because residual = ... - G*V)
            for &(i, j, g) in &self.g_matrix {
                if i < n && j < n {
                    let row = i * h + k;
                    let col = j * h + k;
                    jac[row][col] -= g;
                }
            }

            // Capacitance contribution: -jωC
            for &(i, j, c) in &self.c_matrix {
                if i < n && j < n {
                    let row = i * h + k;
                    let col = j * h + k;
                    jac[row][col] -= Complex64::new(0.0, omega_k) * c;
                }
            }

            // Inductance contribution: -1/(jωL)
            for &(i, j, l) in &self.l_matrix {
                if i < n && j < n && l.abs() > 1e-30 {
                    let row = i * h + k;
                    let col = j * h + k;
                    if k == 0 {
                        // DC: short circuit
                        jac[row][col] -= DC_SHORT_CONDUCTANCE;
                    } else {
                        // AC: Y_L = -j/(ωL)
                        jac[row][col] -= Complex64::new(0.0, -1.0 / (omega_k * l));
                    }
                }
            }
        }

        // --- Nonlinear part: requires FFT-based evaluation ---
        if self.has_nonlinear_devices() {
            self.add_nonlinear_jacobian(&mut jac, state);
        }

        jac
    }

    /// Add nonlinear Jacobian contributions via FFT (Toeplitz/convolution)
    ///
    /// For nonlinear devices, the frequency-domain Jacobian is a Toeplitz matrix
    /// representing convolution: J[k,l] = G[k-l] where G is the DFT of g(t).
    ///
    /// This is the implementation that exactly matches the
    /// FFT-based residual computation, ensuring proper Newton convergence.
    fn add_nonlinear_jacobian(&mut self, jac: &mut [Vec<Complex64>], state: &HbSolverState) {
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;
        let n_time = self.fft.size();

        // Convert voltages to time domain
        let v_time: Vec<Vec<Value>> = (0..n)
            .map(|node| self.fft.to_time_domain(&state.x[node]))
            .collect();

        // Accumulate conductance stamps in time domain for each node pair
        let mut g_time = vec![vec![vec![0.0; n_time]; n]; n]; // [i][j][t]

        if !self.nonlinear_devices.is_empty() {
            let mut node_voltages = vec![0.0; n];
            for t in 0..n_time {
                for node in 0..n {
                    node_voltages[node] = v_time[node][t];
                }
                for device in &self.nonlinear_devices {
                    for ((i, j), g) in device.jacobian(&node_voltages) {
                        if i < n && j < n {
                            g_time[i][j][t] += g;
                        }
                    }
                }
            }
        }

        #[cfg(feature = "veriloga")]
        if !self.veriloga_nonlinear_devices.is_empty() {
            let mut circuit_voltages = vec![0.0; n];
            for t in 0..n_time {
                for node in 0..n {
                    circuit_voltages[node] = v_time[node][t];
                }
                for device in &mut self.veriloga_nonlinear_devices {
                    device.device.update_all_voltages(&circuit_voltages);
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
                            g_time[i][j][t] += entry.value;
                        }
                    }
                }
            }
        }

        // Convert each conductance waveform to frequency domain (Toeplitz row)
        // Then build the proper convolution Jacobian
        for i in 0..n {
            for j in 0..n {
                // Check if there's any significant conductance
                let max_g: Value = g_time[i][j].iter().fold(0.0, |a, &b| a.max(b.abs()));
                if max_g < 1e-30 {
                    continue;
                }

                // FFT the conductance waveform to get G[k] spectrum
                let g_spectrum = self.fft.to_frequency_domain(&g_time[i][j]);

                // Build Toeplitz block for this (i,j) node pair
                // J[i*h+k][j*h+l] = G[k-l] (with periodic extension for negative indices)
                for k in 0..h {
                    for l in 0..h {
                        let row = i * h + k;
                        let col = j * h + l;

                        // Compute index for G[k-l] with wrap-around
                        let diff = k as isize - l as isize;
                        let g_idx = if diff >= 0 {
                            diff as usize
                        } else {
                            // Negative index - use conjugate symmetry: G[-m] = G[m]*
                            // For real g(t), G[-m] = conj(G[m])
                            (-diff) as usize
                        };

                        if g_idx < g_spectrum.len() {
                            let g_val = if diff >= 0 {
                                g_spectrum[g_idx]
                            } else {
                                // Use conjugate for negative frequency
                                g_spectrum[g_idx].conj()
                            };
                            // SUBTRACT device Jacobian for KCL: residual = I_source - I_device
                            // So J = ∂res/∂V = -∂I_device/∂V = -gd
                            jac[row][col] -= g_val;
                        }
                    }
                }
            }
        }

        // Charge-storage coupling: the residual carries jw_k * Q_k, so its
        // derivative is jw_k * C[k-m] - the same Toeplitz structure as the
        // conductances with the ROW harmonic's frequency in front.
        if self
            .nonlinear_devices
            .iter()
            .any(|d| d.has_charge_storage())
        {
            let omega0 = 2.0 * PI * self.config.fundamental_freq;
            let mut c_time = vec![vec![vec![0.0; n_time]; n]; n];
            let mut node_voltages = vec![0.0; n];
            for t in 0..n_time {
                for node in 0..n {
                    node_voltages[node] = v_time[node][t];
                }
                for device in &self.nonlinear_devices {
                    for ((i, j), c) in device.charge_jacobian(&node_voltages) {
                        if i < n && j < n {
                            c_time[i][j][t] += c;
                        }
                    }
                }
            }

            for i in 0..n {
                for j in 0..n {
                    let max_c: Value = c_time[i][j].iter().fold(0.0, |a, &b| a.max(b.abs()));
                    if max_c < 1e-30 {
                        continue;
                    }

                    let c_spectrum = self.fft.to_frequency_domain(&c_time[i][j]);

                    for k in 0..h {
                        let omega_k = (k as f64) * omega0;
                        let jw = Complex64::new(0.0, omega_k);
                        for l in 0..h {
                            let row = i * h + k;
                            let col = j * h + l;

                            let diff = k as isize - l as isize;
                            let c_idx = diff.unsigned_abs();
                            if c_idx < c_spectrum.len() {
                                let c_val = if diff >= 0 {
                                    c_spectrum[c_idx]
                                } else {
                                    c_spectrum[c_idx].conj()
                                };
                                // Residual carries +jw_k*Q_k; J = d(res)/dV
                                // gets -(jw_k * dQ/dV) like the linear caps.
                                jac[row][col] -= jw * c_val;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Solve the Newton step with the EXACT Jacobian in real-split form.
    ///
    /// The one-sided residual depends on both `c_m` and `c_m*` (perturbing a
    /// coefficient perturbs its implied conjugate), so the exact derivative
    /// carries a Toeplitz part `T = dI_k/dc_m = -G[k-m]` AND a Hankel part
    /// `H = dI_k/dc_m* = -G[k+m]` (plus the matching `jw_k*C` charge terms).
    /// `H` is antilinear, so the system is assembled over real unknowns
    /// `[a_0, a_1, b_1, ...]` per node (`c_k = a_k + j b_k`, `b_0 = 0`):
    /// the Toeplitz block maps to `[[Re T, -Im T], [Im T, Re T]]` and the
    /// Hankel block to `[[Re H, Im H], [Im H, -Re H]]`; the DC row keeps only
    /// its real equation and the DC column only its real unknown. Hankel
    /// indices reach `k + m = 2H`, so the coupling spectra are sampled out to
    /// twice the solution's harmonic count (alias-capped by the FFT grid).
    fn solve_jacobian_system_exact(
        &mut self,
        state: &HbSolverState,
        gmin: Value,
    ) -> Result<Vec<Vec<Complex64>>, HbError> {
        let n = self.num_nodes;
        let h = self.num_harmonics + 1; // complex components per node
        let w = 2 * self.num_harmonics + 1; // real unknowns per node
        let size = n * w;
        let omega0 = 2.0 * PI * self.config.fundamental_freq;

        // Row/column index helpers in the real layout.
        let re_idx = |node: usize, k: usize| -> usize {
            if k == 0 {
                node * w
            } else {
                node * w + 2 * k - 1
            }
        };
        let im_idx = |node: usize, k: usize| -> usize { node * w + 2 * k };

        // Toeplitz part (linear + GMIN + nonlinear G and charge), expanded
        // from the existing complex assembly.
        let jac_c = self.build_full_jacobian_with_gmin(state, gmin);
        let mut a = vec![vec![0.0; size]; size];
        for i in 0..n {
            for k in 0..h {
                for j in 0..n {
                    for l in 0..h {
                        let t = jac_c[i * h + k][j * h + l];
                        if t.re == 0.0 && t.im == 0.0 {
                            continue;
                        }
                        let row_re = re_idx(i, k);
                        let col_re = re_idx(j, l);
                        a[row_re][col_re] += t.re;
                        if l > 0 {
                            a[row_re][im_idx(j, l)] += -t.im;
                        }
                        if k > 0 {
                            let row_im = im_idx(i, k);
                            a[row_im][col_re] += t.im;
                            if l > 0 {
                                a[row_im][im_idx(j, l)] += t.re;
                            }
                        }
                    }
                }
            }
        }

        // Hankel part: H = -(G[k+m]) and -(jw_k * C[k+m]), m >= 1.
        if self.has_nonlinear_devices() {
            let extended = 2 * self.num_harmonics;
            let g_spectra = self.conductance_spectra(state, extended);
            let c_spectra = self.capacitance_spectra(state, extended);

            let mut add_hankel = |i: usize, j: usize, k: usize, m: usize, hval: Complex64| {
                let row_re = re_idx(i, k);
                a[row_re][re_idx(j, m)] += hval.re;
                a[row_re][im_idx(j, m)] += hval.im;
                if k > 0 {
                    let row_im = im_idx(i, k);
                    a[row_im][re_idx(j, m)] += hval.im;
                    a[row_im][im_idx(j, m)] += -hval.re;
                }
            };

            for (i, j, spec) in &g_spectra {
                for k in 0..h {
                    for m in 1..h {
                        if let Some(&g) = spec.get(k + m) {
                            add_hankel(*i, *j, k, m, -g);
                        }
                    }
                }
            }
            for (i, j, spec) in &c_spectra {
                for k in 0..h {
                    let jw = Complex64::new(0.0, (k as f64) * omega0);
                    for m in 1..h {
                        if let Some(&c) = spec.get(k + m) {
                            add_hankel(*i, *j, k, m, -(jw * c));
                        }
                    }
                }
            }
        }

        // Realified RHS: -residual, DC keeps only its real equation.
        let mut rhs = vec![0.0; size];
        for node in 0..n {
            for k in 0..h {
                let r = state.residual[node][k];
                rhs[re_idx(node, k)] = -r.re;
                if k > 0 {
                    rhs[im_idx(node, k)] = -r.im;
                }
            }
        }

        let solution = self.solve_real_linear_system(&a, &rhs)?;

        let mut delta_x = vec![vec![Complex64::new(0.0, 0.0); h]; n];
        for node in 0..n {
            for k in 0..h {
                let re = solution[re_idx(node, k)];
                let im = if k > 0 {
                    solution[im_idx(node, k)]
                } else {
                    0.0
                };
                delta_x[node][k] = Complex64::new(re, im);
            }
        }

        Ok(delta_x)
    }

    /// Solve the Jacobian system: J * ΔX = -R
    ///
    /// Large systems use restarted GMRES with a per-harmonic block-Jacobi
    /// preconditioner (O(size²) per Krylov iteration instead of the dense
    /// solve's O(size³)); small systems and any Krylov stagnation take the
    /// exact dense elimination, so the Krylov path can change only speed,
    /// never convergence.
    ///
    /// Returns flattened delta_x vector that maps back to [node][harmonic].
    fn solve_jacobian_system(
        &self,
        jac: &[Vec<Complex64>],
        state: &HbSolverState,
    ) -> Result<Vec<Vec<Complex64>>, HbError> {
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;
        let size = n * h;

        // Flatten RHS (negative residual)
        let mut rhs = Vec::with_capacity(size);
        for node in 0..n {
            for k in 0..h {
                rhs.push(-state.residual[node][k]);
            }
        }

        let try_krylov = self.config.use_krylov || size >= super::krylov::KRYLOV_AUTO_THRESHOLD;
        let flat_solution = if try_krylov && n > 0 && h > 0 {
            match self.solve_jacobian_krylov(jac, &rhs, n, h) {
                Some(solution) => solution,
                None => self.solve_complex_linear_system(jac, &rhs)?,
            }
        } else {
            self.solve_complex_linear_system(jac, &rhs)?
        };

        // Reshape to [node][harmonic]
        let mut delta_x = vec![vec![Complex64::new(0.0, 0.0); h]; n];
        for (idx, &val) in flat_solution.iter().enumerate() {
            let node = idx / h;
            let harmonic = idx % h;
            if node < n {
                delta_x[node][harmonic] = val;
            }
        }

        Ok(delta_x)
    }

    /// Attempt the Newton-step solve via block-Jacobi-preconditioned GMRES.
    ///
    /// Returns `None` when GMRES stagnates or fails to reach the inner
    /// tolerance — the caller then takes the exact dense path.
    fn solve_jacobian_krylov(
        &self,
        jac: &[Vec<Complex64>],
        rhs: &[Complex64],
        num_nodes: usize,
        num_components: usize,
    ) -> Option<Vec<Complex64>> {
        use super::krylov::{BlockJacobiPreconditioner, gmres};

        let preconditioner = BlockJacobiPreconditioner::build(jac, num_nodes, num_components);
        let matvec = |x: &[Complex64]| -> Vec<Complex64> {
            jac.iter()
                .map(|row| row.iter().zip(x).map(|(m, v)| m * v).sum())
                .collect()
        };

        let restart = self.config.gmres_restart.clamp(8, rhs.len().max(8));
        let outcome = gmres(&matvec, &preconditioner, rhs, restart, 4);

        if outcome.converged {
            if self.config.verbose {
                log::debug!(
                    "HB Krylov solve: {} iterations, relative residual {:.2e}",
                    outcome.iterations,
                    outcome.relative_residual
                );
            }
            Some(outcome.solution)
        } else {
            log::debug!(
                "HB Krylov solve stagnated after {} iterations (relative residual {:.2e}); \
                 falling back to dense elimination",
                outcome.iterations,
                outcome.relative_residual
            );
            None
        }
    }

    /// Apply Armijo line search for robust convergence
    ///
    /// Starts with α = 1 (full Newton step), reduces if residual doesn't decrease.
    /// This is critical for convergence on highly nonlinear circuits.

    /// Legacy newton_step for backward compatibility
    pub fn newton_step(
        &mut self,
        state: &mut HbSolverState,
        nonlinear_fn: impl Fn(&[Value]) -> (Value, Value), // (current, dI/dV)
        terminals: &[(usize, usize)],                      // (positive_node, negative_node)
    ) -> Result<(), HbError> {
        // Get time points for nonlinear evaluation
        let n_time = self.fft.size();
        let _period = self.config.period();

        // Convert spectral voltages to time domain for each node
        let mut v_time: Vec<Vec<Value>> = Vec::with_capacity(self.num_nodes);
        for node in 0..self.num_nodes {
            let spectrum = &state.x[node];
            let waveform = self.fft.to_time_domain(spectrum);
            v_time.push(waveform);
        }

        // Evaluate nonlinear elements in time domain
        let mut i_time = vec![vec![0.0; n_time]; self.num_nodes];
        let mut g_time = vec![vec![0.0; n_time]; self.num_nodes];

        for &(np, nn) in terminals {
            for t in 0..n_time {
                let v_pn = v_time.get(np).map(|v| v[t]).unwrap_or(0.0)
                    - v_time.get(nn).map(|v| v[t]).unwrap_or(0.0);
                let (i, g) = nonlinear_fn(&[v_pn]);

                if np < i_time.len() {
                    i_time[np][t] += i;
                    g_time[np][t] += g;
                }
                if nn < i_time.len() {
                    i_time[nn][t] -= i;
                    g_time[nn][t] -= g;
                }
            }
        }

        // Convert currents back to frequency domain
        let mut i_spectrum: Vec<Vec<Complex64>> = Vec::with_capacity(self.num_nodes);
        for node in 0..self.num_nodes {
            let spectrum = self.fft.to_frequency_domain(&i_time[node]);
            i_spectrum.push(spectrum);
        }

        // Add nonlinear current to residual
        for (node, i_spec) in i_spectrum.iter().enumerate() {
            for (k, &i) in i_spec.iter().enumerate() {
                if node < state.residual.len() && k < state.residual[node].len() {
                    state.residual[node][k] += i;
                    state.residual_scale[node][k] += i.norm();
                }
            }
        }

        state.compute_residual_norm();
        state.iteration += 1;

        // Check convergence per KCL row.
        state.converged = state.rows_converged(self.config.tolerance, self.config.abstol);

        Ok(())
    }
}
