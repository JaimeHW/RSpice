//! Binary-scaled complex products and compensated covariance accumulation.
//! Keep transfer and density exponents separate until the complete sum is
//! representable. Shared by periodic and independent-tone noise folding.
use crate::{Complex64, Value};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ScaledComplex {
    pub(crate) mantissa: Complex64,
    pub(crate) exponent: i32,
}

impl ScaledComplex {
    pub(crate) const ZERO: Self = Self {
        mantissa: Complex64::new(0.0, 0.0),
        exponent: 0,
    };

    pub(crate) fn is_zero(self) -> bool {
        self.mantissa.re == 0.0 && self.mantissa.im == 0.0
    }
}

pub(crate) fn scaled_complex_product3(
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
        // A small component can be the entire result after correlated paths
        // cancel. Normalization must not erase or round it before that sum.
        *slot = Complex64::new(
            scale_complex_component_exactly(value.re, -factor_exponent)?,
            scale_complex_component_exactly(value.im, -factor_exponent)?,
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
        scale_complex_component_exactly(mantissa.re, -mantissa_exponent)?,
        scale_complex_component_exactly(mantissa.im, -mantissa_exponent)?,
    );
    Ok(ScaledComplex {
        mantissa: normalized,
        exponent,
    })
}

pub(crate) fn scale_complex_component_exactly(
    component: Value,
    shift: i32,
) -> Result<Value, &'static str> {
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

pub(crate) fn validate_scaled_complex(term: ScaledComplex) -> Result<(), &'static str> {
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

pub(crate) struct ScaledComplexAccumulator {
    common_exponent: i32,
    real_sum: Value,
    real_compensation: Value,
    imag_sum: Value,
    imag_compensation: Value,
    absolute_sum: Value,
    absolute_compensation: Value,
}

impl ScaledComplexAccumulator {
    pub(crate) fn new(common_exponent: i32) -> Self {
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

    pub(crate) fn add(&mut self, term: ScaledComplex) -> Result<(), &'static str> {
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

    pub(crate) fn normalized_sum(&self) -> Result<(Complex64, Value), &'static str> {
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
            return Err("the completed common-scale noise sum is invalid");
        }
        Ok((normalized, normalized_absolute_sum))
    }

    pub(crate) fn into_scaled(self) -> Result<ScaledComplex, &'static str> {
        let (value, _) = self.normalized_sum()?;
        scaled_complex_product3(value, Complex64::ONE, Complex64::ONE, self.common_exponent)
    }

    pub(crate) fn finish(self) -> Result<(Complex64, Value), &'static str> {
        let (normalized, normalized_absolute_sum) = self.normalized_sum()?;
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
            return Err("the completed noise sum is outside the finite binary64 range");
        }
        if (normalized.re != 0.0 && contribution.re == 0.0)
            || (normalized.im != 0.0 && contribution.im == 0.0)
        {
            return Err("a nonzero completed noise component is below the binary64 range");
        }
        Ok((contribution, physical_absolute_sum))
    }
}

