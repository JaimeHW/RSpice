use super::*;
use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use std::collections::{BTreeMap, HashSet};

/// Durable source domain for a prepared analysis identity.
///
/// Simulation-plan IDs are owned by the project's stable plan/tombstones.
/// Manual-deck IDs are deterministic projections of an imported source deck
/// and intentionally have no plan object. `LegacyUnclassified` is reserved
/// for truthful migration of result schemas that persisted an ID but not its
/// domain; current execution must never create it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisResultSourceDomain {
    SimulationPlan,
    ManualDeck,
    #[default]
    LegacyUnclassified,
}

/// Immutable identity of the prepared analysis task that produced a result.
///
/// A result created by the current execution pipeline always carries this
/// record. `AnalysisResult::provenance == None` is reserved exclusively for
/// result history migrated from project formats that predate prepared-task
/// identities; callers must never infer an identity from analysis kind or
/// display order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisResultProvenance {
    source_domain: AnalysisResultSourceDomain,
    source_instance_id: AnalysisInstanceId,
    source_revision: ObjectRevision,
    prepared_snapshot_digest: ContentDigest,
    dependency_ids: Vec<AnalysisInstanceId>,
}

impl AnalysisResultProvenance {
    /// Build a complete, internally consistent prepared-task provenance
    /// record. Dependency order is retained exactly as it appeared in the
    /// frozen prepared snapshot.
    pub fn new(
        source_instance_id: AnalysisInstanceId,
        source_revision: ObjectRevision,
        prepared_snapshot_digest: ContentDigest,
        dependency_ids: Vec<AnalysisInstanceId>,
    ) -> Result<Self, String> {
        Self::new_with_source_domain(
            AnalysisResultSourceDomain::SimulationPlan,
            source_instance_id,
            source_revision,
            prepared_snapshot_digest,
            dependency_ids,
        )
    }

    pub fn new_with_source_domain(
        source_domain: AnalysisResultSourceDomain,
        source_instance_id: AnalysisInstanceId,
        source_revision: ObjectRevision,
        prepared_snapshot_digest: ContentDigest,
        dependency_ids: Vec<AnalysisInstanceId>,
    ) -> Result<Self, String> {
        let mut unique_dependencies = HashSet::with_capacity(dependency_ids.len());
        for dependency_id in &dependency_ids {
            if *dependency_id == source_instance_id {
                return Err(format!(
                    "analysis instance {source_instance_id} cannot depend on itself"
                ));
            }
            if !unique_dependencies.insert(*dependency_id) {
                return Err(format!(
                    "analysis instance {source_instance_id} repeats dependency {dependency_id}"
                ));
            }
        }

        Ok(Self {
            source_domain,
            source_instance_id,
            source_revision,
            prepared_snapshot_digest,
            dependency_ids,
        })
    }

    #[must_use]
    pub const fn source_domain(&self) -> AnalysisResultSourceDomain {
        self.source_domain
    }

    #[must_use]
    pub const fn source_instance_id(&self) -> AnalysisInstanceId {
        self.source_instance_id
    }

    #[must_use]
    pub const fn source_revision(&self) -> ObjectRevision {
        self.source_revision
    }

    #[must_use]
    pub const fn prepared_snapshot_digest(&self) -> ContentDigest {
        self.prepared_snapshot_digest
    }

    #[must_use]
    pub fn dependency_ids(&self) -> &[AnalysisInstanceId] {
        &self.dependency_ids
    }
}

/// Operating point data for a single node or device terminal
#[derive(Debug, Clone, PartialEq)]
pub struct OperatingPointValue {
    /// Node or terminal name (e.g., "V(out)", "I(R1)")
    pub name: String,
    /// Value in base units (volts, amps, etc.)
    pub value: f64,
    /// Unit string for display (e.g., "V", "A", "W")
    pub unit: String,
}

/// DC operating point results - node voltages and branch currents
#[derive(Debug, Clone, Default)]
pub struct DcOpResult {
    /// Node voltages
    pub node_voltages: Vec<OperatingPointValue>,
    /// Branch currents
    pub branch_currents: Vec<OperatingPointValue>,
    /// Power dissipation by device
    pub power_dissipation: Vec<OperatingPointValue>,
}

/// One row of the ranked noise-contributor table (band-integrated).
#[derive(Debug, Clone, PartialEq)]
pub struct NoiseContributorRow {
    /// Device instance name.
    pub device: String,
    /// Noise mechanism label ("thermal", "flicker", "shot", "burst").
    pub mechanism: &'static str,
    /// Output-referred noise power integrated over the band (V²).
    pub power: f64,
    /// Share of the total integrated output noise (percent).
    pub share_pct: f64,
}

/// Ranked noise summary for a noise analysis: per-device/mechanism
/// contributions plus the band total — the table analog designers read
/// first.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NoiseSummary {
    /// Contributors, ranked by integrated power, descending.
    pub rows: Vec<NoiseContributorRow>,
    /// Total integrated output noise over the band (V rms).
    pub total_rms: f64,
    /// Analysis band, for the panel header (Hz).
    pub band: (f64, f64),
}

