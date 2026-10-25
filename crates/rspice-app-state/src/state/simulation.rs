//! Simulation State
//!
//! Manages simulation execution state and results.

use super::schematic::Point;
use rspice_results::yield_analysis::{YieldAnalysisProvenance, YieldResult};
use std::collections::HashMap;

mod ac_bode;
mod analysis_result;
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::dc_mismatch::DcMismatchEvidence;
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::dc_mismatch::{DcMismatchContributorEvidence, DcMismatchScopeEvidence};
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::dc_sweep::DcSweepEvidence;
pub use rspice_results::dc_sweep::DcSweepFamily;
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::dc_sweep::{DcCurveSelection, DcSweepDirection, DcSweepQuantity};
mod cross_probe;
mod result_digest;
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::noise::{
    NoiseContributorRow, NoiseSummary, PeriodicNoiseConversionEvidence,
};
mod run;
#[cfg(test)]
mod run_history;
pub use rspice_results::result_import::{ResultImportFormat, ResultImportSource};
#[cfg(test)]
mod run_receipt;
mod state_impl;
mod state_model;
pub use rspice_results::yield_analysis::YieldEvidence;

pub const MAX_RUN_HISTORY: usize = 20;

#[cfg(any(test, feature = "test-support"))]
mod current_impulses;
#[cfg(any(test, feature = "test-support"))]
pub use current_impulses::current_impulse_history_fixture;

#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::sensitivity::{
    SensitivityBasisEvidence, SensitivityResultMode, SensitivityResultRow,
    SensitivityStudyEvidence, SensitivityStudyRow,
};
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::simulation_values::ComplexResultValue;

pub use rspice_results::floquet::{
    FloquetOrbitKindEvidence, FloquetStabilityVerdictEvidence, PstbStabilityClassificationEvidence,
};
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::floquet::{
    FloquetSpectrumCertificateEvidence, FloquetSpectrumEvidence, PssFloquetMultiplierEvidence,
    PstbFloquetModeEvidence,
};
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::pole_zero::{PoleZeroRootSetEvidence, PoleZeroSpectrumCertificate};

pub use ac_bode::{
    ac_bode_shape_for_analysis, ac_bode_shape_for_selection, ac_bode_summary_for_analysis,
    ac_bode_summary_for_selection,
};
pub use analysis_result::AnalysisResult;
pub use rspice_results::analysis_payload::AnalysisResultPayload;
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::analysis_tag::CanonicalAnalysisKind;
pub use rspice_results::analysis_type::AnalysisType;
pub use rspice_results::convergence_attribution::ConvergenceAttribution;
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::convergence_quality::TransientConvergenceEvidence;
pub use rspice_results::convergence_quality::{ConvergenceReport, PeriodicInitializationMethod};
pub use rspice_results::events::{
    DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
    DigitalEventTraceEvidence, RealEventPointEvidence, RealEventTraceEvidence,
};
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::family_metadata::PeriodicNoiseOutputQuantity;
pub use rspice_results::family_metadata::{
    AnalysisResultFamilyMetadata, MonteCarloVariableMetadata,
};
pub use rspice_results::operating_point::{
    DcOpResult, OperatingPointAnnotationEvidence, OperatingPointValue,
};
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::operating_point::{
    OperatingPointAccuracyEvidence, OperatingPointDeviceDetailEvidence,
    OperatingPointHomotopyEvidence, OperatingPointInitialGuessEvidence,
    OperatingPointNodeInitializationEvidence, OperatingPointPreviousStateEvidence,
    OperatingPointProcessEvidence, OperatingPointSaveDeviceEvidence,
    OperatingPointTemperatureEvidence,
};
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::provenance::AnalysisResultPvtPoint;
pub use rspice_results::provenance::{AnalysisResultProvenance, AnalysisResultSourceDomain};
pub use rspice_results::soa_evidence::{
    SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence,
};
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::soa_evidence::{SoaViolationEvidence, SoaViolationSeverityEvidence};
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::soa_source::{SoaSourceHistory, SoaSourceWaveform};
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::transfer_function::{
    TransferFunctionAccuracyEvidence, TransferFunctionNormalizationEvidence,
    TransferFunctionQuantityEvidence, TransferFunctionScalarEvidence,
};
// Test-only alias: outside tests an attribution's vocabulary is only ever
// named through the attribution's own fields.
pub use cross_probe::{CrossProbeIndex, CrossProbeMapping};
pub use rspice_results::executed_deck::{
    ExecutedDeck, ExecutedDeckArchive, ExecutedDeckPoint, absent_deck_reason,
};
pub use rspice_results::family_measurements::FamilyMemberId;
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::family_measurements::{
    FamilyMeasurementEvidence, FamilyMemberMeasurements,
};
pub use rspice_results::run::{
    EvidenceDomain, ExecutionTarget, RunRetention, SimulationCampaignMembership,
    SimulationExecutionIdentity, SimulationRunLifecycle,
};
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::run_receipt::{
    HierarchyMapRow, PreparedModelSourceIdentity, PreparedRunReceiptInput, PreparedRunTaskReceipt,
    PreparedSourceCheckReceipt,
};
pub use rspice_results::run_receipt::{PreparedRunReceipt, SimulationRunProvenance};
pub use rspice_results::saved_output::SavedOutputMaterializationStatus;
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::saved_output::SavedOutputReceipt;
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::saved_output::{
    SavedOutputAxis, SavedOutputBoundSource, SavedOutputDcMember, SavedOutputSourceBindings,
};
pub use rspice_results::specification::PreparedSpecification;
#[cfg(any(test, feature = "test-support"))]
pub use rspice_results::specification::PreparedSpecificationPolicy;
pub use rspice_results::specification_verdict::SpecificationVerdictStatus;
pub use rspice_results::waveform::SharedWaveformValues;
pub use rspice_vocabulary::hierarchy_path::OccurrenceProbeSpelling;
pub use run::SimulationRun;
pub type RunHistory = rspice_results::run_history::RunHistory<SimulationRun>;
pub use rspice_results::run_history::RunHistoryRevision;
pub use rspice_results::waveform::{DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES, RetainedTrace};
pub use rspice_simulation::execution::SimulationRunIntent;
pub use state_model::{
    RetainedSimulationState, SimulationExecutionState, SimulationSourceState, SimulationState,
    SimulationViewState,
};

/// Why a campaign member never became a run: what the run controller reports
/// when it files the member's refusal with the diagnostic store.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CampaignMemberRefusal {
    /// Its frozen snapshot could not be authorized for dispatch.
    Authorization,
    /// It could not be started: its place in the campaign could not be
    /// recorded, or the run it was authorized for could not be opened.
    Start,
}

pub use rspice_results::monte_carlo_checkpoint::{
    MonteCarloCheckpointEvidence, MonteCarloCheckpointLibrary,
};
