//! Tian's return ratio using both port-voltage and port-current complements.
//!
//! Recover `1-v1` from the opposite voltage-probe terminal and `1-i2` from
//! the injection node's other KCL terms. Subtracting rounded responses from
//! unity discards the return difference of a high-gain feedback loop.
use super::*;
use rspice_veriloga_runtime::arithmetic::{ArithmeticError, ScaledValue};

type WideComplex = [ScaledValue; 2];

fn arithmetic(error: ArithmeticError) -> SimulationError {
    SimulationError::Circuit(format!("STB return-ratio arithmetic: {error:?}"))
}

fn sum(
    terms: impl Iterator<Item = [ScaledValue; 3]> + Clone,
) -> Result<ScaledValue, SimulationError> {
    let one = ScaledValue::new(1.0);
    ScaledValue::sum_triple_products_ratio(terms, [[one; 3]].into_iter()).map_err(arithmetic)
}

/// `p + q - 2*p*q + sign*2*b*c`, retaining products, cancellation and exponents.
fn combination(
    p: WideComplex,
    q: WideComplex,
    b: WideComplex,
    c: WideComplex,
    sign: f64,
) -> Result<WideComplex, SimulationError> {
    let one = ScaledValue::new(1.0);
    let two = ScaledValue::new(2.0);
    let signed_two = ScaledValue::new(2.0 * sign);
    Ok([
        sum([
            [p[0], one, one],
            [q[0], one, one],
            [two.negated(), p[0], q[0]],
            [two, p[1], q[1]],
            [signed_two, b[0], c[0]],
            [signed_two.negated(), b[1], c[1]],
        ]
        .into_iter())?,
        sum([
            [p[1], one, one],
            [q[1], one, one],
            [two.negated(), p[0], q[1]],
            [two.negated(), p[1], q[0]],
            [signed_two, b[0], c[1]],
            [signed_two, b[1], c[0]],
        ]
        .into_iter())?,
    ])
}

fn wide(value: Complex64) -> WideComplex {
    [ScaledValue::new(value.re), ScaledValue::new(value.im)]
}

fn magnitude(value: WideComplex) -> f64 {
    value[0].binary64().hypot(value[1].binary64())
}

pub(super) fn extract(
    matrix: &rspice_matrix::ComplexMatrix,
    voltage_solution: &[Complex64],
    current_solution: &[Complex64],
    positive: usize,
    negative: usize,
    branch: usize,
    abort: &dyn AbortSignal,
) -> Result<Complex64, SimulationError> {
    let a = wide(voltage_solution[positive]);
    let x = wide(-voltage_solution[negative]); // 1 - a, from the voltage constraint
    let b = wide(voltage_solution[branch]);
    let c = wide(current_solution[positive]);
    let d = wide(current_solution[branch]);
    let one = ScaledValue::new(1.0);
    // The unit current injection is at the positive probe terminal: KCL says
    // 1-i2 equals every other row term, including dependent branch currents.
    // Subtract only the probe's unit coefficient: a CCCS may also stamp the
    // same branch column, so excluding the entire column would drop physics.
    let cancelled = std::cell::Cell::new(false);
    let row = matrix
        .row_entries(positive)
        .map_err(SimulationError::Solver)?
        .take_while(|_| {
            if cancelled.get() {
                return false;
            }
            if abort.is_aborted() {
                cancelled.set(true);
                false
            } else {
                true
            }
        });
    let e = [
        sum(row.clone().flat_map(|(column, coefficient)| {
            let v = wide(current_solution[column]);
            let g = wide(coefficient);
            let probe = ScaledValue::new(if column == branch { -1.0 } else { 0.0 });
            [
                [g[0], v[0], one],
                [g[1].negated(), v[1], one],
                [probe, v[0], one],
            ]
        }))?,
        sum(row.flat_map(|(column, coefficient)| {
            let v = wide(current_solution[column]);
            let g = wide(coefficient);
            let probe = ScaledValue::new(if column == branch { -1.0 } else { 0.0 });
            [[g[0], v[1], one], [g[1], v[0], one], [probe, v[1], one]]
        }))?,
    ];
    if cancelled.get() || abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    // The two forms are algebraically identical using a+x=d+e=1.
    // Select the smaller pair to avoid cancellation against rounded unity.
    let numerator = if magnitude(a) + magnitude(d) <= magnitude(x) + magnitude(e) {
        combination(a, d, b, c, 1.0)?
    } else {
        combination(x, e, b, c, 1.0)?
    };
    let denominator = if magnitude(a) + magnitude(e) <= magnitude(x) + magnitude(d) {
        combination(a, e, b, c, -1.0)?
    } else {
        combination(x, d, b, c, -1.0)?
    };
    let norm = [
        [denominator[0], denominator[0], one],
        [denominator[1], denominator[1], one],
    ];
    let real = ScaledValue::sum_triple_products_ratio(
        [
            [numerator[0], denominator[0], one],
            [numerator[1], denominator[1], one],
        ]
        .into_iter(),
        norm.into_iter(),
    )
    .map_err(arithmetic)?;
    let imag = ScaledValue::sum_triple_products_ratio(
        [
            [numerator[1], denominator[0], one],
            [numerator[0].negated(), denominator[1], one],
        ]
        .into_iter(),
        norm.into_iter(),
    )
    .map_err(arithmetic)?;
    let narrow = |value: ScaledValue| {
        let result = value.binary64();
        if !result.is_finite() || (result == 0.0 && !value.is_zero()) {
            Err(SimulationError::Circuit(
                "STB loop gain exceeds finite representable precision".into(),
            ))
        } else {
            Ok(result)
        }
    };
    Ok(Complex64::new(narrow(real)?, narrow(imag)?))
}
