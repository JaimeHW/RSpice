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
use rspice_core::engine::Engine;

/// Complete core stability evidence, including missing quantities and diagnostics.
pub type StbData = rspice_core::analysis::stb::StbResult;

/// Execute around `bias`, returning the time of the transient state it
/// linearized at, if any. Around a state the deck's `.STB` card runs there;
/// the task's configuration is that card's.
pub(crate) fn run_stb_analysis_with_bias(
    netlist_text: &str,
    stb_config: rspice_core::analysis::stb::StbConfig,
    bias: super::Bias<'_>,
    context: super::ServiceContext<'_>,
) -> ServiceRunResult<(StbData, Option<rspice_core::Value>)> {
    let abort = context.abort;
    let netlist = context.parse_around(netlist_text, bias)?;
    ensure_not_aborted(abort)?;
    let engine = Engine::new(context.engine_config(&netlist));
    if let super::Bias::TransientState { checkpoint } = bias {
        let state = super::linearize_at_state(
            &engine,
            &netlist,
            super::StateCard::Stability,
            checkpoint,
            abort,
        )
        .map_err(|error| error.into_service("STB analysis error"))?;
        let rspice_core::engine::SmallSignalCardResult::Stability(analysis) = state.result else {
            return Err(ServiceRunError::Failure(
                "A .STB card around a transient state returned another analysis's result"
                    .to_owned(),
            ));
        };
        ensure_not_aborted(abort)?;
        return Ok((analysis.result, Some(state.state_time)));
    }

    let analysis = engine
        .run_stb_with_abort(&netlist, stb_config, abort)
        .map_err(|error| ServiceRunError::from_core("STB analysis error", error))?;

    ensure_not_aborted(abort)?;
    Ok((analysis.result, None))
}

#[cfg(test)]
mod tests {
    use super::super::test_entry::ctx;
    use super::*;
    use rspice_core::Value;
    use rspice_core::abort_signal::ImmediateAbort;
    use rspice_core::analysis::stb::{StbConfig, StbSweepType};

    fn decade(probe: &str, start: Value, stop: Value, points_per_decade: usize) -> StbConfig {
        StbConfig::new()
            .with_sweep(start, stop, points_per_decade)
            .with_sweep_type(StbSweepType::Decade)
            .with_probe(probe)
            .with_nyquist(true)
    }

    #[test]
    fn stb_service_preserves_typed_entry_abort() {
        let result = run_stb_analysis_with_bias(
            "not a netlist",
            decade("", 0.0, 0.0, 0),
            super::super::Bias::OperatingPoint,
            ctx(&ImmediateAbort),
        );

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn stb_service_retains_the_continuous_core_bode_phase() {
        let (result, _) = run_stb_analysis_with_bias(
            "* three-pole loop\n\
E1 eo 0 n3 0 -1000\nVP eo x 0\n\
R1 x n1 1k\nC1 n1 0 159.154943091895n\n\
E2 b1 0 n1 0 1\nR2 b1 n2 1k\nC2 n2 0 159.154943091895n\n\
E3 b2 0 n2 0 1\nR3 b2 n3 1k\nC3 n3 0 159.154943091895n\n.end\n",
            decade("VP", 100.0, 1e5, 20),
            super::super::Bias::OperatingPoint,
            ctx(&rspice_core::NoAbort),
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
