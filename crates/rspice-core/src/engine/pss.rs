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

#![allow(clippy::needless_range_loop, clippy::too_many_arguments)]
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

type AutonomousNewtonStep = (Vec<Value>, Value, Vec<Vec<Value>>);

/// Recover the monodromy matrix from a converged shooting Jacobian:
/// the shooting residual is F(x0) = x(T) - x0, so J = dF/dx0 = M - I and
/// M = J + I. Reusing the Jacobian saves N+1 full period integrations.
fn monodromy_from_newton_jacobian(mut jacobian: Vec<Vec<Value>>) -> Vec<Vec<Value>> {
    for (i, row) in jacobian.iter_mut().enumerate() {
        if i < row.len() {
            row[i] += 1.0;
        }
    }
    jacobian
}

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

/// Reactive-state and node-solution trajectory recorded on the fixed
/// integration grid, used by the oscillator phase-noise (PPV) machinery.
#[derive(Debug, Default, Clone)]
pub(crate) struct PssStateTrace {
    /// Grid times, starting at 0.
    pub times: Vec<Value>,
    /// Reactive state (cap voltages then inductor currents) per grid point.
    pub states: Vec<Vec<Value>>,
    /// Full node/branch solution per grid point.
    pub solutions: Vec<Vec<Value>>,
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
        self.run_pss_with_state(netlist, config)
            .map(|(result, _, _, _)| result)
    }

    /// `run_pss` plus the converged artifacts the oscillator phase-noise
    /// machinery needs: the prepared circuit/matrix pair and the converged
    /// shooting state x0.
    pub(in crate::engine) fn run_pss_with_state(
        &self,
        netlist: &Netlist,
        config: PssConfig,
    ) -> Result<(PssAnalysisResult, Circuit, StaticMatrix, Vec<Value>), SimulationError> {
        config.validate().map_err(PssError::InvalidConfig)?;

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
        let mut detected_period = if config.is_autonomous() && config.auto_period {
            // Detection only seeds the Newton iteration (the period is now a
            // shooting unknown closed by the phase condition), so a low-
            // confidence detection falls back to the configured guess
            // instead of aborting the analysis.
            self.pss_detect_oscillator_period(&stabilized_waveform, &config)
                .unwrap_or_else(|_| config.period())
        } else {
            period
        };

        // ==================================================================
        // Phase 3: Shooting Newton Loop
        // ==================================================================
        // Finite difference step for Jacobian computation
        const FD_STEP: Value = 1e-8;

        let mut solver = ShootingNewtonSolver::new(config.tolerance, config.max_iterations)
            .with_abstol(config.abstol)
            .with_damping(config.damping_factor)
            .with_fd_step(FD_STEP);

        let mut shooting_state = ShootingState::new(current_state.clone(), detected_period);
        let mut final_waveform: Option<TransientResult> = None;
        let mut iteration = 0;
        // The most recent shooting Jacobian (J = M - I). At convergence it
        // doubles as the monodromy source, saving N+1 period integrations.
        let mut last_jacobian: Option<Vec<Vec<Value>>> = None;

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

            // Compute Newton step using a finite-difference Jacobian whose
            // columns integrate perturbed periods in parallel on per-worker
            // circuit clones (pure per-column work — deterministic).
            if config.is_autonomous() {
                // Oscillators: the period is a Newton unknown alongside the
                // state, closed by a Poincare phase condition.
                let (delta, delta_t, jacobian) = self.pss_compute_autonomous_newton_step(
                    &circuit,
                    &shooting_state,
                    detected_period,
                    &config,
                    FD_STEP,
                )?;
                last_jacobian = Some(jacobian);
                shooting_state.update_x0(&delta, solver.damping);
                let max_dt = config.max_period_change * detected_period;
                detected_period += (solver.damping * delta_t).clamp(-max_dt, max_dt);
                shooting_state.period = detected_period;
            } else {
                let (delta, jacobian) = self.pss_compute_newton_step(
                    &circuit,
                    &shooting_state,
                    detected_period,
                    &config,
                    FD_STEP,
                )?;
                last_jacobian = Some(jacobian);
                shooting_state.update_x0(&delta, solver.damping);
            }
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

        // Compute Floquet multipliers for stability analysis. The converged
        // shooting Jacobian already contains the monodromy (J = M - I), so
        // reuse it instead of re-integrating N+1 periods; the fresh-FD path
        // remains for the zero-iteration case (initial state was already
        // periodic, so no Jacobian was ever built).
        // Autonomous orbits get a FRESH monodromy at the converged (x0, T):
        // the structural unity Floquet multiplier exists only on the closed
        // orbit, so the recycled pre-convergence Jacobian (off-orbit, stale
        // period) is not accurate enough for stability classification there.
        let monodromy = if config.is_autonomous() {
            self.pss_set_reactive_state(&mut circuit, &shooting_state.x0);
            self.pss_compute_monodromy(
                &circuit,
                &shooting_state,
                detected_period,
                &config,
                FD_STEP,
            )?
        } else {
            match last_jacobian {
                Some(jacobian) => monodromy_from_newton_jacobian(jacobian),
                None => self.pss_compute_monodromy(
                    &circuit,
                    &shooting_state,
                    detected_period,
                    &config,
                    FD_STEP,
                )?,
            }
        };
        let floquet_multipliers = solver.compute_floquet_multipliers(&monodromy);
        let is_stable = floquet_multipliers.iter().all(|m| m.norm() <= 1.0 + 1e-6);

        // Build PssResult
        let pss_result = self.pss_build_result(
            &waveform,
            detected_period,
            iteration,
            shooting_state.residual_norm(),
        );

        Ok((
            PssAnalysisResult {
                result: pss_result,
                iterations: iteration,
                final_residual: shooting_state.residual_norm(),
                period: detected_period,
                monodromy,
                floquet_multipliers,
                is_stable,
            },
            circuit,
            matrix,
            shooting_state.x0.clone(),
        ))
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
    ///
    /// Clears every other piece of companion history (capacitor currents,
    /// inductor voltages): the first step of each period integration runs
    /// backward Euler, which reads neither, so the period map becomes a pure
    /// function of the shooting state. Leaving stale history in place would
    /// leak the previous trajectory into the next one and corrupt both the
    /// shooting residual and the finite-difference Jacobian columns.
    pub(in crate::engine) fn pss_set_reactive_state(&self, circuit: &mut Circuit, state: &[Value]) {
        let n_caps = circuit.capacitors.len();

        // Set capacitor voltages
        for (i, v) in state.iter().take(n_caps).enumerate() {
            circuit.capacitors.v_prev[i] = *v;
            circuit.capacitors.v_prev_prev[i] = *v;
            circuit.capacitors.i_prev[i] = 0.0;
        }

        // Set inductor currents
        for (i, current) in state.iter().skip(n_caps).enumerate() {
            circuit.inductors.i_prev[i] = *current;
            circuit.inductors.i_prev_prev[i] = *current;
            circuit.inductors.v_prev[i] = 0.0;
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
            let waveform = self.pss_run_tran_internal(
                circuit,
                matrix,
                dc_solution.to_vec(),
                tstab,
                max_step,
                false,
                None,
            )?;

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
                digital_traces: Vec::new(),
                real_traces: Vec::new(),
                device_op_traces: Vec::new(),
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

        // Node voltages consistent with the frozen reactive state: they seed
        // the first Newton solve and become the genuine t=0 waveform sample.
        let solution = self.pss_initial_node_solution(circuit, matrix, period)?;

        // The period map must vary smoothly with the shooting state: run on
        // the fixed grid (see pss_run_tran_internal).
        let waveform =
            self.pss_run_tran_internal(circuit, matrix, solution, period, max_step, true, None)?;

        let final_state = self.pss_extract_reactive_state(circuit);
        Ok((final_state, waveform))
    }

    /// Solve the network at t = 0 with the reactive state held frozen.
    ///
    /// A backward-Euler companion step of `period * 1e-9` pins each capacitor
    /// to its state voltage and each inductor to its state current (the
    /// resulting error equals the state drift over that tiny interval), while
    /// the nonlinear devices settle to node voltages consistent with that
    /// state. Without this, the waveform's first sample and the Newton seed
    /// were all zeros.
    pub(in crate::engine) fn pss_initial_node_solution(
        &self,
        circuit: &mut Circuit,
        matrix: &mut StaticMatrix,
        period: Value,
    ) -> Result<Vec<Value>, SimulationError> {
        let dt_freeze = period * 1e-9;
        let coeff =
            CompanionCoefficients::for_method(crate::analysis::IntegrationMethod::BackwardEuler);
        let start = vec![0.0; circuit.matrix_size()];

        match self.pss_newton_solve(circuit, matrix, &coeff, dt_freeze, dt_freeze, &start)? {
            Some(solution) => Ok(solution),
            None => Ok(start),
        }
    }

    /// Compute the shooting sensitivity columns by CENTRAL differences on
    /// the fixed-grid period map: column j is
    /// `(F(x0 + h e_j) - F(x0 - h e_j)) / 2h` with `F = Phi - I` when
    /// `subtract_identity` is set (Newton Jacobian) or `F = Phi` otherwise
    /// (monodromy). The fixed integration grid makes Phi smooth in x0, so
    /// the O(h^2) central difference reaches derivative accuracy the
    /// adaptive-grid forward difference never could.
    ///
    /// Columns are pure functions of `(x0, j)`, so they parallelize across
    /// per-worker circuit clones with deterministic results. `CircuitData`
    /// is Send-but-not-Sync (Cell-based device caches), so the work is
    /// chunked AC-sweep-style: one owned clone per worker chunk, matrix
    /// rebuilt per worker (StaticMatrix holds factorization workspaces and
    /// is intentionally not Clone).
    fn pss_sensitivity_columns(
        &self,
        circuit: &Circuit,
        x0: &[Value],
        period: Value,
        config: &PssConfig,
        fd_step: Value,
        subtract_identity: bool,
    ) -> Result<Vec<Vec<Value>>, SimulationError> {
        let n = x0.len();
        if n == 0 {
            return Ok(Vec::new());
        }

        let column = |worker_circuit: &mut Circuit,
                      worker_matrix: &mut StaticMatrix,
                      j: usize|
         -> Result<Vec<Value>, SimulationError> {
            let h = fd_step * (1.0 + x0[j].abs());

            let mut x_plus = x0.to_vec();
            x_plus[j] += h;
            self.pss_set_reactive_state(worker_circuit, &x_plus);
            let (x_t_plus, _) =
                self.pss_simulate_one_period(worker_circuit, worker_matrix, period, config)?;

            let mut x_minus = x0.to_vec();
            x_minus[j] -= h;
            self.pss_set_reactive_state(worker_circuit, &x_minus);
            let (x_t_minus, _) =
                self.pss_simulate_one_period(worker_circuit, worker_matrix, period, config)?;

            Ok((0..n)
                .map(|i| {
                    let f_plus = if subtract_identity {
                        x_t_plus[i] - x_plus[i]
                    } else {
                        x_t_plus[i]
                    };
                    let f_minus = if subtract_identity {
                        x_t_minus[i] - x_minus[i]
                    } else {
                        x_t_minus[i]
                    };
                    (f_plus - f_minus) / (2.0 * h)
                })
                .collect())
        };

        #[cfg(feature = "parallel")]
        if n >= 2 {
            use rayon::prelude::*;

            let workers = rayon::current_num_threads().clamp(1, n);
            let chunk_len = n.div_ceil(workers);
            let indices: Vec<usize> = (0..n).collect();
            let work: Vec<(Circuit, Vec<usize>)> = indices
                .chunks(chunk_len)
                .map(|chunk| (circuit.clone(), chunk.to_vec()))
                .collect();

            let chunk_columns: Result<Vec<Vec<Vec<Value>>>, SimulationError> = work
                .into_par_iter()
                .map(|(mut worker_circuit, chunk)| {
                    let matrix = self.build_matrix(&worker_circuit)?;
                    worker_circuit.link_indices(&matrix);
                    let mut worker_matrix = matrix;
                    chunk
                        .into_iter()
                        .map(|j| column(&mut worker_circuit, &mut worker_matrix, j))
                        .collect()
                })
                .collect();
            return chunk_columns.map(|chunks| chunks.into_iter().flatten().collect());
        }

        let mut worker_circuit = circuit.clone();
        let matrix = self.build_matrix(&worker_circuit)?;
        worker_circuit.link_indices(&matrix);
        let mut worker_matrix = matrix;
        (0..n)
            .map(|j| column(&mut worker_circuit, &mut worker_matrix, j))
            .collect()
    }

    /// Compute Newton step using a finite-difference shooting Jacobian.
    ///
    /// One period integration per state (the unperturbed trajectory is the
    /// shared base point), with columns evaluated in parallel on per-worker
    /// circuit clones. Returns the step together with the Jacobian so the
    /// caller can recycle it as the monodromy at convergence (J = M - I).
    fn pss_compute_newton_step(
        &self,
        circuit: &Circuit,
        state: &ShootingState,
        period: Value,
        config: &PssConfig,
        fd_step: Value,
    ) -> Result<(Vec<Value>, Vec<Vec<Value>>), SimulationError> {
        let n = state.dimension();
        let columns =
            self.pss_sensitivity_columns(circuit, &state.x0, period, config, fd_step, true)?;

        let mut jacobian = vec![vec![0.0; n]; n];
        for (j, column) in columns.iter().enumerate() {
            for i in 0..n {
                jacobian[i][j] = column[i];
            }
        }

        // Solve: (J) * delta = -f0 using simple Gaussian elimination
        let delta = self.pss_solve_linear_system(&jacobian, &state.residual)?;

        Ok((delta, jacobian))
    }

    /// Newton step for AUTONOMOUS shooting: the period T joins the state as
    /// an unknown, closed by the Poincare phase condition f(x(T))^T ds = 0
    /// (no update component along the orbit, where the period map is
    /// neutrally stable). The augmented system is
    ///
    /// ```text
    /// [ M - I        dPhi/dT ] [ds]   [-r]
    /// [ f(x(T))^T    0       ] [dT] = [ 0]
    /// ```
    ///
    /// with dPhi/dT differenced in T on the fixed grid (the step count stays
    /// constant, so the map is smooth in T) and f(x(T)) = dPhi/dT itself,
    /// the orbit tangent at the endpoint.
    fn pss_compute_autonomous_newton_step(
        &self,
        circuit: &Circuit,
        state: &ShootingState,
        period: Value,
        config: &PssConfig,
        fd_step: Value,
    ) -> Result<AutonomousNewtonStep, SimulationError> {
        let n = state.dimension();
        let columns =
            self.pss_sensitivity_columns(circuit, &state.x0, period, config, fd_step, true)?;

        // dPhi/dT by forward difference; Phi_T(x0) is already in state.x_t.
        let h_t = period * 1e-7;
        let mut worker = circuit.clone();
        let m = self.build_matrix(&worker)?;
        worker.link_indices(&m);
        let mut worker_matrix = m;
        self.pss_set_reactive_state(&mut worker, &state.x0);
        let (x_t_plus, _) =
            self.pss_simulate_one_period(&mut worker, &mut worker_matrix, period + h_t, config)?;
        let dphi_dt: Vec<Value> = (0..n).map(|i| (x_t_plus[i] - state.x_t[i]) / h_t).collect();

        let mut jacobian = vec![vec![0.0; n + 1]; n + 1];
        for (j, column) in columns.iter().enumerate() {
            for i in 0..n {
                jacobian[i][j] = column[i];
            }
        }
        for i in 0..n {
            jacobian[i][n] = dphi_dt[i];
            jacobian[n][i] = dphi_dt[i]; // phase row: orbit tangent
        }

        let mut rhs = state.residual.clone();
        rhs.push(0.0); // phase condition has zero residual by construction

        let solution = self.pss_solve_linear_system(&jacobian, &rhs)?;
        let delta_state = solution[..n].to_vec();
        let delta_t = solution[n];

        // Monodromy reuse: the top-left block is M - I at the current T.
        let mut state_jacobian = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                state_jacobian[i][j] = jacobian[i][j];
            }
        }

        Ok((delta_state, delta_t, state_jacobian))
    }

    /// Compute the monodromy matrix via finite differences — the fallback
    /// for the zero-Newton-iteration case (otherwise the converged Newton
    /// Jacobian is recycled as J + I). The converged trajectory endpoint
    /// `state.x_t` is the shared base; columns run in parallel on private
    /// circuit clones.
    fn pss_compute_monodromy(
        &self,
        circuit: &Circuit,
        state: &ShootingState,
        period: Value,
        config: &PssConfig,
        fd_step: Value,
    ) -> Result<Vec<Vec<Value>>, SimulationError> {
        let n = state.dimension();
        let columns =
            self.pss_sensitivity_columns(circuit, &state.x0, period, config, fd_step, false)?;

        let mut monodromy = vec![vec![0.0; n]; n];
        for (j, column) in columns.iter().enumerate() {
            for i in 0..n {
                monodromy[i][j] = column[i];
            }
        }

        Ok(monodromy)
    }

    /// Solve linear system using Gaussian elimination
    pub(in crate::engine) fn pss_solve_linear_system(
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

    /// One companion-stamped Newton solve at `t_next` with step `dt`.
    ///
    /// Stamps the linear network, time-varying sources, reactive companions
    /// for the given integration coefficients, and nonlinear devices, then
    /// iterates to convergence. Reads — never writes — the companion history,
    /// so callers control when the trajectory actually advances. Returns
    /// `None` when Newton fails to converge at this step size.
    fn pss_newton_solve(
        &self,
        circuit: &mut Circuit,
        matrix: &mut StaticMatrix,
        coeff: &CompanionCoefficients,
        t_next: Value,
        dt: Value,
        start: &[Value],
    ) -> Result<Option<Vec<Value>>, SimulationError> {
        let size = circuit.matrix_size();
        let mut new_solution = start.to_vec();
        let mut rhs = vec![0.0; size];

        for _iter in 0..self.config.max_iterations {
            self.pss_stamp_system(circuit, matrix, &mut rhs, coeff, t_next, dt, &new_solution)?;

            match matrix.solve(&rhs) {
                Ok(sol) => {
                    let voltage_converged =
                        self.node_voltage_convergence_met(&new_solution, &sol, circuit.num_nodes());
                    let linearized_residual_converged =
                        self.residual_convergence_met(circuit, matrix, &sol, &rhs);

                    new_solution = sol;

                    if circuit.has_nonlinear_devices() {
                        circuit.update_nonlinear(&new_solution);
                    }

                    let device_converged = !circuit.has_nonlinear_devices()
                        || circuit.nonlinear_converged(self.device_convergence_criteria());

                    if voltage_converged && device_converged && linearized_residual_converged {
                        return Ok(Some(new_solution));
                    }
                }
                Err(_) => return Ok(None),
            }
        }

        Ok(None)
    }

    /// Stamp the full companion-linearized system at one time point:
    /// linear network, time-varying sources, reactive companions for the
    /// given coefficients, and the nonlinear Jacobian linearized at
    /// `linearize_at`. Shared by the Newton iteration and by the
    /// injection-sensitivity solves of the oscillator noise machinery,
    /// which ignore the RHS and solve the stamped matrix against unit
    /// current injections.
    #[allow(clippy::too_many_arguments)]
    pub(in crate::engine) fn pss_stamp_system(
        &self,
        circuit: &mut Circuit,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        coeff: &CompanionCoefficients,
        t_next: Value,
        dt: Value,
        linearize_at: &[Value],
    ) -> Result<(), SimulationError> {
        let size = circuit.matrix_size();
        matrix.clear_values();
        rhs.fill(0.0);

        for i in 0..size {
            matrix.add(i, i, 1e-12);
        }

        circuit.stamp_transient_linear_direct(matrix, rhs);

        // Time-varying independent sources: the static stamp wrote DC
        // values; overwrite the source rows with their value at the
        // end of this step. This is what makes driven PSS periodic —
        // without it a SIN drive stamps as its DC offset and the
        // "steady state" collapses to the DC solution.
        let num_nodes = circuit.num_nodes();
        circuit
            .voltage_sources
            .update_transient_rhs(rhs, t_next, |br_ordinal| num_nodes + br_ordinal);
        circuit.current_sources.update_transient_rhs(rhs, t_next);

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
                // Branch row sign convention: v - r_eq*i = -v_eq (see
                // Inductors::stamp_transient_companion).
                rhs[br_idx] = -veq;
            }
        }

        // Mutual coupling overlays on top of the standalone inductors.
        circuit.stamp_coupled_inductor_pairs_transient(matrix, rhs, dt, coeff);

        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(linearize_at);
            circuit
                .stamp_nonlinear(matrix, rhs, linearize_at)
                .map_err(SimulationError::Circuit)?;
            circuit.stamp_behavioral(
                matrix,
                rhs,
                linearize_at,
                t_next,
                crate::xspice::AnalysisType::Transient,
            );
        }
        Ok(())
    }

    /// Internal transient simulation
    /// `fixed_grid` integrates on a uniform time grid with a deterministic
    /// method sequence (backward Euler first step, trapezoidal after): the
    /// period map then varies SMOOTHLY with the initial state, which is what
    /// makes finite-difference shooting Jacobians and monodromy columns
    /// accurate — an LTE-adaptive grid changes its step decisions
    /// discontinuously under perturbation and floors the achievable
    /// derivative accuracy.
    pub(in crate::engine) fn pss_run_tran_internal(
        &self,
        circuit: &mut Circuit,
        matrix: &mut StaticMatrix,
        mut solution: Vec<Value>,
        tstop: Value,
        max_step: Value,
        fixed_grid: bool,
        mut trace: Option<&mut PssStateTrace>,
    ) -> Result<TransientResult, SimulationError> {
        let num_nodes = circuit.num_nodes();

        let fixed_steps = (tstop / max_step).round().max(1.0) as usize;
        let fixed_dt = tstop / fixed_steps as Value;

        let initial_step = (max_step / 10.0).min(tstop / 100.0);
        let mut timestep =
            TimestepController::new(initial_step, self.config.min_timestep, max_step);
        // Register source-waveform breakpoints (PULSE edges, PWL corners,
        // SIN delay starts) so the integrator lands on them instead of
        // stepping across; without this, hard-edged drives shift the PSS
        // orbit by up to one LTE-sized step per edge.
        let mut breakpoints = BreakpointManager::new();
        Self::collect_transient_source_breakpoints(
            circuit,
            tstop,
            max_step,
            self.config.spice_dialect,
            &mut breakpoints,
        );
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
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
        };

        let mut t = 0.0;
        const MAX_ITERATIONS: usize = 100_000;
        let mut total_iterations = 0;
        let mut first_step = true;

        if let Some(tr) = trace.as_deref_mut() {
            tr.times.push(0.0);
            tr.states.push(self.pss_extract_reactive_state(circuit));
            tr.solutions.push(solution.clone());
        }

        let mut fixed_index = 0usize;
        while t < tstop && total_iterations < MAX_ITERATIONS {
            total_iterations += 1;
            let dt = if fixed_grid {
                if fixed_index >= fixed_steps {
                    break;
                }
                // Anchor each step to the grid so rounding cannot drift the
                // endpoint: t_next = (i+1)*dt exactly.
                (fixed_index + 1) as Value * fixed_dt - t
            } else {
                let (dt, _) = breakpoints.limit_step(t, timestep.dt());
                dt.min(tstop - t)
            };

            // First step runs backward Euler: it reads no capacitor-current or
            // inductor-voltage history, so the trajectory depends only on the
            // shooting state that pss_set_reactive_state installed.
            let current_method = if first_step {
                crate::analysis::IntegrationMethod::BackwardEuler
            } else if fixed_grid {
                crate::analysis::IntegrationMethod::Trapezoidal
            } else {
                trapgear.current_method()
            };
            let coeff = CompanionCoefficients::for_method(current_method);

            let Some(new_solution) =
                self.pss_newton_solve(circuit, matrix, &coeff, t + dt, dt, &solution)?
            else {
                if fixed_grid {
                    // The grid is the contract: a Newton failure on it is a
                    // hard error rather than a silent step change that would
                    // destroy map smoothness.
                    return Err(SimulationError::ConvergenceFailed(total_iterations));
                }
                timestep.force_step(dt * 0.25);
                continue;
            };
            if fixed_grid {
                fixed_index += 1;
            }

            t += dt;
            first_step = false;

            // Update capacitor history with the same companion coefficients
            // that built this step, before shifting the voltage history those
            // coefficients read: i_{n+1} = geq*v_{n+1} - ieq(v_n, v_{n-1}, i_n).
            for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
                let np = cap.pp.row;
                let nn = cap.nn.row;
                let v_new = if np == 0 { 0.0 } else { new_solution[np - 1] }
                    - if nn == 0 { 0.0 } else { new_solution[nn - 1] };

                let capacitance = circuit.capacitors.capacitances[cap_idx];
                let geq = coeff.capacitor_geq(capacitance, dt);
                let ieq = coeff.capacitor_ieq(
                    capacitance,
                    dt,
                    circuit.capacitors.v_prev[cap_idx],
                    circuit.capacitors.v_prev_prev[cap_idx],
                    circuit.capacitors.i_prev[cap_idx],
                );
                circuit.capacitors.i_prev[cap_idx] = geq * v_new - ieq;
                circuit.capacitors.v_prev_prev[cap_idx] = circuit.capacitors.v_prev[cap_idx];
                circuit.capacitors.v_prev[cap_idx] = v_new;
            }

            lte_estimator.record(&new_solution, dt);
            trapgear.update(&new_solution, dt);

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

            if let Some(tr) = trace.as_deref_mut() {
                tr.times.push(t);
                tr.states.push(self.pss_extract_reactive_state(circuit));
                tr.solutions.push(solution.clone());
            }

            result.time.push(t);
            for (i, voltages) in result.voltages.iter_mut().enumerate() {
                voltages.push(solution.get(i).copied().unwrap_or(0.0));
            }

            if !fixed_grid {
                let (lte, _) = lte_estimator.estimate(&solution, dt);
                let scale = lte_estimator.recommend_scale(lte);
                timestep.adjust(lte / scale);
            }
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
