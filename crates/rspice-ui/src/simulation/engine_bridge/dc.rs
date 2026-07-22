use std::collections::HashMap;

use rspice_core::abort_signal::AbortSignal;

use super::{EngineBridge, ensure_not_aborted};
use crate::simulation::config::DcSweepConfig;
use crate::simulation::dialog::{
    OpAccuracy, OpConfig, OpHomotopy, OpInitialGuess, OpNodeInitialization, OpSaveDevice,
};
use crate::simulation::results::{DcOpResult, SimulationResult, WaveformData};
use crate::simulation::runner::SimulationError;

impl EngineBridge {
    /// Run DC operating point analysis.
    pub(super) fn run_dc_op(
        &self,
        netlist: &rspice_core::Netlist,
        config: &OpConfig,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        ensure_not_aborted(abort)?;
        config
            .validate_for_execution()
            .map_err(SimulationError::InvalidConfig)?;
        let validated_startup_directives = netlist.startup_directives().len();
        let mut execution_netlist = netlist.clone();
        if config.node_initialization == OpNodeInitialization::ValidateOnly {
            rspice_core::netlist::validate_startup_directives_with_abort(
                &mut execution_netlist,
                abort,
            )
            .map_err(|error| {
                if error.is_aborted() {
                    SimulationError::Aborted
                } else {
                    SimulationError::InvalidConfig(format!(
                        "startup directive validation failed: {error}"
                    ))
                }
            })?;
        }
        if let (Some(supply), Some(nominal)) = (
            config.run_point.supply_voltage,
            config.run_point.nominal_supply_voltage,
        ) {
            crate::services::simulation_runner::apply_voltage_corner(
                &mut execution_netlist,
                supply,
                nominal,
                abort,
            )
            .map_err(|error| {
                if error.is_aborted() {
                    SimulationError::Aborted
                } else {
                    SimulationError::InvalidConfig(format!(
                        "operating-point PVT supply application failed: {error}"
                    ))
                }
            })?;
        }
        apply_startup_policy(&mut execution_netlist, config);
        let engine = configured_op_engine(self, &execution_netlist, config)?;
        if config.initial_guess == OpInitialGuess::PreviousConverged {
            let previous = config.previous_state.as_ref().ok_or_else(|| {
                SimulationError::InvalidConfig(
                    "previous operating-point state was not bound to this request".to_owned(),
                )
            })?;
            let source = execution_netlist.source_text.as_deref().ok_or_else(|| {
                SimulationError::InvalidConfig(
                    "previous operating-point startup requires exact executable source bytes"
                        .to_owned(),
                )
            })?;
            let effective_source_digest =
                crate::simulation::execution::operating_point_effective_source_digest(
                    source,
                    config.run_point,
                );
            if previous.source_content_digest != effective_source_digest {
                return Err(SimulationError::InvalidConfig(
                    "previous operating-point state belongs to a different effective PVT source"
                        .to_owned(),
                ));
            }
            let circuit = engine
                .build_circuit_with_abort(&execution_netlist, abort)
                .map_err(|error| self.translate_error(error))?;
            if previous.node_names != circuit.node_names_sorted()
                || previous.branch_names != circuit.branch_names_sorted()
                || previous.solution.len() != circuit.matrix_size()
            {
                return Err(SimulationError::InvalidConfig(
                    "previous operating-point state MNA basis does not match the elaborated circuit"
                        .to_owned(),
                ));
            }
        }
        let run = match (config.initial_guess, config.node_initialization) {
            (_, OpNodeInitialization::ForceIcValues) => {
                engine.run_dc_op_forced_ic_with_report_and_abort(&execution_netlist, abort)
            }
            (OpInitialGuess::PreviousConverged, _) => match config.previous_state.as_ref() {
                Some(previous) => engine.run_dc_op_with_previous_solution_and_report_and_abort(
                    &execution_netlist,
                    &previous.solution,
                    abort,
                ),
                None => Err(rspice_core::SimulationError::Circuit(
                    "previous operating-point state was not bound to this request".to_owned(),
                )),
            },
            (OpInitialGuess::ZeroState, _) => {
                engine.run_dc_op_from_zero_with_report_and_abort(&execution_netlist, abort)
            }
            (OpInitialGuess::Automatic | OpInitialGuess::UserNodeVoltages, _) => {
                engine.run_dc_op_with_report_and_abort(&execution_netlist, abort)
            }
        };
        let (core_result, device_report) = run.map_err(|e| self.translate_error(e))?;

        let mut result = convert_dc_result(&core_result, abort)?;
        result.configuration = config.clone();
        result.validated_startup_directives = validated_startup_directives;
        ensure_not_aborted(abort)?;
        let device_report = filter_device_report(device_report, config);
        if !device_report.is_empty() {
            result.device_report = Some(device_report);
        }
        Ok(SimulationResult::DcOp(result))
    }

