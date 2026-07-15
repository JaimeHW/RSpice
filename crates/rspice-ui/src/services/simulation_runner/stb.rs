use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::engine::Engine;
use std::path::Path;

/// STB analysis data for feedback loop stability
#[derive(Debug, Clone)]
pub struct StbData {
    /// Frequency points (Hz)
    pub frequencies: Vec<Value>,
    /// Loop gain magnitude (dB)
    pub loop_gain_db: Vec<Value>,
    /// Loop gain phase (degrees)
    pub loop_phase_deg: Vec<Value>,
    /// Phase margin (degrees)
    pub phase_margin: Value,
    /// Gain margin (dB)
    pub gain_margin: Value,
    /// Unity gain frequency (Hz)
    pub unity_gain_freq: Value,
    /// 180-degree phase crossover frequency (Hz)
    pub phase_crossover_freq: Value,
    /// Whether the loop is stable
    pub is_stable: bool,
}

impl StbData {
    /// Calculate stability from loop gain data
    pub fn calculate_stability(
        frequencies: &[Value],
        gain_db: &[Value],
        phase_deg: &[Value],
    ) -> Self {
        let mut unity_gain_freq = 0.0;
        let mut phase_crossover_freq = 0.0;
        let mut phase_at_unity = 0.0;
        let mut gain_at_phase_cross = 0.0;

        for idx in 1..gain_db.len() {
            if gain_db[idx - 1] >= 0.0 && gain_db[idx] < 0.0 {
                let t = -gain_db[idx - 1] / (gain_db[idx] - gain_db[idx - 1]);
                unity_gain_freq =
                    frequencies[idx - 1] + t * (frequencies[idx] - frequencies[idx - 1]);
                phase_at_unity = phase_deg[idx - 1] + t * (phase_deg[idx] - phase_deg[idx - 1]);
                break;
            }
        }

        for idx in 1..phase_deg.len() {
            if (phase_deg[idx - 1] > -180.0 && phase_deg[idx] <= -180.0)
                || (phase_deg[idx - 1] >= -180.0 && phase_deg[idx] < -180.0)
            {
                let t = (-180.0 - phase_deg[idx - 1]) / (phase_deg[idx] - phase_deg[idx - 1]);
                phase_crossover_freq =
                    frequencies[idx - 1] + t * (frequencies[idx] - frequencies[idx - 1]);
                gain_at_phase_cross = gain_db[idx - 1] + t * (gain_db[idx] - gain_db[idx - 1]);
                break;
            }
        }

        let phase_margin = 180.0 + phase_at_unity;
        let gain_margin = -gain_at_phase_cross;
        let is_stable = phase_margin > 0.0 && gain_margin > 0.0;

        Self {
            frequencies: frequencies.to_vec(),
            loop_gain_db: gain_db.to_vec(),
            loop_phase_deg: phase_deg.to_vec(),
            phase_margin,
            gain_margin,
            unity_gain_freq,
            phase_crossover_freq,
            is_stable,
        }
    }
}

