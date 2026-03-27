//! Modal multiconductor transmission line model.
//!
//! This realizes a coupled RLGC line in modal coordinates so transient
//! propagation uses exact per-mode delays rather than a ladder approximation.

use crate::{Value, circuit::NodeId};
use faer::{Mat, Side};

use super::TransmissionLine;

const MODAL_RELATIVE_EIGEN_TOL: Value = 1e-12;

#[derive(Debug, Clone)]
pub struct CoupledTransmissionLine {
    pub name: String,
    pub near_nodes: Vec<NodeId>,
    pub far_nodes: Vec<NodeId>,
    pub near_ref: NodeId,
    pub far_ref: NodeId,
    modal_from_physical_voltage: Vec<Vec<Value>>,
    modal_from_physical_current: Vec<Vec<Value>>,
    modal_to_physical_current: Vec<Vec<Value>>,
    port_admittance: Vec<Vec<Value>>,
    modal_conductances: Vec<Value>,
    dc_series_resistances: Vec<Value>,
    modes: Vec<TransmissionLine>,
}

impl CoupledTransmissionLine {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        near_nodes: Vec<NodeId>,
        near_ref: NodeId,
        far_nodes: Vec<NodeId>,
        far_ref: NodeId,
        r: &[Vec<Value>],
        l: &[Vec<Value>],
        c: &[Vec<Value>],
        g: &[Vec<Value>],
        length: Value,
    ) -> Result<Self, String> {
        let conductors = near_nodes.len();
        if conductors < 2 || far_nodes.len() != conductors {
            return Err(format!(
                "Coupled transmission line '{}' requires matching near/far conductor counts >= 2",
                name
            ));
        }
        validate_square_matrix("R", r, conductors)?;
        validate_square_matrix("L", l, conductors)?;
        validate_square_matrix("C", c, conductors)?;
        validate_square_matrix("G", g, conductors)?;
        if !length.is_finite() || length <= 0.0 {
            return Err(format!(
                "Coupled transmission line '{}' has invalid length {}",
                name, length
            ));
        }

        let (c_eigs, c_vecs) = symmetric_eigendecompose(c)?;
        let c_tol = positive_eigen_tolerance(&c_eigs);
        if c_eigs.iter().any(|&x| !x.is_finite() || x <= c_tol) {
            return Err(format!(
                "Coupled transmission line '{}' requires a positive-definite capacitance matrix",
                name
            ));
        }
        let c_sqrt = reconstruct_from_eigensystem(&c_eigs, &c_vecs, Value::sqrt);
        let c_inv_sqrt = reconstruct_from_eigensystem(&c_eigs, &c_vecs, |x| 1.0 / x.sqrt());

        let modal_metric = mat_mul(&mat_mul(&c_sqrt, l), &c_sqrt);
        let (mode_metrics, mode_vecs) = symmetric_eigendecompose(&modal_metric)?;
        let mode_tol = positive_eigen_tolerance(&mode_metrics);
        if mode_metrics
            .iter()
            .any(|&x| !x.is_finite() || x <= mode_tol)
        {
            return Err(format!(
                "Coupled transmission line '{}' requires a positive-definite inductance matrix",
                name
            ));
        }

        let u_t = transpose(&mode_vecs);
        let modal_from_physical_voltage = mat_mul(&u_t, &c_sqrt);
        let modal_from_physical_current = mat_mul(&u_t, &c_inv_sqrt);
        let modal_to_physical_current = transpose(&modal_from_physical_voltage);

        let r_modal = mat_mul(
            &mat_mul(&modal_from_physical_voltage, r),
            &transpose(&modal_from_physical_voltage),
        );
        let g_modal = mat_mul(
            &mat_mul(&modal_from_physical_current, g),
            &transpose(&modal_from_physical_current),
        );

        let mut modal_conductances = Vec::with_capacity(conductors);
        let mut modes = Vec::with_capacity(conductors);
        for mode in 0..conductors {
            let z_mode = mode_metrics[mode].max(mode_tol).sqrt();
            let td_mode = length * z_mode;
            let g_mode = 1.0 / z_mode.max(mode_tol.sqrt());
            let r_mode = r_modal[mode][mode].max(0.0);
            let g_shunt_mode = g_modal[mode][mode].max(0.0);
            let alpha = 0.5 * (r_mode / z_mode.max(mode_tol.sqrt()) + g_shunt_mode * z_mode);
            let attenuation = (-alpha * length).exp().clamp(1e-6, 1.0);
            let loss_time_constant =
                0.5 * (r_mode + mode_metrics[mode].max(0.0) * g_shunt_mode) * length * length;

            let mut mode_line = TransmissionLine::new(
                format!("{}#mode{}", name, mode + 1),
                0,
                0,
                0,
                0,
                z_mode,
                td_mode,
            );
            mode_line.set_distributed_rlgc(
                r_mode,
                mode_metrics[mode].max(mode_tol),
                g_shunt_mode,
                1.0,
                length,
            );
            mode_line.set_attenuation(attenuation);
            if loss_time_constant.is_finite() && loss_time_constant > 0.0 {
                mode_line.set_loss_time_constant(loss_time_constant);
            }
            modal_conductances.push(g_mode);
            modes.push(mode_line);
        }

        let port_admittance = assemble_port_admittance(
            &modal_from_physical_voltage,
            &modal_to_physical_current,
            &modal_conductances,
        );
        let dc_series_resistances = (0..conductors)
            .map(|idx| {
                let resistance = r[idx][idx].max(0.0) * length;
                if resistance.is_finite() && resistance > 0.0 {
                    resistance
                } else {
                    1e-3
                }
            })
            .collect();

        Ok(Self {
            name,
            near_nodes,
            far_nodes,
            near_ref,
            far_ref,
            modal_from_physical_voltage,
            modal_from_physical_current,
            modal_to_physical_current,
            port_admittance,
            modal_conductances,
            dc_series_resistances,
            modes,
        })
    }

    #[inline]
    pub fn conductors(&self) -> usize {
        self.near_nodes.len()
    }

    #[inline]
    pub fn min_mode_delay(&self) -> Value {
        self.modes
            .iter()
            .map(TransmissionLine::delay)
            .fold(Value::INFINITY, Value::min)
    }

    #[inline]
    pub fn propagation_delays(&self) -> impl Iterator<Item = Value> + '_ {
        self.modes.iter().map(TransmissionLine::delay)
    }

    #[inline]
    pub fn launched_modal_waves(&self) -> impl Iterator<Item = (Value, Value, Value)> + '_ {
        self.modes.iter().map(|mode| {
            (
                mode.delay(),
                mode.launched_forward_wave(),
                mode.launched_backward_wave(),
            )
        })
    }

    #[inline]
    pub fn port_admittance(&self) -> &[Vec<Value>] {
        &self.port_admittance
    }

    #[inline]
    pub fn dc_series_resistance(&self, conductor: usize) -> Value {
        self.dc_series_resistances[conductor]
    }

    pub fn reset(&mut self) {
        for mode in &mut self.modes {
            mode.reset();
        }
    }

    pub fn modalize_port_voltage(&self, physical: &[Value]) -> Vec<Value> {
        mat_vec_mul(&self.modal_from_physical_voltage, physical)
    }

    pub fn modalize_port_current(&self, physical: &[Value]) -> Vec<Value> {
        mat_vec_mul(&self.modal_from_physical_current, physical)
    }

    pub fn port_equivalent_current(&self, incoming_modal: &[Value]) -> Vec<Value> {
        let weighted: Vec<Value> = incoming_modal
            .iter()
            .zip(self.modal_conductances.iter())
            .map(|(incoming, g)| incoming * g)
            .collect();
        mat_vec_mul(&self.modal_to_physical_current, &weighted)
    }

    pub fn port_currents(
        &self,
        physical_voltage: &[Value],
        incoming_modal: &[Value],
    ) -> Vec<Value> {
        let mut currents = mat_vec_mul(&self.port_admittance, physical_voltage);
        let eq = self.port_equivalent_current(incoming_modal);
        for (current, eq_value) in currents.iter_mut().zip(eq.iter()) {
            *current -= eq_value;
        }
        currents
    }

    pub fn incoming_near_modal(&self, time: Value, far_refs_modal: &[Value]) -> Vec<Value> {
        self.modes
            .iter()
            .zip(far_refs_modal.iter())
            .map(|(mode, far_ref)| {
                let attenuation = mode.attenuation();
                mode.delayed_backward_at(time) + (1.0 - attenuation) * far_ref
            })
            .collect()
    }

    pub fn incoming_far_modal(&self, time: Value, near_refs_modal: &[Value]) -> Vec<Value> {
        self.modes
            .iter()
            .zip(near_refs_modal.iter())
            .map(|(mode, near_ref)| {
                let attenuation = mode.attenuation();
                mode.delayed_forward_at(time) + (1.0 - attenuation) * near_ref
            })
            .collect()
    }

    pub fn update_modal_history(
        &mut self,
        time: Value,
        near_modal_voltages: &[Value],
        near_modal_currents: &[Value],
        far_modal_voltages: &[Value],
        far_modal_currents: &[Value],
    ) {
        for mode in 0..self.modes.len() {
            self.modes[mode].update_history(
                time,
                near_modal_voltages[mode],
                near_modal_currents[mode],
                far_modal_voltages[mode],
                far_modal_currents[mode],
            );
        }
    }
}

