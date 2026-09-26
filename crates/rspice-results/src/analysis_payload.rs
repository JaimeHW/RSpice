//! Exact retained analysis payloads, scalar queries, and evidence validation.

use crate::analysis_type::AnalysisType;
use crate::current_impulses::CurrentImpulseHistoryEvidence;
use crate::dc_mismatch::DcMismatchEvidence;
use crate::dc_sweep::DcSweepEvidence;
use crate::events::{DigitalBusEvidence, DigitalEventTraceEvidence, RealEventTraceEvidence};
use crate::fft::spectrum::{FftMetricsEvidence, FftSpectrumEvidence};
use crate::floquet::{
    FloquetOrbitKindEvidence, FloquetSpectrumEvidence, FloquetStabilityVerdictEvidence,
    PssFloquetMultiplierEvidence, PssFloquetRef, PstbFloquetModeEvidence, PstbRef,
    PstbStabilityClassificationEvidence,
};
use crate::operating_point::{
    OperatingPointAccuracyEvidence, OperatingPointAnnotationEvidence,
    OperatingPointDeviceDetailEvidence, OperatingPointHomotopyEvidence,
    OperatingPointInitialGuessEvidence, OperatingPointNodeInitializationEvidence,
    OperatingPointPreviousStateEvidence, OperatingPointProcessEvidence,
    OperatingPointSaveDeviceEvidence, OperatingPointTemperatureEvidence,
};
use crate::pole_zero::PoleZeroRootSetEvidence;
use crate::sensitivity::{SensitivityResultMode, SensitivityResultRow, SensitivityStudyEvidence};
use crate::simulation_values::ComplexResultValue;
use crate::soa_evidence::{
    SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence, SoaViolationEvidence,
    SoaViolationSeverityEvidence,
};
use crate::soa_source::SoaSourceHistory;
use crate::transfer_function::{
    TransferFunctionAccuracyEvidence, TransferFunctionNormalizationEvidence,
    TransferFunctionQuantityEvidence, TransferFunctionRef, TransferFunctionScalarEvidence,
};
use crate::validation::{require_non_empty, validate_complex_values};
use std::collections::BTreeMap;

mod native_scalar_units;
mod qpac;
mod qpnoise;
mod qpss;
mod qpxf;
pub use qpxf::QpxfDisplayTrace;

/// Retained scalar value, unit, and outcome for specification evaluation.
#[derive(Debug, Clone, PartialEq)]
pub struct ScalarEvidenceCandidate {
    pub unit: Option<rspice_core::analysis::MeasurementUnit>,
    pub value: Option<f64>,
    pub passed: bool,
}

impl ScalarEvidenceCandidate {
    pub fn value_in_unit(&self, requested: &str) -> Result<Option<f64>, String> {
        match (&self.unit, self.value) {
            (Some(unit), Some(value)) if !requested.trim().is_empty() => {
                unit.convert_value(value, requested).map(Some)
            }
            _ => Ok(self.value),
        }
    }
}

const fn default_op_run_point_count() -> u64 {
    1
}

pub fn native_scalar_name_matches(name: &str, canonical: &str, dotted_compatibility: &str) -> bool {
    name.eq_ignore_ascii_case(canonical) || name.eq_ignore_ascii_case(dotted_compatibility)
}

/// The five DC mismatch scalars a specification can bound, each under its
/// canonical name and the dotted compatibility spelling.
///
/// One list, read by the lookup and by the name census alike, so a
/// specification cannot resolve against a name the census does not offer.
fn dc_mismatch_scalar_evidence(
    evidence: &DcMismatchEvidence,
) -> [(&'static str, &'static str, f64); 5] {
    [
        (
            "dcmatch_nominal_value",
            "dcmatch.nominal_value",
            evidence.nominal_value,
        ),
        (
            "dcmatch_sigma_total",
            "dcmatch.sigma_total",
            evidence.sigma_total,
        ),
        (
            "dcmatch_sigma_mismatch",
            "dcmatch.sigma_mismatch",
            evidence.sigma_mismatch,
        ),
        (
            "dcmatch_sigma_process",
            "dcmatch.sigma_process",
            evidence.sigma_process,
        ),
        (
            "dcmatch_quoted_sigma",
            "dcmatch.quoted_sigma",
            evidence.quoted_sigma(),
        ),
    ]
}

/// The scalars a recorded FFT exposes, and only when the run asked for them.
const FFT_METRIC_NAMES: &[&str] = &[
    "fft_thd_db",
    "fft_sndr_db",
    "fft_enob_bits",
    "fft_snr_db",
    "fft_sfdr_db",
    "fft_fundamental_magnitude",
];

fn fft_metric(metrics: &FftMetricsEvidence, name: &str) -> Option<f64> {
    if native_scalar_name_matches(name, "fft_thd_db", "fft.thd_db") {
        Some(metrics.thd_db)
    } else if native_scalar_name_matches(name, "fft_sndr_db", "fft.sndr_db") {
        Some(metrics.sndr_db)
    } else if native_scalar_name_matches(name, "fft_enob_bits", "fft.enob_bits") {
        Some(metrics.enob_bits)
    } else if native_scalar_name_matches(name, "fft_snr_db", "fft.snr_db") {
        Some(metrics.snr_db)
    } else if native_scalar_name_matches(name, "fft_sfdr_db", "fft.sfdr_db") {
        Some(metrics.sfdr_db)
    } else if native_scalar_name_matches(
        name,
        "fft_fundamental_magnitude",
        "fft.fundamental_magnitude",
    ) {
        Some(metrics.fundamental_magnitude)
    } else {
        None
    }
}

