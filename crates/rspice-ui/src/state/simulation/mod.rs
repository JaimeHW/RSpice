//! Simulation State
//!
//! Manages simulation execution state and results.

use super::schematic::Point;
use crate::services::safety::SoAViolation;
use crate::services::yield_manager::YieldResult;
use crate::simulation::optimizer::OptimizerState;
use crate::simulation::reliability_engine::ReliabilityResult;
use rspice_core::Value;
use std::collections::HashMap;

mod ac_bode;
mod analysis_result;
mod analysis_type;
mod cross_probe;
mod run;
mod run_receipt;
mod state_impl;
mod state_model;
mod waveform;

pub const MAX_RUN_HISTORY: usize = 20;

pub use ac_bode::{
    AcBodeMetrics, AcBodeSummary, ac_bode_summary_for_run, ac_bode_summary_for_selection,
    ac_bode_summary_for_source_instance,
};
pub use analysis_result::{
    AnalysisResult, AnalysisResultProvenance, AnalysisResultSourceDomain, DcOpResult,
    NoiseContributorRow, NoiseSummary, OperatingPointValue,
};
pub use analysis_type::AnalysisType;
pub use cross_probe::CrossProbeMapping;
pub use run::SimulationRun;
pub use run_receipt::{
    PreparedRunReceipt, PreparedRunTaskReceipt, PreparedSourceCheckReceipt, SimulationRunProvenance,
};
pub use state_model::{SimulationRunIntent, SimulationState};
pub use waveform::{SharedWaveformValues, WaveformData};
