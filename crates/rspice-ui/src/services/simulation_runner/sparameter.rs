//! S-parameter analysis.
//!
//! Sweeps frequency and extracts the scattering matrix between the declared
//! ports, with the port impedances the run configuration sets.

#![allow(clippy::needless_range_loop)]

use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, generate_freq_points_with_abort,
    parse_runner_netlist_with_abort,
};
use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::ac::AcResult;
use rspice_core::analysis::s_param;
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
}

/// Run N-port S-parameter analysis by solving Y-parameters from AC source
/// injections, with cooperative cancellation.
///
/// This is the shipping entry point; the frequency-analysis spec calls it
/// directly, so unlike its siblings it resolves no source path.
pub fn run_sparameter_analysis_with_abort(
    netlist_text: &str,
    config: &SParameterRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SParameterData> {
    run_sparameter_analysis_with_source_path_and_abort(netlist_text, config, None, abort)
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
        let s_matrix = s_param::s_from_y_with_abort(&y_matrix, &z0_by_port, abort)
            .map_err(network_error)?;
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

/// Map a shared S-parameter conversion failure onto the service error type.
///
/// A singular normalization matrix becomes a reported failure, not the
/// zero-filled matrix this module used to return. Presenting fabricated zeros
/// as a measured S-matrix is worse than reporting nothing, because a plot of
/// zeros looks like a real, very good result.
fn network_error(error: s_param::NetworkError) -> ServiceRunError {
    match error {
        s_param::NetworkError::Aborted => ServiceRunError::Aborted,
        other => ServiceRunError::Failure(other.to_string()),
    }
}

fn branch_current_from_ac(point: &AcResult, branch_ordinal: usize) -> Option<Complex64> {
    let branch_index = branch_ordinal.checked_sub(1)?;
    point.currents.get(branch_index).copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

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

    /// The conversion itself, and its in-loop cancellation, are covered by
    /// `rspice_core::analysis::s_param::network`. What this module
    /// still owns is the mapping from that failure onto the service error
    /// type, so that is what is tested here.
    #[test]
    fn conversion_abort_stays_an_abort_not_a_failure() {
        assert!(matches!(
            network_error(s_param::NetworkError::Aborted),
            ServiceRunError::Aborted
        ));
    }

    #[test]
    fn singular_normalization_is_reported_instead_of_returning_zeros() {
        let error = network_error(s_param::NetworkError::SingularNormalization);

        let ServiceRunError::Failure(message) = error else {
            panic!("a singular matrix must surface as a failure");
        };
        assert!(message.contains("singular"), "{message}");
    }

    #[test]
    fn conversion_reaches_core_and_reports_a_singular_network() {
        // Y = -I/Z0 drives (I + ZY) to exactly zero.
        let z0 = 50.0;
        let y = vec![
            vec![Complex64::new(-1.0 / z0, 0.0), Complex64::new(0.0, 0.0)],
            vec![Complex64::new(0.0, 0.0), Complex64::new(-1.0 / z0, 0.0)],
        ];

        let result = s_param::s_from_y_with_abort(&y, &[z0, z0], &NoAbort).map_err(network_error);

        assert!(matches!(result, Err(ServiceRunError::Failure(_))));
    }
}
