//! DC operating point and DC sweep over the engine bridge.

use std::collections::{HashMap, HashSet};

use rspice_core::abort_signal::AbortSignal;

use super::{EngineBridge, ensure_not_aborted};
use crate::simulation::config::DcSweepConfig;
use crate::simulation::dialog::{OpConfig, OpInitialGuess, OpNodeInitialization, OpSaveDevice};
use crate::simulation::results::{DcOpResult, SimulationResult, WaveformData};
use crate::simulation::runner::SimulationError;

/// What the two branches of a retracing sweep are called.
///
/// The suffix is the trace's own, following the `[key=value]` shape a nested
/// sweep already uses for the same reason: every trace in a DC result is drawn
/// against one shared ascending axis, so the direction a branch was travelled
/// can only live in its name. A reader who sees `V(out) [reverse]` beside
/// `V(out) [forward]` is being told which way the source was moving, which is
/// the only thing that distinguishes the two curves.
const HYSTERESIS_FORWARD: &str = "forward";
const HYSTERESIS_REVERSE: &str = "reverse";

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
                &config.run_point.supply_source_names,
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
                    config.run_point.clone(),
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
        Ok(SimulationResult::DcOp(Box::new(result)))
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
        let mut measurements = Vec::new();

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

                validate_dc_sweep_results(&sweep_results, "nested DC sweep")?;
                ensure_not_aborted(abort)?;
                let mut point_measurements = rspice_core::analysis::evaluate_dc_measurements(
                    &nested_netlist,
                    &sweep_results,
                );
                for measurement in &mut point_measurements {
                    measurement.name =
                        format!("{} [{}={:.16e}]", measurement.name, source2, sweep2_value);
                }
                measurements.extend(point_measurements);
                ensure_not_aborted(abort)?;

                if sweep_values.is_empty() {
                    sweep_values.reserve(sweep_results.len());
                    for (value, _) in &sweep_results {
                        ensure_not_aborted(abort)?;
                        sweep_values.push(*value);
                    }
                } else if sweep_results
                    .iter()
                    .map(|(value, _)| *value)
                    .ne(sweep_values.iter().copied())
                {
                    return Err(SimulationError::SolverError(
                        "Nested DC solves produced inconsistent primary sweep axes".to_owned(),
                    ));
                }

                let first_result = &sweep_results[0].1;
                for (node_idx, node_name) in first_result.node_names.iter().enumerate() {
                    ensure_not_aborted(abort)?;
                    if node_idx == 0 {
                        continue;
                    }
                    let mut voltages = Vec::with_capacity(sweep_results.len());
                    for (_, result) in &sweep_results {
                        ensure_not_aborted(abort)?;
                        voltages.push(result.node_voltages[node_idx]);
                    }
                    let trace_name = format!("{} [{}={:.6}]", node_name, source2, sweep2_value);
                    waveforms.insert(
                        trace_name.clone(),
                        WaveformData::new_time_domain(trace_name, sweep_values.clone(), voltages),
                    );
                }
                for (branch_idx, branch_name) in first_result.branch_names.iter().enumerate() {
                    ensure_not_aborted(abort)?;
                    let mut currents = Vec::with_capacity(sweep_results.len());
                    for (_, result) in &sweep_results {
                        ensure_not_aborted(abort)?;
                        currents.push(result.branch_currents[branch_idx]);
                    }
                    let trace_name =
                        format!("I({}) [{}={:.6}]", branch_name, source2, sweep2_value);
                    waveforms.insert(
                        trace_name.clone(),
                        WaveformData::new_time_domain_in_unit(
                            trace_name,
                            sweep_values.clone(),
                            currents,
                            "A",
                        ),
                    );
                }
            }
        } else if config.hysteresis {
            // One solve, not two. The engine steps an explicit value list in
            // the order given, carrying the previous point's solution and the
            // devices' own state into the next, and never rebuilding the
            // circuit — so the reverse branch genuinely continues from where
            // the forward branch finished. That is the whole content of a
            // hysteresis measurement: two sequential sweeps would each start
            // cold and could not disagree.
            let turnaround = config.retrace_turnaround();
            let spec = rspice_core::netlist::DcSweepSpec::list(config.retrace_points());
            let point_results = engine
                .run_dc_sweep2_spec_with_report_and_abort(
                    netlist,
                    &config.source,
                    &spec,
                    None,
                    abort,
                )
                .map_err(|e| self.translate_error(e))?;
            let sweep_results = point_results
                .into_iter()
                .map(|point| (point.sweep_value, point.result))
                .collect::<Vec<_>>();

            validate_dc_sweep_results(&sweep_results, "bidirectional DC sweep")?;
            ensure_not_aborted(abort)?;
            if sweep_results.len() != turnaround * 2 + 1 {
                return Err(SimulationError::SolverError(format!(
                    "bidirectional DC sweep solved {} points for a {}-point retrace",
                    sweep_results.len(),
                    turnaround * 2 + 1
                )));
            }

            // The turnaround belongs to both branches: it is the last forward
            // point and the first reverse one, so each slice includes it and
            // the two traces meet rather than leaving a one-step gap.
            let forward = &sweep_results[..=turnaround];
            let reverse = &sweep_results[turnaround..];

            sweep_values.extend(forward.iter().map(|(value, _)| *value));
            // Both branches are reported against this one ascending axis. The
            // reverse branch is *travelled* the other way, and its samples are
            // re-ordered to match — the direction is what the branch is named
            // for, not something the x column can carry, because every trace in
            // a DC result shares one axis by construction.
            let reverse_axis = reverse
                .iter()
                .rev()
                .map(|(value, _)| *value)
                .collect::<Vec<_>>();
            if reverse_axis != sweep_values {
                return Err(SimulationError::SolverError(
                    "bidirectional DC sweep branches did not visit the same source values"
                        .to_owned(),
                ));
            }

            for (branch, results) in [(HYSTERESIS_FORWARD, forward), (HYSTERESIS_REVERSE, reverse)]
            {
                ensure_not_aborted(abort)?;
                let mut branch_measurements =
                    rspice_core::analysis::evaluate_dc_measurements(netlist, results);
                for measurement in &mut branch_measurements {
                    measurement.name = format!("{} [{branch}]", measurement.name);
                }
                measurements.extend(branch_measurements);

                // Forward reads in traversal order; reverse is travelled from
                // the turnaround down, so reading it backwards puts its samples
                // under the ascending axis above.
                let ordered = |extract: &dyn Fn(&rspice_core::SimulationResult) -> f64| {
                    let mut values = results.iter().map(|(_, r)| extract(r)).collect::<Vec<_>>();
                    if branch == HYSTERESIS_REVERSE {
                        values.reverse();
                    }
                    values
                };

                let first_result = &results[0].1;
                for (index, node_name) in first_result.node_names.iter().enumerate() {
                    ensure_not_aborted(abort)?;
                    if index == 0 {
                        continue;
                    }
                    let trace_name = format!("{node_name} [{branch}]");
                    let voltages = ordered(&|result| result.node_voltages[index]);
                    waveforms.insert(
                        trace_name.clone(),
                        WaveformData::new_time_domain(trace_name, sweep_values.clone(), voltages),
                    );
                }
                for (index, branch_name) in first_result.branch_names.iter().enumerate() {
                    ensure_not_aborted(abort)?;
                    let trace_name = format!("I({branch_name}) [{branch}]");
                    let currents = ordered(&|result| result.branch_currents[index]);
                    waveforms.insert(
                        trace_name.clone(),
                        WaveformData::new_time_domain_in_unit(
                            trace_name,
                            sweep_values.clone(),
                            currents,
                            "A",
                        ),
                    );
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

            validate_dc_sweep_results(&sweep_results, "DC sweep")?;
            ensure_not_aborted(abort)?;
            measurements = rspice_core::analysis::evaluate_dc_measurements(netlist, &sweep_results);
            ensure_not_aborted(abort)?;

            sweep_values.reserve(sweep_results.len());
            for (value, _) in &sweep_results {
                ensure_not_aborted(abort)?;
                sweep_values.push(*value);
            }

            let first_result = &sweep_results[0].1;
            for (i, name) in first_result.node_names.iter().enumerate() {
                ensure_not_aborted(abort)?;
                if i == 0 {
                    continue;
                }
                let mut voltages = Vec::with_capacity(sweep_results.len());
                for (_, result) in &sweep_results {
                    ensure_not_aborted(abort)?;
                    voltages.push(result.node_voltages[i]);
                }

                waveforms.insert(
                    name.clone(),
                    WaveformData::new_time_domain(name, sweep_values.clone(), voltages),
                );
            }
            for (branch_idx, branch_name) in first_result.branch_names.iter().enumerate() {
                ensure_not_aborted(abort)?;
                let mut currents = Vec::with_capacity(sweep_results.len());
                for (_, result) in &sweep_results {
                    ensure_not_aborted(abort)?;
                    currents.push(result.branch_currents[branch_idx]);
                }
                let trace_name = format!("I({branch_name})");
                waveforms.insert(
                    trace_name.clone(),
                    WaveformData::new_time_domain_in_unit(
                        trace_name,
                        sweep_values.clone(),
                        currents,
                        "A",
                    ),
                );
            }
        }

        Ok(SimulationResult::DcSweep {
            sweep_var: config.source.clone(),
            sweep_values,
            waveforms,
            measurements,
        })
    }
}

