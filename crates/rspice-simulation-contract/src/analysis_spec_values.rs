//! Portable values embedded in retained analysis specifications.

use serde::{Deserialize, Serialize};

/// Numerical formulation used to solve a periodic steady-state request.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PssMethod {
    /// Time-domain shooting Newton solve.
    #[default]
    Shooting,
    /// Frequency-domain harmonic-balance solve.
    HarmonicBalance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationGoal {
    /// Minimize objective value.
    Minimize,
    /// Maximize objective value.
    Maximize,
    /// Reach target objective value.
    Target,
}

/// Optimization algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationAlgorithm {
    /// Gradient descent with line search.
    GradientDescent,
    /// Pattern search.
    PatternSearch,
    /// Simulated annealing.
    SimulatedAnnealing,
}

/// Optimization variable bounds and initial value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptimizationVariable {
    /// Parameter name from netlist `.param`.
    pub name: String,
    /// Lower bound.
    pub min: f64,
    /// Upper bound.
    pub max: f64,
    /// Initial value.
    pub initial: f64,
}

/// S-parameter port definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpPort {
    /// Positive node.
    pub node_pos: String,
    /// Negative/reference node.
    pub node_neg: String,
    /// Optional per-port reference impedance override.
    pub z0: Option<f64>,
}

/// Retained transfer-gain normalization policy.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TfNormalization {
    #[default]
    None,
    RelativeToNominal,
    PerSourceUnit,
}

/// Numerical policy for the DC operating point and zero-hertz solves.
///
/// The same tier the transfer-function form offers; see
/// [`crate::accuracy`] for what a tier name resolves to.
pub type TfAccuracy = crate::accuracy::AnalysisAccuracy;
