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
    ) {
        return None;
    }
    let defaults = ResourceLimits::default();
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
