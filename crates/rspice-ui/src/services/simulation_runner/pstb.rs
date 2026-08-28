//! Periodic stability analysis.
//!
//! Loop gain and phase margin about a periodic steady state, for feedback
//! that is only meaningful over a cycle — switched-mode regulators and
//! sampled loops.

use super::{
    ServiceRunError, ServiceRunResult, build_engine_config,
    error::{ensure_not_aborted, poll_periodically},
    parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::Engine;
use std::fmt;
use std::path::Path;

#[derive(Debug)]
enum PstbRunError {
    InvalidConfig(&'static str),
    CircuitBuild(String),
    Pss(String),
    ProbeNotFound {
        probe: String,
        available: String,
    },
    ProbeNotInductor {
        probe: String,
        branch_ordinal: usize,
        available: String,
    },
    NonSquareMonodromy,
    ProbeStateOutOfRange {
        probe: String,
        state_index: usize,
        monodromy_dim: usize,
    },
    InvalidResult(&'static str),
    InvalidModeData {
        mode: usize,
        reason: &'static str,
    },
}

impl fmt::Display for PstbRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfig(message) => f.write_str(message),
            Self::CircuitBuild(err) => write!(f, "PSTB prerequisite circuit-build error: {err}"),
            Self::Pss(err) => write!(f, "PSTB prerequisite PSS error: {err}"),
            Self::ProbeNotFound { probe, available } => write!(
                f,
                "PSTB probe '{}' was not found in branch-capable elements. Available branches: {}",
                probe, available
            ),
            Self::ProbeNotInductor {
                probe,
                branch_ordinal,
                available,
            } => write!(
                f,
                "PSTB probe '{}' resolved to branch ordinal {} but is not an inductor probe. \
PSTB currently supports dynamic inductor-current probes only. Available inductor probes: {}",
                probe, branch_ordinal, available
            ),
            Self::NonSquareMonodromy => {
                f.write_str("PSTB prerequisite PSS returned a non-square monodromy matrix")
            }
            Self::ProbeStateOutOfRange {
                probe,
                state_index,
                monodromy_dim,
            } => write!(
                f,
                "PSTB probe '{}' maps to reactive state {} but monodromy dimension is {}",
                probe, state_index, monodromy_dim
            ),
            Self::InvalidResult(reason) => write!(f, "PSTB returned an invalid result: {reason}"),
            Self::InvalidModeData { mode, reason } => {
                write!(f, "PSTB mode {mode} returned invalid data: {reason}")
            }
        }
    }
}

impl From<PstbRunError> for ServiceRunError {
    fn from(error: PstbRunError) -> Self {
        Self::Failure(error.to_string())
    }
}

/// Explicit configuration for PSTB execution.
#[derive(Debug, Clone)]
pub struct PstbRunConfig {
    pub pss_fundamental_freq: Value,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: Value,
    pub probe_instance: String,
    pub max_harmonics: usize,
    pub num_multipliers: usize,
    pub stability_threshold: Value,
    pub detect_subharmonics: bool,
    pub eigenvalue_tolerance: Value,
}

impl Default for PstbRunConfig {
    fn default() -> Self {
        Self {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 10,
            pss_tolerance: 1e-3,
            probe_instance: "LPROBE".to_string(),
            max_harmonics: 10,
            num_multipliers: 10,
            stability_threshold: 1.0 + 1e-6,
            detect_subharmonics: true,
            eigenvalue_tolerance: 1e-10,
        }
    }
}

