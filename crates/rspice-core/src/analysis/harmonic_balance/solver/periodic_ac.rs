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

#[inline]
fn complex_is_finite(value: Complex64) -> bool {
    value.re.is_finite() && value.im.is_finite()
}

fn try_zeroed_complex_values(count: usize, context: &str) -> Result<Vec<Complex64>, HbError> {
    let mut values = Vec::new();
    values.try_reserve_exact(count).map_err(|error| {
        HbError::InvalidCircuit(format!(
            "{context} allocation failed for {count} complex values: {error}"
        ))
    })?;
    values.resize(count, Complex64::new(0.0, 0.0));
    Ok(values)
}

#[inline]
fn finite_product_is_representable(left: Value, right: Value) -> bool {
    let product = left * right;
    product.is_finite() && (left == 0.0 || right == 0.0 || product != 0.0)
}

fn periodic_sideband_geometry(
    context: &str,
    num_unknowns: usize,
    sideband_min: i32,
    sideband_max: i32,
) -> Result<(usize, usize, usize), HbError> {
    let span = i64::from(sideband_max) - i64::from(sideband_min);
    if span < 0 {
        return Err(HbError::InvalidCircuit(format!(
            "{context} sideband range is empty"
        )));
    }
    let count = span.checked_add(1).ok_or_else(|| {
        HbError::InvalidCircuit(format!("{context} sideband count overflows i64"))
    })?;
    let sidebands = usize::try_from(count).map_err(|_| {
        HbError::InvalidCircuit(format!(
            "{context} sideband count {count} exceeds this platform"
        ))
    })?;
    let span = usize::try_from(span).map_err(|_| {
        HbError::InvalidCircuit(format!("{context} sideband span exceeds this platform"))
    })?;
    let unknowns = num_unknowns.checked_mul(sidebands).ok_or_else(|| {
        HbError::InvalidCircuit(format!(
            "{context} lifted dimension {num_unknowns} MNA unknowns x {sidebands} sidebands overflows usize"
        ))
    })?;
    Ok((sidebands, unknowns, span))
}

fn validate_periodic_state(
    state: &HbSolverState,
    num_nodes: usize,
    context: &str,
) -> Result<(), HbError> {
    if state.x.len() != num_nodes {
        return Err(HbError::InvalidCircuit(format!(
            "{context} state has {} node spectra for a {num_nodes}-node solver",
            state.x.len()
        )));
    }
    for (node, spectrum) in state.x.iter().enumerate() {
        if spectrum.is_empty() {
            return Err(HbError::InvalidCircuit(format!(
                "{context} state node {node} has no spectral coefficients"
            )));
        }
        if let Some((harmonic, value)) = spectrum
            .iter()
            .copied()
            .enumerate()
            .find(|(_, value)| !complex_is_finite(*value))
        {
            return Err(HbError::InvalidCircuit(format!(
                "{context} state node {node} harmonic {harmonic} is non-finite ({:+.6e}{:+.6e}j)",
                value.re, value.im
            )));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ScaledComplex {
    mantissa: Complex64,
    exponent: i32,
}

impl ScaledComplex {
    const ZERO: Self = Self {
        mantissa: Complex64::new(0.0, 0.0),
        exponent: 0,
    };

    fn is_zero(self) -> bool {
        self.mantissa.re == 0.0 && self.mantissa.im == 0.0
    }
}

fn normalize_scaled_noise_waveform(
    waveform: &[super::devices::ScaledNonnegative],
) -> Result<(Vec<Value>, i32), &'static str> {
    if waveform
        .iter()
        .any(|sample| !sample.mantissa.is_finite() || sample.mantissa < 0.0)
    {
        return Err("a scaled waveform sample has an invalid mantissa");
    }
    let Some(common_exponent) = waveform
        .iter()
        .filter(|sample| sample.mantissa > 0.0)
        .map(|sample| sample.exponent)
        .max()
    else {
        return Ok((vec![0.0; waveform.len()], 0));
    };

    let mut normalized = Vec::with_capacity(waveform.len());
    for sample in waveform {
        if sample.mantissa == 0.0 {
            normalized.push(0.0);
            continue;
        }
        if !(1.0..2.0).contains(&sample.mantissa) {
            return Err("a nonzero scaled waveform mantissa is not normalized");
        }
        let shift = sample
            .exponent
            .checked_sub(common_exponent)
            .ok_or("a scaled waveform exponent range exceeds this platform")?;
        let value = libm::scalbn(sample.mantissa, shift);
        if !value.is_finite() || value <= 0.0 {
            return Err("a nonzero waveform sample is not representable at the common scale");
        }
        if value < Value::MIN_POSITIVE {
            let reverse_shift = shift
                .checked_neg()
                .ok_or("a scaled waveform reverse exponent exceeds this platform")?;
            let recovered = libm::scalbn(value, reverse_shift);
            if recovered.to_bits() != sample.mantissa.to_bits() {
                return Err("a nonzero waveform sample would round at the common scale");
            }
        }
        normalized.push(value);
    }
    Ok((normalized, common_exponent))
}

fn scaled_complex_product3(
    first: Complex64,
    second: Complex64,
    third: Complex64,
    binary_scale_exponent: i32,
) -> Result<ScaledComplex, &'static str> {
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
        return Ok(ScaledComplex::ZERO);
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
    Ok(ScaledComplex {
        mantissa: normalized,
        exponent,
    })
}

fn scale_complex_component_exactly(component: Value, shift: i32) -> Result<Value, &'static str> {
    if component == 0.0 {
        return Ok(0.0);
    }
    let scaled = libm::scalbn(component, shift);
    if !scaled.is_finite() || scaled == 0.0 {
        return Err("a nonzero term component is not representable at the common scale");
    }
    if scaled.abs() < Value::MIN_POSITIVE {
        let reverse_shift = shift
            .checked_neg()
            .ok_or("a scaled-term reverse exponent exceeds this platform")?;
        if libm::scalbn(scaled, reverse_shift).to_bits() != component.to_bits() {
            return Err("a nonzero term component would round at the common scale");
        }
    }
    Ok(scaled)
}

fn compensated_add(
    sum: &mut Value,
    compensation: &mut Value,
    value: Value,
) -> Result<(), &'static str> {
    let next = *sum + value;
    if !next.is_finite() {
        return Err("a common-scale accumulation became non-finite");
    }
    let correction = if sum.abs() >= value.abs() {
        (*sum - next) + value
    } else {
        (value - next) + *sum
    };
    *compensation += correction;
    if !compensation.is_finite() {
        return Err("a common-scale compensation became non-finite");
    }
    *sum = next;
    Ok(())
}

fn validate_scaled_complex(term: ScaledComplex) -> Result<(), &'static str> {
    if !term.mantissa.re.is_finite() || !term.mantissa.im.is_finite() {
        return Err("a scaled term has a non-finite mantissa");
    }
    if !term.is_zero() {
        let scale = term.mantissa.re.abs().max(term.mantissa.im.abs());
        if !(1.0..2.0).contains(&scale) {
            return Err("a nonzero scaled term mantissa is not normalized");
        }
    }
    Ok(())
}

struct ScaledComplexAccumulator {
    common_exponent: i32,
    real_sum: Value,
    real_compensation: Value,
    imag_sum: Value,
    imag_compensation: Value,
    absolute_sum: Value,
    absolute_compensation: Value,
}

impl ScaledComplexAccumulator {
    fn new(common_exponent: i32) -> Self {
        Self {
            common_exponent,
            real_sum: 0.0,
            real_compensation: 0.0,
            imag_sum: 0.0,
            imag_compensation: 0.0,
            absolute_sum: 0.0,
            absolute_compensation: 0.0,
        }
    }

    fn add(&mut self, term: ScaledComplex) -> Result<(), &'static str> {
        validate_scaled_complex(term)?;
        if term.is_zero() {
            return Ok(());
        }
        let shift = term
            .exponent
            .checked_sub(self.common_exponent)
            .ok_or("a scaled-term exponent range exceeds this platform")?;
        let real = scale_complex_component_exactly(term.mantissa.re, shift)?;
        let imag = scale_complex_component_exactly(term.mantissa.im, shift)?;
        compensated_add(&mut self.real_sum, &mut self.real_compensation, real)?;
        compensated_add(&mut self.imag_sum, &mut self.imag_compensation, imag)?;
        let magnitude = Complex64::new(real, imag).norm();
        if !magnitude.is_finite() || magnitude <= 0.0 {
            return Err("a nonzero common-scale term has an invalid magnitude");
        }
        compensated_add(
            &mut self.absolute_sum,
            &mut self.absolute_compensation,
            magnitude,
        )?;
        Ok(())
    }

    fn finish(self) -> Result<(Complex64, Value), &'static str> {
        let normalized = Complex64::new(
            self.real_sum + self.real_compensation,
            self.imag_sum + self.imag_compensation,
        );
        let normalized_absolute_sum = self.absolute_sum + self.absolute_compensation;
        if !normalized.re.is_finite()
            || !normalized.im.is_finite()
            || !normalized_absolute_sum.is_finite()
            || normalized_absolute_sum <= 0.0
        {
            return Err("the completed common-scale white-noise sum is invalid");
        }

        let contribution = Complex64::new(
            libm::scalbn(normalized.re, self.common_exponent),
            libm::scalbn(normalized.im, self.common_exponent),
        );
        let physical_absolute_sum = libm::scalbn(normalized_absolute_sum, self.common_exponent);
        if !contribution.re.is_finite()
            || !contribution.im.is_finite()
            || !physical_absolute_sum.is_finite()
            || physical_absolute_sum <= 0.0
        {
            return Err("the completed white-noise sum is outside the finite binary64 range");
        }
        if (normalized.re != 0.0 && contribution.re == 0.0)
            || (normalized.im != 0.0 && contribution.im == 0.0)
        {
            return Err("a nonzero completed white-noise component is below the binary64 range");
        }
        Ok((contribution, physical_absolute_sum))
    }
}

