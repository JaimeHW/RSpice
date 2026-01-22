//! Numerical solvers for circuit simulation

pub mod convergence;
mod newton;
mod sparse;

#[cfg(feature = "parallel")]
pub mod parallel;

pub use convergence::{
    ConvergenceController, ConvergenceMethod, ConvergenceResult, GminStepper, PseudoTransient,
    SourceStepper,
};
pub use newton::*;
pub use sparse::*;

use crate::Value;
use thiserror::Error;

/// Solver errors
#[derive(Debug, Error)]
pub enum SolverError {
    #[error("Matrix is singular or near-singular")]
    SingularMatrix,

    #[error("Failed to converge after {0} iterations")]
    ConvergenceFailed(usize),

    #[error("Numerical overflow detected")]
    Overflow,

    #[error("Invalid circuit configuration: {0}")]
    InvalidCircuit(String),
}

/// Simulation result containing node voltages and branch currents
#[derive(Debug, Clone)]
pub struct SimulationResult {
    /// Node voltages indexed by node ID (0 = ground = 0V)
    pub node_voltages: Vec<Value>,
    /// Node names indexed by node ID (index 0 = "0" for ground)
    /// This allows mapping node_voltages[i] to actual net names from netlist
    pub node_names: Vec<String>,
    /// Branch currents for voltage sources/inductors
    pub branch_currents: Vec<Value>,
    /// Time points (for transient analysis)
    pub time_points: Vec<Value>,
    /// Voltage waveforms: waveforms[node_id][time_index]
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
            time_points: Vec::new(),
            voltage_waveforms: Vec::new(),
        }
    }

    /// Get voltage at a node
    pub fn voltage(&self, node: usize) -> Value {
        if node == 0 {
            0.0 // Ground
        } else {
            self.node_voltages.get(node).copied().unwrap_or(0.0)
        }
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
        branch_map
            .get(name)
            .and_then(|&idx| self.branch_currents.get(idx).copied())
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
        Self {
            tolerance: 1e-9,
            max_iterations: 50,
            min_timestep: 1e-15,
            max_timestep: 1e-3,
        }
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
