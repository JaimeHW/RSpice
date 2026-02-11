use super::filters::DelayBuffer;
use crate::codegen::LookupTable;
use crate::laplace::StateSpaceFilter;

/// Execution context providing runtime state to the VM.
#[derive(Debug, Clone)]
pub struct VmContext {
    /// Node voltages (indexed by terminal)
    pub voltages: Vec<f64>,
    /// Internal node voltages (for nodes not in port list)
    pub internal_voltages: Vec<f64>,
    /// Branch currents
    pub currents: Vec<f64>,
    /// Terminal-pair branch current lookup table (flattened NxN matrix).
    terminal_pair_currents: Vec<f64>,
    /// Parameter values (indexed by parameter)
    pub parameters: Vec<f64>,
    /// Variable values (indexed by variable)
    pub variables: Vec<f64>,
    /// Current simulation time
    pub time: f64,
    /// Temperature in Kelvin
    pub temperature: f64,
    /// State variable values (current timestep) - for ddt/idt
    pub state_values: Vec<f64>,
    /// State variable values (previous timestep) - for ddt/idt
    pub state_values_prev: Vec<f64>,
    /// Current timestep (delta t) for transient analysis
    pub timestep: f64,
    /// Lookup tables for $table_model interpolation
    pub lookup_tables: Vec<LookupTable>,
    /// Delay buffers for absdelay function
    /// Each buffer stores (time, value) pairs for interpolation
    pub delay_buffers: Vec<DelayBuffer>,
    /// Current analysis type (0=dc, 1=ac, 2=tran, 3=noise)
    pub analysis_type: u8,
    /// Laplace state-space filters
    pub laplace_filters: Vec<StateSpaceFilter>,
}

impl Default for VmContext {
    fn default() -> Self {
        Self {
            voltages: Vec::new(),
            internal_voltages: Vec::new(),
            currents: Vec::new(),
            terminal_pair_currents: Vec::new(),
            parameters: Vec::new(),
            variables: Vec::new(),
            time: 0.0,
            temperature: 300.15, // 27C default
            state_values: Vec::new(),
            state_values_prev: Vec::new(),
            timestep: 0.0,
            lookup_tables: Vec::new(),
            delay_buffers: Vec::new(),
            analysis_type: 0, // DC by default
            laplace_filters: Vec::new(),
        }
    }
}

impl VmContext {
    /// Create a new VM context with specified terminal count.
    pub fn new(num_terminals: usize) -> Self {
        Self {
            voltages: vec![0.0; num_terminals],
            internal_voltages: Vec::new(),
            currents: Vec::new(),
            terminal_pair_currents: vec![f64::NAN; num_terminals.saturating_mul(num_terminals)],
            parameters: Vec::new(),
            variables: Vec::new(),
            time: 0.0,
            temperature: 300.15,
            state_values: Vec::new(),
            state_values_prev: Vec::new(),
            timestep: 0.0,
            lookup_tables: Vec::new(),
            delay_buffers: Vec::new(),
            analysis_type: 0,
            laplace_filters: Vec::new(),
        }
    }

    /// Create with internal nodes.
    pub fn with_internal_nodes(num_terminals: usize, num_internal: usize) -> Self {
        Self {
            voltages: vec![0.0; num_terminals],
            internal_voltages: vec![0.0; num_internal],
            currents: Vec::new(),
            terminal_pair_currents: vec![f64::NAN; num_terminals.saturating_mul(num_terminals)],
            parameters: Vec::new(),
            variables: Vec::new(),
            time: 0.0,
            temperature: 300.15,
            state_values: Vec::new(),
            state_values_prev: Vec::new(),
            timestep: 0.0,
            lookup_tables: Vec::new(),
            delay_buffers: Vec::new(),
            analysis_type: 0,
            laplace_filters: Vec::new(),
        }
    }

