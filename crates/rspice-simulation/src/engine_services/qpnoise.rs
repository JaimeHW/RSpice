//! QPNOISE consumes the exact QPSS producer configuration and retained orbit.
use super::{ServiceRunError, ServiceRunResult};
use crate::error::ensure_not_aborted;
use rspice_core::engine::{QpnoiseAnalysisResult, QpssOperatingPoint};
use rspice_core::netlist::QpnoiseCard;

pub(crate) fn run_qpnoise_analysis_from_qpss_on_materialized_with_context(
    netlist: &rspice_core::Netlist,
    card: &QpnoiseCard,
    point: &QpssOperatingPoint,
    context: super::ServiceContext<'_>,
) -> ServiceRunResult<QpnoiseAnalysisResult> {
    let abort = context.abort;
    ensure_not_aborted(abort)?;
    // The producer's engine settings authenticate the orbit. QPNOISE's authored
    // adjoint tolerance configures its translated solve, not a new producer.
    let engine = context.periodic_engine(
        netlist,
        point.config().solver.relative_tolerance,
        "QPNOISE resolved engine configuration is invalid",
    )?;
    engine
        .run_qpnoise_card_from_qpss_with_abort(netlist, card, point, abort)
        .map_err(|error| ServiceRunError::from_core("QPNOISE", error))
}

#[cfg(test)]
mod tests;
