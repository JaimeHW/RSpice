//! Safe operating area checks.
//!
//! Compares simulated device stress against the configured limits,
//! and reports every violation with the instance and the margin.

use super::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically};
use super::{is_ground_like, normalize_voltage_signal_name, parse_runner_netlist_with_abort};
use crate::services::safety::{
    SoADefinition, SoAEvaluation, SoALimit, SoAManager, SoAParameter, SoAViolation, SoaVoltageBasis,
};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
#[cfg(test)]
use rspice_core::abort_signal::NoAbort;
use rspice_core::netlist::{Element, ElementKind};
use std::collections::HashMap;
use std::path::Path;

mod model_ratings;
mod observation;
mod probes;
mod rules;
#[cfg(test)]
mod rules_tests;
mod terminals;
pub use observation::SoaObservationConfig;
pub use rules::SoaRuleConfig;

/// Configuration for SOA analysis.
#[derive(Debug, Clone)]
pub struct SoaRunConfig {
    /// Import authored native model voltage limits before applying scoped rules.
    pub import_model_voltage_ratings: bool,
    pub observation: SoaObservationConfig,
    pub rules: Vec<SoaRuleConfig>,
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
            observation: SoaObservationConfig::default(),
            rules: Vec::new(),
            import_model_voltage_ratings: false,
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
        self.observation.validate(self.stop_time)?;
        for rule in &self.rules {
            rule.validate()?;
        }
        if !self.stop_time.is_finite() || self.stop_time <= 0.0 {
            return Err("SOA stop_time must be finite and > 0".to_string());
        }
        if !self.step_time.is_finite() || self.step_time <= 0.0 {
            return Err("SOA step_time must be finite and > 0".to_string());
        }
        if self.step_time > self.stop_time {
            return Err("SOA step_time must be <= stop_time".to_string());
        }
        if !self.import_model_voltage_ratings
            && self.rules.is_empty()
            && !self.check_vgs_max
            && !self.check_vds_max
            && !self.check_vbe_max
            && !self.check_vce_max
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

/// The complete sampled stress magnitude behind one evaluated rule.
#[derive(Debug, Clone)]
pub struct SoaStressTrace {
    /// Device the rule constrains.
    pub device_id: String,
    /// Stressed parameter.
    pub parameter: SoAParameter,
    /// One magnitude per transient sample, aligned with [`SoaData::time`].
    pub values: Vec<Value>,
    /// Unit the rule is expressed in.
    pub unit: String,
}

/// SOA analysis output.
#[derive(Debug, Clone)]
pub struct SoaData {
    pub convergence: Option<std::sync::Arc<crate::state::TransientConvergenceEvidence>>,
    /// Transient time vector.
    pub time: Vec<Value>,
    /// Cumulative violation count over time.
    pub violation_count: Vec<Value>,
    /// Collected violations.
    pub violations: Vec<SoAViolation>,
    /// Complete worst-point evidence for every evaluated device rule.
    pub evaluations: Vec<SoAEvaluation>,
    /// Retained stress history per rule, in `evaluations` order.
    ///
    /// A rule appears here only when it was sampled at every retained time
    /// point, so a trace can always be drawn against the full time axis.
    pub stress_history: Vec<SoaStressTrace>,
}

/// Run SOA analysis with default configuration and no source path.
///
/// Test-only. The shipping path is
/// [`run_soa_analysis_with_config_and_source_path_and_abort`], which the device
/// spec calls with the configuration the user set.
#[cfg(test)]
pub fn run_soa_analysis_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SoaData> {
    run_soa_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        &SoaRunConfig::default(),
        None,
        abort,
    )
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
    let mut netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    // The solver evaluates the expanded hierarchy. Register and observe those
    // same concrete instances, including devices inside PDK subcircuits.
    let flattened = rspice_core::netlist::flatten_netlist_with_models_with_abort(&netlist, abort)
        .map_err(|error| match error {
        rspice_core::netlist::ParseWithAbortError::Aborted => ServiceRunError::Aborted,
        rspice_core::netlist::ParseWithAbortError::Parse(
            rspice_core::netlist::ParseError::ResourceLimit(error),
        ) => ServiceRunError::ResourceLimit(error),
        rspice_core::netlist::ParseWithAbortError::Parse(error) => {
            ServiceRunError::Failure(format!("SOA hierarchy error: {error}"))
        }
    })?;
    config
        .observation
        .validate_selection(&flattened.elements)
        .map_err(ServiceRunError::Failure)?;
    let engine = rspice_core::engine::Engine::new(super::build_engine_config(&netlist, None));
    let (layouts, model_limits) =
        terminals::resolve(&netlist, &flattened.elements, config, &engine, abort)?;
    let mut manager = SoAManager::new();
    let resolved = rules::resolve(&flattened.elements, config, &layouts, &model_limits, abort)?;
    let registered_rules: usize = resolved
        .iter()
        .map(|(_, definition)| definition.limits.len())
        .sum();
    for (element_index, definition) in &resolved {
        manager
            .register_device(
                flattened.elements[*element_index].name.clone(),
                definition.clone(),
            )
            .map_err(ServiceRunError::Failure)?;
    }
    if registered_rules == 0 {
        return Err(ServiceRunError::Failure(
            "SOA analysis found no semiconductor device with an applicable enabled rule"
                .to_string(),
        ));
    }

