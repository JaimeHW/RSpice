//! Driven QPSS service: resolved deck execution and tuple-preserving spectra.
use super::{ServiceRunError, ServiceRunResult, parse_runner_netlist_with_abort};
use crate::error::ensure_not_aborted;
use crate::results::qpss::{QpssData, qpss_data_from_operating_point_with_abort};
use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::QpssConfig;
use std::{path::Path, sync::Arc};

#[allow(
    dead_code,
    reason = "retained QPSS source-path adapter for callers and tests"
)]
pub fn run_qpss_analysis_with_source_path_and_abort(
    netlist_text: &str,
    config: QpssConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<QpssData> {
    ensure_not_aborted(abort)?;
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    run_qpss_analysis_on_materialized_with_abort(&netlist, config, abort)
}

pub(crate) fn run_qpss_analysis_on_materialized_with_abort(
    netlist: &rspice_core::Netlist,
    config: QpssConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<QpssData> {
    run_qpss_analysis_with_dc_seed_on_materialized_with_abort(netlist, config, None, abort)
}

/// The caller has applied the selected OP environment to this physical circuit.
pub(crate) fn run_qpss_analysis_with_dc_seed_on_materialized_with_abort(
    netlist: &rspice_core::Netlist,
    config: QpssConfig,
    dc_seed: Option<&rspice_core::engine::PeriodicDcOperatingPointSeed>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<QpssData> {
    run_qpss_analysis_with_dc_seed_on_materialized_with_context(
        netlist,
        config,
        dc_seed,
        super::ServiceContext {
            source_path: None,
            limits: Default::default(),
            abort,
        },
    )
}

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
