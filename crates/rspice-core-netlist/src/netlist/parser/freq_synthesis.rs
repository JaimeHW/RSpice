//! PSpice `FREQ` controlled sources: a measured frequency response table.
//!
//! `E1 out 0 FREQ {V(in)} [DB|MAG] [DEG|RAD] [R_I] = (f, a, b)... [DELAY=T]`
//! answers small-signal analyses with the table itself, read between entries
//! as [`FrequencyResponse`] reads it. The operating point takes the lowest
//! entry as the DC gain. The transient runs a stable real rational fit of the
//! table through the shared LAPLACE state-space synthesis; the fit matches
//! the DC gain exactly and every entry within [`FIT_TOLERANCE`] of the peak
//! gain, and a table no stable rational of order up to [`MAX_ORDER`] follows
//! that closely is refused with the best error reached. `DELAY=T` says the table's phase includes a transport delay `T`:
//! the transient fits the table with that delay taken out and adds it back as
//! a pure delay, which leaves the table, and so every small-signal answer, as
//! written.

use super::{DelayTime, Element, ParseError, delay_line, synthesize_rational_transfer};
use crate::netlist::{
    ElementKind, ElementProvenance, FrequencyResponse, SourceMultiplicity, SynthesizedTransferForm,
};
use num_complex::Complex64;

/// Largest fit error accepted, relative to the table's peak magnitude.
const FIT_TOLERANCE: f64 = 5e-3;
/// Highest denominator order tried.
const MAX_ORDER: usize = 12;

/// How a table states its gain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FreqTableUnits {
    /// Magnitude in dB (`DB`, the default) or as a ratio (`MAG`), with phase
    /// in degrees (`DEG`, the default) or radians (`RAD`).
    Polar { decibels: bool, degrees: bool },
    /// Real and imaginary parts (`R_I`).
    Rectangular,
}

/// Minimum-norm least squares by Householder QR; `rows` are `[a.., rhs]`.
fn least_squares(mut rows: Vec<Vec<f64>>, unknowns: usize) -> Option<Vec<f64>> {
    let m = rows.len();
    for column in 0..unknowns {
        let norm = (column..m)
            .map(|r| rows[r][column].powi(2))
            .sum::<f64>()
            .sqrt();
        if norm == 0.0 {
            return None;
        }
        let alpha = if rows[column][column] > 0.0 {
            -norm
        } else {
            norm
        };
        let mut v: Vec<f64> = (column..m).map(|r| rows[r][column]).collect();
        v[0] -= alpha;
        let vv = v.iter().map(|x| x * x).sum::<f64>();
        if vv == 0.0 {
            continue;
        }
        let reflected = &mut rows[column..];
        for c in column..=unknowns {
            let dot = reflected
                .iter()
                .zip(&v)
                .map(|(row, weight)| weight * row[c])
                .sum::<f64>();
            let scale = 2.0 * dot / vv;
            for (row, weight) in reflected.iter_mut().zip(&v) {
                row[c] -= scale * weight;
            }
        }
    }
    let mut x = vec![0.0; unknowns];
    for row in (0..unknowns).rev() {
        let tail = (row + 1..unknowns)
            .map(|c| rows[row][c] * x[c])
            .sum::<f64>();
        let pivot = rows[row][row];
        if pivot.abs() < 1e-300 {
            return None;
        }
        x[row] = (rows[row][unknowns] - tail) / pivot;
    }
    x.iter().all(|value| value.is_finite()).then_some(x)
}

fn evaluate(coefficients: &[f64], s: Complex64) -> Complex64 {
    coefficients
        .iter()
        .rev()
        .fold(Complex64::new(0.0, 0.0), |acc, &c| acc * s + c)
}

