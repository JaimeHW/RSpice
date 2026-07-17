use super::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically};
use super::{
    is_ground_like, normalize_voltage_signal_name, parse_runner_netlist_with_abort,
    run_transient_analysis_with_source_path_and_abort,
};
use crate::services::safety::{
    SoADefinition, SoAEvaluation, SoALimit, SoAManager, SoAParameter, SoAViolation,
};
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::netlist::{Element, ElementKind};
use std::collections::HashMap;
use std::path::Path;

/// Configuration for SOA analysis.
#[derive(Debug, Clone)]
pub struct SoaRunConfig {
    /// Transient stop time.
    pub stop_time: Value,
    /// Transient step time.
    pub step_time: Value,
    /// Enable Vgs limit checks.
    pub check_vgs_max: bool,
    /// Maximum allowed Vgs magnitude.
    pub max_vgs: Value,
    /// Enable Vds limit checks.
    pub check_vds_max: bool,
    /// Maximum allowed Vds magnitude.
    pub max_vds: Value,
    /// Enable Vbe limit checks.
    pub check_vbe_max: bool,
    /// Maximum allowed Vbe magnitude.
    pub max_vbe: Value,
    /// Enable Vce limit checks.
    pub check_vce_max: bool,
    /// Maximum allowed Vce magnitude.
    pub max_vce: Value,
}

impl Default for SoaRunConfig {
    fn default() -> Self {
        Self {
            stop_time: 1e-6,
            step_time: 1e-9,
            check_vgs_max: true,
            max_vgs: 1.8,
            check_vds_max: true,
            max_vds: 3.3,
            check_vbe_max: true,
            max_vbe: 0.9,
            check_vce_max: true,
            max_vce: 5.0,
        }
    }
}

impl SoaRunConfig {
    pub(super) fn validate(&self) -> Result<(), String> {
        if !self.stop_time.is_finite() || self.stop_time <= 0.0 {
            return Err("SOA stop_time must be finite and > 0".to_string());
        }
        if !self.step_time.is_finite() || self.step_time <= 0.0 {
            return Err("SOA step_time must be finite and > 0".to_string());
        }
        if self.step_time > self.stop_time {
            return Err("SOA step_time must be <= stop_time".to_string());
        }
        if !self.check_vgs_max && !self.check_vds_max && !self.check_vbe_max && !self.check_vce_max
        {
            return Err("SOA requires at least one enabled check".to_string());
        }
        if self.check_vgs_max && (!self.max_vgs.is_finite() || self.max_vgs <= 0.0) {
            return Err("SOA max_vgs must be finite and > 0 when enabled".to_string());
        }
        if self.check_vds_max && (!self.max_vds.is_finite() || self.max_vds <= 0.0) {
            return Err("SOA max_vds must be finite and > 0 when enabled".to_string());
        }
        if self.check_vbe_max && (!self.max_vbe.is_finite() || self.max_vbe <= 0.0) {
            return Err("SOA max_vbe must be finite and > 0 when enabled".to_string());
        }
        if self.check_vce_max && (!self.max_vce.is_finite() || self.max_vce <= 0.0) {
            return Err("SOA max_vce must be finite and > 0 when enabled".to_string());
        }
        Ok(())
    }
}

/// SOA analysis output.
#[derive(Debug, Clone)]
pub struct SoaData {
    /// Transient time vector.
    pub time: Vec<Value>,
    /// Cumulative violation count over time.
    pub violation_count: Vec<Value>,
    /// Collected violations.
    pub violations: Vec<SoAViolation>,
    /// Complete worst-point evidence for every evaluated device rule.
    pub evaluations: Vec<SoAEvaluation>,
}

/// Run SOA analysis using default configuration.
pub fn run_soa_analysis(netlist_text: &str) -> Result<SoaData, String> {
    run_soa_analysis_with_abort(netlist_text, &NoAbort).map_err(|error| error.to_string())
}

/// Run SOA analysis with cooperative cancellation.
pub fn run_soa_analysis_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SoaData> {
    run_soa_analysis_with_source_path_and_abort(netlist_text, None, abort)
}