/// Immutable, analysis-native result evidence that is neither waveform data
/// nor presentation state.
///
/// The payload is persisted, content-digested, and selected with its owning
/// analysis. Viewers must derive from this value instead of keeping
/// a second mutable copy of engineering data.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AnalysisResultPayload {
    /// Complete signed torus spectrum, including branch currents, producer
    /// identity, solver settings and convergence evidence. Display bins can be
    /// reconstructed exactly from the retained lattice.
    Qpac {
        response: std::sync::Arc<rspice_core::engine::QpacAnalysisResult>,
    },
    Qpnoise {
        response: std::sync::Arc<rspice_core::engine::QpnoiseAnalysisResult>,
    },
    Qpxf {
        response: std::sync::Arc<rspice_core::engine::QpxfAnalysisResult>,
    },
    Qpss {
        operating_point: std::sync::Arc<rspice_core::engine::QpssOperatingPoint>,
    },
    DcSweep {
        evidence: std::sync::Arc<DcSweepEvidence>,
    },
    OperatingPoint {
        temperature_mode: OperatingPointTemperatureEvidence,
        temperature_celsius: f64,
        initial_guess: OperatingPointInitialGuessEvidence,
        node_initialization: OperatingPointNodeInitializationEvidence,
        homotopy: OperatingPointHomotopyEvidence,
        annotation: OperatingPointAnnotationEvidence,
        device_detail: OperatingPointDeviceDetailEvidence,
        save_device_op: OperatingPointSaveDeviceEvidence,
        accuracy: OperatingPointAccuracyEvidence,
        selected_devices: Vec<String>,
        #[serde(default)]
        violation_devices: Vec<String>,
        #[serde(default)]
        violation_source_content_digest: Option<rspice_app_types::product::ContentDigest>,
        validated_startup_directives: u64,
        #[serde(default)]
        mna_node_names: Vec<String>,
        #[serde(default)]
        mna_branch_names: Vec<String>,
        #[serde(default)]
        mna_solution: Vec<f64>,
        /// Exact executable source plus voltage-corner mutation that produced
        /// this state. Absent only on legacy retained results, which are not
        /// eligible for Previous-converged startup.
        #[serde(default)]
        effective_source_content_digest: Option<rspice_app_types::product::ContentDigest>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        previous_state: Option<OperatingPointPreviousStateEvidence>,
        #[serde(default)]
        run_point_index: u64,
        #[serde(default = "default_op_run_point_count")]
        run_point_count: u64,
        #[serde(default)]
        run_point_process: OperatingPointProcessEvidence,
        #[serde(default)]
        run_point_supply_voltage: Option<f64>,
        #[serde(default)]
        run_point_nominal_supply_voltage: Option<f64>,
    },
    PoleZero {
        poles: Vec<ComplexResultValue>,
        zeros: Vec<ComplexResultValue>,
        #[serde(default)]
        pole_evidence: PoleZeroRootSetEvidence,
        #[serde(default)]
        zero_evidence: PoleZeroRootSetEvidence,
        /// Finite DC gain when defined. Missing legacy fields deserialize as
        /// unavailable; legacy numeric fields deserialize as `Some(value)`.
        #[serde(default)]
        gain: Option<f64>,
    },
    /// Durable periodic-steady-state Floquet evidence. Execution artifacts
    /// such as the monodromy matrix and shooting state are intentionally not
    /// part of the project result contract.
    PssFloquet {
        /// Absent only on an authenticated schema-v16 migration marker.
        period_s: Option<f64>,
        /// Absent only on an authenticated schema-v16 migration marker.
        fundamental_frequency_hz: Option<f64>,
        /// Absent only on an authenticated schema-v16 migration marker.
        iterations: Option<u64>,
        /// Absent only on an authenticated schema-v16 migration marker.
        residual_norm: Option<f64>,
        multipliers: Vec<PssFloquetMultiplierEvidence>,
        floquet_evidence: FloquetSpectrumEvidence,
        orbit_kind: FloquetOrbitKindEvidence,
        trivial_multiplier_index: Option<u64>,
        stability_verdict: FloquetStabilityVerdictEvidence,
    },
    /// Durable periodic-stability evidence. `modes` is the complete sorted
    /// spectrum; ordinary result waveforms remain presentation-only subsets.
    Pstb {
        /// Absent only on an authenticated schema-v16 migration marker.
        period_s: Option<f64>,
        /// Absent only on an authenticated schema-v16 migration marker.
        fundamental_frequency_hz: Option<f64>,
        /// Exact outer multiplier-magnitude boundary used by the producer.
        stability_threshold: Option<f64>,
        /// Canonical circuit identity whose eigenvector participation is stored.
        probe_instance: Option<String>,
        /// Whether root-of-unity classification was requested.
        detect_subharmonics: Option<bool>,
        modes: Vec<PstbFloquetModeEvidence>,
        floquet_evidence: FloquetSpectrumEvidence,
        orbit_kind: FloquetOrbitKindEvidence,
        trivial_multiplier_index: Option<u64>,
        stability_verdict: FloquetStabilityVerdictEvidence,
        stability_classification: PstbStabilityClassificationEvidence,
        /// None is a real current result when there is no applicable
        /// non-trivial mode; it is never represented by infinity.
        min_stability_margin_db: Option<f64>,
        /// Absent only on an authenticated schema-v16 migration marker.
        max_multiplier_magnitude: Option<f64>,
        /// Absent only on an authenticated schema-v16 migration marker.
        num_unstable: Option<u64>,
        subharmonics: Vec<u64>,
        /// Absent only on an authenticated schema-v16 migration marker.
        converged: Option<bool>,
        /// Absent only on an authenticated schema-v16 migration marker.
        iterations: Option<u64>,
    },
    Sensitivity {
        output: String,
        result_mode: SensitivityResultMode,
        rows: Vec<SensitivityResultRow>,
    },
    /// Linearized DC mismatch spread and its ranked contributors.
    DcMismatch {
        evidence: std::sync::Arc<DcMismatchEvidence>,
    },
    ScalarMeasurements {
        values: BTreeMap<String, f64>,
    },
    TransferFunction {
        input_source: String,
        output_expression: String,
        input_quantity: TransferFunctionQuantityEvidence,
        output_quantity: TransferFunctionQuantityEvidence,
        input_unit: String,
        output_unit: String,
        normalization: TransferFunctionNormalizationEvidence,
        accuracy: TransferFunctionAccuracyEvidence,
        gain: Option<TransferFunctionScalarEvidence>,
        input_resistance: Option<TransferFunctionScalarEvidence>,
        output_resistance: Option<TransferFunctionScalarEvidence>,
        /// Nominal source value used by relative normalization, otherwise absent.
        nominal_input: Option<f64>,
        /// Nominal output value used by relative normalization, otherwise absent.
        nominal_output: Option<f64>,
    },
    Soa {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source_history: Option<std::sync::Arc<SoaSourceHistory>>,
        evaluations: Vec<SoaEvaluationEvidence>,
        violations: Vec<SoaViolationEvidence>,
    },
    /// Committed XSPICE event histories from a transient run.
    ///
    /// Events are the sparse schedule the event solver accepted, not the
    /// analog timestep grid, so they are retained as their own evidence
    /// rather than resampled into waveforms.
    TransientEvents {
        /// Sparse current charge events and their independent coverage.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        current_impulses: Option<CurrentImpulseHistoryEvidence>,
        digital_traces: Vec<DigitalEventTraceEvidence>,
        real_traces: Vec<RealEventTraceEvidence>,
        /// Buses declared over `digital_traces`, in declaration order.
        ///
        /// Defaulted so a result retained before buses existed loads as one
        /// that declares none — which is what it means. An empty table is
        /// also the truth for every run whose model declared no vector port,
        /// so nothing is painted for it.
        #[serde(default)]
        digital_buses: Vec<DigitalBusEvidence>,
    },
    /// One `.FFT` spectrum the transient engine computed inside the solve that
    /// carried its card.
    ///
    /// The coefficients are the result's own complex waveform; what cannot be
    /// a waveform is here — which request produced it, what transform the
    /// engine performed, and the Xyce-compatible figures when the run asked
    /// for them.
    FftSpectrum {
        spectrum: FftSpectrumEvidence,
    },
    /// One `.SENS` study: the filter that chose the variables, the points the
    /// derivatives were taken at, and one column per variable per point.
    ///
    /// The `Sensitivity` variant above is frozen. It still decodes, validates
    /// and digests exactly as it always did, and nothing produces one any
    /// more: a result retained before the Studio ran the engine's own
    /// complete entries differentiated a different variable set, and is
    /// labelled as what it is rather than widened into a shape it never had.
    SensitivityStudy {
        evidence: std::sync::Arc<SensitivityStudyEvidence>,
    },
}