    /// Run DC sweep analysis.
    pub(super) fn run_dc_sweep(
        &self,
        netlist: &rspice_core::Netlist,
        config: &DcSweepConfig,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        ensure_not_aborted(abort)?;
        let engine = self.engine_for_netlist(netlist);
        let nested_cfg = nested_dc_sweep_config(config);
        ensure_not_aborted(abort)?;
        let nested_cfg = nested_cfg?;
        let mut sweep_values = Vec::new();
        let mut waveforms = HashMap::new();

        if let Some((source2, start2, stop2, step2)) = nested_cfg {
            let sweep2 =
                rspice_core::analysis::DcSweep::new(source2.to_string(), start2, stop2, step2);
            let sweep2_values = sweep2.points();
            ensure_not_aborted(abort)?;
            if sweep2_values.is_empty() {
                return Err(SimulationError::InvalidConfig(
                    "Nested DC secondary sweep produced no points".to_string(),
                ));
            }

            for &sweep2_value in &sweep2_values {
                ensure_not_aborted(abort)?;
                let mut nested_netlist = netlist.clone();
                set_dc_source_value(&mut nested_netlist, source2, sweep2_value, abort)?;

                let sweep_results = engine
                    .run_dc_sweep_with_abort(
                        &nested_netlist,
                        &config.source,
                        config.start,
                        config.stop,
                        config.step,
                        abort,
                    )
                    .map_err(|e| self.translate_error(e))?;

                if sweep_results.is_empty() {
                    continue;
                }

                if sweep_values.is_empty() {
                    sweep_values.reserve(sweep_results.len());
                    for (value, _) in &sweep_results {
                        ensure_not_aborted(abort)?;
                        sweep_values.push(*value);
                    }
                }

                if let Some((_, first_result)) = sweep_results.first() {
                    for (node_idx, node_name) in first_result.node_names.iter().enumerate() {
                        ensure_not_aborted(abort)?;
                        if node_idx == 0 {
                            continue;
                        }
                        let mut voltages = Vec::with_capacity(sweep_results.len());
                        for (_, result) in &sweep_results {
                            ensure_not_aborted(abort)?;
                            voltages
                                .push(result.node_voltages.get(node_idx).copied().unwrap_or(0.0));
                        }
                        let trace_name = format!("{} [{}={:.6}]", node_name, source2, sweep2_value);
                        waveforms.insert(
                            trace_name.clone(),
                            WaveformData::new_time_domain(
                                trace_name,
                                sweep_values.clone(),
                                voltages,
                            ),
                        );
                    }
                }
            }
        } else {
            let sweep_results = engine
                .run_dc_sweep_with_abort(
                    netlist,
                    &config.source,
                    config.start,
                    config.stop,
                    config.step,
                    abort,
                )
                .map_err(|e| self.translate_error(e))?;

            sweep_values.reserve(sweep_results.len());
            for (value, _) in &sweep_results {
                ensure_not_aborted(abort)?;
                sweep_values.push(*value);
            }

            if let Some((_, first_result)) = sweep_results.first() {
                for (i, name) in first_result.node_names.iter().enumerate() {
                    ensure_not_aborted(abort)?;
                    if i == 0 {
                        continue;
                    }
                    let mut voltages = Vec::with_capacity(sweep_results.len());
                    for (_, result) in &sweep_results {
                        ensure_not_aborted(abort)?;
                        voltages.push(result.node_voltages.get(i).copied().unwrap_or(0.0));
                    }

                    waveforms.insert(
                        name.clone(),
                        WaveformData::new_time_domain(name, sweep_values.clone(), voltages),
                    );
                }
            }
        }

        let measurements =
            super::measure::evaluate_measurements(netlist, "DC", &sweep_values, &waveforms, abort)?;
        Ok(SimulationResult::DcSweep {
            sweep_var: config.source.clone(),
            sweep_values,
            waveforms,
            measurements,
        })
    }
}

