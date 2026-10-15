//! Admission for resource policies while advanced services acquire explicit limits.

use rspice_core::ResourceLimits;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;

/// Return a blocker when a route cannot enforce a requested execution policy.
/// Source preparation has its own bounded admission before dispatch.
pub fn execution_resource_policy_blocker(
    spec: &AnalysisSpec,
    mut limits: ResourceLimits,
) -> Option<&'static str> {
    if matches!(
        spec,
        AnalysisSpec::LegacyDcOp
            | AnalysisSpec::DcOp { .. }
            | AnalysisSpec::DcSweep { .. }
            | AnalysisSpec::Transient { .. }
            | AnalysisSpec::TransientNoise { .. }
            | AnalysisSpec::Ac { .. }
            | AnalysisSpec::AcData { .. }
            | AnalysisSpec::Noise { .. }
            | AnalysisSpec::PoleZero { .. }
            | AnalysisSpec::Sensitivity { .. }
            | AnalysisSpec::Stb { .. }
            | AnalysisSpec::SParameter { .. }
            | AnalysisSpec::Tf { .. }
            | AnalysisSpec::Disto { .. }
            | AnalysisSpec::DcMismatch { .. }
            | AnalysisSpec::Soa { .. }
            | AnalysisSpec::Pac
            | AnalysisSpec::Pxf
            | AnalysisSpec::Pnoise
            | AnalysisSpec::Pstb
            | AnalysisSpec::Psp { .. }
            | AnalysisSpec::Hbsp { .. }
            | AnalysisSpec::Hbnoise { .. }
            | AnalysisSpec::Fft { .. }
            | AnalysisSpec::Fourier { .. }
            | AnalysisSpec::Envelope { .. }
            | AnalysisSpec::Optimization { .. }
            | AnalysisSpec::MonteCarlo { .. }
            | AnalysisSpec::Pss { .. }
            | AnalysisSpec::PssSpectrum { .. }
            | AnalysisSpec::HarmonicBalance { .. }
            | AnalysisSpec::Qpss { .. }
            | AnalysisSpec::Qpac { .. }
            | AnalysisSpec::Qpxf { .. }
            | AnalysisSpec::Qpnoise { .. }
    ) {
        return None;
    }
    let defaults = ResourceLimits::default();
    // The worker ceiling is not one of the limits this gate is about. Every
    // limit below is enforced by the analysis that reads it, which is why an
    // analysis that does not read them must refuse a run that narrows them.
    // The worker count is enforced by the engine instead — it bounds the
    // pools `Engine::parallel_worker_count` builds, whatever the analysis is
    // — so honouring it never depended on the spec, and refusing a run over
    // it would reject a setting that is in fact applied.
    limits.max_parallel_workers = defaults.max_parallel_workers;
    limits.max_netlist_bytes = defaults.max_netlist_bytes;
    limits.max_netlist_lines = defaults.max_netlist_lines;
    limits.max_expanded_source_bytes = defaults.max_expanded_source_bytes;
    limits.max_dependency_source_bytes = defaults.max_dependency_source_bytes;
    limits.max_include_depth = defaults.max_include_depth;
    limits.max_hierarchy_depth = defaults.max_hierarchy_depth;
    limits.max_flattened_elements = defaults.max_flattened_elements;
    (limits != defaults).then_some(
        "this analysis does not yet enforce custom execution resource limits; use default execution limits",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An analysis outside the enforcing list above still runs when the only
    /// thing the reader changed is the worker ceiling.
    ///
    /// The engine bounds its own pools by that number whatever the analysis
    /// is, so the setting is applied here rather than ignored, and refusing
    /// the run would reject a preference that in fact took effect.
    #[test]
    fn a_narrower_worker_ceiling_alone_does_not_refuse_an_analysis() {
        let mut limits = ResourceLimits::default();
        limits.max_parallel_workers = 2;
        assert_eq!(
            execution_resource_policy_blocker(&NON_ENFORCING, limits),
            None
        );
    }

    /// A spec the list above does not name.
    ///
    /// `Parametric` and `Corner` are the two sweeps that drive other analyses
    /// rather than solving themselves, and they are the only specs the gate
    /// still closes on. If one is ever added to the list, this stops being a
    /// non-enforcing spec and the assertions below say so rather than passing
    /// against a gate that no longer refuses anything.
    const NON_ENFORCING: AnalysisSpec = AnalysisSpec::Parametric;

    #[test]
    fn the_gate_still_has_an_analysis_to_close_on() {
        let mut limits = ResourceLimits::default();
        limits.max_analysis_points /= 2;
        assert!(
            execution_resource_policy_blocker(&NON_ENFORCING, limits).is_some(),
            "the spec these tests rely on is no longer outside the enforcing list"
        );
    }
}
