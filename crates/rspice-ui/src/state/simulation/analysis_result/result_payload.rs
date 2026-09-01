//! Validation rules for retained analysis payloads and their numerical evidence.

use super::*;

pub(super) fn validate_pss_floquet_payload(
    period_s: Option<f64>,
    fundamental_frequency_hz: Option<f64>,
    _iterations: Option<u64>,
    residual_norm: Option<f64>,
    multipliers: &[PssFloquetMultiplierEvidence],
    evidence: &FloquetSpectrumEvidence,
    orbit_kind: FloquetOrbitKindEvidence,
    trivial_multiplier_index: Option<u64>,
    verdict: FloquetStabilityVerdictEvidence,
) -> Result<(), String> {
    if matches!(evidence, FloquetSpectrumEvidence::LegacyUnknown)
        || orbit_kind == FloquetOrbitKindEvidence::LegacyUnknown
    {
        let exact_legacy_marker = period_s.is_none()
            && fundamental_frequency_hz.is_none()
            && _iterations.is_none()
            && residual_norm.is_none()
            && multipliers.is_empty()
            && matches!(evidence, FloquetSpectrumEvidence::LegacyUnknown)
            && orbit_kind == FloquetOrbitKindEvidence::LegacyUnknown
            && trivial_multiplier_index.is_none()
            && verdict == FloquetStabilityVerdictEvidence::Indeterminate;
        return exact_legacy_marker.then_some(()).ok_or_else(|| {
            "legacy PSS Floquet evidence is not an exact migration marker".to_owned()
        });
    }

    let (Some(period_s), Some(frequency_hz), Some(_), Some(residual_norm)) = (
        period_s,
        fundamental_frequency_hz,
        _iterations,
        residual_norm,
    ) else {
        return Err("current PSS Floquet payload is missing global metrics".to_owned());
    };
    if !period_s.is_finite()
        || period_s <= 0.0
        || !frequency_hz.is_finite()
        || frequency_hz <= 0.0
        || !same_retained_float(frequency_hz, 1.0 / period_s)
        || !residual_norm.is_finite()
        || residual_norm < 0.0
    {
        return Err("PSS Floquet period, frequency, or residual is invalid".to_owned());
    }

    let values = multipliers
        .iter()
        .map(|mode| mode.multiplier)
        .collect::<Vec<_>>();
    validate_complex_values(&values, "PSS Floquet multiplier")?;
    if !evidence.is_consistent_with_count(values.len()) {
        return Err(
            "PSS Floquet certificate does not cover the complete multiplier vector".to_owned(),
        );
    }
    if matches!(evidence, FloquetSpectrumEvidence::NoDynamicModes)
        && orbit_kind != FloquetOrbitKindEvidence::Driven
    {
        return Err(
            "a zero-order PSS Floquet spectrum must use the driven orbit policy".to_owned(),
        );
    }

    let expected_trivial = expected_trivial_floquet_index(&values, evidence, orbit_kind)?;
    if trivial_multiplier_index != expected_trivial {
        return Err("PSS autonomous phase-mode index is inconsistent with the spectrum".to_owned());
    }
    let expected_verdict = derive_floquet_verdict(
        &values,
        evidence,
        orbit_kind,
        trivial_multiplier_index,
        rspice_core::analysis::FLOQUET_UNIT_CIRCLE_BAND,
    )?;
    if verdict != expected_verdict {
        return Err("PSS stability verdict is inconsistent with its Floquet evidence".to_owned());
    }
    Ok(())
}

