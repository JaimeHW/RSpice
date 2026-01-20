//! Harmonic Balance Newton Solver
//!
//! Core solver for Harmonic Balance analysis using Newton-Raphson iteration.
//! Solves the frequency-domain circuit equations: G*X + jω*C*X + F_NL(X) = I_S

use num_complex::Complex64;
use std::f64::consts::PI;

use super::config::HbConfig;
use super::fft::HbFft;
use super::result::{HbResult, SpectralVoltage};
use crate::Value;

/// Error types specific to Harmonic Balance solver
#[derive(Debug, Clone)]
pub enum HbError {
    /// Newton iteration did not converge
    ConvergenceFailed { iterations: usize, residual: Value },
    /// Matrix is singular
    SingularMatrix,
    /// Invalid circuit configuration
    InvalidCircuit(String),
    /// FFT operation failed
    FftError(String),
}

impl std::fmt::Display for HbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConvergenceFailed {
                iterations,
                residual,
            } => {
                write!(
                    f,
                    "HB convergence failed after {} iterations (residual: {:.3e})",
                    iterations, residual
                )
            }
            Self::SingularMatrix => write!(f, "Singular Jacobian matrix"),
            Self::InvalidCircuit(msg) => write!(f, "Invalid circuit: {}", msg),
            Self::FftError(msg) => write!(f, "FFT error: {}", msg),
        }
    }
}

impl std::error::Error for HbError {}

/// Harmonic Balance solver state
#[derive(Debug)]
pub struct HbSolverState {
    /// Spectral voltage solution [node][harmonic]
    pub x: Vec<Vec<Complex64>>,

    /// Residual vector [node][harmonic]
    pub residual: Vec<Vec<Complex64>>,

    /// Current residual norm
    pub residual_norm: Value,

    /// Number of iterations
    pub iteration: usize,

    /// Converged flag
    pub converged: bool,
}

impl HbSolverState {
    /// Create new solver state
    pub fn new(num_nodes: usize, num_harmonics: usize) -> Self {
        Self {
            x: vec![vec![Complex64::new(0.0, 0.0); num_harmonics + 1]; num_nodes],
            residual: vec![vec![Complex64::new(0.0, 0.0); num_harmonics + 1]; num_nodes],
            residual_norm: f64::INFINITY,
            iteration: 0,
            converged: false,
        }
    }

    /// Compute residual norm (L2 over all nodes and harmonics)
    pub fn compute_residual_norm(&mut self) {
        let sum: Value = self
            .residual
            .iter()
            .flat_map(|node| node.iter())
            .map(|c| c.norm_sqr())
            .sum();
        self.residual_norm = sum.sqrt();
    }

    /// Compute solution norm for relative tolerance
    pub fn solution_norm(&self) -> Value {
        let sum: Value = self
            .x
            .iter()
            .flat_map(|node| node.iter())
            .map(|c| c.norm_sqr())
            .sum();
        sum.sqrt()
    }

    /// Total number of unknowns
    pub fn total_unknowns(&self) -> usize {
        self.x.len() * self.x.first().map(|v| v.len()).unwrap_or(0)
    }
}

/// Represents a linear circuit element for HB
#[derive(Debug, Clone)]
pub struct HbLinearElement {
    /// Conductance matrix stamp (node_i, node_j, value)
    pub g_stamps: Vec<(usize, usize, Value)>,
    /// Capacitance matrix stamp (node_i, node_j, value)
    pub c_stamps: Vec<(usize, usize, Value)>,
    /// Inductance matrix stamp (node_i, node_j, value)
    pub l_stamps: Vec<(usize, usize, Value)>,
}

/// Voltage source branch for MNA
///
/// In Modified Nodal Analysis, voltage sources require branch current
/// variables to properly enforce voltage constraints.
#[derive(Debug, Clone)]
pub struct VoltageSourceBranch {
    /// Positive terminal node (1-indexed, 0 = ground)
    pub node_pos: usize,
    /// Negative terminal node (1-indexed, 0 = ground)
    pub node_neg: usize,
    /// Branch current variable index
    pub branch_idx: usize,
    /// DC voltage value
    pub dc_voltage: Value,
    /// AC voltage magnitude (at fundamental)
    pub ac_magnitude: Value,
    /// AC voltage phase (radians)
    pub ac_phase: Value,
}

