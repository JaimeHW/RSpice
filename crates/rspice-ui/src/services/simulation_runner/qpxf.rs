//! QPXF consumes the exact QPSS producer configuration and retained orbit.
use super::error::ensure_not_aborted;
use super::{
    ServiceRunError, ServiceRunResult, build_resolved_periodic_engine,
    parse_runner_netlist_with_abort,
};
use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::{QpssOperatingPoint, QpxfAnalysisResult};
use rspice_core::netlist::QpxfCard;
use std::path::Path;

pub fn run_qpxf_analysis_from_qpss_with_source_path_and_abort(
    netlist_text: &str,
    card: &QpxfCard,
    point: &QpssOperatingPoint,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<QpxfAnalysisResult> {
    ensure_not_aborted(abort)?;
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    // The producer's engine settings authenticate the orbit. QPXF's authored
    // adjoint tolerance configures its translated solve, not a new producer.
    let engine = build_resolved_periodic_engine(
        &netlist,
        point.config().solver.relative_tolerance,
        "QPXF resolved engine configuration is invalid",
    )?;
    engine
        .run_qpxf_card_from_qpss_with_abort(&netlist, card, point, abort)
        .map_err(|error| ServiceRunError::from_core("QPXF", error))
}
