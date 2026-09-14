//! Prospective error control for the physical current stored in phase history.

use super::*;
use rspice_veriloga_runtime::arithmetic::ScaledValue;
use rspice_veriloga_runtime::transport_delay::DelayBuffer;

#[derive(Clone, Copy, Debug)]
pub(in crate::engine::transient) struct PhaseInterpolationControl {
    pub device_index: usize,
    pub normalized_error: Value,
    pub next_step: Value,
}

impl PhaseInterpolationControl {
    pub fn ensure_acceptable(self, circuit: &crate::CircuitData) -> Result<(), SimulationError> {
        if self.normalized_error > 1.0 {
            return Err(SimulationError::Circuit(format!(
                "BJT '{}' phase-history interpolation error exceeds the configured current tolerance by {:.6e}; the interval cannot be accepted",
                circuit.bjts.devices[self.device_index].name, self.normalized_error
            )));
        }
        Ok(())
    }

    /// A prescribed grid or exhausted retry budget cannot waive this error.
    pub fn retry_step(
        self,
        attempted: Value,
        minimum: Value,
        locked: bool,
        exhausted: bool,
        circuit: &crate::CircuitData,
    ) -> Result<Option<Value>, SimulationError> {
        if self.normalized_error <= 1.0 {
            return Ok(None);
        }
        let retry = self.next_step.max(minimum);
        if locked || exhausted || !retry.is_finite() || retry >= attempted {
            self.ensure_acceptable(circuit)?;
        }
        Ok(Some(retry))
    }
}

/// Estimate the largest linear-interpolation error on the interval being
/// proposed, before that interval becomes immutable delay history. For a
/// quadratic signal this is exactly |f[t0,t1,t2]|*(t2-t1)^2/4. Irregular
/// accepted timestamps, not integration-order history, determine the estimate.
/// This is a smooth-signal estimator; discontinuities need sided samples and
/// scheduled arrivals, and cannot be exempted from history validation.
pub(in crate::engine::transient) fn phase_interpolation_control(
    history: &DelayBuffer,
    time: Value,
    current: Value,
    reltol: Value,
    abstol: Value,
    device_index: usize,
) -> Result<PhaseInterpolationControl, String> {
    if !time.is_finite()
        || !current.is_finite()
        || !reltol.is_finite()
        || reltol < 0.0
        || !abstol.is_finite()
        || abstol <= 0.0
    {
        return Err(
            "phase interpolation requires finite time/current and valid current tolerances".into(),
        );
    }
    let mut samples = history.accepted_samples().rev();
    let (previous_time, previous) = samples
        .next()
        .ok_or("phase interpolation has no accepted anchor")?;
    let dt = time - previous_time;
    if !dt.is_finite() || dt <= 0.0 {
        return Err("phase interpolation candidate must follow its accepted anchor".into());
    }
    let s = ScaledValue::new;
    let tolerance = s(abstol).plus(s(reltol).multiply(s(current.abs().max(previous.abs()))));
    let (error, order) = if let Some((older_time, older)) = samples.next() {
        let previous_dt = previous_time - older_time;
        if !previous_dt.is_finite() || previous_dt <= 0.0 {
            return Err("phase interpolation history has no positive previous interval".into());
        }
        // Evaluate the complete divided difference before rounding. Even a
        // subnormal physical current or time interval may have a finite error
        // relative to its tolerance. Do not form overflowing slopes or h*h.
        let error = ScaledValue::sum_triple_products_ratio(
            [
                [s(current), s(dt), s(previous_dt)],
                [s(-previous), s(dt), s(previous_dt)],
                [s(-previous), s(dt), s(dt)],
                [s(older), s(dt), s(dt)],
            ]
            .into_iter(),
            [
                [s(4.0), s(previous_dt), s(previous_dt)],
                [s(4.0), s(previous_dt), s(dt)],
            ]
            .into_iter(),
        )
        .map_err(|error| format!("phase interpolation divided difference: {error:?}"))?;
        (error, 2)
    } else {
        // OP/UIC gives a constant prehistory, not a measured input slope. The
        // first interval is controlled by its change from that anchor. Once
        // two physical samples exist, use their measured divided difference.
        (s(current).plus(s(-previous)), 1)
    };
    let normalized_error = error.divide(tolerance).binary64().abs();
    if normalized_error.is_nan() {
        return Err("phase interpolation error is indeterminate".into());
    }
    let scale = if order == 1 {
        0.9 / normalized_error
    } else {
        0.9 / normalized_error.sqrt()
    }
    .clamp(0.1, 2.0);
    Ok(PhaseInterpolationControl {
        device_index,
        normalized_error,
        next_step: dt * scale,
    })
}

#[cfg(test)]
mod tests;
