//! Periodic Steady-State (PSS) Analysis
//!
//! This module provides PSS analysis using the shooting method.
//!
//! # Overview
//!
//! PSS finds the periodic steady-state solution of a circuit by solving the
//! boundary value problem: x(0) = x(T) where T is the period.
//!
//! # Algorithm
//!
//! 1. **Stabilization phase** (`tstab`): Run transient to approach steady-state
//! 2. **Period detection**: For autonomous oscillators, auto-detect the period
//! 3. **Shooting Newton loop**:
//!    - Simulate one period from current `x0`
//!    - Compute residual `x(T) - x(0)`
//!    - Check convergence
//!    - Compute Jacobian (Monodromy - I) via finite differences
//!    - Solve for Newton step and update `x0`
//! 4. Build final `PssResult` with periodic waveform and harmonics

#![allow(clippy::needless_range_loop)]
use super::{Engine, SimulationError, TransientResult};
use crate::analysis::transient::{
    BreakpointManager, CompanionCoefficients, LteEstimator, TimestepController, TrapGearController,
};
use crate::analysis::{
    PeriodDetector, PeriodicWaveform, PssConfig, PssResult, ShootingNewtonSolver, ShootingState,
};
use crate::circuit::Circuit;
use crate::solver::StaticMatrix;
use crate::{Netlist, Value};

/// PSS-specific error types
#[derive(Debug, Clone)]
pub enum PssError {
    /// Newton iteration did not converge
    ConvergenceFailed { iterations: usize, residual: Value },
    /// Period detection failed for autonomous oscillator
    PeriodDetectionFailed(String),
    /// Circuit has no reactive elements (no periodic solution possible)
    NoReactiveElements,
    /// Invalid configuration
    InvalidConfig(String),
}

impl std::fmt::Display for PssError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConvergenceFailed {
                iterations,
                residual,
            } => {
                write!(
                    f,
                    "PSS convergence failed after {} iterations (residual: {:.3e})",
                    iterations, residual
                )
            }
            Self::PeriodDetectionFailed(msg) => write!(f, "Period detection failed: {}", msg),
            Self::NoReactiveElements => write!(f, "Circuit has no capacitors or inductors"),
            Self::InvalidConfig(msg) => write!(f, "Invalid PSS config: {}", msg),
        }
    }
}

impl std::error::Error for PssError {}

impl From<PssError> for SimulationError {
    fn from(e: PssError) -> Self {
        match e {
            PssError::ConvergenceFailed { iterations, .. } => {
                SimulationError::ConvergenceFailed(iterations)
            }
            _ => SimulationError::Circuit(e.to_string()),
        }
    }
}

/// PSS analysis result with detailed convergence info
#[derive(Debug)]
pub struct PssAnalysisResult {
    /// The periodic steady-state solution
    pub result: PssResult,
    /// Number of shooting Newton iterations
    pub iterations: usize,
    /// Final residual norm
    pub final_residual: Value,
    /// Detected/refined period
    pub period: Value,
    /// Monodromy matrix (state transition over one period)
    pub monodromy: Vec<Vec<Value>>,
    /// Floquet multipliers (for stability analysis)
    pub floquet_multipliers: Vec<num_complex::Complex64>,
    /// Whether circuit is stable (all multipliers inside unit circle)
    pub is_stable: bool,
}