fn validate_dc_sweep_results(
    results: &[(f64, rspice_core::SimulationResult)],
    context: &str,
) -> Result<(), SimulationError> {
    let Some((_, first)) = results.first() else {
        return Err(SimulationError::SolverError(format!(
            "{context} produced no solved points"
        )));
    };
    if first.node_names.len() != first.node_voltages.len() {
        return Err(SimulationError::SolverError(format!(
            "{context} reference point has {} node names but {} voltages",
            first.node_names.len(),
            first.node_voltages.len()
        )));
    }
    if first.branch_names.len() != first.branch_currents.len() {
        return Err(SimulationError::SolverError(format!(
            "{context} reference point has {} branch names but {} currents",
            first.branch_names.len(),
            first.branch_currents.len()
        )));
    }
    validate_dc_signal_identities(&first.node_names, &first.branch_names, context)?;
    for (point_index, (axis, point)) in results.iter().enumerate() {
        if !axis.is_finite() {
            return Err(SimulationError::SolverError(format!(
                "{context} point {} has a non-finite sweep coordinate",
                point_index + 1
            )));
        }
        if point.node_names != first.node_names || point.branch_names != first.branch_names {
            return Err(SimulationError::SolverError(format!(
                "{context} point {} changed the solved MNA basis",
                point_index + 1
            )));
        }
        if point.node_voltages.len() != first.node_voltages.len()
            || point.branch_currents.len() != first.branch_currents.len()
        {
            return Err(SimulationError::SolverError(format!(
                "{context} point {} returned an inconsistent solution shape",
                point_index + 1
            )));
        }
        if point
            .node_voltages
            .iter()
            .chain(&point.branch_currents)
            .any(|value| !value.is_finite())
        {
            return Err(SimulationError::SolverError(format!(
                "{context} point {} returned a non-finite solution value",
                point_index + 1
            )));
        }
    }
    Ok(())
}

