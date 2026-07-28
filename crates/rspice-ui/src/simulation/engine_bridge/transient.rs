//! Transient analysis over the engine bridge.

use std::borrow::Cow;
use std::collections::HashMap;

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

        convert_transient_result(netlist, tran_result, config.start_time, abort)
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
    config.max_timestep.unwrap_or(config.step_time).max(1e-18)
}

fn transient_start_index(time: &[f64], start_time: f64) -> usize {
    if !start_time.is_finite() || start_time <= 0.0 {
        return 0;
    }
    time.partition_point(|t| *t < start_time)
}

fn transient_sample_count_after_index(
    time: &[f64],
    voltages: &[Vec<f64>],
    branch_currents: &[Vec<f64>],
    start_idx: usize,
) -> usize {
    let max_time_len = time.len().saturating_sub(start_idx);
    voltages
        .iter()
        .chain(branch_currents)
        .fold(max_time_len, |acc, trace| {
            acc.min(trace.len().saturating_sub(start_idx))
        })
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
    let start_idx = transient_start_index(&tran_result.time, start_time);
    let sample_count = transient_sample_count_after_index(
        &tran_result.time,
        &tran_result.voltages,
        &tran_result.branch_currents,
        start_idx,
    );
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
        let name = tran_result
            .node_names
            .get(node_idx)
            .cloned()
            .unwrap_or_else(|| format!("{}", node_idx + 1));

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
        let branch = tran_result
            .branch_names
            .get(branch_idx)
            .cloned()
            .unwrap_or_else(|| format!("branch{}", branch_idx + 1));
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
            WaveformData::new_time_domain(
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
            ),
        );
    }

    let measurements =
        super::measure::evaluate_measurements(netlist, "TRAN", &filtered_time, &waveforms, abort)?;
    Ok(SimulationResult::Transient {
        time: filtered_time,
        waveforms,
        measurements,
        periodic_state: None,
    })
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
