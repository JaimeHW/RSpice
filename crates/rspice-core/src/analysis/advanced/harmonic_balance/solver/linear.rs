//! Linear harmonic-balance assembly, source setup, and direct solve helpers.

use super::*;
use std::f64::consts::PI;

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
            nonlinear_devices: Vec::new(),
            #[cfg(feature = "veriloga")]
            veriloga_nonlinear_devices: Vec::new(),
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

    /// Add a single conductance stamp at position (node_i, node_j)
    ///
    /// This is a low-level method that adds a single Y-matrix entry.
    /// For a resistor between two nodes, use `add_resistor` instead.
    pub fn add_conductance(&mut self, node_i: usize, node_j: usize, g: Value) {
        self.g_matrix.push((node_i, node_j, g));
    }

    /// Add a resistor between two nodes with full MNA stamping
    ///
    /// For a resistor R between nodes i and j with G = 1/R:
    /// - Y(i,i) += G
    /// - Y(j,j) += G
    /// - Y(i,j) -= G
    /// - Y(j,i) -= G
    ///
    /// Ground is represented by a node index >= num_nodes (effectively ignored).
    pub fn add_resistor(&mut self, node_i: usize, node_j: usize, r: Value) {
        if r.abs() < 1e-30 {
            return; // Avoid division by zero
        }
        let g = 1.0 / r;

        // Full MNA stamp
        self.g_matrix.push((node_i, node_i, g));
        if node_j < self.num_nodes {
            self.g_matrix.push((node_j, node_j, g));
            self.g_matrix.push((node_i, node_j, -g));
            self.g_matrix.push((node_j, node_i, -g));
        }
    }

    /// Add a single capacitance stamp at position (node_i, node_j)
    ///
    /// This is a low-level method. For a capacitor between two nodes,
    /// the caller should add all 4 MNA stamps manually or use a higher-level API.
    pub fn add_capacitance(&mut self, node_i: usize, node_j: usize, c: Value) {
        self.c_matrix.push((node_i, node_j, c));
    }

    /// Add inductance stamp
    ///
    /// In frequency domain, inductor admittance is Y_L = 1/(jÏ‰L).
    /// At DC (Ï‰=0), inductor is short circuit (infinite admittance) - handled specially.
    /// At harmonic k: Y_L(k) = 1/(j * k * Ï‰â‚€ * L) = -j/(k * Ï‰â‚€ * L)
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

    /// Add voltage source with arbitrary AC harmonic entries.
    pub fn add_voltage_source_branch_harmonics(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        dc_voltage: Value,
        harmonics: &[(usize, Value, Value)],
    ) -> usize {
        let branch_idx = self.num_branches;
        let mut branch = VoltageSourceBranch::new(node_pos, node_neg, branch_idx, dc_voltage);
        for (harmonic, magnitude, phase) in harmonics {
            branch = branch.with_harmonic(*harmonic, *magnitude, *phase);
        }
        self.voltage_source_branches.push(branch);
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

    /// Add DC source current contribution at a node
    pub fn add_dc_source(&mut self, node: usize, current: Value) {
        if node < self.source_spectra.len() {
            self.source_spectra[node][0] += Complex64::new(current, 0.0);
        }
    }

    /// Set AC source at a node (sinusoidal at fundamental)
    pub fn set_ac_source(&mut self, node: usize, magnitude: Value, phase: Value) {
        self.set_harmonic_source(node, 1, magnitude, phase);
    }

    /// Add AC source contribution at the fundamental harmonic for a node
    pub fn add_ac_source(&mut self, node: usize, magnitude: Value, phase: Value) {
        self.add_harmonic_source(node, 1, magnitude, phase);
    }

    /// Set AC source contribution at an arbitrary harmonic for a node.
    pub fn set_harmonic_source(
        &mut self,
        node: usize,
        harmonic: usize,
        magnitude: Value,
        phase: Value,
    ) {
        if node < self.source_spectra.len() && harmonic < self.source_spectra[node].len() {
            self.source_spectra[node][harmonic] = Complex64::from_polar(magnitude, phase);
        }
    }

    /// Add AC source contribution at an arbitrary harmonic for a node.
    pub fn add_harmonic_source(
        &mut self,
        node: usize,
        harmonic: usize,
        magnitude: Value,
        phase: Value,
    ) {
        if node < self.source_spectra.len() && harmonic < self.source_spectra[node].len() {
            self.source_spectra[node][harmonic] += Complex64::from_polar(magnitude, phase);
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

    /// Compute residual for linear circuit (KCL: sum of currents INTO node = 0)
    ///
    /// Residual = I_source - (G*X + jÏ‰*C*X + (1/jÏ‰L)*X)
    ///          = I_source - Y*X
    ///
    /// For inductors, admittance Y_L = 1/(jÏ‰L) = -j/(Ï‰L)
    /// At DC (Ï‰=0): inductor is short circuit, requires special handling
    pub fn compute_linear_residual(&self, state: &mut HbSolverState) {
        let omega0 = 2.0 * PI * self.config.fundamental_freq;

        // Start with source currents (positive = current INTO node)
        for node_res in &mut state.residual {
            for c in node_res.iter_mut() {
                *c = Complex64::new(0.0, 0.0);
            }
        }

        // Add source contributions first
        for (node, source) in self.source_spectra.iter().enumerate() {
            if node < state.residual.len() {
                for (k, &s) in source.iter().enumerate() {
                    if k < state.residual[node].len() {
                        state.residual[node][k] += s; // Source current INTO node
                    }
                }
            }
        }

        // Subtract G*X contribution (current through conductance leaves node)
        for &(i, j, g) in &self.g_matrix {
            if i < state.x.len() && j < state.x.len() {
                for k in 0..=self.num_harmonics {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        state.residual[i][k] -= g * state.x[j][k];
                    }
                }
            }
        }

        // Subtract jÏ‰*C*X contribution (capacitor admittance current)
        for &(i, j, c) in &self.c_matrix {
            if i < state.x.len() && j < state.x.len() {
                for k in 0..=self.num_harmonics {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        let omega_k = (k as f64) * omega0;
                        let j_omega = Complex64::new(0.0, omega_k);
                        state.residual[i][k] -= j_omega * c * state.x[j][k];
                    }
                }
            }
        }

        // Subtract 1/(jÏ‰L)*X contribution (inductor admittance current)
        // Y_L = 1/(jÏ‰L) = -j/(Ï‰L)
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
                            state.residual[i][k] -= DC_SHORT_CONDUCTANCE * state.x[j][k];
                        } else {
                            // AC: Y_L = 1/(jÏ‰L) = -j/(Ï‰L)
                            let y_l = Complex64::new(0.0, -1.0 / (omega_k * l));
                            state.residual[i][k] -= y_l * state.x[j][k];
                        }
                    }
                }
            }
        }

        state.compute_residual_norm();
    }

    /// Compute Jacobian for linear circuit (block diagonal)
    ///
    /// J[node_i, k][node_j, l] = Î´_{kl} * (G_{ij} + jÏ‰_k * C_{ij} + 1/(jÏ‰_k * L_{ij}))
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

        // jÏ‰*C contribution (diagonal in harmonics)
        for &(i, j, c) in &self.c_matrix {
            if i < n && j < n {
                for k in 0..h {
                    let omega_k = (k as f64) * omega0;
                    let j_omega = Complex64::new(0.0, omega_k);
                    jac[i][k][j][k] += j_omega * c;
                }
            }
        }

        // 1/(jÏ‰L) contribution (diagonal in harmonics)
        for &(i, j, l) in &self.l_matrix {
            if i < n && j < n && l.abs() > 1e-30 {
                for k in 0..h {
                    let omega_k = (k as f64) * omega0;
                    if k == 0 {
                        // DC: short circuit (large conductance)
                        const DC_SHORT_CONDUCTANCE: Value = 1e6;
                        jac[i][k][j][k] += DC_SHORT_CONDUCTANCE;
                    } else {
                        // AC: Y_L = 1/(jÏ‰L) = -j/(Ï‰L)
                        let y_l = Complex64::new(0.0, -1.0 / (omega_k * l));
                        jac[i][k][j][k] += y_l;
                    }
                }
            }
        }

        jac
    }

    fn voltage_source_value_at_harmonic(
        branch: &VoltageSourceBranch,
        harmonic: usize,
    ) -> Complex64 {
        if harmonic == 0 {
            Complex64::new(branch.dc_voltage, 0.0)
        } else {
            branch
                .ac_harmonics
                .iter()
                .find_map(|(index, value)| (*index == harmonic).then_some(*value))
                .unwrap_or_else(|| Complex64::new(0.0, 0.0))
        }
    }

    fn compute_linear_residual_with_branches(
        &self,
        state: &mut HbSolverState,
        branch_currents: &[Vec<Complex64>],
    ) -> Value {
        let omega0 = 2.0 * PI * self.config.fundamental_freq;
        let h = self.num_harmonics + 1;

        for node_res in &mut state.residual {
            for c in node_res.iter_mut() {
                *c = Complex64::new(0.0, 0.0);
            }
        }

        // Start with nodal current source spectra.
        for (node, source) in self.source_spectra.iter().enumerate() {
            if node < state.residual.len() {
                for (k, &s) in source.iter().enumerate() {
                    if k < state.residual[node].len() {
                        state.residual[node][k] += s;
                    }
                }
            }
        }

        // Subtract linear passive contributions.
        for &(i, j, g) in &self.g_matrix {
            if i < state.x.len() && j < state.x.len() {
                for k in 0..h {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        state.residual[i][k] -= g * state.x[j][k];
                    }
                }
            }
        }
        for &(i, j, c) in &self.c_matrix {
            if i < state.x.len() && j < state.x.len() {
                for k in 0..h {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        let omega_k = (k as f64) * omega0;
                        state.residual[i][k] -= Complex64::new(0.0, omega_k) * c * state.x[j][k];
                    }
                }
            }
        }
        for &(i, j, l) in &self.l_matrix {
            if i < state.x.len() && j < state.x.len() && l.abs() > 1e-30 {
                for k in 0..h {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        if k == 0 {
                            const DC_SHORT_CONDUCTANCE: Value = 1e6;
                            state.residual[i][k] -= DC_SHORT_CONDUCTANCE * state.x[j][k];
                        } else {
                            let omega_k = (k as f64) * omega0;
                            let y_l = Complex64::new(0.0, -1.0 / (omega_k * l));
                            state.residual[i][k] -= y_l * state.x[j][k];
                        }
                    }
                }
            }
        }

        // Subtract MNA branch current coupling in nodal equations.
        for branch in &self.voltage_source_branches {
            let Some(currents) = branch_currents.get(branch.branch_idx) else {
                continue;
            };
            for k in 0..h {
                let ib = currents.get(k).copied().unwrap_or_default();
                if branch.node_pos > 0 && branch.node_pos - 1 < state.residual.len() {
                    state.residual[branch.node_pos - 1][k] -= ib;
                }
                if branch.node_neg > 0 && branch.node_neg - 1 < state.residual.len() {
                    state.residual[branch.node_neg - 1][k] += ib;
                }
            }
        }

        let mut residual_sum: Value = state
            .residual
            .iter()
            .flat_map(|node| node.iter())
            .map(|c| c.norm_sqr())
            .sum();

        // Include branch KVL residuals in the overall convergence norm.
        for branch in &self.voltage_source_branches {
            for k in 0..h {
                let mut v_drop = Complex64::new(0.0, 0.0);
                if branch.node_pos > 0 && branch.node_pos - 1 < state.x.len() {
                    v_drop += state.x[branch.node_pos - 1][k];
                }
                if branch.node_neg > 0 && branch.node_neg - 1 < state.x.len() {
                    v_drop -= state.x[branch.node_neg - 1][k];
                }
                let source_v = Self::voltage_source_value_at_harmonic(branch, k);
                let branch_residual = source_v - v_drop;
                residual_sum += branch_residual.norm_sqr();
            }
        }

        residual_sum.sqrt()
    }

    /// Solve for linear circuit (direct solve for diagonal harmonic blocks).
    ///
    /// Builds Y = G + jÏ‰C + 1/(jÏ‰L) and augments with MNA branch equations for
    /// ideal voltage sources when present.
    pub fn solve_linear(&self, state: &mut HbSolverState) -> Result<(), HbError> {
        let omega0 = 2.0 * PI * self.config.fundamental_freq;
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;
        let m = self.num_branches;
        let total_unknowns = n + m;

        let mut branch_currents = vec![vec![Complex64::new(0.0, 0.0); h]; m];

        // For each harmonic, solve an independent linear system.
        for k in 0..h {
            let omega_k = (k as f64) * omega0;
            let mut y_matrix = vec![vec![Complex64::new(0.0, 0.0); total_unknowns]; total_unknowns];
            let mut rhs = vec![Complex64::new(0.0, 0.0); total_unknowns];

            for &(i, j, g) in &self.g_matrix {
                if i < n && j < n {
                    y_matrix[i][j] += g;
                }
            }

            for &(i, j, c) in &self.c_matrix {
                if i < n && j < n {
                    y_matrix[i][j] += Complex64::new(0.0, omega_k) * c;
                }
            }

            for &(i, j, l) in &self.l_matrix {
                if i < n && j < n && l.abs() > 1e-30 {
                    if k == 0 {
                        const DC_SHORT_CONDUCTANCE: Value = 1e6;
                        y_matrix[i][j] += DC_SHORT_CONDUCTANCE;
                    } else {
                        let y_l = Complex64::new(0.0, -1.0 / (omega_k * l));
                        y_matrix[i][j] += y_l;
                    }
                }
            }

            for node in 0..n {
                rhs[node] = self
                    .source_spectra
                    .get(node)
                    .and_then(|s| s.get(k))
                    .copied()
                    .unwrap_or_default();
            }

            // MNA branch equations for ideal voltage sources.
            for branch in &self.voltage_source_branches {
                let row = n + branch.branch_idx;
                if row >= total_unknowns {
                    continue;
                }

                if branch.node_pos > 0 && branch.node_pos - 1 < n {
                    let np = branch.node_pos - 1;
                    y_matrix[np][row] += Complex64::new(1.0, 0.0);
                    y_matrix[row][np] += Complex64::new(1.0, 0.0);
                }
                if branch.node_neg > 0 && branch.node_neg - 1 < n {
                    let nn = branch.node_neg - 1;
                    y_matrix[nn][row] -= Complex64::new(1.0, 0.0);
                    y_matrix[row][nn] -= Complex64::new(1.0, 0.0);
                }

                rhs[row] = Self::voltage_source_value_at_harmonic(branch, k);
            }

            let solution = self.solve_complex_linear_system(&y_matrix, &rhs)?;

            for node in 0..n {
                if node < state.x.len() && k < state.x[node].len() {
                    state.x[node][k] = solution[node];
                }
            }
            for branch_idx in 0..m {
                let col = n + branch_idx;
                if col < solution.len() && branch_idx < branch_currents.len() {
                    branch_currents[branch_idx][k] = solution[col];
                }
            }
        }

        state.residual_norm = if m == 0 {
            self.compute_linear_residual(state);
            state.residual_norm
        } else {
            self.compute_linear_residual_with_branches(state, &branch_currents)
        };
        state.converged = state.residual_norm < self.config.tolerance;

        Ok(())
    }
}
