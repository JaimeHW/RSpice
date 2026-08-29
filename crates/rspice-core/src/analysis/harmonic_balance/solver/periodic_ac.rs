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
use std::collections::BTreeMap;
use std::f64::consts::PI;

type PeriodicSpectrum = (usize, usize, Vec<Complex64>);

/// One small-signal excitation column: current injections applied at a single
/// input sideband.
#[derive(Debug, Clone)]
pub struct PeriodicAcExcitation {
    /// Input sideband index m: the stimulus sits at `offset + m*f0`.
    pub sideband: i32,
    /// Current injected INTO each node: `(node index, phasor amplitude)`.
    pub injections: Vec<(usize, Complex64)>,
}

/// Sideband conversion admittance represented by sparse linear stamps and
/// periodic coupling spectra.  It supports both the forward PAC product and
/// the plain-transpose product required by the PNoise adjoint.
struct PeriodicConversionOperator<'a> {
    num_nodes: usize,
    num_sidebands: usize,
    sideband_min: i32,
    offset_hz: Value,
    omega0: Value,
    omega_floor: Value,
    g_matrix: &'a [(usize, usize, Value)],
    c_matrix: &'a [(usize, usize, Value)],
    l_matrix: &'a [(usize, usize, Value)],
    g_spectra: &'a [PeriodicSpectrum],
    c_spectra: &'a [PeriodicSpectrum],
}

