use super::build_engine_config;
use rspice_core::engine::Engine;
use rspice_core::Value;
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
    fn validate(&self) -> Result<(), String> {
        if !self.pss_fundamental_freq.is_finite() || self.pss_fundamental_freq <= 0.0 {
            return Err("PSTB requires a positive PSS fundamental frequency".to_string());
        }
        if self.pss_num_harmonics == 0 {
            return Err("PSTB requires at least one PSS harmonic".to_string());
        }
        if !self.pss_tolerance.is_finite() || self.pss_tolerance <= 0.0 {
            return Err("PSTB requires a positive PSS tolerance".to_string());
        }
        if self.probe_instance.trim().is_empty() {
            return Err("PSTB probe instance must be specified".to_string());
        }
        if self.max_harmonics == 0 {
            return Err("PSTB max harmonics must be greater than zero".to_string());
        }
        if self.num_multipliers == 0 {
            return Err("PSTB number of multipliers must be greater than zero".to_string());
        }
        if !self.stability_threshold.is_finite() || self.stability_threshold <= 0.0 {
            return Err("PSTB stability threshold must be positive".to_string());
        }
        if !self.eigenvalue_tolerance.is_finite() || self.eigenvalue_tolerance <= 0.0 {
            return Err("PSTB eigenvalue tolerance must be positive".to_string());
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
    let mut names = Vec::new();
    names.extend(circuit.inductors.names.iter().cloned());
    names.extend(circuit.voltage_sources.names.iter().cloned());
    names.extend(circuit.ccvs.names.iter().cloned());
    names.extend(
        circuit
            .behavioral_sources
            .voltage_sources
            .iter()
            .map(|src| src.name.clone()),
    );
    normalize_branch_name_list(names)
}

fn available_inductor_probe_names(circuit: &rspice_core::circuit::CircuitData) -> Vec<String> {
    normalize_branch_name_list(circuit.inductors.names.clone())
}

fn resolve_pstb_probe(
    circuit: &rspice_core::circuit::CircuitData,
    probe_instance: &str,
) -> Result<ResolvedPstbProbe, String> {
    let probe_name = probe_instance.trim();
    let branch_ordinal = circuit.get_branch_by_name(probe_name).ok_or_else(|| {
        let available = format_branch_name_list(&available_branch_names(circuit));
        format!(
            "PSTB probe '{}' was not found in branch-capable elements. Available branches: {}",
            probe_name, available
        )
    })? as usize;

    let inductor_index = circuit
        .inductors
        .branch_indices
        .iter()
        .position(|branch| *branch == branch_ordinal)
        .ok_or_else(|| {
            let available = format_branch_name_list(&available_inductor_probe_names(circuit));
            format!(
                "PSTB probe '{}' resolved to branch ordinal {} but is not an inductor probe. \
PSTB currently supports dynamic inductor-current probes only. Available inductor probes: {}",
                probe_name, branch_ordinal, available
            )
        })?;

    let state_index = circuit.capacitors.len() + inductor_index;
    let canonical_name = circuit.inductors.names[inductor_index].clone();

    Ok(ResolvedPstbProbe {
        canonical_name,
        branch_ordinal,
        state_index,
    })
}

fn finite_l2_norm(values: impl Iterator<Item = Value>) -> Value {
    let mut sum_sq = 0.0;
    for value in values {
        if !value.is_finite() {
            return f64::INFINITY;
        }
        sum_sq += value * value;
    }
    sum_sq.sqrt()
}

fn sanitize_nonnegative(value: Value) -> Value {
    if value.is_finite() && value >= 0.0 {
        value
    } else {
        0.0
    }
}

fn sanitize_finite(value: Value) -> Value {
    if value.is_finite() {
        value
    } else {
        0.0
    }
}

fn normalized_probe_participation(
    eigenvector: Option<&Vec<num_complex::Complex64>>,
    state_index: usize,
) -> Value {
    let Some(vector) = eigenvector else {
        return 0.0;
    };
    let Some(component) = vector.get(state_index) else {
        return 0.0;
    };
    let denom = vector.iter().map(|v| v.norm_sqr()).sum::<Value>().sqrt();
    if !denom.is_finite() || denom <= 1e-30 {
        return 0.0;
    }
    let ratio = component.norm() / denom;
    if ratio.is_finite() {
        ratio.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

/// Run PSTB analysis using a PSS operating point and monodromy-based Floquet analysis.
pub fn run_pstb_analysis_with_config(
    netlist_text: &str,
    config: &PstbRunConfig,
) -> Result<PstbData, String> {
    use rspice_core::analysis::advanced::pstb::{PstbAnalyzer, PstbConfig};
    use rspice_core::analysis::PssConfig;

    config.validate()?;

    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    let mut sim_config = build_engine_config(&netlist, None);
    sim_config.tolerance = config.pss_tolerance;
    let engine = Engine::new(sim_config);
    let circuit = engine
        .build_circuit(&netlist)
        .map_err(|e| format!("PSTB prerequisite circuit-build error: {}", e))?;
    let probe = resolve_pstb_probe(&circuit, &config.probe_instance)?;

    let pss_harmonics = config.pss_num_harmonics.max(config.max_harmonics);
    let pss_config = PssConfig::new(config.pss_fundamental_freq)
        .with_harmonics(pss_harmonics)
        .with_tolerance(config.pss_tolerance)
        .with_max_iterations(50)
        .with_tstab_periods(10);
    let pss_result = engine
        .run_pss(&netlist, pss_config)
        .map_err(|e| format!("PSTB prerequisite PSS error: {}", e))?;

    if pss_result.monodromy.is_empty() {
        return Err("PSTB prerequisite PSS returned an empty monodromy matrix".to_string());
    }
    let monodromy_dim = pss_result.monodromy.len();
    if pss_result
        .monodromy
        .iter()
        .any(|row| row.len() != monodromy_dim)
    {
        return Err("PSTB prerequisite PSS returned a non-square monodromy matrix".to_string());
    }
    if probe.state_index >= monodromy_dim {
        return Err(format!(
            "PSTB probe '{}' maps to reactive state {} but monodromy dimension is {}",
            probe.canonical_name, probe.state_index, monodromy_dim
        ));
    }

    let probe_row = &pss_result.monodromy[probe.state_index];
    let probe_self_transition = sanitize_finite(probe_row[probe.state_index]);
    let probe_column_norm = sanitize_nonnegative(finite_l2_norm(
        pss_result
            .monodromy
            .iter()
            .map(|row| row[probe.state_index]),
    ));
    let probe_row_norm = sanitize_nonnegative(finite_l2_norm(probe_row.iter().copied()));
    let probe_persistence_db = sanitize_db(20.0 * probe_self_transition.abs().max(1e-30).log10());

    let pstb_config = PstbConfig::new()
        .with_num_eigenvalues(config.num_multipliers)
        .with_eigenvectors(true)
        .with_tolerance(config.eigenvalue_tolerance)
        .with_stability_threshold(config.stability_threshold)
        .with_subharmonic_detection(config.detect_subharmonics);
    let mut analyzer = PstbAnalyzer::new(pstb_config);
    let pstb_result = analyzer.analyze_monodromy(&pss_result.monodromy, pss_result.period);

    let mut mode_indices = Vec::new();
    let mut probe_mode_participation = Vec::new();
    let mut multiplier_magnitude = Vec::new();
    let mut multiplier_phase_deg = Vec::new();
    let mut mode_damping = Vec::new();
    let mut mode_frequency_hz = Vec::new();
    let mut stability_margin_db = Vec::new();

    for (idx, multiplier) in pstb_result
        .multipliers
        .iter()
        .take(config.num_multipliers)
        .enumerate()
    {
        mode_indices.push((idx + 1) as Value);
        probe_mode_participation.push(sanitize_nonnegative(normalized_probe_participation(
            multiplier.eigenvector.as_ref(),
            probe.state_index,
        )));
        multiplier_magnitude.push(multiplier.magnitude());
        multiplier_phase_deg.push(multiplier.phase_degrees());
        mode_damping.push(multiplier.damping());
        mode_frequency_hz.push(multiplier.natural_frequency());
        stability_margin_db.push(sanitize_db(multiplier.stability_margin_db()));
    }

    if mode_indices.is_empty() {
        return Err("PSTB produced no Floquet multipliers".to_string());
    }
    let (dominant_probe_mode, dominant_probe_mode_participation) = probe_mode_participation
        .iter()
        .copied()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(idx, value)| (idx + 1, value))
        .unwrap_or((0, 0.0));

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
    let cfg = PstbRunConfig::default();
    run_pstb_analysis_with_config(netlist_text, &cfg)
}
