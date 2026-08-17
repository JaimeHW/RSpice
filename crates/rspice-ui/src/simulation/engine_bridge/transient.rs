//! Transient analysis over the engine bridge.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use rspice_core::abort_signal::AbortSignal;

use super::{EngineBridge, ensure_not_aborted};
use crate::simulation::config::TransientAnalysisConfig;
use crate::simulation::results::{SimulationResult, WaveformData};
use crate::simulation::runner::SimulationError;
use rspice_core::netlist::AnalysisCommand;

impl EngineBridge {
    /// Run transient analysis.
    pub(super) fn run_transient(
        &self,
        netlist: &rspice_core::Netlist,
        config: &TransientAnalysisConfig,
        abort: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        ensure_not_aborted(abort)?;
        let prepared_netlist = netlist_for_transient_config(netlist, config, abort)?;
        let netlist = prepared_netlist.as_ref();
        let engine = self.engine_for_netlist(netlist);
        let max_step = resolve_transient_max_step(config);
        let tran_result = engine
            .run_tran_with_abort(netlist, config.stop_time, max_step, abort)
            .map_err(|e| self.translate_error(e))?;
        let convergence = engine.convergence_quality();

        let mut converted =
            convert_transient_result(netlist, tran_result, config.start_time, abort)?;
        if let SimulationResult::Transient {
            convergence: slot, ..
        } = &mut converted
        {
            *slot = convergence;
        }
        Ok(converted)
    }
}

fn netlist_for_transient_config<'a>(
    netlist: &'a rspice_core::Netlist,
    config: &TransientAnalysisConfig,
    abort: &dyn AbortSignal,
) -> Result<Cow<'a, rspice_core::Netlist>, SimulationError> {
    ensure_not_aborted(abort)?;
    let mut transient_count = 0usize;
    let mut matching_count = 0usize;
    for analysis in &netlist.analyses {
        ensure_not_aborted(abort)?;
        if matches!(analysis, AnalysisCommand::Tran { .. }) {
            transient_count += 1;
            if transient_command_matches_config(analysis, config) {
                matching_count += 1;
            }
        }
    }
    if transient_count == 1 && matching_count == 1 {
        return Ok(Cow::Borrowed(netlist));
    }

    ensure_not_aborted(abort)?;
    let mut prepared = netlist.clone();
    let mut analyses = Vec::with_capacity(prepared.analyses.len() + 1);
    for analysis in prepared.analyses.drain(..) {
        ensure_not_aborted(abort)?;
        if !matches!(analysis, AnalysisCommand::Tran { .. }) {
            analyses.push(analysis);
        }
    }
    prepared.analyses = analyses;
    prepared
        .analyses
        .push(transient_command_from_config(config));
    ensure_not_aborted(abort)?;
    Ok(Cow::Owned(prepared))
}

fn transient_command_from_config(config: &TransientAnalysisConfig) -> AnalysisCommand {
    AnalysisCommand::Tran {
        step: config.step_time,
        stop: config.stop_time,
        start: (config.start_time > 0.0).then_some(config.start_time),
        max_step: config.max_timestep,
        uic: config.uic,
    }
}

fn transient_command_matches_config(
    analysis: &AnalysisCommand,
    config: &TransientAnalysisConfig,
) -> bool {
    let AnalysisCommand::Tran {
        step,
        stop,
        start,
        max_step,
        uic,
    } = analysis
    else {
        return false;
    };

    *step == config.step_time
        && *stop == config.stop_time
        && *start == (config.start_time > 0.0).then_some(config.start_time)
        && *max_step == config.max_timestep
        && *uic == config.uic
}

fn resolve_transient_max_step(config: &TransientAnalysisConfig) -> f64 {
    // SPICE .tran step is an output interval. Since our transient engine emits
    // accepted timesteps directly, keep internal max-step at or below the
    // requested output step by default to preserve waveform fidelity.
    config.max_timestep.unwrap_or(config.step_time)
}

