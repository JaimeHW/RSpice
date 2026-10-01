//! Host execution-backend availability and shared simulation progress.

pub use crate::progress::SimulationProgress;
pub use rspice_simulation_contract::progress::SimulationStatus;

/// Availability of the execution backend, independent of a run's lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EngineAvailability {
    Ready,
    #[default]
    Starting,
    /// An intentional stop retired the worker; an explicit run can restart it.
    Restartable,
    Unavailable(String),
}

impl EngineAvailability {
    pub fn failure_reason(&self) -> Option<&str> {
        match self {
            Self::Unavailable(reason) => Some(reason),
            _ => None,
        }
    }
}

/// Browser backend qualification facts observed by the host presentation.
#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Copy)]
pub struct EngineJitObservation {
    pub available: bool,
    pub solver_result: Option<f64>,
}
