//! Simulation State
//!
//! Manages simulation execution state and results.

use super::schematic::Point;
use rspice_core::Value;
use rspice_results::yield_analysis::{YieldAnalysisProvenance, YieldResult};
use std::collections::HashMap;

mod ac_bode;
mod analysis_result;
mod dc_mismatch;
pub use dc_mismatch::{DcMismatchContributorEvidence, DcMismatchEvidence, DcMismatchScopeEvidence};
pub(crate) use rspice_results::dc_sweep::DcTraceView;
pub use rspice_results::dc_sweep::{
    DcCurveSelection, DcSweepDirection, DcSweepEvidence, DcSweepFamily, DcSweepQuantity,
};
mod cross_probe;
mod executed_deck;
mod result_digest;
pub use rspice_results::noise::{
    NoiseContributorRow, NoiseFigureEvidence, NoiseSummary, PeriodicNoiseConversionEvidence,
};
mod run;
mod run_history;
pub use rspice_results::result_import::{ResultImportFormat, ResultImportSource};
mod run_receipt;
mod saved_output;
mod specification_verdict;
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
    FloquetOrbitKindEvidence, FloquetSpectrumCertificateEvidence, FloquetSpectrumEvidence,
    FloquetStabilityVerdictEvidence, PssFloquetMultiplierEvidence, PstbFloquetModeEvidence,
    PstbStabilityClassificationEvidence,
};
pub use rspice_results::pole_zero::{PoleZeroRootSetEvidence, PoleZeroSpectrumCertificate};

pub use ac_bode::{
    ac_bode_shape_for_analysis, ac_bode_shape_for_selection, ac_bode_summary_for_analysis,
    ac_bode_summary_for_selection,
};
pub use analysis_result::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultProvenance, AnalysisResultPvtPoint,
    AnalysisResultSourceDomain, DigitalBusEvidence, DigitalBusSourceEvidence,
    DigitalEventPointEvidence, DigitalEventTraceEvidence, MonteCarloVariableMetadata,
    PeriodicNoiseOutputQuantity, RealEventPointEvidence, RealEventTraceEvidence,
    SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence, SoaViolationEvidence,
    SoaViolationSeverityEvidence,
};
pub use rspice_results::analysis_payload::AnalysisResultPayload;
pub use rspice_results::analysis_type::AnalysisType;
pub use rspice_results::convergence_attribution::ConvergenceAttribution;
pub use rspice_results::convergence_quality::{
    ConvergenceReport, PeriodicConvergenceEvidence, PeriodicInitializationMethod,
    TransientConvergenceEvidence,
};
pub use rspice_results::operating_point::{
    DcOpResult, OperatingPointAccuracyEvidence, OperatingPointAnnotationEvidence,
    OperatingPointDeviceDetailEvidence, OperatingPointHomotopyEvidence,
    OperatingPointInitialGuessEvidence, OperatingPointNodeInitializationEvidence,
    OperatingPointPreviousStateEvidence, OperatingPointProcessEvidence,
    OperatingPointSaveDeviceEvidence, OperatingPointTemperatureEvidence, OperatingPointValue,
};
pub use rspice_results::soa_source::{SoaSourceHistory, SoaSourceWaveform};
pub use rspice_results::transfer_function::{
    TransferFunctionAccuracyEvidence, TransferFunctionNormalizationEvidence,
    TransferFunctionQuantityEvidence, TransferFunctionScalarEvidence,
};
pub use rspice_simulation_contract::analysis_tag::CanonicalAnalysisKind;
// Test-only alias: outside tests an attribution's vocabulary is only ever
// named through the attribution's own fields.
pub use cross_probe::{CrossProbeIndex, CrossProbeMapping, OccurrenceProbeSpelling};
pub use executed_deck::{
    ExecutedDeck, ExecutedDeckArchive, ExecutedDeckPoint, absent_deck_reason, sealed_model_sources,
};
#[cfg(test)]
pub use rspice_results::convergence_attribution::ConvergenceFailureClass;
pub use rspice_results::family_measurements::{
    FamilyMeasurementEvidence, FamilyMemberId, FamilyMemberMeasurements,
};
pub use run::{
    ExecutionTarget, RunRetention, SimulationCampaignMembership, SimulationExecutionIdentity,
    SimulationRun, SimulationRunLifecycle,
};
pub use run_history::RunHistory;
pub(crate) use run_history::RunHistoryRevision;
pub use run_receipt::{
    HierarchyMapRow, PreparedModelQualification, PreparedModelSourceIdentity, PreparedRunReceipt,
    PreparedRunTaskReceipt, PreparedSourceCheckReceipt, PreparedSpecification,
    PreparedSpecificationPolicy, SignOffStanding, SimulationRunProvenance,
};
pub use saved_output::{
    SavedOutputAxis, SavedOutputBoundSource, SavedOutputDcMember, SavedOutputMaterializationStatus,
    SavedOutputReceipt, SavedOutputSourceBindings,
};
pub use specification_verdict::{SpecificationVerdict, SpecificationVerdictStatus};
pub use state_impl::EvidenceDomain;
pub use state_model::{SimulationRunIntent, SimulationState};
pub use waveform::{DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES, SharedWaveformValues, WaveformData};

mod monte_carlo_checkpoint;
mod monte_carlo_confidence;
pub use monte_carlo_checkpoint::{MonteCarloCheckpointEvidence, MonteCarloCheckpointLibrary};
pub use monte_carlo_confidence::{
    MonteCarloMeanConfidence, MonteCarloMeanInterval, MonteCarloMeanMethod,
};