impl AnalysisResultPayload {
    /// Construct the only truthful periodic-stability payload that can be
    /// added while migrating a pre-v17 successful result. No numerical
    /// evidence or orbit policy is inferred from presentation waveforms.
    pub fn legacy_periodic_marker(analysis_type: AnalysisType) -> Option<Self> {
        match analysis_type {
            AnalysisType::Pss => Some(Self::PssFloquet {
                period_s: None,
                fundamental_frequency_hz: None,
                iterations: None,
                residual_norm: None,
                multipliers: Vec::new(),
                floquet_evidence: FloquetSpectrumEvidence::LegacyUnknown,
                orbit_kind: FloquetOrbitKindEvidence::LegacyUnknown,
                trivial_multiplier_index: None,
                stability_verdict: FloquetStabilityVerdictEvidence::Indeterminate,
            }),
            AnalysisType::Pstb => Some(Self::Pstb {
                period_s: None,
                fundamental_frequency_hz: None,
                stability_threshold: None,
                probe_instance: None,
                detect_subharmonics: None,
                modes: Vec::new(),
                floquet_evidence: FloquetSpectrumEvidence::LegacyUnknown,
                orbit_kind: FloquetOrbitKindEvidence::LegacyUnknown,
                trivial_multiplier_index: None,
                stability_verdict: FloquetStabilityVerdictEvidence::Indeterminate,
                stability_classification: PstbStabilityClassificationEvidence::Indeterminate,
                min_stability_margin_db: None,
                max_multiplier_magnitude: None,
                num_unstable: None,
                subharmonics: Vec::new(),
                converged: None,
                iterations: None,
            }),
            _ => None,
        }
    }

