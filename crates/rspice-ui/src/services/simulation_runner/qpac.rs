//! QPAC consumes the exact QPSS producer configuration and retained orbit.
use super::error::ensure_not_aborted;
use super::{
    ServiceRunError, ServiceRunResult, build_resolved_periodic_engine,
    parse_runner_netlist_with_abort,
};
use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::{QpacAnalysisResult, QpssOperatingPoint};
use rspice_core::netlist::QpacCard;
use std::path::Path;

pub fn run_qpac_analysis_from_qpss_with_source_path_and_abort(
    netlist_text: &str,
    card: &QpacCard,
    point: &QpssOperatingPoint,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<QpacAnalysisResult> {
    ensure_not_aborted(abort)?;
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    run_qpac_analysis_from_qpss_on_materialized_with_abort(&netlist, card, point, abort)
}

pub(crate) fn run_qpac_analysis_from_qpss_on_materialized_with_abort(
    netlist: &rspice_core::Netlist,
    card: &QpacCard,
    point: &QpssOperatingPoint,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<QpacAnalysisResult> {
    ensure_not_aborted(abort)?;
    // The producer's engine settings authenticate the orbit. QPAC's authored
    // equation tolerances configure its translated solve, not a new producer.
    let engine = build_resolved_periodic_engine(
        &netlist,
        point.config().solver.relative_tolerance,
        "QPAC resolved engine configuration is invalid",
    )?;
    engine
        .run_qpac_card_from_qpss_with_abort(&netlist, card, point, abort)
        .map_err(|error| ServiceRunError::from_core("QPAC", error))
}
