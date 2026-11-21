//! Two-port functions of a network's S-parameters, as ViVA's RF calculator
//! and ADS's network functions read them: the stability factors, the gains
//! at given terminations, the maximum gain, VSWR, and the impedance,
//! admittance and hybrid parameters.
//!
//! Each takes the coefficients as values (`kf(S11, S12, S21, S22)`), so it
//! reads any run's S-parameters, stored or live, and a curve plots against
//! the analysis' frequency. A reflection argument (`gs`, `gl`) is measured
//! against the port's own reference, so zero is the port termination.

use num_complex::Complex64;

use super::evaluator::EvaluationError;
use super::value::{CalcValue, ComplexValue, NumericValue, RealValue, finite};

/// The fewest and most arguments a two-port function takes.
fn arity(name: &str) -> Option<(usize, usize)> {
    Some(match name {
        "kf" | "b1f" | "mu" | "mu_prime" | "gmax" | "gmsg" => (4, 4),
        "gt" => (4, 6),
        "ga" | "gp" => (4, 5),
        "vswr" => (1, 1),
        "zp" | "yp" | "hp" => (6, 8),
        _ => return None,
    })
}

/// Whether `name` (lower case) is a two-port function.
pub(super) fn is_network_function(name: &str) -> bool {
    arity(name).is_some()
}

/// Runs the two-port function `name` (lower case).
pub(super) fn dispatch(name: &str, args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    let (fewest, most) =
        arity(name).ok_or_else(|| EvaluationError::UnknownFunction(name.into()))?;
    if args.len() < fewest || args.len() > most {
        return Err(EvaluationError::ArgCountMismatch {
            func: name.to_owned(),
            expected: if args.len() > most { most } else { fewest },
            actual: args.len(),
        });
    }
    match name {
        "zp" | "yp" | "hp" => parameter(name, args),
        "vswr" => real(args, |values| standing_wave_ratio(values[0])),
        _ => real(args, |values| {
            let network = TwoPort::of(values);
            let termination = |index: usize| values.get(index).copied().unwrap_or_default();
            match name {
                "kf" => network.rollett_k(),
                "b1f" => network.b1(),
                "mu" => network.mu_load(),
                "mu_prime" => network.mu_source(),
                "gmax" => network.maximum_gain(),
                "gmsg" => network.s21.norm() / network.s12.norm(),
                "gt" => network.transducer_gain(termination(4), termination(5)),
                "ga" => network.available_gain(termination(4)),
                _ => network.power_gain(termination(4)),
            }
        }),
    }
}

/// One frequency of a two-port's scattering matrix.
#[derive(Debug, Clone, Copy)]
struct TwoPort {
    s11: Complex64,
    s12: Complex64,
    s21: Complex64,
    s22: Complex64,
}

impl TwoPort {
    fn of(values: &[Complex64]) -> Self {
        Self {
            s11: values[0],
            s12: values[1],
            s21: values[2],
            s22: values[3],
        }
    }

    fn determinant(&self) -> Complex64 {
        self.s11 * self.s22 - self.s12 * self.s21
    }

    /// 1 − |S11|² − |S22|² + |Δ|².
    fn rollett_numerator(&self) -> f64 {
        1.0 - self.s11.norm_sqr() - self.s22.norm_sqr() + self.determinant().norm_sqr()
    }

    /// K = (1 − |S11|² − |S22|² + |Δ|²) / 2|S12·S21|.
    fn rollett_k(&self) -> f64 {
        self.rollett_numerator() / (2.0 * (self.s12 * self.s21).norm())
    }

    /// B1 = 1 + |S11|² − |S22|² − |Δ|², positive with K > 1 for
    /// unconditional stability.
    fn b1(&self) -> f64 {
        1.0 + self.s11.norm_sqr() - self.s22.norm_sqr() - self.determinant().norm_sqr()
    }

    /// μ = (1 − |S11|²) / (|S22 − Δ·S11*| + |S12·S21|).
    fn mu_load(&self) -> f64 {
        (1.0 - self.s11.norm_sqr())
            / ((self.s22 - self.determinant() * self.s11.conj()).norm()
                + (self.s12 * self.s21).norm())
    }

    /// μ′ = (1 − |S22|²) / (|S11 − Δ·S22*| + |S12·S21|).
    fn mu_source(&self) -> f64 {
        (1.0 - self.s22.norm_sqr())
            / ((self.s11 - self.determinant() * self.s22.conj()).norm()
                + (self.s12 * self.s21).norm())
    }

