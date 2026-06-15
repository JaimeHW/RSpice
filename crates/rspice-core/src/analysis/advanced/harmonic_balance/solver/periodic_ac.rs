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
        harmonic_count: usize,
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
                let spectrum = self
                    .fft
                    .to_frequency_domain_n(&g_time[i][j], harmonic_count);
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
        harmonic_count: usize,
    ) -> Vec<(usize, usize, Vec<Complex64>)> {
        let n = self.num_nodes;
        if !self
            .nonlinear_devices
            .iter()
            .any(|d| d.has_charge_storage())
        {
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
                let spectrum = self
                    .fft
                    .to_frequency_domain_n(&c_time[i][j], harmonic_count);
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
        let y = self.assemble_conversion_matrix(state, offset_hz, sideband_min, sideband_max)?;
        let n = self.num_nodes;
        let s = (sideband_max - sideband_min + 1) as usize;
        let size = n * s;

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

    /// Assemble the sideband-coupled small-signal admittance matrix at one
    /// offset frequency: linear elements on the block diagonal at the signed
    /// frequencies `offset + k*f0`, periodic conductance and capacitance
    /// conversion coupling on the Toeplitz off-blocks.
    fn assemble_conversion_matrix(
        &mut self,
        state: &HbSolverState,
        offset_hz: Value,
        sideband_min: i32,
        sideband_max: i32,
    ) -> Result<Vec<Vec<Complex64>>, HbError> {
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

        let span = (sideband_max - sideband_min).unsigned_abs() as usize;
        let spectra = if self.has_nonlinear_devices() {
            self.conductance_spectra(state, span.max(self.num_harmonics))
        } else {
            Vec::new()
        };
        let cap_spectra = if self.has_nonlinear_devices() {
            self.capacitance_spectra(state, span.max(self.num_harmonics))
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

        Ok(y)
    }

    /// Periodically modulated white-noise PSDs of the registered nonlinear
    /// devices around the converged operating point.
    ///
    /// Each entry is a current-noise source between two nodes whose
    /// time-varying intensity s(t) >= 0 (A^2/Hz) is returned as Fourier
    /// coefficients on the HB grid: shot noise `2q|I(t)|` for junction
    /// currents and channel thermal noise `(8/3)kT|gm(t)|` for FETs.
    pub fn device_noise_sources(
        &mut self,
        state: &HbSolverState,
        temperature: Value,
    ) -> Vec<PeriodicNoiseSource> {
        const Q_E: Value = 1.602176634e-19;
        const K_B: Value = 1.380649e-23;

        let n = self.num_nodes;
        let n_time = self.fft.size();
        let v_time: Vec<Vec<Value>> = (0..n)
            .map(|node| self.fft.to_time_domain(&state.x[node]))
            .collect();

        // Accumulate per-source intensity waveforms keyed by node pair.
        let mut intensities: Vec<((usize, usize), String, Vec<Value>)> = Vec::new();
        let mut node_voltages = vec![0.0; n];

        for (d_idx, device) in self.nonlinear_devices.iter().enumerate() {
            let branches = device.noise_branches();
            if branches.is_empty() {
                continue;
            }
            let base = intensities.len();
            for (b, &(p, q)) in branches.iter().enumerate() {
                let label = format!(
                    "{:?}#{d_idx} {}",
                    device.device_type,
                    device.noise_branch_label(b)
                );
                intensities.push(((p, q), label, vec![0.0; n_time]));
            }
            for t in 0..n_time {
                for node in 0..n {
                    node_voltages[node] = v_time[node][t];
                }
                let values = device.noise_intensities(&node_voltages, temperature, Q_E, K_B);
                for (b, value) in values.iter().enumerate() {
                    intensities[base + b].2[t] = *value;
                }
            }
        }

        intensities
            .into_iter()
            .map(|((p, q), name, waveform)| {
                let psd = self.fft.to_frequency_domain(&waveform);
                PeriodicNoiseSource {
                    name,
                    node_pos: p,
                    node_neg: q,
                    psd,
                    flicker: None,
                }
            })
            .collect()
    }

    /// Per-source output noise power spectral densities (V^2/Hz) at one
    /// offset frequency; the total is the sum (sources are independent).
    ///
    /// One adjoint solve `Y^T a = e_out` gives the transfer from a unit
    /// current injected at every (node, sideband) to the output at the
    /// analysis frequency — `e_out` carries +1 at the positive output node
    /// and -1 at the reference for differential outputs. Each periodically
    /// modulated white source then contributes
    /// `sum_{k,m} conj(A_k) A_m S_{k-m}` with `A_k` the adjoint gain across
    /// its terminals at sideband k and `S_d` the Fourier coefficients of its
    /// intensity (`S_{-d} = conj(S_d)`). Stationary sources reduce to the
    /// textbook folding `S0 * sum_k |A_k|^2`.
    pub fn solve_periodic_noise(
        &mut self,
        state: &HbSolverState,
        offset_hz: Value,
        sideband_min: i32,
        sideband_max: i32,
        output_node: usize,
        output_ref: Option<usize>,
        sources: &[PeriodicNoiseSource],
    ) -> Result<Vec<Value>, HbError> {
        let n = self.num_nodes;
        if output_node >= n {
            return Err(HbError::InvalidCircuit(
                "pnoise output node out of range".to_string(),
            ));
        }
        if let Some(r) = output_ref
            && r >= n
        {
            return Err(HbError::InvalidCircuit(
                "pnoise output reference node out of range".to_string(),
            ));
        }
        if sideband_min > 0 || sideband_max < 0 {
            return Err(HbError::InvalidCircuit(
                "pnoise sideband range must include 0 (the analysis frequency)".to_string(),
            ));
        }
        let y = self.assemble_conversion_matrix(state, offset_hz, sideband_min, sideband_max)?;
        let s = (sideband_max - sideband_min + 1) as usize;
        let size = n * s;

        // Adjoint solve with the plain (unconjugated) transpose.
        let mut yt = vec![vec![Complex64::new(0.0, 0.0); size]; size];
        for r in 0..size {
            for c in 0..size {
                yt[c][r] = y[r][c];
            }
        }
        let out_idx = (0 - sideband_min) as usize; // k = 0 entry
        let mut e = vec![Complex64::new(0.0, 0.0); size];
        e[output_node * s + out_idx] = Complex64::new(1.0, 0.0);
        if let Some(r) = output_ref {
            e[r * s + out_idx] -= Complex64::new(1.0, 0.0);
        }
        let adjoint = self.solve_complex_linear_system(&yt, &e)?;

        let mut contributions = Vec::with_capacity(sources.len());
        for source in sources {
            // Adjoint gain across the source terminals per sideband.
            let mut gains = vec![Complex64::new(0.0, 0.0); s];
            for (k_idx, gain) in gains.iter_mut().enumerate() {
                let mut a = Complex64::new(0.0, 0.0);
                if source.node_pos < n {
                    a += adjoint[source.node_pos * s + k_idx];
                }
                if source.node_neg < n {
                    a -= adjoint[source.node_neg * s + k_idx];
                }
                *gain = a;
            }

            let mut contribution = Complex64::new(0.0, 0.0);
            for k_idx in 0..s {
                for m_idx in 0..s {
                    let d = (k_idx as i32) - (m_idx as i32);
                    let d_abs = d.unsigned_abs() as usize;
                    if d_abs >= source.psd.len() {
                        continue;
                    }
                    let s_d = if d >= 0 {
                        source.psd[d_abs]
                    } else {
                        source.psd[d_abs].conj()
                    };
                    contribution += gains[k_idx].conj() * gains[m_idx] * s_d;
                }
            }

            // Stationary flicker folding: the colored density is sampled at
            // each sideband's absolute frequency and folds through |A_k|^2
            // (no sideband correlation for a stationary source).
            if let Some((coeff, ef)) = source.flicker {
                let omega0_hz = self.config.fundamental_freq;
                for (k_idx, gain) in gains.iter().enumerate() {
                    let k = sideband_min + k_idx as i32;
                    let f_abs = (offset_hz + (k as f64) * omega0_hz).abs().max(1e-3);
                    contribution += gain.norm_sqr() * coeff / f_abs.powf(ef);
                }
            }
            // The double sum is Hermitian by construction; numerical
            // round-off leaves a vanishing imaginary part.
            contributions.push(contribution.re.max(0.0));
        }

        Ok(contributions)
    }
}

/// One periodically modulated white current-noise source.
#[derive(Debug, Clone)]
pub struct PeriodicNoiseSource {
    /// Display label for contributor reporting.
    pub name: String,
    /// Terminal the noise current flows into (solver node index; `usize::MAX`
    /// or any out-of-range value means ground).
    pub node_pos: usize,
    /// Terminal the noise current flows out of.
    pub node_neg: usize,
    /// Fourier coefficients of the intensity s(t) >= 0 in A^2/Hz, indexed by
    /// harmonic (c-convention; negative harmonics by conjugation).
    pub psd: Vec<Complex64>,
    /// Stationary flicker term `(coefficient, frequency exponent)`: adds
    /// `coefficient / |f|^exponent` evaluated at each sideband's absolute
    /// frequency. The coefficient already folds the bias dependence
    /// (KF * |I_dc|^AF); bias modulation of 1/f noise is approximated by the
    /// periodic average, the standard folding treatment.
    pub flicker: Option<(Value, Value)>,
}
