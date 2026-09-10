//! Semiconductor device models and shared junction equations.
//!
//! Includes diode/BJT models and equations shared by native and periodic solvers.

mod bjt;
mod diode;
mod limiting;

#[cfg(test)]
pub(crate) use bjt::BJT_ACCEPTED_NONLINEAR_RUNTIME_TAG;
pub(crate) use bjt::{
    AcceptedBjtChargeSnapshotCheckpoint, AcceptedBjtNonlinearCheckpoint,
    BJT_ACCEPTED_CHARGE_SNAPSHOT_STATE_VALUE_COUNT, BJT_ACCEPTED_NONLINEAR_STATE_VALUE_COUNT,
    BJT_DYNAMIC_CHARGE_COUNT, BJT_EXTERNAL_STATE_DIM, BJT_INTERNAL_STATE_DIM, BjtChargeBranch,
    BjtChargeSnapshot, VBIC_ACCEPTED_NONLINEAR_STATE_VALUE_COUNT,
};
pub use bjt::{Bjt, BjtType};
pub(crate) use diode::{
    AcceptedDiodeNonlinearCheckpoint, DIODE_ACCEPTED_NONLINEAR_RUNTIME_TAG, DiodeNonlinearState,
    ResolvedDiodeJunction,
};
pub use diode::{Diode, DiodeLevel};

/// MOS1 body-effect threshold and -dVth/dVbs in the effective polarity
/// frame. Forward body bias uses the same linear continuation as mos1load.c.
/// Its derivative follows that continuation (the reference's 1/(2*sarg)
/// approximation is not the derivative of its forward-bias threshold).
#[inline]
pub(crate) fn mos1_threshold(
    vto: crate::Value,
    gamma: crate::Value,
    phi: crate::Value,
    sqrt_phi: crate::Value,
    vbs: crate::Value,
) -> (crate::Value, crate::Value) {
    // A disabled body effect must not evaluate an irrelevant barrier whose
    // intermediate phi-Vbs can overflow for finite terminal voltages.
    if gamma == 0.0 {
        return (vto, 0.0);
    }
    let sarg = if vbs == 0.0 {
        sqrt_phi
    } else if vbs < 0.0 {
        (phi - vbs).max(0.0).sqrt()
    } else {
        (sqrt_phi - vbs / (sqrt_phi + sqrt_phi)).max(0.0)
    };
    let slope = if sarg <= 0.0 {
        0.0
    } else if vbs > 0.0 {
        gamma / (sqrt_phi + sqrt_phi)
    } else {
        gamma / (sarg + sarg)
    };
    (vto + gamma * (sarg - sqrt_phi), slope)
}

/// Dimensionless NLEV=3 channel-noise shape, to be multiplied by
/// gamma_noise * GDSNOI * beta * overdrive. Keep the factors separate so a
/// small model coefficient can compensate a large geometry or bias factor.
pub(crate) fn mos_nlev3_noise_shape(
    overdrive: crate::Value,
    vds: crate::Value,
    vdsat: crate::Value,
) -> Result<crate::Value, &'static str> {
    if !overdrive.is_finite() || !vds.is_finite() || vds < 0.0 || !vdsat.is_finite() {
        return Err("MOS NLEV=3 channel-noise bias must be finite with nonnegative active VDS");
    }
    if overdrive <= 0.0 {
        return Ok(0.0);
    }
    if overdrive <= vds {
        return Ok(1.0);
    }
    if vdsat <= 0.0 {
        return Err("MOS NLEV=3 channel noise requires positive saturation voltage in triode");
    }
    let alpha = 1.0 - vds / vdsat;
    // (1+alpha+alpha^2)/(1+alpha), without squaring a large alpha.
    let shape = alpha + 1.0 / (1.0 + alpha);
    if !shape.is_finite() || shape < 0.0 {
        return Err("MOS NLEV=3 channel-noise shape is not finite and nonnegative");
    }
    Ok(shape)
}