    /// MAG = |S21|/|S12|·(K − √(K² − 1)) when μ > 1, else MSG = |S21|/|S12|.
    /// Undefined (a hole) where neither exists.
    fn maximum_gain(&self) -> f64 {
        let forward = self.s21.norm_sqr();
        let reverse = (self.s12 * self.s21).norm();
        if self.mu_load() > 1.0 {
            let numerator = self.rollett_numerator();
            let discriminant = numerator * numerator - 4.0 * reverse * reverse;
            if numerator <= 0.0 || discriminant < 0.0 {
                return f64::NAN;
            }
            return 2.0 * forward / (numerator + discriminant.sqrt());
        }
        self.s21.norm() / self.s12.norm()
    }

    fn input_reflection(&self, load: Complex64) -> Complex64 {
        self.s11 + self.s12 * self.s21 * load / (1.0 - self.s22 * load)
    }

    fn output_reflection(&self, source: Complex64) -> Complex64 {
        self.s22 + self.s12 * self.s21 * source / (1.0 - self.s11 * source)
    }

    /// GT = (1 − |Γs|²)·|S21|²·(1 − |ΓL|²) / |(1 − S11·Γs)(1 − S22·ΓL) − S12·S21·Γs·ΓL|².
    fn transducer_gain(&self, source: Complex64, load: Complex64) -> f64 {
        let denominator = (1.0 - self.s11 * source) * (1.0 - self.s22 * load)
            - self.s12 * self.s21 * source * load;
        (1.0 - source.norm_sqr()) * self.s21.norm_sqr() * (1.0 - load.norm_sqr())
            / denominator.norm_sqr()
    }

    /// GA = (1 − |Γs|²)·|S21|² / (|1 − S11·Γs|²·(1 − |Γout|²)).
    fn available_gain(&self, source: Complex64) -> f64 {
        let output = self.output_reflection(source);
        self.s21.norm_sqr() * (1.0 - source.norm_sqr())
            / ((1.0 - self.s11 * source).norm_sqr() * (1.0 - output.norm_sqr()))
    }

    /// GP = |S21|²·(1 − |ΓL|²) / ((1 − |Γin|²)·|1 − S22·ΓL|²).
    fn power_gain(&self, load: Complex64) -> f64 {
        let input = self.input_reflection(load);
        self.s21.norm_sqr() * (1.0 - load.norm_sqr())
            / ((1.0 - input.norm_sqr()) * (1.0 - self.s22 * load).norm_sqr())
    }
}

/// VSWR = (1 + |Γ|) / (1 − |Γ|), a hole for a reflection of one or more.
fn standing_wave_ratio(reflection: Complex64) -> f64 {
    let magnitude = reflection.norm();
    if magnitude < 1.0 {
        (1.0 + magnitude) / (1.0 - magnitude)
    } else {
        f64::NAN
    }
}

/// `zp(i, j, S11, S12, S21, S22, r1, r2)` and its admittance and hybrid
/// kin: the (i, j) coefficient of the two-port on port references `r1` and
/// `r2` ohms (50 Ω when left off).
fn parameter(name: &str, args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    let mut args = args.into_iter();
    let mut index = |which: &str| -> Result<usize, EvaluationError> {
        match args.next().map(CalcValue::into_real) {
            Some(Ok(RealValue::Scalar(value))) if value == 1.0 || value == 2.0 => {
                Ok(value as usize - 1)
            }
            _ => Err(EvaluationError::TypeMismatch(format!(
                "{name}() names its {which} port as 1 or 2"
            ))),
        }
    };
    let (row, column) = (index("row")?, index("column")?);
    let mut rest = args.collect::<Vec<_>>();
    rest.resize(6, CalcValue::Real(RealValue::Scalar(50.0)));
    let form = name.to_owned();
    let values = sampled(rest, |values| {
        let network = TwoPort::of(values);
        let references = [values[4].re, values[5].re];
        let matrix = match form.as_str() {
            "zp" => impedance(&network, references),
            "yp" => admittance(&network, references),
            _ => hybrid(&network, references),
        };
        matrix.map_or(Complex64::new(f64::NAN, f64::NAN), |matrix| {
            let value = matrix[row][column];
            if finite(value) {
                value
            } else {
                Complex64::new(f64::NAN, f64::NAN)
            }
        })
    })?;
    Ok(CalcValue::Complex(values))
}

type Matrix = [[Complex64; 2]; 2];

