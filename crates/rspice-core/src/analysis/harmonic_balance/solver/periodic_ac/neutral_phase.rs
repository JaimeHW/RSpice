//! Preserve the neutral time-shift mode of a retained autonomous orbit.
//!
//! Differentiating the autonomous circuit equation gives A(0) t = 0, where
//! t is the orbit's phase derivative (Demir et al., TCAS-I 2000, Lemma 5.1).
//! Truncation and orbit interpolation leave a small defect r = A(0) t.
//! After checking that defect against the requested relative tolerance, use
//! A(f) - r p^T, p^T t = 1. Write x = x_perp + t*beta/(j*offset),
//! p^T*x_perp = 0. The sparse border [A C*t; p^T 0] solves for x_perp
//! and phase velocity beta without subtracting nearly equal phase currents.
//! The observation carries the explicit phase integrator. The retained orbit
//! and the requested offset are unchanged.

use super::*;
use crate::numerics::krylov::KrylovPreconditioner;

pub(super) trait NoiseAdjointOperator {
    fn dimension(&self) -> usize;
    fn visit_noise_entries(&self, visitor: &mut dyn FnMut(usize, usize, Complex64));

    fn prepare_noise_rhs(
        &self,
        _rhs: &mut [Complex64],
        _phase_response: Option<Complex64>,
    ) -> Result<(), HbError> {
        Ok(())
    }

    fn apply_noise_transpose(&self, input: &[Complex64]) -> Vec<Complex64> {
        let mut output = vec![Complex64::ZERO; self.dimension()];
        self.visit_noise_entries(&mut |row, column, value| {
            output[column] += value * input[row];
        });
        output
    }

    fn noise_dense_transpose(&self) -> Vec<Vec<Complex64>> {
        let size = self.dimension();
        let mut matrix = vec![vec![Complex64::ZERO; size]; size];
        self.visit_noise_entries(&mut |row, column, value| matrix[column][row] += value);
        matrix
    }
}

impl NoiseAdjointOperator for PeriodicConversionOperator<'_> {
    fn dimension(&self) -> usize {
        self.num_unknowns().unwrap_or(0) * self.num_sidebands
    }

    fn visit_noise_entries(&self, visitor: &mut dyn FnMut(usize, usize, Complex64)) {
        self.visit_entries(visitor);
    }

    fn apply_noise_transpose(&self, input: &[Complex64]) -> Vec<Complex64> {
        self.apply_transpose(input)
    }

    fn noise_dense_transpose(&self) -> Vec<Vec<Complex64>> {
        self.to_dense_transpose()
    }
}

pub(super) struct NeutralPhaseOperator<'a, 'stamps> {
    base: &'a PeriodicConversionOperator<'stamps>,
    phase_rate: Vec<Complex64>,
    dual: Vec<Complex64>,
    tangent_scale: Value,
}

