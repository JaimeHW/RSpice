//! Calculator Evaluator
//!
//! Executes the `CalculatorExpr` AST against an `EvaluationContext`.
//! Handles vector arithmetic logic.

use super::ast::{BinaryOp, CalculatorConstant, CalculatorExpr, UnaryOp};
use super::measures::Argument;
pub use super::value::{CalcValue, RealValue};

/// Interface for retrieving simulation data
pub trait EvaluationContext {
    fn get_waveform(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, EvaluationError>;

    /// A direct magnitude projection can read a historical magnitude-only
    /// source without claiming its phase is known.
    fn get_magnitude(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, EvaluationError> {
        super::complex_functions::dispatch("mag", vec![self.get_waveform(signal, dataset)?])
    }

    /// The exact event history of a bus or logic net, `D(name)`, for
    /// `analog()`. A context without event histories names none.
    fn get_event_word(
        &self,
        signal: &str,
        _dataset: Option<&str>,
    ) -> Result<super::functions::EventWord, EvaluationError> {
        Err(EvaluationError::IdentifierNotFound(signal.to_owned()))
    }
}

#[derive(Debug, Clone)]
pub enum EvaluationError {
    IdentifierNotFound(String),
    UnknownFunction(String),
    ArgCountMismatch {
        func: String,
        expected: usize,
        actual: usize,
    },
    TypeMismatch(String),
    MathError(String),
    WaveformMismatch(String),
    PhaseUnavailable(String),
}

impl std::fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IdentifierNotFound(id) => write!(f, "Identifier not found: {}", id),
            Self::UnknownFunction(name) => write!(f, "Unknown function: {}", name),
            Self::ArgCountMismatch {
                func,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Argument count mismatch for {}: expected {}, got {}",
                    func, expected, actual
                )
            }
            Self::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            Self::MathError(msg) => write!(f, "Math error: {}", msg),
            Self::WaveformMismatch(reason) => write!(f, "Cannot combine waveforms: {reason}"),
            Self::PhaseUnavailable(signal) => write!(
                f,
                "Phase is not retained for {signal}; use mag() or dB() explicitly, or rerun to retain complex components"
            ),
        }
    }
}

impl std::error::Error for EvaluationError {}

pub fn evaluate(
    expr: &CalculatorExpr,
    ctx: &impl EvaluationContext,
) -> Result<CalcValue, EvaluationError> {
    let value = match expr {
        CalculatorExpr::Number(val) => Ok(CalcValue::Real(RealValue::Scalar(*val))),

        CalculatorExpr::Constant(c) => match c {
            // These should probably be handled by context or expanded earlier if they depend on context
            // But if we treat them as abstract signals:
            CalculatorConstant::Time => ctx.get_waveform("TIME", None),
            CalculatorConstant::Frequency => ctx.get_waveform("FREQ", None),
        },

        CalculatorExpr::WaveformRef { signal, dataset } => {
            ctx.get_waveform(signal, dataset.as_deref())
        }

        CalculatorExpr::UnaryOp { op, operand } => {
            let val = evaluate(operand, ctx)?;
            match op {
                UnaryOp::Neg => neg_value(val),
            }
        }

        CalculatorExpr::BinaryOp { op, left, right } => {
            let l_val = evaluate(left, ctx)?;
            let r_val = evaluate(right, ctx)?;
            apply_binary_op(*op, l_val, r_val)
        }

        CalculatorExpr::FunctionCall { name, args } => {
            // `analog(D(bus))` reads the bus's exact event history; anything
            // else it is given is evaluated as a value.
            if name.eq_ignore_ascii_case("analog")
                && let [CalculatorExpr::WaveformRef { signal, dataset }] = args.as_slice()
            {
                match ctx.get_event_word(signal, dataset.as_deref()) {
                    Ok(word) => {
                        return super::functions::analog_staircase(&word)
                            .map(CalcValue::Real)?
                            .checked();
                    }
                    Err(EvaluationError::IdentifierNotFound(_)) => {}
                    Err(error) => return Err(error),
                }
            }
            if matches!(
                name.to_ascii_lowercase().as_str(),
                "mag" | "magnitude" | "abs" | "db"
            ) && let [CalculatorExpr::WaveformRef { signal, dataset }] = args.as_slice()
            {
                return if name.eq_ignore_ascii_case("db") {
                    let value = match ctx.get_waveform(signal, dataset.as_deref()) {
                        Err(EvaluationError::PhaseUnavailable(_)) => {
                            ctx.get_magnitude(signal, dataset.as_deref())?
                        }
                        value => value?,
                    };
                    super::complex_functions::dispatch("db", vec![value])?.checked()
                } else {
                    ctx.get_magnitude(signal, dataset.as_deref())?.checked()
                };
            }
            let mut arg_values = Vec::with_capacity(args.len());
            for arg in args {
                arg_values.push(match arg {
                    CalculatorExpr::Keyword(word) => Argument::Keyword(word.clone()),
                    _ => Argument::Value(evaluate(arg, ctx)?),
                });
            }
            super::measures::dispatch(name, arg_values)
        }

        CalculatorExpr::Keyword(word) => Err(EvaluationError::TypeMismatch(format!(
            "\"{word}\" is an option of a measurement function, not a value"
        ))),
    };
    value?.checked()
}