    pub fn scalar_evidence(&self, name: &str) -> Option<ScalarEvidenceCandidate> {
        let value = match self {
            Self::PssFloquet {
                period_s,
                fundamental_frequency_hz,
                multipliers,
                floquet_evidence,
                ..
            } => {
                if native_scalar_name_matches(name, "pss_period", "pss.period") {
                    *period_s
                } else if native_scalar_name_matches(
                    name,
                    "pss_fundamental_frequency",
                    "pss.fundamental_frequency",
                ) {
                    *fundamental_frequency_hz
                } else if native_scalar_name_matches(name, "pss_mode_count", "pss.mode_count")
                    && matches!(
                        floquet_evidence,
                        FloquetSpectrumEvidence::NoDynamicModes
                            | FloquetSpectrumEvidence::Qualified { .. }
                    )
                {
                    Some(multipliers.len() as f64)
                } else {
                    None
                }
            }
            Self::Pstb {
                period_s,
                fundamental_frequency_hz,
                modes,
                floquet_evidence,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                ..
            } => {
                if native_scalar_name_matches(name, "pstb_period", "pstb.period") {
                    *period_s
                } else if native_scalar_name_matches(
                    name,
                    "pstb_fundamental_frequency",
                    "pstb.fundamental_frequency",
                ) {
                    *fundamental_frequency_hz
                } else if native_scalar_name_matches(name, "pstb_mode_count", "pstb.mode_count")
                    && matches!(
                        floquet_evidence,
                        FloquetSpectrumEvidence::NoDynamicModes
                            | FloquetSpectrumEvidence::Qualified { .. }
                    )
                {
                    Some(modes.len() as f64)
                } else if native_scalar_name_matches(
                    name,
                    "pstb_unstable_mode_count",
                    "pstb.unstable_mode_count",
                ) {
                    num_unstable.map(|count| count as f64)
                } else if native_scalar_name_matches(
                    name,
                    "pstb_max_multiplier_magnitude",
                    "pstb.max_multiplier_magnitude",
                ) {
                    *max_multiplier_magnitude
                } else if native_scalar_name_matches(
                    name,
                    "pstb_min_stability_margin_db",
                    "pstb.min_stability_margin_db",
                ) {
                    *min_stability_margin_db
                } else {
                    None
                }
            }
            // Bounding a quoted sigma is why an engineer runs this analysis,
            // so all five sigmas answer a specification by name.
            Self::DcMismatch { evidence } => dc_mismatch_scalar_evidence(evidence)
                .into_iter()
                .find(|(native, dotted, _)| native_scalar_name_matches(name, native, dotted))
                .map(|(_, _, value)| value),
            Self::ScalarMeasurements { values } => values
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(name))
                .map(|(_, value)| *value),
            // Only under `FFTOUT=1`: the engine computes no figure otherwise,
            // and a zero here would read as a measured one.
            Self::FftSpectrum { spectrum } => spectrum
                .metrics
                .as_ref()
                .and_then(|metrics| fft_metric(metrics, name)),
            _ => None,
        }?;
        if !value.is_finite() {
            return None;
        }