/// Sum scaled white-noise terms before crossing binary64's physical exponent
/// range. This avoids separately rounding or underflowing terms whose complete
/// Hermitian sum is representable. Terms that cannot be aligned exactly are
/// rejected rather than silently discarded.
#[cfg(test)]
fn materialize_scaled_complex_sum(
    terms: &[ScaledComplex],
) -> Result<(Complex64, Value), &'static str> {
    for &term in terms {
        validate_scaled_complex(term)?;
    }
    let Some(common_exponent) = terms
        .iter()
        .copied()
        .filter(|term| !term.is_zero())
        .map(|term| term.exponent)
        .max()
    else {
        return Ok((Complex64::new(0.0, 0.0), 0.0));
    };
    let mut accumulator = ScaledComplexAccumulator::new(common_exponent);
    for &term in terms {
        accumulator.add(term)?;
    }
    accumulator.finish()
}

fn visit_white_noise_terms(
    gains: &[Complex64],
    psd: &[Complex64],
    binary_scale_exponent: i32,
    mut visit: impl FnMut(ScaledComplex) -> Result<(), &'static str>,
) -> Result<usize, &'static str> {
    let mut term_count = 0usize;
    for (k_idx, &gain_k) in gains.iter().enumerate() {
        for (m_idx, &gain_m) in gains.iter().enumerate() {
            let d = (k_idx as i32) - (m_idx as i32);
            let d_abs = d.unsigned_abs() as usize;
            if d_abs >= psd.len() {
                continue;
            }
            let s_d = if d >= 0 {
                psd[d_abs]
            } else {
                psd[d_abs].conj()
            };
            visit(scaled_complex_product3(
                gain_k,
                gain_m.conj(),
                s_d,
                binary_scale_exponent,
            )?)?;
            term_count = term_count.saturating_add(1);
        }
    }
    Ok(term_count)
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
    fundamental_hz: Value,
    g_matrix: &'a [(usize, usize, Value)],
    c_matrix: &'a [(usize, usize, Value)],
    l_matrix: &'a [(usize, usize, Value)],
    mna_branches: &'a [PeriodicMnaBranch],
    g_spectra: &'a [PeriodicSpectrum],
    c_spectra: &'a [PeriodicSpectrum],
}