fn configured_op_engine(
    bridge: &EngineBridge,
    netlist: &rspice_core::Netlist,
    config: &OpConfig,
) -> Result<rspice_core::Engine, SimulationError> {
    let (preset, reltol, abstol, max_iterations) = match config.accuracy {
        OpAccuracy::Fast => (rspice_core::ConvergencePreset::Fast, None, None, Some(50)),
        OpAccuracy::Balanced => (
            rspice_core::ConvergencePreset::Default,
            None,
            None,
            Some(150),
        ),
        OpAccuracy::Accurate => (
            rspice_core::ConvergencePreset::Default,
            Some(1.0e-8),
            Some(1.0e-14),
            Some(250),
        ),
        OpAccuracy::Robust => (
            rspice_core::ConvergencePreset::Robust,
            None,
            None,
            Some(500),
        ),
    };
    let overrides = rspice_core::SimulationConfigOverrides {
        temperature_kelvin: Some(config.temperature_celsius + 273.15),
        convergence_preset: Some(preset),
        reltol,
        abstol,
        max_iterations,
        ..Default::default()
    };
    let mut resolved = rspice_core::resolve_simulation_config(
        bridge.engine.config(),
        Some(&netlist.options),
        &overrides,
    );
    let convergence = &mut resolved.convergence_config;
    match config.homotopy {
        OpHomotopy::Adaptive => {}
        OpHomotopy::SourceStepping => {
            convergence.source_stepping = true;
            convergence.gmin_stepping = false;
            convergence.pseudo_transient = false;
            convergence.arc_length = false;
            convergence.nonlinear_continuation =
                Some(rspice_core::netlist::NonlinearContinuationMode::SimultaneousSourceStep);
        }
        OpHomotopy::GminStepping => {
            convergence.source_stepping = false;
            convergence.gmin_stepping = true;
            convergence.pseudo_transient = false;
            convergence.arc_length = false;
            convergence.nonlinear_continuation = None;
        }
        OpHomotopy::PseudoTransient => {
            convergence.source_stepping = false;
            convergence.gmin_stepping = false;
            convergence.pseudo_transient = true;
            convergence.arc_length = false;
            convergence.nonlinear_continuation = None;
        }
        OpHomotopy::None => {
            convergence.source_stepping = false;
            convergence.gmin_stepping = false;
            convergence.pseudo_transient = false;
            convergence.arc_length = false;
            convergence.nonlinear_continuation = None;
        }
    }
    rspice_core::Engine::try_new_with_resolved_config(resolved)
        .map_err(|error| SimulationError::InvalidConfig(error.to_string()))
}

fn apply_startup_policy(netlist: &mut rspice_core::Netlist, config: &OpConfig) {
    let keep_ic = !matches!(config.initial_guess, OpInitialGuess::ZeroState)
        && !matches!(
            config.node_initialization,
            OpNodeInitialization::IgnoreIcAndNodeset | OpNodeInitialization::ValidateOnly
        );
    let keep_nodesets = config.initial_guess == OpInitialGuess::Automatic
        && config.node_initialization == OpNodeInitialization::UseIcAndNodeset;

    netlist.retain_startup_kinds(keep_ic, keep_nodesets);
}