/// The configuration the operating-point engine is constructed from.
///
/// Separated from [`configured_op_engine`] so it can be read without building
/// an engine. It is the only complete statement of what an `.OP` solve
/// resolves to: the deck's `.OPTIONS` are only the second of four layers, and
/// the two that follow — the accuracy tier and the homotopy choice — assign
/// fields the deck may also have stated. A gate or a ledger that stopped at
/// `resolve_simulation_config` would be reporting a policy no solve uses.
pub(in crate::simulation) fn resolved_op_config(
    base: &rspice_core::SimulationConfig,
    netlist_options: &rspice_core::netlist::SimulationOptions,
    config: &OpConfig,
) -> rspice_core::SimulationConfig {
    let overrides = rspice_core::SimulationConfigOverrides {
        temperature_kelvin: Some(config.temperature_celsius + 273.15),
        ..Default::default()
    };
    let mut resolved =
        rspice_core::resolve_simulation_config(base, Some(netlist_options), &overrides);
    // The accuracy tier applies last, on top of the fully resolved policy, so
    // "only tightens" is measured against what the reader would otherwise get.
    config.accuracy.solver_policy().apply(&mut resolved);
    // And the homotopy control after the tier, because it is the more
    // specific statement about the same aids.
    config.homotopy.apply(&mut resolved);
    resolved
}

