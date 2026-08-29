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

fn scaled_complex_product3(
    first: Complex64,
    second: Complex64,
    third: Complex64,
    binary_scale_exponent: i32,
) -> Result<Complex64, &'static str> {
    let factors = [first, second, third];
    if factors
        .iter()
        .any(|value| !value.re.is_finite() || !value.im.is_finite())
    {
        return Err("a factor is non-finite");
    }
    if factors
        .iter()
        .any(|value| value.re == 0.0 && value.im == 0.0)
    {
        return Ok(Complex64::new(0.0, 0.0));
    }

    let mut scaled = [Complex64::new(0.0, 0.0); 3];
    let mut exponent = binary_scale_exponent;
    for (slot, value) in scaled.iter_mut().zip(factors) {
        let scale = value.re.abs().max(value.im.abs());
        let factor_exponent = libm::ilogb(scale);
        exponent = exponent
            .checked_add(factor_exponent)
            .ok_or("the product exponent exceeds this platform")?;
        *slot = Complex64::new(
            libm::scalbn(value.re, -factor_exponent),
            libm::scalbn(value.im, -factor_exponent),
        );
    }

    let mantissa = scaled[0] * scaled[1] * scaled[2];
    if !mantissa.re.is_finite() || !mantissa.im.is_finite() {
        return Err("the normalized product is non-finite");
    }
    let mantissa_scale = mantissa.re.abs().max(mantissa.im.abs());
    if mantissa_scale == 0.0 {
        return Err("a nonzero product vanished during normalized multiplication");
    }
    let mantissa_exponent = libm::ilogb(mantissa_scale);
    exponent = exponent
        .checked_add(mantissa_exponent)
        .ok_or("the normalized product exponent exceeds this platform")?;
    let normalized = Complex64::new(
        libm::scalbn(mantissa.re, -mantissa_exponent),
        libm::scalbn(mantissa.im, -mantissa_exponent),
    );
    let product = Complex64::new(
        libm::scalbn(normalized.re, exponent),
        libm::scalbn(normalized.im, exponent),
    );
    if !product.re.is_finite() || !product.im.is_finite() {
        return Err("the product is not representable as finite complex components");
    }
    if product.re == 0.0 && product.im == 0.0 {
        return Err("the nonzero product is below the representable complex range");
    }
    Ok(product)
}

fn scaled_flicker_density(
    gain: Complex64,
    coefficient: Value,
    coefficient_binary_exponent: i32,
    frequency: Value,
    exponent: Value,
) -> Result<Value, &'static str> {
    if !gain.re.is_finite()
        || !gain.im.is_finite()
        || !coefficient.is_finite()
        || coefficient < 0.0
        || !frequency.is_finite()
        || frequency < 0.0
        || !exponent.is_finite()
    {
        return Err("a flicker-density factor is invalid");
    }
    let gain_scale = gain.re.abs().max(gain.im.abs());
    if coefficient == 0.0 || gain_scale == 0.0 {
        return Ok(0.0);
    }
    let gain_exponent = libm::ilogb(gain_scale);
    let normalized_gain = Complex64::new(
        libm::scalbn(gain.re, -gain_exponent),
        libm::scalbn(gain.im, -gain_exponent),
    );
    let normalized_magnitude = normalized_gain.norm();
    if !normalized_magnitude.is_finite() || normalized_magnitude <= 0.0 {
        return Err("the normalized flicker transfer magnitude is invalid");
    }
    let gain_log2 = Value::from(gain_exponent) + libm::log2(normalized_magnitude);
    if frequency == 0.0 {
        return if exponent < 0.0 {
            Ok(0.0)
        } else if exponent == 0.0 {
            scaled_flicker_density(
                gain,
                coefficient,
                coefficient_binary_exponent,
                1.0,
                exponent,
            )
        } else {
            Err("positive-exponent flicker density is singular at zero frequency")
        };
    }

    let frequency_term = if exponent == 0.0 {
        0.0
    } else {
        exponent * libm::log2(frequency)
    };
    let log2_density =
        2.0 * gain_log2 + libm::log2(coefficient) + Value::from(coefficient_binary_exponent)
            - frequency_term;
    if !log2_density.is_finite() {
        return Err("the flicker-density exponent is non-finite");
    }
    let binary_exponent = libm::floor(log2_density);
    if binary_exponent < i32::MIN as Value || binary_exponent > i32::MAX as Value {
        return Err("the flicker-density exponent exceeds this platform");
    }
    let mantissa = libm::exp2(log2_density - binary_exponent);
    let density = libm::scalbn(mantissa, binary_exponent as i32);
    if !density.is_finite() || density <= 0.0 {
        return Err("the nonzero flicker density is not representable");
    }
    Ok(density)
}

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
                        Complex64::new(inductor_dc_short_admittance(l), 0.0)
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

    /// Materialize only the plain transpose needed by the PNoise adjoint.
    ///
    /// Building `Y` and then copying it into a second `Y^T` allocation doubles
    /// the peak quadratic storage of the small-system recovery path.  Stamping
    /// entries directly at `(column, row)` retains duplicate-stamp summation
    /// while allocating exactly one dense operator.
    fn to_dense_transpose(&self) -> Vec<Vec<Complex64>> {
        let size = self.num_nodes * self.num_sidebands;
        let mut transpose = vec![vec![Complex64::new(0.0, 0.0); size]; size];
        self.visit_entries(|row, column, value| transpose[column][row] += value);
        transpose
    }

    fn try_harmonic_block(&self, k_idx: usize, transpose: bool) -> Result<Vec<Complex64>, HbError> {
        let n = self.num_nodes;
        let block_entries = n.checked_mul(n).ok_or_else(|| {
            HbError::InvalidCircuit(
                "periodic preconditioner block dimension overflows usize".to_string(),
            )
        })?;
        let mut block = Vec::new();
        block.try_reserve_exact(block_entries).map_err(|error| {
            HbError::InvalidCircuit(format!(
                "periodic preconditioner block allocation failed: {error}"
            ))
        })?;
        block.resize(block_entries, Complex64::new(0.0, 0.0));
        let omega_k = self.omega(k_idx);
        let jw = Complex64::new(0.0, omega_k);
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
                    Complex64::new(inductor_dc_short_admittance(l), 0.0)
                } else {
                    Complex64::new(0.0, -1.0 / (omega_k * l))
                };
            }
        }
        for &(i, j, ref spectrum) in self.g_spectra {
            if i < n
                && j < n
                && let Some(&coefficient) = spectrum.first()
            {
                block[i * n + j] += coefficient;
            }
        }
        for &(i, j, ref spectrum) in self.c_spectra {
            if i < n
                && j < n
                && let Some(&coefficient) = spectrum.first()
            {
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
        Ok(block)
    }
}

