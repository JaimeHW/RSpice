//! What the engine is doing, as the engine reports it, and the words the
//! status bar states it in.

pub use rspice_simulation::status::{EngineAvailability, SimulationStatus};

mod engine_segment;

pub use engine_segment::{preflight_status, simulation_engine_status, simulation_progress_percent};