/// Whether every root of the polynomial lies in the open left half-plane,
/// by Durand-Kerner iteration on its monic form.
fn hurwitz(coefficients: &[f64]) -> bool {
    let degree = coefficients.len() - 1;
    let lead = coefficients[degree];
    let monic: Vec<f64> = coefficients.iter().map(|c| c / lead).collect();
    let radius = 1.0 + monic[..degree].iter().fold(0.0f64, |m, c| m.max(c.abs()));
    let mut roots: Vec<Complex64> = (0..degree)
        .map(|k| {
            Complex64::from_polar(
                radius,
                0.4 + k as f64 * std::f64::consts::TAU / degree as f64,
            )
        })
        .collect();
    for _ in 0..500 {
        let mut moved = 0.0f64;
        for i in 0..degree {
            let denominator = (0..degree)
                .filter(|&j| j != i)
                .fold(Complex64::new(1.0, 0.0), |acc, j| {
                    acc * (roots[i] - roots[j])
                });
            let step = evaluate(&monic, roots[i]) / denominator;
            roots[i] -= step;
            moved = moved.max(step.norm() / roots[i].norm().max(1.0));
        }
        if moved < 1e-14 {
            break;
        }
    }
    roots.iter().all(|root| root.re < 0.0 && root.is_finite())
}

/// Sanathanan-Koerner fit of `order` on normalized `samples`, with the DC
/// gain pinned. Returns the numerator and denominator in normalized `s`.
fn fit(samples: &[(f64, Complex64)], dc: f64, order: usize) -> Option<(Vec<f64>, Vec<f64>)> {
    let unknowns = 2 * order;
    let mut weights = vec![1.0; samples.len()];
    let mut solution = None;
    for _ in 0..16 {
        let mut rows = Vec::with_capacity(2 * samples.len());
        for (&(omega, gain), &weight) in samples.iter().zip(&weights) {
            let s = Complex64::new(0.0, omega);
            let mut real = Vec::with_capacity(unknowns + 1);
            let mut imaginary = Vec::with_capacity(unknowns + 1);
            let mut power = s;
            let mut numerator = Vec::with_capacity(order);
            let mut denominator = Vec::with_capacity(order);
            for _ in 0..order {
                numerator.push(power * weight);
                denominator.push(-gain * power * weight);
                power *= s;
            }
            for term in numerator.iter().chain(&denominator) {
                real.push(term.re);
                imaginary.push(term.im);
            }
            let rhs = (gain - dc) * weight;
            real.push(rhs.re);
            imaginary.push(rhs.im);
            rows.push(real);
            rows.push(imaginary);
        }
        let x = least_squares(rows, unknowns)?;
        let numerator: Vec<f64> = std::iter::once(dc)
            .chain(x[..order].iter().copied())
            .collect();
        let denominator: Vec<f64> = std::iter::once(1.0)
            .chain(x[order..].iter().copied())
            .collect();
        for (weight, &(omega, _)) in weights.iter_mut().zip(samples) {
            let magnitude = evaluate(&denominator, Complex64::new(0.0, omega)).norm();
            *weight = 1.0 / magnitude.max(1e-300);
        }
        solution = Some((numerator, denominator));
    }
    solution
}