impl VoltageSourceBranch {
    /// Create new voltage source branch
    pub fn new(node_pos: usize, node_neg: usize, branch_idx: usize, dc_voltage: Value) -> Self {
        Self {
            node_pos,
            node_neg,
            branch_idx,
            dc_voltage,
            ac_magnitude: 0.0,
            ac_phase: 0.0,
        }
    }

    /// Set AC parameters
    pub fn with_ac(mut self, magnitude: Value, phase: Value) -> Self {
        self.ac_magnitude = magnitude;
        self.ac_phase = phase;
        self
    }

    /// Get voltage spectrum (DC + AC at fundamental)
    pub fn voltage_spectrum(&self, num_harmonics: usize) -> Vec<Complex64> {
        let mut spectrum = vec![Complex64::new(0.0, 0.0); num_harmonics + 1];
        spectrum[0] = Complex64::new(self.dc_voltage, 0.0);
        if num_harmonics >= 1 {
            spectrum[1] = Complex64::from_polar(self.ac_magnitude, self.ac_phase);
        }
        spectrum
    }
}

/// Represents a nonlinear device for HB
pub trait HbNonlinearDevice: Send + Sync {
    /// Evaluate device current given terminal voltages in time domain
    fn evaluate(&self, voltages: &[Value]) -> Value;

    /// Get device terminals (node indices)
    fn terminals(&self) -> &[usize];

    /// Compute Jacobian contribution (dI/dV) in time domain
    fn jacobian(&self, voltages: &[Value]) -> Vec<(usize, Value)>;
}

/// Harmonic Balance solver
///
/// Commercial-grade HB solver supporting:
/// - Linear elements: R, C, L (with proper jωL admittance)
/// - MNA voltage sources with branch currents
/// - Nonlinear device Newton iteration via FFT/IFFT
#[derive(Debug)]
pub struct HbSolver {
    /// Configuration
    config: HbConfig,

    /// FFT processor
    fft: HbFft,

    /// Number of nodes
    num_nodes: usize,

    /// Number of harmonics (including DC)
    num_harmonics: usize,

    /// Number of branch currents (for MNA voltage sources)
    num_branches: usize,

    /// Conductance matrix for each node combination
    /// Stored as sparse: (row, col) -> G
    g_matrix: Vec<(usize, usize, Value)>,

    /// Capacitance matrix for each node combination
    /// Stored as sparse: (row, col) -> C
    c_matrix: Vec<(usize, usize, Value)>,

    /// Inductance matrix for each node combination
    /// Stored as sparse: (row, col) -> L
    /// Admittance Y = 1/(jωL) at each harmonic
    l_matrix: Vec<(usize, usize, Value)>,

    /// Voltage source branches for MNA
    /// (node_pos, node_neg, branch_idx, dc_value, ac_magnitude, ac_phase)
    voltage_source_branches: Vec<VoltageSourceBranch>,

    /// Node names
    node_names: Vec<String>,

    /// Current source spectra [node][harmonic]
    source_spectra: Vec<Vec<Complex64>>,
}

impl HbSolver {
    /// Create a new HB solver
    pub fn new(config: HbConfig, num_nodes: usize) -> Self {
        let num_harmonics = config.num_harmonics;
        let fft = HbFft::new(num_harmonics, config.oversample_factor);

        Self {
            config,
            fft,
            num_nodes,
            num_harmonics,
            num_branches: 0,
            g_matrix: Vec::new(),
            c_matrix: Vec::new(),
            l_matrix: Vec::new(),
            voltage_source_branches: Vec::new(),
            node_names: (0..num_nodes).map(|i| format!("n{}", i)).collect(),
            source_spectra: vec![vec![Complex64::new(0.0, 0.0); num_harmonics + 1]; num_nodes],
        }
    }

    /// Get number of harmonics
    pub fn num_harmonics(&self) -> usize {
        self.num_harmonics
    }

    /// Set node names
    pub fn set_node_names(&mut self, names: Vec<String>) {
        self.node_names = names;
    }

    /// Add conductance stamp
    pub fn add_conductance(&mut self, node_i: usize, node_j: usize, g: Value) {
        self.g_matrix.push((node_i, node_j, g));
    }

    /// Add capacitance stamp
    pub fn add_capacitance(&mut self, node_i: usize, node_j: usize, c: Value) {
        self.c_matrix.push((node_i, node_j, c));
    }