pub(super) fn validate_pstb_payload(
    period_s: Option<f64>,
    fundamental_frequency_hz: Option<f64>,
    stability_threshold: Option<f64>,
    probe_instance: Option<&str>,
    detect_subharmonics: Option<bool>,
    modes: &[PstbFloquetModeEvidence],
    evidence: &FloquetSpectrumEvidence,
    orbit_kind: FloquetOrbitKindEvidence,
    trivial_multiplier_index: Option<u64>,
    verdict: FloquetStabilityVerdictEvidence,
    classification: PstbStabilityClassificationEvidence,
    min_stability_margin_db: Option<f64>,
    max_multiplier_magnitude: Option<f64>,
    num_unstable: Option<u64>,
    subharmonics: &[u64],
    converged: Option<bool>,
    iterations: Option<u64>,
) -> Result<(), String> {
    if matches!(evidence, FloquetSpectrumEvidence::LegacyUnknown)
        || orbit_kind == FloquetOrbitKindEvidence::LegacyUnknown
    {
        let exact_legacy_marker = period_s.is_none()
            && fundamental_frequency_hz.is_none()
            && stability_threshold.is_none()
            && probe_instance.is_none()
            && detect_subharmonics.is_none()
            && modes.is_empty()
            && matches!(evidence, FloquetSpectrumEvidence::LegacyUnknown)
            && orbit_kind == FloquetOrbitKindEvidence::LegacyUnknown
            && trivial_multiplier_index.is_none()
            && verdict == FloquetStabilityVerdictEvidence::Indeterminate
            && classification == PstbStabilityClassificationEvidence::Indeterminate
            && min_stability_margin_db.is_none()
            && max_multiplier_magnitude.is_none()
            && num_unstable.is_none()
            && subharmonics.is_empty()
            && converged.is_none()
            && iterations.is_none();
        return exact_legacy_marker
            .then_some(())
            .ok_or_else(|| "legacy PSTB evidence is not an exact migration marker".to_owned());
    }

    let (
        Some(period_s),
        Some(frequency_hz),
        Some(stability_threshold),
        Some(probe_instance),
        Some(detect_subharmonics),
        Some(max_multiplier_magnitude),
        Some(num_unstable),
        Some(true),
        Some(_),
    ) = (
        period_s,
        fundamental_frequency_hz,
        stability_threshold,
        probe_instance,
        detect_subharmonics,
        max_multiplier_magnitude,
        num_unstable,
        converged,
        iterations,
    )
    else {
        return Err(
            "current PSTB payload is missing provenance, convergence, or global metrics".to_owned(),
        );
    };
    if !period_s.is_finite()
        || period_s <= 0.0
        || !frequency_hz.is_finite()
        || frequency_hz <= 0.0
        || !same_retained_float(frequency_hz, 1.0 / period_s)
        || !stability_threshold.is_finite()
        || stability_threshold < 1.0
        || probe_instance.is_empty()
        || probe_instance.trim() != probe_instance
        || probe_instance
            .chars()
            .any(|character| character.is_control() || character.is_whitespace())
    {
        return Err(
            "PSTB period, frequency, stability boundary, or probe identity is invalid".to_owned(),
        );
    }
    if !matches!(
        evidence,
        FloquetSpectrumEvidence::NoDynamicModes | FloquetSpectrumEvidence::Qualified { .. }
    ) || !evidence.is_consistent_with_count(modes.len())
    {
        return Err("PSTB requires a complete current Floquet spectrum".to_owned());
    }
    if matches!(evidence, FloquetSpectrumEvidence::NoDynamicModes)
        && orbit_kind != FloquetOrbitKindEvidence::Driven
    {
        return Err("a zero-order PSTB spectrum must use the driven orbit policy".to_owned());
    }

    let values = modes.iter().map(|mode| mode.multiplier).collect::<Vec<_>>();
    validate_complex_values(&values, "PSTB Floquet multiplier")?;
    if !pstb_modes_are_canonically_sorted(modes) {
        return Err("PSTB Floquet modes are not in canonical sorted order".to_owned());
    }
    let expected_trivial = expected_trivial_floquet_index(&values, evidence, orbit_kind)?;
    if trivial_multiplier_index != expected_trivial {
        return Err(
            "PSTB autonomous phase-mode index is inconsistent with the spectrum".to_owned(),
        );
    }
    let trivial_index = trivial_multiplier_index
        .map(usize::try_from)
        .transpose()
        .map_err(|_| "PSTB phase-mode index does not fit this platform".to_owned())?;

    let mut expected_subharmonics = Vec::new();
    let mut expected_unstable_count = 0_u64;
    let mut expected_min_margin: Option<f64> = None;
    for (index, mode) in modes.iter().enumerate() {
        let value = complex_value(mode.multiplier);
        let magnitude = value.norm();
        if !magnitude.is_finite()
            || magnitude <= 0.0
            || !mode.exponent.real.is_finite()
            || !mode.exponent.imaginary.is_finite()
            || !mode.probe_participation.is_finite()
            || !(0.0..=1.0).contains(&mode.probe_participation)
        {
            return Err(format!(
                "PSTB Floquet mode {index} contains invalid numerical data"
            ));
        }
        let expected_exponent = value.ln() / period_s;
        if !same_retained_float(mode.exponent.real, expected_exponent.re)
            || !same_retained_float(mode.exponent.imaginary, expected_exponent.im)
        {
            return Err(format!(
                "PSTB Floquet mode {index} has an inconsistent exponent"
            ));
        }
        let expected_trivial_flag = trivial_index == Some(index);
        let expected_unstable = !expected_trivial_flag && magnitude > stability_threshold;
        if mode.is_trivial != expected_trivial_flag || mode.is_unstable != expected_unstable {
            return Err(format!(
                "PSTB Floquet mode {index} has inconsistent stability flags"
            ));
        }
        expected_unstable_count += u64::from(expected_unstable);

        if !expected_trivial_flag {
            let margin = -20.0 * magnitude.log10();
            if !margin.is_finite() {
                return Err(format!("PSTB Floquet mode {index} has a non-finite margin"));
            }
            expected_min_margin = Some(match expected_min_margin {
                Some(current) if current.total_cmp(&margin).is_le() => current,
                _ => margin,
            });
        }

        let detected_order = detect_subharmonics
            .then(|| detected_subharmonic_order(value))
            .flatten();
        if mode.subharmonic_order != detected_order {
            return Err(format!(
                "PSTB Floquet mode {index} has inconsistent subharmonic evidence"
            ));
        }
        if let Some(order) = detected_order {
            expected_subharmonics.push(order);
        }
    }

    let expected_max_magnitude = modes
        .first()
        .map_or(0.0, |mode| complex_value(mode.multiplier).norm());
    if !max_multiplier_magnitude.is_finite()
        || !same_retained_float(max_multiplier_magnitude, expected_max_magnitude)
        || num_unstable != expected_unstable_count
        || !same_optional_retained_float(min_stability_margin_db, expected_min_margin)
        || subharmonics != expected_subharmonics
    {
        return Err(
            "PSTB aggregate counts, margins, or subharmonics contradict the complete spectrum"
                .to_owned(),
        );
    }

    let expected_verdict = derive_floquet_verdict(
        &values,
        evidence,
        orbit_kind,
        trivial_multiplier_index,
        stability_threshold - 1.0,
    )?;
    if verdict != expected_verdict {
        return Err("PSTB stability verdict contradicts the complete Floquet spectrum".to_owned());
    }
    let expected_classification = classify_pstb_modes(modes, verdict, trivial_index)?;
    if classification != expected_classification {
        return Err(
            "PSTB rich stability classification contradicts the complete spectrum".to_owned(),
        );
    }
    Ok(())
}

