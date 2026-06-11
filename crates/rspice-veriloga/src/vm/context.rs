use super::filters::{CrossDetector, DelayBuffer, SlewFilter, TransitionFilter};
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
    /// Solution values of branch-current unknowns (potential
    /// contributions), updated from the circuit solution each iteration
    pub branch_current_values: Vec<f64>,
    /// Terminal-pair branch current lookup table (flattened NxN matrix).
    terminal_pair_currents: Vec<f64>,
    /// Parameter values (indexed by parameter)
    pub parameters: Vec<f64>,
    /// Whether each parameter was explicitly set on the instance
    pub param_given: Vec<bool>,
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
    /// Per-slot flag marking state slots that have been written at least
    /// once (used by $limit to detect its first evaluation)
    pub state_initialized: Vec<bool>,
    /// Current timestep (delta t) for transient analysis
    pub timestep: f64,
    /// Lookup tables for $table_model interpolation
    pub lookup_tables: Vec<LookupTable>,
    /// Delay buffers for absdelay function
    /// Each buffer stores (time, value) pairs for interpolation
    pub delay_buffers: Vec<DelayBuffer>,
    /// Transition filters for `transition(...)` state
    pub transition_filters: Vec<TransitionFilter>,
    /// Slew filters for `slew(...)` state
    pub slew_filters: Vec<SlewFilter>,
    /// Cross detectors for `cross(...)` state
    pub cross_detectors: Vec<CrossDetector>,
    /// Current analysis type (0=dc, 1=ac, 2=tran, 3=noise)
    pub analysis_type: u8,
    /// Laplace state-space filters
    pub laplace_filters: Vec<StateSpaceFilter>,
    /// Instance multiplicity ($mfactor): the number of parallel copies
    /// this instance represents
    pub multiplicity: f64,
}

impl Default for VmContext {
    fn default() -> Self {
        Self {
            voltages: Vec::new(),
            internal_voltages: Vec::new(),
            currents: Vec::new(),
            branch_current_values: Vec::new(),
            terminal_pair_currents: Vec::new(),
            parameters: Vec::new(),
            param_given: Vec::new(),
            variables: Vec::new(),
            time: 0.0,
            temperature: 300.15, // 27C default
            state_values: Vec::new(),
            state_values_prev: Vec::new(),
            state_initialized: Vec::new(),
            timestep: 0.0,
            lookup_tables: Vec::new(),
            delay_buffers: Vec::new(),
            transition_filters: Vec::new(),
            slew_filters: Vec::new(),
            cross_detectors: Vec::new(),
            analysis_type: 0, // DC by default
            laplace_filters: Vec::new(),
            multiplicity: 1.0,
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
            branch_current_values: Vec::new(),
            terminal_pair_currents: vec![f64::NAN; num_terminals.saturating_mul(num_terminals)],
            parameters: Vec::new(),
            param_given: Vec::new(),
            variables: Vec::new(),
            time: 0.0,
            temperature: 300.15,
            state_values: Vec::new(),
            state_values_prev: Vec::new(),
            state_initialized: Vec::new(),
            timestep: 0.0,
            lookup_tables: Vec::new(),
            delay_buffers: Vec::new(),
            transition_filters: Vec::new(),
            slew_filters: Vec::new(),
            cross_detectors: Vec::new(),
            analysis_type: 0,
            laplace_filters: Vec::new(),
            multiplicity: 1.0,
        }
    }

    /// Create with internal nodes.
    pub fn with_internal_nodes(num_terminals: usize, num_internal: usize) -> Self {
        Self {
            voltages: vec![0.0; num_terminals],
            internal_voltages: vec![0.0; num_internal],
            currents: Vec::new(),
            branch_current_values: Vec::new(),
            terminal_pair_currents: vec![f64::NAN; num_terminals.saturating_mul(num_terminals)],
            parameters: Vec::new(),
            param_given: Vec::new(),
            variables: Vec::new(),
            time: 0.0,
            temperature: 300.15,
            state_values: Vec::new(),
            state_values_prev: Vec::new(),
            state_initialized: Vec::new(),
            timestep: 0.0,
            lookup_tables: Vec::new(),
            delay_buffers: Vec::new(),
            transition_filters: Vec::new(),
            slew_filters: Vec::new(),
            cross_detectors: Vec::new(),
            analysis_type: 0,
            laplace_filters: Vec::new(),
            multiplicity: 1.0,
        }
    }

    /// Create with state variables for transient analysis.
    pub fn with_states(num_terminals: usize, num_states: usize) -> Self {
        Self {
            voltages: vec![0.0; num_terminals],
            internal_voltages: Vec::new(),
            currents: Vec::new(),
            branch_current_values: Vec::new(),
            terminal_pair_currents: vec![f64::NAN; num_terminals.saturating_mul(num_terminals)],
            parameters: Vec::new(),
            param_given: Vec::new(),
            variables: Vec::new(),
            time: 0.0,
            temperature: 300.15,
            state_values: vec![0.0; num_states],
            state_values_prev: vec![0.0; num_states],
            state_initialized: vec![false; num_states],
            timestep: 0.0,
            lookup_tables: Vec::new(),
            delay_buffers: Vec::new(),
            transition_filters: Vec::new(),
            slew_filters: Vec::new(),
            cross_detectors: Vec::new(),
            analysis_type: 0,
            laplace_filters: Vec::new(),
            multiplicity: 1.0,
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
        self.state_initialized.resize(count, false);
    }

    /// Allocate delay buffers used by `absdelay(...)`.
    pub fn allocate_delay_buffers(&mut self, count: usize) {
        self.delay_buffers.resize_with(count, DelayBuffer::default);
    }

    /// Allocate transition filters used by `transition(...)`.
    pub fn allocate_transition_filters(&mut self, count: usize) {
        self.transition_filters
            .resize_with(count, TransitionFilter::default);
    }

    /// Allocate slew filters used by `slew(...)`.
    pub fn allocate_slew_filters(&mut self, count: usize) {
        self.slew_filters.resize_with(count, SlewFilter::default);
    }

    /// Allocate cross detectors used by `cross(...)`.
    pub fn allocate_cross_detectors(&mut self, count: usize) {
        self.cross_detectors
            .resize_with(count, CrossDetector::default);
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

    /// Get the voltage difference between two nodes in the unified node
    /// space (terminals first, then internal nodes; usize::MAX is the
    /// global reference node).
    #[inline]
    pub fn voltage(&self, pos: usize, neg: usize) -> f64 {
        self.node_potential(pos) - self.node_potential(neg)
    }

    /// Potential of a unified node index against the global reference.
    #[inline]
    pub fn node_potential(&self, node: usize) -> f64 {
        if node == usize::MAX {
            return 0.0;
        }
        let num_terminals = self.voltages.len();
        if node < num_terminals {
            self.voltages[node]
        } else {
            self.internal_voltages
                .get(node - num_terminals)
                .copied()
                .unwrap_or(0.0)
        }
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

    /// Mark a parameter as explicitly given by the instance.
    pub fn mark_param_given(&mut self, index: usize) {
        if index >= self.param_given.len() {
            self.param_given.resize(index + 1, false);
        }
        self.param_given[index] = true;
    }

    /// Whether a parameter was explicitly given by the instance.
    #[inline]
    pub fn is_param_given(&self, index: usize) -> bool {
        self.param_given.get(index).copied().unwrap_or(false)
    }

    /// Set a variable value.
    pub fn set_variable(&mut self, index: usize, value: f64) {
        if index >= self.variables.len() {
            self.variables.resize(index + 1, 0.0);
        }
        self.variables[index] = value;
    }
}