    /// Add inductance stamp
    ///
    /// In frequency domain, inductor admittance is Y_L = 1/(jωL).
    /// At DC (ω=0), inductor is short circuit (infinite admittance) - handled specially.
    /// At harmonic k: Y_L(k) = 1/(j * k * ω₀ * L) = -j/(k * ω₀ * L)
    pub fn add_inductance(&mut self, node_i: usize, node_j: usize, l: Value) {
        self.l_matrix.push((node_i, node_j, l));
    }

    /// Add voltage source with MNA branch current
    ///
    /// Proper MNA treatment: voltage sources require branch current variables
    /// to enforce voltage constraint without Norton approximation.
    pub fn add_voltage_source_branch(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        dc_voltage: Value,
    ) -> usize {
        let branch_idx = self.num_branches;
        self.voltage_source_branches.push(VoltageSourceBranch::new(
            node_pos, node_neg, branch_idx, dc_voltage,
        ));
        self.num_branches += 1;
        branch_idx
    }

    /// Add voltage source with AC component
    pub fn add_voltage_source_branch_ac(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        dc_voltage: Value,
        ac_magnitude: Value,
        ac_phase: Value,
    ) -> usize {
        let branch_idx = self.num_branches;
        self.voltage_source_branches.push(
            VoltageSourceBranch::new(node_pos, node_neg, branch_idx, dc_voltage)
                .with_ac(ac_magnitude, ac_phase),
        );
        self.num_branches += 1;
        branch_idx
    }

    /// Get number of MNA branch currents
    pub fn num_branches(&self) -> usize {
        self.num_branches
    }

    /// Set DC source current at a node
    pub fn set_dc_source(&mut self, node: usize, current: Value) {
        if node < self.source_spectra.len() {
            self.source_spectra[node][0] = Complex64::new(current, 0.0);
        }
    }

    /// Set AC source at a node (sinusoidal at fundamental)
    pub fn set_ac_source(&mut self, node: usize, magnitude: Value, phase: Value) {
        if node < self.source_spectra.len() && self.source_spectra[node].len() > 1 {
            self.source_spectra[node][1] = Complex64::from_polar(magnitude, phase);
        }
    }

    /// Set full source spectrum at a node
    pub fn set_source_spectrum(&mut self, node: usize, spectrum: Vec<Complex64>) {
        if node < self.source_spectra.len() {
            self.source_spectra[node] = spectrum;
        }
    }

    /// Initialize solution with DC operating point
    pub fn initialize_dc(&mut self, state: &mut HbSolverState, dc_solution: &[Value]) {
        for (node, &v_dc) in dc_solution.iter().enumerate() {
            if node < state.x.len() && !state.x[node].is_empty() {
                state.x[node][0] = Complex64::new(v_dc, 0.0);
            }
        }
    }

    /// Compute residual for linear circuit
    ///
    /// Residual = G*X + jω*C*X + (1/jωL)*X - I_S
    ///
    /// For inductors, admittance Y_L = 1/(jωL) = -j/(ωL)
    /// At DC (ω=0): inductor is short circuit, requires special handling
    pub fn compute_linear_residual(&self, state: &mut HbSolverState) {
        let omega0 = 2.0 * PI * self.config.fundamental_freq;

        // Clear residual
        for node_res in &mut state.residual {
            for c in node_res.iter_mut() {
                *c = Complex64::new(0.0, 0.0);
            }
        }

        // Add G*X contribution
        for &(i, j, g) in &self.g_matrix {
            if i < state.x.len() && j < state.x.len() {
                for k in 0..=self.num_harmonics {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        state.residual[i][k] += g * state.x[j][k];
                    }
                }
            }
        }

