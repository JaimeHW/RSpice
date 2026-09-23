//! QPAC consumes the exact QPSS producer configuration and retained orbit.
use super::error::ensure_not_aborted;
use super::{ServiceRunError, ServiceRunResult, build_resolved_periodic_engine};
use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::{QpacAnalysisResult, QpssOperatingPoint};
use rspice_core::netlist::QpacCard;

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
        netlist,
        point.config().solver.relative_tolerance,
        "QPAC resolved engine configuration is invalid",
    )?;
    engine
        .run_qpac_card_from_qpss_with_abort(netlist, card, point, abort)
        .map_err(|error| ServiceRunError::from_core("QPAC", error))
}
