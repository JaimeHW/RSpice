//! Group delay from nonuniform retained complex sweeps, with explicit gaps.
use super::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(
    tag = "kind",
    content = "seconds",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum QpxfGroupDelay {
    Finite(Value),
    /// Phase is undefined at zero or below the authored magnitude floor.
    BelowMagnitudeFloor,
    InsufficientSamples,
    /// A π phase step has no unique unwrap direction on this grid.
    AmbiguousPhaseStep,
    NumericalLimit,
}

/// Nonuniform three-point phase derivative where possible, one-sided at
/// segment ends and a secant for two samples. The sweep must resolve phase
/// rotations; this is a sampled derivative, not an analytic circuit derivative.
pub(super) fn from_samples(
    frequencies: &[Value],
    values: &[Complex64],
    floor: Value,
    abort: &dyn AbortSignal,
) -> Result<Vec<QpxfGroupDelay>, SimulationError> {
    check_abort(abort)?;
    let mut delays = vec![QpxfGroupDelay::BelowMagnitudeFloor; values.len()];
    let usable = |index: usize| values[index].re.hypot(values[index].im) > floor;
    let mut start = 0;
    while start < values.len() {
        if start % 256 == 0 {
            check_abort(abort)?;
        }
        if !usable(start) {
            start += 1;
            continue;
        }
        let mut end = start + 1;
        while end < values.len() && usable(end) {
            if end % 256 == 0 {
                check_abort(abort)?;
            }
            end += 1;
        }
        if end - start == 1 {
            delays[start] = QpxfGroupDelay::InsufficientSamples;
            start = end;
            continue;
        }
        let slopes: Vec<_> = (start..end - 1)
            .map(|i| {
                if i % 256 == 0 {
                    check_abort(abort)?;
                }
                let phase = (values[i + 1].arg() - values[i].arg() + std::f64::consts::PI)
                    .rem_euclid(std::f64::consts::TAU)
                    - std::f64::consts::PI;
                if phase.abs() >= std::f64::consts::PI * (1.0 - 64.0 * Value::EPSILON) {
                    Ok(None)
                } else {
                    Ok(Some(phase / (frequencies[i + 1] - frequencies[i])))
                }
            })
            .collect::<Result<_, SimulationError>>()?;
        for (i, delay) in delays.iter_mut().enumerate().take(end).skip(start) {
            if i % 256 == 0 {
                check_abort(abort)?;
            }
            let derivative = if end - start == 2 {
                slopes[0]
            } else if i == start {
                let h1 = frequencies[i + 1] - frequencies[i];
                let h2 = frequencies[i + 2] - frequencies[i + 1];
                slopes[0]
                    .zip(slopes[1])
                    .map(|(a, b)| (1.0 + h1 / (h1 + h2)) * a - h1 / (h1 + h2) * b)
            } else if i == end - 1 {
                let h1 = frequencies[i - 1] - frequencies[i - 2];
                let h2 = frequencies[i] - frequencies[i - 1];
                slopes[i - start - 2]
                    .zip(slopes[i - start - 1])
                    .map(|(a, b)| (1.0 + h2 / (h1 + h2)) * b - h2 / (h1 + h2) * a)
            } else {
                let h1 = frequencies[i] - frequencies[i - 1];
                let h2 = frequencies[i + 1] - frequencies[i];
                slopes[i - start - 1]
                    .zip(slopes[i - start])
                    .map(|(a, b)| h2 / (h1 + h2) * a + h1 / (h1 + h2) * b)
            };
            *delay = match derivative.map(|value| -value / std::f64::consts::TAU) {
                Some(seconds) if seconds.is_finite() => QpxfGroupDelay::Finite(seconds),
                Some(_) => QpxfGroupDelay::NumericalLimit,
                None => QpxfGroupDelay::AmbiguousPhaseStep,
            };
        }
        start = end;
    }
    Ok(delays)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qpxf_group_delay_preserves_phase_wraps_nonuniform_grid_and_undefined_samples() {
        let frequencies = [-50.0, -20.0, -3.0, 0.0, 7.0, 30.0, 45.0];
        let delay = 0.007;
        let values: Vec<_> = frequencies
            .iter()
            .map(|f| Complex64::from_polar(0.4, 2.9 - std::f64::consts::TAU * delay * f))
            .collect();
        let result = from_samples(&frequencies, &values, 0.0, &NoAbort).unwrap();
        assert!(result.iter().all(
            |value| matches!(value, QpxfGroupDelay::Finite(actual) if (*actual-delay).abs() < 1e-15)
        ));
        // A quadratic phase gives an exact three-point derivative even on a
        // nonuniform grid, including each one-sided endpoint stencil.
        let values: Vec<_> = frequencies
            .iter()
            .map(|f| Complex64::from_polar(1.0, 0.0001 * f * f - 0.01 * f))
            .collect();
        for (f, result) in frequencies
            .iter()
            .zip(from_samples(&frequencies, &values, 0.0, &NoAbort).unwrap())
        {
            let QpxfGroupDelay::Finite(value) = result else {
                panic!("expected finite derivative")
            };
            assert!((value + (0.0002 * f - 0.01) / std::f64::consts::TAU).abs() < 1e-15);
        }
        let values = [
            Complex64::ONE,
            Complex64::ZERO,
            Complex64::new(1e-9, 0.0),
            Complex64::ONE,
            -Complex64::ONE,
        ];
        let result = from_samples(&[0.0, 1.0, 2.0, 3.0, 4.0], &values, 1e-8, &NoAbort).unwrap();
        assert_eq!(
            result,
            [
                QpxfGroupDelay::InsufficientSamples,
                QpxfGroupDelay::BelowMagnitudeFloor,
                QpxfGroupDelay::BelowMagnitudeFloor,
                QpxfGroupDelay::AmbiguousPhaseStep,
                QpxfGroupDelay::AmbiguousPhaseStep
            ]
        );
    }
}