fn transient_start_index(time: &[f64], start_time: f64) -> usize {
    if !start_time.is_finite() || start_time <= 0.0 {
        return 0;
    }
    time.partition_point(|t| *t < start_time)
}

fn filtered_transient_values(
    time: &[f64],
    values: &[f64],
    start_idx: usize,
    sample_count: usize,
    start_time: f64,
    interpolate_start: bool,
) -> Result<Vec<f64>, SimulationError> {
    let mut filtered = Vec::with_capacity(sample_count + usize::from(interpolate_start));
    if interpolate_start {
        let lower = start_idx - 1;
        let span = time[start_idx] - time[lower];
        if !span.is_finite() || span <= 0.0 {
            return Err(SimulationError::SolverError(
                "transient engine returned a non-increasing time axis at the requested output start"
                    .to_owned(),
            ));
        }
        let fraction = (start_time - time[lower]) / span;
        filtered.push(values[lower] + fraction * (values[start_idx] - values[lower]));
    }
    filtered.extend_from_slice(&values[start_idx..start_idx + sample_count]);
    Ok(filtered)
}

fn convert_transient_result(
    netlist: &rspice_core::Netlist,
    tran_result: rspice_core::engine::TransientResult,
    start_time: f64,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    ensure_not_aborted(abort)?;
    validate_transient_result_shape(&tran_result)?;
    let start_idx = transient_start_index(&tran_result.time, start_time);
    let sample_count = tran_result.time.len().saturating_sub(start_idx);
    if sample_count == 0 {
        return Err(SimulationError::SolverError(format!(
            "transient result has no accepted sample at or after requested start time {start_time:.16e} s"
        )));
    }
    // The adaptive engine emits accepted timesteps rather than an interpolated
    // output grid. If it steps across an authored nonzero `.tran` start, retain
    // the authored boundary itself (with interpolated traces), not the sample
    // before it and not the first later timestep. This preserves visible
    // output semantics and gives dependent Fourier windows exact coverage.
    let interpolate_start = start_time.is_finite()
        && start_time > 0.0
        && start_idx > 0
        && sample_count > 0
        && tran_result.time[start_idx] > start_time;
    let mut filtered_time = Vec::with_capacity(sample_count + usize::from(interpolate_start));
    if interpolate_start {
        filtered_time.push(start_time);
    }
    filtered_time.extend_from_slice(&tran_result.time[start_idx..start_idx + sample_count]);
    let mut waveforms = HashMap::new();

    for (node_idx, voltages) in tran_result.voltages.iter().enumerate() {
        ensure_not_aborted(abort)?;
        // The core preserves node/name index alignment when output projection
        // prunes a trace by returning an empty vector for that signal.  Do not
        // manufacture an empty waveform in the retained UI result.
        if voltages.is_empty() {
            continue;
        }
        let name = tran_result.node_names[node_idx].clone();

        waveforms.insert(
            name.clone(),
            WaveformData::new_time_domain(
                &name,
                filtered_time.clone(),
                filtered_transient_values(
                    &tran_result.time,
                    voltages,
                    start_idx,
                    sample_count,
                    start_time,
                    interpolate_start,
                )?,
            ),
        );
    }

    for (branch_idx, currents) in tran_result.branch_currents.iter().enumerate() {
        ensure_not_aborted(abort)?;
        // As with voltage traces, an empty current vector is the core's
        // deliberate representation of a signal excluded by `.save`/output
        // projection.  A non-empty trace is still required to align exactly
        // with the shared time axis by the validation below.
        if currents.is_empty() {
            continue;
        }
        let branch = tran_result.branch_names[branch_idx].clone();
        let name = if branch.len() >= 3
            && (branch.starts_with("I(") || branch.starts_with("i("))
            && branch.ends_with(')')
        {
            branch
        } else {
            format!("I({branch})")
        };
        waveforms.insert(
            name.clone(),
            WaveformData::new_time_domain_in_unit(
                &name,
                filtered_time.clone(),
                filtered_transient_values(
                    &tran_result.time,
                    currents,
                    start_idx,
                    sample_count,
                    start_time,
                    interpolate_start,
                )?,
                "A",
            ),
        );
    }

    let measurements = evaluate_transient_measurements(
        netlist,
        &tran_result,
        &filtered_time,
        start_idx,
        sample_count,
        start_time,
        interpolate_start,
        abort,
    )?;
    let events = collect_event_history(&tran_result, start_time, abort)?;
    Ok(SimulationResult::Transient {
        time: filtered_time,
        waveforms,
        measurements,
        periodic_state: None,
        // The engine is not in scope here; `run_transient` fills this in.
        convergence: Default::default(),
        events,
    })
}