impl Engine {
    /// Run Periodic Steady-State analysis
    ///
    /// This is the main entry point for PSS simulation. It handles both driven
    /// circuits (known period from source frequency) and autonomous oscillators
    /// (period detected from waveform).
    ///
    /// # Arguments
    ///
    /// * `netlist` - The circuit netlist
    /// * `config` - PSS configuration (frequency, tolerances, etc.)
    ///
    /// # Returns
    ///
    /// * `Ok(PssAnalysisResult)` - Converged periodic solution with waveforms
    /// * `Err(SimulationError)` - If convergence fails or circuit is invalid
    pub fn run_pss(
        &self,
        netlist: &Netlist,
        config: PssConfig,
    ) -> Result<PssAnalysisResult, SimulationError> {
        // Build and prepare circuit
        let mut circuit = self.build_circuit(netlist)?;
        let mut matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        // Validate circuit has reactive elements
        let state_dimension = circuit.capacitors.len() + circuit.inductors.len();
        if state_dimension == 0 {
            return Err(PssError::NoReactiveElements.into());
        }

        // Get DC operating point as initial condition.
        let dc_solution = self.solve_dc_operating_point(netlist, &mut circuit, &mut matrix)?;

        // Initialize capacitor/inductor state from DC
        self.pss_initialize_reactive_state(&mut circuit, &dc_solution);

        // ==================================================================
        // Phase 1: Stabilization (tstab)
        // ==================================================================
        let period = config.period();
        let (stabilized_waveform, current_state) =
            self.pss_run_stabilization(&mut circuit, &mut matrix, &dc_solution, &config)?;

        // ==================================================================
        // Phase 2: Period Detection (for autonomous oscillators)
        // ==================================================================
        let detected_period = if config.is_autonomous() && config.auto_period {
            self.pss_detect_oscillator_period(&stabilized_waveform, &config)?
        } else {
            period
        };

        // ==================================================================
        // Phase 3: Shooting Newton Loop
        // ==================================================================
        // Finite difference step for Jacobian computation
        const FD_STEP: Value = 1e-8;

        let mut solver = ShootingNewtonSolver::new(config.tolerance, config.max_iterations)
            .with_damping(config.damping_factor)
            .with_fd_step(FD_STEP);

        let mut shooting_state = ShootingState::new(current_state.clone(), detected_period);
        let mut final_waveform: Option<TransientResult> = None;
        let mut iteration = 0;

        while iteration < config.max_iterations {
            // Simulate one period
            self.pss_set_reactive_state(&mut circuit, &shooting_state.x0);

            let (x_t, waveform) =
                self.pss_simulate_one_period(&mut circuit, &mut matrix, detected_period, &config)?;

            shooting_state.x_t = x_t;
            shooting_state.compute_residual();

            // Check convergence
            if solver.check_convergence(&shooting_state) {
                final_waveform = Some(waveform);
                break;
            }

            // Compute Newton step using finite-difference Jacobian
            // Note: We can't clone circuit/matrix, so we use a simple update strategy
            let delta = self.pss_compute_newton_step(
                &mut circuit,
                &mut matrix,
                &shooting_state,
                detected_period,
                &config,
                FD_STEP,
            )?;

            // Update state with damping
            shooting_state.update_x0(&delta, solver.damping);
            final_waveform = Some(waveform);

            iteration += 1;
        }

        // Check if we converged
        if !solver.has_converged() && iteration >= config.max_iterations {
            return Err(PssError::ConvergenceFailed {
                iterations: iteration,
                residual: shooting_state.residual_norm(),
            }
            .into());
        }

        // ==================================================================
        // Phase 4: Build Result
        // ==================================================================
        let waveform = final_waveform.unwrap();

        // Compute Floquet multipliers for stability analysis
        let monodromy = self.pss_compute_monodromy(
            &mut circuit,
            &mut matrix,
            &shooting_state,
            detected_period,
            &config,
            FD_STEP,
        )?;
        let floquet_multipliers = solver.compute_floquet_multipliers(&monodromy);
        let is_stable = floquet_multipliers.iter().all(|m| m.norm() <= 1.0 + 1e-6);

        // Build PssResult
        let pss_result = self.pss_build_result(
            &waveform,
            detected_period,
            iteration,
            shooting_state.residual_norm(),
        );

        Ok(PssAnalysisResult {
            result: pss_result,
            iterations: iteration,
            final_residual: shooting_state.residual_norm(),
            period: detected_period,
            monodromy,
            floquet_multipliers,
            is_stable,
        })
    }

    /// Initialize reactive element state from DC solution
    fn pss_initialize_reactive_state(&self, circuit: &mut Circuit, dc_solution: &[Value]) {
        // Initialize capacitor voltages
        for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            let np = cap.pp.row;
            let nn = cap.nn.row;
            let v_dc = if np == 0 {
                0.0
            } else {
                dc_solution.get(np - 1).copied().unwrap_or(0.0)
            } - if nn == 0 {
                0.0
            } else {
                dc_solution.get(nn - 1).copied().unwrap_or(0.0)
            };
            circuit.capacitors.v_prev[cap_idx] = v_dc;
            circuit.capacitors.v_prev_prev[cap_idx] = v_dc;
        }