fn configured_op_engine(
    bridge: &EngineBridge,
    netlist: &rspice_core::Netlist,
    config: &OpConfig,
) -> Result<rspice_core::Engine, SimulationError> {
    let resolved = resolved_op_config(bridge.engine.config(), &netlist.options, config);
    // Through the bridge's own engine, not `Engine::try_new_with_resolved_config`:
    // the operating point is the one analysis that records which conductor a
    // failed solve named (`engine/core.rs:1309,1335`,
    // `engine/convergence/solve.rs:163,171,877,1330`), and a freshly
    // constructed engine drops that record with the temporary, leaving
    // `translate_error` nothing to attribute.
    bridge
        .engine
        .try_resolved_with_config(resolved)
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
    if core_result.node_names.len() != core_result.node_voltages.len() {
        return Err(SimulationError::SolverError(format!(
            "DC operating point returned {} node names but {} voltages",
            core_result.node_names.len(),
            core_result.node_voltages.len()
        )));
    }
    if core_result.branch_names.len() != core_result.branch_currents.len() {
        return Err(SimulationError::SolverError(format!(
            "DC operating point returned {} branch names but {} currents",
            core_result.branch_names.len(),
            core_result.branch_currents.len()
        )));
    }
    if core_result
        .node_names
        .iter()
        .chain(&core_result.branch_names)
        .any(|name| name.trim().is_empty())
    {
        return Err(SimulationError::SolverError(
            "DC operating point returned an unnamed MNA quantity".to_owned(),
        ));
    }
    validate_dc_signal_identities(
        &core_result.node_names,
        &core_result.branch_names,
        "DC operating point",
    )?;
    if core_result
        .node_voltages
        .iter()
        .chain(&core_result.branch_currents)
        .any(|value| !value.is_finite())
    {
        return Err(SimulationError::SolverError(
            "DC operating point returned a non-finite solution value".to_owned(),
        ));
    }
    let mut result = DcOpResult {
        mna_node_names: core_result.node_names.iter().skip(1).cloned().collect(),
        mna_branch_names: core_result.branch_names.clone(),
        mna_solution: core_result
            .node_voltages
            .iter()
            .skip(1)
            .chain(&core_result.branch_currents)
            .copied()
            .collect(),
        ..Default::default()
    };

    for (i, &voltage) in core_result.node_voltages.iter().enumerate() {
        ensure_not_aborted(abort)?;
        if i > 0 {
            let name = core_result.node_names[i].clone();
            result.node_voltages.insert(name, voltage);
        }
    }

    for (i, &current) in core_result.branch_currents.iter().enumerate() {
        ensure_not_aborted(abort)?;
        let name = core_result.branch_names[i].clone();
        result.branch_currents.insert(name, current);
    }

    Ok(result)
}