/// What unit inference reads about the data an expression names.
pub trait UnitContext {
    /// The unit a referenced signal reads in, as written (`V`, `V^2/Hz`), or
    /// `None` when nothing states one.
    fn signal_unit(&self, signal: &str, dataset: Option<&str>) -> Option<String>;
    /// The unit of the coordinate the expression's waveforms run over: `s`
    /// for a transient, `Hz` for a frequency sweep, the swept source's unit.
    fn x_unit(&self) -> String;
}

/// The unit an expression's value reads in, by unit algebra over the units of
/// the signals it names: a ratio of voltages is a plain number, V·A is W,
/// `deriv` of volts over time is V/s.
///
/// `Err` says why no unit can be stated — volts added to amperes, the
/// logarithm of volts, a function whose result unit is not known — so a
/// caller never borrows another value's unit for it.
pub fn infer_unit(
    expr: &CalculatorExpr,
    ctx: &impl UnitContext,
) -> Result<super::value::Unit, String> {
    infer(expr, ctx).map(Inferred::into_unit)
}

/// A unit found so far. A bare number takes the unit of whatever it is
/// added to, so `V(out) - 2.5` stays in volts.
#[derive(Debug, Clone, PartialEq)]
enum Inferred {
    Literal(f64),
    Unit(super::value::Unit),
}

impl Inferred {
    fn into_unit(self) -> super::value::Unit {
        match self {
            Self::Literal(_) => super::value::Unit::none(),
            Self::Unit(unit) => unit,
        }
    }
}

fn read_unit(text: &str, what: &str) -> Result<super::value::Unit, String> {
    super::value::Unit::parse(text).ok_or_else(|| format!("{what} reads in “{text}”, not a unit"))
}

