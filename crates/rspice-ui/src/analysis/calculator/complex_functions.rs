//! Complex-aware function dispatch. Projections are explicit; real-only
//! measurements never silently consume the magnitude of a physical signal.

use num_complex::Complex64;

use super::complex_ops;
use super::evaluator::EvaluationError;
use super::functions::FunctionRegistry;
use super::value::{CalcValue, ComplexValue, RealValue, finite, hole};

pub(super) fn dispatch(name: &str, args: Vec<CalcValue>) -> Result<CalcValue, EvaluationError> {
    let name = name.to_ascii_lowercase();
    if name == "complex" {
        count(&name, &args, 2)?;
        let mut args = args.into_iter();
        let real = args.next().unwrap().into_real()?;
        let imag = args.next().unwrap().into_real()?;
        let imaginary = map_complex(CalcValue::Real(imag).into_complex(), |value| {
            Complex64::new(0.0, value.re)
        });
        return complex_ops::binary(
            super::ast::BinaryOp::Add,
            CalcValue::Real(real).into_complex(),
            imaginary,
        )
        .map(CalcValue::Complex);
    }
    if matches!(
        name.as_str(),
        "mag" | "magnitude" | "re" | "real" | "im" | "imag" | "phase" | "phase_deg" | "phase_rad"
    ) {
        count(&name, &args, 1)?;
        return project(args.into_iter().next().unwrap().into_complex(), &name)
            .map(CalcValue::Real);
    }
    if name == "conj" {
        count(&name, &args, 1)?;
        return Ok(CalcValue::Complex(map_complex(
            args.into_iter().next().unwrap().into_complex(),
            |value| value.conj(),
        )));
    }
    if args.iter().all(|arg| matches!(arg, CalcValue::Real(_))) {
        let args = args
            .into_iter()
            .map(CalcValue::into_real)
            .collect::<Result<_, _>>()?;
        return FunctionRegistry::dispatch(&name, args).map(CalcValue::Real);
    }
    if matches!(name.as_str(), "abs" | "db") {
        count(&name, &args, 1)?;
        return project(args.into_iter().next().unwrap().into_complex(), &name)
            .map(CalcValue::Real);
    }
    if matches!(name.as_str(), "sqrt" | "exp" | "ln" | "log" | "log10") {
        count(&name, &args, 1)?;
        let op = match name.as_str() {
            "sqrt" => complex_ops::square_root,
            "exp" => complex_ops::exponential,
            "log10" => |value| complex_ops::logarithm(value) / std::f64::consts::LN_10,
            _ => complex_ops::logarithm,
        };
        return Ok(CalcValue::Complex(map_complex(
            args.into_iter().next().unwrap().into_complex(),
            op,
        )));
    }
    if matches!(name.as_str(), "avg" | "average" | "rms") {
        count(&name, &args, 1)?;
        let value = args.into_iter().next().unwrap().into_complex();
        let (real, imag) = components(value);
        let component_function = if name == "average" { "avg" } else { &name };
        let real = FunctionRegistry::dispatch(component_function, vec![real])?;
        let imag = FunctionRegistry::dispatch(component_function, vec![imag])?;
        let (RealValue::Scalar(real), RealValue::Scalar(imag)) = (real, imag) else {
            return Err(EvaluationError::TypeMismatch(
                "aggregate did not return scalars".to_owned(),
            ));
        };
        return Ok(if name == "rms" {
            CalcValue::Real(RealValue::Scalar(real.hypot(imag)))
        } else {
            CalcValue::Complex(ComplexValue::Scalar(Complex64::new(real, imag)))
        });
    }
    if matches!(
        name.as_str(),
        "deriv" | "derivative" | "integ" | "integral" | "clip" | "yval" | "xval"
    ) {
        let expected = match name.as_str() {
            "clip" => 3,
            "yval" => 2,
            _ => 1,
        };
        count(&name, &args, expected)?;
        let mut args = args.into_iter();
        let (real, imag) = components(args.next().unwrap().into_complex());
        let tail = args
            .map(CalcValue::into_real)
            .collect::<Result<Vec<_>, _>>()?;
        let mut real_args = vec![real];
        real_args.extend(tail.clone());
        let real = FunctionRegistry::dispatch(&name, real_args)?;
        if name == "xval" {
            return Ok(CalcValue::Real(real));
        }
        let mut imag_args = vec![imag];
        imag_args.extend(tail);
        let imag = FunctionRegistry::dispatch(&name, imag_args)?;
        return combine(real, imag).map(CalcValue::Complex);
    }
    Err(EvaluationError::TypeMismatch(format!(
        "{name} requires a real-valued signal; apply real(), imag(), mag(), or phase() explicitly"
    )))
}

fn count(name: &str, args: &[CalcValue], expected: usize) -> Result<(), EvaluationError> {
    if args.len() == expected {
        Ok(())
    } else {
        Err(EvaluationError::ArgCountMismatch {
            func: name.to_owned(),
            expected,
            actual: args.len(),
        })
    }
}

pub(super) fn project(value: ComplexValue, name: &str) -> Result<RealValue, EvaluationError> {
    let sample = |value: Complex64| {
        if !finite(value) {
            return f64::NAN;
        }
        match name {
            "db" => (20.0 / std::f64::consts::LN_10) * complex_ops::logarithm(value).re,
            "re" | "real" => value.re,
            "im" | "imag" => value.im,
            "phase" | "phase_deg" | "phase_rad" if value == Complex64::new(0.0, 0.0) => f64::NAN,
            "phase" | "phase_deg" => value.arg().to_degrees(),
            "phase_rad" => value.arg(),
            _ => value.re.hypot(value.im),
        }
    };
    Ok(match value {
        ComplexValue::Scalar(value) => RealValue::Scalar(sample(value)),
        ComplexValue::Waveform(x, y) => RealValue::Waveform(x, y.into_iter().map(sample).collect()),
    })
}