    /// Create with state variables for transient analysis.
    pub fn with_states(num_terminals: usize, num_states: usize) -> Self {
        Self {
            voltages: vec![0.0; num_terminals],
            internal_voltages: Vec::new(),
            currents: Vec::new(),
            terminal_pair_currents: vec![f64::NAN; num_terminals.saturating_mul(num_terminals)],
            parameters: Vec::new(),
            variables: Vec::new(),
            time: 0.0,
            temperature: 300.15,
            state_values: vec![0.0; num_states],
            state_values_prev: vec![0.0; num_states],
            timestep: 0.0,
            lookup_tables: Vec::new(),
            delay_buffers: Vec::new(),
            analysis_type: 0,
            laplace_filters: Vec::new(),
        }
    }

    /// Advance state for a new timestep (copy current to prev).
    pub fn advance_state(&mut self) {
        self.state_values_prev.clone_from(&self.state_values);
    }

    /// Set the timestep for transient analysis.
    pub fn set_timestep(&mut self, dt: f64) {
        self.timestep = dt;
    }

    /// Allocate state variables.
    pub fn allocate_states(&mut self, count: usize) {
        self.state_values.resize(count, 0.0);
        self.state_values_prev.resize(count, 0.0);
    }

    /// Clear cached branch current values.
    pub fn clear_currents(&mut self) {
        self.currents.clear();
        self.terminal_pair_currents.fill(f64::NAN);
    }

    /// Set branch current from `pos` to `neg`.
    ///
    /// Also populates the reverse direction with opposite sign.
    pub fn set_branch_current(&mut self, pos: usize, neg: usize, value: f64) {
        let n = self.voltages.len();
        if n == 0 || pos >= n || neg >= n {
            return;
        }

        if self.terminal_pair_currents.len() != n.saturating_mul(n) {
            self.terminal_pair_currents
                .resize(n.saturating_mul(n), f64::NAN);
        }

        let idx = pos * n + neg;
        self.terminal_pair_currents[idx] = value;

        if pos != neg {
            let reverse_idx = neg * n + pos;
            self.terminal_pair_currents[reverse_idx] = -value;
        }
    }

    /// Get the voltage difference between two terminals.
    #[inline]
    pub fn voltage(&self, pos: usize, neg: usize) -> f64 {
        let v_pos = self.voltages.get(pos).copied().unwrap_or(0.0);
        let v_neg = self.voltages.get(neg).copied().unwrap_or(0.0);
        v_pos - v_neg
    }

    /// Get internal node voltage.
    #[inline]
    pub fn internal_voltage(&self, idx: usize) -> f64 {
        self.internal_voltages.get(idx).copied().unwrap_or(0.0)
    }

    /// Get current between terminals.
    #[inline]
    pub fn current(&self, pos: usize, neg: usize) -> f64 {
        let n = self.voltages.len();
        if pos < n && neg < n {
            let idx = pos * n + neg;
            if idx < self.terminal_pair_currents.len() {
                let value = self.terminal_pair_currents[idx];
                if value.is_finite() {
                    return value;
                }
            }
        }
        self.currents.first().copied().unwrap_or(0.0)
    }

    /// Get pointer to the terminal-pair current lookup buffer.
    #[inline]
    pub fn terminal_pair_currents_ptr(&self) -> *const f64 {
        self.terminal_pair_currents.as_ptr()
    }

    /// Get length of the terminal-pair current lookup buffer.
    #[inline]
    pub fn terminal_pair_currents_len(&self) -> usize {
        self.terminal_pair_currents.len()
    }

    /// Get number of terminals in this context.
    #[inline]
    pub fn terminal_count(&self) -> usize {
        self.voltages.len()
    }

    /// Get thermal voltage kT/q.
    #[inline]
    pub fn vt(&self) -> f64 {
        const K_BOLTZMANN: f64 = 1.380649e-23;
        const Q_ELECTRON: f64 = 1.602176634e-19;
        K_BOLTZMANN * self.temperature / Q_ELECTRON
    }

    /// Set a parameter value.
    pub fn set_param(&mut self, index: usize, value: f64) {
        if index >= self.parameters.len() {
            self.parameters.resize(index + 1, 0.0);
        }
        self.parameters[index] = value;
    }

    /// Set a variable value.
    pub fn set_variable(&mut self, index: usize, value: f64) {
        if index >= self.variables.len() {
            self.variables.resize(index + 1, 0.0);
        }
        self.variables[index] = value;
    }
}