    // Checking a device requires its terminal observations even if a separate
    // output selection was authored for the visible transient analysis.
    netlist.saves.signals = flattened
        .elements
        .iter()
        .filter(|element| config.observation.includes(element))
        .flat_map(|element| {
            element
                .nodes
                .iter()
                .cloned()
                .map(rspice_core::netlist::SaveSignal::Voltage)
        })
        .collect();
    for (index, definition) in &resolved {
        for limit in &definition.limits {
            if let Some(parameter) = rules::observation_parameter(limit) {
                let signal = rspice_core::netlist::SaveSignal::DeviceParam {
                    device: flattened.elements[*index].name.clone(),
                    param: parameter.into(),
                };
                if !netlist.saves.signals.contains(&signal) {
                    netlist.saves.signals.push(signal);
                }
            }
        }
    }
    let current_probes = probes::register(
        &mut netlist,
        &flattened.elements,
        &resolved,
        &layouts,
        abort,
    )?;

    let result = engine
        .run_tran_with_startup_mode_and_abort(
            &netlist,
            config.stop_time,
            config
                .observation
                .max_step
                .map_or(config.step_time, |step| step.min(config.step_time)),
            rspice_core::engine::TransientStartupMode::from_uic(
                config.observation.use_initial_conditions,
            ),
            abort,
        )
        .map_err(|error| ServiceRunError::from_core("SOA transient error", error))?;
    // Solver quality describes the full source solve, including startup.
    let convergence = crate::state::TransientConvergenceEvidence::capture(
        engine.convergence_quality(),
        &result.time,
        abort,
    )
    .map_err(ServiceRunError::from)?;
    let mut observations = HashMap::new();
    for (element_index, definition) in &resolved {
        let element = &flattened.elements[*element_index];
        for limit in &definition.limits {
            let key = (
                *element_index,
                limit.parameter.base_parameter(),
                limit.voltage_basis,
            );
            if observations.contains_key(&key) {
                continue;
            }
            let samples = if let Some(source) =
                current_probes.get(&(*element_index, limit.parameter.base_parameter()))
            {
                Some(result.try_branch_current_waveform_named(source).ok_or_else(||
                    ServiceRunError::Failure(format!(
                        "SOA requires total terminal current {}({}); the solver returned no trace",
                        limit.parameter.stress_code(), element.name
                    ))
                )?)
            } else if let Some(parameter) = rules::observation_parameter(limit) {
                Some(result.try_device_op_waveform_named(&element.name, parameter)
                    .ok_or_else(|| ServiceRunError::Failure(format!(
                        "SOA requires accepted device observation {}({}); the device returned no trace",
                        parameter, element.name
                    )))?)
            } else {
                None
            };
            if let Some(samples) = samples {
                ensure_not_aborted(abort)?;
                if samples.len() != result.time.len() {
                    return Err(ServiceRunError::Failure(format!(
                        "SOA device observation for '{}' has incomplete sample coverage",
                        element.name
                    )));
                }
                let mut values = Vec::with_capacity(samples.len());
                for (index, sample) in samples.iter().copied().enumerate() {
                    poll_periodically(abort, index)?;
                    if !sample.is_finite() {
                        return Err(ServiceRunError::Failure(format!(
                            "SOA device observation for '{}' contains a non-finite sample",
                            element.name
                        )));
                    }
                    let sample = if limit.parameter == SoAParameter::Temp {
                        let kelvin = rspice_core::constants::celsius_to_kelvin(sample);
                        if !kelvin.is_finite() || kelvin <= 0.0 {
                            return Err(ServiceRunError::Failure(format!(
                                "SOA temperature for '{}' must be above absolute zero",
                                element.name
                            )));
                        }
                        kelvin
                    } else {
                        sample
                    };
                    values.push(sample);
                }
                observations.insert(key, values);
            }
        }
    }
    let names = result.node_names.clone();
    let mut transient =
        super::TransientData::from_retained_voltage_history_with_abort(result, &names, abort)?;
    transient.convergence = Some(std::sync::Arc::new(convergence));
    let first = transient
        .time
        .partition_point(|time| *time < config.observation.start_time);
    if first == transient.time.len() {
        return Err(ServiceRunError::Failure(
            "SOA observation window contains no accepted samples".into(),
        ));
    }
    transient.time.drain(..first);
    for (_, values) in &mut transient.voltages {
        if !values.is_empty() {
            values.drain(..first);
        }
    }