fn validate_transient_result_shape(
    result: &rspice_core::engine::TransientResult,
) -> Result<(), SimulationError> {
    if result.time.is_empty() {
        return Err(SimulationError::SolverError(
            "transient engine returned an empty time axis".to_owned(),
        ));
    }
    if result.step_sizes.len() != result.time.len()
        || result
            .step_sizes
            .iter()
            .any(|step| !step.is_finite() || *step < 0.0)
    {
        return Err(SimulationError::SolverError(
            "transient engine returned an invalid accepted-step history".to_owned(),
        ));
    }
    if result.num_nodes != result.node_names.len()
        || result.node_names.len() != result.voltages.len()
    {
        return Err(SimulationError::SolverError(format!(
            "transient engine returned num_nodes={}, {} node names, and {} voltage waveforms",
            result.num_nodes,
            result.node_names.len(),
            result.voltages.len()
        )));
    }
    if result.branch_names.len() != result.branch_currents.len() {
        return Err(SimulationError::SolverError(format!(
            "transient engine returned {} branch names but {} current waveforms",
            result.branch_names.len(),
            result.branch_currents.len()
        )));
    }
    if result
        .node_names
        .iter()
        .chain(&result.branch_names)
        .any(|name| name.trim().is_empty())
    {
        return Err(SimulationError::SolverError(
            "transient engine returned an unnamed waveform".to_owned(),
        ));
    }
    let mut signal_names =
        HashSet::with_capacity(result.node_names.len() + result.branch_names.len());
    if result
        .node_names
        .iter()
        .map(|name| format!("v({})", name.trim().to_ascii_lowercase()))
        .chain(
            result
                .branch_names
                .iter()
                .map(|name| format!("i({})", name.trim().to_ascii_lowercase())),
        )
        .any(|name| !signal_names.insert(name))
    {
        return Err(SimulationError::SolverError(
            "transient engine returned duplicate waveform identities".to_owned(),
        ));
    }
    if result.time.iter().any(|time| !time.is_finite())
        || result.time.windows(2).any(|pair| pair[1] <= pair[0])
    {
        return Err(SimulationError::SolverError(
            "transient engine returned a non-finite or non-increasing time axis".to_owned(),
        ));
    }
    for (trace_index, trace) in result
        .voltages
        .iter()
        .chain(&result.branch_currents)
        .enumerate()
    {
        if !trace.is_empty() && trace.len() != result.time.len() {
            return Err(SimulationError::SolverError(format!(
                "transient waveform {} has {} samples, expected {}",
                trace_index + 1,
                trace.len(),
                result.time.len()
            )));
        }
        if trace.iter().any(|value| !value.is_finite()) {
            return Err(SimulationError::SolverError(format!(
                "transient waveform {} contains a non-finite sample",
                trace_index + 1
            )));
        }
    }
    let mut auxiliary_names =
        HashSet::with_capacity(result.device_op_traces.len() + result.store_traces.len());
    for trace in &result.device_op_traces {
        let identity = format!(
            "{}:{}",
            trace.device_name.trim().to_ascii_lowercase(),
            trace.parameter.trim().to_ascii_lowercase()
        );
        if trace.device_name.trim().is_empty()
            || trace.parameter.trim().is_empty()
            || !auxiliary_names.insert(identity)
            || trace.values.len() != result.time.len()
            || trace.values.iter().any(|value| value.is_infinite())
        {
            return Err(SimulationError::SolverError(
                "transient engine returned an invalid device operating-point trace".to_owned(),
            ));
        }
    }
    for trace in &result.store_traces {
        let identity = format!("store:{}", trace.name.trim().to_ascii_lowercase());
        if trace.name.trim().is_empty()
            || !auxiliary_names.insert(identity)
            || trace.values.len() != result.time.len()
            || trace.values.iter().any(|value| !value.is_finite())
        {
            return Err(SimulationError::SolverError(
                "transient engine returned an invalid device store trace".to_owned(),
            ));
        }
    }
    Ok(())
}