fn filter_device_report(
    mut report: rspice_core::circuit::DeviceOpReport,
    config: &OpConfig,
) -> rspice_core::circuit::DeviceOpReport {
    if config.save_device_op == OpSaveDevice::Disabled
        || (config.save_device_op == OpSaveDevice::FinalPointOnly && !config.run_point.is_final())
    {
        report.entries.clear();
    }
    report
}

fn convert_dc_result(
    core_result: &rspice_core::SimulationResult,
    abort: &dyn AbortSignal,
) -> Result<DcOpResult, SimulationError> {
    let mut result = DcOpResult::default();

    result.mna_node_names = core_result.node_names.iter().skip(1).cloned().collect();
    result.mna_branch_names = core_result.branch_names.clone();
    result.mna_solution = core_result
        .node_voltages
        .iter()
        .skip(1)
        .chain(&core_result.branch_currents)
        .copied()
        .collect();

    for (i, &voltage) in core_result.node_voltages.iter().enumerate() {
        ensure_not_aborted(abort)?;
        if i > 0 {
            let name = core_result
                .node_names
                .get(i)
                .cloned()
                .unwrap_or_else(|| format!("{}", i));
            result.node_voltages.insert(name, voltage);
        }
    }

    for (i, &current) in core_result.branch_currents.iter().enumerate() {
        ensure_not_aborted(abort)?;
        let name = dc_branch_waveform_name(core_result, i);
        result.branch_currents.insert(name, current);
    }

    Ok(result)
}

fn dc_branch_waveform_name(result: &rspice_core::SimulationResult, branch_idx: usize) -> String {
    result
        .branch_names
        .get(branch_idx)
        .filter(|name| !name.is_empty())
        .cloned()
        .unwrap_or_else(|| (branch_idx + 1).to_string())
}

fn nested_dc_sweep_config(
    config: &DcSweepConfig,
) -> Result<Option<(&str, f64, f64, f64)>, SimulationError> {
    match (&config.source2, config.start2, config.stop2, config.step2) {
        (None, None, None, None) => Ok(None),
        (Some(source2), Some(start2), Some(stop2), Some(step2)) => {
            Ok(Some((source2.as_str(), start2, stop2, step2)))
        }
        _ => Err(SimulationError::InvalidConfig(
            "Nested DC sweep requires source2/start2/stop2/step2".to_string(),
        )),
    }
}

fn set_dc_source_value(
    netlist: &mut rspice_core::Netlist,
    source_name: &str,
    value: f64,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    ensure_not_aborted(abort)?;
    if source_name.trim().is_empty() {
        return Err(SimulationError::InvalidConfig(
            "DC sweep source name cannot be empty".to_string(),
        ));
    }

    for element in &mut netlist.elements {
        ensure_not_aborted(abort)?;
        if !element.name.eq_ignore_ascii_case(source_name) {
            continue;
        }
        if let rspice_core::netlist::ElementKind::VoltageSource(spec) = &mut element.kind {
            if set_source_spec_dc(spec, value) {
                return Ok(());
            }
            return Err(SimulationError::InvalidConfig(format!(
                "Source '{}' is not a DC or DC/AC voltage source",
                source_name
            )));
        }
    }

    Err(SimulationError::InvalidConfig(format!(
        "Source '{}' not found in netlist",
        source_name
    )))
}

