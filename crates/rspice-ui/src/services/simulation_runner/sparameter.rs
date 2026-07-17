#![allow(clippy::needless_range_loop)]

use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, generate_freq_points_with_abort,
    parse_runner_netlist_with_abort,
};
use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::analysis::ac::AcResult;
use rspice_core::engine::Engine;
use rspice_core::netlist::{Element, ElementKind, SourceSpec};
use std::path::Path;

/// Sweep type for S-parameter analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SParameterSweep {
    Decade,
    Octave,
    Linear,
}

impl SParameterSweep {
    fn keyword(self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }
}

/// Port definition for S-parameter analysis.
#[derive(Debug, Clone)]
pub struct SParameterPort {
    pub node_pos: String,
    pub node_neg: String,
    pub z0: Option<Value>,
}

impl SParameterPort {
    pub fn single_ended(node_pos: impl Into<String>) -> Self {
        Self {
            node_pos: node_pos.into(),
            node_neg: "0".to_string(),
            z0: None,
        }
    }
}

/// Explicit configuration for S-parameter execution.
#[derive(Debug, Clone)]
pub struct SParameterRunConfig {
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: SParameterSweep,
    pub z0: Value,
    pub ports: Vec<SParameterPort>,
}

impl SParameterRunConfig {
    fn validate(&self) -> Result<(), String> {
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err("S-parameter start frequency must be positive".to_string());
        }
        if !self.stop_freq.is_finite() || self.stop_freq <= self.start_freq {
            return Err(
                "S-parameter stop frequency must be greater than start frequency".to_string(),
            );
        }
        if self.points_per_unit == 0 {
            return Err("S-parameter points per unit must be greater than zero".to_string());
        }
        if !self.z0.is_finite() || self.z0 <= 0.0 {
            return Err("S-parameter reference impedance must be positive".to_string());
        }
        if self.ports.len() < 2 {
            return Err("S-parameter analysis requires at least 2 ports".to_string());
        }
        for (idx, port) in self.ports.iter().enumerate() {
            if port.node_pos.trim().is_empty() {
                return Err(format!(
                    "S-parameter port{} positive node is required",
                    idx + 1
                ));
            }
            if port.node_neg.trim().is_empty() {
                return Err(format!(
                    "S-parameter port{} negative node is required",
                    idx + 1
                ));
            }
            if let Some(port_z0) = port.z0
                && (!port_z0.is_finite() || port_z0 <= 0.0)
            {
                return Err(format!("S-parameter port{} z0 must be positive", idx + 1));
            }
        }
        Ok(())
    }
}

/// N-port S-parameter analysis output.
#[derive(Debug, Clone)]
pub struct SParameterData {
    pub frequencies: Vec<Value>,
    /// Number of ports in the solved network.
    pub num_ports: usize,
    /// S-parameter matrix traces indexed as [row][col][frequency_index], 0-based.
    pub s: Vec<Vec<Vec<Complex64>>>,
    /// Per-port reference impedances (ohms).
    pub z0: Vec<Value>,
}

