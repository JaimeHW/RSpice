//! Calculator Evaluator
//!
//! Executes the `CalculatorExpr` AST against an `EvaluationContext`.
//! Handles vector arithmetic logic.

use super::ast::{BinaryOp, CalculatorConstant, CalculatorExpr, Cursor, UnaryOp};
use super::measures::Argument;
pub use super::value::{CalcValue, RealValue};
use crate::waveform::WaveformValues;

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

    /// The stored columns of `signal` when it is one stored real trace, so a
    /// reduction over it reads them a block at a time instead of whole.
    /// `None` for anything else (a trace in memory, a complex one, another
    /// run's), which is read as a value.
    fn stored_columns(
        &self,
        _signal: &str,
        _dataset: Option<&str>,
    ) -> Option<(&WaveformValues, &WaveformValues)> {
        None
    }

    /// Where `cursor` stands on the x-axis, for `xA` and `xB`; `None` while
    /// it is not placed. A context with no cursors places none.
    fn cursor(&self, _cursor: Cursor) -> Option<f64> {
        None
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
            CalculatorConstant::Cursor(cursor) => ctx
                .cursor(*cursor)
                .map(|x| CalcValue::Real(RealValue::Scalar(x)))
                .ok_or_else(|| {
                    EvaluationError::IdentifierNotFound(format!(
                        "{}: cursor {} is not placed",
                        cursor.name(),
                        cursor.letter()
                    ))
                }),
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
            let lower = name.to_ascii_lowercase();
            let decibels = matches!(lower.as_str(), "db" | "db20" | "db10");
            if (decibels || matches!(lower.as_str(), "mag" | "magnitude" | "abs"))
                && let [CalculatorExpr::WaveformRef { signal, dataset }] = args.as_slice()
            {
                return if decibels {
                    let value = match ctx.get_waveform(signal, dataset.as_deref()) {
                        Err(EvaluationError::PhaseUnavailable(_)) => {
                            ctx.get_magnitude(signal, dataset.as_deref())?
                        }
                        value => value?,
                    };
                    super::complex_functions::dispatch(&lower, vec![value])?.checked()
                } else {
                    ctx.get_magnitude(signal, dataset.as_deref())?.checked()
                };
            }
            if let Some(value) = stored_measure(name, &lower, args, ctx)? {
                return value.checked();
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
/// `deriv` of volts over time is V/s. The unit keeps the reading its
/// composition gives it: `deriv` of amperes over a frequency sweep is A/Hz,
/// never the coulomb that shares its dimensions.
///
/// `Err` says why no unit can be stated — volts added to amperes, the
/// logarithm of volts, a function whose result unit is not known — so a
/// caller never borrows another value's unit for it.
pub fn infer_unit(
    expr: &CalculatorExpr,
    ctx: &impl UnitContext,
) -> Result<super::value::Unit, String> {
    infer(expr, ctx).map(|inferred| inferred.into_unit().reading_reciprocal_seconds_as_hertz())
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
        // A cursor stands on the x-axis, so it reads in the x-axis' unit.
        CalculatorExpr::Constant(CalculatorConstant::Cursor(_)) => {
            Inferred::Unit(read_unit(&ctx.x_unit(), "The x-axis")?)
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
                // A comparison is 1 or 0, of operands that could be added.
                (
                    BinaryOp::Lt
                    | BinaryOp::Gt
                    | BinaryOp::Le
                    | BinaryOp::Ge
                    | BinaryOp::Eq
                    | BinaryOp::Ne,
                    left,
                    right,
                ) => {
                    if let (Inferred::Unit(left), Inferred::Unit(right)) = (&left, &right)
                        && left != right
                    {
                        return Err(format!(
                            "{} and {} cannot be compared",
                            unit_phrase(left),
                            unit_phrase(right)
                        ));
                    }
                    Inferred::Unit(Unit::none())
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
        | "clip" | "limit" | "unwrap" | "avg" | "average" | "rms" | "min" | "max" | "pp"
        | "yval" => argument(0),
        // A point or a span on the x-axis.
        "xval" | "cross" | "rise" | "risetime" | "fall" | "falltime" | "delay" | "pulsewidth"
        | "period" | "settling" | "settlingtime" | "bandwidth" | "abs_jitter" | "period_jitter"
        | "cc_jitter" | "longterm_jitter" => x_unit(),
        "freq" | "frequency" => Ok(Unit::none().divided_by(&x_unit()?)),
        "deriv" | "derivative" | "slewrate" => Ok(argument(0)?.divided_by(&x_unit()?)),
        "integ" | "integral" => Ok(argument(0)?.times(&x_unit()?)),
        "duty" | "dutycycle" | "overshoot" | "undershoot" | "thd" => named("%"),
        "stddev" | "median" | "percentile" => argument(0),
        "variance" => Ok(argument(0)?.times(&argument(0)?)),
        "skewness" | "kurtosis" => Ok(Unit::none()),
        // The amplitude decibel of a power is a number with no meaning, so
        // it states none and names the power form instead.
        "db" | "db20" => {
            let power = args
                .first()
                .and_then(|arg| infer(arg, ctx).ok())
                .map(Inferred::into_unit)
                .filter(Unit::is_power);
            match power {
                Some(unit) => Err(format!(
                    "{} is a power, whose decibels dB10() reads; dB() and dB20() are \
                     20·log₁₀ for amplitudes",
                    unit.label()
                )),
                None => named("dB"),
            }
        }
        "db10" | "gainmargin" | "snr" | "sinad" | "sfdr" => named("dB"),
        // A two-port's stability factors and power gains are ratios.
        "kf" | "b1f" | "mu" | "mu_prime" | "gt" | "ga" | "gp" | "gmax" | "gmsg" | "vswr" => {
            Ok(Unit::none())
        }
        "zp" => named("Ω"),
        "yp" => named("S"),
        // h11 is an impedance, h22 an admittance, and the cross terms ratios.
        "hp" => match (args.first(), args.get(1)) {
            (Some(CalculatorExpr::Number(row)), Some(CalculatorExpr::Number(column))) => {
                match (*row as i64, *column as i64) {
                    (1, 1) => named("Ω"),
                    (2, 2) => named("S"),
                    _ => Ok(Unit::none()),
                }
            }
            _ => {
                Err("hp() reads its unit from the coefficient it names, as two numbers".to_owned())
            }
        },
        "enob" => Ok(Unit::none()),
        // The delay of a response swept in hertz is in seconds.
        "groupdelay" => {
            let x = x_unit()?;
            if x == named("Hz")? {
                named("s")
            } else {
                Ok(Unit::none().divided_by(&x))
            }
        }
        // The value chosen, which must read in one unit whichever is chosen.
        "if" => {
            let (chosen, otherwise) = (
                args.get(1).map(|arg| infer(arg, ctx)).transpose()?,
                args.get(2).map(|arg| infer(arg, ctx)).transpose()?,
            );
            match (chosen, otherwise) {
                (Some(Inferred::Literal(_)), Some(other))
                | (Some(other), Some(Inferred::Literal(_))) => Ok(other.into_unit()),
                (Some(Inferred::Unit(chosen)), Some(Inferred::Unit(otherwise))) => {
                    if chosen == otherwise {
                        Ok(chosen)
                    } else {
                        Err(format!(
                            "if() chooses between {} and {}, which are not one unit",
                            unit_phrase(&chosen),
                            unit_phrase(&otherwise)
                        ))
                    }
                }
                _ => Err("if() is missing an argument".to_owned()),
            }
        }
        "phase" | "phase_deg" | "phasemargin" => named("°"),
        "phase_rad" => named("rad"),
        // A bus word's integer value and the levels 0 and 1 of a logic
        // signal are plain numbers, whatever the converted signal reads in.
        "analog" | "digital" => Ok(Unit::none()),
        "sqrt" => {
            let unit = argument(0)?;
            unit.powf(0.5)
                .ok_or_else(|| format!("the square root of {} is not a unit", unit.label()))
        }
        // Radians are a plain number; degrees are the one angle that would
        // read silently wrong.
        "sin" | "cos" | "tan" => {
            let unit = argument(0)?;
            if unit.is_none() || unit == named("rad")? {
                Ok(Unit::none())
            } else if unit == named("°")? {
                Err(format!(
                    "{name}() reads radians; phase() is in degrees, phase_rad() in radians"
                ))
            } else {
                Err(format!("{name}() of {} has no unit", unit.label()))
            }
        }
        "asin" | "acos" | "atan" => {
            let unit = argument(0)?;
            if unit.is_none() {
                named("rad")
            } else {
                Err(format!("{name}() of {} has no unit", unit.label()))
            }
        }
        "exp" | "log" | "ln" | "log10" | "sinh" | "cosh" | "tanh" | "asinh" | "acosh" | "atanh" => {
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

/// A measurement of a stored trace, read where it is stored a block at a time: a reduction,
/// a time measurement or a level measure whose trace, or `delay` either of whose traces,
/// [`stored_series`] finds. `None` for anything else, which is read as a value.
fn stored_measure(
    name: &str,
    lower: &str,
    args: &[CalculatorExpr],
    ctx: &impl EvaluationContext,
) -> Result<Option<CalcValue>, EvaluationError> {
    use super::measures::{reduction, reduction_arity, series};
    use super::series::Series;
    let evaluated = |rest: &[CalculatorExpr]| -> Result<Vec<Argument>, EvaluationError> {
        rest.iter()
            .map(|arg| match arg {
                CalculatorExpr::Keyword(word) => Ok(Argument::Keyword(word.clone())),
                _ => evaluate(arg, ctx).map(Argument::Value),
            })
            .collect()
    };
    let scalar = |value: f64| Some(CalcValue::Real(RealValue::Scalar(value)));
    if lower == "delay"
        && let [first, second, rest @ ..] = args
    {
        let (from, to) = (stored_series(first, ctx)?, stored_series(second, ctx)?);
        if from.is_none() && to.is_none() {
            return Ok(None);
        }
        // A trace read as a value is read as such, as `delay` reads both in memory.
        let (from_value, to_value): ([Argument; 1], [Argument; 1]);
        let from = match from {
            Some(from) => from,
            None => {
                from_value = [Argument::Value(evaluate(first, ctx)?)];
                let (x, y) = series(name, &from_value, 0)?;
                Series::memory(x, y)
            }
        };
        let to = match to {
            Some(to) => to,
            None => {
                to_value = [Argument::Value(evaluate(second, ctx)?)];
                let (x, y) = series(name, &to_value, 0)?;
                Series::memory(x, y)
            }
        };
        let rest = evaluated(rest)?;
        return super::time_measures::measure_delay(name, &from, &to, &rest).map(scalar);
    }
    let reduce = reduction(lower);
    let timed = super::time_measures::lookup(lower);
    let level = super::levels::lookup(lower);
    if reduce.is_none() && timed.is_none() && level.is_none() {
        return Ok(None);
    }
    let Some(first) = args.first() else {
        return Ok(None);
    };
    let Some(trace) = stored_series(first, ctx)? else {
        return Ok(None);
    };
    if let Some(reduce) = reduce {
        reduction_arity(name, args.len())?;
        let rest = evaluated(&args[1..])?;
        return Ok(Some(CalcValue::Real(reduce(name, &trace, &rest)?)));
    }
    let rest = evaluated(&args[1..])?;
    if let Some(timed) = timed {
        return Ok(Some(CalcValue::Real(timed.measure(name, &trace, &rest)?)));
    }
    match level {
        Some(level) => Ok(Some(CalcValue::Real(level(name, &trace, &rest)?))),
        None => Ok(None),
    }
}

/// The stored trace a reduction's first argument names, read where it is
/// stored: a signal, or `clip` of one between two scalars. `None` when the
/// argument is anything else, or names no stored trace.
fn stored_series<'c>(
    expr: &CalculatorExpr,
    ctx: &'c impl EvaluationContext,
) -> Result<Option<super::series::Series<'c>>, EvaluationError> {
    use super::series::Series;
    let columns = |expr: &CalculatorExpr| match expr {
        CalculatorExpr::WaveformRef { signal, dataset } => {
            ctx.stored_columns(signal, dataset.as_deref())
        }
        _ => None,
    };
    if let Some((x, y)) = columns(expr) {
        return Ok(Some(Series::stored(x, y)));
    }
    if let CalculatorExpr::FunctionCall { name, args } = expr
        && name.eq_ignore_ascii_case("clip")
        && let [trace, from, to] = args.as_slice()
        && let Some((x, y)) = columns(trace)
    {
        let end = |expr: &CalculatorExpr, what: &str| match evaluate(expr, ctx)? {
            CalcValue::Real(RealValue::Scalar(value)) => Ok(value),
            _ => Err(EvaluationError::TypeMismatch(format!(
                "clip {what} must be a scalar"
            ))),
        };
        let (from, to) = (end(from, "start")?, end(to, "end")?);
        return Series::stored(x, y).clipped(from, to).map(Some);
    }
    Ok(None)
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
        _ if op.is_comparison() => Err(EvaluationError::TypeMismatch(
            "a comparison reads real values; apply real(), mag() or phase() first".to_owned(),
        )),
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
        BinaryOp::Lt => f64::from(l < r),
        BinaryOp::Gt => f64::from(l > r),
        BinaryOp::Le => f64::from(l <= r),
        BinaryOp::Ge => f64::from(l >= r),
        BinaryOp::Eq => f64::from(l == r),
        BinaryOp::Ne => f64::from(l != r),
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
        let table: &[(&str, &str, &str)] = &[
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
            ("deriv(phase(V(out)))", "Hz", "°/Hz"),
            ("S21*V(in)", "Hz", "V"),
            ("rise(V(out))", "s", "s"),
            ("frequency(V(out))", "s", "Hz"),
            ("1/period(V(out))", "s", "Hz"),
            ("overshoot(V(out))", "s", "%"),
            ("deriv(V(out))", "°C", "V/°C"),
            ("rms(I(vdd))*avg(V(out))", "s", "W"),
            // Units that share dimensions keep the reading their composition
            // gives them.
            ("deriv(I(r1))", "Hz", "A/Hz"),
            ("deriv(V(out))", "Hz", "V/Hz"),
            ("integ(V(out)^2)", "s", "V²·s"),
            ("integ(I(r1)^2)", "s", "A²·s"),
            ("V(out)*frequency(V(out))", "s", "V/s"),
            ("V(out)*I(vdd)/freq", "Hz", "W/Hz"),
            // A spectral density reads as one however it is composed.
            ("inoise", "Hz", "A²/Hz"),
            ("sqrt(onoise)^2", "Hz", "V²/Hz"),
            ("onoise*V(out)^2/V(in)^2", "Hz", "V²/Hz"),
            ("sqrt(onoise)/(V(out)/V(in))", "Hz", "V/√Hz"),
            ("onoise/(V(out)/I(r1))", "Hz", "W/Hz"),
            ("inoise*(V(out)/I(r1))", "Hz", "W/Hz"),
            ("sqrt(onoise*inoise)", "Hz", "W/Hz"),
            ("sqrt(onoise/inoise)", "Hz", "Ω"),
            ("integ(onoise)", "Hz", "V²"),
            ("sqrt(integ(onoise))", "Hz", "V"),
            // A converted bus word or logic level is a plain number.
            ("analog(D(code))", "s", ""),
            ("digital(V(clk), 0.9)", "s", ""),
            // Decibels: dB10 reads a power, dB and dB20 an amplitude.
            ("dB10(V(out)*I(vdd))", "s", "dB"),
            ("dB10(onoise)", "Hz", "dB"),
            ("dB20(V(out)/V(in))", "Hz", "dB"),
            ("clip(V(out), 0.001, 0.002)", "s", "V"),
            ("limit(I(vdd), 0, 0.001)", "s", "A"),
            ("atan(V(out)/V(in))", "s", "rad"),
            ("sin(phase_rad(V(out)))", "Hz", ""),
            ("tanh(V(out)/V(in))", "s", ""),
            ("V(out) > 2.5", "s", ""),
            ("stddev(V(out))", "s", "V"),
            ("sfdr(V(out))", "s", "dB"),
            ("xB - xA", "s", "s"),
            ("1 / (xB - xA)", "s", "Hz"),
            ("yval(V(out), xA)", "Hz", "V"),
            ("period_jitter(V(out), \"sd\")", "s", "s"),
            ("abs_jitter(V(out))", "s", "s"),
            ("enob(V(out))", "s", ""),
            ("groupDelay(V(out)/V(in))", "Hz", "s"),
            ("variance(I(vdd), \"sample\")", "s", "A²"),
            ("percentile(V(out), 90)", "s", "V"),
            ("kurtosis(V(out))", "s", ""),
            ("if(V(out) > 2.5, V(out), 0)", "s", "V"),
            ("if(TIME < 0.001, 0, I(vdd))", "s", "A"),
        ];
        for &(text, x, expected) in table {
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
            (
                "cos(phase(V(out)))",
                "cos() reads radians; phase() is in degrees, phase_rad() in radians",
            ),
            ("cosh(V(out))", "cosh() of V has no unit"),
            ("V(out) < I(r1)", "V and A cannot be compared"),
            (
                "if(TIME < 0.001, V(out), I(vdd))",
                "if() chooses between V and A, which are not one unit",
            ),
            ("asin(I(r1))", "asin() of A has no unit"),
            ("V(out)/node", "node states no unit"),
            (
                "db(V(out)*I(vdd))",
                "W is a power, whose decibels dB10() reads; dB() and dB20() are 20·log₁₀ for \
                 amplitudes",
            ),
            (
                "dB20(onoise)",
                "V²/Hz is a power, whose decibels dB10() reads; dB() and dB20() are 20·log₁₀ \
                 for amplitudes",
            ),
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
        assert_ne!(read("V²/Hz"), read("V·V·s"));
        assert_ne!(read("A/Hz"), read("A·s"));
        assert_eq!(read("A/Hz").label(), "A/Hz");
        assert_eq!(read("A·s").label(), "C");
        assert_eq!(read("V²·s").label(), "V²·s");
        assert_eq!(read("V·A/Hz").label(), "W/Hz");
        assert_eq!(read("Hz·s"), Unit::none());
        assert_eq!(read("V/Hz").times(&read("s")), read("V·s²"));
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

    /// Traces by name, for evaluating whole expressions.
    struct Traces(Vec<(&'static str, Vec<f64>, Vec<f64>)>);

    impl EvaluationContext for Traces {
        fn get_waveform(
            &self,
            signal: &str,
            _: Option<&str>,
        ) -> Result<super::CalcValue, EvaluationError> {
            self.0
                .iter()
                .find(|(name, ..)| *name == signal)
                .map(|(_, x, y)| super::CalcValue::create_waveform(x.clone(), y.clone()))
                .ok_or_else(|| EvaluationError::IdentifierNotFound(signal.to_owned()))
        }
    }

    fn evaluated(traces: &Traces, text: &str) -> Result<CalcValue, String> {
        let expr = super::super::parser::try_parse(text, |n| n.parse().map_err(|_| n.to_owned()))
            .map_err(|error| error.to_string())?;
        match evaluate(&expr, traces).map_err(|error| error.to_string())? {
            super::CalcValue::Real(value) => Ok(value),
            other => Err(format!("{text} is complex: {other:?}")),
        }
    }

    /// Traces and a placed cursor pair.
    struct Placed(Traces, Option<f64>, Option<f64>);

    impl EvaluationContext for Placed {
        fn get_waveform(
            &self,
            signal: &str,
            dataset: Option<&str>,
        ) -> Result<super::CalcValue, EvaluationError> {
            self.0.get_waveform(signal, dataset)
        }

        fn cursor(&self, cursor: Cursor) -> Option<f64> {
            match cursor {
                Cursor::A => self.1,
                Cursor::B => self.2,
            }
        }
    }

    /// `xA` and `xB` are where the cursors stand, in any expression: the
    /// span between them, a window between them, a reading at one. An
    /// unplaced cursor is refused by name, and `xa` still names a signal.
    #[test]
    fn the_cursor_names_read_where_the_cursors_stand() {
        let x: Vec<f64> = (0..=100).map(|i| f64::from(i) * 1e-5).collect();
        let ramp: Vec<f64> = x.iter().map(|t| 1e3 * t).collect();
        let placed = Placed(
            Traces(vec![("V(r)", x.clone(), ramp)]),
            Some(2e-4),
            Some(6e-4),
        );
        let scalar = |text: &str| -> f64 {
            let expr =
                super::super::parser::try_parse(text, |n| n.parse().map_err(|_| n.to_owned()))
                    .unwrap();
            match evaluate(&expr, &placed) {
                Ok(super::CalcValue::Real(CalcValue::Scalar(value))) => value,
                other => panic!("{text}: {other:?}"),
            }
        };
        assert_eq!(scalar("xB - xA"), 6e-4 - 2e-4);
        let error = evaluated(&placed.0, "xa").unwrap_err();
        assert!(error.contains("not found: xa"), "{error}");
        // The ramp's mean between the cursors is its value midway.
        let mean = scalar("avg(clip(V(r), xA, xB))");
        assert!((mean - 0.4).abs() <= 1e-12, "{mean}");
        let at = scalar("yval(V(r), xB)");
        assert!((at - 0.6).abs() <= 1e-12, "{at}");
        let unplaced = Placed(Traces(Vec::new()), Some(1.0), None);
        let expr =
            super::super::parser::try_parse("xB", |n| n.parse().map_err(|_| n.to_owned())).unwrap();
        let error = evaluate(&expr, &unplaced).unwrap_err().to_string();
        assert!(error.contains("cursor B is not placed"), "{error}");
        // Without a cursor pair, as in a saved expression, neither is placed.
        assert!(evaluated(&placed.0, "xA").is_err());
    }

    /// A comparison reads 1 where it holds and 0 where it does not, and
    /// `if` with a series condition chooses sample by sample: rectifying a
    /// sine reads |sin| at every sample, a hole in the condition is a hole in
    /// the result, and a branch on another grid is read on the shared one.
    #[test]
    fn if_with_a_series_condition_chooses_sample_by_sample() {
        let x: Vec<f64> = (0..401)
            .map(|i| 1e-3 * (f64::from(i) / 400.0).powf(1.3))
            .collect();
        let sine: Vec<f64> = x
            .iter()
            .map(|t| (2.0 * std::f64::consts::PI * 3e3 * t).sin())
            .collect();
        let mut holed = sine.clone();
        holed[7] = f64::NAN;
        // Three of the fine grid's samples, so the grid they share is the fine one.
        let coarse = vec![x[0], x[200], x[400]];
        let traces = Traces(vec![
            ("V(a)", x.clone(), sine.clone()),
            ("V(h)", x.clone(), holed),
            // A ramp, which linear interpolation reads exactly anywhere.
            (
                "V(r)",
                coarse.clone(),
                coarse.iter().map(|t| 2e3 * t).collect(),
            ),
        ]);
        let series = |text: &str| match evaluated(&traces, text) {
            Ok(CalcValue::Waveform(x, y)) => (x, y),
            other => panic!("{text}: {other:?}"),
        };

        let (gx, at_or_above) = series("V(a) >= 0");
        assert_eq!(gx, x);
        for (reading, value) in at_or_above.iter().zip(&sine) {
            assert_eq!(*reading, f64::from(*value >= 0.0));
        }
        let (rx, rectified) = series("if(V(a) >= 0, V(a), -V(a))");
        assert_eq!(rx, x);
        for (reading, value) in rectified.iter().zip(&sine) {
            assert_eq!(*reading, value.abs());
        }
        let (_, chosen) = series("if(V(h) > 0, 1, 2)");
        assert!(chosen[7].is_nan(), "a hole in the condition is a hole");
        assert_eq!(chosen[8], if sine[8] > 0.0 { 1.0 } else { 2.0 });
        let (shared, ramp_where_positive) = series("if(V(a) > 0, V(r), 0 - 1)");
        assert_eq!(
            shared.len(),
            x.len(),
            "the coarse grid's samples are three of the fine one's"
        );
        for (t, reading) in shared.iter().zip(&ramp_where_positive) {
            let holds = (2.0 * std::f64::consts::PI * 3e3 * t).sin() > 0.0;
            let expected = if holds { 2e3 * t } else { -1.0 };
            assert!(
                (reading - expected).abs() < 1e-12,
                "at {t}: {reading} vs {expected}"
            );
        }

        // A scalar condition chooses the whole of one branch, and a
        // condition is true above 0.5.
        assert_eq!(
            evaluated(&traces, "if(0.3, 3, 4)"),
            Ok(CalcValue::Scalar(4.0))
        );
        assert_eq!(
            evaluated(&traces, "if(0.7, 3, 4)"),
            Ok(CalcValue::Scalar(3.0))
        );
        assert_eq!(
            evaluated(&traces, "if(1 > 2, 3, 4)"),
            Ok(CalcValue::Scalar(4.0))
        );
        assert_eq!(evaluated(&traces, "2 != 2"), Ok(CalcValue::Scalar(0.0)));
        assert_eq!(evaluated(&traces, "1 + 1 == 2"), Ok(CalcValue::Scalar(1.0)));
        let error = evaluated(&traces, "V(a) = 1").unwrap_err();
        assert!(error.contains("compare with == or !="), "{error}");
    }
}
