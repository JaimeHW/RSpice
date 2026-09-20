//! Authored independent-phase steady-state controls; defaults belong to the engine.
use crate::Value;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct QpssCard {
    pub frequencies: Vec<Value>,
    /// Empty uses the engine default; one count broadcasts to all tones.
    pub harmonics: Vec<usize>,
    /// One count broadcasts; otherwise one count per independent phase.
    pub oversample: Option<Vec<usize>>,
    /// Mutually exclusive with oversampling; odd and even grids are supported.
    pub points: Option<Vec<usize>>,
    pub max_mixing_order: Option<usize>,
    pub relative_tolerance: Option<Value>,
    pub current_absolute_tolerance: Option<Value>,
    pub voltage_absolute_tolerance: Option<Value>,
    pub max_iterations: Option<usize>,
    pub max_backtracks: Option<usize>,
    pub linear_solver: Option<String>,
    pub krylov_restart: Option<usize>,
    pub krylov_cycles: Option<usize>,
    pub linear_tolerance: Option<Value>,
    pub dc_initial_state: Option<bool>,
    /// Authored order of (source name, zero-based tone index) assignments.
    /// Several sources may drive one tone, or one source several tones.
    pub sources: Vec<(String, usize)>,
}
