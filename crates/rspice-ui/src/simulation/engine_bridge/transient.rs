use std::borrow::Cow;
use std::collections::HashMap;

use super::EngineBridge;
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
    ) -> Result<SimulationResult, SimulationError> {
        let prepared_netlist = netlist_for_transient_config(netlist, config);
        let netlist = prepared_netlist.as_ref();
        let engine = self.engine_for_netlist(netlist);
        let max_step = resolve_transient_max_step(config);
        let tran_result = engine
            .run_tran(netlist, config.stop_time, max_step)
            .map_err(|e| self.translate_error(e))?;

        Ok(convert_transient_result(
            netlist,
            tran_result,
            config.start_time,
        ))
    }

    /// Run transient analysis with abort signal for cooperative cancellation.
    pub(super) fn run_transient_with_abort(
        &self,
        netlist: &rspice_core::Netlist,
        config: &TransientAnalysisConfig,
        abort: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        let prepared_netlist = netlist_for_transient_config(netlist, config);
        let netlist = prepared_netlist.as_ref();
        let engine = self.engine_for_netlist(netlist);
        let max_step = resolve_transient_max_step(config);
        let tran_result = engine
            .run_tran_with_abort(netlist, config.stop_time, max_step, abort)
            .map_err(|e| self.translate_error(e))?;

        Ok(convert_transient_result(
            netlist,
            tran_result,
            config.start_time,
        ))
    }
}

fn netlist_for_transient_config<'a>(
    netlist: &'a rspice_core::Netlist,
    config: &TransientAnalysisConfig,
) -> Cow<'a, rspice_core::Netlist> {
    let mut transient_count = 0usize;
    let mut matching_count = 0usize;
    for analysis in &netlist.analyses {
        if matches!(analysis, AnalysisCommand::Tran { .. }) {
            transient_count += 1;
            if transient_command_matches_config(analysis, config) {
                matching_count += 1;
            }
        }
    }
    if transient_count == 1 && matching_count == 1 {
        return Cow::Borrowed(netlist);
    }

    let mut prepared = netlist.clone();
    prepared
        .analyses
        .retain(|analysis| !matches!(analysis, AnalysisCommand::Tran { .. }));
    prepared
        .analyses
        .push(transient_command_from_config(config));
    Cow::Owned(prepared)
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
    start_idx: usize,
) -> usize {
    let max_time_len = time.len().saturating_sub(start_idx);
    voltages.iter().fold(max_time_len, |acc, trace| {
        acc.min(trace.len().saturating_sub(start_idx))
    })
}

fn convert_transient_result(
    netlist: &rspice_core::Netlist,
    tran_result: rspice_core::engine::TransientResult,
    start_time: f64,
) -> SimulationResult {
    let start_idx = transient_start_index(&tran_result.time, start_time);
    let sample_count =
        transient_sample_count_after_index(&tran_result.time, &tran_result.voltages, start_idx);
    let filtered_time = tran_result.time[start_idx..start_idx + sample_count].to_vec();
    let mut waveforms = HashMap::new();

    for (node_idx, voltages) in tran_result.voltages.iter().enumerate() {
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
                voltages[start_idx..start_idx + sample_count].to_vec(),
            ),
        );
    }

    let measurements =
        super::measure::evaluate_measurements(netlist, "TRAN", &filtered_time, &waveforms);
    SimulationResult::Transient {
        time: filtered_time,
        waveforms,
        measurements,
    }
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

        let prepared = netlist_for_transient_config(&netlist, &config);
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

        let prepared = netlist_for_transient_config(&netlist, &config);
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

        let prepared = netlist_for_transient_config(&netlist, &config);

        assert!(
            matches!(prepared, std::borrow::Cow::Borrowed(_)),
            "matching manual .tran decks should avoid cloning the parsed netlist"
        );
    }
}