fn infer(expr: &CalculatorExpr, ctx: &impl UnitContext) -> Result<Inferred, String> {
    use super::value::Unit;
    Ok(match expr {
        CalculatorExpr::Number(value) => Inferred::Literal(*value),
        CalculatorExpr::Constant(CalculatorConstant::Time) => Inferred::Unit(read_unit("s", "")?),
        CalculatorExpr::Constant(CalculatorConstant::Frequency) => {
            Inferred::Unit(read_unit("Hz", "")?)
        }
        CalculatorExpr::WaveformRef { signal, dataset } => {
            let text = ctx
                .signal_unit(signal, dataset.as_deref())
                .ok_or_else(|| format!("{signal} states no unit"))?;
            Inferred::Unit(read_unit(&text, signal)?)
        }
        CalculatorExpr::UnaryOp {
            op: UnaryOp::Neg,
            operand,
        } => match infer(operand, ctx)? {
            Inferred::Literal(value) => Inferred::Literal(-value),
            unit => unit,
        },
        CalculatorExpr::BinaryOp { op, left, right } => {
            let (left, right) = (infer(left, ctx)?, infer(right, ctx)?);
            match (op, left, right) {
                (op, Inferred::Literal(left), Inferred::Literal(right)) => {
                    Inferred::Literal(apply_op_scalar(*op, left, right))
                }
                (BinaryOp::Add | BinaryOp::Sub, Inferred::Literal(_), other)
                | (BinaryOp::Add | BinaryOp::Sub, other, Inferred::Literal(_)) => other,
                (BinaryOp::Add | BinaryOp::Sub, Inferred::Unit(left), Inferred::Unit(right)) => {
                    if left != right {
                        return Err(format!(
                            "{} and {} cannot be added",
                            unit_phrase(&left),
                            unit_phrase(&right)
                        ));
                    }
                    Inferred::Unit(left)
                }
                (BinaryOp::Mul, left, right) => {
                    Inferred::Unit(left.into_unit().times(&right.into_unit()))
                }
                (BinaryOp::Div, left, right) => {
                    Inferred::Unit(left.into_unit().divided_by(&right.into_unit()))
                }
                (BinaryOp::Pow, base, Inferred::Literal(exponent)) => {
                    let base = base.into_unit();
                    Inferred::Unit(base.powf(exponent).ok_or_else(|| {
                        format!(
                            "{} to the power {exponent} is not a unit",
                            unit_phrase(&base)
                        )
                    })?)
                }
                (BinaryOp::Pow, base, exponent) => {
                    let (base, exponent) = (base.into_unit(), exponent.into_unit());
                    if !base.is_none() || !exponent.is_none() {
                        return Err(format!(
                            "{} raised to a signal has no unit",
                            unit_phrase(&base)
                        ));
                    }
                    Inferred::Unit(Unit::none())
                }
            }
        }
        CalculatorExpr::FunctionCall { name, args } => {
            Inferred::Unit(function_unit(&name.to_ascii_lowercase(), args, ctx)?)
        }
        CalculatorExpr::Keyword(word) => {
            return Err(format!("\"{word}\" is an option word, not a value"));
        }
    })
}

fn unit_phrase(unit: &super::value::Unit) -> String {
    if unit.is_none() {
        "a plain number".to_owned()
    } else {
        unit.label()
    }
}

/// The unit of a function's result, from the units of its arguments.
fn function_unit(
    name: &str,
    args: &[CalculatorExpr],
    ctx: &impl UnitContext,
) -> Result<super::value::Unit, String> {
    use super::value::Unit;
    let argument = |index: usize| -> Result<Unit, String> {
        let arg = args
            .get(index)
            .ok_or_else(|| format!("{name}() is missing an argument"))?;
        infer(arg, ctx).map(Inferred::into_unit)
    };
    let x_unit = || read_unit(&ctx.x_unit(), "The x-axis");
    let named = |text: &str| read_unit(text, name);
    match name {
        // The value itself, or a level of it.
        "abs" | "mag" | "magnitude" | "re" | "real" | "im" | "imag" | "conj" | "complex"
        | "clip" | "unwrap" | "avg" | "average" | "rms" | "min" | "max" | "pp" | "yval" => {
            argument(0)
        }
        // A point or a span on the x-axis.
        "xval" | "cross" | "rise" | "risetime" | "fall" | "falltime" | "delay" | "pulsewidth"
        | "period" | "settling" | "settlingtime" | "bandwidth" => x_unit(),
        "freq" | "frequency" => Ok(Unit::none().divided_by(&x_unit()?)),
        "deriv" | "derivative" | "slewrate" => Ok(argument(0)?.divided_by(&x_unit()?)),
        "integ" | "integral" => Ok(argument(0)?.times(&x_unit()?)),
        "duty" | "dutycycle" | "overshoot" | "undershoot" | "thd" => named("%"),
        "db" | "gainmargin" => named("dB"),
        "phase" | "phase_deg" | "phasemargin" => named("°"),
        "phase_rad" => named("rad"),
        "sqrt" => {
            let unit = argument(0)?;
            unit.powf(0.5)
                .ok_or_else(|| format!("the square root of {} is not a unit", unit.label()))
        }
        "exp" | "log" | "ln" | "log10" => {
            let unit = argument(0)?;
            if unit.is_none() {
                Ok(unit)
            } else {
                Err(format!("{name}() of {} has no unit", unit.label()))
            }
        }
        _ => Err(format!("the unit of {name}() is not known")),
    }
}

