use super::{
    ServiceRunError, ServiceRunResult, build_engine_config,
    error::{ensure_not_aborted, poll_periodically},
    parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
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
    EmptyMonodromy,
    NonSquareMonodromy,
    ProbeStateOutOfRange {
        probe: String,
        state_index: usize,
        monodromy_dim: usize,
    },
    NoFloquetMultipliers,
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
            Self::EmptyMonodromy => {
                f.write_str("PSTB prerequisite PSS returned an empty monodromy matrix")
            }
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
            Self::NoFloquetMultipliers => f.write_str("PSTB produced no Floquet multipliers"),
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
        if !self.stability_threshold.is_finite() || self.stability_threshold <= 0.0 {
            return Err(PstbRunError::InvalidConfig(
                "PSTB stability threshold must be positive",
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
    /// Fundamental period in seconds.
    pub period: Value,
    /// Fundamental frequency in Hz.
    pub fundamental_frequency: Value,
    /// Probe instance used for metadata correlation.
    pub probe_instance: String,
    /// Probe branch ordinal in MNA ordering (1-indexed branch numbering).
    pub probe_branch_ordinal: usize,
    /// Reactive state index in the PSS/PSTB monodromy matrix (0-indexed).
    pub probe_state_index: usize,
    /// Probe-state self-transition term M[i,i] over one period.
    pub probe_state_self_transition: Value,
    /// Euclidean norm of the probe-state monodromy column ||M[:,i]||2.
    pub probe_state_column_norm: Value,
    /// Euclidean norm of the probe-state monodromy row ||M[i,:]||2.
    pub probe_state_row_norm: Value,
    /// Probe-state persistence in dB, computed from |M[i,i]|.
    pub probe_state_persistence_db: Value,
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
    /// Mode natural frequencies in Hz.
    pub mode_frequency_hz: Vec<Value>,
    /// Per-mode stability margin in dB.
    pub stability_margin_db: Vec<Value>,
    /// Dominant multiplier magnitude.
    pub dominant_multiplier_magnitude: Value,
    /// Global minimum stability margin in dB.
    pub min_stability_margin_db: Value,
    /// Mode index (1-based) with largest probe participation.
    pub dominant_probe_mode: usize,
    /// Largest probe participation value across retained modes.
    pub dominant_probe_mode_participation: Value,
    /// Number of unstable modes.
    pub num_unstable: usize,
    /// Stability classification string.
    pub stability_classification: String,
    /// Whether the periodic orbit is stable.
    pub is_stable: bool,
}

fn stability_type_label(
    stability: rspice_core::analysis::advanced::pstb::StabilityType,
) -> &'static str {
    use rspice_core::analysis::advanced::pstb::StabilityType;
    match stability {
        StabilityType::Stable => "Stable",
        StabilityType::UnstableReal => "UnstableReal",
        StabilityType::UnstableComplex => "UnstableComplex",
        StabilityType::PeriodDoubling => "PeriodDoubling",
        StabilityType::NeimarkSacker => "NeimarkSacker",
        StabilityType::SaddleNode => "SaddleNode",
        StabilityType::Marginal => "Marginal",
    }
}

fn sanitize_db(value: Value) -> Value {
    if value.is_finite() {
        value
    } else if value.is_sign_positive() {
        300.0
    } else {
        -300.0
    }
}

#[derive(Debug, Clone)]
struct ResolvedPstbProbe {
    canonical_name: String,
    branch_ordinal: usize,
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
        branch_ordinal: probe.branch_ordinal,
        state_index: probe.state_index,
    })
}

