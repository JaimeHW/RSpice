use std::collections::HashMap;

use super::EngineBridge;
use crate::simulation::config::TransientAnalysisConfig;
use crate::simulation::results::{SimulationResult, WaveformData};
use crate::simulation::runner::SimulationError;

impl EngineBridge {
    /// Run transient analysis.
    pub(super) fn run_transient(
        &self,
        netlist: &rspice_core::Netlist,
        config: &TransientAnalysisConfig,
    ) -> Result<SimulationResult, SimulationError> {
        warn_unsupported_uic(config);
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
        warn_unsupported_uic(config);
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

fn warn_unsupported_uic(config: &TransientAnalysisConfig) {
    if config.uic {
        log::warn!(
            "Transient UIC requested, but rspice-core transient startup currently uses DC operating-point initialization"
        );
    }
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