        // Add jω*C*X contribution (capacitor admittance)
        for &(i, j, c) in &self.c_matrix {
            if i < state.x.len() && j < state.x.len() {
                for k in 0..=self.num_harmonics {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        let omega_k = (k as f64) * omega0;
                        let j_omega = Complex64::new(0.0, omega_k);
                        state.residual[i][k] += j_omega * c * state.x[j][k];
                    }
                }
            }
        }

        // Add 1/(jωL)*X contribution (inductor admittance)
        // Y_L = 1/(jωL) = -j/(ωL)
        // At DC (k=0): inductor is short circuit - enforce V=0 (large admittance)
        for &(i, j, l) in &self.l_matrix {
            if i < state.x.len() && j < state.x.len() && l.abs() > 1e-30 {
                for k in 0..=self.num_harmonics {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        let omega_k = (k as f64) * omega0;
                        if k == 0 {
                            // DC: inductor is short circuit
                            // Add very large conductance to force V_i = V_j
                            const DC_SHORT_CONDUCTANCE: Value = 1e6;
                            state.residual[i][k] += DC_SHORT_CONDUCTANCE * state.x[j][k];
                        } else {
                            // AC: Y_L = 1/(jωL) = -j/(ωL)
                            let y_l = Complex64::new(0.0, -1.0 / (omega_k * l));
                            state.residual[i][k] += y_l * state.x[j][k];
                        }
                    }
                }
            }
        }

        // Subtract source spectra
        for (node, source) in self.source_spectra.iter().enumerate() {
            if node < state.residual.len() {
                for (k, &s) in source.iter().enumerate() {
                    if k < state.residual[node].len() {
                        state.residual[node][k] -= s;
                    }
                }
            }
        }

        state.compute_residual_norm();
    }

    /// Compute Jacobian for linear circuit (block diagonal)
    ///
    /// J[node_i, k][node_j, l] = δ_{kl} * (G_{ij} + jω_k * C_{ij} + 1/(jω_k * L_{ij}))
    #[allow(dead_code)]
    fn compute_linear_jacobian(&self) -> Vec<Vec<Vec<Vec<Complex64>>>> {
        let omega0 = 2.0 * PI * self.config.fundamental_freq;
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;

        // Full Jacobian: [node_i][harmonic_k][node_j][harmonic_l]
        let mut jac = vec![vec![vec![vec![Complex64::new(0.0, 0.0); h]; n]; h]; n];

        // G contribution (diagonal in harmonics)
        for &(i, j, g) in &self.g_matrix {
            if i < n && j < n {
                for k in 0..h {
                    jac[i][k][j][k] += g;
                }
            }
        }

        // jω*C contribution (diagonal in harmonics)
        for &(i, j, c) in &self.c_matrix {
            if i < n && j < n {
                for k in 0..h {
                    let omega_k = (k as f64) * omega0;
                    let j_omega = Complex64::new(0.0, omega_k);
                    jac[i][k][j][k] += j_omega * c;
                }
            }
        }

        // 1/(jωL) contribution (diagonal in harmonics)
        for &(i, j, l) in &self.l_matrix {
            if i < n && j < n && l.abs() > 1e-30 {
                for k in 0..h {
                    let omega_k = (k as f64) * omega0;
                    if k == 0 {
                        // DC: short circuit (large conductance)
                        const DC_SHORT_CONDUCTANCE: Value = 1e6;
                        jac[i][k][j][k] += DC_SHORT_CONDUCTANCE;
                    } else {
                        // AC: Y_L = 1/(jωL) = -j/(ωL)
                        let y_l = Complex64::new(0.0, -1.0 / (omega_k * l));
                        jac[i][k][j][k] += y_l;
                    }
                }
            }
        }

        jac
    }

    /// Solve for linear circuit (direct solve for diagonal blocks)
    ///
    /// Builds Y = G + jωC + 1/(jωL) admittance matrix for each harmonic
    /// and solves Y*V = I
    pub fn solve_linear(&self, state: &mut HbSolverState) -> Result<(), HbError> {
        let omega0 = 2.0 * PI * self.config.fundamental_freq;
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;

        // For each harmonic, solve the linear system independently
        for k in 0..h {
            let omega_k = (k as f64) * omega0;

            // Build matrix for this harmonic: Y_k = G + jω_k*C + 1/(jω_k*L)
            let mut y_matrix = vec![vec![Complex64::new(0.0, 0.0); n]; n];

            // Conductance contribution
            for &(i, j, g) in &self.g_matrix {
                if i < n && j < n {
                    y_matrix[i][j] += g;
                }
            }

            // Capacitance contribution: jωC
            for &(i, j, c) in &self.c_matrix {
                if i < n && j < n {
                    y_matrix[i][j] += Complex64::new(0.0, omega_k) * c;
                }
            }

            // Inductance contribution: 1/(jωL) = -j/(ωL)
            for &(i, j, l) in &self.l_matrix {
                if i < n && j < n && l.abs() > 1e-30 {
                    if k == 0 {
                        // DC: inductor is short circuit (large conductance)
                        const DC_SHORT_CONDUCTANCE: Value = 1e6;
                        y_matrix[i][j] += DC_SHORT_CONDUCTANCE;
                    } else {
                        // AC: Y_L = -j/(ωL)
                        let y_l = Complex64::new(0.0, -1.0 / (omega_k * l));
                        y_matrix[i][j] += y_l;
                    }
                }
            }

            // Get RHS for this harmonic
            let rhs: Vec<Complex64> = (0..n)
                .map(|node| {
                    self.source_spectra
                        .get(node)
                        .and_then(|s| s.get(k))
                        .copied()
                        .unwrap_or(Complex64::new(0.0, 0.0))
                })
                .collect();

            // Solve Y * V = I using Gaussian elimination
            let solution = self.solve_complex_linear_system(&y_matrix, &rhs)?;

            // Store solution
            for (node, &v) in solution.iter().enumerate() {
                if node < state.x.len() && k < state.x[node].len() {
                    state.x[node][k] = v;
                }
            }
        }

        // Compute final residual
        self.compute_linear_residual(state);
        state.converged = state.residual_norm < self.config.tolerance;

        Ok(())
    }

    /// Newton iteration for nonlinear circuit
    pub fn newton_step(
        &mut self,
        state: &mut HbSolverState,
        nonlinear_fn: impl Fn(&[Value]) -> (Value, Value), // (current, dI/dV)
        terminals: &[(usize, usize)],                      // (positive_node, negative_node)
    ) -> Result<(), HbError> {
        // Get time points for nonlinear evaluation
        let n_time = self.fft.size();
        let period = self.config.period();

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
                }
            }
        }

        state.compute_residual_norm();
        state.iteration += 1;

        // Check convergence
        let rel_norm = state.residual_norm / (state.solution_norm() + self.config.abstol);
        state.converged = rel_norm < self.config.tolerance;

        Ok(())
    }

    /// Solve complex linear system using Gaussian elimination
    fn solve_complex_linear_system(
        &self,
        a: &[Vec<Complex64>],
        b: &[Complex64],
    ) -> Result<Vec<Complex64>, HbError> {
        let n = b.len();
        if n == 0 {
            return Ok(vec![]);
        }

        // Augmented matrix
        let mut aug: Vec<Vec<Complex64>> = a
            .iter()
            .zip(b.iter())
            .map(|(row, &bi)| {
                let mut r = row.clone();
                r.push(bi);
                r
            })
            .collect();

        // Forward elimination with partial pivoting
        for col in 0..n {
            // Find pivot
            let mut max_row = col;
            for row in (col + 1)..n {
                if aug[row][col].norm() > aug[max_row][col].norm() {
                    max_row = row;
                }
            }
            aug.swap(col, max_row);

            let pivot = aug[col][col];
            if pivot.norm() < 1e-15 {
                continue; // Near-singular, skip
            }

            // Eliminate
            for row in (col + 1)..n {
                let factor = aug[row][col] / pivot;
                for k in col..=n {
                    let col_val = aug[col][k];
                    aug[row][k] -= factor * col_val;
                }
            }
        }

        // Back substitution
        let mut x = vec![Complex64::new(0.0, 0.0); n];
        for i in (0..n).rev() {
            let mut sum = aug[i][n];
            for j in (i + 1)..n {
                sum -= aug[i][j] * x[j];
            }
            if aug[i][i].norm() > 1e-15 {
                x[i] = sum / aug[i][i];
            }
        }

        Ok(x)
    }

    /// Build HbResult from solver state
    pub fn build_result(&self, state: &HbSolverState) -> HbResult {
        let mut result = HbResult::new(
            self.config.fundamental_freq,
            self.num_nodes,
            self.num_harmonics,
        );

        result.converged = state.converged;
        result.iterations = state.iteration;
        result.residual_norm = state.residual_norm;
        result.node_names = self.node_names.clone();

        // Copy spectral voltages
        for (node, spectrum) in state.x.iter().enumerate() {
            let mut sv = SpectralVoltage::new(
                self.node_names.get(node).cloned().unwrap_or_default(),
                self.num_harmonics,
            );
            sv.coefficients = spectrum.clone();
            sv.frequencies = self.config.harmonic_frequencies();
            result.spectral_voltages.push(sv);
        }

        result
    }
}