fn finite_l2_norm_with_abort(
    values: impl Iterator<Item = Value>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Value> {
    let mut sum_sq = 0.0;
    for (index, value) in values.enumerate() {
        poll_periodically(abort, index)?;
        if !value.is_finite() {
            return Ok(f64::INFINITY);
        }
        sum_sq += value * value;
    }
    ensure_not_aborted(abort)?;
    Ok(sum_sq.sqrt())
}

fn sanitize_nonnegative(value: Value) -> Value {
    if value.is_finite() && value >= 0.0 {
        value
    } else {
        0.0
    }
}

fn sanitize_finite(value: Value) -> Value {
    if value.is_finite() { value } else { 0.0 }
}

fn normalized_probe_participation_with_abort(
    eigenvector: Option<&[num_complex::Complex64]>,
    state_index: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Value> {
    ensure_not_aborted(abort)?;
    let Some(vector) = eigenvector else {
        return Ok(0.0);
    };
    let Some(component) = vector.get(state_index) else {
        return Ok(0.0);
    };
    let mut denom_squared = 0.0;
    for (index, value) in vector.iter().enumerate() {
        poll_periodically(abort, index)?;
        denom_squared += value.norm_sqr();
    }
    let denom = denom_squared.sqrt();
    if !denom.is_finite() || denom <= 1e-30 {
        return Ok(0.0);
    }
    let ratio = component.norm() / denom;
    if ratio.is_finite() {
        Ok(ratio.clamp(0.0, 1.0))
    } else {
        Ok(0.0)
    }
}

/// Run PSTB analysis using a PSS operating point and monodromy-based Floquet analysis.
pub fn run_pstb_analysis_with_config(
    netlist_text: &str,
    config: &PstbRunConfig,
) -> Result<PstbData, String> {
    run_pstb_analysis_with_config_and_abort(netlist_text, config, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run PSTB analysis with cooperative cancellation.
pub fn run_pstb_analysis_with_config_and_abort(
    netlist_text: &str,
    config: &PstbRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PstbData> {
    run_pstb_analysis_with_config_and_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run PSTB analysis using a PSS operating point and monodromy-based Floquet
/// analysis, resolving relative includes from the source path when provided.
pub fn run_pstb_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &PstbRunConfig,
    source_path: Option<&Path>,
) -> Result<PstbData, String> {
    run_pstb_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        config,
        source_path,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run PSTB analysis with cooperative cancellation and source-relative include
/// resolution.
pub fn run_pstb_analysis_with_config_and_source_path_and_abort(
    netlist_text: &str,
    config: &PstbRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PstbData> {
    use rspice_core::analysis::PssConfig;
    use rspice_core::analysis::advanced::pstb::{PstbAnalyzer, PstbConfig};

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

    let pss_harmonics = config.pss_num_harmonics.max(config.max_harmonics);
    let pss_config = PssConfig::new(config.pss_fundamental_freq)
        .with_harmonics(pss_harmonics)
        .with_tolerance(config.pss_tolerance)
        .with_max_iterations(50)
        .with_tstab_periods(10);
    let pss_result = engine
        .run_pss_with_abort(&netlist, pss_config, abort)
        .map_err(|error| match error {
            rspice_core::SimulationError::Aborted => ServiceRunError::Aborted,
            other => ServiceRunError::from(PstbRunError::Pss(other.to_string())),
        })?;
    ensure_not_aborted(abort)?;

    if pss_result.monodromy.is_empty() {
        return Err(PstbRunError::EmptyMonodromy.into());
    }
    let monodromy_dim = pss_result.monodromy.len();
    for (index, row) in pss_result.monodromy.iter().enumerate() {
        poll_periodically(abort, index)?;
        if row.len() != monodromy_dim {
            return Err(PstbRunError::NonSquareMonodromy.into());
        }
    }
    if probe.state_index >= monodromy_dim {
        return Err(PstbRunError::ProbeStateOutOfRange {
            probe: probe.canonical_name.clone(),
            state_index: probe.state_index,
            monodromy_dim,
        }
        .into());
    }

    let probe_row = &pss_result.monodromy[probe.state_index];
    let probe_self_transition = sanitize_finite(probe_row[probe.state_index]);
    let probe_column_norm = sanitize_nonnegative(finite_l2_norm_with_abort(
        pss_result
            .monodromy
            .iter()
            .map(|row| row[probe.state_index]),
        abort,
    )?);
    let probe_row_norm =
        sanitize_nonnegative(finite_l2_norm_with_abort(probe_row.iter().copied(), abort)?);
    let probe_persistence_db = sanitize_db(20.0 * probe_self_transition.abs().max(1e-30).log10());

    let pstb_config = PstbConfig::new()
        .with_num_eigenvalues(config.num_multipliers)
        .with_eigenvectors(true)
        .with_tolerance(config.eigenvalue_tolerance)
        .with_stability_threshold(config.stability_threshold)
        .with_subharmonic_detection(config.detect_subharmonics);
    let mut analyzer = PstbAnalyzer::new(pstb_config);
    let pstb_result = analyzer
        .analyze_monodromy_with_abort(&pss_result.monodromy, pss_result.period, abort)
        .map_err(ServiceRunError::from)?;

    let retained_modes = pstb_result.multipliers.len().min(config.num_multipliers);
    let mut mode_indices = Vec::with_capacity(retained_modes);
    let mut probe_mode_participation = Vec::with_capacity(retained_modes);
    let mut multiplier_magnitude = Vec::with_capacity(retained_modes);
    let mut multiplier_phase_deg = Vec::with_capacity(retained_modes);
    let mut mode_damping = Vec::with_capacity(retained_modes);
    let mut mode_frequency_hz = Vec::with_capacity(retained_modes);
    let mut stability_margin_db = Vec::with_capacity(retained_modes);

    for (idx, multiplier) in pstb_result
        .multipliers
        .iter()
        .take(config.num_multipliers)
        .enumerate()
    {
        poll_periodically(abort, idx)?;
        mode_indices.push((idx + 1) as Value);
        probe_mode_participation.push(sanitize_nonnegative(
            normalized_probe_participation_with_abort(
                multiplier.eigenvector.as_deref(),
                probe.state_index,
                abort,
            )?,
        ));
        multiplier_magnitude.push(sanitize_nonnegative(multiplier.magnitude()));
        multiplier_phase_deg.push(sanitize_finite(multiplier.phase_degrees()));
        mode_damping.push(sanitize_finite(multiplier.damping()));
        mode_frequency_hz.push(sanitize_nonnegative(multiplier.natural_frequency()));
        stability_margin_db.push(sanitize_db(multiplier.stability_margin_db()));
    }

    if mode_indices.is_empty() {
        return Err(PstbRunError::NoFloquetMultipliers.into());
    }
    let mut dominant_probe_mode = 0;
    let mut dominant_probe_mode_participation = 0.0;
    for (index, &participation) in probe_mode_participation.iter().enumerate() {
        poll_periodically(abort, index)?;
        if dominant_probe_mode == 0
            || participation
                .total_cmp(&dominant_probe_mode_participation)
                .is_gt()
        {
            dominant_probe_mode = index + 1;
            dominant_probe_mode_participation = participation;
        }
    }
    ensure_not_aborted(abort)?;

    Ok(PstbData {
        period: pstb_result.period,
        fundamental_frequency: pstb_result.fundamental_frequency,
        probe_instance: probe.canonical_name,
        probe_branch_ordinal: probe.branch_ordinal,
        probe_state_index: probe.state_index,
        probe_state_self_transition: probe_self_transition,
        probe_state_column_norm: probe_column_norm,
        probe_state_row_norm: probe_row_norm,
        probe_state_persistence_db: probe_persistence_db,
        mode_indices,
        probe_mode_participation,
        multiplier_magnitude,
        multiplier_phase_deg,
        mode_damping,
        mode_frequency_hz,
        stability_margin_db,
        dominant_multiplier_magnitude: pstb_result.max_multiplier_magnitude,
        min_stability_margin_db: sanitize_db(pstb_result.min_stability_margin_db),
        dominant_probe_mode,
        dominant_probe_mode_participation,
        num_unstable: pstb_result.num_unstable,
        stability_classification: stability_type_label(pstb_result.stability).to_string(),
        is_stable: pstb_result.is_stable(),
    })
}

/// Run PSTB analysis with default configuration.
pub fn run_pstb_analysis(netlist_text: &str) -> Result<PstbData, String> {
    run_pstb_analysis_with_abort(netlist_text, &NoAbort).map_err(|error| error.to_string())
}

/// Run PSTB analysis with default configuration and cooperative cancellation.
pub fn run_pstb_analysis_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PstbData> {
    run_pstb_analysis_with_source_path_and_abort(netlist_text, None, abort)
}

/// Run PSTB analysis with default configuration and a source path used to
/// resolve relative includes and model file references.
pub fn run_pstb_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<PstbData, String> {
    run_pstb_analysis_with_source_path_and_abort(netlist_text, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run PSTB analysis with default configuration, source-relative include
/// resolution, and cooperative cancellation.
pub fn run_pstb_analysis_with_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PstbData> {
    let cfg = PstbRunConfig::default();
    run_pstb_analysis_with_config_and_source_path_and_abort(netlist_text, &cfg, source_path, abort)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::ImmediateAbort;

    #[test]
    fn pstb_service_preserves_typed_entry_abort() {
        let mut config = PstbRunConfig::default();
        config.pss_fundamental_freq = 0.0;

        let result =
            run_pstb_analysis_with_config_and_abort("not a netlist", &config, &ImmediateAbort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }
}