        // Initialize inductor currents
        for l_idx in 0..circuit.inductors.names.len() {
            let np = circuit.inductors.node_pos[l_idx];
            let nn = circuit.inductors.node_neg[l_idx];
            let br = circuit.inductors.branch_indices[l_idx];

            let v_dc = if np == 0 {
                0.0
            } else {
                dc_solution.get(np - 1).copied().unwrap_or(0.0)
            } - if nn == 0 {
                0.0
            } else {
                dc_solution.get(nn - 1).copied().unwrap_or(0.0)
            };
            circuit.inductors.v_prev[l_idx] = v_dc;

            if br > 0 {
                let br_idx = circuit.num_nodes() + br - 1;
                let i_dc = dc_solution.get(br_idx).copied().unwrap_or(0.0);
                circuit.inductors.i_prev[l_idx] = i_dc;
                circuit.inductors.i_prev_prev[l_idx] = i_dc;
            }
        }
    }

    /// Extract state vector (capacitor voltages + inductor currents)
    fn pss_extract_reactive_state(&self, circuit: &Circuit) -> Vec<Value> {
        let mut state = Vec::with_capacity(circuit.capacitors.len() + circuit.inductors.len());

        // Capacitor voltages
        for v in &circuit.capacitors.v_prev {
            state.push(*v);
        }

        // Inductor currents
        for i in &circuit.inductors.i_prev {
            state.push(*i);
        }

        state
    }

    /// Set reactive element state from state vector
    fn pss_set_reactive_state(&self, circuit: &mut Circuit, state: &[Value]) {
        let n_caps = circuit.capacitors.len();

        // Set capacitor voltages
        for (i, v) in state.iter().take(n_caps).enumerate() {
            circuit.capacitors.v_prev[i] = *v;
            circuit.capacitors.v_prev_prev[i] = *v;
        }

        // Set inductor currents
        for (i, current) in state.iter().skip(n_caps).enumerate() {
            circuit.inductors.i_prev[i] = *current;
            circuit.inductors.i_prev_prev[i] = *current;
        }
    }

    /// Run stabilization phase (`tstab`)
    fn pss_run_stabilization(
        &self,
        circuit: &mut Circuit,
        matrix: &mut StaticMatrix,
        dc_solution: &[Value],
        config: &PssConfig,
    ) -> Result<(TransientResult, Vec<Value>), SimulationError> {
        let period = config.period();
        let tstab = config.effective_tstab();

        if tstab > 0.0 {
            let max_step = period / 50.0;
            let waveform =
                self.pss_run_tran_internal(circuit, matrix, dc_solution.to_vec(), tstab, max_step)?;

            let final_state = self.pss_extract_reactive_state(circuit);
            Ok((waveform, final_state))
        } else {
            let initial_state = self.pss_extract_reactive_state(circuit);

            let waveform = TransientResult {
                time: vec![0.0],
                voltages: (0..circuit.num_nodes())
                    .map(|i| vec![dc_solution.get(i).copied().unwrap_or(0.0)])
                    .collect(),
                branch_currents: Vec::new(),
                num_nodes: circuit.num_nodes(),
                branch_names: Vec::new(),
                node_names: circuit.node_names_sorted(),
            };

            Ok((waveform, initial_state))
        }
    }

    /// Detect oscillator period from stabilized waveform
    fn pss_detect_oscillator_period(
        &self,
        waveform: &TransientResult,
        config: &PssConfig,
    ) -> Result<Value, SimulationError> {
        if waveform.voltages.is_empty() || waveform.time.len() < 10 {
            return Err(
                PssError::PeriodDetectionFailed("Insufficient waveform data".to_string()).into(),
            );
        }

        let values = &waveform.voltages[0];
        let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min);

        if (max_val - min_val).abs() < 1e-9 {
            return Err(PssError::PeriodDetectionFailed(
                "No oscillation detected (constant voltage)".to_string(),
            )
            .into());
        }

        let detector = PeriodDetector::with_guess(config.period());
        let estimate = detector.detect(&waveform.time, values);

        if estimate.confidence < 0.3 {
            return Err(PssError::PeriodDetectionFailed(format!(
                "Low confidence: {:.1}%",
                estimate.confidence * 100.0
            ))
            .into());
        }

        Ok(estimate.period)
    }

    /// Simulate one complete period
    fn pss_simulate_one_period(
        &self,
        circuit: &mut Circuit,
        matrix: &mut StaticMatrix,
        period: Value,
        config: &PssConfig,
    ) -> Result<(Vec<Value>, TransientResult), SimulationError> {
        let max_step = period / config.points_per_period as f64;

        // Get current voltages as starting solution
        let solution = vec![0.0; circuit.matrix_size()];

        let waveform = self.pss_run_tran_internal(circuit, matrix, solution, period, max_step)?;

        let final_state = self.pss_extract_reactive_state(circuit);
        Ok((final_state, waveform))
    }

    /// Compute Newton step using finite-difference Jacobian
    fn pss_compute_newton_step(
        &self,
        circuit: &mut Circuit,
        matrix: &mut StaticMatrix,
        state: &ShootingState,
        period: Value,
        config: &PssConfig,
        fd_step: Value,
    ) -> Result<Vec<Value>, SimulationError> {
        let n = state.dimension();
        let mut jacobian = vec![vec![0.0; n]; n];
        let x0 = &state.x0;
        let f0 = &state.residual;

        // Compute Jacobian columns via finite differences
        for j in 0..n {
            // Perturb state in direction j
            let mut x_perturbed = x0.clone();
            let h = fd_step * (1.0 + x0[j].abs());
            x_perturbed[j] += h;

            // Simulate with perturbed IC
            self.pss_set_reactive_state(circuit, &x_perturbed);
            let (x_t_perturbed, _) =
                self.pss_simulate_one_period(circuit, matrix, period, config)?;

            // Compute perturbed residual
            let f_perturbed: Vec<Value> = x_t_perturbed
                .iter()
                .zip(x_perturbed.iter())
                .map(|(xt, x0)| xt - x0)
                .collect();

            // Jacobian column: (f_perturbed - f0) / h
            for i in 0..n {
                jacobian[i][j] = (f_perturbed[i] - f0[i]) / h;
            }
        }

        // Restore original state
        self.pss_set_reactive_state(circuit, x0);

        // Solve: (J) * delta = -f0 using simple Gaussian elimination
        let delta = self.pss_solve_linear_system(&jacobian, f0)?;

        Ok(delta)
    }

    /// Compute Monodromy matrix via finite differences
    fn pss_compute_monodromy(
        &self,
        circuit: &mut Circuit,
        matrix: &mut StaticMatrix,
        state: &ShootingState,
        period: Value,
        config: &PssConfig,
        fd_step: Value,
    ) -> Result<Vec<Vec<Value>>, SimulationError> {
        let n = state.dimension();
        let mut monodromy = vec![vec![0.0; n]; n];
        let x0 = &state.x0;

        // Get unperturbed final state
        self.pss_set_reactive_state(circuit, x0);
        let (x_t_base, _) = self.pss_simulate_one_period(circuit, matrix, period, config)?;

        // Compute Monodromy columns via finite differences
        for j in 0..n {
            let mut x_perturbed = x0.clone();
            let h = fd_step * (1.0 + x0[j].abs());
            x_perturbed[j] += h;

            self.pss_set_reactive_state(circuit, &x_perturbed);
            let (x_t_perturbed, _) =
                self.pss_simulate_one_period(circuit, matrix, period, config)?;

            // Monodromy column: d(x_T)/d(x_0) â‰ˆ (x_T_perturbed - x_T_base) / h
            for i in 0..n {
                monodromy[i][j] = (x_t_perturbed[i] - x_t_base[i]) / h;
            }
        }

        // Restore original state
        self.pss_set_reactive_state(circuit, x0);

        Ok(monodromy)
    }

    /// Solve linear system using Gaussian elimination
    fn pss_solve_linear_system(
        &self,
        a: &[Vec<Value>],
        b: &[Value],
    ) -> Result<Vec<Value>, SimulationError> {
        let n = b.len();
        if n == 0 {
            return Ok(vec![]);
        }

        // Augmented matrix
        let mut aug: Vec<Vec<Value>> = a
            .iter()
            .zip(b.iter())
            .map(|(row, &bi)| {
                let mut r = row.clone();
                r.push(-bi); // Solve Ax = -b (Newton step)
                r
            })
            .collect();

        // Forward elimination with partial pivoting
        for col in 0..n {
            // Find pivot
            let mut max_row = col;
            for row in (col + 1)..n {
                if aug[row][col].abs() > aug[max_row][col].abs() {
                    max_row = row;
                }
            }
            aug.swap(col, max_row);

            let pivot = aug[col][col];
            if pivot.abs() < 1e-15 {
                // Near-singular, use regularization
                continue;
            }

            // Eliminate
            for row in (col + 1)..n {
                let factor = aug[row][col] / pivot;
                for k in col..=n {
                    aug[row][k] -= factor * aug[col][k];
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
            if aug[i][i].abs() > 1e-15 {
                x[i] = sum / aug[i][i];
            }
        }

        Ok(x)
    }

    /// Internal transient simulation
    fn pss_run_tran_internal(
        &self,
        circuit: &mut Circuit,
        matrix: &mut StaticMatrix,
        mut solution: Vec<Value>,
        tstop: Value,
        max_step: Value,
    ) -> Result<TransientResult, SimulationError> {
        let num_nodes = circuit.num_nodes();
        let size = circuit.matrix_size();

        let initial_step = (max_step / 10.0).min(tstop / 100.0);
        let mut timestep =
            TimestepController::new(initial_step, self.config.min_timestep, max_step);
        let mut breakpoints = BreakpointManager::new();
        let mut lte_estimator =
            LteEstimator::with_tolerances(self.voltage_reltol(), self.voltage_abstol());
        let mut trapgear = TrapGearController::new();

        let node_names = circuit.node_names_sorted();
        let mut result = TransientResult {
            time: vec![0.0],
            voltages: (0..num_nodes)
                .map(|i| vec![solution.get(i).copied().unwrap_or(0.0)])
                .collect(),
            branch_currents: Vec::new(),
            num_nodes,
            branch_names: Vec::new(),
            node_names,
        };

        let mut t = 0.0;
        const MAX_ITERATIONS: usize = 100_000;
        let mut total_iterations = 0;

        while t < tstop && total_iterations < MAX_ITERATIONS {
            total_iterations += 1;
            let (dt, _) = breakpoints.limit_step(t, timestep.dt());
            let dt = dt.min(tstop - t);

            let mut new_solution = solution.clone();
            let mut rhs = vec![0.0; size];
            let mut converged = false;

            for _iter in 0..self.config.max_iterations {
                matrix.clear_values();
                rhs.fill(0.0);

                for i in 0..size {
                    matrix.add(i, i, 1e-12);
                }

                circuit.stamp_transient_linear_direct(matrix, &mut rhs);

                let current_method = trapgear.current_method();
                let coeff = CompanionCoefficients::for_method(current_method);

                // Stamp capacitors
                for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
                    let capacitance = circuit.capacitors.capacitances[cap_idx];
                    let np = cap.pp.row;
                    let nn = cap.nn.row;
                    let v_n = circuit.capacitors.v_prev[cap_idx];
                    let v_n_minus_1 = circuit.capacitors.v_prev_prev[cap_idx];

                    let geq = coeff.capacitor_geq(capacitance, dt);
                    let i_n_cap = circuit.capacitors.i_prev[cap_idx];
                    let ieq = coeff.capacitor_ieq(capacitance, dt, v_n, v_n_minus_1, i_n_cap);

                    if np > 0 {
                        matrix.add(np - 1, np - 1, geq);
                        if nn > 0 {
                            matrix.add(np - 1, nn - 1, -geq);
                        }
                    }
                    if nn > 0 {
                        if np > 0 {
                            matrix.add(nn - 1, np - 1, -geq);
                        }
                        matrix.add(nn - 1, nn - 1, geq);
                    }
                    if np > 0 {
                        rhs[np - 1] += ieq;
                    }
                    if nn > 0 {
                        rhs[nn - 1] -= ieq;
                    }
                }

                // Stamp inductors
                for l_idx in 0..circuit.inductors.names.len() {
                    let np = circuit.inductors.node_pos[l_idx];
                    let nn = circuit.inductors.node_neg[l_idx];
                    let br = circuit.inductors.branch_indices[l_idx];
                    let inductance = circuit.inductors.inductances[l_idx];
                    let i_n = circuit.inductors.i_prev[l_idx];
                    let i_n_minus_1 = circuit.inductors.i_prev_prev[l_idx];
                    let v_n = circuit.inductors.v_prev[l_idx];

                    let req = coeff.inductor_req(inductance, dt);
                    let veq = coeff.inductor_veq(inductance, dt, i_n, i_n_minus_1, v_n);

                    if np > 0 && br > 0 {
                        let br_idx = circuit.num_nodes() + br - 1;
                        matrix.add(br_idx, np - 1, 1.0);
                        matrix.add(np - 1, br_idx, 1.0);
                    }
                    if nn > 0 && br > 0 {
                        let br_idx = circuit.num_nodes() + br - 1;
                        matrix.add(br_idx, nn - 1, -1.0);
                        matrix.add(nn - 1, br_idx, -1.0);
                    }
                    if br > 0 {
                        let br_idx = circuit.num_nodes() + br - 1;
                        matrix.add(br_idx, br_idx, -req);
                        rhs[br_idx] = veq;
                    }
                }

                if circuit.has_nonlinear_devices() {
                    circuit.update_nonlinear(&new_solution);
                    circuit.stamp_nonlinear(matrix, &mut rhs, &new_solution);
                    circuit.stamp_behavioral(matrix, &mut rhs, &new_solution, t + dt);
                }

                match matrix.solve(&rhs) {
                    Ok(sol) => {
                        let voltage_converged = self.voltage_convergence_met(&new_solution, &sol);
                        let linearized_residual_converged =
                            self.residual_convergence_met(matrix, &sol, &rhs);

                        new_solution = sol;

                        if circuit.has_nonlinear_devices() {
                            circuit.update_nonlinear(&new_solution);
                        }

                        let device_converged = !circuit.has_nonlinear_devices()
                            || circuit.nonlinear_converged(self.device_convergence_criteria());

                        if voltage_converged && device_converged && linearized_residual_converged {
                            converged = true;
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }

            if !converged {
                timestep.force_step(dt * 0.25);
                continue;
            }

            t += dt;
            lte_estimator.record(&new_solution, dt);
            trapgear.update(&new_solution, dt);

            // Update capacitor history
            for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
                let np = cap.pp.row;
                let nn = cap.nn.row;
                let v_new = if np == 0 { 0.0 } else { new_solution[np - 1] }
                    - if nn == 0 { 0.0 } else { new_solution[nn - 1] };
                circuit.capacitors.v_prev_prev[cap_idx] = circuit.capacitors.v_prev[cap_idx];
                circuit.capacitors.v_prev[cap_idx] = v_new;

                // Update current history
                let coeff_update = CompanionCoefficients::for_method(trapgear.current_method());
                let geq = coeff_update.capacitor_geq(circuit.capacitors.capacitances[cap_idx], dt);
                let i_n_cap = circuit.capacitors.i_prev[cap_idx];
                let ieq = coeff_update.capacitor_ieq(
                    circuit.capacitors.capacitances[cap_idx],
                    dt,
                    circuit.capacitors.v_prev_prev[cap_idx],
                    circuit.capacitors.v_prev_prev[cap_idx],
                    i_n_cap,
                );
                circuit.capacitors.i_prev[cap_idx] = geq * v_new - ieq;
            }

            // Update inductor history
            for l_idx in 0..circuit.inductors.names.len() {
                let br = circuit.inductors.branch_indices[l_idx];
                if br > 0 {
                    let br_idx = circuit.num_nodes() + br - 1;
                    let i_new = new_solution[br_idx];
                    circuit.inductors.i_prev_prev[l_idx] = circuit.inductors.i_prev[l_idx];
                    circuit.inductors.i_prev[l_idx] = i_new;

                    let np = circuit.inductors.node_pos[l_idx];
                    let nn = circuit.inductors.node_neg[l_idx];
                    let v_new = if np == 0 { 0.0 } else { new_solution[np - 1] }
                        - if nn == 0 { 0.0 } else { new_solution[nn - 1] };
                    circuit.inductors.v_prev[l_idx] = v_new;
                }
            }

            solution = new_solution;

            result.time.push(t);
            for (i, voltages) in result.voltages.iter_mut().enumerate() {
                voltages.push(solution.get(i).copied().unwrap_or(0.0));
            }

            let (lte, _) = lte_estimator.estimate(&solution, dt);
            let scale = lte_estimator.recommend_scale(lte);
            timestep.adjust(lte / scale);
        }

        Ok(result)
    }

    /// Build PssResult from transient waveform
    fn pss_build_result(
        &self,
        waveform: &TransientResult,
        period: Value,
        iterations: usize,
        residual_norm: Value,
    ) -> PssResult {
        let n_nodes = waveform.num_nodes;
        let n_points = waveform.time.len();

        let mut result = PssResult::new(period, n_nodes, n_points);
        result.time = waveform.time.clone();
        result.iterations = iterations;
        result.residual_norm = residual_norm;
        result.node_names = waveform.node_names.clone();
        result.period_detected = true;

        for (i, wf) in result.waveforms.iter_mut().enumerate() {
            if i < waveform.voltages.len() {
                *wf = PeriodicWaveform::from_values(waveform.voltages[i].clone());
            }
        }

        result
    }
}