/// Exact per-variable evidence retained from a Monte Carlo execution.
///
/// Histogram bins are deliberately not duplicated here: the presentation
/// histogram is already materialized as a waveform, while these source
/// samples and statistics are the immutable evidence needed to recompute any
/// future distribution view without inventing data.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MonteCarloVariableMetadata {
    pub name: String,
    pub samples: Vec<f64>,
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
}

/// One exact complex value retained from an analysis result.
///
/// This is deliberately independent of the pole-zero viewer's presentation
/// model. Root classification is owned by [`AnalysisResultPayload`], while
/// this value preserves the solver's ordered real/imaginary evidence.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComplexResultValue {
    pub real: f64,
    pub imaginary: f64,
}

/// Analysis basis used to produce a retained sensitivity result.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum SensitivityResultMode {
    Dc,
    Ac { frequency_hz: f64 },
}

/// One parameter's exact raw and normalized sensitivity.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SensitivityResultRow {
    pub parameter: String,
    pub raw: f64,
    pub normalized: f64,
}

/// Exact stress metrics retained for one device in a reliability run.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityStressEvidence {
    pub average_gate_stress_v: f64,
    pub average_drain_stress_v: f64,
    pub average_temperature_k: f64,
    pub duration_s: f64,
}

/// Exact parameter shifts retained at one lifetime checkpoint.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityShiftEvidence {
    pub threshold_voltage_shift_v: f64,
    pub mobility_shift: f64,
    pub drain_source_resistance_shift: f64,
}

/// One numerically ordered lifetime checkpoint and its exact parameter shifts.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityCheckpointEvidence {
    pub years: f64,
    pub shift: ReliabilityShiftEvidence,
}

/// Immutable reliability evidence for one analyzed device.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReliabilityDeviceEvidence {
    pub device_id: String,
    pub stress: ReliabilityStressEvidence,
    pub checkpoints: Vec<ReliabilityCheckpointEvidence>,
}

/// Electrical quantity governed by a retained safe-operating-area rule.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum SoaParameterEvidence {
    GateSourceVoltage,
    DrainSourceVoltage,
    GateDrainVoltage,
    BaseEmitterVoltage,
    CollectorEmitterVoltage,
    BaseCollectorVoltage,
    DrainCurrent,
    CollectorCurrent,
    PowerDissipation,
    Temperature,
}

/// Severity assigned by the SOA rule evaluator.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum SoaViolationSeverityEvidence {
    Warning,
    Violation,
    Critical,
}

/// Verdict for one fully evaluated safe-operating-area rule.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum SoaRuleVerdictEvidence {
    Pass,
    Warning,
    Violation,
    Critical,
}

/// Complete worst-point and sampling evidence for one SOA rule.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaEvaluationEvidence {
    pub device_id: String,
    pub parameter: SoaParameterEvidence,
    pub limit_value: f64,
    pub worst_actual_value: f64,
    pub worst_time_s: f64,
    pub sample_count: u64,
    pub unit: String,
    pub description: String,
    pub verdict: SoaRuleVerdictEvidence,
}

/// One exact, source-attributed safe-operating-area violation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaViolationEvidence {
    pub device_id: String,
    pub parameter: SoaParameterEvidence,
    pub limit_value: f64,
    pub actual_value: f64,
    pub time_s: f64,
    pub severity: SoaViolationSeverityEvidence,
}

/// Immutable, analysis-native result evidence that is neither waveform data
/// nor presentation state.
///
/// The payload is persisted, content-digested, and selected with its owning
/// [`AnalysisResult`]. Viewers must derive from this value instead of keeping
/// a second mutable copy of engineering data.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AnalysisResultPayload {
    PoleZero {
        poles: Vec<ComplexResultValue>,
        zeros: Vec<ComplexResultValue>,
        gain: f64,
    },
    Sensitivity {
        output: String,
        result_mode: SensitivityResultMode,
        rows: Vec<SensitivityResultRow>,
    },
    ScalarMeasurements {
        values: BTreeMap<String, f64>,
    },
    Reliability {
        devices: Vec<ReliabilityDeviceEvidence>,
    },
    Soa {
        evaluations: Vec<SoaEvaluationEvidence>,
        violations: Vec<SoaViolationEvidence>,
    },
}

