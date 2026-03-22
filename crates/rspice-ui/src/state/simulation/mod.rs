//! Simulation State
//!
//! Manages simulation execution state and results.

use super::dc_annotation::DcAnnotationState;
use super::schematic::Point;
use crate::services::safety::SoAViolation;
use crate::services::yield_manager::YieldResult;
use crate::simulation::optimizer::OptimizerState;
use crate::simulation::reliability_engine::ReliabilityResult;
use rspice_core::Value;
use std::collections::HashMap;

mod analysis_result;
mod analysis_type;
mod cross_probe;
mod run;
mod state_impl;
mod state_model;
mod waveform;

pub const MAX_RUN_HISTORY: usize = 20;

pub use analysis_result::{AnalysisResult, DcOpResult, OperatingPointValue};
pub use analysis_type::AnalysisType;
pub use cross_probe::CrossProbeMapping;
pub use run::SimulationRun;
pub use state_model::SimulationState;
pub use waveform::WaveformData;

#[cfg(test)]
mod tests;
