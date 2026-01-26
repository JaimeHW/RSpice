//! Simulation Execution Module
//!
//! This module provides the bridge between the UI and the rspice-core simulation engine.
//! It handles async simulation execution, progress tracking, and result management.
//!
//! # Architecture
//!
//! - `runner` - SimulationRunner for async execution
//! - `status` - Status tracking and progress reporting
//! - `results` - Result containers and waveform conversion
//! - `config` - Analysis configuration builders
//! - `netlist_gen` - Schematic to SPICE netlist conversion
//! - `engine_bridge` - Connects UI to rspice-core Engine

pub mod config;
pub mod engine_bridge;
pub mod netlist_gen;
pub mod results;
pub mod runner;
pub mod status;

pub use config::{AnalysisConfig, AnalysisType};
pub use engine_bridge::EngineBridge;
pub use netlist_gen::NetlistGenerator;
pub use results::{SimulationResult, WaveformData};
pub use runner::SimulationRunner;
pub use status::{SimulationProgress, SimulationStatus};

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify all public types are accessible
        let _status = SimulationStatus::Idle;
        let _progress = SimulationProgress::default();
    }
}
