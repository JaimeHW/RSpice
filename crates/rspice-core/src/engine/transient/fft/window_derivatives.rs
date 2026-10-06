//! Analytic window derivatives with binary exponents retained until output.
//! Derivatives are with respect to u=(t-START)/(STOP-START).
use super::*;
use rspice_veriloga_runtime::arithmetic::ScaledValue as S;

type Result<T> = std::result::Result<T, SimulationError>;

fn invalid(detail: &str) -> SimulationError {
    SimulationError::Circuit(format!(".FFT current impulse derivative: {detail}"))
}
fn poll(abort: &dyn AbortSignal, index: usize) -> Result<()> {
    if index.is_multiple_of(64) && abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(())
    }
}
fn zeros<T: Clone>(count: usize, zero: T) -> Result<Vec<T>> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(count)
        .map_err(|_| allocation_error("window derivative workspace"))?;
    values.resize(count, zero);
    Ok(values)
}
fn sum(terms: impl Iterator<Item = [S; 3]> + Clone, abort: &dyn AbortSignal) -> Result<S> {
    let one = S::new(1.0);
    let result = S::sum_triple_products_ratio(
        terms.enumerate().map(|(index, term)| {
            if index.is_multiple_of(64) && abort.is_aborted() {
                [S::new(Value::NAN), one, one]
            } else {
                term
            }
        }),
        [[one; 3]].into_iter(),
    );
    poll(abort, 0)?;
    result.map_err(|_| invalid("window derivative accumulation exceeds its numerical precision"))
}

fn product(left: &[S], right: &[S], abort: &dyn AbortSignal) -> Result<Vec<S>> {
    let mut output = zeros(left.len(), S::new(0.0))?;
    let mut terms = zeros(left.len(), [S::new(0.0); 3])?;
    for (order, value) in output.iter_mut().enumerate() {
        let mut choose = S::new(1.0);
        for j in 0..=order {
            poll(abort, j)?;
            terms[j] = [choose, left[j], right[order - j]];
            choose = choose
                .multiply(S::new((order - j) as Value))
                .divide(S::new((j + 1) as Value));
        }
        *value = sum(terms[..=order].iter().copied(), abort)?;
    }
    Ok(output)
}

fn polynomial_derivatives(
    poly: &window::Polynomial,
    x: Value,
    scale: Value,
    order: usize,
    abort: &dyn AbortSignal,
) -> Result<Vec<S>> {
    let mut result = zeros(order + 1, S::new(0.0))?;
    result[0] = S::new(poly.sine[0] + poly.triangle * 2.0 * x.min(1.0 - x));
    if order > 0 {
        result[1] = S::new(poly.triangle * if x < 0.5 { 2.0 * scale } else { -2.0 * scale });
    }
    let degree = poly
        .sine
        .iter()
        .rposition(|coefficient| *coefficient != 0.0)
        .unwrap_or(0);
    if degree == 0 {
        return Ok(result);
    }
    let (sine, cosine) = window::sine_cosine(x);
    let frequency = S::new(PI * scale);
    let mut frequency_power = S::new(1.0);
    let mut base = zeros(order + 1, S::new(0.0))?;
    for (j, value) in base.iter_mut().enumerate() {
        poll(abort, j)?;
        *value = frequency_power.multiply(S::new(match j % 4 {
            0 => sine,
            1 => cosine,
            2 => -sine,
            _ => -cosine,
        }));
        frequency_power = frequency_power.multiply(frequency);
    }
    let mut power = zeros(order + 1, S::new(0.0))?;
    power[0] = S::new(1.0);
    for degree in 1..=degree {
        power = product(&power, &base, abort)?;
        if poly.sine[degree] != 0.0 {
            for (j, value) in result.iter_mut().enumerate() {
                poll(abort, j)?;
                *value = S::product_sum(*value, S::new(1.0), power[j], S::new(poly.sine[degree]));
            }
        }
    }
    Ok(result)
}

