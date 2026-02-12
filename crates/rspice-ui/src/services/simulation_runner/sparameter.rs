use super::{build_engine_config, generate_freq_points};
use num_complex::Complex64;
use rspice_core::analysis::ac::AcResult;
use rspice_core::engine::Engine;
use rspice_core::netlist::{Element, ElementKind, SourceSpec};
use rspice_core::Value;
use std::fmt;

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
    fn validate(&self) -> Result<(), SParameterRunError> {
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err(SParameterRunError::Validation(
                "S-parameter start frequency must be positive".to_string(),
            ));
        }
        if !self.stop_freq.is_finite() || self.stop_freq <= self.start_freq {
            return Err(SParameterRunError::Validation(
                "S-parameter stop frequency must be greater than start frequency".to_string(),
            ));
        }
        if self.points_per_unit == 0 {
            return Err(SParameterRunError::Validation(
                "S-parameter points per unit must be greater than zero".to_string(),
            ));
        }
        if !self.z0.is_finite() || self.z0 <= 0.0 {
            return Err(SParameterRunError::Validation(
                "S-parameter reference impedance must be positive".to_string(),
            ));
        }
        if self.ports.len() < 2 {
            return Err(SParameterRunError::Validation(
                "S-parameter analysis requires at least 2 ports".to_string(),
            ));
        }
        for (idx, port) in self.ports.iter().enumerate() {
            if port.node_pos.trim().is_empty() {
                return Err(SParameterRunError::Validation(format!(
                    "S-parameter port{} positive node is required",
                    idx + 1
                )));
            }
            if port.node_neg.trim().is_empty() {
                return Err(SParameterRunError::Validation(format!(
                    "S-parameter port{} negative node is required",
                    idx + 1
                )));
            }
            if let Some(port_z0) = port.z0 {
                if !port_z0.is_finite() || port_z0 <= 0.0 {
                    return Err(SParameterRunError::Validation(format!(
                        "S-parameter port{} z0 must be positive",
                        idx + 1
                    )));
                }
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum SParameterRunError {
    Validation(String),
    Parse(String),
    Resolution(String),
    Execution(String),
    Data(String),
}

impl fmt::Display for SParameterRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(message)
            | Self::Parse(message)
            | Self::Resolution(message)
            | Self::Execution(message)
            | Self::Data(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for SParameterRunError {}

/// Run N-port S-parameter analysis by solving Y-parameters from AC source injections.
pub fn run_sparameter_analysis(
    netlist_text: &str,
    config: &SParameterRunConfig,
) -> Result<SParameterData, String> {
    run_sparameter_analysis_typed(netlist_text, config).map_err(|error| error.to_string())
}

fn run_sparameter_analysis_typed(
    netlist_text: &str,
    config: &SParameterRunConfig,
) -> Result<SParameterData, SParameterRunError> {
    config.validate()?;

    let parsed_netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| SParameterRunError::Parse(format!("Parse error: {}", e)))?;

    let frequencies = generate_freq_points(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
    );
    if frequencies.is_empty() {
        return Err(SParameterRunError::Data(
            "S-parameter sweep generated no frequency points".to_string(),
        ));
    }

    let num_ports = config.ports.len();
    let num_freqs = frequencies.len();
    let z0_by_port: Vec<Value> = config
        .ports
        .iter()
        .map(|port| port.z0.unwrap_or(config.z0))
        .collect();
    let mut y = vec![vec![vec![Complex64::new(0.0, 0.0); num_freqs]; num_ports]; num_ports];

    for excite_port in 0..num_ports {
        let mut excited_netlist = parsed_netlist.clone();
        let port_sources =
            inject_sparameter_port_sources(&mut excited_netlist, config, excite_port)?;
        let engine = Engine::new(build_engine_config(&excited_netlist, None));
        let circuit = engine.build_circuit(&excited_netlist).map_err(|e| {
            SParameterRunError::Execution(format!("S-parameter circuit build error: {}", e))
        })?;
        let mut port_branches = Vec::with_capacity(num_ports);
        for port_src in &port_sources {
            let branch = circuit.get_branch_by_name(port_src).ok_or_else(|| {
                SParameterRunError::Resolution(format!(
                    "S-parameter source '{}' branch not found",
                    port_src
                ))
            })? as usize;
            port_branches.push(branch);
        }

        let ac_points = engine.run_ac(&excited_netlist, &frequencies).map_err(|e| {
            SParameterRunError::Execution(format!("S-parameter AC analysis error: {}", e))
        })?;
        if ac_points.len() != frequencies.len() {
            return Err(SParameterRunError::Data(format!(
                "S-parameter AC returned {} points for {} requested frequencies",
                ac_points.len(),
                frequencies.len()
            )));
        }

        for (freq_idx, point) in ac_points.iter().enumerate() {
            // AC source branch current sign is opposite to port-current-into-network.
            for (row_port, (branch, port_src)) in
                port_branches.iter().zip(port_sources.iter()).enumerate()
            {
                let current = -branch_current_from_ac(point, *branch).ok_or_else(|| {
                    SParameterRunError::Data(format!(
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
        let mut y_matrix = vec![vec![Complex64::new(0.0, 0.0); num_ports]; num_ports];
        for row in 0..num_ports {
            for col in 0..num_ports {
                y_matrix[row][col] = y[row][col][freq_idx];
            }
        }
        let s_matrix = compute_s_from_y_matrix(&y_matrix, &z0_by_port);
        for row in 0..num_ports {
            for col in 0..num_ports {
                s[row][col][freq_idx] = s_matrix[row][col];
            }
        }
    }

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
) -> Result<Vec<String>, SParameterRunError> {
    if excite_port >= config.ports.len() {
        return Err(SParameterRunError::Validation(format!(
            "S-parameter excite_port {} out of range for {} ports",
            excite_port,
            config.ports.len()
        )));
    }

    let mut port_sources = Vec::with_capacity(config.ports.len());
    for (idx, port) in config.ports.iter().enumerate() {
        let name = unique_aux_element_name(netlist, &format!("__RSPICE_SP_PORT{}", idx + 1));
        let magnitude = if idx == excite_port { 1.0 } else { 0.0 };
        netlist.elements.push(Element {
            name: name.clone(),
            nodes: vec![port.node_pos.clone(), port.node_neg.clone()],
            kind: ElementKind::VoltageSource(SourceSpec::DcAc {
                dc_value: 0.0,
                ac_magnitude: magnitude,
                ac_phase: 0.0,
            }),
        });
        port_sources.push(name);
    }

    Ok(port_sources)
}

fn unique_aux_element_name(netlist: &rspice_core::Netlist, base: &str) -> String {
    let name_exists = |candidate: &str| {
        netlist
            .elements
            .iter()
            .any(|elem| elem.name.eq_ignore_ascii_case(candidate))
    };

    if !name_exists(base) {
        return base.to_string();
    }

    for idx in 1.. {
        let candidate = format!("{}_{}", base, idx);
        if !name_exists(&candidate) {
            return candidate;
        }
    }
    unreachable!("unbounded iterator should always find a unique name");
}

fn branch_current_from_ac(point: &AcResult, branch_ordinal: usize) -> Option<Complex64> {
    let branch_index = branch_ordinal.checked_sub(1)?;
    point.currents.get(branch_index).copied()
}

fn compute_s_from_y_matrix(y: &[Vec<Complex64>], z0_by_port: &[Value]) -> Vec<Vec<Complex64>> {
    let n = y.len();
    if n == 0 || y.iter().any(|row| row.len() != n) {
        return Vec::new();
    }
    if z0_by_port.len() != n {
        return vec![vec![Complex64::new(0.0, 0.0); n]; n];
    }

    let mut a = identity_complex_matrix(n);
    let mut b = identity_complex_matrix(n);
    for row in 0..n {
        let z0 = Complex64::new(z0_by_port[row], 0.0);
        for col in 0..n {
            let zy = z0 * y[row][col];
            a[row][col] += zy;
            b[row][col] -= zy;
        }
    }

    let Some(inv_a) = invert_complex_matrix(&a) else {
        return vec![vec![Complex64::new(0.0, 0.0); n]; n];
    };
    let mut s = multiply_complex_matrix(&b, &inv_a);
    // General multi-port normalization for non-uniform real reference impedances:
    // S = D^(-1) * (I - ZY) * (I + ZY)^(-1) * D, where D = diag(sqrt(Z0_i)).
    for row in 0..n {
        for col in 0..n {
            let scale = (z0_by_port[col] / z0_by_port[row]).sqrt();
            s[row][col] *= Complex64::new(scale, 0.0);
        }
    }
    s
}

fn identity_complex_matrix(size: usize) -> Vec<Vec<Complex64>> {
    let mut matrix = vec![vec![Complex64::new(0.0, 0.0); size]; size];
    for (idx, row) in matrix.iter_mut().enumerate() {
        row[idx] = Complex64::new(1.0, 0.0);
    }
    matrix
}

fn multiply_complex_matrix(lhs: &[Vec<Complex64>], rhs: &[Vec<Complex64>]) -> Vec<Vec<Complex64>> {
    let rows = lhs.len();
    let cols = rhs.first().map_or(0, |row| row.len());
    let inner = rhs.len();
    let mut out = vec![vec![Complex64::new(0.0, 0.0); cols]; rows];
    for row in 0..rows {
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
    out
}

fn invert_complex_matrix(matrix: &[Vec<Complex64>]) -> Option<Vec<Vec<Complex64>>> {
    let n = matrix.len();
    if n == 0 || matrix.iter().any(|row| row.len() != n) {
        return None;
    }

    let mut augmented = vec![vec![Complex64::new(0.0, 0.0); 2 * n]; n];
    for row in 0..n {
        for col in 0..n {
            augmented[row][col] = matrix[row][col];
        }
        augmented[row][n + row] = Complex64::new(1.0, 0.0);
    }

    for col in 0..n {
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
            return None;
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
        for col in 0..n {
            inverse[row][col] = augmented[row][n + col];
        }
    }
    Some(inverse)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparameter_run_error_display_returns_inner_message() {
        let error = SParameterRunError::Execution("execution-failed".to_string());
        assert_eq!(error.to_string(), "execution-failed");
    }

    #[test]
    fn test_config_validate_rejects_single_port() {
        let config = SParameterRunConfig {
            start_freq: 1e3,
            stop_freq: 1e6,
            points_per_unit: 10,
            sweep: SParameterSweep::Decade,
            z0: 50.0,
            ports: vec![SParameterPort::single_ended("in")],
        };
        let error = config
            .validate()
            .expect_err("single-port config should fail validation");
        assert!(error.to_string().contains("at least 2 ports"));
    }

    #[test]
    fn test_compute_s_from_y_matrix_applies_nonuniform_reference_normalization() {
        let y = vec![
            vec![Complex64::new(0.0, 0.0), Complex64::new(5e-3, 0.0)],
            vec![Complex64::new(5e-3, 0.0), Complex64::new(0.0, 0.0)],
        ];
        let z0 = vec![50.0, 200.0];

        let mut a = identity_complex_matrix(2);
        let mut b = identity_complex_matrix(2);
        for row in 0..2 {
            let zref = Complex64::new(z0[row], 0.0);
            for col in 0..2 {
                let zy = zref * y[row][col];
                a[row][col] += zy;
                b[row][col] -= zy;
            }
        }
        let inv_a = invert_complex_matrix(&a).expect("matrix should be invertible");
        let raw = multiply_complex_matrix(&b, &inv_a);
        let mut expected = raw.clone();
        for row in 0..2 {
            for col in 0..2 {
                let scale = (z0[col] / z0[row]).sqrt();
                expected[row][col] *= Complex64::new(scale, 0.0);
            }
        }

        let actual = compute_s_from_y_matrix(&y, &z0);
        for row in 0..2 {
            for col in 0..2 {
                assert!(
                    (actual[row][col] - expected[row][col]).norm() < 1e-12,
                    "S{}{} mismatch: actual={}, expected={}",
                    row + 1,
                    col + 1,
                    actual[row][col],
                    expected[row][col]
                );
            }
        }
    }

    #[test]
    fn test_compute_s_from_y_matrix_rejects_non_square_input() {
        let y = vec![vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)]];
        let z0 = vec![50.0];
        let s = compute_s_from_y_matrix(&y, &z0);
        assert!(
            s.is_empty(),
            "non-square Y input should return an empty matrix"
        );
    }

    #[test]
    fn test_compute_s_from_y_matrix_returns_zeros_for_reference_length_mismatch() {
        let y = vec![vec![Complex64::new(0.01, 0.0)]];
        let z0 = vec![];
        let s = compute_s_from_y_matrix(&y, &z0);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].len(), 1);
        assert_eq!(s[0][0], Complex64::new(0.0, 0.0));
    }

    #[test]
    fn test_invert_complex_matrix_rejects_singular_matrix() {
        let singular = vec![
            vec![Complex64::new(1.0, 0.0), Complex64::new(2.0, 0.0)],
            vec![Complex64::new(2.0, 0.0), Complex64::new(4.0, 0.0)],
        ];
        assert!(
            invert_complex_matrix(&singular).is_none(),
            "singular matrix inversion should fail"
        );
    }

    #[test]
    fn test_unique_aux_element_name_is_case_insensitive() {
        let mut netlist = rspice_core::netlist::parse_netlist("V1 in 0 1\nR1 in out 1k\n.end\n")
            .expect("netlist should parse");
        netlist.elements.push(Element {
            name: "__rspice_sp_port1".to_string(),
            nodes: vec!["out".to_string(), "0".to_string()],
            kind: ElementKind::Resistor {
                value: 1e3,
                model: None,
                instance_params: Vec::new(),
            },
        });
        let generated = unique_aux_element_name(&netlist, "__RSPICE_SP_PORT1");
        assert_eq!(generated, "__RSPICE_SP_PORT1_1");
    }

    #[test]
    fn test_inject_sparameter_port_sources_sets_single_excited_port() {
        let mut netlist = rspice_core::netlist::parse_netlist("R1 in 0 50\nR2 out 0 50\n.end\n")
            .expect("netlist should parse");
        let config = SParameterRunConfig {
            start_freq: 1e3,
            stop_freq: 1e6,
            points_per_unit: 5,
            sweep: SParameterSweep::Decade,
            z0: 50.0,
            ports: vec![
                SParameterPort::single_ended("in"),
                SParameterPort::single_ended("out"),
            ],
        };

        let names = inject_sparameter_port_sources(&mut netlist, &config, 1)
            .expect("port source injection should succeed");
        assert_eq!(names.len(), 2);
        let injected: Vec<_> = netlist
            .elements
            .iter()
            .filter(|element| names.iter().any(|name| name == &element.name))
            .collect();
        assert_eq!(injected.len(), 2);
        let magnitudes: Vec<Value> = injected
            .iter()
            .map(|element| match &element.kind {
                ElementKind::VoltageSource(SourceSpec::DcAc { ac_magnitude, .. }) => *ac_magnitude,
                other => panic!("expected injected AC source, got {:?}", other),
            })
            .collect();
        assert_eq!(magnitudes.iter().filter(|mag| **mag == 1.0).count(), 1);
        assert_eq!(magnitudes.iter().filter(|mag| **mag == 0.0).count(), 1);
    }
}