/// Run N-port S-parameter analysis by solving Y-parameters from AC source injections.
pub fn run_sparameter_analysis(
    netlist_text: &str,
    config: &SParameterRunConfig,
) -> Result<SParameterData, String> {
    run_sparameter_analysis_with_abort(netlist_text, config, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run N-port S-parameter analysis with cooperative cancellation.
pub fn run_sparameter_analysis_with_abort(
    netlist_text: &str,
    config: &SParameterRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SParameterData> {
    run_sparameter_analysis_with_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run N-port S-parameter analysis with a source path used to resolve relative
/// includes and model file references.
pub fn run_sparameter_analysis_with_source_path(
    netlist_text: &str,
    config: &SParameterRunConfig,
    source_path: Option<&Path>,
) -> Result<SParameterData, String> {
    run_sparameter_analysis_with_source_path_and_abort(netlist_text, config, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run N-port S-parameter analysis with source-path resolution and
/// cooperative cancellation through parsing, solving, and matrix conversion.
pub fn run_sparameter_analysis_with_source_path_and_abort(
    netlist_text: &str,
    config: &SParameterRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SParameterData> {
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;
    ensure_not_aborted(abort)?;

    let parsed_netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;

    let frequencies = generate_freq_points_with_abort(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
        abort,
    )?;

    let num_ports = config.ports.len();
    let num_freqs = frequencies.len();
    let mut z0_by_port = Vec::with_capacity(num_ports);
    for (index, port) in config.ports.iter().enumerate() {
        poll_periodically(abort, index)?;
        z0_by_port.push(port.z0.unwrap_or(config.z0));
    }
    let mut y = vec![vec![vec![Complex64::new(0.0, 0.0); num_freqs]; num_ports]; num_ports];

    for excite_port in 0..num_ports {
        ensure_not_aborted(abort)?;
        let mut excited_netlist = parsed_netlist.clone();
        let port_sources =
            inject_sparameter_port_sources(&mut excited_netlist, config, excite_port, abort)?;
        let engine = Engine::new(build_engine_config(&excited_netlist, None));
        ensure_not_aborted(abort)?;
        let circuit = engine.build_circuit(&excited_netlist).map_err(|error| {
            ServiceRunError::Failure(format!("S-parameter circuit build error: {error}"))
        })?;
        ensure_not_aborted(abort)?;
        let mut port_branches = Vec::with_capacity(num_ports);
        for (index, port_src) in port_sources.iter().enumerate() {
            poll_periodically(abort, index)?;
            let branch = circuit.get_branch_by_name(port_src).ok_or_else(|| {
                ServiceRunError::Failure(format!(
                    "S-parameter source '{}' branch not found",
                    port_src
                ))
            })? as usize;
            port_branches.push(branch);
        }

        let ac_points = engine
            .run_ac_with_abort(&excited_netlist, &frequencies, abort)
            .map_err(|error| ServiceRunError::from_core("S-parameter AC analysis error", error))?;
        if ac_points.len() != frequencies.len() {
            return Err(ServiceRunError::Failure(format!(
                "S-parameter AC returned {} points for {} requested frequencies",
                ac_points.len(),
                frequencies.len()
            )));
        }

        for (freq_idx, point) in ac_points.iter().enumerate() {
            poll_periodically(abort, freq_idx)?;
            // AC source branch current sign is opposite to port-current-into-network.
            for (row_port, (branch, port_src)) in
                port_branches.iter().zip(port_sources.iter()).enumerate()
            {
                let current = -branch_current_from_ac(point, *branch).ok_or_else(|| {
                    ServiceRunError::Failure(format!(
                        "S-parameter missing branch current for {} at point {}",
                        port_src, freq_idx
                    ))
                })?;
                y[row_port][excite_port][freq_idx] = current;
            }
        }
    }

    let mut s = vec![vec![vec![Complex64::new(0.0, 0.0); num_freqs]; num_ports]; num_ports];
    for freq_idx in 0..num_freqs {
        poll_periodically(abort, freq_idx)?;
        let mut y_matrix = vec![vec![Complex64::new(0.0, 0.0); num_ports]; num_ports];
        for row in 0..num_ports {
            for col in 0..num_ports {
                y_matrix[row][col] = y[row][col][freq_idx];
            }
        }
        let s_matrix = compute_s_from_y_matrix(&y_matrix, &z0_by_port, abort)?;
        for row in 0..num_ports {
            for col in 0..num_ports {
                s[row][col][freq_idx] = s_matrix[row][col];
            }
        }
    }

    ensure_not_aborted(abort)?;
    Ok(SParameterData {
        frequencies,
        num_ports,
        s,
        z0: z0_by_port,
    })
}

fn inject_sparameter_port_sources(
    netlist: &mut rspice_core::Netlist,
    config: &SParameterRunConfig,
    excite_port: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<String>> {
    ensure_not_aborted(abort)?;
    if excite_port >= config.ports.len() {
        return Err(ServiceRunError::Failure(format!(
            "S-parameter excite_port {} out of range for {} ports",
            excite_port,
            config.ports.len()
        )));
    }

    let mut port_sources = Vec::with_capacity(config.ports.len());
    for (idx, port) in config.ports.iter().enumerate() {
        poll_periodically(abort, idx)?;
        let name =
            unique_aux_element_name(netlist, &format!("__RSPICE_SP_PORT{}", idx + 1), abort)?;
        let magnitude = if idx == excite_port { 1.0 } else { 0.0 };
        netlist.elements.push(Element {
            name: name.clone(),
            nodes: vec![port.node_pos.clone(), port.node_neg.clone()],
            kind: ElementKind::VoltageSource(SourceSpec::DcAc {
                dc_value: 0.0,
                ac_magnitude: magnitude,
                ac_phase: 0.0,
            }),
            provenance: rspice_core::netlist::ElementProvenance::Authored,
        });
        port_sources.push(name);
    }

    Ok(port_sources)
}

fn unique_aux_element_name(
    netlist: &rspice_core::Netlist,
    base: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<String> {
    let name_exists = |candidate: &str| -> ServiceRunResult<bool> {
        for (index, element) in netlist.elements.iter().enumerate() {
            poll_periodically(abort, index)?;
            if element.name.eq_ignore_ascii_case(candidate) {
                return Ok(true);
            }
        }
        Ok(false)
    };

    if !name_exists(base)? {
        return Ok(base.to_string());
    }

    for idx in 1.. {
        ensure_not_aborted(abort)?;
        let candidate = format!("{}_{}", base, idx);
        if !name_exists(&candidate)? {
            return Ok(candidate);
        }
    }
    unreachable!("unbounded iterator should always find a unique name");
}

fn branch_current_from_ac(point: &AcResult, branch_ordinal: usize) -> Option<Complex64> {
    let branch_index = branch_ordinal.checked_sub(1)?;
    point.currents.get(branch_index).copied()
}

fn compute_s_from_y_matrix(
    y: &[Vec<Complex64>],
    z0_by_port: &[Value],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Vec<Complex64>>> {
    ensure_not_aborted(abort)?;
    let n = y.len();
    if n == 0 || y.iter().any(|row| row.len() != n) {
        return Ok(Vec::new());
    }
    if z0_by_port.len() != n {
        return Ok(vec![vec![Complex64::new(0.0, 0.0); n]; n]);
    }

    let mut a = identity_complex_matrix(n, abort)?;
    let mut b = identity_complex_matrix(n, abort)?;
    for row in 0..n {
        poll_periodically(abort, row)?;
        let z0 = Complex64::new(z0_by_port[row], 0.0);
        for col in 0..n {
            let zy = z0 * y[row][col];
            a[row][col] += zy;
            b[row][col] -= zy;
        }
    }

    let Some(inv_a) = invert_complex_matrix(&a, abort)? else {
        return Ok(vec![vec![Complex64::new(0.0, 0.0); n]; n]);
    };
    let mut s = multiply_complex_matrix(&b, &inv_a, abort)?;
    // General multi-port normalization for non-uniform real reference impedances:
    // S = D^(-1) * (I - ZY) * (I + ZY)^(-1) * D, where D = diag(sqrt(Z0_i)).
    for row in 0..n {
        poll_periodically(abort, row)?;
        for col in 0..n {
            let scale = (z0_by_port[col] / z0_by_port[row]).sqrt();
            s[row][col] *= Complex64::new(scale, 0.0);
        }
    }
    ensure_not_aborted(abort)?;
    Ok(s)
}

fn identity_complex_matrix(
    size: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Vec<Complex64>>> {
    let mut matrix = vec![vec![Complex64::new(0.0, 0.0); size]; size];
    for (idx, row) in matrix.iter_mut().enumerate() {
        poll_periodically(abort, idx)?;
        row[idx] = Complex64::new(1.0, 0.0);
    }
    Ok(matrix)
}

fn multiply_complex_matrix(
    lhs: &[Vec<Complex64>],
    rhs: &[Vec<Complex64>],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Vec<Complex64>>> {
    let rows = lhs.len();
    let cols = rhs.first().map_or(0, |row| row.len());
    let inner = rhs.len();
    let mut out = vec![vec![Complex64::new(0.0, 0.0); cols]; rows];
    for row in 0..rows {
        poll_periodically(abort, row)?;
        for k in 0..inner {
            let lhs_value = lhs[row][k];
            if lhs_value.norm() <= 1e-30 {
                continue;
            }
            for col in 0..cols {
                out[row][col] += lhs_value * rhs[k][col];
            }
        }
    }
    ensure_not_aborted(abort)?;
    Ok(out)
}

fn invert_complex_matrix(
    matrix: &[Vec<Complex64>],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<Vec<Vec<Complex64>>>> {
    ensure_not_aborted(abort)?;
    let n = matrix.len();
    if n == 0 || matrix.iter().any(|row| row.len() != n) {
        return Ok(None);
    }

    let mut augmented = vec![vec![Complex64::new(0.0, 0.0); 2 * n]; n];
    for row in 0..n {
        poll_periodically(abort, row)?;
        for col in 0..n {
            augmented[row][col] = matrix[row][col];
        }
        augmented[row][n + row] = Complex64::new(1.0, 0.0);
    }

    for col in 0..n {
        poll_periodically(abort, col)?;
        let mut pivot_row = col;
        let mut pivot_norm = augmented[col][col].norm();
        for (row, row_data) in augmented.iter().enumerate().skip(col + 1) {
            let candidate_norm = row_data[col].norm();
            if candidate_norm > pivot_norm {
                pivot_norm = candidate_norm;
                pivot_row = row;
            }
        }
        if pivot_norm <= 1e-30 {
            return Ok(None);
        }
        if pivot_row != col {
            augmented.swap(pivot_row, col);
        }

        let pivot = augmented[col][col];
        for idx in col..(2 * n) {
            augmented[col][idx] /= pivot;
        }

        let pivot_snapshot = augmented[col].clone();
        for (row, row_data) in augmented.iter_mut().enumerate() {
            if row == col {
                continue;
            }
            let factor = row_data[col];
            if factor.norm() <= 1e-30 {
                continue;
            }
            for idx in col..(2 * n) {
                row_data[idx] -= factor * pivot_snapshot[idx];
            }
        }
    }

    let mut inverse = vec![vec![Complex64::new(0.0, 0.0); n]; n];
    for row in 0..n {
        poll_periodically(abort, row)?;
        for col in 0..n {
            inverse[row][col] = augmented[row][n + col];
        }
    }
    ensure_not_aborted(abort)?;
    Ok(Some(inverse))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::{CountingAbort, ImmediateAbort};

    fn invalid_config() -> SParameterRunConfig {
        SParameterRunConfig {
            start_freq: 0.0,
            stop_freq: 1.0,
            points_per_unit: 0,
            sweep: SParameterSweep::Decade,
            z0: 50.0,
            ports: Vec::new(),
        }
    }

    #[test]
    fn sparameter_service_preserves_typed_entry_abort() {
        let result =
            run_sparameter_analysis_with_abort("not a netlist", &invalid_config(), &ImmediateAbort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn sparameter_matrix_conversion_honors_in_loop_abort() {
        const PORTS: usize = 128;
        let mut y = vec![vec![Complex64::new(0.0, 0.0); PORTS]; PORTS];
        for (index, row) in y.iter_mut().enumerate() {
            row[index] = Complex64::new(1e-3, 0.0);
        }
        let abort = CountingAbort::new(5);

        let result = compute_s_from_y_matrix(&y, &vec![50.0; PORTS], &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.count() > 5);
    }
}
