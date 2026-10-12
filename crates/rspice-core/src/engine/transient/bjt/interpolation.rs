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
    pub(in crate::engine::transient) fn ensure_acceptable(
        self,
        circuit: &crate::CircuitData,
    ) -> Result<(), SimulationError> {
        if self.normalized_error > 1.0 {
            return Err(SimulationError::Circuit(format!(
                "BJT '{}' phase-history interpolation error exceeds the configured current tolerance by {:.6e}; the interval cannot be accepted",
                circuit.bjts.devices[self.device_index].name, self.normalized_error
            )));
        }
        Ok(())
    }

    /// A prescribed grid or exhausted retry budget cannot waive this error.
    pub(in crate::engine::transient) fn retry_step(
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

/// The budget one phase-history interval's interpolation error is held to.
#[derive(Clone, Copy, Debug)]
pub(in crate::engine::transient) struct PhaseInterpolationTolerance {
    /// Relative current tolerance, on the larger of the interval's two ends.
    pub reltol: Value,
    /// Absolute current tolerance.
    pub abstol: Value,
    /// The current error that is an error in the delayed junction voltage no
    /// larger than the transient truncation tolerance holds the junction
    /// voltage itself to: `gm * (lte_abstol + lte_reltol * |vbe|)`. A
    /// small-signal reading, so it applies to the smooth chord estimate only.
    /// Zero leaves only the current tolerance.
    pub input_referred: Value,
}

/// Estimate the largest linear-interpolation error on the interval being
/// proposed, before that interval becomes immutable delay history. For a
/// quadratic signal this is exactly |f[t0,t1,t2]|*(t2-t1)^2/4. Irregular
/// accepted timestamps, not integration-order history, determine the estimate.
/// This is a smooth-signal estimator; discontinuities need sided samples and
/// scheduled arrivals, and cannot be exempted from history validation.
#[cfg(test)]
pub(in crate::engine::transient) fn phase_interpolation_control(
    history: &DelayBuffer,
    time: Value,
    current: Value,
    reltol: Value,
    abstol: Value,
    device_index: usize,
) -> Result<PhaseInterpolationControl, String> {
    phase_interpolation_control_with_slope(
        history,
        None,
        time,
        current,
        PhaseInterpolationTolerance {
            reltol,
            abstol,
            input_referred: 0.0,
        },
        None,
        device_index,
    )
}

/// [`phase_interpolation_control`] with an outgoing anchor slope, the full
/// tolerance, and the transport delay the history is read through.
///
/// A fixed delay reads the new interval `[t1, t2]` only at `t - delay` for the
/// solve at `t2` and for later times `t > t2`: never before `t2 - delay`. When
/// the interval is longer than twice the delay, the quadratic error model's
/// largest chord error on that read window is `|f[t0,t1,t2]| * d * (h - d)`
/// (with `d` the delay and `h` the interval) rather than the whole interval's
/// `|f[t0,t1,t2]| * h^2 / 4`; every value the history will ever supply is held
/// to the tolerance, and only those.
pub(in crate::engine::transient) fn phase_interpolation_control_with_slope(
    history: &DelayBuffer,
    outgoing_slope: Option<Value>,
    time: Value,
    current: Value,
    tolerance: PhaseInterpolationTolerance,
    delay: Option<Value>,
    device_index: usize,
) -> Result<PhaseInterpolationControl, String> {
    let PhaseInterpolationTolerance {
        reltol,
        abstol,
        input_referred,
    } = tolerance;
    if outgoing_slope.is_some_and(|slope| !slope.is_finite())
        || !time.is_finite()
        || !current.is_finite()
        || !reltol.is_finite()
        || reltol < 0.0
        || !abstol.is_finite()
        || abstol <= 0.0
        || !input_referred.is_finite()
        || input_referred < 0.0
        || delay.is_some_and(|delay| !delay.is_finite() || delay <= 0.0)
    {
        return Err(
            "phase interpolation requires finite time/current and valid current tolerances".into(),
        );
    }
    let mut samples = history.accepted_knots().rev();
    let (previous_time, _, previous) = samples
        .next()
        .ok_or("phase interpolation has no accepted anchor")?;
    let dt = time - previous_time;
    if !dt.is_finite() || dt <= 0.0 {
        return Err("phase interpolation candidate must follow its accepted anchor".into());
    }
    let s = ScaledValue::new;
    let tolerance = s(abstol).plus(s(reltol).multiply(s(current.abs().max(previous.abs()))));
    // A slope before a declared jump or corner cannot predict the next interval.
    let older_sample = if history
        .accepted_left_limits()
        .next_back()
        .is_some_and(|(time, _)| time == previous_time)
    {
        None
    } else {
        samples.next()
    };
    let (error, order) = if let Some((older_time, _, older)) = older_sample {
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
    } else if let Some(slope) = outgoing_slope {
        // An outgoing physical rate belongs to this anchor, even when the
        // incoming derivative jumped. For f=f0+s*h+a*h*h, the linear
        // interpolant's midpoint error is (f-f0-s*h)/4. This is the same
        // quadratic error model as the smooth three-knot estimate above.
        // Never substitute an incoming secant across a corner for this rate.
        let defect = ScaledValue::sum_triple_products_ratio(
            [
                [s(current), s(1.0), s(1.0)],
                [s(-previous), s(1.0), s(1.0)],
                [s(-slope), s(dt), s(1.0)],
            ]
            .into_iter(),
            [[s(4.0), s(1.0), s(1.0)]].into_iter(),
        )
        .map_err(|error| format!("phase outgoing-slope interpolation: {error:?}"))?;
        (defect, 2)
    } else {
        // OP/UIC gives a constant prehistory, not a measured input slope. The
        // first interval is controlled by its change from that anchor. Once
        // two physical samples exist, use their measured divided difference.
        (s(current).plus(s(-previous)), 1)
    };
    let current_error = error.divide(tolerance).binary64().abs();
    if current_error.is_nan() {
        return Err("phase interpolation error is indeterminate".into());
    }
    if order == 1 {
        // A change from a constant prehistory is a large-signal step, not a
        // chord error a junction-voltage tolerance can stand for.
        return Ok(PhaseInterpolationControl {
            device_index,
            normalized_error: current_error,
            next_step: dt * (0.9 / current_error).clamp(0.1, 2.0),
        });
    }
    // Divide by each budget separately: the larger budget is the smaller
    // ratio, and neither division can overflow the other's scale.
    let full_error = if input_referred > 0.0 {
        current_error.min(error.divide(s(input_referred)).binary64().abs())
    } else {
        current_error
    };
    // The read window's share of the whole-interval error, and the step that
    // brings the read-window error to the same 0.81 the unwindowed sqrt rule
    // aims at: `x^2 * E = 0.81` while `x <= 2r`, else `4 r (x - r) E = 0.81`.
    let window = delay.map(|delay| delay / dt).filter(|ratio| *ratio < 0.5);
    let normalized_error =
        window.map_or(full_error, |ratio| full_error * 4.0 * ratio * (1.0 - ratio));
    let unwindowed = 0.9 / full_error.sqrt();
    let scale = match window {
        Some(ratio) if unwindowed > 2.0 * ratio => ratio + 0.81 / (4.0 * full_error * ratio),
        _ => unwindowed,
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
