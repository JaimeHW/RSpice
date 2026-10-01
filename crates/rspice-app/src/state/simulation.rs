//! Simulation State
//!
//! Manages simulation execution state and results.

use super::schematic::Point;
use rspice_results::yield_analysis::{YieldAnalysisProvenance, YieldResult};
use std::collections::HashMap;

mod ac_bode;
mod analysis_result;
pub use rspice_results::dc_mismatch::DcMismatchEvidence;
#[cfg(test)]
pub use rspice_results::dc_mismatch::{DcMismatchContributorEvidence, DcMismatchScopeEvidence};
#[cfg(test)]
pub use rspice_results::dc_sweep::DcSweepEvidence;
pub use rspice_results::dc_sweep::{DcCurveSelection, DcSweepFamily};
#[cfg(test)]
pub use rspice_results::dc_sweep::{DcSweepDirection, DcSweepQuantity};
mod cross_probe;
mod result_digest;
#[cfg(test)]
pub use rspice_results::noise::{NoiseContributorRow, PeriodicNoiseConversionEvidence};
pub use rspice_results::noise::{NoiseFigureEvidence, NoiseSummary};
mod run;
#[cfg(test)]
mod run_history;
pub use rspice_results::result_import::{ResultImportFormat, ResultImportSource};
#[cfg(test)]
mod run_receipt;
mod state_impl;
mod state_model;
mod waveform;
pub use rspice_results::yield_analysis::YieldEvidence;

pub const MAX_RUN_HISTORY: usize = 20;

#[cfg(test)]
mod current_impulses;
#[cfg(test)]
pub(crate) use current_impulses::current_impulse_history_fixture;
pub use rspice_results::current_impulses::CurrentImpulseHistoryEvidence;

pub use rspice_results::sensitivity::{
    SensitivityBasisEvidence, SensitivityResultMode, SensitivityResultRow,
    SensitivityStudyEvidence, SensitivityStudyRow,
};
pub use rspice_results::simulation_values::ComplexResultValue;

pub use rspice_results::fft::spectrum::{FftSpectrumEvidence, FftSpectrumStatusEvidence};
// Test-only, like the attribution vocabulary in `state.rs`: outside tests the
// compatibility mode a spectrum was computed under is only ever read through
// the evidence's own field, never named as a type.
#[cfg(test)]
pub use rspice_results::fft::spectrum::FftSpectrumModeEvidence;

pub use rspice_results::floquet::{
    FloquetOrbitKindEvidence, FloquetSpectrumEvidence, FloquetStabilityVerdictEvidence,
    PstbStabilityClassificationEvidence,
};
#[cfg(test)]
pub use rspice_results::floquet::{
    FloquetSpectrumCertificateEvidence, PssFloquetMultiplierEvidence, PstbFloquetModeEvidence,
};
pub use rspice_results::pole_zero::PoleZeroRootSetEvidence;
#[cfg(test)]
pub use rspice_results::pole_zero::PoleZeroSpectrumCertificate;