impl PstbRunConfig {
    fn validate(&self) -> Result<(), PstbRunError> {
        if !self.pss_fundamental_freq.is_finite() || self.pss_fundamental_freq <= 0.0 {
            return Err(PstbRunError::InvalidConfig(
                "PSTB requires a positive PSS fundamental frequency",
            ));
        }
        if self.pss_num_harmonics == 0 {
            return Err(PstbRunError::InvalidConfig(
                "PSTB requires at least one PSS harmonic",
            ));
        }
        if !self.pss_tolerance.is_finite() || self.pss_tolerance <= 0.0 {
            return Err(PstbRunError::InvalidConfig(
                "PSTB requires a positive PSS tolerance",
            ));
        }
        if self.probe_instance.trim().is_empty() {
            return Err(PstbRunError::InvalidConfig(
                "PSTB probe instance must be specified",
            ));
        }
        if self.max_harmonics == 0 {
            return Err(PstbRunError::InvalidConfig(
                "PSTB max harmonics must be greater than zero",
            ));
        }
        if self.num_multipliers == 0 {
            return Err(PstbRunError::InvalidConfig(
                "PSTB number of multipliers must be greater than zero",
            ));
        }
        if !self.stability_threshold.is_finite() || self.stability_threshold < 1.0 {
            return Err(PstbRunError::InvalidConfig(
                "PSTB stability threshold must be at least one",
            ));
        }
        if !self.eigenvalue_tolerance.is_finite() || self.eigenvalue_tolerance <= 0.0 {
            return Err(PstbRunError::InvalidConfig(
                "PSTB eigenvalue tolerance must be positive",
            ));
        }
        Ok(())
    }
}

/// PSTB analysis data.
#[derive(Debug, Clone)]
pub struct PstbData {
    /// Period of the analyzed periodic orbit.
    pub period: Value,
    /// Fundamental frequency of the analyzed periodic orbit.
    pub fundamental_frequency: Value,
    /// Complete authenticated Floquet spectrum, sorted by magnitude.
    pub modes: Vec<PstbModeData>,
    /// Completeness and residual qualification for `modes`.
    pub floquet_evidence: rspice_core::analysis::FloquetSpectrumEvidence,
    /// Driven/autonomous policy copied from the prerequisite PSS result.
    pub orbit_kind: rspice_core::analysis::FloquetOrbitKind,
    /// Exact outer stability threshold used for classification.
    pub stability_threshold: Value,
    /// Canonical circuit identity of the configured probe.
    pub probe_instance: String,
    /// Whether subharmonic classification was enabled.
    pub detect_subharmonics: bool,
    /// Explicitly selected autonomous phase-mode index.
    pub trivial_multiplier_index: Option<usize>,
    /// Shared four-state stability verdict.
    pub stability_verdict: rspice_core::analysis::FloquetStabilityVerdict,
    /// Rich PSTB classification refining the shared verdict.
    pub stability_classification: rspice_core::analysis::pstb::StabilityType,
    /// Finite signed global margin when an applicable mode exists.
    pub min_stability_margin_db: Option<Value>,
    /// Maximum multiplier magnitude over the complete spectrum.
    pub max_multiplier_magnitude: Value,
    /// Number of non-trivial modes outside the configured outer boundary.
    pub num_unstable: usize,
    /// Detected subharmonic orders over the complete spectrum.
    pub subharmonics: Vec<usize>,
    /// Whether the atomic qualified eigensolve completed.
    pub converged: bool,
    /// Iteration count reported by the eigensolver.
    pub iterations: usize,
    /// Mode indices (1-based) for plotting.
    pub mode_indices: Vec<Value>,
    /// Probe-local mode participation (normalized |v_i| contribution per mode).
    pub probe_mode_participation: Vec<Value>,
    /// Floquet multiplier magnitudes.
    pub multiplier_magnitude: Vec<Value>,
    /// Floquet multiplier phases in degrees.
    pub multiplier_phase_deg: Vec<Value>,
    /// Mode damping factors in 1/s.
    pub mode_damping: Vec<Value>,
    /// Natural mode frequencies in hertz.
    pub mode_frequency_hz: Vec<Value>,
    /// Per-mode stability margin in dB.
    pub stability_margin_db: Vec<Value>,
}

