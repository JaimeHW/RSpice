//! Range-preserving coefficients for periodic distributional observations.
use super::*;

impl ImpulseDerivative {
    /// Signed coefficient times `(2*pi*f)^order`, normalized to the period.
    /// Keep the binary exponent separate so a large derivative factor can be
    /// cancelled by a small authored coefficient without intermediate overflow.
    pub(crate) fn periodic_rate(
        &self,
        frequency: Value,
        weight: Value,
        duration: Value,
        scale: Value,
    ) -> Result<Value, String> {
        if !frequency.is_finite()
            || frequency < 0.0
            || !weight.is_finite()
            || !duration.is_finite()
            || duration <= 0.0
            || !scale.is_finite()
            || scale <= 0.0
            || !self.coefficient.is_finite()
            || self.coefficient == 0.0
            || self.order == 0
        {
            return Err("invalid periodic impulse derivative".into());
        }
        if frequency == 0.0 || weight == 0.0 {
            return Ok(0.0);
        }
        let normalize = |value: Value| {
            let (fraction, exponent) = libm::frexp(value);
            (fraction, i64::from(exponent))
        };
        let multiply = |left: (Value, i64), right: (Value, i64)| {
            let (fraction, exponent) = normalize(left.0 * right.0);
            (fraction, left.1 + right.1 + exponent)
        };
        let mut product = multiply(normalize(self.coefficient), normalize(weight));
        for divisor in [duration, scale] {
            let (fraction, exponent) = normalize(divisor);
            let divided = normalize(product.0 / fraction);
            product = (divided.0, product.1 - exponent + divided.1);
        }
        let mut base = multiply(normalize(std::f64::consts::TAU), normalize(frequency));
        let mut order = self.order;
        // At most 32 iterations, independent of the authored derivative order.
        while order != 0 {
            if order & 1 != 0 {
                product = multiply(product, base);
            }
            order >>= 1;
            if order != 0 {
                base = multiply(base, base);
            }
        }
        let value = libm::scalbn(
            product.0,
            product.1.clamp(i32::MIN as i64, i32::MAX as i64) as i32,
        );
        if !value.is_finite() || value == 0.0 {
            return Err("periodic impulse derivative is not representable".into());
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derivative_periodic_rate_preserves_compensating_binary_ranges() {
        for (coefficient, frequency, expected) in [(1e-300, 1e200, 1e100), (1e300, 1e-200, 1e-100)]
        {
            let point = CurrentImpulseDerivative {
                time: 0.5,
                order: 2,
                coefficient,
            };
            let actual = point.periodic_rate(frequency, -3.0, 2.0, 1.0).unwrap();
            let expected = expected * std::f64::consts::TAU.powi(2) * -1.5;
            assert!((actual / expected - 1.0).abs() < 1e-14);
            assert_eq!(point.periodic_rate(0.0, 1.0, 1.0, 1.0).unwrap(), 0.0);
        }
        let point = CurrentImpulseDerivative {
            time: 0.5,
            order: u32::MAX,
            coefficient: 1.0,
        };
        assert!(point.periodic_rate(2.0, 1.0, 1.0, 1.0).is_err());
        assert!(point.periodic_rate(0.01, 1.0, 1.0, 1.0).is_err());
    }

    #[test]
    fn derivative_trace_round_trip_validation_and_accounting_are_lossless() {
        let point = CurrentImpulseDerivative {
            time: 0.25,
            order: 1,
            coefficient: -1e-20,
        };
        let mut trace = CurrentImpulseTrace {
            owner: CurrentImpulseOwner::Branch {
                branch_name: "H1".into(),
            },
            complete: false,
            points: vec![],
            derivatives: vec![point, CurrentImpulseDerivative { order: 3, ..point }],
        };
        trace.validate(0.0, 1.0).unwrap();
        assert_eq!(
            serde_json::from_str::<CurrentImpulseTrace>(&serde_json::to_string(&trace).unwrap())
                .unwrap(),
            trace
        );
        assert!(trace.has_impulses_in_window(0.0, 0.25));
        assert!(!trace.has_impulses_in_window(0.25, 1.0));
        assert_eq!(
            current_impulse_value_count(Some(&[trace.clone()])),
            trace.owner.value_count() + 6
        );
        for bad in [
            CurrentImpulseDerivative { order: 0, ..point },
            CurrentImpulseDerivative {
                time: -1.0,
                ..point
            },
            CurrentImpulseDerivative {
                coefficient: 0.0,
                ..point
            },
            CurrentImpulseDerivative {
                coefficient: Value::NAN,
                ..point
            },
        ] {
            trace.derivatives[0] = bad;
            assert!(trace.validate(0.0, 1.0).is_err());
        }
        trace.derivatives = vec![point, point];
        assert!(trace.validate(0.0, 1.0).is_err());
    }
}