/// Realize a `FREQ` table as elements. `table` holds `(f, a, b)` triples.
#[allow(clippy::too_many_arguments)]
pub(super) fn synthesize_freq(
    name: &str,
    node_pos: &str,
    node_neg: &str,
    input_expr: &str,
    table: &[f64],
    units: FreqTableUnits,
    delay: f64,
    voltage_output: bool,
    line: usize,
) -> Result<Vec<Element>, ParseError> {
    let fail = |message: String| ParseError::Syntax {
        line,
        message: format!("FREQ source '{name}': {message}"),
    };
    if table.is_empty() || !table.len().is_multiple_of(3) {
        return Err(fail(
            "the table needs (frequency, gain, phase) triples".to_string(),
        ));
    }
    if !(delay.is_finite() && delay >= 0.0) {
        return Err(fail(format!("DELAY={delay} is not a non-negative time")));
    }
    let mut entries: Vec<(f64, Complex64)> = Vec::with_capacity(table.len() / 3);
    for triple in table.chunks_exact(3) {
        let (frequency, a, b) = (triple[0], triple[1], triple[2]);
        let gain = match units {
            FreqTableUnits::Rectangular => Complex64::new(a, b),
            FreqTableUnits::Polar { decibels, degrees } => Complex64::from_polar(
                if decibels { 10f64.powf(a / 20.0) } else { a },
                if degrees { b.to_radians() } else { b },
            ),
        };
        if !(frequency.is_finite() && frequency >= 0.0 && gain.is_finite()) {
            return Err(fail(format!(
                "entry ({frequency}, {a}, {b}) is not a finite gain"
            )));
        }
        if entries.last().is_some_and(|&(last, _)| frequency <= last) {
            return Err(fail("frequencies must strictly increase".to_string()));
        }
        entries.push((frequency, gain));
    }
    let lowest = entries[0].1;
    let dc = if lowest.re < 0.0 {
        -lowest.norm()
    } else {
        lowest.norm()
    };

    // The fit runs on the entries themselves, in s normalized to the top
    // entry, with the stated delay taken out. Between entries the table is an
    // interpolation, not a measurement, so it is not fitted there.
    let top = entries[entries.len() - 1].0;
    let scale = std::f64::consts::TAU * top.max(1e-300);
    let samples: Vec<(f64, Complex64)> = entries
        .iter()
        .filter(|&&(frequency, _)| frequency > 0.0)
        .map(|&(frequency, gain)| {
            let omega = std::f64::consts::TAU * frequency;
            (
                omega / scale,
                gain * Complex64::from_polar(1.0, omega * delay),
            )
        })
        .collect();
    let peak = samples
        .iter()
        .map(|(_, gain)| gain.norm())
        .fold(dc.abs(), f64::max)
        .max(1e-300);
    let error_of = |numerator: &[f64], denominator: &[f64]| {
        samples
            .iter()
            .map(|&(omega, gain)| {
                let s = Complex64::new(0.0, omega);
                (evaluate(numerator, s) / evaluate(denominator, s) - gain).norm()
            })
            .fold(0.0f64, f64::max)
            / peak
    };

    let mut best = error_of(&[dc], &[1.0]);
    let mut realization = (best <= FIT_TOLERANCE).then(|| (vec![dc], vec![1.0], 0, best));
    for order in 1..=MAX_ORDER.min(samples.len()) {
        if realization.is_some() {
            break;
        }
        let Some((numerator, denominator)) = fit(&samples, dc, order) else {
            continue;
        };
        if !hurwitz(&denominator) {
            continue;
        }
        let error = error_of(&numerator, &denominator);
        best = best.min(error);
        if error <= FIT_TOLERANCE {
            realization = Some((numerator, denominator, order, error));
        }
    }
    let Some((numerator, denominator, order, error)) = realization else {
        return Err(fail(format!(
            "no stable rational transfer of order up to {MAX_ORDER} follows the table's entries \
             within {:.2}% of its peak gain (best {:.2}%), so it has no transient realization; \
             add entries, smooth the response, or state its transport delay with DELAY=",
            100.0 * FIT_TOLERANCE,
            100.0 * best
        )));
    };
    log::info!(
        "line {line}: FREQ source '{name}': the transient realizes an order-{order} rational \
         fit within {:.3}% of the peak gain",
        100.0 * error
    );
    let unscale = |coefficients: Vec<f64>| -> Vec<f64> {
        coefficients
            .into_iter()
            .enumerate()
            .map(|(power, c)| c / scale.powi(power as i32))
            .collect()
    };

    let form = SynthesizedTransferForm::Frequency;
    let input_node = format!("{name}.__FIN");
    let mut elements = vec![Element {
        name: format!("{name}.__BFIN"),
        kind: ElementKind::BehavioralVoltage {
            expression: input_expr.to_string(),
            tc1: 0.0,
            tc2: 0.0,
            multiplicity: SourceMultiplicity::default(),
        },
        nodes: vec![input_node.clone(), "0".to_string()],
        provenance: ElementProvenance::GeneratedFrequencyResponseInput {
            owner: name.to_string(),
            node: input_node.clone(),
            response: FrequencyResponse::new(entries),
        },
    }];
    let mut fitted_input = format!("V({input_node})");
    if delay > 0.0 {
        fitted_input = delay_line(
            name,
            1,
            form,
            &fitted_input,
            DelayTime::Fixed(delay),
            &mut elements,
        );
    }
    elements.extend(synthesize_rational_transfer(
        name,
        node_pos,
        node_neg,
        &fitted_input,
        unscale(numerator),
        unscale(denominator),
        voltage_output,
        form,
        line,
    )?);
    Ok(elements)
}