impl AnalysisResultPayload {
    /// Validate exact retained evidence against the analysis that owns it.
    pub fn validate_for(&self, analysis_type: AnalysisType) -> Result<(), String> {
        match self {
            Self::PoleZero { poles, zeros, gain } => {
                if analysis_type != AnalysisType::PoleZero {
                    return Err(format!(
                        "pole-zero payload does not match analysis type {analysis_type:?}"
                    ));
                }
                validate_complex_values(poles, "pole")?;
                validate_complex_values(zeros, "zero")?;
                if !gain.is_finite() {
                    return Err("pole-zero gain is non-finite".to_owned());
                }
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
                    if !row.raw.is_finite() || !row.normalized.is_finite() {
                        return Err(format!(
                            "sensitivity parameter '{}' has a non-finite value",
                            row.parameter
                        ));
                    }
                }
            }
            Self::ScalarMeasurements { values } => {
                if matches!(
                    analysis_type,
                    AnalysisType::PoleZero | AnalysisType::Sensitivity
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
            Self::Reliability { devices } => {
                if analysis_type != AnalysisType::Reliability {
                    return Err(format!(
                        "reliability payload does not match analysis type {analysis_type:?}"
                    ));
                }
                if devices.is_empty() {
                    return Err("reliability payload contains no device evidence".to_owned());
                }
                let mut previous_device: Option<&str> = None;
                for device in devices {
                    require_non_empty(&device.device_id, "reliability device identity")?;
                    if previous_device.is_some_and(|previous| previous >= device.device_id.as_str())
                    {
                        return Err(
                            "reliability devices must have unique, strictly sorted identities"
                                .to_owned(),
                        );
                    }
                    previous_device = Some(&device.device_id);
                    for (label, value) in [
                        ("average gate stress", device.stress.average_gate_stress_v),
                        ("average drain stress", device.stress.average_drain_stress_v),
                        ("average temperature", device.stress.average_temperature_k),
                        ("stress duration", device.stress.duration_s),
                    ] {
                        if !value.is_finite() {
                            return Err(format!(
                                "reliability device '{}' has non-finite {label}",
                                device.device_id
                            ));
                        }
                    }
                    if device.stress.average_temperature_k <= 0.0 {
                        return Err(format!(
                            "reliability device '{}' has a non-positive absolute temperature",
                            device.device_id
                        ));
                    }
                    if device.stress.duration_s < 0.0 {
                        return Err(format!(
                            "reliability device '{}' has a negative stress duration",
                            device.device_id
                        ));
                    }
                    if device.checkpoints.is_empty() {
                        return Err(format!(
                            "reliability device '{}' has no lifetime checkpoints",
                            device.device_id
                        ));
                    }
                    let mut previous_years = None;
                    for checkpoint in &device.checkpoints {
                        if !checkpoint.years.is_finite() || checkpoint.years <= 0.0 {
                            return Err(format!(
                                "reliability device '{}' has an invalid lifetime checkpoint",
                                device.device_id
                            ));
                        }
                        if previous_years.is_some_and(|previous| previous >= checkpoint.years) {
                            return Err(format!(
                                "reliability device '{}' checkpoints must be unique and strictly increasing",
                                device.device_id
                            ));
                        }
                        previous_years = Some(checkpoint.years);
                        let shift = &checkpoint.shift;
                        if [
                            shift.threshold_voltage_shift_v,
                            shift.mobility_shift,
                            shift.drain_source_resistance_shift,
                        ]
                        .into_iter()
                        .any(|value| !value.is_finite())
                        {
                            return Err(format!(
                                "reliability device '{}' checkpoint '{}' years has a non-finite shift",
                                device.device_id, checkpoint.years
                            ));
                        }
                    }
                }
            }
            Self::Soa {
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
                let mut previous_evaluation: Option<&SoaEvaluationEvidence> = None;
                for evaluation in evaluations {
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
                    if evaluation.limit_value <= 0.0 {
                        return Err(format!(
                            "SOA evaluation for '{}' has a non-positive limit",
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
                    let expected_verdict =
                        soa_rule_verdict(evaluation.worst_actual_value, evaluation.limit_value);
                    if evaluation.verdict != expected_verdict {
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
                    if violation.limit_value <= 0.0 || violation.actual_value < 0.0 {
                        return Err(format!(
                            "SOA violation for '{}' has invalid magnitude evidence",
                            violation.device_id
                        ));
                    }
                    let expected_severity =
                        soa_violation_severity(violation.actual_value, violation.limit_value)
                            .ok_or_else(|| {
                                format!(
                                    "SOA event for '{}' does not meet the warning threshold",
                                    violation.device_id
                                )
                            })?;
                    if violation.severity != expected_severity {
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
        }
        Ok(())
    }

    #[must_use]
    pub fn has_data(&self) -> bool {
        match self {
            Self::PoleZero { .. } | Self::Sensitivity { .. } => true,
            Self::ScalarMeasurements { values } => !values.is_empty(),
            Self::Reliability { devices } => !devices.is_empty(),
            Self::Soa { evaluations, .. } => !evaluations.is_empty(),
        }
    }
}

fn soa_rule_verdict(actual: f64, limit: f64) -> SoaRuleVerdictEvidence {
    if actual > limit * 1.2 {
        SoaRuleVerdictEvidence::Critical
    } else if actual > limit {
        SoaRuleVerdictEvidence::Violation
    } else if actual > limit * 0.9 {
        SoaRuleVerdictEvidence::Warning
    } else {
        SoaRuleVerdictEvidence::Pass
    }
}

fn soa_violation_severity(actual: f64, limit: f64) -> Option<SoaViolationSeverityEvidence> {
    if actual > limit * 1.2 {
        Some(SoaViolationSeverityEvidence::Critical)
    } else if actual > limit {
        Some(SoaViolationSeverityEvidence::Violation)
    } else if actual > limit * 0.9 {
        Some(SoaViolationSeverityEvidence::Warning)
    } else {
        None
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

fn validate_complex_values(values: &[ComplexResultValue], label: &str) -> Result<(), String> {
    for (index, value) in values.iter().enumerate() {
        if !value.real.is_finite() || !value.imaginary.is_finite() {
            return Err(format!("{label} {index} has a non-finite component"));
        }
    }
    Ok(())
}

/// Typed, lossless metadata for result families whose execution contract is
/// richer than a collection of plotted waveforms.
///
/// This payload is part of the immutable analysis result. It preserves exact
/// axes, labels, run counts, statistical samples, and convergence outcomes
/// that cannot be reconstructed truthfully from display waveforms alone.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "family", rename_all = "snake_case", deny_unknown_fields)]
pub enum AnalysisResultFamilyMetadata {
    Parametric {
        target: String,
        sweep_values: Vec<f64>,
        failed_points: usize,
    },
    Corner {
        x_values: Vec<f64>,
        x_label: String,
        x_unit: String,
        temperatures_c: Vec<f64>,
        corner_labels: Vec<String>,
        failed_corners: usize,
    },
    MonteCarlo {
        seed: u64,
        runs_requested: usize,
        runs_completed: usize,
        failures: usize,
        all_converged: bool,
        variables: Vec<MonteCarloVariableMetadata>,
    },
    Reliability {
        years: Vec<f64>,
    },
    Optimization {
        iterations: Vec<f64>,
        best_cost: f64,
        best_variables: BTreeMap<String, f64>,
        converged: bool,
    },
    Soa {
        time: Vec<f64>,
    },
}

impl AnalysisResultFamilyMetadata {
    /// Validate retained source evidence independently of any viewer.
    pub fn validate_for(&self, analysis_type: AnalysisType) -> Result<(), String> {
        let compatible = matches!(
            (self, analysis_type),
            (Self::Parametric { .. }, AnalysisType::Parametric)
                | (Self::Corner { .. }, AnalysisType::Corner)
                | (Self::MonteCarlo { .. }, AnalysisType::MonteCarlo)
                | (Self::Reliability { .. }, AnalysisType::Reliability)
                | (Self::Optimization { .. }, AnalysisType::Optimization)
                | (Self::Soa { .. }, AnalysisType::Soa)
        );
        if !compatible {
            return Err(format!(
                "retained family metadata does not match analysis type {analysis_type:?}"
            ));
        }

        match self {
            Self::Parametric {
                target,
                sweep_values,
                ..
            } => {
                require_non_empty(target, "parametric target")?;
                require_finite_values(sweep_values, "parametric sweep values")?;
            }
            Self::Corner {
                x_values,
                x_label,
                temperatures_c,
                corner_labels,
                ..
            } => {
                require_non_empty(x_label, "corner x-axis label")?;
                require_finite_values(x_values, "corner x-axis values")?;
                require_finite_values(temperatures_c, "corner temperatures")?;
                if temperatures_c.len() != x_values.len() || corner_labels.len() != x_values.len() {
                    return Err(
                        "corner x values, temperatures, and labels have different lengths"
                            .to_owned(),
                    );
                }
                if corner_labels.iter().any(|label| label.trim().is_empty()) {
                    return Err("corner metadata contains an empty corner label".to_owned());
                }
            }
            Self::MonteCarlo {
                runs_requested,
                runs_completed,
                failures,
                all_converged,
                variables,
                ..
            } => {
                if runs_completed.saturating_add(*failures) > *runs_requested {
                    return Err(
                        "Monte Carlo completed and failed counts exceed requested runs".to_owned(),
                    );
                }
                if *all_converged && (*failures != 0 || runs_completed != runs_requested) {
                    return Err(
                        "Monte Carlo all_converged contradicts retained run counts".to_owned()
                    );
                }
                let mut names = HashSet::with_capacity(variables.len());
                for variable in variables {
                    require_non_empty(&variable.name, "Monte Carlo variable name")?;
                    if !names.insert(variable.name.as_str()) {
                        return Err(format!(
                            "Monte Carlo metadata repeats variable '{}'",
                            variable.name
                        ));
                    }
                    require_finite_values(&variable.samples, "Monte Carlo samples")?;
                    for (label, value) in [
                        ("mean", variable.mean),
                        ("standard deviation", variable.std_dev),
                        ("minimum", variable.min),
                        ("maximum", variable.max),
                    ] {
                        if !value.is_finite() {
                            return Err(format!(
                                "Monte Carlo variable '{}' has non-finite {label}",
                                variable.name
                            ));
                        }
                    }
                    if variable.std_dev < 0.0 || variable.min > variable.max {
                        return Err(format!(
                            "Monte Carlo variable '{}' has inconsistent statistics",
                            variable.name
                        ));
                    }
                }
            }
            Self::Reliability { years } => {
                require_finite_values(years, "reliability years")?;
                if years.is_empty()
                    || years.iter().any(|years| *years <= 0.0)
                    || !strictly_increasing(years)
                {
                    return Err(
                        "reliability years must be non-empty, positive, unique, and strictly increasing"
                            .to_owned(),
                    );
                }
            }
            Self::Optimization {
                iterations,
                best_cost,
                best_variables,
                ..
            } => {
                require_finite_values(iterations, "optimization iterations")?;
                if !best_cost.is_finite() {
                    return Err("optimization best cost is non-finite".to_owned());
                }
                for (name, value) in best_variables {
                    require_non_empty(name, "optimization variable name")?;
                    if !value.is_finite() {
                        return Err(format!(
                            "optimization variable '{name}' has a non-finite best value"
                        ));
                    }
                }
            }
            Self::Soa { time } => {
                require_finite_values(time, "SOA time")?;
                if time.is_empty()
                    || time.iter().any(|time| *time < 0.0)
                    || !strictly_increasing(time)
                {
                    return Err(
                        "SOA time must be non-empty, nonnegative, unique, and strictly increasing"
                            .to_owned(),
                    );
                }
            }
        }
        Ok(())
    }
}

fn require_non_empty(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{label} is empty"))
    } else {
        Ok(())
    }
}

fn require_finite_values(values: &[f64], label: &str) -> Result<(), String> {
    if values.iter().any(|value| !value.is_finite()) {
        Err(format!("{label} contain a non-finite value"))
    } else {
        Ok(())
    }
}

fn strictly_increasing(values: &[f64]) -> bool {
    values
        .windows(2)
        .all(|pair| normalized_f64(pair[0]) < normalized_f64(pair[1]))
}

fn normalized_f64(value: f64) -> f64 {
    if value == 0.0 { 0.0 } else { value }
}

fn same_retained_float(left: f64, right: f64) -> bool {
    normalized_f64(left).to_bits() == normalized_f64(right).to_bits()
}

fn contains_retained_coordinate(sorted: &[f64], target: f64) -> bool {
    let target = normalized_f64(target);
    sorted
        .binary_search_by(|probe| normalized_f64(*probe).total_cmp(&target))
        .is_ok()
}

/// Single analysis result with metadata and waveforms.
///
/// This represents one analysis within a simulation run, containing
/// all the data needed to display results in the appropriate viewer.
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// Unique ID within the simulation run
    pub id: u64,
    /// Analysis type for viewer selection
    pub analysis_type: AnalysisType,
    /// Human-readable label with parameters (e.g., "AC (1Hz-1GHz)")
    pub label: String,
    /// Unix timestamp when analysis completed
    pub timestamp: f64,
    /// Time-domain or frequency-domain waveforms (for sweep analyses)
    pub waveforms: Vec<WaveformData>,
    /// DC operating point data (for DC Op analysis)
    pub dc_op: Option<DcOpResult>,
    /// Per-device operating point report (bias + small-signal parameters,
    /// the Spectre-style OP info), for DC Op analyses.
    pub device_op: Option<rspice_core::circuit::DeviceOpReport>,
    /// Ranked, band-integrated noise contributors, for noise analyses.
    pub noise_summary: Option<NoiseSummary>,
    /// Exact typed metadata for multi-run and advanced result families. This
    /// is source evidence, not presentation state, and must survive project
    /// and session persistence unchanged.
    pub family_metadata: Option<AnalysisResultFamilyMetadata>,
    /// Exact analysis-native evidence such as pole/zero roots, sensitivity
    /// rows, or scalar-only results. This is immutable retained data, not a
    /// viewer cache.
    pub result_payload: Option<AnalysisResultPayload>,
    /// Evaluated `.MEAS` results for this analysis (specs-matrix rows).
    pub measurements: Vec<rspice_core::MeasureResult>,
    /// Authenticated application receipts for plan-owned saved-output
    /// contracts. The materialized waveform remains in `waveforms`; the
    /// receipt proves why it exists and records deferred/suppressed outcomes.
    pub saved_output_receipts: Vec<SavedOutputReceipt>,
    /// Whether this analysis completed successfully
    pub success: bool,
    /// Error message if analysis failed
    pub error_message: Option<String>,
    /// Exact prepared-task identity. Missing only for migrated legacy result
    /// history that was written before source instance IDs existed.
    pub provenance: Option<AnalysisResultProvenance>,
}

impl AnalysisResult {
    /// Create a new successful analysis result
    pub fn new(id: u64, analysis_type: AnalysisType, label: impl Into<String>) -> Self {
        Self {
            id,
            analysis_type,
            label: label.into(),
            timestamp: Self::current_timestamp(),
            waveforms: Vec::new(),
            dc_op: None,
            device_op: None,
            noise_summary: None,
            family_metadata: None,
            result_payload: None,
            measurements: Vec::new(),
            saved_output_receipts: Vec::new(),
            success: true,
            error_message: None,
            provenance: None,
        }
    }

    /// Create a failed analysis result
    pub fn failed(
        id: u64,
        analysis_type: AnalysisType,
        label: impl Into<String>,
        error: impl Into<String>,
    ) -> Self {
        Self {
            id,
            analysis_type,
            label: label.into(),
            timestamp: Self::current_timestamp(),
            waveforms: Vec::new(),
            dc_op: None,
            device_op: None,
            noise_summary: None,
            family_metadata: None,
            result_payload: None,
            measurements: Vec::new(),
            saved_output_receipts: Vec::new(),
            success: false,
            error_message: Some(error.into()),
            provenance: None,
        }
    }

    /// Add waveform data to this analysis
    pub fn with_waveforms(mut self, waveforms: Vec<WaveformData>) -> Self {
        self.waveforms = waveforms;
        self
    }

    /// Add DC operating point data
    pub fn with_dc_op(mut self, dc_op: DcOpResult) -> Self {
        self.dc_op = Some(dc_op);
        self
    }

    /// Attach the per-device operating-point report.
    pub fn with_device_op(mut self, report: rspice_core::circuit::DeviceOpReport) -> Self {
        if !report.is_empty() {
            self.device_op = Some(report);
        }
        self
    }

    /// Attach the ranked noise-contributor summary.
    pub fn with_noise_summary(mut self, summary: NoiseSummary) -> Self {
        if !summary.rows.is_empty() {
            self.noise_summary = Some(summary);
        }
        self
    }

    /// Attach exact source metadata for an advanced result family.
    #[must_use]
    pub fn with_family_metadata(mut self, metadata: AnalysisResultFamilyMetadata) -> Self {
        debug_assert!(metadata.validate_for(self.analysis_type).is_ok());
        self.family_metadata = Some(metadata);
        self
    }

    /// Attach exact analysis-native result evidence.
    #[must_use]
    pub fn with_result_payload(mut self, payload: AnalysisResultPayload) -> Self {
        debug_assert!(payload.validate_for(self.analysis_type).is_ok());
        self.result_payload = Some(payload);
        self
    }

    /// Attach evaluated `.MEAS` results.
    pub fn with_measurements(mut self, measurements: Vec<rspice_core::MeasureResult>) -> Self {
        self.measurements = measurements;
        self
    }

    /// Attach the exact output-contract receipts produced during completion.
    #[must_use]
    pub fn with_saved_output_receipts(mut self, receipts: Vec<SavedOutputReceipt>) -> Self {
        self.saved_output_receipts = receipts;
        self
    }

    /// Attach the exact prepared task that produced this result.
    #[must_use]
    pub fn with_provenance(mut self, provenance: AnalysisResultProvenance) -> Self {
        self.provenance = Some(provenance);
        self
    }

    /// Get current timestamp as Unix epoch seconds
    fn current_timestamp() -> f64 {
        crate::common::time_compat::unix_epoch().as_secs_f64()
    }

    /// Check if this analysis has any viewable data
    pub fn has_data(&self) -> bool {
        !self.waveforms.is_empty()
            || self.dc_op.is_some()
            || self
                .result_payload
                .as_ref()
                .is_some_and(AnalysisResultPayload::has_data)
    }

    /// Validate relationships between independently versioned retained fields.
    /// Historical analyses may legitimately lack a newer payload; when both
    /// fields exist they must describe one coherent execution.
    pub fn validate_retained_evidence(&self) -> Result<(), String> {
        if let Some(metadata) = &self.family_metadata {
            metadata.validate_for(self.analysis_type)?;
        }
        if let Some(payload) = &self.result_payload {
            payload.validate_for(self.analysis_type)?;
        }

        match (&self.family_metadata, &self.result_payload) {
            (None, Some(AnalysisResultPayload::Reliability { .. })) => {
                return Err("reliability payload is missing its retained lifetime axis".to_owned());
            }
            (None, Some(AnalysisResultPayload::Soa { .. })) => {
                return Err("SOA payload is missing its retained time axis".to_owned());
            }
            (
                Some(AnalysisResultFamilyMetadata::Reliability { years }),
                Some(AnalysisResultPayload::Reliability { devices }),
            ) => {
                for device in devices {
                    if device.checkpoints.len() != years.len()
                        || !device
                            .checkpoints
                            .iter()
                            .zip(years)
                            .all(|(checkpoint, years)| {
                                same_retained_float(checkpoint.years, *years)
                            })
                    {
                        return Err(format!(
                            "reliability device '{}' checkpoints do not match the retained lifetime axis",
                            device.device_id
                        ));
                    }
                }
            }
            (
                Some(AnalysisResultFamilyMetadata::Soa { time }),
                Some(AnalysisResultPayload::Soa {
                    evaluations,
                    violations,
                }),
            ) => {
                let rules = evaluations
                    .iter()
                    .map(|evaluation| {
                        (
                            (evaluation.device_id.as_str(), evaluation.parameter),
                            evaluation,
                        )
                    })
                    .collect::<std::collections::BTreeMap<_, _>>();
                let mut exact_worst_events = std::collections::BTreeSet::new();
                for violation in violations {
                    let key = (violation.device_id.as_str(), violation.parameter);
                    let evaluation = rules.get(&key).ok_or_else(|| {
                        format!(
                            "SOA event for '{}' has no matching evaluated rule",
                            violation.device_id
                        )
                    })?;
                    if !same_retained_float(violation.limit_value, evaluation.limit_value) {
                        return Err(format!(
                            "SOA event for '{}' contradicts its evaluated rule limit",
                            violation.device_id
                        ));
                    }
                    if violation.actual_value > evaluation.worst_actual_value {
                        return Err(format!(
                            "SOA event for '{}' exceeds its retained worst point",
                            violation.device_id
                        ));
                    }
                    if !contains_retained_coordinate(time, violation.time_s) {
                        return Err(format!(
                            "SOA event for '{}' does not reference an exact retained sample",
                            violation.device_id
                        ));
                    }
                    let expected_severity = match evaluation.verdict {
                        SoaRuleVerdictEvidence::Pass => None,
                        SoaRuleVerdictEvidence::Warning => {
                            Some(SoaViolationSeverityEvidence::Warning)
                        }
                        SoaRuleVerdictEvidence::Violation => {
                            Some(SoaViolationSeverityEvidence::Violation)
                        }
                        SoaRuleVerdictEvidence::Critical => {
                            Some(SoaViolationSeverityEvidence::Critical)
                        }
                    };
                    if expected_severity == Some(violation.severity)
                        && same_retained_float(
                            violation.actual_value,
                            evaluation.worst_actual_value,
                        )
                        && same_retained_float(violation.time_s, evaluation.worst_time_s)
                    {
                        exact_worst_events.insert(key);
                    }
                }
                for evaluation in evaluations {
                    let retained_sample_count = u64::try_from(time.len())
                        .map_err(|_| "SOA time axis exceeds the retained count range".to_owned())?;
                    if evaluation.sample_count != retained_sample_count {
                        return Err(format!(
                            "SOA evaluation for '{}' covers {} samples but the retained run has {}",
                            evaluation.device_id,
                            evaluation.sample_count,
                            time.len()
                        ));
                    }
                    if !contains_retained_coordinate(time, evaluation.worst_time_s) {
                        return Err(format!(
                            "SOA evaluation for '{}' does not reference an exact retained sample",
                            evaluation.device_id
                        ));
                    }
                    let key = (evaluation.device_id.as_str(), evaluation.parameter);
                    if evaluation.verdict != SoaRuleVerdictEvidence::Pass
                        && !exact_worst_events.contains(&key)
                    {
                        return Err(format!(
                            "SOA evaluation for '{}' has no exact event at its worst point",
                            evaluation.device_id
                        ));
                    }
                }
            }
            (Some(AnalysisResultFamilyMetadata::Reliability { .. }), Some(payload))
                if !matches!(payload, AnalysisResultPayload::Reliability { .. }) =>
            {
                return Err("reliability metadata has a mismatched retained payload".to_owned());
            }
            (Some(AnalysisResultFamilyMetadata::Soa { .. }), Some(payload))
                if !matches!(payload, AnalysisResultPayload::Soa { .. }) =>
            {
                return Err("SOA metadata has a mismatched retained payload".to_owned());
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod retained_payload_tests {
    use super::*;

    #[test]
    fn pole_zero_payload_requires_matching_type_and_finite_values() {
        let payload = AnalysisResultPayload::PoleZero {
            poles: vec![ComplexResultValue {
                real: -1.0,
                imaginary: 2.0,
            }],
            zeros: Vec::new(),
            gain: 1.0,
        };
        assert!(payload.validate_for(AnalysisType::PoleZero).is_ok());
        assert!(payload.validate_for(AnalysisType::Ac).is_err());

        let invalid = AnalysisResultPayload::PoleZero {
            poles: vec![ComplexResultValue {
                real: f64::INFINITY,
                imaginary: 0.0,
            }],
            zeros: Vec::new(),
            gain: 1.0,
        };
        assert!(invalid.validate_for(AnalysisType::PoleZero).is_err());
    }

    #[test]
    fn sensitivity_payload_requires_canonical_unique_rows_and_valid_basis() {
        let valid = AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: SensitivityResultMode::Ac {
                frequency_hz: 1_000.0,
            },
            rows: vec![
                SensitivityResultRow {
                    parameter: "length".to_owned(),
                    raw: -1.0,
                    normalized: -0.25,
                },
                SensitivityResultRow {
                    parameter: "width".to_owned(),
                    raw: 2.0,
                    normalized: 0.5,
                },
            ],
        };
        assert!(valid.validate_for(AnalysisType::Sensitivity).is_ok());

        let duplicate = AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: SensitivityResultMode::Dc,
            rows: vec![
                SensitivityResultRow {
                    parameter: "width".to_owned(),
                    raw: 1.0,
                    normalized: 1.0,
                },
                SensitivityResultRow {
                    parameter: "width".to_owned(),
                    raw: 2.0,
                    normalized: 2.0,
                },
            ],
        };
        assert!(duplicate.validate_for(AnalysisType::Sensitivity).is_err());

        let invalid_frequency = AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: SensitivityResultMode::Ac { frequency_hz: 0.0 },
            rows: Vec::new(),
        };
        assert!(
            invalid_frequency
                .validate_for(AnalysisType::Sensitivity)
                .is_err()
        );
    }

    #[test]
    fn reliability_payload_requires_canonical_devices_and_exact_lifetime_coverage() {
        let device = ReliabilityDeviceEvidence {
            device_id: "M1".to_owned(),
            stress: ReliabilityStressEvidence {
                average_gate_stress_v: 1.2,
                average_drain_stress_v: 1.8,
                average_temperature_k: 358.15,
                duration_s: 3_600.0,
            },
            checkpoints: vec![
                ReliabilityCheckpointEvidence {
                    years: 1.0,
                    shift: ReliabilityShiftEvidence {
                        threshold_voltage_shift_v: 0.01,
                        mobility_shift: -0.001,
                        drain_source_resistance_shift: 0.0005,
                    },
                },
                ReliabilityCheckpointEvidence {
                    years: 10.0,
                    shift: ReliabilityShiftEvidence {
                        threshold_voltage_shift_v: 0.03,
                        mobility_shift: -0.004,
                        drain_source_resistance_shift: 0.0015,
                    },
                },
            ],
        };
        let valid = AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
            .with_family_metadata(AnalysisResultFamilyMetadata::Reliability {
                years: vec![1.0, 10.0],
            })
            .with_result_payload(AnalysisResultPayload::Reliability {
                devices: vec![device.clone()],
            });
        assert!(valid.validate_retained_evidence().is_ok());

        let payload_without_axis = AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
            .with_result_payload(AnalysisResultPayload::Reliability {
                devices: vec![device.clone()],
            });
        assert!(
            payload_without_axis
                .validate_retained_evidence()
                .expect_err("reliability payload requires its lifetime axis")
                .contains("missing its retained lifetime axis")
        );

        let incomplete = AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
            .with_family_metadata(AnalysisResultFamilyMetadata::Reliability {
                years: vec![1.0, 5.0, 10.0],
            })
            .with_result_payload(AnalysisResultPayload::Reliability {
                devices: vec![device],
            });
        assert!(
            incomplete
                .validate_retained_evidence()
                .expect_err("missing lifetime evidence is rejected")
                .contains("do not match")
        );
    }

    #[test]
    fn soa_payload_requires_complete_rule_coverage_consistent_events_and_axis() {
        let evaluation = SoaEvaluationEvidence {
            device_id: "M1".to_owned(),
            parameter: SoaParameterEvidence::DrainSourceVoltage,
            limit_value: 3.3,
            worst_actual_value: 3.2,
            worst_time_s: 1.0,
            sample_count: 2,
            unit: "V".to_owned(),
            description: "Maximum drain-source voltage".to_owned(),
            verdict: SoaRuleVerdictEvidence::Warning,
        };
        let event = SoaViolationEvidence {
            device_id: "M1".to_owned(),
            parameter: SoaParameterEvidence::DrainSourceVoltage,
            limit_value: 3.3,
            actual_value: 3.2,
            time_s: 1.0,
            severity: SoaViolationSeverityEvidence::Warning,
        };
        let valid = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
            .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
                time: vec![0.0, 1.0],
            })
            .with_result_payload(AnalysisResultPayload::Soa {
                evaluations: vec![evaluation.clone()],
                violations: vec![event.clone()],
            });
        assert!(valid.validate_retained_evidence().is_ok());

        let payload_without_axis = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
            .with_result_payload(AnalysisResultPayload::Soa {
                evaluations: vec![evaluation.clone()],
                violations: vec![event.clone()],
            });
        assert!(
            payload_without_axis
                .validate_retained_evidence()
                .expect_err("SOA payload requires its time axis")
                .contains("missing its retained time axis")
        );

        let mut invalid_event = event.clone();
        invalid_event.severity = SoaViolationSeverityEvidence::Critical;
        assert!(
            AnalysisResultPayload::Soa {
                evaluations: vec![evaluation.clone()],
                violations: vec![invalid_event],
            }
            .validate_for(AnalysisType::Soa)
            .expect_err("contradictory event severity is rejected")
            .contains("severity")
        );

        let contradictory_limit = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
            .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
                time: vec![0.0, 1.0],
            })
            .with_result_payload(AnalysisResultPayload::Soa {
                evaluations: vec![evaluation.clone()],
                violations: vec![SoaViolationEvidence {
                    limit_value: 3.4,
                    ..event.clone()
                }],
            });
        assert!(
            contradictory_limit
                .validate_retained_evidence()
                .expect_err("event rule limit must be exact")
                .contains("contradicts its evaluated rule limit")
        );

        let missing_worst_event = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
            .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
                time: vec![0.0, 1.0],
            })
            .with_result_payload(AnalysisResultPayload::Soa {
                evaluations: vec![evaluation.clone()],
                violations: Vec::new(),
            });
        assert!(
            missing_worst_event
                .validate_retained_evidence()
                .expect_err("non-pass verdict requires exact worst event")
                .contains("no exact event at its worst point")
        );

        let incomplete = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
            .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
                time: vec![0.0, 0.5, 1.0],
            })
            .with_result_payload(AnalysisResultPayload::Soa {
                evaluations: vec![evaluation],
                violations: Vec::new(),
            });
        assert!(
            incomplete
                .validate_retained_evidence()
                .expect_err("incomplete sample coverage is rejected")
                .contains("covers 2 samples")
        );
    }

    #[test]
    fn reliability_and_soa_axes_are_canonical_engineering_coordinates() {
        for years in [Vec::new(), vec![0.0], vec![10.0, 1.0], vec![1.0, 1.0]] {
            assert!(
                AnalysisResultFamilyMetadata::Reliability { years }
                    .validate_for(AnalysisType::Reliability)
                    .is_err()
            );
        }
        for time in [Vec::new(), vec![-1.0, 0.0], vec![0.0, 0.0], vec![1.0, 0.0]] {
            assert!(
                AnalysisResultFamilyMetadata::Soa { time }
                    .validate_for(AnalysisType::Soa)
                    .is_err()
            );
        }
        assert!(
            AnalysisResultFamilyMetadata::Reliability {
                years: vec![1.0, 5.0, 10.0],
            }
            .validate_for(AnalysisType::Reliability)
            .is_ok()
        );
        assert!(
            AnalysisResultFamilyMetadata::Soa {
                time: vec![-0.0, 1.0e-9, 2.0e-9],
            }
            .validate_for(AnalysisType::Soa)
            .is_ok()
        );
    }
}
