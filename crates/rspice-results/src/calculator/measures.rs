//! Arguments for the measurement functions that take options, and the one
//! dispatch every function call goes through.
//!
//! A measurement such as `cross(V(out), 2.5, 1, "rising")` mixes values with
//! quoted option words. The words never become values: they reach the
//! function that reads them as [`Argument::Keyword`], and every function that
//! takes no options refuses one by name instead of reading it as a signal.

use super::complex_functions;
use super::evaluator::EvaluationError;
use super::value::{CalcValue, RealValue};
use super::{frequency_measures, time_measures};

/// One evaluated argument of a function call.
#[derive(Debug, Clone, PartialEq)]
pub(super) enum Argument {
    Value(CalcValue),
    /// A quoted option word, lower-cased by the parser.
    Keyword(String),
}

impl From<RealValue> for Argument {
    fn from(value: RealValue) -> Self {
        Self::Value(CalcValue::Real(value))
    }
}

/// A measurement that reads options: the name as the reader spelled it, for
/// its messages, and the arguments.
pub(super) type Measure = fn(&str, &[Argument]) -> Result<f64, EvaluationError>;

/// Run the function `name` on `args`.
pub(super) fn dispatch(name: &str, args: Vec<Argument>) -> Result<CalcValue, EvaluationError> {
    let lower = name.to_ascii_lowercase();
    if let Some(measure) =
        frequency_measures::lookup(&lower).or_else(|| time_measures::lookup(&lower))
    {
        return measure(name, &args).map(|value| CalcValue::Real(RealValue::Scalar(value)));
    }
    let values = args
        .into_iter()
        .map(|argument| match argument {
            Argument::Value(value) => Ok(value),
            Argument::Keyword(word) => Err(EvaluationError::TypeMismatch(format!(
                "{name} takes no option words; \"{word}\" is not one of its arguments"
            ))),
        })
        .collect::<Result<Vec<_>, _>>()?;
    complex_functions::dispatch(name, values)
}

/// Which way an edge or crossing goes, as ViVA's "Edge Type" names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Edge {
    Rising,
    Falling,
    Either,
}

impl Edge {
    pub(super) fn admits(self, rising: bool) -> bool {
        match self {
            Edge::Rising => rising,
            Edge::Falling => !rising,
            Edge::Either => true,
        }
    }

    pub(super) fn word(self) -> &'static str {
        match self {
            Edge::Rising => "rising",
            Edge::Falling => "falling",
            Edge::Either => "either",
        }
    }
}

pub(super) fn arity(
    name: &str,
    args: &[Argument],
    least: usize,
    most: usize,
) -> Result<(), EvaluationError> {
    if args.len() < least || args.len() > most {
        Err(EvaluationError::ArgCountMismatch {
            func: name.to_owned(),
            expected: if args.len() < least { least } else { most },
            actual: args.len(),
        })
    } else {
        Ok(())
    }
}

fn real<'a>(name: &str, argument: &'a Argument) -> Result<&'a RealValue, EvaluationError> {
    match argument {
        Argument::Value(CalcValue::Real(value)) => Ok(value),
        Argument::Value(CalcValue::Complex(_)) => Err(EvaluationError::TypeMismatch(format!(
            "{name} requires a real-valued signal; apply real(), imag(), mag(), or phase() explicitly"
        ))),
        Argument::Keyword(word) => Err(EvaluationError::TypeMismatch(format!(
            "{name} reads a value where \"{word}\" stands"
        ))),
    }
}

/// The real waveform at `index`, with samples.
pub(super) fn series<'a>(
    name: &str,
    args: &'a [Argument],
    index: usize,
) -> Result<(&'a [f64], &'a [f64]), EvaluationError> {
    match real(name, &args[index])? {
        RealValue::Waveform(x, y) if !x.is_empty() && x.len() == y.len() => Ok((x, y)),
        RealValue::Waveform(..) => Err(EvaluationError::MathError(format!(
            "{name} needs a waveform with samples"
        ))),
        RealValue::Scalar(_) => Err(EvaluationError::TypeMismatch(format!(
            "{name} measures a waveform, not a scalar"
        ))),
    }
}

/// The scalar at `index`, or `None` when the call stops before it.
pub(super) fn number(
    name: &str,
    what: &str,
    args: &[Argument],
    index: usize,
) -> Result<Option<f64>, EvaluationError> {
    let Some(argument) = args.get(index) else {
        return Ok(None);
    };
    match real(name, argument)? {
        RealValue::Scalar(value) if value.is_finite() => Ok(Some(*value)),
        RealValue::Scalar(_) => Err(EvaluationError::MathError(format!(
            "{name} {what} must be a finite number"
        ))),
        RealValue::Waveform(..) => Err(EvaluationError::TypeMismatch(format!(
            "{name} {what} must be a scalar"
        ))),
    }
}

/// A whole number of 1 or more at `index` — which edge, pulse or crossing —
/// defaulting to the first.
pub(super) fn ordinal(
    name: &str,
    what: &str,
    args: &[Argument],
    index: usize,
) -> Result<usize, EvaluationError> {
    match number(name, what, args, index)? {
        None => Ok(1),
        Some(value) if value >= 1.0 && value.fract() == 0.0 && value <= u32::MAX as f64 => {
            Ok(value as usize)
        }
        Some(_) => Err(EvaluationError::MathError(format!(
            "{name} {what} must be a whole number of 1 or more"
        ))),
    }
}

/// The option word at `index`, which must be one of `allowed`, or `None`
/// when the call stops before it.
pub(super) fn keyword<'a>(
    name: &str,
    what: &str,
    args: &'a [Argument],
    index: usize,
    allowed: &[&str],
) -> Result<Option<&'a str>, EvaluationError> {
    let quoted = || {
        allowed
            .iter()
            .map(|word| format!("\"{word}\""))
            .collect::<Vec<_>>()
            .join(", ")
    };
    match args.get(index) {
        None => Ok(None),
        Some(Argument::Keyword(word)) if allowed.contains(&word.as_str()) => Ok(Some(word)),
        Some(Argument::Keyword(word)) => Err(EvaluationError::TypeMismatch(format!(
            "{name} {what} is one of {}, not \"{word}\"",
            quoted()
        ))),
        Some(Argument::Value(_)) => Err(EvaluationError::TypeMismatch(format!(
            "{name} {what} is one of {}",
            quoted()
        ))),
    }
}

/// The edge type at `index`, defaulting to `default`.
pub(super) fn edge(
    name: &str,
    what: &str,
    args: &[Argument],
    index: usize,
    default: Edge,
) -> Result<Edge, EvaluationError> {
    Ok(
        match keyword(name, what, args, index, &["rising", "falling", "either"])? {
            Some("rising") => Edge::Rising,
            Some("falling") => Edge::Falling,
            Some(_) => Edge::Either,
            None => default,
        },
    )
}

/// English ordinal for an error message: 1st, 2nd, 3rd, 11th, 22nd.
pub(super) fn nth(n: usize) -> String {
    let suffix = match (n % 10, n % 100) {
        (_, 11..=13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    };
    format!("{n}{suffix}")
}