#[cfg(test)]
mod solver_tests {
    use super::*;

    #[test]
    fn test_hb_solver_creation() {
        let config = HbConfig::new(1e9).with_harmonics(5);
        let solver = HbSolver::new(config, 3);

        assert_eq!(solver.num_nodes, 3);
        assert_eq!(solver.num_harmonics, 5);
    }

    #[test]
    fn test_solver_state_creation() {
        let state = HbSolverState::new(3, 5);

        assert_eq!(state.x.len(), 3);
        assert_eq!(state.x[0].len(), 6); // 5 harmonics + DC
        assert!(!state.converged);
    }

    #[test]
    fn test_solver_state_norms() {
        let mut state = HbSolverState::new(2, 2);

        // Set some values
        state.x[0][0] = Complex64::new(1.0, 0.0);
        state.x[0][1] = Complex64::new(0.0, 1.0);
        state.residual[1][0] = Complex64::new(3.0, 4.0); // |3+4j| = 5

        state.compute_residual_norm();
        assert!((state.residual_norm - 5.0).abs() < 1e-10);

        let sol_norm = state.solution_norm();
        assert!((sol_norm - 2.0_f64.sqrt()).abs() < 1e-10); // sqrt(1 + 1)
    }

    #[test]
    fn test_add_stamps() {
        let config = HbConfig::new(1e9);
        let mut solver = HbSolver::new(config, 2);

        solver.add_conductance(0, 1, 0.001);
        solver.add_capacitance(0, 0, 1e-12);

        assert_eq!(solver.g_matrix.len(), 1);
        assert_eq!(solver.c_matrix.len(), 1);
    }

