//! Simulation State
//!
//! Manages simulation execution state and results.

use super::schematic::Point;
use crate::services::yield_manager::{YieldAnalysisProvenance, YieldResult};
use rspice_core::Value;
use std::collections::HashMap;

mod ac_bode;
mod analysis_result;
mod analysis_tag;
mod analysis_type;
mod cross_probe;
mod executed_deck;
mod result_digest;
mod run;
mod run_receipt;
mod saved_output;
mod specification_verdict;
mod state_impl;
mod state_model;
mod waveform;

pub const MAX_RUN_HISTORY: usize = 20;

pub use ac_bode::{ac_bode_summary_for_analysis, ac_bode_summary_for_selection};
pub use analysis_result::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisResultProvenance,
    AnalysisResultPvtPoint, AnalysisResultSourceDomain, ComplexResultValue, ConvergenceAttribution,
    DcOpResult, DigitalEventPointEvidence, DigitalEventTraceEvidence, MonteCarloVariableMetadata,
    NoiseContributorRow, NoiseSummary, OperatingPointAccuracyEvidence,
    OperatingPointAnnotationEvidence, OperatingPointDeviceDetailEvidence,
    OperatingPointHomotopyEvidence, OperatingPointInitialGuessEvidence,
    OperatingPointNodeInitializationEvidence, OperatingPointProcessEvidence,
    OperatingPointSaveDeviceEvidence, OperatingPointTemperatureEvidence, OperatingPointValue,
    PeriodicNoiseOutputQuantity, RealEventPointEvidence, RealEventTraceEvidence,
    ReliabilityCheckpointEvidence, ReliabilityDeviceEvidence, ReliabilityShiftEvidence,
    ReliabilityStressEvidence, SensitivityResultMode, SensitivityResultRow, SoaEvaluationEvidence,
    SoaParameterEvidence, SoaRuleVerdictEvidence, SoaViolationEvidence,
    SoaViolationSeverityEvidence, TransferFunctionAccuracyEvidence,
    TransferFunctionNormalizationEvidence, TransferFunctionQuantityEvidence,
    TransferFunctionScalarEvidence,
};
pub use analysis_tag::CanonicalAnalysisKind;
pub use analysis_type::AnalysisType;
pub use cross_probe::{CrossProbeIndex, CrossProbeMapping, OccurrenceProbeSpelling};
pub use executed_deck::{
    ExecutedDeck, ExecutedDeckArchive, ExecutedDeckPoint, sealed_model_sources,
};
pub use run::{
    ExecutionTarget, RunRetention, SimulationCampaignMembership, SimulationExecutionIdentity,
    SimulationRun, SimulationRunLifecycle,
};
pub use run_receipt::{
    HierarchyMapRow, PreparedModelSourceIdentity, PreparedRunReceipt, PreparedRunTaskReceipt,
    PreparedSourceCheckReceipt, PreparedSpecification, PreparedSpecificationPolicy,
    SimulationRunProvenance,
};
pub use saved_output::{SavedOutputMaterializationStatus, SavedOutputReceipt};
pub use specification_verdict::{SpecificationVerdict, SpecificationVerdictStatus};
pub use state_model::{SimulationRunIntent, SimulationState};
pub use waveform::{DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES, SharedWaveformValues, WaveformData};
