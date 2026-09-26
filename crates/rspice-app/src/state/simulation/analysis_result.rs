//! Analysis result data and validation.
//!
//! Combines retained results with application waveforms and presentation state.
//! Payloads, provenance, and their pure validation live in `rspice-results`.

mod soa_source;

use super::*;
use std::collections::{BTreeMap, HashSet};

mod native_scalar_units;
mod quasi_periodic_display;

pub use rspice_results::family_metadata::{
    AnalysisResultFamilyMetadata, MonteCarloVariableMetadata, PeriodicNoiseOutputQuantity,
};

const LIVE_TRANSIENT_PARTIAL_MESSAGE: &str =
    "Transient analysis is running; displayed samples are provisional";
const LIVE_MONTE_CARLO_PARTIAL_MESSAGE: &str =
    "Monte Carlo analysis is running; committed trials are checkpointed";

pub use rspice_results::provenance::{
    AnalysisResultProvenance, AnalysisResultPvtPoint, AnalysisResultSourceDomain,
};

pub use rspice_results::events::{
    DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
    DigitalEventTraceEvidence, RealEventPointEvidence, RealEventTraceEvidence,
};

pub use rspice_results::soa_evidence::{
    SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence, SoaViolationEvidence,
    SoaViolationSeverityEvidence,
};

use rspice_results::analysis_payload::{ScalarEvidenceCandidate, native_scalar_name_matches};

mod result_payload;
pub use result_payload::AnalysisResult;