impl<'a, 'stamps> NeutralPhaseOperator<'a, 'stamps> {
    pub(super) fn new(
        base: &'a PeriodicConversionOperator<'stamps>,
        state: &HbSolverState,
        tolerance: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Self, HbError> {
        if !tolerance.is_finite() || tolerance <= 0.0 || tolerance >= 1.0 {
            return Err(HbError::InvalidCircuit(
                "autonomous noise requires a relative phase-tangent tolerance between zero and one"
                    .into(),
            ));
        }
        if base.offset_hz <= 0.0 {
            return Err(HbError::InvalidCircuit(
                "autonomous sampled noise requires a positive offset; absolute phase has no finite DC density".into(),
            ));
        }
        if state.x.len() != base.num_nodes
            || state.mna_branch_currents.len() != base.mna_branches.len()
        {
            return Err(HbError::InvalidCircuit(
                "autonomous noise tangent requires the complete retained node and branch basis"
                    .into(),
            ));
        }
        if !base.periodic_networks.is_empty() {
            return Err(HbError::InvalidCircuit(
                "autonomous sampled PNOISE is unavailable for distributed-network devices".into(),
            ));
        }
        let size = base.dimension();
        size.checked_add(1).ok_or_else(|| {
            HbError::InvalidCircuit(
                "autonomous noise border dimension overflows this platform".into(),
            )
        })?;
        let mut tangent = try_zeroed_complex_values(size, "autonomous phase tangent")?;
        for (unknown, spectrum) in state.x.iter().chain(&state.mna_branch_currents).enumerate() {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            for band in 0..base.num_sidebands {
                let harmonic = i64::from(base.sideband_min) + band as i64;
                let coefficient = spectrum
                    .get(harmonic.unsigned_abs() as usize)
                    .copied()
                    .ok_or_else(|| {
                        HbError::InvalidCircuit(
                            "autonomous noise phase tangent exceeds the retained harmonic basis"
                                .into(),
                        )
                    })?;
                let coefficient = if harmonic < 0 {
                    coefficient.conj()
                } else {
                    coefficient
                };
                let index = unknown
                    .checked_mul(base.num_sidebands)
                    .and_then(|index| index.checked_add(band))
                    .filter(|&index| index < size)
                    .ok_or_else(|| {
                        HbError::InvalidCircuit(
                            "autonomous noise phase tangent does not match the MNA basis".into(),
                        )
                    })?;
                tangent[index] = Complex64::new(0.0, harmonic as Value) * coefficient;
            }
        }
        let scale = tangent
            .iter()
            .map(|value| value.norm())
            .fold(0.0, Value::max);
        if !scale.is_finite() || scale == 0.0 {
            return Err(HbError::InvalidCircuit(
                "autonomous sampled noise needs nonzero oscillation harmonics inside its sideband window".into(),
            ));
        }
        for value in &mut tangent {
            *value /= scale;
        }
        // Bound the derivative defect against each physical unknown's AC
        // scale, not the amplitude of an individual Fourier bin. A bin can
        // vanish by symmetry while neighboring bins still carry the orbit.
        let mut unknown_scale = try_zeroed_complex_values(
            state.x.len() + state.mna_branch_currents.len(),
            "autonomous phase coordinate scales",
        )?;
        for (scale, harmonics) in unknown_scale
            .iter_mut()
            .zip(tangent.chunks_exact(base.num_sidebands))
        {
            scale.re = harmonics
                .iter()
                .map(|value| value.norm())
                .fold(0.0, Value::max);
        }
        let norm_squared: Value = tangent.iter().map(|value| value.norm_sqr()).sum();
        let mut dual = try_zeroed_complex_values(size, "autonomous phase dual")?;
        for (value, tangent) in dual.iter_mut().zip(&tangent) {
            *value = tangent.conj() / norm_squared;
        }
        let zero_offset = PeriodicConversionOperator {
            offset_hz: 0.0,
            ..*base
        };
        zero_offset.validate("autonomous phase tangent")?;
        let mut defect = try_zeroed_complex_values(size, "autonomous phase defect")?;
        let mut row_scale = try_zeroed_complex_values(size, "autonomous phase row scales")?;
        let mut entries = 0_usize;
        zero_offset.visit_entries(|row, column, value| {
            defect[row] += value * tangent[column];
            row_scale[row].re += value.norm() * unknown_scale[column / base.num_sidebands].re;
            entries = entries.saturating_add(1);
        });
        if abort.is_aborted() {
            return Err(HbError::Aborted);
        }
        let limit = tolerance + 32.0 * Value::EPSILON * entries.max(1) as Value;
        for (row, (defect, scale)) in defect.iter().zip(&row_scale).enumerate() {
            if !complex_is_finite(*defect)
                || !scale.re.is_finite()
                || defect.norm() > limit * scale.re
            {
                return Err(HbError::InvalidCircuit(format!(
                    "autonomous noise phase tangent is unresolved in row {row}: relative defect {:.6e} exceeds {:.6e}; refine the retained PSS orbit or increase its harmonics and the noise sideband window",
                    if scale.re > 0.0 {
                        defect.norm() / scale.re
                    } else {
                        Value::INFINITY
                    },
                    limit,
                )));
            }
        }
        let phase_rate = phase_rate_column(base, &tangent)?;
        Ok(Self {
            base,
            phase_rate,
            dual,
            tangent_scale: scale,
        })
    }
}

impl NoiseAdjointOperator for NeutralPhaseOperator<'_, '_> {
    fn dimension(&self) -> usize {
        self.phase_rate.len() + 1
    }

    fn visit_noise_entries(&self, visitor: &mut dyn FnMut(usize, usize, Complex64)) {
        self.base.visit_entries(&mut *visitor);
        let border = self.phase_rate.len();
        for (index, (&rate, &dual)) in self.phase_rate.iter().zip(&self.dual).enumerate() {
            visitor(index, border, rate);
            visitor(border, index, dual);
        }
    }