fn evaluate_transient_measurements(
    netlist: &rspice_core::Netlist,
    result: &rspice_core::engine::TransientResult,
    filtered_time: &[f64],
    start_idx: usize,
    sample_count: usize,
    start_time: f64,
    interpolate_start: bool,
    abort: &dyn AbortSignal,
) -> Result<Vec<rspice_core::MeasureResult>, SimulationError> {
    if !netlist
        .measurements
        .iter()
        .any(|measurement| measurement.analysis.eq_ignore_ascii_case("TRAN"))
    {
        return Ok(Vec::new());
    }
    ensure_not_aborted(abort)?;
    let filter = |values: &[f64]| {
        if values.is_empty() {
            return Err(SimulationError::SolverError(
                "a transient measurement operand waveform was not retained".to_owned(),
            ));
        }
        filtered_transient_values(
            &result.time,
            values,
            start_idx,
            sample_count,
            start_time,
            interpolate_start,
        )
    };
    let mut voltages = Vec::with_capacity(result.voltages.len());
    for values in &result.voltages {
        ensure_not_aborted(abort)?;
        voltages.push(filter(values)?);
    }
    let mut branch_currents = Vec::with_capacity(result.branch_currents.len());
    for values in &result.branch_currents {
        ensure_not_aborted(abort)?;
        branch_currents.push(filter(values)?);
    }
    let mut device_op_traces = Vec::with_capacity(result.device_op_traces.len());
    for trace in &result.device_op_traces {
        ensure_not_aborted(abort)?;
        device_op_traces.push(rspice_core::engine::TransientDeviceOpTrace {
            device_name: trace.device_name.clone(),
            parameter: trace.parameter.clone(),
            values: filter(&trace.values)?,
        });
    }
    let mut store_traces = Vec::with_capacity(result.store_traces.len());
    for trace in &result.store_traces {
        ensure_not_aborted(abort)?;
        store_traces.push(rspice_core::engine::TransientStoreTrace {
            name: trace.name.clone(),
            values: filter(&trace.values)?,
        });
    }
    let step_sizes = std::iter::once(0.0)
        .chain(filtered_time.windows(2).map(|pair| pair[1] - pair[0]))
        .collect();
    let visible_result = rspice_core::engine::TransientResult {
        time: filtered_time.to_vec(),
        step_sizes,
        voltages,
        branch_currents,
        num_nodes: result.num_nodes,
        node_names: result.node_names.clone(),
        branch_names: result.branch_names.clone(),
        digital_traces: Vec::new(),
        real_traces: Vec::new(),
        device_op_traces,
        store_traces,
    };
    let measurements = rspice_core::analysis::evaluate_tran_measurements(netlist, &visible_result);
    ensure_not_aborted(abort)?;
    Ok(measurements)
}