fn gaussian_derivatives(
    x: Value,
    scale: Value,
    alpha: Value,
    order: usize,
    abort: &dyn AbortSignal,
) -> Result<Vec<S>> {
    let mut result = zeros(order + 1, S::new(0.0))?;
    result[0] = S::new(window::coefficient(FftWindow::Gaussian, x, alpha));
    let linear = S::new(-2.0 * alpha * alpha * (2.0 * x - 1.0) * scale);
    let quadratic_twice = S::new(-4.0 * alpha * alpha * scale * scale);
    if order > 0 {
        result[1] = result[0].multiply(linear);
    }
    for n in 2..=order {
        poll(abort, n)?;
        result[n] = S::product_sum(
            linear,
            result[n - 1],
            quadratic_twice.multiply(S::new((n - 1) as Value)),
            result[n - 2],
        );
    }
    Ok(result)
}

/// d^m/dz^m I0(2*sqrt(z)) = sum z^k/(k!*(k+m)!).
/// After factoring 1/m!, every term is positive and the ratio is at most
/// 100/k^2 (ALFA <= 20). Termination therefore has a geometric tail bound.
fn kaiser_derivatives(
    x: Value,
    scale: Value,
    alpha: Value,
    order: usize,
    abort: &dyn AbortSignal,
) -> Result<Vec<S>> {
    let z = alpha * alpha * x * (1.0 - x);
    let first = S::new(alpha * alpha * (1.0 - 2.0 * x) * scale);
    let second = S::new(-alpha * alpha * scale * scale);
    let mut factorial = zeros(order + 1, S::new(1.0))?;
    let mut hyper = zeros(order + 1, S::new(0.0))?;
    for m in 0..=order {
        poll(abort, m)?;
        if m > 0 {
            factorial[m] = factorial[m - 1].multiply(S::new(m as Value));
        }
        let mut term = 1.0;
        let mut total = 1.0;
        let mut correction = 0.0;
        let mut converged = z == 0.0;
        for k in 1..=128 {
            poll(abort, k)?;
            let ratio = z / (k as Value * (k as Value + m as Value));
            term *= ratio;
            crate::numerics::compensated_add(&mut total, &mut correction, term);
            if ratio < 1.0 && term * ratio / (1.0 - ratio) <= 0.0625 * Value::EPSILON * total {
                converged = true;
                break;
            }
        }
        if !converged {
            return Err(invalid("Kaiser derivative series did not converge"));
        }
        hyper[m] = S::new(total + correction).divide(factorial[m]);
    }
    let mut output = zeros(order + 1, S::new(0.0))?;
    let mut terms = zeros(order / 2 + 1, [S::new(0.0); 3])?;
    for (n, value) in output.iter_mut().enumerate() {
        for (j, term) in terms.iter_mut().enumerate().take(n / 2 + 1) {
            poll(abort, j)?;
            let coefficient = factorial[n]
                .divide(factorial[n - 2 * j])
                .divide(factorial[j]);
            *term = [
                coefficient,
                hyper[n - j],
                first
                    .powu((n - 2 * j) as u32)
                    .multiply(second.powu(j as u32)),
            ];
        }
        *value =
            sum(terms[..=n / 2].iter().copied(), abort)?.divide(S::new(modified_bessel_i0(alpha)));
    }
    Ok(output)
}