/// Graded-junction depletion charge and its voltage derivative. The forward
/// continuation is anchored at the knee so both quantities remain continuous.
/// Callers own model validation and temperature/geometry scaling.
pub(crate) fn depletion_charge_and_capacitance(
    voltage: crate::Value,
    capacitance: crate::Value,
    potential: crate::Value,
    grading: crate::Value,
    forward_coefficient: crate::Value,
) -> (crate::Value, crate::Value) {
    if capacitance == 0.0 {
        return (0.0, 0.0);
    }
    if voltage == 0.0 || grading == 0.0 {
        return (capacitance * voltage, capacitance);
    }
    let offset = (-forward_coefficient).mul_add(potential, voltage);
    let below_knee = offset.is_sign_negative();
    // At the knee use FC directly: rounding FC*Phi and then dividing by
    // a subnormal Phi can otherwise turn a valid FC<1 into exactly one.
    let ratio = if below_knee {
        -voltage / potential
    } else {
        -forward_coefficient
    };
    let log = if below_knee && voltage > 0.5 * potential {
        // Near Phi, subtract the voltages first (Sterbenz-exact), rather
        // than rounding V/Phi to one before taking log1p.
        ((potential - voltage) / potential).ln()
    } else if ratio.is_infinite() {
        // Here |V|/Phi overflows and the +1 is below floating-point resolution.
        (-voltage).ln() - potential.ln()
    } else {
        ratio.ln_1p()
    };
    let (charge, slope_exponent) = if ratio.abs() < crate::Value::MIN_POSITIVE {
        // ln(1+x) = x to working precision, but rounding a subnormal V/Phi
        // before multiplying can lose digits or erase a representable charge.
        let exponent = if below_knee {
            crate::numerics::scaled_exp_product(&[grading - 1.0, voltage], &[potential], 0.0)
        } else {
            (grading - 1.0) * forward_coefficient
        };
        let integral = if exponent == 0.0 {
            1.0
        } else {
            exponent.exp_m1() / exponent
        };
        if below_knee {
            (
                crate::numerics::scaled_exp_product(&[capacitance, voltage, integral], &[], 0.0),
                crate::numerics::scaled_exp_product(&[grading, voltage], &[potential], 0.0),
            )
        } else {
            (
                crate::numerics::scaled_exp_product(
                    &[capacitance, potential, forward_coefficient, integral],
                    &[],
                    0.0,
                ),
                grading * forward_coefficient,
            )
        }
    } else {
        let exponent = (1.0 - grading) * log;
        let charge = if exponent.abs() < 0.5 {
            // The exprel form includes the removable grading=1 limit and
            // does not lose Q when (1-M)*log underflows to zero.
            let integral = if exponent == 0.0 {
                1.0
            } else {
                exponent.exp_m1() / exponent
            };
            crate::numerics::scaled_exp_product(&[-capacitance, potential, log, integral], &[], 0.0)
        } else if exponent > 700.0 {
            // expm1(a) = exp(a)*(1-exp(-a)); keep exp(a) scaled until
            // multiplication by C0*Phi/(1-M) has restored the final range.
            crate::numerics::scaled_exp_product(
                &[capacitance, potential, (-exponent).exp_m1()],
                &[1.0 - grading],
                exponent,
            )
        } else {
            crate::numerics::scaled_exp_product(
                &[-capacitance, potential, exponent.exp_m1()],
                &[1.0 - grading],
                0.0,
            )
        };
        (charge, -grading * log)
    };
    let slope = crate::numerics::scaled_exp_product(&[capacitance], &[], slope_exponent);
    if below_knee {
        (charge, slope)
    } else {
        let (offset_scale, offset_fraction) =
            if offset.is_normal() || forward_coefficient == 0.0 || !offset.is_finite() {
                (offset, 1.0)
            } else {
                // Normalize to V before subtracting FC*Phi. Scaling only the
                // already rounded offset would preserve its subnormal error.
                let voltage_power = libm::ilogb(voltage);
                let potential_power = libm::ilogb(potential);
                let scaled_voltage = libm::scalbn(voltage, -voltage_power);
                let scaled_potential = libm::scalbn(potential, -potential_power);
                let scaled_fc = libm::scalbn(forward_coefficient, potential_power - voltage_power);
                let fraction = (-scaled_fc).mul_add(scaled_potential, scaled_voltage);
                (libm::scalbn(1.0, voltage_power), fraction)
            };
        // Form each integral directly from the model coefficient. Reusing a
        // rounded subnormal capacitance here would magnify its lost digits.
        let linear_charge = crate::numerics::scaled_exp_product(
            &[capacitance, offset_scale, offset_fraction],
            &[],
            slope_exponent,
        );
        let quadratic_charge = crate::numerics::scaled_exp_product(
            &[
                capacitance,
                offset_scale,
                offset_fraction,
                offset_scale,
                offset_fraction,
                grading,
                0.5,
            ],
            &[potential, 1.0 - forward_coefficient],
            slope_exponent,
        );
        let slope_change = crate::numerics::scaled_exp_product(
            &[capacitance, grading, offset_scale, offset_fraction],
            &[potential, 1.0 - forward_coefficient],
            slope_exponent,
        );
        (
            charge + linear_charge + quadratic_charge,
            slope + slope_change,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::depletion_charge_and_capacitance;
    use crate::Value;

    #[test]
    fn nonfinite_junction_trials_propagate_without_integer_overflow() {
        // A rejected Newton trial may contain NaN/infinity. Do not pass
        // ilogb's nonfinite sentinels into integer exponent arithmetic.
        for voltage in [Value::NAN, Value::INFINITY, Value::NEG_INFINITY] {
            for potential in [1e-200, 1.0, 1e200] {
                let (q, _) = depletion_charge_and_capacitance(voltage, 1.0, potential, 0.5, 0.5);
                assert!(!q.is_finite());
            }
        }
    }

    #[test]
    fn depletion_charge_retains_representable_results_across_the_float_range() {
        // Independent 400-decimal-digit evaluations of the defining integral,
        // rounded once to f64. These cover product/exponential range loss,
        // tiny voltage ratios and rounding at a subnormal forward knee.
        for (inputs, expected) in [
            ([-0.5, 1e150, 1e200, 0.5, 0.0], [-5e149, 1e150]),
            ([0.5, 1e150, 1e200, 0.5, 0.0], [5e149, 1e150]),
            ([-1e200, 1e-200, 1e-200, 0.5, 0.5], [-2e-200, 0.0]),
            ([-1e200, 1e150, 1e-100, 1.5, 0.5], [-2e50, 1e-300]),
            (
                [0.5, 1e-300, 1.0, 1100.0, 0.5],
                [6.179_702_133_982_647e27, 1.358_298_529_049_385_9e31],
            ),
            (
                [-1e200, 1e300, 1e-150, 0.5, 0.5],
                [Value::NEG_INFINITY, 1e125],
            ),
            (
                [-Value::from_bits(1), 1e300, 1e300, 0.5, 0.5],
                [-4.940_656_458_412_466e-24, 1e300],
            ),
            (
                [
                    9.999_999_999_999_999e-301,
                    1e150,
                    1e-300,
                    1.0,
                    1.0_f64.next_down(),
                ],
                [3.633_586_450_916_892_3e-149, 6.032_057_205_060_441e165],
            ),
            (
                [Value::from_bits(1), 1e300, Value::from_bits(1), 0.5, 0.5],
                [7.261_134_152_882_507e-24, 2.121_320_343_559_642_6e300],
            ),
            (
                [0.5, Value::from_bits(1), 1e-300, 0.5, 0.9],
                [9.764_829_715_627_73e-24, 3.905_931_886_251_092e-23],
            ),
            (
                [0.5, 1e-300, 1e-300, 0.5, 0.9],
                [1.976_423_537_605_237_7, 7.905_694_150_420_951],
            ),
        ] {
            let [v, c, phi, m, fc] = inputs;
            let (q, cap) = depletion_charge_and_capacitance(v, c, phi, m, fc);
            for (actual, expected) in [q, cap].into_iter().zip(expected) {
                if expected.is_infinite() {
                    assert_eq!(actual, expected, "{inputs:?}");
                } else {
                    assert!(
                        (actual - expected).abs()
                            <= expected.abs() * 2e-13 + 2.0 * Value::from_bits(1),
                        "{inputs:?}: {actual:e} vs {expected:e}"
                    );
                }
            }
        }
        for fc in [0.0, 0.5, 1.0_f64.next_down()] {
            assert_eq!(
                depletion_charge_and_capacitance(0.0, 1e150, 1e200, 0.5, fc),
                (0.0, 1e150)
            );
        }
    }
}
