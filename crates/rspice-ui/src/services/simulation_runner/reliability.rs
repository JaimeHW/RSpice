use super::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically};
use super::{build_engine_config, parse_runner_netlist_with_abort};
use crate::simulation::reliability_engine::{
    ParamShift, ReliabilityEngine, ReliabilityResult, StressMetrics,
};
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::engine::Engine;
use rspice_core::netlist::{Element, ElementKind};
use rspice_core::solver::SimulationResult as CoreSimulationResult;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;

#[derive(Debug)]
enum ReliabilityRunError {
    InvalidConfig(&'static str),
    NoStressedDevices,
}

impl fmt::Display for ReliabilityRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfig(message) => f.write_str(message),
            Self::NoStressedDevices => f.write_str(
                "Reliability analysis found no stressed semiconductor devices in the circuit",
            ),
        }
    }
}

/// Explicit configuration for reliability analysis.
#[derive(Debug, Clone)]
pub struct ReliabilityRunConfig {
    /// Lifetime checkpoints to evaluate (years).
    pub target_years: Vec<Value>,
    /// Enable HCI contribution.
    pub enable_hci: bool,
    /// Enable NBTI contribution.
    pub enable_nbti: bool,
    /// Enable electromigration contribution.
    pub enable_em: bool,
    /// Minimum stress magnitude to include a device.
    pub min_stress_voltage: Value,
}

impl Default for ReliabilityRunConfig {
    fn default() -> Self {
        Self {
            target_years: vec![1.0, 5.0, 10.0],
            enable_hci: true,
            enable_nbti: true,
            enable_em: false,
            min_stress_voltage: 0.1,
        }
    }
}

impl ReliabilityRunConfig {
    fn validate(&self) -> Result<(), ReliabilityRunError> {
        if self.target_years.is_empty() {
            return Err(ReliabilityRunError::InvalidConfig(
                "Reliability target years must not be empty",
            ));
        }
        if self
            .target_years
            .iter()
            .any(|years| !years.is_finite() || *years <= 0.0)
        {
            return Err(ReliabilityRunError::InvalidConfig(
                "Reliability target years must be finite and > 0",
            ));
        }
        if !self.enable_hci && !self.enable_nbti && !self.enable_em {
            return Err(ReliabilityRunError::InvalidConfig(
                "Reliability requires at least one enabled mechanism",
            ));
        }
        if !self.min_stress_voltage.is_finite() || self.min_stress_voltage < 0.0 {
            return Err(ReliabilityRunError::InvalidConfig(
                "Reliability min stress voltage must be finite and >= 0",
            ));
        }
        Ok(())
    }
}

/// Reliability analysis output.
#[derive(Debug, Clone)]
pub struct ReliabilityData {
    /// Evaluated lifetime checkpoints (years).
    pub years: Vec<Value>,
    /// Per-device reliability results.
    pub device_results: Vec<ReliabilityResult>,
}

/// Run reliability analysis with default configuration.
pub fn run_reliability_analysis(netlist_text: &str) -> Result<ReliabilityData, String> {
    run_reliability_analysis_with_abort(netlist_text, &NoAbort).map_err(|error| error.to_string())
}

/// Run reliability analysis with cooperative cancellation.
pub fn run_reliability_analysis_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<ReliabilityData> {
    run_reliability_analysis_with_source_path_and_abort(netlist_text, None, abort)
}

