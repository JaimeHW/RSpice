//! The discrete ngspice BJT Weil operator, separate from exact transport.

use rspice_veriloga_runtime::arithmetic::ScaledValue as S;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::engine::transient) struct WeilHistory {
    pub time: f64,
    pub previous_dt: f64,
    pub delay: f64,
    pub input: f64,
    pub output: f64,
    pub previous_output: f64,
}

pub(super) struct WeilEvaluation {
    pub output: f64,
    pub correction: f64,
    correction_weight: S,
}

fn sum(terms: &[[S; 2]]) -> Result<S, String> {
    S::sum_products_div(terms.iter().copied(), S::new(1.0))
        .map_err(|error| format!("Weil arithmetic: {error:?}"))
}

impl WeilEvaluation {
    pub(super) fn apply_correction_derivative(&self, derivative: f64) -> Result<f64, String> {
        if !derivative.is_finite() {
            return Err("Weil input derivative must be finite".into());
        }
        let result = self.correction_weight.multiply_binary64(derivative);
        if !result.is_finite() {
            return Err("Weil correction derivative is not representable".into());
        }
        Ok(result)
    }
}

impl WeilHistory {
    pub(in crate::engine::transient) fn new(delay: f64, input: f64) -> Result<Self, String> {
        let history = Self {
            time: 0.0,
            previous_dt: 0.0,
            delay,
            input,
            output: input,
            previous_output: input,
        };
        history.validate()?;
        Ok(history)
    }

    pub(in crate::engine::transient) fn validate(&self) -> Result<(), String> {
        if ![
            self.time,
            self.previous_dt,
            self.delay,
            self.input,
            self.output,
            self.previous_output,
        ]
        .iter()
        .all(|value| value.is_finite())
            || self.time < 0.0
            || self.delay <= 0.0
            || self.previous_dt < 0.0
            || self.previous_dt > self.time
            || (self.previous_dt == 0.0
                && (self.time != 0.0 || self.output != self.previous_output))
            || (self.time == 0.0 && self.previous_dt != 0.0)
            || (self.time == 0.0 && self.input != self.output)
        {
            return Err("invalid accepted GP Weil history".into());
        }
        Ok(())
    }

    /// y = y1 + c*(F-y1) + d*(y1-y2), c=3a^2/D, d=r/D,
    /// D=1+3a+3a^2, a=h/tau, r=h/h_previous. This is the discrete
    /// bjtload.c update; the global charge integrator does not replace it.
    pub(super) fn evaluate(
        &self,
        time: f64,
        input: f64,
        delay: f64,
    ) -> Result<WeilEvaluation, String> {
        self.validate()?;
        if !time.is_finite()
            || time < self.time
            || !input.is_finite()
            || delay.to_bits() != self.delay.to_bits()
        {
            return Err("GP Weil trial has invalid time/input or a changed nominal delay".into());
        }
        let dt = time - self.time;
        let a = S::new(dt).divide(S::new(delay));
        let one = S::new(1.0);
        let three_a = a.multiply(S::new(3.0));
        let three_a_squared = three_a.multiply(a);
        let denominator = sum(&[[one, one], [three_a, one], [three_a_squared, one]])?;
        let input_weight = three_a_squared.divide(denominator);
        // Form the complement directly: 1-c can round to zero for tiny tau.
        let memory_weight = sum(&[[one, one], [three_a, one]])?.divide(denominator);
        let rate_weight = if self.previous_dt == 0.0 {
            S::new(0.0) // the two initial outputs are identical
        } else {
            S::new(dt)
                .divide(S::new(self.previous_dt))
                .divide(denominator)
        };
        let old = S::new(self.output);
        let older = S::new(self.previous_output);
        let input = S::new(input);
        let output = sum(&[
            [old, one],
            [input_weight, input],
            [input_weight.negated(), old],
            [rate_weight, old],
            [rate_weight.negated(), older],
        ])?
        .binary64();
        let correction = sum(&[
            [memory_weight, old],
            [memory_weight.negated(), input],
            [rate_weight, old],
            [rate_weight.negated(), older],
        ])?
        .binary64();
        if !output.is_finite() || !correction.is_finite() {
            return Err("GP Weil current is not representable".into());
        }
        Ok(WeilEvaluation {
            output,
            correction,
            correction_weight: memory_weight.negated(),
        })
    }

    /// Preparing never mutates accepted state; commit replaces it only after
    /// the complete device family has validated its accepted candidates.
    pub(in crate::engine::transient) fn prepare(
        &self,
        time: f64,
        input: f64,
        delay: f64,
    ) -> Result<Self, String> {
        if time <= self.time {
            return Err("accepted GP Weil time must advance".into());
        }
        let value = self.evaluate(time, input, delay)?;
        Ok(Self {
            time,
            previous_dt: time - self.time,
            delay,
            input,
            output: value.output,
            previous_output: self.output,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gp_weil_preserves_constant_input_and_scaled_correction() {
        for (delay, time) in [(1e-300, 1e300), (1e300, 1e-300), (1.0, 1.0)] {
            let original = WeilHistory::new(delay, 2e250).unwrap();
            let value = original.evaluate(time, 2e250, delay).unwrap();
            assert_eq!(value.output, 2e250);
            assert_eq!(value.correction, 0.0);
            assert_eq!(original.time, 0.0);
            let accepted = original.prepare(time, 2e250, delay).unwrap();
            assert_eq!(accepted.output, 2e250);
            accepted.validate().unwrap();
        }
        let history = WeilHistory::new(1e-300, 1e308).unwrap();
        let value = history.evaluate(1e300, -1e308, 1e-300).unwrap();
        assert_eq!(value.output, -1e308);
        assert!((value.correction / 2e-292 - 1.0).abs() < 1e-14);
        let action = value.apply_correction_derivative(1e300).unwrap();
        assert!((action / -1e-300 - 1.0).abs() < 1e-14);
        assert!(history.prepare(0.0, 1.0, 1e-300).is_err());
        assert!(history.evaluate(1.0, 1.0, 2e-300).is_err());
        assert!(history.evaluate(1.0, f64::NAN, 1e-300).is_err());
    }
}