    fn prepare_noise_rhs(
        &self,
        rhs: &mut [Complex64],
        phase_response: Option<Complex64>,
    ) -> Result<(), HbError> {
        let phase_response = phase_response.ok_or_else(|| {
            HbError::InvalidCircuit(
                "autonomous noise requires the observation's phase response".into(),
            )
        })?;
        // The observation is per radian of the full retained orbit, whereas
        // the bordered tangent is normalized for numerical conditioning.
        let value = phase_response
            / self.tangent_scale
            / Complex64::new(0.0, 2.0 * PI * self.base.offset_hz);
        if !complex_is_finite(value) {
            return Err(HbError::InvalidCircuit(
                "autonomous noise phase response is not representable".into(),
            ));
        }
        rhs[self.phase_rate.len()] = value;
        Ok(())
    }
}

/// Exact divided difference (A(f)-A(0))*t/(j*2*pi*f). Constant G stamps
/// cancel symbolically. C and exact branch inductances are affine in frequency;
/// the legacy nodal inductance uses its algebraic divided difference.
fn phase_rate_column(
    base: &PeriodicConversionOperator<'_>,
    tangent: &[Complex64],
) -> Result<Vec<Complex64>, HbError> {
    let mut rate = try_zeroed_complex_values(tangent.len(), "autonomous phase-rate column")?;
    let s = base.num_sidebands;
    for band in 0..s {
        for &(row, column, capacitance) in base.c_matrix {
            if row < base.num_nodes && column < base.num_nodes {
                rate[row * s + band] += capacitance * tangent[column * s + band];
            }
        }
        let harmonic = i64::from(base.sideband_min) + band as i64;
        if harmonic != 0 {
            let omega0 = 2.0 * PI * harmonic as Value * base.fundamental_hz;
            let omega1 = base.omega(band);
            for &(row, column, inductance) in base.l_matrix {
                if row < base.num_nodes && column < base.num_nodes {
                    rate[row * s + band] +=
                        tangent[column * s + band] / inductance / omega0 / omega1;
                }
            }
        }
        for (index, branch) in base.mna_branches.iter().enumerate() {
            if let ExactMnaBranch::Inductor { inductance, .. } = branch {
                let coordinate = (base.num_nodes + index) * s + band;
                rate[coordinate] -= *inductance * tangent[coordinate];
            }
        }
        for &(row, column, inductance) in base.mna_inductance_entries {
            rate[row * s + band] -= inductance * tangent[column * s + band];
        }
    }
    for &(row, column, ref spectrum) in base.c_spectra {
        for output_band in 0..s {
            for input_band in 0..s {
                let difference = output_band as i64 - input_band as i64;
                if let Some(&coefficient) = spectrum.get(difference.unsigned_abs() as usize) {
                    let coefficient = if difference < 0 {
                        coefficient.conj()
                    } else {
                        coefficient
                    };
                    rate[row * s + output_band] += coefficient * tangent[column * s + input_band];
                }
            }
        }
    }
    if rate.iter().any(|&value| !complex_is_finite(value))
        || rate.iter().all(|&value| value == Complex64::ZERO)
    {
        return Err(HbError::InvalidCircuit(
            "autonomous noise phase-rate column is zero or non-finite".into(),
        ));
    }
    Ok(rate)
}

pub(super) struct NoisePreconditioner {
    pub base: PeriodicPreconditioner,
    pub base_size: usize,
    pub bordered: bool,
}

impl KrylovPreconditioner for NoisePreconditioner {
    fn apply(&self, residual: &[Complex64]) -> Vec<Complex64> {
        let mut result = self.base.apply(&residual[..self.base_size]);
        if self.bordered {
            result.push(residual[self.base_size]);
        }
        result
    }
}

/// Refine a Krylov candidate when the stricter componentwise
/// certificate needs another correction. Storage stays linear and the final
/// publication gate is unchanged; structural failures are never retried.
pub(super) fn refine_noise_adjoint(
    operator: &dyn NoiseAdjointOperator,
    preconditioner: &dyn KrylovPreconditioner,
    rhs: &[Complex64],
    mut outcome: crate::numerics::krylov::GmresOutcome,
    restart: usize,
    abort: &dyn AbortSignal,
) -> Result<crate::numerics::krylov::GmresOutcome, HbError> {
    for _ in 0..2 {
        if abort.is_aborted() {
            return Err(HbError::Aborted);
        }
        let certificate = rspice_matrix::certify_complex_transpose_solution_by_entry_visitor(
            rhs.len(),
            rhs.len(),
            &outcome.solution,
            rhs,
            |visitor| operator.visit_noise_entries(visitor),
        );
        match certificate {
            Ok(()) => {
                outcome.converged = true;
                break;
            }
            Err(rspice_matrix::SolverError::InaccurateSolution(_)) => {}
            Err(_) => break,
        }
        let residual = rhs
            .iter()
            .zip(operator.apply_noise_transpose(&outcome.solution))
            .map(|(&expected, actual)| expected - actual)
            .collect::<Vec<_>>();
        let correction = crate::numerics::krylov::gmres(
            &|input| operator.apply_noise_transpose(input),
            preconditioner,
            &residual,
            restart,
            6,
        );
        if correction.solution.len() != rhs.len()
            || correction
                .solution
                .iter()
                .any(|&value| !complex_is_finite(value))
        {
            break;
        }
        // The independent certificate, not the inner normwise flag, decides
        // whether this bounded residual correction produced a usable solution.
        outcome.converged = true;
        for (value, correction) in outcome.solution.iter_mut().zip(correction.solution) {
            *value += correction;
        }
        outcome.iterations += correction.iterations;
        let residual_norm = rhs
            .iter()
            .zip(operator.apply_noise_transpose(&outcome.solution))
            .map(|(&expected, actual)| (expected - actual).norm())
            .fold(0.0, Value::max);
        let rhs_norm = rhs.iter().map(|value| value.norm()).fold(0.0, Value::max);
        outcome.relative_residual = if rhs_norm > 0.0 {
            residual_norm / rhs_norm
        } else {
            residual_norm
        };
    }
    Ok(outcome)
}
