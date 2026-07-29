//! Numerical solvers for circuit simulation

pub mod arc_length;
pub mod convergence;
pub mod damping;
mod newton;
pub(crate) use rspice_matrix::klu_backend_enabled;
pub use rspice_matrix::{
    ComplexMatrix, CscIndex, SolverError, SparseLuSolver, StaticMatrix, TripletMatrix, solve_sparse,
};

/// Compatibility path for the solver benchmark and downstream diagnostics.
pub mod klu {
    pub use rspice_matrix::KluSolver;
}

pub use arc_length::{ArcLengthConfig, ArcLengthContinuation, ArcLengthResult, ArcLengthState};
pub use damping::{DampingController, DampingStatistics, DampingStrategy};
pub use convergence::{PseudoTransient, SourceStepper};
pub use newton::*;

use crate::Value;
use std::collections::HashMap;

/// Simulation result containing node voltages and branch currents
#[derive(Debug, Clone)]
pub struct SimulationResult {
    /// Node voltages indexed by node ID (0 = ground = 0V)
    pub node_voltages: Vec<Value>,
    /// Node names indexed by node ID (index 0 = "0" for ground)
    /// This allows mapping `node_voltages[i]` to actual net names from netlist
    pub node_names: Vec<String>,
    /// Branch currents for voltage sources/inductors
    pub branch_currents: Vec<Value>,
    /// Canonical branch names aligned with `branch_currents`
    pub branch_names: Vec<String>,
    /// Named real-valued operating-point observables evaluated from the
    /// converged circuit state. Keys use their canonical SPICE probe form
    /// (for example `I(R1)`, `P(R1)`, or `N(V1_BRANCH)`).
    ///
    /// Unlike reconstructing a device quantity from the source netlist,
    /// these values preserve the actual per-point circuit configuration for
    /// parameter, temperature, and nested DC sweeps.
    pub dc_observables: Vec<(String, Value)>,
    /// Case-insensitive lookup index for observables populated by the engine.
    /// The ordered public vector remains the export view; this index prevents
    /// large .PRINT DC requests from degrading to an O(columns^2) scan.
    dc_observable_index: HashMap<String, Value>,
    /// Time points (for transient analysis)
    pub time_points: Vec<Value>,
    /// Voltage waveforms, indexed `[node_id][time_index]`.
    pub voltage_waveforms: Vec<Vec<Value>>,
}

impl SimulationResult {
    pub fn new(num_nodes: usize, num_branches: usize) -> Self {
        // Build default node names: "0" for ground, "1", "2", etc. for other nodes
        let node_names: Vec<String> = (0..=num_nodes).map(|i| i.to_string()).collect();
        Self {
            node_voltages: vec![0.0; num_nodes + 1], // +1 for ground
            node_names,
            branch_currents: vec![0.0; num_branches],
            branch_names: vec![String::new(); num_branches],
            dc_observables: Vec::new(),
            dc_observable_index: HashMap::new(),
            time_points: Vec::new(),
            voltage_waveforms: Vec::new(),
        }
    }

    /// Append an engine-owned DC observable and update its lookup index.
    pub(crate) fn push_dc_observable(&mut self, name: String, value: Value) {
        self.dc_observable_index
            .insert(name.to_ascii_lowercase(), value);
        self.dc_observables.push((name, value));
    }

    /// Look up a converged DC observable by its SPICE probe name.
    pub fn try_dc_observable_named(&self, name: &str) -> Option<Value> {
        if self.dc_observable_index.len() == self.dc_observables.len()
            && !self.dc_observable_index.is_empty()
        {
            return self
                .dc_observable_index
                .get(&name.to_ascii_lowercase())
                .copied();
        }
        self.dc_observables
            .iter()
            .find_map(|(candidate, value)| candidate.eq_ignore_ascii_case(name).then_some(*value))
    }

    /// Get voltage at a node.
    ///
    /// Panics when `node` is out of range. Use [`Self::try_voltage`] when
    /// probing dynamically-specified nodes.
    #[track_caller]
    pub fn voltage(&self, node: usize) -> Value {
        if node == 0 {
            return 0.0;
        }

        self.try_voltage(node).unwrap_or_else(|| {
            panic!(
                "node {} out of range for SimulationResult with {} nodes",
                node,
                self.node_voltages.len().saturating_sub(1)
            )
        })
    }

    /// Get voltage at a node, returning `None` for invalid node IDs.
    pub fn try_voltage(&self, node: usize) -> Option<Value> {
        if node == 0 {
            Some(0.0)
        } else {
            self.node_voltages.get(node).copied()
        }
    }