    let node_waveforms =
        build_transient_node_lookup(&transient.voltages, transient.time.len(), abort)?;
    let mut violation_count = Vec::with_capacity(transient.time.len());

    for (idx, &time) in transient.time.iter().enumerate() {
        poll_periodically(abort, idx)?;
        let mut values: HashMap<String, HashMap<SoAParameter, Value>> = HashMap::new();

        for (element_index, definition) in &resolved {
            poll_periodically(abort, *element_index)?;
            let element = &flattened.elements[*element_index];
            let mut device_values = HashMap::new();
            for limit in &definition.limits {
                let value = if limit.voltage_basis == SoaVoltageBasis::ExternalTerminals
                    && let Some((positive, reference)) =
                        rules::terminal_pair(limit.parameter, layouts.get(&element.name).copied())
                {
                    if element.nodes.len() <= positive.max(reference) {
                        return Err(ServiceRunError::Failure(format!(
                            "SOA device '{}' has an incomplete terminal basis",
                            element.name
                        )));
                    }
                    sample_node_waveform(&node_waveforms, &element.nodes[positive], idx)?
                        - sample_node_waveform(&node_waveforms, &element.nodes[reference], idx)?
                } else {
                    observations
                        .get(&(
                            *element_index,
                            limit.parameter.base_parameter(),
                            limit.voltage_basis,
                        ))
                        .and_then(|trace| trace.get(idx + first))
                        .copied()
                        .ok_or_else(|| {
                            ServiceRunError::Failure(format!(
                                "SOA device rule for '{}' is missing an accepted sample",
                                element.name
                            ))
                        })?
                };
                if !value.is_finite() {
                    return Err(ServiceRunError::Failure(format!(
                        "SOA {}({}) is non-finite",
                        limit.parameter.stress_code(),
                        element.name
                    )));
                }
                device_values.insert(limit.parameter, limit.parameter.measured_stress(value));
            }
            values.insert(element.name.clone(), device_values);
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
    if evaluations.len() != registered_rules {
        return Err(ServiceRunError::Failure(format!(
            "SOA evaluated {} rules after registering {registered_rules}",
            evaluations.len()
        )));
    }
    let sample_count = transient.time.len();
    let mut stress_history = Vec::with_capacity(evaluations.len());
    for (evaluation_index, evaluation) in evaluations.iter().enumerate() {
        poll_periodically(abort, evaluation_index)?;
        let values = manager
            .stress_history(&evaluation.device_id, evaluation.parameter)
            .ok_or_else(|| {
                ServiceRunError::Failure(format!(
                    "SOA is missing stress history for '{}:{:?}'",
                    evaluation.device_id, evaluation.parameter
                ))
            })?;
        if values.len() != sample_count {
            return Err(ServiceRunError::Failure(format!(
                "SOA stress history for '{}:{:?}' has {} samples; expected {sample_count}",
                evaluation.device_id,
                evaluation.parameter,
                values.len()
            )));
        }
        stress_history.push(SoaStressTrace {
            device_id: evaluation.device_id.clone(),
            parameter: evaluation.parameter,
            values: values.to_vec(),
            unit: evaluation.unit.clone(),
        });
    }
    ensure_not_aborted(abort)?;
    Ok(SoaData {
        convergence: transient.convergence,
        time: transient.time,
        violation_count,
        violations,
        evaluations,
        stress_history,
    })
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
    fn convergence_soa_retains_the_underlying_transient_quality() {
        let result = run_soa_analysis_with_config_and_source_path_and_abort(
            "SOA convergence\nVg g 0 1\nVd d 0 1\nM1 d g 0 0 NM\n.model NM NMOS LEVEL=1\n.end\n",
            &SoaRunConfig {
                stop_time: 1e-6,
                step_time: 1e-7,
                ..Default::default()
            },
            None,
            &NoAbort,
        )
        .unwrap();
        let quality = result
            .convergence
            .as_ref()
            .expect("SOA retains solver quality");
        quality.validate().unwrap();
        let basis = quality.transient.time_basis.as_ref().unwrap();
        assert_eq!(basis.sample_count, result.time.len() as u64);
        assert_eq!(basis.start_s, result.time[0]);
        assert_eq!(basis.stop_s, *result.time.last().unwrap());
        assert!(!result.evaluations.is_empty());
    }

    #[test]
    fn soa_evaluates_each_concrete_device_inside_nested_subcircuits() {
        let deck = "Nested SOA\nVd d 0 1.5\nVlow low 0 0.5\nVhigh high 0 2.5\nXlow d low cell\nXhigh d high cell\n.subckt cell drain gate\nXinner drain gate inner\n.ends cell\n.subckt inner d g\nM1 d g 0 0 NM\n.model NM NMOS LEVEL=1\n.ends inner\n.end\n";
        let result = run_soa_analysis_with_config_and_source_path_and_abort(
            deck,
            &SoaRunConfig {
                stop_time: 1e-8,
                step_time: 1e-9,
                ..Default::default()
            },
            None,
            &NoAbort,
        )
        .unwrap();
        assert_eq!(result.evaluations.len(), 4);
        let mut gates = result
            .evaluations
            .iter()
            .filter(|rule| rule.parameter == SoAParameter::Vgs)
            .collect::<Vec<_>>();
        gates.sort_by(|a, b| a.worst_actual_value.total_cmp(&b.worst_actual_value));
        assert_eq!(gates.len(), 2);
        assert_ne!(gates[0].device_id, gates[1].device_id);
        assert!((gates[0].worst_actual_value - 0.5).abs() < 1e-10);
        assert!((gates[1].worst_actual_value - 2.5).abs() < 1e-10);
        assert_eq!(
            gates[0].verdict,
            crate::services::safety::SoARuleVerdict::Pass
        );
        assert_eq!(
            gates[1].verdict,
            crate::services::safety::SoARuleVerdict::Critical
        );
        assert!(
            result
                .evaluations
                .iter()
                .all(|rule| rule.sample_count == result.time.len() as u64)
        );
    }

    #[test]
    fn soa_observation_window_excludes_startup_stress_and_honors_step_bound() {
        let deck = "Window SOA\nVg g 0 PWL(0 3 0.4u 3 0.5u 1 1u 1)\nVd d 0 1\nM1 d g 0 0 NM\n.model NM NMOS LEVEL=1\n.save V(d)\n.end\n";
        let config = SoaRunConfig {
            stop_time: 1e-6,
            step_time: 1e-7,
            ..Default::default()
        };
        let full =
            run_soa_analysis_with_config_and_source_path_and_abort(deck, &config, None, &NoAbort)
                .unwrap();
        assert!(
            full.evaluations
                .iter()
                .any(|rule| rule.parameter == SoAParameter::Vgs && rule.worst_actual_value > 2.9)
        );
        let selected = run_soa_analysis_with_config_and_source_path_and_abort(
            deck,
            &SoaRunConfig {
                observation: SoaObservationConfig {
                    start_time: 0.75e-6,
                    max_step: Some(1e-8),
                    ..Default::default()
                },
                ..config
            },
            None,
            &NoAbort,
        )
        .unwrap();
        assert!(selected.time[0] >= 0.75e-6);
        assert!(
            selected
                .time
                .windows(2)
                .all(|pair| pair[1] - pair[0] <= 1.00001e-8)
        );
        assert!(
            selected
                .evaluations
                .iter()
                .all(|rule| rule.worst_actual_value < 1.01)
        );
        assert_eq!(
            selected
                .convergence
                .as_ref()
                .unwrap()
                .transient
                .time_basis
                .as_ref()
                .unwrap()
                .start_s,
            0.0
        );
    }

    #[test]
    fn soa_device_and_model_filters_select_only_requested_rules_and_reject_typos() {
        let deck = "Scoped SOA\nVg g 0 2.5\nVd d 0 1\nM1 d g 0 0 NM\nM2 d 0 0 0 OTHER\n.model NM NMOS LEVEL=1\n.model OTHER NMOS LEVEL=1\n.end\n";
        for (devices, models) in [
            (vec!["m2".into()], Vec::new()),
            (Vec::new(), vec!["other".into()]),
        ] {
            let config = SoaRunConfig {
                stop_time: 1e-8,
                step_time: 1e-9,
                observation: SoaObservationConfig {
                    devices,
                    models,
                    ..Default::default()
                },
                ..Default::default()
            };
            let result = run_soa_analysis_with_config_and_source_path_and_abort(
                deck, &config, None, &NoAbort,
            )
            .unwrap();
            assert_eq!(result.evaluations.len(), 2);
            assert!(
                result
                    .evaluations
                    .iter()
                    .all(|rule| rule.device_id.eq_ignore_ascii_case("M2"))
            );
        }
        let config = SoaRunConfig {
            observation: SoaObservationConfig {
                devices: vec!["M1".into(), "M3".into()],
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(
            run_soa_analysis_with_config_and_source_path_and_abort(deck, &config, None, &NoAbort)
                .unwrap_err()
                .to_string()
                .contains("M3")
        );
    }

    #[test]
    fn soa_explicit_initial_conditions_reach_the_transient_startup() {
        let deck = "SOA startup\nVd d 0 1\nR1 g 0 1Meg\nC1 g 0 1u IC=2.5\nM1 d g 0 0 NM\n.model NM NMOS LEVEL=1\n.end\n";
        let gate = |uic| {
            let config = SoaRunConfig {
                stop_time: 1e-8,
                step_time: 1e-9,
                observation: SoaObservationConfig {
                    use_initial_conditions: uic,
                    ..Default::default()
                },
                ..Default::default()
            };
            run_soa_analysis_with_config_and_source_path_and_abort(deck, &config, None, &NoAbort)
                .unwrap()
                .evaluations
                .into_iter()
                .find(|rule| rule.parameter == SoAParameter::Vgs)
                .unwrap()
                .worst_actual_value
        };
        assert!(gate(false) < 1e-10);
        assert!((gate(true) - 2.5).abs() < 1e-6);
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

    #[test]
    fn soa_requires_an_enabled_rule_applicable_to_the_deck() {
        let netlist = rspice_core::Netlist::parse(
            "soa rule routing\n\
             M1 d g s 0 NM\n\
             .model NM NMOS\n\
             .end\n",
        )
        .expect("MOS deck parses");
        let bjt_only = SoaRunConfig {
            check_vgs_max: false,
            check_vds_max: false,
            check_vbe_max: true,
            check_vce_max: true,
            ..SoaRunConfig::default()
        };
        let count = rules::resolve(
            &netlist.elements,
            &bjt_only,
            &Default::default(),
            &Default::default(),
            &NoAbort,
        )
        .expect("rule registration completes")
        .len();

        assert_eq!(count, 0);
    }
}