impl PeriodicConversionOperator<'_> {
    #[inline]
    fn num_unknowns(&self) -> Option<usize> {
        self.num_nodes.checked_add(self.mna_branches.len())
    }

    #[inline]
    fn omega(&self, sideband_index: usize) -> Value {
        let k = i64::from(self.sideband_min) + sideband_index as i64;
        2.0 * PI * (k as Value).mul_add(self.fundamental_hz, self.offset_hz)
    }

    fn validate(&self, context: &str) -> Result<(), HbError> {
        let declared_span = self.num_sidebands.checked_sub(1).ok_or_else(|| {
            HbError::InvalidCircuit(format!("{context} operator has no sidebands"))
        })?;
        let declared_span = i64::try_from(declared_span).map_err(|_| {
            HbError::InvalidCircuit(format!(
                "{context} sideband count exceeds the i64 representation"
            ))
        })?;
        let sideband_max = i64::from(self.sideband_min)
            .checked_add(declared_span)
            .and_then(|value| i32::try_from(value).ok())
            .ok_or_else(|| {
                HbError::InvalidCircuit(format!(
                    "{context} sideband range exceeds the i32 representation"
                ))
            })?;
        let num_unknowns = self.num_unknowns().ok_or_else(|| {
            HbError::InvalidCircuit(format!(
                "{context} node and branch unknown count overflows usize"
            ))
        })?;
        let (sidebands, unknowns, _) =
            periodic_sideband_geometry(context, num_unknowns, self.sideband_min, sideband_max)?;
        if sidebands != self.num_sidebands {
            return Err(HbError::InvalidCircuit(format!(
                "{context} operator declares {} sidebands but its range contains {sidebands}",
                self.num_sidebands
            )));
        }
        if unknowns == 0 {
            return Err(HbError::InvalidCircuit(format!(
                "{context} operator has no unknowns"
            )));
        }
        if !self.l_matrix.is_empty()
            && self
                .mna_branches
                .iter()
                .any(|branch| matches!(branch, PeriodicMnaBranch::Inductor { .. }))
        {
            return Err(HbError::InvalidCircuit(format!(
                "{context} mixes nodal inductor admittances with exact inductor MNA branches"
            )));
        }
        if !self.offset_hz.is_finite() || self.offset_hz < 0.0 {
            return Err(HbError::InvalidCircuit(format!(
                "{context} offset frequency must be finite and non-negative, got {}",
                self.offset_hz
            )));
        }
        if !self.fundamental_hz.is_finite() || self.fundamental_hz <= 0.0 {
            return Err(HbError::InvalidCircuit(format!(
                "{context} fundamental frequency must be finite and positive, got {}",
                self.fundamental_hz
            )));
        }

        for sideband_index in 0..self.num_sidebands {
            let omega = self.omega(sideband_index);
            if !omega.is_finite() {
                return Err(HbError::InvalidCircuit(format!(
                    "{context} sideband {} has a non-representable angular frequency",
                    i64::from(self.sideband_min) + sideband_index as i64
                )));
            }
        }

        for (entry, &(row, column, value)) in self.g_matrix.iter().enumerate() {
            if !value.is_finite() {
                return Err(HbError::InvalidCircuit(format!(
                    "{context} static conductance entry #{entry} ({row}, {column}) is non-finite"
                )));
            }
        }
        for (entry, &(row, column, value)) in self.c_matrix.iter().enumerate() {
            if !value.is_finite() {
                return Err(HbError::InvalidCircuit(format!(
                    "{context} static capacitance entry #{entry} ({row}, {column}) is non-finite"
                )));
            }
            for sideband_index in 0..self.num_sidebands {
                let omega = self.omega(sideband_index);
                if !finite_product_is_representable(omega, value) {
                    return Err(HbError::InvalidCircuit(format!(
                        "{context} static capacitance entry #{entry} ({row}, {column}) produces a non-representable admittance at sideband {}",
                        i64::from(self.sideband_min) + sideband_index as i64
                    )));
                }
            }
        }
        for (entry, &(row, column, inductance)) in self.l_matrix.iter().enumerate() {
            if !inductance.is_finite() || inductance == 0.0 {
                return Err(HbError::InvalidCircuit(format!(
                    "{context} static inductance entry #{entry} ({row}, {column}) must be finite and nonzero"
                )));
            }
            for sideband_index in 0..self.num_sidebands {
                let omega = self.omega(sideband_index);
                if omega != 0.0 {
                    let admittance = 1.0 / (omega * inductance);
                    if !admittance.is_finite() || admittance == 0.0 {
                        return Err(HbError::InvalidCircuit(format!(
                            "{context} inductance entry #{entry} ({row}, {column}) has a non-representable admittance at sideband {}",
                            i64::from(self.sideband_min) + sideband_index as i64
                        )));
                    }
                }
            }
        }
        for (branch_index, branch) in self.mna_branches.iter().enumerate() {
            let (node_pos, node_neg, inductance) = match *branch {
                PeriodicMnaBranch::VoltageSource {
                    node_pos, node_neg, ..
                } => (node_pos, node_neg, None),
                PeriodicMnaBranch::Inductor {
                    node_pos,
                    node_neg,
                    inductance,
                } => (node_pos, node_neg, Some(inductance)),
            };
            if node_pos > self.num_nodes || node_neg > self.num_nodes {
                return Err(HbError::InvalidCircuit(format!(
                    "{context} MNA branch #{branch_index} references node pair ({node_pos}, {node_neg}) outside 0..={}",
                    self.num_nodes
                )));
            }
            if node_pos == node_neg {
                return Err(HbError::InvalidCircuit(format!(
                    "{context} MNA branch #{branch_index} has identical terminals"
                )));
            }
            if let Some(inductance) = inductance {
                if !inductance.is_finite() || inductance == 0.0 {
                    return Err(HbError::InvalidCircuit(format!(
                        "{context} inductor branch #{branch_index} must have finite nonzero inductance"
                    )));
                }
                for sideband_index in 0..self.num_sidebands {
                    let omega = self.omega(sideband_index);
                    if !finite_product_is_representable(omega, inductance) {
                        return Err(HbError::InvalidCircuit(format!(
                            "{context} inductor branch #{branch_index} has a non-representable impedance at sideband {}",
                            i64::from(self.sideband_min) + sideband_index as i64
                        )));
                    }
                }
            }
        }
        for (kind, spectra) in [
            ("conductance", self.g_spectra),
            ("capacitance", self.c_spectra),
        ] {
            for (entry, &(row, column, ref spectrum)) in spectra.iter().enumerate() {
                if row >= self.num_nodes || column >= self.num_nodes {
                    return Err(HbError::InvalidCircuit(format!(
                        "{context} periodic {kind} entry #{entry} ({row}, {column}) is outside its {}-node operator",
                        self.num_nodes
                    )));
                }
                if spectrum.is_empty() {
                    return Err(HbError::InvalidCircuit(format!(
                        "{context} periodic {kind} entry #{entry} ({row}, {column}) has no coefficients"
                    )));
                }
                for (harmonic, &coefficient) in spectrum.iter().enumerate() {
                    if !complex_is_finite(coefficient) {
                        return Err(HbError::InvalidCircuit(format!(
                            "{context} periodic {kind} entry #{entry} ({row}, {column}) harmonic {harmonic} is non-finite"
                        )));
                    }
                    if kind == "capacitance" {
                        for sideband_index in 0..self.num_sidebands {
                            let omega = self.omega(sideband_index);
                            if !finite_product_is_representable(omega, coefficient.re)
                                || !finite_product_is_representable(omega, coefficient.im)
                            {
                                return Err(HbError::InvalidCircuit(format!(
                                    "{context} periodic capacitance entry #{entry} ({row}, {column}) harmonic {harmonic} produces a non-representable admittance at sideband {}",
                                    i64::from(self.sideband_min) + sideband_index as i64
                                )));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
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
                if i < n && j < n {
                    let admittance = if omega_k == 0.0 {
                        Complex64::new(inductor_dc_short_admittance(l), 0.0)
                    } else {
                        Complex64::new(0.0, -1.0 / (omega_k * l))
                    };
                    visitor(i * s + k_idx, j * s + k_idx, admittance);
                }
            }
        }

        for (branch_index, branch) in self.mna_branches.iter().enumerate() {
            let branch_unknown = n + branch_index;
            let (node_pos, node_neg) = match *branch {
                PeriodicMnaBranch::VoltageSource {
                    node_pos, node_neg, ..
                }
                | PeriodicMnaBranch::Inductor {
                    node_pos, node_neg, ..
                } => (node_pos, node_neg),
            };
            for k_idx in 0..s {
                let branch_coordinate = branch_unknown * s + k_idx;
                if node_pos > 0 {
                    let node_coordinate = (node_pos - 1) * s + k_idx;
                    visitor(node_coordinate, branch_coordinate, Complex64::new(1.0, 0.0));
                    visitor(branch_coordinate, node_coordinate, Complex64::new(1.0, 0.0));
                }
                if node_neg > 0 {
                    let node_coordinate = (node_neg - 1) * s + k_idx;
                    visitor(
                        node_coordinate,
                        branch_coordinate,
                        Complex64::new(-1.0, 0.0),
                    );
                    visitor(
                        branch_coordinate,
                        node_coordinate,
                        Complex64::new(-1.0, 0.0),
                    );
                }
                if let PeriodicMnaBranch::Inductor { inductance, .. } = *branch {
                    visitor(
                        branch_coordinate,
                        branch_coordinate,
                        Complex64::new(0.0, -self.omega(k_idx) * inductance),
                    );
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
        let size = self.num_unknowns().unwrap_or(0) * self.num_sidebands;
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
        let size = self.num_unknowns().unwrap_or(0) * self.num_sidebands;
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
        let size = self.num_unknowns().unwrap_or(0) * self.num_sidebands;
        let mut transpose = vec![vec![Complex64::new(0.0, 0.0); size]; size];
        self.visit_entries(|row, column, value| transpose[column][row] += value);
        transpose
    }

    fn try_harmonic_block(&self, k_idx: usize, transpose: bool) -> Result<Vec<Complex64>, HbError> {
        let node_count = self.num_nodes;
        let n = self.num_unknowns().ok_or_else(|| {
            HbError::InvalidCircuit(
                "periodic preconditioner node and branch count overflows usize".to_string(),
            )
        })?;
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
            if i < node_count && j < node_count {
                block[i * n + j] += g;
            }
        }
        for &(i, j, c) in self.c_matrix {
            if i < node_count && j < node_count {
                block[i * n + j] += jw * c;
            }
        }
        for &(i, j, l) in self.l_matrix {
            if i < node_count && j < node_count {
                block[i * n + j] += if omega_k == 0.0 {
                    Complex64::new(inductor_dc_short_admittance(l), 0.0)
                } else {
                    Complex64::new(0.0, -1.0 / (omega_k * l))
                };
            }
        }
        for &(i, j, ref spectrum) in self.g_spectra {
            if i < node_count
                && j < node_count
                && let Some(&coefficient) = spectrum.first()
            {
                block[i * n + j] += coefficient;
            }
        }
        for &(i, j, ref spectrum) in self.c_spectra {
            if i < node_count
                && j < node_count
                && let Some(&coefficient) = spectrum.first()
            {
                block[i * n + j] += jw * coefficient;
            }
        }
        for (branch_index, branch) in self.mna_branches.iter().enumerate() {
            let row = node_count + branch_index;
            let (node_pos, node_neg) = match *branch {
                PeriodicMnaBranch::VoltageSource {
                    node_pos, node_neg, ..
                }
                | PeriodicMnaBranch::Inductor {
                    node_pos, node_neg, ..
                } => (node_pos, node_neg),
            };
            if node_pos > 0 {
                let node = node_pos - 1;
                block[node * n + row] += Complex64::new(1.0, 0.0);
                block[row * n + node] += Complex64::new(1.0, 0.0);
            }
            if node_neg > 0 {
                let node = node_neg - 1;
                block[node * n + row] -= Complex64::new(1.0, 0.0);
                block[row * n + node] -= Complex64::new(1.0, 0.0);
            }
            if let PeriodicMnaBranch::Inductor { inductance, .. } = *branch {
                block[row * n + row] -= jw * inductance;
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
            let num_unknowns = operator.num_unknowns().ok_or_else(|| {
                HbError::InvalidCircuit(
                    "periodic preconditioner node and branch count overflows usize".to_string(),
                )
            })?;
            factors.push(super::krylov::LuFactors::factor(block, num_unknowns));
        }
        let num_unknowns = operator.num_unknowns().ok_or_else(|| {
            HbError::InvalidCircuit(
                "periodic preconditioner node and branch count overflows usize".to_string(),
            )
        })?;
        Ok(Self {
            num_nodes: num_unknowns,
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
            .num_unknowns()
            .ok_or_else(|| {
                HbError::InvalidCircuit(
                    "periodic diagonal-preconditioner node and branch count overflows usize"
                        .to_string(),
                )
            })?
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
            .num_unknowns()
            .and_then(|unknowns| unknowns.checked_mul(unknowns))
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
    fn periodic_state_waveforms(
        &mut self,
        state: &HbSolverState,
        context: &str,
    ) -> Result<Vec<Vec<Value>>, HbError> {
        validate_periodic_state(state, self.num_nodes, context)?;
        let mut waveforms = Vec::with_capacity(self.num_nodes);
        for (node, spectrum) in state.x.iter().enumerate() {
            let waveform = self.fft.to_time_domain(spectrum);
            if waveform.len() != self.fft.size() {
                return Err(HbError::InvalidCircuit(format!(
                    "{context} node {node} produced {} time samples, expected {}",
                    waveform.len(),
                    self.fft.size()
                )));
            }
            if let Some((sample, value)) = waveform
                .iter()
                .copied()
                .enumerate()
                .find(|(_, value)| !value.is_finite())
            {
                return Err(HbError::InvalidCircuit(format!(
                    "{context} node {node} time sample {sample} is non-finite ({value})"
                )));
            }
            waveforms.push(waveform);
        }
        Ok(waveforms)
    }

    fn checked_periodic_spectrum(
        &mut self,
        waveform: &[Value],
        harmonic_count: usize,
        context: &str,
    ) -> Result<Vec<Complex64>, HbError> {
        if waveform.len() != self.fft.size() {
            return Err(HbError::InvalidCircuit(format!(
                "{context} has {} samples, expected {}",
                waveform.len(),
                self.fft.size()
            )));
        }
        let max_abs = waveform
            .iter()
            .copied()
            .map(Value::abs)
            .fold(0.0, Value::max);
        if !max_abs.is_finite() {
            return Err(HbError::InvalidCircuit(format!(
                "{context} contains a non-finite sample"
            )));
        }
        if max_abs == 0.0 {
            return Ok(Vec::new());
        }

        let binary_exponent = libm::ilogb(max_abs);
        let mut normalized = Vec::with_capacity(waveform.len());
        for (sample, &value) in waveform.iter().enumerate() {
            let scaled = libm::scalbn(value, -binary_exponent);
            if !scaled.is_finite() || (value != 0.0 && scaled == 0.0) {
                return Err(HbError::InvalidCircuit(format!(
                    "{context} sample {sample} is not representable at the shared Fourier scale"
                )));
            }
            normalized.push(scaled);
        }

        let mut spectrum = self.fft.to_frequency_domain_n(&normalized, harmonic_count);
        let expected_count = harmonic_count.min((self.fft.size() - 1) / 2) + 1;
        if spectrum.len() != expected_count {
            return Err(HbError::InvalidCircuit(format!(
                "{context} produced {} Fourier coefficients, expected {expected_count}",
                spectrum.len()
            )));
        }
        for (harmonic, coefficient) in spectrum.iter_mut().enumerate() {
            if !complex_is_finite(*coefficient) {
                return Err(HbError::InvalidCircuit(format!(
                    "{context} normalized harmonic {harmonic} is non-finite"
                )));
            }
            let normalized_coefficient = *coefficient;
            coefficient.re = libm::scalbn(coefficient.re, binary_exponent);
            coefficient.im = libm::scalbn(coefficient.im, binary_exponent);
            if !complex_is_finite(*coefficient)
                || (normalized_coefficient.re != 0.0 && coefficient.re == 0.0)
                || (normalized_coefficient.im != 0.0 && coefficient.im == 0.0)
            {
                return Err(HbError::InvalidCircuit(format!(
                    "{context} harmonic {harmonic} is not representable at its physical scale"
                )));
            }
        }
        if spectrum
            .iter()
            .all(|coefficient| *coefficient == Complex64::new(0.0, 0.0))
        {
            return Err(HbError::InvalidCircuit(format!(
                "{context} is nonzero but has no representable retained Fourier coefficient"
            )));
        }
        Ok(spectrum)
    }

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
        let v_time = self.periodic_state_waveforms(state, "periodic conductance evaluation")?;

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
                for (device_index, device) in self.nonlinear_devices.iter().enumerate() {
                    for ((i, j), g) in device.jacobian(&node_voltages) {
                        if !g.is_finite() {
                            return Err(HbError::InvalidCircuit(format!(
                                "periodic conductance from {:?} device #{device_index} at ({i}, {j}), time sample {t}, is non-finite",
                                device.device_type
                            )));
                        }
                        if i > n || j > n {
                            return Err(HbError::InvalidCircuit(format!(
                                "periodic conductance from {:?} device #{device_index} uses invalid node pair ({i}, {j}) for a {n}-node solver",
                                device.device_type
                            )));
                        }
                        if i < n && j < n {
                            let sample =
                                &mut g_time.entry((i, j)).or_insert_with(|| vec![0.0; n_time])[t];
                            *sample += g;
                            if !sample.is_finite() {
                                return Err(HbError::InvalidCircuit(format!(
                                    "periodic conductance accumulation at ({i}, {j}), time sample {t}, is non-finite"
                                )));
                            }
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
                        if !entry.value.is_finite() {
                            return Err(device.runtime_error(
                                "periodic conductance evaluation",
                                format!(
                                    "Jacobian program {} entry {} is non-finite at time sample {t}",
                                    entry.program_idx, entry.jacobian_idx
                                ),
                            ));
                        }
                        let Some(prog_locs) = device.jacobian_locs.get(entry.program_idx) else {
                            return Err(device.runtime_error(
                                "periodic conductance evaluation",
                                format!(
                                    "Jacobian program index {} has no mapped location",
                                    entry.program_idx
                                ),
                            ));
                        };
                        let Some(&(row, col)) = prog_locs.get(entry.jacobian_idx) else {
                            return Err(device.runtime_error(
                                "periodic conductance evaluation",
                                format!(
                                    "Jacobian entry index {} has no mapped location in program {}",
                                    entry.jacobian_idx, entry.program_idx
                                ),
                            ));
                        };
                        if let (Some(i), Some(j)) = (row, col) {
                            if i >= n || j >= n {
                                return Err(device.runtime_error(
                                    "periodic conductance evaluation",
                                    format!(
                                        "mapped Jacobian node pair ({i}, {j}) is outside the {n}-node solver"
                                    ),
                                ));
                            }
                            let sample =
                                &mut g_time.entry((i, j)).or_insert_with(|| vec![0.0; n_time])[t];
                            *sample += entry.value;
                            if !sample.is_finite() {
                                return Err(device.runtime_error(
                                    "periodic conductance evaluation",
                                    format!(
                                        "Jacobian accumulation at ({i}, {j}), time sample {t}, is non-finite"
                                    ),
                                ));
                            }
                        }
                    }
                }
            }
        }

        let mut spectra = Vec::with_capacity(g_time.len());
        for ((i, j), waveform) in g_time {
            if waveform.iter().all(|value| *value == 0.0) {
                continue;
            }
            let spectrum = self.checked_periodic_spectrum(
                &waveform,
                harmonic_count,
                &format!("periodic conductance at ({i}, {j})"),
            )?;
            spectra.push((i, j, spectrum));
        }
        Ok(spectra)
    }

    /// Periodic small-signal capacitance spectra around the operating
    /// point: sparse `(i, j, C)` entries from the device charge Jacobians,
    /// in the same conventions as `conductance_spectra`.
    pub(super) fn capacitance_spectra(
        &mut self,
        state: &HbSolverState,
        harmonic_count: usize,
    ) -> Result<Vec<(usize, usize, Vec<Complex64>)>, HbError> {
        let n = self.num_nodes;
        if !self
            .nonlinear_devices
            .iter()
            .any(|d| d.has_charge_storage())
        {
            return Ok(Vec::new());
        }
        let n_time = self.fft.size();
        let v_time = self.periodic_state_waveforms(state, "periodic capacitance evaluation")?;

        let mut c_time: BTreeMap<(usize, usize), Vec<Value>> = BTreeMap::new();
        let mut node_voltages = vec![0.0; n];
        for t in 0..n_time {
            for node in 0..n {
                node_voltages[node] = v_time[node][t];
            }
            for (device_index, device) in self.nonlinear_devices.iter().enumerate() {
                for ((i, j), c) in device.charge_jacobian(&node_voltages) {
                    if !c.is_finite() {
                        return Err(HbError::InvalidCircuit(format!(
                            "periodic capacitance from {:?} device #{device_index} at ({i}, {j}), time sample {t}, is non-finite",
                            device.device_type
                        )));
                    }
                    if i > n || j > n {
                        return Err(HbError::InvalidCircuit(format!(
                            "periodic capacitance from {:?} device #{device_index} uses invalid node pair ({i}, {j}) for a {n}-node solver",
                            device.device_type
                        )));
                    }
                    if i < n && j < n {
                        let sample =
                            &mut c_time.entry((i, j)).or_insert_with(|| vec![0.0; n_time])[t];
                        *sample += c;
                        if !sample.is_finite() {
                            return Err(HbError::InvalidCircuit(format!(
                                "periodic capacitance accumulation at ({i}, {j}), time sample {t}, is non-finite"
                            )));
                        }
                    }
                }
            }
        }

        let mut spectra = Vec::with_capacity(c_time.len());
        for ((i, j), waveform) in c_time {
            if waveform.iter().all(|value| *value == 0.0) {
                continue;
            }
            let spectrum = self.checked_periodic_spectrum(
                &waveform,
                harmonic_count,
                &format!("periodic capacitance at ({i}, {j})"),
            )?;
            spectra.push((i, j, spectrum));
        }
        Ok(spectra)
    }

    /// Solve the sideband-coupled small-signal system at one offset frequency.
    ///
    /// Unknowns are `V[(node, k)]` for k in `[sideband_min, sideband_max]` at
    /// SIGNED absolute frequencies `f_k = offset + k*f0`. Each excitation is
    /// solved against the same admittance matrix; the result is indexed
    /// `[excitation][node][sideband - sideband_min]`.
    #[cfg(test)]
    pub(crate) fn solve_periodic_ac(
        &mut self,
        state: &HbSolverState,
        offset_hz: Value,
        sideband_min: i32,
        sideband_max: i32,
        excitations: &[PeriodicAcExcitation],
    ) -> Result<Vec<Vec<Vec<Complex64>>>, HbError> {
        self.solve_periodic_ac_with_branch_voltages(
            state,
            offset_hz,
            sideband_min,
            sideband_max,
            excitations,
            &[],
        )
    }

    pub(crate) fn solve_periodic_ac_with_branch_voltages(
        &mut self,
        state: &HbSolverState,
        offset_hz: Value,
        sideband_min: i32,
        sideband_max: i32,
        excitations: &[PeriodicAcExcitation],
        branch_voltages: &[Vec<(usize, Complex64)>],
    ) -> Result<Vec<Vec<Vec<Complex64>>>, HbError> {
        let num_nodes = self.num_nodes;
        let num_unknowns = num_nodes
            .checked_add(self.periodic_mna_branches.len())
            .ok_or_else(|| {
                HbError::InvalidCircuit(
                    "PAC result node and branch count overflows usize".to_string(),
                )
            })?;
        let (num_sidebands, expected_size, _) = periodic_sideband_geometry(
            "PAC result reshape",
            num_unknowns,
            sideband_min,
            sideband_max,
        )?;
        let mut results = Vec::new();
        results
            .try_reserve_exact(excitations.len())
            .map_err(|error| {
                HbError::InvalidCircuit(format!("PAC result-column allocation failed: {error}"))
            })?;
        self.solve_periodic_ac_each_with_branch_voltages(
            state,
            offset_hz,
            sideband_min,
            sideband_max,
            excitations,
            branch_voltages,
            |_, solution| {
                if solution.len() != expected_size {
                    return Err(HbError::InvalidCircuit(format!(
                        "PAC solution contains {} values; expected {}",
                        solution.len(),
                        expected_size
                    )));
                }
                let mut by_node = Vec::new();
                by_node.try_reserve_exact(num_nodes).map_err(|error| {
                    HbError::InvalidCircuit(format!("PAC node-row allocation failed: {error}"))
                })?;
                for node in 0..num_nodes {
                    let start = node * num_sidebands;
                    let mut sidebands = Vec::new();
                    sidebands
                        .try_reserve_exact(num_sidebands)
                        .map_err(|error| {
                            HbError::InvalidCircuit(format!(
                                "PAC sideband-row allocation failed for node {node}: {error}"
                            ))
                        })?;
                    sidebands.extend_from_slice(&solution[start..start + num_sidebands]);
                    by_node.push(sidebands);
                }
                results.push(by_node);
                Ok(())
            },
        )?;
        Ok(results)
    }

    /// Solve PAC excitation columns one at a time and release each full-node
    /// solution after the caller consumes it. This keeps the engine's
    /// conversion-matrix path at O(nodes * sidebands) temporary storage
    /// instead of retaining O(nodes * sidebands^2) values per frequency.
    #[cfg(test)]
    pub(crate) fn solve_periodic_ac_each(
        &mut self,
        state: &HbSolverState,
        offset_hz: Value,
        sideband_min: i32,
        sideband_max: i32,
        excitations: &[PeriodicAcExcitation],
        consume: impl FnMut(usize, Vec<Complex64>) -> Result<(), HbError>,
    ) -> Result<(), HbError> {
        self.solve_periodic_ac_each_with_branch_voltages(
            state,
            offset_hz,
            sideband_min,
            sideband_max,
            excitations,
            &[],
            consume,
        )
    }

    pub(crate) fn solve_periodic_ac_each_with_branch_voltages(
        &mut self,
        state: &HbSolverState,
        offset_hz: Value,
        sideband_min: i32,
        sideband_max: i32,
        excitations: &[PeriodicAcExcitation],
        branch_voltages: &[Vec<(usize, Complex64)>],
        mut consume: impl FnMut(usize, Vec<Complex64>) -> Result<(), HbError>,
    ) -> Result<(), HbError> {
        let n = self.num_nodes;
        let num_unknowns = n
            .checked_add(self.periodic_mna_branches.len())
            .ok_or_else(|| {
                HbError::InvalidCircuit("PAC node and branch count overflows usize".to_string())
            })?;
        let (s, size, span) =
            periodic_sideband_geometry("PAC", num_unknowns, sideband_min, sideband_max)?;
        if size == 0 {
            return Err(HbError::InvalidCircuit(
                "PAC requires at least one circuit unknown".to_string(),
            ));
        }
        if excitations.is_empty() {
            return Err(HbError::InvalidCircuit(
                "PAC requires at least one excitation".to_string(),
            ));
        }
        if !branch_voltages.is_empty() && branch_voltages.len() != excitations.len() {
            return Err(HbError::InvalidCircuit(format!(
                "PAC received {} branch-excitation columns for {} node-excitation columns",
                branch_voltages.len(),
                excitations.len()
            )));
        }

        // Dense direct elimination is still faster for small systems.  Large
        // systems build only sparse spectra and per-sideband preconditioner
        // blocks; the dense matrix is materialized lazily only if GMRES fails.
        let try_krylov = self.config.use_krylov || size >= super::krylov::KRYLOV_AUTO_THRESHOLD;
        let dense = if try_krylov {
            None
        } else {
            Some(self.assemble_conversion_matrix(state, offset_hz, sideband_min, sideband_max)?)
        };

        let (spectra, cap_spectra) = if try_krylov && self.has_nonlinear_devices() {
            (
                self.conductance_spectra(state, span.max(self.num_harmonics))?,
                self.capacitance_spectra(state, span.max(self.num_harmonics))?,
            )
        } else {
            (Vec::new(), Vec::new())
        };
        let operator = PeriodicConversionOperator {
            num_nodes: n,
            num_sidebands: s,
            sideband_min,
            offset_hz,
            fundamental_hz: self.config.fundamental_freq,
            g_matrix: &self.g_matrix,
            c_matrix: &self.c_matrix,
            l_matrix: &self.l_matrix,
            mna_branches: &self.periodic_mna_branches,
            g_spectra: &spectra,
            c_spectra: &cap_spectra,
        };
        operator.validate("PAC")?;
        let preconditioner = if try_krylov {
            Some(PeriodicPreconditioner::build(&operator, false)?)
        } else {
            None
        };

        for (excitation_index, excitation) in excitations.iter().enumerate() {
            let mut rhs = try_zeroed_complex_values(
                size,
                &format!("PAC right-hand side for excitation {excitation_index}"),
            )?;
            let m_idx = i64::from(excitation.sideband) - i64::from(sideband_min);
            if m_idx < 0 || m_idx >= s as i64 {
                return Err(HbError::InvalidCircuit(format!(
                    "PAC excitation sideband {} outside [{}, {}]",
                    excitation.sideband, sideband_min, sideband_max
                )));
            }
            for &(node, amp) in &excitation.injections {
                if node >= n {
                    return Err(HbError::InvalidCircuit(format!(
                        "PAC excitation at sideband {} references node {node}, outside the {n}-node solver",
                        excitation.sideband
                    )));
                }
                if !complex_is_finite(amp) {
                    return Err(HbError::InvalidCircuit(format!(
                        "PAC excitation at sideband {} has a non-finite injection at node {node} ({:+.6e}{:+.6e}j)",
                        excitation.sideband, amp.re, amp.im
                    )));
                }
                let rhs_index = node * s + m_idx as usize;
                rhs[rhs_index] += amp;
                if !complex_is_finite(rhs[rhs_index]) {
                    return Err(HbError::InvalidCircuit(format!(
                        "PAC excitation at sideband {} overflows while accumulating node {node}",
                        excitation.sideband
                    )));
                }
            }
            if let Some(branch_column) = branch_voltages.get(excitation_index) {
                for &(branch, amplitude) in branch_column {
                    if branch >= self.periodic_mna_branches.len() {
                        return Err(HbError::InvalidCircuit(format!(
                            "PAC excitation at sideband {} references MNA branch {branch}, outside the {}-branch solver",
                            excitation.sideband,
                            self.periodic_mna_branches.len()
                        )));
                    }
                    if !complex_is_finite(amplitude) {
                        return Err(HbError::InvalidCircuit(format!(
                            "PAC excitation at sideband {} has a non-finite branch voltage on MNA branch {branch} ({:+.6e}{:+.6e}j)",
                            excitation.sideband, amplitude.re, amplitude.im
                        )));
                    }
                    let rhs_index = (n + branch) * s + m_idx as usize;
                    rhs[rhs_index] += amplitude;
                    if !complex_is_finite(rhs[rhs_index]) {
                        return Err(HbError::InvalidCircuit(format!(
                            "PAC excitation at sideband {} overflows while accumulating MNA branch {branch}",
                            excitation.sideband
                        )));
                    }
                }
            }
            if rhs.iter().all(|value| *value == Complex64::new(0.0, 0.0)) {
                return Err(HbError::InvalidCircuit(format!(
                    "PAC excitation at sideband {} has no nonzero in-range injection after terminal cancellation",
                    excitation.sideband
                )));
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
                self.qualify_periodic_ac_solution(&operator, &rhs, outcome)?
            } else {
                let matrix = dense.as_ref().ok_or_else(|| {
                    HbError::InvalidCircuit(
                        "PAC direct solve is missing its conversion matrix".to_string(),
                    )
                })?;
                self.solve_complex_linear_system(matrix, &rhs)?
            };

            consume(excitation_index, solution)?;
        }

        Ok(())
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
        let n = self.num_nodes;
        let num_unknowns = n
            .checked_add(self.periodic_mna_branches.len())
            .ok_or_else(|| {
                HbError::InvalidCircuit(
                    "PAC conversion node and branch count overflows usize".to_string(),
                )
            })?;
        let (s, size, span) = periodic_sideband_geometry(
            "PAC conversion assembly",
            num_unknowns,
            sideband_min,
            sideband_max,
        )?;
        if size == 0 {
            return Err(HbError::InvalidCircuit(
                "PAC conversion assembly requires at least one circuit unknown".to_string(),
            ));
        }
        let spectra = if self.has_nonlinear_devices() {
            self.conductance_spectra(state, span.max(self.num_harmonics))?
        } else {
            Vec::new()
        };
        let cap_spectra = if self.has_nonlinear_devices() {
            self.capacitance_spectra(state, span.max(self.num_harmonics))?
        } else {
            Vec::new()
        };

        let operator = PeriodicConversionOperator {
            num_nodes: n,
            num_sidebands: s,
            sideband_min,
            offset_hz,
            fundamental_hz: self.config.fundamental_freq,
            g_matrix: &self.g_matrix,
            c_matrix: &self.c_matrix,
            l_matrix: &self.l_matrix,
            mna_branches: &self.periodic_mna_branches,
            g_spectra: &spectra,
            c_spectra: &cap_spectra,
        };
        operator.validate("PAC conversion assembly")?;
        Ok(operator.to_dense())
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
    ) -> Result<Vec<PeriodicNoiseSource>, HbError> {
        use crate::constants::K_BOLTZMANN as K_B;
        use crate::constants::Q_ELECTRON as Q_E;

        let n = self.num_nodes;
        let n_time = self.fft.size();
        let v_time: Vec<Vec<Value>> = (0..n)
            .map(|node| self.fft.to_time_domain(&state.x[node]))
            .collect();

        // Accumulate per-source intensity waveforms keyed by node pair.
        let mut intensities: Vec<(
            (usize, usize),
            String,
            Vec<super::devices::ScaledNonnegative>,
        )> = Vec::new();
        let mut node_voltages = vec![0.0; n];

        for (d_idx, device) in self.nonlinear_devices.iter().enumerate() {
            let temperature_metadata = self
                .nonlinear_noise_temperatures
                .get(d_idx)
                .copied()
                .ok_or_else(|| {
                    HbError::InvalidCircuit(format!(
                        "nonlinear device {d_idx} has no aligned noise-temperature metadata"
                    ))
                })?;
            let source_temperature = temperature_metadata.resolve(temperature);
            let temperature_dependent = matches!(
                device.device_type,
                NonlinearDeviceType::Nmos
                    | NonlinearDeviceType::Pmos
                    | NonlinearDeviceType::Njfet
                    | NonlinearDeviceType::Pjfet
                    | NonlinearDeviceType::VoltageSwitch
                    | NonlinearDeviceType::CurrentSwitch
            );
            if temperature_dependent
                && (!source_temperature.is_finite() || source_temperature <= 0.0)
            {
                return Err(HbError::InvalidCircuit(format!(
                    "periodic noise source {:?}#{d_idx} absolute temperature must be finite and positive, got {source_temperature} K",
                    device.device_type
                )));
            }
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
                intensities.push((
                    (p, q),
                    label,
                    vec![super::devices::ScaledNonnegative::ZERO; n_time],
                ));
            }
            for t in 0..n_time {
                for node in 0..n {
                    node_voltages[node] = v_time[node][t];
                }
                let values = device
                    .noise_intensities(&node_voltages, source_temperature, Q_E, K_B)
                    .map_err(|reason| {
                        HbError::InvalidCircuit(format!(
                            "periodic noise source {:?}#{d_idx} is invalid at time sample {t}: {reason}",
                            device.device_type
                        ))
                    })?;
                if values.len() != branches.len() {
                    return Err(HbError::InvalidCircuit(format!(
                        "periodic noise source {:?}#{d_idx} produced {} intensities for {} branches",
                        device.device_type,
                        values.len(),
                        branches.len()
                    )));
                }
                for (b, value) in values.iter().enumerate() {
                    intensities[base + b].2[t] = *value;
                }
            }
        }

        let mut sources = Vec::with_capacity(intensities.len());
        for ((p, q), name, waveform) in intensities {
            let (normalized, binary_scale_exponent) = normalize_scaled_noise_waveform(&waveform)
                .map_err(|reason| {
                    HbError::InvalidCircuit(format!(
                        "periodic noise source '{name}' cannot share a safe binary scale: {reason}"
                    ))
                })?;
            let psd = self.fft.to_frequency_domain(&normalized);
            if psd
                .iter()
                .any(|value| !value.re.is_finite() || !value.im.is_finite())
            {
                return Err(HbError::InvalidCircuit(format!(
                    "periodic noise source '{name}' Fourier coefficients are non-finite"
                )));
            }
            sources.push(PeriodicNoiseSource {
                name,
                node_pos: p,
                node_neg: q,
                psd,
                binary_scale_exponent,
                flicker: None,
            });
        }
        Ok(sources)
    }

    /// Accept a matrix-free PAC solution only after an independent
    /// componentwise backward-error check.
    ///
    /// The matrix helper certifies a plain transpose. Visiting every PAC
    /// entry at its transposed coordinate therefore certifies the original
    /// forward equation `Y*x=b` without allocating `Y` or `Y^T`.
    fn qualify_periodic_ac_solution(
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
                        visitor(column, row, value);
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
                        "PAC matrix-free solve: {} iterations, componentwise certified \
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
                    "PAC matrix-free solution was not certified after {} iterations \
                     (reported relative residual {:.2e}: {}); using bounded dense recovery",
                    outcome.iterations,
                    outcome.relative_residual,
                    error
                );
                let matrix = operator.to_dense();
                self.solve_complex_linear_system(&matrix, rhs)
            }
            Err(error) => Err(HbError::InvalidCircuit(format!(
                "PAC forward {size}x{size} iterative linear solve is uncertified after {} \
                 iterations (reported normwise relative residual {:.3e}): {error}",
                outcome.iterations, outcome.relative_residual
            ))),
        }
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
            let normalized_pos = (source.node_pos < n).then_some(source.node_pos);
            let normalized_neg = (source.node_neg < n).then_some(source.node_neg);
            if normalized_pos == normalized_neg {
                return Err(HbError::InvalidCircuit(format!(
                    "pnoise source '{}' has identical terminals and no effective injection",
                    source.name
                )));
            }
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
        let num_unknowns = n
            .checked_add(self.periodic_mna_branches.len())
            .ok_or_else(|| {
                HbError::InvalidCircuit("pnoise node and branch count overflows usize".to_string())
            })?;
        let (s, size, span) =
            periodic_sideband_geometry("pnoise", num_unknowns, sideband_min, sideband_max)?;
        if size == 0 {
            return Err(HbError::InvalidCircuit(
                "pnoise requires at least one circuit unknown".to_string(),
            ));
        }

        let try_krylov = self.config.use_krylov || size >= super::krylov::KRYLOV_AUTO_THRESHOLD;
        let (spectra, cap_spectra) = if self.has_nonlinear_devices() {
            (
                self.conductance_spectra(state, span.max(self.num_harmonics))?,
                self.capacitance_spectra(state, span.max(self.num_harmonics))?,
            )
        } else {
            (Vec::new(), Vec::new())
        };
        let operator = PeriodicConversionOperator {
            num_nodes: n,
            num_sidebands: s,
            sideband_min,
            offset_hz,
            fundamental_hz: self.config.fundamental_freq,
            g_matrix: &self.g_matrix,
            c_matrix: &self.c_matrix,
            l_matrix: &self.l_matrix,
            mna_branches: &self.periodic_mna_branches,
            g_spectra: &spectra,
            c_spectra: &cap_spectra,
        };
        operator.validate("pnoise")?;

        // Adjoint solve with the plain (unconjugated) transpose.
        let out_idx = usize::try_from(-i64::from(sideband_min)).map_err(|_| {
            HbError::InvalidCircuit("pnoise sideband-zero index exceeds this platform".to_string())
        })?;
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

            // Pass one determines the common binary exponent without retaining
            // O(sidebands^2) terms. Pass two streams the same deterministic
            // products into the compensated accumulator.
            let mut common_exponent = None;
            let mut term_count = visit_white_noise_terms(
                &gains,
                &source.psd,
                source.binary_scale_exponent,
                |term| {
                    validate_scaled_complex(term)?;
                    if !term.is_zero() {
                        common_exponent = Some(
                            common_exponent
                                .map_or(term.exponent, |current: i32| current.max(term.exponent)),
                        );
                    }
                    Ok(())
                },
            )
            .map_err(|reason| {
                HbError::InvalidCircuit(format!(
                    "pnoise source '{}' white-noise term is invalid: {reason}",
                    source.name
                ))
            })?;
            let (mut contribution, mut absolute_sum) = if let Some(exponent) = common_exponent {
                let mut accumulator = ScaledComplexAccumulator::new(exponent);
                visit_white_noise_terms(
                    &gains,
                    &source.psd,
                    source.binary_scale_exponent,
                    |term| accumulator.add(term),
                )
                .map_err(|reason| {
                    HbError::InvalidCircuit(format!(
                        "pnoise source '{}' white-noise accumulation is invalid: {reason}",
                        source.name
                    ))
                })?;
                accumulator.finish().map_err(|reason| {
                    HbError::InvalidCircuit(format!(
                        "pnoise source '{}' white-noise accumulation is invalid: {reason}",
                        source.name
                    ))
                })?
            } else {
                (Complex64::new(0.0, 0.0), 0.0)
            };

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
                    let sideband_frequency = (k as f64).mul_add(omega0_hz, offset_hz);
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
            fundamental_hz: 1.0e6,
            g_matrix: g,
            c_matrix: c,
            l_matrix: &[],
            mna_branches: &[],
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
    fn nonlinear_noise_waveform_normalization_is_scaled_and_fail_closed() {
        let exact_subnormal_exponent = 2000 - 1074;
        let waveform = [
            super::devices::ScaledNonnegative::ZERO,
            super::devices::ScaledNonnegative {
                mantissa: 1.5,
                exponent: 2000,
            },
            super::devices::ScaledNonnegative {
                mantissa: 1.0,
                exponent: exact_subnormal_exponent,
            },
        ];
        let (normalized, exponent) = normalize_scaled_noise_waveform(&waveform).unwrap();
        assert_eq!(exponent, 2000);
        assert_eq!(normalized[0], 0.0);
        assert_eq!(normalized[1], 1.5);
        assert_eq!(normalized[2].to_bits(), Value::from_bits(1).to_bits());

        let vanished = [
            super::devices::ScaledNonnegative {
                mantissa: 1.0,
                exponent: 2000,
            },
            super::devices::ScaledNonnegative {
                mantissa: 1.0,
                exponent: 2000 - 1075,
            },
        ];
        assert!(normalize_scaled_noise_waveform(&vanished).is_err());

        let rounded = [
            super::devices::ScaledNonnegative {
                mantissa: 1.0,
                exponent: 2000,
            },
            super::devices::ScaledNonnegative {
                mantissa: 1.5,
                exponent: exact_subnormal_exponent,
            },
        ];
        assert!(normalize_scaled_noise_waveform(&rounded).is_err());

        for invalid in [Value::NAN, -1.0, 0.5, 2.0] {
            assert!(
                normalize_scaled_noise_waveform(&[super::devices::ScaledNonnegative {
                    mantissa: invalid,
                    exponent: 0,
                },])
                .is_err()
            );
        }

        let config = HbConfig::new(1.0e6).with_harmonics(1);
        let mut solver = HbSolver::new(config, 1);
        let maximum_sample = super::devices::ScaledNonnegative {
            mantissa: libm::scalbn(Value::MAX, -1023),
            exponent: 1023,
        };
        let maximum_waveform = vec![maximum_sample; solver.fft.size()];
        let (normalized_maximum, maximum_exponent) =
            normalize_scaled_noise_waveform(&maximum_waveform).unwrap();
        let maximum_spectrum = solver.fft.to_frequency_domain(&normalized_maximum);
        assert!(
            maximum_spectrum
                .iter()
                .all(|value| value.re.is_finite() && value.im.is_finite())
        );
        let reconstructed_dc = libm::scalbn(maximum_spectrum[0].re, maximum_exponent);
        assert_eq!(reconstructed_dc.to_bits(), Value::MAX.to_bits());
        assert!(
            maximum_spectrum
                .iter()
                .skip(1)
                .all(|value| value.norm() <= 16.0 * Value::EPSILON)
        );

        let zero_waveform = vec![super::devices::ScaledNonnegative::ZERO; solver.fft.size()];
        let (normalized_zero, zero_exponent) =
            normalize_scaled_noise_waveform(&zero_waveform).unwrap();
        assert_eq!(zero_exponent, 0);
        assert!(normalized_zero.iter().all(|value| *value == 0.0));
        assert!(
            solver
                .fft
                .to_frequency_domain(&normalized_zero)
                .iter()
                .all(|value| *value == Complex64::new(0.0, 0.0))
        );
    }

    #[test]
    fn nonlinear_noise_temperature_metadata_stays_aligned_and_defaults_to_ambient() {
        let config = HbConfig::new(1.0e6).with_harmonics(1);
        let mut offset_solver = HbSolver::new(config.clone(), 4);
        offset_solver.add_diode(2, 3, 1.0e-14, 1.0);
        offset_solver.add_nonlinear_device_with_noise_temperature_offset(
            NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2.0e-5, 0.04),
            150.0,
        );

        let mut ambient_solver = HbSolver::new(config.clone(), 4);
        ambient_solver.add_diode(2, 3, 1.0e-14, 1.0);
        ambient_solver
            .add_nonlinear_device(NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2.0e-5, 0.04));

        let mut state = HbSolverState::new(4, 1);
        state.x[0][0] = Complex64::new(2.0, 0.0);
        state.x[1][0] = Complex64::new(2.0, 0.0);
        let channel_source = |sources: Vec<PeriodicNoiseSource>| {
            sources
                .into_iter()
                .find(|source| source.name.contains("Nmos#1 channel thermal"))
                .expect("the MOS source stays aligned after an earlier diode")
        };
        let offset = channel_source(offset_solver.device_noise_sources(&state, 300.15).unwrap());
        let ambient = channel_source(ambient_solver.device_noise_sources(&state, 450.15).unwrap());
        assert!(
            offset.psd[0].re.is_finite() && offset.psd[0].re > 0.0,
            "offset-temperature MOS source must exercise a nonzero PSD"
        );
        assert!(
            ambient.psd[0].re.is_finite() && ambient.psd[0].re > 0.0,
            "ambient-temperature MOS source must exercise a nonzero PSD"
        );
        assert_eq!(offset.binary_scale_exponent, ambient.binary_scale_exponent);
        assert_eq!(offset.psd, ambient.psd);

        let mut absolute_solver = HbSolver::new(config.clone(), 4);
        absolute_solver.add_diode(2, 3, 1.0e-14, 1.0);
        absolute_solver.add_nonlinear_device_with_absolute_noise_temperature(
            NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2.0e-5, 0.04),
            300.15,
        );
        let mut reference_solver = HbSolver::new(config.clone(), 4);
        reference_solver.add_diode(2, 3, 1.0e-14, 1.0);
        reference_solver
            .add_nonlinear_device(NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2.0e-5, 0.04));
        let absolute = channel_source(
            absolute_solver
                .device_noise_sources(&state, 1.0e20)
                .expect("absolute TEMP survives an extreme ambient temperature"),
        );
        let reference = channel_source(
            reference_solver
                .device_noise_sources(&state, 300.15)
                .unwrap(),
        );
        assert_eq!(
            absolute.binary_scale_exponent,
            reference.binary_scale_exponent
        );
        assert_eq!(absolute.psd, reference.psd);

        let mut invalid_solver = HbSolver::new(config.clone(), 4);
        invalid_solver.add_nonlinear_device_with_noise_temperature_offset(
            NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2.0e-5, 0.04),
            -300.15,
        );
        let error = invalid_solver
            .device_noise_sources(&state, 300.15)
            .expect_err("zero absolute channel-noise temperature must fail");
        let message = error.to_string();
        assert!(message.contains("Nmos#0") && message.contains("finite and positive"));

        for invalid_offset in [Value::NAN, Value::INFINITY, Value::NEG_INFINITY] {
            let mut invalid_solver = HbSolver::new(config.clone(), 4);
            invalid_solver.add_nonlinear_device_with_noise_temperature_offset(
                NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2.0e-5, 0.04),
                invalid_offset,
            );
            let error = invalid_solver
                .device_noise_sources(&state, 300.15)
                .expect_err("non-finite channel-noise temperature must fail");
            assert!(error.to_string().contains("finite and positive"));
        }
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
            .solve_periodic_noise(&state, 0.0, 0, 0, 0, None, std::slice::from_ref(&flicker))
            .expect_err("1/f noise at an exact zero-frequency sideband is singular");
        assert!(singular.to_string().contains("singular 1/f"));

        let below_legacy_floor = solver
            .solve_periodic_noise(&state, 1.0e-6, 0, 0, 0, None, &[flicker])
            .expect("nonzero sub-millihertz flicker density remains physical");
        assert!((below_legacy_floor[0] - 1.0e6).abs() <= 4.0 * Value::EPSILON * 1.0e6);
    }

    #[test]
    fn periodic_noise_term_materialization_preserves_representable_extremes() {
        let materialize_product = |first, second, third, exponent| {
            let term = scaled_complex_product3(first, second, third, exponent)
                .expect("scaled white-noise product remains valid");
            materialize_scaled_complex_sum(&[term])
                .expect("scaled white-noise product remains representable")
                .0
        };
        let large_white = materialize_product(
            Complex64::new(1.0e200, 0.0),
            Complex64::new(1.0e200, 0.0),
            Complex64::new(1.0e-200, 0.0),
            0,
        );
        assert!((large_white.re - 1.0e200).abs() <= 8.0 * Value::EPSILON * 1.0e200);
        assert_eq!(large_white.im, 0.0);

        let small_white = materialize_product(
            Complex64::new(1.0e-200, 0.0),
            Complex64::new(1.0e-200, 0.0),
            Complex64::new(1.0e200, 0.0),
            0,
        );
        assert!((small_white.re - 1.0e-200).abs() <= 8.0 * Value::EPSILON * 1.0e-200);

        let externally_scaled = materialize_product(
            Complex64::new(libm::scalbn(1.0, 500), 0.0),
            Complex64::new(libm::scalbn(1.0, 500), 0.0),
            Complex64::new(1.0, 0.0),
            -1100,
        );
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
    fn white_noise_terms_are_rounded_only_after_the_complete_sum() {
        let below_range_term = ScaledComplex {
            mantissa: Complex64::new(1.5, 0.0),
            exponent: -1075,
        };
        let (sum, absolute_sum) = materialize_scaled_complex_sum(&[below_range_term; 3]).unwrap();
        assert_eq!(sum.re.to_bits(), Value::from_bits(2).to_bits());
        assert_eq!(sum.im, 0.0);
        assert_eq!(absolute_sum.to_bits(), Value::from_bits(2).to_bits());
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
        let branches = [
            PeriodicMnaBranch::VoltageSource {
                node_pos: 1,
                node_neg: 0,
                source_index: 0,
            },
            PeriodicMnaBranch::Inductor {
                node_pos: 2,
                node_neg: 0,
                inductance: 1.0e-6,
            },
        ];
        let operator = PeriodicConversionOperator {
            num_nodes: 2,
            num_sidebands: 5,
            sideband_min: -2,
            offset_hz: 3.0e6,
            fundamental_hz: 1.0e6,
            g_matrix: &g,
            c_matrix: &c,
            l_matrix: &[],
            mna_branches: &branches,
            g_spectra: &spectra,
            c_spectra: &cap_spectra,
        };
        let dense = operator.to_dense();
        let dense_transpose = operator.to_dense_transpose();
        let x = (0..20)
            .map(|index| Complex64::new(index as Value * 0.13 - 0.4, 0.2 - index as Value * 0.07))
            .collect::<Vec<_>>();
        let forward = operator.apply(&x);
        let transpose = operator.apply_transpose(&x);
        for row in 0..20 {
            let expected_forward = (0..20).map(|col| dense[row][col] * x[col]).sum();
            let expected_transpose = (0..20).map(|col| dense[col][row] * x[col]).sum();
            assert_close(forward[row], expected_forward);
            assert_close(transpose[row], expected_transpose);
            for column in 0..20 {
                assert_eq!(dense_transpose[row][column], dense[column][row]);
            }
        }
    }

    #[test]
    fn periodic_mna_krylov_solve_includes_branch_unknowns_and_branch_rhs() {
        let mut config = HbConfig::new(1.0e6).with_harmonics(1);
        config.use_krylov = true;
        let mut solver = HbSolver::new(config, 1);
        solver.add_conductance(0, 0, 2.0);
        solver.add_periodic_voltage_source_branch(1, 0, 0);
        let mut state = HbSolverState::new(1, 1);
        state.converged = true;
        let excitation = PeriodicAcExcitation {
            sideband: 0,
            injections: Vec::new(),
        };
        let mut retained = None;
        solver
            .solve_periodic_ac_each_with_branch_voltages(
                &state,
                1.0e4,
                0,
                0,
                &[excitation],
                &[vec![(0, Complex64::new(1.0, 0.0))]],
                |_, solution| {
                    retained = Some(solution);
                    Ok(())
                },
            )
            .expect("exact branch solve is certified");
        let solution = retained.expect("one PAC column is returned");
        assert_eq!(solution.len(), 2);
        assert_close(solution[0], Complex64::new(1.0, 0.0));
        assert_close(solution[1], Complex64::new(-2.0, 0.0));
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
            fundamental_hz: 1.0,
            g_matrix: &g,
            c_matrix: &[],
            l_matrix: &[],
            mna_branches: &[],
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
    fn pac_forward_rejects_normwise_false_convergence_and_recovers_when_small() {
        // For nonsymmetric A = [[1, 0], [eps, 1]], b = [1, 0], x = [1, 0]
        // has a normwise residual below GMRES_REL_TOL but leaves the second
        // equation wrong at its entire physical scale.
        let eps = 1.0e-12;
        assert!(eps < super::super::krylov::GMRES_REL_TOL);
        let g = vec![(0, 0, 1.0), (1, 0, eps), (1, 1, 1.0)];
        let operator = PeriodicConversionOperator {
            num_nodes: 2,
            num_sidebands: 1,
            sideband_min: 0,
            offset_hz: 1.0,
            fundamental_hz: 1.0,
            g_matrix: &g,
            c_matrix: &[],
            l_matrix: &[],
            mna_branches: &[],
            g_spectra: &[],
            c_spectra: &[],
        };
        let solver = HbSolver::new(HbConfig::new(1.0).with_harmonics(1), 2);
        let rhs = vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)];
        let false_convergence = super::super::krylov::gmres(
            &|input| operator.apply(input),
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
            .qualify_periodic_ac_solution(&operator, &rhs, false_convergence)
            .expect("a small uncertified PAC solution uses bounded dense recovery");
        assert_close(recovered[0], Complex64::new(1.0, 0.0));
        assert_eq!(recovered[1].im, 0.0);
        assert!(
            (recovered[1].re + eps).abs() <= 8.0 * Value::EPSILON * eps,
            "forward correction {} differs from {} at its own scale",
            recovered[1].re,
            -eps
        );
    }

    #[test]
    fn pac_forward_never_materializes_dense_fallback_at_krylov_threshold() {
        let dimension = super::super::krylov::KRYLOV_AUTO_THRESHOLD;
        let eps = 1.0e-12;
        let mut g = (0..dimension)
            .map(|index| (index, index, 1.0))
            .collect::<Vec<_>>();
        g.push((1, 0, eps));
        let operator = PeriodicConversionOperator {
            num_nodes: dimension,
            num_sidebands: 1,
            sideband_min: 0,
            offset_hz: 1.0,
            fundamental_hz: 1.0,
            g_matrix: &g,
            c_matrix: &[],
            l_matrix: &[],
            mna_branches: &[],
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
            .qualify_periodic_ac_solution(&operator, &rhs, false_convergence)
            .expect_err("large uncertified PAC solutions must fail without dense allocation");
        let HbError::InvalidCircuit(message) = error else {
            panic!("unexpected error: {error}");
        };
        assert!(message.contains("PAC forward 256x256"), "{message}");
        assert!(message.contains("after 1 iterations"), "{message}");
        assert!(message.contains("backward-error"), "{message}");
    }

    #[test]
    fn conversion_operator_preserves_sub_1e_30_inductor_admittance() {
        let inductance = libm::scalbn(1.0, -100);
        assert!(inductance < 1.0e-30 && inductance > 0.0);
        let offset_hz = 1.0e9;
        let l = [(0, 0, inductance)];
        let operator = PeriodicConversionOperator {
            num_nodes: 1,
            num_sidebands: 1,
            sideband_min: 0,
            offset_hz,
            fundamental_hz: 1.0e6,
            g_matrix: &[],
            c_matrix: &[],
            l_matrix: &l,
            mna_branches: &[],
            g_spectra: &[],
            c_spectra: &[],
        };
        operator.validate("sub-cutoff inductor test").unwrap();
        let expected = Complex64::new(0.0, -1.0 / (2.0 * PI * offset_hz * inductance));
        assert_close(operator.apply(&[Complex64::new(1.0, 0.0)])[0], expected);
        let block = operator.try_harmonic_block(0, false).unwrap();
        assert_close(block[0], expected);
    }

    #[test]
    fn conversion_operator_rejects_capacitance_admittance_underflow() {
        let min_subnormal = Value::from_bits(1);
        let offset_hz = 0.125 / (2.0 * PI);
        let static_c = [(0, 0, min_subnormal)];
        let static_operator = PeriodicConversionOperator {
            num_nodes: 1,
            num_sidebands: 2,
            sideband_min: 0,
            offset_hz,
            fundamental_hz: 1.0 / (2.0 * PI),
            g_matrix: &[],
            c_matrix: &static_c,
            l_matrix: &[],
            mna_branches: &[],
            g_spectra: &[],
            c_spectra: &[],
        };
        let static_error = static_operator
            .validate("static capacitance underflow test")
            .expect_err("a nonzero static capacitance may not round to zero admittance");
        assert!(
            static_error
                .to_string()
                .contains("non-representable admittance"),
            "{static_error}"
        );

        let periodic_c = [(0, 0, vec![Complex64::new(min_subnormal, 0.0)])];
        let periodic_operator = PeriodicConversionOperator {
            num_nodes: 1,
            num_sidebands: 2,
            sideband_min: 0,
            offset_hz,
            fundamental_hz: 1.0 / (2.0 * PI),
            g_matrix: &[],
            c_matrix: &[],
            l_matrix: &[],
            mna_branches: &[],
            g_spectra: &[],
            c_spectra: &periodic_c,
        };
        let periodic_error = periodic_operator
            .validate("periodic capacitance underflow test")
            .expect_err("a nonzero periodic capacitance may not round to zero admittance");
        assert!(
            periodic_error
                .to_string()
                .contains("non-representable admittance"),
            "{periodic_error}"
        );
    }

    #[test]
    fn periodic_spectrum_scales_extreme_waveforms_and_rejects_erasure() {
        let config = HbConfig::new(1.0e6).with_harmonics(1);
        let mut solver = HbSolver::new(config, 1);
        let sample_count = solver.fft.size();

        let maximum = vec![Value::MAX; sample_count];
        let maximum_spectrum = solver
            .checked_periodic_spectrum(&maximum, 1, "maximum waveform")
            .expect("a representable maximum DC coefficient survives the scaled FFT");
        assert_eq!(maximum_spectrum[0], Complex64::new(Value::MAX, 0.0));

        let mut erased = vec![0.0; sample_count];
        erased[0] = Value::from_bits(1);
        let error = solver
            .checked_periodic_spectrum(&erased, 1, "minimum impulse")
            .expect_err("an unrepresentable nonzero Fourier coefficient must not become zero");
        assert!(
            error
                .to_string()
                .contains("not representable at its physical scale"),
            "{error}"
        );

        let mut mixed_scale = vec![0.0; sample_count];
        mixed_scale[0] = Value::MAX;
        mixed_scale[1] = -Value::MAX;
        mixed_scale[2] = sample_count as Value * Value::from_bits(1);
        let error = solver
            .checked_periodic_spectrum(&mixed_scale, 1, "mixed-scale cancellation waveform")
            .expect_err("normalization may not erase a representable DC residue");
        assert!(
            error
                .to_string()
                .contains("not representable at the shared Fourier scale"),
            "{error}"
        );
    }

    #[test]
    fn periodic_spectra_preserve_sub_1e_30_device_derivatives() {
        let config = HbConfig::new(1.0e6).with_harmonics(1);
        let mut solver = HbSolver::new(config, 4);
        let device = NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.0, 5.0e-31, 0.0)
            .with_intrinsic_gate(5.0e-31);
        solver.add_nonlinear_device(device);
        let mut state = HbSolverState::new(4, 1);
        state.x[0][0] = Complex64::new(0.5, 0.0);
        state.x[1][0] = Complex64::new(1.0, 0.0);

        let conductance = solver.conductance_spectra(&state, 1).unwrap();
        let gm = conductance
            .iter()
            .find(|(row, column, _)| *row == 0 && *column == 1)
            .map(|(_, _, spectrum)| spectrum[0].re)
            .expect("the sub-1e-30 MOS transconductance remains in the spectrum");
        assert!(gm != 0.0 && gm.abs() < 1.0e-30, "gm={gm}");

        let capacitance = solver.capacitance_spectra(&state, 1).unwrap();
        assert!(
            capacitance.iter().any(|(_, _, spectrum)| {
                let dc = spectrum[0];
                dc != Complex64::new(0.0, 0.0) && dc.norm() < 1.0e-30
            }),
            "the sub-1e-30 MOS charge derivative was discarded: {capacitance:?}"
        );
    }

    #[test]
    fn periodic_operator_and_state_validation_fail_closed() {
        let invalid_spectra = [(1, 0, vec![Complex64::new(1.0, 0.0)])];
        let operator = PeriodicConversionOperator {
            num_nodes: 1,
            num_sidebands: 1,
            sideband_min: 0,
            offset_hz: 1.0,
            fundamental_hz: 1.0,
            g_matrix: &[],
            c_matrix: &[],
            l_matrix: &[],
            mna_branches: &[],
            g_spectra: &invalid_spectra,
            c_spectra: &[],
        };
        let error = operator
            .validate("invalid periodic spectrum test")
            .expect_err("out-of-range periodic stamps must fail before solving");
        assert!(error.to_string().contains("outside its 1-node operator"));

        let config = HbConfig::new(1.0e6).with_harmonics(1);
        let mut solver = HbSolver::new(config, 2);
        solver.add_nonlinear_device(NonlinearDeviceInstance::diode(0, 1, 1.0e-14, 1.0));
        let mut state = HbSolverState::new(2, 1);
        state.x[0][0] = Complex64::new(Value::NAN, 0.0);
        let error = solver
            .conductance_spectra(&state, 1)
            .expect_err("non-finite periodic state must fail before device evaluation");
        assert!(
            error
                .to_string()
                .contains("node 0 harmonic 0 is non-finite")
        );
    }

    #[test]
    fn periodic_ac_rejects_invalid_or_cancelled_excitations() {
        let config = HbConfig::new(1.0e6).with_harmonics(1);
        let mut solver = HbSolver::new(config, 1);
        solver.add_conductance(0, 0, 1.0);
        let state = HbSolverState::new(1, 1);

        for (excitation, expected) in [
            (
                PeriodicAcExcitation {
                    sideband: 0,
                    injections: vec![(1, Complex64::new(1.0, 0.0))],
                },
                "outside the 1-node solver",
            ),
            (
                PeriodicAcExcitation {
                    sideband: 0,
                    injections: vec![(0, Complex64::new(Value::NAN, 0.0))],
                },
                "non-finite injection",
            ),
            (
                PeriodicAcExcitation {
                    sideband: 0,
                    injections: vec![
                        (0, Complex64::new(1.0, 0.0)),
                        (0, Complex64::new(-1.0, 0.0)),
                    ],
                },
                "no nonzero in-range injection",
            ),
        ] {
            let error = solver
                .solve_periodic_ac(&state, 1.0e3, 0, 0, &[excitation])
                .expect_err("invalid PAC excitation evidence must fail closed");
            assert!(error.to_string().contains(expected), "{error}");
        }
    }

    #[test]
    fn periodic_ac_propagates_a_stream_consumer_abort_between_columns() {
        let config = HbConfig::new(1.0e6).with_harmonics(1);
        let mut solver = HbSolver::new(config, 1);
        solver.add_conductance(0, 0, 1.0);
        let state = HbSolverState::new(1, 1);
        let excitations = [
            PeriodicAcExcitation {
                sideband: 0,
                injections: vec![(0, Complex64::new(1.0, 0.0))],
            },
            PeriodicAcExcitation {
                sideband: 0,
                injections: vec![(0, Complex64::new(2.0, 0.0))],
            },
        ];
        let mut consumed = 0;
        let error = solver
            .solve_periodic_ac_each(&state, 1.0e3, 0, 0, &excitations, |_, _| {
                consumed += 1;
                Err(HbError::Aborted)
            })
            .expect_err("the consumer abort must stop the column stream");
        assert!(matches!(error, HbError::Aborted));
        assert_eq!(consumed, 1, "no later excitation may be solved or consumed");
    }

    #[test]
    fn periodic_allocation_failure_is_returned_as_a_solver_error() {
        let error = try_zeroed_complex_values(usize::MAX, "PAC allocation regression")
            .expect_err("an impossible capacity must fail without panicking");
        assert!(error.to_string().contains("allocation failed"), "{error}");
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
            fundamental_hz: 1.0,
            g_matrix: &g,
            c_matrix: &[],
            l_matrix: &[],
            mna_branches: &[],
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
            fundamental_hz: 1.0,
            g_matrix: &g,
            c_matrix: &[],
            l_matrix: &[],
            mna_branches: &[],
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
            fundamental_hz: 1.0,
            g_matrix: &g,
            c_matrix: &[],
            l_matrix: &[],
            mna_branches: &[],
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
            fundamental_hz: 1.0,
            g_matrix: &[],
            c_matrix: &[],
            l_matrix: &[],
            mna_branches: &[],
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