    /// Resolve a node name to a node index, treating common ground aliases as
    /// node 0.
    pub fn node_index_named(&self, name: &str) -> Option<usize> {
        if Self::is_ground_name(name) {
            return Some(0);
        }

        self.node_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(name))
    }

    /// Get voltage at a node by name, returning `None` when the name does not
    /// resolve to a known node.
    pub fn try_voltage_named(&self, name: &str) -> Option<Value> {
        let node = self.node_index_named(name)?;
        self.try_voltage(node)
    }

    /// Initialize waveform storage for transient analysis
    pub fn init_waveforms(&mut self, num_nodes: usize, estimated_points: usize) {
        self.voltage_waveforms = vec![Vec::with_capacity(estimated_points); num_nodes + 1];
        self.time_points = Vec::with_capacity(estimated_points);
    }

    /// Record a time point
    pub fn record_point(&mut self, time: Value, voltages: &[Value]) {
        self.time_points.push(time);
        for (i, &v) in voltages.iter().enumerate() {
            if i < self.voltage_waveforms.len() {
                self.voltage_waveforms[i].push(v);
            }
        }
    }

    /// Get branch current by index
    ///
    /// Branch currents are stored for voltage sources and inductors
    /// which require additional MNA variables.
    #[inline]
    pub fn branch_current(&self, branch_idx: usize) -> Option<Value> {
        self.branch_currents.get(branch_idx).copied()
    }

    /// Get branch current by canonical element name stored in the result.
    pub fn branch_current_named(&self, name: &str) -> Option<Value> {
        let branch_idx = self
            .branch_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(name))?;
        self.branch_current(branch_idx)
    }

    /// Get branch current by element name using a lookup map
    ///
    /// # Arguments
    /// * `name` - Element name (e.g., "V1", "L1", "VSRC")
    /// * `branch_map` - Map from element name to branch index
    ///
    /// # Returns
    /// Current value if found, None otherwise
    ///
    /// # Example
    /// ```ignore
    /// let current = result.branch_current_by_name("V1", &circuit.branch_names);
    /// ```
    pub fn branch_current_by_name(
        &self,
        name: &str,
        branch_map: &std::collections::HashMap<String, usize>,
    ) -> Option<Value> {
        self.branch_current_named(name).or_else(|| {
            branch_map
                .get(name)
                .and_then(|&idx| self.branch_currents.get(idx).copied())
        })
    }

    fn is_ground_name(name: &str) -> bool {
        crate::naming::is_spice_ground_name(name)
    }
}

/// Main simulator interface
pub struct Simulator {
    /// Convergence tolerance for Newton-Raphson
    pub tolerance: Value,
    /// Maximum Newton-Raphson iterations
    pub max_iterations: usize,
    /// Minimum timestep for transient analysis
    pub min_timestep: Value,
    /// Maximum timestep for transient analysis
    pub max_timestep: Value,
}

impl Default for Simulator {
    fn default() -> Self {
        crate::engine::SimulationConfig::default().into()
    }
}

impl Simulator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set convergence tolerance
    pub fn with_tolerance(mut self, tol: Value) -> Self {
        self.tolerance = tol;
        self
    }

    /// Set maximum iterations
    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }
}

impl From<&crate::engine::SimulationConfig> for Simulator {
    fn from(config: &crate::engine::SimulationConfig) -> Self {
        Self {
            tolerance: config.tolerance,
            max_iterations: config.max_iterations,
            min_timestep: config.min_timestep,
            max_timestep: config.max_timestep,
        }
    }
}

impl From<crate::engine::SimulationConfig> for Simulator {
    fn from(config: crate::engine::SimulationConfig) -> Self {
        Self::from(&config)
    }
}

#[cfg(test)]
mod tests {
    use super::SimulationResult;

    #[test]
    fn indexed_dc_observable_lookup_is_case_insensitive() {
        let mut result = SimulationResult::new(0, 0);
        result.push_dc_observable("I(R1)".to_string(), 2.5);

        assert_eq!(result.try_dc_observable_named("i(r1)"), Some(2.5));
    }

    #[test]
    fn direct_dc_observable_append_retains_linear_compatibility() {
        let mut result = SimulationResult::new(0, 0);
        result.dc_observables.push(("I(R2)".to_string(), -1.25));

        assert_eq!(result.try_dc_observable_named("i(r2)"), Some(-1.25));
    }
}