fn set_source_spec_dc(spec: &mut rspice_core::netlist::SourceSpec, value: f64) -> bool {
    match spec {
        rspice_core::netlist::SourceSpec::Dc(v) => {
            *v = value;
            true
        }
        rspice_core::netlist::SourceSpec::DcAc { dc_value, .. } => {
            *dc_value = value;
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod operating_point_contract_tests {
    use super::*;
    use crate::product::ContentDigest;
    use crate::simulation::config::AnalysisConfig;
    use crate::simulation::dialog::{
        OpInitialGuess, OpNodeInitialization, OpPreviousState, OpRunPointContext,
    };

    const DIVIDER: &str =
        "divider\nV1 in 0 10\nR1 in out 1k\nR2 out 0 1k\n.ic V(out)=2\n.op\n.end\n";

    #[test]
    fn force_ic_reaches_the_ui_as_an_exact_hard_constrained_result() {
        let config = OpConfig {
            node_initialization: OpNodeInitialization::ForceIcValues,
            ..OpConfig::default()
        };
        let result = EngineBridge::new()
            .run(&AnalysisConfig::DcOp(config), DIVIDER)
            .expect("forced OP");
        let SimulationResult::DcOp(result) = result else {
            panic!("OP result")
        };
        assert!((result.voltage("out").unwrap() - 2.0).abs() <= 1.0e-10);
        assert_eq!(
            result.mna_solution.len(),
            result.mna_node_names.len() + result.mna_branch_names.len()
        );
        assert!(result.branch_currents.contains_key("V1"));
        assert!(!result.branch_currents.contains_key("I(V1)"));
    }

    #[test]
    fn identity_bound_previous_state_round_trips_into_the_core_seed() {
        let bridge = EngineBridge::new();
        let first = bridge
            .run(&AnalysisConfig::dc_op(), DIVIDER)
            .expect("baseline OP");
        let SimulationResult::DcOp(first) = first else {
            panic!("OP result")
        };
        let effective_source_digest =
            crate::simulation::execution::operating_point_effective_source_digest(
                DIVIDER,
                OpRunPointContext::default(),
            );
        let config = OpConfig {
            initial_guess: OpInitialGuess::PreviousConverged,
            node_initialization: OpNodeInitialization::IgnoreIcAndNodeset,
            previous_state: Some(OpPreviousState {
                source_content_digest: effective_source_digest,
                producer_snapshot_digest: ContentDigest::from_bytes([2; 32]),
                producer_result_digest: ContentDigest::from_bytes([3; 32]),
                node_names: first.mna_node_names.clone(),
                branch_names: first.mna_branch_names.clone(),
                solution: first.mna_solution.clone(),
            }),
            ..OpConfig::default()
        };
        let second = bridge
            .run(&AnalysisConfig::DcOp(config), DIVIDER)
            .expect("previous-state OP");
        let SimulationResult::DcOp(second) = second else {
            panic!("OP result")
        };
        assert_eq!(second.mna_node_names, first.mna_node_names);
        assert_eq!(second.mna_branch_names, first.mna_branch_names);
        assert_eq!(second.mna_solution.len(), first.mna_solution.len());
    }

    #[test]
    fn final_point_only_drops_nonfinal_reports_and_keeps_the_final_report() {
        let deck = "diode\nV1 in 0 0.7\nD1 in 0 DTEST\n.model DTEST D\n.op\n.end\n";
        let run = |run_point| {
            let config = OpConfig {
                save_device_op: OpSaveDevice::FinalPointOnly,
                run_point,
                ..OpConfig::default()
            };
            EngineBridge::new()
                .run(&AnalysisConfig::DcOp(config), deck)
                .expect("OP")
        };
        let SimulationResult::DcOp(nonfinal) = run(OpRunPointContext {
            index: 0,
            count: 2,
            ..OpRunPointContext::default()
        }) else {
            panic!("OP result")
        };
        let SimulationResult::DcOp(final_point) = run(OpRunPointContext {
            index: 1,
            count: 2,
            ..OpRunPointContext::default()
        }) else {
            panic!("OP result")
        };
        assert!(nonfinal.device_report.is_none());
        assert!(final_point.device_report.is_some());
    }
}
