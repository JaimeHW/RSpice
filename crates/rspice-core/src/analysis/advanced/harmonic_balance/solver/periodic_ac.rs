//! Periodic small-signal (conversion-matrix) solve around a converged
//! large-signal harmonic-balance operating point.
//!
//! The large-signal solution defines a periodically time-varying small-signal
//! conductance g(t) at every nonlinear device. A stimulus at absolute
//! frequency `offset + m*f0` mixes through g(t) into responses at every
//! `offset + k*f0`. With the sideband phasors kept TWO-SIDED (negative k are
//! independent unknowns, not conjugates), multiplication by real g(t) is
//! exactly the Toeplitz operator `G[k-m]` built from the Fourier coefficients
//! of g(t), with `G[-d] = conj(G[d])`. No conjugate (Hankel) coupling exists
//! in this representation, so the conversion solve below is exact for the
//! memoryless device set the HB runtime supports.

use super::*;
use std::f64::consts::PI;

/// One small-signal excitation column: current injections applied at a single
/// input sideband.
#[derive(Debug, Clone)]
pub struct PeriodicAcExcitation {
    /// Input sideband index m: the stimulus sits at `offset + m*f0`.
    pub sideband: i32,
    /// Current injected INTO each node: `(node index, phasor amplitude)`.
    pub injections: Vec<(usize, Complex64)>,
}

impl HbSolver {
    /// Sample the periodic small-signal conductance spectra around the
    /// converged operating point.
    ///
    /// Returns sparse `(i, j, G)` entries where `G[d]` is the d-th Fourier
    /// coefficient of the time-varying conductance stamp g_ij(t); negative
    /// harmonics follow from conjugate symmetry of the real waveform.
    pub(super) fn conductance_spectra(
        &mut self,
        state: &HbSolverState,
    ) -> Vec<(usize, usize, Vec<Complex64>)> {
        let n = self.num_nodes;
        let n_time = self.fft.size();

        let v_time: Vec<Vec<Value>> = (0..n)
            .map(|node| self.fft.to_time_domain(&state.x[node]))
            .collect();

        let mut g_time = vec![vec![vec![0.0; n_time]; n]; n];

        if !self.nonlinear_devices.is_empty() {
            let mut node_voltages = vec![0.0; n];
            for t in 0..n_time {
                for node in 0..n {
                    node_voltages[node] = v_time[node][t];
                }
                for device in &self.nonlinear_devices {
                    for ((i, j), g) in device.jacobian(&node_voltages) {
                        if i < n && j < n {
                            g_time[i][j][t] += g;
                        }
                    }
                }
            }
        }

        #[cfg(feature = "veriloga")]
        if !self.veriloga_nonlinear_devices.is_empty() {
            let mut circuit_voltages = vec![0.0; n];
            for t in 0..n_time {
                for node in 0..n {
                    circuit_voltages[node] = v_time[node][t];
                }
                for device in &mut self.veriloga_nonlinear_devices {
                    device.device.update_all_voltages(&circuit_voltages);
                    let jac_entries = device.device.compute_jacobian();
                    for entry in jac_entries {
                        let Some(prog_locs) = device.jacobian_locs.get(entry.program_idx) else {
                            continue;
                        };
                        let Some(&(row, col)) = prog_locs.get(entry.jacobian_idx) else {
                            continue;
                        };
                        if let (Some(i), Some(j)) = (row, col)
                            && i < n
                            && j < n
                        {
                            g_time[i][j][t] += entry.value;
                        }
                    }
                }
            }
        }

        let mut spectra = Vec::new();
        for i in 0..n {
            for j in 0..n {
                let max_g: Value = g_time[i][j].iter().fold(0.0, |a, &b| a.max(b.abs()));
                if max_g < 1e-30 {
                    continue;
                }
                let spectrum = self.fft.to_frequency_domain(&g_time[i][j]);
                spectra.push((i, j, spectrum));
            }
        }
        spectra
    }

    /// Periodic small-signal capacitance spectra around the operating
    /// point: sparse `(i, j, C)` entries from the device charge Jacobians,
    /// in the same conventions as `conductance_spectra`.
    pub(super) fn capacitance_spectra(
        &mut self,
        state: &HbSolverState,
    ) -> Vec<(usize, usize, Vec<Complex64>)> {
        let n = self.num_nodes;
        if !self.nonlinear_devices.iter().any(|d| d.has_charge_storage()) {
            return Vec::new();
        }
        let n_time = self.fft.size();

        let v_time: Vec<Vec<Value>> = (0..n)
            .map(|node| self.fft.to_time_domain(&state.x[node]))
            .collect();

        let mut c_time = vec![vec![vec![0.0; n_time]; n]; n];
        let mut node_voltages = vec![0.0; n];
        for t in 0..n_time {
            for node in 0..n {
                node_voltages[node] = v_time[node][t];
            }
            for device in &self.nonlinear_devices {
                for ((i, j), c) in device.charge_jacobian(&node_voltages) {
                    if i < n && j < n {
                        c_time[i][j][t] += c;
                    }
                }
            }
        }

        let mut spectra = Vec::new();
        for i in 0..n {
            for j in 0..n {
                let max_c: Value = c_time[i][j].iter().fold(0.0, |a, &b| a.max(b.abs()));
                if max_c < 1e-30 {
                    continue;
                }
                let spectrum = self.fft.to_frequency_domain(&c_time[i][j]);
                spectra.push((i, j, spectrum));
            }
        }
        spectra
    }

