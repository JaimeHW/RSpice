//! Simulation Engine - Wires netlist → circuit → solver → results
//!
//! This module provides the main simulation loop that connects all components.
//!
//! # Architecture
//!
//! The engine is organized into focused submodules:
//!
//! - [`builder`] - Circuit construction from netlist
//! - [`matrix`] - Matrix topology building  
//! - [`stamping`] - DC stamping functions
//! - [`convergence`] - Newton-Raphson and convergence helpers
//! - [`dc`] - DC operating point and sweep analysis
//! - [`transient`] - Time-domain transient simulation
//! - [`ac`] - Frequency-domain AC analysis
//! - [`advanced`] - Noise, Monte Carlo, pole-zero, sensitivity analysis

mod ac;
mod advanced;
mod builder;
mod convergence;
mod dc;
mod hb;
mod matrix;
mod pss;
mod stamping;
mod transient;

#[cfg(test)]
mod tests;

// Re-export CompressionConfig for public API
pub use crate::analysis::waveform::CompressionConfig;

use crate::Value;
use crate::analysis::waveform::TransientResultCompressed;
use crate::netlist::SourceSpec;
use thiserror::Error;

/// Simulation errors
#[derive(Debug, Error)]
pub enum SimulationError {
    #[error("Circuit error: {0}")]
    Circuit(String),

    #[error("Solver error: {0}")]
    Solver(#[from] crate::solver::SolverError),

    #[error("Netlist error: {0}")]
    Netlist(String),

    #[error("Convergence failed after {0} iterations")]
    ConvergenceFailed(usize),
}

/// Simulation configuration
#[derive(Debug, Clone)]
pub struct SimulationConfig {
    /// Convergence tolerance for Newton-Raphson
    pub tolerance: Value,
    /// Maximum Newton-Raphson iterations
    pub max_iterations: usize,
    /// Minimum timestep for transient analysis
    pub min_timestep: Value,
    /// Maximum timestep for transient analysis
    pub max_timestep: Value,
    /// Temperature in Kelvin
    pub temperature: Value,
    /// Integration method for transient analysis
    pub integration_method: crate::analysis::IntegrationMethod,
    /// Model evaluation bypass configuration for latent device optimization
    pub bypass_config: BypassConfig,
    /// Convergence configuration for DC operating point
    pub convergence_config: ConvergenceConfig,
}

/// Configuration for DC convergence algorithms
///
/// Controls which convergence aids are used when Newton-Raphson fails to converge
/// directly on difficult circuits. Methods are tried in order of increasing
/// computational cost.
#[derive(Debug, Clone)]
pub struct ConvergenceConfig {
    /// Enable GMIN stepping (small conductances to ground)
    pub gmin_stepping: bool,
    /// Enable source stepping (ramp sources from 0 to 100%)
    pub source_stepping: bool,
    /// Enable pseudo-transient continuation
    pub pseudo_transient: bool,
    /// Enable arc-length continuation for non-monotonic curves
    pub arc_length: bool,
    /// Damping strategy for Newton iterations
    pub damping_strategy: DampingStrategy,
    /// Initial GMIN value (typically 1e-12)
    pub gmin_initial: Value,
    /// Target GMIN value (typically 1e-15)
    pub gmin_target: Value,
    /// Verbose convergence logging
    pub verbose: bool,
}

/// Damping strategy for Newton-Raphson iterations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DampingStrategy {
    /// No damping (full Newton step)
    #[default]
    None,
    /// Backtracking line search (Armijo condition)
    LineSearch,
    /// Junction voltage limiting (SPICE-style)
    VoltageLimiting,
    /// Bank-Rose adaptive damping
    BankRose,
    /// Combined: voltage limiting + line search
    Combined,
}

impl Default for ConvergenceConfig {
    fn default() -> Self {
        Self {
            gmin_stepping: true,
            source_stepping: true,
            pseudo_transient: true,
            arc_length: false, // Only for difficult circuits
            damping_strategy: DampingStrategy::VoltageLimiting,
            gmin_initial: 1e-12,
            gmin_target: 1e-15,
            verbose: false,
        }
    }
}

impl ConvergenceConfig {
    /// Create a minimal config (direct Newton only)
    pub fn fast() -> Self {
        Self {
            gmin_stepping: false,
            source_stepping: false,
            pseudo_transient: false,
            arc_length: false,
            damping_strategy: DampingStrategy::None,
            ..Default::default()
        }
    }

    /// Create a robust config (all methods enabled)
    pub fn robust() -> Self {
        Self {
            gmin_stepping: true,
            source_stepping: true,
            pseudo_transient: true,
            arc_length: true,
            damping_strategy: DampingStrategy::Combined,
            verbose: false,
            ..Default::default()
        }
    }

    /// Enable verbose logging
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Set damping strategy
    pub fn with_damping(mut self, strategy: DampingStrategy) -> Self {
        self.damping_strategy = strategy;
        self
    }
}