fn expected_trivial_floquet_index(
    values: &[ComplexResultValue],
    evidence: &FloquetSpectrumEvidence,
    orbit_kind: FloquetOrbitKindEvidence,
) -> Result<Option<u64>, String> {
    match orbit_kind {
        FloquetOrbitKindEvidence::Driven => Ok(None),
        FloquetOrbitKindEvidence::Autonomous => {
            if !matches!(evidence, FloquetSpectrumEvidence::Qualified { .. }) {
                return Ok(None);
            }
            let values = values
                .iter()
                .copied()
                .map(complex_value)
                .collect::<Vec<_>>();
            rspice_core::analysis::select_autonomous_phase_mode(&values)
                .map(u64::try_from)
                .transpose()
                .map_err(|_| {
                    "Floquet phase-mode index does not fit the durable contract".to_owned()
                })
        }
        FloquetOrbitKindEvidence::LegacyUnknown => {
            Err("current Floquet evidence has an unknown orbit policy".to_owned())
        }
    }
}

fn derive_floquet_verdict(
    values: &[ComplexResultValue],
    evidence: &FloquetSpectrumEvidence,
    orbit_kind: FloquetOrbitKindEvidence,
    trivial_multiplier_index: Option<u64>,
    band: f64,
) -> Result<FloquetStabilityVerdictEvidence, String> {
    let values = values
        .iter()
        .copied()
        .map(complex_value)
        .collect::<Vec<_>>();
    let evidence = evidence
        .as_core()
        .ok_or_else(|| "Floquet certificate is not core-authentic".to_owned())?;
    let orbit_kind = match orbit_kind {
        FloquetOrbitKindEvidence::Driven => rspice_core::analysis::FloquetOrbitKind::Driven,
        FloquetOrbitKindEvidence::Autonomous => rspice_core::analysis::FloquetOrbitKind::Autonomous,
        FloquetOrbitKindEvidence::LegacyUnknown => {
            return Ok(FloquetStabilityVerdictEvidence::Indeterminate);
        }
    };
    let trivial_multiplier_index = trivial_multiplier_index
        .map(usize::try_from)
        .transpose()
        .map_err(|_| "Floquet phase-mode index does not fit this platform".to_owned())?;
    let verdict = match rspice_core::analysis::classify_floquet_stability(
        &values,
        &evidence,
        orbit_kind,
        trivial_multiplier_index,
        band,
    ) {
        rspice_core::analysis::FloquetStabilityVerdict::Stable => {
            FloquetStabilityVerdictEvidence::Stable
        }
        rspice_core::analysis::FloquetStabilityVerdict::Unstable => {
            FloquetStabilityVerdictEvidence::Unstable
        }
        rspice_core::analysis::FloquetStabilityVerdict::Marginal => {
            FloquetStabilityVerdictEvidence::Marginal
        }
        rspice_core::analysis::FloquetStabilityVerdict::Indeterminate => {
            FloquetStabilityVerdictEvidence::Indeterminate
        }
        // The durable schema must be deliberately revised before it can
        // authenticate a future core semantic state.
        _ => return Err("unsupported core Floquet stability verdict".to_owned()),
    };
    Ok(verdict)
}