impl PeriodicConversionOperator<'_> {
    #[inline]
    fn omega(&self, sideband_index: usize) -> Value {
        let k = self.sideband_min + sideband_index as i32;
        2.0 * PI * self.offset_hz + (k as Value) * self.omega0
    }

    #[inline]
    fn accumulate(
        &self,
        output: &mut [Complex64],
        input: &[Complex64],
        row: usize,
        column: usize,
        value: Complex64,
        transpose: bool,
    ) {
        if transpose {
            output[column] += value * input[row];
        } else {
            output[row] += value * input[column];
        }
    }

    fn visit_entries(&self, mut visitor: impl FnMut(usize, usize, Complex64)) {
        let n = self.num_nodes;
        let s = self.num_sidebands;
        for k_idx in 0..s {
            let omega_k = self.omega(k_idx);
            for &(i, j, g) in self.g_matrix {
                if i < n && j < n {
                    visitor(i * s + k_idx, j * s + k_idx, Complex64::new(g, 0.0));
                }
            }
            for &(i, j, c) in self.c_matrix {
                if i < n && j < n {
                    visitor(
                        i * s + k_idx,
                        j * s + k_idx,
                        Complex64::new(0.0, omega_k) * c,
                    );
                }
            }
            for &(i, j, l) in self.l_matrix {
                if i < n && j < n && l.abs() > 1e-30 {
                    let admittance = if omega_k.abs() <= self.omega_floor {
                        Complex64::new(1e6, 0.0)
                    } else {
                        Complex64::new(0.0, -1.0 / (omega_k * l))
                    };
                    visitor(i * s + k_idx, j * s + k_idx, admittance);
                }
            }
        }

        for &(i, j, ref spectrum) in self.g_spectra {
            for k_idx in 0..s {
                for m_idx in 0..s {
                    let d = k_idx as i32 - m_idx as i32;
                    let Some(&coefficient) = spectrum.get(d.unsigned_abs() as usize) else {
                        continue;
                    };
                    let coefficient = if d >= 0 {
                        coefficient
                    } else {
                        coefficient.conj()
                    };
                    visitor(i * s + k_idx, j * s + m_idx, coefficient);
                }
            }
        }
        for &(i, j, ref spectrum) in self.c_spectra {
            for k_idx in 0..s {
                let jw = Complex64::new(0.0, self.omega(k_idx));
                for m_idx in 0..s {
                    let d = k_idx as i32 - m_idx as i32;
                    let Some(&coefficient) = spectrum.get(d.unsigned_abs() as usize) else {
                        continue;
                    };
                    let coefficient = if d >= 0 {
                        coefficient
                    } else {
                        coefficient.conj()
                    };
                    visitor(i * s + k_idx, j * s + m_idx, jw * coefficient);
                }
            }
        }
    }

    fn apply_impl(&self, input: &[Complex64], transpose: bool) -> Vec<Complex64> {
        let size = self.num_nodes * self.num_sidebands;
        debug_assert_eq!(input.len(), size);
        let mut output = vec![Complex64::new(0.0, 0.0); size];
        self.visit_entries(|row, column, value| {
            self.accumulate(&mut output, input, row, column, value, transpose);
        });
        output
    }

    fn apply(&self, input: &[Complex64]) -> Vec<Complex64> {
        self.apply_impl(input, false)
    }

    fn apply_transpose(&self, input: &[Complex64]) -> Vec<Complex64> {
        self.apply_impl(input, true)
    }

    fn to_dense(&self) -> Vec<Vec<Complex64>> {
        let size = self.num_nodes * self.num_sidebands;
        let mut dense = vec![vec![Complex64::new(0.0, 0.0); size]; size];
        self.visit_entries(|row, column, value| dense[row][column] += value);
        dense
    }

    fn harmonic_blocks(&self, transpose: bool) -> Vec<Vec<Complex64>> {
        let n = self.num_nodes;
        let mut blocks = Vec::with_capacity(self.num_sidebands);
        for k_idx in 0..self.num_sidebands {
            let omega_k = self.omega(k_idx);
            let jw = Complex64::new(0.0, omega_k);
            let mut block = vec![Complex64::new(0.0, 0.0); n * n];
            for &(i, j, g) in self.g_matrix {
                if i < n && j < n {
                    block[i * n + j] += g;
                }
            }
            for &(i, j, c) in self.c_matrix {
                if i < n && j < n {
                    block[i * n + j] += jw * c;
                }
            }
            for &(i, j, l) in self.l_matrix {
                if i < n && j < n && l.abs() > 1e-30 {
                    block[i * n + j] += if omega_k.abs() <= self.omega_floor {
                        Complex64::new(1e6, 0.0)
                    } else {
                        Complex64::new(0.0, -1.0 / (omega_k * l))
                    };
                }
            }
            for &(i, j, ref spectrum) in self.g_spectra {
                if let Some(&coefficient) = spectrum.first() {
                    block[i * n + j] += coefficient;
                }
            }
            for &(i, j, ref spectrum) in self.c_spectra {
                if let Some(&coefficient) = spectrum.first() {
                    block[i * n + j] += jw * coefficient;
                }
            }
            if transpose {
                for i in 0..n {
                    for j in (i + 1)..n {
                        let a = i * n + j;
                        let b = j * n + i;
                        block.swap(a, b);
                    }
                }
            }
            blocks.push(block);
        }
        blocks
    }
}

struct PeriodicBlockPreconditioner {
    num_nodes: usize,
    num_sidebands: usize,
    factors: Vec<super::krylov::LuFactors>,
}

impl PeriodicBlockPreconditioner {
    fn build(operator: &PeriodicConversionOperator<'_>, transpose: bool) -> Self {
        let factors = operator
            .harmonic_blocks(transpose)
            .into_iter()
            .map(|block| super::krylov::LuFactors::factor(block, operator.num_nodes))
            .collect();
        Self {
            num_nodes: operator.num_nodes,
            num_sidebands: operator.num_sidebands,
            factors,
        }
    }
}