    /// Solve the sideband-coupled small-signal system at one offset frequency.
    ///
    /// Unknowns are `V[(node, k)]` for k in `[sideband_min, sideband_max]` at
    /// SIGNED absolute frequencies `f_k = offset + k*f0`. Each excitation is
    /// solved against the same admittance matrix; the result is indexed
    /// `[excitation][node][sideband - sideband_min]`.
    pub fn solve_periodic_ac(
        &mut self,
        state: &HbSolverState,
        offset_hz: Value,
        sideband_min: i32,
        sideband_max: i32,
        excitations: &[PeriodicAcExcitation],
    ) -> Result<Vec<Vec<Vec<Complex64>>>, HbError> {
        if sideband_max < sideband_min {
            return Err(HbError::InvalidCircuit(
                "PAC sideband range is empty".to_string(),
            ));
        }
        let n = self.num_nodes;
        let s = (sideband_max - sideband_min + 1) as usize;
        let size = n * s;
        let omega0 = 2.0 * PI * self.config.fundamental_freq;
        // Below this magnitude a sideband frequency counts as DC for the
        // inductor admittance, mirroring the operating-point treatment.
        let omega_floor = omega0 * 1e-12;

        let spectra = if self.has_nonlinear_devices() {
            self.conductance_spectra(state)
        } else {
            Vec::new()
        };
        let cap_spectra = if self.has_nonlinear_devices() {
            self.capacitance_spectra(state)
        } else {
            Vec::new()
        };

        let mut y = vec![vec![Complex64::new(0.0, 0.0); size]; size];

        for k_idx in 0..s {
            let k = sideband_min + k_idx as i32;
            let omega_k = 2.0 * PI * offset_hz + (k as f64) * omega0;

            for &(i, j, g) in &self.g_matrix {
                if i < n && j < n {
                    y[i * s + k_idx][j * s + k_idx] += g;
                }
            }
            for &(i, j, c) in &self.c_matrix {
                if i < n && j < n {
                    y[i * s + k_idx][j * s + k_idx] += Complex64::new(0.0, omega_k) * c;
                }
            }
            for &(i, j, l) in &self.l_matrix {
                if i < n && j < n && l.abs() > 1e-30 {
                    if omega_k.abs() <= omega_floor {
                        // DC sideband: short, consistent with the HB
                        // operating-point convention.
                        y[i * s + k_idx][j * s + k_idx] += 1e6;
                    } else {
                        y[i * s + k_idx][j * s + k_idx] +=
                            Complex64::new(0.0, -1.0 / (omega_k * l));
                    }
                }
            }
            // Small diagonal conductance keeps floating subnetworks solvable.
            for node in 0..n {
                y[node * s + k_idx][node * s + k_idx] += 1e-12;
            }
        }

        // Conversion coupling: Y[(i,k),(j,m)] += G_ij[k-m].
        for (i, j, spectrum) in &spectra {
            for k_idx in 0..s {
                for m_idx in 0..s {
                    let d = (k_idx as i32) - (m_idx as i32);
                    let d_abs = d.unsigned_abs() as usize;
                    if d_abs >= spectrum.len() {
                        continue;
                    }
                    let g = if d >= 0 {
                        spectrum[d_abs]
                    } else {
                        spectrum[d_abs].conj()
                    };
                    y[i * s + k_idx][j * s + m_idx] += g;
                }
            }
        }

        // Charge-storage coupling: Y[(i,k),(j,m)] += jw_k * C_ij[k-m] with
        // the signed output-sideband frequency in front, mirroring the
        // linear capacitors' block-diagonal jw_k entries.
        for (i, j, spectrum) in &cap_spectra {
            for k_idx in 0..s {
                let k = sideband_min + k_idx as i32;
                let omega_k = 2.0 * PI * offset_hz + (k as f64) * omega0;
                let jw = Complex64::new(0.0, omega_k);
                for m_idx in 0..s {
                    let d = (k_idx as i32) - (m_idx as i32);
                    let d_abs = d.unsigned_abs() as usize;
                    if d_abs >= spectrum.len() {
                        continue;
                    }
                    let c = if d >= 0 {
                        spectrum[d_abs]
                    } else {
                        spectrum[d_abs].conj()
                    };
                    y[i * s + k_idx][j * s + m_idx] += jw * c;
                }
            }
        }

        let mut results = Vec::with_capacity(excitations.len());
        for excitation in excitations {
            let mut rhs = vec![Complex64::new(0.0, 0.0); size];
            let m_idx = excitation.sideband - sideband_min;
            if m_idx < 0 || m_idx >= s as i32 {
                return Err(HbError::InvalidCircuit(format!(
                    "PAC excitation sideband {} outside [{}, {}]",
                    excitation.sideband, sideband_min, sideband_max
                )));
            }
            for &(node, amp) in &excitation.injections {
                if node < n {
                    rhs[node * s + m_idx as usize] += amp;
                }
            }

            let solution = self.solve_complex_linear_system(&y, &rhs)?;

            let mut by_node = vec![vec![Complex64::new(0.0, 0.0); s]; n];
            for node in 0..n {
                for k_idx in 0..s {
                    by_node[node][k_idx] = solution[node * s + k_idx];
                }
            }
            results.push(by_node);
        }

        Ok(results)
    }
}