/// Retain the committed event schedule, windowed to the same authored start
/// the analog traces are windowed to.
///
/// Events keep their own times: an event node changes when the event solver
/// accepted the change, which is not generally an analog timestep. A node
/// whose whole history precedes the output window is dropped entirely rather
/// than retained empty, so the payload never claims a node it cannot show.
fn collect_event_history(
    tran_result: &rspice_core::engine::TransientResult,
    start_time: f64,
    abort: &dyn AbortSignal,
) -> Result<crate::simulation::results::TransientEventHistory, SimulationError> {
    use crate::simulation::results::{
        DigitalEventPoint, EventNodeHistory, RealEventPoint, TransientEventHistory,
    };

    let window_start = if start_time.is_finite() && start_time > 0.0 {
        start_time
    } else {
        0.0
    };
    let retained = |time: f64| time >= window_start;
    let mut event_nodes =
        HashSet::with_capacity(tran_result.digital_traces.len() + tran_result.real_traces.len());

    let mut digital = Vec::new();
    for trace in &tran_result.digital_traces {
        ensure_not_aborted(abort)?;
        let normalized_name = trace.node_name.trim().to_ascii_lowercase();
        if normalized_name.is_empty() || !event_nodes.insert(normalized_name) {
            return Err(SimulationError::SolverError(
                "transient event history contains an empty or duplicate node identity".to_owned(),
            ));
        }
        if trace.points.iter().any(|point| !point.time.is_finite())
            || trace
                .points
                .windows(2)
                .any(|pair| pair[1].time < pair[0].time)
        {
            return Err(SimulationError::SolverError(format!(
                "transient digital event node '{}' has an invalid time history",
                trace.node_name
            )));
        }
        let points = trace
            .points
            .iter()
            .filter(|point| retained(point.time))
            .map(|point| DigitalEventPoint {
                time_s: point.time,
                value_code: point.value.event_code(),
            })
            .collect::<Vec<_>>();
        if !points.is_empty() {
            digital.push(EventNodeHistory {
                node_name: trace.node_name.clone(),
                points,
            });
        }
    }

    let mut real = Vec::new();
    for trace in &tran_result.real_traces {
        ensure_not_aborted(abort)?;
        let normalized_name = trace.node_name.trim().to_ascii_lowercase();
        if normalized_name.is_empty() || !event_nodes.insert(normalized_name) {
            return Err(SimulationError::SolverError(
                "transient event history contains an empty or duplicate node identity".to_owned(),
            ));
        }
        if trace
            .points
            .iter()
            .any(|point| !point.time.is_finite() || !point.value.is_finite())
            || trace
                .points
                .windows(2)
                .any(|pair| pair[1].time < pair[0].time)
        {
            return Err(SimulationError::SolverError(format!(
                "transient real event node '{}' has an invalid time/value history",
                trace.node_name
            )));
        }
        let points = trace
            .points
            .iter()
            .filter(|point| retained(point.time))
            .map(|point| RealEventPoint {
                time_s: point.time,
                value: point.value,
            })
            .collect::<Vec<_>>();
        if !points.is_empty() {
            real.push(EventNodeHistory {
                node_name: trace.node_name.clone(),
                points,
            });
        }
    }

    Ok(TransientEventHistory { digital, real })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::netlist::AnalysisCommand;

    fn parse_netlist(source: &str) -> rspice_core::Netlist {
        rspice_core::Netlist::parse(source).expect("test netlist parses")
    }

    #[test]
    fn transient_config_injects_uic_command_for_generated_decks() {
        let netlist = parse_netlist(
            "generated transient\n\
             V1 in 0 1\n\
             R1 in 0 1k\n\
             .end\n",
        );
        assert!(
            netlist
                .analyses
                .iter()
                .all(|analysis| !matches!(analysis, AnalysisCommand::Tran { .. }))
        );
        let config = TransientAnalysisConfig {
            stop_time: 1e-6,
            step_time: 1e-9,
            start_time: 2e-9,
            max_timestep: Some(5e-10),
            uic: true,
        };

        let prepared =
            netlist_for_transient_config(&netlist, &config, &rspice_core::abort_signal::NoAbort)
                .expect("transient netlist preparation");
        let tran = prepared
            .analyses
            .iter()
            .find(|analysis| matches!(analysis, AnalysisCommand::Tran { .. }))
            .expect("transient command is synthesized");

        match tran {
            AnalysisCommand::Tran {
                step,
                stop,
                start,
                max_step,
                uic,
            } => {
                assert_eq!(*step, config.step_time);
                assert_eq!(*stop, config.stop_time);
                assert_eq!(*start, Some(config.start_time));
                assert_eq!(*max_step, config.max_timestep);
                assert!(*uic);
            }
            _ => unreachable!("filtered for transient command"),
        }
    }

    #[test]
    fn transient_config_replaces_stale_parsed_tran_command() {
        let netlist = parse_netlist(
            "manual transient\n\
             V1 in 0 1\n\
             R1 in 0 1k\n\
             .tran 1e-9 1e-6\n\
             .end\n",
        );
        let config = TransientAnalysisConfig {
            stop_time: 2e-6,
            step_time: 2e-9,
            start_time: 0.0,
            max_timestep: Some(1e-9),
            uic: true,
        };

        let prepared =
            netlist_for_transient_config(&netlist, &config, &rspice_core::abort_signal::NoAbort)
                .expect("transient netlist preparation");
        let transients: Vec<_> = prepared
            .analyses
            .iter()
            .filter(|analysis| matches!(analysis, AnalysisCommand::Tran { .. }))
            .collect();

        assert_eq!(transients.len(), 1);
        match transients[0] {
            AnalysisCommand::Tran {
                step,
                stop,
                start,
                max_step,
                uic,
            } => {
                assert_eq!(*step, config.step_time);
                assert_eq!(*stop, config.stop_time);
                assert_eq!(*start, None);
                assert_eq!(*max_step, config.max_timestep);
                assert!(*uic);
            }
            _ => unreachable!("filtered for transient command"),
        }
    }

    #[test]
    fn transient_config_reuses_matching_parsed_tran_command() {
        let netlist = parse_netlist(
            "manual transient\n\
             V1 in 0 1\n\
             R1 in 0 1k\n\
             .tran 2e-9 2e-6 1e-9 1e-9 uic\n\
             .end\n",
        );
        let config = TransientAnalysisConfig {
            stop_time: 2e-6,
            step_time: 2e-9,
            start_time: 1e-9,
            max_timestep: Some(1e-9),
            uic: true,
        };

        let prepared =
            netlist_for_transient_config(&netlist, &config, &rspice_core::abort_signal::NoAbort)
                .expect("transient netlist preparation");

        assert!(
            matches!(prepared, std::borrow::Cow::Borrowed(_)),
            "matching manual .tran decks should avoid cloning the parsed netlist"
        );
    }

    #[test]
    fn transient_conversion_preserves_branch_current_waveforms() {
        let netlist = parse_netlist("branch current\nV1 out 0 1\nR1 out 0 1k\n.tran 1n 2n\n.end\n");
        let result = rspice_core::engine::TransientResult {
            time: vec![0.0, 1.0e-9, 2.0e-9],
            step_sizes: vec![0.0, 1.0e-9, 1.0e-9],
            voltages: vec![vec![0.0, 1.0, 1.0]],
            branch_currents: vec![vec![0.0, -1.0e-3, -1.0e-3]],
            num_nodes: 1,
            node_names: vec!["out".to_owned()],
            branch_names: vec!["V1".to_owned()],
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
        };

        let converted =
            convert_transient_result(&netlist, result, 0.0, &rspice_core::abort_signal::NoAbort)
                .expect("transient conversion");
        let SimulationResult::Transient { waveforms, .. } = converted else {
            panic!("expected transient result");
        };
        assert_eq!(waveforms["I(V1)"].y_values, vec![0.0, -1.0e-3, -1.0e-3]);
        assert_eq!(waveforms["I(V1)"].y_unit, "A");
        assert_eq!(waveforms["out"].y_unit, "V");
    }

    #[test]
    fn transient_conversion_omits_output_pruned_waveforms() {
        let netlist = parse_netlist(
            "projected transient\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             C1 out 0 1n\n\
             .tran 1n 2n\n\
             .save v(out)\n\
             .end\n",
        );
        let result = rspice_core::engine::TransientResult {
            time: vec![0.0, 1.0e-9, 2.0e-9],
            step_sizes: vec![0.0, 1.0e-9, 1.0e-9],
            // `in` and I(V1) retain their index slots but are deliberately
            // empty because the authored output contract selected only out.
            voltages: vec![Vec::new(), vec![0.0, 0.5, 1.0]],
            branch_currents: vec![Vec::new()],
            num_nodes: 2,
            node_names: vec!["in".to_owned(), "out".to_owned()],
            branch_names: vec!["V1".to_owned()],
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
        };

        let converted =
            convert_transient_result(&netlist, result, 0.0, &rspice_core::abort_signal::NoAbort)
                .expect("output-pruned transient conversion");
        let SimulationResult::Transient { waveforms, .. } = converted else {
            panic!("expected transient result");
        };

        assert_eq!(waveforms.len(), 1);
        assert_eq!(waveforms["out"].y_values, vec![0.0, 0.5, 1.0]);
        assert!(!waveforms.contains_key("in"));
        assert!(!waveforms.contains_key("I(V1)"));
    }

    #[test]
    fn transient_shape_mismatch_is_a_terminal_error() {
        let result = rspice_core::engine::TransientResult {
            time: vec![0.0, 1.0e-9],
            step_sizes: vec![0.0, 1.0e-9],
            voltages: vec![vec![0.0]],
            branch_currents: Vec::new(),
            num_nodes: 1,
            node_names: vec!["out".to_owned()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
        };

        let error = validate_transient_result_shape(&result)
            .expect_err("a short trace must not truncate the shared time axis");

        assert!(matches!(error, SimulationError::SolverError(_)));
        assert!(error.to_string().contains("1 samples, expected 2"));
    }

    #[test]
    fn adaptive_step_crossing_output_start_is_interpolated_exactly_for_fourier() {
        let netlist = parse_netlist(
            "adaptive start boundary\nV1 out 0 1\nR1 out 0 1k\n.tran 50m 1.15 125m\n.end\n",
        );
        let time = (0..=23)
            .map(|index| f64::from(index) * 0.05)
            .collect::<Vec<_>>();
        let values = time
            .iter()
            .map(|time| (2.0 * std::f64::consts::PI * time).sin())
            .collect::<Vec<_>>();
        let branch_values = values.iter().map(|value| -*value).collect::<Vec<_>>();
        // The initial sample has a zero interval; every later step is the
        // uniform 50 ms this fixture advances by.
        let step_sizes = std::iter::once(0.0)
            .chain(time.windows(2).map(|pair| pair[1] - pair[0]))
            .collect::<Vec<_>>();
        let result = rspice_core::engine::TransientResult {
            time: time.clone(),
            step_sizes,
            voltages: vec![values],
            branch_currents: vec![branch_values],
            num_nodes: 1,
            node_names: vec!["out".to_owned()],
            branch_names: vec!["V1".to_owned()],
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
        };

        let converted =
            convert_transient_result(&netlist, result, 0.125, &rspice_core::abort_signal::NoAbort)
                .expect("adaptive transient conversion");
        let SimulationResult::Transient {
            time, waveforms, ..
        } = converted
        else {
            panic!("expected transient result");
        };
        assert_eq!(
            time.first().copied().map(f64::to_bits),
            Some(0.125_f64.to_bits())
        );
        assert_eq!(waveforms["out"].x_values, time);
        assert_eq!(waveforms["I(V1)"].x_values, time);
        assert_eq!(
            waveforms["I(V1)"].y_values[0].to_bits(),
            (-waveforms["out"].y_values[0]).to_bits()
        );

        let config = crate::services::simulation_runner::FourierRunConfig {
            fundamental_freq: 1.0,
            num_harmonics: 1,
            output_node: "out".to_owned(),
            output_ref: None,
            start_time: 0.125,
            stop_time: 1.125,
            compute_thd: true,
            normalize: false,
        };
        crate::services::simulation_runner::run_fourier_from_signal_with_abort(
            &time,
            &waveforms["out"].y_values,
            &config,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("Fourier consumes the exact interpolated start boundary");
    }
}