impl super::krylov::KrylovPreconditioner for PeriodicBlockPreconditioner {
    fn apply(&self, r: &[Complex64]) -> Vec<Complex64> {
        let mut output = r.to_vec();
        let mut block = vec![Complex64::new(0.0, 0.0); self.num_nodes];
        for k_idx in 0..self.num_sidebands {
            for node in 0..self.num_nodes {
                block[node] = r[node * self.num_sidebands + k_idx];
            }
            self.factors[k_idx].solve_in_place(&mut block);
            for node in 0..self.num_nodes {
                output[node * self.num_sidebands + k_idx] = block[node];
            }
        }
        output
    }
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
    ) -> Result<Vec<(usize, usize, Vec<Complex64>)>, HbError> {
        let n = self.num_nodes;
        let n_time = self.fft.size();

        let v_time: Vec<Vec<Value>> = (0..n)
            .map(|node| self.fft.to_time_domain(&state.x[node]))
            .collect();

        // Device Jacobians are sparse.  Keeping only entries that devices
        // actually stamp avoids the previous O(nodes^2 * time-points)
        // tensor, which was the dominant PAC/HB linearization allocation on
        // large sparse circuits.  BTreeMap preserves deterministic ordering.
        let mut g_time: BTreeMap<(usize, usize), Vec<Value>> = BTreeMap::new();

        if !self.nonlinear_devices.is_empty() {
            let mut node_voltages = vec![0.0; n];
            for t in 0..n_time {
                for node in 0..n {
                    node_voltages[node] = v_time[node][t];
                }
                for device in &self.nonlinear_devices {
                    for ((i, j), g) in device.jacobian(&node_voltages) {
                        if i < n && j < n {
                            g_time.entry((i, j)).or_insert_with(|| vec![0.0; n_time])[t] += g;
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
                    let jac_entries =
                        device.try_compute_jacobian("periodic conductance evaluation")?;
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
                            g_time.entry((i, j)).or_insert_with(|| vec![0.0; n_time])[t] +=
                                entry.value;
                        }
                    }
                }
            }
        }

        Ok(g_time
            .into_iter()
            .filter_map(|((i, j), waveform)| {
                let max_g: Value = waveform.iter().fold(0.0, |a, &b| a.max(b.abs()));
                (max_g >= 1e-30).then(|| {
                    let spectrum = self.fft.to_frequency_domain_n(&waveform, harmonic_count);
                    (i, j, spectrum)
                })
            })
            .collect())
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

        let mut c_time: BTreeMap<(usize, usize), Vec<Value>> = BTreeMap::new();
        let mut node_voltages = vec![0.0; n];
        for t in 0..n_time {
            for node in 0..n {
                node_voltages[node] = v_time[node][t];
            }
            for device in &self.nonlinear_devices {
                for ((i, j), c) in device.charge_jacobian(&node_voltages) {
                    if i < n && j < n {
                        c_time.entry((i, j)).or_insert_with(|| vec![0.0; n_time])[t] += c;
                    }
                }
            }
        }

        c_time
            .into_iter()
            .filter_map(|((i, j), waveform)| {
                let max_c: Value = waveform.iter().fold(0.0, |a, &b| a.max(b.abs()));
                (max_c >= 1e-30).then(|| {
                    let spectrum = self.fft.to_frequency_domain_n(&waveform, harmonic_count);
                    (i, j, spectrum)
                })
            })
            .collect()
    }

    /// Solve the sideband-coupled small-signal system at one offset frequency.
    ///
    /// Unknowns are `V[(node, k)]` for k in `[sideband_min, sideband_max]` at
    /// SIGNED absolute frequencies `f_k = offset + k*f0`. Each excitation is
    /// solved against the same admittance matrix; the result is indexed
    /// `[excitation][node][sideband - sideband_min]`.
    pub(crate) fn solve_periodic_ac(
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

        // Dense direct elimination is still faster for small systems.  Large
        // systems build only sparse spectra and per-sideband preconditioner
        // blocks; the dense matrix is materialized lazily only if GMRES fails.
        let try_krylov = self.config.use_krylov || size >= super::krylov::KRYLOV_AUTO_THRESHOLD;
        let mut dense = if try_krylov {
            None
        } else {
            Some(self.assemble_conversion_matrix(state, offset_hz, sideband_min, sideband_max)?)
        };

        let span = (sideband_max - sideband_min).unsigned_abs() as usize;
        let (spectra, cap_spectra) = if try_krylov && self.has_nonlinear_devices() {
            (
                self.conductance_spectra(state, span.max(self.num_harmonics))?,
                self.capacitance_spectra(state, span.max(self.num_harmonics)),
            )
        } else {
            (Vec::new(), Vec::new())
        };
        let omega0 = 2.0 * PI * self.config.fundamental_freq;
        let operator = PeriodicConversionOperator {
            num_nodes: n,
            num_sidebands: s,
            sideband_min,
            offset_hz,
            omega0,
            omega_floor: omega0 * 1e-12,
            g_matrix: &self.g_matrix,
            c_matrix: &self.c_matrix,
            l_matrix: &self.l_matrix,
            g_spectra: &spectra,
            c_spectra: &cap_spectra,
        };
        let preconditioner =
            try_krylov.then(|| PeriodicBlockPreconditioner::build(&operator, false));

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

            let solution = if let Some(preconditioner) = &preconditioner {
                let restart = self.config.gmres_restart.clamp(8, size.max(8));
                let outcome = super::krylov::gmres(
                    &|input| operator.apply(input),
                    preconditioner,
                    &rhs,
                    restart,
                    6,
                );
                if outcome.converged {
                    if self.config.verbose {
                        log::debug!(
                            "PAC matrix-free solve: {} iterations, relative residual {:.2e}",
                            outcome.iterations,
                            outcome.relative_residual
                        );
                    }
                    outcome.solution
                } else {
                    log::debug!(
                        "PAC matrix-free solve stagnated after {} iterations (relative residual \
                         {:.2e}); falling back to dense elimination",
                        outcome.iterations,
                        outcome.relative_residual
                    );
                    let y = dense.get_or_insert_with(|| operator.to_dense());
                    self.solve_complex_linear_system(y, &rhs)?
                }
            } else {
                self.solve_complex_linear_system(dense.as_ref().expect("dense PAC matrix"), &rhs)?
            };

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
        let omega0 = 2.0 * PI * self.config.fundamental_freq;
        // Below this magnitude a sideband frequency counts as DC for the
        // inductor admittance, mirroring the operating-point treatment.
        let omega_floor = omega0 * 1e-12;

        let span = (sideband_max - sideband_min).unsigned_abs() as usize;
        let spectra = if self.has_nonlinear_devices() {
            self.conductance_spectra(state, span.max(self.num_harmonics))?
        } else {
            Vec::new()
        };
        let cap_spectra = if self.has_nonlinear_devices() {
            self.capacitance_spectra(state, span.max(self.num_harmonics))
        } else {
            Vec::new()
        };

        Ok(PeriodicConversionOperator {
            num_nodes: n,
            num_sidebands: s,
            sideband_min,
            offset_hz,
            omega0,
            omega_floor,
            g_matrix: &self.g_matrix,
            c_matrix: &self.c_matrix,
            l_matrix: &self.l_matrix,
            g_spectra: &spectra,
            c_spectra: &cap_spectra,
        }
        .to_dense())
    }

    /// Periodically modulated white-noise PSDs of the registered nonlinear
    /// devices around the converged operating point.
    ///
    /// Each entry is a current-noise source between two nodes whose
    /// time-varying intensity s(t) >= 0 (A^2/Hz) is returned as Fourier
    /// coefficients on the HB grid: shot noise `2q|I(t)|` for junction
    /// currents and channel thermal noise `(8/3)kT|gm(t)|` for FETs.
    pub(crate) fn device_noise_sources(
        &mut self,
        state: &HbSolverState,
        temperature: Value,
    ) -> Vec<PeriodicNoiseSource> {
        use crate::constants::K_BOLTZMANN as K_B;
        use crate::constants::Q_ELECTRON as Q_E;

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
    /// `sum_{k,m} A_k conj(A_m) S_{k-m}` with `A_k` the adjoint gain across
    /// its terminals at sideband k and `S_d` the Fourier coefficients of its
    /// intensity (`S_{-d} = conj(S_d)`). Stationary sources reduce to the
    /// textbook folding `S0 * sum_k |A_k|^2`.
    pub(crate) fn solve_periodic_noise(
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
        let s = (sideband_max - sideband_min + 1) as usize;
        let size = n * s;

        let try_krylov = self.config.use_krylov || size >= super::krylov::KRYLOV_AUTO_THRESHOLD;
        let span = (sideband_max - sideband_min).unsigned_abs() as usize;
        let (spectra, cap_spectra) = if try_krylov && self.has_nonlinear_devices() {
            (
                self.conductance_spectra(state, span.max(self.num_harmonics))?,
                self.capacitance_spectra(state, span.max(self.num_harmonics)),
            )
        } else {
            (Vec::new(), Vec::new())
        };
        let omega0 = 2.0 * PI * self.config.fundamental_freq;
        let operator = PeriodicConversionOperator {
            num_nodes: n,
            num_sidebands: s,
            sideband_min,
            offset_hz,
            omega0,
            omega_floor: omega0 * 1e-12,
            g_matrix: &self.g_matrix,
            c_matrix: &self.c_matrix,
            l_matrix: &self.l_matrix,
            g_spectra: &spectra,
            c_spectra: &cap_spectra,
        };

        // Adjoint solve with the plain (unconjugated) transpose.
        let out_idx = (0 - sideband_min) as usize; // k = 0 entry
        let mut e = vec![Complex64::new(0.0, 0.0); size];
        e[output_node * s + out_idx] = Complex64::new(1.0, 0.0);
        if let Some(r) = output_ref {
            e[r * s + out_idx] -= Complex64::new(1.0, 0.0);
        }
        let adjoint = if try_krylov {
            let preconditioner = PeriodicBlockPreconditioner::build(&operator, true);
            let restart = self.config.gmres_restart.clamp(8, size.max(8));
            let outcome = super::krylov::gmres(
                &|input| operator.apply_transpose(input),
                &preconditioner,
                &e,
                restart,
                6,
            );
            if outcome.converged {
                if self.config.verbose {
                    log::debug!(
                        "PNoise matrix-free adjoint: {} iterations, relative residual {:.2e}",
                        outcome.iterations,
                        outcome.relative_residual
                    );
                }
                outcome.solution
            } else {
                log::debug!(
                    "PNoise matrix-free adjoint stagnated after {} iterations (relative residual \
                     {:.2e}); falling back to dense elimination",
                    outcome.iterations,
                    outcome.relative_residual
                );
                let y = operator.to_dense();
                let mut yt = vec![vec![Complex64::new(0.0, 0.0); size]; size];
                for r in 0..size {
                    for c in 0..size {
                        yt[c][r] = y[r][c];
                    }
                }
                self.solve_complex_linear_system(&yt, &e)?
            }
        } else {
            let y =
                self.assemble_conversion_matrix(state, offset_hz, sideband_min, sideband_max)?;
            let mut yt = vec![vec![Complex64::new(0.0, 0.0); size]; size];
            for r in 0..size {
                for c in 0..size {
                    yt[c][r] = y[r][c];
                }
            }
            self.solve_complex_linear_system(&yt, &e)?
        };

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
                    contribution += gains[k_idx] * gains[m_idx].conj() * s_d;
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

#[cfg(test)]
mod matrix_free_tests {
    use super::*;

    fn test_operator<'a>(
        g: &'a [(usize, usize, Value)],
        c: &'a [(usize, usize, Value)],
        spectra: &'a [PeriodicSpectrum],
        cap_spectra: &'a [PeriodicSpectrum],
    ) -> PeriodicConversionOperator<'a> {
        PeriodicConversionOperator {
            num_nodes: 2,
            num_sidebands: 5,
            sideband_min: -2,
            offset_hz: 3.0e6,
            omega0: 2.0 * PI * 1.0e6,
            omega_floor: 2.0 * PI * 1.0e-6,
            g_matrix: g,
            c_matrix: c,
            l_matrix: &[],
            g_spectra: spectra,
            c_spectra: cap_spectra,
        }
    }

    fn assert_close(actual: Complex64, expected: Complex64) {
        let scale = actual.norm().max(expected.norm()).max(1.0);
        assert!(
            (actual - expected).norm() <= 2e-12 * scale,
            "actual={actual:?}, expected={expected:?}"
        );
    }

    #[test]
    fn conversion_operator_forward_and_plain_transpose_match_dense() {
        let g = vec![(0, 0, 10.0), (0, 1, -1.0), (1, 0, -0.5), (1, 1, 8.0)];
        let c = vec![(0, 0, 2e-12), (1, 1, 3e-12)];
        let spectra = vec![
            (
                0,
                0,
                vec![
                    Complex64::new(0.7, 0.0),
                    Complex64::new(0.03, -0.02),
                    Complex64::new(-0.01, 0.005),
                    Complex64::new(0.004, 0.002),
                    Complex64::new(0.001, -0.001),
                ],
            ),
            (
                1,
                0,
                vec![
                    Complex64::new(-0.2, 0.0),
                    Complex64::new(0.01, 0.015),
                    Complex64::new(0.003, -0.002),
                    Complex64::new(0.0, 0.001),
                    Complex64::new(-0.0005, 0.0),
                ],
            ),
        ];
        let cap_spectra = vec![(
            1,
            1,
            vec![
                Complex64::new(1.5e-12, 0.0),
                Complex64::new(0.1e-12, -0.05e-12),
                Complex64::new(0.02e-12, 0.01e-12),
                Complex64::new(0.0, 0.005e-12),
                Complex64::new(0.0, 0.0),
            ],
        )];
        let operator = test_operator(&g, &c, &spectra, &cap_spectra);
        let dense = operator.to_dense();
        let x = (0..10)
            .map(|index| Complex64::new(index as Value * 0.13 - 0.4, 0.2 - index as Value * 0.07))
            .collect::<Vec<_>>();
        let forward = operator.apply(&x);
        let transpose = operator.apply_transpose(&x);
        for row in 0..10 {
            let expected_forward = (0..10).map(|col| dense[row][col] * x[col]).sum();
            let expected_transpose = (0..10).map(|col| dense[col][row] * x[col]).sum();
            assert_close(forward[row], expected_forward);
            assert_close(transpose[row], expected_transpose);
        }
    }

    #[test]
    fn conversion_operator_contains_only_physical_conductance() {
        let g = vec![(0, 0, 1.0e-18), (1, 1, 2.0e-18)];
        let operator = test_operator(&g, &[], &[], &[]);
        let dense = operator.to_dense();

        for sideband in 0..operator.num_sidebands {
            assert_eq!(dense[sideband][sideband], Complex64::new(1.0e-18, 0.0));
            let node_1 = operator.num_sidebands + sideband;
            assert_eq!(dense[node_1][node_1], Complex64::new(2.0e-18, 0.0));
        }
    }

    #[test]
    fn matrix_free_conversion_solve_matches_direct_lu() {
        let g = vec![(0, 0, 10.0), (0, 1, -1.0), (1, 0, -0.5), (1, 1, 8.0)];
        let c = vec![(0, 0, 2e-12), (1, 1, 3e-12)];
        let spectra = vec![(
            0,
            0,
            vec![
                Complex64::new(0.7, 0.0),
                Complex64::new(0.01, -0.005),
                Complex64::new(0.003, 0.001),
                Complex64::new(0.001, 0.0),
                Complex64::new(0.0002, 0.0),
            ],
        )];
        let operator = test_operator(&g, &c, &spectra, &[]);
        let rhs = (0..10)
            .map(|index| Complex64::new(0.1 + index as Value * 0.03, -0.02 * index as Value))
            .collect::<Vec<_>>();
        let preconditioner = PeriodicBlockPreconditioner::build(&operator, false);
        let outcome = super::super::krylov::gmres(
            &|input| operator.apply(input),
            &preconditioner,
            &rhs,
            10,
            6,
        );
        assert!(outcome.converged, "relative={}", outcome.relative_residual);

        let dense = operator.to_dense();
        let mut flat = Vec::with_capacity(100);
        for row in &dense {
            flat.extend_from_slice(row);
        }
        let factors = super::super::krylov::LuFactors::factor(flat, 10);
        let mut direct = rhs.clone();
        factors.solve_in_place(&mut direct);
        for (actual, expected) in outcome.solution.into_iter().zip(direct) {
            assert_close(actual, expected);
        }
    }
}