pub(crate) fn scaled_flicker_term(
    gain: Complex64,
    gain_binary_exponent: i32,
    coefficient: Value,
    coefficient_binary_exponent: i32,
    frequency: Value,
    exponent: Value,
) -> Result<ScaledComplex, &'static str> {
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
        return Ok(ScaledComplex::ZERO);
    }
    let gain_exponent = libm::ilogb(gain_scale);
    let normalized_gain = Complex64::new(
        libm::scalbn(gain.re, -gain_exponent),
        libm::scalbn(gain.im, -gain_exponent),
    );
    let normalized_power = normalized_gain.norm_sqr();
    if !normalized_power.is_finite() || normalized_power <= 0.0 {
        return Err("the normalized flicker transfer magnitude is invalid");
    }
    if frequency == 0.0 {
        return if exponent < 0.0 {
            Ok(ScaledComplex::ZERO)
        } else if exponent == 0.0 {
            scaled_flicker_term(
                gain,
                gain_binary_exponent,
                coefficient,
                coefficient_binary_exponent,
                1.0,
                exponent,
            )
        } else {
            Err("positive-exponent flicker density is singular at zero frequency")
        };
    }

    let frequency_power = frequency.powf(exponent);
    let (mantissa, power) = if frequency_power.is_normal() {
        let (mantissa, power) = crate::numerics::product_binary_normalization(
            &[coefficient, normalized_power],
            &[frequency_power],
        );
        let power = i64::from(power)
            + i64::from(coefficient_binary_exponent)
            + 2 * (i64::from(gain_exponent) + i64::from(gain_binary_exponent));
        (
            mantissa,
            i32::try_from(power)
                .map_err(|_| "the flicker term exceeds the retained binary exponent range")?,
        )
    } else {
        // Normalize the coefficient before combining it with the bounded
        // transfer power. Keep the gain's binary scale as a separate power
        // of two: adding it to the source's i32 scale could overflow before
        // the frequency law cancels it.
        let coefficient_exponent = libm::ilogb(coefficient);
        let mantissa = libm::scalbn(coefficient, -coefficient_exponent) * normalized_power;
        crate::numerics::power_product_binary_normalization(
            mantissa,
            coefficient_binary_exponent,
            &[
                (
                    2.0,
                    Value::from(coefficient_exponent)
                        + 2.0 * (Value::from(gain_exponent) + Value::from(gain_binary_exponent)),
                ),
                (frequency, -exponent),
            ],
        )
    };
    if !mantissa.is_finite() || mantissa <= 0.0 {
        return Err("the nonzero flicker term exceeds the retained binary exponent range");
    }
    scaled_complex_product3(
        Complex64::new(mantissa, 0.0),
        Complex64::new(1.0, 0.0),
        Complex64::new(1.0, 0.0),
        power,
    )
}

/// Colored covariance without materializing either source density or transfer
/// product before their exponents cancel. The phase is left intact.
pub(crate) fn scaled_flicker_cross_term(
    left: ScaledComplex,
    right: ScaledComplex,
    coefficient: Value,
    coefficient_binary_exponent: i32,
    frequency: Value,
    exponent: Value,
) -> Result<ScaledComplex, &'static str> {
    validate_scaled_complex(left)?;
    validate_scaled_complex(right)?;
    if !coefficient.is_finite()
        || coefficient < 0.0
        || !frequency.is_finite()
        || frequency < 0.0
        || !exponent.is_finite()
    {
        return Err("a flicker-covariance factor is invalid");
    }
    if left.is_zero() || right.is_zero() || coefficient == 0.0 {
        return Ok(ScaledComplex::ZERO);
    }
    if frequency == 0.0 {
        return if exponent < 0.0 {
            Ok(ScaledComplex::ZERO)
        } else if exponent == 0.0 {
            scaled_flicker_cross_term(
                left,
                right,
                coefficient,
                coefficient_binary_exponent,
                1.0,
                exponent,
            )
        } else {
            Err("positive-exponent flicker density is singular at zero frequency")
        };
    }
    let cross = scaled_complex_product3(
        left.mantissa,
        right.mantissa.conj(),
        Complex64::new(1.0, 0.0),
        0,
    )?;
    let gain_power =
        i64::from(left.exponent) + i64::from(right.exponent) + i64::from(cross.exponent);
    let frequency_power = frequency.powf(exponent);
    let (mantissa, power) = if frequency_power.is_normal() {
        let (mantissa, power) =
            crate::numerics::product_binary_normalization(&[coefficient], &[frequency_power]);
        let power = i64::from(power) + i64::from(coefficient_binary_exponent) + gain_power;
        (
            mantissa,
            i32::try_from(power)
                .map_err(|_| "the flicker covariance exceeds the retained binary exponent range")?,
        )
    } else {
        let coefficient_exponent = libm::ilogb(coefficient);
        crate::numerics::power_product_binary_normalization(
            libm::scalbn(coefficient, -coefficient_exponent),
            coefficient_binary_exponent,
            &[
                (2.0, Value::from(coefficient_exponent) + gain_power as Value),
                (frequency, -exponent),
            ],
        )
    };
    if !mantissa.is_finite() || mantissa <= 0.0 {
        return Err("the nonzero flicker covariance exceeds the retained binary exponent range");
    }
    scaled_complex_product3(
        cross.mantissa,
        Complex64::new(mantissa, 0.0),
        Complex64::new(1.0, 0.0),
        power,
    )
}