        Some(ScalarEvidenceCandidate {
            unit: None,
            value: Some(value),
            passed: true,
        })
    }

    pub fn scalar_evidence_names(&self) -> Vec<String> {
        match self {
            Self::PssFloquet {
                period_s,
                fundamental_frequency_hz,
                floquet_evidence,
                ..
            } => {
                let mut names = Vec::with_capacity(3);
                if period_s.is_some() {
                    names.push("pss_period".to_owned());
                }
                if fundamental_frequency_hz.is_some() {
                    names.push("pss_fundamental_frequency".to_owned());
                }
                if matches!(
                    floquet_evidence,
                    FloquetSpectrumEvidence::NoDynamicModes
                        | FloquetSpectrumEvidence::Qualified { .. }
                ) {
                    names.push("pss_mode_count".to_owned());
                }
                names
            }
            Self::Pstb {
                period_s,
                fundamental_frequency_hz,
                floquet_evidence,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                ..
            } => {
                let mut names = Vec::with_capacity(6);
                if period_s.is_some() {
                    names.push("pstb_period".to_owned());
                }
                if fundamental_frequency_hz.is_some() {
                    names.push("pstb_fundamental_frequency".to_owned());
                }
                if matches!(
                    floquet_evidence,
                    FloquetSpectrumEvidence::NoDynamicModes
                        | FloquetSpectrumEvidence::Qualified { .. }
                ) {
                    names.push("pstb_mode_count".to_owned());
                }
                if num_unstable.is_some() {
                    names.push("pstb_unstable_mode_count".to_owned());
                }
                if max_multiplier_magnitude.is_some() {
                    names.push("pstb_max_multiplier_magnitude".to_owned());
                }
                if min_stability_margin_db.is_some() {
                    names.push("pstb_min_stability_margin_db".to_owned());
                }
                names
            }
            Self::DcMismatch { evidence } => dc_mismatch_scalar_evidence(evidence)
                .into_iter()
                .map(|(native, _, _)| native.to_owned())
                .collect(),
            Self::ScalarMeasurements { values } => values.keys().cloned().collect(),
            Self::FftSpectrum { spectrum } => {
                spectrum.metrics.as_ref().map_or_else(Vec::new, |_| {
                    FFT_METRIC_NAMES
                        .iter()
                        .map(|name| (*name).to_owned())
                        .collect()
                })
            }
            _ => Vec::new(),
        }
    }

    /// Validate exact retained evidence against the analysis that owns it.
    pub fn validate_for(&self, analysis_type: AnalysisType) -> Result<(), String> {
        match self {
            Self::Qpnoise { response } => {
                if analysis_type != AnalysisType::Qpnoise {
                    return Err("QPNOISE payload belongs to a different analysis type".into());
                }
                response
                    .validate_retained_payload_with_abort(
                        &rspice_core::ResourceLimits::default(),
                        &rspice_core::NoAbort,
                    )
                    .map_err(|e| e.to_string())?;
            }
            Self::Qpxf { response } => {
                if analysis_type != AnalysisType::Qpxf {
                    return Err("QPXF payload belongs to a different analysis type".into());
                }
                response
                    .validate_retained_payload_with_abort(
                        &rspice_core::ResourceLimits::default(),
                        &rspice_core::NoAbort,
                    )
                    .map_err(|e| e.to_string())?;
            }
            Self::Qpac { response } => {
                if analysis_type != AnalysisType::Qpac {
                    return Err("QPAC payload belongs to a different analysis type".into());
                }
                response
                    .validate_retained_payload_with_abort(
                        &rspice_core::ResourceLimits::default(),
                        &rspice_core::NoAbort,
                    )
                    .map_err(|e| e.to_string())?;
            }
            Self::Qpss { operating_point } => {
                if analysis_type != AnalysisType::Qpss {
                    return Err("QPSS payload belongs to a different analysis type".into());
                }
                operating_point
                    .validate_retained_payload_with_abort(
                        &rspice_core::ResourceLimits::default(),
                        &rspice_core::NoAbort,
                    )
                    .map_err(|error| error.to_string())?;
            }

            Self::DcSweep { evidence } => {
                if analysis_type != AnalysisType::DcSweep {
                    return Err("DC sweep evidence belongs to a different analysis type".to_owned());
                }
                evidence.validate()?;
            }
            Self::OperatingPoint {
                temperature_celsius,
                selected_devices,
                violation_devices,
                violation_source_content_digest,
                mna_node_names,
                mna_branch_names,
                mna_solution,
                effective_source_content_digest: _,
                initial_guess,
                previous_state,
                run_point_index,
                run_point_count,
                run_point_supply_voltage,
                run_point_nominal_supply_voltage,
                ..
            } => {
                if analysis_type != AnalysisType::DcOp {
                    return Err(format!(
                        "operating-point payload does not match analysis type {analysis_type:?}"
                    ));
                }
                if !temperature_celsius.is_finite() || *temperature_celsius <= -273.15 {
                    return Err("operating-point payload has an invalid temperature".to_owned());
                }
                if *initial_guess == OperatingPointInitialGuessEvidence::PreviousCompatible
                    && previous_state.is_none()
                {
                    return Err(
                        "compatible-circuit OP startup has no retained previous-state identity"
                            .to_owned(),
                    );
                }
                if previous_state.is_some()
                    && !matches!(
                        initial_guess,
                        OperatingPointInitialGuessEvidence::PreviousConverged
                            | OperatingPointInitialGuessEvidence::PreviousCompatible
                    )
                {
                    return Err("operating-point previous-state identity requires previous-solution startup".to_owned());
                }
                if selected_devices.iter().any(|name| {
                    name.is_empty() || name.trim() != name || name.chars().any(char::is_whitespace)
                }) || selected_devices.windows(2).any(|pair| pair[0] >= pair[1])
                {
                    return Err(
                        "operating-point selected devices are not canonical sorted identities"
                            .to_owned(),
                    );
                }
                if violation_devices.iter().any(|name| {
                    name.is_empty() || name.trim() != name || name.chars().any(char::is_whitespace)
                }) || violation_devices.windows(2).any(|pair| pair[0] >= pair[1])
                {
                    return Err(
                        "operating-point violation devices are not canonical sorted identities"
                            .to_owned(),
                    );
                }
                if violation_devices.is_empty() != violation_source_content_digest.is_none() {
                    return Err(
                        "operating-point SOA devices are missing their source identity".to_owned(),
                    );
                }
                let ordered_len = mna_node_names.len().saturating_add(mna_branch_names.len());
                if (!mna_solution.is_empty() && ordered_len != mna_solution.len())
                    || mna_solution.iter().any(|value| !value.is_finite())
                    || mna_node_names
                        .iter()
                        .chain(mna_branch_names)
                        .any(|name| name.is_empty() || name.trim() != name)
                {
                    return Err(
                        "operating-point retained MNA state is incomplete or invalid".to_owned(),
                    );
                }
                if *run_point_count == 0 || *run_point_index >= *run_point_count {
                    return Err("operating-point retained run-point position is invalid".to_owned());
                }
                match (run_point_supply_voltage, run_point_nominal_supply_voltage) {
                    (None, None) => {}
                    (Some(supply), Some(nominal))
                        if supply.is_finite()
                            && *supply > 0.0
                            && nominal.is_finite()
                            && *nominal > 0.0 => {}
                    _ => {
                        return Err(
                            "operating-point retained PVT supply evidence is invalid".to_owned()
                        );
                    }
                }
            }
            Self::PoleZero {
                poles,
                zeros,
                pole_evidence,
                zero_evidence,
                gain,
            } => {
                if analysis_type != AnalysisType::PoleZero {
                    return Err(format!(
                        "pole-zero payload does not match analysis type {analysis_type:?}"
                    ));
                }
                validate_complex_values(poles, "pole")?;
                validate_complex_values(zeros, "zero")?;
                if !pole_evidence.is_consistent_with_count(poles.len()) {
                    return Err(
                        "pole-zero pole evidence is inconsistent with retained roots".to_owned(),
                    );
                }
                if !zero_evidence.is_consistent_with_count(zeros.len()) {
                    return Err(
                        "pole-zero zero evidence is inconsistent with retained roots".to_owned(),
                    );
                }
                if gain.is_some_and(|gain| !gain.is_finite()) {
                    return Err("pole-zero gain is non-finite".to_owned());
                }
            }
            Self::PssFloquet {
                period_s,
                fundamental_frequency_hz,
                iterations,
                residual_norm,
                multipliers,
                floquet_evidence,
                orbit_kind,
                trivial_multiplier_index,
                stability_verdict,
            } => {
                if analysis_type != AnalysisType::Pss {
                    return Err(format!(
                        "PSS Floquet payload does not match analysis type {analysis_type:?}"
                    ));
                }
                PssFloquetRef {
                    period_s: *period_s,
                    fundamental_frequency_hz: *fundamental_frequency_hz,
                    iterations: *iterations,
                    residual_norm: *residual_norm,
                    multipliers,
                    evidence: floquet_evidence,
                    orbit_kind: *orbit_kind,
                    trivial_multiplier_index: *trivial_multiplier_index,
                    verdict: *stability_verdict,
                }
                .validate()?;
            }
            Self::Pstb {
                period_s,
                fundamental_frequency_hz,
                stability_threshold,
                probe_instance,
                detect_subharmonics,
                modes,
                floquet_evidence,
                orbit_kind,
                trivial_multiplier_index,
                stability_verdict,
                stability_classification,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                subharmonics,
                converged,
                iterations,
            } => {
                if analysis_type != AnalysisType::Pstb {
                    return Err(format!(
                        "PSTB payload does not match analysis type {analysis_type:?}"
                    ));
                }
                PstbRef {
                    period_s: *period_s,
                    fundamental_frequency_hz: *fundamental_frequency_hz,
                    stability_threshold: *stability_threshold,
                    probe_instance: probe_instance.as_deref(),
                    detect_subharmonics: *detect_subharmonics,
                    modes,
                    evidence: floquet_evidence,
                    orbit_kind: *orbit_kind,
                    trivial_multiplier_index: *trivial_multiplier_index,
                    verdict: *stability_verdict,
                    classification: *stability_classification,
                    min_stability_margin_db: *min_stability_margin_db,
                    max_multiplier_magnitude: *max_multiplier_magnitude,
                    num_unstable: *num_unstable,
                    subharmonics,
                    converged: *converged,
                    iterations: *iterations,
                }
                .validate()?;
            }
            Self::Sensitivity {
                output,
                result_mode,
                rows,
            } => {
                if analysis_type != AnalysisType::Sensitivity {
                    return Err(format!(
                        "sensitivity payload does not match analysis type {analysis_type:?}"
                    ));
                }
                require_non_empty(output, "sensitivity output")?;
                if let SensitivityResultMode::Ac { frequency_hz } = result_mode
                    && (!frequency_hz.is_finite() || *frequency_hz <= 0.0)
                {
                    return Err(
                        "sensitivity AC frequency must be finite and greater than zero".to_owned(),
                    );
                }
                let mut previous_name: Option<&str> = None;
                for row in rows {
                    require_non_empty(&row.parameter, "sensitivity parameter")?;
                    if previous_name.is_some_and(|previous| previous >= row.parameter.as_str()) {
                        return Err(
                            "sensitivity rows must have unique, strictly sorted parameter names"
                                .to_owned(),
                        );
                    }
                    previous_name = Some(&row.parameter);
                    if [row.raw, row.normalized].into_iter().any(|value| match value {
                        rspice_core::analysis::sensitivity::SensitivityValue::Available(value) => !value.is_finite(),
                        rspice_core::analysis::sensitivity::SensitivityValue::Unavailable { unavailable } => {
                            unavailable == rspice_core::analysis::sensitivity::SensitivityUnavailability::InvalidInput
                        }
                    }) {
                        return Err(format!(
                            "sensitivity parameter '{}' has an invalid value",
                            row.parameter
                        ));
                    }
                }
            }
            Self::SensitivityStudy { evidence } => {
                if analysis_type != AnalysisType::Sensitivity {
                    return Err(format!(
                        "sensitivity study payload does not match analysis type {analysis_type:?}"
                    ));
                }
                evidence.validate()?;
            }
            Self::DcMismatch { evidence } => {
                if analysis_type != AnalysisType::DcMismatch {
                    return Err(format!(
                        "DC mismatch payload does not match analysis type {analysis_type:?}"
                    ));
                }
                evidence.validate()?;
            }
            Self::ScalarMeasurements { values } => {
                if matches!(
                    analysis_type,
                    AnalysisType::PoleZero
                        | AnalysisType::Sensitivity
                        | AnalysisType::Tf
                        | AnalysisType::DcMismatch
                ) {
                    return Err(format!(
                        "scalar result payload does not match analysis type {analysis_type:?}"
                    ));
                }
                for (name, value) in values {
                    require_non_empty(name, "scalar result name")?;
                    if !value.is_finite() {
                        return Err(format!("scalar result '{name}' is non-finite"));
                    }
                }
            }
            Self::TransferFunction {
                input_source,
                output_expression,
                input_quantity,
                output_quantity,
                input_unit,
                output_unit,
                normalization,
                gain,
                input_resistance,
                output_resistance,
                nominal_input,
                nominal_output,
                ..
            } => {
                if analysis_type != AnalysisType::Tf {
                    return Err(format!(
                        "transfer-function payload does not match analysis type {analysis_type:?}"
                    ));
                }
                TransferFunctionRef {
                    input_source,
                    output_expression,
                    input_quantity,
                    output_quantity,
                    input_unit,
                    output_unit,
                    normalization,
                    gain,
                    input_resistance,
                    output_resistance,
                    nominal_input,
                    nominal_output,
                }
                .validate()?;
            }
            Self::Soa {
                source_history: _,
                evaluations,
                violations,
            } => {
                if analysis_type != AnalysisType::Soa {
                    return Err(format!(
                        "SOA payload does not match analysis type {analysis_type:?}"
                    ));
                }
                if evaluations.is_empty() {
                    return Err("SOA payload contains no evaluated-rule evidence".to_owned());
                }
                let derated_rules = evaluations
                    .iter()
                    .filter(|evaluation| {
                        evaluation.derating.is_some() || evaluation.envelope.is_some()
                    })
                    .map(|evaluation| (evaluation.device_id.as_str(), evaluation.parameter))
                    .collect::<std::collections::BTreeSet<_>>();
                let thresholds = evaluations
                    .iter()
                    .map(|evaluation| {
                        (
                            (evaluation.device_id.as_str(), evaluation.parameter),
                            evaluation.thresholds,
                        )
                    })
                    .collect::<std::collections::BTreeMap<_, _>>();
                let mut previous_evaluation: Option<&SoaEvaluationEvidence> = None;
                for evaluation in evaluations {
                    evaluation.thresholds.validate()?;
                    if let Some(duration) = evaluation.duration {
                        duration.validate()?;
                    }
                    require_non_empty(&evaluation.device_id, "SOA device identity")?;
                    require_non_empty(&evaluation.unit, "SOA rule unit")?;
                    require_non_empty(&evaluation.description, "SOA rule description")?;
                    for (label, value) in [
                        ("limit", evaluation.limit_value),
                        ("worst observed value", evaluation.worst_actual_value),
                        ("worst-point time", evaluation.worst_time_s),
                    ] {
                        if !value.is_finite() {
                            return Err(format!(
                                "SOA evaluation for '{}' has non-finite {label}",
                                evaluation.device_id
                            ));
                        }
                    }
                    if let Some(derating) = evaluation.derating {
                        derating.validate()?;
                        if evaluation.parameter != SoaParameterEvidence::PowerDissipation
                            || evaluation.unit != "W"
                        {
                            return Err("SOA temperature derating requires a power-dissipation rule in watts".into());
                        }
                    }
                    if let Some(envelope) = &evaluation.envelope {
                        envelope.curve.validate()?;
                        if !envelope.maximum_current_a.is_finite()
                            || envelope.maximum_current_a <= 0.0
                            || evaluation.unit != "A"
                            || crate::safety::SoaCurrentEnvelope::voltage_parameter(
                                evaluation.parameter.runtime_parameter(),
                            )
                            .is_none()
                            || evaluation.derating.is_some()
                        {
                            return Err("SOA current/voltage evidence requires a positive current cap and an Id/Ic/Ia rule in amperes".into());
                        }
                    }
                    if evaluation.limit_value < 0.0
                        || (evaluation.limit_value == 0.0
                            && !evaluation.parameter.is_directional()
                            && evaluation.derating.is_none()
                            && evaluation.envelope.is_none())
                    {
                        return Err(format!(
                            "SOA evaluation for '{}' has an invalid limit",
                            evaluation.device_id
                        ));
                    }
                    if evaluation.worst_actual_value < 0.0
                        || evaluation.worst_time_s < 0.0
                        || evaluation.sample_count == 0
                    {
                        return Err(format!(
                            "SOA evaluation for '{}' has invalid sampling evidence",
                            evaluation.device_id
                        ));
                    }
                    let expected_verdict = soa_rule_verdict(
                        evaluation.worst_actual_value,
                        evaluation.limit_value,
                        evaluation.thresholds,
                    );
                    if evaluation.duration.is_none() && evaluation.verdict != expected_verdict {
                        return Err(format!(
                            "SOA evaluation for '{}' has a verdict inconsistent with its worst value",
                            evaluation.device_id
                        ));
                    }
                    if previous_evaluation
                        .is_some_and(|previous| soa_evaluation_order(previous, evaluation).is_ge())
                    {
                        return Err(
                            "SOA evaluations must have unique canonical rule identities".to_owned()
                        );
                    }
                    previous_evaluation = Some(evaluation);
                }
                let duration_rules = evaluations
                    .iter()
                    .filter(|evaluation| evaluation.duration.is_some())
                    .map(|evaluation| (evaluation.device_id.as_str(), evaluation.parameter))
                    .collect::<std::collections::BTreeSet<_>>();
                let mut previous: Option<&SoaViolationEvidence> = None;
                for violation in violations {
                    require_non_empty(&violation.device_id, "SOA device identity")?;
                    for (label, value) in [
                        ("limit", violation.limit_value),
                        ("observed value", violation.actual_value),
                        ("time", violation.time_s),
                    ] {
                        if !value.is_finite() {
                            return Err(format!(
                                "SOA violation for '{}' has non-finite {label}",
                                violation.device_id
                            ));
                        }
                    }
                    if violation.time_s < 0.0 {
                        return Err(format!(
                            "SOA violation for '{}' has a negative time",
                            violation.device_id
                        ));
                    }
                    if violation.limit_value < 0.0
                        || (violation.limit_value == 0.0
                            && !violation.parameter.is_directional()
                            && !derated_rules
                                .contains(&(violation.device_id.as_str(), violation.parameter)))
                        || violation.actual_value < 0.0
                    {
                        return Err(format!(
                            "SOA violation for '{}' has invalid magnitude evidence",
                            violation.device_id
                        ));
                    }
                    let expected_severity = soa_violation_severity(
                        violation.actual_value,
                        violation.limit_value,
                        *thresholds
                            .get(&(violation.device_id.as_str(), violation.parameter))
                            .ok_or("SOA event has no matching evaluated rule")?,
                    )
                    .ok_or_else(|| {
                        format!(
                            "SOA event for '{}' does not meet the warning threshold",
                            violation.device_id
                        )
                    })?;
                    let duration_warning = duration_rules
                        .contains(&(violation.device_id.as_str(), violation.parameter))
                        && violation.severity == SoaViolationSeverityEvidence::Warning
                        && thresholds[&(violation.device_id.as_str(), violation.parameter)]
                            .warning_fraction
                            .is_some();
                    if violation.severity != expected_severity && !duration_warning {
                        return Err(format!(
                            "SOA event for '{}' has a severity inconsistent with its value",
                            violation.device_id
                        ));
                    }
                    if previous
                        .is_some_and(|previous| soa_violation_order(previous, violation).is_ge())
                    {
                        return Err(
                            "SOA violations must use unique canonical deterministic order"
                                .to_owned(),
                        );
                    }
                    previous = Some(violation);
                }
            }
            Self::TransientEvents {
                digital_traces,
                real_traces,
                digital_buses,
                current_impulses,
            } => {
                if analysis_type != AnalysisType::Transient {
                    return Err(format!(
                        "event payload does not match analysis type {analysis_type:?}"
                    ));
                }
                crate::events::validate_event_history(
                    digital_traces,
                    real_traces,
                    digital_buses,
                    current_impulses.as_ref(),
                )?;
            }
            Self::FftSpectrum { spectrum } => {
                // The Fourier family owns every retained coefficient
                // spectrum, which is why a recorded FFT joins it rather than
                // adding a second family that renders the same fact.
                if analysis_type != AnalysisType::Fourier {
                    return Err(format!(
                        "recorded FFT payload does not match analysis type {analysis_type:?}"
                    ));
                }
                spectrum.validate()?;
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn has_data(&self) -> bool {
        match self {
            Self::Qpac { .. } | Self::Qpxf { .. } | Self::Qpnoise { .. } | Self::Qpss { .. } | Self::DcSweep { .. } | Self::OperatingPoint { .. } | Self::PoleZero { .. } | Self::PssFloquet { .. } | Self::Pstb { .. } | Self::Sensitivity { .. }
            // A study is an answer even when its filter selected nothing the
            // engine could differentiate: the refusal is the run's, and what
            // it retained states the filter that produced it.
            | Self::SensitivityStudy { .. }
            // A spread is an answer even when no contributor cleared the
            // card's own threshold.
            | Self::DcMismatch { .. }
            // Always: an FFT payload states the transform the engine
            // performed, which is a fact even when the record ran short and
            // the spectrum has no coefficients.
            | Self::FftSpectrum { .. } => true,
            Self::ScalarMeasurements { values } => !values.is_empty(),
            Self::TransferFunction {
                gain,
                input_resistance,
                output_resistance,
                ..
            } => gain.is_some() || input_resistance.is_some() || output_resistance.is_some(),
            Self::Soa { evaluations, .. } => !evaluations.is_empty(),
            Self::TransientEvents {
                digital_traces,
                real_traces,
                current_impulses,
                ..
            } => {
                !digital_traces.is_empty() || !real_traces.is_empty() || current_impulses.is_some()
            }
        }
    }
}

fn soa_rule_verdict(
    actual: f64,
    limit: f64,
    thresholds: crate::safety::SoaThresholds,
) -> SoaRuleVerdictEvidence {
    match thresholds.verdict(actual, limit) {
        crate::safety::SoARuleVerdict::Pass => SoaRuleVerdictEvidence::Pass,
        crate::safety::SoARuleVerdict::Warning => SoaRuleVerdictEvidence::Warning,
        crate::safety::SoARuleVerdict::Violation => SoaRuleVerdictEvidence::Violation,
        crate::safety::SoARuleVerdict::Critical => SoaRuleVerdictEvidence::Critical,
    }
}

fn soa_violation_severity(
    actual: f64,
    limit: f64,
    thresholds: crate::safety::SoaThresholds,
) -> Option<SoaViolationSeverityEvidence> {
    match soa_rule_verdict(actual, limit, thresholds) {
        SoaRuleVerdictEvidence::Pass => None,
        SoaRuleVerdictEvidence::Warning => Some(SoaViolationSeverityEvidence::Warning),
        SoaRuleVerdictEvidence::Violation => Some(SoaViolationSeverityEvidence::Violation),
        SoaRuleVerdictEvidence::Critical => Some(SoaViolationSeverityEvidence::Critical),
    }
}

fn soa_evaluation_order(
    left: &SoaEvaluationEvidence,
    right: &SoaEvaluationEvidence,
) -> std::cmp::Ordering {
    left.device_id
        .cmp(&right.device_id)
        .then_with(|| left.parameter.cmp(&right.parameter))
}

fn soa_violation_order(
    left: &SoaViolationEvidence,
    right: &SoaViolationEvidence,
) -> std::cmp::Ordering {
    left.device_id
        .cmp(&right.device_id)
        .then_with(|| left.time_s.total_cmp(&right.time_s))
        .then_with(|| left.parameter.cmp(&right.parameter))
        .then_with(|| left.severity.cmp(&right.severity))
        .then_with(|| left.limit_value.total_cmp(&right.limit_value))
        .then_with(|| left.actual_value.total_cmp(&right.actual_value))
}