/// Configuration for model evaluation bypass (latent device optimization)
#[derive(Debug, Clone)]
pub struct BypassConfig {
    /// Enable bypass optimization (default: false for stability)
    pub enabled: bool,
    /// Relative voltage tolerance for bypass detection
    pub reltol: Value,
    /// Absolute voltage tolerance for bypass detection  
    pub abstol: Value,
}

impl Default for BypassConfig {
    fn default() -> Self {
        Self {
            enabled: false, // Conservative default - must be explicitly enabled
            reltol: 1e-3,   // 0.1% relative change threshold
            abstol: 1e-6,   // 1µV absolute change threshold
        }
    }
}

impl BypassConfig {
    /// Create bypass config with optimization enabled
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }

    /// Create bypass config with custom tolerances
    pub fn with_tolerances(reltol: Value, abstol: Value) -> Self {
        Self {
            enabled: true,
            reltol,
            abstol,
        }
    }
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            tolerance: 1e-9,
            max_iterations: 50,
            min_timestep: 1e-15,
            max_timestep: 1e-3,
            temperature: 300.0, // Room temperature
            integration_method: crate::analysis::IntegrationMethod::TrapGear,
            bypass_config: BypassConfig::default(),
            convergence_config: ConvergenceConfig::default(),
        }
    }
}

/// Result of transient analysis - time-domain waveforms
#[derive(Debug, Clone)]
pub struct TransientResult {
    /// Time points
    pub time: Vec<Value>,
    /// Voltage waveforms: [node_index][time_index]
    pub voltages: Vec<Vec<Value>>,
    /// Number of nodes
    pub num_nodes: usize,
    /// Node names from the netlist (maps index to original name like "N001", "out", etc.)
    /// Index 0 corresponds to voltages[0], which is node 1 (not ground)
    pub node_names: Vec<String>,
}

impl TransientResult {
    /// Get voltage at a node at a specific time index
    pub fn voltage_at(&self, node: usize, time_index: usize) -> Value {
        if node == 0 || node > self.num_nodes {
            return 0.0; // Ground or invalid
        }
        self.voltages
            .get(node - 1)
            .and_then(|v| v.get(time_index))
            .copied()
            .unwrap_or(0.0)
    }

    /// Get the complete voltage waveform for a node
    pub fn voltage_waveform(&self, node: usize) -> &[Value] {
        if node == 0 || node > self.num_nodes {
            return &[];
        }
        self.voltages
            .get(node - 1)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get number of time points
    pub fn num_points(&self) -> usize {
        self.time.len()
    }
}

impl From<TransientResultCompressed> for TransientResult {
    fn from(compressed: TransientResultCompressed) -> Self {
        // Generate numeric fallback names - caller should override with actual names
        let node_names = (1..=compressed.num_nodes).map(|i| i.to_string()).collect();
        Self {
            time: compressed.time,
            voltages: compressed.voltages,
            num_nodes: compressed.num_nodes,
            node_names,
        }
    }
}

/// Main simulation engine
pub struct Engine {
    pub(crate) config: SimulationConfig,
}

impl Engine {
    /// Create a new engine with the given configuration
    pub fn new(config: SimulationConfig) -> Self {
        Self { config }
    }

    /// Get a reference to the simulation configuration
    pub fn config(&self) -> &SimulationConfig {
        &self.config
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(SimulationConfig::default())
    }
}

/// Extract DC value from a SourceSpec enum
pub(crate) fn extract_dc_value(spec: &SourceSpec) -> Value {
    match spec {
        SourceSpec::Dc(v) => *v,
        SourceSpec::DcAc { dc_value, .. } => *dc_value,
        SourceSpec::Pulse { v1, .. } => *v1, // Use initial value
        SourceSpec::Sin { offset, .. } => *offset, // Use DC offset
        SourceSpec::Pwl { points } => points.first().map(|(_, v)| *v).unwrap_or(0.0),
        SourceSpec::PwlFile { value_offset, .. } => *value_offset, // Use value offset as DC
        SourceSpec::Exp { v1, .. } => *v1,
        SourceSpec::Ac { .. } => 0.0, // AC sources have no DC component
    }
}

/// Extract AC value (magnitude, phase in radians) from a SourceSpec enum
/// Returns (magnitude, phase) tuple, defaulting to (0.0, 0.0) for non-AC sources
pub(crate) fn extract_ac_value(spec: &SourceSpec) -> (Value, Value) {
    match spec {
        SourceSpec::Ac { magnitude, phase } => (*magnitude, *phase),
        SourceSpec::DcAc {
            ac_magnitude,
            ac_phase,
            ..
        } => (*ac_magnitude, *ac_phase),
        // Sin sources have AC component at their frequency
        SourceSpec::Sin {
            amplitude, phase, ..
        } => (*amplitude, *phase),
        // Other sources don't have explicit AC component in HB sense
        _ => (0.0, 0.0),
    }
}