fn classify_pstb_modes(
    modes: &[PstbFloquetModeEvidence],
    verdict: FloquetStabilityVerdictEvidence,
    trivial_index: Option<usize>,
) -> Result<PstbStabilityClassificationEvidence, String> {
    match verdict {
        FloquetStabilityVerdictEvidence::Stable => Ok(PstbStabilityClassificationEvidence::Stable),
        FloquetStabilityVerdictEvidence::Indeterminate => {
            Ok(PstbStabilityClassificationEvidence::Indeterminate)
        }
        FloquetStabilityVerdictEvidence::Unstable => {
            let dominant = modes.iter().find(|mode| mode.is_unstable).ok_or_else(|| {
                "PSTB unstable verdict has no mode outside the stability boundary".to_owned()
            })?;
            if dominant.multiplier.imaginary.abs() > 0.01 {
                Ok(PstbStabilityClassificationEvidence::UnstableComplex)
            } else {
                Ok(PstbStabilityClassificationEvidence::UnstableReal)
            }
        }
        FloquetStabilityVerdictEvidence::Marginal => {
            for (index, mode) in modes.iter().enumerate() {
                if trivial_index == Some(index) {
                    continue;
                }
                let value = complex_value(mode.multiplier);
                if (value + num_complex::Complex64::new(1.0, 0.0)).norm() < 0.01 {
                    return Ok(PstbStabilityClassificationEvidence::PeriodDoubling);
                }
                if (value - num_complex::Complex64::new(1.0, 0.0)).norm() < 0.01 {
                    return Ok(PstbStabilityClassificationEvidence::SaddleNode);
                }
                if (value.norm() - 1.0).abs() < 0.01 && value.im.abs() > 0.01 {
                    return Ok(PstbStabilityClassificationEvidence::NeimarkSacker);
                }
            }
            Ok(PstbStabilityClassificationEvidence::Marginal)
        }
    }
}

