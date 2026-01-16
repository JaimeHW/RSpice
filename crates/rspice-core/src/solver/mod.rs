//! Numerical solvers for circuit simulation

mod sparse;
mod newton;
pub mod convergence;

#[cfg(feature = "parallel")]
pub mod parallel;

pub use sparse::*;
pub use newton::*;
pub use convergence::{
    SourceStepper, GminStepper, PseudoTransient, 
    ConvergenceController, ConvergenceMethod, ConvergenceResult,
};

use thiserror::Error;
use crate::Value;

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
    /// Branch currents for voltage sources/inductors
    pub branch_currents: Vec<Value>,
    /// Time points (for transient analysis)
    pub time_points: Vec<Value>,
    /// Voltage waveforms: waveforms[node_id][time_index]
    pub voltage_waveforms: Vec<Vec<Value>>,
}

impl SimulationResult {
    pub fn new(num_nodes: usize, num_branches: usize) -> Self {
        Self {
            node_voltages: vec![0.0; num_nodes + 1], // +1 for ground
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