fn geometry(
    analysis: &FftAnalysis,
    mode: XyceFftMode,
    stop: Value,
    time: Value,
    order: u32,
) -> Result<Option<(Value, Value)>> {
    let start = analysis.start.unwrap_or(0.0);
    let duration = stop - start;
    let periodic = mode.uses_periodic_windows();
    let last = sample_time(analysis, stop, analysis.points - 1);
    if !periodic && time > last {
        return Ok(None);
    }
    let scale = if periodic {
        1.0
    } else {
        analysis.points as Value / (analysis.points - 1) as Value
    };
    let at_boundary = if periodic { time == stop } else { time == last };
    let mut x = if periodic && time == stop {
        0.0
    } else if !periodic && time == last {
        1.0
    } else {
        ((time - start) / duration * scale).clamp(0.0, 1.0)
    };
    if !at_boundary && (x == 0.0 || x == 1.0) {
        return Err(invalid(
            "normalized event position rounds onto a window boundary",
        ));
    }
    if let Some(poly) = window::polynomial(analysis.window) {
        let center = start + duration * (0.5 / scale);
        if poly.triangle != 0.0 && time == center {
            return Err(invalid(
                "window is not differentiable at its triangular corner",
            ));
        }
        if poly.triangle != 0.0 && x == 0.5 {
            x = if time < center {
                0.5_f64.next_down()
            } else {
                0.5_f64.next_up()
            };
        }
        if at_boundary {
            let first_nonsmooth = if periodic {
                if poly.triangle != 0.0 {
                    Some(1)
                } else {
                    poly.sine
                        .iter()
                        .enumerate()
                        .find(|(n, c)| n % 2 == 1 && **c != 0.0)
                        .map(|(n, _)| n)
                }
            } else if poly.triangle != 0.0 {
                Some(1)
            } else {
                poly.sine.iter().position(|c| *c != 0.0)
            };
            if first_nonsmooth.is_some_and(|n| order as usize >= n) {
                return Err(invalid(
                    "window lacks the required derivative at its support boundary",
                ));
            }
            if !periodic {
                return Ok(None);
            } // Every admitted derivative is zero.
        }
    } else if at_boundary {
        return Err(invalid(
            "window lacks the required derivative at its support boundary",
        ));
    }
    Ok(Some((x, scale)))
}

