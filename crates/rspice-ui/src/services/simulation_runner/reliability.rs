use super::build_engine_config;
use crate::simulation::reliability_engine::{
    ParamShift, ReliabilityEngine, ReliabilityResult, StressMetrics,
};
use rspice_core::engine::Engine;
use rspice_core::netlist::{Element, ElementKind};
use rspice_core::solver::SimulationResult as CoreSimulationResult;
use rspice_core::Value;
use std::collections::HashMap;

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
    pub(super) fn validate(&self) -> Result<(), String> {
        if self.target_years.is_empty() {
            return Err("Reliability target years must not be empty".to_string());
        }
        if self
            .target_years
            .iter()
            .any(|years| !years.is_finite() || *years <= 0.0)
        {
            return Err("Reliability target years must be finite and > 0".to_string());
        }
        if !self.enable_hci && !self.enable_nbti && !self.enable_em {
            return Err("Reliability requires at least one enabled mechanism".to_string());
        }
        if !self.min_stress_voltage.is_finite() || self.min_stress_voltage < 0.0 {
            return Err("Reliability min stress voltage must be finite and >= 0".to_string());
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
    run_reliability_analysis_with_config(netlist_text, &ReliabilityRunConfig::default())
}

/// Run reliability analysis using explicit configuration.
pub fn run_reliability_analysis_with_config(
    netlist_text: &str,
    config: &ReliabilityRunConfig,
) -> Result<ReliabilityData, String> {
    config.validate()?;

    let mut years = config.target_years.clone();
    years.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    years.dedup_by(|a, b| (*a - *b).abs() <= 1e-12);

    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;
    let sim_config = build_engine_config(&netlist, None);
    let temperature_k = sim_config.temperature;
    let engine = Engine::new(sim_config);
    let dc_result = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC operating point error: {}", e))?;

    let node_voltages = build_node_voltage_lookup(&dc_result);
    let stress_data = extract_reliability_stress_data(
        &netlist.elements,
        &node_voltages,
        temperature_k,
        config.min_stress_voltage,
    );
    if stress_data.is_empty() {
        return Err(
            "Reliability analysis found no stressed semiconductor devices in the circuit"
                .to_string(),
        );
    }

    let reliability_engine = ReliabilityEngine::new();
    let mut device_results = reliability_engine.analyze_circuit(&stress_data, &years);
    apply_reliability_mechanism_scaling(&mut device_results, config);
    device_results.sort_by_cached_key(|result| result.device_id.to_ascii_uppercase());

    Ok(ReliabilityData {
        years,
        device_results,
    })
}

fn build_node_voltage_lookup(dc_result: &CoreSimulationResult) -> HashMap<String, Value> {
    let mut lookup = HashMap::new();
    for (idx, node_name) in dc_result.node_names.iter().enumerate() {
        if let Some(voltage) = dc_result.node_voltages.get(idx) {
            lookup.insert(node_name.clone(), *voltage);
            lookup.insert(node_name.to_ascii_uppercase(), *voltage);
        }
    }
    lookup.insert("0".to_string(), 0.0);
    lookup.insert("GND".to_string(), 0.0);
    lookup
}

fn resolve_node_voltage(node_voltages: &HashMap<String, Value>, node_name: &str) -> Value {
    let trimmed = node_name.trim();
    if trimmed.is_empty() {
        return 0.0;
    }
    if trimmed == "0" || trimmed.eq_ignore_ascii_case("gnd") {
        return 0.0;
    }
    node_voltages
        .get(trimmed)
        .copied()
        .or_else(|| node_voltages.get(&trimmed.to_ascii_uppercase()).copied())
        .unwrap_or(0.0)
}

fn extract_reliability_stress_data(
    elements: &[Element],
    node_voltages: &HashMap<String, Value>,
    temperature_k: Value,
    min_stress_voltage: Value,
) -> HashMap<String, StressMetrics> {
    let mut stress_data = HashMap::new();
    let min_stress = min_stress_voltage.max(0.0);

    for element in elements {
        let stress_pair = match &element.kind {
            ElementKind::Mosfet { .. } | ElementKind::Jfet { .. } | ElementKind::Mesfet { .. } => {
                if element.nodes.len() < 3 {
                    None
                } else {
                    let vd = resolve_node_voltage(node_voltages, &element.nodes[0]);
                    let vg = resolve_node_voltage(node_voltages, &element.nodes[1]);
                    let vs = resolve_node_voltage(node_voltages, &element.nodes[2]);
                    Some(((vg - vs).abs(), (vd - vs).abs()))
                }
            }
            ElementKind::Bjt { .. } => {
                if element.nodes.len() < 3 {
                    None
                } else {
                    let vc = resolve_node_voltage(node_voltages, &element.nodes[0]);
                    let vb = resolve_node_voltage(node_voltages, &element.nodes[1]);
                    let ve = resolve_node_voltage(node_voltages, &element.nodes[2]);
                    Some(((vb - ve).abs(), (vc - ve).abs()))
                }
            }
            ElementKind::Diode { .. } => {
                if element.nodes.len() < 2 {
                    None
                } else {
                    let va = resolve_node_voltage(node_voltages, &element.nodes[0]);
                    let vk = resolve_node_voltage(node_voltages, &element.nodes[1]);
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

    stress_data
}

fn apply_reliability_mechanism_scaling(
    results: &mut [ReliabilityResult],
    config: &ReliabilityRunConfig,
) {
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

    for result in results {
        for shift in result.shifts.values_mut() {
            apply_shift_factors(shift, vth_factor, mobility_factor, rds_factor);
        }
    }
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
pub(super) fn extract_reliability_stress_data_for_tests(
    elements: &[Element],
    node_voltages: &HashMap<String, Value>,
    temperature_k: Value,
    min_stress_voltage: Value,
) -> HashMap<String, StressMetrics> {
    extract_reliability_stress_data(elements, node_voltages, temperature_k, min_stress_voltage)
}

#[cfg(test)]
pub(super) fn apply_reliability_mechanism_scaling_for_tests(
    results: &mut [ReliabilityResult],
    config: &ReliabilityRunConfig,
) {
    apply_reliability_mechanism_scaling(results, config)
}