pub(super) fn map_complex(
    value: ComplexValue,
    operation: impl Fn(Complex64) -> Complex64,
) -> ComplexValue {
    match value {
        ComplexValue::Scalar(value) => ComplexValue::Scalar(operation(value)),
        ComplexValue::Waveform(x, y) => ComplexValue::Waveform(
            x,
            y.into_iter()
                .map(|value| {
                    if finite(value) {
                        hole(operation(value))
                    } else {
                        hole(value)
                    }
                })
                .collect(),
        ),
    }
}

fn components(value: ComplexValue) -> (RealValue, RealValue) {
    match value {
        ComplexValue::Scalar(value) => (RealValue::Scalar(value.re), RealValue::Scalar(value.im)),
        ComplexValue::Waveform(x, y) => {
            let (real, imag) = y.into_iter().map(|value| (value.re, value.im)).unzip();
            (
                RealValue::Waveform(x.clone(), real),
                RealValue::Waveform(x, imag),
            )
        }
    }
}

fn combine(real: RealValue, imag: RealValue) -> Result<ComplexValue, EvaluationError> {
    match (real, imag) {
        (RealValue::Scalar(real), RealValue::Scalar(imag)) => {
            Ok(ComplexValue::Scalar(Complex64::new(real, imag)))
        }
        (RealValue::Waveform(x, real), RealValue::Waveform(ix, imag))
            if x == ix && real.len() == imag.len() =>
        {
            Ok(ComplexValue::Waveform(
                x,
                real.into_iter()
                    .zip(imag)
                    .map(|(real, imag)| hole(Complex64::new(real, imag)))
                    .collect(),
            ))
        }
        _ => Err(EvaluationError::WaveformMismatch(
            "complex components produced different sample domains".to_owned(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scalar(re: f64, im: f64) -> CalcValue {
        CalcValue::Complex(ComplexValue::Scalar(Complex64::new(re, im)))
    }

    fn real(name: &str, value: CalcValue) -> f64 {
        let result = dispatch(name, vec![value])
            .unwrap()
            .checked()
            .unwrap()
            .into_real()
            .unwrap();
        let RealValue::Scalar(value) = result else {
            panic!("expected scalar")
        };
        value
    }

    #[test]
    fn complex_projections_and_phase_units_are_explicit() {
        assert_eq!(real("mag", scalar(3.0, 4.0)), 5.0);
        assert_eq!(real("real", scalar(3.0, 4.0)), 3.0);
        assert_eq!(real("imag", scalar(3.0, 4.0)), 4.0);
        let expected_db = 20.0 * (f64::MAX.log10() + 0.5 * 2.0_f64.log10());
        assert!((real("db", scalar(f64::MAX, f64::MAX)) - expected_db).abs() < 1e-11);
        assert_eq!(real("phase", scalar(0.0, 1.0)), 90.0);
        assert_eq!(real("phase_deg", scalar(0.0, -1.0)), -90.0);
        assert_eq!(
            real("phase_rad", scalar(0.0, 1.0)),
            std::f64::consts::FRAC_PI_2
        );
        assert_eq!(
            dispatch("conj", vec![scalar(3.0, 4.0)]).unwrap(),
            scalar(3.0, -4.0)
        );
        assert!(
            dispatch("phase", vec![scalar(0.0, 0.0)])
                .unwrap()
                .checked()
                .is_err()
        );
    }

    #[test]
    fn complex_rms_integrates_rectangular_segments_instead_of_magnitude_samples() {
        let wave = CalcValue::Complex(ComplexValue::Waveform(
            vec![0.0, 1.0],
            vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 1.0)],
        ));
        let rms = real("rms", wave.clone());
        assert!((rms - (2.0_f64 / 3.0).sqrt()).abs() < 1e-14);
        assert_eq!(dispatch("avg", vec![wave]).unwrap(), scalar(0.5, 0.5));
    }

    #[test]
    fn complex_functions_preserve_axes_and_do_not_invent_real_measurements() {
        let wave = CalcValue::Complex(ComplexValue::Waveform(
            vec![0.0, 1.0, 2.0],
            vec![
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 2.0),
                Complex64::new(2.0, 4.0),
            ],
        ));
        assert_eq!(
            dispatch("deriv", vec![wave.clone()]).unwrap(),
            CalcValue::Complex(ComplexValue::Waveform(
                vec![0.0, 1.0, 2.0],
                vec![Complex64::new(1.0, 2.0); 3]
            ))
        );
        let at = dispatch(
            "yval",
            vec![wave.clone(), CalcValue::Real(RealValue::Scalar(0.5))],
        )
        .unwrap();
        assert_eq!(at, scalar(0.5, 1.0));
        for name in ["min", "max", "cross", "unwrap"] {
            assert!(dispatch(name, vec![wave.clone()]).is_err(), "{name}");
        }
    }

    #[test]
    fn complex_domain_failures_are_errors_for_scalars_and_holes_for_series() {
        assert!(
            dispatch("ln", vec![scalar(0.0, 0.0)])
                .unwrap()
                .checked()
                .is_err()
        );
        let wave = CalcValue::Complex(ComplexValue::Waveform(
            vec![0.0, 1.0],
            vec![Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
        ));
        let CalcValue::Complex(ComplexValue::Waveform(x, y)) = dispatch("ln", vec![wave]).unwrap()
        else {
            panic!("complex series")
        };
        assert_eq!(x, [0.0, 1.0]);
        assert!(y[0].re.is_nan() && y[0].im.is_nan());
        assert_eq!(y[1], Complex64::new(0.0, 0.0));
    }
}
