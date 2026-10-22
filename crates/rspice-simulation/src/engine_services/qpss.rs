//! Driven QPSS service: resolved deck execution and tuple-preserving spectra.
use super::{ServiceRunError, ServiceRunResult};
use crate::error::ensure_not_aborted;
use crate::results::qpss::{QpssData, qpss_data_from_operating_point_with_abort};
use rspice_core::engine::QpssConfig;
use std::sync::Arc;

/// The caller has applied the selected OP environment to this physical circuit.
pub(crate) fn run_qpss_analysis_with_dc_seed_on_materialized_with_context(
    netlist: &rspice_core::Netlist,
    config: QpssConfig,
    dc_seed: Option<&rspice_core::engine::PeriodicDcOperatingPointSeed>,
    context: super::ServiceContext<'_>,
) -> ServiceRunResult<QpssData> {
    let abort = context.abort;
    ensure_not_aborted(abort)?;
    let engine = context.periodic_engine(
        netlist,
        config.solver.relative_tolerance,
        "QPSS resolved engine configuration is invalid",
    )?;
    let point = match dc_seed {
        Some(seed) => engine.run_qpss_with_dc_seed_and_abort(netlist, config, seed, abort),
        None => engine.run_qpss_with_abort(netlist, config, abort),
    }
    .map_err(|error| ServiceRunError::from_core("QPSS", error))?;
    engine
        .validate_qpss_operating_point_with_abort(netlist, &point, abort)
        .map_err(|error| ServiceRunError::from_core("QPSS retained state", error))?;
    qpss_data_from_operating_point_with_abort(Arc::new(point), abort)
}