/// Z = D(I + S)(I − S)⁻¹D with D = diag(√r1, √r2).
fn impedance(network: &TwoPort, references: [f64; 2]) -> Option<Matrix> {
    let TwoPort { s11, s12, s21, s22 } = *network;
    let determinant = (1.0 - s11) * (1.0 - s22) - s12 * s21;
    let normalized = [
        [(1.0 + s11) * (1.0 - s22) + s12 * s21, 2.0 * s12],
        [2.0 * s21, (1.0 - s11) * (1.0 + s22) + s12 * s21],
    ];
    scaled(normalized, determinant, references, |ri, rj| {
        (ri * rj).sqrt()
    })
}

/// Y = D⁻¹(I − S)(I + S)⁻¹D⁻¹.
fn admittance(network: &TwoPort, references: [f64; 2]) -> Option<Matrix> {
    let TwoPort { s11, s12, s21, s22 } = *network;
    let determinant = (1.0 + s11) * (1.0 + s22) - s12 * s21;
    let normalized = [
        [(1.0 - s11) * (1.0 + s22) + s12 * s21, -2.0 * s12],
        [-2.0 * s21, (1.0 + s11) * (1.0 - s22) + s12 * s21],
    ];
    scaled(normalized, determinant, references, |ri, rj| {
        1.0 / (ri * rj).sqrt()
    })
}

/// H: h11 = V1/I1 and h21 = I2/I1 with port 2 shorted, h12 = V1/V2 and
/// h22 = I2/V2 with port 1 open.
fn hybrid(network: &TwoPort, references: [f64; 2]) -> Option<Matrix> {
    let TwoPort { s11, s12, s21, s22 } = *network;
    let determinant = (1.0 - s11) * (1.0 + s22) + s12 * s21;
    let normalized = [
        [(1.0 + s11) * (1.0 + s22) - s12 * s21, 2.0 * s12],
        [-2.0 * s21, (1.0 - s11) * (1.0 - s22) - s12 * s21],
    ];
    let [r1, r2] = references;
    let ratio = (r1 / r2).sqrt();
    scaled(normalized, determinant, references, |_, _| 1.0).map(|h| {
        [
            [h[0][0] * r1, h[0][1] * ratio],
            [h[1][0] * ratio, h[1][1] / r2],
        ]
    })
}

/// `normalized / determinant`, each entry scaled by `scale(ri, rj)`. `None`
/// when the determinant vanishes or a reference is not a positive resistance.
fn scaled(
    normalized: Matrix,
    determinant: Complex64,
    references: [f64; 2],
    scale: impl Fn(f64, f64) -> f64,
) -> Option<Matrix> {
    if determinant.norm() < 1.0e-12
        || !references
            .iter()
            .all(|reference| reference.is_finite() && *reference > 0.0)
    {
        return None;
    }
    let mut matrix = normalized;
    for (row, entries) in matrix.iter_mut().enumerate() {
        for (column, entry) in entries.iter_mut().enumerate() {
            *entry = *entry / determinant * scale(references[row], references[column]);
        }
    }
    Some(matrix)
}

/// A real-valued function of the arguments, sample by sample; a value that
/// is not finite is a hole.
fn real(
    args: Vec<CalcValue>,
    function: impl Fn(&[Complex64]) -> f64,
) -> Result<CalcValue, EvaluationError> {
    let values = sampled(args, |values| {
        let value = function(values);
        if value.is_finite() { value } else { f64::NAN }
    })?;
    Ok(CalcValue::Real(values))
}

/// `function` of the arguments at each sample: scalars hold at every sample,
/// and every trace must lie on one frequency grid.
fn sampled<T>(
    args: Vec<CalcValue>,
    function: impl Fn(&[Complex64]) -> T,
) -> Result<NumericValue<T>, EvaluationError> {
    let args = args
        .into_iter()
        .map(CalcValue::into_complex)
        .collect::<Vec<_>>();
    let mut axis: Option<&Vec<f64>> = None;
    for arg in &args {
        if let ComplexValue::Waveform(x, y) = arg {
            if x.len() != y.len() || axis.is_some_and(|axis| axis != x) {
                return Err(EvaluationError::WaveformMismatch(
                    "the coefficients of a two-port must lie on one frequency grid".to_owned(),
                ));
            }
            axis = Some(x);
        }
    }
    let at = |sample: usize| -> Vec<Complex64> {
        args.iter()
            .map(|arg| match arg {
                ComplexValue::Scalar(value) => *value,
                ComplexValue::Waveform(_, y) => y[sample],
            })
            .collect()
    };
    Ok(match axis {
        None => NumericValue::Scalar(function(&at(0))),
        Some(x) => NumericValue::Waveform(
            x.clone(),
            (0..x.len()).map(|sample| function(&at(sample))).collect(),
        ),
    })
}

#[cfg(test)]
mod tests;