fn positive_eigen_tolerance(eigenvalues: &[Value]) -> Value {
    eigenvalues
        .iter()
        .copied()
        .fold(0.0_f64, |scale, value| scale.max(value.abs()))
        .max(Value::EPSILON)
        * MODAL_RELATIVE_EIGEN_TOL
}

fn validate_square_matrix(
    name: &str,
    matrix: &[Vec<Value>],
    expected: usize,
) -> Result<(), String> {
    if matrix.len() != expected || matrix.iter().any(|row| row.len() != expected) {
        return Err(format!(
            "Coupled transmission line matrix '{}' must be {}x{}",
            name, expected, expected
        ));
    }
    Ok(())
}

fn symmetric_eigendecompose(
    matrix: &[Vec<Value>],
) -> Result<(Vec<Value>, Vec<Vec<Value>>), String> {
    let n = matrix.len();
    let mut faer_mat = Mat::<Value>::zeros(n, n);
    for row in 0..n {
        for col in 0..n {
            faer_mat[(row, col)] = matrix[row][col];
        }
    }

    let evd = faer_mat
        .self_adjoint_eigen(Side::Lower)
        .map_err(|err| format!("self-adjoint eigendecomposition failed: {err:?}"))?;
    let eigenvalues = (0..n)
        .map(|idx| evd.S().column_vector()[idx])
        .collect::<Vec<_>>();
    let eigenvectors = (0..n)
        .map(|row| (0..n).map(|col| evd.U()[(row, col)]).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Ok((eigenvalues, eigenvectors))
}

fn reconstruct_from_eigensystem(
    eigenvalues: &[Value],
    eigenvectors: &[Vec<Value>],
    map: impl Fn(Value) -> Value,
) -> Vec<Vec<Value>> {
    let n = eigenvalues.len();
    let mut result = vec![vec![0.0; n]; n];
    for mode in 0..n {
        let weight = map(eigenvalues[mode]);
        for row in 0..n {
            for col in 0..n {
                result[row][col] += eigenvectors[row][mode] * weight * eigenvectors[col][mode];
            }
        }
    }
    result
}

fn assemble_port_admittance(
    modal_from_physical_voltage: &[Vec<Value>],
    modal_to_physical_current: &[Vec<Value>],
    modal_conductances: &[Value],
) -> Vec<Vec<Value>> {
    let n = modal_conductances.len();
    let mut result = vec![vec![0.0; n]; n];
    for row in 0..n {
        for col in 0..n {
            let mut value = 0.0;
            for mode in 0..n {
                value += modal_to_physical_current[row][mode]
                    * modal_conductances[mode]
                    * modal_from_physical_voltage[mode][col];
            }
            result[row][col] = value;
        }
    }
    result
}

fn mat_mul(a: &[Vec<Value>], b: &[Vec<Value>]) -> Vec<Vec<Value>> {
    let rows = a.len();
    let cols = b[0].len();
    let inner = b.len();
    let mut result = vec![vec![0.0; cols]; rows];
    for row in 0..rows {
        for col in 0..cols {
            let mut sum = 0.0;
            for idx in 0..inner {
                sum += a[row][idx] * b[idx][col];
            }
            result[row][col] = sum;
        }
    }
    result
}

fn mat_vec_mul(a: &[Vec<Value>], x: &[Value]) -> Vec<Value> {
    a.iter()
        .map(|row| row.iter().zip(x.iter()).map(|(aij, xj)| aij * xj).sum())
        .collect()
}

fn transpose(matrix: &[Vec<Value>]) -> Vec<Vec<Value>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut result = vec![vec![0.0; rows]; cols];
    for row in 0..rows {
        for col in 0..cols {
            result[col][row] = matrix[row][col];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coupled_tline_modal_decomposition_matches_two_line_modes() {
        let line = CoupledTransmissionLine::new(
            "P1".to_string(),
            vec![1, 2],
            0,
            vec![3, 4],
            0,
            &[vec![0.5, 0.0], vec![0.0, 0.5]],
            &[vec![247.3e-9, 31.65e-9], vec![31.65e-9, 247.3e-9]],
            &[vec![31.4e-12, -2.45e-12], vec![-2.45e-12, 31.4e-12]],
            &[vec![0.0, 0.0], vec![0.0, 0.0]],
            0.3048,
        )
        .expect("modal decomposition");

        let mut mode_delays = line
            .modes
            .iter()
            .map(|mode| mode.delay())
            .collect::<Vec<_>>();
        mode_delays.sort_by(|a, b| a.partial_cmp(b).expect("finite delays"));

        let expected_fast =
            0.3048_f64 * ((247.3e-9_f64 - 31.65e-9_f64) * (31.4e-12_f64 + 2.45e-12_f64)).sqrt();
        let expected_slow =
            0.3048_f64 * ((247.3e-9_f64 + 31.65e-9_f64) * (31.4e-12_f64 - 2.45e-12_f64)).sqrt();
        assert!((mode_delays[0] - expected_fast).abs() / expected_fast < 1e-6);
        assert!((mode_delays[1] - expected_slow).abs() / expected_slow < 1e-6);
    }

    #[test]
    fn test_coupled_tline_modal_loss_time_constants_follow_modal_rlgc() {
        let line = CoupledTransmissionLine::new(
            "P1".to_string(),
            vec![1, 2],
            0,
            vec![3, 4],
            0,
            &[vec![1.0, 0.0], vec![0.0, 2.0]],
            &[vec![3.0e-9, 0.0], vec![0.0, 5.0e-9]],
            &[vec![7.0e-12, 0.0], vec![0.0, 11.0e-12]],
            &[vec![13.0e-6, 0.0], vec![0.0, 17.0e-6]],
            0.4,
        )
        .expect("uncoupled modal line");

        let mut actual = line
            .modes
            .iter()
            .map(TransmissionLine::loss_time_constant)
            .collect::<Vec<_>>();
        actual.sort_by(|a, b| a.partial_cmp(b).expect("finite taus"));

        let mut expected = vec![
            0.5 * (1.0 * 7.0e-12 + 3.0e-9 * 13.0e-6) * 0.4 * 0.4,
            0.5 * (2.0 * 11.0e-12 + 5.0e-9 * 17.0e-6) * 0.4 * 0.4,
        ];
        expected.sort_by(|a, b| a.partial_cmp(b).expect("finite taus"));

        for (actual_tau, expected_tau) in actual.iter().zip(expected.iter()) {
            assert!(
                ((actual_tau - expected_tau) / expected_tau).abs() < 1e-12,
                "expected modal tau {expected_tau}, got {actual_tau}"
            );
        }
    }
}