fn pstb_modes_are_canonically_sorted(modes: &[PstbFloquetModeEvidence]) -> bool {
    modes.windows(2).all(|pair| {
        let left = &pair[0].multiplier;
        let right = &pair[1].multiplier;
        complex_value(*right)
            .norm()
            .total_cmp(&complex_value(*left).norm())
            .then_with(|| left.real.total_cmp(&right.real))
            .then_with(|| left.imaginary.total_cmp(&right.imaginary))
            .is_le()
    })
}

fn detected_subharmonic_order(value: num_complex::Complex64) -> Option<u64> {
    if (value.norm() - 1.0).abs() > 0.01 {
        return None;
    }
    let angle = value.arg().abs();
    (2_u64..=8).find(|order| {
        let expected_angle = 2.0 * std::f64::consts::PI / *order as f64;
        (angle - expected_angle).abs() < 0.01
    })
}

fn complex_value(value: ComplexResultValue) -> num_complex::Complex64 {
    num_complex::Complex64::new(value.real, value.imaginary)
}

fn same_optional_retained_float(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => same_retained_float(left, right),
        (None, None) => true,
        (Some(_), None) | (None, Some(_)) => false,
    }
}

/// An event history is a schedule: nonnegative, finite, and non-decreasing.
///
/// Non-decreasing, not strictly increasing. An event-driven solver settles a
/// node through several delta cycles at one physical time, and every one of
/// those transitions is a committed event with its own value. Their order is
/// the order they were committed in, which is the order they are stored in —
/// so a repeated timestamp is evidence, not corruption.
pub(super) fn validate_event_times(
    node_name: &str,
    times: impl Iterator<Item = f64>,
) -> Result<(), String> {
    let mut previous: Option<f64> = None;
    let mut count = 0usize;
    for time in times {
        count += 1;
        if !time.is_finite() || time < 0.0 {
            return Err(format!(
                "event node '{node_name}' has an invalid event time"
            ));
        }
        if previous.is_some_and(|previous| previous > time) {
            return Err(format!(
                "event node '{node_name}' events must not move backwards in time"
            ));
        }
        previous = Some(time);
    }
    if count == 0 {
        return Err(format!("event node '{node_name}' retained no events"));
    }
    Ok(())
}

pub(super) fn validate_transfer_function_output(
    expression: &str,
    expected_quantity: TransferFunctionQuantityEvidence,
) -> Result<(), String> {
    let trimmed = expression.trim();
    if expression != trimmed {
        return Err("transfer-function output contains surrounding whitespace".to_owned());
    }
    let expression = trimmed;
    let Some(open) = expression.find('(') else {
        return Err(
            "transfer-function output must use V(node), V(node,ref), or I(element)".to_owned(),
        );
    };
    if !expression.ends_with(')') || expression[open + 1..expression.len() - 1].contains(['(', ')'])
    {
        return Err("transfer-function output has unbalanced parentheses".to_owned());
    }
    let function = &expression[..open];
    let arguments = expression[open + 1..expression.len() - 1]
        .split(',')
        .collect::<Vec<_>>();
    if arguments.iter().any(|argument| {
        argument.is_empty()
            || *argument != argument.trim()
            || argument.chars().any(char::is_whitespace)
    }) {
        return Err("transfer-function output contains an invalid identifier".to_owned());
    }
    let quantity = if function.eq_ignore_ascii_case("V") && matches!(arguments.len(), 1 | 2) {
        TransferFunctionQuantityEvidence::Voltage
    } else if function.eq_ignore_ascii_case("I") && arguments.len() == 1 {
        TransferFunctionQuantityEvidence::Current
    } else {
        return Err(
            "transfer-function output must use V(node), V(node,ref), or I(element)".to_owned(),
        );
    };
    if quantity != expected_quantity {
        return Err("transfer-function output quantity contradicts its expression".to_owned());
    }
    Ok(())
}

