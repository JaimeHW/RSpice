//! One arithmetic path for computed and retained AC sensitivity projections.

use super::{Complex64, SensitivityUnavailability, SensitivityValue, Value};
use rspice_veriloga_runtime::arithmetic::{ArithmeticError, ScaledValue};

pub(crate) struct AcSensitivityDerived {
    pub normalized: SensitivityValue<Complex64>,
    pub magnitude: SensitivityValue<Value>,
    pub phase: SensitivityValue<Value>,
}

impl AcSensitivityDerived {
    pub fn new(
        nominal: Value,
        output: Complex64,
        derivative: Complex64,
    ) -> Result<Self, ArithmeticError> {
        if [nominal, output.re, output.im, derivative.re, derivative.im]
            .iter()
            .any(|value| !value.is_finite())
        {
            return Err(ArithmeticError::NonFiniteTerm);
        }
        let magnitude = SensitivityValue::magnitude(output, derivative);
        if output.re == 0.0 && output.im == 0.0 {
            return Ok(Self {
                normalized: SensitivityValue::unavailable(SensitivityUnavailability::ZeroOutput),
                magnitude,
                phase: SensitivityValue::unavailable(SensitivityUnavailability::ZeroOutput),
            });
        }
        let one = ScaledValue::new(1.0);
        let re = ScaledValue::new(output.re);
        let im = ScaledValue::new(output.im);
        let dr = ScaledValue::new(derivative.re);
        let di = ScaledValue::new(derivative.im);
        let parameter = ScaledValue::new(nominal);
        let norm_squared = [[re, re, one], [im, im, one]];
        let ratio = |numerator: [[ScaledValue; 3]; 2]| {
            ScaledValue::sum_triple_products_ratio(numerator.into_iter(), norm_squared.into_iter())
                .map(SensitivityValue::from_scaled)
        };
        Ok(Self {
            normalized: ratio([[re, dr, parameter], [im, di, parameter]])?
                .zip(ratio([[re, di, parameter], [im.negated(), dr, parameter]])?)
                .map(|(re, im)| Complex64::new(re, im)),
            magnitude,
            phase: ratio([[re, di, one], [im.negated(), dr, one]])?,
        })
    }
}