struct PeriodicBlockPreconditioner {
    num_nodes: usize,
    num_sidebands: usize,
    factors: Vec<super::krylov::LuFactors>,
}

impl PeriodicBlockPreconditioner {
    fn try_build(
        operator: &PeriodicConversionOperator<'_>,
        transpose: bool,
    ) -> Result<Self, HbError> {
        let mut factors = Vec::new();
        factors
            .try_reserve_exact(operator.num_sidebands)
            .map_err(|error| {
                HbError::InvalidCircuit(format!(
                    "periodic block-preconditioner allocation failed: {error}"
                ))
            })?;
        for k_idx in 0..operator.num_sidebands {
            let block = operator.try_harmonic_block(k_idx, transpose)?;
            factors.push(super::krylov::LuFactors::factor(block, operator.num_nodes));
        }
        Ok(Self {
            num_nodes: operator.num_nodes,
            num_sidebands: operator.num_sidebands,
            factors,
        })
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

struct PeriodicDiagonalPreconditioner {
    inverse_diagonal: Vec<Complex64>,
}

impl PeriodicDiagonalPreconditioner {
    fn try_build(operator: &PeriodicConversionOperator<'_>) -> Result<Self, HbError> {
        let size = operator
            .num_nodes
            .checked_mul(operator.num_sidebands)
            .ok_or_else(|| {
                HbError::InvalidCircuit(
                    "periodic diagonal-preconditioner dimension overflows usize".to_string(),
                )
            })?;
        let mut diagonal = Vec::new();
        diagonal.try_reserve_exact(size).map_err(|error| {
            HbError::InvalidCircuit(format!(
                "periodic diagonal-preconditioner allocation failed: {error}"
            ))
        })?;
        diagonal.resize(size, Complex64::new(0.0, 0.0));

        let mut invalid_entry = None;
        operator.visit_entries(|row, column, value| {
            if row >= size || column >= size {
                invalid_entry.get_or_insert((row, column));
            } else if row == column {
                diagonal[row] += value;
            }
        });
        if let Some((row, column)) = invalid_entry {
            return Err(HbError::InvalidCircuit(format!(
                "periodic preconditioner entry ({row}, {column}) is outside its {size}x{size} operator"
            )));
        }
        if let Some((index, value)) = diagonal
            .iter()
            .copied()
            .enumerate()
            .find(|(_, value)| !value.re.is_finite() || !value.im.is_finite())
        {
            return Err(HbError::InvalidCircuit(format!(
                "periodic diagonal-preconditioner entry {index} is non-finite after stamp accumulation ({:+.6e}{:+.6e}j)",
                value.re, value.im
            )));
        }

        let one = Complex64::new(1.0, 0.0);
        for value in &mut diagonal {
            let inverse = if *value == Complex64::new(0.0, 0.0) {
                one
            } else {
                let candidate = one / *value;
                if candidate.re.is_finite() && candidate.im.is_finite() {
                    candidate
                } else {
                    one
                }
            };
            *value = inverse;
        }
        Ok(Self {
            inverse_diagonal: diagonal,
        })
    }
}

impl super::krylov::KrylovPreconditioner for PeriodicDiagonalPreconditioner {
    fn apply(&self, residual: &[Complex64]) -> Vec<Complex64> {
        if residual.len() != self.inverse_diagonal.len() {
            return residual.to_vec();
        }
        residual
            .iter()
            .zip(&self.inverse_diagonal)
            .map(|(&value, &inverse)| {
                let scaled = value * inverse;
                if scaled.re.is_finite() && scaled.im.is_finite() {
                    scaled
                } else {
                    value
                }
            })
            .collect()
    }
}

enum PeriodicPreconditioner {
    Block(PeriodicBlockPreconditioner),
    Diagonal(PeriodicDiagonalPreconditioner),
}

impl PeriodicPreconditioner {
    fn build(operator: &PeriodicConversionOperator<'_>, transpose: bool) -> Result<Self, HbError> {
        let block_entries = operator
            .num_nodes
            .checked_mul(operator.num_nodes)
            .and_then(|per_block| per_block.checked_mul(operator.num_sidebands));
        let dense_entry_limit = super::krylov::KRYLOV_AUTO_THRESHOLD
            .checked_mul(super::krylov::KRYLOV_AUTO_THRESHOLD)
            .ok_or_else(|| {
                HbError::InvalidCircuit(
                    "periodic preconditioner dense-entry limit overflows usize".to_string(),
                )
            })?;
        if block_entries.is_some_and(|entries| entries < dense_entry_limit) {
            return PeriodicBlockPreconditioner::try_build(operator, transpose).map(Self::Block);
        }
        PeriodicDiagonalPreconditioner::try_build(operator).map(Self::Diagonal)
    }
}

impl super::krylov::KrylovPreconditioner for PeriodicPreconditioner {
    fn apply(&self, residual: &[Complex64]) -> Vec<Complex64> {
        match self {
            Self::Block(preconditioner) => preconditioner.apply(residual),
            Self::Diagonal(preconditioner) => preconditioner.apply(residual),
        }
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
        let preconditioner = if try_krylov {
            Some(PeriodicPreconditioner::build(&operator, false)?)
        } else {
            None
        };

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
                let restart = super::krylov::bounded_gmres_restart(self.config.gmres_restart, size);
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
                    binary_scale_exponent: 0,
                    flicker: None,
                }
            })
            .collect()
    }

    /// Accept a matrix-free PNoise adjoint only after an independent
    /// componentwise backward-error check.
    ///
    /// GMRES reports a normwise residual.  That criterion can hide a failed
    /// low-scale equation beside a well-scaled row, so convergence alone is
    /// not publication evidence.  Small systems may recover through the
    /// independently certified dense solver; an automatic dense fallback is
    /// forbidden at and above the Krylov threshold because its quadratic
    /// allocation is precisely what the matrix-free route is meant to avoid.
    fn qualify_periodic_noise_adjoint(
        &self,
        operator: &PeriodicConversionOperator<'_>,
        rhs: &[Complex64],
        outcome: super::krylov::GmresOutcome,
    ) -> Result<Vec<Complex64>, HbError> {
        let size = rhs.len();
        let qualification = if outcome.converged {
            rspice_matrix::certify_complex_transpose_solution_by_entry_visitor(
                size,
                size,
                &outcome.solution,
                rhs,
                |visitor| {
                    operator.visit_entries(|row, column, value| {
                        visitor(row, column, value);
                    });
                },
            )
        } else {
            Err(rspice_matrix::SolverError::ConvergenceFailed(
                outcome.iterations,
            ))
        };

        match qualification {
            Ok(()) => {
                if self.config.verbose {
                    log::debug!(
                        "PNoise matrix-free adjoint: {} iterations, componentwise certified \
                         (reported normwise relative residual {:.2e})",
                        outcome.iterations,
                        outcome.relative_residual
                    );
                }
                Ok(outcome.solution)
            }
            Err(
                error @ rspice_matrix::SolverError::ConvergenceFailed(_)
                | error @ rspice_matrix::SolverError::InaccurateSolution(_),
            ) if size < super::krylov::KRYLOV_AUTO_THRESHOLD => {
                log::debug!(
                    "PNoise matrix-free adjoint was not certified after {} iterations \
                     (reported relative residual {:.2e}: {}); using bounded dense recovery",
                    outcome.iterations,
                    outcome.relative_residual,
                    error
                );
                let transpose = operator.to_dense_transpose();
                self.solve_complex_linear_system(&transpose, rhs)
            }
            Err(error) => Err(HbError::InvalidCircuit(format!(
                "PNoise adjoint {size}x{size} iterative linear solve is uncertified after {} \
                 iterations (reported normwise relative residual {:.3e}): {error}",
                outcome.iterations, outcome.relative_residual
            ))),
        }
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
        if !offset_hz.is_finite() || offset_hz < 0.0 {
            return Err(HbError::InvalidCircuit(format!(
                "pnoise offset frequency must be finite and non-negative, got {offset_hz}"
            )));
        }
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
        for source in sources {
            if source.psd.is_empty() {
                return Err(HbError::InvalidCircuit(format!(
                    "pnoise source '{}' has no periodic PSD coefficients",
                    source.name
                )));
            }
            if source
                .psd
                .iter()
                .any(|coefficient| !coefficient.re.is_finite() || !coefficient.im.is_finite())
            {
                return Err(HbError::InvalidCircuit(format!(
                    "pnoise source '{}' contains a non-finite periodic PSD coefficient",
                    source.name
                )));
            }
            let coefficient_scale = source
                .psd
                .iter()
                .map(|coefficient| coefficient.norm())
                .fold(0.0, Value::max);
            let dc_tolerance =
                coefficient_scale * Value::EPSILON * 32.0 * source.psd.len() as Value;
            let dc = source.psd[0];
            if !dc_tolerance.is_finite() || dc.re < -dc_tolerance || dc.im.abs() > dc_tolerance {
                return Err(HbError::InvalidCircuit(format!(
                    "pnoise source '{}' has an invalid DC PSD coefficient ({:+.6e}{:+.6e}j)",
                    source.name, dc.re, dc.im
                )));
            }
            if let Some((coefficient, exponent)) = source.flicker
                && (!coefficient.is_finite() || coefficient < 0.0 || !exponent.is_finite())
            {
                return Err(HbError::InvalidCircuit(format!(
                    "pnoise source '{}' has invalid flicker parameters ({coefficient}, {exponent})",
                    source.name
                )));
            }
        }
        let s = (sideband_max - sideband_min + 1) as usize;
        let size = n * s;

        let try_krylov = self.config.use_krylov || size >= super::krylov::KRYLOV_AUTO_THRESHOLD;
        let span = (sideband_max - sideband_min).unsigned_abs() as usize;
        let (spectra, cap_spectra) = if self.has_nonlinear_devices() {
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
            let preconditioner = PeriodicPreconditioner::build(&operator, true)?;
            let restart = super::krylov::bounded_gmres_restart(self.config.gmres_restart, size);
            let outcome = super::krylov::gmres(
                &|input| operator.apply_transpose(input),
                &preconditioner,
                &e,
                restart,
                6,
            );
            self.qualify_periodic_noise_adjoint(&operator, &e, outcome)?
        } else {
            let transpose = operator.to_dense_transpose();
            self.solve_complex_linear_system(&transpose, &e)?
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
                if !a.re.is_finite() || !a.im.is_finite() {
                    return Err(HbError::InvalidCircuit(format!(
                        "pnoise source '{}' has a non-finite adjoint gain at sideband {}",
                        source.name,
                        sideband_min + k_idx as i32
                    )));
                }
                *gain = a;
            }

            let mut contribution = Complex64::new(0.0, 0.0);
            let mut absolute_sum = 0.0;
            let mut term_count = 0usize;
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
                    let term = scaled_complex_product3(
                        gains[k_idx],
                        gains[m_idx].conj(),
                        s_d,
                        source.binary_scale_exponent,
                    )
                    .map_err(|reason| {
                        HbError::InvalidCircuit(format!(
                            "pnoise source '{}' white-noise term is invalid: {reason}",
                            source.name
                        ))
                    })?;
                    contribution += term;
                    if !contribution.re.is_finite() || !contribution.im.is_finite() {
                        return Err(HbError::InvalidCircuit(format!(
                            "pnoise source '{}' white-noise accumulation became non-finite",
                            source.name
                        )));
                    }
                    absolute_sum += term.norm();
                    if !absolute_sum.is_finite() {
                        return Err(HbError::InvalidCircuit(format!(
                            "pnoise source '{}' white-noise error bound became non-finite",
                            source.name
                        )));
                    }
                    term_count = term_count.saturating_add(1);
                }
            }

            // Stationary flicker folding: the colored density is sampled at
            // each sideband's absolute frequency and folds through |A_k|^2
            // (no sideband correlation for a stationary source).
            if let Some((coeff, ef)) = source.flicker {
                let omega0_hz = self.config.fundamental_freq;
                for (k_idx, gain) in gains.iter().enumerate() {
                    if coeff == 0.0 {
                        continue;
                    }
                    let k = sideband_min + k_idx as i32;
                    let sideband_frequency = offset_hz + (k as f64) * omega0_hz;
                    if !sideband_frequency.is_finite() {
                        return Err(HbError::InvalidCircuit(format!(
                            "pnoise source '{}' has a non-finite sideband frequency at k={k}",
                            source.name
                        )));
                    }
                    let f_abs = sideband_frequency.abs();
                    if f_abs == 0.0 && ef > 0.0 {
                        return Err(HbError::InvalidCircuit(format!(
                            "pnoise source '{}' has singular 1/f noise at the zero-frequency sideband k={k}",
                            source.name
                        )));
                    }
                    let term = scaled_flicker_density(
                        *gain,
                        coeff,
                        source.binary_scale_exponent,
                        f_abs,
                        ef,
                    )
                    .map_err(|reason| {
                        HbError::InvalidCircuit(format!(
                            "pnoise source '{}' produced an invalid flicker-noise density at sideband {k}: {reason}",
                            source.name
                        ))
                    })?;
                    contribution.re += term;
                    if !contribution.re.is_finite() {
                        return Err(HbError::InvalidCircuit(format!(
                            "pnoise source '{}' flicker-noise accumulation became non-finite",
                            source.name
                        )));
                    }
                    absolute_sum += term;
                    if !absolute_sum.is_finite() {
                        return Err(HbError::InvalidCircuit(format!(
                            "pnoise source '{}' flicker-noise error bound became non-finite",
                            source.name
                        )));
                    }
                    term_count = term_count.saturating_add(1);
                }
            }
            // The double sum is Hermitian by construction; numerical
            // round-off leaves a vanishing imaginary part and can place an
            // exact zero a few ulps below zero. Do not let max(0) silently
            // turn NaN or a materially non-physical PSD into a successful
            // result.
            let roundoff_tolerance =
                absolute_sum * Value::EPSILON * 32.0 * (term_count.max(1) as Value);
            if !roundoff_tolerance.is_finite() {
                return Err(HbError::InvalidCircuit(format!(
                    "pnoise source '{}' roundoff bound is non-finite",
                    source.name
                )));
            }
            if contribution.im.abs() > roundoff_tolerance {
                return Err(HbError::InvalidCircuit(format!(
                    "pnoise source '{}' produced a non-Hermitian density ({:+.6e}{:+.6e}j)",
                    source.name, contribution.re, contribution.im
                )));
            }
            if contribution.re < -roundoff_tolerance {
                return Err(HbError::InvalidCircuit(format!(
                    "pnoise source '{}' produced a negative output-noise density {:.6e}",
                    source.name, contribution.re
                )));
            }
            contributions.push(if contribution.re > 0.0 {
                contribution.re
            } else {
                0.0
            });
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
    /// Shared base-2 exponent applied to the PSD coefficients and flicker
    /// coefficient. This preserves physically representable folded results
    /// when an elementary source density lies outside the direct `f64` range.
    pub binary_scale_exponent: i32,
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

    struct IdentityPreconditioner;

    impl super::super::krylov::KrylovPreconditioner for IdentityPreconditioner {
        fn apply(&self, residual: &[Complex64]) -> Vec<Complex64> {
            residual.to_vec()
        }
    }

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
    fn periodic_noise_rejects_invalid_source_densities_instead_of_zero_filling() {
        let config = HbConfig::new(1.0e6).with_harmonics(1);
        let mut solver = HbSolver::new(config, 1);
        solver.add_conductance(0, 0, 1.0);
        let state = HbSolverState::new(1, 1);
        let invalid_source = |density| PeriodicNoiseSource {
            name: "invalid source".to_string(),
            node_pos: 0,
            node_neg: usize::MAX,
            psd: vec![Complex64::new(density, 0.0)],
            binary_scale_exponent: 0,
            flicker: None,
        };

        let non_finite = solver
            .solve_periodic_noise(&state, 1.0e3, 0, 0, 0, None, &[invalid_source(Value::NAN)])
            .expect_err("NaN source density must fail closed");
        assert!(non_finite.to_string().contains("non-finite"));

        let negative = solver
            .solve_periodic_noise(&state, 1.0e3, 0, 0, 0, None, &[invalid_source(-1.0)])
            .expect_err("negative source density must not be clamped to zero");
        assert!(negative.to_string().contains("invalid DC"));

        for (source, expected) in [
            (
                PeriodicNoiseSource {
                    name: "infinite source".to_string(),
                    node_pos: 0,
                    node_neg: usize::MAX,
                    psd: vec![Complex64::new(Value::INFINITY, 0.0)],
                    binary_scale_exponent: 0,
                    flicker: None,
                },
                "non-finite",
            ),
            (
                PeriodicNoiseSource {
                    name: "complex DC source".to_string(),
                    node_pos: 0,
                    node_neg: usize::MAX,
                    psd: vec![Complex64::new(1.0, 1.0)],
                    binary_scale_exponent: 0,
                    flicker: None,
                },
                "invalid DC",
            ),
            (
                PeriodicNoiseSource {
                    name: "empty source".to_string(),
                    node_pos: 0,
                    node_neg: usize::MAX,
                    psd: Vec::new(),
                    binary_scale_exponent: 0,
                    flicker: None,
                },
                "no periodic PSD",
            ),
            (
                PeriodicNoiseSource {
                    name: "invalid flicker source".to_string(),
                    node_pos: 0,
                    node_neg: usize::MAX,
                    psd: vec![Complex64::new(0.0, 0.0)],
                    binary_scale_exponent: 0,
                    flicker: Some((Value::NAN, 1.0)),
                },
                "invalid flicker",
            ),
        ] {
            let error = solver
                .solve_periodic_noise(&state, 1.0e3, 0, 0, 0, None, &[source])
                .expect_err("invalid source evidence must fail closed");
            assert!(
                error.to_string().contains(expected),
                "unexpected error: {error}"
            );
        }

        for density in [0.0, 1.0] {
            let valid = solver
                .solve_periodic_noise(&state, 1.0e3, 0, 0, 0, None, &[invalid_source(density)])
                .expect("finite non-negative DC density remains valid");
            assert_eq!(valid, vec![density]);
        }

        let flicker = PeriodicNoiseSource {
            name: "flicker source".to_string(),
            node_pos: 0,
            node_neg: usize::MAX,
            psd: vec![Complex64::new(0.0, 0.0)],
            binary_scale_exponent: 0,
            flicker: Some((1.0, 1.0)),
        };
        let singular = solver
            .solve_periodic_noise(&state, 0.0, 0, 0, 0, None, &[flicker.clone()])
            .expect_err("1/f noise at an exact zero-frequency sideband is singular");
        assert!(singular.to_string().contains("singular 1/f"));

        let below_legacy_floor = solver
            .solve_periodic_noise(&state, 1.0e-6, 0, 0, 0, None, &[flicker])
            .expect("nonzero sub-millihertz flicker density remains physical");
        assert!((below_legacy_floor[0] - 1.0e6).abs() <= 4.0 * Value::EPSILON * 1.0e6);
    }

    #[test]
    fn periodic_noise_term_materialization_preserves_representable_extremes() {
        let large_white = scaled_complex_product3(
            Complex64::new(1.0e200, 0.0),
            Complex64::new(1.0e200, 0.0),
            Complex64::new(1.0e-200, 0.0),
            0,
        )
        .expect("scaled white-noise product remains representable");
        assert!((large_white.re - 1.0e200).abs() <= 8.0 * Value::EPSILON * 1.0e200);
        assert_eq!(large_white.im, 0.0);

        let small_white = scaled_complex_product3(
            Complex64::new(1.0e-200, 0.0),
            Complex64::new(1.0e-200, 0.0),
            Complex64::new(1.0e200, 0.0),
            0,
        )
        .expect("scaled white-noise product must not underflow prematurely");
        assert!((small_white.re - 1.0e-200).abs() <= 8.0 * Value::EPSILON * 1.0e-200);

        let externally_scaled = scaled_complex_product3(
            Complex64::new(libm::scalbn(1.0, 500), 0.0),
            Complex64::new(libm::scalbn(1.0, 500), 0.0),
            Complex64::new(1.0, 0.0),
            -1100,
        )
        .expect("an out-of-range elementary PSD scale can fold to a finite result");
        assert_eq!(
            externally_scaled,
            Complex64::new(libm::scalbn(1.0, -100), 0.0)
        );

        let folded =
            scaled_flicker_density(Complex64::new(1.0e-100, 0.0), 1.0e-200, 0, 1.0e-200, 2.0)
                .expect("scaled flicker product and ratio remain representable");
        assert!((folded - 1.0).abs() <= 2.0e-12, "got {folded:.16e}");

        let maximum = Value::MAX;
        let maximum_mantissa = libm::scalbn(maximum, -1023);
        let extreme_gain = scaled_flicker_density(
            Complex64::new(maximum, maximum),
            libm::scalbn(1.0, -1022),
            0,
            libm::scalbn(1.0, 500),
            2.0,
        )
        .expect("finite complex components need not have a materialized finite norm");
        let expected = maximum_mantissa * maximum_mantissa * libm::scalbn(1.0, 25);
        assert!((extreme_gain - expected).abs() <= 2.0e-12 * expected);
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
        let dense_transpose = operator.to_dense_transpose();
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
            for column in 0..10 {
                assert_eq!(dense_transpose[row][column], dense[column][row]);
            }
        }
    }

    #[test]
    fn pnoise_adjoint_rejects_normwise_false_convergence_and_recovers_when_small() {
        // For nonsymmetric A = [[1, eps], [0, 1]], b = [1, 0], x = [1, 0]
        // is the exact NORMAL solution but a false TRANSPOSE solution.  Its
        // transpose residual has norm eps < GMRES_REL_TOL, while its second
        // equation has componentwise backward error eps/(eps*1) = 1.
        let eps = 1.0e-12;
        assert!(eps < super::super::krylov::GMRES_REL_TOL);
        let g = vec![(0, 0, 1.0), (0, 1, eps), (1, 1, 1.0)];
        let operator = PeriodicConversionOperator {
            num_nodes: 2,
            num_sidebands: 1,
            sideband_min: 0,
            offset_hz: 1.0,
            omega0: 2.0 * PI,
            omega_floor: 2.0 * PI * 1.0e-12,
            g_matrix: &g,
            c_matrix: &[],
            l_matrix: &[],
            g_spectra: &[],
            c_spectra: &[],
        };
        let solver = HbSolver::new(HbConfig::new(1.0).with_harmonics(1), 2);
        let rhs = vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)];
        let false_convergence = super::super::krylov::gmres(
            &|input| operator.apply_transpose(input),
            &IdentityPreconditioner,
            &rhs,
            2,
            1,
        );
        assert!(false_convergence.converged);
        assert!(false_convergence.relative_residual < super::super::krylov::GMRES_REL_TOL);
        assert_eq!(
            false_convergence.solution[1],
            Complex64::new(0.0, 0.0),
            "the normwise GMRES criterion did not resolve the low-scale equation"
        );

        let recovered = solver
            .qualify_periodic_noise_adjoint(&operator, &rhs, false_convergence)
            .expect("a small uncertified Krylov solution uses bounded dense recovery");
        assert_close(recovered[0], Complex64::new(1.0, 0.0));
        assert!(
            recovered[1].re < 0.0,
            "the transpose correction has the wrong sign"
        );
        assert_eq!(recovered[1].im, 0.0);
        let expected_correction = -eps;
        assert!(
            (recovered[1].re + eps).abs() <= 8.0 * Value::EPSILON * eps,
            "transpose correction {} differs from {} at its own scale",
            recovered[1].re,
            expected_correction
        );
    }

    #[test]
    fn pnoise_adjoint_never_materializes_dense_fallback_at_krylov_threshold() {
        let dimension = super::super::krylov::KRYLOV_AUTO_THRESHOLD;
        let eps = 1.0e-12;
        let mut g = (0..dimension)
            .map(|index| (index, index, 1.0))
            .collect::<Vec<_>>();
        g.push((0, 1, eps));
        let operator = PeriodicConversionOperator {
            num_nodes: dimension,
            num_sidebands: 1,
            sideband_min: 0,
            offset_hz: 1.0,
            omega0: 2.0 * PI,
            omega_floor: 2.0 * PI * 1.0e-12,
            g_matrix: &g,
            c_matrix: &[],
            l_matrix: &[],
            g_spectra: &[],
            c_spectra: &[],
        };
        let solver = HbSolver::new(HbConfig::new(1.0).with_harmonics(1), dimension);
        let mut rhs = vec![Complex64::new(0.0, 0.0); dimension];
        rhs[0] = Complex64::new(1.0, 0.0);
        let mut false_solution = vec![Complex64::new(0.0, 0.0); dimension];
        false_solution[0] = Complex64::new(1.0, 0.0);
        let false_convergence = super::super::krylov::GmresOutcome {
            solution: false_solution,
            iterations: 1,
            relative_residual: eps,
            converged: true,
        };

        let error = solver
            .qualify_periodic_noise_adjoint(&operator, &rhs, false_convergence)
            .expect_err("large uncertified Krylov solutions must fail without dense allocation");
        let HbError::InvalidCircuit(message) = error else {
            panic!("unexpected error: {error}");
        };
        assert!(message.contains("PNoise adjoint 256x256"), "{message}");
        assert!(message.contains("after 1 iterations"), "{message}");
        assert!(message.contains("backward-error"), "{message}");
    }

    #[test]
    fn periodic_preconditioner_uses_linear_storage_at_dense_entry_boundary() {
        let dimension = super::super::krylov::KRYLOV_AUTO_THRESHOLD;
        let g = (0..dimension)
            .map(|index| (index, index, 2.0))
            .collect::<Vec<_>>();
        let operator = PeriodicConversionOperator {
            num_nodes: dimension,
            num_sidebands: 1,
            sideband_min: 0,
            offset_hz: 1.0,
            omega0: 2.0 * PI,
            omega_floor: 2.0 * PI * 1.0e-12,
            g_matrix: &g,
            c_matrix: &[],
            l_matrix: &[],
            g_spectra: &[],
            c_spectra: &[],
        };

        let preconditioner = PeriodicPreconditioner::build(&operator, true)
            .expect("threshold-size production preconditioner builds");
        assert!(
            matches!(&preconditioner, PeriodicPreconditioner::Diagonal(_)),
            "dense block entries equal to the strict limit must use O(system-size) storage"
        );
        let residual = vec![Complex64::new(4.0, -2.0); dimension];
        let scaled = super::super::krylov::KrylovPreconditioner::apply(&preconditioner, &residual);
        assert_eq!(scaled, vec![Complex64::new(2.0, -1.0); dimension]);
    }

    #[test]
    fn periodic_diagonal_preconditioner_rejects_nonfinite_accumulation() {
        let dimension = super::super::krylov::KRYLOV_AUTO_THRESHOLD;
        let mut g = (0..dimension)
            .map(|index| (index, index, 2.0))
            .collect::<Vec<_>>();
        g.push((0, 0, Value::MAX));
        g.push((0, 0, Value::MAX));
        let operator = PeriodicConversionOperator {
            num_nodes: dimension,
            num_sidebands: 1,
            sideband_min: 0,
            offset_hz: 1.0,
            omega0: 2.0 * PI,
            omega_floor: 2.0 * PI * 1.0e-12,
            g_matrix: &g,
            c_matrix: &[],
            l_matrix: &[],
            g_spectra: &[],
            c_spectra: &[],
        };

        let error = match PeriodicPreconditioner::build(&operator, true) {
            Err(error) => error,
            Ok(_) => panic!("overflowed diagonal accumulation must fail before iteration"),
        };
        let HbError::InvalidCircuit(message) = error else {
            panic!("unexpected error: {error}");
        };
        assert!(message.contains("entry 0"), "{message}");
        assert!(
            message.contains("non-finite after stamp accumulation"),
            "{message}"
        );
    }

    #[test]
    fn pnoise_adjoint_never_dense_fallbacks_after_structural_certificate_failure() {
        let invalid_spectra = vec![(1, 0, vec![Complex64::new(1.0, 0.0)])];
        let operator = PeriodicConversionOperator {
            num_nodes: 1,
            num_sidebands: 1,
            sideband_min: 0,
            offset_hz: 1.0,
            omega0: 2.0 * PI,
            omega_floor: 2.0 * PI * 1.0e-12,
            g_matrix: &[],
            c_matrix: &[],
            l_matrix: &[],
            g_spectra: &invalid_spectra,
            c_spectra: &[],
        };
        let solver = HbSolver::new(HbConfig::new(1.0).with_harmonics(1), 1);
        let outcome = super::super::krylov::GmresOutcome {
            solution: vec![Complex64::new(1.0, 0.0)],
            iterations: 1,
            relative_residual: 0.0,
            converged: true,
        };

        let error = solver
            .qualify_periodic_noise_adjoint(&operator, &[Complex64::new(1.0, 0.0)], outcome)
            .expect_err("structural certificate failures must bypass dense recovery");
        let HbError::InvalidCircuit(message) = error else {
            panic!("unexpected error: {error}");
        };
        assert!(
            message.contains("outside native 1x1 matrix"),
            "message={message}"
        );
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
        let preconditioner = PeriodicPreconditioner::build(&operator, false)
            .expect("small matrix-free conversion preconditioner builds");
        assert!(matches!(&preconditioner, PeriodicPreconditioner::Block(_)));
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
