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
    EmptyMonodromy,
    NonSquareMonodromy,
    ProbeStateOutOfRange {
        probe: String,
        state_index: usize,
        monodromy_dim: usize,
    },
    NoFloquetMultipliers,
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
        let magnitude = multiplier.magnitude();
        let phase_degrees = multiplier.phase_degrees();
        let damping = multiplier.damping();
        let natural_frequency = multiplier.natural_frequency();
        let stability_margin = multiplier.stability_margin_db();
        if !multiplier.value.re.is_finite()
            || !multiplier.value.im.is_finite()
            || !magnitude.is_finite()
            || magnitude <= 0.0
            || !phase_degrees.is_finite()
            || !damping.is_finite()
            || !natural_frequency.is_finite()
            || natural_frequency < 0.0
            || !stability_margin.is_finite()
        {
            return Err(PstbRunError::InvalidModeData {
                mode: idx + 1,
                reason: "the current result model cannot faithfully represent a zero or non-finite Floquet mode",
            }
            .into());
        }
        mode_indices.push((idx + 1) as Value);
        probe_mode_participation.push(normalized_probe_participation_with_abort(
            multiplier.eigenvector.as_deref(),
            probe.state_index,
            abort,
        )?);
        multiplier_magnitude.push(magnitude);
        multiplier_phase_deg.push(phase_degrees);
        mode_damping.push(damping);
        mode_frequency_hz.push(natural_frequency);
        stability_margin_db.push(stability_margin);
    }

    if mode_indices.is_empty() {
        return Err(PstbRunError::NoFloquetMultipliers.into());
    }
    ensure_not_aborted(abort)?;

    Ok(PstbData {
        mode_indices,
        probe_mode_participation,
        multiplier_magnitude,
        multiplier_phase_deg,
        mode_damping,
        mode_frequency_hz,
        stability_margin_db,
    })
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