fn neg_value(value: CalcValue) -> Result<CalcValue, EvaluationError> {
    match value {
        CalcValue::Real(value) => neg_real_value(value).map(CalcValue::Real),
        CalcValue::Complex(value) => Ok(CalcValue::Complex(super::complex_functions::map_complex(
            value,
            |value| -value,
        ))),
    }
}

fn apply_binary_op(
    op: BinaryOp,
    left: CalcValue,
    right: CalcValue,
) -> Result<CalcValue, EvaluationError> {
    match (left, right) {
        (CalcValue::Real(left), CalcValue::Real(right)) => {
            apply_real_binary_op(op, left, right).map(CalcValue::Real)
        }
        (left, right) => super::complex_ops::binary(op, left.into_complex(), right.into_complex())
            .map(CalcValue::Complex),
    }
}
fn neg_real_value(val: RealValue) -> Result<RealValue, EvaluationError> {
    match val {
        RealValue::Scalar(s) => Ok(RealValue::Scalar(-s)),
        RealValue::Waveform(x, y) => {
            let new_y = y.into_iter().map(|v| -v).collect();
            Ok(RealValue::Waveform(x, new_y))
        }
    }
}

fn apply_real_binary_op(
    op: BinaryOp,
    left: RealValue,
    right: RealValue,
) -> Result<RealValue, EvaluationError> {
    for value in [&left, &right] {
        if let RealValue::Waveform(x, y) = value {
            super::interpolation::validate_samples(x, y)
                .map_err(|error| EvaluationError::WaveformMismatch(error.to_string()))?;
        }
    }
    match (left, right) {
        (RealValue::Scalar(l), RealValue::Scalar(r)) => {
            Ok(RealValue::Scalar(apply_op_scalar(op, l, r)))
        }
        (RealValue::Scalar(l), RealValue::Waveform(rx, ry)) => {
            let new_y = ry
                .into_iter()
                .map(|r_val| apply_op_scalar(op, l, r_val))
                .collect();
            Ok(RealValue::create_waveform(rx, new_y))
        }
        (RealValue::Waveform(lx, ly), RealValue::Scalar(r)) => {
            let new_y = ly
                .into_iter()
                .map(|l_val| apply_op_scalar(op, l_val, r))
                .collect();
            Ok(RealValue::create_waveform(lx, new_y))
        }
        (RealValue::Waveform(lx, ly), RealValue::Waveform(rx, ry)) => {
            // Equal complete axes permit pointwise operations, including
            // repeated coordinates and matching sweep branches. Unequal axes
            // require an unambiguous interpolation domain.
            let (out_x, left_y, right_y) = if lx == rx {
                (lx, ly, ry)
            } else {
                use super::interpolation::{InterpolationMethod, align_waveforms};
                align_waveforms(&lx, &ly, &rx, &ry, InterpolationMethod::Linear)
                    .map_err(|error| EvaluationError::WaveformMismatch(error.to_string()))?
            };

            let new_y: Vec<f64> = left_y
                .iter()
                .zip(right_y.iter())
                .map(|(l, r)| apply_op_scalar(op, *l, *r))
                .collect();

            Ok(RealValue::create_waveform(out_x, new_y))
        }
    }
}

fn apply_op_scalar(op: BinaryOp, l: f64, r: f64) -> f64 {
    if !l.is_finite() || !r.is_finite() {
        return f64::NAN;
    }
    match op {
        BinaryOp::Add => l + r,
        BinaryOp::Sub => l - r,
        BinaryOp::Mul => l * r,
        BinaryOp::Div => l / r, // Undefined samples remain gaps; checked scalars report an error.
        BinaryOp::Pow => l.powf(r),
    }
}

#[cfg(test)]
mod tests {
    use super::RealValue as CalcValue;
    use super::*;

    fn difference(
        left: (&[f64], &[f64]),
        right: (&[f64], &[f64]),
    ) -> Result<CalcValue, EvaluationError> {
        apply_real_binary_op(
            BinaryOp::Sub,
            CalcValue::Waveform(left.0.to_vec(), left.1.to_vec()),
            CalcValue::Waveform(right.0.to_vec(), right.1.to_vec()),
        )
    }