/// Run STB (loop stability) analysis
///
/// Measures the loop gain and phase of a feedback system to determine
/// phase margin and gain margin using AC analysis data.
pub fn run_stb_analysis(
    netlist_text: &str,
    probe: &str,
    start_freq: Value,
    stop_freq: Value,
    points_per_decade: usize,
) -> Result<StbData, String> {
    run_stb_analysis_with_abort(
        netlist_text,
        probe,
        start_freq,
        stop_freq,
        points_per_decade,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run STB analysis with cooperative cancellation.
pub fn run_stb_analysis_with_abort(
    netlist_text: &str,
    probe: &str,
    start_freq: Value,
    stop_freq: Value,
    points_per_decade: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<StbData> {
    run_stb_analysis_with_source_path_and_abort(
        netlist_text,
        probe,
        start_freq,
        stop_freq,
        points_per_decade,
        None,
        abort,
    )
}

/// Run STB (loop stability) analysis with a source path used to resolve
/// relative includes and model file references.
///
/// `probe` names a 0 V voltage source placed in the feedback loop; the
/// engine measures the true loop gain at that break via Tian's
/// double-injection method. An unknown probe is a hard error — there is no
/// meaningful fallback quantity.
pub fn run_stb_analysis_with_source_path(
    netlist_text: &str,
    probe: &str,
    start_freq: Value,
    stop_freq: Value,
    points_per_decade: usize,
    source_path: Option<&Path>,
) -> Result<StbData, String> {
    run_stb_analysis_with_source_path_and_abort(
        netlist_text,
        probe,
        start_freq,
        stop_freq,
        points_per_decade,
        source_path,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run STB analysis with source-path resolution and cancellation.
#[allow(clippy::too_many_arguments)]
pub fn run_stb_analysis_with_source_path_and_abort(
    netlist_text: &str,
    probe: &str,
    start_freq: Value,
    stop_freq: Value,
    points_per_decade: usize,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<StbData> {
    run_stb_analysis_with_sweep_and_source_path_and_abort(
        netlist_text,
        probe,
        start_freq,
        stop_freq,
        rspice_core::analysis::advanced::stb::StbSweepType::Decade,
        points_per_decade,
        source_path,
        abort,
    )
}

/// Run STB analysis with an explicit sweep type and source path.
pub fn run_stb_analysis_with_sweep_and_source_path(
    netlist_text: &str,
    probe: &str,
    start_freq: Value,
    stop_freq: Value,
    sweep_type: rspice_core::analysis::advanced::stb::StbSweepType,
    points_per_decade: usize,
    source_path: Option<&Path>,
) -> Result<StbData, String> {
    run_stb_analysis_with_sweep_and_source_path_and_abort(
        netlist_text,
        probe,
        start_freq,
        stop_freq,
        sweep_type,
        points_per_decade,
        source_path,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run STB analysis with an explicit sweep, source path, and cancellation.
#[allow(clippy::too_many_arguments)]
pub fn run_stb_analysis_with_sweep_and_source_path_and_abort(
    netlist_text: &str,
    probe: &str,
    start_freq: Value,
    stop_freq: Value,
    sweep_type: rspice_core::analysis::advanced::stb::StbSweepType,
    points_per_decade: usize,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<StbData> {
    use rspice_core::analysis::advanced::stb::StbConfig;

    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    ensure_not_aborted(abort)?;
    let engine = Engine::new(build_engine_config(&netlist, None));

    let stb_config = StbConfig::new()
        .with_sweep(start_freq, stop_freq, points_per_decade)
        .with_sweep_type(sweep_type)
        .with_probe(probe)
        .with_nyquist(true);

    let analysis = engine
        .run_stb_with_abort(&netlist, stb_config, abort)
        .map_err(|error| ServiceRunError::from_core("STB analysis error", error))?;

    let result_frequencies = analysis.frequencies;
    let stb_result = analysis.result;

    let mut loop_gain_db = Vec::with_capacity(stb_result.bode_points.len());
    let mut loop_phase_deg = Vec::with_capacity(stb_result.bode_points.len());
    for (index, point) in stb_result.bode_points.iter().enumerate() {
        poll_periodically(abort, index)?;
        loop_gain_db.push(point.magnitude_db);
        loop_phase_deg.push(point.phase_deg);
    }
    ensure_not_aborted(abort)?;

    Ok(StbData {
        frequencies: result_frequencies,
        loop_gain_db,
        loop_phase_deg,
        phase_margin: stb_result.margins.phase_margin_deg,
        gain_margin: stb_result.margins.gain_margin_db,
        unity_gain_freq: stb_result.margins.unity_gain_bandwidth,
        phase_crossover_freq: stb_result.margins.gain_margin_freq,
        is_stable: stb_result.margins.is_stable(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::ImmediateAbort;

    #[test]
    fn stb_service_preserves_typed_entry_abort() {
        let result = run_stb_analysis_with_abort("not a netlist", "", 0.0, 0.0, 0, &ImmediateAbort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }
}
