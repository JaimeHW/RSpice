//! Numerical solvers for circuit simulation

pub mod arc_length;
pub mod convergence;
pub mod damping;
pub mod enhanced_newton;
pub mod klu;
mod newton;
mod sparse;
pub(crate) use sparse::klu_backend_enabled;

#[cfg(feature = "parallel")]
pub mod parallel;

pub use arc_length::{ArcLengthConfig, ArcLengthContinuation, ArcLengthResult, ArcLengthState};
pub use convergence::{
    ConvergenceController, ConvergenceMethod, ConvergenceResult, GminStepper, PseudoTransient,
    SourceStepper,
};
pub use damping::{DampingController, DampingStatistics, DampingStrategy};
pub use enhanced_newton::{EnhancedNewtonConfig, EnhancedNewtonSolver, EnhancedNewtonState};
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

    #[error("Stored pivot sequence is numerically inadequate for the new values")]
    PivotGrowth,
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
    /// Canonical branch names aligned with `branch_currents`
    pub branch_names: Vec<String>,
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
            branch_names: vec![String::new(); num_branches],
            time_points: Vec::new(),
            voltage_waveforms: Vec::new(),
        }
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
        crate::compat::ground::is_spice_ground_name(name)
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