/// Run reliability analysis with default configuration and a source path used
/// to resolve relative includes and model file references.
pub fn run_reliability_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<ReliabilityData, String> {
    run_reliability_analysis_with_source_path_and_abort(netlist_text, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run reliability analysis with source-path resolution and cooperative
/// cancellation.
pub fn run_reliability_analysis_with_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<ReliabilityData> {
    run_reliability_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        &ReliabilityRunConfig::default(),
        source_path,
        abort,
    )
}

/// Run reliability analysis using explicit configuration.
pub fn run_reliability_analysis_with_config(
    netlist_text: &str,
    config: &ReliabilityRunConfig,
) -> Result<ReliabilityData, String> {
    run_reliability_analysis_with_config_and_abort(netlist_text, config, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run explicitly configured reliability analysis with cooperative
/// cancellation.
pub fn run_reliability_analysis_with_config_and_abort(
    netlist_text: &str,
    config: &ReliabilityRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<ReliabilityData> {
    run_reliability_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        config,
        None,
        abort,
    )
}

/// Run reliability analysis using explicit configuration and a source path used
/// to resolve relative includes and model file references.
pub fn run_reliability_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &ReliabilityRunConfig,
    source_path: Option<&Path>,
) -> Result<ReliabilityData, String> {
    run_reliability_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        config,
        source_path,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run explicitly configured reliability analysis with source-path resolution
/// and cooperative cancellation.
pub fn run_reliability_analysis_with_config_and_source_path_and_abort(
    netlist_text: &str,
    config: &ReliabilityRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<ReliabilityData> {
    ensure_not_aborted(abort)?;
    config
        .validate()
        .map_err(|error| ServiceRunError::Failure(error.to_string()))?;

    let mut years = Vec::with_capacity(config.target_years.len());
    for (index, value) in config.target_years.iter().copied().enumerate() {
        poll_periodically(abort, index)?;
        years.push(value);
    }
    ensure_not_aborted(abort)?;
    years.sort_by(|a, b| a.total_cmp(b));
    years.dedup_by(|a, b| (*a - *b).abs() <= 1e-12);
    ensure_not_aborted(abort)?;

    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let sim_config = build_engine_config(&netlist, None);
    let temperature_k = sim_config.temperature;
    let engine = Engine::new(sim_config);
    let dc_result = engine
        .run_dc_op_with_abort(&netlist, abort)
        .map_err(|error| ServiceRunError::from_core("DC operating point error", error))?;

    let node_voltages = build_node_voltage_lookup(&dc_result, abort)?;
    let stress_data = extract_reliability_stress_data(
        &netlist.elements,
        &node_voltages,
        temperature_k,
        config.min_stress_voltage,
        abort,
    )?;
    if stress_data.is_empty() {
        return Err(ServiceRunError::Failure(
            ReliabilityRunError::NoStressedDevices.to_string(),
        ));
    }

    let reliability_engine = ReliabilityEngine::new();
    let mut device_results =
        analyze_circuit_with_abort(&reliability_engine, &stress_data, &years, abort)?;
    apply_reliability_mechanism_scaling(&mut device_results, config, abort)?;
    ensure_not_aborted(abort)?;
    device_results.sort_by_cached_key(|result| result.device_id.to_ascii_uppercase());
    ensure_not_aborted(abort)?;

    Ok(ReliabilityData {
        years,
        device_results,
    })
}

fn build_node_voltage_lookup(
    dc_result: &CoreSimulationResult,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<HashMap<String, Value>> {
    let mut lookup = HashMap::new();
    for (idx, node_name) in dc_result.node_names.iter().enumerate() {
        poll_periodically(abort, idx)?;
        let voltage = dc_result.node_voltages.get(idx).copied().ok_or_else(|| {
            ServiceRunError::Failure(format!(
                "Reliability DC operating point omitted voltage for node '{node_name}'"
            ))
        })?;
        if !voltage.is_finite() {
            return Err(ServiceRunError::Failure(format!(
                "Reliability DC operating point returned a non-finite voltage for node '{node_name}'"
            )));
        }
        lookup.insert(node_name.clone(), voltage);
        lookup.insert(node_name.to_ascii_uppercase(), voltage);
    }
    lookup.insert("0".to_string(), 0.0);
    lookup.insert("GND".to_string(), 0.0);
    ensure_not_aborted(abort)?;
    Ok(lookup)
}

fn resolve_node_voltage(
    node_voltages: &HashMap<String, Value>,
    node_name: &str,
) -> ServiceRunResult<Value> {
    let trimmed = node_name.trim();
    if trimmed.is_empty() {
        return Err(ServiceRunError::Failure(
            "Reliability device contains an empty node name".to_owned(),
        ));
    }
    if trimmed == "0" || trimmed.eq_ignore_ascii_case("gnd") {
        return Ok(0.0);
    }
    node_voltages
        .get(trimmed)
        .copied()
        .or_else(|| node_voltages.get(&trimmed.to_ascii_uppercase()).copied())
        .ok_or_else(|| {
            ServiceRunError::Failure(format!(
                "Reliability DC operating point omitted required node '{trimmed}'"
            ))
        })
}

fn extract_reliability_stress_data(
    elements: &[Element],
    node_voltages: &HashMap<String, Value>,
    temperature_k: Value,
    min_stress_voltage: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<HashMap<String, StressMetrics>> {
    let mut stress_data = HashMap::new();
    let min_stress = min_stress_voltage.max(0.0);

    for (element_index, element) in elements.iter().enumerate() {
        poll_periodically(abort, element_index)?;
        let stress_pair = match &element.kind {
            ElementKind::Mosfet { .. } | ElementKind::Jfet { .. } | ElementKind::Mesfet { .. } => {
                if element.nodes.len() < 3 {
                    None
                } else {
                    let vd = resolve_node_voltage(node_voltages, &element.nodes[0])?;
                    let vg = resolve_node_voltage(node_voltages, &element.nodes[1])?;
                    let vs = resolve_node_voltage(node_voltages, &element.nodes[2])?;
                    Some(((vg - vs).abs(), (vd - vs).abs()))
                }
            }
            ElementKind::Bjt { .. } => {
                if element.nodes.len() < 3 {
                    None
                } else {
                    let vc = resolve_node_voltage(node_voltages, &element.nodes[0])?;
                    let vb = resolve_node_voltage(node_voltages, &element.nodes[1])?;
                    let ve = resolve_node_voltage(node_voltages, &element.nodes[2])?;
                    Some(((vb - ve).abs(), (vc - ve).abs()))
                }
            }
            ElementKind::Diode { .. } => {
                if element.nodes.len() < 2 {
                    None
                } else {
                    let va = resolve_node_voltage(node_voltages, &element.nodes[0])?;
                    let vk = resolve_node_voltage(node_voltages, &element.nodes[1])?;
                    let vak = (va - vk).abs();
                    Some((vak, vak))
                }
            }
            _ => None,
        };

        let Some((avg_vgs_stress, avg_vds_stress)) = stress_pair else {
            continue;
        };
        if avg_vgs_stress.max(avg_vds_stress) < min_stress {
            continue;
        }

        stress_data.insert(
            element.name.clone(),
            StressMetrics {
                avg_vgs_stress,
                avg_vds_stress,
                avg_temp: temperature_k,
                duration: 3600.0,
            },
        );
    }

    ensure_not_aborted(abort)?;
    Ok(stress_data)
}

fn analyze_circuit_with_abort(
    reliability_engine: &ReliabilityEngine,
    stress_data: &HashMap<String, StressMetrics>,
    target_years: &[Value],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<ReliabilityResult>> {
    let mut results = Vec::with_capacity(stress_data.len());

    for (device_index, (device_id, stress)) in stress_data.iter().enumerate() {
        poll_periodically(abort, device_index)?;
        let mut shifts = HashMap::with_capacity(target_years.len());
        for (year_index, &years) in target_years.iter().enumerate() {
            poll_periodically(abort, year_index)?;
            shifts.insert(
                format!("{}y", years),
                reliability_engine.calculate_shift(stress, years),
            );
        }
        results.push(ReliabilityResult {
            device_id: device_id.clone(),
            stress: stress.clone(),
            shifts,
        });
    }

    ensure_not_aborted(abort)?;
    Ok(results)
}

fn apply_reliability_mechanism_scaling(
    results: &mut [ReliabilityResult],
    config: &ReliabilityRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<()> {
    let mut vth_factor = 0.0;
    let mut mobility_factor = 0.0;
    let mut rds_factor = 0.0;

    if config.enable_hci {
        vth_factor += 1.0;
        mobility_factor += 1.0;
        rds_factor += 0.8;
    }
    if config.enable_nbti {
        vth_factor += 0.85;
        mobility_factor += 0.65;
        rds_factor += 0.4;
    }
    if config.enable_em {
        rds_factor += 2.2;
    }

    for (result_index, result) in results.iter_mut().enumerate() {
        poll_periodically(abort, result_index)?;
        for (shift_index, shift) in result.shifts.values_mut().enumerate() {
            poll_periodically(abort, shift_index)?;
            apply_shift_factors(shift, vth_factor, mobility_factor, rds_factor);
        }
    }
    ensure_not_aborted(abort)
}

fn apply_shift_factors(
    shift: &mut ParamShift,
    vth_factor: Value,
    mobility_factor: Value,
    rds_factor: Value,
) {
    shift.vth_shift *= vth_factor;
    shift.mobility_shift *= mobility_factor;
    shift.rds_shift *= rds_factor;
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    struct AbortOnPoll {
        abort_on: usize,
        polls: AtomicUsize,
    }

    impl AbortOnPoll {
        fn new(abort_on: usize) -> Self {
            Self {
                abort_on,
                polls: AtomicUsize::new(0),
            }
        }
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    #[test]
    fn reliability_honors_early_abort_before_invalid_input() {
        let abort = AbortOnPoll::new(1);
        let result = run_reliability_analysis_with_abort("invalid", &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn reliability_honors_abort_during_device_and_lifetime_work() {
        let stress_data = HashMap::from([(
            "M1".to_string(),
            StressMetrics {
                avg_vgs_stress: 1.0,
                avg_vds_stress: 1.0,
                avg_temp: 300.0,
                duration: 3_600.0,
            },
        )]);
        let years: Vec<Value> = (1..=512).map(|year| year as Value).collect();
        let abort = AbortOnPoll::new(3);
        let engine = ReliabilityEngine::new();

        let result = analyze_circuit_with_abort(&engine, &stress_data, &years, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.polls.load(Ordering::Relaxed) >= 3);
    }

    #[test]
    fn reliability_rejects_missing_required_node_voltage() {
        let voltages = HashMap::from([("OUT".to_owned(), 1.0)]);
        assert_eq!(
            resolve_node_voltage(&voltages, "out").expect("case-insensitive node exists"),
            1.0
        );
        assert_eq!(
            resolve_node_voltage(&voltages, "0").expect("ground is canonical"),
            0.0
        );
        assert!(
            resolve_node_voltage(&voltages, "missing")
                .expect_err("missing node must fail closed")
                .to_string()
                .contains("omitted required node")
        );
    }
}
