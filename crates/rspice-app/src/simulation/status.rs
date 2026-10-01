//! Host execution-backend availability and shared simulation progress.

pub use rspice_simulation::progress::SimulationProgress;
pub use rspice_simulation_contract::progress::SimulationStatus;

/// Availability of the execution backend, independent of a run's lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) enum EngineAvailability {
    Ready,
    #[default]
    Starting,
    /// An intentional stop retired the worker; an explicit run can restart it.
    Restartable,
    #[cfg(any(target_arch = "wasm32", test))]
    Unavailable(String),
}

impl EngineAvailability {
    pub(crate) fn failure_reason(&self) -> Option<&str> {
        match self {
            #[cfg(any(target_arch = "wasm32", test))]
            Self::Unavailable(reason) => Some(reason),
            _ => None,
        }
    }
}