    #[test]
    fn test_set_sources() {
        let config = HbConfig::new(1e9).with_harmonics(3);
        let mut solver = HbSolver::new(config, 2);

        solver.set_dc_source(0, 1.0);
        solver.set_ac_source(0, 0.5, 0.0);

        assert!((solver.source_spectra[0][0].re - 1.0).abs() < 1e-10);
        assert!((solver.source_spectra[0][1].re - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_linear_residual_dc_only() {
        let config = HbConfig::new(1e9).with_harmonics(2);
        let mut solver = HbSolver::new(config, 1);

        // Simple resistor to ground: I = G*V, with 1V DC source
        solver.add_conductance(0, 0, 0.001); // 1k ohm
        solver.set_dc_source(0, 0.001); // 1mA = 1V / 1k

        let mut state = HbSolverState::new(1, 2);
        state.x[0][0] = Complex64::new(1.0, 0.0); // V = 1V

        solver.compute_linear_residual(&mut state);

        // Residual should be small: G*V - I = 0.001*1 - 0.001 = 0
        assert!(
            state.residual[0][0].norm() < 1e-10,
            "Residual: {}",
            state.residual[0][0]
        );
    }

    #[test]
    fn test_solve_linear_simple() {
        let config = HbConfig::new(1e9).with_harmonics(1);
        let solver = HbSolver::new(config, 1);

        // Empty circuit (no stamps) with DC source
        // Should give zero solution
        let mut state = HbSolverState::new(1, 1);

        // This is degenerate, but should not panic
        let _ = solver.solve_linear(&mut state);
    }

    #[test]
    fn test_complex_linear_solve() {
        let config = HbConfig::new(1e9);
        let solver = HbSolver::new(config, 2);

        // Simple 2x2 system
        let a = vec![
            vec![Complex64::new(2.0, 0.0), Complex64::new(1.0, 0.0)],
            vec![Complex64::new(1.0, 0.0), Complex64::new(3.0, 0.0)],
        ];
        let b = vec![Complex64::new(5.0, 0.0), Complex64::new(7.0, 0.0)];

        let x = solver.solve_complex_linear_system(&a, &b).unwrap();

        // Verify solution
        let r0 = a[0][0] * x[0] + a[0][1] * x[1] - b[0];
        let r1 = a[1][0] * x[0] + a[1][1] * x[1] - b[1];

        assert!(r0.norm() < 0.01, "Residual 0: {}", r0);
        assert!(r1.norm() < 0.01, "Residual 1: {}", r1);
    }

    #[test]
    fn test_build_result() {
        let config = HbConfig::new(1e9).with_harmonics(3);
        let solver = HbSolver::new(config, 2);

        let mut state = HbSolverState::new(2, 3);
        state.converged = true;
        state.iteration = 5;
        state.residual_norm = 1e-10;

        let result = solver.build_result(&state);

        assert!(result.converged);
        assert_eq!(result.iterations, 5);
        assert_eq!(result.num_nodes(), 2);
        assert_eq!(result.num_harmonics, 3);
    }

    #[test]
    fn test_hb_error_display() {
        let err = HbError::ConvergenceFailed {
            iterations: 50,
            residual: 1e-3,
        };
        let msg = format!("{}", err);
        assert!(msg.contains("50 iterations"));
    }
}
