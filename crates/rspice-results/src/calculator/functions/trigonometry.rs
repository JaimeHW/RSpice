//! The trigonometric and hyperbolic functions ViVA's calculator lists: `sin`,
//! `cos`, `tan` and their inverses read and return radians, and the
//! hyperbolic ones and their inverses. Each works sample by sample, under the
//! library's domain policy: a sample outside the domain is a hole in a series
//! and an error as a scalar.

use super::{CalcValue, EvaluationError, check_arg_count, domain_error, map_domain};

/// One function: its kernel, and the inputs it is defined for with how a
/// refusal names the rest.
struct Kernel {
    apply: fn(f64) -> f64,
    defined: fn(f64) -> bool,
    outside: &'static str,
}

const EVERYWHERE: fn(f64) -> bool = |_| true;

fn kernel(name: &str) -> Option<Kernel> {
    let unit_interval = |v: f64| (-1.0..=1.0).contains(&v);
    let (apply, defined, outside): (fn(f64) -> f64, fn(f64) -> bool, &str) = match name {
        "sin" => (f64::sin, EVERYWHERE, ""),
        "cos" => (f64::cos, EVERYWHERE, ""),
        "tan" => (f64::tan, EVERYWHERE, ""),
        "asin" => (f64::asin, unit_interval, "a value outside -1 to 1"),
        "acos" => (f64::acos, unit_interval, "a value outside -1 to 1"),
        "atan" => (f64::atan, EVERYWHERE, ""),
        "sinh" => (f64::sinh, EVERYWHERE, ""),
        "cosh" => (f64::cosh, EVERYWHERE, ""),
        "tanh" => (f64::tanh, EVERYWHERE, ""),
        "asinh" => (f64::asinh, EVERYWHERE, ""),
        "acosh" => (f64::acosh, |v| v >= 1.0, "a value below 1"),
        "atanh" => (
            f64::atanh,
            |v| v > -1.0 && v < 1.0,
            "a value outside the open interval -1 to 1",
        ),
        _ => return None,
    };
    Some(Kernel {
        apply,
        defined,
        outside,
    })
}

/// The names this module answers for.
pub(super) const NAMES: [&str; 12] = [
    "sin", "cos", "tan", "asin", "acos", "atan", "sinh", "cosh", "tanh", "asinh", "acosh", "atanh",
];

/// Run the trigonometric or hyperbolic function `name`, one of [`NAMES`].
pub(super) fn apply(name: &str, args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    let Some(kernel) = kernel(name) else {
        return Err(EvaluationError::UnknownFunction(name.to_owned()));
    };
    check_arg_count(name, &args, 1)?;
    match &args[0] {
        CalcValue::Scalar(value) if !(kernel.defined)(*value) => {
            Err(domain_error(name, kernel.outside))
        }
        CalcValue::Scalar(value) => Ok(CalcValue::Scalar((kernel.apply)(*value))),
        CalcValue::Waveform(x, y) => Ok(map_domain(x, y, |value| {
            if (kernel.defined)(value) {
                (kernel.apply)(value)
            } else {
                f64::NAN
            }
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn scalar(name: &str, value: f64) -> Result<f64, EvaluationError> {
        match apply(name, vec![CalcValue::Scalar(value)])? {
            CalcValue::Scalar(value) => Ok(value),
            other => panic!("{name} of a scalar returned {other:?}"),
        }
    }

    /// Each function against its closed form at points where the value is
    /// known exactly, and each inverse undoing its function.
    #[test]
    fn each_function_reads_radians_and_matches_its_closed_form() {
        let close = |name: &str, at: f64, expected: f64| {
            let value = scalar(name, at).unwrap();
            assert!(
                (value - expected).abs() <= 1e-15 * expected.abs().max(1.0),
                "{name}({at}) = {value}, expected {expected}"
            );
        };
        close("sin", PI / 6.0, 0.5);
        close("cos", PI / 3.0, 0.5);
        close("tan", PI / 4.0, 1.0);
        close("asin", 0.5, PI / 6.0);
        close("acos", 0.5, PI / 3.0);
        close("atan", 1.0, PI / 4.0);
        let ln2 = 2f64.ln();
        // sinh(ln 2) = (2 - 1/2) / 2, cosh(ln 2) = (2 + 1/2) / 2.
        close("sinh", ln2, 0.75);
        close("cosh", ln2, 1.25);
        close("tanh", ln2, 0.6);
        close("asinh", 0.75, ln2);
        close("acosh", 1.25, ln2);
        close("atanh", 0.6, ln2);
    }

    /// Outside its domain an inverse is a hole in a series and an error as
    /// a scalar, and the samples around the hole are kept.
    #[test]
    fn an_inverse_outside_its_domain_is_a_hole_or_an_error() {
        for (name, outside) in [
            ("asin", 1.5),
            ("acos", -2.0),
            ("acosh", 0.5),
            ("atanh", 1.0),
        ] {
            assert!(
                matches!(scalar(name, outside), Err(EvaluationError::MathError(_))),
                "{name}({outside}) must be refused"
            );
            let defined = if name == "acosh" { 1.0 } else { 0.0 };
            let series = apply(
                name,
                vec![CalcValue::create_waveform(
                    vec![0.0, 1.0, 2.0],
                    vec![defined, outside, defined],
                )],
            )
            .unwrap();
            let CalcValue::Waveform(_, y) = series else {
                panic!("{name} of a series returned a scalar");
            };
            assert!(
                y[0].is_finite() && y[2].is_finite(),
                "{name}: the defined samples are kept"
            );
            assert!(y[1].is_nan(), "{name}({outside}) must be a hole");
        }
    }
}