fn phase(turns: Value) -> (Value, Value) {
    match turns.fract() {
        0.0 => (1.0, 0.0),
        0.25 => (0.0, -1.0),
        0.5 => (-1.0, 0.0),
        0.75 => (0.0, 1.0),
        fraction => {
            let (s, c) = (-2.0 * PI * fraction).sin_cos();
            (c, s)
        }
    }
}
fn materialize(value: S) -> Result<Value> {
    let result = value.binary64();
    if !result.is_finite() || (result == 0.0 && !value.is_zero()) {
        Err(invalid("windowed coefficient is not representable"))
    } else {
        Ok(result)
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn add_to_bins(
    bins: &mut [Complex<Value>],
    terms: &[crate::analysis::measure_signals::current_observation::CurrentImpulseContribution<
        '_,
    >],
    analysis: &FftAnalysis,
    mode: XyceFftMode,
    transient_stop: Value,
    coherent_gain: Value,
    max_values: usize,
    abort: &dyn AbortSignal,
) -> Result<()> {
    if analysis.window == FftWindow::Rectangular {
        return Ok(());
    }
    let start = analysis.start.unwrap_or(0.0);
    let stop = analysis.stop.unwrap_or(transient_stop);
    let duration = stop - start;
    let mut corrections: Vec<Complex<Value>> = Vec::new();
    for term in terms {
        for (index, point) in term.trace.derivatives.iter().enumerate() {
            poll(abort, index)?;
            if point.time <= start || point.time > stop {
                continue;
            }
            let Some((x, scale)) = geometry(analysis, mode, stop, point.time, point.order)? else {
                continue;
            };
            let count = (point.order as usize)
                .checked_add(1)
                .ok_or(ResourceLimitError {
                    resource: ResourceKind::ResultValues,
                    requested: usize::MAX,
                    limit: max_values,
                })?;
            let requested = count
                .checked_mul(16)
                .and_then(|values| values.checked_add(bins.len().saturating_mul(4)))
                .and_then(|values| values.checked_add(32 * 1024))
                .filter(|values| *values <= isize::MAX as usize / 8)
                .ok_or(ResourceLimitError {
                    resource: ResourceKind::ResultValues,
                    requested: usize::MAX,
                    limit: max_values,
                })?;
            ResourceLimitError::ensure(ResourceKind::ResultValues, requested, max_values)?;
            if corrections.is_empty() {
                corrections = zeros(bins.len(), Complex::new(0.0, 0.0))?;
            }
            let order = point.order as usize;
            let derivatives = if let Some(poly) = window::polynomial(analysis.window) {
                polynomial_derivatives(&poly, x, scale, order, abort)?
            } else if analysis.window == FftWindow::Gaussian {
                gaussian_derivatives(x, scale, analysis.alpha, order, abort)?
            } else {
                kaiser_derivatives(x, scale, analysis.alpha, order, abort)?
            };
            let mut powers = zeros(count, S::new(1.0))?;
            let mut summands = zeros(count, [S::new(0.0); 3])?;
            let normalizer = S::new(point.coefficient)
                .multiply(S::new(term.weight))
                .divide(S::new(duration).powu(point.order))
                .divide(S::new(duration))
                .divide(S::new(coherent_gain));
            for (bin, coefficient) in bins.iter_mut().enumerate() {
                poll(abort, bin)?;
                let omega = S::new(2.0 * PI * bin as Value);
                for j in 1..count {
                    poll(abort, j)?;
                    powers[j] = powers[j - 1].multiply(omega);
                }
                let mut choose = S::new(1.0);
                for (j, summand) in summands.iter_mut().enumerate() {
                    poll(abort, j)?;
                    let negative = (j % 2 == 1) ^ ((order - j) % 4 >= 2);
                    *summand = [
                        if negative { choose.negated() } else { choose },
                        derivatives[j],
                        powers[order - j],
                    ];
                    choose = choose
                        .multiply(S::new((order - j) as Value))
                        .divide(S::new((j + 1) as Value));
                }
                let component = |parity: usize| -> Result<S> {
                    sum(
                        summands
                            .iter()
                            .copied()
                            .enumerate()
                            .filter(|(j, _)| (order - j) % 2 == parity)
                            .map(|(_, term)| term),
                        abort,
                    )
                };
                let real = component(0)?;
                let imaginary = component(1)?;
                let (cosine, sine) = phase(bin as Value * ((point.time - start) / duration));
                let multiplier = if bin == 0 || bin == analysis.points / 2 {
                    normalizer
                } else {
                    normalizer.multiply(S::new(2.0))
                };
                let real_part = materialize(
                    S::product_sum(real, S::new(cosine), imaginary, S::new(-sine))
                        .multiply(multiplier),
                )?;
                let imaginary_part = materialize(
                    S::product_sum(real, S::new(sine), imaginary, S::new(cosine))
                        .multiply(multiplier),
                )?;
                crate::numerics::compensated_add(
                    &mut coefficient.re,
                    &mut corrections[bin].re,
                    real_part,
                );
                crate::numerics::compensated_add(
                    &mut coefficient.im,
                    &mut corrections[bin].im,
                    imaginary_part,
                );
            }
        }
    }
    for (coefficient, correction) in bins.iter_mut().zip(corrections) {
        *coefficient += correction;
        if !coefficient.re.is_finite() || !coefficient.im.is_finite() {
            return Err(invalid("summed coefficient is not finite"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filtered_window_derivative_sums_poll_cancellation() {
        // Real or imaginary derivative terms can all have odd original
        // indices. Poll the filtered sequence, not those original indices.
        let visited = std::cell::Cell::new(0usize);
        let terms = (0..4096)
            .filter(|index| index % 2 == 1)
            .inspect(|_| visited.set(visited.get() + 1))
            .map(|index| [S::new(index as Value), S::new(1.0), S::new(1.0)]);
        let abort = crate::abort_signal::CountingAbort::new(1);
        assert!(matches!(sum(terms, &abort), Err(SimulationError::Aborted)));
        assert!(visited.get() <= 65, "visited {} terms", visited.get());
    }
}
