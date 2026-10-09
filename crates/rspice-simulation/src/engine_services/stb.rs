//! Stability analysis.
//!
//! Loop gain, phase margin, and gain margin at a designated probe, using the
//! Tian double injection that measures a loop without breaking it.
//!
//! Named for what the engine runs. `Engine::run_stb` performs one voltage and
//! one current injection at the probe branch and forms the return ratio from
//! the pair; there is no second method behind a flag, and no configuration
//! selects one.

use super::{ServiceRunError, ServiceRunResult};
use crate::error::ensure_not_aborted;
#[cfg(test)]
use rspice_core::Value;
#[cfg(test)]
use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::Engine;
#[cfg(test)]
use std::path::Path;

/// Complete core stability evidence, including missing quantities and diagnostics.
pub type StbData = rspice_core::analysis::stb::StbResult;

/// Run STB analysis over a decade sweep with cooperative cancellation.
///
/// Test-only. The shipping path is
/// [`run_stb_analysis_with_context`].
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
        rspice_core::analysis::stb::StbConfig::new()
            .with_sweep(start_freq, stop_freq, points_per_decade)
            .with_sweep_type(rspice_core::analysis::stb::StbSweepType::Decade)
            .with_probe(probe)
            .with_nyquist(true),
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
#[cfg(test)]
pub fn run_stb_analysis_with_sweep_and_source_path_and_abort(
    netlist_text: &str,
    stb_config: rspice_core::analysis::stb::StbConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<StbData> {
    run_stb_analysis_with_context(
        netlist_text,
        stb_config,
        super::ServiceContext::with_defaults(source_path, abort),
    )
}

/// Execute with the caller's source, cancellation, and resource policy.
pub fn run_stb_analysis_with_context(
    netlist_text: &str,
    stb_config: rspice_core::analysis::stb::StbConfig,
    context: super::ServiceContext<'_>,
) -> ServiceRunResult<StbData> {
    let abort = context.abort;
    let netlist = context.parse(netlist_text)?;
    ensure_not_aborted(abort)?;
    let engine = Engine::new(context.engine_config(&netlist));

    let analysis = engine
        .run_stb_with_abort(&netlist, stb_config, abort)
        .map_err(|error| ServiceRunError::from_core("STB analysis error", error))?;

    ensure_not_aborted(abort)?;
    Ok(analysis.result)
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

    #[test]
    fn stb_service_retains_the_continuous_core_bode_phase() {
        let result = run_stb_analysis_with_abort(
            "* three-pole loop\n\
E1 eo 0 n3 0 -1000\nVP eo x 0\n\
R1 x n1 1k\nC1 n1 0 159.154943091895n\n\
E2 b1 0 n1 0 1\nR2 b1 n2 1k\nC2 n2 0 159.154943091895n\n\
E3 b2 0 n2 0 1\nR3 b2 n3 1k\nC3 n3 0 159.154943091895n\n.end\n",
            "VP",
            100.0,
            1e5,
            20,
            &rspice_core::NoAbort,
        )
        .unwrap();
        assert_eq!(result.bode_points.len(), 61);
        for point in &result.bode_points {
            let ratio = point.frequency / 1000.0;
            assert!((point.phase_deg.unwrap() + 3.0 * ratio.atan().to_degrees()).abs() < 1e-8);
            assert!(
                (point.magnitude_db.unwrap() - (60.0 - 30.0 * (1.0 + ratio * ratio).log10())).abs()
                    < 1e-8
            );
        }
    }
}
