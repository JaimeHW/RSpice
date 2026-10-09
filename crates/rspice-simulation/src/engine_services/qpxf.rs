//! QPXF consumes the exact QPSS producer configuration and retained orbit.
use super::{ServiceRunError, ServiceRunResult};
use crate::error::ensure_not_aborted;
use rspice_core::engine::{QpssOperatingPoint, QpxfAnalysisResult};
use rspice_core::netlist::QpxfCard;

pub(crate) fn run_qpxf_analysis_from_qpss_on_materialized_with_context(
    netlist: &rspice_core::Netlist,
    card: &QpxfCard,
    point: &QpssOperatingPoint,
    context: super::ServiceContext<'_>,
) -> ServiceRunResult<QpxfAnalysisResult> {
    let abort = context.abort;
    ensure_not_aborted(abort)?;
    // The producer's engine settings authenticate the orbit. QPXF's authored
    // adjoint tolerance configures its translated solve, not a new producer.
    let engine = context.periodic_engine(
        netlist,
        point.config().solver.relative_tolerance,
        "QPXF resolved engine configuration is invalid",
    )?;
    engine
        .run_qpxf_card_from_qpss_with_abort(netlist, card, point, abort)
        .map_err(|error| ServiceRunError::from_core("QPXF", error))
}
