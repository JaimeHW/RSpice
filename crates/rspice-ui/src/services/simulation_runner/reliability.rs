//! Calibrated reliability missions through the resolved simulation engine.
use super::*;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::reliability::ReliabilityRunRequest;
use rspice_core::engine::ReliabilityRunResult;
use std::path::Path;

pub fn run_reliability_analysis_with_source_path_and_abort(
    netlist_text: &str,
    request: &ReliabilityRunRequest,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<ReliabilityRunResult> {
    error::ensure_not_aborted(abort)?;
    request.validate().map_err(ServiceRunError::Failure)?;
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let engine = Engine::try_new_with_resolved_config(build_engine_config(&netlist, None))
        .map_err(|e| {
            ServiceRunError::from_core(
                "Reliability configuration",
                rspice_core::SimulationError::Configuration(e),
            )
        })?;
    let result = engine
        .run_reliability_with_abort(&netlist, request, abort)
        .map_err(|e| ServiceRunError::from_core("Reliability", e))?;
    result
        .validate_retained_payload_with_abort(&engine.config().resource_limits, abort)
        .map_err(|e| ServiceRunError::from_core("Reliability evidence", e))?;
    Ok(result)
}
