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
mod convergence_attribution;
mod cross_probe;
mod executed_deck;
mod family_measurements;
mod result_digest;
mod run;
mod run_receipt;
mod saved_output;
mod specification_verdict;
mod state_impl;
mod state_model;
mod waveform;

pub const MAX_RUN_HISTORY: usize = 20;

pub use ac_bode::{
    ac_bode_shape_for_analysis, ac_bode_shape_for_selection, ac_bode_summary_for_analysis,
    ac_bode_summary_for_selection,
};
pub use analysis_result::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisResultProvenance,
    AnalysisResultPvtPoint, AnalysisResultSourceDomain, ComplexResultValue, DcOpResult,
    DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
    DigitalEventTraceEvidence, FloquetOrbitKindEvidence, FloquetSpectrumCertificateEvidence,
    FloquetSpectrumEvidence, FloquetStabilityVerdictEvidence, MonteCarloVariableMetadata,
    NoiseContributorRow, NoiseSummary, OperatingPointAccuracyEvidence,
    OperatingPointAnnotationEvidence, OperatingPointDeviceDetailEvidence,
    OperatingPointHomotopyEvidence, OperatingPointInitialGuessEvidence,
    OperatingPointNodeInitializationEvidence, OperatingPointProcessEvidence,
    OperatingPointSaveDeviceEvidence, OperatingPointTemperatureEvidence, OperatingPointValue,
    PeriodicNoiseOutputQuantity, PoleZeroRootSetEvidence, PoleZeroSpectrumCertificate,
    PssFloquetMultiplierEvidence, PstbFloquetModeEvidence, PstbStabilityClassificationEvidence,
    RealEventPointEvidence, RealEventTraceEvidence, ReliabilityCheckpointEvidence,
    ReliabilityDeviceEvidence, ReliabilityShiftEvidence, ReliabilityStressEvidence,
    SensitivityResultMode, SensitivityResultRow, SoaEvaluationEvidence, SoaParameterEvidence,
    SoaRuleVerdictEvidence, SoaViolationEvidence, SoaViolationSeverityEvidence,
    TransferFunctionAccuracyEvidence, TransferFunctionNormalizationEvidence,
    TransferFunctionQuantityEvidence, TransferFunctionScalarEvidence,
};
pub use analysis_tag::{AnalysisAvailability, CanonicalAnalysisKind};
pub use analysis_type::AnalysisType;
pub use convergence_attribution::ConvergenceAttribution;
// Test-only alias: outside tests an attribution's vocabulary is only ever
// named through the attribution's own fields.
#[cfg(test)]
pub use convergence_attribution::ConvergenceFailureClass;
pub use cross_probe::{CrossProbeIndex, CrossProbeMapping, OccurrenceProbeSpelling};
pub use executed_deck::{
    ExecutedDeck, ExecutedDeckArchive, ExecutedDeckPoint, absent_deck_reason, sealed_model_sources,
};
pub use family_measurements::{
    FamilyMeasurementEvidence, FamilyMemberId, FamilyMemberMeasurements,
};
pub use run::{
    ExecutionTarget, RunRetention, SimulationCampaignMembership, SimulationExecutionIdentity,
    SimulationRun, SimulationRunLifecycle,
};
pub use run_receipt::{
    HierarchyMapRow, PreparedModelQualification, PreparedModelSourceIdentity, PreparedRunReceipt,
    PreparedRunTaskReceipt, PreparedSourceCheckReceipt, PreparedSpecification,
    PreparedSpecificationPolicy, SignOffStanding, SimulationRunProvenance,
};
pub use saved_output::{SavedOutputMaterializationStatus, SavedOutputReceipt};
pub use specification_verdict::{SpecificationVerdict, SpecificationVerdictStatus};
pub use state_impl::EvidenceDomain;
pub use state_model::{SimulationRunIntent, SimulationState};
pub use waveform::{DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES, SharedWaveformValues, WaveformData};
