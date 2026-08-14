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
    /// Margins extracted from the loop gain. Always present: the extraction
    /// is what stability analysis is for, so it is a result, not an option.
    pub margins: rspice_core::analysis::stb::StabilityMargins,
    /// Nyquist contour, retained only when the configuration asked for it.
    ///
    /// The extraction's own warnings are deliberately not carried here: the
    /// only two it raises — empty input, and multiple unity-gain crossovers —
    /// are already reported exactly by the margins themselves.
    pub nyquist: Option<StbNyquistContour>,
}

/// The Nyquist contour of the loop gain, split into the three parallel
/// vectors a plot consumes.
#[derive(Debug, Clone)]
pub struct StbNyquistContour {
    pub frequencies: Vec<Value>,
    pub real: Vec<Value>,
    pub imaginary: Vec<Value>,
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
        true,
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
    compute_nyquist: bool,
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
        .with_nyquist(compute_nyquist);

    let analysis = engine
        .run_stb_with_abort(&netlist, stb_config, abort)
        .map_err(|error| ServiceRunError::from_core("STB analysis error", error))?;

    let result_frequencies = analysis.frequencies;
    let stb_result = analysis.result;
    if !stb_result.success
        || result_frequencies.is_empty()
        || result_frequencies.len() != stb_result.bode_points.len()
        || result_frequencies
            .iter()
            .any(|frequency| !frequency.is_finite() || *frequency <= 0.0)
        || result_frequencies.windows(2).any(|pair| pair[1] <= pair[0])
    {
        return Err(ServiceRunError::Failure(
            "STB engine returned an unsuccessful or inconsistent Bode result".to_owned(),
        ));
    }

    let mut loop_gain_db = Vec::with_capacity(stb_result.bode_points.len());
    let mut loop_phase_deg = Vec::with_capacity(stb_result.bode_points.len());
    for (index, point) in stb_result.bode_points.iter().enumerate() {
        poll_periodically(abort, index)?;
        let magnitude = point.loop_gain.norm();
        if point.frequency.to_bits() != result_frequencies[index].to_bits()
            || !point.loop_gain.re.is_finite()
            || !point.loop_gain.im.is_finite()
            || !magnitude.is_finite()
            || magnitude <= 0.0
        {
            return Err(ServiceRunError::Failure(format!(
                "STB Bode point {} has an invalid frequency or loop gain",
                index + 1
            )));
        }
        loop_gain_db.push(20.0 * magnitude.log10());
        loop_phase_deg.push(point.loop_gain.arg().to_degrees());
    }
    ensure_not_aborted(abort)?;

    let nyquist = if compute_nyquist {
        if stb_result.nyquist_points.len() != result_frequencies.len() {
            return Err(ServiceRunError::Failure(format!(
                "STB requested a Nyquist contour but received {} points for {} frequencies",
                stb_result.nyquist_points.len(),
                result_frequencies.len()
            )));
        }
        let mut frequencies = Vec::with_capacity(stb_result.nyquist_points.len());
        let mut real = Vec::with_capacity(stb_result.nyquist_points.len());
        let mut imaginary = Vec::with_capacity(stb_result.nyquist_points.len());
        for (index, point) in stb_result.nyquist_points.iter().enumerate() {
            poll_periodically(abort, index)?;
            if point.frequency.to_bits() != result_frequencies[index].to_bits()
                || !point.real.is_finite()
                || !point.imag.is_finite()
            {
                return Err(ServiceRunError::Failure(format!(
                    "STB Nyquist point {} has an invalid frequency or value",
                    index + 1
                )));
            }
            frequencies.push(point.frequency);
            real.push(point.real);
            imaginary.push(point.imag);
        }
        Some(StbNyquistContour {
            frequencies,
            real,
            imaginary,
        })
    } else {
        None
    };
    ensure_not_aborted(abort)?;

    Ok(StbData {
        frequencies: result_frequencies,
        loop_gain_db,
        loop_phase_deg,
        margins: stb_result.margins,
        nyquist,
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