fn validate_dc_signal_identities(
    node_names: &[String],
    branch_names: &[String],
    context: &str,
) -> Result<(), SimulationError> {
    if node_names
        .iter()
        .chain(branch_names)
        .any(|name| name.trim().is_empty())
    {
        return Err(SimulationError::SolverError(format!(
            "{context} returned an unnamed signal"
        )));
    }
    let mut identities = HashSet::with_capacity(node_names.len() + branch_names.len());
    if node_names
        .iter()
        .map(|name| format!("v({})", name.trim().to_ascii_lowercase()))
        .chain(
            branch_names
                .iter()
                .map(|name| format!("i({})", name.trim().to_ascii_lowercase())),
        )
        .any(|identity| !identities.insert(identity))
    {
        return Err(SimulationError::SolverError(format!(
            "{context} returned duplicate signal identities"
        )));
    }
    Ok(())
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
    fn dc_sweep_retains_branch_currents_with_current_units() {
        let deck = "sweep\nVin in 0 0\nR1 in 0 1k\n.dc Vin 0 1 0.5\n.end\n";
        let result = EngineBridge::new()
            .run(
                &AnalysisConfig::DcSweep(DcSweepConfig {
                    source: "Vin".to_owned(),
                    start: 0.0,
                    stop: 1.0,
                    step: 0.5,
                    ..DcSweepConfig::default()
                }),
                deck,
            )
            .expect("DC sweep");
        let SimulationResult::DcSweep {
            sweep_values,
            waveforms,
            ..
        } = result
        else {
            panic!("DC sweep result")
        };

        assert_eq!(sweep_values, vec![0.0, 0.5, 1.0]);
        let current = waveforms
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("I(Vin)"))
            .map(|(_, waveform)| waveform)
            .expect("source branch current is retained");
        assert_eq!(current.y_unit, "A");
        assert_eq!(current.y_values.len(), sweep_values.len());
        assert!(current.y_values.iter().all(|value| value.is_finite()));
    }

    /// A retracing sweep reports each signal twice, named for the direction it
    /// was travelled, over the one ascending axis every DC trace shares.
    ///
    /// The reverse branch is solved from the turnaround downwards, so its
    /// samples arrive in descending source order and are re-ordered to sit
    /// under that axis. This circuit is linear, so the two branches must agree
    /// point for point — which is exactly what catches the re-ordering being
    /// dropped: an un-reversed reverse branch would descend while the axis
    /// ascends, and the comparison below would fail.
    #[test]
    fn a_bidirectional_sweep_reports_both_branches_against_one_axis() {
        let deck = "retrace\nVin in 0 0\nR1 in out 1k\nR2 out 0 1k\n.dc Vin 0 1 0.5\n.end\n";
        let result = EngineBridge::new()
            .run(
                &AnalysisConfig::DcSweep(DcSweepConfig {
                    source: "Vin".to_owned(),
                    start: 0.0,
                    stop: 1.0,
                    step: 0.5,
                    hysteresis: true,
                    ..DcSweepConfig::default()
                }),
                deck,
            )
            .expect("bidirectional DC sweep");
        let SimulationResult::DcSweep {
            sweep_values,
            waveforms,
            ..
        } = result
        else {
            panic!("DC sweep result")
        };

        // One ascending axis, the forward branch's, not the doubled retrace.
        assert_eq!(sweep_values, vec![0.0, 0.5, 1.0]);

        let trace = |name: &str| {
            waveforms
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(name))
                .map(|(_, waveform)| waveform)
                .unwrap_or_else(|| {
                    panic!(
                        "{name} is missing; result holds {:?}",
                        waveforms.keys().collect::<Vec<_>>()
                    )
                })
        };
        let forward = trace("out [forward]");
        let reverse = trace("out [reverse]");

        // Both branches span the whole axis, so neither is dropped by the
        // shared-axis length check on the way to a plot.
        assert_eq!(forward.y_values.len(), sweep_values.len());
        assert_eq!(reverse.y_values.len(), sweep_values.len());
        for (forward, reverse) in forward.y_values.iter().zip(&reverse.y_values) {
            assert!(
                (forward - reverse).abs() <= 1.0e-9,
                "a linear divider must retrace exactly: {forward} vs {reverse}"
            );
        }
        // The divider halves its input, so the forward branch rises with the
        // source. This is what makes the branch comparison above load-bearing.
        assert!((forward.y_values[0] - 0.0).abs() <= 1.0e-9);
        assert!((forward.y_values[2] - 0.5).abs() <= 1.0e-9);

        // Currents are branched the same way, and keep their unit.
        let current = trace("I(Vin) [reverse]");
        assert_eq!(current.y_unit, "A");
        assert_eq!(current.y_values.len(), sweep_values.len());

        // A one-way sweep is unchanged: no branch suffix, one trace per signal.
        assert!(
            !waveforms.keys().any(|name| name == "out"),
            "a retracing sweep names its branches rather than leaving a bare trace"
        );
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
