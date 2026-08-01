//! Stability analysis.
//!
//! Loop gain, phase margin, and gain margin at a designated probe, using the
//! Middlebrook injection that measures a loop without breaking it.

use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
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
}

/// Run STB analysis over a decade sweep with cooperative cancellation.
///
/// Test-only. The shipping path is
/// [`run_stb_analysis_with_sweep_and_source_path_and_abort`].
#[cfg(test)]
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

/// Run STB analysis with source-path resolution and cancellation, fixing the
/// sweep to per-decade.
///
/// Test-only; see [`run_stb_analysis_with_abort`].
#[cfg(test)]
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
        rspice_core::analysis::stb::StbSweepType::Decade,
        points_per_decade,
        source_path,
        abort,
    )
}

/// Run STB (loop stability) analysis with an explicit sweep, source path, and
/// cancellation.
///
/// Measures the loop gain and phase of a feedback system to determine phase
/// margin and gain margin. `probe` names a 0 V voltage source placed in the
/// feedback loop; the engine measures the true loop gain at that break via
/// Tian's double-injection method. An unknown probe is a hard error — there is
/// no meaningful fallback quantity.
pub fn run_stb_analysis_with_sweep_and_source_path_and_abort(
    netlist_text: &str,
    probe: &str,
    start_freq: Value,
    stop_freq: Value,
    sweep_type: rspice_core::analysis::stb::StbSweepType,
    points_per_decade: usize,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<StbData> {
    use rspice_core::analysis::stb::StbConfig;

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