/// Run SOA analysis using default configuration and a source path used to
/// resolve relative includes and model file references.
pub fn run_soa_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<SoaData, String> {
    run_soa_analysis_with_source_path_and_abort(netlist_text, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run SOA analysis with source-path resolution and cooperative cancellation.
pub fn run_soa_analysis_with_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SoaData> {
    run_soa_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        &SoaRunConfig::default(),
        source_path,
        abort,
    )
}

/// Run SOA analysis using explicit configuration.
pub fn run_soa_analysis_with_config(
    netlist_text: &str,
    config: &SoaRunConfig,
) -> Result<SoaData, String> {
    run_soa_analysis_with_config_and_abort(netlist_text, config, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run explicitly configured SOA analysis with cooperative cancellation.
pub fn run_soa_analysis_with_config_and_abort(
    netlist_text: &str,
    config: &SoaRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SoaData> {
    run_soa_analysis_with_config_and_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run SOA analysis using explicit configuration and a source path used to
/// resolve relative includes and model file references.
pub fn run_soa_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &SoaRunConfig,
    source_path: Option<&Path>,
) -> Result<SoaData, String> {
    run_soa_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        config,
        source_path,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run explicitly configured SOA analysis with source-path resolution and
/// cooperative cancellation through parsing, transient solving, and every
/// device/time-point check.
pub fn run_soa_analysis_with_config_and_source_path_and_abort(
    netlist_text: &str,
    config: &SoaRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SoaData> {
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let transient = run_transient_analysis_with_source_path_and_abort(
        netlist_text,
        config.stop_time,
        config.step_time,
        source_path,
        abort,
    )?;

    let mut manager = SoAManager::new();
    register_soa_limits_for_netlist(&mut manager, &netlist.elements, config, abort)?;
    if manager.violations().is_empty() && netlist.elements.is_empty() {
        return Err(ServiceRunError::Failure(
            "SOA analysis received an empty netlist".to_string(),
        ));
    }

    let mut active_devices = 0usize;
    for (element_index, element) in netlist.elements.iter().enumerate() {
        poll_periodically(abort, element_index)?;
        if is_soa_supported_element(&element.kind) {
            active_devices += 1;
        }
    }
    if active_devices == 0 {
        return Err(ServiceRunError::Failure(
            "SOA analysis found no supported semiconductor devices".to_string(),
        ));
    }

    let node_waveforms =
        build_transient_node_lookup(&transient.voltages, transient.time.len(), abort)?;
    let mut violation_count = Vec::with_capacity(transient.time.len());

    for (idx, &time) in transient.time.iter().enumerate() {
        poll_periodically(abort, idx)?;
        let mut values: HashMap<String, HashMap<SoAParameter, Value>> = HashMap::new();

        for (element_index, element) in netlist.elements.iter().enumerate() {
            poll_periodically(abort, element_index)?;
            match &element.kind {
                ElementKind::Mosfet { .. }
                | ElementKind::Jfet { .. }
                | ElementKind::Mesfet { .. } => {
                    if element.nodes.len() < 3 {
                        continue;
                    }
                    let vd = sample_node_waveform(&node_waveforms, &element.nodes[0], idx)?;
                    let vg = sample_node_waveform(&node_waveforms, &element.nodes[1], idx)?;
                    let vs = sample_node_waveform(&node_waveforms, &element.nodes[2], idx)?;
                    let mut device_values = HashMap::new();
                    if config.check_vgs_max {
                        device_values.insert(SoAParameter::Vgs, (vg - vs).abs());
                    }
                    if config.check_vds_max {
                        device_values.insert(SoAParameter::Vds, (vd - vs).abs());
                    }
                    if !device_values.is_empty() {
                        values.insert(element.name.clone(), device_values);
                    }
                }
                ElementKind::Bjt { .. } => {
                    if element.nodes.len() < 3 {
                        continue;
                    }
                    let vc = sample_node_waveform(&node_waveforms, &element.nodes[0], idx)?;
                    let vb = sample_node_waveform(&node_waveforms, &element.nodes[1], idx)?;
                    let ve = sample_node_waveform(&node_waveforms, &element.nodes[2], idx)?;
                    let mut device_values = HashMap::new();
                    if config.check_vbe_max {
                        device_values.insert(SoAParameter::Vbe, (vb - ve).abs());
                    }
                    if config.check_vce_max {
                        device_values.insert(SoAParameter::Vce, (vc - ve).abs());
                    }
                    if !device_values.is_empty() {
                        values.insert(element.name.clone(), device_values);
                    }
                }
                _ => {}
            }
        }

        manager
            .check_point(time, &values)
            .map_err(ServiceRunError::Failure)?;
        violation_count.push(manager.violations().len() as Value);
    }

    let mut violations = Vec::with_capacity(manager.violations().len());
    for (violation_index, violation) in manager.violations().iter().enumerate() {
        poll_periodically(abort, violation_index)?;
        violations.push(violation.clone());
    }
    let mut evaluations = manager.evaluations().cloned().collect::<Vec<_>>();
    evaluations.sort_by(|left, right| {
        left.device_id
            .cmp(&right.device_id)
            .then_with(|| left.parameter.cmp(&right.parameter))
    });
    ensure_not_aborted(abort)?;
    Ok(SoaData {
        time: transient.time,
        violation_count,
        violations,
        evaluations,
    })
}

fn is_soa_supported_element(kind: &ElementKind) -> bool {
    matches!(
        kind,
        ElementKind::Mosfet { .. }
            | ElementKind::Jfet { .. }
            | ElementKind::Mesfet { .. }
            | ElementKind::Bjt { .. }
    )
}

fn register_soa_limits_for_netlist(
    manager: &mut SoAManager,
    elements: &[Element],
    config: &SoaRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<()> {
    for (element_index, element) in elements.iter().enumerate() {
        poll_periodically(abort, element_index)?;
        let mut def = SoADefinition::new();
        match &element.kind {
            ElementKind::Mosfet { .. } | ElementKind::Jfet { .. } | ElementKind::Mesfet { .. } => {
                if config.check_vgs_max {
                    def.add_limit(SoALimit {
                        parameter: SoAParameter::Vgs,
                        max_value: config.max_vgs,
                        unit: "V".to_string(),
                        description: "Maximum gate-source voltage".to_string(),
                    });
                }
                if config.check_vds_max {
                    def.add_limit(SoALimit {
                        parameter: SoAParameter::Vds,
                        max_value: config.max_vds,
                        unit: "V".to_string(),
                        description: "Maximum drain-source voltage".to_string(),
                    });
                }
            }
            ElementKind::Bjt { .. } => {
                if config.check_vbe_max {
                    def.add_limit(SoALimit {
                        parameter: SoAParameter::Vbe,
                        max_value: config.max_vbe,
                        unit: "V".to_string(),
                        description: "Maximum base-emitter voltage".to_string(),
                    });
                }
                if config.check_vce_max {
                    def.add_limit(SoALimit {
                        parameter: SoAParameter::Vce,
                        max_value: config.max_vce,
                        unit: "V".to_string(),
                        description: "Maximum collector-emitter voltage".to_string(),
                    });
                }
            }
            _ => continue,
        }
        if !def.limits.is_empty() {
            manager
                .register_device(element.name.clone(), def)
                .map_err(ServiceRunError::Failure)?;
        }
    }
    ensure_not_aborted(abort)
}

fn build_transient_node_lookup(
    voltages: &[(String, Vec<Value>)],
    expected_samples: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<HashMap<String, Vec<Value>>> {
    let mut map = HashMap::with_capacity(voltages.len() + 2);
    for (trace_index, (name, values)) in voltages.iter().enumerate() {
        poll_periodically(abort, trace_index)?;
        if values.len() != expected_samples {
            return Err(ServiceRunError::Failure(format!(
                "SOA voltage trace '{}' has {} samples; expected {expected_samples}",
                name,
                values.len()
            )));
        }
        let mut copied_values = Vec::with_capacity(values.len());
        for (sample_index, value) in values.iter().copied().enumerate() {
            poll_periodically(abort, sample_index)?;
            if !value.is_finite() {
                return Err(ServiceRunError::Failure(format!(
                    "SOA voltage trace '{name}' contains a non-finite sample at index {sample_index}"
                )));
            }
            copied_values.push(value);
        }
        let key = normalize_voltage_signal_name(name);
        if map.insert(key.clone(), copied_values).is_some() {
            return Err(ServiceRunError::Failure(format!(
                "SOA solver returned duplicate voltage trace '{key}'"
            )));
        }
    }
    map.insert("0".to_string(), Vec::new());
    map.insert("GND".to_string(), Vec::new());
    ensure_not_aborted(abort)?;
    Ok(map)
}

fn sample_node_waveform(
    waveforms: &HashMap<String, Vec<Value>>,
    node_name: &str,
    idx: usize,
) -> ServiceRunResult<Value> {
    if is_ground_like(node_name) {
        return Ok(0.0);
    }
    let key = node_name.trim().to_ascii_uppercase();
    waveforms
        .get(&key)
        .and_then(|values| values.get(idx).copied())
        .ok_or_else(|| {
            ServiceRunError::Failure(format!(
                "SOA solver did not retain required node '{node_name}' sample {idx}"
            ))
        })
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
    fn soa_honors_early_abort_before_invalid_input() {
        let abort = AbortOnPoll::new(1);
        let result = run_soa_analysis_with_abort("invalid", &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn soa_honors_abort_inside_waveform_sample_conversion() {
        let voltages = vec![("V(out)".to_string(), vec![1.0; 512])];
        let abort = AbortOnPoll::new(3);
        let result = build_transient_node_lookup(&voltages, 512, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.polls.load(Ordering::Relaxed) >= 3);
    }

    #[test]
    fn soa_rejects_missing_and_short_required_voltage_traces() {
        let abort = NoAbort;
        let short = vec![("V(out)".to_owned(), vec![0.0])];
        assert!(
            build_transient_node_lookup(&short, 2, &abort)
                .expect_err("short trace must fail closed")
                .to_string()
                .contains("expected 2")
        );

        let traces = HashMap::from([("OUT".to_owned(), vec![0.0, 1.0])]);
        assert!(
            sample_node_waveform(&traces, "missing", 0)
                .expect_err("missing node must fail closed")
                .to_string()
                .contains("required node")
        );
        assert!(
            sample_node_waveform(&traces, "out", 2)
                .expect_err("missing sample must fail closed")
                .to_string()
                .contains("sample 2")
        );
    }
}