pub use ac_bode::{
    ac_bode_shape_for_analysis, ac_bode_shape_for_selection, ac_bode_summary_for_analysis,
    ac_bode_summary_for_selection,
};
pub use analysis_result::AnalysisResult;
pub use rspice_results::analysis_payload::AnalysisResultPayload;
#[cfg(test)]
pub use rspice_results::analysis_tag::CanonicalAnalysisKind;
pub use rspice_results::analysis_type::AnalysisType;
pub use rspice_results::convergence_attribution::ConvergenceAttribution;
#[cfg(test)]
pub use rspice_results::convergence_quality::TransientConvergenceEvidence;
pub use rspice_results::convergence_quality::{ConvergenceReport, PeriodicInitializationMethod};
pub use rspice_results::events::{
    DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
    DigitalEventTraceEvidence, RealEventPointEvidence, RealEventTraceEvidence,
};
#[cfg(test)]
pub use rspice_results::family_metadata::PeriodicNoiseOutputQuantity;
pub use rspice_results::family_metadata::{
    AnalysisResultFamilyMetadata, MonteCarloVariableMetadata,
};
pub use rspice_results::operating_point::{
    DcOpResult, OperatingPointAnnotationEvidence, OperatingPointDeviceDetailEvidence,
    OperatingPointInitialGuessEvidence, OperatingPointValue,
};
#[cfg(test)]
pub use rspice_results::operating_point::{
    OperatingPointAccuracyEvidence, OperatingPointHomotopyEvidence,
    OperatingPointNodeInitializationEvidence, OperatingPointPreviousStateEvidence,
    OperatingPointProcessEvidence, OperatingPointSaveDeviceEvidence,
    OperatingPointTemperatureEvidence,
};
#[cfg(test)]
pub use rspice_results::provenance::AnalysisResultPvtPoint;
pub use rspice_results::provenance::{AnalysisResultProvenance, AnalysisResultSourceDomain};
pub use rspice_results::soa_evidence::{
    SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence,
};
#[cfg(test)]
pub use rspice_results::soa_evidence::{SoaViolationEvidence, SoaViolationSeverityEvidence};
#[cfg(test)]
pub use rspice_results::soa_source::{SoaSourceHistory, SoaSourceWaveform};
pub use rspice_results::transfer_function::{
    TransferFunctionAccuracyEvidence, TransferFunctionNormalizationEvidence,
    TransferFunctionQuantityEvidence, TransferFunctionScalarEvidence,
};
// Test-only alias: outside tests an attribution's vocabulary is only ever
// named through the attribution's own fields.
pub use cross_probe::{CrossProbeIndex, CrossProbeMapping};
pub use rspice_app_types::hierarchy_path::OccurrenceProbeSpelling;
pub use rspice_results::executed_deck::{
    ExecutedDeck, ExecutedDeckArchive, ExecutedDeckPoint, absent_deck_reason,
};
pub use rspice_results::family_measurements::FamilyMemberId;
#[cfg(test)]
pub use rspice_results::family_measurements::{
    FamilyMeasurementEvidence, FamilyMemberMeasurements,
};
pub use rspice_results::run::{
    EvidenceDomain, ExecutionTarget, RunRetention, SimulationCampaignMembership,
    SimulationExecutionIdentity, SimulationRunLifecycle,
};
#[cfg(test)]
pub use rspice_results::run_receipt::{
    HierarchyMapRow, PreparedModelSourceIdentity, PreparedRunReceiptInput, PreparedRunTaskReceipt,
    PreparedSourceCheckReceipt,
};
pub use rspice_results::run_receipt::{
    PreparedRunReceipt, SignOffStanding, SimulationRunProvenance,
};
#[cfg(test)]
pub use rspice_results::saved_output::{
    SavedOutputAxis, SavedOutputBoundSource, SavedOutputDcMember, SavedOutputSourceBindings,
};
pub use rspice_results::saved_output::{SavedOutputMaterializationStatus, SavedOutputReceipt};
pub use rspice_results::specification::PreparedSpecification;
#[cfg(test)]
pub use rspice_results::specification::PreparedSpecificationPolicy;
pub use rspice_results::specification_verdict::SpecificationVerdictStatus;
pub use rspice_results::waveform::SharedWaveformValues;
pub use run::SimulationRun;
pub type RunHistory = rspice_results::run_history::RunHistory<SimulationRun>;
pub(crate) use rspice_results::run_history::RunHistoryRevision;
pub use rspice_simulation::execution::SimulationRunIntent;
pub use state_model::SimulationState;
pub use waveform::{DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES, WaveformData};

pub use rspice_results::monte_carlo::MonteCarloMeanInterval;
#[cfg(test)]
pub use rspice_results::monte_carlo::{MonteCarloMeanConfidence, MonteCarloMeanMethod};
pub use rspice_results::monte_carlo_checkpoint::{
    MonteCarloCheckpointEvidence, MonteCarloCheckpointLibrary,
};
