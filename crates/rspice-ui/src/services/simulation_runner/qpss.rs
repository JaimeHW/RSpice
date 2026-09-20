//! Driven QPSS service: resolved deck execution and tuple-preserving spectra.
use super::error::ensure_not_aborted;
use super::{
    ServiceRunError, ServiceRunResult, build_resolved_periodic_engine,
    parse_runner_netlist_with_abort,
};
use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::{QpssConfig, QpssOperatingPoint};
use rspice_core::{Complex64, Value};
use std::{path::Path, sync::Arc};

#[derive(Debug, Clone)]
pub struct QpssData {
    /// Increasing nonnegative physical frequencies, including DC. A spectral
    /// component is identified by its tuple, not an integer harmonic label.
    pub frequencies: Vec<Value>,
    pub tuples: Vec<Vec<i32>>,
    /// Peak cosine-reference amplitudes for display: twice each positive
    /// Fourier coefficient, with the signed DC coefficient left unchanged.
    pub spectra: Vec<(String, Vec<Complex64>)>,
    pub operating_point: Arc<QpssOperatingPoint>,
}

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
    ensure_not_aborted(abort)?;
    let engine = build_resolved_periodic_engine(
        &netlist,
        config.solver.relative_tolerance,
        "QPSS resolved engine configuration is invalid",
    )?;
    let point = match dc_seed {
        Some(seed) => engine.run_qpss_with_dc_seed_and_abort(netlist, config, seed, abort),
        None => engine.run_qpss_with_abort(netlist, config, abort),
    }
    .map_err(|error| ServiceRunError::from_core("QPSS", error))?;
    engine
        .validate_qpss_operating_point_with_abort(&netlist, &point, abort)
        .map_err(|error| ServiceRunError::from_core("QPSS retained state", error))?;
    qpss_data_from_operating_point_with_abort(Arc::new(point), abort)
}

/// Reconstruct a display projection from the exact retained state. The same
/// projection is used after worker delivery and after loading a saved result.
pub fn qpss_data_from_operating_point_with_abort(
    point: Arc<QpssOperatingPoint>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<QpssData> {
    let grid = point
        .validate_retained_payload_with_abort(&rspice_core::ResourceLimits::default(), abort)
        .map_err(|error| ServiceRunError::from_core("QPSS retained state", error))?;
    let mut indices: Vec<_> = grid
        .frequencies_hz()
        .iter()
        .enumerate()
        .filter_map(|(index, frequency)| (*frequency >= 0.0).then_some(index))
        .collect();
    indices.sort_by(|a, b| grid.frequencies_hz()[*a].total_cmp(&grid.frequencies_hz()[*b]));
    let frequencies = indices
        .iter()
        .map(|index| grid.frequencies_hz()[*index])
        .collect();
    let tuples = indices
        .iter()
        .map(|index| grid.indices()[*index].clone())
        .collect();
    let mut spectra = Vec::new();
    for (row, coefficients) in point.spectra().iter().enumerate() {
        ensure_not_aborted(abort)?;
        let name = if row < point.node_names().len() {
            format!("V({})", point.node_names()[row])
        } else {
            format!(
                "I({})",
                point.branch_names()[row - point.node_names().len()]
            )
        };
        let mut values = Vec::with_capacity(indices.len());
        for &index in &indices {
            let value = coefficients[index] * if index == grid.dc_index() { 1.0 } else { 2.0 };
            if !value.re.is_finite() || !value.im.is_finite() || !value.norm().is_finite() {
                return Err(ServiceRunError::Failure(
                    "QPSS peak-amplitude display scaling overflowed".into(),
                ));
            }
            values.push(value);
        }
        spectra.push((name, values));
    }
    ensure_not_aborted(abort)?;
    Ok(QpssData {
        frequencies,
        tuples,
        spectra,
        operating_point: point,
    })
}