/// One complete retained mode. Display limits never truncate this vector.
#[derive(Debug, Clone, PartialEq)]
pub struct PstbModeData {
    pub multiplier: (Value, Value),
    pub exponent: (Value, Value),
    pub probe_participation: Value,
    pub is_unstable: bool,
    pub is_trivial: bool,
    pub subharmonic_order: Option<usize>,
}

#[derive(Debug, Clone)]
struct ResolvedPstbProbe {
    canonical_name: String,
    state_index: usize,
}

fn normalize_branch_name_list(mut names: Vec<String>) -> Vec<String> {
    names.sort_by_cached_key(|name| name.to_ascii_uppercase());
    names.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    names
}

fn format_branch_name_list(names: &[String]) -> String {
    if names.is_empty() {
        return "<none>".to_string();
    }
    const DISPLAY_LIMIT: usize = 12;
    if names.len() <= DISPLAY_LIMIT {
        return names.join(", ");
    }
    let shown = names[..DISPLAY_LIMIT].join(", ");
    format!("{shown}, ... (+{} more)", names.len() - DISPLAY_LIMIT)
}

fn available_branch_names(circuit: &rspice_core::circuit::CircuitData) -> Vec<String> {
    normalize_branch_name_list(circuit.branch_probe_names())
}

fn available_inductor_probe_names(circuit: &rspice_core::circuit::CircuitData) -> Vec<String> {
    normalize_branch_name_list(circuit.inductor_probe_names())
}

