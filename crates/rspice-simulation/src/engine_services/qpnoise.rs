//! QPNOISE consumes the exact QPSS producer configuration and retained orbit.
use super::{ServiceRunError, ServiceRunResult, parse_runner_netlist_with_abort};
use crate::error::ensure_not_aborted;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::{QpnoiseAnalysisResult, QpssOperatingPoint};
use rspice_core::netlist::QpnoiseCard;
use std::path::Path;

#[allow(
    dead_code,
    reason = "retained QPNOISE source-path adapter for callers and tests"
)]
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
    run_qpnoise_analysis_from_qpss_on_materialized_with_context(
        netlist,
        card,
        point,
        super::ServiceContext {
            source_path: None,
            limits: Default::default(),
            abort,
        },
    )
}

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