    struct Units {
        x: &'static str,
    }

    impl UnitContext for Units {
        fn signal_unit(&self, signal: &str, _dataset: Option<&str>) -> Option<String> {
            let unit = match signal.split('(').next()? {
                "V" => "V",
                "I" => "A",
                "onoise" => "V^2/Hz",
                "inoise" => "A²/Hz",
                "S21" => "ratio",
                "temp" => "°C",
                _ => return None,
            };
            Some(unit.to_owned())
        }

        fn x_unit(&self) -> String {
            self.x.to_owned()
        }
    }

    fn unit_of(text: &str, x: &'static str) -> Result<String, String> {
        let expr = super::super::parser::try_parse(text, |n| n.parse().map_err(|_| n.to_owned()))
            .unwrap_or_else(|error| panic!("{text}: {error:?}"));
        infer_unit(&expr, &Units { x }).map(|unit| unit.label())
    }

    /// The units engineers write for these expressions.
    #[test]
    fn unit_algebra_states_the_unit_an_engineer_writes() {
        let table: [(&str, &str, &str); 30] = [
            ("V(out)", "s", "V"),
            ("V(out)/V(in)", "s", ""),
            ("V(out)*I(vdd)", "s", "W"),
            ("I(vdd)*V(out)", "s", "W"),
            ("V(out)/I(r1)", "s", "Ω"),
            ("I(r1)/V(out)", "s", "S"),
            ("deriv(V(out))", "s", "V/s"),
            ("deriv(I(l1))", "s", "A/s"),
            ("integ(I(c1))", "s", "C"),
            ("integ(V(out)*I(vdd))", "s", "J"),
            ("I(c1)/deriv(V(out))", "s", "F"),
            ("V(out)/deriv(I(l1))", "s", "H"),
            ("V(out) - 2.5", "s", "V"),
            ("2*V(out) + V(in)/2", "s", "V"),
            ("-V(out)", "s", "V"),
            ("V(out)^2/1000", "s", "V²"),
            ("V(out)^2/(V(out)/I(r1))", "s", "W"),
            ("sqrt(onoise)", "Hz", "V/√Hz"),
            ("sqrt(inoise)", "Hz", "A/√Hz"),
            ("onoise", "Hz", "V²/Hz"),
            ("db(V(out)/V(in))", "Hz", "dB"),
            ("phase(V(out))", "Hz", "°"),
            ("deriv(phase(V(out)))", "Hz", "°·s"),
            ("S21*V(in)", "Hz", "V"),
            ("rise(V(out))", "s", "s"),
            ("frequency(V(out))", "s", "Hz"),
            ("1/period(V(out))", "s", "Hz"),
            ("overshoot(V(out))", "s", "%"),
            ("deriv(V(out))", "°C", "V/°C"),
            ("rms(I(vdd))*avg(V(out))", "s", "W"),
        ];
        for (text, x, expected) in table {
            assert_eq!(unit_of(text, x).as_deref(), Ok(expected), "{text} over {x}");
        }
    }

    /// An expression whose unit cannot be stated says why rather than
    /// borrowing a unit.
    #[test]
    fn unit_algebra_refuses_what_has_no_unit() {
        for (text, reason) in [
            ("V(out)+I(r1)", "V and A cannot be added"),
            ("exp(V(out))", "exp() of V has no unit"),
            ("log10(I(r1))", "log10() of A has no unit"),
            ("V(out)^V(in)", "V raised to a signal has no unit"),
            ("V(out)^0.3", "V to the power 0.3 is not a unit"),
            ("mystery(V(out))", "the unit of mystery() is not known"),
            ("V(out)/node", "node states no unit"),
        ] {
            assert_eq!(unit_of(text, "s"), Err(reason.to_owned()), "{text}");
        }
        assert_eq!(unit_of("exp(V(out)/V(in))", "s").as_deref(), Ok(""));
        assert_eq!(unit_of("2^3 + 1", "s").as_deref(), Ok(""));
    }