fn resolve_pstb_probe_with_abort(
    circuit: &rspice_core::circuit::CircuitData,
    probe_instance: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<ResolvedPstbProbe> {
    ensure_not_aborted(abort)?;
    let probe_name = probe_instance.trim();
    let branch_ordinal = circuit.get_branch_by_name(probe_name).ok_or_else(|| {
        let available = format_branch_name_list(&available_branch_names(circuit));
        ServiceRunError::from(PstbRunError::ProbeNotFound {
            probe: probe_name.to_string(),
            available,
        })
    })?;
    ensure_not_aborted(abort)?;

    let probe = circuit
        .inductor_probe_for_branch(branch_ordinal)
        .ok_or_else(|| {
            let available = format_branch_name_list(&available_inductor_probe_names(circuit));
            ServiceRunError::from(PstbRunError::ProbeNotInductor {
                probe: probe_name.to_string(),
                branch_ordinal,
                available,
            })
        })?;
    ensure_not_aborted(abort)?;

    Ok(ResolvedPstbProbe {
        canonical_name: probe.canonical_name,
        state_index: probe.state_index,
    })
}

fn normalized_probe_participation_with_abort(
    eigenvector: Option<&[num_complex::Complex64]>,
    state_index: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Value> {
    ensure_not_aborted(abort)?;
    let vector = eigenvector.ok_or_else(|| {
        ServiceRunError::Failure("PSTB solver did not return a requested eigenvector".to_owned())
    })?;
    let component = vector.get(state_index).ok_or_else(|| {
        ServiceRunError::Failure(
            "PSTB eigenvector does not contain the configured probe state".to_owned(),
        )
    })?;
    let mut denom = 0.0_f64;
    for (index, value) in vector.iter().enumerate() {
        poll_periodically(abort, index)?;
        if !value.re.is_finite() || !value.im.is_finite() {
            return Err(ServiceRunError::Failure(
                "PSTB solver returned a non-finite eigenvector".to_owned(),
            ));
        }
        denom = denom.hypot(value.norm());
    }
    if !denom.is_finite() || denom == 0.0 {
        return Err(ServiceRunError::Failure(
            "PSTB solver returned a zero-norm eigenvector".to_owned(),
        ));
    }
    let ratio = component.norm() / denom;
    if ratio.is_finite() {
        Ok(ratio.clamp(0.0, 1.0))
    } else {
        Err(ServiceRunError::Failure(
            "PSTB probe participation is non-finite".to_owned(),
        ))
    }
}

fn pstb_classification_matches_verdict(
    verdict: rspice_core::analysis::FloquetStabilityVerdict,
    classification: rspice_core::analysis::pstb::StabilityType,
) -> bool {
    use rspice_core::analysis::FloquetStabilityVerdict as Verdict;
    use rspice_core::analysis::pstb::StabilityType as Classification;

    match verdict {
        Verdict::Stable => classification == Classification::Stable,
        Verdict::Unstable => matches!(
            classification,
            Classification::UnstableReal | Classification::UnstableComplex
        ),
        Verdict::Marginal => matches!(
            classification,
            Classification::PeriodDoubling
                | Classification::NeimarkSacker
                | Classification::SaddleNode
                | Classification::Marginal
        ),
        Verdict::Indeterminate => classification == Classification::Indeterminate,
        _ => false,
    }
}

fn pstb_modes_are_sorted(modes: &[rspice_core::analysis::pstb::FloquetMultiplier]) -> bool {
    modes.windows(2).all(|pair| {
        let left = &pair[0];
        let right = &pair[1];
        right
            .magnitude()
            .total_cmp(&left.magnitude())
            .then_with(|| left.value.re.total_cmp(&right.value.re))
            .then_with(|| left.value.im.total_cmp(&right.value.im))
            .is_le()
    })
}

fn build_pstb_data_from_core_result(
    result: rspice_core::analysis::pstb::PstbResult,
    expected_orbit_kind: rspice_core::analysis::FloquetOrbitKind,
    probe_instance: &str,
    probe_state_index: usize,
    max_display_modes: usize,
    stability_threshold: Value,
    detect_subharmonics: bool,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PstbData> {
    use rspice_core::analysis::{
        FloquetOrbitKind, FloquetSpectrumEvidence, classify_floquet_stability,
        select_autonomous_phase_mode,
    };

    ensure_not_aborted(abort)?;
    if !result.period.is_finite()
        || result.period <= 0.0
        || !result.fundamental_frequency.is_finite()
        || result.fundamental_frequency <= 0.0
        || result.fundamental_frequency != 1.0 / result.period
        || !stability_threshold.is_finite()
        || stability_threshold < 1.0
        || !result.converged
        || probe_instance.trim().is_empty()
    {
        return Err(PstbRunError::InvalidResult(
            "period, frequency, convergence, stability boundary, or probe identity is invalid",
        )
        .into());
    }
    if result.orbit_kind != expected_orbit_kind {
        return Err(PstbRunError::InvalidResult(
            "orbit policy does not match the prerequisite PSS result",
        )
        .into());
    }

    let multiplier_values = result
        .multipliers
        .iter()
        .map(|multiplier| multiplier.value)
        .collect::<Vec<_>>();
    let current_evidence = matches!(
        &result.floquet_evidence,
        FloquetSpectrumEvidence::NoDynamicModes | FloquetSpectrumEvidence::Qualified { .. }
    );
    if !current_evidence
        || !result
            .floquet_evidence
            .is_consistent_with(&multiplier_values)
        || (matches!(
            &result.floquet_evidence,
            FloquetSpectrumEvidence::NoDynamicModes
        ) && result.orbit_kind != FloquetOrbitKind::Driven)
    {
        return Err(PstbRunError::InvalidResult(
            "Floquet evidence is absent, non-current, or inconsistent with the spectrum and orbit policy",
        )
        .into());
    }

    let expected_trivial_index = if result.orbit_kind == FloquetOrbitKind::Autonomous
        && matches!(
            &result.floquet_evidence,
            FloquetSpectrumEvidence::Qualified { .. }
        ) {
        select_autonomous_phase_mode(&multiplier_values)
    } else {
        None
    };
    if result.trivial_multiplier_index != expected_trivial_index {
        return Err(PstbRunError::InvalidResult(
            "autonomous phase-mode selection is inconsistent with the spectrum",
        )
        .into());
    }

    let expected_verdict = classify_floquet_stability(
        &multiplier_values,
        &result.floquet_evidence,
        result.orbit_kind,
        result.trivial_multiplier_index,
        stability_threshold - 1.0,
    );
    if result.stability_verdict != expected_verdict
        || !pstb_classification_matches_verdict(result.stability_verdict, result.stability)
    {
        return Err(PstbRunError::InvalidResult(
            "stability verdict or rich classification is inconsistent with the spectrum",
        )
        .into());
    }
    if !pstb_modes_are_sorted(&result.multipliers) {
        return Err(
            PstbRunError::InvalidResult("Floquet modes are not in canonical sorted order").into(),
        );
    }

    let order = result.multipliers.len();
    if result.monodromy.len() != order
        || result
            .monodromy
            .iter()
            .any(|row| row.len() != order || row.iter().any(|value| !value.is_finite()))
    {
        return Err(PstbRunError::InvalidResult(
            "monodromy dimensions or values do not match the authenticated spectrum",
        )
        .into());
    }

    let expected_num_unstable = result
        .multipliers
        .iter()
        .filter(|multiplier| multiplier.is_unstable)
        .count();
    let expected_max_magnitude = result.multipliers.first().map_or(
        0.0,
        rspice_core::analysis::pstb::FloquetMultiplier::magnitude,
    );
    let expected_min_margin = result
        .multipliers
        .iter()
        .enumerate()
        .filter(|(index, _)| result.trivial_multiplier_index != Some(*index))
        .map(|(_, multiplier)| multiplier.stability_margin_db())
        .min_by(f64::total_cmp);
    let expected_subharmonics = if detect_subharmonics {
        result
            .multipliers
            .iter()
            .filter_map(|multiplier| multiplier.subharmonic_order)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    if result.num_unstable != expected_num_unstable
        || !result.max_multiplier_magnitude.is_finite()
        || result.max_multiplier_magnitude != expected_max_magnitude
        || result
            .min_stability_margin_db
            .is_some_and(|margin| !margin.is_finite())
        || result.min_stability_margin_db != expected_min_margin
        || result.subharmonics != expected_subharmonics
    {
        return Err(PstbRunError::InvalidResult(
            "aggregate counts, margins, or subharmonics do not match the complete spectrum",
        )
        .into());
    }

    let mut modes = Vec::with_capacity(order);
    let mut all_participation = Vec::with_capacity(order);
    for (index, multiplier) in result.multipliers.iter().enumerate() {
        poll_periodically(abort, index)?;
        let magnitude = multiplier.magnitude();
        let expected_unstable =
            result.trivial_multiplier_index != Some(index) && magnitude > stability_threshold;
        if multiplier.index != index
            || !multiplier.value.re.is_finite()
            || !multiplier.value.im.is_finite()
            || !multiplier.exponent.re.is_finite()
            || !multiplier.exponent.im.is_finite()
            || !magnitude.is_finite()
            || magnitude <= 0.0
            || multiplier.is_trivial != (result.trivial_multiplier_index == Some(index))
            || multiplier.is_unstable != expected_unstable
            || multiplier
                .eigenvector
                .as_ref()
                .is_none_or(|vector| vector.len() != order)
        {
            return Err(PstbRunError::InvalidModeData {
                mode: index + 1,
                reason: "identity, finite values, flags, or eigenvector cardinality is invalid",
            }
            .into());
        }
        let participation = normalized_probe_participation_with_abort(
            multiplier.eigenvector.as_deref(),
            probe_state_index,
            abort,
        )?;
        all_participation.push(participation);
        modes.push(PstbModeData {
            multiplier: (multiplier.value.re, multiplier.value.im),
            exponent: (multiplier.exponent.re, multiplier.exponent.im),
            probe_participation: participation,
            is_unstable: multiplier.is_unstable,
            is_trivial: multiplier.is_trivial,
            subharmonic_order: detect_subharmonics
                .then_some(multiplier.subharmonic_order)
                .flatten(),
        });
    }

    let display_count = order.min(max_display_modes);
    let mut mode_indices = Vec::with_capacity(display_count);
    let mut probe_mode_participation = Vec::with_capacity(display_count);
    let mut multiplier_magnitude = Vec::with_capacity(display_count);
    let mut multiplier_phase_deg = Vec::with_capacity(display_count);
    let mut mode_damping = Vec::with_capacity(display_count);
    let mut mode_frequency_hz = Vec::with_capacity(display_count);
    let mut stability_margin_db = Vec::with_capacity(display_count);
    for (index, multiplier) in result.multipliers.iter().take(display_count).enumerate() {
        poll_periodically(abort, index)?;
        let phase_degrees = multiplier.phase_degrees();
        let damping = multiplier.damping();
        let natural_frequency = multiplier.natural_frequency();
        let stability_margin = multiplier.stability_margin_db();
        if !phase_degrees.is_finite()
            || !damping.is_finite()
            || !natural_frequency.is_finite()
            || natural_frequency < 0.0
            || !stability_margin.is_finite()
        {
            return Err(PstbRunError::InvalidModeData {
                mode: index + 1,
                reason: "derived display values are non-finite or outside their domain",
            }
            .into());
        }
        mode_indices.push((index + 1) as Value);
        probe_mode_participation.push(all_participation[index]);
        multiplier_magnitude.push(multiplier.magnitude());
        multiplier_phase_deg.push(phase_degrees);
        mode_damping.push(damping);
        mode_frequency_hz.push(natural_frequency);
        stability_margin_db.push(stability_margin);
    }
    ensure_not_aborted(abort)?;

    Ok(PstbData {
        period: result.period,
        fundamental_frequency: result.fundamental_frequency,
        modes,
        floquet_evidence: result.floquet_evidence,
        orbit_kind: result.orbit_kind,
        stability_threshold,
        probe_instance: probe_instance.to_owned(),
        detect_subharmonics,
        trivial_multiplier_index: result.trivial_multiplier_index,
        stability_verdict: result.stability_verdict,
        stability_classification: result.stability,
        min_stability_margin_db: result.min_stability_margin_db,
        max_multiplier_magnitude: result.max_multiplier_magnitude,
        num_unstable: result.num_unstable,
        subharmonics: result.subharmonics,
        converged: result.converged,
        iterations: result.iterations,
        mode_indices,
        probe_mode_participation,
        multiplier_magnitude,
        multiplier_phase_deg,
        mode_damping,
        mode_frequency_hz,
        stability_margin_db,
    })
}

/// Run PSTB standalone -- computing its own PSS operating point rather than
/// receiving one -- with cooperative cancellation.
///
/// Test-only. PSTB ships as a dependent task: the frequency spec runs PSS
/// first and hands the authenticated monodromy matrix to
/// [`run_pstb_analysis_from_pss_with_source_path_and_abort`], so nothing in the
/// product takes this path.
#[cfg(test)]
pub fn run_pstb_analysis_with_config_and_abort(
    netlist_text: &str,
    config: &PstbRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PstbData> {
    run_pstb_analysis_impl(netlist_text, config, None, None, abort)
}

/// Run PSTB from an exact retained PSS state with direct-call source-relative
/// include and model resolution.
pub fn run_pstb_analysis_from_pss_with_source_path_and_abort(
    netlist_text: &str,
    config: &PstbRunConfig,
    operating_point: &rspice_core::engine::PssOperatingPoint,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PstbData> {
    run_pstb_analysis_impl(
        netlist_text,
        config,
        source_path,
        Some(operating_point),
        abort,
    )
}

fn run_pstb_analysis_impl(
    netlist_text: &str,
    config: &PstbRunConfig,
    source_path: Option<&Path>,
    operating_point: Option<&rspice_core::engine::PssOperatingPoint>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PstbData> {
    use rspice_core::analysis::PssConfig;
    use rspice_core::analysis::pstb::{PstbAnalyzer, PstbConfig};

    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::from)?;

    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;

    let mut sim_config = build_engine_config(&netlist, None);
    sim_config.tolerance = config.pss_tolerance;
    let engine = Engine::new(sim_config);
    ensure_not_aborted(abort)?;
    let circuit = engine
        .build_circuit(&netlist)
        .map_err(|error| PstbRunError::CircuitBuild(error.to_string()))
        .map_err(ServiceRunError::from)?;
    ensure_not_aborted(abort)?;
    let probe = resolve_pstb_probe_with_abort(&circuit, &config.probe_instance, abort)?;

    let owned_pss;
    let pss_result = if let Some(operating_point) = operating_point {
        operating_point.analysis()
    } else {
        let pss_harmonics = config.pss_num_harmonics.max(config.max_harmonics);
        let pss_config = PssConfig::new(config.pss_fundamental_freq)
            .with_harmonics(pss_harmonics)
            .with_tolerance(config.pss_tolerance)
            .with_max_iterations(50)
            .with_tstab_periods(10);
        owned_pss = engine
            .run_pss_with_abort(&netlist, pss_config, abort)
            .map_err(|error| match error {
                rspice_core::SimulationError::Aborted => ServiceRunError::Aborted,
                other => ServiceRunError::from(PstbRunError::Pss(other.to_string())),
            })?;
        &owned_pss
    };
    ensure_not_aborted(abort)?;

    let monodromy_dim = pss_result.monodromy.len();
    for (index, row) in pss_result.monodromy.iter().enumerate() {
        poll_periodically(abort, index)?;
        if row.len() != monodromy_dim {
            return Err(PstbRunError::NonSquareMonodromy.into());
        }
    }
    if monodromy_dim > 0 && probe.state_index >= monodromy_dim {
        return Err(PstbRunError::ProbeStateOutOfRange {
            probe: probe.canonical_name.clone(),
            state_index: probe.state_index,
            monodromy_dim,
        }
        .into());
    }

    let pss_orbit_kind = pss_result.result.floquet_orbit_kind;
    if !pss_result.result.has_consistent_floquet_contract()
        || pss_result.result.period_detected
            != (pss_orbit_kind == rspice_core::analysis::FloquetOrbitKind::Autonomous)
    {
        return Err(PstbRunError::InvalidResult(
            "prerequisite PSS Floquet orbit contract is inconsistent",
        )
        .into());
    }

    let pstb_config = PstbConfig::new()
        .with_num_eigenvalues(config.num_multipliers)
        .with_orbit_kind(pss_orbit_kind)
        .with_eigenvectors(true)
        .with_tolerance(config.eigenvalue_tolerance)
        .with_stability_threshold(config.stability_threshold)
        .with_subharmonic_detection(config.detect_subharmonics);
    let mut analyzer = PstbAnalyzer::new(pstb_config);
    let pstb_result = analyzer
        .analyze_monodromy_with_abort(&pss_result.monodromy, pss_result.period, abort)
        .map_err(ServiceRunError::from)?;
    build_pstb_data_from_core_result(
        pstb_result,
        pss_orbit_kind,
        &probe.canonical_name,
        probe.state_index,
        config.num_multipliers,
        config.stability_threshold,
        config.detect_subharmonics,
        abort,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::{ImmediateAbort, NoAbort};
    use rspice_core::analysis::pstb::{PstbAnalyzer, PstbConfig};
    use rspice_core::analysis::{
        FloquetOrbitKind, FloquetSpectrumEvidence, FloquetStabilityVerdict,
    };

    const STABILITY_THRESHOLD: f64 = 1.0 + 1.0e-6;

    fn analyze(
        monodromy: &[Vec<f64>],
        orbit_kind: FloquetOrbitKind,
    ) -> rspice_core::analysis::pstb::PstbResult {
        PstbAnalyzer::new(
            PstbConfig::new()
                .with_orbit_kind(orbit_kind)
                .with_eigenvectors(true)
                .with_stability_threshold(STABILITY_THRESHOLD),
        )
        .analyze_monodromy_with_abort(monodromy, 1.0, &NoAbort)
        .unwrap()
    }

    #[test]
    fn pstb_service_preserves_typed_entry_abort() {
        let mut config = PstbRunConfig::default();
        config.pss_fundamental_freq = 0.0;

        let result =
            run_pstb_analysis_with_config_and_abort("not a netlist", &config, &ImmediateAbort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn driven_zero_order_spectrum_is_authenticated_and_representable() {
        let result = analyze(&[], FloquetOrbitKind::Driven);
        let data = build_pstb_data_from_core_result(
            result,
            FloquetOrbitKind::Driven,
            "LPROBE",
            usize::MAX,
            1,
            STABILITY_THRESHOLD,
            true,
            &NoAbort,
        )
        .unwrap();

        assert!(data.modes.is_empty());
        assert!(data.mode_indices.is_empty());
        assert!(data.probe_mode_participation.is_empty());
        assert!(matches!(
            data.floquet_evidence,
            FloquetSpectrumEvidence::NoDynamicModes
        ));
        assert_eq!(data.stability_verdict, FloquetStabilityVerdict::Stable);
        assert_eq!(data.min_stability_margin_db, None);
        assert_eq!(data.num_unstable, 0);
    }

    #[test]
    fn autonomous_zero_order_spectrum_fails_closed() {
        let result = analyze(&[], FloquetOrbitKind::Autonomous);
        let error = build_pstb_data_from_core_result(
            result,
            FloquetOrbitKind::Autonomous,
            "LPROBE",
            usize::MAX,
            1,
            STABILITY_THRESHOLD,
            true,
            &NoAbort,
        )
        .unwrap_err();

        assert!(error.to_string().contains("evidence"));
        assert!(error.to_string().contains("orbit policy"));
    }

    #[test]
    fn presentation_limit_does_not_truncate_authenticated_spectrum() {
        let result = analyze(
            &[
                vec![0.5, 0.0, 0.0],
                vec![0.0, 0.4, 0.0],
                vec![0.0, 0.0, 0.3],
            ],
            FloquetOrbitKind::Driven,
        );
        let data = build_pstb_data_from_core_result(
            result,
            FloquetOrbitKind::Driven,
            "LPROBE",
            0,
            1,
            STABILITY_THRESHOLD,
            true,
            &NoAbort,
        )
        .unwrap();

        assert_eq!(data.modes.len(), 3);
        assert_eq!(data.mode_indices, vec![1.0]);
        assert_eq!(data.multiplier_magnitude, vec![0.5]);
        assert_eq!(data.stability_threshold, STABILITY_THRESHOLD);
        assert_eq!(data.probe_instance, "LPROBE");
        assert!(data.detect_subharmonics);
        let FloquetSpectrumEvidence::Qualified { certificate } = data.floquet_evidence else {
            panic!("expected a qualified complete spectrum");
        };
        assert_eq!(certificate.problem_order, 3);
        assert_eq!(data.stability_verdict, FloquetStabilityVerdict::Stable);
    }

    #[test]
    fn malformed_current_aggregate_fails_closed() {
        let mut result = analyze(&[vec![0.5]], FloquetOrbitKind::Driven);
        result.num_unstable = 1;

        let error = build_pstb_data_from_core_result(
            result,
            FloquetOrbitKind::Driven,
            "LPROBE",
            0,
            1,
            STABILITY_THRESHOLD,
            true,
            &NoAbort,
        )
        .unwrap_err();

        assert!(error.to_string().contains("aggregate counts"));
    }

    #[test]
    fn blank_probe_provenance_fails_closed() {
        let result = analyze(&[vec![0.5]], FloquetOrbitKind::Driven);
        let error = build_pstb_data_from_core_result(
            result,
            FloquetOrbitKind::Driven,
            " ",
            0,
            1,
            STABILITY_THRESHOLD,
            true,
            &NoAbort,
        )
        .unwrap_err();

        assert!(error.to_string().contains("probe identity"));
    }
}
