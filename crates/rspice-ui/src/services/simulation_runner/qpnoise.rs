//! QPNOISE consumes the exact QPSS producer configuration and retained orbit.
use super::error::ensure_not_aborted;
use super::{
    ServiceRunError, ServiceRunResult, build_resolved_periodic_engine,
    parse_runner_netlist_with_abort,
};
use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::{QpnoiseAnalysisResult, QpssOperatingPoint};
use rspice_core::netlist::QpnoiseCard;
use std::path::Path;

pub fn run_qpnoise_analysis_from_qpss_with_source_path_and_abort(
    netlist_text: &str,
    card: &QpnoiseCard,
    point: &QpssOperatingPoint,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<QpnoiseAnalysisResult> {
    ensure_not_aborted(abort)?;
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    run_qpnoise_analysis_from_qpss_on_materialized_with_abort(&netlist, card, point, abort)
}

pub(crate) fn run_qpnoise_analysis_from_qpss_on_materialized_with_abort(
    netlist: &rspice_core::Netlist,
    card: &QpnoiseCard,
    point: &QpssOperatingPoint,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<QpnoiseAnalysisResult> {
    ensure_not_aborted(abort)?;
    // The producer's engine settings authenticate the orbit. QPNOISE's authored
    // adjoint tolerance configures its translated solve, not a new producer.
    let engine = build_resolved_periodic_engine(
        netlist,
        point.config().solver.relative_tolerance,
        "QPNOISE resolved engine configuration is invalid",
    )?;
    engine
        .run_qpnoise_card_from_qpss_with_abort(netlist, card, point, abort)
        .map_err(|error| ServiceRunError::from_core("QPNOISE", error))
}