pub(super) fn soa_rule_verdict(actual: f64, limit: f64) -> SoaRuleVerdictEvidence {
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

pub(super) fn soa_violation_severity(
    actual: f64,
    limit: f64,
) -> Option<SoaViolationSeverityEvidence> {
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

pub(super) fn soa_evaluation_order(
    left: &SoaEvaluationEvidence,
    right: &SoaEvaluationEvidence,
) -> std::cmp::Ordering {
    left.device_id
        .cmp(&right.device_id)
        .then_with(|| left.parameter.cmp(&right.parameter))
}

pub(super) fn soa_violation_order(
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

pub(super) fn validate_complex_values(
    values: &[ComplexResultValue],
    label: &str,
) -> Result<(), String> {
    for (index, value) in values.iter().enumerate() {
        if !value.real.is_finite() || !value.imaginary.is_finite() {
            return Err(format!("{label} {index} has a non-finite component"));
        }
    }
    Ok(())
}

/// Exact physical quantity retained for the primary periodic-noise trace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PeriodicNoiseOutputQuantity {
    /// Output-referred voltage or current noise power spectral density.
    OutputNoisePowerSpectralDensity,
    /// Single-sideband phase noise L(f) in dBc/Hz.
    PhaseNoiseDbcPerHz,
}

pub(super) fn require_non_empty(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{label} is empty"))
    } else {
        Ok(())
    }
}

pub(super) fn require_finite_values(values: &[f64], label: &str) -> Result<(), String> {
    if values.iter().any(|value| !value.is_finite()) {
        Err(format!("{label} contain a non-finite value"))
    } else {
        Ok(())
    }
}

pub(super) fn strictly_increasing(values: &[f64]) -> bool {
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ScalarEvidenceCandidate {
    pub value: Option<f64>,
    pub passed: bool,
}

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
    /// The design objects the engine named for this failure, when it could
    /// name any. `None` covers every successful run and every failure the
    /// engine could not attribute — a parse error names no conductor.
    pub failure_attribution: Option<ConvergenceAttribution>,
    /// Exact prepared-task identity. Missing only for migrated legacy result
    /// history that was written before source instance IDs existed.
    pub provenance: Option<AnalysisResultProvenance>,
}

impl AnalysisResult {
    /// Construct a presentation-only transient result from accepted solver
    /// points. It is deliberately unsuccessful until the engine returns its
    /// terminal result, which keeps measurement, export, and qualification
    /// paths from mistaking an in-flight prefix for complete evidence.
    pub(crate) fn live_transient_partial(
        id: u64,
        analysis_type: AnalysisType,
        label: impl Into<String>,
    ) -> Self {
        Self::failed(id, analysis_type, label, LIVE_TRANSIENT_PARTIAL_MESSAGE)
    }

    #[must_use]
    pub fn is_live_partial(&self) -> bool {
        !self.success && self.error_message.as_deref() == Some(LIVE_TRANSIENT_PARTIAL_MESSAGE)
    }

    /// Exact prepared-task provenance for current retained results.
    #[must_use]
    pub fn provenance(&self) -> Option<&AnalysisResultProvenance> {
        self.provenance.as_ref()
    }

    /// Exact scalar evidence exposed to specification and result-document
    /// consumers. Explicit `.MEAS` results take precedence over a same-named
    /// analysis-native scalar so one execution cannot be counted twice.
    pub(crate) fn scalar_evidence(&self, name: &str) -> Vec<ScalarEvidenceCandidate> {
        let name = name.trim();
        if name.is_empty() {
            return Vec::new();
        }

        let measurements = self
            .measurements
            .iter()
            .filter(|measurement| measurement.name.eq_ignore_ascii_case(name))
            .map(|measurement| ScalarEvidenceCandidate {
                value: measurement.value.filter(|value| value.is_finite()),
                passed: measurement.passed && measurement.error.is_none(),
            })
            .collect::<Vec<_>>();
        if !measurements.is_empty() {
            return measurements;
        }

        self.result_payload
            .as_ref()
            .and_then(|payload| payload.scalar_evidence(name))
            .into_iter()
            .collect()
    }

    /// Canonical discoverable scalar names for the active retained dataset.
    /// These are evidence keys, not synthesized stability booleans.
    pub(crate) fn scalar_evidence_names(&self) -> Vec<String> {
        let mut names = self
            .measurements
            .iter()
            .map(|measurement| measurement.name.clone())
            .collect::<Vec<_>>();
        if let Some(payload) = &self.result_payload {
            names.extend(payload.scalar_evidence_names());
        }
        names
    }

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
            failure_attribution: None,
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
            failure_attribution: None,
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
        // An empty contributor table is meaningful for the `SummaryOnly`
        // retention policy. The integrated totals and exact analysis band are
        // still authoritative result evidence and must survive conversion and
        // project persistence even when individual contributors were omitted.
        self.noise_summary = Some(summary);
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

    /// Attach the exact prepared task that produced this result.
    #[must_use]
    pub fn with_provenance(mut self, provenance: AnalysisResultProvenance) -> Self {
        self.provenance = Some(provenance);
        self
    }

    /// Get current timestamp as Unix epoch seconds
    fn current_timestamp() -> f64 {
        crate::time_compat::unix_epoch().as_secs_f64()
    }

    /// Check if this analysis has any viewable data
    #[cfg(test)]
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
        let mut waveform_names = HashSet::with_capacity(self.waveforms.len());
        for waveform in &self.waveforms {
            let name = waveform.name.trim();
            if name.is_empty() || waveform.name.chars().any(char::is_control) {
                return Err("retained waveform requires a non-empty control-free name".to_owned());
            }
            if !waveform_names.insert(waveform.name.as_str()) {
                return Err(format!(
                    "retained waveform name '{}' is duplicated",
                    waveform.name
                ));
            }
            if waveform.x.len() != waveform.y.len() {
                return Err(format!(
                    "retained waveform '{}' has {} coordinates but {} values",
                    waveform.name,
                    waveform.x.len(),
                    waveform.y.len()
                ));
            }
            if waveform
                .x
                .iter()
                .chain(waveform.y.iter())
                .any(|value| !value.is_finite())
            {
                return Err(format!(
                    "retained waveform '{}' contains a non-finite coordinate or value",
                    waveform.name
                ));
            }
            if waveform
                .unit
                .as_ref()
                .is_some_and(|unit| unit.trim().is_empty() || unit.chars().any(char::is_control))
            {
                return Err(format!(
                    "retained waveform '{}' has an invalid engineering unit",
                    waveform.name
                ));
            }
            if let Some(complex) = &waveform.complex {
                if complex.source_name.trim().is_empty()
                    || complex.source_name.chars().any(char::is_control)
                {
                    return Err(format!(
                        "retained waveform '{}' has an invalid complex-source name",
                        waveform.name
                    ));
                }
                if complex.real.len() != waveform.x.len() || complex.imag.len() != waveform.x.len()
                {
                    return Err(format!(
                        "retained waveform '{}' complex components do not match its {} coordinates",
                        waveform.name,
                        waveform.x.len()
                    ));
                }
                if complex
                    .real
                    .iter()
                    .chain(complex.imag.iter())
                    .any(|value| !value.is_finite())
                {
                    return Err(format!(
                        "retained waveform '{}' contains a non-finite complex component",
                        waveform.name
                    ));
                }
            }
        }

        let valid_text =
            |text: &str| !text.trim().is_empty() && !text.chars().any(char::is_control);
        if let Some(dc_op) = &self.dc_op {
            for (group, values) in [
                ("node voltage", dc_op.node_voltages.as_slice()),
                ("branch current", dc_op.branch_currents.as_slice()),
                ("device power", dc_op.power_dissipation.as_slice()),
            ] {
                let mut names = HashSet::with_capacity(values.len());
                for value in values {
                    if !valid_text(&value.name) || !valid_text(&value.unit) {
                        return Err(format!(
                            "retained {group} requires a valid canonical name and engineering unit"
                        ));
                    }
                    if !names.insert(value.name.as_str()) {
                        return Err(format!("retained {group} '{}' is duplicated", value.name));
                    }
                    if !value.value.is_finite() {
                        return Err(format!(
                            "retained {group} '{}' contains a non-finite value",
                            value.name
                        ));
                    }
                }
            }
        }
        if let Some(report) = &self.device_op {
            if !report.labels_resolve() {
                return Err("retained device operating-point labels do not resolve".to_owned());
            }
            let mut devices = HashSet::with_capacity(report.entries.len());
            for entry in &report.entries {
                if !valid_text(&entry.name) || !devices.insert(entry.name.as_str()) {
                    return Err(format!(
                        "retained device operating-point identity '{}' is invalid or duplicated",
                        entry.name
                    ));
                }
                if entry.params.iter().any(|(_, value)| !value.is_finite()) {
                    return Err(format!(
                        "retained device operating-point entry '{}' contains a non-finite value",
                        entry.name
                    ));
                }
            }
        }
        if let Some(noise) = &self.noise_summary {
            if !noise.band.0.is_finite()
                || !noise.band.1.is_finite()
                || noise.band.0 < 0.0
                || noise.band.1 < noise.band.0
                || noise
                    .total_rms
                    .is_some_and(|value| !value.is_finite() || value < 0.0)
                || noise
                    .input_rms
                    .is_some_and(|value| !value.is_finite() || value < 0.0)
            {
                return Err("retained noise summary has an invalid band or RMS total".to_owned());
            }
            let mut contributors = HashSet::with_capacity(noise.rows.len());
            for row in &noise.rows {
                if !valid_text(&row.device)
                    || !valid_text(&row.mechanism)
                    || !row.power.is_finite()
                    || row.power < 0.0
                    || !row.share_pct.is_finite()
                    || !(0.0..=100.0).contains(&row.share_pct)
                    || !contributors.insert((row.device.as_str(), row.mechanism.as_str()))
                {
                    return Err(format!(
                        "retained noise contribution '{} / {}' is invalid or duplicated",
                        row.device, row.mechanism
                    ));
                }
            }
        }
        let mut measurement_names = HashSet::with_capacity(self.measurements.len());
        for measurement in &self.measurements {
            if !valid_text(&measurement.name)
                || !measurement_names.insert(measurement.name.as_str())
            {
                return Err(format!(
                    "retained measurement identity '{}' is invalid or duplicated",
                    measurement.name
                ));
            }
            if [
                measurement.value,
                measurement.raw_value,
                measurement.expected,
                measurement.tolerance,
                measurement.failure_limit,
                measurement.event_axis,
            ]
            .into_iter()
            .flatten()
            .any(|value| !value.is_finite())
                || measurement.tolerance.is_some_and(|value| value < 0.0)
                || (measurement.passed
                    && (measurement.value.is_none() || measurement.error.is_some()))
            {
                return Err(format!(
                    "retained measurement '{}' has contradictory or non-finite evidence",
                    measurement.name
                ));
            }
            if measurement.value.is_some() != measurement.raw_value.is_some() {
                return Err(format!(
                    "retained measurement '{}' must carry its raw value exactly when it carries a published value",
                    measurement.name
                ));
            }
            let expected_exceeded = match (measurement.raw_value, measurement.failure_limit) {
                (Some(raw_value), Some(limit)) => raw_value.abs() >= limit,
                _ => false,
            };
            if measurement.failure_limit_exceeded != expected_exceeded {
                return Err(format!(
                    "retained measurement '{}' FAILVALUE verdict does not match abs(raw_value) >= failure_limit",
                    measurement.name
                ));
            }
            if measurement.failure_limit_exceeded && measurement.passed {
                return Err(format!(
                    "retained measurement '{}' cannot pass after its FAILVALUE limit was reached",
                    measurement.name
                ));
            }
        }

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
mod retained_payload_tests;