    #[test]
    fn written_units_read_back_to_one_unit() {
        use super::super::value::Unit;
        let read = |text: &str| Unit::parse(text).unwrap();
        assert_eq!(read("V^2/Hz"), read("V²/Hz"));
        assert_eq!(read("V²/Hz"), read("V·V·s"));
        assert_eq!(read("W"), read("V·A"));
        assert_eq!(read("Ω"), read("ohm"));
        assert_eq!(read("ratio"), Unit::none());
        assert_eq!(read("V/√Hz").label(), "V/√Hz");
        assert_eq!(read("V").divided_by(&read("A·s")).label(), "V/(A·s)");
        assert_eq!(read("x").divided_by(&read("x")), Unit::none());
        assert_eq!(Unit::parse("V/A/s"), None);
        assert_eq!(Unit::parse("V/(A·s)"), None);
        assert_eq!(read("A·s/V").label(), "F");
    }

    #[test]
    fn equal_endpoints_do_not_imply_equal_interior_coordinates() {
        let first = [0.0, 1.0, 3.0];
        let second = [0.0, 2.0, 3.0];
        for (left, right) in [(&first, &second), (&second, &first)] {
            assert_eq!(
                difference((left, left), (right, right)).unwrap(),
                CalcValue::Waveform(vec![0.0, 1.0, 2.0, 3.0], vec![0.0; 4])
            );
        }
    }

    #[test]
    fn alignment_preserves_breakpoints_from_both_operands() {
        assert_eq!(
            difference(
                (&[0.0, 2.0], &[0.0, 0.0]),
                (&[0.0, 1.0, 2.0], &[0.0, 1.0, 0.0])
            )
            .unwrap(),
            CalcValue::Waveform(vec![0.0, 1.0, 2.0], vec![0.0, -1.0, 0.0])
        );
    }

    #[test]
    fn alignment_preserves_the_left_sweeps_direction() {
        let left = [3.0, 1.0, 0.0];
        let right = [0.0, 2.0, 3.0];
        assert_eq!(
            difference((&left, &left), (&right, &right)).unwrap(),
            CalcValue::Waveform(vec![3.0, 2.0, 1.0, 0.0], vec![0.0; 4])
        );
    }

    #[test]
    fn arithmetic_uses_only_the_shared_domain() {
        assert_eq!(
            difference((&[0.0, 2.0], &[0.0, 2.0]), (&[1.0, 3.0], &[2.0, 6.0])).unwrap(),
            CalcValue::Waveform(vec![1.0, 2.0], vec![-1.0, -2.0])
        );
        assert!(difference((&[0.0, 1.0], &[0.0, 1.0]), (&[2.0, 3.0], &[2.0, 3.0])).is_err());
    }

    #[test]
    fn a_single_point_has_no_extrapolated_extent() {
        assert_eq!(
            difference((&[0.0, 2.0], &[0.0, 2.0]), (&[1.0], &[1.0])).unwrap(),
            CalcValue::Waveform(vec![1.0], vec![0.0])
        );
    }

    #[test]
    fn malformed_operands_cannot_be_silently_zipped_or_broadcast() {
        for malformed in [
            CalcValue::Waveform(vec![0.0, 1.0], vec![1.0]),
            CalcValue::Waveform(vec![0.0], vec![1.0, 2.0]),
        ] {
            for other in [
                CalcValue::Scalar(2.0),
                CalcValue::Waveform(vec![0.0, 1.0], vec![1.0, 2.0]),
            ] {
                assert!(
                    apply_real_binary_op(BinaryOp::Add, malformed.clone(), other.clone()).is_err()
                );
                assert!(apply_real_binary_op(BinaryOp::Add, other, malformed.clone()).is_err());
            }
        }
    }

    #[test]
    fn matching_branches_remain_pointwise_but_ambiguous_resampling_is_rejected() {
        for x in [[0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] {
            assert_eq!(
                difference((&x, &[1.0, 2.0, 3.0]), (&x, &[1.0, 2.0, 3.0])).unwrap(),
                CalcValue::Waveform(x.to_vec(), vec![0.0; 3])
            );
            assert!(difference((&x, &[1.0, 2.0, 3.0]), (&[0.0, 1.0], &[1.0, 2.0])).is_err());
        }
    }
}
